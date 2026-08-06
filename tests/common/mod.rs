//! Shared helpers for integration tests
#![allow(dead_code)]

use xkbcommon::xkb;

/// Build an xkbcommon `State` for the given locale and layout variant.
/// An empty `layout` string selects the default variant.
pub fn xkb_new_from_names(locale: &str, layout: &str) -> xkb::State {
    let keymap = xkb_new_keymap_from_names(locale, layout);
    xkb::State::new(&keymap)
}

/// Build an xkbcommon `Keymap` for the given locale and layout variant.
/// An empty `layout` string selects the default variant.
pub fn xkb_new_keymap_from_names(locale: &str, layout: &str) -> xkb::Keymap {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        locale,
        layout,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap()
}
