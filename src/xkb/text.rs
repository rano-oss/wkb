pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}
pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}

pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_context {
        pub refcnt: ::core::ffi::c_int,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: ::core::ffi::c_int,
        pub user_data: *mut ::core::ffi::c_void,
        pub names_dflt: xkb_rule_names,
        pub includes: C2Rust_Unnamed_0,
        pub failed_includes: C2Rust_Unnamed,
        pub atom_table: *mut atom_table,
        pub x11_atom_cache: *mut ::core::ffi::c_void,
        pub text_buffer: [::core::ffi::c_char; 2048],
        pub text_next: usize,
        #[bitfield(name = "use_environment_names", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "use_secure_getenv", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "pending_default_includes", ty = "bool", bits = "2..=2")]
        pub use_environment_names_use_secure_getenv_pending_default_includes: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }

    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t)
            -> *const ::core::ffi::c_char;
        pub fn xkb_context_get_buffer(
            ctx: *mut xkb_context,
            size: usize,
        ) -> *mut ::core::ffi::c_char;
    }
}
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    use super::darray_h::darray_size_t;
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
}
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const ::core::ffi::c_char,
        pub model: *const ::core::ffi::c_char,
        pub layout: *const ::core::ffi::c_char,
        pub variant: *const ::core::ffi::c_char,
        pub options: *const ::core::ffi::c_char,
    }
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_mod_mask_t = u32;
    pub type xkb_mod_index_t = u32;
    pub type xkb_keysym_t = u32;
    pub type xkb_state_component = ::core::ffi::c_uint;
    pub const XKB_STATE_CONTROLS: xkb_state_component = 512;
    pub const XKB_STATE_LEDS: xkb_state_component = 256;
    pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
    pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
    pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
    pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
    pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
    pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
    pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
    pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;

    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_keysym_get_name(
            keysym: xkb_keysym_t,
            buffer: *mut ::core::ffi::c_char,
            size: usize,
        ) -> ::core::ffi::c_int;
    }
}
pub mod keymap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod_set {
        pub mods: [xkb_mod; 32],
        pub num_mods: xkb_mod_index_t,
        pub explicit_vmods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod {
        pub name: xkb_atom_t,
        pub type_0: mod_type,
        pub mapping: xkb_mod_mask_t,
    }
    pub type mod_type = ::core::ffi::c_uint;
    pub const MOD_BOTH: mod_type = 3;
    pub const MOD_VIRT: mod_type = 2;
    pub const MOD_REAL: mod_type = 1;
    pub type xkb_action_type = ::core::ffi::c_uint;
    pub const _ACTION_TYPE_NUM_ENTRIES: xkb_action_type = 21;
    pub const ACTION_TYPE_INTERNAL: xkb_action_type = 20;
    pub const ACTION_TYPE_PRIVATE: xkb_action_type = 19;
    pub const ACTION_TYPE_UNKNOWN: xkb_action_type = 18;
    pub const ACTION_TYPE_UNSUPPORTED_LEGACY: xkb_action_type = 17;
    pub const ACTION_TYPE_REDIRECT_KEY: xkb_action_type = 16;
    pub const ACTION_TYPE_CTRL_LOCK: xkb_action_type = 15;
    pub const ACTION_TYPE_CTRL_SET: xkb_action_type = 14;
    pub const ACTION_TYPE_SWITCH_VT: xkb_action_type = 13;
    pub const ACTION_TYPE_TERMINATE: xkb_action_type = 12;
    pub const ACTION_TYPE_PTR_DEFAULT: xkb_action_type = 11;
    pub const ACTION_TYPE_PTR_LOCK: xkb_action_type = 10;
    pub const ACTION_TYPE_PTR_BUTTON: xkb_action_type = 9;
    pub const ACTION_TYPE_PTR_MOVE: xkb_action_type = 8;
    pub const ACTION_TYPE_GROUP_LOCK: xkb_action_type = 7;
    pub const ACTION_TYPE_GROUP_LATCH: xkb_action_type = 6;
    pub const ACTION_TYPE_GROUP_SET: xkb_action_type = 5;
    pub const ACTION_TYPE_MOD_LOCK: xkb_action_type = 4;
    pub const ACTION_TYPE_MOD_LATCH: xkb_action_type = 3;
    pub const ACTION_TYPE_MOD_SET: xkb_action_type = 2;
    pub const ACTION_TYPE_VOID: xkb_action_type = 1;
    pub const ACTION_TYPE_NONE: xkb_action_type = 0;
    pub type xkb_action_controls = ::core::ffi::c_uint;
    pub const CONTROL_ALL_BOOLEAN: xkb_action_controls = 2088447;
    pub const CONTROL_ALL_BOOLEAN_V1: xkb_action_controls = 2087943;
    pub const CONTROL_ALL: xkb_action_controls = 2088959;
    pub const CONTROL_ALL_V1: xkb_action_controls = 2088455;
    pub const CONTROL_IGNORE_GROUP_LOCK: xkb_action_controls = 1048576;
    pub const CONTROL_BELL: xkb_action_controls = 524288;
    pub const CONTROL_AX_FEEDBACK: xkb_action_controls = 262144;
    pub const CONTROL_AX_TIMEOUT: xkb_action_controls = 131072;
    pub const CONTROL_AX: xkb_action_controls = 65536;
    pub const CONTROL_MOUSE_KEYS_ACCEL: xkb_action_controls = 32768;
    pub const CONTROL_MOUSE_KEYS: xkb_action_controls = 16384;
    pub const CONTROL_DEBOUNCE: xkb_action_controls = 4096;
    pub const CONTROL_SLOW: xkb_action_controls = 2048;
    pub const CONTROL_REPEAT: xkb_action_controls = 1024;
    pub const CONTROL_GROUPS_WRAP: xkb_action_controls = 512;
    pub const CONTROL_OVERLAY8: xkb_action_controls = 256;
    pub const CONTROL_OVERLAY7: xkb_action_controls = 128;
    pub const CONTROL_OVERLAY6: xkb_action_controls = 64;
    pub const CONTROL_OVERLAY5: xkb_action_controls = 32;
    pub const CONTROL_OVERLAY4: xkb_action_controls = 16;
    pub const CONTROL_OVERLAY3: xkb_action_controls = 8;
    pub const CONTROL_OVERLAY2: xkb_action_controls = 4;
    pub const CONTROL_OVERLAY1: xkb_action_controls = 2;
    pub const CONTROL_STICKY_KEYS: xkb_action_controls = 1;
    pub type xkb_match_operation = ::core::ffi::c_uint;
    pub const MATCH_EXACTLY: xkb_match_operation = 4;
    pub const MATCH_ALL: xkb_match_operation = 3;
    pub const MATCH_ANY: xkb_match_operation = 2;
    pub const MATCH_ANY_OR_NONE: xkb_match_operation = 1;
    pub const MATCH_NONE: xkb_match_operation = 0;
    pub const XKB_MAX_GROUPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    pub const XKB_ALL_GROUPS: ::core::ffi::c_ulong =
        ((1 as ::core::ffi::c_ulong) << XKB_MAX_GROUPS).wrapping_sub(1 as ::core::ffi::c_ulong);
    pub const XKB_MOD_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    pub const MOD_REAL_MASK_ALL: xkb_mod_mask_t = 0xff as ::core::ffi::c_int as xkb_mod_mask_t;
    #[inline]
    pub unsafe extern "C" fn format_boolean_controls(
        mut format: xkb_keymap_format,
    ) -> xkb_action_controls {
        unsafe {
            return (if format as ::core::ffi::c_uint
                == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                CONTROL_ALL_BOOLEAN_V1 as ::core::ffi::c_int
            } else {
                CONTROL_ALL_BOOLEAN as ::core::ffi::c_int
            }) as xkb_action_controls;
        }
    }
    use super::atom_h::xkb_atom_t;
    use super::xkbcommon_h::{
        xkb_keymap_format, xkb_mod_index_t, xkb_mod_mask_t, XKB_KEYMAP_FORMAT_TEXT_V1,
    };
}
pub mod text_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LookupEntry {
        pub name: *const ::core::ffi::c_char,
        pub value: u32,
    }
    pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
    pub const CONTROL_NAMES_MIN_V2_INDEX: C2Rust_Unnamed_1 = 0;
    pub const CONTROL_NAMES_MIN_V1_INDEX: C2Rust_Unnamed_1 = 7;
    #[inline]
    pub unsafe extern "C" fn format_control_names_offset(mut format: xkb_keymap_format) -> uint8_t {
        unsafe {
            return (if format as ::core::ffi::c_uint
                == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                CONTROL_NAMES_MIN_V1_INDEX as ::core::ffi::c_int
            } else {
                CONTROL_NAMES_MIN_V2_INDEX as ::core::ffi::c_int
            }) as uint8_t;
        }
    }
    use super::stdint_uintn_h::{u32, uint8_t};
    use super::xkbcommon_h::{xkb_keymap_format, XKB_KEYMAP_FORMAT_TEXT_V1};
}
pub mod stdio_h {

    extern "C" {
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> usize;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn istreq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn strlen_safe(mut s: *const ::core::ffi::c_char) -> usize {
        unsafe {
            return if !s.is_null() { strlen(s) } else { 0 as usize };
        }
    }
    #[inline]
    pub unsafe extern "C" fn strempty(
        mut s: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        unsafe {
            return if !s.is_null() {
                s
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
    }

    use super::string_h::strlen;
    extern "C" {
        pub fn istrcmp(
            a: *const ::core::ffi::c_char,
            b: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod keysym_h {
    pub const XKB_KEYSYM_NAME_MAX_SIZE: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{
    xkb_atom_text, xkb_context, xkb_context_get_buffer, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::keymap_h::{
    format_boolean_controls, mod_type, xkb_action_controls, xkb_action_type, xkb_match_operation,
    xkb_mod, xkb_mod_set, _ACTION_TYPE_NUM_ENTRIES, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET,
    ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL,
    ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE,
    ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK,
    ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE,
    ACTION_TYPE_UNKNOWN, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    MATCH_ALL, MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL,
    MOD_REAL_MASK_ALL, MOD_VIRT, XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MOD_NONE,
};
pub use self::keysym_h::XKB_KEYSYM_NAME_MAX_SIZE;
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_uintn_h::{u32, uint8_t};
use self::stdio_h::snprintf;
use self::string_h::memcpy;
pub use self::text_h::{
    format_control_names_offset, C2Rust_Unnamed_1, LookupEntry, CONTROL_NAMES_MIN_V1_INDEX,
    CONTROL_NAMES_MIN_V2_INDEX,
};
pub use self::types_h::{__uint32_t, __uint8_t};
pub use self::utils_h::{istrcmp, istreq, strempty, strlen_safe};
pub use self::xkbcommon_h::{
    xkb_keymap_format, xkb_keysym_get_name, xkb_keysym_t, xkb_log_level, xkb_mod_index_t,
    xkb_mod_mask_t, xkb_rule_names, xkb_state_component, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS,
    XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED,
    XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE,
    XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
};
#[no_mangle]
pub unsafe extern "C" fn LookupString(
    mut tab: *const LookupEntry,
    mut string: *const ::core::ffi::c_char,
    mut value_rtrn: *mut ::core::ffi::c_uint,
) -> bool {
    unsafe {
        if string.is_null() {
            return false_0 != 0;
        }
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(*entry).name.is_null() {
            if istreq((*entry).name, string) {
                *value_rtrn = (*entry).value as ::core::ffi::c_uint;
                return true_0 != 0;
            }
            entry = entry.offset(1);
        }
        return false_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn LookupValue(
    mut tab: *const LookupEntry,
    mut value: ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(*entry).name.is_null() {
            if (*entry).value == value as u32 {
                return (*entry).name;
            }
            entry = entry.offset(1);
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
#[no_mangle]
pub static mut ctrlMaskNames: [LookupEntry; 25] = [
    LookupEntry {
        name: b"Overlay3\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY3 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay4\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY4 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay5\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY5 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay6\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY6 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay7\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY7 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay8\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY8 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_ALL_BOOLEAN as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"RepeatKeys\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_REPEAT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Repeat\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_REPEAT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AutoRepeat\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_REPEAT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SlowKeys\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_SLOW as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"BounceKeys\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_DEBOUNCE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"StickyKeys\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_STICKY_KEYS as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"MouseKeys\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_MOUSE_KEYS as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"MouseKeysAccel\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_MOUSE_KEYS_ACCEL as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AccessXKeys\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_AX as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AccessXTimeout\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_AX_TIMEOUT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AccessXFeedback\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_AX_FEEDBACK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AudibleBell\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_BELL as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"IgnoreGroupLock\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_IGNORE_GROUP_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay1\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY1 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Overlay2\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_OVERLAY2 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
        value: CONTROL_ALL_BOOLEAN_V1 as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"none\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub static mut modComponentMaskNames: [LookupEntry; 8] = [
    LookupEntry {
        name: b"base\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"latched\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_MODS_LATCHED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"locked\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_MODS_LOCKED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"effective\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"compat\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"any\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"none\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub static mut groupComponentMaskNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"base\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"latched\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"locked\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"effective\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"any\0".as_ptr() as *const ::core::ffi::c_char,
        value: XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"none\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub static mut groupMaskNames: [LookupEntry; 3] = [LookupEntry {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    value: 0,
}; 3];
#[no_mangle]
pub static mut buttonNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"Button1\0".as_ptr() as *const ::core::ffi::c_char,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"Button2\0".as_ptr() as *const ::core::ffi::c_char,
        value: 2 as u32,
    },
    LookupEntry {
        name: b"Button3\0".as_ptr() as *const ::core::ffi::c_char,
        value: 3 as u32,
    },
    LookupEntry {
        name: b"Button4\0".as_ptr() as *const ::core::ffi::c_char,
        value: 4 as u32,
    },
    LookupEntry {
        name: b"Button5\0".as_ptr() as *const ::core::ffi::c_char,
        value: 5 as u32,
    },
    LookupEntry {
        name: b"default\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub static mut useModMapValueNames: [LookupEntry; 5] = [
    LookupEntry {
        name: b"LevelOne\0".as_ptr() as *const ::core::ffi::c_char,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"Level1\0".as_ptr() as *const ::core::ffi::c_char,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"AnyLevel\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as u32,
    },
    LookupEntry {
        name: b"any\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub static mut actionTypeNames: [LookupEntry; 43] = [
    LookupEntry {
        name: b"NoAction\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_NONE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"VoidAction\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_VOID as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SetMods\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_MOD_SET as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LatchMods\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_MOD_LATCH as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockMods\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_MOD_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SetGroup\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_GROUP_SET as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LatchGroup\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_GROUP_LATCH as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockGroup\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_GROUP_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"MovePtr\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_MOVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"MovePointer\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_MOVE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"PtrBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_BUTTON as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"PointerButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_BUTTON as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockPtrBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockPtrButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockPointerButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockPointerBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SetPtrDflt\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_DEFAULT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SetPointerDefault\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PTR_DEFAULT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Terminate\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_TERMINATE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"TerminateServer\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_TERMINATE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SwitchScreen\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_SWITCH_VT as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"SetControls\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_CTRL_SET as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockControls\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_CTRL_LOCK as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"RedirectKey\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Redirect\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Private\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_PRIVATE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"ISOLock\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"ActionMessage\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"MessageAction\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Message\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DeviceBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DevBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DevButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DeviceButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockDeviceBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockDevBtn\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockDevButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"LockDeviceButton\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DeviceValuator\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DevVal\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DeviceVal\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"DevValuator\0".as_ptr() as *const ::core::ffi::c_char,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub static mut symInterpretMatchMaskNames: [LookupEntry; 6] = [
    LookupEntry {
        name: b"NoneOf\0".as_ptr() as *const ::core::ffi::c_char,
        value: MATCH_NONE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AnyOfOrNone\0".as_ptr() as *const ::core::ffi::c_char,
        value: MATCH_ANY_OR_NONE as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AnyOf\0".as_ptr() as *const ::core::ffi::c_char,
        value: MATCH_ANY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"AllOf\0".as_ptr() as *const ::core::ffi::c_char,
        value: MATCH_ALL as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"Exactly\0".as_ptr() as *const ::core::ffi::c_char,
        value: MATCH_EXACTLY as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as u32,
    },
];
#[no_mangle]
pub unsafe extern "C" fn ModIndexText(
    mut ctx: *mut xkb_context,
    mut mods: *const xkb_mod_set,
    mut ndx: xkb_mod_index_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if ndx == XKB_MOD_NONE as xkb_mod_index_t {
            return b"None\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if ndx >= (*mods).num_mods {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return xkb_atom_text(ctx, (*mods).mods[ndx as usize].name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ActionTypeText(mut type_0: xkb_action_type) -> *const ::core::ffi::c_char {
    unsafe {
        let mut name: *const ::core::ffi::c_char = LookupValue(
            &raw const actionTypeNames as *const LookupEntry,
            type_0 as ::core::ffi::c_uint,
        );
        return if !name.is_null() {
            name
        } else {
            b"Private\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn KeysymText(
    mut ctx: *mut xkb_context,
    mut sym: xkb_keysym_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut buffer: *mut ::core::ffi::c_char =
            xkb_context_get_buffer(ctx, XKB_KEYSYM_NAME_MAX_SIZE as usize);
        xkb_keysym_get_name(sym, buffer, XKB_KEYSYM_NAME_MAX_SIZE as usize);
        return buffer;
    }
}
#[no_mangle]
pub unsafe extern "C" fn KeyNameText(
    mut ctx: *mut xkb_context,
    mut name: xkb_atom_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut sname: *const ::core::ffi::c_char = xkb_atom_text(ctx, name);
        let mut len: usize = strlen_safe(sname).wrapping_add(3 as usize);
        let mut buf: *mut ::core::ffi::c_char = xkb_context_get_buffer(ctx, len);
        snprintf(
            buf,
            len,
            b"<%s>\0".as_ptr() as *const ::core::ffi::c_char,
            strempty(sname),
        );
        return buf;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SIMatchText(
    mut type_0: xkb_match_operation,
) -> *const ::core::ffi::c_char {
    unsafe {
        return LookupValue(
            &raw const symInterpretMatchMaskNames as *const LookupEntry,
            type_0 as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ModMaskText(
    mut ctx: *mut xkb_context,
    mut type_0: mod_type,
    mut mods: *const xkb_mod_set,
    mut mask: xkb_mod_mask_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut buf: [::core::ffi::c_char; 1024] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut pos: usize = 0 as usize;
        let mut mod_0: *const xkb_mod = ::core::ptr::null::<xkb_mod>();
        if type_0 as ::core::ffi::c_uint == MOD_REAL as ::core::ffi::c_int as ::core::ffi::c_uint
            || type_0 as ::core::ffi::c_uint
                == MOD_BOTH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"type == MOD_REAL || type == MOD_BOTH\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../src/text.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_uint,
                b"const char *ModMaskText(struct xkb_context *, enum mod_type, const struct xkb_mod_set *, xkb_mod_mask_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if mask == 0 as xkb_mod_mask_t {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if mask == MOD_REAL_MASK_ALL {
            return b"all\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if type_0 as ::core::ffi::c_uint == MOD_REAL as ::core::ffi::c_int as ::core::ffi::c_uint
            && mask & !MOD_REAL_MASK_ALL != 0
            || (mask as ::core::ffi::c_ulong
                & !((1 as ::core::ffi::c_ulong) << (*mods).num_mods)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                != 0) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
        {
            let ret: ::core::ffi::c_int = snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize,
                b"0x%x\0".as_ptr() as *const ::core::ffi::c_char,
                mask,
            ) as ::core::ffi::c_int;
            if ret >= 0 as ::core::ffi::c_int
                && (ret as usize) < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize
            {
            } else {
                __assert_fail(
                    b"ret >= 0 && (usize) ret < sizeof(buf)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/text.c\0".as_ptr() as *const ::core::ffi::c_char,
                    245 as ::core::ffi::c_uint,
                    b"const char *ModMaskText(struct xkb_context *, enum mod_type, const struct xkb_mod_set *, xkb_mod_mask_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            pos = ret as usize;
        } else {
            mod_0 = &raw const (*mods).mods as *const xkb_mod;
            while mask != 0
                && mod_0
                    < (&raw const (*mods).mods as *const xkb_mod).offset((*mods).num_mods as isize)
            {
                if mask & 0x1 as xkb_mod_mask_t != 0 {
                    let mut ret_0: ::core::ffi::c_int = snprintf(
                        (&raw mut buf as *mut ::core::ffi::c_char).offset(pos as isize),
                        (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize)
                            .wrapping_sub(pos),
                        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                        if pos == 0 as usize {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"+\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        xkb_atom_text(ctx, (*mod_0).name),
                    );
                    if ret_0 <= 0 as ::core::ffi::c_int
                        || pos.wrapping_add(ret_0 as usize)
                            >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize
                    {
                        break;
                    }
                    pos = pos.wrapping_add(ret_0 as usize);
                }
                mod_0 = mod_0.offset(1);
                mask >>= 1 as ::core::ffi::c_int;
            }
        }
        pos = pos.wrapping_add(1);
        return memcpy(
            xkb_context_get_buffer(ctx, pos) as *mut ::core::ffi::c_void,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            pos,
        ) as *const ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn LedStateMaskText(
    mut ctx: *mut xkb_context,
    mut lookup: *const LookupEntry,
    mut mask: xkb_state_component,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut pos: usize = 0 as usize;
        if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            return b"0\0".as_ptr() as *const ::core::ffi::c_char;
        }
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while mask as u64 != 0 {
            let mut ret: ::core::ffi::c_int = 0;
            if !(mask as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i == 0) {
                mask = (mask as ::core::ffi::c_uint & !((1 as ::core::ffi::c_uint) << i))
                    as xkb_state_component;
                let maskText: *const ::core::ffi::c_char = LookupValue(
                    lookup as *const LookupEntry,
                    (1 as ::core::ffi::c_uint) << i,
                )
                    as *const ::core::ffi::c_char;
                if !maskText.is_null() {
                } else {
                    __assert_fail(
                        b"maskText != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../src/text.c\0".as_ptr() as *const ::core::ffi::c_char,
                        283 as ::core::ffi::c_uint,
                        b"const char *LedStateMaskText(struct xkb_context *, const LookupEntry *, enum xkb_state_component)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                ret = snprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(pos as isize),
                    (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize)
                        .wrapping_sub(pos),
                    b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if pos == 0 as usize {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"+\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    maskText,
                );
                if ret <= 0 as ::core::ffi::c_int
                    || pos.wrapping_add(ret as usize)
                        >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize
                {
                    break;
                }
                pos = pos.wrapping_add(ret as usize);
            }
            i = i.wrapping_add(1);
        }
        pos = pos.wrapping_add(1);
        return memcpy(
            xkb_context_get_buffer(ctx, pos) as *mut ::core::ffi::c_void,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            pos,
        ) as *const ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ControlMaskText(
    mut ctx: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut mask: xkb_action_controls,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut pos: usize = 0 as usize;
        let all_ctrls: xkb_action_controls = format_boolean_controls(format) as xkb_action_controls;
        mask =
            (mask as ::core::ffi::c_uint & all_ctrls as ::core::ffi::c_uint) as xkb_action_controls;
        if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if mask as ::core::ffi::c_uint == all_ctrls as ::core::ffi::c_uint {
            return b"all\0".as_ptr() as *const ::core::ffi::c_char;
        }
        let control_names_offset: uint8_t = format_control_names_offset(format) as uint8_t;
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while mask as u64 != 0 {
            let mut ret: ::core::ffi::c_int = 0;
            if !(mask as ::core::ffi::c_uint & (1 as ::core::ffi::c_uint) << i == 0) {
                mask = (mask as ::core::ffi::c_uint & !((1 as ::core::ffi::c_uint) << i))
                    as xkb_action_controls;
                let maskText: *const ::core::ffi::c_char = LookupValue(
                    (&raw const ctrlMaskNames as *const LookupEntry)
                        .offset(control_names_offset as ::core::ffi::c_int as isize),
                    (1 as ::core::ffi::c_uint) << i,
                )
                    as *const ::core::ffi::c_char;
                if !maskText.is_null() {
                } else {
                    __assert_fail(
                        b"maskText\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../src/text.c\0".as_ptr() as *const ::core::ffi::c_char,
                        323 as ::core::ffi::c_uint,
                        b"const char *ControlMaskText(struct xkb_context *, enum xkb_keymap_format, enum xkb_action_controls)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                ret = snprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(pos as isize),
                    (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize)
                        .wrapping_sub(pos),
                    b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if pos == 0 as usize {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"+\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    maskText,
                );
                if ret <= 0 as ::core::ffi::c_int
                    || pos.wrapping_add(ret as usize)
                        >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize
                {
                    break;
                }
                pos = pos.wrapping_add(ret as usize);
            }
            i = i.wrapping_add(1);
        }
        pos = pos.wrapping_add(1);
        return memcpy(
            xkb_context_get_buffer(ctx, pos) as *mut ::core::ffi::c_void,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            pos,
        ) as *const ::core::ffi::c_char;
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        groupMaskNames = [
            LookupEntry {
                name: b"none\0".as_ptr() as *const ::core::ffi::c_char,
                value: 0 as u32,
            },
            LookupEntry {
                name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
                value: XKB_ALL_GROUPS as u32,
            },
            LookupEntry {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                value: 0 as u32,
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
