#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! XKB Core — pure Rust XKB implementation.
//!
//! This crate provides complete XKB keymap compilation, state management,
//! compose table support, and keysym handling.

// Core XKB modules (internal)
pub(crate) mod atom;
pub(crate) mod context;
pub(crate) mod features;
pub(crate) mod keymap;
pub(crate) mod shared_ast_types;
pub(crate) mod state;

// Keysym modules (internal)
pub(crate) mod keysym;
pub(crate) mod keysym_case_mappings;

// Parsing and text processing (internal)
pub(crate) mod scanner_utils;
pub(crate) mod text;

// Utilities (internal)
pub(crate) mod utf8_decoding;
pub(crate) mod utils;

// Compilation modules (internal)
pub(crate) mod xkbcomp;

// Rules and registry (internal)
pub(crate) mod registry;

// Messages and logging (internal)
pub(crate) mod messages;

// Public API modules
pub mod compose;
pub mod keysym_utf;
pub mod rust_types;
pub(crate) mod shared_types;

// Re-export only the externally-needed constants from shared_types
pub use shared_types::{XKB_KEY_DOWN, XKB_KEY_REPEATED, XKB_KEY_UP};

/// Path to XKB symbols directory
pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";
