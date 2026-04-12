use crate::xkb_logf;
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
    pub use crate::xkb::context_priv::{xkb_atom_intern, xkb_atom_text};
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
    pub use crate::xkb::context::xkb_context_get_log_verbosity;
    pub use crate::xkb::shared_types::{
        xkb_context, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
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
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_overlay_index_t = uint8_t;
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    pub type C2Rust_Unnamed_13 = DarrayKeysym;
    pub use crate::xkb::xkbcomp::ast_build::stmt_type_to_string;
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
pub mod text_h {
    
    
    
    pub use crate::xkb::text::{LookupEntry, ModMaskText};
}
pub mod xkbcomp_priv_h {
    pub use crate::xkb::shared_ast_types::{
        false_0, pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
        xkb_message_code, xkb_parser_error, xkb_parser_strict_flags, ReportBadType,
        ReportShouldBeArray, XkbcompFeatures, XkbcompLookup, PARSER_FATAL_ERROR,
        PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
        PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
        PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
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
pub mod vmod_h {
    pub use crate::xkb::xkbcomp::vmod::{HandleVModDef, InitVMods, MergeModSets};
}
pub mod expr_h {
    
    
    
    
    
    pub use crate::xkb::xkbcomp::expr::{
        ExprResolveLevel, ExprResolveLhs, ExprResolveModMask, ExprResolveString,
    };
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
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, KeyTypeDef, ParseCommon, UnknownStatement,
    VModDef, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES,
    FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP,
    FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER,
    MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT,
    MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
    STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL,
    STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL,
    STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL,
    STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS,
    STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME,
    STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{
    xkb_atom_intern, xkb_atom_text, xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::{darray_size_t};
use self::expr_h::{ExprResolveLevel, ExprResolveLhs, ExprResolveModMask, ExprResolveString};
use self::include_h::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
pub use self::internal::__va_list_tag;
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
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_intn_h::{i16, i32, i64, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::{calloc, free};
pub use self::text_h::{LookupEntry, ModMaskText};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{istrcmp, istreq, strdup_safe};
use self::vmod_h::{HandleVModDef, InitVMods, MergeModSets};
use crate::xkb::utils::{darray_growalloc, darray_append, darray_resize_zero, darray_free};
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_context_get_log_verbosity, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_led_index_t, xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t,
    xkb_rule_names, xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_strict_flags, C2Rust_Unnamed_14, C2Rust_Unnamed_15, FreeXkbFile, ReportBadType,
    ReportShouldBeArray, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
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
pub struct KeyTypesInfo {
    pub name: *mut i8,
    pub errorCount: ::core::ffi::c_int,
    pub include_depth: u32,
    pub types: C2Rust_Unnamed_16,
    pub mods: xkb_mod_set,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut KeyTypeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyTypeInfo {
    pub defined: type_field,
    pub merge: merge_mode,
    pub name: xkb_atom_t,
    pub mods: xkb_mod_mask_t,
    pub num_levels: xkb_level_index_t,
    pub entries: C2Rust_Unnamed_18,
    pub level_names: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_atom_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_key_type_entry,
}
pub type type_field = u32;
pub const TYPE_FIELD_LEVEL_NAME: type_field = 8;
pub const TYPE_FIELD_PRESERVE: type_field = 4;
pub const TYPE_FIELD_MAP: type_field = 2;
pub const TYPE_FIELD_MASK: type_field = 1;
#[inline]
unsafe fn MapEntryTxt(
    mut info: *mut KeyTypesInfo,
    mut entry: *mut xkb_key_type_entry,
) -> *const i8 {
    unsafe {
        return ModMaskText(
            (*info).ctx,
            MOD_BOTH,
            &raw mut (*info).mods,
            (*entry).mods.mods,
        );
    }
}
#[inline]
unsafe fn TypeTxt(mut info: *mut KeyTypesInfo, mut type_0: *mut KeyTypeInfo) -> *const i8 {
    unsafe {
        return xkb_atom_text((*info).ctx, (*type_0).name);
    }
}
#[inline]
unsafe fn TypeMaskTxt(mut info: *mut KeyTypesInfo, mut type_0: *mut KeyTypeInfo) -> *const i8 {
    unsafe {
        return ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, (*type_0).mods);
    }
}
#[inline]
unsafe fn ReportTypeShouldBeArray(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut field: *const i8,
) -> bool {
    unsafe {
        return ReportShouldBeArray(
            (*info).ctx,
            b"key type\0".as_ptr() as *const i8,
            field,
            TypeTxt(info, type_0),
        );
    }
}
#[inline]
unsafe fn ReportTypeBadType(
    mut info: *mut KeyTypesInfo,
    mut code: xkb_message_code,
    mut type_0: *mut KeyTypeInfo,
    mut field: *const i8,
    mut wanted: *const i8,
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            code,
            b"key type\0".as_ptr() as *const i8,
            field,
            TypeTxt(info, type_0),
            wanted,
        );
    }
}
unsafe fn InitKeyTypesInfo(
    mut info: *mut KeyTypesInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        std::ptr::write_bytes::<KeyTypesInfo>(info as *mut KeyTypesInfo, 0u8, 1);
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0 as u32);
    }
}
unsafe fn ClearKeyTypeInfo(mut type_0: *mut KeyTypeInfo) {
    unsafe {
        darray_free(&mut (*type_0).entries.item, &mut (*type_0).entries.size, &mut (*type_0).entries.alloc);
        darray_free(&mut (*type_0).level_names.item, &mut (*type_0).level_names.size, &mut (*type_0).level_names.alloc);
    }
}
unsafe fn ClearKeyTypesInfo(mut info: *mut KeyTypesInfo) {
    unsafe {
        free((*info).name as *mut ::core::ffi::c_void);
        let mut type_0: *mut KeyTypeInfo = ::core::ptr::null_mut::<KeyTypeInfo>();
        if !(*info).types.item.is_null() {
            type_0 =
                (*info).types.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyTypeInfo;
            while type_0
                < (*info).types.item.offset((*info).types.size as isize) as *mut KeyTypeInfo
            {
                ClearKeyTypeInfo(type_0);
                type_0 = type_0.offset(1);
            }
        }
        darray_free(&mut (*info).types.item, &mut (*info).types.size, &mut (*info).types.alloc);
    }
}
unsafe fn FindMatchingKeyType(
    mut info: *mut KeyTypesInfo,
    mut name: xkb_atom_t,
) -> *mut KeyTypeInfo {
    unsafe {
        let mut old: *mut KeyTypeInfo = ::core::ptr::null_mut::<KeyTypeInfo>();
        if !(*info).types.item.is_null() {
            old = (*info).types.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyTypeInfo;
            while old < (*info).types.item.offset((*info).types.size as isize) as *mut KeyTypeInfo {
                if (*old).name == name {
                    return old;
                }
                old = old.offset(1);
            }
        }
        return ::core::ptr::null_mut::<KeyTypeInfo>();
    }
}
unsafe fn AddKeyType(
    mut info: *mut KeyTypesInfo,
    mut new: *mut KeyTypeInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut old: *mut KeyTypeInfo = ::core::ptr::null_mut::<KeyTypeInfo>();
        let verbosity: ::core::ffi::c_int =
            xkb_context_get_log_verbosity((*info).ctx) as ::core::ffi::c_int;
        old = FindMatchingKeyType(info, (*new).name);
        if !old.is_null() {
            if (*new).merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32 {
                if same_file as ::core::ffi::c_int != 0 && verbosity > 0 as ::core::ffi::c_int
                    || verbosity > 9 as ::core::ffi::c_int
                {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Multiple definitions of the {} key type; Earlier definition ignored\n",
                        XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS
                            as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    );
                }
                ClearKeyTypeInfo(old);
                *old = *new;
                (*new).entries.item = ::core::ptr::null_mut::<xkb_key_type_entry>();
                (*new).entries.size = 0 as darray_size_t;
                (*new).entries.alloc = 0 as darray_size_t;
                (*new).level_names.item = ::core::ptr::null_mut::<xkb_atom_t>();
                (*new).level_names.size = 0 as darray_size_t;
                (*new).level_names.alloc = 0 as darray_size_t;
                return true_0 != 0;
            }
            if same_file {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                    "[XKB-{:03}] Multiple definitions of the {} key type; Later definition ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                );
            }
            ClearKeyTypeInfo(new);
            return true_0 != 0;
        }
        darray_append(&mut (*info).types.item, &mut (*info).types.size, &mut (*info).types.alloc, *new);
        return true_0 != 0;
    }
}
unsafe fn MergeIncludedKeyTypes(
    mut into: *mut KeyTypesInfo,
    mut from: *mut KeyTypesInfo,
    mut merge: merge_mode,
) {
    unsafe {
        if (*from).errorCount > 0 as ::core::ffi::c_int {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        MergeModSets(
            (*into).ctx,
            &raw mut (*into).mods,
            &raw mut (*from).mods,
            merge,
        );
        if (*into).name.is_null() {
            (*into).name =
                _steal(&raw mut (*from).name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        }
        if (*into).types.size == 0 as darray_size_t {
            (*into).types = (*from).types;
            (*from).types.item = ::core::ptr::null_mut::<KeyTypeInfo>();
            (*from).types.size = 0 as darray_size_t;
            (*from).types.alloc = 0 as darray_size_t;
        } else {
            let mut type_0: *mut KeyTypeInfo = ::core::ptr::null_mut::<KeyTypeInfo>();
            if !(*from).types.item.is_null() {
                type_0 =
                    (*from).types.item.offset(0 as ::core::ffi::c_int as isize) as *mut KeyTypeInfo;
                while type_0
                    < (*from).types.item.offset((*from).types.size as isize) as *mut KeyTypeInfo
                {
                    (*type_0).merge = merge;
                    if !AddKeyType(into, type_0, false_0 != 0) {
                        (*into).errorCount += 1;
                    }
                    type_0 = type_0.offset(1);
                }
            }
            darray_free(&mut (*from).types.item, &mut (*from).types.size, &mut (*from).types.alloc);
        };
    }
}
unsafe fn HandleIncludeKeyTypes(
    mut info: *mut KeyTypesInfo,
    mut include: *mut IncludeStmt,
) -> bool {
    unsafe {
        let mut included: KeyTypesInfo = KeyTypesInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            types: C2Rust_Unnamed_16 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<KeyTypeInfo>(),
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as ::core::ffi::c_int;
            return false_0 != 0;
        }
        InitKeyTypesInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as u32),
            &raw mut (*info).mods,
        );
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyTypesInfo = KeyTypesInfo {
                name: ::core::ptr::null_mut::<i8>(),
                errorCount: 0,
                include_depth: 0,
                types: C2Rust_Unnamed_16 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<KeyTypeInfo>(),
                },
                mods: xkb_mod_set {
                    mods: [xkb_mod {
                        name: 0,
                        type_0: 0 as mod_type,
                        mapping: 0,
                    }; 32],
                    num_mods: 0,
                    explicit_vmods: 0,
                },
                ctx: ::core::ptr::null_mut::<xkb_context>(),
                keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            };
            let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_TYPES,
                &raw mut path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
            );
            if file.is_null() {
                (*info).errorCount += 10 as ::core::ffi::c_int;
                ClearKeyTypesInfo(&raw mut included);
                return false_0 != 0;
            }
            InitKeyTypesInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
                &raw mut included.mods,
            );
            HandleKeyTypesFile(&raw mut next_incl, file);
            MergeIncludedKeyTypes(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearKeyTypesInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedKeyTypes(info, &raw mut included, (*include).merge);
        ClearKeyTypesInfo(&raw mut included);
        return (*info).errorCount == 0 as ::core::ffi::c_int;
    }
}
unsafe fn SetModifiers(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if !arrayNdx.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "The modifiers field of a key type is not an array; Illegal array subscript ignored\n",
            );
            return false_0 != 0;
        }
        if !ExprResolveModMask(
            (*info).ctx,
            value,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut mods,
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Key type mask field must be a modifier mask; Key type definition ignored\n",
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        if (*type_0).defined as u32 & TYPE_FIELD_MASK as ::core::ffi::c_int as u32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "Multiple modifier mask definitions for key type {}; Using {}, ignoring {}\n",
                crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*type_0).name)),
                crate::xkb::utils::CStrDisplay(TypeMaskTxt(info, type_0)),
                crate::xkb::utils::CStrDisplay(ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods)),
            );
            return false_0 != 0;
        }
        (*type_0).mods = mods;
        return true_0 != 0;
    }
}
unsafe fn FindMatchingMapEntry(
    mut type_0: *mut KeyTypeInfo,
    mut mods: xkb_mod_mask_t,
) -> *mut xkb_key_type_entry {
    unsafe {
        let mut entry: *mut xkb_key_type_entry = ::core::ptr::null_mut::<xkb_key_type_entry>();
        if !(*type_0).entries.item.is_null() {
            entry = (*type_0)
                .entries
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut xkb_key_type_entry;
            while entry
                < (*type_0)
                    .entries
                    .item
                    .offset((*type_0).entries.size as isize)
                    as *mut xkb_key_type_entry
            {
                if (*entry).mods.mods == mods {
                    return entry;
                }
                entry = entry.offset(1);
            }
        }
        return ::core::ptr::null_mut::<xkb_key_type_entry>();
    }
}
unsafe fn AddMapEntry(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut new: *mut xkb_key_type_entry,
    mut clobber: bool,
    mut report: bool,
) -> bool {
    unsafe {
        let mut old: *mut xkb_key_type_entry = ::core::ptr::null_mut::<xkb_key_type_entry>();
        old = FindMatchingMapEntry(type_0, (*new).mods.mods);
        if !old.is_null() {
            if report as ::core::ffi::c_int != 0 && (*old).level != (*new).level {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "[XKB-{:03}] Multiple map entries for {} in {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay(MapEntryTxt(info, new)),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                    (if clobber as ::core::ffi::c_int != 0 {
                        (*new).level
                    } else {
                        (*old).level
                    })
                    .wrapping_add(1 as xkb_level_index_t),
                    (if clobber as ::core::ffi::c_int != 0 {
                        (*old).level
                    } else {
                        (*new).level
                    })
                    .wrapping_add(1 as xkb_level_index_t),
                );
            } else {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_VERBOSE as ::core::ffi::c_int,
                    "[XKB-{:03}] Multiple occurrences of map[{}]= {} in {}; Ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay(MapEntryTxt(info, new)),
                    (*new).level.wrapping_add(1 as xkb_level_index_t),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                );
                return true_0 != 0;
            }
            if clobber {
                if (*new).level >= (*type_0).num_levels {
                    (*type_0).num_levels = (*new).level.wrapping_add(1 as xkb_level_index_t);
                }
                (*old).level = (*new).level;
            }
            return true_0 != 0;
        }
        if (*new).level >= (*type_0).num_levels {
            (*type_0).num_levels = (*new).level.wrapping_add(1 as xkb_level_index_t);
        }
        darray_append(&mut (*type_0).entries.item, &mut (*type_0).entries.size, &mut (*type_0).entries.alloc, *new);
        return true_0 != 0;
    }
}
unsafe fn SetMapEntry(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut entry: xkb_key_type_entry = xkb_key_type_entry {
            level: 0,
            mods: xkb_mods { mods: 0, mask: 0 },
            preserve: xkb_mods { mods: 0, mask: 0 },
        };
        if arrayNdx.is_null() {
            return ReportTypeShouldBeArray(info, type_0, b"map entry\0".as_ptr() as *const i8);
        }
        if !ExprResolveModMask(
            (*info).ctx,
            arrayNdx,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut entry.mods.mods,
        ) {
            return ReportTypeBadType(
                info,
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
                type_0,
                b"map entry\0".as_ptr() as *const i8,
                b"modifier mask\0".as_ptr() as *const i8,
            );
        }
        if entry.mods.mods & !(*type_0).mods != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                "[XKB-{:03}] Map entry for modifiers not used by type {}; Using {} instead of {}\n",
                XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                crate::xkb::utils::CStrDisplay(ModMaskText(
                    (*info).ctx,
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    entry.mods.mods & (*type_0).mods,
                )),
                crate::xkb::utils::CStrDisplay(MapEntryTxt(info, &raw mut entry)),
            );
            entry.mods.mods &= (*type_0).mods;
        }
        if !ExprResolveLevel((*info).ctx, value, &raw mut entry.level) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Level specifications in a key type must be integer; Ignoring malformed level specification\n",
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        entry.preserve.mods = 0 as xkb_mod_mask_t;
        return AddMapEntry(info, type_0, &raw mut entry, true_0 != 0, true_0 != 0);
    }
}
unsafe fn AddPreserve(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut mods: xkb_mod_mask_t,
    mut preserve_mods: xkb_mod_mask_t,
) -> bool {
    unsafe {
        let mut entry: *mut xkb_key_type_entry = ::core::ptr::null_mut::<xkb_key_type_entry>();
        let mut new: xkb_key_type_entry = xkb_key_type_entry {
            level: 0,
            mods: xkb_mods { mods: 0, mask: 0 },
            preserve: xkb_mods { mods: 0, mask: 0 },
        };
        if !(*type_0).entries.item.is_null() {
            entry = (*type_0)
                .entries
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut xkb_key_type_entry;
            while entry
                < (*type_0)
                    .entries
                    .item
                    .offset((*type_0).entries.size as isize)
                    as *mut xkb_key_type_entry
            {
                if (*entry).mods.mods != mods {
                    entry = entry.offset(1);
                } else {
                    if (*entry).preserve.mods == 0 as xkb_mod_mask_t {
                        (*entry).preserve.mods = preserve_mods;
                        return true_0 != 0;
                    }
                    if (*entry).preserve.mods == preserve_mods {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_VERBOSE as ::core::ffi::c_int,
                            "[XKB-{:03}] Identical definitions for preserve[{}] in {}; Ignored\n",
                            XKB_WARNING_DUPLICATE_ENTRY as ::core::ffi::c_int,
                            crate::xkb::utils::CStrDisplay(ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods)),
                            crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                        );
                        return true_0 != 0;
                    }
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                        "[XKB-{:03}] Multiple definitions for preserve[{}] in {}; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES
                            as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods)),
                        crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                        crate::xkb::utils::CStrDisplay(ModMaskText(
                            (*info).ctx,
                            MOD_BOTH,
                            &raw mut (*info).mods,
                            preserve_mods,
                        )),
                        crate::xkb::utils::CStrDisplay(ModMaskText(
                            (*info).ctx,
                            MOD_BOTH,
                            &raw mut (*info).mods,
                            (*entry).preserve.mods,
                        )),
                    );
                    (*entry).preserve.mods = preserve_mods;
                    return true_0 != 0;
                }
            }
        }
        new.level = 0 as xkb_level_index_t;
        new.mods.mods = mods;
        new.preserve.mods = preserve_mods;
        darray_append(&mut (*type_0).entries.item, &mut (*type_0).entries.size, &mut (*type_0).entries.alloc, new);
        return true_0 != 0;
    }
}
unsafe fn SetPreserve(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            return ReportTypeShouldBeArray(
                info,
                type_0,
                b"preserve entry\0".as_ptr() as *const i8,
            );
        }
        let mut mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if !ExprResolveModMask(
            (*info).ctx,
            arrayNdx,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut mods,
        ) {
            return ReportTypeBadType(
                info,
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_,
                type_0,
                b"preserve entry\0".as_ptr() as *const i8,
                b"modifier mask\0".as_ptr() as *const i8,
            );
        }
        if mods & !(*type_0).mods != 0 {
            let mut before: *const i8 = ::core::ptr::null::<i8>();
            let mut after: *const i8 = ::core::ptr::null::<i8>();
            before = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods);
            mods &= (*type_0).mods;
            after = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods);
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                "[XKB-{:03}] Preserve entry for modifiers not used by the {} type; Index {} converted to {}\n",
                XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                crate::xkb::utils::CStrDisplay(before),
                crate::xkb::utils::CStrDisplay(after),
            );
        }
        let mut preserve_mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if !ExprResolveModMask(
            (*info).ctx,
            value,
            MOD_BOTH,
            &raw mut (*info).mods,
            &raw mut preserve_mods,
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Preserve value in a key type is not a modifier mask; Ignoring preserve[{}] in type {}\n",
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods)),
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
            );
            return false_0 != 0;
        }
        if preserve_mods & !mods != 0 {
            let mut before_0: *const i8 = ::core::ptr::null::<i8>();
            let mut after_0: *const i8 = ::core::ptr::null::<i8>();
            before_0 = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, preserve_mods);
            preserve_mods &= mods;
            after_0 = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, preserve_mods);
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                "[XKB-{:03}] Illegal value for preserve[{}] in type {}; Converted {} to {}\n",
                XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods)),
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                crate::xkb::utils::CStrDisplay(before_0),
                crate::xkb::utils::CStrDisplay(after_0),
            );
        }
        return AddPreserve(info, type_0, mods, preserve_mods);
    }
}
unsafe fn AddLevelName(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut level: xkb_level_index_t,
    mut name: xkb_atom_t,
    mut clobber: bool,
) -> bool {
    unsafe {
        if level >= (*type_0).level_names.size as xkb_level_index_t {
            darray_resize_zero(&mut (*type_0).level_names.item, &mut (*type_0).level_names.size, &mut (*type_0).level_names.alloc, (level as darray_size_t).wrapping_add(1 as darray_size_t));
        } else {
            if *(*type_0).level_names.item.offset(level as isize) == name {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_VERBOSE as ::core::ffi::c_int,
                    "[XKB-{:03}] Duplicate names for level {} of key type {}; Ignored\n",
                    XKB_WARNING_DUPLICATE_ENTRY as ::core::ffi::c_int,
                    level.wrapping_add(1 as xkb_level_index_t),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                );
                return true_0 != 0;
            }
            if *(*type_0).level_names.item.offset(level as isize) != XKB_ATOM_NONE as xkb_atom_t {
                let mut old: *const i8 = ::core::ptr::null::<i8>();
                let mut new: *const i8 = ::core::ptr::null::<i8>();
                old = xkb_atom_text(
                    (*info).ctx,
                    *(*type_0).level_names.item.offset(level as isize),
                );
                new = xkb_atom_text((*info).ctx, name);
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                    "[XKB-{:03}] Multiple names for level {} of key type {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES as ::core::ffi::c_int,
                    level.wrapping_add(1 as xkb_level_index_t),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                    crate::xkb::utils::CStrDisplay(if clobber as ::core::ffi::c_int != 0 { new } else { old }),
                    crate::xkb::utils::CStrDisplay(if clobber as ::core::ffi::c_int != 0 { old } else { new }),
                );
                if !clobber {
                    return true_0 != 0;
                }
            }
        }
        *(*type_0).level_names.item.offset(level as isize) = name;
        return true_0 != 0;
    }
}
unsafe fn SetLevelName(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            return ReportTypeShouldBeArray(info, type_0, b"level name\0".as_ptr() as *const i8);
        }
        let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
        if !ExprResolveLevel((*info).ctx, arrayNdx, &raw mut level) {
            return ReportTypeBadType(
                info,
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
                type_0,
                b"level name\0".as_ptr() as *const i8,
                b"integer\0".as_ptr() as *const i8,
            );
        }
        let mut level_name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, value, &raw mut level_name) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Non-string name for level {} in key type {}; Ignoring illegal level name definition\n",
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                level.wrapping_add(1 as xkb_level_index_t),
                crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*type_0).name)),
            );
            return false_0 != 0;
        }
        return AddLevelName(info, type_0, level, level_name, true_0 != 0);
    }
}
unsafe fn SetKeyTypeField(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut field: *const i8,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ok: bool = false_0 != 0;
        let mut type_field: type_field = 0 as type_field;
        if istreq(field, b"modifiers\0".as_ptr() as *const i8) {
            type_field = TYPE_FIELD_MASK;
            ok = SetModifiers(info, type_0, arrayNdx, value);
        } else if istreq(field, b"map\0".as_ptr() as *const i8) {
            type_field = TYPE_FIELD_MAP;
            ok = SetMapEntry(info, type_0, arrayNdx, value);
        } else if istreq(field, b"preserve\0".as_ptr() as *const i8) {
            type_field = TYPE_FIELD_PRESERVE;
            ok = SetPreserve(info, type_0, arrayNdx, value);
        } else if istreq(field, b"levelname\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
            || istreq(field, b"level_name\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            type_field = TYPE_FIELD_LEVEL_NAME;
            ok = SetLevelName(info, type_0, arrayNdx, value);
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Unknown field \"{}\" in key type \"{}\"; Definition ignored\n",
                XKB_ERROR_UNKNOWN_FIELD as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(field),
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
            );
            ok = (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_TYPE_FIELDS as ::core::ffi::c_int as u32
                == 0;
        }
        (*type_0).defined = ((*type_0).defined as u32 | type_field as u32) as type_field;
        return ok;
    }
}
unsafe fn HandleKeyTypeBody(
    mut info: *mut KeyTypesInfo,
    mut def: *mut VarDef,
    mut type_0: *mut KeyTypeInfo,
) -> bool {
    unsafe {
        let mut ok: bool = true_0 != 0;
        let mut elem: *const i8 = ::core::ptr::null::<i8>();
        let mut field: *const i8 = ::core::ptr::null::<i8>();
        let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        while !def.is_null() {
            if !ExprResolveLhs(
                (*info).ctx,
                (*def).name,
                &raw mut elem,
                &raw mut field,
                &raw mut arrayNdx,
            ) {
                ok = false_0 != 0;
            } else if !elem.is_null() {
                if istreq(elem, b"type\0".as_ptr() as *const i8) {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Support for changing the default type has been removed; Statement \"{}.{}\" ignored.\n",
                        XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(field),
                    );
                } else {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key type statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(field),
                    );
                    ok = false_0 != 0;
                }
            } else if !SetKeyTypeField(info, type_0, field, arrayNdx, (*def).value as *mut ExprDef)
            {
                ok = false_0 != 0;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return ok;
    }
}
unsafe fn HandleKeyTypeDef(mut info: *mut KeyTypesInfo, mut def: *mut KeyTypeDef) -> bool {
    unsafe {
        let mut type_0: KeyTypeInfo = KeyTypeInfo {
            defined: 0 as type_field,
            merge: (*def).merge,
            name: (*def).name,
            mods: 0 as xkb_mod_mask_t,
            num_levels: 1 as xkb_level_index_t,
            entries: C2Rust_Unnamed_18 {
                size: 0 as darray_size_t,
                alloc: 0 as darray_size_t,
                item: ::core::ptr::null_mut::<xkb_key_type_entry>(),
            },
            level_names: C2Rust_Unnamed_17 {
                size: 0 as darray_size_t,
                alloc: 0 as darray_size_t,
                item: ::core::ptr::null_mut::<xkb_atom_t>(),
            },
        };
        if !HandleKeyTypeBody(info, (*def).body, &raw mut type_0)
            || !AddKeyType(info, &raw mut type_0, true_0 != 0)
        {
            (*info).errorCount += 1;
            ClearKeyTypeInfo(&raw mut type_0);
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
unsafe fn HandleGlobalVar(mut info: *mut KeyTypesInfo, mut stmt: *mut VarDef) -> bool {
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
        } else if !elem.is_null()
            && istreq(elem, b"type\0".as_ptr() as *const i8) as ::core::ffi::c_int != 0
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Support for changing the default type has been removed; Statement ignored\n",
                XKB_ERROR_WRONG_STATEMENT_TYPE as ::core::ffi::c_int,
            );
            return true_0 != 0;
        } else if !elem.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Default defined for unknown element \"{}\"; Value for field \"{}.{}\" ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_STATEMENTS as ::core::ffi::c_int as u32
                == 0;
        } else if !field.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS as ::core::ffi::c_int as u32
                == 0;
        }
        return false_0 != 0;
    }
}
unsafe fn HandleKeyTypesFile(mut info: *mut KeyTypesInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        free((*info).name as *mut ::core::ffi::c_void);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
                1 => {
                    ok = HandleIncludeKeyTypes(info, stmt as *mut IncludeStmt);
                }
                27 => {
                    ok = HandleKeyTypeDef(info, stmt as *mut KeyTypeDef);
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef((*info).ctx, &raw mut (*info).mods, stmt as *mut VModDef);
                }
                35 | 36 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Unsupported types {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as ::core::ffi::c_int,
                        crate::xkb::utils::CStrDisplay(if (*stmt).type_0 as u32
                            == STMT_UNKNOWN_COMPOUND as ::core::ffi::c_int as u32
                        {
                            b"compound\0".as_ptr() as *const i8
                        } else {
                            b"declaration\0".as_ptr() as *const i8
                        }),
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
                        "[XKB-{:03}] Key type files may not include other declarations; Ignoring {}\n",
                        XKB_ERROR_WRONG_STATEMENT_TYPE as ::core::ffi::c_int,
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
                    "[XKB-{:03}] Abandoning keytypes file \"{}\"\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as ::core::ffi::c_int,
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn CopyKeyTypesToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut KeyTypesInfo) -> bool {
    unsafe {
        let num_types: darray_size_t = if (*info).types.size == 0 as darray_size_t {
            1 as darray_size_t
        } else {
            (*info).types.size
        };
        let mut types: *mut xkb_key_type = calloc(
            num_types as usize,
            ::core::mem::size_of::<xkb_key_type>() as usize,
        ) as *mut xkb_key_type;
        if types.is_null() {
            return false_0 != 0;
        }
        if (*info).types.size == 0 as darray_size_t {
            let mut type_0: *mut xkb_key_type =
                types.offset(0 as ::core::ffi::c_int as isize) as *mut xkb_key_type;
            (*type_0).mods.mods = 0 as xkb_mod_mask_t;
            (*type_0).num_levels = 1 as xkb_level_index_t;
            (*type_0).entries = ::core::ptr::null_mut::<xkb_key_type_entry>();
            (*type_0).num_entries = 0 as darray_size_t;
            (*type_0).name = xkb_atom_intern(
                (*keymap).ctx,
                b"ONE_LEVEL\0".as_ptr() as *const i8,
                (::core::mem::size_of::<[i8; 10]>() as usize).wrapping_sub(1 as usize),
            );
            (*type_0).level_names = ::core::ptr::null_mut::<xkb_atom_t>();
            (*type_0).num_level_names = 0 as xkb_level_index_t;
            (*type_0).required = true_0 != 0;
        } else {
            let canonical_types: [xkb_atom_t; 4] = [
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"ONE_LEVEL\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 10]>() as usize).wrapping_sub(1 as usize),
                ),
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"TWO_LEVEL\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 10]>() as usize).wrapping_sub(1 as usize),
                ),
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"ALPHABETIC\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 11]>() as usize).wrapping_sub(1 as usize),
                ),
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"KEYPAD\0".as_ptr() as *const i8,
                    (::core::mem::size_of::<[i8; 7]>() as usize).wrapping_sub(1 as usize),
                ),
            ];
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < num_types {
                let mut def: *mut KeyTypeInfo =
                    (*info).types.item.offset(i as isize) as *mut KeyTypeInfo;
                let mut type_1: *mut xkb_key_type = types.offset(i as isize) as *mut xkb_key_type;
                (*type_1).name = (*def).name;
                (*type_1).mods.mods = (*def).mods;
                (*type_1).num_levels = (*def).num_levels;
                (*type_1).num_level_names = (*def).level_names.size as xkb_level_index_t;
                (*type_1).level_names = (*def).level_names.item;
                if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                    *(::core::ptr::null_mut::<::core::ffi::c_void>() as *mut darray_size_t) =
                        (*def).level_names.size;
                }
                (*def).level_names.item = ::core::ptr::null_mut::<xkb_atom_t>();
                (*def).level_names.size = 0 as darray_size_t;
                (*def).level_names.alloc = 0 as darray_size_t;
                (*type_1).entries = (*def).entries.item;
                if !(&raw mut (*type_1).num_entries).is_null() {
                    *&raw mut (*type_1).num_entries = (*def).entries.size;
                }
                (*def).entries.item = ::core::ptr::null_mut::<xkb_key_type_entry>();
                (*def).entries.size = 0 as darray_size_t;
                (*def).entries.alloc = 0 as darray_size_t;
                (*type_1).required = false_0 != 0;
                if (*type_1).num_levels <= 2 as xkb_level_index_t {
                    let mut t: uint8_t = 0 as uint8_t;
                    while (t as ::core::ffi::c_int)
                        < (::core::mem::size_of::<[xkb_atom_t; 4]>() as usize)
                            .wrapping_div(::core::mem::size_of::<xkb_atom_t>() as usize)
                            as uint8_t as ::core::ffi::c_int
                    {
                        if (*type_1).name == canonical_types[t as usize] {
                            (*type_1).required = true_0 != 0;
                            break;
                        } else {
                            t = t.wrapping_add(1);
                        }
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        (*keymap).types_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).types_section_name);
        (*keymap).num_types = num_types;
        (*keymap).types = types;
        (*keymap).mods = (*info).mods;
        return true_0 != 0;
    }
}
pub unsafe fn CompileKeyTypes(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: KeyTypesInfo = KeyTypesInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            types: C2Rust_Unnamed_16 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<KeyTypeInfo>(),
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        InitKeyTypesInfo(
            &raw mut info,
            keymap_info,
            0 as u32,
            &raw mut (*keymap_info).keymap.mods,
        );
        if !file.is_null() {
            HandleKeyTypesFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as ::core::ffi::c_int) {
            if CopyKeyTypesToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearKeyTypesInfo(&raw mut info);
                return true_0 != 0;
            }
        }
        ClearKeyTypesInfo(&raw mut info);
        return false_0 != 0;
    }
}
