//! Common test utilities shared across all test modules

use wkb::{
    self,
    modifiers::{level_index, ALTGR},
    KeyDirection, WKB,
};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

/// Creates a new XKB state from locale and layout names
pub fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        &locale,
        &layout.unwrap_or("".to_string()),
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    xkb::State::new(&keymap)
}

/// Creates a new XKB keymap from locale and layout names
pub fn xkb_new_keymap_from_names(locale: String, layout: Option<String>) -> xkb::Keymap {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        &locale,
        &layout.unwrap_or("".to_string()),
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap()
}

/// Test all keys with specific exceptions for certain locale/layout/key combinations
///
/// This version allows for specifying exceptions where WKB and XKB are expected to differ.
/// The exceptions are specified as tuples of (locale, layout, key_range_or_single_key).
pub fn test_all_keys<C: wkb::composer::Composer>(wkb: WKB<C>, xkb: xkb::State, layout: String) {
    test_all_keys_locale(wkb, xkb, layout, "");
}

pub fn test_all_keys_locale<C: wkb::composer::Composer>(
    wkb: WKB<C>,
    xkb: xkb::State,
    layout: String,
    locale: &str,
) {
    let mut wkb = wkb;
    for i in 0..701 {
        let k1 = wkb.utf8(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            let level = level_index(
                wkb.modifiers.level5(),
                wkb.modifiers.level3(),
                wkb.modifiers.level2(),
            ) as usize;
            println!(
                "locale={} layout={} key={} level={}",
                locale, layout, i, level
            );
            println!("  wkb={:?} xkb={:?}", k1, k2.chars().last());
            if level < wkb.state_keymap.len() {
                println!("  state[{}]={:?}", level, wkb.state_keymap[level].get(&i));
            }
            if level < wkb.caps_lock_keymap.len() {
                println!(
                    "  caps[{}]={:?}",
                    level,
                    wkb.caps_lock_keymap[level].get(&i)
                );
            }
        }
        assert!(k1 == k2.chars().last() || k2.chars().last().is_none());
    }
}

pub fn set_level<C: wkb::composer::Composer>(
    wkb: &mut WKB<C>,
    xkb: &mut xkb::State,
    code: u32,
    level: Option<u8>,
) {
    if let Some(level) = level {
        let mut modifiers = Vec::new();
        match level {
            7 => {
                modifiers.push(wkb.modifiers.level5_code().unwrap().0);
                modifiers.push(wkb.modifiers.level3_code().unwrap().0);
                modifiers.push(wkb.modifiers.level2_code().unwrap().0);
            }
            6 => {
                modifiers.push(wkb.modifiers.level5_code().unwrap().0);
                modifiers.push(wkb.modifiers.level3_code().unwrap().0);
            }
            5 => {
                modifiers.push(wkb.modifiers.level5_code().unwrap().0);
                modifiers.push(wkb.modifiers.level2_code().unwrap().0);
            }
            4 => {
                modifiers.push(wkb.modifiers.level5_code().unwrap().0);
            }
            3 => {
                modifiers.push(wkb.modifiers.level2_code().unwrap().0);
                modifiers.push(wkb.modifiers.level3_code().unwrap_or((ALTGR, None)).0);
            }
            2 => {
                modifiers.push(wkb.modifiers.level3_code().unwrap().0);
            }
            1 => {
                modifiers.push(wkb.modifiers.level2_code().unwrap().0);
            }
            _ => {}
        }
        // Press modifiers down
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
        }
        // Press code down
        wkb.update_key(code, KeyDirection::Down);
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        // Release modifiers up
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
            wkb.update_key(mod_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
    } else {
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        wkb.update_key(code, wkb::KeyDirection::Down);
    }
}

pub fn set_modifier_level<C: wkb::composer::Composer>(
    wkb: &mut WKB<C>,
    xkb: &mut xkb::State,
    level: usize,
) -> bool {
    match level {
        0 => true,
        1 => {
            if let Some((code, lvl)) = wkb.modifiers.level2_code() {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        2 => {
            if let Some((code, lvl)) = wkb.modifiers.level3_code() {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        3 => {
            if let (Some((c3, l3)), Some((c2, l2))) =
                (wkb.modifiers.level3_code(), wkb.modifiers.level2_code())
            {
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        4 => {
            if let Some((code, lvl)) = wkb.modifiers.level5_code() {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        5 => {
            if let (Some((c5, l5)), Some((c2, l2))) =
                (wkb.modifiers.level5_code(), wkb.modifiers.level2_code())
            {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        6 => {
            if let (Some((c5, l5)), Some((c3, l3))) =
                (wkb.modifiers.level5_code(), wkb.modifiers.level3_code())
            {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                true
            } else {
                false
            }
        }
        7 => {
            if let (Some((c5, l5)), Some((c3, l3)), Some((c2, l2))) = (
                wkb.modifiers.level5_code(),
                wkb.modifiers.level3_code(),
                wkb.modifiers.level2_code(),
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
