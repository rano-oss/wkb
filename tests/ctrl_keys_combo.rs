#![cfg(feature = "compositor")]
//! Test CTRL modifier combinations match xkbcommon for all keys.
//! wkb may suppress more keys with CTRL than xkbcommon, so wkb returning
//! None is acceptable even when xkbcommon produces a character.

use test_case::test_matrix;
use wkb::KeyDirection;
use xkbcommon::xkb::Keycode;

include!("../test_data/layouts.rs");

mod common;
use common::{update_both, xkb_new_from_names};

const LEFT_CTRL: u32 = 29;
const RIGHT_CTRL: u32 = 97;
const ALT: u32 = 56;
const SHIFT: u32 = 42;

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

/// Test CTRL modifier combinations match xkbcommon for all keys.
/// wkb may suppress more keys with CTRL than xkbcommon, so wkb returning
/// None is acceptable even when xkbcommon produces a character.
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], [0u8, 1, 2, 3])]
fn ctrl_combo(locale: &str, combo: u8) {
    let mods: &[u32] = match combo {
        0 => &[LEFT_CTRL],
        1 => &[RIGHT_CTRL],
        2 => &[LEFT_CTRL, ALT],
        3 => &[LEFT_CTRL, SHIFT],
        _ => unreachable!(),
    };

    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        for &code in mods {
            update_both(&mut wkb, &mut xkb, code, KeyDirection::Down);
        }

        for keycode in 0..701u32 {
            let wkb_char = wkb.key_char(keycode);
            let xkb_char = normalize_xkb_char(xkb.key_get_utf8(Keycode::new(keycode + 8)));

            // wkb may suppress more keys with CTRL than xkbcommon.
            // If wkb returns a character, it should match xkb.
            assert!(
                wkb_char == xkb_char || wkb_char.is_none(),
                "CTRL combo={combo} mismatch for locale={locale} layout={layout} key={keycode}: wkb={wkb_char:?} xkb={xkb_char:?}",
            );
        }

        for &code in mods {
            update_both(&mut wkb, &mut xkb, code, KeyDirection::Up);
        }
    }
}
