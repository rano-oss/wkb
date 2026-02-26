use wkb::{self, KeyDirection};
use xkbcommon::xkb::{self, Keycode};

mod common;
use common::xkb_new_from_names;

fn serialized_modifiers(state: &xkb::State) -> (u32, u32, u32, u32) {
    (
        state.serialize_mods(xkb::STATE_MODS_DEPRESSED),
        state.serialize_mods(xkb::STATE_MODS_LATCHED),
        state.serialize_mods(xkb::STATE_MODS_LOCKED),
        state.serialize_layout(xkb::STATE_LAYOUT_EFFECTIVE),
    )
}

fn assert_same_modifiers_state(wkb: &wkb::WKB<wkb::ListComposer>, xkb: &xkb::State, context: &str) {
    let wkb_state = wkb.modifiers_state();
    let xkb_state = serialized_modifiers(xkb);
    assert_eq!(
        wkb_state, xkb_state,
        "modifier state mismatch after {context}: wkb={wkb_state:?}, xkb={xkb_state:?}"
    );
}

fn update_both(
    wkb: &mut wkb::WKB<wkb::ListComposer>,
    xkb: &mut xkb::State,
    evdev_code: u32,
    direction: KeyDirection,
) {
    wkb.update_key(evdev_code, direction);
    xkb.update_key(
        Keycode::new(evdev_code + 8),
        match direction {
            KeyDirection::Down => xkb::KeyDirection::Down,
            KeyDirection::Up => xkb::KeyDirection::Up,
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
