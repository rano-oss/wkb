use crate::xkb_logf;

use crate::xkb::utils::{cstr_free, cstr_ndup};

pub use crate::xkb::keymap_priv::XkbEscapeMapName;
pub use crate::xkb::messages::{
    xkb_log_verbosity, xkb_message_code, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
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
    XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE, XKB_LOG_VERBOSITY_DEFAULT,
    XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL, XKB_LOG_VERBOSITY_SILENT,
    XKB_LOG_VERBOSITY_VERBOSE, XKB_WARNING_CANNOT_INFER_KEY_TYPE,
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
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD,
};
pub use crate::xkb::scanner_utils::{scanner, scanner_loc, scanner_token_location, sval};
pub use crate::xkb::shared_ast_types::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, xkb_file_type, xkb_map_flags, ExprAction,
    ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef, ExprIdent,
    ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary, GroupCompatDef,
    IncludeStmt, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef,
    ParseCommon, SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES,
    _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY,
    FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS,
    FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC,
    MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN,
    MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS,
    STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF,
    STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST,
    STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL,
    STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL,
    STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL,
    STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP,
    STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN,
    STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use crate::xkb::utf8_decoding::{utf8_next_code_point, INVALID_UTF8_CODE_POINT};
use crate::xkb::utils::cstr_len;
pub use crate::xkb::utils::{isempty, strdup_safe};
pub use crate::xkb::xkbcomp::include::{
    ParseIncludeMap, MERGE_AUGMENT_PREFIX, MERGE_REPLACE_PREFIX,
};
unsafe fn ExprCreate(mut op: stmt_type) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = Box::into_raw(Box::new(std::mem::zeroed::<ExprDef>()));
        (*expr).common.type_0 = op;
        (*expr).common.next = std::ptr::null_mut();
        return expr;
    }
}

pub unsafe fn ExprCreateString(mut str: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_STRING_LITERAL);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).string.str = str;
        return expr;
    }
}

pub unsafe fn ExprCreateInteger(mut ival: i64) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_INTEGER_LITERAL);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).integer.ival = ival;
        return expr;
    }
}

pub unsafe fn ExprCreateFloat() -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_FLOAT_LITERAL);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        return expr;
    }
}

pub unsafe fn ExprCreateKeyName(mut key_name: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_KEYNAME_LITERAL);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).key_name.key_name = key_name;
        return expr;
    }
}

pub unsafe fn ExprCreateKeySym(mut keysym: xkb_keysym_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_KEYSYM_LITERAL);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).keysym.keysym = keysym;
        return expr;
    }
}

pub unsafe fn ExprCreateIdent(mut ident: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_IDENT);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).ident.ident = ident;
        return expr;
    }
}

pub unsafe fn ExprCreateUnary(mut op: stmt_type, mut child: *mut ExprDef) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(op);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).unary.child = child as *mut ExprDef;
        return expr;
    }
}

pub unsafe fn ExprCreateBinary(
    mut op: stmt_type,
    mut left: *mut ExprDef,
    mut right: *mut ExprDef,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(op);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).binary.left = left as *mut ExprDef;
        (*expr).binary.right = right as *mut ExprDef;
        return expr;
    }
}

pub unsafe fn ExprCreateFieldRef(mut element: xkb_atom_t, mut field: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_FIELD_REF);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).field_ref.element = element;
        (*expr).field_ref.field = field;
        return expr;
    }
}

pub unsafe fn ExprCreateArrayRef(
    mut element: xkb_atom_t,
    mut field: xkb_atom_t,
    mut entry: *mut ExprDef,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_ARRAY_REF);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).array_ref.element = element;
        (*expr).array_ref.field = field;
        (*expr).array_ref.entry = entry as *mut ExprDef;
        return expr;
    }
}

pub unsafe fn ExprEmptyList() -> *mut ExprDef {
    unsafe {
        return ExprCreate(STMT_EXPR_EMPTY_LIST);
    }
}

pub unsafe fn ExprCreateAction(mut name: xkb_atom_t, mut args: *mut ExprDef) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_ACTION_DECL);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).action.name = name;
        (*expr).action.args = args as *mut ExprDef;
        return expr;
    }
}

pub unsafe fn ExprCreateActionList(mut actions: *mut ExprDef) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_ACTION_LIST);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        (*expr).actions.actions = actions as *mut ExprDef;
        return expr;
    }
}

pub unsafe fn ExprCreateKeySymList(mut sym: xkb_keysym_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_KEYSYM_LIST);
        if expr.is_null() {
            return std::ptr::null_mut();
        }
        let ksl = expr as *mut ExprKeysymList;
        std::ptr::write(&raw mut (*ksl).syms, Vec::new());
        if !(sym == XKB_KEY_NoSymbol as xkb_keysym_t) {
            (&mut (*ksl).syms).push(sym);
        }
        return expr;
    }
}

pub unsafe fn ExprAppendKeySymList(mut expr: *mut ExprDef, mut sym: xkb_keysym_t) -> *mut ExprDef {
    unsafe {
        if !(sym == XKB_KEY_NoSymbol as xkb_keysym_t) {
            let ksl = expr as *mut ExprKeysymList;
            (&mut (*ksl).syms).push(sym);
        }
        return expr;
    }
}

pub unsafe fn ExprKeySymListAppendString(
    mut scanner: *mut scanner,
    mut expr: *mut ExprDef,
    mut string: *const i8,
) -> *mut ExprDef {
    unsafe {
        let mut c2rust_current_block: u64;
        let len: usize = cstr_len(string) as usize;
        let mut idx: usize = 0 as usize;
        let mut idx_cp: usize = 1 as usize;
        loop {
            if !(idx < len) {
                c2rust_current_block = 18317007320854588510;
                break;
            }
            let mut count: usize = 0 as usize;
            let mut cp: u32 = utf8_next_code_point(
                string.offset(idx as isize),
                len.wrapping_sub(idx),
                &raw mut count,
            );
            if cp == INVALID_UTF8_CODE_POINT as u32 {
                let mut loc: scanner_loc = scanner_token_location(scanner);
                xkb_logf!(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: Cannot convert string to keysyms: Invalid UTF-8 encoding starting at byte position {} (code point position: {}).\n",
                    XKB_ERROR_INVALID_FILE_ENCODING as i32,
                    crate::xkb::utils::CStrDisplay((*scanner).file_name),
                    loc.line,
                    loc.column,
                    idx.wrapping_add(1 as usize),
                    idx_cp,
                );
                c2rust_current_block = 5140853804782746302;
                break;
            } else {
                let sym: xkb_keysym_t = xkb_utf32_to_keysym(cp) as xkb_keysym_t;
                if sym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    let mut loc_0: scanner_loc = scanner_token_location(scanner);
                    xkb_logf!(
                        (*scanner).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "{}:{}:{}: Cannot convert string to keysyms: Unicode code point U+04{:X} has no keysym equivalent(byte position: {}, code point position: {}).\n",
                        crate::xkb::utils::CStrDisplay((*scanner).file_name),
                        loc_0.line,
                        loc_0.column,
                        cp,
                        idx.wrapping_add(1 as usize),
                        idx_cp,
                    );
                    c2rust_current_block = 5140853804782746302;
                    break;
                } else {
                    let ksl = expr as *mut ExprKeysymList;
                    (&mut (*ksl).syms).push(sym);
                    idx = idx.wrapping_add(count);
                    idx_cp = idx_cp.wrapping_add(1);
                }
            }
        }
        match c2rust_current_block {
            5140853804782746302 => {
                FreeStmt(expr as *mut ParseCommon);
                return std::ptr::null_mut();
            }
            _ => {
                return expr;
            }
        };
    }
}

pub unsafe fn KeysymParseString(mut scanner: *mut scanner, mut string: *const i8) -> xkb_keysym_t {
    unsafe {
        let len: usize = cstr_len(string) as usize;
        if len == 0 as usize {
            let mut loc: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}:{}:{}: Cannot convert string to single keysym: empty string.\n",
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                loc.line,
                loc.column,
            );
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let mut count: usize = 0 as usize;
        let cp: u32 = utf8_next_code_point(string, len, &raw mut count) as u32;
        if cp == INVALID_UTF8_CODE_POINT as u32 {
            let mut loc_0: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Invalid UTF-8 encoding.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                loc_0.line,
                loc_0.column,
            );
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        } else if count != len {
            let mut loc_1: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Expected a single Unicode code point, got: \"{}\".\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                loc_1.line,
                loc_1.column,
                crate::xkb::utils::CStrDisplay(string),
            );
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let sym: xkb_keysym_t = xkb_utf32_to_keysym(cp) as xkb_keysym_t;
        if sym == XKB_KEY_NoSymbol as xkb_keysym_t {
            let mut loc_2: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}:{}:{}: Cannot convert string to single keysym: Unicode code point U+{:04X} has no keysym equivalent.\n",
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                loc_2.line,
                loc_2.column,
                cp,
            );
        }
        return sym;
    }
}

pub unsafe fn KeycodeCreate(mut name: xkb_atom_t, mut value: i64) -> *mut KeycodeDef {
    unsafe {
        let mut def: *mut KeycodeDef = Box::into_raw(Box::new(std::mem::zeroed::<KeycodeDef>()));
        (*def).common.type_0 = STMT_KEYCODE;
        (*def).common.next = std::ptr::null_mut();
        (*def).name = name;
        (*def).value = value;
        return def;
    }
}

pub unsafe fn KeyAliasCreate(mut alias: xkb_atom_t, mut real: xkb_atom_t) -> *mut KeyAliasDef {
    unsafe {
        let mut def: *mut KeyAliasDef = Box::into_raw(Box::new(std::mem::zeroed::<KeyAliasDef>()));
        (*def).common.type_0 = STMT_ALIAS;
        (*def).common.next = std::ptr::null_mut();
        (*def).alias = alias;
        (*def).real = real;
        return def;
    }
}

pub unsafe fn VModCreate(mut name: xkb_atom_t, mut value: *mut ExprDef) -> *mut VModDef {
    unsafe {
        let mut def: *mut VModDef = Box::into_raw(Box::new(std::mem::zeroed::<VModDef>()));
        (*def).common.type_0 = STMT_VMOD;
        (*def).common.next = std::ptr::null_mut();
        (*def).name = name;
        (*def).value = value as *mut ExprDef;
        return def;
    }
}

pub unsafe fn VarCreate(mut name: *mut ExprDef, mut value: *mut ExprDef) -> *mut VarDef {
    unsafe {
        let mut def: *mut VarDef = Box::into_raw(Box::new(std::mem::zeroed::<VarDef>()));
        (*def).common.type_0 = STMT_VAR;
        (*def).common.next = std::ptr::null_mut();
        (*def).name = name as *mut ExprDef;
        (*def).value = value as *mut ExprDef;
        return def;
    }
}

pub unsafe fn BoolVarCreate(mut ident: xkb_atom_t, mut set: bool) -> *mut VarDef {
    unsafe {
        let mut name: *mut ExprDef = std::ptr::null_mut();
        let mut value: *mut ExprDef = std::ptr::null_mut();
        let mut def: *mut VarDef = std::ptr::null_mut();
        name = ExprCreateIdent(ident);
        if name.is_null() {
            return std::ptr::null_mut();
        }
        value = ExprCreate(STMT_EXPR_BOOLEAN_LITERAL);
        if !value.is_null() {
            (*value).boolean.set = set;
        }
        if value.is_null() {
            FreeStmt(name as *mut ParseCommon);
            return std::ptr::null_mut();
        }
        def = VarCreate(name, value);
        if def.is_null() {
            FreeStmt(name as *mut ParseCommon);
            FreeStmt(value as *mut ParseCommon);
            return std::ptr::null_mut();
        }
        return def;
    }
}

pub unsafe fn InterpCreate(mut sym: xkb_keysym_t, mut match_0: *mut ExprDef) -> *mut InterpDef {
    unsafe {
        let mut def: *mut InterpDef = Box::into_raw(Box::new(std::mem::zeroed::<InterpDef>()));
        (*def).common.type_0 = STMT_INTERP;
        (*def).common.next = std::ptr::null_mut();
        (*def).sym = sym;
        (*def).match_0 = match_0 as *mut ExprDef;
        (*def).def = std::ptr::null_mut();
        return def;
    }
}

pub unsafe fn KeyTypeCreate(mut name: xkb_atom_t, mut body: *mut VarDef) -> *mut KeyTypeDef {
    unsafe {
        let mut def: *mut KeyTypeDef = Box::into_raw(Box::new(std::mem::zeroed::<KeyTypeDef>()));
        (*def).common.type_0 = STMT_TYPE;
        (*def).common.next = std::ptr::null_mut();
        (*def).merge = MERGE_DEFAULT;
        (*def).name = name;
        (*def).body = body;
        return def;
    }
}

pub unsafe fn SymbolsCreate(mut keyName: xkb_atom_t, mut symbols: *mut VarDef) -> *mut SymbolsDef {
    unsafe {
        let mut def: *mut SymbolsDef = Box::into_raw(Box::new(std::mem::zeroed::<SymbolsDef>()));
        (*def).common.type_0 = STMT_SYMBOLS;
        (*def).common.next = std::ptr::null_mut();
        (*def).merge = MERGE_DEFAULT;
        (*def).keyName = keyName;
        (*def).symbols = symbols;
        return def;
    }
}

pub unsafe fn GroupCompatCreate(mut group: i64, mut val: *mut ExprDef) -> *mut GroupCompatDef {
    unsafe {
        let mut def: *mut GroupCompatDef =
            Box::into_raw(Box::new(std::mem::zeroed::<GroupCompatDef>()));
        (*def).common.type_0 = STMT_GROUP_COMPAT;
        (*def).common.next = std::ptr::null_mut();
        (*def).merge = MERGE_DEFAULT;
        (*def).group = group;
        (*def).def = val as *mut ExprDef;
        return def;
    }
}

pub unsafe fn ModMapCreate(mut modifier: xkb_atom_t, mut keys: *mut ExprDef) -> *mut ModMapDef {
    unsafe {
        let mut def: *mut ModMapDef = Box::into_raw(Box::new(std::mem::zeroed::<ModMapDef>()));
        (*def).common.type_0 = STMT_MODMAP;
        (*def).common.next = std::ptr::null_mut();
        (*def).merge = MERGE_DEFAULT;
        (*def).modifier = modifier;
        (*def).keys = keys as *mut ExprDef;
        return def;
    }
}

pub unsafe fn LedMapCreate(mut name: xkb_atom_t, mut body: *mut VarDef) -> *mut LedMapDef {
    unsafe {
        let mut def: *mut LedMapDef = Box::into_raw(Box::new(std::mem::zeroed::<LedMapDef>()));
        (*def).common.type_0 = STMT_LED_MAP;
        (*def).common.next = std::ptr::null_mut();
        (*def).merge = MERGE_DEFAULT;
        (*def).name = name;
        (*def).body = body;
        return def;
    }
}

pub unsafe fn LedNameCreate(
    mut ndx: i64,
    mut name: *mut ExprDef,
    mut virtual_0: bool,
) -> *mut LedNameDef {
    unsafe {
        let mut def: *mut LedNameDef = Box::into_raw(Box::new(std::mem::zeroed::<LedNameDef>()));
        (*def).common.type_0 = STMT_LED_NAME;
        (*def).common.next = std::ptr::null_mut();
        (*def).merge = MERGE_DEFAULT;
        (*def).ndx = ndx;
        (*def).name = name as *mut ExprDef;
        (*def).virtual_0 = virtual_0;
        return def;
    }
}

pub unsafe fn UnknownStatementCreate(
    mut type_0: stmt_type,
    mut name: sval,
) -> *mut UnknownStatement {
    unsafe {
        let mut def: *mut UnknownStatement =
            Box::into_raw(Box::new(std::mem::zeroed::<UnknownStatement>()));
        (*def).common.type_0 = type_0;
        (*def).common.next = std::ptr::null_mut();
        (*def).name = cstr_ndup(name.start, name.len);
        if (*def).name.is_null() {
            drop(Box::from_raw(def));
            return std::ptr::null_mut();
        }
        return def;
    }
}

pub unsafe fn IncludeCreate(
    mut ctx: *mut xkb_context,
    mut str: *mut i8,
    mut merge: merge_mode,
) -> *mut IncludeStmt {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut incl: *mut IncludeStmt = std::ptr::null_mut();
        let mut first: *mut IncludeStmt = std::ptr::null_mut();
        let mut stmt: *mut i8 = std::ptr::null_mut();
        let mut tmp: *mut i8 = std::ptr::null_mut();
        let mut nextop: i8 = 0;
        first = std::ptr::null_mut();
        incl = first;
        tmp = str;
        stmt = strdup_safe(str);
        loop {
            if !(!tmp.is_null() && *tmp as i32 != 0) {
                c2rust_current_block = 15125582407903384992;
                break;
            }
            let mut file: *mut i8 = std::ptr::null_mut();
            let mut map: *mut i8 = std::ptr::null_mut();
            let mut extra_data: *mut i8 = std::ptr::null_mut();
            if !ParseIncludeMap(
                &raw mut tmp,
                &raw mut file,
                &raw mut map,
                &raw mut nextop,
                &raw mut extra_data,
            ) {
                c2rust_current_block = 15017566315148339459;
                break;
            }
            if isempty(file) {
                cstr_free(file);
                cstr_free(map);
                cstr_free(extra_data);
            } else {
                if first.is_null() {
                    incl = Box::into_raw(Box::new(std::mem::zeroed::<IncludeStmt>()));
                    first = incl;
                } else {
                    (*incl).next_incl = Box::into_raw(Box::new(std::mem::zeroed::<IncludeStmt>()))
                        as *mut _IncludeStmt;
                    incl = (*incl).next_incl as *mut IncludeStmt;
                }
                if incl.is_null() {
                    cstr_free(file);
                    cstr_free(map);
                    cstr_free(extra_data);
                    c2rust_current_block = 15125582407903384992;
                    break;
                } else {
                    (*incl).common.type_0 = STMT_INCLUDE;
                    (*incl).common.next = std::ptr::null_mut();
                    (*incl).merge = merge;
                    (*incl).stmt = std::ptr::null_mut();
                    (*incl).file = file;
                    (*incl).map = map;
                    (*incl).modifier = extra_data;
                    (*incl).next_incl = std::ptr::null_mut();
                    match nextop as i32 {
                        MERGE_AUGMENT_PREFIX => {
                            merge = MERGE_AUGMENT;
                        }
                        MERGE_REPLACE_PREFIX => {
                            merge = MERGE_REPLACE;
                        }
                        _ => {
                            merge = MERGE_OVERRIDE;
                        }
                    }
                }
            }
        }
        match c2rust_current_block {
            15017566315148339459 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal include statement \"{}\"; Ignored\n",
                    XKB_ERROR_INVALID_INCLUDE_STATEMENT as i32,
                    crate::xkb::utils::CStrDisplay(stmt),
                );
                FreeInclude(first);
                cstr_free(stmt);
                return std::ptr::null_mut();
            }
            _ => {
                if !first.is_null() {
                    (*first).stmt = stmt;
                } else {
                    cstr_free(stmt);
                }
                return first;
            }
        };
    }
}

pub unsafe fn XkbFileCreate(
    mut type_0: xkb_file_type,
    mut name: *mut i8,
    mut defs: *mut ParseCommon,
    mut flags: xkb_map_flags,
) -> *mut XkbFile {
    unsafe {
        let mut file: *mut XkbFile = std::ptr::null_mut();
        file = Box::into_raw(Box::new(std::mem::zeroed::<XkbFile>()));
        XkbEscapeMapName(name);
        (*file).file_type = type_0;
        (*file).name = name;
        (*file).defs = defs;
        (*file).flags = flags;
        return file;
    }
}

pub unsafe fn XkbFileFromComponents(
    mut ctx: *mut xkb_context,
    mut kkctgs: *const xkb_component_names,
) -> *mut XkbFile {
    unsafe {
        let mut c2rust_current_block: u64;
        let components: [*mut i8; 4] = [
            (*kkctgs).keycodes,
            (*kkctgs).types,
            (*kkctgs).compatibility,
            (*kkctgs).symbols,
        ];
        let mut type_0: xkb_file_type = FILE_TYPE_KEYCODES;
        let mut include: *mut IncludeStmt = std::ptr::null_mut();
        let mut file: *mut XkbFile = std::ptr::null_mut();
        let mut defs: *mut ParseCommon = std::ptr::null_mut();
        let mut defsLast: *mut ParseCommon = std::ptr::null_mut();
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        loop {
            if !(type_0 as u32 <= LAST_KEYMAP_FILE_TYPE as u32) {
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
        match c2rust_current_block {
            13536709405535804910 => {
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
            _ => {}
        }
        FreeXkbFile(defs as *mut XkbFile);
        return std::ptr::null_mut();
    }
}
unsafe fn FreeInclude(mut incl: *mut IncludeStmt) {
    unsafe {
        let mut next: *mut IncludeStmt = std::ptr::null_mut();
        while !incl.is_null() {
            next = (*incl).next_incl as *mut IncludeStmt;
            cstr_free((*incl).file);
            cstr_free((*incl).map);
            cstr_free((*incl).modifier);
            cstr_free((*incl).stmt);
            drop(Box::from_raw(incl));
            incl = next;
        }
    }
}

pub unsafe fn FreeStmt(mut stmt: *mut ParseCommon) {
    unsafe {
        let mut next: *mut ParseCommon = std::ptr::null_mut();
        while !stmt.is_null() {
            next = (*stmt).next as *mut ParseCommon;
            match (*stmt).type_0 as u32 {
                1 => {
                    FreeInclude(stmt as *mut IncludeStmt);
                    stmt = std::ptr::null_mut();
                }
                23 | 25 | 22 | 24 => {
                    FreeStmt((*(stmt as *mut ExprUnary)).child as *mut ParseCommon);
                }
                20 | 17 | 18 | 19 | 21 => {
                    FreeStmt((*(stmt as *mut ExprBinary)).left as *mut ParseCommon);
                    FreeStmt((*(stmt as *mut ExprBinary)).right as *mut ParseCommon);
                }
                11 => {
                    FreeStmt((*(stmt as *mut ExprAction)).args as *mut ParseCommon);
                }
                16 => {
                    FreeStmt((*(stmt as *mut ExprActionList)).actions as *mut ParseCommon);
                }
                13 => {
                    FreeStmt((*(stmt as *mut ExprArrayRef)).entry as *mut ParseCommon);
                }
                15 => {
                    // Drop the Vec to free its buffer
                    let ksl = stmt as *mut ExprKeysymList;
                    std::ptr::drop_in_place(&raw mut (*ksl).syms);
                }
                26 => {
                    FreeStmt((*(stmt as *mut VarDef)).name as *mut ParseCommon);
                    FreeStmt((*(stmt as *mut VarDef)).value as *mut ParseCommon);
                }
                27 => {
                    FreeStmt((*(stmt as *mut KeyTypeDef)).body as *mut ParseCommon);
                }
                28 => {
                    FreeStmt((*(stmt as *mut InterpDef)).match_0 as *mut ParseCommon);
                    FreeStmt((*(stmt as *mut InterpDef)).def as *mut ParseCommon);
                }
                29 => {
                    FreeStmt((*(stmt as *mut VModDef)).value as *mut ParseCommon);
                }
                30 => {
                    FreeStmt((*(stmt as *mut SymbolsDef)).symbols as *mut ParseCommon);
                }
                31 => {
                    FreeStmt((*(stmt as *mut ModMapDef)).keys as *mut ParseCommon);
                }
                32 => {
                    FreeStmt((*(stmt as *mut GroupCompatDef)).def as *mut ParseCommon);
                }
                33 => {
                    FreeStmt((*(stmt as *mut LedMapDef)).body as *mut ParseCommon);
                }
                34 => {
                    FreeStmt((*(stmt as *mut LedNameDef)).name as *mut ParseCommon);
                }
                35 | 36 => {
                    cstr_free((*(stmt as *mut UnknownStatement)).name);
                }
                _ => {}
            }
            // Deallocate the stmt with the correct type (Box::from_raw must match Box::into_raw type)
            if !stmt.is_null() {
                match (*stmt).type_0 as u32 {
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

pub unsafe fn FreeXkbFile(mut file: *mut XkbFile) {
    unsafe {
        let mut next: *mut XkbFile = std::ptr::null_mut();
        while !file.is_null() {
            next = (*file).common.next as *mut XkbFile;
            match (*file).file_type as u32 {
                5 => {
                    FreeXkbFile((*file).defs as *mut XkbFile);
                }
                1 | 2 | 3 | 0 | 4 => {
                    FreeStmt((*file).defs);
                }
                _ => {}
            }
            cstr_free((*file).name);
            drop(Box::from_raw(file));
            file = next;
        }
    }
}
static mut xkb_file_type_strings: [*const i8; 7] = [
    b"xkb_keycodes\0".as_ptr() as *const i8,
    b"xkb_types\0".as_ptr() as *const i8,
    b"xkb_compatibility\0".as_ptr() as *const i8,
    b"xkb_symbols\0".as_ptr() as *const i8,
    b"xkb_geometry\0".as_ptr() as *const i8,
    b"xkb_keymap\0".as_ptr() as *const i8,
    b"rules\0".as_ptr() as *const i8,
];

pub unsafe fn xkb_file_type_to_string(mut type_0: xkb_file_type) -> *const i8 {
    unsafe {
        if type_0 as u32 >= _FILE_TYPE_NUM_ENTRIES as u32 {
            return b"unknown\0".as_ptr() as *const i8;
        }
        return xkb_file_type_strings[type_0 as usize];
    }
}
static mut stmt_type_strings: [*const i8; 37] = [
    b"unknown statement\0".as_ptr() as *const i8,
    b"include statement\0".as_ptr() as *const i8,
    b"key name definition\0".as_ptr() as *const i8,
    b"key alias definition\0".as_ptr() as *const i8,
    b"string literal expression\0".as_ptr() as *const i8,
    b"integer literal expression\0".as_ptr() as *const i8,
    b"float literal expression\0".as_ptr() as *const i8,
    b"boolean literal expression\0".as_ptr() as *const i8,
    b"key name expression\0".as_ptr() as *const i8,
    b"keysym expression\0".as_ptr() as *const i8,
    b"identifier expression\0".as_ptr() as *const i8,
    b"action declaration expression\0".as_ptr() as *const i8,
    b"field reference expression\0".as_ptr() as *const i8,
    b"array reference expression\0".as_ptr() as *const i8,
    b"empty list expression\0".as_ptr() as *const i8,
    b"keysym list expression\0".as_ptr() as *const i8,
    b"action list expression\0".as_ptr() as *const i8,
    b"addition expression\0".as_ptr() as *const i8,
    b"substraction expression\0".as_ptr() as *const i8,
    b"multiplication expression\0".as_ptr() as *const i8,
    b"division expression\0".as_ptr() as *const i8,
    b"assignment expression\0".as_ptr() as *const i8,
    b"logical negation expression\0".as_ptr() as *const i8,
    b"arithmetic negation expression\0".as_ptr() as *const i8,
    b"bitwise inversion expression\0".as_ptr() as *const i8,
    b"unary plus expression\0".as_ptr() as *const i8,
    b"variable definition\0".as_ptr() as *const i8,
    b"key type definition\0".as_ptr() as *const i8,
    b"symbol interpretation definition\0".as_ptr() as *const i8,
    b"virtual modifiers definition\0".as_ptr() as *const i8,
    b"key symbols definition\0".as_ptr() as *const i8,
    b"modifier map declaration\0".as_ptr() as *const i8,
    b"group declaration\0".as_ptr() as *const i8,
    b"indicator map declaration\0".as_ptr() as *const i8,
    b"indicator name declaration\0".as_ptr() as *const i8,
    b"unknown declaration statement\0".as_ptr() as *const i8,
    b"unknown compound statement\0".as_ptr() as *const i8,
];

pub unsafe fn stmt_type_to_string(mut type_0: stmt_type) -> *const i8 {
    unsafe {
        if type_0 as u32 >= _STMT_NUM_VALUES as u32 {
            return std::ptr::null();
        }
        return stmt_type_strings[type_0 as usize];
    }
}

pub unsafe fn stmt_type_to_operator_char(mut type_0: stmt_type) -> i8 {
    match type_0 as u32 {
        17 => return '+' as i32 as i8,
        18 => return '-' as i32 as i8,
        19 => return '*' as i32 as i8,
        20 => return '/' as i32 as i8,
        22 => return '!' as i32 as i8,
        23 => return '-' as i32 as i8,
        24 => return '~' as i32 as i8,
        25 => return '+' as i32 as i8,
        _ => return '\0' as i32 as i8,
    };
}
use crate::xkb::keysym_utf::xkb_utf32_to_keysym;
use crate::xkb::shared_types::*;
