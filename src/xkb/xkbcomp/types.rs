use super::prelude::*;
use crate::xkb::context::xkb_atom_intern;
pub use crate::xkb::shared_ast_types::{KeyTypeDef, ReportShouldBeArray};
use crate::xkb::text::ModMaskText;
use crate::xkb::xkbcomp::expr::ExprResolveLevel;
pub struct KeyTypesInfo<'a> {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub types: Vec<KeyTypeInfo>,
    pub mods: xkb_mod_set,
    pub ctx: &'a mut xkb_context,
    pub keymap_info: &'a mut xkb_keymap_info,
}
impl<'a> KeyTypesInfo<'a> {
    pub fn new(ctx: &'a mut xkb_context, keymap_info: &'a mut xkb_keymap_info) -> Self {
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            types: Vec::new(),
            mods: unsafe { std::mem::zeroed() },
            ctx,
            keymap_info,
        }
    }
    #[inline]
    pub fn ctx(&self) -> &xkb_context {
        &*self.ctx
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
fn MapEntryTxt(info: &KeyTypesInfo<'_>, entry: &xkb_key_type_entry) -> String {
    ModMaskText(info.ctx(), MOD_BOTH, &info.mods, entry.mods.mods)
}
#[inline]
fn TypeTxt<'a>(info: &'a KeyTypesInfo<'_>, type_0: &KeyTypeInfo) -> &'a str {
    xkb_atom_text(&info.ctx().atom_table, type_0.name)
}
#[inline]
fn ReportTypeShouldBeArray(info: &KeyTypesInfo<'_>, type_0: &KeyTypeInfo, field: &str) -> bool {
    unsafe {
        ReportShouldBeArray(
            info.ctx as *const _ as *mut _,
            "key type",
            field,
            TypeTxt(info, type_0),
        )
    }
}
#[inline]
fn ReportTypeBadType(
    info: &KeyTypesInfo<'_>,
    code: u32,
    type_0: &KeyTypeInfo,
    field: &str,
    wanted: &str,
) -> bool {
    unsafe {
        ReportBadType(
            info.ctx as *const _ as *mut _,
            code,
            "key type",
            field,
            TypeTxt(info, type_0),
            wanted,
        )
    }
}
fn InitKeyTypesInfo(info: &mut KeyTypesInfo<'_>, include_depth: u32, mods: &xkb_mod_set) {
    info.name = None;
    info.errorCount = 0;
    info.include_depth = include_depth;
    info.types = Vec::new();
    info.mods = unsafe { std::mem::zeroed() };
    InitVMods(&mut info.mods, mods, include_depth > 0_u32);
}
fn ClearKeyTypeInfo(type_0: &mut KeyTypeInfo) {
    type_0.entries.clear();
    type_0.level_names.clear();
}
fn ClearKeyTypesInfo(info: &mut KeyTypesInfo<'_>) {
    info.name = None;
    for type_0 in info.types.iter_mut() {
        ClearKeyTypeInfo(type_0);
    }
    info.types.clear();
}
fn AddKeyType(info: &mut KeyTypesInfo<'_>, new: &mut KeyTypeInfo, same_file: bool) -> bool {
    let verbosity: i32 = xkb_context_get_log_verbosity(info.ctx());
    // FindMatchingKeyType inlined
    let mut old_idx: Option<usize> = None;
    for (i, type_0) in info.types.iter().enumerate() {
        if type_0.name == new.name {
            old_idx = Some(i);
            break;
        }
    }
    if let Some(idx) = old_idx {
        if new.merge != MERGE_AUGMENT {
            if same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32 {
                log::warn!("[XKB-{:03}] Multiple definitions of the {} key type; Earlier definition ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS
                        as i32,
                    xkb_atom_text(&info.ctx().atom_table, new.name));
            }
            ClearKeyTypeInfo(&mut info.types[idx]);
            info.types[idx] = new.clone();
            new.entries = Vec::new();
            new.level_names = Vec::new();
            return true;
        }
        if same_file {
            log::warn!(
                "[XKB-{:03}] Multiple definitions of the {} key type; Later definition ignored\n",
                XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS as i32,
                xkb_atom_text(&info.ctx().atom_table, new.name)
            );
        }
        ClearKeyTypeInfo(new);
        return true;
    }
    info.types.push(new.clone());
    true
}
fn MergeIncludedKeyTypes(
    into: &mut KeyTypesInfo<'_>,
    from: &mut KeyTypesInfo<'_>,
    merge: merge_mode,
) {
    if from.errorCount > 0_i32 {
        into.errorCount += from.errorCount;
        return;
    }
    MergeModSets(&mut *into.ctx, &mut into.mods, &from.mods, merge);
    if into.name.is_none() {
        into.name = from.name.take();
    }
    if into.types.is_empty() {
        into.types = std::mem::take(&mut from.types);
    } else {
        for i in 0..from.types.len() {
            from.types[i].merge = merge;
            let mut type_clone = from.types[i].clone();
            if !AddKeyType(into, &mut type_clone, false) {
                into.errorCount += 1;
            }
        }
        from.types.clear();
    }
}
fn HandleIncludeKeyTypes(info: &mut KeyTypesInfo<'_>, include: *mut IncludeStmt) -> bool {
    unsafe {
        let ctx_ptr = &raw mut *info.ctx;
        let ki_ptr = &raw mut *info.keymap_info;
        let mut included = KeyTypesInfo::new(&mut *ctx_ptr, &mut *ki_ptr);
        if ExceedsIncludeMaxDepth(&mut *ctx_ptr, info.include_depth) {
            info.errorCount += 10_i32;
            return false;
        }
        InitKeyTypesInfo(
            &mut included,
            info.include_depth.wrapping_add(1_u32),
            &info.mods,
        );
        included.name = {
            let inc = &mut *include;
            if inc.stmt.is_empty() {
                None
            } else {
                Some(std::mem::take(&mut inc.stmt))
            }
        };
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl = KeyTypesInfo::new(&mut *ctx_ptr, &mut *ki_ptr);

            let mut path: [i8; 4096] = [0; 4096];
            let file: *mut XkbFile = ProcessIncludeFile(
                &mut *ctx_ptr,
                stmt,
                FILE_TYPE_TYPES,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                info.errorCount += 10_i32;
                ClearKeyTypesInfo(&mut included);
                return false;
            }
            InitKeyTypesInfo(
                &mut next_incl,
                info.include_depth.wrapping_add(1_u32),
                &included.mods,
            );
            HandleKeyTypesFile(&mut next_incl, file);
            MergeIncludedKeyTypes(&mut included, &mut next_incl, (*stmt).merge);
            ClearKeyTypesInfo(&mut next_incl);
            FreeXkbFile(file);
            stmt = match (*stmt).next_incl {
                Some(ref mut b) => b.as_mut() as *mut IncludeStmt,
                None => std::ptr::null_mut(),
            };
        }
        MergeIncludedKeyTypes(info, &mut included, (*include).merge);
        ClearKeyTypesInfo(&mut included);
        info.errorCount == 0_i32
    }
}
fn SetModifiers(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    let mut mods: u32 = 0_u32;
    if !arrayNdx.is_null() {
        log::error!(
            "The modifiers field of a key type is not an array; Illegal array subscript ignored\n"
        );
        return false;
    }
    if !unsafe { ExprResolveModMask(info.ctx, value, MOD_BOTH, &info.mods, &raw mut mods) } {
        log::error!("[XKB-{:03}] Key type mask field must be a modifier mask; Key type definition ignored\n",
            { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK });
        return false;
    }
    if type_0.defined & TYPE_FIELD_MASK != 0 {
        log::warn!(
            "Multiple modifier mask definitions for key type {}; Using {}, ignoring {}\n",
            xkb_atom_text(&info.ctx().atom_table, type_0.name),
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, type_0.mods),
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods)
        );
        return false;
    }
    type_0.mods = mods;
    true
}
fn AddMapEntry(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    new: &xkb_key_type_entry,
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
        if report && old.level != new.level {
            log::warn!(
                "[XKB-{:03}] Multiple map entries for {} in {}; Using {}, ignoring {}\n",
                XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                MapEntryTxt(info, new),
                TypeTxt(info, type_0),
                (if clobber { new.level } else { old.level }).wrapping_add(1_u32),
                (if clobber { old.level } else { new.level }).wrapping_add(1_u32)
            );
        } else {
            log::warn!(
                "[XKB-{:03}] Multiple occurrences of map[{}]= {} in {}; Ignored\n",
                XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                MapEntryTxt(info, new),
                new.level.wrapping_add(1_u32),
                TypeTxt(info, type_0)
            );
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
fn SetMapEntry(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    let mut entry: xkb_key_type_entry = xkb_key_type_entry {
        level: 0,
        mods: xkb_mods { mods: 0, mask: 0 },
        preserve: xkb_mods { mods: 0, mask: 0 },
    };
    if arrayNdx.is_null() {
        return ReportTypeShouldBeArray(info, type_0, "map entry");
    }
    if !unsafe {
        ExprResolveModMask(
            info.ctx,
            arrayNdx,
            MOD_BOTH,
            &info.mods,
            &raw mut entry.mods.mods,
        )
    } {
        return ReportTypeBadType(
            info,
            XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
            type_0,
            "map entry",
            "modifier mask",
        );
    }
    if entry.mods.mods & !type_0.mods != 0 {
        log::warn!(
            "[XKB-{:03}] Map entry for modifiers not used by type {}; Using {} instead of {}\n",
            XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
            TypeTxt(info, type_0),
            ModMaskText(
                info.ctx(),
                MOD_BOTH,
                &info.mods,
                entry.mods.mods & type_0.mods,
            ),
            MapEntryTxt(info, &entry)
        );
        entry.mods.mods &= type_0.mods;
    }
    if !unsafe { ExprResolveLevel(info.ctx, value, &raw mut entry.level) } {
        log::error!("[XKB-{:03}] Level specifications in a key type must be integer; Ignoring malformed level specification\n",
            XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32);
        return false;
    }
    entry.preserve.mods = 0_u32;
    AddMapEntry(info, type_0, &entry, true, true)
}
fn AddPreserve(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    mods: u32,
    preserve_mods: u32,
) -> bool {
    // Find matching entry index first to avoid borrow conflicts
    let match_idx = type_0.entries.iter().position(|e| e.mods.mods == mods);
    if let Some(idx) = match_idx {
        let old_preserve = type_0.entries[idx].preserve.mods;
        if old_preserve == 0_u32 {
            type_0.entries[idx].preserve.mods = preserve_mods;
            return true;
        }
        if old_preserve == preserve_mods {
            log::warn!(
                "[XKB-{:03}] Identical definitions for preserve[{}] in {}; Ignored\n",
                XKB_WARNING_DUPLICATE_ENTRY as i32,
                ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods),
                TypeTxt(info, type_0)
            );
            return true;
        }
        log::warn!(
            "[XKB-{:03}] Multiple definitions for preserve[{}] in {}; Using {}, ignoring {}\n",
            XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES as i32,
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods),
            TypeTxt(info, type_0),
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, preserve_mods,),
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, old_preserve,)
        );
        type_0.entries[idx].preserve.mods = preserve_mods;
        return true;
    }
    let new = xkb_key_type_entry {
        level: 0_u32,
        mods: xkb_mods { mods, mask: 0 },
        preserve: xkb_mods {
            mods: preserve_mods,
            mask: 0,
        },
    };
    type_0.entries.push(new);
    true
}
fn SetPreserve(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    if arrayNdx.is_null() {
        return ReportTypeShouldBeArray(info, type_0, "preserve entry");
    }
    let mut mods: u32 = 0_u32;
    if !unsafe { ExprResolveModMask(info.ctx, arrayNdx, MOD_BOTH, &info.mods, &raw mut mods) } {
        return ReportTypeBadType(
            info,
            XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
            type_0,
            "preserve entry",
            "modifier mask",
        );
    }
    if mods & !type_0.mods != 0 {
        let before: String = ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods);
        mods &= type_0.mods;
        let after: String = ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods);
        log::warn!("[XKB-{:03}] Preserve entry for modifiers not used by the {} type; Index {} converted to {}\n",
            XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
            TypeTxt(info, type_0),
            before,
            after);
    }
    let mut preserve_mods: u32 = 0_u32;
    if !unsafe {
        ExprResolveModMask(
            info.ctx,
            value,
            MOD_BOTH,
            &info.mods,
            &raw mut preserve_mods,
        )
    } {
        log::error!("[XKB-{:03}] Preserve value in a key type is not a modifier mask; Ignoring preserve[{}] in type {}\n",
            { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK },
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods),
            TypeTxt(info, type_0));
        return false;
    }
    if preserve_mods & !mods != 0 {
        let before_0: String = ModMaskText(info.ctx(), MOD_BOTH, &info.mods, preserve_mods);
        preserve_mods &= mods;
        let after_0: String = ModMaskText(info.ctx(), MOD_BOTH, &info.mods, preserve_mods);
        log::warn!(
            "[XKB-{:03}] Illegal value for preserve[{}] in type {}; Converted {} to {}\n",
            XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT as i32,
            ModMaskText(info.ctx(), MOD_BOTH, &info.mods, mods),
            TypeTxt(info, type_0),
            before_0,
            after_0
        );
    }
    AddPreserve(info, type_0, mods, preserve_mods)
}
fn AddLevelName(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    level: u32,
    name: u32,
    clobber: bool,
) -> bool {
    let level_idx = level as usize;
    if level >= type_0.level_names.len() as u32 {
        unsafe { vec_resize_zero(&mut type_0.level_names, level_idx.wrapping_add(1)) };
    } else {
        if type_0.level_names[level_idx] == name {
            log::warn!(
                "[XKB-{:03}] Duplicate names for level {} of key type {}; Ignored\n",
                XKB_WARNING_DUPLICATE_ENTRY as i32,
                level.wrapping_add(1_u32),
                TypeTxt(info, type_0)
            );
            return true;
        }
        if type_0.level_names[level_idx] != XKB_ATOM_NONE {
            let old: &str = xkb_atom_text(&info.ctx().atom_table, type_0.level_names[level_idx]);
            let new: &str = xkb_atom_text(&info.ctx().atom_table, name);
            log::warn!(
                "[XKB-{:03}] Multiple names for level {} of key type {}; Using {}, ignoring {}\n",
                XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES as i32,
                level.wrapping_add(1_u32),
                TypeTxt(info, type_0),
                if clobber { new } else { old },
                if clobber { old } else { new }
            );
            if !clobber {
                return true;
            }
        }
    }
    type_0.level_names[level_idx] = name;
    true
}
fn SetLevelName(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    if arrayNdx.is_null() {
        return ReportTypeShouldBeArray(info, type_0, "level name");
    }
    let mut level: u32 = 0_u32;
    if !unsafe { ExprResolveLevel(info.ctx, arrayNdx, &raw mut level) } {
        return ReportTypeBadType(
            info,
            XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
            type_0,
            "level name",
            "integer",
        );
    }
    let mut level_name: u32 = XKB_ATOM_NONE;
    if !unsafe { ExprResolveString(info.ctx, value, &raw mut level_name) } {
        log::error!("[XKB-{:03}] Non-string name for level {} in key type {}; Ignoring illegal level name definition\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            level.wrapping_add(1_u32),
            xkb_atom_text(&info.ctx().atom_table, type_0.name));
        return false;
    }
    AddLevelName(info, type_0, level, level_name, true)
}
fn SetKeyTypeField(
    info: &mut KeyTypesInfo<'_>,
    type_0: &mut KeyTypeInfo,
    field: &str,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
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
    } else if field.eq_ignore_ascii_case("levelname") || field.eq_ignore_ascii_case("level_name") {
        type_field = TYPE_FIELD_LEVEL_NAME;
        ok = SetLevelName(info, type_0, arrayNdx, value);
    } else {
        log::error!(
            "[XKB-{:03}] Unknown field \"{}\" in key type \"{}\"; Definition ignored\n",
            XKB_ERROR_UNKNOWN_FIELD as i32,
            field,
            TypeTxt(info, type_0)
        );
        ok = info.keymap_info.strict & PARSER_NO_UNKNOWN_TYPE_FIELDS == 0;
    }
    type_0.defined = (type_0.defined | type_field as u32) as type_field;
    ok
}
fn HandleKeyTypeBody(
    info: &mut KeyTypesInfo<'_>,
    mut def: *mut VarDef,
    type_0: &mut KeyTypeInfo,
) -> bool {
    unsafe {
        let mut ok: bool = true;
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        while !def.is_null() {
            if !ExprResolveLhs(
                info.ctx,
                (*def).name.raw(),
                &mut elem,
                &mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_empty() {
                if elem.eq_ignore_ascii_case("type") {
                    log::error!("[XKB-{:03}] Support for changing the default type has been removed; Statement \"{}.{}\" ignored.\n",
                        XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT as i32,
                        elem,
                        field);
                } else {
                    log::error!("[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key type statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                        elem,
                        elem,
                        field);
                    ok = false;
                }
            } else if !SetKeyTypeField(info, type_0, field, arrayNdx, (*def).value.raw()) {
                ok = false;
            }
            def = (*def).common.next as *mut VarDef;
        }
        ok
    }
}
fn HandleGlobalVar(info: &mut KeyTypesInfo<'_>, stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        if !ExprResolveLhs(
            info.ctx,
            (*stmt).name.raw(),
            &mut elem,
            &mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("type") {
            log::error!("[XKB-{:03}] Support for changing the default type has been removed; Statement ignored\n",
                XKB_ERROR_WRONG_STATEMENT_TYPE as i32);
            return true;
        } else if !elem.is_empty() {
            log::error!("[XKB-{:03}] Default defined for unknown element \"{}\"; Value for field \"{}.{}\" ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                elem,
                elem,
                field);
            return info.keymap_info.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
        } else if !field.is_empty() {
            log::error!(
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                field
            );
            return info.keymap_info.strict & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS == 0;
        }
        false
    }
}
unsafe fn HandleKeyTypesFile(info: &mut KeyTypesInfo<'_>, file: *mut XkbFile) {
    unsafe {
        let mut ok: bool;
        info.name = {
            let file_ref = &*file;
            if file_ref.name.is_empty() {
                None
            } else {
                Some(file_ref.name.clone())
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
                    let def = &*(stmt as *const KeyTypeDef);
                    let mut type_0: KeyTypeInfo = KeyTypeInfo {
                        defined: 0 as type_field,
                        merge: def.merge,
                        name: def.name,
                        mods: 0_u32,
                        num_levels: 1_u32,
                        entries: Vec::new(),
                        level_names: Vec::new(),
                    };
                    if !HandleKeyTypeBody(info, def.body.raw(), &mut type_0)
                        || !AddKeyType(info, &mut type_0, true)
                    {
                        info.errorCount += 1;
                        ClearKeyTypeInfo(&mut type_0);
                        ok = false;
                    } else {
                        ok = true;
                    }
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef(info.ctx, &mut info.mods, &*(stmt as *const VModDef));
                }
                35 | 36 => {
                    log::error!(
                        "[XKB-{:03}] Unsupported types {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        if (*stmt).type_0 == STMT_UNKNOWN_COMPOUND {
                            "compound"
                        } else {
                            "declaration"
                        },
                        &(*(stmt as *mut UnknownStatement)).name
                    );
                    ok = info.keymap_info.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    log::error!(
                        "[XKB-{:03}] Key type files may not include other declarations; Ignoring {}\n",
                        XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                log::error!(
                    "[XKB-{:03}] Abandoning keytypes file \"{}\"\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    safe_map_name(&*file)
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
fn CopyKeyTypesToKeymap(keymap: &mut xkb_keymap, info: &mut KeyTypesInfo) -> bool {
    let num_types: u32 = if info.types.is_empty() {
        1_u32
    } else {
        info.types.len() as u32
    };
    let mut types_vec: Vec<xkb_key_type> = Vec::with_capacity(num_types as usize);
    if info.types.is_empty() {
        let type_0 = xkb_key_type {
            name: xkb_atom_intern_bytes(&mut keymap.ctx, b"ONE_LEVEL"),
            mods: xkb_mods { mods: 0, mask: 0 },
            required: true,
            num_levels: 1,
            level_names: Vec::new(),
            entries: Vec::new(),
        };
        types_vec.push(type_0);
    } else {
        let canonical_types: [u32; 4] = [
            xkb_atom_intern_bytes(&mut keymap.ctx, b"ONE_LEVEL"),
            xkb_atom_intern_bytes(&mut keymap.ctx, b"TWO_LEVEL"),
            xkb_atom_intern_bytes(&mut keymap.ctx, b"ALPHABETIC"),
            xkb_atom_intern_bytes(&mut keymap.ctx, b"KEYPAD"),
        ];
        for def in info.types.iter_mut() {
            let level_names = std::mem::take(&mut def.level_names);
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
            types_vec.push(xkb_key_type {
                name: def.name,
                mods: xkb_mods {
                    mods: def.mods,
                    mask: 0,
                },
                required,
                num_levels: def.num_levels,
                level_names,
                entries,
            });
        }
    }
    keymap.types_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut keymap.types_section_name);
    keymap.types = types_vec;
    keymap.mods = info.mods;
    true
}
pub unsafe fn CompileKeyTypes(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let ctx = &mut (*(*keymap_info).keymap).ctx;
        let mut info = KeyTypesInfo::new(ctx, &mut *keymap_info);
        InitKeyTypesInfo(&mut info, 0_u32, &(*(*keymap_info).keymap).mods);
        if !file.is_null() {
            HandleKeyTypesFile(&mut info, file);
        }
        if (info.errorCount == 0_i32)
            && CopyKeyTypesToKeymap(&mut *(*keymap_info).keymap, &mut info)
        {
            ClearKeyTypesInfo(&mut info);
            return true;
        }
        ClearKeyTypesInfo(&mut info);
        false
    }
}
use crate::xkb::context::xkb_atom_intern_bytes;
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
