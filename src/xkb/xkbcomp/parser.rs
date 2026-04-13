use crate::xkb::context_priv::xkb_atom_intern;
pub const XKB_KEY_VoidSymbol: i32 = 0xffffff as i32;
pub const XKB_KEY_0: i32 = 0x30 as i32;
pub const XKB_KEY_section: i32 = 0xa7 as i32;
use crate::xkb::keysym::{xkb_keysym_from_name, xkb_keysym_is_deprecated};
use crate::xkb_logf;

pub const XKB_KEYSYM_MIN: i32 = 0;


use crate::xkb::shared_types::xkb_keysym_t;
pub use crate::xkb::xkbcomp::ast_build::{
    BoolVarCreate, ExprAppendKeySymList, ExprCreateAction, ExprCreateActionList,
    ExprCreateArrayRef, ExprCreateBinary, ExprCreateFieldRef, ExprCreateFloat, ExprCreateIdent,
    ExprCreateInteger, ExprCreateKeyName, ExprCreateKeySym, ExprCreateKeySymList, ExprCreateString,
    ExprCreateUnary, ExprEmptyList, FreeStmt, GroupCompatCreate, IncludeCreate, InterpCreate,
    KeyAliasCreate, KeyTypeCreate, KeycodeCreate, LedMapCreate, LedNameCreate, ModMapCreate,
    SymbolsCreate, VModCreate, VarCreate, XkbFileCreate,
};

pub unsafe fn ExprKeySymListAppendString(
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

pub unsafe fn KeysymParseString(scanner: *mut scanner, string: *const i8) -> xkb_keysym_t {
    unsafe { crate::xkb::xkbcomp::ast_build::KeysymParseString(scanner as *mut _, string) }
}

pub unsafe fn UnknownStatementCreate(
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
pub unsafe fn _xkbcommon_lex(yylval: *mut YYSTYPE, scanner: *mut scanner) -> i32 {
    unsafe { crate::xkb::xkbcomp::scanner::_xkbcommon_lex(yylval as *mut _, scanner as *mut _) }
}

pub use super::scanner::parser_h::{
    yytokentype, C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5,
    C2Rust_Unnamed_6, YYerror, ACTION_TOK, ALIAS, ALPHANUMERIC_KEYS, ALTERNATE, ALTERNATE_GROUP,
    AUGMENT, CBRACE, CBRACKET, COMMA, CPAREN, DECIMAL_DIGIT, DEFAULT, DIVIDE, DOT, END_OF_FILE,
    EQUALS, ERROR_TOK, EXCLAM, FLOAT, FUNCTION_KEYS, GROUP, HIDDEN, IDENT, INCLUDE, INDICATOR,
    INTEGER, INTERPRET, INVERT, KEY, KEYNAME, KEYPAD_KEYS, KEYS, LOGO, MINUS, MODIFIER_KEYS,
    MODIFIER_MAP, OBRACE, OBRACKET, OPAREN, OUTLINE, OVERLAY, OVERRIDE, PARTIAL, PLUS, REPLACE,
    ROW, SECTION, SEMI, SHAPE, SOLID, STRING, TEXT, TIMES, TYPE, VIRTUAL, VIRTUAL_MODS,
    XKB_COMPATMAP, XKB_GEOMETRY, XKB_KEYCODES, XKB_KEYMAP, XKB_LAYOUT, XKB_SEMANTICS, XKB_SYMBOLS,
    XKB_TYPES, YYEMPTY, YYSTYPE, YYUNDEF,
};
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
pub use crate::xkb::scanner_utils::{isvaleq, scanner, scanner_loc, scanner_token_location, sval};
pub use crate::xkb::shared_ast_types::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, xkb_file_type, xkb_map_flags,
    C2Rust_Unnamed_1, ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef,
    ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString,
    ExprUnary, GroupCompatDef, IncludeStmt, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef,
    LedMapDef, LedNameDef, ModMapDef, ParseCommon, SymbolsDef, UnknownStatement, VModDef, VarDef,
    XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT,
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
    STMT_VAR, STMT_VMOD,
};
pub use crate::xkb::shared_ast_types::{safe_map_name, FreeXkbFile};
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::utils::cstr_len;
pub use crate::xkb::utils::{istrncmp, streq, streq_not_null};
use libc::{free, malloc};
#[derive(Copy, Clone)]
#[repr(C)]
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
pub const YYENOMEM: C2Rust_Unnamed_10 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yypcontext_t {
    pub yyssp: *mut yy_state_t,
    pub yytoken: yysymbol_kind_t,
}
pub const YYARGS_MAX: C2Rust_Unnamed_7 = 5;
pub type C2Rust_Unnamed_7 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub head: *mut ExprDef,
    pub last: *mut ExprDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub head: *mut ExprDef,
    pub last: *mut ExprDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
pub type C2Rust_Unnamed_10 = i32;
unsafe fn _xkbcommon_error(mut param: *mut parser_param, mut msg: *const i8) {
    unsafe {
        let mut loc: scanner_loc = scanner_token_location((*param).scanner);
        xkb_logf!(
            (*(*param).scanner).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] {}:{}:{}: {}\n",
            XKB_ERROR_INVALID_XKB_SYNTAX as i32,
            crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
            loc.line,
            loc.column,
            crate::xkb::utils::CStrDisplay(msg),
        );
    }
}
unsafe fn resolve_keysym(
    mut param: *mut parser_param,
    mut name: sval,
    mut sym_rtrn: *mut xkb_keysym_t,
) -> bool {
    unsafe {
        let mut sym: xkb_keysym_t = 0;
        if isvaleq(
            name,
            sval {
                len: (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
                start: b"any\0".as_ptr() as *const i8,
            },
        ) as i32
            != 0
            || isvaleq(
                name,
                sval {
                    len: (::core::mem::size_of::<[i8; 9]>() as usize).wrapping_sub(1 as usize),
                    start: b"nosymbol\0".as_ptr() as *const i8,
                },
            ) as i32
                != 0
        {
            *sym_rtrn = XKB_KEY_NoSymbol as xkb_keysym_t;
            return true;
        }
        if isvaleq(
            name,
            sval {
                len: (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
                start: b"none\0".as_ptr() as *const i8,
            },
        ) as i32
            != 0
            || isvaleq(
                name,
                sval {
                    len: (::core::mem::size_of::<[i8; 11]>() as usize).wrapping_sub(1 as usize),
                    start: b"voidsymbol\0".as_ptr() as *const i8,
                },
            ) as i32
                != 0
        {
            *sym_rtrn = XKB_KEY_VoidSymbol as xkb_keysym_t;
            return true;
        }
        let mut buf: [i8; 31] = [0; 31];
        if name.len >= ::core::mem::size_of::<[i8; 31]>() as usize {
            return false;
        }
        std::ptr::copy_nonoverlapping(name.start as *const u8, &raw mut buf as *mut u8, name.len);
        buf[name.len as usize] = '\0' as i32 as i8;
        sym = xkb_keysym_from_name(&raw mut buf as *mut i8, XKB_KEYSYM_NO_FLAGS);
        if sym != XKB_KEY_NoSymbol as xkb_keysym_t {
            *sym_rtrn = sym;
            if ((*(*param).ctx).log_verbosity >= 2 as i32) as i32
                as i64
                != 0
            {
                let mut ref_name: *const i8 = std::ptr::null();
                if xkb_keysym_is_deprecated(sym, &raw mut buf as *mut i8, &raw mut ref_name) {
                    if ref_name.is_null() {
                        let mut loc: scanner_loc = scanner_token_location((*param).scanner);
                        xkb_logf!(
                            (*(*param).scanner).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: deprecated keysym \"{}\".\n",
                            XKB_WARNING_DEPRECATED_KEYSYM as i32,
                            crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                            loc.line,
                            loc.column,
                            crate::xkb::utils::CStrDisplay(&raw mut buf as *mut i8),
                        );
                    } else {
                        let mut loc_0: scanner_loc = scanner_token_location((*param).scanner);
                        xkb_logf!(
                            (*(*param).scanner).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: deprecated keysym name \"{}\"; please use \"{}\" instead.\n",
                            XKB_WARNING_DEPRECATED_KEYSYM_NAME as i32,
                            crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                            loc_0.line,
                            loc_0.column,
                            crate::xkb::utils::CStrDisplay(&raw mut buf as *mut i8),
                            crate::xkb::utils::CStrDisplay(ref_name),
                        );
                    }
                }
            }
            return true;
        }
        return false;
    }
}
pub const YY_NULLPTR: *mut ::core::ffi::c_void =
    std::ptr::null_mut();
pub const YYSTACK_GAP_MAXIMUM: i64 = ::core::mem::size_of::<yyalloc>() as i64 - 1 as i64;
pub const YYFINAL: i32 = 16 as i32;
pub const YYLAST: i32 = 928 as i32;
pub const YYNTOKENS: i32 = 66 as i32;
pub const YYMAXUTOK: i32 = 257 as i32;
static mut yytranslate: [yytype_int8; 258] = [
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    7 as i32 as yytype_int8,
    8 as i32 as yytype_int8,
    9 as i32 as yytype_int8,
    10 as i32 as yytype_int8,
    11 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    12 as i32 as yytype_int8,
    13 as i32 as yytype_int8,
    14 as i32 as yytype_int8,
    15 as i32 as yytype_int8,
    16 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    17 as i32 as yytype_int8,
    18 as i32 as yytype_int8,
    19 as i32 as yytype_int8,
    20 as i32 as yytype_int8,
    21 as i32 as yytype_int8,
    22 as i32 as yytype_int8,
    23 as i32 as yytype_int8,
    24 as i32 as yytype_int8,
    25 as i32 as yytype_int8,
    26 as i32 as yytype_int8,
    27 as i32 as yytype_int8,
    28 as i32 as yytype_int8,
    29 as i32 as yytype_int8,
    30 as i32 as yytype_int8,
    31 as i32 as yytype_int8,
    32 as i32 as yytype_int8,
    33 as i32 as yytype_int8,
    34 as i32 as yytype_int8,
    35 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    40 as i32 as yytype_int8,
    41 as i32 as yytype_int8,
    42 as i32 as yytype_int8,
    43 as i32 as yytype_int8,
    44 as i32 as yytype_int8,
    45 as i32 as yytype_int8,
    46 as i32 as yytype_int8,
    47 as i32 as yytype_int8,
    48 as i32 as yytype_int8,
    49 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    51 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    52 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
    55 as i32 as yytype_int8,
    56 as i32 as yytype_int8,
    57 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    58 as i32 as yytype_int8,
    59 as i32 as yytype_int8,
    60 as i32 as yytype_int8,
    61 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    65 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
];
unsafe fn yysymbol_name(mut yysymbol: yysymbol_kind_t) -> *const i8 {
    unsafe {
        static mut yy_sname: [*const i8; 150] = [
            b"end of file\0".as_ptr() as *const i8,
            b"error\0".as_ptr() as *const i8,
            b"invalid token\0".as_ptr() as *const i8,
            b"invalid token\0".as_ptr() as *const i8,
            b"xkb_keymap\0".as_ptr() as *const i8,
            b"xkb_keycodes\0".as_ptr() as *const i8,
            b"xkb_types\0".as_ptr() as *const i8,
            b"xkb_symbols\0".as_ptr() as *const i8,
            b"xkb_compatibility\0".as_ptr() as *const i8,
            b"xkb_geometry\0".as_ptr() as *const i8,
            b"xkb_semantics\0".as_ptr() as *const i8,
            b"xkb_layout\0".as_ptr() as *const i8,
            b"include\0".as_ptr() as *const i8,
            b"override\0".as_ptr() as *const i8,
            b"augment\0".as_ptr() as *const i8,
            b"replace\0".as_ptr() as *const i8,
            b"alternate\0".as_ptr() as *const i8,
            b"virtual_modifiers\0".as_ptr() as *const i8,
            b"type\0".as_ptr() as *const i8,
            b"interpret\0".as_ptr() as *const i8,
            b"action\0".as_ptr() as *const i8,
            b"key\0".as_ptr() as *const i8,
            b"alias\0".as_ptr() as *const i8,
            b"group\0".as_ptr() as *const i8,
            b"modifier_map\0".as_ptr() as *const i8,
            b"indicator\0".as_ptr() as *const i8,
            b"shape\0".as_ptr() as *const i8,
            b"keys\0".as_ptr() as *const i8,
            b"row\0".as_ptr() as *const i8,
            b"section\0".as_ptr() as *const i8,
            b"overlay\0".as_ptr() as *const i8,
            b"text\0".as_ptr() as *const i8,
            b"outline\0".as_ptr() as *const i8,
            b"solid\0".as_ptr() as *const i8,
            b"logo\0".as_ptr() as *const i8,
            b"virtual\0".as_ptr() as *const i8,
            b"=\0".as_ptr() as *const i8,
            b"+\0".as_ptr() as *const i8,
            b"-\0".as_ptr() as *const i8,
            b"/\0".as_ptr() as *const i8,
            b"*\0".as_ptr() as *const i8,
            b"{\0".as_ptr() as *const i8,
            b"}\0".as_ptr() as *const i8,
            b"(\0".as_ptr() as *const i8,
            b")\0".as_ptr() as *const i8,
            b"[\0".as_ptr() as *const i8,
            b"]\0".as_ptr() as *const i8,
            b".\0".as_ptr() as *const i8,
            b",\0".as_ptr() as *const i8,
            b";\0".as_ptr() as *const i8,
            b"!\0".as_ptr() as *const i8,
            b"~\0".as_ptr() as *const i8,
            b"string literal\0".as_ptr() as *const i8,
            b"decimal digit\0".as_ptr() as *const i8,
            b"integer literal\0".as_ptr() as *const i8,
            b"float literal\0".as_ptr() as *const i8,
            b"identifier\0".as_ptr() as *const i8,
            b"key name\0".as_ptr() as *const i8,
            b"partial\0".as_ptr() as *const i8,
            b"default\0".as_ptr() as *const i8,
            b"hidden\0".as_ptr() as *const i8,
            b"alphanumeric_keys\0".as_ptr() as *const i8,
            b"modifier_keys\0".as_ptr() as *const i8,
            b"keypad_keys\0".as_ptr() as *const i8,
            b"function_keys\0".as_ptr() as *const i8,
            b"alternate_group\0".as_ptr() as *const i8,
            b"$accept\0".as_ptr() as *const i8,
            b"XkbFile\0".as_ptr() as *const i8,
            b"XkbCompositeMap\0".as_ptr() as *const i8,
            b"XkbCompositeType\0".as_ptr() as *const i8,
            b"XkbMapConfigList\0".as_ptr() as *const i8,
            b"XkbMapConfig\0".as_ptr() as *const i8,
            b"FileType\0".as_ptr() as *const i8,
            b"OptFlags\0".as_ptr() as *const i8,
            b"Flags\0".as_ptr() as *const i8,
            b"Flag\0".as_ptr() as *const i8,
            b"DeclList\0".as_ptr() as *const i8,
            b"Decl\0".as_ptr() as *const i8,
            b"VarDecl\0".as_ptr() as *const i8,
            b"KeyNameDecl\0".as_ptr() as *const i8,
            b"KeyAliasDecl\0".as_ptr() as *const i8,
            b"VModDecl\0".as_ptr() as *const i8,
            b"VModDefList\0".as_ptr() as *const i8,
            b"VModDef\0".as_ptr() as *const i8,
            b"InterpretDecl\0".as_ptr() as *const i8,
            b"InterpretMatch\0".as_ptr() as *const i8,
            b"VarDeclList\0".as_ptr() as *const i8,
            b"KeyTypeDecl\0".as_ptr() as *const i8,
            b"SymbolsDecl\0".as_ptr() as *const i8,
            b"OptSymbolsBody\0".as_ptr() as *const i8,
            b"SymbolsBody\0".as_ptr() as *const i8,
            b"SymbolsVarDecl\0".as_ptr() as *const i8,
            b"MultiKeySymOrActionList\0".as_ptr() as *const i8,
            b"NoSymbolOrActionList\0".as_ptr() as *const i8,
            b"GroupCompatDecl\0".as_ptr() as *const i8,
            b"ModMapDecl\0".as_ptr() as *const i8,
            b"KeyOrKeySymList\0".as_ptr() as *const i8,
            b"KeyOrKeySym\0".as_ptr() as *const i8,
            b"LedMapDecl\0".as_ptr() as *const i8,
            b"LedNameDecl\0".as_ptr() as *const i8,
            b"UnknownDecl\0".as_ptr() as *const i8,
            b"UnknownCompoundStatementDecl\0".as_ptr() as *const i8,
            b"ShapeDecl\0".as_ptr() as *const i8,
            b"SectionDecl\0".as_ptr() as *const i8,
            b"SectionBody\0".as_ptr() as *const i8,
            b"SectionBodyItem\0".as_ptr() as *const i8,
            b"RowBody\0".as_ptr() as *const i8,
            b"RowBodyItem\0".as_ptr() as *const i8,
            b"Keys\0".as_ptr() as *const i8,
            b"Key\0".as_ptr() as *const i8,
            b"OverlayDecl\0".as_ptr() as *const i8,
            b"OverlayKeyList\0".as_ptr() as *const i8,
            b"OverlayKey\0".as_ptr() as *const i8,
            b"OutlineList\0".as_ptr() as *const i8,
            b"OutlineInList\0".as_ptr() as *const i8,
            b"CoordList\0".as_ptr() as *const i8,
            b"Coord\0".as_ptr() as *const i8,
            b"DoodadDecl\0".as_ptr() as *const i8,
            b"DoodadType\0".as_ptr() as *const i8,
            b"FieldSpec\0".as_ptr() as *const i8,
            b"Element\0".as_ptr() as *const i8,
            b"OptMergeMode\0".as_ptr() as *const i8,
            b"MergeMode\0".as_ptr() as *const i8,
            b"ExprList\0".as_ptr() as *const i8,
            b"Expr\0".as_ptr() as *const i8,
            b"Term\0".as_ptr() as *const i8,
            b"MultiActionList\0".as_ptr() as *const i8,
            b"ActionList\0".as_ptr() as *const i8,
            b"NonEmptyActions\0".as_ptr() as *const i8,
            b"Actions\0".as_ptr() as *const i8,
            b"Action\0".as_ptr() as *const i8,
            b"Lhs\0".as_ptr() as *const i8,
            b"OptTerminal\0".as_ptr() as *const i8,
            b"Terminal\0".as_ptr() as *const i8,
            b"MultiKeySymList\0".as_ptr() as *const i8,
            b"KeySymList\0".as_ptr() as *const i8,
            b"NonEmptyKeySyms\0".as_ptr() as *const i8,
            b"KeySyms\0".as_ptr() as *const i8,
            b"KeySym\0".as_ptr() as *const i8,
            b"KeySymLit\0".as_ptr() as *const i8,
            b"SignedNumber\0".as_ptr() as *const i8,
            b"Number\0".as_ptr() as *const i8,
            b"Float\0".as_ptr() as *const i8,
            b"Integer\0".as_ptr() as *const i8,
            b"KeyCode\0".as_ptr() as *const i8,
            b"Ident\0".as_ptr() as *const i8,
            b"String\0".as_ptr() as *const i8,
            b"OptMapName\0".as_ptr() as *const i8,
            b"MapName\0".as_ptr() as *const i8,
            std::ptr::null(),
        ];
        return yy_sname[yysymbol as usize];
    }
}
pub const YYPACT_NINF: i32 = -280 as i32;
static mut yypact: [yytype_int16; 384] = [
    7 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    578 as i32 as yytype_int16,
    847 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -12 as i32 as yytype_int16,
    -12 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    463 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    458 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    172 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    135 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    78 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    114 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    324 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -18 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    143 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -30 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    175 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    179 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    182 as i32 as yytype_int16,
    190 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    197 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    248 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    263 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    237 as i32 as yytype_int16,
    261 as i32 as yytype_int16,
    135 as i32 as yytype_int16,
    258 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    293 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    289 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    869 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    597 as i32 as yytype_int16,
    269 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    461 as i32 as yytype_int16,
    296 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    287 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    825 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -6 as i32 as yytype_int16,
    228 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    294 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    639 as i32 as yytype_int16,
    654 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    478 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    306 as i32 as yytype_int16,
    311 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    318 as i32 as yytype_int16,
    -27 as i32 as yytype_int16,
    313 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    285 as i32 as yytype_int16,
    696 as i32 as yytype_int16,
    319 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    330 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    419 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    711 as i32 as yytype_int16,
    372 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    753 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    325 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    418 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    322 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    323 as i32 as yytype_int16,
    331 as i32 as yytype_int16,
    520 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    337 as i32 as yytype_int16,
    130 as i32 as yytype_int16,
    133 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    170 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    341 as i32 as yytype_int16,
    597 as i32 as yytype_int16,
    290 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    343 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    344 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    189 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    346 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    355 as i32 as yytype_int16,
    -25 as i32 as yytype_int16,
    358 as i32 as yytype_int16,
    319 as i32 as yytype_int16,
    377 as i32 as yytype_int16,
    773 as i32 as yytype_int16,
    375 as i32 as yytype_int16,
    364 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    386 as i32 as yytype_int16,
    368 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    370 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    869 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -38 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    266 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    540 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    845 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    796 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    381 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    227 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    384 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    582 as i32 as yytype_int16,
    239 as i32 as yytype_int16,
    242 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    180 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    400 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    -24 as i32 as yytype_int16,
    405 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    423 as i32 as yytype_int16,
    112 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    433 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    113 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    403 as i32 as yytype_int16,
    425 as i32 as yytype_int16,
    384 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    427 as i32 as yytype_int16,
    -24 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
];
static mut yydefact: [yytype_uint8; 384] = [
    18 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    21 as i32 as yytype_uint8,
    22 as i32 as yytype_uint8,
    23 as i32 as yytype_uint8,
    24 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    26 as i32 as yytype_uint8,
    27 as i32 as yytype_uint8,
    28 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    2 as i32 as yytype_uint8,
    3 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    17 as i32 as yytype_uint8,
    20 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    12 as i32 as yytype_uint8,
    13 as i32 as yytype_uint8,
    15 as i32 as yytype_uint8,
    14 as i32 as yytype_uint8,
    16 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    8 as i32 as yytype_uint8,
    218 as i32 as yytype_uint8,
    218 as i32 as yytype_uint8,
    19 as i32 as yytype_uint8,
    219 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    217 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    10 as i32 as yytype_uint8,
    31 as i32 as yytype_uint8,
    18 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    9 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    5 as i32 as yytype_uint8,
    11 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    214 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    215 as i32 as yytype_uint8,
    32 as i32 as yytype_uint8,
    34 as i32 as yytype_uint8,
    35 as i32 as yytype_uint8,
    30 as i32 as yytype_uint8,
    33 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    39 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    40 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    176 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    214 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    55 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    216 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    201 as i32 as yytype_uint8,
    199 as i32 as yytype_uint8,
    202 as i32 as yytype_uint8,
    203 as i32 as yytype_uint8,
    200 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    198 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    211 as i32 as yytype_uint8,
    210 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    209 as i32 as yytype_uint8,
    185 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    180 as i32 as yytype_uint8,
    184 as i32 as yytype_uint8,
    183 as i32 as yytype_uint8,
    182 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    66 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    213 as i32 as yytype_uint8,
    212 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    176 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    156 as i32 as yytype_uint8,
    173 as i32 as yytype_uint8,
    163 as i32 as yytype_uint8,
    161 as i32 as yytype_uint8,
    164 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    177 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    65 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    100 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    51 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    158 as i32 as yytype_uint8,
    161 as i32 as yytype_uint8,
    157 as i32 as yytype_uint8,
    174 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    171 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    159 as i32 as yytype_uint8,
    160 as i32 as yytype_uint8,
    150 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    178 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    201 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    195 as i32 as yytype_uint8,
    200 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    169 as i32 as yytype_uint8,
    168 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    189 as i32 as yytype_uint8,
    188 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    207 as i32 as yytype_uint8,
    208 as i32 as yytype_uint8,
    206 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    205 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    150 as i32 as yytype_uint8,
    172 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    165 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    149 as i32 as yytype_uint8,
    152 as i32 as yytype_uint8,
    153 as i32 as yytype_uint8,
    151 as i32 as yytype_uint8,
    154 as i32 as yytype_uint8,
    155 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    58 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    193 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    192 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    204 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    170 as i32 as yytype_uint8,
    162 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    179 as i32 as yytype_uint8,
    194 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    167 as i32 as yytype_uint8,
    166 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    196 as i32 as yytype_uint8,
    187 as i32 as yytype_uint8,
    186 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    175 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    191 as i32 as yytype_uint8,
    190 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    197 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    150 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 83] = [
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    434 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    443 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    469 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -45 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    356 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    244 as i32 as yytype_int16,
    251 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    249 as i32 as yytype_int16,
    466 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    302 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    187 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    144 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    257 as i32 as yytype_int16,
    -196 as i32 as yytype_int16,
    259 as i32 as yytype_int16,
    470 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -46 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -279 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -176 as i32 as yytype_int16,
    231 as i32 as yytype_int16,
    -181 as i32 as yytype_int16,
    -35 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    474 as i32 as yytype_int16,
    247 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    240 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    500 as i32 as yytype_int16,
    -182 as i32 as yytype_int16,
    236 as i32 as yytype_int16,
    291 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -44 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
    -37 as i32 as yytype_int16,
    -23 as i32 as yytype_int16,
    528 as i32 as yytype_int16,
    -280 as i32 as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 83] = [
    0 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    12 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    241 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    73 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    95 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    104 as i32 as yytype_int16,
    181 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    186 as i32 as yytype_int16,
    187 as i32 as yytype_int16,
    188 as i32 as yytype_int16,
    189 as i32 as yytype_int16,
    247 as i32 as yytype_int16,
    78 as i32 as yytype_int16,
    79 as i32 as yytype_int16,
    195 as i32 as yytype_int16,
    196 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    81 as i32 as yytype_int16,
    82 as i32 as yytype_int16,
    83 as i32 as yytype_int16,
    84 as i32 as yytype_int16,
    85 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    327 as i32 as yytype_int16,
    369 as i32 as yytype_int16,
    370 as i32 as yytype_int16,
    214 as i32 as yytype_int16,
    355 as i32 as yytype_int16,
    356 as i32 as yytype_int16,
    202 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    205 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    169 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    288 as i32 as yytype_int16,
    289 as i32 as yytype_int16,
    171 as i32 as yytype_int16,
    248 as i32 as yytype_int16,
    226 as i32 as yytype_int16,
    172 as i32 as yytype_int16,
    173 as i32 as yytype_int16,
    227 as i32 as yytype_int16,
    174 as i32 as yytype_int16,
    121 as i32 as yytype_int16,
    175 as i32 as yytype_int16,
    251 as i32 as yytype_int16,
    300 as i32 as yytype_int16,
    252 as i32 as yytype_int16,
    347 as i32 as yytype_int16,
    197 as i32 as yytype_int16,
    106 as i32 as yytype_int16,
    269 as i32 as yytype_int16,
    270 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    176 as i32 as yytype_int16,
    125 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
];
static mut yytable: [yytype_int16; 929] = [
    88 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    250 as i32 as yytype_int16,
    264 as i32 as yytype_int16,
    333 as i32 as yytype_int16,
    335 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    249 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    336 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    113 as i32 as yytype_int16,
    -71 as i32 as yytype_int16,
    200 as i32 as yytype_int16,
    367 as i32 as yytype_int16,
    132 as i32 as yytype_int16,
    133 as i32 as yytype_int16,
    112 as i32 as yytype_int16,
    -71 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    128 as i32 as yytype_int16,
    98 as i32 as yytype_int16,
    129 as i32 as yytype_int16,
    118 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    368 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    114 as i32 as yytype_int16,
    115 as i32 as yytype_int16,
    231 as i32 as yytype_int16,
    116 as i32 as yytype_int16,
    128 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    129 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    260 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    66 as i32 as yytype_int16,
    261 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    127 as i32 as yytype_int16,
    2 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    146 as i32 as yytype_int16,
    357 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    200 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    336 as i32 as yytype_int16,
    271 as i32 as yytype_int16,
    201 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    177 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    272 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    273 as i32 as yytype_int16,
    375 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    285 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    274 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    286 as i32 as yytype_int16,
    107 as i32 as yytype_int16,
    315 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    191 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    190 as i32 as yytype_int16,
    274 as i32 as yytype_int16,
    334 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    206 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    338 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    100 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    339 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    194 as i32 as yytype_int16,
    108 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    250 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    344 as i32 as yytype_int16,
    348 as i32 as yytype_int16,
    350 as i32 as yytype_int16,
    249 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    -124 as i32 as yytype_int16,
    366 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    265 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    274 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    254 as i32 as yytype_int16,
    266 as i32 as yytype_int16,
    267 as i32 as yytype_int16,
    268 as i32 as yytype_int16,
    239 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    373 as i32 as yytype_int16,
    376 as i32 as yytype_int16,
    126 as i32 as yytype_int16,
    360 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    381 as i32 as yytype_int16,
    374 as i32 as yytype_int16,
    377 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    336 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    221 as i32 as yytype_int16,
    223 as i32 as yytype_int16,
    130 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    229 as i32 as yytype_int16,
    230 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    302 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    304 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    305 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    114 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    277 as i32 as yytype_int16,
    182 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    170 as i32 as yytype_int16,
    131 as i32 as yytype_int16,
    178 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    150 as i32 as yytype_int16,
    151 as i32 as yytype_int16,
    180 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    217 as i32 as yytype_int16,
    183 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    345 as i32 as yytype_int16,
    193 as i32 as yytype_int16,
    220 as i32 as yytype_int16,
    198 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    218 as i32 as yytype_int16,
    245 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    306 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    307 as i32 as yytype_int16,
    191 as i32 as yytype_int16,
    135 as i32 as yytype_int16,
    190 as i32 as yytype_int16,
    364 as i32 as yytype_int16,
    136 as i32 as yytype_int16,
    100 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    137 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    228 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    325 as i32 as yytype_int16,
    299 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    206 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    266 as i32 as yytype_int16,
    267 as i32 as yytype_int16,
    268 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    139 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    324 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    359 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    352 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    236 as i32 as yytype_int16,
    143 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    144 as i32 as yytype_int16,
    280 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    325 as i32 as yytype_int16,
    259 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    140 as i32 as yytype_int16,
    362 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    305 as i32 as yytype_int16,
    363 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    307 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    290 as i32 as yytype_int16,
    291 as i32 as yytype_int16,
    292 as i32 as yytype_int16,
    293 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    294 as i32 as yytype_int16,
    295 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    147 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    337 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    148 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    311 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    192 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    164 as i32 as yytype_int16,
    149 as i32 as yytype_int16,
    153 as i32 as yytype_int16,
    165 as i32 as yytype_int16,
    216 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    262 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    219 as i32 as yytype_int16,
    237 as i32 as yytype_int16,
    323 as i32 as yytype_int16,
    238 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    255 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    257 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    258 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    201 as i32 as yytype_int16,
    -181 as i32 as yytype_int16,
    275 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    284 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    164 as i32 as yytype_int16,
    296 as i32 as yytype_int16,
    297 as i32 as yytype_int16,
    165 as i32 as yytype_int16,
    -139 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    -214 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    308 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    312 as i32 as yytype_int16,
    314 as i32 as yytype_int16,
    317 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    358 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    318 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    320 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    329 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    164 as i32 as yytype_int16,
    328 as i32 as yytype_int16,
    331 as i32 as yytype_int16,
    322 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    282 as i32 as yytype_int16,
    351 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    330 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    354 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    365 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    371 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    372 as i32 as yytype_int16,
    378 as i32 as yytype_int16,
    278 as i32 as yytype_int16,
    287 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    379 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    382 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    179 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    309 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    310 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    313 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    353 as i32 as yytype_int16,
    279 as i32 as yytype_int16,
    383 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    380 as i32 as yytype_int16,
    244 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    2 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    319 as i32 as yytype_int16,
    245 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    321 as i32 as yytype_int16,
    246 as i32 as yytype_int16,
    341 as i32 as yytype_int16,
    343 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    346 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    342 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    105 as i32 as yytype_int16,
    349 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    316 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    298 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    299 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    246 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    340 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    245 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    246 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    361 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    299 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    246 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    240 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    242 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    263 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    281 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    283 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    324 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    224 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    201 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    224 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    165 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    2 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
];
static mut yycheck: [yytype_int16; 929] = [
    46 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    200 as i32 as yytype_int16,
    284 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    12 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    244 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    129 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    367 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    132 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    286 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    144 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    165 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    305 as i32 as yytype_int16,
    307 as i32 as yytype_int16,
    322 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    164 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    181 as i32 as yytype_int16,
    182 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    181 as i32 as yytype_int16,
    182 as i32 as yytype_int16,
    181 as i32 as yytype_int16,
    182 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    339 as i32 as yytype_int16,
    340 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    345 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    164 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    217 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    220 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    217 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    217 as i32 as yytype_int16,
    220 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    220 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    136 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    128 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    130 as i32 as yytype_int16,
    143 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    244 as i32 as yytype_int16,
    148 as i32 as yytype_int16,
    137 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    140 as i32 as yytype_int16,
    153 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    149 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    272 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    286 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    305 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    322 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    216 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    340 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    235 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    237 as i32 as yytype_int16,
    238 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    257 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    275 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    336 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    132 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    257 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    261 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    377 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    374 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    272 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    274 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    305 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    307 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    317 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    265 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    -1 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
];

pub unsafe fn parse(
    mut ctx: *mut xkb_context,
    mut scanner: *mut scanner,
    mut map: *const i8,
) -> *mut XkbFile {
    unsafe {
        let mut ret: i32 = 0;
        let mut first: *mut XkbFile = std::ptr::null_mut();
        let mut param: parser_param = parser_param {
            ctx: ctx,
            scanner: scanner,
            rtrn: std::ptr::null_mut(),
            more_maps: false,
        };
        loop {
            ret = _xkbcommon_parse(&raw mut param);
            if !(ret == 0 as i32 && param.more_maps as i32 != 0) {
                break;
            }
            if !map.is_null() {
                if streq_not_null(map, (*param.rtrn).name) {
                    return param.rtrn;
                } else {
                    FreeXkbFile(param.rtrn);
                }
            } else if (*param.rtrn).flags as u32 & MAP_IS_DEFAULT as i32 as u32 != 0
            {
                FreeXkbFile(first);
                return param.rtrn;
            } else if first.is_null() {
                first = param.rtrn;
            } else {
                FreeXkbFile(param.rtrn);
            }
            param.rtrn = std::ptr::null_mut();
        }
        if ret != 0 as i32 {
            FreeXkbFile(first);
            FreeXkbFile(param.rtrn);
            return std::ptr::null_mut();
        }
        if !first.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_DETAILED as i32,
                "[XKB-{:03}] No map in include statement, but \"{}\" contains several; Using first defined map, \"{}\"\n",
                XKB_WARNING_MISSING_DEFAULT_SECTION as i32,
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                crate::xkb::utils::CStrDisplay(safe_map_name(first)),
            );
        }
        return first;
    }
}
static mut yystos: [yytype_uint8; 384] = [
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    58 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    65 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    5 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    8 as i32 as yytype_uint8,
    9 as i32 as yytype_uint8,
    10 as i32 as yytype_uint8,
    11 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    12 as i32 as yytype_uint8,
    13 as i32 as yytype_uint8,
    14 as i32 as yytype_uint8,
    15 as i32 as yytype_uint8,
    16 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    17 as i32 as yytype_uint8,
    18 as i32 as yytype_uint8,
    19 as i32 as yytype_uint8,
    20 as i32 as yytype_uint8,
    21 as i32 as yytype_uint8,
    22 as i32 as yytype_uint8,
    23 as i32 as yytype_uint8,
    24 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    26 as i32 as yytype_uint8,
    28 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    31 as i32 as yytype_uint8,
    32 as i32 as yytype_uint8,
    33 as i32 as yytype_uint8,
    34 as i32 as yytype_uint8,
    35 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    100 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    55 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    18 as i32 as yytype_uint8,
    19 as i32 as yytype_uint8,
    21 as i32 as yytype_uint8,
    23 as i32 as yytype_uint8,
    24 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    26 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    31 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    51 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    28 as i32 as yytype_uint8,
    30 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    39 as i32 as yytype_uint8,
    40 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    55 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    27 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
];

pub unsafe fn parse_next(
    mut ctx: *mut xkb_context,
    mut scanner: *mut scanner,
    mut xkb_file: *mut *mut XkbFile,
) -> bool {
    unsafe {
        let mut ret: i32 = 0;
        let mut param: parser_param = parser_param {
            ctx: ctx,
            scanner: scanner,
            rtrn: std::ptr::null_mut(),
            more_maps: false,
        };
        ret = _xkbcommon_parse(&raw mut param);
        if ret == 0 as i32 && param.more_maps as i32 != 0 {
            *xkb_file = param.rtrn;
            return true;
        } else {
            FreeXkbFile(param.rtrn);
            *xkb_file = std::ptr::null_mut();
            return ret == 0 as i32;
        };
    }
}
static mut yyr1: [yytype_uint8; 220] = [
    0 as i32 as yytype_uint8,
    66 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    100 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
];
static mut yyr2: [yytype_int8; 220] = [
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    7 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    7 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
];
pub const YYINITDEPTH: i32 = 200 as i32;
pub const YYMAXDEPTH: i32 = 10000 as i32;
unsafe fn yypcontext_expected_tokens(
    mut yyctx: *const yypcontext_t,
    mut yyarg: *mut yysymbol_kind_t,
    mut yyargn: i32,
) -> i32 {
    unsafe {
        let mut yycount: i32 = 0 as i32;
        let mut yyn: i32 =
            yypact[*(*yyctx).yyssp as i32 as usize] as i32;
        if !(yyn == YYPACT_NINF) {
            let mut yyxbegin: i32 = if yyn < 0 as i32 {
                -yyn
            } else {
                0 as i32
            };
            let mut yychecklim: i32 = YYLAST - yyn + 1 as i32;
            let mut yyxend: i32 = if yychecklim < YYNTOKENS {
                yychecklim
            } else {
                YYNTOKENS
            };
            let mut yyx: i32 = 0;
            yyx = yyxbegin;
            while yyx < yyxend {
                if yycheck[(yyx + yyn) as usize] as i32 == yyx
                    && yyx != YYSYMBOL_YYerror as i32
                    && true
                {
                    if yyarg.is_null() {
                        yycount += 1;
                    } else if yycount == yyargn {
                        return 0 as i32;
                    } else {
                        let c2rust_fresh4 = yycount;
                        yycount = yycount + 1;
                        *yyarg.offset(c2rust_fresh4 as isize) = yyx as yysymbol_kind_t;
                    }
                }
                yyx += 1;
            }
        }
        if !yyarg.is_null()
            && yycount == 0 as i32
            && (0 as i32) < yyargn
        {
            *yyarg.offset(0 as i32 as isize) = YYSYMBOL_YYEMPTY;
        }
        return yycount;
    }
}
unsafe fn yy_syntax_error_arguments(
    mut yyctx: *const yypcontext_t,
    mut yyarg: *mut yysymbol_kind_t,
    mut yyargn: i32,
) -> i32 {
    unsafe {
        let mut yycount: i32 = 0 as i32;
        if (*yyctx).yytoken as i32 != YYSYMBOL_YYEMPTY as i32 {
            let mut yyn: i32 = 0;
            if !yyarg.is_null() {
                *yyarg.offset(yycount as isize) = (*yyctx).yytoken;
            }
            yycount += 1;
            yyn = yypcontext_expected_tokens(
                yyctx,
                if !yyarg.is_null() {
                    yyarg.offset(1 as i32 as isize)
                } else {
                    yyarg
                },
                yyargn - 1 as i32,
            );
            if yyn == YYENOMEM as i32 {
                return YYENOMEM as i32;
            } else {
                yycount += yyn;
            }
        }
        return yycount;
    }
}
unsafe fn yysyntax_error(
    mut yymsg_alloc: *mut i64,
    mut yymsg: *mut *mut i8,
    mut yyctx: *const yypcontext_t,
) -> i32 {
    unsafe {
        let mut yyformat: *const i8 = std::ptr::null();
        let mut yyarg: [yysymbol_kind_t; 5] = [YYSYMBOL_YYEOF; 5];
        let mut yysize: i64 = 0 as i64;
        let mut yycount: i32 = yy_syntax_error_arguments(
            yyctx,
            &raw mut yyarg as *mut yysymbol_kind_t,
            YYARGS_MAX as i32,
        );
        if yycount == YYENOMEM as i32 {
            return YYENOMEM as i32;
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
        yysize = cstr_len(yyformat) as i64 - (2 as i32 * yycount) as i64 + 1 as i64;
        let mut yyi: i32 = 0;
        yyi = 0 as i32;
        while yyi < yycount {
            let mut yysize1: i64 = yysize + cstr_len(yysymbol_name(yyarg[yyi as usize])) as i64;
            if yysize <= yysize1
                && yysize1
                    <= (if (9223372036854775807 as i64 as u64) < -1 as i32 as u64 {
                        9223372036854775807 as i64 as u64
                    } else {
                        -1 as i32 as u64
                    }) as i64
            {
                yysize = yysize1;
            } else {
                return YYENOMEM as i32;
            }
            yyi += 1;
        }
        if *yymsg_alloc < yysize {
            *yymsg_alloc = 2 as i64 * yysize;
            if !(yysize <= *yymsg_alloc
                && *yymsg_alloc
                    <= (if (9223372036854775807 as i64 as u64) < -1 as i32 as u64 {
                        9223372036854775807 as i64 as u64
                    } else {
                        -1 as i32 as u64
                    }) as i64)
            {
                *yymsg_alloc =
                    (if (9223372036854775807 as i64 as u64) < -1 as i32 as u64 {
                        9223372036854775807 as i64 as u64
                    } else {
                        -1 as i32 as u64
                    }) as i64;
            }
            return -1 as i32;
        }
        let mut yyp: *mut i8 = *yymsg;
        let mut yyi_0: i32 = 0 as i32;
        loop {
            *yyp = *yyformat;
            if !(*yyp as i32 != '\0' as i32) {
                break;
            }
            if *yyp as i32 == '%' as i32
                && *yyformat.offset(1 as i32 as isize) as i32
                    == 's' as i32
                && yyi_0 < yycount
            {
                let c2rust_fresh3 = yyi_0;
                yyi_0 = yyi_0 + 1;
                yyp =
                    crate::xkb::utils::cstr_pcpy(yyp, yysymbol_name(yyarg[c2rust_fresh3 as usize]));
                yyformat = yyformat.offset(2 as isize);
            } else {
                yyp = yyp.offset(1);
                yyformat = yyformat.offset(1);
            }
        }
        return 0 as i32;
    }
}
unsafe fn yydestruct(
    mut yymsg: *const i8,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut YYSTYPE,
    mut param: *mut parser_param,
) {
    unsafe {
        if yymsg.is_null() {
            yymsg = b"Deleting\0".as_ptr() as *const i8;
        }
        match yykind as i32 {
            52 => {
                free((*yyvaluep).str as *mut ::core::ffi::c_void);
            }
            67 => {
                if (*param).rtrn.is_null() {
                    FreeXkbFile((*yyvaluep).file);
                }
            }
            68 => {
                if (*param).rtrn.is_null() {
                    FreeXkbFile((*yyvaluep).file);
                }
            }
            70 => {
                FreeXkbFile((*yyvaluep).fileList.head);
            }
            71 => {
                if (*param).rtrn.is_null() {
                    FreeXkbFile((*yyvaluep).file);
                }
            }
            76 => {
                FreeStmt((*yyvaluep).anyList.head);
            }
            77 => {
                FreeStmt((*yyvaluep).any);
            }
            78 => {
                FreeStmt((*yyvaluep).var as *mut ParseCommon);
            }
            79 => {
                FreeStmt((*yyvaluep).keyCode as *mut ParseCommon);
            }
            80 => {
                FreeStmt((*yyvaluep).keyAlias as *mut ParseCommon);
            }
            81 => {
                FreeStmt((*yyvaluep).vmodList.head as *mut ParseCommon);
            }
            82 => {
                FreeStmt((*yyvaluep).vmodList.head as *mut ParseCommon);
            }
            83 => {
                FreeStmt((*yyvaluep).vmod as *mut ParseCommon);
            }
            84 => {
                FreeStmt((*yyvaluep).interp as *mut ParseCommon);
            }
            85 => {
                FreeStmt((*yyvaluep).interp as *mut ParseCommon);
            }
            86 => {
                FreeStmt((*yyvaluep).varList.head as *mut ParseCommon);
            }
            87 => {
                FreeStmt((*yyvaluep).keyType as *mut ParseCommon);
            }
            88 => {
                FreeStmt((*yyvaluep).syms as *mut ParseCommon);
            }
            89 => {
                FreeStmt((*yyvaluep).varList.head as *mut ParseCommon);
            }
            90 => {
                FreeStmt((*yyvaluep).varList.head as *mut ParseCommon);
            }
            91 => {
                FreeStmt((*yyvaluep).var as *mut ParseCommon);
            }
            92 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            94 => {
                FreeStmt((*yyvaluep).groupCompat as *mut ParseCommon);
            }
            95 => {
                FreeStmt((*yyvaluep).modMask as *mut ParseCommon);
            }
            96 => {
                FreeStmt((*yyvaluep).exprList.head as *mut ParseCommon);
            }
            97 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            98 => {
                FreeStmt((*yyvaluep).ledMap as *mut ParseCommon);
            }
            99 => {
                FreeStmt((*yyvaluep).ledName as *mut ParseCommon);
            }
            115 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            116 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            123 => {
                FreeStmt((*yyvaluep).exprList.head as *mut ParseCommon);
            }
            124 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            125 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            126 => {
                FreeStmt((*yyvaluep).exprList.head as *mut ParseCommon);
            }
            127 => {
                FreeStmt((*yyvaluep).exprList.head as *mut ParseCommon);
            }
            128 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            129 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            130 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            131 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            132 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            133 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            134 => {
                FreeStmt((*yyvaluep).exprList.head as *mut ParseCommon);
            }
            135 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            136 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
            }
            137 => {
                FreeStmt((*yyvaluep).expr as *mut ParseCommon);
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

pub unsafe fn _xkbcommon_parse(mut param: *mut parser_param) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut yychar: i32 = 0;
        static mut yyval_default: YYSTYPE = YYSTYPE { num: 0 };
        let mut yylval: YYSTYPE = yyval_default;
        let mut _xkbcommon_nerrs: i32 = 0 as i32;
        let mut yystate: yy_state_fast_t = 0 as yy_state_fast_t;
        let mut yyerrstatus: i32 = 0 as i32;
        let mut yystacksize: i64 = YYINITDEPTH as i64;
        let mut yyssa: [yy_state_t; 200] = [0; 200];
        let mut yyss: *mut yy_state_t = &raw mut yyssa as *mut yy_state_t;
        let mut yyssp: *mut yy_state_t = yyss;
        let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE { num: 0 }; 200];
        let mut yyvs: *mut YYSTYPE = &raw mut yyvsa as *mut YYSTYPE;
        let mut yyvsp: *mut YYSTYPE = yyvs;
        let mut yyn: i32 = 0;
        let mut yyresult: i32 = 0;
        let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
        let mut yyval: YYSTYPE = YYSTYPE { num: 0 };
        let mut yymsgbuf: [i8; 128] = [0; 128];
        let mut yymsg: *mut i8 = &raw mut yymsgbuf as *mut i8;
        let mut yymsg_alloc: i64 = ::core::mem::size_of::<[i8; 128]>() as i64;
        let mut yylen: i32 = 0 as i32;
        yychar = YYEMPTY as i32;
        's_60: loop {
            (false && (0 as i32 <= yystate && yystate < 384 as i32))
                as i32;
            *yyssp = yystate as yy_state_t;
            if yyss
                .offset(yystacksize as isize)
                .offset(-(1 as i32 as isize))
                <= yyssp
            {
                let mut yysize: i64 = yyssp.offset_from(yyss) as i64 + 1 as i64;
                if YYMAXDEPTH as i64 <= yystacksize {
                    c2rust_current_block = 9310790481625056212;
                    break;
                }
                yystacksize *= 2 as i64;
                if (YYMAXDEPTH as i64) < yystacksize {
                    yystacksize = YYMAXDEPTH as i64;
                }
                let mut yyss1: *mut yy_state_t = yyss;
                let mut yyptr: *mut yyalloc = malloc(
                    (yystacksize
                        * (::core::mem::size_of::<yy_state_t>() as i64
                            + ::core::mem::size_of::<YYSTYPE>() as i64)
                        + (::core::mem::size_of::<yyalloc>() as i64 - 1 as i64))
                        as usize,
                ) as *mut yyalloc;
                if yyptr.is_null() {
                    c2rust_current_block = 9310790481625056212;
                    break;
                }
                let mut yynewbytes: i64 = 0;
                std::ptr::copy_nonoverlapping::<yy_state_t>(
                    yyss,
                    &raw mut (*yyptr).yyss_alloc,
                    yysize as usize,
                );
                yyss = &raw mut (*yyptr).yyss_alloc;
                yynewbytes =
                    yystacksize * ::core::mem::size_of::<yy_state_t>() as i64 + YYSTACK_GAP_MAXIMUM;
                yyptr =
                    yyptr.offset((yynewbytes / ::core::mem::size_of::<yyalloc>() as i64) as isize);
                let mut yynewbytes_0: i64 = 0;
                std::ptr::copy_nonoverlapping::<YYSTYPE>(
                    yyvs,
                    &raw mut (*yyptr).yyvs_alloc,
                    yysize as usize,
                );
                yyvs = &raw mut (*yyptr).yyvs_alloc;
                yynewbytes_0 =
                    yystacksize * ::core::mem::size_of::<YYSTYPE>() as i64 + YYSTACK_GAP_MAXIMUM;
                yyptr = yyptr
                    .offset((yynewbytes_0 / ::core::mem::size_of::<yyalloc>() as i64) as isize);
                if yyss1 != &raw mut yyssa as *mut yy_state_t {
                    free(yyss1 as *mut ::core::ffi::c_void);
                }
                yyssp = yyss
                    .offset(yysize as isize)
                    .offset(-(1 as i32 as isize));
                yyvsp = yyvs
                    .offset(yysize as isize)
                    .offset(-(1 as i32 as isize));
                if yyss
                    .offset(yystacksize as isize)
                    .offset(-(1 as i32 as isize))
                    <= yyssp
                {
                    c2rust_current_block = 7267896227379959561;
                    break;
                }
            }
            if yystate == YYFINAL {
                c2rust_current_block = 5508412643396263508;
                break;
            }
            yyn = yypact[yystate as usize] as i32;
            if yyn == YYPACT_NINF {
                c2rust_current_block = 16146138031620631371;
            } else {
                if yychar == YYEMPTY as i32 {
                    yychar = _xkbcommon_lex(&raw mut yylval, (*param).scanner);
                }
                if yychar <= END_OF_FILE as i32 {
                    yychar = END_OF_FILE as i32;
                    yytoken = YYSYMBOL_YYEOF;
                    c2rust_current_block = 3689906465960840878;
                } else if yychar == YYerror as i32 {
                    yychar = YYUNDEF as i32;
                    yytoken = YYSYMBOL_YYerror;
                    c2rust_current_block = 12965144090463719536;
                } else {
                    yytoken = (if 0 as i32 <= yychar && yychar <= YYMAXUTOK {
                        yytranslate[yychar as usize] as yysymbol_kind_t as i32
                    } else {
                        YYSYMBOL_YYUNDEF as i32
                    }) as yysymbol_kind_t;
                    c2rust_current_block = 3689906465960840878;
                }
                match c2rust_current_block {
                    12965144090463719536 => {}
                    _ => {
                        yyn += yytoken as i32;
                        if yyn < 0 as i32
                            || YYLAST < yyn
                            || yycheck[yyn as usize] as i32
                                != yytoken as i32
                        {
                            c2rust_current_block = 16146138031620631371;
                        } else {
                            yyn = yytable[yyn as usize] as i32;
                            if yyn <= 0 as i32 {
                                yyn = -yyn;
                                c2rust_current_block = 18373490478049949584;
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                }
                                yystate = yyn as yy_state_fast_t;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = YYEMPTY as i32;
                                c2rust_current_block = 10430565463943277256;
                            }
                        }
                    }
                }
            }
            match c2rust_current_block {
                16146138031620631371 => {
                    yyn = yydefact[yystate as usize] as i32;
                    if yyn == 0 as i32 {
                        yytoken = (if yychar == YYEMPTY as i32 {
                            YYSYMBOL_YYEMPTY as i32
                        } else if 0 as i32 <= yychar && yychar <= YYMAXUTOK {
                            yytranslate[yychar as usize] as yysymbol_kind_t as i32
                        } else {
                            YYSYMBOL_YYUNDEF as i32
                        }) as yysymbol_kind_t;
                        if yyerrstatus == 0 {
                            _xkbcommon_nerrs += 1;
                            let mut yyctx: yypcontext_t = yypcontext_t {
                                yyssp: yyssp,
                                yytoken: yytoken,
                            };
                            let mut yymsgp: *const i8 = b"syntax error\0".as_ptr() as *const i8;
                            let mut yysyntax_error_status: i32 = 0;
                            yysyntax_error_status = yysyntax_error(
                                &raw mut yymsg_alloc,
                                &raw mut yymsg,
                                &raw mut yyctx,
                            );
                            if yysyntax_error_status == 0 as i32 {
                                yymsgp = yymsg;
                            } else if yysyntax_error_status == -1 as i32 {
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
                                    yymsg_alloc = ::core::mem::size_of::<[i8; 128]>() as i64;
                                    yysyntax_error_status = YYENOMEM as i32;
                                }
                            }
                            _xkbcommon_error(param, yymsgp);
                            if yysyntax_error_status == YYENOMEM as i32 {
                                c2rust_current_block = 9310790481625056212;
                                break;
                            }
                        }
                        if yyerrstatus == 3 as i32 {
                            if yychar <= END_OF_FILE as i32 {
                                if yychar == END_OF_FILE as i32 {
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
                                yychar = YYEMPTY as i32;
                            }
                        }
                        c2rust_current_block = 12965144090463719536;
                    } else {
                        c2rust_current_block = 18373490478049949584;
                    }
                }
                _ => {}
            }
            match c2rust_current_block {
                18373490478049949584 => {
                    yylen = yyr2[yyn as usize] as i32;
                    yyval = *yyvsp.offset((1 as i32 - yylen) as isize);
                    match yyn {
                        2 => {
                            (*param).rtrn = (*yyvsp.offset(0 as i32 as isize)).file;
                            yyval.file = (*param).rtrn;
                            (*param).more_maps = !(*param).rtrn.is_null();
                            c2rust_current_block = 9699707990742192723;
                        }
                        3 => {
                            (*param).rtrn = (*yyvsp.offset(0 as i32 as isize)).file;
                            yyval.file = (*param).rtrn;
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
                                (*yyvsp.offset(-5 as i32 as isize)).file_type,
                                (*yyvsp.offset(-4 as i32 as isize)).str,
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .fileList
                                    .head as *mut ParseCommon,
                                (*yyvsp.offset(-6 as i32 as isize)).mapFlags,
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
                            if !(*yyvsp.offset(0 as i32 as isize))
                                .file
                                .is_null()
                            {
                                if !(*yyvsp.offset(-1 as i32 as isize))
                                    .fileList
                                    .head
                                    .is_null()
                                {
                                    yyval.fileList.head = (*yyvsp
                                        .offset(-1 as i32 as isize))
                                    .fileList
                                    .head;
                                    (*yyval.fileList.last).common.next = &raw mut (*(*yyvsp
                                        .offset(0 as i32 as isize))
                                    .file)
                                        .common
                                        as *mut _ParseCommon;
                                    yyval.fileList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).file;
                                } else {
                                    yyval.fileList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).file;
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
                                (*yyvsp.offset(-5 as i32 as isize)).file_type,
                                (*yyvsp.offset(-4 as i32 as isize)).str,
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .anyList
                                    .head,
                                (*yyvsp.offset(-6 as i32 as isize)).mapFlags,
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
                            yyval.mapFlags =
                                (*yyvsp.offset(0 as i32 as isize)).mapFlags;
                            c2rust_current_block = 9699707990742192723;
                        }
                        18 => {
                            yyval.mapFlags = 0 as xkb_map_flags;
                            c2rust_current_block = 9699707990742192723;
                        }
                        19 => {
                            yyval.mapFlags = ((*yyvsp.offset(-1 as i32 as isize))
                                .mapFlags as u32
                                | (*yyvsp.offset(0 as i32 as isize)).mapFlags as u32)
                                as xkb_map_flags;
                            c2rust_current_block = 9699707990742192723;
                        }
                        20 => {
                            yyval.mapFlags =
                                (*yyvsp.offset(0 as i32 as isize)).mapFlags;
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
                            if !(*yyvsp.offset(0 as i32 as isize))
                                .any
                                .is_null()
                            {
                                if !(*yyvsp.offset(-1 as i32 as isize))
                                    .anyList
                                    .head
                                    .is_null()
                                {
                                    yyval.anyList.head = (*yyvsp
                                        .offset(-1 as i32 as isize))
                                    .anyList
                                    .head;
                                    let ref mut c2rust_fresh0 = (*(*yyvsp
                                        .offset(-1 as i32 as isize))
                                    .anyList
                                    .last)
                                        .next;
                                    *c2rust_fresh0 =
                                        (*yyvsp.offset(0 as i32 as isize)).any
                                            as *mut _ParseCommon;
                                    yyval.anyList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).any;
                                } else {
                                    yyval.anyList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).any;
                                    yyval.anyList.head = yyval.anyList.last;
                                }
                            }
                            c2rust_current_block = 9699707990742192723;
                        }
                        30 => {
                            let mut vmod: *mut VModDef = (*yyvsp
                                .offset(0 as i32 as isize))
                            .vmodList
                            .head;
                            while !vmod.is_null() {
                                (*vmod).merge =
                                    (*yyvsp.offset(-1 as i32 as isize)).merge;
                                vmod = (*vmod).common.next as *mut VModDef;
                            }
                            if !(*yyvsp.offset(-2 as i32 as isize))
                                .anyList
                                .head
                                .is_null()
                            {
                                yyval.anyList.head = (*yyvsp
                                    .offset(-2 as i32 as isize))
                                .anyList
                                .head;
                                let ref mut c2rust_fresh1 = (*(*yyvsp
                                    .offset(-2 as i32 as isize))
                                .anyList
                                .last)
                                    .next;
                                *c2rust_fresh1 = &raw mut (*(*yyvsp
                                    .offset(0 as i32 as isize))
                                .vmodList
                                .head)
                                    .common
                                    as *mut _ParseCommon;
                                yyval.anyList.last = &raw mut (*(*yyvsp
                                    .offset(0 as i32 as isize))
                                .vmodList
                                .last)
                                    .common;
                            } else {
                                yyval.anyList.head = &raw mut (*(*yyvsp
                                    .offset(0 as i32 as isize))
                                .vmodList
                                .head)
                                    .common;
                                yyval.anyList.last = &raw mut (*(*yyvsp
                                    .offset(0 as i32 as isize))
                                .vmodList
                                .last)
                                    .common;
                            }
                            c2rust_current_block = 9699707990742192723;
                        }
                        31 => {
                            yyval.anyList.last = std::ptr::null_mut();
                            yyval.anyList.head = yyval.anyList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        32 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).var).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).var
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        33 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).interp).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).interp
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        34 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).keyCode).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).keyCode
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        35 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).keyAlias).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).keyAlias
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        36 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).keyType).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).keyType
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        37 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).syms).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).syms
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        38 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).modMask).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).modMask
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        39 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).groupCompat)
                                .merge = (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize))
                                .groupCompat
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        40 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).ledMap).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).ledMap
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        41 => {
                            (*(*yyvsp.offset(0 as i32 as isize)).ledName).merge =
                                (*yyvsp.offset(-1 as i32 as isize)).merge;
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).ledName
                                as *mut ParseCommon;
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
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).unknown
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        46 => {
                            yyval.any = (*yyvsp.offset(0 as i32 as isize)).unknown
                                as *mut ParseCommon;
                            c2rust_current_block = 9699707990742192723;
                        }
                        47 => {
                            yyval.any = IncludeCreate(
                                (*param).ctx,
                                (*yyvsp.offset(0 as i32 as isize)).str,
                                (*yyvsp.offset(-1 as i32 as isize)).merge,
                            ) as *mut ParseCommon;
                            free(
                                (*yyvsp.offset(0 as i32 as isize)).str
                                    as *mut ::core::ffi::c_void,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        48 => {
                            yyval.var = VarCreate(
                                (*yyvsp.offset(-3 as i32 as isize)).expr,
                                (*yyvsp.offset(-1 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        49 => {
                            yyval.var = BoolVarCreate(
                                (*yyvsp.offset(-1 as i32 as isize)).atom,
                                true,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        50 => {
                            yyval.var = BoolVarCreate(
                                (*yyvsp.offset(-1 as i32 as isize)).atom,
                                false,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        51 => {
                            yyval.keyCode = KeycodeCreate(
                                (*yyvsp.offset(-3 as i32 as isize)).atom,
                                (*yyvsp.offset(-1 as i32 as isize)).num,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        52 => {
                            yyval.keyAlias = KeyAliasCreate(
                                (*yyvsp.offset(-3 as i32 as isize)).atom,
                                (*yyvsp.offset(-1 as i32 as isize)).atom,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        53 => {
                            yyval.vmodList =
                                (*yyvsp.offset(-1 as i32 as isize)).vmodList;
                            c2rust_current_block = 9699707990742192723;
                        }
                        54 => {
                            yyval.vmodList.head = (*yyvsp
                                .offset(-2 as i32 as isize))
                            .vmodList
                            .head;
                            (*yyval.vmodList.last).common.next =
                                &raw mut (*(*yyvsp.offset(0 as i32 as isize)).vmod)
                                    .common as *mut _ParseCommon;
                            yyval.vmodList.last =
                                (*yyvsp.offset(0 as i32 as isize)).vmod;
                            c2rust_current_block = 9699707990742192723;
                        }
                        55 => {
                            yyval.vmodList.last =
                                (*yyvsp.offset(0 as i32 as isize)).vmod;
                            yyval.vmodList.head = yyval.vmodList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        56 => {
                            yyval.vmod = VModCreate(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                                std::ptr::null_mut(),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        57 => {
                            yyval.vmod = VModCreate(
                                (*yyvsp.offset(-2 as i32 as isize)).atom,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        58 => {
                            let ref mut c2rust_fresh2 =
                                (*(*yyvsp.offset(-4 as i32 as isize)).interp).def;
                            *c2rust_fresh2 = (*yyvsp.offset(-2 as i32 as isize))
                                .varList
                                .head;
                            yyval.interp =
                                (*yyvsp.offset(-4 as i32 as isize)).interp;
                            c2rust_current_block = 9699707990742192723;
                        }
                        59 => {
                            yyval.interp = InterpCreate(
                                (*yyvsp.offset(-2 as i32 as isize)).keysym,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        60 => {
                            yyval.interp = InterpCreate(
                                (*yyvsp.offset(0 as i32 as isize)).keysym,
                                std::ptr::null_mut(),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        61 => {
                            if !(*yyvsp.offset(0 as i32 as isize))
                                .var
                                .is_null()
                            {
                                if !(*yyvsp.offset(-1 as i32 as isize))
                                    .varList
                                    .head
                                    .is_null()
                                {
                                    yyval.varList.head = (*yyvsp
                                        .offset(-1 as i32 as isize))
                                    .varList
                                    .head;
                                    (*yyval.varList.last).common.next = &raw mut (*(*yyvsp
                                        .offset(0 as i32 as isize))
                                    .var)
                                        .common
                                        as *mut _ParseCommon;
                                    yyval.varList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).var;
                                } else {
                                    yyval.varList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).var;
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
                                (*yyvsp.offset(-4 as i32 as isize)).atom,
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .varList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        64 => {
                            yyval.syms = SymbolsCreate(
                                (*yyvsp.offset(-4 as i32 as isize)).atom,
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .varList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        65 => {
                            yyval.varList =
                                (*yyvsp.offset(0 as i32 as isize)).varList;
                            c2rust_current_block = 9699707990742192723;
                        }
                        66 => {
                            yyval.varList.last = std::ptr::null_mut();
                            yyval.varList.head = yyval.varList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        67 => {
                            yyval.varList.head = (*yyvsp.offset(-2 as i32 as isize))
                                .varList
                                .head;
                            (*yyval.varList.last).common.next =
                                &raw mut (*(*yyvsp.offset(0 as i32 as isize)).var)
                                    .common as *mut _ParseCommon;
                            yyval.varList.last =
                                (*yyvsp.offset(0 as i32 as isize)).var;
                            c2rust_current_block = 9699707990742192723;
                        }
                        68 => {
                            yyval.varList.last =
                                (*yyvsp.offset(0 as i32 as isize)).var;
                            yyval.varList.head = yyval.varList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        69 => {
                            yyval.var = VarCreate(
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        70 => {
                            yyval.var = VarCreate(
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        71 => {
                            yyval.var = BoolVarCreate(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                                true,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        72 => {
                            yyval.var = BoolVarCreate(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                                false,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        73 => {
                            yyval.var = VarCreate(
                                std::ptr::null_mut(),
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        74 => {
                            yyval.expr = (*yyvsp.offset(-1 as i32 as isize))
                                .exprList
                                .head;
                            c2rust_current_block = 9699707990742192723;
                        }
                        75 => {
                            let mut list: C2Rust_Unnamed_9 = C2Rust_Unnamed_9 {
                                head: (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .head,
                                last: (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .last,
                            };
                            let mut k: u32 = 0 as u32;
                            while k
                                < (*yyvsp.offset(-3 as i32 as isize))
                                    .noSymbolOrActionList
                            {
                                let syms: *mut ExprDef =
                                    ExprCreateKeySymList(XKB_KEY_NoSymbol as xkb_keysym_t)
                                        as *mut ExprDef;
                                if syms.is_null() {
                                    c2rust_current_block = 7267896227379959561;
                                    break 's_60;
                                }
                                (*syms).common.next =
                                    &raw mut (*list.head).common as *mut _ParseCommon;
                                list.head = syms;
                                k = k.wrapping_add(1);
                            }
                            yyval.expr = list.head;
                            c2rust_current_block = 9699707990742192723;
                        }
                        76 => {
                            yyval.expr = (*yyvsp.offset(-1 as i32 as isize))
                                .exprList
                                .head;
                            c2rust_current_block = 9699707990742192723;
                        }
                        77 => {
                            let mut list_0: C2Rust_Unnamed_8 = C2Rust_Unnamed_8 {
                                head: (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .head,
                                last: (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .last,
                            };
                            let mut k_0: u32 = 0 as u32;
                            while k_0
                                < (*yyvsp.offset(-3 as i32 as isize))
                                    .noSymbolOrActionList
                            {
                                let acts: *mut ExprDef =
                                    ExprCreateActionList(std::ptr::null_mut())
                                        as *mut ExprDef;
                                if acts.is_null() {
                                    c2rust_current_block = 7267896227379959561;
                                    break 's_60;
                                }
                                (*acts).common.next =
                                    &raw mut (*list_0.head).common as *mut _ParseCommon;
                                list_0.head = acts;
                                k_0 = k_0.wrapping_add(1);
                            }
                            yyval.expr = list_0.head;
                            c2rust_current_block = 9699707990742192723;
                        }
                        78 => {
                            yyval.expr = ExprEmptyList();
                            c2rust_current_block = 9699707990742192723;
                        }
                        79 => {
                            yyval.noSymbolOrActionList = (*yyvsp
                                .offset(-3 as i32 as isize))
                            .noSymbolOrActionList
                            .wrapping_add(1 as u32);
                            c2rust_current_block = 9699707990742192723;
                        }
                        80 => {
                            yyval.noSymbolOrActionList = 1 as u32;
                            c2rust_current_block = 9699707990742192723;
                        }
                        81 => {
                            yyval.noSymbolOrActionList = 0 as u32;
                            c2rust_current_block = 9699707990742192723;
                        }
                        82 => {
                            yyval.groupCompat = GroupCompatCreate(
                                (*yyvsp.offset(-3 as i32 as isize)).num,
                                (*yyvsp.offset(-1 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        83 => {
                            yyval.modMask = ModMapCreate(
                                (*yyvsp.offset(-4 as i32 as isize)).atom,
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .exprList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        84 => {
                            yyval.exprList.head = (*yyvsp
                                .offset(-2 as i32 as isize))
                            .exprList
                            .head;
                            (*yyval.exprList.last).common.next =
                                &raw mut (*(*yyvsp.offset(0 as i32 as isize)).expr)
                                    .common as *mut _ParseCommon;
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        85 => {
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        86 => {
                            yyval.expr = ExprCreateKeyName(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        87 => {
                            yyval.expr = ExprCreateKeySym(
                                (*yyvsp.offset(0 as i32 as isize)).keysym,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        88 => {
                            yyval.ledMap = LedMapCreate(
                                (*yyvsp.offset(-4 as i32 as isize)).atom,
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .varList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        89 => {
                            yyval.ledName = LedNameCreate(
                                (*yyvsp.offset(-3 as i32 as isize)).num,
                                (*yyvsp.offset(-1 as i32 as isize)).expr,
                                false,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        90 => {
                            yyval.ledName = LedNameCreate(
                                (*yyvsp.offset(-3 as i32 as isize)).num,
                                (*yyvsp.offset(-1 as i32 as isize)).expr,
                                true,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        91 => {
                            FreeStmt(
                                (*yyvsp.offset(-3 as i32 as isize)).expr
                                    as *mut ParseCommon,
                            );
                            FreeStmt(
                                (*yyvsp.offset(-1 as i32 as isize)).expr
                                    as *mut ParseCommon,
                            );
                            yyval.unknown = UnknownStatementCreate(
                                STMT_UNKNOWN_DECLARATION,
                                (*yyvsp.offset(-4 as i32 as isize)).sval,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        92 => {
                            FreeStmt(
                                (*yyvsp.offset(-4 as i32 as isize)).expr
                                    as *mut ParseCommon,
                            );
                            FreeStmt(
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .varList
                                    .head as *mut ParseCommon,
                            );
                            yyval.unknown = UnknownStatementCreate(
                                STMT_UNKNOWN_COMPOUND,
                                (*yyvsp.offset(-5 as i32 as isize)).sval,
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
                            FreeStmt(
                                (*yyvsp.offset(0 as i32 as isize)).var
                                    as *mut ParseCommon,
                            );
                            yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                            c2rust_current_block = 9699707990742192723;
                        }
                        100 => {
                            yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                            c2rust_current_block = 9699707990742192723;
                        }
                        101 => {
                            FreeStmt(
                                (*yyvsp.offset(0 as i32 as isize)).ledMap
                                    as *mut ParseCommon,
                            );
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
                            FreeStmt(
                                (*yyvsp.offset(0 as i32 as isize)).var
                                    as *mut ParseCommon,
                            );
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
                            FreeStmt(
                                (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .head as *mut ParseCommon,
                            );
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
                            FreeStmt(
                                (*yyvsp.offset(0 as i32 as isize)).expr
                                    as *mut ParseCommon,
                            );
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
                            FreeStmt(
                                (*yyvsp.offset(-2 as i32 as isize))
                                    .varList
                                    .head as *mut ParseCommon,
                            );
                            yyval.geom = std::ptr::null_mut::<core::ffi::c_void>();
                            c2rust_current_block = 9699707990742192723;
                        }
                        124 => {
                            yyval.num = 0 as i64;
                            c2rust_current_block = 9699707990742192723;
                        }
                        125 => {
                            yyval.num = 0 as i64;
                            c2rust_current_block = 9699707990742192723;
                        }
                        126 => {
                            yyval.num = 0 as i64;
                            c2rust_current_block = 9699707990742192723;
                        }
                        127 => {
                            yyval.num = 0 as i64;
                            c2rust_current_block = 9699707990742192723;
                        }
                        128 => {
                            yyval.atom = (*yyvsp.offset(0 as i32 as isize)).atom;
                            c2rust_current_block = 9699707990742192723;
                        }
                        129 => {
                            yyval.atom = (*yyvsp.offset(0 as i32 as isize)).atom;
                            c2rust_current_block = 9699707990742192723;
                        }
                        130 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"action\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 7]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        131 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"interpret\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 10]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        132 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"type\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 5]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        133 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"key\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 4]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        134 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"group\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 6]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        135 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"modifier_map\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 13]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        136 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"indicator\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 10]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        137 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"shape\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 6]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        138 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"row\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 4]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        139 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"section\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 8]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        140 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"text\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 5]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        141 => {
                            yyval.merge = (*yyvsp.offset(0 as i32 as isize)).merge;
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
                            let mut loc: scanner_loc = scanner_token_location((*param).scanner);
                            xkb_logf!(
                                (*(*param).scanner).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "{}:{}:{}: ignored unsupported legacy merge mode \"alternate\"\n",
                                crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                loc.line,
                                loc.column,
                            );
                            yyval.merge = MERGE_DEFAULT;
                            c2rust_current_block = 9699707990742192723;
                        }
                        148 => {
                            if !(*yyvsp.offset(0 as i32 as isize))
                                .expr
                                .is_null()
                            {
                                if !(*yyvsp.offset(-2 as i32 as isize))
                                    .exprList
                                    .head
                                    .is_null()
                                {
                                    yyval.exprList.head = (*yyvsp
                                        .offset(-2 as i32 as isize))
                                    .exprList
                                    .head;
                                    (*yyval.exprList.last).common.next = &raw mut (*(*yyvsp
                                        .offset(0 as i32 as isize))
                                    .expr)
                                        .common
                                        as *mut _ParseCommon;
                                    yyval.exprList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).expr;
                                } else {
                                    yyval.exprList.last =
                                        (*yyvsp.offset(0 as i32 as isize)).expr;
                                    yyval.exprList.head = yyval.exprList.last;
                                }
                            }
                            c2rust_current_block = 9699707990742192723;
                        }
                        149 => {
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        150 => {
                            yyval.exprList.last = std::ptr::null_mut();
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        151 => {
                            yyval.expr = ExprCreateBinary(
                                STMT_EXPR_DIVIDE,
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        152 => {
                            yyval.expr = ExprCreateBinary(
                                STMT_EXPR_ADD,
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        153 => {
                            yyval.expr = ExprCreateBinary(
                                STMT_EXPR_SUBTRACT,
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        154 => {
                            yyval.expr = ExprCreateBinary(
                                STMT_EXPR_MULTIPLY,
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        155 => {
                            yyval.expr = ExprCreateBinary(
                                STMT_EXPR_ASSIGN,
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        156 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        157 => {
                            yyval.expr = ExprCreateUnary(
                                STMT_EXPR_NEGATE,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        158 => {
                            yyval.expr = ExprCreateUnary(
                                STMT_EXPR_UNARY_PLUS,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        159 => {
                            yyval.expr = ExprCreateUnary(
                                STMT_EXPR_NOT,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        160 => {
                            yyval.expr = ExprCreateUnary(
                                STMT_EXPR_INVERT,
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        161 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        162 => {
                            yyval.expr = ExprCreateAction(
                                (*yyvsp.offset(-3 as i32 as isize)).atom,
                                (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        163 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        164 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        165 => {
                            yyval.expr = (*yyvsp.offset(-1 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        166 => {
                            let mut expr: *mut ExprDef = ExprCreateActionList(
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            yyval.exprList =
                                (*yyvsp.offset(-2 as i32 as isize)).exprList;
                            (*yyval.exprList.last).common.next =
                                &raw mut (*expr).common as *mut _ParseCommon;
                            yyval.exprList.last = expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        167 => {
                            yyval.exprList =
                                (*yyvsp.offset(-2 as i32 as isize)).exprList;
                            (*yyval.exprList.last).common.next =
                                &raw mut (*(*yyvsp.offset(0 as i32 as isize)).expr)
                                    .common as *mut _ParseCommon;
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        168 => {
                            yyval.exprList.last = ExprCreateActionList(
                                (*yyvsp.offset(0 as i32 as isize)).expr,
                            );
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        169 => {
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        170 => {
                            yyval.exprList =
                                (*yyvsp.offset(-2 as i32 as isize)).exprList;
                            (*yyval.exprList.last).common.next =
                                &raw mut (*(*yyvsp.offset(0 as i32 as isize)).expr)
                                    .common as *mut _ParseCommon;
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        171 => {
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        172 => {
                            yyval.expr = ExprCreateActionList(
                                (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        173 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        174 => {
                            yyval.expr = ExprCreateActionList(std::ptr::null_mut());
                            c2rust_current_block = 9699707990742192723;
                        }
                        175 => {
                            yyval.expr = ExprCreateAction(
                                (*yyvsp.offset(-3 as i32 as isize)).atom,
                                (*yyvsp.offset(-1 as i32 as isize))
                                    .exprList
                                    .head,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        176 => {
                            yyval.expr = ExprCreateIdent(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        177 => {
                            yyval.expr = ExprCreateFieldRef(
                                (*yyvsp.offset(-2 as i32 as isize)).atom,
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        178 => {
                            yyval.expr = ExprCreateArrayRef(
                                XKB_ATOM_NONE as xkb_atom_t,
                                (*yyvsp.offset(-3 as i32 as isize)).atom,
                                (*yyvsp.offset(-1 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        179 => {
                            yyval.expr = ExprCreateArrayRef(
                                (*yyvsp.offset(-5 as i32 as isize)).atom,
                                (*yyvsp.offset(-3 as i32 as isize)).atom,
                                (*yyvsp.offset(-1 as i32 as isize)).expr,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        180 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        181 => {
                            yyval.expr = std::ptr::null_mut();
                            c2rust_current_block = 9699707990742192723;
                        }
                        182 => {
                            yyval.expr = ExprCreateString(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        183 => {
                            yyval.expr = ExprCreateInteger(
                                (*yyvsp.offset(0 as i32 as isize)).num,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        184 => {
                            yyval.expr = ExprCreateFloat();
                            c2rust_current_block = 9699707990742192723;
                        }
                        185 => {
                            yyval.expr = ExprCreateKeyName(
                                (*yyvsp.offset(0 as i32 as isize)).atom,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        186 => {
                            let mut expr_0: *mut ExprDef = ExprCreateKeySymList(
                                (*yyvsp.offset(0 as i32 as isize)).keysym,
                            );
                            yyval.exprList =
                                (*yyvsp.offset(-2 as i32 as isize)).exprList;
                            (*yyval.exprList.last).common.next =
                                &raw mut (*expr_0).common as *mut _ParseCommon;
                            yyval.exprList.last = expr_0;
                            c2rust_current_block = 9699707990742192723;
                        }
                        187 => {
                            yyval.exprList =
                                (*yyvsp.offset(-2 as i32 as isize)).exprList;
                            (*yyval.exprList.last).common.next =
                                &raw mut (*(*yyvsp.offset(0 as i32 as isize)).expr)
                                    .common as *mut _ParseCommon;
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        188 => {
                            yyval.exprList.last = ExprCreateKeySymList(
                                (*yyvsp.offset(0 as i32 as isize)).keysym,
                            );
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        189 => {
                            yyval.exprList.last =
                                (*yyvsp.offset(0 as i32 as isize)).expr;
                            yyval.exprList.head = yyval.exprList.last;
                            c2rust_current_block = 9699707990742192723;
                        }
                        190 => {
                            yyval.expr = ExprAppendKeySymList(
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).keysym,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        191 => {
                            yyval.expr = ExprKeySymListAppendString(
                                (*param).scanner,
                                (*yyvsp.offset(-2 as i32 as isize)).expr,
                                (*yyvsp.offset(0 as i32 as isize)).str,
                            );
                            free(
                                (*yyvsp.offset(0 as i32 as isize)).str
                                    as *mut ::core::ffi::c_void,
                            );
                            if yyval.expr.is_null() {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                c2rust_current_block = 9699707990742192723;
                            }
                        }
                        192 => {
                            yyval.expr = ExprCreateKeySymList(
                                (*yyvsp.offset(0 as i32 as isize)).keysym,
                            );
                            if yyval.expr.is_null() {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                c2rust_current_block = 9699707990742192723;
                            }
                        }
                        193 => {
                            yyval.expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as xkb_keysym_t);
                            if yyval.expr.is_null() {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                yyval.expr = ExprKeySymListAppendString(
                                    (*param).scanner,
                                    yyval.expr,
                                    (*yyvsp.offset(0 as i32 as isize)).str,
                                );
                                free(
                                    (*yyvsp.offset(0 as i32 as isize)).str
                                        as *mut ::core::ffi::c_void,
                                );
                                if yyval.expr.is_null() {
                                    c2rust_current_block = 9017681648503218951;
                                } else {
                                    c2rust_current_block = 9699707990742192723;
                                }
                            }
                        }
                        194 => {
                            yyval.expr = (*yyvsp.offset(-1 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        195 => {
                            yyval.expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as xkb_keysym_t);
                            if yyval.expr.is_null() {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                yyval.expr = ExprKeySymListAppendString(
                                    (*param).scanner,
                                    yyval.expr,
                                    (*yyvsp.offset(0 as i32 as isize)).str,
                                );
                                free(
                                    (*yyvsp.offset(0 as i32 as isize)).str
                                        as *mut ::core::ffi::c_void,
                                );
                                if yyval.expr.is_null() {
                                    c2rust_current_block = 9017681648503218951;
                                } else {
                                    c2rust_current_block = 9699707990742192723;
                                }
                            }
                        }
                        196 => {
                            yyval.expr = (*yyvsp.offset(0 as i32 as isize)).expr;
                            c2rust_current_block = 9699707990742192723;
                        }
                        197 => {
                            yyval.expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as xkb_keysym_t);
                            c2rust_current_block = 9699707990742192723;
                        }
                        198 => {
                            yyval.keysym = (*yyvsp.offset(0 as i32 as isize)).keysym;
                            c2rust_current_block = 9699707990742192723;
                        }
                        199 => {
                            yyval.keysym = KeysymParseString(
                                (*param).scanner,
                                (*yyvsp.offset(0 as i32 as isize)).str,
                            );
                            free(
                                (*yyvsp.offset(0 as i32 as isize)).str
                                    as *mut ::core::ffi::c_void,
                            );
                            if yyval.keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
                                c2rust_current_block = 9017681648503218951;
                            } else {
                                c2rust_current_block = 9699707990742192723;
                            }
                        }
                        200 => {
                            if !resolve_keysym(
                                param,
                                (*yyvsp.offset(0 as i32 as isize)).sval,
                                &raw mut yyval.keysym,
                            ) {
                                let mut loc_0: scanner_loc =
                                    scanner_token_location((*param).scanner);
                                xkb_logf!(
                                    (*(*param).scanner).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"{}\"\n",
                                    XKB_WARNING_UNRECOGNIZED_KEYSYM as i32,
                                    crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                    loc_0.line,
                                    loc_0.column,
                                    crate::xkb::utils::CStrNDisplay(
                                        (*yyvsp.offset(0 as isize)).sval.len as usize,
                                        (*yyvsp.offset(0 as isize)).sval.start
                                    ),
                                );
                                yyval.keysym = XKB_KEY_NoSymbol as xkb_keysym_t;
                            }
                            c2rust_current_block = 9699707990742192723;
                        }
                        201 => {
                            yyval.keysym = XKB_KEY_section as xkb_keysym_t;
                            c2rust_current_block = 9699707990742192723;
                        }
                        202 => {
                            yyval.keysym = (XKB_KEY_0 as xkb_keysym_t).wrapping_add(
                                (*yyvsp.offset(0 as i32 as isize)).num
                                    as xkb_keysym_t,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        203 => {
                            if (*yyvsp.offset(0 as i32 as isize)).num
                                < XKB_KEYSYM_MIN as i64
                            {
                                let mut loc_1: scanner_loc =
                                    scanner_token_location((*param).scanner);
                                xkb_logf!(
                                    (*(*param).scanner).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"-{:#06x}\" ({})\n",
                                    XKB_ERROR_INVALID_NUMERIC_KEYSYM as i32,
                                    crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                    loc_1.line,
                                    loc_1.column,
                                    -(*yyvsp.offset(0 as i32 as isize)).num,
                                    (*yyvsp.offset(0 as i32 as isize)).num,
                                );
                                yyval.keysym = XKB_KEY_NoSymbol as xkb_keysym_t;
                            } else {
                                if (*yyvsp.offset(0 as i32 as isize)).num
                                    <= XKB_KEYSYM_MAX as i64
                                {
                                    yyval.keysym = (*yyvsp.offset(0 as i32 as isize))
                                        .num
                                        as xkb_keysym_t;
                                    if ((*(*param).ctx).log_verbosity >= 2 as i32)
                                        as i32
                                        as i64
                                        != 0
                                    {
                                        let mut ref_name: *const i8 = std::ptr::null();
                                        if xkb_keysym_is_deprecated(
                                            yyval.keysym,
                                            std::ptr::null(),
                                            &raw mut ref_name,
                                        ) {
                                            if ref_name.is_null() {
                                                let mut loc_2: scanner_loc =
                                                    scanner_token_location((*param).scanner);
                                                xkb_logf!(
                                                    (*(*param).scanner).ctx,
                                                    XKB_LOG_LEVEL_WARNING,
                                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                                    "[XKB-{:03}] {}:{}:{}: deprecated keysym \"{:#06x}\".\n",
                                                    XKB_WARNING_DEPRECATED_KEYSYM as i32,
                                                    crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                                    loc_2.line,
                                                    loc_2.column,
                                                    yyval.keysym,
                                                );
                                            } else {
                                                let mut loc_3: scanner_loc =
                                                    scanner_token_location((*param).scanner);
                                                xkb_logf!(
                                                    (*(*param).scanner).ctx,
                                                    XKB_LOG_LEVEL_WARNING,
                                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                                    "[XKB-{:03}] {}:{}:{}: deprecated keysym name \"{:#06x}\"; please use \"{}\" instead.\n",
                                                    XKB_WARNING_DEPRECATED_KEYSYM_NAME as i32,
                                                    crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                                    loc_3.line,
                                                    loc_3.column,
                                                    yyval.keysym,
                                                    crate::xkb::utils::CStrDisplay(ref_name),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    let mut loc_4: scanner_loc =
                                        scanner_token_location((*param).scanner);
                                    xkb_logf!(
                                        (*(*param).scanner).ctx,
                                        XKB_LOG_LEVEL_WARNING,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"{:#06x}\" ({})\n",
                                        XKB_ERROR_INVALID_NUMERIC_KEYSYM as i32,
                                        crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                        loc_4.line,
                                        loc_4.column,
                                        (*yyvsp.offset(0 as i32 as isize)).num,
                                        (*yyvsp.offset(0 as i32 as isize)).num,
                                    );
                                    yyval.keysym = XKB_KEY_NoSymbol as xkb_keysym_t;
                                }
                                let mut loc_5: scanner_loc =
                                    scanner_token_location((*param).scanner);
                                xkb_logf!(
                                    (*(*param).scanner).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_COMPREHENSIVE as i32,
                                    "[XKB-{:03}] {}:{}:{}: numeric keysym \"{:#06x}\" ({})\n",
                                    XKB_WARNING_NUMERIC_KEYSYM as i32,
                                    crate::xkb::utils::CStrDisplay((*(*param).scanner).file_name),
                                    loc_5.line,
                                    loc_5.column,
                                    (*yyvsp.offset(0 as i32 as isize)).num,
                                    (*yyvsp.offset(0 as i32 as isize)).num,
                                );
                            }
                            c2rust_current_block = 9699707990742192723;
                        }
                        204 => {
                            yyval.num = -(*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        205 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        206 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        207 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        208 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        209 => {
                            yyval.num = 0 as i64;
                            c2rust_current_block = 9699707990742192723;
                        }
                        210 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        211 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        212 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        213 => {
                            yyval.num = (*yyvsp.offset(0 as i32 as isize)).num;
                            c2rust_current_block = 9699707990742192723;
                        }
                        214 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                (*yyvsp.offset(0 as i32 as isize)).sval.start,
                                (*yyvsp.offset(0 as i32 as isize)).sval.len,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        215 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                b"default\0".as_ptr() as *const i8,
                                (::core::mem::size_of::<[i8; 8]>() as usize)
                                    .wrapping_sub(1 as usize),
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        216 => {
                            yyval.atom = xkb_atom_intern(
                                (*param).ctx,
                                (*yyvsp.offset(0 as i32 as isize)).str,
                                cstr_len((*yyvsp.offset(0 as i32 as isize)).str),
                            );
                            free(
                                (*yyvsp.offset(0 as i32 as isize)).str
                                    as *mut ::core::ffi::c_void,
                            );
                            c2rust_current_block = 9699707990742192723;
                        }
                        217 => {
                            yyval.str = (*yyvsp.offset(0 as i32 as isize)).str;
                            c2rust_current_block = 9699707990742192723;
                        }
                        218 => {
                            yyval.str = std::ptr::null_mut();
                            c2rust_current_block = 9699707990742192723;
                        }
                        219 => {
                            yyval.str = (*yyvsp.offset(0 as i32 as isize)).str;
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
                            yylen = 0 as i32;
                            yystate = *yyssp as yy_state_fast_t;
                            c2rust_current_block = 12965144090463719536;
                        }
                        _ => {
                            yyvsp = yyvsp.offset(-(yylen as isize));
                            yyssp = yyssp.offset(-(yylen as isize));
                            yylen = 0 as i32;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = yyval;
                            let yylhs: i32 =
                                yyr1[yyn as usize] as i32 - YYNTOKENS;
                            let yyi: i32 = yypgoto[yylhs as usize]
                                as i32
                                + *yyssp as i32;
                            yystate = (if 0 as i32 <= yyi
                                && yyi <= YYLAST
                                && yycheck[yyi as usize] as i32
                                    == *yyssp as i32
                            {
                                yytable[yyi as usize] as i32
                            } else {
                                yydefgoto[yylhs as usize] as i32
                            }) as yy_state_fast_t;
                            c2rust_current_block = 10430565463943277256;
                        }
                    }
                }
                _ => {}
            }
            match c2rust_current_block {
                12965144090463719536 => {
                    yyerrstatus = 3 as i32;
                    loop {
                        yyn = yypact[yystate as usize] as i32;
                        if !(yyn == YYPACT_NINF) {
                            yyn += YYSYMBOL_YYerror as i32;
                            if 0 as i32 <= yyn
                                && yyn <= YYLAST
                                && yycheck[yyn as usize] as i32
                                    == YYSYMBOL_YYerror as i32
                            {
                                yyn = yytable[yyn as usize] as i32;
                                if (0 as i32) < yyn {
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
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                            param,
                        );
                        yyvsp = yyvsp.offset(-(1 as i32 as isize));
                        yyssp = yyssp.offset(-(1 as i32 as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn as yy_state_fast_t;
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
        match c2rust_current_block {
            5508412643396263508 => {
                yyresult = 0 as i32;
            }
            7267896227379959561 => {
                yyresult = 1 as i32;
            }
            _ => {
                _xkbcommon_error(param, b"memory exhausted\0".as_ptr() as *const i8);
                yyresult = 2 as i32;
            }
        }
        if yychar != YYEMPTY as i32 {
            yytoken = (if 0 as i32 <= yychar && yychar <= YYMAXUTOK {
                yytranslate[yychar as usize] as yysymbol_kind_t as i32
            } else {
                YYSYMBOL_YYUNDEF as i32
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
                yystos[*yyssp as i32 as usize] as yysymbol_kind_t,
                yyvsp,
                param,
            );
            yyvsp = yyvsp.offset(-(1 as i32 as isize));
            yyssp = yyssp.offset(-(1 as i32 as isize));
        }
        if yyss != &raw mut yyssa as *mut yy_state_t {
            free(yyss as *mut ::core::ffi::c_void);
        }
        if yymsg != &raw mut yymsgbuf as *mut i8 {
            free(yymsg as *mut ::core::ffi::c_void);
        }
        return yyresult;
    }
}
use crate::xkb::shared_types::*;
