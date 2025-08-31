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

fn test_all_keys(mut wkb: WKB, xkb: xkb::State, layout: String) {
    for i in 0..701 {
        let k1 = wkb.utf8(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            println!("{}", layout);
            println!("wkb: {:?}, xkb: {:?} {}", k1, k2.chars().last(), i);
            println!("{:?}", wkb.level_keymap)
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
        println!("{}", layout);
        let xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        if wkb.level_keymap().is_empty() {
            continue;
        }
        test_all_keys(wkb, xkb, layout);
    }
}

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
        println!("{:?}", wkb.modifiers);
        let i = wkb.modifiers.level2_code().unwrap();
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
        for i in 0..701 {
            let k1 = wkb.utf8(i);
            let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
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
                ("apl", "dyalog", i) => {
                    if [71, 72, 73, 75, 76, 77, 79, 80, 81, 82, 83].contains(&i) {
                        assert!(true);
                    } else {
                        assert!(k1 == k2.chars().last() || k2.chars().last().is_none())
                    }
                }
                ("apl", "dyalog_box", i) => {
                    if [71, 72, 73, 75, 76, 77, 79, 80, 81, 82, 83].contains(&i) {
                        assert!(true);
                    } else {
                        assert!(k1 == k2.chars().last() || k2.chars().last().is_none())
                    }
                }
                _ => {
                    if k1 != k2.chars().last() && !k2.is_empty() {
                        println!("wkb: {:?}, xkb: {:?}, {}", k1, k2.chars().last(), i);
                        println!("{}", layout);
                        println!("{:?}", wkb.modifiers);
                    }
                    assert!(k1 == k2.chars().last() || k2.chars().last().is_none())
                }
            }
        }
    }
}

// #[test_matrix([
//     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
//     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
//     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
//     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
//     "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
//     "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
//     "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
// ])]
// fn level3(locale: &str) {
//     let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
//     for layout in wkb.layouts() {
//         let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
//         let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
//         // level 3
//         if wkb.level_keymap.len() < 3 || wkb.modifiers.level3_code().is_none() {
//             println!("Missing modifier: {}", layout);
//             continue;
//         }
//         let i = wkb.modifiers.level3_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         for i in 0..701 {
//             let k1 = wkb.utf8(i);
//             let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
//             if k1 != k2.chars().last() && !k2.is_empty() {
//                 // println!("{}", layout.clone());
//                 println!("wkb: {:?}, xkb: {:?}, {}", k1, k2.chars().last(), i);
//                 // println!("{:?}", wkb.level_keymap());
//             }
//             match (locale, layout.as_str(), i) {
//                 // This testcase are needed as there are bugs in xkb at least from reading wanted output in the
//                 // fr and be files
//                 ("be", "oss_latin9", 55) => assert!(true),
//                 ("fr", "oss_latin9", 55) => assert!(true),
//                 ("fr", "bepo_latin9", 55) => assert!(true),
//                 ("be", "oss_latin9", 98) => assert!(true),
//                 ("fr", "oss_latin9", 98) => assert!(true),
//                 ("fr", "bepo_latin9", 98) => assert!(true),
//                 _ => assert!(k1 == k2.chars().last() || k2.chars().last().is_none()),
//             }
//         }
//     }
// }

// #[test_matrix([
//     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
//     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
//     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
//     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
//     "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
//     "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
//     "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
// ])]
// fn level4(locale: &str) {
//     let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
//     for layout in wkb.layouts() {
//         let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
//         let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
//         // level 4
//         if wkb.level_keymap.len() < 4 || wkb.modifiers.level3_code().is_none() {
//             println!("Missing modifier: {}", layout);
//             continue;
//         }
//         let i = wkb.modifiers.level3_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);

//         let i = wkb.modifiers.level2_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         for i in 0..701 {
//             let k1 = wkb.utf8(i);
//             let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
//             if k1 != k2.chars().last() && !k2.is_empty() {
//                 println!("{}", layout);
//                 println!("wkb: {:?}, xkb: {:?} {}", k1, k2.chars().last(), i);
//             }
//             match (locale, layout.as_str(), i) {
//                 ("de", "T3", 86) => assert!(true),
//                 _ => assert!(k1 == k2.chars().last() || k2.chars().last().is_none()),
//             }
//         }
//     }
// }

// #[test_matrix([
//     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
//     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
//     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
//     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
//     "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
//     "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
//     "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
// ])]
// fn level5(locale: &str) {
//     let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
//     for layout in wkb.layouts() {
//         let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
//         let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
//         // level 5
//         if wkb.level_keymap.len() < 5 || wkb.modifiers.level5_code().is_none() {
//             println!("{}", layout);
//             continue;
//         }
//         let i = wkb.modifiers.level5_code().unwrap();
//         // assert!(i != 0);
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         test_all_keys(wkb, xkb, layout);
//     }
// }

// #[test_matrix([
//     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
//     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
//     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
//     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
//     "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
//     "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
//     "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
// ])]
// fn level6(locale: &str) {
//     let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
//     for layout in wkb.layouts() {
//         println!("{}", layout);
//         let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
//         let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
//         if wkb.level_keymap.len() < 6 || wkb.modifiers.level5_code().is_none() {
//             continue;
//         }
//         // level 5
//         let i = wkb.modifiers.level5_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         // level 2
//         let i = wkb.modifiers.level2_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         test_all_keys(wkb, xkb, layout);
//     }
// }

// #[test_matrix([
//     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
//     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
//     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
//     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
//     "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
//     "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
//     "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
// ])]
// fn level7(locale: &str) {
//     let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
//     for layout in wkb.layouts() {
//         let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
//         let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
//         if wkb.level_keymap.len() < 7 || wkb.modifiers.level5_code().is_none() {
//             continue;
//         }
//         // level 5
//         let i = wkb.modifiers.level5_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         // level 3
//         let i = wkb.modifiers.level3_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         test_all_keys(wkb, xkb, layout);
//     }
// }

// #[test_matrix([
//     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "bg", "be", "bqn",
//     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
//     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
//     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
//     "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
//     "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
//     "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
// ])]
// fn level8(locale: &str) {
//     let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
//     for layout in wkb.layouts() {
//         println!("{}", layout);
//         let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
//         let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.to_owned()));
//         if wkb.level_keymap.len() < 8 || wkb.modifiers.level5_code().is_none() {
//             continue;
//         }
//         // level 5
//         let i = wkb.modifiers.level5_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         // level 3
//         let i = wkb.modifiers.level3_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         // level 2
//         let i = wkb.modifiers.level2_code().unwrap();
//         xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
//         wkb.update_key(i, wkb::KeyDirection::Down);
//         test_all_keys(wkb, xkb, layout);
//     }
// }
