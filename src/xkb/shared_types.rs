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

// ── Opaque types ────────────────────────────────────────────────────

// atom_table is defined in atom.rs — re-export it here for unified access
pub use crate::xkb::atom::atom_table;

// ── xkb_context ─────────────────────────────────────────────────────

use c2rust_bitfields::BitfieldStruct;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct xkb_context {
    pub refcnt: i32,
    pub log_fn: Option<
        unsafe extern "C" fn(*mut xkb_context, xkb_log_level, *const i8, ::core::ffi::VaList) -> (),
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

// ── __va_list_tag ───────────────────────────────────────────────────

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
