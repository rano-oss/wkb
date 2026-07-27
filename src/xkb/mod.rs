//! XKB module — keymap construction from RMLVO names and XKB strings,
//! plus XKB v1 text serialization.

pub(crate) mod keymap;
pub(crate) mod keysym;
#[cfg(feature = "testing")]
pub(crate) mod rxkb;
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

/// Resolve the character for a key given a modifier mask, bypassing the state machine.
fn resolve_char(
    keymap: &keymap::Keymap,
    kc: u32,
    layout: u32,
    mods_mask: u32,
    caps_mask: u32,
) -> Option<char> {
    let key = keymap.inner.get_key(kc)?;
    let group = key.groups.get(layout as usize)?;
    let type_ = keymap.inner.types.get(group.type_idx as usize)?;

    let level_mods = mods_mask & type_.mods.mask;
    let level = type_
        .entries
        .iter()
        .find(|e| shared_types::entry_is_active(e) && e.mods.mask == level_mods)
        .map(|e| e.level)
        .unwrap_or(0);

    let level_data = keymap.inner.get_key_level(key, layout, level)?;
    let raw_sym = *level_data.syms.first()?;

    let sym = if mods_mask & caps_mask != 0
        && !caps_is_consumed(keymap, kc, layout, mods_mask, caps_mask)
    {
        keysym::xkb_keysym_to_upper(raw_sym)
    } else {
        raw_sym
    };
    keysym::keysym_to_char(sym)
}

/// Check if the Caps modifier is consumed by a key's type at the given modifier state.
fn caps_is_consumed(
    keymap: &keymap::Keymap,
    kc: u32,
    layout: u32,
    mods_mask: u32,
    caps_mask: u32,
) -> bool {
    if caps_mask == 0 {
        return true;
    }
    let inner = &keymap.inner;
    let key = match inner.get_key(kc) {
        Some(k) => k,
        None => return true,
    };
    let group = match key.groups.get(layout as usize) {
        Some(g) => g,
        None => return true,
    };
    let type_ = match inner.types.get(group.type_idx as usize) {
        Some(t) => t,
        None => return true,
    };
    let level_mods = mods_mask & type_.mods.mask;
    let entry = type_
        .entries
        .iter()
        .find(|e| shared_types::entry_is_active(e) && e.mods.mask == level_mods);
    let preserve = entry.map(|e| e.preserve.mask).unwrap_or(0);
    let consumed = type_.mods.mask & !preserve;
    (consumed & caps_mask) != 0
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
        (0xfe20, NamedKey::Tab),
        (0xfe34, NamedKey::Enter),
        (0xff08, NamedKey::Backspace),
        (0xff09, NamedKey::Tab),
        (0xff0d, NamedKey::Enter),
        (0xff13, NamedKey::Pause),
        (0xff14, NamedKey::ScrollLock),
        (0xff15, NamedKey::SysReq),
        (0xff1b, NamedKey::Escape),
        (0xff21, NamedKey::KanjiMode),
        (0xff24, NamedKey::Romaji),
        (0xff25, NamedKey::Hiragana),
        (0xff26, NamedKey::Katakana),
        (0xff2a, NamedKey::ZenkakuHankaku),
        (0xff30, NamedKey::EisuToggle),
        (0xff34, NamedKey::HangulHanja),
        (0xff50, NamedKey::Home),
        (0xff51, NamedKey::ArrowLeft),
        (0xff52, NamedKey::ArrowUp),
        (0xff53, NamedKey::ArrowRight),
        (0xff54, NamedKey::ArrowDown),
        (0xff55, NamedKey::PageUp),
        (0xff56, NamedKey::PageDown),
        (0xff57, NamedKey::End),
        (0xff61, NamedKey::PrintScreen),
        (0xff63, NamedKey::Insert),
        (0xff67, NamedKey::ContextMenu),
        (0xff7f, NamedKey::NumLock),
        (0xff80, NamedKey::Space),
        (0xff89, NamedKey::Tab),
        (0xff8d, NamedKey::Enter),
        (0xff95, NamedKey::Home),
        (0xff96, NamedKey::ArrowLeft),
        (0xff97, NamedKey::ArrowUp),
        (0xff98, NamedKey::ArrowRight),
        (0xff99, NamedKey::ArrowDown),
        (0xff9a, NamedKey::PageUp),
        (0xff9b, NamedKey::PageDown),
        (0xff9c, NamedKey::End),
        (0xff9e, NamedKey::Insert),
        (0xff9f, NamedKey::Delete),
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
        (0xffe5, NamedKey::CapsLock),
        (0xffe7, NamedKey::LeftMeta),
        (0xffe8, NamedKey::RightMeta),
        (0xffe9, NamedKey::LeftAlt),
        (0xffea, NamedKey::RightAlt),
        (0xffeb, NamedKey::LeftSuper),
        (0xffec, NamedKey::RightSuper),
        (0xffed, NamedKey::LeftHyper),
        (0xffee, NamedKey::RightHyper),
        (0xffff, NamedKey::Delete),
        (0x1008ff02, NamedKey::BrightnessUp),
        (0x1008ff03, NamedKey::BrightnessDown),
        (0x1008ff05, NamedKey::KeyboardBrightnessUp),
        (0x1008ff06, NamedKey::KeyboardBrightnessDown),
        (0x1008ff11, NamedKey::VolumeDown),
        (0x1008ff12, NamedKey::VolumeMute),
        (0x1008ff13, NamedKey::VolumeUp),
        (0x1008ff14, NamedKey::MediaPlay),
        (0x1008ff15, NamedKey::MediaStop),
        (0x1008ff16, NamedKey::MediaPreviousTrack),
        (0x1008ff17, NamedKey::MediaNextTrack),
        (0x1008ff18, NamedKey::BrowserHome),
        (0x1008ff19, NamedKey::LaunchMail),
        (0x1008ff1d, NamedKey::LaunchCalculator),
        (0x1008ff21, NamedKey::Power),
        (0x1008ff26, NamedKey::BrowserBack),
        (0x1008ff27, NamedKey::BrowserForward),
        (0x1008ff29, NamedKey::BrowserRefresh),
        (0x1008ff2a, NamedKey::PowerOff),
        (0x1008ff2b, NamedKey::WakeUp),
        (0x1008ff2f, NamedKey::Sleep),
        (0x1008ff31, NamedKey::MediaPause),
        (0x1008ff80, NamedKey::LaunchTerminal),
        (0x1008ffa7, NamedKey::Suspend),
        (0x1008ffa8, NamedKey::Hibernate),
    ];
    if (0xfe50..=0xfe8d).contains(&keysym) {
        return NamedKey::Unnamed;
    }
    TABLE
        .binary_search_by_key(&keysym, |&(ks, _)| ks)
        .ok()
        .map(|i| TABLE[i].1)
        .unwrap_or(NamedKey::Unnamed)
}

/// Check if a key could produce a different keysym when Caps modifier is active.
/// Compares KEYSYMS across levels: if any level differs from level 0, the
/// effective level under Caps will produce a different result.
fn key_affected_by_caps(keymap: &keymap::Keymap, kc: u32, layout: usize) -> bool {
    let n = keymap.num_levels_for_key(kc, layout as u32) as usize;
    if n == 0 {
        return false;
    }
    let l0 = keymap.key_get_syms_by_level(kc, layout as u32, 0);
    let Some(&l0_sym) = l0.first() else {
        return false;
    };
    for i in 1..n {
        let syms = keymap.key_get_syms_by_level(kc, layout as u32, i as u32);
        match syms.first() {
            Some(&s) if s != l0_sym => return true,
            None => return true,
            _ => {}
        }
    }
    // All levels same sym → check caps transformation (upper/lower case)
    keysym::xkb_keysym_to_upper(l0_sym) != l0_sym
}

/// Check if Num modifier (Mod2) could affect a key's level resolution.
/// Uses entry-level masks (not type-level aggregate) — matches xkbcommon's
/// actual level-matching logic.
fn key_affected_by_num(keymap: &keymap::Keymap, kc: u32, layout: usize) -> bool {
    const MOD2_MASK: u32 = 1 << 4;
    let key = match keymap.inner.get_key(kc) {
        Some(k) => k,
        None => return false,
    };
    let group = match key.groups.get(layout) {
        Some(g) => g,
        None => return false,
    };
    let Some(type_) = keymap.inner.types.get(group.type_idx as usize) else {
        return false;
    };
    type_.entries.iter().any(|e| (e.mods.mask & MOD2_MASK) != 0)
}

/// Check if the lock key produces the expected lock keysym at a given modifier state.
/// The lock key may not always produce its lock keysym (e.g. Eisu_Toggle at level 0
/// vs Caps_Lock at level 1 on jp locale).
fn lock_activates(keymap: &keymap::Keymap, lock_kc: u32, mods_mask: u32, lock_keysym: u32) -> bool {
    let key = match keymap.inner.get_key(lock_kc) {
        Some(k) => k,
        None => return false,
    };
    let group = match key.groups.first() {
        Some(g) => g,
        None => return false,
    };
    let type_ = match keymap.inner.types.get(group.type_idx as usize) {
        Some(t) => t,
        None => return false,
    };
    let level_mods = mods_mask & type_.mods.mask;
    let level = type_
        .entries
        .iter()
        .find(|e| shared_types::entry_is_active(e) && e.mods.mask == level_mods)
        .map(|e| e.level)
        .unwrap_or(0);
    let level_data = match keymap.inner.get_key_level(key, 0, level) {
        Some(l) => l,
        None => return false,
    };
    level_data.syms.first() == Some(&lock_keysym)
}

/// Build a lock keymap using direct char resolution: compute the character for
/// each (layout, level) cell, adjusting modifier state based on whether the
/// lock key would actually activate at each level.  Sets `None` when the result
/// matches `state_keymap` (inline dedup).
/// Whether the layout activates LevelFive when both LevelThree and Shift
/// are held (via xkbcommon's compat interpret system).  True only when a
/// MODIFIER key (Shift or Level3) produces ISO_Level5_Shift (0xfe11) or
/// ISO_Level5_Latch (0xfe12) at a level reachable while the other modifier
/// is active — e.g., the Shift key producing ISO_Level5_Latch at level 2
/// (the LVL3-only level).  Regular typing keys like AC04 = F that happen to
/// have ISO_Level5_Latch at level 2 do NOT count — they activate LevelFive
/// per-key, not globally for every Shift+LVL3 keypress.
fn layout_has_level5_activation(
    keymap: &keymap::Keymap,
    layout_idx: usize,
    level5_mask: u32,
) -> bool {
    if level5_mask == 0 {
        return false;
    }
    let is_modifier_key = |sym0: u32| -> bool { matches!(sym0, 0xFFE1 | 0xFFE2 | 0xFE03) };
    for kc in keymap.min_keycode()..=keymap.max_keycode() {
        if let Some(k) = keymap.inner.get_key(kc) {
            if k.groups.get(layout_idx).is_some() {
                let num_levels = keymap.inner.key_num_levels(k, layout_idx as u32);
                let level0_sym = keymap
                    .inner
                    .get_key_level(k, layout_idx as u32, 0)
                    .and_then(|ld| ld.syms.first().copied())
                    .unwrap_or(0);
                if !is_modifier_key(level0_sym) {
                    continue;
                }
                for level in 1..num_levels {
                    if let Some(ld) = keymap.inner.get_key_level(k, layout_idx as u32, level) {
                        if ld.syms.iter().any(|&sym| matches!(sym, 0xfe11 | 0xfe12)) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Apply xkbcommon's state-level transformation: on layouts where the
/// interpret system activates LevelFive + consumes Shift when both
/// LevelThree and Shift are held (see `layout_has_level5_activation`),
/// transform the effective modifier mask to match what the state machine
/// actually produces.
fn level5_transform_mods(
    mods_mask: u32,
    layout_has_level5: bool,
    level2_mask: u32,
    level3_mask: u32,
    level5_mask: u32,
) -> u32 {
    if layout_has_level5 && mods_mask & level2_mask != 0 && mods_mask & level3_mask != 0 {
        (mods_mask | level5_mask) & !level2_mask
    } else {
        mods_mask
    }
}

fn build_lock_keymap(
    keymap: &keymap::Keymap,
    state_keymap: &FlatKeymap,
    lock_kc: u32,
    lock_mask: u32,
    lock_keysym: u32,
    affected_by: fn(&keymap::Keymap, u32, usize) -> bool,
    num_keys: usize,
    num_layouts: usize,
    min_keycode: u32,
    max_keycode: u32,
    level_masks: &[u32; 8],
    caps_mask: u32,
    per_layout_level5: &[bool],
    level2_mask: u32,
    level3_mask: u32,
    level5_mask: u32,
) -> FlatKeymap {
    const EVDEV_OFFSET: u32 = 8;
    let mut fk = FlatKeymap::new(num_keys, num_layouts);
    let stride = MAX_LEVELS * num_keys;
    for layout_idx in 0..num_layouts {
        let layout_off = layout_idx * stride;
        let layout_level5 = per_layout_level5[layout_idx];
        for lvl in 0..MAX_LEVELS {
            let lock_active = lock_activates(keymap, lock_kc, level_masks[lvl], lock_keysym);
            let raw_mods = if lock_active {
                lock_mask | level_masks[lvl]
            } else {
                level_masks[lvl]
            };
            let mods_mask = level5_transform_mods(
                raw_mods,
                layout_level5,
                level2_mask,
                level3_mask,
                level5_mask,
            );
            let lvl_off = layout_off + lvl * num_keys;
            for kc in min_keycode..=max_keycode {
                let evdev = (kc - EVDEV_OFFSET) as usize;
                if !affected_by(keymap, kc, layout_idx) {
                    continue;
                }
                if let Some(ch) = resolve_char(keymap, kc, layout_idx as u32, mods_mask, caps_mask)
                {
                    if state_keymap.data[lvl_off + evdev] == Some(ch) {
                        fk.data[lvl_off + evdev] = None;
                    } else {
                        fk.data[lvl_off + evdev] = Some(ch);
                    }
                }
            }
        }
    }
    fk
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

    // Precompute modifier masks for direct level resolution.
    // The name-to-modifier-type mapping mirrors build_modifiers_from_keymap.
    let caps_mask = keymap.mod_get_mask("Lock");
    let num_mask = keymap.mod_get_mask("Mod2");
    let level2_mask = keymap.mod_get_mask("Shift");
    let level3_mask = {
        let m = keymap.mod_get_mask("ISO_Level3_Shift");
        if m != 0 {
            m
        } else {
            let m = keymap.mod_get_mask("Mode_switch");
            if m != 0 {
                m
            } else {
                keymap.mod_get_mask("Mod5")
            }
        }
    };
    let level5_mask = {
        let m = keymap.mod_get_mask("ISO_Level5_Shift");
        if m != 0 {
            m
        } else {
            keymap.mod_get_mask("LevelFive")
        }
    };
    let level_masks: [u32; 8] = [
        0,
        level2_mask,
        level3_mask,
        level2_mask | level3_mask,
        level5_mask,
        level2_mask | level5_mask,
        level3_mask | level5_mask,
        level2_mask | level3_mask | level5_mask,
    ];

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

    // Compute per-layout LevelFive activation.  When true, xkbcommon's
    // state machine transforms Shift+LVL3 → LevelFive+LVL3 (removing Shift).
    let per_layout_level5: Vec<bool> = (0..num_layouts)
        .map(|l| layout_has_level5_activation(keymap, l, level5_mask))
        .collect();

    // Build state_keymap using direct char resolution, applying the same
    // state-level transformation that xkbcommon's interpret system performs
    // (LevelFive activation on Shift+LVL3 for compatible layouts).
    let mut state_keymap = FlatKeymap::new(num_keys, num_layouts);
    let stride = MAX_LEVELS * num_keys;
    for layout_idx in 0..num_layouts {
        let layout_off = layout_idx * stride;
        let layout_level5 = per_layout_level5[layout_idx];
        for lvl in 0..MAX_LEVELS {
            let raw_mods = level_masks[lvl];
            let mods_mask = level5_transform_mods(
                raw_mods,
                layout_level5,
                level2_mask,
                level3_mask,
                level5_mask,
            );
            let lvl_off = layout_off + lvl * num_keys;
            for kc in min_keycode..=max_keycode {
                let evdev = (kc - EVDEV_OFFSET) as usize;
                if let Some(ch) = resolve_char(keymap, kc, layout_idx as u32, mods_mask, 0) {
                    state_keymap.data[lvl_off + evdev] = Some(ch);
                }
            }
        }
    }
    let caps_kc = level_code(&modifiers, ModType::Caps).map(|(c, _)| c + EVDEV_OFFSET);
    let caps_lock_keymap = if let Some(lock_kc) = caps_kc {
        build_lock_keymap(
            keymap,
            &state_keymap,
            lock_kc,
            caps_mask,
            0xffe5,
            key_affected_by_caps,
            num_keys,
            num_layouts,
            min_keycode,
            max_keycode,
            &level_masks,
            caps_mask,
            &per_layout_level5,
            level2_mask,
            level3_mask,
            level5_mask,
        )
    } else {
        FlatKeymap::new(num_keys, num_layouts)
    };
    let num_kc = level_code(&modifiers, ModType::Num).map(|(c, _)| c + EVDEV_OFFSET);
    let num_lock_keys = if let Some(lock_kc) = num_kc {
        build_lock_keymap(
            keymap,
            &state_keymap,
            lock_kc,
            num_mask,
            0xff7f,
            key_affected_by_num,
            num_keys,
            num_layouts,
            min_keycode,
            max_keycode,
            &level_masks,
            0,
            &per_layout_level5,
            level2_mask,
            level3_mask,
            level5_mask,
        )
    } else {
        FlatKeymap::new(num_keys, num_layouts)
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

    let result = build_wkb_from_keymap(&keymap, None, true);
    Ok(result)
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
