use super::*;

pub use super::super::shared_ast_types::{
    KeyAliasDef, KeycodeDef, LedNameDef, ReportNotArray, ReportShouldBeArray,
};
use super::super::text::ModMaskText;
use super::expr::{ExprResolveInteger, ExprResolveLevel};
pub struct KeyTypesInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub types: Vec<KeyTypeInfo>,
    pub mods: xkb_mod_set,
}
impl Default for KeyTypesInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyTypesInfo {
    pub fn new() -> Self {
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            types: Vec::new(),
            mods: Default::default(),
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
pub type type_field = u32;
pub const TYPE_FIELD_LEVEL_NAME: type_field = 8;
pub const TYPE_FIELD_PRESERVE: type_field = 4;
pub const TYPE_FIELD_MAP: type_field = 2;
pub const TYPE_FIELD_MASK: type_field = 1;
#[inline]
fn MapEntryTxt(
    ki: &xkb_keymap_info<'_>,
    info: &KeyTypesInfo,
    entry: &xkb_key_type_entry,
) -> String {
    ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, entry.mods.mods)
}
#[inline]
fn TypeTxt<'a>(ki: &'a xkb_keymap_info<'_>, type_0: &KeyTypeInfo) -> &'a str {
    xkb_atom_text(&ki.ctx().atom_table, type_0.name)
}
#[inline]
fn ReportTypeShouldBeArray(ki: &xkb_keymap_info<'_>, type_0: &KeyTypeInfo, field: &str) -> bool {
    ReportShouldBeArray("key type", field, TypeTxt(ki, type_0))
}
#[inline]
fn ReportTypeBadType(
    ki: &xkb_keymap_info<'_>,
    code: u32,
    type_0: &KeyTypeInfo,
    field: &str,
    wanted: &str,
) -> bool {
    ReportBadType(code, "key type", field, TypeTxt(ki, type_0), wanted)
}
fn InitKeyTypesInfo(info: &mut KeyTypesInfo, include_depth: u32, mods: &xkb_mod_set) {
    info.name = None;
    info.errorCount = 0;
    info.include_depth = include_depth;
    info.types = Vec::new();
    info.mods = Default::default();
    InitVMods(&mut info.mods, mods, include_depth > 0_u32);
}
fn ClearKeyTypeInfo(type_0: &mut KeyTypeInfo) {
    type_0.entries.clear();
    type_0.level_names.clear();
}
fn ClearKeyTypesInfo(info: &mut KeyTypesInfo) {
    info.name = None;
    for type_0 in info.types.iter_mut() {
        ClearKeyTypeInfo(type_0);
    }
    info.types.clear();
}
fn AddKeyType(
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    new: &mut KeyTypeInfo,
    same_file: bool,
) -> bool {
    let verbosity: i32 = xkb_context_get_log_verbosity(ki.ctx());
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
                    xkb_atom_text(&ki.ctx().atom_table, new.name));
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
                xkb_atom_text(&ki.ctx().atom_table, new.name)
            );
        }
        ClearKeyTypeInfo(new);
        return true;
    }
    info.types.push(new.clone());
    true
}
fn MergeIncludedKeyTypes(
    ki: &mut xkb_keymap_info<'_>,
    into: &mut KeyTypesInfo,
    from: &mut KeyTypesInfo,
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
    if into.types.is_empty() {
        into.types = std::mem::take(&mut from.types);
    } else {
        for i in 0..from.types.len() {
            from.types[i].merge = merge;
            let mut type_clone = from.types[i].clone();
            if !AddKeyType(ki, into, &mut type_clone, false) {
                into.errorCount += 1;
            }
        }
        from.types.clear();
    }
}
fn HandleIncludeKeyTypes(
    ki: &mut xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    include: &mut IncludeStmt,
) -> bool {
    let mut included = KeyTypesInfo::new();
    if ExceedsIncludeMaxDepth(info.include_depth) {
        info.errorCount += 10_i32;
        return false;
    }
    InitKeyTypesInfo(
        &mut included,
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
        let mut next_incl = KeyTypesInfo::new();

        let file: Option<Box<XkbFile>> = ProcessIncludeFile(ki.ctx_mut(), stmt, FILE_TYPE_TYPES);
        let Some(mut file) = file else {
            info.errorCount += 10_i32;
            ClearKeyTypesInfo(&mut included);
            return false;
        };
        InitKeyTypesInfo(
            &mut next_incl,
            info.include_depth.wrapping_add(1_u32),
            &included.mods,
        );
        HandleKeyTypesFile(ki, &mut next_incl, &mut file);
        MergeIncludedKeyTypes(ki, &mut included, &mut next_incl, stmt.merge);
        ClearKeyTypesInfo(&mut next_incl);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    MergeIncludedKeyTypes(ki, info, &mut included, include.merge);
    ClearKeyTypesInfo(&mut included);
    info.errorCount == 0_i32
}
fn SetModifiers(
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
) -> bool {
    let mut mods: u32 = 0_u32;
    if arrayNdx.is_some() {
        log::error!(
            "The modifiers field of a key type is not an array; Illegal array subscript ignored\n"
        );
        return false;
    }
    if !ExprResolveModMask(ki.ctx(), value, MOD_BOTH, &info.mods, &mut mods) {
        log::error!("[XKB-{:03}] Key type mask field must be a modifier mask; Key type definition ignored\n",
            { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK });
        return false;
    }
    if type_0.defined & TYPE_FIELD_MASK != 0 {
        log::warn!(
            "Multiple modifier mask definitions for key type {}; Using {}, ignoring {}\n",
            xkb_atom_text(&ki.ctx().atom_table, type_0.name),
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, type_0.mods),
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods)
        );
        return false;
    }
    type_0.mods = mods;
    true
}
fn AddMapEntry(
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
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
                MapEntryTxt(ki, info, new),
                TypeTxt(ki, type_0),
                (if clobber { new.level } else { old.level }).wrapping_add(1_u32),
                (if clobber { old.level } else { new.level }).wrapping_add(1_u32)
            );
        } else {
            log::warn!(
                "[XKB-{:03}] Multiple occurrences of map[{}]= {} in {}; Ignored\n",
                XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                MapEntryTxt(ki, info, new),
                new.level.wrapping_add(1_u32),
                TypeTxt(ki, type_0)
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
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
) -> bool {
    let mut entry: xkb_key_type_entry = xkb_key_type_entry {
        level: 0,
        mods: xkb_mods { mods: 0, mask: 0 },
        preserve: xkb_mods { mods: 0, mask: 0 },
    };
    if arrayNdx.is_none() {
        return ReportTypeShouldBeArray(ki, type_0, "map entry");
    }
    if !ExprResolveModMask(
        ki.ctx(),
        arrayNdx.unwrap(),
        MOD_BOTH,
        &info.mods,
        &mut entry.mods.mods,
    ) {
        return ReportTypeBadType(
            ki,
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
            TypeTxt(ki, type_0),
            ModMaskText(
                ki.ctx(),
                MOD_BOTH,
                &info.mods,
                entry.mods.mods & type_0.mods,
            ),
            MapEntryTxt(ki, info, &entry)
        );
        entry.mods.mods &= type_0.mods;
    }
    if !ExprResolveLevel(ki.ctx(), value, &mut entry.level) {
        log::error!("[XKB-{:03}] Level specifications in a key type must be integer; Ignoring malformed level specification\n",
            XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32);
        return false;
    }
    entry.preserve.mods = 0_u32;
    AddMapEntry(ki, info, type_0, &entry, true, true)
}
fn AddPreserve(
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
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
                ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods),
                TypeTxt(ki, type_0)
            );
            return true;
        }
        log::warn!(
            "[XKB-{:03}] Multiple definitions for preserve[{}] in {}; Using {}, ignoring {}\n",
            XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES as i32,
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods),
            TypeTxt(ki, type_0),
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, preserve_mods,),
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, old_preserve,)
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
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
) -> bool {
    if arrayNdx.is_none() {
        return ReportTypeShouldBeArray(ki, type_0, "preserve entry");
    }
    let mut mods: u32 = 0_u32;
    if !ExprResolveModMask(ki.ctx(), arrayNdx.unwrap(), MOD_BOTH, &info.mods, &mut mods) {
        return ReportTypeBadType(
            ki,
            XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
            type_0,
            "preserve entry",
            "modifier mask",
        );
    }
    if mods & !type_0.mods != 0 {
        let before: String = ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods);
        mods &= type_0.mods;
        let after: String = ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods);
        log::warn!("[XKB-{:03}] Preserve entry for modifiers not used by the {} type; Index {} converted to {}\n",
            XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
            TypeTxt(ki, type_0),
            before,
            after);
    }
    let mut preserve_mods: u32 = 0_u32;
    if !ExprResolveModMask(ki.ctx(), value, MOD_BOTH, &info.mods, &mut preserve_mods) {
        log::error!("[XKB-{:03}] Preserve value in a key type is not a modifier mask; Ignoring preserve[{}] in type {}\n",
            { XKB_ERROR_UNSUPPORTED_MODIFIER_MASK },
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods),
            TypeTxt(ki, type_0));
        return false;
    }
    if preserve_mods & !mods != 0 {
        let before_0: String = ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, preserve_mods);
        preserve_mods &= mods;
        let after_0: String = ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, preserve_mods);
        log::warn!(
            "[XKB-{:03}] Illegal value for preserve[{}] in type {}; Converted {} to {}\n",
            XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT as i32,
            ModMaskText(ki.ctx(), MOD_BOTH, &info.mods, mods),
            TypeTxt(ki, type_0),
            before_0,
            after_0
        );
    }
    AddPreserve(ki, info, type_0, mods, preserve_mods)
}
fn AddLevelName(
    ki: &xkb_keymap_info<'_>,
    _info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    level: u32,
    name: u32,
    clobber: bool,
) -> bool {
    let level_idx = level as usize;
    if level >= type_0.level_names.len() as u32 {
        vec_resize_zero(&mut type_0.level_names, level_idx.wrapping_add(1));
    } else {
        if type_0.level_names[level_idx] == name {
            log::warn!(
                "[XKB-{:03}] Duplicate names for level {} of key type {}; Ignored\n",
                XKB_WARNING_DUPLICATE_ENTRY as i32,
                level.wrapping_add(1_u32),
                TypeTxt(ki, type_0)
            );
            return true;
        }
        if type_0.level_names[level_idx] != XKB_ATOM_NONE {
            let old: &str = xkb_atom_text(&ki.ctx().atom_table, type_0.level_names[level_idx]);
            let new: &str = xkb_atom_text(&ki.ctx().atom_table, name);
            log::warn!(
                "[XKB-{:03}] Multiple names for level {} of key type {}; Using {}, ignoring {}\n",
                XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES as i32,
                level.wrapping_add(1_u32),
                TypeTxt(ki, type_0),
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
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
) -> bool {
    if arrayNdx.is_none() {
        return ReportTypeShouldBeArray(ki, type_0, "level name");
    }
    let mut level: u32 = 0_u32;
    if !ExprResolveLevel(ki.ctx(), arrayNdx.unwrap(), &mut level) {
        return ReportTypeBadType(
            ki,
            XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
            type_0,
            "level name",
            "integer",
        );
    }
    let mut level_name: u32 = XKB_ATOM_NONE;
    if !ExprResolveString(ki.ctx(), value, &mut level_name) {
        log::error!("[XKB-{:03}] Non-string name for level {} in key type {}; Ignoring illegal level name definition\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            level.wrapping_add(1_u32),
            xkb_atom_text(&ki.ctx().atom_table, type_0.name));
        return false;
    }
    AddLevelName(ki, info, type_0, level, level_name, true)
}
fn SetKeyTypeField(
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    value: &ExprDef,
) -> bool {
    let ok: bool;
    let mut type_field: type_field = 0 as type_field;
    if field.eq_ignore_ascii_case("modifiers") {
        type_field = TYPE_FIELD_MASK;
        ok = SetModifiers(ki, info, type_0, arrayNdx, value);
    } else if field.eq_ignore_ascii_case("map") {
        type_field = TYPE_FIELD_MAP;
        ok = SetMapEntry(ki, info, type_0, arrayNdx, value);
    } else if field.eq_ignore_ascii_case("preserve") {
        type_field = TYPE_FIELD_PRESERVE;
        ok = SetPreserve(ki, info, type_0, arrayNdx, value);
    } else if field.eq_ignore_ascii_case("levelname") || field.eq_ignore_ascii_case("level_name") {
        type_field = TYPE_FIELD_LEVEL_NAME;
        ok = SetLevelName(ki, info, type_0, arrayNdx, value);
    } else {
        log::error!(
            "[XKB-{:03}] Unknown field \"{}\" in key type \"{}\"; Definition ignored\n",
            XKB_ERROR_UNKNOWN_FIELD as i32,
            field,
            TypeTxt(ki, type_0)
        );
        ok = ki.strict & PARSER_NO_UNKNOWN_TYPE_FIELDS == 0;
    }
    type_0.defined = (type_0.defined | type_field as u32) as type_field;
    ok
}
fn HandleKeyTypeBody(
    ki: &xkb_keymap_info<'_>,
    info: &mut KeyTypesInfo,
    defs: &[VarDef],
    type_0: &mut KeyTypeInfo,
) -> bool {
    let mut ok: bool = true;
    for def in defs {
        let mut elem_atom: u32 = 0;
        let mut field_atom: u32 = 0;
        let mut arrayNdx: Option<&ExprDef> = None;
        let name_ref = def.name.as_deref().unwrap();
        if !ExprResolveLhs(name_ref, &mut elem_atom, &mut field_atom, &mut arrayNdx) {
            ok = false;
        } else {
            let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom).to_owned();
            let field = xkb_atom_text(&ki.ctx().atom_table, field_atom).to_owned();
            if !elem.is_empty() {
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
            } else {
                let value_ref = def.value.as_deref().unwrap();
                if !SetKeyTypeField(ki, info, type_0, &field, arrayNdx, value_ref) {
                    ok = false;
                }
            }
        }
    }
    ok
}
fn HandleGlobalVar(ki: &xkb_keymap_info<'_>, _info: &mut KeyTypesInfo, stmt: &VarDef) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut arrayNdx: Option<&ExprDef> = None;
    let name_ref = stmt.name.as_deref().unwrap();
    if !ExprResolveLhs(name_ref, &mut elem_atom, &mut field_atom, &mut arrayNdx) {
        return false;
    }
    let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom);
    let field = xkb_atom_text(&ki.ctx().atom_table, field_atom);
    if !elem.is_empty() && elem.eq_ignore_ascii_case("type") {
        log::error!("[XKB-{:03}] Support for changing the default type has been removed; Statement ignored\n",
            XKB_ERROR_WRONG_STATEMENT_TYPE as i32);
        return true;
    } else if !elem.is_empty() {
        log::error!("[XKB-{:03}] Default defined for unknown element \"{}\"; Value for field \"{}.{}\" ignored\n",
            XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
            elem,
            elem,
            field);
        return ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
    } else if !field.is_empty() {
        log::error!(
            "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
            XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
            field
        );
        return ki.strict & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS == 0;
    }
    false
}
fn HandleKeyTypesFile(ki: &mut xkb_keymap_info<'_>, info: &mut KeyTypesInfo, file: &mut XkbFile) {
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
                    ok = HandleIncludeKeyTypes(ki, info, incl);
                }
                Statement::KeyType(def) => {
                    let mut type_0: KeyTypeInfo = KeyTypeInfo {
                        defined: 0 as type_field,
                        merge: def.merge,
                        name: def.name,
                        mods: 0_u32,
                        num_levels: 1_u32,
                        entries: Vec::new(),
                        level_names: Vec::new(),
                    };
                    if !HandleKeyTypeBody(ki, info, &def.body, &mut type_0)
                        || !AddKeyType(ki, info, &mut type_0, true)
                    {
                        info.errorCount += 1;
                        ClearKeyTypeInfo(&mut type_0);
                        ok = false;
                    } else {
                        ok = true;
                    }
                }
                Statement::Var(var) => {
                    ok = HandleGlobalVar(ki, info, var);
                }
                Statement::VMod(vmod) => {
                    ok = HandleVModDef(ki.ctx_mut(), &mut info.mods, vmod);
                }
                Statement::Unknown(unk) => {
                    log::error!(
                        "[XKB-{:03}] Unsupported types {} statement \"{}\"; Ignoring\n",
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
                        "[XKB-{:03}] Key type files may not include other declarations; Ignoring {}\n",
                        XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
                        stmt_type_to_string(stmt.stmt_type()),
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
                    safe_map_name(file)
                );
                break;
            }
        }
    }
}
fn CopyKeyTypesToKeymap(ki: &mut xkb_keymap_info<'_>, info: &mut KeyTypesInfo) -> bool {
    let keymap = ki.keymap_mut();
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
pub fn CompileKeyTypes(file: Option<&mut XkbFile>, keymap_info: &mut xkb_keymap_info<'_>) -> bool {
    let mods = keymap_info.keymap_ref().mods;
    let mut info = KeyTypesInfo::new();
    InitKeyTypesInfo(&mut info, 0_u32, &mods);
    if let Some(file) = file {
        HandleKeyTypesFile(keymap_info, &mut info, file);
    }
    if (info.errorCount == 0_i32) && CopyKeyTypesToKeymap(keymap_info, &mut info) {
        ClearKeyTypesInfo(&mut info);
        return true;
    }
    ClearKeyTypesInfo(&mut info);
    false
}
use super::super::context::xkb_atom_intern_bytes;
use super::super::context::xkb_context_get_log_verbosity;
use super::super::shared_types::*;

// ── Virtual modifier functions (migrated from vmod.rs) ──

pub fn InitVMods(info: &mut xkb_mod_set, mods: &xkb_mod_set, reset: bool) {
    *info = *mods;
    if !reset {
        return;
    }
    for vmod in 0..info.num_mods as usize {
        info.mods[vmod].mapping = 0_u32;
    }
    info.explicit_vmods = 0_u32;
}
pub fn MergeModSets(
    ctx: &mut xkb_context,
    into: &mut xkb_mod_set,
    from: &xkb_mod_set,
    merge: merge_mode,
) {
    let clobber: bool = merge != MERGE_AUGMENT;
    for vmod in 0..from.num_mods as usize {
        let mod_0 = &from.mods[vmod];
        let mask: u32 = 1_u32 << vmod;
        if mod_0.type_0 != MOD_VIRT {
        } else if into.mods[vmod].type_0 == 0_u32 {
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
            let ignore: u32 = if clobber {
                into.mods[vmod].mapping
            } else {
                mod_0.mapping
            };
            log::warn!(
                "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                xkb_atom_text(&ctx.atom_table, mod_0.name),
                ModMaskText(ctx, MOD_REAL, from, use_0),
                ModMaskText(ctx, MOD_REAL, from, ignore)
            );
            into.mods[vmod].mapping = use_0;
        }
    }
    into.num_mods = from.num_mods;
}
pub fn HandleVModDef(ctx: &mut xkb_context, mods: &mut xkb_mod_set, stmt: &VModDef) -> bool {
    let mut mapping: u32 = 0_u32;
    if stmt.value.is_some() {
        let value_ref = stmt.value.as_deref().unwrap();
        if !ExprResolveModMask(ctx, value_ref, MOD_REAL, mods, &mut mapping) {
            log::error!(
                "Declaration of {} ignored\n",
                xkb_atom_text(&ctx.atom_table, stmt.name)
            );
            return false;
        }
    }
    for vmod in 0..mods.num_mods as usize {
        if mods.mods[vmod].name == stmt.name {
            if mods.mods[vmod].type_0 != MOD_VIRT {
                log::error!("Can't add a virtual modifier named \"{}\"; there is already a non-virtual modifier with this name! Ignored\n",
                    xkb_atom_text(&ctx.atom_table, mods.mods[vmod].name));
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
                let ignore: u32 = if clobber {
                    mods.mods[vmod].mapping
                } else {
                    mapping
                };
                log::warn!(
                    "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                    xkb_atom_text(&ctx.atom_table, stmt.name),
                    ModMaskText(ctx, MOD_REAL, mods, use_0),
                    ModMaskText(ctx, MOD_REAL, mods, ignore)
                );
                mods.mods[vmod].mapping = use_0;
            }
            mods.explicit_vmods |= mask;
            return true;
        }
    }
    if mods.num_mods >= XKB_MAX_MODS {
        log::error!(
            "Cannot define virtual modifier {}: too many modifiers defined (maximum {})\n",
            xkb_atom_text(&ctx.atom_table, stmt.name),
            (std::mem::size_of::<u32>()).wrapping_mul(8_usize) as u32
        );
        return false;
    }
    mods.mods[mods.num_mods as usize].name = stmt.name;
    mods.mods[mods.num_mods as usize].type_0 = MOD_VIRT;
    mods.mods[mods.num_mods as usize].mapping = mapping;
    if stmt.value.is_some() {
        let mask_0: u32 = 1_u32 << mods.num_mods;
        mods.explicit_vmods |= mask_0;
    }
    mods.num_mods = mods.num_mods.wrapping_add(1);
    true
}
pub struct KeyNamesInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub keycodes: KeycodeStore,
    pub led_names: [LedNameInfo; 32],
    pub num_led_names: u32,
}
impl Default for KeyNamesInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyNamesInfo {
    pub fn new() -> Self {
        Self {
            name: None,
            errorCount: 0,
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
pub struct LedNameInfo {
    pub merge: merge_mode,
    pub name: u32,
}
#[derive(Clone)]
pub struct KeycodeStore {
    pub min: u32,
    pub low: Vec<u32>,
    pub high: Vec<HighKeycodeEntry>,
    pub names: Vec<KeycodeMatch>,
}
#[derive(Copy, Clone, Default)]
pub struct HighKeycodeEntry {
    pub keycode: u32,
    pub name: u32,
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
        vec_resize_zero(&mut store.names, (name as usize).wrapping_add(1));
    }
    store.names[name as usize] = match_0;
}
fn keycode_store_insert_key(store: &mut KeycodeStore, kc: u32, name: u32) -> bool {
    if name >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (name as usize).wrapping_add(1));
    }
    if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as u32 {
        if kc >= store.low.len() as u32 {
            vec_resize_zero(&mut store.low, (kc as usize).wrapping_add(1));
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
            let mut lower: u32 = 0_u32;
            let mut upper: u32 = idx;
            while lower < upper {
                let mid: u32 = lower.wrapping_add(
                    upper
                        .wrapping_sub(1_u32)
                        .wrapping_sub(lower)
                        .wrapping_div(2_u32),
                );
                let entry: &HighKeycodeEntry = &store.high[mid as usize];
                if entry.keycode < kc {
                    lower = mid.wrapping_add(1_u32);
                } else if entry.keycode > kc {
                    upper = mid;
                }
            }
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
        vec_resize_zero(&mut store.names, (alias as usize).wrapping_add(1));
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
                while idx > 0_u32 {
                    if store.low[(idx.wrapping_sub(1_u32)) as usize] != XKB_ATOM_NONE {
                        store.low.truncate(idx as usize);
                        break;
                    } else {
                        idx = idx.wrapping_sub(1);
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
            if entry.found
                && !entry.is_alias
                && !entry.low
                && entry.index as i32 > match_0.index as i32
            {
                entry.index -= 1_u32;
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
                kc = kc.wrapping_add(1);
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
    let mut lower: u32 = 0_u32;
    let mut upper: u32 = store.high.len() as u32;
    while lower < upper {
        let mid: u32 = lower.wrapping_add(
            upper
                .wrapping_sub(1_u32)
                .wrapping_sub(lower)
                .wrapping_div(2_u32),
        );
        let entry: &HighKeycodeEntry = &store.high[mid as usize];
        if entry.keycode < kc {
            lower = mid.wrapping_add(1_u32);
        } else if entry.keycode > kc {
            upper = mid;
        } else {
            return KeycodeMatch {
                found: true,
                low: false,
                is_alias: false,
                index: mid,
            };
        }
    }
    KeycodeMatch {
        found: false,
        low: false,
        is_alias: false,
        index: 0,
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
fn AddLedName(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    _same_file: bool,
    new: &LedNameInfo,
    new_idx: u32,
    report: bool,
) -> bool {
    let replace: bool = new.merge != MERGE_AUGMENT;
    // FindLedByName inlined
    let mut found_old: Option<u32> = None;
    {
        let mut idx: u32 = 0_u32;
        while idx < info.num_led_names {
            if info.led_names[idx as usize].name == new.name {
                found_old = Some(idx);
                break;
            }
            idx = idx.wrapping_add(1);
        }
    }
    if let Some(old_idx) = found_old {
        if old_idx == new_idx {
            if report {
                log::warn!(
                    "Multiple indicators named \"{}\"; Identical definitions ignored\n",
                    xkb_atom_text(&ki.ctx().atom_table, new.name)
                );
            }
            return true;
        }
        if report {
            let use_0: u32 = if replace as i32 != 0 {
                new_idx.wrapping_add(1_u32)
            } else {
                old_idx.wrapping_add(1_u32)
            };
            let ignore: u32 = if replace as i32 != 0 {
                old_idx.wrapping_add(1_u32)
            } else {
                new_idx.wrapping_add(1_u32)
            };
            log::warn!(
                "Multiple indicators named {}; Using {}, ignoring {}\n",
                xkb_atom_text(&ki.ctx().atom_table, new.name),
                use_0,
                ignore
            );
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
        if report {
            let use_1: u32 = if replace as i32 != 0 {
                new.name
            } else {
                info.led_names[new_idx as usize].name
            };
            let ignore_0: u32 = if replace as i32 != 0 {
                info.led_names[new_idx as usize].name
            } else {
                new.name
            };
            log::warn!(
                "Multiple names for indicator {}; Using {}, ignoring {}\n",
                new_idx.wrapping_add(1_u32),
                xkb_atom_text(&ki.ctx().atom_table, use_1),
                xkb_atom_text(&ki.ctx().atom_table, ignore_0)
            );
        }
        if replace {
            info.led_names[new_idx as usize] = *new;
        }
        return true;
    }
    info.led_names[new_idx as usize] = *new;
    true
}
fn ClearKeyNamesInfo(info: &mut KeyNamesInfo) {
    info.name = None;
    let store = &mut info.keycodes;
    store.high.clear();
    store.names.clear();
}
fn InitKeyNamesInfo(info: &mut KeyNamesInfo, include_depth: u32) {
    info.name = None;
    info.errorCount = 0;
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
fn AddKeyName(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    kc: u32,
    name: u32,
    merge: merge_mode,
    report: bool,
) -> bool {
    let match_name: KeycodeMatch = keycode_store_lookup_name(&info.keycodes, name);
    if match_name.found {
        let clobber: bool = merge != MERGE_AUGMENT;
        if match_name.is_alias {
            if report {
                log::warn!("[XKB-{:03}] Key name <{}> already assigned to an alias; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        xkb_atom_text(&ki.ctx().atom_table, name),
                        if clobber { "key" } else { "alias" },
                        if clobber { "alias" } else { "key" });
            }
            if clobber {
                keycode_store_delete_name(&mut info.keycodes, name);
                // dead store removed: match_name.found = false;
            } else {
                return true;
            }
        } else {
            let old_kc: u32 = {
                if !match_name.found || match_name.is_alias as i32 != 0 {
                    XKB_KEYCODE_INVALID
                } else if match_name.low {
                    match_name.index
                } else {
                    info.keycodes.high[match_name.index as usize].keycode
                }
            };
            if old_kc != kc {
                if report {
                    let use_0: u32 = if clobber as i32 != 0 { kc } else { old_kc };
                    let ignore: u32 = if clobber as i32 != 0 { old_kc } else { kc };
                    log::warn!("[XKB-{:03}] Key name <{}> assigned to multiple keys; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        xkb_atom_text(&ki.ctx().atom_table, name),
                            use_0,
                            ignore);
                }
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
        if !match_kc.found || match_kc.is_alias as i32 != 0 {
            XKB_ATOM_NONE
        } else if match_kc.low {
            info.keycodes.low[match_kc.index as usize]
        } else {
            info.keycodes.high[match_kc.index as usize].name
        }
    };
    if old_name != XKB_ATOM_NONE {
        if old_name == name {
            if report {
                log::warn!("Multiple identical key name definitions; Later occurrences of \"<{}> = {}\" ignored\n",
                        xkb_atom_text(&ki.ctx().atom_table, old_name),
                        kc);
            }
            return true;
        }
        let clobber_0: bool = merge != MERGE_AUGMENT;
        if report {
            let kname: &str = xkb_atom_text(&ki.ctx().atom_table, name);
            let old_kname: &str = xkb_atom_text(&ki.ctx().atom_table, old_name);
            let use_1: &str = if clobber_0 { kname } else { old_kname };
            let ignore_0: &str = if clobber_0 { old_kname } else { kname };
            log::warn!(
                "Multiple names for keycode {}; Using <{}>, ignoring <{}>\n",
                kc,
                use_1,
                ignore_0
            );
        }
        if clobber_0 {
            keycode_store_delete_name(&mut info.keycodes, old_name);
            keycode_store_update_key(&mut info.keycodes, match_kc, name);
        }
    } else if !keycode_store_insert_key(&mut info.keycodes, kc, name) {
        log::error!(
            "[XKB-{:03}] Cannot add keycode\n",
            XKB_ERROR_ALLOCATION_ERROR as i32
        );
        return false;
    }
    true
}
fn MergeKeycodeStores(
    into: &mut KeyNamesInfo,
    from: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    merge: merge_mode,
    report: bool,
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
            if (name != XKB_ATOM_NONE) && !AddKeyName(into, ki, kc, name, merge, report) {
                into.errorCount += 1;
            }
            kc = kc.wrapping_add(1);
        }
        for entry in from.keycodes.high.iter() {
            if !AddKeyName(into, ki, entry.keycode, entry.name, merge, report) {
                into.errorCount += 1;
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
                        if !HandleAliasDef(into, ki, &def, report) {
                            into.errorCount += 1;
                        }
                    }
                }
            }
        }
    };
}
fn MergeIncludedKeycodes(
    into: &mut KeyNamesInfo,
    from: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    merge: merge_mode,
    report: bool,
) {
    if from.errorCount > 0_i32 {
        into.errorCount += from.errorCount;
        return;
    }
    if into.name.is_none() {
        into.name = from.name.take();
    }
    MergeKeycodeStores(into, from, ki, merge, report);
    if into.num_led_names == 0_u32 {
        into.led_names[..from.num_led_names as usize]
            .copy_from_slice(&from.led_names[..from.num_led_names as usize]);
        into.num_led_names = from.num_led_names;
        from.num_led_names = 0_u32;
    } else {
        let mut idx: u32 = 0_u32;
        while idx < from.num_led_names {
            let ledi = from.led_names[idx as usize];
            if ledi.name != XKB_ATOM_NONE {
                let mut ledi = ledi;
                ledi.merge = merge;
                if !AddLedName(into, ki, false, &ledi, idx, report) {
                    into.errorCount += 1;
                }
            }
            idx = idx.wrapping_add(1);
        }
    };
}
fn HandleIncludeKeycodes(
    info: &mut KeyNamesInfo,
    include: &mut IncludeStmt,
    ki: &mut xkb_keymap_info<'_>,
    report: bool,
) -> bool {
    let mut included = KeyNamesInfo::new();
    if ExceedsIncludeMaxDepth(info.include_depth) {
        info.errorCount += 10_i32;
        return false;
    }
    InitKeyNamesInfo(&mut included, 0_u32);
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(std::mem::take(&mut include.stmt))
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = KeyNamesInfo::new();

        let file: Option<Box<XkbFile>> = ProcessIncludeFile(ki.ctx_mut(), stmt, FILE_TYPE_KEYCODES);
        let Some(mut file) = file else {
            info.errorCount += 10_i32;
            ClearKeyNamesInfo(&mut included);
            return false;
        };
        InitKeyNamesInfo(&mut next_incl, info.include_depth.wrapping_add(1_u32));
        HandleKeycodesFile(&mut next_incl, &mut file, ki);
        MergeIncludedKeycodes(&mut included, &mut next_incl, ki, stmt.merge, report);
        ClearKeyNamesInfo(&mut next_incl);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    MergeIncludedKeycodes(info, &mut included, ki, include.merge, report);
    ClearKeyNamesInfo(&mut included);
    info.errorCount == 0_i32
}
fn HandleKeycodeDef(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    stmt: &KeycodeDef,
    report: bool,
) -> bool {
    if stmt.value < 0_i64 || stmt.value > XKB_KEYCODE_MAX as i64 {
        log::error!(
            "Illegal keycode {}: must be between 0..{}; Key ignored\n",
            stmt.value,
            0xffffffff_u32.wrapping_sub(1_u32)
        );
        return false;
    }
    AddKeyName(info, ki, stmt.value as u32, stmt.name, stmt.merge, report)
}
fn HandleAliasDef(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    def: &KeyAliasDef,
    report: bool,
) -> bool {
    let match_name: KeycodeMatch =
        keycode_store_lookup_name(&info.keycodes, def.alias) as KeycodeMatch;
    if match_name.found {
        let clobber: bool = def.merge != MERGE_AUGMENT;
        if match_name.is_alias {
            if def.real == match_name.index {
                if report {
                    log::warn!("[XKB-{:03}] Alias of <{}> for <{}> declared more than once; First definition ignored\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                         xkb_atom_text(&ki.ctx().atom_table, def.alias),
                         xkb_atom_text(&ki.ctx().atom_table, def.real));
                }
            } else {
                let use_0: u32 = if clobber as i32 != 0 {
                    def.real
                } else {
                    match_name.index
                };
                let ignore: u32 = if clobber as i32 != 0 {
                    match_name.index
                } else {
                    def.real
                };
                if report {
                    log::warn!("[XKB-{:03}] Multiple definitions for alias <{}>; Using <{}>, ignoring <{}>\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                         xkb_atom_text(&ki.ctx().atom_table, def.alias),
                        xkb_atom_text(&ki.ctx().atom_table, use_0),
                        xkb_atom_text(&ki.ctx().atom_table, ignore));
                }
                {
                    info.keycodes.names[def.alias as usize].index = use_0;
                }
            }
            return true;
        } else {
            if report {
                log::warn!("[XKB-{:03}] Alias name <{}> already assigned to a real key; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                    xkb_atom_text(&ki.ctx().atom_table, def.alias),
                    if clobber { "alias" } else { "key" },
                    if clobber { "key" } else { "alias" });
            }
            if clobber {
                keycode_store_delete_key(&mut info.keycodes, match_name);
            } else {
                return true;
            }
        }
    }
    keycode_store_insert_alias(&mut info.keycodes, def.alias, def.real)
}
fn HandleKeyNameVar(_info: &mut KeyNamesInfo, ki: &mut xkb_keymap_info<'_>, stmt: &VarDef) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut arrayNdx: Option<&ExprDef> = None;
    let name_ref = stmt.name.as_deref().unwrap();
    if !ExprResolveLhs(name_ref, &mut elem_atom, &mut field_atom, &mut arrayNdx) {
        return false;
    }
    let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom);
    let field = xkb_atom_text(&ki.ctx().atom_table, field_atom);
    if !elem.is_empty() {
        log::error!("[XKB-{:03}] Cannot set global defaults for \"{}\" element; Assignment to \"{}.{}\" ignored\n",
            XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
            elem,
            elem,
            field);
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if !field.eq_ignore_ascii_case("minimum") && !field.eq_ignore_ascii_case("maximum") {
        log::error!(
            "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
            XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
            field
        );
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if arrayNdx.is_some() {
        ReportNotArray("keycodes", field, "defaults");
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    let mut val: i64 = 0_i64;
    let value_ref = stmt.value.as_deref().unwrap();
    if !ExprResolveInteger(ki.ctx(), value_ref, &mut val) || val < 0_i64 || val > u32::MAX as i64 {
        ReportBadType(
            XKB_ERROR_WRONG_FIELD_TYPE,
            "keycodes",
            field,
            "defaults",
            "integer 0..0xfffffffe",
        );
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    true
}
fn HandleLedNameDef(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    def: &LedNameDef,
    report: bool,
) -> bool {
    if def.ndx < 1_i64 || def.ndx > XKB_MAX_LEDS as i64 {
        info.errorCount += 1;
        log::error!(
            "Illegal indicator index ({}) specified; must be between 1 .. {}; Ignored\n",
            def.ndx,
            (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32
        );
        return false;
    }
    let mut name: u32 = XKB_ATOM_NONE;
    let name_expr = def.name.as_deref().unwrap();
    if !ExprResolveString(ki.ctx(), name_expr, &mut name) {
        let mut buf: [u8; 20] = [0; 20];
        let buf_len = {
            let mut w = super::super::utils::LogBuf::new(&mut buf[..19]);
            let _ = core::fmt::Write::write_fmt(&mut w, format_args!("{}", def.ndx));
            w.pos
        };
        info.errorCount += 1;
        return ReportBadType(
            XKB_ERROR_WRONG_FIELD_TYPE,
            "indicator",
            "name",
            std::str::from_utf8(&buf[..buf_len]).unwrap_or("?"),
            "string",
        );
    }
    let ledi: LedNameInfo = LedNameInfo {
        merge: def.merge,
        name,
    };
    AddLedName(
        info,
        ki,
        true,
        &ledi,
        (def.ndx as u32).wrapping_sub(1_u32),
        report,
    )
}
fn HandleKeycodesFile(info: &mut KeyNamesInfo, file: &mut XkbFile, ki: &mut xkb_keymap_info<'_>) {
    {
        let mut ok: bool;
        let verbosity: i32 = xkb_context_get_log_verbosity(ki.ctx());
        let report_same_file: bool = verbosity > 0_i32;
        let report_include: bool = verbosity > 2_i32;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = HandleIncludeKeycodes(info, incl, ki, report_include);
                }
                Statement::Keycode(kc) => {
                    ok = HandleKeycodeDef(info, ki, kc, report_same_file);
                }
                Statement::KeyAlias(ka) => {
                    ok = HandleAliasDef(info, ki, ka, report_same_file);
                }
                Statement::Var(var) => {
                    ok = HandleKeyNameVar(info, ki, var);
                }
                Statement::LedName(ln) => {
                    ok = HandleLedNameDef(info, ki, ln, report_same_file);
                }
                Statement::Unknown(unk) => {
                    log::error!(
                        "[XKB-{:03}] Unsupported keycodes {} statement \"{}\"; Ignoring\n",
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
                        "Keycode files may define key and indicator names only; Ignoring {}\n",
                        stmt_type_to_string(stmt.stmt_type())
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                log::error!("Abandoning keycodes file \"{}\"\n", safe_map_name(file));
                break;
            }
        }
    }
}
fn CopyKeyNamesToKeymap(keymap: &mut xkb_keymap, keycodes: &KeycodeStore) -> bool {
    if keycodes.low.is_empty() && keycodes.high.is_empty() {
        keymap.min_key_code = 8_u32;
        keymap.max_key_code = 255_u32;
        keymap.num_keys_low = keymap.max_key_code.wrapping_add(1_u32);
        keymap.num_keys = keymap.num_keys_low;
    } else {
        keymap.min_key_code = keycodes.min;
        keymap.max_key_code = if keycodes.high.is_empty() {
            (keycodes.low.len() as u32).wrapping_sub(1_u32)
        } else {
            (&keycodes.high)[keycodes.high.len().wrapping_sub(1)].keycode
        };
        keymap.num_keys_low = keycodes.low.len() as u32;
        keymap.num_keys = keymap.num_keys_low.wrapping_add(keycodes.high.len() as u32);
    }
    let mut keys: Vec<xkb_key> = (0..keymap.num_keys as usize)
        .map(|_| xkb_key::default())
        .collect();
    let mut kc: u32 = keymap.min_key_code;
    while kc < keymap.num_keys_low {
        keys[kc as usize].keycode = kc;
        kc = kc.wrapping_add(1);
    }
    let mut kc_0: u32 = keycodes.min;
    while kc_0 < keycodes.low.len() as u32 {
        keys[kc_0 as usize].name = (&keycodes.low)[kc_0 as usize];
        kc_0 = kc_0.wrapping_add(1);
    }
    let mut idx: u32 = keymap.num_keys_low;
    for entry in keycodes.high.iter() {
        keys[idx as usize].keycode = entry.keycode;
        keys[idx as usize].name = entry.name;
        idx = idx.wrapping_add(1);
    }
    keymap.keys = keys;
    true
}
fn CopyKeycodeNameLUT(keymap: &mut xkb_keymap, keycodes: &mut KeycodeStore) -> bool {
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
                            log::warn!("[XKB-{:03}] Attempt to alias <{}> to non-existent key <{}>; Ignored\n",
                                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                xkb_atom_text(&keymap.ctx.atom_table, name),
                                xkb_atom_text(
                                    &keymap.ctx.atom_table,
                                    entry.index
                                ));
                            keycodes.names[name as usize].found = false;
                        } else if match_real.is_alias {
                            log::warn!(
                                "[XKB-{:03}] Attempt to alias <{}> to alias <{}>; Ignored\n",
                                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                xkb_atom_text(&keymap.ctx.atom_table, name),
                                xkb_atom_text(&keymap.ctx.atom_table, entry.index)
                            );
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
        // Move the Vec directly to keymap
        keymap.key_names = std::mem::take(&mut keycodes.names);
    } else {
        keymap.key_names = Vec::new();
    }
    true
}
fn CopyLedNamesToKeymap(
    keymap: &mut xkb_keymap,
    led_names: &[LedNameInfo; 32],
    num_led_names: u32,
) -> bool {
    keymap.num_leds = num_led_names;
    let mut idx: u32 = 0_u32;
    while idx < num_led_names {
        let ledi = &led_names[idx as usize];
        if ledi.name != XKB_ATOM_NONE {
            keymap.leds[idx as usize].name = ledi.name;
        }
        idx = idx.wrapping_add(1);
    }
    true
}
fn CopyKeyNamesInfoToKeymap(info: &mut KeyNamesInfo, ki: &mut xkb_keymap_info<'_>) -> bool {
    let keymap = ki.keymap_mut();
    if !CopyKeyNamesToKeymap(keymap, &info.keycodes)
        || !CopyKeycodeNameLUT(keymap, &mut info.keycodes)
        || !CopyLedNamesToKeymap(keymap, &info.led_names, info.num_led_names)
    {
        return false;
    }
    if keymap.num_keys == 0 || keymap.min_key_code > 0_u32 {
        keymap.redirect_key_auto = 0_u32;
    } else {
        let mut keycode: u32 = XKB_KEYCODE_INVALID.wrapping_sub(1_u32);
        let mut k: u32 = keymap.num_keys;
        loop {
            let c2rust_fresh0 = k;
            k = k.wrapping_sub(1);
            if c2rust_fresh0 <= keymap.num_keys_low {
                break;
            }
            if keycode > (&keymap.keys)[k as usize].keycode {
                break;
            }
            keycode = (&keymap.keys)[k as usize].keycode.wrapping_sub(1_u32);
        }
        keymap.redirect_key_auto = keycode;
    }
    keymap.keycodes_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut keymap.keycodes_section_name);
    true
}
pub fn CompileKeycodes(file: Option<&mut XkbFile>, keymap_info: &mut xkb_keymap_info<'_>) -> bool {
    let mut info = KeyNamesInfo::new();
    InitKeyNamesInfo(&mut info, 0_u32);
    if let Some(file) = file {
        HandleKeycodesFile(&mut info, file, keymap_info);
    }
    if (info.errorCount == 0_i32) && CopyKeyNamesInfoToKeymap(&mut info, keymap_info) {
        ClearKeyNamesInfo(&mut info);
        return true;
    }
    ClearKeyNamesInfo(&mut info);
    false
}
