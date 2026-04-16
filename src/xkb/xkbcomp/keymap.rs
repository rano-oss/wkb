use super::prelude::*;
pub use crate::xkb::shared_ast_types::xkb_file_type_to_string;
use crate::xkb::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;
pub use crate::xkb::shared_types::{
    areOverlappingOverlaysSupported, format_max_groups, format_max_overlays,
    isGroupLockOnReleaseSupported, isModsLatchOnPressSupported, isModsUnLockOnPressSupported,
    real_mod_index, XkbKeyNumLevels, _XKB_MOD_INDEX_NUM_ENTRIES, MAX_ACTIONS_PER_LEVEL,
    XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MAX_GROUPS_X11, XKB_MOD_INDEX_CAPS, XKB_MOD_INDEX_CTRL,
    XKB_MOD_INDEX_MOD1, XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3, XKB_MOD_INDEX_MOD4,
    XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT, XKB_OVERLAY_MAX, XKB_OVERLAY_MAX_X11,
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
pub type compile_file_fn = Option<unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool>;
#[inline]
unsafe fn ComputeEffectiveMask(keymap: *mut xkb_keymap, mods: *mut xkb_mods) {
    unsafe {
        let unknown_mods: u32 =
            !(((1 as u64) << (*keymap).mods.num_mods).wrapping_sub(1 as u64) as u32);
        (*mods).mask = mod_mask_get_effective(keymap, (*mods).mods) | (*mods).mods & unknown_mods;
    }
}
unsafe fn UpdateActionMods(keymap: *mut xkb_keymap, act: *mut xkb_action, modmap: u32) {
    unsafe {
        match (*act).type_0 as u32 {
            2 | 3 | 4 => {
                if (*act).mods.flags as u32 & ACTION_MODS_LOOKUP_MODMAP as u32 != 0 {
                    (*act).mods.mods.mods = modmap;
                }
                ComputeEffectiveMask(keymap, &raw mut (*act).mods.mods);
            }
            _ => {}
        };
    }
}
fn default_interpret() -> xkb_sym_interpret {
    xkb_sym_interpret {
        sym: XKB_KEY_NoSymbol as u32,
        match_0: MATCH_ANY_OR_NONE,
        mods: 0,
        virtual_mod: DEFAULT_INTERPRET_VMOD as u32,
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
unsafe fn FindInterpForKey(
    keymap: *mut xkb_keymap,
    key: *const xkb_key,
    group: u32,
    level: u32,
    interprets: &mut Vec<*const xkb_sym_interpret>,
) -> bool {
    unsafe {
        let mut syms: *const u32 = std::ptr::null();
        let num_syms: i32 ;
        num_syms =
            xkb_keymap_key_get_syms_by_level(keymap, (*key).keycode, group, level, &raw mut syms);
        if num_syms <= 0 as i32 {
            return false;
        }
        let mut s: i32 = 0 as i32;
        while s < num_syms {
            let mut c2rust_current_block_34: u64;
            let mut found: bool = false;
            let mut i: u32 = 0 as u32;
            's_26: loop {
                if !(i < (&(*keymap).sym_interprets).len() as u32) {
                    c2rust_current_block_34 = 7659304154607701039;
                    break;
                }
                let interp: *mut xkb_sym_interpret =
                    &mut (&mut (*keymap).sym_interprets)[i as usize] as *mut xkb_sym_interpret;
                let mods: u32 ;
                found = false;
                if !((*interp).sym != *syms.offset(s as isize)
                    && (*interp).sym != XKB_KEY_NoSymbol as u32)
                {
                    if (*interp).level_one_only as i32 != 0 && level != 0 as u32 {
                        mods = 0 as u32;
                    } else {
                        mods = (*key).modmap;
                    }
                    match (*interp).match_0 as u32 {
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
                    if found as i32 != 0 && i > 0 as u32 && (*interp).sym == XKB_KEY_NoSymbol as u32
                    {
                        if !interprets.is_empty() {
                            for previous in interprets.iter() {
                                if *previous == interp as *const xkb_sym_interpret {
                                    found = false;
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_WARNING,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "Repeated interpretation ignored for keysym #{} \"{}\" at level {}/group {} on key {}.\n",
                                        s + 1 as i32,
                                        crate::xkb::utils::ByteSliceDisplay(KeysymText((*keymap).ctx.clone(), *syms.offset(s as isize))),
                                        level.wrapping_add(1 as u32),
                                        group.wrapping_add(1 as u32),
                                        crate::xkb::utils::ByteSliceDisplay(KeyNameText((*keymap).ctx.clone(), (*key).name)),
                                    );
                                    c2rust_current_block_34 = 2209838995503123840;
                                    break 's_26;
                                }
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
            match c2rust_current_block_34 {
                7659304154607701039 => {
                    if !found {
                        c2rust_current_block_34 = 2209838995503123840;
                    } else {
                        c2rust_current_block_34 = 2989495919056355252;
                    }
                }
                _ => {}
            }
            match c2rust_current_block_34 {
                2209838995503123840 => {
                    interprets.push(
                        &*Box::leak(Box::new(default_interpret())) as *const xkb_sym_interpret
                    );
                }
                _ => {}
            }
            s += 1;
        }
        return true;
    }
}
unsafe fn ApplyInterpsToKey(keymap: *mut xkb_keymap, key: *mut xkb_key) -> bool {
    unsafe {
        let mut vmodmap: u32 = 0 as u32;
        let mut level: u32 ;
        let mut interprets: Vec<*const xkb_sym_interpret> = Vec::new();
        let mut actions: Vec<xkb_action> = Vec::new();
        let mut group: u32 = 0 as u32;
        while group < (*key).num_groups {
            if !(&(*key).groups)[group as usize].explicit_actions {
                level = 0 as u32;
                while level < XkbKeyNumLevels(keymap, key, group) {
                    let mut interp: *const xkb_sym_interpret ;
                    interprets.clear();
                    let found: bool =
                        FindInterpForKey(keymap, key, group, level, &mut interprets) as bool;
                    if found {
                        for &interp_ptr in interprets.iter() {
                            interp = interp_ptr;
                            if group == 0 as u32 && level == 0 as u32 {
                                if (*key).explicit as u32 & EXPLICIT_REPEAT as u32 == 0
                                    && (*interp).repeat
                                {
                                    (*key).repeats = (true) as bool;
                                }
                            }
                            if group == 0 as u32 && level == 0 as u32 || !(*interp).level_one_only {
                                if (*interp).virtual_mod != XKB_MOD_INVALID as u32 {
                                    vmodmap = (vmodmap as u32 | (1 as u32) << (*interp).virtual_mod)
                                        as u32;
                                }
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
                            xkb_logf!(
                                (*keymap).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "Could not append interpret actions to key {}: maximum is {}, got: {}. Dropping excessive actions\n",
                                crate::xkb::utils::ByteSliceDisplay(KeyNameText((*keymap).ctx.clone(), (*key).name)),
                                65535 as i32,
                                actions.len() as u32,
                            );
                            actions.truncate(MAX_ACTIONS_PER_LEVEL as usize);
                        }
                        (&mut (*key).groups)[group as usize].levels[level as usize].actions =
                            actions.clone();
                        if !actions.is_empty() {
                            let ref mut c2rust_fresh1 = (&mut (*key).groups)[group as usize];
                            (*c2rust_fresh1).implicit_actions = true;
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
        if (*key).explicit as u32 & EXPLICIT_VMODMAP as u32 == 0 {
            (*key).vmodmap = vmodmap;
        }
        return true;
    }
}
#[inline]
unsafe fn is_mod_action(action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as u32 == ACTION_TYPE_MOD_SET as u32
            || (*action).type_0 as u32 == ACTION_TYPE_MOD_LATCH as u32
            || (*action).type_0 as u32 == ACTION_TYPE_MOD_LOCK as u32;
    }
}
#[inline]
unsafe fn is_group_action(action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as u32 == ACTION_TYPE_GROUP_SET as u32
            || (*action).type_0 as u32 == ACTION_TYPE_GROUP_LATCH as u32
            || (*action).type_0 as u32 == ACTION_TYPE_GROUP_LOCK as u32;
    }
}
unsafe fn CheckMultipleActionsCategories(keymap: *mut xkb_keymap, key: *mut xkb_key) {
    unsafe {
        let mut g: u32 = 0 as u32;
        while g < (*key).num_groups {
            let mut l: u32 = 0 as u32;
            while l < XkbKeyNumLevels(keymap, key, g) {
                let level: *mut xkb_level =
                    &mut (&mut (*key).groups)[g as usize].levels[l as usize] as *mut xkb_level;
                if !((*level).actions.len() <= 1) {
                    let mut i: u16 = 0 as u16;
                    while (i as usize) < (*level).actions.len() {
                        let action1: *mut xkb_action =
                            &mut (&mut (*level).actions)[i as usize] as *mut xkb_action;
                        let mod_action: bool = is_mod_action(action1);
                        let group_action: bool = is_group_action(action1);
                        if mod_action as i32 != 0
                            || group_action as i32 != 0
                            || (*action1).type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32
                        {
                            let mut j: u16 = (i as i32 + 1 as i32) as u16;
                            while (j as usize) < (*level).actions.len() {
                                let action2: *mut xkb_action =
                                    &mut (&mut (*level).actions)[j as usize] as *mut xkb_action;
                                if (*action1).type_0 as u32 == (*action2).type_0 as u32
                                    || mod_action as i32 != 0 && is_mod_action(action2) as i32 != 0
                                    || group_action as i32 != 0
                                        && is_group_action(action2) as i32 != 0
                                {
                                    let type_0: &[u8] = if mod_action as i32 != 0 {
                                        b"modifiers"
                                    } else if group_action as i32 != 0 {
                                        b"group"
                                    } else {
                                        ActionTypeText((*action1).type_0)
                                    };
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "Cannot use multiple {} actions in the same level. Action #{} for key {} in group {}/level {} ignored.\n",
                                        crate::xkb::utils::ByteSliceDisplay(type_0),
                                        j as i32 + 1 as i32,
                                        crate::xkb::utils::ByteSliceDisplay(KeyNameText((*keymap).ctx.clone(), (*key).name)),
                                        g.wrapping_add(1 as u32),
                                        l.wrapping_add(1 as u32),
                                    );
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
unsafe fn add_key_aliases(
    keymap: *mut xkb_keymap,
    min: u32,
    max: u32,
    aliases: &mut Vec<xkb_key_alias>,
) {
    unsafe {
        let mut alias: u32 = min;
        while alias <= max {
            let entry: KeycodeMatch = (&(*keymap).key_names)[alias as usize];
            if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
                aliases.push(xkb_key_alias {
                    real: entry.index,
                    alias: alias as u32,
                });
            }
            alias = alias.wrapping_add(1);
        }
    }
}
unsafe fn update_pending_key_fields(info: *mut xkb_keymap_info, key: *mut xkb_key) -> bool {
    unsafe {
        if (*key).out_of_range_pending_group {
            let pc: *mut pending_computation = &mut (&mut *(*info).pending_computations)
                [(*key).out_of_range_group_number as usize]
                as *mut pending_computation;
            if !(*pc).computed {
                let mut group: u32 = 0 as u32;
                match ExprResolveGroup(info, (*pc).expr, true, &raw mut group, std::ptr::null_mut())
                    as u32
                {
                    0 => {
                        (*pc).computed = true;
                        (*pc).value = group.wrapping_sub(1 as u32) as u32;
                    }
                    2 => {
                        xkb_logf!(
                            (&raw mut (*(*info).keymap).ctx),
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Invalid key redirect group index\n",
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                        );
                        return (*info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0;
                    }
                    _ => {}
                }
            }
            (*key).out_of_range_pending_group = false;
            (*key).out_of_range_group_number = (*pc).value as u32;
        }
        return true;
    }
}
unsafe fn update_pending_action_fields(
    info: *mut xkb_keymap_info,
    keycode: u32,
    act: *mut xkb_action,
) -> bool {
    unsafe {
        match (*act).type_0 as u32 {
            5 | 6 | 7 => {
                if (*act).group.flags as u32 & ACTION_PENDING_COMPUTATION as u32 != 0 {
                    let pc: *mut pending_computation = &mut (&mut *(*info).pending_computations)
                        [(*act).group.group as usize]
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut group: u32 = 0 as u32;
                        let absolute: bool =
                            (*act).group.flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 != 0;
                        match ExprResolveGroup(
                            info,
                            (*pc).expr,
                            absolute,
                            &raw mut group,
                            std::ptr::null_mut(),
                        ) as u32
                        {
                            2 => {
                                xkb_logf!(
                                    (&raw mut (*(*info).keymap).ctx),
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] Invalid action group index\n",
                                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                                );
                                return false;
                            }
                            1 => {}
                            _ => {
                                (*pc).computed = true;
                                if absolute {
                                    (*pc).value = group.wrapping_sub(1 as u32) as u32;
                                } else {
                                    (*pc).value = group as u32;
                                    if (*(*pc).expr).common.type_0 as u32 == STMT_EXPR_NEGATE as u32
                                    {
                                        (*pc).value = -((*pc).value as i32) as u32;
                                    }
                                }
                            }
                        }
                    }
                    (*act).group.group = (*pc).value as i32;
                    (*act).group.flags = ((*act).group.flags as u32
                        & !(ACTION_PENDING_COMPUTATION as i32) as u32)
                        as xkb_action_flags;
                }
                return true;
            }
            16 => {
                if keycode == XKB_KEYCODE_INVALID as u32
                    || (*act).redirect.keycode != (*(*info).keymap).redirect_key_auto
                {
                    return true;
                } else {
                    (*act).redirect.keycode = keycode;
                }
                return true;
            }
            _ => return true,
        };
    }
}
unsafe fn UpdateDerivedKeymapFields(info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let keymap: *mut xkb_keymap = (*info).keymap;
        let mut num_key_aliases: u32 = 0 as u32;
        let mut min_alias: u32 = 0 as u32;
        let mut max_alias: u32 = 0 as u32;
        let mut alias: u32 = 0 as u32;
        while (alias as usize) < (&(*keymap).key_names).len() {
            let entry: KeycodeMatch = (&(*keymap).key_names)[alias as usize];
            if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
                if num_key_aliases == 0 {
                    min_alias = alias as u32;
                }
                max_alias = alias as u32;
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
        let mut key: *mut xkb_key ;
        {
            let start_idx = if (*keymap).num_keys_low == 0 as u32 {
                0 as u32
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
        let pending_computations: bool = !(&*(*info).pending_computations).is_empty();
        if pending_computations {
            let num_groups: u32 = if (*keymap).num_groups != 0 {
                (*keymap).num_groups
            } else {
                1 as u32
            };
            (*info).lookup.groupIndexNames[GROUP_INDEX_NAME_LAST as usize] = LookupEntry {
                name: GROUP_LAST_INDEX_NAME,
                value: num_groups as u32,
            };
            (*info).lookup.groupMaskNames[GROUP_MASK_NAME_LAST as usize] = LookupEntry {
                name: GROUP_LAST_INDEX_NAME,
                value: (1 as u32) << num_groups.wrapping_sub(1 as u32),
            };
            let mut i: u32 = 0 as u32;
            while (i as usize) < (&(*keymap).sym_interprets).len() {
                let interp: *mut xkb_sym_interpret =
                    &mut (&mut (*keymap).sym_interprets)[i as usize] as *mut xkb_sym_interpret;
                if (*interp).num_actions as i32 <= 1 as i32 {
                    let act: *mut xkb_action = &raw mut (*interp).action;
                    if !update_pending_action_fields(info, XKB_KEYCODE_INVALID as u32, act) {
                        return false;
                    }
                } else {
                    let mut a: u16 = 0 as u16;
                    while (a as i32) < (*interp).num_actions as i32 {
                        let act_0: *mut xkb_action =
                            (*interp).actions.as_mut_ptr().offset(a as isize) as *mut xkb_action;
                        if !update_pending_action_fields(info, XKB_KEYCODE_INVALID as u32, act_0) {
                            return false;
                        }
                        a = a.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        {
            let start_idx = if (*keymap).num_keys_low == 0 as u32 {
                0 as u32
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
        let mut idx: u32 ;
        let mut mod_0: *mut xkb_mod ;
        {
            let start_idx = if (*keymap).num_keys_low == 0 as u32 {
                0 as u32
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
                    if (*key).vmodmap & (1 as u32) << idx != 0 {
                        (*mod_0).mapping |= (*key).modmap;
                    }
                    idx = idx.wrapping_add(1);
                    mod_0 = mod_0.offset(1);
                }
                ki = ki.wrapping_add(1);
            }
        }
        if (*keymap).format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as u32 {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                as *mut xkb_mod;
            while idx < (*keymap).mods.num_mods {
                let mask: u32 = (1 as u32) << idx;
                if (*mod_0).mapping == 0 as u32 && (*keymap).mods.explicit_vmods & mask == 0 {
                    (*mod_0).mapping = mask;
                    (*keymap).mods.explicit_vmods |= mask;
                }
                idx = idx.wrapping_add(1);
                mod_0 = mod_0.offset(1);
            }
        }
        let mut extra_canonical_mods: u32 = 0 as u32;
        idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize) as *mut xkb_mod;
        while idx < (*keymap).mods.num_mods {
            extra_canonical_mods |= (*mod_0).mapping;
            idx = idx.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*keymap).canonical_state_mask |= extra_canonical_mods;
        let mut i_0: u32 = 0 as u32;
        while (i_0 as usize) < (&(*keymap).types).len() {
            ComputeEffectiveMask(keymap, &raw mut (&mut (*keymap).types)[i_0 as usize].mods);
            let mut j: u32 = 0 as u32;
            while j < (&(*keymap).types)[i_0 as usize].entries.len() as u32 {
                if {
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
                        if entry_mods & (1 as u32) << k != 0 && (*mod_0).mapping == 0 as u32 {
                            unbound = true;
                            break;
                        }
                        k = k.wrapping_add(1);
                        mod_0 = mod_0.offset(1);
                    }
                    unbound
                } {
                    (&mut (*keymap).types)[i_0 as usize].entries[j as usize]
                        .mods
                        .mask = 0 as u32;
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
            let start_idx = if (*keymap).num_keys_low == 0 as u32 {
                0 as u32
            } else {
                (*keymap).min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < (*keymap).num_keys {
                key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                if !update_pending_key_fields(info, key) {
                    return false;
                }
                let mut i_1: u32 = 0 as u32;
                while i_1 < (*key).num_groups {
                    let mut j_0: u32 = 0 as u32;
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
                                    || (*act_1).type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32)
                                    && !update_pending_action_fields(info, (*key).keycode, act_1)
                                {
                                    return false;
                                }
                            }
                        } else {
                            let mut k: u16 = 0 as u16;
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
                                    || (*act_2).type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32)
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
        let mut led: *mut xkb_led ;
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while led < (&raw mut (*keymap).leds as *mut xkb_led).offset((*keymap).num_leds as isize) {
            ComputeEffectiveMask(keymap, &raw mut (*led).mods);
            if pending_computations as i32 != 0 && {
                // update_pending_led_fields inlined
                let mut led_ok = true;
                if (*led).pending_groups {
                    let pc: *mut pending_computation = &mut (&mut *(*info).pending_computations)
                        [(*led).groups as usize]
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut mask: u32 = 0 as u32;
                        if !ExprResolveGroupMask(
                            info,
                            (*pc).expr,
                            &raw mut mask,
                            std::ptr::null_mut(),
                        ) {
                            xkb_logf!(
                                (&raw mut (*(*info).keymap).ctx),
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Invalid LED group mask\n",
                                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                            );
                            led_ok = false;
                        }
                        if led_ok {
                            (*pc).computed = true;
                            (*pc).value = mask as u32;
                        }
                    }
                    if led_ok {
                        (*led).pending_groups = (false) as bool;
                        (*led).groups = (*pc).value as u32;
                    }
                }
                !led_ok
            } {
                return false;
            }
            led = led.offset(1);
        }
        return true;
    }
}
static mut compile_file_fns: [compile_file_fn; 4] = {
    [
        Some(CompileKeycodes as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileKeyTypes as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileCompatMap as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileSymbols as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
    ]
};
unsafe fn pending_computations_array_free(p: *mut Vec<pending_computation>) {
    unsafe {
        for pc in (&*p).iter() {
            FreeStmt(pc.expr as *mut ParseCommon);
        }
        (&mut *p).clear();
    }
}
pub unsafe fn CompileKeymap(mut file: *mut XkbFile, keymap: *mut xkb_keymap) -> bool {
    unsafe {
        let mut files: [*mut XkbFile; 4] = [
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ];
        let mut type_0: u32 ;
        let _ctx: *mut xkb_context = &raw mut (*keymap).ctx;
        file = (*file).defs as *mut XkbFile;
        while !file.is_null() {
            if ((*file).file_type as u32) < FIRST_KEYMAP_FILE_TYPE as u32
                || (*file).file_type as u32 > LAST_KEYMAP_FILE_TYPE as u32
            {
                if (*file).file_type as u32 == FILE_TYPE_GEOMETRY as u32 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_BRIEF as i32,
                        "[XKB-{:03}] Geometry sections are not supported; ignoring\n",
                        XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION as i32,
                    );
                } else {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Cannot define {} in a keymap file\n",
                        crate::xkb::utils::CStrDisplay(xkb_file_type_to_string((*file).file_type)),
                    );
                }
            } else if !files[(*file).file_type as usize].is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "More than one {} section in keymap file; All sections after the first ignored\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string((*file).file_type)),
                );
            } else {
                files[(*file).file_type as usize] = file;
            }
            file = (*file).common.next as *mut XkbFile;
        }
        let mut pending_computations: Vec<pending_computation> = Vec::new();
        let mut info: xkb_keymap_info = xkb_keymap_info {
            keymap: keymap,
            strict: (if (*keymap).format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as u32 {
                if (*keymap).flags as u32 & XKB_KEYMAP_COMPILE_STRICT_MODE as u32 != 0 {
                    PARSER_V1_STRICT_FLAGS as i32
                } else {
                    PARSER_V1_LAX_FLAGS as i32
                }
            } else if (*keymap).flags as u32 & XKB_KEYMAP_COMPILE_STRICT_MODE as u32 != 0 {
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
                        name: b"first",
                        value: 1 as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME
                        } else {
                            b""
                        },
                        value: (*keymap).num_groups as u32,
                    },
                    LookupEntry {
                        name: b"",
                        value: 0 as u32,
                    },
                ],
                groupMaskNames: [
                    LookupEntry {
                        name: b"none",
                        value: 0 as u32,
                    },
                    LookupEntry {
                        name: b"first",
                        value: 0x1 as u32,
                    },
                    LookupEntry {
                        name: b"all",
                        value: XKB_ALL_GROUPS as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME
                        } else {
                            b""
                        },
                        value: if (*keymap).num_groups != 0
                            && (*keymap).num_groups <= XKB_MAX_GROUPS as u32
                        {
                            (1 as u32) << (*keymap).num_groups.wrapping_sub(1 as u32)
                        } else {
                            0 as u32
                        },
                    },
                    LookupEntry {
                        name: b"",
                        value: 0 as u32,
                    },
                ],
            },
            pending_computations: &raw mut pending_computations,
        };
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        while type_0 as u32 <= LAST_KEYMAP_FILE_TYPE as u32 {
            if files[type_0 as usize].is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Component {} not provided in keymap\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(type_0)),
                );
            } else {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Compiling {} \"{}\"\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(type_0)),
                    crate::xkb::utils::CStrDisplay(safe_map_name(files[type_0 as usize])),
                );
            }
            let ok: bool = compile_file_fns[type_0 as usize].expect("non-null function pointer")(
                files[type_0 as usize],
                &raw mut info,
            ) as bool;
            if !ok {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Failed to compile {}\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(type_0)),
                );
                // info.keymap is a pointer to the same keymap, no write-back needed
                pending_computations_array_free(&raw mut pending_computations);
                return false;
            }
            type_0 += 1;
        }
        let ok_0: bool = UpdateDerivedKeymapFields(&raw mut info) as bool;
        // info.keymap is a pointer to the same keymap, no write-back needed
        pending_computations_array_free(&raw mut pending_computations);
        return ok_0;
    }
}
use crate::xkb::keymap::xkb_keymap_key_get_syms_by_level;
use crate::xkb::shared_types::*;
