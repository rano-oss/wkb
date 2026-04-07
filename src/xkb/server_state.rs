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
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
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
    pub type xkb_led_mask_t = uint32_t;
    pub type xkb_event_type = ::core::ffi::c_uint;
    pub const XKB_EVENT_TYPE_COMPONENTS_CHANGE: xkb_event_type = 4;
    pub const XKB_EVENT_TYPE_KEY_UP: xkb_event_type = 3;
    pub const XKB_EVENT_TYPE_KEY_REPEATED: xkb_event_type = 2;
    pub const XKB_EVENT_TYPE_KEY_DOWN: xkb_event_type = 1;
    pub type xkb_keyboard_control_flags = ::core::ffi::c_uint;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY8: xkb_keyboard_control_flags = 256;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY7: xkb_keyboard_control_flags = 128;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY6: xkb_keyboard_control_flags = 64;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY5: xkb_keyboard_control_flags = 32;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY4: xkb_keyboard_control_flags = 16;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY3: xkb_keyboard_control_flags = 8;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY2: xkb_keyboard_control_flags = 4;
    pub const XKB_KEYBOARD_CONTROL_OVERLAY1: xkb_keyboard_control_flags = 2;
    pub const XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS: xkb_keyboard_control_flags = 1;
    pub const XKB_KEYBOARD_CONTROL_NO_FLAGS: xkb_keyboard_control_flags = 0;
    pub type xkb_events_flags = ::core::ffi::c_uint;
    pub const XKB_EVENTS_NO_FLAGS: xkb_events_flags = 0;
    pub type xkb_a11y_flags = ::core::ffi::c_uint;
    pub const XKB_A11Y_LATCH_SIMULTANEOUS_KEYS: xkb_a11y_flags = 2;
    pub const XKB_A11Y_LATCH_TO_LOCK: xkb_a11y_flags = 1;
    pub const XKB_A11Y_NO_FLAGS: xkb_a11y_flags = 0;
    pub type xkb_key_direction = ::core::ffi::c_uint;
    pub const XKB_KEY_REPEATED: xkb_key_direction = 2;
    pub const XKB_KEY_DOWN: xkb_key_direction = 1;
    pub const XKB_KEY_UP: xkb_key_direction = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_state_components_update {
        pub size: size_t,
        pub components: xkb_state_component,
        pub affect_latched_mods: xkb_mod_mask_t,
        pub latched_mods: xkb_mod_mask_t,
        pub affect_locked_mods: xkb_mod_mask_t,
        pub locked_mods: xkb_mod_mask_t,
        pub latched_layout: int32_t,
        pub locked_layout: int32_t,
        pub affect_controls: xkb_keyboard_control_flags,
        pub controls: xkb_keyboard_control_flags,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_layout_policy_update {
        pub size: size_t,
        pub policy: xkb_layout_out_of_range_policy,
        pub redirect: xkb_layout_index_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_state_update {
        pub size: size_t,
        pub components: *const xkb_state_components_update,
        pub layout_policy: *const xkb_layout_policy_update,
    }
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::state_priv_h::xkb_event;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::xkbcommon_errors_h::xkb_error_code;
    extern "C" {
        pub type xkb_machine;
        pub type xkb_state;
        pub type xkb_events;
        pub type xkb_machine_options;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_keymap_new_from_names(
            context: *mut xkb_context,
            names: *const xkb_rule_names,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_mod_get_mask(
            keymap: *mut xkb_keymap,
            name: *const ::core::ffi::c_char,
        ) -> xkb_mod_mask_t;
        pub fn xkb_keymap_mod_get_mask2(
            keymap: *mut xkb_keymap,
            idx: xkb_mod_index_t,
        ) -> xkb_mod_mask_t;
        pub fn xkb_keymap_num_layouts(keymap: *mut xkb_keymap) -> xkb_layout_index_t;
        pub fn xkb_keymap_led_get_index(
            keymap: *mut xkb_keymap,
            name: *const ::core::ffi::c_char,
        ) -> xkb_led_index_t;
        pub fn xkb_keymap_key_repeats(
            keymap: *mut xkb_keymap,
            key: xkb_keycode_t,
        ) -> ::core::ffi::c_int;
        pub fn xkb_event_get_type(event: *const xkb_event) -> xkb_event_type;
        pub fn xkb_event_get_keycode(event: *const xkb_event) -> xkb_keycode_t;
        pub fn xkb_events_new_batch(
            context: *mut xkb_context,
            flags: xkb_events_flags,
        ) -> *mut xkb_events;
        pub fn xkb_events_destroy(events: *mut xkb_events);
        pub fn xkb_events_next(events: *mut xkb_events) -> *const xkb_event;
        pub fn xkb_machine_options_new(context: *mut xkb_context) -> *mut xkb_machine_options;
        pub fn xkb_machine_options_destroy(options: *mut xkb_machine_options);
        pub fn xkb_machine_options_update_a11y_flags(
            options: *mut xkb_machine_options,
            affect: xkb_a11y_flags,
            flags: xkb_a11y_flags,
        ) -> xkb_error_code;
        pub fn xkb_machine_options_remap_mods(
            options: *mut xkb_machine_options,
            source: xkb_mod_mask_t,
            target: xkb_mod_mask_t,
        ) -> xkb_error_code;
        pub fn xkb_machine_options_update_shortcut_mods(
            options: *mut xkb_machine_options,
            affect: xkb_mod_mask_t,
            mask: xkb_mod_mask_t,
        ) -> xkb_error_code;
        pub fn xkb_machine_options_remap_shortcut_layout(
            options: *mut xkb_machine_options,
            source: xkb_layout_index_t,
            target: xkb_layout_index_t,
        ) -> xkb_error_code;
        pub fn xkb_machine_new(
            keymap: *mut xkb_keymap,
            options: *const xkb_machine_options,
        ) -> *mut xkb_machine;
        pub fn xkb_machine_unref(machine: *mut xkb_machine);
        pub fn xkb_machine_process_key(
            machine: *mut xkb_machine,
            key: xkb_keycode_t,
            direction: xkb_key_direction,
            events: *mut xkb_events,
        ) -> xkb_error_code;
        pub fn xkb_machine_process_synthetic(
            machine: *mut xkb_machine,
            update: *const xkb_state_update,
            events: *mut xkb_events,
        ) -> xkb_error_code;
        pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
        pub fn xkb_state_unref(state: *mut xkb_state);
        pub fn xkb_state_update_key(
            state: *mut xkb_state,
            key: xkb_keycode_t,
            direction: xkb_key_direction,
        ) -> xkb_state_component;
        pub fn xkb_state_update_synthetic(
            state: *mut xkb_state,
            update: *const xkb_state_update,
            changed: *mut xkb_state_component,
        ) -> xkb_error_code;
        pub fn xkb_state_update_event(
            state: *mut xkb_state,
            event: *const xkb_event,
        ) -> xkb_state_component;
        pub fn xkb_state_serialize_enabled_controls(
            state: *const xkb_state,
            components: xkb_state_component,
        ) -> xkb_keyboard_control_flags;
        pub fn xkb_state_serialize_mods(
            state: *mut xkb_state,
            components: xkb_state_component,
        ) -> xkb_mod_mask_t;
        pub fn xkb_state_serialize_layout(
            state: *mut xkb_state,
            components: xkb_state_component,
        ) -> xkb_layout_index_t;
        pub fn xkb_state_led_name_is_active(
            state: *mut xkb_state,
            name: *const ::core::ffi::c_char,
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
    pub type C2Rust_Unnamed_15 = ::core::ffi::c_uint;
    pub const _LAST_XKB_EVENT_TYPE: C2Rust_Unnamed_15 = 4;
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
pub mod state_priv_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_event {
        pub type_0: xkb_event_type,
        pub c2rust_unnamed: C2Rust_Unnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_13 {
        pub keycode: xkb_keycode_t,
        pub components: C2Rust_Unnamed_14,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_14 {
        pub components: state_components,
        pub changed: xkb_state_component,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct state_components {
        pub base_group: int32_t,
        pub latched_group: int32_t,
        pub locked_group: int32_t,
        pub group: xkb_layout_index_t,
        pub base_mods: xkb_mod_mask_t,
        pub latched_mods: xkb_mod_mask_t,
        pub locked_mods: xkb_mod_mask_t,
        pub mods: xkb_mod_mask_t,
        pub leds: xkb_led_mask_t,
        pub controls: xkb_action_controls,
    }
    use super::keymap_h::xkb_action_controls;
    use super::stdint_intn_h::int32_t;
    use super::xkbcommon_h::{
        xkb_event_type, xkb_keycode_t, xkb_layout_index_t, xkb_led_mask_t, xkb_machine,
        xkb_mod_mask_t, xkb_state, xkb_state_component,
    };
    extern "C" {
        pub fn xkb_machine_get_state(machine: *mut xkb_machine) -> *mut xkb_state;
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
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::xkbcommon_h::{xkb_events, xkb_keymap_format, xkb_machine};
    extern "C" {
        pub fn test_init();
        pub fn test_key_seq2(
            keymap: *mut xkb_keymap,
            sm: *mut xkb_machine,
            events: *mut xkb_events,
            ...
        ) -> ::core::ffi::c_int;
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_file(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            path_rel: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
        pub fn test_compile_buffer(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            buf: *const ::core::ffi::c_char,
            len: size_t,
        ) -> *mut xkb_keymap;
        pub fn test_compile_rmlvo(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            rules: *const ::core::ffi::c_char,
            model: *const ::core::ffi::c_char,
            layout: *const ::core::ffi::c_char,
            variant: *const ::core::ffi::c_char,
            options: *const ::core::ffi::c_char,
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
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
    }
}
pub mod state_h {
    pub const XKB_EVENT_TYPE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub const EVDEV_OFFSET: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    #[inline]
    pub unsafe extern "C" fn _xkb_keymap_mod_get_mask(
        mut keymap: *mut xkb_keymap,
        mut name: *const ::core::ffi::c_char,
    ) -> xkb_mod_mask_t {
        unsafe {
            let mut mask: xkb_mod_mask_t = xkb_keymap_mod_get_mask(keymap, name);
            let __cond: bool = mask != 0;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: Modifier %s is not mapped or defined\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    name,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/state.h\0".as_ptr() as *const ::core::ffi::c_char,
                        32 as ::core::ffi::c_uint,
                        b"xkb_mod_mask_t _xkb_keymap_mod_get_mask(struct xkb_keymap *, const char *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            }
            return mask;
        }
    }
    #[inline]
    pub unsafe extern "C" fn xkb_event_eq(
        mut event1: *const xkb_event,
        mut event2: *const xkb_event,
    ) -> bool {
        unsafe {
            if (*event1).type_0 as ::core::ffi::c_uint != (*event2).type_0 as ::core::ffi::c_uint {
                return false_0 != 0;
            }
            match (*event1).type_0 as ::core::ffi::c_uint {
                1 | 2 | 3 => {
                    return (*event1).c2rust_unnamed.keycode == (*event2).c2rust_unnamed.keycode;
                }
                4 => {
                    return memcmp(
                        &raw const (*event1).c2rust_unnamed.components
                            as *const ::core::ffi::c_void,
                        &raw const (*event2).c2rust_unnamed.components
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<C2Rust_Unnamed_14>() as size_t,
                    ) == 0 as ::core::ffi::c_int;
                }
                _ => return false_0 != 0,
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn print_event(
        mut prefix: *const ::core::ffi::c_char,
        mut event: *const xkb_event,
    ) {
        unsafe {
            fprintf(
                stderr,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                prefix,
            );
            match (*event).type_0 as ::core::ffi::c_uint {
                1 | 2 | 3 => {
                    fprintf(
                        stderr,
                        b"type: key %s; keycode: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                        if (*event).type_0 as ::core::ffi::c_uint
                            == XKB_EVENT_TYPE_KEY_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"up\0".as_ptr() as *const ::core::ffi::c_char
                        } else if (*event).type_0 as ::core::ffi::c_uint
                            == XKB_EVENT_TYPE_KEY_REPEATED as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                        {
                            b"repeat\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"down\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        (*event).c2rust_unnamed.keycode,
                    );
                }
                4 => {
                    fprintf(
                        stderr,
                        b"type: components; changed: 0x%x\n\tgroup: %d %d %d %u\n\tmods: 0x%08x 0x%08x 0x%08x %08x\n\tleds: 0x%08x\n\tcontrols: 0x%08x\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*event).c2rust_unnamed.components.changed
                            as ::core::ffi::c_uint,
                        (*event).c2rust_unnamed.components.components.base_group,
                        (*event).c2rust_unnamed.components.components.latched_group,
                        (*event).c2rust_unnamed.components.components.locked_group,
                        (*event).c2rust_unnamed.components.components.group,
                        (*event).c2rust_unnamed.components.components.base_mods,
                        (*event).c2rust_unnamed.components.components.latched_mods,
                        (*event).c2rust_unnamed.components.components.locked_mods,
                        (*event).c2rust_unnamed.components.components.mods,
                        (*event).c2rust_unnamed.components.components.leds,
                        (*event).c2rust_unnamed.components.components.controls
                            as ::core::ffi::c_uint,
                    );
                }
                _ => {}
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn check_events(
        mut iter: *mut xkb_events,
        mut events: *const xkb_event,
        mut count: size_t,
    ) -> bool {
        unsafe {
            let mut got: *const xkb_event = ::core::ptr::null::<xkb_event>();
            let mut got_count: size_t = 0 as size_t;
            let mut ok: bool = true_0 != 0;
            if count == 1 as size_t
                && (*events.offset(0 as ::core::ffi::c_int as isize)).type_0 as ::core::ffi::c_uint
                    == XKB_EVENT_TYPE_NONE as ::core::ffi::c_uint
            {
                count = 0 as size_t;
            }
            loop {
                got = xkb_events_next(iter);
                if got.is_null() {
                    break;
                }
                got_count = got_count.wrapping_add(1);
                if got_count > count {
                    fprintf(
                        stderr,
                        b"%s() error at event #%zu:\n\0".as_ptr() as *const ::core::ffi::c_char,
                        b"check_events\0".as_ptr() as *const ::core::ffi::c_char,
                        got_count,
                    );
                    print_event(
                        b"Unexpected event:\n\0".as_ptr() as *const ::core::ffi::c_char,
                        got,
                    );
                    break;
                } else {
                    let mut expected: *const xkb_event = events
                        .offset(got_count.wrapping_sub(1 as size_t) as isize)
                        as *const xkb_event;
                    if xkb_event_eq(got, expected) {
                        continue;
                    }
                    fprintf(
                        stderr,
                        b"%s() error at event #%zu:\n\0".as_ptr() as *const ::core::ffi::c_char,
                        b"check_events\0".as_ptr() as *const ::core::ffi::c_char,
                        got_count,
                    );
                    print_event(
                        b"Expected: \0".as_ptr() as *const ::core::ffi::c_char,
                        expected,
                    );
                    print_event(b"Got: \0".as_ptr() as *const ::core::ffi::c_char, got);
                    ok = false_0 != 0;
                    break;
                }
            }
            if got_count != count {
                fprintf(
                    stderr,
                    b"%s() count error: expected %zu, got: %zu\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"check_events\0".as_ptr() as *const ::core::ffi::c_char,
                    count,
                    got_count,
                );
                ok = false_0 != 0;
            }
            return ok;
        }
    }

    use super::__stddef_size_t_h::size_t;
    use super::assert_h::__assert_fail;
    use super::keymap_h::xkb_keymap;
    use super::state_priv_h::{xkb_event, C2Rust_Unnamed_14};
    use super::stdbool_h::{false_0, true_0};
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::memcmp;
    use super::xkbcommon_h::{
        xkb_events, xkb_events_next, xkb_keymap_mod_get_mask, xkb_mod_mask_t,
        XKB_EVENT_TYPE_KEY_REPEATED, XKB_EVENT_TYPE_KEY_UP,
    };
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
pub mod stdlib_h {
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod evdev_scancodes_h {
    pub const KEY_Q: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    pub const KEY_LEFTCTRL: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
    pub const KEY_A: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    pub const KEY_S: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
    pub const KEY_D: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    pub const KEY_J: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    pub const KEY_LEFTSHIFT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
    pub const KEY_CAPSLOCK: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
    pub const KEY_F1: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
    pub const KEY_F2: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
    pub const KEY_F10: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
    pub const KEY_KP1: ::core::ffi::c_int = 79 as ::core::ffi::c_int;
    pub const KEY_LEFT: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
    pub const KEY_LEFTMETA: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
    pub const KEY_COMPOSE: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod xkbcommon_names_h {
    pub const XKB_MOD_NAME_SHIFT: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Shift\0") };
    pub const XKB_MOD_NAME_CTRL: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Control\0") };
    pub const XKB_VMOD_NAME_ALT: [::core::ffi::c_char; 4] =
        unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Alt\0") };
    pub const XKB_VMOD_NAME_LEVEL3: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"LevelThree\0") };
    pub const XKB_VMOD_NAME_LEVEL5: [::core::ffi::c_char; 10] =
        unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"LevelFive\0") };
    pub const XKB_VMOD_NAME_NUM: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"NumLock\0") };
    pub const XKB_VMOD_NAME_SCROLL: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"ScrollLock\0") };
    pub const XKB_VMOD_NAME_SUPER: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Super\0") };
    pub const XKB_LED_NAME_NUM: [::core::ffi::c_char; 9] =
        unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Num Lock\0") };
    pub const XKB_LED_NAME_SCROLL: [::core::ffi::c_char; 12] =
        unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Scroll Lock\0") };
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::evdev_scancodes_h::{
    KEY_A, KEY_CAPSLOCK, KEY_COMPOSE, KEY_D, KEY_F1, KEY_F10, KEY_F2, KEY_J, KEY_KP1, KEY_LEFT,
    KEY_LEFTCTRL, KEY_LEFTMETA, KEY_LEFTSHIFT, KEY_Q, KEY_S,
};
pub use self::internal::__va_list_tag;
pub use self::keymap_h::{
    mod_type, real_mod_index, xkb_action, xkb_action_controls, xkb_action_count_t,
    xkb_action_flags, xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group,
    xkb_group_action, xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias,
    xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level,
    xkb_match_operation, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_15, C2Rust_Unnamed_2,
    C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7,
    C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, _ACTION_TYPE_NUM_ENTRIES,
    _LAST_XKB_EVENT_TYPE, _XKB_MOD_INDEX_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X,
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
    XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT,
};
pub use self::state_h::{
    _xkb_keymap_mod_get_mask, check_events, print_event, xkb_event_eq, EVDEV_OFFSET,
    XKB_EVENT_TYPE_NONE,
};
pub use self::state_priv_h::{
    state_components, xkb_event, xkb_machine_get_state, C2Rust_Unnamed_13, C2Rust_Unnamed_14,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdio_h::{fprintf, stderr};
pub use self::stdlib_h::EXIT_SUCCESS;
use self::string_h::memcpy;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    key_seq_state, test_compile_buffer, test_compile_file, test_compile_rmlvo, test_compile_rules,
    test_context_flags, test_get_context, test_init, test_key_seq2, BOTH,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG, DOWN, FINISH, NEXT, REPEAT, UP,
};
pub use self::types_h::{
    __int16_t, __int32_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_a11y_flags, xkb_context_unref, xkb_event_get_keycode, xkb_event_get_type, xkb_event_type,
    xkb_events, xkb_events_destroy, xkb_events_flags, xkb_events_new_batch, xkb_events_next,
    xkb_key_direction, xkb_keyboard_control_flags, xkb_keycode_t, xkb_keymap_compile_flags,
    xkb_keymap_format, xkb_keymap_key_repeats, xkb_keymap_led_get_index, xkb_keymap_mod_get_mask,
    xkb_keymap_mod_get_mask2, xkb_keymap_new_from_names, xkb_keymap_num_layouts, xkb_keymap_unref,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_layout_policy_update, xkb_led_index_t, xkb_led_mask_t, xkb_level_index_t, xkb_log_level,
    xkb_machine, xkb_machine_new, xkb_machine_options, xkb_machine_options_destroy,
    xkb_machine_options_new, xkb_machine_options_remap_mods,
    xkb_machine_options_remap_shortcut_layout, xkb_machine_options_update_a11y_flags,
    xkb_machine_options_update_shortcut_mods, xkb_machine_process_key,
    xkb_machine_process_synthetic, xkb_machine_unref, xkb_mod_index_t, xkb_mod_mask_t,
    xkb_rule_names, xkb_state, xkb_state_component, xkb_state_components_update,
    xkb_state_led_name_is_active, xkb_state_new, xkb_state_serialize_enabled_controls,
    xkb_state_serialize_layout, xkb_state_serialize_mods, xkb_state_unref, xkb_state_update,
    xkb_state_update_event, xkb_state_update_key, xkb_state_update_synthetic,
    XKB_A11Y_LATCH_SIMULTANEOUS_KEYS, XKB_A11Y_LATCH_TO_LOCK, XKB_A11Y_NO_FLAGS,
    XKB_EVENTS_NO_FLAGS, XKB_EVENT_TYPE_COMPONENTS_CHANGE, XKB_EVENT_TYPE_KEY_DOWN,
    XKB_EVENT_TYPE_KEY_REPEATED, XKB_EVENT_TYPE_KEY_UP, XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
    XKB_KEYBOARD_CONTROL_NO_FLAGS, XKB_KEYBOARD_CONTROL_OVERLAY1, XKB_KEYBOARD_CONTROL_OVERLAY2,
    XKB_KEYBOARD_CONTROL_OVERLAY3, XKB_KEYBOARD_CONTROL_OVERLAY4, XKB_KEYBOARD_CONTROL_OVERLAY5,
    XKB_KEYBOARD_CONTROL_OVERLAY6, XKB_KEYBOARD_CONTROL_OVERLAY7, XKB_KEYBOARD_CONTROL_OVERLAY8,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEY_DOWN, XKB_KEY_REPEATED, XKB_KEY_UP,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_names_h::{
    XKB_LED_NAME_NUM, XKB_LED_NAME_SCROLL, XKB_MOD_NAME_CTRL, XKB_MOD_NAME_SHIFT,
    XKB_VMOD_NAME_ALT, XKB_VMOD_NAME_LEVEL3, XKB_VMOD_NAME_LEVEL5, XKB_VMOD_NAME_NUM,
    XKB_VMOD_NAME_SCROLL, XKB_VMOD_NAME_SUPER,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_state_update_newer {
    pub c2rust_unnamed: C2Rust_Unnamed_16,
    pub extra: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub size: size_t,
    pub current: xkb_state_update,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct params {
    pub size: size_t,
    pub extra: uint64_t,
    pub enabled: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub root: params,
    pub components: params,
    pub layout_policy: params,
    pub error: xkb_error_code,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_layout_policy_update_newer {
    pub c2rust_unnamed: C2Rust_Unnamed_18,
    pub extra: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub size: size_t,
    pub current: xkb_layout_policy_update,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_state_components_update_newer {
    pub c2rust_unnamed: C2Rust_Unnamed_19,
    pub extra: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_19 {
    pub size: size_t,
    pub current: xkb_state_components_update,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub policy: xkb_layout_out_of_range_policy,
    pub redirect_group: xkb_layout_index_t,
    pub locked_group: xkb_layout_index_t,
    pub expected_group: xkb_layout_index_t,
}
pub const STICKY_KEY_EVENTS_API: sticky_key_activation = 2;
pub type sticky_key_activation = ::core::ffi::c_uint;
pub const STICKY_KEY_LEGACY_API: sticky_key_activation = 3;
pub const STICKY_KEY_ACTION_LOCKCONTROLS: sticky_key_activation = 1;
pub const STICKY_KEY_ACTION_SETCONTROLS: sticky_key_activation = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_events {
    pub events: [xkb_event; 3],
    pub events_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub keycode: xkb_keycode_t,
    pub repeats: bool,
    pub down: test_events,
    pub up: test_events,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub controls: xkb_keyboard_control_flags,
    pub kc: xkb_keycode_t,
    pub direction: xkb_key_direction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub controls: xkb_action_controls,
    pub overlays: xkb_overlay_mask_t,
}
unsafe extern "C" fn xkb_state_update_enabled_controls(
    mut state: *mut xkb_state,
    mut affect: xkb_keyboard_control_flags,
    mut controls: xkb_keyboard_control_flags,
) -> xkb_state_component {
    unsafe {
        let components: xkb_state_components_update = xkb_state_components_update {
            size: ::core::mem::size_of::<xkb_state_components_update>() as size_t,
            components: XKB_STATE_CONTROLS,
            affect_latched_mods: 0,
            latched_mods: 0,
            affect_locked_mods: 0,
            locked_mods: 0,
            latched_layout: 0,
            locked_layout: 0,
            affect_controls: affect,
            controls: controls,
        };
        let update: xkb_state_update = xkb_state_update {
            size: ::core::mem::size_of::<xkb_state_update>() as size_t,
            components: &raw const components,
            layout_policy: ::core::ptr::null::<xkb_layout_policy_update>(),
        };
        let mut changed: xkb_state_component = 0 as xkb_state_component;
        xkb_state_update_synthetic(state, &raw const update, &raw mut changed);
        return changed;
    }
}
unsafe extern "C" fn xkb_machine_update_enabled_controls(
    mut machine: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut affect: xkb_keyboard_control_flags,
    mut controls: xkb_keyboard_control_flags,
) -> ::core::ffi::c_int {
    unsafe {
        let components: xkb_state_component =
            (if affect as ::core::ffi::c_uint != 0 || controls as ::core::ffi::c_uint != 0 {
                XKB_STATE_CONTROLS as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as xkb_state_component;
        let components_update: xkb_state_components_update = xkb_state_components_update {
            size: ::core::mem::size_of::<xkb_state_components_update>() as size_t,
            components: components,
            affect_latched_mods: 0,
            latched_mods: 0,
            affect_locked_mods: 0,
            locked_mods: 0,
            latched_layout: 0,
            locked_layout: 0,
            affect_controls: affect,
            controls: controls,
        };
        let update: xkb_state_update = xkb_state_update {
            size: ::core::mem::size_of::<xkb_state_update>() as size_t,
            components: &raw const components_update,
            layout_policy: ::core::ptr::null::<xkb_layout_policy_update>(),
        };
        return xkb_machine_process_synthetic(machine, &raw const update, events)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn xkb_machine_update_latched_locked(
    mut machine: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut affect_latched_mods: xkb_mod_mask_t,
    mut latched_mods: xkb_mod_mask_t,
    mut affect_latched_layout: bool,
    mut latched_layout: int32_t,
    mut affect_locked_mods: xkb_mod_mask_t,
    mut locked_mods: xkb_mod_mask_t,
    mut affect_locked_layout: bool,
    mut locked_layout: int32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let components: xkb_state_component =
            ((if affect_latched_mods != 0 || latched_mods != 0 {
                XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) | (if affect_locked_mods != 0 || locked_mods != 0 {
                XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) | (if affect_latched_layout as ::core::ffi::c_int != 0 {
                XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) | (if affect_locked_layout as ::core::ffi::c_int != 0 {
                XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })) as xkb_state_component;
        let components_update: xkb_state_components_update = xkb_state_components_update {
            size: ::core::mem::size_of::<xkb_state_components_update>() as size_t,
            components: components,
            affect_latched_mods: affect_latched_mods,
            latched_mods: latched_mods,
            affect_locked_mods: affect_locked_mods,
            locked_mods: locked_mods,
            latched_layout: latched_layout,
            locked_layout: locked_layout,
            affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
            controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
        };
        let update: xkb_state_update = xkb_state_update {
            size: ::core::mem::size_of::<xkb_state_update>() as size_t,
            components: &raw const components_update,
            layout_policy: ::core::ptr::null::<xkb_layout_policy_update>(),
        };
        return xkb_machine_process_synthetic(machine, &raw const update, events)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn test_machine_options(mut ctx: *mut xkb_context) {
    unsafe {
        let mut options: *mut xkb_machine_options = xkb_machine_options_new(ctx);
        if !options.is_null() {
        } else {
            __assert_fail(
                b"options\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_uint,
                b"void test_machine_options(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_update_a11y_flags(
            options,
            4294966296 as xkb_a11y_flags,
            XKB_A11Y_NO_FLAGS,
        ) as ::core::ffi::c_int
            == XKB_ERROR_UNSUPPORTED_A11Y_FLAGS as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_update_a11y_flags(options, -1000, 0) == XKB_ERROR_UNSUPPORTED_A11Y_FLAGS\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                178 as ::core::ffi::c_uint,
                b"void test_machine_options(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_update_a11y_flags(options, 1000 as xkb_a11y_flags, XKB_A11Y_NO_FLAGS)
            as ::core::ffi::c_int
            == XKB_ERROR_UNSUPPORTED_A11Y_FLAGS as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_update_a11y_flags(options, 1000, 0) == XKB_ERROR_UNSUPPORTED_A11Y_FLAGS\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                180 as ::core::ffi::c_uint,
                b"void test_machine_options(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_update_a11y_flags(options, XKB_A11Y_NO_FLAGS, 1000 as xkb_a11y_flags)
            as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_update_a11y_flags( options, XKB_A11Y_NO_FLAGS, 1000) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                185 as ::core::ffi::c_uint,
                b"void test_machine_options(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut keymap: *mut xkb_keymap = xkb_keymap_new_from_names(
            ctx,
            ::core::ptr::null::<xkb_rule_names>(),
            XKB_KEYMAP_COMPILE_STRICT_MODE,
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_uint,
                b"void test_machine_options(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut sm: *mut xkb_machine = xkb_machine_new(keymap, options);
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_uint,
                b"void test_machine_options(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
        xkb_machine_options_destroy(options);
    }
}
unsafe extern "C" fn test_initial_derived_values(mut ctx: *mut xkb_context) {
    unsafe {
        let keymap: *mut xkb_keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"us\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"grp1_led:scroll\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_uint,
                b"void test_initial_derived_values(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let sm: *mut xkb_machine =
            xkb_machine_new(keymap, ::core::ptr::null::<xkb_machine_options>()) as *mut xkb_machine;
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                210 as ::core::ffi::c_uint,
                b"void test_initial_derived_values(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let state: *mut xkb_state = xkb_machine_get_state(sm) as *mut xkb_state;
        if xkb_state_led_name_is_active(
            state,
            b"Scroll Lock\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"xkb_state_led_name_is_active(state, XKB_LED_NAME_SCROLL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                212 as ::core::ffi::c_uint,
                b"void test_initial_derived_values(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_state_update(mut ctx: *mut xkb_context) {
    unsafe {
        let keymap: *mut xkb_keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"empty\0".as_ptr() as *const ::core::ffi::c_char,
            b"empty\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                226 as ::core::ffi::c_uint,
                b"void test_state_update(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let state: *mut xkb_state = xkb_state_new(keymap) as *mut xkb_state;
        if !state.is_null() {
        } else {
            __assert_fail(
                b"state\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                229 as ::core::ffi::c_uint,
                b"void test_state_update(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let sm: *mut xkb_machine =
            xkb_machine_new(keymap, ::core::ptr::null::<xkb_machine_options>()) as *mut xkb_machine;
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                231 as ::core::ffi::c_uint,
                b"void test_state_update(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let events: *mut xkb_events =
            xkb_events_new_batch(ctx, XKB_EVENTS_NO_FLAGS) as *mut xkb_events;
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_uint,
                b"void test_state_update(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        static mut tests: [C2Rust_Unnamed_17; 16] = [
            C2Rust_Unnamed_17 {
                root: params {
                    size: 0 as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_INVALID_STRUCT_SIZE,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: 1 as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_INVALID_STRUCT_SIZE,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0 as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_INVALID_STRUCT_SIZE,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 1 as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_INVALID_STRUCT_SIZE,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0 as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                error: XKB_ERROR_ABI_INVALID_STRUCT_SIZE,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 1 as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                error: XKB_ERROR_ABI_INVALID_STRUCT_SIZE,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_SUCCESS,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: ::core::mem::size_of::<xkb_state_components_update>() as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_SUCCESS,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: ::core::mem::size_of::<xkb_layout_policy_update>() as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                error: XKB_SUCCESS,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update_newer>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_SUCCESS,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: ::core::mem::size_of::<xkb_state_components_update_newer>() as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_SUCCESS,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: ::core::mem::size_of::<xkb_layout_policy_update_newer>() as size_t,
                    extra: 0,
                    enabled: true_0 != 0,
                },
                error: XKB_SUCCESS,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update_newer>() as size_t,
                    extra: (1 as uint64_t) << 0 as ::core::ffi::c_int,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_FORWARD_COMPAT,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update_newer>() as size_t,
                    extra: (1 as uint64_t) << 63 as ::core::ffi::c_int,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_FORWARD_COMPAT,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: ::core::mem::size_of::<xkb_state_components_update_newer>() as size_t,
                    extra: (1 as uint64_t) << 63 as ::core::ffi::c_int,
                    enabled: true_0 != 0,
                },
                layout_policy: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                error: XKB_ERROR_ABI_FORWARD_COMPAT,
            },
            C2Rust_Unnamed_17 {
                root: params {
                    size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                    extra: 0 as uint64_t,
                    enabled: false,
                },
                components: params {
                    size: 0,
                    extra: 0,
                    enabled: false_0 != 0,
                },
                layout_policy: params {
                    size: ::core::mem::size_of::<xkb_layout_policy_update_newer>() as size_t,
                    extra: (1 as uint64_t) << 63 as ::core::ffi::c_int,
                    enabled: true_0 != 0,
                },
                error: XKB_ERROR_ABI_FORWARD_COMPAT,
            },
        ];
        let mut s: size_t = 0 as size_t;
        while s
            < (::core::mem::size_of::<[C2Rust_Unnamed_17; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_17>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%zu ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_state_update\0".as_ptr() as *const ::core::ffi::c_char,
                s,
            );
            let components: xkb_state_components_update_newer = xkb_state_components_update_newer {
                c2rust_unnamed: C2Rust_Unnamed_19 {
                    current: xkb_state_components_update {
                        size: tests[s as usize].components.size,
                        components: 0 as xkb_state_component,
                        affect_latched_mods: 0,
                        latched_mods: 0,
                        affect_locked_mods: 0,
                        locked_mods: 0,
                        latched_layout: 0,
                        locked_layout: 0,
                        affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                        controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                    },
                },
                extra: tests[s as usize].components.extra,
            };
            let layout_policy: xkb_layout_policy_update_newer = xkb_layout_policy_update_newer {
                c2rust_unnamed: C2Rust_Unnamed_18 {
                    current: xkb_layout_policy_update {
                        size: tests[s as usize].layout_policy.size,
                        policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
                        redirect: 0,
                    },
                },
                extra: tests[s as usize].layout_policy.extra,
            };
            let update: xkb_state_update_newer = xkb_state_update_newer {
                c2rust_unnamed: C2Rust_Unnamed_16 {
                    current: xkb_state_update {
                        size: tests[s as usize].root.size,
                        components: if tests[s as usize].components.enabled as ::core::ffi::c_int
                            != 0
                        {
                            &raw const components as *mut xkb_state_components_update
                        } else {
                            ::core::ptr::null_mut::<xkb_state_components_update>()
                        },
                        layout_policy: if tests[s as usize].layout_policy.enabled
                            as ::core::ffi::c_int
                            != 0
                        {
                            &raw const layout_policy as *mut xkb_layout_policy_update
                        } else {
                            ::core::ptr::null_mut::<xkb_layout_policy_update>()
                        },
                    },
                },
                extra: tests[s as usize].root.extra,
            };
            let __cond: bool = tests[s as usize].error as ::core::ffi::c_int
                == xkb_state_update_synthetic(
                    state,
                    &raw const update as *mut xkb_state_update,
                    ::core::ptr::null_mut::<xkb_state_component>(),
                ) as ::core::ffi::c_int;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: xkb_state_update_synthetic. Expected %d, got: %d\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    tests[s as usize].error as ::core::ffi::c_int,
                    xkb_state_update_synthetic(
                        state,
                        &raw const update as *mut xkb_state_update,
                        ::core::ptr::null_mut::<xkb_state_component>(),
                    ) as ::core::ffi::c_int,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                        448 as ::core::ffi::c_uint,
                        b"void test_state_update(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            let __cond_0: bool = tests[s as usize].error as ::core::ffi::c_int
                == xkb_machine_process_synthetic(
                    sm,
                    &raw const update as *mut xkb_state_update,
                    events,
                ) as ::core::ffi::c_int;
            if !__cond_0 {
                fprintf(
                    stderr,
                    b"Assertion failure: xkb_machine_process_synthetic. Expected %d, got: %d\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    tests[s as usize].error as ::core::ffi::c_int,
                    xkb_machine_process_synthetic(
                        sm,
                        &raw const update as *mut xkb_state_update,
                        events,
                    ) as ::core::ffi::c_int,
                );
                if __cond_0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                        455 as ::core::ffi::c_uint,
                        b"void test_state_update(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            s = s.wrapping_add(1);
        }
        xkb_events_destroy(events);
        xkb_machine_unref(sm);
        xkb_state_unref(state);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn update_key(
    mut sm: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut state: *mut xkb_state,
    mut use_machine: bool,
    mut key: xkb_keycode_t,
    mut direction: xkb_key_direction,
) -> xkb_state_component {
    unsafe {
        if use_machine {
            if xkb_machine_process_key(sm, key, direction, events) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"xkb_machine_process_key(sm, key, direction, events) == 0\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    472 as ::core::ffi::c_uint,
                    b"enum xkb_state_component update_key(struct xkb_machine *, struct xkb_events *, struct xkb_state *, _Bool, xkb_keycode_t, enum xkb_key_direction)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            let mut event: *const xkb_event = ::core::ptr::null::<xkb_event>();
            let mut changed: xkb_state_component = 0 as xkb_state_component;
            loop {
                event = xkb_events_next(events);
                if event.is_null() {
                    break;
                }
                changed = (changed as ::core::ffi::c_uint
                    | xkb_state_update_event(state, event) as ::core::ffi::c_uint)
                    as xkb_state_component;
            }
            return changed;
        } else {
            return xkb_state_update_key(state, key, direction);
        };
    }
}
unsafe extern "C" fn update_controls(
    mut sm: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut state: *mut xkb_state,
    mut use_machine: bool,
    mut affect: xkb_keyboard_control_flags,
    mut controls: xkb_keyboard_control_flags,
) -> xkb_state_component {
    unsafe {
        if use_machine {
            if xkb_machine_update_enabled_controls(sm, events, affect, controls)
                == 0 as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"xkb_machine_update_enabled_controls(sm, events, affect, controls) == 0\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    495 as ::core::ffi::c_uint,
                    b"enum xkb_state_component update_controls(struct xkb_machine *, struct xkb_events *, struct xkb_state *, _Bool, enum xkb_keyboard_control_flags, enum xkb_keyboard_control_flags)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            let mut event: *const xkb_event = ::core::ptr::null::<xkb_event>();
            let mut changed: xkb_state_component = 0 as xkb_state_component;
            loop {
                event = xkb_events_next(events);
                if event.is_null() {
                    break;
                }
                changed = (changed as ::core::ffi::c_uint
                    | xkb_state_update_event(state, event) as ::core::ffi::c_uint)
                    as xkb_state_component;
            }
            return changed;
        } else {
            return xkb_state_update_enabled_controls(state, affect, controls);
        };
    }
}
unsafe extern "C" fn test_group_wrap(mut ctx: *mut xkb_context) {
    unsafe {
        static mut keymap_str: [::core::ffi::c_char; 174] = unsafe {
            ::core::mem::transmute::<
                [u8; 174],
                [::core::ffi::c_char; 174],
            >(
                *b"default xkb_keymap {\n    xkb_keycodes { <> = 1; };\n    xkb_types { type \"ONE_LEVEL\" { map[none] = 1; }; };\n    xkb_symbols {\n        key <> { [a], [b], [c], [d] };\n    };\n};\0",
            )
        };
        let keymap: *mut xkb_keymap = test_compile_buffer(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            &raw const keymap_str as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 174]>() as size_t,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                521 as ::core::ffi::c_uint,
                b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let num_layouts: xkb_layout_index_t = xkb_keymap_num_layouts(keymap) as xkb_layout_index_t;
        if num_layouts == 4 as xkb_layout_index_t {
        } else {
            __assert_fail(
                b"num_layouts == 4\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                523 as ::core::ffi::c_uint,
                b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let sm: *mut xkb_machine =
            xkb_machine_new(keymap, ::core::ptr::null::<xkb_machine_options>()) as *mut xkb_machine;
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                526 as ::core::ffi::c_uint,
                b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let state: *mut xkb_state = xkb_state_new(keymap) as *mut xkb_state;
        if !state.is_null() {
        } else {
            __assert_fail(
                b"state\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                528 as ::core::ffi::c_uint,
                b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let events: *mut xkb_events =
            xkb_events_new_batch(ctx, XKB_EVENTS_NO_FLAGS) as *mut xkb_events;
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                532 as ::core::ffi::c_uint,
                b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut event: *const xkb_event = ::core::ptr::null::<xkb_event>();
        let mut tests: [C2Rust_Unnamed_20; 6] = [
            C2Rust_Unnamed_20 {
                policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
                redirect_group: 0 as xkb_layout_index_t,
                locked_group: -1 as ::core::ffi::c_int as xkb_layout_index_t,
                expected_group: num_layouts.wrapping_sub(1 as xkb_layout_index_t),
            },
            C2Rust_Unnamed_20 {
                policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
                redirect_group: 0 as xkb_layout_index_t,
                locked_group: num_layouts.wrapping_add(1 as xkb_layout_index_t),
                expected_group: 1 as xkb_layout_index_t,
            },
            C2Rust_Unnamed_20 {
                policy: XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
                redirect_group: 0 as xkb_layout_index_t,
                locked_group: -1 as ::core::ffi::c_int as xkb_layout_index_t,
                expected_group: 0 as xkb_layout_index_t,
            },
            C2Rust_Unnamed_20 {
                policy: XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
                redirect_group: 0 as xkb_layout_index_t,
                locked_group: num_layouts.wrapping_add(1 as xkb_layout_index_t),
                expected_group: num_layouts.wrapping_sub(1 as xkb_layout_index_t),
            },
            C2Rust_Unnamed_20 {
                policy: XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
                redirect_group: 2 as xkb_layout_index_t,
                locked_group: -1 as ::core::ffi::c_int as xkb_layout_index_t,
                expected_group: 2 as xkb_layout_index_t,
            },
            C2Rust_Unnamed_20 {
                policy: XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
                redirect_group: 2 as xkb_layout_index_t,
                locked_group: num_layouts.wrapping_add(1 as xkb_layout_index_t),
                expected_group: 2 as xkb_layout_index_t,
            },
        ];
        let mut t: uint8_t = 0 as uint8_t;
        while (t as ::core::ffi::c_int)
            < (::core::mem::size_of::<[C2Rust_Unnamed_20; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_20>() as usize)
                as uint8_t as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_group_wrap\0".as_ptr() as *const ::core::ffi::c_char,
                t as ::core::ffi::c_int,
            );
            let mut layout_policy: xkb_layout_policy_update = xkb_layout_policy_update {
                size: ::core::mem::size_of::<xkb_layout_policy_update>() as size_t,
                policy: tests[t as usize].policy,
                redirect: tests[t as usize].redirect_group,
            };
            let components: xkb_state_components_update = xkb_state_components_update {
                size: ::core::mem::size_of::<xkb_state_components_update>() as size_t,
                components: XKB_STATE_LAYOUT_LOCKED,
                affect_latched_mods: 0,
                latched_mods: 0,
                affect_locked_mods: 0,
                locked_mods: 0,
                latched_layout: 0,
                locked_layout: tests[t as usize].locked_group as int32_t,
                affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
            };
            let req: xkb_state_update = xkb_state_update {
                size: ::core::mem::size_of::<xkb_state_update>() as size_t,
                components: &raw const components,
                layout_policy: &raw mut layout_policy,
            };
            if xkb_machine_process_synthetic(sm, &raw const req, events) as u64 == 0 {
            } else {
                __assert_fail(
                    b"!xkb_machine_process_synthetic(sm, &req, events)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_uint,
                    b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            loop {
                event = xkb_events_next(events);
                if event.is_null() {
                    break;
                }
                xkb_state_update_event(state, event);
            }
            let __cond: bool = tests[t as usize].expected_group
                == xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE);
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: unexpected effective group. Expected %u, got: %u\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    tests[t as usize].expected_group,
                    xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE),
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                        611 as ::core::ffi::c_uint,
                        b"void test_group_wrap(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            t = t.wrapping_add(1);
        }
        xkb_events_destroy(events);
        xkb_state_unref(state);
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_sticky_keys(mut ctx: *mut xkb_context) {
    unsafe {
        let keymap: *mut xkb_keymap = test_compile_rmlvo(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"ca,cz,de\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"controls,grp:lwin_switch,grp:menu_toggle\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                627 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut sm: *mut xkb_machine =
            xkb_machine_new(keymap, ::core::ptr::null::<xkb_machine_options>());
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                630 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut events: *mut xkb_events = xkb_events_new_batch(ctx, XKB_EVENTS_NO_FLAGS);
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                632 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut state: *mut xkb_state = xkb_state_new(keymap);
        if !state.is_null() {
        } else {
            __assert_fail(
                b"state\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                634 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let shift: xkb_mod_mask_t = xkb_keymap_mod_get_mask2(
            keymap,
            XKB_MOD_INDEX_SHIFT as ::core::ffi::c_int as xkb_mod_index_t,
        ) as xkb_mod_mask_t;
        let caps: xkb_mod_mask_t = xkb_keymap_mod_get_mask2(
            keymap,
            XKB_MOD_INDEX_CAPS as ::core::ffi::c_int as xkb_mod_index_t,
        ) as xkb_mod_mask_t;
        let ctrl: xkb_mod_mask_t = xkb_keymap_mod_get_mask2(
            keymap,
            XKB_MOD_INDEX_CTRL as ::core::ffi::c_int as xkb_mod_index_t,
        ) as xkb_mod_mask_t;
        let mut mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut controls: xkb_keyboard_control_flags = XKB_KEYBOARD_CONTROL_NO_FLAGS;
        let mut changed: xkb_state_component = 0 as xkb_state_component;
        controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
        if controls as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"controls == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                648 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut tests: [sticky_key_activation; 4] = [
            STICKY_KEY_ACTION_SETCONTROLS,
            STICKY_KEY_ACTION_LOCKCONTROLS,
            STICKY_KEY_EVENTS_API,
            STICKY_KEY_LEGACY_API,
        ];
        let mut t: uint8_t = 0 as uint8_t;
        while (t as ::core::ffi::c_int)
            < (::core::mem::size_of::<[sticky_key_activation; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<sticky_key_activation>() as usize)
                as uint8_t as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_sticky_keys\0".as_ptr() as *const ::core::ffi::c_char,
                t as ::core::ffi::c_int,
            );
            let use_events: bool = tests[t as usize] as ::core::ffi::c_uint
                == STICKY_KEY_EVENTS_API as ::core::ffi::c_int as ::core::ffi::c_uint;
            match tests[t as usize] as ::core::ffi::c_uint {
                0 => {
                    changed = update_key(
                        sm,
                        events,
                        state,
                        use_events,
                        (KEY_F1 + EVDEV_OFFSET) as xkb_keycode_t,
                        XKB_KEY_DOWN,
                    );
                    if changed as ::core::ffi::c_uint
                        == XKB_STATE_CONTROLS as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == XKB_STATE_CONTROLS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            675 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                1 => {
                    changed = update_key(
                        sm,
                        events,
                        state,
                        use_events,
                        (KEY_F2 + EVDEV_OFFSET) as xkb_keycode_t,
                        XKB_KEY_DOWN,
                    );
                    if changed as ::core::ffi::c_uint
                        == XKB_STATE_CONTROLS as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == XKB_STATE_CONTROLS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            681 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
                    if controls as ::core::ffi::c_uint
                        == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"controls == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            683 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    changed = update_key(
                        sm,
                        events,
                        state,
                        use_events,
                        (KEY_F2 + EVDEV_OFFSET) as xkb_keycode_t,
                        XKB_KEY_UP,
                    );
                    if changed as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                    } else {
                        __assert_fail(
                            b"changed == 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            686 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                2 => {
                    changed = update_controls(
                        sm,
                        events,
                        state,
                        true_0 != 0,
                        XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
                        XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
                    );
                    if changed as ::core::ffi::c_uint
                        == XKB_STATE_CONTROLS as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == XKB_STATE_CONTROLS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            692 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
                    if controls as ::core::ffi::c_uint
                        == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"controls == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            694 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                3 => {
                    changed = xkb_state_update_enabled_controls(
                        state,
                        XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
                        XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
                    );
                    if changed as ::core::ffi::c_uint
                        == XKB_STATE_CONTROLS as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == XKB_STATE_CONTROLS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            703 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
                    if controls as ::core::ffi::c_uint
                        == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"controls == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            705 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                _ => {}
            }
            controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
            if controls as ::core::ffi::c_uint
                == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS as ::core::ffi::c_int
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"controls == XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    709 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    714 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    717 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == shift {
            } else {
                __assert_fail(
                    b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    724 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    727 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == 0 as xkb_mod_mask_t {
            } else {
                __assert_fail(
                    b"mods == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    729 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    734 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    737 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == shift {
            } else {
                __assert_fail(
                    b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    739 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTCTRL + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    742 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTCTRL + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    745 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == shift | ctrl {
            } else {
                __assert_fail(
                    b"mods == (shift | ctrl)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    747 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_Q + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_LATCHED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    750 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_Q + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"changed == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    753 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_COMPOSE + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                    | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                    | XKB_STATE_LEDS as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_LAYOUT_LOCKED | XKB_STATE_LAYOUT_EFFECTIVE | XKB_STATE_LEDS)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    759 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_COMPOSE + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"changed == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    762 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LOCKED) == 1 as xkb_layout_index_t
            {
            } else {
                __assert_fail(
                    b"xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LOCKED) == 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    763 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE)
                == 1 as xkb_layout_index_t
            {
            } else {
                __assert_fail(
                    b"xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE) == 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    764 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTMETA + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_LAYOUT_DEPRESSED | XKB_STATE_LAYOUT_EFFECTIVE)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    767 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTMETA + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_LAYOUT_DEPRESSED | XKB_STATE_LAYOUT_LATCHED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    770 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LATCHED)
                == 1 as xkb_layout_index_t
            {
            } else {
                __assert_fail(
                    b"xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LATCHED) == 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    771 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LOCKED) == 1 as xkb_layout_index_t
            {
            } else {
                __assert_fail(
                    b"xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LOCKED) == 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    772 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE)
                == 2 as xkb_layout_index_t
            {
            } else {
                __assert_fail(
                    b"xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE) == 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    773 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    778 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    781 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == shift {
            } else {
                __assert_fail(
                    b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    783 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_CAPSLOCK + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                    | XKB_STATE_LEDS as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LOCKED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LEDS)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    787 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_CAPSLOCK + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    790 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == shift | caps {
            } else {
                __assert_fail(
                    b"mods == (shift | caps)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    792 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            match tests[t as usize] as ::core::ffi::c_uint {
                0 => {
                    changed = update_key(
                        sm,
                        events,
                        state,
                        use_events,
                        (KEY_F1 + EVDEV_OFFSET) as xkb_keycode_t,
                        XKB_KEY_UP,
                    );
                    if changed as ::core::ffi::c_uint
                        == (XKB_STATE_CONTROLS as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == (XKB_STATE_CONTROLS | XKB_STATE_LAYOUT_LATCHED | XKB_STATE_LAYOUT_LOCKED | XKB_STATE_LAYOUT_EFFECTIVE | XKB_STATE_MODS_LATCHED | XKB_STATE_MODS_LOCKED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LEDS)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            807 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                1 => {
                    changed = update_key(
                        sm,
                        events,
                        state,
                        use_events,
                        (KEY_F2 + EVDEV_OFFSET) as xkb_keycode_t,
                        XKB_KEY_DOWN,
                    );
                    if changed as ::core::ffi::c_uint
                        == (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == (XKB_STATE_MODS_LATCHED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LAYOUT_LATCHED | XKB_STATE_LAYOUT_EFFECTIVE)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            816 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
                    if mods == caps {
                    } else {
                        __assert_fail(
                            b"mods == caps\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                            818 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                    changed = update_key(
                        sm,
                        events,
                        state,
                        use_events,
                        (KEY_F2 + EVDEV_OFFSET) as xkb_keycode_t,
                        XKB_KEY_UP,
                    );
                    if changed as ::core::ffi::c_uint
                        == (XKB_STATE_CONTROLS as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == (XKB_STATE_CONTROLS | XKB_STATE_LAYOUT_LOCKED | XKB_STATE_LAYOUT_EFFECTIVE | XKB_STATE_MODS_LOCKED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LEDS)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            826 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                2 => {
                    changed = update_controls(
                        sm,
                        events,
                        state,
                        true_0 != 0,
                        XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
                        XKB_KEYBOARD_CONTROL_NO_FLAGS,
                    );
                    if changed as ::core::ffi::c_uint
                        == (XKB_STATE_CONTROLS as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == (XKB_STATE_CONTROLS | XKB_STATE_LAYOUT_LATCHED | XKB_STATE_LAYOUT_LOCKED | XKB_STATE_LAYOUT_EFFECTIVE | XKB_STATE_MODS_LATCHED | XKB_STATE_MODS_LOCKED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LEDS)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            838 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                3 => {
                    changed = xkb_state_update_enabled_controls(
                        state,
                        XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
                        XKB_KEYBOARD_CONTROL_NO_FLAGS,
                    );
                    if changed as ::core::ffi::c_uint
                        == (XKB_STATE_CONTROLS as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"changed == (XKB_STATE_CONTROLS | XKB_STATE_LAYOUT_LATCHED | XKB_STATE_LAYOUT_LOCKED | XKB_STATE_LAYOUT_EFFECTIVE | XKB_STATE_MODS_LATCHED | XKB_STATE_MODS_LOCKED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LEDS)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/server-state.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            852 as ::core::ffi::c_uint,
                            b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    };
                }
                _ => {}
            }
            controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
            if controls as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"controls == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    856 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == 0 as xkb_mod_mask_t {
            } else {
                __assert_fail(
                    b"mods == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    858 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE)
                == 0 as xkb_layout_index_t
            {
            } else {
                __assert_fail(
                    b"xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    859 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_DOWN,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    864 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            changed = update_key(
                sm,
                events,
                state,
                use_events,
                (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
                XKB_KEY_UP,
            );
            if changed as ::core::ffi::c_uint
                == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                    | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    867 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
            if mods == 0 as xkb_mod_mask_t {
            } else {
                __assert_fail(
                    b"mods == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    869 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            controls = xkb_state_serialize_enabled_controls(state, XKB_STATE_CONTROLS);
            if controls as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"controls == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    872 as ::core::ffi::c_uint,
                    b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            t = t.wrapping_add(1);
        }
        xkb_state_unref(state);
        xkb_events_destroy(events);
        xkb_machine_unref(sm);
        let sm_options: *mut xkb_machine_options =
            xkb_machine_options_new(ctx) as *mut xkb_machine_options;
        if !sm_options.is_null() {
        } else {
            __assert_fail(
                b"sm_options\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                884 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_update_a11y_flags(
            sm_options,
            XKB_A11Y_LATCH_TO_LOCK,
            XKB_A11Y_LATCH_TO_LOCK,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_update_a11y_flags( sm_options, XKB_A11Y_LATCH_TO_LOCK, XKB_A11Y_LATCH_TO_LOCK) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                888 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        sm = xkb_machine_new(keymap, sm_options);
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                890 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_options_destroy(sm_options);
        events = xkb_events_new_batch(ctx, XKB_EVENTS_NO_FLAGS);
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                893 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        state = xkb_state_new(keymap);
        if !state.is_null() {
        } else {
            __assert_fail(
                b"state\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                895 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        update_controls(
            sm,
            events,
            state,
            true_0 != 0,
            XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
            XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS,
        );
        changed = update_key(
            sm,
            events,
            state,
            true_0 != 0,
            (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
            XKB_KEY_DOWN,
        );
        if changed as ::core::ffi::c_uint
            == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_EFFECTIVE)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                903 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        changed = update_key(
            sm,
            events,
            state,
            true_0 != 0,
            (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
            XKB_KEY_UP,
        );
        if changed as ::core::ffi::c_uint
            == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"changed == (XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                906 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_LATCHED);
        if mods == shift {
        } else {
            __assert_fail(
                b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                908 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
        if mods == shift {
        } else {
            __assert_fail(
                b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                910 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        changed = update_key(
            sm,
            events,
            state,
            true_0 != 0,
            (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
            XKB_KEY_DOWN,
        );
        if changed as ::core::ffi::c_uint
            == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                | XKB_STATE_LEDS as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"changed == ( XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LATCHED | XKB_STATE_MODS_LOCKED | XKB_STATE_LEDS )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                917 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_DEPRESSED);
        if mods == shift {
        } else {
            __assert_fail(
                b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                919 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_LATCHED);
        if mods == 0 as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"mods == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                921 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_LOCKED);
        if mods == shift {
        } else {
            __assert_fail(
                b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                923 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        changed = update_key(
            sm,
            events,
            state,
            true_0 != 0,
            (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
            XKB_KEY_UP,
        );
        if changed as ::core::ffi::c_uint
            == XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"changed == XKB_STATE_MODS_DEPRESSED\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                926 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
        if mods == shift {
        } else {
            __assert_fail(
                b"mods == shift\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                928 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        changed = update_key(
            sm,
            events,
            state,
            true_0 != 0,
            (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
            XKB_KEY_DOWN,
        );
        if changed as ::core::ffi::c_uint
            == XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"changed == XKB_STATE_MODS_DEPRESSED\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                932 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        changed = update_key(
            sm,
            events,
            state,
            true_0 != 0,
            (KEY_LEFTSHIFT + EVDEV_OFFSET) as xkb_keycode_t,
            XKB_KEY_UP,
        );
        if changed as ::core::ffi::c_uint
            == (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                | XKB_STATE_LEDS as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"changed == ( XKB_STATE_MODS_DEPRESSED | XKB_STATE_MODS_LOCKED | XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LEDS )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                938 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        mods = xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE);
        if mods == 0 as xkb_mod_mask_t {
        } else {
            __assert_fail(
                b"mods == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                940 as ::core::ffi::c_uint,
                b"void test_sticky_keys(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_state_unref(state);
        xkb_events_destroy(events);
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_redirect_key(mut ctx: *mut xkb_context) {
    unsafe {
        let keymap: *mut xkb_keymap = test_compile_file(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"keymaps/redirect-key-1.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                955 as ::core::ffi::c_uint,
                b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut sm: *mut xkb_machine =
            xkb_machine_new(keymap, ::core::ptr::null::<xkb_machine_options>());
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                958 as ::core::ffi::c_uint,
                b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        static mut shift: xkb_mod_mask_t =
            (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_SHIFT as ::core::ffi::c_int;
        static mut ctrl: xkb_mod_mask_t =
            (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_CTRL as ::core::ffi::c_int;
        let mut events: *mut xkb_events = xkb_events_new_batch(ctx, XKB_EVENTS_NO_FLAGS);
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                964 as ::core::ffi::c_uint,
                b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq2(
            keymap,
            sm,
            events,
            30 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            31 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            31 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            31 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            31 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x61 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x73 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x73 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x73 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x73 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq2( keymap, sm, events, KEY_A, BOTH, XKB_KEY_a, NEXT, KEY_A, DOWN, XKB_KEY_a, NEXT, KEY_A, REPEAT, XKB_KEY_a, NEXT, KEY_A, UP, XKB_KEY_a, NEXT, KEY_S, BOTH, XKB_KEY_a, NEXT, KEY_S, DOWN, XKB_KEY_a, NEXT, KEY_S, REPEAT, XKB_KEY_a, NEXT, KEY_S, UP, XKB_KEY_a, NEXT, KEY_D, BOTH, XKB_KEY_s, NEXT, KEY_D, DOWN, XKB_KEY_s, NEXT, KEY_D, REPEAT, XKB_KEY_s, NEXT, KEY_D, UP, XKB_KEY_s, FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                980 as ::core::ffi::c_uint,
                b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false_0 != 0,
            0 as int32_t,
            ctrl,
            ctrl,
            false_0 != 0,
            0 as int32_t,
        );
        let tests: [C2Rust_Unnamed_21; 3] = [
            C2Rust_Unnamed_21 {
                keycode: (EVDEV_OFFSET + KEY_A) as xkb_keycode_t,
                repeats: false_0 != 0,
                down: test_events {
                    events: [
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_KEY_DOWN,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                keycode: (EVDEV_OFFSET + KEY_A) as xkb_keycode_t,
                            },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                    ],
                    events_count: 1 as ::core::ffi::c_uint,
                },
                up: test_events {
                    events: [
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_KEY_UP,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                keycode: (EVDEV_OFFSET + KEY_A) as xkb_keycode_t,
                            },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                    ],
                    events_count: 1 as ::core::ffi::c_uint,
                },
            },
            C2Rust_Unnamed_21 {
                keycode: (EVDEV_OFFSET + KEY_S) as xkb_keycode_t,
                repeats: true_0 != 0,
                down: test_events {
                    events: [
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_KEY_DOWN,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                keycode: (EVDEV_OFFSET + KEY_A) as xkb_keycode_t,
                            },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                    ],
                    events_count: 1 as ::core::ffi::c_uint,
                },
                up: test_events {
                    events: [
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_KEY_UP,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                keycode: (EVDEV_OFFSET + KEY_A) as xkb_keycode_t,
                            },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                        xkb_event {
                            type_0: 0 as xkb_event_type,
                            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                        },
                    ],
                    events_count: 1 as ::core::ffi::c_uint,
                },
            },
            C2Rust_Unnamed_21 {
                keycode: (EVDEV_OFFSET + KEY_D) as xkb_keycode_t,
                repeats: true_0 != 0,
                down: test_events {
                    events: [
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                components: C2Rust_Unnamed_14 {
                                    components: state_components {
                                        base_group: 0,
                                        latched_group: 0,
                                        locked_group: 0,
                                        group: 0,
                                        base_mods: shift,
                                        latched_mods: shift,
                                        locked_mods: shift,
                                        mods: shift,
                                        leds: 0,
                                        controls: 0 as xkb_action_controls,
                                    },
                                    changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                                        as xkb_state_component,
                                },
                            },
                        },
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_KEY_DOWN,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                keycode: (EVDEV_OFFSET + KEY_S) as xkb_keycode_t,
                            },
                        },
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                components: C2Rust_Unnamed_14 {
                                    components: state_components {
                                        base_group: 0,
                                        latched_group: 0,
                                        locked_group: 0,
                                        group: 0,
                                        base_mods: 0 as xkb_mod_mask_t,
                                        latched_mods: 0 as xkb_mod_mask_t,
                                        locked_mods: ctrl,
                                        mods: ctrl,
                                        leds: 0,
                                        controls: 0 as xkb_action_controls,
                                    },
                                    changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                                        as xkb_state_component,
                                },
                            },
                        },
                    ],
                    events_count: 3 as ::core::ffi::c_uint,
                },
                up: test_events {
                    events: [
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                components: C2Rust_Unnamed_14 {
                                    components: state_components {
                                        base_group: 0,
                                        latched_group: 0,
                                        locked_group: 0,
                                        group: 0,
                                        base_mods: shift,
                                        latched_mods: shift,
                                        locked_mods: shift,
                                        mods: shift,
                                        leds: 0,
                                        controls: 0 as xkb_action_controls,
                                    },
                                    changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                                        as xkb_state_component,
                                },
                            },
                        },
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_KEY_UP,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                keycode: (EVDEV_OFFSET + KEY_S) as xkb_keycode_t,
                            },
                        },
                        xkb_event {
                            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                            c2rust_unnamed: C2Rust_Unnamed_13 {
                                components: C2Rust_Unnamed_14 {
                                    components: state_components {
                                        base_group: 0,
                                        latched_group: 0,
                                        locked_group: 0,
                                        group: 0,
                                        base_mods: 0 as xkb_mod_mask_t,
                                        latched_mods: 0 as xkb_mod_mask_t,
                                        locked_mods: ctrl,
                                        mods: ctrl,
                                        leds: 0,
                                        controls: 0 as xkb_action_controls,
                                    },
                                    changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                                        as xkb_state_component,
                                },
                            },
                        },
                    ],
                    events_count: 3 as ::core::ffi::c_uint,
                },
            },
        ];
        let mut t: uint8_t = 0 as uint8_t;
        while (t as ::core::ffi::c_int)
            < (::core::mem::size_of::<[C2Rust_Unnamed_21; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_21>() as usize)
                as uint8_t as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u, keycode: %u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_redirect_key\0".as_ptr() as *const ::core::ffi::c_char,
                t as ::core::ffi::c_int,
                tests[t as usize].keycode,
            );
            if xkb_keymap_key_repeats(keymap, tests[t as usize].keycode)
                == tests[t as usize].repeats as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"xkb_keymap_key_repeats(keymap, tests[t].keycode) == tests[t].repeats\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1126 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_machine_process_key(sm, tests[t as usize].keycode, XKB_KEY_DOWN, events) as u64
                == 0
            {
            } else {
                __assert_fail(
                    b"!xkb_machine_process_key(sm, tests[t].keycode, XKB_KEY_DOWN, events)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1128 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if check_events(
                events,
                &raw const (*(&raw const tests as *const C2Rust_Unnamed_21).offset(t as isize))
                    .down
                    .events as *const xkb_event,
                tests[t as usize].down.events_count as size_t,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"check_events(events, tests[t].down.events, tests[t].down.events_count)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_machine_process_key(sm, tests[t as usize].keycode, XKB_KEY_REPEATED, events)
                as u64
                == 0
            {
            } else {
                __assert_fail(
                    b"!xkb_machine_process_key(sm, tests[t].keycode, XKB_KEY_REPEATED, events)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1132 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if tests[t as usize].repeats {
                let mut ref_0: [xkb_event; 3] = [
                    xkb_event {
                        type_0: 0 as xkb_event_type,
                        c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                    },
                    xkb_event {
                        type_0: 0 as xkb_event_type,
                        c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                    },
                    xkb_event {
                        type_0: 0 as xkb_event_type,
                        c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
                    },
                ];
                memcpy(
                    &raw mut ref_0 as *mut xkb_event as *mut ::core::ffi::c_void,
                    &raw const (*(&raw const tests as *const C2Rust_Unnamed_21).offset(t as isize))
                        .down
                        .events as *const xkb_event
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[xkb_event; 3]>() as size_t,
                );
                ref_0[(tests[t as usize].down.events_count == 3 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int as usize]
                    .type_0 = XKB_EVENT_TYPE_KEY_REPEATED;
                if check_events(
                    events,
                    &raw mut ref_0 as *mut xkb_event,
                    tests[t as usize].down.events_count as size_t,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"check_events(events, ref, tests[t].down.events_count)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1138 as ::core::ffi::c_uint,
                        b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            } else {
                if check_events(events, ::core::ptr::null::<xkb_event>(), 0 as size_t)
                    as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"check_events(events, NULL, 0)\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1140 as ::core::ffi::c_uint,
                        b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            if xkb_machine_process_key(sm, tests[t as usize].keycode, XKB_KEY_UP, events) as u64
                == 0
            {
            } else {
                __assert_fail(
                    b"!xkb_machine_process_key(sm, tests[t].keycode, XKB_KEY_UP, events)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1143 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if check_events(
                events,
                &raw const (*(&raw const tests as *const C2Rust_Unnamed_21).offset(t as isize))
                    .up
                    .events as *const xkb_event,
                tests[t as usize].up.events_count as size_t,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"check_events(events, tests[t].up.events, tests[t].up.events_count)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1145 as ::core::ffi::c_uint,
                    b"void test_redirect_key(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            t = t.wrapping_add(1);
        }
        xkb_events_destroy(events);
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_shortcuts_tweak(mut context: *mut xkb_context) {
    unsafe {
        let keymap: *mut xkb_keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,il,de,ru\0".as_ptr() as *const ::core::ffi::c_char,
            b",,neo,\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:menu_toggle,grp:win_switch,ctrl:rctrl_latch,ctrl:copy\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1160 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let ctrl: xkb_mod_mask_t =
            (1 as xkb_mod_mask_t) << XKB_MOD_INDEX_CTRL as ::core::ffi::c_int;
        let alt: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_ALT.as_ptr()) as xkb_mod_mask_t;
        let level3: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_LEVEL3.as_ptr()) as xkb_mod_mask_t;
        let level5: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_LEVEL5.as_ptr()) as xkb_mod_mask_t;
        let options: *mut xkb_machine_options =
            xkb_machine_options_new(context) as *mut xkb_machine_options;
        if !options.is_null() {
        } else {
            __assert_fail(
                b"options\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1168 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_update_shortcut_mods(options, ctrl, ctrl) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_update_shortcut_mods(options, ctrl, ctrl) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1171 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_shortcut_layout(
            options,
            1 as xkb_layout_index_t,
            2 as xkb_layout_index_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_remap_shortcut_layout(options, 1, 2) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1172 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_shortcut_layout(
            options,
            3 as xkb_layout_index_t,
            0 as xkb_layout_index_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_remap_shortcut_layout(options, 3, 0) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1173 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut sm: *mut xkb_machine = xkb_machine_new(keymap, options);
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1176 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let events: *mut xkb_events =
            xkb_events_new_batch(context, XKB_EVENTS_NO_FLAGS) as *mut xkb_events;
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1180 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq2(
            keymap,
            sm,
            events,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x63 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1008ff57 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xff7e as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x3c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x3c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x3c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe4 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe4 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x78 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfc as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xe4 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe11 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe13 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6ca as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1008ff57 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6ca as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d1 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x6d3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1008ff57 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x71 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x2f as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xce6 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            86 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x3c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            97 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xffe4 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq2( keymap, sm, events, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_C , BOTH, XKB_KEY_c , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_C , BOTH, XKB_KEY_XF86Copy , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_LEFTMETA, DOWN, XKB_KEY_ISO_Group_Shift, NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_LEFTMETA, UP , XKB_KEY_ISO_Group_Shift, NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_LEFTMETA, DOWN, XKB_KEY_ISO_Group_Shift, NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_LEFTMETA, UP , XKB_KEY_ISO_Group_Shift, NEXT, KEY_COMPOSE , BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_102ND , BOTH, XKB_KEY_less , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Q , DOWN, XKB_KEY_x , NEXT, KEY_Q , REPEAT, XKB_KEY_x , NEXT, KEY_Q , UP , XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level5_Lock, NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_102ND , BOTH, XKB_KEY_less , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level5_Lock, NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_102ND , BOTH, XKB_KEY_less , NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_COMPOSE , BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_C , BOTH, XKB_KEY_adiaeresis , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level5_Lock, NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_C , BOTH, XKB_KEY_adiaeresis , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level5_Lock, NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level5_Lock, NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_x , NEXT, KEY_Z , BOTH, XKB_KEY_udiaeresis , NEXT, KEY_C , BOTH, XKB_KEY_adiaeresis , NEXT, KEY_102ND , DOWN, XKB_KEY_ISO_Level5_Shift, NEXT, KEY_102ND , UP , XKB_KEY_ISO_Level5_Lock, NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_COMPOSE , BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Q , BOTH, XKB_KEY_Cyrillic_shorti, NEXT, KEY_Z , BOTH, XKB_KEY_Cyrillic_ya , NEXT, KEY_C , BOTH, XKB_KEY_Cyrillic_es , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_C , BOTH, XKB_KEY_XF86Copy , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Q , BOTH, XKB_KEY_Cyrillic_shorti, NEXT, KEY_Z , BOTH, XKB_KEY_Cyrillic_ya , NEXT, KEY_C , BOTH, XKB_KEY_Cyrillic_es , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_C , BOTH, XKB_KEY_XF86Copy , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_COMPOSE , BOTH, XKB_KEY_ISO_Next_Group , NEXT, KEY_Q , BOTH, XKB_KEY_q , NEXT, KEY_Z , BOTH, XKB_KEY_z , NEXT, KEY_COMPOSE , BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Q , BOTH, XKB_KEY_slash , NEXT, KEY_Z , BOTH, XKB_KEY_hebrew_zain , NEXT, KEY_102ND , BOTH, XKB_KEY_less , NEXT, KEY_RIGHTCTRL,BOTH, XKB_KEY_Control_R , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1322 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let group2: xkb_led_mask_t = (1 as xkb_led_mask_t)
            << xkb_keymap_led_get_index(
                keymap,
                b"Group 2\0".as_ptr() as *const ::core::ffi::c_char,
            );
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1331 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected: [xkb_event; 4] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: 0 as xkb_mod_mask_t,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 4]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1386 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_REPEATED,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_REPEATED, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1389 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_0: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_KEY_REPEATED,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            },
        }];
        if check_events(
            events,
            &raw const expected_0 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1396 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1399 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_1: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_KEY_UP,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            },
        }];
        if check_events(
            events,
            &raw const expected_1 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1406 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (97 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_RIGHTCTRL + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1409 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_2: [xkb_event; 2] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (97 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: ctrl,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_2 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 2]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1433 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (97 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_REPEATED,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_RIGHTCTRL + EVDEV_OFFSET, XKB_KEY_REPEATED, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1436 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_3: [xkb_event; 1] = [xkb_event {
            type_0: 0 as xkb_event_type,
            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
        }];
        if check_events(
            events,
            &raw const expected_3 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1440 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (97 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_RIGHTCTRL + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1443 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_4: [xkb_event; 4] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: ctrl,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (97 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: ctrl,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LATCHED as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_4 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 4]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1501 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (86 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_102ND + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1504 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_5: [xkb_event; 4] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (86 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl | level5,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_5 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 4]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1561 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1564 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_6: [xkb_event; 4] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl | level5,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: ctrl,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: ctrl | level5,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: level5,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_6 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 4]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1622 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1625 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_7: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_KEY_UP,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            },
        }];
        if check_events(
            events,
            &raw const expected_7 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1632 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (86 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_102ND + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1635 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_8: [xkb_event; 2] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (86 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: 0 as xkb_mod_mask_t,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_8 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 2]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1659 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            ctrl,
            ctrl,
            false,
            0 as int32_t,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            true,
            1 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, ctrl, ctrl, false, 0, 0, 0, true, 1) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1668 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_9: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 0 as int32_t,
                        locked_group: 1 as int32_t,
                        group: 1 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: ctrl,
                        locked_mods: 0 as xkb_mod_mask_t,
                        mods: ctrl,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_9 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1687 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            ctrl,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl,
            ctrl,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, ctrl, 0, false, 0, ctrl, ctrl, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1692 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_10: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 0 as int32_t,
                        locked_group: 1 as int32_t,
                        group: 1 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: 0 as xkb_mod_mask_t,
                        locked_mods: ctrl,
                        mods: ctrl,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                        | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_10 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1711 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1714 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_11: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_11 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1755 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_REPEATED,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_REPEATED, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1758 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_12: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_REPEATED,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_12 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1799 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1802 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_13: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 1 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 2 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_13 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1843 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            true,
            1 as int32_t,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            true,
            2 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, true, 1, 0, 0, true, 2) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1848 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_14: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 1 as int32_t,
                        locked_group: 2 as int32_t,
                        group: 3 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: 0 as xkb_mod_mask_t,
                        locked_mods: ctrl,
                        mods: ctrl,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_14 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1868 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, ctrl, 0, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1873 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_15: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 1 as int32_t,
                        locked_group: 2 as int32_t,
                        group: 3 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: 0 as xkb_mod_mask_t,
                        locked_mods: 0 as xkb_mod_mask_t,
                        mods: 0 as xkb_mod_mask_t,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_15 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1892 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            ctrl,
            ctrl,
            false,
            0 as int32_t,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, ctrl, ctrl, false, 0, 0, 0, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1897 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_16: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 1 as int32_t,
                        locked_group: 2 as int32_t,
                        group: 3 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: ctrl,
                        locked_mods: 0 as xkb_mod_mask_t,
                        mods: ctrl,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_16 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1916 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let controls: xkb_keyboard_control_flags = XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS;
        if xkb_machine_update_enabled_controls(sm, events, controls, XKB_KEYBOARD_CONTROL_NO_FLAGS)
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_enabled_controls(sm, events, controls, 0) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1926 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_17: [xkb_event; 1] = [xkb_event {
            type_0: 0 as xkb_event_type,
            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
        }];
        if check_events(
            events,
            &raw const expected_17 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1927 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_enabled_controls(sm, events, controls, controls)
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_enabled_controls(sm, events, controls, controls) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1930 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_18: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 1 as int32_t,
                        locked_group: 2 as int32_t,
                        group: 3 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: ctrl,
                        locked_mods: 0 as xkb_mod_mask_t,
                        mods: ctrl,
                        leds: group2,
                        controls: CONTROL_STICKY_KEYS,
                    },
                    changed: XKB_STATE_CONTROLS,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_18 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1950 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_enabled_controls(sm, events, controls, controls)
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_enabled_controls(sm, events, controls, controls) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1953 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_19: [xkb_event; 1] = [xkb_event {
            type_0: 0 as xkb_event_type,
            c2rust_unnamed: C2Rust_Unnamed_13 { keycode: 0 },
        }];
        if check_events(
            events,
            &raw const expected_19 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1954 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_enabled_controls(sm, events, controls, XKB_KEYBOARD_CONTROL_NO_FLAGS)
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_enabled_controls(sm, events, controls, 0) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1957 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_20: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 0 as int32_t,
                        locked_group: 0 as int32_t,
                        group: 0 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: 0 as xkb_mod_mask_t,
                        locked_mods: 0 as xkb_mod_mask_t,
                        mods: 0 as xkb_mod_mask_t,
                        leds: 0 as xkb_led_mask_t,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LATCHED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_LATCHED as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                        | XKB_STATE_CONTROLS as ::core::ffi::c_int
                        | XKB_STATE_LEDS as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_20 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1980 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_enabled_controls(sm, events, controls, XKB_KEYBOARD_CONTROL_NO_FLAGS)
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_enabled_controls(sm, events, controls, 0) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1982 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl,
            ctrl,
            true,
            3 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, ctrl, ctrl, true, 3) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                1991 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_21: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 0 as int32_t,
                        locked_group: 3 as int32_t,
                        group: 3 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: 0 as xkb_mod_mask_t,
                        locked_mods: ctrl,
                        mods: ctrl,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                        | XKB_STATE_LEDS as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_21 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2012 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (46 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_C + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2015 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_22: [xkb_event; 5] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: 0 as xkb_mod_mask_t,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (133 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 3 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_22 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 5]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2088 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_unref(sm);
        if xkb_machine_options_remap_mods(options, ctrl | alt, level3) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, ctrl | alt, level3)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2097 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        sm = xkb_machine_new(keymap, options);
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2100 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl,
            ctrl,
            true,
            3 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, ctrl, ctrl, true, 3) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2104 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2107 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_23: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 3 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_23 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2148 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (46 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_C + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2151 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_24: [xkb_event; 5] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: 0 as xkb_mod_mask_t,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (133 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: -3 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 3 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl,
                            mods: ctrl,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_24 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 5]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2224 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            alt,
            alt,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, alt, alt, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2228 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_25: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0 as int32_t,
                        latched_group: 0 as int32_t,
                        locked_group: 3 as int32_t,
                        group: 3 as xkb_layout_index_t,
                        base_mods: 0,
                        latched_mods: 0 as xkb_mod_mask_t,
                        locked_mods: ctrl | alt,
                        mods: ctrl | alt,
                        leds: group2,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_25 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2248 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2251 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_26: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 3 as xkb_layout_index_t,
                            base_mods: level3,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: level3,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 3 as int32_t,
                            group: 3 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl | alt,
                            mods: ctrl | alt,
                            leds: group2,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_26 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2294 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            true,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, 0, 0, true, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2298 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Q + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2301 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_27: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level3,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: 0 as xkb_mod_mask_t,
                            mods: level3,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl | alt,
                            mods: ctrl | alt,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_27 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2344 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (46 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_C + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2347 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_28: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: alt,
                            mods: alt,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (133 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0 as int32_t,
                            latched_group: 0 as int32_t,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0,
                            latched_mods: 0 as xkb_mod_mask_t,
                            locked_mods: ctrl | alt,
                            mods: ctrl | alt,
                            leds: 0 as xkb_led_mask_t,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_28 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2386 as ::core::ffi::c_uint,
                b"void test_shortcuts_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_unref(sm);
        xkb_events_destroy(events);
        xkb_machine_options_destroy(options);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_overlays(mut context: *mut xkb_context) {
    unsafe {
        static mut controls_tests: [C2Rust_Unnamed_23; 10] = [
            C2Rust_Unnamed_23 {
                controls: 0 as xkb_action_controls,
                overlays: 0 as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_BELL,
                overlays: 0 as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_OVERLAY1,
                overlays: 0x1 as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: (CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_action_controls,
                overlays: 0x3 as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_OVERLAY3,
                overlays: ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                    as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_OVERLAY8,
                overlays: ((1 as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int)
                    as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_ALL,
                overlays: 0xff as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_ALL_V1,
                overlays: 0x3 as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_ALL_BOOLEAN,
                overlays: 0xff as xkb_overlay_mask_t,
            },
            C2Rust_Unnamed_23 {
                controls: CONTROL_ALL_BOOLEAN_V1,
                overlays: 0x3 as xkb_overlay_mask_t,
            },
        ];
        let mut t: size_t = 0 as size_t;
        while t
            < (::core::mem::size_of::<[C2Rust_Unnamed_23; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_23>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: controls #%zu ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_overlays\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            let __cond: bool = controls_tests[t as usize].overlays as ::core::ffi::c_int
                == (controls_tests[t as usize].controls as ::core::ffi::c_uint
                    >> 1 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_uint)
                        << (::core::mem::size_of::<xkb_overlay_mask_t>() as usize)
                            .wrapping_mul(8 as usize))
                    .wrapping_sub(1 as ::core::ffi::c_uint)) as uint8_t
                    as ::core::ffi::c_int;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: . Expected 0x%02x, got: 0x%02x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    controls_tests[t as usize].overlays as ::core::ffi::c_int,
                    (controls_tests[t as usize].controls as ::core::ffi::c_uint
                        >> 1 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_uint)
                            << (::core::mem::size_of::<xkb_overlay_mask_t>() as usize)
                                .wrapping_mul(8 as usize))
                        .wrapping_sub(1 as ::core::ffi::c_uint)) as uint8_t
                        as ::core::ffi::c_int,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2418 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            t = t.wrapping_add(1);
        }
        let keymap: *mut xkb_keymap = test_compile_file(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"keymaps/overlays-v2-2.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2425 as ::core::ffi::c_uint,
                b"void test_overlays(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let sm: *mut xkb_machine =
            xkb_machine_new(keymap, ::core::ptr::null::<xkb_machine_options>()) as *mut xkb_machine;
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2428 as ::core::ffi::c_uint,
                b"void test_overlays(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut events: *mut xkb_events = xkb_events_new_batch(context, XKB_EVENTS_NO_FLAGS);
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2431 as ::core::ffi::c_uint,
                b"void test_overlays(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        static mut keycode_tests: [C2Rust_Unnamed_22; 44] = [
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY8,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY8,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: 0 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY2,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY2,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY2,
                kc: KEY_LEFT as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY8 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_F10 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY8 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_F10 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY8 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_F10 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY8 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_F10 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY8 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY8 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_F1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_F1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: 0 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: (XKB_KEYBOARD_CONTROL_OVERLAY1 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY2 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY3 as ::core::ffi::c_int
                    | XKB_KEYBOARD_CONTROL_OVERLAY4 as ::core::ffi::c_int)
                    as xkb_keyboard_control_flags,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_J as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY1,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY2,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_DOWN,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY2,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
            C2Rust_Unnamed_22 {
                controls: XKB_KEYBOARD_CONTROL_OVERLAY2,
                kc: KEY_KP1 as xkb_keycode_t,
                direction: XKB_KEY_UP,
            },
        ];
        let mut t_0: size_t = 0 as size_t;
        while t_0
            < (::core::mem::size_of::<[C2Rust_Unnamed_22; 44]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_22>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: keycodes #%zu ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_overlays\0".as_ptr() as *const ::core::ffi::c_char,
                t_0,
            );
            if xkb_machine_update_enabled_controls(
                sm,
                events,
                65535 as xkb_keyboard_control_flags,
                keycode_tests[t_0 as usize].controls,
            ) == 0
            {
            } else {
                __assert_fail(
                    b"!xkb_machine_update_enabled_controls( sm, events, 0xffff, keycode_tests[t].controls )\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2520 as ::core::ffi::c_uint,
                    b"void test_overlays(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if !(keycode_tests[t_0 as usize].kc == 0) {
                if xkb_machine_process_key(
                    sm,
                    (8 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as xkb_keycode_t,
                    keycode_tests[t_0 as usize].direction,
                    events,
                ) as u64
                    == 0
                {
                } else {
                    __assert_fail(
                        b"!xkb_machine_process_key( sm, EVDEV_OFFSET + KEY_J, keycode_tests[t].direction, events )\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/server-state.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2527 as ::core::ffi::c_uint,
                        b"void test_overlays(struct xkb_context *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                let mut event: *const xkb_event = ::core::ptr::null::<xkb_event>();
                loop {
                    event = xkb_events_next(events);
                    if event.is_null() {
                        break;
                    }
                    match xkb_event_get_type(event) as ::core::ffi::c_uint {
                        1 | 3 => {
                            let __cond_0: bool = (8 as xkb_keycode_t)
                                .wrapping_add(keycode_tests[t_0 as usize].kc)
                                == xkb_event_get_keycode(event);
                            if !__cond_0 {
                                fprintf(
                                    stderr,
                                    b"Assertion failure: keycode. Expected %u, got: %u\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (8 as xkb_keycode_t)
                                        .wrapping_add(keycode_tests[t_0 as usize].kc),
                                    xkb_event_get_keycode(event),
                                );
                                if __cond_0 as ::core::ffi::c_int != 0 {
                                } else {
                                    __assert_fail(
                                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"../test/server-state.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2534 as ::core::ffi::c_uint,
                                        b"void test_overlays(struct xkb_context *)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }
            t_0 = t_0.wrapping_add(1);
        }
        xkb_events_destroy(events);
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
    }
}
unsafe extern "C" fn test_modifiers_tweak(mut context: *mut xkb_context) {
    unsafe {
        let keymap: *mut xkb_keymap = test_compile_rules(
            context,
            XKB_KEYMAP_FORMAT_TEXT_V2,
            b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            b"us,de\0".as_ptr() as *const ::core::ffi::c_char,
            b",T3\0".as_ptr() as *const ::core::ffi::c_char,
            b"grp:menu_toggle,grp:alt_caps_toggle,terminate:ctrl_alt_bksp,ctrl:copy\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as *mut xkb_keymap;
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2555 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let shift: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_MOD_NAME_SHIFT.as_ptr()) as xkb_mod_mask_t;
        let ctrl: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_MOD_NAME_CTRL.as_ptr()) as xkb_mod_mask_t;
        let alt: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_ALT.as_ptr()) as xkb_mod_mask_t;
        let super_0: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_SUPER.as_ptr()) as xkb_mod_mask_t;
        let scroll: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_SCROLL.as_ptr()) as xkb_mod_mask_t;
        let level3: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_LEVEL3.as_ptr()) as xkb_mod_mask_t;
        let level5: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_LEVEL5.as_ptr()) as xkb_mod_mask_t;
        let num: xkb_mod_mask_t =
            _xkb_keymap_mod_get_mask(keymap, XKB_VMOD_NAME_NUM.as_ptr()) as xkb_mod_mask_t;
        let options: *mut xkb_machine_options =
            xkb_machine_options_new(context) as *mut xkb_machine_options;
        if !options.is_null() {
        } else {
            __assert_fail(
                b"options\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2567 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, 0 as xkb_mod_mask_t, 0 as xkb_mod_mask_t) as u64
            == 0
        {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, 0, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2569 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, 0 as xkb_mod_mask_t, level3)
            as ::core::ffi::c_int
            == XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_options_remap_mods(options, 0, level3) == XKB_ERROR_UNSUPPORTED_MODIFIER_MASK\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2571 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, scroll, alt) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, scroll, alt)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2572 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, super_0, level3) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, super, level3)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2573 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, alt, level5) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, alt, level5)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2574 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, ctrl | alt, level3) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, ctrl | alt, level3)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2575 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, ctrl, shift) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, ctrl, shift)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2577 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_options_remap_mods(options, ctrl, 0 as xkb_mod_mask_t) as u64 == 0 {
        } else {
            __assert_fail(
                b"!xkb_machine_options_remap_mods(options, ctrl, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2578 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let sm: *mut xkb_machine = xkb_machine_new(keymap, options) as *mut xkb_machine;
        if !sm.is_null() {
        } else {
            __assert_fail(
                b"sm\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2581 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_machine_options_destroy(options);
        let events: *mut xkb_events =
            xkb_events_new_batch(context, XKB_EVENTS_NO_FLAGS) as *mut xkb_events;
        if !events.is_null() {
        } else {
            __assert_fail(
                b"events\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2586 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if test_key_seq2(
            keymap,
            sm,
            events,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x63 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1008ff57 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x63 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1008ff57 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            14 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfed5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x79 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            46 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1008ff57 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            127 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe08 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xfe59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            REPEAT as ::core::ffi::c_int,
            0xfe59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xfe59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x1000000 as ::core::ffi::c_int + 0x27c as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe6d as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            125 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffeb as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            DOWN as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfe59 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            14 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0xfed5 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            56 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe9 as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            21 as ::core::ffi::c_int,
            BOTH as ::core::ffi::c_int,
            0x7a as ::core::ffi::c_int,
            NEXT as ::core::ffi::c_int,
            29 as ::core::ffi::c_int,
            UP as ::core::ffi::c_int,
            0xffe3 as ::core::ffi::c_int,
            FINISH as ::core::ffi::c_int,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_key_seq2( keymap, sm, events, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_C , BOTH, XKB_KEY_c , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_C , BOTH, XKB_KEY_XF86Copy , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTMETA, DOWN, XKB_KEY_Super_L , NEXT, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_C , BOTH, XKB_KEY_c , NEXT, KEY_LEFTMETA, UP , XKB_KEY_Super_L , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_LEFTMETA, DOWN, XKB_KEY_Super_L , NEXT, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_LEFTMETA, UP , XKB_KEY_Super_L , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_C , BOTH, XKB_KEY_XF86Copy , NEXT, KEY_BACKSPACE,BOTH, XKB_KEY_Terminate_Server, NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_Y , BOTH, XKB_KEY_y , NEXT, KEY_C , BOTH, XKB_KEY_XF86Copy , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_COMPOSE , BOTH, XKB_KEY_ISO_Next_Group, NEXT, KEY_Y , BOTH, XKB_KEY_z , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Y , BOTH, XKB_KEY_z , NEXT, KEY_Y , DOWN, XKB_KEY_z , NEXT, KEY_Y , REPEAT, XKB_KEY_z , NEXT, KEY_Y , UP, XKB_KEY_z , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , NEXT, KEY_LEFTMETA, DOWN, XKB_KEY_Super_L , NEXT, KEY_Y , BOTH, XKB_KEY_dead_doubleacute, NEXT, KEY_Y , DOWN, XKB_KEY_dead_doubleacute, NEXT, KEY_Y , REPEAT, XKB_KEY_dead_doubleacute, NEXT, KEY_Y , UP, XKB_KEY_dead_doubleacute, NEXT, KEY_LEFTMETA, UP , XKB_KEY_Super_L , NEXT, KEY_LEFTALT , DOWN, XKB_KEY_Alt_L , NEXT, KEY_Y , BOTH, U(0x027C) , NEXT, KEY_LEFTMETA, DOWN, XKB_KEY_Super_L , NEXT, KEY_Y , BOTH, XKB_KEY_dead_invertedbreve, NEXT, KEY_LEFTMETA, UP , XKB_KEY_Super_L , NEXT, KEY_LEFTCTRL, DOWN, XKB_KEY_Control_L , NEXT, KEY_Y , BOTH, XKB_KEY_dead_doubleacute, NEXT, KEY_BACKSPACE,BOTH, XKB_KEY_Terminate_Server, NEXT, KEY_LEFTALT , UP , XKB_KEY_Alt_L , NEXT, KEY_Y , BOTH, XKB_KEY_z , NEXT, KEY_LEFTCTRL, UP , XKB_KEY_Control_L , FINISH )\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2645 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let num_led: xkb_led_mask_t =
            (1 as xkb_led_mask_t) << xkb_keymap_led_get_index(keymap, XKB_LED_NAME_NUM.as_ptr());
        let scroll_led: xkb_led_mask_t =
            (1 as xkb_led_mask_t) << xkb_keymap_led_get_index(keymap, XKB_LED_NAME_SCROLL.as_ptr());
        let group2_led: xkb_led_mask_t = (1 as xkb_led_mask_t)
            << xkb_keymap_led_get_index(
                keymap,
                b"Group 2\0".as_ptr() as *const ::core::ffi::c_char,
            );
        if xkb_machine_process_key(
            sm,
            (56 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_LEFTALT + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2657 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected: [xkb_event; 2] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (56 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: alt,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 2]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2677 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Y + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2680 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_0: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: level5,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: alt,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_0 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2713 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_REPEATED,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Y + EVDEV_OFFSET, XKB_KEY_REPEATED, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2716 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_1: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: level5,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_REPEATED,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: alt,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_1 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2749 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Y + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2752 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_2: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: level5,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: 0,
                            mods: alt,
                            leds: group2_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_2 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2785 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl | num,
            ctrl | num,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, ctrl | num, ctrl | num, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2790 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_3: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0,
                        latched_group: 0,
                        locked_group: 1 as int32_t,
                        group: 1 as xkb_layout_index_t,
                        base_mods: alt,
                        latched_mods: 0,
                        locked_mods: ctrl | num,
                        mods: ctrl | alt | num,
                        leds: group2_led | num_led,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                        | XKB_STATE_LEDS as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_3 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2808 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Y + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2811 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_4: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level3,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: level3 | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | alt | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_4 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2848 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_Y + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2851 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_5: [xkb_event; 3] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level3,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: level3 | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (21 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | alt | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_5 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 3]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2889 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (14 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_BACKSPACE + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2893 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_6: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_KEY_DOWN,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                keycode: (14 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            },
        }];
        if check_events(
            events,
            &raw const expected_6 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2900 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (56 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_LEFTALT + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2903 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_7: [xkb_event; 4] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: level3,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: level3 | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (56 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | alt | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_7 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 4]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2954 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl | scroll,
            scroll,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, ctrl | scroll, scroll, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2959 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_8: [xkb_event; 1] = [xkb_event {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            c2rust_unnamed: C2Rust_Unnamed_13 {
                components: C2Rust_Unnamed_14 {
                    components: state_components {
                        base_group: 0,
                        latched_group: 0,
                        locked_group: 1 as int32_t,
                        group: 1 as xkb_layout_index_t,
                        base_mods: 0 as xkb_mod_mask_t,
                        latched_mods: 0,
                        locked_mods: num | scroll,
                        mods: num | scroll,
                        leds: group2_led | num_led | scroll_led,
                        controls: 0 as xkb_action_controls,
                    },
                    changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                        | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                        | XKB_STATE_LEDS as ::core::ffi::c_int)
                        as xkb_state_component,
                },
            },
        }];
        if check_events(
            events,
            &raw const expected_8 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 1]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2977 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (58 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_CAPSLOCK + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                2981 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_9: [xkb_event; 4] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: alt,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: alt | num,
                            leds: group2_led | num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (58 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 1 as int32_t,
                            group: 1 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0,
                            locked_mods: num | scroll,
                            mods: num | scroll,
                            leds: group2_led | num_led | scroll_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0,
                            locked_mods: num | scroll,
                            mods: num | scroll,
                            leds: num_led | scroll_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_LAYOUT_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_LAYOUT_EFFECTIVE as ::core::ffi::c_int
                            | XKB_STATE_LEDS as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_9 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 4]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3033 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_update_latched_locked(
            sm,
            events,
            0 as xkb_mod_mask_t,
            0 as xkb_mod_mask_t,
            false,
            0 as int32_t,
            ctrl | alt | scroll,
            ctrl | alt,
            false,
            0 as int32_t,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_update_latched_locked(sm, events, 0, 0, false, 0, ctrl | alt | scroll, ctrl | alt, false, 0) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3038 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (46 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_DOWN,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_C + EVDEV_OFFSET, XKB_KEY_DOWN, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3042 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_10: [xkb_event; 5] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_DOWN,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (133 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0,
                            locked_mods: ctrl | alt | num,
                            mods: ctrl | alt | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_10 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 5]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3107 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (46 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_REPEATED,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_C + EVDEV_OFFSET, XKB_KEY_REPEATED, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3110 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_11: [xkb_event; 5] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_REPEATED,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (133 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0,
                            locked_mods: ctrl | alt | num,
                            mods: ctrl | alt | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_11 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 5]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3175 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if xkb_machine_process_key(
            sm,
            (46 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
            XKB_KEY_UP,
            events,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"xkb_machine_process_key(sm, KEY_C + EVDEV_OFFSET, XKB_KEY_UP, events) == 0\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3178 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let expected_12: [xkb_event; 5] = [
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: num,
                            mods: level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_KEY_UP,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    keycode: (133 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as xkb_keycode_t,
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: level5,
                            latched_mods: 0,
                            locked_mods: ctrl | num,
                            mods: ctrl | level5 | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
            xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                c2rust_unnamed: C2Rust_Unnamed_13 {
                    components: C2Rust_Unnamed_14 {
                        components: state_components {
                            base_group: 0,
                            latched_group: 0,
                            locked_group: 0 as int32_t,
                            group: 0 as xkb_layout_index_t,
                            base_mods: 0 as xkb_mod_mask_t,
                            latched_mods: 0,
                            locked_mods: ctrl | alt | num,
                            mods: ctrl | alt | num,
                            leds: num_led,
                            controls: 0 as xkb_action_controls,
                        },
                        changed: (XKB_STATE_MODS_DEPRESSED as ::core::ffi::c_int
                            | XKB_STATE_MODS_LOCKED as ::core::ffi::c_int
                            | XKB_STATE_MODS_EFFECTIVE as ::core::ffi::c_int)
                            as xkb_state_component,
                    },
                },
            },
        ];
        if check_events(
            events,
            &raw const expected_12 as *const xkb_event,
            (::core::mem::size_of::<[xkb_event; 5]>() as size_t)
                .wrapping_div(::core::mem::size_of::<xkb_event>() as size_t),
        ) as ::core::ffi::c_int
            != 0
        {
        } else {
            __assert_fail(
                b"check_events((events), expected, ARRAY_SIZE(expected))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3244 as ::core::ffi::c_uint,
                b"void test_modifiers_tweak(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_events_destroy(events);
        xkb_machine_unref(sm);
        xkb_keymap_unref(keymap);
    }
}
unsafe fn main_0() -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut context: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !context.is_null() {
        } else {
            __assert_fail(
                b"context\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3257 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_context_unref(::core::ptr::null_mut::<xkb_context>());
        xkb_keymap_unref(::core::ptr::null_mut::<xkb_keymap>());
        xkb_state_unref(::core::ptr::null_mut::<xkb_state>());
        xkb_machine_unref(::core::ptr::null_mut::<xkb_machine>());
        xkb_machine_options_destroy(::core::ptr::null_mut::<xkb_machine_options>());
        xkb_events_destroy(::core::ptr::null_mut::<xkb_events>());
        test_machine_options(context);
        test_initial_derived_values(context);
        if xkb_events_new_batch(context, 4294967295 as xkb_events_flags).is_null() {
        } else {
            __assert_fail(
                b"!xkb_events_new_batch(context, -1)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/server-state.c\0".as_ptr() as *const ::core::ffi::c_char,
                3270 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        test_state_update(context);
        test_group_wrap(context);
        test_sticky_keys(context);
        test_redirect_key(context);
        test_overlays(context);
        test_modifiers_tweak(context);
        test_shortcuts_tweak(context);
        xkb_context_unref(context);
        return EXIT_SUCCESS;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
