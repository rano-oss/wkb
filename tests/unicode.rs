//! Test for Unicode keysym support, particularly for keysyms > U+FFFF and
//! multi-byte Unicode characters in XKB symbol definitions.

/// Minimal keymap with raw Unicode character ㄙ (U+3119) in key definition.
fn keymap_with_raw_unicode() -> String {
    // Uses raw ㄙ character — WKB's preprocessor should convert it to U3119
    "xkb_keymap {
    xkb_keycodes {
        minimum = 8;
        maximum = 255;
        <AD01> = 24;
    };
    xkb_types {
        virtual_modifiers LevelThree;
        type \"FOUR_LEVEL\" {
            modifiers = Shift+LevelThree;
            map[Shift] = Level2;
            map[LevelThree] = Level3;
            map[Shift+LevelThree] = Level4;
            level_name[Level1] = \"Base\";
            level_name[Level2] = \"Shift\";
            level_name[Level3] = \"Alt Base\";
            level_name[Level4] = \"Shift Alt\";
        };
    };
    xkb_compat {
    };
    xkb_symbols {
        key <AD01> { [ ㄙ, Q, paragraph, degree ] };
    };
};"
    .to_string()
}

/// Minimal keymap with U3119 keysym (standard XKB notation).
fn keymap_with_u3119() -> String {
    "xkb_keymap {
    xkb_keycodes {
        minimum = 8;
        maximum = 255;
        <AD01> = 24;
    };
    xkb_types {
        virtual_modifiers LevelThree;
        type \"FOUR_LEVEL\" {
            modifiers = Shift+LevelThree;
            map[Shift] = Level2;
            map[LevelThree] = Level3;
            map[Shift+LevelThree] = Level4;
            level_name[Level1] = \"Base\";
            level_name[Level2] = \"Shift\";
            level_name[Level3] = \"Alt Base\";
            level_name[Level4] = \"Shift Alt\";
        };
    };
    xkb_compat {
    };
    xkb_symbols {
        key <AD01> { [ U3119, Q, paragraph, degree ] };
    };
};"
    .to_string()
}

/// WKB handles U3119 keysym notation.
#[test]
fn wkb_handles_u3119() {
    let mut wkb = wkb::WKB::new_from_string(&keymap_with_u3119()).unwrap();
    let evdev_code = 16; // AD01: keycode 24 - 8
    let result = wkb.press_key(evdev_code);
    let ch = match result.compose {
        Some(wkb::ComposeState::Idle(c)) => Some(c),
        _ => None,
    };
    assert_eq!(ch, Some('ㄙ'), "WKB should produce ㄙ (U+3119) for AD01");
}

/// WKB preprocesses raw Unicode characters to UXXXX notation.
#[test]
fn wkb_handles_raw_unicode() {
    let mut wkb = wkb::WKB::new_from_string(&keymap_with_raw_unicode()).unwrap();
    let evdev_code = 16; // AD01: keycode 24 - 8
    let result = wkb.press_key(evdev_code);
    let ch = match result.compose {
        Some(wkb::ComposeState::Idle(c)) => Some(c),
        _ => None,
    };
    assert_eq!(
        ch,
        Some('ㄙ'),
        "WKB should preprocess raw ㄙ and produce U+3119"
    );
}
