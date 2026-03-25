use std::{collections::BTreeMap, path::Path};

use xkb_parser::{
    ast::{Directive, Key, XkbSymbolsItem},
    keysym_name_to_char, parse,
};

use crate::{
    modifiers::{ModKind, ModType, CAPS_LOCK, TAB},
    xkb::{
        compose_parse::load_compose_table, default_keymap::DEFAULT_MAP, evdev_xkb::XKBCODES_EVDEV,
    },
    BACKSPACE, WKB,
};

pub mod compose_parse;
pub mod default_keymap;
pub mod evdev_xkb;
pub mod repeat;
pub mod xkb_compose_map;

/// Layouts that require non-evdev (Sun) keycodes and cannot be used with the
/// standard evdev keycode table.  They are filtered out everywhere we populate
/// the layouts list so that callers never accidentally try to load them.
pub const SUN_LAYOUTS: &[&str] = &[
    "sun_type6",
    "sun_type6_de",
    "sun_type6_fr",
    "sun_type6_suncompat",
    "sun_type7_suncompat",
    "suncompat",
    "sun_type7",
];

pub fn fix_xkb_edge_cases(
    wkb: &mut WKB<crate::ListComposer>,
    locale: String,
    layout: Option<String>,
) {
    //snowflake special case of bad design I don't want to support any other way!
    match (locale.as_str(), &layout.as_deref()) {
        ("de", Some("ru")) | ("de", Some("ru-recom")) | ("de", Some("ru-translit")) => {
            for i in 0..2 {
                let map = wkb.state_keymap[i].clone();
                let t21 = map.get(&21);
                let t44 = map.get(&44);
                wkb.state_keymap[i].insert(21, *t44.unwrap());
                wkb.state_keymap[i].insert(44, *t21.unwrap());
            }
        }
        _ => {}
    };
    match (locale.as_str(), &layout.as_deref()) {
        // ("at", Some(_))
        // | ("de", Some("basic"))
        // | ("de", Some("mac"))
        // | ("de", Some("mac_nodeadkeys"))
        // | ("de", Some("deadtilde"))
        // | ("de", Some("deadgraveacute"))
        // | ("de", Some("deadacute"))
        // | ("de", Some("ro"))
        // | ("de", Some("ro_nodeadkeys"))
        // | ("de", Some("dsb_qwertz"))
        // | ("de", Some("qwerty"))
        // | ("de", Some("ru"))
        // | ("de", Some("pl"))
        // | ("de", Some("tr"))
        // | ("de", Some("hu"))
        // | ("de", Some("e1"))
        // | ("de", Some("e2"))
        // | ("de", Some("dvorak"))
        // | ("it", Some("lldde"))
        // | ("cz", Some("ucw"))
        // | ("de", Some("nodeadkeys")) => {
        //     custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
        // }
        // ("de", Some("ru-translit")) | ("de", Some("ru-recom")) => {
        //     custom_case_map = Some(HashMap::from([('~', 'ẞ')]));
        // }
        // ("tr", Some("basic"))
        // | ("tr", Some("e"))
        // | ("tr", Some("intl"))
        // | ("tr", Some("olpc"))
        // | ("ua", Some("crh"))
        // | ("ua", Some("crh_f"))
        // | ("ro", Some("crh_dobruja"))
        // | ("tr", Some("f")) => {
        //     custom_case_map = Some(HashMap::from([('i', 'İ'), ('İ', 'i'), ('I', 'ı')]));
        // }
        // ("az", Some("latin")) => caps_is_level2 = Some(vec![23, 39]),
        // ("gh", Some("hausa")) => caps_is_level2 = Some(vec![39]),
        // ("ng", Some("hausa")) | ("gh", Some("fula")) | ("tr", Some("us")) => {
        //     custom_case_map = Some(HashMap::from([('ı', 'İ'), ('İ', 'ı')]));
        // }
        // ("md", Some("gag")) => {
        //     custom_case_map = Some(HashMap::from([
        //         ('ı', 'I'),
        //         ('I', 'ı'),
        //         ('i', 'İ'),
        //         ('İ', 'i'),
        //     ]));
        // }
        ("bqn", Some("bqn")) => {
            // custom_case_map = Some(HashMap::from([
            //     ('𝕨', '𝕎'),
            //     ('𝕤', '𝕊'),
            //     ('𝕗', '𝔽'),
            //     ('𝕘', '𝔾'),
            //     ('𝕩', '𝕏'),
            //     ('𝕎', '𝕨'),
            //     ('𝕊', '𝕤'),
            //     ('𝔽', '𝕗'),
            //     ('𝔾', '𝕘'),
            //     ('𝕏', '𝕩'),
            // ]));
            wkb.state_keymap[1].insert(22, '⊔');
            wkb.state_keymap[1].insert(32, '↕');
            wkb.state_keymap[1].insert(36, '∘');
            wkb.state_keymap[1].insert(46, '↓');
            wkb.state_keymap[1].insert(57, '‿');
        }
        // ("hu", Some("oldhunlig")) => {
        //     caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41, 51, 52, 53]);
        // }
        // ("hu", Some("oldhun_base")) => {
        //     caps_is_level2 = Some(vec![51, 52, 53]);
        // }
        // ("hu", Some("oldhun_origin")) | ("hu", Some("oldhun_lig")) => {
        //     caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 41]);
        // }
        // ("kz", Some("ext")) => caps_is_level2 = Some(vec![2, 7, 8, 43]),
        // ("fr", Some("bepo")) => {
        //     caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        // }
        ("fr", Some("bepo_afnor")) => {
            // caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            wkb.state_keymap.resize(4, BTreeMap::new());
        }
        ("fr", Some("dvorak")) => {
            // caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 17, 18, 41]);
            // custom_case_map = Some(HashMap::from([('à', '/'), (';', '-'), ('ç', 'ç')]))
        }
        // ("kz", Some("latin")) => {
        //     custom_case_map = Some(HashMap::from([('v', 'M')]));
        // }
        // ("us", Some("chr")) => {
        //     caps_is_level2 = Some(vec![
        //         1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        //         26, 27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 49,
        //         50, 51, 52, 53,
        //     ])
        // }
        ("us", Some("mac")) => wkb.caps_lock_keymap[1].clear(),
        // ("il", Some("si2")) | ("il", Some("basic")) => {
        //     caps_is_level2 = Some(vec![
        //         16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 44,
        //         45, 46, 47, 48, 49, 50, 51, 52,
        //     ])
        // }
        // ("il", Some("phonetic")) => {
        //     caps_is_level2 = Some(vec![20, 25, 33, 37, 39, 46, 49, 50, 51, 52])
        // }
        // ("il", Some("biblical")) => {
        //     caps_is_level2 = Some(vec![
        //         2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        //         27, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 49, 50,
        //         51, 52, 53,
        //     ])
        // }
        // ("il", Some("biblicalSIL")) => {
        //     caps_is_level2 = Some(vec![
        //         2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 16, 18, 21, 24, 25, 26, 27, 30, 31, 33, 36, 37,
        //         39, 40, 41, 46, 49, 50, 51, 52, 53,
        //     ])
        // }
        // ("it", Some("geo")) => {
        //     caps_is_level2 = Some(vec![
        //         16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
        //     ])
        // }
        // ("pl", Some("dvp")) | ("us", Some("dvp")) => {
        //     caps_is_level2 = Some(vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 40])
        // }
        ("in", Some("tamilnet_TAB")) | ("lk", Some("tam_TAB")) => {
            wkb.caps_lock_keymap[0].clear();
            wkb.caps_lock_keymap[1].clear();
        }
        ("in", Some("tamilnet_TSCII")) => {
            // custom_case_map = Some(HashMap::from([('þ', 'þ')]));
            wkb.caps_lock_keymap[1].clear();
        }
        // ("in", Some("iipa")) => custom_case_map = Some(HashMap::from([('b', 'Y')])),
        // ("ge", Some("qwerty")) | ("ge", Some("basic")) => {
        //     caps_is_level2 = Some(vec![
        //         16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
        //     ])
        // }
        // ("ge", Some("ergonomic")) => {
        //     caps_is_level2 = Some(vec![
        //         16, 18, 21, 22, 23, 24, 25, 30, 32, 33, 34, 35, 37, 38, 45, 47, 48, 49, 50,
        //     ])
        // }
        // ("ge", Some("mess")) => {
        //     caps_is_level2 = Some(vec![
        //         16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 36, 37, 38, 44, 45,
        //         46, 47, 48, 49, 50,
        //     ])
        // }
        // ("fr", Some("geo")) => {
        //     caps_is_level2 = Some(vec![
        //         17, 18, 20, 22, 24, 25, 26, 27, 30, 31, 32, 37, 38, 39, 40, 44, 48, 51,
        //     ]);
        // }
        // ("gr", Some(_)) => {
        //     caps_is_level2 = Some(vec![17]);
        // }
        ("pl", Some("glagolica")) => {
            wkb.caps_lock_keymap[1].clear();
            let value = *wkb.state_keymap[0].get(&40).unwrap();
            wkb.state_keymap[1].insert(40, value);
        }
        ("cd", _) | ("ml", Some("us-mac")) => wkb.caps_lock_keymap[1].clear(),
        ("ua", Some("winkeysenhanced")) => {
            if let Some(&v0) = wkb.state_keymap[0].get(&86) {
                wkb.state_keymap[2].insert(86, v0);
            }
            if let Some(&v1) = wkb.state_keymap[1].get(&86) {
                wkb.state_keymap[3].insert(86, v1);
            }
        }
        ("apl", Some("apl2")) | ("apl", Some("aplplusII")) => {
            for i in 16..=25 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[1].insert(i, value);
            }
            for i in 30..=38 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[1].insert(i, value);
            }
            for i in 44..=50 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[1].insert(i, value);
            }
        }
        ("ie", Some("ogam")) => {
            wkb.state_keymap[1].insert(43, '\u{1680}');
        }
        ("ie", Some("ogam_is434")) => {
            for (code, value) in wkb.state_keymap[2].clone() {
                if wkb.state_keymap[3].get(&code).is_none() {
                    wkb.state_keymap[3].insert(code, value);
                }
            }
            for (code, value) in wkb.state_keymap[0].clone() {
                if [2, 3, 4, 6, 7, 9, 10, 11, 12, 13].contains(&code) {
                    wkb.state_keymap[2].insert(code, value);
                    let value = *wkb.state_keymap[1].get(&code).unwrap();
                    wkb.state_keymap[3].insert(code, value);
                }
            }
            // Ensure level2/level3 follow xkb for key 86 (᚛ / ᚜).
            wkb.state_keymap[2].insert(86, '᚛');
            wkb.state_keymap[3].insert(86, '᚜');
            wkb.caps_lock_keymap[2].insert(86, '᚛');
            wkb.caps_lock_keymap[3].insert(86, '᚜');
        }
        ("si", Some(_)) => {
            let value = *wkb.state_keymap[0].get(&41).unwrap();
            wkb.state_keymap[2].insert(41, value);
            let value = *wkb.state_keymap[1].get(&41).unwrap();
            wkb.state_keymap[3].insert(41, value);
        }
        ("se", Some("rus")) => {
            let value = *wkb.state_keymap[0].get(&13).unwrap();
            wkb.state_keymap[2].insert(13, value);
            let value = *wkb.state_keymap[1].get(&13).unwrap();
            wkb.state_keymap[3].insert(13, value);
            for i in 16..=27 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[2].insert(i, value);
                let value = *wkb.state_keymap[1].get(&i).unwrap();
                wkb.state_keymap[3].insert(i, value);
            }
            for i in 30..=41 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[2].insert(i, value);
                let value = *wkb.state_keymap[1].get(&i).unwrap();
                wkb.state_keymap[3].insert(i, value);
            }
            for i in 43..=50 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[2].insert(i, value);
                let value = *wkb.state_keymap[1].get(&i).unwrap();
                wkb.state_keymap[3].insert(i, value);
            }
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[2].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[3].insert(86, value);
        }
        ("us", Some("3l")) => {
            let value = *wkb.state_keymap[0].get(&15).unwrap();
            wkb.state_keymap[1].insert(15, value);
            wkb.state_keymap[2].insert(15, value);
            wkb.state_keymap[3].insert(15, value);
            wkb.state_keymap[4].insert(15, value);
            let value = *wkb.state_keymap[0].get(&58).unwrap();
            wkb.state_keymap[1].insert(58, value);
            wkb.state_keymap[3].insert(58, value);
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[4].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[5].insert(86, value);
            let value = *wkb.state_keymap[2].get(&86).unwrap();
            wkb.state_keymap[6].insert(86, value);
        }
        ("us", Some("3l-cros")) => {
            wkb.state_keymap[1].insert(2, '!');
            wkb.state_keymap[1].insert(3, '@');
            wkb.state_keymap[1].insert(4, '#');
            wkb.state_keymap[1].insert(5, '$');
            wkb.state_keymap[1].insert(6, '%');
            wkb.state_keymap[1].insert(7, '^');
            wkb.state_keymap[1].insert(8, '&');
            wkb.state_keymap[1].insert(9, '*');
            wkb.state_keymap[1].insert(10, '(');
            wkb.state_keymap[1].insert(11, ')');
            wkb.state_keymap[3].insert(2, '!');
            wkb.state_keymap[3].insert(3, '@');
            wkb.state_keymap[3].insert(4, '#');
            wkb.state_keymap[3].insert(5, '$');
            wkb.state_keymap[3].insert(6, '%');
            wkb.state_keymap[3].insert(7, '^');
            wkb.state_keymap[3].insert(8, '&');
            wkb.state_keymap[3].insert(9, '*');
            wkb.state_keymap[3].insert(10, '(');
            wkb.state_keymap[3].insert(11, ')');
            let value = *wkb.state_keymap[0].get(&15).unwrap();
            wkb.state_keymap[1].insert(15, value);
            wkb.state_keymap[2].insert(15, value);
            wkb.state_keymap[3].insert(15, value);
            wkb.state_keymap[4].insert(15, value);
            let value = *wkb.state_keymap[0].get(&58).unwrap();
            wkb.state_keymap[1].insert(58, value);
            wkb.state_keymap[3].insert(58, value);
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[4].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[5].insert(86, value);
            let value = *wkb.state_keymap[2].get(&86).unwrap();
            wkb.state_keymap[6].insert(86, value);
            let value = *wkb.state_keymap[0].get(&125).unwrap();
            wkb.state_keymap[1].insert(125, value);
            wkb.state_keymap[3].insert(125, value);
        }
        ("us", Some("3l-emacs")) => {
            wkb.state_keymap[1].insert(2, '!');
            wkb.state_keymap[1].insert(3, '@');
            wkb.state_keymap[1].insert(4, '#');
            wkb.state_keymap[1].insert(5, '$');
            wkb.state_keymap[1].insert(6, '%');
            wkb.state_keymap[1].insert(7, '^');
            wkb.state_keymap[1].insert(8, '&');
            wkb.state_keymap[1].insert(9, '*');
            wkb.state_keymap[1].insert(10, '(');
            wkb.state_keymap[1].insert(11, ')');
            wkb.state_keymap[3].insert(2, '!');
            wkb.state_keymap[3].insert(3, '@');
            wkb.state_keymap[3].insert(4, '#');
            wkb.state_keymap[3].insert(5, '$');
            wkb.state_keymap[3].insert(6, '%');
            wkb.state_keymap[3].insert(7, '^');
            wkb.state_keymap[3].insert(8, '&');
            wkb.state_keymap[3].insert(9, '*');
            wkb.state_keymap[3].insert(10, '(');
            wkb.state_keymap[3].insert(11, ')');
            let value = *wkb.state_keymap[0].get(&15).unwrap();
            wkb.state_keymap[1].insert(15, value);
            wkb.state_keymap[3].insert(15, value);
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[4].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[5].insert(86, value);
            let value = *wkb.state_keymap[2].get(&86).unwrap();
            wkb.state_keymap[6].insert(86, value);
        }
        ("fr", Some("bepo_latin9")) => {
            // caps_is_level2 = Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            wkb.level_exceptions_keymap = vec![BTreeMap::new(); wkb.state_keymap.len()];
            if let Some(&v) = wkb.state_keymap[1].get(&55) {
                wkb.level_exceptions_keymap[1].insert(55, v);
            }
            if let Some(&v) = wkb.state_keymap[2].get(&55) {
                wkb.level_exceptions_keymap[2].insert(55, v);
            }
            if let Some(&v) = wkb.state_keymap[2].get(&98) {
                wkb.level_exceptions_keymap[2].insert(98, v);
            }
            wkb.state_keymap[1].insert(55, '*');
            wkb.state_keymap[2].insert(55, '·');
            wkb.state_keymap[3].insert(55, '×');
            wkb.state_keymap[2].insert(98, '/');
            wkb.state_keymap[3].insert(98, '÷');
        }
        ("fr", Some("oss_latin9")) | ("be", Some("oss_latin9")) => {
            wkb.level_exceptions_keymap = vec![BTreeMap::new(); wkb.state_keymap.len()];
            if let Some(&v) = wkb.state_keymap[1].get(&55) {
                wkb.level_exceptions_keymap[1].insert(55, v);
            }
            if let Some(&v) = wkb.state_keymap[2].get(&55) {
                wkb.level_exceptions_keymap[2].insert(55, v);
            }
            if let Some(&v) = wkb.state_keymap[2].get(&98) {
                wkb.level_exceptions_keymap[2].insert(98, v);
            }
            wkb.state_keymap[1].insert(55, '*');
            wkb.state_keymap[2].insert(55, '·');
            wkb.state_keymap[3].insert(55, '×');
            wkb.state_keymap[2].insert(98, '/');
            wkb.state_keymap[3].insert(98, '÷');
        }
        ("fr", Some("mac")) => {
            wkb.level_exceptions_keymap = vec![BTreeMap::new(); wkb.state_keymap.len()];
            if let Some(&v) = wkb.state_keymap[1].get(&83) {
                wkb.level_exceptions_keymap[1].insert(83, v);
            }
            wkb.state_keymap[1].insert(83, ',');
            let value = *wkb.state_keymap[0].get(&83).unwrap();
            wkb.state_keymap[3].insert(83, value);
        }
        ("fr", Some("azerty")) => {
            // let value = *wkb.state_keymap[0].get(&83).unwrap();
            // wkb.state_keymap[3].insert(83, value);
        }
        ("fr", Some("afnor")) => {
            for i in 0..2 {
                for (code, value) in &wkb.state_keymap[i].clone() {
                    if wkb.state_keymap[i + 4].get(&code).is_none() {
                        wkb.state_keymap[i + 4].insert(*code, *value);
                    }
                }
            }
            wkb.state_keymap[4].insert(86, '<');
            wkb.state_keymap[5].insert(55, '⋅');
            wkb.state_keymap[5].insert(74, '−');
            wkb.state_keymap[5].insert(86, '>');
            wkb.state_keymap[5].insert(98, '∕');
        }
        ("de", Some("T3")) => {
            // custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
            wkb.state_keymap[3].insert(2, 'ʹ');
            wkb.state_keymap[3].insert(3, 'ʺ');
            wkb.state_keymap[3].insert(4, 'ʿ');
            wkb.state_keymap[3].insert(5, 'ʾ');
            wkb.state_keymap[3].insert(6, 'ˁ');
            wkb.state_keymap[3].insert(7, 'ˀ');
            wkb.state_keymap[3].insert(8, '{');
            wkb.state_keymap[3].insert(9, '}');
            wkb.state_keymap[3].insert(10, '[');
            wkb.state_keymap[3].insert(11, ']');
            wkb.state_keymap[3].insert(12, 'ʻ');
            wkb.state_keymap[3].insert(13, '¬');
            wkb.state_keymap[3].insert(15, '\t');
            wkb.state_keymap[3].insert(16, '\u{30d}');
            wkb.state_keymap[3].insert(27, '@');
            wkb.state_keymap[3].insert(30, '\u{329}');
            wkb.state_keymap[3].insert(35, '\u{332}');
            wkb.state_keymap[3].insert(38, '\u{338}');
            wkb.state_keymap[3].insert(39, '°');
            wkb.state_keymap[3].insert(40, '′');
            wkb.state_keymap[3].insert(41, '|');
            wkb.state_keymap[3].insert(43, '″');
            wkb.state_keymap[3].insert(44, '«');
            wkb.state_keymap[3].insert(45, '»');
            wkb.state_keymap[3].insert(46, '―');
            wkb.state_keymap[3].insert(47, '‹');
            wkb.state_keymap[3].insert(48, '›');
            wkb.state_keymap[3].insert(49, '–');
            wkb.state_keymap[3].insert(50, '—');
            wkb.state_keymap[3].insert(51, '$');
            wkb.state_keymap[3].insert(52, '#');
            wkb.state_keymap[3].insert(53, '‑');
            wkb.state_keymap[3].insert(57, '\u{a0}');
        }
        ("brai", Some("home_row"))
        | ("brai", Some("right_hand"))
        | ("brai", Some("keypad"))
        | ("apl", Some("common"))
        | ("apl", Some("dyalog_box"))
        | ("apl", Some("dyalog"))
        | ("kr", Some("hw_keys"))
        | ("jp", Some("hztg_escape"))
        | ("brai", Some("right_hand_invert")) => {
            wkb.state_keymap.push(BTreeMap::from([
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

            if locale.as_str() == "apl"
                && (layout.as_deref() == Some("dyalog") || layout.as_deref() == Some("dyalog_box"))
            {
                wkb.level_exceptions_keymap = vec![BTreeMap::new(); wkb.state_keymap.len()];
                for &code in &[71, 72, 73, 75, 76, 77, 79, 80, 81, 82, 83] {
                    if let Some(&v) = wkb.state_keymap[1].get(&code) {
                        wkb.level_exceptions_keymap[1].insert(code, v);
                    }
                }
                // Preserve pushed-level originals in level_exceptions (level 2) for dyalog only
                if layout.as_deref() == Some("dyalog") {
                    let pushed_level = wkb.state_keymap.len() - 1;
                    wkb.level_exceptions_keymap[pushed_level].insert(55, '*');
                    wkb.level_exceptions_keymap[pushed_level].insert(74, '-');
                    wkb.level_exceptions_keymap[pushed_level].insert(86, '|');
                    wkb.level_exceptions_keymap[pushed_level].insert(98, '/');
                }
                wkb.state_keymap[1].insert(71, '┌');
                wkb.state_keymap[1].insert(72, '┬');
                wkb.state_keymap[1].insert(73, '┐');
                wkb.state_keymap[1].insert(75, '├');
                wkb.state_keymap[1].insert(76, '┼');
                wkb.state_keymap[1].insert(77, '┤');
                wkb.state_keymap[1].insert(79, '└');
                wkb.state_keymap[1].insert(80, '┴');
                wkb.state_keymap[1].insert(81, '┘');
                wkb.state_keymap[1].insert(82, '─');
                wkb.state_keymap[1].insert(83, '│');
            }
        }
        // ("am", Some("eastern")) | ("am", Some("western")) | ("am", Some("eastern-alt")) => {
        //     num_lock_codes.push(2);
        //     num_lock_codes.push(5);
        //     num_lock_codes.push(6);
        //     num_lock_codes.push(7);
        // }
        ("am", Some("eastern")) => {
            wkb.level_exceptions_keymap = vec![BTreeMap::new(); wkb.state_keymap.len()];
            for &code in &[2, 5, 6, 7] {
                if let Some(&v) = wkb.state_keymap[1].get(&code) {
                    wkb.level_exceptions_keymap[1].insert(code, v);
                }
            }
            wkb.state_keymap[1].insert(2, '։');
            wkb.state_keymap[1].insert(5, '՛');
            wkb.state_keymap[1].insert(6, ',');
            wkb.state_keymap[1].insert(7, '-');
        }
        ("am", Some("western")) | ("am", Some("eastern-alt")) => {
            wkb.level_exceptions_keymap = vec![BTreeMap::new(); wkb.state_keymap.len()];
            for &code in &[2, 5, 6, 7] {
                if let Some(&v) = wkb.state_keymap[1].get(&code) {
                    wkb.level_exceptions_keymap[1].insert(code, v);
                }
            }
            wkb.state_keymap[1].insert(2, '։');
            wkb.state_keymap[1].insert(5, '՛');
            wkb.state_keymap[1].insert(6, ',');
            wkb.state_keymap[1].insert(7, '-');
        }
        ("ru", Some("ruintl_en")) => {
            wkb.state_keymap[3].insert(36, 'Ø');
        }
        // ("cm", Some("azerty")) => {
        //     num_lock_codes.push(2);
        //     num_lock_codes.push(3);
        //     num_lock_codes.push(4);
        //     num_lock_codes.push(5);
        //     num_lock_codes.push(6);
        //     num_lock_codes.push(7);
        //     num_lock_codes.push(8);
        //     num_lock_codes.push(9);
        //     num_lock_codes.push(10);
        //     num_lock_codes.push(11);
        // }
        // ("la", Some("stea")) => num_lock_codes.retain(|k| k != &83),
        // ("de", Some("neo_base"))
        // | ("de", Some("neo"))
        // | ("de", Some("neo_qwerty_base"))
        // | ("de", Some("neo_qwerty"))
        // | ("de", Some("neo_qwertz_base"))
        // | ("de", Some("neo_qwertz"))
        // | ("de", Some("bone_base"))
        // | ("de", Some("bone_eszett_home_base"))
        // | ("de", Some("bone_eszett_home"))
        // | ("de", Some("bone"))
        // | ("de", Some("koy_base"))
        // | ("de", Some("koy"))
        // | ("de", Some("adnw_base"))
        // | ("de", Some("adnw")) => {
        //     num_lock_codes = Vec::new();
        //     // custom_case_map = Some(HashMap::from([('ß', 'ẞ')]));
        // }
        _ => {}
    }
    if wkb.state_keymap.len() > 1 {
        for (code, value) in wkb.state_keymap[0].clone() {
            if wkb.state_keymap[1].get(&code).is_none() {
                wkb.state_keymap[1].insert(code, value);
            }
        }
    }
    for i in 0..wkb.state_keymap.len() {
        let next_large = std::cmp::min(i + 2, wkb.state_keymap.len() - 1);
        let next_small = std::cmp::min(i + 1, wkb.state_keymap.len() - 1);
        let next = std::cmp::max(next_small, next_large);
        if next > i {
            for (code, value) in &wkb.state_keymap[i].clone() {
                if wkb.state_keymap[next].get(&code).is_none() {
                    wkb.state_keymap[next].insert(*code, *value);
                }
            }
        }
    }
    for i in 0..wkb.state_keymap.len() {
        for (code, value) in &DEFAULT_MAP[i] {
            if wkb.state_keymap[i].get(&code).is_none() {
                wkb.state_keymap[i].insert(*code, *value);
            }
        }
    }
}

fn extract_key_type(values: &[xkb_parser::ast::KeyValue]) -> Option<String> {
    for v in values {
        if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
            if let xkb_parser::ast::KeyDef::TypeDef(type_def) = key_defs {
                if type_def.group.is_none()
                    || matches!(
                        type_def.group.as_ref().map(|g| g.content),
                        Some("Group1") | Some("group1")
                    )
                {
                    return Some(type_def.content.content.to_string());
                }
            }
        }
    }
    None
}

fn is_numpad_key(keysym: &str, evdev_code: &u32, _key_type: &Option<String>) -> bool {
    if evdev_code == &71
        || evdev_code == &72
        || evdev_code == &73
        || evdev_code == &75
        || evdev_code == &76
        || evdev_code == &77
        || evdev_code == &79
        || evdev_code == &80
        || evdev_code == &81
        || evdev_code == &82
        || evdev_code == &83
    {
        return true;
    }
    if keysym.starts_with("KP") {
        return true;
    }
    false
}

// This recursive function from hell is currently used only to map xkb
// hopefully xkb files can be depricated one day so one does not have to work
// with this cursed file format.
pub fn map_xkb(
    wkb: &mut WKB<crate::ListComposer>,
    path: &Path,
    locale: String,
    layout: Option<String>,
    numpad_key_types: &mut BTreeMap<u32, String>,
) {
    let file = std::fs::read_to_string(&path.join(locale.clone())).expect("dir");
    let xkb = parse(&file).unwrap();
    xkb.definitions.iter().for_each(|d| {
        if let Directive::XkbSymbols(src) = &d.directive {
            let layout = layout.clone().unwrap_or(wkb.current_layout());
            if src.name.content == layout {
                let mut pending_key_type: Option<(String, Option<String>)> = None;
                src.value.iter().for_each(|si| {
                    if let XkbSymbolsItem::KeyType(key_type_item) = si {
                        pending_key_type = Some((
                            key_type_item.name.content.to_string(),
                            key_type_item.group.as_ref().map(|g| g.content.to_string()),
                        ));
                    } else if let XkbSymbolsItem::Include(include) = si {
                        let (locale, layout) = include.parse_name();
                        if layout.is_none() {
                            map_xkb(
                                wkb,
                                path,
                                locale.to_string(),
                                Some("basic".to_string()),
                                numpad_key_types,
                            );
                        } else {
                            map_xkb(
                                wkb,
                                path,
                                locale.to_string(),
                                layout.map(str::to_string),
                                numpad_key_types,
                            );
                        }
                    } else if let XkbSymbolsItem::Key(Key {
                        mode: _,
                        id,
                        values,
                    }) = si
                    {
                        if let Some(evdev_code) = XKBCODES_EVDEV.get(id.content) {
                            let key_type = extract_key_type(values);

                            let evdev_code_copy = *evdev_code;

                            let effective_key_type = key_type.clone().or_else(|| {
                                if let Some((ref type_name, ref group)) = pending_key_type {
                                    if group
                                        .as_ref()
                                        .map(|g| g == "Group1" || g == "group1")
                                        .unwrap_or(true)
                                    {
                                        return Some(type_name.clone());
                                    }
                                }
                                None
                            });

                            if let Some(ref kt) = effective_key_type {
                                if kt.contains("KEYPAD") {
                                    numpad_key_types.insert(evdev_code_copy, kt.clone());
                                }
                            }

                            if pending_key_type.is_some() {
                                pending_key_type = None;
                            }

                            let mut key_type_from_type_def: Option<String> = None;
                            values.iter().for_each(|v| {
                                if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
                                    if let xkb_parser::ast::KeyDef::TypeDef(type_def) = key_defs {
                                        if type_def.group.is_none()
                                            || matches!(
                                                type_def.group.as_ref().map(|g| g.content),
                                                Some("Group1") | Some("group1")
                                            )
                                        {
                                            key_type_from_type_def =
                                                Some(type_def.content.content.to_string());
                                        }
                                    }
                                    if let xkb_parser::ast::KeyDef::SymbolDef(key) = key_defs {
                                        for (i, v) in key.values.values.iter().enumerate() {
                                            if i == wkb.state_keymap.len() {
                                                wkb.state_keymap.push(DEFAULT_MAP[i].clone());
                                                wkb.num_lock_keys.push(BTreeMap::new());
                                                wkb.caps_lock_keymap.push(BTreeMap::new());
                                            }
                                            let single_char = keysym_name_to_char(v.as_ref());
                                            if *evdev_code == 83 {
                                                eprintln!(
                                                    "DEBUG SymbolDef: keysym={:?}, char={:?}, i={}",
                                                    v.as_ref(),
                                                    single_char,
                                                    i
                                                );
                                            }
                                            if let Some(char) = single_char {
                                                wkb.state_keymap[i].insert(*evdev_code, char);

                                                let effective_kt = key_type_from_type_def
                                                    .clone()
                                                    .or_else(|| effective_key_type.clone());
                                                if is_numpad_key(
                                                    v.as_ref(),
                                                    evdev_code,
                                                    &effective_kt,
                                                ) {
                                                    wkb.num_lock_keys[i].insert(*evdev_code, char);
                                                }
                                            }
                                        }
                                    }
                                } else if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                    if *evdev_code == 83 {
                                        eprintln!("DEBUG: KeyNames for KPDL in {}", layout);
                                    }
                                    map_keys_and_modifiers(
                                        wkb,
                                        key,
                                        evdev_code,
                                        layout.clone(),
                                        locale.clone(),
                                        id.content.to_owned(),
                                        effective_key_type.clone(),
                                    );
                                }
                            })
                        }
                    }
                })
            }
        }
    });
    for (i, map) in wkb.state_keymap.iter_mut().enumerate() {
        if i % 2 == 0 && i + 1 < wkb.caps_lock_keymap.len() {
            wkb.caps_lock_keymap[i + 1] = map.clone();
        } else if i % 2 == 1 {
            wkb.caps_lock_keymap[i - 1] = map.clone();
        }
    }

    for i in 0..wkb.state_keymap.len() {
        match i {
            0 | 2 | 4 | 6 => {
                for (key, value) in [
                    (71, '7'),
                    (72, '8'),
                    (73, '9'),
                    (75, '4'),
                    (76, '5'),
                    (77, '6'),
                    (79, '1'),
                    (80, '2'),
                    (81, '3'),
                    (82, '0'),
                ] {
                    if let Some(_) = wkb.num_lock_keys[i].get(&key) {
                    } else if let Some(state_value) = wkb.state_keymap[i].get(&key) {
                        wkb.num_lock_keys[i].insert(key, *state_value);
                    } else {
                        wkb.num_lock_keys[i].insert(key, value);
                    }
                }
                if wkb.num_lock_keys[i].get(&83).is_none() {
                    if i > 0 {
                        let (first, last) = wkb.num_lock_keys.split_at_mut(i);
                        if let Some(value) = first[1].get(&83) {
                            last[0].insert(83, *value);
                        } else {
                            last[0].insert(83, '.');
                        }
                    } else if wkb.state_keymap.get(0).and_then(|m| m.get(&83)).is_none() {
                        wkb.num_lock_keys[i].insert(83, '.');
                    }
                }
            }
            1 => {
                let (first, last) = wkb.num_lock_keys.split_at_mut(1);
                if let Some(value) = last[0].get(&83) {
                    if first[0].get(&83).is_none() {
                        first[0].insert(83, *value);
                    }
                }
            }
            _ => {}
        }
    }
}

fn map_keys_and_modifiers(
    wkb: &mut WKB<crate::ListComposer>,
    key: &xkb_parser::ast::KeyNames,
    evdev_code: &u32,
    layout: String,
    locale: String,
    id: String,
    key_type: Option<String>,
) {
    for (i, v) in key.values.iter().enumerate() {
        if id == "CAPS" && key.values.first().is_some_and(|k| k.content == "BackSpace") {
            let value = *wkb.state_keymap[i].get(&BACKSPACE).unwrap();
            wkb.state_keymap[i].insert(CAPS_LOCK, value);
            wkb.modifiers.0.remove_entry(&CAPS_LOCK);
        } else if id == "CAPS" && key.values.first().is_some_and(|k| k.content == "Tab") {
            let value = *wkb.state_keymap[i].get(&TAB).unwrap();
            wkb.state_keymap[i].insert(CAPS_LOCK, value);
            wkb.modifiers.0.remove_entry(&CAPS_LOCK);
        }
        if i == wkb.state_keymap.len() {
            wkb.state_keymap.push(DEFAULT_MAP[i].clone());
            wkb.num_lock_keys.push(BTreeMap::new());
            wkb.caps_lock_keymap.push(BTreeMap::new());
        }
        let single_char = keysym_name_to_char(v.as_ref());
        if *evdev_code == 83 {
            eprintln!(
                "DEBUG map_keys_and_modifiers: evdev_code=83, keysym={:?}, char={:?}, id={}",
                v.as_ref(),
                single_char,
                id
            );
        }
        if let Some(single_char) = single_char {
            wkb.state_keymap[i].insert(*evdev_code, single_char);
            if *evdev_code == 83 && i == 1 {
                eprintln!(
                    "DEBUG map_keys: inserting num_lock_keys[1][83] = {:?}, is_numpad={}",
                    single_char,
                    is_numpad_key(v.as_ref(), evdev_code, &key_type)
                );
            }
            if is_numpad_key(v.as_ref(), evdev_code, &key_type) {
                if layout == "legacynumber" && (i == 1 || i == 2) {
                    let num_lock_maps = wkb.num_lock_keys.split_at_mut_checked(1);
                    if let Some((ref num_lock_map_start, num_lock_map_end)) = num_lock_maps {
                        if let Some(v) = num_lock_map_start[0].get(evdev_code) {
                            num_lock_map_end[0].insert(*evdev_code, v.clone());
                            num_lock_map_end[1].insert(*evdev_code, v.clone());
                        }
                    }
                } else if layout == "comma" || (layout == "dotoss" && i == 2) {
                    wkb.num_lock_keys[i].insert(*evdev_code, ',');
                } else if (layout == "dotoss" || layout == "dotoss_latin9" || layout == "dot")
                    && i == 0
                {
                    wkb.num_lock_keys[i].insert(*evdev_code, '.');
                } else {
                    wkb.num_lock_keys[i].insert(*evdev_code, single_char);
                }
            }
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
                ("Control_L", _) => if id == "CAPS" {},
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
                // (_, "rshift_both_shiftlock") | (_, "lshift_both_shiftlock") => {}
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
                            if let Some(key) = wkb.state_keymap[i - 1].clone().get(evdev_code) {
                                wkb.state_keymap[i].insert(*evdev_code, *key);
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
        }
    }
}

pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

pub fn new_from_names(locale: String, layout: Option<String>) -> crate::WKB<crate::ListComposer> {
    let path = std::path::Path::new(XKB_SYMBOLS_PATH);
    let string_file = std::fs::read_to_string(path.join(locale.clone()))
        .expect("could not read xkb symbols file");
    let xkb = parse(&string_file).expect("could not parse xkb symbols file");
    let layouts: Vec<String> = xkb
        .symbol_layout_names()
        .into_iter()
        .filter(|name| !SUN_LAYOUTS.contains(name))
        .map(str::to_string)
        .collect();
    let layout_val = if let Some(l) = layout.clone() {
        l
    } else {
        layouts[0].clone()
    };

    let repeat_keys = if let Some(locale_map) = crate::REPEAT_KEYS.get(locale.as_str()) {
        if let Some(layout_repeat) = locale_map.get(layout_val.as_str()) {
            layout_repeat.clone()
        } else {
            crate::REPEAT_DEFAULT.clone()
        }
    } else {
        crate::REPEAT_DEFAULT.clone()
    };
    let regular_composer = load_compose_table(&locale);
    let modifiers = crate::modifiers::Modifiers::default();
    let mut wkb = crate::WKB {
        layouts,
        layout: layout_val.clone(),
        locale: Some(locale.clone()),
        state_keymap: Vec::with_capacity(8),
        composer: regular_composer,
        pressed_keys: std::collections::HashSet::new(),
        repeat_keys,
        modifiers,
        num_lock_keys: Vec::with_capacity(8),
        caps_lock_keymap: Vec::with_capacity(8),
        level_exceptions_keymap: Vec::with_capacity(8),
    };
    let mut numpad_key_types: BTreeMap<u32, String> = BTreeMap::new();

    map_xkb(
        &mut wkb,
        path,
        locale.clone(),
        Some(layout_val.clone()),
        &mut numpad_key_types,
    );
    fix_xkb_edge_cases(&mut wkb, locale.clone(), Some(layout_val.clone()));

    for (&evdev_code, _) in &numpad_key_types {
        if let Some(&level1_char) = wkb.state_keymap.get(1).and_then(|m| m.get(&evdev_code)) {
            for i in 0..wkb.state_keymap.len() {
                if i != 1 {
                    wkb.num_lock_keys[i].insert(evdev_code, level1_char);
                }
            }
        }
    }

    wkb
}

pub fn new_from_string(_string: String) -> crate::WKB<crate::ListComposer> {
    // Basic stub, might need parsing string instead of relying on map_xkb reading from file
    unimplemented!("new_from_string is not yet fully implemented")
}
