
use crate::xkb::utils::{cstr_free, cstr_ndup};

pub use crate::xkb::keymap::XkbEscapeMapName;
pub use crate::xkb::messages::{
    XKB_ERROR_ABI_BACKWARD_COMPAT_, XKB_ERROR_ABI_FORWARD_COMPAT_,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR, XKB_ERROR_CANNOT_RESOLVE_RMLVO,
    XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY, XKB_ERROR_EXPECTED_ARRAY_ENTRY,
    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE, XKB_ERROR_INCLUDED_FILE_NOT_FOUND,
    XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT, XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT,
    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE, XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD,
    XKB_ERROR_INVALID_COMPOSE_LOCALE, XKB_ERROR_INVALID_COMPOSE_SYNTAX,
    XKB_ERROR_INVALID_EXPRESSION_TYPE, XKB_ERROR_INVALID_FILE_ENCODING,
    XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
    XKB_ERROR_INVALID_INCLUDE_STATEMENT, XKB_ERROR_INVALID_MODMAP_ENTRY,
    XKB_ERROR_INVALID_NUMERIC_KEYSYM, XKB_ERROR_INVALID_OPERATION, XKB_ERROR_INVALID_PATH,
    XKB_ERROR_INVALID_REAL_MODIFIER, XKB_ERROR_INVALID_RULES_SYNTAX,
    XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT, XKB_ERROR_INVALID_VALUE, XKB_ERROR_INVALID_XKB_SYNTAX,
    XKB_ERROR_KEYMAP_COMPILATION_FAILED, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_ERROR_OVERLAPPING_OVERLAY,
    XKB_ERROR_RECURSIVE_INCLUDE, XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION,
    XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER, XKB_ERROR_UNKNOWN_ACTION_TYPE,
    XKB_ERROR_UNKNOWN_DEFAULT_FIELD, XKB_ERROR_UNKNOWN_FIELD, XKB_ERROR_UNKNOWN_OPERATOR,
    XKB_ERROR_UNKNOWN_STATEMENT, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_, XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX,
    XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL, XKB_ERROR_WRONG_FIELD_TYPE, XKB_ERROR_WRONG_STATEMENT_TYPE,
    XKB_WARNING_CANNOT_INFER_KEY_TYPE,
    XKB_WARNING_CONFLICTING_KEY_ACTION, XKB_WARNING_CONFLICTING_KEY_FIELDS,
    XKB_WARNING_CONFLICTING_KEY_NAME, XKB_WARNING_CONFLICTING_KEY_SYMBOL,
    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS, XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY, XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES, XKB_WARNING_CONFLICTING_MODMAP,
    XKB_WARNING_DEPRECATED_KEYSYM, XKB_WARNING_DEPRECATED_KEYSYM_NAME, XKB_WARNING_DUPLICATE_ENTRY,
    XKB_WARNING_EXTRA_SYMBOLS_IGNORED, XKB_WARNING_ILLEGAL_KEYCODE_ALIAS,
    XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT, XKB_WARNING_INVALID_ESCAPE_SEQUENCE,
    XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE, XKB_WARNING_MISSING_DEFAULT_SECTION,
    XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX, XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE,
    XKB_WARNING_NON_BASE_GROUP_NAME, XKB_WARNING_NUMERIC_KEYSYM,
    XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE, XKB_WARNING_UNDEFINED_KEYCODE,
    XKB_WARNING_UNDEFINED_KEY_TYPE, XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
    XKB_WARNING_UNRECOGNIZED_KEYSYM, XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL,
    XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION, XKB_WARNING_UNSUPPORTED_LEGACY_ACTION,
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
};
pub use crate::xkb::scanner_utils::{scanner, scanner_loc, sval};
pub use crate::xkb::shared_ast_types::{
    box_from_raw, merge_mode, stmt_type, xkb_map_flags, ExprDef, ExprKind, GroupCompatDef,
    IncludeStmt, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef,
    OptBoxRaw, ParseCommon, SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, _IncludeStmt,
    _ParseCommon, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES,
    FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER,
    MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT,
    MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
    STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL,
    STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL,
    STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL,
    STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS,
    STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME,
    STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES,
};
pub use crate::xkb::utf8_decoding::{utf8_next_code_point_safe, INVALID_UTF8_CODE_POINT};
use crate::xkb::utils::cstr_len;
pub use crate::xkb::xkbcomp::include::{MERGE_AUGMENT_PREFIX, MERGE_REPLACE_PREFIX};
pub fn expr_create(kind: ExprKind) -> *mut ExprDef {
    let type_0 = ExprDef::stmt_type_for_kind(&kind);
    Box::into_raw(Box::new(ExprDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0,
        },
        kind,
    }))
}

pub fn ExprCreateKeySymList(sym: u32) -> *mut ExprDef {
    let mut syms = Vec::new();
    if sym != XKB_KEY_NoSymbol as u32 {
        syms.push(sym);
    }
    expr_create(ExprKind::KeysymList { syms })
}

pub fn ExprAppendKeySymList(expr: *mut ExprDef, sym: u32) -> *mut ExprDef {
    unsafe {
        if sym != XKB_KEY_NoSymbol as u32 {
            if let ExprKind::KeysymList { ref mut syms } = (*expr).kind {
                syms.push(sym);
            }
        }
        expr
    }
}

pub fn ExprKeySymListAppendString(
    scanner: *mut scanner,
    expr: *mut ExprDef,
    string: *const i8,
) -> *mut ExprDef {
    unsafe {
        let c2rust_current_block: u64;
        let len: usize = cstr_len(string);
        let mut idx: usize = 0_usize;
        let mut idx_cp: usize = 1_usize;
        loop {
            if idx >= len {
                c2rust_current_block = 18317007320854588510;
                break;
            }
            let mut count: usize = 0_usize;
            let bytes =
                std::slice::from_raw_parts(string.add(idx) as *const u8, len.wrapping_sub(idx));
            let (cp, cp_len) = utf8_next_code_point_safe(bytes);
            count = cp_len;
            if cp == INVALID_UTF8_CODE_POINT {
                let loc: scanner_loc = (*scanner).token_location();
                log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to keysyms: Invalid UTF-8 encoding starting at byte position {} (code point position: {}).\n",
                    XKB_ERROR_INVALID_FILE_ENCODING as i32,
                    &(*scanner).file_name,
                    loc.line,
                    loc.column,
                    idx.wrapping_add(1_usize),
                    idx_cp);
                c2rust_current_block = 5140853804782746302;
                break;
            } else {
                let sym: u32 = xkb_utf32_to_keysym(cp);
                if sym == XKB_KEY_NoSymbol as u32 {
                    let loc_0: scanner_loc = (*scanner).token_location();
                    log::error!("{}:{}:{}: Cannot convert string to keysyms: Unicode code point U+04{:X} has no keysym equivalent(byte position: {}, code point position: {}).\n",
                        &(*scanner).file_name,
                        loc_0.line,
                        loc_0.column,
                        cp,
                        idx.wrapping_add(1_usize),
                        idx_cp);
                    c2rust_current_block = 5140853804782746302;
                    break;
                } else {
                    if let ExprKind::KeysymList { ref mut syms } = (*expr).kind {
                        syms.push(sym);
                    }
                    idx = idx.wrapping_add(count);
                    idx_cp = idx_cp.wrapping_add(1);
                }
            }
        }
        match c2rust_current_block {
            5140853804782746302 => {
                FreeStmt(expr as *mut ParseCommon);
                std::ptr::null_mut()
            }
            _ => expr,
        }
    }
}

pub fn KeysymParseString(scanner: *mut scanner, string: *const i8) -> u32 {
    unsafe {
        let len: usize = cstr_len(string);
        if len == 0_usize {
            let loc: scanner_loc = (*scanner).token_location();
            log::error!("{}:{}:{}: Cannot convert string to single keysym: empty string.\n",
                &(*scanner).file_name,
                loc.line,
                loc.column);
            return XKB_KEY_NoSymbol as u32;
        }
        let mut count: usize = 0_usize;
        let bytes = std::slice::from_raw_parts(string as *const u8, len);
        let (cp, cp_len) = utf8_next_code_point_safe(bytes);
        count = cp_len;
        if cp == INVALID_UTF8_CODE_POINT {
            let loc_0: scanner_loc = (*scanner).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Invalid UTF-8 encoding.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &(*scanner).file_name,
                loc_0.line,
                loc_0.column);
            return XKB_KEY_NoSymbol as u32;
        } else if count != len {
            let loc_1: scanner_loc = (*scanner).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Expected a single Unicode code point, got: \"{}\".\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &(*scanner).file_name,
                loc_1.line,
                loc_1.column,
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(string)));
            return XKB_KEY_NoSymbol as u32;
        }
        let sym: u32 = xkb_utf32_to_keysym(cp);
        if sym == XKB_KEY_NoSymbol as u32 {
            let loc_2: scanner_loc = (*scanner).token_location();
            log::error!("{}:{}:{}: Cannot convert string to single keysym: Unicode code point U+{:04X} has no keysym equivalent.\n",
                &(*scanner).file_name,
                loc_2.line,
                loc_2.column,
                cp);
        }
        sym
    }
}

pub fn KeycodeCreate(name: u32, value: i64) -> *mut KeycodeDef {
    Box::into_raw(Box::new(KeycodeDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_KEYCODE,
        },
        merge: MERGE_DEFAULT,
        name,
        value,
    }))
}

pub fn KeyAliasCreate(alias: u32, real: u32) -> *mut KeyAliasDef {
    Box::into_raw(Box::new(KeyAliasDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_ALIAS,
        },
        merge: MERGE_DEFAULT,
        alias,
        real,
    }))
}

pub fn VModCreate(name: u32, value: *mut ExprDef) -> *mut VModDef {
    Box::into_raw(Box::new(VModDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_VMOD,
        },
        merge: MERGE_DEFAULT,
        name,
        value: unsafe { box_from_raw(value) },
    }))
}

pub fn VarCreate(name: *mut ExprDef, value: *mut ExprDef) -> *mut VarDef {
    Box::into_raw(Box::new(VarDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_VAR,
        },
        merge: MERGE_DEFAULT,
        name: unsafe { box_from_raw(name) },
        value: unsafe { box_from_raw(value) },
    }))
}

pub fn BoolVarCreate(ident: u32, set: bool) -> *mut VarDef {
    Box::into_raw(Box::new(VarDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_VAR,
        },
        merge: MERGE_DEFAULT,
        name: Some(Box::new(ExprDef {
            common: ParseCommon {
                next: std::ptr::null_mut(),
                type_0: STMT_EXPR_IDENT,
            },
            kind: ExprKind::Ident(ident),
        })),
        value: Some(Box::new(ExprDef {
            common: ParseCommon {
                next: std::ptr::null_mut(),
                type_0: STMT_EXPR_BOOLEAN_LITERAL,
            },
            kind: ExprKind::Boolean(set),
        })),
    }))
}

pub fn InterpCreate(sym: u32, match_0: *mut ExprDef) -> *mut InterpDef {
    Box::into_raw(Box::new(InterpDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_INTERP,
        },
        merge: MERGE_DEFAULT,
        sym,
        match_0: unsafe { box_from_raw(match_0) },
        def: None,
    }))
}

pub fn KeyTypeCreate(name: u32, body: *mut VarDef) -> *mut KeyTypeDef {
    Box::into_raw(Box::new(KeyTypeDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_TYPE,
        },
        merge: MERGE_DEFAULT,
        name,
        body: unsafe { box_from_raw(body) },
    }))
}

pub fn SymbolsCreate(keyName: u32, symbols: *mut VarDef) -> *mut SymbolsDef {
    Box::into_raw(Box::new(SymbolsDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_SYMBOLS,
        },
        merge: MERGE_DEFAULT,
        keyName,
        symbols: unsafe { box_from_raw(symbols) },
    }))
}

pub fn GroupCompatCreate(group: i64, val: *mut ExprDef) -> *mut GroupCompatDef {
    Box::into_raw(Box::new(GroupCompatDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_GROUP_COMPAT,
        },
        merge: MERGE_DEFAULT,
        group,
        def: unsafe { box_from_raw(val) },
    }))
}

pub fn ModMapCreate(modifier: u32, keys: *mut ExprDef) -> *mut ModMapDef {
    Box::into_raw(Box::new(ModMapDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_MODMAP,
        },
        merge: MERGE_DEFAULT,
        modifier,
        keys: unsafe { box_from_raw(keys) },
    }))
}

pub fn LedMapCreate(name: u32, body: *mut VarDef) -> *mut LedMapDef {
    Box::into_raw(Box::new(LedMapDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_LED_MAP,
        },
        merge: MERGE_DEFAULT,
        name,
        body: unsafe { box_from_raw(body) },
    }))
}

pub fn LedNameCreate(ndx: i64, name: *mut ExprDef, virtual_0: bool) -> *mut LedNameDef {
    Box::into_raw(Box::new(LedNameDef {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0: STMT_LED_NAME,
        },
        merge: MERGE_DEFAULT,
        ndx,
        virtual_0,
        name: unsafe { box_from_raw(name) },
    }))
}

pub fn UnknownStatementCreate(type_0: stmt_type, name: sval) -> *mut UnknownStatement {
    let name_str = unsafe { std::str::from_utf8_unchecked(name.as_bytes()).to_string() };
    Box::into_raw(Box::new(UnknownStatement {
        common: ParseCommon {
            next: std::ptr::null_mut(),
            type_0,
        },
        name: name_str,
    }))
}

pub fn IncludeCreate(
    _ctx: *mut xkb_context,
    str: *mut i8,
    mut merge: merge_mode,
) -> *mut IncludeStmt {
    unsafe {
        let mut first: *mut IncludeStmt = std::ptr::null_mut();
        let mut incl: *mut IncludeStmt = std::ptr::null_mut();

        let stmt_str = std::ffi::CStr::from_ptr(str)
            .to_str()
            .unwrap_or("")
            .to_string();
        let mut remaining: Option<&str> = Some(&stmt_str);

        loop {
            let input = match remaining {
                Some(s) if !s.is_empty() => s,
                _ => break,
            };

            let (parsed, rest) = match crate::xkb::xkbcomp::include::ParseIncludeMap(input) {
                Some(r) => r,
                None => {
                    // Parse error
                    log::error!("[XKB-{:03}] Illegal include statement \"{}\"; Ignored\n",
                        XKB_ERROR_INVALID_INCLUDE_STATEMENT as i32,
                        &stmt_str);
                    FreeInclude(first);
                    return std::ptr::null_mut();
                }
            };

            if parsed.file.is_empty() {
                // skip empty segments
                remaining = rest;
                continue;
            }

            let new_incl = Box::into_raw(Box::new(IncludeStmt {
                common: ParseCommon {
                    next: std::ptr::null_mut(),
                    type_0: STMT_INCLUDE,
                },
                merge,
                stmt: String::new(),
                file: parsed.file,
                map: parsed.map,
                modifier: parsed.extra_data,
                next_incl: None,
            }));

            if first.is_null() {
                first = new_incl;
                incl = new_incl;
            } else {
                (*incl).next_incl = Some(Box::from_raw(new_incl));
                incl = (*incl).next_incl.as_mut().unwrap().as_mut() as *mut IncludeStmt;
            }

            match parsed.nextop {
                '|' => merge = MERGE_AUGMENT,
                '^' => merge = MERGE_REPLACE,
                _ => merge = MERGE_OVERRIDE,
            }

            remaining = rest;
        }

        if !first.is_null() {
            (*first).stmt = stmt_str;
        }
        first
    }
}

pub fn XkbFileCreate(
    type_0: u32,
    name: *mut i8,
    defs: *mut ParseCommon,
    flags: xkb_map_flags,
) -> *mut XkbFile {
    unsafe {
        let mut name_str = if name.is_null() {
            String::new()
        } else {
            let s = std::ffi::CStr::from_ptr(name)
                .to_str()
                .unwrap_or("")
                .to_string();
            cstr_free(name);
            s
        };
        XkbEscapeMapName(&mut name_str);
        let file = Box::into_raw(Box::new(XkbFile {
            common: ParseCommon {
                next: std::ptr::null_mut(),
                type_0: 0,
            },
            file_type: type_0,
            name: name_str,
            defs,
            flags,
        }));
        file
    }
}

pub fn XkbFileFromComponents(
    ctx: *mut xkb_context,
    kkctgs: *const xkb_component_names,
) -> *mut XkbFile {
    unsafe {
        let c2rust_current_block: u64;
        let components: [*mut i8; 4] = [
            (*kkctgs).keycodes.as_ptr() as *mut i8,
            (*kkctgs).types.as_ptr() as *mut i8,
            (*kkctgs).compatibility.as_ptr() as *mut i8,
            (*kkctgs).symbols.as_ptr() as *mut i8,
        ];
        let mut type_0: u32;
        let mut include: *mut IncludeStmt;
        let mut file: *mut XkbFile;
        let mut defs: *mut ParseCommon = std::ptr::null_mut();
        let mut defsLast: *mut ParseCommon = std::ptr::null_mut();
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        loop {
            if type_0 > LAST_KEYMAP_FILE_TYPE {
                c2rust_current_block = 13536709405535804910;
                break;
            }
            include = IncludeCreate(ctx, components[type_0 as usize], MERGE_DEFAULT);
            if include.is_null() {
                c2rust_current_block = 183321912633560349;
                break;
            }
            file = XkbFileCreate(
                type_0,
                std::ptr::null_mut(),
                include as *mut ParseCommon,
                0 as xkb_map_flags,
            );
            if file.is_null() {
                FreeInclude(include);
                c2rust_current_block = 183321912633560349;
                break;
            } else {
                if defs.is_null() {
                    defs = &raw mut (*file).common;
                    defsLast = defs;
                } else {
                    (*defsLast).next = &raw mut (*file).common as *mut _ParseCommon;
                    defsLast = (*defsLast).next as *mut ParseCommon;
                }
                type_0 += 1;
            }
        }
        if c2rust_current_block == 13536709405535804910 {
            file = XkbFileCreate(
                FILE_TYPE_KEYMAP,
                std::ptr::null_mut(),
                defs,
                0 as xkb_map_flags,
            );
            if !file.is_null() {
                return file;
            }
        }
        FreeXkbFile(defs as *mut XkbFile);
        std::ptr::null_mut()
    }
}
fn FreeInclude(incl: *mut IncludeStmt) {
    unsafe {
        if !incl.is_null() {
            // Dropping the Box will recursively drop next_incl (Option<Box<>>)
            // and all String fields automatically
            drop(Box::from_raw(incl));
        }
    }
}

pub fn FreeStmt(mut stmt: *mut ParseCommon) {
    unsafe {
        let mut next: *mut ParseCommon;
        while !stmt.is_null() {
            next = (*stmt).next as *mut ParseCommon;
            match (*stmt).type_0 {
                1 => {
                    FreeInclude(stmt as *mut IncludeStmt);
                    stmt = std::ptr::null_mut();
                }
                // All expression types: use ExprDef enum
                4..=25 => {
                    let expr = stmt as *mut ExprDef;
                    // Action.args and ActionList.actions are linked-list heads.
                    // The first node lives in the Box; subsequent nodes are linked
                    // via common.next raw pointers.  We must take ownership of the
                    // whole chain and free it with FreeStmt (which walks common.next).
                    // Binary/Unary/ArrayRef children are single nodes — their
                    // Option<Box<ExprDef>> auto-drops when the parent ExprDef drops.
                    match &mut (*expr).kind {
                        ExprKind::Action { ref mut args, .. } => {
                            if let Some(boxed) = args.take() {
                                FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                            }
                        }
                        ExprKind::ActionList { ref mut actions } => {
                            if let Some(boxed) = actions.take() {
                                FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                            }
                        }
                        _ => {}
                    }
                }
                26 => {
                    // VarDef: name and value are single nodes → auto-drop via Box
                }
                27 => {
                    // KeyTypeDef: body is a linked list
                    if let Some(boxed) = (*(stmt as *mut KeyTypeDef)).body.take() {
                        FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                    }
                }
                28 => {
                    // InterpDef: match_0 is single node → auto-drop. def is linked list.
                    if let Some(boxed) = (*(stmt as *mut InterpDef)).def.take() {
                        FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                    }
                }
                29 => {
                    // VModDef: value is single node → auto-drop
                }
                30 => {
                    // SymbolsDef: symbols is a linked list
                    if let Some(boxed) = (*(stmt as *mut SymbolsDef)).symbols.take() {
                        FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                    }
                }
                31 => {
                    // ModMapDef: keys is a linked list
                    if let Some(boxed) = (*(stmt as *mut ModMapDef)).keys.take() {
                        FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                    }
                }
                32 => {
                    // GroupCompatDef: def is single node → auto-drop
                }
                33 => {
                    // LedMapDef: body is a linked list
                    if let Some(boxed) = (*(stmt as *mut LedMapDef)).body.take() {
                        FreeStmt(Box::into_raw(boxed) as *mut ParseCommon);
                    }
                }
                34 => {
                    // LedNameDef: name is single node → auto-drop
                }
                35 | 36 => {
                    // String name auto-drops via Box::from_raw below
                }
                _ => {}
            }
            // Deallocate the stmt with the correct type (Box::from_raw must match Box::into_raw type)
            if !stmt.is_null() {
                match (*stmt).type_0 {
                    2 => drop(Box::from_raw(stmt as *mut KeycodeDef)),
                    3 => drop(Box::from_raw(stmt as *mut KeyAliasDef)),
                    26 => drop(Box::from_raw(stmt as *mut VarDef)),
                    27 => drop(Box::from_raw(stmt as *mut KeyTypeDef)),
                    28 => drop(Box::from_raw(stmt as *mut InterpDef)),
                    29 => drop(Box::from_raw(stmt as *mut VModDef)),
                    30 => drop(Box::from_raw(stmt as *mut SymbolsDef)),
                    31 => drop(Box::from_raw(stmt as *mut ModMapDef)),
                    32 => drop(Box::from_raw(stmt as *mut GroupCompatDef)),
                    33 => drop(Box::from_raw(stmt as *mut LedMapDef)),
                    34 => drop(Box::from_raw(stmt as *mut LedNameDef)),
                    35 | 36 => drop(Box::from_raw(stmt as *mut UnknownStatement)),
                    _ => drop(Box::from_raw(stmt as *mut ExprDef)), // all STMT_EXPR_* types
                }
            }
            stmt = next;
        }
    }
}

pub fn FreeXkbFile(mut file: *mut XkbFile) {
    unsafe {
        let mut next: *mut XkbFile;
        while !file.is_null() {
            next = (*file).common.next as *mut XkbFile;
            match (*file).file_type {
                5 => {
                    FreeXkbFile((*file).defs as *mut XkbFile);
                }
                0..=4 => {
                    FreeStmt((*file).defs);
                }
                _ => {}
            }
            // String name and child defs auto-drop via Box
            drop(Box::from_raw(file));
            file = next;
        }
    }
}
static XKB_FILE_TYPE_STRINGS: [&str; 7] = [
    "xkb_keycodes",
    "xkb_types",
    "xkb_compatibility",
    "xkb_symbols",
    "xkb_geometry",
    "xkb_keymap",
    "rules",
];

pub fn xkb_file_type_to_string(type_0: u32) -> &'static str {
    if type_0 as usize >= XKB_FILE_TYPE_STRINGS.len() {
        "unknown"
    } else {
        XKB_FILE_TYPE_STRINGS[type_0 as usize]
    }
}
static STMT_TYPE_STRINGS: [&str; 37] = [
    "unknown statement",
    "include statement",
    "key name definition",
    "key alias definition",
    "string literal expression",
    "integer literal expression",
    "float literal expression",
    "boolean literal expression",
    "key name expression",
    "keysym expression",
    "identifier expression",
    "action declaration expression",
    "field reference expression",
    "array reference expression",
    "empty list expression",
    "keysym list expression",
    "action list expression",
    "addition expression",
    "substraction expression",
    "multiplication expression",
    "division expression",
    "assignment expression",
    "logical negation expression",
    "arithmetic negation expression",
    "bitwise inversion expression",
    "unary plus expression",
    "variable definition",
    "key type definition",
    "symbol interpretation definition",
    "virtual modifiers definition",
    "key symbols definition",
    "modifier map declaration",
    "group declaration",
    "indicator map declaration",
    "indicator name declaration",
    "unknown declaration statement",
    "unknown compound statement",
];

pub fn stmt_type_to_string(type_0: u32) -> &'static str {
    if type_0 >= _STMT_NUM_VALUES {
        return "unknown";
    }
    STMT_TYPE_STRINGS[type_0 as usize]
}

pub fn stmt_type_to_operator_char(type_0: u32) -> char {
    match type_0 {
        17 => '+',
        18 => '-',
        19 => '*',
        20 => '/',
        22 => '!',
        23 => '-',
        24 => '~',
        25 => '+',
        _ => '\0',
    }
}
use crate::xkb::keysym_utf::xkb_utf32_to_keysym;
use crate::xkb::shared_types::*;
