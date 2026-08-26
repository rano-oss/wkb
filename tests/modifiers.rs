#![cfg(feature = "compositor")]
use test_case::test_matrix;
use wkb::{KeyDirection, ModType};
use xkbcommon::xkb::{self as xkbcmn, Keycode};

include!("../test_data/layouts.rs");

mod common;
use common::{update_both, xkb_new_from_names};

/// Test modifier state after multiple rapid presses/releases
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn rapid_modifier_changes(locale: &str) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        let mut xkb = xkb_new_from_names(locale, &layout);

        let shift_code = wkb.level_code(ModType::Level2);
        if shift_code.is_none() {
            continue;
        }
        let (shift_code, _) = shift_code.unwrap();

        let test_key = 38u32; // 'a' key

        // Rapidly toggle shift 10 times
        for _ in 0..10 {
            update_both(&mut wkb, &mut xkb, shift_code, KeyDirection::Down);
            update_both(&mut wkb, &mut xkb, shift_code, KeyDirection::Up);
        }

        // Check state after rapid changes
        let wkb_char = wkb.key_char(test_key);
        let xkb_char = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();

        assert!(
            wkb_char == xkb_char || wkb_char.is_none(),
            "Rapid modifier change mismatch: wkb={wkb_char:?} xkb={xkb_char:?}",
        );
    }
}

fn serialized_modifiers(state: &xkbcmn::State) -> (u32, u32, u32, u32) {
    (
        state.serialize_mods(xkbcmn::STATE_MODS_DEPRESSED),
        state.serialize_mods(xkbcmn::STATE_MODS_LATCHED),
        state.serialize_mods(xkbcmn::STATE_MODS_LOCKED),
        state.serialize_layout(xkbcmn::STATE_LAYOUT_EFFECTIVE),
    )
}

fn assert_same_modifiers_state(wkb: &wkb::WKB, xkb: &xkbcmn::State, context: &str) {
    let rm = wkb.raw_modifiers();
    let wkb_state = (rm.depressed, rm.latched, rm.locked, rm.layout);
    let xkb_state = serialized_modifiers(xkb);
    assert_eq!(
        wkb_state, xkb_state,
        "modifier state mismatch after {context}: wkb={wkb_state:?}, xkb={xkb_state:?}"
    );
}

#[test]
fn modifiers_state_matches_xkbcommon() {
    let mut wkb = wkb::WKB::new_from_names("", "", "us", "", None).unwrap();
    let mut xkb = xkb_new_from_names("us", "");

    assert_same_modifiers_state(&wkb, &xkb, "initial state");

    update_both(&mut wkb, &mut xkb, 42, KeyDirection::Down);
    assert_same_modifiers_state(&wkb, &xkb, "pressing left shift");

    update_both(&mut wkb, &mut xkb, 29, KeyDirection::Down);
    assert_same_modifiers_state(&wkb, &xkb, "pressing left ctrl while holding shift");

    update_both(&mut wkb, &mut xkb, 29, KeyDirection::Up);
    assert_same_modifiers_state(&wkb, &xkb, "releasing left ctrl");

    update_both(&mut wkb, &mut xkb, 42, KeyDirection::Up);
    assert_same_modifiers_state(&wkb, &xkb, "releasing left shift");

    update_both(&mut wkb, &mut xkb, 58, KeyDirection::Down);
    assert_same_modifiers_state(&wkb, &xkb, "pressing caps lock");

    update_both(&mut wkb, &mut xkb, 58, KeyDirection::Up);
    assert_same_modifiers_state(&wkb, &xkb, "releasing caps lock");

    update_both(&mut wkb, &mut xkb, 69, KeyDirection::Down);
    assert_same_modifiers_state(&wkb, &xkb, "pressing num lock");

    update_both(&mut wkb, &mut xkb, 69, KeyDirection::Up);
    assert_same_modifiers_state(&wkb, &xkb, "releasing num lock");
}

#[test]
fn test_mm_zawgyi_latch_sequence() {
    let mut wkb = wkb::WKB::new_from_names("", "", "mm", "zawgyi", None).unwrap();

    let context = xkbcmn::Context::new(xkbcmn::CONTEXT_NO_FLAGS);
    let keymap = xkbcmn::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        "mm",
        "zawgyi",
        None,
        xkbcmn::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    let mut xkb_state = xkbcmn::State::new(&keymap);

    let latch_key = 41; // TLDE key

    // Step 1: Press latch
    wkb.press_key(latch_key);
    xkb_state.update_key(Keycode::new(latch_key + 8), xkbcmn::KeyDirection::Down);

    // Step 2: Release latch
    wkb.release_key(latch_key);
    xkb_state.update_key(Keycode::new(latch_key + 8), xkbcmn::KeyDirection::Up);

    // Step 3: Press Shift
    let shift_key = 42;
    wkb.press_key(shift_key);
    xkb_state.update_key(Keycode::new(shift_key + 8), xkbcmn::KeyDirection::Down);

    // Step 4: Press latch again (second press)
    wkb.press_key(latch_key);
    xkb_state.update_key(Keycode::new(latch_key + 8), xkbcmn::KeyDirection::Down);

    // Check key 2 with both active
    let key_2_wkb = wkb.key_char(2);
    let key_2_xkb = xkb_state.key_get_utf8(Keycode::new(10)).chars().last();

    assert_eq!(
        wkb.active_mod_type(wkb::ModType::Level3),
        xkb_state.mod_name_is_active("Mod5", xkbcmn::STATE_MODS_EFFECTIVE),
        "Level3 state should match XKB Mod5"
    );
    assert_eq!(key_2_wkb, key_2_xkb, "Key 2 character should match");
}

#[test]
fn test_cm_modifier_type() {
    let wkb = wkb::WKB::new_from_names("", "", "cm", "qwerty", None).unwrap();
    assert!(
        wkb.level_code(ModType::Level3).is_some(),
        "cm/qwerty should define a Level3 modifier"
    );
}

#[test]
fn test_ie_ogam_shift_type() {
    let wkb = wkb::WKB::new_from_names("", "", "ie", "ogam_is434", None).unwrap();
    assert!(
        wkb.level_code(ModType::Level2).is_some(),
        "ie/ogam_is434 should define a Shift (Level2) modifier"
    );
}
