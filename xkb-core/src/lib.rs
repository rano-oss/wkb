#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]

//! # xkb-core
//!
//! Pure Rust implementation of XKB keymap compilation, keyboard state
//! management, compose table support, and keysym utilities.
//!
//! This crate is the engine behind [`wkb`](https://crates.io/crates/wkb)
//! and is not typically used directly. It provides:
//!
//! - XKB keymap parsing and compilation (RMLVO rules, keymap strings)
//! - Keyboard state tracking (modifiers, groups, LEDs)
//! - Compose / dead-key table parsing and matching
//! - Keysym ↔ UTF-8 conversion and case mapping
//! - XKB registry (RMLVO database) queries

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

// Keymap serialization
pub(crate) mod serialize;

// Public API modules
pub mod compose;
pub mod keysym_utf;
pub mod rust_types;
pub(crate) mod shared_types;

// Re-export only the externally-needed constants from shared_types
pub use shared_types::{XKB_KEY_DOWN, XKB_KEY_REPEATED, XKB_KEY_UP};

/// Path to XKB symbols directory
pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";
