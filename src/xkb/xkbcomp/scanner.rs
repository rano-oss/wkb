use crate::xkb_logf;
pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
}
pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
    pub type __uint64_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
}
pub mod stdint_intn_h {
    pub type i64 = __int64_t;
    use super::types_h::__int64_t;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type u32 = __uint32_t;
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t, __uint8_t};
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
pub mod context_h {
    pub use crate::xkb::context_priv::xkb_atom_intern;
    pub use crate::xkb::shared_types::{
        atom_table, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names, C2Rust_Unnamed,
        C2Rust_Unnamed_0,
    };
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
}
pub mod darray_h {
    pub use crate::xkb::shared_types::darray_size_t;
}
pub mod xkbcommon_h {
    use super::stdint_uintn_h::u32;
    pub use crate::xkb::shared_types::{
        xkb_keysym_t, xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
        XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    };
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
    pub unsafe fn scanner_skip_to_eol(mut s: *mut scanner) {
        unsafe {
            let mut nl: *const i8 = memchr(
                (*s).s.offset((*s).pos as isize) as *const ::core::ffi::c_void,
                '\n' as i32,
                (*s).len.wrapping_sub((*s).pos),
            ) as *const i8;
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
                return false_0 != 0;
            }
            (*s).pos = (*s).pos.wrapping_add(1);
            return true_0 != 0;
        }
    }
    #[inline]
    pub unsafe fn scanner_str(mut s: *mut scanner, mut string: *const i8, mut len: usize) -> bool {
        unsafe {
            if (*s).len.wrapping_sub((*s).pos) < len {
                return false_0 != 0;
            }
            if memcmp(
                (*s).s.offset((*s).pos as isize) as *const ::core::ffi::c_void,
                string as *const ::core::ffi::c_void,
                len,
            ) != 0 as ::core::ffi::c_int
            {
                return false_0 != 0;
            }
            (*s).pos = (*s).pos.wrapping_add(len);
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
    pub unsafe fn scanner_buf_appends_code_point(mut s: *mut scanner, mut c: u32) -> bool {
        unsafe {
            if (*s).buf_pos.wrapping_add(5 as usize)
                <= ::core::mem::size_of::<[i8; 1024]>() as usize
            {
                let mut count: ::core::ffi::c_int = utf32_to_utf8(
                    c,
                    (&raw mut (*s).buf as *mut i8).offset((*s).buf_pos as isize),
                );
                if count == 0 as ::core::ffi::c_int {
                    count = utf32_to_utf8(
                        0xfffd as u32,
                        (&raw mut (*s).buf as *mut i8).offset((*s).buf_pos as isize),
                    );
                }
                if count == 0 as ::core::ffi::c_int {
                    return false_0 != 0;
                }
                (*s).buf_pos = (*s)
                    .buf_pos
                    .wrapping_add((count - 1 as ::core::ffi::c_int) as usize);
                return true_0 != 0;
            } else {
                return false_0 != 0;
            };
        }
    }
    #[inline]
    pub unsafe fn scanner_oct(mut s: *mut scanner, mut out: *mut uint8_t) -> bool {
        unsafe {
            let mut i: uint8_t = 0 as uint8_t;
            let mut c: uint8_t = 0 as uint8_t;
            while scanner_peek(s) as ::core::ffi::c_int >= '0' as i32
                && scanner_peek(s) as ::core::ffi::c_int <= '7' as i32
                && (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
            {
                if (c as ::core::ffi::c_int) < 0o40 as ::core::ffi::c_int {
                    c = (c as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                        + scanner_next(s) as ::core::ffi::c_int
                        - '0' as i32) as uint8_t;
                } else {
                    scanner_next(s);
                    *out = c;
                    return false_0 != 0;
                }
                i = i.wrapping_add(1);
            }
            *out = c;
            return i as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe fn scanner_dec_int64(mut s: *mut scanner, mut out: *mut i64) -> ::core::ffi::c_int {
        unsafe {
            let mut val: uint64_t = 0 as uint64_t;
            let count: ::core::ffi::c_int = parse_dec_to_uint64_t(
                (*s).s.offset((*s).pos as isize),
                (*s).len.wrapping_sub((*s).pos),
                &raw mut val,
            ) as ::core::ffi::c_int;
            if count > 0 as ::core::ffi::c_int {
                if val > INT64_MAX as uint64_t {
                    return -1 as ::core::ffi::c_int;
                }
                (*s).pos = (*s).pos.wrapping_add(count as usize);
                *out = val as i64;
            }
            return count;
        }
    }
    #[inline]
    pub unsafe fn scanner_hex_int64(mut s: *mut scanner, mut out: *mut i64) -> ::core::ffi::c_int {
        unsafe {
            let mut val: uint64_t = 0 as uint64_t;
            let count: ::core::ffi::c_int = parse_hex_to_uint64_t(
                (*s).s.offset((*s).pos as isize),
                (*s).len.wrapping_sub((*s).pos),
                &raw mut val,
            ) as ::core::ffi::c_int;
            if count > 0 as ::core::ffi::c_int {
                if val > INT64_MAX as uint64_t {
                    return -1 as ::core::ffi::c_int;
                }
                (*s).pos = (*s).pos.wrapping_add(count as usize);
                *out = val as i64;
            }
            return count;
        }
    }
    #[inline]
    pub unsafe fn scanner_unicode_code_point(mut s: *mut scanner, mut out: *mut u32) -> bool {
        unsafe {
            if !scanner_chr(s, '{' as i32 as i8) {
                return false_0 != 0;
            }
            let mut cp: u32 = 0 as u32;
            let count: ::core::ffi::c_int = parse_hex_to_uint32_t(
                (*s).s.offset((*s).pos as isize),
                (*s).len.wrapping_sub((*s).pos),
                &raw mut cp,
            ) as ::core::ffi::c_int;
            if count > 0 as ::core::ffi::c_int {
                (*s).pos = (*s).pos.wrapping_add(count as usize);
            }
            let last_valid: usize = (*s).pos;
            while !scanner_eof(s)
                && !scanner_eol(s)
                && scanner_peek(s) as ::core::ffi::c_int != '"' as i32
                && scanner_peek(s) as ::core::ffi::c_int != '}' as i32
            {
                scanner_next(s);
            }
            if scanner_chr(s, '}' as i32 as i8) {
                *out = cp;
                return count > 0 as ::core::ffi::c_int
                    && (*s).pos == last_valid.wrapping_add(1 as usize)
                    && cp <= 0x10ffff as u32;
            }
            (*s).pos = last_valid;
            return false_0 != 0;
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
                return true_0 != 0;
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
                    b"[XKB-%03d] %s:%zu:%zu: unexpected NULL character.\n\0".as_ptr() as *const i8,
                    XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                    (*scanner).file_name,
                    loc.line,
                    loc.column,
                );
                return false_0 != 0;
            }
            if !is_ascii(*(*scanner).s.offset(0 as ::core::ffi::c_int as isize)) {
                let mut loc_0: scanner_loc = scanner_token_location(scanner);
                xkb_logf!(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: unexpected non-ASCII character.\n\0".as_ptr()
                        as *const i8,
                    XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                    (*scanner).file_name,
                    loc_0.line,
                    loc_0.column,
                );
                return false_0 != 0;
            }
            return true_0 != 0;
        }
    }

    use super::context_h::xkb_context;
    use super::messages_codes_h::{XKB_ERROR_INVALID_FILE_ENCODING, XKB_LOG_VERBOSITY_MINIMAL};
    use super::stdbool_h::{false_0, true_0};
    use super::stdint_h::INT64_MAX;
    use super::stdint_intn_h::i64;
    use super::stdint_uintn_h::{u32, uint64_t, uint8_t};
    use super::string_h::{memchr, memcmp};
    use super::utf8_h::utf32_to_utf8;
    use super::utils_h::is_ascii;
    use super::utils_numbers_h::{
        parse_dec_to_uint64_t, parse_hex_to_uint32_t, parse_hex_to_uint64_t,
    };
    use super::xkbcommon_h::XKB_LOG_LEVEL_ERROR;
    use crate::xkb_logf;
    pub unsafe fn scanner_token_location(s: *mut scanner) -> scanner_loc {
        unsafe {
            core::mem::transmute(crate::xkb::scanner_utils::scanner_token_location(
                s as *mut crate::xkb::scanner_utils::scanner_utils_h::scanner,
            ))
        }
    }
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    pub type C2Rust_Unnamed_1 = DarrayKeysym;
}
pub mod parser_h {
    pub type yytokentype = ::core::ffi::c_int;
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
    use super::ast_h::{
        merge_mode, xkb_file_type, xkb_map_flags, ExprDef, GroupCompatDef, InterpDef, KeyAliasDef,
        KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef, ParseCommon, SymbolsDef,
        UnknownStatement, VModDef, VarDef, XkbFile,
    };
    use super::atom_h::xkb_atom_t;
    use super::scanner_utils_h::sval;
    use super::stdint_intn_h::i64;
    use super::stdint_uintn_h::u32;
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod string_h {

    extern "C" {
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: usize,
        ) -> ::core::ffi::c_int;
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strdup(__s: *const i8) -> *mut i8;
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut i8;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn is_valid_char(mut cp: u32) -> bool {
        unsafe {
            return cp != 0 as u32;
        }
    }
    #[inline]
    pub unsafe fn is_ascii(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int & !(0x7f as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe fn is_space(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int == ' ' as i32
                || ch as ::core::ffi::c_int >= '\t' as i32
                    && ch as ::core::ffi::c_int <= '\r' as i32;
        }
    }
    #[inline]
    pub unsafe fn is_alpha(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= 'a' as i32
                && ch as ::core::ffi::c_int <= 'z' as i32
                || ch as ::core::ffi::c_int >= 'A' as i32
                    && ch as ::core::ffi::c_int <= 'Z' as i32;
        }
    }
    #[inline]
    pub unsafe fn is_digit(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= '0' as i32
                && ch as ::core::ffi::c_int <= '9' as i32;
        }
    }
    #[inline]
    pub unsafe fn is_alnum(mut ch: i8) -> bool {
        unsafe {
            return is_alpha(ch) as ::core::ffi::c_int != 0
                || is_digit(ch) as ::core::ffi::c_int != 0;
        }
    }
    #[inline]
    pub unsafe fn is_xdigit(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= '0' as i32
                && ch as ::core::ffi::c_int <= '9' as i32
                || ch as ::core::ffi::c_int >= 'a' as i32
                    && ch as ::core::ffi::c_int <= 'f' as i32
                || ch as ::core::ffi::c_int >= 'A' as i32
                    && ch as ::core::ffi::c_int <= 'F' as i32;
        }
    }
    #[inline]
    pub unsafe fn is_graph(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= '!' as i32
                && ch as ::core::ffi::c_int <= '~' as i32;
        }
    }

    use super::stdint_uintn_h::u32;

    // map_file/unmap_file removed - use crate::xkb::utils::MappedFile instead
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe fn parse_dec_to_uint64_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut uint64_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: uint64_t = 0 as uint64_t;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as u32)
                    < 10 as u32
                && result <= (18446744073709551615 as uint64_t).wrapping_div(10 as uint64_t)
                && result.wrapping_mul(10 as uint64_t)
                    <= (18446744073709551615 as uint64_t).wrapping_sub(
                        (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                            as ::core::ffi::c_uchar as uint64_t,
                    )
            {
                result = result.wrapping_mul(10 as uint64_t).wrapping_add(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as uint64_t,
                );
                i = i.wrapping_add(1);
            }
            *out = result as uint64_t;
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
    pub static mut digits__: [::core::ffi::c_uchar; 256] = [
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    #[inline]
    pub unsafe fn parse_hex_to_uint32_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut u32,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0 as usize;
            while i < len
                && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32)
                    < 16 as u32
                && result <= 4294967295 as u32 >> 4 as ::core::ffi::c_int
            {
                result = result.wrapping_mul(16 as u32).wrapping_add(
                    digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32,
                );
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len || !is_xdigit(*s.offset(i as isize)) {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
    #[inline]
    pub unsafe fn parse_hex_to_uint64_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut uint64_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: uint64_t = 0 as uint64_t;
            let mut i: usize = 0 as usize;
            while i < len
                && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32)
                    < 16 as u32
                && result <= 18446744073709551615 as uint64_t >> 4 as ::core::ffi::c_int
            {
                result = result.wrapping_mul(16 as uint64_t).wrapping_add(
                    digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as uint64_t,
                );
                i = i.wrapping_add(1);
            }
            *out = result as uint64_t;
            return if i >= len || !is_xdigit(*s.offset(i as isize)) {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }

    use super::stdint_uintn_h::{u32, uint64_t};
    use super::utils_h::is_xdigit;
}
pub mod utf8_h {
    use super::stdint_uintn_h::u32;

    /// Native Rust UTF-32 to UTF-8 conversion (replaces C FFI)
    ///
    /// Writes UTF-8 bytes to buffer (including null terminator).
    /// Returns total bytes written (including null), or 0 on failure.
    #[inline]
    pub fn utf32_to_utf8(unichar: u32, buffer: *mut i8) -> ::core::ffi::c_int {
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
            (utf8_bytes.len() + 1) as ::core::ffi::c_int
        }
    }
}
pub mod parser_priv_h {

    // Re-export parse functions from parser module
    pub use super::super::parser::{parse, parse_next};

    pub use crate::xkb::xkbcomp::keywords::keyword_to_token;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const INT64_MAX: i64 = 9223372036854775807 as i64;
}
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::ast_h::{
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
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_atom_intern, xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
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
use self::parser_priv_h::{keyword_to_token, parse, parse_next};
pub use self::scanner_utils_h::{
    scanner, scanner_buf_append, scanner_buf_appends_code_point,
    scanner_check_supported_char_encoding, scanner_chr, scanner_dec_int64, scanner_eof,
    scanner_eol, scanner_hex_int64, scanner_init, scanner_loc, scanner_next, scanner_oct,
    scanner_peek, scanner_skip_to_eol, scanner_str, scanner_token_location,
    scanner_unicode_code_point, sval,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::INT64_MAX;
pub use self::stdint_intn_h::i64;
pub use self::stdint_uintn_h::{u32, uint64_t, uint8_t};
use self::string_h::strdup;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__int64_t, __off64_t, __off_t, __uint32_t, __uint64_t, __uint8_t};
pub use self::utils_h::{
    is_alnum, is_alpha, is_ascii, is_digit, is_graph, is_space, is_valid_char, is_xdigit,
};
pub use self::utils_numbers_h::{
    digits__, parse_dec_to_uint64_t, parse_hex_to_uint32_t, parse_hex_to_uint64_t,
};
pub use self::xkbcommon_h::{
    xkb_keysym_t, xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
    XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
pub static mut DECIMAL_SEPARATOR: i8 = '.' as i32 as i8;
unsafe fn number(
    mut s: *mut scanner,
    mut out: *mut i64,
    mut out_tok: *mut ::core::ffi::c_int,
) -> bool {
    unsafe {
        if scanner_str(
            s,
            b"0x\0".as_ptr() as *const i8,
            (::core::mem::size_of::<[i8; 3]>() as usize).wrapping_sub(1 as usize),
        ) {
            match scanner_hex_int64(s, out) {
                -1 => {
                    *out_tok = ERROR_TOK as ::core::ffi::c_int;
                    return true_0 != 0;
                }
                0 => return false_0 != 0,
                _ => {
                    *out_tok = INTEGER as ::core::ffi::c_int;
                    return true_0 != 0;
                }
            }
        } else {
            let mut is_digit_0: bool = false_0 != 0;
            match scanner_dec_int64(s, out) {
                -1 => {
                    *out_tok = ERROR_TOK as ::core::ffi::c_int;
                    return true_0 != 0;
                }
                0 => return false_0 != 0,
                1 => {
                    is_digit_0 = true_0 != 0;
                }
                _ => {}
            }
            if scanner_chr(s, DECIMAL_SEPARATOR) {
                let mut dec: i64 = 0;
                if scanner_dec_int64(s, &raw mut dec) < 0 as ::core::ffi::c_int {
                    *out_tok = ERROR_TOK as ::core::ffi::c_int;
                    return true_0 != 0;
                }
                *out_tok = FLOAT as ::core::ffi::c_int;
            } else if is_digit_0 {
                *out_tok = DECIMAL_DIGIT as ::core::ffi::c_int;
            } else {
                *out_tok = INTEGER as ::core::ffi::c_int;
            }
            return true_0 != 0;
        };
    }
}
pub unsafe fn _xkbcommon_lex(mut yylval: *mut YYSTYPE, mut s: *mut scanner) -> ::core::ffi::c_int {
    unsafe {
        loop {
            while is_space(scanner_peek(s)) {
                scanner_next(s);
            }
            if scanner_str(
                s,
                b"\xE2\x80\x8E\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
            ) as ::core::ffi::c_int
                != 0
                || scanner_str(
                    s,
                    b"\xE2\x80\x8F\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
                ) as ::core::ffi::c_int
                    != 0
            {
                continue;
            }
            if !(scanner_str(
                s,
                b"//\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 3]>() as usize).wrapping_sub(1 as usize),
            ) as ::core::ffi::c_int
                != 0
                || scanner_chr(s, '#' as i32 as i8) as ::core::ffi::c_int != 0)
            {
                break;
            }
            scanner_skip_to_eol(s);
        }
        if scanner_eof(s) {
            return END_OF_FILE as ::core::ffi::c_int;
        }
        (*s).token_pos = (*s).pos;
        (*s).buf_pos = 0 as usize;
        if scanner_chr(s, '"' as i32 as i8) {
            while !scanner_eof(s)
                && !scanner_eol(s)
                && scanner_peek(s) as ::core::ffi::c_int != '"' as i32
            {
                if scanner_chr(s, '\\' as i32 as i8) {
                    let mut o: uint8_t = 0;
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
                        if scanner_unicode_code_point(s, &raw mut cp) as ::core::ffi::c_int != 0
                            && is_valid_char(cp) as ::core::ffi::c_int != 0
                        {
                            scanner_buf_appends_code_point(s, cp);
                        } else {
                            let mut loc: scanner_loc = scanner_token_location(s);
                            xkb_logf!(
                                (*s).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] %s:%zu:%zu: invalid Unicode escape sequence \"%.*s\" in string literal\n\0"
                                    .as_ptr() as *const i8,
                                XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE
                                    as ::core::ffi::c_int,
                                (*s).file_name,
                                loc.line,
                                loc.column,
                                (*s).pos.wrapping_sub(start_pos).wrapping_add(1 as usize)
                                    as ::core::ffi::c_int,
                                (*s).s.offset(start_pos.wrapping_sub(1 as usize) as isize)
                                    as *const i8,
                            );
                        }
                    } else if scanner_oct(s, &raw mut o) as ::core::ffi::c_int != 0
                        && is_valid_char(o as u32) as ::core::ffi::c_int != 0
                    {
                        scanner_buf_append(s, o as i8);
                    } else if (*s).pos > start_pos {
                        let mut loc_0: scanner_loc = scanner_token_location(s);
                        xkb_logf!(
                            (*s).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s:%zu:%zu: invalid octal escape sequence \"%.*s\" in string literal\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_INVALID_ESCAPE_SEQUENCE as ::core::ffi::c_int,
                            (*s).file_name,
                            loc_0.line,
                            loc_0.column,
                            (*s).pos.wrapping_sub(start_pos).wrapping_add(1 as usize)
                                as ::core::ffi::c_int,
                            (*s).s.offset(start_pos.wrapping_sub(1 as usize) as isize)
                                as *const i8,
                        );
                    } else {
                        let mut loc_1: scanner_loc = scanner_token_location(s);
                        xkb_logf!(
                            (*s).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s:%zu:%zu: unknown escape sequence \"\\%c\" in string literal\n\0"
                                .as_ptr() as *const i8,
                            XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE
                                as ::core::ffi::c_int,
                            (*s).file_name,
                            loc_1.line,
                            loc_1.column,
                            scanner_peek(s) as ::core::ffi::c_int,
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
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"%s:%zu:%zu: unterminated string literal\n\0".as_ptr() as *const i8,
                    (*s).file_name,
                    loc_2.line,
                    loc_2.column,
                );
                return ERROR_TOK as ::core::ffi::c_int;
            }
            (*yylval).str = strdup(&raw mut (*s).buf as *mut i8);
            if (*yylval).str.is_null() {
                return ERROR_TOK as ::core::ffi::c_int;
            }
            return STRING as ::core::ffi::c_int;
        }
        if scanner_chr(s, '<' as i32 as i8) {
            while is_graph(scanner_peek(s)) as ::core::ffi::c_int != 0
                && scanner_peek(s) as ::core::ffi::c_int != '>' as i32
            {
                scanner_next(s);
            }
            if !scanner_chr(s, '>' as i32 as i8) {
                let mut loc_3: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"%s:%zu:%zu: unterminated key name literal\n\0".as_ptr() as *const i8,
                    (*s).file_name,
                    loc_3.line,
                    loc_3.column,
                );
                return ERROR_TOK as ::core::ffi::c_int;
            }
            let mut start: *const i8 = (*s)
                .s
                .offset((*s).token_pos as isize)
                .offset(1 as ::core::ffi::c_int as isize);
            let len: usize = (*s)
                .pos
                .wrapping_sub((*s).token_pos)
                .wrapping_sub(2 as usize);
            (*yylval).atom = xkb_atom_intern((*s).ctx, start, len);
            return KEYNAME as ::core::ffi::c_int;
        }
        if scanner_chr(s, ';' as i32 as i8) {
            return SEMI as ::core::ffi::c_int;
        }
        if scanner_chr(s, '{' as i32 as i8) {
            return OBRACE as ::core::ffi::c_int;
        }
        if scanner_chr(s, '}' as i32 as i8) {
            return CBRACE as ::core::ffi::c_int;
        }
        if scanner_chr(s, '=' as i32 as i8) {
            return EQUALS as ::core::ffi::c_int;
        }
        if scanner_chr(s, '[' as i32 as i8) {
            return OBRACKET as ::core::ffi::c_int;
        }
        if scanner_chr(s, ']' as i32 as i8) {
            return CBRACKET as ::core::ffi::c_int;
        }
        if scanner_chr(s, '(' as i32 as i8) {
            return OPAREN as ::core::ffi::c_int;
        }
        if scanner_chr(s, ')' as i32 as i8) {
            return CPAREN as ::core::ffi::c_int;
        }
        if scanner_chr(s, '.' as i32 as i8) {
            return DOT as ::core::ffi::c_int;
        }
        if scanner_chr(s, ',' as i32 as i8) {
            return COMMA as ::core::ffi::c_int;
        }
        if scanner_chr(s, '+' as i32 as i8) {
            return PLUS as ::core::ffi::c_int;
        }
        if scanner_chr(s, '-' as i32 as i8) {
            return MINUS as ::core::ffi::c_int;
        }
        if scanner_chr(s, '*' as i32 as i8) {
            return TIMES as ::core::ffi::c_int;
        }
        if scanner_chr(s, '/' as i32 as i8) {
            return DIVIDE as ::core::ffi::c_int;
        }
        if scanner_chr(s, '!' as i32 as i8) {
            return EXCLAM as ::core::ffi::c_int;
        }
        if scanner_chr(s, '~' as i32 as i8) {
            return INVERT as ::core::ffi::c_int;
        }
        let mut tok: ::core::ffi::c_int = ERROR_TOK as ::core::ffi::c_int;
        if is_alpha(scanner_peek(s)) as ::core::ffi::c_int != 0
            || scanner_peek(s) as ::core::ffi::c_int == '_' as i32
        {
            while is_alnum(scanner_peek(s)) as ::core::ffi::c_int != 0
                || scanner_peek(s) as ::core::ffi::c_int == '_' as i32
            {
                scanner_next(s);
            }
            let mut start_0: *const i8 = (*s).s.offset((*s).token_pos as isize);
            let len_0: usize = (*s).pos.wrapping_sub((*s).token_pos);
            tok = keyword_to_token(start_0, len_0);
            if tok >= 0 as ::core::ffi::c_int {
                return tok;
            }
            (*yylval).sval = sval {
                len: len_0,
                start: start_0,
            };
            return IDENT as ::core::ffi::c_int;
        }
        if number(s, &raw mut (*yylval).num, &raw mut tok) {
            if tok == ERROR_TOK as ::core::ffi::c_int {
                let mut loc_4: scanner_loc = scanner_token_location(s);
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: malformed number literal\n\0".as_ptr() as *const i8,
                    XKB_ERROR_MALFORMED_NUMBER_LITERAL as ::core::ffi::c_int,
                    (*s).file_name,
                    loc_4.line,
                    loc_4.column,
                );
                return ERROR_TOK as ::core::ffi::c_int;
            }
            return tok;
        }
        let mut loc_5: scanner_loc = scanner_token_location(s);
        xkb_logf!(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"%s:%zu:%zu: unrecognized token\n\0".as_ptr() as *const i8,
            (*s).file_name,
            loc_5.line,
            loc_5.column,
        );
        return ERROR_TOK as ::core::ffi::c_int;
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
        scanner_init(scanner, ctx, string, len, file_name, NULL);
        if !scanner_check_supported_char_encoding(scanner) {
            let mut loc: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                (*scanner).file_name,
                loc.line,
                loc.column,
            );
            let mut loc_0: scanner_loc = scanner_token_location(scanner);
            xkb_logf!(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                (*scanner).file_name,
                loc_0.line,
                loc_0.column,
            );
            return false_0 != 0;
        }
        return true_0 != 0;
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
        if !XkbParseStringInit(ctx, &raw mut scanner, string, len, file_name, map) {
            return ::core::ptr::null_mut::<XkbFile>();
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
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Invalid file descriptor\n\0".as_ptr() as *const i8,
            );
            return ::core::ptr::null_mut::<XkbFile>();
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
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Couldn't read XKB file %s: %s\n\0".as_ptr() as *const i8,
                    file_name,
                    err_msg.as_ptr(),
                );
                std::mem::forget(rust_file); // Don't close fd - caller owns FILE*
                return ::core::ptr::null_mut::<XkbFile>();
            }
        };

        let xkb_file = XkbParseString(ctx, mapped.as_ptr(), mapped.len(), file_name, map);

        // Keep file descriptor open - don't close it
        std::mem::forget(rust_file);

        return xkb_file;
    }
}
