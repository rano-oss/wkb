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
    let ctx = match RxkbContext::new() {
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
    let mut layout_opt = ctx.layout_first();

    while let Some(layout) = layout_opt {
        if let Some(layout_name) = layout.get_name() {
            // Check if this layout matches our locale
            if layout_name == locale {
                match layout.get_variant() {
                    None => {
                        // Base layout (no variant) - store empty string
                        layouts.push(String::new());
                    }
                    Some(variant) => {
                        // Variant layout - store the variant name
                        layouts.push(variant);
                    }
                }
            }
        }

        layout_opt = layout.next();
    }

    // Context is automatically cleaned up via Drop

    // If we didn't find any layouts, return empty string as default (base layout)
    if layouts.is_empty() {
        layouts.push(String::new());
    }

    layouts
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
/// This is the shared logic used by both new_from_names() and new_from_string()
fn build_wkb_from_keymap(
    keymap: &rust_types::Keymap,
    locale: Option<String>,
    layout: Option<String>,
    all_layouts: Vec<String>,
) -> WKB<ListComposer> {
    // Get keycode range
    let min_keycode = keymap.min_keycode();
    let max_keycode = keymap.max_keycode();

    // XKB supports up to 8 levels (0-7)
    // We need to create 8 levels regardless of how many are actually defined in the keymap,
    // because applications may try to access any level, and XKB has fallback rules for undefined levels.
    const XKB_MAX_LEVELS: usize = 8;

    // First pass: determine the maximum number of levels actually defined in the keymap
    let mut max_defined_levels = 1; // At least level 0
    for keycode in min_keycode..=max_keycode {
        let layout_idx = 0;
        let num_levels = keymap.num_levels_for_key(keycode, layout_idx);
        if num_levels > max_defined_levels {
            max_defined_levels = num_levels;
        }
    }

    // Initialize state_keymap with XKB_MAX_LEVELS levels to support all possible modifier combinations
    let mut state_keymap: Vec<BTreeMap<u32, char>> = vec![BTreeMap::new(); XKB_MAX_LEVELS];
    let mut caps_lock_keymap = vec![BTreeMap::new(); XKB_MAX_LEVELS];
    let mut num_lock_keys = vec![BTreeMap::new(); XKB_MAX_LEVELS];
    let mut level_exceptions_keymap = vec![BTreeMap::new(); XKB_MAX_LEVELS];

    // Note: XKB keycodes for evdev start at 9, but evdev codes start at 0
    // The ESC key is XKB keycode 9 and should map to evdev code 1
    // Therefore: evdev_code = xkb_keycode - 8
    let evdev_offset = 8; // Standard offset to convert XKB keycode to evdev code

    // Build modifiers map from XKB keymap FIRST so we can use it to set modifier states
    let modifiers = build_modifiers_from_keymap(keymap, min_keycode, max_keycode);

    // Populate level_exceptions_keymap by querying keysyms directly from keymap
    // This is used by level_key() to return raw keymap level definitions
    for level in 0..XKB_MAX_LEVELS {
        for keycode in min_keycode..=max_keycode {
            let evdev_code = keycode - evdev_offset;
            let layout_idx = 0;

            // Query keysyms directly at this level using safe wrapper
            let syms = keymap.key_get_syms_by_level(keycode, layout_idx, level as u32);

            if let Some(&keysym) = syms.first() {
                if let Some(ch) = keysym_utf::keysym_to_char(keysym) {
                    level_exceptions_keymap[level].insert(evdev_code, ch);
                }
            }
        }
    }

    // Build state_keymap by actually simulating modifier combinations with XKB state
    // This ensures we get the correct behavior for keys whose types don't respond to certain modifiers
    // IMPORTANT: Create a fresh state for each level to avoid locking modifier issues
    {
        let level2_keycode = modifiers.level2_code().map(|(code, _)| code + evdev_offset);
        let level3_keycode = modifiers.level3_code().map(|(code, _)| code + evdev_offset);
        let level5_keycode = modifiers.level5_code().map(|(code, _)| code + evdev_offset);

        for level in 0..XKB_MAX_LEVELS {
            let mut state = match keymap.new_state() {
                Some(s) => s,
                None => continue,
            };
            press_level_modifiers(
                &mut state,
                level,
                level2_keycode,
                level3_keycode,
                level5_keycode,
            );

            for keycode in min_keycode..=max_keycode {
                let evdev_code = keycode - evdev_offset;
                let s = state.key_get_utf8(keycode);
                if let Some(ch) = s.chars().next() {
                    state_keymap[level].insert(evdev_code, ch);
                } else {
                    let syms = keymap.key_get_syms_by_level(keycode, 0, level as u32);
                    if let Some(&keysym) = syms.first() {
                        if let Some(ch) = keysym_utf::keysym_to_char(keysym) {
                            state_keymap[level].insert(evdev_code, ch);
                        }
                    }
                }
            }
        }
    }

    // Now populate caps_lock_keymap: simulate Caps Lock being active with different modifiers
    {
        let caps_lock_xkb_keycode = modifiers
            .level_code(ModType::Caps)
            .map(|(code, _)| code + evdev_offset);

        if let Some(caps_lock_keycode) = caps_lock_xkb_keycode {
            let level2_keycode = modifiers.level2_code().map(|(code, _)| code + evdev_offset);
            let level3_keycode = modifiers.level3_code().map(|(code, _)| code + evdev_offset);
            let level5_keycode = modifiers.level5_code().map(|(code, _)| code + evdev_offset);

            for level in 0..XKB_MAX_LEVELS {
                let mut caps_state = match keymap.new_state() {
                    Some(s) => s,
                    None => continue,
                };
                // Press level modifiers FIRST, then Caps Lock
                press_level_modifiers(
                    &mut caps_state,
                    level,
                    level2_keycode,
                    level3_keycode,
                    level5_keycode,
                );
                caps_state.update_key(caps_lock_keycode, shared_types::XKB_KEY_DOWN);

                for keycode in min_keycode..=max_keycode {
                    let evdev_code = keycode - evdev_offset;
                    let s = caps_state.key_get_utf8(keycode);
                    if let Some(ch) = s.chars().next() {
                        if state_keymap.get(level).and_then(|m| m.get(&evdev_code)) != Some(&ch) {
                            caps_lock_keymap[level].insert(evdev_code, ch);
                        }
                    }
                }
            }
        }
    }

    // Populate num_lock_keys: simulate Num Lock being active with different modifiers
    {
        let num_lock_xkb_keycode = modifiers
            .level_code(ModType::Num)
            .map(|(code, _)| code + evdev_offset);

        if let Some(num_lock_keycode) = num_lock_xkb_keycode {
            let level2_keycode = modifiers.level2_code().map(|(code, _)| code + evdev_offset);
            let level3_keycode = modifiers.level3_code().map(|(code, _)| code + evdev_offset);
            let level5_keycode = modifiers.level5_code().map(|(code, _)| code + evdev_offset);

            for level in 0..XKB_MAX_LEVELS {
                let mut num_state = match keymap.new_state() {
                    Some(s) => s,
                    None => continue,
                };
                // Toggle Num Lock first, then level modifiers
                num_state.update_key(num_lock_keycode, shared_types::XKB_KEY_DOWN);
                num_state.update_key(num_lock_keycode, shared_types::XKB_KEY_UP);
                press_level_modifiers(
                    &mut num_state,
                    level,
                    level2_keycode,
                    level3_keycode,
                    level5_keycode,
                );

                for keycode in min_keycode..=max_keycode {
                    let evdev_code = keycode - evdev_offset;
                    let s = num_state.key_get_utf8(keycode);
                    if let Some(ch) = s.chars().next() {
                        if state_keymap.get(level).and_then(|m| m.get(&evdev_code)) != Some(&ch) {
                            num_lock_keys[level].insert(evdev_code, ch);
                        }
                    }
                }
            }
        }
    }

    // Build composer from XKB compose table
    // NOTE: Compose table iteration has memory safety issues in some configurations
    // For now, we use an empty composer. The compose resolution logic is tested separately.
    let _compose_locale = match locale.as_ref() {
        Some(loc) => match compose::resolve_compose_file(loc) {
            Some(path) => {
                // Extract locale from path like "en_US.UTF-8/Compose"
                path.strip_suffix("/Compose")
                    .unwrap_or("en_US.UTF-8")
                    .to_string()
            }
            None => "en_US.UTF-8".to_string(),
        },
        None => "en_US.UTF-8".to_string(),
    };
    // TODO: Enable when memory safety issues are resolved
    // let composer = build_composer_from_xkb(ctx, &compose_locale);
    let composer = ListComposer::new();

    // Populate repeat_keys: determine which keys are repeatable
    let mut repeat_keys = HashSet::new();
    for keycode in min_keycode..=max_keycode {
        if keymap.key_repeats(keycode) {
            let evdev_code = keycode - evdev_offset;
            repeat_keys.insert(evdev_code);
        }
    }

    // Keymap and Context are automatically cleaned up via Drop

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
    // Start with an EMPTY modifiers map - we'll populate it from the keymap
    // DO NOT use Modifiers::default() because that has hard-coded key mappings
    // that may not match the actual layout (e.g., in neo layout, key 58 is not caps lock)
    let mut modifiers = Modifiers(std::collections::BTreeMap::new());

    // Query all modifiers from the keymap
    let num_mods = keymap.num_mods();

    // Build a map of modifier names to their indices and types
    let mut mod_name_to_type: std::collections::HashMap<String, ModType> =
        std::collections::HashMap::new();

    for mod_idx in 0..num_mods {
        let mod_name = match keymap.mod_get_name(mod_idx) {
            Some(name) => name,
            None => continue,
        };

        // Map XKB modifier names to our ModType
        let mod_type = match mod_name.as_str() {
            "Shift" => ModType::Level2,
            "ISO_Level3_Shift" | "Mode_switch" | "LevelThree" => ModType::Level3,
            "ISO_Level5_Shift" | "LevelFive" => ModType::Level5,
            "Lock" => ModType::Caps,
            "Mod2" => ModType::Num,    // Num_Lock is typically mapped to Mod2
            "Mod5" => ModType::Level3, // Mod5 often used for Level3 (e.g., mm/zawgyi tilde_latch)
            "Scroll_Lock" | "ScrollLock" => ModType::Scroll,
            "Control" => ModType::None, // Control keys use ModType::None
            _ => continue,              // Skip unknown modifiers
        };

        mod_name_to_type.insert(mod_name, mod_type);
    }

    // Now iterate through all keys to find which keys produce which modifiers
    let evdev_offset = 8;
    for keycode in min_keycode..=max_keycode {
        let (modmap, vmodmap) = match keymap.key_get_mods(keycode) {
            Some(mods) => mods,
            None => continue,
        };

        let evdev_code = keycode - evdev_offset;

        // Check both real modifiers (modmap) and virtual modifiers (vmodmap)
        if modmap != 0 || vmodmap != 0 {
            // Check which modifiers this key produces
            for mod_idx in 0..num_mods {
                let mod_name = match keymap.mod_get_name(mod_idx) {
                    Some(name) => name,
                    None => continue,
                };
                let mod_mask = keymap.mod_get_mask(&mod_name);

                // Check both modmap and vmodmap
                if (modmap & mod_mask) != 0 || (vmodmap & mod_mask) != 0 {
                    // This key produces this modifier
                    // First check if key produces a Level3/Level5 keysym (handles Mod5 mapping to Level3)
                    let layout_idx = 0;
                    let level_idx = 0;
                    let syms = keymap.key_get_syms_by_level(keycode, layout_idx, level_idx);

                    let keysym_mod_type = if syms.len() == 1 {
                        let keysym = syms[0];
                        match keysym {
                            // Level3 keysyms
                            0xfe03 | 0xfe04 | 0xfe05 | 0xfe0d => Some(ModType::Level3),
                            // Level5 keysyms
                            0xfe11..=0xfe13 => Some(ModType::Level5),
                            _ => None,
                        }
                    } else {
                        None
                    };

                    // If keysym indicates Level3/Level5, use that; otherwise check mod_name
                    let mod_type = if let Some(mt) = keysym_mod_type {
                        mt
                    } else {
                        let mod_name = match keymap.mod_get_name(mod_idx) {
                            Some(name) => name,
                            None => continue,
                        };
                        if let Some(&mt) = mod_name_to_type.get(&mod_name) {
                            mt
                        } else {
                            continue; // Unknown modifier, skip
                        }
                    };

                    let evdev_code = keycode - evdev_offset;

                    // Special case: Lock modifier can be Caps_Lock or Eisu_toggle (JP layout)
                    // Only treat as Caps if key produces Caps_Lock keysym at ANY level
                    // JP layout has: key <CAPS> {[ Eisu_toggle, Caps_Lock ]};
                    // So level 0 = Eisu_toggle, level 1 = Caps_Lock
                    // We need to create a Leveled modifier for multi-level Caps keys
                    if mod_type == ModType::Caps {
                        // Check ALL levels for Caps_Lock keysym (0xffe5)
                        let num_levels_for_key = keymap.num_levels_for_key(keycode, layout_idx);
                        let mut caps_lock_levels = Vec::new();
                        let mut non_caps_levels = Vec::new();

                        for check_level in 0..num_levels_for_key {
                            let check_syms =
                                keymap.key_get_syms_by_level(keycode, layout_idx, check_level);

                            if check_syms.len() == 1 {
                                if check_syms[0] == 0xffe5 {
                                    caps_lock_levels.push(check_level);
                                } else {
                                    non_caps_levels.push(check_level);
                                }
                            }
                        }

                        if caps_lock_levels.is_empty() {
                            continue; // Not real Caps Lock (e.g., pure Eisu_toggle), skip it
                        }

                        // If we have both Caps_Lock and non-Caps levels, create a Leveled modifier
                        if !non_caps_levels.is_empty() {
                            let mut level_map = BTreeMap::new();

                            // Determine the minimum level with Caps_Lock
                            let min_caps_level = *caps_lock_levels.iter().min().unwrap();

                            // Add ModKind for all levels 0-7
                            for level in 0..8u8 {
                                if level < min_caps_level as u8 {
                                    // Below minimum Caps level: use None
                                    level_map.insert(level, ModKind::None);
                                } else {
                                    // At or above Caps level: use Caps Lock
                                    level_map.insert(
                                        level,
                                        ModKind::Lock {
                                            pressed: false,
                                            locked: 0,
                                            mod_type: ModType::Caps,
                                        },
                                    );
                                }
                            }

                            modifiers.set_modifier(evdev_code, Modifier::Leveled(level_map));
                            continue; // Skip normal Single modifier creation
                        }
                        // Otherwise fall through to create Single modifier
                    }

                    // Create appropriate ModKind based on modifier type
                    // For Level2/Level3/Level5, check keysym to determine if Pressed/Latch/Lock
                    let mod_kind = if mod_type == ModType::Level2
                        || mod_type == ModType::Level3
                        || mod_type == ModType::Level5
                    {
                        if syms.len() == 1 {
                            let keysym = syms[0];
                            match keysym {
                                // Level2 Lock: Shift_Lock
                                0xffe6 => ModKind::Lock {
                                    pressed: false,
                                    locked: 0,
                                    mod_type,
                                },
                                // Level3/Level5 Latch variants
                                0xfe04 | 0xfe12 => ModKind::Latch {
                                    pressed: false,
                                    latched: false,
                                    mod_type,
                                },
                                // Level3/Level5 Lock variants
                                0xfe05 | 0xfe0d | 0xfe13 => ModKind::Lock {
                                    pressed: false,
                                    locked: 0,
                                    mod_type,
                                },
                                // Default to Pressed for Shift variants or others
                                _ => ModKind::Pressed {
                                    pressed: false,
                                    mod_type,
                                },
                            }
                        } else {
                            ModKind::Pressed {
                                pressed: false,
                                mod_type,
                            }
                        }
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

                    // Update the modifiers map
                    modifiers.set_modifier(evdev_code, Modifier::Single(mod_kind));
                }
            }
        }

        // Also check if this key produces a level shift keysym (like ISO_Level5_Shift)
        // This handles cases like level5(rctrl_switch) where RCTL produces ISO_Level5_Shift keysym
        // Only treat as keysym-based modifier if key has ONE level (ONE_LEVEL type)
        // Keys with multiple levels should produce chars at other levels, not act as pure modifiers
        let layout_idx = 0;
        let level_idx = 0;
        let num_levels = keymap.num_levels_for_key(keycode, layout_idx);
        let syms = keymap.key_get_syms_by_level(keycode, layout_idx, level_idx);

        if num_levels == 1 && syms.len() == 1 {
            let keysym = syms[0];

            // Check if this keysym is a level shift/latch/lock
            // Level3: Shift=0xfe03, Latch=0xfe04, Shift_Lock=0xfe05, Lock=0xfe0d
            // Level5: Shift=0xfe11, Latch=0xfe12, Lock=0xfe13
            let mod_kind = match keysym {
                // Level3 variants
                0xfe03 => Some(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::Level3,
                }),
                0xfe04 => Some(ModKind::Latch {
                    pressed: false,
                    latched: false,
                    mod_type: ModType::Level3,
                }),
                0xfe05 | 0xfe0d => Some(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::Level3,
                }),
                // Level5 variants
                0xfe11 => Some(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::Level5,
                }),
                0xfe12 => Some(ModKind::Latch {
                    pressed: false,
                    latched: false,
                    mod_type: ModType::Level5,
                }),
                0xfe13 => Some(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::Level5,
                }),
                _ => None,
            };

            if let Some(mod_kind) = mod_kind {
                modifiers.set_modifier(evdev_code, Modifier::Single(mod_kind));
            }
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
