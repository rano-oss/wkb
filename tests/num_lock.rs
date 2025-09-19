use test_case::test_matrix;
use wkb::{self, modifiers::NUM_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{test_all_keys, test_all_keys_detailed, xkb_new_from_names};

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn default(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);
        if wkb.level_keymap().is_empty() {
            continue;
        }
        test_all_keys(wkb, xkb, layout);
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd","be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la",
    "lk", "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
    "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru",
    "se", "tg", "th", "tj", "tm", "tr",
    "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level2(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 2 {
            continue;
        }
        // level 2
        let i = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);

        // Define exception logic for level2 test
        let level2_exceptions = |loc: &str, lay: &str, key: u32| -> bool {
            match (loc, lay, key) {
                // Some of these layouts were basically bugged out in xkb
                // so we just ignore them here, I checked an armenian keyboard online!
                ("am", "eastern", 2) => true,
                ("am", "eastern", 5..=7) => true,
                ("am", "eastern-alt", 2) => true,
                ("am", "eastern-alt", 5..=7) => true,
                ("am", "western", 2) => true,
                ("am", "western", 5..=7) => true,
                // This testcase are needed as there are bugs in xkb at least from reading wanted output in the
                // fr and be files
                ("be", "oss_latin9", 55) => true,
                ("fr", "oss_latin9", 55) => true,
                ("fr", "bepo_latin9", 55) => true,
                ("fr", "mac", 83) => true,
                ("apl", "dyalog", k) => [71, 72, 73, 75, 76, 77, 79, 80, 81, 82, 83].contains(&k),
                ("apl", "dyalog_box", k) => {
                    [71, 72, 73, 75, 76, 77, 79, 80, 81, 82, 83].contains(&k)
                }
                ("cm", "azerty", k) => [2, 3, 4, 5, 6, 7, 8, 9, 10, 11].contains(&k),
                _ => false,
            }
        };

        test_all_keys_detailed(wkb, xkb, layout, locale, Some(&level2_exceptions));
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level3(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        // level 3
        if wkb.level_keymap.len() < 3 || wkb.modifiers.level3_code().is_none() {
            println!("{}", layout);
            continue;
        }
        let i = wkb.modifiers.level3_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);

        // Define exception logic for level3 test
        let level3_exceptions = |loc: &str, lay: &str, key: u32| -> bool {
            match (loc, lay, key) {
                // This testcase are needed as there are bugs in xkb at least from reading wanted output in the
                // fr and be files
                ("be", "oss_latin9", 55) => true,
                ("fr", "oss_latin9", 55) => true,
                ("fr", "bepo_latin9", 55) => true,
                ("be", "oss_latin9", 98) => true,
                ("fr", "oss_latin9", 98) => true,
                ("fr", "bepo_latin9", 98) => true,
                _ => false,
            }
        };

        test_all_keys_detailed(wkb, xkb, layout, locale, Some(&level3_exceptions));
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level4(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        // level 4
        if wkb.level_keymap.len() < 4 || wkb.modifiers.level3_code().is_none() {
            continue;
        }
        let i = wkb.modifiers.level3_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);

        let i = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);

        // Define exception logic for level4 test
        let level4_exceptions = |loc: &str, lay: &str, key: u32| -> bool {
            match (loc, lay, key) {
                ("de", "T3", 86) => true,
                _ => false,
            }
        };

        test_all_keys_detailed(wkb, xkb, layout, locale, Some(&level4_exceptions));
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level5(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        // println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        // level 5
        if wkb.level_keymap.len() < 5 || wkb.modifiers.level5_code().is_none() {
            continue;
        }
        let i = wkb.modifiers.level5_code().unwrap();
        // assert!(i != 0);
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);
        test_all_keys(wkb, xkb, layout);
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level6(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 6 || wkb.modifiers.level5_code().is_none() {
            continue;
        }
        // level 5
        let i = wkb.modifiers.level5_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // level 2
        let i = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);
        test_all_keys(wkb, xkb, layout);
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level7(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 7 || wkb.modifiers.level5_code().is_none() {
            continue;
        }
        // level 5
        let i = wkb.modifiers.level5_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // level 3
        let i = wkb.modifiers.level3_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);
        test_all_keys(wkb, xkb, layout);
    }
}

// #[ignore]
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level8(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 8 || wkb.modifiers.level5_code().is_none() {
            continue;
        }
        // level 5
        let i = wkb.modifiers.level5_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // level 3
        let i = wkb.modifiers.level3_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // level 2
        let i = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // NumLock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);
        test_all_keys(wkb, xkb, layout);
    }
}
