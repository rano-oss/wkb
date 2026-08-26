#![cfg(feature = "compositor")]
//! Shared helpers for integration tests
#![allow(dead_code)]

use wkb::{level_index, ModType, ALTGR, CAPS_LOCK, NUM_LOCK, WKB};
use xkbcommon::xkb::{self, Keycode};

/// Build an xkbcommon `State` for the given locale and layout variant.
/// An empty `layout` string selects the default variant.
pub fn xkb_new_from_names(locale: &str, layout: &str) -> xkb::State {
    let keymap = xkb_new_keymap_from_names(locale, layout);
    xkb::State::new(&keymap)
}

/// Build an xkbcommon `Keymap` for the given locale and layout variant.
/// An empty `layout` string selects the default variant.
pub fn xkb_new_keymap_from_names(locale: &str, layout: &str) -> xkb::Keymap {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        locale,
        layout,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap()
}

/// Apply a key event to both wkb and xkbcommon backends in sync.
pub fn update_both(
    wkb: &mut wkb::WKB,
    xkb: &mut xkb::State,
    evdev_code: u32,
    direction: wkb::KeyDirection,
) {
    match direction {
        wkb::KeyDirection::Down => {
            wkb.press_key(evdev_code);
            xkb.update_key(Keycode::new(evdev_code + 8), xkb::KeyDirection::Down);
        }
        wkb::KeyDirection::Up => {
            wkb.release_key(evdev_code);
            xkb.update_key(Keycode::new(evdev_code + 8), xkb::KeyDirection::Up);
        }
    }
}

/// Compare every key's character output between wkb and xkbcommon.
pub fn test_all_keys(wkb: WKB, xkb: xkb::State, layout: String, locale: &str) {
    for i in 0..701 {
        let k1 = wkb.key_char(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            let level = level_index(
                wkb.active_mod_type(ModType::Level5),
                wkb.active_mod_type(ModType::Level3),
                wkb.active_mod_type(ModType::Level2),
            );
            println!(
                "locale={} layout={} key={} level={}",
                locale, layout, i, level
            );
            println!("  wkb={:?} xkb={:?}", k1, k2.chars().last());
        }
        assert!(k1 == k2.chars().last() || k2.chars().last().is_none());
    }
}

/// Press the modifier keys that reach a given level state on both backends.
pub fn set_level(wkb: &mut WKB, xkb: &mut xkb::State, code: u32, level: Option<u8>) {
    if let Some(level) = level {
        let mut modifiers = Vec::new();
        match level {
            7 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level3).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
            }
            6 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level3).unwrap().0);
            }
            5 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
            }
            4 => {
                modifiers.push(wkb.level_code(ModType::Level5).unwrap().0);
            }
            3 => {
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
                modifiers.push(wkb.level_code(ModType::Level3).unwrap_or((ALTGR, None)).0);
            }
            2 => {
                modifiers.push(wkb.level_code(ModType::Level3).unwrap().0);
            }
            1 => {
                modifiers.push(wkb.level_code(ModType::Level2).unwrap().0);
            }
            _ => {}
        }
        for &mod_code in &modifiers {
            wkb.press_key(mod_code);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
        }
        wkb.press_key(code);
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        for &mod_code in &modifiers {
            wkb.release_key(mod_code);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
        for &mod_code in &modifiers {
            wkb.press_key(mod_code);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
            wkb.release_key(mod_code);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
    } else {
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        wkb.press_key(code);
    }
}

/// Activate the modifier keys that reach a given level state, if available.
pub fn set_modifier_level(wkb: &mut WKB, xkb: &mut xkb::State, level: usize) -> bool {
    match level {
        0 => true,
        1 => {
            if let Some((code, lvl)) = wkb.level_code(ModType::Level2) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        2 => {
            if let Some((code, lvl)) = wkb.level_code(ModType::Level3) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        3 => {
            if let (Some((c3, l3)), Some((c2, l2))) = (
                wkb.level_code(ModType::Level3),
                wkb.level_code(ModType::Level2),
            ) {
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        4 => {
            if let Some((code, lvl)) = wkb.level_code(ModType::Level5) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        5 => {
            if let (Some((c5, l5)), Some((c2, l2))) = (
                wkb.level_code(ModType::Level5),
                wkb.level_code(ModType::Level2),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        6 => {
            if let (Some((c5, l5)), Some((c3, l3))) = (
                wkb.level_code(ModType::Level5),
                wkb.level_code(ModType::Level3),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                true
            } else {
                false
            }
        }
        7 => {
            if let (Some((c5, l5)), Some((c3, l3)), Some((c2, l2))) = (
                wkb.level_code(ModType::Level5),
                wkb.level_code(ModType::Level3),
                wkb.level_code(ModType::Level2),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Hold down lock modifiers (`1` = num lock, `2` = caps lock) on both backends.
pub fn activate_locks(wkb: &mut WKB, xkb: &mut xkb::State, locks: u8) {
    if locks & 1 != 0 {
        wkb.press_key(NUM_LOCK);
        xkb.update_key(Keycode::new(NUM_LOCK + 8), xkb::KeyDirection::Down);
    }
    if locks & 2 != 0 {
        wkb.press_key(CAPS_LOCK);
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
    }
}
