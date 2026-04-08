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
pub mod __stddef_size_t_h {
    pub type size_t = usize;
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
    pub type __time_t = ::core::ffi::c_long;
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
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
        pub _unused2: [::core::ffi::c_char; 8],
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
pub mod time_t_h {
    pub type time_t = __time_t;
    use super::types_h::__time_t;
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
        pub text_next: size_t,
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
    use super::__stddef_size_t_h::size_t;
    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
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
    pub type xkb_context_flags = ::core::ffi::c_uint;
    pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
    pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
    pub type xkb_keymap_serialize_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_SERIALIZE_EXPLICIT: xkb_keymap_serialize_flags = 4;
    pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
    pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
    pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
    pub const XKB_KEYCODE_MAX: ::core::ffi::c_uint =
        (0xffffffff as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 4294967295 as xkb_keymap_format;
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_context_include_path_append(
            context: *mut xkb_context,
            path: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn xkb_keymap_new_from_string(
            context: *mut xkb_context,
            string: *const ::core::ffi::c_char,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_new_from_buffer(
            context: *mut xkb_context,
            buffer: *const ::core::ffi::c_char,
            length: size_t,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_get_as_string(
            keymap: *mut xkb_keymap,
            format: xkb_keymap_format,
        ) -> *mut ::core::ffi::c_char;
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
    pub const XKB_KEYCODE_MAX_CONTIGUOUS: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
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
}
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    pub type test_compile_buffer_t = Option<
        unsafe extern "C" fn(
            *mut xkb_context,
            xkb_keymap_format,
            *const ::core::ffi::c_char,
            size_t,
            *mut ::core::ffi::c_void,
        ) -> *mut xkb_keymap,
    >;
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::xkbcommon_h::{
        xkb_keymap_compile_flags, xkb_keymap_format, xkb_keymap_serialize_flags,
    };
    extern "C" {
        pub fn test_init();
        pub fn test_get_path(path_rel: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn test_read_file(path_rel: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_buffer(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            buf: *const ::core::ffi::c_char,
            len: size_t,
        ) -> *mut xkb_keymap;
        pub fn test_compile_buffer2(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
            buf: *const ::core::ffi::c_char,
            len: size_t,
        ) -> *mut xkb_keymap;
        pub fn test_compile_output(
            ctx: *mut xkb_context,
            input_format: xkb_keymap_format,
            output_format: xkb_keymap_format,
            compile_buffer_0: test_compile_buffer_t,
            compile_buffer_private: *mut ::core::ffi::c_void,
            test_title: *const ::core::ffi::c_char,
            keymap_str: *const ::core::ffi::c_char,
            keymap_len: size_t,
            rel_path: *const ::core::ffi::c_char,
            update_output_files: bool,
        ) -> bool;
        pub fn test_compile_output2(
            ctx: *mut xkb_context,
            input_format: xkb_keymap_format,
            output_format: xkb_keymap_format,
            serialize_flags: xkb_keymap_serialize_flags,
            compile_buffer_0: test_compile_buffer_t,
            compile_buffer_private: *mut ::core::ffi::c_void,
            test_title: *const ::core::ffi::c_char,
            keymap_str: *const ::core::ffi::c_char,
            keymap_len: size_t,
            rel_path: *const ::core::ffi::c_char,
            update_output_files: bool,
        ) -> bool;
        pub fn test_compile_rules(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            rules: *const ::core::ffi::c_char,
            model: *const ::core::ffi::c_char,
            layout: *const ::core::ffi::c_char,
            variant: *const ::core::ffi::c_char,
            options: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
    }
}
pub mod stdio_h {
    use super::__stddef_size_t_h::size_t;
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn streq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if !s1.is_null() && !s2.is_null() {
            } else {
                __assert_fail(
                    b"s1 && s2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../src/utils.h\0".as_ptr() as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_uint,
                    b"_Bool streq(const char *, const char *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_not_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return false;
            }
            return streq(s1, s2);
        }
    }
    use super::assert_h::__assert_fail;
    use super::string_h::strcmp;
}
pub mod stdbool_h {
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
pub mod stdlib_h {
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
        pub fn rand() -> ::core::ffi::c_int;
        pub fn srand(__seed: ::core::ffi::c_uint);
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn exit(__status: ::core::ffi::c_int) -> !;
        pub fn setenv(
            __name: *const ::core::ffi::c_char,
            __value: *const ::core::ffi::c_char,
            __replace: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod config_h {
    pub const EXIT_INVALID_USAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
}
pub mod test_config_h {
    pub const TEST_XKB_CONFIG_ROOT: [::core::ffi::c_char; 41] = unsafe {
        ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
            *b"/home/rano/Public/libxkbcommon/test/data\0",
        )
    };
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::config_h::EXIT_INVALID_USAGE;
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
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
    XKB_KEYCODE_MAX_CONTIGUOUS,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdio_h::{fprintf, snprintf, stderr};
pub use self::stdlib_h::{atoi, exit, free, rand, setenv, srand, EXIT_SUCCESS};
use self::string_h::strlen;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_config_h::TEST_XKB_CONFIG_ROOT;
pub use self::test_h::{
    test_compile_buffer, test_compile_buffer2, test_compile_buffer_t, test_compile_output,
    test_compile_output2, test_compile_rules, test_context_flags, test_get_context, test_get_path,
    test_init, test_read_file, CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
use self::time_h::time;
pub use self::time_t_h::time_t;
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __off64_t, __off_t, __time_t, __uint16_t,
    __uint32_t, __uint64_t, __uint8_t,
};
pub use self::utils_h::{streq, streq_not_null};
pub use self::xkbcommon_h::{
    xkb_context_flags, xkb_context_include_path_append, xkb_context_new, xkb_context_unref,
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keymap_get_as_string,
    xkb_keymap_key_get_syms_by_level, xkb_keymap_new_from_buffer, xkb_keymap_new_from_string,
    xkb_keymap_serialize_flags, xkb_keymap_unref, xkb_keysym_t, xkb_layout_index_t,
    xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t,
    xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_CONTEXT_NO_DEFAULT_INCLUDES, XKB_CONTEXT_NO_ENVIRONMENT_NAMES, XKB_CONTEXT_NO_FLAGS,
    XKB_CONTEXT_NO_SECURE_GETENV, XKB_KEYCODE_MAX, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_KEYMAP_SERIALIZE_EXPLICIT, XKB_KEYMAP_SERIALIZE_KEEP_UNUSED, XKB_KEYMAP_SERIALIZE_NO_FLAGS,
    XKB_KEYMAP_SERIALIZE_PRETTY, XKB_KEYMAP_USE_ORIGINAL_FORMAT, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
    XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
    XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED,
    XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keymap_test_data {
    pub keymap: *const ::core::ffi::c_char,
    pub expected: *const ::core::ffi::c_char,
    pub skip: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keymap_simple_test_data {
    pub keymap: *const ::core::ffi::c_char,
    pub valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub keymap: *const ::core::ffi::c_char,
    pub path: *const ::core::ffi::c_char,
    pub format: xkb_keymap_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub min: xkb_keycode_t,
    pub max: xkb_keycode_t,
    pub max_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub keymap: *const ::core::ffi::c_char,
    pub expected_v1: *const ::core::ffi::c_char,
    pub expected_v2: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub keymap: *const ::core::ffi::c_char,
    pub expected: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub format: C2Rust_Unnamed_18,
    pub output: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub input: xkb_keymap_format,
    pub output: xkb_keymap_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub path: *const ::core::ffi::c_char,
    pub format: xkb_keymap_format,
    pub serialize_flags: xkb_keymap_serialize_flags,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keymap_multi_versions_test_data {
    pub keymap: *const ::core::ffi::c_char,
    pub no_output: bool,
    pub lenient: bool,
    pub c2rust_unnamed: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_20 {
    pub c2rust_unnamed: C2Rust_Unnamed_22,
    pub c2rust_unnamed_0: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub expected_v1_1: *const ::core::ffi::c_char,
    pub expected_v1_2: *const ::core::ffi::c_char,
    pub expected_v2_2: *const ::core::ffi::c_char,
    pub expected_v2_1: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub compiles_v1: bool,
    pub compiles_v2: bool,
}
static mut bounds: [C2Rust_Unnamed_14; 2] = [C2Rust_Unnamed_14 {
    min: 0,
    max: 0,
    max_count: 0,
}; 2];
unsafe extern "C" fn compile_buffer(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut buf: *const ::core::ffi::c_char,
    mut len: size_t,
    mut private: *mut ::core::ffi::c_void,
) -> *mut xkb_keymap {
    unsafe {
        let flags: xkb_keymap_compile_flags = (if !private.is_null() {
            *(private as *mut xkb_keymap_compile_flags) as ::core::ffi::c_uint
        } else {
            XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int as ::core::ffi::c_uint
        }) as xkb_keymap_compile_flags;
        return test_compile_buffer2(context, format, flags, buf, len);
    }
}
unsafe extern "C" fn test_encodings(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let utf8_with_bom: [::core::ffi::c_char; 18] = ::core::mem::transmute::<
            [u8; 18],
            [::core::ffi::c_char; 18],
        >(*b"\xEF\xBB\xBFxkb_keymap {};\0");
        keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const utf8_with_bom as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 18]>() as size_t,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                56 as ::core::ffi::c_uint,
                b"void test_encodings(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        let utf16_le: [::core::ffi::c_char; 29] =
            ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
                *b"x\0k\0b\0_\0k\0e\0y\0m\0a\0p\0 \0{\0}\0;\0\0",
            );
        keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const utf16_le as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 29]>() as size_t,
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                63 as ::core::ffi::c_uint,
                b"void test_encodings(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let utf16_le_with_bom: [::core::ffi::c_char; 31] =
            ::core::mem::transmute::<[u8; 31], [::core::ffi::c_char; 31]>(
                *b"\xFF\xFEx\0k\0b\0_\0k\0e\0y\0m\0a\0p\0 \0{\0}\0;\0\0",
            );
        keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const utf16_le_with_bom as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as size_t,
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                b"void test_encodings(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let utf16_be: [::core::ffi::c_char; 29] =
            ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
                *b"\0x\0k\0b\0_\0k\0e\0y\0m\0a\0p\0 \0{\0}\0;\0",
            );
        keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const utf16_be as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 29]>() as size_t,
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                76 as ::core::ffi::c_uint,
                b"void test_encodings(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn test_floats(mut ctx: *mut xkb_context) {
    unsafe {
        let tests: [keymap_simple_test_data; 4] = [
            keymap_simple_test_data {
                keymap: b"xkb_keymap {\n  xkb_geometry {\n    width=123.456;\n    width=123.0;\n    width=123.;\n    width=01.234;\n    width=01.0;\n    width=01.;\n    width=001.234;\n    width=001.0;\n    width=001.;\n  };};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                valid: true,
            },
            keymap_simple_test_data {
                keymap: b"xkb_keymap { xkb_geometry { width=.123; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                valid: false,
            },
            keymap_simple_test_data {
                keymap: b"xkb_keymap { xkb_geometry { width=1,23; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                valid: false,
            },
            keymap_simple_test_data {
                keymap: b"xkb_keymap { xkb_geometry { width=1.23e2; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                valid: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_simple_test_data; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_simple_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_floats\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut keymap: *mut xkb_keymap = test_compile_buffer(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                tests[k as usize].keymap,
                strlen(tests[k as usize].keymap),
            );
            if tests[k as usize].valid as ::core::ffi::c_int
                ^ keymap.is_null() as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"tests[k].valid ^ !keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    128 as ::core::ffi::c_uint,
                    b"void test_floats(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_component_syntax_error(mut ctx: *mut xkb_context) {
    unsafe {
        let keymaps: [*const ::core::ffi::c_char; 4] = [
            b"xkb_keymap {  xkb_keycodes {};};};\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes {};  xkb_types {};  xkb_compat {};  xkb_symbols {};};};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes {};}  xkb_types {};  xkb_compat {};  xkb_symbols {};};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes {};;  xkb_types {};  xkb_compat {};  xkb_symbols {};};\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_component_syntax_error\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut keymap: *const xkb_keymap = test_compile_buffer(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                keymaps[k as usize],
                strlen(keymaps[k as usize]),
            );
            if keymap.is_null() {
            } else {
                __assert_fail(
                    b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    170 as ::core::ffi::c_uint,
                    b"void test_component_syntax_error(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_optional_components(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymaps: [keymap_test_data; 9] = [
            keymap_test_data {
                keymap: b"xkb_keymap {};\0".as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap { xkb_keycodes {}; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap { xkb_types {}; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap { xkb_compat {}; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap { xkb_symbols {}; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat { indicator \"XXX\" { modifiers=Lock; }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-no-real-led.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { key <> { [a] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };  xkb_symbols { key <> { [a], type=\"XXX\" }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-basic.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };  xkb_symbols { key <> { vmods=XXX, [a] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 9]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_optional_components\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_optional_components\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    246 as ::core::ffi::c_uint,
                    b"void test_optional_components(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_bidi_chars(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let keymaps: [keymap_test_data; 3] = [
            keymap_test_data {
                keymap: b"\xE2\x80\x8Exkb_keymap {};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"\xE2\x80\x8Fxkb_keymap {};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b" \xE2\x80\x8Fxkb_keymap\xE2\x80\x8E\xE2\x80\x8F\n\xE2\x80\x8E{ \xE2\x80\x8Exkb_keycodes \xE2\x80\x8F{ <>\xE2\x80\x8E= \xE2\x80\x8F1\xE2\x80\x8E;\xE2\x80\x8F}\xE2\x80\x8E ;};\xE2\x80\x8E\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/bidi.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_bidi_chars\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_bidi_chars\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_uint,
                    b"void test_bidi_chars(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_recursive_includes(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let keymaps: [*const ::core::ffi::c_char; 16] = [
            b"Keycodes: recursive\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev+recursive\" };  xkb_types { include \"complete\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"pc\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Keycodes: recursive(bar)\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev+recursive(bar)\" };  xkb_types { include \"complete\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"pc\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Key types: recursive\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"recursive\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"pc\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Key types: recursive(bar)\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"recursive(bar)\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"pc\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Compat: recursive\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"recursive\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"pc\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Compat: recursive(bar)\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"complete\" };  xkb_compat { include \"recursive(bar)\" };  xkb_symbols { include \"pc\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Symbols: recursive\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"complete\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"recursive\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"Symbols: recursive(bar)\0".as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"complete\" };  xkb_compat { include \"complete\" };  xkb_symbols { include \"recursive(bar)\" };};\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u %s ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_recursive_includes\0".as_ptr() as *const ::core::ffi::c_char,
                k,
                keymaps[k as usize],
            );
            k = k.wrapping_add(1);
            keymap = test_compile_buffer(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                keymaps[k as usize],
                strlen(keymaps[k as usize]),
            );
            if keymap.is_null() {
            } else {
                __assert_fail(
                    b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    357 as ::core::ffi::c_uint,
                    b"void test_recursive_includes(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_include_paths(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let simple_str: [::core::ffi::c_char; 162] = ::core::mem::transmute::<
            [u8; 162],
            [::core::ffi::c_char; 162],
        >(
            *b"xkb_keymap {\n  xkb_keycodes \"test\" { include \"evdev\" };\n  xkb_types    \"test\" { include \"complete\" };\n  xkb_symbols  \"test\" { include \"pc+us(basic)+level5\" };\n};\0",
        );
        keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const simple_str as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 162]>() as size_t,
        );
        let mut expected: *mut ::core::ffi::c_char =
            xkb_keymap_get_as_string(keymap, XKB_KEYMAP_USE_ORIGINAL_FORMAT);
        xkb_keymap_unref(keymap);
        static mut keymaps: [*const ::core::ffi::c_char; 2] = [
            b"xkb_keymap {\n  xkb_keycodes \"test\" { include \"%S/evdev\" };\n  xkb_types    \"test\" { include \"%S/complete\" };\n  xkb_symbols  \"test\" { include \"%S/pc+%S/us(basic)+%S/level5\" };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n  xkb_keycodes \"test\" { include \"/home/rano/Public/libxkbcommon/test/data/keycodes/evdev\" };\n  xkb_types    \"test\" { include \"/home/rano/Public/libxkbcommon/test/data/types/complete\" };\n  xkb_symbols  \"test\" { include \"/home/rano/Public/libxkbcommon/test/data/symbols/pc+/home/rano/Public/libxkbcommon/test/data/symbols/us(basic)+/home/rano/Public/libxkbcommon/test/data/symbols/level5\" };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        setenv(
            b"XKB_CONFIG_ROOT\0".as_ptr() as *const ::core::ffi::c_char,
            b"..\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            keymaps[0 as ::core::ffi::c_int as usize],
            strlen(keymaps[0 as ::core::ffi::c_int as usize]),
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                403 as ::core::ffi::c_uint,
                b"void test_include_paths(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        setenv(
            b"XKB_CONFIG_ROOT\0".as_ptr() as *const ::core::ffi::c_char,
            TEST_XKB_CONFIG_ROOT.as_ptr(),
            1 as ::core::ffi::c_int,
        );
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_include_paths\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            keymap = test_compile_buffer(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                keymaps[k as usize],
                strlen(keymaps[k as usize]),
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    411 as ::core::ffi::c_uint,
                    b"void test_include_paths(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let mut got: *mut ::core::ffi::c_char =
                xkb_keymap_get_as_string(keymap, XKB_KEYMAP_USE_ORIGINAL_FORMAT);
            let __cond: bool = streq_not_null(expected, got) as bool;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: test_include_paths. Expected \"%s\", got: \"%s\"\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    expected,
                    got,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        414 as ::core::ffi::c_uint,
                        b"void test_include_paths(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            free(got as *mut ::core::ffi::c_void);
            xkb_keymap_unref(keymap);
            k = k.wrapping_add(1);
        }
        free(expected as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn test_include_default_maps(mut update_output_files: bool) {
    unsafe {
        let mut ctx: *mut xkb_context = xkb_context_new(XKB_CONTEXT_NO_DEFAULT_INCLUDES);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                426 as ::core::ffi::c_uint,
                b"void test_include_default_maps(_Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut include_path: *mut ::core::ffi::c_char =
            test_get_path(b"extra\0".as_ptr() as *const ::core::ffi::c_char);
        if !include_path.is_null() {
        } else {
            __assert_fail(
                b"include_path\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                429 as ::core::ffi::c_uint,
                b"void test_include_default_maps(_Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_context_include_path_append(ctx, include_path) != 0 {
        } else {
            __assert_fail(
                b"xkb_context_include_path_append(ctx, include_path)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                430 as ::core::ffi::c_uint,
                b"void test_include_default_maps(_Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        free(include_path as *mut ::core::ffi::c_void);
        include_path = test_get_path(b"\0".as_ptr() as *const ::core::ffi::c_char);
        if !include_path.is_null() {
        } else {
            __assert_fail(
                b"include_path\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                434 as ::core::ffi::c_uint,
                b"void test_include_default_maps(_Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_context_include_path_append(ctx, include_path) != 0 {
        } else {
            __assert_fail(
                b"xkb_context_include_path_append(ctx, include_path)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                435 as ::core::ffi::c_uint,
                b"void test_include_default_maps(_Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        free(include_path as *mut ::core::ffi::c_void);
        let keymaps: [keymap_test_data; 4] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <CAPS> = 66; };\n  xkb_symbols { include \"capslock\" };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/include-capslock-system.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <CAPS> = 66; };\n  xkb_symbols { include \"capslock(custom)\" };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/include-capslock-custom.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <RALT> = 108; <LVL3> = 92; };\n  xkb_types { include \"basic\" };\n  xkb_symbols { include \"level3\" };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/include-level3-explicit-default.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { include \"banana\" };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/include-banana-implicit-default.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_include_default_maps\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_include_default_maps\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    484 as ::core::ffi::c_uint,
                    b"void test_include_default_maps(_Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
        xkb_context_unref(ctx);
    }
}
unsafe extern "C" fn test_alloc_limits(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let keymaps: [keymap_test_data; 9] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 0xfffffffe; };\n  xkb_symbols { key <> {[a]}; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <xxx> = 0xfffffffe;\n    <> = 0xfffffffd;\n    <> = 0xfffffffe;\n  };\n  xkb_symbols { key <> {[a]}; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 7;\n    <> = 8;\n    <> = 0xfffffffd;\n    <> = 0xfffffffe;\n  };\n  xkb_symbols { key <> {[a]}; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 0xfffffffd;\n    <> = 0xfffffffe;\n    <> = 7;\n    <> = 8;\n  };\n  xkb_symbols { key <> {[a]}; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-2.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <min> = 0xfffffffd;\n    <max> = 0xfffffffe;\n  };\n  xkb_symbols {\n    key <min> {[a]};\n    key <max> {[b]};\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-3.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <max> = 0xfffffffe;\n    <min> = 0xfffffffd;\n  };\n  xkb_symbols {\n    key <min> {[a]};\n    key <max> {[b]};\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-3.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <a> = 0xf;\n    <d> = 0x4000;\n    <e> = 0xfffffffe;\n    <c> = 0x2000;\n    <b> = 0xfff;\n  };\n  xkb_symbols {\n    key <a> {[a]};\n    key <b> {[b]};\n    key <c> {[c]};\n    key <d> {[d]};\n    key <e> {[e]};\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/high-keycodes-4.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"X\" { map[none] = 0xfffffffe; };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"X\" {levelname[0xfffffffe]=\"x\";};\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 9]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_alloc_limits\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_alloc_limits\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    619 as ::core::ffi::c_uint,
                    b"void test_alloc_limits(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_integers(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let not_null_terminated: [::core::ffi::c_char; 1] = ['1' as i32 as ::core::ffi::c_char];
        if test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const not_null_terminated as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!test_compile_buffer(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, not_null_terminated, sizeof(not_null_terminated))\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                630 as ::core::ffi::c_uint,
                b"void test_integers(struct xkb_context *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let skipOverflow: bool = ::core::mem::size_of::<int64_t>() as usize
            >= ::core::mem::size_of::<::core::ffi::c_longlong>() as usize;
        if skipOverflow {
            fprintf(
                stderr,
                b"[WARNING] %s: cannot use checked arithmetic\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"test_integers\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let keymaps: [keymap_test_data; 8] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 0x10000000000000000;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_compat {\n  };\n  xkb_symbols {\n    key <> {\n      [MovePointer(x=-0x7fffffffffffffff - 2,\n                   y=0)]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/integers-overflow.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: skipOverflow,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_compat {\n  };\n  xkb_symbols {\n    key <> {\n      [MovePointer(x=-0x7fffffffffffffff * 2,\n                   y=0)]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/integers-overflow.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: skipOverflow,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_compat {\n  };\n  xkb_symbols {\n    key <> {\n      [MovePointer(x=0,\n                   y=0x7fffffffffffffff + 1)]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/integers-overflow.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: skipOverflow,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_compat {\n  };\n  xkb_symbols {\n    key <> {\n      [MovePointer(x=0,\n                   y=0x7fffffffffffffff * 2)]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/integers-overflow.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: skipOverflow,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 1;\n    indicator 32 = \"xxx\";\n  };\n  xkb_compat {\n    group 0xffffffff = Mod5;\n  };\n  xkb_types {\n    type \"ONE_LEVEL\" {};    type \"TWO_LEVEL\" { modifiers = Shift; map[Shift] = 2;};\n  };\n  xkb_symbols {\n    key <> {\n      actions[1 + -~0x100000001 / 0x100000000]=\n      [MovePointer(x=0x100000000 - 0xfffffffe,\n                   y=~-0x7fff * 0x30000 / 0x2ffff)],\n      [MovePointer(x=-9223372036854775807\n                     +9223372036854775807)],\n      [SetGroup(group=+0), LockGroup(group=+(Last-4))],\n      [MovePointer(x=+0, y=-(1-1))]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/integers.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols {\n    key <> {\n      [MovePointer(x=-9223372036854775808\n                     +9223372036854775807)]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <0> = 10;\n    <1> = 11;\n    <2> = 12;\n    <a> = 20;\n    <b> = 21;\n    <c> = 22;\n  };\n  xkb_types {\n    type \"FOUR_LEVEL\" {\n      modifiers = Shift + Control;\n      map[None] = 01;\n      map[Shift] = 2;\n      map[Control] = 3;\n    };\n  }\n;  xkb_compat {\n    interpret 0   { action = LockGroup(group=2); };\n    interpret 00  { action = LockGroup(group=3); };\n    interpret 0x0 { action = LockGroup(group=4); };\n    interpret 1   { action = LockGroup(group=-1); };\n    interpret 01  { action = LockGroup(group=1); };\n    interpret 2   { action = LockGroup(group=-2); };\n    interpret 02  { action = LockGroup(group=2); };\n    interpret 3   { action = LockGroup(group=-3); };\n    interpret 0x3 { action = LockGroup(group=3); };\n  };\n  xkb_symbols {\n    key <0> { [ 0, 00, 0x0] };\n    key <1> { [ 1, 01, 0x1] };\n    key <2> { [ 2, 02, 0x2] };\n    key <a> { [ 3   ] };\n    key <b> { [ 04  ] };\n    key <c> { [ 0x5 ] };\n    modifier_map Shift   { 0,   3   };\n    modifier_map Lock    { 00,  04  };\n    modifier_map Control { 0x0, 0x5 };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/digits.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_integers\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if keymaps[k as usize].skip {
                fprintf(
                    stderr,
                    b"INFO: skip test\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"test_integers\0".as_ptr() as *const ::core::ffi::c_char,
                    keymaps[k as usize].keymap,
                    strlen(keymaps[k as usize].keymap),
                    keymaps[k as usize].expected,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        819 as ::core::ffi::c_uint,
                        b"void test_integers(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_keycodes(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let keymaps: [keymap_test_data; 21] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 0;\n    override <A> = 1;\n    augment  <A> = 300;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 300;\n    override <A> = 1;\n    augment  <A> = 0;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 0;\n    override <A> = 1;\n    override <A> = 301;\n    override <A> = 300;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-2.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 300;\n    <B> = 1;\n    augment  <B> = 301;\n    override <A> = 1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 0;\n    <B> = 1;\n    augment  <B> = 300;\n    override <A> = 1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 1;\n    <B> = 0;\n    override <B> = 300;\n    augment  <A> = 0;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-multiple-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 301;\n    <B> = 300;\n    override <A> = 1;\n    augment  <B> = 302;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-multiple-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 1;\n    include \"high\"\n    <B> = 2;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-multiple-2.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 0x10000;\n    <B> = 0x10001;\n    <A> = 1;\n    <B> = 0x10002;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-multiple-3.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <B> = 1;\n    <A> = 1;\n    alias <C> = <B>;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    alias <C> = <B>;\n    <B> = 1;\n    <A> = 1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <B> = 1;\n    <A> = 1;\n    alias <C> = <B>;\n    <B> = 2;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-aliases-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    alias <C> = <B>;\n    <B> = 1;\n    <A> = 1;\n    <B> = 2;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-aliases-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    alias <A> = <B>;\n    <A> = 1;\n    <B> = 300;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-multiple-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    alias <C> = <B>;\n    augment <C> = 3;\n    <A> = 1;\n    <B> = 2;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-aliases-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 1;\n    <C> = 3;\n    alias <C> = <B>;\n    <B> = 2;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-aliases-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 1;\n    augment alias <A> = <B>;\n    <B> = 300;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-multiple-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <A> = 1;\n    <B> = 2;\n    alias <B> = <C>;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <B> = 1;\n    alias <A> = <B>;\n    <A> = 1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-bounds-single-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    alias <A> = <X>;\n    alias <B> = <X>;\n    alias <C> = <X>;\n    alias <D> = <X>;\n    alias <E> = <X>;\n    alias <F> = <X>;\n    alias <G> = <X>;\n    alias <H> = <X>;\n    alias <I> = <X>;\n    alias <J> = <X>;\n    alias <K> = <X>;\n    alias <L> = <X>;\n    alias <M> = <X>;\n    alias <N> = <X>;\n    alias <O> = <X>;\n    alias <P> = <X>;\n    <X> = 1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-aliases-2.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    alias <A> = <1>;\n    alias <B> = <1>;\n    alias <C> = <1>;\n    alias <D> = <1>;\n    alias <E> = <1>;\n    <1> = 1;\n    <2> = 2;\n    <3> = 3;\n    <4> = 4;\n    <5> = 5;\n    <6> = 6;\n    <7> = 7;\n    <8> = 8;\n    <9> = 9;\n    <10> = 10;\n    <11> = 11;\n    <12> = 12;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/keycodes-aliases-3.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 21]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: key bounds #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let ctx2: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG) as *mut xkb_context;
            if !ctx2.is_null() {
            } else {
                __assert_fail(
                    b"ctx2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1131 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_compile_output(
                ctx2,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx2, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1136 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_context_unref(ctx2);
            k = k.wrapping_add(1);
        }
        static mut max_iterations: ::core::ffi::c_uint = 1000 as ::core::ffi::c_uint;
        let mut buffer: [::core::ffi::c_char; 2048] = ::core::mem::transmute::<
            [u8; 2048],
            [::core::ffi::c_char; 2048],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < max_iterations {
            let mut available: size_t =
                ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t;
            let mut buf: *mut ::core::ffi::c_char = &raw mut buffer as *mut ::core::ffi::c_char;
            let mut count: ::core::ffi::c_int = snprintf(
                buf,
                available,
                b"default xkb_keymap { xkb_keycodes {\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if count > 0 as ::core::ffi::c_int && (count as size_t) < available {
            } else {
                __assert_fail(
                    b"count > 0 && (size_t) count < available\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1148 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            available = available.wrapping_sub(count as size_t);
            buf = buf.offset(count as isize);
            let mut keycodes: [xkb_keycode_t; 13] = [
                0 as ::core::ffi::c_int as xkb_keycode_t,
                0,
                0,
                0,
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
            let mut keycode_index: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut b: size_t = 0 as size_t;
            while b
                < (::core::mem::size_of::<[C2Rust_Unnamed_14; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_14>() as usize)
            {
                if bounds[b as usize].min < bounds[b as usize].max {
                } else {
                    __assert_fail(
                        b"bounds[b].min < bounds[b].max\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1173 as ::core::ffi::c_uint,
                        b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                let keycode_count: ::core::ffi::c_uint = (rand() as ::core::ffi::c_uint)
                    .wrapping_rem(
                        bounds[b as usize]
                            .max_count
                            .wrapping_add(1 as ::core::ffi::c_uint),
                    );
                let mut k_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                while k_0 < keycode_count {
                    let kc: xkb_keycode_t = bounds[b as usize].min.wrapping_add(
                        (rand() as xkb_keycode_t).wrapping_rem(
                            bounds[b as usize]
                                .max
                                .wrapping_sub(bounds[b as usize].min)
                                .wrapping_add(1 as xkb_keycode_t),
                        ),
                    );
                    if (keycode_index as usize)
                        < (::core::mem::size_of::<[xkb_keycode_t; 13]>() as usize)
                            .wrapping_div(::core::mem::size_of::<xkb_keycode_t>() as usize)
                    {
                    } else {
                        __assert_fail(
                            b"keycode_index < ARRAY_SIZE(keycodes)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1179 as ::core::ffi::c_uint,
                            b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    let c2rust_fresh0 = keycode_index;
                    keycode_index = keycode_index.wrapping_add(1);
                    keycodes[c2rust_fresh0 as usize] = kc;
                    count = snprintf(
                        buf,
                        available,
                        b"<%u> = 0x%x;\n\0".as_ptr() as *const ::core::ffi::c_char,
                        kc,
                        kc,
                    );
                    if count > 0 as ::core::ffi::c_int && (count as size_t) < available {
                    } else {
                        __assert_fail(
                            b"count > 0 && (size_t) count < available\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1183 as ::core::ffi::c_uint,
                            b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    available = available.wrapping_sub(count as size_t);
                    buf = buf.offset(count as isize);
                    k_0 = k_0.wrapping_add(1);
                }
                b = b.wrapping_add(1);
            }
            count = snprintf(
                buf,
                available,
                b"}; xkb_symbols {\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if count > 0 as ::core::ffi::c_int && (count as size_t) < available {
            } else {
                __assert_fail(
                    b"count > 0 && (size_t) count < available\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1190 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            available = available.wrapping_sub(count as size_t);
            buf = buf.offset(count as isize);
            let mut k_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_1 < keycode_index {
                let kc_0: xkb_keycode_t = keycodes[k_1 as usize];
                count = snprintf(
                    buf,
                    available,
                    b"key <%u> { [ 0x%x ] };\n\0".as_ptr() as *const ::core::ffi::c_char,
                    kc_0,
                    kc_0 >> 3 as ::core::ffi::c_int,
                );
                if count > 0 as ::core::ffi::c_int && (count as size_t) < available {
                } else {
                    __assert_fail(
                        b"count > 0 && (size_t) count < available\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1199 as ::core::ffi::c_uint,
                        b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                available = available.wrapping_sub(count as size_t);
                buf = buf.offset(count as isize);
                k_1 = k_1.wrapping_add(1);
            }
            count = snprintf(
                buf,
                available,
                b"}; };\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if count > 0 as ::core::ffi::c_int && (count as size_t) < available {
            } else {
                __assert_fail(
                    b"count > 0 && (size_t) count < available\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1205 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let keymap: *mut xkb_keymap = xkb_keymap_new_from_string(
                ctx,
                &raw mut buffer as *mut ::core::ffi::c_char,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                XKB_KEYMAP_COMPILE_STRICT_MODE,
            ) as *mut xkb_keymap;
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1210 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let mut k_2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_2 < keycode_index {
                let kc_1: xkb_keycode_t = keycodes[k_2 as usize];
                let mut ks: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
                count = xkb_keymap_key_get_syms_by_level(
                    keymap,
                    kc_1,
                    0 as xkb_layout_index_t,
                    0 as xkb_level_index_t,
                    &raw mut ks,
                );
                let __cond: bool = count == 1 as ::core::ffi::c_int;
                if !__cond {
                    fprintf(
                        stderr,
                        b"Assertion failure: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        count,
                    );
                    if __cond as ::core::ffi::c_int != 0 {
                    } else {
                        __assert_fail(
                            b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1215 as ::core::ffi::c_uint,
                            b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                let expected: xkb_keysym_t = kc_1 as xkb_keysym_t >> 3 as ::core::ffi::c_int;
                let __cond_0: bool = *ks == expected;
                if !__cond_0 {
                    fprintf(
                        stderr,
                        b"Assertion failure: <%u> 0x%x != 0x%x\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc_1,
                        *ks,
                        expected,
                    );
                    if __cond_0 as ::core::ffi::c_int != 0 {
                    } else {
                        __assert_fail(
                            b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1219 as ::core::ffi::c_uint,
                            b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                k_2 = k_2.wrapping_add(1);
            }
            xkb_keymap_unref(keymap);
            i = i.wrapping_add(1);
        }
        static mut long_names_1: [::core::ffi::c_char; 1265] = unsafe {
            ::core::mem::transmute::<
                [u8; 1265],
                [::core::ffi::c_char; 1265],
            >(
                *b"xkb_keymap {\n  xkb_keycodes {\n    <0008> = 8;\n    <!09>  = 9;\n    <long-name-000axxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> = 0x000a;\n    <long-name-0fffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> = 0x0fff;\n    <long-name-ffffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> = 0xffff;\n    alias <long-alias-000axxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> =           <long-name-000axxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>;\n    alias <long-alias-0fffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> =           <long-name-0fffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>;\n    alias <long-alias-ffffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> =           <long-name-ffffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>;\n  };\n  xkb_symbols {\n    key <0008> { [0x8] };\n    key <!09>  { [0x9] };\n    key <long-name-000axxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> {[0x000a]};\n    key <long-name-0fffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> {[0x0fff]};\n    key <long-name-ffffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> {[0xffff]};\n    modmap Shift { <0008>, <!09> };\n    modmap Lock {\n      <long-alias-000axxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>,\n      <long-alias-0fffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>,\n      <long-alias-ffffxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>\n    };\n  };\n};\0",
            )
        };
        static mut long_names_2: [::core::ffi::c_char; 707] = unsafe {
            ::core::mem::transmute::<
                [u8; 707],
                [::core::ffi::c_char; 707],
            >(
                *b"xkb_keymap {\n  xkb_keycodes {\n    <!008> = 8;\n    <#009> = 9;\n    <$010> = 10;\n    <%011> = 11;\n    <&012> = 12;\n    <'013> = 13;\n    <(014> = 14;\n    <)015> = 15;\n    <*016> = 16;\n    <+017> = 17;\n    <,018> = 18;\n    <-019> = 19;\n    <.020> = 20;\n    </021> = 21;\n    <0022> = 22;\n    <long-name-0100xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> = 100;\n    <long-name-0101xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> = 101;\n    alias <long-alias-0100xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> =           <long-name-0100xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>;\n    alias <long-alias-0101xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx> =           <long-name-0101xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>;\n  };\n};\0",
            )
        };
        static mut tests: [C2Rust_Unnamed_13; 4] = unsafe {
            [
                C2Rust_Unnamed_13 {
                    keymap: &raw const long_names_1 as *const ::core::ffi::c_char,
                    path: b"keymaps/keycodes-long-names-1-v1.xkb\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    format: XKB_KEYMAP_FORMAT_TEXT_V1,
                },
                C2Rust_Unnamed_13 {
                    keymap: &raw const long_names_1 as *const ::core::ffi::c_char,
                    path: b"keymaps/keycodes-long-names-1-v2.xkb\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    format: XKB_KEYMAP_FORMAT_TEXT_V2,
                },
                C2Rust_Unnamed_13 {
                    keymap: &raw const long_names_2 as *const ::core::ffi::c_char,
                    path: b"keymaps/keycodes-long-names-2.xkb\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    format: XKB_KEYMAP_FORMAT_TEXT_V1,
                },
                C2Rust_Unnamed_13 {
                    keymap: &raw const long_names_2 as *const ::core::ffi::c_char,
                    path: b"keymaps/keycodes-long-names-2.xkb\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    format: XKB_KEYMAP_FORMAT_TEXT_V2,
                },
            ]
        };
        let mut t: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (t as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_13; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_13>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: long key names #%u ***\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"test_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                tests[t as usize].format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                tests[t as usize].keymap,
                strlen(tests[t as usize].keymap),
                tests[t as usize].path,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, tests[t].format, compile_buffer, NULL, __func__, tests[t].keymap, strlen(tests[t].keymap), tests[t].path, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1315 as ::core::ffi::c_uint,
                    b"void test_keycodes(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            t = t.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_masks(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let keymaps: [keymap_test_data; 6] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers Test1 = -1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers Test1 = 0x100000000;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers Test1 = ~0x100000000;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers Test1 = !Mod1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_types {\n    virtual_modifiers Test01 = 0;\n    virtual_modifiers Test02 = 0xffffffff;\n    virtual_modifiers Test11 = 0xf0 + 0x0f;\n    virtual_modifiers Test12 = 0xff - 0x0f;\n    virtual_modifiers Test13 = ~0xf;\n    virtual_modifiers Test14 = ~none;\n    virtual_modifiers Test15 = ~all;\n    virtual_modifiers Test16 = ~0xffffffff;\n    virtual_modifiers Test17 = all - ~Mod1 + Mod2;\n    type \"XXX\" {\n      modifiers = Test12;\n      map[Test12 - Mod5] = 2;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/masks.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n    xkb_keycodes { <a> = 38; };\n    xkb_symbols {\n        virtual_modifiers X = 0xf0000000;\n        key <a> { [ SetMods(mods = 0x00001100) ] };\n    };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/masks-extra-bits.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_masks\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_masks\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1405 as ::core::ffi::c_uint,
                    b"void test_masks(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_interpret(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let keymaps: [keymap_test_data; 5] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat { interpret \"\" { repeat = false; } };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat { interpret \"\xFF\" { repeat = false; }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat { interpret \"\\u{0}\" { repeat = false; }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat { interpret \"ab\" { repeat = false; }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n   interpret 0x1     { repeat = false; };\n   interpret 0xB     { repeat = false; };\n   interpret Shift_L { repeat = false; };\n   interpret a       { repeat = false; };\n   interpret \"\xC3\xA4\"   { repeat = false; };\n   interpret \"\xE2\x9C\xA8\"  { repeat = false; };\n   interpret \"\xF0\x9F\x8E\xBA\"  { repeat = false; };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/compat-interpret.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_interpret\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_interpret\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1469 as ::core::ffi::c_uint,
                    b"void test_interpret(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_group_indices_names(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymaps: [C2Rust_Unnamed_15; 10] = [
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 1;\n    indicator 1 = \"1\";\n    indicator 2 = \"2\";\n  };\n  xkb_compat {\n    interpret a { action = SetGroup(group=Group1);  };\n    interpret b { action = SetGroup(group=-Group2); };\n    interpret c { action = SetGroup(group=+Group3); };\n    indicator \"1\" { groups = Group4; };\n    indicator \"2\" { groups = All-Group1; };\n  };\n  xkb_symbols {\n    name[Group1] = \"1\";\n    name[Group2] = \"2\";\n    name[Group3] = \"3\";\n    name[Group4] = \"4\";\n    key <> {\n      groupsredirect=Group4,\n      symbols[Group1]=[a],\n      symbols[Group2]=[b],\n      symbols[Group3]=[c],\n      symbols[Group4]=[d]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: b"keymaps/group-index-names-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected_v2: b"keymaps/group-index-names-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 1;\n    indicator 1 = \"1\";\n    indicator 2 = \"2\";\n  };\n  xkb_compat {\n    interpret a { action = SetGroup(group=Group32);  };\n    interpret b { action = SetGroup(group=-Group32); };\n    interpret c { action = SetGroup(group=+Group32); };\n    indicator \"1\" { groups = Group32; };\n    indicator \"2\" { groups = All-Group32; };\n  };\n  xkb_symbols {\n    name[Group1] = \"1\";\n    name[Group4] = \"4\";\n    name[Group5] = \"5\";\n    name[Group8] = \"8\";\n    name[Group32] = \"32\";\n    key <> {\n      groupsredirect=Group32,\n      symbols[Group1]=[a],\n      symbols[Group4]=[b],\n      symbols[Group5]=[c],\n      symbols[Group8]=[d],\n      symbols[Group32]=[e]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: b"keymaps/group-index-names-2.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_compat { interpret a { action = SetGroup(group=Group33); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes { indicator 1 = \"1\"; };\n  xkb_compat { indicator \"1\" { groups = Group33; }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { name[Group33] = \"1\"; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { groupsredirect=Group33 };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { symbols[Group33] = [a] };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    interpret ISO_First_Group {\n      action= LockGroup(group=First);\n    };\n    interpret ISO_Last_Group {\n      action= LockGroup(group=Last);\n    };\n    indicator \"First group\" {\n      groups = First;\n    };\n    indicator \"Last group\" {\n      groups = Last;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: b"keymaps/group-index-names-3.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected_v2: b"keymaps/group-index-names-3.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_compat {\n    SetGroup.group = Last;\n    interpret ISO_First_Group {\n      action= LockGroup(group=First);\n    };\n    interpret ISO_Last_Group {\n      action= SetGroup();\n    };\n    indicator \"Later groups\" {\n      groups = All - First;\n    };\n    indicator \"Last group\" {\n      groups = Last;\n    };\n    indicator \"All but last group\" {\n      groups = All - Last;\n    };\n    indicator \"Y\" {\n      groups = Last + First;\n    };\n  };\n  xkb_types {\n    type \"ONE_LEVEL\" {};\n    type \"TWO_LEVEL\" {\n      modifiers = Shift;\n      map[Shift] = 2;\n    };\n  };  xkb_symbols {\n    name[first] = \"1\";\n    SetGroup.group = +(Last + 1);\n    key.groupsRedirect = Last;\n    key <> {\n      symbols[FIRST] = [1],\n      actions[First] = [SetGroup(group=Last), SetGroup(group=-Last)],\n      symbols[2]     = [2],\n      actions[2]     = [SetGroup(group=Last - First), SetGroup()],\n      actions[3]     = [SetGroup(group=First), SetGroup(group=-First)]\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: b"keymaps/group-index-names-4.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected_v2: b"keymaps/group-index-names-4.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed_15 {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { symbols[Last] = [a] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected_v1: ::core::ptr::null::<::core::ffi::c_char>(),
                expected_v2: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_15; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_15>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_group_indices_names\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_group_indices_names\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected_v1,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected_v1, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1681 as ::core::ffi::c_uint,
                    b"void test_group_indices_names(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V2,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_group_indices_names\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected_v2,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected_v2, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1686 as ::core::ffi::c_uint,
                    b"void test_group_indices_names(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_level_indices_names(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymaps: [C2Rust_Unnamed_16; 3] = [
            C2Rust_Unnamed_16 {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"X\" {\n      modifiers = Shift;\n      map[Shift] = Level2048;\n      level_name[Level2048] = \"x\";\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/level-index-names.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed_16 {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"X\" {\n      modifiers = Shift;\n      map[Shift] = Level2049;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_16 {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"X\" {\n      modifiers = Shift;\n      level_name[Level2049] = \"x\";\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_16; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_16>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_level_indices_names\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_level_indices_names\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1744 as ::core::ffi::c_uint,
                    b"void test_level_indices_names(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_multi_keysyms_actions(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymaps: [keymap_test_data; 33] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <01> = 1;\n    <02> = 2;\n    <03> = 3;\n    <04> = 4;\n    <05> = 5;\n    <06> = 6;\n    <07> = 7;\n    <08> = 8;\n    <09> = 9;\n    <10> = 10;\n    <11> = 11;\n    <12> = 12;\n    <13> = 13;\n    <14> = 14;\n    <15> = 15;\n  };\n  xkb_types { include \"basic+extra\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <01> { [ a] };\n    key <02> { [ a, b ] };\n    key <03> { [ {} ] };\n    key <04> { [ {}, b ] };\n    key <05> { [ a, {} ] };\n    key <06> { [ {}, {} ] };\n    key <07> { [ { a } ] };\n    key <08> { [ { a }, { b } ] };\n    key <09> { [ { a, b } ] };\n    key <10> { [ { a, b, c } ] };\n    key <11> { [ a, { b, c } ] };\n    key <12> { [ { a, b }, c ] };\n    key <13> { [ { a, b }, { c, d } ] };\n    key <14> { [ { a, b }, c, { d, a } ] };\n    key <15> { [ { a, b }, { c, d }, a ] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/symbols-multi-keysyms.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {}, b }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {}, {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, { b } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { a }, b }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { a, b }, c }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, { b, c } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, {}, c }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, b, {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {}, b, c }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { a, b }, c, d }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, { b, c }, d }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ a, b, { c, d } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { a, b }, { c, d } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <01> = 1;\n    <02> = 2;\n    <03> = 3;\n    <04> = 4;\n    <05> = 5;\n    <06> = 6;\n    <07> = 7;\n    <08> = 8;\n    <09> = 9;\n    <10> = 10;\n    <11> = 11;\n    <12> = 12;\n    <13> = 13;\n    <14> = 14;\n    <15> = 15;\n  };\n  xkb_types { include \"basic+extra\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <01> { [ SetMods(modifiers=Control)] };\n    key <02> { [ SetMods(modifiers=Control), SetGroup(group=+1) ] };\n    key <03> { [ {} ] };\n    key <04> { [ {}, SetGroup(group=+1) ] };\n    key <05> { [ SetMods(modifiers=Control), {} ] };\n    key <06> { [ {}, {} ] };\n    key <07> { [ { SetMods(modifiers=Control) } ] };\n    key <08> { [ { SetMods(modifiers=Control) }, { SetGroup(group=+1) } ] };\n    key <09> { [ { SetMods(modifiers=Control), SetGroup(group=+1) } ] };\n    key <10> { [ { SetMods(modifiers=Control), SetGroup(group=+1), Private(data=\"foo\") } ] };\n    key <11> { [ SetMods(modifiers=Control), { SetGroup(group=+1), Private(data=\"foo\") } ] };\n    key <12> { [ { SetMods(modifiers=Control), SetGroup(group=+1) }, Private(data=\"foo\") ] };\n    key <13> { [ { SetMods(modifiers=Control), SetGroup(group=+1) }, { Private(data=\"foo\"), Private(data=\"bar\") } ] };\n    key <14> { [ { SetMods(modifiers=Control), SetGroup(group=+1) }, Private(data=\"foo\"), { Private(data=\"bar\"), SetMods(modifiers=Control) } ] };\n    key <15> { [ { SetMods(modifiers=Control), SetGroup(group=+1) }, { Private(data=\"foo\"), Private(data=\"bar\") }, SetMods(modifiers=Control) ] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/symbols-multi-actions.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {}, SetGroup(group=+1) }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {}, {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), { SetGroup(group=+1) } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { SetMods(modifiers=Control) }, SetGroup(group=+1) }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { SetMods(modifiers=Control), SetGroup(group=+1) }, Private(data=\"foo\") }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), { SetGroup(group=+1), Private(data=\"foo\") } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), {}, Private(data=\"foo\") }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), SetGroup(group=+1), {} }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ {}, SetGroup(group=+1), Private(data=\"foo\") }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { SetMods(modifiers=Control), SetGroup(group=+1) }, Private(data=\"foo\"), Private(data=\"bar\") }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), { SetGroup(group=+1), Private(data=\"foo\") }, Private(data=\"bar\") }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ SetMods(modifiers=Control), SetGroup(group=+1), { Private(data=\"foo\"), Private(data=\"bar\") } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <AE01> = 10;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [{ { SetMods(modifiers=Control), SetGroup(group=+1) }, { Private(data=\"foo\"), Private(data=\"bar\") } }] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <10> = 10;\n    <11> = 11;\n    <12> = 12;\n    <13> = 13;\n    <14> = 14;\n    <15> = 15;\n    <16> = 16;\n    <17> = 17;\n    <18> = 18;\n    <19> = 19;\n    <20> = 20;\n    <21> = 21;\n    <22> = 22;\n    <23> = 23;\n    <30> = 30;\n    <31> = 31;\n    <32> = 32;\n    <33> = 33;\n    <34> = 34;\n    <35> = 35;\n    <36> = 36;\n    <37> = 37;\n    <38> = 38;\n    <39> = 39;\n  };\n  xkb_types { include \"basic+extra\" };\n  xkb_compat {\n    interpret 1 { action = {}; };\n    interpret 2 { action = {NoAction()}; };\n    interpret 3 { action = {SetMods()}; };\n    interpret 4 { action = {SetMods(), SetGroup(group=1)}; };\n  };\n  xkb_symbols {\n    key <10> { [any, any ] };\n    key <11> { [{} , {}  ] };\n    key <12> { [any, any ], [SetMods(modifiers=Shift)] };\n    key <13> { [{} , {}  ], [SetMods(modifiers=Shift)] };\n    key <14> { [any, any ], type = \"TWO_LEVEL\" };\n    key <15> { [{} , {}  ], type = \"TWO_LEVEL\" };\n    key <16> { [a, A, any] };\n    key <17> { [a, A, {} ] };\n    key <18> { [a, A, any], type = \"FOUR_LEVEL_SEMIALPHABETIC\" };\n    key <19> { [a, A, {} ], type = \"FOUR_LEVEL_SEMIALPHABETIC\" };\n    key <20> { [a, A, ae, any] };\n    key <21> { [a, A, ae, {} ] };\n    key <22> { [a, A, ae, AE, any] };\n    key <23> { [a, A, ae, AE, {} ] };\n    key <30> { [NoAction(), NoAction() ] };\n    key <31> { actions=[{}, {}         ] };\n    key <32> { [NoAction(), NoAction() ], [a] };\n    key <33> { actions=[{}, {}         ], [a] };\n    key <34> { [NoAction(), NoAction() ], type = \"TWO_LEVEL\" };\n    key <35> { actions=[{}, {}         ], type = \"TWO_LEVEL\" };\n    key <38> { [NoAction(), NoAction() ], type = \"FOUR_LEVEL_SEMIALPHABETIC\" };\n    key <39> { actions=[{}, {}         ], type = \"FOUR_LEVEL_SEMIALPHABETIC\" };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/symbols-multi-keysyms-empty.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: size_t = 0 as size_t;
        while k
            < (::core::mem::size_of::<[keymap_test_data; 33]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%zu ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_multi_keysyms_actions\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V2,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_multi_keysyms_actions\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1908 as ::core::ffi::c_uint,
                    b"void test_multi_keysyms_actions(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
        let keymap_str: [::core::ffi::c_char; 171] = ::core::mem::transmute::<
            [u8; 171],
            [::core::ffi::c_char; 171],
        >(
            *b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_compat { interpret A { action = {SetMods(), SetGroup()}; }; };\n  xkb_symbols  { key <> { [{SetMods(), SetGroup()}] }; };\n};\0",
        );
        if test_compile_output(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_format,
            Some(
                compile_buffer
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_keymap_format,
                        *const ::core::ffi::c_char,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> *mut xkb_keymap,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            b"test_multi_keysyms_actions\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const keymap_str as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 171]>() as size_t)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            b"keymaps/symbols-multi-actions-v1.xkb\0".as_ptr() as *const ::core::ffi::c_char,
            update_output_files,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymap_str, ARRAY_SIZE(keymap_str), GOLDEN_TESTS_OUTPUTS \"symbols-multi-actions-v1.xkb\", update_output_files)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                1925 as ::core::ffi::c_uint,
                b"void test_multi_keysyms_actions(struct xkb_context *, _Bool)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn test_key_keysyms_as_strings(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymaps: [keymap_test_data; 2] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 10; };\n  xkb_symbols {\n    key <> { [\"\xC3\xBC\xFF\"] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <10> = 10;\n    <11> = 11;\n    <12> = 12;\n    <20> = 20;\n    <21> = 21;\n    <22> = 22;\n    <23> = 23;\n    <24> = 24;\n    <25> = 25;\n    <30> = 30;\n    <31> = 31;\n    <32> = 32;\n    <33> = 33;\n    <34> = 34;\n    <35> = 35;\n    <40> = 40;\n    <41> = 41;\n    <42> = 42;\n    <50> = 50;\n    <51> = 51;\n    <52> = 52;\n    <60> = 60;\n    <61> = 61;\n    <62> = 62;\n    <63> = 63;\n    <64> = 64;\n    <65> = 65;\n    <66> = 66;\n    <67> = 67;\n    <68> = 68;\n    <69> = 69;\n    <70> = 70;\n    <71> = 71;\n    <72> = 72;\n    <73> = 73;\n    <74> = 74;\n    <AD08> = 80;\n    <AC05> = 81;\n    <AB05> = 82;\n    <AD01> = 83;\n  };\n  xkb_types { include \"basic\" };\n  xkb_compat {\n   interpret.action = SetMods();\n   interpret \"\xC3\xA4\"           {};\n   interpret \"\xE2\x9C\xA8\"          {};\n   interpret \"\xF0\x9F\x8E\xBA\"          {};\n   interpret \"\\u{1F54A}\"  {};\n   interpret \"\\u{1}\"      {};\n   interpret \"\\u{a}\"      {};\n   interpret \"\\u{1f}\"     {};\n   interpret \"\\u{20}\"     {};\n   interpret \"\\u{7e}\"     {};\n   interpret \"\\u{7f}\"     {};\n   interpret \"\\u{80}\"     {};\n   interpret \"\\u{9f}\"     {};\n   interpret \"\\u{a0}\"     {};\n   interpret \"\\u{ff}\"     {};\n   interpret \"\\u{fdd0}\"   {};\n   interpret \"\\u{fdef}\"   {};\n   interpret \"\\u{fffe}\"   {};\n   interpret \"\\u{ffff}\"   {};\n   interpret \"\\u{10000}\"  {};\n   interpret \"\\u{1ffff}\"  {};\n   interpret \"\\u{10ffff}\" {};\n  };\n  xkb_symbols {\n    key <10> { [\"\", {b, \"\", c}] };\n    key <11> { [{a, \"\"}, {b, \"\"}] };\n    key <12> { [{\"\"}, {\"\", \"\"}] };\n    key <20> { [\"a\", \"bc\"] };\n    key <23> { [\"\xE2\x9C\xA8\", \"\xF0\x9F\x8E\xBA\"] };\n    key <24> { [\"u\xCC\x88\"] };\n    key <25> { [\"\xE2\x88\x80\xE2\x88\x82\xE2\x88\x88\xE2\x84\x9D\xE2\x88\xA7\xE2\x88\xAA\xE2\x89\xA1\xE2\x88\x9E \xE2\x86\x91\xE2\x86\x97\xE2\x86\xA8\xE2\x86\xBB\xE2\x87\xA3 \xE2\x94\x90\xE2\x94\xBC\xE2\x95\x94\xE2\x95\x98\xE2\x96\x91\xE2\x96\xBA\xE2\x98\xBA\xE2\x99\x80 \xEF\xAC\x81\xEF\xBF\xBD\xE2\x91\x80\xE2\x82\x82\xE1\xBC\xA0\xE1\xB8\x82\xD3\xA5\xE1\xBA\x84\xC9\x90\xCB\x90\xE2\x8D\x8E\xD7\x90\xD4\xB1\xE1\x83\x90\"] };\n    key <30> { [{\"a\"      }, {\"bc\"      }] };\n    key <31> { [{\"a\", \"\"}, {\"bc\", \"\"}] };\n    key <32> { [{\"\", \"a\"}, {\"\", \"bc\"}] };\n    key <33> { [{\"\\u{1F54A}\"}, {\"\\u{1f3f3}\\u{fe0f}\"}] };\n    key <34> { [{\"u\\u{0308}\"}] };\n    key <35> { [{\"\xE2\x88\x80\\u{0}\xE2\x88\x82\xE2\x88\x88\xE2\x84\x9D\xE2\x88\xA7\xE2\x88\xAA\xE2\x89\xA1\xE2\x88\x9E \xE2\x86\x91\xE2\x86\x97\xE2\x86\xA8\xE2\x86\xBB\xE2\x87\xA3 \xE2\x94\x90\xE2\x94\xBC\xE2\x95\x94\xE2\x95\x98\xE2\x96\x91\xE2\x96\xBA\xE2\x98\xBA\xE2\x99\x80 \xEF\xAC\x81\xEF\xBF\xBD\xE2\x91\x80\xE2\x82\x82\xE1\xBC\xA0\xE1\xB8\x82\xD3\xA5\xE1\xBA\x84\xC9\x90\xCB\x90\xE2\x8D\x8E\xD7\x90\xD4\xB1\xE1\x83\x90\"}] };\n    key <40> { [{\"a\",       b}, {\"cde\",       f}] };\n    key <41> { [{\"a\", \"\", b}, {\"cde\", \"\", f}] };\n    key <42> { [{\"a\", b, \"\"}, {\"cde\", f, \"\"}] };\n    key <50> { [{a,       \"b\"}, {c,       \"def\"}] };\n    key <51> { [{a, \"\", \"b\"}, {c, \"\", \"def\"}] };\n    key <52> { [{a, \"b\", \"\"}, {c, \"def\", \"\"}] };\n    key <60> { [{\"a\",       \"b\"}, {\"cd\",       \"ef\"}] };\n    key <61> { [{\"a\", \"\", \"b\"}, {\"cd\", \"\", \"ef\"}] };\n    key <63> { [{\"a\",       \"bcd\"}, {\"efg\",       \"h\"}] };\n    key <64> { [{\"a\", \"\", \"bcd\"}, {\"efg\", \"\", \"h\"}] };\n    key <65> { [\"\\u{0}\", \"\\u{10ffff}\"] };\n    key <66> { [\"\\u{1}\", \"\\u{a}\"] };\n    key <67> { [\"\\u{1f}\", \"\\u{20}\"] };\n    key <68> { [\"\\u{7e}\", \"\\u{7f}\"] };\n    key <69> { [\"\\u{80}\", \"\\u{9f}\"] };\n    key <70> { [\"\\u{a0}\", \"\\u{ff}\"] };\n    key <71> { [\"\\u{d800}\", \"\\u{dfff}\"] };\n    key <72> { [\"\\u{fdd0}\", \"\\u{fdef}\"] };\n    key <73> { [\"\\u{fffe}\", \"\\u{ffff}\"] };\n    key <74> { [\"\\u{10000}\", \"\\u{1ffff}\"] };\n    key <AD08> { [ \"ij\" , \"\xC4\xB2\"   ] }; // IJ Dutch digraph\n    key <AC05> { [ \"g\xCC\x83\"  , \"G\xCC\x83\"   ] }; // G\xCC\x83 Guarani letter\n    key <AB05> { [ \"\xD9\x84\xD8\xA7\"\xE2\x80\x8E  , \"\xD9\x84\xD8\xA2\"\xE2\x80\x8E   ] }; // \xE2\x81\xA7\xD9\x84\xD8\xA7\xE2\x81\xA9 \xE2\x81\xA7\xD9\x84\xD8\xA2\xE2\x81\xA9 Arabic Lam-Alef ligatures decomposed\n    key <AD01> { [ \"c\xE2\x80\x99h\", \"C\xE2\x80\x99h\" ] }; // C\xE2\x80\x99H Breton trigraph\n    modifier_map Mod1 { \"\xE2\x9C\xA8\", \"\\u{1F54A}\" };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/string-as-keysyms.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_key_keysyms_as_strings\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_key_keysyms_as_strings\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2079 as ::core::ffi::c_uint,
                    b"void test_key_keysyms_as_strings(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_invalid_symbols_fields(mut ctx: *mut xkb_context) {
    unsafe {
        let keymaps: [*const ::core::ffi::c_char; 7] = [
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { vmods = [] }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { repeat = [] }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { type = [] }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { groupswrap = [] }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { groupsredirect = [] }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { vmods=[], repeats=false }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"xkb_keymap {\n    xkb_keycodes { <> = 9; };\n    xkb_symbols { key <> { repeats=false, vmods=[] }; };\n};\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_invalid_symbols_fields\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut keymap: *const xkb_keymap = test_compile_buffer(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                keymaps[k as usize],
                strlen(keymaps[k as usize]),
            );
            if keymap.is_null() {
            } else {
                __assert_fail(
                    b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2125 as ::core::ffi::c_uint,
                    b"void test_invalid_symbols_fields(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_modifier_maps(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let mut keymaps: [keymap_test_data; 7] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <CAPS> = 66;\n    alias <LOCK> = <CAPS>;\n    <0> = 0;    <1> = 1;    <2> = 2;    <3> = 3;    <4> = 4;    <5> = 5;    <6> = 6;    <7> = 7;    <any>  = 10;    <none> = 11;    <a> = 61;    <b> = 62;    <c> = 63;    <100> = 100;  };\n  xkb_types { include \"basic\" };\n  xkb_symbols {\n    key <CAPS> { [Caps_Lock] };\n    key <any>  { [any, A] };\n    key <none> { [none, N] };\n    key <0>    { [0] };\n    key <1>    { [1] };\n    key <2>    { [2] };\n    key <a>    { [a] };\n    key <b>    { [b] };\n    key <c>    { [\"\xF0\x9F\x8E\xBA\"] };\n    key <3>    { [NotAKeysym, 3] };\n    key <4>    { [\"\\u{0000001F54A}\"]};\n    key <5>    { [\"\\u{d800}\", \"\\u{dfff}\"]};\n    key <6>    { [\"\\u{fdd0}\", \"\\u{fdef}\"]};\n    key <7>    { [\"\\u{fffe}\", \"\\u{ffff}\"]};\n    key <100>  { [C] };\n    modifier_map Lock {\n      <100>, <LOCK>, any, none,\n      0, 1, 0x2, a, \"b\", \"\xF0\x9F\x8E\xBA\", \"\\u{1F54A}\",\n      \"\\u{d800}\", \"\\u{dfff}\",\n      \"\\u{fdd0}\", \"\\u{fdef}\",\n      \"\\u{fffe}\", \"\\u{ffff}\",\n      NotAKeysym\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/symbols-modifier_map.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { modifier_map Lock { \"\" }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { modifier_map Lock { \"\xFF\" }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { modifier_map Lock { \"\\u{0}\" }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { modifier_map Lock { \"ab\" }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { modifier_map Lock { [a] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_symbols { modifier_map Lock { {a, b} }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_modifier_maps\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_modifier_maps\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2240 as ::core::ffi::c_uint,
                    b"void test_modifier_maps(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_empty_compound_statements(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let mut keymaps: [keymap_test_data; 1] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <Q> = 24;\n    <W> = 25;\n    <E> = 26;\n    <R> = 27;\n    <T> = 28;\n    <Y> = 29;\n    <U> = 30;\n    <I> = 31;\n    <O> = 32;\n    <P> = 33;\n    <A> = 38;\n    <S> = 39;\n    <D> = 40;\n    <F> = 41;\n    <G> = 42;\n    <H> = 43;\n    <Z> = 52;\n    <X> = 53;\n    <C> = 54;\n    <V> = 55;\n    <B> = 56;\n    <N> = 57;\n    <M> = 58;\n  };\n  xkb_types {\n    type \"t1\" {};\n    type \"t2\" { modifiers = Shift; map[Shift] = 2; };\n  };\n  xkb_compat {\n    virtual_modifiers M1, M2;\n    indicator \"xxx\" {};\n    indicator.modifiers = Shift;    indicator \"yyy\" {};\n\n    interpret q {};\n    interpret.repeat = true;\n    interpret w {};\n  };\n  xkb_symbols {\n    key <Q> { [q] };\n    key <W> { [SetMods()] };\n    key <E> { [e], type = \"t1\" };\n    key <R> {};\n    key <Y> { [] };\n    key <T> {};\n    key <T> { [] };\n    key <U> {};\n    key <U> { [], [u] };\n    key <I> { [i] };\n    key <I> {};\n    key <O> { [NoSymbol] };\n    key <P> { [NoAction()] };\n    key <A> { vmods = M1 };\n    key <S> { repeat = false };\n    key <D> { type = \"t2\" };\n    key <F> { [], type = \"t2\" };\n    key <G> { type[1] = \"t2\" };\n    key <H> { type[1] = \"t1\", type[2] = \"t2\" };\n    key <Z> {};\n    key.vmods = M1;\n    key <X> {};\n    key <C> { vmods = M2 };\n    key.type = \"t2\";\n    key <V> { vmods = 0 };\n    key <B> { [], vmods = 0 };\n    key.type[1] = \"t1\";\n    key <N> { vmods = 0 };\n    key <M> { [], [], vmods = 0 };\n    modmap Shift   { <Z> };\n    modmap Lock    { <X> };\n    modmap Control { <C> };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/empty-compound-statements.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_empty_compound_statements\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_empty_compound_statements\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2343 as ::core::ffi::c_uint,
                    b"void test_empty_compound_statements(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_escape_sequences(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let bad_octal: [::core::ffi::c_char; 3] = [
            '"' as i32 as ::core::ffi::c_char,
            '\\' as i32 as ::core::ffi::c_char,
            '1' as i32 as ::core::ffi::c_char,
        ];
        if test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const bad_octal as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!test_compile_buffer(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, bad_octal, sizeof(bad_octal))\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2354 as ::core::ffi::c_uint,
                b"void test_escape_sequences(struct xkb_context *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let bad_unicode: [::core::ffi::c_char; 5] = [
            '"' as i32 as ::core::ffi::c_char,
            '\\' as i32 as ::core::ffi::c_char,
            'u' as i32 as ::core::ffi::c_char,
            '{' as i32 as ::core::ffi::c_char,
            '1' as i32 as ::core::ffi::c_char,
        ];
        if test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const bad_unicode as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!test_compile_buffer(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, bad_unicode, sizeof(bad_unicode))\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2357 as ::core::ffi::c_uint,
                b"void test_escape_sequences(struct xkb_context *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let keymap: [::core::ffi::c_char; 1567] = ::core::mem::transmute::<
            [u8; 1567],
            [::core::ffi::c_char; 1567],
        >(
            *b"default xkb_keymap \"\" {\n  xkb_keycodes \"\\u{0}La paix est la seule\\tbataille qui vaille la peine d\\u{02019}\\u{Ea}tre men\\303\\251e.\\n\" {\n    <> = 1;\n    indicator 1 = \"\\0\\n\\u{2328}\\u{fe0f}\";\n    indicator 2 = \"surrogates: \\u{d800} \\u{dfff}\";\n    indicator 3 = \"noncharacters: \\u{fdd0} \\u{fdef} \\u{fffe} \\u{ffff}\";\n    indicator 4 = \"noncharacters: \\u{1fffe} \\u{1ffff} \\u{10fffe} \\u{10ffff}\";\n    indicator 5 = \"out of range: \\u{0} a \\u{110000} \\u{ffffffffffff}\";\n    indicator 6 = \"invalid: \\u a \\uA b \\u{} c \\u{ d \\u} e \\u{1\";\n    indicator 7 = \"invalid: \\u{1234x\\\" y \";\n    indicator 8 = \"invalid: \\u{ 2} x \\u{3 } y\";\n    indicator 9 = \"\\u{+1} \\u{-1} \\u{x} \\u{#} \\u{\\0} \\u{\\}\";\n  };\n  xkb_types \"\\00001\\\\\\00427\\u{22}\\r\\n\" {\n    type \"\\0\\00451\\u{1F3BA}\\u{2728}\\u{01F54a}\\u{0fE0f}\\f\" {\n      modifiers = Shift;\n      map[Shift] = 2;\n      level_name[1] = \"O\\u{f9} ils ont fait un \\u{22}d\\303\\251sert\\u{22}, \\eils disent qu\\u{002019}ils \\12ont fait la \\42paix\\042.\\b\\n\";\n      level_name[2] = \"\\u{0000}Science \\u{73}\\141ns conscience n\\u{2019}est que rui\\\\ne\\u{A} de l\\u{02019}\\u{E2}me.\\r\";\n    };\n  };\n  xkb_compat \"Le c\\u{153}ur a \\v ses raisons que la raison ne con\\u{5C}na\\u{EE}t point\" {\n    indicator \"\\n\\u{2328}\\0\\u{fe0f}\" { modifiers = Shift; };\n  };\n  xkb_symbols \"La libert\\u{00e9} commence o\\u{0000f9} l\\342\\200\\231ignorance finit.\" {\n    name[1] = \"\\n\\0377\\3760\";\n    name[2] = \"\\00427\";\n    key <> {\n      symbols[1]=[a, A],\n      type[1]=\"%1\\u{1F3BA}\\u{2728}\\u{00001F54a}\\u{0fE0f}\\u{0C}\",\n      actions[2]=[Private(type=123, data=\"abc\0def\")]    };\n  };\n};\0",
        );
        let expected: [::core::ffi::c_char; 29] = ::core::mem::transmute::<
            [u8; 29],
            [::core::ffi::c_char; 29],
        >(*b"keymaps/escape-sequences.xkb\0");
        if test_compile_output(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_format,
            Some(
                compile_buffer
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_keymap_format,
                        *const ::core::ffi::c_char,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> *mut xkb_keymap,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            b"test_escape_sequences\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const keymap as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 1567]>() as size_t)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            &raw const expected as *const ::core::ffi::c_char,
            update_output_files,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymap, ARRAY_SIZE(keymap), expected, update_output_files)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2409 as ::core::ffi::c_uint,
                b"void test_escape_sequences(struct xkb_context *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn test_unicode_keysyms(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let mut keymaps: [keymap_test_data; 1] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { include \"evdev\" };\n  xkb_types { include \"basic\" };\n  xkb_symbols {\n    key <AE01> { [U0000, 0x01000000 ] };\n    key <AE02> { [U0001, 0x01000001 ] };\n    key <AE03> { [U000A, 0x0100000a ] };\n    key <AE04> { [U001F, 0x0100001f ] };\n    key <AE05> { [U0020, 0x01000020 ] };\n    key <AE06> { [U007E, 0x0100007e ] };\n    key <AE07> { [U007F, 0x0100007f ] };\n    key <AE08> { [U0080, 0x01000080 ] };\n    key <AE09> { [U009F, 0x0100009f ] };\n    key <AE10> { [U00A0, 0x010000a0 ] };\n    key <AE11> { [U00FF, 0x010000ff ] };\n    key <AD01> { [U0100, 0x01000100 ] };\n    key <AD02> { [UD7FF, 0x0100d7ff ] };\n    key <AD03> { [UD800, 0x0100d800 ] };\n    key <AD04> { [UDFFF, 0x0100dfff ] };\n    key <AD05> { [UE000, 0x0100e000 ] };\n    key <AD06> { [UFDCF, 0x0100fdcf ] };\n    key <AD07> { [UFDD0, 0x0100fdd0 ] };\n    key <AD08> { [UFDEF, 0x0100fdef ] };\n    key <AD09> { [UFDF0, 0x0100fdf0 ] };\n    key <AD10> { [UFFFD, 0x0100fffd ] };\n    key <AD11> { [UFFFE, 0x0100fffe ] };\n    key <AD12> { [UFFFF, 0x0100ffff ] };\n    key <AC01> { [U10000, 0x01010000 ] };\n    key <AC08> { [U1FFFE , 0x0101fffe ] };\n    key <AC09> { [U1FFFF , 0x0101ffff ] };\n    key <AC10> { [U10FFFE, 0x0110fffe ] };\n    key <AC11> { [U10FFFF, 0x0110ffff ] };\n    key <AC12> { [U110000, 0x01110000 ] };\n    key <AB01> { [U0174, 0x01000174 ] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/unicode-keysyms.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_unicode_keysyms\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_unicode_keysyms\0".as_ptr() as *const ::core::ffi::c_char,
                keymaps[k as usize].keymap,
                strlen(keymaps[k as usize].keymap),
                keymaps[k as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymaps[k].keymap, strlen(keymaps[k].keymap), keymaps[k].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2479 as ::core::ffi::c_uint,
                    b"void test_unicode_keysyms(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_no_action_void_action(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymap_str: [::core::ffi::c_char; 255] = ::core::mem::transmute::<
            [u8; 255],
            [::core::ffi::c_char; 255],
        >(
            *b"xkb_keymap {\n  xkb_keycodes { <1> = 1; <2> = 2; <3> = 3; };\n  xkb_symbols {\n   key <1> { [x], [NoAction(mods=1)] };\n   key <2> { [y], [VoidAction(mods=1)] };\n   key <3> { [NoAction()] };\n   key <3> { [VoidAction()] };\n   key <3> { [NoAction()] };\n  };\n};\0",
        );
        let flags: xkb_keymap_compile_flags = (XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int
            & !(XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int))
            as xkb_keymap_compile_flags;
        static mut tests: [C2Rust_Unnamed_17; 4] = [
            C2Rust_Unnamed_17 {
                format: C2Rust_Unnamed_18 {
                    input: XKB_KEYMAP_FORMAT_TEXT_V1,
                    output: XKB_KEYMAP_FORMAT_TEXT_V1,
                },
                output: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_17 {
                format: C2Rust_Unnamed_18 {
                    input: XKB_KEYMAP_FORMAT_TEXT_V1,
                    output: XKB_KEYMAP_FORMAT_TEXT_V2,
                },
                output: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            C2Rust_Unnamed_17 {
                format: C2Rust_Unnamed_18 {
                    input: XKB_KEYMAP_FORMAT_TEXT_V2,
                    output: XKB_KEYMAP_FORMAT_TEXT_V2,
                },
                output: b"keymaps/no_void_action-v2.xkb\0".as_ptr() as *const ::core::ffi::c_char,
            },
            C2Rust_Unnamed_17 {
                format: C2Rust_Unnamed_18 {
                    input: XKB_KEYMAP_FORMAT_TEXT_V2,
                    output: XKB_KEYMAP_FORMAT_TEXT_V1,
                },
                output: b"keymaps/no_void_action-v1.xkb\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        let mut t: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (t as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_17; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_17>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_no_action_void_action\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            if test_compile_output(
                ctx,
                tests[t as usize].format.input,
                tests[t as usize].format.output,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                &raw const flags as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw const keymap_str as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 255]>() as size_t,
                tests[t as usize].output,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, tests[t].format.input, tests[t].format.output, compile_buffer, (void*)&flags, NULL, keymap_str, sizeof(keymap_str), tests[t].output, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2538 as ::core::ffi::c_uint,
                    b"void test_no_action_void_action(struct xkb_context *, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            t = t.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_prebuilt_keymap_roundtrip(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        static mut data: [C2Rust_Unnamed_19; 3] = [
            C2Rust_Unnamed_19 {
                path: b"keymaps/stringcomp-v1.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V1,
                serialize_flags: (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                    as xkb_keymap_serialize_flags,
            },
            C2Rust_Unnamed_19 {
                path: b"keymaps/stringcomp-v1-no-prettyfied.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V1,
                serialize_flags: ((XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                    & !(XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int))
                    as xkb_keymap_serialize_flags,
            },
            C2Rust_Unnamed_19 {
                path: b"keymaps/stringcomp-v2.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V2,
                serialize_flags: (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                    as xkb_keymap_serialize_flags,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_19; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_19>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_prebuilt_keymap_roundtrip\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut original: *mut ::core::ffi::c_char = test_read_file(data[k as usize].path);
            if !original.is_null() {
            } else {
                __assert_fail(
                    b"original\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2573 as ::core::ffi::c_uint,
                    b"void test_prebuilt_keymap_roundtrip(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while i <= 1 as ::core::ffi::c_uint {
                if test_compile_output2(
                    ctx,
                    data[k as usize].format,
                    4294967295 as xkb_keymap_format,
                    data[k as usize].serialize_flags,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"Round-trip\0".as_ptr() as *const ::core::ffi::c_char,
                    original,
                    strlen(original).wrapping_add(i as size_t),
                    data[k as usize].path,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output2(ctx, data[k].format, XKB_KEYMAP_USE_ORIGINAL_FORMAT, data[k].serialize_flags, compile_buffer, NULL, \"Round-trip\", original, strlen(original) + i, data[k].path, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2581 as ::core::ffi::c_uint,
                        b"void test_prebuilt_keymap_roundtrip(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                i = i.wrapping_add(1);
            }
            free(original as *mut ::core::ffi::c_void);
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_keymap_from_rules(mut ctx: *mut xkb_context) {
    unsafe {
        fprintf(
            stderr,
            b"------\n*** %s ***\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"test_keymap_from_rules\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut keymap: *mut xkb_keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"ru,ca,de,us\0".as_ptr() as *const ::core::ffi::c_char,
            b",multix,neo,intl\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2596 as ::core::ffi::c_uint,
                b"void test_keymap_from_rules(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut dump: *mut ::core::ffi::c_char =
            xkb_keymap_get_as_string(keymap, XKB_KEYMAP_USE_ORIGINAL_FORMAT);
        if !dump.is_null() {
        } else {
            __assert_fail(
                b"dump\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2598 as ::core::ffi::c_uint,
                b"void test_keymap_from_rules(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_buffer(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, dump, strlen(dump));
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2602 as ::core::ffi::c_uint,
                b"void test_keymap_from_rules(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        free(dump as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn test_redirect_key(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        let tests: [keymap_test_data; 1] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <A> = 38; <S> = 39; <D> = 40; <F> = 41; <G> = 42;};\n  xkb_symbols {\n    key <A> { [a], [RedirectKey(key=<?>)] };\n    key <S> { repeat=true, [s], [RedirectKey(key=<A>), RedirectKey(key=<D>)] };\n    key <D> { repeat=true, [d], [RedirectKey(key=<S>,mods=Shift,clearMods=Control)] };\n    key <F> { [f], [RedirectKey()] };\n    key <G> { [g], [RedirectKey(keycode=auto)] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/redirect-key-1.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let flags: xkb_keymap_compile_flags = (XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int
            & !(XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int))
            as xkb_keymap_compile_flags;
        let mut t: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (t as usize)
            < (::core::mem::size_of::<[keymap_test_data; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_redirect_key\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                &raw const flags as *mut ::core::ffi::c_void,
                b"test_redirect_key\0".as_ptr() as *const ::core::ffi::c_char,
                tests[t as usize].keymap,
                strlen(tests[t as usize].keymap),
                tests[t as usize].expected,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&flags, __func__, tests[t].keymap, strlen(tests[t].keymap), tests[t].expected, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2639 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            t = t.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_unsupported_legacy_x11_actions(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        let keymap_str: [::core::ffi::c_char; 857] = ::core::mem::transmute::<
            [u8; 857],
            [::core::ffi::c_char; 857],
        >(
            *b"xkb_keymap {\n  xkb_keycodes {\n    <1> = 1;\n    <2> = 2;\n    <3> = 3;\n    <4> = 4;\n    <5> = 5;\n  };\n  xkb_compat {\n    ISOLock.modifiers = modMapMods;\n    DeviceButton.data = <1>;\n    LockDeviceButton.data = <1>;\n    DeviceValuator.data = <1>;\n    ActionMessage.data = <1>;\n    interpret ISO_Lock {\n      action=ISOLock(affect=all);\n    };\n    interpret 0x10000 {\n      action=DeviceButton(data=all);\n    };\n    interpret 0x10001 {\n      action=LockDeviceButton(data=all);\n    };\n    interpret 0x10002 {\n      action=DeviceValuator(data=all);\n    };\n    interpret 0x10003 {\n      action=ActionMessage(data=all);\n    };\n  };\n  xkb_symbols {\n   key <1> { [ISOLock(affect=all)] };\n   key <2> { [DeviceButton(data=<1>)] };\n   key <3> { [LockDeviceButton(data=<1>)] };\n   key <4> { [DeviceValuator(data=<1>)] };\n   key <5> { [ActionMessage(data=<1>)] };\n  };\n};\0",
        );
        if test_compile_output(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_format,
            Some(
                compile_buffer
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_keymap_format,
                        *const ::core::ffi::c_char,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> *mut xkb_keymap,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            b"test_unsupported_legacy_x11_actions\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const keymap_str as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 857]>() as size_t,
            b"keymaps/unsupported-x11-actions\0".as_ptr() as *const ::core::ffi::c_char,
            update_output_files,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, keymap_str, sizeof(keymap_str), GOLDEN_TESTS_OUTPUTS \"unsupported-x11-actions\", update_output_files)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                2691 as ::core::ffi::c_uint,
                b"void test_unsupported_legacy_x11_actions(struct xkb_context *, _Bool)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn test_overlays(mut ctx: *mut xkb_context, mut update_output_files: bool) {
    unsafe {
        static mut tests: [keymap_multi_versions_test_data; 9] = [
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n  };\n  xkb_symbols {\n    key <j> { [j], overlay1 = <xxx> };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: false,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n    <kp1>  = 87;\n    <left> = 113;\n  };\n  xkb_symbols {\n    key <j> { [j], overlay1 = <kp1>, overlay2 = <left> };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n    <kp1>  = 87;\n  };\n  xkb_symbols {\n    key <j> { [j], overlay3 = <kp1> };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n    <kp1>  = 87;\n  };\n  xkb_symbols {\n    key <j> { [j], overlay3 = none };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n    <kp1>  = 87;\n    <left> = 113;\n  };\n  xkb_symbols {\n    key <j>    { [j], overlay2 = <left> };\n    key <j>    { overlay1 = <kp1>, overlay2 = none };\n    key <kp1>  { [KP_1], overlay1 = none, overlay1 = <j> };\n    key <left> { [Left], overlay1 = <kp1> };\n    key <left> { overlay1 = none };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: false,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed_0: C2Rust_Unnamed_21 {
                        expected_v1_1: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v1_2: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_2: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_1: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n    <kp1>  = 87;\n    <left> = 113;\n  };\n  xkb_symbols {\n    key <j>    { [j], overlay1 = <kp1>, overlay1 = none };\n    key <kp1>  { [KP_1], overlay1 = <j> };\n    key <kp1>  { [KP_1], overlay2 = <left> };\n    key <kp1>  { [KP_1], overlay1 = none, overlay2 = none };\n    key <left> { [Left], overlay1 = none, overlay2 = none };\n    augment key <left> { [Left], overlay1 = <j> };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: false,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed_0: C2Rust_Unnamed_21 {
                        expected_v1_1: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v1_2: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_2: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_1: b"keymaps/overlays-v1-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <1>    = 10;\n    <2>    = 11;\n    <3>    = 12;\n    <4>    = 13;\n    <j>    = 44;\n    <k>    = 45;\n    <kp1>  = 87;\n    <kp2>  = 88;\n    <f1>   = 67;\n    <f2>   = 68;\n    <f10>  = 76;\n    <f11>  = 95;\n    <left> = 113;\n    <down> = 116;\n  };\n  xkb_symbols {\n    key <1>    { [LockControls(controls=Overlay1)] };\n    key <2>    { [LockControls(controls=Overlay2)] };\n    key <3>    { [LockControls(controls=none)] };\n    key <4>    { [LockControls(controls=none)] };\n    key <j>    { [j], overlay1 = <kp1>, overlay2 = <left> };\n    key <k>    { [k], overlay1 = <kp2>, overlay2 = <down> };\n    key <kp1>  { [KP_1], overlay1 = none };\n    key <kp2>  { [KP_2] };\n    key <left> { [Left], overlay1 = <kp1> };\n    key <left> { overlay1 = none };\n    key <down> { [Down] };\n    key <f1>   { [F1] };\n    key <f2>   { [F2] };\n    key <f10>  { [F10] };\n    key <f11>  { [F11] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: false,
                lenient: true,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed_0: C2Rust_Unnamed_21 {
                        expected_v1_1: b"keymaps/overlays-v1-2.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v1_2: b"keymaps/overlays-v1-2.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_2: b"keymaps/overlays-v2-1.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_1: b"keymaps/overlays-v1-2.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <j>    = 44;\n    <kp1>  = 87;\n    <left> = 113;\n  };\n  xkb_symbols {\n    key <j>    { [j], overlay[1] = <kp1>, overlay[2] = <left> };\n    key <kp1>  { [KP_1] };\n    key <left> { [Left] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: false,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <1>    = 10;\n    <2>    = 11;\n    <3>    = 12;\n    <4>    = 13;\n    <j>    = 44;\n    <k>    = 45;\n    <kp1>  = 87;\n    <kp2>  = 88;\n    <f1>   = 67;\n    <f2>   = 68;\n    <f10>  = 76;\n    <f11>  = 95;\n    <left> = 113;\n    <down> = 116;\n  };\n  xkb_symbols {\n    key <1> { [LockControls(controls=Overlay1)] };\n    key <2> { [LockControls(controls=Overlay2)] };\n    key <3> { [LockControls(controls=Overlay3)] };\n    key <4> { [LockControls(controls=Overlay4)] };\n    key <j> { [j] };\n    key <j> {\n      overlay1 = <left>, overlay2 = <left>,\n      overlay6 = <j>, overlay7 = <kp1>\n    };\n    key <j> {\n      overlay1 = <kp1>, overlay7 = none,\n      overlay3 = <f1>, overlay4 = <f10>\n    };\n    augment key <j> { overlay4 = <f1> };\n    key <k>    {\n      [k],\n      overlay1 = <kp2>,\n      overlay2 = <down>,\n      overlay3 = <f2>,\n      overlay4 = <f11>\n    };\n    key <kp1>  { [KP_1], overlay2 = <j> };\n    key <kp1>  { [KP_1], overlay1 = none, overlay2 = none, overlay3 = <f1> };\n    key <kp1>  { [KP_1], overlay3 = none };\n    key <kp2>  { [KP_2] };\n    key <left> { [Left], overlay3 = <f1>, overlay4 = <f10> };\n    key <down> { [Down] };\n    key <f1>   { [F1] };\n    key <f2>   { [F2] };\n    key <f10>  { [F10] };\n    key <f11>  { [F11] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: false,
                lenient: true,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed_0: C2Rust_Unnamed_21 {
                        expected_v1_1: ::core::ptr::null::<::core::ffi::c_char>(),
                        expected_v1_2: ::core::ptr::null::<::core::ffi::c_char>(),
                        expected_v2_2: b"keymaps/overlays-v2-2.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected_v2_1: b"keymaps/overlays-v1-2.xkb\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    },
                },
            },
        ];
        let mut t: size_t = 0 as size_t;
        while t
            < (::core::mem::size_of::<[keymap_multi_versions_test_data; 9]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_multi_versions_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%zu ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_overlays\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            let flags: xkb_keymap_compile_flags = (XKB_KEYMAP_COMPILE_STRICT_MODE
                as ::core::ffi::c_int
                & !(if tests[t as usize].lenient as ::core::ffi::c_int != 0 {
                    XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as xkb_keymap_compile_flags;
            if tests[t as usize].no_output {
                let mut keymap: *mut xkb_keymap = test_compile_buffer2(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    flags,
                    tests[t as usize].keymap,
                    strlen(tests[t as usize].keymap),
                );
                if !keymap.is_null() as ::core::ffi::c_int
                    == tests[t as usize].c2rust_unnamed.c2rust_unnamed.compiles_v1
                        as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"!!keymap == tests[t].compiles_v1\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2964 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                xkb_keymap_unref(keymap);
                keymap = test_compile_buffer2(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    flags,
                    tests[t as usize].keymap,
                    strlen(tests[t as usize].keymap),
                );
                if !keymap.is_null() as ::core::ffi::c_int
                    == tests[t as usize].c2rust_unnamed.c2rust_unnamed.compiles_v2
                        as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"!!keymap == tests[t].compiles_v2\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2969 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                xkb_keymap_unref(keymap);
            } else {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const flags as *mut ::core::ffi::c_void,
                    b"v1 -> v1\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[t as usize].keymap,
                    strlen(tests[t as usize].keymap),
                    tests[t as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v1_1,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&flags, \"v1 -> v1\", tests[t].keymap, strlen(tests[t].keymap), tests[t].expected_v1_1, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2978 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const flags as *mut ::core::ffi::c_void,
                    b"v1 -> v2\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[t as usize].keymap,
                    strlen(tests[t as usize].keymap),
                    tests[t as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v1_2,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, compile_buffer, (void*)&flags, \"v1 -> v2\", tests[t].keymap, strlen(tests[t].keymap), tests[t].expected_v1_2, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2984 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const flags as *mut ::core::ffi::c_void,
                    b"v2 -> v2\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[t as usize].keymap,
                    strlen(tests[t as usize].keymap),
                    tests[t as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v2_2,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&flags, \"v2 -> v2\", tests[t].keymap, strlen(tests[t].keymap), tests[t].expected_v2_2, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2990 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const flags as *mut ::core::ffi::c_void,
                    b"v2 -> v1\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[t as usize].keymap,
                    strlen(tests[t as usize].keymap),
                    tests[t as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v2_1,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_FORMAT_TEXT_V1, compile_buffer, (void*)&flags, \"v2 -> v1\", tests[t].keymap, strlen(tests[t].keymap), tests[t].expected_v2_1, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2996 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            t = t.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_extended_layout_indices(
    mut ctx: *mut xkb_context,
    mut update_output_files: bool,
) {
    unsafe {
        static mut tests: [keymap_multi_versions_test_data; 5] = [
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat { interpret a { action = SetGroup(group=5); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { indicator 1 = \"a\"; };\n  xkb_compat   { indicator \"a\" { groups=All-5; }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: true,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols  { name[5] = \"5\"; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols  { key <> { [1], [2], [3], [4], [5] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: true,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed: C2Rust_Unnamed_22 {
                        compiles_v1: false,
                        compiles_v2: true,
                    },
                },
            },
            keymap_multi_versions_test_data {
                keymap: b"xkb_keymap {\n  xkb_keycodes {\n    <a> = 1;\n    <b> = 2;\n    <c> = 3;\n    indicator 1 = \"a\";\n  };\n  xkb_compat {\n    interpret a { action = SetGroup(group=5); };\n    indicator \"a\" { groups=All-5; };\n  };\n  xkb_symbols {\n    name[5] = \"G5\";\n    name[1] = \"G1\";\n    key <a> { symbols[1]=[a], symbols[5]=[Greek_alpha] };\n    key <b> { actions[1]=[Latchgroup(group=-5)], actions[5]=[LockGroup(group=5)], [SetGroup(group=+9)] };\n    key <c> { [1], [2], [3], [4], [5], [6], [7], [8], [9] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                no_output: false,
                lenient: false,
                c2rust_unnamed: C2Rust_Unnamed_20 {
                    c2rust_unnamed_0: C2Rust_Unnamed_21 {
                        expected_v1_1: ::core::ptr::null::<::core::ffi::c_char>(),
                        expected_v1_2: ::core::ptr::null::<::core::ffi::c_char>(),
                        expected_v2_2: b"keymaps/extended-layout-indices-v2.xkb\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        expected_v2_1: b"keymaps/extended-layout-indices-v1.xkb\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    },
                },
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_multi_versions_test_data; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_multi_versions_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_extended_layout_indices\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if tests[k as usize].no_output {
                let mut keymap: *mut xkb_keymap = test_compile_buffer(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                );
                if !keymap.is_null() as ::core::ffi::c_int
                    == tests[k as usize].c2rust_unnamed.c2rust_unnamed.compiles_v1
                        as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"!!keymap == tests[k].compiles_v1\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        3078 as ::core::ffi::c_uint,
                        b"void test_extended_layout_indices(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                xkb_keymap_unref(keymap);
                keymap = test_compile_buffer(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                );
                if !keymap.is_null() as ::core::ffi::c_int
                    == tests[k as usize].c2rust_unnamed.c2rust_unnamed.compiles_v2
                        as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"!!keymap == tests[k].compiles_v2\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        3083 as ::core::ffi::c_uint,
                        b"void test_extended_layout_indices(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                xkb_keymap_unref(keymap);
            } else {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"test_extended_layout_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    tests[k as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v1_1,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, tests[k].keymap, strlen(tests[k].keymap), tests[k].expected_v1_1, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        3092 as ::core::ffi::c_uint,
                        b"void test_extended_layout_indices(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"test_extended_layout_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    tests[k as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v1_2,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, compile_buffer, NULL, __func__, tests[k].keymap, strlen(tests[k].keymap), tests[k].expected_v1_2, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        3098 as ::core::ffi::c_uint,
                        b"void test_extended_layout_indices(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"test_extended_layout_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    tests[k as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v2_2,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, tests[k].keymap, strlen(tests[k].keymap), tests[k].expected_v2_2, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        3104 as ::core::ffi::c_uint,
                        b"void test_extended_layout_indices(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"test_extended_layout_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    tests[k as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .expected_v2_1,
                    update_output_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_FORMAT_TEXT_V1, compile_buffer, NULL, __func__, tests[k].keymap, strlen(tests[k].keymap), tests[k].expected_v2_1, update_output_files)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        3110 as ::core::ffi::c_uint,
                        b"void test_extended_layout_indices(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_compound_statement_errors(mut ctx: *mut xkb_context) {
    unsafe {
        let tests: [keymap_test_data; 4] = [
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"1\" {\n      modifiers[0] = Shift;\n      map[Shift] = 2;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_types {\n    type \"1\" {\n      modifiers = \"xxx\";\n      map[Shift] = 2;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    interpret a {\n      action[0] = NoAction();\n      repeat = true;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
            keymap_test_data {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    interpret a {\n      action = \"xxx\";\n      repeat = true;\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: ::core::ptr::null::<::core::ffi::c_char>(),
                skip: false,
            },
        ];
        let mut t: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (t as usize)
            < (::core::mem::size_of::<[keymap_test_data; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_compound_statement_errors\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            if test_compile_output(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                4294967295 as xkb_keymap_format,
                Some(
                    compile_buffer
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"test_compound_statement_errors\0".as_ptr() as *const ::core::ffi::c_char,
                tests[t as usize].keymap,
                strlen(tests[t as usize].keymap),
                tests[t as usize].expected,
                false,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, NULL, __func__, tests[t].keymap, strlen(tests[t].keymap), tests[t].expected, false)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3174 as ::core::ffi::c_uint,
                    b"void test_compound_statement_errors(struct xkb_context *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            t = t.wrapping_add(1);
        }
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut update_output_files: bool = false;
        let mut seed: ::core::ffi::c_uint =
            time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint;
        let mut arg_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            arg_index += 1;
            if !(arg_index < argc) {
                break;
            }
            if streq(
                *argv.offset(arg_index as isize),
                b"update\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                update_output_files = true;
            } else if streq(
                *argv.offset(arg_index as isize),
                b"--seed\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
                && (arg_index + 1 as ::core::ffi::c_int) < argc
            {
                seed = atoi(*argv.offset((arg_index + 1 as ::core::ffi::c_int) as isize))
                    as ::core::ffi::c_uint;
            } else {
                fprintf(
                    stderr,
                    b"ERROR: unsupported argument: \"%s\".\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *argv.offset(arg_index as isize),
                );
                exit(EXIT_INVALID_USAGE);
            }
        }
        fprintf(
            stderr,
            b"Seed for the pseudo-random generator: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            seed,
        );
        srand(seed);
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                3206 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        static mut buf: [::core::ffi::c_char; 15] = unsafe {
            ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"xkb_keymap {};\0")
        };
        if xkb_keymap_new_from_buffer(
            ctx,
            &raw const buf as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_buffer(ctx, buf, sizeof(buf), XKB_KEYMAP_FORMAT_TEXT_V1, -1)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                3211 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_buffer(
            ctx,
            &raw const buf as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            65535 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_buffer(ctx, buf, sizeof(buf), XKB_KEYMAP_FORMAT_TEXT_V1, 0xffff)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                3213 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut keymap: *mut xkb_keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            0 as size_t,
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/buffercomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                3218 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        test_encodings(ctx);
        test_floats(ctx);
        test_component_syntax_error(ctx);
        test_optional_components(ctx, update_output_files);
        test_bidi_chars(ctx, update_output_files);
        test_recursive_includes(ctx);
        test_include_paths(ctx);
        test_include_default_maps(update_output_files);
        test_alloc_limits(ctx, update_output_files);
        test_integers(ctx, update_output_files);
        test_keycodes(ctx, update_output_files);
        test_masks(ctx, update_output_files);
        test_interpret(ctx, update_output_files);
        test_group_indices_names(ctx, update_output_files);
        test_level_indices_names(ctx, update_output_files);
        test_multi_keysyms_actions(ctx, update_output_files);
        test_key_keysyms_as_strings(ctx, update_output_files);
        test_invalid_symbols_fields(ctx);
        test_modifier_maps(ctx, update_output_files);
        test_empty_compound_statements(ctx, update_output_files);
        test_escape_sequences(ctx, update_output_files);
        test_unicode_keysyms(ctx, update_output_files);
        test_no_action_void_action(ctx, update_output_files);
        test_prebuilt_keymap_roundtrip(ctx, update_output_files);
        test_keymap_from_rules(ctx);
        test_redirect_key(ctx, update_output_files);
        test_unsupported_legacy_x11_actions(ctx, update_output_files);
        test_overlays(ctx, update_output_files);
        test_extended_layout_indices(ctx, update_output_files);
        test_compound_statement_errors(ctx);
        xkb_context_unref(ctx);
        return EXIT_SUCCESS;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        bounds = [
            C2Rust_Unnamed_14 {
                min: ((10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as xkb_keycode_t,
                max: XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t,
                max_count: 3 as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_14 {
                min: (XKB_KEYCODE_MAX_CONTIGUOUS + 1 as ::core::ffi::c_int) as xkb_keycode_t,
                max: XKB_KEYCODE_MAX as xkb_keycode_t,
                max_count: 10 as ::core::ffi::c_uint,
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
