#![feature(extern_types)]
#![feature(c_variadic)]
#![allow(deref_nullptr)]

pub use composer::{ComposeState, Composer, ListComposer, Token};
use std::collections::{BTreeMap, HashSet};
pub mod composer;
pub use modifiers::KeyDirection;
use modifiers::{level_index, Modifiers, *};
pub mod modifiers;
pub mod xkb;

#[derive(Debug, Clone)]
pub struct WKB<C: Composer> {
    pub layouts: Vec<String>,
    pub layout: String,
    pub locale: Option<String>,
    pub pressed_keys: HashSet<u32>,
    pub repeat_keys: HashSet<u32>,
    pub composer: C,
    pub modifiers: Modifiers,
    pub state_keymap: Vec<BTreeMap<u32, char>>,
    pub num_lock_keys: Vec<BTreeMap<u32, char>>,
    pub caps_lock_keymap: Vec<BTreeMap<u32, char>>,
    pub level_exceptions_keymap: Vec<BTreeMap<u32, char>>,
}

impl WKB<ListComposer> {
    /// Create WKB instance from RMLVO names (Rules, Model, Layout, Variant, Options)
    ///
    /// # Example
    /// ```no_run
    /// use wkb::WKB;
    /// let wkb = WKB::new_from_names("us".to_string(), None);
    /// ```
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        xkb::new_from_names(locale, layout)
    }

    /// Create WKB instance from XKB keymap string
    ///
    /// This enables WKB to receive keymaps from Wayland compositors via the
    /// wl_keyboard.keymap event, or from any source that provides XKB format.
    ///
    /// # Example
    /// ```no_run
    /// use wkb::WKB;
    ///
    /// // Receive keymap string from Wayland compositor
    /// let keymap_string = "xkb_keymap { ... }".to_string();
    /// let wkb = WKB::new_from_string(keymap_string);
    /// ```
    pub fn new_from_string(string: String) -> Self {
        xkb::new_from_string(string)
    }
}

impl<C: Composer> WKB<C> {
    /// Get keymap string by reading from filesystem
    ///
    /// # Deprecated
    /// This method just reads from `/usr/share/X11/xkb/symbols/` and doesn't
    /// represent the actual WKB state. Use `new_from_string()` for proper
    /// keymap interchange instead.
    #[deprecated(since = "0.1.0", note = "Use new_from_string() for keymap interchange")]
    pub fn get_keymap_string(&self) -> String {
        if let Some(locale) = &self.locale {
            let path = std::path::Path::new(xkb::XKB_SYMBOLS_PATH);
            std::fs::read_to_string(path.join(locale)).unwrap_or_default()
        } else {
            String::new()
        }
    }

    pub fn modifiers_state(&self) -> (u32, u32, u32, u32) {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let group = 0;
        let mapping = [
            (LEFT_SHIFT, 1),
            (RIGHT_SHIFT, 1),
            (CAPS_LOCK, 2),
            (LEFT_CTRL, 4),
            (RIGHT_CTRL, 4),
            (ALT, 8),
            (NUM_LOCK, 16),
            (LOGO, 64),
            (ALTGR, 128),
        ];
        for (code, bit) in mapping {
            if let Some(modifier) = self.modifiers.0.get(&code) {
                if let Modifier::Single(mk) = modifier {
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
        }

        (depressed, latched, locked, group)
    }

    pub fn leds_state(&self) -> u32 {
        let mut leds = 0;
        if self.modifiers.locked_with_type(NUM_LOCK, ModType::Num) {
            leds |= 1;
        }
        if self.modifiers.locked_with_type(CAPS_LOCK, ModType::Caps) {
            leds |= 2;
        }
        if self
            .modifiers
            .locked_with_type(SCROLL_LOCK, ModType::Scroll)
        {
            leds |= 4;
        }
        leds
    }

    pub fn update_modifiers(&mut self, depressed: u32, latched: u32, locked: u32, group: u32) {
        if let Some(layout) = self.layouts.get(group as usize) {
            self.layout = layout.clone();
        }
        let mapping = [
            (LEFT_SHIFT, 1),
            (RIGHT_SHIFT, 1),
            (CAPS_LOCK, 2),
            (LEFT_CTRL, 4),
            (RIGHT_CTRL, 4),
            (ALT, 8),
            (NUM_LOCK, 16),
            (LOGO, 64),
            (ALTGR, 128),
        ];
        for (code, bit) in mapping {
            let is_depressed = (depressed & bit) != 0;
            let is_locked = (locked & bit) != 0;
            let is_latched = (latched & bit) != 0;

            self.modifiers.0.entry(code).and_modify(|m| {
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
            });
        }
    }

    pub fn level_key(&self, evdev_code: u32, level_index: usize) -> Option<char> {
        if let Some(c) = self
            .level_exceptions_keymap
            .get(level_index)
            .and_then(|m| m.get(&evdev_code).copied())
        {
            return Some(c);
        }
        self.state_keymap
            .get(level_index)
            .and_then(|hm| hm.get(&evdev_code).copied())
    }

    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.contains(&evdev_code)
    }

    pub fn utf8(&mut self, evdev_code: u32) -> Option<char> {
        if self.modifiers.active_mod_type(ModType::None) {
            return None;
        }
        let level5 = self.modifiers.level5() && self.state_keymap.len() > 4;
        let level3 = self.modifiers.level3() && self.state_keymap.len() > 2;
        let level2 = self.modifiers.level2() && self.state_keymap.len() > 1;
        let base_level = level_index(level5, level3, level2);

        if self.modifiers.locked(NUM_LOCK) {
            if let Some(&key) = self
                .num_lock_keys
                .get(base_level)
                .and_then(|m| m.get(&evdev_code))
            {
                return Some(key);
            }
        }

        if self.modifiers.locked(CAPS_LOCK) {
            if let Some(&c) = self
                .caps_lock_keymap
                .get(base_level)
                .and_then(|m| m.get(&evdev_code))
            {
                return Some(c);
            }
        }

        let key = self
            .state_keymap
            .get(base_level)
            .and_then(|m| m.get(&evdev_code).copied());
        key
    }

    pub fn update_key(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let is_modifier = self.modifiers.set_state(evdev_code, key_direction);
        if !is_modifier {
            // Non-modifier key: unlatch any latched modifiers on key press
            if key_direction == KeyDirection::Down {
                self.modifiers.unlatch();
            }
            match key_direction {
                KeyDirection::Up => self.pressed_keys.remove(&evdev_code),
                KeyDirection::Down => self.pressed_keys.insert(evdev_code),
            };
        }
        is_modifier
    }

    pub fn key(&mut self, evdev_code: u32, key_direction: KeyDirection) -> (Option<char>, bool) {
        let is_modifier = self.update_key(evdev_code, key_direction);
        let utf8 = if key_direction == KeyDirection::Down && !is_modifier {
            self.utf8(evdev_code)
        } else {
            None
        };
        (utf8, is_modifier)
    }

    pub fn key_compose(
        &mut self,
        evdev_code: u32,
        key_direction: KeyDirection,
    ) -> (Option<ComposeState>, bool) {
        let is_modifier = self.update_key(evdev_code, key_direction);
        let compose_state = if key_direction == KeyDirection::Down
            && is_modifier
            && self.modifiers.active_mod_type(ModType::Compose)
        {
            Some(self.composer.feed(Token::Compose))
        } else if key_direction == KeyDirection::Down && !is_modifier {
            self.utf8(evdev_code)
                .map(|c| self.composer.feed(Token::Char(c)))
        } else {
            None
        };
        (compose_state, is_modifier)
    }

    pub fn layouts(&self) -> Vec<String> {
        self.layouts.clone()
    }

    pub fn current_layout(&self) -> String {
        self.layout.clone()
    }
}
