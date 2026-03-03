pub use composer::{ComposeState, Composer, ListComposer, UnicodeComposer};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
};
pub mod composer;
pub use modifiers::KeyDirection;
use modifiers::{level_index, Modifiers, *};
use repeat::REPEAT_DEFAULT;
pub mod modifiers;
use xkb::repeat;
mod xkb;
include!(concat!(env!("OUT_DIR"), "/repeat.rs"));

#[derive(Debug, Clone)]
pub struct WKB<C: Composer> {
    layouts: Vec<String>,
    layout: String,
    locale: Option<String>,
    state_keymap: Vec<BTreeMap<u32, char>>,
    regular_composer: C,
    compose_key_composer: C,
    pressed_keys: HashSet<u32>,
    repeat_keys: HashSet<u32>,
    modifiers: Modifiers,
    num_lock_keys: Vec<BTreeMap<u32, char>>,
    caps_lock_table: Vec<BTreeMap<u32, char>>,
    level_exceptions_keymap: Vec<BTreeMap<u32, char>>,
}

impl WKB<ListComposer> {
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        xkb::new_from_names(locale, layout)
    }

    pub fn new_from_string(string: String) -> Self {
        xkb::new_from_string(string)
    }
}

impl<C: Composer> WKB<C> {
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
                match modifier {
                    Modifier::Single(mk) => match mk {
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
                    },
                    _ => {}
                }
            }
        }

        (depressed, latched, locked, group)
    }

    pub fn leds_state(&self) -> u32 {
        let mut leds = 0;
        if self.modifiers.locked(NUM_LOCK) {
            leds |= 1;
        }
        if self.modifiers.locked(CAPS_LOCK) {
            leds |= 2;
        }
        if self.modifiers.locked(SCROLL_LOCK) {
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

            self.modifiers.0.entry(code).and_modify(|m| match m {
                Modifier::Single(mk) => match mk {
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
                },
                _ => {}
            });
        }
    }

    pub fn level_key(&self, evdev_code: u32, level_index: usize) -> Option<char> {
        // First check exceptions
        if let Some(c) = self
            .level_exceptions_keymap
            .get(level_index)
            .and_then(|m| m.get(&evdev_code).copied())
        {
            return Some(c);
        }
        // Fall back to state_keymap
        self.state_keymap
            .get(level_index)
            .and_then(|hm| hm.get(&evdev_code).copied())
    }

    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.contains(&evdev_code)
    }

    pub fn utf8(&mut self, evdev_code: u32) -> Option<char> {
        let level5 = self.modifiers.level5() && self.state_keymap.len() > 4;
        let level3 = self.modifiers.level3() && self.state_keymap.len() > 2;
        let level2 = self.modifiers.level2() && self.state_keymap.len() > 1;
        let base_level = level_index(level5, level3, level2);

        // Num Lock takes priority
        if self.modifiers.locked(NUM_LOCK) {
            if let Some(&key) = self
                .num_lock_keys
                .get(base_level)
                .and_then(|m| m.get(&evdev_code))
            {
                self.modifiers.unlatch();
                return Some(key);
            }
        }

        let caps_active = self.modifiers.caps_active();

        if caps_active {
            if let Some(&c) = self
                .caps_lock_table
                .get(base_level)
                .and_then(|m| m.get(&evdev_code))
            {
                self.modifiers.unlatch();
                return Some(c);
            }
        }

        // Use state_keymap directly
        let key = self
            .state_keymap
            .get(base_level)
            .and_then(|m| m.get(&evdev_code).copied());
        if key.is_some() {
            self.modifiers.unlatch()
        }
        key
    }

    pub fn update_key(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let is_modifier = self.modifiers.set_state(evdev_code, key_direction);
        if !is_modifier {
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
        let compose_state = if key_direction == KeyDirection::Down && !is_modifier {
            self.utf8(evdev_code).map(|c| self.compose_feed(c))
        } else {
            None
        };
        (compose_state, is_modifier)
    }

    pub fn compose_feed(&mut self, character: char) -> ComposeState {
        if self.modifiers.level(ModType::Compose) {
            self.compose_key_composer.feed(character)
        } else {
            self.regular_composer.feed(character)
        }
    }

    pub fn layouts(&self) -> Vec<String> {
        self.layouts.clone()
    }

    pub fn current_layout(&self) -> String {
        self.layout.clone()
    }

    pub fn level_keymap(&self) -> Vec<BTreeMap<u32, char>> {
        self.state_keymap.clone()
    }
}
