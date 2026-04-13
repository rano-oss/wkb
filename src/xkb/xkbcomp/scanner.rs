use crate::xkb::context_priv::xkb_atom_intern;
use crate::xkb::shared_types::*;
use crate::xkb_logf;

use crate::xkb::text::LookupEntry;
pub mod parser_h {
    pub type yytokentype = i32;
    pub const ALTERNATE_GROUP: yytokentype = 77;
    pub const FUNCTION_KEYS: yytokentype = 76;
    pub const KEYPAD_KEYS: yytokentype = 75;
    pub const MODIFIER_KEYS: yytokentype = 74;
    pub const ALPHANUMERIC_KEYS: yytokentype = 73;
    pub const HIDDEN: yytokentype = 72;
    pub const DEFAULT: yytokentype = 71;
    pub const PARTIAL: yytokentype = 70;
    pub const KEYNAME: yytokentype = 65;
    pub const IDENT: yytokentype = 64;
    pub const FLOAT: yytokentype = 63;
    pub const INTEGER: yytokentype = 62;
    pub const DECIMAL_DIGIT: yytokentype = 61;
    pub const STRING: yytokentype = 60;
    pub const INVERT: yytokentype = 55;
    pub const EXCLAM: yytokentype = 54;
    pub const SEMI: yytokentype = 53;
    pub const COMMA: yytokentype = 52;
    pub const DOT: yytokentype = 51;
    pub const CBRACKET: yytokentype = 50;
    pub const OBRACKET: yytokentype = 49;
    pub const CPAREN: yytokentype = 48;
    pub const OPAREN: yytokentype = 47;
    pub const CBRACE: yytokentype = 46;
    pub const OBRACE: yytokentype = 45;
    pub const TIMES: yytokentype = 44;
    pub const DIVIDE: yytokentype = 43;
    pub const MINUS: yytokentype = 42;
    pub const PLUS: yytokentype = 41;
    pub const EQUALS: yytokentype = 40;
    pub const VIRTUAL: yytokentype = 38;
    pub const LOGO: yytokentype = 37;
    pub const SOLID: yytokentype = 36;
    pub const OUTLINE: yytokentype = 35;
    pub const TEXT: yytokentype = 34;
    pub const OVERLAY: yytokentype = 33;
    pub const SECTION: yytokentype = 32;
    pub const ROW: yytokentype = 31;
    pub const KEYS: yytokentype = 30;
    pub const SHAPE: yytokentype = 29;
    pub const INDICATOR: yytokentype = 28;
    pub const MODIFIER_MAP: yytokentype = 27;
    pub const GROUP: yytokentype = 26;
    pub const ALIAS: yytokentype = 25;
    pub const KEY: yytokentype = 24;
    pub const ACTION_TOK: yytokentype = 23;
    pub const INTERPRET: yytokentype = 22;
    pub const TYPE: yytokentype = 21;
    pub const VIRTUAL_MODS: yytokentype = 20;
    pub const ALTERNATE: yytokentype = 14;
    pub const REPLACE: yytokentype = 13;
    pub const AUGMENT: yytokentype = 12;
    pub const OVERRIDE: yytokentype = 11;
    pub const INCLUDE: yytokentype = 10;
    pub const XKB_LAYOUT: yytokentype = 8;
    pub const XKB_SEMANTICS: yytokentype = 7;
    pub const XKB_GEOMETRY: yytokentype = 6;
    pub const XKB_COMPATMAP: yytokentype = 5;
    pub const XKB_SYMBOLS: yytokentype = 4;
    pub const XKB_TYPES: yytokentype = 3;
    pub const XKB_KEYCODES: yytokentype = 2;
    pub const XKB_KEYMAP: yytokentype = 1;
    pub const ERROR_TOK: yytokentype = 255;
    pub const YYUNDEF: yytokentype = 257;
    pub const YYerror: yytokentype = 256;
    pub const END_OF_FILE: yytokentype = 0;
    pub const YYEMPTY: yytokentype = -2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union YYSTYPE {
        pub num: i64,
        pub file_type: xkb_file_type,
        pub str: *mut i8,
        pub sval: sval,
        pub atom: xkb_atom_t,
        pub merge: merge_mode,
        pub mapFlags: xkb_map_flags,
        pub keysym: xkb_keysym_t,
        pub any: *mut ParseCommon,
        pub anyList: C2Rust_Unnamed_6,
        pub noSymbolOrActionList: u32,
        pub expr: *mut ExprDef,
        pub exprList: C2Rust_Unnamed_5,
        pub var: *mut VarDef,
        pub varList: C2Rust_Unnamed_4,
        pub vmod: *mut VModDef,
        pub vmodList: C2Rust_Unnamed_3,
        pub interp: *mut InterpDef,
        pub keyType: *mut KeyTypeDef,
        pub syms: *mut SymbolsDef,
        pub modMask: *mut ModMapDef,
        pub groupCompat: *mut GroupCompatDef,
        pub ledMap: *mut LedMapDef,
        pub ledName: *mut LedNameDef,
        pub keyCode: *mut KeycodeDef,
        pub keyAlias: *mut KeyAliasDef,
        pub unknown: *mut UnknownStatement,
        pub geom: *mut ::core::ffi::c_void,
        pub file: *mut XkbFile,
        pub fileList: C2Rust_Unnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_2 {
        pub head: *mut XkbFile,
        pub last: *mut XkbFile,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_3 {
        pub head: *mut VModDef,
        pub last: *mut VModDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        pub head: *mut VarDef,
        pub last: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        pub head: *mut ExprDef,
        pub last: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_6 {
        pub head: *mut ParseCommon,
        pub last: *mut ParseCommon,
    }
    use crate::xkb::scanner_utils::sval;
    use crate::xkb::shared_ast_types::{
        merge_mode, xkb_file_type, xkb_map_flags, ExprDef, GroupCompatDef, InterpDef, KeyAliasDef,
        KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef, ParseCommon, SymbolsDef,
        UnknownStatement, VModDef, VarDef, XkbFile,
    };
    use crate::xkb::shared_types::xkb_atom_t;
    use crate::xkb::shared_types::xkb_keysym_t;
}
pub mod utf8_h {

    /// Native Rust UTF-32 to UTF-8 conversion (replaces C FFI)
    ///
    /// Writes UTF-8 bytes to buffer (including null terminator).
    /// Returns total bytes written (including null), or 0 on failure.
    #[inline]
    pub fn utf32_to_utf8(unichar: u32, buffer: *mut i8) -> i32 {
        if buffer.is_null() {
            return 0;
        }

        let Some(ch) = char::from_u32(unichar) else {
            return 0;
        };

        unsafe {
            let mut tmp = [0u8; 4];
            let utf8_bytes = ch.encode_utf8(&mut tmp).as_bytes();

            std::ptr::copy_nonoverlapping(utf8_bytes.as_ptr(), buffer as *mut u8, utf8_bytes.len());

            // Null terminate
            *buffer.add(utf8_bytes.len()) = 0;

            // Return length + 1 for null terminator
            (utf8_bytes.len() + 1) as i32
        }
    }
}

// Re-export parse functions from parser module
pub use super::parser::{parse, parse_next};

pub use crate::xkb::xkbcomp::keywords::keyword_to_token;

pub use self::parser_h::{
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
pub use crate::xkb::scanner_utils::{
    scanner, scanner_buf_append, scanner_buf_appends_code_point,
    scanner_check_supported_char_encoding, scanner_chr, scanner_dec_int64, scanner_eof,
    scanner_eol, scanner_hex_int64, scanner_init, scanner_loc, scanner_next, scanner_oct,
    scanner_peek, scanner_skip_to_eol, scanner_str, scanner_token_location,
    scanner_unicode_code_point, sval,
};
pub use crate::xkb::shared_ast_types::{
    _ParseCommon, merge_mode, stmt_type, xkb_file_type, xkb_map_flags, C2Rust_Unnamed_1,
    ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef,
    ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary,
    GroupCompatDef, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef,
    ModMapDef, ParseCommon, SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile,
    _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT,
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
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::utils::cstr_dup;
pub use crate::xkb::utils::{
    digits__, parse_dec_to_uint64_t, parse_hex_to_uint32_t, parse_hex_to_uint64_t,
};
pub use crate::xkb::utils::{
    is_alnum, is_alpha, is_ascii, is_digit, is_graph, is_space, is_valid_char, is_xdigit,
};
use libc::FILE;
pub static mut DECIMAL_SEPARATOR: i8 = '.' as i32 as i8;
unsafe fn number(
    mut s: *mut scanner,
    mut out: *mut i64,
    mut out_tok: *mut i32,
) -> bool {
    unsafe {
        if scanner_str(
            s,
            b"0x\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 3]>()).wrapping_sub(1 as usize),
        ) {
            match scanner_hex_int64(s, out) {
                -1 => {
                    *out_tok = ERROR_TOK as i32;
                    return true;
                }
                0 => return false,
                _ => {
                    *out_tok = INTEGER as i32;
                    return true;
                }
            }
        } else {
            let mut is_digit_0: bool = false;
            match scanner_dec_int64(s, out) {
                -1 => {
                    *out_tok = ERROR_TOK as i32;
                    return true;
                }
                0 => return false,
                1 => {
                    is_digit_0 = true;
                }
                _ => {}
            }
            if scanner_chr(s, DECIMAL_SEPARATOR) {
                let mut dec: i64 = 0;
                if scanner_dec_int64(s, &raw mut dec) < 0 as i32 {
                    *out_tok = ERROR_TOK as i32;
                    return true;
                }
                *out_tok = FLOAT as i32;
            } else if is_digit_0 {
                *out_tok = DECIMAL_DIGIT as i32;
            } else {
                *out_tok = INTEGER as i32;
            }
            return true;
        };
    }
}
pub unsafe fn _xkbcommon_lex(mut yylval: *mut YYSTYPE, mut s: *mut scanner) -> i32 {
    unsafe {
        loop {
            while is_space(scanner_peek(s)) {
                scanner_next(s);
            }
            if scanner_str(
                s,
                b"\xE2\x80\x8E\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1 as usize),
            ) as i32
                != 0
                || scanner_str(
                    s,
                    b"\xE2\x80\x8F\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1 as usize),
                ) as i32
                    != 0
            {
                continue;
            }
            if !(scanner_str(
                s,
                b"//\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 3]>()).wrapping_sub(1 as usize),
            ) as i32
                != 0
                || scanner_chr(s, '#' as i32 as i8) as i32 != 0)
            {
                break;
            }
            scanner_skip_to_eol(s);
        }
        if scanner_eof(s) {
            return END_OF_FILE as i32;
        }
        (*s).token_pos = (*s).pos;
        (*s).buf_pos = 0 as usize;
        if scanner_chr(s, '"' as i32 as i8) {
            while !scanner_eof(s)
                && !scanner_eol(s)
                && scanner_peek(s) as i32 != '"' as i32
            {
                if scanner_chr(s, '\\' as i32 as i8) {
                    let mut o: u8 = 0;
                    let start_pos: usize = (*s).pos;
                    if scanner_chr(s, '\\' as i32 as i8) {
                        scanner_buf_append(s, '\\' as i32 as i8);
                    } else if scanner_chr(s, '"' as i32 as i8) {
                        scanner_buf_append(s, '"' as i32 as i8);
                    } else if scanner_chr(s, 'n' as i32 as i8) {
                        scanner_buf_append(s, '\n' as i32 as i8);
                    } else if scanner_chr(s, 't' as i32 as i8) {
                        scanner_buf_append(s, '\t' as i32 as i8);
                    } else if scanner_chr(s, 'r' as i32 as i8) {
                        scanner_buf_append(s, '\r' as i32 as i8);
                    } else if scanner_chr(s, 'b' as i32 as i8) {
                        scanner_buf_append(s, '\u{8}' as i32 as i8);
                    } else if scanner_chr(s, 'f' as i32 as i8) {
                        scanner_buf_append(s, '\u{c}' as i32 as i8);
                    } else if scanner_chr(s, 'v' as i32 as i8) {
                        scanner_buf_append(s, '\u{b}' as i32 as i8);
                    } else if scanner_chr(s, 'e' as i32 as i8) {
                        scanner_buf_append(s, '\u{1b}' as i32 as i8);
                    } else if scanner_chr(s, 'u' as i32 as i8) {
                        let mut cp: u32 = 0 as u32;
                        if scanner_unicode_code_point(s, &raw mut cp) as i32 != 0
                            && is_valid_char(cp) as i32 != 0
                        {
                            scanner_buf_appends_code_point(s, cp);
                        } else {
                            let mut loc: scanner_loc = scanner_token_location(s);
                            xkb_logf!(
                                (*s).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] {}:{}:{}: invalid Unicode escape sequence \"{}\" in string literal\n",
                                XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE
                                    as i32,
                                crate::xkb::utils::CStrDisplay((*s).file_name),
                                loc.line,
                                loc.column,
                                crate::xkb::utils::CStrNDisplay((*s).pos.wrapping_sub(start_pos).wrapping_add(1 as usize), (*s).s.offset(start_pos.wrapping_sub(1 as usize) as isize)
                                    as *const i8),
                            );
                        }
                    } else if scanner_oct(s, &raw mut o) as i32 != 0
                        && is_valid_char(o as u32) as i32 != 0
                    {
                        scanner_buf_append(s, o as i8);
                    } else if (*s).pos > start_pos {
                        let mut loc_0: scanner_loc = scanner_token_location(s);
                        xkb_logf!(
                            (*s).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: invalid octal escape sequence \"{}\" in string literal\n",
                            XKB_WARNING_INVALID_ESCAPE_SEQUENCE as i32,
                            crate::xkb::utils::CStrDisplay((*s).file_name),
                            loc_0.line,
                            loc_0.column,
                            crate::xkb::utils::CStrNDisplay((*s).pos.wrapping_sub(start_pos).wrapping_add(1 as usize), (*s).s.offset(start_pos.wrapping_sub(1 as usize) as isize)
                                as *const i8),
                        );
                    } else {
                        let mut loc_1: scanner_loc = scanner_token_location(s);
                        xkb_logf!(
                            (*s).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: unknown escape sequence \"\\{}\" in string literal\n",
                            XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE
                                as i32,
                            crate::xkb::utils::CStrDisplay((*s).file_name),
                            loc_1.line,
                            loc_1.column,
                            (scanner_peek(s) as i32 as u8 as char),
                        );
                    }
                } else {
                    scanner_buf_append(s, scanner_next(s));
                }
            }
            if !scanner_buf_append(s, '\0' as i32 as i8) || !scanner_chr(s, '"' as i32 as i8) {
                let mut loc_2: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "{}:{}:{}: unterminated string literal\n",
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_2.line,
                    loc_2.column,
                );
                return ERROR_TOK as i32;
            }
            (*yylval).str = cstr_dup(&raw mut (*s).buf as *mut i8);
            if (*yylval).str.is_null() {
                return ERROR_TOK as i32;
            }
            return STRING as i32;
        }
        if scanner_chr(s, '<' as i32 as i8) {
            while is_graph(scanner_peek(s)) as i32 != 0
                && scanner_peek(s) as i32 != '>' as i32
            {
                scanner_next(s);
            }
            if !scanner_chr(s, '>' as i32 as i8) {
                let mut loc_3: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "{}:{}:{}: unterminated key name literal\n",
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_3.line,
                    loc_3.column,
                );
                return ERROR_TOK as i32;
            }
            let mut start: *const i8 = (*s)
                .s
                .offset((*s).token_pos as isize)
                .offset(1 as i32 as isize);
            let len: usize = (*s)
                .pos
                .wrapping_sub((*s).token_pos)
                .wrapping_sub(2 as usize);
            (*yylval).atom = xkb_atom_intern((*s).ctx, start, len);
            return KEYNAME as i32;
        }
        if scanner_chr(s, ';' as i32 as i8) {
            return SEMI as i32;
        }
        if scanner_chr(s, '{' as i32 as i8) {
            return OBRACE as i32;
        }
        if scanner_chr(s, '}' as i32 as i8) {
            return CBRACE as i32;
        }
        if scanner_chr(s, '=' as i32 as i8) {
            return EQUALS as i32;
        }
        if scanner_chr(s, '[' as i32 as i8) {
            return OBRACKET as i32;
        }
        if scanner_chr(s, ']' as i32 as i8) {
            return CBRACKET as i32;
        }
        if scanner_chr(s, '(' as i32 as i8) {
            return OPAREN as i32;
        }
        if scanner_chr(s, ')' as i32 as i8) {
            return CPAREN as i32;
        }
        if scanner_chr(s, '.' as i32 as i8) {
            return DOT as i32;
        }
        if scanner_chr(s, ',' as i32 as i8) {
            return COMMA as i32;
        }
        if scanner_chr(s, '+' as i32 as i8) {
            return PLUS as i32;
        }
        if scanner_chr(s, '-' as i32 as i8) {
            return MINUS as i32;
        }
        if scanner_chr(s, '*' as i32 as i8) {
            return TIMES as i32;
        }
        if scanner_chr(s, '/' as i32 as i8) {
            return DIVIDE as i32;
        }
        if scanner_chr(s, '!' as i32 as i8) {
            return EXCLAM as i32;
        }
        if scanner_chr(s, '~' as i32 as i8) {
            return INVERT as i32;
        }
        let mut tok: i32 = ERROR_TOK as i32;
        if is_alpha(scanner_peek(s)) as i32 != 0
            || scanner_peek(s) as i32 == '_' as i32
        {
            while is_alnum(scanner_peek(s)) as i32 != 0
                || scanner_peek(s) as i32 == '_' as i32
            {
                scanner_next(s);
            }
            let mut start_0: *const i8 = (*s).s.offset((*s).token_pos as isize);
            let len_0: usize = (*s).pos.wrapping_sub((*s).token_pos);
            tok = keyword_to_token(start_0, len_0);
            if tok >= 0 as i32 {
                return tok;
            }
            (*yylval).sval = sval {
                len: len_0,
                start: start_0,
            };
            return IDENT as i32;
        }
        if number(s, &raw mut (*yylval).num, &raw mut tok) {
            if tok == ERROR_TOK as i32 {
                let mut loc_4: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: malformed number literal\n",
                    XKB_ERROR_MALFORMED_NUMBER_LITERAL as i32,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_4.line,
                    loc_4.column,
                );
                return ERROR_TOK as i32;
            }
            return tok;
        }
        let mut loc_5: scanner_loc = scanner_token_location(s);
        xkb_logf!(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "{}:{}:{}: unrecognized token\n",
            crate::xkb::utils::CStrDisplay((*s).file_name),
            loc_5.line,
            loc_5.column,
        );
        return ERROR_TOK as i32;
    }
}
pub unsafe fn XkbParseStringInit(
    mut ctx: *mut xkb_context,
    mut scanner: *mut scanner,
    mut string: *const i8,
    mut len: usize,
    mut file_name: *const i8,
    mut map: *const i8,
) -> bool {
    unsafe {
        scanner_init(
            scanner,
            ctx,
            string,
            len,
            file_name,
            std::ptr::null_mut::<core::ffi::c_void>(),
        );
        if !scanner_check_supported_char_encoding(scanner) {
            let mut loc: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                loc.line,
                loc.column,
            );
            let mut loc_0: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                crate::xkb::utils::CStrDisplay((*scanner).file_name),
                loc_0.line,
                loc_0.column,
            );
            return false;
        }
        return true;
    }
}
pub unsafe fn XkbParseString(
    mut ctx: *mut xkb_context,
    mut string: *const i8,
    mut len: usize,
    mut file_name: *const i8,
    mut map: *const i8,
) -> *mut XkbFile {
    unsafe {
        let mut scanner: scanner = scanner {
            pos: 0,
            len: 0,
            s: std::ptr::null(),
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: scanner_loc { line: 0, column: 0 },
            file_name: std::ptr::null(),
            ctx: std::ptr::null_mut(),
            priv_0: std::ptr::null_mut(),
        };
        if !XkbParseStringInit(ctx, &raw mut scanner, string, len, file_name, map) {
            return std::ptr::null_mut();
        }
        // Cast types between parser and scanner modules (same C struct, different Rust types)
        return parse(ctx as *mut _, &raw mut scanner as *mut _, map) as *mut XkbFile;
    }
}
pub unsafe fn XkbParseStringNext(
    mut ctx: *mut xkb_context,
    mut scanner: *mut scanner,
    mut map: *const i8,
    mut out: *mut *mut XkbFile,
) -> bool {
    unsafe {
        if !map.is_null() {
            // Cast types between parser and scanner modules (same C struct, different Rust types)
            *out = parse(ctx as *mut _, scanner as *mut _, map) as *mut XkbFile;
            return !(*out).is_null();
        } else {
            // Cast types between parser and scanner modules
            return parse_next(ctx as *mut _, scanner as *mut _, out as *mut _);
        };
    }
}
pub unsafe fn XkbParseFile(
    mut ctx: *mut xkb_context,
    mut file: *mut FILE,
    mut file_name: *const i8,
    mut map: *const i8,
) -> *mut XkbFile {
    unsafe {
        // Get file descriptor from FILE*
        let fd = libc::fileno(file as *mut libc::FILE);
        if fd < 0 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Invalid file descriptor\n",
            );
            return std::ptr::null_mut();
        }

        // Create Rust File from file descriptor
        use crate::xkb::utils::MappedFile;
        use std::fs::File;
        use std::os::unix::io::FromRawFd;

        let rust_file = File::from_raw_fd(fd);

        // Map the file
        let mapped = match MappedFile::new(&rust_file) {
            Ok(m) => m,
            Err(e) => {
                let err_msg = std::ffi::CString::new(e.to_string())
                    .unwrap_or_else(|_| std::ffi::CString::new("unknown error").unwrap());
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Couldn't read XKB file {}: {}\n",
                    crate::xkb::utils::CStrDisplay(file_name),
                    crate::xkb::utils::CStrDisplay(err_msg.as_ptr()),
                );
                std::mem::forget(rust_file); // Don't close fd - caller owns FILE*
                return std::ptr::null_mut();
            }
        };

        let xkb_file = XkbParseString(ctx, mapped.as_ptr(), mapped.len(), file_name, map);

        // Keep file descriptor open - don't close it
        std::mem::forget(rust_file);

        return xkb_file;
    }
}
