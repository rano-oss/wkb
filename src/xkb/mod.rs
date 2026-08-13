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
use crate::groups::{self};
use crate::xkb::keymap::{
    preprocess_unicode_keysyms, xkb_context_new, xkb_keymap_new_from_names,
    xkb_keymap_new_from_string,
};
use crate::xkb::parser::{
    ActionFlags, XKB_CONTEXT_NO_FLAGS, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_FORMAT_TEXT_V1, XkbAction, XkbGroup
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
    let (modifiers, groups) = build_modifiers_from_keymap(keymap);
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

    let caps_kc = modifiers
        .level_code(ModType::Caps)
        .map(|(code, _)| code + EVDEV_OFFSET);
    let num_kc = modifiers
        .level_code(ModType::Num)
        .map(|(code, _)| code + EVDEV_OFFSET);
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
        groups,
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

fn group_kind_from_action(action: &XkbAction) -> Option<groups::GroupKind> {
    use groups::{GroupChange, GroupKind};

    let change = |group: i32, flags: ActionFlags| {
        if flags.contains(ActionFlags::ABSOLUTE_SWITCH) {
            Some(GroupChange::Absolute(u8::try_from(group).ok()?))
        } else {
            Some(GroupChange::Relative(i8::try_from(group).ok()?))
        }
    };

    match action {
        XkbAction::GroupSet(g) => {
            // Momentary GroupSet is relative-only in our representation.
            if g.flags.contains(ActionFlags::ABSOLUTE_SWITCH) {
                return None;
            }

            Some(GroupKind::Press(i8::try_from(g.group).ok()?))
        }

        XkbAction::GroupLatch(g) => {
            let change = change(g.group, g.flags)?;

            Some(match (
                g.flags.contains(ActionFlags::LATCH_TO_LOCK),
                g.flags.contains(ActionFlags::LATCH_ON_PRESS),
            ) {
                (false, false) => GroupKind::LatchOnRelease(change),
                (false, true) => GroupKind::LatchOnPress(change),
                (true, false) => GroupKind::LatchToLockOnRelease(change),
                (true, true) => GroupKind::LatchToLockOnPress(change),
            })
        }

        XkbAction::GroupLock(g) => {
            let change = change(g.group, g.flags)?;

            Some(if g.flags.contains(ActionFlags::LOCK_ON_RELEASE) {
                GroupKind::LockOnRelease(change)
            } else {
                GroupKind::LockOnPress(change)
            })
        }

        _ => None,
    }
}

fn modkind_from_keysym(
    keysym: u32,
    actions: &[XkbAction],
) -> ModKind {
    match keysym {
        // Caps_Lock, Shift_Lock, ISO level locks
        0xffe5 | 0xffe6 | 0xfe05 | 0xfe0d | 0xfe13 => {
            ModKind::Lock {
                pressed: false,
                locked: 0,
            }
        }

        0xfe04 | 0xfe12 => {
            if actions.iter().any(|action| {
                matches!(
                    action,
                    XkbAction::ModLatch(action)
                        if action
                            .flags
                            .contains(ActionFlags::LATCH_ON_PRESS)
                )
            }) {
                ModKind::LatchOnPress {
                    pressed: false,
                    latched: false,
                }
            } else {
                ModKind::Latch {
                    pressed: false,
                    latched: false,
                }
            }
        }

        _ => ModKind::Press {
            pressed: false,
        },
    }
}

/// Expand key-local modifier levels into WKB's global 0..=7 levels.
///
/// `used_bits`:
/// - bit 0: Level2
/// - bit 1: Level3
/// - bit 2: Level5
fn expand_modifier_levels(
    levels: &BTreeMap<u8, StateModifier>,
    used_bits: u8,
) -> BTreeMap<u8, StateModifier> {
    (0u8..8)
        .filter_map(|global_level| {
            let local_level = global_level & used_bits;

            levels
                .get(&local_level)
                .cloned()
                .map(|modifier| (global_level, modifier))
        })
        .collect()
}

fn modifier_level_bits(
    mod_type: ModType,
    group0: &XkbGroup,
) -> u8 {
    /*
     * Temporary WKB-oriented inference.
     *
     * Two-level Caps keys such as Japanese Eisu/Caps are selected
     * only by Shift, so Level3 and Level5 must be ignored.
     */
    if mod_type == ModType::Caps && group0.levels.len() == 2 {
        return 0b001;
    }

    /*
     * Standard WKB level ordering:
     *
     * 0: none
     * 1: Level2
     * 2: Level3
     * 3: Level2 + Level3
     * 4: Level5
     * ...
     */
    match group0.levels.len() {
        0 | 1 => 0,
        2 => 0b001,
        3 | 4 => 0b011,
        _ => 0b111,
    }
}

/// Build modifier and group actions from an XKB keymap.
fn build_modifiers_from_keymap(
    keymap: &keymap::XkbKeymap,
) -> (Modifiers, groups::Groups) {
    const EVDEV_OFFSET: u32 = 8;

    let mut modifiers = Modifiers::new();
    let mut group_entries = Vec::new();

    let mod_defs: Vec<_> = keymap
        .mods
        .mods
        .iter()
        .take(keymap.mods.num_mods as usize)
        .map(|modifier| {
            (
                modifier.mapping,
                modtype_from_name(
                    keymap.ctx.atom_text(modifier.name),
                ),
            )
        })
        .collect();

    let keysym_to_modtype = |keysym: u32| -> Option<ModType> {
        match keysym {
            // Shift
            0xffe1 | 0xffe2 => Some(ModType::Level2),

            // Control
            0xffe3 | 0xffe4 => Some(ModType::None),

            // Caps
            0xffe5 | 0xffe6 => Some(ModType::Caps),

            // Meta, Alt and Super
            0xffe7..=0xffec => Some(ModType::None),

            // NumLock and ScrollLock
            0xff7f => Some(ModType::Num),
            0xff14 => Some(ModType::Scroll),

            // Level3
            0xfe03 | 0xfe04 | 0xfe05 | 0xfe0d => {
                Some(ModType::Level3)
            }

            // Level5
            0xfe11..=0xfe13 => Some(ModType::Level5),

            // Compose
            0xff20 => Some(ModType::Compose),

            _ => None,
        }
    };

    let modkind_from_action =
        |action: &XkbAction| -> Option<ModKind> {
            match action {
                XkbAction::ModSet(_) => {
                    Some(ModKind::Press {
                        pressed: false,
                    })
                }

                XkbAction::ModLatch(action) => {
                    Some(match (
                        action
                            .flags
                            .contains(ActionFlags::LATCH_TO_LOCK),
                        action
                            .flags
                            .contains(ActionFlags::LATCH_ON_PRESS),
                    ) {
                        (false, false) => ModKind::Latch {
                            pressed: false,
                            latched: false,
                        },

                        (false, true) => {
                            ModKind::LatchOnPress {
                                pressed: false,
                                latched: false,
                            }
                        }

                        (true, false) => {
                            ModKind::LatchToLockOnRelease {
                                pressed: false,
                                latched: false,
                                locked: false,
                            }
                        }

                        (true, true) => {
                            ModKind::LatchToLockOnPress {
                                pressed: false,
                                latched: false,
                                locked: false,
                            }
                        }
                    })
                }

                XkbAction::ModLock(action) => {
                    let lock_on_release = action
                        .flags
                        .contains(ActionFlags::LOCK_ON_RELEASE);

                    let unlock_on_press = action
                        .flags
                        .contains(ActionFlags::UNLOCK_ON_PRESS);

                    Some(match (
                        lock_on_release,
                        unlock_on_press,
                    ) {
                        (false, false) => ModKind::Lock {
                            pressed: false,
                            locked: 0,
                        },

                        (true, false) => {
                            ModKind::LockOnRelease {
                                pressed: false,
                                locked: false,
                            }
                        }

                        (false, true) => {
                            ModKind::UnlockOnPress {
                                pressed: false,
                                locked: false,
                            }
                        }

                        (true, true) => {
                            ModKind::LockOnReleaseUnlockOnPress {
                                pressed: false,
                                locked: false,
                                lock: false,
                            }
                        }
                    })
                }

                _ => None,
            }
        };

    for key in &keymap.keys {
        if key.keycode < EVDEV_OFFSET
            || key.keycode > keymap.max_key_code
        {
            continue;
        }

        let evdev_code = key.keycode - EVDEV_OFFSET;

        let Some(group0) = key.groups.first() else {
            continue;
        };

        /*
         * Build group actions independently from modifier actions.
         */
        let group_levels: BTreeMap<u8, groups::GroupKind> = group0
            .levels
            .iter()
            .enumerate()
            .filter_map(|(level, data)| {
                let level = u8::try_from(level).ok()?;

                let action = data
                    .actions
                    .iter()
                    .find_map(group_kind_from_action)?;

                Some((level, action))
            })
            .collect();

        if !group_levels.is_empty() {
            let group = if group0.levels.len() == 1 {
                groups::Group::Single(
                    *group_levels.get(&0).unwrap(),
                )
            } else {
                groups::Group::Leveled(group_levels)
            };

            group_entries.push((evdev_code, group));
        }

        /*
         * Keys without a modifier map can still contain explicit
         * modifier symbols such as ISO_Level3_Latch.
         */
        if key.modmap == 0 && key.vmodmap == 0 {
            let levels: BTreeMap<u8, StateModifier> = group0
                .levels
                .iter()
                .enumerate()
                .filter_map(|(level, data)| {
                    let level = u8::try_from(level).ok()?;

                    let keysym = data
                        .syms
                        .iter()
                        .copied()
                        .find(|keysym| {
                            keysym_to_modtype(*keysym).is_some()
                        })?;

                    let mod_type =
                        keysym_to_modtype(keysym)?;

                    Some((
                        level,
                        StateModifier {
                            mod_type,
                            kind: modkind_from_keysym(
                                keysym,
                                data.actions.as_slice(),
                            ),
                        },
                    ))
                })
                .collect();
            
            if !levels.is_empty() {
                let modifier = if group0.levels.len() == 1 {
                    Modifier::Single(
                        levels.into_values().next().unwrap(),
                    )
                } else {
                    Modifier::Leveled(levels)
                };

                modifiers.set_modifier(evdev_code, modifier);
            }

            continue;
        }

        /*
         * A key can belong to more than one real or virtual modifier.
         */
        for &(mod_mask, named_type) in &mod_defs {
            if (key.modmap & mod_mask) == 0
                && (key.vmodmap & mod_mask) == 0
            {
                continue;
            }

            /*
             * Prefer the modifier type expressed by a symbol. Fall
             * back to the modifier-map name for remapped keys.
             */
            let symbol_type = group0
                .levels
                .iter()
                .flat_map(|level| level.syms.iter().copied())
                .find_map(keysym_to_modtype);

            let Some(mod_type) = symbol_type.or(named_type) else {
                continue;
            };

            /*
             * Construct every modifier level independently.
             *
             * A level participates when:
             *
             * 1. It contains a recognized symbol for this modifier; or
             * 2. It contains a modifier action affecting this map.
             */
            let levels: BTreeMap<u8, StateModifier> = group0
                .levels
                .iter()
                .enumerate()
                .filter_map(|(level, data)| {
                    let level = u8::try_from(level).ok()?;

                    let symbol = data
                        .syms
                        .iter()
                        .copied()
                        .find(|keysym| {
                            keysym_to_modtype(*keysym)
                                == Some(mod_type)
                        });

                    let action = data.actions.iter().find(|action| {
                        match action {
                            XkbAction::ModSet(action)
                            | XkbAction::ModLatch(action)
                            | XkbAction::ModLock(action) => {
                                action.mods.mask & mod_mask != 0
                            }

                            _ => false,
                        }
                    });

                    let kind = if let Some(keysym) = symbol {
                        modkind_from_keysym(
                            keysym,
                            data.actions.as_slice(),
                        )
                    } else {
                        modkind_from_action(action?)?
                    };

                    Some((
                        level,
                        StateModifier {
                            mod_type,
                            kind,
                        },
                    ))
                })
                .collect();

            let used_bits = modifier_level_bits(
                mod_type,
                group0,
            );
            
            let levels = expand_modifier_levels(
                &levels,
                used_bits,
            );

            if !levels.is_empty() {
                let modifier = if group0.levels.len() == 1 {
                    Modifier::Single(
                        levels.into_values().next().unwrap(),
                    )
                } else {
                    Modifier::Leveled(levels)
                };

                modifiers.set_modifier(evdev_code, modifier);
                continue;
            }

            /*
             * No level exposed a recognizable modifier symbol or
             * action. Preserve modifier-map behavior for remapped
             * physical modifier keys.
             *
             * Only use Single here when no level-specific behavior
             * could be recovered.
             */
            let kind = match mod_type {
                ModType::Caps
                | ModType::Num
                | ModType::Scroll => ModKind::Lock {
                    pressed: false,
                    locked: 0,
                },

                _ => ModKind::Press {
                    pressed: false,
                },
            };

            modifiers.set_modifier(
                evdev_code,
                Modifier::Single(StateModifier {
                    mod_type,
                    kind,
                }),
            );
        }
    }

    /*
     * Preserve Control behavior for layouts that remap the standard
     * Control key symbols.
     */
     for code in [LEFT_CTRL, RIGHT_CTRL] {
         modifiers.set_modifier(
             code,
             Modifier::Single(StateModifier {
                 mod_type: ModType::None,
                 kind: ModKind::Press {
                     pressed: false,
                 },
             }),
         );
     }
    (
        modifiers,
        groups::Groups::new(group_entries),
    )
}
