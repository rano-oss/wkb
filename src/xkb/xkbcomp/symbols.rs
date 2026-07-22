use super::super::keymap::xkb_escape_map_name;
use super::super::keymap::{
    lookup_string, CTRL_MASK_NAMES, GROUP_COMPONENT_MASK_NAMES, MOD_COMPONENT_MASK_NAMES,
    SYM_INTERPRET_MATCH_MASK_NAMES, USE_MOD_MAP_VALUE_NAMES,
};
pub(crate) use super::super::keymap::{
    xkb_levels_same_actions, xkb_levels_same_syms, xkb_mod_name_to_index,
};
use super::super::keysym::xkb_keysym_is_keypad;
use super::super::keysym::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
pub(crate) use super::super::shared_types::{
    InterpDef, KeyAliasDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef, SymbolsDef,
};
pub(crate) use super::super::shared_types::{
    MAX_ACTIONS_PER_LEVEL, MOD_REAL_MASK_ALL, XKB_MAX_LEDS, XKB_MOD_NONE, XKB_OVERLAY_INVALID,
};
use super::parser::{exceeds_include_max_depth, process_include_file};

pub(crate) struct SymbolsInfo {
    pub(crate) name: Option<String>,
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) explicit_group: u32,
    pub(crate) max_groups: u32,
    pub(crate) keys: Vec<KeyInfo>,
    pub(crate) default_key: KeyInfo,
    pub(crate) default_actions: ActionsInfo,
    pub(crate) group_names: Vec<u32>,
    pub(crate) modmaps: Vec<ModMapEntry>,
    pub(crate) mods: XkbModSet,
    pub(crate) star_atom: u32,
}
#[derive(Copy, Clone)]
pub(crate) struct ModMapEntry {
    pub(crate) merge: u32,
    pub(crate) have_symbol: bool,
    pub(crate) modifier: u32,
    pub(crate) u: u32,
}
#[derive(Clone)]
pub(crate) struct KeyInfo {
    pub(crate) name: u32,
    pub(crate) vmodmap: u32,
    pub(crate) default_type: u32,
    pub(crate) out_of_range_group_number: u32,
    pub(crate) groups: Vec<GroupInfo>,
    pub(crate) out_of_range_group_policy: u32,
    pub(crate) defined: u32,
    pub(crate) merge: u32,
    pub(crate) repeat: u32,
    pub(crate) out_of_range_pending_group: bool,
    pub(crate) overlays_clear: bool,
    pub(crate) overlays: u8,
    pub(crate) overlay_keys: Vec<u32>,
}
pub(crate) const _KEY_REPEAT_NUM_ENTRIES: u32 = 3;
pub(crate) const KEY_REPEAT_NO: u32 = 2;
pub(crate) const KEY_REPEAT_YES: u32 = 1;
pub(crate) const KEY_REPEAT_UNDEFINED: u32 = 0;
pub(crate) const KEY_FIELD_OVERLAY: u32 = 16;
pub(crate) const KEY_FIELD_VMODMAP: u32 = 8;
pub(crate) const KEY_FIELD_GROUPINFO: u32 = 4;
pub(crate) const KEY_FIELD_DEFAULT_TYPE: u32 = 2;
pub(crate) const KEY_FIELD_REPEAT: u32 = 1;
#[derive(Clone, Default)]
pub(crate) struct GroupInfo {
    pub(crate) levels: Vec<XkbLevel>,
    pub(crate) defined: u32,
    pub(crate) type_0: u32,
}

pub(crate) const GROUP_FIELD_TYPE: u32 = 4;
pub(crate) const GROUP_FIELD_ACTS: u32 = 2;
pub(crate) const GROUP_FIELD_SYMS: u32 = 1;

impl KeyInfo {
    pub(crate) fn new_zeroed() -> Self {
        Self {
            name: 0,
            vmodmap: 0,
            default_type: 0,
            out_of_range_group_number: 0,
            groups: Vec::new(),
            out_of_range_group_policy: 0,
            defined: 0,
            merge: 0,
            repeat: 0,
            out_of_range_pending_group: false,
            overlays_clear: false,
            overlays: 0,
            overlay_keys: Vec::new(),
        }
    }
}

impl SymbolsInfo {
    pub(crate) fn new(ki: &mut XkbKeymapInfo<'_>) -> Self {
        let star_atom = atom_intern(&mut ki.keymap.ctx.atom_table, b"*");
        Self {
            name: None,
            error_count: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: Vec::with_capacity(256),
            default_key: KeyInfo::new_zeroed(),
            default_actions: ActionsInfo {
                actions: [XkbAction::None; 21],
            },
            group_names: Vec::new(),
            modmaps: Vec::new(),
            mods: XkbModSet {
                mods: [XkbMod {
                    name: 0,
                    type_0: 0,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            star_atom,
        }
    }
}

fn resize_groups_zero(v: &mut Vec<GroupInfo>, new_len: usize) {
    v.resize_with(new_len, Default::default);
}

/// Check if an ActionList container actually holds action data (vs keysym data).
/// In the old linked-list model, the head node's type distinguished these.
/// Now both are wrapped in ActionList containers, so we check the first inner node.
fn is_action_list_value(value: &ExprKind) -> bool {
    if let ExprKind::ActionList { actions } = &value {
        if let Some(first) = actions.first() {
            // If the first inner node is an ActionList (actions for one level) or
            // Action (single action), it's action data. KeysymList means keysym data.
            matches!(first, ExprKind::ActionList { .. } | ExprKind::Action { .. })
        } else {
            // Empty ActionList — treat as actions
            true
        }
    } else {
        false
    }
}

/// Extract child expressions from an ActionList container node, or return a single-element slice.
fn collect_expr_list(container: &ExprKind) -> &[ExprKind] {
    match &container {
        ExprKind::ActionList { actions } => actions.as_slice(),
        _ => std::slice::from_ref(container),
    }
}

fn init_key_info_with_atom(keyi: &mut KeyInfo, star_atom: u32) {
    *keyi = KeyInfo {
        name: star_atom,
        vmodmap: 0,
        default_type: 0,
        out_of_range_group_number: 0,
        groups: Vec::new(),
        out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
        defined: 0,
        merge: 0,
        repeat: 0,
        out_of_range_pending_group: false,
        overlays_clear: false,
        overlays: 0,
        overlay_keys: Vec::new(),
    };
}
fn init_symbols_info(
    info: &mut SymbolsInfo,
    ki: &mut XkbKeymapInfo<'_>,
    include_depth: u32,
    mods: &XkbModSet,
) {
    info.include_depth = include_depth;
    info.explicit_group = XKB_LAYOUT_INVALID;
    info.max_groups = ki.features.max_groups;
    init_key_info_with_atom(
        &mut info.default_key,
        atom_intern(&mut ki.keymap.ctx.atom_table, b"*"),
    );
    init_actions_info(&*ki.keymap, &mut info.default_actions);
    init_vmods(&mut info.mods, mods, include_depth > 0);
}
fn merge_groups(
    into: &mut GroupInfo,
    from: &mut GroupInfo,
    clobber: bool,
) -> bool {
    if into.type_0 != from.type_0 && (from.type_0 != XKB_ATOM_NONE) {
        if into.type_0 == XKB_ATOM_NONE {
            into.type_0 = from.type_0;
        } else {
            let use_0: u32 = if clobber { from.type_0 } else { into.type_0 };
            let _ignore: u32 = if clobber { into.type_0 } else { from.type_0 };

            into.type_0 = use_0;
        }
    }
    into.defined |= from.defined & GROUP_FIELD_TYPE;
    if from.levels.is_empty() {
        *from = GroupInfo::default();
        return true;
    }
    if into.levels.is_empty() {
        from.type_0 = into.type_0;
        *into = std::mem::take(from);
        return true;
    }
    let levels_in_both: u32 = if into.levels.len() < from.levels.len() {
        into.levels.len()
    } else {
        from.levels.len()
    } as u32;
    let mut from_keysyms_count: u32 = 0;
    let mut from_actions_count: u32 = 0;
    let mut i: u32 = 0;
    while i < levels_in_both {
        let into_level = &mut into.levels[i as usize];
        let from_level = &mut from.levels[i as usize];
        let from_has_no_keysym: bool = from_level.syms.is_empty();
        let from_has_no_action: bool = from_level.actions.is_empty();
        if !(from_has_no_keysym && from_has_no_action) {
            let into_has_no_keysym: bool = into_level.syms.is_empty();
            let into_has_no_action: bool = into_level.actions.is_empty();
            if into_has_no_keysym && into_has_no_action {
                // StealLevelInfo inlined
                into_level.syms = std::mem::take(&mut from_level.syms);
                into_level.actions = std::mem::take(&mut from_level.actions);
                from_keysyms_count += 1;
                from_actions_count += 1;
            } else {
                if !xkb_levels_same_syms(from_level, into_level) {
                    if !from_has_no_keysym {
                        if clobber {
                            if !from_level.syms.is_empty() {
                                into_level.syms = std::mem::take(&mut from_level.syms);
                                from_keysyms_count += 1;
                            }
                        } else if into_level.syms.is_empty() {
                            if !from_level.syms.is_empty() {
                                into_level.syms = std::mem::take(&mut from_level.syms);
                            }
                            from_keysyms_count += 1;
                        }
                    }
                }
                if !xkb_levels_same_actions(into_level, from_level) {
                    if !from_has_no_action {
                        if clobber {
                            if !from_level.actions.is_empty() {
                                into_level.actions = std::mem::take(&mut from_level.actions);
                                from_actions_count += 1;
                            }
                        } else if into_level.actions.is_empty() {
                            if !from_level.actions.is_empty() {
                                into_level.actions = std::mem::take(&mut from_level.actions);
                            }
                            from_actions_count += 1;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    for level in from.levels[levels_in_both as usize..].iter_mut() {
        let level_val = level.clone();
        into.levels.push(level_val);
        level.syms.clear();
        level.actions.clear();
        from_keysyms_count += 1;
        from_actions_count += 1;
    }
    if from_keysyms_count != 0 {
        if from_keysyms_count == into.levels.len() as u32 {
            into.defined &= !GROUP_FIELD_SYMS;
        }
        into.defined |= from.defined & GROUP_FIELD_SYMS;
    }
    if from_actions_count != 0 {
        if from_actions_count == into.levels.len() as u32 {
            into.defined &= !GROUP_FIELD_ACTS;
        }
        into.defined |= from.defined & GROUP_FIELD_ACTS;
    }
    true
}
fn use_new_field(
    field: u32,
    old: u32,
    new: u32,
    clobber: bool,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        return clobber;
    }
    false
}
fn overlays_get(info: &KeyInfo, bit: u8, key_out: Option<&mut u32>) -> bool {
    if bit as i32 >= (std::mem::size_of::<u8>()).wrapping_mul(8_usize) as u8 as i32 {
        return false;
    }
    let mask: u8 = (1_u32 << bit as i32) as u8;
    if (info.overlays & mask) == 0 {
        return false;
    }
    if let Some(key_out) = key_out {
        let low: u8 = (info.overlays as u32 & (mask as u32).wrapping_sub(1)) as u8;
        let index: usize = (low as u32).count_ones() as usize;
        *key_out = info.overlay_keys[index];
    }
    true
}
fn overlays_insert(keyi: &mut KeyInfo, bit: u8, key: u32) -> bool {
    if bit as i32 >= (std::mem::size_of::<u8>()).wrapping_mul(8_usize) as u8 as i32 {
        return false;
    }
    let mask: u8 = (1_u32 << bit as i32) as u8;
    if (keyi.overlays & mask) != 0 && !keyi.overlays_clear {
        // Bit already set — update existing entry
        let low: u8 = (keyi.overlays as u32 & (mask as u32).wrapping_sub(1)) as u8;
        let index: usize = (low as u32).count_ones() as usize;
        keyi.overlay_keys[index] = key;
        if key == XKB_KEYCODE_INVALID && keyi.overlay_keys.len() == 1 {
            keyi.overlays_clear = true;
        }
        return true;
    }
    // New bit
    let new_overlays: u8 = keyi.overlays | mask;
    let low: u8 = (new_overlays as u32 & (mask as u32).wrapping_sub(1)) as u8;
    let index: usize = (low as u32).count_ones() as usize;

    if keyi.overlays == 0 || keyi.overlays_clear && key == XKB_KEYCODE_INVALID {
        // First overlay or clearing
        keyi.overlay_keys.clear();
        keyi.overlay_keys.push(key);
        keyi.overlays = new_overlays;
        keyi.overlays_clear = key == XKB_KEYCODE_INVALID;
    } else {
        // Insert at correct position in Vec
        keyi.overlay_keys.insert(index, key);
        keyi.overlays = new_overlays;
        keyi.overlays_clear = false;
    }
    true
}
fn merge_overlays(
    ki: &XkbKeymapInfo<'_>,
    into: &mut KeyInfo,
    from: &mut KeyInfo,
    mut clobber: bool,
) -> bool {
    if (from.defined & KEY_FIELD_OVERLAY) != 0 {
        if (into.defined & KEY_FIELD_OVERLAY) == 0 {
            // into has no overlays, take from's
            into.overlays = from.overlays;
            into.overlay_keys = std::mem::take(&mut from.overlay_keys);
            into.defined |= KEY_FIELD_OVERLAY;
        } else if into.overlays_clear && from.overlays_clear {
            into.overlays = (into.overlays as i32 | from.overlays as i32) as u8;
        } else if ki.features.overlapping_overlays {
            // Complex merge with overlapping overlays
            let result_mask: u8 = (into.overlays as i32 | from.overlays as i32) as u8;
            let count: u8 = (result_mask as u32).count_ones() as u8;
            if count == 0 {
                eprintln!(
                    "Critical Error: Reached unreachable line in ../src/xkbcomp/symbols.c at {}",
                    696
                );
                std::process::abort();
            }
            // Determine which one is dest (larger capacity) and which is src
            let swapped = from.overlay_keys.capacity() > into.overlay_keys.capacity();
            if swapped {
                std::mem::swap(into, from);
                clobber = !clobber;
            }
            // Now `into` is dest and `from` is src
            let mut remaining: u8 = from.overlays;
            let mut src_idx: usize = 0;
            while remaining != 0 {
                let lsb: u8 = (remaining as i32
                    & (!(remaining as i32) as u32).wrapping_add(1) as u8 as i32)
                    as u8;
                let bit: u8 = ((lsb as u32).wrapping_sub(1).count_ones()) as u8;
                remaining = (remaining as i32 & !(lsb as i32)) as u8;
                let src_key: u32 = if from.overlays_clear || src_idx >= from.overlay_keys.len() {
                    XKB_KEYCODE_INVALID
                } else {
                    let k = from.overlay_keys[src_idx];
                    src_idx += 1;
                    k
                };
                let mut dest_key: u32 = XKB_KEYCODE_INVALID;
                let conflict: bool = overlays_get(into, bit, Some(&mut dest_key)) as bool;
                if conflict {
                    if dest_key == src_key {
                        continue;
                    }
                }
                if (!conflict || clobber) && !overlays_insert(into, bit, src_key) {
                    return false;
                }
            }
            if swapped {
                // We swapped into/from, so move dest data back to into
                std::mem::swap(into, from);
            }
        } else {
            if into.overlays as i32 == from.overlays as i32
                && into.overlays_clear == from.overlays_clear
            {
                // Check if single overlay keys match
                let into_key = into
                    .overlay_keys
                    .first()
                    .copied()
                    .unwrap_or(XKB_KEYCODE_INVALID);
                let from_key = from
                    .overlay_keys
                    .first()
                    .copied()
                    .unwrap_or(XKB_KEYCODE_INVALID);
                if into_key == from_key {
                    return true;
                }
            }
            if (into.overlays & from.overlays) == 0 {
                if into.overlays_clear {
                    into.overlays = from.overlays;
                    into.overlays_clear = from.overlays_clear;
                    into.overlay_keys = std::mem::take(&mut from.overlay_keys);
                    return true;
                } else if from.overlays_clear {
                    return true;
                }
            }
            if clobber {
                into.overlays = from.overlays;
                into.overlays_clear = from.overlays_clear;
                into.overlay_keys = std::mem::take(&mut from.overlay_keys);
            }
        }
    }
    true
}
fn merge_keys(
    ki: &XkbKeymapInfo<'_>,
    info: &SymbolsInfo,
    into: &mut KeyInfo,
    from: &mut KeyInfo,
) -> bool {
    let mut i: u32;

    let clobber: bool = from.merge != MERGE_AUGMENT;
    if from.merge == MERGE_REPLACE {
        std::mem::swap(into, from);
        init_key_info_with_atom(from, info.star_atom);
        return true;
    }
    let groups_in_both: u32 = (if into.groups.len() < from.groups.len() {
        into.groups.len()
    } else {
        from.groups.len()
    }) as u32;
    i = 0;
    while i < groups_in_both {
        merge_groups(
            &mut into.groups[i as usize],
            &mut from.groups[i as usize],
            clobber,        );
        i += 1;
    }
    for group in from.groups.drain(groups_in_both as usize..) {
        into.groups.push(group);
    }
    if use_new_field(
        KEY_FIELD_VMODMAP,
        into.defined,
        from.defined,
        clobber,

    ) {
        into.vmodmap = from.vmodmap;
        into.defined |= KEY_FIELD_VMODMAP;
    }
    if use_new_field(
        KEY_FIELD_REPEAT,
        into.defined,
        from.defined,
        clobber,

    ) {
        into.repeat = from.repeat;
        into.defined |= KEY_FIELD_REPEAT;
    }
    if use_new_field(
        KEY_FIELD_DEFAULT_TYPE,
        into.defined,
        from.defined,
        clobber,

    ) {
        into.default_type = from.default_type;
        into.defined |= KEY_FIELD_DEFAULT_TYPE;
    }
    if use_new_field(
        KEY_FIELD_GROUPINFO,
        into.defined,
        from.defined,
        clobber,

    ) {
        into.out_of_range_pending_group = from.out_of_range_pending_group;
        into.out_of_range_group_policy = from.out_of_range_group_policy;
        into.out_of_range_group_number = from.out_of_range_group_number;
        into.defined |= KEY_FIELD_GROUPINFO;
    }
    if !merge_overlays(ki, into, from, clobber) {
        return false;
    }
    init_key_info_with_atom(from, info.star_atom);
    true
}
fn add_key_symbols(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
) -> bool {
    // XkbResolveKeyAlias inlined
    {
        let keymap = &*ki.keymap;
        let name = keyi.name;
        if (name as usize) < keymap.key_names.len() {
            let match_0: KeycodeMatch = keymap.key_names[name as usize];
            if match_0.found && match_0.is_alias {
                keyi.name = match_0.index;
            }
        }
    }
    for i in 0..info.keys.len() {
        if info.keys[i].name == keyi.name {
            let mut existing = std::mem::replace(&mut info.keys[i], KeyInfo::new_zeroed());
            let result = merge_keys(ki, info, &mut existing, keyi);
            info.keys[i] = existing;
            return result;
        }
    }
    // Move keyi's data into the keys vec
    let moved = std::mem::replace(keyi, KeyInfo::new_zeroed());
    info.keys.push(moved);
    init_key_info_with_atom(keyi, info.star_atom);
    true
}
fn add_mod_map_entry(info: &mut SymbolsInfo, new: &ModMapEntry) -> bool {
    let clobber: bool = new.merge != MERGE_AUGMENT;
    for old in info.modmaps.iter_mut() {
        if new.have_symbol != old.have_symbol
            || new.have_symbol && new.u != old.u
            || !new.have_symbol && new.u != old.u
        {
            continue;
        }
        if new.modifier == old.modifier {
            return true;
        }
        let use_0: u32 = if clobber { new.modifier } else { old.modifier };
        let _ignore: u32 = if clobber { old.modifier } else { new.modifier };

        old.modifier = use_0;
        return true;
    }
    info.modmaps.push(*new);
    true
}
fn merge_included_symbols(
    ki: &mut XkbKeymapInfo<'_>,
    into: &mut SymbolsInfo,
    from: &mut SymbolsInfo,
    merge: u32,
) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut ki.keymap.ctx, &mut into.mods, &from.mods, merge);
    if into.name.is_none() {
        into.name = from.name.take();
    }
    let group_names_in_both: u32 = (if into.group_names.len() < from.group_names.len() {
        into.group_names.len()
    } else {
        from.group_names.len()
    }) as u32;
    let mut i: u32 = 0;
    while i < group_names_in_both {
        if ((&from.group_names)[i as usize] != 0)
            && !(merge == MERGE_AUGMENT && (&into.group_names)[i as usize] != 0)
        {
            (&mut into.group_names)[i as usize] = (&from.group_names)[i as usize];
        }
        i += 1;
    }
    if group_names_in_both < from.group_names.len() as u32 {
        for gn_idx in group_names_in_both as usize..from.group_names.len() {
            into.group_names.push((&from.group_names)[gn_idx]);
        }
    }
    if into.keys.is_empty() {
        std::mem::swap(&mut into.keys, &mut from.keys);
    } else {
        for keyi in from.keys.iter_mut() {
            keyi.merge = merge as u32;
            if !add_key_symbols(ki, into, keyi) {
                into.error_count += 1;
            }
        }
    }
    if into.modmaps.is_empty() {
        std::mem::swap(&mut into.modmaps, &mut from.modmaps);
    } else {
        for mm in from.modmaps.iter_mut() {
            mm.merge = merge;
            if !add_mod_map_entry(into, mm) {
                into.error_count += 1;
            }
        }
    };
}
fn handle_include_symbols(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    include: &mut IncludeStmt,
) -> bool {
    let mut included = SymbolsInfo::new(ki);
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    init_symbols_info(
        &mut included,
        ki,
        info.include_depth.wrapping_add(1),
        &info.mods,
    );
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(std::mem::take(&mut include.stmt))
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = SymbolsInfo::new(ki);

        let file: Option<Box<XkbFile>> =
            process_include_file(&mut ki.keymap.ctx, stmt, FILE_TYPE_SYMBOLS);
        let Some(mut file) = file else {
            info.error_count += 10;
            return false;
        };
        init_symbols_info(
            &mut next_incl,
            ki,
            info.include_depth.wrapping_add(1),
            &included.mods,
        );
        if !stmt.modifier.is_empty() {
            next_incl.explicit_group = (stmt.modifier.parse::<i32>().unwrap_or(0) - 1) as u32;
            if next_incl.explicit_group >= info.max_groups {
                next_incl.explicit_group = info.explicit_group;
            }
        } else if ki.keymap.num_groups != 0 && next_incl.include_depth == 1 {
            next_incl.explicit_group = 0;
        } else {
            next_incl.explicit_group = info.explicit_group;
        }
        handle_symbols_file(ki, &mut next_incl, &mut file);
        merge_included_symbols(ki, &mut included, &mut next_incl, stmt.merge);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    merge_included_symbols(ki, info, &mut included, include.merge);
    info.error_count == 0
}
fn get_group_index(
    ki: &mut XkbKeymapInfo<'_>,
    info: &SymbolsInfo,
    keyi: &mut KeyInfo,
    array_ndx: Option<&ExprKind>,
    field: u32,
    ndx_rtrn: &mut u32,
) -> bool {
    let _name: &str = if field == GROUP_FIELD_SYMS {
        "symbols"
    } else {
        "actions"
    };
    if array_ndx.is_none() {
        let mut i: u32 = 0;
        if !keyi.groups.is_empty() {
            i = 0;
            while (i as usize) < keyi.groups.len() {
                if keyi.groups[i as usize].defined & field == 0 {
                    *ndx_rtrn = i;
                    return true;
                }
                i += 1;
            }
        }
        if i >= info.max_groups {
            return false;
        }
        let new_len = keyi.groups.len() + 1;
        resize_groups_zero(&mut keyi.groups, new_len);
        *ndx_rtrn = (keyi.groups.len() - 1) as u32;
        return true;
    }
    let mut _pending = false;
    if expr_resolve_group(ki, array_ndx.unwrap(), false, ndx_rtrn, &mut _pending) != PARSER_SUCCESS
    {
        return false;
    }
    *ndx_rtrn -= 1;
    if *ndx_rtrn >= keyi.groups.len() as u32 {
        resize_groups_zero(&mut keyi.groups, (*ndx_rtrn + 1) as usize);
    }
    true
}
fn add_symbols_to_key(
    ki: &mut XkbKeymapInfo<'_>,
    info: &SymbolsInfo,
    keyi: &mut KeyInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let mut ndx: u32 = 0;
    if !get_group_index(ki, info, keyi, array_ndx, GROUP_FIELD_SYMS, &mut ndx) {
        return false;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if value.stmt_type() == STMT_EXPR_EMPTY_LIST {
        groupi.defined |= GROUP_FIELD_SYMS;
        return true;
    }
    if value.stmt_type() != STMT_EXPR_KEYSYM_LIST && value.stmt_type() != STMT_EXPR_ACTION_LIST {
        return false;
    }
    if groupi.defined & GROUP_FIELD_SYMS != 0 {
        return false;
    }
    let mut n_levels: u32 = 0;
    let mut non_empty_levels: u32 = 0;
    let keysym_nodes = collect_expr_list(value);
    for node in keysym_nodes {
        n_levels += 1;
        let ExprKind::KeysymList { ref syms } = node else {
            unreachable!()
        };
        if syms.len() as u32 > 0 {
            non_empty_levels = n_levels;
        }
    }
    if non_empty_levels < n_levels {
        n_levels = non_empty_levels;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if (groupi.levels.len() as u32) < n_levels {
        groupi
            .levels
            .resize_with(n_levels as usize, Default::default);
    }
    groupi.defined |= GROUP_FIELD_SYMS;
    for (level, node) in keysym_nodes.iter().enumerate() {
        if level as u32 >= n_levels {
            break;
        }
        let leveli = &mut keyi.groups[ndx as usize].levels[level];
        let ExprKind::KeysymList { ref syms } = node else {
            unreachable!()
        };
        let syms_len = syms.len() as u32;
        if syms_len > 65535 {
            return false;
        }
        leveli.syms = if syms_len == 0 {
            Vec::new()
        } else {
            syms[..syms_len as usize].to_vec()
        };
    }
    true
}
fn add_actions_to_key(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
    array_ndx: Option<&ExprKind>,
    value: &mut ExprKind,
) -> bool {
    let mut ndx: u32 = 0;
    if !get_group_index(ki, info, keyi, array_ndx, GROUP_FIELD_ACTS, &mut ndx) {
        return false;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if value.stmt_type() == STMT_EXPR_EMPTY_LIST {
        groupi.defined |= GROUP_FIELD_ACTS;
        return true;
    }
    if value.stmt_type() != STMT_EXPR_ACTION_LIST {
        return false;
    }
    if groupi.defined & GROUP_FIELD_ACTS != 0 {
        return false;
    }
    let action_nodes = if let ExprKind::ActionList { ref mut actions } = value {
        actions.as_mut_slice()
    } else {
        std::slice::from_mut(value)
    };
    let n_levels: u32 = action_nodes.len() as u32;
    let groupi = &mut keyi.groups[ndx as usize];
    if (groupi.levels.len() as u32) < n_levels {
        groupi
            .levels
            .resize_with(n_levels as usize, Default::default);
    }
    groupi.defined |= GROUP_FIELD_ACTS;
    let mut level: u32 = 0;
    let mut non_empty_levels: u32 = 0;
    for action_node in action_nodes {
        let ExprKind::ActionList {
            actions: action_vec,
        } = action_node
        else {
            unreachable!()
        };
        let num_actions: u32 = action_vec.len() as u32;
        if num_actions > 65535 {
            return false;
        }
        let mut actions: Vec<XkbAction> = Vec::new();
        let mut action_iter = action_vec.iter_mut();
        let mut no_more_actions: bool = false;
        loop {
            let Some(act_expr) = action_iter.next() else {
                no_more_actions = true;
                break;
            };
            let mut to_act: XkbAction = XkbAction::None;
            let r: u32 = handle_action_def(
                ki,
                &mut info.default_actions,
                &info.mods,
                act_expr,
                &mut to_act,
            );
            if r != PARSER_SUCCESS {
                if r == PARSER_FATAL_ERROR {
                    drop(actions);
                    return false;
                } else {
                    to_act.set_none();
                }
            }
            if to_act.action_type() != ACTION_TYPE_NONE {
                if num_actions == 1 {
                    keyi.groups[ndx as usize].levels[level as usize].actions = vec![to_act];
                    break;
                } else {
                    actions.push(to_act);
                }
            }
        }
        if no_more_actions {
            let leveli = &mut keyi.groups[ndx as usize].levels[level as usize];
            if actions.is_empty() {
                leveli.actions = Vec::new();
            } else {
                leveli.actions = std::mem::take(&mut actions);
            }
        }
        {
            let leveli = &keyi.groups[ndx as usize].levels[level as usize];
            if !leveli.actions.is_empty() || !leveli.syms.is_empty() {
                non_empty_levels = level.wrapping_add(1);
            }
        }
        level += 1;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if non_empty_levels < n_levels {
        if non_empty_levels > 0 {
            groupi.levels.truncate(non_empty_levels as usize);
        } else {
            groupi.levels.clear();
        }
    }
    true
}
static REPEAT_ENTRIES: [LookupEntry; 8] = [
    LookupEntry {
        name: "true",
        value: KEY_REPEAT_YES,
    },
    LookupEntry {
        name: "yes",
        value: KEY_REPEAT_YES,
    },
    LookupEntry {
        name: "on",
        value: KEY_REPEAT_YES,
    },
    LookupEntry {
        name: "false",
        value: KEY_REPEAT_NO,
    },
    LookupEntry {
        name: "no",
        value: KEY_REPEAT_NO,
    },
    LookupEntry {
        name: "off",
        value: KEY_REPEAT_NO,
    },
    LookupEntry {
        name: "default",
        value: KEY_REPEAT_UNDEFINED,
    },
    LookupEntry { name: "", value: 0 },
];
fn expr_resolve_overlay_entry(
    keymap_info: &XkbKeymapInfo<'_>,
    field: &str,
    array_ndx: Option<&ExprKind>,
    expr: &ExprKind,
    _keyi: &KeyInfo,
    overlay_rtrn: &mut u8,
    key_rtrn: &mut u32,
) -> bool {
    if array_ndx.is_some() {
        return false;
    }
    let prefix: usize = (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1_usize);
    let suffix = &field[prefix..];
    let len: usize = suffix.len();
    let (val_parsed, parse_count) = super::super::shared_types::parse_dec_u64(suffix.as_bytes());
    let raw_overlay: i64 = val_parsed as i64;
    if parse_count != len as i32
        || raw_overlay < 1_i64
        || raw_overlay > keymap_info.features.max_overlays as i64
    {
        return false;
    }
    *overlay_rtrn = (raw_overlay as u8 as i32 - 1) as u8;
    match expr.stmt_type() {
        8 => {
            let ExprKind::KeyName(key_name_val) = expr else {
                unreachable!()
            };
            let key_kc = keymap_info
                .keymap
                .key_by_name(*key_name_val, false)
                .map(|k| k.keycode);
            *key_rtrn = key_kc.unwrap_or(XKB_KEYCODE_INVALID);
            if *key_rtrn == XKB_KEYCODE_INVALID {
                return false;
            }
            true
        }
        10 => {
            let ExprKind::Ident(ident_val) = expr else {
                unreachable!()
            };
            let id: &str = atom_text(&keymap_info.keymap.ctx.atom_table, *ident_val);
            if !id.is_empty() && id.eq_ignore_ascii_case("none") {
                *key_rtrn = XKB_KEYCODE_INVALID;
                return true;
            } else if !id.is_empty() && id.eq_ignore_ascii_case("any") {
                *key_rtrn = XKB_KEYCODE_INVALID;
                *overlay_rtrn = XKB_OVERLAY_INVALID as u8;
                return true;
            }
            false
        }
        _ => false,
    }
}
enum SymbolsField {
    Type,
    Symbols,
    Actions,
    Vmods,
    Locking,
    RadioGroup,
    Overlay,
    Repeat,
    GroupsWrap,
    GroupsClamp,
    GroupsRedirect,
}

fn parse_symbols_field(field: &str) -> Option<SymbolsField> {
    if field.eq_ignore_ascii_case("type") {
        Some(SymbolsField::Type)
    } else if field.eq_ignore_ascii_case("symbols") {
        Some(SymbolsField::Symbols)
    } else if field.eq_ignore_ascii_case("actions") {
        Some(SymbolsField::Actions)
    } else if field.eq_ignore_ascii_case("vmods")
        || field.eq_ignore_ascii_case("virtualmods")
        || field.eq_ignore_ascii_case("virtualmodifiers")
    {
        Some(SymbolsField::Vmods)
    } else if field.eq_ignore_ascii_case("locking")
        || field.eq_ignore_ascii_case("lock")
        || field.eq_ignore_ascii_case("locks")
    {
        Some(SymbolsField::Locking)
    } else if field.eq_ignore_ascii_case("radiogroup")
        || field.eq_ignore_ascii_case("permanentradiogroup")
        || field.eq_ignore_ascii_case("allownone")
    {
        Some(SymbolsField::RadioGroup)
    } else if field
        .get(..16)
        .is_some_and(|s| s.eq_ignore_ascii_case("permanentoverlay"))
    {
        Some(SymbolsField::Locking)
    } else if field
        .get(..7)
        .is_some_and(|s| s.eq_ignore_ascii_case("overlay"))
    {
        Some(SymbolsField::Overlay)
    } else if field.eq_ignore_ascii_case("repeating")
        || field.eq_ignore_ascii_case("repeats")
        || field.eq_ignore_ascii_case("repeat")
    {
        Some(SymbolsField::Repeat)
    } else if field.eq_ignore_ascii_case("groupswrap") || field.eq_ignore_ascii_case("wrapgroups") {
        Some(SymbolsField::GroupsWrap)
    } else if field.eq_ignore_ascii_case("groupsclamp") || field.eq_ignore_ascii_case("clampgroups")
    {
        Some(SymbolsField::GroupsClamp)
    } else if field.eq_ignore_ascii_case("groupsredirect")
        || field.eq_ignore_ascii_case("redirectgroups")
    {
        Some(SymbolsField::GroupsRedirect)
    } else {
        None
    }
}

fn set_symbols_field(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<Box<ExprKind>>,
) -> bool {
    let mapped_field = match parse_symbols_field(field) {
        Some(f) => f,
        None => return ki.strict & PARSER_NO_UNKNOWN_KEY_FIELDS == 0,
    };

    match mapped_field {
        SymbolsField::Type => {
            let mut ndx: u32 = 0;
            let mut val: u32 = XKB_ATOM_NONE;
            if !expr_resolve_string(value_opt.as_deref().unwrap(), &mut val) {
                return false;
            }
            if array_ndx.is_none() {
                keyi.default_type = val;
                keyi.defined |= KEY_FIELD_DEFAULT_TYPE;
            } else if {
                let mut _pending = false;
                expr_resolve_group(ki, array_ndx.unwrap(), false, &mut ndx, &mut _pending)
            } != PARSER_SUCCESS
            {
                return false;
            } else {
                ndx -= 1;
                if ndx >= keyi.groups.len() as u32 {
                    resize_groups_zero(&mut keyi.groups, (ndx as usize) + 1);
                }
                keyi.groups[ndx as usize].type_0 = val;
                keyi.groups[ndx as usize].defined |= GROUP_FIELD_TYPE;
            }
        }
        SymbolsField::Symbols => {
            return add_symbols_to_key(ki, info, keyi, array_ndx, value_opt.as_deref().unwrap());
        }
        SymbolsField::Actions => {
            return add_actions_to_key(
                ki,
                info,
                keyi,
                array_ndx,
                value_opt.as_mut().map(|b| &mut **b).unwrap(),
            );
        }
        SymbolsField::Vmods => {
            let mut mask: u32 = 0;
            let val = value_opt.as_deref().unwrap();
            if !expr_resolve_mod_mask(&ki.keymap.ctx, val, MOD_VIRT, &info.mods, &mut mask) {
                return false;
            }
            keyi.vmodmap = mask;
            keyi.defined |= KEY_FIELD_VMODMAP;
        }
        SymbolsField::Locking | SymbolsField::RadioGroup => {}
        SymbolsField::Overlay => {
            let mut overlay: u8 = XKB_OVERLAY_INVALID as u8;
            let mut key: u32 = XKB_KEYCODE_INVALID;
            if !expr_resolve_overlay_entry(
                ki,
                field,
                array_ndx,
                value_opt.as_deref().unwrap(),
                keyi,
                &mut overlay,
                &mut key,
            ) {
                return false;
            }
            if overlay as i32 == XKB_OVERLAY_INVALID {
                return true;
            } else if key != XKB_KEYCODE_INVALID && {
                ki.keymap.get_key(key).is_some_and(|k| k.name == keyi.name)
            } {
            } else {
                let mut prev: u32 = XKB_KEYCODE_INVALID;
                if overlays_get(keyi, overlay, Some(&mut prev)) {
                    if key != prev {}
                } else if ki.features.overlapping_overlays {
                    if !overlays_insert(keyi, overlay, key) {
                        return false;
                    }
                    keyi.defined |= KEY_FIELD_OVERLAY;
                } else {
                    let mask_0: u8 = (1_u32 << overlay as i32) as u8;
                    if keyi.overlays == 0 || keyi.overlays_clear {
                        if key != XKB_KEYCODE_INVALID {
                            keyi.overlays = mask_0;
                            keyi.overlays_clear = false;
                            keyi.overlay_keys = vec![key];
                        } else {
                            keyi.overlays = (keyi.overlays as i32 | mask_0 as i32) as u8;
                            keyi.overlays_clear = true;
                            keyi.overlay_keys = vec![XKB_KEYCODE_INVALID];
                        }
                        keyi.defined |= KEY_FIELD_OVERLAY;
                    } else if keyi.overlays != 0 && key != XKB_KEYCODE_INVALID {
                        let _existing_key = keyi
                            .overlay_keys
                            .first()
                            .copied()
                            .unwrap_or(XKB_KEYCODE_INVALID);
                        return ki.strict & PARSER_NO_FIELD_VALUE_MISMATCH == 0;
                    }
                }
            }
        }
        SymbolsField::Repeat => {
            let mut val_0: u32 = 0;
            if !expr_resolve_enum(
                &ki.keymap.ctx,
                value_opt.as_deref().unwrap(),
                &mut val_0,
                &REPEAT_ENTRIES,
            ) {
                return false;
            }
            keyi.repeat = val_0;
            keyi.defined |= KEY_FIELD_REPEAT;
        }
        SymbolsField::GroupsWrap => {
            let mut set: bool = false;
            if !expr_resolve_boolean(&ki.keymap.ctx, value_opt.as_deref().unwrap(), &mut set) {
                return false;
            }
            keyi.out_of_range_group_policy = if set {
                XKB_LAYOUT_OUT_OF_RANGE_WRAP
            } else {
                XKB_LAYOUT_OUT_OF_RANGE_CLAMP
            };
            keyi.defined |= KEY_FIELD_GROUPINFO;
        }
        SymbolsField::GroupsClamp => {
            let mut set_0: bool = false;
            if !expr_resolve_boolean(&ki.keymap.ctx, value_opt.as_deref().unwrap(), &mut set_0) {
                return false;
            }
            keyi.out_of_range_group_policy = if set_0 {
                XKB_LAYOUT_OUT_OF_RANGE_CLAMP
            } else {
                XKB_LAYOUT_OUT_OF_RANGE_WRAP
            };
            keyi.defined |= KEY_FIELD_GROUPINFO;
        }
        SymbolsField::GroupsRedirect => {
            let mut grp: u32 = 0;
            let mut pending: bool = false;
            if expr_resolve_group(
                ki,
                value_opt.as_deref().unwrap(),
                false,
                &mut grp,
                &mut pending,
            ) != PARSER_SUCCESS
                && !pending
            {
                return false;
            }
            if pending {
                keyi.out_of_range_pending_group = true;
                let pending_index: u32 = ki.pending_computations.len() as u32;
                ki.pending_computations.push(PendingComputation {
                    expr: value_opt.take(),
                    computed: false,
                    value: 0,
                });
                keyi.out_of_range_group_number = pending_index;
            } else {
                keyi.out_of_range_pending_group = false;
                keyi.out_of_range_group_number = grp - 1;
            }
            keyi.out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_REDIRECT;
            keyi.defined |= KEY_FIELD_GROUPINFO;
        }
    }
    true
}
fn set_group_name(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    merge: u32,
) -> bool {
    let array_ndx = match array_ndx {
        Some(a) => a,
        None => {
            return false;
        }
    };
    let mut group: u32 = 0;
    let mut _pending: bool = false;
    if { expr_resolve_group(ki, array_ndx, false, &mut group, &mut _pending) } != PARSER_SUCCESS {
        return false;
    }
    let mut name: u32 = XKB_ATOM_NONE;
    if !expr_resolve_string(value, &mut name) {
        return false;
    }
    let group_to_use: u32;
    if info.explicit_group == XKB_LAYOUT_INVALID {
        group_to_use = group.wrapping_sub(1);
    } else if group.wrapping_sub(1) == 0 {
        group_to_use = info.explicit_group;
    } else {
        return false;
    }
    if group_to_use >= info.group_names.len() as u32 {
        info.group_names.resize((group_to_use as usize) + 1, 0_u32);
    } else {
        let old_name: u32 = info.group_names[group_to_use as usize];
        if old_name != XKB_ATOM_NONE && old_name != name {
            let replace: bool = merge != MERGE_AUGMENT;
            let use_0: u32 = if replace { name } else { old_name };
            let _ignore: u32 = if replace { old_name } else { name };
            name = use_0;
        }
    }
    info.group_names[group_to_use as usize] = name;
    true
}
fn handle_global_var(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    stmt: &mut VarDef,
) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut array_ndx_opt: Option<&ExprKind> = None;
    let ret: bool;
    if !expr_resolve_lhs(
        stmt.name.as_deref().unwrap(),
        &mut elem_atom,
        &mut field_atom,
        &mut array_ndx_opt,
    ) {
        return false;
    }
    let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom).to_owned();
    let field = atom_text(&ki.keymap.ctx.atom_table, field_atom).to_owned();
    let elem: &str = &elem;
    let field: &str = &field;
    if !elem.is_empty() && elem.eq_ignore_ascii_case("key") {
        let mut temp: KeyInfo = {
            let mut init = KeyInfo::new_zeroed();
            init.out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_WRAP;
            init.defined = 0;
            init.merge = MERGE_DEFAULT;
            init.repeat = KEY_REPEAT_UNDEFINED;
            init.out_of_range_pending_group = false;
            init.overlays_clear = false;
            init
        };
        init_key_info_with_atom(&mut temp, atom_intern(&mut ki.keymap.ctx.atom_table, b"*"));
        temp.merge = if temp.merge == MERGE_REPLACE {
            MERGE_OVERRIDE
        } else {
            stmt.merge as u32
        };
        ret = set_symbols_field(ki, info, &mut temp, field, array_ndx_opt, &mut stmt.value);
        let mut dk = std::mem::replace(&mut info.default_key, KeyInfo::new_zeroed());
        merge_keys(ki, info, &mut dk, &mut temp);
        info.default_key = dk;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("name") || field.eq_ignore_ascii_case("groupname"))
    {
        ret = set_group_name(
            ki,
            info,
            array_ndx_opt,
            stmt.value.as_deref().unwrap(),
            stmt.merge,
        );
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("groupswrap") || field.eq_ignore_ascii_case("wrapgroups"))
    {
        ret = true;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("groupsclamp") || field.eq_ignore_ascii_case("clampgroups"))
    {
        ret = true;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("groupsredirect")
            || field.eq_ignore_ascii_case("redirectgroups"))
    {
        ret = true;
    } else if elem.is_empty() && field.eq_ignore_ascii_case("allownone") {
        ret = true;
    } else if !elem.is_empty() {
        let elem_owned = elem.to_owned();
        let field_owned = field.to_owned();
        ret = {
            set_default_action_field(
                ki,
                &mut info.default_actions,
                &mut info.mods,
                &elem_owned,
                &field_owned,
                array_ndx_opt,
                &mut stmt.value,
                stmt.merge,
            ) as u32
                != PARSER_FATAL_ERROR
        };
    } else {
        return ki.strict & PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS == 0;
    }
    ret
}
fn handle_symbols_body(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    defs: &mut [VarDef],
    keyi: &mut KeyInfo,
) -> bool {
    let mut all_valid_entries: bool = true;
    for def in defs.iter_mut() {
        let field_owned: String;
        let field: &str;
        let mut array_ndx_opt: Option<&ExprKind> = None;
        let mut ok: bool = true;
        if def.name.is_none() {
            if def.value.is_none() || !is_action_list_value(def.value.as_ref().unwrap()) {
                field = "symbols";
            } else {
                field = "actions";
            }
        } else {
            let mut elem_atom: u32 = 0;
            let mut field_atom: u32 = 0;
            ok = expr_resolve_lhs(
                def.name.as_deref().unwrap(),
                &mut elem_atom,
                &mut field_atom,
                &mut array_ndx_opt,
            );
            let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom);
            field_owned = atom_text(&ki.keymap.ctx.atom_table, field_atom).to_owned();
            field = &field_owned;
            if ok && !elem.is_empty() {
                ok = false;
            }
        }
        if def.value.is_none() {
            ok = false;
        }
        if !ok || !set_symbols_field(ki, info, keyi, field, array_ndx_opt, &mut def.value) {
            all_valid_entries = false;
        }
    }
    all_valid_entries
}
fn set_explicit_group(_ki: &XkbKeymapInfo<'_>, info: &SymbolsInfo, keyi: &mut KeyInfo) -> bool {
    if info.explicit_group == XKB_LAYOUT_INVALID {
        return true;
    }
    if !keyi.groups.is_empty() {
        for group in keyi.groups[1..].iter_mut() {
            if group.defined != 0 {
                *group = GroupInfo::default();
            }
        }
    }

    keyi.groups
        .resize_with((info.explicit_group as usize) + 1, Default::default);
    if info.explicit_group > 0 {
        // Move groups[0] to groups[explicit_group], replace groups[0] with default
        keyi.groups[info.explicit_group as usize] = std::mem::take(&mut keyi.groups[0]);
    }
    true
}
fn handle_symbols_def(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    stmt: &mut SymbolsDef,
) -> bool {
    // Clone scalar fields from default_key, deep-copy groups
    let dk = &info.default_key;
    let mut keyi = dk.clone();
    keyi.merge = stmt.merge as u32;
    keyi.name = stmt.key_name;
    if handle_symbols_body(ki, info, &mut stmt.symbols, &mut keyi)
        && set_explicit_group(ki, info, &mut keyi)
        && add_key_symbols(ki, info, &mut keyi)
    {
        return true;
    }
    info.error_count += 1;
    false
}
fn handle_mod_map_def(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    def: &mut ModMapDef,
) -> bool {
    let mut tmp: ModMapEntry = ModMapEntry {
        merge: MERGE_DEFAULT,
        have_symbol: false,
        modifier: 0,
        u: 0,
    };
    let ndx: u32;
    let mut ok: bool;
    let modifier_name: &str = atom_text(&ki.keymap.ctx.atom_table, def.modifier);
    if modifier_name.eq_ignore_ascii_case("none") {
        ndx = XKB_MOD_NONE;
    } else {
        ndx = xkb_mod_name_to_index(&info.mods, def.modifier, MOD_REAL);
        if ndx == XKB_MOD_INVALID {
            return false;
        }
    }
    ok = true;
    tmp.modifier = ndx;
    tmp.merge = def.merge;
    for key in def.keys.iter() {
        let mut add_entry = false;
        if key.stmt_type() == STMT_EXPR_KEYNAME_LITERAL {
            tmp.have_symbol = false;
            let ExprKind::KeyName(kn) = key else {
                unreachable!()
            };
            tmp.u = *kn;
            add_entry = true;
        } else if key.stmt_type() == STMT_EXPR_KEYSYM_LITERAL {
            let ExprKind::KeySym(ks) = key else {
                unreachable!()
            };
            if *ks != XKB_KEY_NO_SYMBOL as u32 {
                tmp.have_symbol = true;
                tmp.u = *ks;
                add_entry = true;
            }
        }
        if add_entry {
            ok = add_mod_map_entry(info, &tmp) && ok;
        }
    }
    ok
}
fn handle_symbols_file(ki: &mut XkbKeymapInfo<'_>, info: &mut SymbolsInfo, file: &mut XkbFile) {
    {
        let mut ok: bool;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_symbols(ki, info, incl);
                }
                Statement::Symbols(sym) => {
                    ok = handle_symbols_def(ki, info, sym);
                }
                Statement::Var(var) => {
                    ok = handle_global_var(ki, info, var);
                }
                Statement::VMod(vmod) => {
                    ok = handle_vmod_def(&mut ki.keymap.ctx, &mut info.mods, vmod);
                }
                Statement::ModMap(mm) => {
                    ok = handle_mod_map_def(ki, info, mm);
                }
                Statement::Unknown(_unk) => {
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    ok = false;
                }
            }
            if !ok {
                info.error_count += 1;
            }
            if info.error_count > 10 {
                break;
            }
        }
    }
}
fn find_key_for_symbol(keymap: &mut XkbKeymap, sym: u32) -> Option<&mut XkbKey> {
    let mut got_one_group: bool;
    let mut group: u32 = 0;
    loop {
        let mut level: u32 = 0;
        got_one_group = false;
        let mut got_one_level: bool;
        loop {
            got_one_level = false;
            let start_idx = if keymap.num_keys_low == 0 {
                0_u32
            } else {
                keymap.min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < keymap.num_keys {
                let key = &keymap.keys[ki as usize];
                if group < key.num_groups
                    && level < keymap.types[key.groups[group as usize].type_idx as usize].num_levels
                {
                    got_one_level = true;
                    got_one_group = got_one_level;
                    let level_syms = &key.groups[group as usize].levels[level as usize].syms;
                    if level_syms.contains(&sym) {
                        return Some(&mut keymap.keys[ki as usize]);
                    }
                }
                ki += 1;
            }
            level += 1;
            if !got_one_level {
                break;
            }
        }
        group += 1;
        if !got_one_group {
            break;
        }
    }
    None
}
fn find_automatic_type(ctx: &mut XkbContext, groupi: &GroupInfo) -> u32 {
    let width: u32 = groupi.levels.len() as u32;
    if width == 1 || width == 0 {
        return atom_intern(&mut ctx.atom_table, b"ONE_LEVEL");
    }
    let sym0: u32 = if groupi.levels[0].syms.is_empty() {
        XKB_KEY_NO_SYMBOL as u32
    } else {
        groupi.levels[0].syms[0]
    };
    let sym1: u32 = if groupi.levels[1].syms.is_empty() {
        XKB_KEY_NO_SYMBOL as u32
    } else {
        groupi.levels[1].syms[0]
    };
    if width == 2_u32 {
        if xkb_keysym_is_lower(sym0) && xkb_keysym_is_upper_or_title(sym1) {
            return atom_intern(&mut ctx.atom_table, b"ALPHABETIC");
        }
        if xkb_keysym_is_keypad(sym0) || xkb_keysym_is_keypad(sym1) {
            return atom_intern(&mut ctx.atom_table, b"KEYPAD");
        }
        return atom_intern(&mut ctx.atom_table, b"TWO_LEVEL");
    }
    if width <= 4_u32 {
        if xkb_keysym_is_lower(sym0) && xkb_keysym_is_upper_or_title(sym1) {
            let sym2: u32 = if groupi.levels[2].syms.is_empty() {
                XKB_KEY_NO_SYMBOL as u32
            } else {
                groupi.levels[2].syms[0]
            };
            let sym3: u32 = if width == 4_u32 {
                if groupi.levels[3].syms.is_empty() {
                    XKB_KEY_NO_SYMBOL as u32
                } else {
                    groupi.levels[3].syms[0]
                }
            } else {
                XKB_KEY_NO_SYMBOL as u32
            };
            if xkb_keysym_is_lower(sym2) && xkb_keysym_is_upper_or_title(sym3) {
                return atom_intern(&mut ctx.atom_table, b"FOUR_LEVEL_ALPHABETIC");
            }
            return atom_intern(&mut ctx.atom_table, b"FOUR_LEVEL_SEMIALPHABETIC");
        }
        if xkb_keysym_is_keypad(sym0) || xkb_keysym_is_keypad(sym1) {
            return atom_intern(&mut ctx.atom_table, b"FOUR_LEVEL_KEYPAD");
        }
        return atom_intern(&mut ctx.atom_table, b"FOUR_LEVEL");
    }
    XKB_ATOM_NONE
}
fn find_type_for_group(
    keymap: &mut XkbKeymap,
    keyi: &mut KeyInfo,
    group: u32,
    explicit_type: &mut bool,
) -> u32 {
    let groupi = &keyi.groups[group as usize];
    let mut type_name: u32 = groupi.type_0;
    *explicit_type = true;
    if type_name == XKB_ATOM_NONE {
        if keyi.default_type != XKB_ATOM_NONE {
            type_name = keyi.default_type;
        } else {
            type_name = find_automatic_type(&mut keymap.ctx, groupi);
            if type_name != XKB_ATOM_NONE {
                *explicit_type = false;
            }
        }
    }
    if type_name != XKB_ATOM_NONE {
        let mut i: u32 = 0;
        while (i as usize) < keymap.types.len() {
            if keymap.types[i as usize].name == type_name {
                break;
            }
            i += 1;
        }
        if (i as usize) < keymap.types.len() {
            keymap.types[i as usize].required = true;
            return i;
        }
    }
    keymap.types[0].required = true;
    0
}
fn copy_symbols_def_to_keymap(
    keymap: &mut XkbKeymap,
    _info: &SymbolsInfo,
    keyi: &mut KeyInfo,
) -> bool {
    let mut i: u32;

    // The name is guaranteed to be real and not an alias, so 'false' is safe here
    // Look up key index to avoid holding a mutable borrow on keymap
    let key_idx = if (keyi.name as usize) < keymap.key_names.len() {
        let match_0 = keymap.key_names[keyi.name as usize];
        if match_0.found && !match_0.is_alias {
            Some(match_0.index as usize)
        } else {
            None
        }
    } else {
        None
    };
    let key_idx = match key_idx {
        Some(idx) => idx,
        None => {
            return false;
        }
    };

    // Find the range of groups we need
    keymap.keys[key_idx].num_groups = 0;
    if !keyi.groups.is_empty() {
        for (idx, groupi) in keyi.groups.iter().enumerate() {
            let has_explicit_type = ((keyi.defined & KEY_FIELD_DEFAULT_TYPE) != 0)
                || (groupi.defined & GROUP_FIELD_TYPE != 0);
            if !groupi.levels.is_empty() || has_explicit_type {
                keymap.keys[key_idx].num_groups = (idx as u32) + 1;
            }
            if has_explicit_type {
                keymap.keys[key_idx].explicit = keymap.keys[key_idx].explicit | EXPLICIT_TYPES;
            }
        }
    }

    if keymap.keys[key_idx].num_groups == 0 {
        // A key with no group may still have other fields defined
        if keyi.defined != 0 {
            // goto key_fields
        } else {
            return false;
        }
    } else {
        // Resize groups array
        let __need: usize = keymap.keys[key_idx].num_groups as usize;
        resize_groups_zero(&mut keyi.groups, __need);

        // If there are empty groups between non-empty ones, fill them with data from the first group
        if !keyi.groups.is_empty() {
            let groups_len = keyi.groups.len();
            i = 1;
            while i < groups_len as u32 {
                if keyi.groups[i as usize].defined == 0 {
                    let src = keyi.groups[0].clone();
                    keyi.groups[i as usize] = src;
                }
                i += 1;
            }
        }

        keymap.keys[key_idx].groups = (0..keymap.keys[key_idx].num_groups)
            .map(|_| XkbGroup {
                explicit_symbols: false,
                explicit_actions: false,
                implicit_actions: false,
                explicit_type: false,
                type_idx: 0,
                levels: Vec::new(),
            })
            .collect();

        // Find and assign the groups' types in the keymap
        if !keyi.groups.is_empty() {
            i = 0;
            while i < keyi.groups.len() as u32 {
                let mut explicit_type = false;
                let type_idx: u32 = find_type_for_group(keymap, keyi, i, &mut explicit_type);

                // Always have as many levels as the type specifies
                if keymap.types[type_idx as usize].num_levels
                    < keyi.groups[i as usize].levels.len() as u32
                {
                    for lvl_idx in keymap.types[type_idx as usize].num_levels as usize
                        ..keyi.groups[i as usize].levels.len()
                    {
                        keyi.groups[i as usize].levels[lvl_idx].syms.clear();
                        keyi.groups[i as usize].levels[lvl_idx].actions.clear();
                    }
                }

                // Resize levels array to match type
                let __need_levels: usize = keymap.types[type_idx as usize].num_levels as usize;
                keyi.groups[i as usize]
                    .levels
                    .resize_with(__need_levels, Default::default);

                keymap.keys[key_idx].groups[i as usize].explicit_type = explicit_type;
                keymap.keys[key_idx].groups[i as usize].type_idx = type_idx;

                i += 1;
            }
        }

        // Copy levels
        if !keyi.groups.is_empty() {
            i = 0;
            while i < keyi.groups.len() as u32 {
                let groupi = &mut keyi.groups[i as usize];
                // Compute the capitalization transformation of the keysyms
                for li in 0..groupi.levels.len() {
                    let leveli = &mut groupi.levels[li];
                    match leveli.syms.len() {
                        0 => {
                            leveli.upper = XKB_KEY_NO_SYMBOL as u32;
                        }
                        1 => {
                            leveli.upper = xkb_keysym_to_upper(leveli.syms[0]);
                        }
                        _ => {
                            // Multiple keysyms: check if there is any cased keysym
                            leveli.has_upper = false;
                            let num_syms = leveli.syms.len();
                            for k in 0..num_syms {
                                let upper: u32 = xkb_keysym_to_upper(leveli.syms[k]);
                                if upper != leveli.syms[k] {
                                    leveli.has_upper = true;
                                    break;
                                }
                            }
                            if leveli.has_upper {
                                // Some cased keysyms: store the transformation result
                                let num_syms = leveli.syms.len();
                                leveli.syms.reserve(num_syms);
                                for k in 0..num_syms {
                                    let upper = xkb_keysym_to_upper(leveli.syms[k]);
                                    leveli.syms.push(upper);
                                }
                            }
                        }
                    }
                }

                // Copy the level (steal from Vec)
                if groupi.levels.is_empty() {
                    keymap.keys[key_idx].groups[i as usize].levels = Vec::new();
                } else {
                    let stolen = std::mem::take(&mut groupi.levels);
                    keymap.keys[key_idx].groups[i as usize].levels = stolen;
                }

                let type_idx = keymap.keys[key_idx].groups[i as usize].type_idx;
                if keymap.types[type_idx as usize].num_levels > 1
                    || !keymap.keys[key_idx].groups[i as usize].levels[0]
                        .syms
                        .is_empty()
                {
                    keymap.keys[key_idx].groups[i as usize].explicit_symbols = true;
                    keymap.keys[key_idx].explicit =
                        keymap.keys[key_idx].explicit | EXPLICIT_SYMBOLS;
                }
                if groupi.defined & GROUP_FIELD_ACTS != 0 {
                    keymap.keys[key_idx].groups[i as usize].explicit_actions = true;
                    keymap.keys[key_idx].explicit = keymap.keys[key_idx].explicit | EXPLICIT_INTERP;
                }
                if keymap.keys[key_idx].groups[i as usize].explicit_type {
                    keymap.keys[key_idx].explicit = keymap.keys[key_idx].explicit | EXPLICIT_TYPES;
                }

                i += 1;
            }
        }

        keymap.keys[key_idx].out_of_range_pending_group = keyi.out_of_range_pending_group;
        keymap.keys[key_idx].out_of_range_group_number = keyi.out_of_range_group_number;
        keymap.keys[key_idx].out_of_range_group_policy = keyi.out_of_range_group_policy;
    }

    // key_fields:
    if (keyi.defined & KEY_FIELD_VMODMAP) != 0 {
        keymap.keys[key_idx].vmodmap = keyi.vmodmap;
        keymap.keys[key_idx].explicit = keymap.keys[key_idx].explicit | EXPLICIT_VMODMAP;
    }

    if keyi.repeat != KEY_REPEAT_UNDEFINED {
        keymap.keys[key_idx].repeats = keyi.repeat == KEY_REPEAT_YES;
        keymap.keys[key_idx].explicit = keymap.keys[key_idx].explicit | EXPLICIT_REPEAT;
    }

    if ((keyi.defined & KEY_FIELD_OVERLAY) != 0) && keyi.overlays != 0 && !keyi.overlays_clear {
        // Remove null entries from overlay_keys and clear corresponding bits
        let mut clean_overlays: u8 = 0;
        let mut clean_keys: Vec<u32> = Vec::new();
        let mut remaining: u8 = keyi.overlays;
        let mut idx: usize = 0;
        while remaining != 0 {
            let lsb: u8 = remaining & (!remaining) + 1;
            remaining &= !lsb;
            let k = if idx < keyi.overlay_keys.len() {
                keyi.overlay_keys[idx]
            } else {
                XKB_KEYCODE_INVALID
            };
            idx += 1;
            if k != XKB_KEYCODE_INVALID {
                clean_overlays |= lsb;
                clean_keys.push(k);
            }
        }

        if clean_overlays != 0 {
            keymap.keys[key_idx].overlays = clean_overlays;
            keymap.keys[key_idx].overlay_keys = clean_keys;
            keymap.keys[key_idx].explicit = keymap.keys[key_idx].explicit | EXPLICIT_OVERLAY;
        }
    }

    true
}

fn copy_mod_map_def_to_keymap(
    keymap: &mut XkbKeymap,
    _info: &SymbolsInfo,
    entry: &ModMapEntry,
) -> bool {
    if !entry.have_symbol {
        if let Some(key) = keymap.key_by_name_mut(entry.u, true) {
            if entry.modifier != XKB_MOD_NONE {
                key.modmap |= 1_u32 << entry.modifier;
            }
            true
        } else {
            false
        }
    } else if let Some(key) = find_key_for_symbol(keymap, entry.u) {
        if entry.modifier != XKB_MOD_NONE {
            key.modmap |= 1_u32 << entry.modifier;
        }
        true
    } else {
        false
    }
}
fn copy_symbols_to_keymap(keymap: &mut XkbKeymap, info: &mut SymbolsInfo) -> bool {
    keymap.symbols_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut keymap.symbols_section_name);
    keymap.mods = info.mods;
    keymap.group_names = std::mem::take(&mut info.group_names);
    let mut keys = std::mem::take(&mut info.keys);
    for keyi in keys.iter_mut() {
        if !copy_symbols_def_to_keymap(keymap, info, keyi) {
            info.error_count += 1;
        }
    }
    info.keys = keys;
    for modmap in &info.modmaps {
        if !copy_mod_map_def_to_keymap(keymap, info, modmap) {
            info.error_count += 1;
        }
    }
    true
}
pub(crate) fn compile_symbols(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut info = SymbolsInfo::new(keymap_info);
    init_symbols_info(&mut info, keymap_info, 0_u32, &mods);
    if let Some(file) = file {
        handle_symbols_file(keymap_info, &mut info, file);
    }
    if (info.error_count == 0) && copy_symbols_to_keymap(keymap_info.keymap, &mut info) {
        return true;
    }
    false
}
use super::super::keysym::xkb_keysym_to_upper;
use super::super::shared_types::*;
pub(crate) struct CompatInfo {
    pub(crate) name: Option<String>,
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) default_interp: SymInterpInfo,
    pub(crate) interps: Vec<SymInterpInfo>,
    pub(crate) default_led: LedInfo,
    pub(crate) leds: [LedInfo; 32],
    pub(crate) num_leds: u32,
    pub(crate) default_actions: ActionsInfo,
    pub(crate) mods: XkbModSet,
}
impl Default for CompatInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatInfo {
    pub(crate) fn new() -> Self {
        let zeroed_led = LedInfo {
            defined: 0_u32,
            merge: MERGE_DEFAULT,
            led: XkbLed {
                name: 0,
                which_groups: 0,
                pending_groups: false,
                groups: 0,
                which_mods: 0_u32,
                mods: XkbMods { mods: 0, mask: 0 },
                ctrls: 0,
            },
        };
        Self {
            name: None,
            error_count: 0,
            include_depth: 0,
            default_interp: SymInterpInfo {
                defined: 0_u32,
                merge: MERGE_DEFAULT,
                interp: XkbSymInterpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat: false,
                    required: false,
                    num_actions: 0,
                    action: XkbAction::None,
                    actions: Vec::new(),
                },
            },
            interps: Vec::new(),
            default_led: zeroed_led,
            leds: [zeroed_led; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [XkbAction::None; 21],
            },
            mods: XkbModSet {
                mods: [XkbMod {
                    name: 0,
                    type_0: 0,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
        }
    }
}
#[derive(Copy, Clone)]
pub(crate) struct LedInfo {
    pub(crate) defined: u32,
    pub(crate) merge: u32,
    pub(crate) led: XkbLed,
}
pub(crate) const LED_FIELD_CTRLS: u32 = 4;
pub(crate) const LED_FIELD_GROUPS: u32 = 2;
pub(crate) const LED_FIELD_MODS: u32 = 1;
// C2Rust_Unnamed_18 removed: replaced by Vec<SymInterpInfo>
#[derive(Clone)]
pub(crate) struct SymInterpInfo {
    pub(crate) defined: u32,
    pub(crate) merge: u32,
    pub(crate) interp: XkbSymInterpret,
}
pub(crate) const SI_FIELD_LEVEL_ONE_ONLY: u32 = 8;
pub(crate) const SI_FIELD_AUTO_REPEAT: u32 = 4;
pub(crate) const SI_FIELD_ACTION: u32 = 2;
pub(crate) const SI_FIELD_VIRTUAL_MOD: u32 = 1;
// C2Rust_Unnamed_19 removed: replaced by Vec<XkbSymInterpret>
pub(crate) struct Collect {
    pub(crate) sym_interprets: Vec<XkbSymInterpret>,
}
// C2Rust_Unnamed_20 removed: replaced by Vec<XkbAction>
#[inline]
fn init_interp(info: &mut SymInterpInfo) {
    info.merge = MERGE_DEFAULT;
    info.interp.virtual_mod = XKB_MOD_INVALID;
}
#[inline]
fn init_led(info: &mut LedInfo) {
    info.merge = MERGE_DEFAULT;
}
fn init_compat_info(
    ki: &XkbKeymapInfo<'_>,
    info: &mut CompatInfo,
    include_depth: u32,
    mods: &XkbModSet,
) {
    info.include_depth = include_depth;
    init_actions_info(&*ki.keymap, &mut info.default_actions);
    init_vmods(&mut info.mods, mods, include_depth > 0);
    init_interp(&mut info.default_interp);
    init_led(&mut info.default_led);
}

fn merge_interp(
    old: &mut SymInterpInfo,
    new: &mut SymInterpInfo,
) -> bool {
    let clobber: bool = new.merge != MERGE_AUGMENT;
    if new.merge == MERGE_REPLACE {
        *old = new.clone();
        return true;
    }
    if use_new_field(
        SI_FIELD_VIRTUAL_MOD,
        old.defined,
        new.defined,
        clobber,
    ) {
        old.interp.virtual_mod = new.interp.virtual_mod;
        old.defined |= SI_FIELD_VIRTUAL_MOD;
    }
    if use_new_field(
        SI_FIELD_ACTION,
        old.defined,
        new.defined,
        clobber,
    ) {
        if old.interp.num_actions as i32 > 1_i32 {
            old.interp.actions.clear();
        }
        old.interp.num_actions = new.interp.num_actions;
        if new.interp.num_actions as i32 > 1_i32 {
            old.interp.actions = std::mem::take(&mut new.interp.actions);
            new.interp.action = XkbAction::None;
            new.interp.num_actions = 0_u16;
        } else {
            old.interp.action = new.interp.action;
        }
        old.defined |= SI_FIELD_ACTION;
    }
    if use_new_field(
        SI_FIELD_AUTO_REPEAT,
        old.defined,
        new.defined,
        clobber
    ) {
        old.interp.repeat = new.interp.repeat;
        old.defined |= SI_FIELD_AUTO_REPEAT;
    }
    if use_new_field(
        SI_FIELD_LEVEL_ONE_ONLY,
        old.defined,
        new.defined,
        clobber,
    ) {
        old.interp.level_one_only = new.interp.level_one_only;
        old.defined |= SI_FIELD_LEVEL_ONE_ONLY;
    }
    true
}
fn add_interp(
    info: &mut CompatInfo,
    new: &mut SymInterpInfo,
) -> bool {
    // FindMatchingInterp inlined
    let old_idx = info.interps.iter().position(|i| {
        i.interp.sym == new.interp.sym
            && i.interp.mods == new.interp.mods
            && i.interp.match_0 == new.interp.match_0
    });
    if let Some(idx) = old_idx {
        // Clone the old element out to avoid borrow conflict with info
        let mut old = info.interps[idx].clone();
        let result = merge_interp(&mut old, new);
        info.interps[idx] = old;
        return result;
    }
    info.interps.push(new.clone());
    true
}
fn resolve_state_and_predicate(
    expr: Option<&ExprKind>,
    pred_rtrn: &mut u32,
    mods_rtrn: &mut u32,
    info: &mut CompatInfo,
    ki: &XkbKeymapInfo<'_>,
) -> bool {
    let expr = match expr {
        None => {
            *pred_rtrn = MATCH_ANY_OR_NONE;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        Some(e) => e,
    };
    *pred_rtrn = MATCH_EXACTLY;
    let resolve_expr: &ExprKind;
    if expr.stmt_type() == STMT_EXPR_ACTION_DECL {
        let ExprKind::Action { name, args } = &expr else {
            unreachable!()
        };
        let pred_txt: &str = atom_text(&ki.keymap.ctx.atom_table, *name);
        let mut pred: u32 = 0;
        if !lookup_string(&SYM_INTERPRET_MATCH_MASK_NAMES, pred_txt, &mut pred)
            || args.is_empty()
            || args.len() != 1
        {
            return false;
        }
        *pred_rtrn = pred;
        resolve_expr = &args[0];
    } else if expr.stmt_type() == STMT_EXPR_IDENT {
        let ExprKind::Ident(ident_val) = &expr else {
            unreachable!()
        };
        let pred_txt_0: &str = atom_text(&ki.keymap.ctx.atom_table, *ident_val);
        if !pred_txt_0.is_empty() && pred_txt_0.eq_ignore_ascii_case("any") {
            *pred_rtrn = MATCH_ANY;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        resolve_expr = expr;
    } else {
        resolve_expr = expr;
    }
    expr_resolve_mod_mask(&ki.keymap.ctx, resolve_expr, MOD_REAL, &info.mods, mods_rtrn)
}

fn merge_led_map(
    old: &mut LedInfo,
    new: &mut LedInfo,
) -> bool {
    let clobber: bool = new.merge != MERGE_AUGMENT;
    if old.led.mods.mods == new.led.mods.mods
        && old.led.pending_groups == new.led.pending_groups
        && old.led.groups == new.led.groups
        && old.led.ctrls == new.led.ctrls
        && old.led.which_mods == new.led.which_mods
        && old.led.which_groups as i32 == new.led.which_groups as i32
    {
        old.defined |= new.defined;
        return true;
    }
    if new.merge == MERGE_REPLACE {
        *old = *new;
        return true;
    }
    if use_new_field(
        LED_FIELD_MODS,
        old.defined,
        new.defined,
        clobber,
    ) {
        old.led.which_mods = new.led.which_mods;
        old.led.mods = new.led.mods;
        old.defined |= LED_FIELD_MODS;
    }
    if use_new_field(
        LED_FIELD_GROUPS,
        old.defined,
        new.defined,
        clobber,
    ) {
        old.led.which_groups = new.led.which_groups;
        old.led.groups = new.led.groups;
        old.led.pending_groups = new.led.pending_groups;
        old.defined |= LED_FIELD_GROUPS;
    }
    if use_new_field(
        LED_FIELD_CTRLS,
        old.defined,
        new.defined,
        clobber,

    ) {
        old.led.ctrls = new.led.ctrls;
        old.defined |= LED_FIELD_CTRLS;
    }
    true
}
fn add_led_map(
    info: &mut CompatInfo,
    new: &mut LedInfo,
) -> bool {
    let mut i: u32 = 0;
    while i < info.num_leds {
        if info.leds[i as usize].led.name != new.led.name {
            i += 1;
        } else {
            // Clone the old element out to avoid borrow conflict with info
            let mut old = info.leds[i as usize];
            let result = merge_led_map(&mut old, new);
            info.leds[i as usize] = old;
            return result;
        }
    }
    if info.num_leds >= XKB_MAX_LEDS {
        return false;
    }
    info.leds[info.num_leds as usize] = *new;
    info.num_leds += 1;
    true
}
fn merge_included_compat_maps(
    ki: &mut XkbKeymapInfo<'_>,
    into: &mut CompatInfo,
    from: &mut CompatInfo,
    merge: u32,
) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut ki.keymap.ctx, &mut into.mods, &from.mods, merge);
    if into.name.is_none() {
        into.name = from.name.take();
    }
    if into.interps.is_empty() {
        into.interps = std::mem::take(&mut from.interps);
    } else {
        for interp in from.interps.iter_mut() {
            interp.merge = merge;
            if !add_interp(into, interp) {
                into.error_count += 1;
            }
        }
    }
    if into.num_leds == 0 {
        let n = from.num_leds as usize;
        into.leds[..n].copy_from_slice(&from.leds[..n]);
        into.num_leds = from.num_leds;
        from.num_leds = 0;
    } else {
        for led in from.leds[..from.num_leds as usize].iter_mut() {
            led.merge = merge;
            if !add_led_map(into, led) {
                into.error_count += 1;
            }
        }
    };
}
fn handle_include_compat_map(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut CompatInfo,
    include: &mut IncludeStmt,
) -> bool {
    let mut included = CompatInfo::new();
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    init_compat_info(
        ki,
        &mut included,
        info.include_depth.wrapping_add(1),
        &info.mods,
    );
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(include.stmt.clone())
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = CompatInfo::new();

        let file: Option<Box<XkbFile>> = process_include_file(&mut ki.keymap.ctx, stmt, FILE_TYPE_COMPAT);
        let Some(mut file) = file else {
            info.error_count += 10;
            return false;
        };
        init_compat_info(
            ki,
            &mut next_incl,
            info.include_depth.wrapping_add(1),
            &included.mods,
        );
        next_incl.default_interp = info.default_interp.clone();
        next_incl.default_led = info.default_led;
        handle_compat_map_file(ki, &mut next_incl, &mut file);
        merge_included_compat_maps(ki, &mut included, &mut next_incl, stmt.merge);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    merge_included_compat_maps(ki, info, &mut included, include.merge);
    info.error_count == 0
}
enum InterpField {
    Action,
    VirtualModifier,
    Repeat,
    Locking,
    UseModMap,
}

fn parse_interp_field(field: &str) -> Option<InterpField> {
    if field.eq_ignore_ascii_case("action") {
        Some(InterpField::Action)
    } else if field.eq_ignore_ascii_case("virtualmodifier")
        || field.eq_ignore_ascii_case("virtualmod")
    {
        Some(InterpField::VirtualModifier)
    } else if field.eq_ignore_ascii_case("repeat") {
        Some(InterpField::Repeat)
    } else if field.eq_ignore_ascii_case("locking") {
        Some(InterpField::Locking)
    } else if field.eq_ignore_ascii_case("usemodmap") || field.eq_ignore_ascii_case("usemodmapmods")
    {
        Some(InterpField::UseModMap)
    } else {
        None
    }
}

fn set_interp_field(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    si: &mut SymInterpInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value: &mut ExprKind,
) -> bool {
    let mapped_field = match parse_interp_field(field) {
        Some(f) => f,
        None => return ki.strict & PARSER_NO_UNKNOWN_INTERPRET_FIELDS == 0,
    };

    match mapped_field {
        InterpField::Action => {
            if array_ndx.is_some() {
                return false;
            }
            if value.stmt_type() == STMT_EXPR_ACTION_LIST {
                let ExprKind::ActionList {
                    actions: action_vec,
                } = value
                else {
                    unreachable!()
                };
                let num_actions: u32 = action_vec.len() as u32;
                if num_actions > MAX_ACTIONS_PER_LEVEL as u32 {
                    return false;
                }
                si.interp.num_actions = 0_u16;
                si.interp.action.set_none();
                let mut actions: Vec<XkbAction> = Vec::new();
                for act_expr in action_vec.iter_mut() {
                    let mut to_act: XkbAction = XkbAction::None;
                    match handle_action_def(
                        ki,
                        &mut info.default_actions,
                        &info.mods,
                        act_expr,
                        &mut to_act,
                    ) {
                        1 => {
                            to_act.set_none();
                        }
                        2 => {
                            drop(actions);
                            return false;
                        }
                        _ => {}
                    }
                    if to_act.action_type() != ACTION_TYPE_NONE {
                        if num_actions == 1 {
                            si.interp.num_actions = 1_u16;
                            si.interp.action = to_act;
                        } else {
                            actions.push(to_act);
                        }
                    }
                }
                match actions.len() as u32 {
                    0 => {}
                    1 => {
                        si.interp.num_actions = 1_u16;
                        si.interp.action = actions[0];
                    }
                    _ => {
                        si.interp.num_actions = actions.len() as u16;
                        si.interp.actions = actions;
                    }
                }
            } else {
                match handle_action_def(
                    ki,
                    &mut info.default_actions,
                    &info.mods,
                    value,
                    &mut si.interp.action,
                ) {
                    1 => {
                        si.interp.action.set_none();
                        si.interp.num_actions = 0_u16;
                    }
                    2 => return false,
                    _ => {
                        si.interp.num_actions =
                            (si.interp.action.action_type() != ACTION_TYPE_NONE) as i32 as u16;
                    }
                }
            }
            si.defined |= SI_FIELD_ACTION;
        }
        InterpField::VirtualModifier => {
            if array_ndx.is_some() {
                return false;
            }
            let mut ndx: u32 = 0;
            if !expr_resolve_mod(&ki.keymap.ctx, value, MOD_VIRT, &info.mods, &mut ndx) {
                return false;
            }
            si.interp.virtual_mod = ndx;
            si.defined |= SI_FIELD_VIRTUAL_MOD;
        }
        InterpField::Repeat => {
            let mut set: bool = false;
            if array_ndx.is_some() {
                return false;
            }
            if !expr_resolve_boolean(&ki.keymap.ctx, value, &mut set) {
                return false;
            }
            si.interp.repeat = set;
            si.defined |= SI_FIELD_AUTO_REPEAT;
        }
        InterpField::Locking => {}
        InterpField::UseModMap => {
            let mut val: u32 = 0;
            if array_ndx.is_some() {
                return false;
            }
            if !expr_resolve_enum(&ki.keymap.ctx, value, &mut val, &USE_MOD_MAP_VALUE_NAMES) {
                return false;
            }
            si.interp.level_one_only = val != 0;
            si.defined |= SI_FIELD_LEVEL_ONE_ONLY;
        }
    }
    true
}
enum LedMapField {
    Modifiers,
    Groups,
    Controls,
    AllowExplicit,
    WhichMods,
    WhichGroups,
    Index,
}

fn parse_led_map_field(field: &str) -> Option<LedMapField> {
    if field.eq_ignore_ascii_case("modifiers") || field.eq_ignore_ascii_case("mods") {
        Some(LedMapField::Modifiers)
    } else if field.eq_ignore_ascii_case("groups") {
        Some(LedMapField::Groups)
    } else if field.eq_ignore_ascii_case("controls") || field.eq_ignore_ascii_case("ctrls") {
        Some(LedMapField::Controls)
    } else if field.eq_ignore_ascii_case("allowexplicit") {
        Some(LedMapField::AllowExplicit)
    } else if field.eq_ignore_ascii_case("whichmodstate")
        || field.eq_ignore_ascii_case("whichmodifierstate")
    {
        Some(LedMapField::WhichMods)
    } else if field.eq_ignore_ascii_case("whichgroupstate") {
        Some(LedMapField::WhichGroups)
    } else if field.eq_ignore_ascii_case("driveskbd")
        || field.eq_ignore_ascii_case("driveskeyboard")
        || field.eq_ignore_ascii_case("leddriveskbd")
        || field.eq_ignore_ascii_case("leddriveskeyboard")
        || field.eq_ignore_ascii_case("indicatordriveskbd")
        || field.eq_ignore_ascii_case("indicatordriveskeyboard")
    {
        Some(LedMapField::AllowExplicit)
    } else if field.eq_ignore_ascii_case("index") {
        Some(LedMapField::Index)
    } else {
        None
    }
}

fn set_led_map_field(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    ledi: &mut LedInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<Box<ExprKind>>,
) -> bool {
    let value: &ExprKind = value_opt.as_deref().unwrap();
    let mapped_field = match parse_led_map_field(field) {
        Some(f) => f,
        None => return ki.strict & PARSER_NO_UNKNOWN_LED_FIELDS == 0,
    };

    match mapped_field {
        LedMapField::Modifiers => {
            if array_ndx.is_some() {
                return false;
            }
            if !expr_resolve_mod_mask(
                &ki.keymap.ctx,
                value,
                MOD_BOTH,
                &info.mods,
                &mut ledi.led.mods.mods,
            ) {
                return false;
            }
            ledi.defined |= LED_FIELD_MODS;
        }
        LedMapField::Groups => {
            let mut mask: u32 = 0;
            if array_ndx.is_some() {
                return false;
            }
            let mut pending: bool = false;
            if !expr_resolve_group_mask(ki, value, &mut mask, &mut pending) {
                if pending {
                    ledi.led.pending_groups = true;
                    let pending_index: u32 = ki.pending_computations.len() as u32;
                    ki.pending_computations.push(PendingComputation {
                        expr: value_opt.take(),
                        computed: false,
                        value: 0,
                    });
                    mask = pending_index;
                } else {
                    return false;
                }
            } else {
                ledi.led.pending_groups = false;
            }
            ledi.led.groups = mask;
            ledi.defined |= LED_FIELD_GROUPS;
        }
        LedMapField::Controls => {
            let mut mask_0: u32 = 0;
            if array_ndx.is_some() {
                return false;
            }
            let offset: u8 = ki.features.controls_name_offset;
            if !expr_resolve_mask(
                &ki.keymap.ctx,
                value,
                &mut mask_0,
                &CTRL_MASK_NAMES[offset as usize..],
            ) {
                return false;
            }
            ledi.led.ctrls = mask_0;
            ledi.defined |= LED_FIELD_CTRLS;
        }
        LedMapField::AllowExplicit | LedMapField::Index => {}
        LedMapField::WhichMods => {
            let mut mask_1: u32 = 0;
            if array_ndx.is_some() {
                return false;
            }
            if !expr_resolve_mask(&ki.keymap.ctx, value, &mut mask_1, &MOD_COMPONENT_MASK_NAMES) {
                return false;
            }
            ledi.led.which_mods = mask_1;
        }
        LedMapField::WhichGroups => {
            let mut mask_2: u32 = 0;
            if array_ndx.is_some() {
                return false;
            }
            if !expr_resolve_mask(&ki.keymap.ctx, value, &mut mask_2, &GROUP_COMPONENT_MASK_NAMES) {
                return false;
            }
            ledi.led.which_groups = mask_2;
        }
    }
    true
}
fn handle_compat_global_var(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    stmt: &mut VarDef,
) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut ndx: Option<&ExprKind> = None;
    let ret: bool;
    if !expr_resolve_lhs(
        stmt.name.as_deref().unwrap(),
        &mut elem_atom,
        &mut field_atom,
        &mut ndx,
    ) {
        ret = false;
    } else {
        let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom).to_owned();
        let field = atom_text(&ki.keymap.ctx.atom_table, field_atom).to_owned();
        if !elem.is_empty() && elem.eq_ignore_ascii_case("interpret") {
            let mut temp: SymInterpInfo = SymInterpInfo {
                defined: 0_u32,
                merge: MERGE_DEFAULT,
                interp: XkbSymInterpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat: false,
                    required: false,
                    num_actions: 0,
                    action: XkbAction::None,
                    actions: Vec::new(),
                },
            };
            init_interp(&mut temp);
            temp.merge = (if temp.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                stmt.merge
            }) as u32;
            // ndx borrows from stmt.name, value_ref borrows from stmt.value — disjoint fields
            let value_ref = stmt.value.as_deref_mut().unwrap();
            ret = set_interp_field(info, ki, &mut temp, &field, ndx, value_ref);
            if ret {
                let mut default = info.default_interp.clone();
                merge_interp(&mut default, &mut temp);
                info.default_interp = default;
            }
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("indicator") {
            let mut temp_0: LedInfo = LedInfo {
                defined: 0_u32,
                merge: MERGE_DEFAULT,
                led: XkbLed {
                    name: 0,
                    which_groups: 0,
                    pending_groups: false,
                    groups: 0,
                    which_mods: 0_u32,
                    mods: XkbMods { mods: 0, mask: 0 },
                    ctrls: 0,
                },
            };
            init_led(&mut temp_0);
            temp_0.merge = (if temp_0.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                stmt.merge
            }) as u32;
            // ndx borrows from stmt.name, field/value borrow from different fields — disjoint
            ret = set_led_map_field(info, ki, &mut temp_0, &field, ndx, &mut stmt.value);
            if ret {
                let mut default = info.default_led;
                merge_led_map(&mut default, &mut temp_0);
                info.default_led = default;
            }
        } else if !elem.is_empty() {
            ret = {
                let ndx_ref2 = ndx;

                set_default_action_field(
                    ki,
                    &mut info.default_actions,
                    &mut info.mods,
                    &elem,
                    &field,
                    ndx_ref2,
                    &mut stmt.value,
                    stmt.merge,
                ) as u32
                    != PARSER_FATAL_ERROR
            };
        } else {
            return ki.strict & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS == 0;
        }
    } // close else from expr_resolve_lhs
    ret
}
fn handle_interp_def(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    def: &mut InterpDef,
) -> bool {
    let mut pred: u32 = MATCH_NONE;
    let mut mods: u32 = 0;
    if !resolve_state_and_predicate(def.match_0.as_deref(), &mut pred, &mut mods, info, ki) {
        return false;
    }
    let mut si: SymInterpInfo = info.default_interp.clone();
    si.merge = def.merge;
    si.interp.sym = def.sym;
    si.interp.match_0 = pred;
    si.interp.mods = mods;
    for body_def in &mut def.def {
        let mut elem_atom: u32 = 0;
        let mut field_atom: u32 = 0;
        let mut array_ndx: Option<&ExprKind> = None;
        if !expr_resolve_lhs(
            body_def.name.as_deref().unwrap(),
            &mut elem_atom,
            &mut field_atom,
            &mut array_ndx,
        ) {
            info.error_count += 1;
            return false;
        }
        let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom).to_owned();
        let field = atom_text(&ki.keymap.ctx.atom_table, field_atom).to_owned();
        if !elem.is_empty() {
            info.error_count += 1;
            return false;
        }
        let value_ref = body_def.value.as_deref_mut().unwrap();
        if !set_interp_field(info, ki, &mut si, &field, array_ndx, value_ref) {
            info.error_count += 1;
            return false;
        }
    }
    if !add_interp(info, &mut si) {
        info.error_count += 1;
        return false;
    }
    true
}
fn handle_led_map_def(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    def: &mut LedMapDef,
) -> bool {
    let mut ledi: LedInfo = info.default_led;
    ledi.merge = def.merge;
    ledi.led.name = def.name;
    let mut ok: bool = true;
    for var in def.body.iter_mut() {
        let mut elem_atom: u32 = 0;
        let mut field_atom: u32 = 0;
        let mut array_ndx: Option<&ExprKind> = None;
        if !expr_resolve_lhs(
            var.name.as_deref().unwrap(),
            &mut elem_atom,
            &mut field_atom,
            &mut array_ndx,
        ) {
            ok = false;
        } else {
            let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom).to_owned();
            let field = atom_text(&ki.keymap.ctx.atom_table, field_atom).to_owned();
            if !elem.is_empty() {
                ok = false;
            } else if !set_led_map_field(info, ki, &mut ledi, &field, array_ndx, &mut var.value) {
                ok = false;
            }
        }
    }
    ok && add_led_map(info, &mut ledi)
}
fn handle_compat_map_file(ki: &mut XkbKeymapInfo<'_>, info: &mut CompatInfo, file: &mut XkbFile) {
    {
        let mut ok: bool;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_compat_map(ki, info, incl);
                }
                Statement::Interp(ip) => {
                    ok = handle_interp_def(info, ki, ip);
                }
                Statement::GroupCompat(_) => {
                    ok = true;
                }
                Statement::LedMap(lm) => {
                    ok = handle_led_map_def(info, ki, lm);
                }
                Statement::Var(var) => {
                    ok = handle_compat_global_var(info, ki, var);
                }
                Statement::VMod(vmod) => {
                    ok = handle_vmod_def(&mut ki.keymap.ctx, &mut info.mods, vmod);
                }
                Statement::Unknown(_unk) => {
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    ok = false;
                }
            }
            if !ok {
                info.error_count += 1;
            }
            if info.error_count > 10 {
                break;
            }
        }
    }
}
fn copy_interps(info: &CompatInfo, need_symbol: bool, pred: u32, collect: &mut Collect) {
    for si in &info.interps {
        if si.interp.match_0 == pred
            && (si.interp.sym != XKB_KEY_NO_SYMBOL as u32) as i32 == need_symbol as i32
        {
            collect.sym_interprets.push(si.interp.clone());
        }
    }
}
fn copy_led_map_defs_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut CompatInfo) {
    let mut idx: u32 = 0;
    while idx < info.num_leds {
        let ledi_led = info.leds[idx as usize].led;
        let is_default = std::ptr::eq(
            &info.leds[idx as usize] as *const LedInfo,
            &info.default_led as *const LedInfo,
        );
        let _led_name_text = if is_default {
            "default"
        } else {
            atom_text(&ki.keymap.ctx.atom_table, info.leds[idx as usize].led.name)
        };
        let mut i: u32;
        i = 0;
        while i < ki.keymap.num_leds {
            if ki.keymap.leds[i as usize].name == ledi_led.name {
                break;
            }
            i += 1;
        }
        let mut assign_led = false;
        if i >= ki.keymap.num_leds {
            i = 0;
            while i < ki.keymap.num_leds {
                if ki.keymap.leds[i as usize].name == XKB_ATOM_NONE {
                    break;
                }
                i += 1;
            }
            if i >= ki.keymap.num_leds {
                if i < XKB_MAX_LEDS {
                    i = ki.keymap.num_leds;
                    ki.keymap.num_leds += 1;
                    assign_led = true;
                }
            } else {
                assign_led = true;
            }
        } else {
            assign_led = true;
        }
        if assign_led {
            ki.keymap.leds[i as usize] = ledi_led;
            let led = &mut ki.keymap.leds[i as usize];
            if led.which_groups == 0 && (led.groups != 0 || led.pending_groups) {
                led.which_groups = XKB_STATE_LAYOUT_EFFECTIVE;
            }
            if led.which_mods == 0 && led.mods.mods != 0 {
                led.which_mods = XKB_STATE_MODS_EFFECTIVE;
            }
        }
        idx += 1;
    }
}
fn copy_compat_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut CompatInfo) -> bool {
    // Collect sym_interprets first (doesn't need keymap)
    let sym_interprets = if !info.interps.is_empty() {
        let mut collect: Collect = Collect {
            sym_interprets: Vec::with_capacity(info.interps.len()),
        };
        copy_interps(info, true, MATCH_EXACTLY, &mut collect);
        copy_interps(info, true, MATCH_ALL, &mut collect);
        copy_interps(info, true, MATCH_NONE, &mut collect);
        copy_interps(info, true, MATCH_ANY, &mut collect);
        copy_interps(info, true, MATCH_ANY_OR_NONE, &mut collect);
        copy_interps(info, false, MATCH_EXACTLY, &mut collect);
        copy_interps(info, false, MATCH_ALL, &mut collect);
        copy_interps(info, false, MATCH_NONE, &mut collect);
        copy_interps(info, false, MATCH_ANY, &mut collect);
        copy_interps(info, false, MATCH_ANY_OR_NONE, &mut collect);
        Some(collect.sym_interprets)
    } else {
        None
    };
    // Now get keymap and assign everything
    {
        ki.keymap.compat_section_name = match &info.name {
            Some(s) => s.clone(),
            None => String::new(),
        };
        xkb_escape_map_name(&mut ki.keymap.compat_section_name);
        ki.keymap.mods = info.mods;
        if let Some(interps) = sym_interprets {
            ki.keymap.sym_interprets = interps;
        }
    }
    // copy_led_map_defs_to_keymap needs keymap borrow ended; scope block ensures this
    copy_led_map_defs_to_keymap(ki, info);
    true
}
pub(crate) fn compile_compat_map(file: Option<&mut XkbFile>, ki: &mut XkbKeymapInfo<'_>) -> bool {
    let mods = ki.keymap.mods;
    let mut info = CompatInfo::new();
    init_compat_info(ki, &mut info, 0_u32, &mods);
    if let Some(file) = file {
        handle_compat_map_file(ki, &mut info, file);
    }
    if (info.error_count == 0) && copy_compat_to_keymap(ki, &mut info) {
        return true;
    }
    false
}
pub(crate) struct KeyTypesInfo {
    pub(crate) name: Option<String>,
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) types: Vec<KeyTypeInfo>,
    pub(crate) mods: XkbModSet,
}
impl Default for KeyTypesInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyTypesInfo {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            error_count: 0,
            include_depth: 0,
            types: Vec::new(),
            mods: Default::default(),
        }
    }
}
#[derive(Clone)]
pub(crate) struct KeyTypeInfo {
    pub(crate) defined: u32,
    pub(crate) merge: u32,
    pub(crate) name: u32,
    pub(crate) mods: u32,
    pub(crate) num_levels: u32,
    pub(crate) entries: Vec<XkbKeyTypeEntry>,
    pub(crate) level_names: Vec<u32>,
}
pub(crate) const TYPE_FIELD_LEVEL_NAME: u32 = 8;
pub(crate) const TYPE_FIELD_PRESERVE: u32 = 4;
pub(crate) const TYPE_FIELD_MAP: u32 = 2;
pub(crate) const TYPE_FIELD_MASK: u32 = 1;
fn init_key_types_info(info: &mut KeyTypesInfo, include_depth: u32, mods: &XkbModSet) {
    info.name = None;
    info.error_count = 0;
    info.include_depth = include_depth;
    info.types = Vec::new();
    info.mods = Default::default();
    init_vmods(&mut info.mods, mods, include_depth > 0);
}
fn add_key_type(
    info: &mut KeyTypesInfo,
    new: &mut KeyTypeInfo,
) -> bool {
    // FindMatchingKeyType inlined
    let old_idx = info.types.iter().position(|t| t.name == new.name);
    if let Some(idx) = old_idx {
        if new.merge != MERGE_AUGMENT {
            info.types[idx] = new.clone();
            new.entries = Vec::new();
            new.level_names = Vec::new();
            return true;
        }

        return true;
    }
    info.types.push(new.clone());
    true
}
fn merge_included_key_types(
    ki: &mut XkbKeymapInfo<'_>,
    into: &mut KeyTypesInfo,
    from: &mut KeyTypesInfo,
    merge: u32,
) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut ki.keymap.ctx, &mut into.mods, &from.mods, merge);
    if into.name.is_none() {
        into.name = from.name.take();
    }
    if into.types.is_empty() {
        into.types = std::mem::take(&mut from.types);
    } else {
        for type_0 in from.types.iter_mut() {
            type_0.merge = merge;
            let mut type_clone = type_0.clone();
            if !add_key_type(into, &mut type_clone) {
                into.error_count += 1;
            }
        }
        from.types.clear();
    }
}
fn handle_include_key_types(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    include: &mut IncludeStmt,
) -> bool {
    let mut included = KeyTypesInfo::new();
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    init_key_types_info(
        &mut included,
        info.include_depth.wrapping_add(1),
        &info.mods,
    );
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(std::mem::take(&mut include.stmt))
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = KeyTypesInfo::new();

        let file: Option<Box<XkbFile>> = process_include_file(&mut ki.keymap.ctx, stmt, FILE_TYPE_TYPES);
        let Some(mut file) = file else {
            info.error_count += 10;
            return false;
        };
        init_key_types_info(
            &mut next_incl,
            info.include_depth.wrapping_add(1),
            &included.mods,
        );
        handle_key_types_file(ki, &mut next_incl, &mut file);
        merge_included_key_types(ki, &mut included, &mut next_incl, stmt.merge);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    merge_included_key_types(ki, info, &mut included, include.merge);
    info.error_count == 0
}
fn set_modifiers(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let mut mods: u32 = 0;
    if array_ndx.is_some() {
        return false;
    }
    if !expr_resolve_mod_mask(&ki.keymap.ctx, value, MOD_BOTH, &info.mods, &mut mods) {
        return false;
    }
    if type_0.defined & TYPE_FIELD_MASK != 0 {
        return false;
    }
    type_0.mods = mods;
    true
}
fn add_map_entry(
    _ki: &XkbKeymapInfo<'_>,
    _info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    new: &XkbKeyTypeEntry,
    clobber: bool,
    report: bool,
) -> bool {
    // FindMatchingMapEntry inlined
    let mut old_idx: Option<usize> = None;
    for (i, entry) in type_0.entries.iter().enumerate() {
        if entry.mods.mods == new.mods.mods {
            old_idx = Some(i);
            break;
        }
    }
    if let Some(idx) = old_idx {
        let old = &type_0.entries[idx];
        if !report || old.level == new.level {
            return true;
        }
        if clobber {
            if new.level >= type_0.num_levels {
                type_0.num_levels = new.level.wrapping_add(1_u32);
            }
            type_0.entries[idx].level = new.level;
        }
        return true;
    }
    if new.level >= type_0.num_levels {
        type_0.num_levels = new.level.wrapping_add(1_u32);
    }
    type_0.entries.push(*new);
    true
}
fn set_map_entry(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let mut entry: XkbKeyTypeEntry = XkbKeyTypeEntry {
        level: 0,
        mods: XkbMods { mods: 0, mask: 0 },
        preserve: XkbMods { mods: 0, mask: 0 },
    };
    if array_ndx.is_none() {
        return false;
    }
    if !expr_resolve_mod_mask(
        &ki.keymap.ctx,
        array_ndx.unwrap(),
        MOD_BOTH,
        &info.mods,
        &mut entry.mods.mods,
    ) {
        return false;
    }
    if entry.mods.mods & !type_0.mods != 0 {
        entry.mods.mods &= type_0.mods;
    }
    if !expr_resolve_level(&ki.keymap.ctx, value, &mut entry.level) {
        return false;
    }
    entry.preserve.mods = 0;
    add_map_entry(ki, info, type_0, &entry, true, true)
}
fn add_preserve(
    _ki: &XkbKeymapInfo<'_>,
    _info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    mods: u32,
    preserve_mods: u32,
) -> bool {
    // Find matching entry index first to avoid borrow conflicts
    let match_idx = type_0.entries.iter().position(|e| e.mods.mods == mods);
    if let Some(idx) = match_idx {
        let old_preserve = type_0.entries[idx].preserve.mods;
        if old_preserve == 0 {
            type_0.entries[idx].preserve.mods = preserve_mods;
            return true;
        }
        if old_preserve == preserve_mods {
            return true;
        }
        type_0.entries[idx].preserve.mods = preserve_mods;
        return true;
    }
    let new = XkbKeyTypeEntry {
        level: 0_u32,
        mods: XkbMods { mods, mask: 0 },
        preserve: XkbMods {
            mods: preserve_mods,
            mask: 0,
        },
    };
    type_0.entries.push(new);
    true
}
fn set_preserve(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if array_ndx.is_none() {
        return false;
    }
    let mut mods: u32 = 0;
    if !expr_resolve_mod_mask(
        &ki.keymap.ctx,
        array_ndx.unwrap(),
        MOD_BOTH,
        &info.mods,
        &mut mods,
    ) {
        return false;
    }
    if mods & !type_0.mods != 0 {
        mods &= type_0.mods;
    }
    let mut preserve_mods: u32 = 0;
    if !expr_resolve_mod_mask(&ki.keymap.ctx, value, MOD_BOTH, &info.mods, &mut preserve_mods) {
        return false;
    }
    if preserve_mods & !mods != 0 {
        preserve_mods &= mods;
    }
    add_preserve(ki, info, type_0, mods, preserve_mods)
}
fn add_level_name(
    _ki: &XkbKeymapInfo<'_>,
    _info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    level: u32,
    name: u32,
    clobber: bool,
) -> bool {
    let level_idx = level as usize;
    if level >= type_0.level_names.len() as u32 {
        vec_resize_zero(&mut type_0.level_names, level_idx + 1);
    } else {
        if type_0.level_names[level_idx] == name {
            return true;
        }
        if type_0.level_names[level_idx] != XKB_ATOM_NONE {
            if !clobber {
                return true;
            }
        }
    }
    type_0.level_names[level_idx] = name;
    true
}
fn set_level_name(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if array_ndx.is_none() {
        return false;
    }
    let mut level: u32 = 0;
    if !expr_resolve_level(&ki.keymap.ctx, array_ndx.unwrap(), &mut level) {
        return false;
    }
    let mut level_name: u32 = XKB_ATOM_NONE;
    if !expr_resolve_string(value, &mut level_name) {
        return false;
    }
    add_level_name(ki, info, type_0, level, level_name, true)
}
fn set_key_type_field(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let ok: bool;
    let mut u32: u32 = 0;
    if field.eq_ignore_ascii_case("modifiers") {
        u32 = TYPE_FIELD_MASK;
        ok = set_modifiers(ki, info, type_0, array_ndx, value);
    } else if field.eq_ignore_ascii_case("map") {
        u32 = TYPE_FIELD_MAP;
        ok = set_map_entry(ki, info, type_0, array_ndx, value);
    } else if field.eq_ignore_ascii_case("preserve") {
        u32 = TYPE_FIELD_PRESERVE;
        ok = set_preserve(ki, info, type_0, array_ndx, value);
    } else if field.eq_ignore_ascii_case("levelname") || field.eq_ignore_ascii_case("level_name") {
        u32 = TYPE_FIELD_LEVEL_NAME;
        ok = set_level_name(ki, info, type_0, array_ndx, value);
    } else {
        ok = ki.strict & PARSER_NO_UNKNOWN_TYPE_FIELDS == 0;
    }
    type_0.defined |= u32;
    ok
}
fn handle_key_type_body(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    defs: &[VarDef],
    type_0: &mut KeyTypeInfo,
) -> bool {
    let mut ok: bool = true;
    for def in defs {
        let mut elem_atom: u32 = 0;
        let mut field_atom: u32 = 0;
        let mut array_ndx: Option<&ExprKind> = None;
        let name_ref = def.name.as_deref().unwrap();
        if !expr_resolve_lhs(name_ref, &mut elem_atom, &mut field_atom, &mut array_ndx) {
            ok = false;
        } else {
            let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom).to_owned();
            let field = atom_text(&ki.keymap.ctx.atom_table, field_atom).to_owned();
            if !elem.is_empty() {
                if !elem.eq_ignore_ascii_case("type") {
                    ok = false;
                }
            } else {
                let value_ref = def.value.as_deref().unwrap();
                if !set_key_type_field(ki, info, type_0, &field, array_ndx, value_ref) {
                    ok = false;
                }
            }
        }
    }
    ok
}
fn handle_type_global_var(ki: &XkbKeymapInfo<'_>, _info: &mut KeyTypesInfo, stmt: &VarDef) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut array_ndx: Option<&ExprKind> = None;
    let name_ref = stmt.name.as_deref().unwrap();
    if !expr_resolve_lhs(name_ref, &mut elem_atom, &mut field_atom, &mut array_ndx) {
        return false;
    }
    let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom);
    let field = atom_text(&ki.keymap.ctx.atom_table, field_atom);
    if !elem.is_empty() && elem.eq_ignore_ascii_case("type") {
        return true;
    } else if !elem.is_empty() {
        return ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
    } else if !field.is_empty() {
        return ki.strict & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS == 0;
    }
    false
}
fn handle_key_types_file(ki: &mut XkbKeymapInfo<'_>, info: &mut KeyTypesInfo, file: &mut XkbFile) {
    {
        let mut ok: bool;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_key_types(ki, info, incl);
                }
                Statement::KeyType(def) => {
                    let mut type_0: KeyTypeInfo = KeyTypeInfo {
                        defined: 0_u32,
                        merge: def.merge,
                        name: def.name,
                        mods: 0_u32,
                        num_levels: 1_u32,
                        entries: Vec::new(),
                        level_names: Vec::new(),
                    };
                    if !handle_key_type_body(ki, info, &def.body, &mut type_0)
                        || !add_key_type(info, &mut type_0)
                    {
                        info.error_count += 1;
                        ok = false;
                    } else {
                        ok = true;
                    }
                }
                Statement::Var(var) => {
                    ok = handle_type_global_var(ki, info, var);
                }
                Statement::VMod(vmod) => {
                    ok = handle_vmod_def(&mut ki.keymap.ctx, &mut info.mods, vmod);
                }
                Statement::Unknown(_unk) => {
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    ok = false;
                }
            }
            if !ok {
                info.error_count += 1;
            }
            if info.error_count > 10 {
                break;
            }
        }
    }
}
fn copy_key_types_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut KeyTypesInfo) -> bool {
    // let keymap = ki.keymap;
    let num_types: u32 = if info.types.is_empty() {
        1_u32
    } else {
        info.types.len() as u32
    };
    let mut types_vec: Vec<XkbKeyType> = Vec::with_capacity(num_types as usize);
    if info.types.is_empty() {
        let type_0 = XkbKeyType {
            name: atom_intern(&mut ki.keymap.ctx.atom_table, b"ONE_LEVEL"),
            mods: XkbMods { mods: 0, mask: 0 },
            required: true,
            num_levels: 1,
            entries: Vec::new(),
        };
        types_vec.push(type_0);
    } else {
        let canonical_types: [u32; 4] = [
            atom_intern(&mut ki.keymap.ctx.atom_table, b"ONE_LEVEL"),
            atom_intern(&mut ki.keymap.ctx.atom_table, b"TWO_LEVEL"),
            atom_intern(&mut ki.keymap.ctx.atom_table, b"ALPHABETIC"),
            atom_intern(&mut ki.keymap.ctx.atom_table, b"KEYPAD"),
        ];
        for def in info.types.iter_mut() {
            let entries = std::mem::take(&mut def.entries);
            let mut required = false;
            if def.num_levels <= 2 {
                for t in 0..4 {
                    if def.name == canonical_types[t] {
                        required = true;
                        break;
                    }
                }
            }
            types_vec.push(XkbKeyType {
                name: def.name,
                mods: XkbMods {
                    mods: def.mods,
                    mask: 0,
                },
                required,
                num_levels: def.num_levels,
                entries,
            });
        }
    }
    ki.keymap.types_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut ki.keymap.types_section_name);
    ki.keymap.types = types_vec;
    ki.keymap.mods = info.mods;
    true
}
pub(crate) fn compile_key_types(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut info = KeyTypesInfo::new();
    init_key_types_info(&mut info, 0_u32, &mods);
    if let Some(file) = file {
        handle_key_types_file(keymap_info, &mut info, file);
    }
    if (info.error_count == 0) && copy_key_types_to_keymap(keymap_info, &mut info) {
        return true;
    }
    false
}

// ── Virtual modifier functions (migrated from vmod.rs) ──

pub(crate) fn init_vmods(info: &mut XkbModSet, mods: &XkbModSet, reset: bool) {
    *info = *mods;
    if !reset {
        return;
    }
    for vmod in 0..info.num_mods as usize {
        info.mods[vmod].mapping = 0;
    }
    info.explicit_vmods = 0;
}
pub(crate) fn merge_mod_sets(
    _ctx: &mut XkbContext,
    into: &mut XkbModSet,
    from: &XkbModSet,
    merge: u32,
) {
    let clobber: bool = merge != MERGE_AUGMENT;
    for vmod in 0..from.num_mods as usize {
        let mod_0 = &from.mods[vmod];
        let mask: u32 = 1_u32 << vmod;
        if mod_0.type_0 != MOD_VIRT {
        } else if into.mods[vmod].type_0 == 0 {
            into.mods[vmod] = *mod_0;
            if from.explicit_vmods & mask != 0 {
                into.explicit_vmods |= mask;
            }
        } else if from.explicit_vmods & mask == 0 {
        } else if into.explicit_vmods & mask == 0 {
            into.mods[vmod].mapping = mod_0.mapping;
            into.explicit_vmods |= mask;
        } else if mod_0.mapping != into.mods[vmod].mapping {
            let use_0: u32 = if clobber {
                mod_0.mapping
            } else {
                into.mods[vmod].mapping
            };
            let _ignore: u32 = if clobber {
                into.mods[vmod].mapping
            } else {
                mod_0.mapping
            };
            into.mods[vmod].mapping = use_0;
        }
    }
    into.num_mods = from.num_mods;
}
pub(crate) fn handle_vmod_def(ctx: &mut XkbContext, mods: &mut XkbModSet, stmt: &VModDef) -> bool {
    let mut mapping: u32 = 0;
    if stmt.value.is_some() {
        let value_ref = stmt.value.as_deref().unwrap();
        if !expr_resolve_mod_mask(ctx, value_ref, MOD_REAL, mods, &mut mapping) {
            return false;
        }
    }
    for vmod in 0..mods.num_mods as usize {
        if mods.mods[vmod].name == stmt.name {
            if mods.mods[vmod].type_0 != MOD_VIRT {
                return false;
            }
            let mask: u32 = 1_u32 << vmod;
            if stmt.value.is_none() {
                return true;
            } else if mods.explicit_vmods & mask == 0 {
                mods.mods[vmod].mapping = mapping;
            } else if mods.mods[vmod].mapping != mapping {
                let clobber: bool = stmt.merge != MERGE_AUGMENT;
                let use_0: u32 = if clobber {
                    mapping
                } else {
                    mods.mods[vmod].mapping
                };
                let _ignore: u32 = if clobber {
                    mods.mods[vmod].mapping
                } else {
                    mapping
                };
                mods.mods[vmod].mapping = use_0;
            }
            mods.explicit_vmods |= mask;
            return true;
        }
    }
    if mods.num_mods >= XKB_MAX_MODS {
        return false;
    }
    mods.mods[mods.num_mods as usize].name = stmt.name;
    mods.mods[mods.num_mods as usize].type_0 = MOD_VIRT;
    mods.mods[mods.num_mods as usize].mapping = mapping;
    if stmt.value.is_some() {
        let mask_0: u32 = 1_u32 << mods.num_mods;
        mods.explicit_vmods |= mask_0;
    }
    mods.num_mods = mods.num_mods + 1;
    true
}
pub(crate) struct KeyNamesInfo {
    pub(crate) name: Option<String>,
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) keycodes: KeycodeStore,
    pub(crate) led_names: [LedNameInfo; 32],
    pub(crate) num_led_names: u32,
}
impl Default for KeyNamesInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyNamesInfo {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            error_count: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: XKB_KEYCODE_INVALID,
                low: Vec::new(),
                high: Vec::new(),
                names: Vec::new(),
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
        }
    }
}
#[derive(Copy, Clone)]
pub(crate) struct LedNameInfo {
    pub(crate) merge: u32,
    pub(crate) name: u32,
}
#[derive(Clone)]
pub(crate) struct KeycodeStore {
    pub(crate) min: u32,
    pub(crate) low: Vec<u32>,
    pub(crate) high: Vec<HighKeycodeEntry>,
    pub(crate) names: Vec<KeycodeMatch>,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct HighKeycodeEntry {
    pub(crate) keycode: u32,
    pub(crate) name: u32,
}
fn vec_resize_zero<T: Default>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.resize_with(new_len, Default::default);
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

#[inline]
fn keycode_store_update_key(store: &mut KeycodeStore, match_0: KeycodeMatch, name: u32) {
    if !match_0.found || match_0.is_alias {
        return;
    } else if match_0.low {
        store.low[match_0.index as usize] = name;
    } else {
        store.high[match_0.index as usize].name = name;
    }
    if name >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (name as usize) + 1);
    }
    store.names[name as usize] = match_0;
}
fn keycode_store_insert_key(store: &mut KeycodeStore, kc: u32, name: u32) -> bool {
    if name >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (name as usize) + 1);
    }
    if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as u32 {
        if kc >= store.low.len() as u32 {
            vec_resize_zero(&mut store.low, (kc as usize) + 1);
        }
        store.low[kc as usize] = name;
        if kc < store.min {
            store.min = kc;
        }
        store.names[name as usize] = KeycodeMatch {
            found: true,
            low: true,
            is_alias: false,
            index: kc,
        };
    } else {
        let idx: u32 = store.high.len() as u32;
        if idx != 0 && store.high[(idx.wrapping_sub(1_u32)) as usize].keycode > kc {
            let lower =
                match store.high[..idx as usize].binary_search_by(|entry| entry.keycode.cmp(&kc)) {
                    Ok(i) | Err(i) => i as u32,
                };
            for i in lower as usize..store.high.len() {
                let name_idx = store.high[i].name;
                store.names[name_idx as usize].index += 1;
            }
            store
                .high
                .insert(lower as usize, HighKeycodeEntry { keycode: kc, name });
            store.names[name as usize] = KeycodeMatch {
                found: true,
                low: false,
                is_alias: false,
                index: lower,
            };
        } else {
            store.high.push(HighKeycodeEntry { keycode: kc, name });
            store.names[name as usize] = KeycodeMatch {
                found: true,
                low: false,
                is_alias: false,
                index: idx,
            };
        }
        if store.low.is_empty() {
            store.min = store.high[0].keycode;
        }
    }
    true
}
#[inline]
fn keycode_store_insert_alias(store: &mut KeycodeStore, alias: u32, real: u32) -> bool {
    if alias >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (alias as usize) + 1);
    }
    store.names[alias as usize] = KeycodeMatch {
        found: true,
        low: true,
        is_alias: real != 0,
        index: real,
    };
    true
}
#[inline]
fn keycode_store_delete_name(store: &mut KeycodeStore, name: u32) {
    if (name as usize) < store.names.len() {
        store.names[name as usize].found = false;
    }
}
fn keycode_store_delete_key(store: &mut KeycodeStore, match_0: KeycodeMatch) {
    if !match_0.found || match_0.is_alias {
        return;
    } else if match_0.low {
        let low_name = store.low[match_0.index as usize];
        store.names[low_name as usize].found = false;
        if match_0.index.wrapping_add(1_u32) == store.low.len() as u32 {
            if store.min == match_0.index {
                store.low.clear();
            } else {
                let mut idx: u32 = match_0.index;
                while idx > 0 {
                    if store.low[(idx.wrapping_sub(1_u32)) as usize] != XKB_ATOM_NONE {
                        store.low.truncate(idx as usize);
                        break;
                    } else {
                        idx -= 1;
                    }
                }
            }
        } else {
            store.low[match_0.index as usize] = XKB_ATOM_NONE;
        }
    } else {
        let high_name = store.high[match_0.index as usize].name;
        store.names[high_name as usize].found = false;
        store.high.remove(match_0.index as usize);
        for entry in store.names.iter_mut() {
            if entry.found && !entry.is_alias && !entry.low && entry.index > match_0.index {
                entry.index -= 1;
            }
        }
    }
    if store.low.is_empty() {
        store.min = if store.high.is_empty() {
            XKB_KEYCODE_INVALID
        } else {
            store.high[0].keycode
        };
    } else {
        let mut kc: u32 = store.min;
        while kc < store.low.len() as u32 {
            if store.low[kc as usize] != XKB_ATOM_NONE {
                store.min = kc;
                break;
            } else {
                kc += 1;
            }
        }
    }
}
fn keycode_store_lookup_keycode(store: &KeycodeStore, kc: u32) -> KeycodeMatch {
    if kc < store.low.len() as u32 {
        return KeycodeMatch {
            found: true,
            low: true,
            is_alias: false,
            index: kc,
        };
    } else if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as u32 {
        return KeycodeMatch {
            found: false,
            low: false,
            is_alias: false,
            index: 0,
        };
    }
    match store.high.binary_search_by(|entry| entry.keycode.cmp(&kc)) {
        Ok(mid) => KeycodeMatch {
            found: true,
            low: false,
            is_alias: false,
            index: mid as u32,
        },
        Err(_) => KeycodeMatch {
            found: false,
            low: false,
            is_alias: false,
            index: 0,
        },
    }
}
fn keycode_store_lookup_name(store: &KeycodeStore, name: u32) -> KeycodeMatch {
    if name >= store.names.len() as u32 {
        KeycodeMatch {
            found: false,
            low: false,
            is_alias: false,
            index: 0,
        }
    } else {
        store.names[name as usize]
    }
}
fn add_led_name(
    info: &mut KeyNamesInfo,
    new: &LedNameInfo,
    new_idx: u32,
) -> bool {
    let replace: bool = new.merge != MERGE_AUGMENT;
    // FindLedByName inlined
    let found_old = info.led_names[..info.num_led_names as usize]
        .iter()
        .position(|l| l.name == new.name)
        .map(|i| i as u32);
    if let Some(old_idx) = found_old {
        if old_idx == new_idx {
            return true;
        }
        if replace {
            info.led_names[old_idx as usize].name = XKB_ATOM_NONE;
        } else {
            return true;
        }
    }
    if new_idx >= info.num_led_names {
        info.num_led_names = new_idx.wrapping_add(1_u32);
    }
    if info.led_names[new_idx as usize].name != XKB_ATOM_NONE {
        if replace {
            info.led_names[new_idx as usize] = *new;
        }
        return true;
    }
    info.led_names[new_idx as usize] = *new;
    true
}
fn init_key_names_info(info: &mut KeyNamesInfo, include_depth: u32) {
    info.name = None;
    info.error_count = 0;
    info.include_depth = include_depth;
    info.keycodes = KeycodeStore {
        min: XKB_KEYCODE_INVALID,
        low: Vec::new(),
        high: Vec::new(),
        names: Vec::new(),
    };
    info.led_names = [LedNameInfo {
        merge: MERGE_DEFAULT,
        name: 0,
    }; 32];
    info.num_led_names = 0;
}
fn add_key_name(
    info: &mut KeyNamesInfo,
    kc: u32,
    name: u32,
    merge: u32,
) -> bool {
    let match_name: KeycodeMatch = keycode_store_lookup_name(&info.keycodes, name);
    if match_name.found {
        let clobber: bool = merge != MERGE_AUGMENT;
        if match_name.is_alias {
            if clobber {
                keycode_store_delete_name(&mut info.keycodes, name);
                // dead store removed: match_name.found = false;
            } else {
                return true;
            }
        } else {
            let old_kc: u32 = {
                if !match_name.found || match_name.is_alias {
                    XKB_KEYCODE_INVALID
                } else if match_name.low {
                    match_name.index
                } else {
                    info.keycodes.high[match_name.index as usize].keycode
                }
            };
            if old_kc != kc {
                if clobber {
                    keycode_store_delete_key(&mut info.keycodes, match_name);
                } else {
                    return true;
                }
            }
        }
    }
    let match_kc: KeycodeMatch = keycode_store_lookup_keycode(&info.keycodes, kc);
    let old_name: u32 = {
        if !match_kc.found || match_kc.is_alias {
            XKB_ATOM_NONE
        } else if match_kc.low {
            info.keycodes.low[match_kc.index as usize]
        } else {
            info.keycodes.high[match_kc.index as usize].name
        }
    };
    if old_name != XKB_ATOM_NONE {
        if old_name == name {
            return true;
        }
        let clobber_0: bool = merge != MERGE_AUGMENT;
        if clobber_0 {
            keycode_store_delete_name(&mut info.keycodes, old_name);
            keycode_store_update_key(&mut info.keycodes, match_kc, name);
        }
    } else if !keycode_store_insert_key(&mut info.keycodes, kc, name) {
        return false;
    }
    true
}
fn merge_keycode_stores(
    into: &mut KeyNamesInfo,
    from: &mut KeyNamesInfo,
    merge: u32,
) {
    if into.keycodes.low.is_empty()
        && into.keycodes.high.is_empty()
        && into.keycodes.names.is_empty()
    {
        into.keycodes = std::mem::replace(
            &mut from.keycodes,
            KeycodeStore {
                min: XKB_KEYCODE_INVALID,
                low: Vec::new(),
                high: Vec::new(),
                names: Vec::new(),
            },
        );
    } else {
        let mut kc: u32 = from.keycodes.min;
        while kc < from.keycodes.low.len() as u32 {
            let name: u32 = (&from.keycodes.low)[kc as usize];
            if (name != XKB_ATOM_NONE) && !add_key_name(into, kc, name, merge) {
                into.error_count += 1;
            }
            kc += 1;
        }
        for entry in from.keycodes.high.iter() {
            if !add_key_name(into, entry.keycode, entry.name, merge) {
                into.error_count += 1;
            }
        }
        {
            let names_len = from.keycodes.names.len();
            if names_len > 0 {
                for alias in 0..names_len as u32 {
                    let match_0 = from.keycodes.names[alias as usize];
                    if match_0.found && match_0.is_alias {
                        let def: KeyAliasDef = KeyAliasDef {
                            merge,
                            alias,
                            real: match_0.index,
                        };
                        if !handle_alias_def(into, &def) {
                            into.error_count += 1;
                        }
                    }
                }
            }
        }
    };
}
fn merge_included_keycodes(
    into: &mut KeyNamesInfo,
    from: &mut KeyNamesInfo,
    merge: u32,
) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    if into.name.is_none() {
        into.name = from.name.take();
    }
    merge_keycode_stores(into, from, merge);
    if into.num_led_names == 0 {
        into.led_names[..from.num_led_names as usize]
            .copy_from_slice(&from.led_names[..from.num_led_names as usize]);
        into.num_led_names = from.num_led_names;
        from.num_led_names = 0;
    } else {
        let mut idx: u32 = 0;
        while idx < from.num_led_names {
            let ledi = from.led_names[idx as usize];
            if ledi.name != XKB_ATOM_NONE {
                let mut ledi = ledi;
                ledi.merge = merge;
                if !add_led_name(into, &ledi, idx) {
                    into.error_count += 1;
                }
            }
            idx += 1;
        }
    };
}
fn handle_include_keycodes(
    info: &mut KeyNamesInfo,
    include: &mut IncludeStmt,
    ki: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mut included = KeyNamesInfo::new();
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    init_key_names_info(&mut included, 0_u32);
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(std::mem::take(&mut include.stmt))
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = KeyNamesInfo::new();

        let file: Option<Box<XkbFile>> =
            process_include_file(&mut ki.keymap.ctx, stmt, FILE_TYPE_KEYCODES);
        let Some(mut file) = file else {
            info.error_count += 10;
            return false;
        };
        init_key_names_info(&mut next_incl, info.include_depth.wrapping_add(1));
        handle_keycodes_file(&mut next_incl, &mut file, ki);
        merge_included_keycodes(&mut included, &mut next_incl, stmt.merge);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    merge_included_keycodes(info, &mut included, include.merge);
    info.error_count == 0
}
fn handle_keycode_def(
    info: &mut KeyNamesInfo,
    stmt: &KeycodeDef,
) -> bool {
    if stmt.value < 0_i64 || stmt.value > XKB_KEYCODE_MAX as i64 {
        return false;
    }
    add_key_name(info, stmt.value as u32, stmt.name, stmt.merge)
}
fn handle_alias_def(
    info: &mut KeyNamesInfo,
    def: &KeyAliasDef,
) -> bool {
    let match_name: KeycodeMatch =
        keycode_store_lookup_name(&info.keycodes, def.alias) as KeycodeMatch;
    if match_name.found {
        let clobber: bool = def.merge != MERGE_AUGMENT;
        if match_name.is_alias {
            if def.real == match_name.index {
            } else {
                let use_0: u32 = if clobber { def.real } else { match_name.index };
                let _ignore: u32 = if clobber { match_name.index } else { def.real };

                {
                    info.keycodes.names[def.alias as usize].index = use_0;
                }
            }
            return true;
        } else if clobber {
            keycode_store_delete_key(&mut info.keycodes, match_name);
        } else {
            return true;
        }
    }
    keycode_store_insert_alias(&mut info.keycodes, def.alias, def.real)
}
fn handle_key_name_var(
    ki: &mut XkbKeymapInfo<'_>,
    stmt: &VarDef,
) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut array_ndx: Option<&ExprKind> = None;
    let name_ref = stmt.name.as_deref().unwrap();
    if !expr_resolve_lhs(name_ref, &mut elem_atom, &mut field_atom, &mut array_ndx) {
        return false;
    }
    let elem = atom_text(&ki.keymap.ctx.atom_table, elem_atom);
    let field = atom_text(&ki.keymap.ctx.atom_table, field_atom);
    if !elem.is_empty() {
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if !field.eq_ignore_ascii_case("minimum") && !field.eq_ignore_ascii_case("maximum") {
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if array_ndx.is_some() {
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    let mut val: i64 = 0_i64;
    let value_ref = stmt.value.as_deref().unwrap();
    if !expr_resolve_integer(&ki.keymap.ctx, value_ref, &mut val) || val < 0_i64 || val > u32::MAX as i64
    {
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    true
}
fn handle_led_name_def(
    info: &mut KeyNamesInfo,
    def: &LedNameDef,
) -> bool {
    if def.ndx < 1_i64 || def.ndx > XKB_MAX_LEDS as i64 {
        info.error_count += 1;
        return false;
    }
    let mut name: u32 = XKB_ATOM_NONE;
    let name_expr = def.name.as_deref().unwrap();
    if !expr_resolve_string(name_expr, &mut name) {
        info.error_count += 1;
        return false;
    }
    let ledi: LedNameInfo = LedNameInfo {
        merge: def.merge,
        name,
    };
    add_led_name(
        info,
        &ledi,
        (def.ndx as u32).wrapping_sub(1_u32),
    )
}
fn handle_keycodes_file(info: &mut KeyNamesInfo, file: &mut XkbFile, ki: &mut XkbKeymapInfo<'_>) {
    {
        let mut ok: bool;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_keycodes(info, incl, ki);
                }
                Statement::Keycode(kc) => {
                    ok = handle_keycode_def(info, kc);
                }
                Statement::KeyAlias(ka) => {
                    ok = handle_alias_def(info, ka);
                }
                Statement::Var(var) => {
                    ok = handle_key_name_var(ki, var);
                }
                Statement::LedName(ln) => {
                    ok = handle_led_name_def(info, ln);
                }
                Statement::Unknown(_unk) => {
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    ok = false;
                }
            }
            if !ok {
                info.error_count += 1;
            }
            if info.error_count > 10 {
                break;
            }
        }
    }
}
fn copy_key_names_to_keymap(keymap: &mut XkbKeymap, keycodes: &KeycodeStore) -> bool {
    if keycodes.low.is_empty() && keycodes.high.is_empty() {
        keymap.min_key_code = 8;
        keymap.max_key_code = 255;
        keymap.num_keys_low = keymap.max_key_code.wrapping_add(1_u32);
        keymap.num_keys = keymap.num_keys_low;
    } else {
        keymap.min_key_code = keycodes.min;
        keymap.max_key_code = if keycodes.high.is_empty() {
            (keycodes.low.len() as u32).wrapping_sub(1_u32)
        } else {
            (&keycodes.high)[keycodes.high.len() - 1].keycode
        };
        keymap.num_keys_low = keycodes.low.len() as u32;
        keymap.num_keys = keymap.num_keys_low.wrapping_add(keycodes.high.len() as u32);
    }
    let mut keys: Vec<XkbKey> = (0..keymap.num_keys as usize)
        .map(|_| XkbKey::default())
        .collect();
    let mut kc: u32 = keymap.min_key_code;
    while kc < keymap.num_keys_low {
        keys[kc as usize].keycode = kc;
        kc += 1;
    }
    let mut kc_0: u32 = keycodes.min;
    while kc_0 < keycodes.low.len() as u32 {
        keys[kc_0 as usize].name = (&keycodes.low)[kc_0 as usize];
        kc_0 += 1;
    }
    let mut idx: u32 = keymap.num_keys_low;
    for entry in keycodes.high.iter() {
        keys[idx as usize].keycode = entry.keycode;
        keys[idx as usize].name = entry.name;
        idx += 1;
    }
    keymap.keys = keys;
    true
}
fn copy_keycode_name_lut(keymap: &mut XkbKeymap, keycodes: &mut KeycodeStore) -> bool {
    let names_len = keycodes.names.len();
    {
        if names_len > 0 {
            for name in 0..names_len as u32 {
                let entry = keycodes.names[name as usize];
                if entry.found {
                    if entry.is_alias {
                        let match_real: KeycodeMatch =
                            keycode_store_lookup_name(keycodes, entry.index);
                        if !match_real.found {
                            keycodes.names[name as usize].found = false;
                        } else if match_real.is_alias {
                            keycodes.names[name as usize].found = false;
                        }
                    } else if !entry.low {
                        keycodes.names[name as usize].index += keymap.num_keys_low;
                    }
                }
            }
        }
    }
    if names_len > 0 {
        keymap.key_names = std::mem::take(&mut keycodes.names);
    } else {
        keymap.key_names = Vec::new();
    }
    true
}
fn copy_led_names_to_keymap(
    keymap: &mut XkbKeymap,
    led_names: &[LedNameInfo; 32],
    num_led_names: u32,
) -> bool {
    keymap.num_leds = num_led_names;
    let mut idx: u32 = 0;
    while idx < num_led_names {
        let ledi = &led_names[idx as usize];
        if ledi.name != XKB_ATOM_NONE {
            keymap.leds[idx as usize].name = ledi.name;
        }
        idx += 1;
    }
    true
}
fn copy_key_names_info_to_keymap(info: &mut KeyNamesInfo, ki: &mut XkbKeymapInfo<'_>) -> bool {
    if !copy_key_names_to_keymap(ki.keymap, &info.keycodes)
        || !copy_keycode_name_lut(ki.keymap, &mut info.keycodes)
        || !copy_led_names_to_keymap(ki.keymap, &info.led_names, info.num_led_names)
    {
        return false;
    }
    if ki.keymap.num_keys == 0 || ki.keymap.min_key_code > 0 {
        ki.keymap.redirect_key_auto = 0;
    } else {
        let mut keycode: u32 = XKB_KEYCODE_INVALID.wrapping_sub(1_u32);
        let mut k: u32 = ki.keymap.num_keys;
        loop {
            let old_k = k;
            k -= 1;
            if old_k <= ki.keymap.num_keys_low {
                break;
            }
            if keycode > (&ki.keymap.keys)[k as usize].keycode {
                break;
            }
            keycode = (&ki.keymap.keys)[k as usize].keycode.wrapping_sub(1_u32);
        }
        ki.keymap.redirect_key_auto = keycode;
    }
    ki.keymap.keycodes_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut ki.keymap.keycodes_section_name);
    true
}
pub(crate) fn compile_keycodes(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mut info = KeyNamesInfo::new();
    init_key_names_info(&mut info, 0_u32);
    if let Some(file) = file {
        handle_keycodes_file(&mut info, file, keymap_info);
    }
    if (info.error_count == 0) && copy_key_names_info_to_keymap(&mut info, keymap_info) {
        return true;
    }
    false
}
use super::super::keymap::{ACTION_TYPE_NAMES, BUTTON_NAMES, GROUP_LAST_INDEX_NAME};

pub(crate) use super::super::keymap::action_equal;

use super::super::shared_types::ExprKind;

pub(crate) struct LookupModMaskPriv<'a> {
    pub(crate) mods: &'a XkbModSet,
    pub(crate) mod_type: u32,
}

/// Safe replacement for the IdentLookupFunc + *const c_void pair.
pub(crate) enum IdentLookup<'a> {
    None,
    Simple(&'a [LookupEntry]),
    NamedPattern(&'a NamedIntegerPattern<'a>),
    ModMask(&'a LookupModMaskPriv<'a>),
}

pub(crate) struct NamedIntegerPattern<'a> {
    pub(crate) prefix: &'static str,
    pub(crate) min: u32,
    pub(crate) max: u32,
    pub(crate) entries: &'a [LookupEntry],
    pub(crate) pending_entries: &'a [LookupEntry],
    pub(crate) is_mask: bool,
}

static LEVEL_NAME_PATTERN_ENTRIES: [LookupEntry; 1] = [LookupEntry { name: "", value: 0 }];

fn simple_lookup(ctx: &XkbContext, entries: &[LookupEntry], field: u32) -> Option<u32> {
    if field == XKB_ATOM_NONE {
        return None;
    }
    let s: &str = atom_text(&ctx.atom_table, field);
    for entry in entries {
        if entry.name.is_empty() {
            break;
        }
        if s.eq_ignore_ascii_case(entry.name) {
            return Some(entry.value);
        }
    }
    None
}

fn named_integer_pattern_lookup(
    ctx: &XkbContext,
    pattern: &NamedIntegerPattern,
    field: u32,
    pending_rtrn: Option<&mut bool>,
) -> Option<u32> {
    if field == XKB_ATOM_NONE {
        return None;
    }
    let str_bytes: &str = atom_text(&ctx.atom_table, field);
    let prefix = pattern.prefix;
    let count: i32 = if str_bytes
        .as_bytes()
        .get(..prefix.len())
        .is_some_and(|s| s.eq_ignore_ascii_case(prefix.as_bytes()))
    {
        let suffix = &str_bytes.as_bytes()[prefix.len()..];
        let (val_parsed, c) = super::super::shared_types::parse_dec_u32(suffix);
        // Return parsed value via count mechanism
        let _ = val_parsed;
        c
    } else {
        0_i32
    };

    if count > 0_i32 && prefix.len() + count as usize == str_bytes.len() {
        // Re-parse to get the value
        let suffix = &str_bytes.as_bytes()[prefix.len()..];
        let (val, _) = super::super::shared_types::parse_dec_u32(suffix);
        if val < pattern.min || val > pattern.max {
            return None;
        }
        let result = if pattern.is_mask {
            1_u32 << val.wrapping_sub(pattern.min)
        } else {
            val
        };
        Some(result)
    } else {
        if let Some(val) = simple_lookup(ctx, pattern.entries, field) {
            return Some(val);
        }
        if let Some(pending) = pending_rtrn {
            if let Some(val) = simple_lookup(ctx, pattern.pending_entries, field) {
                *pending = true;
                return Some(val);
            }
        }
        None
    }
}

fn lookup_mod_mask(ctx: &XkbContext, priv_0: &LookupModMaskPriv, field: u32) -> Option<u32> {
    let s: &str = atom_text(&ctx.atom_table, field);
    if s.is_empty() {
        return None;
    }
    if s.eq_ignore_ascii_case("all") {
        return Some(MOD_REAL_MASK_ALL);
    }
    if s.eq_ignore_ascii_case("none") {
        return Some(0_u32);
    }
    let ndx: u32 = xkb_mod_name_to_index(priv_0.mods, field, priv_0.mod_type);
    if ndx == XKB_MOD_INVALID {
        return None;
    }
    Some(1_u32 << ndx)
}

/// Dispatch a lookup based on the IdentLookup variant.
/// Returns Some(value) on success. Sets `pending` to true if applicable.
fn ident_lookup(
    ctx: &XkbContext,
    lookup: &IdentLookup,
    field: u32,
    pending: Option<&mut bool>,
) -> Option<u32> {
    match lookup {
        IdentLookup::None => None,
        IdentLookup::Simple(entries) => simple_lookup(ctx, entries, field),
        IdentLookup::NamedPattern(pattern) => {
            named_integer_pattern_lookup(ctx, pattern, field, pending)
        }
        IdentLookup::ModMask(priv_0) => lookup_mod_mask(ctx, priv_0, field),
    }
}

pub(crate) fn expr_resolve_lhs<'a>(
    expr: &'a ExprKind,
    elem_rtrn: &mut u32,
    field_rtrn: &mut u32,
    index_rtrn: &mut Option<&'a ExprKind>,
) -> bool {
    match expr.stmt_type() {
        10 => {
            let ExprKind::Ident(ident) = &expr else {
                unreachable!()
            };
            *elem_rtrn = XKB_ATOM_NONE;
            *field_rtrn = *ident;
            *index_rtrn = None;
            return *field_rtrn != XKB_ATOM_NONE;
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr else {
                unreachable!()
            };
            *elem_rtrn = *element;
            *field_rtrn = *field;
            *index_rtrn = None;
            return *elem_rtrn != XKB_ATOM_NONE && *field_rtrn != XKB_ATOM_NONE;
        }
        13 => {
            let ExprKind::ArrayRef {
                element,
                field,
                entry,
            } = &expr
            else {
                unreachable!()
            };
            *elem_rtrn = *element;
            *field_rtrn = *field;
            *index_rtrn = entry.as_ref().map(|b| &**b);
            if *element != XKB_ATOM_NONE && *elem_rtrn == XKB_ATOM_NONE {
                return false;
            }
            if *field_rtrn == XKB_ATOM_NONE {
                return false;
            }
            return true;
        }
        _ => {}
    }
    false
}

pub(crate) fn expr_resolve_boolean(ctx: &XkbContext, expr: &ExprKind, set_rtrn: &mut bool) -> bool {
    let ok: bool;
    match expr.stmt_type() {
        7 => {
            let ExprKind::Boolean(set) = &expr else {
                unreachable!()
            };
            *set_rtrn = *set;
            return true;
        }
        4 | 5 | 6 | 8 | 9 => {
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr else {
                unreachable!()
            };
            let ident = atom_text(&ctx.atom_table, *ident_atom);
            if !ident.is_empty() {
                if ident.eq_ignore_ascii_case("true")
                    || ident.eq_ignore_ascii_case("yes")
                    || ident.eq_ignore_ascii_case("on")
                {
                    *set_rtrn = true;
                    return true;
                } else if ident.eq_ignore_ascii_case("false") {
                    *set_rtrn = false;
                    return true;
                } else if ident.eq_ignore_ascii_case("no") || ident.eq_ignore_ascii_case("off") {
                    *set_rtrn = false;
                    return true;
                }
            }
            return false;
        }
        12 => {
            let ExprKind::FieldRef { .. } = &expr else {
                unreachable!()
            };
            return false;
        }
        24 | 22 => {
            let ExprKind::Unary { child, .. } = &expr else {
                unreachable!()
            };
            let child_ref = child.as_deref().unwrap();
            ok = expr_resolve_boolean(ctx, child_ref, set_rtrn);
            if ok {
                *set_rtrn = !*set_rtrn;
            }
            return ok;
        }
        17 | 18 | 19 | 20 | 21 | 23 | 25 | 14 | 11 | 16 | 15 => {}
        _ => {}
    }
    false
}

fn expr_resolve_integer_lookup(
    ctx: &XkbContext,
    expr: &ExprKind,
    val_rtrn: &mut i64,
    pending: Option<&mut bool>,
    lookup: &IdentLookup,
) -> bool {
    let mut ok: bool = false;
    let mut l: i64 = 0_i64;
    let mut r: i64 = 0_i64;
    match expr.stmt_type() {
        5 => {
            let ExprKind::Integer(ival) = &expr else {
                unreachable!()
            };
            *val_rtrn = *ival;
            return true;
        }
        4 | 6 | 7 | 8 | 9 => {
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr else {
                unreachable!()
            };
            let mut pending_local = false;
            let pending_ref = if pending.is_some() {
                Some(&mut pending_local)
            } else {
                None
            };
            if let Some(u) = ident_lookup(ctx, lookup, *ident_atom, pending_ref) {
                *val_rtrn = u as i64;
                ok = true;
            }

            if let Some(p) = pending {
                *p = pending_local;
                if pending_local {
                    return false;
                }
            }
            return ok;
        }
        12 => {
            let ExprKind::FieldRef { .. } = &expr else {
                unreachable!()
            };
            return false;
        }
        17..=20 => {
            let ExprKind::Binary {
                left: bleft,
                right: bright,
                ..
            } = &expr
            else {
                unreachable!()
            };
            let left = bleft.as_deref().unwrap();
            let right = bright.as_deref().unwrap();
            if !expr_resolve_integer_lookup(ctx, left, &mut l, None, lookup)
                || !expr_resolve_integer_lookup(ctx, right, &mut r, None, lookup)
            {
                return false;
            }
            match expr.stmt_type() {
                17 => {
                    let (v, overflow) = l.overflowing_add(r);
                    *val_rtrn = v;
                    if overflow {
                        return false;
                    }
                }
                18 => {
                    let (v, overflow) = l.overflowing_sub(r);
                    *val_rtrn = v;
                    if overflow {
                        return false;
                    }
                }
                19 => {
                    let (v, overflow) = l.overflowing_mul(r);
                    *val_rtrn = v;
                    if overflow {
                        return false;
                    }
                }
                20 => {
                    if r == 0_i64 {
                        return false;
                    }
                    *val_rtrn = l / r;
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        21 => {}
        22 => {
            return false;
        }
        24 | 23 => {
            let ExprKind::Unary { child, .. } = &expr else {
                unreachable!()
            };
            let left = child.as_deref().unwrap();
            if !expr_resolve_integer_lookup(ctx, left, &mut l, None, lookup) {
                return false;
            }
            *val_rtrn = if expr.stmt_type() == STMT_EXPR_NEGATE {
                -l
            } else {
                !l
            };
            return true;
        }
        25 => {
            let ExprKind::Unary { child, .. } = &expr else {
                unreachable!()
            };
            let left = child.as_deref().unwrap();
            return expr_resolve_integer_lookup(ctx, left, val_rtrn, None, lookup);
        }
        _ => {}
    }
    false
}

pub(crate) fn expr_resolve_integer(ctx: &XkbContext, expr: &ExprKind, val_rtrn: &mut i64) -> bool {
    expr_resolve_integer_lookup(ctx, expr, val_rtrn, None, &IdentLookup::None)
}

pub(crate) fn expr_resolve_group(
    keymap_info: &mut XkbKeymapInfo<'_>,
    expr: &ExprKind,
    absolute: bool,
    group_rtrn: &mut u32,
    pending: &mut bool,
) -> u32 {
    static PENDING_GROUP_INDEX_NAMES: [LookupEntry; 2] = [
        LookupEntry {
            name: GROUP_LAST_INDEX_NAME,
            value: 0,
        },
        LookupEntry { name: "", value: 0 },
    ];
    let group_name_pattern = NamedIntegerPattern {
        prefix: "Group",
        min: 1_u32,
        max: keymap_info.features.max_groups,
        entries: &keymap_info.lookup.group_index_names,
        pending_entries: &PENDING_GROUP_INDEX_NAMES,
        is_mask: false,
    };
    let lookup = IdentLookup::NamedPattern(&group_name_pattern);
    let ctx = &keymap_info.keymap.ctx;
    let mut result: i64 = 0_i64;
    if !expr_resolve_integer_lookup(ctx, expr, &mut result, Some(pending), &lookup) {
        return report_mismatch(keymap_info.strict);
    }
    if result > keymap_info.features.max_groups as i64
        || (absolute && result < 1)
        || (!absolute && result < -(keymap_info.features.max_groups as i64))
    {
        return report_mismatch(keymap_info.strict);
    }
    *group_rtrn = result as u32;
    PARSER_SUCCESS
}

pub(crate) fn expr_resolve_level(ctx: &XkbContext, expr: &ExprKind, level_rtrn: &mut u32) -> bool {
    let pattern = NamedIntegerPattern {
        prefix: "Level",
        min: 1_u32,
        max: XKB_LEVEL_MAX_IMPL as u32,
        entries: &LEVEL_NAME_PATTERN_ENTRIES,
        pending_entries: &LEVEL_NAME_PATTERN_ENTRIES,
        is_mask: false,
    };
    let lookup = IdentLookup::NamedPattern(&pattern);
    let mut result: i64 = 0_i64;
    if !expr_resolve_integer_lookup(ctx, expr, &mut result, None, &lookup) {
        return false;
    }
    if result < 1_i64 || result > XKB_LEVEL_MAX_IMPL as i64 {
        return false;
    }
    *level_rtrn = (result - 1_i64) as u32;
    true
}

pub(crate) fn expr_resolve_button(ctx: &XkbContext, expr: &ExprKind, btn_rtrn: &mut i64) -> bool {
    let lookup = IdentLookup::Simple(&BUTTON_NAMES);
    expr_resolve_integer_lookup(ctx, expr, btn_rtrn, None, &lookup)
}

pub(crate) fn expr_resolve_string(expr: &ExprKind, val_rtrn: &mut u32) -> bool {
    match expr.stmt_type() {
        4 => {
            let ExprKind::String(s) = &expr else {
                unreachable!()
            };
            *val_rtrn = *s;
            return true;
        }
        5..=9 => {
            return false;
        }
        10 => {
            return false;
        }
        12 => {
            let ExprKind::FieldRef { .. } = &expr else {
                unreachable!()
            };
            return false;
        }
        17 | 18 | 19 | 20 | 21 | 23 | 24 | 22 | 25 | 14 | 11 | 16 | 15 => {
            return false;
        }
        _ => {}
    }
    false
}

pub(crate) fn expr_resolve_enum(
    ctx: &XkbContext,
    expr: &ExprKind,
    val_rtrn: &mut u32,
    values: &[LookupEntry],
) -> bool {
    if expr.stmt_type() != STMT_EXPR_IDENT {
        return false;
    }
    let ExprKind::Ident(ident_atom) = &expr else {
        unreachable!()
    };
    if let Some(val) = simple_lookup(ctx, values, *ident_atom) {
        *val_rtrn = val;
        return true;
    }
    for entry in values {
        if entry.name.is_empty() {
            break;
        }
    }
    false
}

fn expr_resolve_mask_lookup(
    ctx: &XkbContext,
    expr: &ExprKind,
    val_rtrn: &mut u32,
    pending: Option<&mut bool>,
    lookup: &IdentLookup,
) -> bool {
    let ok: bool;
    let mut l: u32 = 0;
    let mut r: u32 = 0;
    let mut v: i64 = 0;
    let mut already_logged_error = false;
    match &expr {
        ExprKind::Integer(ival) => {
            if *ival < 0 || *ival > u32::MAX as i64 {
                return false;
            }
            *val_rtrn = *ival as u32;
            return true;
        }
        ExprKind::String(_)
        | ExprKind::Float
        | ExprKind::Boolean(_)
        | ExprKind::KeyName(_)
        | ExprKind::KeySym(_) => {
            return false;
        }
        ExprKind::Ident(ident_atom) => {
            let mut pending_local = false;
            let pending_ref = if pending.is_some() {
                Some(&mut pending_local)
            } else {
                None
            };
            if let Some(val) = ident_lookup(ctx, lookup, *ident_atom, pending_ref) {
                *val_rtrn = val;
                ok = true;
            } else {
                ok = false;
            }

            if let Some(p) = pending {
                *p = pending_local;
                if pending_local {
                    return false;
                }
            }
            return ok;
        }
        ExprKind::FieldRef { .. } | ExprKind::ArrayRef { .. } | ExprKind::Action { .. } => {}
        ExprKind::Binary {
            left: bleft,
            right: bright,
            op,
            ..
        } => {
            let left = bleft.as_deref().unwrap();
            let right = bright.as_deref().unwrap();
            if !expr_resolve_mask_lookup(ctx, left, &mut l, None, lookup)
                || !expr_resolve_mask_lookup(ctx, right, &mut r, None, lookup)
            {
                return false;
            }
            match *op {
                STMT_EXPR_ADD => {
                    *val_rtrn = l | r;
                }
                STMT_EXPR_SUBTRACT => {
                    *val_rtrn = l & !r;
                }
                STMT_EXPR_MULTIPLY | STMT_EXPR_DIVIDE => {
                    return false;
                }
                _ => {}
            }
            return true;
        }
        ExprKind::Unary { child, op } => {
            let left = child.as_deref().unwrap();
            if !expr_resolve_integer_lookup(ctx, left, &mut v, None, lookup) {
                return false;
            }
            if *op == STMT_EXPR_INVERT {
                if v < 0 || v > u32::MAX as i64 {
                    return false;
                }
                *val_rtrn = !(v as u32);
                return true;
            }
            return false;
        }
        ExprKind::ActionList { .. } | ExprKind::KeysymList { .. } | ExprKind::EmptyList => {
            already_logged_error = true;
        }
    }
    if !already_logged_error {
        return false;
    }
    false
}

pub(crate) fn expr_resolve_mask(
    ctx: &XkbContext,
    expr: &ExprKind,
    mask_rtrn: &mut u32,
    values: &[LookupEntry],
) -> bool {
    let lookup = IdentLookup::Simple(values);
    expr_resolve_mask_lookup(ctx, expr, mask_rtrn, None, &lookup)
}

pub(crate) fn expr_resolve_mod_mask(
    ctx: &XkbContext,
    expr: &ExprKind,
    mod_type: u32,
    mods: &XkbModSet,
    mask_rtrn: &mut u32,
) -> bool {
    let priv_0 = LookupModMaskPriv { mods, mod_type };
    let lookup = IdentLookup::ModMask(&priv_0);
    expr_resolve_mask_lookup(ctx, expr, mask_rtrn, None, &lookup)
}

pub(crate) fn expr_resolve_mod(
    _ctx: &XkbContext,
    def: &ExprKind,
    mod_type: u32,
    mods: &XkbModSet,
    ndx_rtrn: &mut u32,
) -> bool {
    if def.stmt_type() != STMT_EXPR_IDENT {
        return false;
    }
    let ExprKind::Ident(ident_atom) = &def else {
        unreachable!()
    };
    let name: u32 = *ident_atom;
    let ndx: u32 = xkb_mod_name_to_index(mods, name, mod_type);
    if ndx == XKB_MOD_INVALID {
        return false;
    }
    *ndx_rtrn = ndx;
    true
}

pub(crate) fn expr_resolve_group_mask(
    keymap_info: &mut XkbKeymapInfo<'_>,
    expr: &ExprKind,
    group_rtrn: &mut u32,
    pending_rtrn: &mut bool,
) -> bool {
    static PENDING_GROUP_MASK_NAMES: [LookupEntry; 2] = [
        LookupEntry {
            name: GROUP_LAST_INDEX_NAME,
            value: 0,
        },
        LookupEntry { name: "", value: 0 },
    ];
    let group_name_pattern = NamedIntegerPattern {
        prefix: "Group",
        min: 1_u32,
        max: keymap_info.features.max_groups,
        entries: &keymap_info.lookup.group_mask_names,
        pending_entries: &PENDING_GROUP_MASK_NAMES,
        is_mask: true,
    };
    let lookup = IdentLookup::NamedPattern(&group_name_pattern);
    let ctx = &keymap_info.keymap.ctx;
    expr_resolve_mask_lookup(ctx, expr, group_rtrn, Some(pending_rtrn), &lookup)
}
#[derive(Copy, Clone)]
pub(crate) struct ActionsInfo {
    pub(crate) actions: [XkbAction; 21],
}

pub(crate) const ACTION_FIELD_LATCH_ON_PRESS: u32 = 25;
pub(crate) const ACTION_FIELD_UNLOCK_ON_PRESS: u32 = 24;
pub(crate) const ACTION_FIELD_LOCK_ON_RELEASE: u32 = 23;
pub(crate) const ACTION_FIELD_MODS_TO_CLEAR: u32 = 22;
pub(crate) const ACTION_FIELD_KEYCODE: u32 = 21;
pub(crate) const ACTION_FIELD_DEVICE: u32 = 20;
pub(crate) const ACTION_FIELD_DATA: u32 = 19;
pub(crate) const ACTION_FIELD_SAME: u32 = 18;
pub(crate) const ACTION_FIELD_SCREEN: u32 = 17;
pub(crate) const ACTION_FIELD_COUNT: u32 = 16;
pub(crate) const ACTION_FIELD_TYPE: u32 = 15;
pub(crate) const ACTION_FIELD_CONTROLS: u32 = 14;
pub(crate) const ACTION_FIELD_VALUE: u32 = 13;
pub(crate) const ACTION_FIELD_BUTTON: u32 = 12;
pub(crate) const ACTION_FIELD_ACCEL: u32 = 11;
pub(crate) const ACTION_FIELD_Y: u32 = 10;
pub(crate) const ACTION_FIELD_X: u32 = 9;
pub(crate) const ACTION_FIELD_GROUP: u32 = 8;
pub(crate) const ACTION_FIELD_MODIFIERS: u32 = 7;
pub(crate) const ACTION_FIELD_INCREMENT: u32 = 6;
pub(crate) const ACTION_FIELD_AFFECT: u32 = 5;
pub(crate) const ACTION_FIELD_DEFAULT: u32 = 4;
pub(crate) const ACTION_FIELD_REPORT: u32 = 3;
pub(crate) const ACTION_FIELD_GEN_KEY_EVENT: u32 = 2;
pub(crate) const ACTION_FIELD_LATCH_TO_LOCK: u32 = 1;
pub(crate) const ACTION_FIELD_CLEAR_LOCKS: u32 = 0;
/// A value passed to an action handler.  Combines what used to be two separate
/// parameters (`value: &ExprDef` and `value_ptr: Option<&mut Option<Box<ExprDef>>>`).
pub(crate) enum ActionValue<'v> {
    /// A borrowed reference to a constant or non-ownable ExprDef (e.g. const_true).
    Borrowed(&'v ExprKind),
    /// A mutable reference to an owned ExprDef that can be `.take()`-en.
    Owned(&'v mut Option<Box<ExprKind>>),
}

impl<'v> ActionValue<'v> {
    /// Get a shared reference to the underlying ExprDef.
    #[inline]
    pub(crate) fn get(&self) -> &ExprKind {
        match self {
            ActionValue::Borrowed(e) => e,
            ActionValue::Owned(opt) => opt.as_deref().unwrap(),
        }
    }
    /// Take ownership of the ExprDef (only possible for Owned variant).
    #[inline]
    pub(crate) fn take(&mut self) -> Option<Box<ExprKind>> {
        match self {
            ActionValue::Borrowed(_) => None,
            ActionValue::Owned(opt) => opt.take(),
        }
    }
}

pub(crate) type ActionHandler = fn(
    &mut XkbKeymapInfo<'_>,
    &XkbModSet,
    &mut XkbAction,
    u32,
    Option<&ExprKind>,
    ActionValue<'_>,
) -> u32;
// Constant true/false ExprDef values used in handle_action_def
fn const_true_expr() -> ExprKind {
    ExprKind::Boolean(true)
}
fn const_false_expr() -> ExprKind {
    ExprKind::Boolean(false)
}
pub(crate) fn init_actions_info(keymap: &XkbKeymap, info: &mut ActionsInfo) {
    let mut type_0: u32 = ACTION_TYPE_NONE;
    while type_0 < _ACTION_TYPE_NUM_ENTRIES {
        info.actions[type_0 as usize] = XkbAction::from_type(type_0);
        type_0 += 1;
    }
    info.actions[ACTION_TYPE_PTR_DEFAULT as usize]
        .as_dflt_mut()
        .flags = 0;
    info.actions[ACTION_TYPE_PTR_DEFAULT as usize]
        .as_dflt_mut()
        .value = 1_i8;
    info.actions[ACTION_TYPE_PTR_MOVE as usize]
        .as_ptr_mut()
        .flags = ACTION_ACCEL;
    info.actions[ACTION_TYPE_SWITCH_VT as usize]
        .as_screen_mut()
        .flags = ACTION_SAME_SCREEN;
    info.actions[ACTION_TYPE_REDIRECT_KEY as usize]
        .as_redirect_mut()
        .keycode = keymap.redirect_key_auto;
}
static FIELD_STRINGS: [LookupEntry; 37] = [
    LookupEntry {
        name: "clearLocks",
        value: ACTION_FIELD_CLEAR_LOCKS,
    },
    LookupEntry {
        name: "latchToLock",
        value: ACTION_FIELD_LATCH_TO_LOCK,
    },
    LookupEntry {
        name: "genKeyEvent",
        value: ACTION_FIELD_GEN_KEY_EVENT,
    },
    LookupEntry {
        name: "generateKeyEvent",
        value: ACTION_FIELD_GEN_KEY_EVENT,
    },
    LookupEntry {
        name: "report",
        value: ACTION_FIELD_REPORT,
    },
    LookupEntry {
        name: "default",
        value: ACTION_FIELD_DEFAULT,
    },
    LookupEntry {
        name: "affect",
        value: ACTION_FIELD_AFFECT,
    },
    LookupEntry {
        name: "increment",
        value: ACTION_FIELD_INCREMENT,
    },
    LookupEntry {
        name: "modifiers",
        value: ACTION_FIELD_MODIFIERS,
    },
    LookupEntry {
        name: "mods",
        value: ACTION_FIELD_MODIFIERS,
    },
    LookupEntry {
        name: "group",
        value: ACTION_FIELD_GROUP,
    },
    LookupEntry {
        name: "x",
        value: ACTION_FIELD_X,
    },
    LookupEntry {
        name: "y",
        value: ACTION_FIELD_Y,
    },
    LookupEntry {
        name: "accel",
        value: ACTION_FIELD_ACCEL,
    },
    LookupEntry {
        name: "accelerate",
        value: ACTION_FIELD_ACCEL,
    },
    LookupEntry {
        name: "repeat",
        value: ACTION_FIELD_ACCEL,
    },
    LookupEntry {
        name: "button",
        value: ACTION_FIELD_BUTTON,
    },
    LookupEntry {
        name: "value",
        value: ACTION_FIELD_VALUE,
    },
    LookupEntry {
        name: "controls",
        value: ACTION_FIELD_CONTROLS,
    },
    LookupEntry {
        name: "ctrls",
        value: ACTION_FIELD_CONTROLS,
    },
    LookupEntry {
        name: "type",
        value: ACTION_FIELD_TYPE,
    },
    LookupEntry {
        name: "count",
        value: ACTION_FIELD_COUNT,
    },
    LookupEntry {
        name: "screen",
        value: ACTION_FIELD_SCREEN,
    },
    LookupEntry {
        name: "same",
        value: ACTION_FIELD_SAME,
    },
    LookupEntry {
        name: "sameServer",
        value: ACTION_FIELD_SAME,
    },
    LookupEntry {
        name: "data",
        value: ACTION_FIELD_DATA,
    },
    LookupEntry {
        name: "device",
        value: ACTION_FIELD_DEVICE,
    },
    LookupEntry {
        name: "dev",
        value: ACTION_FIELD_DEVICE,
    },
    LookupEntry {
        name: "key",
        value: ACTION_FIELD_KEYCODE,
    },
    LookupEntry {
        name: "keycode",
        value: ACTION_FIELD_KEYCODE,
    },
    LookupEntry {
        name: "kc",
        value: ACTION_FIELD_KEYCODE,
    },
    LookupEntry {
        name: "clearmods",
        value: ACTION_FIELD_MODS_TO_CLEAR,
    },
    LookupEntry {
        name: "clearmodifiers",
        value: ACTION_FIELD_MODS_TO_CLEAR,
    },
    LookupEntry {
        name: "lockOnRelease",
        value: ACTION_FIELD_LOCK_ON_RELEASE,
    },
    LookupEntry {
        name: "unlockOnPress",
        value: ACTION_FIELD_UNLOCK_ON_PRESS,
    },
    LookupEntry {
        name: "latchOnPress",
        value: ACTION_FIELD_LATCH_ON_PRESS,
    },
    LookupEntry { name: "", value: 0 },
];
fn string_to_action_type(str: &str, type_rtrn: &mut u32) -> bool {
    let mut type_0: u32 = 0;
    let ret: bool = lookup_string(&ACTION_TYPE_NAMES, str, &mut type_0);
    *type_rtrn = type_0;
    ret
}
fn string_to_field(str: &str, field_rtrn: &mut u32) -> bool {
    let mut field: u32 = 0;
    let ret: bool = lookup_string(&FIELD_STRINGS, str, &mut field);
    *field_rtrn = field;
    ret
}
#[inline]
fn report_mismatch(strict: u32) -> u32 {
    if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
        PARSER_FATAL_ERROR
    } else {
        PARSER_RECOVERABLE_ERROR
    }
}
#[inline]
fn report_format_version_mismatch(strict: u32) -> u32 {
    if strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
        PARSER_FATAL_ERROR
    } else {
        PARSER_SUCCESS
    }
}
#[inline]
fn report_illegal(strict: u32) -> u32 {
    if strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
        PARSER_FATAL_ERROR
    } else {
        PARSER_SUCCESS
    }
}

fn handle_no_action(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    _action: &mut XkbAction,
    _field: u32,
    _array_ndx: Option<&ExprKind>,
    _value: ActionValue<'_>,
) -> u32 {
    if keymap_info.strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
        PARSER_FATAL_ERROR
    } else {
        PARSER_SUCCESS
    }
}
fn check_boolean_flag(
    ctx: &XkbContext,
    strict: u32,
    flag: u32,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut u32,
) -> u32 {
    let mut set: bool = false;
    if array_ndx.is_some() {
        return report_mismatch(strict);
    }
    if !expr_resolve_boolean(ctx, value, &mut set) {
        return report_mismatch(strict);
    }
    if set {
        *flags_inout = *flags_inout | flag;
    } else {
        *flags_inout = *flags_inout & !flag;
    }
    PARSER_SUCCESS
}
fn check_modifier_field(
    ctx: &XkbContext,
    strict: u32,
    mods: &XkbModSet,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut u32,
    mods_rtrn: &mut u32,
) -> u32 {
    if array_ndx.is_some() {
        return report_mismatch(strict);
    }
    if value.stmt_type() == STMT_EXPR_IDENT {
        let ident = if let ExprKind::Ident(id) = &value {
            *id
        } else {
            unreachable!()
        };
        let val_str: &str = atom_text(&ctx.atom_table, ident);
        if !val_str.is_empty()
            && (val_str.eq_ignore_ascii_case("usemodmapmods")
                || val_str.eq_ignore_ascii_case("modmapmods"))
        {
            *mods_rtrn = 0;
            *flags_inout = *flags_inout | ACTION_MODS_LOOKUP_MODMAP;
            return PARSER_SUCCESS;
        }
    }
    if !expr_resolve_mod_mask(ctx, value, MOD_BOTH, mods, mods_rtrn) {
        return report_mismatch(strict);
    }
    *flags_inout = *flags_inout & !ACTION_MODS_LOOKUP_MODMAP;
    PARSER_SUCCESS
}
static LOCK_WHICH: [LookupEntry; 5] = [
    LookupEntry {
        name: "both",
        value: 0,
    },
    LookupEntry {
        name: "lock",
        value: ACTION_LOCK_NO_UNLOCK,
    },
    LookupEntry {
        name: "neither",
        value: ACTION_LOCK_NO_LOCK | ACTION_LOCK_NO_UNLOCK,
    },
    LookupEntry {
        name: "unlock",
        value: ACTION_LOCK_NO_LOCK,
    },
    LookupEntry { name: "", value: 0 },
];
fn check_affect_field(
    ctx: &XkbContext,
    strict: u32,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut u32,
) -> u32 {
    if array_ndx.is_some() {
        return report_mismatch(strict);
    }
    let mut flags: u32 = 0;
    if !expr_resolve_enum(ctx, value, &mut flags, &LOCK_WHICH) {
        return report_mismatch(strict);
    }
    *flags_inout = *flags_inout & !(ACTION_LOCK_NO_LOCK | ACTION_LOCK_NO_UNLOCK);
    *flags_inout = *flags_inout | flags;
    PARSER_SUCCESS
}
fn handle_set_latch_lock_mods(
    keymap_info: &mut XkbKeymapInfo<'_>,
    mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let act = action.as_mods_mut();
    let type_0: u32 = act.type_0;
    if field == ACTION_FIELD_MODIFIERS {
        return check_modifier_field(
            ctx,
            keymap_info.strict,
            mods,
            array_ndx,
            value,
            &mut act.flags,
            &mut act.mods.mods,
        );
    }
    if field == ACTION_FIELD_UNLOCK_ON_PRESS {
        if keymap_info.features.mods_unlock_on_press {
            return check_boolean_flag(
                ctx,
                keymap_info.strict,
                ACTION_UNLOCK_ON_PRESS,
                array_ndx,
                value,
                &mut act.flags,
            );
        } else {
            return report_format_version_mismatch(keymap_info.strict);
        }
    }
    if (type_0 == ACTION_TYPE_MOD_SET || type_0 == ACTION_TYPE_MOD_LATCH)
        && field == ACTION_FIELD_CLEAR_LOCKS
    {
        return check_boolean_flag(
            ctx,
            keymap_info.strict,
            ACTION_LOCK_CLEAR,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    if type_0 == ACTION_TYPE_MOD_LATCH {
        if field == ACTION_FIELD_LATCH_TO_LOCK {
            return check_boolean_flag(
                ctx,
                keymap_info.strict,
                ACTION_LATCH_TO_LOCK,
                array_ndx,
                value,
                &mut act.flags,
            );
        }
        if field == ACTION_FIELD_LATCH_ON_PRESS {
            if keymap_info.features.mods_latch_on_press {
                return check_boolean_flag(
                    ctx,
                    keymap_info.strict,
                    ACTION_LATCH_ON_PRESS,
                    array_ndx,
                    value,
                    &mut act.flags,
                );
            } else {
                return report_format_version_mismatch(keymap_info.strict);
            }
        }
    }
    if type_0 == ACTION_TYPE_MOD_LOCK && field == ACTION_FIELD_AFFECT {
        return check_affect_field(ctx, keymap_info.strict, array_ndx, value, &mut act.flags);
    }
    report_illegal(keymap_info.strict)
}
fn check_group_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    array_ndx: Option<&ExprKind>,
    mut value: ActionValue<'_>,
    flags_inout: &mut u32,
    group_rtrn: &mut i32,
) -> u32 {
    let mut idx: u32 = 0;
    let mut flags: u32 = *flags_inout;
    if array_ndx.is_some() {
        return report_mismatch(keymap_info.strict);
    }
    // If the value is a unary negate/plus, rebind to child and record negate.
    let is_negate = value.get().stmt_type() == STMT_EXPR_NEGATE;
    let is_unary = is_negate || value.get().stmt_type() == STMT_EXPR_UNARY_PLUS;
    if is_unary {
        flags = flags & !ACTION_ABSOLUTE_SWITCH;
        // Rebind value to the child field inside the unary expr
        // (for ownership transfer to pending_computations if needed)
        value = match value {
            ActionValue::Owned(opt) => ActionValue::Owned(opt),
            other => other,
        };
    } else {
        flags = flags as u32 | ACTION_ABSOLUTE_SWITCH;
    }
    let spec_holder = value.get();
    let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH != 0;
    let mut pending: bool = false;
    let ret: u32 =
        expr_resolve_group(keymap_info, spec_holder, absolute, &mut idx, &mut pending) as u32;
    if ret as u32 != PARSER_SUCCESS && !pending {
        return ret;
    }
    if pending {
        flags = flags as u32 | ACTION_PENDING_COMPUTATION;
        let pending_index: u32 = keymap_info.pending_computations.len() as u32;
        keymap_info.pending_computations.push(PendingComputation {
            expr: value.take(),
            computed: false,
            value: 0,
        });
        *group_rtrn = pending_index as i32;
    } else {
        flags = flags & !ACTION_PENDING_COMPUTATION;
        if flags & ACTION_ABSOLUTE_SWITCH == 0 {
            *group_rtrn = idx as i32;
            if is_negate {
                *group_rtrn = -*group_rtrn;
            }
        } else {
            *group_rtrn = idx.wrapping_sub(1_u32) as i32;
        }
    }
    *flags_inout = flags;
    PARSER_SUCCESS
}
fn handle_set_latch_lock_group(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let type_0: u32 = action.action_type();
    if field == ACTION_FIELD_GROUP {
        let act = action.as_group_mut();
        return check_group_field(
            keymap_info,
            array_ndx,
            value,
            &mut act.flags,
            &mut act.group,
        );
    }
    let value = value.get();
    let act = action.as_group_mut();
    if (type_0 == ACTION_TYPE_GROUP_SET || type_0 == ACTION_TYPE_GROUP_LATCH)
        && field == ACTION_FIELD_CLEAR_LOCKS
    {
        return check_boolean_flag(
            ctx,
            keymap_info.strict,
            ACTION_LOCK_CLEAR,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    if type_0 == ACTION_TYPE_GROUP_LATCH && field == ACTION_FIELD_LATCH_TO_LOCK {
        return check_boolean_flag(
            ctx,
            keymap_info.strict,
            ACTION_LATCH_TO_LOCK,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    if type_0 == ACTION_TYPE_GROUP_LOCK && field == ACTION_FIELD_LOCK_ON_RELEASE {
        if keymap_info.features.group_lock_on_release {
            return check_boolean_flag(
                ctx,
                keymap_info.strict,
                ACTION_LOCK_ON_RELEASE,
                array_ndx,
                value,
                &mut act.flags,
            );
        } else {
            return report_format_version_mismatch(keymap_info.strict);
        }
    }
    report_illegal(keymap_info.strict)
}
fn handle_move_ptr(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let act = action.as_ptr_mut();
    if field == ACTION_FIELD_X || field == ACTION_FIELD_Y {
        let mut val: i64 = 0_i64;
        let absolute: bool =
            value.stmt_type() != STMT_EXPR_NEGATE && value.stmt_type() != STMT_EXPR_UNARY_PLUS;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if !expr_resolve_integer(ctx, value, &mut val) {
            return report_mismatch(keymap_info.strict);
        }
        if val < i16::MIN as i64 || val > i16::MAX as i64 {
            return report_mismatch(keymap_info.strict);
        }
        if field == ACTION_FIELD_X {
            if absolute {
                act.flags = act.flags | ACTION_ABSOLUTE_X;
            }
            act.x = val as i16;
        } else {
            if absolute {
                act.flags = act.flags | ACTION_ABSOLUTE_Y;
            }
            act.y = val as i16;
        }
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_ACCEL {
        return check_boolean_flag(
            ctx,
            keymap_info.strict,
            ACTION_ACCEL,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    report_illegal(keymap_info.strict)
}
fn handle_ptr_btn(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let type_0 = action.action_type();
    let act = action.as_btn_mut();
    if field == ACTION_FIELD_BUTTON {
        let mut btn: i64 = 0_i64;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if !expr_resolve_button(ctx, value, &mut btn) {
            return report_mismatch(keymap_info.strict);
        }
        if !(0_i64..=5_i64).contains(&btn) {
            return report_mismatch(keymap_info.strict);
        }
        act.button = btn as u8;
        return PARSER_SUCCESS;
    } else if type_0 == ACTION_TYPE_PTR_LOCK && field == ACTION_FIELD_AFFECT {
        return check_affect_field(ctx, keymap_info.strict, array_ndx, value, &mut act.flags);
    } else if field == ACTION_FIELD_COUNT {
        let mut val: i64 = 0_i64;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if !expr_resolve_integer(ctx, value, &mut val) {
            return report_mismatch(keymap_info.strict);
        }
        if !(0_i64..=255_i64).contains(&val) {
            return report_mismatch(keymap_info.strict);
        }
        act.count = val as u8;
        return PARSER_SUCCESS;
    }
    report_illegal(keymap_info.strict)
}
static PTR_DFLTS: [LookupEntry; 4] = [
    LookupEntry {
        name: "dfltbtn",
        value: 1,
    },
    LookupEntry {
        name: "defaultbutton",
        value: 1,
    },
    LookupEntry {
        name: "button",
        value: 1,
    },
    LookupEntry { name: "", value: 0 },
];
fn handle_set_ptr_dflt(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let act = action.as_dflt_mut();
    if field == ACTION_FIELD_AFFECT {
        let mut val: u32 = 0;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if !expr_resolve_enum(ctx, value, &mut val, &PTR_DFLTS) {
            return report_mismatch(keymap_info.strict);
        }
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_BUTTON || field == ACTION_FIELD_VALUE {
        let button: &ExprKind;
        let mut btn: i64 = 0_i64;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if value.stmt_type() == STMT_EXPR_NEGATE || value.stmt_type() == STMT_EXPR_UNARY_PLUS {
            act.flags = act.flags & !ACTION_ABSOLUTE_SWITCH;
            button = if let ExprKind::Unary { child, .. } = &value {
                child.as_deref().unwrap()
            } else {
                unreachable!()
            };
        } else {
            act.flags = act.flags | ACTION_ABSOLUTE_SWITCH;
            button = value;
        }
        if !expr_resolve_button(ctx, button, &mut btn) {
            return report_mismatch(keymap_info.strict);
        }
        if !(0_i64..=5_i64).contains(&btn) {
            return report_mismatch(keymap_info.strict);
        }
        if btn == 0_i64 {
            return report_mismatch(keymap_info.strict);
        }
        act.value = (if value.stmt_type() == STMT_EXPR_NEGATE {
            -btn
        } else {
            btn
        }) as i8;
        return PARSER_SUCCESS;
    }
    report_illegal(keymap_info.strict)
}
fn handle_switch_screen(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let act = action.as_screen_mut();
    if field == ACTION_FIELD_SCREEN {
        let scrn: &ExprKind;
        let mut val: i64 = 0_i64;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if value.stmt_type() == STMT_EXPR_NEGATE || value.stmt_type() == STMT_EXPR_UNARY_PLUS {
            act.flags = act.flags & !ACTION_ABSOLUTE_SWITCH;
            scrn = if let ExprKind::Unary { child, .. } = &value {
                child.as_deref().unwrap()
            } else {
                unreachable!()
            };
        } else {
            act.flags = act.flags | ACTION_ABSOLUTE_SWITCH;
            scrn = value;
        }
        if !expr_resolve_integer(ctx, scrn, &mut val) {
            return report_mismatch(keymap_info.strict);
        }
        val = if value.stmt_type() == STMT_EXPR_NEGATE {
            -val
        } else {
            val
        };
        if val < i8::MIN as i64 || val > i8::MAX as i64 {
            return report_mismatch(keymap_info.strict);
        }
        act.screen = val as i8;
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_SAME {
        return check_boolean_flag(
            ctx,
            keymap_info.strict,
            ACTION_SAME_SCREEN,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    report_illegal(keymap_info.strict)
}
fn handle_set_lock_controls(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let type_0 = action.action_type();
    let act = action.as_ctrls_mut();
    if field == ACTION_FIELD_CONTROLS {
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        let mut mask: u32 = 0;
        let offset: u8 = keymap_info.features.controls_name_offset;
        if !expr_resolve_mask(ctx, value, &mut mask, &CTRL_MASK_NAMES[offset as usize..]) {
            return report_mismatch(keymap_info.strict);
        }
        act.ctrls = mask;
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_AFFECT && type_0 == ACTION_TYPE_CTRL_LOCK {
        return check_affect_field(ctx, keymap_info.strict, array_ndx, value, &mut act.flags);
    }
    report_illegal(keymap_info.strict)
}
fn handle_redirect_key(
    keymap_info: &mut XkbKeymapInfo<'_>,
    mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let act = action.as_redirect_mut();
    if field == ACTION_FIELD_KEYCODE {
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if value.stmt_type() == STMT_EXPR_IDENT {
            let ident = if let ExprKind::Ident(id) = &value {
                *id
            } else {
                unreachable!()
            };
            let val_str: &str = atom_text(&keymap_info.keymap.ctx.atom_table, ident);
            if !val_str.is_empty() && val_str.eq_ignore_ascii_case("auto") {
                act.keycode = keymap_info.keymap.redirect_key_auto;
                return PARSER_SUCCESS;
            }
        }
        if value.stmt_type() != STMT_EXPR_KEYNAME_LITERAL {
            return report_mismatch(keymap_info.strict);
        }
        let key_name_val = if let ExprKind::KeyName(kn) = &value {
            *kn
        } else {
            unreachable!()
        };
        let key = keymap_info.keymap.key_by_name(key_name_val, true);
        if let Some(key) = key {
            act.keycode = key.keycode;
            return PARSER_SUCCESS;
        } else {
            return if keymap_info.strict & PARSER_NO_FIELD_VALUE_MISMATCH != 0 {
                PARSER_FATAL_ERROR
            } else {
                PARSER_RECOVERABLE_ERROR
            };
        }
    }
    if field == ACTION_FIELD_MODIFIERS || field == ACTION_FIELD_MODS_TO_CLEAR {
        let mut flags: u32 = 0;
        let mut m: u32 = 0;
        let ctx: &XkbContext = &keymap_info.keymap.ctx;
        let r: u32 = check_modifier_field(
            ctx,
            keymap_info.strict,
            mods,
            array_ndx,
            value,
            &mut flags,
            &mut m,
        );
        if r as u32 != PARSER_SUCCESS {
            return r;
        }
        if flags as u64 != 0 {
            return report_mismatch(keymap_info.strict);
        }
        act.affect |= m;
        if field == ACTION_FIELD_MODIFIERS {
            act.mods |= m;
        } else {
            act.mods &= !m;
        }
        return PARSER_SUCCESS;
    }
    report_illegal(keymap_info.strict)
}
fn handle_unsupported(
    _keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    _action: &mut XkbAction,
    _field: u32,
    _array_ndx: Option<&ExprKind>,
    _value: ActionValue<'_>,
) -> u32 {
    PARSER_SUCCESS
}
fn handle_private(
    keymap_info: &mut XkbKeymapInfo<'_>,
    _mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let act = action.as_priv_mut();
    if field == ACTION_FIELD_TYPE {
        let mut type_0: i64 = 0_i64;
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        if !expr_resolve_integer(ctx, value, &mut type_0) {
            return report_mismatch(keymap_info.strict);
        }
        if !(0_i64..=255_i64).contains(&type_0) {
            return report_mismatch(keymap_info.strict);
        }
        if type_0 < ACTION_TYPE_PRIVATE as i64 {
            act.type_0 = ACTION_TYPE_NONE;
        } else {
            act.type_0 = type_0 as u32;
        }
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_DATA {
        if array_ndx.is_none() {
            let mut val: u32 = XKB_ATOM_NONE;
            if !expr_resolve_string(value, &mut val) {
                return report_mismatch(keymap_info.strict);
            }
            let str_bytes: &str = atom_text(&ctx.atom_table, val);
            let len: usize = str_bytes.len();
            if len < 1_usize || len > std::mem::size_of::<[u8; 7]>() {
                return report_mismatch(keymap_info.strict);
            }
            act.data = [0u8; 7];
            act.data[..len].copy_from_slice(&str_bytes.as_bytes()[..len]);
            return PARSER_SUCCESS;
        } else {
            let array_ndx = array_ndx.unwrap();
            let mut ndx: i64 = 0_i64;
            let mut datum: i64 = 0_i64;
            if !expr_resolve_integer(ctx, array_ndx, &mut ndx) {
                return report_mismatch(keymap_info.strict);
            }
            if ndx < 0_i64 || ndx as usize >= std::mem::size_of::<[u8; 7]>() {
                return report_mismatch(keymap_info.strict);
            }
            if !expr_resolve_integer(ctx, value, &mut datum) {
                return report_mismatch(keymap_info.strict);
            }
            if !(0_i64..=255_i64).contains(&datum) {
                return report_mismatch(keymap_info.strict);
            }
            act.data[ndx as usize] = datum as u8;
            return PARSER_SUCCESS;
        }
    }
    report_illegal(keymap_info.strict)
}
static HANDLE_ACTION: [ActionHandler; 21] = [
    handle_no_action,
    handle_no_action,
    handle_set_latch_lock_mods,
    handle_set_latch_lock_mods,
    handle_set_latch_lock_mods,
    handle_set_latch_lock_group,
    handle_set_latch_lock_group,
    handle_set_latch_lock_group,
    handle_move_ptr,
    handle_ptr_btn,
    handle_ptr_btn,
    handle_set_ptr_dflt,
    handle_no_action,
    handle_switch_screen,
    handle_set_lock_controls,
    handle_set_lock_controls,
    handle_redirect_key,
    handle_unsupported,
    handle_unsupported,
    handle_private,
    handle_no_action,
];

pub(crate) fn handle_action_def(
    keymap_info: &mut XkbKeymapInfo<'_>,
    info: &mut ActionsInfo,
    mods: &XkbModSet,
    def: &mut ExprKind,
    action: &mut XkbAction,
) -> u32 {
    if def.stmt_type() != STMT_EXPR_ACTION_DECL {
        return PARSER_FATAL_ERROR;
    }
    // Extract action name atom (Copy type, no borrow held)
    let action_name_atom = if let ExprKind::Action { name, .. } = &def {
        *name
    } else {
        unreachable!()
    };
    let action_name: &str = atom_text(&keymap_info.keymap.ctx.atom_table, action_name_atom);
    let mut handler_type: u32 = ACTION_TYPE_NONE;
    if !string_to_action_type(action_name, &mut handler_type) {
        handler_type = ACTION_TYPE_UNKNOWN;
        if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION != 0 {
            return PARSER_FATAL_ERROR;
        }
    }
    *action = info.actions[handler_type as usize];
    if handler_type == ACTION_TYPE_UNSUPPORTED_LEGACY {
        action.set_none();
    }
    let mut ret: u32 = PARSER_SUCCESS;
    let const_true = const_true_expr();
    let const_false = const_false_expr();
    // Get mutable access to the args Vec
    let args = if let ExprKind::Action { ref mut args, .. } = def {
        args
    } else {
        unreachable!()
    };
    for arg in args.iter_mut() {
        let av: ActionValue<'_>;
        let field_ref: &ExprKind;
        let mut array_rtrn_opt: Option<&ExprKind> = None;
        let mut elem_rtrn_atom: u32 = 0;
        let mut field_rtrn_atom: u32 = 0;
        if arg.stmt_type() == STMT_EXPR_ASSIGN {
            if let ExprKind::Binary {
                ref left,
                ref mut right,
                ..
            } = arg
            {
                field_ref = left.as_deref().unwrap();
                av = ActionValue::Owned(right);
            } else {
                unreachable!()
            }
        } else if arg.stmt_type() == STMT_EXPR_NOT || arg.stmt_type() == STMT_EXPR_INVERT {
            field_ref = if let ExprKind::Unary { ref child, .. } = arg {
                child.as_deref().unwrap()
            } else {
                unreachable!()
            };
            av = ActionValue::Borrowed(&const_false);
        } else {
            field_ref = &*arg;
            av = ActionValue::Borrowed(&const_true);
        }
        if !expr_resolve_lhs(
            field_ref,
            &mut elem_rtrn_atom,
            &mut field_rtrn_atom,
            &mut array_rtrn_opt,
        ) {
            return PARSER_FATAL_ERROR;
        }
        let elem_rtrn = atom_text(&keymap_info.keymap.ctx.atom_table, elem_rtrn_atom);
        let field_rtrn = atom_text(&keymap_info.keymap.ctx.atom_table, field_rtrn_atom);
        if !elem_rtrn.is_empty() {
            return PARSER_FATAL_ERROR;
        }
        let mut field_ndx: u32 = ACTION_FIELD_CLEAR_LOCKS;
        if !string_to_field(field_rtrn, &mut field_ndx) {
            if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
                return PARSER_FATAL_ERROR;
            }
        } else {
            match HANDLE_ACTION[handler_type as usize](
                keymap_info,
                mods,
                action,
                field_ndx,
                array_rtrn_opt,
                av,
            ) {
                2 => return PARSER_FATAL_ERROR,
                1 => {
                    ret = PARSER_RECOVERABLE_ERROR;
                }
                _ => {}
            }
        }
    }
    if action.action_type() == ACTION_TYPE_UNKNOWN {
        PARSER_RECOVERABLE_ERROR
    } else {
        ret
    }
}
pub(crate) fn set_default_action_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    info: &mut ActionsInfo,
    mods: &mut XkbModSet,
    elem: &str,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_rtrn: &mut Option<Box<ExprKind>>,
    merge: u32,
) -> u32 {
    let av = ActionValue::Owned(value_rtrn);
    let mut action: u32 = ACTION_TYPE_NONE;
    if !string_to_action_type(elem, &mut action) {
        return if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION != 0 {
            PARSER_FATAL_ERROR
        } else {
            PARSER_RECOVERABLE_ERROR
        };
    }
    let mut action_field: u32 = ACTION_FIELD_CLEAR_LOCKS;
    if !string_to_field(field, &mut action_field) {
        return if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
            PARSER_FATAL_ERROR
        } else {
            PARSER_RECOVERABLE_ERROR
        };
    }
    let into: &mut XkbAction = &mut info.actions[action as usize];
    let mut from: XkbAction = *into;
    let ret: u32 =
        HANDLE_ACTION[action as usize](keymap_info, mods, &mut from, action_field, array_ndx, av);
    if ret != PARSER_SUCCESS {
        return ret;
    }
    if !action_equal(into, &from) {
        let replace: bool = merge != MERGE_AUGMENT;
        if replace {
            *into = from;
        }
    }
    PARSER_SUCCESS
}
