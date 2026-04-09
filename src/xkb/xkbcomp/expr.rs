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
    pub type intmax_t = ::libc::intmax_t;
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
    pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
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
                *const i8,
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
        pub text_buffer: [i8; 2048],
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
        pub item: *mut *mut i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut i8,
    }

    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t) -> *const i8;
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const i8,
            ...
        );
    }
}
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    pub const XKB_ATOM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
        pub rules: *const i8,
        pub model: *const i8,
        pub layout: *const i8,
        pub variant: *const i8,
        pub options: *const i8,
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
    pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    use super::stdint_uintn_h::u32;
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
        pub keycodes_section_name: *mut i8,
        pub symbols_section_name: *mut i8,
        pub types_section_name: *mut i8,
        pub compat_section_name: *mut i8,
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
    pub type xkb_overlay_index_t = uint8_t;
    pub const MOD_REAL_MASK_ALL: xkb_mod_mask_t = 0xff as ::core::ffi::c_int as xkb_mod_mask_t;
    pub const XKB_LEVEL_MAX_IMPL: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    use super::xkbcommon_h::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_level_index_t, xkb_mod_index_t, xkb_mod_mask_t, xkb_state_component,
    };
    extern "C" {
        pub fn XkbModNameToIndex(
            mods: *const xkb_mod_set,
            name: xkb_atom_t,
            type_0: mod_type,
        ) -> xkb_mod_index_t;
    }
}
pub mod ast_h {
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
    use super::atom_h::xkb_atom_t;
    use super::darray_h::darray_size_t;
    use super::stdint_intn_h::int64_t;
    use super::xkbcommon_h::xkb_keysym_t;
    extern "C" {
        pub fn stmt_type_to_string(type_0: stmt_type) -> *const i8;
        pub fn stmt_type_to_operator_char(type_0: stmt_type) -> i8;
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
        pub name: *const i8,
        pub value: u32,
    }
    pub const GROUP_LAST_INDEX_NAME: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"last\0") };
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub static buttonNames: [LookupEntry; 0];
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
        pub features: C2Rust_Unnamed_15,
        pub lookup: C2Rust_Unnamed_14,
        pub pending_computations: *mut pending_computation_array,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_14 {
        pub groupIndexNames: [LookupEntry; 3],
        pub groupMaskNames: [LookupEntry; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_15 {
        pub max_groups: xkb_layout_index_t,
        pub max_overlays: xkb_overlay_index_t,
        pub controls_name_offset: uint8_t,
        pub group_lock_on_release: bool,
        pub mods_unlock_on_press: bool,
        pub mods_latch_on_press: bool,
        pub overlapping_overlays: bool,
    }
    use super::ast_h::ExprDef;
    use super::darray_h::darray_size_t;
    use super::keymap_h::{xkb_keymap, xkb_overlay_index_t};
    use super::stdint_uintn_h::{u32, uint8_t};
    use super::text_h::LookupEntry;
    use super::xkbcommon_h::xkb_layout_index_t;
}
pub mod inttypes_h {
    use super::stdint_h::intmax_t;
    extern "C" {
        pub fn imaxabs(__n: intmax_t) -> intmax_t;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn istreq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn istrneq(mut s1: *const i8, mut s2: *const i8, mut len: usize) -> bool {
        unsafe {
            return istrncmp(s1, s2, len) == 0 as ::core::ffi::c_int;
        }
    }

    extern "C" {
        pub fn istrcmp(a: *const i8, b: *const i8) -> ::core::ffi::c_int;
        pub fn istrncmp(a: *const i8, b: *const i8, n: usize) -> ::core::ffi::c_int;
    }
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe extern "C" fn parse_dec_to_uint32_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut u32,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint)
                    < 10 as ::core::ffi::c_uint
                && result <= (4294967295 as u32).wrapping_div(10 as u32)
                && result.wrapping_mul(10 as u32)
                    <= (4294967295 as u32).wrapping_sub(
                        (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                            as ::core::ffi::c_uchar as u32,
                    )
            {
                result = result.wrapping_mul(10 as u32).wrapping_add(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u32,
                );
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len
                || (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint
                    >= 10 as ::core::ffi::c_uint
            {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }

    use super::stdint_uintn_h::u32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::ast_h::{
    _ParseCommon, stmt_type, stmt_type_to_operator_char, stmt_type_to_string, C2Rust_Unnamed_13,
    ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef,
    ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary,
    ParseCommon, _STMT_NUM_VALUES, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
    STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL,
    STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL,
    STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL,
    STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS,
    STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME,
    STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
use self::inttypes_h::imaxabs;
pub use self::keymap_h::{
    mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, XkbModNameToIndex, _ACTION_TYPE_NUM_ENTRIES,
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
    XKB_LEVEL_MAX_IMPL,
};
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
pub use self::stdint_h::{intmax_t, SIZE_MAX, UINT32_MAX};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
pub use self::text_h::{buttonNames, LookupEntry, GROUP_LAST_INDEX_NAME};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::utils_h::{istrcmp, istreq, istrncmp, istrneq};
pub use self::utils_numbers_h::parse_dec_to_uint32_t;
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t, xkb_layout_index_t,
    xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t,
    xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS,
    XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED,
    XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE,
    XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, xkb_keymap_info, xkb_parser_error,
    xkb_parser_strict_flags, C2Rust_Unnamed_14, C2Rust_Unnamed_15, PARSER_FATAL_ERROR,
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
pub struct LookupModMaskPriv {
    pub mods: *const xkb_mod_set,
    pub mod_type: mod_type,
}
pub type IdentLookupFunc = Option<
    unsafe extern "C" fn(
        *mut xkb_context,
        *const ::core::ffi::c_void,
        xkb_atom_t,
        *mut u32,
        *mut bool,
    ) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_integer_pattern {
    pub prefix: *const i8,
    pub prefix_length: usize,
    pub min: u32,
    pub max: u32,
    pub entries: *const LookupEntry,
    pub pending_entries: *const LookupEntry,
    pub is_mask: bool,
    pub error_id: xkb_message_code,
}
static mut level_name_pattern: named_integer_pattern = named_integer_pattern {
    prefix: ::core::ptr::null::<i8>(),
    prefix_length: 0,
    min: 0,
    max: 0,
    entries: ::core::ptr::null::<LookupEntry>(),
    pending_entries: ::core::ptr::null::<LookupEntry>(),
    is_mask: false,
    error_id: 0 as xkb_message_code,
};
#[no_mangle]
pub unsafe extern "C" fn ExprResolveLhs(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut elem_rtrn: *mut *const i8,
    mut field_rtrn: *mut *const i8,
    mut index_rtrn: *mut *mut ExprDef,
) -> bool {
    unsafe {
        match (*expr).common.type_0 as ::core::ffi::c_uint {
            10 => {
                *elem_rtrn = ::core::ptr::null::<i8>();
                *field_rtrn = xkb_atom_text(ctx, (*expr).ident.ident);
                *index_rtrn = ::core::ptr::null_mut::<ExprDef>();
                return !(*field_rtrn).is_null();
            }
            12 => {
                *elem_rtrn = xkb_atom_text(ctx, (*expr).field_ref.element);
                *field_rtrn = xkb_atom_text(ctx, (*expr).field_ref.field);
                *index_rtrn = ::core::ptr::null_mut::<ExprDef>();
                return !(*elem_rtrn).is_null() && !(*field_rtrn).is_null();
            }
            13 => {
                *elem_rtrn = xkb_atom_text(ctx, (*expr).array_ref.element);
                *field_rtrn = xkb_atom_text(ctx, (*expr).array_ref.field);
                *index_rtrn = (*expr).array_ref.entry as *mut ExprDef;
                if (*expr).array_ref.element != XKB_ATOM_NONE as xkb_atom_t
                    && (*elem_rtrn).is_null()
                {
                    return false_0 != 0;
                }
                if (*field_rtrn).is_null() {
                    return false_0 != 0;
                }
                return true_0 != 0;
            }
            _ => {}
        }
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_CRITICAL,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"[XKB-%03d] Unexpected operator %d in ResolveLhs\n\0".as_ptr() as *const i8,
            XKB_ERROR_INVALID_XKB_SYNTAX as ::core::ffi::c_int,
            (*expr).common.type_0 as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
}
unsafe extern "C" fn SimpleLookup(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut u32,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE as xkb_atom_t {
            return false_0 != 0;
        }
        let mut str: *const i8 = xkb_atom_text(ctx, field);
        let mut entry: *const LookupEntry = priv_0 as *const LookupEntry;
        while !entry.is_null() && !(*entry).name.is_null() {
            if istreq(str, (*entry).name) {
                *val_rtrn = (*entry).value;
                return true_0 != 0;
            }
            entry = entry.offset(1);
        }
        return false_0 != 0;
    }
}
unsafe extern "C" fn NamedIntegerPatternLookup(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut u32,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE as xkb_atom_t {
            return false_0 != 0;
        }
        let str: *const i8 = xkb_atom_text(ctx, field) as *const i8;
        let pattern: *const named_integer_pattern = priv_0 as *const named_integer_pattern;
        let count: ::core::ffi::c_int =
            if istrneq(str, (*pattern).prefix, (*pattern).prefix_length) as ::core::ffi::c_int != 0
            {
                parse_dec_to_uint32_t(
                    str.offset((*pattern).prefix_length as isize),
                    SIZE_MAX as usize,
                    val_rtrn as *mut u32,
                ) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        if count > 0 as ::core::ffi::c_int
            && *str
                .offset((*pattern).prefix_length as isize)
                .offset(count as isize) as ::core::ffi::c_int
                == '\0' as i32
        {
            if *val_rtrn < (*pattern).min || *val_rtrn > (*pattern).max {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s index %u is out of range (%u..%u)\n\0".as_ptr() as *const i8,
                    (*pattern).error_id as ::core::ffi::c_uint,
                    (*pattern).prefix,
                    *val_rtrn,
                    (*pattern).min,
                    (*pattern).max,
                );
                return false_0 != 0;
            }
            if (*pattern).is_mask {
                *val_rtrn =
                    ((1 as ::core::ffi::c_uint) << (*val_rtrn).wrapping_sub((*pattern).min)) as u32;
            }
            return true_0 != 0;
        } else {
            if !(*pattern).entries.is_null()
                && SimpleLookup(
                    ctx,
                    (*pattern).entries as *const ::core::ffi::c_void,
                    field,
                    val_rtrn,
                    ::core::ptr::null_mut::<bool>(),
                ) as ::core::ffi::c_int
                    != 0
            {
                return true_0 != 0;
            }
            if !(*pattern).pending_entries.is_null()
                && !pending_rtrn.is_null()
                && SimpleLookup(
                    ctx,
                    (*pattern).pending_entries as *const ::core::ffi::c_void,
                    field,
                    val_rtrn,
                    ::core::ptr::null_mut::<bool>(),
                ) as ::core::ffi::c_int
                    != 0
            {
                *pending_rtrn = true_0 != 0;
                return true_0 != 0;
            }
            return false_0 != 0;
        };
    }
}
unsafe extern "C" fn LookupModMask(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut xkb_mod_mask_t,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        let mut str: *const i8 = xkb_atom_text(ctx, field);
        if str.is_null() {
            return false_0 != 0;
        }
        if istreq(str, b"all\0".as_ptr() as *const i8) {
            *val_rtrn = MOD_REAL_MASK_ALL;
            return true_0 != 0;
        }
        if istreq(str, b"none\0".as_ptr() as *const i8) {
            *val_rtrn = 0 as xkb_mod_mask_t;
            return true_0 != 0;
        }
        let mut arg: *const LookupModMaskPriv = priv_0 as *const LookupModMaskPriv;
        let mut mods: *const xkb_mod_set = (*arg).mods;
        let mod_type: mod_type = (*arg).mod_type;
        let ndx: xkb_mod_index_t = XkbModNameToIndex(mods, field, mod_type) as xkb_mod_index_t;
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            return false_0 != 0;
        }
        *val_rtrn = ((1 as ::core::ffi::c_uint) << ndx) as xkb_mod_mask_t;
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveBoolean(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut set_rtrn: *mut bool,
) -> bool {
    unsafe {
        let mut ok: bool = false_0 != 0;
        let mut ident: *const i8 = ::core::ptr::null::<i8>();
        match (*expr).common.type_0 as ::core::ffi::c_uint {
            7 => {
                *set_rtrn = (*expr).boolean.set;
                return true_0 != 0;
            }
            4 | 5 | 6 | 8 | 9 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Found %s where boolean was expected\n\0".as_ptr() as *const i8,
                    XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false_0 != 0;
            }
            10 => {
                ident = xkb_atom_text(ctx, (*expr).ident.ident);
                if !ident.is_null() {
                    if istreq(ident, b"true\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                        || istreq(ident, b"yes\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                        || istreq(ident, b"on\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                    {
                        *set_rtrn = true_0 != 0;
                        return true_0 != 0;
                    } else if istreq(ident, b"false\0".as_ptr() as *const i8) as ::core::ffi::c_int
                        != 0
                        || istreq(ident, b"no\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                        || istreq(ident, b"off\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                    {
                        *set_rtrn = false_0 != 0;
                        return true_0 != 0;
                    }
                }
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Identifier \"%s\" of type boolean is unknown\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_IDENTIFIER as ::core::ffi::c_int,
                    ident,
                );
                return false_0 != 0;
            }
            12 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Default \"%s.%s\" of type boolean is unknown\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as ::core::ffi::c_int,
                    xkb_atom_text(ctx, (*expr).field_ref.element),
                    xkb_atom_text(ctx, (*expr).field_ref.field),
                );
                return false_0 != 0;
            }
            24 | 22 => {
                ok = ExprResolveBoolean(ctx, (*expr).unary.child, set_rtrn);
                if ok {
                    *set_rtrn = !*set_rtrn;
                }
                return ok;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 25 | 14 | 11 | 16 | 15 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s of boolean values not permitted\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                    stmt_type_to_string((*expr).common.type_0),
                );
            }
            _ => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Unknown operator %d in ResolveBoolean\n\0".as_ptr() as *const i8,
                    XKB_ERROR_UNKNOWN_OPERATOR as ::core::ffi::c_int,
                    (*expr).common.type_0 as ::core::ffi::c_uint,
                );
            }
        }
        return false_0 != 0;
    }
}
unsafe extern "C" fn ExprResolveIntegerLookup(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut int64_t,
    mut pending: *mut bool,
    mut lookup: IdentLookupFunc,
    mut lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = false_0 != 0;
        let mut l: int64_t = 0 as int64_t;
        let mut r: int64_t = 0 as int64_t;
        let mut u: u32 = 0 as u32;
        let mut left: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut right: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        match (*expr).common.type_0 as ::core::ffi::c_uint {
            5 => {
                *val_rtrn = (*expr).integer.ival;
                return true_0 != 0;
            }
            4 | 6 | 7 | 8 | 9 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Found %s where an int was expected\n\0".as_ptr() as *const i8,
                    XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false_0 != 0;
            }
            10 => {
                if lookup.is_some() {
                    ok = lookup.expect("non-null function pointer")(
                        ctx,
                        lookupPriv,
                        (*expr).ident.ident,
                        &raw mut u,
                        pending,
                    );
                }
                if !ok {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Identifier \"%s\" of type int is unknown\n\0".as_ptr()
                            as *const i8,
                        XKB_ERROR_INVALID_IDENTIFIER as ::core::ffi::c_int,
                        xkb_atom_text(ctx, (*expr).ident.ident),
                    );
                } else {
                    *val_rtrn = u as int64_t;
                }
                if !pending.is_null() && *pending as ::core::ffi::c_int != 0 {
                    return false_0 != 0;
                }
                return ok;
            }
            12 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Default \"%s.%s\" of type int is unknown\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as ::core::ffi::c_int,
                    xkb_atom_text(ctx, (*expr).field_ref.element),
                    xkb_atom_text(ctx, (*expr).field_ref.field),
                );
                return false_0 != 0;
            }
            17 | 18 | 19 | 20 => {
                left = (*expr).binary.left as *mut ExprDef;
                right = (*expr).binary.right as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveIntegerLookup(
                        ctx, right, &raw mut r, pending, lookup, lookupPriv,
                    )
                {
                    return false_0 != 0;
                }
                match (*expr).common.type_0 as ::core::ffi::c_uint {
                    17 => {
                        let (c2rust_fresh0, c2rust_fresh1) = l.overflowing_add(r);
                        *val_rtrn = c2rust_fresh0;
                        if c2rust_fresh1 {
                            xkb_log(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] Addition %ld + %ld has an invalid mathematical result: %ld\n\0"
                                    .as_ptr() as *const i8,
                                XKB_ERROR_INTEGER_OVERFLOW as ::core::ffi::c_int,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return false_0 != 0;
                        }
                    }
                    18 => {
                        let (c2rust_fresh2, c2rust_fresh3) = l.overflowing_sub(r);
                        *val_rtrn = c2rust_fresh2;
                        if c2rust_fresh3 {
                            xkb_log(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] Substraction %ld - %ld has an invalid mathematical result: %ld\n\0"
                                    .as_ptr() as *const i8,
                                XKB_ERROR_INTEGER_OVERFLOW as ::core::ffi::c_int,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return false_0 != 0;
                        }
                    }
                    19 => {
                        let (c2rust_fresh4, c2rust_fresh5) = l.overflowing_mul(r);
                        *val_rtrn = c2rust_fresh4;
                        if c2rust_fresh5 {
                            xkb_log(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] Multiplication %ld * %ld has an invalid mathematical result: %ld\n\0"
                                    .as_ptr() as *const i8,
                                XKB_ERROR_INTEGER_OVERFLOW as ::core::ffi::c_int,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return false_0 != 0;
                        }
                    }
                    20 => {
                        if r == 0 as int64_t {
                            xkb_log(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] Cannot divide by zero: %ld / %ld\n\0".as_ptr()
                                    as *const i8,
                                XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                                l,
                                r,
                            );
                            return false_0 != 0;
                        }
                        *val_rtrn = l / r;
                    }
                    _ => {
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s of integers not permitted\n\0".as_ptr() as *const i8,
                            XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                            stmt_type_to_string((*expr).common.type_0),
                        );
                        return false_0 != 0;
                    }
                }
                return true_0 != 0;
            }
            21 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Assignment operator not implemented yet\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                );
            }
            22 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] The ! operator cannot be applied to an integer\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                );
                return false_0 != 0;
            }
            24 | 23 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv) {
                    return false_0 != 0;
                }
                *val_rtrn = if (*expr).common.type_0 as ::core::ffi::c_uint
                    == STMT_EXPR_NEGATE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    -l
                } else {
                    !l
                };
                return true_0 != 0;
            }
            25 => {
                left = (*expr).unary.child as *mut ExprDef;
                return ExprResolveIntegerLookup(ctx, left, val_rtrn, pending, lookup, lookupPriv);
            }
            _ => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Unknown operator %d in ResolveInteger\n\0".as_ptr() as *const i8,
                    XKB_ERROR_UNKNOWN_OPERATOR as ::core::ffi::c_int,
                    (*expr).common.type_0 as ::core::ffi::c_uint,
                );
            }
        }
        return false_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveInteger(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut int64_t,
) -> bool {
    unsafe {
        return ExprResolveIntegerLookup(
            ctx,
            expr,
            val_rtrn,
            ::core::ptr::null_mut::<bool>(),
            None,
            ::core::ptr::null::<::core::ffi::c_void>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveGroup(
    mut keymap_info: *const xkb_keymap_info,
    mut expr: *const ExprDef,
    mut absolute: bool,
    mut group_rtrn: *mut xkb_layout_index_t,
    mut pending: *mut bool,
) -> xkb_parser_error {
    unsafe {
        static mut pendingGroupIndexNames: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: 0 as u32,
            },
            LookupEntry {
                name: ::core::ptr::null::<i8>(),
                value: 0 as u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: b"Group\0".as_ptr() as *const i8,
            prefix_length: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: (*keymap_info).features.max_groups as u32,
            entries: &raw const (*keymap_info).lookup.groupIndexNames as *const LookupEntry,
            pending_entries: &raw const pendingGroupIndexNames as *const LookupEntry,
            is_mask: false_0 != 0,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        let mut result: int64_t = 0 as int64_t;
        if !ExprResolveIntegerLookup(
            (*keymap_info).keymap.ctx,
            expr,
            &raw mut result,
            pending,
            Some(
                NamedIntegerPatternLookup
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        ) {
            return (if (*keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                PARSER_FATAL_ERROR as ::core::ffi::c_int
            } else {
                PARSER_RECOVERABLE_ERROR as ::core::ffi::c_int
            }) as xkb_parser_error;
        }
        if result < absolute as ::core::ffi::c_int as int64_t
            || result > (*keymap_info).features.max_groups as int64_t
        {
            xkb_log(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Group index %ld is out of range (%u..%u)\n\0".as_ptr() as *const i8,
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                result,
                absolute as ::core::ffi::c_int,
                (*keymap_info).features.max_groups,
            );
            return (if (*keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                PARSER_FATAL_ERROR as ::core::ffi::c_int
            } else {
                PARSER_RECOVERABLE_ERROR as ::core::ffi::c_int
            }) as xkb_parser_error;
        }
        *group_rtrn = result as xkb_layout_index_t;
        return PARSER_SUCCESS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveLevel(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut level_rtrn: *mut xkb_level_index_t,
) -> bool {
    unsafe {
        let mut result: int64_t = 0 as int64_t;
        if !ExprResolveIntegerLookup(
            ctx,
            expr,
            &raw mut result,
            ::core::ptr::null_mut::<bool>(),
            Some(
                NamedIntegerPatternLookup
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const level_name_pattern as *const ::core::ffi::c_void,
        ) {
            return false_0 != 0;
        }
        if result < 1 as int64_t || result > XKB_LEVEL_MAX_IMPL as int64_t {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Shift level %ld is out of range (1..%u)\n\0".as_ptr() as *const i8,
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as ::core::ffi::c_int,
                result,
                2048 as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        *level_rtrn = (result - 1 as int64_t) as xkb_level_index_t;
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveButton(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut btn_rtrn: *mut int64_t,
) -> bool {
    unsafe {
        return ExprResolveIntegerLookup(
            ctx,
            expr,
            btn_rtrn,
            ::core::ptr::null_mut::<bool>(),
            Some(
                SimpleLookup
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const buttonNames as *const LookupEntry as *const ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveString(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut xkb_atom_t,
) -> bool {
    unsafe {
        match (*expr).common.type_0 as ::core::ffi::c_uint {
            4 => {
                *val_rtrn = (*expr).string.str;
                return true_0 != 0;
            }
            5 | 6 | 7 | 8 | 9 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Found %s, expected a string\n\0".as_ptr() as *const i8,
                    XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false_0 != 0;
            }
            10 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Identifier \"%s\" of type string not found\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_IDENTIFIER as ::core::ffi::c_int,
                    xkb_atom_text(ctx, (*expr).ident.ident),
                );
                return false_0 != 0;
            }
            12 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Default \"%s.%s\" of type string not found\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as ::core::ffi::c_int,
                    xkb_atom_text(ctx, (*expr).field_ref.element),
                    xkb_atom_text(ctx, (*expr).field_ref.field),
                );
                return false_0 != 0;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 24 | 22 | 25 | 14 | 11 | 16 | 15 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s of strings not permitted\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_XKB_SYNTAX as ::core::ffi::c_int,
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false_0 != 0;
            }
            _ => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Unknown operator %d in ResolveString\n\0".as_ptr() as *const i8,
                    XKB_ERROR_UNKNOWN_OPERATOR as ::core::ffi::c_int,
                    (*expr).common.type_0 as ::core::ffi::c_uint,
                );
            }
        }
        return false_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveEnum(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut u32,
    mut values: *const LookupEntry,
) -> bool {
    unsafe {
        if (*expr).common.type_0 as ::core::ffi::c_uint
            != STMT_EXPR_IDENT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Found a %s where an enumerated value was expected\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                stmt_type_to_string((*expr).common.type_0),
            );
            return false_0 != 0;
        }
        if !SimpleLookup(
            ctx,
            values as *const ::core::ffi::c_void,
            (*expr).ident.ident,
            val_rtrn,
            ::core::ptr::null_mut::<bool>(),
        ) {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Illegal identifier %s; expected one of:\n\0".as_ptr() as *const i8,
                XKB_ERROR_INVALID_IDENTIFIER as ::core::ffi::c_int,
                xkb_atom_text(ctx, (*expr).ident.ident),
            );
            while !values.is_null() && !(*values).name.is_null() {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] \t%s\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_IDENTIFIER as ::core::ffi::c_int,
                    (*values).name,
                );
                values = values.offset(1);
            }
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn ExprResolveMaskLookup(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut u32,
    mut pending: *mut bool,
    mut lookup: IdentLookupFunc,
    mut lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = false_0 != 0;
        let mut l: u32 = 0 as u32;
        let mut r: u32 = 0 as u32;
        let mut v: int64_t = 0 as int64_t;
        let mut left: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut right: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut bogus: *const i8 = ::core::ptr::null::<i8>();
        let mut c2rust_current_block_47: u64;
        match (*expr).common.type_0 as ::core::ffi::c_uint {
            5 => {
                if (*expr).integer.ival < 0 as int64_t
                    || (*expr).integer.ival > UINT32_MAX as int64_t
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Mask %s%#lx is out of range (0..%#x)\n\0".as_ptr() as *const i8,
                        if (*expr).integer.ival < 0 as int64_t {
                            b"-\0".as_ptr() as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        },
                        imaxabs((*expr).integer.ival as intmax_t),
                        4294967295 as ::core::ffi::c_uint,
                    );
                    return false_0 != 0;
                }
                *val_rtrn = (*expr).integer.ival as u32;
                return true_0 != 0;
            }
            4 | 6 | 7 | 8 | 9 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Found %s where a mask was expected\n\0".as_ptr() as *const i8,
                    XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false_0 != 0;
            }
            10 => {
                ok = lookup.expect("non-null function pointer")(
                    ctx,
                    lookupPriv,
                    (*expr).ident.ident,
                    val_rtrn,
                    pending,
                );
                if !ok {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Identifier \"%s\" of type int is unknown\n\0".as_ptr()
                            as *const i8,
                        XKB_ERROR_INVALID_IDENTIFIER as ::core::ffi::c_int,
                        xkb_atom_text(ctx, (*expr).ident.ident),
                    );
                }
                if !pending.is_null() && *pending as ::core::ffi::c_int != 0 {
                    return false_0 != 0;
                }
                return ok;
            }
            12 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Default \"%s.%s\" of type int is unknown\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as ::core::ffi::c_int,
                    xkb_atom_text(ctx, (*expr).field_ref.element),
                    xkb_atom_text(ctx, (*expr).field_ref.field),
                );
                return false_0 != 0;
            }
            13 => {
                bogus = b"array reference\0".as_ptr() as *const i8;
                c2rust_current_block_47 = 6716998617863641615;
            }
            11 => {
                c2rust_current_block_47 = 6716998617863641615;
            }
            17 | 18 | 19 | 20 => {
                left = (*expr).binary.left as *mut ExprDef;
                right = (*expr).binary.right as *mut ExprDef;
                if !ExprResolveMaskLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveMaskLookup(ctx, right, &raw mut r, pending, lookup, lookupPriv)
                {
                    return false_0 != 0;
                }
                match (*expr).common.type_0 as ::core::ffi::c_uint {
                    17 => {
                        *val_rtrn = l | r;
                    }
                    18 => {
                        *val_rtrn = l & !r;
                    }
                    19 | 20 => {
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Cannot %s masks; Illegal operation ignored\n\0".as_ptr()
                                as *const i8,
                            XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                            if (*expr).common.type_0 as ::core::ffi::c_uint
                                == STMT_EXPR_DIVIDE as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                b"divide\0".as_ptr() as *const i8
                            } else {
                                b"multiply\0".as_ptr() as *const i8
                            },
                        );
                        return false_0 != 0;
                    }
                    _ => {}
                }
                return true_0 != 0;
            }
            21 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Assignment operator not implemented yet\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                );
                c2rust_current_block_47 = 11626999923138678822;
            }
            24 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return false_0 != 0;
                }
                if v < 0 as int64_t || v > UINT32_MAX as int64_t {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Mask %s%#lx is out of range (0..%#x)\n\0".as_ptr() as *const i8,
                        if v < 0 as int64_t {
                            b"-\0".as_ptr() as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        },
                        imaxabs(v as intmax_t),
                        4294967295 as ::core::ffi::c_uint,
                    );
                    return false_0 != 0;
                }
                *val_rtrn = !(v as u32);
                return true_0 != 0;
            }
            25 | 23 | 22 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return false_0 != 0;
                }
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] The '%c' unary operator cannot be used with a mask\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_OPERATION as ::core::ffi::c_int,
                    stmt_type_to_operator_char((*expr).common.type_0) as ::core::ffi::c_int,
                );
                return false_0 != 0;
            }
            _ => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Unknown operator type %d in ResolveMask\n\0".as_ptr() as *const i8,
                    XKB_ERROR_UNKNOWN_OPERATOR as ::core::ffi::c_int,
                    (*expr).common.type_0 as ::core::ffi::c_uint,
                );
                c2rust_current_block_47 = 11626999923138678822;
            }
        }
        match c2rust_current_block_47 {
            11626999923138678822 => {}
            _ => {
                if bogus.is_null() {
                    bogus = b"function use\0".as_ptr() as *const i8;
                }
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Unexpected %s in mask expression; Expression Ignored\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                    bogus,
                );
                return false_0 != 0;
            }
        }
        return false_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveMask(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut mask_rtrn: *mut u32,
    mut values: *const LookupEntry,
) -> bool {
    unsafe {
        return ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn,
            ::core::ptr::null_mut::<bool>(),
            Some(
                SimpleLookup
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            values as *const ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveModMask(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut mod_type: mod_type,
    mut mods: *const xkb_mod_set,
    mut mask_rtrn: *mut xkb_mod_mask_t,
) -> bool {
    unsafe {
        let mut priv_0: LookupModMaskPriv = LookupModMaskPriv {
            mods: mods,
            mod_type: mod_type,
        };
        return ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn as *mut u32,
            ::core::ptr::null_mut::<bool>(),
            Some(
                LookupModMask
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut xkb_mod_mask_t,
                        *mut bool,
                    ) -> bool,
            ),
            &raw mut priv_0 as *const ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveMod(
    mut ctx: *mut xkb_context,
    mut def: *const ExprDef,
    mut mod_type: mod_type,
    mut mods: *const xkb_mod_set,
    mut ndx_rtrn: *mut xkb_mod_index_t,
) -> bool {
    unsafe {
        if (*def).common.type_0 as ::core::ffi::c_uint
            != STMT_EXPR_IDENT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot resolve virtual modifier: found %s where a virtual modifier name was expected\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                stmt_type_to_string((*def).common.type_0),
            );
            return false_0 != 0;
        }
        let mut name: xkb_atom_t = (*def).ident.ident;
        let mut ndx: xkb_mod_index_t = XkbModNameToIndex(mods, name, mod_type);
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot resolve virtual modifier: \"%s\" was not previously declared\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER as ::core::ffi::c_int,
                xkb_atom_text(ctx, name),
            );
            return false_0 != 0;
        }
        *ndx_rtrn = ndx;
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprResolveGroupMask(
    mut keymap_info: *const xkb_keymap_info,
    mut expr: *const ExprDef,
    mut group_rtrn: *mut xkb_layout_mask_t,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        static mut pendingGroupMaskNames: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: 0 as u32,
            },
            LookupEntry {
                name: ::core::ptr::null::<i8>(),
                value: 0 as u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: b"Group\0".as_ptr() as *const i8,
            prefix_length: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: (*keymap_info).features.max_groups as u32,
            entries: &raw const (*keymap_info).lookup.groupMaskNames as *const LookupEntry,
            pending_entries: &raw const pendingGroupMaskNames as *const LookupEntry,
            is_mask: true_0 != 0,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        return ExprResolveMaskLookup(
            (*keymap_info).keymap.ctx,
            expr,
            group_rtrn as *mut u32,
            pending_rtrn,
            Some(
                NamedIntegerPatternLookup
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        level_name_pattern = named_integer_pattern {
            prefix: b"Level\0".as_ptr() as *const i8,
            prefix_length: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: XKB_LEVEL_MAX_IMPL as u32,
            entries: ::core::ptr::null::<LookupEntry>(),
            pending_entries: ::core::ptr::null::<LookupEntry>(),
            is_mask: false_0 != 0,
            error_id: XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
        }
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
