//! Test all keys against xkbcommon across level and lock modifiers
//!
//! This test suite verifies that wkb handles level modifiers (shift, altgr,
//! level3, level5) and lock modifiers (none, num lock, caps lock, both)
//! the same way as xkbcommon for all keys and layouts.

use test_case::test_matrix;
use wkb::{level_index, KeyDirection, ModType, ALTGR, CAPS_LOCK, NUM_LOCK, WKB};
use xkbcommon::xkb::{self, Keycode};

include!("../test_data/layouts.rs");

mod common;
use common::xkb_new_from_names;

fn test_all_keys(wkb: WKB, xkb: xkb::State, layout: String, locale: &str) {
    for i in 0..701 {
        let k1 = wkb.key_char(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            let level = level_index(
                wkb.active_mod_type(wkb::ModType::Level5),
                wkb.active_mod_type(wkb::ModType::Level3),
                wkb.active_mod_type(wkb::ModType::Level2),
            );
            println!(
                "locale={} layout={} key={} level={}",
                locale, layout, i, level
            );
            println!("  wkb={:?} xkb={:?}", k1, k2.chars().last());
        }
        assert!(k1 == k2.chars().last() || k2.chars().last().is_none());
    }
}

fn set_level(wkb: &mut WKB, xkb: &mut xkb::State, code: u32, level: Option<u8>) {
    if let Some(level) = level {
        let mut modifiers = Vec::new();
        match level {
            7 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level3).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
            }
            6 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level3).unwrap().0);
            }
            5 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
            }
            4 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
            }
            3 => {
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level3).unwrap_or((ALTGR, None)).0);
            }
            2 => {
                modifiers.push(wkb.level_code(ModType::Level3).unwrap().0);
            }
            1 => {
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
            }
            _ => {}
        }
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
        }
        wkb.update_key(code, KeyDirection::Down);
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
            wkb.update_key(mod_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
    } else {
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        wkb.update_key(code, KeyDirection::Down);
    }
}

fn set_modifier_level(wkb: &mut WKB, xkb: &mut xkb::State, level: usize) -> bool {
    match level {
        0 => true,
        1 => {
            if let Some((code, lvl)) = wkb.level_code(ModType::Level2) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        2 => {
            if let Some((code, lvl)) = wkb.level_code(ModType::Level3) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        3 => {
            if let (Some((c3, l3)), Some((c2, l2))) = (
                wkb.level_code(ModType::Level3),
                wkb.level_code(ModType::Level2),
            ) {
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        4 => {
            if let Some((code, lvl)) = wkb.level_code(ModType::Level5) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        5 => {
            if let (Some((c5, l5)), Some((c2, l2))) = (
                wkb.level_code(ModType::Level5),
                wkb.level_code(ModType::Level2),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        6 => {
            if let (Some((c5, l5)), Some((c3, l3))) = (
                wkb.level_code(ModType::Level5),
                wkb.level_code(ModType::Level3),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                true
            } else {
                false
            }
        }
        7 => {
            if let (Some((c5, l5)), Some((c3, l3)), Some((c2, l2))) = (
                wkb.level_code(ModType::Level5),
                wkb.level_code(ModType::Level3),
                wkb.level_code(ModType::Level2),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Activate lock modifiers (`1` = num lock, `2` = caps lock) held down
fn activate_locks(wkb: &mut WKB, xkb: &mut xkb::State, locks: u8) {
    if locks & 1 != 0 {
        wkb.update_key(NUM_LOCK, KeyDirection::Down);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Down);
    }
    if locks & 2 != 0 {
        wkb.update_key(CAPS_LOCK, KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
    }
}

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], [0usize, 1, 2, 3, 4, 5, 6, 7], [0u8, 1, 2, 3])]
fn locks(locale: &str, level: usize, locks: u8) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut xkb = xkb_new_from_names(locale, &layout);
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        set_modifier_level(&mut wkb, &mut xkb, level);
        activate_locks(&mut wkb, &mut xkb, locks);
        test_all_keys(wkb, xkb, layout, locale);
    }
}

/// Test toggling caps lock and num lock in sequence on a single key
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn caps_then_num_lock_sequence(locale: &str) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        // Sample key to test (using 'a' key which is typically keycode 38 on evdev)
        let test_key = 38u32;

        // 1. Caps lock only
        wkb.update_key(CAPS_LOCK, KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_char1 = wkb.key_char(test_key);
        let xkb_char1 = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();
        assert!(
            wkb_char1 == xkb_char1 || xkb_char1.is_none(),
            "Caps only mismatch: wkb={:?} xkb={:?}",
            wkb_char1,
            xkb_char1
        );

        // 2. Add num lock (both active)
        wkb.update_key(NUM_LOCK, KeyDirection::Down);
        wkb.update_key(NUM_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_char2 = wkb.key_char(test_key);
        let xkb_char2 = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();
        assert!(
            wkb_char2 == xkb_char2 || xkb_char2.is_none(),
            "Caps+num mismatch: wkb={:?} xkb={:?}",
            wkb_char2,
            xkb_char2
        );

        // 3. Remove caps lock (num only)
        wkb.update_key(CAPS_LOCK, KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_char3 = wkb.key_char(test_key);
        let xkb_char3 = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();
        assert!(
            wkb_char3 == xkb_char3 || xkb_char3.is_none(),
            "Num only mismatch: wkb={:?} xkb={:?}",
            wkb_char3,
            xkb_char3
        );
    }
}
