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
    pub type __int64_t = i64;
}
pub mod stdint_intn_h {
    pub type i8 = __int8_t;
    pub type i16 = __int16_t;
    pub type i32 = __int32_t;
    pub type i64 = __int64_t;
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
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t) -> *const i8;
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
    pub const XKB_ATOM_NONE: i32 = 0 as i32;
    use super::darray_h::darray_size_t;
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = u32;
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
    use super::stdint_uintn_h::u32;
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
    pub type xkb_overlay_index_t = uint8_t;
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
                        return (*keymap).keys.offset(match_0.key.index() as isize) as *mut xkb_key;
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
    extern "C" {
        pub fn action_equal(a: *const xkb_action, b: *const xkb_action) -> bool;
    }
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
    pub type xkb_message_code = u32;
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
pub mod ast_h {
    pub type stmt_type = u32;
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
    pub type merge_mode = u32;
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
        pub ival: i64,
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
    use super::stdint_intn_h::i64;
    use super::xkbcommon_h::xkb_keysym_t;
    extern "C" {
        pub fn stmt_type_to_string(type_0: stmt_type) -> *const i8;
    }
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
    use super::keymap_h::xkb_action_type;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn LookupString(
            tab: *const LookupEntry,
            string: *const i8,
            value_rtrn: *mut u32,
        ) -> bool;
        pub fn LookupValue(tab: *const LookupEntry, value: u32) -> *const i8;
        pub static ctrlMaskNames: [LookupEntry; 0];
        pub static actionTypeNames: [LookupEntry; 0];
        pub fn ActionTypeText(type_0: xkb_action_type) -> *const i8;
        pub fn KeyNameText(ctx: *mut xkb_context, name: xkb_atom_t) -> *const i8;
    }
}
pub mod xkbcomp_priv_h {
    pub type xkb_parser_error = u32;
    pub const PARSER_FATAL_ERROR: xkb_parser_error = 2;
    pub const PARSER_RECOVERABLE_ERROR: xkb_parser_error = 1;
    pub const PARSER_SUCCESS: xkb_parser_error = 0;
    pub type xkb_parser_strict_flags = u32;
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
pub mod action_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ActionsInfo {
        pub actions: [xkb_action; 21],
    }
    use super::keymap_h::xkb_action;
}
pub mod stdlib_h {

    extern "C" {
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: i32,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strlen(__s: *const i8) -> usize;
    }
}
pub mod expr_h {
    use super::ast_h::ExprDef;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::keymap_h::{mod_type, xkb_mod_set};
    use super::stdint_intn_h::i64;
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
        pub fn ExprResolveInteger(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            val_rtrn: *mut i64,
        ) -> bool;
        pub fn ExprResolveGroup(
            keymap_info: *const xkb_keymap_info,
            expr: *const ExprDef,
            absolute: bool,
            group_rtrn: *mut xkb_layout_index_t,
            pending_rtrn: *mut bool,
        ) -> xkb_parser_error;
        pub fn ExprResolveButton(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            btn_rtrn: *mut i64,
        ) -> bool;
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
        pub fn ExprResolveMask(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            mask_rtrn: *mut u32,
            values: *const LookupEntry,
        ) -> bool;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const INT8_MIN: i32 = -128 as i32;
    pub const INT16_MIN: i32 = -32767 as i32 - 1 as i32;
    pub const INT8_MAX: i32 = 127 as i32;
    pub const INT16_MAX: i32 = 32767 as i32;
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn istreq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as i32;
        }
    }
    pub use crate::xkb::utils::istrcmp;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub use self::__stddef_null_h::NULL;

pub use self::action_h::ActionsInfo;
pub use self::ast_h::{
    _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, C2Rust_Unnamed_13, ExprAction,
    ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef, ExprIdent,
    ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary, ParseCommon,
    _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
    MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD,
    STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE,
    STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT,
    STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT,
    STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT,
    STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP,
    STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION,
    STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
use self::expr_h::{
    ExprResolveBoolean, ExprResolveButton, ExprResolveEnum, ExprResolveGroup, ExprResolveInteger,
    ExprResolveLhs, ExprResolveMask, ExprResolveModMask, ExprResolveString,
};
pub use self::internal::__va_list_tag;
pub use self::keymap_h::{
    action_equal, mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, XkbKeyByName, _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
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
pub use self::stdint_h::{INT16_MAX, INT16_MIN, INT8_MAX, INT8_MIN};
pub use self::stdint_intn_h::{i16, i32, i64, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::realloc;
use self::string_h::{memcpy, memset, strlen};
pub use self::text_h::{
    actionTypeNames, ctrlMaskNames, ActionTypeText, KeyNameText, LookupEntry, LookupString,
    LookupValue,
};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::utils_h::{istrcmp, istreq};
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
pub type action_field = u32;
pub const ACTION_FIELD_LATCH_ON_PRESS: action_field = 25;
pub const ACTION_FIELD_UNLOCK_ON_PRESS: action_field = 24;
pub const ACTION_FIELD_LOCK_ON_RELEASE: action_field = 23;
pub const ACTION_FIELD_MODS_TO_CLEAR: action_field = 22;
pub const ACTION_FIELD_KEYCODE: action_field = 21;
pub const ACTION_FIELD_DEVICE: action_field = 20;
pub const ACTION_FIELD_DATA: action_field = 19;
pub const ACTION_FIELD_SAME: action_field = 18;
pub const ACTION_FIELD_SCREEN: action_field = 17;
pub const ACTION_FIELD_COUNT: action_field = 16;
pub const ACTION_FIELD_TYPE: action_field = 15;
pub const ACTION_FIELD_CONTROLS: action_field = 14;
pub const ACTION_FIELD_VALUE: action_field = 13;
pub const ACTION_FIELD_BUTTON: action_field = 12;
pub const ACTION_FIELD_ACCEL: action_field = 11;
pub const ACTION_FIELD_Y: action_field = 10;
pub const ACTION_FIELD_X: action_field = 9;
pub const ACTION_FIELD_GROUP: action_field = 8;
pub const ACTION_FIELD_MODIFIERS: action_field = 7;
pub const ACTION_FIELD_INCREMENT: action_field = 6;
pub const ACTION_FIELD_AFFECT: action_field = 5;
pub const ACTION_FIELD_DEFAULT: action_field = 4;
pub const ACTION_FIELD_REPORT: action_field = 3;
pub const ACTION_FIELD_GEN_KEY_EVENT: action_field = 2;
pub const ACTION_FIELD_LATCH_TO_LOCK: action_field = 1;
pub const ACTION_FIELD_CLEAR_LOCKS: action_field = 0;
pub type actionHandler = Option<
    unsafe extern "C" fn(
        *const xkb_keymap_info,
        *const xkb_mod_set,
        *mut xkb_action,
        action_field,
        *const ExprDef,
        *const ExprDef,
        *mut *mut ExprDef,
    ) -> xkb_parser_error,
>;
static mut constTrue: ExprBoolean = ExprBoolean {
    common: _ParseCommon {
        next: ::core::ptr::null::<_ParseCommon>() as *mut _ParseCommon,
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: true_0 != 0,
};
static mut constFalse: ExprBoolean = ExprBoolean {
    common: _ParseCommon {
        next: ::core::ptr::null::<_ParseCommon>() as *mut _ParseCommon,
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: false_0 != 0,
};
#[no_mangle]
pub unsafe extern "C" fn InitActionsInfo(
    mut keymap: *const xkb_keymap,
    mut info: *mut ActionsInfo,
) {
    unsafe {
        let mut type_0: xkb_action_type = ACTION_TYPE_NONE;
        while (type_0 as u32) < _ACTION_TYPE_NUM_ENTRIES as i32 as u32 {
            (*info).actions[type_0 as usize].type_0 = type_0;
            type_0 += 1;
        }
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as i32 as usize]
            .dflt
            .flags = 0 as xkb_action_flags;
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as i32 as usize]
            .dflt
            .value = 1 as i8;
        (*info).actions[ACTION_TYPE_PTR_MOVE as i32 as usize]
            .ptr
            .flags = ACTION_ACCEL;
        (*info).actions[ACTION_TYPE_SWITCH_VT as i32 as usize]
            .screen
            .flags = ACTION_SAME_SCREEN;
        (*info).actions[ACTION_TYPE_REDIRECT_KEY as i32 as usize]
            .redirect
            .keycode = (*keymap).redirect_key_auto;
    }
}
static mut fieldStrings: [LookupEntry; 37] = [
    LookupEntry {
        name: b"clearLocks\0".as_ptr() as *const i8,
        value: ACTION_FIELD_CLEAR_LOCKS as i32 as u32,
    },
    LookupEntry {
        name: b"latchToLock\0".as_ptr() as *const i8,
        value: ACTION_FIELD_LATCH_TO_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"genKeyEvent\0".as_ptr() as *const i8,
        value: ACTION_FIELD_GEN_KEY_EVENT as i32 as u32,
    },
    LookupEntry {
        name: b"generateKeyEvent\0".as_ptr() as *const i8,
        value: ACTION_FIELD_GEN_KEY_EVENT as i32 as u32,
    },
    LookupEntry {
        name: b"report\0".as_ptr() as *const i8,
        value: ACTION_FIELD_REPORT as i32 as u32,
    },
    LookupEntry {
        name: b"default\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DEFAULT as i32 as u32,
    },
    LookupEntry {
        name: b"affect\0".as_ptr() as *const i8,
        value: ACTION_FIELD_AFFECT as i32 as u32,
    },
    LookupEntry {
        name: b"increment\0".as_ptr() as *const i8,
        value: ACTION_FIELD_INCREMENT as i32 as u32,
    },
    LookupEntry {
        name: b"modifiers\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODIFIERS as i32 as u32,
    },
    LookupEntry {
        name: b"mods\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODIFIERS as i32 as u32,
    },
    LookupEntry {
        name: b"group\0".as_ptr() as *const i8,
        value: ACTION_FIELD_GROUP as i32 as u32,
    },
    LookupEntry {
        name: b"x\0".as_ptr() as *const i8,
        value: ACTION_FIELD_X as i32 as u32,
    },
    LookupEntry {
        name: b"y\0".as_ptr() as *const i8,
        value: ACTION_FIELD_Y as i32 as u32,
    },
    LookupEntry {
        name: b"accel\0".as_ptr() as *const i8,
        value: ACTION_FIELD_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"accelerate\0".as_ptr() as *const i8,
        value: ACTION_FIELD_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"repeat\0".as_ptr() as *const i8,
        value: ACTION_FIELD_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"button\0".as_ptr() as *const i8,
        value: ACTION_FIELD_BUTTON as i32 as u32,
    },
    LookupEntry {
        name: b"value\0".as_ptr() as *const i8,
        value: ACTION_FIELD_VALUE as i32 as u32,
    },
    LookupEntry {
        name: b"controls\0".as_ptr() as *const i8,
        value: ACTION_FIELD_CONTROLS as i32 as u32,
    },
    LookupEntry {
        name: b"ctrls\0".as_ptr() as *const i8,
        value: ACTION_FIELD_CONTROLS as i32 as u32,
    },
    LookupEntry {
        name: b"type\0".as_ptr() as *const i8,
        value: ACTION_FIELD_TYPE as i32 as u32,
    },
    LookupEntry {
        name: b"count\0".as_ptr() as *const i8,
        value: ACTION_FIELD_COUNT as i32 as u32,
    },
    LookupEntry {
        name: b"screen\0".as_ptr() as *const i8,
        value: ACTION_FIELD_SCREEN as i32 as u32,
    },
    LookupEntry {
        name: b"same\0".as_ptr() as *const i8,
        value: ACTION_FIELD_SAME as i32 as u32,
    },
    LookupEntry {
        name: b"sameServer\0".as_ptr() as *const i8,
        value: ACTION_FIELD_SAME as i32 as u32,
    },
    LookupEntry {
        name: b"data\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DATA as i32 as u32,
    },
    LookupEntry {
        name: b"device\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DEVICE as i32 as u32,
    },
    LookupEntry {
        name: b"dev\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DEVICE as i32 as u32,
    },
    LookupEntry {
        name: b"key\0".as_ptr() as *const i8,
        value: ACTION_FIELD_KEYCODE as i32 as u32,
    },
    LookupEntry {
        name: b"keycode\0".as_ptr() as *const i8,
        value: ACTION_FIELD_KEYCODE as i32 as u32,
    },
    LookupEntry {
        name: b"kc\0".as_ptr() as *const i8,
        value: ACTION_FIELD_KEYCODE as i32 as u32,
    },
    LookupEntry {
        name: b"clearmods\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODS_TO_CLEAR as i32 as u32,
    },
    LookupEntry {
        name: b"clearmodifiers\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODS_TO_CLEAR as i32 as u32,
    },
    LookupEntry {
        name: b"lockOnRelease\0".as_ptr() as *const i8,
        value: ACTION_FIELD_LOCK_ON_RELEASE as i32 as u32,
    },
    LookupEntry {
        name: b"unlockOnPress\0".as_ptr() as *const i8,
        value: ACTION_FIELD_UNLOCK_ON_PRESS as i32 as u32,
    },
    LookupEntry {
        name: b"latchOnPress\0".as_ptr() as *const i8,
        value: ACTION_FIELD_LATCH_ON_PRESS as i32 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe extern "C" fn stringToActionType(
    mut str: *const i8,
    mut type_rtrn: *mut xkb_action_type,
) -> bool {
    unsafe {
        let mut type_0: u32 = 0 as u32;
        let ret: bool = LookupString(
            &raw const actionTypeNames as *const LookupEntry,
            str,
            &raw mut type_0,
        ) as bool;
        *type_rtrn = type_0 as xkb_action_type;
        return ret;
    }
}
unsafe extern "C" fn stringToField(mut str: *const i8, mut field_rtrn: *mut action_field) -> bool {
    unsafe {
        let mut field: u32 = 0 as u32;
        let ret: bool = LookupString(
            &raw const fieldStrings as *const LookupEntry,
            str,
            &raw mut field,
        ) as bool;
        *field_rtrn = field as action_field;
        return ret;
    }
}
unsafe extern "C" fn fieldText(mut field: action_field) -> *const i8 {
    unsafe {
        return LookupValue(&raw const fieldStrings as *const LookupEntry, field as u32);
    }
}
#[inline]
unsafe extern "C" fn ReportMismatch(
    mut ctx: *mut xkb_context,
    mut code: xkb_message_code,
    mut action: xkb_action_type,
    mut field: action_field,
    mut type_0: *const i8,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            b"[XKB-%03d] Value of %s field must be of type %s; Action %s definition ignored\n\0"
                .as_ptr() as *const i8,
            code as u32,
            fieldText(field),
            type_0,
            ActionTypeText(action),
        );
        return (if strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe extern "C" fn ReportFormatVersionMismatch(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: action_field,
    mut format: xkb_keymap_format,
    mut versions: *const i8,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            b"[XKB-%03d] Field %s for an action of type %s requires keymap text format %s,  but got: %d; Action definition ignored\n\0"
                .as_ptr() as *const i8,
            XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT as i32,
            fieldText(field),
            ActionTypeText(action),
            versions,
            format as u32,
        );
        return (if strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe extern "C" fn ReportIllegal(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: action_field,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            b"[XKB-%03d] Field %s is not defined for an action of type %s; Action definition ignored\n\0"
                .as_ptr() as *const i8,
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            fieldText(field),
            ActionTypeText(action),
        );
        return (if strict as u32 & PARSER_NO_ILLEGAL_ACTION_FIELDS as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe extern "C" fn ReportActionNotArray(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: action_field,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            b"[XKB-%03d] The %s field in the %s action is not an array; Action definition ignored\n\0"
                .as_ptr() as *const i8,
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            fieldText(field),
            ActionTypeText(action),
        );
        return (if strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error;
    }
}
unsafe extern "C" fn HandleNoAction(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        xkb_log(
            (*keymap_info).keymap.ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            b"[XKB-%03d] The \"%s\" action takes no argument, but got \"%s\" field; Action definition ignored\n\0"
                .as_ptr() as *const i8,
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            ActionTypeText((*action).type_0),
            fieldText(field),
        );
        return (if (*keymap_info).strict as u32 & PARSER_NO_ILLEGAL_ACTION_FIELDS as i32 as u32 != 0
        {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
unsafe extern "C" fn CheckBooleanFlag(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut action: xkb_action_type,
    mut field: action_field,
    mut flag: xkb_action_flags,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        let mut set: bool = false_0 != 0;
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, field, strict);
        }
        if !ExprResolveBoolean(ctx, value, &raw mut set) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                field,
                b"boolean\0".as_ptr() as *const i8,
                strict,
            );
        }
        if set {
            *flags_inout = (*flags_inout as u32 | flag as u32) as xkb_action_flags;
        } else {
            *flags_inout = (*flags_inout as u32 & !(flag as u32)) as xkb_action_flags;
        }
        return PARSER_SUCCESS;
    }
}
unsafe extern "C" fn CheckModifierField(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut mods: *const xkb_mod_set,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
    mut mods_rtrn: *mut xkb_mod_mask_t,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_MODIFIERS, strict);
        }
        if (*value).common.type_0 as u32 == STMT_EXPR_IDENT as i32 as u32 {
            let mut valStr: *const i8 = ::core::ptr::null::<i8>();
            valStr = xkb_atom_text(ctx, (*value).ident.ident);
            if !valStr.is_null()
                && (istreq(valStr, b"usemodmapmods\0".as_ptr() as *const i8) as i32 != 0
                    || istreq(valStr, b"modmapmods\0".as_ptr() as *const i8) as i32 != 0)
            {
                *mods_rtrn = 0 as xkb_mod_mask_t;
                *flags_inout = (*flags_inout as u32 | ACTION_MODS_LOOKUP_MODMAP as i32 as u32)
                    as xkb_action_flags;
                return PARSER_SUCCESS;
            }
        }
        if !ExprResolveModMask(ctx, value, MOD_BOTH, mods, mods_rtrn) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_MODIFIERS,
                b"modifier mask\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout =
            (*flags_inout as u32 & !(ACTION_MODS_LOOKUP_MODMAP as i32) as u32) as xkb_action_flags;
        return PARSER_SUCCESS;
    }
}
static mut lockWhich: [LookupEntry; 5] = [
    LookupEntry {
        name: b"both\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: b"lock\0".as_ptr() as *const i8,
        value: ACTION_LOCK_NO_UNLOCK as i32 as u32,
    },
    LookupEntry {
        name: b"neither\0".as_ptr() as *const i8,
        value: (ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32,
    },
    LookupEntry {
        name: b"unlock\0".as_ptr() as *const i8,
        value: ACTION_LOCK_NO_LOCK as i32 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe extern "C" fn CheckAffectField(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_AFFECT, strict);
        }
        let mut flags: u32 = 0 as u32;
        if !ExprResolveEnum(
            ctx,
            value,
            &raw mut flags,
            &raw const lockWhich as *const LookupEntry,
        ) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_AFFECT,
                b"lock, unlock, both, neither\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout = (*flags_inout as u32
            & !(ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32)
            as xkb_action_flags;
        *flags_inout = (*flags_inout as u32 | flags as xkb_action_flags as u32) as xkb_action_flags;
        return PARSER_SUCCESS;
    }
}
unsafe extern "C" fn HandleSetLatchLockMods(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_mod_action = &raw mut (*action).mods;
        let type_0: xkb_action_type = (*action).type_0;
        if field as u32 == ACTION_FIELD_MODIFIERS as i32 as u32 {
            return CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
                &raw mut (*act).mods.mods,
            );
        }
        if field as u32 == ACTION_FIELD_UNLOCK_ON_PRESS as i32 as u32 {
            if (*keymap_info).features.mods_unlock_on_press {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_UNLOCK_ON_PRESS,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    ctx,
                    (*action).type_0,
                    field,
                    (*keymap_info).keymap.format,
                    b">= 2\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
        }
        if (type_0 as u32 == ACTION_TYPE_MOD_SET as i32 as u32
            || type_0 as u32 == ACTION_TYPE_MOD_LATCH as i32 as u32)
            && field as u32 == ACTION_FIELD_CLEAR_LOCKS as i32 as u32
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LOCK_CLEAR,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_MOD_LATCH as i32 as u32 {
            if field as u32 == ACTION_FIELD_LATCH_TO_LOCK as i32 as u32 {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_LATCH_TO_LOCK,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            }
            if field as u32 == ACTION_FIELD_LATCH_ON_PRESS as i32 as u32 {
                if (*keymap_info).features.mods_latch_on_press {
                    return CheckBooleanFlag(
                        ctx,
                        (*keymap_info).strict,
                        (*action).type_0,
                        field,
                        ACTION_LATCH_ON_PRESS,
                        array_ndx,
                        value,
                        &raw mut (*act).flags,
                    );
                } else {
                    return ReportFormatVersionMismatch(
                        ctx,
                        (*action).type_0,
                        field,
                        (*keymap_info).keymap.format,
                        b">= 2\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
            }
        }
        if type_0 as u32 == ACTION_TYPE_MOD_LOCK as i32 as u32
            && field as u32 == ACTION_FIELD_AFFECT as i32 as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn CheckGroupField(
    mut keymap_info: *const xkb_keymap_info,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    mut flags_inout: *mut xkb_action_flags,
    mut group_rtrn: *mut i32,
) -> xkb_parser_error {
    unsafe {
        let mut spec: *const ExprDef = ::core::ptr::null::<ExprDef>();
        let mut idx: xkb_layout_index_t = 0 as xkb_layout_index_t;
        let mut flags: xkb_action_flags = *flags_inout;
        if !array_ndx.is_null() {
            return ReportActionNotArray(
                (*keymap_info).keymap.ctx,
                action,
                ACTION_FIELD_GROUP,
                (*keymap_info).strict,
            );
        }
        if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32
            || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as i32 as u32
        {
            flags = (flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            spec = (*value).unary.child;
            value_ptr = &raw mut (**value_ptr).unary.child as *mut *mut ExprDef;
        } else {
            flags = (flags as u32 | ACTION_ABSOLUTE_SWITCH as i32 as u32) as xkb_action_flags;
            spec = value;
        }
        let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 != 0;
        let mut pending: bool = false_0 != 0;
        let ret: xkb_parser_error =
            ExprResolveGroup(keymap_info, spec, absolute, &raw mut idx, &raw mut pending)
                as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS as i32 as u32 && !pending {
            ReportMismatch(
                (*keymap_info).keymap.ctx,
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
                action,
                ACTION_FIELD_GROUP,
                b"integer\0".as_ptr() as *const i8,
                (*keymap_info).strict,
            );
            return ret;
        }
        if pending {
            flags = (flags as u32 | ACTION_PENDING_COMPUTATION as i32 as u32) as xkb_action_flags;
            let pending_index: darray_size_t = (*(*keymap_info).pending_computations).size;
            (*(*keymap_info).pending_computations).size = (*(*keymap_info).pending_computations)
                .size
                .wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = (*(*keymap_info).pending_computations).size;
            if __need > (*(*keymap_info).pending_computations).alloc {
                (*(*keymap_info).pending_computations).alloc = darray_next_alloc(
                    (*(*keymap_info).pending_computations).alloc,
                    __need,
                    ::core::mem::size_of::<pending_computation>() as usize,
                );
                (*(*keymap_info).pending_computations).item = realloc(
                    (*(*keymap_info).pending_computations).item as *mut ::core::ffi::c_void,
                    ((*(*keymap_info).pending_computations).alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<pending_computation>() as usize),
                )
                    as *mut pending_computation;
            }
            *(*(*keymap_info).pending_computations).item.offset(
                (*(*keymap_info).pending_computations)
                    .size
                    .wrapping_sub(1 as darray_size_t) as isize,
            ) = pending_computation {
                expr: *value_ptr,
                computed: false,
                value: 0 as u32,
            };
            *value_ptr = ::core::ptr::null_mut::<ExprDef>();
            *group_rtrn = pending_index as i32;
        } else {
            flags =
                (flags as u32 & !(ACTION_PENDING_COMPUTATION as i32) as u32) as xkb_action_flags;
            if flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 == 0 {
                *group_rtrn = idx as i32;
                if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                    *group_rtrn = -*group_rtrn;
                }
            } else {
                *group_rtrn = idx.wrapping_sub(1 as xkb_layout_index_t) as i32;
            }
        }
        *flags_inout = flags;
        return PARSER_SUCCESS;
    }
}
unsafe extern "C" fn HandleSetLatchLockGroup(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_group_action = &raw mut (*action).group;
        let type_0: xkb_action_type = (*action).type_0;
        if field as u32 == ACTION_FIELD_GROUP as i32 as u32 {
            return CheckGroupField(
                keymap_info,
                (*action).type_0,
                array_ndx,
                value,
                value_ptr,
                &raw mut (*act).flags,
                &raw mut (*act).group,
            );
        }
        if (type_0 as u32 == ACTION_TYPE_GROUP_SET as i32 as u32
            || type_0 as u32 == ACTION_TYPE_GROUP_LATCH as i32 as u32)
            && field as u32 == ACTION_FIELD_CLEAR_LOCKS as i32 as u32
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LOCK_CLEAR,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_GROUP_LATCH as i32 as u32
            && field as u32 == ACTION_FIELD_LATCH_TO_LOCK as i32 as u32
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LATCH_TO_LOCK,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_GROUP_LOCK as i32 as u32
            && field as u32 == ACTION_FIELD_LOCK_ON_RELEASE as i32 as u32
        {
            if (*keymap_info).features.group_lock_on_release {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_LOCK_ON_RELEASE,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    ctx,
                    (*action).type_0,
                    field,
                    (*keymap_info).keymap.format,
                    b">= v2\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn HandleMovePtr(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_pointer_action = &raw mut (*action).ptr;
        if field as u32 == ACTION_FIELD_X as i32 as u32
            || field as u32 == ACTION_FIELD_Y as i32 as u32
        {
            let mut val: i64 = 0 as i64;
            let absolute: bool = (*value).common.type_0 as u32 != STMT_EXPR_NEGATE as i32 as u32
                && (*value).common.type_0 as u32 != STMT_EXPR_UNARY_PLUS as i32 as u32;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if val < INT16_MIN as i64 || val > INT16_MAX as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"The %s field in the %s action must be in range %d..%d, but got %ld. Action definition ignored\n\0"
                        .as_ptr() as *const i8,
                    fieldText(field),
                    ActionTypeText((*action).type_0),
                    -32767 as i32 - 1 as i32,
                    32767 as i32,
                    val,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if field as u32 == ACTION_FIELD_X as i32 as u32 {
                if absolute {
                    (*act).flags =
                        ((*act).flags as u32 | ACTION_ABSOLUTE_X as i32 as u32) as xkb_action_flags;
                }
                (*act).x = val as i16;
            } else {
                if absolute {
                    (*act).flags =
                        ((*act).flags as u32 | ACTION_ABSOLUTE_Y as i32 as u32) as xkb_action_flags;
                }
                (*act).y = val as i16;
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_ACCEL as i32 as u32 {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_ACCEL,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn HandlePtrBtn(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_pointer_button_action = &raw mut (*action).btn;
        if field as u32 == ACTION_FIELD_BUTTON as i32 as u32 {
            let mut btn: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveButton(ctx, value, &raw mut btn) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (range 1..5)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if btn < 0 as i64 || btn > 5 as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Button must specify default or be in the range 1..5; Illegal button value %ld ignored\n\0"
                        .as_ptr() as *const i8,
                    btn,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).button = btn as uint8_t;
            return PARSER_SUCCESS;
        } else if (*action).type_0 as u32 == ACTION_TYPE_PTR_LOCK as i32 as u32
            && field as u32 == ACTION_FIELD_AFFECT as i32 as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        } else if field as u32 == ACTION_FIELD_COUNT as i32 as u32 {
            let mut val: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if val < 0 as i64 || val > 255 as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"The count field must have a value in the range 0..255; Illegal count %ld ignored\n\0"
                        .as_ptr() as *const i8,
                    val,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).count = val as uint8_t;
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
static mut ptrDflts: [LookupEntry; 4] = [
    LookupEntry {
        name: b"dfltbtn\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"defaultbutton\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"button\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe extern "C" fn HandleSetPtrDflt(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_pointer_default_action = &raw mut (*action).dflt;
        if field as u32 == ACTION_FIELD_AFFECT as i32 as u32 {
            let mut val: u32 = 0 as u32;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveEnum(
                ctx,
                value,
                &raw mut val,
                &raw const ptrDflts as *const LookupEntry,
            ) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"pointer component\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_BUTTON as i32 as u32
            || field as u32 == ACTION_FIELD_VALUE as i32 as u32
        {
            let mut button: *const ExprDef = ::core::ptr::null::<ExprDef>();
            let mut btn: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32
                || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as i32 as u32
            {
                (*act).flags = ((*act).flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32)
                    as xkb_action_flags;
                button = (*value).unary.child;
            } else {
                (*act).flags = ((*act).flags as u32 | ACTION_ABSOLUTE_SWITCH as i32 as u32)
                    as xkb_action_flags;
                button = value;
            }
            if !ExprResolveButton(ctx, button, &raw mut btn) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (range 1..5)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if btn < 0 as i64 || btn > 5 as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"New default button value must be in the range 1..5; Illegal default button value %ld ignored\n\0"
                        .as_ptr() as *const i8,
                    btn,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if btn == 0 as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Cannot set default pointer button to \"default\"; Illegal default button setting ignored\n\0"
                        .as_ptr() as *const i8,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).value = (if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                -btn
            } else {
                btn
            }) as i8;
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn HandleSwitchScreen(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_switch_screen_action = &raw mut (*action).screen;
        if field as u32 == ACTION_FIELD_SCREEN as i32 as u32 {
            let mut scrn: *const ExprDef = ::core::ptr::null::<ExprDef>();
            let mut val: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32
                || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as i32 as u32
            {
                (*act).flags = ((*act).flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32)
                    as xkb_action_flags;
                scrn = (*value).unary.child;
            } else {
                (*act).flags = ((*act).flags as u32 | ACTION_ABSOLUTE_SWITCH as i32 as u32)
                    as xkb_action_flags;
                scrn = value;
            }
            if !ExprResolveInteger(ctx, scrn, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (-128..127)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            val = if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                -val
            } else {
                val
            };
            if val < INT8_MIN as i64 || val > INT8_MAX as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Screen index must be in the range %d..%d; Illegal screen value %ld ignored\n\0"
                        .as_ptr() as *const i8,
                    -128 as i32,
                    127 as i32,
                    val,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).screen = val as i8;
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_SAME as i32 as u32 {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_SAME_SCREEN,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn HandleSetLockControls(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_controls_action = &raw mut (*action).ctrls;
        if field as u32 == ACTION_FIELD_CONTROLS as i32 as u32 {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            let mut mask: u32 = 0 as u32;
            let offset: uint8_t = (*keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                ctx,
                value,
                &raw mut mask,
                (&raw const ctrlMaskNames as *const LookupEntry).offset(offset as i32 as isize),
            ) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"controls mask\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            (*act).ctrls = mask as xkb_action_controls;
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_AFFECT as i32 as u32
            && (*action).type_0 as u32 == ACTION_TYPE_CTRL_LOCK as i32 as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn HandleRedirectKey(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let keymap: *const xkb_keymap = &raw const (*keymap_info).keymap;
        let ctx: *mut xkb_context = (*keymap).ctx;
        let act: *mut xkb_redirect_key_action = &raw mut (*action).redirect;
        if field as u32 == ACTION_FIELD_KEYCODE as i32 as u32 {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_IDENT as i32 as u32 {
                let mut valStr: *const i8 = xkb_atom_text(ctx, (*value).ident.ident);
                if !valStr.is_null() && istreq(valStr, b"auto\0".as_ptr() as *const i8) as i32 != 0
                {
                    (*act).keycode = (*keymap_info).keymap.redirect_key_auto;
                    return PARSER_SUCCESS;
                }
            }
            if (*value).common.type_0 as u32 != STMT_EXPR_KEYNAME_LITERAL as i32 as u32 {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"key name\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            let key: *const xkb_key = XkbKeyByName(keymap, (*value).key_name.key_name, true_0 != 0);
            if key.is_null() {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"RedirectKey field %s cannot resolve %s to a valid key\n\0".as_ptr()
                        as *const i8,
                    fieldText(field),
                    KeyNameText(ctx, (*value).key_name.key_name),
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_VALUE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).keycode = (*key).keycode;
            return PARSER_SUCCESS;
        }
        if field as u32 == ACTION_FIELD_MODIFIERS as i32 as u32
            || field as u32 == ACTION_FIELD_MODS_TO_CLEAR as i32 as u32
        {
            let mut flags: xkb_action_flags = 0 as xkb_action_flags;
            let mut m: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
            let mut r: xkb_parser_error = CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut flags,
                &raw mut m,
            );
            if r as u32 != PARSER_SUCCESS as i32 as u32 {
                return r;
            }
            if flags as u64 != 0 {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"modifier mask\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            (*act).affect |= m;
            if field as u32 == ACTION_FIELD_MODIFIERS as i32 as u32 {
                (*act).mods |= m;
            } else {
                (*act).mods &= !m;
            }
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe extern "C" fn HandleUnsupported(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        return PARSER_SUCCESS;
    }
}
unsafe extern "C" fn HandlePrivate(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_private_action = &raw mut (*action).priv_0;
        if field as u32 == ACTION_FIELD_TYPE as i32 as u32 {
            let mut type_0: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut type_0) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    ACTION_TYPE_PRIVATE,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if type_0 < 0 as i64 || type_0 > 255 as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Private action type must be in the range 0..255; Illegal type %ld ignored\n\0"
                        .as_ptr() as *const i8,
                    type_0,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if type_0 < ACTION_TYPE_PRIVATE as i32 as i64 {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_INFO,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"Private actions of type %s are not supported; Ignored\n\0".as_ptr()
                        as *const i8,
                    ActionTypeText(type_0 as xkb_action_type),
                );
                (*act).type_0 = ACTION_TYPE_NONE;
            } else {
                (*act).type_0 = type_0 as xkb_action_type;
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_DATA as i32 as u32 {
            if array_ndx.is_null() {
                let mut val: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
                if !ExprResolveString(ctx, value, &raw mut val) {
                    return ReportMismatch(
                        ctx,
                        XKB_ERROR_WRONG_FIELD_TYPE,
                        (*action).type_0,
                        field,
                        b"string\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
                let mut str: *const i8 = xkb_atom_text(ctx, val);
                let mut len: usize = strlen(str);
                if len < 1 as usize || len > ::core::mem::size_of::<[uint8_t; 7]>() as usize {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"A private action has %zu data bytes, but got: %zu; Illegal data ignored\n\0"
                            .as_ptr() as *const i8,
                        ::core::mem::size_of::<[uint8_t; 7]>(),
                        len,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                memset(
                    &raw mut (*act).data as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as i32,
                    ::core::mem::size_of::<[uint8_t; 7]>() as usize,
                );
                memcpy(
                    &raw mut (*act).data as *mut uint8_t as *mut ::core::ffi::c_void,
                    str as *const ::core::ffi::c_void,
                    len,
                );
                return PARSER_SUCCESS;
            } else {
                let mut ndx: i64 = 0 as i64;
                let mut datum: i64 = 0 as i64;
                if !ExprResolveInteger(ctx, array_ndx, &raw mut ndx) {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"Array subscript must be integer; Illegal subscript ignored\n\0".as_ptr()
                            as *const i8,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if ndx < 0 as i64 || ndx as usize >= ::core::mem::size_of::<[uint8_t; 7]>() as usize
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"The data for a private action is %zu bytes long; Attempt to use data[%ld] ignored\n\0"
                            .as_ptr() as *const i8,
                        ::core::mem::size_of::<[uint8_t; 7]>(),
                        ndx,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if !ExprResolveInteger(ctx, value, &raw mut datum) {
                    return ReportMismatch(
                        ctx,
                        XKB_ERROR_WRONG_FIELD_TYPE,
                        (*act).type_0,
                        field,
                        b"integer\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
                if datum < 0 as i64 || datum > 255 as i64 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        b"All data for a private action must be 0..255; Illegal datum %ld ignored\n\0"
                            .as_ptr() as *const i8,
                        datum,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                (*act).data[ndx as usize] = datum as uint8_t;
                return PARSER_SUCCESS;
            }
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
static mut handleAction: [actionHandler; 21] = unsafe {
    [
        Some(
            HandleNoAction
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleNoAction
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleMovePtr
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePtrBtn
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePtrBtn
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetPtrDflt
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleNoAction
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSwitchScreen
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLockControls
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLockControls
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleRedirectKey
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleUnsupported
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleUnsupported
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePrivate
                as unsafe extern "C" fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        None,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn HandleActionDef(
    mut keymap_info: *const xkb_keymap_info,
    mut info: *mut ActionsInfo,
    mut mods: *const xkb_mod_set,
    mut def: *mut ExprDef,
    mut action: *mut xkb_action,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        if (*def).common.type_0 as u32 != STMT_EXPR_ACTION_DECL as i32 as u32 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"[XKB-%03d] Expected an action definition, found %s\n\0".as_ptr() as *const i8,
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string((*def).common.type_0),
            );
            return PARSER_FATAL_ERROR;
        }
        let mut action_name: *const i8 = xkb_atom_text(ctx, (*def).action.name);
        let mut handler_type: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(action_name, &raw mut handler_type) {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"[XKB-%03d] Unknown action \"%s\"\n\0".as_ptr() as *const i8,
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                action_name,
            );
            handler_type = ACTION_TYPE_UNKNOWN;
            if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION as i32 as u32 != 0 {
                return PARSER_FATAL_ERROR;
            }
        }
        *action = (*info).actions[handler_type as usize];
        if handler_type as u32 == ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"[XKB-%03d] Unsupported legacy action type \"%s\".\n\0".as_ptr() as *const i8,
                XKB_WARNING_UNSUPPORTED_LEGACY_ACTION as i32,
                action_name,
            );
            (*action).type_0 = ACTION_TYPE_NONE;
        }
        let mut ret: xkb_parser_error = PARSER_SUCCESS;
        let mut arg: *mut ExprDef = (*def).action.args as *mut ExprDef;
        while !arg.is_null() {
            let mut value: *const ExprDef = ::core::ptr::null::<ExprDef>();
            let mut value_ptr: *mut *mut ExprDef = ::core::ptr::null_mut::<*mut ExprDef>();
            let mut field: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            let mut arrayRtrn: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            let mut elemRtrn: *const i8 = ::core::ptr::null::<i8>();
            let mut fieldRtrn: *const i8 = ::core::ptr::null::<i8>();
            if (*arg).common.type_0 as u32 == STMT_EXPR_ASSIGN as i32 as u32 {
                field = (*arg).binary.left as *mut ExprDef;
                value = (*arg).binary.right;
                value_ptr = &raw mut (*arg).binary.right as *mut *mut ExprDef;
            } else if (*arg).common.type_0 as u32 == STMT_EXPR_NOT as i32 as u32
                || (*arg).common.type_0 as u32 == STMT_EXPR_INVERT as i32 as u32
            {
                field = (*arg).unary.child as *mut ExprDef;
                value = &raw const constFalse as *const ExprDef;
            } else {
                field = arg;
                value = &raw const constTrue as *const ExprDef;
            }
            if !ExprResolveLhs(
                ctx,
                field,
                &raw mut elemRtrn,
                &raw mut fieldRtrn,
                &raw mut arrayRtrn,
            ) {
                return PARSER_FATAL_ERROR;
            }
            if !elemRtrn.is_null() {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"[XKB-%03d] Cannot change defaults in an action definition; Ignoring attempt to change \"%s.%s\".\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    elemRtrn,
                    fieldRtrn,
                );
                return PARSER_FATAL_ERROR;
            }
            let mut fieldNdx: action_field = ACTION_FIELD_CLEAR_LOCKS;
            if !stringToField(fieldRtrn, &raw mut fieldNdx) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    b"[XKB-%03d] Unknown field name %s for action %s discarded\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_ACTION_FIELD as i32,
                    fieldRtrn,
                    ActionTypeText((*action).type_0),
                );
                if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as i32 as u32 != 0
                {
                    return PARSER_FATAL_ERROR;
                }
            } else {
                match handleAction[handler_type as usize].expect("non-null function pointer")(
                    keymap_info,
                    mods,
                    action,
                    fieldNdx,
                    arrayRtrn,
                    value,
                    value_ptr,
                ) as u32
                {
                    2 => return PARSER_FATAL_ERROR,
                    1 => {
                        ret = PARSER_RECOVERABLE_ERROR;
                    }
                    _ => {}
                }
            }
            arg = (*arg).common.next as *mut ExprDef;
        }
        return (if (*action).type_0 as u32 == ACTION_TYPE_UNKNOWN as i32 as u32 {
            PARSER_RECOVERABLE_ERROR as i32 as u32
        } else {
            ret as u32
        }) as xkb_parser_error;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetDefaultActionField(
    mut keymap_info: *const xkb_keymap_info,
    mut info: *mut ActionsInfo,
    mut mods: *mut xkb_mod_set,
    mut elem: *const i8,
    mut field: *const i8,
    mut array_ndx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    mut merge: merge_mode,
) -> xkb_parser_error {
    unsafe {
        let value: *const ExprDef = *value_ptr;
        let mut action: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(elem, &raw mut action) {
            xkb_log(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"[XKB-%03d] Unknown action \"%s\"\n\0".as_ptr() as *const i8,
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                elem,
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION as i32 as u32 != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let mut action_field: action_field = ACTION_FIELD_CLEAR_LOCKS;
        if !stringToField(field, &raw mut action_field) {
            xkb_log(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"[XKB-%03d] Unknown action field \"%s\"\n\0".as_ptr() as *const i8,
                XKB_ERROR_INVALID_ACTION_FIELD as i32,
                field,
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as i32 as u32
                != 0
            {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let into: *mut xkb_action = (&raw mut (*info).actions as *mut xkb_action)
            .offset(action as isize) as *mut xkb_action;
        let mut from: xkb_action = *into;
        let ret: xkb_parser_error = handleAction[action as usize]
            .expect("non-null function pointer")(
            keymap_info,
            mods,
            &raw mut from,
            action_field,
            array_ndx,
            value,
            value_ptr,
        ) as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS as i32 as u32 {
            return ret;
        }
        if !action_equal(into, &raw mut from) {
            let replace: bool = merge as u32 != MERGE_AUGMENT as i32 as u32;
            xkb_log(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_VERBOSE as i32,
                b"Conflicting field \"%s\" for default action \"%s\"; Using %s, ignore %s\n\0"
                    .as_ptr() as *const i8,
                fieldText(action_field),
                ActionTypeText(action),
                if replace as i32 != 0 {
                    b"from\0".as_ptr() as *const i8
                } else {
                    b"into\0".as_ptr() as *const i8
                },
                if replace as i32 != 0 {
                    b"into\0".as_ptr() as *const i8
                } else {
                    b"from\0".as_ptr() as *const i8
                },
            );
            if replace {
                *into = from;
            }
        }
        return PARSER_SUCCESS;
    }
}
