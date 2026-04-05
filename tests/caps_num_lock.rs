//! Test caps lock + num lock combinations across all locales and layouts
//!
//! This test suite verifies that wkb handles the combination of caps lock
//! and num lock modifiers the same way as xkbcommon for all keys.

use test_case::test_matrix;
use wkb::{
    modifiers::{CAPS_LOCK, NUM_LOCK},
    KeyDirection,
};
use xkbcommon::xkb::{self, Keycode};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let variant_str = layout.unwrap_or_else(|| String::new());
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        &locale,
        &variant_str,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    xkb::State::new(&keymap)
}

/// Test all keys with both caps lock and num lock active
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn caps_num_lock_keys(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Find the actual caps lock and num lock keys for this layout
        let caps_code = match wkb.caps_lock_key() {
            Some(code) => code,
            None => {
                // This layout doesn't have a caps lock key, skip this test
                continue;
            }
        };
        let num_code = match wkb.num_lock_key() {
            Some(code) => code,
            None => {
                // This layout doesn't have a num lock key, skip this test
                continue;
            }
        };

        // Activate caps lock
        wkb.update_key(caps_code, KeyDirection::Down);
        wkb.update_key(caps_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Up);

        // Activate num lock
        wkb.update_key(num_code, KeyDirection::Down);
        wkb.update_key(num_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(num_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(num_code + 8), xkb::KeyDirection::Up);

        // Test all keys with both modifiers active
        for keycode in 0..701u32 {
            let wkb_char = wkb.utf8(keycode);
            let xkb_str = xkb.key_get_utf8(Keycode::new(keycode + 8));
            let xkb_char = xkb_str.chars().last();

            if wkb_char != xkb_char && xkb_char.is_some() {
                println!(
                    "locale={} layout={} key={} caps+num: wkb={:?} xkb={:?}",
                    locale, layout, keycode, wkb_char, xkb_char
                );
            }

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "Mismatch at locale={} layout={} key={} with caps+num lock: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }
    }
}

/// Test caps lock only, then add num lock, then remove caps lock (num lock only)
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
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Find the actual caps lock and num lock keys for this layout
        let caps_code = match wkb.caps_lock_key() {
            Some(code) => code,
            None => {
                // This layout doesn't have a caps lock key, skip this test
                continue;
            }
        };
        let num_code = match wkb.num_lock_key() {
            Some(code) => code,
            None => {
                // This layout doesn't have a num lock key, skip this test
                continue;
            }
        };

        // Sample key to test (using 'a' key which is typically keycode 38 on evdev)
        let test_key = 38u32;

        // 1. Caps lock only
        wkb.update_key(caps_code, KeyDirection::Down);
        wkb.update_key(caps_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Up);

        let wkb_char1 = wkb.utf8(test_key);
        let xkb_char1 = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();
        assert!(
            wkb_char1 == xkb_char1 || xkb_char1.is_none(),
            "Caps only mismatch: wkb={:?} xkb={:?}",
            wkb_char1,
            xkb_char1
        );

        // 2. Add num lock (both active)
        wkb.update_key(num_code, KeyDirection::Down);
        wkb.update_key(num_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(num_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(num_code + 8), xkb::KeyDirection::Up);

        let wkb_char2 = wkb.utf8(test_key);
        let xkb_char2 = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();
        assert!(
            wkb_char2 == xkb_char2 || xkb_char2.is_none(),
            "Caps+num mismatch: wkb={:?} xkb={:?}",
            wkb_char2,
            xkb_char2
        );

        // 3. Remove caps lock (num only)
        wkb.update_key(caps_code, KeyDirection::Down);
        wkb.update_key(caps_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Up);

        let wkb_char3 = wkb.utf8(test_key);
        let xkb_char3 = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();
        assert!(
            wkb_char3 == xkb_char3 || xkb_char3.is_none(),
            "Num only mismatch: wkb={:?} xkb={:?}",
            wkb_char3,
            xkb_char3
        );
    }
}

/// Test num lock effect on keypad keys with and without caps lock
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
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Find the actual caps lock and num lock keys for this layout
        let caps_code = match wkb.caps_lock_key() {
            Some(code) => code,
            None => {
                // This layout doesn't have a caps lock key, skip this test
                continue;
            }
        };
        let num_code = match wkb.num_lock_key() {
            Some(code) => code,
            None => {
                // This layout doesn't have a num lock key, skip this test
                continue;
            }
        };

        // Keypad keys (evdev keycodes): KP_0=82, KP_1=79, KP_2=80, etc.
        let keypad_keys = vec![79, 80, 81, 75, 76, 77, 71, 72, 73, 82, 83, 86, 63, 106];

        // Test keypad keys with num lock only
        wkb.update_key(num_code, KeyDirection::Down);
        wkb.update_key(num_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(num_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(num_code + 8), xkb::KeyDirection::Up);

        for &keycode in &keypad_keys {
            let wkb_char = wkb.utf8(keycode);
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
        wkb.update_key(caps_code, KeyDirection::Down);
        wkb.update_key(caps_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(caps_code + 8), xkb::KeyDirection::Up);

        for &keycode in &keypad_keys {
            let wkb_char = wkb.utf8(keycode);
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
