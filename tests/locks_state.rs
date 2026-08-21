//! Lock state-transition tests: toggling caps lock and num lock in sequence
//! on a single key, and keypad keys under lock states.

use test_case::test_matrix;
use wkb::{CAPS_LOCK, NUM_LOCK, WKB};
use xkbcommon::xkb::{self, Keycode};

include!("../test_data/layouts.rs");

mod common;
use common::xkb_new_from_names;

/// Toggle caps lock and num lock in sequence on a single key.
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
        let mut wkb = WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        // Sample key to test (using 'a' key which is typically keycode 38 on evdev)
        let test_key = 38u32;

        // 1. Caps lock only
        wkb.press_key(CAPS_LOCK);
        wkb.release_key(CAPS_LOCK);
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
        wkb.press_key(NUM_LOCK);
        wkb.release_key(NUM_LOCK);
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
        wkb.press_key(CAPS_LOCK);
        wkb.release_key(CAPS_LOCK);
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

/// Num lock effect on keypad keys with and without caps lock.
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn keypad_with_locks(locale: &str) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        // Keypad keys (evdev keycodes): KP_0=82, KP_1=79, KP_2=80, etc.
        let keypad_keys = vec![79, 80, 81, 75, 76, 77, 71, 72, 73, 82, 83, 86, 63, 106];

        // Test keypad keys with num lock only
        wkb.press_key(NUM_LOCK);
        wkb.release_key(NUM_LOCK);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Up);

        for &keycode in &keypad_keys {
            let wkb_char = wkb.key_char(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            if wkb_char != xkb_char && xkb_char.is_some() {
                println!(
                    "locale={} layout={} keypad={} num_only: wkb={:?} xkb={:?}",
                    locale, layout, keycode, wkb_char, xkb_char
                );
            }

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "Keypad mismatch with num lock only"
            );
        }

        // Add caps lock
        wkb.press_key(CAPS_LOCK);
        wkb.release_key(CAPS_LOCK);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Up);

        for &keycode in &keypad_keys {
            let wkb_char = wkb.key_char(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            if wkb_char != xkb_char && xkb_char.is_some() {
                println!(
                    "locale={} layout={} keypad={} caps+num: wkb={:?} xkb={:?}",
                    locale, layout, keycode, wkb_char, xkb_char
                );
            }

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "Keypad mismatch with caps+num lock"
            );
        }
    }
}
