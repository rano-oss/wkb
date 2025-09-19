use test_case::test_matrix;
use wkb::{self, modifiers::CAPS_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{test_all_keys_detailed, xkb_new_from_names};

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

        // Define exception logic for this specific test
        let caps_lock_exceptions = |loc: &str, lay: &str, key: u32| -> bool {
            match (loc, lay, key) {
                ("ara", "mac-phonetic", 2..=11) => true,
                // eg has inconsistent use so I go with my common sense
                ("eg", "cop", 16..=26) => true,
                ("eg", "cop", 30..=40) => true,
                ("eg", "cop", 44..=50) => true,
                ("eg", "cop", 53..=53) => true,
                ("us", "3l-emacs", 3..=57) => true,
                ("me", "latinunicodeyz", 45) => true, // TODO
                _ => false,
            }
        };

        test_all_keys_detailed(wkb, xkb, layout, locale, Some(&caps_lock_exceptions));
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
        let i = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // caps
        if locale != "us" || layout.as_str() != "3l-emacs" {
            xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
            wkb.update_key(CAPS_LOCK, wkb::KeyDirection::Down);
        }

        // Define exception logic for level2_caps test
        let level2_caps_exceptions = |loc: &str, lay: &str, key: u32| -> bool {
            match (loc, lay, key) {
                // Some of these layouts were basically bugged out in xkb
                // so we just ignore them here, I checked an armenian keyboard online!
                ("am", "eastern", 2) => true,
                ("am", "eastern", 5..=7) => true,
                ("am", "eastern-alt", 2) => true,
                ("am", "eastern-alt", 5..=7) => true,
                ("am", "western", 2) => true,
                ("am", "western", 5..=7) => true,
                // This testcase are needed as there are bugs in xkb at least from checking existing keyboards
                ("by", "phonetic", 5) => true,
                ("by", "phonetic", 7) => true,
                // This testcase are needed as there are bugs in xkb at least from reading wanted output in the
                // fr files
                ("fr", "oss_latin9", 55) => true,
                ("fr", "bepo_latin9", 55) => true,
                ("fr", "mac", 83) => true,
                ("be", "oss_latin9", 55) => true,
                ("gr", "polytonic", 17) => true,
                ("gr", "polytonic", 25) => true,
                ("gr", "polytonic", 31) => true,
                ("gr", "polytonic", 33) => true,
                ("gr", "polytonic", 36) => true,
                ("gr", "polytonic", 37) => true,
                ("gr", "polytonic", 48) => true,
                ("iq", "ku_ara", 16) => true,
                ("iq", "ku_ara", 17) => true,
                ("ir", "ku_ara", 16) => true,
                ("ir", "ku_ara", 17) => true,
                ("in", "iipa", 45) => true,
                ("in", "iipa", 21) => true,
                ("kz", "olpc", 43) => true,
                ("kz", "latin", 47) => true,
                ("lv", "adapted", 3) => true,
                ("lv", "adapted", 5) => true,
                ("lv", "adapted", 52) => true,
                ("ru", "phonetic", 5) => true,
                ("ru", "phonetic", 7) => true,
                ("ru", "phonetic_winkeys", 5) => true,
                ("ru", "phonetic_winkeys", 7) => true,
                ("ru", "phonetic_YAZHERTY", 5) => true,
                ("ru", "phonetic_YAZHERTY", 7) => true,
                ("ru", "phonetic_dvorak", 5) => true,
                ("ru", "phonetic_dvorak", 7) => true,
                ("ru", "xal", k)
                    if [4, 5, 7, 8, 9, 10, 17, 18, 19, 20, 26, 27, 40, 46, 51, 52].contains(&k) =>
                {
                    true
                }
                ("ru", "bak", k) if [3, 4, 5, 6, 8, 9, 13, 41, 43].contains(&k) => true,
                _ => false,
            }
        };

        test_all_keys_detailed(wkb, xkb, layout, locale, Some(&level2_caps_exceptions));
    }
}
