//! XKB v1 text serialization from WKB's flat keysym tables.

use super::keynames::{evdev_to_keyname, named_key_to_keysym};
use super::keysym::keysym_get_name;
use crate::flat_keymap::{FlatKeymap, FlatNamedKeyMap, MAX_LEVELS};
use crate::modifiers::*;
use crate::named_keys::NamedKey;
use crate::WKB;

// Generate XKB v1 text format from WKB's flat keysym tables.

/// Determine how many levels a key actually uses across all groups.
/// Checks `named_key_map`, `level_exceptions_keymap`, and the modifier map
/// (modifier keys must be included even if they produce no named key or character).
fn key_max_level(
    named_key_map: &FlatNamedKeyMap,
    level_exceptions: &FlatKeymap,
    modifiers: &Modifiers,
    evdev: u32,
) -> usize {
    let mut max_level = 0;
    for level in (0..MAX_LEVELS).rev() {
        let has_named = named_key_map.get(level, evdev) != NamedKey::Unnamed;
        let has_char = level_exceptions.get(level, evdev).is_some();
        if has_named || has_char {
            if level + 1 > max_level {
                max_level = level + 1;
            }
            break;
        }
    }
    // Modifier keys must always be included even if named_key is Unnamed
    // and they produce no character (e.g. ISO_Level3_Shift).
    if max_level == 0 && modifiers.get(evdev).is_some() {
        max_level = 1;
    }
    max_level
}

/// Resolve the keysym for a modifier key from the modifier map.
///
/// Maps each `ModType` to its canonical keysym so the re-parsed keymap
/// gets the correct modifier interpretation.
fn modifier_keysym(modifiers: &Modifiers, evdev: u32) -> Option<u32> {
    let modifier = modifiers.get(evdev)?;
    match modifier {
        Modifier::Single(mk) => modkind_keysym(mk),
        Modifier::Leveled(map) => map.values().next().and_then(modkind_keysym),
    }
}

fn modkind_keysym(mk: &StateModifier) -> Option<u32> {
    match (&mk.kind, &mk.mod_type) {
        (ModKind::Press { .. }, ModType::Level3) => Some(0xfe03),
        (ModKind::Press { .. }, ModType::Level5) => Some(0xfe11),
        (ModKind::Latch { .. }, ModType::Level3) => Some(0xfe04),
        (ModKind::Latch { .. }, ModType::Level5) => Some(0xfe12),
        (ModKind::Lock { .. }, ModType::Level3) => Some(0xfe0d),
        (ModKind::Lock { .. }, ModType::Level5) => Some(0xfe13),
        _ => None,
    }
}

/// Check if a key is a letter key (has both lowercase and uppercase
/// characters at levels 0 and 1). Used to select ALPHABETIC type.
fn is_alphabetic(keymap: &FlatKeymap, evdev: u32) -> bool {
    matches!(
        (keymap.get(0, evdev), keymap.get(1, evdev)),
        (Some(lower), Some(upper))
            if lower.is_ascii_lowercase()
                && upper.is_ascii_uppercase()
                && upper == lower.to_ascii_uppercase()
    )
}

/// Pick a key type name based on the number of levels.
fn type_for_levels(num_levels: usize) -> &'static str {
    match num_levels {
        0 | 1 => "ONE_LEVEL",
        2 => "TWO_LEVEL",
        3 | 4 => "FOUR_LEVEL",
        _ => "EIGHT_LEVEL",
    }
}

/// Format a keysym as its XKB name, or Unicode/hex fallback.
///
/// Legacy Latin-1 keysyms (0x20-0x7E, 0xA0-0xFF) are emitted as raw hex
/// Unicode keysyms (`0x10000XX`) to match C xkbcommon's serializer output.
/// This ensures round-tripping through xkbcommon produces identical keysym
/// values.
fn sym_name(sym: u32) -> String {
    if sym == 0 {
        return "NoSymbol".to_string();
    }
    // Unicode keysyms (0x01000000+): always emit as raw hex literal.
    // C xkbcommon's serializer does the same — e.g. 0x010000d7 for ×.
    // Using a named keysym (e.g. "multiply") would re-parse to the legacy
    // value (0xd7), causing a round-trip mismatch.
    if (0x0100_0000..=0x0110_ffff).contains(&sym) {
        return format!("{:#010x}", sym);
    }
    // Named keysyms (legacy Latin-1, function keys, etc.): use the name.
    if let Some(name) = keysym_get_name(sym) {
        name.to_string()
    } else {
        format!("{:#010x}", sym)
    }
}

impl WKB {
    /// Generate XKB v1 text format string from flat keysym tables.
    ///
    /// This produces a minimal but fully valid keymap that Wayland clients
    /// can parse.
    #[cfg(feature = "xkb")]
    pub(crate) fn generate_xkb_string(&self) -> String {
        let num_layouts = self.layouts.len();
        let num_keys = self
            .layouts
            .first()
            .map_or(0, |layout| layout.named_key_map.num_keys);
        // XKB keycodes max at 255; evdev = xkb - 8, so max evdev = 247
        let max_evdev = num_keys.min(248) as u32;
        // Estimate capacity: ~40KB for a typical keymap
        let mut out = String::with_capacity(40 * 1024);

        out.push_str("xkb_keymap {\n");

        // ── xkb_keycodes ──
        self.write_keycodes(&mut out, max_evdev);

        // ── xkb_types ──
        out.push_str(TYPES_XKB);

        // ── xkb_compat ──
        out.push_str(COMPAT_XKB);

        // ── xkb_symbols ──
        self.write_symbols(&mut out, max_evdev, num_layouts);

        out.push_str("};\n");
        out
    }

    /// Resolve the keysym for a (layout, level, evdev) triple.
    ///
    /// For named keys, returns the canonical keysym via `named_key_to_keysym`.
    /// For character keys (`NamedKey::Unnamed`), falls back to the character
    /// keymaps (`level_exceptions_keymap` then `state_keymap`) and emits a
    /// Unicode keysym so that the serialized string preserves character data.
    fn resolve_keysym(&self, layout: usize, level: usize, evdev: u32) -> u32 {
        let layout = &self.layouts[layout];
        let nk = layout.named_key_map.get(level, evdev);
        let sym = named_key_to_keysym(nk);
        if sym != 0 {
            return sym;
        }
        // Unnamed key — recover from character keymaps.
        if let Some(ch) = layout.level_exceptions_keymap.get(level, evdev) {
            return 0x0100_0000 | ch as u32;
        }
        if let Some(ch) = layout.state_keymap.get(level, evdev) {
            return 0x0100_0000 | ch as u32;
        }
        // Modifier key — derive keysym from the modifier map so that the
        // re-parsed keymap retains the correct modifier associations.
        if level == 0 {
            if let Some(ks) = modifier_keysym(&layout.modifiers, evdev) {
                return ks;
            }
        }
        0
    }

    fn write_keycodes(&self, out: &mut String, max_evdev: u32) {
        use std::fmt::Write;

        out.push_str("xkb_keycodes \"wkb\" {\n");
        writeln!(out, "\tminimum = 8;").unwrap();
        writeln!(out, "\tmaximum = {};", max_evdev + 8 - 1).unwrap();

        for evdev in 0..max_evdev {
            // Only emit keys that have at least one keysym
            if self.layouts.iter().any(|layout| {
                key_max_level(
                    &layout.named_key_map,
                    &layout.level_exceptions_keymap,
                    &layout.modifiers,
                    evdev,
                ) > 0
            }) {
                let name = evdev_to_keyname(evdev);
                writeln!(out, "\t<{}> = {};", name, evdev + 8).unwrap();
            }
        }

        // LED indicators
        out.push_str("\tindicator 1 = \"Caps Lock\";\n");
        out.push_str("\tindicator 2 = \"Num Lock\";\n");
        out.push_str("\tindicator 3 = \"Scroll Lock\";\n");
        out.push_str("};\n\n");
    }

    fn write_symbols(&self, out: &mut String, max_evdev: u32, num_layouts: usize) {
        use std::fmt::Write;

        out.push_str("xkb_symbols \"wkb\" {\n");

        // Group names
        for (i, layout) in self.layouts.iter().enumerate() {
            writeln!(out, "\tname[{}]= \"{}\";", i + 1, layout.name).unwrap();
        }
        out.push('\n');

        // Per-key symbols
        for evdev in 0..max_evdev {
            let max_level = self
                .layouts
                .iter()
                .map(|layout| {
                    key_max_level(
                        &layout.named_key_map,
                        &layout.level_exceptions_keymap,
                        &layout.modifiers,
                        evdev,
                    )
                })
                .max()
                .unwrap_or(0);
            if max_level == 0 {
                continue;
            }
            let name = evdev_to_keyname(evdev);
            let type_name = if max_level == 2
                && self
                    .layouts
                    .iter()
                    .any(|layout| is_alphabetic(&layout.state_keymap, evdev))
            {
                "ALPHABETIC"
            } else {
                type_for_levels(max_level)
            };

            if num_layouts == 1 {
                // Single-group format
                write!(out, "\tkey <{}> {{ type= \"{}\",", name, type_name).unwrap();
                out.push_str(" [ ");
                for level in 0..max_level {
                    if level > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&sym_name(self.resolve_keysym(0, level, evdev)));
                }
                out.push_str(" ]");
                // repeat
                if self.layouts[0].repeat_keys.contains(evdev) {
                    out.push_str(", repeat=Yes");
                }
                out.push_str(" };\n");
            } else {
                // Multi-group format
                out.push_str("\tkey <");
                out.push_str(&name);
                out.push_str("> {\n");
                // Per-group types
                for g in 0..num_layouts {
                    let layout = &self.layouts[g];
                    let glevel = key_max_level(
                        &layout.named_key_map,
                        &layout.level_exceptions_keymap,
                        &layout.modifiers,
                        evdev,
                    );
                    let gt = if glevel == 2 && is_alphabetic(&layout.state_keymap, evdev) {
                        "ALPHABETIC"
                    } else {
                        type_for_levels(glevel)
                    };
                    writeln!(out, "\t\ttype[group{}]= \"{}\",", g + 1, gt).unwrap();
                }
                // Per-group symbols
                for g in 0..num_layouts {
                    write!(out, "\t\tsymbols[{}]= [ ", g + 1).unwrap();
                    for level in 0..max_level {
                        if level > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(&sym_name(self.resolve_keysym(g, level, evdev)));
                    }
                    if g < num_layouts - 1 {
                        out.push_str(" ],\n");
                    } else {
                        out.push_str(" ]");
                    }
                }
                if self
                    .layouts
                    .iter()
                    .any(|layout| layout.repeat_keys.contains(evdev))
                {
                    out.push_str(",\n\t\trepeat=Yes");
                }
                out.push('\n');
                out.push_str("\t};\n");
            }
        }

        // modifier_map entries
        out.push_str("\tmodifier_map Shift { <I050>, <I062> };\n");
        out.push_str("\tmodifier_map Lock { <I066> };\n");
        out.push_str("\tmodifier_map Control { <I037>, <I105> };\n");
        out.push_str("\tmodifier_map Mod1 { <I064> };\n");
        out.push_str("\tmodifier_map Mod2 { <I077> };\n");
        out.push_str("\tmodifier_map Mod4 { <I133> };\n");
        out.push_str("\tmodifier_map Mod5 { <I092> };\n");
        out.push_str("};\n\n");
    }
}

const TYPES_XKB: &str = "xkb_types \"wkb\" {\n\tvirtual_modifiers NumLock,Alt,LevelThree,LevelFive;\n\n\ttype \"ONE_LEVEL\" {\n\t\tmodifiers= none;\n\t\tlevel_name[Level1]= \"Any\";\n\t};\n\ttype \"TWO_LEVEL\" {\n\t\tmodifiers= Shift;\n\t\tmap[Shift]= Level2;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t};\n\ttype \"ALPHABETIC\" {\n\t\tmodifiers= Shift+Lock;\n\t\tmap[Shift]= Level2;\n\t\tmap[Lock]= Level2;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t};\n\ttype \"FOUR_LEVEL\" {\n\t\tmodifiers= Shift+LevelThree;\n\t\tmap[Shift]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t};\n\ttype \"FOUR_LEVEL_SEMIALPHABETIC\" {\n\t\tmodifiers= Shift+Lock+LevelThree;\n\t\tmap[Shift]= Level2;\n\t\tmap[Lock]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tmap[Lock+LevelThree]= Level3;\n\t\tmap[Shift+Lock+LevelThree]= Level4;\n\t\tpreserve[Lock+LevelThree]= Lock;\n\t\tpreserve[Shift+Lock+LevelThree]= Lock;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t};\n\ttype \"EIGHT_LEVEL\" {\n\t\tmodifiers= Shift+LevelThree+LevelFive;\n\t\tmap[Shift]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tmap[LevelFive]= Level5;\n\t\tmap[Shift+LevelFive]= Level6;\n\t\tmap[LevelThree+LevelFive]= Level7;\n\t\tmap[Shift+LevelThree+LevelFive]= Level8;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t\tlevel_name[Level5]= \"X1\";\n\t\tlevel_name[Level6]= \"X2\";\n\t\tlevel_name[Level7]= \"X3\";\n\t\tlevel_name[Level8]= \"X4\";\n\t};\n};\n";

const COMPAT_XKB: &str = "xkb_compat \"wkb\" {\n\tvirtual_modifiers NumLock,Alt,LevelThree,LevelFive;\n\n\tinterpret Any+AnyOf(all) {\n\t\taction= SetMods(modifiers=modMapMods,clearLocks);\n\t};\n\tinterpret Shift_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Shift,clearLocks);\n\t};\n\tinterpret Shift_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Shift,clearLocks);\n\t};\n\tinterpret Caps_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=Lock);\n\t};\n\tinterpret Num_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=NumLock);\n\t};\n\tinterpret Control_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Control,clearLocks);\n\t};\n\tinterpret Control_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Control,clearLocks);\n\t};\n\tinterpret Alt_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Alt,clearLocks);\n\t};\n\tinterpret Super_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Mod4,clearLocks);\n\t};\n\tinterpret Super_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Mod4,clearLocks);\n\t};\n\tinterpret ISO_Level3_Shift+AnyOf(all) {\n\t\taction= SetMods(modifiers=LevelThree,clearLocks);\n\t};\n\tinterpret Scroll_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=Mod3);\n\t};\n\n\tindicator \"Caps Lock\" {\n\t\tmodifiers= Lock;\n\t};\n\tindicator \"Num Lock\" {\n\t\tmodifiers= NumLock;\n\t};\n\tindicator \"Scroll Lock\" {\n\t\tmodifiers= Mod3;\n\t};\n};\n";
