//! XKB module — keymap construction from RMLVO names and XKB strings,
//! plus XKB v1 text serialization.
#[cfg(feature = "client")]
pub(crate) mod compose;
pub(crate) mod keymap;
pub(crate) mod keynames;
pub(crate) mod keysym;
mod parse_xkb;
pub(crate) mod parser;
pub(crate) mod serialize;
pub(crate) mod symbols;
use crate::flat_keymap::{FlatKeymap, FlatNamedKeyMap, MAX_LEVELS};
use crate::xkb::keymap::{xkb_context_new, xkb_keymap_new_from_names, xkb_keymap_new_from_string};
use crate::xkb::parser::{ActionFlags, XkbAction, XkbGroupAction};
use crate::WKB;
use crate::{modifiers::*, KBLayout};
use crate::{Group, GroupChange, GroupKind, Groups, KeyBitSet};
#[cfg(feature = "client")]
pub use compose::{load_compose_from_path, load_compose_from_path_uncached};
pub use keynames::keysym_to_named_key;
use std::collections::BTreeMap;
#[derive(Debug, thiserror::Error)]
pub enum XkbError {
    #[error("Failed to create XKB context")]
    ContextCreation,
    #[error("Failed to compile keymap")]
    KeymapCompilation,
    #[error("Failed to parse keymap string")]
    KeymapParsing,
}
pub(crate) fn level_code(modifiers: &Modifiers, mod_type: ModType) -> Option<(u32, Option<u8>)> {
    let mut other_mod = None;
    for (code, modifier) in modifiers.iter() {
        match modifier {
            Modifier::Single(state_modifier) => {
                if state_modifier.has_mod_type(mod_type) {
                    match state_modifier.kind {
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
                for (level, state_modifier) in map {
                    if state_modifier.has_mod_type(mod_type) {
                        match state_modifier.kind {
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
    fn new(xkb_key_type: &parser::XkbKeyType) -> Self {
        let default_state = CompiledTypeState {
            level: 0,
            consumed_mods: xkb_key_type.mods.mask,
        };
        let mut by_mask = [default_state; REAL_MOD_STATES];
        // Reverse iteration preserves the old "first matching entry wins"
        // behaviour when malformed input contains duplicate masks.
        for entry in xkb_key_type.entries.iter().rev() {
            if (entry.mods.mods == 0 || entry.mods.mask != 0)
                && (entry.mods.mask as usize) < REAL_MOD_STATES
            {
                by_mask[entry.mods.mask as usize] = CompiledTypeState {
                    level: entry.level,
                    consumed_mods: xkb_key_type.mods.mask & !entry.preserve.mask,
                };
            }
        }
        let states = std::array::from_fn(|state| by_mask[state & xkb_key_type.mods.mask as usize]);
        Self {
            states,
            num_lock_affected: xkb_key_type
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
fn group_change(action: XkbGroupAction) -> Option<GroupChange> {
    if action.flags.contains(ActionFlags::ABSOLUTE_SWITCH) {
        u8::try_from(action.group).ok().map(GroupChange::Absolute)
    } else {
        i8::try_from(action.group)
            .ok()
            .filter(|delta| *delta != 0)
            .map(GroupChange::Relative)
    }
}
fn group_key_combinations(
    keymap: &keymap::XkbKeymap,
    owner_keycode: u32,
    modifier_mask: u32,
) -> Vec<Vec<u32>> {
    let owner_mods = keymap
        .get_key(owner_keycode)
        .map_or(0, |key| key.modmap | key.vmodmap.unwrap_or(0));
    let modifier_mask = modifier_mask & !owner_mods;
    let mut combinations = vec![Vec::new()];
    for bit in (0..u32::BITS).map(|index| 1 << index) {
        if modifier_mask & bit == 0 {
            continue;
        }
        let candidates = keymap
            .keys
            .iter()
            .enumerate()
            .filter(|(keycode, key)| {
                *keycode >= 8
                    && *keycode != owner_keycode as usize
                    && (key.modmap | key.vmodmap.unwrap_or(0)) & bit != 0
            })
            .map(|(keycode, _)| keycode as u32 - 8)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            return Vec::new();
        }
        combinations = combinations
            .into_iter()
            .flat_map(|combination| {
                candidates.iter().map(move |&candidate| {
                    let mut next = combination.clone();
                    if !next.contains(&candidate) {
                        next.push(candidate);
                    }
                    next
                })
            })
            .collect();
    }
    for combination in &mut combinations {
        combination.sort_unstable();
        combination.dedup();
    }
    combinations.sort_unstable();
    combinations.dedup();
    combinations
}
fn group_kind(action: XkbAction) -> Option<GroupKind> {
    match action {
        XkbAction::GroupSet(action) => {
            if action.flags.contains(ActionFlags::ABSOLUTE_SWITCH) {
                return None;
            }
            let GroupChange::Relative(delta) = group_change(action)? else {
                return None;
            };
            Some(GroupKind::Press(delta))
        }
        XkbAction::GroupLatch(action) => {
            let change = group_change(action)?;
            if action.flags.contains(ActionFlags::LATCH_TO_LOCK) {
                Some(GroupKind::LatchToLockOnRelease(change))
            } else {
                Some(GroupKind::LatchOnRelease(change))
            }
        }
        XkbAction::GroupLock(action) => {
            let change = group_change(action)?;
            Some(GroupKind::LockOnPress(change))
        }
        _ => None,
    }
}
fn build_groups_from_keymap(keymap: &keymap::XkbKeymap) -> Groups {
    const EVDEV_OFFSET: u32 = 8;
    let mut entries = Vec::new();
    for (keycode, key) in keymap.keys.iter().enumerate() {
        let keycode = keycode as u32;
        if keycode < EVDEV_OFFSET {
            continue;
        }
        let evdev_code = keycode - EVDEV_OFFSET;
        for key_group in &key.groups {
            let Some(key_type) = keymap.types.get(key_group.type_idx as usize) else {
                continue;
            };
            for entry in &key_type.entries {
                if entry.mods.mods != 0 && entry.mods.mask == 0 {
                    continue;
                }
                let Some(level) = key_group.levels.get(entry.level as usize) else {
                    continue;
                };
                let Some(action) = level.action.and_then(group_kind) else {
                    continue;
                };
                for mut keys in group_key_combinations(keymap, keycode, entry.mods.mask) {
                    keys.push(evdev_code);
                    keys.sort_unstable();
                    keys.dedup();
                    let group = Group { keys, action };
                    if !entries.contains(&group) {
                        entries.push(group);
                    }
                }
            }
            let Some(level) = key_group.levels.first() else {
                continue;
            };
            let Some(action) = level.action.and_then(group_kind) else {
                continue;
            };
            let group = Group {
                keys: vec![evdev_code],
                action,
            };
            if !entries.contains(&group) {
                entries.push(group);
            }
        }
    }
    Groups::new(entries)
}
fn resolve_char(
    group: &parser::XkbGroup,
    type_: &CompiledType,
    mods: u32,
    caps_mask: u32,
) -> Option<char> {
    let state = type_.state(mods);
    let raw_sym = group.levels.get(state.level as usize)?.sym;
    let sym = if mods & caps_mask != 0 && state.consumed_mods & caps_mask == 0 {
        keysym::xkb_keysym_to_upper(raw_sym)
    } else {
        raw_sym
    };
    keysym::keysym_to_char(sym)
}
fn key_affected_by_caps(group: &parser::XkbGroup, num_levels: usize) -> bool {
    let Some(l0_sym) = group
        .levels
        .first()
        .map(|level| level.sym)
        .filter(|&sym| sym != 0)
    else {
        return false;
    };
    group.levels.len() < num_levels
        || group
            .levels
            .iter()
            .take(num_levels)
            .skip(1)
            .any(|level| level.sym != l0_sym)
        || keysym::xkb_keysym_to_upper(l0_sym) != l0_sym
}
fn lock_activation(
    keymap: &keymap::XkbKeymap,
    types: &[CompiledType],
    lock_kc: Option<u32>,
    lock_keysym: u32,
    level_masks: &[u32; MAX_LEVELS],
) -> [bool; MAX_LEVELS] {
    let target = lock_kc
        .and_then(|kc| keymap.get_key(kc))
        .and_then(|key| key.groups.first())
        .and_then(|group| {
            types
                .get(group.type_idx as usize)
                .map(|type_| (group, type_))
        });
    std::array::from_fn(|level| {
        target.is_some_and(|(group, type_)| {
            group
                .levels
                .get(type_.state(level_masks[level]).level as usize)
                .is_some_and(|data| data.sym == lock_keysym)
        })
    })
}
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
                let sym = group.levels.first().map(|level| level.sym);
                matches!(sym, Some(0xFFE1 | 0xFFE2 | 0xFE03))
                    && group
                        .levels
                        .iter()
                        .skip(1)
                        .any(|level| matches!(level.sym, 0xfe11 | 0xfe12))
            })
}
#[cfg_attr(not(feature = "client"), allow(unused_variables))]
fn build_wkb_from_keymap(keymap: &keymap::XkbKeymap, layout_locales: Option<&str>) -> WKB {
    const EVDEV_OFFSET: u32 = 8;
    let min_keycode = keymap.min_key_code.max(EVDEV_OFFSET);
    let max_keycode = keymap.keys.len().saturating_sub(1) as u32;
    let num_keys = keymap.keys.len().saturating_sub(EVDEV_OFFSET as usize);
    let num_layouts = (keymap.num_groups as usize).max(1);
    let modifiers = build_modifiers_from_keymap(keymap);
    let compiled_types: Vec<_> = keymap.types.iter().map(CompiledType::new).collect();
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
    let groups = build_groups_from_keymap(keymap);
    let caps_kc = level_code(&modifiers, ModType::Caps).map(|(code, _)| code + EVDEV_OFFSET);
    let num_kc = level_code(&modifiers, ModType::Num).map(|(code, _)| code + EVDEV_OFFSET);
    let caps_active = lock_activation(keymap, &compiled_types, caps_kc, 0xffe5, &level_masks);
    let num_active = lock_activation(keymap, &compiled_types, num_kc, 0xff7f, &level_masks);
    #[cfg(feature = "client")]
    let env_locale = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .ok();
    let mut layouts = Vec::with_capacity(num_layouts);
    for layout_idx in 0..num_layouts {
        let layout_level5 = layout_has_level5_activation(keymap, layout_idx, level5_mask);
        let transform = |mods| {
            if layout_level5 && mods & level2_mask != 0 && mods & level3_mask != 0 {
                (mods | level5_mask) & !level2_mask
            } else {
                mods
            }
        };
        let states: [[u32; MAX_LEVELS]; 4] = std::array::from_fn(|kind| {
            std::array::from_fn(|level| {
                transform(
                    level_masks[level]
                        | (u32::from(kind & 1 != 0 && caps_active[level]) * caps_mask)
                        | (u32::from(kind & 2 != 0 && num_active[level]) * num_mask),
                )
            })
        });
        let mut level_exceptions_keymap = FlatKeymap::new(num_keys);
        let mut named_key_map = FlatNamedKeyMap::new(num_keys);
        let mut maps: [FlatKeymap; 4] = std::array::from_fn(|_| FlatKeymap::new(num_keys));
        let mut repeat_keys = KeyBitSet::default();
        for (kc, key) in keymap.keys.iter().enumerate() {
            let kc = kc as u32;
            if kc < min_keycode || kc > max_keycode {
                continue;
            }
            let evdev = (kc - EVDEV_OFFSET) as usize;
            if key.repeat == Some(true) {
                repeat_keys.insert(evdev as u32);
            }
            let raw_group =
                (!key.groups.is_empty()).then(|| &key.groups[layout_idx % key.groups.len()]);
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
                if let Some(sym) = raw_group
                    .and_then(|group| group.levels.get(level))
                    .map(|data| data.sym)
                    .filter(|&sym| sym != 0)
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
                let base = resolve_char(group, type_, states[0][level], 0);
                maps[0].data[idx] = base;
                for (kind, affected) in [
                    true,
                    caps_affected,
                    num_affected,
                    caps_affected || num_affected,
                ]
                .into_iter()
                .enumerate()
                .skip(1)
                {
                    if !affected {
                        continue;
                    }
                    let value = resolve_char(
                        group,
                        type_,
                        states[kind][level],
                        u32::from(kind & 1 != 0 && caps_affected) * caps_mask,
                    );
                    if value != base {
                        maps[kind].data[idx] = value;
                    }
                }
            }
        }
        let [state_keymap, caps_lock_keymap, num_lock_keys, caps_num_lock_keys] = maps;
        #[cfg(feature = "client")]
        let composer = {
            use compose::layout_composer;

            let locale_hints: Vec<&str> = layout_locales
                .map(|locales| locales.split(',').collect())
                .unwrap_or_default();
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
                    layout_composer(&path, &reachable)
                })
                .unwrap_or_default()
        };
        layouts.push(KBLayout {
            name: keymap
                .group_names
                .get(layout_idx)
                .map(|&name| keymap.ctx.atom_text(name))
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .unwrap_or_else(|| format!("Layout {layout_idx}")),
            repeat_keys,
            #[cfg(feature = "client")]
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
        groups,
    }
}
pub(crate) fn new_from_names(
    _rules: &str,
    _model: &str,
    layout: &str,
    variant: &str,
    options: Option<&str>,
) -> Result<WKB, XkbError> {
    use parser::XkbRuleNames;
    let ctx = xkb_context_new();
    let rmlvo = XkbRuleNames {
        layout: layout.into(),
        variant: variant.into(),
        options: options.unwrap_or("").into(),
    };
    let keymap = xkb_keymap_new_from_names(ctx, &rmlvo)?;
    Ok(build_wkb_from_keymap(&keymap, Some(layout)))
}
pub(crate) fn new_from_string(string: &str) -> Result<WKB, XkbError> {
    let ctx = xkb_context_new();
    if string.as_bytes().contains(&0) {
        return Err(XkbError::KeymapParsing);
    }
    let keymap = xkb_keymap_new_from_string(ctx, string.as_bytes())?;
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
fn build_modifiers_from_keymap(keymap: &keymap::XkbKeymap) -> Modifiers {
    let mut modifiers = Modifiers::new();
    let keysym_to_modtype = |ks: u32| -> Option<ModType> {
        match ks {
            0xfe03 | 0xfe04 | 0xfe05 | 0xfe0d => Some(ModType::Level3),
            0xfe11..=0xfe13 => Some(ModType::Level5),
            0xff20 => Some(ModType::Compose),
            _ => None,
        }
    };
    let keysym_to_state_modifier = |ks: u32, mt: ModType| -> StateModifier {
        match ks {
            0xffe6 | 0xfe05 | 0xfe0d | 0xfe13 => StateModifier {
                kind: ModKind::Lock {
                    pressed: false,
                    locked: 0,
                },
                mod_type: mt,
            },
            0xfe04 | 0xfe12 => StateModifier {
                kind: ModKind::Latch {
                    pressed: false,
                    latched: false,
                },
                mod_type: mt,
            },
            _ => StateModifier {
                kind: ModKind::Press { pressed: false },
                mod_type: mt,
            },
        }
    };
    const EVDEV_OFFSET: u32 = 8;
    for (keycode, key) in keymap.keys.iter().enumerate() {
        let keycode = keycode as u32;
        if keycode < EVDEV_OFFSET {
            continue;
        }
        let evdev_code = keycode - EVDEV_OFFSET;
        let Some(g0) = key.groups.first() else {
            continue;
        };
        let sym = g0
            .levels
            .first()
            .map(|level| level.sym)
            .filter(|&sym| sym != 0);
        let num_levels = g0.levels.len() as u32;
        if num_levels == 1 {
            if let Some((sym, mt)) = sym.and_then(|sym| keysym_to_modtype(sym).map(|mt| (sym, mt)))
            {
                modifiers.set_modifier(
                    evdev_code,
                    Modifier::Single(keysym_to_state_modifier(sym, mt)),
                );
                continue;
            }
        }
        let vmodmap = key.vmodmap.unwrap_or(0);
        if key.modmap == 0 && vmodmap == 0 {
            continue;
        }
        for modifier in keymap.mods.mods.iter().take(keymap.mods.num_mods as usize) {
            let mod_mask = modifier.mapping;
            let named_type = modtype_from_name(keymap.ctx.atom_text(modifier.name));
            if (key.modmap & mod_mask) == 0 && (vmodmap & mod_mask) == 0 {
                continue;
            }
            let mod_type = sym.and_then(keysym_to_modtype).or(named_type);
            let Some(mod_type) = mod_type else { continue };
            if mod_type == ModType::Caps {
                let caps_levels = (0..num_levels).filter(|&level| {
                    g0.levels
                        .get(level as usize)
                        .is_some_and(|level| level.sym == 0xffe5)
                });
                let Some(min_caps) = caps_levels.clone().min() else {
                    continue;
                };
                if caps_levels.count() < num_levels as usize {
                    let level_map: BTreeMap<u8, StateModifier> = (min_caps as u8..8)
                        .map(|level| {
                            (
                                level,
                                StateModifier {
                                    kind: ModKind::Lock {
                                        pressed: false,
                                        locked: 0,
                                    },
                                    mod_type: ModType::Caps,
                                },
                            )
                        })
                        .collect();
                    modifiers.set_modifier(evdev_code, Modifier::Leveled(level_map));
                    continue;
                }
            }
            let state_modifier = match (sym, mod_type) {
                (Some(sym), ModType::Level2 | ModType::Level3 | ModType::Level5) => {
                    keysym_to_state_modifier(sym, mod_type)
                }
                (_, ModType::Caps | ModType::Num | ModType::Scroll) => StateModifier {
                    kind: ModKind::Lock {
                        pressed: false,
                        locked: 0,
                    },
                    mod_type,
                },
                (_, _) => StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type,
                },
            };
            modifiers.set_modifier(evdev_code, Modifier::Single(state_modifier));
        }
    }
    for &code in &[LEFT_CTRL, RIGHT_CTRL] {
        let already_control = modifiers.iter().any(|(c, m)| {
            *c == code
                && matches!(
                    m,
                    Modifier::Single(StateModifier {
                        kind: ModKind::Press { .. },
                        mod_type: ModType::None
                    }) | Modifier::Leveled(_)
                )
        });
        if !already_control {
            modifiers.set_modifier(
                code,
                Modifier::Single(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::None,
                }),
            );
        }
    }
    modifiers
}
