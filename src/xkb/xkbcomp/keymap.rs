use super::prelude::*;
pub use crate::xkb::shared_ast_types::xkb_file_type_to_string;
use crate::xkb::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;
pub use crate::xkb::shared_types::{
    areOverlappingOverlaysSupported, format_max_groups, format_max_overlays,
    isGroupLockOnReleaseSupported, isModsLatchOnPressSupported, isModsUnLockOnPressSupported,
    real_mod_index, XkbKeyNumLevels, MAX_ACTIONS_PER_LEVEL, XKB_ALL_GROUPS, XKB_MAX_GROUPS,
    XKB_MAX_GROUPS_X11, XKB_MOD_INDEX_CAPS, XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1,
    XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3, XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5,
    XKB_MOD_INDEX_SHIFT, XKB_OVERLAY_MAX, XKB_OVERLAY_MAX_X11, _XKB_MOD_INDEX_NUM_ENTRIES,
};
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
pub type compile_file_fn = Option<fn(Option<&mut XkbFile>, &mut xkb_keymap_info) -> bool>;
#[inline]
fn ComputeEffectiveMask(keymap: *mut xkb_keymap, mods: *mut xkb_mods) {
    unsafe {
        let unknown_mods: u32 = !((1_u64 << (*keymap).mods.num_mods).wrapping_sub(1_u64) as u32);
        (*mods).mask = mod_mask_get_effective(keymap, (*mods).mods) | (*mods).mods & unknown_mods;
    }
}
fn UpdateActionMods(keymap: *mut xkb_keymap, act: *mut xkb_action, modmap: u32) {
    unsafe {
        if let 2..=4 = (*act).type_0 {
            if (*act).mods.flags & ACTION_MODS_LOOKUP_MODMAP != 0 {
                (*act).mods.mods.mods = modmap;
            }
            ComputeEffectiveMask(keymap, &raw mut (*act).mods.mods);
        };
    }
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
        action: xkb_action {
            type_0: ACTION_TYPE_NONE,
        },
        actions: Vec::new(),
    }
}
fn FindInterpForKey(
    keymap: *mut xkb_keymap,
    key: *const xkb_key,
    group: u32,
    level: u32,
    interprets: &mut Vec<*const xkb_sym_interpret>,
) -> bool {
    unsafe {
        let mut syms: *const u32 = std::ptr::null();

        let num_syms: i32 =
            xkb_keymap_key_get_syms_by_level(keymap, (*key).keycode, group, level, &raw mut syms);
        if num_syms <= 0_i32 {
            return false;
        }
        let mut s: i32 = 0_i32;
        while s < num_syms {
            let mut c2rust_current_block_34: u64;
            let mut found: bool = false;
            let mut i: u32 = 0_u32;
            's_26: loop {
                if i >= (*keymap).sym_interprets.len() as u32 {
                    c2rust_current_block_34 = 7659304154607701039;
                    break;
                }
                let interp: *mut xkb_sym_interpret =
                    &mut (&mut (*keymap).sym_interprets)[i as usize] as *mut xkb_sym_interpret;
                let mods: u32;
                found = false;
                if !((*interp).sym != *syms.offset(s as isize)
                    && (*interp).sym != XKB_KEY_NoSymbol as u32)
                {
                    if (*interp).level_one_only as i32 != 0 && level != 0_u32 {
                        mods = 0_u32;
                    } else {
                        mods = (*key).modmap;
                    }
                    match (*interp).match_0 {
                        0 => {
                            found = (*interp).mods & mods == 0;
                        }
                        1 => {
                            found = mods == 0 || (*interp).mods & mods != 0;
                        }
                        2 => {
                            found = (*interp).mods & mods != 0;
                        }
                        3 => {
                            found = (*interp).mods & mods == (*interp).mods;
                        }
                        4 => {
                            found = (*interp).mods == mods;
                        }
                        _ => {}
                    }
                    if found as i32 != 0
                        && i > 0_u32
                        && (*interp).sym == XKB_KEY_NoSymbol as u32
                        && !interprets.is_empty()
                    {
                        for previous in interprets.iter() {
                            if std::ptr::eq(*previous, interp) {
                                found = false;
                                log::warn!("Repeated interpretation ignored for keysym #{} \"{}\" at level {}/group {} on key <{}>.\n",
                                        s + 1_i32,
                                        KeysymText(*syms.offset(s as isize)),
                                        level.wrapping_add(1_u32),
                                        group.wrapping_add(1_u32),
                                        xkb_atom_text(&(*keymap).ctx.atom_table, (*key).name));
                                c2rust_current_block_34 = 2209838995503123840;
                                break 's_26;
                            }
                        }
                    }
                    if found {
                        interprets.push(interp as *const xkb_sym_interpret);
                        (*interp).required = true;
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
                interprets
                    .push(&*Box::leak(Box::new(default_interpret())) as *const xkb_sym_interpret);
            }
            s += 1;
        }
        true
    }
}
fn ApplyInterpsToKey(keymap: *mut xkb_keymap, key: *mut xkb_key) -> bool {
    unsafe {
        let mut vmodmap: u32 = 0_u32;
        let mut level: u32;
        let mut interprets: Vec<*const xkb_sym_interpret> = Vec::new();
        let mut actions: Vec<xkb_action> = Vec::new();
        let mut group: u32 = 0_u32;
        while group < (*key).num_groups {
            if !(&(*key).groups)[group as usize].explicit_actions {
                level = 0_u32;
                while level < XkbKeyNumLevels(keymap, key, group) {
                    let mut interp: *const xkb_sym_interpret;
                    interprets.clear();
                    let found: bool = FindInterpForKey(keymap, key, group, level, &mut interprets);
                    if found {
                        for &interp_ptr in interprets.iter() {
                            interp = interp_ptr;
                            if group == 0_u32
                                && level == 0_u32
                                && (*key).explicit & EXPLICIT_REPEAT == 0
                                && (*interp).repeat
                            {
                                (*key).repeats = true;
                            }
                            if (group == 0_u32 && level == 0_u32 || !(*interp).level_one_only)
                                && (*interp).virtual_mod != XKB_MOD_INVALID
                            {
                                vmodmap |= 1_u32 << (*interp).virtual_mod;
                            }
                            match (*interp).num_actions as i32 {
                                0 => {}
                                1 => {
                                    actions.push((*interp).action);
                                }
                                _ => {
                                    actions.extend_from_slice(&(*interp).actions);
                                }
                            }
                        }
                        if (actions.len() as u32 != 0) as i64 > MAX_ACTIONS_PER_LEVEL as i64 {
                            log::warn!("Could not append interpret actions to key <{}>: maximum is {}, got: {}. Dropping excessive actions\n",
                                xkb_atom_text(&(*keymap).ctx.atom_table, (*key).name),
                                65535_i32,
                                actions.len() as u32);
                            actions.truncate(MAX_ACTIONS_PER_LEVEL as usize);
                        }
                        (&mut (*key).groups)[group as usize].levels[level as usize].actions =
                            actions.clone();
                        if !actions.is_empty() {
                            let c2rust_fresh1 = &mut (&mut (*key).groups)[group as usize];
                            c2rust_fresh1.implicit_actions = true;
                        }
                        actions.clear();
                    }
                    level = level.wrapping_add(1);
                }
                if (&(*key).groups)[group as usize].implicit_actions {
                    (*key).implicit_actions = true;
                }
            }
            group = group.wrapping_add(1);
        }
        if (*key).explicit & EXPLICIT_VMODMAP == 0 {
            (*key).vmodmap = vmodmap;
        }
        true
    }
}
#[inline]
fn is_mod_action(action: *mut xkb_action) -> bool {
    unsafe {
        (*action).type_0 == ACTION_TYPE_MOD_SET
            || (*action).type_0 == ACTION_TYPE_MOD_LATCH
            || (*action).type_0 == ACTION_TYPE_MOD_LOCK
    }
}
#[inline]
fn is_group_action(action: *mut xkb_action) -> bool {
    unsafe {
        (*action).type_0 == ACTION_TYPE_GROUP_SET
            || (*action).type_0 == ACTION_TYPE_GROUP_LATCH
            || (*action).type_0 == ACTION_TYPE_GROUP_LOCK
    }
}
fn CheckMultipleActionsCategories(keymap: *mut xkb_keymap, key: *mut xkb_key) {
    unsafe {
        let mut g: u32 = 0_u32;
        while g < (*key).num_groups {
            let mut l: u32 = 0_u32;
            while l < XkbKeyNumLevels(keymap, key, g) {
                let level: *mut xkb_level =
                    &mut (&mut (*key).groups)[g as usize].levels[l as usize] as *mut xkb_level;
                if (*level).actions.len() > 1 {
                    let mut i: u16 = 0_u16;
                    while (i as usize) < (*level).actions.len() {
                        let action1: *mut xkb_action =
                            &mut (&mut (*level).actions)[i as usize] as *mut xkb_action;
                        let mod_action: bool = is_mod_action(action1);
                        let group_action: bool = is_group_action(action1);
                        if mod_action as i32 != 0
                            || group_action as i32 != 0
                            || (*action1).type_0 == ACTION_TYPE_REDIRECT_KEY
                        {
                            let mut j: u16 = (i as i32 + 1_i32) as u16;
                            while (j as usize) < (*level).actions.len() {
                                let action2: *mut xkb_action =
                                    &mut (&mut (*level).actions)[j as usize] as *mut xkb_action;
                                if (*action1).type_0 == (*action2).type_0
                                    || mod_action as i32 != 0 && is_mod_action(action2) as i32 != 0
                                    || group_action as i32 != 0
                                        && is_group_action(action2) as i32 != 0
                                {
                                    let type_0: &str = if mod_action as i32 != 0 {
                                        "modifiers"
                                    } else if group_action as i32 != 0 {
                                        "group"
                                    } else {
                                        ActionTypeText((*action1).type_0)
                                    };
                                    log::error!("Cannot use multiple {} actions in the same level. Action #{} for key <{}> in group {}/level {} ignored.\n",
                                        type_0,
                                        j as i32 + 1_i32,
                                        xkb_atom_text(&(*keymap).ctx.atom_table, (*key).name),
                                        g.wrapping_add(1_u32),
                                        l.wrapping_add(1_u32));
                                    (*action2).type_0 = ACTION_TYPE_NONE;
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
}
fn add_key_aliases(keymap: *mut xkb_keymap, min: u32, max: u32, aliases: &mut Vec<xkb_key_alias>) {
    unsafe {
        let mut alias: u32 = min;
        while alias <= max {
            let entry: KeycodeMatch = (&(*keymap).key_names)[alias as usize];
            if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
                aliases.push(xkb_key_alias {
                    real: entry.index,
                    alias,
                });
            }
            alias = alias.wrapping_add(1);
        }
    }
}
fn update_pending_key_fields(info: *mut xkb_keymap_info, key: *mut xkb_key) -> bool {
    unsafe {
        if (*key).out_of_range_pending_group {
            let info_ref = &mut *info;
            let pc: *mut pending_computation = &mut info_ref.pending_computations
                [(*key).out_of_range_group_number as usize]
                as *mut pending_computation;
            if !(*pc).computed {
                let mut group: u32 = 0_u32;
                let mut pending_dummy = false;
                match ExprResolveGroup(
                    info_ref,
                    unsafe { (*pc).expr.as_deref().unwrap() },
                    true,
                    &mut group,
                    &mut pending_dummy,
                ) as u32
                {
                    0 => {
                        (*pc).computed = true;
                        (*pc).value = group.wrapping_sub(1_u32);
                    }
                    2 => {
                        log::error!("[XKB-{:03}] Invalid key redirect group index\n", {
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                        });
                        return (*info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0;
                    }
                    _ => {}
                }
            }
            (*key).out_of_range_pending_group = false;
            (*key).out_of_range_group_number = (*pc).value;
        }
        true
    }
}
fn update_pending_action_fields(
    info: *mut xkb_keymap_info,
    keycode: u32,
    act: *mut xkb_action,
) -> bool {
    unsafe {
        match (*act).type_0 {
            5..=7 => {
                if (*act).group.flags & ACTION_PENDING_COMPUTATION != 0 {
                    let info_ref = &mut *info;
                    let pc: *mut pending_computation = &mut info_ref.pending_computations
                        [(*act).group.group as usize]
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut group: u32 = 0_u32;
                        let absolute: bool = (*act).group.flags & ACTION_ABSOLUTE_SWITCH != 0;
                        let mut pending_dummy = false;
                        match ExprResolveGroup(
                            info_ref,
                            &*(*pc).expr.raw(),
                            absolute,
                            &mut group,
                            &mut pending_dummy,
                        ) as u32
                        {
                            2 => {
                                log::error!("[XKB-{:03}] Invalid action group index\n", {
                                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                                });
                                return false;
                            }
                            1 => {}
                            _ => {
                                (*pc).computed = true;
                                if absolute {
                                    (*pc).value = group.wrapping_sub(1_u32);
                                } else {
                                    (*pc).value = group;
                                    if (*(*pc).expr.raw()).common.type_0 == STMT_EXPR_NEGATE {
                                        (*pc).value = -((*pc).value as i32) as u32;
                                    }
                                }
                            }
                        }
                    }
                    (*act).group.group = (*pc).value as i32;
                    (*act).group.flags = ((*act).group.flags
                        & !(ACTION_PENDING_COMPUTATION as i32) as u32)
                        as xkb_action_flags;
                }
                true
            }
            16 => {
                if keycode == XKB_KEYCODE_INVALID
                    || (*act).redirect.keycode != (*(*info).keymap).redirect_key_auto
                {
                    return true;
                } else {
                    (*act).redirect.keycode = keycode;
                }
                true
            }
            _ => true,
        }
    }
}
fn UpdateDerivedKeymapFields(info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let keymap: *mut xkb_keymap = (*info).keymap;
        let mut num_key_aliases: u32 = 0_u32;
        let mut min_alias: u32 = 0_u32;
        let mut max_alias: u32 = 0_u32;
        let mut alias: u32 = 0_u32;
        while (alias as usize) < (*keymap).key_names.len() {
            let entry: KeycodeMatch = (&(*keymap).key_names)[alias as usize];
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
            (*keymap).key_aliases = aliases;
        }
        // key_names is no longer needed after compilation; drop it
        (*keymap).key_names = Vec::new();
        let mut key: *mut xkb_key;
        {
            let start_idx = if (*keymap).num_keys_low == 0_u32 {
                0_u32
            } else {
                (*keymap).min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < (*keymap).num_keys {
                key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                (*keymap).num_groups = if (*keymap).num_groups > (*key).num_groups {
                    (*keymap).num_groups
                } else {
                    (*key).num_groups
                };
                ki = ki.wrapping_add(1);
            }
        }
        let pending_computations: bool = !(*info).pending_computations.is_empty();
        if pending_computations {
            let num_groups: u32 = if (*keymap).num_groups != 0 {
                (*keymap).num_groups
            } else {
                1_u32
            };
            (*info).lookup.groupIndexNames[GROUP_INDEX_NAME_LAST as usize] = LookupEntry {
                name: GROUP_LAST_INDEX_NAME,
                value: num_groups,
            };
            (*info).lookup.groupMaskNames[GROUP_MASK_NAME_LAST as usize] = LookupEntry {
                name: GROUP_LAST_INDEX_NAME,
                value: 1_u32 << num_groups.wrapping_sub(1_u32),
            };
            let mut i: u32 = 0_u32;
            while (i as usize) < (*keymap).sym_interprets.len() {
                let interp: *mut xkb_sym_interpret =
                    &mut (&mut (*keymap).sym_interprets)[i as usize] as *mut xkb_sym_interpret;
                if (*interp).num_actions as i32 <= 1_i32 {
                    let act: *mut xkb_action = &raw mut (*interp).action;
                    if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, act) {
                        return false;
                    }
                } else {
                    let mut a: u16 = 0_u16;
                    while (a as i32) < (*interp).num_actions as i32 {
                        let act_0: *mut xkb_action =
                            (*interp).actions.as_mut_ptr().offset(a as isize) as *mut xkb_action;
                        if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, act_0) {
                            return false;
                        }
                        a = a.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        {
            let start_idx = if (*keymap).num_keys_low == 0_u32 {
                0_u32
            } else {
                (*keymap).min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < (*keymap).num_keys {
                key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                if !ApplyInterpsToKey(keymap, key) {
                    return false;
                }
                CheckMultipleActionsCategories(keymap, key);
                ki = ki.wrapping_add(1);
            }
        }
        let mut idx: u32;
        let mut mod_0: *mut xkb_mod;
        {
            let start_idx = if (*keymap).num_keys_low == 0_u32 {
                0_u32
            } else {
                (*keymap).min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < (*keymap).num_keys {
                key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
                mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                    .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                    as *mut xkb_mod;
                while idx < (*keymap).mods.num_mods {
                    if (*key).vmodmap & 1_u32 << idx != 0 {
                        (*mod_0).mapping |= (*key).modmap;
                    }
                    idx = idx.wrapping_add(1);
                    mod_0 = mod_0.offset(1);
                }
                ki = ki.wrapping_add(1);
            }
        }
        if (*keymap).format >= XKB_KEYMAP_FORMAT_TEXT_V2 {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                as *mut xkb_mod;
            while idx < (*keymap).mods.num_mods {
                let mask: u32 = 1_u32 << idx;
                if (*mod_0).mapping == 0_u32 && (*keymap).mods.explicit_vmods & mask == 0 {
                    (*mod_0).mapping = mask;
                    (*keymap).mods.explicit_vmods |= mask;
                }
                idx = idx.wrapping_add(1);
                mod_0 = mod_0.offset(1);
            }
        }
        let mut extra_canonical_mods: u32 = 0_u32;
        idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize) as *mut xkb_mod;
        while idx < (*keymap).mods.num_mods {
            extra_canonical_mods |= (*mod_0).mapping;
            idx = idx.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*keymap).canonical_state_mask |= extra_canonical_mods;
        let mut i_0: u32 = 0_u32;
        while (i_0 as usize) < (*keymap).types.len() {
            ComputeEffectiveMask(keymap, &raw mut (&mut (*keymap).types)[i_0 as usize].mods);
            let mut j: u32 = 0_u32;
            while j < (&(*keymap).types)[i_0 as usize].entries.len() as u32 {
                let res = {
                    // has_unbound_vmods inlined
                    let entry_mods = (&(*keymap).types)[i_0 as usize].entries[j as usize]
                        .mods
                        .mods;
                    let mut unbound = false;
                    let mut k: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
                    let mut mod_0: *mut xkb_mod = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                        .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                        as *mut xkb_mod;
                    while k < (*keymap).mods.num_mods {
                        if entry_mods & 1_u32 << k != 0 && (*mod_0).mapping == 0_u32 {
                            unbound = true;
                            break;
                        }
                        k = k.wrapping_add(1);
                        mod_0 = mod_0.offset(1);
                    }
                    unbound
                };
                if res {
                    (&mut (*keymap).types)[i_0 as usize].entries[j as usize]
                        .mods
                        .mask = 0_u32;
                } else {
                    ComputeEffectiveMask(
                        keymap,
                        &raw mut (&mut (*keymap).types)[i_0 as usize].entries[j as usize].mods,
                    );
                    ComputeEffectiveMask(
                        keymap,
                        &raw mut (&mut (*keymap).types)[i_0 as usize].entries[j as usize].preserve,
                    );
                }
                j = j.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
        {
            let start_idx = if (*keymap).num_keys_low == 0_u32 {
                0_u32
            } else {
                (*keymap).min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < (*keymap).num_keys {
                key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                if !update_pending_key_fields(info, key) {
                    return false;
                }
                let mut i_1: u32 = 0_u32;
                while i_1 < (*key).num_groups {
                    let mut j_0: u32 = 0_u32;
                    while j_0 < XkbKeyNumLevels(keymap, key, i_1) {
                        if (&(*key).groups)[i_1 as usize].levels[j_0 as usize]
                            .actions
                            .len()
                            <= 1
                        {
                            if !(&(*key).groups)[i_1 as usize].levels[j_0 as usize]
                                .actions
                                .is_empty()
                            {
                                let act_1: *mut xkb_action = &mut (&mut (*key).groups)[i_1 as usize]
                                    .levels[j_0 as usize]
                                    .actions[0]
                                    as *mut xkb_action;
                                UpdateActionMods(keymap, act_1, (*key).modmap);
                                if (pending_computations as i32 != 0
                                    || (*act_1).type_0 == ACTION_TYPE_REDIRECT_KEY)
                                    && !update_pending_action_fields(info, (*key).keycode, act_1)
                                {
                                    return false;
                                }
                            }
                        } else {
                            let mut k: u16 = 0_u16;
                            while (k as usize)
                                < (&(*key).groups)[i_1 as usize].levels[j_0 as usize]
                                    .actions
                                    .len()
                            {
                                let act_2: *mut xkb_action = &mut (&mut (*key).groups)[i_1 as usize]
                                    .levels[j_0 as usize]
                                    .actions[k as usize]
                                    as *mut xkb_action;
                                UpdateActionMods(keymap, act_2, (*key).modmap);
                                if (pending_computations as i32 != 0
                                    || (*act_2).type_0 == ACTION_TYPE_REDIRECT_KEY)
                                    && !update_pending_action_fields(info, (*key).keycode, act_2)
                                {
                                    return false;
                                }
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
        let mut led: *mut xkb_led;
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while led < (&raw mut (*keymap).leds as *mut xkb_led).offset((*keymap).num_leds as isize) {
            ComputeEffectiveMask(keymap, &raw mut (*led).mods);
            if pending_computations as i32 != 0 && {
                // update_pending_led_fields inlined
                let mut led_ok = true;
                if (*led).pending_groups {
                    let info_ref = &mut *info;
                    let pc: *mut pending_computation = &mut info_ref.pending_computations
                        [(*led).groups as usize]
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut mask: u32 = 0_u32;
                        let mut pending_dummy = false;
                        if !ExprResolveGroupMask(
                            info_ref,
                            &*(*pc).expr.raw(),
                            &mut mask,
                            &mut pending_dummy,
                        ) {
                            log::error!("[XKB-{:03}] Invalid LED group mask\n", {
                                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                            });
                            led_ok = false;
                        }
                        if led_ok {
                            (*pc).computed = true;
                            (*pc).value = mask;
                        }
                    }
                    if led_ok {
                        (*led).pending_groups = false;
                        (*led).groups = (*pc).value;
                    }
                }
                !led_ok
            } {
                return false;
            }
            led = led.offset(1);
        }
        true
    }
}
static COMPILE_FILE_FNS: [compile_file_fn; 4] = {
    [
        Some(CompileKeycodes as fn(Option<&mut XkbFile>, &mut xkb_keymap_info) -> bool),
        Some(CompileKeyTypes as fn(Option<&mut XkbFile>, &mut xkb_keymap_info) -> bool),
        Some(CompileCompatMap as fn(Option<&mut XkbFile>, &mut xkb_keymap_info) -> bool),
        Some(CompileSymbols as fn(Option<&mut XkbFile>, &mut xkb_keymap_info) -> bool),
    ]
};
fn pending_computations_array_free(p: &mut Vec<pending_computation>) {
    unsafe {
        for pc in p.iter_mut() {
            if let Some(boxed) = pc.expr.take() {
                FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
            }
        }
        p.clear();
    }
}
pub fn CompileKeymap(mut file: *mut XkbFile, keymap: *mut xkb_keymap) -> bool {
    unsafe {
        let mut files: [*mut XkbFile; 4] = [
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ];
        let mut type_0: u32;
        let _ctx: *mut xkb_context = &raw mut (*keymap).ctx;
        for stmt in (*file).defs.iter_mut() {
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
                } else if !files[sub_file.file_type as usize].is_null() {
                    log::error!("More than one {} section in keymap file; All sections after the first ignored\n",
                        xkb_file_type_to_string(sub_file.file_type));
                } else {
                    files[sub_file.file_type as usize] =
                        &**sub_file as *const XkbFile as *mut XkbFile;
                }
            }
        }
        let mut info: xkb_keymap_info = xkb_keymap_info {
            keymap,
            strict: (if (*keymap).format == XKB_KEYMAP_FORMAT_TEXT_V1 {
                if (*keymap).flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
                    PARSER_V1_STRICT_FLAGS as i32
                } else {
                    PARSER_V1_LAX_FLAGS as i32
                }
            } else if (*keymap).flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
                PARSER_V2_STRICT_FLAGS as i32
            } else {
                PARSER_V2_LAX_FLAGS as i32
            }) as xkb_parser_strict_flags,
            features: XkbcompFeatures {
                max_groups: format_max_groups((*keymap).format),
                max_overlays: format_max_overlays((*keymap).format),
                controls_name_offset: format_control_names_offset((*keymap).format),
                group_lock_on_release: isGroupLockOnReleaseSupported((*keymap).format),
                mods_unlock_on_press: isModsUnLockOnPressSupported((*keymap).format),
                mods_latch_on_press: isModsLatchOnPressSupported((*keymap).format),
                overlapping_overlays: areOverlappingOverlaysSupported((*keymap).format),
            },
            lookup: XkbcompLookup {
                groupIndexNames: [
                    LookupEntry {
                        name: "first",
                        value: 1_u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME
                        } else {
                            ""
                        },
                        value: (*keymap).num_groups,
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
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME
                        } else {
                            ""
                        },
                        value: if (*keymap).num_groups != 0
                            && (*keymap).num_groups <= XKB_MAX_GROUPS as u32
                        {
                            1_u32 << (*keymap).num_groups.wrapping_sub(1_u32)
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
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        while type_0 <= LAST_KEYMAP_FILE_TYPE {
            if files[type_0 as usize].is_null() {
                log::debug!(
                    "Component {} not provided in keymap\n",
                    xkb_file_type_to_string(type_0)
                );
            } else {
                log::debug!(
                    "Compiling {} \"{}\"\n",
                    xkb_file_type_to_string(type_0),
                    safe_map_name(&*files[type_0 as usize])
                );
            }
            let ok: bool = COMPILE_FILE_FNS[type_0 as usize].expect("non-null function pointer")(
                if files[type_0 as usize].is_null() {
                    None
                } else {
                    Some(&mut *files[type_0 as usize])
                },
                &mut info,
            ) as bool;
            if !ok {
                log::error!("Failed to compile {}\n", xkb_file_type_to_string(type_0));
                // info.keymap is a pointer to the same keymap, no write-back needed
                pending_computations_array_free(&mut info.pending_computations);
                return false;
            }
            type_0 += 1;
        }
        let ok_0: bool = UpdateDerivedKeymapFields(&raw mut info) as bool;
        // info.keymap is a pointer to the same keymap, no write-back needed
        pending_computations_array_free(&mut info.pending_computations);
        ok_0
    }
}
use crate::xkb::keymap::xkb_keymap_key_get_syms_by_level;
use crate::xkb::shared_types::*;
