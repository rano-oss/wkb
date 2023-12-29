use std::{collections::HashMap, path::Path, fs::File, os::fd::OwnedFd, io::Read};

use evdev_xkb::XKBCODES_EVDEV;
use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};
use xkb_utf8::XKBCODES_DEF_TO_UTF8;
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct WKB {
    layouts: Vec<String>,
    layout: String,
    keymap: Keymap,
    level_keymap: LevelKeymap,
    modifiers: Modifiers,
    compose_status: ComposeStatus,
    compose_char: char,
}

fn parse_include(input: &str) -> (String, Option<String>) {
    let re = Regex::new(r"^([\w]+)(?:\(([\w]+)\))?$").unwrap();
    let capture = re.captures(input).unwrap();
    (
        capture.get(1).map(|m| m.as_str().to_string()).unwrap(),
        capture.get(2).map(|m| m.as_str().to_string()),
    )
}

impl WKB {    
    pub fn new_from_fd(fd: OwnedFd) -> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let mut wkb = Self {
            layouts: Vec::new(),
            layout: String::new(),
            keymap: HashMap::new(),
            modifiers: Modifiers::default(),
            compose_status: ComposeStatus::Idle,
            level_keymap: Vec::new(),
            compose_char: char::default()
        };
        wkb.read_layouts(path, None, Some(fd));
        wkb
    }
    
    pub fn new_from_names(locale: String, layout: Option<String>)-> Self {
        let path = Path::new("/usr/share/X11/xkb/symbols/");
        let mut wkb = Self {
            layouts: Vec::new(),
            layout: String::new(),
            keymap: HashMap::new(),
            modifiers: Modifiers::default(),
            compose_status: ComposeStatus::Idle,
            level_keymap: Vec::new(),
            compose_char: char::default()            
        };
        wkb.read_layouts(path, Some(locale.clone()), None);
        wkb.layout = if let Some(layout) = layout { layout } else { wkb.layouts()[0].clone()};
        wkb.map(path, locale,  None);
        wkb
    }
    
    pub fn level_key(&self, evdev_code: u32, level_index: usize)->Option<char>{
        self.level_keymap.get(level_index).and_then(
            |hm| hm.get(&evdev_code).copied()
        )
    }
    
    pub fn key(&self, evdev_code: u32) -> Option<char> {
        match self.modifiers {
            Modifiers {
                shift: false,
                caps_lock: false,
                alt_gr: false,
                alt:false,
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
                    ComposeStatus::Idle |
                    ComposeStatus::Composed |
                    ComposeStatus::Cancelled => {
                        self.compose_char = character;
                        self.compose_status = ComposeStatus::Composing;
                        FeedResult::Accepted
                    },
                    ComposeStatus::Composing => {
                        if let Some(character) = unicode_normalization::char::compose(character, self.compose_char){
                            self.compose_char = character;
                            FeedResult::Accepted
                        } else {
                            self.compose_status = ComposeStatus::Cancelled;
                            FeedResult::Ignored
                        }
                    },
                }
            } else if self.compose_status == ComposeStatus::Composing {
                if let Some(character) = unicode_normalization::char::compose(character, self.compose_char){
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
    
    pub fn layouts(&self)->Vec<String>{
        self.layouts.clone()
    }
    
    pub fn current_layout(&self) -> String {
        self.layout.clone()
    }
    
    pub fn level_keymap(&self) -> LevelKeymap {
        self.level_keymap.clone()
    }
    
    pub fn keymap(&self) -> Keymap {
        self.keymap.clone()
    }
    
    fn read_layouts(&mut self, path: &Path, locale: Option<String>, fd: Option<OwnedFd>) {
        let mut string_file = String::new();
        if let Some(fd) = fd {
            let mut file = File::from(fd);
            file.read_to_string(&mut string_file).expect("Could not read file");
        } else if let Some(locale) = locale {
            string_file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
        }
        let xkb = parse(&string_file).expect("neither names nor file set");
        xkb.definitions.iter().for_each(|d| {
            match &d.directive {
                Directive::XkbSymbols(src) => {
                    self.layouts.push(src.name.content.to_string());
                }
                _ => {}
            }
        })
    }

    fn map(&mut self, path: &Path, locale: String, layout: Option<String>) {
        let file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
        let xkb = parse(&file).unwrap();
        xkb.definitions.iter().for_each(|d| {
            match &d.directive {
                Directive::XkbSymbols(src) => {
                    let layout = layout.clone().unwrap_or(self.current_layout());
                    if src.name.content == layout {
                        src.value.iter().for_each(|si| {
                            match si {
                                XkbSymbolsItem::Include(Include { name }) => {
                                    let (locale, layout) = parse_include(name);
                                    self.map(path, locale, layout);
                                }
                                XkbSymbolsItem::Key(Key { mode:_, id, values }) => {
                                    // Why not just unwrap here? Surely there shouldn't be any symbols not defined in evdev...
                                    // ph has AB00 which does not exists in evdev...
                                    if let Some(evdev_code) = XKBCODES_EVDEV.get(id.content){
                                        values.iter().for_each(|v|{
                                            match v {
                                                xkb_parser::ast::KeyValue::KeyNames(key) => {
                                                    for (i,v) in key.values.iter().enumerate(){
                                                        let single_char = XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned();
                                                        if let Some(single_char) = single_char {
                                                            if i > self.level_keymap.len() {
                                                                for j in self.level_keymap.len()..i {
                                                                    self.level_keymap.insert(j, HashMap::new());
                                                                }
                                                            }
                                                            let level = if let Some(level)  = self.level_keymap.get_mut(i){
                                                                level
                                                            } else {
                                                                self.level_keymap.insert(i, HashMap::new());
                                                                &mut self.level_keymap[i]
                                                            };
                                                            level.insert(*evdev_code, single_char);
                                                        } else {
                                                            if v.content != "none" && v.content != "VoidSymbol" && v.content != "any" && v.content != "ISO_Level3_Shift" && v.content != "NoSymbol" && v.content != "KP_Separator" {
                                                                println!("{:?}", v);
                                                                // println!("{:?}", locale);
                                                            }
                                                        }                                                        
                                                    }
                                                },
                                                _ => {}
                                            }
                                        });
                                    }
                                }
                                _ => {}
                            }
                        });
                    }
                }
                _ => {}
            }
        });
    }
}
