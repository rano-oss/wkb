//! XKB module — keymap construction from RMLVO names and XKB strings,
//! plus XKB v1 text serialization.

pub(crate) mod compose;
pub(crate) mod keymap;
pub(crate) mod keynames;
pub(crate) mod keysym;
pub(crate) mod parser;
pub(crate) mod parser_tables;
pub(crate) mod symbols;

use crate::flat_keymap::{FlatKeymap, FlatNamedKeyMap, MAX_LEVELS};
use crate::named_keys::NamedKey;
use crate::xkb::keymap::{
    preprocess_unicode_keysyms, xkb_context_new, xkb_keymap_new_from_names,
    xkb_keymap_new_from_string,
};
use crate::xkb::parser::{
    XKB_CONTEXT_NO_FLAGS, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_FORMAT_TEXT_V1,
};
#[cfg(not(feature = "compose"))]
use crate::Composer;
use crate::KeyBitSet;
use crate::WKB;
use crate::{modifiers::*, KBLayout};
use compose::{layout_composer, load_compose_entries};
pub use compose::{load_compose_from_path, load_compose_from_path_uncached};
pub use keynames::keysym_to_named_key;
use keynames::{evdev_to_keyname, named_key_to_keysym};
use std::ffi::CString;

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
                        ModKind::Press { .. } => return Some((*code, None)),
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
                            ModKind::Press { .. } => return Some((*code, Some(*level))),
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

const REAL_MOD_STATES: usize = parser::MOD_REAL_MASK_ALL as usize + 1;

#[derive(Clone, Copy, Default)]
struct CompiledTypeState {
    level: u32,
    consumed_mods: u32,
}

struct CompiledType {
    states: [CompiledTypeState; REAL_MOD_STATES],
    num_lock_affected: bool,
}

impl CompiledType {
    fn new(type_: &parser::XkbKeyType) -> Self {
        let states = std::array::from_fn(|state| {
            let level_mods = state as u32 & type_.mods.mask;
            let entry = type_.entries.iter().find(|entry| {
                (entry.mods.mods == 0 || entry.mods.mask != 0) && entry.mods.mask == level_mods
            });
            CompiledTypeState {
                level: entry.map_or(0, |entry| entry.level),
                consumed_mods: type_.mods.mask & !entry.map_or(0, |entry| entry.preserve.mask),
            }
        });
        Self {
            states,
            num_lock_affected: type_
                .entries
                .iter()
                .any(|entry| entry.mods.mask & (1 << 4) != 0),
        }
    }

    #[inline]
    fn state(&self, mods: u32) -> CompiledTypeState {
        self.states[(mods & parser::MOD_REAL_MASK_ALL) as usize]
    }
}

fn resolve_char(
    group: &parser::XkbGroup,
    type_: &CompiledType,
    mods: u32,
    caps_mask: u32,
) -> Option<char> {
    let state = type_.state(mods);
    let raw_sym = *group.levels.get(state.level as usize)?.syms.first()?;
    let sym = if mods & caps_mask != 0 && state.consumed_mods & caps_mask == 0 {
        keysym::xkb_keysym_to_upper(raw_sym)
    } else {
        raw_sym
    };
    keysym::keysym_to_char(sym)
}

/// Check if a key could produce a different keysym when Caps modifier is active.
/// Compares KEYSYMS across levels: if any level differs from level 0, the
/// effective level under Caps will produce a different result.
fn key_affected_by_caps(group: &parser::XkbGroup, num_levels: usize) -> bool {
    let Some(&l0_sym) = group.levels.first().and_then(|level| level.syms.first()) else {
        return false;
    };
    group.levels.len() < num_levels
        || group
            .levels
            .iter()
            .take(num_levels)
            .skip(1)
            .any(|level| level.syms.first() != Some(&l0_sym))
        || keysym::xkb_keysym_to_upper(l0_sym) != l0_sym
}

fn lock_activation(
    keymap: &keymap::XkbKeymap,
    types: &[CompiledType],
    lock_kc: Option<u32>,
    lock_keysym: u32,
    level_masks: &[u32; MAX_LEVELS],
) -> [bool; MAX_LEVELS] {
    let Some(group) = lock_kc
        .and_then(|kc| keymap.get_key(kc))
        .and_then(|key| key.groups.first())
    else {
        return [false; MAX_LEVELS];
    };
    let Some(type_) = types.get(group.type_idx as usize) else {
        return [false; MAX_LEVELS];
    };
    std::array::from_fn(|level| {
        group
            .levels
            .get(type_.state(level_masks[level]).level as usize)
            .and_then(|data| data.syms.first())
            == Some(&lock_keysym)
    })
}

/// Whether the layout activates LevelFive when both LevelThree and Shift
/// are held (via xkbcommon's compat interpret system).  True only when a
/// MODIFIER key (Shift or Level3) produces ISO_Level5_Shift (0xfe11) or
/// ISO_Level5_Latch (0xfe12) at a level reachable while the other modifier
/// is active — e.g., the Shift key producing ISO_Level5_Latch at level 2
/// (the LVL3-only level).  Regular typing keys like AC04 = F that happen to
/// have ISO_Level5_Latch at level 2 do NOT count — they activate LevelFive
/// per-key, not globally for every Shift+LVL3 keypress.
fn layout_has_level5_activation(
    keymap: &keymap::XkbKeymap,
    layout_idx: usize,
    level5_mask: u32,
) -> bool {
    level5_mask != 0
        && keymap
            .keys
            .iter()
            .filter_map(|key| key.groups.get(layout_idx))
            .any(|group| {
                let sym = group
                    .levels
                    .first()
                    .and_then(|level| level.syms.first())
                    .copied();
                matches!(sym, Some(0xFFE1 | 0xFFE2 | 0xFE03))
                    && group
                        .levels
                        .iter()
                        .skip(1)
                        .any(|level| level.syms.iter().any(|&sym| matches!(sym, 0xfe11 | 0xfe12)))
            })
}

/// Build WKB instance from an XKB keymap, extracting all layouts.
fn build_wkb_from_keymap(keymap: &keymap::XkbKeymap, layout_locales: Option<&str>) -> WKB {
    const EVDEV_OFFSET: u32 = 8;

    let (min_keycode, max_keycode) = (keymap.min_key_code, keymap.max_key_code);
    let min_keycode = min_keycode.max(EVDEV_OFFSET);
    let num_keys = if max_keycode >= EVDEV_OFFSET {
        (max_keycode - EVDEV_OFFSET + 1) as usize
    } else {
        0
    };
    let num_layouts = (keymap.num_groups as usize).max(1);

    // Modifiers are global to the keymap (not per-layout), use layout 0.
    let modifiers = build_modifiers_from_keymap(keymap);
    let compiled_types: Vec<_> = keymap.types.iter().map(CompiledType::new).collect();

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

    // Compute per-layout LevelFive activation before the merged flat-keymap pass.
    let per_layout_level5: Vec<bool> = (0..num_layouts)
        .map(|l| layout_has_level5_activation(keymap, l, level5_mask))
        .collect();

    let caps_kc = level_code(&modifiers, ModType::Caps).map(|(code, _)| code + EVDEV_OFFSET);
    let num_kc = level_code(&modifiers, ModType::Num).map(|(code, _)| code + EVDEV_OFFSET);
    let caps_active = lock_activation(keymap, &compiled_types, caps_kc, 0xffe5, &level_masks);
    let num_active = lock_activation(keymap, &compiled_types, num_kc, 0xff7f, &level_masks);
    let layout_states: Vec<_> = per_layout_level5
        .iter()
        .map(|&layout_level5| {
            let transform = |mods| {
                if layout_level5 && mods & level2_mask != 0 && mods & level3_mask != 0 {
                    (mods | level5_mask) & !level2_mask
                } else {
                    mods
                }
            };
            (
                std::array::from_fn::<_, MAX_LEVELS, _>(|level| transform(level_masks[level])),
                std::array::from_fn::<_, MAX_LEVELS, _>(|level| {
                    transform(level_masks[level] | (u32::from(caps_active[level]) * caps_mask))
                }),
                std::array::from_fn::<_, MAX_LEVELS, _>(|level| {
                    transform(level_masks[level] | (u32::from(num_active[level]) * num_mask))
                }),
                std::array::from_fn::<_, MAX_LEVELS, _>(|level| {
                    transform(
                        level_masks[level]
                            | (u32::from(caps_active[level]) * caps_mask)
                            | (u32::from(num_active[level]) * num_mask),
                    )
                }),
            )
        })
        .collect();

    let layout_names: Vec<String> = (0..num_layouts)
        .map(|i| {
            keymap
                .group_names
                .get(i)
                .map(|&name| keymap.ctx.atom_text(name))
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .unwrap_or_else(|| format!("Layout {}", i))
        })
        .collect();
    let locale_hints: Vec<&str> = layout_locales
        .map(|locales| locales.split(',').collect())
        .unwrap_or_default();
    #[cfg(feature = "compose")]
    let env_locale = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .ok();

    let mut layouts = Vec::with_capacity(num_layouts);
    for (layout_idx, (base_states, caps_states, num_states, combined_states)) in
        layout_states.iter().enumerate()
    {
        let mut level_exceptions_keymap = FlatKeymap::new(num_keys);
        let mut named_key_map = FlatNamedKeyMap::new(num_keys);
        let mut state_keymap = FlatKeymap::new(num_keys);
        let mut caps_lock_keymap = FlatKeymap::new(num_keys);
        let mut caps_num_lock_keys = FlatKeymap::new(num_keys);
        let mut num_lock_keys = FlatKeymap::new(num_keys);
        let mut repeat_keys = KeyBitSet::new();

        for key in &keymap.keys {
            let kc = key.keycode;
            if kc < min_keycode || kc > max_keycode {
                continue;
            }
            let evdev = (kc - EVDEV_OFFSET) as usize;
            if key.repeats {
                repeat_keys.insert(evdev as u32);
            }
            let raw_group = keymap::xkb_wrap_group_into_range(
                layout_idx as i32,
                key.num_groups,
                key.out_of_range_group_policy,
                key.out_of_range_group_number,
            )
            .and_then(|group| key.groups.get(group as usize));
            let state_group = key.groups.get(layout_idx);
            let state_type =
                state_group.and_then(|group| compiled_types.get(group.type_idx as usize));
            let (caps_affected, num_affected) = state_group
                .and_then(|group| {
                    let type_ = keymap.types.get(group.type_idx as usize)?;
                    let compiled = compiled_types.get(group.type_idx as usize)?;
                    Some((
                        key_affected_by_caps(group, type_.num_levels as usize),
                        compiled.num_lock_affected,
                    ))
                })
                .unwrap_or_default();

            for level in 0..MAX_LEVELS {
                let idx = level * num_keys + evdev;
                if let Some(&sym) = raw_group
                    .and_then(|group| group.levels.get(level))
                    .and_then(|data| data.syms.first())
                {
                    if sym != 0 {
                        named_key_map.data[idx] = keysym_to_named_key(sym);
                    }
                    if let Some(ch) = keysym::keysym_to_char(sym) {
                        level_exceptions_keymap.data[idx] = Some(ch);
                    }
                }
                let (Some(group), Some(type_)) = (state_group, state_type) else {
                    continue;
                };
                let base = resolve_char(group, type_, base_states[level], 0);
                state_keymap.data[idx] = base;
                if caps_affected {
                    if let Some(ch) = resolve_char(group, type_, caps_states[level], caps_mask) {
                        if Some(ch) != base {
                            caps_lock_keymap.data[idx] = Some(ch);
                        }
                    }
                }
                if num_affected {
                    if let Some(ch) = resolve_char(group, type_, num_states[level], 0) {
                        if Some(ch) != base {
                            num_lock_keys.data[idx] = Some(ch);
                        }
                    }
                }
                if caps_affected || num_affected {
                    if let Some(ch) = resolve_char(
                        group,
                        type_,
                        combined_states[level],
                        u32::from(caps_affected) * caps_mask,
                    ) {
                        if Some(ch) != base {
                            caps_num_lock_keys.data[idx] = Some(ch);
                        }
                    }
                }
            }
        }

        #[cfg(feature = "compose")]
        let composer = {
            let mut reachable: Vec<char> = state_keymap
                .data
                .iter()
                .chain(&caps_lock_keymap.data)
                .chain(&num_lock_keys.data)
                .chain(&caps_num_lock_keys.data)
                .filter_map(|ch| *ch)
                .collect();
            reachable.sort_unstable();
            reachable.dedup();
            let compose_locale = locale_hints
                .get(layout_idx)
                .copied()
                .filter(|locale| !locale.is_empty())
                .or(env_locale.as_deref());
            compose_locale
                .and_then(keymap::resolve_compose_file)
                .map(|subpath| {
                    let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
                    let table = load_compose_entries(&path);
                    layout_composer(&table, &reachable)
                })
                .unwrap_or_default()
        };

        #[cfg(not(feature = "compose"))]
        let composer = Composer::new();

        layouts.push(KBLayout {
            name: layout_names[layout_idx].clone(),
            repeat_keys,
            composer,
            modifiers: modifiers.clone(),
            state_keymap,
            num_lock_keys,
            caps_lock_keymap,
            caps_num_lock_keys,
            level_exceptions_keymap,
            named_key_map,
        });
    }

    WKB {
        current_layout_idx: 0,
        layouts,
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
    use parser::XkbRuleNames;

    let ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);

    let rmlvo = XkbRuleNames {
        rules: rules.into(),
        model: model.into(),
        layout: layout.into(),
        variant: variant.into(),
        options: options.unwrap_or("").into(),
    };

    let keymap = xkb_keymap_new_from_names(ctx, &rmlvo, XKB_KEYMAP_COMPILE_NO_FLAGS)
        .ok_or(XkbError::KeymapCompilation)?;

    let result = build_wkb_from_keymap(&keymap, Some(layout));
    Ok(result)
}

/// Create a new WKB instance from a keymap string.
pub(crate) fn new_from_string(string: &str) -> Result<WKB, XkbError> {
    let ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);

    let processed = preprocess_unicode_keysyms(string);
    let keymap_cstr = CString::new(processed.as_ref())
        .ok()
        .ok_or(XkbError::KeymapParsing)?;
    let keymap = xkb_keymap_new_from_string(
        ctx,
        &keymap_cstr,
        XKB_KEYMAP_FORMAT_TEXT_V1,
        XKB_KEYMAP_COMPILE_NO_FLAGS,
    )
    .ok_or(XkbError::KeymapCompilation)?;
    let keymap = keymap;

    Ok(build_wkb_from_keymap(&keymap, None))
}

fn modtype_from_name(name: &str) -> Option<ModType> {
    match name {
        "Shift" => Some(ModType::Level2),
        "ISO_Level3_Shift" | "Mode_switch" | "LevelThree" => Some(ModType::Level3),
        "ISO_Level5_Shift" | "LevelFive" => Some(ModType::Level5),
        "Lock" => Some(ModType::Caps),
        "Mod2" => Some(ModType::Num),
        "Mod5" => Some(ModType::Level3),
        "Scroll_Lock" | "ScrollLock" => Some(ModType::Scroll),
        "Control" => Some(ModType::None),
        _ => None,
    }
}

/// Build Modifiers struct from XKB keymap
fn build_modifiers_from_keymap(keymap: &keymap::XkbKeymap) -> Modifiers {
    let mut modifiers = Modifiers::new();
    let mod_defs: Vec<_> = keymap
        .mods
        .mods
        .iter()
        .take(keymap.mods.num_mods as usize)
        .map(|modifier| {
            (
                modifier.mapping,
                modtype_from_name(keymap.ctx.atom_text(modifier.name)),
            )
        })
        .collect();

    let keysym_to_modtype = |ks: u32| -> Option<ModType> {
        match ks {
            0xfe03 | 0xfe04 | 0xfe05 | 0xfe0d => Some(ModType::Level3),
            0xfe11..=0xfe13 => Some(ModType::Level5),
            0xff20 => Some(ModType::Compose),
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
            _ => ModKind::Press {
                pressed: false,
                mod_type: mt,
            },
        }
    };

    const EVDEV_OFFSET: u32 = 8;
    for key in &keymap.keys {
        if key.keycode < EVDEV_OFFSET {
            continue;
        }
        let evdev_code = key.keycode - EVDEV_OFFSET;
        let Some(g0) = key.groups.first() else {
            continue;
        };
        let syms = g0.levels.first().map(|l| l.syms.as_slice()).unwrap_or(&[]);
        let num_levels = g0.levels.len() as u32;

        if num_levels == 1 && syms.len() == 1 {
            if let Some(mt) = keysym_to_modtype(syms[0]) {
                modifiers
                    .set_modifier(evdev_code, Modifier::Single(keysym_to_modkind(syms[0], mt)));
                continue;
            }
        }

        if key.modmap == 0 && key.vmodmap == 0 {
            continue;
        }

        for &(mod_mask, named_type) in &mod_defs {
            if (key.modmap & mod_mask) == 0 && (key.vmodmap & mod_mask) == 0 {
                continue;
            }

            let mod_type = if syms.len() == 1 {
                keysym_to_modtype(syms[0]).or(named_type)
            } else {
                named_type
            };
            let Some(mod_type) = mod_type else { continue };

            if mod_type == ModType::Caps {
                let caps_levels: Vec<u32> = (0..num_levels)
                    .filter(|&lvl| {
                        g0.levels.get(lvl as usize).and_then(|l| l.syms.first()) == Some(&0xffe5)
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
                    _ => ModKind::Press {
                        pressed: false,
                        mod_type,
                    },
                }
            };
            modifiers.set_modifier(evdev_code, Modifier::Single(mod_kind));
        }
    }

    // xkbcommon assigns the Control modifier to the standard LCTL/RCTL keys
    // via its default modifier map even when a layout remaps their symbols
    // (br/thinkpad, kr/kr104) or repurposes them as a level switch (ca/multix).
    // Ensure they suppress output like xkbcommon would.
    for &code in &[LEFT_CTRL, RIGHT_CTRL] {
        let already_control = modifiers.iter().any(|(c, m)| {
            *c == code
                && matches!(
                    m,
                    Modifier::Single(ModKind::Press {
                        mod_type: ModType::None,
                        ..
                    }) | Modifier::Leveled(_)
                )
        });
        if !already_control {
            modifiers.set_modifier(
                code,
                Modifier::Single(ModKind::Press {
                    pressed: false,
                    mod_type: ModType::None,
                }),
            );
        }
    }
    modifiers
}

// Generate XKB v1 text format from WKB's flat keysym tables.

use self::keysym::keysym_get_name;

/// Determine how many levels a key actually uses across all groups.
/// Checks `named_key_map`, `level_exceptions_keymap`, and the modifier map
/// (modifier keys must be included even if they produce no named key or character).
fn key_max_level(
    named_key_map: &FlatNamedKeyMap,
    level_exceptions: &FlatKeymap,
    modifiers: &Modifiers,
    evdev: u32,
) -> usize {
    let mut max_level = 0;
    for level in (0..MAX_LEVELS).rev() {
        let has_named = named_key_map.get(level, evdev) != NamedKey::Unnamed;
        let has_char = level_exceptions.get(level, evdev).is_some();
        if has_named || has_char {
            if level + 1 > max_level {
                max_level = level + 1;
            }
            break;
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
        ModKind::Press {
            mod_type: ModType::Level3,
            ..
        } => Some(0xfe03),
        ModKind::Press {
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
fn is_alphabetic(keymap: &FlatKeymap, evdev: u32) -> bool {
    matches!(
        (keymap.get(0, evdev), keymap.get(1, evdev)),
        (Some(lower), Some(upper))
            if lower.is_ascii_lowercase()
                && upper.is_ascii_uppercase()
                && upper == lower.to_ascii_uppercase()
    )
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
        let num_layouts = self.layouts.len();
        let num_keys = self
            .layouts
            .first()
            .map_or(0, |layout| layout.named_key_map.num_keys);
        // XKB keycodes max at 255; evdev = xkb - 8, so max evdev = 247
        let max_evdev = num_keys.min(248) as u32;
        // Estimate capacity: ~40KB for a typical keymap
        let mut out = String::with_capacity(40 * 1024);

        out.push_str("xkb_keymap {\n");

        // ── xkb_keycodes ──
        self.write_keycodes(&mut out, max_evdev);

        // ── xkb_types ──
        out.push_str(TYPES_XKB);

        // ── xkb_compat ──
        out.push_str(COMPAT_XKB);

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
        let layout = &self.layouts[layout];
        let nk = layout.named_key_map.get(level, evdev);
        let sym = named_key_to_keysym(nk);
        if sym != 0 {
            return sym;
        }
        // Unnamed key — recover from character keymaps.
        if let Some(ch) = layout.level_exceptions_keymap.get(level, evdev) {
            return 0x0100_0000 | ch as u32;
        }
        if let Some(ch) = layout.state_keymap.get(level, evdev) {
            return 0x0100_0000 | ch as u32;
        }
        // Modifier key — derive keysym from the modifier map so that the
        // re-parsed keymap retains the correct modifier associations.
        if level == 0 {
            if let Some(ks) = modifier_keysym(&layout.modifiers, evdev) {
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
            if self.layouts.iter().any(|layout| {
                key_max_level(
                    &layout.named_key_map,
                    &layout.level_exceptions_keymap,
                    &layout.modifiers,
                    evdev,
                ) > 0
            }) {
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
        for (i, layout) in self.layouts.iter().enumerate() {
            writeln!(out, "\tname[{}]= \"{}\";", i + 1, layout.name).unwrap();
        }
        out.push('\n');

        // Per-key symbols
        for evdev in 0..max_evdev {
            let max_level = self
                .layouts
                .iter()
                .map(|layout| {
                    key_max_level(
                        &layout.named_key_map,
                        &layout.level_exceptions_keymap,
                        &layout.modifiers,
                        evdev,
                    )
                })
                .max()
                .unwrap_or(0);
            if max_level == 0 {
                continue;
            }
            let name = evdev_to_keyname(evdev);
            let type_name = if max_level == 2
                && self
                    .layouts
                    .iter()
                    .any(|layout| is_alphabetic(&layout.state_keymap, evdev))
            {
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
                if self.layouts[0].repeat_keys.contains(evdev) {
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
                    let layout = &self.layouts[g];
                    let glevel = key_max_level(
                        &layout.named_key_map,
                        &layout.level_exceptions_keymap,
                        &layout.modifiers,
                        evdev,
                    );
                    let gt = if glevel == 2 && is_alphabetic(&layout.state_keymap, evdev) {
                        "ALPHABETIC"
                    } else {
                        type_for_levels(glevel)
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
                if self
                    .layouts
                    .iter()
                    .any(|layout| layout.repeat_keys.contains(evdev))
                {
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

const TYPES_XKB: &str = "xkb_types \"wkb\" {\n\tvirtual_modifiers NumLock,Alt,LevelThree,LevelFive;\n\n\ttype \"ONE_LEVEL\" {\n\t\tmodifiers= none;\n\t\tlevel_name[Level1]= \"Any\";\n\t};\n\ttype \"TWO_LEVEL\" {\n\t\tmodifiers= Shift;\n\t\tmap[Shift]= Level2;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t};\n\ttype \"ALPHABETIC\" {\n\t\tmodifiers= Shift+Lock;\n\t\tmap[Shift]= Level2;\n\t\tmap[Lock]= Level2;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t};\n\ttype \"FOUR_LEVEL\" {\n\t\tmodifiers= Shift+LevelThree;\n\t\tmap[Shift]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t};\n\ttype \"FOUR_LEVEL_SEMIALPHABETIC\" {\n\t\tmodifiers= Shift+Lock+LevelThree;\n\t\tmap[Shift]= Level2;\n\t\tmap[Lock]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tmap[Lock+LevelThree]= Level3;\n\t\tmap[Shift+Lock+LevelThree]= Level4;\n\t\tpreserve[Lock+LevelThree]= Lock;\n\t\tpreserve[Shift+Lock+LevelThree]= Lock;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t};\n\ttype \"EIGHT_LEVEL\" {\n\t\tmodifiers= Shift+LevelThree+LevelFive;\n\t\tmap[Shift]= Level2;\n\t\tmap[LevelThree]= Level3;\n\t\tmap[Shift+LevelThree]= Level4;\n\t\tmap[LevelFive]= Level5;\n\t\tmap[Shift+LevelFive]= Level6;\n\t\tmap[LevelThree+LevelFive]= Level7;\n\t\tmap[Shift+LevelThree+LevelFive]= Level8;\n\t\tlevel_name[Level1]= \"Base\";\n\t\tlevel_name[Level2]= \"Shift\";\n\t\tlevel_name[Level3]= \"Alt Base\";\n\t\tlevel_name[Level4]= \"Shift Alt\";\n\t\tlevel_name[Level5]= \"X1\";\n\t\tlevel_name[Level6]= \"X2\";\n\t\tlevel_name[Level7]= \"X3\";\n\t\tlevel_name[Level8]= \"X4\";\n\t};\n};\n";

const COMPAT_XKB: &str = "xkb_compat \"wkb\" {\n\tvirtual_modifiers NumLock,Alt,LevelThree,LevelFive;\n\n\tinterpret Any+AnyOf(all) {\n\t\taction= SetMods(modifiers=modMapMods,clearLocks);\n\t};\n\tinterpret Shift_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Shift,clearLocks);\n\t};\n\tinterpret Shift_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Shift,clearLocks);\n\t};\n\tinterpret Caps_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=Lock);\n\t};\n\tinterpret Num_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=NumLock);\n\t};\n\tinterpret Control_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Control,clearLocks);\n\t};\n\tinterpret Control_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Control,clearLocks);\n\t};\n\tinterpret Alt_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Alt,clearLocks);\n\t};\n\tinterpret Super_L+AnyOf(all) {\n\t\taction= SetMods(modifiers=Mod4,clearLocks);\n\t};\n\tinterpret Super_R+AnyOf(all) {\n\t\taction= SetMods(modifiers=Mod4,clearLocks);\n\t};\n\tinterpret ISO_Level3_Shift+AnyOf(all) {\n\t\taction= SetMods(modifiers=LevelThree,clearLocks);\n\t};\n\tinterpret Scroll_Lock+AnyOf(all) {\n\t\taction= LockMods(modifiers=Mod3);\n\t};\n\n\tindicator \"Caps Lock\" {\n\t\tmodifiers= Lock;\n\t};\n\tindicator \"Num Lock\" {\n\t\tmodifiers= NumLock;\n\t};\n\tindicator \"Scroll Lock\" {\n\t\tmodifiers= Mod3;\n\t};\n};\n";
