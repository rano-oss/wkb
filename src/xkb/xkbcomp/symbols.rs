use super::prelude::*;
use crate::xkb::context::xkb_atom_intern;
pub use crate::xkb::keymap::{XkbLevelsSameActions, XkbLevelsSameSyms, XkbModNameToIndex};
use crate::xkb::keysym::xkb_keysym_is_keypad;
use crate::xkb::keysym_case_mappings::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
pub use crate::xkb::shared_ast_types::{ModMapDef, SymbolsDef};
pub use crate::xkb::shared_types::{
    XkbKeyByName, XkbKeyNumLevels, XKB_MOD_NONE, XKB_OVERLAY_INVALID,
};
use crate::xkb::text::ModIndexText;
use crate::xkb::utils::cstr_free;
use crate::xkb::utils::{istrneq, memdup};
pub use crate::xkb::xkbcomp::action::{
    ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField,
};
use libc::{abort, realloc};
#[derive(Clone)]
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
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModMapEntry {
    pub merge: merge_mode,
    pub haveSymbol: bool,
    pub modifier: u32,
    pub u: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_19 {
    pub keyName: u32,
    pub keySym: u32,
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
    pub overlay_keys: Vec<*const xkb_key>,
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
#[derive(Clone)]
pub struct GroupInfo {
    pub levels: Vec<xkb_level>,
    pub defined: group_field,
    pub type_0: u32,
}

impl Default for GroupInfo {
    fn default() -> Self {
        Self {
            levels: Vec::new(),
            defined: 0,
            type_0: 0,
        }
    }
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
    pub fn new_zeroed() -> Self {
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: Vec::new(),
            default_key: KeyInfo::new_zeroed(),
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            group_names: Vec::new(),
            modmaps: Vec::new(),
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as u32,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: std::ptr::null_mut(),
            keymap_info: std::ptr::null(),
        }
    }
}

/// Resize a Vec<T> to `new_len`, zero-initializing any new elements.
/// If `new_len` < current len, the Vec is truncated.
/// WARNING: Only safe for types where all-zeros is a valid representation.
/// For types containing Vec/String/Box, use resize_groups_zero or similar.
unsafe fn vec_resize_zero<T>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        let ptr = v.as_mut_ptr().add(old_len);
        std::ptr::write_bytes(ptr, 0, new_len - old_len);
        v.set_len(new_len);
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

/// Resize a Vec<GroupInfo> to `new_len`, properly initializing new elements.
/// Unlike vec_resize_zero, this correctly initializes Vec fields in GroupInfo.
unsafe fn resize_groups_zero(v: &mut Vec<GroupInfo>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        // Zero-fill the raw memory first (for scalar fields)
        let ptr = v.as_mut_ptr().add(old_len);
        std::ptr::write_bytes(ptr, 0, new_len - old_len);
        v.set_len(new_len);
        // Now properly initialize Vec fields in new elements
        for i in old_len..new_len {
            std::ptr::write(&raw mut (*v.as_mut_ptr().add(i)).levels, Vec::new());
        }
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

unsafe fn InitGroupInfo(mut groupi: *mut GroupInfo) {
    unsafe {
        std::ptr::write(
            groupi,
            GroupInfo {
                levels: Vec::new(),
                defined: 0 as group_field,
                type_0: 0,
            },
        );
    }
}
unsafe fn ClearGroupInfo(mut groupi: *mut GroupInfo) {
    unsafe {
        (*groupi).levels.clear();
    }
}
unsafe fn CopyGroupInfo(mut to: *mut GroupInfo, mut from: *const GroupInfo) {
    unsafe {
        (*to).defined = (*from).defined;
        (*to).type_0 = (*from).type_0;
        (*to).levels = (*from).levels.clone();
    }
}
unsafe fn InitKeyInfo(mut ctx: *mut xkb_context, mut keyi: *mut KeyInfo) {
    unsafe {
        std::ptr::write_bytes::<KeyInfo>(keyi as *mut KeyInfo, 0u8, 1);
        (*keyi).name = xkb_atom_intern(
            ctx,
            b"*\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 2]>()).wrapping_sub(1 as usize),
        );
        (*keyi).out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_WRAP as u32;
        std::ptr::write(&raw mut (*keyi).groups, Vec::new());
        std::ptr::write(&raw mut (*keyi).overlay_keys, Vec::new());
    }
}
unsafe fn ClearKeyInfo(mut keyi: *mut KeyInfo) {
    unsafe {
        for groupi in (*keyi).groups.iter_mut() {
            ClearGroupInfo(groupi as *mut GroupInfo);
        }
        (*keyi).groups.clear();
        (*keyi).overlay_keys.clear();
    }
}
unsafe fn InitSymbolsInfo(
    mut info: *mut SymbolsInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        std::ptr::write_bytes::<SymbolsInfo>(info as *mut SymbolsInfo, 0u8, 1);
        std::ptr::write(&raw mut (*info).keys, Vec::new());
        std::ptr::write(&raw mut (*info).group_names, Vec::new());
        std::ptr::write(&raw mut (*info).modmaps, Vec::new());
        (*info).ctx = &raw mut (*(*keymap_info).keymap).ctx;
        (*info).include_depth = include_depth;
        (*info).keymap_info = keymap_info;
        (*info).max_groups = (*keymap_info).features.max_groups;
        InitKeyInfo(
            &raw mut (*(*keymap_info).keymap).ctx,
            &raw mut (*info).default_key,
        );
        InitActionsInfo((*keymap_info).keymap, &raw mut (*info).default_actions);
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0 as u32);
        (*info).explicit_group = XKB_LAYOUT_INVALID as u32;
    }
}
unsafe fn ClearSymbolsInfo(mut info: *mut SymbolsInfo) {
    unsafe {
        (*info).name = None;
        for keyi in (*info).keys.iter_mut() {
            ClearKeyInfo(keyi as *mut KeyInfo);
        }
        (*info).keys.clear();
        (*info).group_names.clear();
        (*info).modmaps.clear();
        ClearKeyInfo(&raw mut (*info).default_key);
    }
}
unsafe fn KeyInfoText(mut info: *mut SymbolsInfo, mut keyi: *mut KeyInfo) -> &'static [u8] {
    unsafe {
        return KeyNameText((*((*info).ctx)).clone(), (*keyi).name);
    }
}
unsafe fn MergeGroups(
    mut info: *mut SymbolsInfo,
    mut into: *mut GroupInfo,
    mut from: *mut GroupInfo,
    mut clobber: bool,
    mut report: bool,
    mut group: u32,
    mut key_name: u32,
) -> bool {
    unsafe {
        if (*into).type_0 != (*from).type_0 {
            if !((*from).type_0 == XKB_ATOM_NONE as u32) {
                if (*into).type_0 == XKB_ATOM_NONE as u32 {
                    (*into).type_0 = (*from).type_0;
                } else {
                    let mut use_0: u32 = if clobber as i32 != 0 {
                        (*from).type_0
                    } else {
                        (*into).type_0
                    };
                    let mut ignore: u32 = if clobber as i32 != 0 {
                        (*into).type_0
                    } else {
                        (*from).type_0
                    };
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Multiple definitions for group {} type of key {}; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS
                                as i32,
                            group.wrapping_add(1 as u32),
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), key_name)),
                            crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*((*info).ctx)).atom_table, use_0)),
                            crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*((*info).ctx)).atom_table, ignore)),
                        );
                    }
                    (*into).type_0 = use_0;
                }
            }
        }
        (*into).defined = ((*into).defined as u32
            | (*from).defined as u32 & GROUP_FIELD_TYPE as u32)
            as group_field;
        if (*from).levels.len() == 0 {
            InitGroupInfo(from);
            return true;
        }
        if (*into).levels.len() == 0 {
            (*from).type_0 = (*into).type_0;
            std::ptr::write(into, std::ptr::read(from));
            InitGroupInfo(from);
            return true;
        }
        let levels_in_both: u32 = if (*into).levels.len() < (*from).levels.len() {
            (*into).levels.len()
        } else {
            (*from).levels.len()
        } as u32;
        let mut fromKeysymsCount: u32 = 0 as u32;
        let mut fromActionsCount: u32 = 0 as u32;
        let mut i: u32 = 0 as u32;
        while i < levels_in_both {
            let intoLevel: *mut xkb_level =
                &mut (&mut (*into).levels)[i as usize] as *mut xkb_level;
            let fromLevel: *mut xkb_level =
                &mut (&mut (*from).levels)[i as usize] as *mut xkb_level;
            let fromHasNoKeysym: bool = (*fromLevel).syms.is_empty();
            let fromHasNoAction: bool = (*fromLevel).actions.is_empty();
            if !(fromHasNoKeysym as i32 != 0 && fromHasNoAction as i32 != 0) {
                let intoHasNoKeysym: bool = (*intoLevel).syms.is_empty();
                let intoHasNoAction: bool = (*intoLevel).actions.is_empty();
                if intoHasNoKeysym as i32 != 0 && intoHasNoAction as i32 != 0 {
                    // StealLevelInfo inlined
                    (*intoLevel).syms = std::mem::take(&mut (*fromLevel).syms);
                    (*intoLevel).actions = std::mem::take(&mut (*fromLevel).actions);
                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                    fromActionsCount = fromActionsCount.wrapping_add(1);
                } else {
                    if !XkbLevelsSameSyms(fromLevel, intoLevel) {
                        if report as i32 != 0
                            && !(intoHasNoKeysym as i32 != 0 || fromHasNoKeysym as i32 != 0)
                        {
                            xkb_logf!(
                                (*info).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Multiple symbols for level {}/group {} on key {}; Using {}, ignoring {}\n",
                                XKB_WARNING_CONFLICTING_KEY_SYMBOL as i32,
                                i.wrapping_add(1 as u32),
                                group.wrapping_add(1 as u32),
                                crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), key_name)),
                                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                                    b"from\0".as_ptr() as *const i8
                                } else {
                                    b"to\0".as_ptr() as *const i8
                                }),
                                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                                    b"to\0".as_ptr() as *const i8
                                } else {
                                    b"from\0".as_ptr() as *const i8
                                }),
                            );
                        }
                        if !fromHasNoKeysym {
                            if clobber {
                                if !(*fromLevel).syms.is_empty() {
                                    (*intoLevel).syms = std::mem::take(&mut (*fromLevel).syms);
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                }
                            } else {
                                if (*intoLevel).syms.is_empty() {
                                    if !(*fromLevel).syms.is_empty() {
                                        (*intoLevel).syms = std::mem::take(&mut (*fromLevel).syms);
                                    }
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                }
                            }
                        }
                    }
                    if !XkbLevelsSameActions(intoLevel, fromLevel) {
                        if report as i32 != 0
                            && !(intoHasNoAction as i32 != 0 || fromHasNoAction as i32 != 0)
                        {
                            if (*intoLevel).actions.len() > 1 || (*fromLevel).actions.len() > 1 {
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] Multiple actions for level {}/group {} on key {}; {}\n",
                                    XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                                    i.wrapping_add(1 as u32),
                                    group.wrapping_add(1 as u32),
                                    crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), key_name)),
                                    crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                                        b"Using from, ignoring to\0".as_ptr()
                                            as *const i8
                                    } else {
                                        b"Using to, ignoring from\0".as_ptr()
                                            as *const i8
                                    }),
                                );
                            } else {
                                let use_action: &xkb_action = if clobber {
                                    &(&(*fromLevel).actions)[0]
                                } else {
                                    &(&(*intoLevel).actions)[0]
                                };
                                let ignore_action: &xkb_action = if clobber {
                                    &(&(*intoLevel).actions)[0]
                                } else {
                                    &(&(*fromLevel).actions)[0]
                                };
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] Multiple actions for level {}/group {} on key {}; Using {}, ignoring {}\n",
                                    XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                                    i.wrapping_add(1 as u32),
                                    group.wrapping_add(1 as u32),
                                    crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), key_name)),
                                    crate::xkb::utils::ByteSliceDisplay(ActionTypeText((*use_action).type_0)),
                                    crate::xkb::utils::ByteSliceDisplay(ActionTypeText((*ignore_action).type_0)),
                                );
                            }
                        }
                        if !fromHasNoAction {
                            if clobber {
                                if !(*fromLevel).actions.is_empty() {
                                    (*intoLevel).actions =
                                        std::mem::take(&mut (*fromLevel).actions);
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                }
                            } else {
                                if (*intoLevel).actions.is_empty() {
                                    if !(*fromLevel).actions.is_empty() {
                                        (*intoLevel).actions =
                                            std::mem::take(&mut (*fromLevel).actions);
                                    }
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let mut level_idx: u32 = levels_in_both;
        while level_idx < (&(*from).levels).len() as u32 {
            let level_val = (&(*from).levels)[level_idx as usize].clone();
            (&mut (*into).levels).push(level_val);
            (&mut (*from).levels)[level_idx as usize].syms.clear();
            (&mut (*from).levels)[level_idx as usize].actions.clear();
            fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
            fromActionsCount = fromActionsCount.wrapping_add(1);
            level_idx = level_idx.wrapping_add(1);
        }
        if fromKeysymsCount != 0 {
            if fromKeysymsCount == (*into).levels.len() as u32 {
                (*into).defined =
                    ((*into).defined as u32 & !(GROUP_FIELD_SYMS as i32) as u32) as group_field;
            }
            (*into).defined = ((*into).defined as u32
                | (*from).defined as u32 & GROUP_FIELD_SYMS as u32)
                as group_field;
        }
        if fromActionsCount != 0 {
            if fromActionsCount == (*into).levels.len() as u32 {
                (*into).defined =
                    ((*into).defined as u32 & !(GROUP_FIELD_ACTS as i32) as u32) as group_field;
            }
            (*into).defined = ((*into).defined as u32
                | (*from).defined as u32 & GROUP_FIELD_ACTS as u32)
                as group_field;
        }
        return true;
    }
}
unsafe fn UseNewKeyField(
    mut field: key_field,
    mut old: key_field,
    mut new: key_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut key_field,
) -> bool {
    unsafe {
        if old as u32 & field as u32 == 0 {
            return new as u32 & field as u32 != 0;
        }
        if new as u32 & field as u32 != 0 {
            if report {
                *collide = (*collide as u32 | field as u32) as key_field;
            }
            return clobber;
        }
        return false;
    }
}
unsafe fn overlays_get(
    mut info: *const KeyInfo,
    mut bit: xkb_overlay_index_t,
    mut key_out: *mut *const xkb_key,
) -> bool {
    unsafe {
        if bit as i32
            >= (std::mem::size_of::<xkb_overlay_mask_t>()).wrapping_mul(8 as usize)
                as xkb_overlay_index_t as i32
        {
            return false;
        }
        let mask: xkb_overlay_mask_t = ((1 as u32) << bit as i32) as xkb_overlay_mask_t;
        if (*info).overlays as i32 & mask as i32 == 0 {
            return false;
        }
        if !key_out.is_null() {
            let low: xkb_overlay_mask_t = ((*info).overlays as u32
                & (mask as u32).wrapping_sub(1 as u32))
                as xkb_overlay_mask_t;
            let index: usize = (low as u32).count_ones() as usize;
            *key_out = (&(*info).overlay_keys)[index];
        }
        return true;
    }
}
unsafe fn overlays_insert(
    mut keyi: *mut KeyInfo,
    mut bit: xkb_overlay_index_t,
    mut key: *const xkb_key,
) -> bool {
    unsafe {
        if bit as i32
            >= (std::mem::size_of::<xkb_overlay_mask_t>()).wrapping_mul(8 as usize)
                as xkb_overlay_index_t as i32
        {
            return false;
        }
        let mask: xkb_overlay_mask_t = ((1 as u32) << bit as i32) as xkb_overlay_mask_t;
        if (*keyi).overlays as i32 & mask as i32 != 0 && !(*keyi).overlays_clear {
            // Bit already set — update existing entry
            let low: xkb_overlay_mask_t = ((*keyi).overlays as u32
                & (mask as u32).wrapping_sub(1 as u32))
                as xkb_overlay_mask_t;
            let index: usize = (low as u32).count_ones() as usize;
            (&mut (*keyi).overlay_keys)[index] = key;
            if key.is_null() && (&(*keyi).overlay_keys).len() == 1 {
                (*keyi).overlays_clear = true;
            }
            return true;
        }
        // New bit
        let new_overlays: xkb_overlay_mask_t =
            ((*keyi).overlays as i32 | mask as i32) as xkb_overlay_mask_t;
        let low: xkb_overlay_mask_t =
            (new_overlays as u32 & (mask as u32).wrapping_sub(1 as u32)) as xkb_overlay_mask_t;
        let index: usize = (low as u32).count_ones() as usize;

        if (*keyi).overlays == 0 || (*keyi).overlays_clear as i32 != 0 && key.is_null() {
            // First overlay or clearing
            (*keyi).overlay_keys.clear();
            (*keyi).overlay_keys.push(key);
            (*keyi).overlays = new_overlays;
            (*keyi).overlays_clear = key.is_null();
        } else {
            // Insert at correct position in Vec
            (*keyi).overlay_keys.insert(index, key);
            (*keyi).overlays = new_overlays;
            (*keyi).overlays_clear = false;
        }
        return true;
    }
}
unsafe fn merge_overlays(
    mut info: *mut SymbolsInfo,
    mut into: *mut KeyInfo,
    mut from: *mut KeyInfo,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut key_field,
) -> bool {
    unsafe {
        if !((*from).defined as i32 & KEY_FIELD_OVERLAY as i32 == 0) {
            if (*into).defined as i32 & KEY_FIELD_OVERLAY as i32 == 0 {
                // into has no overlays, take from's
                (*into).overlays = (*from).overlays;
                (*into).overlay_keys = std::mem::take(&mut (*from).overlay_keys);
                (*into).defined = (*into).defined | KEY_FIELD_OVERLAY as i32 as key_field;
            } else if (*into).overlays_clear as i32 != 0 && (*from).overlays_clear as i32 != 0 {
                (*into).overlays =
                    ((*into).overlays as i32 | (*from).overlays as i32) as xkb_overlay_mask_t;
            } else if (*(*info).keymap_info).features.overlapping_overlays {
                // Complex merge with overlapping overlays
                let result_mask: xkb_overlay_mask_t =
                    ((*into).overlays as i32 | (*from).overlays as i32) as xkb_overlay_mask_t;
                let count: xkb_overlay_index_t =
                    (result_mask as u32).count_ones() as xkb_overlay_index_t;
                if count as i32 == 0 as i32 {
                    eprintln!(
                        "Critical Error: Reached unreachable line in {} at {}",
                        "../src/xkbcomp/symbols.c", 696
                    );
                    abort();
                }
                let mut dest: *mut KeyInfo = into;
                let mut src: *mut KeyInfo = from;
                if (*from).overlay_keys.capacity() > (*into).overlay_keys.capacity() {
                    dest = from;
                    src = into;
                    clobber = !clobber;
                }
                // Iterate over src's overlay bits and merge into dest
                let mut remaining: xkb_overlay_mask_t = (*src).overlays;
                let mut src_idx: usize = 0;
                while remaining != 0 {
                    let lsb: xkb_overlay_mask_t = (remaining as i32
                        & (!(remaining as i32) as u32).wrapping_add(1 as u32) as xkb_overlay_mask_t
                            as i32)
                        as xkb_overlay_mask_t;
                    let bit: xkb_overlay_index_t =
                        ((lsb as u32).wrapping_sub(1 as u32).count_ones()) as xkb_overlay_index_t;
                    remaining = (remaining as i32 & !(lsb as i32)) as xkb_overlay_mask_t;
                    let src_key: *const xkb_key =
                        if (*src).overlays_clear || src_idx >= (*src).overlay_keys.len() {
                            std::ptr::null()
                        } else {
                            let k = (&(*src).overlay_keys)[src_idx];
                            src_idx += 1;
                            k
                        };
                    let mut dest_key: *const xkb_key = std::ptr::null();
                    let conflict: bool = overlays_get(dest, bit, &raw mut dest_key) as bool;
                    if conflict {
                        if dest_key == src_key {
                            continue;
                        }
                        if report {
                            *collide = (*collide as u32 | KEY_FIELD_OVERLAY as u32) as key_field;
                        }
                    }
                    if (!conflict || clobber as i32 != 0) && !overlays_insert(dest, bit, src_key) {
                        return false;
                    }
                }
                if into != dest {
                    (*into).overlays = (*dest).overlays;
                    (*into).overlays_clear = (*dest).overlays_clear as bool;
                    (*into).overlay_keys = std::mem::take(&mut (*dest).overlay_keys);
                }
            } else {
                if (*into).overlays as i32 == (*from).overlays as i32
                    && (*into).overlays_clear as i32 == (*from).overlays_clear as i32
                {
                    // Check if single overlay keys match
                    let into_key = (*into)
                        .overlay_keys
                        .first()
                        .copied()
                        .unwrap_or(std::ptr::null());
                    let from_key = (*from)
                        .overlay_keys
                        .first()
                        .copied()
                        .unwrap_or(std::ptr::null());
                    if into_key == from_key {
                        return true;
                    }
                }
                if (*into).overlays as i32 & (*from).overlays as i32 == 0 {
                    if (*into).overlays_clear {
                        (*into).overlays = (*from).overlays;
                        (*into).overlays_clear = (*from).overlays_clear as bool;
                        (*into).overlay_keys = std::mem::take(&mut (*from).overlay_keys);
                        return true;
                    } else if (*from).overlays_clear {
                        return true;
                    }
                }
                if report {
                    *collide = (*collide as u32 | KEY_FIELD_OVERLAY as u32) as key_field;
                }
                if clobber {
                    (*into).overlays = (*from).overlays;
                    (*into).overlays_clear = (*from).overlays_clear as bool;
                    (*into).overlay_keys = std::mem::take(&mut (*from).overlay_keys);
                }
            }
        }
        return true;
    }
}
unsafe fn MergeKeys(
    mut info: *mut SymbolsInfo,
    mut into: *mut KeyInfo,
    mut from: *mut KeyInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut i: u32 = 0;
        let mut groups_in_both: u32 = 0;
        let mut collide: key_field = 0 as key_field;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let clobber: bool = (*from).merge as i32 != MERGE_AUGMENT as i32;
        let report: bool = same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32;
        if (*from).merge as i32 == MERGE_REPLACE as i32 {
            ClearKeyInfo(into);
            std::ptr::write(into, std::ptr::read(from));
            InitKeyInfo((*info).ctx, from);
            return true;
        }
        groups_in_both = (if (*into).groups.len() < (*from).groups.len() {
            (*into).groups.len()
        } else {
            (*from).groups.len()
        }) as u32;
        i = 0 as u32;
        while i < groups_in_both {
            MergeGroups(
                info,
                &mut (&mut (*into).groups)[i as usize] as *mut GroupInfo,
                &mut (&mut (*from).groups)[i as usize] as *mut GroupInfo,
                clobber,
                report,
                i,
                (*into).name,
            );
            i = i.wrapping_add(1);
        }
        i = groups_in_both;
        while i < (&(*from).groups).len() as u32 {
            let group_val = std::ptr::read(&(&(*from).groups)[i as usize]);
            (&mut (*into).groups).push(group_val);
            InitGroupInfo(&mut (&mut (*from).groups)[i as usize] as *mut GroupInfo);
            i = i.wrapping_add(1);
        }
        if UseNewKeyField(
            KEY_FIELD_VMODMAP,
            (*into).defined,
            (*from).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).vmodmap = (*from).vmodmap;
            (*into).defined = (*into).defined | KEY_FIELD_VMODMAP as i32 as key_field;
        }
        if UseNewKeyField(
            KEY_FIELD_REPEAT,
            (*into).defined,
            (*from).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).repeat = (*from).repeat;
            (*into).defined = (*into).defined | KEY_FIELD_REPEAT as i32 as key_field;
        }
        if UseNewKeyField(
            KEY_FIELD_DEFAULT_TYPE,
            (*into).defined,
            (*from).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).default_type = (*from).default_type;
            (*into).defined = (*into).defined | KEY_FIELD_DEFAULT_TYPE as i32 as key_field;
        }
        if UseNewKeyField(
            KEY_FIELD_GROUPINFO,
            (*into).defined,
            (*from).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).out_of_range_pending_group = (*from).out_of_range_pending_group as bool;
            (*into).out_of_range_group_policy = (*from).out_of_range_group_policy;
            (*into).out_of_range_group_number = (*from).out_of_range_group_number;
            (*into).defined = (*into).defined | KEY_FIELD_GROUPINFO as i32 as key_field;
        }
        if !merge_overlays(info, into, from, clobber, report, &raw mut collide) {
            return false;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Symbol map for key {} redefined; Using {} definition for conflicting fields\n",
                XKB_WARNING_CONFLICTING_KEY_FIELDS as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), (*into).name)),
                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                    b"first\0".as_ptr() as *const i8
                } else {
                    b"last\0".as_ptr() as *const i8
                }),
            );
        }
        ClearKeyInfo(from);
        InitKeyInfo((*info).ctx, from);
        return true;
    }
}
unsafe fn AddKeySymbols(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        // XkbResolveKeyAlias inlined
        {
            let keymap = (*(*info).keymap_info).keymap;
            let name = (*keyi).name;
            if (name as usize) < (&(*keymap).key_names).len() {
                let match_0: KeycodeMatch = (&(*keymap).key_names)[name as usize];
                if match_0.found as i32 != 0 && match_0.is_alias as i32 != 0 {
                    (*keyi).name = match_0.index;
                }
            }
        }
        for iter in (*info).keys.iter_mut() {
            if iter.name == (*keyi).name {
                return MergeKeys(info, iter as *mut KeyInfo, keyi, same_file);
            }
        }
        (*info).keys.push(std::ptr::read(keyi));
        InitKeyInfo((*info).ctx, keyi);
        return true;
    }
}
unsafe fn AddModMapEntry(mut info: *mut SymbolsInfo, mut new: *mut ModMapEntry) -> bool {
    unsafe {
        let mut clobber: bool = (*new).merge as u32 != MERGE_AUGMENT as u32;
        for old in (*info).modmaps.iter_mut() {
            if (*new).haveSymbol as i32 != old.haveSymbol as i32
                || (*new).haveSymbol as i32 != 0 && (*new).u.keySym != old.u.keySym
                || !(*new).haveSymbol && (*new).u.keyName != old.u.keyName
            {
                continue;
            }
            if (*new).modifier == old.modifier {
                return true;
            }
            let use_0: u32 = if clobber as i32 != 0 {
                (*new).modifier
            } else {
                old.modifier
            };
            let ignore: u32 = if clobber as i32 != 0 {
                old.modifier
            } else {
                (*new).modifier
            };
            if (*new).haveSymbol {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Symbol \"{}\" added to modifier map for multiple modifiers; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_MODMAP as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeysymText((*((*info).ctx)).clone(), (*new).u.keySym)),
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, use_0)),
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, ignore)),
                );
            } else {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Key \"{}\" added to modifier map for multiple modifiers; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_MODMAP as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), (*new).u.keyName)),
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, use_0)),
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, ignore)),
                );
            }
            old.modifier = use_0;
            return true;
        }
        (*info).modmaps.push(*new);
        return true;
    }
}
unsafe fn MergeIncludedSymbols(
    mut into: *mut SymbolsInfo,
    mut from: *mut SymbolsInfo,
    mut merge: merge_mode,
) {
    unsafe {
        let mut group_names_in_both: u32 = 0;
        if (*from).errorCount > 0 as i32 {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        MergeModSets(
            (*into).ctx,
            &raw mut (*into).mods,
            &raw mut (*from).mods,
            merge,
        );
        if (*into).name.is_none() {
            (*into).name = (*from).name.take();
        }
        group_names_in_both = (if (&(*into).group_names).len() < (&(*from).group_names).len() {
            (&(*into).group_names).len()
        } else {
            (&(*from).group_names).len()
        }) as u32;
        let mut i: u32 = 0 as u32;
        while i < group_names_in_both {
            if !((&(*from).group_names)[i as usize] == 0) {
                if !(merge as u32 == MERGE_AUGMENT as u32
                    && (&(*into).group_names)[i as usize] != 0)
                {
                    (&mut (*into).group_names)[i as usize] = (&(*from).group_names)[i as usize];
                }
            }
            i = i.wrapping_add(1);
        }
        if group_names_in_both < (&(*from).group_names).len() as u32 {
            for gn_idx in group_names_in_both as usize..(&(*from).group_names).len() {
                (&mut (*into).group_names).push((&(*from).group_names)[gn_idx]);
            }
        }
        if (*into).keys.is_empty() {
            std::mem::swap(&mut (*into).keys, &mut (*from).keys);
        } else {
            for keyi in (*from).keys.iter_mut() {
                keyi.merge = merge as merge_mode;
                if !AddKeySymbols(into, keyi as *mut KeyInfo, false) {
                    (*into).errorCount += 1;
                }
            }
        }
        if (*into).modmaps.is_empty() {
            std::mem::swap(&mut (*into).modmaps, &mut (*from).modmaps);
        } else {
            for mm in (*from).modmaps.iter_mut() {
                mm.merge = merge;
                if !AddModMapEntry(into, mm as *mut ModMapEntry) {
                    (*into).errorCount += 1;
                }
            }
        };
    }
}
unsafe fn HandleIncludeSymbols(mut info: *mut SymbolsInfo, mut include: *mut IncludeStmt) -> bool {
    unsafe {
        let mut included: SymbolsInfo = SymbolsInfo::new_zeroed();
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as i32;
            return false;
        }
        InitSymbolsInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as u32),
            &raw mut (*info).mods,
        );
        included.name = if (*include).stmt.is_null() {
            None
        } else {
            let s = Some(String::from(
                std::ffi::CStr::from_ptr((*include).stmt)
                    .to_str()
                    .unwrap_or(""),
            ));
            cstr_free((*include).stmt);
            (*include).stmt = std::ptr::null_mut();
            s
        };
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: SymbolsInfo = SymbolsInfo::new_zeroed();
            let mut file: *mut XkbFile = std::ptr::null_mut();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_SYMBOLS,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearSymbolsInfo(&raw mut included);
                return false;
            }
            InitSymbolsInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
                &raw mut included.mods,
            );
            if !(*stmt).modifier.is_null() {
                next_incl.explicit_group =
                    (crate::xkb::utils::cstr_atoi((*stmt).modifier) - 1 as i32) as u32;
                if next_incl.explicit_group >= (*info).max_groups {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Cannot set explicit group to {} - must be between 1..{}; Ignoring group number\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                        next_incl.explicit_group.wrapping_add(1 as u32),
                        (*info).max_groups,
                    );
                    next_incl.explicit_group = (*info).explicit_group;
                }
            } else if (*(*(*info).keymap_info).keymap).num_groups != 0 as u32
                && next_incl.include_depth == 1 as u32
            {
                next_incl.explicit_group = 0 as u32;
            } else {
                next_incl.explicit_group = (*info).explicit_group;
            }
            HandleSymbolsFile(&raw mut next_incl, file);
            MergeIncludedSymbols(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearSymbolsInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedSymbols(info, &raw mut included, (*include).merge);
        ClearSymbolsInfo(&raw mut included);
        return (*info).errorCount == 0 as i32;
    }
}
unsafe fn GetGroupIndex(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut field: group_field,
    mut ndx_rtrn: *mut u32,
) -> bool {
    unsafe {
        let mut name: *const i8 = if field as u32 == GROUP_FIELD_SYMS as u32 {
            b"symbols\0".as_ptr() as *const i8
        } else {
            b"actions\0".as_ptr() as *const i8
        };
        if arrayNdx.is_null() {
            let mut i: u32 = 0 as u32;
            let mut groupi: *mut GroupInfo = std::ptr::null_mut();
            if !(*keyi).groups.is_empty() {
                i = 0 as u32;
                groupi = (*keyi).groups.as_mut_ptr();
                while i < (*keyi).groups.len() as u32 {
                    if (*groupi).defined as u32 & field as u32 == 0 {
                        *ndx_rtrn = i;
                        return true;
                    }
                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }
            if i >= (*info).max_groups {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Too many groups of {} for key {} (max {}); Ignoring {} defined for extra groups\n",
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                    crate::xkb::utils::CStrDisplay(name),
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                    (*info).max_groups,
                    crate::xkb::utils::CStrDisplay(name),
                );
                return false;
            }
            resize_groups_zero(&mut (*keyi).groups, (*keyi).groups.len().wrapping_add(1));
            *ndx_rtrn = (*keyi).groups.len().wrapping_sub(1) as u32;
            return true;
        }
        if ExprResolveGroup(
            (*info).keymap_info,
            arrayNdx,
            false,
            ndx_rtrn,
            std::ptr::null_mut(),
        ) as u32
            != PARSER_SUCCESS as u32
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Illegal group index for {} of key {}\nDefinition with non-integer array index ignored\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                crate::xkb::utils::CStrDisplay(name),
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }
        *ndx_rtrn = (*ndx_rtrn).wrapping_sub(1);
        if *ndx_rtrn >= (*keyi).groups.len() as u32 {
            resize_groups_zero(&mut (*keyi).groups, (*ndx_rtrn).wrapping_add(1) as usize);
        }
        return true;
    }
}
unsafe fn AddSymbolsToKey(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ndx: u32 = 0 as u32;
        if !GetGroupIndex(info, keyi, arrayNdx, GROUP_FIELD_SYMS, &raw mut ndx) {
            return false;
        }
        let mut groupi: *mut GroupInfo = &mut (&mut (*keyi).groups)[ndx as usize] as *mut GroupInfo;
        if (*value).common.type_0 as u32 == STMT_EXPR_EMPTY_LIST as u32 {
            (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_SYMS as u32) as group_field;
            return true;
        }
        if (*value).common.type_0 as u32 != STMT_EXPR_KEYSYM_LIST as u32 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Expected a list of symbols, found {}; Ignoring symbols for group {} of {}\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string((*value).common.type_0),
                ndx.wrapping_add(1 as u32),
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }
        if (*groupi).defined as u32 & GROUP_FIELD_SYMS as u32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Symbols for key {}, group {} already defined; Ignoring duplicate definition\n",
                XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                ndx.wrapping_add(1 as u32),
            );
            return false;
        }
        let mut nLevels: u32 = 0 as u32;
        let mut nonEmptyLevels: u32 = 0 as u32;
        let mut keysymList: *mut ExprKeysymList = value as *mut ExprKeysymList;
        while !keysymList.is_null() {
            nLevels = nLevels.wrapping_add(1);
            if (&(*keysymList).syms).len() as u32 > 0 as u32 {
                nonEmptyLevels = nLevels;
            }
            keysymList = (*keysymList).common.next as *mut ExprKeysymList;
        }
        if nonEmptyLevels < nLevels {
            nLevels = nonEmptyLevels;
        }
        if ((*groupi).levels.len() as u32) < nLevels {
            (*groupi)
                .levels
                .resize_with(nLevels as usize, Default::default);
        }
        (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_SYMS as u32) as group_field;
        let mut level: u32 = 0 as u32;
        let mut keysymList_0: *mut ExprKeysymList = value as *mut ExprKeysymList;
        while !keysymList_0.is_null() && level < nLevels {
            let mut leveli: *mut xkb_level =
                &mut (&mut (*groupi).levels)[level as usize] as *mut xkb_level;
            let syms_len = (&(*keysymList_0).syms).len() as u32;
            if (syms_len > 65535 as u32) as i64 != 0 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key {} has too many keysyms for group {}, level {}; expected max {}, got: {}\n",
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                    ndx.wrapping_add(1 as u32),
                    level.wrapping_add(1 as u32),
                    65535 as i32,
                    syms_len,
                );
                return false;
            }
            (*leveli).syms = if syms_len == 0 {
                Vec::new()
            } else {
                (&(*keysymList_0).syms)[..syms_len as usize].to_vec()
            };
            keysymList_0 = (*keysymList_0).common.next as *mut ExprKeysymList;
            level = level.wrapping_add(1);
        }
        return true;
    }
}
unsafe fn AddActionsToKey(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ndx: u32 = 0 as u32;
        if !GetGroupIndex(info, keyi, arrayNdx, GROUP_FIELD_ACTS, &raw mut ndx) {
            return false;
        }
        let mut groupi: *mut GroupInfo = &mut (&mut (*keyi).groups)[ndx as usize] as *mut GroupInfo;
        if (*value).common.type_0 as u32 == STMT_EXPR_EMPTY_LIST as u32 {
            (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_ACTS as u32) as group_field;
            return true;
        }
        if (*value).common.type_0 as u32 != STMT_EXPR_ACTION_LIST as u32 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_CRITICAL,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Bad expression type ({}) for action list value; Ignoring actions for group {} of {}\n",
                XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                (*value).common.type_0 as u32,
                ndx,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }
        if (*groupi).defined as u32 & GROUP_FIELD_ACTS as u32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_CRITICAL,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Actions for key {}, group {} already defined\n",
                XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                ndx,
            );
            return false;
        }
        let mut nLevels: u32 = 0 as u32;
        let mut p: *mut ParseCommon = &raw mut (*value).common;
        while !p.is_null() {
            nLevels = nLevels.wrapping_add(1);
            p = (*p).next as *mut ParseCommon;
        }
        if ((*groupi).levels.len() as u32) < nLevels {
            (*groupi)
                .levels
                .resize_with(nLevels as usize, Default::default);
        }
        (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_ACTS as u32) as group_field;
        let mut level: u32 = 0 as u32;
        let mut nonEmptyLevels: u32 = 0 as u32;
        let mut actionList: *mut ExprActionList = value as *mut ExprActionList;
        while !actionList.is_null() {
            let mut c2rust_current_block_102: u64;
            let mut leveli: *mut xkb_level =
                &mut (&mut (*groupi).levels)[level as usize] as *mut xkb_level;
            let mut num_actions: u32 = 0 as u32;
            let mut act: *mut ExprDef = (*actionList).actions as *mut ExprDef;
            while !act.is_null() {
                num_actions = num_actions.wrapping_add(1);
                act = (*act).common.next as *mut ExprDef;
            }
            if (num_actions > 65535 as u32) as i64 != 0 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key {} has too many actions for group {}, level {}; expected max {}, got: {}\n",
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                    ndx.wrapping_add(1 as u32),
                    level.wrapping_add(1 as u32),
                    65535 as i32,
                    num_actions,
                );
                return false;
            }
            let mut actions: Vec<xkb_action> = Vec::new();
            let mut act_0: *mut ExprDef = (*actionList).actions as *mut ExprDef;
            loop {
                if act_0.is_null() {
                    c2rust_current_block_102 = 1134115459065347084;
                    break;
                }
                let mut toAct: xkb_action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                let r: xkb_parser_error = HandleActionDef(
                    (*info).keymap_info,
                    &raw mut (*info).default_actions,
                    &raw mut (*info).mods,
                    act_0,
                    &raw mut toAct,
                ) as xkb_parser_error;
                if r as u32 != PARSER_SUCCESS as u32 {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Illegal action definition for {}; Action for group {}/level {} ignored\n",
                        XKB_ERROR_INVALID_VALUE as i32,
                        crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                        ndx.wrapping_add(1 as u32),
                        level.wrapping_add(1 as u32),
                    );
                    if r as u32 == PARSER_FATAL_ERROR as u32 {
                        drop(actions);
                        return false;
                    } else {
                        toAct.type_0 = ACTION_TYPE_NONE;
                    }
                }
                if !(toAct.type_0 as u32 == ACTION_TYPE_NONE as u32) {
                    if (num_actions == 1 as u32) as i64 != 0 {
                        (*leveli).actions = vec![toAct];
                        c2rust_current_block_102 = 1829140360157350833;
                        break;
                    } else {
                        actions.push(toAct);
                    }
                }
                act_0 = (*act_0).common.next as *mut ExprDef;
            }
            match c2rust_current_block_102 {
                1134115459065347084 => {
                    if actions.is_empty() {
                        (*leveli).actions = Vec::new();
                    } else {
                        (*leveli).actions = std::mem::take(&mut actions);
                    }
                }
                _ => {}
            }
            if !(*leveli).actions.is_empty() || !(*leveli).syms.is_empty() {
                nonEmptyLevels = level.wrapping_add(1 as u32);
            }
            actionList = (*actionList).common.next as *mut ExprActionList;
            level = level.wrapping_add(1);
        }
        if nonEmptyLevels < nLevels {
            if nonEmptyLevels > 0 as u32 {
                (*groupi).levels.truncate(nonEmptyLevels as usize);
            } else {
                (*groupi).levels.clear();
            }
        }
        return true;
    }
}
static mut repeatEntries: [LookupEntry; 8] = [
    LookupEntry {
        name: b"true",
        value: KEY_REPEAT_YES as u32,
    },
    LookupEntry {
        name: b"yes",
        value: KEY_REPEAT_YES as u32,
    },
    LookupEntry {
        name: b"on",
        value: KEY_REPEAT_YES as u32,
    },
    LookupEntry {
        name: b"false",
        value: KEY_REPEAT_NO as u32,
    },
    LookupEntry {
        name: b"no",
        value: KEY_REPEAT_NO as u32,
    },
    LookupEntry {
        name: b"off",
        value: KEY_REPEAT_NO as u32,
    },
    LookupEntry {
        name: b"default",
        value: KEY_REPEAT_UNDEFINED as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
unsafe fn ExprResolveOverlayEntry(
    mut keymap_info: *const xkb_keymap_info,
    field: &[u8],
    mut arrayNdx: *const ExprDef,
    mut expr: *const ExprDef,
    mut keyi: *mut KeyInfo,
    mut overlay_rtrn: *mut xkb_overlay_index_t,
    mut key_rtrn: *mut *const xkb_key,
) -> bool {
    unsafe {
        if !arrayNdx.is_null() {
            xkb_logf!(
                (*(*keymap_info).keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Overlay field \"{}\" in {} does not support array index; ignored\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::ByteSliceDisplay(field),
                crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                    (*(*keymap_info).keymap).ctx.clone(),
                    (*keyi).name
                )),
            );
            return false;
        }
        let prefix: usize = (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize);
        let suffix = &field[prefix..];
        let len: usize = suffix.len();
        let mut raw_overlay: i64 = XKB_OVERLAY_INVALID as i64;
        let (val_parsed, parse_count) = crate::xkb::utils::parse_dec_u64(suffix);
        raw_overlay = val_parsed as i64;
        if parse_count != len as i32
            || raw_overlay < 1 as i64
            || raw_overlay > (*keymap_info).features.max_overlays as i64
        {
            xkb_logf!(
                (*(*keymap_info).keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unsupported overlay index \"{}\" field for {}: expected 1..{}, got: {}; ignored\n",
                XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX as i32,
                crate::xkb::utils::ByteSliceDisplay(field),
                crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*keymap_info).keymap).ctx.clone(), (*keyi).name)),
                (*keymap_info).features.max_overlays as i32,
                raw_overlay,
            );
            return false;
        }
        *overlay_rtrn =
            (raw_overlay as xkb_overlay_index_t as i32 - 1 as i32) as xkb_overlay_index_t;
        match (*expr).common.type_0 as u32 {
            8 => {
                *key_rtrn = XkbKeyByName((*keymap_info).keymap, (*expr).key_name.key_name, false);
                if (*key_rtrn).is_null() {
                    xkb_logf!(
                        (*(*keymap_info).keymap).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unknown key \"{}\" for field {} in {}\n",
                        XKB_WARNING_UNDEFINED_KEYCODE as i32,
                        crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(
                            &(*(*keymap_info).keymap).ctx.atom_table,
                            (*expr).key_name.key_name
                        )),
                        crate::xkb::utils::ByteSliceDisplay(field),
                        crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                            (*(*keymap_info).keymap).ctx.clone(),
                            (*keyi).name
                        )),
                    );
                    return false;
                }
                return true;
            }
            10 => {
                let id: &[u8] = xkb_atom_text_bytes(
                    &(*(*keymap_info).keymap).ctx.atom_table,
                    (*expr).ident.ident,
                );
                if !id.is_empty() && istreq(id, b"none") {
                    *key_rtrn = std::ptr::null();
                    return true;
                } else if !id.is_empty() && istreq(id, b"any") {
                    *key_rtrn = std::ptr::null();
                    *overlay_rtrn = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
                    return true;
                }
                xkb_logf!(
                    (*(*keymap_info).keymap).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unsupported overlay value \"{}\" for field {} in {}\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::ByteSliceDisplay(id),
                    crate::xkb::utils::ByteSliceDisplay(field),
                    crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                        (*(*keymap_info).keymap).ctx.clone(),
                        (*keyi).name
                    )),
                );
                return false;
            }
            _ => {
                xkb_logf!(
                    (*(*keymap_info).keymap).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Expected {} for field \"{}\" in {}, got: {}\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    stmt_type_to_string(STMT_EXPR_KEYNAME_LITERAL),
                    crate::xkb::utils::ByteSliceDisplay(field),
                    crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                        (*(*keymap_info).keymap).ctx.clone(),
                        (*keyi).name
                    )),
                    stmt_type_to_string((*expr).common.type_0),
                );
                return false;
            }
        };
    }
}
unsafe fn SetSymbolsField(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    field: &[u8],
    mut arrayNdx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if istreq(field, b"type") {
            let mut ndx: u32 = 0 as u32;
            let mut val: u32 = XKB_ATOM_NONE as u32;
            if !ExprResolveString((*info).ctx, value, &raw mut val) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The type field of a key symbol map must be a string; Ignoring illegal type definition\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                );
                return false;
            }
            if arrayNdx.is_null() {
                (*keyi).default_type = val;
                (*keyi).defined = (*keyi).defined | KEY_FIELD_DEFAULT_TYPE as i32 as key_field;
            } else if ExprResolveGroup(
                (*info).keymap_info,
                arrayNdx,
                false,
                &raw mut ndx,
                std::ptr::null_mut(),
            ) as u32
                != PARSER_SUCCESS as u32
            {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal group index for type of key {}; Definition with non-integer array index ignored\n",
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            } else {
                ndx = ndx.wrapping_sub(1);
                if ndx >= (*keyi).groups.len() as u32 {
                    resize_groups_zero(&mut (*keyi).groups, (ndx as usize).wrapping_add(1));
                }
                (&mut (*keyi).groups)[ndx as usize].type_0 = val;
                let ref mut c2rust_fresh8 = (&mut (*keyi).groups)[ndx as usize].defined;
                *c2rust_fresh8 = (*c2rust_fresh8 as u32 | GROUP_FIELD_TYPE as u32) as group_field;
            }
        } else if istreq(field, b"symbols") {
            return AddSymbolsToKey(info, keyi, arrayNdx, value);
        } else if istreq(field, b"actions") {
            return AddActionsToKey(info, keyi, arrayNdx, value);
        } else if istreq(field, b"vmods")
            || istreq(field, b"virtualmods")
            || istreq(field, b"virtualmodifiers")
        {
            let mut mask: u32 = 0 as u32;
            if !ExprResolveModMask(
                (*info).ctx,
                value,
                MOD_VIRT,
                &raw mut (*info).mods,
                &raw mut mask,
            ) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Expected a virtual modifier mask, found {}; Ignoring virtual modifiers definition for key {}\n",
                    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as i32,
                    stmt_type_to_string((*value).common.type_0),
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).vmodmap = mask;
            (*keyi).defined = (*keyi).defined | KEY_FIELD_VMODMAP as i32 as key_field;
        } else if istreq(field, b"locking") || istreq(field, b"lock") || istreq(field, b"locks") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Key behaviors not supported; Ignoring locking specification for key {}\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
        } else if istreq(field, b"radiogroup")
            || istreq(field, b"permanentradiogroup")
            || istreq(field, b"allownone")
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Radio groups not supported; Ignoring radio group specification for key {}\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
        } else if istrneq(
            b"permanentoverlay",
            field,
            (std::mem::size_of::<[i8; 17]>()).wrapping_sub(1 as usize),
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Permanent overlays not supported; Ignoring overlay specification for key {}\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
        } else if istrneq(
            b"overlay",
            field,
            (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize),
        ) {
            let mut overlay: xkb_overlay_index_t = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
            let mut key: *const xkb_key = std::ptr::null();
            if !ExprResolveOverlayEntry(
                (*info).keymap_info,
                field,
                arrayNdx,
                *value_ptr,
                keyi,
                &raw mut overlay,
                &raw mut key,
            ) {
                return false;
            }
            if overlay as i32 == XKB_OVERLAY_INVALID {
                return true;
            } else if !key.is_null() && (*key).name == (*keyi).name {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as i32,
                    "Cannot overlay a key to itself; Ignoring overlay {} specification for key {}\n",
                    overlay as i32 + 1 as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
            } else {
                let mut prev: *const xkb_key = std::ptr::null();
                if overlays_get(keyi, overlay, &raw mut prev) {
                    if key != prev {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Conflicting overlays defined in key {}; use overlay{}={}, ignore overlay{}={}\n",
                            XKB_WARNING_CONFLICTING_KEY_FIELDS as i32,
                            crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                            overlay as i32 + 1 as i32,
                            crate::xkb::utils::ByteSliceDisplay(if !prev.is_null() {
                                KeyNameText((*((*info).ctx)).clone(), (*prev).name)
                            } else {
                                b"none"
                            }),
                            overlay as i32 + 1 as i32,
                            crate::xkb::utils::ByteSliceDisplay(if !key.is_null() {
                                KeyNameText((*((*info).ctx)).clone(), (*key).name)
                            } else {
                                b"none"
                            }),
                        );
                    }
                } else if (*(*info).keymap_info).features.overlapping_overlays {
                    if !overlays_insert(keyi, overlay, key) {
                        return false;
                    }
                    (*keyi).defined = (*keyi).defined | KEY_FIELD_OVERLAY as i32 as key_field;
                } else {
                    let mask_0: xkb_overlay_mask_t =
                        ((1 as u32) << overlay as i32) as xkb_overlay_mask_t;
                    if (*keyi).overlays == 0 || (*keyi).overlays_clear as i32 != 0 {
                        if !key.is_null() {
                            (*keyi).overlays = mask_0;
                            (*keyi).overlays_clear = (false) as bool;
                            (*keyi).overlay_keys = vec![key];
                        } else {
                            (*keyi).overlays =
                                ((*keyi).overlays as i32 | mask_0 as i32) as xkb_overlay_mask_t;
                            (*keyi).overlays_clear = (true) as bool;
                            (*keyi).overlay_keys = vec![std::ptr::null()];
                        }
                        (*keyi).defined = (*keyi).defined | KEY_FIELD_OVERLAY as i32 as key_field;
                    } else if (*keyi).overlays != 0 {
                        if !key.is_null() {
                            let existing_key = (*keyi)
                                .overlay_keys
                                .first()
                                .copied()
                                .unwrap_or(std::ptr::null());
                            xkb_logf!(
                                (*info).ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Overlapping overlays are not allowed in {}; use overlay{}={}, ignore overlay{}={}\n",
                                XKB_ERROR_OVERLAPPING_OVERLAY as i32,
                                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                                (*keyi).overlays as i32,
                                crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                                    (*(*info).ctx).clone(),
                                    if existing_key.is_null() { 0 } else { (*existing_key).name },
                                )),
                                overlay as i32 + 1 as i32,
                                crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), (*key).name)),
                            );
                            return (*(*info).keymap_info).strict as u32
                                & PARSER_NO_FIELD_VALUE_MISMATCH as u32
                                == 0;
                        }
                    }
                }
            }
        } else if istreq(field, b"repeating")
            || istreq(field, b"repeats")
            || istreq(field, b"repeat")
        {
            let mut val_0: u32 = 0 as u32;
            if !ExprResolveEnum(
                (*info).ctx,
                value,
                &raw mut val_0,
                &raw const repeatEntries as *const LookupEntry,
            ) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal repeat setting for {}; Non-boolean repeat setting ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).repeat = val_0 as key_repeat as key_repeat;
            (*keyi).defined = (*keyi).defined | KEY_FIELD_REPEAT as i32 as key_field;
        } else if istreq(field, b"groupswrap") || istreq(field, b"wrapgroups") {
            let mut set: bool = false;
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal groupsWrap setting for {}; Non-boolean value ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).out_of_range_group_policy = if set {
                XKB_LAYOUT_OUT_OF_RANGE_WRAP
            } else {
                XKB_LAYOUT_OUT_OF_RANGE_CLAMP
            };
            (*keyi).defined = (*keyi).defined | KEY_FIELD_GROUPINFO as i32 as key_field;
        } else if istreq(field, b"groupsclamp") || istreq(field, b"clampgroups") {
            let mut set_0: bool = false;
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set_0) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal groupsClamp setting for {}; Non-boolean value ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).out_of_range_group_policy = if set_0 {
                XKB_LAYOUT_OUT_OF_RANGE_CLAMP
            } else {
                XKB_LAYOUT_OUT_OF_RANGE_WRAP
            };
            (*keyi).defined = (*keyi).defined | KEY_FIELD_GROUPINFO as i32 as key_field;
        } else if istreq(field, b"groupsredirect") || istreq(field, b"redirectgroups") {
            let mut grp: u32 = 0 as u32;
            let mut pending: bool = false;
            if ExprResolveGroup(
                (*info).keymap_info,
                value,
                false,
                &raw mut grp,
                &raw mut pending,
            ) as u32
                != PARSER_SUCCESS as u32
                && !pending
            {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal group index for redirect of key {}; Definition with non-integer group ignored\n",
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            if pending {
                (*keyi).out_of_range_pending_group = (true) as bool;
                let pending_index: u32 =
                    (&*(*(*info).keymap_info).pending_computations).len() as u32;
                (&mut *(*(*info).keymap_info).pending_computations).push(pending_computation {
                    expr: *value_ptr,
                    computed: false,
                    value: 0 as u32,
                });
                *value_ptr = std::ptr::null_mut();
                (*keyi).out_of_range_group_number = pending_index as u32;
            } else {
                (*keyi).out_of_range_pending_group = (false) as bool;
                (*keyi).out_of_range_group_number = grp.wrapping_sub(1 as u32);
            }
            (*keyi).out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_REDIRECT;
            (*keyi).defined = (*keyi).defined | KEY_FIELD_GROUPINFO as i32 as key_field;
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown field \"{}\" in a key; definition ignored\n",
                XKB_ERROR_UNKNOWN_FIELD as i32,
                crate::xkb::utils::ByteSliceDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_KEY_FIELDS as u32 == 0;
        }
        return true;
    }
}
unsafe fn SetGroupName(
    mut info: *mut SymbolsInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
    mut merge: merge_mode,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] You must specify an index when specifying a group name; Group name definition without array subscript ignored\n",
                XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX as i32,
            );
            return false;
        }
        let mut group: u32 = 0 as u32;
        if ExprResolveGroup(
            (*info).keymap_info,
            arrayNdx,
            false,
            &raw mut group,
            std::ptr::null_mut(),
        ) as u32
            != PARSER_SUCCESS as u32
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Illegal index in group name definition; Definition with non-integer array index ignored\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
            );
            return false;
        }
        let mut name: u32 = XKB_ATOM_NONE as u32;
        if !ExprResolveString((*info).ctx, value, &raw mut name) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Group name must be a string; Illegal name for group {} ignored\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                group,
            );
            return false;
        }
        let mut group_to_use: u32 = 0;
        if (*info).explicit_group == XKB_LAYOUT_INVALID as u32 {
            group_to_use = group.wrapping_sub(1 as u32);
        } else if group.wrapping_sub(1 as u32) == 0 as u32 {
            group_to_use = (*info).explicit_group;
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] An explicit group was specified for the '{}' map, but it provides a name for a group other than Group1 ({}); Ignoring group name '{}'\n",
                XKB_WARNING_NON_BASE_GROUP_NAME as i32,
                (*info).name.as_deref().unwrap_or(""),
                group,
                crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*((*info).ctx)).atom_table, name)),
            );
            return false;
        }
        if group_to_use >= (*info).group_names.len() as u32 {
            (*info)
                .group_names
                .resize((group_to_use as usize).wrapping_add(1), 0 as u32);
        } else {
            let old_name: u32 = (&(*info).group_names)[group_to_use as usize];
            if old_name != XKB_ATOM_NONE as u32 && old_name != name {
                let replace: bool = merge as u32 != MERGE_AUGMENT as u32;
                let use_0: u32 = if replace as i32 != 0 { name } else { old_name };
                let ignore: u32 = if replace as i32 != 0 { old_name } else { name };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Multiple definitions of group {} name in map '{}'; Using '{}', ignoring '{}'\n",
                group_to_use,
                (*info).name.as_deref().unwrap_or(""),
                    crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*((*info).ctx)).atom_table, use_0)),
                    crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*((*info).ctx)).atom_table, ignore)),
                );
                name = use_0;
            }
        }
        (&mut (*info).group_names)[group_to_use as usize] = name;
        return true;
    }
}
unsafe fn HandleGlobalVar(mut info: *mut SymbolsInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: &[u8] = b"";
        let mut field: &[u8] = b"";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        let mut ret: bool = false;
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &mut elem,
            &mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        }
        if !elem.is_empty() && istreq(elem, b"key") {
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
            InitKeyInfo((*info).ctx, &raw mut temp);
            temp.merge = if temp.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                (*stmt).merge as merge_mode
            };
            ret = SetSymbolsField(info, &raw mut temp, field, arrayNdx, &raw mut (*stmt).value);
            MergeKeys(info, &raw mut (*info).default_key, &raw mut temp, true);
        } else if elem.is_empty() && (istreq(field, b"name") || istreq(field, b"groupname")) {
            ret = SetGroupName(info, arrayNdx, (*stmt).value as *mut ExprDef, (*stmt).merge);
        } else if elem.is_empty() && (istreq(field, b"groupswrap") || istreq(field, b"wrapgroups"))
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Global \"groupswrap\" not supported; Ignored\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if elem.is_empty()
            && (istreq(field, b"groupsclamp") || istreq(field, b"clampgroups"))
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Global \"groupsclamp\" not supported; Ignored\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if elem.is_empty()
            && (istreq(field, b"groupsredirect") || istreq(field, b"redirectgroups"))
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Global \"groupsredirect\" not supported; Ignored\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if elem.is_empty() && istreq(field, b"allownone") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Radio groups not supported; Ignoring \"allownone\" specification\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if !elem.is_empty() {
            ret = SetDefaultActionField(
                (*info).keymap_info,
                &raw mut (*info).default_actions,
                &raw mut (*info).mods,
                elem,
                field,
                arrayNdx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as u32
                != PARSER_FATAL_ERROR as u32;
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::ByteSliceDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS as u32
                == 0;
        }
        return ret;
    }
}
unsafe fn HandleSymbolsBody(
    mut info: *mut SymbolsInfo,
    mut def: *mut VarDef,
    mut keyi: *mut KeyInfo,
) -> bool {
    unsafe {
        let mut all_valid_entries: bool = true;
        while !def.is_null() {
            let mut field: &[u8] = b"";
            let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
            let mut ok: bool = true;
            if (*def).name.is_null() {
                if (*def).value.is_null() as i64 != 0
                    || (*(*def).value).common.type_0 as u32 != STMT_EXPR_ACTION_LIST as u32
                {
                    field = b"symbols";
                } else {
                    field = b"actions";
                }
                arrayNdx = std::ptr::null_mut();
            } else {
                let mut elem: &[u8] = b"";
                ok = ExprResolveLhs(
                    (*info).ctx,
                    (*def).name,
                    &mut elem,
                    &mut field,
                    &raw mut arrayNdx,
                );
                if ok as i32 != 0 && !elem.is_empty() {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                        crate::xkb::utils::ByteSliceDisplay(elem),
                        crate::xkb::utils::ByteSliceDisplay(elem),
                        crate::xkb::utils::ByteSliceDisplay(field),
                    );
                    ok = false;
                }
            }
            if (*def).value.is_null() as i64 != 0 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Could not allocate the value of field \"{}\". Statement ignored.\n",
                    XKB_ERROR_ALLOCATION_ERROR as i32,
                    crate::xkb::utils::ByteSliceDisplay(field),
                );
                ok = false;
            }
            if !ok || !SetSymbolsField(info, keyi, field, arrayNdx, &raw mut (*def).value) {
                all_valid_entries = false;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return all_valid_entries;
    }
}
unsafe fn SetExplicitGroup(mut info: *mut SymbolsInfo, mut keyi: *mut KeyInfo) -> bool {
    unsafe {
        let mut i: u32 = 0;
        let mut groupi: *mut GroupInfo = std::ptr::null_mut();
        let mut warn: bool = false;
        if (*info).explicit_group == XKB_LAYOUT_INVALID as u32 {
            return true;
        }
        if !(&(*keyi).groups).is_empty() {
            i = 1 as u32;
            while i < (&(*keyi).groups).len() as u32 {
                if (&(*keyi).groups)[i as usize].defined as u64 != 0 {
                    warn = true;
                    ClearGroupInfo(&mut (&mut (*keyi).groups)[i as usize] as *mut GroupInfo);
                    InitGroupInfo(&mut (&mut (*keyi).groups)[i as usize] as *mut GroupInfo);
                }
                i = i.wrapping_add(1);
            }
        }
        if warn {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] For the map {} the explicit group {} is specified, but key {} has more than one group defined; All groups except first one will be ignored\n",
                XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE as i32,
                (*info).name.as_deref().unwrap_or(""),
                (*info).explicit_group.wrapping_add(1 as u32),
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
        }
        (*keyi).groups.resize_with(
            ((*info).explicit_group as usize).wrapping_add(1),
            Default::default,
        );
        if (*info).explicit_group > 0 as u32 {
            let swapped = std::ptr::read(&(&(*keyi).groups)[0]);
            std::ptr::write(
                &mut (&mut (*keyi).groups)[(*info).explicit_group as usize],
                swapped,
            );
            InitGroupInfo(&mut (&mut (*keyi).groups)[0] as *mut GroupInfo);
        }
        return true;
    }
}
unsafe fn HandleSymbolsDef(mut info: *mut SymbolsInfo, mut stmt: *mut SymbolsDef) -> bool {
    unsafe {
        let mut keyi: KeyInfo = KeyInfo::new_zeroed();
        keyi = (*info).default_key.clone();
        keyi.groups = Vec::new();
        if !(*info).default_key.groups.is_empty() {
            // Shallow copy the GroupInfo structs (bitwise), then deep-copy inner pointers
            keyi.groups
                .extend_from_slice(&(&(*info).default_key.groups));
        }
        let mut i: u32 = 0 as u32;
        while i < keyi.groups.len() as u32 {
            CopyGroupInfo(
                &mut keyi.groups[i as usize] as *mut GroupInfo,
                &(&(*info).default_key.groups)[i as usize] as *const GroupInfo,
            );
            i = i.wrapping_add(1);
        }
        keyi.merge = (*stmt).merge as merge_mode;
        keyi.name = (*stmt).keyName;
        if HandleSymbolsBody(info, (*stmt).symbols, &raw mut keyi) as i32 != 0
            && SetExplicitGroup(info, &raw mut keyi) as i32 != 0
            && AddKeySymbols(info, &raw mut keyi, true) as i32 != 0
        {
            return true;
        }
        ClearKeyInfo(&raw mut keyi);
        (*info).errorCount += 1;
        return false;
    }
}
unsafe fn HandleModMapDef(mut info: *mut SymbolsInfo, mut def: *mut ModMapDef) -> bool {
    unsafe {
        let mut tmp: ModMapEntry = ModMapEntry {
            merge: MERGE_DEFAULT,
            haveSymbol: false,
            modifier: 0,
            u: C2Rust_Unnamed_19 { keyName: 0 },
        };
        let mut ndx: u32 = 0;
        let mut ok: bool = false;
        let mut ctx: *mut xkb_context = (*info).ctx;
        let modifier_name: &[u8] = xkb_atom_text_bytes(&(*ctx).atom_table, (*def).modifier);
        if istreq(modifier_name, b"none") {
            ndx = XKB_MOD_NONE as u32;
        } else {
            ndx = XkbModNameToIndex(&raw mut (*info).mods, (*def).modifier, MOD_REAL);
            if ndx == XKB_MOD_INVALID as u32 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal modifier map definition; Ignoring map for non-modifier \"{}\"\n",
                    XKB_ERROR_INVALID_REAL_MODIFIER as i32,
                    crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*ctx).atom_table, (*def).modifier)),
                );
                return false;
            }
        }
        ok = true;
        tmp.modifier = ndx;
        tmp.merge = (*def).merge;
        let mut c2rust_current_block_19: u64;
        let mut key: *mut ExprDef = (*def).keys as *mut ExprDef;
        while !key.is_null() {
            if (*key).common.type_0 as u32 == STMT_EXPR_KEYNAME_LITERAL as u32 {
                tmp.haveSymbol = false;
                tmp.u.keyName = (*key).key_name.key_name;
                c2rust_current_block_19 = 5601891728916014340;
            } else if (*key).common.type_0 as u32 == STMT_EXPR_KEYSYM_LITERAL as u32 {
                if (*key).keysym.keysym == XKB_KEY_NoSymbol as u32 {
                    c2rust_current_block_19 = 13536709405535804910;
                } else {
                    tmp.haveSymbol = true;
                    tmp.u.keySym = (*key).keysym.keysym;
                    c2rust_current_block_19 = 5601891728916014340;
                }
            } else {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Modmap entries may contain only key names or keysyms; Illegal definition for {} modifier ignored\n",
                    XKB_ERROR_INVALID_MODMAP_ENTRY as i32,
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, tmp.modifier)),
                );
                c2rust_current_block_19 = 13536709405535804910;
            }
            match c2rust_current_block_19 {
                5601891728916014340 => {
                    ok = AddModMapEntry(info, &raw mut tmp) as i32 != 0 && ok as i32 != 0;
                }
                _ => {}
            }
            key = (*key).common.next as *mut ExprDef;
        }
        return ok;
    }
}
unsafe fn HandleSymbolsFile(mut info: *mut SymbolsInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        (*info).name = if (*file).name.is_null() {
            None
        } else {
            Some(String::from(
                std::ffi::CStr::from_ptr((*file).name)
                    .to_str()
                    .unwrap_or(""),
            ))
        };
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
                1 => {
                    ok = HandleIncludeSymbols(info, stmt as *mut IncludeStmt);
                }
                30 => {
                    ok = HandleSymbolsDef(info, stmt as *mut SymbolsDef);
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef((*info).ctx, &raw mut (*info).mods, stmt as *mut VModDef);
                }
                31 => {
                    ok = HandleModMapDef(info, stmt as *mut ModMapDef);
                }
                35 | 36 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported symbols {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        crate::xkb::utils::CStrDisplay(
                            if (*stmt).type_0 as u32 == STMT_UNKNOWN_COMPOUND as u32 {
                                b"compound\0".as_ptr() as *const i8
                            } else {
                                b"declaration\0".as_ptr() as *const i8
                            }
                        ),
                        crate::xkb::utils::CStrDisplay((*(stmt as *mut UnknownStatement)).name),
                    );
                    ok = (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_STATEMENTS as u32
                        == 0;
                }
                _ => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Symbols files may not include other types; Ignoring {}\n",
                        XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10 as i32 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Abandoning symbols file \"{}\"\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn FindKeyForSymbol(mut keymap: *mut xkb_keymap, mut sym: u32) -> *mut xkb_key {
    unsafe {
        let mut got_one_group: bool = false;
        let mut group: u32 = 0 as u32;
        loop {
            let mut level: u32 = 0 as u32;
            got_one_group = false;
            let mut got_one_level: bool = false;
            loop {
                got_one_level = false;
                let mut key: *mut xkb_key = std::ptr::null_mut();
                let start_idx = if (*keymap).num_keys_low == 0 as u32 {
                    0 as u32
                } else {
                    (*keymap).min_key_code
                };
                let mut ki: u32 = start_idx;
                while ki < (*keymap).num_keys {
                    key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                    if group < (*key).num_groups && level < XkbKeyNumLevels(keymap, key, group) {
                        got_one_level = true;
                        got_one_group = got_one_level;
                        let level_syms =
                            &(&(*key).groups)[group as usize].levels[level as usize].syms;
                        if level_syms.contains(&sym) {
                            return key;
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
        return std::ptr::null_mut();
    }
}
unsafe fn FindAutomaticType(mut ctx: *mut xkb_context, mut groupi: *mut GroupInfo) -> u32 {
    unsafe {
        let mut sym0: u32 = 0;
        let mut sym1: u32 = 0;
        let width: u32 = (*groupi).levels.len() as u32;
        if width == 1 as u32 || width <= 0 as u32 {
            return xkb_atom_intern(
                ctx,
                b"ONE_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
            );
        }
        sym0 = if (&(*groupi).levels)[0].syms.is_empty() {
            XKB_KEY_NoSymbol as u32
        } else {
            (&(*groupi).levels)[0].syms[0]
        };
        sym1 = if (&(*groupi).levels)[1].syms.is_empty() {
            XKB_KEY_NoSymbol as u32
        } else {
            (&(*groupi).levels)[1].syms[0]
        };
        if width == 2 as u32 {
            if xkb_keysym_is_lower(sym0) as i32 != 0
                && xkb_keysym_is_upper_or_title(sym1) as i32 != 0
            {
                return xkb_atom_intern(
                    ctx,
                    b"ALPHABETIC\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 11]>()).wrapping_sub(1 as usize),
                );
            }
            if xkb_keysym_is_keypad(sym0) as i32 != 0 || xkb_keysym_is_keypad(sym1) as i32 != 0 {
                return xkb_atom_intern(
                    ctx,
                    b"KEYPAD\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
                );
            }
            return xkb_atom_intern(
                ctx,
                b"TWO_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
            );
        }
        if width <= 4 as u32 {
            if xkb_keysym_is_lower(sym0) as i32 != 0
                && xkb_keysym_is_upper_or_title(sym1) as i32 != 0
            {
                let mut sym2: u32 = 0;
                let mut sym3: u32 = 0;
                sym2 = if (&(*groupi).levels)[2].syms.is_empty() {
                    XKB_KEY_NoSymbol as u32
                } else {
                    (&(*groupi).levels)[2].syms[0]
                };
                sym3 = if width == 4 as u32 {
                    if (&(*groupi).levels)[3].syms.is_empty() {
                        XKB_KEY_NoSymbol as u32
                    } else {
                        (&(*groupi).levels)[3].syms[0]
                    }
                } else {
                    XKB_KEY_NoSymbol as u32
                };
                if xkb_keysym_is_lower(sym2) as i32 != 0
                    && xkb_keysym_is_upper_or_title(sym3) as i32 != 0
                {
                    return xkb_atom_intern(
                        ctx,
                        b"FOUR_LEVEL_ALPHABETIC\0".as_ptr() as *const i8,
                        (std::mem::size_of::<[i8; 22]>()).wrapping_sub(1 as usize),
                    );
                }
                return xkb_atom_intern(
                    ctx,
                    b"FOUR_LEVEL_SEMIALPHABETIC\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 26]>()).wrapping_sub(1 as usize),
                );
            }
            if xkb_keysym_is_keypad(sym0) as i32 != 0 || xkb_keysym_is_keypad(sym1) as i32 != 0 {
                return xkb_atom_intern(
                    ctx,
                    b"FOUR_LEVEL_KEYPAD\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 18]>()).wrapping_sub(1 as usize),
                );
            }
            return xkb_atom_intern(
                ctx,
                b"FOUR_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 11]>()).wrapping_sub(1 as usize),
            );
        }
        return XKB_ATOM_NONE as u32;
    }
}
unsafe fn FindTypeForGroup(
    mut keymap: *mut xkb_keymap,
    mut keyi: *mut KeyInfo,
    mut group: u32,
    mut explicit_type: *mut bool,
) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let mut groupi: *mut GroupInfo =
            &mut (&mut (*keyi).groups)[group as usize] as *mut GroupInfo;
        let mut type_name: u32 = (*groupi).type_0;
        *explicit_type = true;
        if type_name == XKB_ATOM_NONE as u32 {
            if (*keyi).default_type != XKB_ATOM_NONE as u32 {
                type_name = (*keyi).default_type;
            } else {
                type_name = FindAutomaticType(&raw mut (*keymap).ctx, groupi);
                if type_name != XKB_ATOM_NONE as u32 {
                    *explicit_type = false;
                }
            }
        }
        if type_name == XKB_ATOM_NONE as u32 {
            xkb_logf!(
                (*keymap).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Couldn't find an automatic type for key '{}' group {} with {} levels; Using the default type\n",
                XKB_WARNING_CANNOT_INFER_KEY_TYPE as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyNameText((*keymap).ctx.clone(), (*keyi).name)),
                group.wrapping_add(1 as u32),
                (*groupi).levels.len(),
            );
        } else {
            i = 0;
            i = 0 as u32;
            while (i as usize) < (&(*keymap).types).len() {
                if (&(*keymap).types)[i as usize].name == type_name {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i as usize >= (&(*keymap).types).len() {
                xkb_logf!(
                    (*keymap).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The type \"{}\" for key '{}' group {} was not previously defined; Using the default type\n",
                    XKB_WARNING_UNDEFINED_KEY_TYPE as i32,
                    crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*keymap).ctx.atom_table, type_name)),
                    crate::xkb::utils::ByteSliceDisplay(KeyNameText((*keymap).ctx.clone(), (*keyi).name)),
                    group.wrapping_add(1 as u32),
                );
            } else {
                (&mut (*keymap).types)[i as usize].required = true;
                return i;
            }
        }
        (&mut (*keymap).types)[0].required = true;
        return 0;
    }
}
unsafe fn CopySymbolsDefToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
) -> bool {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
        let mut groupi: *mut GroupInfo = std::ptr::null_mut();
        let mut i: u32 = 0;

        // The name is guaranteed to be real and not an alias, so 'false' is safe here
        key = XkbKeyByName(keymap, (*keyi).name, false);
        if key.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_DETAILED as i32,
                "[XKB-{:03}] Key {} not found in keycodes; Symbols ignored\n",
                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }

        // Find the range of groups we need
        (*key).num_groups = 0;
        if !(*keyi).groups.is_empty() {
            i = 0;
            groupi = (*keyi).groups.as_mut_ptr();
            while i < (*keyi).groups.len() as u32 {
                // Skip groups that have no levels and no explicit type
                let has_explicit_type = ((*keyi).defined as i32 & KEY_FIELD_DEFAULT_TYPE as i32
                    != 0)
                    || ((*groupi).defined as u32 & GROUP_FIELD_TYPE as u32 != 0);
                if (*groupi).levels.len() > 0 || has_explicit_type {
                    (*key).num_groups = i.wrapping_add(1);
                }
                if has_explicit_type {
                    (*key).explicit =
                        ((*key).explicit as u32 | EXPLICIT_TYPES as u32) as xkb_explicit_components;
                }
                i = i.wrapping_add(1);
                groupi = groupi.offset(1);
            }
        }

        if (*key).num_groups <= 0 {
            // A key with no group may still have other fields defined
            if (*keyi).defined as i32 != 0 {
                // goto key_fields
            } else {
                return false;
            }
        } else {
            // Resize groups array
            let __need: usize = (*key).num_groups as usize;
            resize_groups_zero(&mut (*keyi).groups, __need);

            // If there are empty groups between non-empty ones, fill them with data from the first group
            if !(*keyi).groups.is_empty() {
                // Must use raw pointers because CopyGroupInfo takes ptrs and we need group0 as const while mutating others
                let groups_ptr = (*keyi).groups.as_mut_ptr();
                let groups_len = (*keyi).groups.len();
                i = 1;
                while i < groups_len as u32 {
                    if (*groups_ptr.add(i as usize)).defined == 0 {
                        CopyGroupInfo(groups_ptr.add(i as usize), groups_ptr as *const GroupInfo);
                    }
                    i = i.wrapping_add(1);
                }
            }

            (*key).groups = (0..(*key).num_groups)
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
            if !(*keyi).groups.is_empty() {
                i = 0;
                groupi = (*keyi).groups.as_mut_ptr();
                while i < (*keyi).groups.len() as u32 {
                    let mut explicit_type = false;
                    let type_idx: u32 = FindTypeForGroup(keymap, keyi, i, &raw mut explicit_type);

                    // Always have as many levels as the type specifies
                    if (&(*keymap).types)[type_idx as usize].num_levels
                        < (*groupi).levels.len() as u32
                    {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_BRIEF as i32,
                            "[XKB-{:03}] Type \"{}\" has {} levels, but {} has {} levels; Ignoring extra symbols\n",
                            XKB_WARNING_EXTRA_SYMBOLS_IGNORED as i32,
                            crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*keymap).ctx.atom_table, (&(*keymap).types)[type_idx as usize].name)),
                            (&(*keymap).types)[type_idx as usize].num_levels,
                            crate::xkb::utils::ByteSliceDisplay(KeyInfoText(info, keyi)),
                            (*groupi).levels.len(),
                        );

                        for lvl_idx in (&(*keymap).types)[type_idx as usize].num_levels as usize
                            ..(*groupi).levels.len()
                        {
                            (&mut (*groupi).levels)[lvl_idx].syms.clear();
                            (&mut (*groupi).levels)[lvl_idx].actions.clear();
                        }
                    }

                    // Resize levels array to match type
                    let __need_levels: usize =
                        (&(*keymap).types)[type_idx as usize].num_levels as usize;
                    (*groupi)
                        .levels
                        .resize_with(__need_levels, Default::default);

                    (&mut (*key).groups)[i as usize].explicit_type = explicit_type;
                    (&mut (*key).groups)[i as usize].type_idx = type_idx;

                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            // Copy levels
            if !(*keyi).groups.is_empty() {
                i = 0;
                groupi = (*keyi).groups.as_mut_ptr();
                while i < (*keyi).groups.len() as u32 {
                    // Compute the capitalization transformation of the keysyms
                    for li in 0..(&(*groupi).levels).len() {
                        let leveli = &mut (&mut (*groupi).levels)[li];
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
                    if (*groupi).levels.is_empty() {
                        (&mut (*key).groups)[i as usize].levels = Vec::new();
                    } else {
                        let stolen = std::mem::take(&mut (*groupi).levels);
                        (&mut (*key).groups)[i as usize].levels = stolen;
                    }

                    if (&(*keymap).types)[(&(*key).groups)[i as usize].type_idx as usize].num_levels
                        > 1
                        || !(&(*key).groups)[i as usize].levels[0].syms.is_empty()
                    {
                        (&mut (*key).groups)[i as usize].explicit_symbols = true;
                        (*key).explicit = ((*key).explicit as u32 | EXPLICIT_SYMBOLS as u32)
                            as xkb_explicit_components;
                    }
                    if (*groupi).defined as u32 & GROUP_FIELD_ACTS as u32 != 0 {
                        (&mut (*key).groups)[i as usize].explicit_actions = true;
                        (*key).explicit = ((*key).explicit as u32 | EXPLICIT_INTERP as u32)
                            as xkb_explicit_components;
                    }
                    if (&(*key).groups)[i as usize].explicit_type {
                        (*key).explicit = ((*key).explicit as u32 | EXPLICIT_TYPES as u32)
                            as xkb_explicit_components;
                    }

                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            (*key).out_of_range_pending_group = (*keyi).out_of_range_pending_group;
            (*key).out_of_range_group_number = (*keyi).out_of_range_group_number;
            (*key).out_of_range_group_policy = (*keyi).out_of_range_group_policy;
        }

        // key_fields:
        if (*keyi).defined as i32 & KEY_FIELD_VMODMAP as i32 != 0 {
            (*key).vmodmap = (*keyi).vmodmap;
            (*key).explicit =
                ((*key).explicit as u32 | EXPLICIT_VMODMAP as u32) as xkb_explicit_components;
        }

        if (*keyi).repeat != KEY_REPEAT_UNDEFINED as key_repeat {
            (*key).repeats = (*keyi).repeat == KEY_REPEAT_YES as key_repeat;
            (*key).explicit =
                ((*key).explicit as u32 | EXPLICIT_REPEAT as u32) as xkb_explicit_components;
        }

        if ((*keyi).defined as i32 & KEY_FIELD_OVERLAY as i32 != 0)
            && (*keyi).overlays != 0
            && !(*keyi).overlays_clear
        {
            // Remove null entries from overlay_keys and clear corresponding bits
            let mut clean_overlays: xkb_overlay_mask_t = 0;
            let mut clean_keys: Vec<*const xkb_key> = Vec::new();
            let mut remaining: xkb_overlay_mask_t = (*keyi).overlays;
            let mut idx: usize = 0;
            while remaining != 0 {
                let lsb: xkb_overlay_mask_t = remaining & (!remaining).wrapping_add(1);
                remaining &= !lsb;
                let k = if idx < (&(*keyi).overlay_keys).len() {
                    (&(*keyi).overlay_keys)[idx]
                } else {
                    std::ptr::null()
                };
                idx += 1;
                if !k.is_null() {
                    clean_overlays |= lsb;
                    clean_keys.push(k);
                }
            }

            if clean_overlays != 0 {
                (*key).overlays = clean_overlays;
                std::ptr::write(&raw mut (*key).overlay_keys, clean_keys);
                (*key).explicit =
                    ((*key).explicit as u32 | EXPLICIT_OVERLAY as u32) as xkb_explicit_components;
            }
        }

        return true;
    }
}

unsafe fn CopyModMapDefToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
    mut entry: *mut ModMapEntry,
) -> bool {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
        if !(*entry).haveSymbol {
            key = XkbKeyByName(keymap, (*entry).u.keyName, true);
            if key.is_null() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "[XKB-{:03}] Key {} not found in keycodes; Modifier map entry for {} not updated\n",
                    XKB_WARNING_UNDEFINED_KEYCODE as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeyNameText((*((*info).ctx)).clone(), (*entry).u.keyName)),
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, (*entry).modifier)),
                );
                return false;
            }
        } else {
            key = FindKeyForSymbol(keymap, (*entry).u.keySym);
            if key.is_null() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "[XKB-{:03}] Key \"{}\" not found in symbol map; Modifier map entry for {} not updated\n",
                    XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL as i32,
                    crate::xkb::utils::ByteSliceDisplay(KeysymText((*((*info).ctx)).clone(), (*entry).u.keySym)),
                    crate::xkb::utils::ByteSliceDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, (*entry).modifier)),
                );
                return false;
            }
        }
        if (*entry).modifier != XKB_MOD_NONE as u32 {
            (*key).modmap = ((*key).modmap as u32 | (1 as u32) << (*entry).modifier) as u32;
        }
        return true;
    }
}
unsafe fn CopySymbolsToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut SymbolsInfo) -> bool {
    unsafe {
        (*keymap).symbols_section_name = match &(*info).name {
            Some(s) => s.clone(),
            None => String::new(),
        };
        xkb_escape_map_name(&mut (*keymap).symbols_section_name);
        (*keymap).mods = (*info).mods;
        (*keymap).group_names = std::mem::take(&mut (*info).group_names);
        for keyi in (*info).keys.iter_mut() {
            if !CopySymbolsDefToKeymap(keymap, info, keyi as *mut KeyInfo) {
                (*info).errorCount += 1;
            }
        }
        if xkb_context_get_log_verbosity(&raw mut (*keymap).ctx) > 3 as i32 {
            let start_idx = if (*keymap).num_keys_low == 0 as u32 {
                0 as u32
            } else {
                (*keymap).min_key_code
            };
            let mut ki: u32 = start_idx;
            while ki < (*keymap).num_keys {
                let key: *mut xkb_key = &mut (&mut (*keymap).keys)[ki as usize] as *mut xkb_key;
                if !((*key).name == XKB_ATOM_NONE as u32) {
                    if ((*key).num_groups as i32) < 1 as i32 {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_INFO,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "No symbols defined for {}\n",
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                                (*(*info).ctx).clone(),
                                (*key).name
                            )),
                        );
                    }
                }
                ki = ki.wrapping_add(1);
            }
        }
        for mm in (*info).modmaps.iter_mut() {
            if !CopyModMapDefToKeymap(keymap, info, mm as *mut ModMapEntry) {
                (*info).errorCount += 1;
            }
        }
        return true;
    }
}
pub unsafe fn CompileSymbols(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: SymbolsInfo = SymbolsInfo::new_zeroed();
        InitSymbolsInfo(
            &raw mut info,
            keymap_info,
            0 as u32,
            &raw mut (*(*keymap_info).keymap).mods,
        );
        if !file.is_null() {
            HandleSymbolsFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopySymbolsToKeymap((*keymap_info).keymap, &raw mut info) {
                ClearSymbolsInfo(&raw mut info);
                return true;
            }
        }
        ClearSymbolsInfo(&raw mut info);
        return false;
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::keysym_case_mappings::xkb_keysym_to_upper;
use crate::xkb::shared_types::*;
