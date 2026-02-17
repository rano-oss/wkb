use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::{self, File},
    io::{self, BufRead, Read},
    os::fd::OwnedFd,
    path::Path,
};

use crate::composer::{Composer, ListComposer};
use crate::modifiers::Modifiers;
use crate::repeat::REPEAT_DEFAULT;
use crate::REPEAT_KEYS;
use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};

use crate::default_keymap::DEFAULT_MAP;
use crate::modifiers::{ModKind, ModType, BACKSPACE, CAPS_LOCK, TAB};
use crate::WKB;

pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

pub mod evdev_xkb;
pub mod xkb_utf8;

use evdev_xkb::XKBCODES_EVDEV;
use xkb_utf8::XKBCODES_DEF_TO_UTF8;

fn load_compose_table(locale: &str) -> Option<(ListComposer, ListComposer)> {
    let compose_dir_path = Path::new("/usr/share/X11/locale/compose.dir");
    if !compose_dir_path.exists() {
        return None;
    }

    let file = fs::File::open(compose_dir_path).ok()?;
    let reader = io::BufReader::new(file);

    let mut compose_file_path = None;
    for line in reader.lines() {
        let line = line.ok()?;
        if line.trim().starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == locale {
            compose_file_path = Some(parts[0].to_string());
            break;
        }
    }

    let compose_file_path = compose_file_path?;
    let full_path = Path::new("/usr/share/X11/locale").join(compose_file_path);
    let file = fs::File::open(full_path).ok()?;
    let reader = io::BufReader::new(file);

    let mut regular = ListComposer::new();
    let mut compose_key = ListComposer::new();

    for line in reader.lines() {
        let line = line.ok()?;
        if line.trim().starts_with('#') {
            continue;
        }
        if let Some((keys_part, value_part)) = line.split_once(':') {
            let keys_str = keys_part.trim();
            let value_str = value_part.trim();

            let mut keys: Vec<char> = Vec::new();
            let mut is_multi_key = false;

            for key in keys_str.split_whitespace() {
                if key == "<Multi_key>" {
                    if keys.is_empty() {
                        is_multi_key = true;
                    }
                    continue;
                }
                let key_name = key.trim_start_matches('<').trim_end_matches('>');
                if let Some(&c) = XKBCODES_DEF_TO_UTF8.get(key_name) {
                    keys.push(c);
                } else if key_name.len() == 1 {
                    keys.push(key_name.chars().next().unwrap());
                }
            }

            if keys.is_empty() {
                continue;
            }

            // Parse value
            let out_char = if value_str.starts_with('"') && value_str.ends_with('"') {
                let inner = &value_str[1..value_str.len() - 1];
                // Handle basic escapes if needed, for now take chars
                // If it's a string, ListComposer currently takes char.
                // We'll take the first char if possible, or handle sequences if supported later.
                // For most compose rules, it results in a single character.
                inner.chars().next()
            } else {
                let val_name = value_str.trim();
                XKBCODES_DEF_TO_UTF8.get(val_name).copied()
            };

            if let Some(out) = out_char {
                if is_multi_key {
                    compose_key.insert(&keys, out);
                } else {
                    regular.insert(&keys, out);
                }
            }
        }
    }

    Some((regular, compose_key))
}

fn read_layouts(path: &Path, locale: Option<String>, fd: Option<OwnedFd>) -> Vec<String> {
    let mut string_file = String::new();
    if let Some(fd) = fd {
        let mut file = File::from(fd);
        let _ = file.read_to_string(&mut string_file);
    } else if let Some(locale) = locale {
        if let Ok(content) = std::fs::read_to_string(&path.join(locale)) {
            string_file = content;
        }
    }
    let mut layouts = Vec::new();
    if let Ok(xkb) = parse(&string_file) {
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
    } else {
        let re = Regex::new(r#"xkb_symbols\s+"([\w\s_\-\d]+)"\s+\{"#).unwrap();
        for cap in re.captures_iter(&string_file) {
            if let Some(name) = cap.get(1) {
                let name = name.as_str();
                if ![
                    "sun_type6",
                    "sun_type6_de",
                    "sun_type6_fr",
                    "sun_type6_suncompat",
                    "sun_type7_suncompat",
                    "suncompat",
                    "sun_type7",
                ]
                .contains(&name)
                {
                    layouts.push(name.to_string());
                }
            }
        }
    }
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

pub fn new_from_names(locale: String, layout: Option<String>) -> WKB<ListComposer> {
    let path = Path::new(XKB_SYMBOLS_PATH);
    let layouts = read_layouts(path, Some(locale.clone()), None);
    let layout = if let Some(layout) = layout {
        layout
    } else {
        layouts[0].clone()
    };
    let (regular_composer, compose_key_composer) =
        load_compose_table(&locale).unwrap_or_else(|| (ListComposer::new(), ListComposer::new()));

    let mut wkb = new_wkb(
        layouts,
        layout.clone(),
        Some(locale.clone()),
        regular_composer,
        compose_key_composer,
    );
    map_xkb(&mut wkb, path, locale.clone(), Some(layout.clone()));
    fix_xkb_edge_cases(&mut wkb, locale, Some(layout));
    wkb
}

pub fn new_from_string(string: String) -> WKB<ListComposer> {
    let mut wkb = new_wkb(
        Vec::new(),
        "".to_string(),
        None,
        ListComposer::new(),
        ListComposer::new(),
    );
    map_xkb_from_str(&mut wkb, &string, Path::new(XKB_SYMBOLS_PATH), None, None);
    wkb
}

fn new_wkb(
    layouts: Vec<String>,
    layout: String,
    locale: Option<String>,
    regular_composer: ListComposer,
    compose_key_composer: ListComposer,
) -> WKB<ListComposer> {
    let repeat_keys = if let Some(locale) = &locale {
        if let Some(locale_map) = REPEAT_KEYS.get(locale.as_str()) {
            if let Some(layout_set) = locale_map.get(layout.as_str()) {
                layout_set.clone()
            } else {
                REPEAT_DEFAULT.clone()
            }
        } else {
            REPEAT_DEFAULT.clone()
        }
    } else {
        REPEAT_DEFAULT.clone()
    };
    WKB {
        layouts,
        layout,
        locale,
        regular_composer,
        compose_key_composer,
        level_keymap: Vec::with_capacity(8),
        pressed_keys: HashSet::new(),
        repeat_keys,
        custom_case_map: None,
        caps_is_level2: None,
        modifiers: Modifiers::default(),
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
    }
}

fn map_xkb<C: Composer>(wkb: &mut WKB<C>, path: &Path, locale: String, layout: Option<String>) {
    let Ok(file) = std::fs::read_to_string(&path.join(locale.clone())) else {
        return;
    };
    map_xkb_from_str(wkb, &file, path, Some(locale), layout);
}

fn map_xkb_from_str<C: Composer>(
    wkb: &mut WKB<C>,
    file: &str,
    path: &Path,
    locale: Option<String>,
    layout: Option<String>,
) {
    let file = file.replace("symbols=[", "[");
    let Ok(xkb) = parse(&file) else {
        return;
    };
    xkb.definitions.iter().for_each(|d| {
        if let Directive::XkbSymbols(src) = &d.directive {
            let layout = layout.clone().unwrap_or(wkb.current_layout());
            if src.name.content == layout {
                src.value.iter().for_each(|si| {
                    if let XkbSymbolsItem::Include(Include { name }) = si {
                        let (locale, layout) = parse_include(name);
                        if layout.is_none() {
                            map_xkb(wkb, path, locale, Some("basic".to_string()));
                        } else {
                            map_xkb(wkb, path, locale, layout);
                        }
                    } else if let XkbSymbolsItem::Key(Key {
                        mode: _,
                        id,
                        values,
                    }) = si
                    {
                        if let Some(evdev_code) = XKBCODES_EVDEV.get(id.content) {
                            values.iter().for_each(|v| {
                                if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
                                    if let xkb_parser::ast::KeyDef::SymbolDef(key) = key_defs {
                                        for (i, v) in key.values.values.iter().enumerate() {
                                            if i == wkb.level_keymap.len() {
                                                if i < DEFAULT_MAP.len() {
                                                    wkb.level_keymap.push(DEFAULT_MAP[i].clone());
                                                } else {
                                                    wkb.level_keymap.push(BTreeMap::new());
                                                }
                                            }
                                            let single_char =
                                                XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned();
                                            if let Some(char) = single_char {
                                                wkb.level_keymap[i].insert(*evdev_code, char);
                                            }
                                        }
                                    }
                                } else if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                    map_keys_and_modifiers(
                                        wkb,
                                        key,
                                        evdev_code,
                                        layout.clone(),
                                        locale.clone().unwrap_or_default(),
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

fn map_keys_and_modifiers<C: Composer>(
    wkb: &mut WKB<C>,
    key: &xkb_parser::ast::KeyNames,
    evdev_code: &u32,
    layout: String,
    locale: String,
    id: String,
) {
    if id == "CAPS" {
        if key.values.first().is_some_and(|k| k.content == "BackSpace") {
            wkb.remap.insert(*evdev_code, BACKSPACE);
            let value = *wkb.level_keymap[0].get(&BACKSPACE).unwrap();
            wkb.level_keymap[1].insert(CAPS_LOCK, value);
        } else if key.values.first().is_some_and(|k| k.content == "Tab") {
            wkb.remap.insert(*evdev_code, TAB);
        }
    }
    for (i, v) in key.values.iter().enumerate() {
        if i == wkb.level_keymap.len() {
            if i < DEFAULT_MAP.len() {
                wkb.level_keymap.push(DEFAULT_MAP[i].clone());
            } else {
                wkb.level_keymap.push(BTreeMap::new());
            }
        }
        if let Some(remap_key_code) = XKBCODES_EVDEV.get(v.content) {
            wkb.remap.insert(*evdev_code, *remap_key_code);
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
        } else if first_char.is_some_and(|c| c == '0') && second_char.is_some_and(|c| c == 'x') {
            hex_string_to_unicode_char(v)
        } else {
            XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned()
        };
        if let Some(single_char) = single_char {
            wkb.level_keymap[i].insert(*evdev_code, single_char);
        } else {
            match (v.content, layout.as_str()) {
                ("Eisu_toggle", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::None,
                        },
                        1,
                    );
                }
                ("Control_L", _) => {
                    if id == "CAPS" {
                        wkb.remap.insert(*evdev_code, 29);
                    }
                }
                ("Shift_L", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level2,
                        },
                        i as u8,
                    );
                }
                ("Shift_R", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level2,
                        },
                        i as u8,
                    );
                }
                ("Shift_Lock", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level2,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level3_Shift", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level3_Lock", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level3_Latch", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Latch {
                            pressed: false,
                            latched: false,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level5_Shift", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Level5,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level5_Lock", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level5,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level5_Latch", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Latch {
                            pressed: false,
                            latched: false,
                            mod_type: ModType::Level5,
                        },
                        i as u8,
                    );
                }
                ("Multi_key", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Compose,
                        },
                        i as u8,
                    );
                }
                (_, "rshift_both_shiftlock") | (_, "lshift_both_shiftlock") => {
                    wkb.right_left_shift_caps = true;
                }
                (_, "bksl_switch") => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                (_, "caps_switch") => {
                    if locale == "level3" {
                        wkb.modifiers.insert(
                            *evdev_code,
                            ModKind::Pressed {
                                pressed: false,
                                mod_type: ModType::Level3,
                            },
                            i as u8,
                        );
                    } else {
                        wkb.modifiers.insert(
                            *evdev_code,
                            ModKind::Pressed {
                                pressed: false,
                                mod_type: ModType::Level5,
                            },
                            i as u8,
                        );
                    };
                }
                _ => {
                    if v.contains("none") {
                        if i > 0 {
                            if let Some(key) = wkb.level_keymap[i - 1].clone().get(evdev_code) {
                                wkb.level_keymap[i].insert(*evdev_code, *key);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn fix_xkb_edge_cases<C: Composer>(wkb: &mut WKB<C>, locale: String, layout: Option<String>) {
    //snowflake special case of bad design I don't want to support any other way!
    match (locale.as_str(), &layout.as_deref()) {
        ("de", Some("ru")) | ("de", Some("ru-recom")) | ("de", Some("ru-translit")) => {
            for i in 0..2 {
                let map = wkb.level_keymap[i].clone();
                let t21 = map.get(&21);
                let t44 = map.get(&44);
                wkb.level_keymap[i].insert(21, *t44.unwrap());
                wkb.level_keymap[i].insert(44, *t21.unwrap());
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
        | ("it", Some("lldde"))
        | ("cz", Some("ucw"))
        | ("de", Some("nodeadkeys")) => {
            wkb.custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
        }
        ("de", Some("ru-translit")) | ("de", Some("ru-recom")) => {
            wkb.custom_case_map = Some(HashMap::from([('~', 'ẞ')]));
        }
        ("tr", Some("basic"))
        | ("tr", Some("e"))
        | ("tr", Some("intl"))
        | ("tr", Some("olpc"))
        | ("ua", Some("crh"))
        | ("ua", Some("crh_f"))
        | ("ro", Some("crh_dobruja"))
        | ("tr", Some("f")) => {
            wkb.custom_case_map = Some(HashMap::from([('i', 'İ'), ('İ', 'i'), ('I', 'ı')]));
        }
        ("az", Some("latin")) => wkb.caps_is_level2 = Some(vec![23, 39]),
        ("gh", Some("hausa")) => wkb.caps_is_level2 = Some(vec![39]),
        ("ng", Some("hausa")) | ("gh", Some("fula")) | ("tr", Some("us")) => {
            wkb.custom_case_map = Some(HashMap::from([('ı', 'İ'), ('İ', 'ı')]));
        }
        ("md", Some("gag")) => {
            wkb.custom_case_map = Some(HashMap::from([
                ('ı', 'I'),
                ('I', 'ı'),
                ('i', 'İ'),
                ('İ', 'i'),
            ]));
        }
        ("bqn", Some("bqn")) => {
            wkb.custom_case_map = Some(HashMap::from([
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
            wkb.level_keymap[1].insert(22, '⊔');
            wkb.level_keymap[1].insert(32, '↕');
            wkb.level_keymap[1].insert(36, '∘');
            wkb.level_keymap[1].insert(46, '↓');
            wkb.level_keymap[1].insert(57, '‿');
        }
        ("hu", Some("oldhunlig")) => {
            wkb.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41, 51, 52, 53]);
        }
        ("hu", Some("oldhun_base")) => {
            wkb.caps_is_level2 = Some(vec![51, 52, 53]);
        }
        ("hu", Some("oldhun_origin")) | ("hu", Some("oldhun_lig")) => {
            wkb.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41]);
        }
        ("kz", Some("ext")) => wkb.caps_is_level2 = Some(vec![2, 7, 8, 43]),
        ("fr", Some("bepo")) => {
            wkb.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        }
        ("fr", Some("bepo_afnor")) => {
            wkb.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            wkb.level_keymap.resize(4, BTreeMap::new());
        }
        ("fr", Some("dvorak")) => {
            wkb.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 17, 18, 41]);
            wkb.custom_case_map = Some(HashMap::from([('à', '/'), (';', '-'), ('ç', 'ç')]))
        }
        ("kz", Some("latin")) => {
            wkb.custom_case_map = Some(HashMap::from([('v', 'M')]));
        }
        ("us", Some("chr")) => {
            wkb.caps_is_level2 = Some(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 49,
                50, 51, 52, 53,
            ])
        }
        ("us", Some("mac")) => wkb.caps_lock_level2_disabled = true,
        ("il", Some("si2")) | ("il", Some("basic")) => {
            wkb.caps_is_level2 = Some(vec![
                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 44,
                45, 46, 47, 48, 49, 50, 51, 52,
            ])
        }
        ("il", Some("phonetic")) => {
            wkb.caps_is_level2 = Some(vec![20, 25, 33, 37, 39, 46, 49, 50, 51, 52])
        }
        ("il", Some("biblical")) => {
            wkb.caps_is_level2 = Some(vec![
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 49, 50,
                51, 52, 53,
            ])
        }
        ("il", Some("biblicalSIL")) => {
            wkb.caps_is_level2 = Some(vec![
                2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 16, 18, 21, 24, 25, 26, 27, 30, 31, 33, 36, 37,
                39, 40, 41, 46, 49, 50, 51, 52, 53,
            ])
        }
        ("it", Some("geo")) => {
            wkb.caps_is_level2 = Some(vec![
                16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
            ])
        }
        ("pl", Some("dvp")) | ("us", Some("dvp")) => {
            wkb.caps_is_level2 = Some(vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 40])
        }
        ("in", Some("tamilnet_TAB")) | ("lk", Some("tam_TAB")) => {
            wkb.caps_lock_disabled = true;
            wkb.caps_lock_level2_disabled = true;
        }
        ("in", Some("tamilnet_TSCII")) => {
            wkb.custom_case_map = Some(HashMap::from([('þ', 'þ')]));
            wkb.caps_lock_level2_disabled = true;
        }
        ("in", Some("iipa")) => wkb.custom_case_map = Some(HashMap::from([('b', 'Y')])),
        ("ge", Some("qwerty")) | ("ge", Some("basic")) => {
            wkb.caps_is_level2 = Some(vec![
                16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
            ])
        }
        ("ge", Some("ergonomic")) => {
            wkb.caps_is_level2 = Some(vec![
                16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
            ])
        }
        ("ge", Some("mess")) => {
            wkb.caps_is_level2 = Some(vec![
                16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 44, 45,
                46, 47, 48, 49, 50,
            ])
        }
        ("fr", Some("geo")) => {
            wkb.caps_is_level2 = Some(vec![
                17, 18, 20, 22, 24, 25, 26, 27, 30, 31, 32, 37, 38, 39, 40, 44, 48, 51,
            ]);
        }
        ("gr", Some(_)) => {
            wkb.caps_is_level2 = Some(vec![17]);
        }
        ("pl", Some("glagolica")) => {
            wkb.caps_lock_level2_disabled = true;
            let value = *wkb.level_keymap[0].get(&40).unwrap();
            wkb.level_keymap[1].insert(40, value);
        }
        ("cd", _) | ("ml", Some("us-mac")) => wkb.caps_lock_level2_disabled = true,
        ("apl", Some("apl2")) | ("apl", Some("aplplusII")) => {
            for i in 16..=25 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[1].insert(i, value);
            }
            for i in 30..=38 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[1].insert(i, value);
            }
            for i in 44..=50 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[1].insert(i, value);
            }
        }
        ("ie", Some("ogam")) => {
            wkb.level_keymap[1].insert(43, '\u{1680}');
        }
        ("ie", Some("ogam_is434")) => {
            for (code, value) in wkb.level_keymap[2].clone() {
                if wkb.level_keymap[3].get(&code).is_none() {
                    wkb.level_keymap[3].insert(code, value);
                }
            }
            for (code, value) in wkb.level_keymap[0].clone() {
                if [2, 3, 4, 6, 7, 9, 10, 11, 12, 13].contains(&code) {
                    wkb.level_keymap[2].insert(code, value);
                    let value = *wkb.level_keymap[1].get(&code).unwrap();
                    wkb.level_keymap[3].insert(code, value);
                }
            }
        }
        ("si", Some(_)) => {
            let value = *wkb.level_keymap[0].get(&41).unwrap();
            wkb.level_keymap[2].insert(41, value);
            let value = *wkb.level_keymap[1].get(&41).unwrap();
            wkb.level_keymap[3].insert(41, value);
        }
        ("se", Some("rus")) => {
            let value = *wkb.level_keymap[0].get(&13).unwrap();
            wkb.level_keymap[2].insert(13, value);
            let value = *wkb.level_keymap[1].get(&13).unwrap();
            wkb.level_keymap[3].insert(13, value);
            for i in 16..=27 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[2].insert(i, value);
                let value = *wkb.level_keymap[1].get(&i).unwrap();
                wkb.level_keymap[3].insert(i, value);
            }
            for i in 30..=41 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[2].insert(i, value);
                let value = *wkb.level_keymap[1].get(&i).unwrap();
                wkb.level_keymap[3].insert(i, value);
            }
            for i in 43..=50 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[2].insert(i, value);
                let value = *wkb.level_keymap[1].get(&i).unwrap();
                wkb.level_keymap[3].insert(i, value);
            }
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[2].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[3].insert(86, value);
        }
        ("us", Some("3l")) => {
            wkb.level_keymap[1].insert(2, '!');
            wkb.level_keymap[1].insert(3, '@');
            wkb.level_keymap[1].insert(4, '#');
            wkb.level_keymap[1].insert(5, '$');
            wkb.level_keymap[1].insert(6, '%');
            wkb.level_keymap[1].insert(7, '^');
            wkb.level_keymap[1].insert(8, '&');
            wkb.level_keymap[1].insert(9, '*');
            wkb.level_keymap[1].insert(10, '(');
            wkb.level_keymap[1].insert(11, ')');
            wkb.level_keymap[3].insert(2, '!');
            wkb.level_keymap[3].insert(3, '@');
            wkb.level_keymap[3].insert(4, '#');
            wkb.level_keymap[3].insert(5, '$');
            wkb.level_keymap[3].insert(6, '%');
            wkb.level_keymap[3].insert(7, '^');
            wkb.level_keymap[3].insert(8, '&');
            wkb.level_keymap[3].insert(9, '*');
            wkb.level_keymap[3].insert(10, '(');
            wkb.level_keymap[3].insert(11, ')');
            let value = *wkb.level_keymap[0].get(&15).unwrap();
            wkb.level_keymap[1].insert(15, value);
            wkb.level_keymap[2].insert(15, value);
            wkb.level_keymap[3].insert(15, value);
            wkb.level_keymap[4].insert(15, value);
            let value = *wkb.level_keymap[0].get(&58).unwrap();
            wkb.level_keymap[1].insert(58, value);
            wkb.level_keymap[3].insert(58, value);
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[4].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[5].insert(86, value);
            let value = *wkb.level_keymap[2].get(&86).unwrap();
            wkb.level_keymap[6].insert(86, value);
        }
        ("us", Some("3l-cros")) => {
            wkb.level_keymap[1].insert(2, '!');
            wkb.level_keymap[1].insert(3, '@');
            wkb.level_keymap[1].insert(4, '#');
            wkb.level_keymap[1].insert(5, '$');
            wkb.level_keymap[1].insert(6, '%');
            wkb.level_keymap[1].insert(7, '^');
            wkb.level_keymap[1].insert(8, '&');
            wkb.level_keymap[1].insert(9, '*');
            wkb.level_keymap[1].insert(10, '(');
            wkb.level_keymap[1].insert(11, ')');
            wkb.level_keymap[3].insert(2, '!');
            wkb.level_keymap[3].insert(3, '@');
            wkb.level_keymap[3].insert(4, '#');
            wkb.level_keymap[3].insert(5, '$');
            wkb.level_keymap[3].insert(6, '%');
            wkb.level_keymap[3].insert(7, '^');
            wkb.level_keymap[3].insert(8, '&');
            wkb.level_keymap[3].insert(9, '*');
            wkb.level_keymap[3].insert(10, '(');
            wkb.level_keymap[3].insert(11, ')');
            let value = *wkb.level_keymap[0].get(&15).unwrap();
            wkb.level_keymap[1].insert(15, value);
            wkb.level_keymap[2].insert(15, value);
            wkb.level_keymap[3].insert(15, value);
            wkb.level_keymap[4].insert(15, value);
            let value = *wkb.level_keymap[0].get(&58).unwrap();
            wkb.level_keymap[1].insert(58, value);
            wkb.level_keymap[3].insert(58, value);
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[4].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[5].insert(86, value);
            let value = *wkb.level_keymap[2].get(&86).unwrap();
            wkb.level_keymap[6].insert(86, value);
            let value = *wkb.level_keymap[0].get(&125).unwrap();
            wkb.level_keymap[1].insert(125, value);
            wkb.level_keymap[3].insert(125, value);
        }
        ("us", Some("3l-emacs")) => {
            wkb.level_keymap[1].insert(2, '!');
            wkb.level_keymap[1].insert(3, '@');
            wkb.level_keymap[1].insert(4, '#');
            wkb.level_keymap[1].insert(5, '$');
            wkb.level_keymap[1].insert(6, '%');
            wkb.level_keymap[1].insert(7, '^');
            wkb.level_keymap[1].insert(8, '&');
            wkb.level_keymap[1].insert(9, '*');
            wkb.level_keymap[1].insert(10, '(');
            wkb.level_keymap[1].insert(11, ')');
            wkb.level_keymap[3].insert(2, '!');
            wkb.level_keymap[3].insert(3, '@');
            wkb.level_keymap[3].insert(4, '#');
            wkb.level_keymap[3].insert(5, '$');
            wkb.level_keymap[3].insert(6, '%');
            wkb.level_keymap[3].insert(7, '^');
            wkb.level_keymap[3].insert(8, '&');
            wkb.level_keymap[3].insert(9, '*');
            wkb.level_keymap[3].insert(10, '(');
            wkb.level_keymap[3].insert(11, ')');
            let value = *wkb.level_keymap[0].get(&15).unwrap();
            wkb.level_keymap[1].insert(15, value);
            wkb.level_keymap[3].insert(15, value);
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[4].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[5].insert(86, value);
            let value = *wkb.level_keymap[2].get(&86).unwrap();
            wkb.level_keymap[6].insert(86, value);
        }
        ("fr", Some("bepo_latin9")) => {
            wkb.caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            let value = *wkb.level_keymap[2].get(&55).unwrap();
            wkb.level_keymap[3].insert(55, value);
            let value = *wkb.level_keymap[2].get(&98).unwrap();
            wkb.level_keymap[3].insert(98, value);
        }
        ("fr", Some("oss_latin9")) | ("be", Some("oss_latin9")) => {
            let value = *wkb.level_keymap[2].get(&55).unwrap();
            wkb.level_keymap[3].insert(55, value);
            let value = *wkb.level_keymap[2].get(&98).unwrap();
            wkb.level_keymap[3].insert(98, value);
        }
        ("fr", Some("mac")) => {
            let value = *wkb.level_keymap[0].get(&83).unwrap();
            wkb.level_keymap[3].insert(83, value);
        }
        ("fr", Some("azerty")) => {
            // let value = *wkb.level_keymap[0].get(&83).unwrap();
            // wkb.level_keymap[3].insert(83, value);
        }
        ("fr", Some("afnor")) => {
            for i in 0..2 {
                for (code, value) in &wkb.level_keymap[i].clone() {
                    if wkb.level_keymap[i + 4].get(&code).is_none() {
                        wkb.level_keymap[i + 4].insert(*code, *value);
                    }
                }
            }
            wkb.level_keymap[4].insert(86, '<');
            wkb.level_keymap[5].insert(55, '⋅');
            wkb.level_keymap[5].insert(74, '−');
            wkb.level_keymap[5].insert(86, '>');
            wkb.level_keymap[5].insert(98, '∕');
        }
        ("de", Some("T3")) => {
            wkb.custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
            wkb.level_keymap[3].insert(2, 'ʹ');
            wkb.level_keymap[3].insert(3, 'ʺ');
            wkb.level_keymap[3].insert(4, 'ʿ');
            wkb.level_keymap[3].insert(5, 'ʾ');
            wkb.level_keymap[3].insert(6, 'ˁ');
            wkb.level_keymap[3].insert(7, 'ˀ');
            wkb.level_keymap[3].insert(8, '{');
            wkb.level_keymap[3].insert(9, '}');
            wkb.level_keymap[3].insert(10, '[');
            wkb.level_keymap[3].insert(11, ']');
            wkb.level_keymap[3].insert(12, 'ʻ');
            wkb.level_keymap[3].insert(13, '¬');
            wkb.level_keymap[3].insert(15, '\t');
            wkb.level_keymap[3].insert(16, '\u{30d}');
            wkb.level_keymap[3].insert(27, '@');
            wkb.level_keymap[3].insert(30, '\u{329}');
            wkb.level_keymap[3].insert(35, '\u{332}');
            wkb.level_keymap[3].insert(38, '\u{338}');
            wkb.level_keymap[3].insert(39, '°');
            wkb.level_keymap[3].insert(40, '′');
            wkb.level_keymap[3].insert(41, '|');
            wkb.level_keymap[3].insert(43, '″');
            wkb.level_keymap[3].insert(44, '«');
            wkb.level_keymap[3].insert(45, '»');
            wkb.level_keymap[3].insert(46, '―');
            wkb.level_keymap[3].insert(47, '‹');
            wkb.level_keymap[3].insert(48, '›');
            wkb.level_keymap[3].insert(49, '–');
            wkb.level_keymap[3].insert(50, '—');
            wkb.level_keymap[3].insert(51, '$');
            wkb.level_keymap[3].insert(52, '#');
            wkb.level_keymap[3].insert(53, '‑');
            wkb.level_keymap[3].insert(57, '\u{a0}');
        }
        ("brai", Some("home_row"))
        | ("brai", Some("right_hand"))
        | ("brai", Some("keypad"))
        | ("apl", Some("common"))
        | ("apl", Some("dyalog_box"))
        | ("kr", Some("hw_keys"))
        | ("jp", Some("hztg_escape"))
        | ("brai", Some("right_hand_invert")) => {
            wkb.level_keymap.push(BTreeMap::from([
                (43, '|'),
                (71, '7'),
                (72, '8'),
                (73, '9'),
                (75, '4'),
                (76, '5'),
                (77, '6'),
                (78, '+'),
                (79, '1'),
                (80, '2'),
                (81, '3'),
                (82, '0'),
                (83, '.'),
                (86, '>'),
            ]));
        }
        ("am", Some("eastern")) | ("am", Some("western")) | ("am", Some("eastern-alt")) => {
            wkb.num_lock_keys.push(2);
            wkb.num_lock_keys.push(5);
            wkb.num_lock_keys.push(6);
            wkb.num_lock_keys.push(7);
        }
        ("ru", Some("ruintl_en")) => {
            wkb.level_keymap[3].insert(36, 'Ø');
        }
        ("cm", Some("azerty")) => {
            wkb.num_lock_keys.push(2);
            wkb.num_lock_keys.push(3);
            wkb.num_lock_keys.push(4);
            wkb.num_lock_keys.push(5);
            wkb.num_lock_keys.push(6);
            wkb.num_lock_keys.push(7);
            wkb.num_lock_keys.push(8);
            wkb.num_lock_keys.push(9);
            wkb.num_lock_keys.push(10);
            wkb.num_lock_keys.push(11);
        }
        ("la", Some("stea")) => wkb.num_lock_keys.retain(|k| k != &83),
        ("de", Some("neo_base"))
        | ("de", Some("neo"))
        | ("de", Some("neo_qwerty_base"))
        | ("de", Some("neo_qwerty"))
        | ("de", Some("neo_qwertz_base"))
        | ("de", Some("neo_qwertz"))
        | ("de", Some("bone_base"))
        | ("de", Some("bone_eszett_home_base"))
        | ("de", Some("bone_eszett_home"))
        | ("de", Some("bone"))
        | ("de", Some("koy_base"))
        | ("de", Some("koy"))
        | ("de", Some("adnw_base"))
        | ("de", Some("adnw")) => {
            wkb.num_lock_keys = Vec::new();
            wkb.custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
        }
        _ => {}
    }
    if wkb.level_keymap.len() > 1 {
        for (code, value) in wkb.level_keymap[0].clone() {
            if wkb.level_keymap[1].get(&code).is_none() {
                wkb.level_keymap[1].insert(code, value);
            }
        }
    }
    for i in 0..wkb.level_keymap.len() {
        let next_large = std::cmp::min(i + 2, wkb.level_keymap.len() - 1);
        let next_small = std::cmp::min(i + 1, wkb.level_keymap.len() - 1);
        let next = std::cmp::max(next_small, next_large);
        if next > i {
            for (code, value) in &wkb.level_keymap[i].clone() {
                if wkb.level_keymap[next].get(&code).is_none() {
                    wkb.level_keymap[next].insert(*code, *value);
                }
            }
        }
    }
    for i in 0..std::cmp::min(wkb.level_keymap.len(), DEFAULT_MAP.len()) {
        for (code, value) in &DEFAULT_MAP[i] {
            if wkb.level_keymap[i].get(&code).is_none() {
                wkb.level_keymap[i].insert(*code, *value);
            }
        }
    }
}
