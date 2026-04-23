/// Test new_from_string() - parsing XKB keymap strings
/// This enables WKB to receive keymaps from Wayland compositors
use wkb::{xkb, KeyDirection, WKB};

#[test]
fn test_new_from_string_us() {
    // Get XKB keymap string for US layout
    let keymap_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("setxkbmap -layout us -print 2>/dev/null | xkbcomp -xkb - - 2>/dev/null")
        .output()
        .expect("Failed to generate keymap");

    let keymap_str = String::from_utf8_lossy(&keymap_output.stdout).to_string();

    // Create WKB from string
    let mut wkb_from_string = WKB::new_from_string(keymap_str);

    // Create WKB from names for comparison
    let mut wkb_from_names = WKB::new_from_names("us".to_string(), None);

    // Test basic keys match
    for keycode in [38, 44, 2, 3, 4] {
        let from_string = wkb_from_string.utf8(keycode);
        let from_names = wkb_from_names.utf8(keycode);

        assert_eq!(
            from_string, from_names,
            "Code {} mismatch: from_string={:?} != from_names={:?}",
            keycode, from_string, from_names
        );
    }

    // Test with Shift
    if let Some((shift_code, _)) = xkb::level2_code(&wkb_from_string.modifiers()) {
        wkb_from_string.update_key(shift_code, KeyDirection::Down);
        wkb_from_names.update_key(shift_code, KeyDirection::Down);

        for keycode in [38, 44, 2] {
            let from_string = wkb_from_string.utf8(keycode);
            let from_names = wkb_from_names.utf8(keycode);

            assert_eq!(
                from_string, from_names,
                "Code {} with Shift: from_string={:?} != from_names={:?}",
                keycode, from_string, from_names
            );
        }
    }
}

#[test]
fn test_new_from_string_de() {
    // German layout test
    let keymap_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("setxkbmap -layout de -print 2>/dev/null | xkbcomp -xkb - - 2>/dev/null")
        .output()
        .expect("Failed to generate keymap");

    let keymap_str = String::from_utf8_lossy(&keymap_output.stdout).to_string();

    let mut wkb_from_string = WKB::new_from_string(keymap_str);
    let mut wkb_from_names = WKB::new_from_names("de".to_string(), None);

    // Test y/z swap (German layout characteristic)
    for keycode in [44, 52] {
        // z, y positions
        let from_string = wkb_from_string.utf8(keycode);
        let from_names = wkb_from_names.utf8(keycode);

        assert_eq!(
            from_string, from_names,
            "DE layout code {} mismatch",
            keycode
        );
    }
}

#[test]
fn test_new_from_string_dvorak() {
    // Dvorak variant test
    let keymap_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("setxkbmap -layout us -variant dvorak -print 2>/dev/null | xkbcomp -xkb - - 2>/dev/null")
        .output()
        .expect("Failed to generate keymap");

    let keymap_str = String::from_utf8_lossy(&keymap_output.stdout).to_string();

    let mut wkb_from_string = WKB::new_from_string(keymap_str);
    let mut wkb_from_names = WKB::new_from_names("us".to_string(), Some("dvorak".to_string()));

    // Test several keys
    for keycode in [38, 39, 40, 44] {
        let from_string = wkb_from_string.utf8(keycode);
        let from_names = wkb_from_names.utf8(keycode);

        assert_eq!(from_string, from_names, "Dvorak code {} mismatch", keycode);
    }
}

#[test]
fn test_new_from_string_modifiers() {
    // Test that modifiers are properly detected from string
    let keymap_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("setxkbmap -layout us -print 2>/dev/null | xkbcomp -xkb - - 2>/dev/null")
        .output()
        .expect("Failed to generate keymap");

    let keymap_str = String::from_utf8_lossy(&keymap_output.stdout).to_string();

    let wkb = WKB::new_from_string(keymap_str);

    // Check modifiers exist
    assert!(
        xkb::level2_code(&wkb.modifiers()).is_some(),
        "Level2 (Shift) should be detected"
    );
    assert!(
        xkb::level3_code(&wkb.modifiers()).is_some(),
        "Level3 (AltGr) should be detected"
    );
}

#[test]
fn test_new_from_string_caps_lock() {
    // Test Caps Lock behavior with string-parsed keymap
    let keymap_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("setxkbmap -layout us -print 2>/dev/null | xkbcomp -xkb - - 2>/dev/null")
        .output()
        .expect("Failed to generate keymap");

    let keymap_str = String::from_utf8_lossy(&keymap_output.stdout).to_string();

    let mut wkb = WKB::new_from_string(keymap_str);

    // Test 'a' without caps
    let lowercase = wkb.utf8(38);
    assert_eq!(lowercase, Some('l'), "Expected lowercase 'l'");

    // Press and release Caps Lock
    wkb.update_key(58, KeyDirection::Down);
    wkb.update_key(58, KeyDirection::Up);

    // Test 'a' with caps
    let uppercase = wkb.utf8(38);
    assert_eq!(
        uppercase,
        Some('L'),
        "Expected uppercase 'L' with Caps Lock"
    );
}
