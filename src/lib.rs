//! # wkb — Wayland Keyboard
//!
//! A lightweight, pure Rust keyboard handling library for Wayland.
//! WKB compiles XKB keymaps, tracks modifier and compose state, and maps
//! evdev key codes to characters — all without C dependencies.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use wkb::WKB;
//!
//! // Build from an XKB keymap string (e.g. received from a Wayland compositor)
//! let keymap_string = std::fs::read_to_string("/path/to/keymap").unwrap();
//! let mut wkb = WKB::new_from_string(&keymap_string).unwrap();
//!
//! // Process a key press (evdev code 38 = 'a' on US layout)
//! let result = wkb.press_key(38);
//! ```
//!
//! ## Feature Flags
//!
//! - **`xkb`** (default) — XKB keymap compilation via the `xkb-core` crate.
//! - **`compose`** (default) — Compose-key / dead-key sequence support.
//! - **`testing`** — Exposes internal helpers for integration tests. Not part of the public API.

use std::fmt;

pub use composer::ComposeState;
use composer::{Composer, Token};
mod composer;
mod modifiers;
use modifiers::{
    level_index, KeyDirection, ModKind, ModType, Modifier, Modifiers, CAPS_LOCK, MODIFIER_MAPPING,
    MOD_ALT, MOD_CAPS_LOCK, MOD_CTRL, MOD_LOGO, MOD_NUM_LOCK, MOD_SHIFT, NUM_LOCK, SCROLL_LOCK,
};
pub use modifiers::{ModifiersState, LED_CAPS_LOCK, LED_NUM_LOCK, LED_SCROLL_LOCK};
mod bitset;
pub(crate) use bitset::KeyBitSet;
mod flat_keymap;
pub(crate) use flat_keymap::{FlatKeymap, FlatKeysymMap};
pub mod keysyms;
pub use keysyms::Keysym;
/// Test-only utilities. Not part of the public API.
#[cfg(feature = "testing")]
pub mod testing;
#[cfg(feature = "xkb")]
mod xkb;
#[cfg(feature = "xkb")]
pub use xkb::XkbError;

/// Errors from WKB operations (not related to XKB parsing/compilation).
#[derive(Debug)]
pub enum WkbError {
    /// Layout index out of range.
    InvalidLayout(usize),
}

impl fmt::Display for WkbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WkbError::InvalidLayout(idx) => write!(f, "Invalid layout index: {}", idx),
        }
    }
}

impl std::error::Error for WkbError {}

/// Core keyboard state machine. Tracks modifier state, key presses, and compose sequences.
#[derive(Debug, Clone)]
pub struct WKB {
    pub(crate) repeat_keys: KeyBitSet,
    pub(crate) composer: Composer,
    pub(crate) modifiers: Modifiers,
    pub(crate) state_keymap: FlatKeymap,
    pub(crate) num_lock_keys: FlatKeymap,
    pub(crate) caps_lock_keymap: FlatKeymap,
    /// Index of the currently active layout (0-based).
    pub(crate) current_layout_idx: usize,
    /// Human-readable names for each layout in the keymap.
    pub(crate) layout_names: Vec<String>,
    /// Flat keysym lookup table, same indexing as state_keymap.
    pub(crate) keysym_map: FlatKeysymMap,
    #[cfg(feature = "xkb")]
    pub(crate) level_exceptions_keymap: FlatKeymap,
}

// WKB is Send + Sync: all fields are owned, no Rc/RefCell.
// The Rc<xkb_keymap> from xkb-core is only used during construction and not stored.
unsafe impl Send for WKB {}
unsafe impl Sync for WKB {}

#[cfg(feature = "xkb")]
impl WKB {
    /// Create WKB instance from RMLVO names, matching xkbcommon's `xkb_keymap_new_from_names`.
    ///
    /// `layout` and `variant` are comma-separated lists (e.g. `"us,fr"`, `"dvorak,azerty"`).
    pub fn new_from_names(
        rules: &str,
        model: &str,
        layout: &str,
        variant: &str,
        options: Option<&str>,
    ) -> Result<Self, XkbError> {
        xkb::new_from_names(rules, model, layout, variant, options)
    }

    /// Create WKB instance from an XKB keymap string (v1 text format).
    pub fn new_from_string(keymap: &str) -> Result<Self, XkbError> {
        xkb::new_from_string(keymap)
    }
}

impl WKB {
    /// Reset all transient input state: compose sequence.
    /// Call on wl_keyboard.leave or when focus changes.
    pub fn reset_state(&mut self) {
        self.composer.reset();
    }

    /// Return the current modifier state.
    ///
    /// The returned [`ModifiersState`] contains both high-level boolean fields
    /// (`ctrl`, `alt`, `shift`, etc.) and raw bitmasks (`depressed`, `latched`,
    /// `locked`, `layout`) suitable for `wl_keyboard.modifiers`.
    pub fn modifiers_state(&self) -> ModifiersState {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let layout = self.current_layout_idx as u32;
        for (code, bit) in MODIFIER_MAPPING {
            if let Some(Modifier::Single(mk)) = self.modifiers.get(code) {
                match mk {
                    ModKind::Pressed { pressed: true, .. } => depressed |= bit,
                    ModKind::Lock {
                        pressed, locked: l, ..
                    } => {
                        if *pressed {
                            depressed |= bit;
                        }
                        if *l > 0 {
                            locked |= bit;
                        }
                    }
                    ModKind::Latch {
                        pressed,
                        latched: is_latched,
                        ..
                    } => {
                        if *pressed {
                            depressed |= bit;
                        }
                        if *is_latched {
                            latched |= bit;
                        }
                    }
                    _ => {}
                }
            }
        }
        let effective = depressed | latched | locked;
        ModifiersState {
            ctrl: (effective & MOD_CTRL) != 0,
            alt: (effective & MOD_ALT) != 0,
            shift: (effective & MOD_SHIFT) != 0,
            caps_lock: (effective & MOD_CAPS_LOCK) != 0,
            logo: (effective & MOD_LOGO) != 0,
            num_lock: (effective & MOD_NUM_LOCK) != 0,
            depressed,
            latched,
            locked,
            layout,
        }
    }

    /// Return the LED indicator state as a bitmask.
    ///
    /// Use [`LED_NUM_LOCK`], [`LED_CAPS_LOCK`], [`LED_SCROLL_LOCK`] to test bits.
    pub fn leds_state(&self) -> u32 {
        let mut leds = 0;
        if self.modifiers.locked_with_type(NUM_LOCK, ModType::Num) {
            leds |= LED_NUM_LOCK;
        }
        if self.modifiers.locked_with_type(CAPS_LOCK, ModType::Caps) {
            leds |= LED_CAPS_LOCK;
        }
        if self
            .modifiers
            .locked_with_type(SCROLL_LOCK, ModType::Scroll)
        {
            leds |= LED_SCROLL_LOCK;
        }
        leds
    }

    /// Apply modifier state received from `wl_keyboard.modifiers`.
    ///
    /// The `group` parameter selects the active layout index.
    pub fn update_modifiers(&mut self, depressed: u32, latched: u32, locked: u32, group: u32) {
        // Switch layout based on group index from compositor.
        if (group as usize) < self.num_layouts() {
            let _ = self.set_layout(group as usize);
        }
        for (code, bit) in MODIFIER_MAPPING {
            let is_depressed = (depressed & bit) != 0;
            let is_locked = (locked & bit) != 0;
            let is_latched = (latched & bit) != 0;

            if let Some(m) = self.modifiers.get_mut(code) {
                if let Modifier::Single(mk) = m {
                    match mk {
                        ModKind::Pressed { pressed, .. } => *pressed = is_depressed,
                        ModKind::Lock {
                            pressed, locked, ..
                        } => {
                            *pressed = is_depressed;
                            *locked = if is_locked { 1 } else { 0 };
                        }
                        ModKind::Latch {
                            pressed, latched, ..
                        } => {
                            *pressed = is_depressed;
                            *latched = is_latched;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Return whether the given evdev keycode is a repeating key.
    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.contains(evdev_code)
    }

    /// Return the number of layouts in this keymap.
    pub fn num_layouts(&self) -> usize {
        self.layout_names.len()
    }

    /// Return the index of the currently active layout.
    pub fn active_layout_idx(&self) -> usize {
        self.current_layout_idx
    }

    /// Switch to a different layout by index.
    pub fn set_layout(&mut self, layout_idx: usize) -> Result<(), WkbError> {
        if layout_idx >= self.layout_names.len() {
            return Err(WkbError::InvalidLayout(layout_idx));
        }
        self.current_layout_idx = layout_idx;
        Ok(())
    }

    /// Return the name of the layout at the given index.
    pub fn layout_name(&self, layout_idx: usize) -> Option<&str> {
        self.layout_names.get(layout_idx).map(|s| s.as_str())
    }

    /// Serialize the underlying XKB keymap to v1 text format.
    ///
    /// Generates the string on demand from the flat keysym tables.
    /// Returns the generated XKB v1 keymap string.
    #[cfg(feature = "xkb")]
    pub fn as_xkb_string(&self) -> Option<String> {
        Some(self.generate_xkb_string())
    }

    /// Get the keysym for an evdev keycode under the current modifier state.
    /// Returns `0` (NoSymbol) if no keysym is mapped.
    pub fn state_keysym(&self, evdev_code: u32) -> u32 {
        let (none_active, level2, level3, level5) = self.modifiers.active_none_and_levels();
        if none_active {
            return 0;
        }
        let nk = self.keysym_map.num_keys;
        let level5 = level5 && self.keysym_map.data.len() > 4 * nk;
        let level3 = level3 && self.keysym_map.data.len() > 2 * nk;
        let level2 = level2 && self.keysym_map.data.len() > 1 * nk;
        let level = level_index(level5, level3, level2);
        self.keysym_map
            .get(self.current_layout_idx, level, evdev_code)
    }

    /// Get the keysym at a specific layout and level for an evdev keycode.
    /// Bypasses current modifier state.
    /// Returns `0` (KEY_NoSymbol) if no keysym is mapped.
    pub fn level_keysym(&self, evdev_code: u32, layout: usize, level: usize) -> u32 {
        self.keysym_map.get(layout, level, evdev_code)
    }

    /// Get the character at a specific layout and level for an evdev keycode.
    /// Bypasses current modifier state.
    /// Does not consider caps lock or num lock overrides.
    pub fn level_key_char(&self, evdev_code: u32, layout: usize, level: usize) -> Option<char> {
        #[cfg(feature = "xkb")]
        if let Some(exception_char) = self.level_exceptions_keymap.get(layout, level, evdev_code) {
            return Some(exception_char);
        }
        self.state_keymap.get(layout, level, evdev_code)
    }

    /// Resolve the character for the given evdev keycode under the current modifier state.
    /// This is a pure lookup with no side effects — it does not update modifier state
    /// or advance compose sequences. Use this for:
    /// - `text_with_all_modifiers` (winit): the raw character including all modifier effects
    /// - Re-resolving characters when modifiers change during key repeat
    #[inline]
    pub fn key_char(&self, evdev_code: u32) -> Option<char> {
        let (none_active, level2, level3, level5) = self.modifiers.active_none_and_levels();
        if none_active {
            return None;
        }
        let nk = self.state_keymap.num_keys;
        let level5 = level5 && self.state_keymap.data.len() > 4 * nk;
        let level3 = level3 && self.state_keymap.data.len() > 2 * nk;
        let level2 = level2 && self.state_keymap.data.len() > 1 * nk;
        let base_level = level_index(level5, level3, level2);
        let layout_index = self.current_layout_idx;

        if self.modifiers.locked(NUM_LOCK) {
            if let Some(key) = self.num_lock_keys.get(layout_index, base_level, evdev_code) {
                return Some(key);
            }
        }

        if self.modifiers.locked(CAPS_LOCK) {
            if let Some(c) = self
                .caps_lock_keymap
                .get(layout_index, base_level, evdev_code)
            {
                return Some(c);
            }
        }

        self.state_keymap.get(layout_index, base_level, evdev_code)
    }

    /// Update internal modifier state for a key event. Returns `true` if the key is a modifier.
    pub(crate) fn update_key(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let is_modifier = self.modifiers.set_state(evdev_code, key_direction);
        if !is_modifier && key_direction == KeyDirection::Down {
            self.modifiers.unlatch();
        }
        is_modifier
    }

    /// Process a key press. Updates modifier state and advances compose sequences.
    ///
    /// Returns a [`KeyResult`] with the keysym, compose state, and whether the key is a modifier.
    /// Extract the final character from the [`ComposeState`] variant:
    /// - `Idle(char)` — no compose active, this is the character
    /// - `Finished(char)` — compose sequence completed, this is the composed character
    /// - `Composing(_)` — mid-sequence, no final character yet
    /// - `Cancelled` — broken compose sequence
    pub fn press_key(&mut self, evdev_code: u32) -> KeyResult {
        let is_modifier = self.update_key(evdev_code, KeyDirection::Down);
        let keysym = self.state_keysym(evdev_code);
        #[cfg(feature = "compose")]
        let compose = if is_modifier && self.modifiers.active_mod_type(ModType::Compose) {
            Some(self.composer.feed(Token::Compose))
        } else if !is_modifier {
            self.key_char(evdev_code)
                .map(|c| self.composer.feed(Token::Char(c)))
        } else {
            None
        };
        #[cfg(not(feature = "compose"))]
        let compose = if !is_modifier {
            self.key_char(evdev_code).map(ComposeState::Idle)
        } else {
            None
        };

        KeyResult {
            keysym,
            compose,
            is_modifier,
        }
    }

    /// Process a key release. Updates modifier state.
    ///
    /// Compose is not advanced on release. The returned `keysym` reflects
    /// the keysym under the (now updated) modifier state.
    pub fn release_key(&mut self, evdev_code: u32) -> KeyResult {
        let is_modifier = self.update_key(evdev_code, KeyDirection::Up);
        let keysym = self.state_keysym(evdev_code);
        KeyResult {
            keysym,
            compose: None,
            is_modifier,
        }
    }

    /// Process a key repeat. Does NOT update modifier state or advance compose.
    ///
    /// Returns the keysym and raw character under the current modifier state.
    /// The compose field will be `Some(Idle(char))` if a character is available,
    /// or `None` for modifier keys or keys with no character mapping.
    pub fn repeat_key(&self, evdev_code: u32) -> KeyResult {
        let keysym = self.state_keysym(evdev_code);
        let ch = self.key_char(evdev_code);
        let compose = ch.map(ComposeState::Idle);
        KeyResult {
            keysym,
            compose,
            is_modifier: false,
        }
    }
}

/// Result of a key event processed by [`WKB::press_key`], [`WKB::release_key`], or [`WKB::repeat_key`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyResult {
    /// The keysym for this key under the current modifier state.
    /// `0` (NoSymbol) if no keysym is mapped or the `xkb` feature is disabled.
    pub keysym: u32,

    /// The compose state after processing this key.
    ///
    /// - `None` — no character produced (modifier key, release event, or unmapped key)
    /// - `Some(Idle(char))` — no compose active, `char` is the direct character
    /// - `Some(Composing(seq))` — mid-compose sequence, `seq` is the sequence so far
    /// - `Some(Finished(char))` — compose completed, `char` is the composed character
    /// - `Some(Cancelled)` — compose sequence was broken
    pub compose: Option<ComposeState>,

    /// Whether the key is a modifier (Shift, Ctrl, Alt, etc.).
    pub is_modifier: bool,
}
