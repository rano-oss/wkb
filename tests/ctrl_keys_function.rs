//! Test function keys with CTRL (wkb should match xkbcommon exactly).

use test_case::test_matrix;
use wkb::KeyDirection;
use xkbcommon::xkb::Keycode;

include!("../test_data/layouts.rs");

mod common;
use common::{update_both, xkb_new_from_names};

const LEFT_CTRL: u32 = 29;

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

/// Test function keys with CTRL (wkb should match xkbcommon exactly)
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn ctrl_function_keys(locale: &str) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        // Function keys F1-F12 (keycodes 59-68, 87-88)
        let function_keys = vec![59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 87, 88];

        update_both(&mut wkb, &mut xkb, LEFT_CTRL, KeyDirection::Down);

        for &keycode in &function_keys {
            let wkb_char = wkb.key_char(keycode);
            let xkb_char = normalize_xkb_char(xkb.key_get_utf8(Keycode::new(keycode + 8)));

            assert_eq!(
                wkb_char, xkb_char,
                "CTRL+F-key should match for locale={locale} layout={layout} key={keycode}: wkb={wkb_char:?} xkb={xkb_char:?}",
            );
        }

        update_both(&mut wkb, &mut xkb, LEFT_CTRL, KeyDirection::Up);
    }
}
