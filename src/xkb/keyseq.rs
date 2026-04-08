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
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
}
pub mod stdint_intn_h {
    pub type int8_t = __int8_t;
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
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
    pub type xkb_keysym_flags = ::core::ffi::c_uint;
    pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;
    pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
    pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 4294967295 as xkb_keymap_format;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_keysym_from_name(
            name: *const ::core::ffi::c_char,
            flags: xkb_keysym_flags,
        ) -> xkb_keysym_t;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_get_as_string(
            keymap: *mut xkb_keymap,
            format: xkb_keymap_format,
        ) -> *mut ::core::ffi::c_char;
        pub fn xkb_keymap_key_by_name(
            keymap: *mut xkb_keymap,
            name: *const ::core::ffi::c_char,
        ) -> xkb_keycode_t;
        pub fn xkb_keymap_key_repeats(
            keymap: *mut xkb_keymap,
            key: xkb_keycode_t,
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
    pub type key_seq_state = ::core::ffi::c_uint;
    pub const FINISH: key_seq_state = 5;
    pub const NEXT: key_seq_state = 4;
    pub const BOTH: key_seq_state = 3;
    pub const UP: key_seq_state = 2;
    pub const REPEAT: key_seq_state = 1;
    pub const DOWN: key_seq_state = 0;
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::xkbcommon_h::xkb_keymap_format;
    extern "C" {
        pub fn test_init();
        pub fn test_key_seq(keymap: *mut xkb_keymap, ...) -> ::core::ffi::c_int;
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_file(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            path_rel: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
        pub fn test_compile_string(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            string: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
        pub fn test_compile_buffer(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            buf: *const ::core::ffi::c_char,
            len: size_t,
        ) -> *mut xkb_keymap;
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
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub mod evdev_scancodes_h {
    pub const KEY_Y: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
    pub const KEY_CAPSLOCK: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
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
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_Caps_Lock: ::core::ffi::c_int = 0xffe5 as ::core::ffi::c_int;
    pub const XKB_KEY_Y: ::core::ffi::c_int = 0x59 as ::core::ffi::c_int;
    pub const XKB_KEY_y: ::core::ffi::c_int = 0x79 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::evdev_scancodes_h::{KEY_CAPSLOCK, KEY_Y};
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
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdio_h::{fprintf, stderr};
pub use self::stdlib_h::{free, EXIT_SUCCESS};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    key_seq_state, test_compile_buffer, test_compile_file, test_compile_rules, test_compile_string,
    test_context_flags, test_get_context, test_init, test_key_seq, BOTH,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG, DOWN, FINISH, NEXT, REPEAT, UP,
};
pub use self::types_h::{
    __int16_t, __int32_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use self::xkbcommon_h::{
    xkb_context_unref, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keymap_get_as_string, xkb_keymap_key_by_name, xkb_keymap_key_repeats, xkb_keymap_unref,
    xkb_keysym_flags, xkb_keysym_from_name, xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t,
    xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t, xkb_log_level,
    xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, XKB_KEYSYM_CASE_INSENSITIVE,
    XKB_KEYSYM_NO_FLAGS, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_keysyms_h::{XKB_KEY_Caps_Lock, XKB_KEY_y, XKB_KEY_Y};
pub use self::FILE_h::FILE;
pub type fake_keys = ::core::ffi::c_uint;
pub const KEY_LVL3: fake_keys = 84;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_properties {
    pub name: *const ::core::ffi::c_char,
    pub repeats: bool,
    pub vmodmap: xkb_mod_mask_t,
}
static mut keymap_formats: [xkb_keymap_format; 2] =
    [XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2];
#[inline]
unsafe extern "C" fn get_keysym(
    mut keymap: *mut xkb_keymap,
    mut v1: xkb_keysym_t,
    mut v2: xkb_keysym_t,
) -> xkb_keysym_t {
    unsafe {
        return if (*keymap).format as ::core::ffi::c_uint
            == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            v1
        } else {
            v2
        };
    }
}
unsafe extern "C" fn test_group_lock(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut f: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (f as usize)
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,de\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:alt_shift_toggle\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    46 as ::core::ffi::c_uint,
                    b"void test_group_lock(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x79 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x7a as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x79 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x79 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq((keymap), KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_z, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    63 as ::core::ffi::c_uint,
                    b"void test_group_lock(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            f = f.wrapping_add(1);
        }
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,de\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:alt_shift_toggle,grp:lockOnPress\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if test_key_seq(
            keymap,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq((keymap), KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_z, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                76 as ::core::ffi::c_uint,
                b"void test_group_lock(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,de\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:alt_shift_toggle,grp:lockOnRelease\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if test_key_seq(
            keymap,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq((keymap), KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_z, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_z, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_z, NEXT, KEY_Y, DOWN, XKB_KEY_z, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, UP, XKB_KEY_z, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                113 as ::core::ffi::c_uint,
                b"void test_group_lock(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_group_latch(mut ctx: *mut xkb_context) {
    unsafe {
        let mut f: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (f as usize)
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            fprintf(
                stderr,
                b"=== %s, format %u ===\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_group_latch\0".as_ptr() as *const ::core::ffi::c_char,
                keymap_formats[f as usize] as ::core::ffi::c_uint,
            );
            let mut keymap: *mut xkb_keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
                b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:menu_latch_group2,grp:sclk_toggle,grp:lctrl_rctrl_switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_YEN, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, UINT32_MAX, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x68 as xkb_keysym_t, 0xce9 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_h, XKB_KEY_hebrew_yod), NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    209 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0xce9 as xkb_keysym_t, 0x6c8 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_hebrew_yod, XKB_KEY_Cyrillic_ha), NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    248 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
                b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:menu_latch_group2_lock,grp:sclk_toggle,grp:lctrl_rctrl_switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    288 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_YEN, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, UINT32_MAX, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    289 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x68 as xkb_keysym_t, 0xce9 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_h, XKB_KEY_hebrew_yod), NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0xce9 as xkb_keysym_t, 0x6c8 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_hebrew_yod, XKB_KEY_Cyrillic_ha), NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    309 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
                b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:menu_latch,grp:sclk_toggle,grp:lctrl_rctrl_switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_YEN, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, UINT32_MAX, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    321 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x68 as xkb_keysym_t, 0xce9 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_h, XKB_KEY_hebrew_yod), NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0xce9 as xkb_keysym_t, 0x6c8 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_hebrew_yod, XKB_KEY_Cyrillic_ha), NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
                b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:menu_latch_lock,grp:sclk_toggle,grp:lctrl_rctrl_switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    333 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    335 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_YEN, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, UINT32_MAX, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    336 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x68 as xkb_keysym_t, 0xce9 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_h, XKB_KEY_hebrew_yod), NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0xce9 as xkb_keysym_t, 0x6c8 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_hebrew_yod, XKB_KEY_Cyrillic_ha), NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTCTRL, BOTH, XKB_KEY_ISO_First_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    338 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_l, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    356 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
                b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:menu_latch_negative,grp:sclk_toggle,grp:lctrl_rctrl_switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_l, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    392 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_YEN, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, UINT32_MAX, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    417 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x68 as xkb_keysym_t, 0x73 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_h, XKB_KEY_s), NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    446 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x6c8 as xkb_keysym_t, 0xce9 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_Cyrillic_ha, XKB_KEY_hebrew_yod), NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    487 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    489 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
                b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
                b"grp:menu_latch_negative_lock,grp:sclk_toggle,grp:lctrl_rctrl_switch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    498 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x65 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_l, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_YEN, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, UINT32_MAX, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x68 as xkb_keysym_t, 0x73 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_h, XKB_KEY_s), NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    502 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x6c8 as xkb_keysym_t, 0xce9 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe0e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, get_keysym(keymap, XKB_KEY_Cyrillic_ha, XKB_KEY_hebrew_yod), NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_SCROLLLOCK, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, DOWN, XKB_KEY_ISO_Group_Latch, NEXT, KEY_SCROLLLOCK, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, UP, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_RIGHTCTRL, BOTH, XKB_KEY_ISO_Last_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    503 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x68 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x73 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                70 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c8 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6c5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                35 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xce9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                18 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xcf7 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_s, NEXT, KEY_E, BOTH, XKB_KEY_l, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_SCROLLLOCK, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_ha, NEXT, KEY_E, BOTH, XKB_KEY_Cyrillic_ie, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_E, BOTH, XKB_KEY_hebrew_qoph, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    523 as ::core::ffi::c_uint,
                    b"void test_group_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            f = f.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_mod_set(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"shift:breaks_caps\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if test_key_seq(
            keymap,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_A, BOTH, XKB_KEY_A, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_A, BOTH, XKB_KEY_a, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_A, BOTH, XKB_KEY_A, FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                552 as ::core::ffi::c_uint,
                b"void test_mod_set(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                553 as ::core::ffi::c_uint,
                b"void test_mod_set(struct xkb_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"shift:breaks_caps-v2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                561 as ::core::ffi::c_uint,
                b"void test_mod_set(struct xkb_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_A, BOTH, XKB_KEY_A, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_A, BOTH, XKB_KEY_A, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_A, BOTH, XKB_KEY_a, FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                569 as ::core::ffi::c_uint,
                b"void test_mod_set(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_mod_lock(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut f: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (f as usize)
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            keymap = test_compile_rules(
                ctx,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
                b"us\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    585 as ::core::ffi::c_uint,
                    b"void test_mod_lock(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x79 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                58 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x59 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                58 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x59 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                58 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x79 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq((keymap), KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_Y, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_Y, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_y, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    601 as ::core::ffi::c_uint,
                    b"void test_mod_lock(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            f = f.wrapping_add(1);
        }
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"caps:unlock-on-release\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                613 as ::core::ffi::c_uint,
                b"void test_mod_lock(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq((keymap), KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_Y, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_Y, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_y, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_uint,
                b"void test_mod_lock(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc105\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"caps:unlock-on-press\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                626 as ::core::ffi::c_uint,
                b"void test_mod_lock(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        test_key_seq(
            keymap,
            KEY_Y,
            BOTH as ::core::ffi::c_int,
            XKB_KEY_y,
            NEXT as ::core::ffi::c_int,
            KEY_CAPSLOCK,
            BOTH as ::core::ffi::c_int,
            XKB_KEY_Caps_Lock,
            NEXT as ::core::ffi::c_int,
            KEY_Y,
            BOTH as ::core::ffi::c_int,
            XKB_KEY_Y,
            NEXT as ::core::ffi::c_int,
            KEY_CAPSLOCK,
            DOWN as ::core::ffi::c_int,
            XKB_KEY_Caps_Lock,
            NEXT as ::core::ffi::c_int,
            KEY_Y,
            BOTH as ::core::ffi::c_int,
            XKB_KEY_y,
            NEXT as ::core::ffi::c_int,
            KEY_CAPSLOCK,
            UP as ::core::ffi::c_int,
            XKB_KEY_Caps_Lock,
            NEXT as ::core::ffi::c_int,
            KEY_Y,
            BOTH as ::core::ffi::c_int,
            XKB_KEY_y,
            FINISH as ::core::ffi::c_int,
        );
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_mod_latch(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut f: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (f as usize)
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            fprintf(
                stderr,
                b"=== %s, format %u ===\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_mod_latch\0".as_ptr() as *const ::core::ffi::c_char,
                keymap_formats[f as usize] as ::core::ffi::c_uint,
            );
            keymap = test_compile_rules(
                context,
                keymap_formats[f as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"latch\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    653 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                124 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                4294967295 as ::core::ffi::c_uint,
                BOTH as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                59 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x1008fe01 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                KEY_LVL3 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x71 as xkb_keysym_t, 0x51 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                58 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x71 as xkb_keysym_t, 0x51 as xkb_keysym_t),
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_YEN , BOTH, XKB_KEY_NoSymbol, NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, UINT32_MAX , BOTH, XKB_KEY_NoSymbol, NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_F1 , BOTH, XKB_KEY_XF86Switch_VT_1, NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_LVL3 , BOTH, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_Q , BOTH, get_keysym(keymap, XKB_KEY_q, XKB_KEY_Q), NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_CAPSLOCK , BOTH, XKB_KEY_ISO_Group_Latch , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_Q , BOTH, get_keysym(keymap, XKB_KEY_q, XKB_KEY_Q), FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    698 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                KEY_LVL3 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                KEY_LVL3 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                KEY_LVL3 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                KEY_LVL3 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_LVL3 , DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_LVL3 , UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_LVL3 , DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_LVL3 , UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_1 , BOTH, XKB_KEY_1 , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    720 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , UP, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , UP, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , UP, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock , NEXT, KEY_1 , BOTH, XKB_KEY_1 , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    769 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                59 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x1008fe01 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                58 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe06 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_F1 , BOTH, XKB_KEY_XF86Switch_VT_1, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_CAPSLOCK , BOTH, XKB_KEY_ISO_Group_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L, NEXT, KEY_Q , DOWN, XKB_KEY_Q , NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L, NEXT, KEY_Q , UP , XKB_KEY_q , NEXT, KEY_Q , DOWN, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L, NEXT, KEY_Q , UP , XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Q , DOWN, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L, NEXT, KEY_Q , UP , XKB_KEY_Q , NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    812 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                60 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xff7e as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x71 as xkb_keysym_t, 0x51 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0xb9 as xkb_keysym_t, 0xa1 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x71 as xkb_keysym_t, 0x51 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0xb9 as xkb_keysym_t, 0xa1 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                97 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe4 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                86 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe05 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTCTRL , BOTH, XKB_KEY_Control_R , NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_F2 , BOTH, XKB_KEY_ISO_Group_Shift, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_RIGHTCTRL , BOTH, XKB_KEY_Control_R, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_Q , BOTH, get_keysym(keymap, XKB_KEY_q, XKB_KEY_Q), NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, get_keysym(keymap, XKB_KEY_onesuperior, XKB_KEY_exclamdown), NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_RIGHTCTRL , DOWN, XKB_KEY_Control_R, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_RIGHTCTRL , UP , XKB_KEY_Control_R, NEXT, KEY_Q , BOTH, get_keysym(keymap, XKB_KEY_q, XKB_KEY_Q), NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, get_keysym(keymap, XKB_KEY_onesuperior, XKB_KEY_exclamdown), NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTCTRL , DOWN, XKB_KEY_Control_R, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTCTRL , UP , XKB_KEY_Control_R, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_RIGHTCTRL , DOWN, XKB_KEY_Control_R, NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_RIGHTCTRL , UP , XKB_KEY_Control_R, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level3_Lock, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_102ND , BOTH, XKB_KEY_ISO_Level3_Lock, NEXT, KEY_1 , BOTH, XKB_KEY_1 , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    892 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x51 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                54 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe2 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                54 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe2 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x71 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_Q , BOTH, XKB_KEY_Q , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L, NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Shift_R, NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Shift_R, NEXT, KEY_Q , BOTH, XKB_KEY_q , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    911 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                29 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe3 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x2b as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                54 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe2 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                54 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe2 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                54 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe2 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                54 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe2 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_LEFTCTRL , BOTH, XKB_KEY_Control_L, NEXT, KEY_LEFTALT , BOTH, XKB_KEY_Alt_L , NEXT, KEY_1 , BOTH, XKB_KEY_plus , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Shift_R , NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Shift_R , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Shift_R , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Shift_R , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_RIGHTALT , BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    973 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x21 as xkb_keysym_t, 0xa1 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x21 as xkb_keysym_t, 0xa1 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x21 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xb9 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x21 as xkb_keysym_t, 0xa1 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                get_keysym(keymap, 0x21 as xkb_keysym_t, 0xa1 as xkb_keysym_t),
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x31 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq(keymap, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , DOWN, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , UP , XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, get_keysym(keymap, XKB_KEY_exclam, XKB_KEY_exclamdown), NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, get_keysym(keymap, XKB_KEY_exclam, XKB_KEY_exclamdown), NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclam , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_exclamdown , NEXT, KEY_1 , BOTH, XKB_KEY_onesuperior , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_1 , BOTH, get_keysym(keymap, XKB_KEY_exclam, XKB_KEY_exclamdown), NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , NEXT, KEY_RIGHTALT , DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_RIGHTALT , UP , XKB_KEY_ISO_Level3_Latch, NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, get_keysym(keymap, XKB_KEY_exclam, XKB_KEY_exclamdown), NEXT, KEY_LEFTSHIFT , BOTH, XKB_KEY_Shift_L , NEXT, KEY_1 , BOTH, XKB_KEY_1 , FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1067 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            f = f.wrapping_add(1);
        }
        let mut f_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (f_0 as usize)
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            keymap = test_compile_rules(
                context,
                keymap_formats[f_0 as usize],
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"de\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b"lv3:ralt_latch\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1079 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if test_key_seq(
                keymap,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x61 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x61 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x61 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xe6 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                100 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe04 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                30 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x61 as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq((keymap), KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, FINISH)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1106 as ::core::ffi::c_uint,
                    b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            f_0 = f_0.wrapping_add(1);
        }
        keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"de\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"lv3:ralt_latch,lv3:latchOnRelease\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1117 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq((keymap), KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1118 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"de\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"lv3:ralt_latch,lv3:latchOnPress\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1129 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_ae, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_A , BOTH, XKB_KEY_a, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1154 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        let lock_breaks_latch: [::core::ffi::c_char; 426] = ::core::mem::transmute::<
            [u8; 426],
            [::core::ffi::c_char; 426],
        >(
            *b"xkb_keymap {\n  xkb_keycodes { <lshift> = 50; <a> = 38; };\n  xkb_types { include \"basic\" };\n  xkb_compat {\n    interpret ISO_Level2_Latch {\n      action = LatchMods(modifiers=Shift,latchToLock,clearLocks);\n    };\n    interpret Caps_Lock {\n      action = {LockMods(modifiers=Lock), VoidAction()};\n    };\n  };\n  xkb_symbols {\n    key <lshift> { [ISO_Level2_Latch, Caps_Lock], type=\"ALPHABETIC\" };\n    key <a> { [a, A] };\n  };\n};\0",
        );
        keymap = test_compile_buffer(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            &raw const lock_breaks_latch as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 426]>() as size_t,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe02 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe02 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x41 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_ISO_Level2_Latch, NEXT, KEY_A , BOTH, XKB_KEY_A, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_ISO_Level2_Latch, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_A , BOTH, XKB_KEY_A, NEXT, KEY_A , BOTH, XKB_KEY_A, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_A , BOTH, XKB_KEY_a, NEXT, KEY_A , BOTH, XKB_KEY_a, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1201 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        let lv5_latch_breaks_lv3_latch: [::core::ffi::c_char; 613] = ::core::mem::transmute::<
            [u8; 613],
            [::core::ffi::c_char; 613],
        >(
            *b"xkb_keymap {\n  xkb_keycodes { <lshift> = 50; <ralt> = 108; <e> = 26; <f> = 41; };\n  xkb_types  { include \"complete\" };\n  xkb_compat { include \"complete\" };\n  xkb_symbols {\n    virtual_modifiers LevelFive;\n    key <lshift> { [ISO_Level2_Latch], [LatchMods(modifiers=Shift)]};\n    key <ralt> { [ISO_Level3_Latch] };\n    key.type = \"EIGHT_LEVEL_SEMIALPHABETIC\";\n    key <e> { [e,          E,          EuroSign,         any, schwa, SCHWA] };\n    key <f> { [f,          F,          ISO_Level5_Latch, any, any,   any  ],\n              [NoAction(), NoAction(), {VoidAction(), LatchMods(modifiers=LevelFive)}] };\n  };\n};\0",
        );
        keymap = test_compile_buffer(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            &raw const lv5_latch_breaks_lv3_latch as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 613]>() as size_t,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1229 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x20ac as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            33 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1000259 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            33 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe02 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x100018f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_E , BOTH, XKB_KEY_e, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_E , BOTH, XKB_KEY_EuroSign, NEXT, KEY_E , BOTH, XKB_KEY_e, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_F, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_E , BOTH, XKB_KEY_schwa, NEXT, KEY_E , BOTH, XKB_KEY_e, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_F, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_LEFTSHIFT, BOTH, XKB_KEY_ISO_Level2_Latch, NEXT, KEY_E , BOTH, XKB_KEY_SCHWA, NEXT, KEY_E , BOTH, XKB_KEY_e, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1251 as ::core::ffi::c_uint,
                b"void test_mod_latch(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_explicit_actions(mut ctx: *mut xkb_context) {
    unsafe {
        let mut original: *mut xkb_keymap = test_compile_file(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"keymaps/explicit-actions.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !original.is_null() {
        } else {
            __assert_fail(
                b"original\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1267 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut dump: *mut ::core::ffi::c_char =
            xkb_keymap_get_as_string(original, XKB_KEYMAP_USE_ORIGINAL_FORMAT);
        if !dump.is_null() {
        } else {
            __assert_fail(
                b"dump\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1272 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut roundtrip: *mut xkb_keymap =
            test_compile_string(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, dump);
        free(dump as *mut ::core::ffi::c_void);
        let mut keymaps: [*mut xkb_keymap; 2] = [original, roundtrip];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[*mut xkb_keymap; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut xkb_keymap>() as usize)
        {
            if !keymaps[k as usize].is_null() {
            } else {
                __assert_fail(
                    b"keymaps[k]\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1280 as ::core::ffi::c_uint,
                    b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let keys: [key_properties; 4] = [
                key_properties {
                    name: b"LALT\0".as_ptr() as *const ::core::ffi::c_char,
                    repeats: false_0 != 0,
                    vmodmap: 0 as xkb_mod_mask_t,
                },
                key_properties {
                    name: b"LVL3\0".as_ptr() as *const ::core::ffi::c_char,
                    repeats: false_0 != 0,
                    vmodmap: (1 as xkb_mod_mask_t) << 10 as ::core::ffi::c_int,
                },
                key_properties {
                    name: b"AD05\0".as_ptr() as *const ::core::ffi::c_char,
                    repeats: true_0 != 0,
                    vmodmap: 0 as xkb_mod_mask_t,
                },
                key_properties {
                    name: b"AD06\0".as_ptr() as *const ::core::ffi::c_char,
                    repeats: true_0 != 0,
                    vmodmap: 0 as xkb_mod_mask_t,
                },
            ];
            let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[key_properties; 4]>() as usize)
                    .wrapping_div(::core::mem::size_of::<key_properties>() as usize)
            {
                let mut kc: xkb_keycode_t =
                    xkb_keymap_key_by_name(keymaps[k as usize], keys[i as usize].name);
                if kc != 0xffffffff as xkb_keycode_t {
                } else {
                    __assert_fail(
                        b"kc != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1302 as ::core::ffi::c_uint,
                        b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                if keys[i as usize].repeats as ::core::ffi::c_int
                    == xkb_keymap_key_repeats(keymaps[k as usize], kc)
                {
                } else {
                    __assert_fail(
                        b"keys[i].repeats == xkb_keymap_key_repeats(keymaps[k], kc)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1303 as ::core::ffi::c_uint,
                        b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                if keys[i as usize].vmodmap
                    == (*(*keymaps[k as usize]).keys.offset(kc as isize)).vmodmap
                {
                } else {
                    __assert_fail(
                        b"keys[i].vmodmap == keymaps[k]->keys[kc].vmodmap\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1304 as ::core::ffi::c_uint,
                        b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                i = i.wrapping_add(1);
            }
            if test_key_seq(
                keymaps[k as usize],
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x79 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x59 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x7a as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x8fb as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                127 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xfe08 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6b as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe11 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0xa1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe11 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xffe1 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                DOWN as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x7ea as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                56 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xfe03 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                42 as ::core::ffi::c_int,
                UP as ::core::ffi::c_int,
                0xffe5 as ::core::ffi::c_int,
                NEXT as ::core::ffi::c_int,
                21 as ::core::ffi::c_int,
                BOTH as ::core::ffi::c_int,
                0x6b as ::core::ffi::c_int,
                FINISH as ::core::ffi::c_int,
            ) != 0
            {
            } else {
                __assert_fail(
                    b"test_key_seq( keymaps[k], KEY_Y, BOTH, XKB_KEY_y, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_Y, BOTH, XKB_KEY_Y, NEXT, KEY_LEFTALT, UP, XKB_KEY_Shift_L, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_z, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_Y, BOTH, XKB_KEY_leftarrow, NEXT, KEY_LEFTALT, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y, BOTH, XKB_KEY_k, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_Y, BOTH, XKB_KEY_exclamdown, NEXT, KEY_LEFTALT, UP, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_Y, BOTH, XKB_KEY_Greek_kappa, NEXT, KEY_LEFTALT, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_Y, BOTH, XKB_KEY_k, FINISH )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1328 as ::core::ffi::c_uint,
                    b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymaps[k as usize]);
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_simultaneous_modifier_clear(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"simultaneous-mods-latches\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1341 as ::core::ffi::c_uint,
                b"void test_simultaneous_modifier_clear(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_ydiaeresis , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_Z , BOTH, XKB_KEY_z , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1357 as ::core::ffi::c_uint,
                b"void test_simultaneous_modifier_clear(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            97 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe4 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            45 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_RIGHTCTRL, BOTH, XKB_KEY_Control_R , NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_LEFTMETA, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_ydiaeresis , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_X , BOTH, XKB_KEY_x , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1368 as ::core::ffi::c_uint,
                b"void test_simultaneous_modifier_clear(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            125 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            126 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTMETA, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTMETA, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_ydiaeresis , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_Z , BOTH, XKB_KEY_z , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1377 as ::core::ffi::c_uint,
                b"void test_simultaneous_modifier_clear(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            125 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe04 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTMETA, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_y , NEXT, KEY_Z , BOTH, XKB_KEY_y , NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level3_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_Z , BOTH, XKB_KEY_z , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1392 as ::core::ffi::c_uint,
                b"void test_simultaneous_modifier_clear(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            97 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe4 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            126 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1000292 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1000292 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            126 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe12 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            45 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_RIGHTCTRL, BOTH, XKB_KEY_Control_R , NEXT, KEY_RIGHTALT, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_RIGHTMETA, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_ezh , NEXT, KEY_Z , BOTH, XKB_KEY_ezh , NEXT, KEY_RIGHTMETA, BOTH, XKB_KEY_ISO_Level5_Latch, NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_X , BOTH, XKB_KEY_x , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1405 as ::core::ffi::c_uint,
                b"void test_simultaneous_modifier_clear(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_keymaps(
    mut ctx: *mut xkb_context,
    mut rules: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1415 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,il,ru,de\0".as_ptr() as *const ::core::ffi::c_char,
            b",,phonetic,neo\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:alt_shift_toggle,grp:menu_toggle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1421 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_L, BOTH, XKB_KEY_l, NEXT, KEY_L, BOTH, XKB_KEY_l, NEXT, KEY_O, BOTH, XKB_KEY_o, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1428 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x45 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_E, BOTH, XKB_KEY_E, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_L, BOTH, XKB_KEY_l, NEXT, KEY_O, BOTH, XKB_KEY_o, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1438 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, DOWN, XKB_KEY_h, NEXT, KEY_H, REPEAT, XKB_KEY_h, NEXT, KEY_H, REPEAT, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_H, REPEAT, XKB_KEY_H, NEXT, KEY_H, REPEAT, XKB_KEY_H, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_H, REPEAT, XKB_KEY_h, NEXT, KEY_H, REPEAT, XKB_KEY_h, NEXT, KEY_H, UP, XKB_KEY_h, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1452 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x45 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_E, BOTH, XKB_KEY_E, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_O, BOTH, XKB_KEY_O, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1462 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x45 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_E, BOTH, XKB_KEY_E, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_RIGHTSHIFT, UP, XKB_KEY_Shift_R, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_O, BOTH, XKB_KEY_O, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1472 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x45 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_E, BOTH, XKB_KEY_E, NEXT, KEY_RIGHTSHIFT, DOWN, XKB_KEY_Shift_R, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_RIGHTSHIFT, UP, XKB_KEY_Shift_R, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_O, BOTH, XKB_KEY_o, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1487 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1502 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_Caps_Lock, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_Caps_Lock, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_Caps_Lock, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_Caps_Lock, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1516 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            37 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xcec as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            33 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xceb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_K, BOTH, XKB_KEY_hebrew_lamed, NEXT, KEY_F, BOTH, XKB_KEY_hebrew_kaph, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_O, BOTH, XKB_KEY_o, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1528 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_LEFTALT, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1534 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_ISO_Next_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_ISO_Next_Group, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1540 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x48 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x45 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_H, BOTH, XKB_KEY_H, NEXT, KEY_E, BOTH, XKB_KEY_E, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_O, BOTH, XKB_KEY_O, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1549 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_O, BOTH, XKB_KEY_o, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1558 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x45 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x4f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_Caps_Lock, NEXT, KEY_E, BOTH, XKB_KEY_E, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_L, BOTH, XKB_KEY_L, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_Caps_Lock, NEXT, KEY_O, BOTH, XKB_KEY_O, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1567 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x65 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            38 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            24 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_E, BOTH, XKB_KEY_e, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_Caps_Lock, NEXT, KEY_L, BOTH, XKB_KEY_l, NEXT, KEY_L, BOTH, XKB_KEY_l, NEXT, KEY_O, BOTH, XKB_KEY_o, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1575 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            79 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff9c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            69 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff7f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            79 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            80 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            69 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff7f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            80 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff99 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_KP1, BOTH, XKB_KEY_KP_End, NEXT, KEY_NUMLOCK, BOTH, XKB_KEY_Num_Lock, NEXT, KEY_KP1, BOTH, XKB_KEY_KP_1, NEXT, KEY_KP2, BOTH, XKB_KEY_KP_2, NEXT, KEY_NUMLOCK, BOTH, XKB_KEY_Num_Lock, NEXT, KEY_KP2, BOTH, XKB_KEY_KP_Down, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1598 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x31 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x21 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x31 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6f6 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_1, BOTH, XKB_KEY_1, NEXT, KEY_Q, BOTH, XKB_KEY_Cyrillic_ya, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_1, BOTH, XKB_KEY_exclam, NEXT, KEY_Q, BOTH, XKB_KEY_Cyrillic_YA, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_V, BOTH, XKB_KEY_Cyrillic_zhe, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_1, BOTH, XKB_KEY_1, NEXT, KEY_V, BOTH, XKB_KEY_Cyrillic_ZHE, NEXT, KEY_RIGHTSHIFT, DOWN, XKB_KEY_Shift_R, NEXT, KEY_V, BOTH, XKB_KEY_Cyrillic_zhe, NEXT, KEY_RIGHTSHIFT, UP, XKB_KEY_Shift_R, NEXT, KEY_V, BOTH, XKB_KEY_Cyrillic_ZHE, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1617 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x31 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            71 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb7 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xb0 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x58 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            71 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            xkb_keysym_from_name(
                b"U2714\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_KEYSYM_NO_FLAGS,
            ),
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            7 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x36 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x53 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            81 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            7 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xa2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xaae as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            71 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            xkb_keysym_from_name(
                b"U2195\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_KEYSYM_NO_FLAGS,
            ),
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xaf7 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7eb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            57 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xa0 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            72 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x8dc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x70 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_1, BOTH, XKB_KEY_1, NEXT, KEY_Q, BOTH, XKB_KEY_x, NEXT, KEY_KP7, BOTH, XKB_KEY_KP_7, NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_1, BOTH, XKB_KEY_degree, NEXT, KEY_Q, BOTH, XKB_KEY_X, NEXT, KEY_KP7, BOTH, KS(\"U2714\"), NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_6, BOTH, XKB_KEY_6, NEXT, KEY_H, BOTH, XKB_KEY_S, NEXT, KEY_KP3, BOTH, XKB_KEY_KP_3, NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_RIGHTSHIFT, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_6, BOTH, XKB_KEY_cent, NEXT, KEY_Q, BOTH, XKB_KEY_ellipsis, NEXT, KEY_KP7, BOTH, KS(\"U2195\"), NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_5, BOTH, XKB_KEY_malesymbol, NEXT, KEY_E, BOTH, XKB_KEY_Greek_lambda, NEXT, KEY_SPACE, BOTH, XKB_KEY_nobreakspace, NEXT, KEY_KP8, BOTH, XKB_KEY_intersection, NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_V, BOTH, XKB_KEY_p, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1692 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            rules,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"de\0".as_ptr() as *const ::core::ffi::c_char,
            b"neo\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1697 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xb7 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff52 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            57 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb0 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            72 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff97 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe20 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff52 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            57 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb0 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            72 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff97 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            xkb_keysym_from_name(
                b"U2221\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_KEYSYM_NO_FLAGS,
            ),
            NEXT as ::core::ffi::c_int,
            18 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7cb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            57 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            xkb_keysym_from_name(
                b"U202F\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_KEYSYM_NO_FLAGS,
            ),
            NEXT as ::core::ffi::c_int,
            72 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            xkb_keysym_from_name(
                b"U22C2\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_KEYSYM_NO_FLAGS,
            ),
            NEXT as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff1b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            15 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7f0 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            54 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x70 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            47 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff0d as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_5, BOTH, XKB_KEY_periodcentered, NEXT, KEY_E, BOTH, XKB_KEY_Up, NEXT, KEY_SPACE, BOTH, XKB_KEY_KP_0, NEXT, KEY_KP8, BOTH, XKB_KEY_KP_Up, NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level5_Lock, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_RIGHTSHIFT, DOWN, XKB_KEY_Shift_R, NEXT, KEY_5, BOTH, XKB_KEY_NoSymbol, NEXT, KEY_8, BOTH, XKB_KEY_ISO_Left_Tab, NEXT, KEY_E, BOTH, XKB_KEY_Up, NEXT, KEY_SPACE, BOTH, XKB_KEY_KP_0, NEXT, KEY_KP8, BOTH, XKB_KEY_KP_Up, NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_RIGHTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level5_Lock, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_5, BOTH, KS(\"U2221\"), NEXT, KEY_E, BOTH, XKB_KEY_Greek_LAMBDA, NEXT, KEY_SPACE, BOTH, KS(\"U202F\"), NEXT, KEY_KP8, BOTH, KS(\"U22C2\"), NEXT, KEY_ESC, BOTH, XKB_KEY_Escape, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level5_Lock, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_CAPSLOCK, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_RIGHTSHIFT, DOWN, XKB_KEY_Shift_R, NEXT, KEY_TAB, BOTH, XKB_KEY_ISO_Level5_Lock, NEXT, KEY_V, BOTH, XKB_KEY_Greek_pi, NEXT, KEY_RIGHTSHIFT, UP, XKB_KEY_Caps_Lock, NEXT, KEY_V, BOTH, XKB_KEY_asciitilde, NEXT, KEY_CAPSLOCK, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_V, BOTH, XKB_KEY_p, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level5_Lock, NEXT, KEY_V, BOTH, XKB_KEY_Return, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1744 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            rules,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,il,ru\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:alt_shift_toggle_bidir,grp:menu_toggle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1751 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTALT, UP, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1757 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_LEFTALT, DOWN, XKB_KEY_Alt_L, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTALT, UP, XKB_KEY_Alt_L, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1763 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe0a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_er, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_LEFTALT, BOTH, XKB_KEY_ISO_Prev_Group, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_er, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1785 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            rules,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,il,ru\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:switch,grp:lswitch,grp:menu_toggle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1791 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1803 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d2 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_er, NEXT, KEY_LEFTALT, UP, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_LEFTALT, UP, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_er, NEXT, KEY_COMPOSE, BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_LEFTALT, DOWN, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_Cyrillic_er, NEXT, KEY_LEFTALT, UP, XKB_KEY_Mode_switch, NEXT, KEY_H, BOTH, XKB_KEY_hebrew_yod, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_H, BOTH, XKB_KEY_h, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1829 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            rules,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            b"euro\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1834 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x35 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x20ac as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe03 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_5, BOTH, XKB_KEY_5, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_5, BOTH, XKB_KEY_EuroSign, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Shift, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1840 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_file(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"keymaps/unbound-vmod.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1846 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            35 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x68 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            12 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xdf as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_H, BOTH, XKB_KEY_h, NEXT, KEY_Z, BOTH, XKB_KEY_y, NEXT, KEY_MINUS, BOTH, XKB_KEY_ssharp, NEXT, KEY_Z, BOTH, XKB_KEY_y, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1852 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            rules,
            b"applealu_ansi\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"terminate:ctrl_alt_bksp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1858 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq(
            keymap,
            6 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x35 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            79 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            69 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xff0b as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            79 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            79 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            79 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffb1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            42 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            58 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq(keymap, KEY_5, BOTH, XKB_KEY_5, NEXT, KEY_KP1, BOTH, XKB_KEY_KP_1, NEXT, KEY_NUMLOCK, BOTH, XKB_KEY_Clear, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_KP1, BOTH, XKB_KEY_KP_1, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_KP1, BOTH, XKB_KEY_KP_1, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_KP1, BOTH, XKB_KEY_KP_1, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_CAPSLOCK, BOTH, XKB_KEY_Caps_Lock, NEXT, KEY_A, BOTH, XKB_KEY_a, FINISH)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1873 as ::core::ffi::c_uint,
                b"void test_keymaps(struct xkb_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe fn main_0() -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/keyseq.c\0".as_ptr() as *const ::core::ffi::c_char,
                1884 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        test_keymaps(ctx, b"evdev\0".as_ptr() as *const ::core::ffi::c_char);
        test_keymaps(
            ctx,
            b"evdev-pure-virtual-mods\0".as_ptr() as *const ::core::ffi::c_char,
        );
        test_simultaneous_modifier_clear(ctx);
        test_group_lock(ctx);
        test_group_latch(ctx);
        test_mod_set(ctx);
        test_mod_lock(ctx);
        test_mod_latch(ctx);
        test_explicit_actions(ctx);
        xkb_context_unref(ctx);
        return EXIT_SUCCESS;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
