use test_case::test_matrix;
use wkb::{self, WKB};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        &locale,
        &layout.unwrap_or("".to_string()),
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    xkb::State::new(&keymap)
}

fn test_all_keys(wkb: WKB, xkb: xkb::State) {
    for i in 0..701 {
        let k1 = wkb.utf8(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            println!("wkb: {:?}, xkb: {:?} {}", k1, k2.chars().last(), i);
        }
        assert!(k1 == k2.chars().last() || k2.chars().last().is_none());
    }
}

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
fn default(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        test_all_keys(wkb, xkb);
    }
}

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
        // caps
        xkb.update_key(Keycode::new(58 as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(58, wkb::KeyDirection::Down);
        for i in 0..701 {
            let k1 = wkb.utf8(i);
            let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
            if k1 != k2.chars().last() && !k2.is_empty() {
                println!(
                    "{:?} wkb: {:?} {:?}, xkb: {:?}, {}",
                    layout,
                    k1,
                    wkb.level_key(i, 0),
                    k2.chars().last(),
                    i
                );
            }
            match (locale, layout.as_str(), i) {
                ("ara", "mac-phonetic", 2..=11) => assert!(true),
                // eg has inconsistent use so I go with my common sense
                ("eg", "cop", 16..=26) => assert!(true),
                ("eg", "cop", 30..=40) => assert!(true),
                ("eg", "cop", 44..=50) => assert!(true),
                ("eg", "cop", 53..=53) => assert!(true),
                ("us", "3l-emacs", 3..=57) => assert!(true),
                ("me", "latinunicodeyz", 45) => assert!(true), // TODO
                _ => assert!(k1 == k2.chars().last() || k2.chars().last().is_none()),
            }
        }
    }
}

// #[ignore]
#[test_matrix([
    // "af", "al", "am",
    // "ancient", "apl", "ara", "at", "au",
    // "az",
    // "ba", "bd", 
    // "be", 
    // "bg", 
    // "bqn",
    // "br", "brai", "bt", "bw", "by",
    // "ca",
    // "cd",
    // // "ch", "cm", "cn", "cz",
    // "de",
    // // "dk", "dz",
    // // "ee", "eg", "epo", "es", "et", "eu", "fi", "fo",
    // "fr",
    // // "gb",
    // "ge",
    // // "gh", "gn",
    // "gr",
    // // "hr", "hu",
    // // "id",
    "ie", "il", "in", "iq", "ir",
    // "is",
    // "it",
    // // "jp",
    // // "ke", "kg", "kh", "kr",
    // "kz",
    // // "la", "lk", "lt",
    // "lv",
    // // "ma", "md", "me", "mk",
    // "ml",
    // // "mm", "mn", "mt", "mv", "my", "latam", "latin",
    // // "ng",
    // // "nl", "no", "np", "nz", "ph", "pk",
    // "pl",
    // // "pt",
    // "ro",
    // // "rs",
    // "ru",
    // // "se", "si", "sk", "sn", "tg",
    // // "th", "tj", "tm",
    // "tr", "trans",
    // // "tw", "tz",
    // "ua", "us",
    // "uz",
    // "vn",
    // "za",
])]
fn level2_caps(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
        let state_modifiers = wkb.modifiers.evdev_codes();
        // level 2
        let i = state_modifiers[0];
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        // caps
        xkb.update_key(Keycode::new(58 as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(58, wkb::KeyDirection::Down);
        for i in 0..701 {
            let k1 = wkb.utf8(i);
            let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
            if k1 != k2.chars().last() && !k2.is_empty() {
                println!("wkb: {:?}, xkb: {:?}, {}", k1, k2.chars().last(), i);
            }
            match (locale, layout.as_str(), i) {
                // Some of these layouts were basically bugged out in xkb
                // so we just ignore them here, I checked an armenian keyboard online!
                ("am", "eastern", 2) => assert!(true),
                ("am", "eastern", 5..=7) => assert!(true),
                ("am", "eastern-alt", 2) => assert!(true),
                ("am", "eastern-alt", 5..=7) => assert!(true),
                ("am", "western", 2) => assert!(true),
                ("am", "western", 5..=7) => assert!(true),
                // This testcase are needed as there are bugs in xkb at least from checking existing keyboards
                ("by", "phonetic", 5) => assert!(true),
                ("by", "phonetic", 7) => assert!(true),
                // This testcase are needed as there are bugs in xkb at least from reading wanted output in the
                // fr and be files
                ("fr", "oss_latin9", 55) => assert!(true),
                ("fr", "bepo_latin9", 55) => assert!(true),
                ("fr", "mac", 83) => assert!(true),
                ("gr", "polytonic", 17) => assert!(true),
                ("gr", "polytonic", 25) => assert!(true),
                ("gr", "polytonic", 31) => assert!(true),
                ("gr", "polytonic", 33) => assert!(true),
                ("gr", "polytonic", 36) => assert!(true),
                ("gr", "polytonic", 37) => assert!(true),
                ("gr", "polytonic", 48) => assert!(true),
                _ => {} // _ => assert!(k1 == k2.chars().last() || k2.chars().last().is_none()),
            }
        }
    }
    assert!(false);
}

#[ignore]
#[test_matrix([
    "af", "al","am", "ancient","apl", "ara", "at", "au", "az", "ba", "bd","be", "bg", "bqn",
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
        let state_modifiers = wkb.modifiers.evdev_codes();
        // level 2
        let i = state_modifiers[0];
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        for i in 0..701 {
            let k1 = wkb.utf8(i);
            let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
            if k1 != k2.chars().last() && !k2.is_empty() {
                println!("wkb: {:?}, xkb: {:?}, {}", k1, k2.chars().last(), i);
            }
            match (locale, layout.as_str(), i) {
                // Some of these layouts were basically bugged out in xkb
                // so we just ignore them here, I checked an armenian keyboard online!
                ("am", "eastern", 2) => assert!(true),
                ("am", "eastern", 5..=7) => assert!(true),
                ("am", "eastern-alt", 2) => assert!(true),
                ("am", "eastern-alt", 5..=7) => assert!(true),
                ("am", "western", 2) => assert!(true),
                ("am", "western", 5..=7) => assert!(true),
                // This testcase are needed as there are bugs in xkb at least from reading wanted output in the
                // fr and be files
                ("be", "oss_latin9", 55) => assert!(true),
                ("fr", "oss_latin9", 55) => assert!(true),
                ("fr", "bepo_latin9", 55) => assert!(true),
                ("fr", "mac", 83) => assert!(true),
                _ => assert!(k1 == k2.chars().last() || k2.chars().last().is_none()),
            }
        }
    }
}

#[ignore]
#[test_matrix([
    "af", "al", 
    "am", 
    "ancient", 
    "apl", 
    "ara", "at", "au", "az", "ba", "bd", 
    "bg", 
    "be", 
    "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", 
    "no", 
    "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level3(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        println!("{}", layout);
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout));
        let state_modifiers = wkb.modifiers.evdev_codes();
        // level 3
        let i = state_modifiers[2];
        if i == 0 {
            assert!(true)
        }
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        test_all_keys(wkb, xkb);
    }
}
