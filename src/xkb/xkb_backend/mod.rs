#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub(crate) mod keymap;
pub(crate) mod keysym;
pub(crate) mod shared_types;
pub(crate) mod xkbcomp;

pub use shared_types::{XKB_KEY_DOWN, XKB_KEY_UP};
