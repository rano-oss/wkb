#![cfg(feature = "compositor")]
//! Test LED state functions (Caps Lock, Num Lock, Scroll Lock)
//!
//! This test suite verifies that wkb reports LED states that match
//! xkbcommon exactly, without making assumptions about expected behavior.
//! The tests simply ensure both libraries agree on LED state.

use test_case::test_matrix;
use wkb::{KeyDirection, CAPS_LOCK, NUM_LOCK, SCROLL_LOCK};

include!("../test_data/layouts.rs");

mod common;
use common::{update_both, xkb_new_from_names};

fn wkb_led_state(wkb: &wkb::WKB, lock_key: u32) -> bool {
    let leds = wkb.leds_state();
    match lock_key {
        CAPS_LOCK => leds.caps_lock,
        NUM_LOCK => leds.num_lock,
        SCROLL_LOCK => leds.scroll_lock,
        _ => unreachable!(),
    }
}

/// Test that a lock key toggles its LED on, then off, matching xkbcommon
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], [CAPS_LOCK, NUM_LOCK, SCROLL_LOCK])]
fn lock_led(locale: &str, lock_key: u32) {
    let led_name = match lock_key {
        CAPS_LOCK => "Caps Lock",
        NUM_LOCK => "Num Lock",
        SCROLL_LOCK => "Scroll Lock",
        _ => unreachable!(),
    };

    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        let led_idx = xkb.get_keymap().led_get_index(led_name);

        assert_eq!(
            wkb_led_state(&wkb, lock_key),
            xkb.led_index_is_active(led_idx),
            "Initial {led_name} LED mismatch for locale={locale} layout={layout}"
        );

        // Toggle the lock on
        update_both(&mut wkb, &mut xkb, lock_key, KeyDirection::Down);
        update_both(&mut wkb, &mut xkb, lock_key, KeyDirection::Up);
        assert_eq!(
            wkb_led_state(&wkb, lock_key),
            xkb.led_index_is_active(led_idx),
            "{led_name} LED after first press mismatch for locale={locale} layout={layout}"
        );

        // Toggle the lock off
        update_both(&mut wkb, &mut xkb, lock_key, KeyDirection::Down);
        update_both(&mut wkb, &mut xkb, lock_key, KeyDirection::Up);
        assert_eq!(
            wkb_led_state(&wkb, lock_key),
            xkb.led_index_is_active(led_idx),
            "{led_name} LED after second press mismatch for locale={locale} layout={layout}"
        );
    }
}

/// Test all three lock keys pressed together
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn all_locks_pressed(locale: &str) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        // Get LED indices
        let led_names = ["Caps Lock", "Num Lock", "Scroll Lock"];
        let led_indices: Vec<_> = led_names
            .iter()
            .map(|name| xkb.get_keymap().led_get_index(name))
            .collect();

        // Press all three lock keys
        for &code in &[CAPS_LOCK, NUM_LOCK, SCROLL_LOCK] {
            update_both(&mut wkb, &mut xkb, code, KeyDirection::Down);
            update_both(&mut wkb, &mut xkb, code, KeyDirection::Up);
        }

        // Compare LED states between wkb and xkbcommon
        let wkb_leds = wkb.leds_state();
        let xkb_leds: Vec<bool> = led_indices
            .iter()
            .map(|&idx| xkb.led_index_is_active(idx))
            .collect();

        assert_eq!(
            (wkb_leds.caps_lock, wkb_leds.num_lock, wkb_leds.scroll_lock),
            (xkb_leds[0], xkb_leds[1], xkb_leds[2]),
            "LED mismatch after all keys pressed for locale={locale} layout={layout}"
        );
    }
}
