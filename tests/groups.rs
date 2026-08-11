use wkb::ir::LayoutFile;
use wkb::{ComposeState, KeyDirection, LockFlags, ALTGR, LEFT_SHIFT, RIGHT_SHIFT, WKB};
use xkbcommon::xkb::{self, Keycode};

fn states(options: &str) -> (WKB, xkb::State) {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        "us,de",
        "",
        Some(options.to_string()),
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    let text = keymap.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    let wkb = WKB::new_from_string(&text).unwrap();
    (wkb, xkb::State::new(&keymap))
}

fn update_both(wkb: &mut WKB, state: &mut xkb::State, code: u32, direction: KeyDirection) {
    wkb.update_key(code, direction);
    state.update_key(
        Keycode::new(code + 8),
        match direction {
            KeyDirection::Down => xkb::KeyDirection::Down,
            KeyDirection::Up => xkb::KeyDirection::Up,
        },
    );
}

fn assert_group(wkb: &WKB, state: &xkb::State, context: &str) {
    assert_eq!(
        wkb.active_layout_idx() as u32,
        state.serialize_layout(xkb::STATE_LAYOUT_EFFECTIVE),
        "layout mismatch after {context}"
    );
}

#[test]
fn momentary_group_restores_on_release() {
    let (mut wkb, mut state) = states("grp:switch");

    update_both(&mut wkb, &mut state, ALTGR, KeyDirection::Down);
    assert_group(&wkb, &state, "pressing the group switch");
    assert_eq!(wkb.active_layout_idx(), 1);

    update_both(&mut wkb, &mut state, ALTGR, KeyDirection::Up);
    assert_group(&wkb, &state, "releasing the group switch");
    assert_eq!(wkb.active_layout_idx(), 0);
}

#[test]
fn locked_group_persists_and_toggles() {
    let (mut wkb, mut state) = states("grp:toggle");

    for (direction, expected) in [
        (KeyDirection::Down, 1),
        (KeyDirection::Up, 1),
        (KeyDirection::Down, 0),
        (KeyDirection::Up, 0),
    ] {
        update_both(&mut wkb, &mut state, ALTGR, direction);
        assert_group(&wkb, &state, "updating the locked group key");
        assert_eq!(wkb.active_layout_idx(), expected);
    }
}

#[test]
fn dual_leveled_group_action_uses_the_pressed_level() {
    let (mut wkb, mut state) = states("grp:ctrl_shift_toggle");

    update_both(&mut wkb, &mut state, 29, KeyDirection::Down);
    update_both(&mut wkb, &mut state, 42, KeyDirection::Down);
    assert_group(&wkb, &state, "pressing Ctrl+Shift");

    update_both(&mut wkb, &mut state, 42, KeyDirection::Up);
    update_both(&mut wkb, &mut state, 29, KeyDirection::Up);
    assert_group(&wkb, &state, "releasing Ctrl+Shift");
}

#[test]
fn latched_group_is_consumed_after_one_key() {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let lock_keymap = xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        "us,de",
        "",
        Some("grp:toggle".to_string()),
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    let lock_text = lock_keymap.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    let latch_text = lock_text.replace("LockGroup(group=+1)", "LatchGroup(group=+1)");
    assert_ne!(
        latch_text, lock_text,
        "serialized keymap has no group lock action"
    );

    let latch_keymap = xkb::Keymap::new_from_string(
        &context,
        latch_text.clone(),
        xkb::KEYMAP_FORMAT_TEXT_V1,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    let mut state = xkb::State::new(&latch_keymap);
    let mut wkb = WKB::new_from_string(&latch_text).unwrap();

    update_both(&mut wkb, &mut state, ALTGR, KeyDirection::Down);
    update_both(&mut wkb, &mut state, ALTGR, KeyDirection::Up);
    assert_group(&wkb, &state, "latching the group");
    assert_eq!(wkb.active_layout_idx(), 1);

    let xkb_char = state.key_get_utf8(Keycode::new(38)).chars().last();
    let result = wkb.press_key(30);
    let wkb_char = match result.compose {
        Some(ComposeState::Idle(ch)) => Some(ch),
        other => panic!("expected a direct character, got {other:?}"),
    };
    assert_eq!(wkb_char, xkb_char);

    state.update_key(Keycode::new(38), xkb::KeyDirection::Down);
    assert_group(&wkb, &state, "consuming the latched group");
    assert_eq!(wkb.active_layout_idx(), 0);
}

#[test]
fn shift_tap_switches_zhuyin_and_norwegian_without_changing_shift_hold() {
    let zhuyin = LayoutFile::from_ron_str(include_str!("../ron_layouts/tw.zhuyin.ron")).unwrap();
    let norwegian = WKB::new_from_names("", "", "no", "", None)
        .unwrap()
        .export_layout(0)
        .unwrap();
    let mut wkb = WKB::new_from_layouts(vec![zhuyin, norwegian]).unwrap();
    assert!(wkb.set_group_key(LEFT_SHIFT, 1, LockFlags::TAP));
    assert!(wkb.set_group_key(RIGHT_SHIFT, 1, LockFlags::TAP));

    // An unused Shift release changes group; pressing Shift itself does not.
    wkb.press_key(LEFT_SHIFT);
    assert_eq!(wkb.active_layout_idx(), 0);
    wkb.release_key(LEFT_SHIFT);
    assert_eq!(wkb.active_layout_idx(), 1);

    // Held Shift remains an ordinary Norwegian Shift. Using another key
    // cancels TAP, so releasing Shift does not switch layout.
    wkb.press_key(LEFT_SHIFT);
    assert_eq!(wkb.active_layout_idx(), 1);
    assert_eq!(wkb.key_char(30), Some('A'));
    wkb.press_key(30);
    wkb.release_key(30);
    wkb.release_key(LEFT_SHIFT);
    assert_eq!(wkb.active_layout_idx(), 1);

    wkb.press_key(RIGHT_SHIFT);
    wkb.release_key(RIGHT_SHIFT);
    assert_eq!(wkb.active_layout_idx(), 0);

    // Zhuyin deliberately has one level. Held Shift therefore produces no
    // character, and using a key still cancels the pending group TAP.
    wkb.press_key(LEFT_SHIFT);
    assert_eq!(wkb.key_char(30), None);
    wkb.press_key(30);
    wkb.release_key(30);
    wkb.release_key(LEFT_SHIFT);
    assert_eq!(wkb.active_layout_idx(), 0);
}
