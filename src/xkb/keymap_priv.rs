pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
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
    pub type i8 = __int8_t;
    pub type i16 = __int16_t;
    pub type i32 = __int32_t;
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
        pub refcnt: i32,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const i8,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: i32,
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
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: i32,
            fmt: *const i8,
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
    pub type darray_size_t = u32;
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
    pub type xkb_log_level = u32;
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
    pub type xkb_layout_out_of_range_policy = u32;
    pub const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: xkb_layout_out_of_range_policy = 2;
    pub const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: xkb_layout_out_of_range_policy = 1;
    pub const XKB_LAYOUT_OUT_OF_RANGE_WRAP: xkb_layout_out_of_range_policy = 0;
    pub type xkb_state_component = u32;
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
    pub type xkb_keymap_format = u32;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    pub type xkb_keymap_compile_flags = u32;
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    pub const XKB_LAYOUT_INVALID: u32 = 0xffffffff as u32;
    pub const XKB_MOD_INVALID: u32 = 0xffffffff as u32;
    use super::context_h::xkb_context;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;
    }
}
pub mod keymap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_keymap {
        pub ctx: *mut xkb_context,
        pub refcnt: i32,
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
    pub type mod_type = u32;
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
    pub type xkb_internal_action_flags = u32;
    pub const INTERNAL_BREAKS_MOD_LATCH: xkb_internal_action_flags = 2;
    pub const INTERNAL_BREAKS_GROUP_LATCH: xkb_internal_action_flags = 1;
    pub type xkb_action_type = u32;
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
    pub type xkb_action_flags = u32;
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
        pub x: i16,
        pub y: i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_switch_screen_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub screen: i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_default_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub value: i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_controls_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub ctrls: xkb_action_controls,
    }
    pub type xkb_action_controls = u32;
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
        pub group: i32,
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
    pub type xkb_match_operation = u32;
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
    pub type xkb_explicit_components = u32;
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
    pub const XKB_MAX_GROUPS: i32 = 32 as i32;
    pub const MOD_REAL_MASK_ALL: xkb_mod_mask_t = 0xff as i32 as xkb_mod_mask_t;
    #[inline]
    pub unsafe fn XkbKeyNumLevels(
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
    use super::stdint_intn_h::{i16, i32, i8};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    use super::xkbcommon_h::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_level_index_t, xkb_mod_index_t, xkb_mod_mask_t, xkb_state_component,
    };
}
pub mod enums_h {
    pub type xkb_enumerations_values = u32;
    pub const XKB_COMPOSE_FEED_RESULT_VALUES: xkb_enumerations_values = 3;
    pub const XKB_COMPOSE_STATUS_VALUES: xkb_enumerations_values = 15;
    pub const XKB_COMPOSE_STATE_FLAGS_VALUES: xkb_enumerations_values = 0;
    pub const XKB_COMPOSE_FORMAT_VALUES: xkb_enumerations_values = 2;
    pub const XKB_COMPOSE_COMPILE_FLAGS_VALUES: xkb_enumerations_values = 0;
    pub const XKB_CONSUMED_MODE_VALUES: xkb_enumerations_values = 3;
    pub const XKB_STATE_MATCH_VALUES: xkb_enumerations_values = 65539;
    pub const XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES: xkb_enumerations_values = 7;
    pub const XKB_KEY_DIRECTION_VALUES: xkb_enumerations_values = 7;
    pub const XKB_A11Y_FLAGS_VALUES: xkb_enumerations_values = 3;
    pub const XKB_EVENTS_FLAGS_VALUES: xkb_enumerations_values = 0;
    pub const XKB_KEYBOARD_CONTROL_FLAGS_VALUES: xkb_enumerations_values = 511;
    pub const XKB_STATE_COMPONENT_VALUES: xkb_enumerations_values = 1023;
    pub const XKB_EVENT_TYPE_VALUES: xkb_enumerations_values = 30;
    pub const XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES: xkb_enumerations_values = 3;
    pub const XKB_KEYMAP_SERIALIZE_FLAGS_VALUES: xkb_enumerations_values = 7;
    pub const XKB_KEYMAP_FORMAT_VALUES: xkb_enumerations_values = 6;
    pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: xkb_enumerations_values = 1;
    pub const XKB_CONTEXT_FLAGS_VALUES: xkb_enumerations_values = 7;
    pub const XKB_KEYSYM_FLAGS_VALUES: xkb_enumerations_values = 1;
    pub const XKB_RMLVO_BUILDER_FLAGS_VALUES: xkb_enumerations_values = 0;
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = i32;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
pub mod string_h {

    extern "C" {
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: usize,
        ) -> i32;
        pub fn strlen(__s: *const i8) -> usize;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    }
}
pub mod stdint_h {
    pub const INT32_MAX: i32 = 2147483647 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub mod xkbcommon_names_h {
    pub const XKB_MOD_NAME_SHIFT: [i8; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"Shift\0") };
    pub const XKB_MOD_NAME_CAPS: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Lock\0") };
    pub const XKB_MOD_NAME_CTRL: [i8; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"Control\0") };
    pub const XKB_MOD_NAME_MOD1: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod1\0") };
    pub const XKB_MOD_NAME_MOD2: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod2\0") };
    pub const XKB_MOD_NAME_MOD3: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod3\0") };
    pub const XKB_MOD_NAME_MOD4: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod4\0") };
    pub const XKB_MOD_NAME_MOD5: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod5\0") };
}
pub use self::__stddef_null_h::NULL;

pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{
    xkb_atom_intern, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
pub use self::enums_h::{
    xkb_enumerations_values, XKB_A11Y_FLAGS_VALUES, XKB_COMPOSE_COMPILE_FLAGS_VALUES,
    XKB_COMPOSE_FEED_RESULT_VALUES, XKB_COMPOSE_FORMAT_VALUES, XKB_COMPOSE_STATE_FLAGS_VALUES,
    XKB_COMPOSE_STATUS_VALUES, XKB_CONSUMED_MODE_VALUES, XKB_CONTEXT_FLAGS_VALUES,
    XKB_EVENTS_FLAGS_VALUES, XKB_EVENT_TYPE_VALUES, XKB_KEYBOARD_CONTROL_FLAGS_VALUES,
    XKB_KEYMAP_COMPILE_FLAGS_VALUES, XKB_KEYMAP_FORMAT_VALUES,
    XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES, XKB_KEYMAP_SERIALIZE_FLAGS_VALUES,
    XKB_KEYSYM_FLAGS_VALUES, XKB_KEY_DIRECTION_VALUES, XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES,
    XKB_RMLVO_BUILDER_FLAGS_VALUES, XKB_STATE_COMPONENT_VALUES, XKB_STATE_MATCH_VALUES,
};
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
    C2Rust_Unnamed_9, KeycodeMatch, XkbKeyNumLevels, _ACTION_TYPE_NUM_ENTRIES,
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
    XKB_MAX_GROUPS,
};
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::INT32_MAX;
pub use self::stdint_intn_h::{i16, i32, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::calloc;
use self::string_h::{memcmp, strlen};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::xkbcommon_h::{
    xkb_context_ref, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
    xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
    xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
    xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE,
    XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_INVALID,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_names_h::{
    XKB_MOD_NAME_CAPS, XKB_MOD_NAME_CTRL, XKB_MOD_NAME_MOD1, XKB_MOD_NAME_MOD2, XKB_MOD_NAME_MOD3,
    XKB_MOD_NAME_MOD4, XKB_MOD_NAME_MOD5, XKB_MOD_NAME_SHIFT,
};
unsafe fn update_builtin_keymap_fields(mut keymap: *mut xkb_keymap) {
    unsafe {
        static mut builtin_mods: [*const i8; 8] = [
            XKB_MOD_NAME_SHIFT.as_ptr(),
            XKB_MOD_NAME_CAPS.as_ptr(),
            XKB_MOD_NAME_CTRL.as_ptr(),
            XKB_MOD_NAME_MOD1.as_ptr(),
            XKB_MOD_NAME_MOD2.as_ptr(),
            XKB_MOD_NAME_MOD3.as_ptr(),
            XKB_MOD_NAME_MOD4.as_ptr(),
            XKB_MOD_NAME_MOD5.as_ptr(),
        ];
        let mut i: xkb_mod_index_t = 0 as xkb_mod_index_t;
        while (i as usize)
            < (::core::mem::size_of::<[*const i8; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const i8>() as usize)
        {
            (*keymap).mods.mods[i as usize].name = xkb_atom_intern(
                (*keymap).ctx,
                builtin_mods[i as usize],
                strlen(builtin_mods[i as usize]),
            );
            (*keymap).mods.mods[i as usize].type_0 = MOD_REAL;
            (*keymap).mods.mods[i as usize].mapping = ((1 as u32) << i) as xkb_mod_mask_t;
            i = i.wrapping_add(1);
        }
        (*keymap).mods.num_mods = (::core::mem::size_of::<[*const i8; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const i8>() as usize)
            as xkb_mod_index_t;
        (*keymap).canonical_state_mask = MOD_REAL_MASK_ALL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_new(
    mut ctx: *mut xkb_context,
    mut func: *const i8,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        static mut XKB_KEYMAP_COMPILE_FLAGS: xkb_keymap_compile_flags =
            XKB_KEYMAP_COMPILE_FLAGS_VALUES as i32 as xkb_keymap_compile_flags;
        if flags as u32 & !(XKB_KEYMAP_COMPILE_FLAGS as u32) != 0 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unrecognized keymap compilation flags: 0x%x\n\0".as_ptr() as *const i8,
                func,
                flags as u32 & !(XKB_KEYMAP_COMPILE_FLAGS as u32),
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let keymap: *mut xkb_keymap =
            calloc(1 as usize, ::core::mem::size_of::<xkb_keymap>() as usize) as *mut xkb_keymap;
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        (*keymap).refcnt = 1 as i32;
        (*keymap).ctx = xkb_context_ref(ctx);
        (*keymap).format = format;
        (*keymap).flags = flags;
        update_builtin_keymap_fields(keymap);
        return keymap;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbEscapeMapName(mut name: *mut i8) {
    unsafe {
        static mut legal: [::core::ffi::c_uchar; 32] = [
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0xa7 as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x83 as i32 as ::core::ffi::c_uchar,
            0xfe as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x87 as i32 as ::core::ffi::c_uchar,
            0xfe as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x7f as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
            0x7f as i32 as ::core::ffi::c_uchar,
            0xff as i32 as ::core::ffi::c_uchar,
        ];
        if name.is_null() {
            return;
        }
        while *name != 0 {
            let mut c: ::core::ffi::c_uchar = *name as ::core::ffi::c_uchar;
            if legal[(c as i32 / 8 as i32) as usize] as u32 & (1 as u32) << c as i32 % 8 as i32 == 0
            {
                *name = '_' as i32 as i8;
            }
            name = name.offset(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbModNameToIndex(
    mut mods: *const xkb_mod_set,
    mut name: xkb_atom_t,
    mut type_0: mod_type,
) -> xkb_mod_index_t {
    unsafe {
        let mut i: xkb_mod_index_t = 0;
        let mut mod_0: *const xkb_mod = ::core::ptr::null::<xkb_mod>();
        i = 0 as xkb_mod_index_t;
        mod_0 = &raw const (*mods).mods as *const xkb_mod;
        while i < (*mods).num_mods {
            if (*mod_0).type_0 as u32 & type_0 as u32 != 0 && name == (*mod_0).name {
                return i;
            }
            i = i.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        return XKB_MOD_INVALID as xkb_mod_index_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbLevelsSameSyms(
    mut a: *const xkb_level,
    mut b: *const xkb_level,
) -> bool {
    unsafe {
        if (*a).num_syms as i32 != (*b).num_syms as i32 {
            return false_0 != 0;
        }
        if (*a).num_syms as i32 <= 1 as i32 {
            return (*a).s.sym == (*b).s.sym;
        }
        return memcmp(
            (*a).s.syms as *const ::core::ffi::c_void,
            (*b).s.syms as *const ::core::ffi::c_void,
            (::core::mem::size_of::<xkb_keysym_t>() as usize).wrapping_mul((*a).num_syms as usize),
        ) == 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn action_equal(mut a: *const xkb_action, mut b: *const xkb_action) -> bool {
    unsafe {
        if (*a).type_0 as u32 != (*b).type_0 as u32 {
            return false_0 != 0;
        }
        match (*a).type_0 as u32 {
            0 | 1 => return true_0 != 0,
            2 | 3 | 4 => {
                return (*a).mods.flags as u32 == (*b).mods.flags as u32
                    && (*a).mods.mods.mask == (*b).mods.mods.mask
                    && (*a).mods.mods.mods == (*b).mods.mods.mods;
            }
            5 | 6 | 7 => {
                return (*a).group.flags as u32 == (*b).group.flags as u32
                    && (*a).group.group == (*b).group.group;
            }
            8 => {
                return (*a).ptr.flags as u32 == (*b).ptr.flags as u32
                    && (*a).ptr.x as i32 == (*b).ptr.x as i32
                    && (*a).ptr.y as i32 == (*b).ptr.y as i32;
            }
            9 | 10 => {
                return (*a).btn.flags as u32 == (*b).btn.flags as u32
                    && (*a).btn.button as i32 == (*b).btn.button as i32
                    && (*a).btn.count as i32 == (*b).btn.count as i32;
            }
            11 => {
                return (*a).dflt.flags as u32 == (*b).dflt.flags as u32
                    && (*a).dflt.value as i32 == (*b).dflt.value as i32;
            }
            12 => return true_0 != 0,
            13 => {
                return (*a).screen.flags as u32 == (*b).screen.flags as u32
                    && (*a).screen.screen as i32 == (*b).screen.screen as i32;
            }
            14 | 15 => {
                return (*a).ctrls.flags as u32 == (*b).ctrls.flags as u32
                    && (*a).ctrls.ctrls as u32 == (*b).ctrls.ctrls as u32;
            }
            16 => {
                return (*a).redirect.keycode == (*b).redirect.keycode
                    && (*a).redirect.affect == (*b).redirect.affect
                    && (*a).redirect.mods == (*b).redirect.mods;
            }
            17 | 18 => return true_0 != 0,
            20 => {
                return (*a).internal.flags as u32 == (*b).internal.flags as u32
                    && (*a).internal.c2rust_unnamed.clear_latched_mods
                        == (*b).internal.c2rust_unnamed.clear_latched_mods;
            }
            _ => {
                return memcmp(
                    &raw const (*a).priv_0.data as *const uint8_t as *const ::core::ffi::c_void,
                    &raw const (*b).priv_0.data as *const uint8_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[uint8_t; 7]>() as usize,
                ) == 0 as i32;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbLevelsSameActions(
    mut a: *const xkb_level,
    mut b: *const xkb_level,
) -> bool {
    unsafe {
        if (*a).num_actions as i32 != (*b).num_actions as i32 {
            return false_0 != 0;
        }
        if (*a).num_actions as i32 <= 1 as i32 {
            return action_equal(&raw const (*a).a.action, &raw const (*b).a.action);
        }
        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
        while (k as i32) < (*a).num_actions as i32 {
            if !action_equal(
                (*a).a.actions.offset(k as isize) as *mut xkb_action,
                (*b).a.actions.offset(k as isize) as *mut xkb_action,
            ) {
                return false_0 != 0;
            }
            k = k.wrapping_add(1);
        }
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbWrapGroupIntoRange(
    mut group: i32,
    mut num_groups: xkb_layout_index_t,
    mut out_of_range_group_policy: xkb_layout_out_of_range_policy,
    mut out_of_range_group_number: xkb_layout_index_t,
) -> xkb_layout_index_t {
    unsafe {
        if num_groups == 0 as xkb_layout_index_t {
            return XKB_LAYOUT_INVALID as xkb_layout_index_t;
        }
        if group >= 0 as i32 && (group as xkb_layout_index_t) < num_groups {
            return group as xkb_layout_index_t;
        }
        match out_of_range_group_policy as u32 {
            2 => {
                if out_of_range_group_number >= num_groups {
                    return 0 as xkb_layout_index_t;
                }
                return out_of_range_group_number;
            }
            1 => {
                if group < 0 as i32 {
                    return 0 as xkb_layout_index_t;
                } else {
                    return num_groups.wrapping_sub(1 as xkb_layout_index_t);
                }
            }
            0 | _ => {
                let rem: i32 = group % num_groups as i32;
                return (if rem >= 0 as i32 {
                    rem
                } else {
                    rem + num_groups as i32
                }) as xkb_layout_index_t;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_key_get_actions_by_level(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut actions: *mut *const xkb_action,
) -> xkb_action_count_t {
    unsafe {
        let mut count: xkb_action_count_t = 0;
        let mut c2rust_current_block: u64;
        if !key.is_null() {
            layout = XkbWrapGroupIntoRange(
                layout as i32,
                (*key).num_groups(),
                (*key).out_of_range_group_policy(),
                (*key).out_of_range_group_number(),
            );
            if !(layout == XKB_LAYOUT_INVALID as xkb_layout_index_t) {
                if !(level >= XkbKeyNumLevels(key, layout)) {
                    count = (*(*(*key).groups.offset(layout as isize))
                        .levels
                        .offset(level as isize))
                    .num_actions;
                    match count as i32 {
                        0 => {}
                        1 => {
                            c2rust_current_block = 945040705843674513;
                            match c2rust_current_block {
                                15050610111240922756 => {
                                    *actions = (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                }
                                _ => {
                                    *actions = &raw mut (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .action;
                                }
                            }
                            return count;
                        }
                        _ => {
                            c2rust_current_block = 15050610111240922756;
                            match c2rust_current_block {
                                15050610111240922756 => {
                                    *actions = (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                }
                                _ => {
                                    *actions = &raw mut (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .action;
                                }
                            }
                            return count;
                        }
                    }
                }
            }
        }
        *actions = ::core::ptr::null::<xkb_action>();
        return 0 as xkb_action_count_t;
    }
}
