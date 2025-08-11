use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::Read,
    os::fd::OwnedFd,
    path::Path,
};

use default_keymap::DEFAULT_MAP;
use evdev_xkb::XKBCODES_EVDEV;
use regex::Regex;
use repeat::REPEAT_DEFAULT;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};
use xkb_utf8::XKBCODES_DEF_TO_UTF8;
mod default_keymap;
mod evdev_xkb;
mod repeat;
mod xkb_utf8;
include!(concat!(env!("OUT_DIR"), "/repeat.rs"));

#[derive(Debug, Clone, Copy, Default)]
pub struct Modifier {
    pressed: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct LockModifier {
    pressed: bool,
    locked: u8,
}

#[derive(Debug, Clone, Copy, Default)]
struct LatchModifier {
    pressed: bool,
    latched: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Modifiers {
    remap: HashMap<u32, u32>,
    left_ctrl: Modifier,
    right_ctrl: Modifier,
    alt: Modifier,
    alt_gr: Modifier,
    left_shift: Modifier,
    right_shift: Modifier,
    logo: Modifier,
    caps_lock: LockModifier,
    num_lock: LockModifier,
    scroll_lock: LockModifier,
    pub level2shift: ((u32, Modifier), (u32, Modifier)),
    level2lock: (u32, LockModifier),
    pub level3shift: ((u32, Modifier), Option<(u32, Modifier)>),
    level3lock: (u32, LockModifier),
    level3latch: (u32, LatchModifier),
    pub level5shift: (u32, Modifier),
    level5lock: (u32, LockModifier),
    level5latch: (u32, LatchModifier),
    compose_key: (u32, Modifier),
    caps_lock_disabled: bool,
    caps_lock_level2_disabled: bool,
    right_left_shift_caps: bool,
    shift_caps: bool,
}

impl Modifiers {
    fn level5(&self) -> bool {
        self.level5shift.1.pressed || self.level5lock.1.locked > 0 || self.level5latch.1.latched
    }

    fn level3(&self) -> bool {
        self.level3shift.0 .1.pressed
            || self.level3shift.1.is_some_and(|s| s.1.pressed)
            || self.level3lock.1.locked > 0
            || self.level3latch.1.latched
    }

    fn level2(&self) -> bool {
        self.level2shift.0 .1.pressed || self.level2shift.1 .1.pressed
    }

    fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let evdev_code = if let Some(code) = self.remap.get(&evdev_code) {
            *code
        } else {
            evdev_code
        };
        let mut updated = false;
        if self.level2shift.0 .0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level2shift.0 .1.pressed = true,
                KeyDirection::Up => self.level2shift.0 .1.pressed = false,
            }
            updated = true;
        } else if self.level2shift.1 .0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level2shift.1 .1.pressed = true,
                KeyDirection::Up => self.level2shift.1 .1.pressed = false,
            }
            updated = true;
        } else if self.level3shift.0 .0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level3shift.0 .1.pressed = true,
                KeyDirection::Up => self.level3shift.0 .1.pressed = false,
            }
            updated = true;
        } else if self.level3lock.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => {
                    self.level3lock.1.pressed = true;
                    if self.level3lock.1.locked == 0 {
                        self.level3lock.1.locked = 2;
                    }
                }
                KeyDirection::Up => {
                    self.level3lock.1.pressed = false;
                    if self.level3lock.1.locked != 0 {
                        self.level3lock.1.locked -= 1;
                    }
                }
            }
            updated = true;
        } else if self.level3latch.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => {
                    self.level3latch.1.pressed = true;
                    self.level3latch.1.latched = true;
                }
                KeyDirection::Up => self.level3latch.1.pressed = false,
            }
            updated = true;
        } else if self.level5shift.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level5shift.1.pressed = true,
                KeyDirection::Up => self.level5shift.1.pressed = false,
            }
            updated = true;
        } else if self.level5lock.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => {
                    self.level5lock.1.pressed = true;
                    if self.level5lock.1.locked == 0 {
                        self.level5lock.1.locked = 2;
                    }
                }
                KeyDirection::Up => {
                    self.level5lock.1.pressed = false;
                    if self.level5lock.1.locked != 0 {
                        self.level5lock.1.locked -= 1;
                    }
                }
            }
            updated = true;
        } else if self.level5latch.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => {
                    self.level5latch.1.pressed = true;
                    self.level5latch.1.latched = true;
                }
                KeyDirection::Up => self.level5latch.1.pressed = false,
            }
            updated = true;
        }
        match (evdev_code, key_direction) {
            (29, KeyDirection::Down) => self.left_ctrl.pressed = true,
            (29, KeyDirection::Up) => self.left_ctrl.pressed = false,
            (42, KeyDirection::Down) => {
                self.left_shift.pressed = true;
                if (self.right_left_shift_caps
                    && self.right_shift.pressed
                    && self.left_shift.pressed
                    || self.shift_caps && self.caps_lock.pressed)
                    && self.caps_lock.locked == 0
                {
                    self.caps_lock.locked = 1;
                } else if self.right_left_shift_caps
                    && self.right_shift.pressed
                    && self.left_shift.pressed
                    || self.shift_caps && self.caps_lock.pressed
                {
                    self.caps_lock.locked = 0;
                }
            }
            (42, KeyDirection::Up) => self.left_shift.pressed = false,
            (54, KeyDirection::Down) => {
                self.right_shift.pressed = true;
                if (self.right_left_shift_caps
                    && self.right_shift.pressed
                    && self.left_shift.pressed
                    || self.shift_caps && self.caps_lock.pressed)
                    && self.caps_lock.locked == 0
                {
                    self.caps_lock.locked = 1;
                } else if self.right_left_shift_caps
                    && self.right_shift.pressed
                    && self.left_shift.pressed
                    || self.shift_caps && self.caps_lock.pressed
                {
                    self.caps_lock.locked = 0;
                }
            }
            (54, KeyDirection::Up) => self.right_shift.pressed = false,
            (97, KeyDirection::Down) => self.right_ctrl.pressed = true,
            (97, KeyDirection::Up) => self.right_ctrl.pressed = false,
            (56, KeyDirection::Down) => self.alt.pressed = true,
            (56, KeyDirection::Up) => self.alt.pressed = false,
            (100, KeyDirection::Down) => self.alt_gr.pressed = true,
            (100, KeyDirection::Up) => self.alt_gr.pressed = false,
            (125, KeyDirection::Down) => self.logo.pressed = true,
            (125, KeyDirection::Up) => self.logo.pressed = false,
            (58, KeyDirection::Down) => {
                self.caps_lock.pressed = true;
                if self.shift_caps && self.right_shift.pressed || self.left_shift.pressed {
                    self.caps_lock.locked = 1;
                } else if self.caps_lock.locked == 0
                    && !self.right_left_shift_caps
                    && !self.shift_caps
                {
                    self.caps_lock.locked = 2;
                }
            }
            (58, KeyDirection::Up) => {
                self.caps_lock.pressed = false;
                if self.caps_lock.locked > 0 && !self.right_left_shift_caps {
                    self.caps_lock.locked -= 1;
                }
            }
            (69, KeyDirection::Down) => {
                self.num_lock.pressed = true;
                if self.num_lock.locked == 0 {
                    self.num_lock.locked = 2;
                }
            }
            (69, KeyDirection::Up) => {
                self.num_lock.pressed = false;
                if self.num_lock.locked > 0 {
                    self.num_lock.locked -= 1;
                }
            }
            (70, KeyDirection::Down) => {
                self.scroll_lock.pressed = true;
                if self.scroll_lock.locked == 0 {
                    self.scroll_lock.locked = 2;
                }
            }
            (70, KeyDirection::Up) => {
                self.scroll_lock.pressed = false;
                if self.scroll_lock.locked > 0 {
                    self.scroll_lock.locked -= 1;
                }
            }
            (_, _) => return updated,
        };
        updated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyDirection {
    Up,
    Down,
}

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

fn read_layouts(path: &Path, locale: Option<String>, fd: Option<OwnedFd>) -> Vec<String> {
    let mut string_file = String::new();
    if let Some(fd) = fd {
        let mut file = File::from(fd);
        file.read_to_string(&mut string_file)
            .expect("Could not read file");
    } else if let Some(locale) = locale {
        string_file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
    }
    let xkb = parse(&string_file).expect("neither names nor file set");
    let mut layouts = Vec::new();
    xkb.definitions.iter().for_each(|d| match &d.directive {
        Directive::XkbSymbols(src) => {
            let name = src.name.content.to_string();
            if ![
                "sun_type6",
                "sun_type6_de",
                "sun_type6_fr",
                "sun_type6_suncompat",
                "sun_type7_suncompat",
                "suncompat",
                "sun_type7",
            ]
            .contains(&name.as_str())
            {
                layouts.push(src.name.content.to_string());
            }
        }
        _ => {}
    });
    layouts
}

fn parse_include(input: &str) -> (String, Option<String>) {
    let re = Regex::new(r"([\w]+)(?:\(([\w\-]+)\))?$").unwrap();
    let capture = re.captures(input).unwrap();
    (
        capture.get(1).map(|m| m.as_str().to_string()).unwrap(),
        capture.get(2).map(|m| m.as_str().to_string()),
    )
}

fn unicode_string_to_unicode_char(s: &str) -> Option<char> {
    let number = &s[1..];

    u32::from_str_radix(number, 16)
        .ok()
        .and_then(std::char::from_u32)
}

fn hex_string_to_unicode_char(s: &str) -> Option<char> {
    let split_pos = s.char_indices().nth_back(4).unwrap().0;
    let number = &s[split_pos..];

    u32::from_str_radix(number, 16)
        .ok()
        .and_then(std::char::from_u32)
}

fn to_uppercase(c: char, custom_case_map: Option<HashMap<char, char>>) -> char {
    if let Some(case_map) = custom_case_map {
        if let Some(uc) = case_map.get(&c) {
            return *uc;
        }
    }
    let uc = c.to_uppercase();
    if uc.len() > 1 {
        return c;
    }
    uc.last().unwrap()
}

fn to_lowercase(c: char, custom_case_map: Option<HashMap<char, char>>) -> char {
    if let Some(case_map) = custom_case_map {
        if let Some(c) = case_map.get(&c) {
            return *c;
        }
    }
    if c.is_lowercase() {
        let uc = c.to_uppercase();
        if uc.len() > 1 {
            return c;
        }
        return uc.last().unwrap();
    }
    let lc = c.to_lowercase();
    if lc.len() > 1 {
        return c;
    }
    lc.last().unwrap()
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
    caps_is_level2: Option<Vec<u32>>,
    pub modifiers: Modifiers,
}

impl WKB {
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let layouts = read_layouts(path, Some(locale.clone()), None);
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
        let mut modifiers = Modifiers::default();
        modifiers.level2shift = ((42, Modifier::default()), (54, Modifier::default()));
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
        };
        wkb.map_xkb(path, locale.clone(), Some(layout.clone()));
        wkb.fix_xkb_edge_cases(locale, Some(layout));
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
        let level5 = self.modifiers.level5();
        let level3 = self.modifiers.level3();
        let level2 = self.modifiers.level2() && self.level_keymap.len() > 1;
        if [
            71, 72, 73, // 7, 8, 9
            75, 76, 77, // 4, 5, 6
            79, 80, 81, // 1, 2, 3
            82, 83, // 0, .
        ]
        .contains(&evdev_code)
            && self.modifiers.num_lock.locked > 0
        {
            self.level_key(evdev_code, 1)
        } else if level5 && level3 && level2 {
            self.modifiers.level3latch.1.latched = false;
            self.level_key(evdev_code, 7)
        } else if level5 && level3 {
            self.modifiers.level3latch.1.latched = false;
            self.level_key(evdev_code, 6)
        } else if level5 && level2 {
            self.level_key(evdev_code, 5)
        } else if level5 {
            self.level_key(evdev_code, 4)
        } else if level3 && level2 {
            self.modifiers.level3latch.1.latched = false;
            // if self.modifiers.caps_lock.locked > 0 {
            //     self.level_key(evdev_code, 3)
            //         .map(|c| c.to_uppercase().last().unwrap())
            // } else {
            self.level_key(evdev_code, 3)
            // }
        } else if level3 {
            self.modifiers.level3latch.1.latched = false;
            // if self.modifiers.caps_lock.locked > 0 {
            //     self.level_key(evdev_code, 2)
            //         .map(|c| c.to_uppercase().last().unwrap())
            // } else {
            self.level_key(evdev_code, 2)
            // }
        } else if level2 {
            if self.modifiers.caps_lock.locked > 0 && !self.modifiers.caps_lock_level2_disabled {
                if let Some(caps_level2_list) = &self.caps_is_level2 {
                    if caps_level2_list.contains(&evdev_code) {
                        return self.level_key(evdev_code, 0);
                    }
                }
                self.level_key(evdev_code, 1)
                    .map(|c| to_lowercase(c, self.custom_case_map.clone()))
            } else {
                self.level_key(evdev_code, 1)
            }
        } else {
            if self.modifiers.caps_lock.locked > 0 && !self.modifiers.caps_lock_disabled {
                if let Some(caps_level2_list) = &self.caps_is_level2 {
                    if caps_level2_list.contains(&evdev_code) {
                        return self.level_key(evdev_code, 1);
                    }
                }
                self.level_key(evdev_code, 0)
                    .map(|c| to_uppercase(c, self.custom_case_map.clone()))
            } else {
                self.level_key(evdev_code, 0)
            }
        }
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

    fn fix_xkb_edge_cases(&mut self, locale: String, layout: Option<String>) {
        //snowflake special case of bad design I don't want to support any other way!
        match (locale.as_str(), &layout.as_deref()) {
            ("de", Some("ru")) | ("de", Some("ru-recom")) | ("de", Some("ru-translit")) => {
                for i in 0..2 {
                    let map = self.level_keymap[i].clone();
                    let t21 = map.get(&21);
                    let t44 = map.get(&44);
                    self.level_keymap[i].insert(21, *t44.unwrap());
                    self.level_keymap[i].insert(44, *t21.unwrap());
                }
            }
            _ => {}
        };
        match (locale.as_str(), &layout.as_deref()) {
            ("at", Some(_))
            | ("de", Some("basic"))
            | ("de", Some("mac"))
            | ("de", Some("mac_nodeadkeys"))
            | ("de", Some("deadtilde"))
            | ("de", Some("neo_base"))
            | ("de", Some("deadgraveacute"))
            | ("de", Some("deadacute"))
            | ("de", Some("ro"))
            | ("de", Some("ro_nodeadkeys"))
            | ("de", Some("dsb_qwertz"))
            | ("de", Some("qwerty"))
            | ("de", Some("ru"))
            | ("de", Some("pl"))
            | ("de", Some("tr"))
            | ("de", Some("hu"))
            | ("de", Some("e1"))
            | ("de", Some("e2"))
            | ("de", Some("dvorak"))
            | ("de", Some("adnw_base"))
            | ("de", Some("koy_base"))
            | ("de", Some("bone_base"))
            | ("de", Some("bone_eszett_home_base"))
            | ("de", Some("neo_qwertz_base"))
            | ("de", Some("neo_qwerty_base"))
            | ("it", Some("lldde"))
            | ("cz", Some("ucw"))
            | ("de", Some("nodeadkeys")) => {
                self.custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
            }
            ("de", Some("ru-translit")) | ("de", Some("ru-recom")) => {
                self.custom_case_map = Some(HashMap::from([('~', 'ẞ')]));
            }
            ("tr", Some("basic"))
            | ("tr", Some("e"))
            | ("tr", Some("intl"))
            | ("tr", Some("olpc"))
            | ("ua", Some("crh"))
            | ("ua", Some("crh_f"))
            | ("ro", Some("crh_dobruja"))
            | ("tr", Some("f")) => {
                self.custom_case_map = Some(HashMap::from([('i', 'İ'), ('İ', 'i'), ('I', 'ı')]));
            }
            ("az", Some("latin")) => self.caps_is_level2 = Some(vec![23, 39]),
            ("gh", Some("hausa")) => self.caps_is_level2 = Some(vec![39]),
            ("ng", Some("hausa")) | ("gh", Some("fula")) | ("tr", Some("us")) => {
                self.custom_case_map = Some(HashMap::from([('ı', 'İ'), ('İ', 'ı')]));
            }
            ("md", Some("gag")) => {
                self.custom_case_map = Some(HashMap::from([
                    ('ı', 'I'),
                    ('I', 'ı'),
                    ('i', 'İ'),
                    ('İ', 'i'),
                ]));
            }
            ("bqn", Some("bqn")) => {
                self.custom_case_map = Some(HashMap::from([
                    ('𝕨', '𝕎'),
                    ('𝕤', '𝕊'),
                    ('𝕗', '𝔽'),
                    ('𝕘', '𝔾'),
                    ('𝕩', '𝕏'),
                    ('𝕎', '𝕨'),
                    ('𝕊', '𝕤'),
                    ('𝔽', '𝕗'),
                    ('𝔾', '𝕘'),
                    ('𝕏', '𝕩'),
                ]));
                self.level_keymap[1].insert(22, '⊔');
                self.level_keymap[1].insert(32, '↕');
                self.level_keymap[1].insert(36, '∘');
                self.level_keymap[1].insert(46, '↓');
                self.level_keymap[1].insert(57, '‿');
            }
            ("hu", Some("oldhunlig")) => {
                self.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41, 51, 52, 53]);
            }
            ("hu", Some("oldhun_base")) => {
                self.caps_is_level2 = Some(vec![51, 52, 53]);
            }
            ("hu", Some("oldhun_origin")) | ("hu", Some("oldhun_lig")) => {
                self.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41]);
            }
            ("kz", Some("ext")) => self.caps_is_level2 = Some(vec![2, 7, 8, 43]),
            ("fr", Some("bepo")) => {
                self.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            }
            ("fr", Some("bepo_afnor")) => {
                self.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
                self.level_keymap.resize(4, BTreeMap::new());
            }
            ("fr", Some("dvorak")) => {
                self.caps_is_level2 =
                    Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 17, 18, 41]);
                self.custom_case_map = Some(HashMap::from([('à', '/'), (';', '-'), ('ç', 'ç')]))
            }
            ("kz", Some("latin")) => {
                self.custom_case_map = Some(HashMap::from([('v', 'M')]));
            }
            ("us", Some("chr")) => {
                self.caps_is_level2 = Some(vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                    25, 26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47,
                    48, 49, 50, 51, 52, 53,
                ])
            }
            ("us", Some("mac")) => self.modifiers.caps_lock_level2_disabled = true,
            ("il", Some("si2")) | ("il", Some("basic")) => {
                self.caps_is_level2 = Some(vec![
                    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                    44, 45, 46, 47, 48, 49, 50, 51, 52,
                ])
            }
            ("il", Some("phonetic")) => {
                self.caps_is_level2 = Some(vec![20, 25, 33, 37, 39, 46, 49, 50, 51, 52])
            }
            ("il", Some("biblical")) => {
                self.caps_is_level2 = Some(vec![
                    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48,
                    49, 50, 51, 52, 53,
                ])
            }
            ("il", Some("biblicalSIL")) => {
                self.caps_is_level2 = Some(vec![
                    2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 16, 18, 21, 24, 25, 26, 27, 30, 31, 33, 36,
                    37, 39, 40, 41, 46, 49, 50, 51, 52, 53,
                ])
            }
            ("it", Some("geo")) => {
                self.caps_is_level2 = Some(vec![
                    16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
                ])
            }
            ("pl", Some("dvp")) | ("us", Some("dvp")) => {
                self.caps_is_level2 = Some(vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 40])
            }
            ("in", Some("tamilnet_TAB")) | ("lk", Some("tam_TAB")) => {
                self.modifiers.caps_lock_disabled = true;
                self.modifiers.caps_lock_level2_disabled = true;
            }
            ("in", Some("tamilnet_TSCII")) => {
                self.custom_case_map = Some(HashMap::from([('þ', 'þ')]));
                self.modifiers.caps_lock_level2_disabled = true;
            }
            ("in", Some("iipa")) => self.custom_case_map = Some(HashMap::from([('b', 'Y')])),
            ("ge", Some("qwerty")) | ("ge", Some("basic")) => {
                self.caps_is_level2 = Some(vec![
                    16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
                ])
            }
            ("ge", Some("ergonomic")) => {
                self.caps_is_level2 = Some(vec![
                    16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
                ])
            }
            ("ge", Some("mess")) => {
                self.caps_is_level2 = Some(vec![
                    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 44,
                    45, 46, 47, 48, 49, 50,
                ])
            }
            ("fr", Some("geo")) => {
                self.caps_is_level2 = Some(vec![
                    17, 18, 20, 22, 24, 25, 26, 27, 30, 31, 32, 37, 38, 39, 40, 44, 48, 51,
                ]);
            }
            ("gr", Some(_)) => {
                self.caps_is_level2 = Some(vec![17]);
            }
            ("pl", Some("glagolica")) => {
                self.modifiers.caps_lock_level2_disabled = true;
                let value = *self.level_keymap[0].get(&40).unwrap();
                self.level_keymap[1].insert(40, value);
            }
            ("cd", _) | ("ml", Some("us-mac")) => self.modifiers.caps_lock_level2_disabled = true,
            ("apl", Some("apl2")) | ("apl", Some("aplplusII")) => {
                for i in 16..=25 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[1].insert(i, value);
                }
                for i in 30..=38 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[1].insert(i, value);
                }
                for i in 44..=50 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[1].insert(i, value);
                }
            }
            ("ie", Some("ogam")) => {
                self.level_keymap[1].insert(43, '\u{1680}');
            }
            ("ie", Some("ogam_is434")) => {
                for (code, value) in self.level_keymap[2].clone() {
                    if self.level_keymap[3].get(&code).is_none() {
                        self.level_keymap[3].insert(code, value);
                    }
                }
                for (code, value) in self.level_keymap[0].clone() {
                    if [2, 3, 4, 6, 7, 9, 10, 11, 12, 13].contains(&code) {
                        self.level_keymap[2].insert(code, value);
                        let value = *self.level_keymap[1].get(&code).unwrap();
                        self.level_keymap[3].insert(code, value);
                    }
                }
            }
            ("si", Some(_)) => {
                let value = *self.level_keymap[0].get(&41).unwrap();
                self.level_keymap[2].insert(41, value);
                let value = *self.level_keymap[1].get(&41).unwrap();
                self.level_keymap[3].insert(41, value);
            }
            ("se", Some("rus")) => {
                let value = *self.level_keymap[0].get(&13).unwrap();
                self.level_keymap[2].insert(13, value);
                let value = *self.level_keymap[1].get(&13).unwrap();
                self.level_keymap[3].insert(13, value);
                for i in 16..=27 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[2].insert(i, value);
                    let value = *self.level_keymap[1].get(&i).unwrap();
                    self.level_keymap[3].insert(i, value);
                }
                for i in 30..=41 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[2].insert(i, value);
                    let value = *self.level_keymap[1].get(&i).unwrap();
                    self.level_keymap[3].insert(i, value);
                }
                for i in 43..=50 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[2].insert(i, value);
                    let value = *self.level_keymap[1].get(&i).unwrap();
                    self.level_keymap[3].insert(i, value);
                }
                let value = *self.level_keymap[0].get(&86).unwrap();
                self.level_keymap[2].insert(86, value);
                let value = *self.level_keymap[1].get(&86).unwrap();
                self.level_keymap[3].insert(86, value);
            }
            ("us", Some("3l")) => {
                for i in 2..=11 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[1].insert(i, value);
                    self.level_keymap[3].insert(i, value);
                }
                let value = *self.level_keymap[0].get(&15).unwrap();
                self.level_keymap[1].insert(15, value);
                self.level_keymap[2].insert(15, value);
                self.level_keymap[3].insert(15, value);
                self.level_keymap[4].insert(15, value);
                let value = *self.level_keymap[0].get(&58).unwrap();
                self.level_keymap[1].insert(58, value);
                self.level_keymap[3].insert(58, value);
                let value = *self.level_keymap[0].get(&86).unwrap();
                self.level_keymap[4].insert(86, value);
                let value = *self.level_keymap[1].get(&86).unwrap();
                self.level_keymap[5].insert(86, value);
                let value = *self.level_keymap[2].get(&86).unwrap();
                self.level_keymap[6].insert(86, value);
            }
            ("us", Some("3l-cros")) => {
                for i in 2..=11 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[1].insert(i, value);
                    self.level_keymap[3].insert(i, value);
                }
                let value = *self.level_keymap[0].get(&15).unwrap();
                self.level_keymap[1].insert(15, value);
                self.level_keymap[2].insert(15, value);
                self.level_keymap[3].insert(15, value);
                self.level_keymap[4].insert(15, value);
                let value = *self.level_keymap[0].get(&58).unwrap();
                self.level_keymap[1].insert(58, value);
                self.level_keymap[3].insert(58, value);
                let value = *self.level_keymap[0].get(&86).unwrap();
                self.level_keymap[4].insert(86, value);
                let value = *self.level_keymap[1].get(&86).unwrap();
                self.level_keymap[5].insert(86, value);
                let value = *self.level_keymap[2].get(&86).unwrap();
                self.level_keymap[6].insert(86, value);
                let value = *self.level_keymap[0].get(&125).unwrap();
                self.level_keymap[1].insert(125, value);
                self.level_keymap[3].insert(125, value);
            }
            ("us", Some("3l-emacs")) => {
                for i in 2..=11 {
                    let value = *self.level_keymap[0].get(&i).unwrap();
                    self.level_keymap[1].insert(i, value);
                    self.level_keymap[3].insert(i, value);
                }
                let value = *self.level_keymap[0].get(&15).unwrap();
                self.level_keymap[1].insert(15, value);
                self.level_keymap[3].insert(15, value);
                let value = *self.level_keymap[0].get(&86).unwrap();
                self.level_keymap[4].insert(86, value);
                let value = *self.level_keymap[1].get(&86).unwrap();
                self.level_keymap[5].insert(86, value);
                let value = *self.level_keymap[2].get(&86).unwrap();
                self.level_keymap[6].insert(86, value);
            }
            ("fr", Some("bepo_latin9")) => {
                self.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
                let value = *self.level_keymap[2].get(&55).unwrap();
                self.level_keymap[3].insert(55, value);
                let value = *self.level_keymap[2].get(&98).unwrap();
                self.level_keymap[3].insert(98, value);
            }
            ("fr", Some("oss_latin9")) | ("be", Some("oss_latin9")) => {
                let value = *self.level_keymap[2].get(&55).unwrap();
                self.level_keymap[3].insert(55, value);
                let value = *self.level_keymap[2].get(&98).unwrap();
                self.level_keymap[3].insert(98, value);
            }
            ("fr", Some("mac")) => {
                let value = *self.level_keymap[0].get(&83).unwrap();
                self.level_keymap[3].insert(83, value);
            }
            ("fr", Some("azerty")) => {
                // let value = *self.level_keymap[0].get(&83).unwrap();
                // self.level_keymap[3].insert(83, value);
            }
            ("de", Some("T3")) => {
                self.custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
                self.level_keymap[3].insert(2, 'ʹ');
                self.level_keymap[3].insert(3, 'ʺ');
                self.level_keymap[3].insert(4, 'ʿ');
                self.level_keymap[3].insert(5, 'ʾ');
                self.level_keymap[3].insert(6, 'ˁ');
                self.level_keymap[3].insert(7, 'ˀ');
                self.level_keymap[3].insert(8, '{');
                self.level_keymap[3].insert(9, '}');
                self.level_keymap[3].insert(10, '[');
                self.level_keymap[3].insert(11, ']');
                self.level_keymap[3].insert(12, 'ʻ');
                self.level_keymap[3].insert(13, '¬');
                self.level_keymap[3].insert(15, '\t');
                self.level_keymap[3].insert(16, '\u{30d}');
                self.level_keymap[3].insert(27, '@');
                self.level_keymap[3].insert(30, '\u{329}');
                self.level_keymap[3].insert(35, '\u{332}');
                self.level_keymap[3].insert(38, '\u{338}');
                self.level_keymap[3].insert(39, '°');
                self.level_keymap[3].insert(40, '′');
                self.level_keymap[3].insert(41, '|');
                self.level_keymap[3].insert(43, '″');
                self.level_keymap[3].insert(44, '«');
                self.level_keymap[3].insert(45, '»');
                self.level_keymap[3].insert(46, '―');
                self.level_keymap[3].insert(47, '‹');
                self.level_keymap[3].insert(48, '›');
                self.level_keymap[3].insert(49, '–');
                self.level_keymap[3].insert(50, '—');
                self.level_keymap[3].insert(51, '$');
                self.level_keymap[3].insert(52, '#');
                self.level_keymap[3].insert(53, '‑');
                self.level_keymap[3].insert(57, '\u{a0}');
                // self.level_keymap[3].insert(86, '¦');
            }
            _ => {}
        }
        if self.level_keymap.len() > 1 {
            for (code, value) in self.level_keymap[0].clone() {
                if self.level_keymap[1].get(&code).is_none() {
                    self.level_keymap[1].insert(code, value);
                }
            }
        }
        for i in 0..self.level_keymap.len() {
            let next_large = std::cmp::min(i + 2, self.level_keymap.len() - 1);
            let next_small = std::cmp::min(i + 1, self.level_keymap.len() - 1);
            let next = std::cmp::max(next_small, next_large);
            if next > i {
                for (code, value) in &self.level_keymap[i].clone() {
                    if self.level_keymap[next].get(&code).is_none() {
                        self.level_keymap[next].insert(*code, *value);
                    }
                }
            }
        }
        for i in 0..self.level_keymap.len() {
            for (code, value) in &DEFAULT_MAP[i] {
                if self.level_keymap[i].get(&code).is_none() {
                    self.level_keymap[i].insert(*code, *value);
                }
            }
        }
    }

    // This recursive function from hell is currently used only to map xkb
    // hopefully xkb files can be depricated one day so one does not have to work
    // with this cursed file format.
    fn map_xkb(&mut self, path: &Path, locale: String, layout: Option<String>) {
        let file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
        let xkb = parse(&file).unwrap();
        xkb.definitions.iter().for_each(|d| {
            if let Directive::XkbSymbols(src) = &d.directive {
                let layout = layout.clone().unwrap_or(self.current_layout());
                if src.name.content == layout {
                    src.value.iter().for_each(|si| {
                        if let XkbSymbolsItem::Include(Include { name }) = si {
                            let (locale, layout) = parse_include(name);
                            if layout.is_none() {
                                self.map_xkb(path, locale, Some("basic".to_string()));
                            } else {
                                self.map_xkb(path, locale, layout);
                            }
                        } else if let XkbSymbolsItem::Key(Key {
                            mode: _,
                            id,
                            values,
                        }) = si
                        {
                            // Why not just unwrap here? Surely there shouldn't be any symbols not defined in evdev...
                            // ph has AB00 which does not exists in evdev...
                            if let Some(evdev_code) = XKBCODES_EVDEV.get(id.content) {
                                // println!("{:?} {:?}", values, id);
                                values.iter().for_each(|v| {
                                    if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
                                        if let xkb_parser::ast::KeyDef::SymbolDef(key) = key_defs {
                                            for (i, v) in key.values.values.iter().enumerate() {
                                                let single_char =
                                                    XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned();
                                                if let Some(char) = single_char {
                                                    self.level_keymap[i].insert(*evdev_code, char);
                                                }
                                            }
                                        }
                                    } else if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                        self.map_keys_and_modifiers(
                                            key,
                                            evdev_code,
                                            layout.clone(),
                                            locale.clone(),
                                            id.content.to_owned(),
                                        );
                                    }
                                })
                            }
                        }
                    })
                }
            }
        });
    }

    fn map_keys_and_modifiers(
        &mut self,
        key: &xkb_parser::ast::KeyNames,
        evdev_code: &u32,
        layout: String,
        locale: String,
        id: String,
    ) {
        if id == "CAPS" {
            if key.values.first().is_some_and(|k| k.content == "BackSpace") {
                self.modifiers.remap.insert(*evdev_code, 14);
                let value = *self.level_keymap[0].get(&14).unwrap();
                self.level_keymap[1].insert(58, value);
            } else if key.values.first().is_some_and(|k| k.content == "Tab") {
                self.modifiers.remap.insert(*evdev_code, 15);
            }
        }
        for (i, v) in key.values.iter().enumerate() {
            if i == self.level_keymap.len() {
                self.level_keymap.push(DEFAULT_MAP[i].clone());
            }
            if let Some(remap_key_code) = XKBCODES_EVDEV.get(v.content) {
                self.modifiers.remap.insert(*evdev_code, *remap_key_code);
            }
            let mut chars = v.chars();
            let count = chars.clone().count();
            let first_char = chars.next();
            let is_hex = chars.all(|c| c.is_ascii_hexdigit());
            let mut chars = v.chars();
            chars.next();
            let second_char = chars.next();
            let single_char = if count == 1 && first_char.is_some_and(|c| c.is_alphanumeric()) {
                first_char
            } else if first_char.is_some_and(|c| c == 'U') && is_hex {
                unicode_string_to_unicode_char(v)
            } else if first_char.is_some_and(|c| c == '0') && second_char.is_some_and(|c| c == 'x')
            {
                hex_string_to_unicode_char(v)
            } else {
                XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned()
            };
            if let Some(single_char) = single_char {
                self.level_keymap[i].insert(*evdev_code, single_char);
            } else {
                match (v.content, layout.as_str()) {
                    ("Eisu_toggle", _) => {
                        self.modifiers.shift_caps = true;
                    }
                    ("Control_L", _) => {
                        if id == "CAPS" {
                            self.modifiers.remap.insert(*evdev_code, 29);
                        }
                    }
                    ("Shift_Lock", _) => {
                        self.modifiers.level2lock = (*evdev_code, LockModifier::default());
                    }
                    ("ISO_Level3_Shift", _) => {
                        if self.modifiers.level3shift.0 .0 != 0 {
                            self.modifiers.level3shift.1 = Some((*evdev_code, Modifier::default()))
                        } else {
                            self.modifiers.level3shift.0 = (*evdev_code, Modifier::default());
                        }
                    }
                    ("ISO_Level3_Lock", _) => {
                        self.modifiers.level3lock = (*evdev_code, LockModifier::default());
                    }
                    ("ISO_Level3_Latch", _) => {
                        self.modifiers.level3latch = (*evdev_code, LatchModifier::default());
                    }
                    ("ISO_Level5_Shift", _) => {
                        self.modifiers.level5shift = (*evdev_code, Modifier::default());
                    }
                    ("ISO_Level5_Lock", _) => {
                        self.modifiers.level5lock = (*evdev_code, LockModifier::default());
                    }
                    ("ISO_Level5_Latch", _) => {
                        self.modifiers.level5latch = (*evdev_code, LatchModifier::default());
                    }
                    ("Multi_key", _) => {
                        self.modifiers.compose_key = (*evdev_code, Modifier::default());
                    }
                    (_, "rshift_both_shiftlock") | (_, "lshift_both_shiftlock") => {
                        self.modifiers.right_left_shift_caps = true;
                    }
                    (_, "bksl_switch") => {
                        if self.modifiers.level3shift.0 .0 != 0 {
                            self.modifiers.level3shift.1 = Some((*evdev_code, Modifier::default()))
                        } else {
                            self.modifiers.level3shift.0 = (*evdev_code, Modifier::default());
                        }
                    }
                    (_, "caps_switch") => {
                        if locale == "level3" {
                            if self.modifiers.level3shift.0 .0 != 0 {
                                self.modifiers.level3shift.1 =
                                    Some((*evdev_code, Modifier::default()))
                            } else {
                                self.modifiers.level3shift.0 = (*evdev_code, Modifier::default());
                            }
                        } else {
                            self.modifiers.level5shift = (58, Modifier::default());
                        };
                    }
                    _ => {
                        if v.contains("none") {
                            if i > 0 {
                                if let Some(key) = self.level_keymap[i - 1].clone().get(evdev_code)
                                {
                                    self.level_keymap[i].insert(*evdev_code, *key);
                                }
                            }
                        } else if !v.contains("NoSymbol")
                            && !v.contains("any")
                            && !v.contains("VoidSymbol")
                        {
                            // println!("{:?}", v);
                            // println!("{}", *evdev_code);
                            // println!("{}", layout);
                            // println!("{}", id);
                        }
                    }
                }
                // match layout.as_str() {
                //     "ralt_switch" => {
                //         self.modifiers.level3 = (100, Modifier::default());
                //     }
                //     "lalt_switch" => {
                //         self.modifiers.level3 = (56, Modifier::default());
                //     }
                //     "enter_switch" => {
                //         self.modifiers.level3 = (96, Modifier::default());
                //     }
                //     "rctrl_switch" | "switch" => {
                //         if locale == "level3" {
                //             self.modifiers.level3 = (97, Modifier::default());
                //         } else {
                //             self.modifiers.level5 = (97, Modifier::default());
                //         };
                //     }
                //     "lock" => {
                //         // Hypr
                //         self.modifiers.level5lock = (199, LockModifier::default());
                //     }
                //     "ralt_switch_lock" => {
                //         self.modifiers.level5lock = (100, LockModifier::default());
                //     }
                //     "lsgt_switch_lock" => {
                //         self.modifiers.level5lock = (86, LockModifier::default());
                //     }
            }
        }
    }
}
