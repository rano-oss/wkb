//! Test-only utilities for WKB integration tests.
//! Not part of the public API — use `wkb::testing::*` in test files only.

pub use crate::composer::{ComposeState, Composer, Token};
use crate::flat_keymap::MAX_LEVELS;
pub use crate::modifiers::{ModType, Modifiers};
use crate::xkb;
pub use crate::WKB;

// Re-export modifier constants and helpers needed by integration tests.
pub use crate::modifiers::{level_index, KeyDirection, ALTGR, CAPS_LOCK, NUM_LOCK, SCROLL_LOCK};

// Re-export compose parsing utilities needed by compose tests.
pub mod compose_parse {
    pub use crate::xkb::load_compose_from_path;
    pub use xkb_core::compose::{
        keysym_name_to_char, parse_compose_file, resolve_compose_file, ComposeEntry,
    };
}

/// Feed a token to a composer (wraps the `pub(crate)` method for tests).
pub fn composer_feed(composer: &mut Composer, token: Token) -> ComposeState {
    composer.feed(token)
}

/// Get all available layout variants for a given locale (test utility).
pub fn get_all_layouts_for_locale(locale: &str) -> Vec<String> {
    use xkb_core::rust_types::RxkbContext;

    let mut ctx = match RxkbContext::new() {
        Some(ctx) => ctx,
        None => return vec![String::new()],
    };

    ctx.include_path_append_default();

    if !ctx.parse("evdev") {
        return vec![String::new()];
    }

    let mut layouts = Vec::new();

    for layout in ctx.layouts() {
        let layout_name = layout.name();
        if !layout_name.is_empty() && layout_name == locale {
            let variant = layout.variant();
            if variant.is_empty() {
                layouts.push(String::new());
            } else {
                layouts.push(variant.to_string());
            }
        }
    }

    if layouts.is_empty() {
        layouts.push(String::new());
    }

    layouts
}

pub trait WKBTestExt {
    fn active_mod_type(&self, mod_type: ModType) -> bool;
    fn modifiers(&self) -> &Modifiers;
    fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)>;
    fn level2_code(&self) -> Option<(u32, Option<u8>)>;
    fn level3_code(&self) -> Option<(u32, Option<u8>)>;
    fn level5_code(&self) -> Option<(u32, Option<u8>)>;
    fn update_key(&mut self, evdev_code: u32, key_direction: crate::KeyDirection) -> bool;
    fn key_char(&self, evdev_code: u32) -> Option<char>;
    fn composer(&self) -> &Composer;
    fn num_levels(&self) -> usize;
    fn producible_chars(&self) -> std::collections::HashSet<char>;
    fn pending(&self) -> &[Token];
    fn feed(&mut self, token: Token) -> ComposeState;
}

impl WKBTestExt for WKB {
    fn active_mod_type(&self, mod_type: ModType) -> bool {
        self.modifiers.active_mod_type(mod_type)
    }

    fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)> {
        xkb::level_code(&self.modifiers, mod_type)
    }

    fn level2_code(&self) -> Option<(u32, Option<u8>)> {
        xkb::level2_code(&self.modifiers)
    }

    fn level3_code(&self) -> Option<(u32, Option<u8>)> {
        xkb::level3_code(&self.modifiers)
    }

    fn level5_code(&self) -> Option<(u32, Option<u8>)> {
        xkb::level5_code(&self.modifiers)
    }

    fn update_key(&mut self, evdev_code: u32, key_direction: crate::KeyDirection) -> bool {
        self.update_key(evdev_code, key_direction)
    }

    fn key_char(&self, evdev_code: u32) -> Option<char> {
        self.key_char(evdev_code)
    }

    fn composer(&self) -> &Composer {
        &self.composer
    }

    fn num_levels(&self) -> usize {
        MAX_LEVELS
    }

    fn producible_chars(&self) -> std::collections::HashSet<char> {
        let mut chars = std::collections::HashSet::new();
        let num_layouts = self.layout_names.len();
        for li in 0..num_layouts {
            for lvl in 0..MAX_LEVELS {
                for k in 0..self.state_keymap.num_keys as u32 {
                    if let Some(ch) = self.state_keymap.get(li, lvl, k) {
                        chars.insert(ch);
                    }
                }
            }
        }
        chars
    }

    fn pending(&self) -> &[Token] {
        &self.composer.pending
    }

    fn feed(&mut self, token: Token) -> ComposeState {
        self.composer.feed(token)
    }
}
