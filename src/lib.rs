use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
};

pub use modifiers::KeyDirection;
use modifiers::{level_index, Modifiers, *};
use repeat::REPEAT_DEFAULT;
pub mod modifiers;
use xkb::repeat;
mod xkb;
include!(concat!(env!("OUT_DIR"), "/repeat.rs"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComposeStatus {
    Idle,
    Composing,
    Composed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeedResult {
    Ignored,
    Accepted,
}

#[derive(Debug, Clone)]
pub struct WKB {
    layouts: Vec<String>,
    layout: String,
    pub level_keymap: Vec<BTreeMap<u32, char>>,
    compose_status: ComposeStatus,
    compose_char: char,
    pressed_keys: HashSet<u32>,
    repeat_keys: HashSet<u32>,
    pub custom_case_map: Option<HashMap<char, char>>, // TODO public?
    pub modifiers: Modifiers,
    num_lock_keys: Vec<u32>,
    remap: HashMap<u32, u32>,
    caps_is_level2: Option<Vec<u32>>,
    caps_lock_disabled: bool,
    caps_lock_level2_disabled: bool,
    right_left_shift_caps: bool,
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
        let repeat_keys = if let Some(locale) = REPEAT_KEYS.get(&locale.as_str()) {
            if let Some(layout) = locale.get(&layout.as_str()) {
                layout.clone()
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
            compose_status: ComposeStatus::Idle,
            level_keymap: Vec::with_capacity(8),
            compose_char: char::default(),
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

    pub fn level_key(&self, evdev_code: u32, level_index: usize) -> Option<char> {
        if let Some(character) = self
            .level_keymap
            .get(level_index)
            .and_then(|hm| hm.get(&evdev_code).copied())
        {
            Some(character)
        } else {
            None
        }
    }

    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.get(&evdev_code).is_some()
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

    pub fn update_key(&mut self, evdev_code: u32, key_direction: KeyDirection) {
        if !self.modifiers.set_state(evdev_code, key_direction) {
            match key_direction {
                KeyDirection::Up => self.pressed_keys.remove(&evdev_code),
                KeyDirection::Down => self.pressed_keys.insert(evdev_code),
            };
        }
    }

    pub fn compose_feed(&mut self, character: char) -> FeedResult {
        if unicode_normalization::char::is_combining_mark(character) {
            match self.compose_status {
                ComposeStatus::Idle | ComposeStatus::Composed | ComposeStatus::Cancelled => {
                    self.compose_char = character;
                    self.compose_status = ComposeStatus::Composing;
                    FeedResult::Accepted
                }
                ComposeStatus::Composing => {
                    if let Some(character) =
                        unicode_normalization::char::compose(character, self.compose_char)
                    {
                        self.compose_char = character;
                        FeedResult::Accepted
                    } else {
                        self.compose_status = ComposeStatus::Cancelled;
                        FeedResult::Ignored
                    }
                }
            }
        } else if self.compose_status == ComposeStatus::Composing {
            if let Some(character) =
                unicode_normalization::char::compose(character, self.compose_char)
            {
                self.compose_status = ComposeStatus::Composed;
                self.compose_char = character;
                FeedResult::Accepted
            } else {
                self.compose_status = ComposeStatus::Cancelled;
                FeedResult::Ignored
            }
        } else {
            self.compose_status = ComposeStatus::Cancelled;
            FeedResult::Ignored
        }
    }

    pub fn compose_status(&self) -> ComposeStatus {
        self.compose_status
    }

    pub fn compose_utf8(&self) -> char {
        self.compose_char
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
