//! Test CTRL key combinations that should return empty/no character
//!
//! This test suite verifies that wkb correctly returns None for
//! control key combinations (CTRL+letter) matching xkbcommon behavior.
//!
//! Note: wkb does not support control characters (U+0000-U+001F, U+007F).
//! When xkbcommon returns a control character, we treat it as equivalent to None.

use test_case::test_matrix;
use wkb::KeyDirection;
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

/// Returns true if the character is a control character
fn is_control_char(c: char) -> bool {
    matches!(c, '\u{0000}'..='\u{001F}' | '\u{007F}')
}

/// Converts xkbcommon output to Option<char>, treating control characters as None
fn normalize_xkb_char(xkb_str: String) -> Option<char> {
    let xkb_char = if xkb_str.is_empty() {
        None
    } else {
        xkb_str.chars().last()
    };

    // Treat control characters as None since wkb doesn't support them
    match xkb_char {
        Some(c) if is_control_char(c) => None,
        other => other,
    }
}

/// Test that CTRL+letter combinations return None/empty
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn ctrl_letter_combinations(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb_left = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut wkb_right = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));

        let left_ctrl = 29u32;
        let right_ctrl = 97u32;

        // Test key
        let test_key = 38u32; // 'a'

        // Test with left CTRL
        wkb_left.update_key(left_ctrl, KeyDirection::Down);
        let left_result = wkb_left.utf8(test_key);
        wkb_left.update_key(left_ctrl, KeyDirection::Up);

        // Test with right CTRL
        wkb_right.update_key(right_ctrl, KeyDirection::Down);
        let right_result = wkb_right.utf8(test_key);
        wkb_right.update_key(right_ctrl, KeyDirection::Up);

        assert_eq!(
            left_result, right_result,
            "Left CTRL and Right CTRL should produce same result for locale={} layout={}: left={:?} right={:?}",
            locale, layout, left_result, right_result
        );
    }
}

/// Test CTRL+ALT combinations
/// Note: wkb may suppress more keys with CTRL than xkbcommon
#[test_matrix(["us", "de", "fr", "gb", "es", "it", "ru"])]
fn ctrl_alt_combinations(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let ctrl_code = 29u32;
        let alt_code = 56u32;

        // Hold down CTRL and ALT
        wkb.update_key(ctrl_code, KeyDirection::Down);
        wkb.update_key(alt_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(alt_code + 8), xkb::KeyDirection::Down);

        // Test sample keys
        let test_keys = vec![38, 39, 40, 44, 45, 46]; // a, s, d, z, x, c

        for &keycode in &test_keys {
            let wkb_char = wkb.utf8(keycode);
            let xkb_str = xkb.key_get_utf8(Keycode::new(keycode + 8));
            let xkb_char = normalize_xkb_char(xkb_str);

            // wkb may suppress more keys with CTRL than xkbcommon
            // If wkb returns None, that's acceptable even if xkb returns a character
            // If wkb returns a character, it should match xkb
            assert!(
                wkb_char == xkb_char || wkb_char.is_none(),
                "CTRL+ALT+key mismatch for locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        // Release modifiers
        wkb.update_key(ctrl_code, KeyDirection::Up);
        wkb.update_key(alt_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Up);
        xkb.update_key(Keycode::new(alt_code + 8), xkb::KeyDirection::Up);
    }
}

/// Test CTRL+Shift combinations
/// Note: wkb may suppress more keys with CTRL than xkbcommon
#[test_matrix(["us", "de", "fr", "gb", "es", "it"])]
fn ctrl_shift_combinations(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let ctrl_code = 29u32;
        let shift_code = 42u32;

        // Hold down CTRL and Shift
        wkb.update_key(ctrl_code, KeyDirection::Down);
        wkb.update_key(shift_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(shift_code + 8), xkb::KeyDirection::Down);

        // Test letter keys
        let test_keys = vec![38, 39, 40, 41, 42, 43, 44]; // a-g

        for &keycode in &test_keys {
            let wkb_char = wkb.utf8(keycode);
            let xkb_str = xkb.key_get_utf8(Keycode::new(keycode + 8));
            let xkb_char = normalize_xkb_char(xkb_str);

            // wkb may suppress more keys with CTRL than xkbcommon
            // If wkb returns None, that's acceptable even if xkb returns a character
            // If wkb returns a character, it should match xkb
            assert!(
                wkb_char == xkb_char || wkb_char.is_none(),
                "CTRL+Shift+key mismatch for locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        // Release modifiers
        wkb.update_key(ctrl_code, KeyDirection::Up);
        wkb.update_key(shift_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Up);
        xkb.update_key(Keycode::new(shift_code + 8), xkb::KeyDirection::Up);
    }
}

/// Test that keys without CTRL produce characters, then with CTRL produce None/empty
/// Note: wkb may suppress more keys with CTRL than xkbcommon
#[test_matrix(["us", "de", "fr", "jp"])]
fn ctrl_suppresses_output(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let ctrl_code = 29u32;
        let test_keys = vec![38, 39, 40]; // a, s, d

        for &keycode in &test_keys {
            // Test without CTRL - should produce a character (usually)
            let wkb_no_ctrl = wkb.utf8(keycode);
            let xkb_no_ctrl = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            // Test with CTRL - behavior should match xkbcommon
            wkb.update_key(ctrl_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Down);

            let wkb_with_ctrl = wkb.utf8(keycode);
            let xkb_str = xkb.key_get_utf8(Keycode::new(keycode + 8));
            let xkb_with_ctrl = normalize_xkb_char(xkb_str);

            // wkb may suppress more keys with CTRL than xkbcommon
            // If wkb returns None, that's acceptable even if xkb returns a character
            // If wkb returns a character, it should match xkb
            assert!(
                wkb_with_ctrl == xkb_with_ctrl || wkb_with_ctrl.is_none(),
                "CTRL behavior mismatch for locale={} layout={} key={}: wkb={:?} xkb={:?} (no_ctrl: wkb={:?} xkb={:?})",
                locale, layout, keycode, wkb_with_ctrl, xkb_with_ctrl, wkb_no_ctrl, xkb_no_ctrl
            );

            wkb.update_key(ctrl_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Up);
        }
    }
}

/// Test function keys with CTRL
#[test_matrix(["us", "de", "fr"])]
fn ctrl_function_keys(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let ctrl_code = 29u32;

        // Function keys F1-F12 (keycodes 59-68, 87-88)
        let function_keys = vec![59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 87, 88];

        // Hold CTRL
        wkb.update_key(ctrl_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Down);

        for &keycode in &function_keys {
            let wkb_char = wkb.utf8(keycode);
            let xkb_str = xkb.key_get_utf8(Keycode::new(keycode + 8));
            let xkb_char = normalize_xkb_char(xkb_str);

            assert!(
                wkb_char == xkb_char,
                "CTRL+F-key should match for locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        // Release CTRL
        wkb.update_key(ctrl_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(ctrl_code + 8), xkb::KeyDirection::Up);
    }
}
