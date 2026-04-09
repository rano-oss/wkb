use c2rust_bitfields;
pub mod internal {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}
pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
    pub type __ssize_t = ::core::ffi::c_long;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}

pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
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
        pub _unused2: [::core::ffi::c_char; 8],
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
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn vsnprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}
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
pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_context {
        pub refcnt: ::core::ffi::c_int,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: ::core::ffi::c_int,
        pub user_data: *mut ::core::ffi::c_void,
        pub names_dflt: xkb_rule_names,
        pub includes: C2Rust_Unnamed_0,
        pub failed_includes: C2Rust_Unnamed,
        pub atom_table: *mut atom_table,
        pub x11_atom_cache: *mut ::core::ffi::c_void,
        pub text_buffer: [::core::ffi::c_char; 2048],
        pub text_next: usize,
        #[bitfield(name = "use_environment_names", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "use_secure_getenv", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "pending_default_includes", ty = "bool", bits = "2..=2")]
        pub use_environment_names_use_secure_getenv_pending_default_includes: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }

    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::rmlvo_h::RMLVO;
    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
        pub fn xkb_context_sanitize_rule_names(
            ctx: *mut xkb_context,
            rmlvo: *mut xkb_rule_names,
        ) -> RMLVO;
    }
}
pub mod atom_h {
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_char {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut ::core::ffi::c_char,
    }
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if (need as usize)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as usize)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, usize)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }

    use super::assert_h::__assert_fail;
}
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const ::core::ffi::c_char,
        pub model: *const ::core::ffi::c_char,
        pub layout: *const ::core::ffi::c_char,
        pub variant: *const ::core::ffi::c_char,
        pub options: *const ::core::ffi::c_char,
    }
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_layout_index_t = u32;
    pub type xkb_layout_mask_t = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_component_names {
        pub keycodes: *mut ::core::ffi::c_char,
        pub compatibility: *mut ::core::ffi::c_char,
        pub geometry: *mut ::core::ffi::c_char,
        pub symbols: *mut ::core::ffi::c_char,
        pub types: *mut ::core::ffi::c_char,
    }
    pub const XKB_LAYOUT_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    use super::stdint_uintn_h::u32;
}
pub mod rmlvo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder {
        pub rules: *mut ::core::ffi::c_char,
        pub model: *mut ::core::ffi::c_char,
        pub layouts: xkb_rmlvo_builder_layouts,
        pub options: xkb_rmlvo_builder_options,
        pub refcnt: ::core::ffi::c_int,
        pub ctx: *mut xkb_context,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_options {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_rmlvo_builder_option,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_option {
        pub option: *mut ::core::ffi::c_char,
        pub layout: xkb_layout_index_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_layouts {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_rmlvo_builder_layout,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_layout {
        pub layout: *mut ::core::ffi::c_char,
        pub variant: *mut ::core::ffi::c_char,
    }
    pub type RMLVO = ::core::ffi::c_uint;
    pub const RMLVO_OPTIONS: RMLVO = 16;
    pub const RMLVO_VARIANT: RMLVO = 8;
    pub const RMLVO_LAYOUT: RMLVO = 4;
    pub const RMLVO_MODEL: RMLVO = 2;
    pub const RMLVO_RULES: RMLVO = 1;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::xkbcommon_h::xkb_layout_index_t;
}
pub mod ast_h {
    pub type xkb_file_type = ::core::ffi::c_uint;
    pub const FILE_TYPE_INVALID: xkb_file_type = 7;
    pub const _FILE_TYPE_NUM_ENTRIES: xkb_file_type = 7;
    pub const FILE_TYPE_RULES: xkb_file_type = 6;
    pub const FILE_TYPE_KEYMAP: xkb_file_type = 5;
    pub const FILE_TYPE_GEOMETRY: xkb_file_type = 4;
    pub const LAST_KEYMAP_FILE_TYPE: xkb_file_type = 3;
    pub const FIRST_KEYMAP_FILE_TYPE: xkb_file_type = 0;
    pub const FILE_TYPE_SYMBOLS: xkb_file_type = 3;
    pub const FILE_TYPE_COMPAT: xkb_file_type = 2;
    pub const FILE_TYPE_TYPES: xkb_file_type = 1;
    pub const FILE_TYPE_KEYCODES: xkb_file_type = 0;
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
    pub type xkb_message_code = ::core::ffi::c_uint;
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
        pub start: *const ::core::ffi::c_char,
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
        pub s: *const ::core::ffi::c_char,
        pub buf: [::core::ffi::c_char; 1024],
        pub buf_pos: usize,
        pub token_pos: usize,
        pub cached_pos: usize,
        pub cached_loc: scanner_loc,
        pub file_name: *const ::core::ffi::c_char,
        pub ctx: *mut xkb_context,
        pub priv_0: *mut ::core::ffi::c_void,
    }
    #[inline]
    pub unsafe extern "C" fn svaleq(mut s1: sval, mut s2: sval) -> bool {
        unsafe {
            return s1.len == s2.len
                && memcmp(
                    s1.start as *const ::core::ffi::c_void,
                    s2.start as *const ::core::ffi::c_void,
                    s1.len,
                ) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn svaleq_prefix(mut s1: sval, mut s2: sval) -> bool {
        unsafe {
            return s1.len <= s2.len
                && memcmp(
                    s1.start as *const ::core::ffi::c_void,
                    s2.start as *const ::core::ffi::c_void,
                    s1.len,
                ) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_init(
        mut s: *mut scanner,
        mut ctx: *mut xkb_context,
        mut string: *const ::core::ffi::c_char,
        mut len: usize,
        mut file_name: *const ::core::ffi::c_char,
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
    pub unsafe extern "C" fn scanner_peek(mut s: *mut scanner) -> ::core::ffi::c_char {
        unsafe {
            if ((*s).pos >= (*s).len) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                return '\0' as i32 as ::core::ffi::c_char;
            }
            return *(*s).s.offset((*s).pos as isize);
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_eof(mut s: *mut scanner) -> bool {
        unsafe {
            return (*s).pos >= (*s).len;
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_eol(mut s: *mut scanner) -> bool {
        unsafe {
            return scanner_peek(s) as ::core::ffi::c_int == '\n' as i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_skip_to_eol(mut s: *mut scanner) {
        unsafe {
            let mut nl: *const ::core::ffi::c_char = memchr(
                (*s).s.offset((*s).pos as isize) as *const ::core::ffi::c_void,
                '\n' as i32,
                (*s).len.wrapping_sub((*s).pos),
            ) as *const ::core::ffi::c_char;
            let new_pos: usize = if !nl.is_null() {
                nl.offset_from((*s).s) as ::core::ffi::c_long as usize
            } else {
                (*s).len
            };
            (*s).pos = new_pos;
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_next(mut s: *mut scanner) -> ::core::ffi::c_char {
        unsafe {
            if scanner_eof(s) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                return '\0' as i32 as ::core::ffi::c_char;
            }
            let c2rust_fresh0 = (*s).pos;
            (*s).pos = (*s).pos.wrapping_add(1);
            return *(*s).s.offset(c2rust_fresh0 as isize);
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_chr(mut s: *mut scanner, mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            if (scanner_peek(s) as ::core::ffi::c_int != ch as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                return false_0 != 0;
            }
            (*s).pos = (*s).pos.wrapping_add(1);
            return true_0 != 0;
        }
    }
    #[inline]
    pub unsafe extern "C" fn scanner_str(
        mut s: *mut scanner,
        mut string: *const ::core::ffi::c_char,
        mut len: usize,
    ) -> bool {
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
    pub unsafe extern "C" fn scanner_check_supported_char_encoding(
        mut scanner: *mut scanner,
    ) -> bool {
        unsafe {
            if scanner_str(
                scanner,
                b"\xEF\xBB\xBF\0".as_ptr() as *const ::core::ffi::c_char,
                3 as usize,
            ) as ::core::ffi::c_int
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
                xkb_log(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: unexpected NULL character.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                    (*scanner).file_name,
                    loc.line,
                    loc.column,
                );
                return false_0 != 0;
            }
            if !is_ascii(*(*scanner).s.offset(0 as ::core::ffi::c_int as isize)) {
                let mut loc_0: scanner_loc = scanner_token_location(scanner);
                xkb_log(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: unexpected non-ASCII character.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
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

    use super::context_h::{xkb_context, xkb_log};
    use super::darray_h::darray_size_t;
    use super::messages_codes_h::{XKB_ERROR_INVALID_FILE_ENCODING, XKB_LOG_VERBOSITY_MINIMAL};
    use super::stdbool_h::{false_0, true_0};
    use super::string_h::{memchr, memcmp};
    use super::utils_h::is_ascii;
    use super::xkbcommon_h::XKB_LOG_LEVEL_ERROR;
    extern "C" {
        pub fn scanner_token_location(s: *mut scanner) -> scanner_loc;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn memmove(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
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
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: usize,
        ) -> ::core::ffi::c_int;
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> usize;
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn strlen_safe(mut s: *const ::core::ffi::c_char) -> usize {
        unsafe {
            return if !s.is_null() { strlen(s) } else { 0 as usize };
        }
    }
    #[inline]
    pub unsafe extern "C" fn isempty(mut s: *const ::core::ffi::c_char) -> bool {
        unsafe {
            return s.is_null()
                || *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn is_ascii(mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int & !(0x7f as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn is_space(mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int == ' ' as i32
                || ch as ::core::ffi::c_int >= '\t' as i32
                    && ch as ::core::ffi::c_int <= '\r' as i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn is_graph(mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= '!' as i32
                && ch as ::core::ffi::c_int <= '~' as i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn snprintf_safe(
        mut buf: *mut ::core::ffi::c_char,
        mut sz: usize,
        mut format: *const ::core::ffi::c_char,
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
    use super::string_h::strlen;
    use super::FILE_h::FILE;
    // map_file/unmap_file removed - use crate::xkb::utils::MappedFile instead
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe extern "C" fn parse_dec_to_uint32_t(
        mut s: *const ::core::ffi::c_char,
        mut len: usize,
        mut out: *mut u32,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint)
                    < 10 as ::core::ffi::c_uint
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
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint
                    >= 10 as ::core::ffi::c_uint
            {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }

    use super::stdint_uintn_h::u32;
}
pub mod include_h {
    pub const MERGE_OVERRIDE_PREFIX: ::core::ffi::c_int = '+' as i32;
    pub const MERGE_AUGMENT_PREFIX: ::core::ffi::c_int = '|' as i32;
    pub const MERGE_REPLACE_PREFIX: ::core::ffi::c_int = '^' as i32;

    use super::ast_h::xkb_file_type;
    use super::context_h::xkb_context;
    use super::stdio_h::ssize_t;
    use super::FILE_h::FILE;
    extern "C" {
        pub fn expand_path(
            ctx: *mut xkb_context,
            parent_file_name: *const ::core::ffi::c_char,
            name: *const ::core::ffi::c_char,
            name_len: usize,
            type_0: xkb_file_type,
            buf: *mut ::core::ffi::c_char,
            buf_size: usize,
        ) -> ssize_t;
        pub fn FindFileInXkbPath(
            ctx: *mut xkb_context,
            parent_file_name: *const ::core::ffi::c_char,
            name: *const ::core::ffi::c_char,
            name_len: usize,
            type_0: xkb_file_type,
            buf: *mut ::core::ffi::c_char,
            buf_size: usize,
            offset: *mut ::core::ffi::c_uint,
            required: bool,
        ) -> *mut FILE;
    }
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
}
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
    }
}
pub mod keymap_h {
    pub const XKB_MAX_GROUPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
}
pub mod rules_h {
    pub const OPTIONS_GROUP_SPECIFIER_PREFIX: ::core::ffi::c_int = '!' as i32;
}
pub mod utils_paths_h {
    extern "C" {
        pub fn is_absolute_path(path: *const ::core::ffi::c_char) -> bool;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
pub use self::ast_h::{
    xkb_file_type, _FILE_TYPE_NUM_ENTRIES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID,
    FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES,
    FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
};
pub use self::context_h::{
    xkb_context, xkb_context_sanitize_rule_names, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::{darray_char, darray_next_alloc, darray_size_t};
use self::errno_h::__errno_location;
pub use self::include_h::{
    expand_path, FindFileInXkbPath, MERGE_AUGMENT_PREFIX, MERGE_OVERRIDE_PREFIX,
    MERGE_REPLACE_PREFIX,
};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::keymap_h::XKB_MAX_GROUPS;
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
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::SIZE_MAX;
pub use self::stdint_uintn_h::{u32, uint8_t};
pub use self::stdio_h::{fclose, fopen, snprintf, ssize_t, va_list, vsnprintf};
use self::stdlib_h::{calloc, free, realloc};
use self::string_h::{memcpy, memmove, memset, strchr, strerror, strlen, strncmp};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __ssize_t, __uint32_t, __uint64_t, __uint8_t};
pub use self::utils_h::{is_ascii, is_graph, is_space, isempty, snprintf_safe, strlen_safe};
pub use self::utils_numbers_h::parse_dec_to_uint32_t;
use self::utils_paths_h::is_absolute_path;
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_component_names, xkb_layout_index_t, xkb_layout_mask_t, xkb_log_level, xkb_rule_names,
    XKB_LAYOUT_INVALID, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
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
pub type rules_kccgst = ::core::ffi::c_uint;
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
pub type kccgst_index_t = uint8_t;
pub type mlvo_index_t = uint8_t;
pub type mlvo_match_type = ::core::ffi::c_uint;
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
pub type kccgst_mask_t = uint8_t;
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
pub type mlvo_mask_t = uint8_t;
pub type rules_mlvo = ::core::ffi::c_uint;
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
pub type rules_token = ::core::ffi::c_uint;
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
    pub name: *const ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub range: layout_index_ranges,
}
pub type layout_index_ranges = ::core::ffi::c_uint;
pub type wildcard_match_type = ::core::ffi::c_uint;
pub const WILDCARD_MATCH_ALL: wildcard_match_type = 1;
pub const WILDCARD_MATCH_NONEMPTY: wildcard_match_type = 0;
pub const MAX_INCLUDE_DEPTH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn is_ident(mut ch: ::core::ffi::c_char) -> bool {
    unsafe {
        return is_graph(ch) as ::core::ffi::c_int != 0 && ch as ::core::ffi::c_int != '\\' as i32;
    }
}
unsafe extern "C" fn lex(mut s: *mut scanner, mut val: *mut lvalue) -> rules_token {
    unsafe {
        loop {
            while scanner_chr(s, ' ' as i32 as ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || scanner_chr(s, '\t' as i32 as ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || scanner_chr(s, '\r' as i32 as ::core::ffi::c_char) as ::core::ffi::c_int != 0
            {
            }
            if scanner_str(
                s,
                b"//\0".as_ptr() as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as usize)
                    .wrapping_sub(1 as usize),
            ) {
                scanner_skip_to_eol(s);
            }
            if scanner_eol(s) {
                while scanner_eol(s) {
                    scanner_next(s);
                }
                return TOK_END_OF_LINE;
            }
            if !scanner_chr(s, '\\' as i32 as ::core::ffi::c_char) {
                break;
            }
            scanner_chr(s, '\r' as i32 as ::core::ffi::c_char);
            if !scanner_eol(s) {
                let mut loc: scanner_loc = scanner_token_location(s);
                xkb_log(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: illegal new line escape; must appear at end of line\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    (*s).file_name,
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
        if scanner_chr(s, '!' as i32 as ::core::ffi::c_char) {
            return TOK_BANG;
        }
        if scanner_chr(s, '=' as i32 as ::core::ffi::c_char) {
            return TOK_EQUALS;
        }
        if scanner_chr(s, '*' as i32 as ::core::ffi::c_char) {
            return TOK_WILD_CARD_STAR;
        }
        if scanner_str(
            s,
            b"<none>\0".as_ptr() as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_NONE;
        }
        if scanner_str(
            s,
            b"<some>\0".as_ptr() as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_SOME;
        }
        if scanner_str(
            s,
            b"<any>\0".as_ptr() as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_ANY;
        }
        if scanner_chr(s, '$' as i32 as ::core::ffi::c_char) {
            (*val).string.start = (*s).s.offset((*s).pos as isize);
            (*val).string.len = 0 as usize;
            while is_ident(scanner_peek(s)) {
                scanner_next(s);
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            if (*val).string.len == 0 as usize {
                let mut loc_0: scanner_loc = scanner_token_location(s);
                xkb_log(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: unexpected character after '$'; expected name\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    (*s).file_name,
                    loc_0.line,
                    loc_0.column,
                );
                return TOK_ERROR;
            }
            return TOK_GROUP_NAME;
        }
        if scanner_str(
            s,
            b"include\0".as_ptr() as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize).wrapping_sub(1 as usize),
        ) {
            return TOK_INCLUDE;
        }
        if is_ident('+' as i32 as ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"is_ident(MERGE_OVERRIDE_PREFIX)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                131 as ::core::ffi::c_uint,
                b"enum rules_token lex(struct scanner *, union lvalue *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if is_ident('|' as i32 as ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"is_ident(MERGE_AUGMENT_PREFIX)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_uint,
                b"enum rules_token lex(struct scanner *, union lvalue *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if is_ident('^' as i32 as ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"is_ident(MERGE_REPLACE_PREFIX)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                133 as ::core::ffi::c_uint,
                b"enum rules_token lex(struct scanner *, union lvalue *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
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
        xkb_log(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"[XKB-%03d] %s:%zu:%zu: unrecognized token\n\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
            (*s).file_name,
            loc_1.line,
            loc_1.column,
        );
        return TOK_ERROR;
    }
}
static mut rules_mlvo_svals: [sval; 4] = [sval {
    len: 0,
    start: ::core::ptr::null::<::core::ffi::c_char>(),
}; 4];
static mut rules_kccgst_svals: [sval; 5] = [sval {
    len: 0,
    start: ::core::ptr::null::<::core::ffi::c_char>(),
}; 5];
pub const OPTIONS_MATCH_ALL_GROUPS: ::core::ffi::c_int = XKB_MAX_GROUPS;
unsafe extern "C" fn strip_spaces(mut v: sval) -> sval {
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
unsafe extern "C" fn split_comma_separated_mlvo(
    mut ctx: *mut xkb_context,
    mut mlvo: rules_mlvo,
    mut s: *const ::core::ffi::c_char,
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
                        start: ::core::ptr::null::<::core::ffi::c_char>(),
                    },
                };
                init.set_matched(false);
                init.set_layout(0);
                init
            };
            arr.size = arr.size.wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = arr.size;
            if __need > arr.alloc {
                arr.alloc = darray_next_alloc(
                    arr.alloc,
                    __need,
                    ::core::mem::size_of::<matched_sval>() as usize,
                );
                arr.item = realloc(
                    arr.item as *mut ::core::ffi::c_void,
                    (arr.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                ) as *mut matched_sval;
            }
            *arr.item
                .offset(arr.size.wrapping_sub(1 as darray_size_t) as isize) = val;
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
                init.set_matched(false_0 != 0);
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
                let layout_start: *const ::core::ffi::c_char = s;
                let mut layout: xkb_layout_index_t = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                let mut count: ::core::ffi::c_int =
                    parse_dec_to_uint32_t(s, SIZE_MAX as usize, &raw mut layout);
                if count > 0 as ::core::ffi::c_int {
                    s = s.offset(count as isize);
                    if layout == 0 as xkb_layout_index_t
                        || layout > XKB_MAX_GROUPS as xkb_layout_index_t
                    {
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Invalid layout index %u for the RMVLO component: \"%.*s\"\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                            layout,
                            val_0.sval.len as ::core::ffi::c_uint,
                            val_0.sval.start,
                        );
                    } else if mlvo as ::core::ffi::c_uint
                        != MLVO_OPTION as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        xkb_log(
                            ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"Layout index %u is not supported for the RMLVO component: \"%.*s\"\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            layout,
                            val_0.sval.len as ::core::ffi::c_uint,
                            val_0.sval.start,
                        );
                    } else {
                        val_0.set_layout(
                            layout.wrapping_sub(1 as xkb_layout_index_t) as xkb_layout_index_t
                        );
                    }
                }
                let layout_index_end: *const ::core::ffi::c_char = s;
                while *s as ::core::ffi::c_int != '\0' as i32
                    && *s as ::core::ffi::c_int != ',' as i32
                {
                    s = s.offset(1);
                }
                if count <= 0 as ::core::ffi::c_int || layout_index_end != s {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Invalid layout index \"%.*s\" for the RMLVO component \"%.*s\"; discarding specifier.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                        s.offset_from(layout_start) as ::core::ffi::c_long
                            as ::core::ffi::c_uint,
                        layout_start,
                        val_0.sval.len as ::core::ffi::c_uint,
                        val_0.sval.start,
                    );
                    val_0.set_layout(
                        OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t as xkb_layout_index_t,
                    );
                }
            }
            arr.size = arr.size.wrapping_add(1 as darray_size_t);
            let mut __need_0: darray_size_t = arr.size;
            if __need_0 > arr.alloc {
                arr.alloc = darray_next_alloc(
                    arr.alloc,
                    __need_0,
                    ::core::mem::size_of::<matched_sval>() as usize,
                );
                arr.item = realloc(
                    arr.item as *mut ::core::ffi::c_void,
                    (arr.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                ) as *mut matched_sval;
            }
            *arr.item
                .offset(arr.size.wrapping_sub(1 as darray_size_t) as isize) = val_0;
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
unsafe extern "C" fn matcher_new_from_rmlvo(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut rules: *mut *const ::core::ffi::c_char,
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
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                b"x\0".as_ptr() as *const ::core::ffi::c_char
            },
            variant: if (*rmlvo).layouts.size == 0 as darray_size_t {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                b"x\0".as_ptr() as *const ::core::ffi::c_char
            },
            options: if (*rmlvo).options.size == 0 as darray_size_t {
                ::core::ptr::null::<::core::ffi::c_char>()
            } else {
                b"x\0".as_ptr() as *const ::core::ffi::c_char
            },
        };
        let changed: RMLVO = xkb_context_sanitize_rule_names((*rmlvo).ctx, &raw mut names) as RMLVO;
        if changed as ::core::ffi::c_uint & RMLVO_RULES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            *rules = names.rules;
        } else {
            *rules = (*rmlvo).rules;
        }
        if changed as ::core::ffi::c_uint & RMLVO_MODEL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            (*m).rmlvo.model.sval.start = names.model;
        } else {
            (*m).rmlvo.model.sval.start = (*rmlvo).model;
        }
        (*m).rmlvo.model.sval.len = strlen_safe((*rmlvo).model);
        (*m).rmlvo
            .model
            .set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t as xkb_layout_index_t);
        if changed as ::core::ffi::c_uint
            & RMLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || changed as ::core::ffi::c_uint
                & RMLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
        } else {
            __assert_fail(
                b"(changed & RMLVO_LAYOUT) || !(changed & RMLVO_VARIANT)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                440 as ::core::ffi::c_uint,
                b"struct matcher *matcher_new_from_rmlvo(const struct xkb_rmlvo_builder *, const char **)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if changed as ::core::ffi::c_uint
            & RMLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            (*m).rmlvo.layouts =
                split_comma_separated_mlvo((*rmlvo).ctx, MLVO_LAYOUT, names.layout);
            (*m).rmlvo.variants =
                split_comma_separated_mlvo((*rmlvo).ctx, MLVO_VARIANT, names.variant);
            if (*m).rmlvo.layouts.size > (*m).rmlvo.variants.size {
                if !isempty(names.variant) {
                    xkb_log(
                        (*m).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"More layouts than variants: \"%s\" vs. \"%s\".\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        if !names.layout.is_null() {
                            names.layout
                        } else {
                            b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        if !names.variant.is_null() {
                            names.variant
                        } else {
                            b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
                let mut __oldSize: darray_size_t = (*m).rmlvo.variants.size;
                let mut __newSize: darray_size_t = (*m).rmlvo.layouts.size;
                (*m).rmlvo.variants.size = __newSize;
                if __newSize > __oldSize {
                    let mut __need: darray_size_t = __newSize;
                    if __need > (*m).rmlvo.variants.alloc {
                        (*m).rmlvo.variants.alloc = darray_next_alloc(
                            (*m).rmlvo.variants.alloc,
                            __need,
                            ::core::mem::size_of::<matched_sval>() as usize,
                        );
                        (*m).rmlvo.variants.item = realloc(
                            (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.variants.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                    }
                    memset(
                        (*m).rmlvo.variants.item.offset(__oldSize as isize) as *mut matched_sval
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (__newSize.wrapping_sub(__oldSize) as usize)
                            .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                    );
                }
            } else if (*m).rmlvo.layouts.size < (*m).rmlvo.variants.size {
                xkb_log(
                    (*m).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Less layouts than variants: \"%s\" vs. \"%s\".\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if !names.layout.is_null() {
                        names.layout
                    } else {
                        b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if !names.variant.is_null() {
                        names.variant
                    } else {
                        b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                (*m).rmlvo.variants.size = (*m).rmlvo.layouts.size;
                let mut __need_0: darray_size_t = (*m).rmlvo.variants.size;
                if __need_0 > (*m).rmlvo.variants.alloc {
                    (*m).rmlvo.variants.alloc = darray_next_alloc(
                        (*m).rmlvo.variants.alloc,
                        __need_0,
                        ::core::mem::size_of::<matched_sval>() as usize,
                    );
                    (*m).rmlvo.variants.item =
                        realloc(
                            (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.variants.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                }
                if (*m).rmlvo.variants.size > 0 as darray_size_t {
                    (*m).rmlvo.variants.alloc = (*m).rmlvo.variants.size;
                    (*m).rmlvo.variants.item =
                        realloc(
                            (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.variants.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                }
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
                                len: strlen_safe((*layout).layout),
                                start: (*layout).layout,
                            },
                        };
                        init.set_matched(false_0 != 0);
                        init.set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t);
                        init
                    };
                    (*m).rmlvo.layouts.size =
                        (*m).rmlvo.layouts.size.wrapping_add(1 as darray_size_t);
                    let mut __need_1: darray_size_t = (*m).rmlvo.layouts.size;
                    if __need_1 > (*m).rmlvo.layouts.alloc {
                        (*m).rmlvo.layouts.alloc = darray_next_alloc(
                            (*m).rmlvo.layouts.alloc,
                            __need_1,
                            ::core::mem::size_of::<matched_sval>() as usize,
                        );
                        (*m).rmlvo.layouts.item = realloc(
                            (*m).rmlvo.layouts.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.layouts.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                    }
                    *(*m)
                        .rmlvo
                        .layouts
                        .item
                        .offset(
                            (*m).rmlvo.layouts.size.wrapping_sub(1 as darray_size_t)
                                as isize,
                        ) = val;
                    val.sval.start = (*layout).variant;
                    val.sval.len = strlen_safe((*layout).variant);
                    (*m).rmlvo.variants.size =
                        (*m).rmlvo.variants.size.wrapping_add(1 as darray_size_t);
                    let mut __need_2: darray_size_t = (*m).rmlvo.variants.size;
                    if __need_2 > (*m).rmlvo.variants.alloc {
                        (*m).rmlvo.variants.alloc = darray_next_alloc(
                            (*m).rmlvo.variants.alloc,
                            __need_2,
                            ::core::mem::size_of::<matched_sval>() as usize,
                        );
                        (*m).rmlvo.variants.item = realloc(
                            (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.variants.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                    }
                    *(*m).rmlvo.variants.item.offset(
                        (*m).rmlvo.variants.size.wrapping_sub(1 as darray_size_t) as isize,
                    ) = val;
                    layout = layout.offset(1);
                }
            }
        }
        if changed as ::core::ffi::c_uint
            & RMLVO_OPTIONS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
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
                                len: strlen_safe((*option).option),
                                start: (*option).option,
                            },
                        };
                        init.set_matched(false_0 != 0);
                        init.set_layout(
                            if (*option).layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
                                OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t
                            } else {
                                (*option).layout
                            },
                        );
                        init
                    };
                    (*m).rmlvo.options.size =
                        (*m).rmlvo.options.size.wrapping_add(1 as darray_size_t);
                    let mut __need_3: darray_size_t = (*m).rmlvo.options.size;
                    if __need_3 > (*m).rmlvo.options.alloc {
                        (*m).rmlvo.options.alloc = darray_next_alloc(
                            (*m).rmlvo.options.alloc,
                            __need_3,
                            ::core::mem::size_of::<matched_sval>() as usize,
                        );
                        (*m).rmlvo.options.item = realloc(
                            (*m).rmlvo.options.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.options.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                    }
                    *(*m)
                        .rmlvo
                        .options
                        .item
                        .offset(
                            (*m).rmlvo.options.size.wrapping_sub(1 as darray_size_t)
                                as isize,
                        ) = val_0;
                    option = option.offset(1);
                }
            }
        }
        return m;
    }
}
unsafe extern "C" fn matcher_new_from_names(
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
        (*m).rmlvo.model.sval.len = strlen_safe((*rmlvo).model);
        (*m).rmlvo
            .model
            .set_layout(OPTIONS_MATCH_ALL_GROUPS as xkb_layout_index_t as xkb_layout_index_t);
        (*m).rmlvo.layouts = split_comma_separated_mlvo(ctx, MLVO_LAYOUT, (*rmlvo).layout);
        (*m).rmlvo.variants = split_comma_separated_mlvo(ctx, MLVO_VARIANT, (*rmlvo).variant);
        (*m).rmlvo.options = split_comma_separated_mlvo(ctx, MLVO_OPTION, (*rmlvo).options);
        if (*m).rmlvo.layouts.size > (*m).rmlvo.variants.size {
            if !isempty((*rmlvo).variant) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"More layouts than variants: \"%s\" vs. \"%s\".\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if !(*rmlvo).layout.is_null() {
                        (*rmlvo).layout
                    } else {
                        b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if !(*rmlvo).variant.is_null() {
                        (*rmlvo).variant
                    } else {
                        b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
            }
            let mut __oldSize: darray_size_t = (*m).rmlvo.variants.size;
            let mut __newSize: darray_size_t = (*m).rmlvo.layouts.size;
            (*m).rmlvo.variants.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*m).rmlvo.variants.alloc {
                    (*m).rmlvo.variants.alloc = darray_next_alloc(
                        (*m).rmlvo.variants.alloc,
                        __need,
                        ::core::mem::size_of::<matched_sval>() as usize,
                    );
                    (*m).rmlvo.variants.item =
                        realloc(
                            (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                            ((*m).rmlvo.variants.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                        ) as *mut matched_sval;
                }
                memset(
                    (*m).rmlvo.variants.item.offset(__oldSize as isize) as *mut matched_sval
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as usize)
                        .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                );
            }
        } else if (*m).rmlvo.layouts.size < (*m).rmlvo.variants.size {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Less layouts than variants: \"%s\" vs. \"%s\".\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if !(*rmlvo).layout.is_null() {
                    (*rmlvo).layout
                } else {
                    b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !(*rmlvo).variant.is_null() {
                    (*rmlvo).variant
                } else {
                    b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            (*m).rmlvo.variants.size = (*m).rmlvo.layouts.size;
            let mut __need_0: darray_size_t = (*m).rmlvo.variants.size;
            if __need_0 > (*m).rmlvo.variants.alloc {
                (*m).rmlvo.variants.alloc = darray_next_alloc(
                    (*m).rmlvo.variants.alloc,
                    __need_0,
                    ::core::mem::size_of::<matched_sval>() as usize,
                );
                (*m).rmlvo.variants.item = realloc(
                    (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                    ((*m).rmlvo.variants.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                ) as *mut matched_sval;
            }
            if (*m).rmlvo.variants.size > 0 as darray_size_t {
                (*m).rmlvo.variants.alloc = (*m).rmlvo.variants.size;
                (*m).rmlvo.variants.item = realloc(
                    (*m).rmlvo.variants.item as *mut ::core::ffi::c_void,
                    ((*m).rmlvo.variants.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<matched_sval>() as usize),
                ) as *mut matched_sval;
            }
        }
        return m;
    }
}
unsafe extern "C" fn matcher_free(mut m: *mut matcher) {
    unsafe {
        if m.is_null() {
            return;
        }
        free((*m).rmlvo.layouts.item as *mut ::core::ffi::c_void);
        (*m).rmlvo.layouts.item = ::core::ptr::null_mut::<matched_sval>();
        (*m).rmlvo.layouts.size = 0 as darray_size_t;
        (*m).rmlvo.layouts.alloc = 0 as darray_size_t;
        free((*m).rmlvo.variants.item as *mut ::core::ffi::c_void);
        (*m).rmlvo.variants.item = ::core::ptr::null_mut::<matched_sval>();
        (*m).rmlvo.variants.size = 0 as darray_size_t;
        (*m).rmlvo.variants.alloc = 0 as darray_size_t;
        free((*m).rmlvo.options.item as *mut ::core::ffi::c_void);
        (*m).rmlvo.options.item = ::core::ptr::null_mut::<matched_sval>();
        (*m).rmlvo.options.size = 0 as darray_size_t;
        (*m).rmlvo.options.alloc = 0 as darray_size_t;
        let mut group: *mut group = ::core::ptr::null_mut::<group>();
        if !(*m).groups.item.is_null() {
            group = (*m).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut group;
            while group < (*m).groups.item.offset((*m).groups.size as isize) as *mut group {
                free((*group).elements.item as *mut ::core::ffi::c_void);
                (*group).elements.item = ::core::ptr::null_mut::<sval>();
                (*group).elements.size = 0 as darray_size_t;
                (*group).elements.alloc = 0 as darray_size_t;
                group = group.offset(1);
            }
        }
        free((*m).pending_kccgst.buffer.item as *mut ::core::ffi::c_void);
        (*m).pending_kccgst.buffer.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*m).pending_kccgst.buffer.size = 0 as darray_size_t;
        (*m).pending_kccgst.buffer.alloc = 0 as darray_size_t;
        free((*m).pending_kccgst.slices.item as *mut ::core::ffi::c_void);
        (*m).pending_kccgst.slices.item = ::core::ptr::null_mut::<kccgst_buffer_slice>();
        (*m).pending_kccgst.slices.size = 0 as darray_size_t;
        (*m).pending_kccgst.slices.alloc = 0 as darray_size_t;
        let mut i: kccgst_index_t = 0 as kccgst_index_t;
        while (i as ::core::ffi::c_int)
            < _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as kccgst_index_t as ::core::ffi::c_int
        {
            free((*m).kccgst[i as usize].item as *mut ::core::ffi::c_void);
            (*m).kccgst[i as usize].item = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*m).kccgst[i as usize].size = 0 as darray_size_t;
            (*m).kccgst[i as usize].alloc = 0 as darray_size_t;
            i = i.wrapping_add(1);
        }
        free((*m).groups.item as *mut ::core::ffi::c_void);
        (*m).groups.item = ::core::ptr::null_mut::<group>();
        (*m).groups.size = 0 as darray_size_t;
        (*m).groups.alloc = 0 as darray_size_t;
        free(m as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn matcher_group_start_new(mut m: *mut matcher, mut name: sval) {
    unsafe {
        let mut group: group = group {
            name: name,
            elements: darray_sval {
                size: 0 as darray_size_t,
                alloc: 0 as darray_size_t,
                item: ::core::ptr::null_mut::<sval>(),
            },
        };
        (*m).groups.size = (*m).groups.size.wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = (*m).groups.size;
        if __need > (*m).groups.alloc {
            (*m).groups.alloc = darray_next_alloc(
                (*m).groups.alloc,
                __need,
                ::core::mem::size_of::<group>() as usize,
            );
            (*m).groups.item = realloc(
                (*m).groups.item as *mut ::core::ffi::c_void,
                ((*m).groups.alloc as usize).wrapping_mul(::core::mem::size_of::<group>() as usize),
            ) as *mut group;
        }
        *(*m)
            .groups
            .item
            .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize) = group;
    }
}
unsafe extern "C" fn matcher_group_add_element(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut element: sval,
) {
    unsafe {
        let ref mut c2rust_fresh1 = (*(*m)
            .groups
            .item
            .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
        .elements
        .size;
        *c2rust_fresh1 = (*(*m)
            .groups
            .item
            .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
        .elements
        .size
        .wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = *c2rust_fresh1;
        if __need
            > (*(*m)
                .groups
                .item
                .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
            .elements
            .alloc
        {
            let ref mut c2rust_fresh2 = (*(*m)
                .groups
                .item
                .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
            .elements
            .alloc;
            *c2rust_fresh2 = darray_next_alloc(
                (*(*m)
                    .groups
                    .item
                    .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
                .elements
                .alloc,
                __need,
                ::core::mem::size_of::<sval>() as usize,
            );
            let ref mut c2rust_fresh3 = (*(*m)
                .groups
                .item
                .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
            .elements
            .item;
            *c2rust_fresh3 = realloc(
                (*(*m)
                    .groups
                    .item
                    .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
                .elements
                .item as *mut ::core::ffi::c_void,
                (*c2rust_fresh2 as usize).wrapping_mul(::core::mem::size_of::<sval>() as usize),
            ) as *mut sval;
        }
        *(*(*m)
            .groups
            .item
            .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
        .elements
        .item
        .offset(
            (*(*m)
                .groups
                .item
                .offset((*m).groups.size.wrapping_sub(1 as darray_size_t) as isize))
            .elements
            .size
            .wrapping_sub(1 as darray_size_t) as isize,
        ) = element;
    }
}
unsafe extern "C" fn matcher_include(
    mut m: *mut matcher,
    mut parent_scanner: *mut scanner,
    mut include_depth: ::core::ffi::c_uint,
    mut inc: sval,
) {
    unsafe {
        if include_depth >= MAX_INCLUDE_DEPTH as ::core::ffi::c_uint {
            let mut loc: scanner_loc = scanner_token_location(parent_scanner);
            xkb_log(
                (*parent_scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s:%zu:%zu: maximum include depth (%u) exceeded; maybe there is an include loop?\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*parent_scanner).file_name,
                loc.line,
                loc.column,
                MAX_INCLUDE_DEPTH,
            );
            return;
        }
        let mut stmt_file: *const ::core::ffi::c_char = inc.start;
        let mut stmt_file_len: usize = inc.len;
        let mut buf: [::core::ffi::c_char; 4096] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let expanded: ssize_t = expand_path(
            (*m).ctx,
            (*parent_scanner).file_name,
            stmt_file,
            stmt_file_len,
            FILE_TYPE_RULES,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
        ) as ssize_t;
        if expanded < 0 as ssize_t {
            return;
        } else if expanded > 0 as ssize_t {
            stmt_file = &raw mut buf as *mut ::core::ffi::c_char;
            stmt_file_len = expanded as usize;
            if *stmt_file.offset(stmt_file_len as isize) as ::core::ffi::c_int == '\0' as i32 {
            } else {
                __assert_fail(
                    b"stmt_file[stmt_file_len] == '\\0'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                    604 as ::core::ffi::c_uint,
                    b"void matcher_include(struct matcher *, struct scanner *, unsigned int, struct sval)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
        }
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut offset: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let absolute_path: bool = is_absolute_path(stmt_file) as bool;
        if absolute_path {
            if expanded == 0 {
                if stmt_file_len < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize {
                    memcpy(
                        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        stmt_file as *const ::core::ffi::c_void,
                        stmt_file_len,
                    );
                    buf[stmt_file_len as usize] = '\0' as i32 as ::core::ffi::c_char;
                    stmt_file = &raw mut buf as *mut ::core::ffi::c_char;
                } else {
                    xkb_log(
                        (*m).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Path is too long: %zu > %zu, got raw path: %.*s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        XKB_ERROR_INVALID_PATH as ::core::ffi::c_int,
                        stmt_file_len,
                        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                        stmt_file_len as ::core::ffi::c_uint,
                        stmt_file,
                    );
                    return;
                }
            } else {
                if *stmt_file.offset(stmt_file_len as isize) as ::core::ffi::c_int == '\0' as i32 {
                } else {
                    __assert_fail(
                        b"stmt_file[stmt_file_len] == '\\0'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/rules.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        628 as ::core::ffi::c_uint,
                        b"void matcher_include(struct matcher *, struct scanner *, unsigned int, struct sval)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            }
            file = fopen(stmt_file, b"rb\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        } else if (expanded != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            file = ::core::ptr::null_mut::<FILE>();
        } else {
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name,
                stmt_file,
                stmt_file_len,
                FILE_TYPE_RULES,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                &raw mut offset,
                true_0 != 0,
            );
        }
        while !file.is_null() {
            if strlen_safe(&raw mut buf as *mut ::core::ffi::c_char)
                < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize
            {
            } else {
                __assert_fail(
                    b"strlen_safe(buf) < sizeof(buf)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                    655 as ::core::ffi::c_uint,
                    b"void matcher_include(struct matcher *, struct scanner *, unsigned int, struct sval)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            let mut ret: bool = read_rules_file(
                (*m).ctx,
                m,
                include_depth.wrapping_add(1 as ::core::ffi::c_uint),
                file,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            fclose(file);
            if ret {
                return;
            }
            xkb_log(
                (*m).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"No components returned from included XKB rules \"%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
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
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                &raw mut offset,
                true_0 != 0,
            );
        }
        xkb_log(
            (*m).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"Failed to open included XKB rules \"%.*s\"\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            stmt_file_len as ::core::ffi::c_uint,
            stmt_file,
        );
    }
}
unsafe extern "C" fn matcher_mapping_start_new(mut m: *mut matcher) {
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
        (*m).mapping.has_layout_idx_range = false_0 != 0;
        (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx =
            XKB_LAYOUT_INVALID as xkb_layout_index_t;
        (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx =
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx;
        (*m).mapping.num_kccgst = 0 as kccgst_index_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_kccgst as mlvo_index_t;
        (*m).mapping.defined_mlvo_mask = 0 as mlvo_mask_t;
        (*m).mapping.defined_kccgst_mask = 0 as kccgst_mask_t;
        (*m).mapping.c2rust_unnamed_0.active = true_0 as xkb_layout_mask_t;
    }
}
unsafe extern "C" fn parse_layout_int_index(
    mut s: *const ::core::ffi::c_char,
    mut max_len: usize,
    mut out: *mut xkb_layout_index_t,
) -> ::core::ffi::c_int {
    unsafe {
        if max_len >= 3 as usize {
        } else {
            __assert_fail(
                b"max_len >= 3\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                701 as ::core::ffi::c_uint,
                b"int parse_layout_int_index(const char *, usize, xkb_layout_index_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut val: u32 = 0 as u32;
        let count: ::core::ffi::c_int = parse_dec_to_uint32_t(
            s.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
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
unsafe extern "C" fn extract_layout_index(
    mut s: *const ::core::ffi::c_char,
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
unsafe extern "C" fn extract_mapping_layout_index(
    mut s: *const ::core::ffi::c_char,
    mut max_len: usize,
    mut out: *mut xkb_layout_index_t,
) -> ::core::ffi::c_int {
    unsafe {
        static mut names: [C2Rust_Unnamed_7; 4] = [
            C2Rust_Unnamed_7 {
                name: b"single]\0".as_ptr() as *const ::core::ffi::c_char,
                length: 7 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_SINGLE,
            },
            C2Rust_Unnamed_7 {
                name: b"first]\0".as_ptr() as *const ::core::ffi::c_char,
                length: 6 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_FIRST,
            },
            C2Rust_Unnamed_7 {
                name: b"later]\0".as_ptr() as *const ::core::ffi::c_char,
                length: 6 as ::core::ffi::c_int,
                range: LAYOUT_INDEX_LATER,
            },
            C2Rust_Unnamed_7 {
                name: b"any]\0".as_ptr() as *const ::core::ffi::c_char,
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
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_7; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_7>() as usize)
        {
            if strncmp(
                s.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
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
unsafe extern "C" fn is_mlvo_mask_defined(mut m: *mut matcher, mut mlvo: rules_mlvo) -> bool {
    unsafe {
        return (*m).mapping.defined_mlvo_mask as ::core::ffi::c_uint
            & (1 as ::core::ffi::c_uint) << mlvo as ::core::ffi::c_uint
            != 0;
    }
}
unsafe extern "C" fn matcher_mapping_set_mlvo(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
) {
    unsafe {
        let mut mlvo: rules_mlvo = MLVO_MODEL;
        let mut mlvo_sval: sval = sval {
            len: 0,
            start: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        mlvo = MLVO_MODEL;
        while (mlvo as ::core::ffi::c_uint)
            < _MLVO_NUM_ENTRIES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            mlvo_sval = rules_mlvo_svals[mlvo as usize];
            if svaleq_prefix(mlvo_sval, ident) {
                break;
            }
            mlvo += 1;
        }
        if mlvo as ::core::ffi::c_uint
            >= _MLVO_NUM_ENTRIES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"%.*s\" is not a valid value here; ignoring rule set\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc.line,
                loc.column,
                ident.len as ::core::ffi::c_uint,
                ident.start,
            );
            (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
            return;
        }
        if is_mlvo_mask_defined(m, mlvo) {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"%.*s\" appears twice on the same line; ignoring rule set\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc_0.line,
                loc_0.column,
                mlvo_sval.len as ::core::ffi::c_uint,
                mlvo_sval.start,
            );
            (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
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
                xkb_log(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"%.*s\" may only be followed by a valid group index; ignoring rule set\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    (*s).file_name,
                    loc_1.line,
                    loc_1.column,
                    mlvo_sval.len as ::core::ffi::c_uint,
                    mlvo_sval.start,
                );
                (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
                return;
            }
            if mlvo as ::core::ffi::c_uint
                == MLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx = idx;
            } else if mlvo as ::core::ffi::c_uint
                == MLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx = idx;
            } else {
                let mut loc_2: scanner_loc = scanner_token_location(s);
                xkb_log(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"%.*s\" cannot be followed by a group index; ignoring rule set\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                    (*s).file_name,
                    loc_2.line,
                    loc_2.column,
                    mlvo_sval.len as ::core::ffi::c_uint,
                    mlvo_sval.start,
                );
                (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
                return;
            }
        } else if mlvo as ::core::ffi::c_uint
            == MLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx =
                LAYOUT_INDEX_SINGLE as ::core::ffi::c_uint as xkb_layout_index_t;
        } else if mlvo as ::core::ffi::c_uint
            == MLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx =
                LAYOUT_INDEX_SINGLE as ::core::ffi::c_uint as xkb_layout_index_t;
        }
        if (mlvo as ::core::ffi::c_uint == MLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
            && is_mlvo_mask_defined(m, MLVO_VARIANT) as ::core::ffi::c_int != 0
            || mlvo as ::core::ffi::c_uint
                == MLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
                && is_mlvo_mask_defined(m, MLVO_LAYOUT) as ::core::ffi::c_int != 0)
            && (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                != (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        {
            let mut loc_3: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"layout\" index must be the same as the \"variant\" index\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc_3.line,
                loc_3.column,
            );
            (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
            return;
        }
        (*m).mapping.mlvo_at_pos[(*m).mapping.num_mlvo as usize] = mlvo;
        (*m).mapping.defined_mlvo_mask = ((*m).mapping.defined_mlvo_mask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_uint as mlvo_mask_t as ::core::ffi::c_int)
                << mlvo as ::core::ffi::c_uint)
            as mlvo_mask_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_mlvo.wrapping_add(1);
    }
}
unsafe extern "C" fn matcher_mapping_set_layout_bounds(mut m: *mut matcher) {
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
                if !is_mlvo_mask_defined(m, MLVO_LAYOUT) && !is_mlvo_mask_defined(m, MLVO_VARIANT) {
                } else {
                    __assert_fail(
                        b"!is_mlvo_mask_defined(m, MLVO_LAYOUT) && !is_mlvo_mask_defined(m, MLVO_VARIANT)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/rules.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        884 as ::core::ffi::c_uint,
                        b"void matcher_mapping_set_layout_bounds(struct matcher *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                (*m).mapping.has_layout_idx_range = false_0 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    XKB_LAYOUT_INVALID as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    XKB_LAYOUT_INVALID as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask = 0x1 as xkb_layout_mask_t;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967293 => {
                (*m).mapping.has_layout_idx_range = true_0 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    1 as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    (if (32 as darray_size_t) < (*m).rmlvo.layouts.size {
                        32 as darray_size_t
                    } else {
                        (*m).rmlvo.layouts.size
                    }) as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    (((1 as ::core::ffi::c_ulong)
                        << (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max)
                        .wrapping_sub(1 as ::core::ffi::c_ulong)
                        as xkb_layout_mask_t as ::core::ffi::c_ulong
                        & !(1 as ::core::ffi::c_ulong)) as xkb_layout_mask_t;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967294 => {
                (*m).mapping.has_layout_idx_range = true_0 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    0 as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    (if (32 as darray_size_t) < (*m).rmlvo.layouts.size {
                        32 as darray_size_t
                    } else {
                        (*m).rmlvo.layouts.size
                    }) as xkb_layout_index_t;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    (((1 as ::core::ffi::c_ulong)
                        << (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max)
                        as xkb_layout_mask_t as ::core::ffi::c_ulong)
                        .wrapping_sub(1 as ::core::ffi::c_ulong)
                        as xkb_layout_mask_t;
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
                (*m).mapping.has_layout_idx_range = false_0 != 0;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min = idx;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    idx.wrapping_add(1 as xkb_layout_index_t);
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    ((1 as ::core::ffi::c_uint) << idx) as xkb_layout_mask_t;
            }
            _ => {}
        };
    }
}
unsafe extern "C" fn matcher_mapping_set_kccgst(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
) {
    unsafe {
        let mut kccgst: rules_kccgst = KCCGST_KEYCODES;
        let mut kccgst_sval: sval = sval {
            len: 0,
            start: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        kccgst = KCCGST_KEYCODES;
        while (kccgst as ::core::ffi::c_uint)
            < _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            kccgst_sval = rules_kccgst_svals[kccgst as usize];
            if svaleq(rules_kccgst_svals[kccgst as usize], ident) {
                break;
            }
            kccgst += 1;
        }
        if kccgst as ::core::ffi::c_uint
            >= _KCCGST_NUM_ENTRIES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"%.*s\" is not a valid value here; ignoring rule set\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc.line,
                loc.column,
                ident.len as ::core::ffi::c_uint,
                ident.start,
            );
            (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
            return;
        }
        if (*m).mapping.defined_kccgst_mask as ::core::ffi::c_uint
            & (1 as ::core::ffi::c_uint) << kccgst as ::core::ffi::c_uint
            != 0
        {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: \"%.*s\" appears twice on the same line; ignoring rule set\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc_0.line,
                loc_0.column,
                kccgst_sval.len as ::core::ffi::c_uint,
                kccgst_sval.start,
            );
            (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
            return;
        }
        (*m).mapping.kccgst_at_pos[(*m).mapping.num_kccgst as usize] = kccgst;
        (*m).mapping.defined_kccgst_mask = ((*m).mapping.defined_kccgst_mask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_uint as kccgst_mask_t as ::core::ffi::c_int)
                << kccgst as ::core::ffi::c_uint)
            as kccgst_mask_t;
        (*m).mapping.num_kccgst = (*m).mapping.num_kccgst.wrapping_add(1);
    }
}
unsafe extern "C" fn matcher_mapping_verify(mut m: *mut matcher, mut s: *mut scanner) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        if (*m).mapping.num_mlvo as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: must have at least one value on the left hand side; ignoring rule set\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc.line,
                loc.column,
            );
        } else if (*m).mapping.num_kccgst as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid mapping: must have at least one value on the right hand side; ignoring rule set\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc_0.line,
                loc_0.column,
            );
        } else {
            if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
                if (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                    != 0xffffffff as xkb_layout_index_t
                {
                } else {
                    __assert_fail(
                        b"m->mapping.layout_idx != XKB_LAYOUT_INVALID\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/rules.c\0".as_ptr() as *const ::core::ffi::c_char,
                        983 as ::core::ffi::c_uint,
                        b"_Bool matcher_mapping_verify(struct matcher *, struct scanner *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
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
                        if (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
                            != 0xffffffff as xkb_layout_index_t
                        {
                        } else {
                            __assert_fail(
                                b"m->mapping.variant_idx != XKB_LAYOUT_INVALID\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"../src/xkbcomp/rules.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1006 as ::core::ffi::c_uint,
                                b"_Bool matcher_mapping_verify(struct matcher *, struct scanner *)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        };
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
                        _ => return true_0 != 0,
                    }
                }
            }
        }
        (*m).mapping.c2rust_unnamed_0.active = false_0 as xkb_layout_mask_t;
        return false_0 != 0;
    }
}
unsafe extern "C" fn matcher_rule_start_new(mut m: *mut matcher) {
    unsafe {
        memset(
            &raw mut (*m).rule as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<rule>() as usize,
        );
        (*m).rule.skip = (*m).mapping.c2rust_unnamed_0.active == 0;
    }
}
unsafe extern "C" fn matcher_rule_set_mlvo_common(
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
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid rule: has more values than the mapping line; ignoring rule\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc.line,
                loc.column,
            );
            (*m).rule.skip = true_0 != 0;
            return;
        }
        (*m).rule.match_type_at_pos[(*m).rule.num_mlvo_values as usize] = match_type;
        (*m).rule.mlvo_value_at_pos[(*m).rule.num_mlvo_values as usize] = ident;
        (*m).rule.num_mlvo_values = (*m).rule.num_mlvo_values.wrapping_add(1);
    }
}
unsafe extern "C" fn matcher_rule_set_mlvo_wildcard(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut match_type: mlvo_match_type,
) {
    unsafe {
        let mut dummy: sval = sval {
            len: 0 as usize,
            start: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        matcher_rule_set_mlvo_common(m, s, dummy, match_type);
    }
}
unsafe extern "C" fn matcher_rule_set_mlvo_group(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
    }
}
unsafe extern "C" fn matcher_rule_set_mlvo(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
    }
}
unsafe extern "C" fn matcher_rule_set_kccgst(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
) {
    unsafe {
        if (*m).rule.num_kccgst_values as ::core::ffi::c_int
            >= (*m).mapping.num_kccgst as ::core::ffi::c_int
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid rule: has more values than the mapping line; ignoring rule\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc.line,
                loc.column,
            );
            (*m).rule.skip = true_0 != 0;
            return;
        }
        (*m).rule.kccgst_value_at_pos[(*m).rule.num_kccgst_values as usize] = ident;
        (*m).rule.num_kccgst_values = (*m).rule.num_kccgst_values.wrapping_add(1);
    }
}
unsafe extern "C" fn match_group(mut m: *mut matcher, mut group_name: sval, mut to: sval) -> bool {
    unsafe {
        let mut group: *mut group = ::core::ptr::null_mut::<group>();
        let mut element: *mut sval = ::core::ptr::null_mut::<sval>();
        let mut found: bool = false_0 != 0;
        if !(*m).groups.item.is_null() {
            group = (*m).groups.item.offset(0 as ::core::ffi::c_int as isize) as *mut group;
            while group < (*m).groups.item.offset((*m).groups.size as isize) as *mut group {
                if svaleq((*group).name, group_name) {
                    found = true_0 != 0;
                    break;
                } else {
                    group = group.offset(1);
                }
            }
        }
        if !found {
            return false_0 != 0;
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
                    return true_0 != 0;
                }
                element = element.offset(1);
            }
        }
        return false_0 != 0;
    }
}
unsafe extern "C" fn match_value(
    mut m: *mut matcher,
    mut val: sval,
    mut to: sval,
    mut match_type: mlvo_match_type,
    mut wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        match match_type as ::core::ffi::c_uint {
            1 => {
                return wildcard_type as ::core::ffi::c_uint
                    == WILDCARD_MATCH_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                    || to.len != 0;
            }
            2 => return to.len == 0,
            3 => return to.len != 0,
            4 => return true_0 != 0,
            5 => return match_group(m, val, to),
            _ => {
                if match_type as ::core::ffi::c_uint
                    == MLVO_MATCH_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                } else {
                    __assert_fail(
                        b"match_type == MLVO_MATCH_NORMAL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/rules.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1145 as ::core::ffi::c_uint,
                        b"_Bool match_value(struct matcher *, struct sval, struct sval, enum mlvo_match_type, enum wildcard_match_type)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                return svaleq(val, to);
            }
        };
    }
}
unsafe extern "C" fn match_value_and_mark(
    mut m: *mut matcher,
    mut val: sval,
    mut to: *mut matched_sval,
    mut match_type: mlvo_match_type,
    mut wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        let mut matched: bool = match_value(m, val, (*to).sval, match_type, wildcard_type);
        if matched {
            (*to).set_matched((true_0 != 0) as bool);
        }
        return matched;
    }
}
unsafe extern "C" fn expand_rmlvo_in_kccgst_value(
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
        let mut str: *const ::core::ffi::c_char = value.start;
        let mut mlv: rules_mlvo = MLVO_MODEL;
        let mut idx: xkb_layout_index_t = 0;
        let mut pfx: ::core::ffi::c_char = 0;
        let mut sfx: ::core::ffi::c_char = 0;
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
                xkb_log(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: Invalid %%i in %%-expansion: there is no corresponding layout nor variant in the MLVO fields of the rules header.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION
                        as ::core::ffi::c_int,
                    (*s).file_name,
                    loc.line,
                    loc.column,
                );
            } else {
                *i = (*i).wrapping_add(1);
                let mut index_str: [::core::ffi::c_char; 12] = [0; 12];
                let mut count: ::core::ffi::c_int = snprintf(
                    &raw mut index_str as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as usize,
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    layout_idx.wrapping_add(1 as xkb_layout_index_t),
                );
                let mut __count: darray_size_t = count as darray_size_t;
                let mut __oldSize: darray_size_t = (*expanded).size;
                (*expanded).size = __oldSize
                    .wrapping_add(__count)
                    .wrapping_add(1 as darray_size_t);
                let mut __need: darray_size_t = (*expanded).size;
                if __need > (*expanded).alloc {
                    (*expanded).alloc = darray_next_alloc(
                        (*expanded).alloc,
                        __need,
                        ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                    );
                    (*expanded).item = realloc(
                        (*expanded).item as *mut ::core::ffi::c_void,
                        ((*expanded).alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    ) as *mut ::core::ffi::c_char;
                }
                memcpy(
                    (*expanded).item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
                    &raw mut index_str as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    (__count as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                );
                *(*expanded)
                    .item
                    .offset((*expanded).size.wrapping_sub(1 as darray_size_t) as isize) =
                    0 as ::core::ffi::c_char;
                (*expanded).size = (*expanded).size.wrapping_sub(1);
                return true_0 != 0;
            }
        } else {
            sfx = 0 as ::core::ffi::c_char;
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
                    sfx = ')' as i32 as ::core::ffi::c_char;
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
                            expanded_index = false_0 != 0;
                            if *i < value.len
                                && *str.offset(*i as isize) as ::core::ffi::c_int == '[' as i32
                            {
                                if mlv as ::core::ffi::c_uint
                                    != MLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
                                    && mlv as ::core::ffi::c_uint
                                        != MLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let mut loc_0: scanner_loc = scanner_token_location(s);
                                    xkb_log(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        b"[XKB-%03d] %s:%zu:%zu: invalid index in %%-expansion; may only index layout or variant\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                                        (*s).file_name,
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
                                            if layout_idx != 0xffffffff as xkb_layout_index_t {
                                            } else {
                                                __assert_fail(
                                                    b"layout_idx != XKB_LAYOUT_INVALID\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"../src/xkbcomp/rules.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1235 as ::core::ffi::c_uint,
                                                    b"_Bool expand_rmlvo_in_kccgst_value(struct matcher *, struct scanner *, struct sval, xkb_layout_index_t, darray_char *, usize *)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                            };
                                            idx = layout_idx;
                                            expanded_index = true_0 != 0;
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
                                            if mlv as ::core::ffi::c_uint
                                                == MLVO_LAYOUT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
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
                                            } else if mlv as ::core::ffi::c_uint
                                                == MLVO_VARIANT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
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
                                            } else if mlv as ::core::ffi::c_uint
                                                == MLVO_MODEL as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return true_0 != 0;
                                            }
                                            if pfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                let mut __count_0: darray_size_t =
                                                    1 as darray_size_t;
                                                let mut __oldSize_0: darray_size_t =
                                                    (*expanded).size;
                                                (*expanded).size = __oldSize_0
                                                    .wrapping_add(__count_0)
                                                    .wrapping_add(1 as darray_size_t);
                                                let mut __need_0: darray_size_t = (*expanded).size;
                                                if __need_0 > (*expanded).alloc {
                                                    (*expanded).alloc = darray_next_alloc(
                                                        (*expanded).alloc,
                                                        __need_0,
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    );
                                                    (*expanded).item = realloc(
                                                        (*expanded).item
                                                            as *mut ::core::ffi::c_void,
                                                        ((*expanded).alloc as usize).wrapping_mul(
                                                            ::core::mem::size_of::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                                as usize,
                                                        ),
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    (*expanded).item.offset(__oldSize_0 as isize)
                                                        as *mut ::core::ffi::c_void,
                                                    &raw mut pfx as *const ::core::ffi::c_void,
                                                    (__count_0 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                );
                                                *(*expanded).item.offset(
                                                    (*expanded)
                                                        .size
                                                        .wrapping_sub(1 as darray_size_t)
                                                        as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            let mut __count_1: darray_size_t =
                                                (*expanded_value).sval.len as darray_size_t;
                                            let mut __oldSize_1: darray_size_t = (*expanded).size;
                                            (*expanded).size = __oldSize_1
                                                .wrapping_add(__count_1)
                                                .wrapping_add(1 as darray_size_t);
                                            let mut __need_1: darray_size_t = (*expanded).size;
                                            if __need_1 > (*expanded).alloc {
                                                (*expanded).alloc = darray_next_alloc(
                                                    (*expanded).alloc,
                                                    __need_1,
                                                    ::core::mem::size_of::<::core::ffi::c_char>()
                                                        as usize,
                                                );
                                                (*expanded).item = realloc(
                                                    (*expanded).item as *mut ::core::ffi::c_void,
                                                    ((*expanded).alloc as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                )
                                                    as *mut ::core::ffi::c_char;
                                            }
                                            memcpy(
                                                (*expanded).item.offset(__oldSize_1 as isize)
                                                    as *mut ::core::ffi::c_void,
                                                (*expanded_value).sval.start
                                                    as *const ::core::ffi::c_void,
                                                (__count_1 as usize).wrapping_mul(
                                                    ::core::mem::size_of::<::core::ffi::c_char>()
                                                        as usize,
                                                ),
                                            );
                                            *(*expanded).item.offset(
                                                (*expanded).size.wrapping_sub(1 as darray_size_t)
                                                    as isize,
                                            ) = 0 as ::core::ffi::c_char;
                                            (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                let mut __count_2: darray_size_t =
                                                    1 as darray_size_t;
                                                let mut __oldSize_2: darray_size_t =
                                                    (*expanded).size;
                                                (*expanded).size = __oldSize_2
                                                    .wrapping_add(__count_2)
                                                    .wrapping_add(1 as darray_size_t);
                                                let mut __need_2: darray_size_t = (*expanded).size;
                                                if __need_2 > (*expanded).alloc {
                                                    (*expanded).alloc = darray_next_alloc(
                                                        (*expanded).alloc,
                                                        __need_2,
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    );
                                                    (*expanded).item = realloc(
                                                        (*expanded).item
                                                            as *mut ::core::ffi::c_void,
                                                        ((*expanded).alloc as usize).wrapping_mul(
                                                            ::core::mem::size_of::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                                as usize,
                                                        ),
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    (*expanded).item.offset(__oldSize_2 as isize)
                                                        as *mut ::core::ffi::c_void,
                                                    &raw mut sfx as *const ::core::ffi::c_void,
                                                    (__count_2 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                );
                                                *(*expanded).item.offset(
                                                    (*expanded)
                                                        .size
                                                        .wrapping_sub(1 as darray_size_t)
                                                        as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            (*expanded_value).set_matched((true_0 != 0) as bool);
                                            return true_0 != 0;
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
                            expanded_index = false_0 != 0;
                            if *i < value.len
                                && *str.offset(*i as isize) as ::core::ffi::c_int == '[' as i32
                            {
                                if mlv as ::core::ffi::c_uint
                                    != MLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
                                    && mlv as ::core::ffi::c_uint
                                        != MLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let mut loc_0: scanner_loc = scanner_token_location(s);
                                    xkb_log(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        b"[XKB-%03d] %s:%zu:%zu: invalid index in %%-expansion; may only index layout or variant\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                                        (*s).file_name,
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
                                            if layout_idx != 0xffffffff as xkb_layout_index_t {
                                            } else {
                                                __assert_fail(
                                                    b"layout_idx != XKB_LAYOUT_INVALID\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"../src/xkbcomp/rules.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1235 as ::core::ffi::c_uint,
                                                    b"_Bool expand_rmlvo_in_kccgst_value(struct matcher *, struct scanner *, struct sval, xkb_layout_index_t, darray_char *, usize *)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                            };
                                            idx = layout_idx;
                                            expanded_index = true_0 != 0;
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
                                            if mlv as ::core::ffi::c_uint
                                                == MLVO_LAYOUT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
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
                                            } else if mlv as ::core::ffi::c_uint
                                                == MLVO_VARIANT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
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
                                            } else if mlv as ::core::ffi::c_uint
                                                == MLVO_MODEL as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return true_0 != 0;
                                            }
                                            if pfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                let mut __count_0: darray_size_t =
                                                    1 as darray_size_t;
                                                let mut __oldSize_0: darray_size_t =
                                                    (*expanded).size;
                                                (*expanded).size = __oldSize_0
                                                    .wrapping_add(__count_0)
                                                    .wrapping_add(1 as darray_size_t);
                                                let mut __need_0: darray_size_t = (*expanded).size;
                                                if __need_0 > (*expanded).alloc {
                                                    (*expanded).alloc = darray_next_alloc(
                                                        (*expanded).alloc,
                                                        __need_0,
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    );
                                                    (*expanded).item = realloc(
                                                        (*expanded).item
                                                            as *mut ::core::ffi::c_void,
                                                        ((*expanded).alloc as usize).wrapping_mul(
                                                            ::core::mem::size_of::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                                as usize,
                                                        ),
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    (*expanded).item.offset(__oldSize_0 as isize)
                                                        as *mut ::core::ffi::c_void,
                                                    &raw mut pfx as *const ::core::ffi::c_void,
                                                    (__count_0 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                );
                                                *(*expanded).item.offset(
                                                    (*expanded)
                                                        .size
                                                        .wrapping_sub(1 as darray_size_t)
                                                        as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            let mut __count_1: darray_size_t =
                                                (*expanded_value).sval.len as darray_size_t;
                                            let mut __oldSize_1: darray_size_t = (*expanded).size;
                                            (*expanded).size = __oldSize_1
                                                .wrapping_add(__count_1)
                                                .wrapping_add(1 as darray_size_t);
                                            let mut __need_1: darray_size_t = (*expanded).size;
                                            if __need_1 > (*expanded).alloc {
                                                (*expanded).alloc = darray_next_alloc(
                                                    (*expanded).alloc,
                                                    __need_1,
                                                    ::core::mem::size_of::<::core::ffi::c_char>()
                                                        as usize,
                                                );
                                                (*expanded).item = realloc(
                                                    (*expanded).item as *mut ::core::ffi::c_void,
                                                    ((*expanded).alloc as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                )
                                                    as *mut ::core::ffi::c_char;
                                            }
                                            memcpy(
                                                (*expanded).item.offset(__oldSize_1 as isize)
                                                    as *mut ::core::ffi::c_void,
                                                (*expanded_value).sval.start
                                                    as *const ::core::ffi::c_void,
                                                (__count_1 as usize).wrapping_mul(
                                                    ::core::mem::size_of::<::core::ffi::c_char>()
                                                        as usize,
                                                ),
                                            );
                                            *(*expanded).item.offset(
                                                (*expanded).size.wrapping_sub(1 as darray_size_t)
                                                    as isize,
                                            ) = 0 as ::core::ffi::c_char;
                                            (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                let mut __count_2: darray_size_t =
                                                    1 as darray_size_t;
                                                let mut __oldSize_2: darray_size_t =
                                                    (*expanded).size;
                                                (*expanded).size = __oldSize_2
                                                    .wrapping_add(__count_2)
                                                    .wrapping_add(1 as darray_size_t);
                                                let mut __need_2: darray_size_t = (*expanded).size;
                                                if __need_2 > (*expanded).alloc {
                                                    (*expanded).alloc = darray_next_alloc(
                                                        (*expanded).alloc,
                                                        __need_2,
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    );
                                                    (*expanded).item = realloc(
                                                        (*expanded).item
                                                            as *mut ::core::ffi::c_void,
                                                        ((*expanded).alloc as usize).wrapping_mul(
                                                            ::core::mem::size_of::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                                as usize,
                                                        ),
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    (*expanded).item.offset(__oldSize_2 as isize)
                                                        as *mut ::core::ffi::c_void,
                                                    &raw mut sfx as *const ::core::ffi::c_void,
                                                    (__count_2 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                );
                                                *(*expanded).item.offset(
                                                    (*expanded)
                                                        .size
                                                        .wrapping_sub(1 as darray_size_t)
                                                        as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            (*expanded_value).set_matched((true_0 != 0) as bool);
                                            return true_0 != 0;
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
                            expanded_index = false_0 != 0;
                            if *i < value.len
                                && *str.offset(*i as isize) as ::core::ffi::c_int == '[' as i32
                            {
                                if mlv as ::core::ffi::c_uint
                                    != MLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
                                    && mlv as ::core::ffi::c_uint
                                        != MLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let mut loc_0: scanner_loc = scanner_token_location(s);
                                    xkb_log(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        b"[XKB-%03d] %s:%zu:%zu: invalid index in %%-expansion; may only index layout or variant\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                                        (*s).file_name,
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
                                            if layout_idx != 0xffffffff as xkb_layout_index_t {
                                            } else {
                                                __assert_fail(
                                                    b"layout_idx != XKB_LAYOUT_INVALID\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"../src/xkbcomp/rules.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1235 as ::core::ffi::c_uint,
                                                    b"_Bool expand_rmlvo_in_kccgst_value(struct matcher *, struct scanner *, struct sval, xkb_layout_index_t, darray_char *, usize *)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                            };
                                            idx = layout_idx;
                                            expanded_index = true_0 != 0;
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
                                            if mlv as ::core::ffi::c_uint
                                                == MLVO_LAYOUT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
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
                                            } else if mlv as ::core::ffi::c_uint
                                                == MLVO_VARIANT as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
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
                                            } else if mlv as ::core::ffi::c_uint
                                                == MLVO_MODEL as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return true_0 != 0;
                                            }
                                            if pfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                let mut __count_0: darray_size_t =
                                                    1 as darray_size_t;
                                                let mut __oldSize_0: darray_size_t =
                                                    (*expanded).size;
                                                (*expanded).size = __oldSize_0
                                                    .wrapping_add(__count_0)
                                                    .wrapping_add(1 as darray_size_t);
                                                let mut __need_0: darray_size_t = (*expanded).size;
                                                if __need_0 > (*expanded).alloc {
                                                    (*expanded).alloc = darray_next_alloc(
                                                        (*expanded).alloc,
                                                        __need_0,
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    );
                                                    (*expanded).item = realloc(
                                                        (*expanded).item
                                                            as *mut ::core::ffi::c_void,
                                                        ((*expanded).alloc as usize).wrapping_mul(
                                                            ::core::mem::size_of::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                                as usize,
                                                        ),
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    (*expanded).item.offset(__oldSize_0 as isize)
                                                        as *mut ::core::ffi::c_void,
                                                    &raw mut pfx as *const ::core::ffi::c_void,
                                                    (__count_0 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                );
                                                *(*expanded).item.offset(
                                                    (*expanded)
                                                        .size
                                                        .wrapping_sub(1 as darray_size_t)
                                                        as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            let mut __count_1: darray_size_t =
                                                (*expanded_value).sval.len as darray_size_t;
                                            let mut __oldSize_1: darray_size_t = (*expanded).size;
                                            (*expanded).size = __oldSize_1
                                                .wrapping_add(__count_1)
                                                .wrapping_add(1 as darray_size_t);
                                            let mut __need_1: darray_size_t = (*expanded).size;
                                            if __need_1 > (*expanded).alloc {
                                                (*expanded).alloc = darray_next_alloc(
                                                    (*expanded).alloc,
                                                    __need_1,
                                                    ::core::mem::size_of::<::core::ffi::c_char>()
                                                        as usize,
                                                );
                                                (*expanded).item = realloc(
                                                    (*expanded).item as *mut ::core::ffi::c_void,
                                                    ((*expanded).alloc as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                )
                                                    as *mut ::core::ffi::c_char;
                                            }
                                            memcpy(
                                                (*expanded).item.offset(__oldSize_1 as isize)
                                                    as *mut ::core::ffi::c_void,
                                                (*expanded_value).sval.start
                                                    as *const ::core::ffi::c_void,
                                                (__count_1 as usize).wrapping_mul(
                                                    ::core::mem::size_of::<::core::ffi::c_char>()
                                                        as usize,
                                                ),
                                            );
                                            *(*expanded).item.offset(
                                                (*expanded).size.wrapping_sub(1 as darray_size_t)
                                                    as isize,
                                            ) = 0 as ::core::ffi::c_char;
                                            (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            if sfx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                            {
                                                let mut __count_2: darray_size_t =
                                                    1 as darray_size_t;
                                                let mut __oldSize_2: darray_size_t =
                                                    (*expanded).size;
                                                (*expanded).size = __oldSize_2
                                                    .wrapping_add(__count_2)
                                                    .wrapping_add(1 as darray_size_t);
                                                let mut __need_2: darray_size_t = (*expanded).size;
                                                if __need_2 > (*expanded).alloc {
                                                    (*expanded).alloc = darray_next_alloc(
                                                        (*expanded).alloc,
                                                        __need_2,
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    );
                                                    (*expanded).item = realloc(
                                                        (*expanded).item
                                                            as *mut ::core::ffi::c_void,
                                                        ((*expanded).alloc as usize).wrapping_mul(
                                                            ::core::mem::size_of::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                                as usize,
                                                        ),
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    (*expanded).item.offset(__oldSize_2 as isize)
                                                        as *mut ::core::ffi::c_void,
                                                    &raw mut sfx as *const ::core::ffi::c_void,
                                                    (__count_2 as usize).wrapping_mul(
                                                        ::core::mem::size_of::<::core::ffi::c_char>(
                                                        )
                                                            as usize,
                                                    ),
                                                );
                                                *(*expanded).item.offset(
                                                    (*expanded)
                                                        .size
                                                        .wrapping_sub(1 as darray_size_t)
                                                        as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                (*expanded).size = (*expanded).size.wrapping_sub(1);
                                            }
                                            (*expanded_value).set_matched((true_0 != 0) as bool);
                                            return true_0 != 0;
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
        xkb_log(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"[XKB-%03d] %s:%zu:%zu: invalid %%-expansion in value; not used\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
            (*s).file_name,
            loc_1.line,
            loc_1.column,
        );
        return false_0 != 0;
    }
}
unsafe extern "C" fn expand_qualifier_in_kccgst_value(
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
        let mut str: *const ::core::ffi::c_char = value.start;
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
                xkb_log(
                    (*s).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                    b"%s:%zu:%zu: Using :all qualifier with indices range is not recommended.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*s).file_name,
                    loc.line,
                    loc.column,
                );
            }
            let mut __count: darray_size_t = 1 as darray_size_t;
            let mut __oldSize: darray_size_t = (*expanded).size;
            (*expanded).size = __oldSize
                .wrapping_add(__count)
                .wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = (*expanded).size;
            if __need > (*expanded).alloc {
                (*expanded).alloc = darray_next_alloc(
                    (*expanded).alloc,
                    __need,
                    ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                );
                (*expanded).item = realloc(
                    (*expanded).item as *mut ::core::ffi::c_void,
                    ((*expanded).alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                ) as *mut ::core::ffi::c_char;
            }
            memcpy(
                (*expanded).item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
                b"1\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                (__count as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
            );
            *(*expanded)
                .item
                .offset((*expanded).size.wrapping_sub(1 as darray_size_t) as isize) =
                0 as ::core::ffi::c_char;
            (*expanded).size = (*expanded).size.wrapping_sub(1);
            if (*m).rmlvo.layouts.size > 1 as darray_size_t {
                let mut layout_index: [::core::ffi::c_char; 12] = [0; 12];
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
                        (*expanded).size = (*expanded).size.wrapping_add(1 as darray_size_t);
                        let mut __need_0: darray_size_t = (*expanded).size;
                        if __need_0 > (*expanded).alloc {
                            (*expanded).alloc = darray_next_alloc(
                                (*expanded).alloc,
                                __need_0,
                                ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                            );
                            (*expanded).item =
                                realloc(
                                    (*expanded).item as *mut ::core::ffi::c_void,
                                    ((*expanded).alloc as usize).wrapping_mul(
                                        ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                                    ),
                                ) as *mut ::core::ffi::c_char;
                        }
                        *(*expanded)
                            .item
                            .offset((*expanded).size.wrapping_sub(1 as darray_size_t) as isize) =
                            '+' as i32 as ::core::ffi::c_char;
                    }
                    let mut __count_0: darray_size_t = prefix_length;
                    let mut __oldSize_0: darray_size_t = (*expanded).size;
                    (*expanded).size = __oldSize_0
                        .wrapping_add(__count_0)
                        .wrapping_add(1 as darray_size_t);
                    let mut __need_1: darray_size_t = (*expanded).size;
                    if __need_1 > (*expanded).alloc {
                        (*expanded).alloc = darray_next_alloc(
                            (*expanded).alloc,
                            __need_1,
                            ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                        );
                        (*expanded).item = realloc(
                            (*expanded).item as *mut ::core::ffi::c_void,
                            ((*expanded).alloc as usize).wrapping_mul(::core::mem::size_of::<
                                ::core::ffi::c_char,
                            >(
                            )
                                as usize),
                        ) as *mut ::core::ffi::c_char;
                    }
                    memcpy(
                        (*expanded).item.offset(__oldSize_0 as isize) as *mut ::core::ffi::c_void,
                        (*expanded).item.offset(prefix_idx as isize) as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (__count_0 as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    );
                    *(*expanded)
                        .item
                        .offset((*expanded).size.wrapping_sub(1 as darray_size_t) as isize) =
                        0 as ::core::ffi::c_char;
                    (*expanded).size = (*expanded).size.wrapping_sub(1);
                    let mut count: ::core::ffi::c_int = snprintf(
                        &raw mut layout_index as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as usize,
                        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                        l.wrapping_add(1 as xkb_layout_index_t),
                    );
                    let mut __count_1: darray_size_t = count as darray_size_t;
                    let mut __oldSize_1: darray_size_t = (*expanded).size;
                    (*expanded).size = __oldSize_1
                        .wrapping_add(__count_1)
                        .wrapping_add(1 as darray_size_t);
                    let mut __need_2: darray_size_t = (*expanded).size;
                    if __need_2 > (*expanded).alloc {
                        (*expanded).alloc = darray_next_alloc(
                            (*expanded).alloc,
                            __need_2,
                            ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                        );
                        (*expanded).item = realloc(
                            (*expanded).item as *mut ::core::ffi::c_void,
                            ((*expanded).alloc as usize).wrapping_mul(::core::mem::size_of::<
                                ::core::ffi::c_char,
                            >(
                            )
                                as usize),
                        ) as *mut ::core::ffi::c_char;
                    }
                    memcpy(
                        (*expanded).item.offset(__oldSize_1 as isize) as *mut ::core::ffi::c_void,
                        &raw mut layout_index as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (__count_1 as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    );
                    *(*expanded)
                        .item
                        .offset((*expanded).size.wrapping_sub(1 as darray_size_t) as isize) =
                        0 as ::core::ffi::c_char;
                    (*expanded).size = (*expanded).size.wrapping_sub(1);
                    l = l.wrapping_add(1);
                }
            }
            *i = (*i).wrapping_add(3 as usize);
        }
    }
}
#[inline]
unsafe extern "C" fn concat_kccgst(
    mut into: *mut darray_char,
    mut size: darray_size_t,
    mut from: *const ::core::ffi::c_char,
) {
    unsafe {
        let from_plus: bool = *from.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == MERGE_OVERRIDE_PREFIX
            || *from.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == MERGE_AUGMENT_PREFIX
            || *from.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == MERGE_REPLACE_PREFIX;
        if from_plus as ::core::ffi::c_int != 0 || (*into).size == 0 as darray_size_t {
            let mut __count: darray_size_t = size;
            let mut __oldSize: darray_size_t = (*into).size;
            (*into).size = __oldSize
                .wrapping_add(__count)
                .wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = (*into).size;
            if __need > (*into).alloc {
                (*into).alloc = darray_next_alloc(
                    (*into).alloc,
                    __need,
                    ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                );
                (*into).item = realloc(
                    (*into).item as *mut ::core::ffi::c_void,
                    ((*into).alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                ) as *mut ::core::ffi::c_char;
            }
            memcpy(
                (*into).item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
                from as *const ::core::ffi::c_void,
                (__count as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
            );
            *(*into)
                .item
                .offset((*into).size.wrapping_sub(1 as darray_size_t) as isize) =
                0 as ::core::ffi::c_char;
            (*into).size = (*into).size.wrapping_sub(1);
        } else {
            let ch: ::core::ffi::c_char = (if (*into).size == 0 as darray_size_t {
                '\0' as i32
            } else {
                *(*into).item.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            let into_plus: bool = ch as ::core::ffi::c_int == MERGE_OVERRIDE_PREFIX
                || ch as ::core::ffi::c_int == MERGE_AUGMENT_PREFIX
                || ch as ::core::ffi::c_int == MERGE_REPLACE_PREFIX;
            if into_plus {
                let mut __count_0: darray_size_t = size;
                let mut __oldSize_0: darray_size_t = (*into).size;
                (*into).size = __count_0
                    .wrapping_add(__oldSize_0)
                    .wrapping_add(1 as darray_size_t);
                let mut __need_0: darray_size_t = (*into).size;
                if __need_0 > (*into).alloc {
                    (*into).alloc = darray_next_alloc(
                        (*into).alloc,
                        __need_0,
                        ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                    );
                    (*into).item = realloc(
                        (*into).item as *mut ::core::ffi::c_void,
                        ((*into).alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    ) as *mut ::core::ffi::c_char;
                }
                memmove(
                    (*into).item.offset(__count_0 as isize) as *mut ::core::ffi::c_void,
                    (*into).item as *const ::core::ffi::c_void,
                    (__oldSize_0 as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                );
                memcpy(
                    (*into).item as *mut ::core::ffi::c_void,
                    from as *const ::core::ffi::c_void,
                    (__count_0 as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                );
                *(*into)
                    .item
                    .offset((*into).size.wrapping_sub(1 as darray_size_t) as isize) =
                    0 as ::core::ffi::c_char;
                (*into).size = (*into).size.wrapping_sub(1);
            }
        };
    }
}
unsafe extern "C" fn append_expanded_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut merge: bool,
    mut to: *mut darray_char,
    mut value: sval,
    mut layout_idx: xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut str: *const ::core::ffi::c_char = value.start;
        let mut expanded: darray_char = darray_char {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let mut last_item_idx: darray_size_t = 0 as darray_size_t;
        let mut has_separator: bool = false_0 != 0;
        let mut i: usize = 0 as usize;
        loop {
            if !(i < value.len) {
                c2rust_current_block = 10758786907990354186;
                break;
            }
            match *str.offset(i as isize) as ::core::ffi::c_int {
                58 => {
                    let mut __count: darray_size_t = 1 as darray_size_t;
                    let mut __oldSize: darray_size_t = expanded.size;
                    expanded.size = __oldSize
                        .wrapping_add(__count)
                        .wrapping_add(1 as darray_size_t);
                    let mut __need: darray_size_t = expanded.size;
                    if __need > expanded.alloc {
                        expanded.alloc = darray_next_alloc(
                            expanded.alloc,
                            __need,
                            ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                        );
                        expanded.item = realloc(
                            expanded.item as *mut ::core::ffi::c_void,
                            (expanded.alloc as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_char>() as usize
                                ),
                        ) as *mut ::core::ffi::c_char;
                    }
                    let c2rust_fresh4 = i;
                    i = i.wrapping_add(1);
                    memcpy(
                        expanded.item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
                        str.offset(c2rust_fresh4 as isize) as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (__count as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    );
                    *expanded
                        .item
                        .offset(expanded.size.wrapping_sub(1 as darray_size_t) as isize) =
                        0 as ::core::ffi::c_char;
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
                    let mut __count_0: darray_size_t = 1 as darray_size_t;
                    let mut __oldSize_0: darray_size_t = expanded.size;
                    expanded.size = __oldSize_0
                        .wrapping_add(__count_0)
                        .wrapping_add(1 as darray_size_t);
                    let mut __need_0: darray_size_t = expanded.size;
                    if __need_0 > expanded.alloc {
                        expanded.alloc = darray_next_alloc(
                            expanded.alloc,
                            __need_0,
                            ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                        );
                        expanded.item = realloc(
                            expanded.item as *mut ::core::ffi::c_void,
                            (expanded.alloc as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_char>() as usize
                                ),
                        ) as *mut ::core::ffi::c_char;
                    }
                    let c2rust_fresh5 = i;
                    i = i.wrapping_add(1);
                    memcpy(
                        expanded.item.offset(__oldSize_0 as isize) as *mut ::core::ffi::c_void,
                        str.offset(c2rust_fresh5 as isize) as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (__count_0 as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    );
                    *expanded
                        .item
                        .offset(expanded.size.wrapping_sub(1 as darray_size_t) as isize) =
                        0 as ::core::ffi::c_char;
                    expanded.size = expanded.size.wrapping_sub(1);
                    last_item_idx = expanded.size.wrapping_sub(1 as darray_size_t);
                    has_separator = true_0 != 0;
                }
                _ => {
                    let mut __count_1: darray_size_t = 1 as darray_size_t;
                    let mut __oldSize_1: darray_size_t = expanded.size;
                    expanded.size = __oldSize_1
                        .wrapping_add(__count_1)
                        .wrapping_add(1 as darray_size_t);
                    let mut __need_1: darray_size_t = expanded.size;
                    if __need_1 > expanded.alloc {
                        expanded.alloc = darray_next_alloc(
                            expanded.alloc,
                            __need_1,
                            ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                        );
                        expanded.item = realloc(
                            expanded.item as *mut ::core::ffi::c_void,
                            (expanded.alloc as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_char>() as usize
                                ),
                        ) as *mut ::core::ffi::c_char;
                    }
                    let c2rust_fresh6 = i;
                    i = i.wrapping_add(1);
                    memcpy(
                        expanded.item.offset(__oldSize_1 as isize) as *mut ::core::ffi::c_void,
                        str.offset(c2rust_fresh6 as isize) as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (__count_1 as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    );
                    *expanded
                        .item
                        .offset(expanded.size.wrapping_sub(1 as darray_size_t) as isize) =
                        0 as ::core::ffi::c_char;
                    expanded.size = expanded.size.wrapping_sub(1);
                }
            }
        }
        match c2rust_current_block {
            1032266188497003083 => {
                free(expanded.item as *mut ::core::ffi::c_void);
                expanded.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
                expanded.size = 0 as darray_size_t;
                expanded.alloc = 0 as darray_size_t;
                return false_0 != 0;
            }
            _ => {
                if merge {
                    if !(expanded.size == 0 as darray_size_t) {
                        concat_kccgst(to, expanded.size, expanded.item);
                    }
                } else if expanded.size > 0 as darray_size_t {
                    let mut __count_2: darray_size_t = expanded.size;
                    let mut __oldSize_2: darray_size_t = (*to).size;
                    (*to).size = __oldSize_2.wrapping_add(__count_2);
                    let mut __need_2: darray_size_t = (*to).size;
                    if __need_2 > (*to).alloc {
                        (*to).alloc = darray_next_alloc(
                            (*to).alloc,
                            __need_2,
                            ::core::mem::size_of::<::core::ffi::c_char>() as usize,
                        );
                        (*to).item = realloc(
                            (*to).item as *mut ::core::ffi::c_void,
                            ((*to).alloc as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_char>() as usize
                                ),
                        ) as *mut ::core::ffi::c_char;
                    }
                    memcpy(
                        (*to).item.offset(__oldSize_2 as isize) as *mut ::core::ffi::c_void,
                        expanded.item as *const ::core::ffi::c_void,
                        (__count_2 as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize),
                    );
                }
                free(expanded.item as *mut ::core::ffi::c_void);
                expanded.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
                expanded.size = 0 as darray_size_t;
                expanded.alloc = 0 as darray_size_t;
                return true_0 != 0;
            }
        };
    }
}
unsafe extern "C" fn matcher_append_pending_kccgst(mut m: *mut matcher) -> bool {
    unsafe {
        if !(*m).mapping.has_layout_idx_range {
            return true_0 != 0;
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
                    if (*slice).kccgst() as ::core::ffi::c_uint == kccgst as ::core::ffi::c_uint
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
        (*m).mapping.has_layout_idx_range = false_0 != 0;
        return true_0 != 0;
    }
}
unsafe extern "C" fn matcher_rule_verify(mut m: *mut matcher, mut s: *mut scanner) {
    unsafe {
        if (*m).rule.num_mlvo_values as ::core::ffi::c_int
            != (*m).mapping.num_mlvo as ::core::ffi::c_int
            || (*m).rule.num_kccgst_values as ::core::ffi::c_int
                != (*m).mapping.num_kccgst as ::core::ffi::c_int
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            xkb_log(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: invalid rule: must have same number of values as mapping line; ignoring rule\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                (*s).file_name,
                loc.line,
                loc.column,
            );
            (*m).rule.skip = true_0 != 0;
        }
    }
}
unsafe extern "C" fn matcher_rule_apply_if_matches(mut m: *mut matcher, mut s: *mut scanner) {
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
            let mut matched: bool = false_0 != 0;
            if mlvo as ::core::ffi::c_uint
                == MLVO_MODEL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                to = &raw mut (*m).rmlvo.model;
                matched = match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_ALL);
            } else if (*m).mapping.has_layout_idx_range {
                idx = (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
                while idx < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max
                    && candidate_layouts != 0
                {
                    let mask: xkb_layout_mask_t = (1 as xkb_layout_mask_t) << idx;
                    if candidate_layouts & mask != 0 {
                        match mlvo as ::core::ffi::c_uint {
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
                                    matched = true_0 != 0;
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
                                    matched = true_0 != 0;
                                } else {
                                    candidate_layouts &= !mask;
                                }
                            }
                            _ => {
                                if mlvo as ::core::ffi::c_uint
                                    == MLVO_OPTION as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                } else {
                                    __assert_fail(
                                        b"mlvo == MLVO_OPTION\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"../src/xkbcomp/rules.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_uint,
                                        b"void matcher_rule_apply_if_matches(struct matcher *, struct scanner *)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                };
                                let mut found_option: bool = false_0 != 0;
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
                                                matched = true_0 != 0;
                                                found_option = true_0 != 0;
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
                match mlvo as ::core::ffi::c_uint {
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
                        if mlvo as ::core::ffi::c_uint
                            == MLVO_OPTION as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                        } else {
                            __assert_fail(
                                b"mlvo == MLVO_OPTION\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"../src/xkbcomp/rules.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1578 as ::core::ffi::c_uint,
                                b"void matcher_rule_apply_if_matches(struct matcher *, struct scanner *)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        };
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
                            false_0 != 0,
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
                        (*buf).slices.size = (*buf).slices.size.wrapping_add(1 as darray_size_t);
                        let mut __need: darray_size_t = (*buf).slices.size;
                        if __need > (*buf).slices.alloc {
                            (*buf).slices.alloc = darray_next_alloc(
                                (*buf).slices.alloc,
                                __need,
                                ::core::mem::size_of::<kccgst_buffer_slice>() as usize,
                            );
                            (*buf).slices.item = realloc(
                                (*buf).slices.item as *mut ::core::ffi::c_void,
                                ((*buf).slices.alloc as usize).wrapping_mul(
                                    ::core::mem::size_of::<kccgst_buffer_slice>() as usize,
                                ),
                            )
                                as *mut kccgst_buffer_slice;
                        }
                        *(*buf)
                            .slices
                            .item
                            .offset((*buf).slices.size.wrapping_sub(1 as darray_size_t) as isize) =
                            slice;
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
                    true_0 != 0,
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
unsafe extern "C" fn gettok(mut m: *mut matcher, mut s: *mut scanner) -> rules_token {
    unsafe {
        return lex(s, &raw mut (*m).val);
    }
}
unsafe extern "C" fn matcher_match(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut include_depth: ::core::ffi::c_uint,
    mut string: *const ::core::ffi::c_char,
    mut len: usize,
    mut file_name: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut tok: rules_token = TOK_END_OF_FILE;
        if m.is_null() {
            return false_0 != 0;
        }
        '_initial: loop {
            tok = gettok(m, s);
            match tok as ::core::ffi::c_uint {
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
                match tok as ::core::ffi::c_uint {
                    3 => {
                        matcher_group_start_new(m, (*m).val.string);
                        tok = gettok(m, s);
                        match tok as ::core::ffi::c_uint {
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
                        match tok as ::core::ffi::c_uint {
                            2 => {}
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                        matcher_include(m, s, include_depth, (*m).val.string);
                        tok = gettok(m, s);
                        match tok as ::core::ffi::c_uint {
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
                            match tok as ::core::ffi::c_uint {
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
                            match tok as ::core::ffi::c_uint {
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
                            match tok as ::core::ffi::c_uint {
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
                                        match tok as ::core::ffi::c_uint {
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
                                        match tok as ::core::ffi::c_uint {
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
                match tok as ::core::ffi::c_uint {
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
            13194772801876125100 => return true_0 != 0,
            _ => {
                match tok as ::core::ffi::c_uint {
                    11 => {}
                    _ => {
                        let mut loc: scanner_loc = scanner_token_location(s);
                        xkb_log(
                            (*s).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] %s:%zu:%zu: unexpected token\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            XKB_ERROR_INVALID_RULES_SYNTAX as ::core::ffi::c_int,
                            (*s).file_name,
                            loc.line,
                            loc.column,
                        );
                    }
                }
                return false_0 != 0;
            }
        };
    }
}
unsafe extern "C" fn read_rules_file(
    mut ctx: *mut xkb_context,
    mut matcher: *mut matcher,
    mut include_depth: ::core::ffi::c_uint,
    mut file: *mut FILE,
    mut path: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        let mut ret: bool = false;
        let mut scanner: scanner = scanner {
            pos: 0,
            len: 0,
            s: ::core::ptr::null::<::core::ffi::c_char>(),
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: scanner_loc { line: 0, column: 0 },
            file_name: ::core::ptr::null::<::core::ffi::c_char>(),
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            priv_0: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };

        // Convert FILE* to Rust File and map it
        use crate::xkb::utils::MappedFile;
        use std::fs::File;
        use std::os::unix::io::FromRawFd;

        let fd = libc::fileno(file as *mut libc::FILE);
        if fd < 0 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Invalid file descriptor\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }

        let rust_file = File::from_raw_fd(fd);
        let mapped = match MappedFile::new(&rust_file) {
            Ok(m) => m,
            Err(e) => {
                let err_msg = std::ffi::CString::new(e.to_string())
                    .unwrap_or_else(|_| std::ffi::CString::new("unknown error").unwrap());
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Couldn't read rules file \"%s\": %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    path,
                    err_msg.as_ptr(),
                );
                std::mem::forget(rust_file);
                return false_0 != 0;
            }
        };

        scanner_init(
            &raw mut scanner,
            (*matcher).ctx,
            mapped.as_ptr(),
            mapped.len(),
            path,
            NULL,
        );
        if !scanner_check_supported_char_encoding(&raw mut scanner) {
            let mut loc: scanner_loc = scanner_token_location(&raw mut scanner);
            xkb_log(
                scanner.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                scanner.file_name,
                loc.line,
                loc.column,
            );
            let mut loc_0: scanner_loc = scanner_token_location(&raw mut scanner);
            xkb_log(
                scanner.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                scanner.file_name,
                loc_0.line,
                loc_0.column,
            );
            std::mem::forget(rust_file);
            return false_0 != 0;
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
unsafe extern "C" fn xkb_resolve_partial_rules(
    mut ctx: *mut xkb_context,
    mut path: *mut ::core::ffi::c_char,
    mut path_size: usize,
    mut rules: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
    mut matcher: *mut matcher,
) -> bool {
    unsafe {
        let mut partial_rules: [::core::ffi::c_char; 60] = [0; 60];
        if !snprintf_safe(
            &raw mut partial_rules as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as usize,
            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            rules,
            suffix,
        ) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot load XKB rules \"%s%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                rules,
                suffix,
            );
            return false_0 != 0;
        }
        let mut offset: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let len: usize = strlen(&raw mut partial_rules as *mut ::core::ffi::c_char) as usize;
        loop {
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut partial_rules as *mut ::core::ffi::c_char,
                len,
                FILE_TYPE_RULES,
                path,
                path_size,
                &raw mut offset,
                false_0 != 0,
            );
            if file.is_null() {
                break;
            }
            let ok: bool =
                read_rules_file(ctx, matcher, 0 as ::core::ffi::c_uint, file, path) as bool;
            fclose(file);
            if !ok {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Error while parsing XKB rules \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                    path,
                );
                return false_0 != 0;
            }
            offset = offset.wrapping_add(1);
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn xkb_resolve_rules(
    mut ctx: *mut xkb_context,
    mut rules: *const ::core::ffi::c_char,
    mut matcher: *mut matcher,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut mval: *mut matched_sval = ::core::ptr::null_mut::<matched_sval>();
        let mut ret: bool = false_0 != 0;
        let mut offset: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
        let file: *mut FILE = FindFileInXkbPath(
            ctx,
            b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char,
            rules,
            strlen(rules),
            FILE_TYPE_RULES,
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            &raw mut offset,
            true_0 != 0,
        ) as *mut FILE;
        if file.is_null() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot load XKB rules \"%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                rules,
            );
        } else {
            ret = xkb_resolve_partial_rules(
                ctx,
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                rules,
                b".pre\0".as_ptr() as *const ::core::ffi::c_char,
                matcher,
            );
            if ret {
                ret = read_rules_file(
                    ctx,
                    matcher,
                    0 as ::core::ffi::c_uint,
                    file,
                    &raw mut path as *mut ::core::ffi::c_char,
                );
                if !ret {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Error while parsing XKB rules \"%s\"\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                        &raw mut path as *mut ::core::ffi::c_char,
                    );
                } else {
                    ret = xkb_resolve_partial_rules(
                        ctx,
                        &raw mut path as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                        rules,
                        b".post\0".as_ptr() as *const ::core::ffi::c_char,
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
                            xkb_log(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                b"[XKB-%03d] No components returned from XKB rules \"%s\"\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                rules,
                            );
                            ret = false_0 != 0;
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
                                .item = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                                .item = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            (*matcher).kccgst[KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                .size = 0 as darray_size_t;
                            (*matcher).kccgst[KCCGST_GEOMETRY as ::core::ffi::c_int as usize]
                                .alloc = 0 as darray_size_t;
                            mval = &raw mut (*matcher).rmlvo.model;
                            if !(*mval).matched() && (*mval).sval.len > 0 as usize {
                                xkb_log(
                                    (*matcher).ctx,
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                    b"[XKB-%03d] Unrecognized RMLVO model \"%.*s\" was ignored\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                    (*mval).sval.len as ::core::ffi::c_uint,
                                    (*mval).sval.start,
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
                                        xkb_log(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                            b"[XKB-%03d] Unrecognized RMLVO layout \"%.*s\" was ignored\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                            (*mval).sval.len as ::core::ffi::c_uint,
                                            (*mval).sval.start,
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
                                        xkb_log(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                            b"[XKB-%03d] Unrecognized RMLVO variant \"%.*s\" was ignored\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                            (*mval).sval.len as ::core::ffi::c_uint,
                                            (*mval).sval.start,
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
                                        xkb_log(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                            b"[XKB-%03d] Unrecognized RMLVO option \"%.*s\" was ignored\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as ::core::ffi::c_int,
                                            (*mval).sval.len as ::core::ffi::c_uint,
                                            (*mval).sval.start,
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*out).symbols.is_null() && !explicit_layouts.is_null() {
                                *explicit_layouts = 1 as xkb_layout_index_t;
                                let mut symbols: *const ::core::ffi::c_char = (*out).symbols;
                                loop {
                                    symbols = strchr(symbols, ':' as i32);
                                    if !(!symbols.is_null()
                                        && *symbols.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            != '\0' as i32)
                                    {
                                        break;
                                    }
                                    let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                                    symbols = symbols.offset(1);
                                    let count: ::core::ffi::c_int = parse_dec_to_uint32_t(
                                        symbols,
                                        SIZE_MAX as usize,
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
#[no_mangle]
pub unsafe extern "C" fn xkb_components_from_rmlvo_builder(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut rules: *const ::core::ffi::c_char = (*rmlvo).rules;
        let mut matcher: *mut matcher = matcher_new_from_rmlvo(rmlvo, &raw mut rules);
        if matcher.is_null() {
            return false_0 != 0;
        }
        let ret: bool =
            xkb_resolve_rules((*rmlvo).ctx, rules, matcher, out, explicit_layouts) as bool;
        matcher_free(matcher);
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_components_from_rules_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *const xkb_rule_names,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut matcher: *mut matcher = matcher_new_from_names(ctx, rmlvo);
        if matcher.is_null() {
            return false_0 != 0;
        }
        let ret: bool =
            xkb_resolve_rules(ctx, (*rmlvo).rules, matcher, out, explicit_layouts) as bool;
        matcher_free(matcher);
        return ret;
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        rules_kccgst_svals = [
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"types\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"compat\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"symbols\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"geometry\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        rules_mlvo_svals = [
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"model\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"layout\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"variant\0".as_ptr() as *const ::core::ffi::c_char,
            },
            sval {
                len: (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                    .wrapping_sub(1 as usize),
                start: b"option\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
