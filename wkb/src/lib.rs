use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    os::fd::OwnedFd,
    path::Path,
};

use default_keymap::{
    DEFAULT_LEVEL1_KEYPADMAP, DEFAULT_LEVEL1_MAP, DEFAULT_LEVEL2_KEYPADMAP, DEFAULT_LEVEL2_MAP,
    DEFAULT_LEVEL3_KEYPADMAP, DEFAULT_LEVEL3_MAP, DEFAULT_LEVEL4_KEYPADMAP, DEFAULT_LEVEL4_MAP,
};
use evdev_xkb::XKBCODES_EVDEV;
use regex::Regex;
use repeat::REPEAT_KEYS;
use repeat_default::REPEAT_DEFAULT;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};
use xkb_utf8::XKBCODES_DEF_TO_UTF8;
mod default_keymap;
mod evdev_xkb;
mod repeat;
mod repeat_default;
mod xkb_utf8;

#[derive(Debug, Clone, Copy, Default)]
struct Modifier {
    pressed: bool,
    // level: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct LockedModifier {
    pressed: bool,
    locked: u8,
    // level: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct Modifiers {
    ctrl: Modifier,
    alt: Modifier,
    alt_gr: Modifier,
    shift: Modifier,
    logo: Modifier,
    caps_lock: LockedModifier,
    num_lock: LockedModifier,
    scroll_lock: LockedModifier,
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

type LevelKeymap = Vec<HashMap<u32, char>>;
type LevelKeypadmap = Vec<HashMap<u32, char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModifierType {
    Press,
    Lock,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Mod {
    name: String,
    level: usize,
    mod_type: ModifierType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Levels {
    level2: usize,
    level3: usize,
    level5: usize,
}

#[derive(Debug, Clone)]
pub struct WKB {
    layouts: Vec<String>,
    layout: String,
    level_keymap: LevelKeymap,
    level_keypadmap: LevelKeypadmap,
    level_modifiers: HashMap<u32, Mod>,
    compose_status: ComposeStatus,
    compose_char: char,
    pressed_levels: Levels,
    locked_levels: Levels,
    pressed_keys: HashSet<u32>,
    repeat_keys: HashSet<u32>,
    modifiers: Modifiers,
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

impl WKB {
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let level_keypadmap = vec![
            DEFAULT_LEVEL1_KEYPADMAP.clone(),
            DEFAULT_LEVEL2_KEYPADMAP.clone(),
            DEFAULT_LEVEL3_KEYPADMAP.clone(),
            DEFAULT_LEVEL4_KEYPADMAP.clone(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];
        let level_keymap = vec![
            DEFAULT_LEVEL1_MAP.clone(),
            DEFAULT_LEVEL2_MAP.clone(),
            DEFAULT_LEVEL3_MAP.clone(),
            DEFAULT_LEVEL4_MAP.clone(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];
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

        let mut wkb = Self {
            layouts,
            layout,
            level_modifiers: HashMap::from([
                (
                    42,
                    Mod {
                        name: "LeftShift".to_string(),
                        level: 2,
                        mod_type: ModifierType::Press,
                    },
                ),
                (
                    54,
                    Mod {
                        name: "RightShift".to_string(),
                        level: 2,
                        mod_type: ModifierType::Press,
                    },
                ),
            ]),
            compose_status: ComposeStatus::Idle,
            level_keymap,
            level_keypadmap,
            compose_char: char::default(),
            pressed_levels: Levels::default(),
            locked_levels: Levels::default(),
            pressed_keys: HashSet::new(),
            repeat_keys,
            modifiers: Modifiers::default(),
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

    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.get(&evdev_code).is_some()
    }

    pub fn utf8(&self, evdev_code: u32) -> Option<char> {
        let level5 = self.pressed_levels.level5 != 0 || self.locked_levels.level5 != 0;
        let level3 = self.pressed_levels.level3 != 0 || self.locked_levels.level3 != 0;
        let level2 = self.pressed_levels.level2 != 0 || self.locked_levels.level2 != 0;
        if level5 && level3 && level2 {
            self.level_key(evdev_code, 7)
        } else if level5 && level3 {
            self.level_key(evdev_code, 6)
        } else if level5 && level2 {
            self.level_key(evdev_code, 5)
        } else if level5 {
            self.level_key(evdev_code, 4)
        } else if level3 && level2 {
            self.level_key(evdev_code, 3)
        } else if level3 {
            self.level_key(evdev_code, 2)
        } else if level2 {
            self.level_key(evdev_code, 1)
        } else {
            self.level_key(evdev_code, 0)
        }
    }

    pub fn update_key(&mut self, evdev_code: u32, direction: KeyDirection) {
        match (evdev_code, direction) {
            (29 | 97, KeyDirection::Down) => self.modifiers.ctrl.pressed = true,
            (29 | 97, KeyDirection::Up) => self.modifiers.ctrl.pressed = false,
            (42 | 54, KeyDirection::Down) => self.modifiers.shift.pressed = true,
            (42 | 54, KeyDirection::Up) => self.modifiers.shift.pressed = false,
            (56, KeyDirection::Down) => self.modifiers.alt.pressed = true,
            (56, KeyDirection::Up) => self.modifiers.alt.pressed = false,
            (100, KeyDirection::Down) => self.modifiers.alt_gr.pressed = true,
            (100, KeyDirection::Up) => self.modifiers.alt_gr.pressed = false,
            (125, KeyDirection::Down) => self.modifiers.logo.pressed = true,
            (125, KeyDirection::Up) => self.modifiers.logo.pressed = false,
            (58, KeyDirection::Down) => {
                self.modifiers.caps_lock.pressed = true;
                if self.modifiers.caps_lock.locked == 0 {
                    self.modifiers.caps_lock.locked = 2;
                }
            }
            (58, KeyDirection::Up) => {
                self.modifiers.caps_lock.pressed = false;
                if self.modifiers.caps_lock.locked > 0 {
                    self.modifiers.caps_lock.locked -= 1;
                }
            }
            (69, KeyDirection::Down) => {
                self.modifiers.num_lock.pressed = true;
                if self.modifiers.num_lock.locked == 0 {
                    self.modifiers.num_lock.locked = 2;
                }
            }
            (69, KeyDirection::Up) => {
                self.modifiers.num_lock.pressed = false;
                if self.modifiers.num_lock.locked > 0 {
                    self.modifiers.num_lock.locked -= 1;
                }
            }
            (70, KeyDirection::Down) => {
                self.modifiers.scroll_lock.pressed = true;
                if self.modifiers.scroll_lock.locked == 0 {
                    self.modifiers.scroll_lock.locked = 2;
                }
            }
            (70, KeyDirection::Up) => {
                self.modifiers.scroll_lock.pressed = false;
                if self.modifiers.scroll_lock.locked > 0 {
                    self.modifiers.scroll_lock.locked -= 1;
                }
            }
            (_, _) => (),
        };
        let modifier = self.level_modifiers.get(&evdev_code);
        if let Some(modifier) = modifier {
            match (direction, modifier.mod_type, modifier.level) {
                (KeyDirection::Down, ModifierType::Press, 2) => self.pressed_levels.level2 += 1,
                (KeyDirection::Up, ModifierType::Press, 2) => self.pressed_levels.level2 -= 1,
                (KeyDirection::Down, ModifierType::Press, 3) => self.pressed_levels.level3 += 1,
                (KeyDirection::Up, ModifierType::Press, 3) => self.pressed_levels.level3 -= 1,
                (KeyDirection::Down, ModifierType::Press, 5) => self.pressed_levels.level5 += 1,
                (KeyDirection::Up, ModifierType::Press, 5) => self.pressed_levels.level5 -= 1,
                (KeyDirection::Down, ModifierType::Lock, 5) => {
                    if self.locked_levels.level5 == 0 {
                        self.locked_levels.level5 += 2;
                    }
                }
                (KeyDirection::Up, ModifierType::Lock, 5) => {
                    if self.locked_levels.level5 > 0 {
                        self.locked_levels.level5 -= 1;
                    }
                }
                (_, _, _) => unreachable!(),
            };
        } else {
            match direction {
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

    pub fn level_keypadmap(&self) -> LevelKeypadmap {
        self.level_keypadmap.clone()
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
                                                if id.content.starts_with("KP") {
                                                    self.level_keypadmap[i]
                                                        .insert(evdev_code, single_char);
                                                } else {
                                                    self.level_keymap[i]
                                                        .insert(evdev_code, single_char);
                                                }
                                            } else {
                                                match layout.as_str() {
                                                    "ralt_switch" => {
                                                        self.level_modifiers.insert(
                                                            100,
                                                            Mod {
                                                                name: "AltGr".to_string(),
                                                                level: 3,
                                                                mod_type: ModifierType::Press,
                                                            },
                                                        );
                                                    }
                                                    "lalt_switch" => {
                                                        self.level_modifiers.insert(
                                                            56,
                                                            Mod {
                                                                name: "Alt".to_string(),
                                                                level: 3,
                                                                mod_type: ModifierType::Press,
                                                            },
                                                        );
                                                    }
                                                    "enter_switch" => {
                                                        self.level_modifiers.insert(
                                                            96,
                                                            Mod {
                                                                name: "KeyPadEnter".to_string(),
                                                                level: 3,
                                                                mod_type: ModifierType::Press,
                                                            },
                                                        );
                                                    }
                                                    "bksl_switch" => {
                                                        self.level_modifiers.insert(
                                                            43,
                                                            Mod {
                                                                name: "Backslash".to_string(),
                                                                level: 3,
                                                                mod_type: ModifierType::Press,
                                                            },
                                                        );
                                                    }
                                                    "rctrl_switch" | "switch" => {
                                                        self.level_modifiers.insert(
                                                            97,
                                                            Mod {
                                                                name: "RightCtrl".to_string(),
                                                                level: if locale == "level3" {
                                                                    3
                                                                } else {
                                                                    5
                                                                },
                                                                mod_type: ModifierType::Press,
                                                            },
                                                        );
                                                    }
                                                    "caps_switch" => {
                                                        self.level_modifiers.insert(
                                                            58,
                                                            Mod {
                                                                name: "CapsLock".to_string(),
                                                                level: if locale == "level3" {
                                                                    3
                                                                } else {
                                                                    5
                                                                },
                                                                mod_type: ModifierType::Press,
                                                            },
                                                        );
                                                    }
                                                    "lock" => {
                                                        self.level_modifiers.insert(
                                                            199,
                                                            Mod {
                                                                name: "Hypr".to_string(),
                                                                level: 5,
                                                                mod_type: ModifierType::Lock,
                                                            },
                                                        );
                                                    }
                                                    "ralt_switch_lock" => {
                                                        self.level_modifiers.insert(
                                                            100,
                                                            Mod {
                                                                name: "AltGr".to_string(),
                                                                level: 5,
                                                                mod_type: ModifierType::Lock,
                                                            },
                                                        );
                                                    }
                                                    "lsgt_switch_lock" => {
                                                        self.level_modifiers.insert(
                                                            86,
                                                            Mod {
                                                                name: "LessGreater".to_string(),
                                                                level: 5,
                                                                mod_type: ModifierType::Lock,
                                                            },
                                                        );
                                                    }
                                                    _ => {
                                                        if !v.contains("VoidSymbol") {
                                                            // println!("{:?}", v);
                                                            // println!("{}", *evdev_code);
                                                            // println!("{}", layout);
                                                        }
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
