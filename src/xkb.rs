//! XKB module — re-exports from xkb-core and WKB integration glue.

// Re-export xkb-core public API selectively

// WKB integration functions
use crate::composer::Token;
use crate::modifiers::*;
use crate::FlatKeymap;
use crate::ListComposer;
use crate::{KeyBitSet, WKB};

/// Get all available layouts/variants for a given locale
fn get_all_layouts_for_locale(locale: &str) -> Vec<String> {
    use xkb_core::rust_types::RxkbContext;

    let mut ctx = match RxkbContext::new() {
        Some(ctx) => ctx,
        None => return vec![String::new()],
    };

    ctx.include_path_append_default();

    if !ctx.parse("evdev") {
        return vec![String::new()];
    }

    let mut layouts = Vec::new();

    for layout in ctx.layouts() {
        let layout_name = layout.name();
        if !layout_name.is_empty() && layout_name == locale {
            let variant = layout.variant();
            if variant.is_empty() {
                layouts.push(String::new());
            } else {
                layouts.push(variant.to_string());
            }
        }
    }

    if layouts.is_empty() {
        layouts.push(String::new());
    }

    layouts
}

/// Get the keycode (and optional level) for a specific modifier type.
pub(crate) fn level_code(modifiers: &Modifiers, mod_type: ModType) -> Option<(u32, Option<u8>)> {
    let mut other_mod = None;

    for (code, modifier) in modifiers.iter() {
        match modifier {
            Modifier::Single(mod_kind) => {
                if mod_kind.get_modkind_from_modtype(mod_type).is_some() {
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

pub(crate) fn level2_code(modifiers: &Modifiers) -> Option<(u32, Option<u8>)> {
    level_code(modifiers, ModType::Level2)
}

pub(crate) fn level3_code(modifiers: &Modifiers) -> Option<(u32, Option<u8>)> {
    level_code(modifiers, ModType::Level3)
}

pub(crate) fn level5_code(modifiers: &Modifiers) -> Option<(u32, Option<u8>)> {
    level_code(modifiers, ModType::Level5)
}

/// Press the appropriate level modifier keys on `state` for the given level index.
fn press_level_modifiers(
    state: &mut xkb_core::rust_types::State,
    level: usize,
    level2: Option<u32>,
    level3: Option<u32>,
    level5: Option<u32>,
) {
    if level & 4 != 0 {
        if let Some(kc) = level5 {
            state.update_key(kc, xkb_core::XKB_KEY_DOWN);
        }
    }
    if level & 2 != 0 {
        if let Some(kc) = level3 {
            state.update_key(kc, xkb_core::XKB_KEY_DOWN);
        }
    }
    if level & 1 != 0 {
        if let Some(kc) = level2 {
            state.update_key(kc, xkb_core::XKB_KEY_DOWN);
        }
    }
}

/// Load compose entries from a file and build a ListComposer.
pub fn load_compose_from_path(path: &std::path::Path) -> ListComposer {
    let mut regular = ListComposer::new();

    let entries = xkb_core::compose::parse_compose_file(path);

    for entry in entries {
        let mut tokens: Vec<Token> = Vec::new();
        let mk_idx = entry.multi_key_index;

        for (i, ch) in entry.keys.iter().enumerate() {
            if let Some(idx) = mk_idx {
                if idx == i {
                    tokens.push(Token::Compose);
                }
            }
            tokens.push(Token::Char(*ch));
        }
        regular.insert(&tokens, entry.output);
    }
    regular
}

/// Build WKB instance from an XKB keymap
fn build_wkb_from_keymap(
    keymap: &xkb_core::rust_types::Keymap,
    locale: Option<String>,
    layout: Option<String>,
    all_layouts: Vec<String>,
    store_keymap: bool,
) -> WKB<ListComposer> {
    const XKB_MAX_LEVELS: usize = 8;
    const EVDEV_OFFSET: u32 = 8;

    let (min_keycode, max_keycode) = (keymap.min_keycode(), keymap.max_keycode());
    // Keycodes below EVDEV_OFFSET don't map to evdev codes; clamp to avoid underflow.
    let min_keycode = min_keycode.max(EVDEV_OFFSET);
    let num_keys = if max_keycode >= EVDEV_OFFSET {
        (max_keycode - EVDEV_OFFSET + 1) as usize
    } else {
        0
    };
    let modifiers = build_modifiers_from_keymap(keymap, min_keycode, max_keycode);

    let get_char = |kc: u32, state: &xkb_core::rust_types::State, lvl: usize| -> Option<char> {
        state.key_get_utf8(kc).chars().next().or_else(|| {
            keymap
                .key_get_syms_by_level(kc, 0, lvl as u32)
                .first()
                .and_then(|&s| xkb_core::keysym_utf::keysym_to_char(s))
        })
    };

    let populate_lock = |lock_kc: Option<u32>,
                         toggle: bool,
                         level_keys: (Option<u32>, Option<u32>, Option<u32>)|
     -> FlatKeymap {
        let mut fk = FlatKeymap::new(num_keys);
        if let Some(lkc) = lock_kc {
            for lvl in 0..XKB_MAX_LEVELS {
                if let Some(mut st) = keymap.new_state() {
                    if toggle {
                        st.update_key(lkc, xkb_core::XKB_KEY_DOWN);
                        st.update_key(lkc, xkb_core::XKB_KEY_UP);
                    }
                    press_level_modifiers(&mut st, lvl, level_keys.0, level_keys.1, level_keys.2);
                    if !toggle {
                        st.update_key(lkc, xkb_core::XKB_KEY_DOWN);
                    }
                    for kc in min_keycode..=max_keycode {
                        if let Some(ch) = get_char(kc, &st, lvl) {
                            fk.set(lvl, kc - EVDEV_OFFSET, ch);
                        }
                    }
                }
            }
        }
        fk
    };

    let level_keys = (
        level2_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
        level3_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
        level5_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
    );

    let mut level_exceptions_keymap = FlatKeymap::new(num_keys);
    for lvl in 0..XKB_MAX_LEVELS {
        for kc in min_keycode..=max_keycode {
            if let Some(&sym) = keymap.key_get_syms_by_level(kc, 0, lvl as u32).first() {
                if let Some(ch) = xkb_core::keysym_utf::keysym_to_char(sym) {
                    level_exceptions_keymap.set(lvl, kc - EVDEV_OFFSET, ch);
                }
            }
        }
    }

    let mut state_keymap = FlatKeymap::new(num_keys);
    for lvl in 0..XKB_MAX_LEVELS {
        if let Some(mut st) = keymap.new_state() {
            press_level_modifiers(&mut st, lvl, level_keys.0, level_keys.1, level_keys.2);
            for kc in min_keycode..=max_keycode {
                if let Some(ch) = get_char(kc, &st, lvl) {
                    state_keymap.set(lvl, kc - EVDEV_OFFSET, ch);
                }
            }
        }
    }

    let caps_lock_keymap = {
        let caps_kc = level_code(&modifiers, ModType::Caps).map(|(c, _)| c + EVDEV_OFFSET);
        let mut fk = populate_lock(caps_kc, false, level_keys);
        // Remove entries that are identical to state_keymap (only keep diffs)
        for lvl in 0..XKB_MAX_LEVELS {
            for k in 0..fk.num_keys as u32 {
                if let Some(v) = fk.get(lvl, k) {
                    if state_keymap.get(lvl, k) == Some(v) {
                        fk.data[lvl * fk.num_keys + k as usize] = None;
                    }
                }
            }
        }
        fk
    };

    let num_lock_keys = {
        let num_kc = level_code(&modifiers, ModType::Num).map(|(c, _)| c + EVDEV_OFFSET);
        let mut fk = populate_lock(num_kc, true, level_keys);
        for lvl in 0..XKB_MAX_LEVELS {
            for k in 0..fk.num_keys as u32 {
                if let Some(v) = fk.get(lvl, k) {
                    if state_keymap.get(lvl, k) == Some(v) {
                        fk.data[lvl * fk.num_keys + k as usize] = None;
                    }
                }
            }
        }
        fk
    };

    let mut repeat_keys = KeyBitSet::new();
    for kc in min_keycode..=max_keycode {
        if keymap.key_repeats(kc) {
            repeat_keys.insert(kc - EVDEV_OFFSET);
        }
    }

    #[cfg(feature = "compose")]
    let composer = locale
        .as_deref()
        .and_then(xkb_core::compose::resolve_compose_file)
        .map(|subpath| {
            let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
            load_compose_from_path(&path)
        })
        .unwrap_or_else(ListComposer::new);

    #[cfg(not(feature = "compose"))]
    let composer = ListComposer::new();

    WKB {
        layouts: all_layouts,
        layout: layout
            .clone()
            .unwrap_or_else(|| locale.clone().unwrap_or_default()),
        // locale,
        pressed_keys: KeyBitSet::new(),
        repeat_keys,
        composer,
        modifiers,
        state_keymap,
        num_lock_keys,
        caps_lock_keymap,
        level_exceptions_keymap,
        xkb_keymap: if store_keymap {
            Some(keymap.clone())
        } else {
            None
        },
    }
}

/// Create a new WKB instance from locale and layout names
pub fn new_from_names(locale: String, layout: Option<String>) -> WKB<ListComposer> {
    use xkb_core::rust_types::{Context, RuleNames};

    let all_layouts = if layout.is_none() {
        get_all_layouts_for_locale(&locale)
    } else {
        vec![layout.clone().unwrap()]
    };

    let ctx = Context::new().expect("Failed to create XKB context");
    let rules = RuleNames::evdev(locale.clone(), layout.clone());

    let keymap = ctx
        .keymap_from_names(&rules)
        .unwrap_or_else(|| panic!("Failed to compile keymap for layout: {:?}", layout));

    build_wkb_from_keymap(&keymap, Some(locale), layout, all_layouts, true)
}

/// Build Modifiers struct from XKB keymap
fn build_modifiers_from_keymap(
    keymap: &xkb_core::rust_types::Keymap,
    min_keycode: u32,
    max_keycode: u32,
) -> Modifiers {
    let mut modifiers = Modifiers::new();
    let num_mods = keymap.num_mods();

    let keysym_to_modtype = |ks: u32| -> Option<ModType> {
        match ks {
            0xfe03 | 0xfe04 | 0xfe05 | 0xfe0d => Some(ModType::Level3),
            0xfe11..=0xfe13 => Some(ModType::Level5),
            _ => None,
        }
    };

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
    for keycode in min_keycode.max(EVDEV_OFFSET)..=max_keycode {
        let evdev_code = keycode - EVDEV_OFFSET;
        let syms = keymap.key_get_syms_by_level(keycode, 0, 0);
        let num_levels = keymap.num_levels_for_key(keycode, 0);

        if num_levels == 1 && syms.len() == 1 {
            if let Some(mt) = keysym_to_modtype(syms[0]) {
                modifiers
                    .set_modifier(evdev_code, Modifier::Single(keysym_to_modkind(syms[0], mt)));
                continue;
            }
        }

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

            let mod_type = if syms.len() == 1 {
                keysym_to_modtype(syms[0]).or_else(|| mod_name_to_type.get(&mod_name).copied())
            } else {
                mod_name_to_type.get(&mod_name).copied()
            };
            let mod_type = match mod_type {
                Some(mt) => mt,
                None => continue,
            };

            if mod_type == ModType::Caps {
                let caps_levels: Vec<u32> = (0..num_levels)
                    .filter(|&lvl| {
                        keymap.key_get_syms_by_level(keycode, 0, lvl).first() == Some(&0xffe5)
                    })
                    .collect();
                if caps_levels.is_empty() {
                    continue;
                }
                if caps_levels.len() < num_levels as usize {
                    let min_caps = *caps_levels.iter().min().unwrap();
                    let level_map: std::collections::BTreeMap<u8, ModKind> = (0..8)
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
    use xkb_core::rust_types::Context;

    let ctx = Context::new().expect("Failed to create XKB context");

    let keymap = ctx
        .keymap_from_string(&string)
        .expect("Failed to parse keymap from string");

    build_wkb_from_keymap(&keymap, None, None, vec![String::new()], true)
}

/// Backward-compatible alias for compose module access
pub mod compose_parse {
    pub use super::load_compose_from_path;
    pub use xkb_core::compose::*;
}

#[cfg(test)]
mod wrapper_tests {
    use xkb_core::rust_types::{Context, RuleNames};

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

        assert_eq!(keymap.num_layouts(), 3);
        assert_eq!(keymap.layout_get_name(0), Some("English (US)".to_string()));
        assert_eq!(keymap.layout_get_name(1), Some("French".to_string()));
        assert_eq!(keymap.layout_get_name(2), Some("German".to_string()));

        let layouts = keymap.get_all_layouts();
        assert_eq!(layouts.len(), 3);
        assert_eq!(layouts[0], "English (US)");

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

        let num_mods = keymap.num_mods();
        assert!(num_mods > 0);

        let mods = keymap.get_all_mods();
        assert_eq!(mods.len(), num_mods as usize);
        assert!(mods.contains(&"Shift".to_string()));
        assert!(mods.contains(&"Control".to_string()));

        assert!(keymap.mod_get_index("Shift").is_some());
        assert!(keymap.mod_get_index("Control").is_some());
        assert_eq!(keymap.mod_get_index("NonExistent"), None);

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

        let num_leds = keymap.num_leds();
        assert!(num_leds > 0);

        let leds = keymap.get_all_leds();
        assert_eq!(leds.len(), num_leds as usize);

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

        for (xkb_keycode, _evdev_code) in keymap.keycodes() {
            if let Some(name) = keymap.key_get_name(xkb_keycode) {
                assert_eq!(keymap.key_by_name(&name), Some(xkb_keycode));
            }
        }

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

        assert!(!state.mod_name_is_active("Shift", 0xFF));
        assert!(!state.mod_name_is_active("Control", 0xFF));

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

        for (xkb_keycode, _evdev_code) in keymap.keycodes().take(10) {
            let syms = state.key_get_syms(xkb_keycode);
            let one_sym = state.key_get_one_sym(xkb_keycode);

            if !syms.is_empty() {
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

        let shift_mask = keymap.mod_get_mask("Shift");
        state.update_mask(shift_mask, 0, 0, 0, 0, 0);

        assert!(state.mod_name_is_active("Shift", 0xFF));
    }
}
