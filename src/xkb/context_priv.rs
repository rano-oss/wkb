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
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod stdio_h {
    pub type va_list = __gnuc_va_list;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;
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
        pub fn xkb_context_include_path_get_system_path(
            ctx: *mut xkb_context,
        ) -> *const ::core::ffi::c_char;
    }
}
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    use super::__stddef_size_t_h::size_t;
    use super::darray_h::darray_size_t;
    extern "C" {
        pub type atom_table;
        pub fn atom_table_size(table: *mut atom_table) -> darray_size_t;
        pub fn atom_intern(
            table: *mut atom_table,
            string: *const ::core::ffi::c_char,
            len: size_t,
            add: bool,
        ) -> xkb_atom_t;
        pub fn atom_text(table: *mut atom_table, atom: xkb_atom_t) -> *const ::core::ffi::c_char;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
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
    use super::context_h::xkb_context;
    extern "C" {
        pub fn xkb_context_include_path_append_default(
            context: *mut xkb_context,
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
pub mod rmlvo_h {
    pub type RMLVO = ::core::ffi::c_uint;
    pub const RMLVO_OPTIONS: RMLVO = 16;
    pub const RMLVO_VARIANT: RMLVO = 8;
    pub const RMLVO_LAYOUT: RMLVO = 4;
    pub const RMLVO_MODEL: RMLVO = 2;
    pub const RMLVO_RULES: RMLVO = 1;
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
pub mod config_h {
    pub const DEFAULT_XKB_LAYOUT: [::core::ffi::c_char; 3] =
        unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"us\0") };
    pub const DEFAULT_XKB_MODEL: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"pc105\0") };
    pub const DEFAULT_XKB_OPTIONS: *mut ::core::ffi::c_void = NULL;
    pub const DEFAULT_XKB_RULES: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"evdev\0") };
    pub const DEFAULT_XKB_VARIANT: *mut ::core::ffi::c_void = NULL;
    use super::__stddef_null_h::NULL;
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
        pub fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn secure_getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn isempty(mut s: *const ::core::ffi::c_char) -> bool {
        unsafe {
            return s.is_null()
                || *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32;
        }
    }
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
pub use self::atom_h::{atom_intern, atom_table, atom_table_size, atom_text, xkb_atom_t};
pub use self::config_h::{
    DEFAULT_XKB_LAYOUT, DEFAULT_XKB_MODEL, DEFAULT_XKB_OPTIONS, DEFAULT_XKB_RULES,
    DEFAULT_XKB_VARIANT,
};
pub use self::context_h::{
    xkb_context, xkb_context_include_path_get_system_path, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
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
pub use self::rmlvo_h::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdio_h::va_list;
use self::stdlib_h::{getenv, secure_getenv};
use self::string_h::strlen;
pub use self::utils_h::isempty;
pub use self::xkbcommon_h::{
    xkb_context_include_path_append_default, xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
#[no_mangle]
pub unsafe extern "C" fn xkb_context_getenv(
    mut ctx: *mut xkb_context,
    mut name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if (*ctx).use_secure_getenv() {
            return secure_getenv(name);
        } else {
            return getenv(name);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_init_includes(mut ctx: *mut xkb_context) -> bool {
    unsafe {
        if (*ctx).pending_default_includes() {
            if (*ctx).failed_includes.size == 0 as darray_size_t {
                if xkb_context_include_path_append_default(ctx) == 0 {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Failed to add any default include path (system path: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as ::core::ffi::c_int,
                        xkb_context_include_path_get_system_path(ctx),
                    );
                    return false_0 != 0;
                }
                (*ctx).set_pending_default_includes((false_0 != 0) as bool);
            } else {
                return false_0 != 0;
            }
        }
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_num_failed_include_paths(
    mut ctx: *mut xkb_context,
) -> darray_size_t {
    unsafe {
        return if xkb_context_init_includes(ctx) as ::core::ffi::c_int != 0 {
            (*ctx).failed_includes.size
        } else {
            0 as darray_size_t
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_failed_include_path_get(
    mut ctx: *mut xkb_context,
    mut idx: darray_size_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        if idx >= xkb_context_num_failed_include_paths(ctx) {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return *(*ctx).failed_includes.item.offset(idx as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_atom_table_size(mut ctx: *mut xkb_context) -> darray_size_t {
    unsafe {
        return atom_table_size((*ctx).atom_table);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_atom_lookup(
    mut ctx: *mut xkb_context,
    mut string: *const ::core::ffi::c_char,
) -> xkb_atom_t {
    unsafe {
        return atom_intern((*ctx).atom_table, string, strlen(string), false_0 != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_atom_intern(
    mut ctx: *mut xkb_context,
    mut string: *const ::core::ffi::c_char,
    mut len: size_t,
) -> xkb_atom_t {
    unsafe {
        return atom_intern((*ctx).atom_table, string, len, true_0 != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_atom_text(
    mut ctx: *mut xkb_context,
    mut atom: xkb_atom_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        return atom_text((*ctx).atom_table, atom);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_log(
    mut ctx: *mut xkb_context,
    mut level: xkb_log_level,
    mut verbosity: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut args: ::core::ffi::VaList;
        if ((*ctx).log_level as ::core::ffi::c_uint) < level as ::core::ffi::c_uint
            || (*ctx).log_verbosity < verbosity
        {
            return;
        }
        args = c2rust_args.clone();
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, fmt, args);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_get_buffer(
    mut ctx: *mut xkb_context,
    mut size: size_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut rtrn: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if size >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize)
            .wrapping_sub((*ctx).text_next as usize)
            <= size
        {
            (*ctx).text_next = 0 as size_t;
        }
        rtrn = (&raw mut (*ctx).text_buffer as *mut ::core::ffi::c_char)
            .offset((*ctx).text_next as isize) as *mut ::core::ffi::c_char;
        (*ctx).text_next = (*ctx).text_next.wrapping_add(size);
        return rtrn;
    }
}
unsafe extern "C" fn xkb_context_get_default_rules(
    mut ctx: *mut xkb_context,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(
                ctx,
                b"XKB_DEFAULT_RULES\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return if !env.is_null() {
            env
        } else {
            DEFAULT_XKB_RULES.as_ptr()
        };
    }
}
unsafe extern "C" fn xkb_context_get_default_model(
    mut ctx: *mut xkb_context,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(
                ctx,
                b"XKB_DEFAULT_MODEL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return if !env.is_null() {
            env
        } else {
            DEFAULT_XKB_MODEL.as_ptr()
        };
    }
}
unsafe extern "C" fn xkb_context_get_default_layout(
    mut ctx: *mut xkb_context,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(
                ctx,
                b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return if !env.is_null() {
            env
        } else {
            DEFAULT_XKB_LAYOUT.as_ptr()
        };
    }
}
unsafe extern "C" fn xkb_context_get_default_variant(
    mut ctx: *mut xkb_context,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut layout: *const ::core::ffi::c_char = xkb_context_getenv(
            ctx,
            b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !layout.is_null() && (*ctx).use_environment_names() as ::core::ffi::c_int != 0 {
            env = xkb_context_getenv(
                ctx,
                b"XKB_DEFAULT_VARIANT\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return if !env.is_null() {
            env
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
}
unsafe extern "C" fn xkb_context_get_default_options(
    mut ctx: *mut xkb_context,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(
                ctx,
                b"XKB_DEFAULT_OPTIONS\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return if !env.is_null() {
            env
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_sanitize_rule_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *mut xkb_rule_names,
) -> RMLVO {
    unsafe {
        let mut modified: RMLVO = 0 as RMLVO;
        if isempty((*rmlvo).rules) {
            (*rmlvo).rules = xkb_context_get_default_rules(ctx);
            modified = (modified as ::core::ffi::c_uint
                | RMLVO_RULES as ::core::ffi::c_int as ::core::ffi::c_uint)
                as RMLVO;
        }
        if isempty((*rmlvo).model) {
            (*rmlvo).model = xkb_context_get_default_model(ctx);
            modified = (modified as ::core::ffi::c_uint
                | RMLVO_MODEL as ::core::ffi::c_int as ::core::ffi::c_uint)
                as RMLVO;
        }
        if isempty((*rmlvo).layout) {
            (*rmlvo).layout = xkb_context_get_default_layout(ctx);
            modified = (modified as ::core::ffi::c_uint
                | RMLVO_LAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint)
                as RMLVO;
            let variant: *const ::core::ffi::c_char =
                xkb_context_get_default_variant(ctx) as *const ::core::ffi::c_char;
            if !isempty((*rmlvo).variant) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Layout not provided, but variant set to \"%s\": ignoring variant and using defaults for both: layout=\"%s\", variant=\"%s\".\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*rmlvo).variant,
                    (*rmlvo).layout,
                    if !variant.is_null() {
                        variant
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
            }
            (*rmlvo).variant = variant;
            modified = (modified as ::core::ffi::c_uint
                | RMLVO_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint)
                as RMLVO;
        }
        if (*rmlvo).options.is_null() {
            (*rmlvo).options = xkb_context_get_default_options(ctx);
            modified = (modified as ::core::ffi::c_uint
                | RMLVO_OPTIONS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as RMLVO;
        }
        return modified;
    }
}
