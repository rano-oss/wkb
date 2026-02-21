use test_case::test_matrix;
use wkb;

mod common;
use common::{key_range, set_level, single_key, test_all_keys, xkb_new_from_names};

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
        let xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        if wkb.level_keymap().is_empty() {
            continue;
        }
        test_all_keys(wkb, xkb, layout, &[]);
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
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 2 {
            continue;
        }
        // level 2
        let (code, level) = wkb.modifiers.level2_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);

        test_all_keys(wkb, xkb, layout, &[]);
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
            continue;
        }
        let (code, level) = wkb.modifiers.level3_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);

        test_all_keys(wkb, xkb, layout, &[]);
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
        let (code, level) = wkb.modifiers.level3_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);
        let (code, level) = wkb.modifiers.level2_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);

        test_all_keys(wkb, xkb, layout, &[]);
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
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        // level 5
        if wkb.level_keymap.len() < 5 || wkb.modifiers.level5_code().is_none() {
            continue;
        }
        // println!("{}", wkb.modifiers);
        let (code, level) = wkb.modifiers.level5_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);
        test_all_keys(wkb, xkb, layout, &[]);
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
        let (code, level) = wkb.modifiers.level5_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);
        // level 2
        let (code, level) = wkb.modifiers.level2_code().unwrap();
        set_level(&mut wkb, &mut xkb, code, level);
        test_all_keys(wkb, xkb, layout, &[]);
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
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        if wkb.level_keymap.len() < 7
            || wkb.modifiers.level5_code().is_none()
            || wkb.modifiers.level3_code().is_none()
        {
            continue;
        }
        // level 5
        let (code, level) = wkb.modifiers.level5_code().unwrap();
        set_level(&mut &mut &mut wkb, &mut xkb, code, level);
        // level 3
        let (code, level) = wkb.modifiers.level3_code().unwrap();
        set_level(&mut &mut &mut wkb, &mut xkb, code, level);
        test_all_keys(wkb, xkb, layout, &[]);
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
        if wkb.level_keymap.len() < 8
            || wkb.modifiers.level5_code().is_none()
            || wkb.modifiers.level3_code().is_none()
        {
            continue;
        }
        // level 5
        let (code, level) = wkb.modifiers.level5_code().unwrap();
        set_level(&mut &mut &mut wkb, &mut xkb, code, level);
        // level 3
        let (code, level) = wkb.modifiers.level3_code().unwrap();
        set_level(&mut &mut &mut wkb, &mut xkb, code, level);
        // level 2
        let (code, level) = wkb.modifiers.level2_code().unwrap();
        set_level(&mut &mut &mut wkb, &mut xkb, code, level);
        test_all_keys(wkb, xkb, layout, &[]);
    }
}
