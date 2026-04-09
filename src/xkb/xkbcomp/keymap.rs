pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
    pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}

pub mod types_h {
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
}
pub mod stdint_intn_h {
    pub type int8_t = __int8_t;
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
pub mod stdint_h {
    pub type uintptr_t = usize;
    pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
}
pub mod xkbcommon_errors_h {
    pub type xkb_error_code = ::core::ffi::c_int;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT: xkb_error_code = 914;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT: xkb_error_code = 876;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE: xkb_error_code = 450;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS: xkb_error_code = 371;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX: xkb_error_code = 237;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY: xkb_error_code = 214;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK: xkb_error_code = 60;
    pub const XKB_SUCCESS: xkb_error_code = 0;
    pub const XKB_ERROR_INVALID: xkb_error_code = -1;
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

    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
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
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if (need as usize)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as usize)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, usize)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }

    use super::assert_h::__assert_fail;
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
    pub type xkb_layout_index_t = u32;
    pub type xkb_keycode_t = u32;
    pub type xkb_mod_mask_t = u32;
    pub type xkb_mod_index_t = u32;
    pub type xkb_keysym_t = u32;
    pub type xkb_level_index_t = u32;
    pub type xkb_layout_out_of_range_policy = ::core::ffi::c_uint;
    pub const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: xkb_layout_out_of_range_policy = 2;
    pub const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: xkb_layout_out_of_range_policy = 1;
    pub const XKB_LAYOUT_OUT_OF_RANGE_WRAP: xkb_layout_out_of_range_policy = 0;
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
    pub type xkb_layout_mask_t = u32;
    pub type xkb_led_index_t = u32;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    pub const XKB_KEYCODE_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    use super::keymap_h::xkb_keymap;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_keymap_key_get_syms_by_level(
            keymap: *mut xkb_keymap,
            key: xkb_keycode_t,
            layout: xkb_layout_index_t,
            level: xkb_level_index_t,
            syms_out: *mut *const xkb_keysym_t,
        ) -> ::core::ffi::c_int;
    }
}
pub mod keymap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_keymap {
        pub ctx: *mut xkb_context,
        pub refcnt: ::core::ffi::c_int,
        pub flags: xkb_keymap_compile_flags,
        pub format: xkb_keymap_format,
        pub num_leds: xkb_led_index_t,
        pub leds: [xkb_led; 32],
        pub min_key_code: xkb_keycode_t,
        pub max_key_code: xkb_keycode_t,
        pub num_keys: xkb_keycode_t,
        pub num_keys_low: xkb_keycode_t,
        pub keys: *mut xkb_key,
        pub c2rust_unnamed: C2Rust_Unnamed_3,
        pub types: *mut xkb_key_type,
        pub num_types: darray_size_t,
        pub num_sym_interprets: darray_size_t,
        pub sym_interprets: *mut xkb_sym_interpret,
        pub mods: xkb_mod_set,
        pub canonical_state_mask: xkb_mod_mask_t,
        pub redirect_key_auto: xkb_keycode_t,
        pub num_groups: xkb_layout_index_t,
        pub num_group_names: xkb_layout_index_t,
        pub group_names: *mut xkb_atom_t,
        pub keycodes_section_name: *mut ::core::ffi::c_char,
        pub symbols_section_name: *mut ::core::ffi::c_char,
        pub types_section_name: *mut ::core::ffi::c_char,
        pub compat_section_name: *mut ::core::ffi::c_char,
    }
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
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_sym_interpret {
        pub sym: xkb_keysym_t,
        pub match_0: xkb_match_operation,
        pub mods: xkb_mod_mask_t,
        pub virtual_mod: xkb_mod_index_t,
        pub level_one_only: bool,
        #[bitfield(name = "repeat", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "required", ty = "bool", bits = "1..=1")]
        pub repeat_required: [u8; 1],
        pub num_actions: xkb_action_count_t,
        pub a: C2Rust_Unnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_1 {
        pub action: xkb_action,
        pub actions: *mut xkb_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union xkb_action {
        pub type_0: xkb_action_type,
        pub mods: xkb_mod_action,
        pub group: xkb_group_action,
        pub ctrls: xkb_controls_action,
        pub dflt: xkb_pointer_default_action,
        pub screen: xkb_switch_screen_action,
        pub ptr: xkb_pointer_action,
        pub btn: xkb_pointer_button_action,
        pub redirect: xkb_redirect_key_action,
        pub priv_0: xkb_private_action,
        pub internal: xkb_internal_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_internal_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_internal_action_flags,
        pub c2rust_unnamed: C2Rust_Unnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_2 {
        pub clear_latched_mods: xkb_mod_mask_t,
    }
    pub type xkb_internal_action_flags = ::core::ffi::c_uint;
    pub const INTERNAL_BREAKS_MOD_LATCH: xkb_internal_action_flags = 2;
    pub const INTERNAL_BREAKS_GROUP_LATCH: xkb_internal_action_flags = 1;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_private_action {
        pub type_0: xkb_action_type,
        pub data: [uint8_t; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_redirect_key_action {
        pub type_0: xkb_action_type,
        pub keycode: xkb_keycode_t,
        pub affect: xkb_mod_mask_t,
        pub mods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_button_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub count: uint8_t,
        pub button: uint8_t,
    }
    pub type xkb_action_flags = ::core::ffi::c_uint;
    pub const ACTION_PENDING_COMPUTATION: xkb_action_flags = 8192;
    pub const ACTION_LATCH_ON_PRESS: xkb_action_flags = 4096;
    pub const ACTION_UNLOCK_ON_PRESS: xkb_action_flags = 2048;
    pub const ACTION_LOCK_ON_RELEASE: xkb_action_flags = 1024;
    pub const ACTION_SAME_SCREEN: xkb_action_flags = 512;
    pub const ACTION_ACCEL: xkb_action_flags = 256;
    pub const ACTION_ABSOLUTE_Y: xkb_action_flags = 128;
    pub const ACTION_ABSOLUTE_X: xkb_action_flags = 64;
    pub const ACTION_ABSOLUTE_SWITCH: xkb_action_flags = 32;
    pub const ACTION_MODS_LOOKUP_MODMAP: xkb_action_flags = 16;
    pub const ACTION_LOCK_NO_UNLOCK: xkb_action_flags = 8;
    pub const ACTION_LOCK_NO_LOCK: xkb_action_flags = 4;
    pub const ACTION_LATCH_TO_LOCK: xkb_action_flags = 2;
    pub const ACTION_LOCK_CLEAR: xkb_action_flags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub x: int16_t,
        pub y: int16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_switch_screen_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub screen: int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_default_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub value: int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_controls_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub ctrls: xkb_action_controls,
    }
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_group_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub group: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub mods: xkb_mods,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mods {
        pub mods: xkb_mod_mask_t,
        pub mask: xkb_mod_mask_t,
    }
    pub type xkb_action_count_t = uint16_t;
    pub type xkb_match_operation = ::core::ffi::c_uint;
    pub const MATCH_EXACTLY: xkb_match_operation = 4;
    pub const MATCH_ALL: xkb_match_operation = 3;
    pub const MATCH_ANY: xkb_match_operation = 2;
    pub const MATCH_ANY_OR_NONE: xkb_match_operation = 1;
    pub const MATCH_NONE: xkb_match_operation = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_key_type {
        pub name: xkb_atom_t,
        pub mods: xkb_mods,
        pub required: bool,
        pub num_levels: xkb_level_index_t,
        pub num_level_names: xkb_level_index_t,
        pub level_names: *mut xkb_atom_t,
        pub num_entries: darray_size_t,
        pub entries: *mut xkb_key_type_entry,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_key_type_entry {
        pub level: xkb_level_index_t,
        pub mods: xkb_mods,
        pub preserve: xkb_mods,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_3 {
        pub c2rust_unnamed: C2Rust_Unnamed_5,
        pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        pub num_key_aliases: darray_size_t,
        pub key_aliases: *mut xkb_key_alias,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_key_alias {
        pub real: xkb_atom_t,
        pub alias: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        pub num_key_names: darray_size_t,
        pub key_names: *mut KeycodeMatch,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union KeycodeMatch {
        pub c2rust_unnamed: C2Rust_Unnamed_8,
        pub key: C2Rust_Unnamed_7,
        pub alias: C2Rust_Unnamed_6,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_6 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "c2rust_unnamed", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "real", ty = "xkb_atom_t", bits = "3..=31")]
        pub found_c2rust_unnamed_is_alias_real: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_7 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "low", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "index", ty = "darray_size_t", bits = "3..=31")]
        pub found_low_is_alias_index: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_8 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "c2rust_unnamed", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "c2rust_unnamed_0", ty = "darray_size_t", bits = "3..=31")]
        pub found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_key {
        pub keycode: xkb_keycode_t,
        pub name: xkb_atom_t,
        pub explicit: xkb_explicit_components,
        pub modmap: xkb_mod_mask_t,
        pub vmodmap: xkb_mod_mask_t,
        pub overlays: xkb_overlay_mask_t,
        #[bitfield(name = "overlays_inline", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "repeats", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "implicit_actions", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "out_of_range_pending_group", ty = "bool", bits = "3..=3")]
        #[bitfield(
            name = "out_of_range_group_policy",
            ty = "xkb_layout_out_of_range_policy",
            bits = "4..=7"
        )]
        #[bitfield(
            name = "out_of_range_group_number",
            ty = "xkb_layout_index_t",
            bits = "8..=15"
        )]
        #[bitfield(name = "num_groups", ty = "xkb_layout_index_t", bits = "16..=23")]
        pub overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups:
            [u8; 3],
        pub groups: *mut xkb_group,
        pub c2rust_unnamed: C2Rust_Unnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_9 {
        pub overlay_key: *const xkb_key,
        pub overlays_keys: *mut *const xkb_key,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_group {
        #[bitfield(name = "explicit_symbols", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "explicit_actions", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "implicit_actions", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "explicit_type", ty = "bool", bits = "3..=3")]
        pub explicit_symbols_explicit_actions_implicit_actions_explicit_type: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub type_0: *const xkb_key_type,
        pub levels: *mut xkb_level,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_level {
        pub num_syms: xkb_keysym_count_t,
        pub num_actions: xkb_action_count_t,
        pub c2rust_unnamed: C2Rust_Unnamed_12,
        pub s: C2Rust_Unnamed_11,
        pub a: C2Rust_Unnamed_10,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_10 {
        pub action: xkb_action,
        pub actions: *mut xkb_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_11 {
        pub sym: xkb_keysym_t,
        pub syms: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_12 {
        pub upper: xkb_keysym_t,
        pub has_upper: bool,
    }
    pub type xkb_keysym_count_t = uint16_t;
    pub type xkb_overlay_mask_t = uint8_t;
    pub type xkb_explicit_components = ::core::ffi::c_uint;
    pub const EXPLICIT_OVERLAY: xkb_explicit_components = 32;
    pub const EXPLICIT_REPEAT: xkb_explicit_components = 16;
    pub const EXPLICIT_VMODMAP: xkb_explicit_components = 8;
    pub const EXPLICIT_TYPES: xkb_explicit_components = 4;
    pub const EXPLICIT_INTERP: xkb_explicit_components = 2;
    pub const EXPLICIT_SYMBOLS: xkb_explicit_components = 1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_led {
        pub name: xkb_atom_t,
        #[bitfield(name = "which_groups", ty = "xkb_state_component", bits = "0..=30")]
        #[bitfield(name = "pending_groups", ty = "bool", bits = "31..=31")]
        pub which_groups_pending_groups: [u8; 4],
        pub groups: xkb_layout_mask_t,
        pub which_mods: xkb_state_component,
        pub mods: xkb_mods,
        pub ctrls: xkb_action_controls,
    }
    pub type real_mod_index = ::core::ffi::c_uint;
    pub const _XKB_MOD_INDEX_NUM_ENTRIES: real_mod_index = 8;
    pub const XKB_MOD_INDEX_MOD5: real_mod_index = 7;
    pub const XKB_MOD_INDEX_MOD4: real_mod_index = 6;
    pub const XKB_MOD_INDEX_MOD3: real_mod_index = 5;
    pub const XKB_MOD_INDEX_MOD2: real_mod_index = 4;
    pub const XKB_MOD_INDEX_MOD1: real_mod_index = 3;
    pub const XKB_MOD_INDEX_CTRL: real_mod_index = 2;
    pub const XKB_MOD_INDEX_CAPS: real_mod_index = 1;
    pub const XKB_MOD_INDEX_SHIFT: real_mod_index = 0;
    pub type xkb_overlay_index_t = uint8_t;
    pub type C2Rust_Unnamed_14 = ::core::ffi::c_uint;
    pub const FALLBACK_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_14 = 0;
    pub const DEFAULT_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_14 = 1;
    pub const DEFAULT_KEY_REPEAT: C2Rust_Unnamed_14 = 0;
    pub type C2Rust_Unnamed_15 = ::core::ffi::c_uint;
    pub const FALLBACK_INTERPRET_VMODMAP: C2Rust_Unnamed_15 = 0;
    pub const DEFAULT_INTERPRET_VMODMAP: C2Rust_Unnamed_15 = 0;
    pub const DEFAULT_INTERPRET_VMOD: C2Rust_Unnamed_15 = 4294967295;
    pub const DEFAULT_KEY_VMODMAP: C2Rust_Unnamed_15 = 0;
    pub const XKB_MAX_GROUPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    pub const XKB_ALL_GROUPS: ::core::ffi::c_ulong =
        ((1 as ::core::ffi::c_ulong) << XKB_MAX_GROUPS).wrapping_sub(1 as ::core::ffi::c_ulong);
    pub const XKB_MAX_GROUPS_X11: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    #[inline]
    pub unsafe extern "C" fn format_max_groups(
        mut format: xkb_keymap_format,
    ) -> xkb_layout_index_t {
        unsafe {
            return (if format as ::core::ffi::c_uint
                == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                XKB_MAX_GROUPS_X11
            } else {
                XKB_MAX_GROUPS
            }) as xkb_layout_index_t;
        }
    }
    pub const XKB_OVERLAY_MAX_X11: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    pub const XKB_OVERLAY_MAX: ::core::ffi::c_ulong = (::core::mem::size_of::<xkb_overlay_mask_t>()
        as ::core::ffi::c_ulong)
        .wrapping_mul(CHAR_BIT as ::core::ffi::c_ulong);
    #[inline]
    pub unsafe extern "C" fn format_max_overlays(
        mut format: xkb_keymap_format,
    ) -> xkb_overlay_index_t {
        unsafe {
            return (if format as ::core::ffi::c_uint
                == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                XKB_OVERLAY_MAX_X11 as usize
            } else {
                XKB_OVERLAY_MAX as usize
            }) as xkb_overlay_index_t;
        }
    }
    pub const MAX_ACTIONS_PER_LEVEL: ::core::ffi::c_int = UINT16_MAX;
    #[inline]
    pub unsafe extern "C" fn XkbKeyNumLevels(
        mut key: *const xkb_key,
        mut layout: xkb_layout_index_t,
    ) -> xkb_level_index_t {
        unsafe {
            return (*(*(*key).groups.offset(layout as isize)).type_0).num_levels;
        }
    }
    #[inline]
    pub unsafe extern "C" fn isModsUnLockOnPressSupported(mut format: xkb_keymap_format) -> bool {
        unsafe {
            return format as ::core::ffi::c_uint
                >= XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
    }
    #[inline]
    pub unsafe extern "C" fn isGroupLockOnReleaseSupported(mut format: xkb_keymap_format) -> bool {
        unsafe {
            return format as ::core::ffi::c_uint
                >= XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
    }
    #[inline]
    pub unsafe extern "C" fn isModsLatchOnPressSupported(mut format: xkb_keymap_format) -> bool {
        unsafe {
            return format as ::core::ffi::c_uint
                >= XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
    }
    #[inline]
    pub unsafe extern "C" fn areOverlappingOverlaysSupported(
        mut format: xkb_keymap_format,
    ) -> bool {
        unsafe {
            return format as ::core::ffi::c_uint
                >= XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
    }
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::limits_h::CHAR_BIT;
    use super::stdint_h::UINT16_MAX;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    use super::xkbcommon_h::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_level_index_t, xkb_mod_index_t, xkb_mod_mask_t, xkb_state_component,
        XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    };
    extern "C" {
        pub fn mod_mask_get_effective(
            keymap: *mut xkb_keymap,
            mods: xkb_mod_mask_t,
        ) -> xkb_mod_mask_t;
    }
}
pub mod ast_h {
    pub type xkb_file_type = ::core::ffi::c_uint;
    pub const FILE_TYPE_INVALID: xkb_file_type = 7;
    pub const _FILE_TYPE_NUM_ENTRIES: xkb_file_type = 7;
    pub const FILE_TYPE_RULES: xkb_file_type = 6;
    pub const FILE_TYPE_KEYMAP: xkb_file_type = 5;
    pub const FILE_TYPE_GEOMETRY: xkb_file_type = 4;
    pub const LAST_KEYMAP_FILE_TYPE: xkb_file_type = 3;
    pub const FIRST_KEYMAP_FILE_TYPE: xkb_file_type = 0;
    pub const FILE_TYPE_SYMBOLS: xkb_file_type = 3;
    pub const FILE_TYPE_COMPAT: xkb_file_type = 2;
    pub const FILE_TYPE_TYPES: xkb_file_type = 1;
    pub const FILE_TYPE_KEYCODES: xkb_file_type = 0;
    pub type stmt_type = ::core::ffi::c_uint;
    pub const _STMT_NUM_VALUES: stmt_type = 37;
    pub const STMT_UNKNOWN_COMPOUND: stmt_type = 36;
    pub const STMT_UNKNOWN_DECLARATION: stmt_type = 35;
    pub const STMT_LED_NAME: stmt_type = 34;
    pub const STMT_LED_MAP: stmt_type = 33;
    pub const STMT_GROUP_COMPAT: stmt_type = 32;
    pub const STMT_MODMAP: stmt_type = 31;
    pub const STMT_SYMBOLS: stmt_type = 30;
    pub const STMT_VMOD: stmt_type = 29;
    pub const STMT_INTERP: stmt_type = 28;
    pub const STMT_TYPE: stmt_type = 27;
    pub const STMT_VAR: stmt_type = 26;
    pub const STMT_EXPR_UNARY_PLUS: stmt_type = 25;
    pub const STMT_EXPR_INVERT: stmt_type = 24;
    pub const STMT_EXPR_NEGATE: stmt_type = 23;
    pub const STMT_EXPR_NOT: stmt_type = 22;
    pub const STMT_EXPR_ASSIGN: stmt_type = 21;
    pub const STMT_EXPR_DIVIDE: stmt_type = 20;
    pub const STMT_EXPR_MULTIPLY: stmt_type = 19;
    pub const STMT_EXPR_SUBTRACT: stmt_type = 18;
    pub const STMT_EXPR_ADD: stmt_type = 17;
    pub const STMT_EXPR_ACTION_LIST: stmt_type = 16;
    pub const STMT_EXPR_KEYSYM_LIST: stmt_type = 15;
    pub const STMT_EXPR_EMPTY_LIST: stmt_type = 14;
    pub const STMT_EXPR_ARRAY_REF: stmt_type = 13;
    pub const STMT_EXPR_FIELD_REF: stmt_type = 12;
    pub const STMT_EXPR_ACTION_DECL: stmt_type = 11;
    pub const STMT_EXPR_IDENT: stmt_type = 10;
    pub const STMT_EXPR_KEYSYM_LITERAL: stmt_type = 9;
    pub const STMT_EXPR_KEYNAME_LITERAL: stmt_type = 8;
    pub const STMT_EXPR_BOOLEAN_LITERAL: stmt_type = 7;
    pub const STMT_EXPR_FLOAT_LITERAL: stmt_type = 6;
    pub const STMT_EXPR_INTEGER_LITERAL: stmt_type = 5;
    pub const STMT_EXPR_STRING_LITERAL: stmt_type = 4;
    pub const STMT_ALIAS: stmt_type = 3;
    pub const STMT_KEYCODE: stmt_type = 2;
    pub const STMT_INCLUDE: stmt_type = 1;
    pub const STMT_UNKNOWN: stmt_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ParseCommon {
        pub next: *mut _ParseCommon,
        pub type_0: stmt_type,
    }
    pub type ParseCommon = _ParseCommon;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union ExprDef {
        pub common: ParseCommon,
        pub ident: ExprIdent,
        pub string: ExprString,
        pub boolean: ExprBoolean,
        pub integer: ExprInteger,
        pub key_name: ExprKeyName,
        pub keysym: ExprKeySym,
        pub binary: ExprBinary,
        pub unary: ExprUnary,
        pub field_ref: ExprFieldRef,
        pub array_ref: ExprArrayRef,
        pub action: ExprAction,
        pub actions: ExprActionList,
        pub keysym_list: ExprKeysymList,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeysymList {
        pub common: ParseCommon,
        pub syms: C2Rust_Unnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_13 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprActionList {
        pub common: ParseCommon,
        pub actions: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprAction {
        pub common: ParseCommon,
        pub name: xkb_atom_t,
        pub args: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprArrayRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
        pub entry: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprFieldRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprUnary {
        pub common: ParseCommon,
        pub child: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBinary {
        pub common: ParseCommon,
        pub left: *mut ExprDef,
        pub right: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeySym {
        pub common: ParseCommon,
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeyName {
        pub common: ParseCommon,
        pub key_name: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprInteger {
        pub common: ParseCommon,
        pub ival: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBoolean {
        pub common: ParseCommon,
        pub set: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprString {
        pub common: ParseCommon,
        pub str: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprIdent {
        pub common: ParseCommon,
        pub ident: xkb_atom_t,
    }
    pub type xkb_map_flags = ::core::ffi::c_uint;
    pub const MAP_IS_ALTGR: xkb_map_flags = 128;
    pub const MAP_HAS_FN: xkb_map_flags = 64;
    pub const MAP_HAS_KEYPAD: xkb_map_flags = 32;
    pub const MAP_HAS_MODIFIER: xkb_map_flags = 16;
    pub const MAP_HAS_ALPHANUMERIC: xkb_map_flags = 8;
    pub const MAP_IS_HIDDEN: xkb_map_flags = 4;
    pub const MAP_IS_PARTIAL: xkb_map_flags = 2;
    pub const MAP_IS_DEFAULT: xkb_map_flags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XkbFile {
        pub common: ParseCommon,
        pub name: *mut ::core::ffi::c_char,
        pub defs: *mut ParseCommon,
        pub file_type: xkb_file_type,
        pub flags: xkb_map_flags,
    }
    use super::atom_h::xkb_atom_t;
    use super::darray_h::darray_size_t;
    use super::stdint_intn_h::int64_t;
    use super::xkbcommon_h::xkb_keysym_t;
    extern "C" {
        pub fn xkb_file_type_to_string(type_0: xkb_file_type) -> *const ::core::ffi::c_char;
    }
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
    pub type xkb_message_code = ::core::ffi::c_uint;
    pub const _XKB_LOG_MESSAGE_MAX_CODE: xkb_message_code = 971;
    pub const XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE: xkb_message_code = 971;
    pub const XKB_ERROR_INVALID_RULES_SYNTAX: xkb_message_code = 967;
    pub const XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL: xkb_message_code = 965;
    pub const XKB_ERROR_INVALID_IDENTIFIER: xkb_message_code = 949;
    pub const XKB_WARNING_CONFLICTING_KEY_FIELDS: xkb_message_code = 935;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT_: xkb_message_code = 914;
    pub const XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX: xkb_message_code = 903;
    pub const XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY: xkb_message_code = 901;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS: xkb_message_code = 893;
    pub const XKB_WARNING_CONFLICTING_KEY_ACTION: xkb_message_code = 883;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT_: xkb_message_code = 876;
    pub const XKB_ERROR_UNKNOWN_ACTION_TYPE: xkb_message_code = 844;
    pub const XKB_ERROR_KEYMAP_COMPILATION_FAILED: xkb_message_code = 822;
    pub const XKB_ERROR_UNKNOWN_FIELD: xkb_message_code = 812;
    pub const XKB_WARNING_CONFLICTING_MODMAP: xkb_message_code = 800;
    pub const XKB_ERROR_INVALID_VALUE: xkb_message_code = 796;
    pub const XKB_ERROR_INVALID_EXPRESSION_TYPE: xkb_message_code = 784;
    pub const XKB_WARNING_UNDEFINED_KEYCODE: xkb_message_code = 770;
    pub const XKB_ERROR_INVALID_XKB_SYNTAX: xkb_message_code = 769;
    pub const XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION: xkb_message_code = 762;
    pub const XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT: xkb_message_code = 742;
    pub const XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD: xkb_message_code = 711;
    pub const XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE: xkb_message_code = 700;
    pub const XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT: xkb_message_code = 693;
    pub const XKB_ERROR_INVALID_COMPOSE_SYNTAX: xkb_message_code = 685;
    pub const XKB_ERROR_INVALID_COMPOSE_LOCALE: xkb_message_code = 679;
    pub const XKB_ERROR_INVALID_INCLUDED_FILE: xkb_message_code = 661;
    pub const XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE: xkb_message_code = 645;
    pub const XKB_ERROR_UNKNOWN_DEFAULT_FIELD: xkb_message_code = 639;
    pub const XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH: xkb_message_code = 632;
    pub const XKB_ERROR_INVALID_REAL_MODIFIER: xkb_message_code = 623;
    pub const XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE: xkb_message_code = 607;
    pub const XKB_ERROR_CANNOT_RESOLVE_RMLVO: xkb_message_code = 595;
    pub const XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX: xkb_message_code = 588;
    pub const XKB_ERROR_WRONG_FIELD_TYPE: xkb_message_code = 578;
    pub const XKB_ERROR_INVALID_ACTION_FIELD: xkb_message_code = 563;
    pub const XKB_ERROR_ALLOCATION_ERROR: xkb_message_code = 550;
    pub const XKB_ERROR_INVALID_FILE_ENCODING: xkb_message_code = 542;
    pub const XKB_WARNING_CONFLICTING_KEY_NAME: xkb_message_code = 523;
    pub const XKB_WARNING_EXTRA_SYMBOLS_IGNORED: xkb_message_code = 516;
    pub const XKB_WARNING_NUMERIC_KEYSYM: xkb_message_code = 489;
    pub const XKB_ERROR_INVALID_OPERATION: xkb_message_code = 478;
    pub const XKB_WARNING_CONFLICTING_KEY_SYMBOL: xkb_message_code = 461;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE_: xkb_message_code = 450;
    pub const XKB_WARNING_MISSING_DEFAULT_SECTION: xkb_message_code = 433;
    pub const XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE: xkb_message_code = 428;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS: xkb_message_code = 407;
    pub const XKB_ERROR_RECURSIVE_INCLUDE: xkb_message_code = 386;
    pub const XKB_WARNING_DUPLICATE_ENTRY: xkb_message_code = 378;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_: xkb_message_code = 371;
    pub const XKB_WARNING_UNSUPPORTED_LEGACY_ACTION: xkb_message_code = 362;
    pub const XKB_ERROR_OVERLAPPING_OVERLAY: xkb_message_code = 355;
    pub const XKB_ERROR_UNKNOWN_OPERATOR: xkb_message_code = 345;
    pub const XKB_ERROR_INCLUDED_FILE_NOT_FOUND: xkb_message_code = 338;
    pub const XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL: xkb_message_code = 312;
    pub const XKB_WARNING_NON_BASE_GROUP_NAME: xkb_message_code = 305;
    pub const XKB_WARNING_DEPRECATED_KEYSYM_NAME: xkb_message_code = 302;
    pub const XKB_WARNING_DEPRECATED_KEYSYM: xkb_message_code = 301;
    pub const XKB_WARNING_UNDEFINED_KEY_TYPE: xkb_message_code = 286;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY: xkb_message_code = 266;
    pub const XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT: xkb_message_code = 254;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES: xkb_message_code = 239;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_: xkb_message_code = 237;
    pub const XKB_ERROR_UNKNOWN_STATEMENT: xkb_message_code = 222;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_: xkb_message_code = 214;
    pub const XKB_ERROR_INVALID_MODMAP_ENTRY: xkb_message_code = 206;
    pub const XKB_ERROR_INVALID_INCLUDE_STATEMENT: xkb_message_code = 203;
    pub const XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT: xkb_message_code = 195;
    pub const XKB_WARNING_INVALID_ESCAPE_SEQUENCE: xkb_message_code = 193;
    pub const XKB_WARNING_CANNOT_INFER_KEY_TYPE: xkb_message_code = 183;
    pub const XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION: xkb_message_code = 172;
    pub const XKB_ERROR_INVALID_PATH: xkb_message_code = 161;
    pub const XKB_ERROR_WRONG_STATEMENT_TYPE: xkb_message_code = 150;
    pub const XKB_ERROR_INSUFFICIENT_BUFFER_SIZE: xkb_message_code = 134;
    pub const XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER: xkb_message_code = 123;
    pub const XKB_WARNING_UNRECOGNIZED_KEYSYM: xkb_message_code = 107;
    pub const XKB_WARNING_ILLEGAL_KEYCODE_ALIAS: xkb_message_code = 101;
    pub const XKB_ERROR_INVALID_NUMERIC_KEYSYM: xkb_message_code = 82;
    pub const XKB_ERROR_EXPECTED_ARRAY_ENTRY: xkb_message_code = 77;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_: xkb_message_code = 60;
    pub const XKB_ERROR_INTEGER_OVERFLOW: xkb_message_code = 52;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES: xkb_message_code = 43;
    pub const XKB_ERROR_MALFORMED_NUMBER_LITERAL: xkb_message_code = 34;
    pub const _XKB_LOG_MESSAGE_MIN_CODE: xkb_message_code = 34;
}
pub mod text_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LookupEntry {
        pub name: *const ::core::ffi::c_char,
        pub value: u32,
    }
    pub type C2Rust_Unnamed_16 = ::core::ffi::c_uint;
    pub const CONTROL_NAMES_MIN_V2_INDEX: C2Rust_Unnamed_16 = 0;
    pub const CONTROL_NAMES_MIN_V1_INDEX: C2Rust_Unnamed_16 = 7;
    pub const GROUP_LAST_INDEX_NAME: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"last\0") };
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
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_action_type;
    use super::stdint_uintn_h::{u32, uint8_t};
    use super::xkbcommon_h::{xkb_keymap_format, xkb_keysym_t, XKB_KEYMAP_FORMAT_TEXT_V1};
    extern "C" {
        pub fn ActionTypeText(type_0: xkb_action_type) -> *const ::core::ffi::c_char;
        pub fn KeysymText(ctx: *mut xkb_context, sym: xkb_keysym_t) -> *const ::core::ffi::c_char;
        pub fn KeyNameText(ctx: *mut xkb_context, name: xkb_atom_t) -> *const ::core::ffi::c_char;
    }
}
pub mod xkbcomp_priv_h {
    pub type xkb_parser_error = ::core::ffi::c_uint;
    pub const PARSER_FATAL_ERROR: xkb_parser_error = 2;
    pub const PARSER_RECOVERABLE_ERROR: xkb_parser_error = 1;
    pub const PARSER_SUCCESS: xkb_parser_error = 0;
    pub type xkb_parser_strict_flags = ::core::ffi::c_uint;
    pub const PARSER_V2_LAX_FLAGS: xkb_parser_strict_flags = 0;
    pub const PARSER_V2_STRICT_FLAGS: xkb_parser_strict_flags = 16383;
    pub const PARSER_V1_LAX_FLAGS: xkb_parser_strict_flags = 16379;
    pub const PARSER_V1_STRICT_FLAGS: xkb_parser_strict_flags = 16383;
    pub const PARSER_NO_ILLEGAL_ACTION_FIELDS: xkb_parser_strict_flags = 8192;
    pub const PARSER_NO_UNKNOWN_ACTION_FIELDS: xkb_parser_strict_flags = 4096;
    pub const PARSER_NO_UNKNOWN_ACTION: xkb_parser_strict_flags = 2048;
    pub const PARSER_NO_UNKNOWN_KEY_FIELDS: xkb_parser_strict_flags = 1024;
    pub const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: xkb_parser_strict_flags = 512;
    pub const PARSER_NO_UNKNOWN_LED_FIELDS: xkb_parser_strict_flags = 256;
    pub const PARSER_NO_UNKNOWN_INTERPRET_FIELDS: xkb_parser_strict_flags = 128;
    pub const PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS: xkb_parser_strict_flags = 64;
    pub const PARSER_NO_UNKNOWN_TYPE_FIELDS: xkb_parser_strict_flags = 32;
    pub const PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS: xkb_parser_strict_flags = 16;
    pub const PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS: xkb_parser_strict_flags = 8;
    pub const PARSER_NO_FIELD_VALUE_MISMATCH: xkb_parser_strict_flags = 4;
    pub const PARSER_NO_FIELD_TYPE_MISMATCH: xkb_parser_strict_flags = 2;
    pub const PARSER_NO_UNKNOWN_STATEMENTS: xkb_parser_strict_flags = 1;
    pub const PARSER_NO_STRICT_FLAGS: xkb_parser_strict_flags = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pending_computation {
        pub expr: *mut ExprDef,
        pub computed: bool,
        pub value: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pending_computation_array {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut pending_computation,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_keymap_info {
        pub keymap: xkb_keymap,
        pub strict: xkb_parser_strict_flags,
        pub features: C2Rust_Unnamed_18,
        pub lookup: C2Rust_Unnamed_17,
        pub pending_computations: *mut pending_computation_array,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_17 {
        pub groupIndexNames: [LookupEntry; 3],
        pub groupMaskNames: [LookupEntry; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_18 {
        pub max_groups: xkb_layout_index_t,
        pub max_overlays: xkb_overlay_index_t,
        pub controls_name_offset: uint8_t,
        pub group_lock_on_release: bool,
        pub mods_unlock_on_press: bool,
        pub mods_latch_on_press: bool,
        pub overlapping_overlays: bool,
    }
    #[inline]
    pub unsafe extern "C" fn safe_map_name(mut file: *mut XkbFile) -> *const ::core::ffi::c_char {
        unsafe {
            return if !(*file).name.is_null() {
                (*file).name as *const ::core::ffi::c_char
            } else {
                b"(unnamed map)\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
    }
    use super::ast_h::{ExprDef, XkbFile};
    use super::darray_h::darray_size_t;
    use super::keymap_h::{xkb_keymap, xkb_overlay_index_t};
    use super::stdint_uintn_h::{u32, uint8_t};
    use super::text_h::LookupEntry;
    use super::xkbcommon_h::xkb_layout_index_t;
    extern "C" {
        pub fn CompileKeycodes(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool;
        pub fn CompileKeyTypes(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool;
        pub fn CompileCompatMap(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool;
        pub fn CompileSymbols(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn is_aligned(
        mut pointer: *const ::core::ffi::c_void,
        mut byte_count: usize,
    ) -> bool {
        unsafe {
            return (pointer as uintptr_t).wrapping_rem(byte_count as uintptr_t) == 0 as uintptr_t;
        }
    }
    #[inline]
    pub unsafe extern "C" fn memdup(
        mut mem: *const ::core::ffi::c_void,
        mut nmemb: usize,
        mut size: usize,
    ) -> *mut ::core::ffi::c_void {
        unsafe {
            let mut p: *mut ::core::ffi::c_void = calloc(nmemb, size);
            if !p.is_null() {
                memcpy(p, mem, nmemb.wrapping_mul(size));
            }
            return p;
        }
    }

    use super::stdint_h::uintptr_t;
    use super::stdlib_h::calloc;
    use super::string_h::memcpy;
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
pub mod ast_build_h {
    use super::ast_h::ParseCommon;
    extern "C" {
        pub fn FreeStmt(stmt: *mut ParseCommon);
    }
}
pub mod expr_h {
    use super::ast_h::ExprDef;
    use super::xkbcommon_h::{xkb_layout_index_t, xkb_layout_mask_t};
    use super::xkbcomp_priv_h::{xkb_keymap_info, xkb_parser_error};
    extern "C" {
        pub fn ExprResolveGroup(
            keymap_info: *const xkb_keymap_info,
            expr: *const ExprDef,
            absolute: bool,
            group_rtrn: *mut xkb_layout_index_t,
            pending_rtrn: *mut bool,
        ) -> xkb_parser_error;
        pub fn ExprResolveGroupMask(
            keymap_info: *const xkb_keymap_info,
            expr: *const ExprDef,
            group_rtrn: *mut xkb_layout_mask_t,
            pending_rtrn: *mut bool,
        ) -> bool;
    }
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
use self::ast_build_h::FreeStmt;
pub use self::ast_h::{
    _ParseCommon, stmt_type, xkb_file_type, xkb_file_type_to_string, xkb_map_flags,
    C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef,
    ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString,
    ExprUnary, ParseCommon, XkbFile, _FILE_TYPE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES,
    FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
    MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR,
    MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, STMT_ALIAS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN,
    STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF,
    STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT,
    STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY,
    STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT,
    STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP,
    STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
use self::expr_h::{ExprResolveGroup, ExprResolveGroupMask};
pub use self::internal::{__va_list_tag, __CHAR_BIT__};
pub use self::keymap_h::{
    areOverlappingOverlaysSupported, format_max_groups, format_max_overlays,
    isGroupLockOnReleaseSupported, isModsLatchOnPressSupported, isModsUnLockOnPressSupported,
    mod_mask_get_effective, mod_type, real_mod_index, xkb_action, xkb_action_controls,
    xkb_action_count_t, xkb_action_flags, xkb_action_type, xkb_controls_action,
    xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action,
    xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type, xkb_key_type_entry,
    xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation, xkb_mod,
    xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_14, C2Rust_Unnamed_15,
    C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6,
    C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, XkbKeyNumLevels,
    _ACTION_TYPE_NUM_ENTRIES, _XKB_MOD_INDEX_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
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
    DEFAULT_INTERPRET_KEY_REPEAT, DEFAULT_INTERPRET_VMOD, DEFAULT_INTERPRET_VMODMAP,
    DEFAULT_KEY_REPEAT, DEFAULT_KEY_VMODMAP, EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT,
    EXPLICIT_SYMBOLS, EXPLICIT_TYPES, EXPLICIT_VMODMAP, FALLBACK_INTERPRET_KEY_REPEAT,
    FALLBACK_INTERPRET_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL,
    MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MAX_ACTIONS_PER_LEVEL, MOD_BOTH,
    MOD_REAL, MOD_VIRT, XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MAX_GROUPS_X11, XKB_MOD_INDEX_CAPS,
    XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1, XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3,
    XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT, XKB_OVERLAY_MAX,
    XKB_OVERLAY_MAX_X11,
};
pub use self::limits_h::CHAR_BIT;
pub use self::messages_codes_h::{
    xkb_log_verbosity, xkb_message_code, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
    XKB_ERROR_ABI_BACKWARD_COMPAT_, XKB_ERROR_ABI_FORWARD_COMPAT_,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR, XKB_ERROR_CANNOT_RESOLVE_RMLVO,
    XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY, XKB_ERROR_EXPECTED_ARRAY_ENTRY,
    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE, XKB_ERROR_INCLUDED_FILE_NOT_FOUND,
    XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT, XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT,
    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE, XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD,
    XKB_ERROR_INVALID_COMPOSE_LOCALE, XKB_ERROR_INVALID_COMPOSE_SYNTAX,
    XKB_ERROR_INVALID_EXPRESSION_TYPE, XKB_ERROR_INVALID_FILE_ENCODING,
    XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
    XKB_ERROR_INVALID_INCLUDE_STATEMENT, XKB_ERROR_INVALID_MODMAP_ENTRY,
    XKB_ERROR_INVALID_NUMERIC_KEYSYM, XKB_ERROR_INVALID_OPERATION, XKB_ERROR_INVALID_PATH,
    XKB_ERROR_INVALID_REAL_MODIFIER, XKB_ERROR_INVALID_RULES_SYNTAX,
    XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT, XKB_ERROR_INVALID_VALUE, XKB_ERROR_INVALID_XKB_SYNTAX,
    XKB_ERROR_KEYMAP_COMPILATION_FAILED, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_ERROR_OVERLAPPING_OVERLAY,
    XKB_ERROR_RECURSIVE_INCLUDE, XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION,
    XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER, XKB_ERROR_UNKNOWN_ACTION_TYPE,
    XKB_ERROR_UNKNOWN_DEFAULT_FIELD, XKB_ERROR_UNKNOWN_FIELD, XKB_ERROR_UNKNOWN_OPERATOR,
    XKB_ERROR_UNKNOWN_STATEMENT, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_, XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX,
    XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL, XKB_ERROR_WRONG_FIELD_TYPE, XKB_ERROR_WRONG_STATEMENT_TYPE,
    XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE, XKB_LOG_VERBOSITY_DEFAULT,
    XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL, XKB_LOG_VERBOSITY_SILENT,
    XKB_LOG_VERBOSITY_VERBOSE, XKB_WARNING_CANNOT_INFER_KEY_TYPE,
    XKB_WARNING_CONFLICTING_KEY_ACTION, XKB_WARNING_CONFLICTING_KEY_FIELDS,
    XKB_WARNING_CONFLICTING_KEY_NAME, XKB_WARNING_CONFLICTING_KEY_SYMBOL,
    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS, XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY, XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES, XKB_WARNING_CONFLICTING_MODMAP,
    XKB_WARNING_DEPRECATED_KEYSYM, XKB_WARNING_DEPRECATED_KEYSYM_NAME, XKB_WARNING_DUPLICATE_ENTRY,
    XKB_WARNING_EXTRA_SYMBOLS_IGNORED, XKB_WARNING_ILLEGAL_KEYCODE_ALIAS,
    XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT, XKB_WARNING_INVALID_ESCAPE_SEQUENCE,
    XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE, XKB_WARNING_MISSING_DEFAULT_SECTION,
    XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX, XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE,
    XKB_WARNING_NON_BASE_GROUP_NAME, XKB_WARNING_NUMERIC_KEYSYM,
    XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE, XKB_WARNING_UNDEFINED_KEYCODE,
    XKB_WARNING_UNDEFINED_KEY_TYPE, XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
    XKB_WARNING_UNRECOGNIZED_KEYSYM, XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL,
    XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION, XKB_WARNING_UNSUPPORTED_LEGACY_ACTION,
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::{uintptr_t, UINT16_MAX};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::{calloc, free, realloc};
use self::string_h::memcpy;
pub use self::text_h::{
    format_control_names_offset, ActionTypeText, C2Rust_Unnamed_16, KeyNameText, KeysymText,
    LookupEntry, CONTROL_NAMES_MIN_V1_INDEX, CONTROL_NAMES_MIN_V2_INDEX, GROUP_LAST_INDEX_NAME,
};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::utils_h::{is_aligned, memdup};
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keymap_key_get_syms_by_level,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_led_index_t, xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t,
    xkb_rule_names, xkb_state_component, XKB_KEYCODE_INVALID, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_error, xkb_parser_strict_flags, C2Rust_Unnamed_17, C2Rust_Unnamed_18,
    CompileCompatMap, CompileKeyTypes, CompileKeycodes, CompileSymbols, PARSER_FATAL_ERROR,
    PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS,
    PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
    PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
    PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_sym_interprets {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *const xkb_sym_interpret,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_action,
}
pub const GROUP_MASK_NAME_LAST: C2Rust_Unnamed_21 = 3;
pub const GROUP_INDEX_NAME_LAST: C2Rust_Unnamed_20 = 1;
pub type compile_file_fn = Option<unsafe extern "C" fn(*mut XkbFile, *mut xkb_keymap_info) -> bool>;
pub type C2Rust_Unnamed_20 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_21 = ::core::ffi::c_uint;
unsafe extern "C" fn has_unbound_vmods(
    mut keymap: *mut xkb_keymap,
    mut mask: xkb_mod_mask_t,
) -> bool {
    unsafe {
        let mut k: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        k = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
            as *mut xkb_mod;
        while k < (*keymap).mods.num_mods {
            if mask & (1 as xkb_mod_mask_t) << k != 0 && (*mod_0).mapping == 0 as xkb_mod_mask_t {
                return true_0 != 0;
            }
            k = k.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        return false_0 != 0;
    }
}
#[inline]
unsafe extern "C" fn ComputeEffectiveMask(mut keymap: *mut xkb_keymap, mut mods: *mut xkb_mods) {
    unsafe {
        let unknown_mods: xkb_mod_mask_t =
            !(((1 as ::core::ffi::c_ulong) << (*keymap).mods.num_mods)
                .wrapping_sub(1 as ::core::ffi::c_ulong) as xkb_mod_mask_t);
        (*mods).mask = mod_mask_get_effective(keymap, (*mods).mods) | (*mods).mods & unknown_mods;
    }
}
unsafe extern "C" fn UpdateActionMods(
    mut keymap: *mut xkb_keymap,
    mut act: *mut xkb_action,
    mut modmap: xkb_mod_mask_t,
) {
    unsafe {
        match (*act).type_0 as ::core::ffi::c_uint {
            2 | 3 | 4 => {
                if (*act).mods.flags as ::core::ffi::c_uint
                    & ACTION_MODS_LOOKUP_MODMAP as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
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
unsafe extern "C" fn FindInterpForKey(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut group: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut interprets: *mut xkb_sym_interprets,
) -> bool {
    unsafe {
        let mut syms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let mut num_syms: ::core::ffi::c_int = 0;
        num_syms =
            xkb_keymap_key_get_syms_by_level(keymap, (*key).keycode, group, level, &raw mut syms);
        if num_syms <= 0 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while s < num_syms {
            let mut c2rust_current_block_34: u64;
            let mut found: bool = false_0 != 0;
            let mut i: darray_size_t = 0 as darray_size_t;
            's_26: loop {
                if !(i < (*keymap).num_sym_interprets) {
                    c2rust_current_block_34 = 7659304154607701039;
                    break;
                }
                let interp: *mut xkb_sym_interpret =
                    (*keymap).sym_interprets.offset(i as isize) as *mut xkb_sym_interpret;
                let mut mods: xkb_mod_mask_t = 0;
                found = false_0 != 0;
                if !((*interp).sym != *syms.offset(s as isize)
                    && (*interp).sym != XKB_KEY_NoSymbol as xkb_keysym_t)
                {
                    if (*interp).level_one_only as ::core::ffi::c_int != 0
                        && level != 0 as xkb_level_index_t
                    {
                        mods = 0 as xkb_mod_mask_t;
                    } else {
                        mods = (*key).modmap;
                    }
                    match (*interp).match_0 as ::core::ffi::c_uint {
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
                    if found as ::core::ffi::c_int != 0
                        && i > 0 as darray_size_t
                        && (*interp).sym == XKB_KEY_NoSymbol as xkb_keysym_t
                    {
                        let mut previous_interp: *mut *const xkb_sym_interpret =
                            ::core::ptr::null_mut::<*const xkb_sym_interpret>();
                        if !(*interprets).item.is_null() {
                            previous_interp =
                                (*interprets).item.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut *const xkb_sym_interpret;
                            while previous_interp
                                < (*interprets).item.offset((*interprets).size as isize)
                                    as *mut *const xkb_sym_interpret
                            {
                                if *previous_interp == interp as *const xkb_sym_interpret {
                                    found = false_0 != 0;
                                    xkb_log(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_WARNING,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        b"Repeated interpretation ignored for keysym #%d \"%s\" at level %u/group %u on key %s.\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        s + 1 as ::core::ffi::c_int,
                                        KeysymText((*keymap).ctx, *syms.offset(s as isize)),
                                        level.wrapping_add(1 as xkb_level_index_t),
                                        group.wrapping_add(1 as xkb_layout_index_t),
                                        KeyNameText((*keymap).ctx, (*key).name),
                                    );
                                    c2rust_current_block_34 = 2209838995503123840;
                                    break 's_26;
                                } else {
                                    previous_interp = previous_interp.offset(1);
                                }
                            }
                        }
                    }
                    if found {
                        (*interprets).size = (*interprets).size.wrapping_add(1 as darray_size_t);
                        let mut __need: darray_size_t = (*interprets).size;
                        if __need > (*interprets).alloc {
                            (*interprets).alloc = darray_next_alloc(
                                (*interprets).alloc,
                                __need,
                                ::core::mem::size_of::<*const xkb_sym_interpret>() as usize,
                            );
                            (*interprets).item = realloc(
                                (*interprets).item as *mut ::core::ffi::c_void,
                                ((*interprets).alloc as usize).wrapping_mul(
                                    ::core::mem::size_of::<*const xkb_sym_interpret>() as usize,
                                ),
                            )
                                as *mut *const xkb_sym_interpret;
                        }
                        let ref mut c2rust_fresh2 = *(*interprets)
                            .item
                            .offset((*interprets).size.wrapping_sub(1 as darray_size_t) as isize);
                        *c2rust_fresh2 = interp;
                        (*interp).set_required((true_0 != 0) as bool);
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
                    (*interprets).size = (*interprets).size.wrapping_add(1 as darray_size_t);
                    let mut __need_0: darray_size_t = (*interprets).size;
                    if __need_0 > (*interprets).alloc {
                        (*interprets).alloc = darray_next_alloc(
                            (*interprets).alloc,
                            __need_0,
                            ::core::mem::size_of::<*const xkb_sym_interpret>() as usize,
                        );
                        (*interprets).item =
                            realloc(
                                (*interprets).item as *mut ::core::ffi::c_void,
                                ((*interprets).alloc as usize).wrapping_mul(
                                    ::core::mem::size_of::<*const xkb_sym_interpret>() as usize,
                                ),
                            ) as *mut *const xkb_sym_interpret;
                    }
                    let ref mut c2rust_fresh3 = *(*interprets)
                        .item
                        .offset((*interprets).size.wrapping_sub(1 as darray_size_t) as isize);
                    *c2rust_fresh3 = &raw const default_interpret;
                }
                _ => {}
            }
            s += 1;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn ApplyInterpsToKey(mut keymap: *mut xkb_keymap, mut key: *mut xkb_key) -> bool {
    unsafe {
        let mut vmodmap: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut level: xkb_level_index_t = 0;
        let mut interprets: xkb_sym_interprets = xkb_sym_interprets {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<*const xkb_sym_interpret>(),
        };
        let mut actions: C2Rust_Unnamed_19 = C2Rust_Unnamed_19 {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<xkb_action>(),
        };
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while group < (*key).num_groups() {
            if !(*(*key).groups.offset(group as isize)).explicit_actions() {
                level = 0 as xkb_level_index_t;
                while level < XkbKeyNumLevels(key, group) {
                    if (*(*(*key).groups.offset(group as isize))
                        .levels
                        .offset(level as isize))
                    .num_actions as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"key->groups[group].levels[level].num_actions == 0\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../src/xkbcomp/keymap.c\0".as_ptr() as *const ::core::ffi::c_char,
                            191 as ::core::ffi::c_uint,
                            b"_Bool ApplyInterpsToKey(struct xkb_keymap *, struct xkb_key *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                    let mut interp_iter: *mut *const xkb_sym_interpret =
                        ::core::ptr::null_mut::<*const xkb_sym_interpret>();
                    let mut interp: *const xkb_sym_interpret =
                        ::core::ptr::null::<xkb_sym_interpret>();
                    let mut k: usize = 0;
                    interprets.size = 0 as darray_size_t;
                    let mut __need: darray_size_t = interprets.size;
                    if __need > interprets.alloc {
                        interprets.alloc = darray_next_alloc(
                            interprets.alloc,
                            __need,
                            ::core::mem::size_of::<*const xkb_sym_interpret>() as usize,
                        );
                        interprets.item = realloc(
                            interprets.item as *mut ::core::ffi::c_void,
                            (interprets.alloc as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*const xkb_sym_interpret>() as usize
                                ),
                        )
                            as *mut *const xkb_sym_interpret;
                    }
                    let found: bool =
                        FindInterpForKey(keymap, key, group, level, &raw mut interprets) as bool;
                    if found {
                        if !interprets.item.is_null() {
                            k = 0 as usize;
                            interp_iter = interprets.item.offset(0 as ::core::ffi::c_int as isize)
                                as *mut *const xkb_sym_interpret;
                            while k < interprets.size as usize {
                                interp = *interp_iter;
                                if group == 0 as xkb_layout_index_t
                                    && level == 0 as xkb_level_index_t
                                {
                                    if (*key).explicit as ::core::ffi::c_uint
                                        & EXPLICIT_REPEAT as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        == 0
                                        && (*interp).repeat() as ::core::ffi::c_int != 0
                                    {
                                        (*key).set_repeats((true_0 != 0) as bool);
                                    }
                                }
                                if group == 0 as xkb_layout_index_t
                                    && level == 0 as xkb_level_index_t
                                    || !(*interp).level_one_only
                                {
                                    if (*interp).virtual_mod != XKB_MOD_INVALID as xkb_mod_index_t {
                                        vmodmap = (vmodmap as ::core::ffi::c_uint
                                            | (1 as ::core::ffi::c_uint) << (*interp).virtual_mod)
                                            as xkb_mod_mask_t;
                                    }
                                }
                                match (*interp).num_actions as ::core::ffi::c_int {
                                    0 => {}
                                    1 => {
                                        actions.size =
                                            actions.size.wrapping_add(1 as darray_size_t);
                                        let mut __need_0: darray_size_t = actions.size;
                                        if __need_0 > actions.alloc {
                                            actions.alloc = darray_next_alloc(
                                                actions.alloc,
                                                __need_0,
                                                ::core::mem::size_of::<xkb_action>() as usize,
                                            );
                                            actions.item = realloc(
                                                actions.item as *mut ::core::ffi::c_void,
                                                (actions.alloc as usize).wrapping_mul(
                                                    ::core::mem::size_of::<xkb_action>() as usize,
                                                ),
                                            )
                                                as *mut xkb_action;
                                        }
                                        *actions.item.offset(
                                            actions.size.wrapping_sub(1 as darray_size_t) as isize,
                                        ) = (*interp).a.action;
                                    }
                                    _ => {
                                        let mut __count: darray_size_t =
                                            (*interp).num_actions as darray_size_t;
                                        let mut __oldSize: darray_size_t = actions.size;
                                        actions.size = __oldSize.wrapping_add(__count);
                                        let mut __need_1: darray_size_t = actions.size;
                                        if __need_1 > actions.alloc {
                                            actions.alloc = darray_next_alloc(
                                                actions.alloc,
                                                __need_1,
                                                ::core::mem::size_of::<xkb_action>() as usize,
                                            );
                                            actions.item = realloc(
                                                actions.item as *mut ::core::ffi::c_void,
                                                (actions.alloc as usize).wrapping_mul(
                                                    ::core::mem::size_of::<xkb_action>() as usize,
                                                ),
                                            )
                                                as *mut xkb_action;
                                        }
                                        memcpy(
                                            actions.item.offset(__oldSize as isize)
                                                as *mut ::core::ffi::c_void,
                                            (*interp).a.actions as *const ::core::ffi::c_void,
                                            (__count as usize).wrapping_mul(
                                                ::core::mem::size_of::<xkb_action>() as usize,
                                            ),
                                        );
                                    }
                                }
                                k = k.wrapping_add(1);
                                interp_iter = interp_iter.offset(1);
                            }
                        }
                        if (actions.size != 0) as ::core::ffi::c_int as ::core::ffi::c_long
                            > MAX_ACTIONS_PER_LEVEL as ::core::ffi::c_long
                        {
                            xkb_log(
                                (*keymap).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"Could not append interpret actions to key %s: maximum is %u, got: %u. Dropping excessive actions\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                KeyNameText((*keymap).ctx, (*key).name),
                                65535 as ::core::ffi::c_int,
                                actions.size,
                            );
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_actions = MAX_ACTIONS_PER_LEVEL as xkb_action_count_t;
                        } else {
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_actions = actions.size as xkb_action_count_t;
                        }
                        match actions.size {
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
                                .action = *actions.item.offset(0 as ::core::ffi::c_int as isize);
                            }
                            _ => {
                                let ref mut c2rust_fresh0 =
                                    (*(*(*key).groups.offset(group as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                *c2rust_fresh0 = memdup(
                                    actions.item as *const ::core::ffi::c_void,
                                    (*(*(*key).groups.offset(group as isize))
                                        .levels
                                        .offset(level as isize))
                                    .num_actions as usize,
                                    ::core::mem::size_of::<xkb_action>() as usize,
                                )
                                    as *mut xkb_action;
                                if (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .actions
                                .is_null()
                                {
                                    xkb_log(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        b"[XKB-%03d] Could not allocate interpret actions\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
                                    );
                                    free(actions.item as *mut ::core::ffi::c_void);
                                    actions.item = ::core::ptr::null_mut::<xkb_action>();
                                    actions.size = 0 as darray_size_t;
                                    actions.alloc = 0 as darray_size_t;
                                    free(interprets.item as *mut ::core::ffi::c_void);
                                    interprets.item =
                                        ::core::ptr::null_mut::<*const xkb_sym_interpret>();
                                    interprets.size = 0 as darray_size_t;
                                    interprets.alloc = 0 as darray_size_t;
                                    return false_0 != 0;
                                }
                            }
                        }
                        if !(actions.size == 0 as darray_size_t) {
                            let ref mut c2rust_fresh1 = *(*key).groups.offset(group as isize);
                            (*c2rust_fresh1).set_implicit_actions((true_0 != 0) as bool);
                        }
                        actions.size = 0 as darray_size_t;
                        let mut __need_2: darray_size_t = actions.size;
                        if __need_2 > actions.alloc {
                            actions.alloc = darray_next_alloc(
                                actions.alloc,
                                __need_2,
                                ::core::mem::size_of::<xkb_action>() as usize,
                            );
                            actions.item = realloc(
                                actions.item as *mut ::core::ffi::c_void,
                                (actions.alloc as usize).wrapping_mul(::core::mem::size_of::<
                                    xkb_action,
                                >(
                                )
                                    as usize),
                            ) as *mut xkb_action;
                        }
                    }
                    level = level.wrapping_add(1);
                }
                if (*(*key).groups.offset(group as isize)).implicit_actions() {
                    (*key).set_implicit_actions((true_0 != 0) as bool);
                }
            }
            group = group.wrapping_add(1);
        }
        free(actions.item as *mut ::core::ffi::c_void);
        actions.item = ::core::ptr::null_mut::<xkb_action>();
        actions.size = 0 as darray_size_t;
        actions.alloc = 0 as darray_size_t;
        free(interprets.item as *mut ::core::ffi::c_void);
        interprets.item = ::core::ptr::null_mut::<*const xkb_sym_interpret>();
        interprets.size = 0 as darray_size_t;
        interprets.alloc = 0 as darray_size_t;
        if (*key).explicit as ::core::ffi::c_uint
            & EXPLICIT_VMODMAP as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            (*key).vmodmap = vmodmap;
        }
        return true_0 != 0;
    }
}
#[inline]
unsafe extern "C" fn is_mod_action(mut action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as ::core::ffi::c_uint
            == ACTION_TYPE_MOD_SET as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*action).type_0 as ::core::ffi::c_uint
                == ACTION_TYPE_MOD_LATCH as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*action).type_0 as ::core::ffi::c_uint
                == ACTION_TYPE_MOD_LOCK as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
}
#[inline]
unsafe extern "C" fn is_group_action(mut action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as ::core::ffi::c_uint
            == ACTION_TYPE_GROUP_SET as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*action).type_0 as ::core::ffi::c_uint
                == ACTION_TYPE_GROUP_LATCH as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*action).type_0 as ::core::ffi::c_uint
                == ACTION_TYPE_GROUP_LOCK as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn CheckMultipleActionsCategories(
    mut keymap: *mut xkb_keymap,
    mut key: *mut xkb_key,
) {
    unsafe {
        let mut g: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while g < (*key).num_groups() {
            let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
            while l < XkbKeyNumLevels(key, g) {
                let mut level: *mut xkb_level = (*(*key).groups.offset(g as isize))
                    .levels
                    .offset(l as isize)
                    as *mut xkb_level;
                if !((*level).num_actions as ::core::ffi::c_int <= 1 as ::core::ffi::c_int) {
                    let mut i: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (i as ::core::ffi::c_int) < (*level).num_actions as ::core::ffi::c_int {
                        let mut action1: *mut xkb_action =
                            (*level).a.actions.offset(i as isize) as *mut xkb_action;
                        let mut mod_action: bool = is_mod_action(action1);
                        let mut group_action: bool = is_group_action(action1);
                        if mod_action as ::core::ffi::c_int != 0
                            || group_action as ::core::ffi::c_int != 0
                            || (*action1).type_0 as ::core::ffi::c_uint
                                == ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                        {
                            let mut j: xkb_action_count_t = (i as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as xkb_action_count_t;
                            while (j as ::core::ffi::c_int)
                                < (*level).num_actions as ::core::ffi::c_int
                            {
                                let mut action2: *mut xkb_action =
                                    (*level).a.actions.offset(j as isize) as *mut xkb_action;
                                if (*action1).type_0 as ::core::ffi::c_uint
                                    == (*action2).type_0 as ::core::ffi::c_uint
                                    || mod_action as ::core::ffi::c_int != 0
                                        && is_mod_action(action2) as ::core::ffi::c_int != 0
                                    || group_action as ::core::ffi::c_int != 0
                                        && is_group_action(action2) as ::core::ffi::c_int != 0
                                {
                                    let type_0: *const ::core::ffi::c_char =
                                        if mod_action as ::core::ffi::c_int != 0 {
                                            b"modifiers\0".as_ptr() as *const ::core::ffi::c_char
                                        } else if group_action as ::core::ffi::c_int != 0 {
                                            b"group\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            ActionTypeText((*action1).type_0)
                                                as *const ::core::ffi::c_char
                                        };
                                    xkb_log(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        b"Cannot use multiple %s actions in the same level. Action #%u for key %s in group %u/level %u ignored.\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        type_0,
                                        j as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                        KeyNameText((*keymap).ctx, (*key).name),
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
unsafe extern "C" fn add_key_aliases(
    mut keymap: *mut xkb_keymap,
    mut min: darray_size_t,
    mut max: darray_size_t,
    mut aliases: *mut xkb_key_alias,
) {
    unsafe {
        let mut alias: darray_size_t = min;
        while alias <= max {
            let entry: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(alias as isize);
            if entry.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
                && entry.c2rust_unnamed.found() as ::core::ffi::c_int != 0
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
unsafe extern "C" fn update_pending_key_fields(
    mut info: *mut xkb_keymap_info,
    mut key: *mut xkb_key,
) -> bool {
    unsafe {
        if (*key).out_of_range_pending_group() {
            let pc: *mut pending_computation = (*(*info).pending_computations)
                .item
                .offset((*key).out_of_range_group_number() as isize)
                as *mut pending_computation;
            if !(*pc).computed {
                let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                match ExprResolveGroup(
                    info,
                    (*pc).expr,
                    true_0 != 0,
                    &raw mut group,
                    ::core::ptr::null_mut::<bool>(),
                ) as ::core::ffi::c_uint
                {
                    0 => {
                        (*pc).computed = true_0 != 0;
                        (*pc).value = group.wrapping_sub(1 as xkb_layout_index_t) as u32;
                    }
                    2 => {
                        xkb_log(
                            (*info).keymap.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Invalid key redirect group index\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                        );
                        return (*info).strict as ::core::ffi::c_uint
                            & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            != 0;
                    }
                    _ => {}
                }
            }
            (*key).set_out_of_range_pending_group((false_0 != 0) as bool);
            (*key).set_out_of_range_group_number(
                (*pc).value as xkb_layout_index_t as xkb_layout_index_t,
            );
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn update_pending_action_fields(
    mut info: *mut xkb_keymap_info,
    mut keycode: xkb_keycode_t,
    mut act: *mut xkb_action,
) -> bool {
    unsafe {
        match (*act).type_0 as ::core::ffi::c_uint {
            5 | 6 | 7 => {
                if (*act).group.flags as ::core::ffi::c_uint
                    & ACTION_PENDING_COMPUTATION as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    let pc: *mut pending_computation = (*(*info).pending_computations)
                        .item
                        .offset((*act).group.group as isize)
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                        let absolute: bool = (*act).group.flags as ::core::ffi::c_uint
                            & ACTION_ABSOLUTE_SWITCH as ::core::ffi::c_int as ::core::ffi::c_uint
                            != 0;
                        match ExprResolveGroup(
                            info,
                            (*pc).expr,
                            absolute,
                            &raw mut group,
                            ::core::ptr::null_mut::<bool>(),
                        ) as ::core::ffi::c_uint
                        {
                            2 => {
                                xkb_log(
                                    (*info).keymap.ctx,
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                    b"[XKB-%03d] Invalid action group index\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                                );
                                return false_0 != 0;
                            }
                            1 => {}
                            _ => {
                                (*pc).computed = true_0 != 0;
                                if absolute {
                                    (*pc).value = group.wrapping_sub(1 as xkb_layout_index_t)
                                        as int32_t
                                        as u32;
                                } else {
                                    (*pc).value = group as u32;
                                    if (*(*pc).expr).common.type_0 as ::core::ffi::c_uint
                                        == STMT_EXPR_NEGATE as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        (*pc).value = -((*pc).value as int32_t) as u32;
                                    }
                                }
                            }
                        }
                    }
                    (*act).group.group = (*pc).value as int32_t;
                    (*act).group.flags = ((*act).group.flags as ::core::ffi::c_uint
                        & !(ACTION_PENDING_COMPUTATION as ::core::ffi::c_int)
                            as ::core::ffi::c_uint)
                        as xkb_action_flags;
                }
                return true_0 != 0;
            }
            16 => {
                if keycode == XKB_KEYCODE_INVALID as xkb_keycode_t
                    || (*act).redirect.keycode != (*info).keymap.redirect_key_auto
                {
                    return true_0 != 0;
                } else {
                    (*act).redirect.keycode = keycode;
                }
                return true_0 != 0;
            }
            _ => return true_0 != 0,
        };
    }
}
unsafe extern "C" fn update_pending_led_fields(
    mut info: *mut xkb_keymap_info,
    mut led: *mut xkb_led,
) -> bool {
    unsafe {
        if (*led).pending_groups() {
            let pc: *mut pending_computation = (*(*info).pending_computations)
                .item
                .offset((*led).groups as isize)
                as *mut pending_computation;
            if !(*pc).computed {
                let mut mask: xkb_layout_mask_t = 0 as xkb_layout_mask_t;
                if !ExprResolveGroupMask(
                    info,
                    (*pc).expr,
                    &raw mut mask,
                    ::core::ptr::null_mut::<bool>(),
                ) {
                    xkb_log(
                        (*info).keymap.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Invalid LED group mask\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                    );
                    return false_0 != 0;
                }
                (*pc).computed = true_0 != 0;
                (*pc).value = mask as u32;
            }
            (*led).set_pending_groups((false_0 != 0) as bool);
            (*led).groups = (*pc).value as xkb_layout_mask_t;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn UpdateDerivedKeymapFields(mut info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let keymap: *mut xkb_keymap = &raw mut (*info).keymap;
        let mut num_key_aliases: darray_size_t = 0 as darray_size_t;
        let mut min_alias: darray_size_t = 0 as darray_size_t;
        let mut max_alias: darray_size_t = 0 as darray_size_t;
        let mut alias: xkb_atom_t = 0 as xkb_atom_t;
        while alias < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
            let entry: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(alias as isize);
            if entry.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
                && entry.c2rust_unnamed.found() as ::core::ffi::c_int != 0
            {
                if num_key_aliases == 0 {
                    min_alias = alias as darray_size_t;
                }
                max_alias = alias as darray_size_t;
                num_key_aliases = num_key_aliases.wrapping_add(1);
            }
            alias = alias.wrapping_add(1);
        }
        if num_key_aliases != 0 {
            let required_space: darray_size_t = (::core::mem::size_of::<xkb_key_alias>() as usize)
                .wrapping_div(::core::mem::size_of::<KeycodeMatch>() as usize)
                .wrapping_mul(num_key_aliases as usize)
                as darray_size_t;
            if num_key_aliases <= (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
            } else {
                __assert_fail(
                    b"num_key_aliases <= keymap->num_key_names\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keymap.c\0".as_ptr() as *const ::core::ffi::c_char,
                    528 as ::core::ffi::c_uint,
                    b"_Bool UpdateDerivedKeymapFields(struct xkb_keymap_info *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
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
                    (num_key_aliases as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_key_alias>() as usize),
                ) as *mut xkb_key_alias;
                if r.is_null() {
                    return false_0 != 0;
                }
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = r;
            } else if (*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .num_key_names
                .wrapping_sub(max_alias)
                .wrapping_sub(1 as darray_size_t)
                > required_space
            {
                let aliases: *mut xkb_key_alias = (*keymap)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .key_names
                    .offset(max_alias as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset(!is_aligned(
                        (*keymap)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .key_names
                            .offset(max_alias as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<xkb_key_alias>() as usize,
                    ) as ::core::ffi::c_int as isize)
                    as *mut xkb_key_alias;
                add_key_aliases(keymap, min_alias, max_alias, aliases);
                memcpy(
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases
                        as *mut ::core::ffi::c_void,
                    aliases as *const ::core::ffi::c_void,
                    (num_key_aliases as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_key_alias>() as usize),
                );
                let r_0: *mut xkb_key_alias = realloc(
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases
                        as *mut ::core::ffi::c_void,
                    (num_key_aliases as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_key_alias>() as usize),
                ) as *mut xkb_key_alias;
                if r_0.is_null() {
                    return false_0 != 0;
                }
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = r_0;
            } else {
                let aliases_0: *mut xkb_key_alias = calloc(
                    num_key_aliases as usize,
                    ::core::mem::size_of::<xkb_key_alias>() as usize,
                ) as *mut xkb_key_alias;
                if aliases_0.is_null() {
                    return false_0 != 0;
                }
                add_key_aliases(keymap, min_alias, max_alias, aliases_0);
                free((*keymap).c2rust_unnamed.c2rust_unnamed.key_names as *mut ::core::ffi::c_void);
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = aliases_0;
            }
        }
        (*keymap).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases = num_key_aliases;
        let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
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
        let pending_computations: bool =
            !((*(*info).pending_computations).size == 0 as darray_size_t);
        if pending_computations {
            let num_groups: xkb_layout_index_t = if (*keymap).num_groups != 0 {
                (*keymap).num_groups
            } else {
                1 as xkb_layout_index_t
            };
            (*info).lookup.groupIndexNames[GROUP_INDEX_NAME_LAST as ::core::ffi::c_int as usize] =
                LookupEntry {
                    name: GROUP_LAST_INDEX_NAME.as_ptr(),
                    value: num_groups as u32,
                };
            (*info).lookup.groupMaskNames[GROUP_MASK_NAME_LAST as ::core::ffi::c_int as usize] =
                LookupEntry {
                    name: GROUP_LAST_INDEX_NAME.as_ptr(),
                    value: (1 as u32) << num_groups.wrapping_sub(1 as xkb_layout_index_t),
                };
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < (*keymap).num_sym_interprets {
                let interp: *mut xkb_sym_interpret =
                    (*keymap).sym_interprets.offset(i as isize) as *mut xkb_sym_interpret;
                if (*interp).num_actions as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
                    let act: *mut xkb_action = &raw mut (*interp).a.action;
                    if !update_pending_action_fields(
                        info,
                        XKB_KEYCODE_INVALID as xkb_keycode_t,
                        act,
                    ) {
                        return false_0 != 0;
                    }
                } else {
                    let mut a: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (a as ::core::ffi::c_int) < (*interp).num_actions as ::core::ffi::c_int {
                        let act_0: *mut xkb_action =
                            (*interp).a.actions.offset(a as isize) as *mut xkb_action;
                        if !update_pending_action_fields(
                            info,
                            XKB_KEYCODE_INVALID as xkb_keycode_t,
                            act_0,
                        ) {
                            return false_0 != 0;
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
                return false_0 != 0;
            }
            CheckMultipleActionsCategories(keymap, key);
            key = key.offset(1);
        }
        let mut idx: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
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
        if (*keymap).format as ::core::ffi::c_uint
            >= XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
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
        if (*keymap).canonical_state_mask == 0xff as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"keymap->canonical_state_mask == MOD_REAL_MASK_ALL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../src/xkbcomp/keymap.c\0".as_ptr() as *const ::core::ffi::c_char,
                664 as ::core::ffi::c_uint,
                b"_Bool UpdateDerivedKeymapFields(struct xkb_keymap_info *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut extra_canonical_mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        idx = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
            as *mut xkb_mod;
        while idx < (*keymap).mods.num_mods {
            extra_canonical_mods |= (*mod_0).mapping;
            idx = idx.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*keymap).canonical_state_mask |= extra_canonical_mods;
        let mut i_0: darray_size_t = 0 as darray_size_t;
        while i_0 < (*keymap).num_types {
            ComputeEffectiveMask(
                keymap,
                &raw mut (*(*keymap).types.offset(i_0 as isize)).mods,
            );
            let mut j: darray_size_t = 0 as darray_size_t;
            while j < (*(*keymap).types.offset(i_0 as isize)).num_entries {
                if has_unbound_vmods(
                    keymap,
                    (*(*(*keymap).types.offset(i_0 as isize))
                        .entries
                        .offset(j as isize))
                    .mods
                    .mods,
                ) {
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
                return false_0 != 0;
            }
            let mut i_1: xkb_layout_index_t = 0 as xkb_layout_index_t;
            while i_1 < (*key).num_groups() {
                let mut j_0: xkb_level_index_t = 0 as xkb_level_index_t;
                while j_0 < XkbKeyNumLevels(key, i_1) {
                    if (*(*(*key).groups.offset(i_1 as isize))
                        .levels
                        .offset(j_0 as isize))
                    .num_actions as ::core::ffi::c_int
                        <= 1 as ::core::ffi::c_int
                    {
                        let act_1: *mut xkb_action =
                            &raw mut (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .a
                            .action;
                        UpdateActionMods(keymap, act_1, (*key).modmap);
                        if (pending_computations as ::core::ffi::c_int != 0
                            || (*act_1).type_0 as ::core::ffi::c_uint
                                == ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int
                                    as ::core::ffi::c_uint)
                            && !update_pending_action_fields(info, (*key).keycode, act_1)
                        {
                            return false_0 != 0;
                        }
                    } else {
                        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
                        while (k as ::core::ffi::c_int)
                            < (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .num_actions as ::core::ffi::c_int
                        {
                            let act_2: *mut xkb_action = (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .a
                            .actions
                            .offset(k as isize)
                                as *mut xkb_action;
                            UpdateActionMods(keymap, act_2, (*key).modmap);
                            if (pending_computations as ::core::ffi::c_int != 0
                                || (*act_2).type_0 as ::core::ffi::c_uint
                                    == ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int
                                        as ::core::ffi::c_uint)
                                && !update_pending_action_fields(info, (*key).keycode, act_2)
                            {
                                return false_0 != 0;
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
        let mut led: *mut xkb_led = ::core::ptr::null_mut::<xkb_led>();
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while led < (&raw mut (*keymap).leds as *mut xkb_led).offset((*keymap).num_leds as isize) {
            ComputeEffectiveMask(keymap, &raw mut (*led).mods);
            if pending_computations as ::core::ffi::c_int != 0
                && !update_pending_led_fields(info, led)
            {
                return false_0 != 0;
            }
            led = led.offset(1);
        }
        return true_0 != 0;
    }
}
static mut compile_file_fns: [compile_file_fn; 4] = unsafe {
    [
        Some(CompileKeycodes as unsafe extern "C" fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileKeyTypes as unsafe extern "C" fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileCompatMap as unsafe extern "C" fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileSymbols as unsafe extern "C" fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
    ]
};
unsafe extern "C" fn pending_computations_array_free(mut p: *mut pending_computation_array) {
    unsafe {
        let mut pc: *mut pending_computation = ::core::ptr::null_mut::<pending_computation>();
        if !(*p).item.is_null() {
            pc = (*p).item.offset(0 as ::core::ffi::c_int as isize) as *mut pending_computation;
            while pc < (*p).item.offset((*p).size as isize) as *mut pending_computation {
                FreeStmt((*pc).expr as *mut ParseCommon);
                pc = pc.offset(1);
            }
        }
        free((*p).item as *mut ::core::ffi::c_void);
        (*p).item = ::core::ptr::null_mut::<pending_computation>();
        (*p).size = 0 as darray_size_t;
        (*p).alloc = 0 as darray_size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompileKeymap(
    mut file: *mut XkbFile,
    mut keymap: *mut xkb_keymap,
) -> bool {
    unsafe {
        let mut files: [*mut XkbFile; 4] = [
            ::core::ptr::null_mut::<XkbFile>(),
            ::core::ptr::null_mut::<XkbFile>(),
            ::core::ptr::null_mut::<XkbFile>(),
            ::core::ptr::null_mut::<XkbFile>(),
        ];
        let mut type_0: xkb_file_type = FILE_TYPE_KEYCODES;
        let mut ctx: *mut xkb_context = (*keymap).ctx;
        file = (*file).defs as *mut XkbFile;
        while !file.is_null() {
            if ((*file).file_type as ::core::ffi::c_uint)
                < FIRST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*file).file_type as ::core::ffi::c_uint
                    > LAST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*file).file_type as ::core::ffi::c_uint
                    == FILE_TYPE_GEOMETRY as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                        b"[XKB-%03d] Geometry sections are not supported; ignoring\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION as ::core::ffi::c_int,
                    );
                } else {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Cannot define %s in a keymap file\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        xkb_file_type_to_string((*file).file_type),
                    );
                }
            } else if !files[(*file).file_type as usize].is_null() {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"More than one %s section in keymap file; All sections after the first ignored\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    xkb_file_type_to_string((*file).file_type),
                );
            } else {
                files[(*file).file_type as usize] = file;
            }
            file = (*file).common.next as *mut XkbFile;
        }
        let mut pending_computations: pending_computation_array = pending_computation_array {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<pending_computation>(),
        };
        let mut info: xkb_keymap_info = xkb_keymap_info {
            keymap: *keymap,
            strict: (if (*keymap).format as ::core::ffi::c_uint
                == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*keymap).flags as ::core::ffi::c_uint
                    & XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    PARSER_V1_STRICT_FLAGS as ::core::ffi::c_int
                } else {
                    PARSER_V1_LAX_FLAGS as ::core::ffi::c_int
                }
            } else if (*keymap).flags as ::core::ffi::c_uint
                & XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                PARSER_V2_STRICT_FLAGS as ::core::ffi::c_int
            } else {
                PARSER_V2_LAX_FLAGS as ::core::ffi::c_int
            }) as xkb_parser_strict_flags,
            features: C2Rust_Unnamed_18 {
                max_groups: format_max_groups((*keymap).format),
                max_overlays: format_max_overlays((*keymap).format),
                controls_name_offset: format_control_names_offset((*keymap).format),
                group_lock_on_release: isGroupLockOnReleaseSupported((*keymap).format),
                mods_unlock_on_press: isModsUnLockOnPressSupported((*keymap).format),
                mods_latch_on_press: isModsLatchOnPressSupported((*keymap).format),
                overlapping_overlays: areOverlappingOverlaysSupported((*keymap).format),
            },
            lookup: C2Rust_Unnamed_17 {
                groupIndexNames: [
                    LookupEntry {
                        name: b"first\0".as_ptr() as *const ::core::ffi::c_char,
                        value: 1 as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME.as_ptr()
                        } else {
                            ::core::ptr::null::<::core::ffi::c_char>()
                        },
                        value: (*keymap).num_groups as u32,
                    },
                    LookupEntry {
                        name: ::core::ptr::null::<::core::ffi::c_char>(),
                        value: 0 as u32,
                    },
                ],
                groupMaskNames: [
                    LookupEntry {
                        name: b"none\0".as_ptr() as *const ::core::ffi::c_char,
                        value: 0 as u32,
                    },
                    LookupEntry {
                        name: b"first\0".as_ptr() as *const ::core::ffi::c_char,
                        value: 0x1 as u32,
                    },
                    LookupEntry {
                        name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
                        value: XKB_ALL_GROUPS as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME.as_ptr()
                        } else {
                            ::core::ptr::null::<::core::ffi::c_char>()
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
                        name: ::core::ptr::null::<::core::ffi::c_char>(),
                        value: 0 as u32,
                    },
                ],
            },
            pending_computations: &raw mut pending_computations,
        };
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        while type_0 as ::core::ffi::c_uint
            <= LAST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if files[type_0 as usize].is_null() {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Component %s not provided in keymap\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    xkb_file_type_to_string(type_0),
                );
            } else {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Compiling %s \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                    xkb_file_type_to_string(type_0),
                    safe_map_name(files[type_0 as usize]),
                );
            }
            let ok: bool = compile_file_fns[type_0 as usize].expect("non-null function pointer")(
                files[type_0 as usize],
                &raw mut info,
            ) as bool;
            if !ok {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Failed to compile %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    xkb_file_type_to_string(type_0),
                );
                *keymap = info.keymap;
                pending_computations_array_free(&raw mut pending_computations);
                return false_0 != 0;
            }
            type_0 += 1;
        }
        let ok_0: bool = UpdateDerivedKeymapFields(&raw mut info) as bool;
        *keymap = info.keymap;
        pending_computations_array_free(&raw mut pending_computations);
        return ok_0;
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        default_interpret = {
            let mut init = xkb_sym_interpret {
                repeat_required: [0; 1],
                sym: XKB_KEY_NoSymbol as xkb_keysym_t,
                match_0: MATCH_ANY_OR_NONE,
                mods: 0 as xkb_mod_mask_t,
                virtual_mod: DEFAULT_INTERPRET_VMOD as ::core::ffi::c_uint as xkb_mod_index_t,
                level_one_only: false,
                num_actions: 0 as xkb_action_count_t,
                a: C2Rust_Unnamed_1 {
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                },
            };
            init.set_repeat(DEFAULT_INTERPRET_KEY_REPEAT as ::core::ffi::c_int != 0);
            init.set_required(false);
            init
        }
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
