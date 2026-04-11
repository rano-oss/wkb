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
    pub use crate::xkb::context_priv::xkb_atom_intern;
    pub use crate::xkb::shared_types::{
        atom_table, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names, C2Rust_Unnamed,
        C2Rust_Unnamed_0,
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
    use super::stdint_uintn_h::u32;
    pub use crate::xkb::shared_types::{
        xkb_context, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
        xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE,
        XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_INVALID,
        XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
        XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
        XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID,
        XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
        XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
        XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
    extern "C" {
        pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;
    }
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;
}
pub mod enums_h {
    pub type xkb_enumerations_values = u32;
    pub const XKB_COMPOSE_FEED_RESULT_VALUES: xkb_enumerations_values = 3;
    pub const XKB_COMPOSE_STATUS_VALUES: xkb_enumerations_values = 15;
    pub const XKB_COMPOSE_STATE_FLAGS_VALUES: xkb_enumerations_values = 0;
    pub const XKB_COMPOSE_FORMAT_VALUES: xkb_enumerations_values = 2;
    pub const XKB_COMPOSE_COMPILE_FLAGS_VALUES: xkb_enumerations_values = 0;
    pub const XKB_CONSUMED_MODE_VALUES: xkb_enumerations_values = 3;
    pub const XKB_STATE_MATCH_VALUES: xkb_enumerations_values = 65539;
    pub const XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES: xkb_enumerations_values = 7;
    pub const XKB_KEY_DIRECTION_VALUES: xkb_enumerations_values = 7;
    pub const XKB_A11Y_FLAGS_VALUES: xkb_enumerations_values = 3;
    pub const XKB_EVENTS_FLAGS_VALUES: xkb_enumerations_values = 0;
    pub const XKB_KEYBOARD_CONTROL_FLAGS_VALUES: xkb_enumerations_values = 511;
    pub const XKB_STATE_COMPONENT_VALUES: xkb_enumerations_values = 1023;
    pub const XKB_EVENT_TYPE_VALUES: xkb_enumerations_values = 30;
    pub const XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES: xkb_enumerations_values = 3;
    pub const XKB_KEYMAP_SERIALIZE_FLAGS_VALUES: xkb_enumerations_values = 7;
    pub const XKB_KEYMAP_FORMAT_VALUES: xkb_enumerations_values = 6;
    pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: xkb_enumerations_values = 1;
    pub const XKB_CONTEXT_FLAGS_VALUES: xkb_enumerations_values = 7;
    pub const XKB_KEYSYM_FLAGS_VALUES: xkb_enumerations_values = 1;
    pub const XKB_RMLVO_BUILDER_FLAGS_VALUES: xkb_enumerations_values = 0;
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
pub mod string_h {

    extern "C" {
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: usize,
        ) -> i32;
        pub fn strlen(__s: *const i8) -> usize;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    }
}
pub mod stdint_h {
    pub const INT32_MAX: i32 = 2147483647 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub mod xkbcommon_names_h {
    pub const XKB_MOD_NAME_SHIFT: [i8; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"Shift\0") };
    pub const XKB_MOD_NAME_CAPS: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Lock\0") };
    pub const XKB_MOD_NAME_CTRL: [i8; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"Control\0") };
    pub const XKB_MOD_NAME_MOD1: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod1\0") };
    pub const XKB_MOD_NAME_MOD2: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod2\0") };
    pub const XKB_MOD_NAME_MOD3: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod3\0") };
    pub const XKB_MOD_NAME_MOD4: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod4\0") };
    pub const XKB_MOD_NAME_MOD5: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod5\0") };
}
pub use self::__stddef_null_h::NULL;

pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{
    xkb_atom_intern, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
pub use self::enums_h::{
    xkb_enumerations_values, XKB_A11Y_FLAGS_VALUES, XKB_COMPOSE_COMPILE_FLAGS_VALUES,
    XKB_COMPOSE_FEED_RESULT_VALUES, XKB_COMPOSE_FORMAT_VALUES, XKB_COMPOSE_STATE_FLAGS_VALUES,
    XKB_COMPOSE_STATUS_VALUES, XKB_CONSUMED_MODE_VALUES, XKB_CONTEXT_FLAGS_VALUES,
    XKB_EVENTS_FLAGS_VALUES, XKB_EVENT_TYPE_VALUES, XKB_KEYBOARD_CONTROL_FLAGS_VALUES,
    XKB_KEYMAP_COMPILE_FLAGS_VALUES, XKB_KEYMAP_FORMAT_VALUES,
    XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES, XKB_KEYMAP_SERIALIZE_FLAGS_VALUES,
    XKB_KEYSYM_FLAGS_VALUES, XKB_KEY_DIRECTION_VALUES, XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES,
    XKB_RMLVO_BUILDER_FLAGS_VALUES, XKB_STATE_COMPONENT_VALUES, XKB_STATE_MATCH_VALUES,
};
pub use self::internal::__va_list_tag;
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
    C2Rust_Unnamed_9, KeycodeMatch, XkbKeyNumLevels, _ACTION_TYPE_NUM_ENTRIES,
    ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL,
    ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK,
    ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP,
    ACTION_PENDING_COMPUTATION, ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET,
    ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL,
    ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE,
    ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK,
    ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE,
    ACTION_TYPE_UNKNOWN, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS,
    CONTROL_ALL, CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX,
    CONTROL_AX_FEEDBACK, CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT,
    XKB_MAX_GROUPS,
};
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::INT32_MAX;
pub use self::stdint_intn_h::{i16, i32, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::calloc;
use self::string_h::{memcmp, strlen};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::xkbcommon_h::{
    xkb_context_ref, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
    xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
    xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
    xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE,
    XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_INVALID,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_names_h::{
    XKB_MOD_NAME_CAPS, XKB_MOD_NAME_CTRL, XKB_MOD_NAME_MOD1, XKB_MOD_NAME_MOD2, XKB_MOD_NAME_MOD3,
    XKB_MOD_NAME_MOD4, XKB_MOD_NAME_MOD5, XKB_MOD_NAME_SHIFT,
};
unsafe fn update_builtin_keymap_fields(mut keymap: *mut xkb_keymap) {
    unsafe {
        static mut builtin_mods: [*const i8; 8] = [
            XKB_MOD_NAME_SHIFT.as_ptr(),
            XKB_MOD_NAME_CAPS.as_ptr(),
            XKB_MOD_NAME_CTRL.as_ptr(),
            XKB_MOD_NAME_MOD1.as_ptr(),
            XKB_MOD_NAME_MOD2.as_ptr(),
            XKB_MOD_NAME_MOD3.as_ptr(),
            XKB_MOD_NAME_MOD4.as_ptr(),
            XKB_MOD_NAME_MOD5.as_ptr(),
        ];
        let mut i: xkb_mod_index_t = 0 as xkb_mod_index_t;
        while (i as usize)
            < (::core::mem::size_of::<[*const i8; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const i8>() as usize)
        {
            (*keymap).mods.mods[i as usize].name = xkb_atom_intern(
                (*keymap).ctx,
                builtin_mods[i as usize],
                strlen(builtin_mods[i as usize]),
            );
            (*keymap).mods.mods[i as usize].type_0 = MOD_REAL;
            (*keymap).mods.mods[i as usize].mapping = ((1 as u32) << i) as xkb_mod_mask_t;
            i = i.wrapping_add(1);
        }
        (*keymap).mods.num_mods = (::core::mem::size_of::<[*const i8; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const i8>() as usize)
            as xkb_mod_index_t;
        (*keymap).canonical_state_mask = MOD_REAL_MASK_ALL;
    }
}
pub unsafe fn xkb_keymap_new(
    mut ctx: *mut xkb_context,
    mut func: *const i8,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        static mut XKB_KEYMAP_COMPILE_FLAGS: xkb_keymap_compile_flags =
            XKB_KEYMAP_COMPILE_FLAGS_VALUES as i32 as xkb_keymap_compile_flags;
        if flags as u32 & !(XKB_KEYMAP_COMPILE_FLAGS as u32) != 0 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unrecognized keymap compilation flags: 0x%x\n\0".as_ptr() as *const i8,
                func,
                flags as u32 & !(XKB_KEYMAP_COMPILE_FLAGS as u32),
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let keymap: *mut xkb_keymap =
            calloc(1 as usize, ::core::mem::size_of::<xkb_keymap>() as usize) as *mut xkb_keymap;
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        (*keymap).refcnt = 1 as i32;
        (*keymap).ctx = xkb_context_ref(ctx);
        (*keymap).format = format;
        (*keymap).flags = flags;
        update_builtin_keymap_fields(keymap);
        return keymap;
    }
}
pub unsafe fn XkbEscapeMapName(mut name: *mut i8) {
    unsafe {
        static mut legal: [::core::ffi::c_uchar; 32] = [
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0xa7 as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x83 as i32 as ::core::ffi::c_uchar,
            0xfe as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x87 as i32 as ::core::ffi::c_uchar,
            0xfe as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x7f as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x7f as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
        ];
        if name.is_null() {
            return;
        }
        while *name != 0 {
            let mut c: ::core::ffi::c_uchar = *name as ::core::ffi::c_uchar;
            if legal[(c as i32 / 8 as i32) as usize] as u32 & (1 as u32) << c as i32 % 8 as i32 == 0
            {
                *name = '_' as i32 as i8;
            }
            name = name.offset(1);
        }
    }
}
pub unsafe fn XkbModNameToIndex(
    mut mods: *const xkb_mod_set,
    mut name: xkb_atom_t,
    mut type_0: mod_type,
) -> xkb_mod_index_t {
    unsafe {
        let mut i: xkb_mod_index_t = 0;
        let mut mod_0: *const xkb_mod = ::core::ptr::null::<xkb_mod>();
        i = 0 as xkb_mod_index_t;
        mod_0 = &raw const (*mods).mods as *const xkb_mod;
        while i < (*mods).num_mods {
            if (*mod_0).type_0 as u32 & type_0 as u32 != 0 && name == (*mod_0).name {
                return i;
            }
            i = i.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        return XKB_MOD_INVALID as xkb_mod_index_t;
    }
}
pub unsafe fn XkbLevelsSameSyms(
    mut a: *const xkb_level,
    mut b: *const xkb_level,
) -> bool {
    unsafe {
        if (*a).num_syms as i32 != (*b).num_syms as i32 {
            return false_0 != 0;
        }
        if (*a).num_syms as i32 <= 1 as i32 {
            return (*a).s.sym == (*b).s.sym;
        }
        return memcmp(
            (*a).s.syms as *const ::core::ffi::c_void,
            (*b).s.syms as *const ::core::ffi::c_void,
            (::core::mem::size_of::<xkb_keysym_t>() as usize).wrapping_mul((*a).num_syms as usize),
        ) == 0 as i32;
    }
}
pub unsafe fn action_equal(mut a: *const xkb_action, mut b: *const xkb_action) -> bool {
    unsafe {
        if (*a).type_0 as u32 != (*b).type_0 as u32 {
            return false_0 != 0;
        }
        match (*a).type_0 as u32 {
            0 | 1 => return true_0 != 0,
            2 | 3 | 4 => {
                return (*a).mods.flags as u32 == (*b).mods.flags as u32
                    && (*a).mods.mods.mask == (*b).mods.mods.mask
                    && (*a).mods.mods.mods == (*b).mods.mods.mods;
            }
            5 | 6 | 7 => {
                return (*a).group.flags as u32 == (*b).group.flags as u32
                    && (*a).group.group == (*b).group.group;
            }
            8 => {
                return (*a).ptr.flags as u32 == (*b).ptr.flags as u32
                    && (*a).ptr.x as i32 == (*b).ptr.x as i32
                    && (*a).ptr.y as i32 == (*b).ptr.y as i32;
            }
            9 | 10 => {
                return (*a).btn.flags as u32 == (*b).btn.flags as u32
                    && (*a).btn.button as i32 == (*b).btn.button as i32
                    && (*a).btn.count as i32 == (*b).btn.count as i32;
            }
            11 => {
                return (*a).dflt.flags as u32 == (*b).dflt.flags as u32
                    && (*a).dflt.value as i32 == (*b).dflt.value as i32;
            }
            12 => return true_0 != 0,
            13 => {
                return (*a).screen.flags as u32 == (*b).screen.flags as u32
                    && (*a).screen.screen as i32 == (*b).screen.screen as i32;
            }
            14 | 15 => {
                return (*a).ctrls.flags as u32 == (*b).ctrls.flags as u32
                    && (*a).ctrls.ctrls as u32 == (*b).ctrls.ctrls as u32;
            }
            16 => {
                return (*a).redirect.keycode == (*b).redirect.keycode
                    && (*a).redirect.affect == (*b).redirect.affect
                    && (*a).redirect.mods == (*b).redirect.mods;
            }
            17 | 18 => return true_0 != 0,
            20 => {
                return (*a).internal.flags as u32 == (*b).internal.flags as u32
                    && (*a).internal.c2rust_unnamed.clear_latched_mods
                        == (*b).internal.c2rust_unnamed.clear_latched_mods;
            }
            _ => {
                return memcmp(
                    &raw const (*a).priv_0.data as *const uint8_t as *const ::core::ffi::c_void,
                    &raw const (*b).priv_0.data as *const uint8_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[uint8_t; 7]>() as usize,
                ) == 0 as i32;
            }
        };
    }
}
pub unsafe fn XkbLevelsSameActions(
    mut a: *const xkb_level,
    mut b: *const xkb_level,
) -> bool {
    unsafe {
        if (*a).num_actions as i32 != (*b).num_actions as i32 {
            return false_0 != 0;
        }
        if (*a).num_actions as i32 <= 1 as i32 {
            return action_equal(&raw const (*a).a.action, &raw const (*b).a.action);
        }
        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
        while (k as i32) < (*a).num_actions as i32 {
            if !action_equal(
                (*a).a.actions.offset(k as isize) as *mut xkb_action,
                (*b).a.actions.offset(k as isize) as *mut xkb_action,
            ) {
                return false_0 != 0;
            }
            k = k.wrapping_add(1);
        }
        return true_0 != 0;
    }
}
pub unsafe fn XkbWrapGroupIntoRange(
    mut group: i32,
    mut num_groups: xkb_layout_index_t,
    mut out_of_range_group_policy: xkb_layout_out_of_range_policy,
    mut out_of_range_group_number: xkb_layout_index_t,
) -> xkb_layout_index_t {
    unsafe {
        if num_groups == 0 as xkb_layout_index_t {
            return XKB_LAYOUT_INVALID as xkb_layout_index_t;
        }
        if group >= 0 as i32 && (group as xkb_layout_index_t) < num_groups {
            return group as xkb_layout_index_t;
        }
        match out_of_range_group_policy as u32 {
            2 => {
                if out_of_range_group_number >= num_groups {
                    return 0 as xkb_layout_index_t;
                }
                return out_of_range_group_number;
            }
            1 => {
                if group < 0 as i32 {
                    return 0 as xkb_layout_index_t;
                } else {
                    return num_groups.wrapping_sub(1 as xkb_layout_index_t);
                }
            }
            0 | _ => {
                let rem: i32 = group % num_groups as i32;
                return (if rem >= 0 as i32 {
                    rem
                } else {
                    rem + num_groups as i32
                }) as xkb_layout_index_t;
            }
        };
    }
}
pub unsafe fn xkb_keymap_key_get_actions_by_level(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut actions: *mut *const xkb_action,
) -> xkb_action_count_t {
    unsafe {
        let mut count: xkb_action_count_t = 0;
        let mut c2rust_current_block: u64;
        if !key.is_null() {
            layout = XkbWrapGroupIntoRange(
                layout as i32,
                (*key).num_groups(),
                (*key).out_of_range_group_policy(),
                (*key).out_of_range_group_number(),
            );
            if !(layout == XKB_LAYOUT_INVALID as xkb_layout_index_t) {
                if !(level >= XkbKeyNumLevels(key, layout)) {
                    count = (*(*(*key).groups.offset(layout as isize))
                        .levels
                        .offset(level as isize))
                    .num_actions;
                    match count as i32 {
                        0 => {}
                        1 => {
                            c2rust_current_block = 945040705843674513;
                            match c2rust_current_block {
                                15050610111240922756 => {
                                    *actions = (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                }
                                _ => {
                                    *actions = &raw mut (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .action;
                                }
                            }
                            return count;
                        }
                        _ => {
                            c2rust_current_block = 15050610111240922756;
                            match c2rust_current_block {
                                15050610111240922756 => {
                                    *actions = (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                }
                                _ => {
                                    *actions = &raw mut (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .action;
                                }
                            }
                            return count;
                        }
                    }
                }
            }
        }
        *actions = ::core::ptr::null::<xkb_action>();
        return 0 as xkb_action_count_t;
    }
}
