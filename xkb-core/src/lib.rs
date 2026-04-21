#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! XKB Core — pure Rust XKB implementation.
//!
//! This crate provides complete XKB keymap compilation, state management,
//! compose table support, and keysym handling.

// Core XKB modules
pub mod atom;
pub mod context;
pub mod features;
pub mod keymap;
pub mod shared_ast_types;
pub mod shared_types;
pub mod state;

// Keysym modules
pub mod keysym;
pub mod keysym_case_mappings;
pub mod keysym_utf;

// Parsing and text processing
pub mod scanner_utils;
pub mod text;

// Utilities
pub mod utf8_decoding;
pub mod utils;

// Compilation modules
pub mod xkbcomp;

// Compose support
pub mod compose;

// Rules and registry
pub mod registry;

// Messages and logging
pub mod messages;

// Rust-native wrapper types
pub mod rust_types;

/// Path to XKB symbols directory
pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";
