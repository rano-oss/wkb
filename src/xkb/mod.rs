//! XKB module — keymap construction from RMLVO names and XKB strings,
//! plus XKB v1 text serialization.

pub(crate) mod compose;
pub(crate) mod keymap;
pub(crate) mod keynames;
pub(crate) mod keysym;
pub(crate) mod parser;
pub(crate) mod parser_tables;
pub(crate) mod serialize;
pub(crate) mod symbols;

use crate::flat_keymap::{FlatKeymap, FlatNamedKeyMap, MAX_LEVELS};
use crate::xkb::keymap::{
    preprocess_unicode_keysyms, xkb_context_new, xkb_keymap_new_from_names,
    xkb_keymap_new_from_string,
};
use crate::xkb::parser::{
    ActionFlags, XkbAction, XKB_CONTEXT_NO_FLAGS, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_FORMAT_TEXT_V1,
};
#[cfg(not(feature = "compose"))]
use crate::Composer;
use crate::KeyBitSet;
use crate::WKB;
use crate::{modifiers::*, KBLayout};
use compose::{layout_composer, load_compose_entries};
pub use compose::{load_compose_from_path, load_compose_from_path_uncached};
pub use keynames::keysym_to_named_key;
use std::collections::BTreeMap;
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
    for (code, modifier) in modifiers.iter() {
        match modifier {
            Modifier::Single(effect) => {
                if effect.mod_kind_from_mod_type(mod_type).is_some() {
                    return Some((*code, None));
                }
            }
            Modifier::Leveled(map) => {
                for (level, effect) in map {
                    if effect.mod_kind_from_mod_type(mod_type).is_some() {
                        return Some((*code, Some(*level)));
                    }
                }
            }
        }
    }
    None
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
        group_state: GroupState::default(),
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

/// Convert an XKB group action into a [`Group`], encoding the target group
/// index or relative delta into `Group.id` (see [`Group::resolve`]).
fn group_from_action(action: &XkbAction) -> Option<Group> {
    let (g, kind) = match action {
        XkbAction::GroupSet(g) => (g, GroupKind::Set),
        XkbAction::GroupLatch(g) => (g, GroupKind::Latch),
        XkbAction::GroupLock(g) => (
            g,
            GroupKind::Lock {
                on_release: g.flags.contains(ActionFlags::LOCK_ON_RELEASE),
            },
        ),
        _ => return None,
    };
    let id = if g.flags.contains(ActionFlags::ABSOLUTE_SWITCH) {
        // Absolute 0-based layout index; must fit in the low 7 bits.
        if !(0..GROUP_RELATIVE_MARKER as i32).contains(&g.group) {
            return None;
        }
        g.group as u8
    } else {
        // Relative delta, sign-extended at runtime by `Group::resolve`.
        let low = if g.group >= 0 {
            (g.group as u8).min(0x3F)
        } else {
            let delta = (-g.group as u8).min(0x40);
            0x80u8.wrapping_sub(delta)
        };
        GROUP_RELATIVE_MARKER | low
    };
    Some(Group::new(
        id,
        kind,
        g.flags.contains(ActionFlags::LOCK_CLEAR),
        g.flags.contains(ActionFlags::LATCH_TO_LOCK),
    ))
}

/// Combine a base modifier effect with an optional group action at a level.
fn combine_effect(mut effect: KeyEffect, group: Option<&Group>) -> KeyEffect {
    if let Some(group) = group {
        effect.group = Some(*group);
        // A no-op modifier slot (e.g. the Caps level below a `grp:caps_toggle`
        // group level) becomes a pure group switch.
        if effect
            .modifier
            .as_ref()
            .is_some_and(|state| matches!(state.kind, ModKind::None))
        {
            effect.modifier = None;
        }
    }
    effect
}

/// Merge per-level group actions into a modifier, producing `Group`/`Dual`
/// effects where a key both maps to a modifier and switches group.
fn apply_groups(modifier: Modifier, groups: &BTreeMap<u8, Group>) -> Modifier {
    if groups.is_empty() {
        return modifier;
    }
    match modifier {
        Modifier::Single(effect) => {
            let only_at_0 = groups.len() == 1 && groups.contains_key(&0);
            if only_at_0 {
                Modifier::Single(combine_effect(effect, groups.get(&0)))
            } else {
                let map: BTreeMap<u8, KeyEffect> = (0..8)
                    .map(|level| (level, combine_effect(effect.clone(), groups.get(&level))))
                    .collect();
                Modifier::Leveled(map)
            }
        }
        Modifier::Leveled(map) => {
            let merged: BTreeMap<u8, KeyEffect> = (0..8)
                .map(|level| {
                    let effect = map.get(&level).cloned().unwrap_or_default();
                    (level, combine_effect(effect, groups.get(&level)))
                })
                .collect();
            Modifier::Leveled(merged)
        }
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

    let keysym_to_modkind = |ks: u32, mt: ModType| -> KeyEffect {
        let kind = match ks {
            0xffe6 | 0xfe05 | 0xfe0d | 0xfe13 => ModKind::Lock,
            0xfe04 | 0xfe12 => ModKind::Latch,
            _ => ModKind::Press,
        };
        KeyEffect::from_modifier(StateModifier::new(mt, kind))
    };

    const EVDEV_OFFSET: u32 = 8;
    for key in &keymap.keys {
        if key.keycode < EVDEV_OFFSET || key.keycode > keymap.max_key_code {
            continue;
        }
        let evdev_code = key.keycode - EVDEV_OFFSET;
        let Some(g0) = key.groups.first() else {
            continue;
        };
        let syms = g0.levels.first().map(|l| l.syms.as_slice()).unwrap_or(&[]);
        let num_levels = g0.levels.len() as u32;

        let level_groups: BTreeMap<u8, Group> = g0
            .levels
            .iter()
            .enumerate()
            .filter_map(|(li, level)| {
                level
                    .actions
                    .iter()
                    .find_map(group_from_action)
                    .map(|group| (li as u8, group))
            })
            .collect();

        if num_levels == 1 && syms.len() == 1 {
            if let Some(mt) = keysym_to_modtype(syms[0]) {
                let modifier = apply_groups(
                    Modifier::Single(keysym_to_modkind(syms[0], mt)),
                    &level_groups,
                );
                modifiers.set_modifier(evdev_code, modifier);
                continue;
            }
        }

        if key.modmap == 0 && key.vmodmap == 0 {
            if !level_groups.is_empty() {
                modifiers.set_modifier(
                    evdev_code,
                    apply_groups(Modifier::Single(KeyEffect::default()), &level_groups),
                );
            }
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
                    let level_map: std::collections::BTreeMap<u8, KeyEffect> = (0..8)
                        .map(|l| {
                            (
                                l,
                                if l < min_caps as u8 {
                                    KeyEffect::from_modifier(StateModifier::new(
                                        ModType::Caps,
                                        ModKind::None,
                                    ))
                                } else {
                                    KeyEffect::from_modifier(StateModifier::new(
                                        ModType::Caps,
                                        ModKind::Lock,
                                    ))
                                },
                            )
                        })
                        .collect();
                    modifiers.set_modifier(
                        evdev_code,
                        apply_groups(Modifier::Leveled(level_map), &level_groups),
                    );
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
                let kind = match mod_type {
                    ModType::Caps | ModType::Num | ModType::Scroll => ModKind::Lock,
                    _ => ModKind::Press,
                };
                KeyEffect::from_modifier(StateModifier::new(mod_type, kind))
            };
            modifiers.set_modifier(
                evdev_code,
                apply_groups(Modifier::Single(mod_kind), &level_groups),
            );
        }

        // Keys with a group action but no recognized modifier (e.g. the Alt
        // key under `grp:alt_shift_toggle`) still need the group switch.
        if !level_groups.is_empty() && modifiers.get(evdev_code).is_none() {
            modifiers.set_modifier(
                evdev_code,
                apply_groups(Modifier::Single(KeyEffect::default()), &level_groups),
            );
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
                    Modifier::Single(KeyEffect {
                        modifier: Some(StateModifier {
                            mod_type: ModType::None,
                            ..
                        }),
                        ..
                    }) | Modifier::Leveled(_)
                )
        });
        if !already_control {
            modifiers.set_modifier(
                code,
                Modifier::Single(KeyEffect::from_modifier(StateModifier::new(
                    ModType::None,
                    ModKind::Press,
                ))),
            );
        }
    }
    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xkb::parser::XkbRuleNames;

    fn build_mods(options: &str, layout: &str) -> Modifiers {
        let ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        let rmlvo = XkbRuleNames {
            rules: "evdev".into(),
            model: "pc105".into(),
            layout: layout.into(),
            variant: "".into(),
            options: options.into(),
        };
        let keymap = xkb_keymap_new_from_names(ctx, &rmlvo, XKB_KEYMAP_COMPILE_NO_FLAGS).unwrap();
        build_modifiers_from_keymap(&keymap)
    }

    fn single_group(m: &Modifiers, code: u32) -> Option<&Group> {
        match m.get(code)? {
            Modifier::Single(effect) => effect.group.as_ref(),
            _ => None,
        }
    }

    fn level_effect(m: &Modifiers, code: u32, level: u8) -> Option<&KeyEffect> {
        match m.get(code)? {
            Modifier::Single(effect) if level == 0 => Some(effect),
            Modifier::Leveled(map) => map.get(&level),
            _ => None,
        }
    }

    #[test]
    fn group_toggle_is_relative_lock_on_altgr() {
        let m = build_mods("grp:toggle", "us,de");
        let g = single_group(&m, ALTGR).expect("grp:toggle should put a Group on AltGr");
        assert!(g.is_relative(), "group=+1 is a relative delta");
        assert!(matches!(g.kind(), GroupKind::Lock { .. }));
        assert_eq!(g.resolve(0, 2), Some(1));
        assert_eq!(g.resolve(1, 2), Some(0));
    }

    #[test]
    fn group_switch_is_press() {
        let m = build_mods("grp:switch", "us,de");
        let g = single_group(&m, ALTGR).expect("grp:switch should put a Group on AltGr");
        assert_eq!(g.kind(), GroupKind::Set);
        assert_eq!(g.resolve(0, 2), Some(1));
    }

    #[test]
    fn ctrl_shift_toggle_is_dual_at_level_one() {
        let m = build_mods("grp:ctrl_shift_toggle", "us,de");
        for code in [LEFT_CTRL, LEFT_SHIFT, RIGHT_SHIFT, RIGHT_CTRL] {
            let effect = level_effect(&m, code, 1)
                .unwrap_or_else(|| panic!("{code} should be a group modifier at level 1"));
            let state = effect
                .modifier
                .as_ref()
                .unwrap_or_else(|| panic!("{code} should have a modifier at level 1"));
            let group = effect
                .group
                .as_ref()
                .unwrap_or_else(|| panic!("{code} should have a group action at level 1"));
            assert!(matches!(group.kind(), GroupKind::Lock { .. }));
            assert_eq!(group.resolve(0, 2), Some(1));
            let _ = state;
        }
    }

    #[test]
    fn alt_shift_toggle_groups_alt_and_shift() {
        let m = build_mods("grp:alt_shift_toggle", "us,de");
        let effect = level_effect(&m, ALT, 1)
            .unwrap_or_else(|| panic!("Alt should be a group modifier at level 1"));
        assert!(
            effect.modifier.is_none() && effect.group.is_some(),
            "Alt has no modifier: {effect:?}"
        );
        let shift =
            level_effect(&m, LEFT_SHIFT, 1).expect("Shift should be a group modifier at level 1");
        assert!(shift.modifier.is_some() && shift.group.is_some());
    }

    #[test]
    fn caps_toggle_switches_group_at_level_zero() {
        let m = build_mods("grp:caps_toggle", "us,de");
        let modifier = m.get(CAPS_LOCK).expect("caps key should be a modifier");
        let Modifier::Leveled(map) = modifier else {
            panic!("caps_toggle should produce a Leveled modifier");
        };
        assert!(
            matches!(map.get(&0), Some(effect) if effect.modifier.is_none() && effect.group.is_some()),
            "level 0 should be a pure group switch"
        );
        assert!(
            matches!(
                map.get(&1),
                Some(KeyEffect {
                    modifier: Some(StateModifier {
                        kind: ModKind::Lock,
                        mod_type: ModType::Caps,
                        ..
                    }),
                    ..
                })
            ),
            "level 1 should be Caps Lock"
        );
    }

    #[test]
    fn no_group_option_leaves_modifiers_unchanged() {
        let m = build_mods("", "us");
        assert!(m.get(ALTGR).is_none() || single_group(&m, ALTGR).is_none());
    }

    #[test]
    fn group_key_switches_layout() {
        let mut wkb = new_from_names("", "", "us,de", "", Some("grp:toggle")).unwrap();
        assert_eq!(wkb.active_layout_idx(), 0);
        wkb.update_key(ALTGR, KeyDirection::Down);
        assert_eq!(
            wkb.active_layout_idx(),
            1,
            "AltGr press should switch to layout 1"
        );
        wkb.update_key(ALTGR, KeyDirection::Up);
        assert_eq!(wkb.active_layout_idx(), 1, "release should not revert");
        wkb.update_key(ALTGR, KeyDirection::Down);
        assert_eq!(wkb.active_layout_idx(), 0, "second press toggles back");
        wkb.update_key(ALTGR, KeyDirection::Up);
        assert_eq!(wkb.active_layout_idx(), 0);
    }
}
