use crate::xkb::context::xkb_atom_intern;
pub const XKB_KEY_VoidSymbol: i32 = 0xffffff_i32;
pub const XKB_KEY_0: i32 = 0x30_i32;
pub const XKB_KEY_section: i32 = 0xa7_i32;
use crate::xkb::keysym::{xkb_keysym_from_name, xkb_keysym_is_deprecated};

pub const XKB_KEYSYM_MIN: i32 = 0;

use crate::xkb::shared_ast_types::{
    box_from_raw, collect_vardefs, from_common, to_common, to_common_or_null,
};
pub use crate::xkb::xkbcomp::ast_build::{
    expr_create, BoolVarCreate, ExprAppendKeySymList, ExprCreateKeySymList, FreeStmt,
    GroupCompatCreate, IncludeCreate, InterpCreate, KeyAliasCreate, KeyTypeCreate, KeycodeCreate,
    LedMapCreate, LedNameCreate, ModMapCreate, SymbolsCreate, VModCreate, VarCreate, XkbFileCreate,
};

pub fn ExprKeySymListAppendString(
    param: *mut scanner,
    expr: *mut ExprDef,
    string: *const i8,
) -> *mut ExprDef {
    unsafe {
        crate::xkb::xkbcomp::ast_build::ExprKeySymListAppendString(
            param as *mut _,
            expr as *mut _,
            string,
        ) as *mut ExprDef
    }
}

pub fn KeysymParseString(scanner: *mut scanner, string: *const i8) -> u32 {
    unsafe { crate::xkb::xkbcomp::ast_build::KeysymParseString(scanner as *mut _, string) }
}

pub fn UnknownStatementCreate(
    type_0: crate::xkb::shared_ast_types::stmt_type,
    name: sval,
) -> *mut crate::xkb::shared_ast_types::UnknownStatement {
    unsafe {
        crate::xkb::xkbcomp::ast_build::UnknownStatementCreate(
            type_0,
            *(&name as *const sval as *const _),
        ) as *mut _
    }
}
pub fn _xkbcommon_lex(yylval: *mut YYSTYPE, scanner: *mut scanner) -> i32 {
    unsafe { crate::xkb::xkbcomp::scanner::_xkbcommon_lex(yylval as *mut _, scanner as *mut _) }
}

pub use super::scanner::parser_h::{
    YYerror, ACTION_TOK, ALIAS, ALPHANUMERIC_KEYS, ALTERNATE, ALTERNATE_GROUP, AUGMENT, CBRACE,
    CBRACKET, COMMA, CPAREN, DECIMAL_DIGIT, DEFAULT, DIVIDE, DOT, END_OF_FILE, EQUALS, ERROR_TOK,
    EXCLAM, FLOAT, FUNCTION_KEYS, GROUP, HIDDEN, IDENT, INCLUDE, INDICATOR, INTEGER, INTERPRET,
    INVERT, KEY, KEYNAME, KEYPAD_KEYS, KEYS, LOGO, MINUS, MODIFIER_KEYS, MODIFIER_MAP, OBRACE,
    OBRACKET, OPAREN, OUTLINE, OVERLAY, OVERRIDE, PARTIAL, PLUS, REPLACE, ROW, SECTION, SEMI,
    SHAPE, SOLID, STRING, TEXT, TIMES, TYPE, VIRTUAL, VIRTUAL_MODS, XKB_COMPATMAP, XKB_GEOMETRY,
    XKB_KEYCODES, XKB_KEYMAP, XKB_LAYOUT, XKB_SEMANTICS, XKB_SYMBOLS, XKB_TYPES, YYEMPTY, YYSTYPE,
    YYUNDEF,
};
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
pub use crate::xkb::scanner_utils::{isvaleq, scanner, scanner_loc, sval};
pub use crate::xkb::shared_ast_types::{collect_exprs, safe_map_name, FreeXkbFile};
pub use crate::xkb::shared_ast_types::{
    ExprDef, ExprKind, GroupCompatDef, IncludeStmt, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef,
    LedMapDef, LedNameDef, ModMapDef, ParseCommon, SymbolsDef, UnknownStatement, VModDef, VarDef,
    XkbFile, _IncludeStmt, _ParseCommon, merge_mode, stmt_type, xkb_map_flags, FILE_TYPE_COMPAT,
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
use crate::xkb::utils::cstr_len;
use libc::{free, malloc};
#[derive(Clone)]
pub struct parser_param {
    pub ctx: *mut xkb_context,
    pub scanner: *mut scanner,
    pub rtrn: *mut XkbFile,
    pub more_maps: bool,
}
pub type yy_state_t = yytype_int16;
pub type yytype_int16 = ::core::ffi::c_short;
pub type yysymbol_kind_t = i32;
pub const YYSYMBOL_MapName: yysymbol_kind_t = 148;
pub const YYSYMBOL_OptMapName: yysymbol_kind_t = 147;
pub const YYSYMBOL_String: yysymbol_kind_t = 146;
pub const YYSYMBOL_Ident: yysymbol_kind_t = 145;
pub const YYSYMBOL_KeyCode: yysymbol_kind_t = 144;
pub const YYSYMBOL_Integer: yysymbol_kind_t = 143;
pub const YYSYMBOL_Float: yysymbol_kind_t = 142;
pub const YYSYMBOL_Number: yysymbol_kind_t = 141;
pub const YYSYMBOL_SignedNumber: yysymbol_kind_t = 140;
pub const YYSYMBOL_KeySymLit: yysymbol_kind_t = 139;
pub const YYSYMBOL_KeySym: yysymbol_kind_t = 138;
pub const YYSYMBOL_KeySyms: yysymbol_kind_t = 137;
pub const YYSYMBOL_NonEmptyKeySyms: yysymbol_kind_t = 136;
pub const YYSYMBOL_KeySymList: yysymbol_kind_t = 135;
pub const YYSYMBOL_MultiKeySymList: yysymbol_kind_t = 134;
pub const YYSYMBOL_Terminal: yysymbol_kind_t = 133;
pub const YYSYMBOL_OptTerminal: yysymbol_kind_t = 132;
pub const YYSYMBOL_Lhs: yysymbol_kind_t = 131;
pub const YYSYMBOL_Action: yysymbol_kind_t = 130;
pub const YYSYMBOL_Actions: yysymbol_kind_t = 129;
pub const YYSYMBOL_NonEmptyActions: yysymbol_kind_t = 128;
pub const YYSYMBOL_ActionList: yysymbol_kind_t = 127;
pub const YYSYMBOL_MultiActionList: yysymbol_kind_t = 126;
pub const YYSYMBOL_Term: yysymbol_kind_t = 125;
pub const YYSYMBOL_Expr: yysymbol_kind_t = 124;
pub const YYSYMBOL_ExprList: yysymbol_kind_t = 123;
pub const YYSYMBOL_MergeMode: yysymbol_kind_t = 122;
pub const YYSYMBOL_OptMergeMode: yysymbol_kind_t = 121;
pub const YYSYMBOL_Element: yysymbol_kind_t = 120;
pub const YYSYMBOL_FieldSpec: yysymbol_kind_t = 119;
pub const YYSYMBOL_DoodadType: yysymbol_kind_t = 118;
pub const YYSYMBOL_DoodadDecl: yysymbol_kind_t = 117;
pub const YYSYMBOL_Coord: yysymbol_kind_t = 116;
pub const YYSYMBOL_CoordList: yysymbol_kind_t = 115;
pub const YYSYMBOL_OutlineInList: yysymbol_kind_t = 114;
pub const YYSYMBOL_OutlineList: yysymbol_kind_t = 113;
pub const YYSYMBOL_OverlayKey: yysymbol_kind_t = 112;
pub const YYSYMBOL_OverlayKeyList: yysymbol_kind_t = 111;
pub const YYSYMBOL_OverlayDecl: yysymbol_kind_t = 110;
pub const YYSYMBOL_Key: yysymbol_kind_t = 109;
pub const YYSYMBOL_Keys: yysymbol_kind_t = 108;
pub const YYSYMBOL_RowBodyItem: yysymbol_kind_t = 107;
pub const YYSYMBOL_RowBody: yysymbol_kind_t = 106;
pub const YYSYMBOL_SectionBodyItem: yysymbol_kind_t = 105;
pub const YYSYMBOL_SectionBody: yysymbol_kind_t = 104;
pub const YYSYMBOL_SectionDecl: yysymbol_kind_t = 103;
pub const YYSYMBOL_ShapeDecl: yysymbol_kind_t = 102;
pub const YYSYMBOL_UnknownCompoundStatementDecl: yysymbol_kind_t = 101;
pub const YYSYMBOL_UnknownDecl: yysymbol_kind_t = 100;
pub const YYSYMBOL_LedNameDecl: yysymbol_kind_t = 99;
pub const YYSYMBOL_LedMapDecl: yysymbol_kind_t = 98;
pub const YYSYMBOL_KeyOrKeySym: yysymbol_kind_t = 97;
pub const YYSYMBOL_KeyOrKeySymList: yysymbol_kind_t = 96;
pub const YYSYMBOL_ModMapDecl: yysymbol_kind_t = 95;
pub const YYSYMBOL_GroupCompatDecl: yysymbol_kind_t = 94;
pub const YYSYMBOL_NoSymbolOrActionList: yysymbol_kind_t = 93;
pub const YYSYMBOL_MultiKeySymOrActionList: yysymbol_kind_t = 92;
pub const YYSYMBOL_SymbolsVarDecl: yysymbol_kind_t = 91;
pub const YYSYMBOL_SymbolsBody: yysymbol_kind_t = 90;
pub const YYSYMBOL_OptSymbolsBody: yysymbol_kind_t = 89;
pub const YYSYMBOL_SymbolsDecl: yysymbol_kind_t = 88;
pub const YYSYMBOL_KeyTypeDecl: yysymbol_kind_t = 87;
pub const YYSYMBOL_VarDeclList: yysymbol_kind_t = 86;
pub const YYSYMBOL_InterpretMatch: yysymbol_kind_t = 85;
pub const YYSYMBOL_InterpretDecl: yysymbol_kind_t = 84;
pub const YYSYMBOL_VModDef: yysymbol_kind_t = 83;
pub const YYSYMBOL_VModDefList: yysymbol_kind_t = 82;
pub const YYSYMBOL_VModDecl: yysymbol_kind_t = 81;
pub const YYSYMBOL_KeyAliasDecl: yysymbol_kind_t = 80;
pub const YYSYMBOL_KeyNameDecl: yysymbol_kind_t = 79;
pub const YYSYMBOL_VarDecl: yysymbol_kind_t = 78;
pub const YYSYMBOL_Decl: yysymbol_kind_t = 77;
pub const YYSYMBOL_DeclList: yysymbol_kind_t = 76;
pub const YYSYMBOL_Flag: yysymbol_kind_t = 75;
pub const YYSYMBOL_Flags: yysymbol_kind_t = 74;
pub const YYSYMBOL_OptFlags: yysymbol_kind_t = 73;
pub const YYSYMBOL_FileType: yysymbol_kind_t = 72;
pub const YYSYMBOL_XkbMapConfig: yysymbol_kind_t = 71;
pub const YYSYMBOL_XkbMapConfigList: yysymbol_kind_t = 70;
pub const YYSYMBOL_XkbCompositeType: yysymbol_kind_t = 69;
pub const YYSYMBOL_XkbCompositeMap: yysymbol_kind_t = 68;
pub const YYSYMBOL_XkbFile: yysymbol_kind_t = 67;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 66;
pub const YYSYMBOL_ALTERNATE_GROUP: yysymbol_kind_t = 65;
pub const YYSYMBOL_FUNCTION_KEYS: yysymbol_kind_t = 64;
pub const YYSYMBOL_KEYPAD_KEYS: yysymbol_kind_t = 63;
pub const YYSYMBOL_MODIFIER_KEYS: yysymbol_kind_t = 62;
pub const YYSYMBOL_ALPHANUMERIC_KEYS: yysymbol_kind_t = 61;
pub const YYSYMBOL_HIDDEN: yysymbol_kind_t = 60;
pub const YYSYMBOL_DEFAULT: yysymbol_kind_t = 59;
pub const YYSYMBOL_PARTIAL: yysymbol_kind_t = 58;
pub const YYSYMBOL_KEYNAME: yysymbol_kind_t = 57;
pub const YYSYMBOL_IDENT: yysymbol_kind_t = 56;
pub const YYSYMBOL_FLOAT: yysymbol_kind_t = 55;
pub const YYSYMBOL_INTEGER: yysymbol_kind_t = 54;
pub const YYSYMBOL_DECIMAL_DIGIT: yysymbol_kind_t = 53;
pub const YYSYMBOL_STRING: yysymbol_kind_t = 52;
pub const YYSYMBOL_INVERT: yysymbol_kind_t = 51;
pub const YYSYMBOL_EXCLAM: yysymbol_kind_t = 50;
pub const YYSYMBOL_SEMI: yysymbol_kind_t = 49;
pub const YYSYMBOL_COMMA: yysymbol_kind_t = 48;
pub const YYSYMBOL_DOT: yysymbol_kind_t = 47;
pub const YYSYMBOL_CBRACKET: yysymbol_kind_t = 46;
pub const YYSYMBOL_OBRACKET: yysymbol_kind_t = 45;
pub const YYSYMBOL_CPAREN: yysymbol_kind_t = 44;
pub const YYSYMBOL_OPAREN: yysymbol_kind_t = 43;
pub const YYSYMBOL_CBRACE: yysymbol_kind_t = 42;
pub const YYSYMBOL_OBRACE: yysymbol_kind_t = 41;
pub const YYSYMBOL_TIMES: yysymbol_kind_t = 40;
pub const YYSYMBOL_DIVIDE: yysymbol_kind_t = 39;
pub const YYSYMBOL_MINUS: yysymbol_kind_t = 38;
pub const YYSYMBOL_PLUS: yysymbol_kind_t = 37;
pub const YYSYMBOL_EQUALS: yysymbol_kind_t = 36;
pub const YYSYMBOL_VIRTUAL: yysymbol_kind_t = 35;
pub const YYSYMBOL_LOGO: yysymbol_kind_t = 34;
pub const YYSYMBOL_SOLID: yysymbol_kind_t = 33;
pub const YYSYMBOL_OUTLINE: yysymbol_kind_t = 32;
pub const YYSYMBOL_TEXT: yysymbol_kind_t = 31;
pub const YYSYMBOL_OVERLAY: yysymbol_kind_t = 30;
pub const YYSYMBOL_SECTION: yysymbol_kind_t = 29;
pub const YYSYMBOL_ROW: yysymbol_kind_t = 28;
pub const YYSYMBOL_KEYS: yysymbol_kind_t = 27;
pub const YYSYMBOL_SHAPE: yysymbol_kind_t = 26;
pub const YYSYMBOL_INDICATOR: yysymbol_kind_t = 25;
pub const YYSYMBOL_MODIFIER_MAP: yysymbol_kind_t = 24;
pub const YYSYMBOL_GROUP: yysymbol_kind_t = 23;
pub const YYSYMBOL_ALIAS: yysymbol_kind_t = 22;
pub const YYSYMBOL_KEY: yysymbol_kind_t = 21;
pub const YYSYMBOL_ACTION_TOK: yysymbol_kind_t = 20;
pub const YYSYMBOL_INTERPRET: yysymbol_kind_t = 19;
pub const YYSYMBOL_TYPE: yysymbol_kind_t = 18;
pub const YYSYMBOL_VIRTUAL_MODS: yysymbol_kind_t = 17;
pub const YYSYMBOL_ALTERNATE: yysymbol_kind_t = 16;
pub const YYSYMBOL_REPLACE: yysymbol_kind_t = 15;
pub const YYSYMBOL_AUGMENT: yysymbol_kind_t = 14;
pub const YYSYMBOL_OVERRIDE: yysymbol_kind_t = 13;
pub const YYSYMBOL_INCLUDE: yysymbol_kind_t = 12;
pub const YYSYMBOL_XKB_LAYOUT: yysymbol_kind_t = 11;
pub const YYSYMBOL_XKB_SEMANTICS: yysymbol_kind_t = 10;
pub const YYSYMBOL_XKB_GEOMETRY: yysymbol_kind_t = 9;
pub const YYSYMBOL_XKB_COMPATMAP: yysymbol_kind_t = 8;
pub const YYSYMBOL_XKB_SYMBOLS: yysymbol_kind_t = 7;
pub const YYSYMBOL_XKB_TYPES: yysymbol_kind_t = 6;
pub const YYSYMBOL_XKB_KEYCODES: yysymbol_kind_t = 5;
pub const YYSYMBOL_XKB_KEYMAP: yysymbol_kind_t = 4;
pub const YYSYMBOL_ERROR_TOK: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yytype_uint8 = ::core::ffi::c_uchar;
pub type yytype_int8 = i8;
pub type yy_state_fast_t = i32;
pub const YYENOMEM: i32 = -2;
#[derive(Copy, Clone)]
pub struct yypcontext_t {
    pub yyssp: *mut yy_state_t,
    pub yytoken: yysymbol_kind_t,
}
pub const YYARGS_MAX: u32 = 5;
#[derive(Copy, Clone)]
pub struct VModList {
    pub head: *mut ExprDef,
    pub last: *mut ExprDef,
}
#[derive(Copy, Clone)]
pub struct ExprList {
    pub head: *mut ExprDef,
    pub last: *mut ExprDef,
}
#[derive(Copy, Clone)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}

fn _xkbcommon_error(param: *mut parser_param, msg: *const i8) {
    unsafe {
        let loc: scanner_loc = (*(*param).scanner).token_location();
        let msg_str = std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(msg));
        log::error!(
            "[XKB-{:03}] {}:{}:{}: {}\n",
            XKB_ERROR_INVALID_XKB_SYNTAX as i32,
            &(*(*param).scanner).file_name,
            loc.line,
            loc.column,
            msg_str
        );
    }
}
fn resolve_keysym(param: *mut parser_param, name: sval, sym_rtrn: *mut u32) -> bool {
    unsafe {
        if isvaleq(
            name,
            sval {
                len: (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1_usize),
                start: b"any\0".as_ptr() as *const i8,
            },
        ) as i32
            != 0
            || isvaleq(
                name,
                sval {
                    len: (std::mem::size_of::<[i8; 9]>()).wrapping_sub(1_usize),
                    start: b"nosymbol\0".as_ptr() as *const i8,
                },
            ) as i32
                != 0
        {
            *sym_rtrn = XKB_KEY_NoSymbol as u32;
            return true;
        }
        if isvaleq(
            name,
            sval {
                len: (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1_usize),
                start: b"none\0".as_ptr() as *const i8,
            },
        ) as i32
            != 0
            || isvaleq(
                name,
                sval {
                    len: (std::mem::size_of::<[i8; 11]>()).wrapping_sub(1_usize),
                    start: b"voidsymbol\0".as_ptr() as *const i8,
                },
            ) as i32
                != 0
        {
            *sym_rtrn = XKB_KEY_VoidSymbol as u32;
            return true;
        }
        let mut buf: [i8; 31] = [0; 31];
        if name.len >= std::mem::size_of::<[i8; 31]>() {
            return false;
        }
        std::ptr::copy_nonoverlapping(name.start as *const u8, &raw mut buf as *mut u8, name.len);
        buf[name.len] = '\0' as i32 as i8;
        let sym: u32 = xkb_keysym_from_name(&raw mut buf as *mut i8, XKB_KEYSYM_NO_FLAGS);
        if sym != XKB_KEY_NoSymbol as u32 {
            *sym_rtrn = sym;
            if ((*(*param).ctx).log_verbosity >= 2_i32) as i32 as i64 != 0 {
                let mut ref_name: *const i8 = std::ptr::null();
                if xkb_keysym_is_deprecated(sym, &raw mut buf as *mut i8, &raw mut ref_name) {
                    let buf_str = std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                        &raw mut buf as *const i8,
                    ));
                    if ref_name.is_null() {
                        let loc: scanner_loc = (*(*param).scanner).token_location();
                        log::warn!(
                            "[XKB-{:03}] {}:{}:{}: deprecated keysym \"{}\".\n",
                            XKB_WARNING_DEPRECATED_KEYSYM as i32,
                            &(*(*param).scanner).file_name,
                            loc.line,
                            loc.column,
                            buf_str
                        );
                    } else {
                        let ref_str = std::str::from_utf8_unchecked(
                            crate::xkb::utils::cstr_as_bytes(ref_name),
                        );
                        let loc_0: scanner_loc = (*(*param).scanner).token_location();
                        log::warn!("[XKB-{:03}] {}:{}:{}: deprecated keysym name \"{}\"; please use \"{}\" instead.\n",
                            XKB_WARNING_DEPRECATED_KEYSYM_NAME as i32,
                            &(*(*param).scanner).file_name,
                            loc_0.line,
                            loc_0.column,
                            buf_str,
                            ref_str);
                    }
                }
            }
            return true;
        }
        false
    }
}
pub const YY_NULLPTR: *mut ::core::ffi::c_void = std::ptr::null_mut();
pub const YYSTACK_GAP_MAXIMUM: i64 = std::mem::size_of::<yyalloc>() as i64 - 1;
pub const YYFINAL: i32 = 16;
pub const YYLAST: i32 = 928;
pub const YYNTOKENS: i32 = 66;
pub const YYMAXUTOK: i32 = 257;
static YYTRANSLATE: [i8; 258] = [
    0, 4, 5, 6, 7, 8, 9, 10, 11, 2, 12, 13, 14, 15, 16, 2, 2, 2, 2, 2, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 2, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 51, 2, 2, 2, 2, 52, 53, 54, 55, 56, 57, 2, 2, 2, 2, 58, 59, 60, 61, 62, 63, 64,
    65, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1, 2,
];
fn yysymbol_name(yysymbol: yysymbol_kind_t) -> *const i8 {
    static YY_SNAME: [&str; 150] = [
        "end of file\0",
        "error\0",
        "invalid token\0",
        "invalid token\0",
        "xkb_keymap\0",
        "xkb_keycodes\0",
        "xkb_types\0",
        "xkb_symbols\0",
        "xkb_compatibility\0",
        "xkb_geometry\0",
        "xkb_semantics\0",
        "xkb_layout\0",
        "include\0",
        "override\0",
        "augment\0",
        "replace\0",
        "alternate\0",
        "virtual_modifiers\0",
        "type\0",
        "interpret\0",
        "action\0",
        "key\0",
        "alias\0",
        "group\0",
        "modifier_map\0",
        "indicator\0",
        "shape\0",
        "keys\0",
        "row\0",
        "section\0",
        "overlay\0",
        "text\0",
        "outline\0",
        "solid\0",
        "logo\0",
        "virtual\0",
        "=\0",
        "+\0",
        "-\0",
        "/\0",
        "*\0",
        "{\0",
        "}\0",
        "(\0",
        ")\0",
        "[\0",
        "]\0",
        ".\0",
        ",\0",
        ";\0",
        "!\0",
        "~\0",
        "string literal\0",
        "decimal digit\0",
        "integer literal\0",
        "float literal\0",
        "identifier\0",
        "key name\0",
        "partial\0",
        "default\0",
        "hidden\0",
        "alphanumeric_keys\0",
        "modifier_keys\0",
        "keypad_keys\0",
        "function_keys\0",
        "alternate_group\0",
        "$accept\0",
        "XkbFile\0",
        "XkbCompositeMap\0",
        "XkbCompositeType\0",
        "XkbMapConfigList\0",
        "XkbMapConfig\0",
        "FileType\0",
        "OptFlags\0",
        "Flags\0",
        "Flag\0",
        "DeclList\0",
        "Decl\0",
        "VarDecl\0",
        "KeyNameDecl\0",
        "KeyAliasDecl\0",
        "VModDecl\0",
        "VModDefList\0",
        "VModDef\0",
        "InterpretDecl\0",
        "InterpretMatch\0",
        "VarDeclList\0",
        "KeyTypeDecl\0",
        "SymbolsDecl\0",
        "OptSymbolsBody\0",
        "SymbolsBody\0",
        "SymbolsVarDecl\0",
        "MultiKeySymOrActionList\0",
        "NoSymbolOrActionList\0",
        "GroupCompatDecl\0",
        "ModMapDecl\0",
        "KeyOrKeySymList\0",
        "KeyOrKeySym\0",
        "LedMapDecl\0",
        "LedNameDecl\0",
        "UnknownDecl\0",
        "UnknownCompoundStatementDecl\0",
        "ShapeDecl\0",
        "SectionDecl\0",
        "SectionBody\0",
        "SectionBodyItem\0",
        "RowBody\0",
        "RowBodyItem\0",
        "Keys\0",
        "Key\0",
        "OverlayDecl\0",
        "OverlayKeyList\0",
        "OverlayKey\0",
        "OutlineList\0",
        "OutlineInList\0",
        "CoordList\0",
        "Coord\0",
        "DoodadDecl\0",
        "DoodadType\0",
        "FieldSpec\0",
        "Element\0",
        "OptMergeMode\0",
        "MergeMode\0",
        "ExprList\0",
        "Expr\0",
        "Term\0",
        "MultiActionList\0",
        "ActionList\0",
        "NonEmptyActions\0",
        "Actions\0",
        "Action\0",
        "Lhs\0",
        "OptTerminal\0",
        "Terminal\0",
        "MultiKeySymList\0",
        "KeySymList\0",
        "NonEmptyKeySyms\0",
        "KeySyms\0",
        "KeySym\0",
        "KeySymLit\0",
        "SignedNumber\0",
        "Number\0",
        "Float\0",
        "Integer\0",
        "KeyCode\0",
        "Ident\0",
        "String\0",
        "OptMapName\0",
        "MapName\0",
        "\0",
    ];
    YY_SNAME[yysymbol as usize].as_ptr() as *const i8
}
pub const YYPACT_NINF: i32 = -280_i32;
static YYPACT: [i16; 384] = [
    7, -280, -280, -280, -280, -280, -280, -280, -280, -280, 32, -280, -280, 578, 847, -280, -280,
    -280, -280, -280, -280, -280, -280, -280, -280, -12, -12, -280, -280, 22, -280, 36, -280, -280,
    463, 10, 53, -280, 458, -280, -280, -280, -280, -280, 57, -280, 25, 34, -280, -280, 64, 59,
    172, -280, 40, 61, 135, 64, 154, 59, -280, 59, 78, -280, -280, -280, 114, 64, 324, 120, -280,
    -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280,
    59, -18, -280, 134, 143, -280, -280, -30, -280, 175, -280, 179, -280, -280, -280, -280, -280,
    182, 190, -280, 197, 213, -280, -280, 248, 222, 263, 234, 237, 261, 135, 258, -280, -280, 276,
    293, -280, -280, -280, 142, 289, 332, 869, 332, -280, 64, -280, 332, -280, -280, 332, 597, 269,
    332, 60, 332, -280, 35, 461, 296, -280, -280, 332, -280, -280, 287, -280, -280, -280, -280,
    -280, -280, -280, -280, -280, -280, 332, 332, 825, 332, 332, 332, -6, 228, -280, -280, -280,
    301, -280, -280, 294, 103, -280, 433, 639, 654, 433, 478, 64, 306, 311, -280, -280, 318, -27,
    313, 233, -280, 13, -280, -280, 285, 696, 319, 96, 37, -280, 45, -280, 330, 59, 326, 59, -280,
    -280, 419, -280, -280, -280, 332, 711, 372, -280, 753, -280, -280, -280, -280, 325, 48, -280,
    418, -280, -280, 332, 332, 332, 332, 332, -280, 332, 332, -280, 322, -280, 323, 331, 520, -280,
    337, 130, 133, -280, -280, 170, -280, -280, -280, 341, 597, 290, -280, -280, 343, 60, -280,
    344, 56, 189, -280, -280, -280, 346, -280, 355, -25, 358, 319, 377, 773, 375, 364, -280, 386,
    368, -280, 370, 332, -280, 869, -280, -38, 433, 253, 253, -280, -280, 433, 266, -280, -280,
    -280, -280, 67, -280, -280, 540, -280, 845, -280, 161, -280, -280, -280, 433, -280, -280, -280,
    -280, -280, 96, -280, -280, -280, -280, 796, 433, 381, -280, 227, -280, 384, -280, -280, -280,
    -280, 30, -280, -280, 332, -280, -280, 208, 582, 239, 242, -280, -280, 180, -280, -280, -280,
    400, 89, -24, 405, -280, 423, 112, -280, -280, 433, -280, -280, -280, -280, -280, -280, -280,
    -280, 332, -280, 113, -280, -280, 403, 425, 384, 117, 427, -24, -280, -280, -280, -280, -280,
    -280,
];
static YYDEFACT: [u8; 384] = [
    18, 4, 21, 22, 23, 24, 25, 26, 27, 28, 0, 2, 3, 0, 17, 20, 1, 6, 12, 13, 15, 14, 16, 7, 8, 218,
    218, 19, 219, 0, 217, 0, 10, 31, 18, 142, 0, 9, 0, 143, 145, 144, 146, 147, 0, 29, 0, 141, 5,
    11, 0, 132, 131, 130, 133, 0, 134, 135, 136, 137, 138, 139, 140, 125, 126, 127, 0, 0, 214, 0,
    215, 32, 34, 35, 30, 33, 36, 37, 39, 38, 40, 41, 45, 46, 42, 43, 44, 0, 176, 129, 0, 128, 47,
    214, 0, 55, 56, 216, 0, 201, 199, 202, 203, 200, 0, 60, 198, 0, 0, 211, 210, 0, 0, 0, 0, 0, 0,
    0, 0, 209, 185, 0, 180, 184, 183, 182, 0, 0, 0, 0, 0, 49, 0, 53, 0, 62, 62, 0, 66, 0, 0, 0, 0,
    62, 0, 0, 0, 50, 62, 0, 213, 212, 0, 62, 132, 131, 133, 134, 135, 136, 137, 139, 140, 0, 0, 0,
    0, 0, 0, 176, 0, 156, 173, 163, 161, 164, 128, 177, 0, 54, 57, 0, 0, 59, 81, 0, 0, 65, 68, 73,
    0, 128, 0, 0, 86, 0, 85, 87, 0, 0, 0, 0, 0, 116, 0, 121, 0, 136, 138, 0, 99, 101, 0, 97, 102,
    100, 0, 0, 0, 51, 0, 158, 161, 157, 174, 0, 0, 171, 0, 159, 160, 150, 0, 0, 0, 0, 178, 0, 0,
    48, 0, 61, 0, 201, 0, 195, 200, 0, 0, 169, 168, 0, 189, 188, 72, 0, 0, 0, 52, 82, 0, 0, 89, 0,
    0, 0, 207, 208, 206, 0, 205, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 91, 0, 150, 172, 0, 165, 0, 149,
    152, 153, 151, 154, 155, 0, 63, 58, 80, 193, 0, 192, 78, 0, 76, 0, 74, 0, 64, 67, 70, 69, 83,
    84, 88, 117, 204, 0, 93, 115, 94, 120, 0, 119, 0, 106, 0, 104, 0, 95, 90, 92, 123, 0, 170, 162,
    0, 179, 194, 0, 0, 0, 0, 167, 166, 0, 196, 187, 186, 0, 0, 0, 0, 103, 0, 0, 113, 175, 148, 191,
    190, 79, 77, 75, 197, 122, 118, 150, 109, 0, 108, 98, 0, 0, 0, 0, 0, 0, 114, 111, 112, 110,
    105, 107,
];
static YYPGOTO: [yytype_int16; 83] = [
    -280, -280, -280, -280, -280, 434, -280, 443, -280, 469, -280, -280, -45, -280, -280, -280,
    -280, 356, -280, -280, 51, -280, -280, -280, -280, 244, 251, -280, -280, -280, -280, 249, 466,
    -280, -280, -280, -280, -280, -280, 302, -280, 187, -280, 138, -280, -280, 144, -280, 257,
    -196, 259, 470, -280, -46, -280, -280, -280, -279, 63, 5, 232, -280, -176, 231, -181, -35,
    -280, 474, 247, -280, 240, -280, 500, -182, 236, 291, -280, -44, -280, -37, -23, 528, -280,
];
static YYDEFGOTO: [yytype_int16; 83] = [
    0, 10, 11, 25, 34, 12, 26, 13, 14, 15, 35, 45, 241, 72, 73, 74, 94, 95, 75, 104, 181, 76, 77,
    186, 187, 188, 189, 247, 78, 79, 195, 196, 211, 81, 82, 83, 84, 85, 212, 213, 326, 327, 369,
    370, 214, 355, 356, 202, 203, 204, 205, 215, 87, 169, 89, 46, 47, 288, 289, 171, 248, 226, 172,
    173, 227, 174, 121, 175, 251, 300, 252, 347, 197, 106, 269, 270, 123, 124, 152, 176, 125, 29,
    30,
];
static YYTABLE: [yytype_int16; 929] = [
    88, 71, 253, 250, 264, 333, 335, 1, 249, 91, 336, 90, 111, 96, 113, -71, 200, 367, 132, 133,
    112, -71, 39, 40, 41, 42, 43, 128, 98, 129, 118, 93, 16, 368, 70, 114, 115, 231, 116, 128, 28,
    129, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 44, 60, 61, 260, 62, 63, 64, 65, 66, 261, 301, 32,
    127, 2, 3, 4, 5, 6, 7, 8, 9, 146, 357, 67, 200, 33, 336, 271, 201, 68, 69, 177, 70, 272, 92,
    273, 375, 99, 285, 93, 88, 274, 70, 96, 286, 107, 315, 88, 210, 191, 48, 190, 274, 334, 49,
    206, 91, 338, 90, 97, 100, 101, 102, 339, 103, 194, 108, 225, 93, 253, 250, 70, 344, 348, 350,
    249, 222, 222, -124, 366, 222, 222, 265, 88, 88, 274, 225, 117, 232, 233, 234, 235, 91, 91, 90,
    90, 254, 266, 267, 268, 239, 88, 373, 376, 126, 360, 301, 381, 374, 377, 91, 301, 90, 336, 88,
    210, 221, 223, 130, 88, 229, 230, 88, 91, 302, 90, 303, 304, 91, 305, 90, 91, 114, 90, 277,
    182, 109, 110, 99, 170, 131, 178, 199, 150, 151, 180, 225, 217, 183, 99, 345, 193, 220, 198,
    97, 109, 110, 99, 88, 134, 218, 245, 101, 102, 306, 103, 307, 191, 135, 190, 364, 136, 100,
    101, 102, 137, 103, 228, 88, 325, 299, 101, 102, 206, 103, 99, 138, 91, 225, 90, 266, 267, 268,
    154, 155, 53, 156, 139, 157, 158, 159, 160, 324, 60, 161, 225, 162, 225, 359, 101, 102, 141,
    103, 232, 233, 234, 235, 352, 232, 233, 234, 235, 236, 143, 225, 67, 144, 280, 88, 325, 259,
    93, 140, 362, 70, 305, 363, 91, 307, 90, 234, 235, 225, 290, 291, 292, 293, 142, 294, 295, 145,
    232, 233, 234, 235, 147, 154, 155, 53, 156, 337, 157, 158, 159, 160, 148, 60, 161, 311, 162,
    232, 233, 234, 235, 192, 163, 164, 149, 153, 165, 216, 166, 262, 184, 219, 237, 323, 238, 167,
    168, 97, 109, 110, 119, 93, 120, 255, 70, 154, 155, 53, 156, 257, 157, 158, 159, 160, 256, 60,
    161, 258, 162, 201, -181, 275, 276, 284, 163, 164, 296, 297, 165, -139, 166, 97, 109, 110, 119,
    -214, 120, 167, 168, 97, 109, 110, 119, 93, 120, 308, 70, 312, 314, 317, 154, 155, 53, 156,
    358, 157, 158, 159, 160, 318, 60, 161, 320, 162, 232, 233, 234, 235, 329, 163, 164, 328, 331,
    322, 332, 166, 282, 351, 232, 233, 234, 235, 167, 168, 97, 109, 110, 119, 93, 120, 330, 70,
    154, 155, 53, 156, 354, 157, 158, 207, 160, 365, 208, 161, 209, 62, 63, 64, 65, 371, 232, 233,
    234, 235, 372, 378, 278, 287, 18, 19, 20, 21, 22, 37, 67, 232, 233, 234, 235, 379, 93, 382, 38,
    70, 154, 155, 53, 156, 27, 157, 158, 207, 160, 179, 208, 161, 209, 62, 63, 64, 65, 154, 155,
    53, 156, 309, 157, 158, 159, 160, 36, 60, 243, 310, 162, 313, 67, 80, 353, 279, 383, 86, 93,
    380, 244, 70, 2, 3, 4, 5, 6, 7, 8, 9, 319, 245, 101, 102, 321, 246, 341, 343, 70, 154, 155, 53,
    156, 122, 157, 158, 159, 160, 346, 60, 243, 342, 162, 105, 349, 31, 0, 316, 0, 154, 155, 53,
    156, 298, 157, 158, 159, 160, 0, 60, 243, 0, 162, 299, 101, 102, 0, 246, 0, 0, 70, 0, 340, 17,
    18, 19, 20, 21, 22, 23, 24, 0, 0, 245, 101, 102, 0, 246, 0, 0, 70, 154, 155, 53, 156, 0, 157,
    158, 159, 160, 0, 60, 243, 0, 162, 0, 154, 155, 53, 156, 0, 157, 158, 159, 160, 361, 60, 161,
    0, 162, 0, 0, 0, 0, 0, 299, 101, 102, 0, 246, 0, 0, 70, 184, 0, 0, 0, 0, 185, 0, 0, 0, 0, 0,
    93, 0, 0, 70, 154, 155, 53, 156, 0, 157, 158, 159, 160, 0, 60, 161, 0, 162, 0, 154, 155, 53,
    156, 0, 157, 158, 159, 160, 240, 60, 161, 0, 162, 0, 0, 0, 67, 0, 0, 0, 0, 0, 93, 242, 0, 70,
    0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 93, 0, 0, 70, 154, 155, 53, 156, 0, 157, 158, 159, 160, 0,
    60, 161, 0, 162, 0, 154, 155, 53, 156, 0, 157, 158, 159, 160, 263, 60, 161, 0, 162, 0, 0, 0,
    67, 0, 0, 0, 0, 0, 93, 281, 0, 70, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 93, 0, 0, 70, 154, 155,
    53, 156, 0, 157, 158, 159, 160, 0, 60, 161, 0, 162, 0, 0, 0, 0, 0, 0, 154, 155, 53, 156, 283,
    157, 158, 159, 160, 324, 60, 161, 67, 162, 0, 0, 0, 0, 93, 0, 0, 70, 0, 154, 155, 53, 156, 0,
    157, 158, 159, 160, 67, 60, 161, 0, 162, 0, 93, 0, 0, 70, 0, 0, 0, 0, 0, 224, 0, 0, 201, 0,
    154, 155, 53, 156, 0, 157, 158, 159, 160, 93, 60, 161, 70, 162, 0, 0, 0, 0, 0, 0, 154, 155, 53,
    156, 224, 157, 158, 159, 160, 0, 60, 161, 0, 162, 0, 0, 0, 0, 93, 0, 0, 70, 0, 165, 154, 155,
    53, 156, 0, 157, 158, 159, 160, 0, 60, 161, 0, 162, 93, 0, 0, 70, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0, 70,
];
static YYCHECK: [yytype_int16; 929] = [
    46, 46, 184, 184, 200, 284, 44, 0, 184, 46, 48, 46, 56, 50, 58, 42, 41, 41, 48, 49, 57, 48, 12,
    13, 14, 15, 16, 45, 51, 47, 67, 56, 0, 57, 59, 58, 59, 43, 61, 45, 52, 47, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 26, 42, 28, 29, 42, 31, 32, 33, 34, 35, 48, 244, 41, 87, 58, 59, 60, 61, 62,
    63, 64, 65, 117, 44, 50, 41, 41, 48, 42, 45, 56, 57, 129, 59, 48, 52, 42, 367, 29, 42, 56, 138,
    48, 59, 132, 48, 57, 42, 145, 145, 138, 49, 138, 48, 286, 49, 144, 145, 42, 145, 52, 52, 53,
    54, 48, 56, 57, 57, 165, 56, 303, 303, 59, 305, 307, 322, 303, 163, 164, 52, 42, 167, 168, 38,
    181, 182, 48, 184, 25, 37, 38, 39, 40, 181, 182, 181, 182, 185, 53, 54, 55, 49, 199, 42, 42,
    36, 339, 340, 42, 48, 48, 199, 345, 199, 48, 212, 212, 163, 164, 36, 217, 167, 168, 220, 212,
    46, 212, 48, 46, 217, 48, 217, 220, 207, 220, 209, 136, 53, 54, 29, 128, 49, 130, 143, 53, 54,
    134, 244, 148, 137, 29, 41, 140, 153, 142, 52, 53, 54, 29, 256, 36, 149, 52, 53, 54, 46, 56,
    48, 256, 41, 256, 42, 41, 52, 53, 54, 37, 56, 166, 276, 276, 52, 53, 54, 272, 56, 29, 41, 276,
    286, 276, 53, 54, 55, 18, 19, 20, 21, 36, 23, 24, 25, 26, 27, 28, 29, 303, 31, 305, 52, 53, 54,
    41, 56, 37, 38, 39, 40, 42, 37, 38, 39, 40, 46, 41, 322, 50, 41, 216, 326, 326, 49, 56, 36, 46,
    59, 48, 46, 326, 48, 326, 39, 40, 340, 232, 233, 234, 235, 36, 237, 238, 41, 37, 38, 39, 40,
    49, 18, 19, 20, 21, 46, 23, 24, 25, 26, 41, 28, 29, 257, 31, 37, 38, 39, 40, 57, 37, 38, 36,
    41, 41, 36, 43, 49, 45, 49, 36, 275, 45, 50, 51, 52, 53, 54, 55, 56, 57, 42, 59, 18, 19, 20,
    21, 36, 23, 24, 25, 26, 48, 28, 29, 49, 31, 45, 41, 36, 41, 43, 37, 38, 49, 49, 41, 43, 43, 52,
    53, 54, 55, 43, 57, 50, 51, 52, 53, 54, 55, 56, 57, 49, 59, 49, 49, 48, 18, 19, 20, 21, 336,
    23, 24, 25, 26, 49, 28, 29, 49, 31, 37, 38, 39, 40, 49, 37, 38, 41, 49, 41, 49, 43, 49, 41, 37,
    38, 39, 40, 50, 51, 52, 53, 54, 55, 56, 57, 49, 59, 18, 19, 20, 21, 57, 23, 24, 25, 26, 46, 28,
    29, 30, 31, 32, 33, 34, 49, 37, 38, 39, 40, 36, 57, 42, 44, 5, 6, 7, 8, 9, 34, 50, 37, 38, 39,
    40, 49, 56, 49, 34, 59, 18, 19, 20, 21, 14, 23, 24, 25, 26, 132, 28, 29, 30, 31, 32, 33, 34,
    18, 19, 20, 21, 256, 23, 24, 25, 26, 42, 28, 29, 257, 31, 261, 50, 46, 326, 212, 377, 46, 56,
    374, 41, 59, 58, 59, 60, 61, 62, 63, 64, 65, 272, 52, 53, 54, 274, 56, 303, 305, 59, 18, 19,
    20, 21, 68, 23, 24, 25, 26, 307, 28, 29, 303, 31, 52, 317, 26, -1, 265, -1, 18, 19, 20, 21, 42,
    23, 24, 25, 26, -1, 28, 29, -1, 31, 52, 53, 54, -1, 56, -1, -1, 59, -1, 41, 4, 5, 6, 7, 8, 9,
    10, 11, -1, -1, 52, 53, 54, -1, 56, -1, -1, 59, 18, 19, 20, 21, -1, 23, 24, 25, 26, -1, 28, 29,
    -1, 31, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26, 42, 28, 29, -1, 31, -1, -1, -1, -1, -1, 52, 53,
    54, -1, 56, -1, -1, 59, 45, -1, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, -1, -1, 59, 18, 19, 20,
    21, -1, 23, 24, 25, 26, -1, 28, 29, -1, 31, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26, 42, 28, 29,
    -1, 31, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, 42, -1, 59, -1, -1, -1, -1, -1, 50, -1, -1, -1,
    -1, -1, 56, -1, -1, 59, 18, 19, 20, 21, -1, 23, 24, 25, 26, -1, 28, 29, -1, 31, -1, 18, 19, 20,
    21, -1, 23, 24, 25, 26, 42, 28, 29, -1, 31, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, 42, -1, 59,
    -1, -1, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, -1, -1, 59, 18, 19, 20, 21, -1, 23, 24, 25, 26,
    -1, 28, 29, -1, 31, -1, -1, -1, -1, -1, -1, 18, 19, 20, 21, 42, 23, 24, 25, 26, 27, 28, 29, 50,
    31, -1, -1, -1, -1, 56, -1, -1, 59, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26, 50, 28, 29, -1, 31,
    -1, 56, -1, -1, 59, -1, -1, -1, -1, -1, 42, -1, -1, 45, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26,
    56, 28, 29, 59, 31, -1, -1, -1, -1, -1, -1, 18, 19, 20, 21, 42, 23, 24, 25, 26, -1, 28, 29, -1,
    31, -1, -1, -1, -1, 56, -1, -1, 59, -1, 41, 18, 19, 20, 21, -1, 23, 24, 25, 26, -1, 28, 29, -1,
    31, 56, -1, -1, 59, 58, 59, 60, 61, 62, 63, 64, 65, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 56, -1, -1, 59,
];

pub fn parse(ctx: *mut xkb_context, scanner: *mut scanner, map: *const i8) -> *mut XkbFile {
    unsafe {
        let mut ret: i32;
        let mut first: *mut XkbFile = std::ptr::null_mut();
        let mut param: parser_param = parser_param {
            ctx,
            scanner,
            rtrn: std::ptr::null_mut(),
            more_maps: false,
        };
        loop {
            ret = _xkbcommon_parse(&raw mut param);
            if !(ret == 0_i32 && param.more_maps as i32 != 0) {
                break;
            }
            if !map.is_null() {
                let map_str = std::ffi::CStr::from_ptr(map).to_str().unwrap_or("");
                let rtrn_ref = &*param.rtrn;
                if map_str == rtrn_ref.name {
                    return param.rtrn;
                } else {
                    FreeXkbFile(param.rtrn);
                }
            } else if (*param.rtrn).flags as u32 & MAP_IS_DEFAULT != 0 {
                FreeXkbFile(first);
                return param.rtrn;
            } else if first.is_null() {
                first = param.rtrn;
            } else {
                FreeXkbFile(param.rtrn);
            }
            param.rtrn = std::ptr::null_mut();
        }
        if ret != 0_i32 {
            FreeXkbFile(first);
            FreeXkbFile(param.rtrn);
            return std::ptr::null_mut();
        }
        if !first.is_null() {
            log::warn!("[XKB-{:03}] No map in include statement, but \"{}\" contains several; Using first defined map, \"{}\"\n",
                XKB_WARNING_MISSING_DEFAULT_SECTION as i32,
                &(*scanner).file_name,
                safe_map_name(&*first));
        }
        first
    }
}
static YYSTOS: [yytype_uint8; 384] = [
    0, 0, 58, 59, 60, 61, 62, 63, 64, 65, 67, 68, 71, 73, 74, 75, 0, 4, 5, 6, 7, 8, 9, 10, 11, 69,
    72, 75, 52, 147, 148, 147, 41, 41, 70, 76, 42, 71, 73, 12, 13, 14, 15, 16, 42, 77, 121, 122,
    49, 49, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 28, 29, 31, 32, 33, 34, 35, 50, 56, 57, 59, 78,
    79, 80, 81, 84, 87, 88, 94, 95, 98, 99, 100, 101, 102, 103, 117, 118, 119, 120, 131, 145, 52,
    56, 82, 83, 145, 52, 146, 29, 52, 53, 54, 56, 85, 138, 139, 57, 57, 53, 54, 143, 145, 143, 146,
    146, 146, 25, 145, 55, 57, 132, 133, 142, 143, 146, 36, 146, 45, 47, 36, 49, 48, 49, 36, 41,
    41, 37, 41, 36, 36, 41, 36, 41, 41, 41, 143, 49, 41, 36, 53, 54, 144, 41, 18, 19, 21, 23, 24,
    25, 26, 29, 31, 37, 38, 41, 43, 50, 51, 119, 124, 125, 128, 129, 131, 133, 145, 119, 124, 83,
    124, 86, 86, 124, 45, 50, 89, 90, 91, 92, 131, 145, 57, 124, 57, 96, 97, 138, 124, 86, 41, 45,
    113, 114, 115, 116, 145, 25, 28, 30, 78, 98, 104, 105, 110, 117, 36, 86, 124, 49, 86, 125, 131,
    125, 42, 119, 127, 130, 124, 125, 125, 43, 37, 38, 39, 40, 46, 36, 45, 49, 42, 78, 42, 29, 41,
    52, 56, 93, 126, 128, 130, 134, 136, 139, 145, 42, 48, 36, 49, 49, 42, 48, 49, 42, 115, 38, 53,
    54, 55, 140, 141, 42, 48, 42, 48, 36, 41, 146, 42, 105, 124, 42, 49, 42, 43, 42, 48, 44, 123,
    124, 124, 124, 124, 124, 124, 124, 49, 49, 42, 52, 135, 139, 46, 48, 46, 48, 46, 48, 49, 91,
    92, 124, 49, 97, 49, 42, 141, 48, 49, 114, 49, 116, 41, 124, 27, 78, 106, 107, 41, 49, 49, 49,
    49, 123, 130, 44, 48, 46, 42, 48, 41, 126, 134, 129, 130, 41, 136, 137, 139, 140, 115, 41, 42,
    107, 57, 111, 112, 44, 124, 52, 139, 42, 46, 46, 42, 46, 42, 41, 57, 108, 109, 49, 36, 42, 48,
    123, 42, 48, 57, 49, 112, 42, 49, 109,
];

pub fn parse_next(
    ctx: *mut xkb_context,
    scanner: *mut scanner,
    xkb_file: *mut *mut XkbFile,
) -> bool {
    unsafe {
        let mut param: parser_param = parser_param {
            ctx,
            scanner,
            rtrn: std::ptr::null_mut(),
            more_maps: false,
        };
        let ret: i32 = _xkbcommon_parse(&raw mut param);
        if ret == 0_i32 && param.more_maps as i32 != 0 {
            *xkb_file = param.rtrn;
            true
        } else {
            FreeXkbFile(param.rtrn);
            *xkb_file = std::ptr::null_mut();
            ret == 0_i32
        }
    }
}
static YYR1: [yytype_uint8; 220] = [
    0, 66, 67, 67, 67, 68, 69, 69, 69, 70, 70, 71, 72, 72, 72, 72, 72, 73, 73, 74, 74, 75, 75, 75,
    75, 75, 75, 75, 75, 76, 76, 76, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77,
    78, 78, 78, 79, 80, 81, 82, 82, 83, 83, 84, 85, 85, 86, 86, 87, 88, 89, 89, 90, 90, 91, 91, 91,
    91, 91, 92, 92, 92, 92, 92, 93, 93, 93, 94, 95, 96, 96, 97, 97, 98, 99, 99, 100, 101, 102, 102,
    103, 104, 104, 105, 105, 105, 105, 105, 106, 106, 107, 107, 108, 108, 109, 109, 110, 111, 111,
    112, 113, 113, 114, 114, 114, 115, 115, 116, 117, 118, 118, 118, 118, 119, 119, 120, 120, 120,
    120, 120, 120, 120, 120, 120, 120, 120, 121, 121, 122, 122, 122, 122, 122, 123, 123, 123, 124,
    124, 124, 124, 124, 124, 125, 125, 125, 125, 125, 125, 125, 125, 125, 126, 126, 126, 126, 127,
    127, 128, 129, 129, 130, 131, 131, 131, 131, 132, 132, 133, 133, 133, 133, 134, 134, 134, 134,
    135, 135, 135, 135, 136, 136, 137, 137, 138, 138, 139, 139, 139, 139, 140, 140, 141, 141, 141,
    142, 143, 143, 144, 144, 145, 145, 146, 147, 147, 148,
];
static YYR2: [yytype_int8; 220] = [
    0, 2, 1, 1, 1, 7, 1, 1, 1, 2, 0, 7, 1, 1, 1, 1, 1, 1, 0, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 0,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 2, 3, 4, 5, 3, 3, 1, 1, 3, 6, 3, 1, 2, 0, 6,
    6, 1, 0, 3, 1, 3, 3, 1, 2, 1, 3, 5, 3, 5, 3, 4, 2, 0, 5, 6, 3, 1, 1, 1, 6, 5, 6, 5, 6, 6, 6, 6,
    2, 1, 5, 1, 1, 1, 1, 2, 1, 5, 1, 3, 1, 1, 3, 6, 3, 1, 3, 3, 1, 3, 5, 3, 3, 1, 5, 6, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 3, 1, 0, 3, 3, 3, 3, 3, 1, 2, 2, 2,
    2, 1, 4, 1, 1, 3, 3, 3, 1, 1, 3, 1, 3, 1, 2, 4, 1, 3, 4, 6, 1, 0, 1, 1, 1, 1, 3, 3, 1, 1, 3, 3,
    1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
];
pub const YYINITDEPTH: i32 = 200_i32;
pub const YYMAXDEPTH: i32 = 10000_i32;
fn yypcontext_expected_tokens(
    yyctx: *const yypcontext_t,
    yyarg: *mut yysymbol_kind_t,
    yyargn: i32,
) -> i32 {
    unsafe {
        let mut yycount: i32 = 0_i32;
        let yyn: i32 = YYPACT[*(*yyctx).yyssp as usize] as i32;
        if yyn != YYPACT_NINF {
            let yyxbegin: i32 = if yyn < 0_i32 { -yyn } else { 0_i32 };
            let yychecklim: i32 = YYLAST - yyn + 1_i32;
            let yyxend: i32 = if yychecklim < YYNTOKENS {
                yychecklim
            } else {
                YYNTOKENS
            };
            let mut yyx: i32;
            yyx = yyxbegin;
            while yyx < yyxend {
                if YYCHECK[(yyx + yyn) as usize] as i32 == yyx && yyx != YYSYMBOL_YYerror && true {
                    if yyarg.is_null() {
                        yycount += 1;
                    } else if yycount == yyargn {
                        return 0_i32;
                    } else {
                        let c2rust_fresh4 = yycount;
                        yycount += 1;
                        *yyarg.offset(c2rust_fresh4 as isize) = yyx as yysymbol_kind_t;
                    }
                }
                yyx += 1;
            }
        }
        if !yyarg.is_null() && yycount == 0_i32 && 0_i32 < yyargn {
            *yyarg.offset(0_i32 as isize) = YYSYMBOL_YYEMPTY;
        }
        yycount
    }
}
fn yy_syntax_error_arguments(
    yyctx: *const yypcontext_t,
    yyarg: *mut yysymbol_kind_t,
    yyargn: i32,
) -> i32 {
    unsafe {
        let mut yycount: i32 = 0_i32;
        if (*yyctx).yytoken != YYSYMBOL_YYEMPTY {
            if !yyarg.is_null() {
                *yyarg.offset(yycount as isize) = (*yyctx).yytoken;
            }
            yycount += 1;
            let yyn: i32 = yypcontext_expected_tokens(
                yyctx,
                if !yyarg.is_null() {
                    yyarg.offset(1_i32 as isize)
                } else {
                    yyarg
                },
                yyargn - 1_i32,
            );
            if yyn == YYENOMEM {
                return YYENOMEM;
            } else {
                yycount += yyn;
            }
        }
        yycount
    }
}
fn yysyntax_error(yymsg_alloc: *mut i64, yymsg: *mut *mut i8, yyctx: *const yypcontext_t) -> i32 {
    unsafe {
        let mut yyformat: *const i8;
        let mut yyarg: [yysymbol_kind_t; 5] = [YYSYMBOL_YYEOF; 5];
        let mut yysize: i64;
        let yycount: i32 = yy_syntax_error_arguments(
            yyctx,
            &raw mut yyarg as *mut yysymbol_kind_t,
            YYARGS_MAX as i32,
        );
        if yycount == YYENOMEM {
            return YYENOMEM;
        }
        match yycount {
            1 => {
                yyformat = b"syntax error, unexpected %s\0".as_ptr() as *const i8;
            }
            2 => {
                yyformat = b"syntax error, unexpected %s, expecting %s\0".as_ptr() as *const i8;
            }
            3 => {
                yyformat =
                    b"syntax error, unexpected %s, expecting %s or %s\0".as_ptr() as *const i8;
            }
            4 => {
                yyformat = b"syntax error, unexpected %s, expecting %s or %s or %s\0".as_ptr()
                    as *const i8;
            }
            5 => {
                yyformat = b"syntax error, unexpected %s, expecting %s or %s or %s or %s\0".as_ptr()
                    as *const i8;
            }
            0 | _ => {
                yyformat = b"syntax error\0".as_ptr() as *const i8;
            }
        }
        yysize = cstr_len(yyformat) as i64 - (2_i32 * yycount) as i64 + 1_i64;
        let mut yyi: i32;
        yyi = 0_i32;
        while yyi < yycount {
            let yysize1: i64 = yysize + cstr_len(yysymbol_name(yyarg[yyi as usize])) as i64;
            if yysize <= yysize1
                && yysize1
                    <= (if (9223372036854775807_i64 as u64) < -1_i32 as u64 {
                        9223372036854775807_i64 as u64
                    } else {
                        -1_i32 as u64
                    }) as i64
            {
                yysize = yysize1;
            } else {
                return YYENOMEM;
            }
            yyi += 1;
        }
        if *yymsg_alloc < yysize {
            *yymsg_alloc = 2_i64 * yysize;
            if !(yysize <= *yymsg_alloc
                && *yymsg_alloc
                    <= (if (9223372036854775807_i64 as u64) < -1_i32 as u64 {
                        9223372036854775807_i64 as u64
                    } else {
                        -1_i32 as u64
                    }) as i64)
            {
                *yymsg_alloc = (if (9223372036854775807_i64 as u64) < -1_i32 as u64 {
                    9223372036854775807_i64 as u64
                } else {
                    -1_i32 as u64
                }) as i64;
            }
            return -1_i32;
        }
        let mut yyp: *mut i8 = *yymsg;
        let mut yyi_0: i32 = 0_i32;
        loop {
            *yyp = *yyformat;
            if *yyp as i32 == '\0' as i32 {
                break;
            }
            if *yyp as i32 == '%' as i32
                && *yyformat.offset(1_i32 as isize) as i32 == 's' as i32
                && yyi_0 < yycount
            {
                let c2rust_fresh3 = yyi_0;
                yyi_0 += 1;
                yyp =
                    crate::xkb::utils::cstr_pcpy(yyp, yysymbol_name(yyarg[c2rust_fresh3 as usize]));
                yyformat = yyformat.offset(2_isize);
            } else {
                yyp = yyp.offset(1);
                yyformat = yyformat.offset(1);
            }
        }
        0_i32
    }
}
fn yydestruct(
    yymsg: *const i8,
    yykind: yysymbol_kind_t,
    yyvaluep: *mut YYSTYPE,
    param: *mut parser_param,
) {
    unsafe {
        if yymsg.is_null() {
            // dead store removed: yymsg = b"Deleting\0".as_ptr() as *const i8;
        }
        match yykind {
            52 => {
                free((*yyvaluep).str as *mut ::core::ffi::c_void);
            }
            67 if (*param).rtrn.is_null() => {
                FreeXkbFile((*yyvaluep).file);
            }
            68 if (*param).rtrn.is_null() => {
                FreeXkbFile((*yyvaluep).file);
            }
            70 => {
                FreeXkbFile((*yyvaluep).fileList.head);
            }
            71 if (*param).rtrn.is_null() => {
                FreeXkbFile((*yyvaluep).file);
            }
            76 => {
                FreeStmt((*yyvaluep).anyList.head);
            }
            77 => {
                FreeStmt((*yyvaluep).any);
            }
            78 => {
                FreeStmt(to_common_or_null!((*yyvaluep).var));
            }
            79 => {
                FreeStmt(to_common_or_null!((*yyvaluep).keyCode));
            }
            80 => {
                FreeStmt(to_common_or_null!((*yyvaluep).keyAlias));
            }
            81 => {
                FreeStmt(to_common_or_null!((*yyvaluep).vmodList.head));
            }
            82 => {
                FreeStmt(to_common_or_null!((*yyvaluep).vmodList.head));
            }
            83 => {
                FreeStmt(to_common_or_null!((*yyvaluep).vmod));
            }
            84 => {
                FreeStmt(to_common_or_null!((*yyvaluep).interp));
            }
            85 => {
                FreeStmt(to_common_or_null!((*yyvaluep).interp));
            }
            86 => {
                FreeStmt(to_common_or_null!((*yyvaluep).varList.head));
            }
            87 => {
                FreeStmt(to_common_or_null!((*yyvaluep).keyType));
            }
            88 => {
                FreeStmt(to_common_or_null!((*yyvaluep).syms));
            }
            89 => {
                FreeStmt(to_common_or_null!((*yyvaluep).varList.head));
            }
            90 => {
                FreeStmt(to_common_or_null!((*yyvaluep).varList.head));
            }
            91 => {
                FreeStmt(to_common_or_null!((*yyvaluep).var));
            }
            92 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            94 => {
                FreeStmt(to_common_or_null!((*yyvaluep).groupCompat));
            }
            95 => {
                FreeStmt(to_common_or_null!((*yyvaluep).modMask));
            }
            96 => {
                FreeStmt(to_common_or_null!((*yyvaluep).exprList.head));
            }
            97 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            98 => {
                FreeStmt(to_common_or_null!((*yyvaluep).ledMap));
            }
            99 => {
                FreeStmt(to_common_or_null!((*yyvaluep).ledName));
            }
            115 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            116 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            123 => {
                FreeStmt(to_common_or_null!((*yyvaluep).exprList.head));
            }
            124 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            125 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            126 => {
                FreeStmt(to_common_or_null!((*yyvaluep).exprList.head));
            }
            127 => {
                FreeStmt(to_common_or_null!((*yyvaluep).exprList.head));
            }
            128 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            129 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            130 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            131 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            132 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            133 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            134 => {
                FreeStmt(to_common_or_null!((*yyvaluep).exprList.head));
            }
            135 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            136 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            137 => {
                FreeStmt(to_common_or_null!((*yyvaluep).expr));
            }
            147 => {
                free((*yyvaluep).str as *mut ::core::ffi::c_void);
            }
            148 => {
                free((*yyvaluep).str as *mut ::core::ffi::c_void);
            }
            _ => {}
        };
    }
}

pub fn _xkbcommon_parse(param: *mut parser_param) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut yychar: i32;
        static mut YYVAL_DEFAULT: YYSTYPE = YYSTYPE { num: 0 };
        let mut yylval: YYSTYPE = YYVAL_DEFAULT;
        let mut _xkbcommon_nerrs: i32 = 0_i32;
        let mut yystate: yy_state_fast_t = 0 as yy_state_fast_t;
        let mut yyerrstatus: i32 = 0_i32;
        let mut yystacksize: i64 = YYINITDEPTH as i64;
        let mut yyssa: [yy_state_t; 200] = [0; 200];
        let mut yyss: *mut yy_state_t = &raw mut yyssa as *mut yy_state_t;
        let mut yyssp: *mut yy_state_t = yyss;
        let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE { num: 0 }; 200];
        let mut yyvs: *mut YYSTYPE = &raw mut yyvsa as *mut YYSTYPE;
        let mut yyvsp: *mut YYSTYPE = yyvs;
        let mut yyn: i32;
        let yyresult: i32;
        let mut yytoken: yysymbol_kind_t;
        #[allow(unused_assignments)]
        let mut yyval: YYSTYPE = YYSTYPE { num: 0 };
        let mut yymsgbuf: [i8; 128] = [0; 128];
        let mut yymsg: *mut i8 = &raw mut yymsgbuf as *mut i8;
        let mut yymsg_alloc: i64 = std::mem::size_of::<[i8; 128]>() as i64;
        let mut yylen: i32 = 0_i32;
        yychar = YYEMPTY;
        's_60: loop {
            if false {
                (0_i32..384_i32).contains(&yystate);
            }
            *yyssp = yystate as yy_state_t;
            if yyss.offset(yystacksize as isize).offset(-(1_i32 as isize)) <= yyssp {
                let yysize: i64 = yyssp.offset_from(yyss) as i64 + 1_i64;
                if YYMAXDEPTH as i64 <= yystacksize {
                    c2rust_current_block = 9310790481625056212;
                    break;
                }
                yystacksize *= 2_i64;
                if (YYMAXDEPTH as i64) < yystacksize {
                    yystacksize = YYMAXDEPTH as i64;
                }
                let yyss1: *mut yy_state_t = yyss;
                let mut yyptr: *mut yyalloc = malloc(
                    (yystacksize
                        * (std::mem::size_of::<yy_state_t>() as i64
                            + std::mem::size_of::<YYSTYPE>() as i64)
                        + (std::mem::size_of::<yyalloc>() as i64 - 1_i64))
                        as usize,
                ) as *mut yyalloc;
                if yyptr.is_null() {
                    c2rust_current_block = 9310790481625056212;
                    break;
                }

                std::ptr::copy_nonoverlapping::<yy_state_t>(
                    yyss,
                    &raw mut (*yyptr).yyss_alloc,
                    yysize as usize,
                );
                yyss = &raw mut (*yyptr).yyss_alloc;
                let yynewbytes: i64 =
                    yystacksize * std::mem::size_of::<yy_state_t>() as i64 + YYSTACK_GAP_MAXIMUM;
                yyptr = yyptr.offset((yynewbytes / std::mem::size_of::<yyalloc>() as i64) as isize);

                std::ptr::copy_nonoverlapping::<YYSTYPE>(
                    yyvs,
                    &raw mut (*yyptr).yyvs_alloc,
                    yysize as usize,
                );
                yyvs = &raw mut (*yyptr).yyvs_alloc;
                if yyss1 != &raw mut yyssa as *mut yy_state_t {
                    free(yyss1 as *mut ::core::ffi::c_void);
                }
                yyssp = yyss.offset(yysize as isize).offset(-(1_i32 as isize));
                yyvsp = yyvs.offset(yysize as isize).offset(-(1_i32 as isize));
                if yyss.offset(yystacksize as isize).offset(-(1_i32 as isize)) <= yyssp {
                    c2rust_current_block = 7267896227379959561;
                    break;
                }
            }
            if yystate == YYFINAL {
                c2rust_current_block = 5508412643396263508;
                break;
            }
            yyn = YYPACT[yystate as usize] as i32;
            if yyn == YYPACT_NINF {
                c2rust_current_block = 16146138031620631371;
            } else {
                if yychar == YYEMPTY {
                    yychar = _xkbcommon_lex(&raw mut yylval, (*param).scanner);
                }
                if yychar <= END_OF_FILE {
                    yychar = END_OF_FILE;
                    yytoken = YYSYMBOL_YYEOF;
                    c2rust_current_block = 3689906465960840878;
                } else if yychar == YYerror {
                    yychar = YYUNDEF;
                    yytoken = YYSYMBOL_YYerror;
                    c2rust_current_block = 12965144090463719536;
                } else {
                    yytoken = (if (0_i32..=YYMAXUTOK).contains(&yychar) {
                        YYTRANSLATE[yychar as usize] as yysymbol_kind_t
                    } else {
                        YYSYMBOL_YYUNDEF
                    }) as yysymbol_kind_t;
                    c2rust_current_block = 3689906465960840878;
                }
                match c2rust_current_block {
                    12965144090463719536 => {}
                    _ => {
                        yyn += yytoken as i32;
                        if !(0_i32..=YYLAST).contains(&yyn)
                            || YYCHECK[yyn as usize] as i32 != yytoken as i32
                        {
                            c2rust_current_block = 16146138031620631371;
                        } else {
                            yyn = YYTABLE[yyn as usize] as i32;
                            if yyn <= 0_i32 {
                                yyn = -yyn;
                                c2rust_current_block = 18373490478049949584;
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                }
                                yystate = yyn as yy_state_fast_t;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = YYEMPTY;
                                c2rust_current_block = 10430565463943277256;
                            }
                        }
                    }
                }
            }
            if c2rust_current_block == 16146138031620631371 {
                yyn = YYDEFACT[yystate as usize] as i32;
                if yyn == 0_i32 {
                    yytoken = (if yychar == YYEMPTY {
                        YYSYMBOL_YYEMPTY
                    } else if (0_i32..=YYMAXUTOK).contains(&yychar) {
                        YYTRANSLATE[yychar as usize] as yysymbol_kind_t
                    } else {
                        YYSYMBOL_YYUNDEF
                    }) as yysymbol_kind_t;
                    if yyerrstatus == 0 {
                        _xkbcommon_nerrs += 1;
                        let mut yyctx: yypcontext_t = yypcontext_t { yyssp, yytoken };
                        let mut yymsgp: *const i8 = b"syntax error\0".as_ptr() as *const i8;
                        let mut yysyntax_error_status: i32;
                        yysyntax_error_status =
                            yysyntax_error(&raw mut yymsg_alloc, &raw mut yymsg, &raw mut yyctx);
                        if yysyntax_error_status == 0_i32 {
                            yymsgp = yymsg;
                        } else if yysyntax_error_status == -1_i32 {
                            if yymsg != &raw mut yymsgbuf as *mut i8 {
                                free(yymsg as *mut ::core::ffi::c_void);
                            }
                            yymsg = malloc(yymsg_alloc as usize) as *mut i8;
                            if !yymsg.is_null() {
                                yysyntax_error_status = yysyntax_error(
                                    &raw mut yymsg_alloc,
                                    &raw mut yymsg,
                                    &raw mut yyctx,
                                );
                                yymsgp = yymsg;
                            } else {
                                yymsg = &raw mut yymsgbuf as *mut i8;
                                yymsg_alloc = std::mem::size_of::<[i8; 128]>() as i64;
                                yysyntax_error_status = YYENOMEM;
                            }
                        }
                        _xkbcommon_error(param, yymsgp);
                        if yysyntax_error_status == YYENOMEM {
                            c2rust_current_block = 9310790481625056212;
                            break;
                        }
                    }
                    if yyerrstatus == 3_i32 {
                        if yychar <= END_OF_FILE {
                            if yychar == END_OF_FILE {
                                c2rust_current_block = 7267896227379959561;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0".as_ptr() as *const i8,
                                yytoken,
                                &raw mut yylval,
                                param,
                            );
                            yychar = YYEMPTY;
                        }
                    }
                    c2rust_current_block = 12965144090463719536;
                } else {
                    c2rust_current_block = 18373490478049949584;
                }
            }
            if c2rust_current_block == 18373490478049949584 {
                yylen = YYR2[yyn as usize] as i32;
                yyval = *yyvsp.offset((1_i32 - yylen) as isize);
                match yyn {
                    2 => {
                        (*param).rtrn = (*yyvsp.offset(0_i32 as isize)).file;
                        yyval.file = (*param).rtrn;
                        (*param).more_maps = !(*param).rtrn.is_null();
                        c2rust_current_block = 9699707990742192723;
                    }
                    3 => {
                        (*param).rtrn = (*yyvsp.offset(0_i32 as isize)).file;
                        // dead store removed: yyval.file = (*param).rtrn;
                        (*param).more_maps = !(*param).rtrn.is_null();
                        c2rust_current_block = 5508412643396263508;
                        break;
                    }
                    4 => {
                        (*param).rtrn = std::ptr::null_mut();
                        yyval.file = (*param).rtrn;
                        (*param).more_maps = false;
                        c2rust_current_block = 9699707990742192723;
                    }
                    5 => {
                        yyval.file = XkbFileCreate(
                            (*yyvsp.offset(-5_i32 as isize)).file_type,
                            (*yyvsp.offset(-4_i32 as isize)).str,
                            to_common_or_null!((*yyvsp.offset(-2_i32 as isize)).fileList.head),
                            (*yyvsp.offset(-6_i32 as isize)).mapFlags,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    6 => {
                        yyval.file_type = FILE_TYPE_KEYMAP;
                        c2rust_current_block = 9699707990742192723;
                    }
                    7 => {
                        yyval.file_type = FILE_TYPE_KEYMAP;
                        c2rust_current_block = 9699707990742192723;
                    }
                    8 => {
                        yyval.file_type = FILE_TYPE_KEYMAP;
                        c2rust_current_block = 9699707990742192723;
                    }
                    9 => {
                        if !(*yyvsp.offset(0_i32 as isize)).file.is_null() {
                            if !(*yyvsp.offset(-1_i32 as isize)).fileList.head.is_null() {
                                yyval.fileList.head =
                                    (*yyvsp.offset(-1_i32 as isize)).fileList.head;
                                (*yyval.fileList.last).common.next =
                                    to_common!((*yyvsp.offset(0_i32 as isize)).file);
                                yyval.fileList.last = (*yyvsp.offset(0_i32 as isize)).file;
                            } else {
                                yyval.fileList.last = (*yyvsp.offset(0_i32 as isize)).file;
                                yyval.fileList.head = yyval.fileList.last;
                            }
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    10 => {
                        yyval.fileList.last = std::ptr::null_mut();
                        yyval.fileList.head = yyval.fileList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    11 => {
                        yyval.file = XkbFileCreate(
                            (*yyvsp.offset(-5_i32 as isize)).file_type,
                            (*yyvsp.offset(-4_i32 as isize)).str,
                            (*yyvsp.offset(-2_i32 as isize)).anyList.head,
                            (*yyvsp.offset(-6_i32 as isize)).mapFlags,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    12 => {
                        yyval.file_type = FILE_TYPE_KEYCODES;
                        c2rust_current_block = 9699707990742192723;
                    }
                    13 => {
                        yyval.file_type = FILE_TYPE_TYPES;
                        c2rust_current_block = 9699707990742192723;
                    }
                    14 => {
                        yyval.file_type = FILE_TYPE_COMPAT;
                        c2rust_current_block = 9699707990742192723;
                    }
                    15 => {
                        yyval.file_type = FILE_TYPE_SYMBOLS;
                        c2rust_current_block = 9699707990742192723;
                    }
                    16 => {
                        yyval.file_type = FILE_TYPE_GEOMETRY;
                        c2rust_current_block = 9699707990742192723;
                    }
                    17 => {
                        yyval.mapFlags = (*yyvsp.offset(0_i32 as isize)).mapFlags;
                        c2rust_current_block = 9699707990742192723;
                    }
                    18 => {
                        yyval.mapFlags = 0 as xkb_map_flags;
                        c2rust_current_block = 9699707990742192723;
                    }
                    19 => {
                        yyval.mapFlags = ((*yyvsp.offset(-1_i32 as isize)).mapFlags
                            | (*yyvsp.offset(0_i32 as isize)).mapFlags)
                            as xkb_map_flags;
                        c2rust_current_block = 9699707990742192723;
                    }
                    20 => {
                        yyval.mapFlags = (*yyvsp.offset(0_i32 as isize)).mapFlags;
                        c2rust_current_block = 9699707990742192723;
                    }
                    21 => {
                        yyval.mapFlags = MAP_IS_PARTIAL;
                        c2rust_current_block = 9699707990742192723;
                    }
                    22 => {
                        yyval.mapFlags = MAP_IS_DEFAULT;
                        c2rust_current_block = 9699707990742192723;
                    }
                    23 => {
                        yyval.mapFlags = MAP_IS_HIDDEN;
                        c2rust_current_block = 9699707990742192723;
                    }
                    24 => {
                        yyval.mapFlags = MAP_HAS_ALPHANUMERIC;
                        c2rust_current_block = 9699707990742192723;
                    }
                    25 => {
                        yyval.mapFlags = MAP_HAS_MODIFIER;
                        c2rust_current_block = 9699707990742192723;
                    }
                    26 => {
                        yyval.mapFlags = MAP_HAS_KEYPAD;
                        c2rust_current_block = 9699707990742192723;
                    }
                    27 => {
                        yyval.mapFlags = MAP_HAS_FN;
                        c2rust_current_block = 9699707990742192723;
                    }
                    28 => {
                        yyval.mapFlags = MAP_IS_ALTGR;
                        c2rust_current_block = 9699707990742192723;
                    }
                    29 => {
                        if !(*yyvsp.offset(0_i32 as isize)).any.is_null() {
                            if !(*yyvsp.offset(-1_i32 as isize)).anyList.head.is_null() {
                                yyval.anyList.head = (*yyvsp.offset(-1_i32 as isize)).anyList.head;
                                let c2rust_fresh0 =
                                    &mut (*(*yyvsp.offset(-1_i32 as isize)).anyList.last).next;
                                *c2rust_fresh0 = (*yyvsp.offset(0_i32 as isize)).any;
                                yyval.anyList.last = (*yyvsp.offset(0_i32 as isize)).any;
                            } else {
                                yyval.anyList.last = (*yyvsp.offset(0_i32 as isize)).any;
                                yyval.anyList.head = yyval.anyList.last;
                            }
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    30 => {
                        let mut vmod: *mut VModDef = (*yyvsp.offset(0_i32 as isize)).vmodList.head;
                        while !vmod.is_null() {
                            (*vmod).merge = (*yyvsp.offset(-1_i32 as isize)).merge;
                            vmod = from_common!((*vmod).common.next, VModDef);
                        }
                        if !(*yyvsp.offset(-2_i32 as isize)).anyList.head.is_null() {
                            yyval.anyList.head = (*yyvsp.offset(-2_i32 as isize)).anyList.head;
                            let c2rust_fresh1 =
                                &mut (*(*yyvsp.offset(-2_i32 as isize)).anyList.last).next;
                            *c2rust_fresh1 =
                                to_common!((*yyvsp.offset(0_i32 as isize)).vmodList.head);
                            yyval.anyList.last =
                                &raw mut (*(*yyvsp.offset(0_i32 as isize)).vmodList.last).common;
                        } else {
                            yyval.anyList.head =
                                &raw mut (*(*yyvsp.offset(0_i32 as isize)).vmodList.head).common;
                            yyval.anyList.last =
                                &raw mut (*(*yyvsp.offset(0_i32 as isize)).vmodList.last).common;
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    31 => {
                        yyval.anyList.last = std::ptr::null_mut();
                        yyval.anyList.head = yyval.anyList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    32 => {
                        (*(*yyvsp.offset(0_i32 as isize)).var).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).var);
                        c2rust_current_block = 9699707990742192723;
                    }
                    33 => {
                        (*(*yyvsp.offset(0_i32 as isize)).interp).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).interp);
                        c2rust_current_block = 9699707990742192723;
                    }
                    34 => {
                        (*(*yyvsp.offset(0_i32 as isize)).keyCode).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).keyCode);
                        c2rust_current_block = 9699707990742192723;
                    }
                    35 => {
                        (*(*yyvsp.offset(0_i32 as isize)).keyAlias).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).keyAlias);
                        c2rust_current_block = 9699707990742192723;
                    }
                    36 => {
                        (*(*yyvsp.offset(0_i32 as isize)).keyType).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).keyType);
                        c2rust_current_block = 9699707990742192723;
                    }
                    37 => {
                        (*(*yyvsp.offset(0_i32 as isize)).syms).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).syms);
                        c2rust_current_block = 9699707990742192723;
                    }
                    38 => {
                        (*(*yyvsp.offset(0_i32 as isize)).modMask).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).modMask);
                        c2rust_current_block = 9699707990742192723;
                    }
                    39 => {
                        (*(*yyvsp.offset(0_i32 as isize)).groupCompat).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).groupCompat);
                        c2rust_current_block = 9699707990742192723;
                    }
                    40 => {
                        (*(*yyvsp.offset(0_i32 as isize)).ledMap).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).ledMap);
                        c2rust_current_block = 9699707990742192723;
                    }
                    41 => {
                        (*(*yyvsp.offset(0_i32 as isize)).ledName).merge =
                            (*yyvsp.offset(-1_i32 as isize)).merge;
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).ledName);
                        c2rust_current_block = 9699707990742192723;
                    }
                    42 => {
                        yyval.any = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    43 => {
                        yyval.any = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    44 => {
                        yyval.any = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    45 => {
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).unknown);
                        c2rust_current_block = 9699707990742192723;
                    }
                    46 => {
                        yyval.any = to_common_or_null!((*yyvsp.offset(0_i32 as isize)).unknown);
                        c2rust_current_block = 9699707990742192723;
                    }
                    47 => {
                        yyval.any = to_common_or_null!(IncludeCreate(
                            (*param).ctx,
                            (*yyvsp.offset(0_i32 as isize)).str,
                            (*yyvsp.offset(-1_i32 as isize)).merge,
                        ));
                        free((*yyvsp.offset(0_i32 as isize)).str as *mut ::core::ffi::c_void);
                        c2rust_current_block = 9699707990742192723;
                    }
                    48 => {
                        yyval.var = VarCreate(
                            box_from_raw((*yyvsp.offset(-3_i32 as isize)).expr),
                            box_from_raw((*yyvsp.offset(-1_i32 as isize)).expr),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    49 => {
                        yyval.var = BoolVarCreate((*yyvsp.offset(-1_i32 as isize)).atom, true);
                        c2rust_current_block = 9699707990742192723;
                    }
                    50 => {
                        yyval.var = BoolVarCreate((*yyvsp.offset(-1_i32 as isize)).atom, false);
                        c2rust_current_block = 9699707990742192723;
                    }
                    51 => {
                        yyval.keyCode = KeycodeCreate(
                            (*yyvsp.offset(-3_i32 as isize)).atom,
                            (*yyvsp.offset(-1_i32 as isize)).num,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    52 => {
                        yyval.keyAlias = KeyAliasCreate(
                            (*yyvsp.offset(-3_i32 as isize)).atom,
                            (*yyvsp.offset(-1_i32 as isize)).atom,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    53 => {
                        yyval.vmodList = (*yyvsp.offset(-1_i32 as isize)).vmodList;
                        c2rust_current_block = 9699707990742192723;
                    }
                    54 => {
                        yyval.vmodList.head = (*yyvsp.offset(-2_i32 as isize)).vmodList.head;
                        (*yyval.vmodList.last).common.next =
                            to_common!((*yyvsp.offset(0_i32 as isize)).vmod);
                        yyval.vmodList.last = (*yyvsp.offset(0_i32 as isize)).vmod;
                        c2rust_current_block = 9699707990742192723;
                    }
                    55 => {
                        yyval.vmodList.last = (*yyvsp.offset(0_i32 as isize)).vmod;
                        yyval.vmodList.head = yyval.vmodList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    56 => {
                        yyval.vmod = VModCreate((*yyvsp.offset(0_i32 as isize)).atom, None);
                        c2rust_current_block = 9699707990742192723;
                    }
                    57 => {
                        yyval.vmod = VModCreate(
                            (*yyvsp.offset(-2_i32 as isize)).atom,
                            box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    58 => {
                        let c2rust_fresh2 = &mut (*(*yyvsp.offset(-4_i32 as isize)).interp).def;
                        *c2rust_fresh2 =
                            collect_vardefs((*yyvsp.offset(-2_i32 as isize)).varList.head);
                        yyval.interp = (*yyvsp.offset(-4_i32 as isize)).interp;
                        c2rust_current_block = 9699707990742192723;
                    }
                    59 => {
                        yyval.interp = InterpCreate(
                            (*yyvsp.offset(-2_i32 as isize)).keysym,
                            box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    60 => {
                        yyval.interp = InterpCreate((*yyvsp.offset(0_i32 as isize)).keysym, None);
                        c2rust_current_block = 9699707990742192723;
                    }
                    61 => {
                        if !(*yyvsp.offset(0_i32 as isize)).var.is_null() {
                            if !(*yyvsp.offset(-1_i32 as isize)).varList.head.is_null() {
                                yyval.varList.head = (*yyvsp.offset(-1_i32 as isize)).varList.head;
                                (*yyval.varList.last).common.next =
                                    to_common!((*yyvsp.offset(0_i32 as isize)).var);
                                yyval.varList.last = (*yyvsp.offset(0_i32 as isize)).var;
                            } else {
                                yyval.varList.last = (*yyvsp.offset(0_i32 as isize)).var;
                                yyval.varList.head = yyval.varList.last;
                            }
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    62 => {
                        yyval.varList.last = std::ptr::null_mut();
                        yyval.varList.head = yyval.varList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    63 => {
                        yyval.keyType = KeyTypeCreate(
                            (*yyvsp.offset(-4_i32 as isize)).atom,
                            collect_vardefs((*yyvsp.offset(-2_i32 as isize)).varList.head),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    64 => {
                        yyval.syms = SymbolsCreate(
                            (*yyvsp.offset(-4_i32 as isize)).atom,
                            collect_vardefs((*yyvsp.offset(-2_i32 as isize)).varList.head),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    65 => {
                        yyval.varList = (*yyvsp.offset(0_i32 as isize)).varList;
                        c2rust_current_block = 9699707990742192723;
                    }
                    66 => {
                        yyval.varList.last = std::ptr::null_mut();
                        yyval.varList.head = yyval.varList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    67 => {
                        yyval.varList.head = (*yyvsp.offset(-2_i32 as isize)).varList.head;
                        (*yyval.varList.last).common.next =
                            to_common!((*yyvsp.offset(0_i32 as isize)).var);
                        yyval.varList.last = (*yyvsp.offset(0_i32 as isize)).var;
                        c2rust_current_block = 9699707990742192723;
                    }
                    68 => {
                        yyval.varList.last = (*yyvsp.offset(0_i32 as isize)).var;
                        yyval.varList.head = yyval.varList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    69 => {
                        yyval.var = VarCreate(
                            box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    70 => {
                        yyval.var = VarCreate(
                            box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    71 => {
                        yyval.var = BoolVarCreate((*yyvsp.offset(0_i32 as isize)).atom, true);
                        c2rust_current_block = 9699707990742192723;
                    }
                    72 => {
                        yyval.var = BoolVarCreate((*yyvsp.offset(0_i32 as isize)).atom, false);
                        c2rust_current_block = 9699707990742192723;
                    }
                    73 => {
                        yyval.var =
                            VarCreate(None, box_from_raw((*yyvsp.offset(0_i32 as isize)).expr));
                        c2rust_current_block = 9699707990742192723;
                    }
                    74 => {
                        yyval.expr = (*yyvsp.offset(-1_i32 as isize)).exprList.head;
                        c2rust_current_block = 9699707990742192723;
                    }
                    75 => {
                        let mut list: ExprList = ExprList {
                            head: (*yyvsp.offset(-1_i32 as isize)).exprList.head,
                            last: (*yyvsp.offset(-1_i32 as isize)).exprList.last,
                        };
                        let mut k: u32 = 0_u32;
                        while k < (*yyvsp.offset(-3_i32 as isize)).noSymbolOrActionList {
                            let syms: *mut ExprDef =
                                ExprCreateKeySymList(XKB_KEY_NoSymbol as u32) as *mut ExprDef;
                            if syms.is_null() {
                                c2rust_current_block = 7267896227379959561;
                                break 's_60;
                            }
                            (*syms).common.next = to_common!(list.head);
                            list.head = syms;
                            k = k.wrapping_add(1);
                        }
                        yyval.expr = list.head;
                        c2rust_current_block = 9699707990742192723;
                    }
                    76 => {
                        yyval.expr = (*yyvsp.offset(-1_i32 as isize)).exprList.head;
                        c2rust_current_block = 9699707990742192723;
                    }
                    77 => {
                        let mut list_0: VModList = VModList {
                            head: (*yyvsp.offset(-1_i32 as isize)).exprList.head,
                            last: (*yyvsp.offset(-1_i32 as isize)).exprList.last,
                        };
                        let mut k_0: u32 = 0_u32;
                        while k_0 < (*yyvsp.offset(-3_i32 as isize)).noSymbolOrActionList {
                            let acts: *mut ExprDef = expr_create(ExprKind::ActionList {
                                actions: Vec::new(),
                            }) as *mut ExprDef;
                            if acts.is_null() {
                                c2rust_current_block = 7267896227379959561;
                                break 's_60;
                            }
                            (*acts).common.next = to_common!(list_0.head);
                            list_0.head = acts;
                            k_0 = k_0.wrapping_add(1);
                        }
                        yyval.expr = list_0.head;
                        c2rust_current_block = 9699707990742192723;
                    }
                    78 => {
                        yyval.expr = expr_create(ExprKind::EmptyList);
                        c2rust_current_block = 9699707990742192723;
                    }
                    79 => {
                        yyval.noSymbolOrActionList = (*yyvsp.offset(-3_i32 as isize))
                            .noSymbolOrActionList
                            .wrapping_add(1_u32);
                        c2rust_current_block = 9699707990742192723;
                    }
                    80 => {
                        yyval.noSymbolOrActionList = 1_u32;
                        c2rust_current_block = 9699707990742192723;
                    }
                    81 => {
                        yyval.noSymbolOrActionList = 0_u32;
                        c2rust_current_block = 9699707990742192723;
                    }
                    82 => {
                        yyval.groupCompat = GroupCompatCreate(
                            (*yyvsp.offset(-3_i32 as isize)).num,
                            box_from_raw((*yyvsp.offset(-1_i32 as isize)).expr),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    83 => {
                        yyval.modMask = ModMapCreate(
                            (*yyvsp.offset(-4_i32 as isize)).atom,
                            collect_exprs((*yyvsp.offset(-2_i32 as isize)).exprList.head),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    84 => {
                        yyval.exprList.head = (*yyvsp.offset(-2_i32 as isize)).exprList.head;
                        (*yyval.exprList.last).common.next =
                            to_common!((*yyvsp.offset(0_i32 as isize)).expr);
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    85 => {
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    86 => {
                        yyval.expr =
                            expr_create(ExprKind::KeyName((*yyvsp.offset(0_i32 as isize)).atom));
                        c2rust_current_block = 9699707990742192723;
                    }
                    87 => {
                        yyval.expr =
                            expr_create(ExprKind::KeySym((*yyvsp.offset(0_i32 as isize)).keysym));
                        c2rust_current_block = 9699707990742192723;
                    }
                    88 => {
                        yyval.ledMap = LedMapCreate(
                            (*yyvsp.offset(-4_i32 as isize)).atom,
                            collect_vardefs((*yyvsp.offset(-2_i32 as isize)).varList.head),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    89 => {
                        yyval.ledName = LedNameCreate(
                            (*yyvsp.offset(-3_i32 as isize)).num,
                            box_from_raw((*yyvsp.offset(-1_i32 as isize)).expr),
                            false,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    90 => {
                        yyval.ledName = LedNameCreate(
                            (*yyvsp.offset(-3_i32 as isize)).num,
                            box_from_raw((*yyvsp.offset(-1_i32 as isize)).expr),
                            true,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    91 => {
                        FreeStmt(to_common_or_null!((*yyvsp.offset(-3_i32 as isize)).expr));
                        FreeStmt(to_common_or_null!((*yyvsp.offset(-1_i32 as isize)).expr));
                        yyval.unknown = UnknownStatementCreate(
                            STMT_UNKNOWN_DECLARATION,
                            (*yyvsp.offset(-4_i32 as isize)).sval,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    92 => {
                        FreeStmt(to_common_or_null!((*yyvsp.offset(-4_i32 as isize)).expr));
                        FreeStmt(to_common_or_null!(
                            (*yyvsp.offset(-2_i32 as isize)).varList.head
                        ));
                        yyval.unknown = UnknownStatementCreate(
                            STMT_UNKNOWN_COMPOUND,
                            (*yyvsp.offset(-5_i32 as isize)).sval,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    93 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    94 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    95 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    96 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    97 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    98 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    99 => {
                        FreeStmt(to_common_or_null!((*yyvsp.offset(0_i32 as isize)).var));
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    100 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    101 => {
                        FreeStmt(to_common_or_null!((*yyvsp.offset(0_i32 as isize)).ledMap));
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    102 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    103 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    104 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    105 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    106 => {
                        FreeStmt(to_common_or_null!((*yyvsp.offset(0_i32 as isize)).var));
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    107 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    108 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    109 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    110 => {
                        FreeStmt(to_common_or_null!(
                            (*yyvsp.offset(-1_i32 as isize)).exprList.head
                        ));
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    111 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    112 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    113 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    114 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    115 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    116 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    117 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    118 => {
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    119 => {
                        FreeStmt(to_common_or_null!((*yyvsp.offset(0_i32 as isize)).expr));
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    120 => {
                        yyval.expr = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    121 => {
                        yyval.expr = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    122 => {
                        yyval.expr = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    123 => {
                        FreeStmt(to_common_or_null!(
                            (*yyvsp.offset(-2_i32 as isize)).varList.head
                        ));
                        yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                        c2rust_current_block = 9699707990742192723;
                    }
                    124 => {
                        yyval.num = 0_i64;
                        c2rust_current_block = 9699707990742192723;
                    }
                    125 => {
                        yyval.num = 0_i64;
                        c2rust_current_block = 9699707990742192723;
                    }
                    126 => {
                        yyval.num = 0_i64;
                        c2rust_current_block = 9699707990742192723;
                    }
                    127 => {
                        yyval.num = 0_i64;
                        c2rust_current_block = 9699707990742192723;
                    }
                    128 => {
                        yyval.atom = (*yyvsp.offset(0_i32 as isize)).atom;
                        c2rust_current_block = 9699707990742192723;
                    }
                    129 => {
                        yyval.atom = (*yyvsp.offset(0_i32 as isize)).atom;
                        c2rust_current_block = 9699707990742192723;
                    }
                    130 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"action");
                        c2rust_current_block = 9699707990742192723;
                    }
                    131 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"interpret");
                        c2rust_current_block = 9699707990742192723;
                    }
                    132 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"type");
                        c2rust_current_block = 9699707990742192723;
                    }
                    133 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"key");
                        c2rust_current_block = 9699707990742192723;
                    }
                    134 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"group");
                        c2rust_current_block = 9699707990742192723;
                    }
                    135 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"modifier_map");
                        c2rust_current_block = 9699707990742192723;
                    }
                    136 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"indicator");
                        c2rust_current_block = 9699707990742192723;
                    }
                    137 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"shape");
                        c2rust_current_block = 9699707990742192723;
                    }
                    138 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"row");
                        c2rust_current_block = 9699707990742192723;
                    }
                    139 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"section");
                        c2rust_current_block = 9699707990742192723;
                    }
                    140 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"text");
                        c2rust_current_block = 9699707990742192723;
                    }
                    141 => {
                        yyval.merge = (*yyvsp.offset(0_i32 as isize)).merge;
                        c2rust_current_block = 9699707990742192723;
                    }
                    142 => {
                        yyval.merge = MERGE_DEFAULT;
                        c2rust_current_block = 9699707990742192723;
                    }
                    143 => {
                        yyval.merge = MERGE_DEFAULT;
                        c2rust_current_block = 9699707990742192723;
                    }
                    144 => {
                        yyval.merge = MERGE_AUGMENT;
                        c2rust_current_block = 9699707990742192723;
                    }
                    145 => {
                        yyval.merge = MERGE_OVERRIDE;
                        c2rust_current_block = 9699707990742192723;
                    }
                    146 => {
                        yyval.merge = MERGE_REPLACE;
                        c2rust_current_block = 9699707990742192723;
                    }
                    147 => {
                        let loc: scanner_loc = (*(*param).scanner).token_location();
                        log::warn!(
                            "{}:{}:{}: ignored unsupported legacy merge mode \"alternate\"\n",
                            &(*(*param).scanner).file_name,
                            loc.line,
                            loc.column
                        );
                        yyval.merge = MERGE_DEFAULT;
                        c2rust_current_block = 9699707990742192723;
                    }
                    148 => {
                        if !(*yyvsp.offset(0_i32 as isize)).expr.is_null() {
                            if !(*yyvsp.offset(-2_i32 as isize)).exprList.head.is_null() {
                                yyval.exprList.head =
                                    (*yyvsp.offset(-2_i32 as isize)).exprList.head;
                                (*yyval.exprList.last).common.next =
                                    to_common!((*yyvsp.offset(0_i32 as isize)).expr);
                                yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                            } else {
                                yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                                yyval.exprList.head = yyval.exprList.last;
                            }
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    149 => {
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    150 => {
                        yyval.exprList.last = std::ptr::null_mut();
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    151 => {
                        yyval.expr = expr_create(ExprKind::Binary {
                            op: STMT_EXPR_DIVIDE,
                            left: box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            right: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    152 => {
                        yyval.expr = expr_create(ExprKind::Binary {
                            op: STMT_EXPR_ADD,
                            left: box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            right: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    153 => {
                        yyval.expr = expr_create(ExprKind::Binary {
                            op: STMT_EXPR_SUBTRACT,
                            left: box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            right: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    154 => {
                        yyval.expr = expr_create(ExprKind::Binary {
                            op: STMT_EXPR_MULTIPLY,
                            left: box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            right: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    155 => {
                        yyval.expr = expr_create(ExprKind::Binary {
                            op: STMT_EXPR_ASSIGN,
                            left: box_from_raw((*yyvsp.offset(-2_i32 as isize)).expr),
                            right: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    156 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    157 => {
                        yyval.expr = expr_create(ExprKind::Unary {
                            op: STMT_EXPR_NEGATE,
                            child: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    158 => {
                        yyval.expr = expr_create(ExprKind::Unary {
                            op: STMT_EXPR_UNARY_PLUS,
                            child: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    159 => {
                        yyval.expr = expr_create(ExprKind::Unary {
                            op: STMT_EXPR_NOT,
                            child: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    160 => {
                        yyval.expr = expr_create(ExprKind::Unary {
                            op: STMT_EXPR_INVERT,
                            child: box_from_raw((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    161 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    162 => {
                        yyval.expr = expr_create(ExprKind::Action {
                            name: (*yyvsp.offset(-3_i32 as isize)).atom,
                            args: collect_exprs((*yyvsp.offset(-1_i32 as isize)).exprList.head),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    163 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    164 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    165 => {
                        yyval.expr = (*yyvsp.offset(-1_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    166 => {
                        let expr: *mut ExprDef = expr_create(ExprKind::ActionList {
                            actions: collect_exprs((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        yyval.exprList = (*yyvsp.offset(-2_i32 as isize)).exprList;
                        (*yyval.exprList.last).common.next = to_common!(expr);
                        yyval.exprList.last = expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    167 => {
                        yyval.exprList = (*yyvsp.offset(-2_i32 as isize)).exprList;
                        (*yyval.exprList.last).common.next =
                            to_common!((*yyvsp.offset(0_i32 as isize)).expr);
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    168 => {
                        yyval.exprList.last = expr_create(ExprKind::ActionList {
                            actions: collect_exprs((*yyvsp.offset(0_i32 as isize)).expr),
                        });
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    169 => {
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    170 => {
                        yyval.exprList = (*yyvsp.offset(-2_i32 as isize)).exprList;
                        (*yyval.exprList.last).common.next =
                            to_common!((*yyvsp.offset(0_i32 as isize)).expr);
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    171 => {
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    172 => {
                        yyval.expr = expr_create(ExprKind::ActionList {
                            actions: collect_exprs((*yyvsp.offset(-1_i32 as isize)).exprList.head),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    173 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    174 => {
                        yyval.expr = expr_create(ExprKind::ActionList {
                            actions: Vec::new(),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    175 => {
                        yyval.expr = expr_create(ExprKind::Action {
                            name: (*yyvsp.offset(-3_i32 as isize)).atom,
                            args: collect_exprs((*yyvsp.offset(-1_i32 as isize)).exprList.head),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    176 => {
                        yyval.expr =
                            expr_create(ExprKind::Ident((*yyvsp.offset(0_i32 as isize)).atom));
                        c2rust_current_block = 9699707990742192723;
                    }
                    177 => {
                        yyval.expr = expr_create(ExprKind::FieldRef {
                            element: (*yyvsp.offset(-2_i32 as isize)).atom,
                            field: (*yyvsp.offset(0_i32 as isize)).atom,
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    178 => {
                        yyval.expr = expr_create(ExprKind::ArrayRef {
                            element: XKB_ATOM_NONE,
                            field: (*yyvsp.offset(-3_i32 as isize)).atom,
                            entry: box_from_raw((*yyvsp.offset(-1_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    179 => {
                        yyval.expr = expr_create(ExprKind::ArrayRef {
                            element: (*yyvsp.offset(-5_i32 as isize)).atom,
                            field: (*yyvsp.offset(-3_i32 as isize)).atom,
                            entry: box_from_raw((*yyvsp.offset(-1_i32 as isize)).expr),
                        });
                        c2rust_current_block = 9699707990742192723;
                    }
                    180 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    181 => {
                        yyval.expr = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    182 => {
                        yyval.expr =
                            expr_create(ExprKind::String((*yyvsp.offset(0_i32 as isize)).atom));
                        c2rust_current_block = 9699707990742192723;
                    }
                    183 => {
                        yyval.expr =
                            expr_create(ExprKind::Integer((*yyvsp.offset(0_i32 as isize)).num));
                        c2rust_current_block = 9699707990742192723;
                    }
                    184 => {
                        yyval.expr = expr_create(ExprKind::Float);
                        c2rust_current_block = 9699707990742192723;
                    }
                    185 => {
                        yyval.expr =
                            expr_create(ExprKind::KeyName((*yyvsp.offset(0_i32 as isize)).atom));
                        c2rust_current_block = 9699707990742192723;
                    }
                    186 => {
                        let expr_0: *mut ExprDef =
                            ExprCreateKeySymList((*yyvsp.offset(0_i32 as isize)).keysym);
                        yyval.exprList = (*yyvsp.offset(-2_i32 as isize)).exprList;
                        (*yyval.exprList.last).common.next = to_common!(expr_0);
                        yyval.exprList.last = expr_0;
                        c2rust_current_block = 9699707990742192723;
                    }
                    187 => {
                        yyval.exprList = (*yyvsp.offset(-2_i32 as isize)).exprList;
                        (*yyval.exprList.last).common.next =
                            to_common!((*yyvsp.offset(0_i32 as isize)).expr);
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    188 => {
                        yyval.exprList.last =
                            ExprCreateKeySymList((*yyvsp.offset(0_i32 as isize)).keysym);
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    189 => {
                        yyval.exprList.last = (*yyvsp.offset(0_i32 as isize)).expr;
                        yyval.exprList.head = yyval.exprList.last;
                        c2rust_current_block = 9699707990742192723;
                    }
                    190 => {
                        yyval.expr = ExprAppendKeySymList(
                            (*yyvsp.offset(-2_i32 as isize)).expr,
                            (*yyvsp.offset(0_i32 as isize)).keysym,
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    191 => {
                        yyval.expr = ExprKeySymListAppendString(
                            (*param).scanner,
                            (*yyvsp.offset(-2_i32 as isize)).expr,
                            (*yyvsp.offset(0_i32 as isize)).str,
                        );
                        free((*yyvsp.offset(0_i32 as isize)).str as *mut ::core::ffi::c_void);
                        if yyval.expr.is_null() {
                            c2rust_current_block = 9017681648503218951;
                        } else {
                            c2rust_current_block = 9699707990742192723;
                        }
                    }
                    192 => {
                        yyval.expr = ExprCreateKeySymList((*yyvsp.offset(0_i32 as isize)).keysym);
                        if yyval.expr.is_null() {
                            c2rust_current_block = 9017681648503218951;
                        } else {
                            c2rust_current_block = 9699707990742192723;
                        }
                    }
                    193 => {
                        yyval.expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as u32);
                        if yyval.expr.is_null() {
                            c2rust_current_block = 9017681648503218951;
                        } else {
                            yyval.expr = ExprKeySymListAppendString(
                                (*param).scanner,
                                yyval.expr,
                                (*yyvsp.offset(0_i32 as isize)).str,
                            );
                            free((*yyvsp.offset(0_i32 as isize)).str as *mut ::core::ffi::c_void);
                            if yyval.expr.is_null() {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                c2rust_current_block = 9699707990742192723;
                            }
                        }
                    }
                    194 => {
                        yyval.expr = (*yyvsp.offset(-1_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    195 => {
                        yyval.expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as u32);
                        if yyval.expr.is_null() {
                            c2rust_current_block = 9017681648503218951;
                        } else {
                            yyval.expr = ExprKeySymListAppendString(
                                (*param).scanner,
                                yyval.expr,
                                (*yyvsp.offset(0_i32 as isize)).str,
                            );
                            free((*yyvsp.offset(0_i32 as isize)).str as *mut ::core::ffi::c_void);
                            if yyval.expr.is_null() {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                c2rust_current_block = 9699707990742192723;
                            }
                        }
                    }
                    196 => {
                        yyval.expr = (*yyvsp.offset(0_i32 as isize)).expr;
                        c2rust_current_block = 9699707990742192723;
                    }
                    197 => {
                        yyval.expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as u32);
                        c2rust_current_block = 9699707990742192723;
                    }
                    198 => {
                        yyval.keysym = (*yyvsp.offset(0_i32 as isize)).keysym;
                        c2rust_current_block = 9699707990742192723;
                    }
                    199 => {
                        yyval.keysym = KeysymParseString(
                            (*param).scanner,
                            (*yyvsp.offset(0_i32 as isize)).str,
                        );
                        free((*yyvsp.offset(0_i32 as isize)).str as *mut ::core::ffi::c_void);
                        if yyval.keysym == XKB_KEY_NoSymbol as u32 {
                            c2rust_current_block = 9017681648503218951;
                        } else {
                            c2rust_current_block = 9699707990742192723;
                        }
                    }
                    200 => {
                        if !resolve_keysym(
                            param,
                            (*yyvsp.offset(0_i32 as isize)).sval,
                            &raw mut yyval.keysym,
                        ) {
                            let loc_0: scanner_loc = (*(*param).scanner).token_location();
                            log::warn!(
                                "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"{}\"\n",
                                XKB_WARNING_UNRECOGNIZED_KEYSYM as i32,
                                &(*(*param).scanner).file_name,
                                loc_0.line,
                                loc_0.column,
                                (*yyvsp.offset(0_isize)).sval.as_str()
                            );
                            yyval.keysym = XKB_KEY_NoSymbol as u32;
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    201 => {
                        yyval.keysym = XKB_KEY_section as u32;
                        c2rust_current_block = 9699707990742192723;
                    }
                    202 => {
                        yyval.keysym = (XKB_KEY_0 as u32)
                            .wrapping_add((*yyvsp.offset(0_i32 as isize)).num as u32);
                        c2rust_current_block = 9699707990742192723;
                    }
                    203 => {
                        if (*yyvsp.offset(0_i32 as isize)).num < XKB_KEYSYM_MIN as i64 {
                            let loc_1: scanner_loc = (*(*param).scanner).token_location();
                            log::warn!(
                                "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"-{:#06x}\" ({})\n",
                                XKB_ERROR_INVALID_NUMERIC_KEYSYM as i32,
                                &(*(*param).scanner).file_name,
                                loc_1.line,
                                loc_1.column,
                                -(*yyvsp.offset(0_i32 as isize)).num,
                                (*yyvsp.offset(0_i32 as isize)).num
                            );
                            yyval.keysym = XKB_KEY_NoSymbol as u32;
                        } else {
                            if (*yyvsp.offset(0_i32 as isize)).num <= XKB_KEYSYM_MAX as i64 {
                                yyval.keysym = (*yyvsp.offset(0_i32 as isize)).num as u32;
                                if ((*(*param).ctx).log_verbosity >= 2_i32) as i32 as i64 != 0 {
                                    let mut ref_name: *const i8 = std::ptr::null();
                                    if xkb_keysym_is_deprecated(
                                        yyval.keysym,
                                        std::ptr::null(),
                                        &raw mut ref_name,
                                    ) {
                                        if ref_name.is_null() {
                                            let loc_2: scanner_loc =
                                                (*(*param).scanner).token_location();
                                            log::warn!("[XKB-{:03}] {}:{}:{}: deprecated keysym \"{:#06x}\".\n",
                                                XKB_WARNING_DEPRECATED_KEYSYM as i32,
                                                &(*(*param).scanner).file_name,
                                                loc_2.line,
                                                loc_2.column,
                                                yyval.keysym);
                                        } else {
                                            let loc_3: scanner_loc =
                                                (*(*param).scanner).token_location();
                                            log::warn!("[XKB-{:03}] {}:{}:{}: deprecated keysym name \"{:#06x}\"; please use \"{}\" instead.\n",
                                                XKB_WARNING_DEPRECATED_KEYSYM_NAME as i32,
                                                &(*(*param).scanner).file_name,
                                                loc_3.line,
                                                loc_3.column,
                                                yyval.keysym,
                                                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(ref_name)));
                                        }
                                    }
                                }
                            } else {
                                let loc_4: scanner_loc = (*(*param).scanner).token_location();
                                log::warn!(
                                    "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"{:#06x}\" ({})\n",
                                    XKB_ERROR_INVALID_NUMERIC_KEYSYM as i32,
                                    &(*(*param).scanner).file_name,
                                    loc_4.line,
                                    loc_4.column,
                                    (*yyvsp.offset(0_i32 as isize)).num,
                                    (*yyvsp.offset(0_i32 as isize)).num
                                );
                                yyval.keysym = XKB_KEY_NoSymbol as u32;
                            }
                            let loc_5: scanner_loc = (*(*param).scanner).token_location();
                            log::warn!(
                                "[XKB-{:03}] {}:{}:{}: numeric keysym \"{:#06x}\" ({})\n",
                                XKB_WARNING_NUMERIC_KEYSYM as i32,
                                &(*(*param).scanner).file_name,
                                loc_5.line,
                                loc_5.column,
                                (*yyvsp.offset(0_i32 as isize)).num,
                                (*yyvsp.offset(0_i32 as isize)).num
                            );
                        }
                        c2rust_current_block = 9699707990742192723;
                    }
                    204 => {
                        yyval.num = -(*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    205 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    206 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    207 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    208 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    209 => {
                        yyval.num = 0_i64;
                        c2rust_current_block = 9699707990742192723;
                    }
                    210 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    211 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    212 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    213 => {
                        yyval.num = (*yyvsp.offset(0_i32 as isize)).num;
                        c2rust_current_block = 9699707990742192723;
                    }
                    214 => {
                        yyval.atom = xkb_atom_intern(
                            (*param).ctx,
                            (*yyvsp.offset(0_i32 as isize)).sval.as_bytes(),
                        );
                        c2rust_current_block = 9699707990742192723;
                    }
                    215 => {
                        yyval.atom = xkb_atom_intern((*param).ctx, b"default");
                        c2rust_current_block = 9699707990742192723;
                    }
                    216 => {
                        yyval.atom = xkb_atom_intern(
                            (*param).ctx,
                            std::slice::from_raw_parts(
                                (*yyvsp.offset(0_i32 as isize)).str as *const u8,
                                cstr_len((*yyvsp.offset(0_i32 as isize)).str),
                            ),
                        );
                        free((*yyvsp.offset(0_i32 as isize)).str as *mut ::core::ffi::c_void);
                        c2rust_current_block = 9699707990742192723;
                    }
                    217 => {
                        yyval.str = (*yyvsp.offset(0_i32 as isize)).str;
                        c2rust_current_block = 9699707990742192723;
                    }
                    218 => {
                        yyval.str = std::ptr::null_mut();
                        c2rust_current_block = 9699707990742192723;
                    }
                    219 => {
                        yyval.str = (*yyvsp.offset(0_i32 as isize)).str;
                        c2rust_current_block = 9699707990742192723;
                    }
                    _ => {
                        c2rust_current_block = 9699707990742192723;
                    }
                }
                match c2rust_current_block {
                    9017681648503218951 => {
                        _xkbcommon_nerrs += 1;
                        yyvsp = yyvsp.offset(-(yylen as isize));
                        yyssp = yyssp.offset(-(yylen as isize));
                        yylen = 0_i32;
                        yystate = *yyssp as yy_state_fast_t;
                        c2rust_current_block = 12965144090463719536;
                    }
                    _ => {
                        yyvsp = yyvsp.offset(-(yylen as isize));
                        yyssp = yyssp.offset(-(yylen as isize));
                        yylen = 0_i32;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yyval;
                        let yylhs: i32 = YYR1[yyn as usize] as i32 - YYNTOKENS;
                        let yyi: i32 = YYPGOTO[yylhs as usize] as i32 + *yyssp as i32;
                        yystate = (if (0_i32..=YYLAST).contains(&yyi)
                            && YYCHECK[yyi as usize] as i32 == *yyssp as i32
                        {
                            YYTABLE[yyi as usize] as i32
                        } else {
                            YYDEFGOTO[yylhs as usize] as i32
                        }) as yy_state_fast_t;
                        c2rust_current_block = 10430565463943277256;
                    }
                }
            }
            if c2rust_current_block == 12965144090463719536 {
                yyerrstatus = 3_i32;
                loop {
                    yyn = YYPACT[yystate as usize] as i32;
                    if yyn != YYPACT_NINF {
                        yyn += YYSYMBOL_YYerror;
                        if (0_i32..=YYLAST).contains(&yyn)
                            && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYerror
                        {
                            yyn = YYTABLE[yyn as usize] as i32;
                            if 0_i32 < yyn {
                                break;
                            }
                        }
                    }
                    if yyssp == yyss {
                        c2rust_current_block = 7267896227379959561;
                        break 's_60;
                    }
                    yydestruct(
                        b"Error: popping\0".as_ptr() as *const i8,
                        YYSTOS[yystate as usize] as yysymbol_kind_t,
                        yyvsp,
                        param,
                    );
                    yyvsp = yyvsp.offset(-(1_i32 as isize));
                    yyssp = yyssp.offset(-(1_i32 as isize));
                    yystate = *yyssp as yy_state_fast_t;
                }
                yyvsp = yyvsp.offset(1);
                *yyvsp = yylval;
                yystate = yyn as yy_state_fast_t;
            }
            yyssp = yyssp.offset(1);
        }
        match c2rust_current_block {
            5508412643396263508 => {
                yyresult = 0_i32;
            }
            7267896227379959561 => {
                yyresult = 1_i32;
            }
            _ => {
                _xkbcommon_error(param, b"memory exhausted\0".as_ptr() as *const i8);
                yyresult = 2_i32;
            }
        }
        if yychar != YYEMPTY {
            yytoken = (if (0_i32..=YYMAXUTOK).contains(&yychar) {
                YYTRANSLATE[yychar as usize] as yysymbol_kind_t
            } else {
                YYSYMBOL_YYUNDEF
            }) as yysymbol_kind_t;
            yydestruct(
                b"Cleanup: discarding lookahead\0".as_ptr() as *const i8,
                yytoken,
                &raw mut yylval,
                param,
            );
        }
        yyvsp = yyvsp.offset(-(yylen as isize));
        yyssp = yyssp.offset(-(yylen as isize));
        while yyssp != yyss {
            yydestruct(
                b"Cleanup: popping\0".as_ptr() as *const i8,
                YYSTOS[*yyssp as usize] as yysymbol_kind_t,
                yyvsp,
                param,
            );
            yyvsp = yyvsp.offset(-(1_i32 as isize));
            yyssp = yyssp.offset(-(1_i32 as isize));
        }
        if yyss != &raw mut yyssa as *mut yy_state_t {
            free(yyss as *mut ::core::ffi::c_void);
        }
        if yymsg != &raw mut yymsgbuf as *mut i8 {
            free(yymsg as *mut ::core::ffi::c_void);
        }
        yyresult
    }
}
use crate::xkb::shared_types::*;
