use crate::xkb::context_priv::xkb_context_sanitize_rule_names;
use crate::xkb_logf;
use c2rust_bitfields;

pub mod xkbcommon_errors_h {
    pub type xkb_error_code = ::core::ffi::c_int;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT: xkb_error_code = 914;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT: xkb_error_code = 876;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE: xkb_error_code = 450;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS: xkb_error_code = 371;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX: xkb_error_code = 237;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY: xkb_error_code = 214;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK: xkb_error_code = 60;
    pub const XKB_SUCCESS: xkb_error_code = 0;
    pub const XKB_ERROR_INVALID: xkb_error_code = -1;
}
pub mod rmlvo_h {
    pub use crate::xkb::rmlvo::rmlvo_h::*;
    pub type RMLVO = u32;
    pub const RMLVO_OPTIONS: RMLVO = 16;
    pub const RMLVO_VARIANT: RMLVO = 8;
    pub const RMLVO_LAYOUT: RMLVO = 4;
    pub const RMLVO_MODEL: RMLVO = 2;
    pub const RMLVO_RULES: RMLVO = 1;
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
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
    pub struct sval {
        pub len: usize,
        pub start: *const i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_sval {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut sval,
    }
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
    pub unsafe fn svaleq(mut s1: sval, mut s2: sval) -> bool {
        unsafe {
            return s1.len == s2.len
                && std::slice::from_raw_parts(s1.start as *const u8, s1.len)
                    == std::slice::from_raw_parts(s2.start as *const u8, s1.len);
        }
    }
    #[inline]
    pub unsafe fn svaleq_prefix(mut s1: sval, mut s2: sval) -> bool {
        unsafe {
            return s1.len <= s2.len
                && std::slice::from_raw_parts(s1.start as *const u8, s1.len)
                    == std::slice::from_raw_parts(s2.start as *const u8, s1.len);
        }
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
    pub unsafe fn scanner_skip_to_eol(mut s: *mut scanner) {
        unsafe {
            let mut nl: *const i8 = crate::xkb::utils::byte_memchr(
                (*s).s.offset((*s).pos as isize),
                b'\n',
                (*s).len.wrapping_sub((*s).pos),
            );
            let new_pos: usize = if !nl.is_null() {
                nl.offset_from((*s).s) as i64 as usize
            } else {
                (*s).len
            };
            (*s).pos = new_pos;
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
                return 0 != 0;
            }
            (*s).pos = (*s).pos.wrapping_add(1);
            return 1 != 0;
        }
    }
    #[inline]
    pub unsafe fn scanner_str(mut s: *mut scanner, mut string: *const i8, mut len: usize) -> bool {
        unsafe {
            if (*s).len.wrapping_sub((*s).pos) < len {
                return 0 != 0;
            }
            if std::slice::from_raw_parts((*s).s.offset((*s).pos as isize) as *const u8, len)
                != std::slice::from_raw_parts(string as *const u8, len)
            {
                return 0 != 0;
            }
            (*s).pos = (*s).pos.wrapping_add(len);
            return 1 != 0;
        }
    }
    #[inline]
    pub unsafe fn scanner_check_supported_char_encoding(mut scanner: *mut scanner) -> bool {
        unsafe {
            if scanner_str(scanner, b"\xEF\xBB\xBF\0".as_ptr() as *const i8, 3 as usize)
                as ::core::ffi::c_int
                != 0
                || (*scanner).len < 2 as usize
            {
                return 1 != 0;
            }
            if *(*scanner).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
                || *(*scanner).s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
            {
                let mut loc: scanner_loc = scanner_token_location(scanner);
                xkb_logf!(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: unexpected NULL character.\n",
                    XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*scanner).file_name),
                    loc.line,
                    loc.column,
                );
                return 0 != 0;
            }
            if !is_ascii(*(*scanner).s.offset(0 as ::core::ffi::c_int as isize)) {
                let mut loc_0: scanner_loc = scanner_token_location(scanner);
                xkb_logf!(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: unexpected non-ASCII character.\n",
                    XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*scanner).file_name),
                    loc_0.line,
                    loc_0.column,
                );
                return 0 != 0;
            }
            return 1 != 0;
        }
    }

    use super::messages_codes_h::{XKB_ERROR_INVALID_FILE_ENCODING, XKB_LOG_VERBOSITY_MINIMAL};
    use super::utils_h::is_ascii;
    use crate::xkb::shared_types::darray_size_t;
    use crate::xkb::shared_types::xkb_context;
    use crate::xkb::shared_types::XKB_LOG_LEVEL_ERROR;
    use crate::xkb_logf;
    pub unsafe fn scanner_token_location(s: *mut scanner) -> scanner_loc {
        unsafe {
            core::mem::transmute(crate::xkb::scanner_utils::scanner_token_location(
                s as *mut crate::xkb::scanner_utils::scanner_utils_h::scanner,
            ))
        }
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn isempty(mut s: *const i8) -> bool {
        unsafe {
            return s.is_null()
                || *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32;
        }
    }
    #[inline]
    pub unsafe fn is_ascii(mut ch: i8) -> bool {
        return ch as ::core::ffi::c_int & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int;
    }
    #[inline]
    pub unsafe fn is_space(mut ch: i8) -> bool {
        return ch as ::core::ffi::c_int == ' ' as i32
            || ch as ::core::ffi::c_int >= '\t' as i32 && ch as ::core::ffi::c_int <= '\r' as i32;
    }
    #[inline]
    pub unsafe fn is_graph(mut ch: i8) -> bool {
        return ch as ::core::ffi::c_int >= '!' as i32 && ch as ::core::ffi::c_int <= '~' as i32;
    }

    // map_file/unmap_file removed - use crate::xkb::utils::MappedFile instead
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe fn parse_dec_to_uint32_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut u32,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as u32)
                    < 10 as u32
                && result <= (4294967295 as u32).wrapping_div(10 as u32)
                && result.wrapping_mul(10 as u32)
                    <= (4294967295 as u32).wrapping_sub(
                        (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                            as ::core::ffi::c_uchar as u32,
                    )
            {
                result = result.wrapping_mul(10 as u32).wrapping_add(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u32,
                );
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len
                || (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as u32
                    >= 10 as u32
            {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
}
pub mod include_h {
    use libc::FILE;
    pub const MERGE_OVERRIDE_PREFIX: ::core::ffi::c_int = '+' as i32;
    pub const MERGE_AUGMENT_PREFIX: ::core::ffi::c_int = '|' as i32;
    pub const MERGE_REPLACE_PREFIX: ::core::ffi::c_int = '^' as i32;
    pub use crate::xkb::xkbcomp::include::expand_path;

    use super::ast_h::xkb_file_type;
    use crate::xkb::shared_types::xkb_context;

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
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
    }
}
pub mod rules_h {
    pub const OPTIONS_GROUP_SPECIFIER_PREFIX: ::core::ffi::c_int = '!' as i32;
}
pub mod utils_paths_h {
    pub use crate::xkb::utils_paths::is_absolute_path;
}

pub use self::ast_h::{
    xkb_file_type, _FILE_TYPE_NUM_ENTRIES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID,
    FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES,
    FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
};
pub use self::include_h::{
    expand_path, FindFileInXkbPath, MERGE_AUGMENT_PREFIX, MERGE_OVERRIDE_PREFIX,
    MERGE_REPLACE_PREFIX,
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
pub use self::rmlvo_h::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_layouts,
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options, RMLVO, RMLVO_LAYOUT, RMLVO_MODEL,
    RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use self::rules_h::OPTIONS_GROUP_SPECIFIER_PREFIX;
pub use self::scanner_utils_h::{
    darray_sval, scanner, scanner_check_supported_char_encoding, scanner_chr, scanner_eof,
    scanner_eol, scanner_init, scanner_loc, scanner_next, scanner_peek, scanner_skip_to_eol,
    scanner_str, scanner_token_location, sval, svaleq, svaleq_prefix,
};
pub use self::utils_h::{is_ascii, is_graph, is_space, isempty};
pub use self::utils_numbers_h::parse_dec_to_uint32_t;
use self::utils_paths_h::is_absolute_path;
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use crate::xkb::shared_types::XKB_MAX_GROUPS;
pub use crate::xkb::shared_types::{darray_char, darray_size_t};
use crate::xkb::utils::{
    cstr_len, cstr_len_safe, cstr_ncmp, darray_append, darray_appends, darray_appends_nul,
    darray_growalloc, darray_resize, darray_resize_zero,
};
use libc::{calloc, fclose, fopen, free, FILE};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matcher {
    pub ctx: *mut xkb_context,
    pub rmlvo: rule_names,
    pub val: lvalue,
    pub groups: C2Rust_Unnamed_6,
    pub mapping: mapping,
    pub rule: rule,
    pub pending_kccgst: kccgst_buffer,
    pub kccgst: [darray_char; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kccgst_buffer {
    pub buffer: darray_char,
    pub slices: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut kccgst_buffer_slice,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct kccgst_buffer_slice {
    #[bitfield(name = "length", ty = "u32", bits = "0..=27")]
    #[bitfield(name = "kccgst", ty = "rules_kccgst", bits = "28..=31")]
    pub length_kccgst: [u8; 4],
    pub layout: xkb_layout_index_t,
}
pub type rules_kccgst = u32;
pub const _KCCGST_NUM_ENTRIES: rules_kccgst = 5;
pub const KCCGST_GEOMETRY: rules_kccgst = 4;
pub const KCCGST_SYMBOLS: rules_kccgst = 3;
pub const KCCGST_COMPAT: rules_kccgst = 2;
pub const KCCGST_TYPES: rules_kccgst = 1;
pub const KCCGST_KEYCODES: rules_kccgst = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub mlvo_value_at_pos: [sval; 4],
    pub match_type_at_pos: [mlvo_match_type; 4],
    pub kccgst_value_at_pos: [sval; 5],
    pub num_mlvo_values: mlvo_index_t,
    pub num_kccgst_values: kccgst_index_t,
    pub skip: bool,
}
pub type kccgst_index_t = u8;
pub type mlvo_index_t = u8;
pub type mlvo_match_type = u32;
pub const MLVO_MATCH_GROUP: mlvo_match_type = 5;
pub const MLVO_MATCH_WILDCARD_ANY: mlvo_match_type = 4;
pub const MLVO_MATCH_WILDCARD_SOME: mlvo_match_type = 3;
pub const MLVO_MATCH_WILDCARD_NONE: mlvo_match_type = 2;
pub const MLVO_MATCH_WILDCARD_LEGACY: mlvo_match_type = 1;
pub const MLVO_MATCH_NORMAL: mlvo_match_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapping {
    pub mlvo_at_pos: [rules_mlvo; 4],
    pub num_mlvo: mlvo_index_t,
    pub defined_mlvo_mask: mlvo_mask_t,
    pub has_layout_idx_range: bool,
    pub c2rust_unnamed: C2Rust_Unnamed_3,
    pub c2rust_unnamed_0: C2Rust_Unnamed_2,
    pub kccgst_at_pos: [rules_kccgst; 5],
    pub num_kccgst: kccgst_index_t,
    pub defined_kccgst_mask: kccgst_mask_t,
}
pub type kccgst_mask_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub active: xkb_layout_mask_t,
    pub layouts_candidates_mask: xkb_layout_mask_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub layout_idx_min: xkb_layout_index_t,
    pub layout_idx_max: xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub layout_idx: xkb_layout_index_t,
    pub variant_idx: xkb_layout_index_t,
}
pub type mlvo_mask_t = u8;
pub type rules_mlvo = u32;
pub const _MLVO_NUM_ENTRIES: rules_mlvo = 4;
pub const MLVO_OPTION: rules_mlvo = 3;
pub const MLVO_VARIANT: rules_mlvo = 2;
pub const MLVO_LAYOUT: rules_mlvo = 1;
pub const MLVO_MODEL: rules_mlvo = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub name: sval,
    pub elements: darray_sval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union lvalue {
    pub string: sval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule_names {
    pub model: matched_sval,
    pub layouts: darray_matched_sval,
    pub variants: darray_matched_sval,
    pub options: darray_matched_sval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct darray_matched_sval {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut matched_sval,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct matched_sval {
    pub sval: sval,
    #[bitfield(name = "matched", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "layout", ty = "xkb_layout_index_t", bits = "1..=31")]
    pub matched_layout: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub const TOK_ERROR: rules_token = 11;
pub type rules_token = u32;
pub const TOK_INCLUDE: rules_token = 10;
pub const TOK_WILD_CARD_ANY: rules_token = 9;
pub const TOK_WILD_CARD_SOME: rules_token = 8;
pub const TOK_WILD_CARD_NONE: rules_token = 7;
pub const TOK_WILD_CARD_STAR: rules_token = 6;
pub const TOK_EQUALS: rules_token = 5;
pub const TOK_BANG: rules_token = 4;
pub const TOK_GROUP_NAME: rules_token = 3;
pub const TOK_IDENTIFIER: rules_token = 2;
pub const TOK_END_OF_LINE: rules_token = 1;
pub const TOK_END_OF_FILE: rules_token = 0;
pub const LAYOUT_INDEX_FIRST: layout_index_ranges = 4294967292;
pub const LAYOUT_INDEX_SINGLE: layout_index_ranges = 4294967291;
pub const LAYOUT_INDEX_ANY: layout_index_ranges = 4294967294;
pub const LAYOUT_INDEX_LATER: layout_index_ranges = 4294967293;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub name: *const i8,
    pub length: ::core::ffi::c_int,
    pub range: layout_index_ranges,
}
pub type layout_index_ranges = u32;
pub type wildcard_match_type = u32;
pub const WILDCARD_MATCH_ALL: wildcard_match_type = 1;
pub const WILDCARD_MATCH_NONEMPTY: wildcard_match_type = 0;
pub const MAX_INCLUDE_DEPTH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[inline]
unsafe fn is_ident(mut ch: i8) -> bool {
    unsafe {
        return is_graph(ch) as ::core::ffi::c_int != 0 && ch as ::core::ffi::c_int != '\\' as i32;
    }
}
unsafe fn lex(mut s: *mut scanner, mut val: *mut lvalue) -> rules_token {
    unsafe {
        loop {
            while scanner_chr(s, ' ' as i32 as i8) as ::core::ffi::c_int != 0
                || scanner_chr(s, '\t' as i32 as i8) as ::core::ffi::c_int != 0
                || scanner_chr(s, '\r' as i32 as i8) as ::core::ffi::c_int != 0
            {}
            if scanner_str(
                s,
                b"//\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 3]>() as usize).wrapping_sub(1 as usize),
            ) {
                scanner_skip_to_eol(s);
            }
            if scanner_eol(s) {
                while scanner_eol(s) {
                    scanner_next(s);
                }
                return TOK_END_OF_LINE;
            }
            if !scanner_chr(s, '\\' as i32 as i8) {
                break;
            }
            scanner_chr(s, '\r' as i32 as i8);
            if !scanner_eol(s) {
                let mut loc: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: illegal new line escape; must appear at end of line\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc.line,
                    loc.column,
                );
                return TOK_ERROR;
            }
            scanner_next(s);
        }
        if scanner_eof(s) {
            return TOK_END_OF_FILE;
        }
        (*s).token_pos = (*s).pos;
        if scanner_chr(s, '!' as i32 as i8) {
            return TOK_BANG;
        }
        if scanner_chr(s, '=' as i32 as i8) {
            return TOK_EQUALS;
        }
        if scanner_chr(s, '*' as i32 as i8) {
            return TOK_WILD_CARD_STAR;
        }
        if scanner_str(
            s,
            b"<none>\0".as_ptr() as *const i8,
            (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_NONE;
        }
        if scanner_str(
            s,
            b"<some>\0".as_ptr() as *const i8,
            (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_SOME;
        }
        if scanner_str(
            s,
            b"<any>\0".as_ptr() as *const i8,
            (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_ANY;
        }
        if scanner_chr(s, '$' as i32 as i8) {
            (*val).string.start = (*s).s.offset((*s).pos as isize);
            (*val).string.len = 0 as usize;
            while is_ident(scanner_peek(s)) {
                scanner_next(s);
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            if (*val).string.len == 0 as usize {
                let mut loc_0: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: unexpected character after '$'; expected name\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_0.line,
                    loc_0.column,
                );
                return TOK_ERROR;
            }
            return TOK_GROUP_NAME;
        }
        if scanner_str(
            s,
            b"include\0".as_ptr() as *const i8,
            (::core::mem::size_of::<[i8; 8]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_INCLUDE;
        }
        if is_ident(scanner_peek(s)) {
            (*val).string.start = (*s).s.offset((*s).pos as isize);
            (*val).string.len = 0 as usize;
            while is_ident(scanner_peek(s)) {
                scanner_next(s);
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            return TOK_IDENTIFIER;
        }
        let mut loc_1: scanner_loc = scanner_token_location(s);
        xkb_logf!(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            "[XKB-{:03}] {}:{}:{}: unrecognized token\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
            crate::xkb::utils::CStrDisplay((*s).file_name),
            loc_1.line,
            loc_1.column,
        );
        return TOK_ERROR;
    }
}
static mut rules_mlvo_svals: [sval; 4] = [sval {
    len: 0,
    start: ::core::ptr::null::<i8>(),
}; 4];
static mut rules_kccgst_svals: [sval; 5] = [sval {
    len: 0,
    start: ::core::ptr::null::<i8>(),
}; 5];
pub const OPTIONS_MATCH_ALL_GROUPS: ::core::ffi::c_int = XKB_MAX_GROUPS;
unsafe fn strip_spaces(mut v: sval) -> sval {
    unsafe {
        while v.len > 0 as usize
            && is_space(*v.start.offset(0 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                != 0
        {
            v.len = v.len.wrapping_sub(1);
            v.start = v.start.offset(1);
        }
        while v.len > 0 as usize
            && is_space(*v.start.offset(v.len.wrapping_sub(1 as usize) as isize))
                as ::core::ffi::c_int
                != 0
        {
            v.len = v.len.wrapping_sub(1);
        }
        return v;
    }
}
unsafe fn split_comma_separated_mlvo(
    mut ctx: *mut xkb_context,
    mut mlvo: rules_mlvo,
    mut s: *const i8,
) -> darray_matched_sval {
    unsafe {
        let mut arr: darray_matched_sval = darray_matched_sval {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<matched_sval>(),
        };
        if s.is_null() {
            let mut val: matched_sval = {
                let mut init = matched_sval {
                    matched_layout: [0; 4],
                    c2rust_padding: [0; 4],
                    sval: sval {
                        len: 0 as usize,
                        start: ::core::ptr::null::<i8>(),
                    },
                };
                init.set_matched(false);
                init.set_layout(0);
                init
            };
            darray_append(&mut arr.item, &mut arr.size, &mut arr.alloc, val);
            return arr;
        }
        loop {
            let mut val_0: matched_sval = {
                let mut init = matched_sval {
                    matched_layout: [0; 4],
                    c2rust_padding: [0; 4],
                    sval: sval {
                        len: 0 as usize,
                        start: s,
                    },
                };
                init.set_matched(0 != 0);
                init.set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t);
                init
            };
            while *s as ::core::ffi::c_int != '\0' as i32
                && *s as ::core::ffi::c_int != ',' as i32
                && *s as ::core::ffi::c_int != OPTIONS_GROUP_SPECIFIER_PREFIX
            {
                s = s.offset(1);
                val_0.sval.len = val_0.sval.len.wrapping_add(1);
            }
            val_0.sval = strip_spaces(val_0.sval);
            if *s as ::core::ffi::c_int == OPTIONS_GROUP_SPECIFIER_PREFIX {
                s = s.offset(1);
                let layout_start: *const i8 = s;
                let mut layout: xkb_layout_index_t = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                let mut count: ::core::ffi::c_int =
                    parse_dec_to_uint32_t(s, usize::MAX as usize, &raw mut layout);
                if count > 0 as ::core::ffi::c_int {
                    s = s.offset(count as isize);
                    if layout == 0 as xkb_layout_index_t
                        || layout > XKB_MAX_GROUPS as xkb_layout_index_t
                    {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "[XKB-{:03}] Invalid layout index {} for the RMVLO component: \"{}\"\n",
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                            layout,
                            crate::xkb::utils::CStrNDisplay(
                                val_0.sval.len as usize,
                                val_0.sval.start
                            ),
                        );
                    } else if mlvo as u32 != MLVO_OPTION as ::core::ffi::c_int as u32 {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "Layout index {} is not supported for the RMLVO component: \"{}\"\n",
                            layout,
                            crate::xkb::utils::CStrNDisplay(
                                val_0.sval.len as usize,
                                val_0.sval.start
                            ),
                        );
                    } else {
                        val_0.set_layout(
                            layout.wrapping_sub(1 as xkb_layout_index_t) as xkb_layout_index_t
                        );
                    }
                }
                let layout_index_end: *const i8 = s;
                while *s as ::core::ffi::c_int != '\0' as i32
                    && *s as ::core::ffi::c_int != ',' as i32
                {
                    s = s.offset(1);
                }
                if count <= 0 as ::core::ffi::c_int || layout_index_end != s {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Invalid layout index \"{}\" for the RMLVO component \"{}\"; discarding specifier.\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                        crate::xkb::utils::CStrNDisplay(s.offset_from(layout_start) as i64
                            as usize, layout_start),
                        crate::xkb::utils::CStrNDisplay(val_0.sval.len as usize, val_0.sval.start),
                    );
                    val_0.set_layout(
                        OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t as xkb_layout_index_t,
                    );
                }
            }
            darray_append(&mut arr.item, &mut arr.size, &mut arr.alloc, val_0);
            if *s as ::core::ffi::c_int == '\0' as i32 {
                break;
            }
            if *s as ::core::ffi::c_int == ',' as i32 {
                s = s.offset(1);
            }
        }
        return arr;
    }
}
unsafe fn matcher_new_from_rmlvo(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut rules: *mut *const i8,
) -> *mut matcher {
    unsafe {
        let mut m: *mut matcher =
            calloc(1 as usize, ::core::mem::size_of::<matcher>() as usize) as *mut matcher;
        if m.is_null() {
            return ::core::ptr::null_mut::<matcher>();
        }
        (*m).ctx = (*rmlvo).ctx;
        let mut names: xkb_rule_names = xkb_rule_names {
            rules: (*rmlvo).rules,
            model: (*rmlvo).model,
            layout: if (*rmlvo).layouts.size == 0 as darray_size_t {
                ::core::ptr::null::<i8>()
            } else {
                b"x\0".as_ptr() as *const i8
            },
            variant: if (*rmlvo).layouts.size == 0 as darray_size_t {
                ::core::ptr::null::<i8>()
            } else {
                b"x\0".as_ptr() as *const i8
            },
            options: if (*rmlvo).options.size == 0 as darray_size_t {
                ::core::ptr::null::<i8>()
            } else {
                b"x\0".as_ptr() as *const i8
            },
        };
        let changed: RMLVO = xkb_context_sanitize_rule_names((*rmlvo).ctx, &raw mut names) as RMLVO;
        if changed as u32 & RMLVO_RULES as ::core::ffi::c_int as u32 != 0 {
            *rules = names.rules;
        } else {
            *rules = (*rmlvo).rules;
        }
        if changed as u32 & RMLVO_MODEL as ::core::ffi::c_int as u32 != 0 {
            (*m).rmlvo.model.sval.start = names.model;
        } else {
            (*m).rmlvo.model.sval.start = (*rmlvo).model;
        }
        (*m).rmlvo.model.sval.len = cstr_len_safe((*rmlvo).model);
        (*m).rmlvo
            .model
            .set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t as xkb_layout_index_t);
        if changed as u32 & RMLVO_LAYOUT as ::core::ffi::c_int as u32 != 0 {
            (*m).rmlvo.layouts =
                split_comma_separated_mlvo((*rmlvo).ctx, MLVO_LAYOUT, names.layout);
            (*m).rmlvo.variants =
                split_comma_separated_mlvo((*rmlvo).ctx, MLVO_VARIANT, names.variant);
            if (*m).rmlvo.layouts.size > (*m).rmlvo.variants.size {
                if !isempty(names.variant) {
                    xkb_logf!(
                        (*m).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "More layouts than variants: \"{}\" vs. \"{}\".\n",
                        crate::xkb::utils::CStrDisplay(if !names.layout.is_null() {
                            names.layout
                        } else {
                            b"(none)\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if !names.variant.is_null() {
                            names.variant
                        } else {
                            b"(none)\0".as_ptr() as *const i8
                        }),
                    );
                }
                darray_resize_zero(
                    &mut (*m).rmlvo.variants.item,
                    &mut (*m).rmlvo.variants.size,
                    &mut (*m).rmlvo.variants.alloc,
                    (*m).rmlvo.layouts.size,
                );
            } else if (*m).rmlvo.layouts.size < (*m).rmlvo.variants.size {
                xkb_logf!(
                    (*m).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Less layouts than variants: \"{}\" vs. \"{}\".\n",
                    crate::xkb::utils::CStrDisplay(if !names.layout.is_null() {
                        names.layout
                    } else {
                        b"(none)\0".as_ptr() as *const i8
                    }),
                    crate::xkb::utils::CStrDisplay(if !names.variant.is_null() {
                        names.variant
                    } else {
                        b"(none)\0".as_ptr() as *const i8
                    }),
                );
                darray_resize(
                    &mut (*m).rmlvo.variants.item,
                    &mut (*m).rmlvo.variants.size,
                    &mut (*m).rmlvo.variants.alloc,
                    (*m).rmlvo.layouts.size,
                );
            }
        } else {
            let mut layout: *mut xkb_rmlvo_builder_layout =
                ::core::ptr::null_mut::<xkb_rmlvo_builder_layout>();
            if !(*rmlvo).layouts.item.is_null() {
                layout = (*rmlvo)
                    .layouts
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut xkb_rmlvo_builder_layout;
                while layout
                    < (*rmlvo).layouts.item.offset((*rmlvo).layouts.size as isize)
                        as *mut xkb_rmlvo_builder_layout
                {
                    let mut val: matched_sval = {
                        let mut init = matched_sval {
                            matched_layout: [0; 4],
                            c2rust_padding: [0; 4],
                            sval: sval {
                                len: cstr_len_safe((*layout).layout),
                                start: (*layout).layout,
                            },
                        };
                        init.set_matched(0 != 0);
                        init.set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t);
                        init
                    };
                    darray_append(
                        &mut (*m).rmlvo.layouts.item,
                        &mut (*m).rmlvo.layouts.size,
                        &mut (*m).rmlvo.layouts.alloc,
                        val,
                    );
                    val.sval.start = (*layout).variant;
                    val.sval.len = cstr_len_safe((*layout).variant);
                    darray_append(
                        &mut (*m).rmlvo.variants.item,
                        &mut (*m).rmlvo.variants.size,
                        &mut (*m).rmlvo.variants.alloc,
                        val,
                    );
                    layout = layout.offset(1);
                }
            }
        }
        if changed as u32 & RMLVO_OPTIONS as ::core::ffi::c_int as u32 != 0 {
            (*m).rmlvo.options =
                split_comma_separated_mlvo((*rmlvo).ctx, MLVO_OPTION, names.options);
        } else {
            let mut option: *mut xkb_rmlvo_builder_option =
                ::core::ptr::null_mut::<xkb_rmlvo_builder_option>();
            if !(*rmlvo).options.item.is_null() {
                option = (*rmlvo)
                    .options
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut xkb_rmlvo_builder_option;
                while option
                    < (*rmlvo).options.item.offset((*rmlvo).options.size as isize)
                        as *mut xkb_rmlvo_builder_option
                {
                    let mut val_0: matched_sval = {
                        let mut init = matched_sval {
                            matched_layout: [0; 4],
                            c2rust_padding: [0; 4],
                            sval: sval {
                                len: cstr_len_safe((*option).option),
                                start: (*option).option,
                            },
                        };
                        init.set_matched(0 != 0);
                        init.set_layout(
                            if (*option).layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t
                            } else {
                                (*option).layout
                            },
                        );
                        init
                    };
                    darray_append(
                        &mut (*m).rmlvo.options.item,
                        &mut (*m).rmlvo.options.size,
                        &mut (*m).rmlvo.options.alloc,
                        val_0,
                    );
                    option = option.offset(1);
                }
            }
        }
        return m;
    }
}
unsafe fn matcher_new_from_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *const xkb_rule_names,
) -> *mut matcher {
    unsafe {
        let mut m: *mut matcher =
            calloc(1 as usize, ::core::mem::size_of::<matcher>() as usize) as *mut matcher;
        if m.is_null() {
            return ::core::ptr::null_mut::<matcher>();
        }
        (*m).ctx = ctx;
        (*m).rmlvo.model.sval.start = (*rmlvo).model;
        (*m).rmlvo.model.sval.len = cstr_len_safe((*rmlvo).model);
        (*m).rmlvo
            .model
            .set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t as xkb_layout_index_t);
        (*m).rmlvo.layouts = split_comma_separated_mlvo(ctx, MLVO_LAYOUT, (*rmlvo).layout);
        (*m).rmlvo.variants = split_comma_separated_mlvo(ctx, MLVO_VARIANT, (*rmlvo).variant);
        (*m).rmlvo.options = split_comma_separated_mlvo(ctx, MLVO_OPTION, (*rmlvo).options);
        if (*m).rmlvo.layouts.size > (*m).rmlvo.variants.size {
            if !isempty((*rmlvo).variant) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "More layouts than variants: \"{}\" vs. \"{}\".\n",
                    crate::xkb::utils::CStrDisplay(if !(*rmlvo).layout.is_null() {
                        (*rmlvo).layout
                    } else {
                        b"(none)\0".as_ptr() as *const i8
                    }),
                    crate::xkb::utils::CStrDisplay(if !(*rmlvo).variant.is_null() {
                        (*rmlvo).variant
                    } else {
                        b"(none)\0".as_ptr() as *const i8
                    }),
                );
            }
            darray_resize_zero(
                &mut (*m).rmlvo.variants.item,
                &mut (*m).rmlvo.variants.size,
                &mut (*m).rmlvo.variants.alloc,
                (*m).rmlvo.layouts.size,
            );
        } else if (*m).rmlvo.layouts.size < (*m).rmlvo.variants.size {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "Less layouts than variants: \"{}\" vs. \"{}\".\n",
                crate::xkb::utils::CStrDisplay(if !(*rmlvo).layout.is_null() {
                    (*rmlvo).layout
                } else {
                    b"(none)\0".as_ptr() as *const i8
                }),
                crate::xkb::utils::CStrDisplay(if !(*rmlvo).variant.is_null() {
                    (*rmlvo).variant
                } else {
                    b"(none)\0".as_ptr() as *const i8
                }),
            );
            darray_resize(
                &mut (*m).rmlvo.variants.item,
                &mut (*m).rmlvo.variants.size,
                &mut (*m).rmlvo.variants.alloc,
                (*m).rmlvo.layouts.size,
            );
        }
        return m;
    }
}
unsafe fn matcher_free(mut m: *mut matcher) {
    unsafe {
        if m.is_null() {
            return;
        }
        crate::xkb::utils::darray_free(
            &mut (*m).rmlvo.layouts.item,
            &mut (*m).rmlvo.layouts.size,
            &mut (*m).rmlvo.layouts.alloc,
        );
        crate::xkb::utils::darray_free(
            &mut (*m).rmlvo.variants.item,
            &mut (*m).rmlvo.variants.size,
            &mut (*m).rmlvo.variants.alloc,
        );
        crate::xkb::utils::darray_free(
            &mut (*m).rmlvo.options.item,
            &mut (*m).rmlvo.options.size,
            &mut (*m).rmlvo.options.alloc,
        );
        let mut group: *mut group = ::core::ptr::null_mut::<group>();
        if !(*m).groups.item.is_null() {
            group = (*m).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut group;
            while group < (*m).groups.item.offset((*m).groups.size as isize) as *mut group {
                crate::xkb::utils::darray_free(
                    &mut (*group).elements.item,
                    &mut (*group).elements.size,
                    &mut (*group).elements.alloc,
                );
                group = group.offset(1);
            }
        }
        crate::xkb::utils::darray_free(
            &mut (*m).pending_kccgst.buffer.item,
            &mut (*m).pending_kccgst.buffer.size,
            &mut (*m).pending_kccgst.buffer.alloc,
        );
        crate::xkb::utils::darray_free(
            &mut (*m).pending_kccgst.slices.item,
            &mut (*m).pending_kccgst.slices.size,
            &mut (*m).pending_kccgst.slices.alloc,
        );
        let mut i: kccgst_index_t = 0 as kccgst_index_t;
        while (i as ::core::ffi::c_int)
            < _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as kccgst_index_t as ::core::ffi::c_int
        {
            crate::xkb::utils::darray_free(
                &mut (*m).kccgst[i as usize].item,
                &mut (*m).kccgst[i as usize].size,
                &mut (*m).kccgst[i as usize].alloc,
            );
            i = i.wrapping_add(1);
        }
        crate::xkb::utils::darray_free(
            &mut (*m).groups.item,
            &mut (*m).groups.size,
            &mut (*m).groups.alloc,
        );
        free(m as *mut ::core::ffi::c_void);
    }
}
unsafe fn matcher_group_start_new(mut m: *mut matcher, mut name: sval) {
    unsafe {
        let mut group: group = group {
            name: name,
            elements: darray_sval {
                size: 0 as darray_size_t,
                alloc: 0 as darray_size_t,
                item: ::core::ptr::null_mut::<sval>(),
            },
        };
        darray_append(
            &mut (*m).groups.item,
            &mut (*m).groups.size,
            &mut (*m).groups.alloc,
            group,
        );
    }
}
unsafe fn matcher_group_add_element(mut m: *mut matcher, mut s: *mut scanner, mut element: sval) {
    unsafe {
        let last_group = &mut *(*m)
            .groups
            .item
            .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize);
        darray_append(
            &mut last_group.elements.item,
            &mut last_group.elements.size,
            &mut last_group.elements.alloc,
            element,
        );
    }
}
unsafe fn matcher_include(
    mut m: *mut matcher,
    mut parent_scanner: *mut scanner,
    mut include_depth: u32,
    mut inc: sval,
) {
    unsafe {
        if include_depth >= MAX_INCLUDE_DEPTH as u32 {
            let mut loc: scanner_loc = scanner_token_location(parent_scanner);
            xkb_logf!(
                (*parent_scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "{}:{}:{}: maximum include depth ({}) exceeded; maybe there is an include loop?\n",
                crate::xkb::utils::CStrDisplay((*parent_scanner).file_name),
                loc.line,
                loc.column,
                MAX_INCLUDE_DEPTH,
            );
            return;
        }
        let mut stmt_file: *const i8 = inc.start;
        let mut stmt_file_len: usize = inc.len;
        let mut buf: [i8; 4096] = [
            0 as ::core::ffi::c_int as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let expanded: isize = expand_path(
            (*m).ctx,
            (*parent_scanner).file_name,
            stmt_file,
            stmt_file_len,
            FILE_TYPE_RULES,
            &raw mut buf as *mut i8,
            ::core::mem::size_of::<[i8; 4096]>() as usize,
        ) as isize;
        if expanded < 0 as isize {
            return;
        } else if expanded > 0 as isize {
            stmt_file = &raw mut buf as *mut i8;
            stmt_file_len = expanded as usize;
        }
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut offset: u32 = 0 as u32;
        let absolute_path: bool = is_absolute_path(stmt_file) as bool;
        if absolute_path {
            if expanded == 0 {
                if stmt_file_len < ::core::mem::size_of::<[i8; 4096]>() as usize {
                    std::ptr::copy_nonoverlapping(
                        stmt_file as *const u8,
                        &raw mut buf as *mut i8 as *mut u8,
                        stmt_file_len,
                    );
                    buf[stmt_file_len as usize] = '\0' as i32 as i8;
                    stmt_file = &raw mut buf as *mut i8;
                } else {
                    xkb_logf!(
                        (*m).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Path is too long: {} > {}, got raw path: {}\n",
                        XKB_ERROR_INVALID_PATH as ::core::ffi::c_int,
                        stmt_file_len,
                        ::core::mem::size_of::<[i8; 4096]>(),
                        crate::xkb::utils::CStrNDisplay(stmt_file_len as usize, stmt_file),
                    );
                    return;
                }
            } else {
            }
            file = fopen(stmt_file, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        } else if (expanded != 0) as ::core::ffi::c_int as i64 != 0 {
            file = ::core::ptr::null_mut::<FILE>();
        } else {
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name,
                stmt_file,
                stmt_file_len,
                FILE_TYPE_RULES,
                &raw mut buf as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
                &raw mut offset,
                1 != 0,
            );
        }
        while !file.is_null() {
            let mut ret: bool = read_rules_file(
                (*m).ctx,
                m,
                include_depth.wrapping_add(1 as u32),
                file,
                &raw mut buf as *mut i8,
            );
            fclose(file);
            if ret {
                return;
            }
            xkb_logf!(
                (*m).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "No components returned from included XKB rules \"{}\"\n",
                crate::xkb::utils::CStrDisplay(&raw mut buf as *mut i8),
            );
            if absolute_path {
                break;
            }
            offset = offset.wrapping_add(1);
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name,
                stmt_file,
                stmt_file_len,
                FILE_TYPE_RULES,
                &raw mut buf as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
                &raw mut offset,
                1 != 0,
            );
        }
        xkb_logf!(
            (*m).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            "Failed to open included XKB rules \"{}\"\n",
            crate::xkb::utils::CStrNDisplay(stmt_file_len as usize, stmt_file),
        );
    }
}
unsafe fn matcher_mapping_start_new(mut m: *mut matcher) {
    unsafe {
        let mut i: mlvo_index_t = 0 as mlvo_index_t;
        while (i as ::core::ffi::c_int)
            < _MLVO_NUM_ENTRIES as ::core::ffi::c_int as mlvo_index_t as ::core::ffi::c_int
        {
            (*m).mapping.mlvo_at_pos[i as usize] = _MLVO_NUM_ENTRIES;
            i = i.wrapping_add(1);
        }
        let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
        while (i_0 as ::core::ffi::c_int)
            < _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as kccgst_index_t as ::core::ffi::c_int
        {
            (*m).mapping.kccgst_at_pos[i_0 as usize] = _KCCGST_NUM_ENTRIES;
            i_0 = i_0.wrapping_add(1);
        }
        (*m).mapping.has_layout_idx_range = 0 != 0;
        (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx =
            XKB_LAYOUT_INVALID as xkb_layout_index_t;
        (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx =
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx;
        (*m).mapping.num_kccgst = 0 as kccgst_index_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_kccgst as mlvo_index_t;
        (*m).mapping.defined_mlvo_mask = 0 as mlvo_mask_t;
        (*m).mapping.defined_kccgst_mask = 0 as kccgst_mask_t;
        (*m).mapping.c2rust_unnamed_0.active = 1 as xkb_layout_mask_t;
    }
}
unsafe fn parse_layout_int_index(
    mut s: *const i8,
    mut max_len: usize,
    mut out: *mut xkb_layout_index_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: u32 = 0 as u32;
        let count: ::core::ffi::c_int = parse_dec_to_uint32_t(
            s.offset(1 as ::core::ffi::c_int as isize) as *const i8,
            max_len.wrapping_sub(2 as usize),
            &raw mut val,
        ) as ::core::ffi::c_int;
        if count <= 0 as ::core::ffi::c_int
            || *s.offset((1 as ::core::ffi::c_int + count) as isize) as ::core::ffi::c_int
                != ']' as i32
            || val == 0 as u32
            || val > XKB_MAX_GROUPS as u32
        {
            return -1 as ::core::ffi::c_int;
        }
        *out = val.wrapping_sub(1 as u32) as xkb_layout_index_t;
        return count + 2 as ::core::ffi::c_int;
    }
}
unsafe fn extract_layout_index(
    mut s: *const i8,
    mut max_len: usize,
    mut out: *mut xkb_layout_index_t,
) -> ::core::ffi::c_int {
    unsafe {
        *out = XKB_LAYOUT_INVALID as xkb_layout_index_t;
        if max_len < 3 as usize
            || *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '[' as i32
        {
            return -1 as ::core::ffi::c_int;
        }
        if max_len > 3 as usize
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '%' as i32
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'i' as i32
            && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ']' as i32
        {
            return 4 as ::core::ffi::c_int;
        }
        return parse_layout_int_index(s, max_len, out);
    }
}
unsafe fn extract_mapping_layout_index(
    mut s: *const i8,
    mut max_len: usize,
    mut out: *mut xkb_layout_index_t,
) -> ::core::ffi::c_int {
    unsafe {
        static mut names: [C2Rust_Unnamed_7; 4] = [
            C2Rust_Unnamed_7 {
                name: b"single]\0".as_ptr() as *const i8,
                length: 7 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_SINGLE,
            },
            C2Rust_Unnamed_7 {
                name: b"first]\0".as_ptr() as *const i8,
                length: 6 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_FIRST,
            },
            C2Rust_Unnamed_7 {
                name: b"later]\0".as_ptr() as *const i8,
                length: 6 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_LATER,
            },
            C2Rust_Unnamed_7 {
                name: b"any]\0".as_ptr() as *const i8,
                length: 4 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_ANY,
            },
        ];
        if max_len < 3 as usize
            || *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '[' as i32
        {
            *out = XKB_LAYOUT_INVALID as xkb_layout_index_t;
            return -1 as ::core::ffi::c_int;
        }
        let mut k: u32 = 0 as u32;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_7; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_7>() as usize)
        {
            if cstr_ncmp(
                s.offset(1 as ::core::ffi::c_int as isize) as *const i8,
                names[k as usize].name,
                names[k as usize].length as usize,
            ) == 0 as ::core::ffi::c_int
            {
                *out = names[k as usize].range as xkb_layout_index_t;
                return names[k as usize].length + 1 as ::core::ffi::c_int;
            }
            k = k.wrapping_add(1);
        }
        *out = XKB_LAYOUT_INVALID as xkb_layout_index_t;
        return parse_layout_int_index(s, max_len, out);
    }
}
#[inline]
unsafe fn is_mlvo_mask_defined(mut m: *mut matcher, mut mlvo: rules_mlvo) -> bool {
    unsafe {
        return (*m).mapping.defined_mlvo_mask as u32 & (1 as u32) << mlvo as u32 != 0;
    }
}
unsafe fn matcher_mapping_set_mlvo(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        let mut mlvo: rules_mlvo = MLVO_MODEL;
        let mut mlvo_sval: sval = sval {
            len: 0,
            start: ::core::ptr::null::<i8>(),
        };
        mlvo = MLVO_MODEL;
        while (mlvo as u32) < _MLVO_NUM_ENTRIES as ::core::ffi::c_int as u32 {
            mlvo_sval = rules_mlvo_svals[mlvo as usize];
            if svaleq_prefix(mlvo_sval, ident) {
                break;
            }
            mlvo += 1;
        }
        if mlvo as u32 >= _MLVO_NUM_ENTRIES as ::core::ffi::c_int as u32 {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
                crate::xkb::utils::CStrNDisplay(ident.len as usize, ident.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
            return;
        }
        if is_mlvo_mask_defined(m, mlvo) {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
                crate::xkb::utils::CStrNDisplay(mlvo_sval.len as usize, mlvo_sval.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
            return;
        }
        if mlvo_sval.len < ident.len {
            let mut idx: xkb_layout_index_t = 0;
            let mut consumed: ::core::ffi::c_int = extract_mapping_layout_index(
                ident.start.offset(mlvo_sval.len as isize),
                ident.len.wrapping_sub(mlvo_sval.len),
                &raw mut idx,
            );
            if ident.len.wrapping_sub(mlvo_sval.len) as ::core::ffi::c_int != consumed {
                let mut loc_1: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" may only be followed by a valid group index; ignoring rule set\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_1.line,
                    loc_1.column,
                    crate::xkb::utils::CStrNDisplay(mlvo_sval.len as usize, mlvo_sval.start),
                );
                (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
                return;
            }
            if mlvo as u32 == MLVO_LAYOUT as ::core::ffi::c_int as u32 {
                (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx = idx;
            } else if mlvo as u32 == MLVO_VARIANT as ::core::ffi::c_int as u32 {
                (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx = idx;
            } else {
                let mut loc_2: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" cannot be followed by a group index; ignoring rule set\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_2.line,
                    loc_2.column,
                    crate::xkb::utils::CStrNDisplay(mlvo_sval.len as usize, mlvo_sval.start),
                );
                (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
                return;
            }
        } else if mlvo as u32 == MLVO_LAYOUT as ::core::ffi::c_int as u32 {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx =
                LAYOUT_INDEX_SINGLE as u32 as xkb_layout_index_t;
        } else if mlvo as u32 == MLVO_VARIANT as ::core::ffi::c_int as u32 {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx =
                LAYOUT_INDEX_SINGLE as u32 as xkb_layout_index_t;
        }
        if (mlvo as u32 == MLVO_LAYOUT as ::core::ffi::c_int as u32
            && is_mlvo_mask_defined(m, MLVO_VARIANT) as ::core::ffi::c_int != 0
            || mlvo as u32 == MLVO_VARIANT as ::core::ffi::c_int as u32
                && is_mlvo_mask_defined(m, MLVO_LAYOUT) as ::core::ffi::c_int != 0)
            && (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                != (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        {
            let mut loc_3: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"layout\" index must be the same as the \"variant\" index\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_3.line,
                loc_3.column,
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
            return;
        }
        (*m).mapping.mlvo_at_pos[(*m).mapping.num_mlvo as usize] = mlvo;
        (*m).mapping.defined_mlvo_mask = ((*m).mapping.defined_mlvo_mask as ::core::ffi::c_int
            | (1 as u32 as mlvo_mask_t as ::core::ffi::c_int) << mlvo as u32)
            as mlvo_mask_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_mlvo.wrapping_add(1);
    }
}
unsafe fn matcher_mapping_set_layout_bounds(mut m: *mut matcher) {
    unsafe {
        let mut idx: xkb_layout_index_t = if (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
            < (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
        } else {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        };
        let mut c2rust_current_block_17: u64;
        match idx {
            XKB_LAYOUT_INVALID => {
                (*m).mapping.has_layout_idx_range = 0 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    XKB_LAYOUT_INVALID as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    XKB_LAYOUT_INVALID as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask = 0x1 as xkb_layout_mask_t;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967293 => {
                (*m).mapping.has_layout_idx_range = 1 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    1 as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    (if (32 as darray_size_t) < (*m).rmlvo.layouts.size {
                        32 as darray_size_t
                    } else {
                        (*m).rmlvo.layouts.size
                    }) as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    (((1 as u64) << (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max)
                        .wrapping_sub(1 as u64) as xkb_layout_mask_t as u64
                        & !(1 as u64)) as xkb_layout_mask_t;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967294 => {
                (*m).mapping.has_layout_idx_range = 1 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    0 as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    (if (32 as darray_size_t) < (*m).rmlvo.layouts.size {
                        32 as darray_size_t
                    } else {
                        (*m).rmlvo.layouts.size
                    }) as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    (((1 as u64) << (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max)
                        as xkb_layout_mask_t as u64)
                        .wrapping_sub(1 as u64) as xkb_layout_mask_t;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967291 | 4294967292 => {
                idx = 0 as xkb_layout_index_t;
                c2rust_current_block_17 = 9641388756612255828;
            }
            _ => {
                c2rust_current_block_17 = 9641388756612255828;
            }
        }
        match c2rust_current_block_17 {
            9641388756612255828 => {
                (*m).mapping.has_layout_idx_range = 0 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min = idx;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    idx.wrapping_add(1 as xkb_layout_index_t);
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    ((1 as u32) << idx) as xkb_layout_mask_t;
            }
            _ => {}
        };
    }
}
unsafe fn matcher_mapping_set_kccgst(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        let mut kccgst: rules_kccgst = KCCGST_KEYCODES;
        let mut kccgst_sval: sval = sval {
            len: 0,
            start: ::core::ptr::null::<i8>(),
        };
        kccgst = KCCGST_KEYCODES;
        while (kccgst as u32) < _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as u32 {
            kccgst_sval = rules_kccgst_svals[kccgst as usize];
            if svaleq(rules_kccgst_svals[kccgst as usize], ident) {
                break;
            }
            kccgst += 1;
        }
        if kccgst as u32 >= _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as u32 {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
                crate::xkb::utils::CStrNDisplay(ident.len as usize, ident.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
            return;
        }
        if (*m).mapping.defined_kccgst_mask as u32 & (1 as u32) << kccgst as u32 != 0 {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
                crate::xkb::utils::CStrNDisplay(kccgst_sval.len as usize, kccgst_sval.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
            return;
        }
        (*m).mapping.kccgst_at_pos[(*m).mapping.num_kccgst as usize] = kccgst;
        (*m).mapping.defined_kccgst_mask = ((*m).mapping.defined_kccgst_mask as ::core::ffi::c_int
            | (1 as u32 as kccgst_mask_t as ::core::ffi::c_int) << kccgst as u32)
            as kccgst_mask_t;
        (*m).mapping.num_kccgst = (*m).mapping.num_kccgst.wrapping_add(1);
    }
}
unsafe fn matcher_mapping_verify(mut m: *mut matcher, mut s: *mut scanner) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        if (*m).mapping.num_mlvo as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the left hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
        } else if (*m).mapping.num_kccgst as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the right hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
            );
        } else {
            if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
                match (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx {
                    4294967291 => {
                        c2rust_current_block = 4840043166261277618;
                        match c2rust_current_block {
                            14825033830842003582 => {
                                if (*m).rmlvo.layouts.size < 2 as darray_size_t
                                    || (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                                        >= (*m).rmlvo.layouts.size as xkb_layout_index_t
                                {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                            _ => {
                                if (*m).rmlvo.layouts.size > 1 as darray_size_t {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                        }
                    }
                    4294967294 | 4294967293 | 4294967292 => {
                        c2rust_current_block = 8831408221741692167;
                    }
                    _ => {
                        c2rust_current_block = 14825033830842003582;
                        match c2rust_current_block {
                            14825033830842003582 => {
                                if (*m).rmlvo.layouts.size < 2 as darray_size_t
                                    || (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                                        >= (*m).rmlvo.layouts.size as xkb_layout_index_t
                                {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                            _ => {
                                if (*m).rmlvo.layouts.size > 1 as darray_size_t {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                        }
                    }
                }
            } else {
                c2rust_current_block = 8831408221741692167;
            }
            match c2rust_current_block {
                436805222042109220 => {}
                _ => {
                    if is_mlvo_mask_defined(m, MLVO_VARIANT) {
                        match (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx {
                            4294967291 => {
                                c2rust_current_block = 13345507216710712890;
                                match c2rust_current_block {
                                    10338831042980687939 => {
                                        if (*m).rmlvo.variants.size < 2 as darray_size_t
                                            || (*m)
                                                .mapping
                                                .c2rust_unnamed
                                                .c2rust_unnamed
                                                .variant_idx
                                                >= (*m).rmlvo.variants.size as xkb_layout_index_t
                                        {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                    _ => {
                                        if (*m).rmlvo.variants.size > 1 as darray_size_t {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                }
                            }
                            4294967294 | 4294967293 | 4294967292 => {
                                c2rust_current_block = 10652014663920648156;
                            }
                            _ => {
                                c2rust_current_block = 10338831042980687939;
                                match c2rust_current_block {
                                    10338831042980687939 => {
                                        if (*m).rmlvo.variants.size < 2 as darray_size_t
                                            || (*m)
                                                .mapping
                                                .c2rust_unnamed
                                                .c2rust_unnamed
                                                .variant_idx
                                                >= (*m).rmlvo.variants.size as xkb_layout_index_t
                                        {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                    _ => {
                                        if (*m).rmlvo.variants.size > 1 as darray_size_t {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        c2rust_current_block = 10652014663920648156;
                    }
                    match c2rust_current_block {
                        436805222042109220 => {}
                        _ => return 1 != 0,
                    }
                }
            }
        }
        (*m).mapping.c2rust_unnamed_0.active = 0 as xkb_layout_mask_t;
        return 0 != 0;
    }
}
unsafe fn matcher_rule_start_new(mut m: *mut matcher) {
    unsafe {
        std::ptr::write_bytes::<rule>(&raw mut (*m).rule as *mut rule, 0u8, 1);
        (*m).rule.skip = (*m).mapping.c2rust_unnamed_0.active == 0;
    }
}
unsafe fn matcher_rule_set_mlvo_common(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
    mut match_type: mlvo_match_type,
) {
    unsafe {
        if (*m).rule.num_mlvo_values as ::core::ffi::c_int
            >= (*m).mapping.num_mlvo as ::core::ffi::c_int
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            (*m).rule.skip = 1 != 0;
            return;
        }
        (*m).rule.match_type_at_pos[(*m).rule.num_mlvo_values as usize] = match_type;
        (*m).rule.mlvo_value_at_pos[(*m).rule.num_mlvo_values as usize] = ident;
        (*m).rule.num_mlvo_values = (*m).rule.num_mlvo_values.wrapping_add(1);
    }
}
unsafe fn matcher_rule_set_mlvo_wildcard(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut match_type: mlvo_match_type,
) {
    unsafe {
        let mut dummy: sval = sval {
            len: 0 as usize,
            start: ::core::ptr::null::<i8>(),
        };
        matcher_rule_set_mlvo_common(m, s, dummy, match_type);
    }
}
unsafe fn matcher_rule_set_mlvo_group(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
    }
}
unsafe fn matcher_rule_set_mlvo(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
    }
}
unsafe fn matcher_rule_set_kccgst(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        if (*m).rule.num_kccgst_values as ::core::ffi::c_int
            >= (*m).mapping.num_kccgst as ::core::ffi::c_int
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            (*m).rule.skip = 1 != 0;
            return;
        }
        (*m).rule.kccgst_value_at_pos[(*m).rule.num_kccgst_values as usize] = ident;
        (*m).rule.num_kccgst_values = (*m).rule.num_kccgst_values.wrapping_add(1);
    }
}
unsafe fn match_group(mut m: *mut matcher, mut group_name: sval, mut to: sval) -> bool {
    unsafe {
        let mut group: *mut group = ::core::ptr::null_mut::<group>();
        let mut element: *mut sval = ::core::ptr::null_mut::<sval>();
        let mut found: bool = 0 != 0;
        if !(*m).groups.item.is_null() {
            group = (*m).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut group;
            while group < (*m).groups.item.offset((*m).groups.size as isize) as *mut group {
                if svaleq((*group).name, group_name) {
                    found = 1 != 0;
                    break;
                } else {
                    group = group.offset(1);
                }
            }
        }
        if !found {
            return 0 != 0;
        }
        if !(*group).elements.item.is_null() {
            element = (*group)
                .elements
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut sval;
            while element
                < (*group)
                    .elements
                    .item
                    .offset((*group).elements.size as isize) as *mut sval
            {
                if svaleq(to, *element) {
                    return 1 != 0;
                }
                element = element.offset(1);
            }
        }
        return 0 != 0;
    }
}
unsafe fn match_value(
    mut m: *mut matcher,
    mut val: sval,
    mut to: sval,
    mut match_type: mlvo_match_type,
    mut wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        match match_type as u32 {
            1 => {
                return wildcard_type as u32 == WILDCARD_MATCH_ALL as ::core::ffi::c_int as u32
                    || to.len != 0;
            }
            2 => return to.len == 0,
            3 => return to.len != 0,
            4 => return 1 != 0,
            5 => return match_group(m, val, to),
            _ => {
                return svaleq(val, to);
            }
        };
    }
}
unsafe fn match_value_and_mark(
    mut m: *mut matcher,
    mut val: sval,
    mut to: *mut matched_sval,
    mut match_type: mlvo_match_type,
    mut wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        let mut matched: bool = match_value(m, val, (*to).sval, match_type, wildcard_type);
        if matched {
            (*to).set_matched((1 != 0) as bool);
        }
        return matched;
    }
}
unsafe fn expand_rmlvo_in_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut value: sval,
    mut layout_idx: xkb_layout_index_t,
    mut expanded: *mut darray_char,
    mut i: *mut usize,
) -> bool {
    unsafe {
        let mut expanded_index: bool = false;
        let mut c2rust_current_block: u64;
        let mut str: *const i8 = value.start;
        let mut mlv: rules_mlvo = MLVO_MODEL;
        let mut idx: xkb_layout_index_t = 0;
        let mut pfx: i8 = 0;
        let mut sfx: i8 = 0;
        let mut expanded_value: *mut matched_sval = ::core::ptr::null_mut::<matched_sval>();
        if *str.offset(*i as isize) as ::core::ffi::c_int == 'i' as i32
            && ((*i).wrapping_add(1 as usize) == value.len
                || (*str.offset((*i).wrapping_add(1 as usize) as isize) as ::core::ffi::c_int
                    == MERGE_OVERRIDE_PREFIX
                    || *str.offset((*i).wrapping_add(1 as usize) as isize) as ::core::ffi::c_int
                        == MERGE_AUGMENT_PREFIX
                    || *str.offset((*i).wrapping_add(1 as usize) as isize) as ::core::ffi::c_int
                        == MERGE_REPLACE_PREFIX))
        {
            if layout_idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                let mut loc: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] {}:{}:{}: Invalid %i in %-expansion: there is no corresponding layout nor variant in the MLVO fields of the rules header.\n",
                    XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION
                        as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc.line,
                    loc.column,
                );
            } else {
                *i = (*i).wrapping_add(1);
                let mut index_str: [i8; 12] = [0; 12];
                let mut count: i32 = crate::xkb::utils::snprintf_c(
                    &raw mut index_str as *mut i8,
                    ::core::mem::size_of::<[i8; 12]>() as usize,
                    format_args!("{}", layout_idx.wrapping_add(1 as xkb_layout_index_t)),
                );
                darray_appends_nul(
                    &mut (*expanded).item,
                    &mut (*expanded).size,
                    &mut (*expanded).alloc,
                    &raw mut index_str as *mut i8 as *const i8,
                    count as u32,
                );
                (*expanded).size = (*expanded).size.wrapping_sub(1);
                return 1 != 0;
            }
        } else {
            sfx = 0 as i8;
            pfx = sfx;
            if *str.offset(*i as isize) as ::core::ffi::c_int == '(' as i32
                || (*str.offset(*i as isize) as ::core::ffi::c_int == MERGE_OVERRIDE_PREFIX
                    || *str.offset(*i as isize) as ::core::ffi::c_int == MERGE_AUGMENT_PREFIX
                    || *str.offset(*i as isize) as ::core::ffi::c_int == MERGE_REPLACE_PREFIX)
                || *str.offset(*i as isize) as ::core::ffi::c_int == '_' as i32
                || *str.offset(*i as isize) as ::core::ffi::c_int == '-' as i32
            {
                pfx = *str.offset(*i as isize);
                if *str.offset(*i as isize) as ::core::ffi::c_int == '(' as i32 {
                    sfx = ')' as i32 as i8;
                }
                *i = (*i).wrapping_add(1);
                if *i >= value.len {
                    c2rust_current_block = 14165246690716487359;
                } else {
                    c2rust_current_block = 17478428563724192186;
                }
            } else {
                c2rust_current_block = 17478428563724192186;
            }
            match c2rust_current_block {
                14165246690716487359 => {}
                _ => {
                    let c2rust_fresh7 = *i;
                    *i = (*i).wrapping_add(1);
                    match *str.offset(c2rust_fresh7 as isize) as ::core::ffi::c_int {
                        109 => {
                            c2rust_current_block = 1495343238628690102;
                            match c2rust_current_block {
                                17806387538889038492 => {
                                    mlv = MLVO_VARIANT;
                                }
                                14869399783518996101 => {
                                    mlv = MLVO_LAYOUT;
                                }
                                _ => {
                                    mlv = MLVO_MODEL;
                                }
                            }
                            idx = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                            expanded_index = 0 != 0;
                            if *i < value.len
                                && *str.offset(*i as isize) as ::core::ffi::c_int == '[' as i32
                            {
                                if mlv as u32 != MLVO_LAYOUT as ::core::ffi::c_int as u32
                                    && mlv as u32 != MLVO_VARIANT as ::core::ffi::c_int as u32
                                {
                                    let mut loc_0: scanner_loc = scanner_token_location(s);
                                    xkb_logf!(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        "[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                                        crate::xkb::utils::CStrDisplay((*s).file_name),
                                        loc_0.line,
                                        loc_0.column,
                                    );
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let mut consumed: ::core::ffi::c_int = extract_layout_index(
                                        str.offset(*i as isize),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1 as ::core::ffi::c_int {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                            idx = layout_idx;
                                            expanded_index = 1 != 0;
                                        }
                                        *i = (*i).wrapping_add(consumed as usize);
                                        c2rust_current_block = 10758786907990354186;
                                    }
                                }
                            } else {
                                c2rust_current_block = 10758786907990354186;
                            }
                            match c2rust_current_block {
                                14165246690716487359 => {}
                                _ => {
                                    if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.offset(c2rust_fresh8 as isize)
                                                as ::core::ffi::c_int
                                                != sfx as ::core::ffi::c_int
                                            {
                                                c2rust_current_block = 14165246690716487359;
                                            } else {
                                                c2rust_current_block = 2122094917359643297;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 2122094917359643297;
                                    }
                                    match c2rust_current_block {
                                        14165246690716487359 => {}
                                        _ => {
                                            expanded_value =
                                                ::core::ptr::null_mut::<matched_sval>();
                                            if mlv as u32
                                                == MLVO_LAYOUT as ::core::ffi::c_int as u32
                                            {
                                                if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                                    if (*m).rmlvo.layouts.size == 1 as darray_size_t
                                                    {
                                                        expanded_value =
                                                            (*m).rmlvo.layouts.item.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as *mut matched_sval;
                                                    }
                                                } else if idx
                                                    < (*m).rmlvo.layouts.size as xkb_layout_index_t
                                                    && (expanded_index as ::core::ffi::c_int != 0
                                                        || (*m).rmlvo.layouts.size
                                                            > 1 as darray_size_t)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .layouts
                                                        .item
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32
                                                == MLVO_VARIANT as ::core::ffi::c_int as u32
                                            {
                                                if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                                    if (*m).rmlvo.variants.size
                                                        == 1 as darray_size_t
                                                    {
                                                        expanded_value =
                                                            (*m).rmlvo.variants.item.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as *mut matched_sval;
                                                    }
                                                } else if idx
                                                    < (*m).rmlvo.variants.size as xkb_layout_index_t
                                                    && (expanded_index as ::core::ffi::c_int != 0
                                                        || (*m).rmlvo.variants.size
                                                            > 1 as darray_size_t)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .variants
                                                        .item
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32
                                                == MLVO_MODEL as ::core::ffi::c_int as u32
                                            {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return 1 != 0;
                                            }
                                            if pfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                darray_appends_nul(
                                                    &mut (*expanded).item,
                                                    &mut (*expanded).size,
                                                    &mut (*expanded).alloc,
                                                    &raw const pfx as *const i8,
                                                    1,
                                                );
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            darray_appends_nul(
                                                &mut (*expanded).item,
                                                &mut (*expanded).size,
                                                &mut (*expanded).alloc,
                                                (*expanded_value).sval.start,
                                                (*expanded_value).sval.len as u32,
                                            );
                                            (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                darray_appends_nul(
                                                    &mut (*expanded).item,
                                                    &mut (*expanded).size,
                                                    &mut (*expanded).alloc,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            (*expanded_value).set_matched((1 != 0) as bool);
                                            return 1 != 0;
                                        }
                                    }
                                }
                            }
                        }
                        108 => {
                            c2rust_current_block = 14869399783518996101;
                            match c2rust_current_block {
                                17806387538889038492 => {
                                    mlv = MLVO_VARIANT;
                                }
                                14869399783518996101 => {
                                    mlv = MLVO_LAYOUT;
                                }
                                _ => {
                                    mlv = MLVO_MODEL;
                                }
                            }
                            idx = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                            expanded_index = 0 != 0;
                            if *i < value.len
                                && *str.offset(*i as isize) as ::core::ffi::c_int == '[' as i32
                            {
                                if mlv as u32 != MLVO_LAYOUT as ::core::ffi::c_int as u32
                                    && mlv as u32 != MLVO_VARIANT as ::core::ffi::c_int as u32
                                {
                                    let mut loc_0: scanner_loc = scanner_token_location(s);
                                    xkb_logf!(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        "[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                                        crate::xkb::utils::CStrDisplay((*s).file_name),
                                        loc_0.line,
                                        loc_0.column,
                                    );
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let mut consumed: ::core::ffi::c_int = extract_layout_index(
                                        str.offset(*i as isize),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1 as ::core::ffi::c_int {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                            idx = layout_idx;
                                            expanded_index = 1 != 0;
                                        }
                                        *i = (*i).wrapping_add(consumed as usize);
                                        c2rust_current_block = 10758786907990354186;
                                    }
                                }
                            } else {
                                c2rust_current_block = 10758786907990354186;
                            }
                            match c2rust_current_block {
                                14165246690716487359 => {}
                                _ => {
                                    if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.offset(c2rust_fresh8 as isize)
                                                as ::core::ffi::c_int
                                                != sfx as ::core::ffi::c_int
                                            {
                                                c2rust_current_block = 14165246690716487359;
                                            } else {
                                                c2rust_current_block = 2122094917359643297;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 2122094917359643297;
                                    }
                                    match c2rust_current_block {
                                        14165246690716487359 => {}
                                        _ => {
                                            expanded_value =
                                                ::core::ptr::null_mut::<matched_sval>();
                                            if mlv as u32
                                                == MLVO_LAYOUT as ::core::ffi::c_int as u32
                                            {
                                                if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                                    if (*m).rmlvo.layouts.size == 1 as darray_size_t
                                                    {
                                                        expanded_value =
                                                            (*m).rmlvo.layouts.item.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as *mut matched_sval;
                                                    }
                                                } else if idx
                                                    < (*m).rmlvo.layouts.size as xkb_layout_index_t
                                                    && (expanded_index as ::core::ffi::c_int != 0
                                                        || (*m).rmlvo.layouts.size
                                                            > 1 as darray_size_t)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .layouts
                                                        .item
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32
                                                == MLVO_VARIANT as ::core::ffi::c_int as u32
                                            {
                                                if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                                    if (*m).rmlvo.variants.size
                                                        == 1 as darray_size_t
                                                    {
                                                        expanded_value =
                                                            (*m).rmlvo.variants.item.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as *mut matched_sval;
                                                    }
                                                } else if idx
                                                    < (*m).rmlvo.variants.size as xkb_layout_index_t
                                                    && (expanded_index as ::core::ffi::c_int != 0
                                                        || (*m).rmlvo.variants.size
                                                            > 1 as darray_size_t)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .variants
                                                        .item
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32
                                                == MLVO_MODEL as ::core::ffi::c_int as u32
                                            {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return 1 != 0;
                                            }
                                            if pfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                darray_appends_nul(
                                                    &mut (*expanded).item,
                                                    &mut (*expanded).size,
                                                    &mut (*expanded).alloc,
                                                    &raw const pfx as *const i8,
                                                    1,
                                                );
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            darray_appends_nul(
                                                &mut (*expanded).item,
                                                &mut (*expanded).size,
                                                &mut (*expanded).alloc,
                                                (*expanded_value).sval.start,
                                                (*expanded_value).sval.len as u32,
                                            );
                                            (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                darray_appends_nul(
                                                    &mut (*expanded).item,
                                                    &mut (*expanded).size,
                                                    &mut (*expanded).alloc,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            (*expanded_value).set_matched((1 != 0) as bool);
                                            return 1 != 0;
                                        }
                                    }
                                }
                            }
                        }
                        118 => {
                            c2rust_current_block = 17806387538889038492;
                            match c2rust_current_block {
                                17806387538889038492 => {
                                    mlv = MLVO_VARIANT;
                                }
                                14869399783518996101 => {
                                    mlv = MLVO_LAYOUT;
                                }
                                _ => {
                                    mlv = MLVO_MODEL;
                                }
                            }
                            idx = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                            expanded_index = 0 != 0;
                            if *i < value.len
                                && *str.offset(*i as isize) as ::core::ffi::c_int == '[' as i32
                            {
                                if mlv as u32 != MLVO_LAYOUT as ::core::ffi::c_int as u32
                                    && mlv as u32 != MLVO_VARIANT as ::core::ffi::c_int as u32
                                {
                                    let mut loc_0: scanner_loc = scanner_token_location(s);
                                    xkb_logf!(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        "[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                                        crate::xkb::utils::CStrDisplay((*s).file_name),
                                        loc_0.line,
                                        loc_0.column,
                                    );
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let mut consumed: ::core::ffi::c_int = extract_layout_index(
                                        str.offset(*i as isize),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1 as ::core::ffi::c_int {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                            idx = layout_idx;
                                            expanded_index = 1 != 0;
                                        }
                                        *i = (*i).wrapping_add(consumed as usize);
                                        c2rust_current_block = 10758786907990354186;
                                    }
                                }
                            } else {
                                c2rust_current_block = 10758786907990354186;
                            }
                            match c2rust_current_block {
                                14165246690716487359 => {}
                                _ => {
                                    if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.offset(c2rust_fresh8 as isize)
                                                as ::core::ffi::c_int
                                                != sfx as ::core::ffi::c_int
                                            {
                                                c2rust_current_block = 14165246690716487359;
                                            } else {
                                                c2rust_current_block = 2122094917359643297;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 2122094917359643297;
                                    }
                                    match c2rust_current_block {
                                        14165246690716487359 => {}
                                        _ => {
                                            expanded_value =
                                                ::core::ptr::null_mut::<matched_sval>();
                                            if mlv as u32
                                                == MLVO_LAYOUT as ::core::ffi::c_int as u32
                                            {
                                                if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                                    if (*m).rmlvo.layouts.size == 1 as darray_size_t
                                                    {
                                                        expanded_value =
                                                            (*m).rmlvo.layouts.item.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as *mut matched_sval;
                                                    }
                                                } else if idx
                                                    < (*m).rmlvo.layouts.size as xkb_layout_index_t
                                                    && (expanded_index as ::core::ffi::c_int != 0
                                                        || (*m).rmlvo.layouts.size
                                                            > 1 as darray_size_t)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .layouts
                                                        .item
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32
                                                == MLVO_VARIANT as ::core::ffi::c_int as u32
                                            {
                                                if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                                    if (*m).rmlvo.variants.size
                                                        == 1 as darray_size_t
                                                    {
                                                        expanded_value =
                                                            (*m).rmlvo.variants.item.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as *mut matched_sval;
                                                    }
                                                } else if idx
                                                    < (*m).rmlvo.variants.size as xkb_layout_index_t
                                                    && (expanded_index as ::core::ffi::c_int != 0
                                                        || (*m).rmlvo.variants.size
                                                            > 1 as darray_size_t)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .variants
                                                        .item
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32
                                                == MLVO_MODEL as ::core::ffi::c_int as u32
                                            {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return 1 != 0;
                                            }
                                            if pfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                darray_appends_nul(
                                                    &mut (*expanded).item,
                                                    &mut (*expanded).size,
                                                    &mut (*expanded).alloc,
                                                    &raw const pfx as *const i8,
                                                    1,
                                                );
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            darray_appends_nul(
                                                &mut (*expanded).item,
                                                &mut (*expanded).size,
                                                &mut (*expanded).alloc,
                                                (*expanded_value).sval.start,
                                                (*expanded_value).sval.len as u32,
                                            );
                                            (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                darray_appends_nul(
                                                    &mut (*expanded).item,
                                                    &mut (*expanded).size,
                                                    &mut (*expanded).alloc,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            (*expanded_value).set_matched((1 != 0) as bool);
                                            return 1 != 0;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let mut loc_1: scanner_loc = scanner_token_location(s);
        xkb_logf!(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
            crate::xkb::utils::CStrDisplay((*s).file_name),
            loc_1.line,
            loc_1.column,
        );
        return 0 != 0;
    }
}
unsafe fn expand_qualifier_in_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut value: sval,
    mut expanded: *mut darray_char,
    mut has_layout_idx_range: bool,
    mut has_separator: bool,
    mut prefix_idx: darray_size_t,
    mut i: *mut usize,
) {
    unsafe {
        let mut str: *const i8 = value.start;
        if ((*i).wrapping_add(3 as usize) <= value.len
            || (*str.offset((*i).wrapping_add(3 as usize) as isize) as ::core::ffi::c_int
                == MERGE_OVERRIDE_PREFIX
                || *str.offset((*i).wrapping_add(3 as usize) as isize) as ::core::ffi::c_int
                    == MERGE_AUGMENT_PREFIX
                || *str.offset((*i).wrapping_add(3 as usize) as isize) as ::core::ffi::c_int
                    == MERGE_REPLACE_PREFIX))
            && *str.offset(*i as isize) as ::core::ffi::c_int == 'a' as i32
            && *str.offset((*i).wrapping_add(1 as usize) as isize) as ::core::ffi::c_int
                == 'l' as i32
            && *str.offset((*i).wrapping_add(2 as usize) as isize) as ::core::ffi::c_int
                == 'l' as i32
        {
            if has_layout_idx_range {
                let mut loc: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                    "{}:{}:{}: Using :all qualifier with indices range is not recommended.\n",
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc.line,
                    loc.column,
                );
            }
            darray_appends_nul(
                &mut (*expanded).item,
                &mut (*expanded).size,
                &mut (*expanded).alloc,
                b"1\0".as_ptr() as *const i8,
                1,
            );
            (*expanded).size = (*expanded).size.wrapping_sub(1);
            if (*m).rmlvo.layouts.size > 1 as darray_size_t {
                let mut layout_index: [i8; 12] = [0; 12];
                let prefix_length: darray_size_t = (*expanded)
                    .size
                    .wrapping_sub(prefix_idx)
                    .wrapping_sub(1 as darray_size_t);
                let mut l: xkb_layout_index_t = 1 as xkb_layout_index_t;
                while l
                    < (if (32 as darray_size_t) < (*m).rmlvo.layouts.size {
                        32 as xkb_layout_index_t
                    } else {
                        (*m).rmlvo.layouts.size as xkb_layout_index_t
                    })
                {
                    if !has_separator {
                        darray_append(
                            &mut (*expanded).item,
                            &mut (*expanded).size,
                            &mut (*expanded).alloc,
                            '+' as i32 as i8,
                        );
                    }
                    {
                        let old_size = (*expanded).size;
                        let new_size = old_size.wrapping_add(prefix_length).wrapping_add(1);
                        darray_growalloc(&mut (*expanded).item, &mut (*expanded).alloc, new_size);
                        (*expanded).size = new_size;
                        std::ptr::copy_nonoverlapping(
                            (*expanded).item.offset(prefix_idx as isize),
                            (*expanded).item.offset(old_size as isize),
                            prefix_length as usize,
                        );
                        *(*expanded).item.offset(new_size.wrapping_sub(1) as isize) = 0;
                        (*expanded).size = (*expanded).size.wrapping_sub(1);
                    }
                    let mut count: i32 = crate::xkb::utils::snprintf_c(
                        &raw mut layout_index as *mut i8,
                        ::core::mem::size_of::<[i8; 12]>() as usize,
                        format_args!("{}", l.wrapping_add(1 as xkb_layout_index_t)),
                    );
                    darray_appends_nul(
                        &mut (*expanded).item,
                        &mut (*expanded).size,
                        &mut (*expanded).alloc,
                        &raw mut layout_index as *mut i8 as *const i8,
                        count as u32,
                    );
                    (*expanded).size = (*expanded).size.wrapping_sub(1);
                    l = l.wrapping_add(1);
                }
            }
            *i = (*i).wrapping_add(3 as usize);
        }
    }
}
#[inline]
unsafe fn concat_kccgst(mut into: *mut darray_char, mut size: darray_size_t, mut from: *const i8) {
    unsafe {
        let from_plus: bool = *from.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == MERGE_OVERRIDE_PREFIX
            || *from.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == MERGE_AUGMENT_PREFIX
            || *from.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == MERGE_REPLACE_PREFIX;
        if from_plus as ::core::ffi::c_int != 0 || (*into).size == 0 as darray_size_t {
            darray_appends_nul(
                &mut (*into).item,
                &mut (*into).size,
                &mut (*into).alloc,
                from,
                size,
            );
            (*into).size = (*into).size.wrapping_sub(1);
        } else {
            let ch: i8 = (if (*into).size == 0 as darray_size_t {
                '\0' as i32
            } else {
                *(*into).item.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            }) as i8;
            let into_plus: bool = ch as ::core::ffi::c_int == MERGE_OVERRIDE_PREFIX
                || ch as ::core::ffi::c_int == MERGE_AUGMENT_PREFIX
                || ch as ::core::ffi::c_int == MERGE_REPLACE_PREFIX;
            if into_plus {
                let old_size = (*into).size;
                let new_size = size.wrapping_add(old_size).wrapping_add(1);
                darray_growalloc(&mut (*into).item, &mut (*into).alloc, new_size);
                (*into).size = new_size;
                std::ptr::copy(
                    (*into).item,
                    (*into).item.offset(size as isize),
                    old_size as usize,
                );
                std::ptr::copy_nonoverlapping(
                    from as *const u8,
                    (*into).item as *mut u8,
                    size as usize,
                );
                *(*into).item.offset(new_size.wrapping_sub(1) as isize) = 0 as i8;
                (*into).size = (*into).size.wrapping_sub(1);
            }
        };
    }
}
unsafe fn append_expanded_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut merge: bool,
    mut to: *mut darray_char,
    mut value: sval,
    mut layout_idx: xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut str: *const i8 = value.start;
        let mut expanded: darray_char = darray_char {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<i8>(),
        };
        let mut last_item_idx: darray_size_t = 0 as darray_size_t;
        let mut has_separator: bool = 0 != 0;
        let mut i: usize = 0 as usize;
        loop {
            if !(i < value.len) {
                c2rust_current_block = 10758786907990354186;
                break;
            }
            match *str.offset(i as isize) as ::core::ffi::c_int {
                58 => {
                    let c2rust_fresh4 = i;
                    i = i.wrapping_add(1);
                    darray_appends_nul(
                        &mut expanded.item,
                        &mut expanded.size,
                        &mut expanded.alloc,
                        str.offset(c2rust_fresh4 as isize),
                        1,
                    );
                    expanded.size = expanded.size.wrapping_sub(1);
                    expand_qualifier_in_kccgst_value(
                        m,
                        s,
                        value,
                        &raw mut expanded,
                        (*m).mapping.has_layout_idx_range,
                        has_separator,
                        last_item_idx,
                        &raw mut i,
                    );
                }
                37 => {
                    i = i.wrapping_add(1);
                    if i >= value.len
                        || !expand_rmlvo_in_kccgst_value(
                            m,
                            s,
                            value,
                            layout_idx,
                            &raw mut expanded,
                            &raw mut i,
                        )
                    {
                        c2rust_current_block = 1032266188497003083;
                        break;
                    }
                }
                MERGE_OVERRIDE_PREFIX | MERGE_AUGMENT_PREFIX | MERGE_REPLACE_PREFIX => {
                    let c2rust_fresh5 = i;
                    i = i.wrapping_add(1);
                    darray_appends_nul(
                        &mut expanded.item,
                        &mut expanded.size,
                        &mut expanded.alloc,
                        str.offset(c2rust_fresh5 as isize),
                        1,
                    );
                    expanded.size = expanded.size.wrapping_sub(1);
                    last_item_idx = expanded.size.wrapping_sub(1 as darray_size_t);
                    has_separator = 1 != 0;
                }
                _ => {
                    let c2rust_fresh6 = i;
                    i = i.wrapping_add(1);
                    darray_appends_nul(
                        &mut expanded.item,
                        &mut expanded.size,
                        &mut expanded.alloc,
                        str.offset(c2rust_fresh6 as isize),
                        1,
                    );
                    expanded.size = expanded.size.wrapping_sub(1);
                }
            }
        }
        match c2rust_current_block {
            1032266188497003083 => {
                crate::xkb::utils::darray_free(
                    &mut expanded.item,
                    &mut expanded.size,
                    &mut expanded.alloc,
                );
                return 0 != 0;
            }
            _ => {
                if merge {
                    if !(expanded.size == 0 as darray_size_t) {
                        concat_kccgst(to, expanded.size, expanded.item);
                    }
                } else if expanded.size > 0 as darray_size_t {
                    darray_appends(
                        &mut (*to).item,
                        &mut (*to).size,
                        &mut (*to).alloc,
                        expanded.item as *const i8,
                        expanded.size,
                    );
                }
                crate::xkb::utils::darray_free(
                    &mut expanded.item,
                    &mut expanded.size,
                    &mut expanded.alloc,
                );
                return 1 != 0;
            }
        };
    }
}
unsafe fn matcher_append_pending_kccgst(mut m: *mut matcher) -> bool {
    unsafe {
        if !(*m).mapping.has_layout_idx_range {
            return 1 != 0;
        }
        let mut i: kccgst_index_t = 0 as kccgst_index_t;
        while (i as ::core::ffi::c_int) < (*m).mapping.num_kccgst as ::core::ffi::c_int {
            let kccgst: rules_kccgst = (*m).mapping.kccgst_at_pos[i as usize];
            let mut layout: xkb_layout_index_t =
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
            while layout < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max {
                let buf: *const kccgst_buffer = &raw mut (*m).pending_kccgst;
                let mut offset: usize = 0 as usize;
                let mut k: darray_size_t = 0 as darray_size_t;
                while k < (*buf).slices.size {
                    let slice: *const kccgst_buffer_slice =
                        (*buf).slices.item.offset(k as isize) as *mut kccgst_buffer_slice;
                    if (*slice).kccgst() as u32 == kccgst as u32
                        && (*slice).layout == layout
                        && (*slice).length() as ::core::ffi::c_int != 0
                    {
                        concat_kccgst(
                            (&raw mut (*m).kccgst as *mut darray_char).offset(kccgst as isize)
                                as *mut darray_char,
                            (*slice).length() as darray_size_t,
                            (*buf).buffer.item.offset(offset as isize),
                        );
                    }
                    offset = offset.wrapping_add((*slice).length() as usize);
                    k = k.wrapping_add(1);
                }
                layout = layout.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*m).mapping.has_layout_idx_range = 0 != 0;
        return 1 != 0;
    }
}
unsafe fn matcher_rule_verify(mut m: *mut matcher, mut s: *mut scanner) {
    unsafe {
        if (*m).rule.num_mlvo_values as ::core::ffi::c_int
            != (*m).mapping.num_mlvo as ::core::ffi::c_int
            || (*m).rule.num_kccgst_values as ::core::ffi::c_int
                != (*m).mapping.num_kccgst as ::core::ffi::c_int
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: invalid rule: must have same number of values as mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            (*m).rule.skip = 1 != 0;
        }
    }
}
unsafe fn matcher_rule_apply_if_matches(mut m: *mut matcher, mut s: *mut scanner) {
    unsafe {
        let mut candidate_layouts: xkb_layout_mask_t =
            (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask;
        let mut idx: xkb_layout_index_t = 0;
        let mut i: mlvo_index_t = 0 as mlvo_index_t;
        while (i as ::core::ffi::c_int) < (*m).mapping.num_mlvo as ::core::ffi::c_int {
            let mut mlvo: rules_mlvo = (*m).mapping.mlvo_at_pos[i as usize];
            let mut value: sval = (*m).rule.mlvo_value_at_pos[i as usize];
            let mut match_type: mlvo_match_type = (*m).rule.match_type_at_pos[i as usize];
            let mut to: *mut matched_sval = ::core::ptr::null_mut::<matched_sval>();
            let mut matched: bool = 0 != 0;
            if mlvo as u32 == MLVO_MODEL as ::core::ffi::c_int as u32 {
                to = &raw mut (*m).rmlvo.model;
                matched = match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_ALL);
            } else if (*m).mapping.has_layout_idx_range {
                idx = (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
                while idx < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max
                    && candidate_layouts != 0
                {
                    let mask: xkb_layout_mask_t = (1 as xkb_layout_mask_t) << idx;
                    if candidate_layouts & mask != 0 {
                        match mlvo as u32 {
                            1 => {
                                to = (*m).rmlvo.layouts.item.offset(idx as isize)
                                    as *mut matched_sval;
                                if match_value_and_mark(
                                    m,
                                    value,
                                    to,
                                    match_type,
                                    WILDCARD_MATCH_NONEMPTY,
                                ) {
                                    matched = 1 != 0;
                                } else {
                                    candidate_layouts &= !mask;
                                }
                            }
                            2 => {
                                to = (*m).rmlvo.variants.item.offset(idx as isize)
                                    as *mut matched_sval;
                                if match_value_and_mark(
                                    m,
                                    value,
                                    to,
                                    match_type,
                                    WILDCARD_MATCH_NONEMPTY,
                                ) {
                                    matched = 1 != 0;
                                } else {
                                    candidate_layouts &= !mask;
                                }
                            }
                            _ => {
                                let mut found_option: bool = 0 != 0;
                                if !(*m).rmlvo.options.item.is_null() {
                                    to = (*m)
                                        .rmlvo
                                        .options
                                        .item
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut matched_sval;
                                    while to
                                        < (*m)
                                            .rmlvo
                                            .options
                                            .item
                                            .offset((*m).rmlvo.options.size as isize)
                                            as *mut matched_sval
                                    {
                                        if !((*to).layout() as ::core::ffi::c_int
                                            != OPTIONS_MATCH_ALL_GROUPS
                                            && (*to).layout() != idx)
                                        {
                                            if match_value_and_mark(
                                                m,
                                                value,
                                                to,
                                                match_type,
                                                WILDCARD_MATCH_ALL,
                                            ) {
                                                matched = 1 != 0;
                                                found_option = 1 != 0;
                                                break;
                                            }
                                        }
                                        to = to.offset(1);
                                    }
                                }
                                if !found_option {
                                    candidate_layouts &= !mask;
                                }
                            }
                        }
                    }
                    idx = idx.wrapping_add(1);
                }
            } else {
                match mlvo as u32 {
                    1 => {
                        to = (*m).rmlvo.layouts.item.offset(
                            (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min as isize,
                        ) as *mut matched_sval;
                        matched =
                            match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_NONEMPTY);
                    }
                    2 => {
                        to = (*m).rmlvo.variants.item.offset(
                            (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min as isize,
                        ) as *mut matched_sval;
                        matched =
                            match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_NONEMPTY);
                    }
                    _ => {
                        if !(*m).rmlvo.options.item.is_null() {
                            to = (*m)
                                .rmlvo
                                .options
                                .item
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut matched_sval;
                            while to
                                < (*m)
                                    .rmlvo
                                    .options
                                    .item
                                    .offset((*m).rmlvo.options.size as isize)
                                    as *mut matched_sval
                            {
                                if !((*to).layout() as ::core::ffi::c_int
                                    != OPTIONS_MATCH_ALL_GROUPS
                                    && (*to).layout()
                                        != (*m)
                                            .mapping
                                            .c2rust_unnamed
                                            .c2rust_unnamed_0
                                            .layout_idx_min)
                                {
                                    matched = match_value_and_mark(
                                        m,
                                        value,
                                        to,
                                        match_type,
                                        WILDCARD_MATCH_ALL,
                                    );
                                    if matched {
                                        break;
                                    }
                                }
                                to = to.offset(1);
                            }
                        }
                    }
                }
            }
            if !matched {
                return;
            }
            i = i.wrapping_add(1);
        }
        if (*m).mapping.has_layout_idx_range {
            idx = (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
            while idx < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max {
                if candidate_layouts & (1 as xkb_layout_mask_t) << idx != 0 {
                    let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
                    while (i_0 as ::core::ffi::c_int)
                        < (*m).mapping.num_kccgst as ::core::ffi::c_int
                    {
                        let kccgst: rules_kccgst = (*m).mapping.kccgst_at_pos[i_0 as usize];
                        let value_0: sval = (*m).rule.kccgst_value_at_pos[i_0 as usize];
                        let buf: *mut kccgst_buffer = &raw mut (*m).pending_kccgst;
                        let prev_buffer_length: darray_size_t = (*buf).buffer.size;
                        append_expanded_kccgst_value(
                            m,
                            s,
                            0 != 0,
                            &raw mut (*buf).buffer,
                            value_0,
                            idx,
                        );
                        let length: u32 =
                            (*buf).buffer.size.wrapping_sub(prev_buffer_length) as u32;
                        let slice: kccgst_buffer_slice = {
                            let mut init = kccgst_buffer_slice {
                                length_kccgst: [0; 4],
                                layout: idx,
                            };
                            init.set_length(length);
                            init.set_kccgst(kccgst);
                            init
                        };
                        darray_append(
                            &mut (*buf).slices.item,
                            &mut (*buf).slices.size,
                            &mut (*buf).slices.alloc,
                            slice,
                        );
                        i_0 = i_0.wrapping_add(1);
                    }
                }
                idx = idx.wrapping_add(1);
            }
        } else {
            let mut i_1: kccgst_index_t = 0 as kccgst_index_t;
            while (i_1 as ::core::ffi::c_int) < (*m).mapping.num_kccgst as ::core::ffi::c_int {
                let mut kccgst_0: rules_kccgst = (*m).mapping.kccgst_at_pos[i_1 as usize];
                let mut value_1: sval = (*m).rule.kccgst_value_at_pos[i_1 as usize];
                append_expanded_kccgst_value(
                    m,
                    s,
                    1 != 0,
                    (&raw mut (*m).kccgst as *mut darray_char).offset(kccgst_0 as isize)
                        as *mut darray_char,
                    value_1,
                    (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min,
                );
                i_1 = i_1.wrapping_add(1);
            }
        }
        if !is_mlvo_mask_defined(m, MLVO_OPTION) {
            (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask &= !candidate_layouts;
        }
    }
}
unsafe fn gettok(mut m: *mut matcher, mut s: *mut scanner) -> rules_token {
    unsafe {
        return lex(s, &raw mut (*m).val);
    }
}
unsafe fn matcher_match(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut include_depth: u32,
    mut string: *const i8,
    mut len: usize,
    mut file_name: *const i8,
) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut tok: rules_token = TOK_END_OF_FILE;
        if m.is_null() {
            return 0 != 0;
        }
        '_initial: loop {
            tok = gettok(m, s);
            match tok as u32 {
                4 => {}
                1 => {
                    continue;
                }
                0 => {
                    c2rust_current_block = 13194772801876125100;
                    break;
                }
                _ => {
                    c2rust_current_block = 2196083827608010402;
                    break;
                }
            }
            loop {
                tok = gettok(m, s);
                match tok as u32 {
                    3 => {
                        matcher_group_start_new(m, (*m).val.string);
                        tok = gettok(m, s);
                        match tok as u32 {
                            5 => {
                                break;
                            }
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                    }
                    10 => {
                        tok = gettok(m, s);
                        match tok as u32 {
                            2 => {}
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                        matcher_include(m, s, include_depth, (*m).val.string);
                        tok = gettok(m, s);
                        match tok as u32 {
                            1 => {
                                continue '_initial;
                            }
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                    }
                    2 => {
                        matcher_mapping_start_new(m);
                        matcher_mapping_set_mlvo(m, s, (*m).val.string);
                        loop {
                            tok = gettok(m, s);
                            match tok as u32 {
                                2 => {}
                                5 => {
                                    break;
                                }
                                _ => {
                                    c2rust_current_block = 2196083827608010402;
                                    break '_initial;
                                }
                            }
                            if (*m).mapping.c2rust_unnamed_0.active != 0 {
                                matcher_mapping_set_mlvo(m, s, (*m).val.string);
                            }
                        }
                        loop {
                            tok = gettok(m, s);
                            match tok as u32 {
                                2 => {}
                                1 => {
                                    break;
                                }
                                _ => {
                                    c2rust_current_block = 2196083827608010402;
                                    break '_initial;
                                }
                            }
                            if (*m).mapping.c2rust_unnamed_0.active != 0 {
                                matcher_mapping_set_kccgst(m, s, (*m).val.string);
                            }
                        }
                        if (*m).mapping.c2rust_unnamed_0.active != 0
                            && matcher_mapping_verify(m, s) as ::core::ffi::c_int != 0
                        {
                            matcher_mapping_set_layout_bounds(m);
                            if (*m).mapping.has_layout_idx_range {
                                (*m).pending_kccgst.buffer.size = 0 as darray_size_t;
                                (*m).pending_kccgst.slices.size = 0 as darray_size_t;
                            }
                        }
                        loop {
                            tok = gettok(m, s);
                            match tok as u32 {
                                4 => {
                                    matcher_append_pending_kccgst(m);
                                    break;
                                }
                                1 => {}
                                0 => {
                                    matcher_append_pending_kccgst(m);
                                    c2rust_current_block = 13194772801876125100;
                                    break '_initial;
                                }
                                _ => {
                                    matcher_rule_start_new(m);
                                    loop {
                                        match tok as u32 {
                                            2 => {
                                                if !(*m).rule.skip {
                                                    if (*m).val.string.len == 1 as usize
                                                        && *(*m).val.string.start.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == '+' as i32
                                                    {
                                                        matcher_rule_set_mlvo_wildcard(
                                                            m,
                                                            s,
                                                            MLVO_MATCH_WILDCARD_SOME,
                                                        );
                                                    } else {
                                                        matcher_rule_set_mlvo(
                                                            m,
                                                            s,
                                                            (*m).val.string,
                                                        );
                                                    }
                                                }
                                            }
                                            6 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_LEGACY,
                                                    );
                                                }
                                            }
                                            7 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_NONE,
                                                    );
                                                }
                                            }
                                            8 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_SOME,
                                                    );
                                                }
                                            }
                                            9 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_ANY,
                                                    );
                                                }
                                            }
                                            3 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_group(
                                                        m,
                                                        s,
                                                        (*m).val.string,
                                                    );
                                                }
                                            }
                                            5 => {
                                                break;
                                            }
                                            _ => {
                                                c2rust_current_block = 2196083827608010402;
                                                break '_initial;
                                            }
                                        }
                                        tok = gettok(m, s);
                                    }
                                    loop {
                                        tok = gettok(m, s);
                                        match tok as u32 {
                                            2 => {}
                                            1 => {
                                                break;
                                            }
                                            _ => {
                                                c2rust_current_block = 2196083827608010402;
                                                break '_initial;
                                            }
                                        }
                                        if !(*m).rule.skip {
                                            matcher_rule_set_kccgst(m, s, (*m).val.string);
                                        }
                                    }
                                    if !(*m).rule.skip {
                                        matcher_rule_verify(m, s);
                                    }
                                    if !(*m).rule.skip {
                                        matcher_rule_apply_if_matches(m, s);
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        c2rust_current_block = 2196083827608010402;
                        break '_initial;
                    }
                }
            }
            loop {
                tok = gettok(m, s);
                match tok as u32 {
                    2 => {}
                    1 => {
                        break;
                    }
                    _ => {
                        c2rust_current_block = 2196083827608010402;
                        break '_initial;
                    }
                }
                matcher_group_add_element(m, s, (*m).val.string);
            }
        }
        match c2rust_current_block {
            13194772801876125100 => return 1 != 0,
            _ => {
                match tok as u32 {
                    11 => {}
                    _ => {
                        let mut loc: scanner_loc = scanner_token_location(s);
                        xkb_logf!(
                            (*s).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "[XKB-{:03}] {}:{}:{}: unexpected token\n",
                            XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                            crate::xkb::utils::CStrDisplay((*s).file_name),
                            loc.line,
                            loc.column,
                        );
                    }
                }
                return 0 != 0;
            }
        };
    }
}
unsafe fn read_rules_file(
    mut ctx: *mut xkb_context,
    mut matcher: *mut matcher,
    mut include_depth: u32,
    mut file: *mut FILE,
    mut path: *const i8,
) -> bool {
    unsafe {
        let mut ret: bool = false;
        let mut scanner: scanner = scanner {
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

        // Convert FILE* to Rust File and map it
        use crate::xkb::utils::MappedFile;
        use std::fs::File;
        use std::os::unix::io::FromRawFd;

        let fd = libc::fileno(file as *mut libc::FILE);
        if fd < 0 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "Invalid file descriptor\n",
            );
            return 0 != 0;
        }

        let rust_file = File::from_raw_fd(fd);
        let mapped = match MappedFile::new(&rust_file) {
            Ok(m) => m,
            Err(e) => {
                let err_msg = std::ffi::CString::new(e.to_string())
                    .unwrap_or_else(|_| std::ffi::CString::new("unknown error").unwrap());
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Couldn't read rules file \"{}\": {}\n",
                    crate::xkb::utils::CStrDisplay(path),
                    crate::xkb::utils::CStrDisplay(err_msg.as_ptr()),
                );
                std::mem::forget(rust_file);
                return 0 != 0;
            }
        };

        scanner_init(
            &raw mut scanner,
            (*matcher).ctx,
            mapped.as_ptr(),
            mapped.len(),
            path,
            std::ptr::null_mut::<core::ffi::c_void>(),
        );
        if !scanner_check_supported_char_encoding(&raw mut scanner) {
            let mut loc: scanner_loc = scanner_token_location(&raw mut scanner);
            xkb_logf!(
                scanner.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(scanner.file_name),
                loc.line,
                loc.column,
            );
            let mut loc_0: scanner_loc = scanner_token_location(&raw mut scanner);
            xkb_logf!(
                scanner.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(scanner.file_name),
                loc_0.line,
                loc_0.column,
            );
            std::mem::forget(rust_file);
            return 0 != 0;
        }
        ret = matcher_match(
            matcher,
            &raw mut scanner,
            include_depth,
            mapped.as_ptr(),
            mapped.len(),
            path,
        );
        std::mem::forget(rust_file);
        return ret;
    }
}
unsafe fn xkb_resolve_partial_rules(
    mut ctx: *mut xkb_context,
    mut path: *mut i8,
    mut path_size: usize,
    mut rules: *const i8,
    mut suffix: *const i8,
    mut matcher: *mut matcher,
) -> bool {
    unsafe {
        let mut partial_rules: [i8; 60] = [0; 60];
        let (_, _trunc) = crate::xkb::utils::snprintf_args(
            &raw mut partial_rules as *mut i8,
            ::core::mem::size_of::<[i8; 60]>() as usize,
            format_args!(
                "{}{}",
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(suffix)
            ),
        );
        if _trunc {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Cannot load XKB rules \"{}{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(suffix),
            );
            return 0 != 0;
        }
        let mut offset: u32 = 0 as u32;
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let len: usize = cstr_len(&raw mut partial_rules as *mut i8) as usize;
        loop {
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                &raw mut partial_rules as *mut i8,
                len,
                FILE_TYPE_RULES,
                path,
                path_size,
                &raw mut offset,
                0 != 0,
            );
            if file.is_null() {
                break;
            }
            let ok: bool = read_rules_file(ctx, matcher, 0 as u32, file, path) as bool;
            fclose(file);
            if !ok {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay(path),
                );
                return 0 != 0;
            }
            offset = offset.wrapping_add(1);
        }
        return 1 != 0;
    }
}
unsafe fn xkb_resolve_rules(
    mut ctx: *mut xkb_context,
    mut rules: *const i8,
    mut matcher: *mut matcher,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut mval: *mut matched_sval = ::core::ptr::null_mut::<matched_sval>();
        let mut ret: bool = 0 != 0;
        let mut offset: u32 = 0 as u32;
        let mut path: [i8; 4096] = [0; 4096];
        let file: *mut FILE = FindFileInXkbPath(
            ctx,
            b"(unknown)\0".as_ptr() as *const i8,
            rules,
            cstr_len(rules),
            FILE_TYPE_RULES,
            &raw mut path as *mut i8,
            ::core::mem::size_of::<[i8; 4096]>() as usize,
            &raw mut offset,
            1 != 0,
        ) as *mut FILE;
        if file.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Cannot load XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(rules),
            );
        } else {
            ret = xkb_resolve_partial_rules(
                ctx,
                &raw mut path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
                rules,
                b".pre\0".as_ptr() as *const i8,
                matcher,
            );
            if ret {
                ret = read_rules_file(ctx, matcher, 0 as u32, file, &raw mut path as *mut i8);
                if !ret {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                        XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(&raw mut path as *mut i8),
                    );
                } else {
                    ret = xkb_resolve_partial_rules(
                        ctx,
                        &raw mut path as *mut i8,
                        ::core::mem::size_of::<[i8; 4096]>() as usize,
                        rules,
                        b".post\0".as_ptr() as *const i8,
                        matcher,
                    );
                    if ret {
                        if (*matcher).kccgst[KCCGST_KEYCODES as ::core::ffi::c_int as usize].size
                            == 0 as darray_size_t
                            || (*matcher).kccgst[KCCGST_TYPES as ::core::ffi::c_int as usize].size
                                == 0 as darray_size_t
                            || (*matcher).kccgst[KCCGST_COMPAT as ::core::ffi::c_int as usize].size
                                == 0 as darray_size_t
                            || (*matcher).kccgst[KCCGST_SYMBOLS as ::core::ffi::c_int as usize].size
                                == 0 as darray_size_t
                        {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                "[XKB-{:03}] No components returned from XKB rules \"{}\"\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                crate::xkb::utils::CStrDisplay(rules),
                            );
                            ret = 0 != 0;
                        } else {
                            (*out).keycodes = (*matcher).kccgst
                                [KCCGST_KEYCODES as ::core::ffi::c_int as usize]
                                .item;
                            if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                                *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                    as *mut darray_size_t) = (*matcher).kccgst
                                    [KCCGST_KEYCODES as ::core::ffi::c_int as usize]
                                    .size;
                            }
                            (*matcher).kccgst[KCCGST_KEYCODES as ::core::ffi::c_int as usize]
                                .item = ::core::ptr::null_mut::<i8>();
                            (*matcher).kccgst[KCCGST_KEYCODES as ::core::ffi::c_int as usize]
                                .size = 0 as darray_size_t;
                            (*matcher).kccgst[KCCGST_KEYCODES as ::core::ffi::c_int as usize]
                                .alloc = 0 as darray_size_t;
                            (*out).types =
                                (*matcher).kccgst[KCCGST_TYPES as ::core::ffi::c_int as usize].item;
                            if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                                *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                    as *mut darray_size_t) = (*matcher).kccgst
                                    [KCCGST_TYPES as ::core::ffi::c_int as usize]
                                    .size;
                            }
                            (*matcher).kccgst[KCCGST_TYPES as ::core::ffi::c_int as usize].item =
                                ::core::ptr::null_mut::<i8>();
                            (*matcher).kccgst[KCCGST_TYPES as ::core::ffi::c_int as usize].size =
                                0 as darray_size_t;
                            (*matcher).kccgst[KCCGST_TYPES as ::core::ffi::c_int as usize].alloc =
                                0 as darray_size_t;
                            (*out).compatibility = (*matcher).kccgst
                                [KCCGST_COMPAT as ::core::ffi::c_int as usize]
                                .item;
                            if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                                *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                    as *mut darray_size_t) = (*matcher).kccgst
                                    [KCCGST_COMPAT as ::core::ffi::c_int as usize]
                                    .size;
                            }
                            (*matcher).kccgst[KCCGST_COMPAT as ::core::ffi::c_int as usize].item =
                                ::core::ptr::null_mut::<i8>();
                            (*matcher).kccgst[KCCGST_COMPAT as ::core::ffi::c_int as usize].size =
                                0 as darray_size_t;
                            (*matcher).kccgst[KCCGST_COMPAT as ::core::ffi::c_int as usize].alloc =
                                0 as darray_size_t;
                            (*out).symbols = (*matcher).kccgst
                                [KCCGST_SYMBOLS as ::core::ffi::c_int as usize]
                                .item;
                            if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                                *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                    as *mut darray_size_t) = (*matcher).kccgst
                                    [KCCGST_SYMBOLS as ::core::ffi::c_int as usize]
                                    .size;
                            }
                            (*matcher).kccgst[KCCGST_SYMBOLS as ::core::ffi::c_int as usize].item =
                                ::core::ptr::null_mut::<i8>();
                            (*matcher).kccgst[KCCGST_SYMBOLS as ::core::ffi::c_int as usize].size =
                                0 as darray_size_t;
                            (*matcher).kccgst[KCCGST_SYMBOLS as ::core::ffi::c_int as usize]
                                .alloc = 0 as darray_size_t;
                            (*out).geometry = (*matcher).kccgst
                                [KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                .item;
                            if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                                *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                    as *mut darray_size_t) = (*matcher).kccgst
                                    [KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                    .size;
                            }
                            (*matcher).kccgst[KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                .item = ::core::ptr::null_mut::<i8>();
                            (*matcher).kccgst[KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                .size = 0 as darray_size_t;
                            (*matcher).kccgst[KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                .alloc = 0 as darray_size_t;
                            mval = &raw mut (*matcher).rmlvo.model;
                            if !(*mval).matched() && (*mval).sval.len > 0 as usize {
                                xkb_logf!(
                                    (*matcher).ctx,
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                    "[XKB-{:03}] Unrecognized RMLVO model \"{}\" was ignored\n",
                                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                    crate::xkb::utils::CStrNDisplay(
                                        (*mval).sval.len as usize,
                                        (*mval).sval.start
                                    ),
                                );
                            }
                            if !(*matcher).rmlvo.layouts.item.is_null() {
                                mval = (*matcher)
                                    .rmlvo
                                    .layouts
                                    .item
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .layouts
                                        .item
                                        .offset((*matcher).rmlvo.layouts.size as isize)
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched() && (*mval).sval.len > 0 as usize {
                                        xkb_logf!(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                            "[XKB-{:03}] Unrecognized RMLVO layout \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                            crate::xkb::utils::CStrNDisplay((*mval).sval.len as usize, (*mval).sval.start),
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*matcher).rmlvo.variants.item.is_null() {
                                mval = (*matcher)
                                    .rmlvo
                                    .variants
                                    .item
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .variants
                                        .item
                                        .offset((*matcher).rmlvo.variants.size as isize)
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched() && (*mval).sval.len > 0 as usize {
                                        xkb_logf!(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                            "[XKB-{:03}] Unrecognized RMLVO variant \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                            crate::xkb::utils::CStrNDisplay((*mval).sval.len as usize, (*mval).sval.start),
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*matcher).rmlvo.options.item.is_null() {
                                mval = (*matcher)
                                    .rmlvo
                                    .options
                                    .item
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .options
                                        .item
                                        .offset((*matcher).rmlvo.options.size as isize)
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched() && (*mval).sval.len > 0 as usize {
                                        xkb_logf!(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                            "[XKB-{:03}] Unrecognized RMLVO option \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                            crate::xkb::utils::CStrNDisplay((*mval).sval.len as usize, (*mval).sval.start),
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*out).symbols.is_null() && !explicit_layouts.is_null() {
                                *explicit_layouts = 1 as xkb_layout_index_t;
                                let mut symbols: *const i8 = (*out).symbols;
                                loop {
                                    symbols = crate::xkb::utils::cstr_chr(symbols, ':' as i32);
                                    if !(!symbols.is_null()
                                        && *symbols.offset(1 as isize) as i32 != '\0' as i32)
                                    {
                                        break;
                                    }
                                    let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                                    symbols = symbols.offset(1);
                                    let count: ::core::ffi::c_int = parse_dec_to_uint32_t(
                                        symbols,
                                        usize::MAX as usize,
                                        &raw mut group,
                                    )
                                        as ::core::ffi::c_int;
                                    if count > 0 as ::core::ffi::c_int
                                        && (*symbols.offset(count as isize) as ::core::ffi::c_int
                                            == '\0' as i32
                                            || (*symbols.offset(count as isize)
                                                as ::core::ffi::c_int
                                                == MERGE_OVERRIDE_PREFIX
                                                || *symbols.offset(count as isize)
                                                    as ::core::ffi::c_int
                                                    == MERGE_AUGMENT_PREFIX
                                                || *symbols.offset(count as isize)
                                                    as ::core::ffi::c_int
                                                    == MERGE_REPLACE_PREFIX))
                                        && group > 0 as xkb_layout_index_t
                                        && group <= XKB_MAX_GROUPS as xkb_layout_index_t
                                    {
                                        *explicit_layouts = if *explicit_layouts > group {
                                            *explicit_layouts
                                        } else {
                                            group
                                        };
                                        symbols = symbols.offset(count as isize);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !file.is_null() {
            fclose(file);
        }
        return ret;
    }
}
pub unsafe fn xkb_components_from_rmlvo_builder(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut rules: *const i8 = (*rmlvo).rules;
        let mut matcher: *mut matcher = matcher_new_from_rmlvo(rmlvo, &raw mut rules);
        if matcher.is_null() {
            return 0 != 0;
        }
        let ret: bool =
            xkb_resolve_rules((*rmlvo).ctx, rules, matcher, out, explicit_layouts) as bool;
        matcher_free(matcher);
        return ret;
    }
}
pub unsafe fn xkb_components_from_rules_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *const xkb_rule_names,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut matcher: *mut matcher = matcher_new_from_names(ctx, rmlvo);
        if matcher.is_null() {
            return 0 != 0;
        }
        let ret: bool =
            xkb_resolve_rules(ctx, (*rmlvo).rules, matcher, out, explicit_layouts) as bool;
        matcher_free(matcher);
        return ret;
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        rules_kccgst_svals = [
            sval {
                len: (::core::mem::size_of::<[i8; 9]>() as usize).wrapping_sub(1 as usize),
                start: b"keycodes\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
                start: b"types\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
                start: b"compat\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 8]>() as usize).wrapping_sub(1 as usize),
                start: b"symbols\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 9]>() as usize).wrapping_sub(1 as usize),
                start: b"geometry\0".as_ptr() as *const i8,
            },
        ];
        rules_mlvo_svals = [
            sval {
                len: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
                start: b"model\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
                start: b"layout\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 8]>() as usize).wrapping_sub(1 as usize),
                start: b"variant\0".as_ptr() as *const i8,
            },
            sval {
                len: (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
                start: b"option\0".as_ptr() as *const i8,
            },
        ];
    }
}
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
