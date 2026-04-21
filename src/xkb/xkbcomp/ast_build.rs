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
    XKB_WARNING_CANNOT_INFER_KEY_TYPE, XKB_WARNING_CONFLICTING_KEY_ACTION,
    XKB_WARNING_CONFLICTING_KEY_FIELDS, XKB_WARNING_CONFLICTING_KEY_NAME,
    XKB_WARNING_CONFLICTING_KEY_SYMBOL, XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES, XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
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
    merge_mode, stmt_type, xkb_map_flags, ExprDef, ExprKind, GroupCompatDef, IncludeStmt,
    InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef, Statement,
    SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, _IncludeStmt, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES,
    FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
    MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR,
    MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
    MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD,
    STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE,
    STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT,
    STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT,
    STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT,
    STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP,
    STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION,
    STMT_VAR, STMT_VMOD, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES,
};
pub use crate::xkb::utf8_decoding::{utf8_next_code_point_safe, INVALID_UTF8_CODE_POINT};
pub use crate::xkb::xkbcomp::include::{MERGE_AUGMENT_PREFIX, MERGE_REPLACE_PREFIX};

pub fn expr_create(kind: ExprKind) -> Box<ExprDef> {
    Box::new(ExprDef { kind })
}

pub fn ExprCreateKeySymList(sym: u32) -> Box<ExprDef> {
    let mut syms = Vec::new();
    if sym != XKB_KEY_NoSymbol as u32 {
        syms.push(sym);
    }
    expr_create(ExprKind::KeysymList { syms })
}

pub fn ExprAppendKeySymList(mut expr: Box<ExprDef>, sym: u32) -> Box<ExprDef> {
    if sym != XKB_KEY_NoSymbol as u32 {
        if let ExprKind::KeysymList { ref mut syms } = expr.kind {
            syms.push(sym);
        }
    }
    expr
}

pub fn ExprKeySymListAppendString(
    scanner: &mut scanner,
    mut expr: Box<ExprDef>,
    string: &str,
) -> Option<Box<ExprDef>> {
    let bytes = string.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;
    let mut idx_cp: usize = 1;
    loop {
        if idx >= len {
            return Some(expr);
        }
        let (cp, cp_len) = utf8_next_code_point_safe(&bytes[idx..]);
        if cp == INVALID_UTF8_CODE_POINT {
            let loc = scanner.token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to keysyms: Invalid UTF-8 encoding starting at byte position {} (code point position: {}).\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &scanner.file_name,
                loc.line,
                loc.column,
                idx + 1,
                idx_cp);
            return None;
        }
        let sym = xkb_utf32_to_keysym(cp);
        if sym == XKB_KEY_NoSymbol as u32 {
            let loc = scanner.token_location();
            log::error!("{}:{}:{}: Cannot convert string to keysyms: Unicode code point U+04{:X} has no keysym equivalent(byte position: {}, code point position: {}).\n",
                &scanner.file_name,
                loc.line,
                loc.column,
                cp,
                idx + 1,
                idx_cp);
            return None;
        }
        if let ExprKind::KeysymList { ref mut syms } = expr.kind {
            syms.push(sym);
        }
        idx += cp_len;
        idx_cp += 1;
    }
}

pub fn KeysymParseString(scanner: &mut scanner, string: &str) -> u32 {
    let bytes = string.as_bytes();
    let len = bytes.len();
    if len == 0 {
        let loc = scanner.token_location();
        log::error!(
            "{}:{}:{}: Cannot convert string to single keysym: empty string.\n",
            &scanner.file_name,
            loc.line,
            loc.column
        );
        return XKB_KEY_NoSymbol as u32;
    }
    let (cp, cp_len) = utf8_next_code_point_safe(bytes);
    if cp == INVALID_UTF8_CODE_POINT {
        let loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Invalid UTF-8 encoding.\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &scanner.file_name,
            loc.line,
            loc.column);
        return XKB_KEY_NoSymbol as u32;
    } else if cp_len != len {
        let loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Expected a single Unicode code point, got: \"{}\".\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &scanner.file_name,
            loc.line,
            loc.column,
            string);
        return XKB_KEY_NoSymbol as u32;
    }
    let sym = xkb_utf32_to_keysym(cp);
    if sym == XKB_KEY_NoSymbol as u32 {
        let loc = scanner.token_location();
        log::error!("{}:{}:{}: Cannot convert string to single keysym: Unicode code point U+{:04X} has no keysym equivalent.\n",
            &scanner.file_name,
            loc.line,
            loc.column,
            cp);
    }
    sym
}

pub fn KeycodeCreate(name: u32, value: i64) -> Box<KeycodeDef> {
    Box::new(KeycodeDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub fn KeyAliasCreate(alias: u32, real: u32) -> Box<KeyAliasDef> {
    Box::new(KeyAliasDef {
        merge: MERGE_DEFAULT,
        alias,
        real,
    })
}

pub fn VModCreate(name: u32, value: Option<Box<ExprDef>>) -> Box<VModDef> {
    Box::new(VModDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub fn VarCreate(name: Option<Box<ExprDef>>, value: Option<Box<ExprDef>>) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub fn BoolVarCreate(ident: u32, set: bool) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name: Some(Box::new(ExprDef {
            kind: ExprKind::Ident(ident),
        })),
        value: Some(Box::new(ExprDef {
            kind: ExprKind::Boolean(set),
        })),
    })
}

pub fn InterpCreate(sym: u32, match_0: Option<Box<ExprDef>>) -> Box<InterpDef> {
    Box::new(InterpDef {
        merge: MERGE_DEFAULT,
        sym,
        match_0,
        def: Vec::new(),
    })
}

pub fn KeyTypeCreate(name: u32, body: Vec<VarDef>) -> Box<KeyTypeDef> {
    Box::new(KeyTypeDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub fn SymbolsCreate(keyName: u32, symbols: Vec<VarDef>) -> Box<SymbolsDef> {
    Box::new(SymbolsDef {
        merge: MERGE_DEFAULT,
        keyName,
        symbols,
    })
}

pub fn GroupCompatCreate(group: i64, val: Option<Box<ExprDef>>) -> Box<GroupCompatDef> {
    Box::new(GroupCompatDef {
        merge: MERGE_DEFAULT,
        group,
        def: val,
    })
}

pub fn ModMapCreate(modifier: u32, keys: Vec<ExprDef>) -> Box<ModMapDef> {
    Box::new(ModMapDef {
        merge: MERGE_DEFAULT,
        modifier,
        keys,
    })
}

pub fn LedMapCreate(name: u32, body: Vec<VarDef>) -> Box<LedMapDef> {
    Box::new(LedMapDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub fn LedNameCreate(ndx: i64, name: Option<Box<ExprDef>>, virtual_0: bool) -> Box<LedNameDef> {
    Box::new(LedNameDef {
        merge: MERGE_DEFAULT,
        ndx,
        virtual_0,
        name,
    })
}

pub fn UnknownStatementCreate(type_0: stmt_type, name: &str) -> Box<UnknownStatement> {
    Box::new(UnknownStatement {
        stmt_type: type_0,
        name: name.to_string(),
    })
}

pub fn IncludeCreate(
    _ctx: &mut xkb_context,
    stmt_str: &str,
    mut merge: merge_mode,
) -> Option<Box<IncludeStmt>> {
    let mut items: Vec<Box<IncludeStmt>> = Vec::new();
    let mut remaining: Option<&str> = Some(stmt_str);

    loop {
        let input = match remaining {
            Some(s) if !s.is_empty() => s,
            _ => break,
        };

        let (parsed, rest) = match crate::xkb::xkbcomp::include::ParseIncludeMap(input) {
            Some(r) => r,
            None => {
                log::error!(
                    "[XKB-{:03}] Illegal include statement \"{}\"; Ignored\n",
                    XKB_ERROR_INVALID_INCLUDE_STATEMENT as i32,
                    stmt_str
                );
                return None;
            }
        };

        if parsed.file.is_empty() {
            remaining = rest;
            continue;
        }

        items.push(Box::new(IncludeStmt {
            merge,
            stmt: String::new(),
            file: parsed.file,
            map: parsed.map,
            modifier: parsed.extra_data,
            next_incl: None,
        }));

        match parsed.nextop {
            '|' => merge = MERGE_AUGMENT,
            '^' => merge = MERGE_REPLACE,
            _ => merge = MERGE_OVERRIDE,
        }

        remaining = rest;
    }

    // Build linked list in reverse
    let mut first: Option<Box<IncludeStmt>> = None;
    for mut item in items.into_iter().rev() {
        item.next_incl = first;
        first = Some(item);
    }

    if let Some(ref mut f) = first {
        f.stmt = stmt_str.to_string();
    }
    first
}

pub fn XkbFileCreate(
    type_0: u32,
    name: Option<String>,
    defs: Vec<Statement>,
    flags: xkb_map_flags,
) -> Box<XkbFile> {
    let mut name_str = name.unwrap_or_default();
    XkbEscapeMapName(&mut name_str);
    Box::new(XkbFile {
        file_type: type_0,
        name: name_str,
        defs,
        flags,
    })
}

pub fn XkbFileFromComponents(
    ctx: &mut xkb_context,
    kkctgs: &xkb_component_names,
) -> Option<Box<XkbFile>> {
    let components = [
        &kkctgs.keycodes,
        &kkctgs.types,
        &kkctgs.compatibility,
        &kkctgs.symbols,
    ];
    let mut file_stmts: Vec<Statement> = Vec::new();
    let mut type_0 = FIRST_KEYMAP_FILE_TYPE;
    while type_0 <= LAST_KEYMAP_FILE_TYPE {
        let component_bytes: Vec<u8> = components[type_0 as usize]
            .iter()
            .map(|&b| b as u8)
            .collect();
        let end = component_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(component_bytes.len());
        let component_str = std::str::from_utf8(&component_bytes[..end]).unwrap_or("");
        let include = IncludeCreate(ctx, component_str, MERGE_DEFAULT)?;
        let defs = vec![Statement::Include(include)];
        let file = XkbFileCreate(type_0, None, defs, 0);
        file_stmts.push(Statement::XkbFile(file));
        type_0 += 1;
    }
    Some(XkbFileCreate(FILE_TYPE_KEYMAP, None, file_stmts, 0))
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
