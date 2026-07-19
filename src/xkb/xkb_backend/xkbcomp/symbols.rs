use super::super::context::xkb_atom_intern_ref;
pub use super::super::keymap::{XkbLevelsSameActions, XkbLevelsSameSyms, XkbModNameToIndex};
use super::super::keysym::xkb_keysym_is_keypad;
use super::super::keysym_case_mappings::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
use super::super::text::{
    ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames,
    useModMapValueNames, LookupString, ModMaskText, SIMatchText,
};
use super::*;

pub use super::super::shared_ast_types::{
    InterpDef, LedMapDef, ModMapDef, ReportBadField, ReportNotArray, SymbolsDef,
};
pub use super::super::shared_types::{
    MAX_ACTIONS_PER_LEVEL, MOD_REAL_MASK_ALL, XKB_MAX_LEDS, XKB_MOD_NONE, XKB_OVERLAY_INVALID,
};
use super::super::text::ModIndexText;

pub use super::expr::{ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField};
use super::expr::{ExprResolveGroupMask, ExprResolveMask, ExprResolveMod};

pub struct SymbolsInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub explicit_group: u32,
    pub max_groups: u32,
    pub keys: Vec<KeyInfo>,
    pub default_key: KeyInfo,
    pub default_actions: ActionsInfo,
    pub group_names: Vec<u32>,
    pub modmaps: Vec<ModMapEntry>,
    pub mods: xkb_mod_set,
    pub star_atom: u32,
}
#[derive(Copy, Clone)]
pub struct ModMapEntry {
    pub merge: merge_mode,
    pub haveSymbol: bool,
    pub modifier: u32,
    /// keyName (atom) when !haveSymbol, keySym when haveSymbol
    pub u: u32,
}
#[derive(Clone)]
pub struct KeyInfo {
    pub name: u32,
    pub vmodmap: u32,
    pub default_type: u32,
    pub out_of_range_group_number: u32,
    pub groups: Vec<GroupInfo>,
    pub out_of_range_group_policy: u32,
    pub defined: key_field,
    pub merge: merge_mode,
    pub repeat: key_repeat,
    pub out_of_range_pending_group: bool,
    pub overlays_clear: bool,
    pub overlays: xkb_overlay_mask_t,
    pub overlay_keys: Vec<u32>,
}
pub type key_repeat = u32;
pub const _KEY_REPEAT_NUM_ENTRIES: key_repeat = 3;
pub const KEY_REPEAT_NO: key_repeat = 2;
pub const KEY_REPEAT_YES: key_repeat = 1;
pub const KEY_REPEAT_UNDEFINED: key_repeat = 0;
pub type key_field = u32;
pub const KEY_FIELD_ALL: key_field = 31;
pub const KEY_FIELD_OVERLAY: key_field = 16;
pub const KEY_FIELD_VMODMAP: key_field = 8;
pub const KEY_FIELD_GROUPINFO: key_field = 4;
pub const KEY_FIELD_DEFAULT_TYPE: key_field = 2;
pub const KEY_FIELD_REPEAT: key_field = 1;
#[derive(Clone, Default)]
pub struct GroupInfo {
    pub levels: Vec<xkb_level>,
    pub defined: group_field,
    pub type_0: u32,
}

pub type group_field = u32;
pub const GROUP_FIELD_TYPE: group_field = 4;
pub const GROUP_FIELD_ACTS: group_field = 2;
pub const GROUP_FIELD_SYMS: group_field = 1;

impl KeyInfo {
    pub fn new_zeroed() -> Self {
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
    pub fn new(ki: &mut xkb_keymap_info<'_>) -> Self {
        let star_atom = xkb_atom_intern_ref(&mut ki.keymap_mut().ctx, b"*");
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: Vec::with_capacity(256),
            default_key: KeyInfo::new_zeroed(),
            default_actions: ActionsInfo {
                actions: [xkb_action::None; 21],
            },
            group_names: Vec::new(),
            modmaps: Vec::new(),
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0_u32,
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
fn is_action_list_value(value: &ExprDef) -> bool {
    if let ExprKind::ActionList { actions } = &value.kind {
        if let Some(first) = actions.first() {
            // If the first inner node is an ActionList (actions for one level) or
            // Action (single action), it's action data. KeysymList means keysym data.
            matches!(
                first.kind,
                ExprKind::ActionList { .. } | ExprKind::Action { .. }
            )
        } else {
            // Empty ActionList — treat as actions
            true
        }
    } else {
        false
    }
}

/// Extract child expressions from an ActionList container node, or return a single-element slice.
fn collect_expr_list(container: &ExprDef) -> &[ExprDef] {
    match &container.kind {
        ExprKind::ActionList { actions } => actions.as_slice(),
        _ => std::slice::from_ref(container),
    }
}

/// Extract child expressions from an ActionList container node as mutable slice.
fn collect_expr_list_mut(container: &mut ExprDef) -> &mut [ExprDef] {
    if matches!(container.kind, ExprKind::ActionList { .. }) {
        if let ExprKind::ActionList { ref mut actions } = container.kind {
            return actions.as_mut_slice();
        }
        unreachable!()
    } else {
        std::slice::from_mut(container)
    }
}

fn InitGroupInfo(groupi: &mut GroupInfo) {
    *groupi = GroupInfo::default();
}
fn ClearGroupInfo(groupi: &mut GroupInfo) {
    groupi.levels.clear();
}
fn InitKeyInfo_with_atom(keyi: &mut KeyInfo, star_atom: u32) {
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
fn InitKeyInfo(ctx: &mut xkb_context, keyi: &mut KeyInfo) {
    InitKeyInfo_with_atom(keyi, xkb_atom_intern_ref(ctx, b"*"));
}
fn ClearKeyInfo(keyi: &mut KeyInfo) {
    for groupi in keyi.groups.iter_mut() {
        ClearGroupInfo(groupi);
    }
    keyi.groups.clear();
    keyi.overlay_keys.clear();
}
fn InitSymbolsInfo(
    info: &mut SymbolsInfo,
    ki: &mut xkb_keymap_info<'_>,
    include_depth: u32,
    mods: &xkb_mod_set,
) {
    info.include_depth = include_depth;
    info.explicit_group = XKB_LAYOUT_INVALID;
    info.max_groups = ki.features.max_groups;
    InitKeyInfo(ki.ctx_mut(), &mut info.default_key);
    InitActionsInfo(ki.keymap_ref(), &mut info.default_actions);
    InitVMods(&mut info.mods, mods, include_depth > 0_u32);
}
fn ClearSymbolsInfo(info: &mut SymbolsInfo) {
    info.name = None;
    for keyi in info.keys.iter_mut() {
        ClearKeyInfo(keyi);
    }
    info.keys.clear();
    info.group_names.clear();
    info.modmaps.clear();
    ClearKeyInfo(&mut info.default_key);
}
fn KeyInfoText<'a>(ctx: &'a xkb_context, keyi: &KeyInfo) -> &'a str {
    xkb_atom_text(&ctx.atom_table, keyi.name)
}
fn MergeGroups(
    ki: &xkb_keymap_info<'_>,
    into: &mut GroupInfo,
    from: &mut GroupInfo,
    clobber: bool,
    report: bool,
    group: u32,
    key_name: u32,
) -> bool {
    if into.type_0 != from.type_0 && (from.type_0 != XKB_ATOM_NONE) {
        if into.type_0 == XKB_ATOM_NONE {
            into.type_0 = from.type_0;
        } else {
            let use_0: u32 = if clobber { from.type_0 } else { into.type_0 };
            let ignore: u32 = if clobber { into.type_0 } else { from.type_0 };
            if report {
                log::warn!("[XKB-{:03}] Multiple definitions for group {} type of key <{}>; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS as i32,
                        group.wrapping_add(1_u32),
                        xkb_atom_text(&ki.ctx().atom_table, key_name),
                        xkb_atom_text(&ki.ctx().atom_table, use_0),
                        xkb_atom_text(&ki.ctx().atom_table, ignore));
            }
            into.type_0 = use_0;
        }
    }
    into.defined = (into.defined | from.defined & GROUP_FIELD_TYPE) as group_field;
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
    let mut fromKeysymsCount: u32 = 0_u32;
    let mut fromActionsCount: u32 = 0_u32;
    let mut i: u32 = 0_u32;
    while i < levels_in_both {
        let intoLevel = &mut into.levels[i as usize];
        let fromLevel = &mut from.levels[i as usize];
        let fromHasNoKeysym: bool = fromLevel.syms.is_empty();
        let fromHasNoAction: bool = fromLevel.actions.is_empty();
        if !(fromHasNoKeysym && fromHasNoAction) {
            let intoHasNoKeysym: bool = intoLevel.syms.is_empty();
            let intoHasNoAction: bool = intoLevel.actions.is_empty();
            if intoHasNoKeysym && intoHasNoAction {
                // StealLevelInfo inlined
                intoLevel.syms = std::mem::take(&mut fromLevel.syms);
                intoLevel.actions = std::mem::take(&mut fromLevel.actions);
                fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                fromActionsCount = fromActionsCount.wrapping_add(1);
            } else {
                if !XkbLevelsSameSyms(fromLevel, intoLevel) {
                    if report && !(intoHasNoKeysym || fromHasNoKeysym) {
                        log::warn!("[XKB-{:03}] Multiple symbols for level {}/group {} on key <{}>; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_SYMBOL as i32,
                            i.wrapping_add(1_u32),
                            group.wrapping_add(1_u32),
                            xkb_atom_text(&ki.ctx().atom_table, key_name),
                            if clobber { "from" } else { "to" },
                            if clobber { "to" } else { "from" });
                    }
                    if !fromHasNoKeysym {
                        if clobber {
                            if !fromLevel.syms.is_empty() {
                                intoLevel.syms = std::mem::take(&mut fromLevel.syms);
                                fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                            }
                        } else if intoLevel.syms.is_empty() {
                            if !fromLevel.syms.is_empty() {
                                intoLevel.syms = std::mem::take(&mut fromLevel.syms);
                            }
                            fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                        }
                    }
                }
                if !XkbLevelsSameActions(intoLevel, fromLevel) {
                    if report && !(intoHasNoAction || fromHasNoAction) {
                        if intoLevel.actions.len() > 1 || fromLevel.actions.len() > 1 {
                            log::warn!("[XKB-{:03}] Multiple actions for level {}/group {} on key <{}>; {}\n",
                                XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                                i.wrapping_add(1_u32),
                                group.wrapping_add(1_u32),
                                xkb_atom_text(&ki.ctx().atom_table, key_name),
                                if clobber { "Using from, ignoring to" } else { "Using to, ignoring from" });
                        } else {
                            let use_action: &xkb_action = if clobber {
                                &fromLevel.actions[0]
                            } else {
                                &intoLevel.actions[0]
                            };
                            let ignore_action: &xkb_action = if clobber {
                                &intoLevel.actions[0]
                            } else {
                                &fromLevel.actions[0]
                            };
                            log::warn!("[XKB-{:03}] Multiple actions for level {}/group {} on key <{}>; Using {}, ignoring {}\n",
                                XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                                i.wrapping_add(1_u32),
                                group.wrapping_add(1_u32),
                                xkb_atom_text(&ki.ctx().atom_table, key_name),
                                ActionTypeText(use_action.action_type()),
                                ActionTypeText(ignore_action.action_type()));
                        }
                    }
                    if !fromHasNoAction {
                        if clobber {
                            if !fromLevel.actions.is_empty() {
                                intoLevel.actions = std::mem::take(&mut fromLevel.actions);
                                fromActionsCount = fromActionsCount.wrapping_add(1);
                            }
                        } else if intoLevel.actions.is_empty() {
                            if !fromLevel.actions.is_empty() {
                                intoLevel.actions = std::mem::take(&mut fromLevel.actions);
                            }
                            fromActionsCount = fromActionsCount.wrapping_add(1);
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let mut level_idx: u32 = levels_in_both;
    while level_idx < from.levels.len() as u32 {
        let level_val = from.levels[level_idx as usize].clone();
        into.levels.push(level_val);
        from.levels[level_idx as usize].syms.clear();
        from.levels[level_idx as usize].actions.clear();
        fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
        fromActionsCount = fromActionsCount.wrapping_add(1);
        level_idx = level_idx.wrapping_add(1);
    }
    if fromKeysymsCount != 0 {
        if fromKeysymsCount == into.levels.len() as u32 {
            into.defined = (into.defined & !(GROUP_FIELD_SYMS as i32) as u32) as group_field;
        }
        into.defined = (into.defined | from.defined & GROUP_FIELD_SYMS) as group_field;
    }
    if fromActionsCount != 0 {
        if fromActionsCount == into.levels.len() as u32 {
            into.defined = (into.defined & !(GROUP_FIELD_ACTS as i32) as u32) as group_field;
        }
        into.defined = (into.defined | from.defined & GROUP_FIELD_ACTS) as group_field;
    }
    true
}
fn UseNewKeyField(
    field: key_field,
    old: key_field,
    new: key_field,
    clobber: bool,
    report: bool,
    collide: &mut key_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            *collide = (*collide | field) as key_field;
        }
        return clobber;
    }
    false
}
fn overlays_get(info: &KeyInfo, bit: xkb_overlay_index_t, key_out: Option<&mut u32>) -> bool {
    if bit as i32
        >= (std::mem::size_of::<xkb_overlay_mask_t>()).wrapping_mul(8_usize) as xkb_overlay_index_t
            as i32
    {
        return false;
    }
    let mask: xkb_overlay_mask_t = (1_u32 << bit as i32) as xkb_overlay_mask_t;
    if info.overlays as i32 & mask as i32 == 0 {
        return false;
    }
    if let Some(key_out) = key_out {
        let low: xkb_overlay_mask_t =
            (info.overlays as u32 & (mask as u32).wrapping_sub(1_u32)) as xkb_overlay_mask_t;
        let index: usize = (low as u32).count_ones() as usize;
        *key_out = info.overlay_keys[index];
    }
    true
}
fn overlays_insert(keyi: &mut KeyInfo, bit: xkb_overlay_index_t, key: u32) -> bool {
    if bit as i32
        >= (std::mem::size_of::<xkb_overlay_mask_t>()).wrapping_mul(8_usize) as xkb_overlay_index_t
            as i32
    {
        return false;
    }
    let mask: xkb_overlay_mask_t = (1_u32 << bit as i32) as xkb_overlay_mask_t;
    if keyi.overlays as i32 & mask as i32 != 0 && !keyi.overlays_clear {
        // Bit already set — update existing entry
        let low: xkb_overlay_mask_t =
            (keyi.overlays as u32 & (mask as u32).wrapping_sub(1_u32)) as xkb_overlay_mask_t;
        let index: usize = (low as u32).count_ones() as usize;
        keyi.overlay_keys[index] = key;
        if key == XKB_KEYCODE_INVALID && keyi.overlay_keys.len() == 1 {
            keyi.overlays_clear = true;
        }
        return true;
    }
    // New bit
    let new_overlays: xkb_overlay_mask_t =
        (keyi.overlays as i32 | mask as i32) as xkb_overlay_mask_t;
    let low: xkb_overlay_mask_t =
        (new_overlays as u32 & (mask as u32).wrapping_sub(1_u32)) as xkb_overlay_mask_t;
    let index: usize = (low as u32).count_ones() as usize;

    if keyi.overlays == 0 || keyi.overlays_clear as i32 != 0 && key == XKB_KEYCODE_INVALID {
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
    ki: &xkb_keymap_info<'_>,
    into: &mut KeyInfo,
    from: &mut KeyInfo,
    mut clobber: bool,
    report: bool,
    collide: &mut key_field,
) -> bool {
    if from.defined as i32 & KEY_FIELD_OVERLAY as i32 != 0 {
        if into.defined as i32 & KEY_FIELD_OVERLAY as i32 == 0 {
            // into has no overlays, take from's
            into.overlays = from.overlays;
            into.overlay_keys = std::mem::take(&mut from.overlay_keys);
            into.defined |= KEY_FIELD_OVERLAY as i32 as key_field;
        } else if into.overlays_clear as i32 != 0 && from.overlays_clear as i32 != 0 {
            into.overlays = (into.overlays as i32 | from.overlays as i32) as xkb_overlay_mask_t;
        } else if ki.features.overlapping_overlays {
            // Complex merge with overlapping overlays
            let result_mask: xkb_overlay_mask_t =
                (into.overlays as i32 | from.overlays as i32) as xkb_overlay_mask_t;
            let count: xkb_overlay_index_t =
                (result_mask as u32).count_ones() as xkb_overlay_index_t;
            if count as i32 == 0_i32 {
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
            let mut remaining: xkb_overlay_mask_t = from.overlays;
            let mut src_idx: usize = 0;
            while remaining != 0 {
                let lsb: xkb_overlay_mask_t = (remaining as i32
                    & (!(remaining as i32) as u32).wrapping_add(1_u32) as xkb_overlay_mask_t as i32)
                    as xkb_overlay_mask_t;
                let bit: xkb_overlay_index_t =
                    ((lsb as u32).wrapping_sub(1_u32).count_ones()) as xkb_overlay_index_t;
                remaining = (remaining as i32 & !(lsb as i32)) as xkb_overlay_mask_t;
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
                    if report {
                        *collide = (*collide | KEY_FIELD_OVERLAY) as key_field;
                    }
                }
                if (!conflict || clobber as i32 != 0) && !overlays_insert(into, bit, src_key) {
                    return false;
                }
            }
            if swapped {
                // We swapped into/from, so move dest data back to into
                std::mem::swap(into, from);
            }
        } else {
            if into.overlays as i32 == from.overlays as i32
                && into.overlays_clear as i32 == from.overlays_clear as i32
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
            if into.overlays as i32 & from.overlays as i32 == 0 {
                if into.overlays_clear {
                    into.overlays = from.overlays;
                    into.overlays_clear = from.overlays_clear;
                    into.overlay_keys = std::mem::take(&mut from.overlay_keys);
                    return true;
                } else if from.overlays_clear {
                    return true;
                }
            }
            if report {
                *collide = (*collide | KEY_FIELD_OVERLAY) as key_field;
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
fn MergeKeys(
    ki: &xkb_keymap_info<'_>,
    info: &SymbolsInfo,
    into: &mut KeyInfo,
    from: &mut KeyInfo,
    same_file: bool,
) -> bool {
    let mut i: u32;

    let mut collide: key_field = 0 as key_field;
    let verbosity: i32 = xkb_context_get_log_verbosity(ki.ctx());
    let clobber: bool = from.merge as i32 != MERGE_AUGMENT as i32;
    let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
    if from.merge as i32 == MERGE_REPLACE as i32 {
        ClearKeyInfo(into);
        std::mem::swap(into, from);
        InitKeyInfo_with_atom(from, info.star_atom);
        return true;
    }
    let groups_in_both: u32 = (if into.groups.len() < from.groups.len() {
        into.groups.len()
    } else {
        from.groups.len()
    }) as u32;
    i = 0_u32;
    while i < groups_in_both {
        MergeGroups(
            ki,
            &mut into.groups[i as usize],
            &mut from.groups[i as usize],
            clobber,
            report,
            i,
            into.name,
        );
        i = i.wrapping_add(1);
    }
    i = groups_in_both;
    while i < from.groups.len() as u32 {
        let group_val = std::mem::take(&mut from.groups[i as usize]);
        into.groups.push(group_val);
        i = i.wrapping_add(1);
    }
    if UseNewKeyField(
        KEY_FIELD_VMODMAP,
        into.defined,
        from.defined,
        clobber,
        report,
        &mut collide,
    ) {
        into.vmodmap = from.vmodmap;
        into.defined |= KEY_FIELD_VMODMAP as i32 as key_field;
    }
    if UseNewKeyField(
        KEY_FIELD_REPEAT,
        into.defined,
        from.defined,
        clobber,
        report,
        &mut collide,
    ) {
        into.repeat = from.repeat;
        into.defined |= KEY_FIELD_REPEAT as i32 as key_field;
    }
    if UseNewKeyField(
        KEY_FIELD_DEFAULT_TYPE,
        into.defined,
        from.defined,
        clobber,
        report,
        &mut collide,
    ) {
        into.default_type = from.default_type;
        into.defined |= KEY_FIELD_DEFAULT_TYPE as i32 as key_field;
    }
    if UseNewKeyField(
        KEY_FIELD_GROUPINFO,
        into.defined,
        from.defined,
        clobber,
        report,
        &mut collide,
    ) {
        into.out_of_range_pending_group = from.out_of_range_pending_group;
        into.out_of_range_group_policy = from.out_of_range_group_policy;
        into.out_of_range_group_number = from.out_of_range_group_number;
        into.defined |= KEY_FIELD_GROUPINFO as i32 as key_field;
    }
    if !merge_overlays(ki, into, from, clobber, report, &mut collide) {
        return false;
    }
    if collide as u64 != 0 {
        log::warn!("[XKB-{:03}] Symbol map for key <{}> redefined; Using {} definition for conflicting fields\n",
            XKB_WARNING_CONFLICTING_KEY_FIELDS as i32,
            xkb_atom_text(&ki.ctx().atom_table, into.name),
            if clobber { "first" } else { "last" });
    }
    ClearKeyInfo(from);
    InitKeyInfo_with_atom(from, info.star_atom);
    true
}
fn AddKeySymbols(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
    same_file: bool,
) -> bool {
    // XkbResolveKeyAlias inlined
    {
        let keymap = ki.keymap_ref();
        let name = keyi.name;
        if (name as usize) < keymap.key_names.len() {
            let match_0: KeycodeMatch = keymap.key_names[name as usize];
            if match_0.found as i32 != 0 && match_0.is_alias as i32 != 0 {
                keyi.name = match_0.index;
            }
        }
    }
    for i in 0..info.keys.len() {
        if info.keys[i].name == keyi.name {
            let mut existing = std::mem::replace(&mut info.keys[i], KeyInfo::new_zeroed());
            let result = MergeKeys(ki, info, &mut existing, keyi, same_file);
            info.keys[i] = existing;
            return result;
        }
    }
    // Move keyi's data into the keys vec
    let moved = std::mem::replace(keyi, KeyInfo::new_zeroed());
    info.keys.push(moved);
    InitKeyInfo_with_atom(keyi, info.star_atom);
    true
}
fn AddModMapEntry(ki: &xkb_keymap_info<'_>, info: &mut SymbolsInfo, new: &ModMapEntry) -> bool {
    let clobber: bool = new.merge != MERGE_AUGMENT;
    let ctx = ki.ctx();
    let mods = &info.mods;
    for old in info.modmaps.iter_mut() {
        if new.haveSymbol as i32 != old.haveSymbol as i32
            || new.haveSymbol as i32 != 0 && new.u != old.u
            || !new.haveSymbol && new.u != old.u
        {
            continue;
        }
        if new.modifier == old.modifier {
            return true;
        }
        let use_0: u32 = if clobber as i32 != 0 {
            new.modifier
        } else {
            old.modifier
        };
        let ignore: u32 = if clobber as i32 != 0 {
            old.modifier
        } else {
            new.modifier
        };
        if new.haveSymbol {
            log::warn!("[XKB-{:03}] Symbol \"{}\" added to modifier map for multiple modifiers; Using {}, ignoring {}\n",
                XKB_WARNING_CONFLICTING_MODMAP as i32,
                KeysymText(new.u),
                ModIndexText(ctx, mods, use_0),
                ModIndexText(ctx, mods, ignore));
        } else {
            log::warn!("[XKB-{:03}] Key \"<{}>\" added to modifier map for multiple modifiers; Using {}, ignoring {}\n",
                XKB_WARNING_CONFLICTING_MODMAP as i32,
                xkb_atom_text(&ctx.atom_table, new.u),
                ModIndexText(ctx, mods, use_0),
                ModIndexText(ctx, mods, ignore));
        }
        old.modifier = use_0;
        return true;
    }
    info.modmaps.push(*new);
    true
}
fn MergeIncludedSymbols(
    ki: &mut xkb_keymap_info<'_>,
    into: &mut SymbolsInfo,
    from: &mut SymbolsInfo,
    merge: merge_mode,
) {
    if from.errorCount > 0_i32 {
        into.errorCount += from.errorCount;
        return;
    }
    MergeModSets(ki.ctx_mut(), &mut into.mods, &from.mods, merge);
    if into.name.is_none() {
        into.name = from.name.take();
    }
    let group_names_in_both: u32 = (if into.group_names.len() < from.group_names.len() {
        into.group_names.len()
    } else {
        from.group_names.len()
    }) as u32;
    let mut i: u32 = 0_u32;
    while i < group_names_in_both {
        if ((&from.group_names)[i as usize] != 0)
            && !(merge == MERGE_AUGMENT && (&into.group_names)[i as usize] != 0)
        {
            (&mut into.group_names)[i as usize] = (&from.group_names)[i as usize];
        }
        i = i.wrapping_add(1);
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
            keyi.merge = merge as merge_mode;
            if !AddKeySymbols(ki, into, keyi, false) {
                into.errorCount += 1;
            }
        }
    }
    if into.modmaps.is_empty() {
        std::mem::swap(&mut into.modmaps, &mut from.modmaps);
    } else {
        for mm in from.modmaps.iter_mut() {
            mm.merge = merge;
            if !AddModMapEntry(ki, into, mm) {
                into.errorCount += 1;
            }
        }
    };
}
fn HandleIncludeSymbols(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    include: &mut IncludeStmt,
) -> bool {
    let mut included = SymbolsInfo::new(ki);
    if ExceedsIncludeMaxDepth(info.include_depth) {
        info.errorCount += 10_i32;
        return false;
    }
    InitSymbolsInfo(
        &mut included,
        ki,
        info.include_depth.wrapping_add(1_u32),
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

        let file: Option<Box<XkbFile>> = ProcessIncludeFile(ki.ctx_mut(), stmt, FILE_TYPE_SYMBOLS);
        let Some(mut file) = file else {
            info.errorCount += 10_i32;
            ClearSymbolsInfo(&mut included);
            return false;
        };
        InitSymbolsInfo(
            &mut next_incl,
            ki,
            info.include_depth.wrapping_add(1_u32),
            &included.mods,
        );
        if !stmt.modifier.is_empty() {
            next_incl.explicit_group = (stmt.modifier.parse::<i32>().unwrap_or(0) - 1_i32) as u32;
            if next_incl.explicit_group >= info.max_groups {
                log::error!("[XKB-{:03}] Cannot set explicit group to {} - must be between 1..{}; Ignoring group number\n",
                    { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                    next_incl.explicit_group.wrapping_add(1_u32),
                    info.max_groups);
                next_incl.explicit_group = info.explicit_group;
            }
        } else if ki.keymap.num_groups != 0_u32 && next_incl.include_depth == 1_u32 {
            next_incl.explicit_group = 0_u32;
        } else {
            next_incl.explicit_group = info.explicit_group;
        }
        HandleSymbolsFile(ki, &mut next_incl, &mut file);
        MergeIncludedSymbols(ki, &mut included, &mut next_incl, stmt.merge);
        ClearSymbolsInfo(&mut next_incl);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    MergeIncludedSymbols(ki, info, &mut included, include.merge);
    ClearSymbolsInfo(&mut included);
    info.errorCount == 0_i32
}
fn GetGroupIndex(
    ki: &mut xkb_keymap_info<'_>,
    info: &SymbolsInfo,
    keyi: &mut KeyInfo,
    arrayNdx: Option<&ExprDef>,
    field: group_field,
    ndx_rtrn: &mut u32,
) -> bool {
    let name: &str = if field == GROUP_FIELD_SYMS {
        "symbols"
    } else {
        "actions"
    };
    if arrayNdx.is_none() {
        let mut i: u32 = 0_u32;
        if !keyi.groups.is_empty() {
            i = 0_u32;
            while (i as usize) < keyi.groups.len() {
                if keyi.groups[i as usize].defined & field == 0 {
                    *ndx_rtrn = i;
                    return true;
                }
                i = i.wrapping_add(1);
            }
        }
        if i >= info.max_groups {
            log::error!("[XKB-{:03}] Too many groups of {} for key <{}> (max {}); Ignoring {} defined for extra groups\n",
                { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                name,
                KeyInfoText(ki.ctx(), keyi),
                info.max_groups,
                name);
            return false;
        }
        let new_len = keyi.groups.len().wrapping_add(1);
        resize_groups_zero(&mut keyi.groups, new_len);
        *ndx_rtrn = keyi.groups.len().wrapping_sub(1) as u32;
        return true;
    }
    let mut pending_dummy = false;
    if ExprResolveGroup(ki, arrayNdx.unwrap(), false, ndx_rtrn, &mut pending_dummy)
        != PARSER_SUCCESS
    {
        log::error!("[XKB-{:03}] Illegal group index for {} of key <{}>\nDefinition with non-integer array index ignored\n",
            { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
            name,
            KeyInfoText(ki.ctx(), keyi));
        return false;
    }
    *ndx_rtrn = ndx_rtrn.wrapping_sub(1);
    if *ndx_rtrn >= keyi.groups.len() as u32 {
        resize_groups_zero(&mut keyi.groups, ndx_rtrn.wrapping_add(1) as usize);
    }
    true
}
fn AddSymbolsToKey(
    ki: &mut xkb_keymap_info<'_>,
    info: &SymbolsInfo,
    keyi: &mut KeyInfo,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
) -> bool {
    let mut ndx: u32 = 0_u32;
    if !GetGroupIndex(ki, info, keyi, arrayNdx, GROUP_FIELD_SYMS, &mut ndx) {
        return false;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if value.stmt_type() == STMT_EXPR_EMPTY_LIST {
        groupi.defined = (groupi.defined | GROUP_FIELD_SYMS) as group_field;
        return true;
    }
    if value.stmt_type() != STMT_EXPR_KEYSYM_LIST && value.stmt_type() != STMT_EXPR_ACTION_LIST {
        log::error!("[XKB-{:03}] Expected a list of symbols, found {}; Ignoring symbols for group {} of <{}>\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(value.stmt_type()),
            ndx.wrapping_add(1_u32),
            KeyInfoText(ki.ctx(), keyi));
        return false;
    }
    if groupi.defined & GROUP_FIELD_SYMS != 0 {
        log::error!("[XKB-{:03}] Symbols for key <{}>, group {} already defined; Ignoring duplicate definition\n",
            XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY as i32,
            KeyInfoText(ki.ctx(), keyi),
            ndx.wrapping_add(1_u32));
        return false;
    }
    let mut nLevels: u32 = 0_u32;
    let mut nonEmptyLevels: u32 = 0_u32;
    let keysym_nodes = collect_expr_list(value);
    for node in keysym_nodes {
        nLevels = nLevels.wrapping_add(1);
        let ExprKind::KeysymList { ref syms } = node.kind else {
            unreachable!()
        };
        if syms.len() as u32 > 0_u32 {
            nonEmptyLevels = nLevels;
        }
    }
    if nonEmptyLevels < nLevels {
        nLevels = nonEmptyLevels;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if (groupi.levels.len() as u32) < nLevels {
        groupi
            .levels
            .resize_with(nLevels as usize, Default::default);
    }
    groupi.defined = (groupi.defined | GROUP_FIELD_SYMS) as group_field;
    for (level, node) in keysym_nodes.iter().enumerate() {
        if level as u32 >= nLevels {
            break;
        }
        let leveli = &mut keyi.groups[ndx as usize].levels[level];
        let ExprKind::KeysymList { ref syms } = node.kind else {
            unreachable!()
        };
        let syms_len = syms.len() as u32;
        if (syms_len > 65535_u32) as i64 != 0 {
            log::error!(
                "Key <{}> has too many keysyms for group {}, level {}; expected max {}, got: {}\n",
                KeyInfoText(ki.ctx(), keyi),
                ndx.wrapping_add(1_u32),
                (level as u32).wrapping_add(1_u32),
                65535_i32,
                syms_len
            );
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
fn AddActionsToKey(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
    arrayNdx: Option<&ExprDef>,
    value: &mut ExprDef,
) -> bool {
    let mut ndx: u32 = 0_u32;
    if !GetGroupIndex(ki, info, keyi, arrayNdx, GROUP_FIELD_ACTS, &mut ndx) {
        return false;
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if value.stmt_type() == STMT_EXPR_EMPTY_LIST {
        groupi.defined = (groupi.defined | GROUP_FIELD_ACTS) as group_field;
        return true;
    }
    if value.stmt_type() != STMT_EXPR_ACTION_LIST {
        log::error!("[XKB-{:03}] Bad expression type ({}) for action list value; Ignoring actions for group {} of <{}>\n",
            XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
            value.stmt_type(),
            ndx,
            KeyInfoText(ki.ctx(), keyi));
        return false;
    }
    if groupi.defined & GROUP_FIELD_ACTS != 0 {
        log::error!(
            "[XKB-{:03}] Actions for key <{}>, group {} already defined\n",
            XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
            KeyInfoText(ki.ctx(), keyi),
            ndx
        );
        return false;
    }
    let action_nodes = collect_expr_list_mut(value);
    let nLevels: u32 = action_nodes.len() as u32;
    let groupi = &mut keyi.groups[ndx as usize];
    if (groupi.levels.len() as u32) < nLevels {
        groupi
            .levels
            .resize_with(nLevels as usize, Default::default);
    }
    groupi.defined = (groupi.defined | GROUP_FIELD_ACTS) as group_field;
    let mut level: u32 = 0_u32;
    let mut nonEmptyLevels: u32 = 0_u32;
    for action_node in action_nodes {
        let c2rust_current_block_102: u64;
        let ExprKind::ActionList {
            actions: action_vec,
        } = &mut action_node.kind
        else {
            unreachable!()
        };
        let num_actions: u32 = action_vec.len() as u32;
        if (num_actions > 65535_u32) as i64 != 0 {
            log::error!(
                "Key <{}> has too many actions for group {}, level {}; expected max {}, got: {}\n",
                KeyInfoText(ki.ctx(), keyi),
                ndx.wrapping_add(1_u32),
                level.wrapping_add(1_u32),
                65535_i32,
                num_actions
            );
            return false;
        }
        let mut actions: Vec<xkb_action> = Vec::new();
        let mut action_iter = action_vec.iter_mut();
        loop {
            let Some(act_expr) = action_iter.next() else {
                c2rust_current_block_102 = 1134115459065347084;
                break;
            };
            let mut toAct: xkb_action = xkb_action::None;
            let r: u32 = HandleActionDef(
                ki,
                &mut info.default_actions,
                &info.mods,
                act_expr,
                &mut toAct,
            );
            if r != PARSER_SUCCESS {
                log::error!("[XKB-{:03}] Illegal action definition for <{}>; Action for group {}/level {} ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    KeyInfoText(ki.ctx(), keyi),
                    ndx.wrapping_add(1_u32),
                    level.wrapping_add(1_u32));
                if r == PARSER_FATAL_ERROR {
                    drop(actions);
                    return false;
                } else {
                    toAct.set_none();
                }
            }
            if toAct.action_type() != ACTION_TYPE_NONE {
                if (num_actions == 1_u32) as i64 != 0 {
                    keyi.groups[ndx as usize].levels[level as usize].actions = vec![toAct];
                    c2rust_current_block_102 = 1829140360157350833;
                    break;
                } else {
                    actions.push(toAct);
                }
            }
        }
        if c2rust_current_block_102 == 1134115459065347084 {
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
                nonEmptyLevels = level.wrapping_add(1_u32);
            }
        }
        level = level.wrapping_add(1);
    }
    let groupi = &mut keyi.groups[ndx as usize];
    if nonEmptyLevels < nLevels {
        if nonEmptyLevels > 0_u32 {
            groupi.levels.truncate(nonEmptyLevels as usize);
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
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
fn ExprResolveOverlayEntry(
    keymap_info: &xkb_keymap_info<'_>,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    expr: &ExprDef,
    keyi: &KeyInfo,
    overlay_rtrn: &mut xkb_overlay_index_t,
    key_rtrn: &mut u32,
) -> bool {
    if arrayNdx.is_some() {
        log::error!(
            "[XKB-{:03}] Overlay field \"{}\" in <{}> does not support array index; ignored\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            field,
            xkb_atom_text(&keymap_info.keymap.ctx.atom_table, keyi.name)
        );
        return false;
    }
    let prefix: usize = (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1_usize);
    let suffix = &field[prefix..];
    let len: usize = suffix.len();
    #[allow(unused_assignments)]
    let mut raw_overlay: i64 = XKB_OVERLAY_INVALID as i64;
    let (val_parsed, parse_count) = super::super::utils::parse_dec_u64(suffix.as_bytes());
    raw_overlay = val_parsed as i64;
    if parse_count != len as i32
        || raw_overlay < 1_i64
        || raw_overlay > keymap_info.features.max_overlays as i64
    {
        log::error!("[XKB-{:03}] Unsupported overlay index \"{}\" field for <{}>: expected 1..{}, got: {}; ignored\n",
            XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX as i32,
            field,
            xkb_atom_text(&keymap_info.keymap.ctx.atom_table, keyi.name),
            keymap_info.features.max_overlays as i32,
            raw_overlay);
        return false;
    }
    *overlay_rtrn = (raw_overlay as xkb_overlay_index_t as i32 - 1_i32) as xkb_overlay_index_t;
    match expr.stmt_type() {
        8 => {
            let ExprKind::KeyName(key_name_val) = expr.kind else {
                unreachable!()
            };
            let key_kc = keymap_info
                .keymap
                .key_by_name(key_name_val, false)
                .map(|k| k.keycode);
            *key_rtrn = key_kc.unwrap_or(XKB_KEYCODE_INVALID);
            if *key_rtrn == XKB_KEYCODE_INVALID {
                log::error!(
                    "[XKB-{:03}] Unknown key \"{}\" for field {} in <{}>\n",
                    XKB_WARNING_UNDEFINED_KEYCODE as i32,
                    xkb_atom_text(&keymap_info.keymap.ctx.atom_table, key_name_val),
                    field,
                    xkb_atom_text(&keymap_info.keymap.ctx.atom_table, keyi.name)
                );
                return false;
            }
            true
        }
        10 => {
            let ExprKind::Ident(ident_val) = expr.kind else {
                unreachable!()
            };
            let id: &str = xkb_atom_text(&keymap_info.keymap.ctx.atom_table, ident_val);
            if !id.is_empty() && id.eq_ignore_ascii_case("none") {
                *key_rtrn = XKB_KEYCODE_INVALID;
                return true;
            } else if !id.is_empty() && id.eq_ignore_ascii_case("any") {
                *key_rtrn = XKB_KEYCODE_INVALID;
                *overlay_rtrn = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
                return true;
            }
            log::error!(
                "[XKB-{:03}] Unsupported overlay value \"{}\" for field {} in <{}>\n",
                XKB_ERROR_INVALID_VALUE as i32,
                id,
                field,
                xkb_atom_text(&keymap_info.keymap.ctx.atom_table, keyi.name)
            );
            false
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Expected {} for field \"{}\" in <{}>, got: {}\n",
                XKB_ERROR_INVALID_VALUE as i32,
                stmt_type_to_string(STMT_EXPR_KEYNAME_LITERAL),
                field,
                xkb_atom_text(&keymap_info.keymap.ctx.atom_table, keyi.name),
                stmt_type_to_string(expr.stmt_type())
            );
            false
        }
    }
}
fn SetSymbolsField(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    keyi: &mut KeyInfo,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    value_opt: &mut Option<Box<ExprDef>>,
) -> bool {
    if field.eq_ignore_ascii_case("type") {
        let mut ndx: u32 = 0_u32;
        let mut val: u32 = XKB_ATOM_NONE;
        if !ExprResolveString(ki.ctx(), value_opt.as_deref().unwrap(), &mut val) {
            log::error!("[XKB-{:03}] The type field of a key symbol map must be a string; Ignoring illegal type definition\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32);
            return false;
        }
        if arrayNdx.is_none() {
            keyi.default_type = val;
            keyi.defined |= KEY_FIELD_DEFAULT_TYPE as i32 as key_field;
        } else if {
            let mut pending_dummy = false;
            ExprResolveGroup(ki, arrayNdx.unwrap(), false, &mut ndx, &mut pending_dummy)
        } != PARSER_SUCCESS
        {
            log::error!("[XKB-{:03}] Illegal group index for type of key <{}>; Definition with non-integer array index ignored\n",
                { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                KeyInfoText(ki.ctx(), keyi));
            return false;
        } else {
            ndx = ndx.wrapping_sub(1);
            if ndx >= keyi.groups.len() as u32 {
                resize_groups_zero(&mut keyi.groups, (ndx as usize).wrapping_add(1));
            }
            keyi.groups[ndx as usize].type_0 = val;
            let c2rust_fresh8 = &mut keyi.groups[ndx as usize].defined;
            *c2rust_fresh8 = (*c2rust_fresh8 | GROUP_FIELD_TYPE) as group_field;
        }
    } else if field.eq_ignore_ascii_case("symbols") {
        return AddSymbolsToKey(ki, info, keyi, arrayNdx, value_opt.as_deref().unwrap());
    } else if field.eq_ignore_ascii_case("actions") {
        return AddActionsToKey(
            ki,
            info,
            keyi,
            arrayNdx,
            value_opt.as_mut().map(|b| &mut **b).unwrap(),
        );
    } else if field.eq_ignore_ascii_case("vmods")
        || field.eq_ignore_ascii_case("virtualmods")
        || field.eq_ignore_ascii_case("virtualmodifiers")
    {
        let mut mask: u32 = 0_u32;
        let val = value_opt.as_deref().unwrap();
        if !ExprResolveModMask(ki.ctx(), val, MOD_VIRT, &info.mods, &mut mask) {
            log::error!("[XKB-{:03}] Expected a virtual modifier mask, found {}; Ignoring virtual modifiers definition for key <{}>\n",
                { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK },
                stmt_type_to_string(val.stmt_type()),
                KeyInfoText(ki.ctx(), keyi));
            return false;
        }
        keyi.vmodmap = mask;
        keyi.defined |= KEY_FIELD_VMODMAP as i32 as key_field;
    } else if field.eq_ignore_ascii_case("locking")
        || field.eq_ignore_ascii_case("lock")
        || field.eq_ignore_ascii_case("locks")
    {
        log::warn!("[XKB-{:03}] Key behaviors not supported; Ignoring locking specification for key <{}>\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            KeyInfoText(ki.ctx(), keyi));
    } else if field.eq_ignore_ascii_case("radiogroup")
        || field.eq_ignore_ascii_case("permanentradiogroup")
        || field.eq_ignore_ascii_case("allownone")
    {
        log::warn!("[XKB-{:03}] Radio groups not supported; Ignoring radio group specification for key <{}>\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            KeyInfoText(ki.ctx(), keyi));
    } else if field
        .get(..16)
        .is_some_and(|s| s.eq_ignore_ascii_case("permanentoverlay"))
    {
        log::warn!("[XKB-{:03}] Permanent overlays not supported; Ignoring overlay specification for key <{}>\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            KeyInfoText(ki.ctx(), keyi));
    } else if field
        .get(..7)
        .is_some_and(|s| s.eq_ignore_ascii_case("overlay"))
    {
        let mut overlay: xkb_overlay_index_t = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
        let mut key: u32 = XKB_KEYCODE_INVALID;
        if !ExprResolveOverlayEntry(
            ki,
            field,
            arrayNdx,
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
            log::warn!(
                "Cannot overlay a key to itself; Ignoring overlay {} specification for key <{}>\n",
                overlay as i32 + 1_i32,
                KeyInfoText(ki.ctx(), keyi)
            );
        } else {
            let mut prev: u32 = XKB_KEYCODE_INVALID;
            if overlays_get(keyi, overlay, Some(&mut prev)) {
                if key != prev {
                    log::warn!("[XKB-{:03}] Conflicting overlays defined in key <{}>; use overlay{}=<{}>, ignore overlay{}=<{}>\n",
                        XKB_WARNING_CONFLICTING_KEY_FIELDS as i32,
                        KeyInfoText(ki.ctx(), keyi),
                        overlay as i32 + 1_i32,
                        if prev != XKB_KEYCODE_INVALID {
                            xkb_atom_text(&ki.ctx().atom_table, ki.keymap.get_key(prev).map(|k| k.name).unwrap_or(0))
                        } else {
                            "none"
                        },
                        overlay as i32 + 1_i32,
                        if key != XKB_KEYCODE_INVALID {
                            xkb_atom_text(&ki.ctx().atom_table, ki.keymap.get_key(key).map(|k| k.name).unwrap_or(0))
                        } else {
                            "none"
                        });
                }
            } else if ki.features.overlapping_overlays {
                if !overlays_insert(keyi, overlay, key) {
                    return false;
                }
                keyi.defined |= KEY_FIELD_OVERLAY as i32 as key_field;
            } else {
                let mask_0: xkb_overlay_mask_t = (1_u32 << overlay as i32) as xkb_overlay_mask_t;
                if keyi.overlays == 0 || keyi.overlays_clear as i32 != 0 {
                    if key != XKB_KEYCODE_INVALID {
                        keyi.overlays = mask_0;
                        keyi.overlays_clear = false;
                        keyi.overlay_keys = vec![key];
                    } else {
                        keyi.overlays =
                            (keyi.overlays as i32 | mask_0 as i32) as xkb_overlay_mask_t;
                        keyi.overlays_clear = true;
                        keyi.overlay_keys = vec![XKB_KEYCODE_INVALID];
                    }
                    keyi.defined |= KEY_FIELD_OVERLAY as i32 as key_field;
                } else if keyi.overlays != 0 && key != XKB_KEYCODE_INVALID {
                    let existing_key = keyi
                        .overlay_keys
                        .first()
                        .copied()
                        .unwrap_or(XKB_KEYCODE_INVALID);
                    log::error!("[XKB-{:03}] Overlapping overlays are not allowed in <{}>; use overlay{}=<{}>, ignore overlay{}=<{}>\n",
                            XKB_ERROR_OVERLAPPING_OVERLAY as i32,
                            KeyInfoText(ki.ctx(), keyi),
                            keyi.overlays as i32,
                            xkb_atom_text(
                                &ki.ctx().atom_table,
                                if existing_key == XKB_KEYCODE_INVALID { 0 } else { ki.keymap.get_key(existing_key).map(|k| k.name).unwrap_or(0) },
                            ),
                            overlay as i32 + 1_i32,
                            xkb_atom_text(&ki.ctx().atom_table, ki.keymap.get_key(key).map(|k| k.name).unwrap_or(0)));
                    return ki.strict & PARSER_NO_FIELD_VALUE_MISMATCH == 0;
                }
            }
        }
    } else if field.eq_ignore_ascii_case("repeating")
        || field.eq_ignore_ascii_case("repeats")
        || field.eq_ignore_ascii_case("repeat")
    {
        let mut val_0: u32 = 0_u32;
        if !ExprResolveEnum(
            ki.ctx(),
            value_opt.as_deref().unwrap(),
            &mut val_0,
            &REPEAT_ENTRIES,
        ) {
            log::error!(
                "[XKB-{:03}] Illegal repeat setting for <{}>; Non-boolean repeat setting ignored\n",
                XKB_ERROR_INVALID_VALUE as i32,
                KeyInfoText(ki.ctx(), keyi)
            );
            return false;
        }
        keyi.repeat = val_0 as key_repeat as key_repeat;
        keyi.defined |= KEY_FIELD_REPEAT as i32 as key_field;
    } else if field.eq_ignore_ascii_case("groupswrap") || field.eq_ignore_ascii_case("wrapgroups") {
        let mut set: bool = false;
        if !ExprResolveBoolean(ki.ctx(), value_opt.as_deref().unwrap(), &mut set) {
            log::error!(
                "[XKB-{:03}] Illegal groupsWrap setting for <{}>; Non-boolean value ignored\n",
                XKB_ERROR_INVALID_VALUE as i32,
                KeyInfoText(ki.ctx(), keyi)
            );
            return false;
        }
        keyi.out_of_range_group_policy = if set {
            XKB_LAYOUT_OUT_OF_RANGE_WRAP
        } else {
            XKB_LAYOUT_OUT_OF_RANGE_CLAMP
        };
        keyi.defined |= KEY_FIELD_GROUPINFO as i32 as key_field;
    } else if field.eq_ignore_ascii_case("groupsclamp") || field.eq_ignore_ascii_case("clampgroups")
    {
        let mut set_0: bool = false;
        if !ExprResolveBoolean(ki.ctx(), value_opt.as_deref().unwrap(), &mut set_0) {
            log::error!(
                "[XKB-{:03}] Illegal groupsClamp setting for <{}>; Non-boolean value ignored\n",
                XKB_ERROR_INVALID_VALUE as i32,
                KeyInfoText(ki.ctx(), keyi)
            );
            return false;
        }
        keyi.out_of_range_group_policy = if set_0 {
            XKB_LAYOUT_OUT_OF_RANGE_CLAMP
        } else {
            XKB_LAYOUT_OUT_OF_RANGE_WRAP
        };
        keyi.defined |= KEY_FIELD_GROUPINFO as i32 as key_field;
    } else if field.eq_ignore_ascii_case("groupsredirect")
        || field.eq_ignore_ascii_case("redirectgroups")
    {
        let mut grp: u32 = 0_u32;
        let mut pending: bool = false;
        if ExprResolveGroup(
            ki,
            value_opt.as_deref().unwrap(),
            false,
            &mut grp,
            &mut pending,
        ) != PARSER_SUCCESS
            && !pending
        {
            log::error!("[XKB-{:03}] Illegal group index for redirect of key <{}>; Definition with non-integer group ignored\n",
                { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                KeyInfoText(ki.ctx(), keyi));
            return false;
        }
        if pending {
            keyi.out_of_range_pending_group = true;
            let pending_index: u32 = ki.pending_computations.len() as u32;
            ki.pending_computations.push(pending_computation {
                expr: value_opt.take(),
                computed: false,
                value: 0_u32,
            });
            keyi.out_of_range_group_number = pending_index;
        } else {
            keyi.out_of_range_pending_group = false;
            keyi.out_of_range_group_number = grp.wrapping_sub(1_u32);
        }
        keyi.out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_REDIRECT;
        keyi.defined |= KEY_FIELD_GROUPINFO as i32 as key_field;
    } else {
        log::error!(
            "[XKB-{:03}] Unknown field \"{}\" in a key; definition ignored\n",
            XKB_ERROR_UNKNOWN_FIELD as i32,
            field
        );
        return ki.strict & PARSER_NO_UNKNOWN_KEY_FIELDS == 0;
    }
    true
}
fn SetGroupName(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
    merge: merge_mode,
) -> bool {
    let arrayNdx = match arrayNdx {
        Some(a) => a,
        None => {
            log::warn!("[XKB-{:03}] You must specify an index when specifying a group name; Group name definition without array subscript ignored\n",
                XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX as i32);
            return false;
        }
    };
    let mut group: u32 = 0_u32;
    let mut pending_dummy: bool = false;
    if { ExprResolveGroup(ki, arrayNdx, false, &mut group, &mut pending_dummy) } as u32
        != PARSER_SUCCESS
    {
        log::error!("[XKB-{:03}] Illegal index in group name definition; Definition with non-integer array index ignored\n",
            { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX });
        return false;
    }
    let mut name: u32 = XKB_ATOM_NONE;
    if !ExprResolveString(ki.ctx(), value, &mut name) {
        log::error!(
            "[XKB-{:03}] Group name must be a string; Illegal name for group {} ignored\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            group
        );
        return false;
    }
    let group_to_use: u32;
    if info.explicit_group == XKB_LAYOUT_INVALID {
        group_to_use = group.wrapping_sub(1_u32);
    } else if group.wrapping_sub(1_u32) == 0_u32 {
        group_to_use = info.explicit_group;
    } else {
        log::warn!("[XKB-{:03}] An explicit group was specified for the '{}' map, but it provides a name for a group other than Group1 ({}); Ignoring group name '{}'\n",
            XKB_WARNING_NON_BASE_GROUP_NAME as i32,
            info.name.as_deref().unwrap_or(""),
            group,
            xkb_atom_text(&ki.ctx().atom_table, name));
        return false;
    }
    if group_to_use >= info.group_names.len() as u32 {
        info.group_names
            .resize((group_to_use as usize).wrapping_add(1), 0_u32);
    } else {
        let old_name: u32 = info.group_names[group_to_use as usize];
        if old_name != XKB_ATOM_NONE && old_name != name {
            let replace: bool = merge != MERGE_AUGMENT;
            let use_0: u32 = if replace as i32 != 0 { name } else { old_name };
            let ignore: u32 = if replace as i32 != 0 { old_name } else { name };
            log::warn!(
                "Multiple definitions of group {} name in map '{}'; Using '{}', ignoring '{}'\n",
                group_to_use,
                info.name.as_deref().unwrap_or(""),
                xkb_atom_text(&ki.ctx().atom_table, use_0),
                xkb_atom_text(&ki.ctx().atom_table, ignore)
            );
            name = use_0;
        }
    }
    info.group_names[group_to_use as usize] = name;
    true
}
fn HandleGlobalVar(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    stmt: &mut VarDef,
) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut arrayNdx_opt: Option<&ExprDef> = None;
    let ret: bool;
    if !ExprResolveLhs(
        stmt.name.as_deref().unwrap(),
        &mut elem_atom,
        &mut field_atom,
        &mut arrayNdx_opt,
    ) {
        return false;
    }
    let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom).to_owned();
    let field = xkb_atom_text(&ki.ctx().atom_table, field_atom).to_owned();
    let elem: &str = &elem;
    let field: &str = &field;
    if !elem.is_empty() && elem.eq_ignore_ascii_case("key") {
        let mut temp: KeyInfo = {
            let mut init = KeyInfo::new_zeroed();
            init.out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_WRAP;
            init.defined = 0 as key_field;
            init.merge = MERGE_DEFAULT;
            init.repeat = KEY_REPEAT_UNDEFINED;
            init.out_of_range_pending_group = false;
            init.overlays_clear = false;
            init
        };
        InitKeyInfo(ki.ctx_mut(), &mut temp);
        temp.merge = if temp.merge == MERGE_REPLACE {
            MERGE_OVERRIDE
        } else {
            stmt.merge as merge_mode
        };
        ret = SetSymbolsField(ki, info, &mut temp, field, arrayNdx_opt, &mut stmt.value);
        let mut dk = std::mem::replace(&mut info.default_key, KeyInfo::new_zeroed());
        MergeKeys(ki, info, &mut dk, &mut temp, true);
        info.default_key = dk;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("name") || field.eq_ignore_ascii_case("groupname"))
    {
        ret = SetGroupName(
            ki,
            info,
            arrayNdx_opt,
            stmt.value.as_deref().unwrap(),
            stmt.merge,
        );
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("groupswrap") || field.eq_ignore_ascii_case("wrapgroups"))
    {
        log::error!(
            "[XKB-{:03}] Global \"groupswrap\" not supported; Ignored\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32
        );
        ret = true;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("groupsclamp") || field.eq_ignore_ascii_case("clampgroups"))
    {
        log::error!(
            "[XKB-{:03}] Global \"groupsclamp\" not supported; Ignored\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32
        );
        ret = true;
    } else if elem.is_empty()
        && (field.eq_ignore_ascii_case("groupsredirect")
            || field.eq_ignore_ascii_case("redirectgroups"))
    {
        log::error!(
            "[XKB-{:03}] Global \"groupsredirect\" not supported; Ignored\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32
        );
        ret = true;
    } else if elem.is_empty() && field.eq_ignore_ascii_case("allownone") {
        log::error!(
            "[XKB-{:03}] Radio groups not supported; Ignoring \"allownone\" specification\n",
            XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32
        );
        ret = true;
    } else if !elem.is_empty() {
        let elem_owned = elem.to_owned();
        let field_owned = field.to_owned();
        ret = {
            SetDefaultActionField(
                ki,
                &mut info.default_actions,
                &mut info.mods,
                &elem_owned,
                &field_owned,
                arrayNdx_opt,
                &mut stmt.value,
                stmt.merge,
            ) as u32
                != PARSER_FATAL_ERROR
        };
    } else {
        log::error!(
            "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
            XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
            field
        );
        return ki.strict & PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS == 0;
    }
    ret
}
fn HandleSymbolsBody(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    defs: &mut [VarDef],
    keyi: &mut KeyInfo,
) -> bool {
    let mut all_valid_entries: bool = true;
    for def in defs.iter_mut() {
        let field_owned: String;
        let field: &str;
        let mut arrayNdx_opt: Option<&ExprDef> = None;
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
            ok = ExprResolveLhs(
                def.name.as_deref().unwrap(),
                &mut elem_atom,
                &mut field_atom,
                &mut arrayNdx_opt,
            );
            let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom);
            field_owned = xkb_atom_text(&ki.ctx().atom_table, field_atom).to_owned();
            field = &field_owned;
            if ok as i32 != 0 && !elem.is_empty() {
                log::error!("[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    elem,
                    elem,
                    field);
                ok = false;
            }
        }
        if def.value.is_none() {
            log::error!(
                "[XKB-{:03}] Could not allocate the value of field \"{}\". Statement ignored.\n",
                XKB_ERROR_ALLOCATION_ERROR as i32,
                field
            );
            ok = false;
        }
        if !ok || !SetSymbolsField(ki, info, keyi, &field, arrayNdx_opt, &mut def.value) {
            all_valid_entries = false;
        }
    }
    all_valid_entries
}
fn SetExplicitGroup(ki: &xkb_keymap_info<'_>, info: &SymbolsInfo, keyi: &mut KeyInfo) -> bool {
    let mut warn: bool = false;
    if info.explicit_group == XKB_LAYOUT_INVALID {
        return true;
    }
    if !keyi.groups.is_empty() {
        let mut i: u32 = 1_u32;
        while i < keyi.groups.len() as u32 {
            if keyi.groups[i as usize].defined as u64 != 0 {
                warn = true;
                ClearGroupInfo(&mut keyi.groups[i as usize]);
                InitGroupInfo(&mut keyi.groups[i as usize]);
            }
            i = i.wrapping_add(1);
        }
    }
    if warn {
        log::warn!("[XKB-{:03}] For the map {} the explicit group {} is specified, but key <{}> has more than one group defined; All groups except first one will be ignored\n",
            XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE as i32,
            info.name.as_deref().unwrap_or(""),
            info.explicit_group.wrapping_add(1_u32),
            KeyInfoText(ki.ctx(), keyi));
    }
    keyi.groups.resize_with(
        (info.explicit_group as usize).wrapping_add(1),
        Default::default,
    );
    if info.explicit_group > 0_u32 {
        // Move groups[0] to groups[explicit_group], replace groups[0] with default
        keyi.groups[info.explicit_group as usize] = std::mem::take(&mut keyi.groups[0]);
    }
    true
}
fn HandleSymbolsDef(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    stmt: &mut SymbolsDef,
) -> bool {
    // Clone scalar fields from default_key, deep-copy groups
    let dk = &info.default_key;
    let mut keyi = KeyInfo {
        name: dk.name,
        vmodmap: dk.vmodmap,
        default_type: dk.default_type,
        out_of_range_group_number: dk.out_of_range_group_number,
        groups: dk.groups.clone(), // deep clone via derive(Clone)
        out_of_range_group_policy: dk.out_of_range_group_policy,
        defined: dk.defined,
        merge: dk.merge,
        repeat: dk.repeat,
        out_of_range_pending_group: dk.out_of_range_pending_group,
        overlays_clear: dk.overlays_clear,
        overlays: dk.overlays,
        overlay_keys: dk.overlay_keys.clone(),
    };
    keyi.merge = stmt.merge as merge_mode;
    keyi.name = stmt.keyName;
    if HandleSymbolsBody(ki, info, &mut stmt.symbols, &mut keyi) as i32 != 0
        && SetExplicitGroup(ki, info, &mut keyi) as i32 != 0
        && AddKeySymbols(ki, info, &mut keyi, true) as i32 != 0
    {
        return true;
    }
    ClearKeyInfo(&mut keyi);
    info.errorCount += 1;
    false
}
fn HandleModMapDef(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut SymbolsInfo,
    def: &mut ModMapDef,
) -> bool {
    let mut tmp: ModMapEntry = ModMapEntry {
        merge: MERGE_DEFAULT,
        haveSymbol: false,
        modifier: 0,
        u: 0,
    };
    let ndx: u32;
    let mut ok: bool;
    let modifier_name: &str = xkb_atom_text(&ki.ctx().atom_table, def.modifier);
    if modifier_name.eq_ignore_ascii_case("none") {
        ndx = XKB_MOD_NONE;
    } else {
        ndx = XkbModNameToIndex(&info.mods, def.modifier, MOD_REAL);
        if ndx == XKB_MOD_INVALID {
            log::error!("[XKB-{:03}] Illegal modifier map definition; Ignoring map for non-modifier \"{}\"\n",
                XKB_ERROR_INVALID_REAL_MODIFIER as i32,
                xkb_atom_text(&ki.ctx().atom_table, def.modifier));
            return false;
        }
    }
    ok = true;
    tmp.modifier = ndx;
    tmp.merge = def.merge;
    let mut c2rust_current_block_19: u64;
    for key in def.keys.iter() {
        if key.stmt_type() == STMT_EXPR_KEYNAME_LITERAL {
            tmp.haveSymbol = false;
            let ExprKind::KeyName(kn) = key.kind else {
                unreachable!()
            };
            tmp.u = kn;
            c2rust_current_block_19 = 5601891728916014340;
        } else if key.stmt_type() == STMT_EXPR_KEYSYM_LITERAL {
            let ExprKind::KeySym(ks) = key.kind else {
                unreachable!()
            };
            if ks == XKB_KEY_NoSymbol as u32 {
                c2rust_current_block_19 = 13536709405535804910;
            } else {
                tmp.haveSymbol = true;
                tmp.u = ks;
                c2rust_current_block_19 = 5601891728916014340;
            }
        } else {
            log::error!("[XKB-{:03}] Modmap entries may contain only key names or keysyms; Illegal definition for {} modifier ignored\n",
                XKB_ERROR_INVALID_MODMAP_ENTRY as i32,
                ModIndexText(ki.ctx(), &info.mods, tmp.modifier));
            c2rust_current_block_19 = 13536709405535804910;
        }
        if c2rust_current_block_19 == 5601891728916014340 {
            ok = AddModMapEntry(ki, info, &tmp) as i32 != 0 && ok as i32 != 0;
        }
    }
    ok
}
fn HandleSymbolsFile(ki: &mut xkb_keymap_info<'_>, info: &mut SymbolsInfo, file: &mut XkbFile) {
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
                    ok = HandleIncludeSymbols(ki, info, incl);
                }
                Statement::Symbols(sym) => {
                    ok = HandleSymbolsDef(ki, info, sym);
                }
                Statement::Var(var) => {
                    ok = HandleGlobalVar(ki, info, var);
                }
                Statement::VMod(vmod) => {
                    ok = HandleVModDef(ki.ctx_mut(), &mut info.mods, vmod);
                }
                Statement::ModMap(mm) => {
                    ok = HandleModMapDef(ki, info, mm);
                }
                Statement::Unknown(unk) => {
                    log::error!(
                        "[XKB-{:03}] Unsupported symbols {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        if unk.stmt_type == STMT_UNKNOWN_COMPOUND {
                            "compound"
                        } else {
                            "declaration"
                        },
                        &unk.name
                    );
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    log::error!(
                        "[XKB-{:03}] Symbols files may not include other types; Ignoring {}\n",
                        XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
                        stmt_type_to_string(stmt.stmt_type())
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                log::error!(
                    "[XKB-{:03}] Abandoning symbols file \"{}\"\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    safe_map_name(file)
                );
                break;
            }
        }
    }
}
fn FindKeyForSymbol(keymap: &mut xkb_keymap, sym: u32) -> Option<&mut xkb_key> {
    let mut got_one_group: bool;
    let mut group: u32 = 0_u32;
    loop {
        let mut level: u32 = 0_u32;
        got_one_group = false;
        let mut got_one_level: bool;
        loop {
            got_one_level = false;
            let start_idx = if keymap.num_keys_low == 0_u32 {
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
                ki = ki.wrapping_add(1);
            }
            level = level.wrapping_add(1);
            if !got_one_level {
                break;
            }
        }
        group = group.wrapping_add(1);
        if !got_one_group {
            break;
        }
    }
    None
}
fn FindAutomaticType(ctx: &mut xkb_context, groupi: &GroupInfo) -> u32 {
    let width: u32 = groupi.levels.len() as u32;
    if width == 1_u32 || width <= 0_u32 {
        return xkb_atom_intern_ref(ctx, b"ONE_LEVEL");
    }
    let sym0: u32 = if groupi.levels[0].syms.is_empty() {
        XKB_KEY_NoSymbol as u32
    } else {
        groupi.levels[0].syms[0]
    };
    let sym1: u32 = if groupi.levels[1].syms.is_empty() {
        XKB_KEY_NoSymbol as u32
    } else {
        groupi.levels[1].syms[0]
    };
    if width == 2_u32 {
        if xkb_keysym_is_lower(sym0) as i32 != 0 && xkb_keysym_is_upper_or_title(sym1) as i32 != 0 {
            return xkb_atom_intern_ref(ctx, b"ALPHABETIC");
        }
        if xkb_keysym_is_keypad(sym0) as i32 != 0 || xkb_keysym_is_keypad(sym1) as i32 != 0 {
            return xkb_atom_intern_ref(ctx, b"KEYPAD");
        }
        return xkb_atom_intern_ref(ctx, b"TWO_LEVEL");
    }
    if width <= 4_u32 {
        if xkb_keysym_is_lower(sym0) as i32 != 0 && xkb_keysym_is_upper_or_title(sym1) as i32 != 0 {
            let sym2: u32 = if groupi.levels[2].syms.is_empty() {
                XKB_KEY_NoSymbol as u32
            } else {
                groupi.levels[2].syms[0]
            };
            let sym3: u32 = if width == 4_u32 {
                if groupi.levels[3].syms.is_empty() {
                    XKB_KEY_NoSymbol as u32
                } else {
                    groupi.levels[3].syms[0]
                }
            } else {
                XKB_KEY_NoSymbol as u32
            };
            if xkb_keysym_is_lower(sym2) as i32 != 0
                && xkb_keysym_is_upper_or_title(sym3) as i32 != 0
            {
                return xkb_atom_intern_ref(ctx, b"FOUR_LEVEL_ALPHABETIC");
            }
            return xkb_atom_intern_ref(ctx, b"FOUR_LEVEL_SEMIALPHABETIC");
        }
        if xkb_keysym_is_keypad(sym0) as i32 != 0 || xkb_keysym_is_keypad(sym1) as i32 != 0 {
            return xkb_atom_intern_ref(ctx, b"FOUR_LEVEL_KEYPAD");
        }
        return xkb_atom_intern_ref(ctx, b"FOUR_LEVEL");
    }
    XKB_ATOM_NONE
}
fn FindTypeForGroup(
    keymap: &mut xkb_keymap,
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
            type_name = FindAutomaticType(&mut keymap.ctx, groupi);
            if type_name != XKB_ATOM_NONE {
                *explicit_type = false;
            }
        }
    }
    if type_name == XKB_ATOM_NONE {
        log::warn!("[XKB-{:03}] Couldn't find an automatic type for key '<{}>' group {} with {} levels; Using the default type\n",
            XKB_WARNING_CANNOT_INFER_KEY_TYPE as i32,
            xkb_atom_text(&keymap.ctx.atom_table, keyi.name),
            group.wrapping_add(1_u32),
            keyi.groups[group as usize].levels.len());
    } else {
        let mut i: u32 = 0_u32;
        while (i as usize) < keymap.types.len() {
            if keymap.types[i as usize].name == type_name {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i as usize >= keymap.types.len() {
            log::warn!("[XKB-{:03}] The type \"{}\" for key '<{}>' group {} was not previously defined; Using the default type\n",
                XKB_WARNING_UNDEFINED_KEY_TYPE as i32,
                xkb_atom_text(&keymap.ctx.atom_table, type_name),
                xkb_atom_text(&keymap.ctx.atom_table, keyi.name),
                group.wrapping_add(1_u32));
        } else {
            keymap.types[i as usize].required = true;
            return i;
        }
    }
    keymap.types[0].required = true;
    0
}
fn CopySymbolsDefToKeymap(
    keymap: &mut xkb_keymap,
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
            log::warn!(
                "[XKB-{:03}] Key <{}> not found in keycodes; Symbols ignored\n",
                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                KeyInfoText(&keymap.ctx, keyi)
            );
            return false;
        }
    };

    // Find the range of groups we need
    keymap.keys[key_idx].num_groups = 0;
    if !keyi.groups.is_empty() {
        for (idx, groupi) in keyi.groups.iter().enumerate() {
            let has_explicit_type = (keyi.defined as i32 & KEY_FIELD_DEFAULT_TYPE as i32 != 0)
                || (groupi.defined & GROUP_FIELD_TYPE != 0);
            if !groupi.levels.is_empty() || has_explicit_type {
                keymap.keys[key_idx].num_groups = (idx as u32).wrapping_add(1);
            }
            if has_explicit_type {
                keymap.keys[key_idx].explicit =
                    (keymap.keys[key_idx].explicit | EXPLICIT_TYPES) as xkb_explicit_components;
            }
        }
    }

    if keymap.keys[key_idx].num_groups <= 0 {
        // A key with no group may still have other fields defined
        if keyi.defined as i32 != 0 {
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
                i = i.wrapping_add(1);
            }
        }

        keymap.keys[key_idx].groups = (0..keymap.keys[key_idx].num_groups)
            .map(|_| xkb_group {
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
                let type_idx: u32 = FindTypeForGroup(keymap, keyi, i, &mut explicit_type);

                // Always have as many levels as the type specifies
                if keymap.types[type_idx as usize].num_levels
                    < keyi.groups[i as usize].levels.len() as u32
                {
                    log::warn!("[XKB-{:03}] Type \"{}\" has {} levels, but <{}> has {} levels; Ignoring extra symbols\n",
                        XKB_WARNING_EXTRA_SYMBOLS_IGNORED as i32,
                        xkb_atom_text(&keymap.ctx.atom_table, keymap.types[type_idx as usize].name),
                        keymap.types[type_idx as usize].num_levels,
                        KeyInfoText(&keymap.ctx, keyi),
                        keyi.groups[i as usize].levels.len());

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

                i = i.wrapping_add(1);
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
                            leveli.upper = XKB_KEY_NoSymbol as u32;
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
                    keymap.keys[key_idx].explicit = (keymap.keys[key_idx].explicit
                        | EXPLICIT_SYMBOLS)
                        as xkb_explicit_components;
                }
                if groupi.defined & GROUP_FIELD_ACTS != 0 {
                    keymap.keys[key_idx].groups[i as usize].explicit_actions = true;
                    keymap.keys[key_idx].explicit = (keymap.keys[key_idx].explicit
                        | EXPLICIT_INTERP)
                        as xkb_explicit_components;
                }
                if keymap.keys[key_idx].groups[i as usize].explicit_type {
                    keymap.keys[key_idx].explicit =
                        (keymap.keys[key_idx].explicit | EXPLICIT_TYPES) as xkb_explicit_components;
                }

                i = i.wrapping_add(1);
            }
        }

        keymap.keys[key_idx].out_of_range_pending_group = keyi.out_of_range_pending_group;
        keymap.keys[key_idx].out_of_range_group_number = keyi.out_of_range_group_number;
        keymap.keys[key_idx].out_of_range_group_policy = keyi.out_of_range_group_policy;
    }

    // key_fields:
    if keyi.defined as i32 & KEY_FIELD_VMODMAP as i32 != 0 {
        keymap.keys[key_idx].vmodmap = keyi.vmodmap;
        keymap.keys[key_idx].explicit =
            (keymap.keys[key_idx].explicit | EXPLICIT_VMODMAP) as xkb_explicit_components;
    }

    if keyi.repeat != KEY_REPEAT_UNDEFINED as key_repeat {
        keymap.keys[key_idx].repeats = keyi.repeat == KEY_REPEAT_YES as key_repeat;
        keymap.keys[key_idx].explicit =
            (keymap.keys[key_idx].explicit | EXPLICIT_REPEAT) as xkb_explicit_components;
    }

    if (keyi.defined as i32 & KEY_FIELD_OVERLAY as i32 != 0)
        && keyi.overlays != 0
        && !keyi.overlays_clear
    {
        // Remove null entries from overlay_keys and clear corresponding bits
        let mut clean_overlays: xkb_overlay_mask_t = 0;
        let mut clean_keys: Vec<u32> = Vec::new();
        let mut remaining: xkb_overlay_mask_t = keyi.overlays;
        let mut idx: usize = 0;
        while remaining != 0 {
            let lsb: xkb_overlay_mask_t = remaining & (!remaining).wrapping_add(1);
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
            keymap.keys[key_idx].explicit =
                (keymap.keys[key_idx].explicit | EXPLICIT_OVERLAY) as xkb_explicit_components;
        }
    }

    true
}

fn CopyModMapDefToKeymap(keymap: &mut xkb_keymap, info: &SymbolsInfo, entry: &ModMapEntry) -> bool {
    if !entry.haveSymbol {
        if let Some(key) = keymap.key_by_name_mut(entry.u, true) {
            if entry.modifier != XKB_MOD_NONE {
                key.modmap |= 1_u32 << entry.modifier;
            }
            true
        } else {
            log::warn!("[XKB-{:03}] Key <{}> not found in keycodes; Modifier map entry for {} not updated\n",
                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                xkb_atom_text(&keymap.ctx.atom_table, entry.u),
                ModIndexText(&keymap.ctx, &info.mods, entry.modifier));
            false
        }
    } else if let Some(key) = FindKeyForSymbol(keymap, entry.u) {
        if entry.modifier != XKB_MOD_NONE {
            key.modmap |= 1_u32 << entry.modifier;
        }
        true
    } else {
        log::warn!("[XKB-{:03}] Key \"{}\" not found in symbol map; Modifier map entry for {} not updated\n",
            XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL as i32,
            KeysymText(entry.u),
            ModIndexText(&keymap.ctx, &info.mods, entry.modifier));
        false
    }
}
fn CopySymbolsToKeymap(keymap: &mut xkb_keymap, info: &mut SymbolsInfo) -> bool {
    keymap.symbols_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut keymap.symbols_section_name);
    keymap.mods = info.mods;
    keymap.group_names = std::mem::take(&mut info.group_names);
    let mut keys = std::mem::take(&mut info.keys);
    for keyi in keys.iter_mut() {
        if !CopySymbolsDefToKeymap(keymap, info, keyi) {
            info.errorCount += 1;
        }
    }
    info.keys = keys;
    if xkb_context_get_log_verbosity(&keymap.ctx) > 3_i32 {
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            let key = &keymap.keys[ki as usize];
            if (key.name != XKB_ATOM_NONE) && (key.num_groups as i32) < 1_i32 {
                log::info!(
                    "No symbols defined for <{}>\n",
                    xkb_atom_text(&keymap.ctx.atom_table, key.name)
                );
            }
            ki = ki.wrapping_add(1);
        }
    }
    for i in 0..info.modmaps.len() {
        if !CopyModMapDefToKeymap(keymap, info, &info.modmaps[i]) {
            info.errorCount += 1;
        }
    }
    true
}
pub fn CompileSymbols(file: Option<&mut XkbFile>, keymap_info: &mut xkb_keymap_info<'_>) -> bool {
    let mods = keymap_info.keymap_ref().mods;
    let mut info = SymbolsInfo::new(keymap_info);
    InitSymbolsInfo(&mut info, keymap_info, 0_u32, &mods);
    if let Some(file) = file {
        HandleSymbolsFile(keymap_info, &mut info, file);
    }
    if (info.errorCount == 0_i32) && CopySymbolsToKeymap(keymap_info.keymap_mut(), &mut info) {
        ClearSymbolsInfo(&mut info);
        return true;
    }
    ClearSymbolsInfo(&mut info);
    false
}
use super::super::context::xkb_context_get_log_verbosity;
use super::super::keysym_case_mappings::xkb_keysym_to_upper;
use super::super::shared_types::*;
pub struct CompatInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub default_interp: SymInterpInfo,
    pub interps: Vec<SymInterpInfo>,
    pub default_led: LedInfo,
    pub leds: [LedInfo; 32],
    pub num_leds: u32,
    pub default_actions: ActionsInfo,
    pub mods: xkb_mod_set,
}
impl Default for CompatInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatInfo {
    pub fn new() -> Self {
        let zeroed_led = LedInfo {
            defined: 0 as led_field,
            merge: MERGE_DEFAULT,
            led: xkb_led {
                name: 0,
                which_groups: 0,
                pending_groups: false,
                groups: 0,
                which_mods: 0_u32,
                mods: xkb_mods { mods: 0, mask: 0 },
                ctrls: 0 as xkb_action_controls,
            },
        };
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            default_interp: SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat: false,
                    required: false,
                    num_actions: 0,
                    action: xkb_action::None,
                    actions: Vec::new(),
                },
            },
            interps: Vec::new(),
            default_led: zeroed_led,
            leds: [zeroed_led; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action::None; 21],
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0_u32,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
        }
    }
}
#[derive(Copy, Clone)]
pub struct LedInfo {
    pub defined: led_field,
    pub merge: merge_mode,
    pub led: xkb_led,
}
pub type led_field = u32;
pub const LED_FIELD_CTRLS: led_field = 4;
pub const LED_FIELD_GROUPS: led_field = 2;
pub const LED_FIELD_MODS: led_field = 1;
// C2Rust_Unnamed_18 removed: replaced by Vec<SymInterpInfo>
#[derive(Clone)]
pub struct SymInterpInfo {
    pub defined: si_field,
    pub merge: merge_mode,
    pub interp: xkb_sym_interpret,
}
pub type si_field = u32;
pub const SI_FIELD_LEVEL_ONE_ONLY: si_field = 8;
pub const SI_FIELD_AUTO_REPEAT: si_field = 4;
pub const SI_FIELD_ACTION: si_field = 2;
pub const SI_FIELD_VIRTUAL_MOD: si_field = 1;
// C2Rust_Unnamed_19 removed: replaced by Vec<xkb_sym_interpret>
pub struct collect {
    pub sym_interprets: Vec<xkb_sym_interpret>,
}
// C2Rust_Unnamed_20 removed: replaced by Vec<xkb_action>
fn siText(si: &SymInterpInfo, info: &mut CompatInfo, ki: &xkb_keymap_info<'_>) -> String {
    if std::ptr::eq(si, &info.default_interp) {
        return "default".to_string();
    }
    format!(
        "{}+{}({})",
        KeysymText(si.interp.sym),
        SIMatchText(si.interp.match_0),
        ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, si.interp.mods),
    )
}
#[inline]
fn ReportSINotArray(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    si: &SymInterpInfo,
    field: &str,
) -> bool {
    ReportNotArray("symbol interpretation", field, &siText(si, info, ki))
}
#[inline]
fn ReportSIBadType(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    si: &SymInterpInfo,
    field: &str,
    wanted: &str,
) -> bool {
    ReportBadType(
        XKB_ERROR_WRONG_FIELD_TYPE,
        "symbol interpretation",
        field,
        &siText(si, info, ki),
        wanted,
    )
}
fn LEDText<'a>(info: &'a CompatInfo, ki: &'a xkb_keymap_info<'_>, ledi: &LedInfo) -> &'a str {
    if std::ptr::eq(ledi, &info.default_led) {
        "default"
    } else {
        xkb_atom_text(&ki.ctx().atom_table, ledi.led.name)
    }
}
#[inline]
fn ReportLedBadType(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    ledi: &LedInfo,
    field: &str,
    wanted: &str,
) -> bool {
    ReportBadType(
        XKB_ERROR_WRONG_FIELD_TYPE,
        "indicator map",
        field,
        LEDText(info, ki, ledi),
        wanted,
    )
}
#[inline]
fn ReportLedNotArray(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    ledi: &LedInfo,
    field: &str,
) -> bool {
    ReportNotArray("indicator map", field, LEDText(info, ki, ledi))
}
#[inline]
fn InitInterp(info: &mut SymInterpInfo) {
    info.merge = MERGE_DEFAULT;
    info.interp.virtual_mod = XKB_MOD_INVALID;
}
#[inline]
fn InitLED(info: &mut LedInfo) {
    info.merge = MERGE_DEFAULT;
}
fn InitCompatInfo(
    ki: &xkb_keymap_info<'_>,
    info: &mut CompatInfo,
    include_depth: u32,
    mods: &xkb_mod_set,
) {
    info.include_depth = include_depth;
    InitActionsInfo(ki.keymap_ref(), &mut info.default_actions);
    InitVMods(&mut info.mods, mods, include_depth > 0_u32);
    InitInterp(&mut info.default_interp);
    InitLED(&mut info.default_led);
}
fn ClearCompatInfo(info: &mut CompatInfo) {
    info.name = None;
    info.interps.clear();
}
fn UseNewInterpField(
    field: si_field,
    old: si_field,
    new: si_field,
    clobber: bool,
    report: bool,
    collide: &mut si_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            *collide = (*collide | field) as si_field;
        }
        return clobber;
    }
    false
}
fn MergeInterp(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    old: &mut SymInterpInfo,
    new: &mut SymInterpInfo,
    same_file: bool,
) -> bool {
    let clobber: bool = new.merge != MERGE_AUGMENT;
    let verbosity: i32 = xkb_context_get_log_verbosity(ki.ctx());
    let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
    let mut collide: si_field = 0 as si_field;
    if new.merge == MERGE_REPLACE {
        if report {
            log::warn!(
                "Multiple definitions for \"{}\"; Earlier interpretation ignored\n",
                siText(new, info, ki)
            );
        }
        *old = new.clone();
        return true;
    }
    if UseNewInterpField(
        SI_FIELD_VIRTUAL_MOD,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.interp.virtual_mod = new.interp.virtual_mod;
        old.defined = (old.defined | SI_FIELD_VIRTUAL_MOD) as si_field;
    }
    if UseNewInterpField(
        SI_FIELD_ACTION,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        if old.interp.num_actions as i32 > 1_i32 {
            old.interp.actions.clear();
        }
        old.interp.num_actions = new.interp.num_actions;
        if new.interp.num_actions as i32 > 1_i32 {
            old.interp.actions = std::mem::take(&mut new.interp.actions);
            new.interp.action = xkb_action::None;
            new.interp.num_actions = 0_u16;
        } else {
            old.interp.action = new.interp.action;
        }
        old.defined = (old.defined | SI_FIELD_ACTION) as si_field;
    }
    if UseNewInterpField(
        SI_FIELD_AUTO_REPEAT,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.interp.repeat = new.interp.repeat;
        old.defined = (old.defined | SI_FIELD_AUTO_REPEAT) as si_field;
    }
    if UseNewInterpField(
        SI_FIELD_LEVEL_ONE_ONLY,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.interp.level_one_only = new.interp.level_one_only;
        old.defined = (old.defined | SI_FIELD_LEVEL_ONE_ONLY) as si_field;
    }
    if collide as u64 != 0 {
        log::warn!(
            "Multiple interpretations of \"{}\"; Using {} definition for duplicate fields\n",
            siText(old, info, ki),
            if clobber { "last" } else { "first" }
        );
    }
    true
}
fn AddInterp(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    new: &mut SymInterpInfo,
    same_file: bool,
) -> bool {
    // FindMatchingInterp inlined
    let mut old_idx: Option<usize> = None;
    for i in 0..info.interps.len() {
        if info.interps[i].interp.sym == new.interp.sym
            && info.interps[i].interp.mods == new.interp.mods
            && info.interps[i].interp.match_0 == new.interp.match_0
        {
            old_idx = Some(i);
            break;
        }
    }
    if let Some(idx) = old_idx {
        // Clone the old element out to avoid borrow conflict with info
        let mut old = info.interps[idx].clone();
        let result = MergeInterp(info, ki, &mut old, new, same_file);
        info.interps[idx] = old;
        return result;
    }
    info.interps.push(new.clone());
    true
}
fn ResolveStateAndPredicate(
    expr: Option<&ExprDef>,
    pred_rtrn: &mut u32,
    mods_rtrn: &mut u32,
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
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
    let resolve_expr: &ExprDef;
    if expr.stmt_type() == STMT_EXPR_ACTION_DECL {
        let ExprKind::Action { name, args } = &expr.kind else {
            unreachable!()
        };
        let pred_txt: &str = xkb_atom_text(&ki.ctx().atom_table, *name);
        let mut pred: u32 = 0_u32;
        if !LookupString(&symInterpretMatchMaskNames, pred_txt, &mut pred)
            || args.is_empty()
            || args.len() != 1
        {
            log::error!("Illegal modifier predicate \"{}\"; Ignored\n", pred_txt);
            return false;
        }
        *pred_rtrn = pred;
        resolve_expr = &args[0];
    } else if expr.stmt_type() == STMT_EXPR_IDENT {
        let ExprKind::Ident(ident_val) = &expr.kind else {
            unreachable!()
        };
        let pred_txt_0: &str = xkb_atom_text(&ki.ctx().atom_table, *ident_val);
        if !pred_txt_0.is_empty() && pred_txt_0.eq_ignore_ascii_case("any") {
            *pred_rtrn = MATCH_ANY;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        resolve_expr = expr;
    } else {
        resolve_expr = expr;
    }
    ExprResolveModMask(ki.ctx(), resolve_expr, MOD_REAL, &info.mods, mods_rtrn)
}
fn UseNewLEDField(
    field: led_field,
    old: led_field,
    new: led_field,
    clobber: bool,
    report: bool,
    collide: &mut led_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            *collide = (*collide | field) as led_field;
        }
        return clobber;
    }
    false
}
fn MergeLedMap(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    old: &mut LedInfo,
    new: &mut LedInfo,
    same_file: bool,
) -> bool {
    let mut collide: led_field;
    let clobber: bool = new.merge != MERGE_AUGMENT;
    let verbosity: i32 = xkb_context_get_log_verbosity(ki.ctx());
    let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
    if old.led.mods.mods == new.led.mods.mods
        && old.led.pending_groups as i32 == new.led.pending_groups as i32
        && old.led.groups == new.led.groups
        && old.led.ctrls == new.led.ctrls
        && old.led.which_mods == new.led.which_mods
        && old.led.which_groups as i32 == new.led.which_groups as i32
    {
        old.defined = (old.defined | new.defined) as led_field;
        return true;
    }
    if new.merge == MERGE_REPLACE {
        if report {
            log::warn!(
                "Map for indicator {} redefined; Earlier definition ignored\n",
                LEDText(info, ki, old)
            );
        }
        *old = *new;
        return true;
    }
    collide = 0 as led_field;
    if UseNewLEDField(
        LED_FIELD_MODS,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.led.which_mods = new.led.which_mods;
        old.led.mods = new.led.mods;
        old.defined = (old.defined | LED_FIELD_MODS) as led_field;
    }
    if UseNewLEDField(
        LED_FIELD_GROUPS,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.led.which_groups = new.led.which_groups;
        old.led.groups = new.led.groups;
        old.led.pending_groups = new.led.pending_groups;
        old.defined = (old.defined | LED_FIELD_GROUPS) as led_field;
    }
    if UseNewLEDField(
        LED_FIELD_CTRLS,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.led.ctrls = new.led.ctrls;
        old.defined = (old.defined | LED_FIELD_CTRLS) as led_field;
    }
    if collide as u64 != 0 {
        log::warn!(
            "Map for indicator {} redefined; Using {} definition for duplicate fields\n",
            LEDText(info, ki, old),
            if clobber { "last" } else { "first" }
        );
    }
    true
}
fn AddLedMap(
    info: &mut CompatInfo,
    ki: &xkb_keymap_info<'_>,
    new: &mut LedInfo,
    same_file: bool,
) -> bool {
    let mut i: u32 = 0_u32;
    while i < info.num_leds {
        if info.leds[i as usize].led.name != new.led.name {
            i = i.wrapping_add(1);
        } else {
            // Clone the old element out to avoid borrow conflict with info
            let mut old = info.leds[i as usize];
            let result = MergeLedMap(info, ki, &mut old, new, same_file);
            info.leds[i as usize] = old;
            return result;
        }
    }
    if info.num_leds >= XKB_MAX_LEDS {
        log::error!(
            "Too many LEDs defined (maximum {})\n",
            (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32
        );
        return false;
    }
    let c2rust_fresh1 = info.num_leds;
    info.num_leds = info.num_leds.wrapping_add(1);
    info.leds[c2rust_fresh1 as usize] = *new;
    true
}
fn MergeIncludedCompatMaps(
    ki: &mut xkb_keymap_info<'_>,
    into: &mut CompatInfo,
    from: &mut CompatInfo,
    merge: merge_mode,
) {
    if from.errorCount > 0_i32 {
        into.errorCount += from.errorCount;
        return;
    }
    MergeModSets(ki.ctx_mut(), &mut into.mods, &from.mods, merge);
    if into.name.is_none() {
        into.name = from.name.take();
    }
    if into.interps.is_empty() {
        into.interps = std::mem::take(&mut from.interps);
    } else {
        for i in 0..from.interps.len() {
            (&mut from.interps)[i].merge = merge;
            let si = &mut from.interps[i];
            if !AddInterp(into, ki, si, false) {
                into.errorCount += 1;
            }
        }
    }
    if into.num_leds == 0_u32 {
        let n = from.num_leds as usize;
        into.leds[..n].copy_from_slice(&from.leds[..n]);
        into.num_leds = from.num_leds;
        from.num_leds = 0_u32;
    } else {
        for i in 0..from.num_leds as usize {
            from.leds[i].merge = merge;
            let ledi = &mut from.leds[i];
            if !AddLedMap(into, ki, ledi, false) {
                into.errorCount += 1;
            }
        }
    };
}
fn HandleIncludeCompatMap(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut CompatInfo,
    include: &mut IncludeStmt,
) -> bool {
    let mut included = CompatInfo::new();
    if ExceedsIncludeMaxDepth(info.include_depth) {
        info.errorCount += 10_i32;
        return false;
    }
    InitCompatInfo(
        ki,
        &mut included,
        info.include_depth.wrapping_add(1_u32),
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

        let file: Option<Box<XkbFile>> = ProcessIncludeFile(ki.ctx_mut(), stmt, FILE_TYPE_COMPAT);
        let Some(mut file) = file else {
            info.errorCount += 10_i32;
            ClearCompatInfo(&mut included);
            return false;
        };
        InitCompatInfo(
            ki,
            &mut next_incl,
            info.include_depth.wrapping_add(1_u32),
            &included.mods,
        );
        next_incl.default_interp = info.default_interp.clone();
        next_incl.default_led = info.default_led;
        HandleCompatMapFile(ki, &mut next_incl, &mut file);
        MergeIncludedCompatMaps(ki, &mut included, &mut next_incl, stmt.merge);
        ClearCompatInfo(&mut next_incl);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    MergeIncludedCompatMaps(ki, info, &mut included, include.merge);
    ClearCompatInfo(&mut included);
    info.errorCount == 0_i32
}
fn SetInterpField(
    info: &mut CompatInfo,
    ki: &mut xkb_keymap_info<'_>,
    si: &mut SymInterpInfo,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    value: &mut ExprDef,
) -> bool {
    if field.eq_ignore_ascii_case("action") {
        if arrayNdx.is_some() {
            return ReportSINotArray(info, ki, si, field);
        }
        if value.stmt_type() == STMT_EXPR_ACTION_LIST {
            let ExprKind::ActionList {
                actions: action_vec,
            } = &mut value.kind
            else {
                unreachable!()
            };
            let num_actions: u32 = action_vec.len() as u32;
            if num_actions > MAX_ACTIONS_PER_LEVEL as u32 {
                log::error!(
                    "Interpret {} has too many actions; expected max {}, got: {}\n",
                    &siText(si, info, ki),
                    65535_i32,
                    num_actions
                );
                return false;
            }
            si.interp.num_actions = 0_u16;
            si.interp.action.set_none();
            let mut actions: Vec<xkb_action> = Vec::new();
            for act_expr in action_vec.iter_mut() {
                let mut toAct: xkb_action = xkb_action::None;
                match HandleActionDef(
                    ki,
                    &mut info.default_actions,
                    &info.mods,
                    act_expr,
                    &mut toAct,
                ) {
                    1 => {
                        toAct.set_none();
                    }
                    2 => {
                        drop(actions);
                        return false;
                    }
                    _ => {}
                }
                if toAct.action_type() != ACTION_TYPE_NONE {
                    if (num_actions == 1_u32) as i64 != 0 {
                        si.interp.num_actions = 1_u16;
                        si.interp.action = toAct;
                    } else {
                        actions.push(toAct);
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
            match HandleActionDef(
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
        si.defined = (si.defined | SI_FIELD_ACTION) as si_field;
    } else if field.eq_ignore_ascii_case("virtualmodifier")
        || field.eq_ignore_ascii_case("virtualmod")
    {
        if arrayNdx.is_some() {
            return ReportSINotArray(info, ki, si, field);
        }
        let mut ndx: u32 = 0_u32;
        if !ExprResolveMod(ki.ctx(), value, MOD_VIRT, &info.mods, &mut ndx) {
            return ReportSIBadType(info, ki, si, field, "virtual modifier");
        }
        si.interp.virtual_mod = ndx;
        si.defined = (si.defined | SI_FIELD_VIRTUAL_MOD) as si_field;
    } else if field.eq_ignore_ascii_case("repeat") {
        let mut set: bool = false;
        if arrayNdx.is_some() {
            return ReportSINotArray(info, ki, si, field);
        }
        if !ExprResolveBoolean(ki.ctx(), value, &mut set) {
            return ReportSIBadType(info, ki, si, field, "boolean");
        }
        si.interp.repeat = set;
        si.defined = (si.defined | SI_FIELD_AUTO_REPEAT) as si_field;
    } else if field.eq_ignore_ascii_case("locking") {
        log::debug!("The \"locking\" field in symbol interpretation is unsupported; Ignored\n");
    } else if field.eq_ignore_ascii_case("usemodmap") || field.eq_ignore_ascii_case("usemodmapmods")
    {
        let mut val: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportSINotArray(info, ki, si, field);
        }
        if !ExprResolveEnum(ki.ctx(), value, &mut val, &useModMapValueNames) {
            return ReportSIBadType(info, ki, si, field, "level specification");
        }
        si.interp.level_one_only = val != 0;
        si.defined = (si.defined | SI_FIELD_LEVEL_ONE_ONLY) as si_field;
    } else {
        ReportBadField("symbol interpretation", field, &siText(si, info, ki));
        return ki.strict & PARSER_NO_UNKNOWN_INTERPRET_FIELDS == 0;
    }
    true
}
fn SetLedMapField(
    info: &mut CompatInfo,
    ki: &mut xkb_keymap_info<'_>,
    ledi: &mut LedInfo,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    value_opt: &mut Option<Box<ExprDef>>,
) -> bool {
    let value: &ExprDef = value_opt.as_deref().unwrap();
    if field.eq_ignore_ascii_case("modifiers") || field.eq_ignore_ascii_case("mods") {
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ki, ledi, field);
        }
        if !ExprResolveModMask(
            ki.ctx(),
            value,
            MOD_BOTH,
            &info.mods,
            &mut ledi.led.mods.mods,
        ) {
            return ReportLedBadType(info, ki, ledi, field, "modifier mask");
        }
        ledi.defined = (ledi.defined | LED_FIELD_MODS) as led_field;
    } else if field.eq_ignore_ascii_case("groups") {
        let mut mask: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ki, ledi, field);
        }
        let mut pending: bool = false;
        if !ExprResolveGroupMask(ki, value, &mut mask, &mut pending) {
            if pending {
                ledi.led.pending_groups = true;
                let pending_index: u32 = ki.pending_computations.len() as u32;
                ki.pending_computations.push(pending_computation {
                    expr: value_opt.take(),
                    computed: false,
                    value: 0_u32,
                });
                mask = pending_index;
            } else {
                return ReportLedBadType(info, ki, ledi, field, "group mask");
            }
        } else {
            ledi.led.pending_groups = false;
        }
        ledi.led.groups = mask;
        ledi.defined = (ledi.defined | LED_FIELD_GROUPS) as led_field;
    } else if field.eq_ignore_ascii_case("controls") || field.eq_ignore_ascii_case("ctrls") {
        let mut mask_0: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ki, ledi, field);
        }
        let offset: u8 = ki.features.controls_name_offset;
        if !ExprResolveMask(
            ki.ctx(),
            value,
            &mut mask_0,
            &ctrlMaskNames[offset as usize..],
        ) {
            return ReportLedBadType(info, ki, ledi, field, "controls mask");
        }
        ledi.led.ctrls = mask_0 as xkb_action_controls;
        ledi.defined = (ledi.defined | LED_FIELD_CTRLS) as led_field;
    } else if field.eq_ignore_ascii_case("allowexplicit") {
        log::debug!(
            "The \"allowExplicit\" field in indicator statements is unsupported; Ignored\n"
        );
    } else if field.eq_ignore_ascii_case("whichmodstate")
        || field.eq_ignore_ascii_case("whichmodifierstate")
    {
        let mut mask_1: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ki, ledi, field);
        }
        if !ExprResolveMask(ki.ctx(), value, &mut mask_1, &modComponentMaskNames) {
            return ReportLedBadType(info, ki, ledi, field, "mask of modifier state components");
        }
        ledi.led.which_mods = mask_1;
    } else if field.eq_ignore_ascii_case("whichgroupstate") {
        let mut mask_2: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ki, ledi, field);
        }
        if !ExprResolveMask(ki.ctx(), value, &mut mask_2, &groupComponentMaskNames) {
            return ReportLedBadType(info, ki, ledi, field, "mask of group state components");
        }
        ledi.led.which_groups = mask_2;
    } else if field.eq_ignore_ascii_case("driveskbd")
        || field.eq_ignore_ascii_case("driveskeyboard")
        || field.eq_ignore_ascii_case("leddriveskbd")
        || field.eq_ignore_ascii_case("leddriveskeyboard")
        || field.eq_ignore_ascii_case("indicatordriveskbd")
        || field.eq_ignore_ascii_case("indicatordriveskeyboard")
    {
        log::debug!(
            "The \"{}\" field in indicator statements is unsupported; Ignored\n",
            field
        );
    } else if field.eq_ignore_ascii_case("index") {
        log::error!("The \"index\" field in indicator statements is unsupported; Ignored\n");
    } else {
        log::error!(
            "Unknown field \"{}\" in map for {} indicator; Definition ignored\n",
            field,
            LEDText(info, ki, ledi)
        );
        return ki.strict & PARSER_NO_UNKNOWN_LED_FIELDS == 0;
    }
    true
}
fn HandleCompatGlobalVar(
    info: &mut CompatInfo,
    ki: &mut xkb_keymap_info<'_>,
    stmt: &mut VarDef,
) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut ndx: Option<&ExprDef> = None;
    let ret: bool;
    if !ExprResolveLhs(
        stmt.name.as_deref().unwrap(),
        &mut elem_atom,
        &mut field_atom,
        &mut ndx,
    ) {
        ret = false;
    } else {
        let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom).to_owned();
        let field = xkb_atom_text(&ki.ctx().atom_table, field_atom).to_owned();
        if !elem.is_empty() && elem.eq_ignore_ascii_case("interpret") {
            let mut temp: SymInterpInfo = SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat: false,
                    required: false,
                    num_actions: 0,
                    action: xkb_action::None,
                    actions: Vec::new(),
                },
            };
            InitInterp(&mut temp);
            temp.merge = (if temp.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                stmt.merge
            }) as merge_mode;
            // ndx borrows from stmt.name, value_ref borrows from stmt.value — disjoint fields
            let value_ref = stmt.value.as_deref_mut().unwrap();
            ret = SetInterpField(info, ki, &mut temp, &field, ndx, value_ref);
            if ret {
                let mut default = info.default_interp.clone();
                MergeInterp(info, ki, &mut default, &mut temp, true);
                info.default_interp = default;
            }
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("indicator") {
            let mut temp_0: LedInfo = LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups: 0,
                    pending_groups: false,
                    groups: 0,
                    which_mods: 0_u32,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            };
            InitLED(&mut temp_0);
            temp_0.merge = (if temp_0.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                stmt.merge
            }) as merge_mode;
            // ndx borrows from stmt.name, field/value borrow from different fields — disjoint
            ret = SetLedMapField(info, ki, &mut temp_0, &field, ndx, &mut stmt.value);
            if ret {
                let mut default = info.default_led;
                MergeLedMap(info, ki, &mut default, &mut temp_0, true);
                info.default_led = default;
            }
        } else if !elem.is_empty() {
            ret = {
                let ndx_ref2 = ndx;

                SetDefaultActionField(
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
            log::error!(
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                field
            );
            return ki.strict & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS == 0;
        }
    } // close else from ExprResolveLhs
    ret
}
fn HandleInterpBody(
    info: &mut CompatInfo,
    ki: &mut xkb_keymap_info<'_>,
    defs: &mut [VarDef],
    si: &mut SymInterpInfo,
) -> bool {
    let mut ok: bool = true;
    for def in defs {
        let mut elem_atom: u32 = 0;
        let mut field_atom: u32 = 0;
        let mut arrayNdx: Option<&ExprDef> = None;
        if !ExprResolveLhs(
            def.name.as_deref().unwrap(),
            &mut elem_atom,
            &mut field_atom,
            &mut arrayNdx,
        ) {
            ok = false;
        } else {
            let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom).to_owned();
            let field = xkb_atom_text(&ki.ctx().atom_table, field_atom).to_owned();
            if !elem.is_empty() {
                log::error!("Cannot set a global default value for \"{}\" element from within an interpret statement; Move assignment to \"{}.{}\" to the global file scope\n",
                    elem,
                    elem,
                    field);
                ok = false;
            } else {
                let value_ref = def.value.as_deref_mut().unwrap();
                if !SetInterpField(info, ki, si, &field, arrayNdx, value_ref) {
                    ok = false;
                }
            }
        }
    }
    ok
}
fn HandleInterpDef(
    info: &mut CompatInfo,
    ki: &mut xkb_keymap_info<'_>,
    def: &mut InterpDef,
) -> bool {
    let mut pred: u32 = MATCH_NONE;
    let mut mods: u32 = 0;
    #[allow(unused_assignments)]
    let mut si: SymInterpInfo = SymInterpInfo {
        defined: 0 as si_field,
        merge: MERGE_DEFAULT,
        interp: xkb_sym_interpret {
            sym: 0,
            match_0: MATCH_NONE,
            mods: 0,
            virtual_mod: 0,
            level_one_only: false,
            repeat: false,
            required: false,
            num_actions: 0,
            action: xkb_action::None,
            actions: Vec::new(),
        },
    };
    if !ResolveStateAndPredicate(def.match_0.as_deref(), &mut pred, &mut mods, info, ki) {
        log::error!("Couldn't determine matching modifiers; Symbol interpretation ignored\n");
        return false;
    }
    si = info.default_interp.clone();
    si.merge = def.merge;
    si.interp.sym = def.sym;
    si.interp.match_0 = pred;
    si.interp.mods = mods;
    if !HandleInterpBody(info, ki, &mut def.def, &mut si) {
        info.errorCount += 1;
        return false;
    }
    if !AddInterp(info, ki, &mut si, true) {
        info.errorCount += 1;
        return false;
    }
    true
}
fn HandleLedMapDef(
    info: &mut CompatInfo,
    ki: &mut xkb_keymap_info<'_>,
    def: &mut LedMapDef,
) -> bool {
    let mut ledi: LedInfo = info.default_led;
    ledi.merge = def.merge;
    ledi.led.name = def.name;
    let mut ok: bool = true;
    for var in def.body.iter_mut() {
        let mut elem_atom: u32 = 0;
        let mut field_atom: u32 = 0;
        let mut arrayNdx: Option<&ExprDef> = None;
        if !ExprResolveLhs(
            var.name.as_deref().unwrap(),
            &mut elem_atom,
            &mut field_atom,
            &mut arrayNdx,
        ) {
            ok = false;
        } else {
            let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom).to_owned();
            let field = xkb_atom_text(&ki.ctx().atom_table, field_atom).to_owned();
            if !elem.is_empty() {
                log::error!("[XKB-{:03}] Cannot set defaults for \"{}\" element in indicator map; Assignment to {}.{} ignored\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    elem,
                    elem,
                    field);
                ok = false;
            } else if !SetLedMapField(info, ki, &mut ledi, &field, arrayNdx, &mut var.value) {
                ok = false;
            }
        }
    }
    ok && AddLedMap(info, ki, &mut ledi, true)
}
fn HandleCompatMapFile(ki: &mut xkb_keymap_info<'_>, info: &mut CompatInfo, file: &mut XkbFile) {
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
                    ok = HandleIncludeCompatMap(ki, info, incl);
                }
                Statement::Interp(ip) => {
                    ok = HandleInterpDef(info, ki, ip);
                }
                Statement::GroupCompat(_) => {
                    log::debug!("The \"group\" statement in compat is unsupported; Ignored\n");
                    ok = true;
                }
                Statement::LedMap(lm) => {
                    ok = HandleLedMapDef(info, ki, lm);
                }
                Statement::Var(var) => {
                    ok = HandleCompatGlobalVar(info, ki, var);
                }
                Statement::VMod(vmod) => {
                    ok = HandleVModDef(ki.ctx_mut(), &mut info.mods, vmod);
                }
                Statement::Unknown(unk) => {
                    log::error!(
                        "[XKB-{:03}] Unsupported compatibility {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        if unk.stmt_type == STMT_UNKNOWN_COMPOUND {
                            "compound"
                        } else {
                            "declaration"
                        },
                        &unk.name
                    );
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    log::error!(
                        "Compat files may not include other types; Ignoring {}\n",
                        stmt_type_to_string(stmt.stmt_type())
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                log::error!("Abandoning compatibility map \"{}\"\n", safe_map_name(file));
                break;
            }
        }
    }
}
fn CopyInterps(info: &CompatInfo, needSymbol: bool, pred: u32, collect: &mut collect) {
    for i in 0..info.interps.len() {
        let si = &info.interps[i];
        if si.interp.match_0 == pred
            && (si.interp.sym != XKB_KEY_NoSymbol as u32) as i32 == needSymbol as i32
        {
            collect.sym_interprets.push(si.interp.clone());
        }
    }
}
fn CopyLedMapDefsToKeymap(ki: &mut xkb_keymap_info<'_>, info: &mut CompatInfo) {
    let keymap = ki.keymap_mut();
    let mut c2rust_current_block_11: u64;
    let mut idx: u32 = 0_u32;
    while idx < info.num_leds {
        let ledi_led = info.leds[idx as usize].led;
        let is_default = std::ptr::eq(
            &info.leds[idx as usize] as *const LedInfo,
            &info.default_led as *const LedInfo,
        );
        let led_name_text = if is_default {
            "default"
        } else {
            xkb_atom_text(&keymap.ctx.atom_table, info.leds[idx as usize].led.name)
        };
        let mut i: u32;
        i = 0_u32;
        while i < keymap.num_leds {
            if keymap.leds[i as usize].name == ledi_led.name {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i >= keymap.num_leds {
            log::debug!("Indicator name \"{}\" was not declared in the keycodes section; Adding new indicator\n",
                led_name_text);
            i = 0_u32;
            while i < keymap.num_leds {
                if keymap.leds[i as usize].name == XKB_ATOM_NONE {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i >= keymap.num_leds {
                if i >= XKB_MAX_LEDS {
                    log::error!(
                        "Too many indicators (maximum is {}); Indicator name \"{}\" ignored\n",
                        (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32,
                        led_name_text
                    );
                    c2rust_current_block_11 = 792017965103506125;
                } else {
                    i = keymap.num_leds;
                    keymap.num_leds = keymap.num_leds.wrapping_add(1);
                    c2rust_current_block_11 = 17860125682698302841;
                }
            } else {
                c2rust_current_block_11 = 17860125682698302841;
            }
        } else {
            c2rust_current_block_11 = 17860125682698302841;
        }
        if c2rust_current_block_11 == 17860125682698302841 {
            keymap.leds[i as usize] = ledi_led;
            let led = &mut keymap.leds[i as usize];
            if led.which_groups as i32 == 0_i32
                && (led.groups != 0_u32 || led.pending_groups as i32 != 0)
            {
                led.which_groups = XKB_STATE_LAYOUT_EFFECTIVE;
            }
            if led.which_mods == 0_u32 && led.mods.mods != 0_u32 {
                led.which_mods = XKB_STATE_MODS_EFFECTIVE;
            }
        }
        idx = idx.wrapping_add(1);
    }
}
fn CopyCompatToKeymap(ki: &mut xkb_keymap_info<'_>, info: &mut CompatInfo) -> bool {
    // Collect sym_interprets first (doesn't need keymap)
    let sym_interprets = if !info.interps.is_empty() {
        let mut collect: collect = collect {
            sym_interprets: Vec::with_capacity(info.interps.len()),
        };
        CopyInterps(info, true, MATCH_EXACTLY, &mut collect);
        CopyInterps(info, true, MATCH_ALL, &mut collect);
        CopyInterps(info, true, MATCH_NONE, &mut collect);
        CopyInterps(info, true, MATCH_ANY, &mut collect);
        CopyInterps(info, true, MATCH_ANY_OR_NONE, &mut collect);
        CopyInterps(info, false, MATCH_EXACTLY, &mut collect);
        CopyInterps(info, false, MATCH_ALL, &mut collect);
        CopyInterps(info, false, MATCH_NONE, &mut collect);
        CopyInterps(info, false, MATCH_ANY, &mut collect);
        CopyInterps(info, false, MATCH_ANY_OR_NONE, &mut collect);
        Some(collect.sym_interprets)
    } else {
        None
    };
    // Now get keymap and assign everything
    {
        let keymap = ki.keymap_mut();
        keymap.compat_section_name = match &info.name {
            Some(s) => s.clone(),
            None => String::new(),
        };
        xkb_escape_map_name(&mut keymap.compat_section_name);
        keymap.mods = info.mods;
        if let Some(interps) = sym_interprets {
            keymap.sym_interprets = interps;
        }
    }
    // CopyLedMapDefsToKeymap needs keymap borrow ended; scope block ensures this
    CopyLedMapDefsToKeymap(ki, info);
    true
}
pub fn CompileCompatMap(file: Option<&mut XkbFile>, ki: &mut xkb_keymap_info<'_>) -> bool {
    let mods = ki.keymap_ref().mods;
    let mut info = CompatInfo::new();
    InitCompatInfo(ki, &mut info, 0_u32, &mods);
    if let Some(file) = file {
        HandleCompatMapFile(ki, &mut info, file);
    }
    if (info.errorCount == 0_i32) && CopyCompatToKeymap(ki, &mut info) {
        ClearCompatInfo(&mut info);
        return true;
    }
    ClearCompatInfo(&mut info);
    false
}
