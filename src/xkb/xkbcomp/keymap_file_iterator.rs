use crate::xkb_logf;
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod types_h {
    pub type __uint64_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
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
        pub _cur_column: u16,
        pub _vtable_offset: :i8,
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
pub mod context_h {
    pub use crate::xkb::shared_types::*;
}
pub mod atom_h {
    pub use crate::xkb::shared_types::*;
}
pub mod darray_h {
    pub use crate::xkb::shared_types::*;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_char {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut i8,
    }
    pub unsafe fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }
}
pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::*;
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

    use super::context_h::xkb_context;
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    pub use crate::xkb::xkbcomp::ast_build::xkb_file_type_to_string;
}
pub mod keymap_file_iterator_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_file_include {
        #[bitfield(name = "valid", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "explicit_section", ty = "bool", bits = "1..=1")]
        pub valid_explicit_section: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
        pub merge: merge_mode,
        pub path: darray_size_t,
        pub file: darray_size_t,
        pub section: darray_size_t,
        pub modifier: darray_size_t,
        pub flags: xkb_map_flags,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_file_include_group {
        pub start: darray_size_t,
        pub end: darray_size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_file_section {
        pub name: darray_size_t,
        pub file_type: xkb_file_type,
        pub flags: xkb_map_flags,
        pub include_groups: C2Rust_Unnamed_3,
        pub includes: C2Rust_Unnamed_2,
        pub buffer: darray_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_2 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_file_include,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_3 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_file_include_group,
    }
    pub type xkb_file_iterator_flags = u32;
    pub const XKB_FILE_ITERATOR_NO_INCLUDES: xkb_file_iterator_flags = 2;
    pub const XKB_FILE_ITERATOR_FAIL_ON_INCLUDE_ERROR: xkb_file_iterator_flags = 1;
    pub const XKB_FILE_ITERATOR_NO_FLAG: xkb_file_iterator_flags = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_file_iterator {
        pub flags: xkb_file_iterator_flags,
        pub finished: bool,
        pub path: *const i8,
        pub map: *const i8,
        pub type_0: xkb_file_type,
        pub scanner: scanner,
        pub section: xkb_file_section,
        pub pending_xkb_file: *mut XkbFile,
        pub pending_section: *mut XkbFile,
        pub ctx: *mut xkb_context,
    }
    use super::ast_h::{merge_mode, xkb_file_type, xkb_map_flags, XkbFile};
    use super::context_h::xkb_context;
    use super::darray_h::{darray_char, darray_size_t};
    use super::scanner_utils_h::scanner;
}
pub mod string_h {}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        pub fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn strcpy_safe(mut dest: *mut i8, mut size: usize, mut src: *const i8) -> *mut i8 {
        unsafe {
            if dest.is_null() || size == 0 || src.is_null() {
                return ::core::ptr::null_mut::<i8>();
            }
            let limit: *const i8 = dest
                .offset(size as isize)
                .offset(-(1 as ::core::ffi::c_int as isize));
            while dest < limit as *mut i8 && *src as ::core::ffi::c_int != 0 {
                let c2rust_fresh0 = src;
                src = src.offset(1);
                let c2rust_fresh1 = dest;
                dest = dest.offset(1);
                *c2rust_fresh1 = *c2rust_fresh0;
            }
            *dest = '\0' as i32 as i8;
            return if *src as ::core::ffi::c_int != 0 {
                ::core::ptr::null_mut::<i8>()
            } else {
                dest
            };
        }
    }
}
pub mod include_h {
    pub use crate::xkb::xkbcomp::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};

    use super::ast_h::xkb_file_type;
    use super::context_h::xkb_context;
    use super::FILE_h::FILE;

    pub unsafe fn FindFileInXkbPath(
        ctx: *mut xkb_context,
        parent_file_name: *const i8,
        name: *const i8,
        name_len: usize,
        type_0: xkb_file_type,
        buf: *mut i8,
        buf_size: usize,
        offset: *mut u32,
        required: bool,
    ) -> *mut FILE {
        unsafe {
            crate::xkb::xkbcomp::include::FindFileInXkbPath(
                ctx,
                parent_file_name,
                name,
                name_len,
                type_0,
                buf,
                buf_size,
                offset,
                required,
            ) as *mut FILE
        }
    }
}
pub mod xkbcomp_priv_h {

    use super::ast_h::XkbFile;
    use super::context_h::xkb_context;
    use super::scanner_utils_h::scanner;
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

    pub unsafe fn XkbParseStringInit(
        ctx: *mut xkb_context,
        scanner: *mut scanner,
        string: *const i8,
        len: usize,
        file_name: *const i8,
        map: *const i8,
    ) -> bool {
        unsafe {
            crate::xkb::xkbcomp::scanner::XkbParseStringInit(
                ctx,
                scanner as *mut _,
                string,
                len,
                file_name,
                map,
            )
        }
    }

    pub unsafe fn XkbParseStringNext(
        ctx: *mut xkb_context,
        scanner: *mut scanner,
        map: *const i8,
        out: *mut *mut XkbFile,
    ) -> bool {
        unsafe {
            crate::xkb::xkbcomp::scanner::XkbParseStringNext(
                ctx,
                scanner as *mut _,
                map,
                out as *mut *mut _,
            )
        }
    }

    pub use crate::xkb::xkbcomp::ast_build::FreeXkbFile;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod utils_paths_h {
    pub use crate::xkb::utils_paths::is_absolute_path;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
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
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_char, darray_next_alloc, darray_size_t};
use self::include_h::{ExceedsIncludeMaxDepth, FindFileInXkbPath, ProcessIncludeFile};
pub use self::internal::__va_list_tag;
pub use self::keymap_file_iterator_h::{
    xkb_file_include, xkb_file_include_group, xkb_file_iterator, xkb_file_iterator_flags,
    xkb_file_section, C2Rust_Unnamed_2, C2Rust_Unnamed_3, XKB_FILE_ITERATOR_FAIL_ON_INCLUDE_ERROR,
    XKB_FILE_ITERATOR_NO_FLAG, XKB_FILE_ITERATOR_NO_INCLUDES,
};
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
pub use self::scanner_utils_h::{scanner, scanner_loc};
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::{fclose, fopen};
use self::stdlib_h::{calloc, free, realloc};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::utils_h::strcpy_safe;
use self::utils_paths_h::is_absolute_path;
pub use self::xkbcommon_h::{
    xkb_keymap_compile_flags, xkb_keymap_format, xkb_log_level, xkb_rule_names,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
use self::xkbcomp_priv_h::{FreeXkbFile, XkbParseFile, XkbParseStringInit, XkbParseStringNext};
pub use self::FILE_h::FILE;
use crate::xkb::utils::cstr_len;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub flag: xkb_map_flags,
    pub name: *const i8,
}

pub unsafe fn xkb_file_type_name(mut type_0: xkb_file_type) -> *const i8 {
    unsafe {
        if type_0 as u32 > FILE_TYPE_KEYMAP as ::core::ffi::c_int as u32 {
            return b"unknown\0".as_ptr() as *const i8;
        }
        static mut xkb_file_type_strings: [*const i8; 7] = [
            b"keycodes\0".as_ptr() as *const i8,
            b"types\0".as_ptr() as *const i8,
            b"compatibility\0".as_ptr() as *const i8,
            b"symbols\0".as_ptr() as *const i8,
            b"geometry\0".as_ptr() as *const i8,
            b"keymap\0".as_ptr() as *const i8,
            ::core::ptr::null::<i8>(),
        ];
        return xkb_file_type_strings[type_0 as usize];
    }
}

pub unsafe fn xkb_merge_mode_name(mut merge: merge_mode) -> *const i8 {
    unsafe {
        if merge as u32 >= _MERGE_MODE_NUM_ENTRIES as ::core::ffi::c_int as u32 {
            return b"unknown\0".as_ptr() as *const i8;
        }
        static mut merge_mode_strings: [*const i8; 4] = [
            b"default\0".as_ptr() as *const i8,
            b"augment\0".as_ptr() as *const i8,
            b"override\0".as_ptr() as *const i8,
            b"replace\0".as_ptr() as *const i8,
        ];
        return merge_mode_strings[merge as usize];
    }
}

pub unsafe fn xkb_map_flags_string_iter(
    mut index: *mut u32,
    mut flags: xkb_map_flags,
) -> *const i8 {
    unsafe {
        if flags as u64 == 0 {
            return ::core::ptr::null::<i8>();
        }
        static mut names: [C2Rust_Unnamed_1; 8] = [
            C2Rust_Unnamed_1 {
                flag: MAP_IS_DEFAULT,
                name: b"default\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_IS_PARTIAL,
                name: b"partial\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_IS_HIDDEN,
                name: b"hidden\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_HAS_ALPHANUMERIC,
                name: b"alphanumeric\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_HAS_MODIFIER,
                name: b"modifiers\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_HAS_KEYPAD,
                name: b"keypad\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_HAS_FN,
                name: b"fn\0".as_ptr() as *const i8,
            },
            C2Rust_Unnamed_1 {
                flag: MAP_IS_ALTGR,
                name: b"altgr\0".as_ptr() as *const i8,
            },
        ];
        while (*index as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_1; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_1>() as usize)
        {
            if flags as u32 & names[*index as usize].flag as u32 != 0 {
                let c2rust_fresh2 = *index;
                *index = (*index).wrapping_add(1);
                return names[c2rust_fresh2 as usize].name;
            }
            *index = (*index).wrapping_add(1);
        }
        return ::core::ptr::null::<i8>();
    }
}

pub unsafe fn xkb_resolve_file(
    mut ctx: *mut xkb_context,
    mut file_type: xkb_file_type,
    mut path: *const i8,
    mut map: *const i8,
    mut resolved_path: *mut i8,
    mut resolved_path_size: usize,
    mut resolved_map: *mut i8,
    mut resolved_map_size: usize,
) -> *mut FILE {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut offset: u32 = 0 as u32;
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut candidate: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let path_len: usize = cstr_len(path) as usize;
        let absolute_path: bool = is_absolute_path(path) as bool;
        if absolute_path {
            file = fopen(path, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        } else {
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                path,
                path_len,
                file_type,
                resolved_path,
                resolved_path_size,
                &raw mut offset,
                true_0 != 0,
            );
        }
        loop {
            if file.is_null() {
                c2rust_current_block = 16203760046146113240;
                break;
            }
            let xkb_file: *mut XkbFile = XkbParseFile(ctx, file, path, map) as *mut XkbFile;
            if !xkb_file.is_null() {
                if (file_type as u32) < _FILE_TYPE_NUM_ENTRIES as ::core::ffi::c_int as u32
                    && (*xkb_file).file_type as u32 != file_type as u32
                {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"File of wrong type (expected %s, got %s); file \"%s\" ignored\n\0"
                            .as_ptr() as *const i8,
                        xkb_file_type_to_string(file_type),
                        xkb_file_type_to_string((*xkb_file).file_type),
                        if absolute_path as ::core::ffi::c_int != 0 {
                            path
                        } else {
                            resolved_path as *const i8
                        },
                    );
                    c2rust_current_block = 6705605813258909411;
                } else if !map.is_null()
                    || (*xkb_file).flags as u32 != 0 && MAP_IS_DEFAULT as ::core::ffi::c_int != 0
                {
                    if strcpy_safe(
                        resolved_map,
                        resolved_map_size,
                        if !(*xkb_file).name.is_null() {
                            (*xkb_file).name as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        },
                    )
                    .is_null()
                    {
                        FreeXkbFile(xkb_file);
                        c2rust_current_block = 7064064774883755911;
                        break;
                    } else {
                        c2rust_current_block = 4495394744059808450;
                    }
                } else if candidate.is_null() {
                    candidate = file;
                    if strcpy_safe(
                        resolved_map,
                        resolved_map_size,
                        if !(*xkb_file).name.is_null() {
                            (*xkb_file).name as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        },
                    )
                    .is_null()
                    {
                        FreeXkbFile(xkb_file);
                        c2rust_current_block = 7064064774883755911;
                        break;
                    } else {
                        c2rust_current_block = 4495394744059808450;
                    }
                } else {
                    c2rust_current_block = 6705605813258909411;
                }
            } else {
                c2rust_current_block = 6705605813258909411;
            }
            match c2rust_current_block {
                6705605813258909411 => {
                    fclose(file);
                    file = ::core::ptr::null_mut::<FILE>();
                }
                _ => {}
            }
            FreeXkbFile(xkb_file);
            if !file.is_null() || absolute_path as ::core::ffi::c_int != 0 {
                c2rust_current_block = 16203760046146113240;
                break;
            }
            offset = offset.wrapping_add(1);
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                path,
                path_len,
                file_type,
                resolved_path,
                resolved_path_size,
                &raw mut offset,
                true_0 != 0,
            );
        }
        match c2rust_current_block {
            16203760046146113240 => {
                if file.is_null() {
                    file = candidate;
                } else if !candidate.is_null() {
                    fclose(candidate);
                }
                if !(absolute_path as ::core::ffi::c_int != 0
                    && !file.is_null()
                    && strcpy_safe(resolved_path, resolved_path_size, path).is_null())
                {
                    return file;
                }
            }
            _ => {}
        }
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"[XKB-%03d] Cannot copy resolved path or section\n\0".as_ptr() as *const i8,
            XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
        );
        fclose(file);
        return ::core::ptr::null_mut::<FILE>();
    }
}

pub unsafe fn xkb_file_section_init(mut section: *mut xkb_file_section) {
    unsafe {
        (*section).include_groups.item = ::core::ptr::null_mut::<xkb_file_include_group>();
        (*section).include_groups.size = 0 as darray_size_t;
        (*section).include_groups.alloc = 0 as darray_size_t;
        (*section).includes.item = ::core::ptr::null_mut::<xkb_file_include>();
        (*section).includes.size = 0 as darray_size_t;
        (*section).includes.alloc = 0 as darray_size_t;
        (*section).buffer.item = ::core::ptr::null_mut::<i8>();
        (*section).buffer.size = 0 as darray_size_t;
        (*section).buffer.alloc = 0 as darray_size_t;
        (*section).buffer.size = (*section).buffer.size.wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = (*section).buffer.size;
        if __need > (*section).buffer.alloc {
            (*section).buffer.alloc = darray_next_alloc(
                (*section).buffer.alloc,
                __need,
                ::core::mem::size_of::<i8>() as usize,
            );
            (*section).buffer.item = realloc(
                (*section).buffer.item as *mut ::core::ffi::c_void,
                ((*section).buffer.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<i8>() as usize),
            ) as *mut i8;
        }
        *(*section)
            .buffer
            .item
            .offset((*section).buffer.size.wrapping_sub(1 as darray_size_t) as isize) =
            '\0' as i32 as i8;
    }
}
unsafe fn xkb_file_section_reset(mut section: *mut xkb_file_section) {
    unsafe {
        (*section).include_groups.size = 0 as darray_size_t;
        (*section).includes.size = 0 as darray_size_t;
        (*section).buffer.size = 1 as darray_size_t;
    }
}

pub unsafe fn xkb_file_section_free(mut section: *mut xkb_file_section) {
    unsafe {
        if section.is_null() {
            return;
        }
        free((*section).include_groups.item as *mut ::core::ffi::c_void);
        (*section).include_groups.item = ::core::ptr::null_mut::<xkb_file_include_group>();
        (*section).include_groups.size = 0 as darray_size_t;
        (*section).include_groups.alloc = 0 as darray_size_t;
        free((*section).includes.item as *mut ::core::ffi::c_void);
        (*section).includes.item = ::core::ptr::null_mut::<xkb_file_include>();
        (*section).includes.size = 0 as darray_size_t;
        (*section).includes.alloc = 0 as darray_size_t;
        free((*section).buffer.item as *mut ::core::ffi::c_void);
        (*section).buffer.item = ::core::ptr::null_mut::<i8>();
        (*section).buffer.size = 0 as darray_size_t;
        (*section).buffer.alloc = 0 as darray_size_t;
    }
}
unsafe fn xkb_file_section_set_meta_data(
    mut ctx: *mut xkb_context,
    mut section: *mut xkb_file_section,
    mut xkb_file: *const XkbFile,
) -> bool {
    unsafe {
        (*section).file_type = (*xkb_file).file_type;
        (*section).flags = (*xkb_file).flags;
        if !(*xkb_file).name.is_null() {
            let mut idx: darray_size_t = (*section).buffer.size;
            let mut __count: darray_size_t =
                cstr_len((*xkb_file).name).wrapping_add(1 as usize) as darray_size_t;
            let mut __oldSize: darray_size_t = (*section).buffer.size;
            (*section).buffer.size = __oldSize.wrapping_add(__count);
            let mut __need: darray_size_t = (*section).buffer.size;
            if __need > (*section).buffer.alloc {
                (*section).buffer.alloc = darray_next_alloc(
                    (*section).buffer.alloc,
                    __need,
                    ::core::mem::size_of::<i8>() as usize,
                );
                (*section).buffer.item = realloc(
                    (*section).buffer.item as *mut ::core::ffi::c_void,
                    ((*section).buffer.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<i8>() as usize),
                ) as *mut i8;
            }
            std::ptr::copy_nonoverlapping(
                (*xkb_file).name as *const u8,
                (*section).buffer.item.offset(__oldSize as isize) as *mut u8,
                __count as usize,
            );
            (*section).name = idx;
        } else {
            (*section).name = 0 as darray_size_t;
        }
        return true_0 != 0;
    }
}
unsafe fn xkb_file_section_append_includes(
    mut ctx: *mut xkb_context,
    mut flags: xkb_file_iterator_flags,
    mut section_path: *const i8,
    mut section: *mut xkb_file_section,
    mut file_type: xkb_file_type,
    mut include: *mut IncludeStmt,
) -> bool {
    unsafe {
        let mut group: *mut xkb_file_include_group =
            ::core::ptr::null_mut::<xkb_file_include_group>();
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut buf: [i8; 4096] = [0; 4096];
            let mut xkb_file: *mut XkbFile = ProcessIncludeFile(
                ctx,
                stmt,
                file_type,
                &raw mut buf as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
            );
            let valid: bool = !xkb_file.is_null();
            if valid as ::core::ffi::c_int != 0
                || flags as u32
                    & XKB_FILE_ITERATOR_FAIL_ON_INCLUDE_ERROR as ::core::ffi::c_int as u32
                    == 0
            {
                let path: darray_size_t = (*section).buffer.size;
                let mut __count: darray_size_t =
                    cstr_len(&raw mut buf as *mut i8).wrapping_add(1 as usize) as darray_size_t;
                let mut __oldSize: darray_size_t = (*section).buffer.size;
                (*section).buffer.size = __oldSize.wrapping_add(__count);
                let mut __need: darray_size_t = (*section).buffer.size;
                if __need > (*section).buffer.alloc {
                    (*section).buffer.alloc = darray_next_alloc(
                        (*section).buffer.alloc,
                        __need,
                        ::core::mem::size_of::<i8>() as usize,
                    );
                    (*section).buffer.item = realloc(
                        (*section).buffer.item as *mut ::core::ffi::c_void,
                        ((*section).buffer.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<i8>() as usize),
                    ) as *mut i8;
                }
                std::ptr::copy_nonoverlapping(
                    &raw mut buf as *const u8,
                    (*section).buffer.item.offset(__oldSize as isize) as *mut u8,
                    __count as usize,
                );
                let file: darray_size_t = (*section).buffer.size;
                let mut __count_0: darray_size_t =
                    cstr_len((*stmt).file).wrapping_add(1 as usize) as darray_size_t;
                let mut __oldSize_0: darray_size_t = (*section).buffer.size;
                (*section).buffer.size = __oldSize_0.wrapping_add(__count_0);
                let mut __need_0: darray_size_t = (*section).buffer.size;
                if __need_0 > (*section).buffer.alloc {
                    (*section).buffer.alloc = darray_next_alloc(
                        (*section).buffer.alloc,
                        __need_0,
                        ::core::mem::size_of::<i8>() as usize,
                    );
                    (*section).buffer.item = realloc(
                        (*section).buffer.item as *mut ::core::ffi::c_void,
                        ((*section).buffer.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<i8>() as usize),
                    ) as *mut i8;
                }
                std::ptr::copy_nonoverlapping(
                    (*stmt).file as *const u8,
                    (*section).buffer.item.offset(__oldSize_0 as isize) as *mut u8,
                    __count_0 as usize,
                );
                let section_name: darray_size_t = if !(*stmt).map.is_null()
                    || valid as ::core::ffi::c_int != 0 && !(*xkb_file).name.is_null()
                {
                    (*section).buffer.size
                } else {
                    0 as darray_size_t
                };
                if section_name != 0 {
                    let mut __count_1: darray_size_t = cstr_len(if !(*stmt).map.is_null() {
                        (*stmt).map
                    } else {
                        (*xkb_file).name
                    })
                    .wrapping_add(1 as usize)
                        as darray_size_t;
                    let mut __oldSize_1: darray_size_t = (*section).buffer.size;
                    (*section).buffer.size = __oldSize_1.wrapping_add(__count_1);
                    let mut __need_1: darray_size_t = (*section).buffer.size;
                    if __need_1 > (*section).buffer.alloc {
                        (*section).buffer.alloc = darray_next_alloc(
                            (*section).buffer.alloc,
                            __need_1,
                            ::core::mem::size_of::<i8>() as usize,
                        );
                        (*section).buffer.item = realloc(
                            (*section).buffer.item as *mut ::core::ffi::c_void,
                            ((*section).buffer.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<i8>() as usize),
                        ) as *mut i8;
                    }
                    std::ptr::copy_nonoverlapping(
                        (if !(*stmt).map.is_null() {
                            (*stmt).map
                        } else {
                            (*xkb_file).name
                        }) as *const u8,
                        (*section).buffer.item.offset(__oldSize_1 as isize) as *mut u8,
                        __count_1 as usize,
                    );
                }
                let modifier: darray_size_t = if !(*stmt).modifier.is_null() {
                    (*section).buffer.size
                } else {
                    0 as darray_size_t
                };
                if modifier != 0 {
                    let mut __count_2: darray_size_t =
                        cstr_len((*stmt).modifier).wrapping_add(1 as usize) as darray_size_t;
                    let mut __oldSize_2: darray_size_t = (*section).buffer.size;
                    (*section).buffer.size = __oldSize_2.wrapping_add(__count_2);
                    let mut __need_2: darray_size_t = (*section).buffer.size;
                    if __need_2 > (*section).buffer.alloc {
                        (*section).buffer.alloc = darray_next_alloc(
                            (*section).buffer.alloc,
                            __need_2,
                            ::core::mem::size_of::<i8>() as usize,
                        );
                        (*section).buffer.item = realloc(
                            (*section).buffer.item as *mut ::core::ffi::c_void,
                            ((*section).buffer.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<i8>() as usize),
                        ) as *mut i8;
                    }
                    std::ptr::copy_nonoverlapping(
                        (*stmt).modifier as *const u8,
                        (*section).buffer.item.offset(__oldSize_2 as isize) as *mut u8,
                        __count_2 as usize,
                    );
                }
                let section_flags: xkb_map_flags = (if valid as ::core::ffi::c_int != 0 {
                    (*xkb_file).flags as u32
                } else {
                    0 as u32
                }) as xkb_map_flags;
                let inc: xkb_file_include = {
                    let mut init = xkb_file_include {
                        valid_explicit_section: [0; 1],
                        c2rust_padding: [0; 3],
                        merge: (*stmt).merge,
                        path: path,
                        file: file,
                        section: section_name,
                        modifier: modifier,
                        flags: section_flags,
                    };
                    init.set_valid(valid);
                    init.set_explicit_section(!(*stmt).map.is_null());
                    init
                };
                let idx: darray_size_t = (*section).includes.size;
                (*section).includes.size =
                    (*section).includes.size.wrapping_add(1 as darray_size_t);
                let mut __need_3: darray_size_t = (*section).includes.size;
                if __need_3 > (*section).includes.alloc {
                    (*section).includes.alloc = darray_next_alloc(
                        (*section).includes.alloc,
                        __need_3,
                        ::core::mem::size_of::<xkb_file_include>() as usize,
                    );
                    (*section).includes.item =
                        realloc(
                            (*section).includes.item as *mut ::core::ffi::c_void,
                            ((*section).includes.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<xkb_file_include>() as usize),
                        ) as *mut xkb_file_include;
                }
                *(*section)
                    .includes
                    .item
                    .offset((*section).includes.size.wrapping_sub(1 as darray_size_t) as isize) =
                    inc;
                if group.is_null() {
                    let group_idx: darray_size_t = (*section).include_groups.size;
                    (*section).include_groups.size = (*section)
                        .include_groups
                        .size
                        .wrapping_add(1 as darray_size_t);
                    let mut __need_4: darray_size_t = (*section).include_groups.size;
                    if __need_4 > (*section).include_groups.alloc {
                        (*section).include_groups.alloc = darray_next_alloc(
                            (*section).include_groups.alloc,
                            __need_4,
                            ::core::mem::size_of::<xkb_file_include_group>() as usize,
                        );
                        (*section).include_groups.item = realloc(
                            (*section).include_groups.item as *mut ::core::ffi::c_void,
                            ((*section).include_groups.alloc as usize).wrapping_mul(
                                ::core::mem::size_of::<xkb_file_include_group>() as usize,
                            ),
                        )
                            as *mut xkb_file_include_group;
                    }
                    *(*section).include_groups.item.offset(
                        (*section)
                            .include_groups
                            .size
                            .wrapping_sub(1 as darray_size_t) as isize,
                    ) = xkb_file_include_group {
                        start: idx,
                        end: idx,
                    };
                    group = (*section).include_groups.item.offset(group_idx as isize)
                        as *mut xkb_file_include_group;
                } else {
                    (*group).end = idx;
                }
                FreeXkbFile(xkb_file);
            } else {
                let name: *const i8 =
                    xkb_file_section_get_string(section, (*section).name) as *const i8;
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s include failure in: %s%s%s%s\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as ::core::ffi::c_int,
                    xkb_file_type_name(file_type),
                    section_path,
                    if (*section).name != 0 {
                        b" (section: \"\0".as_ptr() as *const i8
                    } else {
                        b"\0".as_ptr() as *const i8
                    },
                    name,
                    if (*section).name != 0 {
                        b"\")\0".as_ptr() as *const i8
                    } else {
                        b"\0".as_ptr() as *const i8
                    },
                );
                FreeXkbFile(xkb_file);
                return false_0 != 0;
            }
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        return true_0 != 0;
    }
}
unsafe fn xkb_file_section_process(
    mut ctx: *mut xkb_context,
    mut flags: xkb_file_iterator_flags,
    mut path: *const i8,
    mut section: *mut xkb_file_section,
    mut xkb_file: *const XkbFile,
) -> bool {
    unsafe {
        let mut ok: bool = true_0 != 0;
        let mut stmt: *mut ParseCommon = (*xkb_file).defs;
        while !stmt.is_null() {
            if (*stmt).type_0 as u32 == STMT_INCLUDE as ::core::ffi::c_int as u32 {
                ok = xkb_file_section_append_includes(
                    ctx,
                    flags,
                    path,
                    section,
                    (*xkb_file).file_type,
                    stmt as *mut IncludeStmt,
                );
                if !ok {
                    break;
                }
            }
            stmt = (*stmt).next as *mut ParseCommon;
        }
        return ok;
    }
}

pub unsafe fn xkb_file_section_parse(
    mut ctx: *mut xkb_context,
    mut iterator_flags: xkb_file_iterator_flags,
    mut format: xkb_keymap_format,
    mut compile_flags: xkb_keymap_compile_flags,
    mut include_depth: u32,
    mut path: *const i8,
    mut map: *const i8,
    mut section: *mut xkb_file_section,
) -> bool {
    unsafe {
        if ExceedsIncludeMaxDepth(ctx, include_depth) {
            return false_0 != 0;
        }
        let mut file: *mut FILE = fopen(path, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        if file.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Cannot open file: %s\n\0".as_ptr() as *const i8,
                path,
            );
            return false_0 != 0;
        }
        let mut xkb_file: *mut XkbFile = XkbParseFile(ctx, file, path, map);
        fclose(file);
        if xkb_file.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Cannot parse map \"%s\" in file: %s\n\0".as_ptr() as *const i8,
                if !map.is_null() {
                    map
                } else {
                    b"(no map)\0".as_ptr() as *const i8
                },
                path,
            );
            return false_0 != 0;
        }
        xkb_file_section_reset(section);
        let no_includes: bool =
            iterator_flags as u32 & XKB_FILE_ITERATOR_NO_INCLUDES as ::core::ffi::c_int as u32 != 0;
        let ok: bool = xkb_file_section_set_meta_data(ctx, section, xkb_file) as ::core::ffi::c_int
            != 0
            && (no_includes as ::core::ffi::c_int != 0
                || xkb_file_section_process(ctx, iterator_flags, path, section, xkb_file)
                    as ::core::ffi::c_int
                    != 0);
        FreeXkbFile(xkb_file);
        return ok;
    }
}

pub unsafe fn xkb_file_iterator_new_from_buffer(
    mut ctx: *mut xkb_context,
    mut iterator_flags: xkb_file_iterator_flags,
    mut format: xkb_keymap_format,
    mut compile_flags: xkb_keymap_compile_flags,
    mut path: *const i8,
    mut map: *const i8,
    mut file_type: xkb_file_type,
    mut string: *const i8,
    mut length: usize,
) -> *mut xkb_file_iterator {
    unsafe {
        let iter: *mut xkb_file_iterator = calloc(
            1 as usize,
            ::core::mem::size_of::<xkb_file_iterator>() as usize,
        ) as *mut xkb_file_iterator;
        if iter.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot allocate file iterator\n\0".as_ptr() as *const i8,
                XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<xkb_file_iterator>();
        }
        (*iter).flags = iterator_flags;
        (*iter).ctx = ctx;
        (*iter).path = path;
        (*iter).map = map;
        (*iter).type_0 = file_type;
        xkb_file_section_init(&raw mut (*iter).section);
        if !XkbParseStringInit(
            ctx,
            &raw mut (*iter).scanner,
            string,
            length,
            path,
            ::core::ptr::null::<i8>(),
        ) {
            xkb_file_iterator_free(iter);
            return ::core::ptr::null_mut::<xkb_file_iterator>();
        }
        return iter;
    }
}

pub unsafe fn xkb_file_iterator_free(mut iter: *mut xkb_file_iterator) {
    unsafe {
        if iter.is_null() {
            return;
        }
        xkb_file_section_free(&raw mut (*iter).section);
        FreeXkbFile((*iter).pending_xkb_file);
        free(iter as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn xkb_file_iterator_next(
    mut iter: *mut xkb_file_iterator,
    mut section: *mut *const xkb_file_section,
) -> bool {
    unsafe {
        let mut xkb_file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        let mut process_includes: bool = false;
        let mut c2rust_current_block: u64;
        if (*iter).finished {
            *section = ::core::ptr::null::<xkb_file_section>();
            return true_0 != 0;
        }
        loop {
            xkb_file = ::core::ptr::null_mut::<XkbFile>();
            if !(*iter).pending_xkb_file.is_null() {
                if !(*iter).pending_section.is_null() {
                    xkb_file = (*iter).pending_section;
                    c2rust_current_block = 4521797313793381377;
                } else {
                    FreeXkbFile((*iter).pending_xkb_file);
                    (*iter).pending_xkb_file = ::core::ptr::null_mut::<XkbFile>();
                    c2rust_current_block = 1394248824506584008;
                }
            } else {
                c2rust_current_block = 1394248824506584008;
            }
            match c2rust_current_block {
                1394248824506584008 => {
                    if !XkbParseStringNext(
                        (*iter).ctx,
                        &raw mut (*iter).scanner,
                        (*iter).map,
                        &raw mut xkb_file,
                    ) {
                        xkb_logf!(
                            (*iter).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"Error while parsing section in file: %s\n\0".as_ptr() as *const i8,
                            (*iter).path,
                        );
                        c2rust_current_block = 3132808253564788397;
                        break;
                    } else if xkb_file.is_null() {
                        (*iter).finished = true_0 != 0;
                        *section = ::core::ptr::null::<xkb_file_section>();
                        return true_0 != 0;
                    }
                }
                _ => {}
            }
            xkb_file_section_reset(&raw mut (*iter).section);
            if !xkb_file_section_set_meta_data((*iter).ctx, &raw mut (*iter).section, xkb_file) {
                c2rust_current_block = 3132808253564788397;
                break;
            }
            *section = &raw mut (*iter).section;
            if (*xkb_file).file_type as u32 == FILE_TYPE_KEYMAP as ::core::ffi::c_int as u32 {
                (*iter).pending_xkb_file = xkb_file;
                (*iter).pending_section = (*xkb_file).defs as *mut XkbFile;
                (*iter).map = ::core::ptr::null::<i8>();
                return true_0 != 0;
            } else if (*iter).type_0 as u32 != FILE_TYPE_INVALID as ::core::ffi::c_int as u32
                && (*xkb_file).file_type as u32 != (*iter).type_0 as u32
            {
                if !(*iter).pending_xkb_file.is_null() {
                    (*iter).pending_section = (*xkb_file).common.next as *mut XkbFile;
                } else {
                    xkb_logf!(
                        (*iter).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"File type mismatch: %s, section: %s\n\0".as_ptr() as *const i8,
                        (*iter).path,
                        if !(*xkb_file).name.is_null() {
                            (*xkb_file).name as *const i8
                        } else {
                            b"(no name)\0".as_ptr() as *const i8
                        },
                    );
                    c2rust_current_block = 3132808253564788397;
                    break;
                }
            } else {
                if !(*iter).map.is_null() {
                    (*iter).finished = true_0 != 0;
                }
                process_includes = (*iter).flags as u32
                    & XKB_FILE_ITERATOR_NO_INCLUDES as ::core::ffi::c_int as u32
                    == 0;
                if process_includes as ::core::ffi::c_int != 0
                    && !xkb_file_section_process(
                        (*iter).ctx,
                        (*iter).flags,
                        (*iter).path,
                        &raw mut (*iter).section,
                        xkb_file,
                    )
                {
                    c2rust_current_block = 3132808253564788397;
                    break;
                } else {
                    c2rust_current_block = 11932355480408055363;
                    break;
                }
            }
        }
        match c2rust_current_block {
            3132808253564788397 => {
                FreeXkbFile(xkb_file);
                *section = ::core::ptr::null::<xkb_file_section>();
                return false_0 != 0;
            }
            _ => {
                if !(*iter).pending_section.is_null() {
                    (*iter).pending_section = (*xkb_file).common.next as *mut XkbFile;
                } else {
                    FreeXkbFile(xkb_file);
                }
                return true_0 != 0;
            }
        };
    }
}

pub unsafe fn xkb_file_section_get_string(
    mut section: *const xkb_file_section,
    mut idx: darray_size_t,
) -> *const i8 {
    unsafe {
        return (*section).buffer.item.offset(idx as isize) as *mut i8;
    }
}
