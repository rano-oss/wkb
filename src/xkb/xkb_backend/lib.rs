#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

mod atom;
mod compose;
mod context;
mod keymap;
mod keysym;
mod keysym_case_mappings;
mod keysym_utf;
mod messages;
mod registry;
mod rust_types;
pub mod rust_types;
mod scanner_utils;
mod serialize;
mod shared_ast_types;
mod shared_types;
pub(crate) mod shared_types;
mod state;
mod text;
mod utf8_decoding;
mod utils;
mod xkbcomp;

// Re-export only the externally-needed constants from shared_types
pub use shared_types::{XKB_KEY_DOWN, XKB_KEY_REPEATED, XKB_KEY_UP};

/// Path to XKB symbols directory
pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";
