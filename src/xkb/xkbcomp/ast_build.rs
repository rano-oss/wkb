use c2rust_bitfields;
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
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
}
pub mod stdint_intn_h {
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
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
    use super::internal::__va_list_tag;
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
    pub type xkb_atom_t = darray_size_t;
    use super::darray_h::darray_size_t;
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
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
    pub type xkb_keysym_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_component_names {
        pub keycodes: *mut ::core::ffi::c_char,
        pub compatibility: *mut ::core::ffi::c_char,
        pub geometry: *mut ::core::ffi::c_char,
        pub symbols: *mut ::core::ffi::c_char,
        pub types: *mut ::core::ffi::c_char,
    }
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        pub fn xkb_utf32_to_keysym(codepoint: uint32_t) -> xkb_keysym_t;
    }
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
    pub type stmt_type = ::core::ffi::c_uint;
    pub const _STMT_NUM_VALUES: stmt_type = 37;
    pub const STMT_UNKNOWN_COMPOUND: stmt_type = 36;
    pub const STMT_UNKNOWN_DECLARATION: stmt_type = 35;
    pub const STMT_LED_NAME: stmt_type = 34;
    pub const STMT_LED_MAP: stmt_type = 33;
    pub const STMT_GROUP_COMPAT: stmt_type = 32;
    pub const STMT_MODMAP: stmt_type = 31;
    pub const STMT_SYMBOLS: stmt_type = 30;
    pub const STMT_VMOD: stmt_type = 29;
    pub const STMT_INTERP: stmt_type = 28;
    pub const STMT_TYPE: stmt_type = 27;
    pub const STMT_VAR: stmt_type = 26;
    pub const STMT_EXPR_UNARY_PLUS: stmt_type = 25;
    pub const STMT_EXPR_INVERT: stmt_type = 24;
    pub const STMT_EXPR_NEGATE: stmt_type = 23;
    pub const STMT_EXPR_NOT: stmt_type = 22;
    pub const STMT_EXPR_ASSIGN: stmt_type = 21;
    pub const STMT_EXPR_DIVIDE: stmt_type = 20;
    pub const STMT_EXPR_MULTIPLY: stmt_type = 19;
    pub const STMT_EXPR_SUBTRACT: stmt_type = 18;
    pub const STMT_EXPR_ADD: stmt_type = 17;
    pub const STMT_EXPR_ACTION_LIST: stmt_type = 16;
    pub const STMT_EXPR_KEYSYM_LIST: stmt_type = 15;
    pub const STMT_EXPR_EMPTY_LIST: stmt_type = 14;
    pub const STMT_EXPR_ARRAY_REF: stmt_type = 13;
    pub const STMT_EXPR_FIELD_REF: stmt_type = 12;
    pub const STMT_EXPR_ACTION_DECL: stmt_type = 11;
    pub const STMT_EXPR_IDENT: stmt_type = 10;
    pub const STMT_EXPR_KEYSYM_LITERAL: stmt_type = 9;
    pub const STMT_EXPR_KEYNAME_LITERAL: stmt_type = 8;
    pub const STMT_EXPR_BOOLEAN_LITERAL: stmt_type = 7;
    pub const STMT_EXPR_FLOAT_LITERAL: stmt_type = 6;
    pub const STMT_EXPR_INTEGER_LITERAL: stmt_type = 5;
    pub const STMT_EXPR_STRING_LITERAL: stmt_type = 4;
    pub const STMT_ALIAS: stmt_type = 3;
    pub const STMT_KEYCODE: stmt_type = 2;
    pub const STMT_INCLUDE: stmt_type = 1;
    pub const STMT_UNKNOWN: stmt_type = 0;
    pub type merge_mode = ::core::ffi::c_uint;
    pub const _MERGE_MODE_NUM_ENTRIES: merge_mode = 4;
    pub const MERGE_REPLACE: merge_mode = 3;
    pub const MERGE_OVERRIDE: merge_mode = 2;
    pub const MERGE_AUGMENT: merge_mode = 1;
    pub const MERGE_DEFAULT: merge_mode = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ParseCommon {
        pub next: *mut _ParseCommon,
        pub type_0: stmt_type,
    }
    pub type ParseCommon = _ParseCommon;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _IncludeStmt {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub stmt: *mut ::core::ffi::c_char,
        pub file: *mut ::core::ffi::c_char,
        pub map: *mut ::core::ffi::c_char,
        pub modifier: *mut ::core::ffi::c_char,
        pub next_incl: *mut _IncludeStmt,
    }
    pub type IncludeStmt = _IncludeStmt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union ExprDef {
        pub common: ParseCommon,
        pub ident: ExprIdent,
        pub string: ExprString,
        pub boolean: ExprBoolean,
        pub integer: ExprInteger,
        pub key_name: ExprKeyName,
        pub keysym: ExprKeySym,
        pub binary: ExprBinary,
        pub unary: ExprUnary,
        pub field_ref: ExprFieldRef,
        pub array_ref: ExprArrayRef,
        pub action: ExprAction,
        pub actions: ExprActionList,
        pub keysym_list: ExprKeysymList,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeysymList {
        pub common: ParseCommon,
        pub syms: C2Rust_Unnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_1 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprActionList {
        pub common: ParseCommon,
        pub actions: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprAction {
        pub common: ParseCommon,
        pub name: xkb_atom_t,
        pub args: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprArrayRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
        pub entry: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprFieldRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprUnary {
        pub common: ParseCommon,
        pub child: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBinary {
        pub common: ParseCommon,
        pub left: *mut ExprDef,
        pub right: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeySym {
        pub common: ParseCommon,
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeyName {
        pub common: ParseCommon,
        pub key_name: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprInteger {
        pub common: ParseCommon,
        pub ival: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBoolean {
        pub common: ParseCommon,
        pub set: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprString {
        pub common: ParseCommon,
        pub str: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprIdent {
        pub common: ParseCommon,
        pub ident: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VarDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: *mut ExprDef,
        pub value: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VModDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub value: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct KeycodeDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub value: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct KeyAliasDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub alias: xkb_atom_t,
        pub real: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct KeyTypeDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub body: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct SymbolsDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub keyName: xkb_atom_t,
        pub symbols: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ModMapDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub modifier: xkb_atom_t,
        pub keys: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GroupCompatDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub group: int64_t,
        pub def: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct InterpDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub sym: xkb_keysym_t,
        pub match_0: *mut ExprDef,
        pub def: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LedNameDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub virtual_0: bool,
        pub ndx: int64_t,
        pub name: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LedMapDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub body: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct UnknownStatement {
        pub common: ParseCommon,
        pub name: *mut ::core::ffi::c_char,
    }
    pub type xkb_map_flags = ::core::ffi::c_uint;
    pub const MAP_IS_ALTGR: xkb_map_flags = 128;
    pub const MAP_HAS_FN: xkb_map_flags = 64;
    pub const MAP_HAS_KEYPAD: xkb_map_flags = 32;
    pub const MAP_HAS_MODIFIER: xkb_map_flags = 16;
    pub const MAP_HAS_ALPHANUMERIC: xkb_map_flags = 8;
    pub const MAP_IS_HIDDEN: xkb_map_flags = 4;
    pub const MAP_IS_PARTIAL: xkb_map_flags = 2;
    pub const MAP_IS_DEFAULT: xkb_map_flags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XkbFile {
        pub common: ParseCommon,
        pub name: *mut ::core::ffi::c_char,
        pub defs: *mut ParseCommon,
        pub file_type: xkb_file_type,
        pub flags: xkb_map_flags,
    }
    use super::atom_h::xkb_atom_t;
    use super::darray_h::darray_size_t;
    use super::stdint_intn_h::int64_t;
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod scanner_utils_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct sval {
        pub len: size_t,
        pub start: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct scanner_loc {
        pub line: size_t,
        pub column: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct scanner {
        pub pos: size_t,
        pub len: size_t,
        pub s: *const ::core::ffi::c_char,
        pub buf: [::core::ffi::c_char; 1024],
        pub buf_pos: size_t,
        pub token_pos: size_t,
        pub cached_pos: size_t,
        pub cached_loc: scanner_loc,
        pub file_name: *const ::core::ffi::c_char,
        pub ctx: *mut xkb_context,
        pub priv_0: *mut ::core::ffi::c_void,
    }
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    extern "C" {
        pub fn scanner_token_location(s: *mut scanner) -> scanner_loc;
    }
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn strndup(
            __string: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn strdup_safe(
        mut s: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        unsafe {
            return if !s.is_null() {
                strdup(s)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
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
    use super::__stddef_null_h::NULL;
    use super::string_h::strdup;
}
pub mod utf8_decoding_h {
    pub const INVALID_UTF8_CODE_POINT: ::core::ffi::c_uint = UINT32_MAX;
    use super::__stddef_size_t_h::size_t;
    use super::stdint_h::UINT32_MAX;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        pub fn utf8_next_code_point(
            s: *const ::core::ffi::c_char,
            max_size: size_t,
            size_out: *mut size_t,
        ) -> uint32_t;
    }
}
pub mod stdint_h {
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
pub mod keymap_h {
    extern "C" {
        pub fn XkbEscapeMapName(name: *mut ::core::ffi::c_char);
    }
}
pub mod include_h {
    pub const MERGE_AUGMENT_PREFIX: ::core::ffi::c_int = 124;
    pub const MERGE_REPLACE_PREFIX: ::core::ffi::c_int = 94;
    extern "C" {
        pub fn ParseIncludeMap(
            str_inout: *mut *mut ::core::ffi::c_char,
            file_rtrn: *mut *mut ::core::ffi::c_char,
            map_rtrn: *mut *mut ::core::ffi::c_char,
            nextop_rtrn: *mut ::core::ffi::c_char,
            extra_data: *mut *mut ::core::ffi::c_char,
        ) -> bool;
    }
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::ast_h::{
    stmt_type, xkb_file_type, C2Rust_Unnamed_1, ExprAction, ExprActionList, ExprArrayRef,
    ExprBinary, ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName,
    ExprKeySym, ExprKeysymList, ExprString, ExprUnary, IncludeStmt, KeyAliasDef, KeycodeDef,
    VModDef, VarDef, _ParseCommon, merge_mode, xkb_map_flags, GroupCompatDef, InterpDef,
    KeyTypeDef, LedMapDef, LedNameDef, ModMapDef, ParseCommon, SymbolsDef, UnknownStatement,
    XkbFile, _IncludeStmt, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID,
    FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES,
    FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN,
    MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL,
    MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN,
    STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF,
    STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT,
    STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY,
    STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT,
    STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP,
    STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES,
};
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
pub use self::include_h::{ParseIncludeMap, MERGE_AUGMENT_PREFIX, MERGE_REPLACE_PREFIX};
pub use self::internal::__va_list_tag;
use self::keymap_h::XkbEscapeMapName;
pub use self::messages_codes_h::{
    xkb_log_verbosity, xkb_message_code, XKB_ERROR_ABI_BACKWARD_COMPAT_,
    XKB_ERROR_ABI_FORWARD_COMPAT_, XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR,
    XKB_ERROR_CANNOT_RESOLVE_RMLVO, XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY,
    XKB_ERROR_EXPECTED_ARRAY_ENTRY, XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE,
    XKB_ERROR_INCLUDED_FILE_NOT_FOUND, XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT,
    XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT, XKB_ERROR_INSUFFICIENT_BUFFER_SIZE,
    XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD, XKB_ERROR_INVALID_COMPOSE_LOCALE,
    XKB_ERROR_INVALID_COMPOSE_SYNTAX, XKB_ERROR_INVALID_EXPRESSION_TYPE,
    XKB_ERROR_INVALID_FILE_ENCODING, XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
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
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
};
pub use self::scanner_utils_h::{scanner, scanner_loc, scanner_token_location, sval};
pub use self::stdint_h::UINT32_MAX;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::uint32_t;
use self::stdlib_h::{calloc, free, malloc, realloc};
use self::string_h::{strdup, strlen, strndup};
pub use self::types_h::{__int64_t, __uint32_t};
pub use self::utf8_decoding_h::{utf8_next_code_point, INVALID_UTF8_CODE_POINT};
pub use self::utils_h::{isempty, strdup_safe};
pub use self::xkbcommon_h::{
    xkb_component_names, xkb_keysym_t, xkb_log_level, xkb_rule_names, xkb_utf32_to_keysym,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
unsafe extern "C" fn ExprCreate(mut op: stmt_type) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef =
            malloc(::core::mem::size_of::<ExprDef>() as size_t) as *mut ExprDef;
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).common.type_0 = op;
        (*expr).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateString(mut str: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_STRING_LITERAL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).string.str = str;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateInteger(mut ival: int64_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_INTEGER_LITERAL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).integer.ival = ival;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateFloat() -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_FLOAT_LITERAL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateBoolean(mut set: bool) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_BOOLEAN_LITERAL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).boolean.set = set;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateKeyName(mut key_name: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_KEYNAME_LITERAL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).key_name.key_name = key_name;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateKeySym(mut keysym: xkb_keysym_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_KEYSYM_LITERAL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).keysym.keysym = keysym;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateIdent(mut ident: xkb_atom_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_IDENT);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).ident.ident = ident;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateUnary(
    mut op: stmt_type,
    mut child: *mut ExprDef,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(op);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).unary.child = child as *mut ExprDef;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateBinary(
    mut op: stmt_type,
    mut left: *mut ExprDef,
    mut right: *mut ExprDef,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(op);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).binary.left = left as *mut ExprDef;
        (*expr).binary.right = right as *mut ExprDef;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateFieldRef(
    mut element: xkb_atom_t,
    mut field: xkb_atom_t,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_FIELD_REF);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).field_ref.element = element;
        (*expr).field_ref.field = field;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateArrayRef(
    mut element: xkb_atom_t,
    mut field: xkb_atom_t,
    mut entry: *mut ExprDef,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_ARRAY_REF);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).array_ref.element = element;
        (*expr).array_ref.field = field;
        (*expr).array_ref.entry = entry as *mut ExprDef;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprEmptyList() -> *mut ExprDef {
    unsafe {
        return ExprCreate(STMT_EXPR_EMPTY_LIST);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateAction(
    mut name: xkb_atom_t,
    mut args: *mut ExprDef,
) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_ACTION_DECL);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).action.name = name;
        (*expr).action.args = args as *mut ExprDef;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateActionList(mut actions: *mut ExprDef) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_ACTION_LIST);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).actions.actions = actions as *mut ExprDef;
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprCreateKeySymList(mut sym: xkb_keysym_t) -> *mut ExprDef {
    unsafe {
        let mut expr: *mut ExprDef = ExprCreate(STMT_EXPR_KEYSYM_LIST);
        if expr.is_null() {
            return ::core::ptr::null_mut::<ExprDef>();
        }
        (*expr).keysym_list.syms.item = ::core::ptr::null_mut::<xkb_keysym_t>();
        (*expr).keysym_list.syms.size = 0 as darray_size_t;
        (*expr).keysym_list.syms.alloc = 0 as darray_size_t;
        if !(sym == XKB_KEY_NoSymbol as xkb_keysym_t) {
            (*expr).keysym_list.syms.size = (*expr)
                .keysym_list
                .syms
                .size
                .wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = (*expr).keysym_list.syms.size;
            if __need > (*expr).keysym_list.syms.alloc {
                (*expr).keysym_list.syms.alloc = darray_next_alloc(
                    (*expr).keysym_list.syms.alloc,
                    __need,
                    ::core::mem::size_of::<xkb_keysym_t>() as size_t,
                );
                (*expr).keysym_list.syms.item = realloc(
                    (*expr).keysym_list.syms.item as *mut ::core::ffi::c_void,
                    ((*expr).keysym_list.syms.alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<xkb_keysym_t>() as size_t),
                ) as *mut xkb_keysym_t;
            }
            *(*expr).keysym_list.syms.item.offset(
                (*expr)
                    .keysym_list
                    .syms
                    .size
                    .wrapping_sub(1 as darray_size_t) as isize,
            ) = sym;
        }
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprAppendKeySymList(
    mut expr: *mut ExprDef,
    mut sym: xkb_keysym_t,
) -> *mut ExprDef {
    unsafe {
        if !(sym == XKB_KEY_NoSymbol as xkb_keysym_t) {
            (*expr).keysym_list.syms.size = (*expr)
                .keysym_list
                .syms
                .size
                .wrapping_add(1 as darray_size_t);
            let mut __need: darray_size_t = (*expr).keysym_list.syms.size;
            if __need > (*expr).keysym_list.syms.alloc {
                (*expr).keysym_list.syms.alloc = darray_next_alloc(
                    (*expr).keysym_list.syms.alloc,
                    __need,
                    ::core::mem::size_of::<xkb_keysym_t>() as size_t,
                );
                (*expr).keysym_list.syms.item = realloc(
                    (*expr).keysym_list.syms.item as *mut ::core::ffi::c_void,
                    ((*expr).keysym_list.syms.alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<xkb_keysym_t>() as size_t),
                ) as *mut xkb_keysym_t;
            }
            *(*expr).keysym_list.syms.item.offset(
                (*expr)
                    .keysym_list
                    .syms
                    .size
                    .wrapping_sub(1 as darray_size_t) as isize,
            ) = sym;
        }
        return expr;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExprKeySymListAppendString(
    mut scanner: *mut scanner,
    mut expr: *mut ExprDef,
    mut string: *const ::core::ffi::c_char,
) -> *mut ExprDef {
    unsafe {
        let mut c2rust_current_block: u64;
        let len: size_t = strlen(string) as size_t;
        let mut idx: size_t = 0 as size_t;
        let mut idx_cp: size_t = 1 as size_t;
        loop {
            if !(idx < len) {
                c2rust_current_block = 18317007320854588510;
                break;
            }
            let mut count: size_t = 0 as size_t;
            let mut cp: uint32_t = utf8_next_code_point(
                string.offset(idx as isize),
                len.wrapping_sub(idx),
                &raw mut count,
            );
            if cp == INVALID_UTF8_CODE_POINT as uint32_t {
                let mut loc: scanner_loc = scanner_token_location(scanner);
                xkb_log(
                    (*scanner).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] %s:%zu:%zu: Cannot convert string to keysyms: Invalid UTF-8 encoding starting at byte position %zu (code point position: %zu).\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                    (*scanner).file_name,
                    loc.line,
                    loc.column,
                    idx.wrapping_add(1 as size_t),
                    idx_cp,
                );
                c2rust_current_block = 5140853804782746302;
                break;
            } else {
                let sym: xkb_keysym_t = xkb_utf32_to_keysym(cp) as xkb_keysym_t;
                if sym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    let mut loc_0: scanner_loc = scanner_token_location(scanner);
                    xkb_log(
                        (*scanner).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"%s:%zu:%zu: Cannot convert string to keysyms: Unicode code point U+04%X has no keysym equivalent(byte position: %zu, code point position: %zu).\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*scanner).file_name,
                        loc_0.line,
                        loc_0.column,
                        cp,
                        idx.wrapping_add(1 as size_t),
                        idx_cp,
                    );
                    c2rust_current_block = 5140853804782746302;
                    break;
                } else {
                    (*expr).keysym_list.syms.size = (*expr)
                        .keysym_list
                        .syms
                        .size
                        .wrapping_add(1 as darray_size_t);
                    let mut __need: darray_size_t = (*expr).keysym_list.syms.size;
                    if __need > (*expr).keysym_list.syms.alloc {
                        (*expr).keysym_list.syms.alloc = darray_next_alloc(
                            (*expr).keysym_list.syms.alloc,
                            __need,
                            ::core::mem::size_of::<xkb_keysym_t>() as size_t,
                        );
                        (*expr).keysym_list.syms.item = realloc(
                            (*expr).keysym_list.syms.item as *mut ::core::ffi::c_void,
                            ((*expr).keysym_list.syms.alloc as size_t)
                                .wrapping_mul(::core::mem::size_of::<xkb_keysym_t>() as size_t),
                        )
                            as *mut xkb_keysym_t;
                    }
                    *(*expr).keysym_list.syms.item.offset(
                        (*expr)
                            .keysym_list
                            .syms
                            .size
                            .wrapping_sub(1 as darray_size_t) as isize,
                    ) = sym;
                    idx = idx.wrapping_add(count);
                    idx_cp = idx_cp.wrapping_add(1);
                }
            }
        }
        match c2rust_current_block {
            5140853804782746302 => {
                FreeStmt(expr as *mut ParseCommon);
                return ::core::ptr::null_mut::<ExprDef>();
            }
            _ => {
                if *string.offset(idx as isize) as ::core::ffi::c_int == '\0' as i32 {
                } else {
                    __assert_fail(
                        b"string[idx] == '\\0'\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/ast-build.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        242 as ::core::ffi::c_uint,
                        b"ExprDef *ExprKeySymListAppendString(struct scanner *, ExprDef *, const char *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                return expr;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn KeysymParseString(
    mut scanner: *mut scanner,
    mut string: *const ::core::ffi::c_char,
) -> xkb_keysym_t {
    unsafe {
        let len: size_t = strlen(string) as size_t;
        if len == 0 as size_t {
            let mut loc: scanner_loc = scanner_token_location(scanner);
            xkb_log(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s:%zu:%zu: Cannot convert string to single keysym: empty string.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*scanner).file_name,
                loc.line,
                loc.column,
            );
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let mut count: size_t = 0 as size_t;
        let cp: uint32_t = utf8_next_code_point(string, len, &raw mut count) as uint32_t;
        if cp == INVALID_UTF8_CODE_POINT as uint32_t {
            let mut loc_0: scanner_loc = scanner_token_location(scanner);
            xkb_log(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: Cannot convert string to single keysym: Invalid UTF-8 encoding.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                (*scanner).file_name,
                loc_0.line,
                loc_0.column,
            );
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        } else if count != len {
            let mut loc_1: scanner_loc = scanner_token_location(scanner);
            xkb_log(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] %s:%zu:%zu: Cannot convert string to single keysym: Expected a single Unicode code point, got: \"%s\".\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                (*scanner).file_name,
                loc_1.line,
                loc_1.column,
                string,
            );
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let sym: xkb_keysym_t = xkb_utf32_to_keysym(cp) as xkb_keysym_t;
        if sym == XKB_KEY_NoSymbol as xkb_keysym_t {
            let mut loc_2: scanner_loc = scanner_token_location(scanner);
            xkb_log(
                (*scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s:%zu:%zu: Cannot convert string to single keysym: Unicode code point U+%04X has no keysym equivalent.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*scanner).file_name,
                loc_2.line,
                loc_2.column,
                cp,
            );
        }
        return sym;
    }
}
#[no_mangle]
pub unsafe extern "C" fn KeycodeCreate(
    mut name: xkb_atom_t,
    mut value: int64_t,
) -> *mut KeycodeDef {
    unsafe {
        let mut def: *mut KeycodeDef =
            malloc(::core::mem::size_of::<KeycodeDef>() as size_t) as *mut KeycodeDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<KeycodeDef>();
        }
        (*def).common.type_0 = STMT_KEYCODE;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).name = name;
        (*def).value = value;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn KeyAliasCreate(
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> *mut KeyAliasDef {
    unsafe {
        let mut def: *mut KeyAliasDef =
            malloc(::core::mem::size_of::<KeyAliasDef>() as size_t) as *mut KeyAliasDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<KeyAliasDef>();
        }
        (*def).common.type_0 = STMT_ALIAS;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).alias = alias;
        (*def).real = real;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VModCreate(mut name: xkb_atom_t, mut value: *mut ExprDef) -> *mut VModDef {
    unsafe {
        let mut def: *mut VModDef =
            malloc(::core::mem::size_of::<VModDef>() as size_t) as *mut VModDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<VModDef>();
        }
        (*def).common.type_0 = STMT_VMOD;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).name = name;
        (*def).value = value as *mut ExprDef;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VarCreate(mut name: *mut ExprDef, mut value: *mut ExprDef) -> *mut VarDef {
    unsafe {
        let mut def: *mut VarDef =
            malloc(::core::mem::size_of::<VarDef>() as size_t) as *mut VarDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<VarDef>();
        }
        (*def).common.type_0 = STMT_VAR;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).name = name as *mut ExprDef;
        (*def).value = value as *mut ExprDef;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BoolVarCreate(mut ident: xkb_atom_t, mut set: bool) -> *mut VarDef {
    unsafe {
        let mut name: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut value: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut def: *mut VarDef = ::core::ptr::null_mut::<VarDef>();
        name = ExprCreateIdent(ident);
        if name.is_null() {
            return ::core::ptr::null_mut::<VarDef>();
        }
        value = ExprCreateBoolean(set);
        if value.is_null() {
            FreeStmt(name as *mut ParseCommon);
            return ::core::ptr::null_mut::<VarDef>();
        }
        def = VarCreate(name, value);
        if def.is_null() {
            FreeStmt(name as *mut ParseCommon);
            FreeStmt(value as *mut ParseCommon);
            return ::core::ptr::null_mut::<VarDef>();
        }
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InterpCreate(
    mut sym: xkb_keysym_t,
    mut match_0: *mut ExprDef,
) -> *mut InterpDef {
    unsafe {
        let mut def: *mut InterpDef =
            malloc(::core::mem::size_of::<InterpDef>() as size_t) as *mut InterpDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<InterpDef>();
        }
        (*def).common.type_0 = STMT_INTERP;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).sym = sym;
        (*def).match_0 = match_0 as *mut ExprDef;
        (*def).def = ::core::ptr::null_mut::<VarDef>();
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn KeyTypeCreate(
    mut name: xkb_atom_t,
    mut body: *mut VarDef,
) -> *mut KeyTypeDef {
    unsafe {
        let mut def: *mut KeyTypeDef =
            malloc(::core::mem::size_of::<KeyTypeDef>() as size_t) as *mut KeyTypeDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<KeyTypeDef>();
        }
        (*def).common.type_0 = STMT_TYPE;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).merge = MERGE_DEFAULT;
        (*def).name = name;
        (*def).body = body;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SymbolsCreate(
    mut keyName: xkb_atom_t,
    mut symbols: *mut VarDef,
) -> *mut SymbolsDef {
    unsafe {
        let mut def: *mut SymbolsDef =
            malloc(::core::mem::size_of::<SymbolsDef>() as size_t) as *mut SymbolsDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<SymbolsDef>();
        }
        (*def).common.type_0 = STMT_SYMBOLS;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).merge = MERGE_DEFAULT;
        (*def).keyName = keyName;
        (*def).symbols = symbols;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GroupCompatCreate(
    mut group: int64_t,
    mut val: *mut ExprDef,
) -> *mut GroupCompatDef {
    unsafe {
        let mut def: *mut GroupCompatDef =
            malloc(::core::mem::size_of::<GroupCompatDef>() as size_t) as *mut GroupCompatDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<GroupCompatDef>();
        }
        (*def).common.type_0 = STMT_GROUP_COMPAT;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).merge = MERGE_DEFAULT;
        (*def).group = group;
        (*def).def = val as *mut ExprDef;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ModMapCreate(
    mut modifier: xkb_atom_t,
    mut keys: *mut ExprDef,
) -> *mut ModMapDef {
    unsafe {
        let mut def: *mut ModMapDef =
            malloc(::core::mem::size_of::<ModMapDef>() as size_t) as *mut ModMapDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<ModMapDef>();
        }
        (*def).common.type_0 = STMT_MODMAP;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).merge = MERGE_DEFAULT;
        (*def).modifier = modifier;
        (*def).keys = keys as *mut ExprDef;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn LedMapCreate(
    mut name: xkb_atom_t,
    mut body: *mut VarDef,
) -> *mut LedMapDef {
    unsafe {
        let mut def: *mut LedMapDef =
            malloc(::core::mem::size_of::<LedMapDef>() as size_t) as *mut LedMapDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<LedMapDef>();
        }
        (*def).common.type_0 = STMT_LED_MAP;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).merge = MERGE_DEFAULT;
        (*def).name = name;
        (*def).body = body;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn LedNameCreate(
    mut ndx: int64_t,
    mut name: *mut ExprDef,
    mut virtual_0: bool,
) -> *mut LedNameDef {
    unsafe {
        let mut def: *mut LedNameDef =
            malloc(::core::mem::size_of::<LedNameDef>() as size_t) as *mut LedNameDef;
        if def.is_null() {
            return ::core::ptr::null_mut::<LedNameDef>();
        }
        (*def).common.type_0 = STMT_LED_NAME;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).merge = MERGE_DEFAULT;
        (*def).ndx = ndx;
        (*def).name = name as *mut ExprDef;
        (*def).virtual_0 = virtual_0;
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn UnknownStatementCreate(
    mut type_0: stmt_type,
    mut name: sval,
) -> *mut UnknownStatement {
    unsafe {
        let mut def: *mut UnknownStatement =
            malloc(::core::mem::size_of::<UnknownStatement>() as size_t) as *mut UnknownStatement;
        if def.is_null() {
            return ::core::ptr::null_mut::<UnknownStatement>();
        }
        (*def).common.type_0 = type_0;
        (*def).common.next = ::core::ptr::null_mut::<_ParseCommon>();
        (*def).name = strndup(name.start, name.len);
        if (*def).name.is_null() {
            free(def as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<UnknownStatement>();
        }
        return def;
    }
}
#[no_mangle]
pub unsafe extern "C" fn IncludeCreate(
    mut ctx: *mut xkb_context,
    mut str: *mut ::core::ffi::c_char,
    mut merge: merge_mode,
) -> *mut IncludeStmt {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut incl: *mut IncludeStmt = ::core::ptr::null_mut::<IncludeStmt>();
        let mut first: *mut IncludeStmt = ::core::ptr::null_mut::<IncludeStmt>();
        let mut stmt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nextop: ::core::ffi::c_char = 0;
        first = ::core::ptr::null_mut::<IncludeStmt>();
        incl = first;
        tmp = str;
        stmt = strdup_safe(str);
        loop {
            if !(!tmp.is_null() && *tmp as ::core::ffi::c_int != 0) {
                c2rust_current_block = 15125582407903384992;
                break;
            }
            let mut file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut map: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut extra_data: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !ParseIncludeMap(
                &raw mut tmp,
                &raw mut file,
                &raw mut map,
                &raw mut nextop,
                &raw mut extra_data,
            ) {
                c2rust_current_block = 15017566315148339459;
                break;
            }
            if isempty(file) {
                free(file as *mut ::core::ffi::c_void);
                free(map as *mut ::core::ffi::c_void);
                free(extra_data as *mut ::core::ffi::c_void);
            } else {
                if first.is_null() {
                    incl =
                        malloc(::core::mem::size_of::<IncludeStmt>() as size_t) as *mut IncludeStmt;
                    first = incl;
                } else {
                    (*incl).next_incl = malloc(::core::mem::size_of::<IncludeStmt>() as size_t)
                        as *mut _IncludeStmt;
                    incl = (*incl).next_incl as *mut IncludeStmt;
                }
                if incl.is_null() {
                    free(file as *mut ::core::ffi::c_void);
                    free(map as *mut ::core::ffi::c_void);
                    free(extra_data as *mut ::core::ffi::c_void);
                    c2rust_current_block = 15125582407903384992;
                    break;
                } else {
                    (*incl).common.type_0 = STMT_INCLUDE;
                    (*incl).common.next = ::core::ptr::null_mut::<_ParseCommon>();
                    (*incl).merge = merge;
                    (*incl).stmt = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    (*incl).file = file;
                    (*incl).map = map;
                    (*incl).modifier = extra_data;
                    (*incl).next_incl = ::core::ptr::null_mut::<_IncludeStmt>();
                    match nextop as ::core::ffi::c_int {
                        MERGE_AUGMENT_PREFIX => {
                            merge = MERGE_AUGMENT;
                        }
                        MERGE_REPLACE_PREFIX => {
                            merge = MERGE_REPLACE;
                        }
                        _ => {
                            merge = MERGE_OVERRIDE;
                        }
                    }
                }
            }
        }
        match c2rust_current_block {
            15017566315148339459 => {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Illegal include statement \"%s\"; Ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    XKB_ERROR_INVALID_INCLUDE_STATEMENT as ::core::ffi::c_int,
                    stmt,
                );
                FreeInclude(first);
                free(stmt as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<IncludeStmt>();
            }
            _ => {
                if !first.is_null() {
                    (*first).stmt = stmt;
                } else {
                    free(stmt as *mut ::core::ffi::c_void);
                }
                return first;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbFileCreate(
    mut type_0: xkb_file_type,
    mut name: *mut ::core::ffi::c_char,
    mut defs: *mut ParseCommon,
    mut flags: xkb_map_flags,
) -> *mut XkbFile {
    unsafe {
        let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        file = calloc(1 as size_t, ::core::mem::size_of::<XkbFile>() as size_t) as *mut XkbFile;
        if file.is_null() {
            return ::core::ptr::null_mut::<XkbFile>();
        }
        XkbEscapeMapName(name);
        (*file).file_type = type_0;
        (*file).name = name;
        (*file).defs = defs;
        (*file).flags = flags;
        return file;
    }
}
#[no_mangle]
pub unsafe extern "C" fn XkbFileFromComponents(
    mut ctx: *mut xkb_context,
    mut kkctgs: *const xkb_component_names,
) -> *mut XkbFile {
    unsafe {
        let mut c2rust_current_block: u64;
        let components: [*mut ::core::ffi::c_char; 4] = [
            (*kkctgs).keycodes,
            (*kkctgs).types,
            (*kkctgs).compatibility,
            (*kkctgs).symbols,
        ];
        let mut type_0: xkb_file_type = FILE_TYPE_KEYCODES;
        let mut include: *mut IncludeStmt = ::core::ptr::null_mut::<IncludeStmt>();
        let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        let mut defs: *mut ParseCommon = ::core::ptr::null_mut::<ParseCommon>();
        let mut defsLast: *mut ParseCommon = ::core::ptr::null_mut::<ParseCommon>();
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        loop {
            if !(type_0 as ::core::ffi::c_uint
                <= LAST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                c2rust_current_block = 13536709405535804910;
                break;
            }
            include = IncludeCreate(ctx, components[type_0 as usize], MERGE_DEFAULT);
            if include.is_null() {
                c2rust_current_block = 183321912633560349;
                break;
            }
            file = XkbFileCreate(
                type_0,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                include as *mut ParseCommon,
                0 as xkb_map_flags,
            );
            if file.is_null() {
                FreeInclude(include);
                c2rust_current_block = 183321912633560349;
                break;
            } else {
                if defs.is_null() {
                    defs = &raw mut (*file).common;
                    defsLast = defs;
                } else {
                    (*defsLast).next = &raw mut (*file).common as *mut _ParseCommon;
                    defsLast = (*defsLast).next as *mut ParseCommon;
                }
                type_0 += 1;
            }
        }
        match c2rust_current_block {
            13536709405535804910 => {
                file = XkbFileCreate(
                    FILE_TYPE_KEYMAP,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    defs,
                    0 as xkb_map_flags,
                );
                if !file.is_null() {
                    return file;
                }
            }
            _ => {}
        }
        FreeXkbFile(defs as *mut XkbFile);
        return ::core::ptr::null_mut::<XkbFile>();
    }
}
unsafe extern "C" fn FreeInclude(mut incl: *mut IncludeStmt) {
    unsafe {
        let mut next: *mut IncludeStmt = ::core::ptr::null_mut::<IncludeStmt>();
        while !incl.is_null() {
            next = (*incl).next_incl as *mut IncludeStmt;
            free((*incl).file as *mut ::core::ffi::c_void);
            free((*incl).map as *mut ::core::ffi::c_void);
            free((*incl).modifier as *mut ::core::ffi::c_void);
            free((*incl).stmt as *mut ::core::ffi::c_void);
            free(incl as *mut ::core::ffi::c_void);
            incl = next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeStmt(mut stmt: *mut ParseCommon) {
    unsafe {
        let mut next: *mut ParseCommon = ::core::ptr::null_mut::<ParseCommon>();
        while !stmt.is_null() {
            next = (*stmt).next as *mut ParseCommon;
            match (*stmt).type_0 as ::core::ffi::c_uint {
                1 => {
                    FreeInclude(stmt as *mut IncludeStmt);
                    stmt = ::core::ptr::null_mut::<ParseCommon>();
                }
                23 | 25 | 22 | 24 => {
                    FreeStmt((*(stmt as *mut ExprUnary)).child as *mut ParseCommon);
                }
                20 | 17 | 18 | 19 | 21 => {
                    FreeStmt((*(stmt as *mut ExprBinary)).left as *mut ParseCommon);
                    FreeStmt((*(stmt as *mut ExprBinary)).right as *mut ParseCommon);
                }
                11 => {
                    FreeStmt((*(stmt as *mut ExprAction)).args as *mut ParseCommon);
                }
                16 => {
                    FreeStmt((*(stmt as *mut ExprActionList)).actions as *mut ParseCommon);
                }
                13 => {
                    FreeStmt((*(stmt as *mut ExprArrayRef)).entry as *mut ParseCommon);
                }
                15 => {
                    free((*(stmt as *mut ExprKeysymList)).syms.item as *mut ::core::ffi::c_void);
                    let ref mut c2rust_fresh0 = (*(stmt as *mut ExprKeysymList)).syms.item;
                    *c2rust_fresh0 = ::core::ptr::null_mut::<xkb_keysym_t>();
                    (*(stmt as *mut ExprKeysymList)).syms.size = 0 as darray_size_t;
                    (*(stmt as *mut ExprKeysymList)).syms.alloc = 0 as darray_size_t;
                }
                26 => {
                    FreeStmt((*(stmt as *mut VarDef)).name as *mut ParseCommon);
                    FreeStmt((*(stmt as *mut VarDef)).value as *mut ParseCommon);
                }
                27 => {
                    FreeStmt((*(stmt as *mut KeyTypeDef)).body as *mut ParseCommon);
                }
                28 => {
                    FreeStmt((*(stmt as *mut InterpDef)).match_0 as *mut ParseCommon);
                    FreeStmt((*(stmt as *mut InterpDef)).def as *mut ParseCommon);
                }
                29 => {
                    FreeStmt((*(stmt as *mut VModDef)).value as *mut ParseCommon);
                }
                30 => {
                    FreeStmt((*(stmt as *mut SymbolsDef)).symbols as *mut ParseCommon);
                }
                31 => {
                    FreeStmt((*(stmt as *mut ModMapDef)).keys as *mut ParseCommon);
                }
                32 => {
                    FreeStmt((*(stmt as *mut GroupCompatDef)).def as *mut ParseCommon);
                }
                33 => {
                    FreeStmt((*(stmt as *mut LedMapDef)).body as *mut ParseCommon);
                }
                34 => {
                    FreeStmt((*(stmt as *mut LedNameDef)).name as *mut ParseCommon);
                }
                35 | 36 => {
                    free((*(stmt as *mut UnknownStatement)).name as *mut ::core::ffi::c_void);
                }
                _ => {}
            }
            free(stmt as *mut ::core::ffi::c_void);
            stmt = next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeXkbFile(mut file: *mut XkbFile) {
    unsafe {
        let mut next: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        while !file.is_null() {
            next = (*file).common.next as *mut XkbFile;
            match (*file).file_type as ::core::ffi::c_uint {
                5 => {
                    FreeXkbFile((*file).defs as *mut XkbFile);
                }
                1 | 2 | 3 | 0 | 4 => {
                    FreeStmt((*file).defs);
                }
                _ => {}
            }
            free((*file).name as *mut ::core::ffi::c_void);
            free(file as *mut ::core::ffi::c_void);
            file = next;
        }
    }
}
static mut xkb_file_type_strings: [*const ::core::ffi::c_char; 7] = [
    b"xkb_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
    b"xkb_types\0".as_ptr() as *const ::core::ffi::c_char,
    b"xkb_compatibility\0".as_ptr() as *const ::core::ffi::c_char,
    b"xkb_symbols\0".as_ptr() as *const ::core::ffi::c_char,
    b"xkb_geometry\0".as_ptr() as *const ::core::ffi::c_char,
    b"xkb_keymap\0".as_ptr() as *const ::core::ffi::c_char,
    b"rules\0".as_ptr() as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn xkb_file_type_to_string(
    mut type_0: xkb_file_type,
) -> *const ::core::ffi::c_char {
    unsafe {
        if type_0 as ::core::ffi::c_uint
            >= _FILE_TYPE_NUM_ENTRIES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
        return xkb_file_type_strings[type_0 as usize];
    }
}
static mut stmt_type_strings: [*const ::core::ffi::c_char; 37] = [
    b"unknown statement\0".as_ptr() as *const ::core::ffi::c_char,
    b"include statement\0".as_ptr() as *const ::core::ffi::c_char,
    b"key name definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"key alias definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"string literal expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"integer literal expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"float literal expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"boolean literal expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"key name expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"keysym expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"identifier expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"action declaration expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"field reference expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"array reference expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"empty list expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"keysym list expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"action list expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"addition expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"substraction expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"multiplication expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"division expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"assignment expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"logical negation expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"arithmetic negation expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"bitwise inversion expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"unary plus expression\0".as_ptr() as *const ::core::ffi::c_char,
    b"variable definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"key type definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"symbol interpretation definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"virtual modifiers definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"key symbols definition\0".as_ptr() as *const ::core::ffi::c_char,
    b"modifier map declaration\0".as_ptr() as *const ::core::ffi::c_char,
    b"group declaration\0".as_ptr() as *const ::core::ffi::c_char,
    b"indicator map declaration\0".as_ptr() as *const ::core::ffi::c_char,
    b"indicator name declaration\0".as_ptr() as *const ::core::ffi::c_char,
    b"unknown declaration statement\0".as_ptr() as *const ::core::ffi::c_char,
    b"unknown compound statement\0".as_ptr() as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn stmt_type_to_string(mut type_0: stmt_type) -> *const ::core::ffi::c_char {
    unsafe {
        if type_0 as ::core::ffi::c_uint
            >= _STMT_NUM_VALUES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return stmt_type_strings[type_0 as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn stmt_type_to_operator_char(mut type_0: stmt_type) -> ::core::ffi::c_char {
    unsafe {
        match type_0 as ::core::ffi::c_uint {
            17 => return '+' as i32 as ::core::ffi::c_char,
            18 => return '-' as i32 as ::core::ffi::c_char,
            19 => return '*' as i32 as ::core::ffi::c_char,
            20 => return '/' as i32 as ::core::ffi::c_char,
            22 => return '!' as i32 as ::core::ffi::c_char,
            23 => return '-' as i32 as ::core::ffi::c_char,
            24 => return '~' as i32 as ::core::ffi::c_char,
            25 => return '+' as i32 as ::core::ffi::c_char,
            _ => return '\0' as i32 as ::core::ffi::c_char,
        };
    }
}
