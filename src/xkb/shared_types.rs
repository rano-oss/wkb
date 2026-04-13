//! Shared type definitions used across multiple modules.
//!
//! These types were previously duplicated in every file's local `mod xxx_h` blocks
//! (a c2rust artifact). This module provides canonical definitions so that all modules
//! can share the same Rust types, enabling direct `pub use` imports instead of
//! `extern "C"` FFI imports.

// ── Leaf type aliases ────────────────────────────────────────────────

pub type darray_size_t = u32;
pub type xkb_atom_t = darray_size_t;

// ── xkbcommon public types ───────────────────────────────────────────

pub type xkb_log_level = u32;
pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;

pub type xkb_keycode_t = u32;
pub type xkb_mod_mask_t = u32;
pub type xkb_mod_index_t = u32;
pub type xkb_keysym_t = u32;
pub type xkb_layout_index_t = u32;
pub type xkb_level_index_t = u32;
pub type xkb_layout_mask_t = u32;
pub type xkb_led_index_t = u32;

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

pub type xkb_keymap_format = u32;
pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;

pub type xkb_keymap_compile_flags = u32;
pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;

pub const XKB_LAYOUT_INVALID: u32 = 0xffffffff;
pub const XKB_MOD_INVALID: u32 = 0xffffffff;

// ── xkb_rule_names ──────────────────────────────────────────────────

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const i8,
    pub model: *const i8,
    pub layout: *const i8,
    pub variant: *const i8,
    pub options: *const i8,
}

// ── Darray helper structs (used in xkb_context) ─────────────────────

/// String darray (includes field)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *mut i8,
}

/// String darray (failed_includes field)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *mut i8,
}

/// Char darray (used in rules, compose, keymap_file_iterator)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct darray_char {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut i8,
}

/// String pointer darray (used in common, context, registry)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct darray_string {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *mut i8,
}

// ── Opaque types ────────────────────────────────────────────────────

// atom_table is defined in atom.rs — re-export it here for unified access
pub use crate::xkb::atom::atom_table;

// ── xkb_context ─────────────────────────────────────────────────────

use c2rust_bitfields::BitfieldStruct;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct xkb_context {
    pub refcnt: i32,
    pub log_fn: Option<unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ()>,
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

// ── __va_list_tag ───────────────────────────────────────────────────

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}

// ── C stdint type aliases ───────────────────────────────────────────

pub type uint16_t = u16;
pub type uint8_t = u8;

// ── keymap_h types (from keymap_priv.rs) ────────────────────────────

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
    pub data: [u8; 7],
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
    pub count: u8,
    pub button: u8,
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

pub type xkb_action_count_t = u16;

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

pub type xkb_keysym_count_t = u16;
pub type xkb_overlay_mask_t = u8;
pub type xkb_overlay_index_t = u8;

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

// ── Additional xkbcommon types ──────────────────────────────────────

pub type xkb_keymap_serialize_flags = u32;
pub const XKB_KEYMAP_SERIALIZE_EXPLICIT: xkb_keymap_serialize_flags = 4;
pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;

pub type xkb_led_mask_t = u32;

// ── C constants ─────────────────────────────────────────────────────

pub const CHAR_BIT: i32 = 8;
pub const UINT16_MAX: i32 = 65535;

pub const XKB_MAX_LEDS: xkb_led_index_t = (::core::mem::size_of::<xkb_led_mask_t>() as usize)
    .wrapping_mul(CHAR_BIT as usize) as xkb_led_index_t;
pub const MAX_ACTIONS_PER_LEVEL: i32 = UINT16_MAX;

// ── config_h constants ──────────────────────────────────────────────

pub const DEFAULT_XKB_LAYOUT: [i8; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [i8; 3]>(*b"us\0") };
pub const DEFAULT_XKB_MODEL: [i8; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"pc105\0") };
pub const DEFAULT_XKB_OPTIONS: *mut ::core::ffi::c_void = std::ptr::null_mut::<core::ffi::c_void>();
pub const DEFAULT_XKB_RULES: [i8; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"evdev\0") };
pub const DEFAULT_XKB_VARIANT: *mut ::core::ffi::c_void = std::ptr::null_mut::<core::ffi::c_void>();

pub const DFLT_XKB_CONFIG_EXTRA_PATH: [i8; 19] =
    unsafe { ::core::mem::transmute::<[u8; 19], [i8; 19]>(*b"/usr/local/etc/xkb\0") };
pub const DFLT_XKB_CONFIG_ROOT: [i8; 30] =
    unsafe { ::core::mem::transmute::<[u8; 30], [i8; 30]>(*b"/usr/share/xkeyboard-config-2\0") };
pub const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: [i8; 30] =
    unsafe { ::core::mem::transmute::<[u8; 30], [i8; 30]>(*b"/usr/share/xkeyboard-config.d\0") };
pub const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: [i8; 32] =
    unsafe { ::core::mem::transmute::<[u8; 32], [i8; 32]>(*b"/usr/share/xkeyboard-config-2.d\0") };
pub const DFLT_XKB_LEGACY_ROOT: [i8; 19] =
    unsafe { ::core::mem::transmute::<[u8; 19], [i8; 19]>(*b"/usr/share/X11/xkb\0") };

pub const EXIT_INVALID_USAGE: i32 = 2;

// ── xkbcommon_h types (moved from duplicated pub mod xkbcommon_h blocks) ─

pub type xkb_context_flags = u32;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;

pub type xkb_key_direction = u32;
pub const XKB_KEY_UP: xkb_key_direction = 0;
pub const XKB_KEY_DOWN: xkb_key_direction = 1;
pub const XKB_KEY_REPEATED: xkb_key_direction = 2;

pub type xkb_event_type = u32;
pub const XKB_EVENT_TYPE_KEY_DOWN: xkb_event_type = 1;
pub const XKB_EVENT_TYPE_KEY_REPEATED: xkb_event_type = 2;
pub const XKB_EVENT_TYPE_KEY_UP: xkb_event_type = 3;
pub const XKB_EVENT_TYPE_COMPONENTS_CHANGE: xkb_event_type = 4;

pub type xkb_consumed_mode = u32;
pub const XKB_CONSUMED_MODE_XKB: xkb_consumed_mode = 0;
pub const XKB_CONSUMED_MODE_GTK: xkb_consumed_mode = 1;

pub type xkb_keysym_flags = u32;
pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;

pub type xkb_state_match = u32;
pub const XKB_STATE_MATCH_ANY: xkb_state_match = 1;
pub const XKB_STATE_MATCH_ALL: xkb_state_match = 2;
pub const XKB_STATE_MATCH_NON_EXCLUSIVE: xkb_state_match = 65536;

pub type xkb_a11y_flags = u32;
pub const XKB_A11Y_NO_FLAGS: xkb_a11y_flags = 0;
pub const XKB_A11Y_LATCH_TO_LOCK: xkb_a11y_flags = 1;
pub const XKB_A11Y_LATCH_SIMULTANEOUS_KEYS: xkb_a11y_flags = 2;

pub type xkb_keyboard_control_flags = u32;
pub const XKB_KEYBOARD_CONTROL_NO_FLAGS: xkb_keyboard_control_flags = 0;
pub const XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS: xkb_keyboard_control_flags = 1;
pub const XKB_KEYBOARD_CONTROL_OVERLAY1: xkb_keyboard_control_flags = 2;
pub const XKB_KEYBOARD_CONTROL_OVERLAY2: xkb_keyboard_control_flags = 4;
pub const XKB_KEYBOARD_CONTROL_OVERLAY3: xkb_keyboard_control_flags = 8;
pub const XKB_KEYBOARD_CONTROL_OVERLAY4: xkb_keyboard_control_flags = 16;
pub const XKB_KEYBOARD_CONTROL_OVERLAY5: xkb_keyboard_control_flags = 32;
pub const XKB_KEYBOARD_CONTROL_OVERLAY6: xkb_keyboard_control_flags = 64;
pub const XKB_KEYBOARD_CONTROL_OVERLAY7: xkb_keyboard_control_flags = 128;
pub const XKB_KEYBOARD_CONTROL_OVERLAY8: xkb_keyboard_control_flags = 256;

pub type xkb_events_flags = u32;
pub const XKB_EVENTS_NO_FLAGS: xkb_events_flags = 0;

pub type xkb_rmlvo_builder_flags = u32;
pub const XKB_RMLVO_BUILDER_NO_FLAGS: xkb_rmlvo_builder_flags = 0;

pub type xkb_keymap_key_iterator_flags = u32;
pub const XKB_KEYMAP_KEY_ITERATOR_NO_FLAGS: xkb_keymap_key_iterator_flags = 0;
pub const XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER: xkb_keymap_key_iterator_flags = 1;
pub const XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND: xkb_keymap_key_iterator_flags = 2;

pub type xkb_keymap_key_iter_t =
    Option<unsafe fn(*mut xkb_keymap, xkb_keycode_t, *mut ::core::ffi::c_void) -> ()>;

pub const XKB_KEYCODE_INVALID: u32 = 0xffffffff;
pub const XKB_KEYCODE_MAX: u32 = 0xffffffff_u32.wrapping_sub(1);
pub const XKB_LED_INVALID: u32 = 0xffffffff;
pub const XKB_LEVEL_INVALID: u32 = 0xffffffff;
pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 0xffffffff;

pub const XKB_KEYSYM_MAX: i32 = 0x1fffffff;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_component_names {
    pub keycodes: *mut i8,
    pub compatibility: *mut i8,
    pub geometry: *mut i8,
    pub symbols: *mut i8,
    pub types: *mut i8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_state_components_update {
    pub size: usize,
    pub components: xkb_state_component,
    pub affect_latched_mods: xkb_mod_mask_t,
    pub latched_mods: xkb_mod_mask_t,
    pub affect_locked_mods: xkb_mod_mask_t,
    pub locked_mods: xkb_mod_mask_t,
    pub latched_layout: i32,
    pub locked_layout: i32,
    pub affect_controls: xkb_keyboard_control_flags,
    pub controls: xkb_keyboard_control_flags,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_layout_policy_update {
    pub size: usize,
    pub policy: xkb_layout_out_of_range_policy,
    pub redirect: xkb_layout_index_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_state_update {
    pub size: usize,
    pub components: *const xkb_state_components_update,
    pub layout_policy: *const xkb_layout_policy_update,
}

pub const XKB_ATOM_NONE: i32 = 0;

// ── keymap_h types & constants (moved from duplicated pub mod keymap_h blocks) ─

pub type C2Rust_Unnamed_13 = u32;
pub type C2Rust_Unnamed_14 = u32;
pub type C2Rust_Unnamed_15 = u32;
pub type C2Rust_Unnamed_23 = u32;
pub type real_mod_index = u32;

pub const _LAST_XKB_EVENT_TYPE: C2Rust_Unnamed_13 = 4;

pub const FALLBACK_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_14 = 0;
pub const DEFAULT_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_14 = 1;
pub const DEFAULT_KEY_REPEAT: C2Rust_Unnamed_14 = 0;

pub const FALLBACK_INTERPRET_VMODMAP: C2Rust_Unnamed_15 = 0;
pub const DEFAULT_INTERPRET_VMODMAP: C2Rust_Unnamed_15 = 0;
pub const DEFAULT_INTERPRET_VMOD: C2Rust_Unnamed_15 = 4294967295;
pub const DEFAULT_KEY_VMODMAP: C2Rust_Unnamed_15 = 0;

pub const XKB_MOD_ALL: C2Rust_Unnamed_23 = 4294967295;
pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub const XKB_MOD_INDEX_SHIFT: real_mod_index = 0;
pub const XKB_MOD_INDEX_CAPS: real_mod_index = 1;
pub const XKB_MOD_INDEX_CTRL: real_mod_index = 2;
pub const XKB_MOD_INDEX_MOD1: real_mod_index = 3;
pub const XKB_MOD_INDEX_MOD2: real_mod_index = 4;
pub const XKB_MOD_INDEX_MOD3: real_mod_index = 5;
pub const XKB_MOD_INDEX_MOD4: real_mod_index = 6;
pub const XKB_MOD_INDEX_MOD5: real_mod_index = 7;
pub const _XKB_MOD_INDEX_NUM_ENTRIES: real_mod_index = 8;

pub const XKB_MAX_GROUPS_X11: i32 = 4;
pub const XKB_ALL_GROUPS: u64 = ((1u64) << XKB_MAX_GROUPS).wrapping_sub(1u64);

pub const XKB_OVERLAY_MAX_X11: i32 = 2;
pub const XKB_OVERLAY_MAX: u64 =
    (::core::mem::size_of::<xkb_overlay_mask_t>() as u64).wrapping_mul(CHAR_BIT as u64);
pub const XKB_OVERLAY1_CONTROLS_OFFSET: i32 = 1;
pub const XKB_OVERLAY_INVALID: i32 = 255;

pub const XKB_KEYCODE_MAX_CONTIGUOUS: i32 = 0xfff;
pub const XKB_LEVEL_MAX_IMPL: i32 = 2048;
pub const XKB_MAX_MODS: xkb_mod_index_t = (::core::mem::size_of::<xkb_mod_mask_t>() as usize)
    .wrapping_mul(CHAR_BIT as usize) as xkb_mod_index_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_keymap_format_ops {
    pub keymap_new_from_rmlvo: Option<
        unsafe fn(*mut xkb_keymap, *const crate::xkb::rmlvo::rmlvo_h::xkb_rmlvo_builder) -> bool,
    >,
    pub keymap_new_from_names: Option<unsafe fn(*mut xkb_keymap, *const xkb_rule_names) -> bool>,
    pub keymap_new_from_string: Option<unsafe fn(*mut xkb_keymap, *const i8, usize) -> bool>,
    pub keymap_new_from_file: Option<unsafe fn(*mut xkb_keymap, *mut libc::FILE) -> bool>,
    pub keymap_get_as_string: Option<
        unsafe fn(*mut xkb_keymap, xkb_keymap_format, xkb_keymap_serialize_flags) -> *mut i8,
    >,
}

// ── Inline helpers ──────────────────────────────────────────────────

#[inline]
pub unsafe fn XkbKeyNumLevels(
    mut key: *const xkb_key,
    mut layout: xkb_layout_index_t,
) -> xkb_level_index_t {
    unsafe {
        return (*(*(*key).groups.offset(layout as isize)).type_0).num_levels;
    }
}

#[inline]
pub unsafe fn XkbKey(mut keymap: *mut xkb_keymap, mut kc: xkb_keycode_t) -> *const xkb_key {
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

#[inline]
pub unsafe fn XkbKeyByName(
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

#[inline]
pub unsafe fn entry_is_active(mut entry: *const xkb_key_type_entry) -> bool {
    unsafe {
        return (*entry).mods.mods == 0 as xkb_mod_mask_t
            || (*entry).mods.mask != 0 as xkb_mod_mask_t;
    }
}

#[inline]
pub unsafe fn format_max_overlays(mut format: xkb_keymap_format) -> xkb_overlay_index_t {
    return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as i32 as u32 {
        XKB_OVERLAY_MAX_X11 as usize
    } else {
        XKB_OVERLAY_MAX as usize
    }) as xkb_overlay_index_t;
}

#[inline]
pub unsafe fn format_max_groups(mut format: xkb_keymap_format) -> xkb_layout_index_t {
    return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as i32 as u32 {
        XKB_MAX_GROUPS_X11
    } else {
        XKB_MAX_GROUPS
    }) as xkb_layout_index_t;
}

#[inline]
pub unsafe fn format_boolean_controls(mut format: u32) -> xkb_action_controls {
    return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as i32 as u32 {
        CONTROL_ALL_BOOLEAN_V1 as i32
    } else {
        CONTROL_ALL_BOOLEAN as i32
    }) as xkb_action_controls;
}

#[inline]
pub unsafe fn isModsUnLockOnPressSupported(mut format: xkb_keymap_format) -> bool {
    return format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as i32 as u32;
}

#[inline]
pub unsafe fn isGroupLockOnReleaseSupported(mut format: xkb_keymap_format) -> bool {
    return format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as i32 as u32;
}

#[inline]
pub unsafe fn isModsLatchOnPressSupported(mut format: xkb_keymap_format) -> bool {
    return format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as i32 as u32;
}

#[inline]
pub unsafe fn areOverlappingOverlaysSupported(mut format: xkb_keymap_format) -> bool {
    return format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as i32 as u32;
}

// Error codes (from xkbcommon_errors_h)
pub type xkb_error_code = i32;
pub const XKB_ERROR_ABI_BACKWARD_COMPAT: xkb_error_code = 914;
pub const XKB_ERROR_ABI_FORWARD_COMPAT: xkb_error_code = 876;
pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE: xkb_error_code = 450;
pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS: xkb_error_code = 371;
pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX: xkb_error_code = 237;
pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY: xkb_error_code = 214;
pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK: xkb_error_code = 60;
pub const XKB_KEY_NoSymbol: i32 = 0;

pub const XKB_SUCCESS: xkb_error_code = 0;
pub const XKB_ERROR_INVALID: xkb_error_code = -1;

// ── struct_timespec_h ─────────────────────────────────────────────────
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

// ── struct_stat_h ─────────────────────────────────────────────────────
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
}

// ── bits_stat_h ───────────────────────────────────────────────────────
pub const __S_IFMT: i32 = 0o170000;

// ── errno_base_h ──────────────────────────────────────────────────────
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const ENOTDIR: i32 = 20;

// ── locale_h ──────────────────────────────────────────────────────────
pub const __LC_CTYPE: i32 = 0;
pub const __LC_ALL: i32 = 6;

// ── include_locale_h ──────────────────────────────────────────────────
pub const LC_CTYPE: i32 = __LC_CTYPE;
pub const LC_ALL: i32 = __LC_ALL;

// ── unistd_h ──────────────────────────────────────────────────────────
pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;
pub const R_OK: i32 = 4;
pub const X_OK: i32 = 1;

// ── fcntl_linux_h ─────────────────────────────────────────────────────
pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 0o1;

// ── dirent_h ──────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [i8; 256],
}

// NOTE: DIR type alias and __dirstream extern type are in utils.rs
// (because __dirstream is an extern type that must be declared alongside its extern block)
