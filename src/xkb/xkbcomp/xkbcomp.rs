pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
}

pub mod types_h {
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
}
pub mod stdint_intn_h {
    pub type i8 = __int8_t;
    pub type i16 = __int16_t;
    pub type i32 = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
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
pub mod darray_h {
    pub use crate::xkb::shared_types::darray_size_t;
}
pub mod context_h {
    pub use crate::xkb::context_priv::{
        xkb_context_get_buffer, xkb_context_sanitize_rule_names, RMLVO,
    };
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
    extern "C" {
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: i32,
            fmt: *const i8,
            ...
        );
    }
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
}
pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
        xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE,
        XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
        XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL,
        XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
        XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
        XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
        XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_component_names {
        pub keycodes: *mut i8,
        pub compatibility: *mut i8,
        pub geometry: *mut i8,
        pub symbols: *mut i8,
        pub types: *mut i8,
    }
    pub type xkb_keymap_serialize_flags = u32;
    pub const XKB_KEYMAP_SERIALIZE_EXPLICIT: xkb_keymap_serialize_flags = 4;
    pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
    pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
    pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
}
pub mod keymap_h {
    use super::rmlvo_h::xkb_rmlvo_builder;
    use super::FILE_h::FILE;
    pub use crate::xkb::shared_types::*;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_keymap_format_ops {
        pub keymap_new_from_rmlvo:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rmlvo_builder) -> bool>,
        pub keymap_new_from_names:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rule_names) -> bool>,
        pub keymap_new_from_string:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const i8, usize) -> bool>,
        pub keymap_new_from_file: Option<unsafe extern "C" fn(*mut xkb_keymap, *mut FILE) -> bool>,
        pub keymap_get_as_string: Option<
            unsafe extern "C" fn(
                *mut xkb_keymap,
                xkb_keymap_format,
                xkb_keymap_serialize_flags,
            ) -> *mut i8,
        >,
    }
    pub const XKB_MAX_GROUPS_X11: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    #[inline]
    pub unsafe fn format_max_groups(mut format: xkb_keymap_format) -> xkb_layout_index_t {
        unsafe {
            return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as u32 {
                XKB_MAX_GROUPS_X11
            } else {
                XKB_MAX_GROUPS
            }) as xkb_layout_index_t;
        }
    }
}
pub mod rmlvo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder {
        pub rules: *mut i8,
        pub model: *mut i8,
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
        pub option: *mut i8,
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
        pub layout: *mut i8,
        pub variant: *mut i8,
    }
    pub type RMLVO = u32;
    pub const RMLVO_OPTIONS: RMLVO = 16;
    pub const RMLVO_VARIANT: RMLVO = 8;
    pub const RMLVO_LAYOUT: RMLVO = 4;
    pub const RMLVO_MODEL: RMLVO = 2;
    pub const RMLVO_RULES: RMLVO = 1;

    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::xkbcommon_h::{xkb_layout_index_t, xkb_rule_names};
    extern "C" {
        pub fn xkb_rmlvo_builder_to_rules_names(
            builder: *const xkb_rmlvo_builder,
            rmlvo: *mut xkb_rule_names,
            buf: *mut i8,
            buf_size: usize,
        ) -> bool;
    }
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    extern "C" {
        pub fn xkb_file_type_to_string(type_0: xkb_file_type) -> *const i8;
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
pub mod rules_h {
    use super::context_h::xkb_context;
    use super::rmlvo_h::xkb_rmlvo_builder;
    use super::xkbcommon_h::{xkb_component_names, xkb_layout_index_t, xkb_rule_names};
    extern "C" {
        pub fn xkb_components_from_rmlvo_builder(
            rmlvo: *const xkb_rmlvo_builder,
            out: *mut xkb_component_names,
            explicit_layouts: *mut xkb_layout_index_t,
        ) -> bool;
        pub fn xkb_components_from_rules_names(
            ctx: *mut xkb_context,
            rmlvo: *const xkb_rule_names,
            out: *mut xkb_component_names,
            explicit_layouts: *mut xkb_layout_index_t,
        ) -> bool;
    }
}
pub mod xkbcomp_priv_h {

    use super::ast_h::XkbFile;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::xkbcommon_h::{xkb_component_names, xkb_keymap_format, xkb_keymap_serialize_flags};
    use super::FILE_h::FILE;

    // Stub implementation for text_v1_keymap_get_as_string (serialization not yet implemented)
    pub unsafe extern "C" fn text_v1_keymap_get_as_string(
        _keymap: *mut xkb_keymap,
        _format: xkb_keymap_format,
        _flags: xkb_keymap_serialize_flags,
    ) -> *mut i8 {
        // TODO: Implement keymap serialization (keymap-dump.c functionality)
        ::core::ptr::null_mut()
    }

    extern "C" {
        pub fn XkbParseFile(
            ctx: *mut xkb_context,
            file: *mut FILE,
            file_name: *const i8,
            map: *const i8,
        ) -> *mut XkbFile;
        pub fn XkbParseString(
            ctx: *mut xkb_context,
            string: *const i8,
            len: usize,
            file_name: *const i8,
            map: *const i8,
        ) -> *mut XkbFile;
        pub fn FreeXkbFile(file: *mut XkbFile);
        pub fn XkbFileFromComponents(
            ctx: *mut xkb_context,
            kkctgs: *const xkb_component_names,
        ) -> *mut XkbFile;
        pub fn CompileKeymap(file: *mut XkbFile, keymap: *mut xkb_keymap) -> bool;
    }
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
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::ast_h::{
    _ParseCommon, stmt_type, xkb_file_type, xkb_file_type_to_string, xkb_map_flags, ParseCommon,
    XkbFile, _FILE_TYPE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY,
    FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS,
    FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC,
    MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN,
    MAP_IS_PARTIAL, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD,
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
pub use self::context_h::{
    xkb_context, xkb_context_get_buffer, xkb_context_sanitize_rule_names, xkb_log, C2Rust_Unnamed,
    C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::keymap_h::{
    format_max_groups, mod_type, xkb_action, xkb_action_controls, xkb_action_count_t,
    xkb_action_flags, xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group,
    xkb_group_action, xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias,
    xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keymap_format_ops, xkb_keysym_count_t,
    xkb_led, xkb_level, xkb_match_operation, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods,
    xkb_overlay_mask_t, xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action,
    xkb_private_action, xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret,
    C2Rust_Unnamed_1, C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2,
    C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7,
    C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, _ACTION_TYPE_NUM_ENTRIES,
    ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL,
    ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK,
    ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP,
    ACTION_PENDING_COMPUTATION, ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET,
    ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL,
    ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE,
    ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK,
    ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE,
    ACTION_TYPE_UNKNOWN, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS,
    CONTROL_ALL, CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX,
    CONTROL_AX_FEEDBACK, CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_GROUPS,
    XKB_MAX_GROUPS_X11,
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
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options, xkb_rmlvo_builder_to_rules_names, RMLVO,
    RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
use self::rules_h::{xkb_components_from_rmlvo_builder, xkb_components_from_rules_names};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_intn_h::{i16, i32, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::free;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{
    __int16_t, __int32_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use self::xkbcommon_h::{
    xkb_component_names, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keymap_serialize_flags, xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t,
    xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t, xkb_log_level,
    xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_SERIALIZE_EXPLICIT, XKB_KEYMAP_SERIALIZE_KEEP_UNUSED,
    XKB_KEYMAP_SERIALIZE_NO_FLAGS, XKB_KEYMAP_SERIALIZE_PRETTY, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
    XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
    XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED,
    XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
};
use self::xkbcomp_priv_h::{
    text_v1_keymap_get_as_string, CompileKeymap, FreeXkbFile, XkbFileFromComponents, XkbParseFile,
    XkbParseString,
};
pub use self::FILE_h::FILE;

pub unsafe fn xkb_components_names_from_rules(
    mut ctx: *mut xkb_context,
    mut rmlvo_in: *const xkb_rule_names,
    mut rmlvo_out: *mut xkb_rule_names,
    mut components_out: *mut xkb_component_names,
) -> bool {
    unsafe {
        let mut rmlvo: xkb_rule_names = *rmlvo_in;
        xkb_context_sanitize_rule_names(ctx, &raw mut rmlvo);
        if !rmlvo_out.is_null() {
            *rmlvo_out = rmlvo;
        }
        if components_out.is_null() {
            return !rmlvo_out.is_null();
        }
        *components_out = xkb_component_names {
            keycodes: ::core::ptr::null_mut::<i8>(),
            compatibility: ::core::ptr::null_mut::<i8>(),
            geometry: ::core::ptr::null_mut::<i8>(),
            symbols: ::core::ptr::null_mut::<i8>(),
            types: ::core::ptr::null_mut::<i8>(),
        };
        return xkb_components_from_rules_names(
            ctx,
            &raw mut rmlvo,
            components_out,
            ::core::ptr::null_mut::<xkb_layout_index_t>(),
        );
    }
}
unsafe fn compile_keymap_file(mut keymap: *mut xkb_keymap, mut file: *mut XkbFile) -> bool {
    unsafe {
        if (*file).file_type as u32 != FILE_TYPE_KEYMAP as ::core::ffi::c_int as u32 {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot compile a %s file alone into a keymap\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
                xkb_file_type_to_string((*file).file_type),
            );
            return false_0 != 0;
        }
        if !CompileKeymap(file, keymap) {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Failed to compile keymap\n\0".as_ptr() as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn text_v1_keymap_new_from_rmlvo(
    mut keymap: *mut xkb_keymap,
    mut rmlvo: *const xkb_rmlvo_builder,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut kccgst: xkb_component_names = xkb_component_names {
            keycodes: ::core::ptr::null_mut::<i8>(),
            compatibility: ::core::ptr::null_mut::<i8>(),
            geometry: ::core::ptr::null_mut::<i8>(),
            symbols: ::core::ptr::null_mut::<i8>(),
            types: ::core::ptr::null_mut::<i8>(),
        };
        let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        if (*(*keymap).ctx).log_level as u32 >= XKB_LOG_LEVEL_DEBUG as ::core::ffi::c_int as u32 {
            let mut names: xkb_rule_names = xkb_rule_names {
                rules: ::core::ptr::null::<i8>(),
                model: ::core::ptr::null::<i8>(),
                layout: ::core::ptr::null::<i8>(),
                variant: ::core::ptr::null::<i8>(),
                options: ::core::ptr::null::<i8>(),
            };
            let buf_size: usize =
                (::core::mem::size_of::<[i8; 2048]>() as usize).wrapping_sub(1 as usize);
            let mut buf: *mut i8 = xkb_context_get_buffer((*rmlvo).ctx, buf_size);
            if buf.is_null() as ::core::ffi::c_int as i64 != 0 {
                return false_0 != 0;
            }
            ok = xkb_rmlvo_builder_to_rules_names(rmlvo, &raw mut names, buf, buf_size);
            if !ok as ::core::ffi::c_int as i64 != 0 {
                return false_0 != 0;
            }
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Compiling from RMLVO builder: rules '%s', model '%s', layout '%s', variant '%s', options '%s'\n\0"
                    .as_ptr() as *const i8,
                names.rules,
                names.model,
                names.layout,
                names.variant,
                names.options,
            );
        }
        ok = xkb_components_from_rmlvo_builder(
            rmlvo,
            &raw mut kccgst,
            &raw mut (*keymap).num_groups,
        );
        if !ok {
            let mut names_0: xkb_rule_names = xkb_rule_names {
                rules: ::core::ptr::null::<i8>(),
                model: ::core::ptr::null::<i8>(),
                layout: ::core::ptr::null::<i8>(),
                variant: ::core::ptr::null::<i8>(),
                options: ::core::ptr::null::<i8>(),
            };
            let buf_size_0: usize = ::core::mem::size_of::<[i8; 2048]>() as usize;
            let mut buf_0: *mut i8 = xkb_context_get_buffer((*rmlvo).ctx, buf_size_0);
            if buf_0.is_null() as ::core::ffi::c_int as i64 != 0 {
                return false_0 != 0;
            }
            ok = xkb_rmlvo_builder_to_rules_names(rmlvo, &raw mut names_0, buf_0, buf_size_0);
            if !ok as ::core::ffi::c_int as i64 != 0 {
                return false_0 != 0;
            }
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Couldn't look up rules '%s', model '%s', layout '%s', variant '%s', options '%s'\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
                names_0.rules,
                names_0.model,
                names_0.layout,
                names_0.variant,
                names_0.options,
            );
            return false_0 != 0;
        }
        let max_groups: xkb_layout_index_t =
            format_max_groups((*keymap).format) as xkb_layout_index_t;
        if (*keymap).num_groups > max_groups {
            (*keymap).num_groups = max_groups;
        }
        xkb_log(
            (*keymap).ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"Compiling from KcCGST: keycodes '%s', types '%s', compat '%s', symbols '%s'\n\0"
                .as_ptr() as *const i8,
            kccgst.keycodes,
            kccgst.types,
            kccgst.compatibility,
            kccgst.symbols,
        );
        file = XkbFileFromComponents((*keymap).ctx, &raw mut kccgst);
        free(kccgst.keycodes as *mut ::core::ffi::c_void);
        free(kccgst.types as *mut ::core::ffi::c_void);
        free(kccgst.compatibility as *mut ::core::ffi::c_void);
        free(kccgst.symbols as *mut ::core::ffi::c_void);
        free(kccgst.geometry as *mut ::core::ffi::c_void);
        if file.is_null() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Failed to generate parsed XKB file from components\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        ok = compile_keymap_file(keymap, file);
        FreeXkbFile(file);
        return ok;
    }
}
unsafe extern "C" fn text_v1_keymap_new_from_names(
    mut keymap: *mut xkb_keymap,
    mut rmlvo: *const xkb_rule_names,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut kccgst: xkb_component_names = xkb_component_names {
            keycodes: ::core::ptr::null_mut::<i8>(),
            compatibility: ::core::ptr::null_mut::<i8>(),
            geometry: ::core::ptr::null_mut::<i8>(),
            symbols: ::core::ptr::null_mut::<i8>(),
            types: ::core::ptr::null_mut::<i8>(),
        };
        let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        xkb_log(
            (*keymap).ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"Compiling from RMLVO: rules '%s', model '%s', layout '%s', variant '%s', options '%s'\n\0"
                .as_ptr() as *const i8,
            (*rmlvo).rules,
            (*rmlvo).model,
            (*rmlvo).layout,
            (*rmlvo).variant,
            (*rmlvo).options,
        );
        ok = xkb_components_from_rules_names(
            (*keymap).ctx,
            rmlvo,
            &raw mut kccgst,
            &raw mut (*keymap).num_groups,
        );
        if !ok {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Couldn't look up rules '%s', model '%s', layout '%s', variant '%s', options '%s'\n\0"
                    .as_ptr() as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
                (*rmlvo).rules,
                (*rmlvo).model,
                (*rmlvo).layout,
                (*rmlvo).variant,
                (*rmlvo).options,
            );
            return false_0 != 0;
        }
        let max_groups: xkb_layout_index_t =
            format_max_groups((*keymap).format) as xkb_layout_index_t;
        if (*keymap).num_groups > max_groups {
            (*keymap).num_groups = max_groups;
        }
        xkb_log(
            (*keymap).ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"Compiling from KcCGST: keycodes '%s', types '%s', compat '%s', symbols '%s'\n\0"
                .as_ptr() as *const i8,
            kccgst.keycodes,
            kccgst.types,
            kccgst.compatibility,
            kccgst.symbols,
        );
        file = XkbFileFromComponents((*keymap).ctx, &raw mut kccgst);
        free(kccgst.keycodes as *mut ::core::ffi::c_void);
        free(kccgst.types as *mut ::core::ffi::c_void);
        free(kccgst.compatibility as *mut ::core::ffi::c_void);
        free(kccgst.symbols as *mut ::core::ffi::c_void);
        free(kccgst.geometry as *mut ::core::ffi::c_void);
        if file.is_null() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Failed to generate parsed XKB file from components\n\0".as_ptr()
                    as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        ok = compile_keymap_file(keymap, file);
        FreeXkbFile(file);
        return ok;
    }
}
unsafe extern "C" fn text_v1_keymap_new_from_string(
    mut keymap: *mut xkb_keymap,
    mut string: *const i8,
    mut len: usize,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut xkb_file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        xkb_file = XkbParseString(
            (*keymap).ctx,
            string,
            len,
            b"(input string)\0".as_ptr() as *const i8,
            ::core::ptr::null::<i8>(),
        );
        if xkb_file.is_null() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Failed to parse input xkb string\n\0".as_ptr() as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        ok = compile_keymap_file(keymap, xkb_file);
        FreeXkbFile(xkb_file);
        return ok;
    }
}
unsafe extern "C" fn text_v1_keymap_new_from_file(
    mut keymap: *mut xkb_keymap,
    mut file: *mut FILE,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut xkb_file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
        xkb_file = XkbParseFile(
            (*keymap).ctx,
            file,
            b"(unknown file)\0".as_ptr() as *const i8,
            ::core::ptr::null::<i8>(),
        );
        if xkb_file.is_null() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Failed to parse input xkb file\n\0".as_ptr() as *const i8,
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        ok = compile_keymap_file(keymap, xkb_file);
        FreeXkbFile(xkb_file);
        return ok;
    }
}
#[no_mangle]
pub static mut text_v1_keymap_format_ops: xkb_keymap_format_ops = unsafe {
    xkb_keymap_format_ops {
        keymap_new_from_rmlvo: Some(
            text_v1_keymap_new_from_rmlvo
                as unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rmlvo_builder) -> bool,
        ),
        keymap_new_from_names: Some(
            text_v1_keymap_new_from_names
                as unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rule_names) -> bool,
        ),
        keymap_new_from_string: Some(
            text_v1_keymap_new_from_string
                as unsafe extern "C" fn(*mut xkb_keymap, *const i8, usize) -> bool,
        ),
        keymap_new_from_file: Some(
            text_v1_keymap_new_from_file
                as unsafe extern "C" fn(*mut xkb_keymap, *mut FILE) -> bool,
        ),
        keymap_get_as_string: Some(
            text_v1_keymap_get_as_string
                as unsafe extern "C" fn(
                    *mut xkb_keymap,
                    xkb_keymap_format,
                    xkb_keymap_serialize_flags,
                ) -> *mut i8,
        ),
    }
};
