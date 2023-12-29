// use test_case::test_matrix;
use wkb;
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::Keymap {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        &locale,
        &layout.unwrap_or("".to_string()),
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap()
}

// #[test_matrix(1..701, 0..1)]
// #[ignore]
// fn layout(i: usize, l: usize) {
//     let locale = "no".to_string();
//     let layout = None;
//     let wkb = wkb::WKB::new_from_names(locale.clone(), layout.clone());
//     let xkb = xkb_new_from_names(locale, layout);
//     debug_assert_eq!(
//         wkb.level_key(i as u32, l),
//         xkb.key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, l as u32)
//             .first()
//             .map(|k| k.key_char().unwrap_or_default())
//     );
// }

#[test]
#[ignore]
fn level() {
    let locale = "no".to_string();
    let layout = None;
    let level = 1;
    let wkb = wkb::WKB::new_from_names(locale.clone(), layout.clone());
    let xkb = xkb_new_from_names(locale, layout);
    for i in 1..701 {
        let k1 = wkb.level_key(i as u32, level);
        let mut k2 = xkb
            .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, level as u32)
            .first()
            .map(|k| k.key_char().unwrap_or_default());
        if k2.unwrap_or_default() == '\0' {
            k2 = None;
        }
        if k1 != k2 && k2 != None {
            println!("{:?}, {:?}, {}", k1, k2, i);
            // println!("({}, {:?}),", i, k2.unwrap_or_default());
        }
    }
}

#[test]
#[ignore]
fn level1_all() {
    let lang = [
        // "altwin", //META character
        // "brai", // Do I need to support missing characters cause blind?
        // "capslock",
        // "ctrl",
        // "compose", //spesial compose shit?
        // "empty", //uselsess for testing
        // "eurosign",
        // "group",
        // "inet", //Do I need to support this?
        // "keypad",
        // "kpdl",
        // "level2",
        // "level3",
        // "level5",
        // "nbsp",
        // "olpc",
        // "parens",
        // "pc",
        // "rupeesign",
        // "shift",
        // "srvr_ctrl",
        // "terminate",
        // "typo",
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg",
        "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id",
        "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
        "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
        "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th",
        "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    for locale in lang {
        let layout = None;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), layout.clone());
        let xkb = xkb_new_from_names(locale.to_string(), layout);
        for i in 1..701 {
            let k1 = wkb.level_key(i as u32, 0);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, 0 as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2 != None {
                println!("{:?}, {:?}, {}", k1, k2, i);
                // println!("({}, {:?}),", i, k2.unwrap_or_default());
            }
        }
    }
}

#[test]
// #[ignore]
fn level2_all() {
    let lang = [
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg",
        "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id",
        "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
        "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
        "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th",
        "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    for locale in lang {
        let layout = None;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), layout.clone());
        let xkb = xkb_new_from_names(locale.to_string(), layout);
        for i in 1..701 {
            let k1 = wkb.level_key(i as u32, 1);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, 1 as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2 != None {
                println!("{:?}, {:?}, {}", k1, k2, i);
                // println!("({}, {:?}),", i, k2.unwrap_or_default());
            }
        }
    }
}

#[test]
#[ignore]
fn level3_all() {
    let lang = [
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg",
        "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id",
        "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
        "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
        "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th",
        "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    for locale in lang {
        let layout = None;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), layout.clone());
        let xkb = xkb_new_from_names(locale.to_string(), layout);
        for i in 1..701 {
            let k1 = wkb.level_key(i as u32, 2);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, 2 as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2 != None {
                println!("{:?}, {:?}, {}", k1, k2, i);
                // println!("({}, {:?}),", i, k2.unwrap_or_default());
            }
        }
    }
}

#[test]
#[ignore]
fn level4_all() {
    let lang = [
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg",
        "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id",
        "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
        "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
        "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th",
        "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    for locale in lang {
        let layout = None;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), layout.clone());
        let xkb = xkb_new_from_names(locale.to_string(), layout);
        for i in 1..701 {
            let k1 = wkb.level_key(i as u32, 3);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, 3 as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2 != None {
                println!("{:?}, {:?}, {}", k1, k2, i);
                // println!("({}, {:?}),", i, k2.unwrap_or_default());
            }
        }
    }
}

#[test]
#[ignore]
fn level5_all() {
    let lang = [
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg",
        "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id",
        "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
        "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
        "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th",
        "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    for locale in lang {
        let layout = None;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), layout.clone());
        let xkb = xkb_new_from_names(locale.to_string(), layout);
        for i in 1..701 {
            let k1 = wkb.level_key(i as u32, 4);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, 4 as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2 != None {
                println!("{:?}, {:?}, {}", k1, k2, i);
                // println!("({}, {:?}),", i, k2.unwrap_or_default());
            }
        }
    }
}

#[test]
#[ignore]
fn level6_all() {
    let lang = [
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg",
        "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id",
        "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
        "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin",
        "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th",
        "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    for locale in lang {
        let layout = None;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), layout.clone());
        let xkb = xkb_new_from_names(locale.to_string(), layout);
        for i in 1..701 {
            let k1 = wkb.level_key(i as u32, 5);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, 5 as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2 != None {
                println!("{:?}, {:?}, {}", k1, k2, i);
                // println!("({}, {:?}),", i, k2.unwrap_or_default());
            }
        }
    }
}
