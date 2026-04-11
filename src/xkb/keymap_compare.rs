pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
}
pub mod types_h {
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
}
pub mod stdint_intn_h {
    pub type i8 = __int8_t;
    pub type i16 = __int16_t;
    pub type i32 = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}

pub mod context_h {
    pub use crate::xkb::context_priv::xkb_atom_text;
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
    extern "C" {
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: i32,
            fmt: *const i8,
            ...
        );
    }
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
}
pub mod darray_h {
    pub use crate::xkb::shared_types::darray_size_t;
}
pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::{
        xkb_keycode_t, xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t,
        xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t, xkb_log_level,
        xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
        XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
        XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
        XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS,
        XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED,
        XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED,
        XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
    };
    pub use crate::xkb::shared_types::{
        xkb_keymap_compile_flags, xkb_keymap_format, XKB_KEYMAP_COMPILE_NO_FLAGS,
        XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    };
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = i32;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
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
pub mod utils_h {
    #[inline]
    pub unsafe fn streq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return strcmp(s1, s2) == 0 as i32;
        }
    }
    #[inline]
    pub unsafe fn streq_null(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return s1 == s2;
            }
            return streq(s1, s2);
        }
    }
    use super::string_h::strcmp;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod string_h {
    extern "C" {
        pub fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    }
}
pub mod stdbool_h {}
pub use self::__stddef_null_h::NULL;

pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::keymap_compare_h::{
    xkb_keymap_compare_property, XKB_KEYMAP_CMP_ALL, XKB_KEYMAP_CMP_KEYCODES, XKB_KEYMAP_CMP_LEDS,
    XKB_KEYMAP_CMP_MODS, XKB_KEYMAP_CMP_POSSIBLY_DROPPED, XKB_KEYMAP_CMP_SYMBOLS,
    XKB_KEYMAP_CMP_TYPES,
};
pub use self::keymap_h::{
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
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use self::stdint_intn_h::{i16, i32, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::utils_h::{streq, streq_null};
pub use self::xkbcommon_h::{
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t, xkb_layout_index_t,
    xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t,
    xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use crate::xkb::keymap_priv::action_equal;
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Modifier #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                    mod_0,
                    name1,
                    name2,
                );
                identical = false;
            }
            if (*mod1).type_0 as u32 != (*mod2).type_0 as u32 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Modifier #%u types do not match: %d != %d\n\0".as_ptr() as *const i8,
                    mod_0,
                    (*mod1).type_0 as u32,
                    (*mod2).type_0 as u32,
                );
                identical = false;
            }
            if (*mod1).mapping != (*mod2).mapping {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Modifier #%u mappings do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                    mod_0,
                    (*mod1).mapping,
                    (*mod2).mapping,
                );
                identical = false;
            }
            mod_0 = mod_0.wrapping_add(1);
        }
        if (*keymap1).mods.num_mods != (*keymap2).mods.num_mods {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Modifiers counts do not match: %u != %u\n\0".as_ptr() as *const i8,
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
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Keycodes counts do not match: %u != %u\n\0".as_ptr() as *const i8,
                (*keymap1).num_keys,
                (*keymap2).num_keys,
            );
            identical = false;
        }
        if (*keymap1).min_key_code != (*keymap2).min_key_code {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Min keycodes do not match: %u != %u\n\0".as_ptr() as *const i8,
                (*keymap1).min_key_code,
                (*keymap2).min_key_code,
            );
            identical = false;
        }
        if (*keymap1).max_key_code != (*keymap2).max_key_code {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Low keycodes counts do not match: %u != %u\n\0".as_ptr() as *const i8,
                (*keymap1).num_keys_low,
                (*keymap2).num_keys_low,
            );
            identical = false;
        }
        if (*keymap1).max_key_code != (*keymap2).max_key_code {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Max keycodes do not match: %u != %u\n\0".as_ptr() as *const i8,
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Key #%u keycodes do not match: %x != %x\n\0".as_ptr() as *const i8,
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
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key 0x%x names do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                        kc,
                        name1,
                        name2,
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Alias #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                    a,
                    alias1,
                    alias2,
                );
                identical = false;
            }
            let real1: *const i8 = xkb_atom_text((*keymap1).ctx, (*entry1).real) as *const i8;
            let real2: *const i8 = xkb_atom_text((*keymap2).ctx, (*entry2).real) as *const i8;
            if !streq_null(real1, real2) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Alias #%u \"%s\" target do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                        as *const i8,
                    a,
                    alias1,
                    real1,
                    real2,
                );
                identical = false;
            }
            a = a.wrapping_add(1);
        }
        if (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
            != (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Aliases count do not match: %u != %u\n\0".as_ptr() as *const i8,
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"LED #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                    led,
                    name1,
                    name2,
                );
                identical = false;
            }
            if (*led1).which_groups() as i32 != (*led2).which_groups() as i32 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"LED #%u \"%s\" `which_groups` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const i8,
                    led,
                    name1,
                    (*led1).which_groups() as i32,
                    (*led2).which_groups() as i32,
                );
                identical = false;
            }
            if (*led1).groups != (*led2).groups {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"LED #%u \"%s\" `groups` do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                    led,
                    name1,
                    (*led1).groups,
                    (*led2).groups,
                );
                identical = false;
            }
            if (*led1).which_mods as u32 != (*led2).which_mods as u32 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"LED #%u \"%s\" `which_mods` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const i8,
                    led,
                    name1,
                    (*led1).which_mods as u32,
                    (*led2).which_mods as u32,
                );
                identical = false;
            }
            if (*led1).mods.mods != (*led2).mods.mods {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"LED #%u \"%s\" `mods` do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                    led,
                    name1,
                    (*led1).mods.mods,
                    (*led2).mods.mods,
                );
                identical = false;
            }
            if (*led1).ctrls as u32 != (*led2).ctrls as u32 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"LED #%u \"%s\" `ctrls` do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                    led,
                    name1,
                    (*led1).ctrls as u32,
                    (*led2).ctrls as u32,
                );
                identical = false;
            }
            led = led.wrapping_add(1);
        }
        if (*keymap1).num_leds != (*keymap2).num_leds {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"LEDs count do not match: %u != %u\n\0".as_ptr() as *const i8,
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
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key type names do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                name1,
                name2,
            );
            identical = false;
        }
        if (*type1).mods.mods != (*type2).mods.mods {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key type \"%s\" mods do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                name1,
                (*type1).mods.mods,
                (*type2).mods.mods,
            );
            return false;
        }
        if (*type1).num_levels != (*type2).num_levels {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key type \"%s\" levels count do not match: %u != %u\n\0".as_ptr() as *const i8,
                name1,
                (*type1).num_levels,
                (*type2).num_levels,
            );
            return false;
        }
        if (*type1).num_level_names != (*type2).num_level_names {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key type \"%s\" level names count do not match: %u != %u\n\0".as_ptr()
                    as *const i8,
                name1,
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
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key type \"%s\" level #%u names do not match: \"%s\" != \"%s\"\n\0"
                            .as_ptr() as *const i8,
                        name1,
                        l,
                        lname1,
                        lname2,
                    );
                    identical = false;
                }
                l = l.wrapping_add(1);
            }
        }
        if (*type1).num_entries != (*type2).num_entries {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key type \"%s\" entries count do not match: %u != %u\n\0".as_ptr() as *const i8,
                name1,
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
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key type \"%s\" entry #%u levels do not match: %u != %u\n\0".as_ptr()
                            as *const i8,
                        name1,
                        e,
                        (*entry1).level,
                        (*entry2).level,
                    );
                    identical = false;
                }
                if (*entry1).mods.mods != (*entry2).mods.mods {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key type \"%s\" entry #%u mods do not match: 0x%x != 0x%x\n\0".as_ptr()
                            as *const i8,
                        name1,
                        e,
                        (*entry1).mods.mods,
                        (*entry2).mods.mods,
                    );
                    identical = false;
                }
                if (*entry1).preserve.mods != (*entry2).preserve.mods {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key type \"%s\" entry #%u preserve do not match: 0x%x != 0x%x\n\0"
                            .as_ptr() as *const i8,
                        name1,
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
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key types counts do not match: %u != %u\n\0".as_ptr() as *const i8,
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
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Key 0x%x/group %u types do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                kc,
                g,
                name1,
                name2,
            );
            return false;
        }
        let mut identical: bool = true;
        let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
        while l < (*(*group1).type_0).num_levels {
            let level1: *const xkb_level = (*group1).levels.offset(l as isize) as *mut xkb_level;
            let level2: *const xkb_level = (*group2).levels.offset(l as isize) as *mut xkb_level;
            if (*level1).num_syms as i32 != (*level2).num_syms as i32 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Key 0x%x/group %u/level %u keysyms count do not match: %u != %u\n\0".as_ptr()
                        as *const i8,
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
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            b"Key 0x%x/group %u/level %u keysyms #%u do not match: 0x%x != 0x%x\n\0"
                                .as_ptr() as *const i8,
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Key 0x%x/group %u/level %u keysyms do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const i8,
                    kc,
                    g,
                    l,
                    (*level1).s.sym,
                    (*level2).s.sym,
                );
                identical = false;
            }
            if (*level1).num_actions as i32 != (*level2).num_actions as i32 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Key 0x%x/group %u/level %u actions count do not match: %u != %u\n\0".as_ptr()
                        as *const i8,
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
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            b"Key 0x%x/group %u/level %u actions #%u do not match\n\0".as_ptr()
                                as *const i8,
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Key 0x%x/group %u/level %u actions do not match\n\0".as_ptr() as *const i8,
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
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Group counts do not match: %u != %u\n\0".as_ptr() as *const i8,
                (*keymap1).num_groups,
                (*keymap2).num_groups,
            );
            identical = false;
        }
        if (*keymap1).num_group_names != (*keymap2).num_group_names {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"Group name counts do not match: %u != %u\n\0".as_ptr() as *const i8,
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
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Group #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr() as *const i8,
                        g,
                        name1,
                        name2,
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
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Key #%u keycodes do not match: %x != %x\n\0".as_ptr() as *const i8,
                    k,
                    (*key1).keycode,
                    (*key2).keycode,
                );
                identical = false;
            } else {
                let kc: xkb_keycode_t = (*key1).keycode;
                if (*key1).modmap != (*key2).modmap {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key 0x%x modmap do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                        kc,
                        (*key1).modmap,
                        (*key2).modmap,
                    );
                    identical = false;
                }
                if (*key1).vmodmap != (*key2).vmodmap {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key 0x%x vmodmap do not match: 0x%x != 0x%x\n\0".as_ptr() as *const i8,
                        kc,
                        (*key1).vmodmap,
                        (*key2).vmodmap,
                    );
                    identical = false;
                }
                if (*key1).repeats() as i32 != (*key2).repeats() as i32 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key 0x%x repeats do not match: %d != %d\n\0".as_ptr() as *const i8,
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
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key 0x%x out-of-range do not match: %d != %d or %u != %u\n\0".as_ptr()
                            as *const i8,
                        kc,
                        (*key1).out_of_range_group_policy() as i32,
                        (*key2).out_of_range_group_policy() as i32,
                        (*key1).out_of_range_group_number() as i32,
                        (*key2).out_of_range_group_number() as i32,
                    );
                    identical = false;
                }
                if (*key1).num_groups() as i32 != (*key2).num_groups() as i32 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Key 0x%x groups counts do not match: %u != %u\n\0".as_ptr() as *const i8,
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
