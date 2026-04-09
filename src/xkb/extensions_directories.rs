pub mod internal {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
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
pub mod stdio_h {
    pub type va_list = __gnuc_va_list;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    use super::FILE_h::FILE;

    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn vsnprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
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
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_context_num_include_paths(context: *mut xkb_context) -> ::core::ffi::c_uint;
        pub fn xkb_context_include_path_get(
            context: *mut xkb_context,
            index: ::core::ffi::c_uint,
        ) -> *const ::core::ffi::c_char;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
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
pub mod xkbregistry_h {
    pub type rxkb_popularity = ::core::ffi::c_uint;
    pub const RXKB_POPULARITY_EXOTIC: rxkb_popularity = 2;
    pub const RXKB_POPULARITY_STANDARD: rxkb_popularity = 1;
    pub type rxkb_context_flags = ::core::ffi::c_uint;
    pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
    pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
    pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
    pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;
    extern "C" {
        pub type rxkb_context;
        pub type rxkb_layout;
        pub type rxkb_option_group;
        pub type rxkb_option;
        pub fn rxkb_context_new(flags: rxkb_context_flags) -> *mut rxkb_context;
        pub fn rxkb_context_parse(
            ctx: *mut rxkb_context,
            ruleset: *const ::core::ffi::c_char,
        ) -> bool;
        pub fn rxkb_context_unref(ctx: *mut rxkb_context) -> *mut rxkb_context;
        pub fn rxkb_layout_first(ctx: *mut rxkb_context) -> *mut rxkb_layout;
        pub fn rxkb_layout_next(l: *mut rxkb_layout) -> *mut rxkb_layout;
        pub fn rxkb_layout_ref(l: *mut rxkb_layout) -> *mut rxkb_layout;
        pub fn rxkb_layout_unref(l: *mut rxkb_layout) -> *mut rxkb_layout;
        pub fn rxkb_layout_get_name(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_variant(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_description(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_popularity(l: *mut rxkb_layout) -> rxkb_popularity;
        pub fn rxkb_option_group_first(ctx: *mut rxkb_context) -> *mut rxkb_option_group;
        pub fn rxkb_option_group_next(g: *mut rxkb_option_group) -> *mut rxkb_option_group;
        pub fn rxkb_option_group_get_name(m: *mut rxkb_option_group) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_first(group: *mut rxkb_option_group) -> *mut rxkb_option;
        pub fn rxkb_option_next(o: *mut rxkb_option) -> *mut rxkb_option;
        pub fn rxkb_option_ref(o: *mut rxkb_option) -> *mut rxkb_option;
        pub fn rxkb_option_unref(o: *mut rxkb_option) -> *mut rxkb_option;
        pub fn rxkb_option_get_name(o: *mut rxkb_option) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_get_description(o: *mut rxkb_option) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_get_popularity(o: *mut rxkb_option) -> rxkb_popularity;
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
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
            return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return s1 == s2;
            }
            return streq(s1, s2);
        }
    }
    #[inline]
    pub unsafe extern "C" fn snprintf_safe(
        mut buf: *mut ::core::ffi::c_char,
        mut sz: usize,
        mut format: *const ::core::ffi::c_char,
        mut c2rust_args: ...
    ) -> bool {
        unsafe {
            let mut ap: ::core::ffi::VaList;
            let mut rc: ::core::ffi::c_int = 0;
            ap = c2rust_args.clone();
            rc = vsnprintf(buf, sz, format, ap);
            return rc >= 0 as ::core::ffi::c_int && (rc as usize) < sz;
        }
    }

    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::stdio_h::vsnprintf;
    use super::string_h::strcmp;
}
pub mod test_h {
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::xkbcommon_h::xkb_keymap_format;
    extern "C" {
        pub fn test_init();
        pub fn test_get_path(path_rel: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub mod assert_h {
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 40] = unsafe {
        ::core::mem::transmute::<[u8; 40], [::core::ffi::c_char; 40]>(
            *b"_Bool streq(const char *, const char *)\0",
        )
    };
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
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn exit(__status: ::core::ffi::c_int) -> !;
        pub fn setenv(
            __name: *const ::core::ffi::c_char,
            __value: *const ::core::ffi::c_char,
            __replace: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    }
}
pub mod string_h {
    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
    pub const NULL_0: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::{NULL, NULL_0};

pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::{__builtin_va_list, __va_list_tag};
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
pub use self::stdio_h::{fprintf, stderr, va_list, vsnprintf};
pub use self::stdlib_h::{exit, free, setenv, unsetenv, EXIT_FAILURE, EXIT_SUCCESS};
use self::string_h::strcmp;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
use self::test_h::{test_compile_rules, test_get_path, test_init};
pub use self::types_h::{
    __int16_t, __int32_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use self::utils_h::{snprintf_safe, streq, streq_null};
pub use self::xkbcommon_h::{
    xkb_context_flags, xkb_context_include_path_get, xkb_context_new,
    xkb_context_num_include_paths, xkb_context_unref, xkb_keycode_t, xkb_keymap_compile_flags,
    xkb_keymap_format, xkb_keymap_key_get_syms_by_level, xkb_keymap_unref, xkb_keysym_t,
    xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
    xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
    xkb_state_component, XKB_CONTEXT_NO_DEFAULT_INCLUDES, XKB_CONTEXT_NO_ENVIRONMENT_NAMES,
    XKB_CONTEXT_NO_FLAGS, XKB_CONTEXT_NO_SECURE_GETENV, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbregistry_h::{
    rxkb_context, rxkb_context_flags, rxkb_context_new, rxkb_context_parse, rxkb_context_unref,
    rxkb_layout, rxkb_layout_first, rxkb_layout_get_description, rxkb_layout_get_name,
    rxkb_layout_get_popularity, rxkb_layout_get_variant, rxkb_layout_next, rxkb_layout_ref,
    rxkb_layout_unref, rxkb_option, rxkb_option_first, rxkb_option_get_description,
    rxkb_option_get_name, rxkb_option_get_popularity, rxkb_option_group, rxkb_option_group_first,
    rxkb_option_group_get_name, rxkb_option_group_next, rxkb_option_next, rxkb_option_ref,
    rxkb_option_unref, rxkb_popularity, RXKB_CONTEXT_LOAD_EXOTIC_RULES,
    RXKB_CONTEXT_NO_DEFAULT_INCLUDES, RXKB_CONTEXT_NO_FLAGS, RXKB_CONTEXT_NO_SECURE_GETENV,
    RXKB_POPULARITY_EXOTIC, RXKB_POPULARITY_STANDARD,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub layout: *const ::core::ffi::c_char,
    pub variant: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub group: *const ::core::ffi::c_char,
    pub option: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
}
unsafe extern "C" fn fetch_layout(
    mut ctx: *mut rxkb_context,
    mut layout: *const ::core::ffi::c_char,
    mut variant: *const ::core::ffi::c_char,
) -> *mut rxkb_layout {
    unsafe {
        let mut l: *mut rxkb_layout = rxkb_layout_first(ctx);
        while !l.is_null() {
            let mut v: *const ::core::ffi::c_char = rxkb_layout_get_variant(l);
            if streq(rxkb_layout_get_name(l), layout) as ::core::ffi::c_int != 0
                && (v.is_null() && variant.is_null()
                    || !v.is_null()
                        && !variant.is_null()
                        && streq(v, variant) as ::core::ffi::c_int != 0)
            {
                return rxkb_layout_ref(l);
            }
            l = rxkb_layout_next(l);
        }
        return ::core::ptr::null_mut::<rxkb_layout>();
    }
}
unsafe extern "C" fn test_layouts(
    mut xkb_root: *const ::core::ffi::c_char,
    mut update_output_files: bool,
) {
    unsafe {
        let mut ctx: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut keysyms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let mut unversioned_extensions_path: *mut ::core::ffi::c_char =
            test_get_path(b"extensions/without-rules\0".as_ptr() as *const ::core::ffi::c_char);
        if !unversioned_extensions_path.is_null() {
        } else {
            __assert_fail(
                b"unversioned_extensions_path\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                53 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut versioned_extensions_path: *mut ::core::ffi::c_char =
            test_get_path(b"extensions/without-rules-2\0".as_ptr() as *const ::core::ffi::c_char);
        if !versioned_extensions_path.is_null() {
        } else {
            __assert_fail(
                b"versioned_extensions_path\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        setenv(
            b"XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
            unversioned_extensions_path,
            1 as ::core::ffi::c_int,
        );
        setenv(
            b"XKB_CONFIG_VERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
            versioned_extensions_path,
            1 as ::core::ffi::c_int,
        );
        ctx = xkb_context_new(XKB_CONTEXT_NO_ENVIRONMENT_NAMES);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut buf: [::core::ffi::c_char; 4096] = ::core::mem::transmute::<
            [u8; 4096],
            [::core::ffi::c_char; 4096],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        if xkb_context_num_include_paths(ctx) == 4 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"xkb_context_num_include_paths(ctx) == 4\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if snprintf_safe(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            versioned_extensions_path,
            b"p1\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"snprintf_safe(buf, sizeof(buf), \"%s/%s\", versioned_extensions_path, \"p1\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                64 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 0 as ::core::ffi::c_uint),
            &raw mut buf as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 0), buf) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if snprintf_safe(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            versioned_extensions_path,
            b"p2\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"snprintf_safe(buf, sizeof(buf), \"%s/%s\", versioned_extensions_path, \"p2\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 1 as ::core::ffi::c_uint),
            &raw mut buf as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 1), buf) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if snprintf_safe(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            unversioned_extensions_path,
            b"p3\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"snprintf_safe(buf, sizeof(buf), \"%s/%s\", unversioned_extensions_path, \"p3\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 2 as ::core::ffi::c_uint),
            &raw mut buf as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 2), buf) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 3 as ::core::ffi::c_uint),
            xkb_root,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 3), xkb_root) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        free(unversioned_extensions_path as *mut ::core::ffi::c_void);
        free(versioned_extensions_path as *mut ::core::ffi::c_void);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"a,b,c\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                80 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as xkb_keycode_t,
                0 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_A, 0, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0x7e1 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_Greek_alpha\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                83 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as xkb_keycode_t,
                1 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_A, 1, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0xe1 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_aacute\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as xkb_keycode_t,
                2 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_A, 2, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0xe4 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_adiaeresis\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        xkb_context_unref(ctx);
        let mut rctx: *mut rxkb_context = rxkb_context_new(RXKB_CONTEXT_LOAD_EXOTIC_RULES);
        if !rctx.is_null() {
        } else {
            __assert_fail(
                b"rctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if rxkb_context_parse(rctx, b"evdev\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"rxkb_context_parse(rctx, \"evdev\")\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_uint,
                b"void test_layouts(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut registry_tests: [C2Rust_Unnamed_13; 3] = [
            C2Rust_Unnamed_13 {
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                description: b"A\0".as_ptr() as *const ::core::ffi::c_char,
                popularity: RXKB_POPULARITY_STANDARD,
            },
            C2Rust_Unnamed_13 {
                layout: b"b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                description: b"B\0".as_ptr() as *const ::core::ffi::c_char,
                popularity: RXKB_POPULARITY_EXOTIC,
            },
            C2Rust_Unnamed_13 {
                layout: b"c\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                description: b"C\0".as_ptr() as *const ::core::ffi::c_char,
                popularity: RXKB_POPULARITY_STANDARD,
            },
        ];
        let mut t: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (t as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_13; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_13>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_layouts\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            let l: *mut rxkb_layout = fetch_layout(
                rctx,
                registry_tests[t as usize].layout,
                registry_tests[t as usize].variant,
            ) as *mut rxkb_layout;
            if !l.is_null() {
            } else {
                __assert_fail(
                    b"l\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                    129 as ::core::ffi::c_uint,
                    b"void test_layouts(const char *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if streq_null(
                rxkb_layout_get_description(l),
                registry_tests[t as usize].description,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"streq_null(rxkb_layout_get_description(l), registry_tests[t].description)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                    131 as ::core::ffi::c_uint,
                    b"void test_layouts(const char *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if rxkb_layout_get_popularity(l) as ::core::ffi::c_uint
                == registry_tests[t as usize].popularity as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"rxkb_layout_get_popularity(l) == registry_tests[t].popularity\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                    132 as ::core::ffi::c_uint,
                    b"void test_layouts(const char *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            rxkb_layout_unref(l);
            t = t.wrapping_add(1);
        }
        rxkb_context_unref(rctx);
    }
}
unsafe extern "C" fn fetch_option(
    mut ctx: *mut rxkb_context,
    mut grp: *const ::core::ffi::c_char,
    mut opt: *const ::core::ffi::c_char,
) -> *mut rxkb_option {
    unsafe {
        let mut g: *mut rxkb_option_group = rxkb_option_group_first(ctx);
        while !g.is_null() {
            if streq(grp, rxkb_option_group_get_name(g)) {
                let mut o: *mut rxkb_option = rxkb_option_first(g);
                while !o.is_null() {
                    if streq(opt, rxkb_option_get_name(o)) {
                        return rxkb_option_ref(o);
                    }
                    o = rxkb_option_next(o);
                }
            }
            g = rxkb_option_group_next(g);
        }
        return ::core::ptr::null_mut::<rxkb_option>();
    }
}
unsafe extern "C" fn test_options(
    mut xkb_root: *const ::core::ffi::c_char,
    mut update_output_files: bool,
) {
    unsafe {
        let mut ctx: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut keysyms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let mut unversioned_extensions_path: *mut ::core::ffi::c_char =
            test_get_path(b"extensions/with-rules\0".as_ptr() as *const ::core::ffi::c_char);
        if !unversioned_extensions_path.is_null() {
        } else {
            __assert_fail(
                b"unversioned_extensions_path\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                169 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut versioned_extensions_path: *mut ::core::ffi::c_char =
            test_get_path(b"extensions/with-rules-2\0".as_ptr() as *const ::core::ffi::c_char);
        if !versioned_extensions_path.is_null() {
        } else {
            __assert_fail(
                b"versioned_extensions_path\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        setenv(
            b"XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
            unversioned_extensions_path,
            1 as ::core::ffi::c_int,
        );
        setenv(
            b"XKB_CONFIG_VERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
            versioned_extensions_path,
            1 as ::core::ffi::c_int,
        );
        ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                175 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut buf: [::core::ffi::c_char; 4096] = ::core::mem::transmute::<
            [u8; 4096],
            [::core::ffi::c_char; 4096],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        if xkb_context_num_include_paths(ctx) == 4 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"xkb_context_num_include_paths(ctx) == 4\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                178 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if snprintf_safe(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            versioned_extensions_path,
            b"p1\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"snprintf_safe(buf, sizeof(buf), \"%s/%s\", versioned_extensions_path, \"p1\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                180 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 0 as ::core::ffi::c_uint),
            &raw mut buf as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 0), buf) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if snprintf_safe(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            versioned_extensions_path,
            b"p2\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"snprintf_safe(buf, sizeof(buf), \"%s/%s\", versioned_extensions_path, \"p2\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                183 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 1 as ::core::ffi::c_uint),
            &raw mut buf as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 1), buf) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if snprintf_safe(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            unversioned_extensions_path,
            b"p3\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"snprintf_safe(buf, sizeof(buf), \"%s/%s\", unversioned_extensions_path, \"p3\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                186 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 2 as ::core::ffi::c_uint),
            &raw mut buf as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 2), buf) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if strcmp(
            xkb_context_include_path_get(ctx, 3 as ::core::ffi::c_uint),
            xkb_root,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"strcmp(xkb_context_include_path_get(ctx, 3), xkb_root) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        free(unversioned_extensions_path as *mut ::core::ffi::c_void);
        free(versioned_extensions_path as *mut ::core::ffi::c_void);
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"cz,ca,de\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"opt:1,opt:2,opt:3!2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                197 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as xkb_keycode_t,
                0 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_A, 0, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                199 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0x7e1 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_Greek_alpha\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                200 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as xkb_keycode_t,
                0 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_S, 0, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                202 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0x1b6 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_sacute\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                203 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as xkb_keycode_t,
                1 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_A, 1, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0xe4 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_adiaeresis\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                206 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as xkb_keycode_t,
                1 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_S, 1, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                208 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0x1b6 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_sacute\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as xkb_keycode_t,
                2 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_A, 2, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0x61 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_a\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                212 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if 1 as ::core::ffi::c_int
            == xkb_keymap_key_get_syms_by_level(
                keymap,
                (8 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as xkb_keycode_t,
                2 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut keysyms,
            )
        {
        } else {
            __assert_fail(
                b"1 == xkb_keymap_key_get_syms_by_level(keymap, EVDEV_OFFSET + KEY_S, 2, 0, &keysyms)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                214 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if *keysyms.offset(0 as ::core::ffi::c_int as isize) == 0x1b6 as xkb_keysym_t {
        } else {
            __assert_fail(
                b"keysyms[0] == XKB_KEY_sacute\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                215 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        xkb_context_unref(ctx);
        let mut rctx: *mut rxkb_context = rxkb_context_new(RXKB_CONTEXT_LOAD_EXOTIC_RULES);
        if !rctx.is_null() {
        } else {
            __assert_fail(
                b"rctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if rxkb_context_parse(rctx, b"evdev\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"rxkb_context_parse(rctx, \"evdev\")\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_uint,
                b"void test_options(const char *, _Bool)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut registry_tests: [C2Rust_Unnamed_14; 3] = [
            C2Rust_Unnamed_14 {
                group: b"opt\0".as_ptr() as *const ::core::ffi::c_char,
                option: b"opt:1\0".as_ptr() as *const ::core::ffi::c_char,
                description: b"1\0".as_ptr() as *const ::core::ffi::c_char,
                popularity: RXKB_POPULARITY_STANDARD,
            },
            C2Rust_Unnamed_14 {
                group: b"opt\0".as_ptr() as *const ::core::ffi::c_char,
                option: b"opt:2\0".as_ptr() as *const ::core::ffi::c_char,
                description: b"2\0".as_ptr() as *const ::core::ffi::c_char,
                popularity: RXKB_POPULARITY_EXOTIC,
            },
            C2Rust_Unnamed_14 {
                group: b"opt\0".as_ptr() as *const ::core::ffi::c_char,
                option: b"opt:3\0".as_ptr() as *const ::core::ffi::c_char,
                description: b"3\0".as_ptr() as *const ::core::ffi::c_char,
                popularity: RXKB_POPULARITY_STANDARD,
            },
        ];
        let mut t: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (t as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_14; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_14>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_options\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            let opt: *mut rxkb_option = fetch_option(
                rctx,
                registry_tests[t as usize].group,
                registry_tests[t as usize].option,
            ) as *mut rxkb_option;
            if !opt.is_null() {
            } else {
                __assert_fail(
                    b"opt\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                    256 as ::core::ffi::c_uint,
                    b"void test_options(const char *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if streq_null(
                rxkb_option_get_description(opt),
                registry_tests[t as usize].description,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"streq_null(rxkb_option_get_description(opt), registry_tests[t].description)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                    258 as ::core::ffi::c_uint,
                    b"void test_options(const char *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if rxkb_option_get_popularity(opt) as ::core::ffi::c_uint
                == registry_tests[t as usize].popularity as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"rxkb_option_get_popularity(opt) == registry_tests[t].popularity\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                    259 as ::core::ffi::c_uint,
                    b"void test_options(const char *, _Bool)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            rxkb_option_unref(opt);
            t = t.wrapping_add(1);
        }
        rxkb_context_unref(rctx);
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut update_output_files: bool = false_0 != 0;
        if argc > 1 as ::core::ffi::c_int {
            if streq(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"update\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                update_output_files = true_0 != 0;
            } else {
                fprintf(
                    stderr,
                    b"ERROR: unsupported argument: \"%s\".\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                );
                exit(EXIT_FAILURE);
            }
        }
        unsetenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
        unsetenv(b"XDG_CONFIG_HOME\0".as_ptr() as *const ::core::ffi::c_char);
        unsetenv(b"XDG_CONFIG_DIR\0".as_ptr() as *const ::core::ffi::c_char);
        setenv(
            b"XKB_CONFIG_EXTRA_PATH\0".as_ptr() as *const ::core::ffi::c_char,
            b"\xC2\xA1SKIP!\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        let xkb_root: *mut ::core::ffi::c_char =
            test_get_path(b"\0".as_ptr() as *const ::core::ffi::c_char) as *mut ::core::ffi::c_char;
        if !xkb_root.is_null() {
        } else {
            __assert_fail(
                b"xkb_root\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/extensions-directories.c\0".as_ptr() as *const ::core::ffi::c_char,
                289 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        setenv(
            b"XKB_CONFIG_ROOT\0".as_ptr() as *const ::core::ffi::c_char,
            xkb_root,
            1 as ::core::ffi::c_int,
        );
        test_layouts(xkb_root, update_output_files);
        test_options(xkb_root, update_output_files);
        free(xkb_root as *mut ::core::ffi::c_void);
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
