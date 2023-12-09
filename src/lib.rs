use std::{collections::HashMap, path::Path};

use evdev_xkb::XKBCODES_EVDEV;
use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};
use xkb_utf8::XKBCODES_DEF_TO_UTF8;
pub mod evdev_xkb;
mod xkb_utf8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CharModifier {
    None,
    Shift,
    CapsLock,
    AltGr,
    AltGrAndShift,
}

#[derive(Debug, Clone, Default)]
struct Modifiers {
    shift: bool,
    caps_lock: bool,
    alt_gr: bool,
}

#[derive(Debug, Clone)]
pub struct Keymap {
    map: HashMap<CharModifier, HashMap<u32, char>>, // length of CharModifiers
    modifiers: Modifiers
}

fn parse_include(input: &str) -> (String, Option<String>) {
    let re = Regex::new(r"^([\w]+)(?:\(([\w]+)\))?$").unwrap();
    let capture = re.captures(input).unwrap();
    (
        capture.get(1).map(|m| m.as_str().to_string()).unwrap(),
        capture.get(2).map(|m| m.as_str().to_string()),
    )
}

impl Keymap {
    pub fn new()-> Self {
        let mut map = HashMap::from([
            (CharModifier::None, HashMap::new()),
            (CharModifier::Shift, HashMap::new()),
            (CharModifier::CapsLock, HashMap::new()),
            (CharModifier::AltGr, HashMap::new()),
            (CharModifier::AltGrAndShift, HashMap::new())
        ]);
        map.shrink_to_fit();
        Self{
            map,
            modifiers: Modifiers::default()
        }
    }

    pub fn map(&mut self, path: &Path, locale: String, layout: Option<String>) {
        let file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
        let xkb = parse(&file).unwrap();
        xkb.definitions.iter().for_each(|d| {
            match &d.directive {
                Directive::XkbSymbols(src) => {
                    let layout = if let Some(layout) = &layout {
                        layout
                    } else {
                        "basic"
                    };
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
                                                            match i {
                                                                0 => {
                                                                    let keymap = self.map.get_mut(&CharModifier::None).unwrap();
                                                                    keymap.insert(*evdev_code, single_char);
                                                                    if !single_char.is_alphabetic() {
                                                                        let keymap = self.map.get_mut(&CharModifier::CapsLock).unwrap();
                                                                        keymap.insert(*evdev_code, single_char);
                                                                    }
                                                                }
                                                                1 => {
                                                                    let keymap = self.map.get_mut(&CharModifier::Shift).unwrap();
                                                                    keymap.insert(*evdev_code, single_char);
                                                                    if single_char.is_alphabetic() {
                                                                        let keymap = self.map.get_mut(&CharModifier::CapsLock).unwrap();
                                                                        keymap.insert(*evdev_code, single_char);
                                                                    }
                                                                }
                                                                2 => {
                                                                    let keymap = self.map.get_mut(&CharModifier::AltGr).unwrap();
                                                                    keymap.insert(*evdev_code, single_char);
                                                                }
                                                                3 => {                                                            
                                                                    let keymap = self.map.get_mut(&CharModifier::AltGrAndShift).unwrap();
                                                                    keymap.insert(*evdev_code, single_char);
                                                                }
                                                                _ => {}
                                                            }
                                                        } else {
                                                            if v.content != "none" && v.content != "VoidSymbol" && v.content != "any" && v.content != "ISO_Level3_Shift" && v.content != "KP_Delete" && v.content != "KP_Separator" {
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

    pub fn key(&self, evdev_code: u32) -> Option<char> {
        match self.modifiers {
            Modifiers { shift:false, caps_lock:false, alt_gr:false } => {
                let mods_map = self.map.get(&CharModifier::None).unwrap();
                mods_map.get(&evdev_code).copied()
            }
            _ => {None}
        }
    }
}
