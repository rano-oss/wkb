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

    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t)
            -> *const ::core::ffi::c_char;
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
    extern "C" {
        pub fn action_equal(a: *const xkb_action, b: *const xkb_action) -> bool;
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
}
pub mod keymap_compare_h {
    pub type xkb_keymap_compare_property = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_CMP_POSSIBLY_DROPPED: xkb_keymap_compare_property = 4;
    pub const XKB_KEYMAP_CMP_ALL: xkb_keymap_compare_property = 31;
    pub const XKB_KEYMAP_CMP_SYMBOLS: xkb_keymap_compare_property = 16;
    pub const XKB_KEYMAP_CMP_KEYCODES: xkb_keymap_compare_property = 8;
    pub const XKB_KEYMAP_CMP_TYPES: xkb_keymap_compare_property = 4;
    pub const XKB_KEYMAP_CMP_LEDS: xkb_keymap_compare_property = 2;
    pub const XKB_KEYMAP_CMP_MODS: xkb_keymap_compare_property = 1;
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
    use super::assert_h::__assert_fail;
    use super::string_h::strcmp;
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
pub mod string_h {
    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod stdbool_h {}
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::keymap_compare_h::{
    xkb_keymap_compare_property, XKB_KEYMAP_CMP_ALL, XKB_KEYMAP_CMP_KEYCODES, XKB_KEYMAP_CMP_LEDS,
    XKB_KEYMAP_CMP_MODS, XKB_KEYMAP_CMP_POSSIBLY_DROPPED, XKB_KEYMAP_CMP_SYMBOLS,
    XKB_KEYMAP_CMP_TYPES,
};
pub use self::keymap_h::{
    action_equal, mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
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
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::utils_h::{streq, streq_null};
pub use self::xkbcommon_h::{
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t, xkb_layout_index_t,
    xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t,
    xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
unsafe extern "C" fn keymap_compare_mods(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let mod_max: xkb_mod_index_t = if (*keymap1).mods.num_mods < (*keymap2).mods.num_mods {
            (*keymap1).mods.num_mods
        } else {
            (*keymap2).mods.num_mods
        };
        let mut mod_0: xkb_mod_index_t = 0 as xkb_mod_index_t;
        while mod_0 < mod_max {
            let mod1: *const xkb_mod = (&raw const (*keymap1).mods.mods as *const xkb_mod)
                .offset(mod_0 as isize) as *const xkb_mod;
            let mod2: *const xkb_mod = (&raw const (*keymap2).mods.mods as *const xkb_mod)
                .offset(mod_0 as isize) as *const xkb_mod;
            let name1: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap1).ctx, (*mod1).name) as *const ::core::ffi::c_char;
            let name2: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap2).ctx, (*mod2).name) as *const ::core::ffi::c_char;
            if !streq_null(name1, name2) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Modifier #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mod_0,
                    name1,
                    name2,
                );
                identical = false;
            }
            if (*mod1).type_0 as ::core::ffi::c_uint != (*mod2).type_0 as ::core::ffi::c_uint {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Modifier #%u types do not match: %d != %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mod_0,
                    (*mod1).type_0 as ::core::ffi::c_uint,
                    (*mod2).type_0 as ::core::ffi::c_uint,
                );
                identical = false;
            }
            if (*mod1).mapping != (*mod2).mapping {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Modifier #%u mappings do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mod_0,
                    (*mod1).mapping,
                    (*mod2).mapping,
                );
                identical = false;
            }
            mod_0 = mod_0.wrapping_add(1);
        }
        if (*keymap1).mods.num_mods != (*keymap2).mods.num_mods {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Modifiers counts do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*keymap1).mods.num_mods,
                (*keymap2).mods.num_mods,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe extern "C" fn keymap_compare_keycodes(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        if (*keymap1).num_keys != (*keymap2).num_keys {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Keycodes counts do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*keymap1).num_keys,
                (*keymap2).num_keys,
            );
            identical = false;
        }
        if (*keymap1).min_key_code != (*keymap2).min_key_code {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Min keycodes do not match: %u != %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*keymap1).min_key_code,
                (*keymap2).min_key_code,
            );
            identical = false;
        }
        if (*keymap1).max_key_code != (*keymap2).max_key_code {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Low keycodes counts do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*keymap1).num_keys_low,
                (*keymap2).num_keys_low,
            );
            identical = false;
        }
        if (*keymap1).max_key_code != (*keymap2).max_key_code {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Max keycodes do not match: %u != %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*keymap1).min_key_code,
                (*keymap2).min_key_code,
            );
            identical = false;
        }
        let k_max: xkb_keycode_t = if (*keymap1).num_keys < (*keymap2).num_keys {
            (*keymap1).num_keys
        } else {
            (*keymap2).num_keys
        };
        let mut k: xkb_keycode_t = 0 as xkb_keycode_t;
        while k < k_max {
            let key1: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            let key2: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            if (*key1).keycode != (*key2).keycode {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key #%u keycodes do not match: %x != %x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    k,
                    (*key1).keycode,
                    (*key2).keycode,
                );
                identical = false;
            } else {
                let kc: xkb_keycode_t = (*key1).keycode;
                let name1: *const ::core::ffi::c_char =
                    xkb_atom_text((*keymap1).ctx, (*key1).name) as *const ::core::ffi::c_char;
                let name2: *const ::core::ffi::c_char =
                    xkb_atom_text((*keymap2).ctx, (*key2).name) as *const ::core::ffi::c_char;
                if !streq_null(name1, name2) {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key 0x%x names do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc,
                        name1,
                        name2,
                    );
                    identical = false;
                }
            }
            k = k.wrapping_add(1);
        }
        let a_max: darray_size_t = if (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
            < (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        {
            (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        } else {
            (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        };
        let mut a: darray_size_t = 0 as darray_size_t;
        while a < a_max {
            let entry1: *const xkb_key_alias = (*keymap1)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .key_aliases
                .offset(a as isize)
                as *mut xkb_key_alias;
            let entry2: *const xkb_key_alias = (*keymap2)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .key_aliases
                .offset(a as isize)
                as *mut xkb_key_alias;
            let alias1: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap1).ctx, (*entry1).alias) as *const ::core::ffi::c_char;
            let alias2: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap2).ctx, (*entry2).alias) as *const ::core::ffi::c_char;
            if !streq_null(alias1, alias2) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Alias #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    a,
                    alias1,
                    alias2,
                );
                identical = false;
            }
            let real1: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap1).ctx, (*entry1).real) as *const ::core::ffi::c_char;
            let real2: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap2).ctx, (*entry2).real) as *const ::core::ffi::c_char;
            if !streq_null(real1, real2) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Alias #%u \"%s\" target do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    a,
                    alias1,
                    real1,
                    real2,
                );
                identical = false;
            }
            a = a.wrapping_add(1);
        }
        if (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
            != (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases
        {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Aliases count do not match: %u != %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*keymap1).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases,
                (*keymap2).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe extern "C" fn keymap_compare_leds(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let mut led_max: xkb_led_index_t = if (*keymap1).num_leds < (*keymap2).num_leds {
            (*keymap1).num_leds
        } else {
            (*keymap2).num_leds
        };
        let mut led: xkb_led_index_t = 0 as xkb_led_index_t;
        while led < led_max {
            let led1: *const xkb_led = (&raw const (*keymap1).leds as *const xkb_led)
                .offset(led as isize) as *const xkb_led;
            let led2: *const xkb_led = (&raw const (*keymap2).leds as *const xkb_led)
                .offset(led as isize) as *const xkb_led;
            let name1: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap1).ctx, (*led1).name) as *const ::core::ffi::c_char;
            let name2: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap2).ctx, (*led2).name) as *const ::core::ffi::c_char;
            if !streq_null(name1, name2) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"LED #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    led,
                    name1,
                    name2,
                );
                identical = false;
            }
            if (*led1).which_groups() as ::core::ffi::c_int
                != (*led2).which_groups() as ::core::ffi::c_int
            {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"LED #%u \"%s\" `which_groups` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    led,
                    name1,
                    (*led1).which_groups() as ::core::ffi::c_int,
                    (*led2).which_groups() as ::core::ffi::c_int,
                );
                identical = false;
            }
            if (*led1).groups != (*led2).groups {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"LED #%u \"%s\" `groups` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    led,
                    name1,
                    (*led1).groups,
                    (*led2).groups,
                );
                identical = false;
            }
            if (*led1).which_mods as ::core::ffi::c_uint
                != (*led2).which_mods as ::core::ffi::c_uint
            {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"LED #%u \"%s\" `which_mods` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    led,
                    name1,
                    (*led1).which_mods as ::core::ffi::c_uint,
                    (*led2).which_mods as ::core::ffi::c_uint,
                );
                identical = false;
            }
            if (*led1).mods.mods != (*led2).mods.mods {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"LED #%u \"%s\" `mods` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    led,
                    name1,
                    (*led1).mods.mods,
                    (*led2).mods.mods,
                );
                identical = false;
            }
            if (*led1).ctrls as ::core::ffi::c_uint != (*led2).ctrls as ::core::ffi::c_uint {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"LED #%u \"%s\" `ctrls` do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    led,
                    name1,
                    (*led1).ctrls as ::core::ffi::c_uint,
                    (*led2).ctrls as ::core::ffi::c_uint,
                );
                identical = false;
            }
            led = led.wrapping_add(1);
        }
        if (*keymap1).num_leds != (*keymap2).num_leds {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"LEDs count do not match: %u != %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*keymap1).num_leds,
                (*keymap2).num_leds,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe extern "C" fn compare_types(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
    mut type1: *const xkb_key_type,
    mut type2: *const xkb_key_type,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let name1: *const ::core::ffi::c_char =
            xkb_atom_text((*keymap1).ctx, (*type1).name) as *const ::core::ffi::c_char;
        let name2: *const ::core::ffi::c_char =
            xkb_atom_text((*keymap2).ctx, (*type2).name) as *const ::core::ffi::c_char;
        if !streq_null(name1, name2) {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key type names do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name1,
                name2,
            );
            identical = false;
        }
        if (*type1).mods.mods != (*type2).mods.mods {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key type \"%s\" mods do not match: 0x%x != 0x%x\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name1,
                (*type1).mods.mods,
                (*type2).mods.mods,
            );
            return false;
        }
        if (*type1).num_levels != (*type2).num_levels {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key type \"%s\" levels count do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name1,
                (*type1).num_levels,
                (*type2).num_levels,
            );
            return false;
        }
        if (*type1).num_level_names != (*type2).num_level_names {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key type \"%s\" level names count do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name1,
                (*type1).num_level_names,
                (*type2).num_level_names,
            );
            identical = false;
        } else {
            let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
            while l < (*type1).num_level_names {
                let lname1: *const ::core::ffi::c_char =
                    xkb_atom_text((*keymap1).ctx, *(*type1).level_names.offset(l as isize))
                        as *const ::core::ffi::c_char;
                let lname2: *const ::core::ffi::c_char =
                    xkb_atom_text((*keymap2).ctx, *(*type2).level_names.offset(l as isize))
                        as *const ::core::ffi::c_char;
                if !streq_null(lname1, lname2) {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key type \"%s\" level #%u names do not match: \"%s\" != \"%s\"\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        name1,
                        l,
                        lname1,
                        lname2,
                    );
                    identical = false;
                }
                l = l.wrapping_add(1);
            }
        }
        if (*type1).num_entries != (*type2).num_entries {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key type \"%s\" entries count do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name1,
                (*type1).num_entries,
                (*type2).num_entries,
            );
            identical = false;
        } else {
            let mut e: darray_size_t = 0 as darray_size_t;
            while e < (*type1).num_entries {
                let entry1: *const xkb_key_type_entry =
                    (*type1).entries.offset(e as isize) as *mut xkb_key_type_entry;
                let entry2: *const xkb_key_type_entry =
                    (*type2).entries.offset(e as isize) as *mut xkb_key_type_entry;
                if (*entry1).level != (*entry2).level {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key type \"%s\" entry #%u levels do not match: %u != %u\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        name1,
                        e,
                        (*entry1).level,
                        (*entry2).level,
                    );
                    identical = false;
                }
                if (*entry1).mods.mods != (*entry2).mods.mods {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key type \"%s\" entry #%u mods do not match: 0x%x != 0x%x\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        name1,
                        e,
                        (*entry1).mods.mods,
                        (*entry2).mods.mods,
                    );
                    identical = false;
                }
                if (*entry1).preserve.mods != (*entry2).preserve.mods {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key type \"%s\" entry #%u preserve do not match: 0x%x != 0x%x\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        name1,
                        e,
                        (*entry1).preserve.mods,
                        (*entry2).preserve.mods,
                    );
                    identical = false;
                }
                e = e.wrapping_add(1);
            }
        }
        return identical;
    }
}
unsafe extern "C" fn keymap_compare_types(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        let t_max: darray_size_t = if (*keymap1).num_types < (*keymap2).num_types {
            (*keymap1).num_types
        } else {
            (*keymap2).num_types
        };
        let mut t: darray_size_t = 0 as darray_size_t;
        while t < t_max {
            identical = compare_types(
                ctx,
                keymap1,
                keymap2,
                (*keymap1).types.offset(t as isize) as *mut xkb_key_type,
                (*keymap2).types.offset(t as isize) as *mut xkb_key_type,
            ) as ::core::ffi::c_int
                != 0
                && identical as ::core::ffi::c_int != 0;
            t = t.wrapping_add(1);
        }
        if (*keymap1).num_types != (*keymap2).num_types {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key types counts do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*keymap1).num_types,
                (*keymap2).num_types,
            );
            identical = false;
        }
        return identical;
    }
}
unsafe extern "C" fn compare_groups(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
    mut kc: xkb_keycode_t,
    mut g: xkb_layout_index_t,
    mut group1: *const xkb_group,
    mut group2: *const xkb_group,
) -> bool {
    unsafe {
        if !compare_types(ctx, keymap1, keymap2, (*group1).type_0, (*group2).type_0) {
            let name1: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap1).ctx, (*(*group1).type_0).name)
                    as *const ::core::ffi::c_char;
            let name2: *const ::core::ffi::c_char =
                xkb_atom_text((*keymap2).ctx, (*(*group2).type_0).name)
                    as *const ::core::ffi::c_char;
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Key 0x%x/group %u types do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                kc,
                g,
                name1,
                name2,
            );
            return false;
        }
        if (*(*group1).type_0).num_levels == (*(*group2).type_0).num_levels {
        } else {
            __assert_fail(
                b"group1->type->num_levels == group2->type->num_levels\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../src/keymap-compare.c\0".as_ptr() as *const ::core::ffi::c_char,
                386 as ::core::ffi::c_uint,
                b"_Bool compare_groups(struct xkb_context *, const struct xkb_keymap *, const struct xkb_keymap *, xkb_keycode_t, xkb_layout_index_t, const struct xkb_group *, const struct xkb_group *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut identical: bool = true;
        let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
        while l < (*(*group1).type_0).num_levels {
            let level1: *const xkb_level = (*group1).levels.offset(l as isize) as *mut xkb_level;
            let level2: *const xkb_level = (*group2).levels.offset(l as isize) as *mut xkb_level;
            if (*level1).num_syms as ::core::ffi::c_int != (*level2).num_syms as ::core::ffi::c_int
            {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key 0x%x/group %u/level %u keysyms count do not match: %u != %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    kc,
                    g,
                    l,
                    (*level1).num_syms as ::core::ffi::c_int,
                    (*level2).num_syms as ::core::ffi::c_int,
                );
                identical = false;
            } else if (*level1).num_syms as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                let mut k: xkb_keysym_count_t = 0 as xkb_keysym_count_t;
                while (k as ::core::ffi::c_int) < (*level1).num_syms as ::core::ffi::c_int {
                    if !(*(*level1).s.syms.offset(k as isize)
                        == *(*level2).s.syms.offset(k as isize))
                    {
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"Key 0x%x/group %u/level %u keysyms #%u do not match: 0x%x != 0x%x\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            kc,
                            g,
                            l,
                            k as ::core::ffi::c_int,
                            *(*level1).s.syms.offset(k as isize),
                            *(*level2).s.syms.offset(k as isize),
                        );
                        identical = false;
                    }
                    k = k.wrapping_add(1);
                }
            } else if (*level1).s.sym != (*level2).s.sym {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key 0x%x/group %u/level %u keysyms do not match: 0x%x != 0x%x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    kc,
                    g,
                    l,
                    (*level1).s.sym,
                    (*level2).s.sym,
                );
                identical = false;
            }
            if (*level1).num_actions as ::core::ffi::c_int
                != (*level2).num_actions as ::core::ffi::c_int
            {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key 0x%x/group %u/level %u actions count do not match: %u != %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    kc,
                    g,
                    l,
                    (*level1).num_actions as ::core::ffi::c_int,
                    (*level2).num_actions as ::core::ffi::c_int,
                );
                identical = false;
            } else if (*level1).num_actions as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                let mut a: xkb_action_count_t = 0 as xkb_action_count_t;
                while (a as ::core::ffi::c_int) < (*level1).num_actions as ::core::ffi::c_int {
                    if !action_equal(
                        (*level1).a.actions.offset(a as isize) as *mut xkb_action,
                        (*level2).a.actions.offset(a as isize) as *mut xkb_action,
                    ) {
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"Key 0x%x/group %u/level %u actions #%u do not match\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            kc,
                            g,
                            l,
                            a as ::core::ffi::c_int,
                        );
                        identical = false;
                    }
                    a = a.wrapping_add(1);
                }
            } else if (*level1).num_actions as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && !action_equal(&raw const (*level1).a.action, &raw const (*level2).a.action)
            {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key 0x%x/group %u/level %u actions do not match\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    kc,
                    g,
                    l,
                );
                identical = false;
            }
            l = l.wrapping_add(1);
        }
        return identical;
    }
}
unsafe extern "C" fn keymap_compare_symbols(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        if (*keymap1).num_groups != (*keymap2).num_groups {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Group counts do not match: %u != %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*keymap1).num_groups,
                (*keymap2).num_groups,
            );
            identical = false;
        }
        if (*keymap1).num_group_names != (*keymap2).num_group_names {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Group name counts do not match: %u != %u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*keymap1).num_group_names,
                (*keymap2).num_group_names,
            );
            identical = false;
        } else {
            let mut g: xkb_layout_index_t = 0 as xkb_layout_index_t;
            while g < (*keymap1).num_group_names {
                let name1: *const ::core::ffi::c_char =
                    xkb_atom_text((*keymap1).ctx, *(*keymap1).group_names.offset(g as isize))
                        as *const ::core::ffi::c_char;
                let name2: *const ::core::ffi::c_char =
                    xkb_atom_text((*keymap2).ctx, *(*keymap2).group_names.offset(g as isize))
                        as *const ::core::ffi::c_char;
                if !streq_null(name1, name2) {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Group #%u names do not match: \"%s\" != \"%s\"\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g,
                        name1,
                        name2,
                    );
                    identical = false;
                }
                g = g.wrapping_add(1);
            }
        }
        let k_max: xkb_keycode_t = if (*keymap1).num_keys < (*keymap2).num_keys {
            (*keymap1).num_keys
        } else {
            (*keymap2).num_keys
        };
        let mut k: xkb_keycode_t = 0 as xkb_keycode_t;
        while k < k_max {
            let key1: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            let key2: *const xkb_key = (*keymap1).keys.offset(k as isize) as *mut xkb_key;
            if (*key1).keycode != (*key2).keycode {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Key #%u keycodes do not match: %x != %x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    k,
                    (*key1).keycode,
                    (*key2).keycode,
                );
                identical = false;
            } else {
                let kc: xkb_keycode_t = (*key1).keycode;
                if (*key1).modmap != (*key2).modmap {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key 0x%x modmap do not match: 0x%x != 0x%x\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc,
                        (*key1).modmap,
                        (*key2).modmap,
                    );
                    identical = false;
                }
                if (*key1).vmodmap != (*key2).vmodmap {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key 0x%x vmodmap do not match: 0x%x != 0x%x\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc,
                        (*key1).vmodmap,
                        (*key2).vmodmap,
                    );
                    identical = false;
                }
                if (*key1).repeats() as ::core::ffi::c_int
                    != (*key2).repeats() as ::core::ffi::c_int
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key 0x%x repeats do not match: %d != %d\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc,
                        (*key1).repeats() as ::core::ffi::c_int,
                        (*key2).repeats() as ::core::ffi::c_int,
                    );
                    identical = false;
                }
                if (*key1).out_of_range_group_policy() as ::core::ffi::c_int
                    != (*key2).out_of_range_group_policy() as ::core::ffi::c_int
                    || (*key1).out_of_range_group_number() as ::core::ffi::c_int
                        != (*key2).out_of_range_group_number() as ::core::ffi::c_int
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key 0x%x out-of-range do not match: %d != %d or %u != %u\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc,
                        (*key1).out_of_range_group_policy() as ::core::ffi::c_int,
                        (*key2).out_of_range_group_policy() as ::core::ffi::c_int,
                        (*key1).out_of_range_group_number() as ::core::ffi::c_int,
                        (*key2).out_of_range_group_number() as ::core::ffi::c_int,
                    );
                    identical = false;
                }
                if (*key1).num_groups() as ::core::ffi::c_int
                    != (*key2).num_groups() as ::core::ffi::c_int
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Key 0x%x groups counts do not match: %u != %u\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        kc,
                        (*key1).num_groups() as ::core::ffi::c_int,
                        (*key2).num_groups() as ::core::ffi::c_int,
                    );
                    identical = false;
                }
                let g_max: xkb_layout_index_t = (if ((*key1).num_groups() as ::core::ffi::c_int)
                    < (*key2).num_groups() as ::core::ffi::c_int
                {
                    (*key1).num_groups() as ::core::ffi::c_int
                } else {
                    (*key2).num_groups() as ::core::ffi::c_int
                }) as xkb_layout_index_t;
                let mut g_0: xkb_layout_index_t = 0 as xkb_layout_index_t;
                while g_0 < g_max {
                    identical = compare_groups(
                        ctx,
                        keymap1,
                        keymap2,
                        kc,
                        g_0,
                        (*key1).groups.offset(g_0 as isize) as *mut xkb_group,
                        (*key2).groups.offset(g_0 as isize) as *mut xkb_group,
                    ) as ::core::ffi::c_int
                        != 0
                        && identical as ::core::ffi::c_int != 0;
                    g_0 = g_0.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        return identical;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_compare(
    mut ctx: *mut xkb_context,
    mut keymap1: *const xkb_keymap,
    mut keymap2: *const xkb_keymap,
    mut properties: xkb_keymap_compare_property,
) -> bool {
    unsafe {
        let mut identical: bool = true;
        if properties as ::core::ffi::c_uint
            & XKB_KEYMAP_CMP_MODS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            identical = keymap_compare_mods(ctx, keymap1, keymap2) as ::core::ffi::c_int != 0
                && identical as ::core::ffi::c_int != 0;
        }
        if properties as ::core::ffi::c_uint
            & XKB_KEYMAP_CMP_TYPES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            identical = keymap_compare_types(ctx, keymap1, keymap2) as ::core::ffi::c_int != 0
                && identical as ::core::ffi::c_int != 0;
        }
        if properties as ::core::ffi::c_uint
            & XKB_KEYMAP_CMP_LEDS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            identical = keymap_compare_leds(ctx, keymap1, keymap2) as ::core::ffi::c_int != 0
                && identical as ::core::ffi::c_int != 0;
        }
        if properties as ::core::ffi::c_uint
            & XKB_KEYMAP_CMP_KEYCODES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            identical = keymap_compare_keycodes(ctx, keymap1, keymap2) as ::core::ffi::c_int != 0
                && identical as ::core::ffi::c_int != 0;
        }
        if properties as ::core::ffi::c_uint
            & XKB_KEYMAP_CMP_SYMBOLS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            identical = keymap_compare_symbols(ctx, keymap1, keymap2) as ::core::ffi::c_int != 0
                && identical as ::core::ffi::c_int != 0;
        }
        return identical;
    }
}
