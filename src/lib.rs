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
//! let keymap_string = std::fs::read_to_string("/path/to/keymap").unwrap();
//! let mut wkb = WKB::new_from_string(&keymap_string).unwrap();
//!
//! # #[cfg(feature = "client")]
//! {
//!     wkb.update_modifiers(0, 0, 0, 0);
//!     let physical = wkb.physical_key(30);
//!     let named = wkb.named_key(30);
//!     let character = wkb.key_char(30);
//!     let compose = wkb.compose(30);
//!     println!(
//!         "physical={physical:?} named={named:?} char={character:?} compose={compose:?}"
//!     );
//! }
//!
//! # #[cfg(feature = "compositor")]
//! {
//!     let changes = wkb.press_key(30);
//!     assert!(!changes.leds_updated);
//! }
//! ```
//!
//! ## Key Event API
//!
//! | Method | Role | Use case |
//! |--------|------|----------|
//! | [`WKB::press_key`] | compositor | Key down — updates modifier/group state |
//! | [`WKB::release_key`] | compositor | Key up — updates modifier/group state |
//! | [`WKB::compose`] | client | Feed a key into compose processing |
//! | [`WKB::update_modifiers`] | client | Apply `wl_keyboard.modifiers` |
//!
//! Key state mutation, key identity, character lookup, and compose processing
//! are intentionally separate. Public keycodes are always raw Linux/evdev codes.
//!
//! ## Feature Flags
//!
//! - **`xkb`** — XKB keymap compilation (enabled by default via `client`).
//! - **`compositor`** — [`WKB::press_key`] / [`WKB::release_key`] update modifiers,
//!   groups, and LEDs (Smithay). Mutually exclusive with `client`.
//! - **`client`** (default) — Compose trie, [`WKB::compose`], [`WKB::leave`].
//!   Use [`WKB::update_modifiers`] from the compositor; no `press_key` on clients.

#[cfg(all(feature = "compositor", feature = "client", not(feature = "full")))]
compile_error!(
    "features `compositor` and `client` are mutually exclusive; enable exactly one (or `full` for tests)"
);

#[cfg(not(any(feature = "compositor", feature = "client")))]
compile_error!("enable either the `compositor` or `client` feature");

use crate::modifiers::*;
use composer::Composer;
#[cfg(feature = "client")]
pub use composer::{ComposeState, ComposeString};
#[cfg(feature = "client")]
use composer::Token;
mod composer;
mod flat_keymap;
mod groups;
mod modifiers;
mod physical_keys;
pub(crate) use flat_keymap::{FlatKeymap, FlatNamedKeyMap};
pub use groups::{Group, GroupChange, GroupKind, Groups};
pub use modifiers::{
    level_index, KeyDirection, ModType, ALTGR, CAPS_LOCK, LEFT_SHIFT, NUM_LOCK, RIGHT_SHIFT,
    SCROLL_LOCK, LedState
};
pub use physical_keys::PhysicalKey;
/// Intermediate representation for persisted layout data files.
pub mod ir;
mod named_keys;
pub use named_keys::NamedKey;
#[cfg(feature = "xkb")]
mod xkb;
#[cfg(feature = "xkb")]
pub use xkb::XkbError;
#[cfg(feature = "xkb")]
#[doc(hidden)]
pub use xkb::{keysym_to_named_key, load_compose_from_path, load_compose_from_path_uncached};
pub(crate) const BITSET_WORDS: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub(crate) struct KeyBitSet {
    bits: [u64; BITSET_WORDS],
}

impl KeyBitSet {
    pub(crate) fn contains(&self, key: u32) -> bool {
        let k = key as usize;
        if k < BITSET_WORDS * 64 {
            self.bits[k >> 6] & (1u64 << (k & 63)) != 0
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn insert(&mut self, key: u32) -> bool {
        let k = key as usize;
        if k >= BITSET_WORDS * 64 {
            return false;
        }
        let mask = 1u64 << (k & 63);
        let word = &mut self.bits[k >> 6];
        let present = *word & mask != 0;
        *word |= mask;
        !present
    }

    #[inline]
    pub(crate) fn remove(&mut self, key: u32) -> bool {
        let k = key as usize;
        if k >= BITSET_WORDS * 64 {
            return false;
        }
        let mask = 1u64 << (k & 63);
        let word = &mut self.bits[k >> 6];
        let present = *word & mask != 0;
        *word &= !mask;
        present
    }
}

/// Errors from WKB operations (not related to XKB parsing/compilation).
#[derive(Debug, thiserror::Error)]
pub enum WkbError {
    /// Layout index out of range.
    #[error("Invalid layout index: {0}")]
    InvalidLayout(usize),
}

/// Core keyboard state machine. Tracks modifier state, key presses, and compose sequences.
#[derive(Debug, Clone)]
pub struct KBLayout {
    pub(crate) name: String,
    pub(crate) repeat_keys: KeyBitSet,
    pub(crate) composer: Composer,
    pub(crate) modifiers: Modifiers,
    pub(crate) state_keymap: FlatKeymap,
    pub(crate) num_lock_keys: FlatKeymap,
    pub(crate) caps_lock_keymap: FlatKeymap,
    /// Overrides active while BOTH Num Lock and Caps Lock are locked.
    pub(crate) caps_num_lock_keys: FlatKeymap,
    pub(crate) named_key_map: FlatNamedKeyMap,
    #[cfg(feature = "xkb")]
    pub(crate) level_exceptions_keymap: FlatKeymap,
}

/// Core keyboard state machine. Tracks modifier state, key presses, and compose sequences.
#[derive(Debug, Clone)]
pub struct WKB {
    pub(crate) layouts: Vec<KBLayout>,
    pub(crate) current_layout_idx: usize,
    pub(crate) groups: Groups,
}

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
    /// Clear in-progress compose on the active layout (e.g. `wl_keyboard.leave`).
    #[cfg(feature = "client")]
    pub fn leave(&mut self) {
        self.layouts[self.current_layout_idx].composer.reset();
    }

    /// Return the raw modifier bitmasks for `wl_keyboard.modifiers`.
    ///
    /// Returns depressed, latched, locked bitmasks and the active layout index.
    pub fn raw_modifiers(&self) -> RawModifiers {
        self.layouts[self.current_layout_idx]
            .modifiers
            .state(self.current_layout_idx)
    }

    /// Return `true` if the Shift modifier is active.
    pub fn shift(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_SHIFT != 0
    }

    /// Return `true` if the Control modifier is active.
    pub fn ctrl(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_CTRL != 0
    }

    /// Return `true` if the Alt modifier is active.
    pub fn alt(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_ALT != 0
    }

    /// Return `true` if the Logo (Super/Windows) modifier is active.
    pub fn logo(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_LOGO != 0
    }

    /// Return `true` if Caps Lock is active.
    pub fn caps_lock(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_CAPS_LOCK != 0
    }

    /// Return `true` if Num Lock is active.
    pub fn num_lock(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_NUM_LOCK != 0
    }

    pub fn level3(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_ALTGR != 0
    }

    pub fn level5(&self) -> bool {
        let raw = self.raw_modifiers();
        (raw.depressed | raw.latched | raw.locked) & modifiers::MOD_SCROLL_LOCK != 0
    }

    /// Apply modifier state received from `wl_keyboard.modifiers`.
    ///
    /// The `group` parameter selects the active layout index when it is valid.
    /// Returns whether the externally observable raw modifiers or LED state
    /// actually changed. An invalid group that causes no effective change
    /// reports neither flag.
    pub fn update_modifiers(
        &mut self,
        depressed: u32,
        latched: u32,
        locked: u32,
        group: u32,
    ) -> StateChanges {
        let before_mods = self.raw_modifiers();
        let before_leds = self.leds_state();
        if (group as usize) < self.num_layouts() {
            let new_layout = group as usize;
            #[cfg(feature = "client")]
            if new_layout != self.current_layout_idx {
                self.layouts[self.current_layout_idx].composer.reset();
            }
            self.groups.set_layout(new_layout, self.num_layouts());
            self.current_layout_idx = new_layout;
        }
        self.layouts[self.current_layout_idx]
            .modifiers
            .update(depressed, latched, locked);
        StateChanges {
            is_modifier: false,
            modifiers_updated: self.raw_modifiers() != before_mods,
            leds_updated: self.leds_state() != before_leds,
        }
    }

    /// Physical key position for a raw evdev keycode.
    ///
    /// Independent of layout, modifiers, remapping, and compose state.
    #[inline]
    pub fn physical_key(&self, evdev_code: u32) -> PhysicalKey {
        PhysicalKey::from_evdev(evdev_code)
    }

    /// Return the LED indicator state.
    pub fn leds_state(&self) -> LedState {
        self.layouts[self.current_layout_idx].modifiers.leds_state()
    }

    /// Return whether the given evdev keycode is a repeating key.
    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.layouts[self.current_layout_idx]
            .repeat_keys
            .contains(evdev_code)
    }

    /// Return the number of layouts in this keymap.
    pub fn num_layouts(&self) -> usize {
        self.layouts.len()
    }

    /// Return the index of the currently active layout.
    pub fn active_layout_idx(&self) -> usize {
        self.current_layout_idx
    }

    /// Switch to a different layout by index.
    pub fn set_layout(&mut self, layout_idx: usize) -> Result<(), WkbError> {
        if layout_idx >= self.layouts.len() {
            return Err(WkbError::InvalidLayout(layout_idx));
        }
        if layout_idx != self.current_layout_idx {
            let old_kb_layout = &mut self.layouts
            [self.current_layout_idx];
            #[cfg(feature = "client")]
            old_kb_layout.composer.reset();
            let raw = old_kb_layout.modifiers.state(layout_idx);
            self.layouts[layout_idx]
                .modifiers
                .update(raw.depressed, raw.latched, raw.locked);
        }
        self.groups.set_layout(layout_idx, self.num_layouts());
        self.current_layout_idx = layout_idx;
        Ok(())
    }

    /// Return the name of the layout at the given index.
    pub fn layout_name(&self, layout_idx: usize) -> Option<&str> {
        self.layouts.get(layout_idx).map(|s| s.name.as_str())
    }

    /// Serialize the underlying XKB keymap to v1 text format.
    ///
    /// Generates the string on demand from the flat keysym tables.
    /// Returns the generated XKB v1 keymap string.
    #[cfg(feature = "xkb")]
    pub fn as_xkb_string(&self) -> Option<String> {
        Some(self.generate_xkb_string())
    }

    /// Get the named, non-character identity for an evdev keycode under the
    /// current modifier state.
    ///
    /// Returns [`NamedKey::Unnamed`] when the selected key is a character or
    /// has no named mapping. Ctrl/Alt/Logo do not blank named keys. If the
    /// selected level has no named mapping, lower levels are tried so ONE_LEVEL
    /// keys such as Shift and Escape keep their identity.
    pub fn named_key(&self, evdev_code: u32) -> NamedKey {
        let kb_layout = &self.layouts[self.current_layout_idx];
        let (_none_active, level2, level3, level5) = kb_layout.modifiers.active_none_and_levels();
        let nk = kb_layout.named_key_map.num_keys;
        let level5 = level5 && kb_layout.named_key_map.data.len() > 4 * nk;
        let level3 = level3 && kb_layout.named_key_map.data.len() > 2 * nk;
        let level2 = level2 && kb_layout.named_key_map.data.len() > nk;
        let level = level_index(level5, level3, level2);
        for l in (0..=level).rev() {
            let named = kb_layout.named_key_map.get(l, evdev_code);
            if named != NamedKey::Unnamed {
                return named;
            }
        }
        NamedKey::Unnamed
    }

    /// Get the named key at a specific layout and level for an evdev keycode.
    /// Bypasses current modifier state.
    /// Returns [`NamedKey::Unnamed`] if no named key is mapped.
    pub fn level_named_key(&self, evdev_code: u32, layout: usize, level: usize) -> NamedKey {
        self.layouts[layout].named_key_map.get(level, evdev_code)
    }

    /// Get the character at a specific layout and level for an evdev keycode.
    /// Bypasses current modifier state.
    /// Does not consider caps lock or num lock overrides.
    pub fn level_char(&self, evdev_code: u32, layout: usize, level: usize) -> Option<char> {
        #[cfg(feature = "xkb")]
        if let Some(exception_char) = self.layouts[layout]
            .level_exceptions_keymap
            .get(level, evdev_code)
        {
            return Some(exception_char);
        }
        self.layouts[layout].state_keymap.get(level, evdev_code)
    }

    /// Resolve the character for the given evdev keycode under the current modifier state.
    /// This is a pure lookup with no side effects — it does not update modifier state
    /// or advance compose sequences. Use this for:
    /// - `text_with_all_modifiers` (winit): the raw character including all modifier effects
    /// - Re-resolving characters when modifiers change during key repeat
    ///
    /// Returns `None` while Ctrl, Alt, or Logo are active so callers do not treat
    /// shortcut chords as typed text.
    pub fn key_char(&self, evdev_code: u32) -> Option<char> {
        let kb_layout = &self.layouts[self.current_layout_idx];
        let (none_active, level2, level3, level5) = kb_layout.modifiers.active_none_and_levels();
        if none_active {
            return None;
        }
        let nk = kb_layout.state_keymap.num_keys;
        let level5 = level5 && kb_layout.state_keymap.data.len() > 4 * nk;
        let level3 = level3 && kb_layout.state_keymap.data.len() > 2 * nk;
        let level2 = level2 && kb_layout.state_keymap.data.len() > nk;
        let base_level = level_index(level5, level3, level2);
        if kb_layout.modifiers.num_locked() && kb_layout.modifiers.caps_locked() {
            if let Some(c) = kb_layout.caps_num_lock_keys.get(base_level, evdev_code) {
                return Some(c);
            }
        }
        if kb_layout.modifiers.num_locked() {
            if let Some(c) = kb_layout.num_lock_keys.get(base_level, evdev_code) {
                return Some(c);
            }
        }
        if kb_layout.modifiers.caps_locked() {
            if let Some(c) = kb_layout.caps_lock_keymap.get(base_level, evdev_code) {
                return Some(c);
            }
        }
        kb_layout.state_keymap.get(base_level, evdev_code)
    }

    /// Return whether the given modifier type is currently active.
    #[doc(hidden)]
    pub fn active_mod_type(&self, mod_type: ModType) -> bool {
        self.layouts[self.current_layout_idx]
            .modifiers
            .active_mod_type(mod_type)
    }

    /// Return the keycode (and optional level) for the given modifier type.
    #[doc(hidden)]
    pub fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)> {
        let modifiers = &self.layouts[self.current_layout_idx].modifiers;
        let mut other_mod = None;

        for (code, modifier) in modifiers.iter() {
            match modifier {
                Modifier::Single(state_modifier) => {
                    if state_modifier.has_mod_type(mod_type) {
                        match state_modifier.kind {
                            ModKind::Press { .. } => return Some((*code, None)),
                            _ => {
                                if other_mod.is_none() {
                                    other_mod = Some((*code, None));
                                }
                            }
                        }
                    }
                }
                Modifier::Leveled(map) => {
                    for (level, state_modifier) in map {
                        if state_modifier.has_mod_type(mod_type) {
                            match state_modifier.kind {
                                ModKind::Press { .. } => return Some((*code, Some(*level))),
                                _ => {
                                    if other_mod.is_none() {
                                        other_mod = Some((*code, Some(*level)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        other_mod
    }

    /// Designate an evdev keycode as the Compose (Multi_key) key.
    ///
    /// Keymaps compiled with an explicit `Multi_key` mapping detect the
    /// compose key automatically. For keymaps without one, this lets the
    /// caller designate a physical key — pressing it feeds the Compose token
    /// into the compose sequence, matching the desktop `compose:XXX` option
    /// behavior. Applies to all layouts; any existing modifier on the key is
    /// replaced.
    #[cfg(feature = "client")]
    pub fn set_compose_key(&mut self, evdev_code: u32) {
        for layout in &mut self.layouts {
            layout.modifiers.set_modifier(
                evdev_code,
                Modifier::Single(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::Compose,
                }),
            );
        }
    }

    /// Add a relative group-lock action.
    ///
    /// `delta = 1` cycles forward through layouts. Combining this with
    /// [`LockFlags::TAP`] changes layout only when the key is released without
    /// another key being pressed while it was held.
    pub fn set_group_key(&mut self, evdev_code: u32, kind: GroupKind) -> bool {
        self.groups.set_key(evdev_code, kind);
        true
    }

    /// Process a key press (compositor role only).
    ///
    /// Updates modifier, group, and LED state. Wayland clients use
    /// [`Self::update_modifiers`] and [`Self::compose`] instead.
    #[cfg(feature = "compositor")]
    pub fn press_key(&mut self, evdev_code: u32) -> StateChanges {
        self.change_key_state(evdev_code, KeyDirection::Down)
    }

    /// Process a key release (compositor role only).
    #[cfg(feature = "compositor")]
    pub fn release_key(&mut self, evdev_code: u32) -> StateChanges {
        self.change_key_state(evdev_code, KeyDirection::Up)
    }

    #[cfg(feature = "compositor")]
    #[inline]
    fn change_key_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> StateChanges {
        let before_modifiers = self.raw_modifiers();
        let before_leds = self.leds_state();
        let layouts = self.layouts.len();
        let is_modifier = self.layouts[self.current_layout_idx]
            .modifiers
            .set_state(evdev_code, key_direction);
        let new_layout = self
            .groups
            .update(evdev_code, key_direction, !is_modifier, layouts);
        if new_layout != self.current_layout_idx {
            let raw = self.layouts[self.current_layout_idx].modifiers.state(new_layout);
            self.layouts[new_layout]
                .modifiers
                .update(raw.depressed, raw.latched, raw.locked);
            self.current_layout_idx = new_layout;
        }
        if !is_modifier && key_direction == KeyDirection::Down {
            self.layouts[self.current_layout_idx].modifiers.unlatch();
        }
        StateChanges {
            is_modifier,
            modifiers_updated: self.raw_modifiers() != before_modifiers,
            leds_updated: self.leds_state() != before_leds,
        }
    }

    /// Feed a key into compose processing without changing modifier state.
    ///
    /// Client role only. Call after [`Self::update_modifiers`] on key press.
    #[cfg(feature = "client")]
    pub fn compose(&mut self, evdev_code: u32) -> Option<ComposeState> {
        let is_compose_key =
            self.layouts[self.current_layout_idx]
                .modifiers
                .iter()
                .any(|(code, modifier)| {
                    *code == evdev_code
                        && match modifier {
                            Modifier::Single(modifier) => modifier.has_mod_type(ModType::Compose),
                            Modifier::Leveled(levels) => levels
                                .iter()
                                .any(|(_, modifier)| modifier.has_mod_type(ModType::Compose)),
                        }
                });
        let token = if is_compose_key {
            Token::Compose
        } else if let Some(c) = self.key_char(evdev_code) {
            Token::Char(c)
        } else {
            return self.layouts[self.current_layout_idx].composer.buffer()
        };
        Some(self.layouts[self.current_layout_idx].composer.feed(token))
    }

    /// Export a layout as an [`ir::LayoutFile`] for persistence. This is the
    /// generation path for wkb layout data files.
    pub fn export_layout(&self, layout_idx: usize) -> Result<ir::LayoutFile, ir::IrError> {
        let layout = self
            .layouts
            .get(layout_idx)
            .ok_or(ir::IrError::InvalidLayoutIndex(layout_idx))?;
        ir::LayoutFile::try_from(layout)
    }

    /// Rebuild a [`WKB`] from one or more [`ir::LayoutFile`]s. Each file
    /// becomes one layout group, in order. This is the loading path for
    /// standalone wkb without XKB compilation.
    pub fn new_from_layouts(files: Vec<ir::LayoutFile>) -> Result<Self, ir::IrError> {
        let mut layouts = Vec::with_capacity(files.len());
        for file in files {
            layouts.push(KBLayout::try_from(file)?);
        }
        Ok(WKB {
            current_layout_idx: 0,
            layouts,
            groups: Groups::default(),
        })
    }
}

/// Flags describing what externally observable state changed during a key or
/// modifier update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StateChanges {
    /// Depressed, latched, or locked modifiers, or the active layout/group, changed.
    pub modifiers_updated: bool,
    /// [`WKB::leds_state`] changed.
    pub leds_updated: bool,
    pub is_modifier: bool,
}
