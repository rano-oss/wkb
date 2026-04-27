//! Test LED state functions (Caps Lock, Num Lock, Scroll Lock)
//!
//! This test suite verifies that wkb reports LED states that match
//! xkbcommon exactly, without making assumptions about expected behavior.
//! The tests simply ensure both libraries agree on LED state.

use test_case::test_matrix;
use wkb::testing::{KeyDirection, WKBTestExt, CAPS_LOCK, NUM_LOCK, SCROLL_LOCK};
use xkbcommon::xkb::{self, Keycode};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let variant_str = layout.unwrap_or_default();
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

/// Test Caps Lock LED state
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn caps_lock_led(locale: &str) {
    for layout in wkb::testing::get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Get LED index for caps lock from xkbcommon
        let caps_led_name = "Caps Lock";
        let xkb_keymap = xkb.get_keymap();
        let caps_led_idx = xkb_keymap.led_get_index(caps_led_name);

        // Check initial state matches between wkb and xkbcommon
        let wkb_leds = wkb.leds_state();
        let wkb_caps_on = wkb_leds.caps_lock;

        let xkb_caps_on = xkb.led_index_is_active(caps_led_idx);

        assert_eq!(
            wkb_caps_on, xkb_caps_on,
            "Initial caps LED mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_caps_on, xkb_caps_on
        );

        // Press and release caps lock key
        wkb.update_key(CAPS_LOCK, KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_leds = wkb.leds_state();
        let wkb_caps_on = wkb_leds.caps_lock;
        let xkb_caps_on = xkb.led_index_is_active(caps_led_idx);

        assert_eq!(
            wkb_caps_on, xkb_caps_on,
            "Caps LED after first press mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_caps_on, xkb_caps_on
        );

        // Press and release caps lock key again
        wkb.update_key(CAPS_LOCK, KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_leds = wkb.leds_state();
        let wkb_caps_on = wkb_leds.caps_lock;
        let xkb_caps_on = xkb.led_index_is_active(caps_led_idx);

        assert_eq!(
            wkb_caps_on, xkb_caps_on,
            "Caps LED after second press mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_caps_on, xkb_caps_on
        );
    }
}

/// Test Num Lock LED state
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn num_lock_led(locale: &str) {
    for layout in wkb::testing::get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Get LED index for num lock from xkbcommon
        let num_led_name = "Num Lock";
        let xkb_keymap = xkb.get_keymap();
        let num_led_idx = xkb_keymap.led_get_index(num_led_name);

        // Check initial state matches between wkb and xkbcommon
        let wkb_leds = wkb.leds_state();
        let wkb_num_on = wkb_leds.num_lock;
        let xkb_num_on = xkb.led_index_is_active(num_led_idx);

        assert_eq!(
            wkb_num_on, xkb_num_on,
            "Initial num LED mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_num_on, xkb_num_on
        );

        // Press and release num lock key
        wkb.update_key(NUM_LOCK, KeyDirection::Down);
        wkb.update_key(NUM_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_leds = wkb.leds_state();
        let wkb_num_on = wkb_leds.num_lock;
        let xkb_num_on = xkb.led_index_is_active(num_led_idx);

        assert_eq!(
            wkb_num_on, xkb_num_on,
            "Num LED after first press mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_num_on, xkb_num_on
        );

        // Press and release num lock key again
        wkb.update_key(NUM_LOCK, KeyDirection::Down);
        wkb.update_key(NUM_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_leds = wkb.leds_state();
        let wkb_num_on = wkb_leds.num_lock;
        let xkb_num_on = xkb.led_index_is_active(num_led_idx);

        assert_eq!(
            wkb_num_on, xkb_num_on,
            "Num LED after second press mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_num_on, xkb_num_on
        );
    }
}

/// Test Scroll Lock LED state
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn scroll_lock_led(locale: &str) {
    for layout in wkb::testing::get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Get LED index for scroll lock from xkbcommon
        let scroll_led_name = "Scroll Lock";
        let xkb_keymap = xkb.get_keymap();
        let scroll_led_idx = xkb_keymap.led_get_index(scroll_led_name);

        // Check initial state matches between wkb and xkbcommon
        let wkb_leds = wkb.leds_state();
        let wkb_scroll_on = wkb_leds.scroll_lock;
        let xkb_scroll_on = xkb.led_index_is_active(scroll_led_idx);

        assert_eq!(
            wkb_scroll_on, xkb_scroll_on,
            "Initial scroll LED mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_scroll_on, xkb_scroll_on
        );

        // Press and release scroll lock key
        wkb.update_key(SCROLL_LOCK, KeyDirection::Down);
        wkb.update_key(SCROLL_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(SCROLL_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(SCROLL_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_leds = wkb.leds_state();
        let wkb_scroll_on = wkb_leds.scroll_lock;
        let xkb_scroll_on = xkb.led_index_is_active(scroll_led_idx);

        assert_eq!(
            wkb_scroll_on, xkb_scroll_on,
            "Scroll LED after first press mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_scroll_on, xkb_scroll_on
        );

        // Press and release scroll lock key again
        wkb.update_key(SCROLL_LOCK, KeyDirection::Down);
        wkb.update_key(SCROLL_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(SCROLL_LOCK + 8), xkb::KeyDirection::Down);
        xkb.update_key(Keycode::new(SCROLL_LOCK + 8), xkb::KeyDirection::Up);

        let wkb_leds = wkb.leds_state();
        let wkb_scroll_on = wkb_leds.scroll_lock;
        let xkb_scroll_on = xkb.led_index_is_active(scroll_led_idx);

        assert_eq!(
            wkb_scroll_on, xkb_scroll_on,
            "Scroll LED after second press mismatch for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_scroll_on, xkb_scroll_on
        );
    }
}

/// Test all three lock keys pressed
#[test_matrix(["us", "de", "fr", "gb", "es", "it", "ru", "jp"])]
fn all_locks_pressed(locale: &str) {
    for layout in wkb::testing::get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Get LED indices
        let xkb_keymap = xkb.get_keymap();
        let caps_led_idx = xkb_keymap.led_get_index("Caps Lock");
        let num_led_idx = xkb_keymap.led_get_index("Num Lock");
        let scroll_led_idx = xkb_keymap.led_get_index("Scroll Lock");

        // Press all three lock keys
        for &code in &[CAPS_LOCK, NUM_LOCK, SCROLL_LOCK] {
            wkb.update_key(code, KeyDirection::Down);
            wkb.update_key(code, KeyDirection::Up);
            xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
            xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Up);
        }

        // Compare LED states between wkb and xkbcommon
        let wkb_leds = wkb.leds_state();
        let wkb_caps = wkb_leds.caps_lock;
        let wkb_num = wkb_leds.num_lock;
        let wkb_scroll = wkb_leds.scroll_lock;

        let xkb_caps = xkb.led_index_is_active(caps_led_idx);
        let xkb_num = xkb.led_index_is_active(num_led_idx);
        let xkb_scroll = xkb.led_index_is_active(scroll_led_idx);

        assert_eq!(
            wkb_caps, xkb_caps,
            "Caps LED mismatch after all keys pressed for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_caps, xkb_caps
        );
        assert_eq!(
            wkb_num, xkb_num,
            "Num LED mismatch after all keys pressed for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_num, xkb_num
        );
        assert_eq!(
            wkb_scroll, xkb_scroll,
            "Scroll LED mismatch after all keys pressed for locale={} layout={}: wkb={} xkb={}",
            locale, layout, wkb_scroll, xkb_scroll
        );
    }
}
