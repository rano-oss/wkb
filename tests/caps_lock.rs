use test_case::test_matrix;
use wkb::{self, modifiers::CAPS_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{key_range, multiple_keys, single_key, test_all_keys_locale, xkb_new_from_names};

#[test]
fn debug_bqn_caps_lock() {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        "bqn",
        "bqn",
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();

    let mut state = xkb::State::new(&keymap);
    let kc_w = Keycode::new(17 + 8); // AD02 = evdev 17
    let kc_shift = Keycode::new(42 + 8);
    let kc_caps = Keycode::new(58 + 8);

    // No mods
    eprintln!(
        "BQN W no-mods: {:?}",
        state.key_get_utf8(kc_w).chars().collect::<Vec<_>>()
    );

    // Caps only
    state.update_key(kc_caps, xkb::KeyDirection::Down);
    eprintln!(
        "BQN W caps:    {:?}",
        state.key_get_utf8(kc_w).chars().collect::<Vec<_>>()
    );

    // Shift + Caps
    state.update_key(kc_shift, xkb::KeyDirection::Down);
    eprintln!(
        "BQN W shift+caps: {:?}",
        state.key_get_utf8(kc_w).chars().collect::<Vec<_>>()
    );
    state.update_key(kc_shift, xkb::KeyDirection::Up);

    // Now test WKB
    let mut wkb_state = wkb::WKB::new_from_names("bqn".to_string(), Some("bqn".to_string()));
    eprintln!("WKB level_keymap len: {}", wkb_state.level_keymap.len());
    eprintln!(
        "WKB caps_lock_table len: {}",
        wkb_state.caps_lock_table.len()
    );

    for (level, map) in wkb_state.caps_lock_table.iter().enumerate() {
        if let Some(&c) = map.get(&17) {
            eprintln!(
                "WKB caps_lock_table[{}][17] = {:?} (U+{:04X})",
                level, c, c as u32
            );
        } else {
            eprintln!("WKB caps_lock_table[{}][17] = None", level);
        }
    }
    for level in 0..wkb_state.level_keymap.len().min(4) {
        if let Some(&c) = wkb_state.level_keymap[level].get(&17) {
            eprintln!(
                "WKB level_keymap[{}][17] = {:?} (U+{:04X})",
                level, c, c as u32
            );
        } else {
            eprintln!("WKB level_keymap[{}][17] = None", level);
        }
    }

    // Test WKB with just caps
    wkb_state.update_key(CAPS_LOCK, wkb::KeyDirection::Down);
    eprintln!("WKB W caps: {:?}", wkb_state.utf8(17));

    // Test WKB with shift+caps
    let (shift_code, _) = wkb_state.modifiers.level2_code().unwrap();
    wkb_state.update_key(shift_code, wkb::KeyDirection::Down);
    eprintln!("WKB W shift+caps: {:?}", wkb_state.utf8(17));
}

#[test]
fn debug_tamilnet_tscii_key51() {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    // Helper: dump the xkb_symbols section from a compiled keymap
    let dump_symbols = |keymap_str: &str, label: &str| {
        let mut in_symbols = false;
        let mut depth: i32 = 0;
        for line in keymap_str.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("xkb_symbols") {
                in_symbols = true;
            }
            if in_symbols {
                depth += line.matches('{').count() as i32;
                depth -= line.matches('}').count() as i32;
                eprintln!("{}: {}", label, line);
                if depth <= 0 && in_symbols {
                    in_symbols = false;
                }
            }
        }
    };

    // ── tamilnet_TSCII ──
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        "in",
        "tamilnet_TSCII",
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    let ks = keymap.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    dump_symbols(&ks, "TSCII");

    let mut state = xkb::State::new(&keymap);
    let kc = Keycode::new(51 + 8);
    eprintln!(
        "TSCII Key51 no-caps: {:?}",
        state.key_get_utf8(kc).chars().collect::<Vec<_>>()
    );
    state.update_key(Keycode::new(58 + 8), xkb::KeyDirection::Down);
    eprintln!(
        "TSCII Key51 caps:    {:?}",
        state.key_get_utf8(kc).chars().collect::<Vec<_>>()
    );

    // ── iipa ──
    let keymap3 = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        "in",
        "iipa",
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    let ks3 = keymap3.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    dump_symbols(&ks3, "IIPA");

    let mut state3 = xkb::State::new(&keymap3);
    let kc3 = Keycode::new(17 + 8);
    eprintln!(
        "IIPA Key17 no-caps: {:?}",
        state3.key_get_utf8(kc3).chars().collect::<Vec<_>>()
    );
    state3.update_key(Keycode::new(58 + 8), xkb::KeyDirection::Down);
    eprintln!(
        "IIPA Key17 caps:    {:?}",
        state3.key_get_utf8(kc3).chars().collect::<Vec<_>>()
    );
}

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
            ("us", "3l-emacs", single_key(98)),
            ("me", "latinunicodeyz", single_key(45)),
        ];

        test_all_keys_locale(wkb, xkb, layout, locale, caps_lock_exceptions);
    }
    // assert!(false);
}

// #[ignore]
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
            ("jp", "", key_range(16, 57)),
            ("apl", "dyalog", key_range(71, 83)),
            ("in", "tamilnet_TSCII", single_key(45)),
            ("ba", "unicode", multiple_keys(vec![16, 17, 45])),
            ("hr", "unicode", multiple_keys(vec![16, 17, 45])),
            ("me", "latinunicode", multiple_keys(vec![16, 17, 45])),
            ("rs", "twoletter", multiple_keys(vec![16, 17, 45])),
            ("rs", "latinunicode", multiple_keys(vec![16, 17, 45])),
        ];

        test_all_keys_locale(wkb, xkb, layout, locale, level2_caps_exceptions);
    }
}
