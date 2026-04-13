use crate::xkb_logf;
use crate::xkb::context_priv::xkb_atom_text;

pub mod keymap_compare_h {
    pub type xkb_keymap_compare_property = u32;
    pub const XKB_KEYMAP_CMP_POSSIBLY_DROPPED: xkb_keymap_compare_property = 4;
    pub const XKB_KEYMAP_CMP_ALL: xkb_keymap_compare_property = 31;
    pub const XKB_KEYMAP_CMP_SYMBOLS: xkb_keymap_compare_property = 16;
    pub const XKB_KEYMAP_CMP_KEYCODES: xkb_keymap_compare_property = 8;
    pub const XKB_KEYMAP_CMP_TYPES: xkb_keymap_compare_property = 4;
    pub const XKB_KEYMAP_CMP_LEDS: xkb_keymap_compare_property = 2;
    pub const XKB_KEYMAP_CMP_MODS: xkb_keymap_compare_property = 1;
}

pub use self::keymap_compare_h::{
    xkb_keymap_compare_property, XKB_KEYMAP_CMP_ALL, XKB_KEYMAP_CMP_KEYCODES, XKB_KEYMAP_CMP_LEDS,
    XKB_KEYMAP_CMP_MODS, XKB_KEYMAP_CMP_POSSIBLY_DROPPED, XKB_KEYMAP_CMP_SYMBOLS,
    XKB_KEYMAP_CMP_TYPES,
};
pub use crate::xkb::shared_types::{
    mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t, xkb_pointer_action,
    xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
    ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS,
    ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK,
    ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION,
    ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH,
    ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH,
    ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE,
    ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE,
    ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT,
};
pub use crate::xkb::messages::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use crate::xkb::utils::{streq, streq_null};
pub use crate::xkb::keymap_priv::action_equal;
pub use crate::xkb::shared_types::darray_size_t;
unsafe fn keymap_compare_mods(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let mod_max: xkb_mod_index_t = if (*keymap1).mods.num_mods < (*keymap2).mods.num_mods {
            (*keymap1).mods.num_mods
        } else {
            (*keymap2).mods.num_mods
        };
        let mut mod_0: xkb_mod_index_t = 0 as xkb_mod_index_t;
        while mod_0 < mod_max {
            let mod1: *const xkb_mod = (&raw const (*keymap1).mods.mods as *const xkb_mod)
                .offset(mod_0 as isize) as *const xkb_mod;
            let mod2: *const xkb_mod = (&raw const (*keymap2).mods.mods as *const xkb_mod)
                .offset(mod_0 as isize) as *const xkb_mod;
            let name1: *const i8 = xkb_atom_text((*keymap1).ctx, (*mod1).name) as *const i8;
            let name2: *const i8 = xkb_atom_text((*keymap2).ctx, (*mod2).name) as *const i8;
            if !streq_null(name1, name2) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Modifier #{} names do not match: \"{}\" != \"{}\"\n",
                    mod_0,
                    crate::xkb::utils::CStrDisplay(name1),
                    crate::xkb::utils::CStrDisplay(name2),
                );
                identical = false;
            }
            if (*mod1).type_0 as u32 != (*mod2).type_0 as u32 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Modifier #{} types do not match: {} != {}\n",
                    mod_0,
                    (*mod1).type_0 as u32,
                    (*mod2).type_0 as u32,
                );
                identical = false;
            }
            if (*mod1).mapping != (*mod2).mapping {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Modifier #{} mappings do not match: 0x{:x} != 0x{:x}\n",
                    mod_0,
                    (*mod1).mapping,
                    (*mod2).mapping,
                );
                identical = false;
            }
            mod_0 = mod_0.wrapping_add(1);
        }
        if (*keymap1).mods.num_mods != (*keymap2).mods.num_mods {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Modifiers counts do not match: {} != {}\n",
                (*keymap1).mods.num_mods,
                (*keymap2).mods.num_mods,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe fn keymap_compare_keycodes(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        if (*keymap1).num_keys != (*keymap2).num_keys {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Keycodes counts do not match: {} != {}\n",
                (*keymap1).num_keys,
                (*keymap2).num_keys,
            );
            identical = false;
        }
        if (*keymap1).min_key_code != (*keymap2).min_key_code {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Min keycodes do not match: {} != {}\n",
                (*keymap1).min_key_code,
                (*keymap2).min_key_code,
            );
            identical = false;
        }
        if (*keymap1).max_key_code != (*keymap2).max_key_code {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Low keycodes counts do not match: {} != {}\n",
                (*keymap1).num_keys_low,
                (*keymap2).num_keys_low,
            );
            identical = false;
        }
        if (*keymap1).max_key_code != (*keymap2).max_key_code {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Max keycodes do not match: {} != {}\n",
                (*keymap1).min_key_code,
                (*keymap2).min_key_code,
            );
            identical = false;
        }
        let k_max: xkb_keycode_t = if (*keymap1).num_keys < (*keymap2).num_keys {
            (*keymap1).num_keys
        } else {
            (*keymap2).num_keys
        };
        let mut k: xkb_keycode_t = 0 as xkb_keycode_t;
        while k < k_max {
            let key1: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            let key2: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            if (*key1).keycode != (*key2).keycode {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key #{} keycodes do not match: {:x} != {:x}\n",
                    k,
                    (*key1).keycode,
                    (*key2).keycode,
                );
                identical = false;
            } else {
                let kc: xkb_keycode_t = (*key1).keycode;
                let name1: *const i8 = xkb_atom_text((*keymap1).ctx, (*key1).name) as *const i8;
                let name2: *const i8 = xkb_atom_text((*keymap2).ctx, (*key2).name) as *const i8;
                if !streq_null(name1, name2) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key 0x{:x} names do not match: \"{}\" != \"{}\"\n",
                        kc,
                        crate::xkb::utils::CStrDisplay(name1),
                        crate::xkb::utils::CStrDisplay(name2),
                    );
                    identical = false;
                }
            }
            k = k.wrapping_add(1);
        }
        let a_max: darray_size_t = if (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
            < (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        {
            (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        } else {
            (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        };
        let mut a: darray_size_t = 0 as darray_size_t;
        while a < a_max {
            let entry1: *const xkb_key_alias = (*keymap1)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .key_aliases
                .offset(a as isize)
                as *mut xkb_key_alias;
            let entry2: *const xkb_key_alias = (*keymap2)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .key_aliases
                .offset(a as isize)
                as *mut xkb_key_alias;
            let alias1: *const i8 = xkb_atom_text((*keymap1).ctx, (*entry1).alias) as *const i8;
            let alias2: *const i8 = xkb_atom_text((*keymap2).ctx, (*entry2).alias) as *const i8;
            if !streq_null(alias1, alias2) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Alias #{} names do not match: \"{}\" != \"{}\"\n",
                    a,
                    crate::xkb::utils::CStrDisplay(alias1),
                    crate::xkb::utils::CStrDisplay(alias2),
                );
                identical = false;
            }
            let real1: *const i8 = xkb_atom_text((*keymap1).ctx, (*entry1).real) as *const i8;
            let real2: *const i8 = xkb_atom_text((*keymap2).ctx, (*entry2).real) as *const i8;
            if !streq_null(real1, real2) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Alias #{} \"{}\" target do not match: \"{}\" != \"{}\"\n",
                    a,
                    crate::xkb::utils::CStrDisplay(alias1),
                    crate::xkb::utils::CStrDisplay(real1),
                    crate::xkb::utils::CStrDisplay(real2),
                );
                identical = false;
            }
            a = a.wrapping_add(1);
        }
        if (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
            != (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Aliases count do not match: {} != {}\n",
                (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases,
                (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe fn keymap_compare_leds(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let mut led_max: xkb_led_index_t = if (*keymap1).num_leds < (*keymap2).num_leds {
            (*keymap1).num_leds
        } else {
            (*keymap2).num_leds
        };
        let mut led: xkb_led_index_t = 0 as xkb_led_index_t;
        while led < led_max {
            let led1: *const xkb_led = (&raw const (*keymap1).leds as *const xkb_led)
                .offset(led as isize) as *const xkb_led;
            let led2: *const xkb_led = (&raw const (*keymap2).leds as *const xkb_led)
                .offset(led as isize) as *const xkb_led;
            let name1: *const i8 = xkb_atom_text((*keymap1).ctx, (*led1).name) as *const i8;
            let name2: *const i8 = xkb_atom_text((*keymap2).ctx, (*led2).name) as *const i8;
            if !streq_null(name1, name2) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "LED #{} names do not match: \"{}\" != \"{}\"\n",
                    led,
                    crate::xkb::utils::CStrDisplay(name1),
                    crate::xkb::utils::CStrDisplay(name2),
                );
                identical = false;
            }
            if (*led1).which_groups() as i32 != (*led2).which_groups() as i32 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "LED #{} \"{}\" `which_groups` do not match: 0x{:x} != 0x{:x}\n",
                    led,
                    crate::xkb::utils::CStrDisplay(name1),
                    (*led1).which_groups() as i32,
                    (*led2).which_groups() as i32,
                );
                identical = false;
            }
            if (*led1).groups != (*led2).groups {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "LED #{} \"{}\" `groups` do not match: 0x{:x} != 0x{:x}\n",
                    led,
                    crate::xkb::utils::CStrDisplay(name1),
                    (*led1).groups,
                    (*led2).groups,
                );
                identical = false;
            }
            if (*led1).which_mods as u32 != (*led2).which_mods as u32 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "LED #{} \"{}\" `which_mods` do not match: 0x{:x} != 0x{:x}\n",
                    led,
                    crate::xkb::utils::CStrDisplay(name1),
                    (*led1).which_mods as u32,
                    (*led2).which_mods as u32,
                );
                identical = false;
            }
            if (*led1).mods.mods != (*led2).mods.mods {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "LED #{} \"{}\" `mods` do not match: 0x{:x} != 0x{:x}\n",
                    led,
                    crate::xkb::utils::CStrDisplay(name1),
                    (*led1).mods.mods,
                    (*led2).mods.mods,
                );
                identical = false;
            }
            if (*led1).ctrls as u32 != (*led2).ctrls as u32 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "LED #{} \"{}\" `ctrls` do not match: 0x{:x} != 0x{:x}\n",
                    led,
                    crate::xkb::utils::CStrDisplay(name1),
                    (*led1).ctrls as u32,
                    (*led2).ctrls as u32,
                );
                identical = false;
            }
            led = led.wrapping_add(1);
        }
        if (*keymap1).num_leds != (*keymap2).num_leds {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "LEDs count do not match: {} != {}\n",
                (*keymap1).num_leds,
                (*keymap2).num_leds,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe fn compare_types(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
    mut type1: *const xkb_key_type,
    mut type2: *const xkb_key_type,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let name1: *const i8 = xkb_atom_text((*keymap1).ctx, (*type1).name) as *const i8;
        let name2: *const i8 = xkb_atom_text((*keymap2).ctx, (*type2).name) as *const i8;
        if !streq_null(name1, name2) {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key type names do not match: \"{}\" != \"{}\"\n",
                crate::xkb::utils::CStrDisplay(name1),
                crate::xkb::utils::CStrDisplay(name2),
            );
            identical = false;
        }
        if (*type1).mods.mods != (*type2).mods.mods {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key type \"{}\" mods do not match: 0x{:x} != 0x{:x}\n",
                crate::xkb::utils::CStrDisplay(name1),
                (*type1).mods.mods,
                (*type2).mods.mods,
            );
            return false;
        }
        if (*type1).num_levels != (*type2).num_levels {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key type \"{}\" levels count do not match: {} != {}\n",
                crate::xkb::utils::CStrDisplay(name1),
                (*type1).num_levels,
                (*type2).num_levels,
            );
            return false;
        }
        if (*type1).num_level_names != (*type2).num_level_names {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key type \"{}\" level names count do not match: {} != {}\n",
                crate::xkb::utils::CStrDisplay(name1),
                (*type1).num_level_names,
                (*type2).num_level_names,
            );
            identical = false;
        } else {
            let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
            while l < (*type1).num_level_names {
                let lname1: *const i8 =
                    xkb_atom_text((*keymap1).ctx, *(*type1).level_names.offset(l as isize))
                        as *const i8;
                let lname2: *const i8 =
                    xkb_atom_text((*keymap2).ctx, *(*type2).level_names.offset(l as isize))
                        as *const i8;
                if !streq_null(lname1, lname2) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key type \"{}\" level #{} names do not match: \"{}\" != \"{}\"\n",
                        crate::xkb::utils::CStrDisplay(name1),
                        l,
                        crate::xkb::utils::CStrDisplay(lname1),
                        crate::xkb::utils::CStrDisplay(lname2),
                    );
                    identical = false;
                }
                l = l.wrapping_add(1);
            }
        }
        if (*type1).num_entries != (*type2).num_entries {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key type \"{}\" entries count do not match: {} != {}\n",
                crate::xkb::utils::CStrDisplay(name1),
                (*type1).num_entries,
                (*type2).num_entries,
            );
            identical = false;
        } else {
            let mut e: darray_size_t = 0 as darray_size_t;
            while e < (*type1).num_entries {
                let entry1: *const xkb_key_type_entry =
                    (*type1).entries.offset(e as isize) as *mut xkb_key_type_entry;
                let entry2: *const xkb_key_type_entry =
                    (*type2).entries.offset(e as isize) as *mut xkb_key_type_entry;
                if (*entry1).level != (*entry2).level {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key type \"{}\" entry #{} levels do not match: {} != {}\n",
                        crate::xkb::utils::CStrDisplay(name1),
                        e,
                        (*entry1).level,
                        (*entry2).level,
                    );
                    identical = false;
                }
                if (*entry1).mods.mods != (*entry2).mods.mods {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key type \"{}\" entry #{} mods do not match: 0x{:x} != 0x{:x}\n",
                        crate::xkb::utils::CStrDisplay(name1),
                        e,
                        (*entry1).mods.mods,
                        (*entry2).mods.mods,
                    );
                    identical = false;
                }
                if (*entry1).preserve.mods != (*entry2).preserve.mods {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key type \"{}\" entry #{} preserve do not match: 0x{:x} != 0x{:x}\n",
                        crate::xkb::utils::CStrDisplay(name1),
                        e,
                        (*entry1).preserve.mods,
                        (*entry2).preserve.mods,
                    );
                    identical = false;
                }
                e = e.wrapping_add(1);
            }
        }
        return identical;
    }
}
unsafe fn keymap_compare_types(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let t_max: darray_size_t = if (*keymap1).num_types < (*keymap2).num_types {
            (*keymap1).num_types
        } else {
            (*keymap2).num_types
        };
        let mut t: darray_size_t = 0 as darray_size_t;
        while t < t_max {
            identical = compare_types(
                ctx,
                keymap1,
                keymap2,
                (*keymap1).types.offset(t as isize) as *mut xkb_key_type,
                (*keymap2).types.offset(t as isize) as *mut xkb_key_type,
            ) as i32
                != 0
                && identical as i32 != 0;
            t = t.wrapping_add(1);
        }
        if (*keymap1).num_types != (*keymap2).num_types {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key types counts do not match: {} != {}\n",
                (*keymap1).num_types,
                (*keymap2).num_types,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe fn compare_groups(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
    mut kc: xkb_keycode_t,
    mut g: xkb_layout_index_t,
    mut group1: *const xkb_group,
    mut group2: *const xkb_group,
) -> bool {
    unsafe {
        if !compare_types(ctx, keymap1, keymap2, (*group1).type_0, (*group2).type_0) {
            let name1: *const i8 =
                xkb_atom_text((*keymap1).ctx, (*(*group1).type_0).name) as *const i8;
            let name2: *const i8 =
                xkb_atom_text((*keymap2).ctx, (*(*group2).type_0).name) as *const i8;
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Key 0x{:x}/group {} types do not match: \"{}\" != \"{}\"\n",
                kc,
                g,
                crate::xkb::utils::CStrDisplay(name1),
                crate::xkb::utils::CStrDisplay(name2),
            );
            return false;
        }
        let mut identical: bool = true;
        let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
        while l < (*(*group1).type_0).num_levels {
            let level1: *const xkb_level = (*group1).levels.offset(l as isize) as *mut xkb_level;
            let level2: *const xkb_level = (*group2).levels.offset(l as isize) as *mut xkb_level;
            if (*level1).num_syms as i32 != (*level2).num_syms as i32 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key 0x{:x}/group {}/level {} keysyms count do not match: {} != {}\n",
                    kc,
                    g,
                    l,
                    (*level1).num_syms as i32,
                    (*level2).num_syms as i32,
                );
                identical = false;
            } else if (*level1).num_syms as i32 > 1 as i32 {
                let mut k: xkb_keysym_count_t = 0 as xkb_keysym_count_t;
                while (k as i32) < (*level1).num_syms as i32 {
                    if !(*(*level1).s.syms.offset(k as isize)
                        == *(*level2).s.syms.offset(k as isize))
                    {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "Key 0x{:x}/group {}/level {} keysyms #{} do not match: 0x{:x} != 0x{:x}\n",
                            kc,
                            g,
                            l,
                            k as i32,
                            *(*level1).s.syms.offset(k as isize),
                            *(*level2).s.syms.offset(k as isize),
                        );
                        identical = false;
                    }
                    k = k.wrapping_add(1);
                }
            } else if (*level1).s.sym != (*level2).s.sym {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key 0x{:x}/group {}/level {} keysyms do not match: 0x{:x} != 0x{:x}\n",
                    kc,
                    g,
                    l,
                    (*level1).s.sym,
                    (*level2).s.sym,
                );
                identical = false;
            }
            if (*level1).num_actions as i32 != (*level2).num_actions as i32 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key 0x{:x}/group {}/level {} actions count do not match: {} != {}\n",
                    kc,
                    g,
                    l,
                    (*level1).num_actions as i32,
                    (*level2).num_actions as i32,
                );
                identical = false;
            } else if (*level1).num_actions as i32 > 1 as i32 {
                let mut a: xkb_action_count_t = 0 as xkb_action_count_t;
                while (a as i32) < (*level1).num_actions as i32 {
                    if !action_equal(
                        (*level1).a.actions.offset(a as isize) as *mut xkb_action,
                        (*level2).a.actions.offset(a as isize) as *mut xkb_action,
                    ) {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "Key 0x{:x}/group {}/level {} actions #{} do not match\n",
                            kc,
                            g,
                            l,
                            a as i32,
                        );
                        identical = false;
                    }
                    a = a.wrapping_add(1);
                }
            } else if (*level1).num_actions as i32 == 1 as i32
                && !action_equal(&raw const (*level1).a.action, &raw const (*level2).a.action)
            {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key 0x{:x}/group {}/level {} actions do not match\n",
                    kc,
                    g,
                    l,
                );
                identical = false;
            }
            l = l.wrapping_add(1);
        }
        return identical;
    }
}
unsafe fn keymap_compare_symbols(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        if (*keymap1).num_groups != (*keymap2).num_groups {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Group counts do not match: {} != {}\n",
                (*keymap1).num_groups,
                (*keymap2).num_groups,
            );
            identical = false;
        }
        if (*keymap1).num_group_names != (*keymap2).num_group_names {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Group name counts do not match: {} != {}\n",
                (*keymap1).num_group_names,
                (*keymap2).num_group_names,
            );
            identical = false;
        } else {
            let mut g: xkb_layout_index_t = 0 as xkb_layout_index_t;
            while g < (*keymap1).num_group_names {
                let name1: *const i8 =
                    xkb_atom_text((*keymap1).ctx, *(*keymap1).group_names.offset(g as isize))
                        as *const i8;
                let name2: *const i8 =
                    xkb_atom_text((*keymap2).ctx, *(*keymap2).group_names.offset(g as isize))
                        as *const i8;
                if !streq_null(name1, name2) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Group #{} names do not match: \"{}\" != \"{}\"\n",
                        g,
                        crate::xkb::utils::CStrDisplay(name1),
                        crate::xkb::utils::CStrDisplay(name2),
                    );
                    identical = false;
                }
                g = g.wrapping_add(1);
            }
        }
        let k_max: xkb_keycode_t = if (*keymap1).num_keys < (*keymap2).num_keys {
            (*keymap1).num_keys
        } else {
            (*keymap2).num_keys
        };
        let mut k: xkb_keycode_t = 0 as xkb_keycode_t;
        while k < k_max {
            let key1: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            let key2: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            if (*key1).keycode != (*key2).keycode {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key #{} keycodes do not match: {:x} != {:x}\n",
                    k,
                    (*key1).keycode,
                    (*key2).keycode,
                );
                identical = false;
            } else {
                let kc: xkb_keycode_t = (*key1).keycode;
                if (*key1).modmap != (*key2).modmap {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key 0x{:x} modmap do not match: 0x{:x} != 0x{:x}\n",
                        kc,
                        (*key1).modmap,
                        (*key2).modmap,
                    );
                    identical = false;
                }
                if (*key1).vmodmap != (*key2).vmodmap {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key 0x{:x} vmodmap do not match: 0x{:x} != 0x{:x}\n",
                        kc,
                        (*key1).vmodmap,
                        (*key2).vmodmap,
                    );
                    identical = false;
                }
                if (*key1).repeats() as i32 != (*key2).repeats() as i32 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key 0x{:x} repeats do not match: {} != {}\n",
                        kc,
                        (*key1).repeats() as i32,
                        (*key2).repeats() as i32,
                    );
                    identical = false;
                }
                if (*key1).out_of_range_group_policy() as i32
                    != (*key2).out_of_range_group_policy() as i32
                    || (*key1).out_of_range_group_number() as i32
                        != (*key2).out_of_range_group_number() as i32
                {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key 0x{:x} out-of-range do not match: {} != {} or {} != {}\n",
                        kc,
                        (*key1).out_of_range_group_policy() as i32,
                        (*key2).out_of_range_group_policy() as i32,
                        (*key1).out_of_range_group_number() as i32,
                        (*key2).out_of_range_group_number() as i32,
                    );
                    identical = false;
                }
                if (*key1).num_groups() as i32 != (*key2).num_groups() as i32 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Key 0x{:x} groups counts do not match: {} != {}\n",
                        kc,
                        (*key1).num_groups() as i32,
                        (*key2).num_groups() as i32,
                    );
                    identical = false;
                }
                let g_max: xkb_layout_index_t =
                    (if ((*key1).num_groups() as i32) < (*key2).num_groups() as i32 {
                        (*key1).num_groups() as i32
                    } else {
                        (*key2).num_groups() as i32
                    }) as xkb_layout_index_t;
                let mut g_0: xkb_layout_index_t = 0 as xkb_layout_index_t;
                while g_0 < g_max {
                    identical = compare_groups(
                        ctx,
                        keymap1,
                        keymap2,
                        kc,
                        g_0,
                        (*key1).groups.offset(g_0 as isize) as *mut xkb_group,
                        (*key2).groups.offset(g_0 as isize) as *mut xkb_group,
                    ) as i32
                        != 0
                        && identical as i32 != 0;
                    g_0 = g_0.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        return identical;
    }
}

pub unsafe fn xkb_keymap_compare(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
    mut properties: xkb_keymap_compare_property,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        if properties as u32 & XKB_KEYMAP_CMP_MODS as i32 as u32 != 0 {
            identical =
                keymap_compare_mods(ctx, keymap1, keymap2) as i32 != 0 && identical as i32 != 0;
        }
        if properties as u32 & XKB_KEYMAP_CMP_TYPES as i32 as u32 != 0 {
            identical =
                keymap_compare_types(ctx, keymap1, keymap2) as i32 != 0 && identical as i32 != 0;
        }
        if properties as u32 & XKB_KEYMAP_CMP_LEDS as i32 as u32 != 0 {
            identical =
                keymap_compare_leds(ctx, keymap1, keymap2) as i32 != 0 && identical as i32 != 0;
        }
        if properties as u32 & XKB_KEYMAP_CMP_KEYCODES as i32 as u32 != 0 {
            identical =
                keymap_compare_keycodes(ctx, keymap1, keymap2) as i32 != 0 && identical as i32 != 0;
        }
        if properties as u32 & XKB_KEYMAP_CMP_SYMBOLS as i32 as u32 != 0 {
            identical =
                keymap_compare_symbols(ctx, keymap1, keymap2) as i32 != 0 && identical as i32 != 0;
        }
        return identical;
    }
}
use crate::xkb::shared_types::*;
