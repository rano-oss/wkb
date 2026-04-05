use ::c2rust_bitfields;
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
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod sys_types_h {
    pub type ssize_t = isize;
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
    use super::internal::__va_list_tag;
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
    pub type xkb_layout_index_t = uint32_t;
    pub type xkb_keycode_t = uint32_t;
    pub type xkb_mod_mask_t = uint32_t;
    pub type xkb_mod_index_t = uint32_t;
    pub type xkb_keysym_t = uint32_t;
    pub type xkb_level_index_t = uint32_t;
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
    pub type xkb_layout_mask_t = uint32_t;
    pub type xkb_led_index_t = uint32_t;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    pub type xkb_key_direction = ::core::ffi::c_uint;
    pub const XKB_KEY_REPEATED: xkb_key_direction = 2;
    pub const XKB_KEY_DOWN: xkb_key_direction = 1;
    pub const XKB_KEY_UP: xkb_key_direction = 0;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        pub type xkb_state;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_min_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;
        pub fn xkb_keymap_max_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;
        pub fn xkb_keymap_key_by_name(
            keymap: *mut xkb_keymap,
            name: *const ::core::ffi::c_char,
        ) -> xkb_keycode_t;
        pub fn xkb_keymap_num_mods(keymap: *mut xkb_keymap) -> xkb_mod_index_t;
        pub fn xkb_keymap_mod_get_index(
            keymap: *mut xkb_keymap,
            name: *const ::core::ffi::c_char,
        ) -> xkb_mod_index_t;
        pub fn xkb_keymap_mod_get_mask(
            keymap: *mut xkb_keymap,
            name: *const ::core::ffi::c_char,
        ) -> xkb_mod_mask_t;
        pub fn xkb_keymap_mod_get_mask2(
            keymap: *mut xkb_keymap,
            idx: xkb_mod_index_t,
        ) -> xkb_mod_mask_t;
        pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
        pub fn xkb_state_unref(state: *mut xkb_state);
        pub fn xkb_state_update_key(
            state: *mut xkb_state,
            key: xkb_keycode_t,
            direction: xkb_key_direction,
        ) -> xkb_state_component;
        pub fn xkb_state_update_mask(
            state: *mut xkb_state,
            depressed_mods: xkb_mod_mask_t,
            latched_mods: xkb_mod_mask_t,
            locked_mods: xkb_mod_mask_t,
            depressed_layout: xkb_layout_index_t,
            latched_layout: xkb_layout_index_t,
            locked_layout: xkb_layout_index_t,
        ) -> xkb_state_component;
        pub fn xkb_state_serialize_mods(
            state: *mut xkb_state,
            components: xkb_state_component,
        ) -> xkb_mod_mask_t;
        pub fn xkb_state_mod_index_is_active(
            state: *mut xkb_state,
            idx: xkb_mod_index_t,
            type_0: xkb_state_component,
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
    #[inline]
    pub unsafe extern "C" fn XkbKey(
        mut keymap: *mut xkb_keymap,
        mut kc: xkb_keycode_t,
    ) -> *const xkb_key {
        unsafe {
            if kc < (*keymap).min_key_code || kc > (*keymap).max_key_code {
                return ::core::ptr::null::<xkb_key>();
            } else if kc < (*keymap).num_keys_low {
                return (*keymap).keys.offset(kc as isize) as *mut xkb_key;
            } else {
                let mut lower: xkb_keycode_t = (*keymap).num_keys_low;
                let mut upper: xkb_keycode_t = (*keymap).num_keys;
                while lower < upper {
                    let mid: xkb_keycode_t = lower.wrapping_add(
                        upper
                            .wrapping_sub(1 as xkb_keycode_t)
                            .wrapping_sub(lower)
                            .wrapping_div(2 as xkb_keycode_t),
                    );
                    let key: *const xkb_key = (*keymap).keys.offset(mid as isize) as *mut xkb_key;
                    if (*key).keycode < kc {
                        lower = mid.wrapping_add(1 as xkb_keycode_t);
                    } else if (*key).keycode > kc {
                        upper = mid;
                    } else {
                        return key;
                    }
                }
                return ::core::ptr::null::<xkb_key>();
            };
        }
    }
    use super::__stddef_null_h::NULL;
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
        pub fn mod_mask_get_effective(
            keymap: *mut xkb_keymap,
            mods: xkb_mod_mask_t,
        ) -> xkb_mod_mask_t;
    }
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
    pub const EVDEV_OFFSET: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
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
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub mod evdev_scancodes_h {
    pub const KEY_LEFTCTRL: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
    pub const KEY_LEFTSHIFT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
    pub const KEY_RIGHTSHIFT: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
    pub const KEY_LEFTALT: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
    pub const KEY_CAPSLOCK: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
    pub const KEY_NUMLOCK: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
    pub const KEY_RIGHTCTRL: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
    pub const KEY_RIGHTALT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    pub const KEY_LEFTMETA: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
    pub const KEY_RIGHTMETA: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod xkbcommon_names_h {
    pub const XKB_MOD_NAME_SHIFT: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Shift\0") };
    pub const XKB_MOD_NAME_CAPS: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Lock\0") };
    pub const XKB_MOD_NAME_CTRL: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Control\0") };
    pub const XKB_MOD_NAME_MOD1: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod1\0") };
    pub const XKB_MOD_NAME_MOD2: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod2\0") };
    pub const XKB_MOD_NAME_MOD3: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod3\0") };
    pub const XKB_MOD_NAME_MOD4: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod4\0") };
    pub const XKB_MOD_NAME_MOD5: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Mod5\0") };
    pub const XKB_VMOD_NAME_ALT: [::core::ffi::c_char; 4] =
        unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Alt\0") };
    pub const XKB_VMOD_NAME_HYPER: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Hyper\0") };
    pub const XKB_VMOD_NAME_LEVEL3: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"LevelThree\0") };
    pub const XKB_VMOD_NAME_LEVEL5: [::core::ffi::c_char; 10] =
        unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"LevelFive\0") };
    pub const XKB_VMOD_NAME_META: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Meta\0") };
    pub const XKB_VMOD_NAME_NUM: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"NumLock\0") };
    pub const XKB_VMOD_NAME_SCROLL: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"ScrollLock\0") };
    pub const XKB_VMOD_NAME_SUPER: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Super\0") };
}
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::evdev_scancodes_h::{
    KEY_CAPSLOCK, KEY_LEFTALT, KEY_LEFTCTRL, KEY_LEFTMETA, KEY_LEFTSHIFT, KEY_NUMLOCK,
    KEY_RIGHTALT, KEY_RIGHTCTRL, KEY_RIGHTMETA, KEY_RIGHTSHIFT,
};
pub use self::internal::__va_list_tag;
pub use self::keymap_h::{
    mod_mask_get_effective, mod_type, real_mod_index, xkb_action, xkb_action_controls,
    xkb_action_count_t, xkb_action_flags, xkb_action_type, xkb_controls_action,
    xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action,
    xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type, xkb_key_type_entry,
    xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation, xkb_mod,
    xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t, xkb_pointer_action,
    xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, XkbKey, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X,
    ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK,
    ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE,
    ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION, ACTION_SAME_SCREEN,
    ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK,
    ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK,
    ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON,
    ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY,
    ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MOD_INDEX_CAPS,
    XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1, XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3,
    XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT, _ACTION_TYPE_NUM_ENTRIES,
    _XKB_MOD_INDEX_NUM_ENTRIES,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::stdio_h::{fprintf, stderr};
use self::string_h::strlen;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::sys_types_h::ssize_t;
pub use self::test_h::{
    key_seq_state, test_compile_buffer, test_compile_file, test_compile_rules, test_compile_string,
    test_context_flags, test_get_context, test_init, test_key_seq, BOTH,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG, DOWN, EVDEV_OFFSET, FINISH, NEXT, REPEAT, UP,
};
pub use self::types_h::{
    __int16_t, __int32_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use self::xkbcommon_h::{
    xkb_context_unref, xkb_key_direction, xkb_keycode_t, xkb_keymap_compile_flags,
    xkb_keymap_format, xkb_keymap_key_by_name, xkb_keymap_max_keycode, xkb_keymap_min_keycode,
    xkb_keymap_mod_get_index, xkb_keymap_mod_get_mask, xkb_keymap_mod_get_mask2,
    xkb_keymap_num_mods, xkb_keymap_unref, xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t,
    xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t, xkb_log_level,
    xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state, xkb_state_component,
    xkb_state_mod_index_is_active, xkb_state_new, xkb_state_serialize_mods, xkb_state_unref,
    xkb_state_update_key, xkb_state_update_mask, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_KEY_DOWN, XKB_KEY_REPEATED, XKB_KEY_UP, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
    XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
    XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED,
    XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
};
pub use self::FILE_h::FILE;
pub use self::__stddef_null_h::NULL;
pub use self::xkbcommon_names_h::{
    XKB_MOD_NAME_CAPS, XKB_MOD_NAME_CTRL, XKB_MOD_NAME_MOD1, XKB_MOD_NAME_MOD2, XKB_MOD_NAME_MOD3,
    XKB_MOD_NAME_MOD4, XKB_MOD_NAME_MOD5, XKB_MOD_NAME_SHIFT, XKB_VMOD_NAME_ALT,
    XKB_VMOD_NAME_HYPER, XKB_VMOD_NAME_LEVEL3, XKB_VMOD_NAME_LEVEL5, XKB_VMOD_NAME_META,
    XKB_VMOD_NAME_NUM, XKB_VMOD_NAME_SCROLL, XKB_VMOD_NAME_SUPER,
};
pub type real_mod_mask = ::core::ffi::c_uint;
pub const NoModifier: real_mod_mask = 0;
pub const Mod5Mask: real_mod_mask = 128;
pub const Mod4Mask: real_mod_mask = 64;
pub const Mod3Mask: real_mod_mask = 32;
pub const Mod2Mask: real_mod_mask = 16;
pub const Mod1Mask: real_mod_mask = 8;
pub const ControlMask: real_mod_mask = 4;
pub const LockMask: real_mod_mask = 2;
pub const ShiftMask: real_mod_mask = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub keymap: *const ::core::ffi::c_char,
    pub m1: mod_props,
    pub m2: mod_props,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mod_props {
    pub type_0: mod_type,
    pub mapping: xkb_mod_mask_t,
    pub mapping_effective: xkb_mod_mask_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub name: *const ::core::ffi::c_char,
    pub index: xkb_mod_index_t,
    pub mapping: xkb_mod_mask_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub path: *const ::core::ffi::c_char,
    pub formats: [xkb_keymap_format; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub mod_0: *const ::core::ffi::c_char,
    pub keycodes: [xkb_keycode_t; 10],
}
unsafe extern "C" fn test_real_mod(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
    mut idx: xkb_mod_index_t,
    mut mapping: xkb_mod_mask_t,
) -> bool {
    unsafe {
        return xkb_keymap_mod_get_index(keymap, name) == idx
            && (*keymap).mods.mods[idx as usize].type_0 as ::core::ffi::c_uint
                == MOD_REAL as ::core::ffi::c_int as ::core::ffi::c_uint
            && mapping == (*keymap).mods.mods[idx as usize].mapping
            && mapping == (1 as xkb_mod_mask_t) << idx
            && xkb_keymap_mod_get_mask(keymap, name) == mapping
            && xkb_keymap_mod_get_mask2(keymap, idx) == mapping;
    }
}
unsafe extern "C" fn test_virtual_mod(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
    mut idx: xkb_mod_index_t,
    mut mapping: xkb_mod_mask_t,
) -> bool {
    unsafe {
        return xkb_keymap_mod_get_index(keymap, name) == idx
            && (*keymap).mods.mods[idx as usize].type_0 as ::core::ffi::c_uint
                == MOD_VIRT as ::core::ffi::c_int as ::core::ffi::c_uint
            && mapping == (*keymap).mods.mods[idx as usize].mapping
            && xkb_keymap_mod_get_mask(keymap, name) == mapping
            && xkb_keymap_mod_get_mask2(keymap, idx) == mapping;
    }
}
unsafe extern "C" fn test_modifiers_names(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                64 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Shift\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_SHIFT as ::core::ffi::c_int as xkb_mod_index_t,
            ShiftMask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_SHIFT, XKB_MOD_INDEX_SHIFT, ShiftMask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Lock\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_CAPS as ::core::ffi::c_int as xkb_mod_index_t,
            LockMask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_CAPS, XKB_MOD_INDEX_CAPS, LockMask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                69 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Control\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_CTRL as ::core::ffi::c_int as xkb_mod_index_t,
            ControlMask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_CTRL, XKB_MOD_INDEX_CTRL, ControlMask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod1\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD1 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_MOD1, XKB_MOD_INDEX_MOD1, Mod1Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod2\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD2 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod2Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_MOD2, XKB_MOD_INDEX_MOD2, Mod2Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod3\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD3 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod3Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_MOD3, XKB_MOD_INDEX_MOD3, Mod3Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod4\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD4 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_MOD4, XKB_MOD_INDEX_MOD4, Mod4Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                74 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod5\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod5Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_MOD5, XKB_MOD_INDEX_MOD5, Mod5Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                75 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod1\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD1 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_ALT, XKB_MOD_INDEX_MOD1, Mod1Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod2\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD2 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod2Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_NUM, XKB_MOD_INDEX_MOD2, Mod2Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                79 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_real_mod(
            keymap,
            b"Mod4\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_MOD_INDEX_MOD4 as ::core::ffi::c_int as xkb_mod_index_t,
            Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_real_mod(keymap, XKB_MOD_NAME_LOGO, XKB_MOD_INDEX_MOD4, Mod4Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                80 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"Alt\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as xkb_mod_index_t,
            Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_ALT, XKB_MOD_INDEX_MOD5 + 2, Mod1Mask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"Meta\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
                as xkb_mod_index_t,
            Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_META, XKB_MOD_INDEX_MOD5 + 11, Mod1Mask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"NumLock\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as xkb_mod_index_t,
            Mod2Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_NUM, XKB_MOD_INDEX_MOD5 + 1, Mod2Mask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"Super\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                as xkb_mod_index_t,
            Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_SUPER, XKB_MOD_INDEX_MOD5 + 12, Mod4Mask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                87 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"Hyper\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int)
                as xkb_mod_index_t,
            Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_HYPER, XKB_MOD_INDEX_MOD5 + 13, Mod4Mask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"LevelThree\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as xkb_mod_index_t,
            Mod5Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_LEVEL3, XKB_MOD_INDEX_MOD5 + 3, Mod5Mask)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"ScrollLock\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_mod_index_t,
            0 as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_SCROLL, XKB_MOD_INDEX_MOD5 + 8, 0 )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                90 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"LevelFive\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as xkb_mod_index_t,
            0 as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, XKB_VMOD_NAME_LEVEL5, XKB_MOD_INDEX_MOD5 + 9, 0)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if (*keymap).mods.num_mods == 21 as xkb_mod_index_t {
        } else {
            __assert_fail(
                b"keymap->mods.num_mods == 21\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"LAlt\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as xkb_mod_index_t,
            0 as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, \"LAlt\", XKB_MOD_INDEX_MOD5 + 4, 0 )\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                95 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"RAlt\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as xkb_mod_index_t,
            0 as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, \"RAlt\", XKB_MOD_INDEX_MOD5 + 5, 0 )\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"LControl\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as xkb_mod_index_t,
            0 as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, \"LControl\", XKB_MOD_INDEX_MOD5 + 7, 0 )\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                97 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"RControl\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as xkb_mod_index_t,
            0 as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, \"RControl\", XKB_MOD_INDEX_MOD5 + 6, 0 )\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_virtual_mod(
            keymap,
            b"AltGr\0".as_ptr() as *const ::core::ffi::c_char,
            (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
                as xkb_mod_index_t,
            Mod5Mask as ::core::ffi::c_int as xkb_mod_mask_t,
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"test_virtual_mod(keymap, \"AltGr\", XKB_MOD_INDEX_MOD5 + 10, Mod5Mask)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                99 as ::core::ffi::c_uint,
                b"void test_modifiers_names(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_modmap_none(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut key: *const xkb_key = ::core::ptr::null::<xkb_key>();
        let mut keycode: xkb_keycode_t = 0;
        keymap = test_compile_file(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"keymaps/modmap-none.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"LVL3\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == NoModifier as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == NoModifier\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                119 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"LFSH\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == NoModifier as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == NoModifier\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                124 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"RTSH\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == NoModifier as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == NoModifier\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"LWIN\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod4Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                134 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"RWIN\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod4Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                139 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"LCTL\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                142 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == ControlMask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == ControlMask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                144 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"RCTL\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == ControlMask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == ControlMask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                149 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"LALT\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                152 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod1Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"RALT\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap
            == (Mod2Mask as ::core::ffi::c_int | Mod5Mask as ::core::ffi::c_int) as xkb_mod_mask_t
        {
        } else {
            __assert_fail(
                b"key->modmap == (Mod2Mask | Mod5Mask)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                159 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"CAPS\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == LockMask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == LockMask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD01\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                167 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod1Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                169 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD02\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                172 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == NoModifier as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == NoModifier\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD03\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == NoModifier as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == NoModifier\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD04\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                182 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod1Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD05\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod2Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod2Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD06\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod3Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod3Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                194 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD07\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                197 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod1Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                199 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD08\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                202 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod2Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod2Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                204 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        keycode = xkb_keymap_key_by_name(keymap, b"AD09\0".as_ptr() as *const ::core::ffi::c_char);
        if keycode != 0xffffffff as xkb_keycode_t {
        } else {
            __assert_fail(
                b"keycode != XKB_KEYCODE_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        key = XkbKey(keymap, keycode);
        if (*key).modmap == Mod3Mask as ::core::ffi::c_int as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"key->modmap == Mod3Mask\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"void test_modmap_none(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_explicit_virtual_modifiers(mut context: *mut xkb_context) {
    unsafe {
        let tests: [C2Rust_Unnamed_13; 5] = [
            C2Rust_Unnamed_13 {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers M1 = 0x100, M2 = 0x200;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                m1: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x100 as xkb_mod_mask_t,
                    mapping_effective: 0x100 as xkb_mod_mask_t,
                },
                m2: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x200 as xkb_mod_mask_t,
                    mapping_effective: 0x200 as xkb_mod_mask_t,
                },
            },
            C2Rust_Unnamed_13 {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers M1 = 0x100, M2 = 0x100;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                m1: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x100 as xkb_mod_mask_t,
                    mapping_effective: 0x100 as xkb_mod_mask_t,
                },
                m2: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x100 as xkb_mod_mask_t,
                    mapping_effective: 0x100 as xkb_mod_mask_t,
                },
            },
            C2Rust_Unnamed_13 {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers M1 = 0x100, M2 = 0x300;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                m1: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x100 as xkb_mod_mask_t,
                    mapping_effective: 0x100 as xkb_mod_mask_t,
                },
                m2: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x300 as xkb_mod_mask_t,
                    mapping_effective: 0x300 as xkb_mod_mask_t,
                },
            },
            C2Rust_Unnamed_13 {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers M1 = 0x200, M2 = 0x100;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                m1: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x200 as xkb_mod_mask_t,
                    mapping_effective: 0x100 as xkb_mod_mask_t,
                },
                m2: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x100 as xkb_mod_mask_t,
                    mapping_effective: 0x200 as xkb_mod_mask_t,
                },
            },
            C2Rust_Unnamed_13 {
                keymap: b"xkb_keymap {\n  xkb_compat {\n    virtual_modifiers M1 = 0x400, M2 = 0x800;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                m1: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x400 as xkb_mod_mask_t,
                    mapping_effective: 0 as xkb_mod_mask_t,
                },
                m2: mod_props {
                    type_0: MOD_VIRT,
                    mapping: 0x800 as xkb_mod_mask_t,
                    mapping_effective: 0 as xkb_mod_mask_t,
                },
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_13; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_13>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_explicit_virtual_modifiers\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut keymap: *mut xkb_keymap = test_compile_buffer(
                context,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                tests[k as usize].keymap,
                strlen(tests[k as usize].keymap),
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    331 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let m1_idx: xkb_mod_index_t =
                xkb_keymap_mod_get_index(keymap, b"M1\0".as_ptr() as *const ::core::ffi::c_char)
                    as xkb_mod_index_t;
            let m2_idx: xkb_mod_index_t =
                xkb_keymap_mod_get_index(keymap, b"M2\0".as_ptr() as *const ::core::ffi::c_char)
                    as xkb_mod_index_t;
            if m1_idx == 8 as xkb_mod_index_t {
            } else {
                __assert_fail(
                    b"m1_idx == 8\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    335 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if m2_idx == 9 as xkb_mod_index_t {
            } else {
                __assert_fail(
                    b"m2_idx == 9\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    336 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if (*keymap).mods.mods[m1_idx as usize].type_0 as ::core::ffi::c_uint
                == tests[k as usize].m1.type_0 as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"keymap->mods.mods[m1_idx].type == tests[k].m1.type\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if (*keymap).mods.mods[m2_idx as usize].type_0 as ::core::ffi::c_uint
                == tests[k as usize].m2.type_0 as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"keymap->mods.mods[m2_idx].type == tests[k].m2.type\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    338 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let m1: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << m1_idx;
            let m2: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << m2_idx;
            let m1_mapping: xkb_mod_mask_t = mod_mask_get_effective(keymap, m1) as xkb_mod_mask_t;
            let m2_mapping: xkb_mod_mask_t = mod_mask_get_effective(keymap, m2) as xkb_mod_mask_t;
            if m1_mapping == tests[k as usize].m1.mapping {
            } else {
                __assert_fail(
                    b"m1_mapping == tests[k].m1.mapping\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    344 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if m2_mapping == tests[k as usize].m2.mapping {
            } else {
                __assert_fail(
                    b"m2_mapping == tests[k].m2.mapping\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    345 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if mod_mask_get_effective(keymap, m1_mapping) == tests[k as usize].m1.mapping_effective
            {
            } else {
                __assert_fail(
                    b"mod_mask_get_effective(keymap, m1_mapping) == tests[k].m1.mapping_effective\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    348 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if mod_mask_get_effective(keymap, m2_mapping) == tests[k as usize].m2.mapping_effective
            {
            } else {
                __assert_fail(
                    b"mod_mask_get_effective(keymap, m2_mapping) == tests[k].m2.mapping_effective\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    350 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let mut state: *mut xkb_state = xkb_state_new(keymap);
            if !state.is_null() {
            } else {
                __assert_fail(
                    b"state\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    353 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let noise: xkb_mod_mask_t = 0x8000 as xkb_mod_mask_t;
            if (*keymap).canonical_state_mask & noise == 0 as xkb_mod_mask_t {
            } else {
                __assert_fail(
                    b"(keymap->canonical_state_mask & noise) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    358 as ::core::ffi::c_uint,
                    b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let set_masks: [xkb_mod_mask_t; 2] = [m1_mapping, m2_mapping];
            let mut m: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while (m as usize)
                < (::core::mem::size_of::<[xkb_mod_mask_t; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<xkb_mod_mask_t>() as usize)
            {
                let expected: xkb_mod_mask_t = set_masks[m as usize];
                xkb_state_update_mask(
                    state,
                    expected | noise,
                    0 as xkb_mod_mask_t,
                    noise,
                    0 as xkb_layout_index_t,
                    0 as xkb_layout_index_t,
                    0 as xkb_layout_index_t,
                );
                let got: xkb_mod_mask_t =
                    xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE) as xkb_mod_mask_t;
                let __cond: bool = got == expected;
                if !__cond {
                    fprintf(
                        stderr,
                        b"Assertion failure: expected %#x, got %#x\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        expected,
                        got,
                    );
                    if __cond as ::core::ffi::c_int != 0 {
                    } else {
                        __assert_fail(
                            b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                            369 as ::core::ffi::c_uint,
                            b"void test_explicit_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                if xkb_state_mod_index_is_active(state, m1_idx, XKB_STATE_MODS_EFFECTIVE)
                    == (expected & m1_mapping == m1_mapping) as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"xkb_state_mod_index_is_active(state, m1_idx, XKB_STATE_MODS_EFFECTIVE) == ((expected & m1_mapping) == m1_mapping)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                        374 as ::core::ffi::c_uint,
                        b"void test_explicit_virtual_modifiers(struct xkb_context *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if xkb_state_mod_index_is_active(state, m2_idx, XKB_STATE_MODS_EFFECTIVE)
                    == (expected & m2_mapping == m2_mapping) as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"xkb_state_mod_index_is_active(state, m2_idx, XKB_STATE_MODS_EFFECTIVE) == ((expected & m2_mapping) == m2_mapping)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                        379 as ::core::ffi::c_uint,
                        b"void test_explicit_virtual_modifiers(struct xkb_context *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                m = m.wrapping_add(1);
            }
            xkb_state_unref(state);
            xkb_keymap_unref(keymap);
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_virtual_modifiers_mapping_hack(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                403 as ::core::ffi::c_uint,
                b"void test_virtual_modifiers_mapping_hack(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut state: *mut xkb_state = xkb_state_new(keymap);
        if !state.is_null() {
        } else {
            __assert_fail(
                b"state\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                406 as ::core::ffi::c_uint,
                b"void test_virtual_modifiers_mapping_hack(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        static mut mods: [C2Rust_Unnamed_14; 21] = [
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_SHIFT.as_ptr(),
                index: XKB_MOD_INDEX_SHIFT as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_SHIFT as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_CAPS.as_ptr(),
                index: XKB_MOD_INDEX_CAPS as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_CAPS as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_CTRL.as_ptr(),
                index: XKB_MOD_INDEX_CTRL as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_CTRL as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_MOD1.as_ptr(),
                index: XKB_MOD_INDEX_MOD1 as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_MOD1 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_MOD2.as_ptr(),
                index: XKB_MOD_INDEX_MOD2 as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_MOD2 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_MOD3.as_ptr(),
                index: XKB_MOD_INDEX_MOD3 as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_MOD3 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_MOD4.as_ptr(),
                index: XKB_MOD_INDEX_MOD4 as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_MOD4 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_MOD_NAME_MOD5.as_ptr(),
                index: XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int as xkb_mod_index_t,
                mapping: (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_ALT.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_META.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod1Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_NUM.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod2Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_SUPER.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_HYPER.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod4Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_LEVEL3.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod5Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_SCROLL.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: 0 as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: XKB_VMOD_NAME_LEVEL5.as_ptr(),
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: 0 as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: b"LAlt\0".as_ptr() as *const ::core::ffi::c_char,
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: 0 as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: b"RAlt\0".as_ptr() as *const ::core::ffi::c_char,
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: 0 as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: b"LControl\0".as_ptr() as *const ::core::ffi::c_char,
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: 0 as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: b"RControl\0".as_ptr() as *const ::core::ffi::c_char,
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: 0 as xkb_mod_mask_t,
            },
            C2Rust_Unnamed_14 {
                name: b"AltGr\0".as_ptr() as *const ::core::ffi::c_char,
                index: (XKB_MOD_INDEX_MOD5 as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
                    as xkb_mod_index_t,
                mapping: Mod5Mask as ::core::ffi::c_int as xkb_mod_mask_t,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_14; 21]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_14>() as usize)
        {
            let index: xkb_mod_mask_t =
                xkb_keymap_mod_get_index(keymap, mods[k as usize].name) as xkb_mod_mask_t;
            let __cond: bool = index == mods[k as usize].index;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: %s: expected %u, got: %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mods[k as usize].name,
                    mods[k as usize].index,
                    index,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                        529 as ::core::ffi::c_uint,
                        b"void test_virtual_modifiers_mapping_hack(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << index;
            xkb_state_update_mask(
                state,
                mask,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
            );
            let mapping: xkb_mod_mask_t =
                xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE) as xkb_mod_mask_t;
            let __cond_0: bool = mapping == mods[k as usize].mapping;
            if !__cond_0 {
                fprintf(
                    stderr,
                    b"Assertion failure: %s: expected %#x, got: %#x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mods[k as usize].name,
                    mods[k as usize].mapping,
                    mapping,
                );
                if __cond_0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                        536 as ::core::ffi::c_uint,
                        b"void test_virtual_modifiers_mapping_hack(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            if mapping == xkb_keymap_mod_get_mask(keymap, mods[k as usize].name) {
            } else {
                __assert_fail(
                    b"mapping == xkb_keymap_mod_get_mask(keymap, mods[k].name)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    537 as ::core::ffi::c_uint,
                    b"void test_virtual_modifiers_mapping_hack(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if mapping == xkb_keymap_mod_get_mask2(keymap, mods[k as usize].index) {
            } else {
                __assert_fail(
                    b"mapping == xkb_keymap_mod_get_mask2(keymap, mods[k].index)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    538 as ::core::ffi::c_uint,
                    b"void test_virtual_modifiers_mapping_hack(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
        xkb_state_unref(state);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_pure_virtual_modifiers(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut keymaps: [C2Rust_Unnamed_15; 2] = [
            C2Rust_Unnamed_15 {
                path: b"keymaps/pure-virtual-mods-explicit.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                formats: [XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2],
            },
            C2Rust_Unnamed_15 {
                path: b"keymaps/pure-virtual-mods-implicit.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                formats: [XKB_KEYMAP_FORMAT_TEXT_V2, 0 as xkb_keymap_format],
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_15; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_15>() as usize)
        {
            let mut f: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while (f as usize)
                < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
            {
                if keymaps[k as usize].formats[f as usize] as u64 == 0 {
                    break;
                }
                fprintf(
                    stderr,
                    b"------\n*** %s: #%u %s (%d) ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"test_pure_virtual_modifiers\0".as_ptr() as *const ::core::ffi::c_char,
                    k,
                    keymaps[k as usize].path,
                    keymaps[k as usize].formats[f as usize] as ::core::ffi::c_uint,
                );
                keymap = test_compile_file(
                    context,
                    keymaps[k as usize].formats[f as usize],
                    keymaps[k as usize].path,
                );
                if !keymap.is_null() {
                } else {
                    __assert_fail(
                        b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                        574 as ::core::ffi::c_uint,
                        b"void test_pure_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                if test_key_seq(
                    keymap,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x77 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    30 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x61 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x61 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    30 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x61 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    48 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x62 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x62 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    48 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x62 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    46 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x63 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x63 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    46 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x63 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    32 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x64 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x64 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    32 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x64 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    18 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x65 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x65 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    18 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x65 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    33 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x66 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x66 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    33 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x66 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    34 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x67 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x67 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    34 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x67 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    35 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x68 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x68 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    35 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x68 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    23 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x69 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x69 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    23 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x69 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    36 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x6a as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x6a as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    36 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x6a as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    37 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x6b as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x6b as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    37 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x6b as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    38 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x6c as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x6c as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    38 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x6c as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    50 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x6d as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x6d as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    50 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x6d as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    49 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x6e as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x6e as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    49 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x6e as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    24 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x6f as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x6f as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    24 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x6f as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    25 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x70 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x70 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    25 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x70 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x71 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x71 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x71 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    19 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x72 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x72 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    19 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x72 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    31 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x73 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x73 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    31 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x73 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    20 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x74 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x74 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    20 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x74 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    22 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x75 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x75 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    22 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x75 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    47 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x76 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x76 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    42 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0xffe1 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x56 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    42 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0xffe1 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    47 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x76 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    30 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x61 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    31 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x73 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x31 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    100 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0xfe03 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x34 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    31 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x73 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x33 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    100 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0xfe03 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x71 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x32 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x71 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    48 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x62 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    46 as ::core::ffi::c_int,
                    DOWN as ::core::ffi::c_int,
                    0x63 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    17 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x35 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    46 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x63 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    48 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x62 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    30 as ::core::ffi::c_int,
                    UP as ::core::ffi::c_int,
                    0x61 as ::core::ffi::c_int,
                    NEXT as ::core::ffi::c_int,
                    21 as ::core::ffi::c_int,
                    BOTH as ::core::ffi::c_int,
                    0x79 as ::core::ffi::c_int,
                    FINISH as ::core::ffi::c_int,
                ) != 0
                {
                } else {
                    __assert_fail(
                        b"test_key_seq(keymap, KEY_W, BOTH, XKB_KEY_w, NEXT, KEY_A, DOWN, XKB_KEY_a, NEXT, KEY_W, BOTH, XKB_KEY_a, NEXT, KEY_A, UP, XKB_KEY_a, NEXT, KEY_B, DOWN, XKB_KEY_b, NEXT, KEY_W, BOTH, XKB_KEY_b, NEXT, KEY_B, UP, XKB_KEY_b, NEXT, KEY_C, DOWN, XKB_KEY_c, NEXT, KEY_W, BOTH, XKB_KEY_c, NEXT, KEY_C, UP, XKB_KEY_c, NEXT, KEY_D, DOWN, XKB_KEY_d, NEXT, KEY_W, BOTH, XKB_KEY_d, NEXT, KEY_D, UP, XKB_KEY_d, NEXT, KEY_E, DOWN, XKB_KEY_e, NEXT, KEY_W, BOTH, XKB_KEY_e, NEXT, KEY_E, UP, XKB_KEY_e, NEXT, KEY_F, DOWN, XKB_KEY_f, NEXT, KEY_W, BOTH, XKB_KEY_f, NEXT, KEY_F, UP, XKB_KEY_f, NEXT, KEY_G, DOWN, XKB_KEY_g, NEXT, KEY_W, BOTH, XKB_KEY_g, NEXT, KEY_G, UP, XKB_KEY_g, NEXT, KEY_H, DOWN, XKB_KEY_h, NEXT, KEY_W, BOTH, XKB_KEY_h, NEXT, KEY_H, UP, XKB_KEY_h, NEXT, KEY_I, DOWN, XKB_KEY_i, NEXT, KEY_W, BOTH, XKB_KEY_i, NEXT, KEY_I, UP, XKB_KEY_i, NEXT, KEY_J, DOWN, XKB_KEY_j, NEXT, KEY_W, BOTH, XKB_KEY_j, NEXT, KEY_J, UP, XKB_KEY_j, NEXT, KEY_K, DOWN, XKB_KEY_k, NEXT, KEY_W, BOTH, XKB_KEY_k, NEXT, KEY_K, UP, XKB_KEY_k, NEXT, KEY_L, DOWN, XKB_KEY_l, NEXT, KEY_W, BOTH, XKB_KEY_l, NEXT, KEY_L, UP, XKB_KEY_l, NEXT, KEY_M, DOWN, XKB_KEY_m, NEXT, KEY_W, BOTH, XKB_KEY_m, NEXT, KEY_M, UP, XKB_KEY_m, NEXT, KEY_N, DOWN, XKB_KEY_n, NEXT, KEY_W, BOTH, XKB_KEY_n, NEXT, KEY_N, UP, XKB_KEY_n, NEXT, KEY_O, DOWN, XKB_KEY_o, NEXT, KEY_W, BOTH, XKB_KEY_o, NEXT, KEY_O, UP, XKB_KEY_o, NEXT, KEY_P, DOWN, XKB_KEY_p, NEXT, KEY_W, BOTH, XKB_KEY_p, NEXT, KEY_P, UP, XKB_KEY_p, NEXT, KEY_Q, DOWN, XKB_KEY_q, NEXT, KEY_W, BOTH, XKB_KEY_q, NEXT, KEY_Q, UP, XKB_KEY_q, NEXT, KEY_R, DOWN, XKB_KEY_r, NEXT, KEY_W, BOTH, XKB_KEY_r, NEXT, KEY_R, UP, XKB_KEY_r, NEXT, KEY_S, DOWN, XKB_KEY_s, NEXT, KEY_W, BOTH, XKB_KEY_s, NEXT, KEY_S, UP, XKB_KEY_s, NEXT, KEY_T, DOWN, XKB_KEY_t, NEXT, KEY_W, BOTH, XKB_KEY_t, NEXT, KEY_T, UP, XKB_KEY_t, NEXT, KEY_U, DOWN, XKB_KEY_u, NEXT, KEY_W, BOTH, XKB_KEY_u, NEXT, KEY_U, UP, XKB_KEY_u, NEXT, KEY_V, DOWN, XKB_KEY_v, NEXT, KEY_W, BOTH, XKB_KEY_v, NEXT, KEY_LEFTSHIFT, DOWN, XKB_KEY_Shift_L, NEXT, KEY_W, BOTH, XKB_KEY_V, NEXT, KEY_LEFTSHIFT, UP, XKB_KEY_Shift_L, NEXT, KEY_V, UP, XKB_KEY_v, NEXT, KEY_A, DOWN, XKB_KEY_a, NEXT, KEY_S, DOWN, XKB_KEY_s, NEXT, KEY_W, BOTH, XKB_KEY_1, NEXT, KEY_RIGHTALT, DOWN, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_W, BOTH, XKB_KEY_4, NEXT, KEY_S, UP, XKB_KEY_s, NEXT, KEY_W, BOTH, XKB_KEY_3, NEXT, KEY_RIGHTALT, UP, XKB_KEY_ISO_Level3_Shift, NEXT, KEY_Q, DOWN, XKB_KEY_q, NEXT, KEY_W, BOTH, XKB_KEY_2, NEXT, KEY_Q, UP, XKB_KEY_q, NEXT, KEY_B, DOWN, XKB_KEY_b, NEXT, KEY_C, DOWN, XKB_KEY_c, NEXT, KEY_W, BOTH, XKB_KEY_5, NEXT, KEY_C, UP, XKB_KEY_c, NEXT, KEY_B, UP, XKB_KEY_b, NEXT, KEY_A, UP, XKB_KEY_a, NEXT, KEY_Y, BOTH, XKB_KEY_y, FINISH)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                        664 as ::core::ffi::c_uint,
                        b"void test_pure_virtual_modifiers(struct xkb_context *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                xkb_keymap_unref(keymap);
                f = f.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
        let keymap_str: [::core::ffi::c_char; 220] = ::core::mem::transmute::<
            [u8; 220],
            [::core::ffi::c_char; 220],
        >(
            *b"xkb_keymap {  xkb_keycodes { include \"evdev\" };  xkb_types { include \"complete\" };  xkb_compat { include \"complete+basic(invalid-pure-virtual-modifiers)\" };  xkb_symbols { include \"pc(pc105-pure-virtual-modifiers)\" };};\0",
        );
        keymap = test_compile_string(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const keymap_str as *const ::core::ffi::c_char,
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                677 as ::core::ffi::c_uint,
                b"void test_pure_virtual_modifiers(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn xkb_keymap_mod_get_codes(
    mut keymap: *mut xkb_keymap,
    mut mod_0: xkb_mod_index_t,
    mut codes_out: *mut xkb_keycode_t,
    mut codes_size: size_t,
) -> ssize_t {
    unsafe {
        if mod_0 >= xkb_keymap_num_mods(keymap) {
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        let mut idx: ssize_t = 0 as ssize_t;
        let mut k: xkb_keycode_t = xkb_keymap_min_keycode(keymap);
        while k <= xkb_keymap_max_keycode(keymap) && idx >= 0 as ssize_t {
            let state: *mut xkb_state = xkb_state_new(keymap) as *mut xkb_state;
            if !state.is_null() {
            } else {
                __assert_fail(
                    b"state\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    711 as ::core::ffi::c_uint,
                    b"ssize_t xkb_keymap_mod_get_codes(struct xkb_keymap *, xkb_mod_index_t, xkb_keycode_t *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            static mut directions: [xkb_key_direction; 2] = [XKB_KEY_DOWN, XKB_KEY_UP];
            let mut d: size_t = 0 as size_t;
            while d
                < (::core::mem::size_of::<[xkb_key_direction; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<xkb_key_direction>() as usize)
            {
                if xkb_state_update_key(state, k, directions[d as usize]) as ::core::ffi::c_uint
                    != 0
                    && xkb_state_mod_index_is_active(state, mod_0, XKB_STATE_MODS_EFFECTIVE) != 0
                {
                    if (idx as size_t) < codes_size {
                        if idx
                            < (18446744073709551615 as ::core::ffi::c_ulong)
                                .wrapping_div(2 as ::core::ffi::c_ulong)
                                as ssize_t
                        {
                        } else {
                            __assert_fail(
                                b"idx < (ssize_t) (SIZE_MAX / 2)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"../test/modifiers.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                723 as ::core::ffi::c_uint,
                                b"ssize_t xkb_keymap_mod_get_codes(struct xkb_keymap *, xkb_mod_index_t, xkb_keycode_t *, size_t)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        };
                        let c2rust_fresh0 = idx;
                        idx = idx + 1;
                        *codes_out.offset(c2rust_fresh0 as isize) = k;
                    } else {
                        idx = -2 as ::core::ffi::c_int as ssize_t;
                    }
                    break;
                } else {
                    d = d.wrapping_add(1);
                }
            }
            xkb_state_unref(state);
            k = k.wrapping_add(1);
        }
        return idx;
    }
}
unsafe extern "C" fn test_get_modifier_keycodes(mut context: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"cz\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                745 as ::core::ffi::c_uint,
                b"void test_get_modifier_keycodes(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mods: [C2Rust_Unnamed_16; 15] = [
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_SHIFT.as_ptr(),
                keycodes: [
                    KEY_LEFTSHIFT as xkb_keycode_t,
                    KEY_RIGHTSHIFT as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_CAPS.as_ptr(),
                keycodes: [
                    KEY_CAPSLOCK as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_CTRL.as_ptr(),
                keycodes: [
                    KEY_LEFTCTRL as xkb_keycode_t,
                    KEY_RIGHTCTRL as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_MOD1.as_ptr(),
                keycodes: [
                    KEY_LEFTALT as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_MOD2.as_ptr(),
                keycodes: [
                    KEY_NUMLOCK as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_MOD3.as_ptr(),
                keycodes: [
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
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_MOD4.as_ptr(),
                keycodes: [
                    KEY_LEFTMETA as xkb_keycode_t,
                    KEY_RIGHTMETA as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_MOD_NAME_MOD5.as_ptr(),
                keycodes: [
                    KEY_LVL3 as xkb_keycode_t,
                    KEY_RIGHTALT as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_ALT.as_ptr(),
                keycodes: [
                    KEY_LEFTALT as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_META.as_ptr(),
                keycodes: [
                    KEY_LEFTALT as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_SUPER.as_ptr(),
                keycodes: [
                    KEY_LEFTMETA as xkb_keycode_t,
                    KEY_RIGHTMETA as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_HYPER.as_ptr(),
                keycodes: [
                    KEY_LEFTMETA as xkb_keycode_t,
                    KEY_RIGHTMETA as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_NUM.as_ptr(),
                keycodes: [
                    KEY_NUMLOCK as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_LEVEL3.as_ptr(),
                keycodes: [
                    KEY_LVL3 as xkb_keycode_t,
                    KEY_RIGHTALT as xkb_keycode_t,
                    0 as ::core::ffi::c_int as xkb_keycode_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            C2Rust_Unnamed_16 {
                mod_0: XKB_VMOD_NAME_LEVEL5.as_ptr(),
                keycodes: [
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
                ],
            },
        ];
        let mut got: [xkb_keycode_t; 10] = [
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
        ];
        let mut m: size_t = 0 as size_t;
        while m
            < (::core::mem::size_of::<[C2Rust_Unnamed_16; 15]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_16>() as usize)
        {
            let mod_0: xkb_mod_index_t =
                xkb_keymap_mod_get_index(keymap, mods[m as usize].mod_0) as xkb_mod_index_t;
            if mod_0 != 0xffffffff as xkb_mod_index_t {
            } else {
                __assert_fail(
                    b"mod != XKB_MOD_INVALID\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    773 as ::core::ffi::c_uint,
                    b"void test_get_modifier_keycodes(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let count: ssize_t = xkb_keymap_mod_get_codes(
                keymap,
                mod_0,
                &raw mut got as *mut xkb_keycode_t,
                (::core::mem::size_of::<[xkb_keycode_t; 10]>() as size_t)
                    .wrapping_div(::core::mem::size_of::<xkb_keycode_t>() as size_t),
            ) as ssize_t;
            if count >= 0 as ssize_t {
            } else {
                __assert_fail(
                    b"count >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                    776 as ::core::ffi::c_uint,
                    b"void test_get_modifier_keycodes(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let expected: *const xkb_keycode_t =
                &raw const (*(&raw const mods as *const C2Rust_Unnamed_16).offset(m as isize))
                    .keycodes as *const xkb_keycode_t;
            let mut k: size_t = 0 as size_t;
            while k
                < (::core::mem::size_of::<[xkb_keycode_t; 10]>() as usize)
                    .wrapping_div(::core::mem::size_of::<xkb_keycode_t>() as usize)
            {
                if *expected.offset(k as isize) == 0 as xkb_keycode_t {
                    let __cond: bool = k == count as size_t;
                    if !__cond {
                        fprintf(
                            stderr,
                            b"Assertion failure: Mod %s: expected %zu, got %zd\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            mods[m as usize].mod_0,
                            k,
                            count,
                        );
                        if __cond as ::core::ffi::c_int != 0 {
                        } else {
                            __assert_fail(
                                b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                                782 as ::core::ffi::c_uint,
                                b"void test_get_modifier_keycodes(struct xkb_context *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        };
                    }
                    break;
                } else {
                    let __cond_0: bool = k < count as size_t;
                    if !__cond_0 {
                        fprintf(
                            stderr,
                            b"Assertion failure: Mod %s: Missing keycode 0x%u\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            mods[m as usize].mod_0,
                            *expected.offset(k as isize),
                        );
                        if __cond_0 as ::core::ffi::c_int != 0 {
                        } else {
                            __assert_fail(
                                b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                                787 as ::core::ffi::c_uint,
                                b"void test_get_modifier_keycodes(struct xkb_context *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        };
                    }
                    let __cond_1: bool = got[k as usize]
                        == (*expected.offset(k as isize)).wrapping_add(8 as xkb_keycode_t);
                    if !__cond_1 {
                        fprintf(
                            stderr,
                            b"Assertion failure: Mod %s: Expected keycode %u but got: %u\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            mods[m as usize].mod_0,
                            (*expected.offset(k as isize)).wrapping_add(8 as xkb_keycode_t),
                            got[k as usize],
                        );
                        if __cond_1 as ::core::ffi::c_int != 0 {
                        } else {
                            __assert_fail(
                                b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                                790 as ::core::ffi::c_uint,
                                b"void test_get_modifier_keycodes(struct xkb_context *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        };
                    }
                    k = k.wrapping_add(1);
                }
            }
            m = m.wrapping_add(1);
        }
        xkb_keymap_unref(keymap);
    }
}
pub const KEY_LVL3: ::core::ffi::c_int = 92 as ::core::ffi::c_int - EVDEV_OFFSET;
unsafe fn main_0() -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut context: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !context.is_null() {
        } else {
            __assert_fail(
                b"context\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/modifiers.c\0".as_ptr() as *const ::core::ffi::c_char,
                804 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        test_modmap_none(context);
        test_modifiers_names(context);
        test_explicit_virtual_modifiers(context);
        test_virtual_modifiers_mapping_hack(context);
        test_pure_virtual_modifiers(context);
        test_get_modifier_keycodes(context);
        xkb_context_unref(context);
        return 0 as ::core::ffi::c_int;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
