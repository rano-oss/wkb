use crate::xkb_logf;
pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
    pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}
pub mod types_h {
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
}
pub mod stdint_intn_h {
    pub type i8 = __int8_t;
    pub type i16 = __int16_t;
    pub type i32 = __int32_t;
    pub type i64 = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}

pub mod context_h {
    pub use crate::xkb::context_priv::xkb_atom_text;
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
    pub const XKB_ATOM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod darray_h {
    pub use crate::xkb::shared_types::darray_size_t;
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
    pub type xkb_led_mask_t = u32;
    pub const XKB_KEYCODE_INVALID: u32 = 0xffffffff as u32;
    pub const XKB_KEYCODE_MAX: u32 = (0xffffffff as u32).wrapping_sub(1 as u32);
    
    pub use crate::xkb::context::xkb_context_get_log_verbosity;
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_overlay_index_t = uint8_t;
    pub const XKB_MAX_LEDS: xkb_led_index_t = (::core::mem::size_of::<xkb_led_mask_t>() as usize)
        .wrapping_mul(CHAR_BIT as usize)
        as xkb_led_index_t;
    pub const XKB_KEYCODE_MAX_CONTIGUOUS: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
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
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    pub type C2Rust_Unnamed_13 = DarrayKeysym;
    pub use crate::xkb::xkbcomp::ast_build::stmt_type_to_string;
}
pub mod text_h {
    
    
    pub use crate::xkb::text::{KeyNameText, LookupEntry};
}
pub mod xkbcomp_priv_h {
    pub use crate::xkb::shared_ast_types::{
        false_0, pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
        xkb_message_code, xkb_parser_error, xkb_parser_strict_flags, ReportBadType, ReportNotArray,
        XkbcompFeatures, XkbcompLookup, PARSER_FATAL_ERROR, PARSER_NO_FIELD_TYPE_MISMATCH,
        PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS,
        PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
        PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
        PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
        PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
        PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS,
        PARSER_V1_LAX_FLAGS, PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
    };
    pub type C2Rust_Unnamed_14 = XkbcompLookup;
    pub type C2Rust_Unnamed_15 = XkbcompFeatures;
    pub use crate::xkb::xkbcomp::ast_build::FreeXkbFile;
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
    pub unsafe fn istreq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe fn strdup_safe(mut s: *const i8) -> *mut i8 {
        unsafe { cstr_dup(s) }
    }

    use crate::xkb::utils::cstr_dup;
    pub use crate::xkb::utils::istrcmp;
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
pub mod expr_h {
    
    
    
    
    pub use crate::xkb::xkbcomp::expr::{ExprResolveInteger, ExprResolveLhs, ExprResolveString};
}
pub mod util_mem_h {
    #[inline]
    pub unsafe fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        unsafe {
            let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
            let mut swapped: *mut ::core::ffi::c_void = *original;
            *original = NULL;
            return swapped;
        }
    }
    use super::__stddef_null_h::NULL;
}
pub mod include_h {
    pub use crate::xkb::xkbcomp::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const UINT32_MAX: u32 = 4294967295 as u32;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, KeyAliasDef, KeycodeDef, LedNameDef,
    ParseCommon, UnknownStatement, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES,
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
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{xkb_atom_text, xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_size_t};
use self::expr_h::{ExprResolveInteger, ExprResolveLhs, ExprResolveString};
use self::include_h::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
pub use self::internal::{__va_list_tag, __CHAR_BIT__};
pub use self::keymap_h::{
    mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
    ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS,
    ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK,
    ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION,
    ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH,
    ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH,
    ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE,
    ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE,
    ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT,
    XKB_KEYCODE_MAX_CONTIGUOUS, XKB_MAX_LEDS,
};
pub use self::limits_h::CHAR_BIT;
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
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::UINT32_MAX;
pub use self::stdint_intn_h::{i16, i32, i64, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::{calloc, free, realloc};
use crate::xkb::utils::{darray_growalloc, darray_append, darray_resize_zero, darray_free};
pub use self::text_h::{KeyNameText, LookupEntry};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{istrcmp, istreq, strdup_safe};
pub use self::xkbcommon_h::{
    xkb_context_get_log_verbosity, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_led_index_t, xkb_led_mask_t, xkb_level_index_t, xkb_log_level, xkb_mod_index_t,
    xkb_mod_mask_t, xkb_rule_names, xkb_state_component, XKB_KEYCODE_INVALID, XKB_KEYCODE_MAX,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_strict_flags, C2Rust_Unnamed_14, C2Rust_Unnamed_15, FreeXkbFile, ReportBadType,
    ReportNotArray, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
    PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
    PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_V1_LAX_FLAGS, PARSER_V1_STRICT_FLAGS,
    PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use crate::xkb::keymap_priv::XkbEscapeMapName;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyNamesInfo {
    pub name: *mut i8,
    pub errorCount: ::core::ffi::c_int,
    pub include_depth: u32,
    pub keycodes: KeycodeStore,
    pub led_names: [LedNameInfo; 32],
    pub num_led_names: xkb_led_index_t,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedNameInfo {
    pub merge: merge_mode,
    pub name: xkb_atom_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeycodeStore {
    pub min: xkb_keycode_t,
    pub low: C2Rust_Unnamed_18,
    pub high: C2Rust_Unnamed_17,
    pub names: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut KeycodeMatch,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut HighKeycodeEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighKeycodeEntry {
    pub keycode: xkb_keycode_t,
    pub name: xkb_atom_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_atom_t,
}
#[inline]
unsafe fn keycode_store_init(mut store: *mut KeycodeStore) {
    unsafe {
        (*store).low.item = ::core::ptr::null_mut::<xkb_atom_t>();
        (*store).low.size = 0 as darray_size_t;
        (*store).low.alloc = 0 as darray_size_t;
        (*store).high.item = ::core::ptr::null_mut::<HighKeycodeEntry>();
        (*store).high.size = 0 as darray_size_t;
        (*store).high.alloc = 0 as darray_size_t;
        (*store).names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*store).names.size = 0 as darray_size_t;
        (*store).names.alloc = 0 as darray_size_t;
        (*store).min = XKB_KEYCODE_INVALID as xkb_keycode_t;
    }
}
#[inline]
unsafe fn keycode_store_free(mut store: *mut KeycodeStore) {
    unsafe {
        darray_free(&mut (*store).low.item, &mut (*store).low.size, &mut (*store).low.alloc);
        darray_free(&mut (*store).high.item, &mut (*store).high.size, &mut (*store).high.alloc);
        darray_free(&mut (*store).names.item, &mut (*store).names.size, &mut (*store).names.alloc);
    }
}
#[inline]
unsafe fn keycode_store_update_key(
    mut store: *mut KeycodeStore,
    mut match_0: KeycodeMatch,
    mut name: xkb_atom_t,
) {
    unsafe {
        if (!match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as i64
            != 0
        {
            return;
        } else if match_0.key.low() {
            *(*store).low.item.offset(match_0.key.index() as isize) = name;
        } else {
            (*(*store).high.item.offset(match_0.key.index() as isize)).name = name;
        }
        if name >= (*store).names.size {
            darray_resize_zero(&mut (*store).names.item, &mut (*store).names.size, &mut (*store).names.alloc, (name as darray_size_t).wrapping_add(1 as darray_size_t));
        }
        *(*store).names.item.offset(name as isize) = match_0;
    }
}
unsafe fn keycode_store_insert_key(
    mut store: *mut KeycodeStore,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
) -> bool {
    unsafe {
        if name >= (*store).names.size {
            darray_resize_zero(&mut (*store).names.item, &mut (*store).names.size, &mut (*store).names.alloc, (name as darray_size_t).wrapping_add(1 as darray_size_t));
        }
        if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            if kc >= (*store).low.size as xkb_keycode_t {
                darray_resize_zero(&mut (*store).low.item, &mut (*store).low.size, &mut (*store).low.alloc, (kc as darray_size_t).wrapping_add(1 as darray_size_t));
            }
            *(*store).low.item.offset(kc as isize) = name;
            if kc < (*store).min {
                (*store).min = kc;
            }
            *(*store).names.item.offset(name as isize) = KeycodeMatch {
                key: {
                    let mut init = C2Rust_Unnamed_7 {
                        found_low_is_alias_index: [0; 4],
                    };
                    init.set_found(true_0 != 0);
                    init.set_low(true_0 != 0);
                    init.set_is_alias(false_0 != 0);
                    init.set_index(kc as darray_size_t);
                    init
                },
            };
        } else {
            let idx: darray_size_t = (*store).high.size;
            if idx != 0
                && (*(*store)
                    .high
                    .item
                    .offset(idx.wrapping_sub(1 as darray_size_t) as isize))
                .keycode
                    > kc
            {
                let mut lower: darray_size_t = 0 as darray_size_t;
                let mut upper: darray_size_t = idx;
                while lower < upper {
                    let mid: darray_size_t = lower.wrapping_add(
                        upper
                            .wrapping_sub(1 as darray_size_t)
                            .wrapping_sub(lower)
                            .wrapping_div(2 as darray_size_t),
                    );
                    let entry: *const HighKeycodeEntry =
                        (*store).high.item.offset(mid as isize) as *mut HighKeycodeEntry;
                    if (*entry).keycode < kc {
                        lower = mid.wrapping_add(1 as darray_size_t);
                    } else if (*entry).keycode > kc {
                        upper = mid;
                    } else {
                    }
                }
                let mut entry_0: *mut HighKeycodeEntry =
                    ::core::ptr::null_mut::<HighKeycodeEntry>();
                if !(*store).high.item.is_null() {
                    entry_0 = (*store).high.item.offset(lower as isize) as *mut HighKeycodeEntry;
                    while entry_0
                        < (*store).high.item.offset((*store).high.size as isize)
                            as *mut HighKeycodeEntry
                    {
                        let ref mut c2rust_fresh4 =
                            (*(*store).names.item.offset((*entry_0).name as isize)).key;
                        (*c2rust_fresh4).set_index((*c2rust_fresh4).index() + 1 as darray_size_t);
                        entry_0 = entry_0.offset(1);
                    }
                }
                let mut __index: darray_size_t = lower;
                (*store).high.size = (*store).high.size.wrapping_add(1 as darray_size_t);
                darray_growalloc(&mut (*store).high.item, &mut (*store).high.alloc, (*store).high.size);
                std::ptr::copy(
                    (*store).high.item.offset(__index as isize),
                    (*store)
                        .high
                        .item
                        .offset(__index as isize)
                        .offset(1 as ::core::ffi::c_int as isize),
                    (*store)
                        .high
                        .size
                        .wrapping_sub(__index)
                        .wrapping_sub(1 as darray_size_t) as usize,
                );
                *(*store).high.item.offset(__index as isize) = HighKeycodeEntry {
                    keycode: kc,
                    name: name,
                };
                *(*store).names.item.offset(name as isize) = KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true_0 != 0);
                        init.set_low(false_0 != 0);
                        init.set_is_alias(false_0 != 0);
                        init.set_index(lower);
                        init
                    },
                };
            } else {
                darray_append(&mut (*store).high.item, &mut (*store).high.size, &mut (*store).high.alloc, HighKeycodeEntry { keycode: kc, name: name, });
                *(*store).names.item.offset(name as isize) = KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true_0 != 0);
                        init.set_low(false_0 != 0);
                        init.set_is_alias(false_0 != 0);
                        init.set_index(idx);
                        init
                    },
                };
            }
            if (*store).low.size == 0 as darray_size_t {
                (*store).min =
                    (*(*store).high.item.offset(0 as ::core::ffi::c_int as isize)).keycode;
            }
        }
        return true_0 != 0;
    }
}
#[inline]
unsafe fn keycode_store_insert_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        if alias >= (*store).names.size {
            darray_resize_zero(&mut (*store).names.item, &mut (*store).names.size, &mut (*store).names.alloc, (alias as darray_size_t).wrapping_add(1 as darray_size_t));
        }
        *(*store).names.item.offset(alias as isize) = KeycodeMatch {
            alias: {
                let mut init = C2Rust_Unnamed_6 {
                    found_c2rust_unnamed_is_alias_real: [0; 4],
                };
                init.set_found(true_0 != 0);
                init.set_c2rust_unnamed(true_0 != 0);
                init.set_is_alias(real != 0);
                init.set_real(real);
                init
            },
        };
        return true_0 != 0;
    }
}
#[inline]
unsafe fn keycode_store_update_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        let ref mut c2rust_fresh3 = (*(*store).names.item.offset(alias as isize)).alias;
        (*c2rust_fresh3).set_real(real as xkb_atom_t);
        return true_0 != 0;
    }
}
#[inline]
unsafe fn keycode_store_delete_name(mut store: *const KeycodeStore, mut name: xkb_atom_t) {
    unsafe {
        if name < (*store).names.size {
            let ref mut c2rust_fresh5 = (*(*store).names.item.offset(name as isize)).c2rust_unnamed;
            (*c2rust_fresh5).set_found((false_0 != 0) as bool);
        }
    }
}
unsafe fn keycode_store_delete_key(mut store: *mut KeycodeStore, match_0: KeycodeMatch) {
    unsafe {
        if (!match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as i64
            != 0
        {
            return;
        } else if match_0.key.low() {
            let ref mut c2rust_fresh1 = (*(*store)
                .names
                .item
                .offset(*(*store).low.item.offset(match_0.key.index() as isize) as isize))
            .c2rust_unnamed;
            (*c2rust_fresh1).set_found((false_0 != 0) as bool);
            if match_0.key.index().wrapping_add(1 as u32) == (*store).low.size {
                if (*store).min == match_0.key.index() as xkb_keycode_t {
                    (*store).low.size = 0 as darray_size_t;
                } else {
                    let mut idx: darray_size_t = match_0.key.index();
                    while idx > 0 as darray_size_t {
                        if *(*store)
                            .low
                            .item
                            .offset(idx.wrapping_sub(1 as darray_size_t) as isize)
                            != XKB_ATOM_NONE as xkb_atom_t
                        {
                            (*store).low.size = idx;
                            break;
                        } else {
                            idx = idx.wrapping_sub(1);
                        }
                    }
                }
            } else {
                *(*store).low.item.offset(match_0.key.index() as isize) =
                    XKB_ATOM_NONE as xkb_atom_t;
            }
        } else {
            let ref mut c2rust_fresh2 = (*(*store)
                .names
                .item
                .offset((*(*store).high.item.offset(match_0.key.index() as isize)).name as isize))
            .c2rust_unnamed;
            (*c2rust_fresh2).set_found((false_0 != 0) as bool);
            let mut __index: darray_size_t = match_0.key.index();
            if __index < (*store).high.size.wrapping_sub(1 as darray_size_t) {
                std::ptr::copy(
                    (*store)
                        .high
                        .item
                        .offset(__index.wrapping_add(1 as darray_size_t) as isize),
                    (*store).high.item.offset(__index as isize),
                    (*store)
                        .high
                        .size
                        .wrapping_sub(1 as darray_size_t)
                        .wrapping_sub(__index) as usize,
                );
            }
            (*store).high.size = (*store).high.size.wrapping_sub(1);
            let mut entry: *mut KeycodeMatch = ::core::ptr::null_mut::<KeycodeMatch>();
            if !(*store).names.item.is_null() {
                entry = (*store).names.item.offset(0 as ::core::ffi::c_int as isize)
                    as *mut KeycodeMatch;
                while entry
                    < (*store).names.item.offset((*store).names.size as isize) as *mut KeycodeMatch
                {
                    if (*entry).c2rust_unnamed.found() as ::core::ffi::c_int != 0
                        && !(*entry).c2rust_unnamed.is_alias()
                        && !(*entry).key.low()
                        && (*entry).key.index() as ::core::ffi::c_int
                            > match_0.key.index() as ::core::ffi::c_int
                    {
                        (*entry)
                            .key
                            .set_index((*entry).key.index() - 1 as darray_size_t);
                    }
                    entry = entry.offset(1);
                }
            }
        }
        if (*store).low.size == 0 as darray_size_t {
            (*store).min = if (*store).high.size == 0 as darray_size_t {
                XKB_KEYCODE_INVALID as xkb_keycode_t
            } else {
                (*(*store).high.item.offset(0 as ::core::ffi::c_int as isize)).keycode
            };
        } else {
            let mut kc: xkb_keycode_t = (*store).min;
            while kc < (*store).low.size as xkb_keycode_t {
                if *(*store).low.item.offset(kc as isize) != XKB_ATOM_NONE as xkb_atom_t {
                    (*store).min = kc;
                    break;
                } else {
                    kc = kc.wrapping_add(1);
                }
            }
        };
    }
}
#[inline]
unsafe fn keycode_store_get_keycode(
    mut store: *const KeycodeStore,
    mut match_0: KeycodeMatch,
) -> xkb_keycode_t {
    unsafe {
        if !match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
        {
            return XKB_KEYCODE_INVALID as xkb_keycode_t;
        } else if match_0.key.low() {
            return match_0.key.index() as xkb_keycode_t;
        } else {
            return (*(*store).high.item.offset(match_0.key.index() as isize)).keycode;
        };
    }
}
#[inline]
unsafe fn keycode_store_get_key_name(
    mut store: *const KeycodeStore,
    mut match_0: KeycodeMatch,
) -> xkb_atom_t {
    unsafe {
        if !match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
        {
            return XKB_ATOM_NONE as xkb_atom_t;
        } else if match_0.key.low() {
            return *(*store).low.item.offset(match_0.key.index() as isize);
        } else {
            return (*(*store).high.item.offset(match_0.key.index() as isize)).name;
        };
    }
}
unsafe fn keycode_store_lookup_keycode(
    mut store: *const KeycodeStore,
    mut kc: xkb_keycode_t,
) -> KeycodeMatch {
    unsafe {
        if kc < (*store).low.size as xkb_keycode_t {
            return KeycodeMatch {
                key: {
                    let mut init = C2Rust_Unnamed_7 {
                        found_low_is_alias_index: [0; 4],
                    };
                    init.set_found(true_0 != 0);
                    init.set_low(true_0 != 0);
                    init.set_is_alias(false_0 != 0);
                    init.set_index(kc as darray_size_t);
                    init
                },
            };
        } else if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            return KeycodeMatch {
                c2rust_unnamed: {
                    let mut init = C2Rust_Unnamed_8 {
                        found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                    };
                    init.set_found(false_0 != 0);
                    init.set_c2rust_unnamed(false);
                    init
                },
            };
        }
        let mut lower: darray_size_t = 0 as darray_size_t;
        let mut upper: darray_size_t = (*store).high.size;
        while lower < upper {
            let mid: darray_size_t = lower.wrapping_add(
                upper
                    .wrapping_sub(1 as darray_size_t)
                    .wrapping_sub(lower)
                    .wrapping_div(2 as darray_size_t),
            );
            let entry: *mut HighKeycodeEntry =
                (*store).high.item.offset(mid as isize) as *mut HighKeycodeEntry;
            if (*entry).keycode < kc {
                lower = mid.wrapping_add(1 as darray_size_t);
            } else if (*entry).keycode > kc {
                upper = mid;
            } else {
                return KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true_0 != 0);
                        init.set_low(false_0 != 0);
                        init.set_is_alias(false_0 != 0);
                        init.set_index(mid);
                        init
                    },
                };
            }
        }
        return KeycodeMatch {
            c2rust_unnamed: {
                let mut init = C2Rust_Unnamed_8 {
                    found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                };
                init.set_found(false_0 != 0);
                init.set_c2rust_unnamed(false);
                init
            },
        };
    }
}
unsafe fn keycode_store_lookup_name(
    mut store: *const KeycodeStore,
    mut name: xkb_atom_t,
) -> KeycodeMatch {
    unsafe {
        if name >= (*store).names.size {
            return KeycodeMatch {
                c2rust_unnamed: {
                    let mut init = C2Rust_Unnamed_8 {
                        found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                    };
                    init.set_found(false_0 != 0);
                    init.set_c2rust_unnamed(false_0 != 0);
                    init
                },
            };
        } else {
            return *(*store).names.item.offset(name as isize);
        };
    }
}
unsafe fn FindLedByName(
    mut info: *mut KeyNamesInfo,
    mut name: xkb_atom_t,
    mut idx_out: *mut xkb_led_index_t,
) -> *mut LedNameInfo {
    unsafe {
        let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
        while idx < (*info).num_led_names {
            let mut ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
                .offset(idx as isize)
                as *mut LedNameInfo;
            if (*ledi).name == name {
                *idx_out = idx;
                return ledi;
            }
            idx = idx.wrapping_add(1);
        }
        return ::core::ptr::null_mut::<LedNameInfo>();
    }
}
unsafe fn AddLedName(
    mut info: *mut KeyNamesInfo,
    mut same_file: bool,
    mut new: *mut LedNameInfo,
    mut new_idx: xkb_led_index_t,
    mut report: bool,
) -> bool {
    unsafe {
        let mut old_idx: xkb_led_index_t = 0;
        let mut old: *mut LedNameInfo = ::core::ptr::null_mut::<LedNameInfo>();
        let replace: bool = (*new).merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32;
        old = FindLedByName(info, (*new).name, &raw mut old_idx);
        if !old.is_null() {
            if old_idx == new_idx {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Multiple indicators named \"{}\"; Identical definitions ignored\n",
                        crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    );
                }
                return true_0 != 0;
            }
            if report {
                let mut use_0: xkb_led_index_t = if replace as ::core::ffi::c_int != 0 {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                };
                let mut ignore: xkb_led_index_t = if replace as ::core::ffi::c_int != 0 {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Multiple indicators named {}; Using {}, ignoring {}\n",
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    use_0,
                    ignore,
                );
            }
            if replace {
                (*old).name = XKB_ATOM_NONE as xkb_atom_t;
            } else {
                return true_0 != 0;
            }
        }
        if new_idx >= (*info).num_led_names {
            (*info).num_led_names = new_idx.wrapping_add(1 as xkb_led_index_t);
        }
        old = (&raw mut (*info).led_names as *mut LedNameInfo).offset(new_idx as isize)
            as *mut LedNameInfo;
        if (*old).name != XKB_ATOM_NONE as xkb_atom_t {
            if report {
                let use_1: xkb_atom_t = if replace as ::core::ffi::c_int != 0 {
                    (*new).name
                } else {
                    (*old).name
                };
                let ignore_0: xkb_atom_t = if replace as ::core::ffi::c_int != 0 {
                    (*old).name
                } else {
                    (*new).name
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Multiple names for indicator {}; Using {}, ignoring {}\n",
                    new_idx.wrapping_add(1 as xkb_led_index_t),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, use_1)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, ignore_0)),
                );
            }
            if replace {
                *old = *new;
            }
            return true_0 != 0;
        }
        *old = *new;
        return true_0 != 0;
    }
}
unsafe fn ClearKeyNamesInfo(mut info: *mut KeyNamesInfo) {
    unsafe {
        free((*info).name as *mut ::core::ffi::c_void);
        keycode_store_free(&raw mut (*info).keycodes);
    }
}
unsafe fn InitKeyNamesInfo(
    mut info: *mut KeyNamesInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
) {
    unsafe {
        std::ptr::write_bytes::<KeyNamesInfo>(info as *mut KeyNamesInfo, 0u8, 1);
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        keycode_store_init(&raw mut (*info).keycodes);
    }
}
unsafe fn AddKeyName(
    mut info: *mut KeyNamesInfo,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
    mut merge: merge_mode,
    mut report: bool,
) -> bool {
    unsafe {
        let mut match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, name);
        if match_name.c2rust_unnamed.found() {
            let clobber: bool = merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32;
            if match_name.c2rust_unnamed.is_alias() {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Key name {} already assigned to an alias; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                        crate::xkb::utils::CStrDisplay(if clobber as ::core::ffi::c_int != 0 {
                            b"key\0".as_ptr() as *const i8
                        } else {
                            b"alias\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if clobber as ::core::ffi::c_int != 0 {
                            b"alias\0".as_ptr() as *const i8
                        } else {
                            b"key\0".as_ptr() as *const i8
                        }),
                    );
                }
                if clobber {
                    keycode_store_delete_name(&raw mut (*info).keycodes, name);
                    match_name.c2rust_unnamed.set_found((false_0 != 0) as bool);
                } else {
                    return true_0 != 0;
                }
            } else {
                let old_kc: xkb_keycode_t =
                    keycode_store_get_keycode(&raw mut (*info).keycodes, match_name)
                        as xkb_keycode_t;
                if old_kc != kc {
                    if report {
                        let use_0: xkb_keycode_t = if clobber as ::core::ffi::c_int != 0 {
                            kc
                        } else {
                            old_kc
                        };
                        let ignore: xkb_keycode_t = if clobber as ::core::ffi::c_int != 0 {
                            old_kc
                        } else {
                            kc
                        };
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "[XKB-{:03}] Key name {} assigned to multiple keys; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                            use_0,
                            ignore,
                        );
                    }
                    if clobber {
                        keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                    } else {
                        return true_0 != 0;
                    }
                }
            }
        }
        let match_kc: KeycodeMatch =
            keycode_store_lookup_keycode(&raw mut (*info).keycodes, kc) as KeycodeMatch;
        let old_name: xkb_atom_t =
            keycode_store_get_key_name(&raw mut (*info).keycodes, match_kc) as xkb_atom_t;
        if old_name != XKB_ATOM_NONE as xkb_atom_t {
            if old_name == name {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Multiple identical key name definitions; Later occurrences of \"{} = {}\" ignored\n",
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, old_name)),
                        kc,
                    );
                }
                return true_0 != 0;
            }
            let clobber_0: bool = merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32;
            if report {
                let kname: *const i8 = KeyNameText((*info).ctx, name) as *const i8;
                let old_kname: *const i8 = KeyNameText((*info).ctx, old_name) as *const i8;
                let use_1: *const i8 = if clobber_0 as ::core::ffi::c_int != 0 {
                    kname
                } else {
                    old_kname
                };
                let ignore_0: *const i8 = if clobber_0 as ::core::ffi::c_int != 0 {
                    old_kname
                } else {
                    kname
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Multiple names for keycode {}; Using {}, ignoring {}\n",
                    kc,
                    crate::xkb::utils::CStrDisplay(use_1),
                    crate::xkb::utils::CStrDisplay(ignore_0),
                );
            }
            if clobber_0 {
                keycode_store_delete_name(&raw mut (*info).keycodes, old_name);
                keycode_store_update_key(&raw mut (*info).keycodes, match_kc, name);
            }
        } else if !keycode_store_insert_key(&raw mut (*info).keycodes, kc, name) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Cannot add keycode\n",
                XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
unsafe fn MergeKeycodeStores(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (*into).keycodes.low.size == 0 as darray_size_t
            && (*into).keycodes.high.size == 0 as darray_size_t
            && (*into).keycodes.names.size == 0 as darray_size_t
        {
            (*into).keycodes = (*from).keycodes;
            (*from).keycodes.low.item = ::core::ptr::null_mut::<xkb_atom_t>();
            (*from).keycodes.low.size = 0 as darray_size_t;
            (*from).keycodes.low.alloc = 0 as darray_size_t;
            (*from).keycodes.high.item = ::core::ptr::null_mut::<HighKeycodeEntry>();
            (*from).keycodes.high.size = 0 as darray_size_t;
            (*from).keycodes.high.alloc = 0 as darray_size_t;
            (*from).keycodes.names.item = ::core::ptr::null_mut::<KeycodeMatch>();
            (*from).keycodes.names.size = 0 as darray_size_t;
            (*from).keycodes.names.alloc = 0 as darray_size_t;
        } else {
            let mut kc: xkb_keycode_t = (*from).keycodes.min;
            while kc < (*from).keycodes.low.size as xkb_keycode_t {
                let name: xkb_atom_t = *(*from).keycodes.low.item.offset(kc as isize);
                if !(name == XKB_ATOM_NONE as xkb_atom_t) {
                    if !AddKeyName(into, kc, name, merge, report) {
                        (*into).errorCount += 1;
                    }
                }
                kc = kc.wrapping_add(1);
            }
            let mut new: *const HighKeycodeEntry = ::core::ptr::null::<HighKeycodeEntry>();
            if !(*from).keycodes.high.item.is_null() {
                new = (*from)
                    .keycodes
                    .high
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut HighKeycodeEntry;
                while new
                    < (*from)
                        .keycodes
                        .high
                        .item
                        .offset((*from).keycodes.high.size as isize)
                        as *mut HighKeycodeEntry as *const HighKeycodeEntry
                {
                    if !AddKeyName(into, (*new).keycode, (*new).name, merge, report) {
                        (*into).errorCount += 1;
                    }
                    new = new.offset(1);
                }
            }
            let mut match_0: *mut KeycodeMatch = ::core::ptr::null_mut::<KeycodeMatch>();
            let mut alias: xkb_atom_t = 0;
            if !(*from).keycodes.names.item.is_null() {
                alias = 0 as xkb_atom_t;
                match_0 = (*from)
                    .keycodes
                    .names
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut KeycodeMatch;
                while alias < (*from).keycodes.names.size {
                    if !(!(*match_0).c2rust_unnamed.found()
                        || !(*match_0).c2rust_unnamed.is_alias())
                    {
                        let def: KeyAliasDef = KeyAliasDef {
                            common: _ParseCommon {
                                next: ::core::ptr::null_mut::<_ParseCommon>(),
                                type_0: STMT_UNKNOWN,
                            },
                            merge: merge,
                            alias: alias,
                            real: (*match_0).alias.real(),
                        };
                        if !HandleAliasDef(into, &raw const def, report) {
                            (*into).errorCount += 1;
                        }
                    }
                    alias = alias.wrapping_add(1);
                    match_0 = match_0.offset(1);
                }
            }
        };
    }
}
unsafe fn MergeIncludedKeycodes(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (*from).errorCount > 0 as ::core::ffi::c_int {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        if (*into).name.is_null() {
            (*into).name =
                _steal(&raw mut (*from).name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        }
        MergeKeycodeStores(into, from, merge, report);
        if (*into).num_led_names == 0 as xkb_led_index_t {
            std::ptr::copy_nonoverlapping::<LedNameInfo>(
                &raw mut (*from).led_names as *mut LedNameInfo,
                &raw mut (*into).led_names as *mut LedNameInfo,
                (*from).num_led_names as usize,
            );
            (*into).num_led_names = (*from).num_led_names;
            (*from).num_led_names = 0 as xkb_led_index_t;
        } else {
            let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
            while idx < (*from).num_led_names {
                let mut ledi: *mut LedNameInfo = (&raw mut (*from).led_names as *mut LedNameInfo)
                    .offset(idx as isize)
                    as *mut LedNameInfo;
                if !((*ledi).name == XKB_ATOM_NONE as xkb_atom_t) {
                    (*ledi).merge = merge;
                    if !AddLedName(into, false_0 != 0, ledi, idx, report) {
                        (*into).errorCount += 1;
                    }
                }
                idx = idx.wrapping_add(1);
            }
        };
    }
}
unsafe fn HandleIncludeKeycodes(
    mut info: *mut KeyNamesInfo,
    mut include: *mut IncludeStmt,
    mut report: bool,
) -> bool {
    unsafe {
        let mut included: KeyNamesInfo = KeyNamesInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: 0,
                low: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_atom_t>(),
                },
                high: C2Rust_Unnamed_17 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<HighKeycodeEntry>(),
                },
                names: C2Rust_Unnamed_16 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<KeycodeMatch>(),
                },
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as ::core::ffi::c_int;
            return false_0 != 0;
        }
        InitKeyNamesInfo(&raw mut included, (*info).keymap_info, 0 as u32);
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyNamesInfo = KeyNamesInfo {
                name: ::core::ptr::null_mut::<i8>(),
                errorCount: 0,
                include_depth: 0,
                keycodes: KeycodeStore {
                    min: 0,
                    low: C2Rust_Unnamed_18 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<xkb_atom_t>(),
                    },
                    high: C2Rust_Unnamed_17 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<HighKeycodeEntry>(),
                    },
                    names: C2Rust_Unnamed_16 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<KeycodeMatch>(),
                    },
                },
                led_names: [LedNameInfo {
                    merge: MERGE_DEFAULT,
                    name: 0,
                }; 32],
                num_led_names: 0,
                ctx: ::core::ptr::null_mut::<xkb_context>(),
                keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            };
            let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_KEYCODES,
                &raw mut path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
            );
            if file.is_null() {
                (*info).errorCount += 10 as ::core::ffi::c_int;
                ClearKeyNamesInfo(&raw mut included);
                return false_0 != 0;
            }
            InitKeyNamesInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
            );
            HandleKeycodesFile(&raw mut next_incl, file);
            MergeIncludedKeycodes(&raw mut included, &raw mut next_incl, (*stmt).merge, report);
            ClearKeyNamesInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedKeycodes(info, &raw mut included, (*include).merge, report);
        ClearKeyNamesInfo(&raw mut included);
        return (*info).errorCount == 0 as ::core::ffi::c_int;
    }
}
unsafe fn HandleKeycodeDef(
    mut info: *mut KeyNamesInfo,
    mut stmt: *mut KeycodeDef,
    mut report: bool,
) -> bool {
    unsafe {
        if (*stmt).value < 0 as i64 || (*stmt).value > XKB_KEYCODE_MAX as i64 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "Illegal keycode {}: must be between 0..{}; Key ignored\n",
                (*stmt).value,
                (0xffffffff as u32).wrapping_sub(1 as u32),
            );
            return false_0 != 0;
        }
        return AddKeyName(
            info,
            (*stmt).value as xkb_keycode_t,
            (*stmt).name,
            (*stmt).merge,
            report,
        );
    }
}
unsafe fn HandleAliasDef(
    mut info: *mut KeyNamesInfo,
    mut def: *const KeyAliasDef,
    mut report: bool,
) -> bool {
    unsafe {
        let match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, (*def).alias) as KeycodeMatch;
        if match_name.c2rust_unnamed.found() {
            let clobber: bool = (*def).merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32;
            if match_name.c2rust_unnamed.is_alias() {
                if (*def).real == match_name.alias.real() {
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "[XKB-{:03}] Alias of {} for {} declared more than once; First definition ignored\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).real)),
                        );
                    }
                } else {
                    let use_0: xkb_atom_t = if clobber as ::core::ffi::c_int != 0 {
                        (*def).real
                    } else {
                        match_name.alias.real()
                    };
                    let ignore: xkb_atom_t = if clobber as ::core::ffi::c_int != 0 {
                        match_name.alias.real()
                    } else {
                        (*def).real
                    };
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "[XKB-{:03}] Multiple definitions for alias {}; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, use_0)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, ignore)),
                        );
                    }
                    keycode_store_update_alias(&raw mut (*info).keycodes, (*def).alias, use_0);
                }
                return true_0 != 0;
            } else {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Alias name {} already assigned to a real key; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                        crate::xkb::utils::CStrDisplay(if clobber as ::core::ffi::c_int != 0 {
                            b"alias\0".as_ptr() as *const i8
                        } else {
                            b"key\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if clobber as ::core::ffi::c_int != 0 {
                            b"key\0".as_ptr() as *const i8
                        } else {
                            b"alias\0".as_ptr() as *const i8
                        }),
                    );
                }
                if clobber {
                    keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                } else {
                    return true_0 != 0;
                }
            }
        }
        return keycode_store_insert_alias(&raw mut (*info).keycodes, (*def).alias, (*def).real);
    }
}
unsafe fn HandleKeyNameVar(mut info: *mut KeyNamesInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const i8 = ::core::ptr::null::<i8>();
        let mut field: *const i8 = ::core::ptr::null::<i8>();
        let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut arrayNdx,
        ) {
            return false_0 != 0;
        }
        if !elem.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Cannot set global defaults for \"{}\" element; Assignment to \"{}.{}\" ignored\n",
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as ::core::ffi::c_int as u32
                == 0;
        }
        if !istreq(field, b"minimum\0".as_ptr() as *const i8)
            && !istreq(field, b"maximum\0".as_ptr() as *const i8)
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as ::core::ffi::c_int as u32
                == 0;
        }
        if !arrayNdx.is_null() {
            ReportNotArray(
                (*info).ctx,
                b"keycodes\0".as_ptr() as *const i8,
                field,
                b"defaults\0".as_ptr() as *const i8,
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as u32
                == 0;
        }
        let mut val: i64 = 0 as i64;
        if !ExprResolveInteger((*info).ctx, (*stmt).value, &raw mut val)
            || val < 0 as i64
            || val > UINT32_MAX as i64
        {
            ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"keycodes\0".as_ptr() as *const i8,
                field,
                b"defaults\0".as_ptr() as *const i8,
                b"integer 0..0xfffffffe\0".as_ptr() as *const i8,
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as u32
                == 0;
        }
        return true_0 != 0;
    }
}
unsafe fn HandleLedNameDef(
    mut info: *mut KeyNamesInfo,
    mut def: *mut LedNameDef,
    mut report: bool,
) -> bool {
    unsafe {
        if (*def).ndx < 1 as i64 || (*def).ndx > XKB_MAX_LEDS as i64 {
            (*info).errorCount += 1;
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "Illegal indicator index ({}) specified; must be between 1 .. {}; Ignored\n",
                (*def).ndx,
                (::core::mem::size_of::<xkb_led_mask_t>() as usize).wrapping_mul(8 as usize)
                    as xkb_led_index_t,
            );
            return false_0 != 0;
        }
        let mut name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, (*def).name, &raw mut name) {
            let mut buf: [i8; 20] = [0; 20];
            crate::xkb::utils::snprintf_args(
                &raw mut buf as *mut i8,
                ::core::mem::size_of::<[i8; 20]>() as usize,
                format_args!("{}", (*def).ndx),
            );
            (*info).errorCount += 1;
            return ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"indicator\0".as_ptr() as *const i8,
                b"name\0".as_ptr() as *const i8,
                &raw mut buf as *mut i8,
                b"string\0".as_ptr() as *const i8,
            );
        }
        let mut ledi: LedNameInfo = LedNameInfo {
            merge: (*def).merge,
            name: name,
        };
        return AddLedName(
            info,
            true_0 != 0,
            &raw mut ledi,
            ((*def).ndx as xkb_led_index_t).wrapping_sub(1 as xkb_led_index_t),
            report,
        );
    }
}
unsafe fn HandleKeycodesFile(mut info: *mut KeyNamesInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        let verbosity: ::core::ffi::c_int =
            xkb_context_get_log_verbosity((*info).ctx) as ::core::ffi::c_int;
        let report_same_file: bool = verbosity > 0 as ::core::ffi::c_int;
        let report_include: bool = verbosity > 7 as ::core::ffi::c_int;
        free((*info).name as *mut ::core::ffi::c_void);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
                1 => {
                    ok = HandleIncludeKeycodes(info, stmt as *mut IncludeStmt, report_include);
                }
                2 => {
                    ok = HandleKeycodeDef(info, stmt as *mut KeycodeDef, report_same_file);
                }
                3 => {
                    ok = HandleAliasDef(info, stmt as *mut KeyAliasDef, report_same_file);
                }
                26 => {
                    ok = HandleKeyNameVar(info, stmt as *mut VarDef);
                }
                34 => {
                    ok = HandleLedNameDef(info, stmt as *mut LedNameDef, report_same_file);
                }
                35 | 36 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Unsupported keycodes {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        crate::xkb::utils::CStrDisplay(
                            if (*stmt).type_0 as u32 == STMT_UNKNOWN_COMPOUND as i32 as u32 {
                                b"compound\0".as_ptr() as *const i8
                            } else {
                                b"declaration\0".as_ptr() as *const i8
                            }
                        ),
                        crate::xkb::utils::CStrDisplay((*(stmt as *mut UnknownStatement)).name),
                    );
                    ok = (*(*info).keymap_info).strict as u32
                        & PARSER_NO_UNKNOWN_STATEMENTS as ::core::ffi::c_int as u32
                        == 0;
                }
                _ => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Keycode files may define key and indicator names only; Ignoring {}\n",
                        crate::xkb::utils::CStrDisplay(stmt_type_to_string((*stmt).type_0)),
                    );
                    ok = false_0 != 0;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10 as ::core::ffi::c_int {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Abandoning keycodes file \"{}\"\n",
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn CopyKeyNamesToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
    unsafe {
        if (*info).keycodes.low.size == 0 as darray_size_t
            && (*info).keycodes.high.size == 0 as darray_size_t
        {
            (*keymap).min_key_code = 8 as xkb_keycode_t;
            (*keymap).max_key_code = 255 as xkb_keycode_t;
            (*keymap).num_keys_low = (*keymap).max_key_code.wrapping_add(1 as xkb_keycode_t);
            (*keymap).num_keys = (*keymap).num_keys_low;
        } else {
            (*keymap).min_key_code = (*info).keycodes.min;
            (*keymap).max_key_code =
                if (*info).keycodes.high.size == 0 as darray_size_t {
                    ((*info).keycodes.low.size as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t)
                } else {
                    (*(*info).keycodes.high.item.offset(
                        (*info).keycodes.high.size.wrapping_sub(1 as darray_size_t) as isize,
                    ))
                    .keycode
                };
            (*keymap).num_keys_low = (*info).keycodes.low.size as xkb_keycode_t;
            (*keymap).num_keys = (*keymap)
                .num_keys_low
                .wrapping_add((*info).keycodes.high.size as xkb_keycode_t);
        }
        let keys: *mut xkb_key = calloc(
            (*keymap).num_keys as usize,
            ::core::mem::size_of::<xkb_key>() as usize,
        ) as *mut xkb_key;
        if keys.is_null() {
            (*keymap).num_keys = 0 as xkb_keycode_t;
            (*keymap).max_key_code = XKB_KEYCODE_INVALID as xkb_keycode_t;
            (*keymap).min_key_code = (*keymap).max_key_code;
            return false_0 != 0;
        }
        let mut kc: xkb_keycode_t = (*keymap).min_key_code;
        while kc < (*keymap).num_keys_low {
            (*keys.offset(kc as isize)).keycode = kc;
            kc = kc.wrapping_add(1);
        }
        let mut kc_0: xkb_keycode_t = (*info).keycodes.min;
        while kc_0 < (*info).keycodes.low.size as xkb_keycode_t {
            (*keys.offset(kc_0 as isize)).name = *(*info).keycodes.low.item.offset(kc_0 as isize);
            kc_0 = kc_0.wrapping_add(1);
        }
        let mut idx: xkb_keycode_t = (*keymap).num_keys_low;
        let mut entry: *const HighKeycodeEntry = ::core::ptr::null::<HighKeycodeEntry>();
        if !(*info).keycodes.high.item.is_null() {
            entry = (*info)
                .keycodes
                .high
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut HighKeycodeEntry;
            while entry
                < (*info)
                    .keycodes
                    .high
                    .item
                    .offset((*info).keycodes.high.size as isize)
                    as *mut HighKeycodeEntry as *const HighKeycodeEntry
            {
                (*keys.offset(idx as isize)).keycode = (*entry).keycode;
                (*keys.offset(idx as isize)).name = (*entry).name;
                idx = idx.wrapping_add(1);
                entry = entry.offset(1);
            }
        }
        (*keymap).keys = keys;
        return true_0 != 0;
    }
}
unsafe fn CopyKeycodeNameLUT(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
    unsafe {
        let mut match_0: *mut KeycodeMatch = ::core::ptr::null_mut::<KeycodeMatch>();
        let mut name: xkb_atom_t = 0;
        if !(*info).keycodes.names.item.is_null() {
            name = 0 as xkb_atom_t;
            match_0 = (*info)
                .keycodes
                .names
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut KeycodeMatch;
            while name < (*info).keycodes.names.size {
                if (*match_0).c2rust_unnamed.found() {
                    if (*match_0).c2rust_unnamed.is_alias() {
                        let match_real: KeycodeMatch = keycode_store_lookup_name(
                            &raw mut (*info).keycodes,
                            (*match_0).alias.real(),
                        ) as KeycodeMatch;
                        if !match_real.c2rust_unnamed.found() {
                            xkb_logf!(
                                (*info).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                                "[XKB-{:03}] Attempt to alias {} to non-existent key {}; Ignored\n",
                                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                                crate::xkb::utils::CStrDisplay(KeyNameText(
                                    (*info).ctx,
                                    (*match_0).alias.real()
                                )),
                            );
                            (*match_0).c2rust_unnamed.set_found((false_0 != 0) as bool);
                        } else {
                        }
                    } else if !(*match_0).key.low() {
                        (*match_0).key.set_index(
                            (*match_0).key.index() + (*keymap).num_keys_low as u32 as darray_size_t,
                        );
                    }
                }
                name = name.wrapping_add(1);
                match_0 = match_0.offset(1);
            }
        }
        if (*info).keycodes.names.size > 0 as darray_size_t {
            (*info).keycodes.names.alloc = (*info).keycodes.names.size;
            (*info).keycodes.names.item = realloc(
                (*info).keycodes.names.item as *mut ::core::ffi::c_void,
                ((*info).keycodes.names.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as usize),
            ) as *mut KeycodeMatch;
        }
        (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names = (*info).keycodes.names.size;
        (*keymap).c2rust_unnamed.c2rust_unnamed.key_names = (*info).keycodes.names.item;
        if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
            *(::core::ptr::null_mut::<::core::ffi::c_void>() as *mut darray_size_t) =
                (*info).keycodes.names.size;
        }
        (*info).keycodes.names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*info).keycodes.names.size = 0 as darray_size_t;
        (*info).keycodes.names.alloc = 0 as darray_size_t;
        (*info).keycodes.names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*info).keycodes.names.size = 0 as darray_size_t;
        (*info).keycodes.names.alloc = 0 as darray_size_t;
        return true_0 != 0;
    }
}
unsafe fn CopyLedNamesToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
    unsafe {
        (*keymap).num_leds = (*info).num_led_names;
        let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
        while idx < (*info).num_led_names {
            let mut ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
                .offset(idx as isize)
                as *mut LedNameInfo;
            if !((*ledi).name == XKB_ATOM_NONE as xkb_atom_t) {
                (*keymap).leds[idx as usize].name = (*ledi).name;
            }
            idx = idx.wrapping_add(1);
        }
        return true_0 != 0;
    }
}
unsafe fn CopyKeyNamesInfoToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut KeyNamesInfo,
) -> bool {
    unsafe {
        if !CopyKeyNamesToKeymap(keymap, info)
            || !CopyKeycodeNameLUT(keymap, info)
            || !CopyLedNamesToKeymap(keymap, info)
        {
            return false_0 != 0;
        }
        if (*keymap).num_keys == 0 || (*keymap).min_key_code > 0 as xkb_keycode_t {
            (*keymap).redirect_key_auto = 0 as xkb_keycode_t;
        } else {
            let mut keycode: xkb_keycode_t =
                (XKB_KEYCODE_INVALID as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t);
            let mut k: xkb_keycode_t = (*keymap).num_keys;
            loop {
                let c2rust_fresh0 = k;
                k = k.wrapping_sub(1);
                if !(c2rust_fresh0 > (*keymap).num_keys_low) {
                    break;
                }
                if keycode > (*(*keymap).keys.offset(k as isize)).keycode {
                    break;
                }
                keycode = (*(*keymap).keys.offset(k as isize))
                    .keycode
                    .wrapping_sub(1 as xkb_keycode_t);
            }
            (*keymap).redirect_key_auto = keycode;
        }
        (*keymap).keycodes_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).keycodes_section_name);
        return true_0 != 0;
    }
}
pub unsafe fn CompileKeycodes(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: KeyNamesInfo = KeyNamesInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: 0,
                low: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_atom_t>(),
                },
                high: C2Rust_Unnamed_17 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<HighKeycodeEntry>(),
                },
                names: C2Rust_Unnamed_16 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<KeycodeMatch>(),
                },
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        InitKeyNamesInfo(&raw mut info, keymap_info, 0 as u32);
        if !file.is_null() {
            HandleKeycodesFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as ::core::ffi::c_int) {
            if CopyKeyNamesInfoToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearKeyNamesInfo(&raw mut info);
                return true_0 != 0;
            }
        }
        ClearKeyNamesInfo(&raw mut info);
        return false_0 != 0;
    }
}
