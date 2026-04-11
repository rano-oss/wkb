pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
    pub type __builtin_va_list = [__va_list_tag; 1];
}

pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod types_h {
    pub type __uint64_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
    pub type __ssize_t = i64;
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut i8,
        pub _IO_read_end: *mut i8,
        pub _IO_read_base: *mut i8,
        pub _IO_write_base: *mut i8,
        pub _IO_write_ptr: *mut i8,
        pub _IO_write_end: *mut i8,
        pub _IO_buf_base: *mut i8,
        pub _IO_buf_end: *mut i8,
        pub _IO_save_base: *mut i8,
        pub _IO_backup_base: *mut i8,
        pub _IO_save_end: *mut i8,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [i8; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [i8; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [i8; 8],
    }
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod stdio_h {
    pub type va_list = __gnuc_va_list;
    pub type ssize_t = __ssize_t;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;

    extern "C" {
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        pub fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
        pub fn snprintf(
            __s: *mut i8,
            __maxlen: usize,
            __format: *const i8,
            ...
        ) -> ::core::ffi::c_int;
        pub fn vsnprintf(
            __s: *mut i8,
            __maxlen: usize,
            __format: *const i8,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
    pub type xkb_message_code = u32;
    pub const _XKB_LOG_MESSAGE_MAX_CODE: xkb_message_code = 971;
    pub const XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE: xkb_message_code = 971;
    pub const XKB_ERROR_INVALID_RULES_SYNTAX: xkb_message_code = 967;
    pub const XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL: xkb_message_code = 965;
    pub const XKB_ERROR_INVALID_IDENTIFIER: xkb_message_code = 949;
    pub const XKB_WARNING_CONFLICTING_KEY_FIELDS: xkb_message_code = 935;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT_: xkb_message_code = 914;
    pub const XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX: xkb_message_code = 903;
    pub const XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY: xkb_message_code = 901;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS: xkb_message_code = 893;
    pub const XKB_WARNING_CONFLICTING_KEY_ACTION: xkb_message_code = 883;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT_: xkb_message_code = 876;
    pub const XKB_ERROR_UNKNOWN_ACTION_TYPE: xkb_message_code = 844;
    pub const XKB_ERROR_KEYMAP_COMPILATION_FAILED: xkb_message_code = 822;
    pub const XKB_ERROR_UNKNOWN_FIELD: xkb_message_code = 812;
    pub const XKB_WARNING_CONFLICTING_MODMAP: xkb_message_code = 800;
    pub const XKB_ERROR_INVALID_VALUE: xkb_message_code = 796;
    pub const XKB_ERROR_INVALID_EXPRESSION_TYPE: xkb_message_code = 784;
    pub const XKB_WARNING_UNDEFINED_KEYCODE: xkb_message_code = 770;
    pub const XKB_ERROR_INVALID_XKB_SYNTAX: xkb_message_code = 769;
    pub const XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION: xkb_message_code = 762;
    pub const XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT: xkb_message_code = 742;
    pub const XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD: xkb_message_code = 711;
    pub const XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE: xkb_message_code = 700;
    pub const XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT: xkb_message_code = 693;
    pub const XKB_ERROR_INVALID_COMPOSE_SYNTAX: xkb_message_code = 685;
    pub const XKB_ERROR_INVALID_COMPOSE_LOCALE: xkb_message_code = 679;
    pub const XKB_ERROR_INVALID_INCLUDED_FILE: xkb_message_code = 661;
    pub const XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE: xkb_message_code = 645;
    pub const XKB_ERROR_UNKNOWN_DEFAULT_FIELD: xkb_message_code = 639;
    pub const XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH: xkb_message_code = 632;
    pub const XKB_ERROR_INVALID_REAL_MODIFIER: xkb_message_code = 623;
    pub const XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE: xkb_message_code = 607;
    pub const XKB_ERROR_CANNOT_RESOLVE_RMLVO: xkb_message_code = 595;
    pub const XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX: xkb_message_code = 588;
    pub const XKB_ERROR_WRONG_FIELD_TYPE: xkb_message_code = 578;
    pub const XKB_ERROR_INVALID_ACTION_FIELD: xkb_message_code = 563;
    pub const XKB_ERROR_ALLOCATION_ERROR: xkb_message_code = 550;
    pub const XKB_ERROR_INVALID_FILE_ENCODING: xkb_message_code = 542;
    pub const XKB_WARNING_CONFLICTING_KEY_NAME: xkb_message_code = 523;
    pub const XKB_WARNING_EXTRA_SYMBOLS_IGNORED: xkb_message_code = 516;
    pub const XKB_WARNING_NUMERIC_KEYSYM: xkb_message_code = 489;
    pub const XKB_ERROR_INVALID_OPERATION: xkb_message_code = 478;
    pub const XKB_WARNING_CONFLICTING_KEY_SYMBOL: xkb_message_code = 461;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE_: xkb_message_code = 450;
    pub const XKB_WARNING_MISSING_DEFAULT_SECTION: xkb_message_code = 433;
    pub const XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE: xkb_message_code = 428;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS: xkb_message_code = 407;
    pub const XKB_ERROR_RECURSIVE_INCLUDE: xkb_message_code = 386;
    pub const XKB_WARNING_DUPLICATE_ENTRY: xkb_message_code = 378;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_: xkb_message_code = 371;
    pub const XKB_WARNING_UNSUPPORTED_LEGACY_ACTION: xkb_message_code = 362;
    pub const XKB_ERROR_OVERLAPPING_OVERLAY: xkb_message_code = 355;
    pub const XKB_ERROR_UNKNOWN_OPERATOR: xkb_message_code = 345;
    pub const XKB_ERROR_INCLUDED_FILE_NOT_FOUND: xkb_message_code = 338;
    pub const XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL: xkb_message_code = 312;
    pub const XKB_WARNING_NON_BASE_GROUP_NAME: xkb_message_code = 305;
    pub const XKB_WARNING_DEPRECATED_KEYSYM_NAME: xkb_message_code = 302;
    pub const XKB_WARNING_DEPRECATED_KEYSYM: xkb_message_code = 301;
    pub const XKB_WARNING_UNDEFINED_KEY_TYPE: xkb_message_code = 286;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY: xkb_message_code = 266;
    pub const XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT: xkb_message_code = 254;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES: xkb_message_code = 239;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_: xkb_message_code = 237;
    pub const XKB_ERROR_UNKNOWN_STATEMENT: xkb_message_code = 222;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_: xkb_message_code = 214;
    pub const XKB_ERROR_INVALID_MODMAP_ENTRY: xkb_message_code = 206;
    pub const XKB_ERROR_INVALID_INCLUDE_STATEMENT: xkb_message_code = 203;
    pub const XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT: xkb_message_code = 195;
    pub const XKB_WARNING_INVALID_ESCAPE_SEQUENCE: xkb_message_code = 193;
    pub const XKB_WARNING_CANNOT_INFER_KEY_TYPE: xkb_message_code = 183;
    pub const XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION: xkb_message_code = 172;
    pub const XKB_ERROR_INVALID_PATH: xkb_message_code = 161;
    pub const XKB_ERROR_WRONG_STATEMENT_TYPE: xkb_message_code = 150;
    pub const XKB_ERROR_INSUFFICIENT_BUFFER_SIZE: xkb_message_code = 134;
    pub const XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER: xkb_message_code = 123;
    pub const XKB_WARNING_UNRECOGNIZED_KEYSYM: xkb_message_code = 107;
    pub const XKB_WARNING_ILLEGAL_KEYCODE_ALIAS: xkb_message_code = 101;
    pub const XKB_ERROR_INVALID_NUMERIC_KEYSYM: xkb_message_code = 82;
    pub const XKB_ERROR_EXPECTED_ARRAY_ENTRY: xkb_message_code = 77;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_: xkb_message_code = 60;
    pub const XKB_ERROR_INTEGER_OVERFLOW: xkb_message_code = 52;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES: xkb_message_code = 43;
    pub const XKB_ERROR_MALFORMED_NUMBER_LITERAL: xkb_message_code = 34;
    pub const _XKB_LOG_MESSAGE_MIN_CODE: xkb_message_code = 34;
}
pub mod context_h {
    pub use crate::xkb::context_priv::{
        xkb_context_failed_include_path_get, xkb_context_getenv,
        xkb_context_num_failed_include_paths,
    };
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_context, xkb_log_level, xkb_rule_names, C2Rust_Unnamed,
        C2Rust_Unnamed_0,
    };
    extern "C" {
        pub fn xkb_context_include_path_get_extra_path(ctx: *mut xkb_context) -> *const i8;
        pub fn xkb_context_include_path_get_system_path(ctx: *mut xkb_context) -> *const i8;
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const i8,
            ...
        );
    }
}
pub mod atom_h {
    pub use crate::xkb::shared_types::atom_table;
}
pub mod darray_h {
    pub use crate::xkb::shared_types::darray_size_t;
}
pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::{
        xkb_context, xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
        XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    };
    extern "C" {
        pub fn xkb_context_num_include_paths(context: *mut xkb_context) -> u32;
        pub fn xkb_context_include_path_get(context: *mut xkb_context, index: u32) -> *const i8;
    }
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    extern "C" {
        pub fn xkb_file_type_to_string(type_0: xkb_file_type) -> *const i8;
    }
}
pub mod scanner_utils_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct scanner_loc {
        pub line: usize,
        pub column: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct scanner {
        pub pos: usize,
        pub len: usize,
        pub s: *const i8,
        pub buf: [i8; 1024],
        pub buf_pos: usize,
        pub token_pos: usize,
        pub cached_pos: usize,
        pub cached_loc: scanner_loc,
        pub file_name: *const i8,
        pub ctx: *mut xkb_context,
        pub priv_0: *mut ::core::ffi::c_void,
    }
    #[inline]
    pub unsafe fn scanner_init(
        mut s: *mut scanner,
        mut ctx: *mut xkb_context,
        mut string: *const i8,
        mut len: usize,
        mut file_name: *const i8,
        mut priv_0: *mut ::core::ffi::c_void,
    ) {
        unsafe {
            (*s).s = string;
            (*s).len = len;
            (*s).pos = 0 as usize;
            (*s).token_pos = 0 as usize;
            (*s).cached_pos = 0 as usize;
            (*s).cached_loc.column = 1 as usize;
            (*s).cached_loc.line = (*s).cached_loc.column;
            (*s).file_name = file_name;
            (*s).ctx = ctx;
            (*s).priv_0 = priv_0;
        }
    }
    #[inline]
    pub unsafe fn scanner_peek(mut s: *mut scanner) -> i8 {
        unsafe {
            if ((*s).pos >= (*s).len) as ::core::ffi::c_int as i64 != 0 {
                return '\0' as i32 as i8;
            }
            return *(*s).s.offset((*s).pos as isize);
        }
    }
    #[inline]
    pub unsafe fn scanner_eof(mut s: *mut scanner) -> bool {
        unsafe {
            return (*s).pos >= (*s).len;
        }
    }
    #[inline]
    pub unsafe fn scanner_eol(mut s: *mut scanner) -> bool {
        unsafe {
            return scanner_peek(s) as ::core::ffi::c_int == '\n' as i32;
        }
    }
    #[inline]
    pub unsafe fn scanner_next(mut s: *mut scanner) -> i8 {
        unsafe {
            if scanner_eof(s) as ::core::ffi::c_int as i64 != 0 {
                return '\0' as i32 as i8;
            }
            let c2rust_fresh0 = (*s).pos;
            (*s).pos = (*s).pos.wrapping_add(1);
            return *(*s).s.offset(c2rust_fresh0 as isize);
        }
    }
    #[inline]
    pub unsafe fn scanner_chr(mut s: *mut scanner, mut ch: i8) -> bool {
        unsafe {
            if (scanner_peek(s) as ::core::ffi::c_int != ch as ::core::ffi::c_int)
                as ::core::ffi::c_int as i64
                != 0
            {
                return false_0 != 0;
            }
            (*s).pos = (*s).pos.wrapping_add(1);
            return true_0 != 0;
        }
    }
    #[inline]
    pub unsafe fn scanner_buf_append(mut s: *mut scanner, mut ch: i8) -> bool {
        unsafe {
            if (*s).buf_pos.wrapping_add(1 as usize)
                >= ::core::mem::size_of::<[i8; 1024]>() as usize
            {
                return false_0 != 0;
            }
            let c2rust_fresh1 = (*s).buf_pos;
            (*s).buf_pos = (*s).buf_pos.wrapping_add(1);
            (*s).buf[c2rust_fresh1 as usize] = ch;
            return true_0 != 0;
        }
    }
    #[inline]
    pub unsafe fn scanner_buf_appends(mut s: *mut scanner, mut str: *const i8) -> bool {
        unsafe {
            let mut ret: ::core::ffi::c_int = 0;
            ret = snprintf(
                (&raw mut (*s).buf as *mut i8).offset((*s).buf_pos as isize),
                (::core::mem::size_of::<[i8; 1024]>() as usize).wrapping_sub((*s).buf_pos),
                b"%s\0".as_ptr() as *const i8,
                str,
            );
            if ret < 0 as ::core::ffi::c_int
                || ret as usize
                    >= (::core::mem::size_of::<[i8; 1024]>() as usize)
                        .wrapping_sub((*s).buf_pos as usize)
            {
                return false_0 != 0;
            }
            (*s).buf_pos = (*s).buf_pos.wrapping_add(ret as usize);
            return true_0 != 0;
        }
    }

    use super::context_h::xkb_context;
    use super::stdbool_h::{false_0, true_0};
    use super::stdio_h::snprintf;
    extern "C" {
        pub fn scanner_token_location(s: *mut scanner) -> scanner_loc;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strdup(__s: *const i8) -> *mut i8;
        pub fn strchr(__s: *const i8, __c: ::core::ffi::c_int) -> *mut i8;
        pub fn strpbrk(__s: *const i8, __accept: *const i8) -> *mut i8;
        pub fn strlen(__s: *const i8) -> usize;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn snprintf_safe(
        mut buf: *mut i8,
        mut sz: usize,
        mut format: *const i8,
        mut c2rust_args: ...
    ) -> bool {
        unsafe {
            let mut ap: ::core::ffi::VaList;
            let mut rc: ::core::ffi::c_int = 0;
            ap = c2rust_args.clone();
            rc = vsnprintf(buf, sz, format, ap);
            return rc >= 0 as ::core::ffi::c_int && (rc as usize) < sz;
        }
    }

    use super::stdio_h::vsnprintf;
}
pub mod xkbcomp_priv_h {
    use super::ast_h::XkbFile;
    use super::context_h::xkb_context;
    use super::FILE_h::FILE;

    pub unsafe fn XkbParseFile(
        ctx: *mut xkb_context,
        file: *mut FILE,
        file_name: *const i8,
        map: *const i8,
    ) -> *mut XkbFile {
        unsafe {
            crate::xkb::xkbcomp::scanner::XkbParseFile(ctx, file as *mut _, file_name, map)
                as *mut XkbFile
        }
    }

    extern "C" {
        pub fn FreeXkbFile(file: *mut XkbFile);
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdlib_h {
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod include_h {
    pub const INCLUDE_MAX_DEPTH: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    pub const MERGE_OVERRIDE_PREFIX: ::core::ffi::c_int = '+' as i32;
    pub const MERGE_AUGMENT_PREFIX: ::core::ffi::c_int = '|' as i32;
    pub const MERGE_REPLACE_PREFIX: ::core::ffi::c_int = '^' as i32;
    pub static mut MERGE_MODE_PREFIXES: [i8; 4] = [
        MERGE_OVERRIDE_PREFIX as i8,
        MERGE_AUGMENT_PREFIX as i8,
        MERGE_REPLACE_PREFIX as i8,
        0 as ::core::ffi::c_int as i8,
    ];
}
pub mod utils_paths_h {
    pub use crate::xkb::utils_paths::is_absolute_path;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::NULL;

pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, xkb_file_type, xkb_file_type_to_string,
    xkb_map_flags, IncludeStmt, ParseCommon, XkbFile, _FILE_TYPE_NUM_ENTRIES,
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
pub use self::context_h::{
    xkb_context, xkb_context_failed_include_path_get, xkb_context_getenv,
    xkb_context_include_path_get_extra_path, xkb_context_include_path_get_system_path,
    xkb_context_num_failed_include_paths, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
pub use self::include_h::{
    INCLUDE_MAX_DEPTH, MERGE_AUGMENT_PREFIX, MERGE_MODE_PREFIXES, MERGE_OVERRIDE_PREFIX,
    MERGE_REPLACE_PREFIX,
};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::messages_codes_h::{
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
pub use self::scanner_utils_h::{
    scanner, scanner_buf_append, scanner_buf_appends, scanner_chr, scanner_eof, scanner_eol,
    scanner_init, scanner_loc, scanner_next, scanner_peek, scanner_token_location,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdio_h::{fclose, fopen, snprintf, ssize_t, va_list, vsnprintf};
use self::stdlib_h::free;
use self::string_h::{memcpy, strchr, strdup, strlen, strpbrk};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __ssize_t, __uint64_t};
pub use self::utils_h::snprintf_safe;
use self::utils_paths_h::is_absolute_path;
pub use self::xkbcommon_h::{
    xkb_context_include_path_get, xkb_context_num_include_paths, xkb_log_level, xkb_rule_names,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING,
};
use self::xkbcomp_priv_h::{FreeXkbFile, XkbParseFile};
pub use self::FILE_h::FILE;
pub unsafe fn ParseIncludeMap(
    mut str_inout: *mut *mut i8,
    mut file_rtrn: *mut *mut i8,
    mut map_rtrn: *mut *mut i8,
    mut nextop_rtrn: *mut i8,
    mut extra_data: *mut *mut i8,
) -> bool {
    unsafe {
        let mut tmp: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut str: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut next: *mut i8 = ::core::ptr::null_mut::<i8>();
        str = *str_inout;
        next = strpbrk(str, &raw const MERGE_MODE_PREFIXES as *const i8);
        if !next.is_null() {
            *nextop_rtrn = *next;
            let c2rust_fresh2 = next;
            next = next.offset(1);
            *c2rust_fresh2 = '\0' as i32 as i8;
        } else {
            *nextop_rtrn = '\0' as i32 as i8;
            next = ::core::ptr::null_mut::<i8>();
        }
        tmp = strchr(str, ':' as i32);
        if !tmp.is_null() {
            let c2rust_fresh3 = tmp;
            tmp = tmp.offset(1);
            *c2rust_fresh3 = '\0' as i32 as i8;
            *extra_data = strdup(tmp);
        } else {
            *extra_data = ::core::ptr::null_mut::<i8>();
        }
        tmp = strchr(str, '(' as i32);
        if tmp.is_null() {
            *file_rtrn = strdup(str);
            *map_rtrn = ::core::ptr::null_mut::<i8>();
        } else if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '(' as i32
        {
            free(*extra_data as *mut ::core::ffi::c_void);
            return false_0 != 0;
        } else {
            let c2rust_fresh4 = tmp;
            tmp = tmp.offset(1);
            *c2rust_fresh4 = '\0' as i32 as i8;
            *file_rtrn = strdup(str);
            str = tmp;
            tmp = strchr(str, ')' as i32);
            if tmp.is_null()
                || *tmp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\0' as i32
            {
                free(*file_rtrn as *mut ::core::ffi::c_void);
                free(*extra_data as *mut ::core::ffi::c_void);
                return false_0 != 0;
            }
            let c2rust_fresh5 = tmp;
            tmp = tmp.offset(1);
            *c2rust_fresh5 = '\0' as i32 as i8;
            *map_rtrn = strdup(str);
        }
        if *nextop_rtrn as ::core::ffi::c_int == '\0' as i32 {
            *str_inout = ::core::ptr::null_mut::<i8>();
        } else if *nextop_rtrn as ::core::ffi::c_int == MERGE_OVERRIDE_PREFIX
            || *nextop_rtrn as ::core::ffi::c_int == MERGE_AUGMENT_PREFIX
            || *nextop_rtrn as ::core::ffi::c_int == MERGE_REPLACE_PREFIX
        {
            *str_inout = next;
        } else {
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
static mut xkb_file_type_include_dirs: [*const i8; 7] = [
    b"keycodes\0".as_ptr() as *const i8,
    b"types\0".as_ptr() as *const i8,
    b"compat\0".as_ptr() as *const i8,
    b"symbols\0".as_ptr() as *const i8,
    b"geometry\0".as_ptr() as *const i8,
    b"keymap\0".as_ptr() as *const i8,
    b"rules\0".as_ptr() as *const i8,
];
unsafe fn DirectoryForInclude(mut type_0: xkb_file_type) -> *const i8 {
    unsafe {
        if type_0 as u32 >= _FILE_TYPE_NUM_ENTRIES as ::core::ffi::c_int as u32 {
            return b"\0".as_ptr() as *const i8;
        }
        return xkb_file_type_include_dirs[type_0 as usize];
    }
}
unsafe fn LogIncludePaths(mut ctx: *mut xkb_context) {
    unsafe {
        if xkb_context_num_include_paths(ctx) > 0 as u32 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %u include paths searched:\n\0".as_ptr() as *const i8,
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
                xkb_context_num_include_paths(ctx),
            );
            let mut i: u32 = 0 as u32;
            while i < xkb_context_num_include_paths(ctx) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] \t%s\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
                    xkb_context_include_path_get(ctx, i),
                );
                i = i.wrapping_add(1);
            }
        } else {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] There are no include paths to search\n\0".as_ptr() as *const i8,
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
            );
        }
        if xkb_context_num_failed_include_paths(ctx) > 0 as darray_size_t {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %u include paths could not be added:\n\0".as_ptr() as *const i8,
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
                xkb_context_num_failed_include_paths(ctx),
            );
            let mut i_0: darray_size_t = 0 as darray_size_t;
            while i_0 < xkb_context_num_failed_include_paths(ctx) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] \t%s\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
                    xkb_context_failed_include_path_get(ctx, i_0),
                );
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
}
unsafe fn expand_percent(
    mut ctx: *mut xkb_context,
    mut parent_file_name: *const i8,
    mut typeDir: *const i8,
    mut buf: *mut i8,
    mut buf_size: usize,
    mut name: *const i8,
    mut name_len: usize,
) -> usize {
    unsafe {
        let mut s: scanner = scanner {
            pos: 0,
            len: 0,
            s: ::core::ptr::null::<i8>(),
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: scanner_loc { line: 0, column: 0 },
            file_name: ::core::ptr::null::<i8>(),
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            priv_0: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        scanner_init(&raw mut s, ctx, name, name_len, parent_file_name, NULL);
        s.buf_pos = 0 as usize;
        while !scanner_eof(&raw mut s) && !scanner_eol(&raw mut s) {
            if scanner_chr(&raw mut s, '%' as i32 as i8) {
                if scanner_chr(&raw mut s, '%' as i32 as i8) {
                    scanner_buf_append(&raw mut s, '%' as i32 as i8);
                } else if scanner_chr(&raw mut s, 'H' as i32 as i8) {
                    let mut home: *const i8 =
                        xkb_context_getenv(ctx, b"HOME\0".as_ptr() as *const i8);
                    if home.is_null() {
                        let mut loc: scanner_loc = scanner_token_location(&raw mut s);
                        xkb_log(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"%s:%zu:%zu: %%H was used in an include statement, but the HOME environment variable is not set\n\0"
                                .as_ptr() as *const i8,
                            s.file_name,
                            loc.line,
                            loc.column,
                        );
                        return 0 as usize;
                    }
                    if !scanner_buf_appends(&raw mut s, home) {
                        let mut loc_0: scanner_loc = scanner_token_location(&raw mut s);
                        xkb_log(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s:%zu:%zu: include path after expanding %%H is too long\n\0"
                                .as_ptr() as *const i8,
                            XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
                            s.file_name,
                            loc_0.line,
                            loc_0.column,
                        );
                        return 0 as usize;
                    }
                } else if scanner_chr(&raw mut s, 'S' as i32 as i8) {
                    let mut default_root: *const i8 = xkb_context_include_path_get_system_path(ctx);
                    if !scanner_buf_appends(&raw mut s, default_root)
                        || !scanner_buf_append(&raw mut s, '/' as i32 as i8)
                        || !scanner_buf_appends(&raw mut s, typeDir)
                    {
                        let mut loc_1: scanner_loc = scanner_token_location(&raw mut s);
                        xkb_log(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s:%zu:%zu: include path after expanding %%S is too long\n\0"
                                .as_ptr() as *const i8,
                            XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
                            s.file_name,
                            loc_1.line,
                            loc_1.column,
                        );
                        return 0 as usize;
                    }
                } else if scanner_chr(&raw mut s, 'E' as i32 as i8) {
                    let mut default_root_0: *const i8 =
                        xkb_context_include_path_get_extra_path(ctx);
                    if !scanner_buf_appends(&raw mut s, default_root_0)
                        || !scanner_buf_append(&raw mut s, '/' as i32 as i8)
                        || !scanner_buf_appends(&raw mut s, typeDir)
                    {
                        let mut loc_2: scanner_loc = scanner_token_location(&raw mut s);
                        xkb_log(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s:%zu:%zu: include path after expanding %%E is too long\n\0"
                                .as_ptr() as *const i8,
                            XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
                            s.file_name,
                            loc_2.line,
                            loc_2.column,
                        );
                        return 0 as usize;
                    }
                } else {
                    let mut loc_3: scanner_loc = scanner_token_location(&raw mut s);
                    xkb_log(
                        s.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] %s:%zu:%zu: unknown %% format (%c) in include statement\n\0"
                            .as_ptr() as *const i8,
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
                        s.file_name,
                        loc_3.line,
                        loc_3.column,
                        scanner_peek(&raw mut s) as ::core::ffi::c_int,
                    );
                    return 0 as usize;
                }
            } else {
                scanner_buf_append(&raw mut s, scanner_next(&raw mut s));
            }
        }
        if !scanner_buf_append(&raw mut s, '\0' as i32 as i8) {
            let mut loc_4: scanner_loc = scanner_token_location(&raw mut s);
            xkb_log(
                s.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: include path is too long; max: %zu\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
                s.file_name,
                loc_4.line,
                loc_4.column,
                ::core::mem::size_of::<[i8; 1024]>(),
            );
            return 0 as usize;
        }
        if (s.buf_pos > buf_size) as ::core::ffi::c_int as i64 != 0 {
            let mut loc_5: scanner_loc = scanner_token_location(&raw mut s);
            xkb_log(
                s.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: include path is too long: %zu > %zu\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
                s.file_name,
                loc_5.line,
                loc_5.column,
                s.buf_pos,
                buf_size,
            );
            return 0 as usize;
        }
        memcpy(
            buf as *mut ::core::ffi::c_void,
            &raw mut s.buf as *mut i8 as *const ::core::ffi::c_void,
            s.buf_pos,
        );
        return s.buf_pos;
    }
}
pub unsafe fn expand_path(
    mut ctx: *mut xkb_context,
    mut parent_file_name: *const i8,
    mut name: *const i8,
    mut name_len: usize,
    mut type_0: xkb_file_type,
    mut buf: *mut i8,
    mut buf_size: usize,
) -> ssize_t {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut k: usize = 0;
        k = 0 as usize;
        loop {
            if !(k < name_len) {
                c2rust_current_block = 17179679302217393232;
                break;
            }
            if *name.offset(k as isize) as ::core::ffi::c_int == '%' as i32 {
                c2rust_current_block = 15593259132448327734;
                break;
            }
            k = k.wrapping_add(1);
        }
        match c2rust_current_block {
            17179679302217393232 => return 0 as ssize_t,
            _ => {
                if (k >= buf_size) as ::core::ffi::c_int as i64 != 0 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Path is too long: %zu > %zu, got raw path: %.*s\n\0".as_ptr()
                            as *const i8,
                        XKB_ERROR_INVALID_PATH as ::core::ffi::c_int,
                        k,
                        buf_size,
                        name_len as u32,
                        name,
                    );
                    return -1 as ::core::ffi::c_int as ssize_t;
                }
                memcpy(
                    buf as *mut ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    k,
                );
                let mut typeDir: *const i8 = DirectoryForInclude(type_0);
                let mut count: usize = expand_percent(
                    ctx,
                    parent_file_name,
                    typeDir,
                    buf.offset(k as isize),
                    buf_size.wrapping_sub(k),
                    name.offset(k as isize),
                    name_len.wrapping_sub(k),
                );
                if count == 0 {
                    return -1 as ::core::ffi::c_int as ssize_t;
                }
                count = count.wrapping_add(k);
                return count as ssize_t - 1 as ssize_t;
            }
        };
    }
}
pub unsafe fn FindFileInXkbPath(
    mut ctx: *mut xkb_context,
    mut parent_file_name: *const i8,
    mut name: *const i8,
    mut name_len: usize,
    mut type_0: xkb_file_type,
    mut buf: *mut i8,
    mut buf_size: usize,
    mut offset: *mut u32,
    mut required: bool,
) -> *mut FILE {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut name_buffer: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut typeDir: *const i8 = DirectoryForInclude(type_0);
        let mut i: u32 = *offset;
        loop {
            if !(i < xkb_context_num_include_paths(ctx)) {
                c2rust_current_block = 8515828400728868193;
                break;
            }
            if !snprintf_safe(
                buf,
                buf_size,
                b"%s/%s/%.*s\0".as_ptr() as *const i8,
                xkb_context_include_path_get(ctx, i),
                typeDir,
                name_len as u32,
                name,
            ) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Path is too long: expected max length of %zu, got: %s/%s/%.*s\n\0"
                        .as_ptr() as *const i8,
                    XKB_ERROR_INVALID_PATH as ::core::ffi::c_int,
                    buf_size,
                    xkb_context_include_path_get(ctx, i),
                    typeDir,
                    name_len as u32,
                    name,
                );
            } else {
                file = fopen(buf, b"rb\0".as_ptr() as *const i8) as *mut FILE;
                if !file.is_null() {
                    *offset = i;
                    c2rust_current_block = 17619028831370153636;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        match c2rust_current_block {
            8515828400728868193 => {
                if required as ::core::ffi::c_int != 0 && *offset == 0 as u32 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Couldn't find file \"%s/%.*s\" in include paths\n\0".as_ptr()
                            as *const i8,
                        XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
                        typeDir,
                        name_len as u32,
                        name,
                    );
                    LogIncludePaths(ctx);
                }
            }
            _ => {}
        }
        free(name_buffer as *mut ::core::ffi::c_void);
        return file;
    }
}
pub unsafe fn ExceedsIncludeMaxDepth(mut ctx: *mut xkb_context, mut include_depth: u32) -> bool {
    unsafe {
        if include_depth >= INCLUDE_MAX_DEPTH as u32 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Exceeded include depth threshold (%u)\0".as_ptr() as *const i8,
                XKB_ERROR_RECURSIVE_INCLUDE as ::core::ffi::c_int,
                15 as ::core::ffi::c_int,
            );
            return true_0 != 0;
        } else {
            return false_0 != 0;
        };
    }
}
pub unsafe fn ProcessIncludeFile(
    mut ctx: *mut xkb_context,
    mut stmt: *const IncludeStmt,
    mut file_type: xkb_file_type,
    mut path: *mut i8,
    mut path_size: usize,
) -> *mut XkbFile {
    unsafe {
        let mut xkb_file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        let mut candidate: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        let mut stmt_file: *const i8 = (*stmt).file;
        let mut stmt_file_len: usize = strlen(stmt_file);
        let expanded: ssize_t = expand_path(
            ctx,
            b"(unknown)\0".as_ptr() as *const i8,
            stmt_file,
            stmt_file_len,
            file_type,
            path,
            path_size,
        ) as ssize_t;
        if expanded < 0 as ssize_t {
            return ::core::ptr::null_mut::<XkbFile>();
        } else if expanded > 0 as ssize_t {
            stmt_file = path;
            stmt_file_len = expanded as usize;
        }
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut offset: u32 = 0 as u32;
        let absolute_path: bool = is_absolute_path(stmt_file) as bool;
        if absolute_path {
            file = fopen(stmt_file, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        } else if (expanded != 0) as ::core::ffi::c_int as i64 != 0 {
            file = ::core::ptr::null_mut::<FILE>();
        } else {
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                stmt_file,
                stmt_file_len,
                file_type,
                path,
                path_size,
                &raw mut offset,
                true_0 != 0,
            );
        }
        while !file.is_null() {
            xkb_file = XkbParseFile(ctx, file, (*stmt).file, (*stmt).map);
            fclose(file);
            if !xkb_file.is_null() {
                if (*xkb_file).file_type as u32 != file_type as u32 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Include file of wrong type (expected %s, got %s); Include file \"%s\" ignored\n\0"
                            .as_ptr() as *const i8,
                        XKB_ERROR_INVALID_INCLUDED_FILE as ::core::ffi::c_int,
                        xkb_file_type_to_string(file_type),
                        xkb_file_type_to_string((*xkb_file).file_type),
                        (*stmt).file,
                    );
                    FreeXkbFile(xkb_file);
                    xkb_file = ::core::ptr::null_mut::<XkbFile>();
                } else if !(*stmt).map.is_null()
                    || (*xkb_file).flags as u32 != 0 && MAP_IS_DEFAULT as ::core::ffi::c_int != 0
                {
                    break;
                } else if candidate.is_null() {
                    candidate = xkb_file;
                    xkb_file = ::core::ptr::null_mut::<XkbFile>();
                } else {
                    FreeXkbFile(xkb_file);
                    xkb_file = ::core::ptr::null_mut::<XkbFile>();
                }
            }
            if absolute_path {
                break;
            }
            offset = offset.wrapping_add(1);
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                stmt_file,
                stmt_file_len,
                file_type,
                path,
                path_size,
                &raw mut offset,
                true_0 != 0,
            );
        }
        if xkb_file.is_null() {
            xkb_file = candidate;
        } else {
            FreeXkbFile(candidate);
        }
        if xkb_file.is_null() {
            if !(*stmt).map.is_null() {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Couldn't process include statement for '%s(%s)'\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_INCLUDED_FILE as ::core::ffi::c_int,
                    (*stmt).file,
                    (*stmt).map,
                );
            } else {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Couldn't process include statement for '%s'\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_INCLUDED_FILE as ::core::ffi::c_int,
                    (*stmt).file,
                );
            }
        }
        return xkb_file;
    }
}
