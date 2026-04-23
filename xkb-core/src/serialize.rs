//! XKB keymap text serialization (v1 format).
//!
//! Produces output compatible with `xkb_keymap_get_as_string()` from libxkbcommon.

use crate::context::xkb_atom_text;
use crate::keysym::xkb_keysym_get_name;
use crate::shared_types::*;
use crate::text::*;

/// Serialize a compiled keymap to the XKB v1 text format.
pub fn xkb_keymap_get_as_string(keymap: &xkb_keymap) -> String {
    let mut buf = String::with_capacity(16 * 1024);
    buf.push_str("xkb_keymap {\n");
    write_keycodes(&mut buf, keymap);
    write_types(&mut buf, keymap);
    write_compat(&mut buf, keymap);
    write_symbols(&mut buf, keymap);
    buf.push_str("};\n");
    buf
}

// ── Keycodes section ────────────────────────────────────────────────

fn write_keycodes(buf: &mut String, km: &xkb_keymap) {
    buf.push_str("xkb_keycodes ");
    write_section_name(buf, &km.keycodes_section_name);
    buf.push_str(" {\n");

    buf.push_str(&format!("\tminimum = {};\n", km.min_key_code));
    buf.push_str(&format!("\tmaximum = {};\n", km.max_key_code));

    for kc in km.min_key_code..=km.max_key_code {
        if let Some(key) = km.get_key(kc) {
            let name = xkb_atom_text(&km.ctx.atom_table, key.name);
            if !name.is_empty() {
                buf.push_str(&format!("\t<{}> = {};\n", name, kc));
            }
        }
    }

    for alias in &km.key_aliases {
        let alias_name = xkb_atom_text(&km.ctx.atom_table, alias.alias);
        let real_name = xkb_atom_text(&km.ctx.atom_table, alias.real);
        if !alias_name.is_empty() && !real_name.is_empty() {
            buf.push_str(&format!("\talias <{}> = <{}>;\n", alias_name, real_name));
        }
    }

    for i in 0..km.num_leds {
        let led = &km.leds[i as usize];
        let name = xkb_atom_text(&km.ctx.atom_table, led.name);
        if !name.is_empty() {
            buf.push_str(&format!("\tindicator {} = \"{}\";\n", i + 1, name));
        }
    }

    buf.push_str("};\n\n");
}

// ── Types section ───────────────────────────────────────────────────

fn write_types(buf: &mut String, km: &xkb_keymap) {
    buf.push_str("xkb_types ");
    write_section_name(buf, &km.types_section_name);
    buf.push_str(" {\n");

    // Virtual modifier declarations
    write_vmods(buf, km);

    for kt in km.types.iter() {
        let type_name = xkb_atom_text(&km.ctx.atom_table, kt.name);
        buf.push_str(&format!("\ttype \"{}\" {{\n", type_name));

        buf.push_str(&format!(
            "\t\tmodifiers= {};\n",
            ModMaskText(&km.ctx, MOD_BOTH, &km.mods, kt.mods.mods)
        ));

        for entry in &kt.entries {
            if !entry_is_active(entry) {
                continue;
            }
            let mod_text = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, entry.mods.mods);
            buf.push_str(&format!(
                "\t\tmap[{}]= Level{};\n",
                mod_text,
                entry.level + 1
            ));
            if entry.preserve.mods != 0 {
                let preserve_text = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, entry.preserve.mods);
                buf.push_str(&format!("\t\tpreserve[{}]= {};\n", mod_text, preserve_text));
            }
        }

        for (li, level_name) in kt.level_names.iter().enumerate() {
            if *level_name != XKB_ATOM_NONE {
                let ln = xkb_atom_text(&km.ctx.atom_table, *level_name);
                buf.push_str(&format!("\t\tlevel_name[Level{}]= \"{}\";\n", li + 1, ln));
            }
        }

        buf.push_str("\t};\n");
    }

    buf.push_str("};\n\n");
}

// ── Compat section ──────────────────────────────────────────────────

fn write_compat(buf: &mut String, km: &xkb_keymap) {
    buf.push_str("xkb_compat ");
    write_section_name(buf, &km.compat_section_name);
    buf.push_str(" {\n");

    // Virtual modifier declarations
    write_vmods(buf, km);

    for si in &km.sym_interprets {
        let sym_text = if si.sym != XKB_KEY_NoSymbol as u32 {
            KeysymText(si.sym)
        } else {
            "Any".to_string()
        };
        let match_text = SIMatchText(si.match_0);
        let mod_text = ModMaskText(&km.ctx, MOD_REAL, &km.mods, si.mods);

        buf.push_str(&format!(
            "\tinterpret {}+{}({}) {{\n",
            sym_text, match_text, mod_text
        ));

        if si.virtual_mod != XKB_MOD_INVALID {
            let vm = ModIndexText(&km.ctx, &km.mods, si.virtual_mod);
            buf.push_str(&format!("\t\tvirtualModifier= {};\n", vm));
        }

        if si.level_one_only {
            buf.push_str("\t\tuseModMapMods=level1;\n");
        }

        if si.repeat {
            buf.push_str("\t\trepeat= True;\n");
        }

        write_action(buf, km, &si.action, "\t\t");

        buf.push_str("\t};\n");
    }

    // LED (indicator) maps
    for i in 0..km.num_leds {
        let led = &km.leds[i as usize];
        let name = xkb_atom_text(&km.ctx.atom_table, led.name);
        if name.is_empty() {
            continue;
        }
        // Only write non-trivial indicators
        if led.which_groups == 0
            && led.groups == 0
            && led.which_mods == 0
            && led.mods.mods == 0
            && led.ctrls == 0
        {
            continue;
        }
        buf.push_str(&format!("\tindicator \"{}\" {{\n", name));

        if led.which_mods != 0 {
            let mod_text = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, led.mods.mods);
            buf.push_str(&format!("\t\tmodifiers= {};\n", mod_text));
        }
        if led.which_groups != 0 && led.groups != 0 {
            buf.push_str(&format!("\t\tgroups= {:#x};\n", led.groups));
        }
        if led.ctrls != 0 {
            buf.push_str(&format!("\t\tcontrols= {};\n", led.ctrls));
        }

        buf.push_str("\t};\n");
    }

    buf.push_str("};\n\n");
}

// ── Symbols section ─────────────────────────────────────────────────

fn write_symbols(buf: &mut String, km: &xkb_keymap) {
    buf.push_str("xkb_symbols ");
    write_section_name(buf, &km.symbols_section_name);
    buf.push_str(" {\n");

    // Group names
    for i in 0..km.num_groups {
        if let Some(gn_atom) = km.group_names.get(i as usize) {
            if *gn_atom != XKB_ATOM_NONE {
                let gn = xkb_atom_text(&km.ctx.atom_table, *gn_atom);
                buf.push_str(&format!("\tname[group{}]=\"{}\";\n", i + 1, gn));
            }
        }
    }

    for kc in km.min_key_code..=km.max_key_code {
        let key = match km.get_key(kc) {
            Some(k) => k,
            None => continue,
        };
        if key.num_groups == 0 {
            continue;
        }
        let key_name = xkb_atom_text(&km.ctx.atom_table, key.name);
        if key_name.is_empty() {
            continue;
        }

        buf.push_str(&format!("\tkey <{}> {{", key_name));

        // Type per group (if explicitly set)
        let mut has_explicit_type = false;
        for gi in 0..key.num_groups {
            let group = &key.groups[gi as usize];
            if group.explicit_type {
                has_explicit_type = true;
                break;
            }
        }
        if has_explicit_type {
            if key.num_groups > 1 {
                buf.push('\n');
                for gi in 0..key.num_groups {
                    let group = &key.groups[gi as usize];
                    if group.explicit_type {
                        let type_name = xkb_atom_text(
                            &km.ctx.atom_table,
                            km.types[group.type_idx as usize].name,
                        );
                        buf.push_str(&format!("\t\ttype[group{}]= \"{}\",\n", gi + 1, type_name));
                    }
                }
            } else {
                let group = &key.groups[0];
                let type_name =
                    xkb_atom_text(&km.ctx.atom_table, km.types[group.type_idx as usize].name);
                buf.push_str(&format!(" type= \"{}\",", type_name));
            }
        }

        // Symbols per group
        if key.num_groups > 1 {
            buf.push('\n');
            for gi in 0..key.num_groups {
                write_key_symbols_group(buf, km, key, gi, true);
            }
            buf.push('\t');
        } else {
            write_key_symbols_group(buf, km, key, 0, false);
        }

        // Repeat
        if key.explicit & EXPLICIT_REPEAT != 0 {
            if key.repeats {
                buf.push_str(" repeat=Yes,");
            } else {
                buf.push_str(" repeat=No,");
            }
        }

        // Virtual modifier map
        if key.vmodmap != 0 && key.explicit & EXPLICIT_VMODMAP != 0 {
            let vm = ModMaskText(&km.ctx, MOD_VIRT, &km.mods, key.vmodmap);
            buf.push_str(&format!(" virtualMods= {},", vm));
        }

        buf.push_str(" };\n");
    }

    // Modifier map
    for kc in km.min_key_code..=km.max_key_code {
        if let Some(key) = km.get_key(kc) {
            if key.modmap != 0 {
                let key_name = xkb_atom_text(&km.ctx.atom_table, key.name);
                let mm = ModMaskText(&km.ctx, MOD_REAL, &km.mods, key.modmap);
                buf.push_str(&format!("\tmodifier_map {} {{ <{}> }};\n", mm, key_name));
            }
        }
    }

    buf.push_str("};\n\n");
}

// ── Helpers ─────────────────────────────────────────────────────────

fn write_section_name(buf: &mut String, name: &str) {
    if name.is_empty() {
        buf.push('"');
        buf.push('"');
    } else {
        buf.push('"');
        buf.push_str(name);
        buf.push('"');
    }
}

fn write_vmods(buf: &mut String, km: &xkb_keymap) {
    let num_real = _XKB_MOD_INDEX_NUM_ENTRIES;
    if km.mods.num_mods > num_real {
        buf.push_str("\tvirtual_modifiers ");
        let mut first = true;
        for i in num_real..km.mods.num_mods {
            let name = xkb_atom_text(&km.ctx.atom_table, km.mods.mods[i as usize].name);
            if !name.is_empty() {
                if !first {
                    buf.push(',');
                }
                buf.push_str(name);
                first = false;
            }
        }
        buf.push_str(";\n\n");
    }
}

fn write_key_symbols_group(
    buf: &mut String,
    km: &xkb_keymap,
    key: &xkb_key,
    group_idx: u32,
    multigroup: bool,
) {
    let group = &key.groups[group_idx as usize];
    let num_levels = km.types[group.type_idx as usize].num_levels;

    if multigroup {
        buf.push_str("\t\tsymbols[Group");
        buf.push_str(&(group_idx + 1).to_string());
        buf.push_str("]= [ ");
    } else {
        buf.push_str(" [ ");
    }

    for li in 0..num_levels {
        if li > 0 {
            buf.push_str(", ");
        }
        let level = &group.levels[li as usize];
        if level.syms.is_empty() {
            buf.push_str("NoSymbol");
        } else if level.syms.len() == 1 {
            buf.push_str(&keysym_text(level.syms[0]));
        } else {
            buf.push_str("{ ");
            for (si, &sym) in level.syms.iter().enumerate() {
                if si > 0 {
                    buf.push_str(", ");
                }
                buf.push_str(&keysym_text(sym));
            }
            buf.push_str(" }");
        }
    }
    buf.push_str(" ]");

    // Actions (if explicitly set)
    if group.explicit_actions {
        if multigroup {
            buf.push_str(",\n\t\tactions[Group");
            buf.push_str(&(group_idx + 1).to_string());
            buf.push_str("]= [ ");
        } else {
            buf.push_str(",\n\t\tactions= [ ");
        }
        for li in 0..num_levels {
            if li > 0 {
                buf.push_str(", ");
            }
            let level = &group.levels[li as usize];
            if level.actions.is_empty() {
                buf.push_str("NoAction()");
            } else {
                write_action_inline(buf, km, &level.actions[0]);
            }
        }
        buf.push_str(" ]");
    }

    if multigroup {
        buf.push_str(",\n");
    }
}

fn keysym_text(sym: u32) -> String {
    if sym == XKB_KEY_NoSymbol as u32 {
        "NoSymbol".to_string()
    } else {
        xkb_keysym_get_name(sym)
    }
}

fn write_action(buf: &mut String, km: &xkb_keymap, action: &xkb_action, prefix: &str) {
    match action {
        xkb_action::None => {
            buf.push_str(&format!("{}action= NoAction();\n", prefix));
        }
        xkb_action::ModSet(m) => {
            let mods = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, m.mods.mods);
            let clear = if m.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if m.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!(
                "{}action= SetMods(modifiers={}{}{});\n",
                prefix, mods, clear, latch
            ));
        }
        xkb_action::ModLatch(m) => {
            let mods = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, m.mods.mods);
            let clear = if m.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if m.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!(
                "{}action= LatchMods(modifiers={}{}{});\n",
                prefix, mods, clear, latch
            ));
        }
        xkb_action::ModLock(m) => {
            let mods = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, m.mods.mods);
            buf.push_str(&format!(
                "{}action= LockMods(modifiers={});\n",
                prefix, mods
            ));
        }
        xkb_action::GroupSet(g) => {
            let val = if g.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", g.group + 1)
            } else if g.group >= 0 {
                format!("+{}", g.group)
            } else {
                format!("{}", g.group)
            };
            let clear = if g.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if g.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!(
                "{}action= SetGroup(group={}{}{});\n",
                prefix, val, clear, latch
            ));
        }
        xkb_action::GroupLatch(g) => {
            let val = if g.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", g.group + 1)
            } else if g.group >= 0 {
                format!("+{}", g.group)
            } else {
                format!("{}", g.group)
            };
            let clear = if g.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if g.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!(
                "{}action= LatchGroup(group={}{}{});\n",
                prefix, val, clear, latch
            ));
        }
        xkb_action::GroupLock(g) => {
            let val = if g.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", g.group + 1)
            } else if g.group >= 0 {
                format!("+{}", g.group)
            } else {
                format!("{}", g.group)
            };
            buf.push_str(&format!("{}action= LockGroup(group={});\n", prefix, val));
        }
        xkb_action::Terminate => {
            buf.push_str(&format!("{}action= Terminate();\n", prefix));
        }
        xkb_action::PtrMove(p) => {
            let x_flag = if p.flags & ACTION_ABSOLUTE_X != 0 {
                ""
            } else if p.x >= 0 {
                "+"
            } else {
                ""
            };
            let y_flag = if p.flags & ACTION_ABSOLUTE_Y != 0 {
                ""
            } else if p.y >= 0 {
                "+"
            } else {
                ""
            };
            buf.push_str(&format!(
                "{}action= MovePtr(x={}{},y={}{});\n",
                prefix, x_flag, p.x, y_flag, p.y
            ));
        }
        xkb_action::PtrButton(b) => {
            buf.push_str(&format!("{}action= PtrBtn(button={});\n", prefix, b.button));
        }
        xkb_action::PtrLock(b) => {
            buf.push_str(&format!(
                "{}action= LockPtrBtn(button={});\n",
                prefix, b.button
            ));
        }
        xkb_action::PtrDefault(d) => {
            let val = if d.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", d.value)
            } else if d.value >= 0 {
                format!("+{}", d.value)
            } else {
                format!("{}", d.value)
            };
            buf.push_str(&format!(
                "{}action= SetPtrDflt(affect=button,button={});\n",
                prefix, val
            ));
        }
        xkb_action::SwitchVt(s) => {
            let val = if s.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", s.screen)
            } else if s.screen >= 0 {
                format!("+{}", s.screen)
            } else {
                format!("{}", s.screen)
            };
            let same = if s.flags & ACTION_SAME_SCREEN != 0 {
                ",!same"
            } else {
                ""
            };
            buf.push_str(&format!(
                "{}action= SwitchScreen(screen={}{});\n",
                prefix, val, same
            ));
        }
        xkb_action::CtrlSet(c) => {
            buf.push_str(&format!(
                "{}action= SetControls(controls={});\n",
                prefix, c.ctrls
            ));
        }
        xkb_action::CtrlLock(c) => {
            buf.push_str(&format!(
                "{}action= LockControls(controls={});\n",
                prefix, c.ctrls
            ));
        }
        xkb_action::RedirectKey(r) => {
            buf.push_str(&format!(
                "{}action= RedirectKey(keycode={});\n",
                prefix, r.keycode
            ));
        }
        _ => {
            buf.push_str(&format!("{}action= NoAction();\n", prefix));
        }
    }
}

fn write_action_inline(buf: &mut String, km: &xkb_keymap, action: &xkb_action) {
    match action {
        xkb_action::None => buf.push_str("NoAction()"),
        xkb_action::ModSet(m) => {
            let mods = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, m.mods.mods);
            let clear = if m.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if m.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!("SetMods(modifiers={}{}{}", mods, clear, latch));
            if m.flags & ACTION_MODS_LOOKUP_MODMAP != 0 {
                buf.push_str(",modMapMods");
            }
            buf.push(')');
        }
        xkb_action::ModLatch(m) => {
            let mods = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, m.mods.mods);
            let clear = if m.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if m.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!("LatchMods(modifiers={}{}{})", mods, clear, latch));
        }
        xkb_action::ModLock(m) => {
            let mods = ModMaskText(&km.ctx, MOD_BOTH, &km.mods, m.mods.mods);
            buf.push_str(&format!("LockMods(modifiers={})", mods));
        }
        xkb_action::GroupSet(g) => {
            let val = if g.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", g.group + 1)
            } else if g.group >= 0 {
                format!("+{}", g.group)
            } else {
                format!("{}", g.group)
            };
            let clear = if g.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if g.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!("SetGroup(group={}{}{})", val, clear, latch));
        }
        xkb_action::GroupLatch(g) => {
            let val = if g.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", g.group + 1)
            } else if g.group >= 0 {
                format!("+{}", g.group)
            } else {
                format!("{}", g.group)
            };
            let clear = if g.flags & ACTION_LOCK_CLEAR != 0 {
                ",clearLocks"
            } else {
                ""
            };
            let latch = if g.flags & ACTION_LATCH_TO_LOCK != 0 {
                ",latchToLock"
            } else {
                ""
            };
            buf.push_str(&format!("LatchGroup(group={}{}{})", val, clear, latch));
        }
        xkb_action::GroupLock(g) => {
            let val = if g.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", g.group + 1)
            } else if g.group >= 0 {
                format!("+{}", g.group)
            } else {
                format!("{}", g.group)
            };
            buf.push_str(&format!("LockGroup(group={})", val));
        }
        xkb_action::Terminate => buf.push_str("Terminate()"),
        xkb_action::PtrMove(p) => {
            let x_flag = if p.flags & ACTION_ABSOLUTE_X != 0 {
                ""
            } else if p.x >= 0 {
                "+"
            } else {
                ""
            };
            let y_flag = if p.flags & ACTION_ABSOLUTE_Y != 0 {
                ""
            } else if p.y >= 0 {
                "+"
            } else {
                ""
            };
            buf.push_str(&format!("MovePtr(x={}{},y={}{})", x_flag, p.x, y_flag, p.y));
        }
        xkb_action::PtrButton(b) => {
            buf.push_str(&format!("PtrBtn(button={})", b.button));
        }
        xkb_action::PtrLock(b) => {
            buf.push_str(&format!("LockPtrBtn(button={})", b.button));
        }
        xkb_action::PtrDefault(d) => {
            let val = if d.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", d.value)
            } else if d.value >= 0 {
                format!("+{}", d.value)
            } else {
                format!("{}", d.value)
            };
            buf.push_str(&format!("SetPtrDflt(affect=button,button={})", val));
        }
        xkb_action::SwitchVt(s) => {
            let val = if s.flags & ACTION_ABSOLUTE_SWITCH != 0 {
                format!("{}", s.screen)
            } else if s.screen >= 0 {
                format!("+{}", s.screen)
            } else {
                format!("{}", s.screen)
            };
            buf.push_str(&format!("SwitchScreen(screen={})", val));
        }
        xkb_action::CtrlSet(c) => {
            buf.push_str(&format!("SetControls(controls={})", c.ctrls));
        }
        xkb_action::CtrlLock(c) => {
            buf.push_str(&format!("LockControls(controls={})", c.ctrls));
        }
        xkb_action::RedirectKey(r) => {
            buf.push_str(&format!("RedirectKey(keycode={})", r.keycode));
        }
        _ => buf.push_str("NoAction()"),
    }
}
