pub(crate) use super::keymap::xkb_mod_name_to_index;
use super::keymap::{
    lookup_string, CTRL_MASK_NAMES, GROUP_COMPONENT_MASK_NAMES, MOD_COMPONENT_MASK_NAMES,
    SYM_INTERPRET_MATCH_MASK_NAMES, USE_MOD_MAP_VALUE_NAMES,
};
use super::keysym::xkb_keysym_is_keypad;
use super::keysym::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
use super::parser::{exceeds_include_max_depth, process_include_file};
pub(crate) use super::parser::{
    InterpDef, KeyAliasDef, KeycodeDef, LedNameDef, ModMapDef, NamedVarDef,
};
pub(crate) use super::parser::{
    MergeMode, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH,
    ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH,
    ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE,
    ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE,
    ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, MAX_ACTIONS_PER_LEVEL, MOD_REAL_MASK_ALL,
    XKB_MAX_LEDS, XKB_MOD_NONE, XKB_OVERLAY_INVALID, _ACTION_TYPE_NUM_ENTRIES,
};
use std::collections::HashMap;

macro_rules! some_or_false {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return false,
        }
    };
    ($value:expr, $return:expr) => {
        match $value {
            Some(value) => value,
            None => return $return,
        }
    };
}
macro_rules! include_file {
    ($ki:expr, $info:expr, $stmt:expr, $type:expr) => {
        match process_include_file(&mut $ki.keymap.ctx, $stmt, $type) {
            Some(file) => file,
            None => {
                $info.error_count += 10;
                return false;
            }
        }
    };
}

#[derive(Default)]
pub(crate) struct SymbolsInfo {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) explicit_group: Option<u32>,
    pub(crate) max_groups: u32,
    pub(crate) keys: Vec<KeyInfo>,
    pub(crate) default_key: KeyInfo,
    pub(crate) default_actions: ActionsInfo,
    pub(crate) group_names: Vec<u32>,
    pub(crate) modmaps: Vec<ModMapEntry>,
    pub(crate) mods: XkbModSet,
    pub(crate) star_atom: u32,
    pub(crate) key_index: HashMap<u32, usize>,
    pub(crate) modmap_index: HashMap<ModMapTarget, usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ModMapTarget {
    Key(u32),
    Symbol(u32),
}

#[derive(Copy, Clone)]
pub(crate) struct ModMapEntry {
    pub(crate) merge: MergeMode,
    pub(crate) modifier: u32,
    pub(crate) target: ModMapTarget,
}
#[derive(Clone, Default)]
pub(crate) struct KeyInfo {
    pub(crate) name: u32,
    pub(crate) vmodmap: Option<u32>,
    pub(crate) default_type: Option<u32>,
    pub(crate) out_of_range_group_number: u32,
    pub(crate) groups: Vec<GroupInfo>,
    pub(crate) out_of_range_group_policy: u32,
    pub(crate) defined: u32,
    pub(crate) merge: MergeMode,
    pub(crate) repeat: u32,
    pub(crate) out_of_range_pending_group: bool,
    pub(crate) overlays_clear: bool,
    pub(crate) overlays: [Option<u32>; 8],
}
pub(crate) const KEY_REPEAT_NO: u32 = 2;
pub(crate) const KEY_REPEAT_YES: u32 = 1;
pub(crate) const KEY_REPEAT_UNDEFINED: u32 = 0;
pub(crate) const KEY_FIELD_OVERLAY: u32 = 16;
pub(crate) const KEY_FIELD_GROUPINFO: u32 = 4;
pub(crate) const KEY_FIELD_REPEAT: u32 = 1;
#[derive(Clone, Default)]
pub(crate) struct GroupInfo {
    pub(crate) levels: Vec<XkbLevel>,
    pub(crate) defined: u32,
    pub(crate) type_0: Option<u32>,
}

pub(crate) const GROUP_FIELD_ACTS: u32 = 2;
pub(crate) const GROUP_FIELD_SYMS: u32 = 1;

impl SymbolsInfo {
    fn new(ki: &mut XkbKeymapInfo<'_>, include_depth: u32, mods: &XkbModSet) -> Self {
        let star_atom = ki.keymap.ctx.atom_intern(b"*");
        let mut info = Self {
            include_depth,
            max_groups: ki.features.max_groups,
            keys: Vec::with_capacity(256),
            default_key: KeyInfo {
                name: star_atom,
                out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
                ..Default::default()
            },
            star_atom,
            ..Default::default()
        };
        init_actions_info(&mut info.default_actions);
        init_vmods(&mut info.mods, mods, include_depth > 0);
        info
    }
}

fn is_action_list_value(value: &ExprKind) -> bool {
    match value {
        ExprKind::ActionList { actions } => match actions.first() {
            None => true,
            Some(first) => matches!(first, ExprKind::ActionList { .. } | ExprKind::Action { .. }),
        },
        _ => false,
    }
}

/// Extract child expressions from an ActionList container node, or return a single-element slice.
fn collect_expr_list(container: &ExprKind) -> &[ExprKind] {
    match container {
        ExprKind::ActionList { actions } => actions.as_slice(),
        _ => std::slice::from_ref(container),
    }
}

fn init_key_info_with_atom(keyi: &mut KeyInfo, star_atom: u32) {
    *keyi = KeyInfo {
        name: star_atom,
        out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
        ..Default::default()
    };
}
fn merge_groups(into: &mut GroupInfo, from: &mut GroupInfo, clobber: bool) -> bool {
    if let Some(from_type) = from.type_0 {
        if into.type_0.is_none() || clobber {
            into.type_0 = Some(from_type);
        }
    }
    if from.levels.is_empty() {
        *from = GroupInfo::default();
        return true;
    }
    if into.levels.is_empty() {
        from.type_0 = into.type_0;
        *into = std::mem::take(from);
        return true;
    }
    let levels_in_both = into.levels.len().min(from.levels.len());
    let mut from_keysyms_count = 0;
    let mut from_actions_count = 0;
    for (into_level, from_level) in into.levels.iter_mut().zip(&mut from.levels) {
        if from_level.syms.is_empty() && from_level.actions.is_empty() {
            continue;
        }
        if into_level.syms.is_empty() && into_level.actions.is_empty() {
            into_level.syms = std::mem::take(&mut from_level.syms);
            into_level.actions = std::mem::take(&mut from_level.actions);
            from_keysyms_count += 1;
            from_actions_count += 1;
            continue;
        }
        if !from_level.syms.is_empty()
            && from_level.syms != into_level.syms
            && (clobber || into_level.syms.is_empty())
        {
            into_level.syms = std::mem::take(&mut from_level.syms);
            from_keysyms_count += 1;
        }
        if !from_level.actions.is_empty()
            && (into_level.actions.len() != from_level.actions.len()
                || !into_level
                    .actions
                    .iter()
                    .zip(&from_level.actions)
                    .all(|(a, b)| action_equal(a, b)))
            && (clobber || into_level.actions.is_empty())
        {
            into_level.actions = std::mem::take(&mut from_level.actions);
            from_actions_count += 1;
        }
    }
    for level in from.levels.drain(levels_in_both..) {
        into.levels.push(level);
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
fn use_new_field(field: u32, old: u32, new: u32, clobber: bool) -> bool {
    new & field != 0 && (old & field == 0 || clobber)
}
fn overlays_insert(keyi: &mut KeyInfo, bit: u8, key: u32) -> bool {
    if let Some(entry) = keyi.overlays.get_mut(bit as usize) {
        *entry = Some(key);
        if key == XKB_KEYCODE_INVALID {
            keyi.overlays_clear = true;
        }
        true
    } else {
        false
    }
}
fn merge_overlays(ki: &XkbKeymapInfo<'_>, into: &mut KeyInfo, from: &mut KeyInfo) -> bool {
    if (from.defined & KEY_FIELD_OVERLAY) != 0 {
        if (into.defined & KEY_FIELD_OVERLAY) == 0 {
            into.overlays = from.overlays;
            into.defined |= KEY_FIELD_OVERLAY;
        } else if into.overlays_clear && from.overlays_clear {
            // Both cleared — keep both invalid entries
        } else if ki.features.overlapping_overlays {
            for i in 0..8 {
                if let Some(key) = from.overlays[i] {
                    let dest_key = into.overlays[i];
                    if dest_key.is_none() {
                        into.overlays[i] = Some(key);
                        into.overlays_clear = false;
                    }
                }
            }
        } else {
            let into_has = into.overlays.iter().any(|o| o.is_some());
            let from_has = from.overlays.iter().any(|o| o.is_some());
            if into_has == from_has
                && into.overlays == from.overlays
                && into.overlays_clear == from.overlays_clear
            {
                return true;
            }
            let no_overlap = into
                .overlays
                .iter()
                .zip(from.overlays.iter())
                .all(|(a, b)| a.is_none() || b.is_none());
            if no_overlap {
                if into.overlays_clear {
                    into.overlays = from.overlays;
                    into.overlays_clear = from.overlays_clear;
                    return true;
                } else if from.overlays_clear {
                    return true;
                }
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
    let clobber: bool = from.merge != MergeMode::Augment;
    if from.merge == MergeMode::Replace {
        std::mem::swap(into, from);
        init_key_info_with_atom(from, info.star_atom);
        return true;
    }
    let groups_in_both = into.groups.len().min(from.groups.len()) as u32;
    for i in 0..groups_in_both as usize {
        merge_groups(&mut into.groups[i], &mut from.groups[i], clobber);
    }
    for group in from.groups.drain(groups_in_both as usize..) {
        into.groups.push(group);
    }
    if from.vmodmap.is_some() && (into.vmodmap.is_none() || clobber) {
        into.vmodmap = from.vmodmap;
    }
    if use_new_field(KEY_FIELD_REPEAT, into.defined, from.defined, clobber) {
        into.repeat = from.repeat;
        into.defined |= KEY_FIELD_REPEAT;
    }
    if from.default_type.is_some() && (into.default_type.is_none() || clobber) {
        into.default_type = from.default_type;
    }
    if use_new_field(KEY_FIELD_GROUPINFO, into.defined, from.defined, clobber) {
        into.out_of_range_pending_group = from.out_of_range_pending_group;
        into.out_of_range_group_policy = from.out_of_range_group_policy;
        into.out_of_range_group_number = from.out_of_range_group_number;
        into.defined |= KEY_FIELD_GROUPINFO;
    }
    if !merge_overlays(ki, into, from) {
        return false;
    }
    init_key_info_with_atom(from, info.star_atom);
    true
}
fn add_key_symbols(ki: &mut XkbKeymapInfo<'_>, info: &mut SymbolsInfo, keyi: &mut KeyInfo) -> bool {
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
    if let Some(&i) = info.key_index.get(&keyi.name) {
        let mut existing = std::mem::take(&mut info.keys[i]);
        let result = merge_keys(ki, info, &mut existing, keyi);
        info.keys[i] = existing;
        return result;
    }
    // Move keyi's data into the keys vec
    let moved = std::mem::take(keyi);
    info.key_index.insert(moved.name, info.keys.len());
    info.keys.push(moved);
    init_key_info_with_atom(keyi, info.star_atom);
    true
}
fn add_mod_map_entry(info: &mut SymbolsInfo, new: &ModMapEntry) {
    let clobber: bool = new.merge != MergeMode::Augment;
    let key = new.target;
    if let Some(&i) = info.modmap_index.get(&key) {
        let old = &mut info.modmaps[i];
        if new.modifier == old.modifier {
            return;
        }
        old.modifier = if clobber { new.modifier } else { old.modifier };
        return;
    }
    info.modmap_index.insert(key, info.modmaps.len());
    info.modmaps.push(*new);
}
fn merge_included_symbols(
    ki: &mut XkbKeymapInfo<'_>,
    into: &mut SymbolsInfo,
    from: &mut SymbolsInfo,
    merge: MergeMode,
) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut into.mods, &from.mods, merge);
    let group_names_in_both = into.group_names.len().min(from.group_names.len());
    for i in 0..group_names_in_both {
        if from.group_names[i] != 0 && !(merge == MergeMode::Augment && into.group_names[i] != 0) {
            into.group_names[i] = from.group_names[i];
        }
    }
    if group_names_in_both < from.group_names.len() {
        for &gn in &from.group_names[group_names_in_both..] {
            into.group_names.push(gn);
        }
    }
    if into.keys.is_empty() {
        std::mem::swap(&mut into.keys, &mut from.keys);
        std::mem::swap(&mut into.key_index, &mut from.key_index);
    } else {
        for keyi in from.keys.iter_mut() {
            keyi.merge = merge;
            if !add_key_symbols(ki, into, keyi) {
                into.error_count += 1;
            }
        }
    }
    if into.modmaps.is_empty() {
        std::mem::swap(&mut into.modmaps, &mut from.modmaps);
        std::mem::swap(&mut into.modmap_index, &mut from.modmap_index);
    } else {
        for mm in from.modmaps.iter_mut() {
            mm.merge = merge;
            add_mod_map_entry(into, mm);
        }
    };
}
fn handle_include_symbols(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    includes: &mut [IncludeStmt],
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    let mut included = SymbolsInfo::new(ki, info.include_depth.wrapping_add(1), &info.mods);
    for stmt in includes.iter() {
        let mut file = include_file!(ki, info, stmt, FileType::Symbols);
        let mut next = SymbolsInfo::new(ki, info.include_depth.wrapping_add(1), &included.mods);
        next.explicit_group = if !stmt.modifier.is_empty() {
            let group = (stmt.modifier.parse::<i32>().unwrap_or(0) - 1) as u32;
            (group < info.max_groups)
                .then_some(group)
                .or(info.explicit_group)
        } else if ki.keymap.num_groups != 0 && next.include_depth == 1 {
            Some(0)
        } else {
            info.explicit_group
        };
        handle_symbols_file(ki, &mut next, &mut file);
        merge_included_symbols(ki, &mut included, &mut next, stmt.merge);
    }
    if let Some(first) = includes.first() {
        merge_included_symbols(ki, info, &mut included, first.merge);
    }
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
    if array_ndx.is_none() {
        for (i, group) in keyi.groups.iter().enumerate() {
            if group.defined & field == 0 {
                *ndx_rtrn = i as u32;
                return true;
            }
        }
        let i = keyi.groups.len() as u32;
        if i >= info.max_groups {
            return false;
        }
        let new_len = keyi.groups.len() + 1;
        keyi.groups.resize_with(new_len, Default::default);
        *ndx_rtrn = (keyi.groups.len() - 1) as u32;
        return true;
    }
    let mut _pending = false;
    if expr_resolve_group(ki, array_ndx.unwrap(), false, ndx_rtrn, &mut _pending)
        != ParseStatus::Success
    {
        return false;
    }
    *ndx_rtrn -= 1;
    if *ndx_rtrn >= keyi.groups.len() as u32 {
        keyi.groups
            .resize_with((*ndx_rtrn + 1) as usize, Default::default);
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
    if matches!(value, ExprKind::EmptyList) {
        groupi.defined |= GROUP_FIELD_SYMS;
        return true;
    }
    if !matches!(
        value,
        ExprKind::KeysymList { .. } | ExprKind::ActionList { .. }
    ) {
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
    if matches!(value, ExprKind::EmptyList) {
        groupi.defined |= GROUP_FIELD_ACTS;
        return true;
    }
    if !matches!(value, ExprKind::ActionList { .. }) {
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
    let mut non_empty_levels: u32 = 0;
    for (level, action_node) in (0_u32..).zip(action_nodes.iter_mut()) {
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
        let leveli = &mut keyi.groups[ndx as usize].levels[level as usize];
        leveli.actions.clear();
        for act_expr in action_vec.iter_mut() {
            let mut to_act: XkbAction = XkbAction::None;
            let r = handle_action_def(
                ki,
                &mut info.default_actions,
                &info.mods,
                act_expr,
                &mut to_act,
            );
            if r == ParseStatus::Fatal {
                return false;
            }
            if r == ParseStatus::Success && !matches!(to_act, XkbAction::None) {
                leveli.actions.push(to_act);
            }
        }
        if !leveli.actions.is_empty() || !leveli.syms.is_empty() {
            non_empty_levels = level.wrapping_add(1);
        }
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
    lookup_entry("true", KEY_REPEAT_YES),
    lookup_entry("yes", KEY_REPEAT_YES),
    lookup_entry("on", KEY_REPEAT_YES),
    lookup_entry("false", KEY_REPEAT_NO),
    lookup_entry("no", KEY_REPEAT_NO),
    lookup_entry("off", KEY_REPEAT_NO),
    lookup_entry("default", KEY_REPEAT_UNDEFINED),
    lookup_entry("", 0),
];
fn expr_resolve_overlay_entry(
    keymap_info: &XkbKeymapInfo<'_>,
    field: &str,
    array_ndx: Option<&ExprKind>,
    expr: &ExprKind,
    overlay_rtrn: &mut u8,
    key_rtrn: &mut u32,
) -> bool {
    if array_ndx.is_some() {
        return false;
    }
    let prefix: usize = 7;
    let suffix = &field[prefix..];
    let len: usize = suffix.len();
    let (val_parsed, parse_count) = super::parser::parse_dec_u64(suffix.as_bytes());
    let raw_overlay: i64 = val_parsed as i64;
    if parse_count != len as i32
        || raw_overlay < 1_i64
        || raw_overlay > keymap_info.features.max_overlays as i64
    {
        return false;
    }
    *overlay_rtrn = (raw_overlay as u8 as i32 - 1) as u8;
    match expr {
        ExprKind::KeyName(key_name_val) => {
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
        ExprKind::Ident(ident_val) => {
            let id: &str = keymap_info.keymap.ctx.atom_text(*ident_val);
            if !id.is_empty() && id.eq_ignore_ascii_case("none") {
                *key_rtrn = XKB_KEYCODE_INVALID;
                return true;
            } else if !id.is_empty() && id.eq_ignore_ascii_case("any") {
                *key_rtrn = XKB_KEYCODE_INVALID;
                *overlay_rtrn = XKB_OVERLAY_INVALID;
                return true;
            }
            false
        }
        _ => false,
    }
}
macro_rules! field_parser {
    ($type:ident, $parse:ident { $($variant:ident => [$($name:literal),+]),+ $(,)? }) => {
        #[derive(Clone, Copy)]
        enum $type { $($variant),+ }
        fn $parse(field: &str) -> Option<$type> {
            $(if [$($name),+].iter().any(|name| field.eq_ignore_ascii_case(name)) {
                return Some($type::$variant);
            })+
            None
        }
    };
}

field_parser!(SymbolsField, parse_symbols_field_exact {
    Type => ["type"], Symbols => ["symbols"], Actions => ["actions"],
    Vmods => ["vmods", "virtualmods", "virtualmodifiers"],
    Locking => ["locking", "lock", "locks"],
    RadioGroup => ["radiogroup", "permanentradiogroup", "allownone"],
    Overlay => ["overlay"],
    Repeat => ["repeating", "repeats", "repeat"], GroupsWrap => ["groupswrap", "wrapgroups"],
    GroupsClamp => ["groupsclamp", "clampgroups"], GroupsRedirect => ["groupsredirect", "redirectgroups"]
});
fn parse_symbols_field(field: &str) -> Option<SymbolsField> {
    parse_symbols_field_exact(field).or_else(|| {
        if field
            .get(..16)
            .is_some_and(|s| s.eq_ignore_ascii_case("permanentoverlay"))
        {
            Some(SymbolsField::Locking)
        } else if field
            .get(..7)
            .is_some_and(|s| s.eq_ignore_ascii_case("overlay"))
        {
            Some(SymbolsField::Overlay)
        } else {
            None
        }
    })
}

fn add_pending_computation(info: &mut XkbKeymapInfo<'_>, expr: Option<ExprKind>) -> u32 {
    let index = info.pending_computations.len() as u32;
    info.pending_computations.push(PendingComputation {
        expr,
        computed: false,
        value: 0,
    });
    index
}

fn set_symbols_field(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<ExprKind>,
) -> bool {
    let mapped_field = match parse_symbols_field(field) {
        Some(f) => f,
        None => return ki.strict & PARSER_NO_UNKNOWN_KEY_FIELDS == 0,
    };

    match mapped_field {
        SymbolsField::Type => {
            let mut ndx: u32 = 0;
            let val = some_or_false!(expr_resolve_string(value_opt.as_ref().unwrap()));
            if let Some(array_ndx) = array_ndx {
                let mut _pending = false;
                if expr_resolve_group(ki, array_ndx, false, &mut ndx, &mut _pending)
                    != ParseStatus::Success
                {
                    return false;
                }
                ndx -= 1;
                if ndx >= keyi.groups.len() as u32 {
                    keyi.groups
                        .resize_with((ndx as usize) + 1, Default::default);
                }
                keyi.groups[ndx as usize].type_0 = Some(val);
            } else {
                keyi.default_type = Some(val);
            }
        }
        SymbolsField::Symbols => {
            return add_symbols_to_key(ki, info, keyi, array_ndx, value_opt.as_ref().unwrap());
        }
        SymbolsField::Actions => {
            return add_actions_to_key(ki, info, keyi, array_ndx, value_opt.as_mut().unwrap());
        }
        SymbolsField::Vmods => {
            let val = value_opt.as_ref().unwrap();
            let mask = some_or_false!(expr_resolve_mod_mask(
                &ki.keymap.ctx,
                val,
                MOD_VIRT,
                &info.mods
            ));
            keyi.vmodmap = Some(mask);
        }
        SymbolsField::Locking | SymbolsField::RadioGroup => {}
        SymbolsField::Overlay => {
            let mut overlay: u8 = XKB_OVERLAY_INVALID;
            let mut key: u32 = XKB_KEYCODE_INVALID;
            if !expr_resolve_overlay_entry(
                ki,
                field,
                array_ndx,
                value_opt.as_ref().unwrap(),
                &mut overlay,
                &mut key,
            ) {
                return false;
            }
            if overlay == XKB_OVERLAY_INVALID
                || key != XKB_KEYCODE_INVALID
                    && ki.keymap.get_key(key).is_some_and(|k| k.name == keyi.name)
            {
                return true;
            }
            if ki.features.overlapping_overlays {
                if overlays_insert(keyi, overlay, key) {
                    keyi.defined |= KEY_FIELD_OVERLAY;
                }
            } else {
                let has_none = keyi.overlays.iter().all(|o| o.is_none());
                if has_none || keyi.overlays_clear {
                    keyi.overlays[overlay as usize] = Some(key);
                    keyi.overlays_clear = key == XKB_KEYCODE_INVALID;
                    keyi.defined |= KEY_FIELD_OVERLAY;
                } else if keyi.overlays[overlay as usize].is_none() && key != XKB_KEYCODE_INVALID {
                    return ki.strict & PARSER_NO_FIELD_VALUE_MISMATCH == 0;
                }
            }
        }
        SymbolsField::Repeat => {
            let Some(val_0) =
                expr_resolve_enum(&ki.keymap.ctx, value_opt.as_ref().unwrap(), &REPEAT_ENTRIES)
            else {
                return false;
            };
            keyi.repeat = val_0;
            keyi.defined |= KEY_FIELD_REPEAT;
        }
        SymbolsField::GroupsWrap | SymbolsField::GroupsClamp => {
            let set = some_or_false!(expr_resolve_boolean(
                &ki.keymap.ctx,
                value_opt.as_ref().unwrap()
            ));
            let wrap = matches!(mapped_field, SymbolsField::GroupsWrap);
            keyi.out_of_range_group_policy = if set == wrap {
                XKB_LAYOUT_OUT_OF_RANGE_WRAP
            } else {
                XKB_LAYOUT_OUT_OF_RANGE_CLAMP
            };
            keyi.defined |= KEY_FIELD_GROUPINFO;
        }
        SymbolsField::GroupsRedirect => {
            let mut grp: u32 = 0;
            let mut pending: bool = false;
            if expr_resolve_group(
                ki,
                value_opt.as_ref().unwrap(),
                false,
                &mut grp,
                &mut pending,
            ) != ParseStatus::Success
                && !pending
            {
                return false;
            }
            if pending {
                keyi.out_of_range_pending_group = true;
                keyi.out_of_range_group_number = add_pending_computation(ki, value_opt.take());
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
    merge: MergeMode,
) -> bool {
    let array_ndx = match array_ndx {
        Some(a) => a,
        None => {
            return false;
        }
    };
    let mut group: u32 = 0;
    let mut _pending: bool = false;
    if { expr_resolve_group(ki, array_ndx, false, &mut group, &mut _pending) }
        != ParseStatus::Success
    {
        return false;
    }
    let mut name = some_or_false!(expr_resolve_string(value));
    let group_to_use = match info.explicit_group {
        None => group.wrapping_sub(1),
        Some(explicit_group) if group.wrapping_sub(1) == 0 => explicit_group,
        Some(_) => return false,
    };
    if group_to_use >= info.group_names.len() as u32 {
        info.group_names.resize((group_to_use as usize) + 1, 0_u32);
    } else {
        let old_name: u32 = info.group_names[group_to_use as usize];
        if old_name != XKB_ATOM_NONE && old_name != name {
            name = if merge != MergeMode::Augment {
                name
            } else {
                old_name
            };
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
    let ret: bool;
    let lhs = some_or_false!(expr_resolve_lhs(stmt.name.as_ref().unwrap()));
    let elem_atom = lhs.element;
    let field_atom = lhs.field;
    let array_ndx_opt = lhs.index;
    let elem = ki.keymap.ctx.atom_text(elem_atom).to_owned();
    let field = ki.keymap.ctx.atom_text(field_atom).to_owned();
    if !elem.is_empty() && elem.eq_ignore_ascii_case("key") {
        let mut temp: KeyInfo = {
            KeyInfo {
                out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
                ..Default::default()
            }
        };
        init_key_info_with_atom(&mut temp, ki.keymap.ctx.atom_intern(b"*"));
        temp.merge = if temp.merge == MergeMode::Replace {
            MergeMode::Override
        } else {
            stmt.merge
        };
        ret = set_symbols_field(ki, info, &mut temp, &field, array_ndx_opt, &mut stmt.value);
        let mut dk = std::mem::take(&mut info.default_key);
        merge_keys(ki, info, &mut dk, &mut temp);
        info.default_key = dk;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("name") || field.eq_ignore_ascii_case("groupname"))
    {
        ret = set_group_name(
            ki,
            info,
            array_ndx_opt,
            stmt.value.as_ref().unwrap(),
            stmt.merge,
        );
    } else if elem.is_empty()
        && [
            "groupswrap",
            "wrapgroups",
            "groupsclamp",
            "clampgroups",
            "groupsredirect",
            "redirectgroups",
            "allownone",
        ]
        .iter()
        .any(|name| field.eq_ignore_ascii_case(name))
    {
        ret = true;
    } else if !elem.is_empty() {
        ret = {
            set_default_action_field(
                ki,
                &mut info.default_actions,
                &mut info.mods,
                &elem,
                &field,
                array_ndx_opt,
                &mut stmt.value,
                stmt.merge,
            ) != ParseStatus::Fatal
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
        if let Some(name) = def.name.as_ref() {
            if let Some(lhs) = expr_resolve_lhs(name) {
                array_ndx_opt = lhs.index;
                let elem = ki.keymap.ctx.atom_text(lhs.element);
                field_owned = ki.keymap.ctx.atom_text(lhs.field).to_owned();
                field = &field_owned;
                if !elem.is_empty() {
                    ok = false;
                }
            } else {
                field_owned = String::new();
                field = &field_owned;
                ok = false;
            }
        } else if def.value.is_none() || !is_action_list_value(def.value.as_ref().unwrap()) {
            field = "symbols";
        } else {
            field = "actions";
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
fn set_explicit_group(info: &SymbolsInfo, keyi: &mut KeyInfo) {
    let eg = match info.explicit_group {
        None => return,
        Some(v) => v,
    };
    if !keyi.groups.is_empty() {
        for group in keyi.groups[1..].iter_mut() {
            if group.defined != 0 || group.type_0.is_some() {
                *group = GroupInfo::default();
            }
        }
    }

    keyi.groups.resize_with((eg as usize) + 1, Default::default);
    if eg > 0 {
        keyi.groups[eg as usize] = std::mem::take(&mut keyi.groups[0]);
    }
}
fn handle_symbols_def(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    stmt: &mut NamedVarDef,
) -> bool {
    // Clone scalar fields from default_key, deep-copy groups
    let dk = &info.default_key;
    let mut keyi = dk.clone();
    keyi.merge = stmt.merge;
    keyi.name = stmt.name;
    if handle_symbols_body(ki, info, &mut stmt.body, &mut keyi) {
        set_explicit_group(info, &mut keyi);
        if add_key_symbols(ki, info, &mut keyi) {
            return true;
        }
    }
    info.error_count += 1;
    false
}
fn handle_mod_map_def(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsInfo,
    def: &mut ModMapDef,
) -> bool {
    let modifier_name: &str = ki.keymap.ctx.atom_text(def.modifier);
    let ndx = if modifier_name.eq_ignore_ascii_case("none") {
        XKB_MOD_NONE
    } else {
        match xkb_mod_name_to_index(&info.mods, def.modifier, MOD_REAL) {
            Some(n) => n,
            None => return false,
        }
    };
    for key in def.keys.iter() {
        let target = if let ExprKind::KeyName(kn) = key {
            Some(ModMapTarget::Key(*kn))
        } else if let ExprKind::KeySym(ks) = key {
            (*ks != XKB_KEY_NO_SYMBOL).then_some(ModMapTarget::Symbol(*ks))
        } else {
            None
        };
        if let Some(target) = target {
            add_mod_map_entry(
                info,
                &ModMapEntry {
                    merge: def.merge,
                    modifier: ndx,
                    target,
                },
            );
        }
    }
    true
}
fn handle_symbols_file(ki: &mut XkbKeymapInfo<'_>, info: &mut SymbolsInfo, file: &mut XkbFile) {
    {
        let mut ok: bool;
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
                Statement::Unknown => {
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
fn find_automatic_type(ctx: &mut XkbContext, groupi: &GroupInfo) -> u32 {
    let width: u32 = groupi.levels.len() as u32;
    if width == 1 || width == 0 {
        return ctx.atom_intern(b"ONE_LEVEL");
    }
    let sym0: u32 = if groupi.levels[0].syms.is_empty() {
        XKB_KEY_NO_SYMBOL
    } else {
        groupi.levels[0].syms[0]
    };
    let sym1: u32 = if groupi.levels[1].syms.is_empty() {
        XKB_KEY_NO_SYMBOL
    } else {
        groupi.levels[1].syms[0]
    };
    if width == 2_u32 {
        if xkb_keysym_is_lower(sym0) && xkb_keysym_is_upper_or_title(sym1) {
            return ctx.atom_intern(b"ALPHABETIC");
        }
        if xkb_keysym_is_keypad(sym0) || xkb_keysym_is_keypad(sym1) {
            return ctx.atom_intern(b"KEYPAD");
        }
        return ctx.atom_intern(b"TWO_LEVEL");
    }
    if width <= 4_u32 {
        if xkb_keysym_is_lower(sym0) && xkb_keysym_is_upper_or_title(sym1) {
            let sym2: u32 = if groupi.levels[2].syms.is_empty() {
                XKB_KEY_NO_SYMBOL
            } else {
                groupi.levels[2].syms[0]
            };
            let sym3: u32 = if width == 4_u32 {
                if groupi.levels[3].syms.is_empty() {
                    XKB_KEY_NO_SYMBOL
                } else {
                    groupi.levels[3].syms[0]
                }
            } else {
                XKB_KEY_NO_SYMBOL
            };
            if xkb_keysym_is_lower(sym2) && xkb_keysym_is_upper_or_title(sym3) {
                return ctx.atom_intern(b"FOUR_LEVEL_ALPHABETIC");
            }
            return ctx.atom_intern(b"FOUR_LEVEL_SEMIALPHABETIC");
        }
        if xkb_keysym_is_keypad(sym0) || xkb_keysym_is_keypad(sym1) {
            return ctx.atom_intern(b"FOUR_LEVEL_KEYPAD");
        }
        return ctx.atom_intern(b"FOUR_LEVEL");
    }
    XKB_ATOM_NONE
}
fn find_type_for_group(
    keymap: &mut XkbKeymap,
    keyi: &mut KeyInfo,
    group: u32,
    type_map: &HashMap<u32, u32>,
) -> u32 {
    let groupi = &keyi.groups[group as usize];
    let mut type_name: u32 = groupi.type_0.unwrap_or(XKB_ATOM_NONE);
    if type_name == XKB_ATOM_NONE {
        if let Some(default_type) = keyi.default_type {
            type_name = default_type;
        } else {
            type_name = find_automatic_type(&mut keymap.ctx, groupi);
        }
    }
    if type_name != XKB_ATOM_NONE {
        if let Some(&idx) = type_map.get(&type_name) {
            return idx;
        }
    }
    0
}
fn copy_symbols_def_to_keymap(
    keymap: &mut XkbKeymap,
    keyi: &mut KeyInfo,
    type_map: &HashMap<u32, u32>,
) -> bool {
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
        None => return false,
    };

    keymap.keys[key_idx].num_groups = 0;
    if !keyi.groups.is_empty() {
        for (idx, groupi) in keyi.groups.iter().enumerate() {
            let has_explicit_type = keyi.default_type.is_some() || groupi.type_0.is_some();
            if !groupi.levels.is_empty() || has_explicit_type {
                keymap.keys[key_idx].num_groups = (idx as u32) + 1;
            }
        }
    }

    if keymap.keys[key_idx].num_groups == 0 {
        if keyi.defined == 0 && keyi.default_type.is_none() {
            return false;
        }
    } else {
        let num_groups = keymap.keys[key_idx].num_groups as usize;
        keyi.groups.resize_with(num_groups, Default::default);

        for i in 1..keyi.groups.len() {
            if keyi.groups[i].defined == 0 && keyi.groups[i].type_0.is_none() {
                keyi.groups[i] = keyi.groups[0].clone();
            }
        }

        keymap.keys[key_idx].groups = (0..num_groups).map(|_| XkbGroup::default()).collect();

        for i in 0..keyi.groups.len() as u32 {
            let type_idx = find_type_for_group(keymap, keyi, i, type_map);

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

            let need_levels = keymap.types[type_idx as usize].num_levels as usize;
            keyi.groups[i as usize]
                .levels
                .resize_with(need_levels, Default::default);

            keymap.keys[key_idx].groups[i as usize].type_idx = type_idx;
        }

        for i in 0..keyi.groups.len() {
            let groupi = &mut keyi.groups[i];
            for li in 0..groupi.levels.len() {
                let leveli = &mut groupi.levels[li];
                if leveli.syms.len() > 1 {
                    let has_upper = leveli.syms.iter().any(|&s| xkb_keysym_to_upper(s) != s);
                    if has_upper {
                        let orig = leveli.syms.len();
                        leveli.syms.reserve(orig);
                        for i in 0..orig {
                            leveli.syms.push(xkb_keysym_to_upper(leveli.syms[i]));
                        }
                    }
                }
            }

            keymap.keys[key_idx].groups[i].levels = std::mem::take(&mut groupi.levels);

            if groupi.defined & GROUP_FIELD_ACTS != 0 {
                keymap.keys[key_idx].groups[i].explicit_actions = true;
            }
        }

        keymap.keys[key_idx].out_of_range_pending_group = keyi.out_of_range_pending_group;
        keymap.keys[key_idx].out_of_range_group_number = keyi.out_of_range_group_number;
        keymap.keys[key_idx].out_of_range_group_policy = keyi.out_of_range_group_policy;
    }

    if let Some(vmodmap) = keyi.vmodmap {
        keymap.keys[key_idx].vmodmap = vmodmap;
        keymap.keys[key_idx].explicit_vmodmap = true;
    }

    if keyi.repeat != KEY_REPEAT_UNDEFINED {
        keymap.keys[key_idx].repeats = keyi.repeat == KEY_REPEAT_YES;
        keymap.keys[key_idx].explicit_repeat = true;
    }

    true
}
fn copy_symbols_to_keymap(keymap: &mut XkbKeymap, info: &mut SymbolsInfo) {
    let type_map: HashMap<u32, u32> = keymap
        .types
        .iter()
        .enumerate()
        .map(|(i, t)| (t.name, i as u32))
        .collect();
    keymap.mods = info.mods;
    keymap.group_names = std::mem::take(&mut info.group_names);
    let mut keys = std::mem::take(&mut info.keys);
    for keyi in keys.iter_mut() {
        if !copy_symbols_def_to_keymap(keymap, keyi, &type_map) {
            info.error_count += 1;
        }
    }
    info.keys = keys;
    let start = if keymap.num_keys_low == 0 {
        0_usize
    } else {
        keymap.min_key_code as usize
    };
    let mut sym_to_key: HashMap<u32, usize> = HashMap::new();
    for ki in start..keymap.num_keys.min(keymap.keys.len() as u32) as usize {
        let key = &keymap.keys[ki];
        for gi in 0..key.num_groups.min(key.groups.len() as u32) {
            let g = &key.groups[gi as usize];
            let num_levels = keymap
                .types
                .get(g.type_idx as usize)
                .map_or(0, |t| t.num_levels);
            for li in 0..num_levels.min(g.levels.len() as u32) {
                for &sym in &g.levels[li as usize].syms {
                    sym_to_key.entry(sym).or_insert(ki);
                }
            }
        }
    }
    for modmap in &info.modmaps {
        match modmap.target {
            ModMapTarget::Symbol(sym) => {
                if let Some(&ki) = sym_to_key.get(&sym) {
                    if modmap.modifier != XKB_MOD_NONE {
                        keymap.keys[ki].modmap |= 1_u32 << modmap.modifier;
                    }
                } else {
                    info.error_count += 1;
                }
            }
            ModMapTarget::Key(name) => {
                if let Some(key) = keymap.key_by_name_mut(name, true) {
                    if modmap.modifier != XKB_MOD_NONE {
                        key.modmap |= 1_u32 << modmap.modifier;
                    }
                } else {
                    info.error_count += 1;
                }
            }
        }
    }
}
pub(crate) fn compile_symbols(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut info = SymbolsInfo::new(keymap_info, 0, &mods);
    if let Some(file) = file {
        handle_symbols_file(keymap_info, &mut info, file);
    }
    if info.error_count != 0 {
        return false;
    }
    copy_symbols_to_keymap(keymap_info.keymap, &mut info);
    true
}
use super::keysym::xkb_keysym_to_upper;
use super::parser::*;
#[derive(Clone, Default)]
pub(crate) struct CompatInfo {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) default_interp: SymInterpInfo,
    pub(crate) interps: Vec<SymInterpInfo>,
    pub(crate) default_led: LedInfo,
    pub(crate) leds: [LedInfo; 32],
    pub(crate) num_leds: u32,
    pub(crate) default_actions: ActionsInfo,
    pub(crate) mods: XkbModSet,
    pub(crate) interp_index: HashMap<(u32, u32, u32), usize>,
    pub(crate) led_index: HashMap<u32, u32>,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct LedInfo {
    pub(crate) defined: u32,
    pub(crate) merge: MergeMode,
    pub(crate) led: XkbLed,
}
pub(crate) const LED_FIELD_CTRLS: u32 = 4;
pub(crate) const LED_FIELD_GROUPS: u32 = 2;
pub(crate) const LED_FIELD_MODS: u32 = 1;
// C2Rust_Unnamed_18 removed: replaced by Vec<SymInterpInfo>
#[derive(Clone, Default)]
pub(crate) struct SymInterpInfo {
    pub(crate) defined: u32,
    pub(crate) merge: MergeMode,
    pub(crate) interp: XkbSymInterpret,
}
pub(crate) const SI_FIELD_LEVEL_ONE_ONLY: u32 = 8;
pub(crate) const SI_FIELD_AUTO_REPEAT: u32 = 4;
pub(crate) const SI_FIELD_ACTION: u32 = 2;
pub(crate) const SI_FIELD_VIRTUAL_MOD: u32 = 1;
// C2Rust_Unnamed_19 removed: replaced by Vec<XkbSymInterpret>
#[inline]
fn compat_info(include_depth: u32, mods: &XkbModSet) -> CompatInfo {
    let mut info = CompatInfo {
        include_depth,
        ..Default::default()
    };
    init_actions_info(&mut info.default_actions);
    init_vmods(&mut info.mods, mods, include_depth > 0);
    info.default_interp.interp.virtual_mod = XKB_MOD_INVALID;
    info
}

fn merge_interp(old: &mut SymInterpInfo, new: &mut SymInterpInfo) {
    let clobber: bool = new.merge != MergeMode::Augment;
    if new.merge == MergeMode::Replace {
        *old = new.clone();
        return;
    }
    if use_new_field(SI_FIELD_VIRTUAL_MOD, old.defined, new.defined, clobber) {
        old.interp.virtual_mod = new.interp.virtual_mod;
        old.defined |= SI_FIELD_VIRTUAL_MOD;
    }
    if use_new_field(SI_FIELD_ACTION, old.defined, new.defined, clobber) {
        old.interp.actions = std::mem::take(&mut new.interp.actions);
        old.defined |= SI_FIELD_ACTION;
    }
    if use_new_field(SI_FIELD_AUTO_REPEAT, old.defined, new.defined, clobber) {
        old.interp.repeat = new.interp.repeat;
        old.defined |= SI_FIELD_AUTO_REPEAT;
    }
    if use_new_field(SI_FIELD_LEVEL_ONE_ONLY, old.defined, new.defined, clobber) {
        old.interp.level_one_only = new.interp.level_one_only;
        old.defined |= SI_FIELD_LEVEL_ONE_ONLY;
    }
}
fn add_interp(info: &mut CompatInfo, new: &mut SymInterpInfo) {
    let key = (new.interp.sym, new.interp.mods, new.interp.match_0);
    if let Some(&idx) = info.interp_index.get(&key) {
        let mut old = info.interps[idx].clone();
        merge_interp(&mut old, new);
        info.interps[idx] = old;
        return;
    }
    info.interp_index.insert(key, info.interps.len());
    info.interps.push(new.clone());
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
    if let ExprKind::Action { name, args } = expr {
        let pred_txt: &str = ki.keymap.ctx.atom_text(*name);
        if args.len() != 1 {
            return false;
        }
        let pred = some_or_false!(lookup_string(&SYM_INTERPRET_MATCH_MASK_NAMES, pred_txt));
        *pred_rtrn = pred;
        resolve_expr = &args[0];
    } else if let ExprKind::Ident(ident_val) = expr {
        let pred_txt_0: &str = ki.keymap.ctx.atom_text(*ident_val);
        if !pred_txt_0.is_empty() && pred_txt_0.eq_ignore_ascii_case("any") {
            *pred_rtrn = MATCH_ANY;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        resolve_expr = expr;
    } else {
        resolve_expr = expr;
    }
    let mods = some_or_false!(expr_resolve_mod_mask(
        &ki.keymap.ctx,
        resolve_expr,
        MOD_REAL,
        &info.mods
    ));
    *mods_rtrn = mods;
    true
}

fn merge_led_map(old: &mut LedInfo, new: &mut LedInfo) {
    let clobber: bool = new.merge != MergeMode::Augment;
    if old.led.mods.mods == new.led.mods.mods
        && old.led.pending_groups == new.led.pending_groups
        && old.led.groups == new.led.groups
        && old.led.ctrls == new.led.ctrls
        && old.led.which_mods == new.led.which_mods
        && old.led.which_groups as i32 == new.led.which_groups as i32
    {
        old.defined |= new.defined;
        return;
    }
    if new.merge == MergeMode::Replace {
        *old = *new;
        return;
    }
    if use_new_field(LED_FIELD_MODS, old.defined, new.defined, clobber) {
        old.led.which_mods = new.led.which_mods;
        old.led.mods = new.led.mods;
        old.defined |= LED_FIELD_MODS;
    }
    if use_new_field(LED_FIELD_GROUPS, old.defined, new.defined, clobber) {
        old.led.which_groups = new.led.which_groups;
        old.led.groups = new.led.groups;
        old.led.pending_groups = new.led.pending_groups;
        old.defined |= LED_FIELD_GROUPS;
    }
    if use_new_field(LED_FIELD_CTRLS, old.defined, new.defined, clobber) {
        old.led.ctrls = new.led.ctrls;
        old.defined |= LED_FIELD_CTRLS;
    }
}
fn add_led_map(info: &mut CompatInfo, new: &mut LedInfo) -> bool {
    if let Some(&i) = info.led_index.get(&new.led.name) {
        let mut old = info.leds[i as usize];
        merge_led_map(&mut old, new);
        info.leds[i as usize] = old;
        return true;
    }
    if info.num_leds >= XKB_MAX_LEDS {
        return false;
    }
    info.led_index.insert(new.led.name, info.num_leds);
    info.leds[info.num_leds as usize] = *new;
    info.num_leds += 1;
    true
}
fn merge_included_compat_maps(into: &mut CompatInfo, from: &mut CompatInfo, merge: MergeMode) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut into.mods, &from.mods, merge);
    if into.interps.is_empty() {
        into.interps = std::mem::take(&mut from.interps);
        into.interp_index = std::mem::take(&mut from.interp_index);
    } else {
        for interp in from.interps.iter_mut() {
            interp.merge = merge;
            add_interp(into, interp);
        }
    }
    if into.num_leds == 0 {
        let n = from.num_leds as usize;
        into.leds[..n].copy_from_slice(&from.leds[..n]);
        into.num_leds = from.num_leds;
        from.num_leds = 0;
        into.led_index = std::mem::take(&mut from.led_index);
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
    includes: &mut [IncludeStmt],
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    let mut included = compat_info(info.include_depth.wrapping_add(1), &info.mods);
    for stmt in includes.iter() {
        let mut file = include_file!(ki, info, stmt, FileType::Compat);
        let mut next = compat_info(info.include_depth.wrapping_add(1), &included.mods);
        next.default_interp = info.default_interp.clone();
        next.default_led = info.default_led;
        handle_compat_map_file(ki, &mut next, &mut file);
        merge_included_compat_maps(&mut included, &mut next, stmt.merge);
    }
    if let Some(first) = includes.first() {
        merge_included_compat_maps(info, &mut included, first.merge);
    }
    info.error_count == 0
}
field_parser!(InterpField, parse_interp_field {
    Action => ["action"], VirtualModifier => ["virtualmodifier", "virtualmod"],
    Repeat => ["repeat"], Locking => ["locking"], UseModMap => ["usemodmap", "usemodmapmods"]
});

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
    if array_ndx.is_some() && !matches!(mapped_field, InterpField::Locking) {
        return false;
    }

    match mapped_field {
        InterpField::Action => {
            si.interp.actions.clear();
            let action_vec = if let ExprKind::ActionList { actions } = value {
                actions.as_mut_slice()
            } else {
                std::slice::from_mut(value)
            };
            if action_vec.len() > MAX_ACTIONS_PER_LEVEL as usize {
                return false;
            }
            for act_expr in action_vec {
                let mut action = XkbAction::None;
                match handle_action_def(
                    ki,
                    &mut info.default_actions,
                    &info.mods,
                    act_expr,
                    &mut action,
                ) {
                    ParseStatus::Fatal => return false,
                    ParseStatus::Recoverable => action = XkbAction::None,
                    _ => {}
                }
                if !matches!(action, XkbAction::None) {
                    si.interp.actions.push(action);
                }
            }
            si.defined |= SI_FIELD_ACTION;
        }
        InterpField::VirtualModifier => {
            let ndx = some_or_false!(expr_resolve_mod(value, MOD_VIRT, &info.mods));
            si.interp.virtual_mod = ndx;
            si.defined |= SI_FIELD_VIRTUAL_MOD;
        }
        InterpField::Repeat => {
            let set = some_or_false!(expr_resolve_boolean(&ki.keymap.ctx, value));
            si.interp.repeat = set;
            si.defined |= SI_FIELD_AUTO_REPEAT;
        }
        InterpField::Locking => {}
        InterpField::UseModMap => {
            let val = some_or_false!(expr_resolve_enum(
                &ki.keymap.ctx,
                value,
                &USE_MOD_MAP_VALUE_NAMES
            ));
            si.interp.level_one_only = val != 0;
            si.defined |= SI_FIELD_LEVEL_ONE_ONLY;
        }
    }
    true
}
field_parser!(LedMapField, parse_led_map_field {
    Modifiers => ["modifiers", "mods"], Groups => ["groups"], Controls => ["controls", "ctrls"],
    AllowExplicit => ["allowexplicit", "driveskbd", "driveskeyboard", "leddriveskbd",
        "leddriveskeyboard", "indicatordriveskbd", "indicatordriveskeyboard"],
    WhichMods => ["whichmodstate", "whichmodifierstate"], WhichGroups => ["whichgroupstate"],
    Index => ["index"]
});

fn set_led_map_field(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    ledi: &mut LedInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<ExprKind>,
) -> bool {
    let value: &ExprKind = value_opt.as_ref().unwrap();
    let mapped_field = match parse_led_map_field(field) {
        Some(f) => f,
        None => return ki.strict & PARSER_NO_UNKNOWN_LED_FIELDS == 0,
    };
    if array_ndx.is_some()
        && !matches!(
            mapped_field,
            LedMapField::AllowExplicit | LedMapField::Index
        )
    {
        return false;
    }

    match mapped_field {
        LedMapField::Modifiers => {
            let mods = some_or_false!(expr_resolve_mod_mask(
                &ki.keymap.ctx,
                value,
                MOD_BOTH,
                &info.mods
            ));
            ledi.led.mods.mods = mods;
            ledi.defined |= LED_FIELD_MODS;
        }
        LedMapField::Groups => {
            let mut mask: u32 = 0;
            let mut pending: bool = false;
            if !expr_resolve_group_mask(ki, value, &mut mask, &mut pending) {
                if pending {
                    ledi.led.pending_groups = true;
                    mask = add_pending_computation(ki, value_opt.take());
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
            let offset: u8 = ki.features.controls_name_offset;
            let Some(mask_0) =
                expr_resolve_mask(&ki.keymap.ctx, value, &CTRL_MASK_NAMES[offset as usize..])
            else {
                return false;
            };
            ledi.led.ctrls = ControlsFlags::from_bits_retain(mask_0);
            ledi.defined |= LED_FIELD_CTRLS;
        }
        LedMapField::AllowExplicit | LedMapField::Index => {}
        LedMapField::WhichMods => {
            let mask_1 = some_or_false!(expr_resolve_mask(
                &ki.keymap.ctx,
                value,
                &MOD_COMPONENT_MASK_NAMES
            ));
            ledi.led.which_mods = mask_1;
        }
        LedMapField::WhichGroups => {
            let Some(mask_2) =
                expr_resolve_mask(&ki.keymap.ctx, value, &GROUP_COMPONENT_MASK_NAMES)
            else {
                return false;
            };
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
    let ret: bool;
    if let Some(lhs) = expr_resolve_lhs(stmt.name.as_ref().unwrap()) {
        let elem_atom = lhs.element;
        let field_atom = lhs.field;
        let ndx = lhs.index;
        let elem = ki.keymap.ctx.atom_text(elem_atom).to_owned();
        let field = ki.keymap.ctx.atom_text(field_atom).to_owned();
        if !elem.is_empty() && elem.eq_ignore_ascii_case("interpret") {
            let mut temp: SymInterpInfo = SymInterpInfo::default();
            temp.merge = if temp.merge == MergeMode::Replace {
                MergeMode::Override
            } else {
                stmt.merge
            };
            let value_ref = stmt.value.as_mut().unwrap();
            ret = set_interp_field(info, ki, &mut temp, &field, ndx, value_ref);
            if ret {
                let mut default = info.default_interp.clone();
                merge_interp(&mut default, &mut temp);
                info.default_interp = default;
            }
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("indicator") {
            let mut temp_0: LedInfo = LedInfo::default();
            temp_0.merge = if temp_0.merge == MergeMode::Replace {
                MergeMode::Override
            } else {
                stmt.merge
            };
            ret = set_led_map_field(info, ki, &mut temp_0, &field, ndx, &mut stmt.value);
            if ret {
                let mut default = info.default_led;
                merge_led_map(&mut default, &mut temp_0);
                info.default_led = default;
            }
        } else if !elem.is_empty() {
            ret = set_default_action_field(
                ki,
                &mut info.default_actions,
                &mut info.mods,
                &elem,
                &field,
                ndx,
                &mut stmt.value,
                stmt.merge,
            ) != ParseStatus::Fatal;
        } else {
            return ki.strict & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS == 0;
        }
    } else {
        ret = false;
    }
    ret
}
fn handle_interp_def(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    def: &mut InterpDef,
) -> bool {
    let mut pred: u32 = MATCH_NONE;
    let mut mods: u32 = 0;
    if !resolve_state_and_predicate(def.match_0.as_ref(), &mut pred, &mut mods, info, ki) {
        return false;
    }
    let mut si: SymInterpInfo = info.default_interp.clone();
    si.merge = def.merge;
    si.interp.sym = def.sym;
    si.interp.match_0 = pred;
    si.interp.mods = mods;
    for body_def in &mut def.def {
        let Some(lhs) = expr_resolve_lhs(body_def.name.as_ref().unwrap()) else {
            info.error_count += 1;
            return false;
        };
        let elem = ki.keymap.ctx.atom_text(lhs.element).to_owned();
        let field = ki.keymap.ctx.atom_text(lhs.field).to_owned();
        if !elem.is_empty() {
            info.error_count += 1;
            return false;
        }
        let value_ref = body_def.value.as_mut().unwrap();
        if !set_interp_field(info, ki, &mut si, &field, lhs.index, value_ref) {
            info.error_count += 1;
            return false;
        }
    }
    add_interp(info, &mut si);
    true
}
fn handle_led_map_def(
    info: &mut CompatInfo,
    ki: &mut XkbKeymapInfo<'_>,
    def: &mut NamedVarDef,
) -> bool {
    let mut ledi: LedInfo = info.default_led;
    ledi.merge = def.merge;
    ledi.led.name = def.name;
    let mut ok: bool = true;
    for var in def.body.iter_mut() {
        if let Some(lhs) = expr_resolve_lhs(var.name.as_ref().unwrap()) {
            let elem = ki.keymap.ctx.atom_text(lhs.element).to_owned();
            let field = ki.keymap.ctx.atom_text(lhs.field).to_owned();
            if !elem.is_empty()
                || !set_led_map_field(info, ki, &mut ledi, &field, lhs.index, &mut var.value)
            {
                ok = false;
            }
        } else {
            ok = false;
        }
    }
    ok && add_led_map(info, &mut ledi)
}
fn handle_compat_map_file(ki: &mut XkbKeymapInfo<'_>, info: &mut CompatInfo, file: &mut XkbFile) {
    {
        let mut ok: bool;
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_compat_map(ki, info, incl);
                }
                Statement::Interp(ip) => {
                    ok = handle_interp_def(info, ki, ip);
                }
                Statement::GroupCompat => {
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
                Statement::Unknown => {
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
fn copy_led_map_defs_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut CompatInfo) {
    for idx in 0..info.num_leds {
        let ledi_led = info.leds[idx as usize].led;
        let leds = &ki.keymap.leds[..ki.keymap.num_leds as usize];
        let slot = leds
            .iter()
            .position(|led| led.name == ledi_led.name)
            .or_else(|| leds.iter().position(|led| led.name == XKB_ATOM_NONE))
            .or_else(|| {
                (ki.keymap.num_leds < XKB_MAX_LEDS).then(|| {
                    let slot = ki.keymap.num_leds as usize;
                    ki.keymap.num_leds += 1;
                    slot
                })
            });
        if let Some(slot) = slot {
            ki.keymap.leds[slot] = ledi_led;
            let led = &mut ki.keymap.leds[slot];
            if led.which_groups == 0 && (led.groups != 0 || led.pending_groups) {
                led.which_groups = XKB_STATE_LAYOUT_EFFECTIVE;
            }
            if led.which_mods == 0 && led.mods.mods != 0 {
                led.which_mods = XKB_STATE_MODS_EFFECTIVE;
            }
        }
    }
}
fn copy_compat_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut CompatInfo) {
    let mut interps = Vec::with_capacity(info.interps.len());
    for need_symbol in [true, false] {
        for pred in [
            MATCH_EXACTLY,
            MATCH_ALL,
            MATCH_NONE,
            MATCH_ANY,
            MATCH_ANY_OR_NONE,
        ] {
            interps.extend(
                info.interps
                    .iter()
                    .filter(|si| {
                        si.interp.match_0 == pred
                            && (si.interp.sym != XKB_KEY_NO_SYMBOL) == need_symbol
                    })
                    .map(|si| si.interp.clone()),
            );
        }
    }
    ki.keymap.mods = info.mods;
    if !interps.is_empty() {
        ki.sym_interprets = interps;
    }
    copy_led_map_defs_to_keymap(ki, info);
}
pub(crate) fn compile_compat_map(file: Option<&mut XkbFile>, ki: &mut XkbKeymapInfo<'_>) -> bool {
    let mods = ki.keymap.mods;
    let mut info = compat_info(0, &mods);
    if let Some(file) = file {
        handle_compat_map_file(ki, &mut info, file);
    }
    if info.error_count != 0 {
        return false;
    }
    copy_compat_to_keymap(ki, &mut info);
    true
}
#[derive(Default)]
pub(crate) struct KeyTypesInfo {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) types: Vec<KeyTypeInfo>,
    pub(crate) type_index: HashMap<u32, usize>,
    pub(crate) mods: XkbModSet,
}

#[derive(Clone, Default)]
pub(crate) struct KeyTypeInfo {
    pub(crate) defined: u32,
    pub(crate) merge: MergeMode,
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
fn key_types_info(include_depth: u32, mods: &XkbModSet) -> KeyTypesInfo {
    let mut info = KeyTypesInfo {
        include_depth,
        ..Default::default()
    };
    init_vmods(&mut info.mods, mods, include_depth > 0);
    info
}
fn add_key_type(info: &mut KeyTypesInfo, new: &mut KeyTypeInfo) {
    if let Some(&idx) = info.type_index.get(&new.name) {
        if new.merge != MergeMode::Augment {
            std::mem::swap(&mut info.types[idx], new);
            return;
        }
        return;
    }
    info.type_index.insert(new.name, info.types.len());
    info.types.push(std::mem::take(new));
}
fn merge_included_key_types(into: &mut KeyTypesInfo, from: &mut KeyTypesInfo, merge: MergeMode) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut into.mods, &from.mods, merge);
    if into.types.is_empty() {
        into.types = std::mem::take(&mut from.types);
        into.type_index = std::mem::take(&mut from.type_index);
    } else {
        for mut type_0 in from.types.drain(..) {
            type_0.merge = merge;
            add_key_type(into, &mut type_0);
        }
    }
}
fn handle_include_key_types(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    includes: &mut [IncludeStmt],
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    let mut included = key_types_info(info.include_depth.wrapping_add(1), &info.mods);
    for stmt in includes.iter() {
        let mut file = include_file!(ki, info, stmt, FileType::Types);
        let mut next = key_types_info(info.include_depth.wrapping_add(1), &included.mods);
        handle_key_types_file(ki, &mut next, &mut file);
        merge_included_key_types(&mut included, &mut next, stmt.merge);
    }
    if let Some(first) = includes.first() {
        merge_included_key_types(info, &mut included, first.merge);
    }
    info.error_count == 0
}
fn set_modifiers(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if array_ndx.is_some() {
        return false;
    }
    let mods = some_or_false!(expr_resolve_mod_mask(
        &ki.keymap.ctx,
        value,
        MOD_BOTH,
        &info.mods
    ));
    if type_0.defined & TYPE_FIELD_MASK != 0 {
        return false;
    }
    type_0.mods = mods;
    true
}
fn add_map_entry(type_0: &mut KeyTypeInfo, new: &XkbKeyTypeEntry) {
    // FindMatchingMapEntry inlined
    let mut old_idx: Option<usize> = None;
    for (i, entry) in type_0.entries.iter().enumerate() {
        if entry.mods.mods == new.mods.mods {
            old_idx = Some(i);
            break;
        }
    }
    if let Some(idx) = old_idx {
        if type_0.entries[idx].level != new.level {
            if new.level >= type_0.num_levels {
                type_0.num_levels = new.level.wrapping_add(1_u32);
            }
            type_0.entries[idx].level = new.level;
        }
        return;
    }
    if new.level >= type_0.num_levels {
        type_0.num_levels = new.level.wrapping_add(1_u32);
    }
    type_0.entries.push(*new);
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
    let Some(mods) =
        expr_resolve_mod_mask(&ki.keymap.ctx, array_ndx.unwrap(), MOD_BOTH, &info.mods)
    else {
        return false;
    };
    entry.mods.mods = mods;
    if entry.mods.mods & !type_0.mods != 0 {
        entry.mods.mods &= type_0.mods;
    }
    let level = some_or_false!(expr_resolve_level(&ki.keymap.ctx, value));
    entry.level = level;
    entry.preserve.mods = 0;
    add_map_entry(type_0, &entry);
    true
}
fn add_preserve(type_0: &mut KeyTypeInfo, mods: u32, preserve_mods: u32) {
    // Find matching entry index first to avoid borrow conflicts
    let match_idx = type_0.entries.iter().position(|e| e.mods.mods == mods);
    if let Some(idx) = match_idx {
        let old_preserve = type_0.entries[idx].preserve.mods;
        if old_preserve == 0 {
            type_0.entries[idx].preserve.mods = preserve_mods;
            return;
        }
        if old_preserve == preserve_mods {
            return;
        }
        type_0.entries[idx].preserve.mods = preserve_mods;
        return;
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
    let Some(mut mods) =
        expr_resolve_mod_mask(&ki.keymap.ctx, array_ndx.unwrap(), MOD_BOTH, &info.mods)
    else {
        return false;
    };
    if mods & !type_0.mods != 0 {
        mods &= type_0.mods;
    }
    let Some(mut preserve_mods) =
        expr_resolve_mod_mask(&ki.keymap.ctx, value, MOD_BOTH, &info.mods)
    else {
        return false;
    };
    if preserve_mods & !mods != 0 {
        preserve_mods &= mods;
    }
    add_preserve(type_0, mods, preserve_mods);
    true
}
fn add_level_name(type_0: &mut KeyTypeInfo, level: u32, name: u32) {
    let level_idx = level as usize;
    if type_0.level_names.get(level_idx) == Some(&name) {
        return;
    }
    if level >= type_0.level_names.len() as u32 {
        type_0.level_names.resize(level_idx + 1, 0);
    }
    type_0.level_names[level_idx] = name;
}
fn set_level_name(
    ki: &XkbKeymapInfo<'_>,
    type_0: &mut KeyTypeInfo,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if array_ndx.is_none() {
        return false;
    }
    let level = some_or_false!(expr_resolve_level(&ki.keymap.ctx, array_ndx.unwrap()));
    let level_name = some_or_false!(expr_resolve_string(value));
    add_level_name(type_0, level, level_name);
    true
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
        ok = set_level_name(ki, type_0, array_ndx, value);
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
        let name_ref = def.name.as_ref().unwrap();
        if let Some(lhs) = expr_resolve_lhs(name_ref) {
            let elem = ki.keymap.ctx.atom_text(lhs.element);
            let field = ki.keymap.ctx.atom_text(lhs.field);
            if !elem.is_empty() {
                if !elem.eq_ignore_ascii_case("type") {
                    ok = false;
                }
            } else {
                let value_ref = def.value.as_ref().unwrap();
                if !set_key_type_field(ki, info, type_0, field, lhs.index, value_ref) {
                    ok = false;
                }
            }
        } else {
            ok = false;
        }
    }
    ok
}
fn handle_type_global_var(ki: &XkbKeymapInfo<'_>, stmt: &VarDef) -> bool {
    let name_ref = stmt.name.as_ref().unwrap();
    let lhs = some_or_false!(expr_resolve_lhs(name_ref));
    let elem = ki.keymap.ctx.atom_text(lhs.element);
    let field = ki.keymap.ctx.atom_text(lhs.field);
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
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_key_types(ki, info, incl);
                }
                Statement::KeyType(def) => {
                    let mut type_0: KeyTypeInfo = KeyTypeInfo {
                        merge: def.merge,
                        name: def.name,
                        num_levels: 1_u32,
                        ..Default::default()
                    };
                    if !handle_key_type_body(ki, info, &def.body, &mut type_0) {
                        info.error_count += 1;
                        ok = false;
                    } else {
                        add_key_type(info, &mut type_0);
                        ok = true;
                    }
                }
                Statement::Var(var) => {
                    ok = handle_type_global_var(ki, var);
                }
                Statement::VMod(vmod) => {
                    ok = handle_vmod_def(&mut ki.keymap.ctx, &mut info.mods, vmod);
                }
                Statement::Unknown => {
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
fn copy_key_types_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut KeyTypesInfo) {
    // let keymap = ki.keymap;
    let num_types: u32 = if info.types.is_empty() {
        1_u32
    } else {
        info.types.len() as u32
    };
    let mut types_vec: Vec<XkbKeyType> = Vec::with_capacity(num_types as usize);
    if info.types.is_empty() {
        let type_0 = XkbKeyType {
            name: ki.keymap.ctx.atom_intern(b"ONE_LEVEL"),
            mods: XkbMods { mods: 0, mask: 0 },
            num_levels: 1,
            entries: Vec::new(),
        };
        types_vec.push(type_0);
    } else {
        for def in info.types.iter_mut() {
            let entries = std::mem::take(&mut def.entries);
            types_vec.push(XkbKeyType {
                name: def.name,
                mods: XkbMods {
                    mods: def.mods,
                    mask: 0,
                },
                num_levels: def.num_levels,
                entries,
            });
        }
    }
    ki.keymap.types = types_vec;
    ki.keymap.mods = info.mods;
}
pub(crate) fn compile_key_types(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut info = key_types_info(0, &mods);
    if let Some(file) = file {
        handle_key_types_file(keymap_info, &mut info, file);
    }
    if info.error_count != 0 {
        return false;
    }
    copy_key_types_to_keymap(keymap_info, &mut info);
    true
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
pub(crate) fn merge_mod_sets(into: &mut XkbModSet, from: &XkbModSet, merge: MergeMode) {
    let clobber: bool = merge != MergeMode::Augment;
    for vmod in 0..from.num_mods as usize {
        let mod_0 = &from.mods[vmod];
        let mask: u32 = 1_u32 << vmod;
        if mod_0.type_0 != MOD_VIRT {
            continue;
        }
        if into.mods[vmod].type_0 == 0 {
            into.mods[vmod] = *mod_0;
            if from.explicit_vmods & mask != 0 {
                into.explicit_vmods |= mask;
            }
        } else if from.explicit_vmods & mask != 0 {
            if into.explicit_vmods & mask == 0 {
                into.mods[vmod].mapping = mod_0.mapping;
                into.explicit_vmods |= mask;
            } else if clobber {
                into.mods[vmod].mapping = mod_0.mapping;
            }
        }
    }
    into.num_mods = from.num_mods;
}
pub(crate) fn handle_vmod_def(ctx: &mut XkbContext, mods: &mut XkbModSet, stmt: &VModDef) -> bool {
    let mut mapping: u32 = 0;
    if let Some(value_ref) = stmt.value.as_ref() {
        let value = some_or_false!(expr_resolve_mod_mask(ctx, value_ref, MOD_REAL, mods));
        mapping = value;
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
                let clobber: bool = stmt.merge != MergeMode::Augment;
                let use_0: u32 = if clobber {
                    mapping
                } else {
                    mods.mods[vmod].mapping
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
    mods.num_mods += 1;
    true
}
#[derive(Default)]
pub(crate) struct KeyNamesInfo {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) keycodes: KeycodeStore,
    pub(crate) led_names: [LedNameInfo; 32],
    pub(crate) num_led_names: u32,
    pub(crate) led_name_index: HashMap<u32, u32>,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct LedNameInfo {
    pub(crate) merge: MergeMode,
    pub(crate) name: u32,
}
#[derive(Clone)]
pub(crate) struct KeycodeStore {
    pub(crate) min: u32,
    pub(crate) low: Vec<u32>,
    pub(crate) high: Vec<HighKeycodeEntry>,
    pub(crate) names: Vec<KeycodeMatch>,
}
impl Default for KeycodeStore {
    fn default() -> Self {
        Self {
            min: XKB_KEYCODE_INVALID,
            low: Vec::new(),
            high: Vec::new(),
            names: Vec::new(),
        }
    }
}
#[derive(Copy, Clone, Default)]
pub(crate) struct HighKeycodeEntry {
    pub(crate) keycode: u32,
    pub(crate) name: u32,
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
        store.names.resize(name as usize + 1, Default::default());
    }
    store.names[name as usize] = match_0;
}
fn keycode_store_insert_key(store: &mut KeycodeStore, kc: u32, name: u32) -> bool {
    if name >= store.names.len() as u32 {
        store.names.resize(name as usize + 1, Default::default());
    }
    if kc <= XKB_KEYCODE_MAX_CONTIGUOUS {
        if kc >= store.low.len() as u32 {
            store.low.resize(kc as usize + 1, 0);
        }
        store.low[kc as usize] = name;
        if kc < store.min {
            store.min = kc;
        }
        store.names[name as usize] = KeycodeMatch {
            found: true,
            low: true,
            index: kc,
            ..Default::default()
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
                index: lower,
                ..Default::default()
            };
        } else {
            store.high.push(HighKeycodeEntry { keycode: kc, name });
            store.names[name as usize] = KeycodeMatch {
                found: true,
                index: idx,
                ..Default::default()
            };
        }
        if store.low.is_empty() {
            store.min = store.high[0].keycode;
        }
    }
    true
}
#[inline]
fn keycode_store_insert_alias(store: &mut KeycodeStore, alias: u32, real: u32) {
    if alias >= store.names.len() as u32 {
        store.names.resize(alias as usize + 1, Default::default());
    }
    store.names[alias as usize] = KeycodeMatch {
        found: true,
        low: true,
        is_alias: real != 0,
        index: real,
    };
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
                for idx in (1..=match_0.index).rev() {
                    if store.low[(idx - 1) as usize] != XKB_ATOM_NONE {
                        store.low.truncate(idx as usize);
                        break;
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
        for kc in store.min..store.low.len() as u32 {
            if store.low[kc as usize] != XKB_ATOM_NONE {
                store.min = kc;
                break;
            }
        }
    }
}
fn keycode_store_lookup_keycode(store: &KeycodeStore, kc: u32) -> KeycodeMatch {
    if kc < store.low.len() as u32 {
        return KeycodeMatch {
            found: true,
            low: true,
            index: kc,
            ..Default::default()
        };
    } else if kc <= XKB_KEYCODE_MAX_CONTIGUOUS {
        return KeycodeMatch::default();
    }
    match store.high.binary_search_by(|entry| entry.keycode.cmp(&kc)) {
        Ok(mid) => KeycodeMatch {
            found: true,
            index: mid as u32,
            ..Default::default()
        },
        Err(_) => KeycodeMatch::default(),
    }
}
fn keycode_store_lookup_name(store: &KeycodeStore, name: u32) -> KeycodeMatch {
    if name >= store.names.len() as u32 {
        KeycodeMatch::default()
    } else {
        store.names[name as usize]
    }
}
fn add_led_name(info: &mut KeyNamesInfo, new: &LedNameInfo, new_idx: u32) {
    let replace: bool = new.merge != MergeMode::Augment;
    if let Some(&old_idx) = info.led_name_index.get(&new.name) {
        if old_idx == new_idx {
            return;
        }
        if replace {
            info.led_names[old_idx as usize].name = XKB_ATOM_NONE;
            info.led_name_index.remove(&new.name);
        } else {
            return;
        }
    }
    if new_idx >= info.num_led_names {
        info.num_led_names = new_idx.wrapping_add(1_u32);
    }
    if info.led_names[new_idx as usize].name != XKB_ATOM_NONE {
        if replace {
            info.led_name_index
                .remove(&info.led_names[new_idx as usize].name);
            info.led_names[new_idx as usize] = *new;
            info.led_name_index.insert(new.name, new_idx);
        }
        return;
    }
    info.led_names[new_idx as usize] = *new;
    info.led_name_index.insert(new.name, new_idx);
}
fn key_names_info(include_depth: u32) -> KeyNamesInfo {
    KeyNamesInfo {
        include_depth,
        ..Default::default()
    }
}
fn add_key_name(info: &mut KeyNamesInfo, kc: u32, name: u32, merge: MergeMode) -> bool {
    let match_name: KeycodeMatch = keycode_store_lookup_name(&info.keycodes, name);
    if match_name.found {
        let clobber: bool = merge != MergeMode::Augment;
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
        let clobber_0: bool = merge != MergeMode::Augment;
        if clobber_0 {
            keycode_store_delete_name(&mut info.keycodes, old_name);
            keycode_store_update_key(&mut info.keycodes, match_kc, name);
        }
    } else if !keycode_store_insert_key(&mut info.keycodes, kc, name) {
        return false;
    }
    true
}
fn merge_keycode_stores(into: &mut KeyNamesInfo, from: &mut KeyNamesInfo, merge: MergeMode) {
    if into.keycodes.low.is_empty()
        && into.keycodes.high.is_empty()
        && into.keycodes.names.is_empty()
    {
        into.keycodes = std::mem::take(&mut from.keycodes);
    } else {
        for kc in from.keycodes.min..from.keycodes.low.len() as u32 {
            let name = from.keycodes.low[kc as usize];
            if (name != XKB_ATOM_NONE) && !add_key_name(into, kc, name, merge) {
                into.error_count += 1;
            }
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
                        handle_alias_def(into, &def);
                    }
                }
            }
        }
    };
}
fn merge_included_keycodes(into: &mut KeyNamesInfo, from: &mut KeyNamesInfo, merge: MergeMode) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_keycode_stores(into, from, merge);
    if into.num_led_names == 0 {
        into.led_names[..from.num_led_names as usize]
            .copy_from_slice(&from.led_names[..from.num_led_names as usize]);
        into.num_led_names = from.num_led_names;
        from.num_led_names = 0;
    } else {
        for idx in 0..from.num_led_names as usize {
            let ledi = from.led_names[idx];
            if ledi.name != XKB_ATOM_NONE {
                let mut ledi = ledi;
                ledi.merge = merge;
                add_led_name(into, &ledi, idx as u32);
            }
        }
    };
}
fn handle_include_keycodes(
    info: &mut KeyNamesInfo,
    includes: &mut [IncludeStmt],
    ki: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mut included = key_names_info(0);
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    for stmt in includes.iter() {
        let mut file = include_file!(ki, info, stmt, FileType::Keycodes);
        let mut next = key_names_info(info.include_depth.wrapping_add(1));
        handle_keycodes_file(&mut next, &mut file, ki);
        merge_included_keycodes(&mut included, &mut next, stmt.merge);
    }
    if let Some(first) = includes.first() {
        merge_included_keycodes(info, &mut included, first.merge);
    }
    info.error_count == 0
}
fn handle_keycode_def(info: &mut KeyNamesInfo, stmt: &KeycodeDef) -> bool {
    if stmt.value < 0_i64 || stmt.value > XKB_KEYCODE_MAX as i64 {
        return false;
    }
    add_key_name(info, stmt.value as u32, stmt.name, stmt.merge)
}
fn handle_alias_def(info: &mut KeyNamesInfo, def: &KeyAliasDef) {
    let match_name: KeycodeMatch =
        keycode_store_lookup_name(&info.keycodes, def.alias) as KeycodeMatch;
    if match_name.found {
        let clobber: bool = def.merge != MergeMode::Augment;
        if match_name.is_alias {
            if def.real != match_name.index && clobber {
                info.keycodes.names[def.alias as usize].index = def.real;
            }
            return;
        } else if clobber {
            keycode_store_delete_key(&mut info.keycodes, match_name);
        } else {
            return;
        }
    }
    keycode_store_insert_alias(&mut info.keycodes, def.alias, def.real);
}
fn handle_key_name_var(ki: &mut XkbKeymapInfo<'_>, stmt: &VarDef) -> bool {
    let name_ref = stmt.name.as_ref().unwrap();
    let lhs = some_or_false!(expr_resolve_lhs(name_ref));
    let elem = ki.keymap.ctx.atom_text(lhs.element).to_owned();
    let field = ki.keymap.ctx.atom_text(lhs.field).to_owned();
    if !elem.is_empty() {
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if !field.eq_ignore_ascii_case("minimum") && !field.eq_ignore_ascii_case("maximum") {
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if lhs.index.is_some() {
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    let value_ref = stmt.value.as_ref().unwrap();
    let Some(val) = expr_resolve_integer(&ki.keymap.ctx, value_ref) else {
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    };
    if val < 0_i64 || val > u32::MAX as i64 {
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    true
}
fn handle_led_name_def(info: &mut KeyNamesInfo, def: &LedNameDef) -> bool {
    if def.ndx < 1_i64 || def.ndx > XKB_MAX_LEDS as i64 {
        info.error_count += 1;
        return false;
    }
    let name_expr = def.name.as_ref().unwrap();
    let Some(name) = expr_resolve_string(name_expr) else {
        info.error_count += 1;
        return false;
    };
    let ledi: LedNameInfo = LedNameInfo {
        merge: def.merge,
        name,
    };
    add_led_name(info, &ledi, (def.ndx as u32).wrapping_sub(1_u32));
    true
}
fn handle_keycodes_file(info: &mut KeyNamesInfo, file: &mut XkbFile, ki: &mut XkbKeymapInfo<'_>) {
    {
        let mut ok: bool;
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = handle_include_keycodes(info, incl, ki);
                }
                Statement::Keycode(kc) => {
                    ok = handle_keycode_def(info, kc);
                }
                Statement::KeyAlias(ka) => {
                    handle_alias_def(info, ka);
                    ok = true;
                }
                Statement::Var(var) => {
                    ok = handle_key_name_var(ki, var);
                }
                Statement::LedName(ln) => {
                    ok = handle_led_name_def(info, ln);
                }
                Statement::Unknown => {
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
fn copy_key_names_to_keymap(keymap: &mut XkbKeymap, keycodes: &KeycodeStore) {
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
    for kc in keymap.min_key_code..keymap.num_keys_low {
        keys[kc as usize].keycode = kc;
    }
    for kc in keycodes.min..keycodes.low.len() as u32 {
        keys[kc as usize].name = keycodes.low[kc as usize];
    }
    let mut idx: u32 = keymap.num_keys_low;
    for entry in keycodes.high.iter() {
        keys[idx as usize].keycode = entry.keycode;
        keys[idx as usize].name = entry.name;
        idx += 1;
    }
    keymap.keys = keys;
}
fn copy_keycode_name_lut(keymap: &mut XkbKeymap, keycodes: &mut KeycodeStore) {
    for name in 0..keycodes.names.len() {
        let entry = keycodes.names[name];
        if entry.found {
            if entry.is_alias {
                let match_real = keycode_store_lookup_name(keycodes, entry.index);
                if !match_real.found || match_real.is_alias {
                    keycodes.names[name].found = false;
                }
            } else if !entry.low {
                keycodes.names[name].index += keymap.num_keys_low;
            }
        }
    }
    keymap.key_names = std::mem::take(&mut keycodes.names);
}
fn copy_led_names_to_keymap(
    keymap: &mut XkbKeymap,
    led_names: &[LedNameInfo; 32],
    num_led_names: u32,
) {
    keymap.num_leds = num_led_names;
    for (idx, ledi) in led_names.iter().enumerate().take(num_led_names as usize) {
        if ledi.name != XKB_ATOM_NONE {
            keymap.leds[idx].name = ledi.name;
        }
    }
}
pub(crate) fn compile_keycodes(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mut info = key_names_info(0);
    if let Some(file) = file {
        handle_keycodes_file(&mut info, file, keymap_info);
    }
    if info.error_count != 0 {
        return false;
    }
    copy_key_names_to_keymap(keymap_info.keymap, &info.keycodes);
    copy_keycode_name_lut(keymap_info.keymap, &mut info.keycodes);
    copy_led_names_to_keymap(keymap_info.keymap, &info.led_names, info.num_led_names);
    true
}
use super::keymap::{ACTION_TYPE_NAMES, GROUP_LAST_INDEX_NAME};

pub(crate) use super::keymap::action_equal;

use super::parser::ExprKind;

/// Safe replacement for the IdentLookupFunc + *const c_void pair.
pub(crate) enum IdentLookup<'a> {
    None,
    Simple(&'a [LookupEntry]),
    NamedPattern(&'a NamedIntegerPattern<'a>),
    ModMask(&'a XkbModSet, u32),
}

pub(crate) struct NamedIntegerPattern<'a> {
    pub(crate) prefix: &'static str,
    pub(crate) min: u32,
    pub(crate) max: u32,
    pub(crate) entries: &'a [LookupEntry],
    pub(crate) pending_entries: &'a [LookupEntry],
    pub(crate) is_mask: bool,
}

static LEVEL_NAME_PATTERN_ENTRIES: [LookupEntry; 1] = [lookup_entry("", 0)];

fn simple_lookup(ctx: &XkbContext, entries: &[LookupEntry], field: u32) -> Option<u32> {
    if field == XKB_ATOM_NONE {
        return None;
    }
    let s: &str = ctx.atom_text(field);
    entries
        .iter()
        .take_while(|entry| !entry.name.is_empty())
        .find(|entry| s.eq_ignore_ascii_case(entry.name))
        .map(|entry| entry.value)
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
    let str_bytes: &str = ctx.atom_text(field);
    let prefix = pattern.prefix;
    if str_bytes
        .as_bytes()
        .get(..prefix.len())
        .is_some_and(|s| s.eq_ignore_ascii_case(prefix.as_bytes()))
    {
        let suffix = &str_bytes.as_bytes()[prefix.len()..];
        let (val, count) = super::parser::parse_dec_u32(suffix);
        if count > 0 && prefix.len() + count as usize == str_bytes.len() {
            return (pattern.min..=pattern.max).contains(&val).then(|| {
                if pattern.is_mask {
                    1 << val.wrapping_sub(pattern.min)
                } else {
                    val
                }
            });
        }
    }
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
        IdentLookup::ModMask(mods, mod_type) => match ctx.atom_text(field) {
            s if s.eq_ignore_ascii_case("all") => Some(MOD_REAL_MASK_ALL),
            s if s.eq_ignore_ascii_case("none") => Some(0),
            "" => None,
            _ => xkb_mod_name_to_index(mods, field, *mod_type).map(|ndx| 1 << ndx),
        },
    }
}

struct Lhs<'a> {
    element: u32,
    field: u32,
    index: Option<&'a ExprKind>,
}

fn expr_resolve_lhs(expr: &ExprKind) -> Option<Lhs<'_>> {
    match expr {
        ExprKind::Ident(field) if *field != XKB_ATOM_NONE => Some(Lhs {
            element: XKB_ATOM_NONE,
            field: *field,
            index: None,
        }),
        ExprKind::FieldRef {
            element,
            field,
            index,
        } if *field != XKB_ATOM_NONE && (*element != XKB_ATOM_NONE || index.is_some()) => {
            Some(Lhs {
                element: *element,
                field: *field,
                index: index.as_deref(),
            })
        }
        _ => None,
    }
}

pub(crate) fn expr_resolve_boolean(ctx: &XkbContext, expr: &ExprKind) -> Option<bool> {
    match expr {
        ExprKind::Boolean(set) => Some(*set),
        ExprKind::Ident(ident_atom) => {
            let ident = ctx.atom_text(*ident_atom);
            if ident.eq_ignore_ascii_case("true")
                || ident.eq_ignore_ascii_case("yes")
                || ident.eq_ignore_ascii_case("on")
            {
                Some(true)
            } else if ident.eq_ignore_ascii_case("false")
                || ident.eq_ignore_ascii_case("no")
                || ident.eq_ignore_ascii_case("off")
            {
                Some(false)
            } else {
                None
            }
        }
        ExprKind::Unary {
            child,
            op: UnaryOp::Not | UnaryOp::Invert,
        } => expr_resolve_boolean(ctx, child.as_deref()?).map(|set| !set),
        _ => None,
    }
}

fn expr_resolve_integer_lookup(
    ctx: &XkbContext,
    expr: &ExprKind,
    pending: Option<&mut bool>,
    lookup: &IdentLookup,
) -> Option<i64> {
    match expr {
        ExprKind::Integer(value) => Some(*value),
        ExprKind::Ident(ident_atom) => {
            let mut pending_local = false;
            let pending_ref = if pending.is_some() {
                Some(&mut pending_local)
            } else {
                None
            };
            let value = ident_lookup(ctx, lookup, *ident_atom, pending_ref).map(i64::from);
            if let Some(p) = pending {
                *p = pending_local;
                if pending_local {
                    return None;
                }
            }
            value
        }
        ExprKind::Binary {
            left,
            right,
            op: op @ (BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide),
        } => {
            let left = expr_resolve_integer_lookup(ctx, left.as_deref()?, None, lookup)?;
            let right = expr_resolve_integer_lookup(ctx, right.as_deref()?, None, lookup)?;
            match op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Subtract => left.checked_sub(right),
                BinaryOp::Multiply => left.checked_mul(right),
                BinaryOp::Divide => left.checked_div(right),
                BinaryOp::Assign => unreachable!(),
            }
        }
        ExprKind::Unary { child, op } => {
            let value = expr_resolve_integer_lookup(ctx, child.as_deref()?, None, lookup)?;
            match op {
                UnaryOp::Invert => Some(!value),
                UnaryOp::Negate => value.checked_neg(),
                UnaryOp::Plus => Some(value),
                UnaryOp::Not => None,
            }
        }
        _ => None,
    }
}

pub(crate) fn expr_resolve_integer(ctx: &XkbContext, expr: &ExprKind) -> Option<i64> {
    expr_resolve_integer_lookup(ctx, expr, None, &IdentLookup::None)
}

pub(crate) fn expr_resolve_group(
    keymap_info: &mut XkbKeymapInfo<'_>,
    expr: &ExprKind,
    absolute: bool,
    group_rtrn: &mut u32,
    pending: &mut bool,
) -> ParseStatus {
    static PENDING_GROUP_INDEX_NAMES: [LookupEntry; 2] =
        [lookup_entry(GROUP_LAST_INDEX_NAME, 0), lookup_entry("", 0)];
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
    let result = some_or_false!(
        expr_resolve_integer_lookup(ctx, expr, Some(pending), &lookup),
        report_mismatch(keymap_info.strict)
    );
    if result > keymap_info.features.max_groups as i64
        || (absolute && result < 1)
        || (!absolute && result < -(keymap_info.features.max_groups as i64))
    {
        return report_mismatch(keymap_info.strict);
    }
    *group_rtrn = result as u32;
    ParseStatus::Success
}

pub(crate) fn expr_resolve_level(ctx: &XkbContext, expr: &ExprKind) -> Option<u32> {
    let pattern = NamedIntegerPattern {
        prefix: "Level",
        min: 1_u32,
        max: XKB_LEVEL_MAX_IMPL,
        entries: &LEVEL_NAME_PATTERN_ENTRIES,
        pending_entries: &LEVEL_NAME_PATTERN_ENTRIES,
        is_mask: false,
    };
    let lookup = IdentLookup::NamedPattern(&pattern);
    let result = expr_resolve_integer_lookup(ctx, expr, None, &lookup)?;
    (1..=XKB_LEVEL_MAX_IMPL as i64)
        .contains(&result)
        .then_some((result - 1) as u32)
}

pub(crate) fn expr_resolve_string(expr: &ExprKind) -> Option<u32> {
    match expr {
        ExprKind::String(value) => Some(*value),
        _ => None,
    }
}

pub(crate) fn expr_resolve_enum(
    ctx: &XkbContext,
    expr: &ExprKind,
    values: &[LookupEntry],
) -> Option<u32> {
    let ExprKind::Ident(ident_atom) = expr else {
        return None;
    };
    simple_lookup(ctx, values, *ident_atom)
}

fn expr_resolve_mask_lookup(
    ctx: &XkbContext,
    expr: &ExprKind,
    pending: Option<&mut bool>,
    lookup: &IdentLookup,
) -> Option<u32> {
    match expr {
        ExprKind::Integer(ival) => {
            if *ival < 0 || *ival > u32::MAX as i64 {
                None
            } else {
                Some(*ival as u32)
            }
        }
        ExprKind::Ident(ident_atom) => {
            let mut pending_local = false;
            let pending_ref = if pending.is_some() {
                Some(&mut pending_local)
            } else {
                None
            };
            let value = ident_lookup(ctx, lookup, *ident_atom, pending_ref);
            if let Some(p) = pending {
                *p = pending_local;
                if pending_local {
                    return None;
                }
            }
            value
        }
        ExprKind::Binary { left, right, op } => {
            let left = expr_resolve_mask_lookup(ctx, left.as_deref()?, None, lookup)?;
            let right = expr_resolve_mask_lookup(ctx, right.as_deref()?, None, lookup)?;
            match op {
                BinaryOp::Add => Some(left | right),
                BinaryOp::Subtract => Some(left & !right),
                BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Assign => None,
            }
        }
        ExprKind::Unary { child, op } => {
            if *op != UnaryOp::Invert {
                None
            } else {
                let value = expr_resolve_integer_lookup(ctx, child.as_deref()?, None, lookup)?;
                if value < 0 || value > u32::MAX as i64 {
                    None
                } else {
                    Some(!(value as u32))
                }
            }
        }
        _ => None,
    }
}

pub(crate) fn expr_resolve_mask(
    ctx: &XkbContext,
    expr: &ExprKind,
    values: &[LookupEntry],
) -> Option<u32> {
    let lookup = IdentLookup::Simple(values);
    expr_resolve_mask_lookup(ctx, expr, None, &lookup)
}

pub(crate) fn expr_resolve_mod_mask(
    ctx: &XkbContext,
    expr: &ExprKind,
    mod_type: u32,
    mods: &XkbModSet,
) -> Option<u32> {
    let lookup = IdentLookup::ModMask(mods, mod_type);
    expr_resolve_mask_lookup(ctx, expr, None, &lookup)
}

pub(crate) fn expr_resolve_mod(def: &ExprKind, mod_type: u32, mods: &XkbModSet) -> Option<u32> {
    let ExprKind::Ident(ident_atom) = def else {
        return None;
    };
    xkb_mod_name_to_index(mods, *ident_atom, mod_type)
}

pub(crate) fn expr_resolve_group_mask(
    keymap_info: &mut XkbKeymapInfo<'_>,
    expr: &ExprKind,
    group_rtrn: &mut u32,
    pending_rtrn: &mut bool,
) -> bool {
    static PENDING_GROUP_MASK_NAMES: [LookupEntry; 2] =
        [lookup_entry(GROUP_LAST_INDEX_NAME, 0), lookup_entry("", 0)];
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
    let group = some_or_false!(expr_resolve_mask_lookup(
        ctx,
        expr,
        Some(pending_rtrn),
        &lookup
    ));
    *group_rtrn = group;
    true
}
pub(crate) type ActionsInfo = [XkbAction; 21];

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
///
pub(crate) enum ActionValue<'v> {
    /// A borrowed reference to a constant or non-ownable ExprDef (e.g. const_true).
    Borrowed(&'v ExprKind),
    /// A mutable reference to an owned ExprDef that can be `.take()`-en.
    Owned(&'v mut Option<ExprKind>),
    /// An owned recursive expression edge.
    Boxed(&'v mut Option<Box<ExprKind>>),
}

impl<'v> ActionValue<'v> {
    /// Get a shared reference to the underlying ExprDef.
    #[inline]
    pub(crate) fn get(&self) -> &ExprKind {
        match self {
            ActionValue::Borrowed(e) => e,
            ActionValue::Owned(opt) => opt.as_ref().unwrap(),
            ActionValue::Boxed(opt) => opt.as_deref().unwrap(),
        }
    }
    /// Take ownership of the ExprDef (only possible for Owned variant).
    #[inline]
    pub(crate) fn take(&mut self) -> Option<ExprKind> {
        match self {
            ActionValue::Borrowed(_) => None,
            ActionValue::Owned(opt) => opt.take(),
            ActionValue::Boxed(opt) => opt.take().map(|expr| *expr),
        }
    }

    fn unary_child(self) -> Option<(bool, ActionValue<'v>)> {
        match self {
            ActionValue::Borrowed(ExprKind::Unary { child, op }) => Some((
                *op == UnaryOp::Negate,
                ActionValue::Borrowed(child.as_deref()?),
            )),
            ActionValue::Owned(expr) => {
                let ExprKind::Unary { child, op } = expr.as_mut()? else {
                    return None;
                };
                Some((*op == UnaryOp::Negate, ActionValue::Boxed(child)))
            }
            ActionValue::Boxed(expr) => {
                let ExprKind::Unary { child, op } = expr.as_deref_mut()? else {
                    return None;
                };
                Some((*op == UnaryOp::Negate, ActionValue::Boxed(child)))
            }
            ActionValue::Borrowed(_) => None,
        }
    }
}

pub(crate) fn init_actions_info(info: &mut ActionsInfo) {
    for type_0 in ACTION_TYPE_NONE.._ACTION_TYPE_NUM_ENTRIES {
        info[type_0 as usize] = match type_0 {
            ACTION_TYPE_NONE => XkbAction::None,
            ACTION_TYPE_VOID => XkbAction::Void,
            ACTION_TYPE_MOD_SET => XkbAction::ModSet(Default::default()),
            ACTION_TYPE_MOD_LATCH => XkbAction::ModLatch(Default::default()),
            ACTION_TYPE_MOD_LOCK => XkbAction::ModLock(Default::default()),
            ACTION_TYPE_GROUP_SET => XkbAction::GroupSet(Default::default()),
            ACTION_TYPE_GROUP_LATCH => XkbAction::GroupLatch(Default::default()),
            ACTION_TYPE_GROUP_LOCK => XkbAction::GroupLock(Default::default()),
            ACTION_TYPE_CTRL_SET => XkbAction::CtrlSet(Default::default()),
            ACTION_TYPE_CTRL_LOCK => XkbAction::CtrlLock(Default::default()),
            ACTION_TYPE_UNKNOWN => XkbAction::Unknown,
            ACTION_TYPE_PRIVATE => XkbAction::Private(Default::default()),
            ACTION_TYPE_INTERNAL => XkbAction::Internal(Default::default()),
            _ => XkbAction::None,
        };
    }
}
static FIELD_STRINGS: [LookupEntry; 37] = [
    lookup_entry("clearLocks", ACTION_FIELD_CLEAR_LOCKS),
    lookup_entry("latchToLock", ACTION_FIELD_LATCH_TO_LOCK),
    lookup_entry("genKeyEvent", ACTION_FIELD_GEN_KEY_EVENT),
    lookup_entry("generateKeyEvent", ACTION_FIELD_GEN_KEY_EVENT),
    lookup_entry("report", ACTION_FIELD_REPORT),
    lookup_entry("default", ACTION_FIELD_DEFAULT),
    lookup_entry("affect", ACTION_FIELD_AFFECT),
    lookup_entry("increment", ACTION_FIELD_INCREMENT),
    lookup_entry("modifiers", ACTION_FIELD_MODIFIERS),
    lookup_entry("mods", ACTION_FIELD_MODIFIERS),
    lookup_entry("group", ACTION_FIELD_GROUP),
    lookup_entry("x", ACTION_FIELD_X),
    lookup_entry("y", ACTION_FIELD_Y),
    lookup_entry("accel", ACTION_FIELD_ACCEL),
    lookup_entry("accelerate", ACTION_FIELD_ACCEL),
    lookup_entry("repeat", ACTION_FIELD_ACCEL),
    lookup_entry("button", ACTION_FIELD_BUTTON),
    lookup_entry("value", ACTION_FIELD_VALUE),
    lookup_entry("controls", ACTION_FIELD_CONTROLS),
    lookup_entry("ctrls", ACTION_FIELD_CONTROLS),
    lookup_entry("type", ACTION_FIELD_TYPE),
    lookup_entry("count", ACTION_FIELD_COUNT),
    lookup_entry("screen", ACTION_FIELD_SCREEN),
    lookup_entry("same", ACTION_FIELD_SAME),
    lookup_entry("sameServer", ACTION_FIELD_SAME),
    lookup_entry("data", ACTION_FIELD_DATA),
    lookup_entry("device", ACTION_FIELD_DEVICE),
    lookup_entry("dev", ACTION_FIELD_DEVICE),
    lookup_entry("key", ACTION_FIELD_KEYCODE),
    lookup_entry("keycode", ACTION_FIELD_KEYCODE),
    lookup_entry("kc", ACTION_FIELD_KEYCODE),
    lookup_entry("clearmods", ACTION_FIELD_MODS_TO_CLEAR),
    lookup_entry("clearmodifiers", ACTION_FIELD_MODS_TO_CLEAR),
    lookup_entry("lockOnRelease", ACTION_FIELD_LOCK_ON_RELEASE),
    lookup_entry("unlockOnPress", ACTION_FIELD_UNLOCK_ON_PRESS),
    lookup_entry("latchOnPress", ACTION_FIELD_LATCH_ON_PRESS),
    lookup_entry("", 0),
];
#[inline]
fn report_mismatch(strict: u32) -> ParseStatus {
    if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
        ParseStatus::Fatal
    } else {
        ParseStatus::Recoverable
    }
}
#[inline]
fn report_format_version_mismatch(strict: u32) -> ParseStatus {
    if strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
        ParseStatus::Fatal
    } else {
        ParseStatus::Success
    }
}
#[inline]
fn report_illegal(strict: u32) -> ParseStatus {
    if strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
        ParseStatus::Fatal
    } else {
        ParseStatus::Success
    }
}

fn check_boolean_flag(
    ctx: &XkbContext,
    strict: u32,
    flag: ActionFlags,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut ActionFlags,
) -> ParseStatus {
    if array_ndx.is_some() {
        return report_mismatch(strict);
    }
    let set = some_or_false!(expr_resolve_boolean(ctx, value), report_mismatch(strict));
    flags_inout.set(flag, set);
    ParseStatus::Success
}
fn check_boolean_flag_feature(
    ctx: &XkbContext,
    strict: u32,
    flag: ActionFlags,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut ActionFlags,
    feature_enabled: bool,
) -> ParseStatus {
    if feature_enabled {
        check_boolean_flag(ctx, strict, flag, array_ndx, value, flags_inout)
    } else {
        report_format_version_mismatch(strict)
    }
}
fn check_modifier_field(
    ctx: &XkbContext,
    strict: u32,
    mods: &XkbModSet,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut u32,
    mods_rtrn: &mut u32,
) -> ParseStatus {
    if array_ndx.is_some() {
        return report_mismatch(strict);
    }
    if let ExprKind::Ident(ident) = value {
        let ident = *ident;
        let val_str: &str = ctx.atom_text(ident);
        if !val_str.is_empty()
            && (val_str.eq_ignore_ascii_case("usemodmapmods")
                || val_str.eq_ignore_ascii_case("modmapmods"))
        {
            *mods_rtrn = 0;
            *flags_inout |= ActionFlags::MODS_LOOKUP_MODMAP.bits();
            return ParseStatus::Success;
        }
    }
    let resolved_mods = some_or_false!(
        expr_resolve_mod_mask(ctx, value, MOD_BOTH, mods),
        report_mismatch(strict)
    );
    *mods_rtrn = resolved_mods;
    *flags_inout &= !ActionFlags::MODS_LOOKUP_MODMAP.bits();
    ParseStatus::Success
}
static LOCK_WHICH: [LookupEntry; 5] = [
    lookup_entry("both", 0),
    lookup_entry("lock", ActionFlags::LOCK_NO_UNLOCK.bits()),
    lookup_entry(
        "neither",
        ActionFlags::LOCK_NO_LOCK.bits() | ActionFlags::LOCK_NO_UNLOCK.bits(),
    ),
    lookup_entry("unlock", ActionFlags::LOCK_NO_LOCK.bits()),
    lookup_entry("", 0),
];
fn check_affect_field(
    ctx: &XkbContext,
    strict: u32,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    flags_inout: &mut ActionFlags,
) -> ParseStatus {
    if array_ndx.is_some() {
        return report_mismatch(strict);
    }
    let flags = some_or_false!(
        expr_resolve_enum(ctx, value, &LOCK_WHICH),
        report_mismatch(strict)
    );
    *flags_inout &= !(ActionFlags::LOCK_NO_LOCK | ActionFlags::LOCK_NO_UNLOCK);
    *flags_inout |= ActionFlags::from_bits_retain(flags);
    ParseStatus::Success
}
fn handle_set_latch_lock_mods(
    keymap_info: &mut XkbKeymapInfo<'_>,
    mods: &XkbModSet,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> ParseStatus {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let is_set_or_latch = matches!(action, XkbAction::ModSet(_) | XkbAction::ModLatch(_));
    let is_latch = matches!(action, XkbAction::ModLatch(_));
    let is_lock = matches!(action, XkbAction::ModLock(_));
    let act = match action {
        XkbAction::ModSet(ref mut m)
        | XkbAction::ModLatch(ref mut m)
        | XkbAction::ModLock(ref mut m) => m,
        _ => return report_illegal(keymap_info.strict),
    };
    if field == ACTION_FIELD_MODIFIERS {
        let mut raw_flags = act.flags.bits();
        let ret = check_modifier_field(
            ctx,
            keymap_info.strict,
            mods,
            array_ndx,
            value,
            &mut raw_flags,
            &mut act.mods.mods,
        );
        act.flags = ActionFlags::from_bits_retain(raw_flags);
        return ret;
    }
    if is_lock && field == ACTION_FIELD_AFFECT {
        return check_affect_field(ctx, keymap_info.strict, array_ndx, value, &mut act.flags);
    }
    let (flag, enabled) = match field {
        ACTION_FIELD_UNLOCK_ON_PRESS => (
            ActionFlags::UNLOCK_ON_PRESS,
            keymap_info.features.mods_unlock_on_press,
        ),
        ACTION_FIELD_CLEAR_LOCKS if is_set_or_latch => (ActionFlags::LOCK_CLEAR, true),
        ACTION_FIELD_LATCH_TO_LOCK if is_latch => (ActionFlags::LATCH_TO_LOCK, true),
        ACTION_FIELD_LATCH_ON_PRESS if is_latch => (
            ActionFlags::LATCH_ON_PRESS,
            keymap_info.features.mods_latch_on_press,
        ),
        _ => return report_illegal(keymap_info.strict),
    };
    check_boolean_flag_feature(
        ctx,
        keymap_info.strict,
        flag,
        array_ndx,
        value,
        &mut act.flags,
        enabled,
    )
}
fn check_group_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    array_ndx: Option<&ExprKind>,
    mut value: ActionValue<'_>,
    flags_inout: &mut u32,
    group_rtrn: &mut i32,
) -> ParseStatus {
    let mut idx: u32 = 0;
    let mut flags: u32 = *flags_inout;
    if array_ndx.is_some() {
        return report_mismatch(keymap_info.strict);
    }
    let is_unary = matches!(
        value.get(),
        ExprKind::Unary {
            op: UnaryOp::Negate | UnaryOp::Plus,
            ..
        }
    );
    let is_negate = if is_unary {
        flags &= !ActionFlags::ABSOLUTE_SWITCH.bits();
        let (is_negate, child) = value.unary_child().unwrap();
        value = child;
        is_negate
    } else {
        flags |= ActionFlags::ABSOLUTE_SWITCH.bits();
        false
    };
    let spec_holder = value.get();
    let absolute: bool = flags & ActionFlags::ABSOLUTE_SWITCH.bits() != 0;
    let mut pending: bool = false;
    let ret = expr_resolve_group(keymap_info, spec_holder, absolute, &mut idx, &mut pending);
    if ret != ParseStatus::Success && !pending {
        return ret;
    }
    if pending {
        flags |= ActionFlags::PENDING_COMPUTATION.bits();
        *group_rtrn = add_pending_computation(keymap_info, value.take()) as i32;
    } else {
        flags &= !ActionFlags::PENDING_COMPUTATION.bits();
        if flags & ActionFlags::ABSOLUTE_SWITCH.bits() == 0 {
            *group_rtrn = idx as i32;
            if is_negate {
                *group_rtrn = -*group_rtrn;
            }
        } else {
            *group_rtrn = idx.wrapping_sub(1_u32) as i32;
        }
    }
    *flags_inout = flags;
    ParseStatus::Success
}
fn handle_set_latch_lock_group(
    keymap_info: &mut XkbKeymapInfo<'_>,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> ParseStatus {
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    if field == ACTION_FIELD_GROUP {
        let act = match action {
            XkbAction::GroupSet(ref mut g)
            | XkbAction::GroupLatch(ref mut g)
            | XkbAction::GroupLock(ref mut g) => g,
            _ => return report_illegal(keymap_info.strict),
        };
        let mut raw_flags = act.flags.bits();
        let ret = check_group_field(
            keymap_info,
            array_ndx,
            value,
            &mut raw_flags,
            &mut act.group,
        );
        act.flags = ActionFlags::from_bits_retain(raw_flags);
        return ret;
    }
    let value = value.get();
    let is_set_or_latch = matches!(action, XkbAction::GroupSet(_) | XkbAction::GroupLatch(_));
    let is_latch = matches!(action, XkbAction::GroupLatch(_));
    let is_lock = matches!(action, XkbAction::GroupLock(_));
    let act = match action {
        XkbAction::GroupSet(ref mut g)
        | XkbAction::GroupLatch(ref mut g)
        | XkbAction::GroupLock(ref mut g) => g,
        _ => return report_illegal(keymap_info.strict),
    };
    let (flag, enabled) = match field {
        ACTION_FIELD_CLEAR_LOCKS if is_set_or_latch => (ActionFlags::LOCK_CLEAR, true),
        ACTION_FIELD_LATCH_TO_LOCK if is_latch => (ActionFlags::LATCH_TO_LOCK, true),
        ACTION_FIELD_LOCK_ON_RELEASE if is_lock => (
            ActionFlags::LOCK_ON_RELEASE,
            keymap_info.features.group_lock_on_release,
        ),
        _ => return report_illegal(keymap_info.strict),
    };
    check_boolean_flag_feature(
        ctx,
        keymap_info.strict,
        flag,
        array_ndx,
        value,
        &mut act.flags,
        enabled,
    )
}

fn handle_set_lock_controls(
    keymap_info: &mut XkbKeymapInfo<'_>,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> ParseStatus {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let is_lock = matches!(action, XkbAction::CtrlLock(_));
    let act = match action {
        XkbAction::CtrlSet(ref mut c) | XkbAction::CtrlLock(ref mut c) => c,
        _ => return report_illegal(keymap_info.strict),
    };
    if field == ACTION_FIELD_CONTROLS {
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        let offset: u8 = keymap_info.features.controls_name_offset;
        let mask = some_or_false!(
            expr_resolve_mask(ctx, value, &CTRL_MASK_NAMES[offset as usize..]),
            report_mismatch(keymap_info.strict)
        );
        act.ctrls = ControlsFlags::from_bits_retain(mask);
        return ParseStatus::Success;
    } else if is_lock && field == ACTION_FIELD_AFFECT {
        return check_affect_field(ctx, keymap_info.strict, array_ndx, value, &mut act.flags);
    }
    report_illegal(keymap_info.strict)
}

fn handle_private(
    keymap_info: &mut XkbKeymapInfo<'_>,
    action: &mut XkbAction,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> ParseStatus {
    let value = value.get();
    let ctx: &XkbContext = &keymap_info.keymap.ctx;
    let act = match action {
        XkbAction::Private(ref mut p) => p,
        _ => return report_illegal(keymap_info.strict),
    };
    if field == ACTION_FIELD_TYPE {
        if array_ndx.is_some() {
            return report_mismatch(keymap_info.strict);
        }
        let type_0 = some_or_false!(
            expr_resolve_integer(ctx, value),
            report_mismatch(keymap_info.strict)
        );
        if !(0_i64..=255_i64).contains(&type_0) {
            return report_mismatch(keymap_info.strict);
        }
        return ParseStatus::Success;
    } else if field == ACTION_FIELD_DATA {
        if let Some(array_ndx) = array_ndx {
            let ndx = some_or_false!(
                expr_resolve_integer(ctx, array_ndx),
                report_mismatch(keymap_info.strict)
            );
            if ndx < 0_i64 || ndx as usize >= std::mem::size_of::<[u8; 7]>() {
                return report_mismatch(keymap_info.strict);
            }
            let datum = some_or_false!(
                expr_resolve_integer(ctx, value),
                report_mismatch(keymap_info.strict)
            );
            if !(0_i64..=255_i64).contains(&datum) {
                return report_mismatch(keymap_info.strict);
            }
            act.data[ndx as usize] = datum as u8;
            return ParseStatus::Success;
        } else {
            let val = some_or_false!(
                expr_resolve_string(value),
                report_mismatch(keymap_info.strict)
            );
            let str_bytes: &str = ctx.atom_text(val);
            let len: usize = str_bytes.len();
            if len < 1_usize || len > std::mem::size_of::<[u8; 7]>() {
                return report_mismatch(keymap_info.strict);
            }
            act.data = [0u8; 7];
            act.data[..len].copy_from_slice(&str_bytes.as_bytes()[..len]);
            return ParseStatus::Success;
        }
    }
    report_illegal(keymap_info.strict)
}

fn handle_action_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    mods: &XkbModSet,
    action: &mut XkbAction,
    action_type: u32,
    field: u32,
    array_ndx: Option<&ExprKind>,
    value: ActionValue<'_>,
) -> ParseStatus {
    match action_type {
        ACTION_TYPE_MOD_SET..=ACTION_TYPE_MOD_LOCK => {
            handle_set_latch_lock_mods(keymap_info, mods, action, field, array_ndx, value)
        }
        ACTION_TYPE_GROUP_SET..=ACTION_TYPE_GROUP_LOCK => {
            handle_set_latch_lock_group(keymap_info, action, field, array_ndx, value)
        }
        ACTION_TYPE_VOID
        | ACTION_TYPE_PTR_MOVE
        | ACTION_TYPE_PTR_BUTTON
        | ACTION_TYPE_PTR_LOCK
        | ACTION_TYPE_PTR_DEFAULT
        | ACTION_TYPE_TERMINATE
        | ACTION_TYPE_SWITCH_VT
        | ACTION_TYPE_REDIRECT_KEY => ParseStatus::Success,
        ACTION_TYPE_CTRL_SET | ACTION_TYPE_CTRL_LOCK => {
            handle_set_lock_controls(keymap_info, action, field, array_ndx, value)
        }
        ACTION_TYPE_PRIVATE => handle_private(keymap_info, action, field, array_ndx, value),
        _ => report_illegal(keymap_info.strict),
    }
}

pub(crate) fn handle_action_def(
    keymap_info: &mut XkbKeymapInfo<'_>,
    info: &mut ActionsInfo,
    mods: &XkbModSet,
    def: &mut ExprKind,
    action: &mut XkbAction,
) -> ParseStatus {
    let ExprKind::Action { name, args } = def else {
        return ParseStatus::Fatal;
    };
    let handler_type = lookup_string(&ACTION_TYPE_NAMES, keymap_info.keymap.ctx.atom_text(*name))
        .unwrap_or(ACTION_TYPE_UNKNOWN);
    if handler_type == ACTION_TYPE_UNKNOWN && keymap_info.strict & PARSER_NO_UNKNOWN_ACTION != 0 {
        return ParseStatus::Fatal;
    }
    *action = info[handler_type as usize];
    if handler_type == ACTION_TYPE_UNSUPPORTED_LEGACY {
        *action = XkbAction::None;
    }
    let mut ret: ParseStatus = ParseStatus::Success;
    let const_true = ExprKind::Boolean(true);
    let const_false = ExprKind::Boolean(false);
    for arg in args.iter_mut() {
        let av: ActionValue<'_>;
        let field_ref: &ExprKind;
        if let ExprKind::Binary {
            op: BinaryOp::Assign,
            left,
            right,
        } = arg
        {
            field_ref = left.as_deref().unwrap();
            av = ActionValue::Boxed(right);
        } else if let ExprKind::Unary {
            op: UnaryOp::Not | UnaryOp::Invert,
            child,
        } = arg
        {
            field_ref = child.as_deref().unwrap();
            av = ActionValue::Borrowed(&const_false);
        } else {
            field_ref = &*arg;
            av = ActionValue::Borrowed(&const_true);
        }
        let Some(lhs) = expr_resolve_lhs(field_ref) else {
            return ParseStatus::Fatal;
        };
        let elem_rtrn = keymap_info.keymap.ctx.atom_text(lhs.element);
        let field_rtrn = keymap_info.keymap.ctx.atom_text(lhs.field);
        if !elem_rtrn.is_empty() {
            return ParseStatus::Fatal;
        }
        let Some(field_ndx) = lookup_string(&FIELD_STRINGS, field_rtrn) else {
            if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
                return ParseStatus::Fatal;
            }
            continue;
        };
        let parse_status = handle_action_field(
            keymap_info,
            mods,
            action,
            handler_type,
            field_ndx,
            lhs.index,
            av,
        );
        match parse_status {
            ParseStatus::Fatal => return ParseStatus::Fatal,
            ParseStatus::Recoverable => {
                ret = ParseStatus::Recoverable;
            }
            _ => {}
        }
    }
    if matches!(action, XkbAction::Unknown) {
        ParseStatus::Recoverable
    } else {
        ret
    }
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn set_default_action_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    info: &mut ActionsInfo,
    mods: &mut XkbModSet,
    elem: &str,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_rtrn: &mut Option<ExprKind>,
    merge: MergeMode,
) -> ParseStatus {
    let av = ActionValue::Owned(value_rtrn);
    let Some(action) = lookup_string(&ACTION_TYPE_NAMES, elem) else {
        return if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION != 0 {
            ParseStatus::Fatal
        } else {
            ParseStatus::Recoverable
        };
    };
    let Some(action_field) = lookup_string(&FIELD_STRINGS, field) else {
        return if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
            ParseStatus::Fatal
        } else {
            ParseStatus::Recoverable
        };
    };
    let into: &mut XkbAction = &mut info[action as usize];
    let mut from: XkbAction = *into;
    let ret = handle_action_field(
        keymap_info,
        mods,
        &mut from,
        action,
        action_field,
        array_ndx,
        av,
    );
    if ret != ParseStatus::Success {
        return ret;
    }
    if !action_equal(into, &from) {
        let replace: bool = merge != MergeMode::Augment;
        if replace {
            *into = from;
        }
    }
    ParseStatus::Success
}
