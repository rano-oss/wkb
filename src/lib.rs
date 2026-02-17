use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
};

pub use modifiers::KeyDirection;
use modifiers::{level_index, Modifiers, *};
use repeat::REPEAT_DEFAULT;

use composer::{ComposeState, Composer, UnicodeComposer};
mod composer;
mod default_keymap;
pub mod modifiers;
mod repeat;
pub mod xkb;

include!(concat!(env!("OUT_DIR"), "/repeat.rs"));

#[derive(Debug, Clone)]
pub struct WKB<C: Composer = UnicodeComposer> {
    pub(crate) layouts: Vec<String>,
    pub(crate) layout: String,
    pub(crate) locale: Option<String>,
    pub level_keymap: Vec<BTreeMap<u32, char>>,
    pub composer: C,
    pub(crate) pressed_keys: HashSet<u32>,
    pub(crate) repeat_keys: HashSet<u32>,
    pub custom_case_map: Option<HashMap<char, char>>,
    pub modifiers: Modifiers,
    pub(crate) num_lock_keys: Vec<u32>,
    pub(crate) remap: HashMap<u32, u32>,
    pub(crate) caps_is_level2: Option<Vec<u32>>,
    pub(crate) caps_lock_disabled: bool,
    pub(crate) caps_lock_level2_disabled: bool,
    pub(crate) right_left_shift_caps: bool,
}

impl WKB {
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let layouts = xkb::read_layouts(path, Some(locale.clone()), None);
        let layout = if let Some(layout) = layout {
            layout
        } else {
            layouts[0].clone()
        };
        let repeat_keys = if let Some(locale_map) = REPEAT_KEYS.get(&locale.as_str()) {
            if let Some(layout_set) = locale_map.get(&layout.as_str()) {
                layout_set.clone()
            } else {
                REPEAT_DEFAULT.clone()
            }
        } else {
            REPEAT_DEFAULT.clone()
        };
        let modifiers = Modifiers::default();
        let mut wkb = Self {
            layouts,
            layout: layout.clone(),
            locale: Some(locale.clone()),
            composer: UnicodeComposer::new(),
            level_keymap: Vec::with_capacity(8),
            pressed_keys: HashSet::new(),
            repeat_keys,
            custom_case_map: None,
            caps_is_level2: None,
            modifiers,
            num_lock_keys: vec![
                71, 72, 73, // 7, 8, 9
                75, 76, 77, // 4, 5, 6
                79, 80, 81, // 1, 2, 3
                82, 83, // 0, .
            ],
            remap: HashMap::new(),
            caps_lock_disabled: false,
            caps_lock_level2_disabled: false,
            right_left_shift_caps: false,
        };
        xkb::map_xkb(&mut wkb, path, locale.clone(), Some(layout.clone()));
        xkb::fix_xkb_edge_cases(&mut wkb, locale, Some(layout));
        wkb
    }

    pub fn new_from_string(string: String) -> Self {
        let modifiers = Modifiers::default();
        let repeat_keys = REPEAT_DEFAULT.clone();
        let mut wkb = Self {
            layouts: Vec::new(),
            layout: "".to_string(),
            locale: None,
            composer: UnicodeComposer::new(),
            level_keymap: Vec::with_capacity(8),
            pressed_keys: HashSet::new(),
            repeat_keys,
            custom_case_map: None,
            caps_is_level2: None,
            modifiers,
            num_lock_keys: vec![
                71, 72, 73, // 7, 8, 9
                75, 76, 77, // 4, 5, 6
                79, 80, 81, // 1, 2, 3
                82, 83, // 0, .
            ],
            remap: HashMap::new(),
            caps_lock_disabled: false,
            caps_lock_level2_disabled: false,
            right_left_shift_caps: false,
        };
        xkb::map_xkb_from_str(
            &mut wkb,
            &string,
            Path::new("/usr/share/X11/xkb/symbols/"),
            None,
            None,
        );
        wkb
    }
}

impl<C: Composer> WKB<C> {
    pub fn get_keymap_string(&self) -> String {
        if let Some(locale) = &self.locale {
            let path = Path::new("/usr/share/X11/xkb/symbols/");
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
                        ModKind::Lock { locked: l, .. } if *l > 0 => locked |= bit,
                        ModKind::Latch { latched: true, .. } => latched |= bit,
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
        self.level_keymap
            .get(level_index)
            .and_then(|hm| hm.get(&evdev_code).copied())
    }

    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.contains(&evdev_code)
    }

    pub fn utf8(&mut self, evdev_code: u32) -> Option<char> {
        let level5 = self.modifiers.level5() && self.level_keymap.len() > 4;
        let level3 = self.modifiers.level3() && self.level_keymap.len() > 2;
        let level2 = self.modifiers.level2() && self.level_keymap.len() > 1;
        let level_index = level_index(level5, level3, level2);
        if self.right_left_shift_caps
            && self.modifiers.pressed(RIGHT_SHIFT)
            && self.modifiers.pressed(LEFT_SHIFT)
            || self.modifiers.locked(CAPS_LOCK)
        {} //TODO: fix CAPS_LOCK improve!
        let key = if self.num_lock_keys.contains(&evdev_code) && self.modifiers.locked(NUM_LOCK) {
            self.level_key(evdev_code, 1)
        } else {
            self.level_key(evdev_code, level_index)
        };
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

    pub fn process_key_event(
        &mut self,
        evdev_code: u32,
        key_direction: KeyDirection,
    ) -> (Option<char>, bool) {
        let is_modifier = self.update_key(evdev_code, key_direction);
        let utf8 = if key_direction == KeyDirection::Down && !is_modifier {
            self.utf8(evdev_code)
        } else {
            None
        };
        (utf8, is_modifier)
    }

    pub fn compose_feed(&mut self, character: char) -> ComposeState {
        self.composer.feed(character)
    }

    pub fn layouts(&self) -> Vec<String> {
        self.layouts.clone()
    }

    pub fn current_layout(&self) -> String {
        self.layout.clone()
    }

    pub fn level_keymap(&self) -> Vec<BTreeMap<u32, char>> {
        self.level_keymap.clone()
    }
}
