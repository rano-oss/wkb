use c2rust_bitfields;
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
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
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
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}

pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut i8,
        pub _IO_read_end: *mut i8,
        pub _IO_read_base: *mut i8,
        pub _IO_write_base: *mut i8,
        pub _IO_write_ptr: *mut i8,
        pub _IO_write_end: *mut i8,
        pub _IO_buf_base: *mut i8,
        pub _IO_buf_end: *mut i8,
        pub _IO_save_base: *mut i8,
        pub _IO_backup_base: *mut i8,
        pub _IO_save_end: *mut i8,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [i8; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [i8; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [i8; 8],
    }
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
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
        pub fn xkb_atom_intern(ctx: *mut xkb_context, string: *const i8, len: usize) -> xkb_atom_t;
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
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }
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
    pub const XKB_LAYOUT_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    use super::context_h::xkb_context;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_keysym_to_upper(keysym: xkb_keysym_t) -> xkb_keysym_t;
        pub fn xkb_context_get_log_verbosity(context: *mut xkb_context) -> ::core::ffi::c_int;
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
    pub type C2Rust_Unnamed_14 = ::core::ffi::c_uint;
    pub const FALLBACK_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_14 = 0;
    pub const DEFAULT_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_14 = 1;
    pub const DEFAULT_KEY_REPEAT: C2Rust_Unnamed_14 = 0;
    pub type C2Rust_Unnamed_15 = ::core::ffi::c_uint;
    pub const FALLBACK_INTERPRET_VMODMAP: C2Rust_Unnamed_15 = 0;
    pub const DEFAULT_INTERPRET_VMODMAP: C2Rust_Unnamed_15 = 0;
    pub const DEFAULT_INTERPRET_VMOD: C2Rust_Unnamed_15 = 4294967295;
    pub const DEFAULT_KEY_VMODMAP: C2Rust_Unnamed_15 = 0;
    pub const XKB_MOD_NONE: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    pub const XKB_OVERLAY_INVALID: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
    #[inline]
    pub unsafe extern "C" fn XkbKeyByName(
        mut keymap: *const xkb_keymap,
        mut name: xkb_atom_t,
        mut use_aliases: bool,
    ) -> *mut xkb_key {
        unsafe {
            if name < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
                let match_0: KeycodeMatch = *(*keymap)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .key_names
                    .offset(name as isize);
                if match_0.c2rust_unnamed.found() {
                    if !match_0.c2rust_unnamed.is_alias() {
                        let key_index = match_0.key.index();
                        let actual_name = (*(*keymap).keys.offset(key_index as isize)).name;
                        return (*keymap).keys.offset(key_index as isize) as *mut xkb_key;
                    } else if use_aliases {
                        return (*keymap).keys.offset(
                            (*(*keymap)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .key_names
                                .offset(match_0.alias.real() as isize))
                            .key
                            .index() as isize,
                        ) as *mut xkb_key;
                    }
                }
            }
            return ::core::ptr::null_mut::<xkb_key>();
        }
    }
    #[inline]
    pub unsafe extern "C" fn XkbKeyNumLevels(
        mut key: *const xkb_key,
        mut layout: xkb_layout_index_t,
    ) -> xkb_level_index_t {
        unsafe {
            return (*(*(*key).groups.offset(layout as isize)).type_0).num_levels;
        }
    }

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
        pub fn clear_level(leveli: *mut xkb_level);
        pub fn XkbEscapeMapName(name: *mut i8);
        pub fn XkbModNameToIndex(
            mods: *const xkb_mod_set,
            name: xkb_atom_t,
            type_0: mod_type,
        ) -> xkb_mod_index_t;
        pub fn XkbLevelsSameSyms(a: *const xkb_level, b: *const xkb_level) -> bool;
        pub fn XkbLevelsSameActions(a: *const xkb_level, b: *const xkb_level) -> bool;
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
    pub type merge_mode = ::core::ffi::c_uint;
    pub const _MERGE_MODE_NUM_ENTRIES: merge_mode = 4;
    pub const MERGE_REPLACE: merge_mode = 3;
    pub const MERGE_OVERRIDE: merge_mode = 2;
    pub const MERGE_AUGMENT: merge_mode = 1;
    pub const MERGE_DEFAULT: merge_mode = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ParseCommon {
        pub next: *mut _ParseCommon,
        pub type_0: stmt_type,
    }
    pub type ParseCommon = _ParseCommon;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _IncludeStmt {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub stmt: *mut i8,
        pub file: *mut i8,
        pub map: *mut i8,
        pub modifier: *mut i8,
        pub next_incl: *mut _IncludeStmt,
    }
    pub type IncludeStmt = _IncludeStmt;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VarDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: *mut ExprDef,
        pub value: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VModDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub value: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct SymbolsDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub keyName: xkb_atom_t,
        pub symbols: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ModMapDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub modifier: xkb_atom_t,
        pub keys: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct UnknownStatement {
        pub common: ParseCommon,
        pub name: *mut i8,
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
        pub name: *mut i8,
        pub defs: *mut ParseCommon,
        pub file_type: xkb_file_type,
        pub flags: xkb_map_flags,
    }
    use super::atom_h::xkb_atom_t;
    use super::darray_h::darray_size_t;
    use super::stdint_intn_h::int64_t;
    use super::xkbcommon_h::xkb_keysym_t;
    extern "C" {
        pub fn stmt_type_to_string(type_0: stmt_type) -> *const i8;
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
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::keymap_h::{xkb_action_type, xkb_mod_set};
    use super::stdint_uintn_h::u32;
    use super::xkbcommon_h::{xkb_keysym_t, xkb_mod_index_t};
    extern "C" {
        pub fn ModIndexText(
            ctx: *mut xkb_context,
            mods: *const xkb_mod_set,
            ndx: xkb_mod_index_t,
        ) -> *const i8;
        pub fn ActionTypeText(type_0: xkb_action_type) -> *const i8;
        pub fn KeysymText(ctx: *mut xkb_context, sym: xkb_keysym_t) -> *const i8;
        pub fn KeyNameText(ctx: *mut xkb_context, name: xkb_atom_t) -> *const i8;
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
        pub features: C2Rust_Unnamed_17,
        pub lookup: C2Rust_Unnamed_16,
        pub pending_computations: *mut pending_computation_array,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_16 {
        pub groupIndexNames: [LookupEntry; 3],
        pub groupMaskNames: [LookupEntry; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_17 {
        pub max_groups: xkb_layout_index_t,
        pub max_overlays: xkb_overlay_index_t,
        pub controls_name_offset: uint8_t,
        pub group_lock_on_release: bool,
        pub mods_unlock_on_press: bool,
        pub mods_latch_on_press: bool,
        pub overlapping_overlays: bool,
    }
    #[inline]
    pub unsafe extern "C" fn safe_map_name(mut file: *mut XkbFile) -> *const i8 {
        unsafe {
            return if !(*file).name.is_null() {
                (*file).name as *const i8
            } else {
                b"(unnamed map)\0".as_ptr() as *const i8
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
        pub fn FreeXkbFile(file: *mut XkbFile);
    }
}
pub mod action_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ActionsInfo {
        pub actions: [xkb_action; 21],
    }
    use super::ast_h::{merge_mode, ExprDef};
    use super::keymap_h::{xkb_action, xkb_keymap, xkb_mod_set};

    use super::xkbcomp_priv_h::{xkb_keymap_info, xkb_parser_error};

    extern "C" {
        pub fn InitActionsInfo(keymap: *const xkb_keymap, info: *mut ActionsInfo);
        pub fn HandleActionDef(
            keymap_info: *const xkb_keymap_info,
            info: *mut ActionsInfo,
            mods: *const xkb_mod_set,
            def: *mut ExprDef,
            action: *mut xkb_action,
        ) -> xkb_parser_error;
        pub fn SetDefaultActionField(
            keymap_info: *const xkb_keymap_info,
            info: *mut ActionsInfo,
            mods: *mut xkb_mod_set,
            elem: *const i8,
            field: *const i8,
            array_ndx: *mut ExprDef,
            value_ptr: *mut *mut ExprDef,
            merge: merge_mode,
        ) -> xkb_parser_error;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn memmove(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strdup(__s: *const i8) -> *mut i8;
        pub fn strlen(__s: *const i8) -> usize;
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(__stream: *mut FILE, __format: *const i8, ...) -> ::core::ffi::c_int;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn atoi(__nptr: *const i8) -> ::core::ffi::c_int;
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn abort() -> !;
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
    #[inline]
    pub unsafe extern "C" fn strdup_safe(mut s: *const i8) -> *mut i8 {
        unsafe {
            return if !s.is_null() {
                strdup(s)
            } else {
                ::core::ptr::null_mut::<i8>()
            };
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

    use super::stdlib_h::calloc;
    use super::string_h::{memcpy, strdup};
    extern "C" {
        pub fn istrcmp(a: *const i8, b: *const i8) -> ::core::ffi::c_int;
        pub fn istrncmp(a: *const i8, b: *const i8, n: usize) -> ::core::ffi::c_int;
    }
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe extern "C" fn parse_dec_to_uint64_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut uint64_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: uint64_t = 0 as uint64_t;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint)
                    < 10 as ::core::ffi::c_uint
                && result <= (18446744073709551615 as uint64_t).wrapping_div(10 as uint64_t)
                && result.wrapping_mul(10 as uint64_t)
                    <= (18446744073709551615 as uint64_t).wrapping_sub(
                        (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                            as ::core::ffi::c_uchar as uint64_t,
                    )
            {
                result = result.wrapping_mul(10 as uint64_t).wrapping_add(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as uint64_t,
                );
                i = i.wrapping_add(1);
            }
            *out = result as uint64_t;
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
    #[inline]
    pub unsafe extern "C" fn popcount32(mut x: u32) -> ::core::ffi::c_uint {
        unsafe {
            return (x as ::core::ffi::c_ulong).count_ones() as i32 as ::core::ffi::c_uint;
        }
    }
    #[inline]
    pub unsafe extern "C" fn next_pow2(mut x: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
        unsafe {
            if x <= 1 as ::core::ffi::c_uint {
                return 1 as ::core::ffi::c_uint;
            }
            return (1 as ::core::ffi::c_uint)
                << (::core::mem::size_of::<::core::ffi::c_uint>() as usize)
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(
                        x.wrapping_sub(1 as ::core::ffi::c_uint).leading_zeros() as i32 as usize,
                    );
        }
    }

    use super::limits_h::CHAR_BIT;
    use super::stdint_uintn_h::{u32, uint64_t};
}
pub mod keysym_h {
    use super::xkbcommon_h::xkb_keysym_t;
    extern "C" {
        pub fn xkb_keysym_is_lower(keysym: xkb_keysym_t) -> bool;
        pub fn xkb_keysym_is_upper_or_title(keysym: xkb_keysym_t) -> bool;
        pub fn xkb_keysym_is_keypad(keysym: xkb_keysym_t) -> bool;
    }
}
pub mod vmod_h {
    use super::ast_h::{merge_mode, VModDef};
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_mod_set;

    extern "C" {
        pub fn InitVMods(info: *mut xkb_mod_set, mods: *const xkb_mod_set, reset: bool);
        pub fn MergeModSets(
            ctx: *mut xkb_context,
            into: *mut xkb_mod_set,
            from: *const xkb_mod_set,
            merge: merge_mode,
        );
        pub fn HandleVModDef(
            ctx: *mut xkb_context,
            mods: *mut xkb_mod_set,
            stmt: *mut VModDef,
        ) -> bool;
    }
}
pub mod expr_h {
    use super::ast_h::ExprDef;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::keymap_h::{mod_type, xkb_mod_set};
    use super::stdint_uintn_h::u32;
    use super::text_h::LookupEntry;
    use super::xkbcommon_h::{xkb_layout_index_t, xkb_mod_mask_t};
    use super::xkbcomp_priv_h::{xkb_keymap_info, xkb_parser_error};
    extern "C" {
        pub fn ExprResolveLhs(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            elem_rtrn: *mut *const i8,
            field_rtrn: *mut *const i8,
            index_rtrn: *mut *mut ExprDef,
        ) -> bool;
        pub fn ExprResolveModMask(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            mod_type: mod_type,
            mods: *const xkb_mod_set,
            mask_rtrn: *mut xkb_mod_mask_t,
        ) -> bool;
        pub fn ExprResolveBoolean(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            set_rtrn: *mut bool,
        ) -> bool;
        pub fn ExprResolveGroup(
            keymap_info: *const xkb_keymap_info,
            expr: *const ExprDef,
            absolute: bool,
            group_rtrn: *mut xkb_layout_index_t,
            pending_rtrn: *mut bool,
        ) -> xkb_parser_error;
        pub fn ExprResolveString(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            val_rtrn: *mut xkb_atom_t,
        ) -> bool;
        pub fn ExprResolveEnum(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            val_rtrn: *mut u32,
            values: *const LookupEntry,
        ) -> bool;
    }
}
pub mod util_mem_h {
    #[inline]
    pub unsafe extern "C" fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        unsafe {
            let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
            let mut swapped: *mut ::core::ffi::c_void = *original;
            *original = NULL;
            return swapped;
        }
    }
    use super::__stddef_null_h::NULL;
}
pub mod include_h {

    use super::ast_h::{xkb_file_type, IncludeStmt, XkbFile};
    use super::context_h::xkb_context;
    extern "C" {
        pub fn ExceedsIncludeMaxDepth(
            ctx: *mut xkb_context,
            include_depth: ::core::ffi::c_uint,
        ) -> bool;
        pub fn ProcessIncludeFile(
            ctx: *mut xkb_context,
            stmt: *const IncludeStmt,
            file_type: xkb_file_type,
            path: *mut i8,
            path_size: usize,
        ) -> *mut XkbFile;
    }
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod stdint_h {
    pub const UINT8_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
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

pub use self::action_h::{ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField};
pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, ModMapDef, ParseCommon, SymbolsDef,
    UnknownStatement, VModDef, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES,
    FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER,
    MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT,
    MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
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
pub use self::context_h::{
    xkb_atom_intern, xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
use self::expr_h::{
    ExprResolveBoolean, ExprResolveEnum, ExprResolveGroup, ExprResolveLhs, ExprResolveModMask,
    ExprResolveString,
};
use self::include_h::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
pub use self::internal::{__va_list_tag, __CHAR_BIT__};
pub use self::keymap_h::{
    clear_level, mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_14, C2Rust_Unnamed_15,
    C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6,
    C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, XkbEscapeMapName,
    XkbKeyByName, XkbKeyNumLevels, XkbLevelsSameActions, XkbLevelsSameSyms, XkbModNameToIndex,
    _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y,
    ACTION_ACCEL, ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR,
    ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP,
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
    DEFAULT_INTERPRET_KEY_REPEAT, DEFAULT_INTERPRET_VMOD, DEFAULT_INTERPRET_VMODMAP,
    DEFAULT_KEY_REPEAT, DEFAULT_KEY_VMODMAP, EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT,
    EXPLICIT_SYMBOLS, EXPLICIT_TYPES, EXPLICIT_VMODMAP, FALLBACK_INTERPRET_KEY_REPEAT,
    FALLBACK_INTERPRET_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL,
    MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT,
    XKB_MOD_NONE, XKB_OVERLAY_INVALID,
};
use self::keysym_h::{xkb_keysym_is_keypad, xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
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
pub use self::stdint_h::UINT8_MAX;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint64_t, uint8_t};
use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::{abort, atoi, calloc, free, realloc};
use self::string_h::{memcpy, memmove, memset, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::text_h::{ActionTypeText, KeyNameText, KeysymText, LookupEntry, ModIndexText};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t,
    __uint64_t, __uint8_t,
};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{istrcmp, istreq, istrncmp, istrneq, memdup, strdup_safe};
pub use self::utils_numbers_h::{next_pow2, parse_dec_to_uint64_t, popcount32};
use self::vmod_h::{HandleVModDef, InitVMods, MergeModSets};
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_context_get_log_verbosity, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keysym_t, xkb_keysym_to_upper, xkb_layout_index_t, xkb_layout_mask_t,
    xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t, xkb_log_level,
    xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_INVALID, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
    XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
    XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED,
    XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_error, xkb_parser_strict_flags, C2Rust_Unnamed_16, C2Rust_Unnamed_17, FreeXkbFile,
    PARSER_FATAL_ERROR, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
    PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
    PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolsInfo {
    pub name: *mut i8,
    pub errorCount: ::core::ffi::c_int,
    pub include_depth: ::core::ffi::c_uint,
    pub explicit_group: xkb_layout_index_t,
    pub max_groups: xkb_layout_index_t,
    pub keys: C2Rust_Unnamed_24,
    pub default_key: KeyInfo,
    pub default_actions: ActionsInfo,
    pub group_names: C2Rust_Unnamed_20,
    pub modmaps: C2Rust_Unnamed_18,
    pub mods: xkb_mod_set,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut ModMapEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModMapEntry {
    pub merge: merge_mode,
    pub haveSymbol: bool,
    pub modifier: xkb_mod_index_t,
    pub u: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_19 {
    pub keyName: xkb_atom_t,
    pub keySym: xkb_keysym_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_atom_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct KeyInfo {
    pub name: xkb_atom_t,
    pub vmodmap: xkb_mod_mask_t,
    pub default_type: xkb_atom_t,
    pub out_of_range_group_number: xkb_layout_index_t,
    pub groups: C2Rust_Unnamed_22,
    #[bitfield(
        name = "out_of_range_group_policy",
        ty = "xkb_layout_out_of_range_policy",
        bits = "0..=7"
    )]
    #[bitfield(name = "defined", ty = "key_field", bits = "8..=23")]
    #[bitfield(name = "merge", ty = "merge_mode", bits = "24..=31")]
    #[bitfield(name = "repeat", ty = "key_repeat", bits = "32..=39")]
    #[bitfield(name = "out_of_range_pending_group", ty = "bool", bits = "40..=40")]
    #[bitfield(name = "overlays_clear", ty = "bool", bits = "41..=41")]
    pub out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear:
        [u8; 6],
    pub overlays_alloc: xkb_overlay_index_t,
    pub overlays: xkb_overlay_mask_t,
    pub c2rust_unnamed: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_21 {
    pub overlay_key: *const xkb_key,
    pub overlays_keys: *mut *const xkb_key,
}
pub type key_repeat = ::core::ffi::c_uint;
pub const _KEY_REPEAT_NUM_ENTRIES: key_repeat = 3;
pub const KEY_REPEAT_NO: key_repeat = 2;
pub const KEY_REPEAT_YES: key_repeat = 1;
pub const KEY_REPEAT_UNDEFINED: key_repeat = 0;
pub type key_field = ::core::ffi::c_uint;
pub const KEY_FIELD_ALL: key_field = 31;
pub const KEY_FIELD_OVERLAY: key_field = 16;
pub const KEY_FIELD_VMODMAP: key_field = 8;
pub const KEY_FIELD_GROUPINFO: key_field = 4;
pub const KEY_FIELD_DEFAULT_TYPE: key_field = 2;
pub const KEY_FIELD_REPEAT: key_field = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut GroupInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupInfo {
    pub levels: C2Rust_Unnamed_23,
    pub defined: group_field,
    pub type_0: xkb_atom_t,
}
pub type group_field = ::core::ffi::c_uint;
pub const GROUP_FIELD_TYPE: group_field = 4;
pub const GROUP_FIELD_ACTS: group_field = 2;
pub const GROUP_FIELD_SYMS: group_field = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_level,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut KeyInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_action,
}
unsafe extern "C" fn StealLevelInfo(mut into: *mut xkb_level, mut from: *mut xkb_level) {
    unsafe {
        clear_level(into);
        if (*from).num_syms as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            (*into).s.syms = _steal(&raw mut (*from).s.syms as *mut ::core::ffi::c_void)
                as *mut xkb_keysym_t as *mut xkb_keysym_t;
        } else {
            (*into).s.sym = (*from).s.sym;
        }
        (*into).num_syms = (*from).num_syms;
        (*from).num_syms = 0 as xkb_keysym_count_t;
        if (*from).num_actions as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            (*into).a.actions = _steal(&raw mut (*from).a.actions as *mut ::core::ffi::c_void)
                as *mut xkb_action as *mut xkb_action;
        } else {
            (*into).a.action = (*from).a.action;
        }
        (*into).num_actions = (*from).num_actions;
        (*from).num_actions = 0 as xkb_action_count_t;
    }
}
unsafe extern "C" fn InitGroupInfo(mut groupi: *mut GroupInfo) {
    unsafe {
        memset(
            groupi as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GroupInfo>() as usize,
        );
    }
}
unsafe extern "C" fn ClearGroupInfo(mut groupi: *mut GroupInfo) {
    unsafe {
        let mut leveli: *mut xkb_level = ::core::ptr::null_mut::<xkb_level>();
        if !(*groupi).levels.item.is_null() {
            leveli = (*groupi)
                .levels
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut xkb_level;
            while leveli
                < (*groupi).levels.item.offset((*groupi).levels.size as isize) as *mut xkb_level
            {
                clear_level(leveli);
                leveli = leveli.offset(1);
            }
        }
        free((*groupi).levels.item as *mut ::core::ffi::c_void);
        (*groupi).levels.item = ::core::ptr::null_mut::<xkb_level>();
        (*groupi).levels.size = 0 as darray_size_t;
        (*groupi).levels.alloc = 0 as darray_size_t;
    }
}
unsafe extern "C" fn CopyGroupInfo(mut to: *mut GroupInfo, mut from: *const GroupInfo) {
    unsafe {
        (*to).defined = (*from).defined;
        (*to).type_0 = (*from).type_0;
        (*to).levels.item = ::core::ptr::null_mut::<xkb_level>();
        (*to).levels.size = 0 as darray_size_t;
        (*to).levels.alloc = 0 as darray_size_t;
        let mut __count: darray_size_t = (*from).levels.size;
        (*to).levels.size = __count;
        let mut __need: darray_size_t = (*to).levels.size;
        if __need > (*to).levels.alloc {
            (*to).levels.alloc = darray_next_alloc(
                (*to).levels.alloc,
                __need,
                ::core::mem::size_of::<xkb_level>() as usize,
            );
            (*to).levels.item = realloc(
                (*to).levels.item as *mut ::core::ffi::c_void,
                ((*to).levels.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
            ) as *mut xkb_level;
        }
        if __count != 0 as darray_size_t {
            memcpy(
                (*to).levels.item as *mut ::core::ffi::c_void,
                (*from).levels.item as *const ::core::ffi::c_void,
                (__count as usize).wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
            );
        }
        let mut j: xkb_level_index_t = 0 as xkb_level_index_t;
        while j < (*to).levels.size as xkb_level_index_t {
            if (*(*from).levels.item.offset(j as isize)).num_syms as ::core::ffi::c_int
                > 1 as ::core::ffi::c_int
            {
                let ref mut c2rust_fresh0 = (*(*to).levels.item.offset(j as isize)).s.syms;
                *c2rust_fresh0 = memdup(
                    (*(*from).levels.item.offset(j as isize)).s.syms as *const ::core::ffi::c_void,
                    (*(*from).levels.item.offset(j as isize)).num_syms as usize,
                    ::core::mem::size_of::<xkb_keysym_t>() as usize,
                ) as *mut xkb_keysym_t;
            }
            if (*(*from).levels.item.offset(j as isize)).num_actions as ::core::ffi::c_int
                > 1 as ::core::ffi::c_int
            {
                let ref mut c2rust_fresh1 = (*(*to).levels.item.offset(j as isize)).a.actions;
                *c2rust_fresh1 = memdup(
                    (*(*from).levels.item.offset(j as isize)).a.actions
                        as *const ::core::ffi::c_void,
                    (*(*from).levels.item.offset(j as isize)).num_actions as usize,
                    ::core::mem::size_of::<xkb_action>() as usize,
                ) as *mut xkb_action;
            }
            j = j.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn InitKeyInfo(mut ctx: *mut xkb_context, mut keyi: *mut KeyInfo) {
    unsafe {
        memset(
            keyi as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<KeyInfo>() as usize,
        );
        (*keyi).name = xkb_atom_intern(
            ctx,
            b"*\0".as_ptr() as *const i8,
            (::core::mem::size_of::<[i8; 2]>() as usize).wrapping_sub(1 as usize),
        );
        (*keyi).set_out_of_range_group_policy(
            XKB_LAYOUT_OUT_OF_RANGE_WRAP as xkb_layout_out_of_range_policy,
        );
        (*keyi).groups.item = ::core::ptr::null_mut::<GroupInfo>();
        (*keyi).groups.size = 0 as darray_size_t;
        (*keyi).groups.alloc = 0 as darray_size_t;
        (*keyi).c2rust_unnamed.overlay_key = ::core::ptr::null::<xkb_key>();
    }
}
unsafe extern "C" fn ClearKeyInfo(mut keyi: *mut KeyInfo) {
    unsafe {
        let mut groupi: *mut GroupInfo = ::core::ptr::null_mut::<GroupInfo>();
        if !(*keyi).groups.item.is_null() {
            groupi = (*keyi).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut GroupInfo;
            while groupi
                < (*keyi).groups.item.offset((*keyi).groups.size as isize) as *mut GroupInfo
            {
                ClearGroupInfo(groupi);
                groupi = groupi.offset(1);
            }
        }
        free((*keyi).groups.item as *mut ::core::ffi::c_void);
        (*keyi).groups.item = ::core::ptr::null_mut::<GroupInfo>();
        (*keyi).groups.size = 0 as darray_size_t;
        (*keyi).groups.alloc = 0 as darray_size_t;
        if (*keyi).overlays_alloc != 0 {
            free((*keyi).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn InitSymbolsInfo(
    mut info: *mut SymbolsInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: ::core::ffi::c_uint,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        memset(
            info as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SymbolsInfo>() as usize,
        );
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).include_depth = include_depth;
        (*info).keymap_info = keymap_info;
        (*info).max_groups = (*keymap_info).features.max_groups;
        InitKeyInfo((*keymap_info).keymap.ctx, &raw mut (*info).default_key);
        InitActionsInfo(
            &raw const (*keymap_info).keymap,
            &raw mut (*info).default_actions,
        );
        InitVMods(
            &raw mut (*info).mods,
            mods,
            include_depth > 0 as ::core::ffi::c_uint,
        );
        (*info).explicit_group = XKB_LAYOUT_INVALID as xkb_layout_index_t;
    }
}
unsafe extern "C" fn ClearSymbolsInfo(mut info: *mut SymbolsInfo) {
    unsafe {
        let mut keyi: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        free((*info).name as *mut ::core::ffi::c_void);
        if !(*info).keys.item.is_null() {
            keyi = (*info).keys.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyInfo;
            while keyi < (*info).keys.item.offset((*info).keys.size as isize) as *mut KeyInfo {
                ClearKeyInfo(keyi);
                keyi = keyi.offset(1);
            }
        }
        free((*info).keys.item as *mut ::core::ffi::c_void);
        (*info).keys.item = ::core::ptr::null_mut::<KeyInfo>();
        (*info).keys.size = 0 as darray_size_t;
        (*info).keys.alloc = 0 as darray_size_t;
        free((*info).group_names.item as *mut ::core::ffi::c_void);
        (*info).group_names.item = ::core::ptr::null_mut::<xkb_atom_t>();
        (*info).group_names.size = 0 as darray_size_t;
        (*info).group_names.alloc = 0 as darray_size_t;
        free((*info).modmaps.item as *mut ::core::ffi::c_void);
        (*info).modmaps.item = ::core::ptr::null_mut::<ModMapEntry>();
        (*info).modmaps.size = 0 as darray_size_t;
        (*info).modmaps.alloc = 0 as darray_size_t;
        ClearKeyInfo(&raw mut (*info).default_key);
    }
}
unsafe extern "C" fn KeyInfoText(mut info: *mut SymbolsInfo, mut keyi: *mut KeyInfo) -> *const i8 {
    unsafe {
        return KeyNameText((*info).ctx, (*keyi).name);
    }
}
unsafe extern "C" fn MergeGroups(
    mut info: *mut SymbolsInfo,
    mut into: *mut GroupInfo,
    mut from: *mut GroupInfo,
    mut clobber: bool,
    mut report: bool,
    mut group: xkb_layout_index_t,
    mut key_name: xkb_atom_t,
) -> bool {
    unsafe {
        if (*into).type_0 != (*from).type_0 {
            if !((*from).type_0 == XKB_ATOM_NONE as xkb_atom_t) {
                if (*into).type_0 == XKB_ATOM_NONE as xkb_atom_t {
                    (*into).type_0 = (*from).type_0;
                } else {
                    let mut use_0: xkb_atom_t = if clobber as ::core::ffi::c_int != 0 {
                        (*from).type_0
                    } else {
                        (*into).type_0
                    };
                    let mut ignore: xkb_atom_t = if clobber as ::core::ffi::c_int != 0 {
                        (*into).type_0
                    } else {
                        (*from).type_0
                    };
                    if report {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Multiple definitions for group %u type of key %s; Using %s, ignoring %s\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS
                                as ::core::ffi::c_int,
                            group.wrapping_add(1 as xkb_layout_index_t),
                            KeyNameText((*info).ctx, key_name),
                            xkb_atom_text((*info).ctx, use_0),
                            xkb_atom_text((*info).ctx, ignore),
                        );
                    }
                    (*into).type_0 = use_0;
                }
            }
        }
        (*into).defined = ((*into).defined as ::core::ffi::c_uint
            | (*from).defined as ::core::ffi::c_uint
                & GROUP_FIELD_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint)
            as group_field;
        if (*from).levels.size == 0 as darray_size_t {
            InitGroupInfo(from);
            return true_0 != 0;
        }
        if (*into).levels.size == 0 as darray_size_t {
            (*from).type_0 = (*into).type_0;
            *into = *from;
            InitGroupInfo(from);
            return true_0 != 0;
        }
        let levels_in_both: darray_size_t = if (*into).levels.size < (*from).levels.size {
            (*into).levels.size
        } else {
            (*from).levels.size
        };
        let mut fromKeysymsCount: darray_size_t = 0 as darray_size_t;
        let mut fromActionsCount: darray_size_t = 0 as darray_size_t;
        let mut i: darray_size_t = 0 as darray_size_t;
        while i < levels_in_both {
            let intoLevel: *mut xkb_level =
                (*into).levels.item.offset(i as isize) as *mut xkb_level;
            let fromLevel: *mut xkb_level =
                (*from).levels.item.offset(i as isize) as *mut xkb_level;
            let fromHasNoKeysym: bool =
                (*fromLevel).num_syms as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
            let fromHasNoAction: bool =
                (*fromLevel).num_actions as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
            if !(fromHasNoKeysym as ::core::ffi::c_int != 0
                && fromHasNoAction as ::core::ffi::c_int != 0)
            {
                let intoHasNoKeysym: bool =
                    (*intoLevel).num_syms as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
                let intoHasNoAction: bool =
                    (*intoLevel).num_actions as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
                if intoHasNoKeysym as ::core::ffi::c_int != 0
                    && intoHasNoAction as ::core::ffi::c_int != 0
                {
                    StealLevelInfo(intoLevel, fromLevel);
                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                    fromActionsCount = fromActionsCount.wrapping_add(1);
                } else {
                    if !XkbLevelsSameSyms(fromLevel, intoLevel) {
                        if report as ::core::ffi::c_int != 0
                            && !(intoHasNoKeysym as ::core::ffi::c_int != 0
                                || fromHasNoKeysym as ::core::ffi::c_int != 0)
                        {
                            xkb_log(
                                (*info).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] Multiple symbols for level %u/group %u on key %s; Using %s, ignoring %s\n\0"
                                    .as_ptr() as *const i8,
                                XKB_WARNING_CONFLICTING_KEY_SYMBOL as ::core::ffi::c_int,
                                i.wrapping_add(1 as darray_size_t),
                                group.wrapping_add(1 as xkb_layout_index_t),
                                KeyNameText((*info).ctx, key_name),
                                if clobber as ::core::ffi::c_int != 0 {
                                    b"from\0".as_ptr() as *const i8
                                } else {
                                    b"to\0".as_ptr() as *const i8
                                },
                                if clobber as ::core::ffi::c_int != 0 {
                                    b"to\0".as_ptr() as *const i8
                                } else {
                                    b"from\0".as_ptr() as *const i8
                                },
                            );
                        }
                        if !fromHasNoKeysym {
                            if clobber {
                                if ((*fromLevel).num_syms as ::core::ffi::c_int
                                    > 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    if ((*intoLevel).num_syms as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        free((*intoLevel).s.syms as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).s.syms = (*fromLevel).s.syms;
                                    (*intoLevel).num_syms = (*fromLevel).num_syms;
                                    (*fromLevel).num_syms = 0 as xkb_keysym_count_t;
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                } else if (*fromLevel).s.sym != XKB_KEY_NoSymbol as xkb_keysym_t {
                                    if ((*intoLevel).num_syms as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        free((*intoLevel).s.syms as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).s.sym = (*fromLevel).s.sym;
                                    (*intoLevel).num_syms = 1 as xkb_keysym_count_t;
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                }
                            } else if !(((*intoLevel).num_syms as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0)
                            {
                                if (*intoLevel).s.sym == XKB_KEY_NoSymbol as xkb_keysym_t {
                                    if ((*fromLevel).num_syms as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*intoLevel).s.syms = (*fromLevel).s.syms;
                                        (*intoLevel).num_syms = (*fromLevel).num_syms;
                                        (*fromLevel).num_syms = 0 as xkb_keysym_count_t;
                                    } else {
                                        (*intoLevel).s.sym = (*fromLevel).s.sym;
                                        (*intoLevel).num_syms = 1 as xkb_keysym_count_t;
                                    }
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                }
                            }
                        }
                    }
                    if !XkbLevelsSameActions(intoLevel, fromLevel) {
                        if report as ::core::ffi::c_int != 0
                            && !(intoHasNoAction as ::core::ffi::c_int != 0
                                || fromHasNoAction as ::core::ffi::c_int != 0)
                        {
                            if ((*intoLevel).num_actions as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                xkb_log(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                    b"[XKB-%03d] Multiple actions for level %u/group %u on key %s; %s\n\0"
                                        .as_ptr() as *const i8,
                                    XKB_WARNING_CONFLICTING_KEY_ACTION as ::core::ffi::c_int,
                                    i.wrapping_add(1 as darray_size_t),
                                    group.wrapping_add(1 as xkb_layout_index_t),
                                    KeyNameText((*info).ctx, key_name),
                                    if clobber as ::core::ffi::c_int != 0 {
                                        b"Using from, ignoring to\0".as_ptr()
                                            as *const i8
                                    } else {
                                        b"Using to, ignoring from\0".as_ptr()
                                            as *const i8
                                    },
                                );
                            } else {
                                let mut use_1: *mut xkb_action =
                                    ::core::ptr::null_mut::<xkb_action>();
                                let mut ignore_0: *mut xkb_action =
                                    ::core::ptr::null_mut::<xkb_action>();
                                use_1 = if clobber as ::core::ffi::c_int != 0 {
                                    &raw mut (*fromLevel).a.action
                                } else {
                                    &raw mut (*intoLevel).a.action
                                };
                                ignore_0 = if clobber as ::core::ffi::c_int != 0 {
                                    &raw mut (*intoLevel).a.action
                                } else {
                                    &raw mut (*fromLevel).a.action
                                };
                                xkb_log(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                    b"[XKB-%03d] Multiple actions for level %u/group %u on key %s; Using %s, ignoring %s\n\0"
                                        .as_ptr() as *const i8,
                                    XKB_WARNING_CONFLICTING_KEY_ACTION as ::core::ffi::c_int,
                                    i.wrapping_add(1 as darray_size_t),
                                    group.wrapping_add(1 as xkb_layout_index_t),
                                    KeyNameText((*info).ctx, key_name),
                                    ActionTypeText((*use_1).type_0),
                                    ActionTypeText((*ignore_0).type_0),
                                );
                            }
                        }
                        if !fromHasNoAction {
                            if clobber {
                                if ((*fromLevel).num_actions as ::core::ffi::c_int
                                    > 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    if ((*intoLevel).num_actions as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        free((*intoLevel).a.actions as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).a.actions = (*fromLevel).a.actions;
                                    (*intoLevel).num_actions = (*fromLevel).num_actions;
                                    (*fromLevel).num_actions = 0 as xkb_action_count_t;
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                } else if (*fromLevel).a.action.type_0 as ::core::ffi::c_uint
                                    != ACTION_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    if ((*intoLevel).num_actions as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        free((*intoLevel).a.actions as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).a.action = (*fromLevel).a.action;
                                    (*intoLevel).num_actions = 1 as xkb_action_count_t;
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                }
                            } else if !(((*intoLevel).num_actions as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0)
                            {
                                if (*intoLevel).a.action.type_0 as ::core::ffi::c_uint
                                    == ACTION_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    if ((*fromLevel).num_actions as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*intoLevel).a.actions = (*fromLevel).a.actions;
                                        (*intoLevel).num_actions = (*fromLevel).num_actions;
                                        (*fromLevel).num_actions = 0 as xkb_action_count_t;
                                    } else {
                                        (*intoLevel).a.action = (*fromLevel).a.action;
                                        (*intoLevel).num_actions = 1 as xkb_action_count_t;
                                    }
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let mut level: *mut xkb_level = ::core::ptr::null_mut::<xkb_level>();
        if !(*from).levels.item.is_null() {
            level = (*from).levels.item.offset(levels_in_both as isize) as *mut xkb_level;
            while level < (*from).levels.item.offset((*from).levels.size as isize) as *mut xkb_level
            {
                (*into).levels.size = (*into).levels.size.wrapping_add(1 as darray_size_t);
                let mut __need: darray_size_t = (*into).levels.size;
                if __need > (*into).levels.alloc {
                    (*into).levels.alloc = darray_next_alloc(
                        (*into).levels.alloc,
                        __need,
                        ::core::mem::size_of::<xkb_level>() as usize,
                    );
                    (*into).levels.item = realloc(
                        (*into).levels.item as *mut ::core::ffi::c_void,
                        ((*into).levels.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                    ) as *mut xkb_level;
                }
                *(*into)
                    .levels
                    .item
                    .offset((*into).levels.size.wrapping_sub(1 as darray_size_t) as isize) = *level;
                (*level).num_syms = 0 as xkb_keysym_count_t;
                (*level).num_actions = 0 as xkb_action_count_t;
                fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                fromActionsCount = fromActionsCount.wrapping_add(1);
                level = level.offset(1);
            }
        }
        if fromKeysymsCount != 0 {
            if fromKeysymsCount == (*into).levels.size {
                (*into).defined = ((*into).defined as ::core::ffi::c_uint
                    & !(GROUP_FIELD_SYMS as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    as group_field;
            }
            (*into).defined = ((*into).defined as ::core::ffi::c_uint
                | (*from).defined as ::core::ffi::c_uint
                    & GROUP_FIELD_SYMS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as group_field;
        }
        if fromActionsCount != 0 {
            if fromActionsCount == (*into).levels.size {
                (*into).defined = ((*into).defined as ::core::ffi::c_uint
                    & !(GROUP_FIELD_ACTS as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    as group_field;
            }
            (*into).defined = ((*into).defined as ::core::ffi::c_uint
                | (*from).defined as ::core::ffi::c_uint
                    & GROUP_FIELD_ACTS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as group_field;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn UseNewKeyField(
    mut field: key_field,
    mut old: key_field,
    mut new: key_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut key_field,
) -> bool {
    unsafe {
        if old as ::core::ffi::c_uint & field as ::core::ffi::c_uint == 0 {
            return new as ::core::ffi::c_uint & field as ::core::ffi::c_uint != 0;
        }
        if new as ::core::ffi::c_uint & field as ::core::ffi::c_uint != 0 {
            if report {
                *collide =
                    (*collide as ::core::ffi::c_uint | field as ::core::ffi::c_uint) as key_field;
            }
            return clobber;
        }
        return false_0 != 0;
    }
}
unsafe extern "C" fn overlays_get(
    mut info: *const KeyInfo,
    mut bit: xkb_overlay_index_t,
    mut key_out: *mut *const xkb_key,
) -> bool {
    unsafe {
        if bit as ::core::ffi::c_int
            >= (::core::mem::size_of::<xkb_overlay_mask_t>() as usize)
                .wrapping_mul(CHAR_BIT as usize) as xkb_overlay_index_t
                as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        let mask: xkb_overlay_mask_t =
            ((1 as ::core::ffi::c_uint) << bit as ::core::ffi::c_int) as xkb_overlay_mask_t;
        if (*info).overlays as ::core::ffi::c_int & mask as ::core::ffi::c_int == 0 {
            return false_0 != 0;
        }
        if !key_out.is_null() {
            if (*info).overlays_alloc == 0 {
                *key_out = (*info).c2rust_unnamed.overlay_key;
            } else {
                let low: xkb_overlay_mask_t = ((*info).overlays as ::core::ffi::c_uint
                    & (mask as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint))
                    as xkb_overlay_mask_t;
                let index: xkb_overlay_index_t = popcount32(low as u32) as xkb_overlay_index_t;
                *key_out = *(*info).c2rust_unnamed.overlays_keys.offset(index as isize);
            }
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn overlays_insert(
    mut keyi: *mut KeyInfo,
    mut bit: xkb_overlay_index_t,
    mut key: *const xkb_key,
) -> bool {
    unsafe {
        if bit as ::core::ffi::c_int
            >= (::core::mem::size_of::<xkb_overlay_mask_t>() as usize)
                .wrapping_mul(CHAR_BIT as usize) as xkb_overlay_index_t
                as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        let mask: xkb_overlay_mask_t =
            ((1 as ::core::ffi::c_uint) << bit as ::core::ffi::c_int) as xkb_overlay_mask_t;
        if (*keyi).overlays as ::core::ffi::c_int & mask as ::core::ffi::c_int != 0
            && !(*keyi).overlays_clear()
        {
            if (*keyi).overlays_alloc == 0 {
                (*keyi).c2rust_unnamed.overlay_key = key;
                if key.is_null() {
                    (*keyi).set_overlays_clear((true_0 != 0) as bool);
                }
            } else {
                let low: xkb_overlay_mask_t = ((*keyi).overlays as ::core::ffi::c_int
                    & (mask as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                        as xkb_overlay_mask_t as ::core::ffi::c_int)
                    as xkb_overlay_mask_t;
                let index: xkb_overlay_index_t = popcount32(low as u32) as xkb_overlay_index_t;
                let ref mut c2rust_fresh4 =
                    *(*keyi).c2rust_unnamed.overlays_keys.offset(index as isize);
                *c2rust_fresh4 = key;
            }
            return true_0 != 0;
        }
        if (*keyi).overlays == 0
            || (*keyi).overlays_clear() as ::core::ffi::c_int != 0 && key.is_null()
        {
            (*keyi).c2rust_unnamed.overlay_key = key;
            (*keyi).overlays = ((*keyi).overlays as ::core::ffi::c_int | mask as ::core::ffi::c_int)
                as xkb_overlay_mask_t;
            (*keyi).set_overlays_clear(key.is_null() as bool);
        } else if (*keyi).overlays_alloc == 0 {
            let overlays: xkb_overlay_mask_t = ((*keyi).overlays as ::core::ffi::c_int
                | mask as ::core::ffi::c_int)
                as xkb_overlay_mask_t;
            let alloc: xkb_overlay_index_t = popcount32(overlays as u32) as xkb_overlay_index_t;
            let tmp: *mut *const xkb_key = calloc(
                alloc as usize,
                ::core::mem::size_of::<*const xkb_key>() as usize,
            ) as *mut *const xkb_key;
            if tmp.is_null() {
                return false_0 != 0;
            }
            if !(*keyi).overlays_clear() {
                let low_0: xkb_overlay_mask_t = (overlays as ::core::ffi::c_int
                    & ((*keyi).overlays as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        as xkb_overlay_mask_t as ::core::ffi::c_int)
                    as xkb_overlay_mask_t;
                let idx: xkb_overlay_index_t = popcount32(low_0 as u32) as xkb_overlay_index_t;
                let ref mut c2rust_fresh5 = *tmp.offset(idx as isize);
                *c2rust_fresh5 = (*keyi).c2rust_unnamed.overlay_key;
            }
            let low_1: xkb_overlay_mask_t = (overlays as ::core::ffi::c_int
                & (mask as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                    as xkb_overlay_mask_t as ::core::ffi::c_int)
                as xkb_overlay_mask_t;
            let idx_0: xkb_overlay_index_t = popcount32(low_1 as u32) as xkb_overlay_index_t;
            let ref mut c2rust_fresh6 = *tmp.offset(idx_0 as isize);
            *c2rust_fresh6 = key;
            (*keyi).c2rust_unnamed.overlays_keys = tmp;
            (*keyi).overlays_alloc = alloc;
            (*keyi).overlays = overlays;
            (*keyi).set_overlays_clear((false_0 != 0) as bool);
        } else {
            let overlays_0: xkb_overlay_mask_t = ((*keyi).overlays as ::core::ffi::c_int
                | mask as ::core::ffi::c_int)
                as xkb_overlay_mask_t;
            let count: xkb_overlay_index_t = popcount32(overlays_0 as u32) as xkb_overlay_index_t;
            if count as ::core::ffi::c_int > (*keyi).overlays_alloc as ::core::ffi::c_int {
                let alloc_0: xkb_overlay_index_t =
                    next_pow2(count as ::core::ffi::c_uint) as xkb_overlay_index_t;
                let tmp_0: *mut *const xkb_key = realloc(
                    (*keyi).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void,
                    (alloc_0 as usize)
                        .wrapping_mul(::core::mem::size_of::<*const xkb_key>() as usize),
                ) as *mut *const xkb_key;
                if tmp_0.is_null() {
                    return false_0 != 0;
                }
                (*keyi).c2rust_unnamed.overlays_keys = tmp_0;
                (*keyi).overlays_alloc = alloc_0;
            }
            let low_2: xkb_overlay_mask_t = (overlays_0 as ::core::ffi::c_int
                & (mask as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                    as xkb_overlay_mask_t as ::core::ffi::c_int)
                as xkb_overlay_mask_t;
            let index_0: xkb_overlay_index_t = popcount32(low_2 as u32) as xkb_overlay_index_t;
            if index_0 as ::core::ffi::c_int >= (*keyi).overlays_alloc as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Critical Error: Reached unreachable line in %s at %d\n\0".as_ptr()
                        as *const i8,
                    b"../src/xkbcomp/symbols.c\0".as_ptr() as *const i8,
                    654 as ::core::ffi::c_int,
                );
                abort();
            }
            if (index_0 as ::core::ffi::c_uint)
                < (count as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
            {
                memmove(
                    (*keyi)
                        .c2rust_unnamed
                        .overlays_keys
                        .offset(index_0 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    (*keyi)
                        .c2rust_unnamed
                        .overlays_keys
                        .offset(index_0 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    ((count as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        .wrapping_sub(index_0 as ::core::ffi::c_uint)
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<*const xkb_key>() as usize),
                );
            }
            let ref mut c2rust_fresh7 = *(*keyi)
                .c2rust_unnamed
                .overlays_keys
                .offset(index_0 as isize);
            *c2rust_fresh7 = key;
            (*keyi).overlays = overlays_0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn merge_overlays(
    mut info: *mut SymbolsInfo,
    mut into: *mut KeyInfo,
    mut from: *mut KeyInfo,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut key_field,
) -> bool {
    unsafe {
        if !((*from).defined() as ::core::ffi::c_int & KEY_FIELD_OVERLAY as ::core::ffi::c_int == 0)
        {
            if (*into).defined() as ::core::ffi::c_int & KEY_FIELD_OVERLAY as ::core::ffi::c_int
                == 0
            {
                (*into).overlays = (*from).overlays;
                (*into).c2rust_unnamed.overlays_keys = (*from).c2rust_unnamed.overlays_keys;
                (*from).c2rust_unnamed.overlays_keys = ::core::ptr::null_mut::<*const xkb_key>();
                (*into).overlays_alloc = (*from).overlays_alloc;
                (*from).overlays_alloc = 0 as xkb_overlay_index_t;
                (*into).set_defined(
                    (*into).defined() | KEY_FIELD_OVERLAY as ::core::ffi::c_int as key_field,
                );
            } else if (*into).overlays_clear() as ::core::ffi::c_int != 0
                && (*from).overlays_clear() as ::core::ffi::c_int != 0
            {
                (*into).overlays = ((*into).overlays as ::core::ffi::c_int
                    | (*from).overlays as ::core::ffi::c_int)
                    as xkb_overlay_mask_t;
            } else if (*(*info).keymap_info).features.overlapping_overlays {
                let result_mask: xkb_overlay_mask_t = ((*into).overlays as ::core::ffi::c_int
                    | (*from).overlays as ::core::ffi::c_int)
                    as xkb_overlay_mask_t;
                let count: xkb_overlay_index_t =
                    popcount32(result_mask as u32) as xkb_overlay_index_t;
                if count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Critical Error: Reached unreachable line in %s at %d\n\0".as_ptr()
                            as *const i8,
                        b"../src/xkbcomp/symbols.c\0".as_ptr() as *const i8,
                        696 as ::core::ffi::c_int,
                    );
                    abort();
                }
                let mut dest: *mut KeyInfo = into;
                let mut src: *mut KeyInfo = from;
                if !((*into).overlays_alloc as ::core::ffi::c_int >= count as ::core::ffi::c_int
                    && (*into).overlays_alloc as ::core::ffi::c_int
                        >= (*from).overlays_alloc as ::core::ffi::c_int)
                {
                    if (*from).overlays_alloc as ::core::ffi::c_int >= count as ::core::ffi::c_int {
                        dest = from;
                        src = into;
                        clobber = !clobber;
                    } else if count as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        if ((*into).overlays_alloc as ::core::ffi::c_int)
                            < (*from).overlays_alloc as ::core::ffi::c_int
                        {
                            dest = from;
                            src = into;
                            clobber = !clobber;
                        }
                        if (*dest).overlays_alloc == 0 {
                            let mut tmp: *mut *const xkb_key = calloc(
                                count as usize,
                                ::core::mem::size_of::<*const xkb_key>() as usize,
                            )
                                as *mut *const xkb_key;
                            if tmp.is_null() {
                                return false_0 != 0;
                            }
                            if !(*dest).c2rust_unnamed.overlay_key.is_null() {
                                let ref mut c2rust_fresh2 =
                                    *tmp.offset(0 as ::core::ffi::c_int as isize);
                                *c2rust_fresh2 = (*dest).c2rust_unnamed.overlay_key;
                            }
                            (*dest).c2rust_unnamed.overlays_keys = tmp;
                            (*dest).overlays_alloc = count;
                        } else {
                            let mut tmp_0: *mut *const xkb_key =
                                realloc(
                                    (*dest).c2rust_unnamed.overlays_keys
                                        as *mut ::core::ffi::c_void,
                                    (count as usize).wrapping_mul(::core::mem::size_of::<
                                        *const xkb_key,
                                    >(
                                    )
                                        as usize),
                                ) as *mut *const xkb_key;
                            if tmp_0.is_null() {
                                return false_0 != 0;
                            }
                            (*dest).c2rust_unnamed.overlays_keys = tmp_0;
                            (*dest).overlays_alloc = count;
                        }
                    }
                }
                let mut remaining: xkb_overlay_mask_t = (*src).overlays;
                let mut src_keys: *mut *const xkb_key =
                    if (*src).overlays_clear() as ::core::ffi::c_int != 0 {
                        ::core::ptr::null_mut::<*const xkb_key>()
                    } else if (*src).overlays_alloc as ::core::ffi::c_int != 0 {
                        (*src).c2rust_unnamed.overlays_keys
                    } else {
                        &raw mut (*src).c2rust_unnamed.overlay_key
                    };
                while remaining != 0 {
                    let lsb: xkb_overlay_mask_t = (remaining as ::core::ffi::c_int
                        & (!(remaining as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            as xkb_overlay_mask_t as ::core::ffi::c_int)
                        as xkb_overlay_mask_t;
                    let bit: xkb_overlay_index_t =
                        popcount32((lsb as u32).wrapping_sub(1 as u32)) as xkb_overlay_index_t;
                    remaining = (remaining as ::core::ffi::c_int & !(lsb as ::core::ffi::c_int))
                        as xkb_overlay_mask_t;
                    if !(*src).overlays_clear()
                        && (*src).overlays_alloc == 0
                        && remaining as ::core::ffi::c_int != 0
                    {
                        fprintf(
                            stderr,
                            b"Critical Error: Reached unreachable line in %s at %d\n\0".as_ptr()
                                as *const i8,
                            b"../src/xkbcomp/symbols.c\0".as_ptr() as *const i8,
                            758 as ::core::ffi::c_int,
                        );
                        abort();
                    }
                    let mut src_key: *const xkb_key = if !src_keys.is_null() {
                        let c2rust_fresh3 = src_keys;
                        src_keys = src_keys.offset(1);
                        *c2rust_fresh3
                    } else {
                        ::core::ptr::null::<xkb_key>()
                    };
                    let mut dest_key: *const xkb_key = ::core::ptr::null::<xkb_key>();
                    let conflict: bool = overlays_get(dest, bit, &raw mut dest_key) as bool;
                    if conflict {
                        if dest_key == src_key {
                            continue;
                        }
                        if report {
                            *collide = (*collide as ::core::ffi::c_uint
                                | KEY_FIELD_OVERLAY as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as key_field;
                        }
                    }
                    if (!conflict || clobber as ::core::ffi::c_int != 0)
                        && !overlays_insert(dest, bit, src_key)
                    {
                        return false_0 != 0;
                    }
                }
                if into != dest {
                    if (*into).overlays_alloc != 0 {
                        free((*into).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void);
                    }
                    (*into).overlays = (*from).overlays;
                    (*into).set_overlays_clear((*from).overlays_clear() as bool);
                    (*into).overlays_alloc = (*from).overlays_alloc;
                    if (*from).overlays_alloc != 0 {
                        (*into).c2rust_unnamed.overlays_keys = (*from).c2rust_unnamed.overlays_keys;
                        (*from).c2rust_unnamed.overlays_keys =
                            ::core::ptr::null_mut::<*const xkb_key>();
                        (*from).overlays_alloc = 0 as xkb_overlay_index_t;
                    } else {
                        (*into).c2rust_unnamed.overlay_key = (*from).c2rust_unnamed.overlay_key;
                    }
                }
            } else {
                if (*into).overlays as ::core::ffi::c_int == (*from).overlays as ::core::ffi::c_int
                    && (*into).overlays_clear() as ::core::ffi::c_int
                        == (*from).overlays_clear() as ::core::ffi::c_int
                    && (*into).c2rust_unnamed.overlay_key == (*from).c2rust_unnamed.overlay_key
                {
                    return true_0 != 0;
                }
                if (*into).overlays as ::core::ffi::c_int & (*from).overlays as ::core::ffi::c_int
                    == 0
                {
                    if (*into).overlays_clear() {
                        (*into).overlays = (*from).overlays;
                        (*into).set_overlays_clear((*from).overlays_clear() as bool);
                        (*into).overlays_alloc = (*from).overlays_alloc;
                        (*into).c2rust_unnamed.overlay_key = (*from).c2rust_unnamed.overlay_key;
                        return true_0 != 0;
                    } else if (*from).overlays_clear() {
                        return true_0 != 0;
                    }
                }
                if report {
                    *collide = (*collide as ::core::ffi::c_uint
                        | KEY_FIELD_OVERLAY as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as key_field;
                }
                if clobber {
                    (*into).overlays = (*from).overlays;
                    (*into).set_overlays_clear((*from).overlays_clear() as bool);
                    (*into).overlays_alloc = (*from).overlays_alloc;
                    (*into).c2rust_unnamed.overlay_key = (*from).c2rust_unnamed.overlay_key;
                }
            }
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn MergeKeys(
    mut info: *mut SymbolsInfo,
    mut into: *mut KeyInfo,
    mut from: *mut KeyInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut i: xkb_layout_index_t = 0;
        let mut groups_in_both: xkb_layout_index_t = 0;
        let mut collide: key_field = 0 as key_field;
        let verbosity: ::core::ffi::c_int =
            xkb_context_get_log_verbosity((*info).ctx) as ::core::ffi::c_int;
        let clobber: bool =
            (*from).merge() as ::core::ffi::c_int != MERGE_AUGMENT as ::core::ffi::c_int;
        let report: bool = same_file as ::core::ffi::c_int != 0
            && verbosity > 0 as ::core::ffi::c_int
            || verbosity > 9 as ::core::ffi::c_int;
        if (*from).merge() as ::core::ffi::c_int == MERGE_REPLACE as ::core::ffi::c_int {
            ClearKeyInfo(into);
            *into = *from;
            InitKeyInfo((*info).ctx, from);
            return true_0 != 0;
        }
        groups_in_both = (if (*into).groups.size < (*from).groups.size {
            (*into).groups.size
        } else {
            (*from).groups.size
        }) as xkb_layout_index_t;
        i = 0 as xkb_layout_index_t;
        while i < groups_in_both {
            MergeGroups(
                info,
                (*into).groups.item.offset(i as isize) as *mut GroupInfo,
                (*from).groups.item.offset(i as isize) as *mut GroupInfo,
                clobber,
                report,
                i,
                (*into).name,
            );
            i = i.wrapping_add(1);
        }
        i = groups_in_both;
        while i < (*from).groups.size as xkb_layout_index_t {
            (*into).groups.size = (*into).groups.size.wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = (*into).groups.size;
            if __need > (*into).groups.alloc {
                (*into).groups.alloc = darray_next_alloc(
                    (*into).groups.alloc,
                    __need,
                    ::core::mem::size_of::<GroupInfo>() as usize,
                );
                (*into).groups.item = realloc(
                    (*into).groups.item as *mut ::core::ffi::c_void,
                    ((*into).groups.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                ) as *mut GroupInfo;
            }
            *(*into)
                .groups
                .item
                .offset((*into).groups.size.wrapping_sub(1 as darray_size_t) as isize) =
                *(*from).groups.item.offset(i as isize);
            InitGroupInfo((*from).groups.item.offset(i as isize) as *mut GroupInfo);
            i = i.wrapping_add(1);
        }
        if UseNewKeyField(
            KEY_FIELD_VMODMAP,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).vmodmap = (*from).vmodmap;
            (*into).set_defined(
                (*into).defined() | KEY_FIELD_VMODMAP as ::core::ffi::c_int as key_field,
            );
        }
        if UseNewKeyField(
            KEY_FIELD_REPEAT,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).set_repeat((*from).repeat() as key_repeat);
            (*into).set_defined(
                (*into).defined() | KEY_FIELD_REPEAT as ::core::ffi::c_int as key_field,
            );
        }
        if UseNewKeyField(
            KEY_FIELD_DEFAULT_TYPE,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).default_type = (*from).default_type;
            (*into).set_defined(
                (*into).defined() | KEY_FIELD_DEFAULT_TYPE as ::core::ffi::c_int as key_field,
            );
        }
        if UseNewKeyField(
            KEY_FIELD_GROUPINFO,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).set_out_of_range_pending_group((*from).out_of_range_pending_group() as bool);
            (*into).set_out_of_range_group_policy(
                (*from).out_of_range_group_policy() as xkb_layout_out_of_range_policy
            );
            (*into).out_of_range_group_number = (*from).out_of_range_group_number;
            (*into).set_defined(
                (*into).defined() | KEY_FIELD_GROUPINFO as ::core::ffi::c_int as key_field,
            );
        }
        if !merge_overlays(info, into, from, clobber, report, &raw mut collide) {
            return false_0 != 0;
        }
        if collide as u64 != 0 {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Symbol map for key %s redefined; Using %s definition for conflicting fields\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_CONFLICTING_KEY_FIELDS as ::core::ffi::c_int,
                KeyNameText((*info).ctx, (*into).name),
                if clobber as ::core::ffi::c_int != 0 {
                    b"first\0".as_ptr() as *const i8
                } else {
                    b"last\0".as_ptr() as *const i8
                },
            );
        }
        ClearKeyInfo(from);
        InitKeyInfo((*info).ctx, from);
        return true_0 != 0;
    }
}
unsafe extern "C" fn XkbResolveKeyAlias(
    mut keymap: *const xkb_keymap,
    mut name: xkb_atom_t,
) -> xkb_atom_t {
    unsafe {
        if name < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
            let match_0: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(name as isize);
            if match_0.c2rust_unnamed.found() as ::core::ffi::c_int != 0
                && match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
            {
                return match_0.alias.real();
            }
        }
        return name;
    }
}
unsafe extern "C" fn AddKeySymbols(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        (*keyi).name = XkbResolveKeyAlias(&raw const (*(*info).keymap_info).keymap, (*keyi).name);
        let mut iter: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        if !(*info).keys.item.is_null() {
            iter = (*info).keys.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyInfo;
            while iter < (*info).keys.item.offset((*info).keys.size as isize) as *mut KeyInfo {
                if (*iter).name == (*keyi).name {
                    return MergeKeys(info, iter, keyi, same_file);
                }
                iter = iter.offset(1);
            }
        }
        (*info).keys.size = (*info).keys.size.wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = (*info).keys.size;
        if __need > (*info).keys.alloc {
            (*info).keys.alloc = darray_next_alloc(
                (*info).keys.alloc,
                __need,
                ::core::mem::size_of::<KeyInfo>() as usize,
            );
            (*info).keys.item = realloc(
                (*info).keys.item as *mut ::core::ffi::c_void,
                ((*info).keys.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<KeyInfo>() as usize),
            ) as *mut KeyInfo;
        }
        *(*info)
            .keys
            .item
            .offset((*info).keys.size.wrapping_sub(1 as darray_size_t) as isize) = *keyi;
        InitKeyInfo((*info).ctx, keyi);
        return true_0 != 0;
    }
}
unsafe extern "C" fn AddModMapEntry(mut info: *mut SymbolsInfo, mut new: *mut ModMapEntry) -> bool {
    unsafe {
        let mut old: *mut ModMapEntry = ::core::ptr::null_mut::<ModMapEntry>();
        let mut clobber: bool = (*new).merge as ::core::ffi::c_uint
            != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
        if !(*info).modmaps.item.is_null() {
            old = (*info)
                .modmaps
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut ModMapEntry;
            while old
                < (*info).modmaps.item.offset((*info).modmaps.size as isize) as *mut ModMapEntry
            {
                let mut use_0: xkb_mod_index_t = 0;
                let mut ignore: xkb_mod_index_t = 0;
                if (*new).haveSymbol as ::core::ffi::c_int
                    != (*old).haveSymbol as ::core::ffi::c_int
                    || (*new).haveSymbol as ::core::ffi::c_int != 0
                        && (*new).u.keySym != (*old).u.keySym
                    || !(*new).haveSymbol && (*new).u.keyName != (*old).u.keyName
                {
                    old = old.offset(1);
                } else {
                    if (*new).modifier == (*old).modifier {
                        return true_0 != 0;
                    }
                    use_0 = if clobber as ::core::ffi::c_int != 0 {
                        (*new).modifier
                    } else {
                        (*old).modifier
                    };
                    ignore = if clobber as ::core::ffi::c_int != 0 {
                        (*old).modifier
                    } else {
                        (*new).modifier
                    };
                    if (*new).haveSymbol {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Symbol \"%s\" added to modifier map for multiple modifiers; Using %s, ignoring %s\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_CONFLICTING_MODMAP as ::core::ffi::c_int,
                            KeysymText((*info).ctx, (*new).u.keySym),
                            ModIndexText((*info).ctx, &raw mut (*info).mods, use_0),
                            ModIndexText((*info).ctx, &raw mut (*info).mods, ignore),
                        );
                    } else {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Key \"%s\" added to modifier map for multiple modifiers; Using %s, ignoring %s\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_CONFLICTING_MODMAP as ::core::ffi::c_int,
                            KeyNameText((*info).ctx, (*new).u.keyName),
                            ModIndexText((*info).ctx, &raw mut (*info).mods, use_0),
                            ModIndexText((*info).ctx, &raw mut (*info).mods, ignore),
                        );
                    }
                    (*old).modifier = use_0;
                    return true_0 != 0;
                }
            }
        }
        (*info).modmaps.size = (*info).modmaps.size.wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = (*info).modmaps.size;
        if __need > (*info).modmaps.alloc {
            (*info).modmaps.alloc = darray_next_alloc(
                (*info).modmaps.alloc,
                __need,
                ::core::mem::size_of::<ModMapEntry>() as usize,
            );
            (*info).modmaps.item = realloc(
                (*info).modmaps.item as *mut ::core::ffi::c_void,
                ((*info).modmaps.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<ModMapEntry>() as usize),
            ) as *mut ModMapEntry;
        }
        *(*info)
            .modmaps
            .item
            .offset((*info).modmaps.size.wrapping_sub(1 as darray_size_t) as isize) = *new;
        return true_0 != 0;
    }
}
unsafe extern "C" fn MergeIncludedSymbols(
    mut into: *mut SymbolsInfo,
    mut from: *mut SymbolsInfo,
    mut merge: merge_mode,
) {
    unsafe {
        let mut group_name: *mut xkb_atom_t = ::core::ptr::null_mut::<xkb_atom_t>();
        let mut group_names_in_both: xkb_layout_index_t = 0;
        if (*from).errorCount > 0 as ::core::ffi::c_int {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        MergeModSets(
            (*into).ctx,
            &raw mut (*into).mods,
            &raw mut (*from).mods,
            merge,
        );
        if (*into).name.is_null() {
            (*into).name =
                _steal(&raw mut (*from).name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        }
        group_names_in_both = (if (*into).group_names.size < (*from).group_names.size {
            (*into).group_names.size
        } else {
            (*from).group_names.size
        }) as xkb_layout_index_t;
        let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while i < group_names_in_both {
            if !(*(*from).group_names.item.offset(i as isize) == 0) {
                if !(merge as ::core::ffi::c_uint
                    == MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint
                    && *(*into).group_names.item.offset(i as isize) != 0)
                {
                    *(*into).group_names.item.offset(i as isize) =
                        *(*from).group_names.item.offset(i as isize);
                }
            }
            i = i.wrapping_add(1);
        }
        if !(*from).group_names.item.is_null() {
            group_name = (*from)
                .group_names
                .item
                .offset(group_names_in_both as isize) as *mut xkb_atom_t;
            while group_name
                < (*from)
                    .group_names
                    .item
                    .offset((*from).group_names.size as isize) as *mut xkb_atom_t
            {
                (*into).group_names.size =
                    (*into).group_names.size.wrapping_add(1 as darray_size_t);
                let mut __need: darray_size_t = (*into).group_names.size;
                if __need > (*into).group_names.alloc {
                    (*into).group_names.alloc = darray_next_alloc(
                        (*into).group_names.alloc,
                        __need,
                        ::core::mem::size_of::<xkb_atom_t>() as usize,
                    );
                    (*into).group_names.item =
                        realloc(
                            (*into).group_names.item as *mut ::core::ffi::c_void,
                            ((*into).group_names.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as usize),
                        ) as *mut xkb_atom_t;
                }
                *(*into)
                    .group_names
                    .item
                    .offset((*into).group_names.size.wrapping_sub(1 as darray_size_t) as isize) =
                    *group_name;
                group_name = group_name.offset(1);
            }
        }
        if (*into).keys.size == 0 as darray_size_t {
            (*into).keys = (*from).keys;
            (*from).keys.item = ::core::ptr::null_mut::<KeyInfo>();
            (*from).keys.size = 0 as darray_size_t;
            (*from).keys.alloc = 0 as darray_size_t;
        } else {
            let mut keyi: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
            if !(*from).keys.item.is_null() {
                keyi = (*from).keys.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyInfo;
                while keyi < (*from).keys.item.offset((*from).keys.size as isize) as *mut KeyInfo {
                    (*keyi).set_merge(merge as merge_mode);
                    if !AddKeySymbols(into, keyi, false_0 != 0) {
                        (*into).errorCount += 1;
                    }
                    keyi = keyi.offset(1);
                }
            }
        }
        if (*into).modmaps.size == 0 as darray_size_t {
            (*into).modmaps = (*from).modmaps;
            (*from).modmaps.item = ::core::ptr::null_mut::<ModMapEntry>();
            (*from).modmaps.size = 0 as darray_size_t;
            (*from).modmaps.alloc = 0 as darray_size_t;
        } else {
            let mut mm: *mut ModMapEntry = ::core::ptr::null_mut::<ModMapEntry>();
            if !(*from).modmaps.item.is_null() {
                mm = (*from)
                    .modmaps
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ModMapEntry;
                while mm
                    < (*from).modmaps.item.offset((*from).modmaps.size as isize) as *mut ModMapEntry
                {
                    (*mm).merge = merge;
                    if !AddModMapEntry(into, mm) {
                        (*into).errorCount += 1;
                    }
                    mm = mm.offset(1);
                }
            }
        };
    }
}
unsafe extern "C" fn HandleIncludeSymbols(
    mut info: *mut SymbolsInfo,
    mut include: *mut IncludeStmt,
) -> bool {
    unsafe {
        let mut included: SymbolsInfo = SymbolsInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: C2Rust_Unnamed_24 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<KeyInfo>(),
            },
            default_key: KeyInfo {
                name: 0,
                vmodmap: 0,
                default_type: 0,
                out_of_range_group_number: 0,
                groups: C2Rust_Unnamed_22 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<GroupInfo>(),
                },
                out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                overlays_alloc: 0,
                overlays: 0,
                c2rust_unnamed: C2Rust_Unnamed_21 {
                    overlay_key: ::core::ptr::null::<xkb_key>(),
                },
            },
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            group_names: C2Rust_Unnamed_20 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<xkb_atom_t>(),
            },
            modmaps: C2Rust_Unnamed_18 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<ModMapEntry>(),
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as ::core::ffi::c_int;
            return false_0 != 0;
        }
        InitSymbolsInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as ::core::ffi::c_uint),
            &raw mut (*info).mods,
        );
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: SymbolsInfo = SymbolsInfo {
                name: ::core::ptr::null_mut::<i8>(),
                errorCount: 0,
                include_depth: 0,
                explicit_group: 0,
                max_groups: 0,
                keys: C2Rust_Unnamed_24 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<KeyInfo>(),
                },
                default_key: KeyInfo {
                    name: 0,
                    vmodmap: 0,
                    default_type: 0,
                    out_of_range_group_number: 0,
                    groups: C2Rust_Unnamed_22 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<GroupInfo>(),
                    },
                    out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                    overlays_alloc: 0,
                    overlays: 0,
                    c2rust_unnamed: C2Rust_Unnamed_21 {
                        overlay_key: ::core::ptr::null::<xkb_key>(),
                    },
                },
                default_actions: ActionsInfo {
                    actions: [xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    }; 21],
                },
                group_names: C2Rust_Unnamed_20 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_atom_t>(),
                },
                modmaps: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<ModMapEntry>(),
                },
                mods: xkb_mod_set {
                    mods: [xkb_mod {
                        name: 0,
                        type_0: 0 as mod_type,
                        mapping: 0,
                    }; 32],
                    num_mods: 0,
                    explicit_vmods: 0,
                },
                ctx: ::core::ptr::null_mut::<xkb_context>(),
                keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            };
            let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_SYMBOLS,
                &raw mut path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
            );
            if file.is_null() {
                (*info).errorCount += 10 as ::core::ffi::c_int;
                ClearSymbolsInfo(&raw mut included);
                return false_0 != 0;
            }
            InitSymbolsInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as ::core::ffi::c_uint),
                &raw mut included.mods,
            );
            if !(*stmt).modifier.is_null() {
                next_incl.explicit_group =
                    (atoi((*stmt).modifier) - 1 as ::core::ffi::c_int) as xkb_layout_index_t;
                if next_incl.explicit_group >= (*info).max_groups {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Cannot set explicit group to %u - must be between 1..%u; Ignoring group number\n\0"
                            .as_ptr() as *const i8,
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                        next_incl.explicit_group.wrapping_add(1 as xkb_layout_index_t),
                        (*info).max_groups,
                    );
                    next_incl.explicit_group = (*info).explicit_group;
                }
            } else if (*(*info).keymap_info).keymap.num_groups != 0 as xkb_layout_index_t
                && next_incl.include_depth == 1 as ::core::ffi::c_uint
            {
                next_incl.explicit_group = 0 as xkb_layout_index_t;
            } else {
                next_incl.explicit_group = (*info).explicit_group;
            }
            HandleSymbolsFile(&raw mut next_incl, file);
            MergeIncludedSymbols(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearSymbolsInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedSymbols(info, &raw mut included, (*include).merge);
        ClearSymbolsInfo(&raw mut included);
        return (*info).errorCount == 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn GetGroupIndex(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut field: group_field,
    mut ndx_rtrn: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut name: *const i8 = if field as ::core::ffi::c_uint
            == GROUP_FIELD_SYMS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            b"symbols\0".as_ptr() as *const i8
        } else {
            b"actions\0".as_ptr() as *const i8
        };
        if arrayNdx.is_null() {
            let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
            let mut groupi: *mut GroupInfo = ::core::ptr::null_mut::<GroupInfo>();
            if !(*keyi).groups.item.is_null() {
                i = 0 as xkb_layout_index_t;
                groupi =
                    (*keyi).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut GroupInfo;
                while i < (*keyi).groups.size as xkb_layout_index_t {
                    if (*groupi).defined as ::core::ffi::c_uint & field as ::core::ffi::c_uint == 0
                    {
                        *ndx_rtrn = i;
                        return true_0 != 0;
                    }
                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }
            if i >= (*info).max_groups {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Too many groups of %s for key %s (max %u); Ignoring %s defined for extra groups\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                    name,
                    KeyInfoText(info, keyi),
                    (*info).max_groups,
                    name,
                );
                return false_0 != 0;
            }
            let mut __oldSize: darray_size_t = (*keyi).groups.size;
            let mut __newSize: darray_size_t = (*keyi).groups.size.wrapping_add(1 as darray_size_t);
            (*keyi).groups.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*keyi).groups.alloc {
                    (*keyi).groups.alloc = darray_next_alloc(
                        (*keyi).groups.alloc,
                        __need,
                        ::core::mem::size_of::<GroupInfo>() as usize,
                    );
                    (*keyi).groups.item = realloc(
                        (*keyi).groups.item as *mut ::core::ffi::c_void,
                        ((*keyi).groups.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                    ) as *mut GroupInfo;
                }
                memset(
                    (*keyi).groups.item.offset(__oldSize as isize) as *mut GroupInfo
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as usize)
                        .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                );
            }
            *ndx_rtrn = (*keyi).groups.size.wrapping_sub(1 as darray_size_t) as xkb_layout_index_t;
            return true_0 != 0;
        }
        if ExprResolveGroup(
            (*info).keymap_info,
            arrayNdx,
            false_0 != 0,
            ndx_rtrn,
            ::core::ptr::null_mut::<bool>(),
        ) as ::core::ffi::c_uint
            != PARSER_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Illegal group index for %s of key %s\nDefinition with non-integer array index ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                name,
                KeyInfoText(info, keyi),
            );
            return false_0 != 0;
        }
        *ndx_rtrn = (*ndx_rtrn).wrapping_sub(1);
        if *ndx_rtrn >= (*keyi).groups.size as xkb_layout_index_t {
            let mut __oldSize_0: darray_size_t = (*keyi).groups.size;
            let mut __newSize_0: darray_size_t = (*ndx_rtrn).wrapping_add(1 as darray_size_t);
            (*keyi).groups.size = __newSize_0;
            if __newSize_0 > __oldSize_0 {
                let mut __need_0: darray_size_t = __newSize_0;
                if __need_0 > (*keyi).groups.alloc {
                    (*keyi).groups.alloc = darray_next_alloc(
                        (*keyi).groups.alloc,
                        __need_0,
                        ::core::mem::size_of::<GroupInfo>() as usize,
                    );
                    (*keyi).groups.item = realloc(
                        (*keyi).groups.item as *mut ::core::ffi::c_void,
                        ((*keyi).groups.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                    ) as *mut GroupInfo;
                }
                memset(
                    (*keyi).groups.item.offset(__oldSize_0 as isize) as *mut GroupInfo
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize_0.wrapping_sub(__oldSize_0) as usize)
                        .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                );
            }
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn AddSymbolsToKey(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ndx: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if !GetGroupIndex(info, keyi, arrayNdx, GROUP_FIELD_SYMS, &raw mut ndx) {
            return false_0 != 0;
        }
        let mut groupi: *mut GroupInfo = (*keyi).groups.item.offset(ndx as isize) as *mut GroupInfo;
        if (*value).common.type_0 as ::core::ffi::c_uint
            == STMT_EXPR_EMPTY_LIST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*groupi).defined = ((*groupi).defined as ::core::ffi::c_uint
                | GROUP_FIELD_SYMS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as group_field;
            return true_0 != 0;
        }
        if (*value).common.type_0 as ::core::ffi::c_uint
            != STMT_EXPR_KEYSYM_LIST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Expected a list of symbols, found %s; Ignoring symbols for group %u of %s\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                stmt_type_to_string((*value).common.type_0),
                ndx.wrapping_add(1 as xkb_layout_index_t),
                KeyInfoText(info, keyi),
            );
            return false_0 != 0;
        }
        if (*groupi).defined as ::core::ffi::c_uint
            & GROUP_FIELD_SYMS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Symbols for key %s, group %u already defined; Ignoring duplicate definition\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY as ::core::ffi::c_int,
                KeyInfoText(info, keyi),
                ndx.wrapping_add(1 as xkb_layout_index_t),
            );
            return false_0 != 0;
        }
        let mut nLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut nonEmptyLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut keysymList: *mut ExprKeysymList = value as *mut ExprKeysymList;
        while !keysymList.is_null() {
            nLevels = nLevels.wrapping_add(1);
            if (*keysymList).syms.size > 0 as darray_size_t {
                nonEmptyLevels = nLevels;
            }
            keysymList = (*keysymList).common.next as *mut ExprKeysymList;
        }
        if nonEmptyLevels < nLevels {
            nLevels = nonEmptyLevels;
        }
        if ((*groupi).levels.size as xkb_level_index_t) < nLevels {
            let mut __oldSize: darray_size_t = (*groupi).levels.size;
            let mut __newSize: darray_size_t = nLevels as darray_size_t;
            (*groupi).levels.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*groupi).levels.alloc {
                    (*groupi).levels.alloc = darray_next_alloc(
                        (*groupi).levels.alloc,
                        __need,
                        ::core::mem::size_of::<xkb_level>() as usize,
                    );
                    (*groupi).levels.item = realloc(
                        (*groupi).levels.item as *mut ::core::ffi::c_void,
                        ((*groupi).levels.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                    ) as *mut xkb_level;
                }
                memset(
                    (*groupi).levels.item.offset(__oldSize as isize) as *mut xkb_level
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                );
            }
        }
        (*groupi).defined = ((*groupi).defined as ::core::ffi::c_uint
            | GROUP_FIELD_SYMS as ::core::ffi::c_int as ::core::ffi::c_uint)
            as group_field;
        let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut keysymList_0: *mut ExprKeysymList = value as *mut ExprKeysymList;
        while !keysymList_0.is_null() && level < nLevels {
            let mut leveli: *mut xkb_level =
                (*groupi).levels.item.offset(level as isize) as *mut xkb_level;
            if ((*keysymList_0).syms.size > 65535 as darray_size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key %s has too many keysyms for group %u, level %u; expected max %u, got: %u\n\0"
                        .as_ptr() as *const i8,
                    KeyInfoText(info, keyi),
                    ndx.wrapping_add(1 as xkb_layout_index_t),
                    level.wrapping_add(1 as xkb_level_index_t),
                    65535 as ::core::ffi::c_int,
                    (*keysymList_0).syms.size,
                );
                return false_0 != 0;
            }
            (*leveli).num_syms = (*keysymList_0).syms.size as xkb_keysym_count_t;
            match (*leveli).num_syms as ::core::ffi::c_int {
                0 => {
                    (*leveli).s.sym = XKB_KEY_NoSymbol as xkb_keysym_t;
                }
                1 => {
                    (*leveli).s.sym = *(*keysymList_0)
                        .syms
                        .item
                        .offset(0 as ::core::ffi::c_int as isize);
                }
                _ => {
                    if (*keysymList_0).syms.size > 0 as darray_size_t {
                        (*keysymList_0).syms.alloc = (*keysymList_0).syms.size;
                        (*keysymList_0).syms.item = realloc(
                            (*keysymList_0).syms.item as *mut ::core::ffi::c_void,
                            ((*keysymList_0).syms.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<xkb_keysym_t>() as usize),
                        ) as *mut xkb_keysym_t;
                    }
                    (*leveli).s.syms = (*keysymList_0).syms.item;
                    if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                        *(::core::ptr::null_mut::<::core::ffi::c_void>() as *mut darray_size_t) =
                            (*keysymList_0).syms.size;
                    }
                    (*keysymList_0).syms.item = ::core::ptr::null_mut::<xkb_keysym_t>();
                    (*keysymList_0).syms.size = 0 as darray_size_t;
                    (*keysymList_0).syms.alloc = 0 as darray_size_t;
                    let mut k: xkb_keysym_count_t = 0 as xkb_keysym_count_t;
                    while (k as ::core::ffi::c_int) < (*leveli).num_syms as ::core::ffi::c_int {
                        k = k.wrapping_add(1);
                    }
                }
            }
            keysymList_0 = (*keysymList_0).common.next as *mut ExprKeysymList;
            level = level.wrapping_add(1);
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn AddActionsToKey(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ndx: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if !GetGroupIndex(info, keyi, arrayNdx, GROUP_FIELD_ACTS, &raw mut ndx) {
            return false_0 != 0;
        }
        let mut groupi: *mut GroupInfo = (*keyi).groups.item.offset(ndx as isize) as *mut GroupInfo;
        if (*value).common.type_0 as ::core::ffi::c_uint
            == STMT_EXPR_EMPTY_LIST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*groupi).defined = ((*groupi).defined as ::core::ffi::c_uint
                | GROUP_FIELD_ACTS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as group_field;
            return true_0 != 0;
        }
        if (*value).common.type_0 as ::core::ffi::c_uint
            != STMT_EXPR_ACTION_LIST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_CRITICAL,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Bad expression type (%d) for action list value; Ignoring actions for group %u of %s\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_INVALID_EXPRESSION_TYPE as ::core::ffi::c_int,
                (*value).common.type_0 as ::core::ffi::c_uint,
                ndx,
                KeyInfoText(info, keyi),
            );
            return false_0 != 0;
        }
        if (*groupi).defined as ::core::ffi::c_uint
            & GROUP_FIELD_ACTS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_CRITICAL,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Actions for key %s, group %u already defined\n\0".as_ptr()
                    as *const i8,
                XKB_WARNING_CONFLICTING_KEY_ACTION as ::core::ffi::c_int,
                KeyInfoText(info, keyi),
                ndx,
            );
            return false_0 != 0;
        }
        let mut nLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut p: *mut ParseCommon = &raw mut (*value).common;
        while !p.is_null() {
            nLevels = nLevels.wrapping_add(1);
            p = (*p).next as *mut ParseCommon;
        }
        if ((*groupi).levels.size as xkb_level_index_t) < nLevels {
            let mut __oldSize: darray_size_t = (*groupi).levels.size;
            let mut __newSize: darray_size_t = nLevels as darray_size_t;
            (*groupi).levels.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*groupi).levels.alloc {
                    (*groupi).levels.alloc = darray_next_alloc(
                        (*groupi).levels.alloc,
                        __need,
                        ::core::mem::size_of::<xkb_level>() as usize,
                    );
                    (*groupi).levels.item = realloc(
                        (*groupi).levels.item as *mut ::core::ffi::c_void,
                        ((*groupi).levels.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                    ) as *mut xkb_level;
                }
                memset(
                    (*groupi).levels.item.offset(__oldSize as isize) as *mut xkb_level
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                );
            }
        }
        (*groupi).defined = ((*groupi).defined as ::core::ffi::c_uint
            | GROUP_FIELD_ACTS as ::core::ffi::c_int as ::core::ffi::c_uint)
            as group_field;
        let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut nonEmptyLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut actionList: *mut ExprActionList = value as *mut ExprActionList;
        while !actionList.is_null() {
            let mut c2rust_current_block_102: u64;
            let mut leveli: *mut xkb_level =
                (*groupi).levels.item.offset(level as isize) as *mut xkb_level;
            let mut num_actions: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut act: *mut ExprDef = (*actionList).actions as *mut ExprDef;
            while !act.is_null() {
                num_actions = num_actions.wrapping_add(1);
                act = (*act).common.next as *mut ExprDef;
            }
            if (num_actions > 65535 as ::core::ffi::c_uint) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key %s has too many actions for group %u, level %u; expected max %u, got: %u\n\0"
                        .as_ptr() as *const i8,
                    KeyInfoText(info, keyi),
                    ndx.wrapping_add(1 as xkb_layout_index_t),
                    level.wrapping_add(1 as xkb_level_index_t),
                    65535 as ::core::ffi::c_int,
                    num_actions,
                );
                return false_0 != 0;
            }
            let mut actions: C2Rust_Unnamed_25 = C2Rust_Unnamed_25 {
                size: 0 as darray_size_t,
                alloc: 0 as darray_size_t,
                item: ::core::ptr::null_mut::<xkb_action>(),
            };
            let mut act_0: *mut ExprDef = (*actionList).actions as *mut ExprDef;
            loop {
                if act_0.is_null() {
                    c2rust_current_block_102 = 1134115459065347084;
                    break;
                }
                let mut toAct: xkb_action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                let r: xkb_parser_error = HandleActionDef(
                    (*info).keymap_info,
                    &raw mut (*info).default_actions,
                    &raw mut (*info).mods,
                    act_0,
                    &raw mut toAct,
                ) as xkb_parser_error;
                if r as ::core::ffi::c_uint
                    != PARSER_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Illegal action definition for %s; Action for group %u/level %u ignored\n\0"
                            .as_ptr() as *const i8,
                        XKB_ERROR_INVALID_VALUE as ::core::ffi::c_int,
                        KeyInfoText(info, keyi),
                        ndx.wrapping_add(1 as xkb_layout_index_t),
                        level.wrapping_add(1 as xkb_level_index_t),
                    );
                    if r as ::core::ffi::c_uint
                        == PARSER_FATAL_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        free(actions.item as *mut ::core::ffi::c_void);
                        actions.item = ::core::ptr::null_mut::<xkb_action>();
                        actions.size = 0 as darray_size_t;
                        actions.alloc = 0 as darray_size_t;
                        return false_0 != 0;
                    } else {
                        toAct.type_0 = ACTION_TYPE_NONE;
                    }
                }
                if !(toAct.type_0 as ::core::ffi::c_uint
                    == ACTION_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if (num_actions == 1 as ::core::ffi::c_uint) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        (*leveli).num_actions = 1 as xkb_action_count_t;
                        (*leveli).a.action = toAct;
                        c2rust_current_block_102 = 1829140360157350833;
                        break;
                    } else {
                        actions.size = actions.size.wrapping_add(1 as darray_size_t);
                        let mut __need_0: darray_size_t = actions.size;
                        if __need_0 > actions.alloc {
                            actions.alloc = darray_next_alloc(
                                actions.alloc,
                                __need_0,
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
                        *actions
                            .item
                            .offset(actions.size.wrapping_sub(1 as darray_size_t) as isize) = toAct;
                    }
                }
                act_0 = (*act_0).common.next as *mut ExprDef;
            }
            match c2rust_current_block_102 {
                1134115459065347084 => {
                    if actions.size == 0 as darray_size_t {
                        (*leveli).num_actions = 0 as xkb_action_count_t;
                    } else if (actions.size > 1 as darray_size_t) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        (*leveli).num_actions = actions.size as xkb_action_count_t;
                        if actions.size > 0 as darray_size_t {
                            actions.alloc = actions.size;
                            actions.item = realloc(
                                actions.item as *mut ::core::ffi::c_void,
                                (actions.alloc as usize).wrapping_mul(::core::mem::size_of::<
                                    xkb_action,
                                >(
                                )
                                    as usize),
                            ) as *mut xkb_action;
                        }
                        (*leveli).a.actions = actions.item;
                        if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                            *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                as *mut darray_size_t) = actions.size;
                        }
                        actions.item = ::core::ptr::null_mut::<xkb_action>();
                        actions.size = 0 as darray_size_t;
                        actions.alloc = 0 as darray_size_t;
                        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
                        while (k as ::core::ffi::c_int)
                            < (*leveli).num_actions as ::core::ffi::c_int
                        {
                            k = k.wrapping_add(1);
                        }
                    } else {
                        (*leveli).num_actions = 1 as xkb_action_count_t;
                        (*leveli).a.action = *actions.item.offset(0 as ::core::ffi::c_int as isize);
                        free(actions.item as *mut ::core::ffi::c_void);
                        actions.item = ::core::ptr::null_mut::<xkb_action>();
                        actions.size = 0 as darray_size_t;
                        actions.alloc = 0 as darray_size_t;
                    }
                }
                _ => {}
            }
            if (*leveli).num_actions as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                || (*leveli).num_syms as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                nonEmptyLevels = level.wrapping_add(1 as xkb_level_index_t);
            }
            actionList = (*actionList).common.next as *mut ExprActionList;
            level = level.wrapping_add(1);
        }
        if nonEmptyLevels < nLevels {
            (*groupi).levels.size = nonEmptyLevels as darray_size_t;
            if nonEmptyLevels > 0 as xkb_level_index_t {
                if (*groupi).levels.size > 0 as darray_size_t {
                    (*groupi).levels.alloc = (*groupi).levels.size;
                    (*groupi).levels.item = realloc(
                        (*groupi).levels.item as *mut ::core::ffi::c_void,
                        ((*groupi).levels.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                    ) as *mut xkb_level;
                }
            } else {
                free((*groupi).levels.item as *mut ::core::ffi::c_void);
                (*groupi).levels.item = ::core::ptr::null_mut::<xkb_level>();
                (*groupi).levels.size = 0 as darray_size_t;
                (*groupi).levels.alloc = 0 as darray_size_t;
            }
        }
        return true_0 != 0;
    }
}
static mut repeatEntries: [LookupEntry; 8] = [
    LookupEntry {
        name: b"true\0".as_ptr() as *const i8,
        value: KEY_REPEAT_YES as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"yes\0".as_ptr() as *const i8,
        value: KEY_REPEAT_YES as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"on\0".as_ptr() as *const i8,
        value: KEY_REPEAT_YES as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"false\0".as_ptr() as *const i8,
        value: KEY_REPEAT_NO as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"no\0".as_ptr() as *const i8,
        value: KEY_REPEAT_NO as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"off\0".as_ptr() as *const i8,
        value: KEY_REPEAT_NO as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: b"default\0".as_ptr() as *const i8,
        value: KEY_REPEAT_UNDEFINED as ::core::ffi::c_int as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe extern "C" fn ExprResolveOverlayEntry(
    mut keymap_info: *const xkb_keymap_info,
    mut field: *const i8,
    mut arrayNdx: *const ExprDef,
    mut expr: *const ExprDef,
    mut keyi: *mut KeyInfo,
    mut overlay_rtrn: *mut xkb_overlay_index_t,
    mut key_rtrn: *mut *const xkb_key,
) -> bool {
    unsafe {
        if !arrayNdx.is_null() {
            xkb_log(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Overlay field \"%s\" in %s does not support array index; ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                field,
                KeyNameText((*keymap_info).keymap.ctx, (*keyi).name),
            );
            return false_0 != 0;
        }
        let prefix: usize = (::core::mem::size_of::<[i8; 8]>() as usize).wrapping_sub(1 as usize);
        let len: usize = strlen(field.offset(prefix as isize)) as usize;
        let mut raw_overlay: int64_t = XKB_OVERLAY_INVALID as int64_t;
        if parse_dec_to_uint64_t(
            field.offset(prefix as isize),
            len,
            &raw mut raw_overlay as *mut uint64_t,
        ) != len as ::core::ffi::c_int
            || raw_overlay < 1 as int64_t
            || raw_overlay > (*keymap_info).features.max_overlays as int64_t
        {
            xkb_log(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Unsupported overlay index \"%s\" field for %s: expected 1..%u, got: %ld; ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX as ::core::ffi::c_int,
                field,
                KeyNameText((*keymap_info).keymap.ctx, (*keyi).name),
                (*keymap_info).features.max_overlays as ::core::ffi::c_int,
                raw_overlay,
            );
            return false_0 != 0;
        }
        *overlay_rtrn = (raw_overlay as xkb_overlay_index_t as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as xkb_overlay_index_t;
        match (*expr).common.type_0 as ::core::ffi::c_uint {
            8 => {
                *key_rtrn = XkbKeyByName(
                    &raw const (*keymap_info).keymap,
                    (*expr).key_name.key_name,
                    false_0 != 0,
                );
                if (*key_rtrn).is_null() {
                    xkb_log(
                        (*keymap_info).keymap.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Unknown key \"%s\" for field %s in %s\n\0".as_ptr()
                            as *const i8,
                        XKB_WARNING_UNDEFINED_KEYCODE as ::core::ffi::c_int,
                        xkb_atom_text((*keymap_info).keymap.ctx, (*expr).key_name.key_name),
                        field,
                        KeyNameText((*keymap_info).keymap.ctx, (*keyi).name),
                    );
                    return false_0 != 0;
                }
                return true_0 != 0;
            }
            10 => {
                let id: *const i8 =
                    xkb_atom_text((*keymap_info).keymap.ctx, (*expr).ident.ident) as *const i8;
                if !id.is_null()
                    && istreq(id, b"none\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                {
                    *key_rtrn = ::core::ptr::null::<xkb_key>();
                    return true_0 != 0;
                } else if !id.is_null()
                    && istreq(id, b"any\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                {
                    *key_rtrn = ::core::ptr::null::<xkb_key>();
                    *overlay_rtrn = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
                    return true_0 != 0;
                }
                xkb_log(
                    (*keymap_info).keymap.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Unsupported overlay value \"%s\" for field %s in %s\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_VALUE as ::core::ffi::c_int,
                    id,
                    field,
                    KeyNameText((*keymap_info).keymap.ctx, (*keyi).name),
                );
                return false_0 != 0;
            }
            _ => {
                xkb_log(
                    (*keymap_info).keymap.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Expected %s for field \"%s\" in %s, got: %s\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_VALUE as ::core::ffi::c_int,
                    stmt_type_to_string(STMT_EXPR_KEYNAME_LITERAL),
                    field,
                    KeyNameText((*keymap_info).keymap.ctx, (*keyi).name),
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false_0 != 0;
            }
        };
    }
}
unsafe extern "C" fn SetSymbolsField(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut field: *const i8,
    mut arrayNdx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if istreq(field, b"type\0".as_ptr() as *const i8) {
            let mut ndx: xkb_layout_index_t = 0 as xkb_layout_index_t;
            let mut val: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
            if !ExprResolveString((*info).ctx, value, &raw mut val) {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] The type field of a key symbol map must be a string; Ignoring illegal type definition\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                );
                return false_0 != 0;
            }
            if arrayNdx.is_null() {
                (*keyi).default_type = val;
                (*keyi).set_defined(
                    (*keyi).defined() | KEY_FIELD_DEFAULT_TYPE as ::core::ffi::c_int as key_field,
                );
            } else if ExprResolveGroup(
                (*info).keymap_info,
                arrayNdx,
                false_0 != 0,
                &raw mut ndx,
                ::core::ptr::null_mut::<bool>(),
            ) as ::core::ffi::c_uint
                != PARSER_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal group index for type of key %s; Definition with non-integer array index ignored\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                    KeyInfoText(info, keyi),
                );
                return false_0 != 0;
            } else {
                ndx = ndx.wrapping_sub(1);
                if ndx >= (*keyi).groups.size as xkb_layout_index_t {
                    let mut __oldSize: darray_size_t = (*keyi).groups.size;
                    let mut __newSize: darray_size_t =
                        (ndx as darray_size_t).wrapping_add(1 as darray_size_t);
                    (*keyi).groups.size = __newSize;
                    if __newSize > __oldSize {
                        let mut __need: darray_size_t = __newSize;
                        if __need > (*keyi).groups.alloc {
                            (*keyi).groups.alloc = darray_next_alloc(
                                (*keyi).groups.alloc,
                                __need,
                                ::core::mem::size_of::<GroupInfo>() as usize,
                            );
                            (*keyi).groups.item = realloc(
                                (*keyi).groups.item as *mut ::core::ffi::c_void,
                                ((*keyi).groups.alloc as usize)
                                    .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                            ) as *mut GroupInfo;
                        }
                        memset(
                            (*keyi).groups.item.offset(__oldSize as isize) as *mut GroupInfo
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (__newSize.wrapping_sub(__oldSize) as usize)
                                .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                        );
                    }
                }
                (*(*keyi).groups.item.offset(ndx as isize)).type_0 = val;
                let ref mut c2rust_fresh8 = (*(*keyi).groups.item.offset(ndx as isize)).defined;
                *c2rust_fresh8 = (*c2rust_fresh8 as ::core::ffi::c_uint
                    | GROUP_FIELD_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as group_field;
            }
        } else if istreq(field, b"symbols\0".as_ptr() as *const i8) {
            return AddSymbolsToKey(info, keyi, arrayNdx, value);
        } else if istreq(field, b"actions\0".as_ptr() as *const i8) {
            return AddActionsToKey(info, keyi, arrayNdx, value);
        } else if istreq(field, b"vmods\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"virtualmods\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"virtualmodifiers\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            let mut mask: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
            if !ExprResolveModMask(
                (*info).ctx,
                value,
                MOD_VIRT,
                &raw mut (*info).mods,
                &raw mut mask,
            ) {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Expected a virtual modifier mask, found %s; Ignoring virtual modifiers definition for key %s\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as ::core::ffi::c_int,
                    stmt_type_to_string((*value).common.type_0),
                    KeyInfoText(info, keyi),
                );
                return false_0 != 0;
            }
            (*keyi).vmodmap = mask;
            (*keyi).set_defined(
                (*keyi).defined() | KEY_FIELD_VMODMAP as ::core::ffi::c_int as key_field,
            );
        } else if istreq(field, b"locking\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"lock\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"locks\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                b"[XKB-%03d] Key behaviors not supported; Ignoring locking specification for key %s\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
                KeyInfoText(info, keyi),
            );
        } else if istreq(field, b"radiogroup\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"permanentradiogroup\0".as_ptr() as *const i8) as ::core::ffi::c_int
                != 0
            || istreq(field, b"allownone\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                b"[XKB-%03d] Radio groups not supported; Ignoring radio group specification for key %s\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
                KeyInfoText(info, keyi),
            );
        } else if istrneq(
            b"permanentoverlay\0".as_ptr() as *const i8,
            field,
            (::core::mem::size_of::<[i8; 17]>() as usize).wrapping_sub(1 as usize),
        ) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                b"[XKB-%03d] Permanent overlays not supported; Ignoring overlay specification for key %s\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
                KeyInfoText(info, keyi),
            );
        } else if istrneq(
            b"overlay\0".as_ptr() as *const i8,
            field,
            (::core::mem::size_of::<[i8; 8]>() as usize).wrapping_sub(1 as usize),
        ) {
            let mut overlay: xkb_overlay_index_t = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
            let mut key: *const xkb_key = ::core::ptr::null::<xkb_key>();
            if !ExprResolveOverlayEntry(
                (*info).keymap_info,
                field,
                arrayNdx,
                *value_ptr,
                keyi,
                &raw mut overlay,
                &raw mut key,
            ) {
                return false_0 != 0;
            }
            if overlay as ::core::ffi::c_int == XKB_OVERLAY_INVALID {
                return true_0 != 0;
            } else if !key.is_null() && (*key).name == (*keyi).name {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                    b"Cannot overlay a key to itself; Ignoring overlay %u specification for key %s\n\0"
                        .as_ptr() as *const i8,
                    overlay as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                    KeyInfoText(info, keyi),
                );
            } else {
                let mut prev: *const xkb_key = ::core::ptr::null::<xkb_key>();
                if overlays_get(keyi, overlay, &raw mut prev) {
                    if key != prev {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Conflicting overlays defined in key %s; use overlay%d=%s, ignore overlay%u=%s\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_CONFLICTING_KEY_FIELDS as ::core::ffi::c_int,
                            KeyInfoText(info, keyi),
                            overlay as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                            if !prev.is_null() {
                                KeyNameText((*info).ctx, (*prev).name)
                            } else {
                                b"none\0".as_ptr() as *const i8
                            },
                            overlay as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                            if !key.is_null() {
                                KeyNameText((*info).ctx, (*key).name)
                            } else {
                                b"none\0".as_ptr() as *const i8
                            },
                        );
                    }
                } else if (*(*info).keymap_info).features.overlapping_overlays {
                    if !overlays_insert(keyi, overlay, key) {
                        return false_0 != 0;
                    }
                    (*keyi).set_defined(
                        (*keyi).defined() | KEY_FIELD_OVERLAY as ::core::ffi::c_int as key_field,
                    );
                } else {
                    let mask_0: xkb_overlay_mask_t = ((1 as ::core::ffi::c_uint)
                        << overlay as ::core::ffi::c_int)
                        as xkb_overlay_mask_t;
                    if (*keyi).overlays == 0 || (*keyi).overlays_clear() as ::core::ffi::c_int != 0
                    {
                        if !key.is_null() {
                            (*keyi).overlays = mask_0;
                            (*keyi).set_overlays_clear((false_0 != 0) as bool);
                            (*keyi).c2rust_unnamed.overlay_key = key;
                        } else {
                            (*keyi).overlays = ((*keyi).overlays as ::core::ffi::c_int
                                | mask_0 as ::core::ffi::c_int)
                                as xkb_overlay_mask_t;
                            (*keyi).set_overlays_clear((true_0 != 0) as bool);
                        }
                        (*keyi).set_defined(
                            (*keyi).defined()
                                | KEY_FIELD_OVERLAY as ::core::ffi::c_int as key_field,
                        );
                    } else if (*keyi).overlays != 0 {
                        if !key.is_null() {
                            xkb_log(
                                (*info).ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] Overlapping overlays are not allowed in %s; use overlay%d=%s, ignore overlay%u=%s\n\0"
                                    .as_ptr() as *const i8,
                                XKB_ERROR_OVERLAPPING_OVERLAY as ::core::ffi::c_int,
                                KeyInfoText(info, keyi),
                                (*keyi).overlays as ::core::ffi::c_int,
                                KeyNameText(
                                    (*info).ctx,
                                    (*(*keyi).c2rust_unnamed.overlay_key).name,
                                ),
                                overlay as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                KeyNameText((*info).ctx, (*key).name),
                            );
                            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                                & PARSER_NO_FIELD_VALUE_MISMATCH as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                == 0;
                        }
                    }
                }
            }
        } else if istreq(field, b"repeating\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"repeats\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"repeat\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            let mut val_0: u32 = 0 as u32;
            if !ExprResolveEnum(
                (*info).ctx,
                value,
                &raw mut val_0,
                &raw const repeatEntries as *const LookupEntry,
            ) {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal repeat setting for %s; Non-boolean repeat setting ignored\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_INVALID_VALUE as ::core::ffi::c_int,
                    KeyInfoText(info, keyi),
                );
                return false_0 != 0;
            }
            (*keyi).set_repeat(val_0 as key_repeat as key_repeat);
            (*keyi).set_defined(
                (*keyi).defined() | KEY_FIELD_REPEAT as ::core::ffi::c_int as key_field,
            );
        } else if istreq(field, b"groupswrap\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"wrapgroups\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            let mut set: bool = false_0 != 0;
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set) {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal groupsWrap setting for %s; Non-boolean value ignored\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_INVALID_VALUE as ::core::ffi::c_int,
                    KeyInfoText(info, keyi),
                );
                return false_0 != 0;
            }
            (*keyi).set_out_of_range_group_policy(
                (if set as ::core::ffi::c_int != 0 {
                    XKB_LAYOUT_OUT_OF_RANGE_WRAP as ::core::ffi::c_int
                } else {
                    XKB_LAYOUT_OUT_OF_RANGE_CLAMP as ::core::ffi::c_int
                }) as xkb_layout_out_of_range_policy
                    as xkb_layout_out_of_range_policy,
            );
            (*keyi).set_defined(
                (*keyi).defined() | KEY_FIELD_GROUPINFO as ::core::ffi::c_int as key_field,
            );
        } else if istreq(field, b"groupsclamp\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"clampgroups\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            let mut set_0: bool = false_0 != 0;
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set_0) {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal groupsClamp setting for %s; Non-boolean value ignored\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_INVALID_VALUE as ::core::ffi::c_int,
                    KeyInfoText(info, keyi),
                );
                return false_0 != 0;
            }
            (*keyi).set_out_of_range_group_policy(
                (if set_0 as ::core::ffi::c_int != 0 {
                    XKB_LAYOUT_OUT_OF_RANGE_CLAMP as ::core::ffi::c_int
                } else {
                    XKB_LAYOUT_OUT_OF_RANGE_WRAP as ::core::ffi::c_int
                }) as xkb_layout_out_of_range_policy
                    as xkb_layout_out_of_range_policy,
            );
            (*keyi).set_defined(
                (*keyi).defined() | KEY_FIELD_GROUPINFO as ::core::ffi::c_int as key_field,
            );
        } else if istreq(field, b"groupsredirect\0".as_ptr() as *const i8) as ::core::ffi::c_int
            != 0
            || istreq(field, b"redirectgroups\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            let mut grp: xkb_layout_index_t = 0 as xkb_layout_index_t;
            let mut pending: bool = false_0 != 0;
            if ExprResolveGroup(
                (*info).keymap_info,
                value,
                false_0 != 0,
                &raw mut grp,
                &raw mut pending,
            ) as ::core::ffi::c_uint
                != PARSER_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
                && !pending
            {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal group index for redirect of key %s; Definition with non-integer group ignored\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                    KeyInfoText(info, keyi),
                );
                return false_0 != 0;
            }
            if pending {
                (*keyi).set_out_of_range_pending_group((true_0 != 0) as bool);
                let pending_index: darray_size_t =
                    (*(*(*info).keymap_info).pending_computations).size;
                (*(*(*info).keymap_info).pending_computations).size = (*(*(*info).keymap_info)
                    .pending_computations)
                    .size
                    .wrapping_add(1 as darray_size_t);
                let mut __need_0: darray_size_t =
                    (*(*(*info).keymap_info).pending_computations).size;
                if __need_0 > (*(*(*info).keymap_info).pending_computations).alloc {
                    (*(*(*info).keymap_info).pending_computations).alloc = darray_next_alloc(
                        (*(*(*info).keymap_info).pending_computations).alloc,
                        __need_0,
                        ::core::mem::size_of::<pending_computation>() as usize,
                    );
                    (*(*(*info).keymap_info).pending_computations).item = realloc(
                        (*(*(*info).keymap_info).pending_computations).item
                            as *mut ::core::ffi::c_void,
                        ((*(*(*info).keymap_info).pending_computations).alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<pending_computation>() as usize),
                    )
                        as *mut pending_computation;
                }
                *(*(*(*info).keymap_info).pending_computations).item.offset(
                    (*(*(*info).keymap_info).pending_computations)
                        .size
                        .wrapping_sub(1 as darray_size_t) as isize,
                ) = pending_computation {
                    expr: *value_ptr,
                    computed: false,
                    value: 0 as u32,
                };
                *value_ptr = ::core::ptr::null_mut::<ExprDef>();
                (*keyi).out_of_range_group_number = pending_index as xkb_layout_index_t;
            } else {
                (*keyi).set_out_of_range_pending_group((false_0 != 0) as bool);
                (*keyi).out_of_range_group_number = grp.wrapping_sub(1 as xkb_layout_index_t);
            }
            (*keyi).set_out_of_range_group_policy(
                XKB_LAYOUT_OUT_OF_RANGE_REDIRECT as xkb_layout_out_of_range_policy,
            );
            (*keyi).set_defined(
                (*keyi).defined() | KEY_FIELD_GROUPINFO as ::core::ffi::c_int as key_field,
            );
        } else {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Unknown field \"%s\" in a key; definition ignored\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_UNKNOWN_FIELD as ::core::ffi::c_int,
                field,
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_UNKNOWN_KEY_FIELDS as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn SetGroupName(
    mut info: *mut SymbolsInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
    mut merge: merge_mode,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                b"[XKB-%03d] You must specify an index when specifying a group name; Group name definition without array subscript ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if ExprResolveGroup(
            (*info).keymap_info,
            arrayNdx,
            false_0 != 0,
            &raw mut group,
            ::core::ptr::null_mut::<bool>(),
        ) as ::core::ffi::c_uint
            != PARSER_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Illegal index in group name definition; Definition with non-integer array index ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        let mut name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, value, &raw mut name) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Group name must be a string; Illegal name for group %u ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                group,
            );
            return false_0 != 0;
        }
        let mut group_to_use: xkb_layout_index_t = 0;
        if (*info).explicit_group == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            group_to_use = group.wrapping_sub(1 as xkb_layout_index_t);
        } else if group.wrapping_sub(1 as xkb_layout_index_t) == 0 as xkb_layout_index_t {
            group_to_use = (*info).explicit_group;
        } else {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] An explicit group was specified for the '%s' map, but it provides a name for a group other than Group1 (%u); Ignoring group name '%s'\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_NON_BASE_GROUP_NAME as ::core::ffi::c_int,
                (*info).name,
                group,
                xkb_atom_text((*info).ctx, name),
            );
            return false_0 != 0;
        }
        if group_to_use >= (*info).group_names.size as xkb_layout_index_t {
            let mut __oldSize: darray_size_t = (*info).group_names.size;
            let mut __newSize: darray_size_t =
                (group_to_use as darray_size_t).wrapping_add(1 as darray_size_t);
            (*info).group_names.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*info).group_names.alloc {
                    (*info).group_names.alloc = darray_next_alloc(
                        (*info).group_names.alloc,
                        __need,
                        ::core::mem::size_of::<xkb_atom_t>() as usize,
                    );
                    (*info).group_names.item =
                        realloc(
                            (*info).group_names.item as *mut ::core::ffi::c_void,
                            ((*info).group_names.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as usize),
                        ) as *mut xkb_atom_t;
                }
                memset(
                    (*info).group_names.item.offset(__oldSize as isize) as *mut xkb_atom_t
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as usize),
                );
            }
        } else {
            let old_name: xkb_atom_t = *(*info).group_names.item.offset(group_to_use as isize);
            if old_name != XKB_ATOM_NONE as xkb_atom_t && old_name != name {
                let replace: bool = merge as ::core::ffi::c_uint
                    != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
                let use_0: xkb_atom_t = if replace as ::core::ffi::c_int != 0 {
                    name
                } else {
                    old_name
                };
                let ignore: xkb_atom_t = if replace as ::core::ffi::c_int != 0 {
                    old_name
                } else {
                    name
                };
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Multiple definitions of group %u name in map '%s'; Using '%s', ignoring '%s'\n\0"
                        .as_ptr() as *const i8,
                    group_to_use,
                    (*info).name,
                    xkb_atom_text((*info).ctx, use_0),
                    xkb_atom_text((*info).ctx, ignore),
                );
                name = use_0;
            }
        }
        *(*info).group_names.item.offset(group_to_use as isize) = name;
        return true_0 != 0;
    }
}
unsafe extern "C" fn HandleGlobalVar(mut info: *mut SymbolsInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const i8 = ::core::ptr::null::<i8>();
        let mut field: *const i8 = ::core::ptr::null::<i8>();
        let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut ret: bool = false;
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut arrayNdx,
        ) {
            return false_0 != 0;
        }
        if !elem.is_null()
            && istreq(elem, b"key\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            let mut temp: KeyInfo = {
                let mut init = KeyInfo {
                    out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                    name: 0 as xkb_atom_t,
                    vmodmap: 0,
                    default_type: 0,
                    out_of_range_group_number: 0,
                    groups: C2Rust_Unnamed_22 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<GroupInfo>(),
                    },
                    overlays_alloc: 0,
                    overlays: 0,
                    c2rust_unnamed: C2Rust_Unnamed_21 {
                        overlay_key: ::core::ptr::null::<xkb_key>(),
                    },
                };
                init.set_out_of_range_group_policy(XKB_LAYOUT_OUT_OF_RANGE_WRAP);
                init.set_defined(0 as key_field);
                init.set_merge(MERGE_DEFAULT);
                init.set_repeat(KEY_REPEAT_UNDEFINED);
                init.set_out_of_range_pending_group(false);
                init.set_overlays_clear(false);
                init
            };
            InitKeyInfo((*info).ctx, &raw mut temp);
            temp.set_merge(
                (if temp.merge() as ::core::ffi::c_int == MERGE_REPLACE as ::core::ffi::c_int {
                    MERGE_OVERRIDE as ::core::ffi::c_int as ::core::ffi::c_uint
                } else {
                    (*stmt).merge as ::core::ffi::c_uint
                }) as merge_mode as merge_mode,
            );
            ret = SetSymbolsField(info, &raw mut temp, field, arrayNdx, &raw mut (*stmt).value);
            MergeKeys(
                info,
                &raw mut (*info).default_key,
                &raw mut temp,
                true_0 != 0,
            );
        } else if elem.is_null()
            && (istreq(field, b"name\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                || istreq(field, b"groupname\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0)
        {
            ret = SetGroupName(info, arrayNdx, (*stmt).value as *mut ExprDef, (*stmt).merge);
        } else if elem.is_null()
            && (istreq(field, b"groupswrap\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                || istreq(field, b"wrapgroups\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0)
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Global \"groupswrap\" not supported; Ignored\n\0".as_ptr()
                    as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
            );
            ret = true_0 != 0;
        } else if elem.is_null()
            && (istreq(field, b"groupsclamp\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                || istreq(field, b"clampgroups\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0)
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Global \"groupsclamp\" not supported; Ignored\n\0".as_ptr()
                    as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
            );
            ret = true_0 != 0;
        } else if elem.is_null()
            && (istreq(field, b"groupsredirect\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
                || istreq(field, b"redirectgroups\0".as_ptr() as *const i8) as ::core::ffi::c_int
                    != 0)
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Global \"groupsredirect\" not supported; Ignored\n\0".as_ptr()
                    as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
            );
            ret = true_0 != 0;
        } else if elem.is_null()
            && istreq(field, b"allownone\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Radio groups not supported; Ignoring \"allownone\" specification\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as ::core::ffi::c_int,
            );
            ret = true_0 != 0;
        } else if !elem.is_null() {
            ret = SetDefaultActionField(
                (*info).keymap_info,
                &raw mut (*info).default_actions,
                &raw mut (*info).mods,
                elem,
                field,
                arrayNdx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as ::core::ffi::c_uint
                != PARSER_FATAL_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint;
        } else {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Default defined for unknown field \"%s\"; Ignored\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as ::core::ffi::c_int,
                field,
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                == 0;
        }
        return ret;
    }
}
unsafe extern "C" fn HandleSymbolsBody(
    mut info: *mut SymbolsInfo,
    mut def: *mut VarDef,
    mut keyi: *mut KeyInfo,
) -> bool {
    unsafe {
        let mut all_valid_entries: bool = true_0 != 0;
        while !def.is_null() {
            let mut field: *const i8 = ::core::ptr::null::<i8>();
            let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            let mut ok: bool = true_0 != 0;
            if (*def).name.is_null() {
                if (*def).value.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    || (*(*def).value).common.type_0 as ::core::ffi::c_uint
                        != STMT_EXPR_ACTION_LIST as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    field = b"symbols\0".as_ptr() as *const i8;
                } else {
                    field = b"actions\0".as_ptr() as *const i8;
                }
                arrayNdx = ::core::ptr::null_mut::<ExprDef>();
            } else {
                let mut elem: *const i8 = ::core::ptr::null::<i8>();
                ok = ExprResolveLhs(
                    (*info).ctx,
                    (*def).name,
                    &raw mut elem,
                    &raw mut field,
                    &raw mut arrayNdx,
                );
                if ok as ::core::ffi::c_int != 0 && !elem.is_null() {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Cannot set global defaults for \"%s\" element within a key statement: move statements to the global file scope. Assignment to \"%s.%s\" ignored.\n\0"
                            .as_ptr() as *const i8,
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as ::core::ffi::c_int,
                        elem,
                        elem,
                        field,
                    );
                    ok = false_0 != 0;
                }
            }
            if (*def).value.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Could not allocate the value of field \"%s\". Statement ignored.\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
                    field,
                );
                ok = false_0 != 0;
            }
            if !ok || !SetSymbolsField(info, keyi, field, arrayNdx, &raw mut (*def).value) {
                all_valid_entries = false_0 != 0;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return all_valid_entries;
    }
}
unsafe extern "C" fn SetExplicitGroup(mut info: *mut SymbolsInfo, mut keyi: *mut KeyInfo) -> bool {
    unsafe {
        let mut i: xkb_layout_index_t = 0;
        let mut groupi: *mut GroupInfo = ::core::ptr::null_mut::<GroupInfo>();
        let mut warn: bool = false_0 != 0;
        if (*info).explicit_group == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return true_0 != 0;
        }
        if !(*keyi).groups.item.is_null() {
            i = 1 as xkb_layout_index_t;
            groupi = (*keyi).groups.item.offset(1 as ::core::ffi::c_int as isize) as *mut GroupInfo;
            while i < (*keyi).groups.size as xkb_layout_index_t {
                if (*groupi).defined as u64 != 0 {
                    warn = true_0 != 0;
                    ClearGroupInfo(groupi);
                    InitGroupInfo(groupi);
                }
                i = i.wrapping_add(1);
                groupi = groupi.offset(1);
            }
        }
        if warn {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] For the map %s the explicit group %u is specified, but key %s has more than one group defined; All groups except first one will be ignored\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE as ::core::ffi::c_int,
                (*info).name,
                (*info).explicit_group.wrapping_add(1 as xkb_layout_index_t),
                KeyInfoText(info, keyi),
            );
        }
        let mut __oldSize: darray_size_t = (*keyi).groups.size;
        let mut __newSize: darray_size_t =
            ((*info).explicit_group as darray_size_t).wrapping_add(1 as darray_size_t);
        (*keyi).groups.size = __newSize;
        if __newSize > __oldSize {
            let mut __need: darray_size_t = __newSize;
            if __need > (*keyi).groups.alloc {
                (*keyi).groups.alloc = darray_next_alloc(
                    (*keyi).groups.alloc,
                    __need,
                    ::core::mem::size_of::<GroupInfo>() as usize,
                );
                (*keyi).groups.item = realloc(
                    (*keyi).groups.item as *mut ::core::ffi::c_void,
                    ((*keyi).groups.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                ) as *mut GroupInfo;
            }
            memset(
                (*keyi).groups.item.offset(__oldSize as isize) as *mut GroupInfo
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (__newSize.wrapping_sub(__oldSize) as usize)
                    .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
            );
        }
        if (*info).explicit_group > 0 as xkb_layout_index_t {
            *(*keyi).groups.item.offset((*info).explicit_group as isize) =
                *(*keyi).groups.item.offset(0 as ::core::ffi::c_int as isize);
            InitGroupInfo(
                (*keyi).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut GroupInfo
            );
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn HandleSymbolsDef(
    mut info: *mut SymbolsInfo,
    mut stmt: *mut SymbolsDef,
) -> bool {
    unsafe {
        let mut keyi: KeyInfo = KeyInfo {
            name: 0,
            vmodmap: 0,
            default_type: 0,
            out_of_range_group_number: 0,
            groups: C2Rust_Unnamed_22 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<GroupInfo>(),
            },
            out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
            overlays_alloc: 0,
            overlays: 0,
            c2rust_unnamed: C2Rust_Unnamed_21 {
                overlay_key: ::core::ptr::null::<xkb_key>(),
            },
        };
        keyi = (*info).default_key;
        keyi.groups.item = ::core::ptr::null_mut::<GroupInfo>();
        keyi.groups.size = 0 as darray_size_t;
        keyi.groups.alloc = 0 as darray_size_t;
        let mut __count: darray_size_t = (*info).default_key.groups.size;
        keyi.groups.size = __count;
        let mut __need: darray_size_t = keyi.groups.size;
        if __need > keyi.groups.alloc {
            keyi.groups.alloc = darray_next_alloc(
                keyi.groups.alloc,
                __need,
                ::core::mem::size_of::<GroupInfo>() as usize,
            );
            keyi.groups.item = realloc(
                keyi.groups.item as *mut ::core::ffi::c_void,
                (keyi.groups.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
            ) as *mut GroupInfo;
        }
        if __count != 0 as darray_size_t {
            memcpy(
                keyi.groups.item as *mut ::core::ffi::c_void,
                (*info).default_key.groups.item as *const ::core::ffi::c_void,
                (__count as usize).wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
            );
        }
        let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while i < keyi.groups.size as xkb_layout_index_t {
            CopyGroupInfo(
                keyi.groups.item.offset(i as isize) as *mut GroupInfo,
                (*info).default_key.groups.item.offset(i as isize) as *mut GroupInfo,
            );
            i = i.wrapping_add(1);
        }
        keyi.set_merge((*stmt).merge as merge_mode);
        keyi.name = (*stmt).keyName;
        if HandleSymbolsBody(info, (*stmt).symbols, &raw mut keyi) as ::core::ffi::c_int != 0
            && SetExplicitGroup(info, &raw mut keyi) as ::core::ffi::c_int != 0
            && AddKeySymbols(info, &raw mut keyi, true_0 != 0) as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        ClearKeyInfo(&raw mut keyi);
        (*info).errorCount += 1;
        return false_0 != 0;
    }
}
unsafe extern "C" fn HandleModMapDef(mut info: *mut SymbolsInfo, mut def: *mut ModMapDef) -> bool {
    unsafe {
        let mut tmp: ModMapEntry = ModMapEntry {
            merge: MERGE_DEFAULT,
            haveSymbol: false,
            modifier: 0,
            u: C2Rust_Unnamed_19 { keyName: 0 },
        };
        let mut ndx: xkb_mod_index_t = 0;
        let mut ok: bool = false;
        let mut ctx: *mut xkb_context = (*info).ctx;
        let mut modifier_name: *const i8 = xkb_atom_text(ctx, (*def).modifier);
        if istreq(modifier_name, b"none\0".as_ptr() as *const i8) {
            ndx = XKB_MOD_NONE as xkb_mod_index_t;
        } else {
            ndx = XkbModNameToIndex(&raw mut (*info).mods, (*def).modifier, MOD_REAL);
            if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal modifier map definition; Ignoring map for non-modifier \"%s\"\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_INVALID_REAL_MODIFIER as ::core::ffi::c_int,
                    xkb_atom_text(ctx, (*def).modifier),
                );
                return false_0 != 0;
            }
        }
        ok = true_0 != 0;
        tmp.modifier = ndx;
        tmp.merge = (*def).merge;
        let mut c2rust_current_block_19: u64;
        let mut key: *mut ExprDef = (*def).keys as *mut ExprDef;
        while !key.is_null() {
            if (*key).common.type_0 as ::core::ffi::c_uint
                == STMT_EXPR_KEYNAME_LITERAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                tmp.haveSymbol = false_0 != 0;
                tmp.u.keyName = (*key).key_name.key_name;
                c2rust_current_block_19 = 5601891728916014340;
            } else if (*key).common.type_0 as ::core::ffi::c_uint
                == STMT_EXPR_KEYSYM_LITERAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*key).keysym.keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    c2rust_current_block_19 = 13536709405535804910;
                } else {
                    tmp.haveSymbol = true_0 != 0;
                    tmp.u.keySym = (*key).keysym.keysym;
                    c2rust_current_block_19 = 5601891728916014340;
                }
            } else {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Modmap entries may contain only key names or keysyms; Illegal definition for %s modifier ignored\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_INVALID_MODMAP_ENTRY as ::core::ffi::c_int,
                    ModIndexText((*info).ctx, &raw mut (*info).mods, tmp.modifier),
                );
                c2rust_current_block_19 = 13536709405535804910;
            }
            match c2rust_current_block_19 {
                5601891728916014340 => {
                    ok = AddModMapEntry(info, &raw mut tmp) as ::core::ffi::c_int != 0
                        && ok as ::core::ffi::c_int != 0;
                }
                _ => {}
            }
            key = (*key).common.next as *mut ExprDef;
        }
        return ok;
    }
}
unsafe extern "C" fn HandleSymbolsFile(mut info: *mut SymbolsInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        free((*info).name as *mut ::core::ffi::c_void);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as ::core::ffi::c_uint {
                1 => {
                    ok = HandleIncludeSymbols(info, stmt as *mut IncludeStmt);
                }
                30 => {
                    ok = HandleSymbolsDef(info, stmt as *mut SymbolsDef);
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef((*info).ctx, &raw mut (*info).mods, stmt as *mut VModDef);
                }
                31 => {
                    ok = HandleModMapDef(info, stmt as *mut ModMapDef);
                }
                35 | 36 => {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Unsupported symbols %s statement \"%s\"; Ignoring\n\0".as_ptr()
                            as *const i8,
                        XKB_ERROR_UNKNOWN_STATEMENT as ::core::ffi::c_int,
                        if (*stmt).type_0 as ::core::ffi::c_uint
                            == STMT_UNKNOWN_COMPOUND as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"compound\0".as_ptr() as *const i8
                        } else {
                            b"declaration\0".as_ptr() as *const i8
                        },
                        (*(stmt as *mut UnknownStatement)).name,
                    );
                    ok = (*(*info).keymap_info).strict as ::core::ffi::c_uint
                        & PARSER_NO_UNKNOWN_STATEMENTS as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0;
                }
                _ => {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Symbols files may not include other types; Ignoring %s\n\0"
                            .as_ptr() as *const i8,
                        XKB_ERROR_WRONG_STATEMENT_TYPE as ::core::ffi::c_int,
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false_0 != 0;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10 as ::core::ffi::c_int {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Abandoning symbols file \"%s\"\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_XKB_SYNTAX as ::core::ffi::c_int,
                    safe_map_name(file),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe extern "C" fn FindKeyForSymbol(
    mut keymap: *mut xkb_keymap,
    mut sym: xkb_keysym_t,
) -> *mut xkb_key {
    unsafe {
        let mut got_one_group: bool = false;
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        loop {
            let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
            got_one_group = false_0 != 0;
            let mut got_one_level: bool = false;
            loop {
                got_one_level = false_0 != 0;
                let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
                key = (*keymap).keys.offset(
                    (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                        0 as xkb_keycode_t
                    } else {
                        (*keymap).min_key_code
                    }) as isize,
                );
                while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
                    if group < (*key).num_groups() && level < XkbKeyNumLevels(key, group) {
                        got_one_level = true_0 != 0;
                        got_one_group = got_one_level;
                        let num_syms: xkb_keysym_count_t =
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_syms;
                        if num_syms as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            let mut k: xkb_keysym_count_t = 0 as xkb_keysym_count_t;
                            while (k as ::core::ffi::c_int) < num_syms as ::core::ffi::c_int {
                                if *(*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .s
                                .syms
                                .offset(k as isize)
                                    == sym
                                {
                                    return key;
                                }
                                k = k.wrapping_add(1);
                            }
                        } else if num_syms as ::core::ffi::c_int != 0
                            && (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .s
                            .sym == sym
                        {
                            return key;
                        }
                    }
                    key = key.offset(1);
                }
                level = level.wrapping_add(1);
                if !got_one_level {
                    break;
                }
            }
            group = group.wrapping_add(1);
            if !got_one_group {
                break;
            }
        }
        return ::core::ptr::null_mut::<xkb_key>();
    }
}
unsafe extern "C" fn FindAutomaticType(
    mut ctx: *mut xkb_context,
    mut groupi: *mut GroupInfo,
) -> xkb_atom_t {
    unsafe {
        let mut sym0: xkb_keysym_t = 0;
        let mut sym1: xkb_keysym_t = 0;
        let width: xkb_level_index_t = (*groupi).levels.size as xkb_level_index_t;
        if width == 1 as xkb_level_index_t || width <= 0 as xkb_level_index_t {
            return xkb_atom_intern(
                ctx,
                b"ONE_LEVEL\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 10]>() as usize).wrapping_sub(1 as usize),
            );
        }
        sym0 = if (*(*groupi)
            .levels
            .item
            .offset(0 as ::core::ffi::c_int as isize))
        .num_syms as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            XKB_KEY_NoSymbol as xkb_keysym_t
        } else if (*(*groupi)
            .levels
            .item
            .offset(0 as ::core::ffi::c_int as isize))
        .num_syms as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            (*(*groupi)
                .levels
                .item
                .offset(0 as ::core::ffi::c_int as isize))
            .s
            .sym
        } else {
            *(*(*groupi)
                .levels
                .item
                .offset(0 as ::core::ffi::c_int as isize))
            .s
            .syms
            .offset(0 as ::core::ffi::c_int as isize)
        };
        sym1 = if (*(*groupi)
            .levels
            .item
            .offset(1 as ::core::ffi::c_int as isize))
        .num_syms as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            XKB_KEY_NoSymbol as xkb_keysym_t
        } else if (*(*groupi)
            .levels
            .item
            .offset(1 as ::core::ffi::c_int as isize))
        .num_syms as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            (*(*groupi)
                .levels
                .item
                .offset(1 as ::core::ffi::c_int as isize))
            .s
            .sym
        } else {
            *(*(*groupi)
                .levels
                .item
                .offset(1 as ::core::ffi::c_int as isize))
            .s
            .syms
            .offset(0 as ::core::ffi::c_int as isize)
        };
        if width == 2 as xkb_level_index_t {
            if xkb_keysym_is_lower(sym0) as ::core::ffi::c_int != 0
                && xkb_keysym_is_upper_or_title(sym1) as ::core::ffi::c_int != 0
            {
                return xkb_atom_intern(
                    ctx,
                    b"ALPHABETIC\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 11]>() as usize).wrapping_sub(1 as usize),
                );
            }
            if xkb_keysym_is_keypad(sym0) as ::core::ffi::c_int != 0
                || xkb_keysym_is_keypad(sym1) as ::core::ffi::c_int != 0
            {
                return xkb_atom_intern(
                    ctx,
                    b"KEYPAD\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
                );
            }
            return xkb_atom_intern(
                ctx,
                b"TWO_LEVEL\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 10]>() as usize).wrapping_sub(1 as usize),
            );
        }
        if width <= 4 as xkb_level_index_t {
            if xkb_keysym_is_lower(sym0) as ::core::ffi::c_int != 0
                && xkb_keysym_is_upper_or_title(sym1) as ::core::ffi::c_int != 0
            {
                let mut sym2: xkb_keysym_t = 0;
                let mut sym3: xkb_keysym_t = 0;
                sym2 = if (*(*groupi)
                    .levels
                    .item
                    .offset(2 as ::core::ffi::c_int as isize))
                .num_syms as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    XKB_KEY_NoSymbol as xkb_keysym_t
                } else if (*(*groupi)
                    .levels
                    .item
                    .offset(2 as ::core::ffi::c_int as isize))
                .num_syms as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    (*(*groupi)
                        .levels
                        .item
                        .offset(2 as ::core::ffi::c_int as isize))
                    .s
                    .sym
                } else {
                    *(*(*groupi)
                        .levels
                        .item
                        .offset(2 as ::core::ffi::c_int as isize))
                    .s
                    .syms
                    .offset(0 as ::core::ffi::c_int as isize)
                };
                sym3 = if width == 4 as xkb_level_index_t {
                    if (*(*groupi)
                        .levels
                        .item
                        .offset(3 as ::core::ffi::c_int as isize))
                    .num_syms as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        XKB_KEY_NoSymbol as xkb_keysym_t
                    } else if (*(*groupi)
                        .levels
                        .item
                        .offset(3 as ::core::ffi::c_int as isize))
                    .num_syms as ::core::ffi::c_int
                        == 1 as ::core::ffi::c_int
                    {
                        (*(*groupi)
                            .levels
                            .item
                            .offset(3 as ::core::ffi::c_int as isize))
                        .s
                        .sym
                    } else {
                        *(*(*groupi)
                            .levels
                            .item
                            .offset(3 as ::core::ffi::c_int as isize))
                        .s
                        .syms
                        .offset(0 as ::core::ffi::c_int as isize)
                    }
                } else {
                    XKB_KEY_NoSymbol as xkb_keysym_t
                };
                if xkb_keysym_is_lower(sym2) as ::core::ffi::c_int != 0
                    && xkb_keysym_is_upper_or_title(sym3) as ::core::ffi::c_int != 0
                {
                    return xkb_atom_intern(
                        ctx,
                        b"FOUR_LEVEL_ALPHABETIC\0".as_ptr() as *const i8,
                        (::core::mem::size_of::<[i8; 22]>() as usize).wrapping_sub(1 as usize),
                    );
                }
                return xkb_atom_intern(
                    ctx,
                    b"FOUR_LEVEL_SEMIALPHABETIC\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 26]>() as usize).wrapping_sub(1 as usize),
                );
            }
            if xkb_keysym_is_keypad(sym0) as ::core::ffi::c_int != 0
                || xkb_keysym_is_keypad(sym1) as ::core::ffi::c_int != 0
            {
                return xkb_atom_intern(
                    ctx,
                    b"FOUR_LEVEL_KEYPAD\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 18]>() as usize).wrapping_sub(1 as usize),
                );
            }
            return xkb_atom_intern(
                ctx,
                b"FOUR_LEVEL\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 11]>() as usize).wrapping_sub(1 as usize),
            );
        }
        return XKB_ATOM_NONE as xkb_atom_t;
    }
}
unsafe extern "C" fn FindTypeForGroup(
    mut keymap: *mut xkb_keymap,
    mut keyi: *mut KeyInfo,
    mut group: xkb_layout_index_t,
    mut explicit_type: *mut bool,
) -> *const xkb_key_type {
    unsafe {
        let mut i: darray_size_t = 0;
        let mut groupi: *mut GroupInfo =
            (*keyi).groups.item.offset(group as isize) as *mut GroupInfo;
        let mut type_name: xkb_atom_t = (*groupi).type_0;
        *explicit_type = true_0 != 0;
        if type_name == XKB_ATOM_NONE as xkb_atom_t {
            if (*keyi).default_type != XKB_ATOM_NONE as xkb_atom_t {
                type_name = (*keyi).default_type;
            } else {
                type_name = FindAutomaticType((*keymap).ctx, groupi);
                if type_name != XKB_ATOM_NONE as xkb_atom_t {
                    *explicit_type = false_0 != 0;
                }
            }
        }
        if type_name == XKB_ATOM_NONE as xkb_atom_t {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Couldn't find an automatic type for key '%s' group %u with %u levels; Using the default type\n\0"
                    .as_ptr() as *const i8,
                XKB_WARNING_CANNOT_INFER_KEY_TYPE as ::core::ffi::c_int,
                KeyNameText((*keymap).ctx, (*keyi).name),
                group.wrapping_add(1 as xkb_layout_index_t),
                (*groupi).levels.size,
            );
        } else {
            i = 0;
            i = 0 as darray_size_t;
            while i < (*keymap).num_types {
                if (*(*keymap).types.offset(i as isize)).name == type_name {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i >= (*keymap).num_types {
                xkb_log(
                    (*keymap).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] The type \"%s\" for key '%s' group %u was not previously defined; Using the default type\n\0"
                        .as_ptr() as *const i8,
                    XKB_WARNING_UNDEFINED_KEY_TYPE as ::core::ffi::c_int,
                    xkb_atom_text((*keymap).ctx, type_name),
                    KeyNameText((*keymap).ctx, (*keyi).name),
                    group.wrapping_add(1 as xkb_layout_index_t),
                );
            } else {
                (*(*keymap).types.offset(i as isize)).required = true_0 != 0;
                return (*keymap).types.offset(i as isize) as *mut xkb_key_type;
            }
        }
        (*(*keymap).types.offset(0 as ::core::ffi::c_int as isize)).required = true_0 != 0;
        return (*keymap).types.offset(0 as ::core::ffi::c_int as isize) as *mut xkb_key_type;
    }
}
unsafe extern "C" fn CopySymbolsDefToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
) -> bool {
    unsafe {
        let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
        let mut groupi: *mut GroupInfo = ::core::ptr::null_mut::<GroupInfo>();
        let mut group0: *const GroupInfo = ::core::ptr::null::<GroupInfo>();
        let mut i: xkb_layout_index_t = 0;

        // The name is guaranteed to be real and not an alias, so 'false' is safe here
        key = XkbKeyByName(keymap, (*keyi).name, false);
        if key.is_null() {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                b"[XKB-%03d] Key %s not found in keycodes; Symbols ignored\n\0".as_ptr()
                    as *const i8,
                XKB_WARNING_UNDEFINED_KEYCODE as ::core::ffi::c_int,
                KeyInfoText(info, keyi),
            );
            return false;
        }

        // Find the range of groups we need
        (*key).set_num_groups(0);
        if !(*keyi).groups.item.is_null() {
            i = 0;
            groupi = (*keyi).groups.item.offset(0) as *mut GroupInfo;
            while i < (*keyi).groups.size as xkb_layout_index_t {
                // Skip groups that have no levels and no explicit type
                let has_explicit_type = ((*keyi).defined() as ::core::ffi::c_int
                    & KEY_FIELD_DEFAULT_TYPE as ::core::ffi::c_int
                    != 0)
                    || ((*groupi).defined as ::core::ffi::c_uint
                        & GROUP_FIELD_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0);
                if (*groupi).levels.size > 0 || has_explicit_type {
                    (*key).set_num_groups(i.wrapping_add(1));
                }
                if has_explicit_type {
                    (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                        | EXPLICIT_TYPES as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as xkb_explicit_components;
                }
                i = i.wrapping_add(1);
                groupi = groupi.offset(1);
            }
        }

        if (*key).num_groups() <= 0 {
            // A key with no group may still have other fields defined
            if (*keyi).defined() as ::core::ffi::c_int != 0 {
                // goto key_fields
            } else {
                return false;
            }
        } else {
            // Resize groups array
            let __need: darray_size_t = (*key).num_groups() as darray_size_t;
            if __need > (*keyi).groups.alloc {
                (*keyi).groups.alloc = darray_next_alloc(
                    (*keyi).groups.alloc,
                    __need,
                    ::core::mem::size_of::<GroupInfo>() as usize,
                );
                (*keyi).groups.item = realloc(
                    (*keyi).groups.item as *mut ::core::ffi::c_void,
                    ((*keyi).groups.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<GroupInfo>() as usize),
                ) as *mut GroupInfo;
            }
            if __need < (*keyi).groups.size {
                // Zero out elements being added
                let start_ptr = (*keyi).groups.item.offset((*keyi).groups.size as isize);
                let count = (__need - (*keyi).groups.size) as usize;
                ::core::ptr::write_bytes(start_ptr, 0, count);
            }
            (*keyi).groups.size = __need;

            // If there are empty groups between non-empty ones, fill them with data from the first group
            group0 = (*keyi).groups.item.offset(0) as *const GroupInfo;
            if !(*keyi).groups.item.is_null() {
                i = 1;
                groupi = (*keyi).groups.item.offset(1) as *mut GroupInfo;
                while i < (*keyi).groups.size as xkb_layout_index_t {
                    if (*groupi).defined == 0 {
                        CopyGroupInfo(groupi, group0);
                    }
                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            (*key).groups = calloc(
                (*key).num_groups() as usize,
                ::core::mem::size_of::<xkb_group>() as usize,
            ) as *mut xkb_group;

            // Find and assign the groups' types in the keymap
            if !(*keyi).groups.item.is_null() {
                i = 0;
                groupi = (*keyi).groups.item.offset(0) as *mut GroupInfo;
                while i < (*keyi).groups.size as xkb_layout_index_t {
                    let mut explicit_type = false;
                    let type_0: *const xkb_key_type =
                        FindTypeForGroup(keymap, keyi, i, &raw mut explicit_type);

                    // Always have as many levels as the type specifies
                    if (*type_0).num_levels < (*groupi).levels.size {
                        let mut leveli: *mut xkb_level = ::core::ptr::null_mut::<xkb_level>();
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                            b"[XKB-%03d] Type \"%s\" has %u levels, but %s has %u levels; Ignoring extra symbols\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_EXTRA_SYMBOLS_IGNORED as ::core::ffi::c_int,
                            xkb_atom_text((*keymap).ctx, (*type_0).name),
                            (*type_0).num_levels,
                            KeyInfoText(info, keyi),
                            (*groupi).levels.size,
                        );

                        if !(*groupi).levels.item.is_null() {
                            leveli = (*groupi).levels.item.offset((*type_0).num_levels as isize)
                                as *mut xkb_level;
                            while leveli
                                < (*groupi).levels.item.offset((*groupi).levels.size as isize)
                                    as *mut xkb_level
                            {
                                clear_level(leveli);
                                leveli = leveli.offset(1);
                            }
                        }
                    }

                    // Resize levels array to match type
                    let __need_levels: darray_size_t = (*type_0).num_levels;
                    if __need_levels > (*groupi).levels.alloc {
                        (*groupi).levels.alloc = darray_next_alloc(
                            (*groupi).levels.alloc,
                            __need_levels,
                            ::core::mem::size_of::<xkb_level>() as usize,
                        );
                        (*groupi).levels.item =
                            realloc(
                                (*groupi).levels.item as *mut ::core::ffi::c_void,
                                ((*groupi).levels.alloc as usize)
                                    .wrapping_mul(::core::mem::size_of::<xkb_level>() as usize),
                            ) as *mut xkb_level;
                    }
                    if __need_levels > (*groupi).levels.size {
                        // Zero out new elements
                        let start_ptr =
                            (*groupi).levels.item.offset((*groupi).levels.size as isize);
                        let count = (__need_levels - (*groupi).levels.size) as usize;
                        ::core::ptr::write_bytes(start_ptr, 0, count);
                    }
                    (*groupi).levels.size = __need_levels;

                    (*(*key).groups.offset(i as isize)).set_explicit_type(explicit_type);
                    (*(*key).groups.offset(i as isize)).type_0 = type_0;

                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            // Copy levels
            if !(*keyi).groups.item.is_null() {
                i = 0;
                groupi = (*keyi).groups.item.offset(0) as *mut GroupInfo;
                while i < (*keyi).groups.size as xkb_layout_index_t {
                    // Compute the capitalization transformation of the keysyms
                    let mut leveli: *mut xkb_level = ::core::ptr::null_mut::<xkb_level>();
                    if !(*groupi).levels.item.is_null() {
                        leveli = (*groupi).levels.item.offset(0) as *mut xkb_level;
                        while leveli
                            < (*groupi).levels.item.offset((*groupi).levels.size as isize)
                                as *mut xkb_level
                        {
                            match (*leveli).num_syms {
                                0 => {
                                    (*leveli).c2rust_unnamed.upper =
                                        XKB_KEY_NoSymbol as xkb_keysym_t;
                                }
                                1 => {
                                    (*leveli).c2rust_unnamed.upper =
                                        xkb_keysym_to_upper((*leveli).s.sym);
                                }
                                _ => {
                                    // Multiple keysyms: check if there is any cased keysym
                                    (*leveli).c2rust_unnamed.has_upper = false;
                                    let mut k: xkb_keysym_count_t = 0;
                                    while k < (*leveli).num_syms {
                                        let upper: xkb_keysym_t = xkb_keysym_to_upper(
                                            *(*leveli).s.syms.offset(k as isize),
                                        );
                                        if upper != *(*leveli).s.syms.offset(k as isize) {
                                            (*leveli).c2rust_unnamed.has_upper = true;
                                            break;
                                        }
                                        k = k.wrapping_add(1);
                                    }
                                    if (*leveli).c2rust_unnamed.has_upper {
                                        // Some cased keysyms: store the transformation result
                                        (*leveli).s.syms = realloc(
                                            (*leveli).s.syms as *mut ::core::ffi::c_void,
                                            (2 * (*leveli).num_syms as usize).wrapping_mul(
                                                ::core::mem::size_of::<xkb_keysym_t>() as usize,
                                            ),
                                        )
                                            as *mut xkb_keysym_t;
                                        if (*leveli).s.syms.is_null() {
                                            return false;
                                        }
                                        let mut k: xkb_keysym_count_t = 0;
                                        while k < (*leveli).num_syms {
                                            *(*leveli)
                                                .s
                                                .syms
                                                .offset(((*leveli).num_syms + k) as isize) =
                                                xkb_keysym_to_upper(
                                                    *(*leveli).s.syms.offset(k as isize),
                                                );
                                            k = k.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                            leveli = leveli.offset(1);
                        }
                    }

                    // Copy the level (darray_steal)
                    (*(*key).groups.offset(i as isize)).levels = (*groupi).levels.item;
                    if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                        *(::core::ptr::null_mut::<::core::ffi::c_void>() as *mut darray_size_t) =
                            (*groupi).levels.size;
                    }
                    (*groupi).levels.item = ::core::ptr::null_mut::<xkb_level>();
                    (*groupi).levels.size = 0;
                    (*groupi).levels.alloc = 0;

                    if (*(*(*key).groups.offset(i as isize)).type_0).num_levels > 1
                        || (*(*(*key).groups.offset(i as isize)).levels.offset(0)).num_syms > 0
                    {
                        (*(*key).groups.offset(i as isize)).set_explicit_symbols(true);
                        (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                            | EXPLICIT_SYMBOLS as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as xkb_explicit_components;
                    }
                    if (*groupi).defined as ::core::ffi::c_uint
                        & GROUP_FIELD_ACTS as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                    {
                        (*(*key).groups.offset(i as isize)).set_explicit_actions(true);
                        (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                            | EXPLICIT_INTERP as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as xkb_explicit_components;
                    }
                    if (*(*key).groups.offset(i as isize)).explicit_type() {
                        (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                            | EXPLICIT_TYPES as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as xkb_explicit_components;
                    }

                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            (*key).set_out_of_range_pending_group((*keyi).out_of_range_pending_group());
            (*key).set_out_of_range_group_number((*keyi).out_of_range_group_number);
            (*key).set_out_of_range_group_policy((*keyi).out_of_range_group_policy());
        }

        // key_fields:
        if (*keyi).defined() as ::core::ffi::c_int & KEY_FIELD_VMODMAP as ::core::ffi::c_int != 0 {
            (*key).vmodmap = (*keyi).vmodmap;
            (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                | EXPLICIT_VMODMAP as ::core::ffi::c_int as ::core::ffi::c_uint)
                as xkb_explicit_components;
        }

        if (*keyi).repeat() != KEY_REPEAT_UNDEFINED as key_repeat {
            (*key).set_repeats((*keyi).repeat() == KEY_REPEAT_YES as key_repeat);
            (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                | EXPLICIT_REPEAT as ::core::ffi::c_int as ::core::ffi::c_uint)
                as xkb_explicit_components;
        }

        if ((*keyi).defined() as ::core::ffi::c_int & KEY_FIELD_OVERLAY as ::core::ffi::c_int != 0)
            && (*keyi).overlays != 0
            && !(*keyi).overlays_clear()
        {
            (*key).overlays = (*keyi).overlays;
            if (*keyi).overlays_alloc != 0 {
                // Remove empty entries
                let mut remaining: xkb_overlay_mask_t = (*key).overlays;
                let mut overlays_keys: *mut *const xkb_key = (*keyi).c2rust_unnamed.overlays_keys;
                while remaining != 0 {
                    // isolate lowest set bit
                    let lsb: xkb_overlay_mask_t = remaining & (!remaining).wrapping_add(1);
                    remaining = remaining & !lsb;
                    if !(*overlays_keys).is_null() {
                        overlays_keys = overlays_keys.offset(1);
                    } else {
                        // drop current null value
                        (*key).overlays &= !lsb;
                    }
                }

                if (*key).overlays != 0 {
                    // Steal
                    (*key).c2rust_unnamed.overlays_keys = (*keyi).c2rust_unnamed.overlays_keys;
                    (*keyi).c2rust_unnamed.overlays_keys = ::core::ptr::null_mut();
                    (*keyi).overlays_alloc = 0;

                    (*key).set_overlays_inline(false);
                    (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                        | EXPLICIT_OVERLAY as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as xkb_explicit_components;
                }
            } else {
                (*key).c2rust_unnamed.overlay_key = (*keyi).c2rust_unnamed.overlay_key;
                (*key).set_overlays_inline(true);
                (*key).explicit = ((*key).explicit as ::core::ffi::c_uint
                    | EXPLICIT_OVERLAY as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as xkb_explicit_components;
            }
        }

        return true;
    }
}

unsafe extern "C" fn CopyModMapDefToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
    mut entry: *mut ModMapEntry,
) -> bool {
    unsafe {
        let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
        if !(*entry).haveSymbol {
            key = XkbKeyByName(keymap, (*entry).u.keyName, true_0 != 0);
            if key.is_null() {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                    b"[XKB-%03d] Key %s not found in keycodes; Modifier map entry for %s not updated\n\0"
                        .as_ptr() as *const i8,
                    XKB_WARNING_UNDEFINED_KEYCODE as ::core::ffi::c_int,
                    KeyNameText((*info).ctx, (*entry).u.keyName),
                    ModIndexText((*info).ctx, &raw mut (*info).mods, (*entry).modifier),
                );
                return false_0 != 0;
            }
        } else {
            key = FindKeyForSymbol(keymap, (*entry).u.keySym);
            if key.is_null() {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                    b"[XKB-%03d] Key \"%s\" not found in symbol map; Modifier map entry for %s not updated\n\0"
                        .as_ptr() as *const i8,
                    XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL as ::core::ffi::c_int,
                    KeysymText((*info).ctx, (*entry).u.keySym),
                    ModIndexText((*info).ctx, &raw mut (*info).mods, (*entry).modifier),
                );
                return false_0 != 0;
            }
        }
        if (*entry).modifier != XKB_MOD_NONE as xkb_mod_index_t {
            (*key).modmap = ((*key).modmap as ::core::ffi::c_uint
                | (1 as ::core::ffi::c_uint) << (*entry).modifier)
                as xkb_mod_mask_t;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn CopySymbolsToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
) -> bool {
    unsafe {
        let mut keyi: *mut KeyInfo = ::core::ptr::null_mut::<KeyInfo>();
        let mut mm: *mut ModMapEntry = ::core::ptr::null_mut::<ModMapEntry>();
        (*keymap).symbols_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).symbols_section_name);
        (*keymap).mods = (*info).mods;
        (*keymap).num_group_names = (*info).group_names.size as xkb_layout_index_t;
        (*keymap).group_names = (*info).group_names.item;
        if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
            *(::core::ptr::null_mut::<::core::ffi::c_void>() as *mut darray_size_t) =
                (*info).group_names.size;
        }
        (*info).group_names.item = ::core::ptr::null_mut::<xkb_atom_t>();
        (*info).group_names.size = 0 as darray_size_t;
        (*info).group_names.alloc = 0 as darray_size_t;
        if !(*info).keys.item.is_null() {
            keyi = (*info).keys.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyInfo;
            while keyi < (*info).keys.item.offset((*info).keys.size as isize) as *mut KeyInfo {
                if !CopySymbolsDefToKeymap(keymap, info, keyi) {
                    (*info).errorCount += 1;
                }
                keyi = keyi.offset(1);
            }
        }
        if xkb_context_get_log_verbosity((*keymap).ctx) > 3 as ::core::ffi::c_int {
            let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
            key = (*keymap).keys.offset(
                (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                    0 as xkb_keycode_t
                } else {
                    (*keymap).min_key_code
                }) as isize,
            );
            while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
                if !((*key).name == XKB_ATOM_NONE as xkb_atom_t) {
                    if ((*key).num_groups() as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_INFO,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"No symbols defined for %s\n\0".as_ptr() as *const i8,
                            KeyNameText((*info).ctx, (*key).name),
                        );
                    }
                }
                key = key.offset(1);
            }
        }
        if !(*info).modmaps.item.is_null() {
            mm = (*info)
                .modmaps
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut ModMapEntry;
            while mm
                < (*info).modmaps.item.offset((*info).modmaps.size as isize) as *mut ModMapEntry
            {
                if !CopyModMapDefToKeymap(keymap, info, mm) {
                    (*info).errorCount += 1;
                }
                mm = mm.offset(1);
            }
        }
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompileSymbols(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: SymbolsInfo = SymbolsInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: C2Rust_Unnamed_24 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<KeyInfo>(),
            },
            default_key: KeyInfo {
                name: 0,
                vmodmap: 0,
                default_type: 0,
                out_of_range_group_number: 0,
                groups: C2Rust_Unnamed_22 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<GroupInfo>(),
                },
                out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                overlays_alloc: 0,
                overlays: 0,
                c2rust_unnamed: C2Rust_Unnamed_21 {
                    overlay_key: ::core::ptr::null::<xkb_key>(),
                },
            },
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            group_names: C2Rust_Unnamed_20 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<xkb_atom_t>(),
            },
            modmaps: C2Rust_Unnamed_18 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<ModMapEntry>(),
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        InitSymbolsInfo(
            &raw mut info,
            keymap_info,
            0 as ::core::ffi::c_uint,
            &raw mut (*keymap_info).keymap.mods,
        );
        if !file.is_null() {
            HandleSymbolsFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as ::core::ffi::c_int) {
            if CopySymbolsToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearSymbolsInfo(&raw mut info);
                return true_0 != 0;
            }
        }
        ClearSymbolsInfo(&raw mut info);
        return false_0 != 0;
    }
}
