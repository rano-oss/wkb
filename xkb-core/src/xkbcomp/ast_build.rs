pub use crate::keymap::XkbEscapeMapName;
pub use crate::messages::{XKB_ERROR_INVALID_FILE_ENCODING, XKB_ERROR_INVALID_INCLUDE_STATEMENT};
pub use crate::scanner_utils::scanner;
pub use crate::shared_ast_types::{
    merge_mode, stmt_type, xkb_map_flags, ExprDef, ExprKind, GroupCompatDef, IncludeStmt,
    InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef, Statement,
    SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, FILE_TYPE_KEYMAP,
    FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
    MERGE_REPLACE, _STMT_NUM_VALUES,
};
pub use crate::utf8_decoding::{utf8_next_code_point_safe, INVALID_UTF8_CODE_POINT};

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
        let sym = utf32_to_keysym(cp);
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
    let sym = utf32_to_keysym(cp);
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

        let (parsed, rest) = match crate::xkbcomp::include::ParseIncludeMap(input) {
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
use crate::keysym_utf::utf32_to_keysym;
use crate::shared_types::*;
