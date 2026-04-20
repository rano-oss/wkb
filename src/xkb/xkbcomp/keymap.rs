use super::prelude::*;
use crate::xkb::shared_ast_types::to_common_or_null;
pub use crate::xkb::shared_ast_types::xkb_file_type_to_string;
pub use crate::xkb::shared_types::{
    areOverlappingOverlaysSupported, format_max_groups, format_max_overlays,
    isGroupLockOnReleaseSupported, isModsLatchOnPressSupported, isModsUnLockOnPressSupported,
    real_mod_index, MAX_ACTIONS_PER_LEVEL, XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MAX_GROUPS_X11,
    XKB_MOD_INDEX_CAPS, XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1, XKB_MOD_INDEX_MOD2,
    XKB_MOD_INDEX_MOD3, XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT,
    XKB_OVERLAY_MAX, XKB_OVERLAY_MAX_X11, _XKB_MOD_INDEX_NUM_ENTRIES,
};
use crate::xkb::shared_types::{MOD_REAL_MASK_ALL, XKB_KEYMAP_FORMAT_TEXT_V1};
pub use crate::xkb::state::mod_mask_get_effective;
use crate::xkb::text::{format_control_names_offset, GROUP_LAST_INDEX_NAME};

pub use crate::xkb::xkbcomp::ast_build::FreeStmt;
pub use crate::xkb::xkbcomp::compat::CompileCompatMap;
use crate::xkb::xkbcomp::expr::ExprResolveGroupMask;
pub use crate::xkb::xkbcomp::keycodes::CompileKeycodes;
pub use crate::xkb::xkbcomp::symbols::CompileSymbols;
pub use crate::xkb::xkbcomp::types::CompileKeyTypes;

pub const GROUP_MASK_NAME_LAST: u32 = 3;
pub const GROUP_INDEX_NAME_LAST: u32 = 1;
pub type compile_file_fn =
    Option<for<'a> fn(Option<&mut XkbFile>, &mut xkb_keymap_info<'a>) -> bool>;
#[inline]
fn ComputeEffectiveMask(keymap: &xkb_keymap, mods: &mut xkb_mods) {
    let unknown_mods: u32 = !((1_u64 << keymap.mods.num_mods).wrapping_sub(1_u64) as u32);
    mods.mask = mod_mask_get_effective(keymap, mods.mods) | mods.mods & unknown_mods;
}
/// Version that takes the mod_set separately to allow calling on fields of keymap.
#[inline]
fn compute_effective_mask_with(mod_set: &xkb_mod_set, mods: &mut xkb_mods) {
    let unknown_mods: u32 = !((1_u64 << mod_set.num_mods).wrapping_sub(1_u64) as u32);
    // Inline mod_mask_get_effective logic
    let mut mask: u32 = mods.mods & MOD_REAL_MASK_ALL;
    let mut i: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
    while i < mod_set.num_mods {
        if mods.mods & (1u32 << i) != 0 {
            mask |= mod_set.mods[i as usize].mapping;
        }
        i += 1;
    }
    mods.mask = mask | mods.mods & unknown_mods;
}
fn UpdateActionMods(keymap: &xkb_keymap, act: &mut xkb_action, modmap: u32) {
    if let 2..=4 = act.action_type() {
        if act.as_mods().flags & ACTION_MODS_LOOKUP_MODMAP != 0 {
            act.as_mods_mut().mods.mods = modmap;
        }
        ComputeEffectiveMask(keymap, &mut act.as_mods_mut().mods);
    };
}
fn default_interpret() -> xkb_sym_interpret {
    xkb_sym_interpret {
        sym: XKB_KEY_NoSymbol as u32,
        match_0: MATCH_ANY_OR_NONE,
        mods: 0,
        virtual_mod: DEFAULT_INTERPRET_VMOD,
        level_one_only: false,
        repeat: DEFAULT_INTERPRET_KEY_REPEAT as i32 != 0,
        required: false,
        num_actions: 0,
        action: xkb_action::None,
        actions: Vec::new(),
    }
}
/// Returns interp indices into `keymap.sym_interprets`, or `usize::MAX` for default interprets.
fn FindInterpForKey(
    keymap: &mut xkb_keymap,
    key_idx: usize,
    group: u32,
    level: u32,
    interp_indices: &mut Vec<usize>,
) -> bool {
    let keycode = keymap.keys[key_idx].keycode;
    let syms = xkb_keymap_key_get_syms_by_level_ref(keymap, keycode, group, level).to_vec();

    if syms.is_empty() {
        return false;
    }
    let key_modmap = keymap.keys[key_idx].modmap;
    let key_name = keymap.keys[key_idx].name;
    let num_syms = syms.len() as i32;
    let mut s: i32 = 0_i32;
    while s < num_syms {
        let mut c2rust_current_block_34: u64;
        let mut found: bool = false;
        let mut i: u32 = 0_u32;
        's_26: loop {
            if i >= keymap.sym_interprets.len() as u32 {
                c2rust_current_block_34 = 7659304154607701039;
                break;
            }
            let interp = &keymap.sym_interprets[i as usize];
            let mods: u32;
            found = false;
            if !(interp.sym != syms[s as usize] && interp.sym != XKB_KEY_NoSymbol as u32) {
                if interp.level_one_only as i32 != 0 && level != 0_u32 {
                    mods = 0_u32;
                } else {
                    mods = key_modmap;
                }
                match interp.match_0 {
                    0 => {
                        found = interp.mods & mods == 0;
                    }
                    1 => {
                        found = mods == 0 || interp.mods & mods != 0;
                    }
                    2 => {
                        found = interp.mods & mods != 0;
                    }
                    3 => {
                        found = interp.mods & mods == interp.mods;
                    }
                    4 => {
                        found = interp.mods == mods;
                    }
                    _ => {}
                }
                if found as i32 != 0
                    && i > 0_u32
                    && interp.sym == XKB_KEY_NoSymbol as u32
                    && !interp_indices.is_empty()
                {
                    for &prev_idx in interp_indices.iter() {
                        if prev_idx == i as usize {
                            found = false;
                            log::warn!("Repeated interpretation ignored for keysym #{} \"{}\" at level {}/group {} on key <{}>.\n",
                                    s + 1_i32,
                                    KeysymText(syms[s as usize]),
                                    level.wrapping_add(1_u32),
                                    group.wrapping_add(1_u32),
                                    xkb_atom_text(&keymap.ctx.atom_table, key_name));
                            c2rust_current_block_34 = 2209838995503123840;
                            break 's_26;
                        }
                    }
                }
                if found {
                    interp_indices.push(i as usize);
                    keymap.sym_interprets[i as usize].required = true;
                    c2rust_current_block_34 = 7659304154607701039;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if c2rust_current_block_34 == 7659304154607701039 {
            if !found {
                c2rust_current_block_34 = 2209838995503123840;
            } else {
                c2rust_current_block_34 = 2989495919056355252;
            }
        }
        if c2rust_current_block_34 == 2209838995503123840 {
            // usize::MAX signals "use default interpret"
            interp_indices.push(usize::MAX);
        }
        s += 1;
    }
    true
}
fn ApplyInterpsToKey(keymap: &mut xkb_keymap, key_idx: usize) -> bool {
    let mut vmodmap: u32 = 0_u32;
    let mut level: u32;
    let mut interp_indices: Vec<usize> = Vec::new();
    let mut actions: Vec<xkb_action> = Vec::new();
    let num_groups = keymap.keys[key_idx].num_groups;
    let mut group: u32 = 0_u32;
    while group < num_groups {
        if !keymap.keys[key_idx].groups[group as usize].explicit_actions {
            level = 0_u32;
            let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], group);
            while level < num_levels {
                interp_indices.clear();
                let found: bool =
                    FindInterpForKey(keymap, key_idx, group, level, &mut interp_indices);
                if found {
                    let default_interp = default_interpret();
                    let key_explicit = keymap.keys[key_idx].explicit;
                    let key_name = keymap.keys[key_idx].name;
                    for &idx in interp_indices.iter() {
                        let interp = if idx == usize::MAX {
                            &default_interp
                        } else {
                            &keymap.sym_interprets[idx]
                        };
                        if group == 0_u32
                            && level == 0_u32
                            && key_explicit & EXPLICIT_REPEAT == 0
                            && interp.repeat
                        {
                            keymap.keys[key_idx].repeats = true;
                        }
                        if (group == 0_u32 && level == 0_u32 || !interp.level_one_only)
                            && interp.virtual_mod != XKB_MOD_INVALID
                        {
                            vmodmap |= 1_u32 << interp.virtual_mod;
                        }
                        match interp.num_actions as i32 {
                            0 => {}
                            1 => {
                                actions.push(interp.action);
                            }
                            _ => {
                                actions.extend_from_slice(&interp.actions);
                            }
                        }
                    }
                    if (actions.len() as u32 != 0) as i64 > MAX_ACTIONS_PER_LEVEL as i64 {
                        log::warn!("Could not append interpret actions to key <{}>: maximum is {}, got: {}. Dropping excessive actions\n",
                            xkb_atom_text(&keymap.ctx.atom_table, key_name),
                            65535_i32,
                            actions.len() as u32);
                        actions.truncate(MAX_ACTIONS_PER_LEVEL as usize);
                    }
                    keymap.keys[key_idx].groups[group as usize].levels[level as usize].actions =
                        actions.clone();
                    if !actions.is_empty() {
                        keymap.keys[key_idx].groups[group as usize].implicit_actions = true;
                    }
                    actions.clear();
                }
                level = level.wrapping_add(1);
            }
            if keymap.keys[key_idx].groups[group as usize].implicit_actions {
                keymap.keys[key_idx].implicit_actions = true;
            }
        }
        group = group.wrapping_add(1);
    }
    if keymap.keys[key_idx].explicit & EXPLICIT_VMODMAP == 0 {
        keymap.keys[key_idx].vmodmap = vmodmap;
    }
    true
}
#[inline]
fn is_mod_action(action: &xkb_action) -> bool {
    action.action_type() == ACTION_TYPE_MOD_SET
        || action.action_type() == ACTION_TYPE_MOD_LATCH
        || action.action_type() == ACTION_TYPE_MOD_LOCK
}
#[inline]
fn is_group_action(action: &xkb_action) -> bool {
    action.action_type() == ACTION_TYPE_GROUP_SET
        || action.action_type() == ACTION_TYPE_GROUP_LATCH
        || action.action_type() == ACTION_TYPE_GROUP_LOCK
}
fn CheckMultipleActionsCategories(keymap: &mut xkb_keymap, key_idx: usize) {
    let num_groups = keymap.keys[key_idx].num_groups;
    let key_name = keymap.keys[key_idx].name;
    let mut g: u32 = 0_u32;
    while g < num_groups {
        let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], g);
        let mut l: u32 = 0_u32;
        while l < num_levels {
            let level: &mut xkb_level =
                &mut keymap.keys[key_idx].groups[g as usize].levels[l as usize];
            if level.actions.len() > 1 {
                let mut i: u16 = 0_u16;
                while (i as usize) < level.actions.len() {
                    let mod_action: bool = is_mod_action(&level.actions[i as usize]);
                    let group_action: bool = is_group_action(&level.actions[i as usize]);
                    let action1_type = level.actions[i as usize].action_type();
                    if mod_action as i32 != 0
                        || group_action as i32 != 0
                        || action1_type == ACTION_TYPE_REDIRECT_KEY
                    {
                        let mut j: u16 = (i as i32 + 1_i32) as u16;
                        while (j as usize) < level.actions.len() {
                            if action1_type == level.actions[j as usize].action_type()
                                || mod_action as i32 != 0
                                    && is_mod_action(&level.actions[j as usize]) as i32 != 0
                                || group_action as i32 != 0
                                    && is_group_action(&level.actions[j as usize]) as i32 != 0
                            {
                                let type_0: &str = if mod_action as i32 != 0 {
                                    "modifiers"
                                } else if group_action as i32 != 0 {
                                    "group"
                                } else {
                                    ActionTypeText(action1_type)
                                };
                                log::error!("Cannot use multiple {} actions in the same level. Action #{} for key <{}> in group {}/level {} ignored.\n",
                                    type_0,
                                    j as i32 + 1_i32,
                                    xkb_atom_text(&keymap.ctx.atom_table, key_name),
                                    g.wrapping_add(1_u32),
                                    l.wrapping_add(1_u32));
                                level.actions[j as usize].set_none();
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            l = l.wrapping_add(1);
        }
        g = g.wrapping_add(1);
    }
}
fn add_key_aliases(keymap: &xkb_keymap, min: u32, max: u32, aliases: &mut Vec<xkb_key_alias>) {
    let mut alias: u32 = min;
    while alias <= max {
        let entry: KeycodeMatch = keymap.key_names[alias as usize];
        if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
            aliases.push(xkb_key_alias {
                real: entry.index,
                alias,
            });
        }
        alias = alias.wrapping_add(1);
    }
}
fn update_pending_key_fields(info: &mut xkb_keymap_info<'_>, key_idx: usize) -> bool {
    if info.keymap.keys[key_idx].out_of_range_pending_group {
        let idx = info.keymap.keys[key_idx].out_of_range_group_number as usize;
        if !info.pending_computations[idx].computed {
            // Extract raw pointer to expr to avoid holding a borrow of info
            let expr_ptr = info.pending_computations[idx].expr.raw();
            let expr_ref = unsafe { &*expr_ptr };
            let mut group: u32 = 0_u32;
            let mut pending_dummy = false;
            match ExprResolveGroup(info, expr_ref, true, &mut group, &mut pending_dummy) as u32 {
                0 => {
                    info.pending_computations[idx].computed = true;
                    info.pending_computations[idx].value = group.wrapping_sub(1_u32);
                }
                2 => {
                    log::error!("[XKB-{:03}] Invalid key redirect group index\n", {
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                    });
                    return info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0;
                }
                _ => {}
            }
        }
        info.keymap.keys[key_idx].out_of_range_pending_group = false;
        info.keymap.keys[key_idx].out_of_range_group_number = info.pending_computations[idx].value;
    }
    true
}
fn update_pending_action_fields(
    info: &mut xkb_keymap_info<'_>,
    keycode: u32,
    act: &mut xkb_action,
) -> bool {
    match act.action_type() {
        5..=7 => {
            if act.as_group().flags & ACTION_PENDING_COMPUTATION != 0 {
                let pc: &mut pending_computation =
                    &mut info.pending_computations[act.as_group().group as usize];
                if !pc.computed {
                    let mut group: u32 = 0_u32;
                    let absolute: bool = act.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0;
                    let mut pending_dummy = false;
                    let expr_ref = unsafe { &*pc.expr.raw() };
                    match ExprResolveGroup(info, expr_ref, absolute, &mut group, &mut pending_dummy)
                        as u32
                    {
                        2 => {
                            log::error!("[XKB-{:03}] Invalid action group index\n", {
                                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                            });
                            return false;
                        }
                        1 => {}
                        _ => {
                            let pc = &mut info.pending_computations[act.as_group().group as usize];
                            pc.computed = true;
                            if absolute {
                                pc.value = group.wrapping_sub(1_u32);
                            } else {
                                pc.value = group;
                                if unsafe { (*pc.expr.raw()).common.type_0 } == STMT_EXPR_NEGATE {
                                    pc.value = -(pc.value as i32) as u32;
                                }
                            }
                        }
                    }
                }
                let pc = &info.pending_computations[act.as_group().group as usize];
                act.as_group_mut().group = pc.value as i32;
                act.as_group_mut().flags = (act.as_group().flags
                    & !(ACTION_PENDING_COMPUTATION as i32) as u32)
                    as xkb_action_flags;
            }
            true
        }
        16 => {
            if keycode == XKB_KEYCODE_INVALID
                || act.as_redirect().keycode != info.keymap.redirect_key_auto
            {
                return true;
            } else {
                act.as_redirect_mut().keycode = keycode;
            }
            true
        }
        _ => true,
    }
}
fn UpdateDerivedKeymapFields(info: &mut xkb_keymap_info<'_>) -> bool {
    let keymap: &mut xkb_keymap = &mut *info.keymap;
    let mut num_key_aliases: u32 = 0_u32;
    let mut min_alias: u32 = 0_u32;
    let mut max_alias: u32 = 0_u32;
    let mut alias: u32 = 0_u32;
    while (alias as usize) < keymap.key_names.len() {
        let entry: KeycodeMatch = keymap.key_names[alias as usize];
        if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
            if num_key_aliases == 0 {
                min_alias = alias;
            }
            max_alias = alias;
            num_key_aliases = num_key_aliases.wrapping_add(1);
        }
        alias = alias.wrapping_add(1);
    }
    if num_key_aliases != 0 {
        let mut aliases: Vec<xkb_key_alias> = Vec::with_capacity(num_key_aliases as usize);
        add_key_aliases(keymap, min_alias, max_alias, &mut aliases);
        keymap.key_aliases = aliases;
    }
    // key_names is no longer needed after compilation; drop it
    keymap.key_names = Vec::new();
    {
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            let key_num_groups = keymap.keys[ki as usize].num_groups;
            keymap.num_groups = if keymap.num_groups > key_num_groups {
                keymap.num_groups
            } else {
                key_num_groups
            };
            ki = ki.wrapping_add(1);
        }
    }
    let pending_computations: bool = !info.pending_computations.is_empty();
    if pending_computations {
        let num_groups: u32 = if info.keymap.num_groups != 0 {
            info.keymap.num_groups
        } else {
            1_u32
        };
        info.lookup.groupIndexNames[GROUP_INDEX_NAME_LAST as usize] = LookupEntry {
            name: GROUP_LAST_INDEX_NAME,
            value: num_groups,
        };
        info.lookup.groupMaskNames[GROUP_MASK_NAME_LAST as usize] = LookupEntry {
            name: GROUP_LAST_INDEX_NAME,
            value: 1_u32 << num_groups.wrapping_sub(1_u32),
        };
        let mut i: u32 = 0_u32;
        while (i as usize) < info.keymap.sym_interprets.len() {
            let num_actions = info.keymap.sym_interprets[i as usize].num_actions;
            if num_actions as i32 <= 1_i32 {
                // Extract action, update, write back
                let mut action = info.keymap.sym_interprets[i as usize].action;
                if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, &mut action) {
                    return false;
                }
                info.keymap.sym_interprets[i as usize].action = action;
            } else {
                let mut a: u16 = 0_u16;
                while (a as i32) < num_actions as i32 {
                    let mut action = info.keymap.sym_interprets[i as usize].actions[a as usize];
                    if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, &mut action) {
                        return false;
                    }
                    info.keymap.sym_interprets[i as usize].actions[a as usize] = action;
                    a = a.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    {
        let keymap = &mut *info.keymap;
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            if !ApplyInterpsToKey(keymap, ki as usize) {
                return false;
            }
            CheckMultipleActionsCategories(keymap, ki as usize);
            ki = ki.wrapping_add(1);
        }
    }
    {
        let keymap = &mut *info.keymap;
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            let key_vmodmap = keymap.keys[ki as usize].vmodmap;
            let key_modmap = keymap.keys[ki as usize].modmap;
            let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            while idx < keymap.mods.num_mods {
                if key_vmodmap & 1_u32 << idx != 0 {
                    keymap.mods.mods[idx as usize].mapping |= key_modmap;
                }
                idx = idx.wrapping_add(1);
            }
            ki = ki.wrapping_add(1);
        }
    }
    {
        let keymap = &mut *info.keymap;
        if keymap.format >= XKB_KEYMAP_FORMAT_TEXT_V2 {
            let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            while idx < keymap.mods.num_mods {
                let mask: u32 = 1_u32 << idx;
                if keymap.mods.mods[idx as usize].mapping == 0_u32
                    && keymap.mods.explicit_vmods & mask == 0
                {
                    keymap.mods.mods[idx as usize].mapping = mask;
                    keymap.mods.explicit_vmods |= mask;
                }
                idx = idx.wrapping_add(1);
            }
        }
        let mut extra_canonical_mods: u32 = 0_u32;
        {
            let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            while idx < keymap.mods.num_mods {
                extra_canonical_mods |= keymap.mods.mods[idx as usize].mapping;
                idx = idx.wrapping_add(1);
            }
        }
        keymap.canonical_state_mask |= extra_canonical_mods;
        let mut i_0: u32 = 0_u32;
        while (i_0 as usize) < keymap.types.len() {
            compute_effective_mask_with(&keymap.mods, &mut keymap.types[i_0 as usize].mods);
            let mut j: u32 = 0_u32;
            while j < keymap.types[i_0 as usize].entries.len() as u32 {
                let res = {
                    // has_unbound_vmods inlined
                    let entry_mods = keymap.types[i_0 as usize].entries[j as usize].mods.mods;
                    let mut unbound = false;
                    let mut k: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
                    while k < keymap.mods.num_mods {
                        if entry_mods & 1_u32 << k != 0
                            && keymap.mods.mods[k as usize].mapping == 0_u32
                        {
                            unbound = true;
                            break;
                        }
                        k = k.wrapping_add(1);
                    }
                    unbound
                };
                if res {
                    keymap.types[i_0 as usize].entries[j as usize].mods.mask = 0_u32;
                } else {
                    compute_effective_mask_with(
                        &keymap.mods,
                        &mut keymap.types[i_0 as usize].entries[j as usize].mods,
                    );
                    compute_effective_mask_with(
                        &keymap.mods,
                        &mut keymap.types[i_0 as usize].entries[j as usize].preserve,
                    );
                }
                j = j.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    {
        let start_idx = if info.keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            info.keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < info.keymap.num_keys {
            if !update_pending_key_fields(info, ki as usize) {
                return false;
            }
            let key_num_groups = info.keymap.keys[ki as usize].num_groups;
            let key_modmap = info.keymap.keys[ki as usize].modmap;
            let key_keycode = info.keymap.keys[ki as usize].keycode;
            let mut i_1: u32 = 0_u32;
            while i_1 < key_num_groups {
                let num_levels = {
                    let key = &info.keymap.keys[ki as usize];
                    info.keymap.types[key.groups[i_1 as usize].type_idx as usize].num_levels
                };
                let mut j_0: u32 = 0_u32;
                while j_0 < num_levels {
                    let num_actions = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                        [j_0 as usize]
                        .actions
                        .len();
                    if num_actions <= 1 {
                        if num_actions == 1 {
                            let mut act = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[0];
                            UpdateActionMods(&*info.keymap, &mut act, key_modmap);
                            if (pending_computations as i32 != 0
                                || act.action_type() == ACTION_TYPE_REDIRECT_KEY)
                                && !update_pending_action_fields(info, key_keycode, &mut act)
                            {
                                return false;
                            }
                            info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[0] = act;
                        }
                    } else {
                        let mut k: u16 = 0_u16;
                        while (k as usize) < num_actions {
                            let mut act = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[k as usize];
                            UpdateActionMods(&*info.keymap, &mut act, key_modmap);
                            if (pending_computations as i32 != 0
                                || act.action_type() == ACTION_TYPE_REDIRECT_KEY)
                                && !update_pending_action_fields(info, key_keycode, &mut act)
                            {
                                return false;
                            }
                            info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[k as usize] = act;
                            k = k.wrapping_add(1);
                        }
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                i_1 = i_1.wrapping_add(1);
            }
            ki = ki.wrapping_add(1);
        }
    }
    let keymap = &mut *info.keymap;
    let mut led_idx: u32 = 0_u32;
    while led_idx < keymap.num_leds {
        compute_effective_mask_with(&keymap.mods, &mut keymap.leds[led_idx as usize].mods);
        led_idx = led_idx.wrapping_add(1);
    }
    if pending_computations {
        let mut led_idx: u32 = 0_u32;
        while led_idx < info.keymap.num_leds {
            if info.keymap.leds[led_idx as usize].pending_groups {
                let groups_idx = info.keymap.leds[led_idx as usize].groups as usize;
                if !info.pending_computations[groups_idx].computed {
                    let expr_ref = unsafe { &*info.pending_computations[groups_idx].expr.raw() };
                    let mut mask: u32 = 0_u32;
                    let mut pending_dummy = false;
                    if !ExprResolveGroupMask(info, expr_ref, &mut mask, &mut pending_dummy) {
                        log::error!("[XKB-{:03}] Invalid LED group mask\n", {
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                        });
                        return false;
                    }
                    info.pending_computations[groups_idx].computed = true;
                    info.pending_computations[groups_idx].value = mask;
                }
                let value = info.pending_computations[groups_idx].value;
                info.keymap.leds[led_idx as usize].pending_groups = false;
                info.keymap.leds[led_idx as usize].groups = value;
            }
            led_idx = led_idx.wrapping_add(1);
        }
    }
    true
}
static COMPILE_FILE_FNS: [compile_file_fn; 4] = {
    [
        Some(CompileKeycodes as for<'a> fn(Option<&mut XkbFile>, &mut xkb_keymap_info<'a>) -> bool),
        Some(CompileKeyTypes as for<'a> fn(Option<&mut XkbFile>, &mut xkb_keymap_info<'a>) -> bool),
        Some(
            CompileCompatMap as for<'a> fn(Option<&mut XkbFile>, &mut xkb_keymap_info<'a>) -> bool,
        ),
        Some(CompileSymbols as for<'a> fn(Option<&mut XkbFile>, &mut xkb_keymap_info<'a>) -> bool),
    ]
};
fn pending_computations_array_free(p: &mut Vec<pending_computation>) {
    for pc in p.iter_mut() {
        if let Some(boxed) = pc.expr.take() {
            FreeStmt(to_common_or_null!(Box::into_raw(boxed)));
        }
    }
    p.clear();
}
pub fn CompileKeymap(file: &mut XkbFile, keymap: &mut xkb_keymap) -> bool {
    let mut file_indices: [Option<usize>; 4] = [None; 4];
    for (idx, stmt) in file.defs.iter().enumerate() {
        if let Statement::XkbFile(ref sub_file) = stmt {
            if sub_file.file_type < FIRST_KEYMAP_FILE_TYPE
                || sub_file.file_type > LAST_KEYMAP_FILE_TYPE
            {
                if sub_file.file_type == FILE_TYPE_GEOMETRY {
                    log::warn!(
                        "[XKB-{:03}] Geometry sections are not supported; ignoring\n",
                        XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION as i32
                    );
                } else {
                    log::error!(
                        "Cannot define {} in a keymap file\n",
                        xkb_file_type_to_string(sub_file.file_type)
                    );
                }
            } else if file_indices[sub_file.file_type as usize].is_some() {
                log::error!("More than one {} section in keymap file; All sections after the first ignored\n",
                    xkb_file_type_to_string(sub_file.file_type));
            } else {
                file_indices[sub_file.file_type as usize] = Some(idx);
            }
        }
    }
    let km_format = keymap.format;
    let km_flags = keymap.flags;
    let km_num_groups = keymap.num_groups;
    let mut info = xkb_keymap_info {
        keymap,
        strict: (if km_format == XKB_KEYMAP_FORMAT_TEXT_V1 {
            if km_flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
                PARSER_V1_STRICT_FLAGS as i32
            } else {
                PARSER_V1_LAX_FLAGS as i32
            }
        } else if km_flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
            PARSER_V2_STRICT_FLAGS as i32
        } else {
            PARSER_V2_LAX_FLAGS as i32
        }) as u32,
        features: XkbcompFeatures {
            max_groups: format_max_groups(km_format),
            max_overlays: format_max_overlays(km_format),
            controls_name_offset: format_control_names_offset(km_format),
            group_lock_on_release: isGroupLockOnReleaseSupported(km_format),
            mods_unlock_on_press: isModsUnLockOnPressSupported(km_format),
            mods_latch_on_press: isModsLatchOnPressSupported(km_format),
            overlapping_overlays: areOverlappingOverlaysSupported(km_format),
        },
        lookup: XkbcompLookup {
            groupIndexNames: [
                LookupEntry {
                    name: "first",
                    value: 1_u32,
                },
                LookupEntry {
                    name: if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    value: km_num_groups,
                },
                LookupEntry {
                    name: "",
                    value: 0_u32,
                },
            ],
            groupMaskNames: [
                LookupEntry {
                    name: "none",
                    value: 0_u32,
                },
                LookupEntry {
                    name: "first",
                    value: 0x1_u32,
                },
                LookupEntry {
                    name: "all",
                    value: XKB_ALL_GROUPS as u32,
                },
                LookupEntry {
                    name: if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    value: if km_num_groups != 0 && km_num_groups <= XKB_MAX_GROUPS as u32 {
                        1_u32 << km_num_groups.wrapping_sub(1_u32)
                    } else {
                        0_u32
                    },
                },
                LookupEntry {
                    name: "",
                    value: 0_u32,
                },
            ],
        },
        pending_computations: Vec::new(),
    };
    let mut type_0: u32 = FIRST_KEYMAP_FILE_TYPE;
    while type_0 <= LAST_KEYMAP_FILE_TYPE {
        if file_indices[type_0 as usize].is_none() {
            log::debug!(
                "Component {} not provided in keymap\n",
                xkb_file_type_to_string(type_0)
            );
        } else {
            let idx = file_indices[type_0 as usize].unwrap();
            let sub_name = if let Statement::XkbFile(ref sub_file) = file.defs[idx] {
                safe_map_name(sub_file)
            } else {
                ""
            };
            log::debug!(
                "Compiling {} \"{}\"\n",
                xkb_file_type_to_string(type_0),
                sub_name
            );
        }
        let file_arg: Option<&mut XkbFile> = file_indices[type_0 as usize].map(|idx| {
            if let Statement::XkbFile(ref mut sub_file) = file.defs[idx] {
                &mut **sub_file
            } else {
                unreachable!()
            }
        });
        let ok: bool = COMPILE_FILE_FNS[type_0 as usize].expect("non-null function pointer")(
            file_arg, &mut info,
        ) as bool;
        if !ok {
            log::error!("Failed to compile {}\n", xkb_file_type_to_string(type_0));
            pending_computations_array_free(&mut info.pending_computations);
            return false;
        }
        type_0 += 1;
    }
    let ok_0: bool = UpdateDerivedKeymapFields(&mut info) as bool;
    pending_computations_array_free(&mut info.pending_computations);
    ok_0
}
use crate::xkb::keymap::xkb_keymap_key_get_syms_by_level_ref;
use crate::xkb::shared_types::*;
