pub mod internal {
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
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
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
        pub text_next: size_t,
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
    use super::__stddef_size_t_h::size_t;
    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
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
        mut itemSize: size_t,
    ) -> darray_size_t {
        unsafe {
            if (need as size_t)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as size_t)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, size_t)\0"
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
    use super::__stddef_size_t_h::size_t;
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
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    use super::context_h::xkb_context;

    extern "C" {
        pub type xkb_keymap;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_context_set_user_data(
            context: *mut xkb_context,
            user_data: *mut ::core::ffi::c_void,
        );
        pub fn xkb_context_get_user_data(context: *mut xkb_context) -> *mut ::core::ffi::c_void;
        pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);
        pub fn xkb_context_set_log_verbosity(
            context: *mut xkb_context,
            verbosity: ::core::ffi::c_int,
        );
        pub fn xkb_context_set_log_fn(
            context: *mut xkb_context,
            log_fn_0: Option<
                unsafe extern "C" fn(
                    *mut xkb_context,
                    xkb_log_level,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
            >,
        );
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    }
}
pub mod xkbcommon_compose_h {
    pub type xkb_compose_compile_flags = ::core::ffi::c_uint;
    pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
    pub type xkb_compose_format = ::core::ffi::c_uint;
    pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    extern "C" {
        pub type xkb_compose_table;
        pub fn xkb_compose_table_new_from_buffer(
            context: *mut xkb_context,
            buffer: *const ::core::ffi::c_char,
            length: size_t,
            locale: *const ::core::ffi::c_char,
            format: xkb_compose_format,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;
        pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);
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
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::xkbcommon_h::{xkb_keymap, xkb_keymap_format};
    extern "C" {
        pub fn test_init();
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_buffer(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            buf: *const ::core::ffi::c_char,
            len: size_t,
        ) -> *mut xkb_keymap;
    }
}
pub mod stdio_h {

    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
        pub fn vasprintf(
            __ptr: *mut *mut ::core::ffi::c_char,
            __f: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}
pub mod stdlib_h {
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn setenv(
            __name: *const ::core::ffi::c_char,
            __value: *const ::core::ffi::c_char,
            __replace: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn streq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if !s1.is_null() && !s2.is_null() {
            } else {
                __assert_fail(
                    b"s1 && s2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../src/utils.h\0".as_ptr() as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
            return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return s1 == s2;
            }
            return streq(s1, s2);
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_not_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return false;
            }
            return streq(s1, s2);
        }
    }
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::string_h::strcmp;
}
pub mod include_locale_h {
    pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
    use super::locale_h::__LC_ALL;
    extern "C" {
        pub fn setlocale(
            __category: ::core::ffi::c_int,
            __locale: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
    }
}
pub mod assert_h {
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 40] = unsafe {
        ::core::mem::transmute::<[u8; 40], [::core::ffi::c_char; 40]>(
            *b"_Bool streq(const char *, const char *)\0",
        )
    };
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
    pub const NULL_0: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod locale_h {
    pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
}
pub mod stdbool_h {
}
pub use self::__stddef_null_h::{NULL, NULL_0};
pub use self::__stddef_size_t_h::size_t;
pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
pub use self::context_h::{xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_char, darray_next_alloc, darray_size_t};
pub use self::include_locale_h::{setlocale, LC_ALL};
pub use self::internal::__va_list_tag;
pub use self::locale_h::__LC_ALL;
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
use self::stdio_h::{fprintf, printf, stderr, vasprintf};
pub use self::stdlib_h::{free, realloc, setenv, EXIT_SUCCESS};
use self::string_h::{memcpy, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    test_compile_buffer, test_context_flags, test_get_context, test_init,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::utils_h::{streq, streq_not_null, streq_null};
pub use self::xkbcommon_compose_h::{
    xkb_compose_compile_flags, xkb_compose_format, xkb_compose_table,
    xkb_compose_table_new_from_buffer, xkb_compose_table_unref, XKB_COMPOSE_COMPILE_NO_FLAGS,
    XKB_COMPOSE_FORMAT_TEXT_V1,
};
pub use self::xkbcommon_h::{
    xkb_context_get_user_data, xkb_context_set_log_fn, xkb_context_set_log_level,
    xkb_context_set_log_verbosity, xkb_context_set_user_data, xkb_context_unref, xkb_keymap,
    xkb_keymap_format, xkb_keymap_unref, xkb_log_level, xkb_rule_names, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_data {
    pub input: *const ::core::ffi::c_char,
    pub log: *const ::core::ffi::c_char,
    pub error: bool,
}
unsafe extern "C" fn log_level_to_string(mut level: xkb_log_level) -> *const ::core::ffi::c_char {
    unsafe {
        match level as ::core::ffi::c_uint {
            10 => return b"critical\0".as_ptr() as *const ::core::ffi::c_char,
            20 => return b"error\0".as_ptr() as *const ::core::ffi::c_char,
            30 => return b"warning\0".as_ptr() as *const ::core::ffi::c_char,
            40 => return b"info\0".as_ptr() as *const ::core::ffi::c_char,
            50 => return b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            _ => {}
        }
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
unsafe extern "C" fn log_fn(
    mut ctx: *mut xkb_context,
    mut level: xkb_log_level,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    unsafe {
        let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut size: ::core::ffi::c_int = 0;
        let mut ls: *mut darray_char = xkb_context_get_user_data(ctx) as *mut darray_char;
        if !ls.is_null() {
        } else {
            __assert_fail(
                b"ls\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_uint,
                b"void log_fn(struct xkb_context *, enum xkb_log_level, const char *, struct __va_list_tag *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        size = vasprintf(&raw mut s, fmt, args);
        if size != -1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"size != -1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                48 as ::core::ffi::c_uint,
                b"void log_fn(struct xkb_context *, enum xkb_log_level, const char *, struct __va_list_tag *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut __str: *const ::core::ffi::c_char = log_level_to_string(level);
        let mut __count: darray_size_t =
            (strlen(__str) as darray_size_t).wrapping_add(1 as darray_size_t);
        let mut __oldSize: darray_size_t = (*ls).size;
        (*ls).size = __oldSize.wrapping_add(__count);
        let mut __need: darray_size_t = (*ls).size;
        if __need > (*ls).alloc {
            (*ls).alloc = darray_next_alloc(
                (*ls).alloc,
                __need,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            );
            (*ls).item = realloc(
                (*ls).item as *mut ::core::ffi::c_void,
                ((*ls).alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
        }
        memcpy(
            (*ls).item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
            __str as *const ::core::ffi::c_void,
            (__count as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        (*ls).size = (*ls).size.wrapping_sub(1);
        let mut __count_0: darray_size_t =
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as darray_size_t;
        let mut __oldSize_0: darray_size_t = (*ls).size;
        (*ls).size = __oldSize_0.wrapping_add(__count_0);
        let mut __need_0: darray_size_t = (*ls).size;
        if __need_0 > (*ls).alloc {
            (*ls).alloc = darray_next_alloc(
                (*ls).alloc,
                __need_0,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            );
            (*ls).item = realloc(
                (*ls).item as *mut ::core::ffi::c_void,
                ((*ls).alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
        }
        memcpy(
            (*ls).item.offset(__oldSize_0 as isize) as *mut ::core::ffi::c_void,
            b": \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            (__count_0 as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        (*ls).size = (*ls).size.wrapping_sub(1);
        let mut __str_0: *const ::core::ffi::c_char = s;
        let mut __count_1: darray_size_t =
            (strlen(__str_0) as darray_size_t).wrapping_add(1 as darray_size_t);
        let mut __oldSize_1: darray_size_t = (*ls).size;
        (*ls).size = __oldSize_1.wrapping_add(__count_1);
        let mut __need_1: darray_size_t = (*ls).size;
        if __need_1 > (*ls).alloc {
            (*ls).alloc = darray_next_alloc(
                (*ls).alloc,
                __need_1,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            );
            (*ls).item = realloc(
                (*ls).item as *mut ::core::ffi::c_void,
                ((*ls).alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
        }
        memcpy(
            (*ls).item.offset(__oldSize_1 as isize) as *mut ::core::ffi::c_void,
            __str_0 as *const ::core::ffi::c_void,
            (__count_1 as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        (*ls).size = (*ls).size.wrapping_sub(1);
        free(s as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn test_basic() {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        ret = setenv(
            b"XKB_LOG_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
            b"warn\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        if ret == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"ret == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                61 as ::core::ffi::c_uint,
                b"void test_basic(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        ret = setenv(
            b"XKB_LOG_VERBOSITY\0".as_ptr() as *const ::core::ffi::c_char,
            b"5\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        if ret == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"ret == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                63 as ::core::ffi::c_uint,
                b"void test_basic(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_uint,
                b"void test_basic(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut log_string: darray_char = darray_char {
            size: 0,
            alloc: 0,
            item: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        log_string.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
        log_string.size = 0 as darray_size_t;
        log_string.alloc = 0 as darray_size_t;
        xkb_context_set_user_data(ctx, &raw mut log_string as *mut ::core::ffi::c_void);
        xkb_context_set_log_fn(
            ctx,
            Some(
                log_fn
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_log_level,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            ),
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_WARNING,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"first warning: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            87 as ::core::ffi::c_int,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_INFO,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"first info\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"first debug: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"hello\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"first error: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            115415 as ::core::ffi::c_ulong,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_WARNING,
            5 as ::core::ffi::c_int,
            b"first verbose 5\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_context_set_log_level(ctx, XKB_LOG_LEVEL_DEBUG);
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_WARNING,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"second warning: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            87 as ::core::ffi::c_int,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"second debug: %s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"hello\0".as_ptr() as *const ::core::ffi::c_char,
            b"world\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_INFO,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"second info\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"[XKB-%03d] second error: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_ERROR_MALFORMED_NUMBER_LITERAL as ::core::ffi::c_int,
            115415 as ::core::ffi::c_ulong,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_WARNING,
            6 as ::core::ffi::c_int,
            b"second verbose 6\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_context_set_log_verbosity(ctx, XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int);
        xkb_context_set_log_level(ctx, XKB_LOG_LEVEL_CRITICAL);
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_WARNING,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"third warning: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            87 as ::core::ffi::c_int,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"third debug: %s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"hello\0".as_ptr() as *const ::core::ffi::c_char,
            b"world\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_INFO,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"third info\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"third error: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            115415 as ::core::ffi::c_ulong,
        );
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_WARNING,
            0 as ::core::ffi::c_int,
            b"third verbose 0\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        printf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            log_string.item,
        );
        if streq(
            log_string.item,
            b"warning: first warning: 87\nerror: first error: 115415\nwarning: first verbose 5\nwarning: second warning: 87\ndebug: second debug: hello world\ninfo: second info\nerror: [XKB-034] second error: 115415\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int != 0
        {} else {
            __assert_fail(
                b"streq(darray_items(log_string), \"warning: first warning: 87\\n\" \"error: first error: 115415\\n\" \"warning: first verbose 5\\n\" \"warning: second warning: 87\\n\" \"debug: second debug: hello world\\n\" \"info: second info\\n\" \"error: [XKB-034] second error: 115415\\n\")\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_uint,
                b"void test_basic(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_context_unref(ctx);
        free(log_string.item as *mut ::core::ffi::c_void);
        log_string.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
        log_string.size = 0 as darray_size_t;
        log_string.alloc = 0 as darray_size_t;
    }
}
unsafe extern "C" fn test_keymaps() {
    unsafe {
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                119 as ::core::ffi::c_uint,
                b"void test_keymaps(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut log_string: darray_char = darray_char {
            size: 0,
            alloc: 0,
            item: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        log_string.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
        log_string.size = 0 as darray_size_t;
        log_string.alloc = 0 as darray_size_t;
        xkb_context_set_user_data(ctx, &raw mut log_string as *mut ::core::ffi::c_void);
        xkb_context_set_log_fn(
            ctx,
            Some(
                log_fn
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_log_level,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            ),
        );
        xkb_context_set_log_level(ctx, XKB_LOG_LEVEL_WARNING);
        xkb_context_set_log_verbosity(ctx, XKB_LOG_VERBOSITY_COMPREHENSIVE as ::core::ffi::c_int);
        let keymaps: [test_data; 14] = [
            test_data {
                input: b"\0".as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-822] Failed to parse input xkb string\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b" \0".as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-822] Failed to parse input xkb string\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-822] Failed to parse input xkb string\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"xkb_keymap {\n\0".as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-769] (input string):1:12: syntax error, unexpected end of file\nerror: [XKB-822] Failed to parse input xkb string\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"xkb_keymap \"\\j\"\n { xkb_symbols = {};\n};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log: b"warning: [XKB-645] (input string):1:12: unknown escape sequence \"\\j\" in string literal\nerror: [XKB-769] (input string):2:16: syntax error, unexpected =, expecting {\nerror: [XKB-822] Failed to parse input xkb string\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"xkb_keymap {\n xkb_keycodes { <> = 1; };\n xkb_symbols { key <> { [\"\xC3\xBC\xFF\"] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-542] (input string):3:26: Cannot convert string to keysyms: Invalid UTF-8 encoding starting at byte position 3 (code point position: 2).\nerror: [XKB-822] Failed to parse input xkb string\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"xkb_keymap {\n  xkb_keycodes {\n    <> = 1;\n\n    alias <1> = <>;\n    alias <1> =\n                <>;\n  };\n  xkb_types \"\\400x\\j\" { };\n  xkb_compat {\n    interpret invalidKeysym +\n                              Any { repeat = true; };\n  };\n  xkb_symbols { key <> {[0x30, leftshoe]}; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"warning: [XKB-193] (input string):9:13: invalid octal escape sequence \"\\400\" in string literal\nwarning: [XKB-645] (input string):9:13: unknown escape sequence \"\\j\" in string literal\nwarning: [XKB-107] (input string):11:15: unrecognized keysym \"invalidKeysym\"\nwarning: [XKB-489] (input string):14:26: numeric keysym \"0x0030\" (48)\nwarning: [XKB-301] (input string):14:32: deprecated keysym \"leftshoe\".\nwarning: [XKB-433] No map in include statement, but \"(input string)\" contains several; Using first defined map, \"(unnamed map)\"\nwarning: [XKB-523] Alias of <1> for <> declared more than once; First definition ignored\nwarning: [XKB-286] The type \"TWO_LEVEL\" for key '<>' group 1 was not previously defined; Using the default type\nwarning: [XKB-516] Type \"ONE_LEVEL\" has 1 levels, but <> has 2 levels; Ignoring extra symbols\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: false,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_compat {\n    NoAction.data = 1;\n    interpret VoidSymbol { action= VoidAction(data=<garbage>); };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-563] The \"NoAction\" action takes no argument, but got \"data\" field; Action definition ignored\nerror: [XKB-563] The \"VoidAction\" action takes no argument, but got \"data\" field; Action definition ignored\nerror: Failed to compile xkb_compatibility\nerror: [XKB-822] Failed to compile keymap\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols {\n    NoAction.data = 1;\n    key <> { [TerminateServer(data=<garbage>)] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-563] The \"NoAction\" action takes no argument, but got \"data\" field; Action definition ignored\nerror: [XKB-563] The \"Terminate\" action takes no argument, but got \"data\" field; Action definition ignored\nerror: [XKB-796] Illegal action definition for <>; Action for group 1/level 1 ignored\nerror: Failed to compile xkb_symbols\nerror: [XKB-822] Failed to compile keymap\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_compat {\n    interpret ISO_Lock+AnyOf(all) {\n      action= ISOLock(modifiers=modMapMods,affect=all);\n    };\n    interpret VoidSymbol+AnyOf(all) {\n      action= DeviceButton(data=<garbage>);\n    };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"warning: [XKB-362] Unsupported legacy action type \"ISOLock\".\nwarning: [XKB-362] Unsupported legacy action type \"DeviceButton\".\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: false,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_keycodes { foobar; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-639] Default defined for unknown field \"foobar\"; Ignored\nerror: Failed to compile xkb_keycodes\nerror: [XKB-822] Failed to compile keymap\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_types { foobar; };\n};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log: b"error: [XKB-639] Default defined for unknown field \"foobar\"; Ignored\nerror: Failed to compile xkb_types\nerror: [XKB-822] Failed to compile keymap\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_compat { foobar; };\n};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log: b"error: [XKB-639] Default defined for unknown field \"foobar\"; Ignored\nerror: Failed to compile xkb_compatibility\nerror: [XKB-822] Failed to compile keymap\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"default xkb_keymap {\n  xkb_symbols { foobar; };\n};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log: b"error: [XKB-639] Default defined for unknown field \"foobar\"; Ignored\nerror: Failed to compile xkb_symbols\nerror: [XKB-822] Failed to compile keymap\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_keymaps\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut keymap: *mut xkb_keymap = test_compile_buffer(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                keymaps[k as usize].input,
                strlen(keymaps[k as usize].input),
            );
            let __cond: bool = keymaps[k as usize].error as ::core::ffi::c_int
                ^ !keymap.is_null() as ::core::ffi::c_int
                != 0;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    log_string.item,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                        312 as ::core::ffi::c_uint,
                        b"void test_keymaps(void)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            }
            xkb_keymap_unref(keymap);
            let __cond_0: bool = streq_not_null(log_string.item, keymaps[k as usize].log) as bool;
            if !__cond_0 {
                fprintf(
                    stderr,
                    b"Assertion failure: Expected:\n%s\nGot:\n%s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    keymaps[k as usize].log,
                    log_string.item,
                );
                if __cond_0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                        316 as ::core::ffi::c_uint,
                        b"void test_keymaps(void)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            }
            free(log_string.item as *mut ::core::ffi::c_void);
            log_string.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
            log_string.size = 0 as darray_size_t;
            log_string.alloc = 0 as darray_size_t;
            k = k.wrapping_add(1);
        }
        xkb_context_unref(ctx);
    }
}
unsafe extern "C" fn test_compose() {
    unsafe {
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                327 as ::core::ffi::c_uint,
                b"void test_compose(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut log_string: darray_char = darray_char {
            size: 0,
            alloc: 0,
            item: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        log_string.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
        log_string.size = 0 as darray_size_t;
        log_string.alloc = 0 as darray_size_t;
        xkb_context_set_user_data(ctx, &raw mut log_string as *mut ::core::ffi::c_void);
        xkb_context_set_log_fn(
            ctx,
            Some(
                log_fn
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_log_level,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            ),
        );
        xkb_context_set_log_level(ctx, XKB_LOG_LEVEL_WARNING);
        xkb_context_set_log_verbosity(ctx, XKB_LOG_VERBOSITY_VERBOSE as ::core::ffi::c_int);
        let composes: [test_data; 7] = [
            test_data {
                input: b"\0".as_ptr() as *const ::core::ffi::c_char,
                log: ::core::ptr::null::<::core::ffi::c_char>(),
                error: false,
            },
            test_data {
                input: b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                log: ::core::ptr::null::<::core::ffi::c_char>(),
                error: false,
            },
            test_data {
                input: b"\xFF\n\0".as_ptr() as *const ::core::ffi::c_char,
                log: b"error: [XKB-542] (input string):1:1: unexpected non-ASCII character.\nerror: [XKB-542] (input string):1:1: This could be a file encoding issue. Supported file encodings are ASCII and UTF-8.\nerror: (input string):1:1: failed to parse file\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"<leftshoe> : x\ninclude \"x\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log: b"warning: [XKB-301] (input string):1:1: deprecated keysym \"leftshoe\".\nerror: (input string):2:9: failed to open included Compose file \"x\": No such file or directory\nerror: (input string):2:9: failed to parse file\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: true,
            },
            test_data {
                input: b"<a> : \"a\"\n\n<b> : \"i\\j\\xk\n<0x30> : \"\\400\" invalidKeysym\n<0> <1> <2> <3> <4> <5> <6> <7> <8> <9> <leftshoe> : \"\"\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"warning: [XKB-645] (input string):3:7: unknown escape sequence \"\\j\" in string literal\nwarning: [XKB-193] (input string):3:7: illegal hexadecimal escape sequence \"\\x\" in string literal\nerror: [XKB-685] (input string):3:7: unterminated string literal\nwarning: [XKB-193] (input string):4:10: illegal octal escape sequence \"\\400\" in string literal\nerror: (input string):4:17: unrecognized keysym \"invalidKeysym\" on right-hand side\nwarning: [XKB-301] (input string):5:41: deprecated keysym \"leftshoe\".\nwarning: [XKB-685] (input string):5:41: too many keysyms (11) on left-hand side; skipping line\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: false,
            },
            test_data {
                input: b":\n<a> :\n#\n<c> : \"a\" \"b\"\n<d> : a b\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log: b"warning: (input string):1:1: expected at least one keysym on left-hand side; skipping line\nwarning: [XKB-685] (input string):2:5: right-hand side must have at least one of string or keysym; skipping line\nwarning: (input string):4:11: right-hand side can have at most one string; skipping line\nerror: [XKB-685] (input string):5:9: unrecognized modifier \"b\"\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: false,
            },
            test_data {
                input: b"<a> : a\n<a> : a\n<b>     : b\n<b> <c> : x\n<c> <d> : y\n<c>     : c\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                log: b"warning: (input string):2:7: this compose sequence is a duplicate of another; skipping line\nwarning: (input string):4:11: a sequence already exists which is a prefix of this sequence; overriding\nwarning: (input string):6:11: this compose sequence is a prefix of another; overriding\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                error: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_compose\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut table: *mut xkb_compose_table = xkb_compose_table_new_from_buffer(
                ctx,
                composes[k as usize].input,
                strlen(composes[k as usize].input),
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_COMPOSE_FORMAT_TEXT_V1,
                XKB_COMPOSE_COMPILE_NO_FLAGS,
            );
            if composes[k as usize].error as ::core::ffi::c_int
                ^ !table.is_null() as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"composes[k].error ^ !!table\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                    420 as ::core::ffi::c_uint,
                    b"void test_compose(void)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            xkb_compose_table_unref(table);
            let __cond: bool = streq_null(log_string.item, composes[k as usize].log) as bool;
            if !__cond {
                fprintf(
                    stderr,
                    b"Assertion failure: Expected:\n%s\nGot:\n%s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    log_string.item,
                    composes[k as usize].log,
                );
                if __cond as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/log.c\0".as_ptr() as *const ::core::ffi::c_char,
                        424 as ::core::ffi::c_uint,
                        b"void test_compose(void)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            }
            free(log_string.item as *mut ::core::ffi::c_void);
            log_string.item = ::core::ptr::null_mut::<::core::ffi::c_char>();
            log_string.size = 0 as darray_size_t;
            log_string.alloc = 0 as darray_size_t;
            k = k.wrapping_add(1);
        }
        xkb_context_unref(ctx);
    }
}
unsafe fn main_0() -> ::core::ffi::c_int {
    unsafe {
        test_init();
        setlocale(LC_ALL, b"C\0".as_ptr() as *const ::core::ffi::c_char);
        test_basic();
        test_keymaps();
        test_compose();
        return EXIT_SUCCESS;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
