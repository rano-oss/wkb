use std::collections::BTreeMap;

use wkb::ir::{LayoutFile, ModAction};
use wkb::{
    ComposeState, GroupChange, GroupKind, KeyDirection, ModType, ALTGR, CAPS_LOCK, LEFT_SHIFT,
    RIGHT_SHIFT, WKB,
};
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

// ── Helpers for the moved unit tests ──────────────────────────────────────

const ALT: u32 = 56;

/// A single-group layout whose key 42 carries `action` and whose key 30 maps
/// to `a`/`A`/`ä` at levels 0/1/2 (plain / Level2 / Level3).
fn layout_with_action(action: ModAction) -> LayoutFile {
    LayoutFile {
        version: wkb::ir::FORMAT_VERSION,
        layout: "test".to_string(),
        repeat_keys: Vec::new(),
        modifiers: vec![(42, vec![(0, action)])],
        keymap: BTreeMap::from([
            (0u8, BTreeMap::from([(30u32, 'a')])),
            (1u8, BTreeMap::from([(30u32, 'A')])),
            (2u8, BTreeMap::from([(30u32, 'ä')])),
        ]),
        num_lock_keys: BTreeMap::new(),
        caps_lock_keymap: BTreeMap::new(),
        caps_num_lock_keys: BTreeMap::new(),
        keysym_map: BTreeMap::new(),
        compose: Vec::new(),
    }
}

/// Like [`layout_with_action`], but also maps key 30 to `A` while Caps is locked.
fn layout_with_caps(action: ModAction) -> LayoutFile {
    let mut file = layout_with_action(action);
    file.caps_lock_keymap = BTreeMap::from([(0u8, BTreeMap::from([(30u32, 'A')]))]);
    file
}

fn tap(wkb: &mut WKB, code: u32) {
    wkb.update_key(code, KeyDirection::Down);
    wkb.update_key(code, KeyDirection::Up);
}

// ── Group/modifier state-machine unit tests, moved out of src ────────────

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
    let zhuyin = LayoutFile::from_ron_str(include_str!("common/tw.zhuyin.ron")).unwrap();
    let norwegian = WKB::new_from_names("", "", "no", "", None)
        .unwrap()
        .export_layout(0)
        .unwrap();
    let mut wkb = WKB::new_from_layouts(vec![zhuyin, norwegian]).unwrap();

    assert!(wkb.set_group_key(LEFT_SHIFT, GroupKind::Tap(GroupChange::Relative(1))));
    assert!(wkb.set_group_key(RIGHT_SHIFT, GroupKind::Tap(GroupChange::Relative(1))));

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

#[test]
fn latch_on_press_activate_on_named_edge() {
    let file = layout_with_action(ModAction::Latch(ModType::Level3)); //OnPress
    let mut wkb = WKB::new_from_layouts(vec![file]).unwrap();

    assert_eq!(wkb.key_char(30), Some('a'), "idle");
    wkb.update_key(42, KeyDirection::Down);
    assert_eq!(wkb.key_char(30), Some('ä'), "level 3 active while held");
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('ä'), "level 3 latched after release");

    // A second press cycle unlatches both variants.
    wkb.update_key(42, KeyDirection::Down);
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('a'), "unlatched");
}

#[test]
fn latch_on_release_activate_on_named_edge() {
    let file = layout_with_action(ModAction::Latch(ModType::Level3));
    let mut wkb = WKB::new_from_layouts(vec![file]).unwrap();

    assert_eq!(wkb.key_char(30), Some('a'), "idle");
    wkb.update_key(42, KeyDirection::Down);
    assert_eq!(wkb.key_char(30), Some('ä'), "level 3 active while held");
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('ä'), "level 3 latched after release");

    // A second press cycle unlatches both variants.
    wkb.update_key(42, KeyDirection::Down);
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('a'), "unlatched");
}

#[test]
fn lock_flags_use_and_combine_their_named_edges() {
    // UNLOCK_ON_PRESS: the first tap locks level 2, the next press clears it.
    let file = layout_with_action(ModAction::UnlockOnPress(ModType::Level2)); //UnlockOnPress
    let mut wkb = WKB::new_from_layouts(vec![file]).unwrap();
    tap(&mut wkb, 42);
    assert_eq!(wkb.key_char(30), Some('A'), "first tap locks");
    wkb.update_key(42, KeyDirection::Down);
    assert_eq!(
        wkb.key_char(30),
        Some('a'),
        "second press unlocks immediately"
    );
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('a'), "stays unlocked after release");

    // LOCK_ON_RELEASE: the lock engages only on release, and holds while the
    // key is held again.
    let file = layout_with_action(ModAction::Lock(ModType::Level2)); //LockOnRelease
    let mut wkb = WKB::new_from_layouts(vec![file]).unwrap();
    assert_eq!(wkb.key_char(30), Some('a'), "idle");
    wkb.update_key(42, KeyDirection::Down);
    assert_eq!(wkb.key_char(30), Some('A'), "active while held");
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('A'), "locks on release");
    wkb.update_key(42, KeyDirection::Down);
    assert_eq!(wkb.key_char(30), Some('A'), "lock holds while re-pressed");
    wkb.update_key(42, KeyDirection::Up);
    assert_eq!(wkb.key_char(30), Some('a'), "release clears the lock");
}

#[test]
fn caps_lock_unlocks_on_press() {
    let file = layout_with_caps(ModAction::UnlockOnPress(ModType::Caps)); //UnlockOnPress
    let mut wkb = WKB::new_from_layouts(vec![file]).unwrap();
    tap(&mut wkb, 42);
    assert_eq!(wkb.key_char(30), Some('A'), "first tap locks caps");
    tap(&mut wkb, 42);
    assert_eq!(wkb.key_char(30), Some('a'), "second press unlocks caps");
}

#[test]
fn alt_shift_toggle_groups_alt_and_shift() {
    let (mut wkb, mut state) = states("grp:alt_shift_toggle");

    // Alt alone (level 0) is `NoSymbol`: the group does not change.
    update_both(&mut wkb, &mut state, ALT, KeyDirection::Down);
    update_both(&mut wkb, &mut state, ALT, KeyDirection::Up);
    assert_group(&wkb, &state, "tapping alt alone");
    assert_eq!(wkb.active_layout_idx(), 0);

    // Alt+Shift (level 1) carries the group action: it toggles to the next group.
    update_both(&mut wkb, &mut state, ALT, KeyDirection::Down);
    update_both(&mut wkb, &mut state, LEFT_SHIFT, KeyDirection::Down);
    assert_group(&wkb, &state, "holding alt+shift");
    update_both(&mut wkb, &mut state, LEFT_SHIFT, KeyDirection::Up);
    update_both(&mut wkb, &mut state, ALT, KeyDirection::Up);
    assert_group(&wkb, &state, "releasing alt+shift");
    assert_eq!(wkb.active_layout_idx(), 1);
}

#[test]
fn caps_toggle_switches_group_at_level_zero() {
    let (mut wkb, mut state) = states("grp:caps_toggle");
    for expected in [1, 0] {
        update_both(&mut wkb, &mut state, CAPS_LOCK, KeyDirection::Down);
        update_both(&mut wkb, &mut state, CAPS_LOCK, KeyDirection::Up);
        assert_group(&wkb, &state, "tapping caps");
        assert_eq!(wkb.active_layout_idx(), expected);
    }
}

#[test]
fn no_group_option_leaves_altgr_a_plain_level_key() {
    let (mut wkb, mut state) = states("");
    update_both(&mut wkb, &mut state, ALTGR, KeyDirection::Down);
    assert_group(&wkb, &state, "holding altgr");
    assert_eq!(wkb.active_layout_idx(), 0);
    update_both(&mut wkb, &mut state, ALTGR, KeyDirection::Up);
    assert_group(&wkb, &state, "releasing altgr");
    assert_eq!(wkb.active_layout_idx(), 0);
}
