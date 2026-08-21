//! Physical vs logical keys, state-change reporting, and multi-symbol parse errors.

use wkb::{NamedKey, PhysicalKey, StateChanges, CAPS_LOCK, LEFT_SHIFT, NUM_LOCK, WKB};

const KEY_Q: u32 = 16;
const KEY_ESC: u32 = 1;
const KEY_LEFT: u32 = 105;
const KEY_LEFTCTRL: u32 = 29;
const KEY_LEFTALT: u32 = 56;
const KEY_LEFTMETA: u32 = 125;
const MOD_SHIFT: u32 = 1 << 0;
const MOD_CAPS_LOCK: u32 = 1 << 1;

fn us() -> WKB {
    WKB::new_from_names("", "", "us", "", None).unwrap()
}

fn us_fr() -> WKB {
    WKB::new_from_names("", "", "us,fr", "", None).unwrap()
}

fn keymap_caps_escape() -> String {
    r#"xkb_keymap {
    xkb_keycodes {
        minimum = 8;
        maximum = 255;
        <CAPS> = 66;
        <ESC> = 9;
    };
    xkb_types { };
    xkb_compat { };
    xkb_symbols {
        key <CAPS> { [ Escape ] };
        key <ESC> { [ Escape ] };
    };
};"#
    .into()
}

fn keymap_level3_latch() -> String {
    r#"xkb_keymap {
    xkb_keycodes {
        minimum = 8;
        maximum = 255;
        <RALT> = 108;
        <AD01> = 24;
    };
    xkb_types { };
    xkb_compat { };
    xkb_symbols {
        key <RALT> { [ ISO_Level3_Latch ] };
        key <AD01> { [ q, Q ] };
    };
};"#
    .into()
}

fn keymap_multi_symbol(level2: bool) -> String {
    let symbols = if level2 {
        "key <AD01> { [ q, {Q, A} ] };"
    } else {
        "key <AD01> { [ {q, a}, Q ] };"
    };
    format!(
        r#"xkb_keymap {{
    xkb_keycodes {{
        minimum = 8;
        maximum = 255;
        <AD01> = 24;
    }};
    xkb_types {{ }};
    xkb_compat {{ }};
    xkb_symbols {{
        {symbols}
    }};
}};"#
    )
}

#[test]
fn physical_key_stable_across_layouts() {
    let mut wkb = us_fr();
    let us_phys = wkb.physical_key(KEY_Q);
    assert_eq!(us_phys, PhysicalKey::KeyQ);
    wkb.set_layout(1).unwrap();
    assert_eq!(wkb.physical_key(KEY_Q), us_phys);
}

#[test]
fn same_physical_key_different_logical_chars() {
    let mut wkb = us_fr();
    assert_eq!(wkb.key_char(KEY_Q), Some('q'));
    wkb.set_layout(1).unwrap();
    assert_eq!(wkb.key_char(KEY_Q), Some('a'));
    assert_eq!(wkb.physical_key(KEY_Q), PhysicalKey::KeyQ);
}

#[test]
fn caps_remapped_to_escape_keeps_physical_caps() {
    let wkb = WKB::new_from_string(&keymap_caps_escape()).unwrap();
    assert_eq!(wkb.physical_key(CAPS_LOCK), PhysicalKey::CapsLock);
    assert_eq!(
        wkb.named_key(CAPS_LOCK),
        NamedKey::Escape
    );
}

#[test]
fn shift_changes_logical_char_not_physical() {
    let mut wkb = us();
    assert_eq!(wkb.physical_key(KEY_Q), PhysicalKey::KeyQ);
    assert_eq!(wkb.key_char(KEY_Q), Some('q'));
    wkb.press_key(LEFT_SHIFT);
    assert_eq!(wkb.physical_key(KEY_Q), PhysicalKey::KeyQ);
    assert_eq!(wkb.key_char(KEY_Q), Some('Q'));
}

#[test]
fn named_keys_stay_named_under_modifiers() {
    let mut wkb = us();
    let named = [(KEY_ESC, NamedKey::Escape), (KEY_LEFT, NamedKey::ArrowLeft)];
    for (code, expected) in named {
        assert_eq!(wkb.named_key(code), expected);
        for modifier in [KEY_LEFTCTRL, KEY_LEFTALT, KEY_LEFTMETA] {
            wkb.press_key(modifier);
            assert_eq!(
                wkb.named_key(code),
                expected,
                "modifier {modifier} turned {expected:?} unidentified"
            );
            wkb.release_key(modifier);
        }
    }
}

#[test]
fn pressing_modifier_reports_modifiers_updated() {
    let mut wkb = us();
    let result = wkb.press_key(LEFT_SHIFT);
    assert!(result.modifiers_updated);
    assert!(!result.leds_updated);
}

#[test]
fn consuming_latched_modifier_reports_modifiers_updated() {
    let mut wkb = WKB::new_from_string(&keymap_level3_latch()).unwrap();
    let ralt = 100;
    let latch = wkb.press_key(ralt);
    assert!(latch.modifiers_updated);
    wkb.release_key(ralt);
    assert_ne!(wkb.raw_modifiers().latched, 0);

    let result = wkb.press_key(KEY_Q);
    assert!(result.modifiers_updated);
    assert_eq!(wkb.raw_modifiers().latched, 0);
}

#[test]
fn lock_transition_reports_leds_updated_iff_leds_change() {
    let mut wkb = us();
    for _ in 0..2 {
        let before = wkb.leds_state();
        let press = wkb.press_key(CAPS_LOCK);
        let after_press = wkb.leds_state();
        assert_eq!(press.leds_updated, before != after_press);

        let before = wkb.leds_state();
        let release = wkb.release_key(CAPS_LOCK);
        let after_release = wkb.leds_state();
        assert_eq!(release.leds_updated, before != after_release);
    }
}

#[test]
fn update_modifiers_reports_actual_changes() {
    let mut wkb = us_fr();

    let none = wkb.update_modifiers(0, 0, 0, 0);
    assert_eq!(none, StateChanges::default());

    let shift_only = wkb.update_modifiers(MOD_SHIFT, 0, 0, 0);
    assert_eq!(
        shift_only,
        StateChanges {
            is_modifier: false,
            modifiers_updated: true,
            leds_updated: false,
        }
    );

    let same_shift = wkb.update_modifiers(MOD_SHIFT, 0, 0, 0);
    assert_eq!(same_shift, StateChanges::default());

    let caps = wkb.update_modifiers(0, 0, MOD_CAPS_LOCK, 0);
    assert_eq!(
        caps,
        StateChanges {
            is_modifier: false,
            modifiers_updated: true,
            leds_updated: true,
        }
    );
    assert!(wkb.leds_state().caps_lock);

    wkb.update_modifiers(0, 0, 0, 0);
    let group = wkb.update_modifiers(0, 0, 0, 1);
    assert!(group.modifiers_updated);
    assert!(!group.leds_updated);
    assert_eq!(wkb.active_layout_idx(), 1);

    let invalid_group = wkb.update_modifiers(0, 0, 0, 99);
    assert_eq!(invalid_group, StateChanges::default());
}

#[test]
fn release_reports_state_change_during_release() {
    let mut wkb = us();
    wkb.press_key(CAPS_LOCK);
    wkb.release_key(CAPS_LOCK);
    wkb.press_key(CAPS_LOCK);
    assert!(wkb.leds_state().caps_lock);

    let before = wkb.leds_state();
    let release = wkb.release_key(CAPS_LOCK);
    let after = wkb.leds_state();
    assert_eq!(release.leds_updated, before != after);
    if before != after {
        assert!(release.leds_updated);
        assert!(!after.caps_lock);
    }
}

#[test]
fn unknown_evdev_is_unidentified() {
    let wkb = us();
    for code in [0, 9999] {
        assert_eq!(wkb.physical_key(code), PhysicalKey::Unidentified);
        let mut kb = us();
        kb.press_key(code);
        let result = kb.physical_key(code);
        assert_eq!(result, PhysicalKey::Unidentified);
    }
}

#[test]
fn multi_symbol_level_uses_first_symbol() {
    for multi_on_level_2 in [false, true] {
        let wkb =
            WKB::new_from_string(&keymap_multi_symbol(multi_on_level_2)).unwrap();
        assert_eq!(
            wkb.level_char(KEY_Q, 0, 0),
            Some('q'),
        );
        assert_eq!(
            wkb.level_char(KEY_Q, 0, 1),
            Some('q'),
        );
    }
}

#[test]
fn num_lock_led_follows_leds_state() {
    let mut wkb = us();
    let before = wkb.leds_state();
    let press = wkb.press_key(NUM_LOCK);
    assert_eq!(press.leds_updated, before != wkb.leds_state());
}
