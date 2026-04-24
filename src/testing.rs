//! Test-only utilities for WKB integration tests.
//! Not part of the public API — use `wkb::testing::*` in test files only.

pub use crate::composer::{ComposeState, ListComposer, Token};
pub use crate::modifiers::{ModType, Modifiers};
use crate::xkb;
pub use crate::WKB;

// Re-export modifier constants and helpers needed by integration tests.
pub use crate::modifiers::{level_index, ALTGR, CAPS_LOCK, NUM_LOCK, SCROLL_LOCK};

// Re-export compose parsing utilities needed by compose tests.
pub mod compose_parse {
    pub use crate::xkb::compose_parse::*;
}

pub trait WKBTestExt {
    fn active_mod_type(&self, mod_type: ModType) -> bool;
    fn modifiers(&self) -> &Modifiers;
    fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)>;
    fn level2_code(&self) -> Option<(u32, Option<u8>)>;
    fn level3_code(&self) -> Option<(u32, Option<u8>)>;
    fn level5_code(&self) -> Option<(u32, Option<u8>)>;
    fn update_key(&mut self, evdev_code: u32, key_direction: crate::KeyDirection) -> bool;
    fn utf8(&mut self, evdev_code: u32) -> Option<char>;
    fn composer(&self) -> &ListComposer;
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

    fn utf8(&mut self, evdev_code: u32) -> Option<char> {
        self.utf8(evdev_code)
    }

    fn composer(&self) -> &ListComposer {
        &self.composer
    }
}

pub trait ListComposerTestExt {
    fn feed(&mut self, token: Token) -> ComposeState;
}

impl ListComposerTestExt for ListComposer {
    fn feed(&mut self, token: Token) -> ComposeState {
        self.feed(token)
    }
}
