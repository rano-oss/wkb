//! XKB (X Keyboard Extension) support using rust-xkb
//!
//! This module contains the libxkbcommon functionality transpiled from C to Rust.
//! It provides complete XKB keymap compilation and state management for Wayland.
// Core XKB modules
pub mod atom;
pub mod context;
pub mod features;
pub mod keymap;
pub mod shared_ast_types;
pub mod shared_types;
pub mod state;

// Keysym modules
pub mod keysym;
pub mod keysym_case_mappings;
pub mod keysym_utf;

// Parsing and text processing
pub mod scanner_utils;
pub mod text;

// Utilities
pub mod utf8_decoding;
pub mod utils;

// Compilation modules
pub mod xkbcomp; // XKB compiler with all submodules

// Compose support
pub mod compose;
/// Backward-compatible alias (tests reference the old module name)
pub use compose as compose_parse;

// Rules and registry
pub mod registry;

// Messages and logging
pub mod messages;

// Rust-native wrapper types
pub mod rust_types;

// WKB integration functions
use crate::modifiers::*;
use crate::{ListComposer, WKB};
use std::collections::{BTreeMap, HashSet};

/// Path to XKB symbols directory
pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

/// Get all available layouts/variants for a given locale
fn get_all_layouts_for_locale(locale: &str) -> Vec<String> {
    use rust_types::RxkbContext;

    // Create registry context
    let mut ctx = match RxkbContext::new() {
        Some(ctx) => ctx,
        None => return vec![String::new()], // Failed to create context, return default
    };

    // Load default paths
    ctx.include_path_append_default();

    // Parse the registry for evdev ruleset
    if !ctx.parse("evdev") {
        return vec![String::new()]; // Parse failed, return default
    }

    let mut layouts = Vec::new();

    // Iterate through all layouts
    for layout in ctx.layouts() {
        let layout_name = layout.name();
        if !layout_name.is_empty() && layout_name == locale {
            let variant = layout.variant();
            if variant.is_empty() {
                // Base layout (no variant) - store empty string
                layouts.push(String::new());
            } else {
                // Variant layout - store the variant name
                layouts.push(variant.to_string());
            }
        }
    }

    // Context is automatically cleaned up via Drop

    // If we didn't find any layouts, return empty string as default (base layout)
    if layouts.is_empty() {
        layouts.push(String::new());
    }

    layouts
}

/// Get the keycode (and optional level) for a specific modifier type from the Modifiers state.
/// Returns the first pressed modifier of the given type, or if none are pressed,
/// the first latched/locked modifier of that type.
pub fn level_code(modifiers: &Modifiers, mod_type: ModType) -> Option<(u32, Option<u8>)> {
    let mut other_mod = None;

    for (code, modifier) in modifiers.0.iter() {
        match modifier {
            Modifier::Single(mod_kind) => {
                if mod_kind.get_modkind_from_modtype(mod_type).is_some() {
                    // Prefer Pressed over Latch/Lock
                    match mod_kind {
                        ModKind::Pressed { .. } => return Some((*code, None)),

                        _ => {
                            if other_mod.is_none() {
                                other_mod = Some((*code, None));
                            }
                        }
                    }
                }
            }
            Modifier::Leveled(map) => {
                for (level, mod_kind) in map {
                    if mod_kind.get_modkind_from_modtype(mod_type).is_some() {
                        match mod_kind {
                            ModKind::Pressed { .. } => return Some((*code, Some(*level))),
                            _ => {
                                if other_mod.is_none() {
                                    other_mod = Some((*code, Some(*level)));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    other_mod
}

/// Get the keycode (and optional level) for Level2 (Shift) modifier.
pub fn level2_code(modifiers: &Modifiers) -> Option<(u32, Option<u8>)> {
    level_code(modifiers, ModType::Level2)
}

/// Get the keycode (and optional level) for Level3 (AltGr) modifier.
pub fn level3_code(modifiers: &Modifiers) -> Option<(u32, Option<u8>)> {
    level_code(modifiers, ModType::Level3)
}

/// Get the keycode (and optional level) for Level5 modifier.
pub fn level5_code(modifiers: &Modifiers) -> Option<(u32, Option<u8>)> {
    level_code(modifiers, ModType::Level5)
}

/// Press the appropriate level modifier keys on `state` for the given level index.
/// Level 0 = no modifiers, 1 = Shift, 2 = Level3, 3 = Level3+Shift, etc.
fn press_level_modifiers(
    state: &mut rust_types::State,
    level: usize,
    level2: Option<u32>,
    level3: Option<u32>,
    level5: Option<u32>,
) {
    // Level modifiers form a 3-bit field: bit0=Level2(Shift), bit1=Level3, bit2=Level5
    if level & 4 != 0 {
        if let Some(kc) = level5 {
            state.update_key(kc, shared_types::XKB_KEY_DOWN);
        }
    }
    if level & 2 != 0 {
        if let Some(kc) = level3 {
            state.update_key(kc, shared_types::XKB_KEY_DOWN);
        }
    }
    if level & 1 != 0 {
        if let Some(kc) = level2 {
            state.update_key(kc, shared_types::XKB_KEY_DOWN);
        }
    }
}

/// Build WKB instance from an XKB keymap pointer
fn build_wkb_from_keymap(
    keymap: &rust_types::Keymap,
    locale: Option<String>,
    layout: Option<String>,
    all_layouts: Vec<String>,
) -> WKB<ListComposer> {
    const XKB_MAX_LEVELS: usize = 8;
    const EVDEV_OFFSET: u32 = 8;

    let (min_keycode, max_keycode) = (keymap.min_keycode(), keymap.max_keycode());
    let modifiers = build_modifiers_from_keymap(keymap, min_keycode, max_keycode);

    // Helper: get char from keycode at level
    let get_char = |kc: u32, state: &rust_types::State, lvl: usize| -> Option<char> {
        state.key_get_utf8(kc).chars().next().or_else(|| {
            keymap
                .key_get_syms_by_level(kc, 0, lvl as u32)
                .first()
                .and_then(|&s| keysym_utf::keysym_to_char(s))
        })
    };

    // Helper: populate keymap with lock modifier
    let populate_lock = |lock_kc: Option<u32>,
                         toggle: bool,
                         level_keys: (Option<u32>, Option<u32>, Option<u32>)|
     -> Vec<BTreeMap<u32, char>> {
        let mut maps = vec![BTreeMap::new(); XKB_MAX_LEVELS];
        if let Some(lkc) = lock_kc {
            for (lvl, map) in maps.iter_mut().enumerate().take(XKB_MAX_LEVELS) {
                if let Some(mut st) = keymap.new_state() {
                    if toggle {
                        st.update_key(lkc, shared_types::XKB_KEY_DOWN);
                        st.update_key(lkc, shared_types::XKB_KEY_UP);
                    }
                    press_level_modifiers(&mut st, lvl, level_keys.0, level_keys.1, level_keys.2);
                    if !toggle {
                        st.update_key(lkc, shared_types::XKB_KEY_DOWN);
                    }
                    for kc in min_keycode..=max_keycode {
                        if let Some(ch) = get_char(kc, &st, lvl) {
                            map.insert(kc - EVDEV_OFFSET, ch);
                        }
                    }
                }
            }
        }
        maps
    };

    let level_keys = (
        level2_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
        level3_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
        level5_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
    );

    // Build level exceptions (direct keysym mapping)
    let mut level_exceptions_keymap = vec![BTreeMap::new(); XKB_MAX_LEVELS];
    for (lvl, map) in level_exceptions_keymap
        .iter_mut()
        .enumerate()
        .take(XKB_MAX_LEVELS)
    {
        for kc in min_keycode..=max_keycode {
            if let Some(&sym) = keymap.key_get_syms_by_level(kc, 0, lvl as u32).first() {
                if let Some(ch) = keysym_utf::keysym_to_char(sym) {
                    map.insert(kc - EVDEV_OFFSET, ch);
                }
            }
        }
    }

    // Build state keymap
    let mut state_keymap = vec![BTreeMap::new(); XKB_MAX_LEVELS];
    for (lvl, map) in state_keymap.iter_mut().enumerate().take(XKB_MAX_LEVELS) {
        if let Some(mut st) = keymap.new_state() {
            press_level_modifiers(&mut st, lvl, level_keys.0, level_keys.1, level_keys.2);
            for kc in min_keycode..=max_keycode {
                if let Some(ch) = get_char(kc, &st, lvl) {
                    map.insert(kc - EVDEV_OFFSET, ch);
                }
            }
        }
    }

    // Build caps/num lock keymaps (only store differences)
    let caps_lock_keymap = {
        let caps_kc = level_code(&modifiers, ModType::Caps).map(|(c, _)| c + EVDEV_OFFSET);
        let mut maps = populate_lock(caps_kc, false, level_keys);
        // Keep only differences from state_keymap
        for (lvl, map) in maps.iter_mut().enumerate() {
            map.retain(|&k, &mut v| state_keymap.get(lvl).and_then(|m| m.get(&k)) != Some(&v));
        }
        maps
    };

    let num_lock_keys = {
        let num_kc = level_code(&modifiers, ModType::Num).map(|(c, _)| c + EVDEV_OFFSET);
        let mut maps = populate_lock(num_kc, true, level_keys);
        for (lvl, map) in maps.iter_mut().enumerate() {
            map.retain(|&k, &mut v| state_keymap.get(lvl).and_then(|m| m.get(&k)) != Some(&v));
        }
        maps
    };

    // Build repeat info
    let repeat_keys = (min_keycode..=max_keycode)
        .filter(|&kc| keymap.key_repeats(kc))
        .map(|kc| kc - EVDEV_OFFSET)
        .collect();

    // Load compose table from locale
    let composer = locale
        .as_deref()
        .and_then(compose::resolve_compose_file)
        .map(|subpath| {
            let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
            compose::load_compose_from_path(&path)
        })
        .unwrap_or_else(ListComposer::new);

    WKB {
        layouts: all_layouts,
        layout: layout
            .clone()
            .unwrap_or_else(|| locale.clone().unwrap_or_default()),
        locale,
        pressed_keys: HashSet::new(),
        repeat_keys,
        composer,
        modifiers,
        state_keymap,
        num_lock_keys,
        caps_lock_keymap,
        level_exceptions_keymap,
    }
}

/// Create a new WKB instance from locale and layout names using rust-xkb
pub fn new_from_names(locale: String, layout: Option<String>) -> WKB<ListComposer> {
    use rust_types::{Context, RuleNames};

    // Get all available layouts for this locale
    let all_layouts = if layout.is_none() {
        get_all_layouts_for_locale(&locale)
    } else {
        // If a specific layout was requested, just use that one
        vec![layout.clone().unwrap()]
    };

    // Create XKB context using safe wrapper
    let ctx = Context::new().expect("Failed to create XKB context");

    // Prepare RMLVO names using Rust-native wrapper
    let rules = RuleNames::evdev(locale.clone(), layout.clone());

    // Compile keymap from RMLVO names
    let keymap = ctx
        .keymap_from_names(&rules)
        .unwrap_or_else(|| panic!("Failed to compile keymap for layout: {:?}", layout));

    // Build WKB from the keymap using safe wrappers
    build_wkb_from_keymap(&keymap, Some(locale), layout, all_layouts)
}

/// Build Modifiers struct from XKB keymap by querying modifier mappings
fn build_modifiers_from_keymap(
    keymap: &rust_types::Keymap,
    min_keycode: u32,
    max_keycode: u32,
) -> Modifiers {
    let mut modifiers = Modifiers(std::collections::BTreeMap::new());
    let num_mods = keymap.num_mods();

    // Helper: get ModType from keysym
    let keysym_to_modtype = |ks: u32| -> Option<ModType> {
        match ks {
            0xfe03 | 0xfe04 | 0xfe05 | 0xfe0d => Some(ModType::Level3),
            0xfe11..=0xfe13 => Some(ModType::Level5),
            _ => None,
        }
    };

    // Helper: create ModKind from keysym
    let keysym_to_modkind = |ks: u32, mt: ModType| -> ModKind {
        match ks {
            0xffe6 | 0xfe05 | 0xfe0d | 0xfe13 => ModKind::Lock {
                pressed: false,
                locked: 0,
                mod_type: mt,
            },
            0xfe04 | 0xfe12 => ModKind::Latch {
                pressed: false,
                latched: false,
                mod_type: mt,
            },
            _ => ModKind::Pressed {
                pressed: false,
                mod_type: mt,
            },
        }
    };

    // Build modifier name map
    let mod_name_to_type: std::collections::HashMap<String, ModType> = (0..num_mods)
        .filter_map(|i| {
            keymap.mod_get_name(i).and_then(|n| {
                Some((
                    n.clone(),
                    match n.as_str() {
                        "Shift" => ModType::Level2,
                        "ISO_Level3_Shift" | "Mode_switch" | "LevelThree" => ModType::Level3,
                        "ISO_Level5_Shift" | "LevelFive" => ModType::Level5,
                        "Lock" => ModType::Caps,
                        "Mod2" => ModType::Num,
                        "Mod5" => ModType::Level3,
                        "Scroll_Lock" | "ScrollLock" => ModType::Scroll,
                        "Control" => ModType::None,
                        _ => return None,
                    },
                ))
            })
        })
        .collect();

    const EVDEV_OFFSET: u32 = 8;
    for keycode in min_keycode..=max_keycode {
        let evdev_code = keycode - EVDEV_OFFSET;
        let syms = keymap.key_get_syms_by_level(keycode, 0, 0);
        let num_levels = keymap.num_levels_for_key(keycode, 0);

        // Handle keysym-based modifiers (one-level keys only)
        if num_levels == 1 && syms.len() == 1 {
            if let Some(mt) = keysym_to_modtype(syms[0]) {
                modifiers
                    .set_modifier(evdev_code, Modifier::Single(keysym_to_modkind(syms[0], mt)));
                continue;
            }
        }

        // Handle modifier map keys
        let (modmap, vmodmap) = match keymap.key_get_mods(keycode) {
            Some(m) => m,
            None => continue,
        };
        if modmap == 0 && vmodmap == 0 {
            continue;
        }

        for mod_idx in 0..num_mods {
            let mod_name = match keymap.mod_get_name(mod_idx) {
                Some(n) => n,
                None => continue,
            };
            let mod_mask = keymap.mod_get_mask(&mod_name);
            if (modmap & mod_mask) == 0 && (vmodmap & mod_mask) == 0 {
                continue;
            }

            // Determine ModType from keysym or name
            let mod_type = if syms.len() == 1 {
                keysym_to_modtype(syms[0]).or_else(|| mod_name_to_type.get(&mod_name).copied())
            } else {
                mod_name_to_type.get(&mod_name).copied()
            };
            let mod_type = match mod_type {
                Some(mt) => mt,
                None => continue,
            };

            // Special case: Caps lock with mixed keysyms (JP layout)
            if mod_type == ModType::Caps {
                let caps_levels: Vec<u32> = (0..num_levels)
                    .filter(|&lvl| {
                        keymap.key_get_syms_by_level(keycode, 0, lvl).get(0) == Some(&0xffe5)
                    })
                    .collect();
                if caps_levels.is_empty() {
                    continue;
                }
                if caps_levels.len() < num_levels as usize {
                    let min_caps = *caps_levels.iter().min().unwrap();
                    let level_map: BTreeMap<u8, ModKind> = (0..8)
                        .map(|l| {
                            (
                                l,
                                if l < min_caps as u8 {
                                    ModKind::None
                                } else {
                                    ModKind::Lock {
                                        pressed: false,
                                        locked: 0,
                                        mod_type: ModType::Caps,
                                    }
                                },
                            )
                        })
                        .collect();
                    modifiers.set_modifier(evdev_code, Modifier::Leveled(level_map));
                    continue;
                }
            }

            // Create Single modifier
            let mod_kind = if syms.len() == 1
                && matches!(
                    mod_type,
                    ModType::Level2 | ModType::Level3 | ModType::Level5
                ) {
                keysym_to_modkind(syms[0], mod_type)
            } else {
                match mod_type {
                    ModType::Caps | ModType::Num | ModType::Scroll => ModKind::Lock {
                        pressed: false,
                        locked: 0,
                        mod_type,
                    },
                    _ => ModKind::Pressed {
                        pressed: false,
                        mod_type,
                    },
                }
            };
            modifiers.set_modifier(evdev_code, Modifier::Single(mod_kind));
        }
    }
    modifiers
}

/// Create a new WKB instance from a keymap string
pub fn new_from_string(string: String) -> WKB<ListComposer> {
    use rust_types::Context;

    // Create XKB context using safe wrapper
    let ctx = Context::new().expect("Failed to create XKB context");

    // Parse keymap from string using safe wrapper
    let keymap = ctx
        .keymap_from_string(&string)
        .expect("Failed to parse keymap from string");

    // We don't know the original locale/layout from the string, so use None
    // and empty layouts list. Applications typically don't need these when
    // loading from string (they already have the full keymap).
    build_wkb_from_keymap(&keymap, None, None, vec![String::new()])
}

#[cfg(test)]
mod wrapper_tests {
    use super::rust_types::{Context, RuleNames};

    #[test]
    fn test_keymap_layout_methods() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames {
            rules: String::new(),
            model: String::new(),
            layout: "us,fr,de".to_string(),
            variant: String::new(),
            options: String::new(),
        };
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        // Test num_layouts
        assert_eq!(keymap.num_layouts(), 3);

        // Test layout names
        assert_eq!(keymap.layout_get_name(0), Some("English (US)".to_string()));
        assert_eq!(keymap.layout_get_name(1), Some("French".to_string()));
        assert_eq!(keymap.layout_get_name(2), Some("German".to_string()));

        // Test get_all_layouts
        let layouts = keymap.get_all_layouts();
        assert_eq!(layouts.len(), 3);
        assert_eq!(layouts[0], "English (US)");

        // Test layout_get_index
        assert_eq!(keymap.layout_get_index("English (US)"), Some(0));
        assert_eq!(keymap.layout_get_index("French"), Some(1));
        assert_eq!(keymap.layout_get_index("NonExistent"), None);
    }

    #[test]
    fn test_keymap_modifier_methods() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames::default();
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        // Test num_mods
        let num_mods = keymap.num_mods();
        assert!(num_mods > 0);

        // Test get_all_mods
        let mods = keymap.get_all_mods();
        assert_eq!(mods.len(), num_mods as usize);
        assert!(mods.contains(&"Shift".to_string()));
        assert!(mods.contains(&"Control".to_string()));

        // Test mod_get_index
        assert!(keymap.mod_get_index("Shift").is_some());
        assert!(keymap.mod_get_index("Control").is_some());
        assert_eq!(keymap.mod_get_index("NonExistent"), None);

        // Test mod_get_mask
        let shift_mask = keymap.mod_get_mask("Shift");
        assert!(shift_mask > 0);
    }

    #[test]
    fn test_keymap_led_methods() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames::default();
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        // Test num_leds
        let num_leds = keymap.num_leds();
        assert!(num_leds > 0);

        // Test get_all_leds
        let leds = keymap.get_all_leds();
        assert_eq!(leds.len(), num_leds as usize);

        // Common LED names
        if let Some(idx) = keymap.led_get_index("Caps Lock") {
            assert!(keymap.led_get_name(idx).is_some());
        }
    }

    #[test]
    fn test_keymap_key_methods() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames::default();
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        // Test key_get_name and key_by_name
        for (xkb_keycode, _evdev_code) in keymap.keycodes() {
            if let Some(name) = keymap.key_get_name(xkb_keycode) {
                // Verify round-trip works
                assert_eq!(keymap.key_by_name(&name), Some(xkb_keycode));
            }
        }

        // Test specific known keys
        if let Some(keycode) = keymap.key_by_name("SPCE") {
            assert_eq!(keymap.key_get_name(keycode), Some("SPCE".to_string()));
        }
    }

    #[test]
    fn test_keymap_num_layouts_for_key() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames {
            rules: String::new(),
            model: String::new(),
            layout: "us,fr".to_string(),
            variant: String::new(),
            options: String::new(),
        };
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        // Most keys should support both layouts
        let min_keycode = keymap.min_keycode();
        let num_layouts = keymap.num_layouts_for_key(min_keycode);
        assert!(num_layouts > 0);
        assert!(num_layouts <= keymap.num_layouts());
    }

    #[test]
    fn test_state_modifier_queries() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames::default();
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");
        let state = keymap.new_state().expect("Failed to create state");

        // Initially no modifiers should be active
        assert!(!state.mod_name_is_active("Shift", 0xFF));
        assert!(!state.mod_name_is_active("Control", 0xFF));

        // Get shift keycode and press it
        if let Some(shift_idx) = keymap.mod_get_index("Shift") {
            assert!(!state.mod_index_is_active(shift_idx, 0xFF));
        }
    }

    #[test]
    fn test_state_key_get_syms() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames::default();
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");
        let state = keymap.new_state().expect("Failed to create state");

        // Get syms for a key (e.g., space bar which is typically keycode 65 in XKB)
        for (xkb_keycode, _evdev_code) in keymap.keycodes().take(10) {
            let syms = state.key_get_syms(xkb_keycode);
            let one_sym = state.key_get_one_sym(xkb_keycode);

            if !syms.is_empty() {
                // If there are syms, one_sym should match the first one
                assert_eq!(one_sym, syms[0]);
            }
        }
    }

    #[test]
    fn test_state_update_mask() {
        let context = Context::new().expect("Failed to create context");
        let rules = RuleNames::default();
        let keymap = context
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");
        let mut state = keymap.new_state().expect("Failed to create state");

        // Update state with shift pressed
        let shift_mask = keymap.mod_get_mask("Shift");
        state.update_mask(shift_mask, 0, 0, 0, 0, 0);

        // Verify shift is now active
        assert!(state.mod_name_is_active("Shift", 0xFF));
    }
}
