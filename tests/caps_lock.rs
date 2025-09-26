use test_case::test_matrix;
use wkb::{self, modifiers::CAPS_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{key_range, multiple_keys, single_key, test_all_keys, xkb_new_from_names};

#[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn caps_lock(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        if wkb.level_keymap().is_empty() {
            continue;
        }
        // caps
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, wkb::KeyDirection::Down);

        // Define exception slice for this specific test
        let caps_lock_exceptions = &[
            ("ara", "mac-phonetic", key_range(2, 11)),
            ("eg", "cop", key_range(16, 26)),
            ("eg", "cop", key_range(30, 40)),
            ("eg", "cop", key_range(44, 50)),
            ("eg", "cop", single_key(53)),
            ("us", "3l-emacs", key_range(3, 57)),
            ("me", "latinunicodeyz", single_key(45)),
        ];

        test_all_keys(wkb, xkb, layout, caps_lock_exceptions);
    }
    // assert!(false);
}

#[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd",
    "be",
    "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de",
    "dk", "dz", "ee", "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb",
    "ge", "gh", "gn", "gr", "hr", "hu", "id", "ie", "il", "in","iq", "ir",
    "is", "it", "jp",
    "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt",
    "lv", "ma", "md",
    "me", "mk", "ml",
    "mm", "mn", "mt", "mv", "my", "latam", "latin",
    "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl",
    "pt", "ro",
    "rs",
    "ru",
    "se", "si", "sk", "sn", "tg", "th", "tj", "tm", "tr", "trans", "tw", "tz", "ua",
    "us",
    "uz", "vn", "za",
])]
fn level2_caps(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 2 {
            continue;
        }
        // level 2
        let (i, _) = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // caps
        if locale != "us" || layout.as_str() != "3l-emacs" {
            xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
            wkb.update_key(CAPS_LOCK, wkb::KeyDirection::Down);
        }

        // Define exception slice for level2_caps test
        let level2_caps_exceptions = &[
            ("am", "eastern", single_key(2)),
            ("am", "eastern", key_range(5, 7)),
            ("am", "eastern-alt", single_key(2)),
            ("am", "eastern-alt", key_range(5, 7)),
            ("am", "western", single_key(2)),
            ("am", "western", key_range(5, 7)),
            ("by", "phonetic", single_key(5)),
            ("by", "phonetic", single_key(7)),
            ("fr", "oss_latin9", single_key(55)),
            ("fr", "bepo_latin9", single_key(55)),
            ("fr", "mac", single_key(83)),
            ("be", "oss_latin9", single_key(55)),
            ("gr", "polytonic", single_key(17)),
            ("gr", "polytonic", single_key(25)),
            ("gr", "polytonic", single_key(31)),
            ("gr", "polytonic", single_key(33)),
            ("gr", "polytonic", single_key(36)),
            ("gr", "polytonic", single_key(37)),
            ("gr", "polytonic", single_key(48)),
            ("iq", "ku_ara", single_key(16)),
            ("iq", "ku_ara", single_key(17)),
            ("ir", "ku_ara", single_key(16)),
            ("ir", "ku_ara", single_key(17)),
            ("in", "iipa", single_key(45)),
            ("in", "iipa", single_key(21)),
            ("kz", "olpc", single_key(43)),
            ("kz", "latin", single_key(47)),
            ("lv", "adapted", single_key(3)),
            ("lv", "adapted", single_key(5)),
            ("lv", "adapted", single_key(52)),
            ("ru", "phonetic", single_key(5)),
            ("ru", "phonetic", single_key(7)),
            ("ru", "phonetic_winkeys", single_key(5)),
            ("ru", "phonetic_winkeys", single_key(7)),
            ("ru", "phonetic_YAZHERTY", single_key(5)),
            ("ru", "phonetic_YAZHERTY", single_key(7)),
            ("ru", "phonetic_dvorak", single_key(5)),
            ("ru", "phonetic_dvorak", single_key(7)),
            (
                "ru",
                "xal",
                multiple_keys(vec![
                    4, 5, 7, 8, 9, 10, 17, 18, 19, 20, 26, 27, 40, 46, 51, 52,
                ]),
            ),
            (
                "ru",
                "bak",
                multiple_keys(vec![3, 4, 5, 6, 8, 9, 13, 41, 43]),
            ),
        ];

        test_all_keys(wkb, xkb, layout, level2_caps_exceptions);
    }
}
