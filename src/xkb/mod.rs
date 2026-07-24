//! XKB module — keymap construction from RMLVO names and XKB strings,
//! plus XKB v1 text serialization.

pub(crate) mod keymap;
pub(crate) mod keysym;
pub(crate) mod shared_types;
pub(crate) mod xkbcomp;

use crate::composer::Token;
use crate::flat_keymap::{FlatKeymap, FlatNamedKeyMap, MAX_LEVELS};
use crate::modifiers::*;
use crate::named_keys::NamedKey;
use crate::Composer;
use crate::KeyBitSet;
use crate::WKB;

// ── Error type ──

/// Errors returned by XKB keymap construction.
#[derive(Debug, thiserror::Error)]
pub enum XkbError {
    /// Failed to create an XKB context.
    #[error("Failed to create XKB context")]
    ContextCreation,
    /// Failed to compile keymap from RMLVO names.
    #[error("Failed to compile keymap")]
    KeymapCompilation,
    /// Failed to parse keymap from string.
    #[error("Failed to parse keymap string")]
    KeymapParsing,
}

/// Get the keycode (and optional level) for a specific modifier type.
pub(crate) fn level_code(modifiers: &Modifiers, mod_type: ModType) -> Option<(u32, Option<u8>)> {
    let mut other_mod = None;

    for (code, modifier) in modifiers.iter() {
        match modifier {
            Modifier::Single(mod_kind) => {
                if mod_kind.has_mod_type(mod_type) {
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
                    if mod_kind.has_mod_type(mod_type) {
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

/// Press the appropriate level modifier keys on `state` for the given level index.
fn press_level_modifiers(
    state: &mut keymap::State,
    level: usize,
    level2: Option<u32>,
    level3: Option<u32>,
    level5: Option<u32>,
) {
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

/// Load compose entries from a file and build a ListComposer.
/// Uses first-wins semantics to match xkbcommon behavior: if multiple
/// entries resolve to the same token sequence, only the first is kept.
pub fn load_compose_from_path(path: &std::path::Path) -> Composer {
    let mut regular = Composer::new();
    let mut seen: std::collections::HashSet<Vec<u32>> = std::collections::HashSet::new();

    let entries = keymap::parse_compose_file(path);

    for entry in entries {
        let mut tokens: Vec<Token> = Vec::new();
        let mut key: Vec<u32> = Vec::new();
        let mk_idx = entry.multi_key_index;

        for (i, ch) in entry.keys.iter().enumerate() {
            if let Some(idx) = mk_idx {
                if idx == i {
                    tokens.push(Token::Compose);
                    key.push(0);
                }
            }
            tokens.push(Token::Char(*ch));
            key.push(*ch as u32);
        }
        if seen.insert(key) {
            regular.insert(&tokens, entry.output);
        }
    }
    regular
}

/// Map an XKB keysym value to a [`NamedKey`].
///
/// KP variants collapse to their main key equivalents (KP_Enter → Enter).
/// ISO_Left_Tab → Tab, ISO_Enter → Enter.
/// Dead keys and character-producing keys map to `Unnamed`.
pub(crate) fn keysym_to_named_key(keysym: u32) -> NamedKey {
    const TABLE: &[(u32, NamedKey)] = &[
        (0x0020, NamedKey::Space),
        (0xff09, NamedKey::Tab),
        (0xff08, NamedKey::Backspace),
        (0xff0d, NamedKey::Enter),
        (0xff1b, NamedKey::Escape),
        (0xffff, NamedKey::Delete),
        (0xff63, NamedKey::Insert),
        (0xff51, NamedKey::ArrowLeft),
        (0xff53, NamedKey::ArrowRight),
        (0xff52, NamedKey::ArrowUp),
        (0xff54, NamedKey::ArrowDown),
        (0xff50, NamedKey::Home),
        (0xff57, NamedKey::End),
        (0xff55, NamedKey::PageUp),
        (0xff56, NamedKey::PageDown),
        (0xffbe, NamedKey::F1),
        (0xffbf, NamedKey::F2),
        (0xffc0, NamedKey::F3),
        (0xffc1, NamedKey::F4),
        (0xffc2, NamedKey::F5),
        (0xffc3, NamedKey::F6),
        (0xffc4, NamedKey::F7),
        (0xffc5, NamedKey::F8),
        (0xffc6, NamedKey::F9),
        (0xffc7, NamedKey::F10),
        (0xffc8, NamedKey::F11),
        (0xffc9, NamedKey::F12),
        (0xffca, NamedKey::F13),
        (0xffcb, NamedKey::F14),
        (0xffcc, NamedKey::F15),
        (0xffcd, NamedKey::F16),
        (0xffce, NamedKey::F17),
        (0xffcf, NamedKey::F18),
        (0xffd0, NamedKey::F19),
        (0xffd1, NamedKey::F20),
        (0xffd2, NamedKey::F21),
        (0xffd3, NamedKey::F22),
        (0xffd4, NamedKey::F23),
        (0xffd5, NamedKey::F24),
        (0xffd6, NamedKey::F25),
        (0xffd7, NamedKey::F26),
        (0xffd8, NamedKey::F27),
        (0xffd9, NamedKey::F28),
        (0xffda, NamedKey::F29),
        (0xffdb, NamedKey::F30),
        (0xffdc, NamedKey::F31),
        (0xffdd, NamedKey::F32),
        (0xffde, NamedKey::F33),
        (0xffdf, NamedKey::F34),
        (0xffe0, NamedKey::F35),
        (0xffe1, NamedKey::LeftShift),
        (0xffe2, NamedKey::RightShift),
        (0xffe3, NamedKey::LeftControl),
        (0xffe4, NamedKey::RightControl),
        (0xffe9, NamedKey::LeftAlt),
        (0xffea, NamedKey::RightAlt),
        (0xffe7, NamedKey::LeftMeta),
        (0xffe8, NamedKey::RightMeta),
        (0xffeb, NamedKey::LeftSuper),
        (0xffec, NamedKey::RightSuper),
        (0xffed, NamedKey::LeftHyper),
        (0xffee, NamedKey::RightHyper),
        (0xffe5, NamedKey::CapsLock),
        (0xff7f, NamedKey::NumLock),
        (0xff14, NamedKey::ScrollLock),
        (0xff61, NamedKey::PrintScreen),
        (0xff13, NamedKey::Pause),
        (0xff15, NamedKey::SysReq),
        (0xff67, NamedKey::ContextMenu),
        (0x1008ff21, NamedKey::Power),
        (0x1008ff2a, NamedKey::PowerOff),
        (0x1008ff2f, NamedKey::Sleep),
        (0x1008ff2b, NamedKey::WakeUp),
        (0x1008ffa7, NamedKey::Suspend),
        (0x1008ffa8, NamedKey::Hibernate),
        (0x1008ff14, NamedKey::MediaPlay),
        (0x1008ff31, NamedKey::MediaPause),
        (0x1008ff15, NamedKey::MediaStop),
        (0x1008ff17, NamedKey::MediaNextTrack),
        (0x1008ff16, NamedKey::MediaPreviousTrack),
        (0x1008ff13, NamedKey::VolumeUp),
        (0x1008ff11, NamedKey::VolumeDown),
        (0x1008ff12, NamedKey::VolumeMute),
        (0x1008ff26, NamedKey::BrowserBack),
        (0x1008ff27, NamedKey::BrowserForward),
        (0x1008ff29, NamedKey::BrowserRefresh),
        (0x1008ff18, NamedKey::BrowserHome),
        (0x1008ff19, NamedKey::LaunchMail),
        (0x1008ff1d, NamedKey::LaunchCalculator),
        (0x1008ff80, NamedKey::LaunchTerminal),
        (0x1008ff02, NamedKey::BrightnessUp),
        (0x1008ff03, NamedKey::BrightnessDown),
        (0x1008ff05, NamedKey::KeyboardBrightnessUp),
        (0x1008ff06, NamedKey::KeyboardBrightnessDown),
        (0xff21, NamedKey::KanjiMode),
        (0xff25, NamedKey::Hiragana),
        (0xff26, NamedKey::Katakana),
        (0xff24, NamedKey::Romaji),
        (0xff2a, NamedKey::ZenkakuHankaku),
        (0xff30, NamedKey::EisuToggle),
        (0xff34, NamedKey::HangulHanja),
        (0xff80, NamedKey::Space),
        (0xff8d, NamedKey::Enter),
        (0xff89, NamedKey::Tab),
        (0xff9f, NamedKey::Delete),
        (0xff9e, NamedKey::Insert),
        (0xff95, NamedKey::Home),
        (0xff9c, NamedKey::End),
        (0xff9a, NamedKey::PageUp),
        (0xff9b, NamedKey::PageDown),
        (0xff97, NamedKey::ArrowUp),
        (0xff99, NamedKey::ArrowDown),
        (0xff96, NamedKey::ArrowLeft),
        (0xff98, NamedKey::ArrowRight),
        (0xfe20, NamedKey::Tab),
        (0xfe34, NamedKey::Enter),
    ];
    if (0xfe50..=0xfe8d).contains(&keysym) {
        return NamedKey::Unnamed;
    }
    TABLE
        .iter()
        .find(|(ks, _)| *ks == keysym)
        .map(|(_, nk)| *nk)
        .unwrap_or(NamedKey::Unnamed)
}

/// Remove entries from `fk` that are identical to `state_keymap` (keep only diffs).
fn dedup_against_state(fk: &mut FlatKeymap, state_keymap: &FlatKeymap, num_layouts: usize) {
    for layout_idx in 0..num_layouts {
        for lvl in 0..MAX_LEVELS {
            for k in 0..fk.num_keys as u32 {
                if let Some(v) = fk.get(layout_idx, lvl, k) {
                    if state_keymap.get(layout_idx, lvl, k) == Some(v) {
                        let idx = (layout_idx * MAX_LEVELS + lvl) * fk.num_keys + k as usize;
                        fk.data[idx] = None;
                    }
                }
            }
        }
    }
}

/// Build WKB instance from an XKB keymap, extracting all layouts.
fn build_wkb_from_keymap(keymap: &keymap::Keymap, locale: Option<&str>, store_keymap: bool) -> WKB {
    const EVDEV_OFFSET: u32 = 8;

    let (min_keycode, max_keycode) = (keymap.min_keycode(), keymap.max_keycode());
    let min_keycode = min_keycode.max(EVDEV_OFFSET);
    let num_keys = if max_keycode >= EVDEV_OFFSET {
        (max_keycode - EVDEV_OFFSET + 1) as usize
    } else {
        0
    };
    let num_layouts = (keymap.num_layouts() as usize).max(1);

    // Modifiers are global to the keymap (not per-layout), use layout 0.
    let modifiers = build_modifiers_from_keymap(keymap, min_keycode, max_keycode);

    let level_keys = (
        level_code(&modifiers, ModType::Level2).map(|(c, _)| c + EVDEV_OFFSET),
        level_code(&modifiers, ModType::Level3).map(|(c, _)| c + EVDEV_OFFSET),
        level_code(&modifiers, ModType::Level5).map(|(c, _)| c + EVDEV_OFFSET),
    );

    // ── Build flat keymaps for ALL layouts ──

    // Build level_exceptions_keymap and named_key_map in a single pass
    // (both use key_get_syms_by_level, no state needed)
    let mut level_exceptions_keymap = FlatKeymap::new(num_keys, num_layouts);
    let mut named_key_map = FlatNamedKeyMap::new(num_keys, num_layouts);
    for layout_idx in 0..num_layouts {
        for lvl in 0..MAX_LEVELS {
            for kc in min_keycode..=max_keycode {
                let syms = keymap.key_get_syms_by_level(kc, layout_idx as u32, lvl as u32);
                if let Some(&sym) = syms.first() {
                    let evdev = kc - EVDEV_OFFSET;
                    if sym != 0 {
                        named_key_map.set(layout_idx, lvl, evdev, keysym_to_named_key(sym));
                    }
                    if let Some(ch) = keysym::keysym_to_char(sym) {
                        level_exceptions_keymap.set(layout_idx, lvl, evdev, ch);
                    }
                }
            }
        }
    }

    let get_char =
        |kc: u32, state: &keymap::State, layout_idx: usize, lvl: usize| -> Option<char> {
            match state.key_get_one_sym(kc) {
                Some(sym) => keysym::keysym_to_char(sym),
                None => keymap
                    .key_get_syms_by_level(kc, layout_idx as u32, lvl as u32)
                    .first()
                    .and_then(|&s| keysym::keysym_to_char(s)),
            }
        };

    let mut state_keymap = FlatKeymap::new(num_keys, num_layouts);
    for layout_idx in 0..num_layouts {
        for lvl in 0..MAX_LEVELS {
            if let Some(mut st) = keymap.new_state() {
                // Set the layout group on the state before querying.
                if layout_idx > 0 {
                    st.update_mask(0, 0, 0, 0, 0, layout_idx as u32);
                }
                press_level_modifiers(&mut st, lvl, level_keys.0, level_keys.1, level_keys.2);
                for kc in min_keycode..=max_keycode {
                    if let Some(ch) = get_char(kc, &st, layout_idx, lvl) {
                        state_keymap.set(layout_idx, lvl, kc - EVDEV_OFFSET, ch);
                    }
                }
            }
        }
    }

    let populate_lock = |lock_kc: Option<u32>,
                         toggle: bool,
                         level_keys: (Option<u32>, Option<u32>, Option<u32>)|
     -> FlatKeymap {
        let mut fk = FlatKeymap::new(num_keys, num_layouts);
        if let Some(lkc) = lock_kc {
            for layout_idx in 0..num_layouts {
                for lvl in 0..MAX_LEVELS {
                    if let Some(mut st) = keymap.new_state() {
                        if layout_idx > 0 {
                            st.update_mask(0, 0, 0, 0, 0, layout_idx as u32);
                        }
                        if toggle {
                            st.update_key(lkc, shared_types::XKB_KEY_DOWN);
                            st.update_key(lkc, shared_types::XKB_KEY_UP);
                        }
                        press_level_modifiers(
                            &mut st,
                            lvl,
                            level_keys.0,
                            level_keys.1,
                            level_keys.2,
                        );
                        if !toggle {
                            st.update_key(lkc, shared_types::XKB_KEY_DOWN);
                        }
                        for kc in min_keycode..=max_keycode {
                            if let Some(ch) = get_char(kc, &st, layout_idx, lvl) {
                                fk.set(layout_idx, lvl, kc - EVDEV_OFFSET, ch);
                            }
                        }
                    }
                }
            }
        }
        fk
    };

    let caps_lock_keymap = {
        let caps_kc = level_code(&modifiers, ModType::Caps).map(|(c, _)| c + EVDEV_OFFSET);
        let mut fk = populate_lock(caps_kc, false, level_keys);
        dedup_against_state(&mut fk, &state_keymap, num_layouts);
        fk
    };

    let num_lock_keys = {
        let num_kc = level_code(&modifiers, ModType::Num).map(|(c, _)| c + EVDEV_OFFSET);
        let mut fk = populate_lock(num_kc, true, level_keys);
        dedup_against_state(&mut fk, &state_keymap, num_layouts);
        fk
    };

    let mut repeat_keys = KeyBitSet::new();
    for kc in min_keycode..=max_keycode {
        if keymap.key_repeats(kc) {
            repeat_keys.insert(kc - EVDEV_OFFSET);
        }
    }

    // Extract layout names from keymap
    let layout_names: Vec<String> = (0..num_layouts)
        .map(|i| {
            keymap
                .layout_get_name(i as u32)
                .unwrap_or_else(|| format!("Layout {}", i))
        })
        .collect();

    // Cache XKB string for Wayland client sharing
    let _ = store_keymap; // no longer cached; generated on demand

    #[cfg(feature = "compose")]
    let composer = {
        // Resolve compose locale from environment (LC_ALL > LC_CTYPE > LANG),
        // falling back to the explicit locale hint (e.g. layout name).
        let env_locale = std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LC_CTYPE"))
            .or_else(|_| std::env::var("LANG"))
            .ok();
        let compose_locale = env_locale.as_deref().or(locale);
        compose_locale
            .and_then(keymap::resolve_compose_file)
            .map(|subpath| {
                let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
                load_compose_from_path(&path)
            })
            .unwrap_or_default()
    };

    #[cfg(not(feature = "compose"))]
    let composer = Composer::new();
    WKB {
        current_layout_idx: 0,
        layout_names,
        repeat_keys,
        composer,
        modifiers,
        state_keymap,
        num_lock_keys,
        caps_lock_keymap,
        level_exceptions_keymap,
        named_key_map,
    }
}

/// Create a new WKB instance from RMLVO names.
pub(crate) fn new_from_names(
    rules: &str,
    model: &str,
    layout: &str,
    variant: &str,
    options: Option<&str>,
) -> Result<WKB, XkbError> {
    use keymap::Context;
    use shared_types::XkbRuleNames;

    let ctx = Context::new().ok_or(XkbError::ContextCreation)?;
    let rmlvo = XkbRuleNames::from_strs(rules, model, layout, variant, options.unwrap_or(""));

    let keymap = ctx
        .keymap_from_names(&rmlvo)
        .ok_or(XkbError::KeymapCompilation)?;

    Ok(build_wkb_from_keymap(&keymap, None, true))
}

/// Create a new WKB instance from a keymap string.
pub(crate) fn new_from_string(string: &str) -> Result<WKB, XkbError> {
    use keymap::Context;

    let ctx = Context::new().ok_or(XkbError::ContextCreation)?;

    let keymap = ctx
        .keymap_from_string(string)
        .ok_or(XkbError::KeymapParsing)?;

    Ok(build_wkb_from_keymap(&keymap, None, true))
}

/// Build Modifiers struct from XKB keymap
fn build_modifiers_from_keymap(
    keymap: &keymap::Keymap,
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

/// Map a [`NamedKey`] back to its XKB keysym value.
///
/// Returns `0` (NoSymbol) for [`NamedKey::Unnamed`] and for character keys
/// that don't have a canonical keysym.
pub(crate) fn named_key_to_keysym(key: NamedKey) -> u32 {
    const TABLE: &[(NamedKey, u32)] = &[
        (NamedKey::Space, 0x0020),
        (NamedKey::Enter, 0xff0d),
        (NamedKey::Tab, 0xff09),
        (NamedKey::Backspace, 0xff08),
        (NamedKey::Escape, 0xff1b),
        (NamedKey::Delete, 0xffff),
        (NamedKey::Insert, 0xff63),
        (NamedKey::ArrowLeft, 0xff51),
        (NamedKey::ArrowRight, 0xff53),
        (NamedKey::ArrowUp, 0xff52),
        (NamedKey::ArrowDown, 0xff54),
        (NamedKey::Home, 0xff50),
        (NamedKey::End, 0xff57),
        (NamedKey::PageUp, 0xff55),
        (NamedKey::PageDown, 0xff56),
        (NamedKey::F1, 0xffbe),
        (NamedKey::F2, 0xffbf),
        (NamedKey::F3, 0xffc0),
        (NamedKey::F4, 0xffc1),
        (NamedKey::F5, 0xffc2),
        (NamedKey::F6, 0xffc3),
        (NamedKey::F7, 0xffc4),
        (NamedKey::F8, 0xffc5),
        (NamedKey::F9, 0xffc6),
        (NamedKey::F10, 0xffc7),
        (NamedKey::F11, 0xffc8),
        (NamedKey::F12, 0xffc9),
        (NamedKey::F13, 0xffca),
        (NamedKey::F14, 0xffcb),
        (NamedKey::F15, 0xffcc),
        (NamedKey::F16, 0xffcd),
        (NamedKey::F17, 0xffce),
        (NamedKey::F18, 0xffcf),
        (NamedKey::F19, 0xffd0),
        (NamedKey::F20, 0xffd1),
        (NamedKey::F21, 0xffd2),
        (NamedKey::F22, 0xffd3),
        (NamedKey::F23, 0xffd4),
        (NamedKey::F24, 0xffd5),
        (NamedKey::F25, 0xffd6),
        (NamedKey::F26, 0xffd7),
        (NamedKey::F27, 0xffd8),
        (NamedKey::F28, 0xffd9),
        (NamedKey::F29, 0xffda),
        (NamedKey::F30, 0xffdb),
        (NamedKey::F31, 0xffdc),
        (NamedKey::F32, 0xffdd),
        (NamedKey::F33, 0xffde),
        (NamedKey::F34, 0xffdf),
        (NamedKey::F35, 0xffe0),
        (NamedKey::LeftShift, 0xffe1),
        (NamedKey::RightShift, 0xffe2),
        (NamedKey::LeftControl, 0xffe3),
        (NamedKey::RightControl, 0xffe4),
        (NamedKey::LeftAlt, 0xffe9),
        (NamedKey::RightAlt, 0xffea),
        (NamedKey::LeftMeta, 0xffe7),
        (NamedKey::RightMeta, 0xffe8),
        (NamedKey::LeftSuper, 0xffeb),
        (NamedKey::RightSuper, 0xffec),
        (NamedKey::LeftHyper, 0xffed),
        (NamedKey::RightHyper, 0xffee),
        (NamedKey::CapsLock, 0xffe5),
        (NamedKey::NumLock, 0xff7f),
        (NamedKey::ScrollLock, 0xff14),
        (NamedKey::PrintScreen, 0xff61),
        (NamedKey::Pause, 0xff13),
        (NamedKey::SysReq, 0xff15),
        (NamedKey::ContextMenu, 0xff67),
        (NamedKey::Power, 0x1008ff21),
        (NamedKey::PowerOff, 0x1008ff2a),
        (NamedKey::Sleep, 0x1008ff2f),
        (NamedKey::WakeUp, 0x1008ff2b),
        (NamedKey::Suspend, 0x1008ffa7),
        (NamedKey::Hibernate, 0x1008ffa8),
        (NamedKey::MediaPlay, 0x1008ff14),
        (NamedKey::MediaPause, 0x1008ff31),
        (NamedKey::MediaStop, 0x1008ff15),
        (NamedKey::MediaNextTrack, 0x1008ff17),
        (NamedKey::MediaPreviousTrack, 0x1008ff16),
        (NamedKey::VolumeUp, 0x1008ff13),
        (NamedKey::VolumeDown, 0x1008ff11),
        (NamedKey::VolumeMute, 0x1008ff12),
        (NamedKey::BrowserBack, 0x1008ff26),
        (NamedKey::BrowserForward, 0x1008ff27),
        (NamedKey::BrowserRefresh, 0x1008ff29),
        (NamedKey::BrowserHome, 0x1008ff18),
        (NamedKey::LaunchMail, 0x1008ff19),
        (NamedKey::LaunchCalculator, 0x1008ff1d),
        (NamedKey::LaunchTerminal, 0x1008ff80),
        (NamedKey::BrightnessUp, 0x1008ff02),
        (NamedKey::BrightnessDown, 0x1008ff03),
        (NamedKey::KeyboardBrightnessUp, 0x1008ff05),
        (NamedKey::KeyboardBrightnessDown, 0x1008ff06),
        (NamedKey::KanjiMode, 0xff21),
        (NamedKey::Hiragana, 0xff25),
        (NamedKey::Katakana, 0xff26),
        (NamedKey::Romaji, 0xff24),
        (NamedKey::ZenkakuHankaku, 0xff2a),
        (NamedKey::EisuToggle, 0xff30),
        (NamedKey::HangulHanja, 0xff34),
    ];
    TABLE
        .iter()
        .find(|(nk, _)| *nk == key)
        .map(|(_, ks)| *ks)
        .unwrap_or(0)
}

// Generate XKB v1 text format from WKB's flat keysym tables.

use self::keysym::keysym_get_name;

// ── Standard evdev → XKB key name table ──
// Indexed by evdev code (0-based). `None` entries use fallback `I{evdev+8:03}`.

const EVDEV_KEYNAMES: &[(u32, &str)] = &[
    (1, "ESC"),
    (2, "AE01"),
    (3, "AE02"),
    (4, "AE03"),
    (5, "AE04"),
    (6, "AE05"),
    (7, "AE06"),
    (8, "AE07"),
    (9, "AE08"),
    (10, "AE09"),
    (11, "AE10"),
    (12, "AE11"),
    (13, "AE12"),
    (14, "BKSP"),
    (15, "TAB"),
    (16, "AD01"),
    (17, "AD02"),
    (18, "AD03"),
    (19, "AD04"),
    (20, "AD05"),
    (21, "AD06"),
    (22, "AD07"),
    (23, "AD08"),
    (24, "AD09"),
    (25, "AD10"),
    (26, "AD11"),
    (27, "AD12"),
    (28, "RTRN"),
    (29, "LCTL"),
    (30, "AC01"),
    (31, "AC02"),
    (32, "AC03"),
    (33, "AC04"),
    (34, "AC05"),
    (35, "AC06"),
    (36, "AC07"),
    (37, "AC08"),
    (38, "AC09"),
    (39, "AC10"),
    (40, "AC11"),
    (41, "TLDE"),
    (42, "LFSH"),
    (43, "BKSL"),
    (44, "AB01"),
    (45, "AB02"),
    (46, "AB03"),
    (47, "AB04"),
    (48, "AB05"),
    (49, "AB06"),
    (50, "AB07"),
    (51, "AB08"),
    (52, "AB09"),
    (53, "AB10"),
    (54, "RTSH"),
    (55, "KPMU"),
    (56, "LALT"),
    (57, "SPCE"),
    (58, "CAPS"),
    (59, "FK01"),
    (60, "FK02"),
    (61, "FK03"),
    (62, "FK04"),
    (63, "FK05"),
    (64, "FK06"),
    (65, "FK07"),
    (66, "FK08"),
    (67, "FK09"),
    (68, "FK10"),
    (69, "NMLK"),
    (70, "SCLK"),
    (71, "KP7"),
    (72, "KP8"),
    (73, "KP9"),
    (74, "KPSU"),
    (75, "KP4"),
    (76, "KP5"),
    (77, "KP6"),
    (78, "KPAD"),
    (79, "KP1"),
    (80, "KP2"),
    (81, "KP3"),
    (82, "KP0"),
    (83, "KPDL"),
    (84, "LVL3"),
    (86, "LSGT"),
    (87, "FK11"),
    (88, "FK12"),
    (89, "AB11"),
    (90, "KATA"),
    (91, "HIRA"),
    (92, "HENK"),
    (93, "HKTG"),
    (94, "MUHE"),
    (95, "JPCM"),
    (96, "KPEN"),
    (97, "RCTL"),
    (98, "KPDV"),
    (99, "PRSC"),
    (100, "RALT"),
    (101, "LNFD"),
    (102, "HOME"),
    (103, "UP"),
    (104, "PGUP"),
    (105, "LEFT"),
    (106, "RGHT"),
    (107, "END"),
    (108, "DOWN"),
    (109, "PGDN"),
    (110, "INS"),
    (111, "DELE"),
    (113, "MUTE"),
    (116, "POWR"),
    (117, "KPEQ"),
    (119, "PAUS"),
    (122, "HNGL"),
    (123, "HJCV"),
    (124, "AE13"),
    (125, "LWIN"),
    (126, "RWIN"),
    (127, "COMP"),
    (128, "STOP"),
    (129, "AGAI"),
    (130, "PROP"),
    (131, "UNDO"),
    (132, "FRNT"),
    (133, "COPY"),
    (134, "OPEN"),
    (135, "PAST"),
    (136, "FIND"),
    (137, "CUT"),
    (138, "HELP"),
    (175, "MDSW"),
    (183, "FK13"),
    (184, "FK14"),
    (185, "FK15"),
    (186, "FK16"),
    (187, "FK17"),
    (188, "FK18"),
    (189, "FK19"),
    (190, "FK20"),
    (191, "FK21"),
    (192, "FK22"),
    (193, "FK23"),
    (194, "FK24"),
    (195, "LVL5"),
    (196, "ALT"),
    (197, "META"),
    (198, "SUPR"),
    (199, "HYPR"),
    // Well-known multimedia / generic keys
    (114, "VOLD"),
    (115, "VOLU"),
    (163, "NEXS"),
    (164, "PLPA"),
    (165, "PRVS"),
    (166, "STPS"),
    (172, "I180"),
    (176, "I184"),
    (177, "I185"),
    (178, "I186"),
    (179, "I187"),
    (180, "I188"),
    (181, "I189"),
    (182, "I190"),
];

/// Get the XKB key name for an evdev code.
fn evdev_to_keyname(evdev: u32) -> String {
    // Binary search or linear — table is small
    for &(code, name) in EVDEV_KEYNAMES {
        if code == evdev {
            return name.to_string();
        }
    }
    // Fallback: generic I{xkb_keycode} format
    format!("I{:03}", evdev + 8)
}

/// Determine how many levels a key actually uses across all groups.
/// Checks `named_key_map`, `level_exceptions_keymap`, and the modifier map
/// (modifier keys must be included even if they produce no named key or character).
fn key_max_level(
    named_key_map: &FlatNamedKeyMap,
    level_exceptions: &FlatKeymap,
    modifiers: &Modifiers,
    evdev: u32,
    num_layouts: usize,
) -> usize {
    let mut max_level = 0;
    for layout in 0..num_layouts {
        for level in (0..MAX_LEVELS).rev() {
            let has_named = named_key_map.get(layout, level, evdev) != NamedKey::Unnamed;
            let has_char = level_exceptions.get(layout, level, evdev).is_some();
            if has_named || has_char {
                if level + 1 > max_level {
                    max_level = level + 1;
                }
                break;
            }
        }
    }
    // Modifier keys must always be included even if named_key is Unnamed
    // and they produce no character (e.g. ISO_Level3_Shift).
    if max_level == 0 && modifiers.get(evdev).is_some() {
        max_level = 1;
    }
    max_level
}

/// Resolve the keysym for a modifier key from the modifier map.
///
/// Maps each `ModType` to its canonical keysym so the re-parsed keymap
/// gets the correct modifier interpretation.
fn modifier_keysym(modifiers: &Modifiers, evdev: u32) -> Option<u32> {
    let modifier = modifiers.get(evdev)?;
    match modifier {
        Modifier::Single(mk) => modkind_keysym(mk),
        Modifier::Leveled(map) => map.values().next().and_then(modkind_keysym),
    }
}

fn modkind_keysym(mk: &ModKind) -> Option<u32> {
    match mk {
        ModKind::Pressed {
            mod_type: ModType::Level3,
            ..
        } => Some(0xfe03),
        ModKind::Pressed {
            mod_type: ModType::Level5,
            ..
        } => Some(0xfe11),
        ModKind::Latch {
            mod_type: ModType::Level3,
            ..
        } => Some(0xfe04),
        ModKind::Latch {
            mod_type: ModType::Level5,
            ..
        } => Some(0xfe12),
        ModKind::Lock {
            mod_type: ModType::Level3,
            ..
        } => Some(0xfe0d),
        ModKind::Lock {
            mod_type: ModType::Level5,
            ..
        } => Some(0xfe13),
        _ => None,
    }
}

/// Check if a key is a letter key (has both lowercase and uppercase
/// characters at levels 0 and 1). Used to select ALPHABETIC type.
fn is_alphabetic(keymap: &FlatKeymap, evdev: u32, num_layouts: usize) -> bool {
    for layout in 0..num_layouts {
        if let (Some(ch0), Some(ch1)) = (keymap.get(layout, 0, evdev), keymap.get(layout, 1, evdev))
        {
            if ch0.is_ascii_lowercase()
                && ch1.is_ascii_uppercase()
                && ch1 == ch0.to_ascii_uppercase()
            {
                return true;
            }
        }
    }
    false
}

/// Pick a key type name based on the number of levels.
fn type_for_levels(num_levels: usize) -> &'static str {
    match num_levels {
        0 | 1 => "ONE_LEVEL",
        2 => "TWO_LEVEL",
        3 | 4 => "FOUR_LEVEL",
        _ => "EIGHT_LEVEL",
    }
}

/// Format a keysym as its XKB name, or Unicode/hex fallback.
///
/// Legacy Latin-1 keysyms (0x20-0x7E, 0xA0-0xFF) are emitted as raw hex
/// Unicode keysyms (`0x10000XX`) to match C xkbcommon's serializer output.
/// This ensures round-tripping through xkbcommon produces identical keysym
/// values.
fn sym_name(sym: u32) -> String {
    if sym == 0 {
        return "NoSymbol".to_string();
    }
    // Unicode keysyms (0x01000000+): always emit as raw hex literal.
    // C xkbcommon's serializer does the same — e.g. 0x010000d7 for ×.
    // Using a named keysym (e.g. "multiply") would re-parse to the legacy
    // value (0xd7), causing a round-trip mismatch.
    if (0x0100_0000..=0x0110_ffff).contains(&sym) {
        return format!("{:#010x}", sym);
    }
    // Named keysyms (legacy Latin-1, function keys, etc.): use the name.
    if let Some(name) = keysym_get_name(sym) {
        name.to_string()
    } else {
        format!("{:#010x}", sym)
    }
}

impl WKB {
    /// Generate XKB v1 text format string from flat keysym tables.
    ///
    /// This produces a minimal but fully valid keymap that Wayland clients
    /// can parse.
    #[cfg(feature = "xkb")]
    pub(crate) fn generate_xkb_string(&self) -> String {
        let num_layouts = self.named_key_map.num_layouts;
        let num_keys = self.named_key_map.num_keys;
        // XKB keycodes max at 255; evdev = xkb - 8, so max evdev = 247
        let max_evdev = num_keys.min(248) as u32;
        // Estimate capacity: ~40KB for a typical keymap
        let mut out = String::with_capacity(40 * 1024);

        out.push_str("xkb_keymap {\n");

        // ── xkb_keycodes ──
        self.write_keycodes(&mut out, max_evdev);

        // ── xkb_types ──
        write_types(&mut out);

        // ── xkb_compat ──
        write_compat(&mut out);

        // ── xkb_symbols ──
        self.write_symbols(&mut out, max_evdev, num_layouts);

        out.push_str("};\n");
        out
    }

    /// Resolve the keysym for a (layout, level, evdev) triple.
    ///
    /// For named keys, returns the canonical keysym via `named_key_to_keysym`.
    /// For character keys (`NamedKey::Unnamed`), falls back to the character
    /// keymaps (`level_exceptions_keymap` then `state_keymap`) and emits a
    /// Unicode keysym so that the serialized string preserves character data.
    fn resolve_keysym(&self, layout: usize, level: usize, evdev: u32) -> u32 {
        let nk = self.named_key_map.get(layout, level, evdev);
        let sym = named_key_to_keysym(nk);
        if sym != 0 {
            return sym;
        }
        // Unnamed key — recover from character keymaps.
        if let Some(ch) = self.level_exceptions_keymap.get(layout, level, evdev) {
            return 0x0100_0000 | ch as u32;
        }
        if let Some(ch) = self.state_keymap.get(layout, level, evdev) {
            return 0x0100_0000 | ch as u32;
        }
        // Modifier key — derive keysym from the modifier map so that the
        // re-parsed keymap retains the correct modifier associations.
        if level == 0 {
            if let Some(ks) = modifier_keysym(&self.modifiers, evdev) {
                return ks;
            }
        }
        0
    }

    fn write_keycodes(&self, out: &mut String, max_evdev: u32) {
        use std::fmt::Write;

        out.push_str("xkb_keycodes \"wkb\" {\n");
        writeln!(out, "\tminimum = 8;").unwrap();
        writeln!(out, "\tmaximum = {};", max_evdev + 8 - 1).unwrap();

        for evdev in 0..max_evdev {
            // Only emit keys that have at least one keysym
            if key_max_level(
                &self.named_key_map,
                &self.level_exceptions_keymap,
                &self.modifiers,
                evdev,
                self.named_key_map.num_layouts,
            ) > 0
            {
                let name = evdev_to_keyname(evdev);
                writeln!(out, "\t<{}> = {};", name, evdev + 8).unwrap();
            }
        }

        // LED indicators
        out.push_str("\tindicator 1 = \"Caps Lock\";\n");
        out.push_str("\tindicator 2 = \"Num Lock\";\n");
        out.push_str("\tindicator 3 = \"Scroll Lock\";\n");
        out.push_str("};\n\n");
    }

    fn write_symbols(&self, out: &mut String, max_evdev: u32, num_layouts: usize) {
        use std::fmt::Write;

        out.push_str("xkb_symbols \"wkb\" {\n");

        // Group names
        for (i, name) in self.layout_names.iter().enumerate() {
            writeln!(out, "\tname[{}]= \"{}\";", i + 1, name).unwrap();
        }
        out.push('\n');

        // Per-key symbols
        for evdev in 0..max_evdev {
            let max_level = key_max_level(
                &self.named_key_map,
                &self.level_exceptions_keymap,
                &self.modifiers,
                evdev,
                num_layouts,
            );
            if max_level == 0 {
                continue;
            }
            let name = evdev_to_keyname(evdev);
            let type_name =
                if max_level == 2 && is_alphabetic(&self.state_keymap, evdev, num_layouts) {
                    "ALPHABETIC"
                } else {
                    type_for_levels(max_level)
                };

            if num_layouts == 1 {
                // Single-group format
                write!(out, "\tkey <{}> {{ type= \"{}\",", name, type_name).unwrap();
                out.push_str(" [ ");
                for level in 0..max_level {
                    if level > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&sym_name(self.resolve_keysym(0, level, evdev)));
                }
                out.push_str(" ]");
                // repeat
                if self.repeat_keys.contains(evdev) {
                    out.push_str(", repeat=Yes");
                }
                out.push_str(" };\n");
            } else {
                // Multi-group format
                out.push_str("\tkey <");
                out.push_str(&name);
                out.push_str("> {\n");
                // Per-group types
                for g in 0..num_layouts {
                    // Compute per-group level count
                    let mut glevel = 0;
                    for level in (0..MAX_LEVELS).rev() {
                        if self.named_key_map.get(g, level, evdev) != NamedKey::Unnamed {
                            glevel = level + 1;
                            break;
                        }
                    }
                    let gt = if glevel.max(max_level) == 2
                        && is_alphabetic(&self.state_keymap, evdev, num_layouts)
                    {
                        "ALPHABETIC"
                    } else {
                        type_for_levels(glevel.max(max_level))
                    };
                    writeln!(out, "\t\ttype[group{}]= \"{}\",", g + 1, gt).unwrap();
                }
                // Per-group symbols
                for g in 0..num_layouts {
                    write!(out, "\t\tsymbols[{}]= [ ", g + 1).unwrap();
                    for level in 0..max_level {
                        if level > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(&sym_name(self.resolve_keysym(g, level, evdev)));
                    }
                    if g < num_layouts - 1 {
                        out.push_str(" ],\n");
                    } else {
                        out.push_str(" ]");
                    }
                }
                if self.repeat_keys.contains(evdev) {
                    out.push_str(",\n\t\trepeat=Yes");
                }
                out.push('\n');
                out.push_str("\t};\n");
            }
        }

        // modifier_map entries
        out.push_str("\tmodifier_map Shift { <LFSH> };\n");
        out.push_str("\tmodifier_map Shift { <RTSH> };\n");
        out.push_str("\tmodifier_map Lock { <CAPS> };\n");
        out.push_str("\tmodifier_map Control { <LCTL> };\n");
        out.push_str("\tmodifier_map Control { <RCTL> };\n");
        out.push_str("\tmodifier_map Mod1 { <LALT> };\n");
        out.push_str("\tmodifier_map Mod2 { <NMLK> };\n");
        out.push_str("\tmodifier_map Mod4 { <LWIN> };\n");
        out.push_str("\tmodifier_map Mod5 { <LVL3> };\n");
        out.push_str("};\n\n");
    }
}

/// Write standard key types.
fn write_types(out: &mut String) {
    out.push_str(include_str!("data/types.xkb"));
}

/// Write a minimal but valid xkb_compat section.
fn write_compat(out: &mut String) {
    out.push_str(include_str!("data/compat.xkb"));
}
