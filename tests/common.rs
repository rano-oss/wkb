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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn test_all_keys(
    wkb: WKB,
    xkb: xkb::State,
    layout: String,
    exceptions: &[(&str, &str, KeyRange)],
) {
    let mut wkb = wkb;
    for i in 0..701 {
        let k1 = wkb.utf8(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            // Check if this is an expected exception
            let is_exception = exceptions
                .iter()
                .any(|(_exc_locale, exc_layout, exc_key_range)| {
                    layout.contains(exc_layout) && exc_key_range.contains(i)
                });

            if !is_exception {
                let level = level_index(
                    wkb.modifiers.level5(),
                    wkb.modifiers.level3(),
                    wkb.modifiers.level2(),
                ) as usize;
                println!("{}", layout);
                println!("wkb: {:?}, xkb: {:?} {}", k1, k2.chars().last(), i);
                println!("level: {}", level);
                println!("{}", wkb.modifiers);
                println!("{:?}", wkb.level_keymap[level]);
            }
        }

        // Check if this key should be excepted from assertion
        let should_skip_assert =
            exceptions
                .iter()
                .any(|(_exc_locale, exc_layout, exc_key_range)| {
                    layout.contains(exc_layout) && exc_key_range.contains(i)
                });

        if !should_skip_assert {
            assert!(k1 == k2.chars().last() || k2.chars().last().is_none());
        }
    }
}

/// Represents a range or single key for exceptions
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum KeyRange {
    Single(u32),
    Range(u32, u32), // inclusive range
    Multiple(Vec<u32>),
}

impl KeyRange {
    pub fn contains(&self, key: u32) -> bool {
        match self {
            KeyRange::Single(k) => *k == key,
            KeyRange::Range(start, end) => key >= *start && key <= *end,
            KeyRange::Multiple(keys) => keys.contains(&key),
        }
    }
}

/// Helper function to create a single key exception
#[allow(dead_code)]
pub fn single_key(key: u32) -> KeyRange {
    KeyRange::Single(key)
}

/// Helper function to create a key range exception
#[allow(dead_code)]
pub fn key_range(start: u32, end: u32) -> KeyRange {
    KeyRange::Range(start, end)
}

/// Helper function to create multiple specific keys exception
#[allow(dead_code)]
pub fn multiple_keys(keys: Vec<u32>) -> KeyRange {
    KeyRange::Multiple(keys)
}

pub fn set_level(wkb: &mut WKB, xkb: &mut xkb::State, code: u32, level: Option<u8>) {
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
