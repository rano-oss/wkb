use std::{
    collections::{HashMap, HashSet},
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
struct Modifier {
    pressed: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct LockModifier {
    pressed: bool,
    locked: u8,
}

#[derive(Debug, Clone, Default)]
pub struct Modifiers {
    left_ctrl: Modifier,
    right_ctrl: Modifier,
    alt: Modifier,
    alt_gr: Modifier,
    left_shift: Modifier,
    right_shift: Modifier,
    logo: Modifier,
    caps_lock: LockModifier,
    caps_lock_disabled: bool,
    right_left_shift_caps: bool,
    num_lock: LockModifier,
    scroll_lock: LockModifier,
    level2: ((u32, Modifier), (u32, Modifier)),
    level3: (u32, Modifier),
    level5: (u32, Modifier),
    level5lock: (u32, LockModifier),
}

impl Modifiers {
    pub fn evdev_codes(&self) -> Vec<u32> {
        vec![
            self.level2.0 .0,
            self.level2.1 .0,
            self.level3.0,
            self.level5.0,
            self.level5lock.0,
        ]
    }

    fn level5(&self) -> bool {
        self.level5.1.pressed || self.level5lock.1.locked > 0
    }

    fn level3(&self) -> bool {
        self.level3.1.pressed
    }

    fn level2(&self) -> bool {
        self.level2.0 .1.pressed || self.level2.1 .1.pressed
    }

    fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let mut updated = false;
        if self.level2.0 .0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level2.0 .1.pressed = true,
                KeyDirection::Up => self.level2.0 .1.pressed = false,
            }
            updated = true;
        } else if self.level2.1 .0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level2.1 .1.pressed = true,
                KeyDirection::Up => self.level2.1 .1.pressed = false,
            }
            updated = true;
        } else if self.level3.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level3.1.pressed = true,
                KeyDirection::Up => self.level3.1.pressed = false,
            }
            updated = true;
        } else if self.level5.0 == evdev_code {
            match key_direction {
                KeyDirection::Down => self.level5.1.pressed = true,
                KeyDirection::Up => self.level5.1.pressed = false,
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
                    if self.level5lock.1.locked == 0 {
                        self.level5lock.1.locked -= 1;
                    }
                }
            }
            updated = true;
        }
        match (evdev_code, key_direction) {
            (29, KeyDirection::Down) => self.left_ctrl.pressed = true,
            (29, KeyDirection::Up) => self.left_ctrl.pressed = false,
            (42, KeyDirection::Down) => self.left_shift.pressed = true,
            (42, KeyDirection::Up) => self.left_shift.pressed = false,
            (54, KeyDirection::Down) => self.right_shift.pressed = true,
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
                if self.caps_lock.locked == 0 {
                    self.caps_lock.locked = 2;
                }
            }
            (58, KeyDirection::Up) => {
                self.caps_lock.pressed = false;
                if self.caps_lock.locked > 0 {
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

fn to_uppercase(c: char, custom_uppercase_map: Option<HashMap<char, char>>) -> char {
    if let Some(uppercase_map) = custom_uppercase_map {
        if let Some(uc) = uppercase_map.get(&c) {
            return *uc;
        }
    }
    let uc = c.to_uppercase();
    if uc.len() > 1 {
        return c;
    }
    uc.last().unwrap()
}

type LevelKeymap = [HashMap<u32, char>; 8];

#[derive(Debug, Clone)]
pub struct WKB {
    layouts: Vec<String>,
    layout: String,
    level_keymap: LevelKeymap,
    compose_status: ComposeStatus,
    compose_char: char,
    pressed_keys: HashSet<u32>,
    repeat_keys: HashSet<u32>,
    custom_uppercase_map: Option<HashMap<char, char>>,
    caps_level2: Option<Vec<u32>>,
    pub modifiers: Modifiers,
}

impl WKB {
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let level_keymap: [HashMap<u32, char>; 8] = core::array::from_fn(|_| HashMap::new());
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
        modifiers.level2 = ((42, Modifier::default()), (54, Modifier::default()));
        let mut wkb = Self {
            layouts,
            layout: layout.clone(),
            compose_status: ComposeStatus::Idle,
            level_keymap,
            compose_char: char::default(),
            pressed_keys: HashSet::new(),
            repeat_keys,
            custom_uppercase_map: None,
            caps_level2: None,
            modifiers,
        };
        wkb.map(path, locale.clone(), Some(layout.clone()));
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

    pub fn utf8(&self, evdev_code: u32) -> Option<char> {
        let level5 = self.modifiers.level5();
        let level3 = self.modifiers.level3();
        let level2 = self.modifiers.level2();
        if [
            71, 72, 73, // 7, 8, 9
            75, 76, 77, //4, 5, 6
            79, 80, 81, // 1, 2, 3
            82, // 0
        ]
        .contains(&evdev_code)
            && self.modifiers.num_lock.locked > 0
            // && !level2
            && !level3
            && !level5
        {
            self.level_key(evdev_code, 1)
        } else if level5 && level3 && level2 {
            self.level_key(evdev_code, 7)
        } else if level5 && level3 {
            self.level_key(evdev_code, 6)
        } else if level5 && level2 {
            self.level_key(evdev_code, 5)
        } else if level5 {
            self.level_key(evdev_code, 4)
        } else if level3 && level2 {
            if self.modifiers.caps_lock.locked > 0 {
                self.level_key(evdev_code, 3)
                    .map(|c| c.to_uppercase().last().unwrap())
            } else {
                self.level_key(evdev_code, 3)
            }
        } else if level3 {
            // if self.modifiers.caps_lock.locked > 0 {
            //     self.level_key(evdev_code, 2)
            //         .map(|c| c.to_uppercase().last().unwrap())
            // } else {
            self.level_key(evdev_code, 2)
            // }
        } else if level2 {
            if self.modifiers.caps_lock.locked > 0 {
                self.level_key(evdev_code, 1)
                    .map(|c| c.to_lowercase().last().unwrap())
            } else {
                self.level_key(evdev_code, 1)
            }
        } else {
            if (self.modifiers.caps_lock.locked > 0
                || (self.modifiers.right_left_shift_caps
                    && self.modifiers.right_shift.pressed
                    && self.modifiers.left_shift.pressed))
                && !self.modifiers.caps_lock_disabled
            {
                if let Some(caps_level2_list) = &self.caps_level2 {
                    if caps_level2_list.contains(&evdev_code) {
                        return self.level_key(evdev_code, 1);
                    }
                }
                self.level_key(evdev_code, 0)
                    .map(|c| to_uppercase(c, self.custom_uppercase_map.clone()))
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

    pub fn level_keymap(&self) -> LevelKeymap {
        self.level_keymap.clone()
    }

    // This recursive function from hell is currently used only to map xkb
    // hopefully xkb files can be depricated one day so one does not have to work
    // with this cursed file format.
    fn fix_xkb_edge_cases(&mut self, locale: String, layout: Option<String>) {
        match (locale.as_str(), &layout.as_deref()) {
            ("at", Some("basic"))
            | ("at", Some("nodeadkeys"))
            | ("de", Some("basic"))
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
            | ("de", Some("adnw_base"))
            | ("de", Some("koy_base"))
            | ("de", Some("bone_base"))
            | ("de", Some("bone_eszett_home_base"))
            | ("de", Some("neo_qwertz_base"))
            | ("de", Some("neo_qwerty_base"))
            | ("it", Some("lldde"))
            | ("de", Some("nodeadkeys")) => {
                self.custom_uppercase_map = Some(HashMap::from([('ß', 'ẞ')]));
            }
            ("tr", Some("basic"))
            | ("tr", Some("e"))
            | ("gh", Some("hausa"))
            | ("ua", Some("crh"))
            | ("ua", Some("crh_f"))
            | ("ro", Some("crh_dobruja"))
            | ("tr", Some("f")) => {
                self.custom_uppercase_map = Some(HashMap::from([('i', 'İ')]));
            }
            ("ng", Some("hausa")) | ("gh", Some("fula")) | ("tr", Some("us")) => {
                self.custom_uppercase_map = Some(HashMap::from([('ı', 'İ')]));
            }
            ("md", Some("gag")) => {
                self.custom_uppercase_map = Some(HashMap::from([('ı', 'ı')]));
            }
            ("hu", Some("oldhunlig")) => {
                self.caps_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41, 51, 52, 53]);
            }
            ("hu", Some("oldhun_base")) => {
                self.caps_level2 = Some(vec![51, 52, 53]);
            }
            ("hu", Some("oldhun_origin")) | ("hu", Some("oldhun_lig")) => {
                self.caps_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41]);
            }
            ("kz", Some("ext")) => self.caps_level2 = Some(vec![2, 7, 8, 43]),
            ("fr", Some("bepo")) | ("fr", Some("bepo_latin9")) | ("fr", Some("bepo_afnor")) => {
                self.custom_uppercase_map = Some(HashMap::from([
                    ('"', '1'),
                    ('«', '2'),
                    ('»', '3'),
                    ('(', '4'),
                    (')', '5'),
                    ('@', '6'),
                    ('+', '7'),
                    ('-', '8'),
                    ('/', '9'),
                    ('*', '0'),
                ]));
            }
            ("kz", Some("latin")) => {
                self.custom_uppercase_map = Some(HashMap::from([('v', 'M')]));
            }
            ("us", Some("chr")) => {
                self.caps_level2 = Some(vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                    25, 26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47,
                    48, 49, 50, 51, 52, 53,
                ])
            }
            ("il", Some("si2")) | ("il", Some("basic")) => {
                self.caps_level2 = Some(vec![
                    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                    44, 45, 46, 47, 48, 49, 50, 51, 52,
                ])
            }
            ("il", Some("phonetic")) => {
                self.caps_level2 = Some(vec![20, 25, 33, 37, 39, 46, 49, 50, 51, 52])
            }
            ("il", Some("biblical")) => {
                self.caps_level2 = Some(vec![
                    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48,
                    49, 50, 51, 52, 53,
                ])
            }
            ("il", Some("biblicalSIL")) => {
                self.caps_level2 = Some(vec![
                    2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 16, 18, 21, 24, 25, 26, 27, 30, 31, 33, 36,
                    37, 39, 40, 41, 46, 49, 50, 51, 52, 53,
                ])
            }
            // ("ie", Some("ogam_is434")) => {
            //     self.caps_level2 = Some(vec![
            //         5, 8, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37,
            //         38, 41, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 86,
            //     ])
            // }
            ("pl", Some("dvp")) | ("us", Some("dvp")) => {
                self.caps_level2 = Some(vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 40])
            }
            ("in", Some("tamilnet_TAB")) => {
                self.custom_uppercase_map = Some(HashMap::from([('ø', 'Á'), ('ù', 'Â')]))
            }
            ("lk", Some("tam_TAB")) => {
                self.custom_uppercase_map = Some(HashMap::from([('ø', 'Á'), ('ù', 'Â')]))
            }
            ("in", Some("iipa")) => self.custom_uppercase_map = Some(HashMap::from([('b', 'Y')])),
            ("us", Some("colemak"))
            | ("us", Some("colemak_dh"))
            | ("us", Some("colemak_dh_wide"))
            | ("us", Some("colemak_dh_iso"))
            | ("us", Some("colemak_dh_wide_iso"))
            | ("us", Some("colemak_dh_ortho"))
            | ("us", Some("workman"))
            | ("us", Some("workman-intl"))
            | ("us", Some("workman-p"))
            | ("al", Some("veqilharxhi"))
            | ("ge", Some("basic"))
            | ("ge", Some("ru"))
            | ("fr", Some("geo"))
            | ("pt", Some("colemak"))
            | ("pl", Some("colemak"))
            | ("pl", Some("colemak_dh_ansi"))
            | ("pl", Some("colemak_dh"))
            | ("gr", Some("colemak"))
            | (_, Some("rulemak"))
            | ("jp", Some("106"))
            | ("lt", Some("lekp"))
            | ("jp", Some("common"))
            | ("gb", Some("colemak")) => {
                self.modifiers.caps_lock_disabled = true;
            }
            ("gr", Some(_)) => {
                self.custom_uppercase_map = Some(HashMap::from([('ς', 'ς')]));
            }
            // There seems to be syntax for adding symbols twice if it is the only one on two levels
            // this seems to fix that problem...
            ("tr", Some("ku")) | ("tr", Some("ku_f")) => self.custom_uppercase_map = None,
            ("pl", Some("glagolica")) => {
                self.level_keymap[1].insert(43, '|');
            }
            ("pl", _)
            | ("bqn", _)
            | ("ancient", _)
            | ("ru", _)
            | ("tg", _)
            | ("pk", _)
            | ("lt", _)
            | ("in", _)
            | ("ma", _)
            | ("dz", _)
            | ("gn", _)
            | ("la", _)
            | ("fr", Some("azerty"))
            | ("apl", Some("dyalog_base"))
            | ("apl", Some("common"))
            | ("apl", Some("unified"))
            | ("apl", Some("sax"))
            | ("apl", Some("aplx"))
            | ("jp", Some("kana"))
            | ("jp", Some("hztg_escape")) => {
                for (code, value) in self.level_keymap[0].clone() {
                    if self.level_keymap[1].get(&code).is_none() {
                        self.level_keymap[1].insert(code, value);
                    }
                }
            }
            ("us", Some("3l")) | ("us", Some("3l-cros")) | ("us", Some("norman")) => {
                self.modifiers.caps_lock_disabled = true;
                for (code, value) in self.level_keymap[0].clone() {
                    if self.level_keymap[1].get(&code).is_none() {
                        self.level_keymap[1].insert(code, value);
                    }
                }
            }
            ("jp", Some("OADG109A")) => {
                self.modifiers.caps_lock_disabled = true;
                for (code, value) in self.level_keymap[0].clone() {
                    if self.level_keymap[1].get(&code).is_none() {
                        self.level_keymap[1].insert(code, value);
                    }
                }
            }
            ("us", Some("3l-emacs")) => {
                self.level_keymap[1].insert(15, '\t');
            }
            ("apl", Some("dyalog_codes")) => {
                self.level_keymap[1].insert(103, '\u{f820}');
                self.level_keymap[1].insert(108, '\u{f81f}');
            }
            ("ie", Some("ogam")) => {
                for (code, value) in self.level_keymap[0].clone() {
                    if self.level_keymap[1].get(&code).is_none() {
                        self.level_keymap[1].insert(code, value);
                    }
                }
                self.level_keymap[1].insert(43, '\u{1680}');
            }
            _ => {}
        }
        for i in 0..5 {
            for (code, value) in &DEFAULT_MAP[i] {
                if self.level_keymap[i].get(&code).is_none() {
                    self.level_keymap[i].insert(*code, *value);
                }
            }
        }
    }

    fn map(&mut self, path: &Path, locale: String, layout: Option<String>) {
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
                                self.map(path, locale, Some("basic".to_string()));
                            } else {
                                self.map(path, locale, layout);
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
                                //snowflake special case of bad design I don't want to support any other way!
                                let evdev_code =
                                    match (locale.as_str(), layout.as_str(), evdev_code) {
                                        ("de", "ru", 21) => 44,
                                        ("de", "ru", 44) => 21,
                                        ("de", "ru-recom", 21) => 44,
                                        ("de", "ru-recom", 44) => 21,
                                        ("de", "ru-translit", 21) => 44,
                                        ("de", "ru-translit", 44) => 21,
                                        _ => *evdev_code,
                                    };
                                values.iter().for_each(|v| {
                                    if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
                                        if let xkb_parser::ast::KeyDef::SymbolDef(key) = key_defs {
                                            for (i, v) in key.values.values.iter().enumerate() {
                                                let single_char =
                                                    XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned();
                                                if let Some(char) = single_char {
                                                    self.level_keymap[i].insert(evdev_code, char);
                                                }
                                            }
                                        }
                                    } else if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                        for (i, v) in key.values.iter().enumerate() {
                                            let mut chars = v.chars();
                                            let count = chars.clone().count();
                                            let first_char = chars.next();
                                            let is_hex = chars.all(|c| c.is_ascii_hexdigit());
                                            let mut chars = v.chars();
                                            chars.next();
                                            let second_char = chars.next();
                                            let single_char = if count == 1
                                                && first_char.is_some_and(|c| c.is_alphanumeric())
                                            {
                                                first_char
                                            } else if first_char.is_some_and(|c| c == 'U') && is_hex
                                            {
                                                unicode_string_to_unicode_char(v)
                                            } else if first_char.is_some_and(|c| c == '0')
                                                && second_char.is_some_and(|c| c == 'x')
                                            {
                                                hex_string_to_unicode_char(v)
                                            } else {
                                                XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned()
                                            };
                                            if let Some(single_char) = single_char {
                                                self.level_keymap[i]
                                                    .insert(evdev_code, single_char);
                                            } else {
                                                match layout.as_str() {
                                                    "ralt_switch" => {
                                                        self.modifiers.level3 =
                                                            (100, Modifier::default());
                                                    }
                                                    "lalt_switch" => {
                                                        self.modifiers.level3 =
                                                            (56, Modifier::default());
                                                    }
                                                    "enter_switch" => {
                                                        self.modifiers.level3 =
                                                            (96, Modifier::default());
                                                    }
                                                    "bksl_switch" => {
                                                        self.modifiers.level3 =
                                                            (43, Modifier::default());
                                                    }
                                                    "rctrl_switch" | "switch" => {
                                                        if locale == "level3" {
                                                            self.modifiers.level3 =
                                                                (97, Modifier::default());
                                                        } else {
                                                            self.modifiers.level5 =
                                                                (97, Modifier::default());
                                                        };
                                                    }
                                                    "caps_switch" => {
                                                        if locale == "level3" {
                                                            self.modifiers.level3 =
                                                                (58, Modifier::default());
                                                        } else {
                                                            self.modifiers.level5 =
                                                                (58, Modifier::default());
                                                        };
                                                    }
                                                    "lock" => {
                                                        // Hypr
                                                        self.modifiers.level5lock =
                                                            (199, LockModifier::default());
                                                    }
                                                    "ralt_switch_lock" => {
                                                        self.modifiers.level5lock =
                                                            (100, LockModifier::default());
                                                    }
                                                    "lsgt_switch_lock" => {
                                                        self.modifiers.level5lock =
                                                            (86, LockModifier::default());
                                                    }
                                                    "lshift_both_shiftlock"
                                                    | "rshift_both_shiftlock" => {
                                                        self.modifiers.right_left_shift_caps = true
                                                    }
                                                    _ => {
                                                        // if !v.contains("VoidSymbol") {
                                                        // println!("{:?}", v);
                                                        // println!("{}", *evdev_code);
                                                        println!("{}", layout);
                                                        // }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    })
                }
            }
        });
    }
}
