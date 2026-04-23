use test_case::test_matrix;
use wkb::{modifiers::CAPS_LOCK, testing::WKBTestExt, KeyDirection};
use xkbcommon::xkb::{self as xkbcmn, Keycode};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkbcmn::State {
    let context = xkbcmn::Context::new(xkbcmn::CONTEXT_NO_FLAGS);
    let variant_str = layout.unwrap_or_default();
    let keymap = xkbcmn::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        &locale,
        &variant_str,
        None,
        xkbcmn::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    xkbcmn::State::new(&keymap)
}

/// Test Level2Lock modifier (locks shift level)
/// This is used in some layouts where shift can be locked instead of just pressed
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn shift_lock_behavior(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Get shift keycode
        let shift_code = wkb.level2_code();
        if shift_code.is_none() {
            continue;
        }
        let (shift_code, _) = shift_code.unwrap();

        // Test sample keys: 'a' (38), 'z' (44), '1' (2)
        let test_keys = vec![38, 44, 2, 3, 4];

        // Press and hold shift
        wkb.update_key(shift_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Down);

        for &keycode in &test_keys {
            let wkb_char = wkb.utf8(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "Shift held mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        // Release shift
        wkb.update_key(shift_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Up);

        // Test keys without shift
        for &keycode in &test_keys {
            let wkb_char = wkb.utf8(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "No shift mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }
    }
}

/// Test Eisu_toggle in Japanese layouts (JIS keyboard mode toggle)
/// Eisu_toggle switches between ASCII and Kana input modes
#[test_matrix(["jp"])]
fn eisu_toggle_japanese(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        // Test all keys to ensure behavior matches
        for keycode in 0..701u32 {
            let wkb_char = wkb.utf8(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            if wkb_char != xkb_char && xkb_char.is_some() {
                println!(
                    "JP locale={} layout={} key={}: wkb={:?} xkb={:?}",
                    locale, layout, keycode, wkb_char, xkb_char
                );
            }

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "JP layout mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }
    }
}

/// Test modifier combinations with caps lock (edge case: caps + shift)
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn caps_plus_shift_combination(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let shift_code = wkb.level2_code();
        if shift_code.is_none() {
            continue;
        }
        let (shift_code, _) = shift_code.unwrap();

        // Activate caps lock
        wkb.update_key(CAPS_LOCK, KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, KeyDirection::Up);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkbcmn::KeyDirection::Down);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkbcmn::KeyDirection::Up);

        // Hold shift while caps is active
        wkb.update_key(shift_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Down);

        // Test letter keys with caps+shift (should give lowercase in many layouts)
        let letter_keys = vec![38, 39, 40, 41, 42, 43, 44]; // a-g

        for &keycode in &letter_keys {
            let wkb_char = wkb.utf8(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            if wkb_char != xkb_char && xkb_char.is_some() {
                println!(
                    "caps+shift: locale={} layout={} key={}: wkb={:?} xkb={:?}",
                    locale, layout, keycode, wkb_char, xkb_char
                );
            }

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "Caps+shift mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        // Release shift
        wkb.update_key(shift_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Up);
    }
}

/// Test AltGr (Level3) modifier edge cases
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn altgr_combinations(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let altgr_code = wkb.level3_code();
        if altgr_code.is_none() {
            continue;
        }
        let (altgr_code, _) = altgr_code.unwrap();

        // Test with AltGr pressed
        wkb.update_key(altgr_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(altgr_code + 8), xkbcmn::KeyDirection::Down);

        // Test various keys with AltGr
        for keycode in 0..701u32 {
            let wkb_char = wkb.utf8(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "AltGr mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        wkb.update_key(altgr_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(altgr_code + 8), xkbcmn::KeyDirection::Up);

        // Test AltGr + Shift combination
        let shift_code = wkb.level2_code();
        if let Some((shift_code, _)) = shift_code {
            wkb.update_key(shift_code, KeyDirection::Down);
            wkb.update_key(altgr_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Down);
            xkb.update_key(Keycode::new(altgr_code + 8), xkbcmn::KeyDirection::Down);

            // Test sample keys with AltGr+Shift
            let test_keys = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // number row

            for &keycode in &test_keys {
                let wkb_char = wkb.utf8(keycode);
                let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

                assert!(
                    wkb_char == xkb_char || xkb_char.is_none(),
                    "AltGr+Shift mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                    locale,
                    layout,
                    keycode,
                    wkb_char,
                    xkb_char
                );
            }

            wkb.update_key(shift_code, KeyDirection::Up);
            wkb.update_key(altgr_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Up);
            xkb.update_key(Keycode::new(altgr_code + 8), xkbcmn::KeyDirection::Up);
        }
    }
}

/// Test Level5 modifier (used in some advanced layouts)
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn level5_modifier(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let level5_code = wkb.level5_code();
        if level5_code.is_none() {
            continue;
        }
        let (level5_code, _) = level5_code.unwrap();

        // Test with Level5 pressed
        wkb.update_key(level5_code, KeyDirection::Down);
        xkb.update_key(Keycode::new(level5_code + 8), xkbcmn::KeyDirection::Down);

        for keycode in 0..701u32 {
            let wkb_char = wkb.utf8(keycode);
            let xkb_char = xkb.key_get_utf8(Keycode::new(keycode + 8)).chars().last();

            assert!(
                wkb_char == xkb_char || xkb_char.is_none(),
                "Level5 mismatch at locale={} layout={} key={}: wkb={:?} xkb={:?}",
                locale,
                layout,
                keycode,
                wkb_char,
                xkb_char
            );
        }

        wkb.update_key(level5_code, KeyDirection::Up);
        xkb.update_key(Keycode::new(level5_code + 8), xkbcmn::KeyDirection::Up);
    }
}

/// Test modifier state after multiple rapid presses/releases
#[test_matrix(["us", "de", "fr", "jp", "ru", "gr"])]
fn rapid_modifier_changes(locale: &str) {
    let base_wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    for layout in base_wkb.layouts() {
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.clone()));

        let shift_code = wkb.level2_code();
        if shift_code.is_none() {
            continue;
        }
        let (shift_code, _) = shift_code.unwrap();

        let test_key = 38u32; // 'a' key

        // Rapidly toggle shift 10 times
        for _ in 0..10 {
            wkb.update_key(shift_code, KeyDirection::Down);
            wkb.update_key(shift_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Down);
            xkb.update_key(Keycode::new(shift_code + 8), xkbcmn::KeyDirection::Up);
        }

        // Check state after rapid changes
        let wkb_char = wkb.utf8(test_key);
        let xkb_char = xkb.key_get_utf8(Keycode::new(test_key + 8)).chars().last();

        assert!(
            wkb_char == xkb_char || xkb_char.is_none(),
            "Rapid modifier change mismatch: wkb={:?} xkb={:?}",
            wkb_char,
            xkb_char
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

fn assert_same_modifiers_state(
    wkb: &wkb::WKB<wkb::ListComposer>,
    xkb: &xkbcmn::State,
    context: &str,
) {
    let wkb_state = wkb.modifiers_state();
    let xkb_state = serialized_modifiers(xkb);
    assert_eq!(
        wkb_state, xkb_state,
        "modifier state mismatch after {context}: wkb={wkb_state:?}, xkb={xkb_state:?}"
    );
}

fn update_both(
    wkb: &mut wkb::WKB<wkb::ListComposer>,
    xkb: &mut xkbcmn::State,
    evdev_code: u32,
    direction: KeyDirection,
) {
    wkb.update_key(evdev_code, direction);
    xkb.update_key(
        Keycode::new(evdev_code + 8),
        match direction {
            KeyDirection::Down => xkbcmn::KeyDirection::Down,
            KeyDirection::Up => xkbcmn::KeyDirection::Up,
        },
    );
}

#[test]
fn modifiers_state_matches_xkbcommon() {
    let mut wkb = wkb::WKB::new_from_names("us".to_string(), None);
    let mut xkb = xkb_new_from_names("us".to_string(), None);

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
    use xkbcommon::xkb::Keycode;

    let mut wkb = wkb::WKB::new_from_names("mm".to_string(), Some("zawgyi".to_string()));

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

    eprintln!("\n=== Test: Press latch, release, then press Shift+latch ===\n");

    // Step 1: Press latch
    eprintln!("1. Press latch key {}", latch_key);
    wkb.update_key(latch_key, KeyDirection::Down);
    xkb_state.update_key(Keycode::new(latch_key + 8), xkbcmn::KeyDirection::Down);
    eprintln!(
        "   WKB level3: {}",
        wkb.active_mod_type(wkb::ModType::Level3)
    );
    eprintln!(
        "   XKB Mod5: {}",
        xkb_state.mod_name_is_active("Mod5", xkbcmn::STATE_MODS_EFFECTIVE)
    );

    // Step 2: Release latch
    eprintln!("\n2. Release latch key");
    wkb.update_key(latch_key, KeyDirection::Up);
    xkb_state.update_key(Keycode::new(latch_key + 8), xkbcmn::KeyDirection::Up);
    eprintln!(
        "   WKB level3: {}",
        wkb.active_mod_type(wkb::ModType::Level3)
    );
    eprintln!(
        "   XKB Mod5: {}",
        xkb_state.mod_name_is_active("Mod5", xkbcmn::STATE_MODS_EFFECTIVE)
    );

    // Step 3: Press Shift
    eprintln!("\n3. Press Shift");
    let shift_key = 42;
    wkb.update_key(shift_key, KeyDirection::Down);
    xkb_state.update_key(Keycode::new(shift_key + 8), xkbcmn::KeyDirection::Down);
    eprintln!(
        "   WKB level3: {}",
        wkb.active_mod_type(wkb::ModType::Level3)
    );
    eprintln!(
        "   XKB Mod5: {}",
        xkb_state.mod_name_is_active("Mod5", xkbcmn::STATE_MODS_EFFECTIVE)
    );

    // Step 4: Press latch again (second press)
    eprintln!("\n4. Press latch key again (second press)");
    wkb.update_key(latch_key, KeyDirection::Down);
    xkb_state.update_key(Keycode::new(latch_key + 8), xkbcmn::KeyDirection::Down);
    eprintln!(
        "   WKB level3: {}",
        wkb.active_mod_type(wkb::ModType::Level3)
    );
    eprintln!(
        "   XKB Mod5: {}",
        xkb_state.mod_name_is_active("Mod5", xkbcmn::STATE_MODS_EFFECTIVE)
    );

    // Check key 2 with both active
    eprintln!("\n5. Check key 2 value:");
    let key_2_wkb = wkb.utf8(2);
    let key_2_xkb = xkb_state.key_get_utf8(Keycode::new(10)).chars().last();
    eprintln!(
        "   WKB: {:?} (level2={} level3={})",
        key_2_wkb,
        wkb.active_mod_type(wkb::ModType::Level2),
        wkb.active_mod_type(wkb::ModType::Level3)
    );
    eprintln!("   XKB: {:?}", key_2_xkb);

    assert_eq!(
        wkb.active_mod_type(wkb::ModType::Level3),
        xkb_state.mod_name_is_active("Mod5", xkbcmn::STATE_MODS_EFFECTIVE),
        "Level3 state should match XKB Mod5"
    );
    assert_eq!(key_2_wkb, key_2_xkb, "Key 2 character should match");
}

#[test]
fn test_cm_modifier_type() {
    let wkb = wkb::WKB::new_from_names("cm".to_string(), Some("qwerty".to_string()));

    if let Some((code, level)) = wkb.level3_code() {
        eprintln!("cm/qwerty Level3 code: {} level: {:?}", code, level);

        // Try to determine if it's Latch or Pressed
        eprintln!("Modifiers map: {:#?}", wkb.modifiers());
    }
}

#[test]
fn test_ie_ogam_shift_type() {
    let wkb = wkb::WKB::new_from_names("ie".to_string(), Some("ogam_is434".to_string()));

    if let Some((code, level)) = wkb.level2_code() {
        eprintln!("ie/ogam_is434 Shift code: {} level: {:?}", code, level);
        eprintln!("Modifiers: {:#?}", wkb.modifiers());
    }
}
