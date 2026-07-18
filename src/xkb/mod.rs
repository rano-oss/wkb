//! XKB module — keymap construction from RMLVO names and XKB strings,
//! plus XKB v1 text serialization.

mod serialize;

use crate::bitset::KeyBitSet;
use crate::composer::Token;
use crate::flat_keymap::{FlatKeymap, FlatNamedKeyMap, MAX_LEVELS};
use crate::modifiers::*;
use crate::named_keys::NamedKey;
use crate::Composer;
use crate::WKB;

// ── Error type ──

/// Errors returned by XKB keymap construction.
#[derive(Debug)]
pub enum XkbError {
    /// Failed to create an XKB context.
    ContextCreation,
    /// Failed to compile keymap from RMLVO names.
    KeymapCompilation,
    /// Failed to parse keymap from string.
    KeymapParsing,
}

impl std::fmt::Display for XkbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XkbError::ContextCreation => write!(f, "Failed to create XKB context"),
            XkbError::KeymapCompilation => write!(f, "Failed to compile keymap"),
            XkbError::KeymapParsing => write!(f, "Failed to parse keymap string"),
        }
    }
}

impl std::error::Error for XkbError {}

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
/// Uses first-wins semantics to match xkbcommon behavior: if multiple
/// entries resolve to the same token sequence, only the first is kept.
pub fn load_compose_from_path(path: &std::path::Path) -> Composer {
    let mut regular = Composer::new();
    let mut seen: std::collections::HashSet<Vec<u32>> = std::collections::HashSet::new();

    let entries = xkb_core::compose::parse_compose_file(path);

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
    match keysym {
        0x0020 => NamedKey::Space,
        0xff09 => NamedKey::Tab,
        0xff08 => NamedKey::Backspace,
        0xff0d => NamedKey::Enter,
        0xff1b => NamedKey::Escape,
        0xffff => NamedKey::Delete,
        0xff63 => NamedKey::Insert,
        0xff51 => NamedKey::ArrowLeft,
        0xff53 => NamedKey::ArrowRight,
        0xff52 => NamedKey::ArrowUp,
        0xff54 => NamedKey::ArrowDown,
        0xff50 => NamedKey::Home,
        0xff57 => NamedKey::End,
        0xff55 => NamedKey::PageUp,
        0xff56 => NamedKey::PageDown,

        0xffbe => NamedKey::F1,
        0xffbf => NamedKey::F2,
        0xffc0 => NamedKey::F3,
        0xffc1 => NamedKey::F4,
        0xffc2 => NamedKey::F5,
        0xffc3 => NamedKey::F6,
        0xffc4 => NamedKey::F7,
        0xffc5 => NamedKey::F8,
        0xffc6 => NamedKey::F9,
        0xffc7 => NamedKey::F10,
        0xffc8 => NamedKey::F11,
        0xffc9 => NamedKey::F12,
        0xffca => NamedKey::F13,
        0xffcb => NamedKey::F14,
        0xffcc => NamedKey::F15,
        0xffcd => NamedKey::F16,
        0xffce => NamedKey::F17,
        0xffcf => NamedKey::F18,
        0xffd0 => NamedKey::F19,
        0xffd1 => NamedKey::F20,
        0xffd2 => NamedKey::F21,
        0xffd3 => NamedKey::F22,
        0xffd4 => NamedKey::F23,
        0xffd5 => NamedKey::F24,
        0xffd6 => NamedKey::F25,
        0xffd7 => NamedKey::F26,
        0xffd8 => NamedKey::F27,
        0xffd9 => NamedKey::F28,
        0xffda => NamedKey::F29,
        0xffdb => NamedKey::F30,
        0xffdc => NamedKey::F31,
        0xffdd => NamedKey::F32,
        0xffde => NamedKey::F33,
        0xffdf => NamedKey::F34,
        0xffe0 => NamedKey::F35,

        0xffe1 => NamedKey::LeftShift,
        0xffe2 => NamedKey::RightShift,
        0xffe3 => NamedKey::LeftControl,
        0xffe4 => NamedKey::RightControl,
        0xffe9 => NamedKey::LeftAlt,
        0xffea => NamedKey::RightAlt,
        0xffe7 => NamedKey::LeftMeta,
        0xffe8 => NamedKey::RightMeta,
        0xffeb => NamedKey::LeftSuper,
        0xffec => NamedKey::RightSuper,
        0xffed => NamedKey::LeftHyper,
        0xffee => NamedKey::RightHyper,

        0xffe5 => NamedKey::CapsLock,
        0xff7f => NamedKey::NumLock,
        0xff14 => NamedKey::ScrollLock,

        0xff61 => NamedKey::PrintScreen,
        0xff13 => NamedKey::Pause,
        0xff15 => NamedKey::SysReq,
        0xff67 => NamedKey::ContextMenu,

        0x1008ff21 => NamedKey::Power,
        0x1008ff2a => NamedKey::PowerOff,
        0x1008ff2f => NamedKey::Sleep,
        0x1008ff2b => NamedKey::WakeUp,
        0x1008ffa7 => NamedKey::Suspend,
        0x1008ffa8 => NamedKey::Hibernate,

        0x1008ff14 => NamedKey::MediaPlay,
        0x1008ff31 => NamedKey::MediaPause,
        0x1008ff15 => NamedKey::MediaStop,
        0x1008ff17 => NamedKey::MediaNextTrack,
        0x1008ff16 => NamedKey::MediaPreviousTrack,
        0x1008ff13 => NamedKey::VolumeUp,
        0x1008ff11 => NamedKey::VolumeDown,
        0x1008ff12 => NamedKey::VolumeMute,

        0x1008ff26 => NamedKey::BrowserBack,
        0x1008ff27 => NamedKey::BrowserForward,
        0x1008ff29 => NamedKey::BrowserRefresh,
        0x1008ff18 => NamedKey::BrowserHome,

        0x1008ff19 => NamedKey::LaunchMail,
        0x1008ff1d => NamedKey::LaunchCalculator,
        0x1008ff80 => NamedKey::LaunchTerminal,

        0x1008ff02 => NamedKey::BrightnessUp,
        0x1008ff03 => NamedKey::BrightnessDown,
        0x1008ff05 => NamedKey::KeyboardBrightnessUp,
        0x1008ff06 => NamedKey::KeyboardBrightnessDown,

        0xff21 => NamedKey::KanjiMode,
        0xff25 => NamedKey::Hiragana,
        0xff26 => NamedKey::Katakana,
        0xff24 => NamedKey::Romaji,
        0xff2a => NamedKey::ZenkakuHankaku,
        0xff30 => NamedKey::EisuToggle,

        0xff34 => NamedKey::HangulHanja,

        0xff80 => NamedKey::Space,
        0xff8d => NamedKey::Enter,
        0xff89 => NamedKey::Tab,
        0xff9f => NamedKey::Delete,
        0xff9e => NamedKey::Insert,
        0xff95 => NamedKey::Home,
        0xff9c => NamedKey::End,
        0xff9a => NamedKey::PageUp,
        0xff9b => NamedKey::PageDown,
        0xff97 => NamedKey::ArrowUp,
        0xff99 => NamedKey::ArrowDown,
        0xff96 => NamedKey::ArrowLeft,
        0xff98 => NamedKey::ArrowRight,

        0xfe20 => NamedKey::Tab,
        0xfe34 => NamedKey::Enter,

        0xfe50..=0xfe8d => NamedKey::Unnamed,

        _ => NamedKey::Unnamed,
    }
}

/// Build WKB instance from an XKB keymap, extracting all layouts.
fn build_wkb_from_keymap(
    keymap: &xkb_core::rust_types::Keymap,
    locale: Option<&str>,
    store_keymap: bool,
) -> WKB {
    const XKB_MAX_LEVELS: usize = 8;
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
        level2_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
        level3_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
        level5_code(&modifiers).map(|(c, _)| c + EVDEV_OFFSET),
    );

    // ── Build flat keymaps for ALL layouts ──

    // Build level_exceptions_keymap and named_key_map in a single pass
    // (both use key_get_syms_by_level, no state needed)
    let mut level_exceptions_keymap = FlatKeymap::new(num_keys, num_layouts);
    let mut named_key_map = FlatNamedKeyMap::new(num_keys, num_layouts);
    for layout_idx in 0..num_layouts {
        for lvl in 0..XKB_MAX_LEVELS {
            for kc in min_keycode..=max_keycode {
                let syms = keymap.key_get_syms_by_level(kc, layout_idx as u32, lvl as u32);
                if let Some(&sym) = syms.first() {
                    let evdev = kc - EVDEV_OFFSET;
                    if sym != 0 {
                        named_key_map.set(layout_idx, lvl, evdev, keysym_to_named_key(sym));
                    }
                    if let Some(ch) = xkb_core::keysym_utf::keysym_to_char(sym) {
                        level_exceptions_keymap.set(layout_idx, lvl, evdev, ch);
                    }
                }
            }
        }
    }

    let get_char = |kc: u32,
                    state: &xkb_core::rust_types::State,
                    layout_idx: usize,
                    lvl: usize|
     -> Option<char> {
        let sym = state.key_get_one_sym(kc);
        if sym != 0 {
            xkb_core::keysym_utf::keysym_to_char(sym)
        } else {
            keymap
                .key_get_syms_by_level(kc, layout_idx as u32, lvl as u32)
                .first()
                .and_then(|&s| xkb_core::keysym_utf::keysym_to_char(s))
        }
    };

    let mut state_keymap = FlatKeymap::new(num_keys, num_layouts);
    for layout_idx in 0..num_layouts {
        for lvl in 0..XKB_MAX_LEVELS {
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
                for lvl in 0..XKB_MAX_LEVELS {
                    if let Some(mut st) = keymap.new_state() {
                        if layout_idx > 0 {
                            st.update_mask(0, 0, 0, 0, 0, layout_idx as u32);
                        }
                        if toggle {
                            st.update_key(lkc, xkb_core::XKB_KEY_DOWN);
                            st.update_key(lkc, xkb_core::XKB_KEY_UP);
                        }
                        press_level_modifiers(
                            &mut st,
                            lvl,
                            level_keys.0,
                            level_keys.1,
                            level_keys.2,
                        );
                        if !toggle {
                            st.update_key(lkc, xkb_core::XKB_KEY_DOWN);
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
        // Remove entries identical to state_keymap (only keep diffs)
        for layout_idx in 0..num_layouts {
            for lvl in 0..XKB_MAX_LEVELS {
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
        fk
    };

    let num_lock_keys = {
        let num_kc = level_code(&modifiers, ModType::Num).map(|(c, _)| c + EVDEV_OFFSET);
        let mut fk = populate_lock(num_kc, true, level_keys);
        for layout_idx in 0..num_layouts {
            for lvl in 0..XKB_MAX_LEVELS {
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
            .and_then(xkb_core::compose::resolve_compose_file)
            .map(|subpath| {
                let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
                load_compose_from_path(&path)
            })
            .unwrap_or_else(Composer::new)
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
pub fn new_from_names(
    rules: &str,
    model: &str,
    layout: &str,
    variant: &str,
    options: Option<&str>,
) -> Result<WKB, XkbError> {
    use xkb_core::rust_types::{Context, RuleNames};

    let ctx = Context::new().ok_or(XkbError::ContextCreation)?;
    let rule_names = RuleNames {
        rules: rules.to_string(),
        model: model.to_string(),
        layout: layout.to_string(),
        variant: variant.to_string(),
        options: options.unwrap_or("").to_string(),
    };

    let keymap = ctx
        .keymap_from_names(&rule_names)
        .ok_or(XkbError::KeymapCompilation)?;

    Ok(build_wkb_from_keymap(&keymap, None, true))
}

/// Create a new WKB instance from a keymap string.
pub fn new_from_string(string: &str) -> Result<WKB, XkbError> {
    use xkb_core::rust_types::Context;

    let ctx = Context::new().ok_or(XkbError::ContextCreation)?;

    let keymap = ctx
        .keymap_from_string(string)
        .ok_or(XkbError::KeymapParsing)?;

    Ok(build_wkb_from_keymap(&keymap, None, true))
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

/// Map a [`NamedKey`] back to its XKB keysym value.
///
/// Returns `0` (NoSymbol) for [`NamedKey::Unnamed`] and for character keys
/// that don't have a canonical keysym.
pub(crate) fn named_key_to_keysym(key: NamedKey) -> u32 {
    use crate::keysyms as ks;
    match key {
        NamedKey::Unnamed => 0,

        // Navigation and editing
        NamedKey::Space => ks::space,
        NamedKey::Enter => ks::Return,
        NamedKey::Tab => ks::Tab,
        NamedKey::Backspace => ks::BackSpace,
        NamedKey::Escape => ks::Escape,
        NamedKey::Delete => ks::Delete,
        NamedKey::Insert => ks::Insert,
        NamedKey::ArrowLeft => ks::Left,
        NamedKey::ArrowRight => ks::Right,
        NamedKey::ArrowUp => ks::Up,
        NamedKey::ArrowDown => ks::Down,
        NamedKey::Home => ks::Home,
        NamedKey::End => ks::End,
        NamedKey::PageUp => ks::Prior,
        NamedKey::PageDown => ks::Next,

        // Function keys
        NamedKey::F1 => ks::F1,
        NamedKey::F2 => ks::F2,
        NamedKey::F3 => ks::F3,
        NamedKey::F4 => ks::F4,
        NamedKey::F5 => ks::F5,
        NamedKey::F6 => ks::F6,
        NamedKey::F7 => ks::F7,
        NamedKey::F8 => ks::F8,
        NamedKey::F9 => ks::F9,
        NamedKey::F10 => ks::F10,
        NamedKey::F11 => ks::F11,
        NamedKey::F12 => ks::F12,
        NamedKey::F13 => ks::F13,
        NamedKey::F14 => ks::F14,
        NamedKey::F15 => ks::F15,
        NamedKey::F16 => ks::F16,
        NamedKey::F17 => ks::F17,
        NamedKey::F18 => ks::F18,
        NamedKey::F19 => ks::F19,
        NamedKey::F20 => ks::F20,
        NamedKey::F21 => ks::F21,
        NamedKey::F22 => ks::F22,
        NamedKey::F23 => ks::F23,
        NamedKey::F24 => ks::F24,
        NamedKey::F25 => ks::F25,
        NamedKey::F26 => ks::F26,
        NamedKey::F27 => ks::F27,
        NamedKey::F28 => ks::F28,
        NamedKey::F29 => ks::F29,
        NamedKey::F30 => ks::F30,
        NamedKey::F31 => ks::F31,
        NamedKey::F32 => ks::F32,
        NamedKey::F33 => ks::F33,
        NamedKey::F34 => ks::F34,
        NamedKey::F35 => ks::F35,

        // Modifiers
        NamedKey::LeftShift => ks::Shift_L,
        NamedKey::RightShift => ks::Shift_R,
        NamedKey::LeftControl => ks::Control_L,
        NamedKey::RightControl => ks::Control_R,
        NamedKey::LeftAlt => ks::Alt_L,
        NamedKey::RightAlt => ks::Alt_R,
        NamedKey::LeftMeta => ks::Meta_L,
        NamedKey::RightMeta => ks::Meta_R,
        NamedKey::LeftSuper => ks::Super_L,
        NamedKey::RightSuper => ks::Super_R,
        NamedKey::LeftHyper => ks::Hyper_L,
        NamedKey::RightHyper => ks::Hyper_R,

        // Locks
        NamedKey::CapsLock => ks::Caps_Lock,
        NamedKey::NumLock => ks::Num_Lock,
        NamedKey::ScrollLock => ks::Scroll_Lock,

        // System
        NamedKey::PrintScreen => ks::Print,
        NamedKey::Pause => ks::Pause,
        NamedKey::SysReq => ks::Sys_Req,
        NamedKey::ContextMenu => ks::Menu,

        // Power (XF86)
        NamedKey::Power => 0x1008ff21,
        NamedKey::PowerOff => 0x1008ff2a,
        NamedKey::Sleep => 0x1008ff2f,
        NamedKey::WakeUp => 0x1008ff2b,
        NamedKey::Suspend => 0x1008ffa7,
        NamedKey::Hibernate => 0x1008ffa8,

        // Media (XF86)
        NamedKey::MediaPlay => 0x1008ff14,
        NamedKey::MediaPause => 0x1008ff31,
        NamedKey::MediaStop => 0x1008ff15,
        NamedKey::MediaNextTrack => 0x1008ff17,
        NamedKey::MediaPreviousTrack => 0x1008ff16,
        NamedKey::VolumeUp => 0x1008ff13,
        NamedKey::VolumeDown => 0x1008ff11,
        NamedKey::VolumeMute => 0x1008ff12,

        // Browser (XF86)
        NamedKey::BrowserBack => 0x1008ff26,
        NamedKey::BrowserForward => 0x1008ff27,
        NamedKey::BrowserRefresh => 0x1008ff29,
        NamedKey::BrowserHome => 0x1008ff18,

        // Launch (XF86)
        NamedKey::LaunchMail => 0x1008ff19,
        NamedKey::LaunchCalculator => 0x1008ff1d,
        NamedKey::LaunchTerminal => 0x1008ff80,

        // Display (XF86)
        NamedKey::BrightnessUp => 0x1008ff02,
        NamedKey::BrightnessDown => 0x1008ff03,
        NamedKey::KeyboardBrightnessUp => 0x1008ff05,
        NamedKey::KeyboardBrightnessDown => 0x1008ff06,

        // Japanese input
        NamedKey::KanjiMode => ks::Kanji,
        NamedKey::Hiragana => ks::Hiragana,
        NamedKey::Katakana => 0xff26,
        NamedKey::Romaji => ks::Romaji,
        NamedKey::ZenkakuHankaku => ks::Zenkaku_Hankaku,
        NamedKey::EisuToggle => ks::Eisu_toggle,

        // Korean input
        NamedKey::HangulHanja => 0xff34,
    }
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

    #[test]
    fn test_multi_layout_wkb() {
        let wkb = crate::WKB::new_from_names("", "", "us,fr", "", None).unwrap();
        assert_eq!(wkb.num_layouts(), 2);
        assert!(wkb.layout_name(0).is_some());
        assert!(wkb.layout_name(1).is_some());
        assert_eq!(wkb.active_layout_idx(), 0);
    }

    #[test]
    fn test_layout_switching() {
        let mut wkb = crate::WKB::new_from_names("", "", "us,fr", "", None).unwrap();
        assert_eq!(wkb.active_layout_idx(), 0);

        wkb.set_layout(1).unwrap();
        assert_eq!(wkb.active_layout_idx(), 1);

        assert!(wkb.set_layout(5).is_err());
    }

    #[test]
    fn test_xkb_string_roundtrip() {
        // Test 1: single layout roundtrip
        let wkb = crate::WKB::new_from_names("", "", "us", "", None).unwrap();
        let s = wkb.as_xkb_string().expect("should have xkb string");
        let wkb2 = crate::WKB::new_from_string(&s).unwrap();
        assert_eq!(wkb2.num_layouts(), 1);

        // Test 2: multi-layout roundtrip
        let wkb = crate::WKB::new_from_names("", "", "us,fr", "", None).unwrap();
        let s = wkb.as_xkb_string().expect("should have xkb string");
        let wkb2 = crate::WKB::new_from_string(&s).unwrap();
        assert_eq!(wkb2.num_layouts(), 2);

        // Test 2: multi-layout roundtrip
        let wkb = crate::WKB::new_from_names("", "", "us,fr", "", None).unwrap();
        let s = wkb.as_xkb_string().expect("should have xkb string");
        let wkb2 = crate::WKB::new_from_string(&s).unwrap();
        assert_eq!(wkb2.num_layouts(), 2);
    }

    #[test]
    fn test_keysym_flat_table() {
        use crate::named_keys::NamedKey;

        let wkb = crate::WKB::new_from_names("", "", "us", "", None).unwrap();

        // 'a' key (evdev 30) at level 0: character key → Unnamed
        let key = wkb.level_named_key(30, 0, 0);
        assert_eq!(key, NamedKey::Unnamed, "character key should be Unnamed");

        // 'a' key at level 1 (shifted): character key → Unnamed
        let key = wkb.level_named_key(30, 0, 1);
        assert_eq!(
            key,
            NamedKey::Unnamed,
            "shifted character key should be Unnamed"
        );

        // Return key (evdev 28) → Enter
        let key = wkb.level_named_key(28, 0, 0);
        assert_eq!(key, NamedKey::Enter, "expected Enter for evdev 28");

        // Escape key (evdev 1) → Escape
        let key = wkb.level_named_key(1, 0, 0);
        assert_eq!(key, NamedKey::Escape, "expected Escape for evdev 1");

        // state_keysym should use current modifier state (level 0 = unshifted)
        let key = wkb.state_named_key(30);
        assert_eq!(
            key,
            NamedKey::Unnamed,
            "state_keysym should return Unnamed for 'a' unshifted"
        );
    }

    #[test]
    fn test_keysym_get_name() {
        assert_eq!(
            crate::keysyms::keysym_get_name(crate::keysyms::a),
            Some("a")
        );
        assert_eq!(
            crate::keysyms::keysym_get_name(crate::keysyms::BackSpace),
            Some("BackSpace")
        );
        assert_eq!(
            crate::keysyms::keysym_get_name(crate::keysyms::Return),
            Some("Return")
        );
        assert_eq!(crate::keysyms::keysym_get_name(0xDEADBEEF), None);
    }

    #[test]
    fn test_vt_switch() {
        assert_eq!(
            crate::keysyms::vt_switch(crate::keysyms::XF86Switch_VT_1),
            Some(1)
        );
        assert_eq!(
            crate::keysyms::vt_switch(crate::keysyms::XF86Switch_VT_12),
            Some(12)
        );
        assert_eq!(crate::keysyms::vt_switch(crate::keysyms::a), None);
    }
}
