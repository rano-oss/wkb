use crate::xkb_logf;
pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
    pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}

pub mod context_h {
    pub use crate::xkb::context_priv::{xkb_atom_text, xkb_context_get_buffer};
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
    pub const XKB_ATOM_NONE: i32 = 0 as i32;
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
        XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
        XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED,
        XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
    pub type xkb_led_mask_t = u32;
    pub use crate::xkb::context::xkb_context_get_log_verbosity;
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_overlay_index_t = u8;
    pub type C2Rust_Unnamed_13 = u32;
    pub const FALLBACK_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_13 = 0;
    pub const DEFAULT_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_13 = 1;
    pub const DEFAULT_KEY_REPEAT: C2Rust_Unnamed_13 = 0;
    pub type C2Rust_Unnamed_14 = u32;
    pub const FALLBACK_INTERPRET_VMODMAP: C2Rust_Unnamed_14 = 0;
    pub const DEFAULT_INTERPRET_VMODMAP: C2Rust_Unnamed_14 = 0;
    pub const DEFAULT_INTERPRET_VMOD: C2Rust_Unnamed_14 = 4294967295;
    pub const DEFAULT_KEY_VMODMAP: C2Rust_Unnamed_14 = 0;
    pub const XKB_MAX_LEDS: xkb_led_index_t = (::core::mem::size_of::<xkb_led_mask_t>() as usize)
        .wrapping_mul(CHAR_BIT as usize)
        as xkb_led_index_t;
    pub const MAX_ACTIONS_PER_LEVEL: i32 = u16::MAX as i32;
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = i32;
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
    pub type C2Rust_Unnamed_15 = DarrayKeysym;
    pub use crate::xkb::xkbcomp::ast_build::stmt_type_to_string;
}
pub mod text_h {
    pub use crate::xkb::text::{
        ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames,
        useModMapValueNames, KeysymText, LookupEntry, LookupString, ModMaskText, SIMatchText,
    };
}
pub mod xkbcomp_priv_h {
    pub use crate::xkb::shared_ast_types::{
        pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
        xkb_message_code, xkb_parser_error, xkb_parser_strict_flags, ReportBadField, ReportBadType,
        ReportNotArray, XkbcompFeatures, XkbcompLookup, PARSER_FATAL_ERROR,
        PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
        PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
        PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
        PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS,
        PARSER_V1_LAX_FLAGS, PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
    };
    pub type C2Rust_Unnamed_16 = XkbcompLookup;
    pub type C2Rust_Unnamed_17 = XkbcompFeatures;
    pub use crate::xkb::xkbcomp::ast_build::FreeXkbFile;
}
pub mod action_h {
    pub use crate::xkb::xkbcomp::action::action_h::ActionsInfo;
    pub use crate::xkb::xkbcomp::action::{
        HandleActionDef, InitActionsInfo, SetDefaultActionField,
    };
}
pub mod stdlib_h {
    extern "C" {
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn istreq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as i32;
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
    pub const CHAR_BIT: i32 = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
pub mod vmod_h {
    pub use crate::xkb::xkbcomp::vmod::{HandleVModDef, InitVMods, MergeModSets};
}
pub mod expr_h {

    pub use crate::xkb::xkbcomp::expr::{
        ExprResolveBoolean, ExprResolveEnum, ExprResolveGroupMask, ExprResolveLhs, ExprResolveMask,
        ExprResolveMod, ExprResolveModMask,
    };
}
pub mod util_mem_h {
    #[inline]
    pub unsafe fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
        let mut swapped: *mut ::core::ffi::c_void = *original;
        *original = std::ptr::null_mut::<core::ffi::c_void>();
        return swapped;
    }
}
pub mod include_h {
    pub use crate::xkb::xkbcomp::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: i32 = 0 as i32;
}

pub use self::action_h::{ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField};
pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_15, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, InterpDef, LedMapDef, ParseCommon,
    UnknownStatement, VModDef, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES,
    FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
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
    xkb_atom_text, xkb_context, xkb_context_get_buffer, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
use self::expr_h::{
    ExprResolveBoolean, ExprResolveEnum, ExprResolveGroupMask, ExprResolveLhs, ExprResolveMask,
    ExprResolveMod, ExprResolveModMask,
};
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
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_13, C2Rust_Unnamed_14,
    C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6,
    C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, _ACTION_TYPE_NUM_ENTRIES,
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
    DEFAULT_INTERPRET_KEY_REPEAT, DEFAULT_INTERPRET_VMOD, DEFAULT_INTERPRET_VMODMAP,
    DEFAULT_KEY_REPEAT, DEFAULT_KEY_VMODMAP, EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT,
    EXPLICIT_SYMBOLS, EXPLICIT_TYPES, EXPLICIT_VMODMAP, FALLBACK_INTERPRET_KEY_REPEAT,
    FALLBACK_INTERPRET_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL,
    MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MAX_ACTIONS_PER_LEVEL, MOD_BOTH,
    MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT, XKB_MAX_LEDS,
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
use self::stdlib_h::{free, realloc};
pub use self::text_h::{
    ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames,
    useModMapValueNames, KeysymText, LookupEntry, LookupString, ModMaskText, SIMatchText,
};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{istrcmp, istreq, strdup_safe};
use self::vmod_h::{HandleVModDef, InitVMods, MergeModSets};
pub use self::xkbcommon_h::{
    xkb_context_get_log_verbosity, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_led_index_t, xkb_led_mask_t, xkb_level_index_t, xkb_log_level, xkb_mod_index_t,
    xkb_mod_mask_t, xkb_rule_names, xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_error, xkb_parser_strict_flags, C2Rust_Unnamed_16, C2Rust_Unnamed_17, FreeXkbFile,
    ReportBadField, ReportBadType, ReportNotArray, PARSER_FATAL_ERROR,
    PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS,
    PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
    PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
    PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use crate::xkb::keymap_priv::XkbEscapeMapName;
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::utils::{darray_append, darray_free};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompatInfo {
    pub name: *mut i8,
    pub errorCount: i32,
    pub include_depth: u32,
    pub default_interp: SymInterpInfo,
    pub interps: C2Rust_Unnamed_18,
    pub default_led: LedInfo,
    pub leds: [LedInfo; 32],
    pub num_leds: xkb_led_index_t,
    pub default_actions: ActionsInfo,
    pub mods: xkb_mod_set,
    pub keymap_info: *const xkb_keymap_info,
    pub ctx: *mut xkb_context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedInfo {
    pub defined: led_field,
    pub merge: merge_mode,
    pub led: xkb_led,
}
pub type led_field = u32;
pub const LED_FIELD_CTRLS: led_field = 4;
pub const LED_FIELD_GROUPS: led_field = 2;
pub const LED_FIELD_MODS: led_field = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut SymInterpInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymInterpInfo {
    pub defined: si_field,
    pub merge: merge_mode,
    pub interp: xkb_sym_interpret,
}
pub type si_field = u32;
pub const SI_FIELD_LEVEL_ONE_ONLY: si_field = 8;
pub const SI_FIELD_AUTO_REPEAT: si_field = 4;
pub const SI_FIELD_ACTION: si_field = 2;
pub const SI_FIELD_VIRTUAL_MOD: si_field = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_sym_interpret,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collect {
    pub sym_interprets: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_action,
}
unsafe fn siText(mut si: *mut SymInterpInfo, mut info: *mut CompatInfo) -> *const i8 {
    unsafe {
        let mut buf: *mut i8 = xkb_context_get_buffer((*info).ctx, 128 as usize);
        if si == &raw mut (*info).default_interp {
            return b"default\0".as_ptr() as *const i8;
        }
        crate::xkb::utils::snprintf_args(
            buf,
            128 as usize,
            format_args!(
                "{}+{}({})",
                crate::xkb::utils::CStrDisplay(KeysymText((*info).ctx, (*si).interp.sym)),
                crate::xkb::utils::CStrDisplay(SIMatchText((*si).interp.match_0)),
                crate::xkb::utils::CStrDisplay(ModMaskText(
                    (*info).ctx,
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    (*si).interp.mods,
                )),
            ),
        );
        return buf;
    }
}
#[inline]
unsafe fn ReportSINotArray(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    mut field: *const i8,
) -> bool {
    unsafe {
        return ReportNotArray(
            (*info).ctx,
            b"symbol interpretation\0".as_ptr() as *const i8,
            field,
            siText(si, info),
        );
    }
}
#[inline]
unsafe fn ReportSIBadType(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    mut field: *const i8,
    mut wanted: *const i8,
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            b"symbol interpretation\0".as_ptr() as *const i8,
            field,
            siText(si, info),
            wanted,
        );
    }
}
unsafe fn LEDText(mut info: *mut CompatInfo, mut ledi: *mut LedInfo) -> *const i8 {
    unsafe {
        if ledi == &raw mut (*info).default_led {
            return b"default\0".as_ptr() as *const i8;
        } else {
            return xkb_atom_text((*info).ctx, (*ledi).led.name);
        };
    }
}
#[inline]
unsafe fn ReportLedBadType(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    mut field: *const i8,
    mut wanted: *const i8,
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            b"indicator map\0".as_ptr() as *const i8,
            field,
            LEDText(info, ledi),
            wanted,
        );
    }
}
#[inline]
unsafe fn ReportLedNotArray(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    mut field: *const i8,
) -> bool {
    unsafe {
        return ReportNotArray(
            (*info).ctx,
            b"indicator map\0".as_ptr() as *const i8,
            field,
            LEDText(info, ledi),
        );
    }
}
#[inline]
unsafe fn InitInterp(mut info: *mut SymInterpInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
        (*info).interp.virtual_mod = XKB_MOD_INVALID as xkb_mod_index_t;
    }
}
#[inline]
unsafe fn InitLED(mut info: *mut LedInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
    }
}
unsafe fn InitCompatInfo(
    mut info: *mut CompatInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        std::ptr::write_bytes::<CompatInfo>(info as *mut CompatInfo, 0u8, 1);
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        InitActionsInfo(
            &raw const (*keymap_info).keymap,
            &raw mut (*info).default_actions,
        );
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0 as u32);
        InitInterp(&raw mut (*info).default_interp);
        InitLED(&raw mut (*info).default_led);
    }
}
unsafe fn ClearCompatInfo(mut info: *mut CompatInfo) {
    unsafe {
        free((*info).name as *mut ::core::ffi::c_void);
        darray_free(
            &mut (*info).interps.item,
            &mut (*info).interps.size,
            &mut (*info).interps.alloc,
        );
    }
}
unsafe fn FindMatchingInterp(
    mut info: *mut CompatInfo,
    mut new: *mut SymInterpInfo,
) -> *mut SymInterpInfo {
    unsafe {
        let mut old: *mut SymInterpInfo = ::core::ptr::null_mut::<SymInterpInfo>();
        if !(*info).interps.item.is_null() {
            old = (*info).interps.item.offset(0 as i32 as isize) as *mut SymInterpInfo;
            while old
                < (*info).interps.item.offset((*info).interps.size as isize) as *mut SymInterpInfo
            {
                if (*old).interp.sym == (*new).interp.sym
                    && (*old).interp.mods == (*new).interp.mods
                    && (*old).interp.match_0 as u32 == (*new).interp.match_0 as u32
                {
                    return old;
                }
                old = old.offset(1);
            }
        }
        return ::core::ptr::null_mut::<SymInterpInfo>();
    }
}
unsafe fn UseNewInterpField(
    mut field: si_field,
    mut old: si_field,
    mut new: si_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut si_field,
) -> bool {
    unsafe {
        if old as u32 & field as u32 == 0 {
            return new as u32 & field as u32 != 0;
        }
        if new as u32 & field as u32 != 0 {
            if report {
                *collide = (*collide as u32 | field as u32) as si_field;
            }
            return clobber;
        }
        return 0 != 0;
    }
}
unsafe fn MergeInterp(
    mut info: *mut CompatInfo,
    mut old: *mut SymInterpInfo,
    mut new: *mut SymInterpInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let clobber: bool = (*new).merge as u32 != MERGE_AUGMENT as i32 as u32;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let report: bool = same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32;
        let mut collide: si_field = 0 as si_field;
        if (*new).merge as u32 == MERGE_REPLACE as i32 as u32 {
            if report {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple definitions for \"{}\"; Earlier interpretation ignored\n",
                    crate::xkb::utils::CStrDisplay(siText(new, info)),
                );
            }
            *old = *new;
            return 1 != 0;
        }
        if UseNewInterpField(
            SI_FIELD_VIRTUAL_MOD,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.virtual_mod = (*new).interp.virtual_mod;
            (*old).defined =
                ((*old).defined as u32 | SI_FIELD_VIRTUAL_MOD as i32 as u32) as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_ACTION,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            if (*old).interp.num_actions as i32 > 1 as i32 {
                free((*old).interp.a.actions as *mut ::core::ffi::c_void);
            }
            (*old).interp.num_actions = (*new).interp.num_actions;
            if (*new).interp.num_actions as i32 > 1 as i32 {
                (*old).interp.a.actions = (*new).interp.a.actions;
                (*new).interp.a.action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                (*new).interp.num_actions = 0 as xkb_action_count_t;
            } else {
                (*old).interp.a.action = (*new).interp.a.action;
            }
            (*old).defined = ((*old).defined as u32 | SI_FIELD_ACTION as i32 as u32) as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_AUTO_REPEAT,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.set_repeat((*new).interp.repeat() as bool);
            (*old).defined =
                ((*old).defined as u32 | SI_FIELD_AUTO_REPEAT as i32 as u32) as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_LEVEL_ONE_ONLY,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.level_one_only = (*new).interp.level_one_only;
            (*old).defined =
                ((*old).defined as u32 | SI_FIELD_LEVEL_ONE_ONLY as i32 as u32) as si_field;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Multiple interpretations of \"{}\"; Using {} definition for duplicate fields\n",
                crate::xkb::utils::CStrDisplay(siText(old, info)),
                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                    b"last\0".as_ptr() as *const i8
                } else {
                    b"first\0".as_ptr() as *const i8
                }),
            );
        }
        return 1 != 0;
    }
}
unsafe fn AddInterp(
    mut info: *mut CompatInfo,
    mut new: *mut SymInterpInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut old: *mut SymInterpInfo = FindMatchingInterp(info, new);
        if !old.is_null() {
            return MergeInterp(info, old, new, same_file);
        }
        darray_append(
            &mut (*info).interps.item,
            &mut (*info).interps.size,
            &mut (*info).interps.alloc,
            *new,
        );
        return 1 != 0;
    }
}
unsafe fn ResolveStateAndPredicate(
    mut expr: *mut ExprDef,
    mut pred_rtrn: *mut xkb_match_operation,
    mut mods_rtrn: *mut xkb_mod_mask_t,
    mut info: *mut CompatInfo,
) -> bool {
    unsafe {
        if expr.is_null() {
            *pred_rtrn = MATCH_ANY_OR_NONE;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return 1 != 0;
        }
        *pred_rtrn = MATCH_EXACTLY;
        if (*expr).common.type_0 as u32 == STMT_EXPR_ACTION_DECL as i32 as u32 {
            let mut pred_txt: *const i8 = xkb_atom_text((*info).ctx, (*expr).action.name);
            let mut pred: u32 = 0 as u32;
            if !LookupString(
                &raw const symInterpretMatchMaskNames as *const LookupEntry,
                pred_txt,
                &raw mut pred,
            ) || (*expr).action.args.is_null()
                || !(*(*expr).action.args).common.next.is_null()
            {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Illegal modifier predicate \"{}\"; Ignored\n",
                    crate::xkb::utils::CStrDisplay(pred_txt),
                );
                return 0 != 0;
            }
            *pred_rtrn = pred as xkb_match_operation;
            expr = (*expr).action.args as *mut ExprDef;
        } else if (*expr).common.type_0 as u32 == STMT_EXPR_IDENT as i32 as u32 {
            let mut pred_txt_0: *const i8 = xkb_atom_text((*info).ctx, (*expr).ident.ident);
            if !pred_txt_0.is_null()
                && istreq(pred_txt_0, b"any\0".as_ptr() as *const i8) as i32 != 0
            {
                *pred_rtrn = MATCH_ANY;
                *mods_rtrn = MOD_REAL_MASK_ALL;
                return 1 != 0;
            }
        }
        return ExprResolveModMask(
            (*info).ctx,
            expr,
            MOD_REAL,
            &raw mut (*info).mods,
            mods_rtrn,
        );
    }
}
unsafe fn UseNewLEDField(
    mut field: led_field,
    mut old: led_field,
    mut new: led_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut led_field,
) -> bool {
    unsafe {
        if old as u32 & field as u32 == 0 {
            return new as u32 & field as u32 != 0;
        }
        if new as u32 & field as u32 != 0 {
            if report {
                *collide = (*collide as u32 | field as u32) as led_field;
            }
            return clobber;
        }
        return 0 != 0;
    }
}
unsafe fn MergeLedMap(
    mut info: *mut CompatInfo,
    mut old: *mut LedInfo,
    mut new: *mut LedInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut collide: led_field = 0 as led_field;
        let clobber: bool = (*new).merge as u32 != MERGE_AUGMENT as i32 as u32;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let report: bool = same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32;
        if (*old).led.mods.mods == (*new).led.mods.mods
            && (*old).led.pending_groups() as i32 == (*new).led.pending_groups() as i32
            && (*old).led.groups == (*new).led.groups
            && (*old).led.ctrls as u32 == (*new).led.ctrls as u32
            && (*old).led.which_mods as u32 == (*new).led.which_mods as u32
            && (*old).led.which_groups() as i32 == (*new).led.which_groups() as i32
        {
            (*old).defined = ((*old).defined as u32 | (*new).defined as u32) as led_field;
            return 1 != 0;
        }
        if (*new).merge as u32 == MERGE_REPLACE as i32 as u32 {
            if report {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Map for indicator {} redefined; Earlier definition ignored\n",
                    crate::xkb::utils::CStrDisplay(LEDText(info, old)),
                );
            }
            *old = *new;
            return 1 != 0;
        }
        collide = 0 as led_field;
        if UseNewLEDField(
            LED_FIELD_MODS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).led.which_mods = (*new).led.which_mods;
            (*old).led.mods = (*new).led.mods;
            (*old).defined = ((*old).defined as u32 | LED_FIELD_MODS as i32 as u32) as led_field;
        }
        if UseNewLEDField(
            LED_FIELD_GROUPS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old)
                .led
                .set_which_groups((*new).led.which_groups() as xkb_state_component);
            (*old).led.groups = (*new).led.groups;
            (*old)
                .led
                .set_pending_groups((*new).led.pending_groups() as bool);
            (*old).defined = ((*old).defined as u32 | LED_FIELD_GROUPS as i32 as u32) as led_field;
        }
        if UseNewLEDField(
            LED_FIELD_CTRLS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).led.ctrls = (*new).led.ctrls;
            (*old).defined = ((*old).defined as u32 | LED_FIELD_CTRLS as i32 as u32) as led_field;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Map for indicator {} redefined; Using {} definition for duplicate fields\n",
                crate::xkb::utils::CStrDisplay(LEDText(info, old)),
                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                    b"last\0".as_ptr() as *const i8
                } else {
                    b"first\0".as_ptr() as *const i8
                }),
            );
        }
        return 1 != 0;
    }
}
unsafe fn AddLedMap(mut info: *mut CompatInfo, mut new: *mut LedInfo, mut same_file: bool) -> bool {
    unsafe {
        let mut i: xkb_led_index_t = 0 as xkb_led_index_t;
        while i < (*info).num_leds {
            let mut old: *mut LedInfo =
                (&raw mut (*info).leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
            if (*old).led.name != (*new).led.name {
                i = i.wrapping_add(1);
            } else {
                return MergeLedMap(info, old, new, same_file);
            }
        }
        if (*info).num_leds >= XKB_MAX_LEDS {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Too many LEDs defined (maximum {})\n",
                (::core::mem::size_of::<xkb_led_mask_t>() as usize).wrapping_mul(8 as usize)
                    as xkb_led_index_t,
            );
            return 0 != 0;
        }
        let c2rust_fresh1 = (*info).num_leds;
        (*info).num_leds = (*info).num_leds.wrapping_add(1);
        (*info).leds[c2rust_fresh1 as usize] = *new;
        return 1 != 0;
    }
}
unsafe fn MergeIncludedCompatMaps(
    mut into: *mut CompatInfo,
    mut from: *mut CompatInfo,
    mut merge: merge_mode,
) {
    unsafe {
        if (*from).errorCount > 0 as i32 {
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
        if (*into).interps.size == 0 as darray_size_t {
            (*into).interps = (*from).interps;
            (*from).interps.item = ::core::ptr::null_mut::<SymInterpInfo>();
            (*from).interps.size = 0 as darray_size_t;
            (*from).interps.alloc = 0 as darray_size_t;
        } else {
            let mut si: *mut SymInterpInfo = ::core::ptr::null_mut::<SymInterpInfo>();
            if !(*from).interps.item.is_null() {
                si = (*from).interps.item.offset(0 as i32 as isize) as *mut SymInterpInfo;
                while si
                    < (*from).interps.item.offset((*from).interps.size as isize)
                        as *mut SymInterpInfo
                {
                    (*si).merge = merge;
                    if !AddInterp(into, si, 0 != 0) {
                        (*into).errorCount += 1;
                    }
                    si = si.offset(1);
                }
            }
        }
        if (*into).num_leds == 0 as xkb_led_index_t {
            std::ptr::copy_nonoverlapping::<LedInfo>(
                &raw mut (*from).leds as *mut LedInfo,
                &raw mut (*into).leds as *mut LedInfo,
                (*from).num_leds as usize,
            );
            (*into).num_leds = (*from).num_leds;
            (*from).num_leds = 0 as xkb_led_index_t;
        } else {
            let mut i: xkb_led_index_t = 0 as xkb_led_index_t;
            while i < (*from).num_leds {
                let mut ledi: *mut LedInfo =
                    (&raw mut (*from).leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
                (*ledi).merge = merge;
                if !AddLedMap(into, ledi, 0 != 0) {
                    (*into).errorCount += 1;
                }
                i = i.wrapping_add(1);
            }
        };
    }
}
unsafe fn HandleIncludeCompatMap(mut info: *mut CompatInfo, mut include: *mut IncludeStmt) -> bool {
    unsafe {
        let mut included: CompatInfo = CompatInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            default_interp: SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat_required: [0; 1],
                    num_actions: 0,
                    a: C2Rust_Unnamed_1 {
                        action: xkb_action {
                            type_0: ACTION_TYPE_NONE,
                        },
                    },
                },
            },
            interps: C2Rust_Unnamed_18 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<SymInterpInfo>(),
            },
            default_led: LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            },
            leds: [LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            }; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
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
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            ctx: ::core::ptr::null_mut::<xkb_context>(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as i32;
            return 0 != 0;
        }
        InitCompatInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as u32),
            &raw mut (*info).mods,
        );
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: CompatInfo = CompatInfo {
                name: ::core::ptr::null_mut::<i8>(),
                errorCount: 0,
                include_depth: 0,
                default_interp: SymInterpInfo {
                    defined: 0 as si_field,
                    merge: MERGE_DEFAULT,
                    interp: xkb_sym_interpret {
                        sym: 0,
                        match_0: MATCH_NONE,
                        mods: 0,
                        virtual_mod: 0,
                        level_one_only: false,
                        repeat_required: [0; 1],
                        num_actions: 0,
                        a: C2Rust_Unnamed_1 {
                            action: xkb_action {
                                type_0: ACTION_TYPE_NONE,
                            },
                        },
                    },
                },
                interps: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<SymInterpInfo>(),
                },
                default_led: LedInfo {
                    defined: 0 as led_field,
                    merge: MERGE_DEFAULT,
                    led: xkb_led {
                        name: 0,
                        which_groups_pending_groups: [0; 4],
                        groups: 0,
                        which_mods: 0 as xkb_state_component,
                        mods: xkb_mods { mods: 0, mask: 0 },
                        ctrls: 0 as xkb_action_controls,
                    },
                },
                leds: [LedInfo {
                    defined: 0 as led_field,
                    merge: MERGE_DEFAULT,
                    led: xkb_led {
                        name: 0,
                        which_groups_pending_groups: [0; 4],
                        groups: 0,
                        which_mods: 0 as xkb_state_component,
                        mods: xkb_mods { mods: 0, mask: 0 },
                        ctrls: 0 as xkb_action_controls,
                    },
                }; 32],
                num_leds: 0,
                default_actions: ActionsInfo {
                    actions: [xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    }; 21],
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
                keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
                ctx: ::core::ptr::null_mut::<xkb_context>(),
            };
            let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_COMPAT,
                &raw mut path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearCompatInfo(&raw mut included);
                return 0 != 0;
            }
            InitCompatInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
                &raw mut included.mods,
            );
            next_incl.default_interp = (*info).default_interp;
            next_incl.default_led = (*info).default_led;
            HandleCompatMapFile(&raw mut next_incl, file);
            MergeIncludedCompatMaps(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearCompatInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedCompatMaps(info, &raw mut included, (*include).merge);
        ClearCompatInfo(&raw mut included);
        return (*info).errorCount == 0 as i32;
    }
}
unsafe fn SetInterpField(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    mut field: *const i8,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        if istreq(field, b"action\0".as_ptr() as *const i8) {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_ACTION_LIST as i32 as u32 {
                let mut num_actions: u32 = 0 as u32;
                let mut act: *mut ExprDef = (*value).actions.actions as *mut ExprDef;
                while !act.is_null() {
                    num_actions = num_actions.wrapping_add(1);
                    act = (*act).common.next as *mut ExprDef;
                }
                if num_actions > MAX_ACTIONS_PER_LEVEL as u32 {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Interpret {} has too many actions; expected max {}, got: {}\n",
                        crate::xkb::utils::CStrDisplay(siText(si, info)),
                        65535 as i32,
                        num_actions,
                    );
                    return 0 != 0;
                }
                (*si).interp.num_actions = 0 as xkb_action_count_t;
                (*si).interp.a.action.type_0 = ACTION_TYPE_NONE;
                let mut actions: C2Rust_Unnamed_20 = C2Rust_Unnamed_20 {
                    size: 0 as darray_size_t,
                    alloc: 0 as darray_size_t,
                    item: ::core::ptr::null_mut::<xkb_action>(),
                };
                let mut act_0: *mut ExprDef = (*value).actions.actions as *mut ExprDef;
                while !act_0.is_null() {
                    let mut toAct: xkb_action = xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    };
                    match HandleActionDef(
                        (*info).keymap_info,
                        &raw mut (*info).default_actions,
                        &raw mut (*info).mods,
                        act_0,
                        &raw mut toAct,
                    ) as u32
                    {
                        1 => {
                            toAct.type_0 = ACTION_TYPE_NONE;
                        }
                        2 => {
                            darray_free(&mut actions.item, &mut actions.size, &mut actions.alloc);
                            return 0 != 0;
                        }
                        _ => {}
                    }
                    if !(toAct.type_0 as u32 == ACTION_TYPE_NONE as i32 as u32) {
                        if (num_actions == 1 as u32) as i32 as i64 != 0 {
                            (*si).interp.num_actions = 1 as xkb_action_count_t;
                            (*si).interp.a.action = toAct;
                        } else {
                            darray_append(
                                &mut actions.item,
                                &mut actions.size,
                                &mut actions.alloc,
                                toAct,
                            );
                        }
                    }
                    act_0 = (*act_0).common.next as *mut ExprDef;
                }
                match actions.size {
                    0 => {}
                    1 => {
                        (*si).interp.num_actions = 1 as xkb_action_count_t;
                        (*si).interp.a.action = *actions.item.offset(1 as i32 as isize);
                        darray_free(&mut actions.item, &mut actions.size, &mut actions.alloc);
                    }
                    _ => {
                        if actions.size > 0 as darray_size_t {
                            actions.alloc = actions.size;
                            actions.item = realloc(
                                actions.item as *mut ::core::ffi::c_void,
                                (actions.alloc as usize).wrapping_mul(::core::mem::size_of::<
                                    xkb_action,
                                >(
                                )
                                    as usize),
                            ) as *mut xkb_action;
                        }
                        (*si).interp.num_actions = actions.size as xkb_action_count_t;
                        (*si).interp.a.actions = actions.item;
                        if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                            *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                as *mut darray_size_t) = actions.size;
                        }
                        actions.item = ::core::ptr::null_mut::<xkb_action>();
                        actions.size = 0 as darray_size_t;
                        actions.alloc = 0 as darray_size_t;
                    }
                }
            } else {
                match HandleActionDef(
                    (*info).keymap_info,
                    &raw mut (*info).default_actions,
                    &raw mut (*info).mods,
                    value,
                    &raw mut (*si).interp.a.action,
                ) as u32
                {
                    1 => {
                        (*si).interp.a.action.type_0 = ACTION_TYPE_NONE;
                        (*si).interp.num_actions = 0 as xkb_action_count_t;
                    }
                    2 => return 0 != 0,
                    _ => {
                        (*si).interp.num_actions =
                            ((*si).interp.a.action.type_0 as u32 != ACTION_TYPE_NONE as i32 as u32)
                                as i32 as xkb_action_count_t;
                    }
                }
            }
            (*si).defined = ((*si).defined as u32 | SI_FIELD_ACTION as i32 as u32) as si_field;
        } else if istreq(field, b"virtualmodifier\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"virtualmod\0".as_ptr() as *const i8) as i32 != 0
        {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            let mut ndx: xkb_mod_index_t = 0 as xkb_mod_index_t;
            if !ExprResolveMod(
                (*info).ctx,
                value,
                MOD_VIRT,
                &raw mut (*info).mods,
                &raw mut ndx,
            ) {
                return ReportSIBadType(
                    info,
                    si,
                    field,
                    b"virtual modifier\0".as_ptr() as *const i8,
                );
            }
            (*si).interp.virtual_mod = ndx;
            (*si).defined = ((*si).defined as u32 | SI_FIELD_VIRTUAL_MOD as i32 as u32) as si_field;
        } else if istreq(field, b"repeat\0".as_ptr() as *const i8) {
            let mut set: bool = 0 != 0;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set) {
                return ReportSIBadType(info, si, field, b"boolean\0".as_ptr() as *const i8);
            }
            (*si).interp.set_repeat(set as bool);
            (*si).defined = ((*si).defined as u32 | SI_FIELD_AUTO_REPEAT as i32 as u32) as si_field;
        } else if istreq(field, b"locking\0".as_ptr() as *const i8) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"locking\" field in symbol interpretation is unsupported; Ignored\n",
            );
        } else if istreq(field, b"usemodmap\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"usemodmapmods\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut val: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveEnum(
                (*info).ctx,
                value,
                &raw mut val,
                &raw const useModMapValueNames as *const LookupEntry,
            ) {
                return ReportSIBadType(
                    info,
                    si,
                    field,
                    b"level specification\0".as_ptr() as *const i8,
                );
            }
            (*si).interp.level_one_only = val != 0;
            (*si).defined =
                ((*si).defined as u32 | SI_FIELD_LEVEL_ONE_ONLY as i32 as u32) as si_field;
        } else {
            ReportBadField(
                (*info).ctx,
                b"symbol interpretation\0".as_ptr() as *const i8,
                field,
                siText(si, info),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_INTERPRET_FIELDS as i32 as u32
                == 0;
        }
        return 1 != 0;
    }
}
unsafe fn SetLedMapField(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    mut field: *const i8,
    mut arrayNdx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if istreq(field, b"modifiers\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"mods\0".as_ptr() as *const i8) as i32 != 0
        {
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveModMask(
                (*info).ctx,
                value,
                MOD_BOTH,
                &raw mut (*info).mods,
                &raw mut (*ledi).led.mods.mods,
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"modifier mask\0".as_ptr() as *const i8,
                );
            }
            (*ledi).defined = ((*ledi).defined as u32 | LED_FIELD_MODS as i32 as u32) as led_field;
        } else if istreq(field, b"groups\0".as_ptr() as *const i8) {
            let mut mask: xkb_layout_mask_t = 0 as xkb_layout_mask_t;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let mut pending: bool = 0 != 0;
            if !ExprResolveGroupMask((*info).keymap_info, value, &raw mut mask, &raw mut pending) {
                if pending {
                    (*ledi).led.set_pending_groups((1 != 0) as bool);
                    let pending_index: darray_size_t =
                        (*(*(*info).keymap_info).pending_computations).size;
                    darray_append(
                        &mut (*(*(*info).keymap_info).pending_computations).item,
                        &mut (*(*(*info).keymap_info).pending_computations).size,
                        &mut (*(*(*info).keymap_info).pending_computations).alloc,
                        pending_computation {
                            expr: *value_ptr,
                            computed: false,
                            value: 0 as u32,
                        },
                    );
                    *value_ptr = ::core::ptr::null_mut::<ExprDef>();
                    mask = pending_index as xkb_layout_mask_t;
                } else {
                    return ReportLedBadType(
                        info,
                        ledi,
                        field,
                        b"group mask\0".as_ptr() as *const i8,
                    );
                }
            } else {
                (*ledi).led.set_pending_groups((0 != 0) as bool);
            }
            (*ledi).led.groups = mask;
            (*ledi).defined =
                ((*ledi).defined as u32 | LED_FIELD_GROUPS as i32 as u32) as led_field;
        } else if istreq(field, b"controls\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"ctrls\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut mask_0: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let offset: u8 = (*(*info).keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_0,
                (&raw const ctrlMaskNames as *const LookupEntry).offset(offset as i32 as isize),
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"controls mask\0".as_ptr() as *const i8,
                );
            }
            (*ledi).led.ctrls = mask_0 as xkb_action_controls;
            (*ledi).defined = ((*ledi).defined as u32 | LED_FIELD_CTRLS as i32 as u32) as led_field;
        } else if istreq(field, b"allowexplicit\0".as_ptr() as *const i8) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"allowExplicit\" field in indicator statements is unsupported; Ignored\n",
            );
        } else if istreq(field, b"whichmodstate\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"whichmodifierstate\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut mask_1: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_1,
                &raw const modComponentMaskNames as *const LookupEntry,
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"mask of modifier state components\0".as_ptr() as *const i8,
                );
            }
            (*ledi).led.which_mods = mask_1 as xkb_state_component;
        } else if istreq(field, b"whichgroupstate\0".as_ptr() as *const i8) {
            let mut mask_2: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_2,
                &raw const groupComponentMaskNames as *const LookupEntry,
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"mask of group state components\0".as_ptr() as *const i8,
                );
            }
            (*ledi)
                .led
                .set_which_groups(mask_2 as xkb_state_component as xkb_state_component);
        } else if istreq(field, b"driveskbd\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"driveskeyboard\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"leddriveskbd\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"leddriveskeyboard\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"indicatordriveskbd\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"indicatordriveskeyboard\0".as_ptr() as *const i8) as i32 != 0
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"{}\" field in indicator statements is unsupported; Ignored\n",
                crate::xkb::utils::CStrDisplay(field),
            );
        } else if istreq(field, b"index\0".as_ptr() as *const i8) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"index\" field in indicator statements is unsupported; Ignored\n",
            );
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Unknown field \"{}\" in map for {} indicator; Definition ignored\n",
                crate::xkb::utils::CStrDisplay(field),
                crate::xkb::utils::CStrDisplay(LEDText(info, ledi)),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_LED_FIELDS as i32 as u32
                == 0;
        }
        return 1 != 0;
    }
}
unsafe fn HandleGlobalVar(mut info: *mut CompatInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const i8 = ::core::ptr::null::<i8>();
        let mut field: *const i8 = ::core::ptr::null::<i8>();
        let mut ndx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut ret: bool = false;
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut ndx,
        ) {
            ret = 0 != 0;
        } else if !elem.is_null() && istreq(elem, b"interpret\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut temp: SymInterpInfo = SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat_required: [0; 1],
                    num_actions: 0,
                    a: C2Rust_Unnamed_1 {
                        action: xkb_action {
                            type_0: ACTION_TYPE_NONE,
                        },
                    },
                },
            };
            InitInterp(&raw mut temp);
            temp.merge = (if temp.merge as u32 == MERGE_REPLACE as i32 as u32 {
                MERGE_OVERRIDE as i32 as u32
            } else {
                (*stmt).merge as u32
            }) as merge_mode;
            ret = SetInterpField(
                info,
                &raw mut temp,
                field,
                ndx,
                (*stmt).value as *mut ExprDef,
            );
            if ret {
                MergeInterp(info, &raw mut (*info).default_interp, &raw mut temp, 1 != 0);
            }
        } else if !elem.is_null() && istreq(elem, b"indicator\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut temp_0: LedInfo = LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            };
            InitLED(&raw mut temp_0);
            temp_0.merge = (if temp_0.merge as u32 == MERGE_REPLACE as i32 as u32 {
                MERGE_OVERRIDE as i32 as u32
            } else {
                (*stmt).merge as u32
            }) as merge_mode;
            ret = SetLedMapField(info, &raw mut temp_0, field, ndx, &raw mut (*stmt).value);
            if ret {
                MergeLedMap(info, &raw mut (*info).default_led, &raw mut temp_0, 1 != 0);
            }
        } else if !elem.is_null() {
            ret = SetDefaultActionField(
                (*info).keymap_info,
                &raw mut (*info).default_actions,
                &raw mut (*info).mods,
                elem,
                field,
                ndx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as u32
                != PARSER_FATAL_ERROR as i32 as u32;
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS as i32 as u32
                == 0;
        }
        return ret;
    }
}
unsafe fn HandleInterpBody(
    mut info: *mut CompatInfo,
    mut def: *mut VarDef,
    mut si: *mut SymInterpInfo,
) -> bool {
    unsafe {
        let mut ok: bool = 1 != 0;
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
                ok = 0 != 0;
            } else if !elem.is_null() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Cannot set a global default value for \"{}\" element from within an interpret statement; Move assignment to \"{}.{}\" to the global file scope\n",
                    crate::xkb::utils::CStrDisplay(elem),
                    crate::xkb::utils::CStrDisplay(elem),
                    crate::xkb::utils::CStrDisplay(field),
                );
                ok = 0 != 0;
            } else if !SetInterpField(info, si, field, arrayNdx, (*def).value as *mut ExprDef) {
                ok = 0 != 0;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return ok;
    }
}
unsafe fn HandleInterpDef(mut info: *mut CompatInfo, mut def: *mut InterpDef) -> bool {
    unsafe {
        let mut pred: xkb_match_operation = MATCH_NONE;
        let mut mods: xkb_mod_mask_t = 0;
        let mut si: SymInterpInfo = SymInterpInfo {
            defined: 0 as si_field,
            merge: MERGE_DEFAULT,
            interp: xkb_sym_interpret {
                sym: 0,
                match_0: MATCH_NONE,
                mods: 0,
                virtual_mod: 0,
                level_one_only: false,
                repeat_required: [0; 1],
                num_actions: 0,
                a: C2Rust_Unnamed_1 {
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                },
            },
        };
        if !ResolveStateAndPredicate(
            (*def).match_0 as *mut ExprDef,
            &raw mut pred,
            &raw mut mods,
            info,
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Couldn't determine matching modifiers; Symbol interpretation ignored\n",
            );
            return 0 != 0;
        }
        si = (*info).default_interp;
        si.merge = (*def).merge;
        si.interp.sym = (*def).sym;
        si.interp.match_0 = pred;
        si.interp.mods = mods;
        if !HandleInterpBody(info, (*def).def, &raw mut si) {
            (*info).errorCount += 1;
            return 0 != 0;
        }
        if !AddInterp(info, &raw mut si, 1 != 0) {
            (*info).errorCount += 1;
            return 0 != 0;
        }
        return 1 != 0;
    }
}
unsafe fn HandleLedMapDef(mut info: *mut CompatInfo, mut def: *mut LedMapDef) -> bool {
    unsafe {
        let mut ledi: LedInfo = (*info).default_led;
        ledi.merge = (*def).merge;
        ledi.led.name = (*def).name;
        let mut ok: bool = 1 != 0;
        let mut var: *mut VarDef = (*def).body;
        while !var.is_null() {
            let mut elem: *const i8 = ::core::ptr::null::<i8>();
            let mut field: *const i8 = ::core::ptr::null::<i8>();
            let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            if !ExprResolveLhs(
                (*info).ctx,
                (*var).name,
                &raw mut elem,
                &raw mut field,
                &raw mut arrayNdx,
            ) {
                ok = 0 != 0;
            } else if !elem.is_null() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Cannot set defaults for \"{}\" element in indicator map; Assignment to {}.{} ignored\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    crate::xkb::utils::CStrDisplay(elem),
                    crate::xkb::utils::CStrDisplay(elem),
                    crate::xkb::utils::CStrDisplay(field),
                );
                ok = 0 != 0;
            } else if !SetLedMapField(info, &raw mut ledi, field, arrayNdx, &raw mut (*var).value) {
                ok = 0 != 0;
            }
            var = (*var).common.next as *mut VarDef;
        }
        return ok as i32 != 0 && AddLedMap(info, &raw mut ledi, 1 != 0) as i32 != 0;
    }
}
unsafe fn HandleCompatMapFile(mut info: *mut CompatInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        free((*info).name as *mut ::core::ffi::c_void);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
                1 => {
                    ok = HandleIncludeCompatMap(info, stmt as *mut IncludeStmt);
                }
                28 => {
                    ok = HandleInterpDef(info, stmt as *mut InterpDef);
                }
                32 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_DEBUG,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "The \"group\" statement in compat is unsupported; Ignored\n",
                    );
                    ok = 1 != 0;
                }
                33 => {
                    ok = HandleLedMapDef(info, stmt as *mut LedMapDef);
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
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported compatibility {} statement \"{}\"; Ignoring\n",
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
                        & PARSER_NO_UNKNOWN_STATEMENTS as i32 as u32
                        == 0;
                }
                _ => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Compat files may not include other types; Ignoring {}\n",
                        crate::xkb::utils::CStrDisplay(stmt_type_to_string((*stmt).type_0)),
                    );
                    ok = 0 != 0;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10 as i32 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Abandoning compatibility map \"{}\"\n",
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn CopyInterps(
    mut info: *mut CompatInfo,
    mut needSymbol: bool,
    mut pred: xkb_match_operation,
    mut collect: *mut collect,
) {
    unsafe {
        let mut si: *mut SymInterpInfo = ::core::ptr::null_mut::<SymInterpInfo>();
        if !(*info).interps.item.is_null() {
            si = (*info).interps.item.offset(0 as i32 as isize) as *mut SymInterpInfo;
            while si
                < (*info).interps.item.offset((*info).interps.size as isize) as *mut SymInterpInfo
            {
                if (*si).interp.match_0 as u32 == pred as u32
                    && ((*si).interp.sym != XKB_KEY_NoSymbol as xkb_keysym_t) as i32
                        == needSymbol as i32
                {
                    darray_append(
                        &mut (*collect).sym_interprets.item,
                        &mut (*collect).sym_interprets.size,
                        &mut (*collect).sym_interprets.alloc,
                        (*si).interp,
                    );
                }
                si = si.offset(1);
            }
        }
    }
}
unsafe fn CopyLedMapDefsToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut CompatInfo) {
    unsafe {
        let mut c2rust_current_block_11: u64;
        let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
        while idx < (*info).num_leds {
            let mut ledi: *mut LedInfo =
                (&raw mut (*info).leds as *mut LedInfo).offset(idx as isize) as *mut LedInfo;
            let mut i: xkb_led_index_t = 0;
            let mut led: *mut xkb_led = ::core::ptr::null_mut::<xkb_led>();
            i = 0 as xkb_led_index_t;
            led = &raw mut (*keymap).leds as *mut xkb_led;
            while i < (*keymap).num_leds {
                if (*led).name == (*ledi).led.name {
                    break;
                }
                i = i.wrapping_add(1);
                led = led.offset(1);
            }
            if i >= (*keymap).num_leds {
                xkb_logf!(
                    (*keymap).ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Indicator name \"{}\" was not declared in the keycodes section; Adding new indicator\n",
                    crate::xkb::utils::CStrDisplay(LEDText(info, ledi)),
                );
                i = 0 as xkb_led_index_t;
                led = &raw mut (*keymap).leds as *mut xkb_led;
                while i < (*keymap).num_leds {
                    if (*led).name == XKB_ATOM_NONE as xkb_atom_t {
                        break;
                    }
                    i = i.wrapping_add(1);
                    led = led.offset(1);
                }
                if i >= (*keymap).num_leds {
                    if i >= XKB_MAX_LEDS {
                        xkb_logf!(
                            (*keymap).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "Too many indicators (maximum is {}); Indicator name \"{}\" ignored\n",
                            (::core::mem::size_of::<xkb_led_mask_t>() as usize)
                                .wrapping_mul(8 as usize)
                                as xkb_led_index_t,
                            crate::xkb::utils::CStrDisplay(LEDText(info, ledi)),
                        );
                        c2rust_current_block_11 = 792017965103506125;
                    } else {
                        let c2rust_fresh0 = (*keymap).num_leds;
                        (*keymap).num_leds = (*keymap).num_leds.wrapping_add(1);
                        led = (&raw mut (*keymap).leds as *mut xkb_led)
                            .offset(c2rust_fresh0 as isize)
                            as *mut xkb_led;
                        c2rust_current_block_11 = 17860125682698302841;
                    }
                } else {
                    c2rust_current_block_11 = 17860125682698302841;
                }
            } else {
                c2rust_current_block_11 = 17860125682698302841;
            }
            match c2rust_current_block_11 {
                17860125682698302841 => {
                    *led = (*ledi).led;
                    if (*led).which_groups() as i32 == 0 as i32
                        && ((*led).groups != 0 as xkb_layout_mask_t
                            || (*led).pending_groups() as i32 != 0)
                    {
                        (*led).set_which_groups(XKB_STATE_LAYOUT_EFFECTIVE as xkb_state_component);
                    }
                    if (*led).which_mods as u32 == 0 as u32
                        && (*led).mods.mods != 0 as xkb_mod_mask_t
                    {
                        (*led).which_mods = XKB_STATE_MODS_EFFECTIVE;
                    }
                }
                _ => {}
            }
            idx = idx.wrapping_add(1);
        }
    }
}
unsafe fn CopyCompatToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut CompatInfo) -> bool {
    unsafe {
        (*keymap).compat_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).compat_section_name);
        (*keymap).mods = (*info).mods;
        if !((*info).interps.size == 0 as darray_size_t) {
            let mut collect: collect = collect {
                sym_interprets: C2Rust_Unnamed_19 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_sym_interpret>(),
                },
            };
            collect.sym_interprets.item = ::core::ptr::null_mut::<xkb_sym_interpret>();
            collect.sym_interprets.size = 0 as darray_size_t;
            collect.sym_interprets.alloc = 0 as darray_size_t;
            CopyInterps(info, 1 != 0, MATCH_EXACTLY, &raw mut collect);
            CopyInterps(info, 1 != 0, MATCH_ALL, &raw mut collect);
            CopyInterps(info, 1 != 0, MATCH_NONE, &raw mut collect);
            CopyInterps(info, 1 != 0, MATCH_ANY, &raw mut collect);
            CopyInterps(info, 1 != 0, MATCH_ANY_OR_NONE, &raw mut collect);
            CopyInterps(info, 0 != 0, MATCH_EXACTLY, &raw mut collect);
            CopyInterps(info, 0 != 0, MATCH_ALL, &raw mut collect);
            CopyInterps(info, 0 != 0, MATCH_NONE, &raw mut collect);
            CopyInterps(info, 0 != 0, MATCH_ANY, &raw mut collect);
            CopyInterps(info, 0 != 0, MATCH_ANY_OR_NONE, &raw mut collect);
            (*keymap).sym_interprets = collect.sym_interprets.item;
            if !(&raw mut (*keymap).num_sym_interprets).is_null() {
                *&raw mut (*keymap).num_sym_interprets = collect.sym_interprets.size;
            }
            collect.sym_interprets.item = ::core::ptr::null_mut::<xkb_sym_interpret>();
            collect.sym_interprets.size = 0 as darray_size_t;
            collect.sym_interprets.alloc = 0 as darray_size_t;
        }
        CopyLedMapDefsToKeymap(keymap, info);
        return 1 != 0;
    }
}
pub unsafe fn CompileCompatMap(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: CompatInfo = CompatInfo {
            name: ::core::ptr::null_mut::<i8>(),
            errorCount: 0,
            include_depth: 0,
            default_interp: SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat_required: [0; 1],
                    num_actions: 0,
                    a: C2Rust_Unnamed_1 {
                        action: xkb_action {
                            type_0: ACTION_TYPE_NONE,
                        },
                    },
                },
            },
            interps: C2Rust_Unnamed_18 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<SymInterpInfo>(),
            },
            default_led: LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            },
            leds: [LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            }; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
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
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            ctx: ::core::ptr::null_mut::<xkb_context>(),
        };
        InitCompatInfo(
            &raw mut info,
            keymap_info,
            0 as u32,
            &raw mut (*keymap_info).keymap.mods,
        );
        if !file.is_null() {
            HandleCompatMapFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopyCompatToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearCompatInfo(&raw mut info);
                return 1 != 0;
            }
        }
        ClearCompatInfo(&raw mut info);
        return 0 != 0;
    }
}
