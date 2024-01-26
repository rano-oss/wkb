use std::{collections::HashMap, fs::File, io::Read, os::fd::OwnedFd, path::Path};

use default_keymap::{
    DEFAULT_LEVEL1_KEYPADMAP, DEFAULT_LEVEL1_MAP, DEFAULT_LEVEL2_KEYPADMAP, DEFAULT_LEVEL2_MAP,
    DEFAULT_LEVEL3_KEYPADMAP, DEFAULT_LEVEL3_MAP, DEFAULT_LEVEL4_KEYPADMAP, DEFAULT_LEVEL4_MAP,
};
use evdev_xkb::XKBCODES_EVDEV;
use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};
use xkb_utf8::XKBCODES_DEF_TO_UTF8;
mod default_keymap;
pub mod evdev_xkb;
mod xkb_utf8;

// TODO: replace this with proper modifiers from xkbcommon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharModifier {
    None,
    Shift,
    CapsLock,
    AltGr,
    AltGrAndShift,
}

#[derive(Debug, Clone, Default, Copy)]
struct Modifiers {
    alt: bool,
    shift: bool,
    caps_lock: bool,
    alt_gr: bool,
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

type Keymap = HashMap<CharModifier, HashMap<u32, char>>;
type LevelKeymap = Vec<HashMap<u32, char>>;
type LevelKeypadmap = Vec<HashMap<u32, char>>;

#[derive(Debug, Clone)]
pub struct WKB {
    layouts: Vec<String>,
    layout: String,
    keymap: Keymap,
    level_keymap: LevelKeymap,
    level_keypadmap: LevelKeypadmap,
    modifiers: Modifiers,
    compose_status: ComposeStatus,
    compose_char: char,
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

impl WKB {
    pub fn new_from_fd(fd: OwnedFd) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let level_keymap = vec![
            DEFAULT_LEVEL1_MAP.clone(),
            DEFAULT_LEVEL2_MAP.clone(),
            DEFAULT_LEVEL3_MAP.clone(),
        ];
        let mut wkb = Self {
            layouts: Vec::new(),
            layout: String::new(),
            keymap: HashMap::new(),
            modifiers: Modifiers::default(),
            compose_status: ComposeStatus::Idle,
            level_keymap,
            level_keypadmap: Vec::new(),
            compose_char: char::default(),
        };
        wkb.read_layouts(path, None, Some(fd));
        wkb
    }

    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let level_keypadmap = vec![
            DEFAULT_LEVEL1_KEYPADMAP.clone(),
            DEFAULT_LEVEL2_KEYPADMAP.clone(),
            DEFAULT_LEVEL3_KEYPADMAP.clone(),
            DEFAULT_LEVEL4_KEYPADMAP.clone(),
        ];
        let level_keymap = vec![
            DEFAULT_LEVEL1_MAP.clone(),
            DEFAULT_LEVEL2_MAP.clone(),
            DEFAULT_LEVEL3_MAP.clone(),
            DEFAULT_LEVEL4_MAP.clone(),
        ];
        let mut wkb = Self {
            layouts: Vec::new(),
            layout: String::new(),
            keymap: HashMap::new(),
            modifiers: Modifiers::default(),
            compose_status: ComposeStatus::Idle,
            level_keymap,
            level_keypadmap,
            compose_char: char::default(),
        };
        wkb.read_layouts(path, Some(locale.clone()), None);
        wkb.layout = if let Some(layout) = layout {
            layout
        } else {
            wkb.layouts()[0].clone()
        };
        wkb.map(path, locale, None);
        wkb
    }

    pub fn level_key(&self, evdev_code: u32, level_index: usize) -> Option<char> {
        if let Some(character) = self
            .level_keymap
            .get(level_index)
            .and_then(|hm| hm.get(&evdev_code).copied())
        {
            Some(character)
        } else if let Some(character) = self
            .level_keypadmap
            .get(level_index)
            .and_then(|hm| hm.get(&evdev_code).copied())
        {
            Some(character)
        } else {
            None
        }
    }

    pub fn key(&self, evdev_code: u32) -> Option<char> {
        match self.modifiers {
            Modifiers {
                shift: false,
                caps_lock: false,
                alt_gr: false,
                alt: false,
            } => {
                let mods_map = self.keymap.get(&CharModifier::None).unwrap();
                mods_map.get(&evdev_code).copied()
            }
            _ => None,
        }
    }

    pub fn compose_feed(&mut self, evdev_code: u32) -> FeedResult {
        if let Some(character) = self.key(evdev_code) {
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

    pub fn level_keypadmap(&self) -> LevelKeypadmap {
        self.level_keypadmap.clone()
    }

    pub fn keymap(&self) -> Keymap {
        self.keymap.clone()
    }

    fn read_layouts(&mut self, path: &Path, locale: Option<String>, fd: Option<OwnedFd>) {
        let mut string_file = String::new();
        if let Some(fd) = fd {
            let mut file = File::from(fd);
            file.read_to_string(&mut string_file)
                .expect("Could not read file");
        } else if let Some(locale) = locale {
            string_file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
        }
        let xkb = parse(&string_file).expect("neither names nor file set");
        xkb.definitions.iter().for_each(|d| match &d.directive {
            Directive::XkbSymbols(src) => {
                self.layouts.push(src.name.content.to_string());
            }
            _ => {}
        })
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
                                values.iter().for_each(|v| {
                                    if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                        for (i, v) in key.values.iter().enumerate() {
                                            if locale != "keypad" && i >= self.level_keymap.len() {
                                                self.level_keymap.insert(i, HashMap::new());
                                            } else if locale == "keypad"
                                                && i >= self.level_keypadmap.len()
                                            {
                                                self.level_keypadmap.insert(i, HashMap::new());
                                            }
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
                                                if locale == "keypad" {
                                                    self.level_keypadmap[i]
                                                        .insert(*evdev_code, single_char);
                                                } else {
                                                    self.level_keymap[i]
                                                        .insert(*evdev_code, single_char);
                                                }
                                            } else {
                                                // println!("{:?}", v);
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
