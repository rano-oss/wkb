use super::prelude::*;
pub use crate::xkb::shared_ast_types::xkb_file_type_to_string;
use crate::xkb::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;
pub use crate::xkb::shared_types::{
    areOverlappingOverlaysSupported, format_max_groups, format_max_overlays,
    isGroupLockOnReleaseSupported, isModsLatchOnPressSupported, isModsUnLockOnPressSupported,
    real_mod_index, C2Rust_Unnamed_15, XkbKeyNumLevels, _XKB_MOD_INDEX_NUM_ENTRIES,
    MAX_ACTIONS_PER_LEVEL, XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MAX_GROUPS_X11, XKB_MOD_INDEX_CAPS,
    XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1, XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3,
    XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT, XKB_OVERLAY_MAX,
    XKB_OVERLAY_MAX_X11,
};
pub use crate::xkb::state::mod_mask_get_effective;
use crate::xkb::text::{format_control_names_offset, GROUP_LAST_INDEX_NAME};
pub use crate::xkb::utils::{is_aligned, memdup};
pub use crate::xkb::xkbcomp::ast_build::FreeStmt;
pub use crate::xkb::xkbcomp::compat::CompileCompatMap;
use crate::xkb::xkbcomp::expr::ExprResolveGroupMask;
pub use crate::xkb::xkbcomp::keycodes::CompileKeycodes;
pub use crate::xkb::xkbcomp::symbols::CompileSymbols;
pub use crate::xkb::xkbcomp::types::CompileKeyTypes;
use libc::realloc;

pub const GROUP_MASK_NAME_LAST: C2Rust_Unnamed_21 = 3;
pub const GROUP_INDEX_NAME_LAST: C2Rust_Unnamed_20 = 1;
pub type compile_file_fn = Option<unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool>;
pub type C2Rust_Unnamed_20 = u32;
pub type C2Rust_Unnamed_21 = u32;
#[inline]
unsafe fn ComputeEffectiveMask(mut keymap: *mut xkb_keymap, mut mods: *mut xkb_mods) {
    unsafe {
        let unknown_mods: xkb_mod_mask_t =
            !(((1 as u64) << (*keymap).mods.num_mods).wrapping_sub(1 as u64) as xkb_mod_mask_t);
        (*mods).mask = mod_mask_get_effective(keymap, (*mods).mods) | (*mods).mods & unknown_mods;
    }
}
unsafe fn UpdateActionMods(
    mut keymap: *mut xkb_keymap,
    mut act: *mut xkb_action,
    mut modmap: xkb_mod_mask_t,
) {
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
static mut default_interpret: xkb_sym_interpret = xkb_sym_interpret {
    sym: 0,
    match_0: MATCH_NONE,
    mods: 0,
    virtual_mod: 0,
    level_one_only: false,
    repeat_required: [0; 1],
    num_actions: 0,
    a: C2Rust_Unnamed_1 {
        action: xkb_action {
            type_0: ACTION_TYPE_NONE,
        },
    },
};
unsafe fn FindInterpForKey(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut group: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    interprets: &mut Vec<*const xkb_sym_interpret>,
) -> bool {
    unsafe {
        let mut syms: *const xkb_keysym_t = std::ptr::null();
        let mut num_syms: i32 = 0;
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
                if !(i < (*keymap).num_sym_interprets) {
                    c2rust_current_block_34 = 7659304154607701039;
                    break;
                }
                let interp: *mut xkb_sym_interpret =
                    (*keymap).sym_interprets.offset(i as isize) as *mut xkb_sym_interpret;
                let mut mods: xkb_mod_mask_t = 0;
                found = false;
                if !((*interp).sym != *syms.offset(s as isize)
                    && (*interp).sym != XKB_KEY_NoSymbol as xkb_keysym_t)
                {
                    if (*interp).level_one_only as i32 != 0 && level != 0 as xkb_level_index_t {
                        mods = 0 as xkb_mod_mask_t;
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
                    if found as i32 != 0
                        && i > 0 as u32
                        && (*interp).sym == XKB_KEY_NoSymbol as xkb_keysym_t
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
                                        crate::xkb::utils::CStrDisplay(KeysymText((*keymap).ctx, *syms.offset(s as isize))),
                                        level.wrapping_add(1 as xkb_level_index_t),
                                        group.wrapping_add(1 as xkb_layout_index_t),
                                        crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*key).name)),
                                    );
                                    c2rust_current_block_34 = 2209838995503123840;
                                    break 's_26;
                                }
                            }
                        }
                    }
                    if found {
                        interprets.push(interp as *const xkb_sym_interpret);
                        (*interp).set_required((true) as bool);
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
                    interprets.push(&raw const default_interpret);
                }
                _ => {}
            }
            s += 1;
        }
        return true;
    }
}
unsafe fn ApplyInterpsToKey(mut keymap: *mut xkb_keymap, mut key: *mut xkb_key) -> bool {
    unsafe {
        let mut vmodmap: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut level: xkb_level_index_t = 0;
        let mut interprets: Vec<*const xkb_sym_interpret> = Vec::new();
        let mut actions: Vec<xkb_action> = Vec::new();
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while group < (*key).num_groups() {
            if !(*(*key).groups.offset(group as isize)).explicit_actions() {
                level = 0 as xkb_level_index_t;
                while level < XkbKeyNumLevels(key, group) {
                    let mut interp: *const xkb_sym_interpret = std::ptr::null();
                    interprets.clear();
                    let found: bool =
                        FindInterpForKey(keymap, key, group, level, &mut interprets) as bool;
                    if found {
                        for &interp_ptr in interprets.iter() {
                            interp = interp_ptr;
                            if group == 0 as xkb_layout_index_t && level == 0 as xkb_level_index_t {
                                if (*key).explicit as u32 & EXPLICIT_REPEAT as u32 == 0
                                    && (*interp).repeat() as i32 != 0
                                {
                                    (*key).set_repeats((true) as bool);
                                }
                            }
                            if group == 0 as xkb_layout_index_t && level == 0 as xkb_level_index_t
                                || !(*interp).level_one_only
                            {
                                if (*interp).virtual_mod != XKB_MOD_INVALID as xkb_mod_index_t {
                                    vmodmap = (vmodmap as u32 | (1 as u32) << (*interp).virtual_mod)
                                        as xkb_mod_mask_t;
                                }
                            }
                            match (*interp).num_actions as i32 {
                                0 => {}
                                1 => {
                                    actions.push((*interp).a.action);
                                }
                                _ => {
                                    actions.extend_from_slice(std::slice::from_raw_parts(
                                        (*interp).a.actions,
                                        (*interp).num_actions as usize,
                                    ));
                                }
                            }
                        }
                        if (actions.len() as u32 != 0) as i64 > MAX_ACTIONS_PER_LEVEL as i64 {
                            xkb_logf!(
                                (*keymap).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "Could not append interpret actions to key {}: maximum is {}, got: {}. Dropping excessive actions\n",
                                crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*key).name)),
                                65535 as i32,
                                actions.len() as u32,
                            );
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_actions = MAX_ACTIONS_PER_LEVEL as xkb_action_count_t;
                        } else {
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_actions = actions.len() as xkb_action_count_t;
                        }
                        match actions.len() as u32 {
                            0 => {
                                (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .action = xkb_action {
                                    type_0: ACTION_TYPE_NONE,
                                };
                            }
                            1 => {
                                (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .action = actions[0];
                            }
                            _ => {
                                let ref mut c2rust_fresh0 =
                                    (*(*(*key).groups.offset(group as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                *c2rust_fresh0 = memdup(
                                    actions.as_ptr() as *const ::core::ffi::c_void,
                                    (*(*(*key).groups.offset(group as isize))
                                        .levels
                                        .offset(level as isize))
                                    .num_actions as usize,
                                    std::mem::size_of::<xkb_action>(),
                                )
                                    as *mut xkb_action;
                                if (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .actions
                                .is_null()
                                {
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "[XKB-{:03}] Could not allocate interpret actions\n",
                                        XKB_ERROR_ALLOCATION_ERROR as i32,
                                    );
                                    return false;
                                }
                            }
                        }
                        if !actions.is_empty() {
                            let ref mut c2rust_fresh1 = *(*key).groups.offset(group as isize);
                            (*c2rust_fresh1).set_implicit_actions((true) as bool);
                        }
                        actions.clear();
                    }
                    level = level.wrapping_add(1);
                }
                if (*(*key).groups.offset(group as isize)).implicit_actions() {
                    (*key).set_implicit_actions((true) as bool);
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
unsafe fn is_mod_action(mut action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as u32 == ACTION_TYPE_MOD_SET as u32
            || (*action).type_0 as u32 == ACTION_TYPE_MOD_LATCH as u32
            || (*action).type_0 as u32 == ACTION_TYPE_MOD_LOCK as u32;
    }
}
#[inline]
unsafe fn is_group_action(mut action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as u32 == ACTION_TYPE_GROUP_SET as u32
            || (*action).type_0 as u32 == ACTION_TYPE_GROUP_LATCH as u32
            || (*action).type_0 as u32 == ACTION_TYPE_GROUP_LOCK as u32;
    }
}
unsafe fn CheckMultipleActionsCategories(mut keymap: *mut xkb_keymap, mut key: *mut xkb_key) {
    unsafe {
        let mut g: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while g < (*key).num_groups() {
            let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
            while l < XkbKeyNumLevels(key, g) {
                let mut level: *mut xkb_level = (*(*key).groups.offset(g as isize))
                    .levels
                    .offset(l as isize)
                    as *mut xkb_level;
                if !((*level).num_actions as i32 <= 1 as i32) {
                    let mut i: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (i as i32) < (*level).num_actions as i32 {
                        let mut action1: *mut xkb_action =
                            (*level).a.actions.offset(i as isize) as *mut xkb_action;
                        let mut mod_action: bool = is_mod_action(action1);
                        let mut group_action: bool = is_group_action(action1);
                        if mod_action as i32 != 0
                            || group_action as i32 != 0
                            || (*action1).type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32
                        {
                            let mut j: xkb_action_count_t =
                                (i as i32 + 1 as i32) as xkb_action_count_t;
                            while (j as i32) < (*level).num_actions as i32 {
                                let mut action2: *mut xkb_action =
                                    (*level).a.actions.offset(j as isize) as *mut xkb_action;
                                if (*action1).type_0 as u32 == (*action2).type_0 as u32
                                    || mod_action as i32 != 0 && is_mod_action(action2) as i32 != 0
                                    || group_action as i32 != 0
                                        && is_group_action(action2) as i32 != 0
                                {
                                    let type_0: *const i8 = if mod_action as i32 != 0 {
                                        b"modifiers\0".as_ptr() as *const i8
                                    } else if group_action as i32 != 0 {
                                        b"group\0".as_ptr() as *const i8
                                    } else {
                                        ActionTypeText((*action1).type_0) as *const i8
                                    };
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "Cannot use multiple {} actions in the same level. Action #{} for key {} in group {}/level {} ignored.\n",
                                        crate::xkb::utils::CStrDisplay(type_0),
                                        j as i32 + 1 as i32,
                                        crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*key).name)),
                                        g.wrapping_add(1 as xkb_layout_index_t),
                                        l.wrapping_add(1 as xkb_level_index_t),
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
    mut keymap: *mut xkb_keymap,
    mut min: u32,
    mut max: u32,
    mut aliases: *mut xkb_key_alias,
) {
    unsafe {
        let mut alias: u32 = min;
        while alias <= max {
            let entry: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(alias as isize);
            if entry.c2rust_unnamed.is_alias() as i32 != 0
                && entry.c2rust_unnamed.found() as i32 != 0
            {
                *aliases = xkb_key_alias {
                    real: entry.alias.real(),
                    alias: alias as xkb_atom_t,
                };
                aliases = aliases.offset(1);
            }
            alias = alias.wrapping_add(1);
        }
    }
}
unsafe fn update_pending_key_fields(mut info: *mut xkb_keymap_info, mut key: *mut xkb_key) -> bool {
    unsafe {
        if (*key).out_of_range_pending_group() {
            let pc: *mut pending_computation = &mut (&mut *(*info).pending_computations)
                [(*key).out_of_range_group_number() as usize]
                as *mut pending_computation;
            if !(*pc).computed {
                let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                match ExprResolveGroup(info, (*pc).expr, true, &raw mut group, std::ptr::null_mut())
                    as u32
                {
                    0 => {
                        (*pc).computed = true;
                        (*pc).value = group.wrapping_sub(1 as xkb_layout_index_t) as u32;
                    }
                    2 => {
                        xkb_logf!(
                            (*info).keymap.ctx,
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
            (*key).set_out_of_range_pending_group((false) as bool);
            (*key).set_out_of_range_group_number(
                (*pc).value as xkb_layout_index_t as xkb_layout_index_t,
            );
        }
        return true;
    }
}
unsafe fn update_pending_action_fields(
    mut info: *mut xkb_keymap_info,
    mut keycode: xkb_keycode_t,
    mut act: *mut xkb_action,
) -> bool {
    unsafe {
        match (*act).type_0 as u32 {
            5 | 6 | 7 => {
                if (*act).group.flags as u32 & ACTION_PENDING_COMPUTATION as u32 != 0 {
                    let pc: *mut pending_computation = &mut (&mut *(*info).pending_computations)
                        [(*act).group.group as usize]
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
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
                                    (*info).keymap.ctx,
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
                                    (*pc).value =
                                        group.wrapping_sub(1 as xkb_layout_index_t) as u32;
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
                if keycode == XKB_KEYCODE_INVALID as xkb_keycode_t
                    || (*act).redirect.keycode != (*info).keymap.redirect_key_auto
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
unsafe fn UpdateDerivedKeymapFields(mut info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let keymap: *mut xkb_keymap = &raw mut (*info).keymap;
        let mut num_key_aliases: u32 = 0 as u32;
        let mut min_alias: u32 = 0 as u32;
        let mut max_alias: u32 = 0 as u32;
        let mut alias: xkb_atom_t = 0 as xkb_atom_t;
        while alias < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
            let entry: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(alias as isize);
            if entry.c2rust_unnamed.is_alias() as i32 != 0
                && entry.c2rust_unnamed.found() as i32 != 0
            {
                if num_key_aliases == 0 {
                    min_alias = alias as u32;
                }
                max_alias = alias as u32;
                num_key_aliases = num_key_aliases.wrapping_add(1);
            }
            alias = alias.wrapping_add(1);
        }
        if num_key_aliases != 0 {
            let required_space: u32 = (std::mem::size_of::<xkb_key_alias>())
                .wrapping_div(std::mem::size_of::<KeycodeMatch>())
                .wrapping_mul(num_key_aliases as usize)
                as u32;
            if min_alias >= required_space {
                add_key_aliases(
                    keymap,
                    min_alias,
                    max_alias,
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases,
                );
                let r: *mut xkb_key_alias = realloc(
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases
                        as *mut ::core::ffi::c_void,
                    (num_key_aliases as usize).wrapping_mul(std::mem::size_of::<xkb_key_alias>()),
                ) as *mut xkb_key_alias;
                if r.is_null() {
                    return false;
                }
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = r;
            } else if (*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .num_key_names
                .wrapping_sub(max_alias)
                .wrapping_sub(1 as u32)
                > required_space
            {
                let aliases: *mut xkb_key_alias = (*keymap)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .key_names
                    .offset(max_alias as isize)
                    .offset(1 as i32 as isize)
                    .offset(!is_aligned(
                        (*keymap)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .key_names
                            .offset(max_alias as isize)
                            .offset(1 as i32 as isize)
                            as *const ::core::ffi::c_void,
                        std::mem::size_of::<xkb_key_alias>(),
                    ) as i32 as isize)
                    as *mut xkb_key_alias;
                add_key_aliases(keymap, min_alias, max_alias, aliases);
                std::ptr::copy_nonoverlapping::<xkb_key_alias>(
                    aliases,
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases,
                    num_key_aliases as usize,
                );
                let r_0: *mut xkb_key_alias = realloc(
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases
                        as *mut ::core::ffi::c_void,
                    (num_key_aliases as usize).wrapping_mul(std::mem::size_of::<xkb_key_alias>()),
                ) as *mut xkb_key_alias;
                if r_0.is_null() {
                    return false;
                }
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = r_0;
            } else {
                let aliases_0: *mut xkb_key_alias = calloc(
                    num_key_aliases as usize,
                    std::mem::size_of::<xkb_key_alias>(),
                ) as *mut xkb_key_alias;
                if aliases_0.is_null() {
                    return false;
                }
                add_key_aliases(keymap, min_alias, max_alias, aliases_0);
                free((*keymap).c2rust_unnamed.c2rust_unnamed.key_names as *mut ::core::ffi::c_void);
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = aliases_0;
            }
        }
        (*keymap).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases = num_key_aliases;
        let mut key: *mut xkb_key = std::ptr::null_mut();
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            (*keymap).num_groups = if (*keymap).num_groups > (*key).num_groups() {
                (*keymap).num_groups
            } else {
                (*key).num_groups()
            };
            key = key.offset(1);
        }
        let pending_computations: bool = !(&*(*info).pending_computations).is_empty();
        if pending_computations {
            let num_groups: xkb_layout_index_t = if (*keymap).num_groups != 0 {
                (*keymap).num_groups
            } else {
                1 as xkb_layout_index_t
            };
            (*info).lookup.groupIndexNames[GROUP_INDEX_NAME_LAST as usize] = LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: num_groups as u32,
            };
            (*info).lookup.groupMaskNames[GROUP_MASK_NAME_LAST as usize] = LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: (1 as u32) << num_groups.wrapping_sub(1 as xkb_layout_index_t),
            };
            let mut i: u32 = 0 as u32;
            while i < (*keymap).num_sym_interprets {
                let interp: *mut xkb_sym_interpret =
                    (*keymap).sym_interprets.offset(i as isize) as *mut xkb_sym_interpret;
                if (*interp).num_actions as i32 <= 1 as i32 {
                    let act: *mut xkb_action = &raw mut (*interp).a.action;
                    if !update_pending_action_fields(
                        info,
                        XKB_KEYCODE_INVALID as xkb_keycode_t,
                        act,
                    ) {
                        return false;
                    }
                } else {
                    let mut a: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (a as i32) < (*interp).num_actions as i32 {
                        let act_0: *mut xkb_action =
                            (*interp).a.actions.offset(a as isize) as *mut xkb_action;
                        if !update_pending_action_fields(
                            info,
                            XKB_KEYCODE_INVALID as xkb_keycode_t,
                            act_0,
                        ) {
                            return false;
                        }
                        a = a.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            if !ApplyInterpsToKey(keymap, key) {
                return false;
            }
            CheckMultipleActionsCategories(keymap, key);
            key = key.offset(1);
        }
        let mut idx: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = std::ptr::null_mut();
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as xkb_mod_index_t;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                as *mut xkb_mod;
            while idx < (*keymap).mods.num_mods {
                if (*key).vmodmap & (1 as xkb_mod_mask_t) << idx != 0 {
                    (*mod_0).mapping |= (*key).modmap;
                }
                idx = idx.wrapping_add(1);
                mod_0 = mod_0.offset(1);
            }
            key = key.offset(1);
        }
        if (*keymap).format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as u32 {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as xkb_mod_index_t;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                as *mut xkb_mod;
            while idx < (*keymap).mods.num_mods {
                let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << idx;
                if (*mod_0).mapping == 0 as xkb_mod_mask_t
                    && (*keymap).mods.explicit_vmods & mask == 0
                {
                    (*mod_0).mapping = mask;
                    (*keymap).mods.explicit_vmods |= mask;
                }
                idx = idx.wrapping_add(1);
                mod_0 = mod_0.offset(1);
            }
        }
        let mut extra_canonical_mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        idx = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as xkb_mod_index_t;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize) as *mut xkb_mod;
        while idx < (*keymap).mods.num_mods {
            extra_canonical_mods |= (*mod_0).mapping;
            idx = idx.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*keymap).canonical_state_mask |= extra_canonical_mods;
        let mut i_0: u32 = 0 as u32;
        while i_0 < (*keymap).num_types {
            ComputeEffectiveMask(
                keymap,
                &raw mut (*(*keymap).types.offset(i_0 as isize)).mods,
            );
            let mut j: u32 = 0 as u32;
            while j < (*(*keymap).types.offset(i_0 as isize)).num_entries {
                if {
                    // has_unbound_vmods inlined
                    let entry_mods = (*(*(*keymap).types.offset(i_0 as isize))
                        .entries
                        .offset(j as isize))
                    .mods
                    .mods;
                    let mut unbound = false;
                    let mut k: xkb_mod_index_t =
                        _XKB_MOD_INDEX_NUM_ENTRIES as i32 as xkb_mod_index_t;
                    let mut mod_0: *mut xkb_mod = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                        .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize)
                        as *mut xkb_mod;
                    while k < (*keymap).mods.num_mods {
                        if entry_mods & (1 as xkb_mod_mask_t) << k != 0
                            && (*mod_0).mapping == 0 as xkb_mod_mask_t
                        {
                            unbound = true;
                            break;
                        }
                        k = k.wrapping_add(1);
                        mod_0 = mod_0.offset(1);
                    }
                    unbound
                } {
                    (*(*(*keymap).types.offset(i_0 as isize))
                        .entries
                        .offset(j as isize))
                    .mods
                    .mask = 0 as xkb_mod_mask_t;
                } else {
                    ComputeEffectiveMask(
                        keymap,
                        &raw mut (*(*(*keymap).types.offset(i_0 as isize))
                            .entries
                            .offset(j as isize))
                        .mods,
                    );
                    ComputeEffectiveMask(
                        keymap,
                        &raw mut (*(*(*keymap).types.offset(i_0 as isize))
                            .entries
                            .offset(j as isize))
                        .preserve,
                    );
                }
                j = j.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            if !update_pending_key_fields(info, key) {
                return false;
            }
            let mut i_1: xkb_layout_index_t = 0 as xkb_layout_index_t;
            while i_1 < (*key).num_groups() {
                let mut j_0: xkb_level_index_t = 0 as xkb_level_index_t;
                while j_0 < XkbKeyNumLevels(key, i_1) {
                    if (*(*(*key).groups.offset(i_1 as isize))
                        .levels
                        .offset(j_0 as isize))
                    .num_actions as i32
                        <= 1 as i32
                    {
                        let act_1: *mut xkb_action =
                            &raw mut (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .a
                            .action;
                        UpdateActionMods(keymap, act_1, (*key).modmap);
                        if (pending_computations as i32 != 0
                            || (*act_1).type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32)
                            && !update_pending_action_fields(info, (*key).keycode, act_1)
                        {
                            return false;
                        }
                    } else {
                        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
                        while (k as i32)
                            < (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .num_actions as i32
                        {
                            let act_2: *mut xkb_action = (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .a
                            .actions
                            .offset(k as isize)
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
            key = key.offset(1);
        }
        let mut led: *mut xkb_led = std::ptr::null_mut();
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while led < (&raw mut (*keymap).leds as *mut xkb_led).offset((*keymap).num_leds as isize) {
            ComputeEffectiveMask(keymap, &raw mut (*led).mods);
            if pending_computations as i32 != 0 && {
                // update_pending_led_fields inlined
                let mut led_ok = true;
                if (*led).pending_groups() {
                    let pc: *mut pending_computation = &mut (&mut *(*info).pending_computations)
                        [(*led).groups as usize]
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut mask: xkb_layout_mask_t = 0 as xkb_layout_mask_t;
                        if !ExprResolveGroupMask(
                            info,
                            (*pc).expr,
                            &raw mut mask,
                            std::ptr::null_mut(),
                        ) {
                            xkb_logf!(
                                (*info).keymap.ctx,
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
                        (*led).set_pending_groups((false) as bool);
                        (*led).groups = (*pc).value as xkb_layout_mask_t;
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
pub unsafe fn CompileKeymap(mut file: *mut XkbFile, mut keymap: *mut xkb_keymap) -> bool {
    unsafe {
        let mut files: [*mut XkbFile; 4] = [
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ];
        let mut type_0: xkb_file_type = FILE_TYPE_KEYCODES;
        let mut ctx: *mut xkb_context = (*keymap).ctx;
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
            keymap: *keymap,
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
                        name: b"first\0".as_ptr() as *const i8,
                        value: 1 as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME.as_ptr()
                        } else {
                            std::ptr::null()
                        },
                        value: (*keymap).num_groups as u32,
                    },
                    LookupEntry {
                        name: std::ptr::null(),
                        value: 0 as u32,
                    },
                ],
                groupMaskNames: [
                    LookupEntry {
                        name: b"none\0".as_ptr() as *const i8,
                        value: 0 as u32,
                    },
                    LookupEntry {
                        name: b"first\0".as_ptr() as *const i8,
                        value: 0x1 as u32,
                    },
                    LookupEntry {
                        name: b"all\0".as_ptr() as *const i8,
                        value: XKB_ALL_GROUPS as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME.as_ptr()
                        } else {
                            std::ptr::null()
                        },
                        value: if (*keymap).num_groups != 0
                            && (*keymap).num_groups <= XKB_MAX_GROUPS as xkb_layout_index_t
                        {
                            (1 as u32) << (*keymap).num_groups.wrapping_sub(1 as xkb_layout_index_t)
                        } else {
                            0 as u32
                        },
                    },
                    LookupEntry {
                        name: std::ptr::null(),
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
                *keymap = info.keymap;
                pending_computations_array_free(&raw mut pending_computations);
                return false;
            }
            type_0 += 1;
        }
        let ok_0: bool = UpdateDerivedKeymapFields(&raw mut info) as bool;
        *keymap = info.keymap;
        pending_computations_array_free(&raw mut pending_computations);
        return ok_0;
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        default_interpret = {
            let mut init = xkb_sym_interpret {
                repeat_required: [0; 1],
                sym: XKB_KEY_NoSymbol as xkb_keysym_t,
                match_0: MATCH_ANY_OR_NONE,
                mods: 0 as xkb_mod_mask_t,
                virtual_mod: DEFAULT_INTERPRET_VMOD as u32 as xkb_mod_index_t,
                level_one_only: false,
                num_actions: 0 as xkb_action_count_t,
                a: C2Rust_Unnamed_1 {
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                },
            };
            init.set_repeat(DEFAULT_INTERPRET_KEY_REPEAT as i32 != 0);
            init.set_required(false);
            init
        }
    }
}
use crate::xkb::keymap::xkb_keymap_key_get_syms_by_level;
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
