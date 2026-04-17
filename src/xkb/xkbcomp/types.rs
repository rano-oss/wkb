use super::prelude::*;
use crate::xkb::context::xkb_atom_intern;
pub use crate::xkb::shared_ast_types::{KeyTypeDef, ReportShouldBeArray};
use crate::xkb::text::ModMaskText;
use crate::xkb::xkbcomp::expr::ExprResolveLevel;
#[derive(Clone)]
pub struct KeyTypesInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub types: Vec<KeyTypeInfo>,
    pub mods: xkb_mod_set,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
impl KeyTypesInfo {
    pub fn new_zeroed() -> Self {
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            types: Vec::new(),
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0_u32,
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
#[derive(Clone)]
pub struct KeyTypeInfo {
    pub defined: type_field,
    pub merge: merge_mode,
    pub name: u32,
    pub mods: u32,
    pub num_levels: u32,
    pub entries: Vec<xkb_key_type_entry>,
    pub level_names: Vec<u32>,
}
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
pub type type_field = u32;
pub const TYPE_FIELD_LEVEL_NAME: type_field = 8;
pub const TYPE_FIELD_PRESERVE: type_field = 4;
pub const TYPE_FIELD_MAP: type_field = 2;
pub const TYPE_FIELD_MASK: type_field = 1;
#[inline]
unsafe fn MapEntryTxt(info: *mut KeyTypesInfo, entry: *mut xkb_key_type_entry) -> &'static [u8] {
    unsafe {
        ModMaskText(
            (*(*info).ctx).clone(),
            MOD_BOTH,
            &raw mut (*info).mods,
            (*entry).mods.mods,
        )
    }
}
#[inline]
unsafe fn TypeTxt<'a>(info: *mut KeyTypesInfo, type_0: *mut KeyTypeInfo) -> &'a str {
    unsafe { xkb_atom_text(&(*(*info).ctx).atom_table, (*type_0).name) }
}
#[inline]
unsafe fn ReportTypeShouldBeArray(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    field: &str,
) -> bool {
    unsafe { ReportShouldBeArray((*info).ctx, "key type", field, TypeTxt(info, type_0)) }
}
#[inline]
unsafe fn ReportTypeBadType(
    info: *mut KeyTypesInfo,
    code: xkb_message_code,
    type_0: *mut KeyTypeInfo,
    field: &str,
    wanted: &str,
) -> bool {
    unsafe {
        ReportBadType(
            (*info).ctx,
            code,
            "key type",
            field,
            TypeTxt(info, type_0),
            wanted,
        )
    }
}
unsafe fn InitKeyTypesInfo(
    info: *mut KeyTypesInfo,
    keymap_info: *const xkb_keymap_info,
    include_depth: u32,
    mods: *const xkb_mod_set,
) {
    unsafe {
        (*info).name = None;
        (*info).errorCount = 0;
        (*info).include_depth = 0;
        (*info).types = Vec::new();
        (*info).mods = std::mem::zeroed();
        (*info).ctx = &raw mut (*(*keymap_info).keymap).ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0_u32);
    }
}
unsafe fn ClearKeyTypeInfo(type_0: *mut KeyTypeInfo) {
    unsafe {
        (*type_0).entries.clear();
        (*type_0).level_names.clear();
    }
}
unsafe fn ClearKeyTypesInfo(info: *mut KeyTypesInfo) {
    unsafe {
        (*info).name = None;
        for type_0 in (*info).types.iter_mut() {
            ClearKeyTypeInfo(type_0 as *mut KeyTypeInfo);
        }
        (*info).types.clear();
    }
}
unsafe fn AddKeyType(info: *mut KeyTypesInfo, new: *mut KeyTypeInfo, same_file: bool) -> bool {
    unsafe {
        let mut old: *mut KeyTypeInfo = std::ptr::null_mut();
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx);
        // FindMatchingKeyType inlined
        for type_0 in (*info).types.iter_mut() {
            if type_0.name == (*new).name {
                old = type_0 as *mut KeyTypeInfo;
                break;
            }
        }
        if !old.is_null() {
            if (*new).merge != MERGE_AUGMENT {
                if same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32 {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Multiple definitions of the {} key type; Earlier definition ignored\n",
                        XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS
                            as i32,
                        xkb_atom_text(&(*(*info).ctx).atom_table, (*new).name),
                    );
                }
                ClearKeyTypeInfo(old);
                *old = (*new).clone();
                (*new).entries = Vec::new();
                (*new).level_names = Vec::new();
                return true;
            }
            if same_file {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "[XKB-{:03}] Multiple definitions of the {} key type; Later definition ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS as i32,
                    xkb_atom_text(&(*(*info).ctx).atom_table, (*new).name),
                );
            }
            ClearKeyTypeInfo(new);
            return true;
        }
        (*info).types.push((*new).clone());
        true
    }
}
unsafe fn MergeIncludedKeyTypes(
    into: *mut KeyTypesInfo,
    from: *mut KeyTypesInfo,
    merge: merge_mode,
) {
    unsafe {
        if (*from).errorCount > 0_i32 {
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
        if (*into).types.is_empty() {
            (*into).types = std::mem::take(&mut (*from).types);
        } else {
            let mut type_0: *mut KeyTypeInfo;
            for i in 0..(*from).types.len() {
                type_0 = (*from).types.as_mut_ptr().add(i);
                (*type_0).merge = merge;
                if !AddKeyType(into, type_0, false) {
                    (*into).errorCount += 1;
                }
            }
            (*from).types.clear();
        };
    }
}
unsafe fn HandleIncludeKeyTypes(info: *mut KeyTypesInfo, include: *mut IncludeStmt) -> bool {
    unsafe {
        let mut included: KeyTypesInfo = KeyTypesInfo::new_zeroed();
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10_i32;
            return false;
        }
        InitKeyTypesInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1_u32),
            &raw mut (*info).mods,
        );
        included.name = {
            let ptr = _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *const i8;
            if ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(ptr)
                        .to_str()
                        .unwrap_or("")
                        .to_string(),
                )
            }
        };
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyTypesInfo = KeyTypesInfo::new_zeroed();

            let mut path: [i8; 4096] = [0; 4096];
            let file: *mut XkbFile = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_TYPES,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10_i32;
                ClearKeyTypesInfo(&raw mut included);
                return false;
            }
            InitKeyTypesInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1_u32),
                &raw mut included.mods,
            );
            HandleKeyTypesFile(&raw mut next_incl, file);
            MergeIncludedKeyTypes(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearKeyTypesInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedKeyTypes(info, &raw mut included, (*include).merge);
        ClearKeyTypesInfo(&raw mut included);
        (*info).errorCount == 0_i32
    }
}
unsafe fn SetModifiers(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut mods: u32 = 0_u32;
        if !arrayNdx.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The modifiers field of a key type is not an array; Illegal array subscript ignored\n",
            );
            return false;
        }
        if !ExprResolveModMask(
            (*info).ctx,
            value,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut mods,
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Key type mask field must be a modifier mask; Key type definition ignored\n",
                { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK },
            );
            return false;
        }
        if (*type_0).defined & TYPE_FIELD_MASK != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Multiple modifier mask definitions for key type {}; Using {}, ignoring {}\n",
                xkb_atom_text(&(*(*info).ctx).atom_table, (*type_0).name),
                crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                    (*(*info).ctx).clone(),
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    (*type_0).mods
                )),
                crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                    (*(*info).ctx).clone(),
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    mods
                )),
            );
            return false;
        }
        (*type_0).mods = mods;
        true
    }
}
unsafe fn AddMapEntry(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    new: *mut xkb_key_type_entry,
    clobber: bool,
    report: bool,
) -> bool {
    unsafe {
        let mut old: *mut xkb_key_type_entry = std::ptr::null_mut();
        // FindMatchingMapEntry inlined
        for entry in (*type_0).entries.iter_mut() {
            if entry.mods.mods == (*new).mods.mods {
                old = entry as *mut xkb_key_type_entry;
                break;
            }
        }
        if !old.is_null() {
            if report as i32 != 0 && (*old).level != (*new).level {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Multiple map entries for {} in {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                    crate::xkb::utils::ByteSliceDisplay(MapEntryTxt(info, new)),
                    TypeTxt(info, type_0),
                    (if clobber as i32 != 0 {
                        (*new).level
                    } else {
                        (*old).level
                    })
                    .wrapping_add(1_u32),
                    (if clobber as i32 != 0 {
                        (*old).level
                    } else {
                        (*new).level
                    })
                    .wrapping_add(1_u32),
                );
            } else {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_VERBOSE as i32,
                    "[XKB-{:03}] Multiple occurrences of map[{}]= {} in {}; Ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                    crate::xkb::utils::ByteSliceDisplay(MapEntryTxt(info, new)),
                    (*new).level.wrapping_add(1_u32),
                    TypeTxt(info, type_0),
                );
                return true;
            }
            if clobber {
                if (*new).level >= (*type_0).num_levels {
                    (*type_0).num_levels = (*new).level.wrapping_add(1_u32);
                }
                (*old).level = (*new).level;
            }
            return true;
        }
        if (*new).level >= (*type_0).num_levels {
            (*type_0).num_levels = (*new).level.wrapping_add(1_u32);
        }
        (*type_0).entries.push(*new);
        true
    }
}
unsafe fn SetMapEntry(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut entry: xkb_key_type_entry = xkb_key_type_entry {
            level: 0,
            mods: xkb_mods { mods: 0, mask: 0 },
            preserve: xkb_mods { mods: 0, mask: 0 },
        };
        if arrayNdx.is_null() {
            return ReportTypeShouldBeArray(info, type_0, "map entry");
        }
        if !ExprResolveModMask(
            (*info).ctx,
            arrayNdx,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut entry.mods.mods,
        ) {
            return ReportTypeBadType(
                info,
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
                type_0,
                "map entry",
                "modifier mask",
            );
        }
        if entry.mods.mods & !(*type_0).mods != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Map entry for modifiers not used by type {}; Using {} instead of {}\n",
                XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
                TypeTxt(info, type_0),
                crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                    (*(*info).ctx).clone(),
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    entry.mods.mods & (*type_0).mods,
                )),
                crate::xkb::utils::ByteSliceDisplay(MapEntryTxt(info, &raw mut entry)),
            );
            entry.mods.mods &= (*type_0).mods;
        }
        if !ExprResolveLevel((*info).ctx, value, &raw mut entry.level) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Level specifications in a key type must be integer; Ignoring malformed level specification\n",
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32,
            );
            return false;
        }
        entry.preserve.mods = 0_u32;
        AddMapEntry(info, type_0, &raw mut entry, true, true)
    }
}
unsafe fn AddPreserve(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    mods: u32,
    preserve_mods: u32,
) -> bool {
    unsafe {
        let _entry: *mut xkb_key_type_entry = std::ptr::null_mut();
        let mut new: xkb_key_type_entry = xkb_key_type_entry {
            level: 0,
            mods: xkb_mods { mods: 0, mask: 0 },
            preserve: xkb_mods { mods: 0, mask: 0 },
        };
        for e in (*type_0).entries.iter_mut() {
            if e.mods.mods != mods {
                continue;
            } else {
                if e.preserve.mods == 0_u32 {
                    e.preserve.mods = preserve_mods;
                    return true;
                }
                if e.preserve.mods == preserve_mods {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_VERBOSE as i32,
                        "[XKB-{:03}] Identical definitions for preserve[{}] in {}; Ignored\n",
                        XKB_WARNING_DUPLICATE_ENTRY as i32,
                        crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                            (*(*info).ctx).clone(),
                            MOD_BOTH,
                            &raw mut (*info).mods,
                            mods
                        )),
                        TypeTxt(info, type_0),
                    );
                    return true;
                }
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as i32,
                    "[XKB-{:03}] Multiple definitions for preserve[{}] in {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES
                        as i32,
                    crate::xkb::utils::ByteSliceDisplay(ModMaskText((*(*info).ctx).clone(), MOD_BOTH, &raw mut (*info).mods, mods)),
                    TypeTxt(info, type_0),
                    crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                        (*(*info).ctx).clone(),
                        MOD_BOTH,
                        &raw mut (*info).mods,
                        preserve_mods,
                    )),
                    crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                        (*(*info).ctx).clone(),
                        MOD_BOTH,
                        &raw mut (*info).mods,
                        e.preserve.mods,
                    )),
                );
                e.preserve.mods = preserve_mods;
                return true;
            }
        }
        new.level = 0_u32;
        new.mods.mods = mods;
        new.preserve.mods = preserve_mods;
        (*type_0).entries.push(new);
        true
    }
}
unsafe fn SetPreserve(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            return ReportTypeShouldBeArray(info, type_0, "preserve entry");
        }
        let mut mods: u32 = 0_u32;
        if !ExprResolveModMask(
            (*info).ctx,
            arrayNdx,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut mods,
        ) {
            return ReportTypeBadType(
                info,
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
                type_0,
                "preserve entry",
                "modifier mask",
            );
        }
        if mods & !(*type_0).mods != 0 {
            let before: &[u8] = ModMaskText(
                (*(*info).ctx).clone(),
                MOD_BOTH,
                &raw mut (*info).mods,
                mods,
            );
            mods &= (*type_0).mods;
            let after: &[u8] = ModMaskText(
                (*(*info).ctx).clone(),
                MOD_BOTH,
                &raw mut (*info).mods,
                mods,
            );
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Preserve entry for modifiers not used by the {} type; Index {} converted to {}\n",
                XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
                TypeTxt(info, type_0),
                crate::xkb::utils::ByteSliceDisplay(before),
                crate::xkb::utils::ByteSliceDisplay(after),
            );
        }
        let mut preserve_mods: u32 = 0_u32;
        if !ExprResolveModMask(
            (*info).ctx,
            value,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut preserve_mods,
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Preserve value in a key type is not a modifier mask; Ignoring preserve[{}] in type {}\n",
                { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK },
                crate::xkb::utils::ByteSliceDisplay(ModMaskText((*(*info).ctx).clone(), MOD_BOTH, &raw mut (*info).mods, mods)),
                TypeTxt(info, type_0),
            );
            return false;
        }
        if preserve_mods & !mods != 0 {
            let before_0: &[u8] = ModMaskText(
                (*(*info).ctx).clone(),
                MOD_BOTH,
                &raw mut (*info).mods,
                preserve_mods,
            );
            preserve_mods &= mods;
            let after_0: &[u8] = ModMaskText(
                (*(*info).ctx).clone(),
                MOD_BOTH,
                &raw mut (*info).mods,
                preserve_mods,
            );
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Illegal value for preserve[{}] in type {}; Converted {} to {}\n",
                XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT as i32,
                crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                    (*(*info).ctx).clone(),
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    mods
                )),
                TypeTxt(info, type_0),
                crate::xkb::utils::ByteSliceDisplay(before_0),
                crate::xkb::utils::ByteSliceDisplay(after_0),
            );
        }
        AddPreserve(info, type_0, mods, preserve_mods)
    }
}
unsafe fn AddLevelName(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    level: u32,
    name: u32,
    clobber: bool,
) -> bool {
    unsafe {
        if level >= (*type_0).level_names.len() as u32 {
            vec_resize_zero(&mut (*type_0).level_names, (level as usize).wrapping_add(1));
        } else {
            if *(*type_0).level_names.as_ptr().add(level as usize) == name {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_VERBOSE as i32,
                    "[XKB-{:03}] Duplicate names for level {} of key type {}; Ignored\n",
                    XKB_WARNING_DUPLICATE_ENTRY as i32,
                    level.wrapping_add(1_u32),
                    TypeTxt(info, type_0),
                );
                return true;
            }
            if *(*type_0).level_names.as_ptr().add(level as usize) != XKB_ATOM_NONE {
                let old: &str = xkb_atom_text(
                    &(*(*info).ctx).atom_table,
                    *(*type_0).level_names.as_ptr().add(level as usize),
                );
                let new: &str = xkb_atom_text(&(*(*info).ctx).atom_table, name);
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as i32,
                    "[XKB-{:03}] Multiple names for level {} of key type {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES as i32,
                    level.wrapping_add(1_u32),
                    TypeTxt(info, type_0),
                    if clobber { new } else { old },
                    if clobber { old } else { new },
                );
                if !clobber {
                    return true;
                }
            }
        }
        *(*type_0).level_names.as_mut_ptr().add(level as usize) = name;
        true
    }
}
unsafe fn SetLevelName(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            return ReportTypeShouldBeArray(info, type_0, "level name");
        }
        let mut level: u32 = 0_u32;
        if !ExprResolveLevel((*info).ctx, arrayNdx, &raw mut level) {
            return ReportTypeBadType(
                info,
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
                type_0,
                "level name",
                "integer",
            );
        }
        let mut level_name: u32 = XKB_ATOM_NONE;
        if !ExprResolveString((*info).ctx, value, &raw mut level_name) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Non-string name for level {} in key type {}; Ignoring illegal level name definition\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                level.wrapping_add(1_u32),
                xkb_atom_text(&(*(*info).ctx).atom_table, (*type_0).name),
            );
            return false;
        }
        AddLevelName(info, type_0, level, level_name, true)
    }
}
unsafe fn SetKeyTypeField(
    info: *mut KeyTypesInfo,
    type_0: *mut KeyTypeInfo,
    field: &str,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    unsafe {
        let ok: bool;
        let mut type_field: type_field = 0 as type_field;
        if field.eq_ignore_ascii_case("modifiers") {
            type_field = TYPE_FIELD_MASK;
            ok = SetModifiers(info, type_0, arrayNdx, value);
        } else if field.eq_ignore_ascii_case("map") {
            type_field = TYPE_FIELD_MAP;
            ok = SetMapEntry(info, type_0, arrayNdx, value);
        } else if field.eq_ignore_ascii_case("preserve") {
            type_field = TYPE_FIELD_PRESERVE;
            ok = SetPreserve(info, type_0, arrayNdx, value);
        } else if field.eq_ignore_ascii_case("levelname")
            || field.eq_ignore_ascii_case("level_name")
        {
            type_field = TYPE_FIELD_LEVEL_NAME;
            ok = SetLevelName(info, type_0, arrayNdx, value);
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown field \"{}\" in key type \"{}\"; Definition ignored\n",
                XKB_ERROR_UNKNOWN_FIELD as i32,
                field,
                TypeTxt(info, type_0),
            );
            ok = (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_TYPE_FIELDS == 0;
        }
        (*type_0).defined = ((*type_0).defined | type_field as u32) as type_field;
        ok
    }
}
unsafe fn HandleKeyTypeBody(
    info: *mut KeyTypesInfo,
    mut def: *mut VarDef,
    type_0: *mut KeyTypeInfo,
) -> bool {
    unsafe {
        let mut ok: bool = true;
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        while !def.is_null() {
            if !ExprResolveLhs(
                (*info).ctx,
                (*def).name,
                &mut elem,
                &mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_empty() {
                if elem.eq_ignore_ascii_case("type") {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Support for changing the default type has been removed; Statement \"{}.{}\" ignored.\n",
                        XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT as i32,
                        elem,
                        field,
                    );
                } else {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key type statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                        elem,
                        elem,
                        field,
                    );
                    ok = false;
                }
            } else if !SetKeyTypeField(info, type_0, field, arrayNdx, (*def).value as *mut ExprDef)
            {
                ok = false;
            }
            def = (*def).common.next as *mut VarDef;
        }
        ok
    }
}
unsafe fn HandleGlobalVar(info: *mut KeyTypesInfo, stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &mut elem,
            &mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("type") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Support for changing the default type has been removed; Statement ignored\n",
                XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
            );
            return true;
        } else if !elem.is_empty() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown element \"{}\"; Value for field \"{}.{}\" ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                elem,
                elem,
                field,
            );
            return (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
        } else if !field.is_empty() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                field,
            );
            return (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS == 0;
        }
        false
    }
}
unsafe fn HandleKeyTypesFile(info: *mut KeyTypesInfo, file: *mut XkbFile) {
    unsafe {
        let mut ok: bool;
        (*info).name = {
            let ptr = (*file).name as *const i8;
            if ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(ptr)
                        .to_str()
                        .unwrap_or("")
                        .to_string(),
                )
            }
        };
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 {
                1 => {
                    ok = HandleIncludeKeyTypes(info, stmt as *mut IncludeStmt);
                }
                27 => {
                    // HandleKeyTypeDef inlined
                    let def = stmt as *mut KeyTypeDef;
                    let mut type_0: KeyTypeInfo = KeyTypeInfo {
                        defined: 0 as type_field,
                        merge: (*def).merge,
                        name: (*def).name,
                        mods: 0_u32,
                        num_levels: 1_u32,
                        entries: Vec::new(),
                        level_names: Vec::new(),
                    };
                    if !HandleKeyTypeBody(info, (*def).body, &raw mut type_0)
                        || !AddKeyType(info, &raw mut type_0, true)
                    {
                        (*info).errorCount += 1;
                        ClearKeyTypeInfo(&raw mut type_0);
                        ok = false;
                    } else {
                        ok = true;
                    }
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef((*info).ctx, &raw mut (*info).mods, stmt as *mut VModDef);
                }
                35 | 36 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported types {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        crate::xkb::utils::CStrDisplay(
                            if (*stmt).type_0 == STMT_UNKNOWN_COMPOUND {
                                b"compound\0".as_ptr() as *const i8
                            } else {
                                b"declaration\0".as_ptr() as *const i8
                            }
                        ),
                        crate::xkb::utils::CStrDisplay((*(stmt as *mut UnknownStatement)).name),
                    );
                    ok = (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    println!(
                "{:?} {} {} [XKB-{{:03}}] Key type files may not include other declarations; Ignoring {{}}\n {} {:?}",
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        { XKB_LOG_VERBOSITY_MINIMAL },
                        XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10_i32 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Abandoning keytypes file \"{}\"\n",
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
unsafe fn CopyKeyTypesToKeymap(keymap: *mut xkb_keymap, info: *mut KeyTypesInfo) -> bool {
    unsafe {
        let num_types: u32 = if (*info).types.is_empty() {
            1_u32
        } else {
            (*info).types.len() as u32
        };
        let mut types_vec: Vec<xkb_key_type> = Vec::with_capacity(num_types as usize);
        if (*info).types.is_empty() {
            let type_0 = xkb_key_type {
                name: xkb_atom_intern(&raw mut (*keymap).ctx, b"ONE_LEVEL"),
                mods: xkb_mods { mods: 0, mask: 0 },
                required: true,
                num_levels: 1,
                level_names: Vec::new(),
                entries: Vec::new(),
            };
            types_vec.push(type_0);
        } else {
            let canonical_types: [u32; 4] = [
                xkb_atom_intern(&raw mut (*keymap).ctx, b"ONE_LEVEL"),
                xkb_atom_intern(&raw mut (*keymap).ctx, b"TWO_LEVEL"),
                xkb_atom_intern(&raw mut (*keymap).ctx, b"ALPHABETIC"),
                xkb_atom_intern(&raw mut (*keymap).ctx, b"KEYPAD"),
            ];
            let mut i: u32 = 0_u32;
            while i < num_types {
                let def: *mut KeyTypeInfo = (*info).types.as_mut_ptr().add(i as usize);
                let level_names = std::mem::take(&mut (*def).level_names);
                let entries = std::mem::take(&mut (*def).entries);
                let mut required = false;
                if (*def).num_levels <= 2 {
                    for t in 0..4 {
                        if (*def).name == canonical_types[t] {
                            required = true;
                            break;
                        }
                    }
                }
                types_vec.push(xkb_key_type {
                    name: (*def).name,
                    mods: xkb_mods {
                        mods: (*def).mods,
                        mask: 0,
                    },
                    required,
                    num_levels: (*def).num_levels,
                    level_names,
                    entries,
                });
                i = i.wrapping_add(1);
            }
        }
        (*keymap).types_section_name = match &(*info).name {
            Some(s) => s.clone(),
            None => String::new(),
        };
        xkb_escape_map_name(&mut (*keymap).types_section_name);
        (*keymap).types = types_vec;
        (*keymap).mods = (*info).mods;
        true
    }
}
pub unsafe fn CompileKeyTypes(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let mut info: KeyTypesInfo = KeyTypesInfo::new_zeroed();
        InitKeyTypesInfo(
            &raw mut info,
            keymap_info,
            0_u32,
            &raw mut (*(*keymap_info).keymap).mods,
        );
        if !file.is_null() {
            HandleKeyTypesFile(&raw mut info, file);
        }
        if (info.errorCount == 0_i32) && CopyKeyTypesToKeymap((*keymap_info).keymap, &raw mut info)
        {
            ClearKeyTypesInfo(&raw mut info);
            return true;
        }
        ClearKeyTypesInfo(&raw mut info);
        false
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
