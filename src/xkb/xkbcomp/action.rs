use crate::xkb_logf;
pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
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
        XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
        XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
        XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_overlay_index_t = u8;
    #[inline]
    pub unsafe fn XkbKeyByName(
        mut keymap: *const xkb_keymap,
        mut name: xkb_atom_t,
        mut use_aliases: bool,
    ) -> *mut xkb_key {
        unsafe {
            if name < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
                let match_0: KeycodeMatch = *(*keymap)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .key_names
                    .offset(name as isize);
                if match_0.c2rust_unnamed.found() {
                    if !match_0.c2rust_unnamed.is_alias() {
                        return (*keymap).keys.offset(match_0.key.index() as isize) as *mut xkb_key;
                    } else if use_aliases {
                        return (*keymap).keys.offset(
                            (*(*keymap)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .key_names
                                .offset(match_0.alias.real() as isize))
                            .key
                            .index() as isize,
                        ) as *mut xkb_key;
                    }
                }
            }
            return ::core::ptr::null_mut::<xkb_key>();
        }
    }
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
    pub type C2Rust_Unnamed_13 = DarrayKeysym;
    pub use crate::xkb::xkbcomp::ast_build::stmt_type_to_string;
}
pub mod text_h {

    pub use crate::xkb::text::{
        actionTypeNames, ctrlMaskNames, ActionTypeText, KeyNameText, LookupEntry, LookupString,
        LookupValue,
    };
}
pub mod xkbcomp_priv_h {
    pub use crate::xkb::shared_ast_types::{
        pending_computation, pending_computation_array, xkb_keymap_info, xkb_parser_error,
        xkb_parser_strict_flags, XkbcompFeatures, XkbcompLookup, PARSER_FATAL_ERROR,
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
}
pub mod action_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ActionsInfo {
        pub actions: [xkb_action; 21],
    }
    use super::keymap_h::xkb_action;
}
pub use crate::xkb::xkbcomp::expr::{
    ExprResolveBoolean, ExprResolveButton, ExprResolveEnum, ExprResolveGroup, ExprResolveInteger,
    ExprResolveLhs, ExprResolveMask, ExprResolveModMask, ExprResolveString,
};
pub mod utils_h {
    #[inline]
    pub unsafe fn istreq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as i32;
        }
    }
    pub use crate::xkb::utils::istrcmp;
}

pub use self::action_h::ActionsInfo;
pub use self::ast_h::{
    _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, C2Rust_Unnamed_13, ExprAction,
    ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef, ExprIdent,
    ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary, ParseCommon,
    _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
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
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{xkb_atom_text, xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
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
    C2Rust_Unnamed_9, KeycodeMatch, XkbKeyByName, _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
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
pub use self::text_h::{
    actionTypeNames, ctrlMaskNames, ActionTypeText, KeyNameText, LookupEntry, LookupString,
    LookupValue,
};
pub use self::utils_h::{istrcmp, istreq};
pub use self::xkbcommon_h::{
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t, xkb_layout_index_t,
    xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t,
    xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, xkb_keymap_info, xkb_parser_error,
    xkb_parser_strict_flags, C2Rust_Unnamed_14, C2Rust_Unnamed_15, PARSER_FATAL_ERROR,
    PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS,
    PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
    PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
    PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use crate::xkb::keymap_priv::action_equal;
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::utils::cstr_len;
use crate::xkb::utils::darray_append;
pub type action_field = u32;
pub const ACTION_FIELD_LATCH_ON_PRESS: action_field = 25;
pub const ACTION_FIELD_UNLOCK_ON_PRESS: action_field = 24;
pub const ACTION_FIELD_LOCK_ON_RELEASE: action_field = 23;
pub const ACTION_FIELD_MODS_TO_CLEAR: action_field = 22;
pub const ACTION_FIELD_KEYCODE: action_field = 21;
pub const ACTION_FIELD_DEVICE: action_field = 20;
pub const ACTION_FIELD_DATA: action_field = 19;
pub const ACTION_FIELD_SAME: action_field = 18;
pub const ACTION_FIELD_SCREEN: action_field = 17;
pub const ACTION_FIELD_COUNT: action_field = 16;
pub const ACTION_FIELD_TYPE: action_field = 15;
pub const ACTION_FIELD_CONTROLS: action_field = 14;
pub const ACTION_FIELD_VALUE: action_field = 13;
pub const ACTION_FIELD_BUTTON: action_field = 12;
pub const ACTION_FIELD_ACCEL: action_field = 11;
pub const ACTION_FIELD_Y: action_field = 10;
pub const ACTION_FIELD_X: action_field = 9;
pub const ACTION_FIELD_GROUP: action_field = 8;
pub const ACTION_FIELD_MODIFIERS: action_field = 7;
pub const ACTION_FIELD_INCREMENT: action_field = 6;
pub const ACTION_FIELD_AFFECT: action_field = 5;
pub const ACTION_FIELD_DEFAULT: action_field = 4;
pub const ACTION_FIELD_REPORT: action_field = 3;
pub const ACTION_FIELD_GEN_KEY_EVENT: action_field = 2;
pub const ACTION_FIELD_LATCH_TO_LOCK: action_field = 1;
pub const ACTION_FIELD_CLEAR_LOCKS: action_field = 0;
pub type actionHandler = Option<
    unsafe fn(
        *const xkb_keymap_info,
        *const xkb_mod_set,
        *mut xkb_action,
        action_field,
        *const ExprDef,
        *const ExprDef,
        *mut *mut ExprDef,
    ) -> xkb_parser_error,
>;
static mut constTrue: ExprBoolean = ExprBoolean {
    common: _ParseCommon {
        next: ::core::ptr::null::<_ParseCommon>() as *mut _ParseCommon,
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: 1 != 0,
};
static mut constFalse: ExprBoolean = ExprBoolean {
    common: _ParseCommon {
        next: ::core::ptr::null::<_ParseCommon>() as *mut _ParseCommon,
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: 0 != 0,
};
pub unsafe fn InitActionsInfo(mut keymap: *const xkb_keymap, mut info: *mut ActionsInfo) {
    unsafe {
        let mut type_0: xkb_action_type = ACTION_TYPE_NONE;
        while (type_0 as u32) < _ACTION_TYPE_NUM_ENTRIES as i32 as u32 {
            (*info).actions[type_0 as usize].type_0 = type_0;
            type_0 += 1;
        }
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as i32 as usize]
            .dflt
            .flags = 0 as xkb_action_flags;
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as i32 as usize]
            .dflt
            .value = 1 as i8;
        (*info).actions[ACTION_TYPE_PTR_MOVE as i32 as usize]
            .ptr
            .flags = ACTION_ACCEL;
        (*info).actions[ACTION_TYPE_SWITCH_VT as i32 as usize]
            .screen
            .flags = ACTION_SAME_SCREEN;
        (*info).actions[ACTION_TYPE_REDIRECT_KEY as i32 as usize]
            .redirect
            .keycode = (*keymap).redirect_key_auto;
    }
}
static mut fieldStrings: [LookupEntry; 37] = [
    LookupEntry {
        name: b"clearLocks\0".as_ptr() as *const i8,
        value: ACTION_FIELD_CLEAR_LOCKS as i32 as u32,
    },
    LookupEntry {
        name: b"latchToLock\0".as_ptr() as *const i8,
        value: ACTION_FIELD_LATCH_TO_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"genKeyEvent\0".as_ptr() as *const i8,
        value: ACTION_FIELD_GEN_KEY_EVENT as i32 as u32,
    },
    LookupEntry {
        name: b"generateKeyEvent\0".as_ptr() as *const i8,
        value: ACTION_FIELD_GEN_KEY_EVENT as i32 as u32,
    },
    LookupEntry {
        name: b"report\0".as_ptr() as *const i8,
        value: ACTION_FIELD_REPORT as i32 as u32,
    },
    LookupEntry {
        name: b"default\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DEFAULT as i32 as u32,
    },
    LookupEntry {
        name: b"affect\0".as_ptr() as *const i8,
        value: ACTION_FIELD_AFFECT as i32 as u32,
    },
    LookupEntry {
        name: b"increment\0".as_ptr() as *const i8,
        value: ACTION_FIELD_INCREMENT as i32 as u32,
    },
    LookupEntry {
        name: b"modifiers\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODIFIERS as i32 as u32,
    },
    LookupEntry {
        name: b"mods\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODIFIERS as i32 as u32,
    },
    LookupEntry {
        name: b"group\0".as_ptr() as *const i8,
        value: ACTION_FIELD_GROUP as i32 as u32,
    },
    LookupEntry {
        name: b"x\0".as_ptr() as *const i8,
        value: ACTION_FIELD_X as i32 as u32,
    },
    LookupEntry {
        name: b"y\0".as_ptr() as *const i8,
        value: ACTION_FIELD_Y as i32 as u32,
    },
    LookupEntry {
        name: b"accel\0".as_ptr() as *const i8,
        value: ACTION_FIELD_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"accelerate\0".as_ptr() as *const i8,
        value: ACTION_FIELD_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"repeat\0".as_ptr() as *const i8,
        value: ACTION_FIELD_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"button\0".as_ptr() as *const i8,
        value: ACTION_FIELD_BUTTON as i32 as u32,
    },
    LookupEntry {
        name: b"value\0".as_ptr() as *const i8,
        value: ACTION_FIELD_VALUE as i32 as u32,
    },
    LookupEntry {
        name: b"controls\0".as_ptr() as *const i8,
        value: ACTION_FIELD_CONTROLS as i32 as u32,
    },
    LookupEntry {
        name: b"ctrls\0".as_ptr() as *const i8,
        value: ACTION_FIELD_CONTROLS as i32 as u32,
    },
    LookupEntry {
        name: b"type\0".as_ptr() as *const i8,
        value: ACTION_FIELD_TYPE as i32 as u32,
    },
    LookupEntry {
        name: b"count\0".as_ptr() as *const i8,
        value: ACTION_FIELD_COUNT as i32 as u32,
    },
    LookupEntry {
        name: b"screen\0".as_ptr() as *const i8,
        value: ACTION_FIELD_SCREEN as i32 as u32,
    },
    LookupEntry {
        name: b"same\0".as_ptr() as *const i8,
        value: ACTION_FIELD_SAME as i32 as u32,
    },
    LookupEntry {
        name: b"sameServer\0".as_ptr() as *const i8,
        value: ACTION_FIELD_SAME as i32 as u32,
    },
    LookupEntry {
        name: b"data\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DATA as i32 as u32,
    },
    LookupEntry {
        name: b"device\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DEVICE as i32 as u32,
    },
    LookupEntry {
        name: b"dev\0".as_ptr() as *const i8,
        value: ACTION_FIELD_DEVICE as i32 as u32,
    },
    LookupEntry {
        name: b"key\0".as_ptr() as *const i8,
        value: ACTION_FIELD_KEYCODE as i32 as u32,
    },
    LookupEntry {
        name: b"keycode\0".as_ptr() as *const i8,
        value: ACTION_FIELD_KEYCODE as i32 as u32,
    },
    LookupEntry {
        name: b"kc\0".as_ptr() as *const i8,
        value: ACTION_FIELD_KEYCODE as i32 as u32,
    },
    LookupEntry {
        name: b"clearmods\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODS_TO_CLEAR as i32 as u32,
    },
    LookupEntry {
        name: b"clearmodifiers\0".as_ptr() as *const i8,
        value: ACTION_FIELD_MODS_TO_CLEAR as i32 as u32,
    },
    LookupEntry {
        name: b"lockOnRelease\0".as_ptr() as *const i8,
        value: ACTION_FIELD_LOCK_ON_RELEASE as i32 as u32,
    },
    LookupEntry {
        name: b"unlockOnPress\0".as_ptr() as *const i8,
        value: ACTION_FIELD_UNLOCK_ON_PRESS as i32 as u32,
    },
    LookupEntry {
        name: b"latchOnPress\0".as_ptr() as *const i8,
        value: ACTION_FIELD_LATCH_ON_PRESS as i32 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe fn stringToActionType(mut str: *const i8, mut type_rtrn: *mut xkb_action_type) -> bool {
    unsafe {
        let mut type_0: u32 = 0 as u32;
        let ret: bool = LookupString(
            &raw const actionTypeNames as *const LookupEntry,
            str,
            &raw mut type_0,
        ) as bool;
        *type_rtrn = type_0 as xkb_action_type;
        return ret;
    }
}
unsafe fn stringToField(mut str: *const i8, mut field_rtrn: *mut action_field) -> bool {
    unsafe {
        let mut field: u32 = 0 as u32;
        let ret: bool = LookupString(
            &raw const fieldStrings as *const LookupEntry,
            str,
            &raw mut field,
        ) as bool;
        *field_rtrn = field as action_field;
        return ret;
    }
}
unsafe fn fieldText(mut field: action_field) -> *const i8 {
    unsafe {
        return LookupValue(&raw const fieldStrings as *const LookupEntry, field as u32);
    }
}
#[inline]
unsafe fn ReportMismatch(
    mut ctx: *mut xkb_context,
    mut code: xkb_message_code,
    mut action: xkb_action_type,
    mut field: action_field,
    mut type_0: *const i8,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Value of {} field must be of type {}; Action {} definition ignored\n",
            code as u32,
            crate::xkb::utils::CStrDisplay(fieldText(field)),
            crate::xkb::utils::CStrDisplay(type_0),
            crate::xkb::utils::CStrDisplay(ActionTypeText(action)),
        );
        return (if strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe fn ReportFormatVersionMismatch(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: action_field,
    mut format: xkb_keymap_format,
    mut versions: *const i8,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Field {} for an action of type {} requires keymap text format {},  but got: {}; Action definition ignored\n",
            XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT as i32,
            crate::xkb::utils::CStrDisplay(fieldText(field)),
            crate::xkb::utils::CStrDisplay(ActionTypeText(action)),
            crate::xkb::utils::CStrDisplay(versions),
            format as u32,
        );
        return (if strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe fn ReportIllegal(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: action_field,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Field {} is not defined for an action of type {}; Action definition ignored\n",
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            crate::xkb::utils::CStrDisplay(fieldText(field)),
            crate::xkb::utils::CStrDisplay(ActionTypeText(action)),
        );
        return (if strict as u32 & PARSER_NO_ILLEGAL_ACTION_FIELDS as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe fn ReportActionNotArray(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: action_field,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] The {} field in the {} action is not an array; Action definition ignored\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            crate::xkb::utils::CStrDisplay(fieldText(field)),
            crate::xkb::utils::CStrDisplay(ActionTypeText(action)),
        );
        return (if strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error;
    }
}
unsafe fn HandleNoAction(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            (*keymap_info).keymap.ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] The \"{}\" action takes no argument, but got \"{}\" field; Action definition ignored\n",
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            crate::xkb::utils::CStrDisplay(ActionTypeText((*action).type_0)),
            crate::xkb::utils::CStrDisplay(fieldText(field)),
        );
        return (if (*keymap_info).strict as u32 & PARSER_NO_ILLEGAL_ACTION_FIELDS as i32 as u32 != 0
        {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
unsafe fn CheckBooleanFlag(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut action: xkb_action_type,
    mut field: action_field,
    mut flag: xkb_action_flags,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        let mut set: bool = 0 != 0;
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, field, strict);
        }
        if !ExprResolveBoolean(ctx, value, &raw mut set) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                field,
                b"boolean\0".as_ptr() as *const i8,
                strict,
            );
        }
        if set {
            *flags_inout = (*flags_inout as u32 | flag as u32) as xkb_action_flags;
        } else {
            *flags_inout = (*flags_inout as u32 & !(flag as u32)) as xkb_action_flags;
        }
        return PARSER_SUCCESS;
    }
}
unsafe fn CheckModifierField(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut mods: *const xkb_mod_set,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
    mut mods_rtrn: *mut xkb_mod_mask_t,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_MODIFIERS, strict);
        }
        if (*value).common.type_0 as u32 == STMT_EXPR_IDENT as i32 as u32 {
            let mut valStr: *const i8 = ::core::ptr::null::<i8>();
            valStr = xkb_atom_text(ctx, (*value).ident.ident);
            if !valStr.is_null()
                && (istreq(valStr, b"usemodmapmods\0".as_ptr() as *const i8) as i32 != 0
                    || istreq(valStr, b"modmapmods\0".as_ptr() as *const i8) as i32 != 0)
            {
                *mods_rtrn = 0 as xkb_mod_mask_t;
                *flags_inout = (*flags_inout as u32 | ACTION_MODS_LOOKUP_MODMAP as i32 as u32)
                    as xkb_action_flags;
                return PARSER_SUCCESS;
            }
        }
        if !ExprResolveModMask(ctx, value, MOD_BOTH, mods, mods_rtrn) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_MODIFIERS,
                b"modifier mask\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout =
            (*flags_inout as u32 & !(ACTION_MODS_LOOKUP_MODMAP as i32) as u32) as xkb_action_flags;
        return PARSER_SUCCESS;
    }
}
static mut lockWhich: [LookupEntry; 5] = [
    LookupEntry {
        name: b"both\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: b"lock\0".as_ptr() as *const i8,
        value: ACTION_LOCK_NO_UNLOCK as i32 as u32,
    },
    LookupEntry {
        name: b"neither\0".as_ptr() as *const i8,
        value: (ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32,
    },
    LookupEntry {
        name: b"unlock\0".as_ptr() as *const i8,
        value: ACTION_LOCK_NO_LOCK as i32 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe fn CheckAffectField(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_AFFECT, strict);
        }
        let mut flags: u32 = 0 as u32;
        if !ExprResolveEnum(
            ctx,
            value,
            &raw mut flags,
            &raw const lockWhich as *const LookupEntry,
        ) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_AFFECT,
                b"lock, unlock, both, neither\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout = (*flags_inout as u32
            & !(ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32)
            as xkb_action_flags;
        *flags_inout = (*flags_inout as u32 | flags as xkb_action_flags as u32) as xkb_action_flags;
        return PARSER_SUCCESS;
    }
}
unsafe fn HandleSetLatchLockMods(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_mod_action = &raw mut (*action).mods;
        let type_0: xkb_action_type = (*action).type_0;
        if field as u32 == ACTION_FIELD_MODIFIERS as i32 as u32 {
            return CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
                &raw mut (*act).mods.mods,
            );
        }
        if field as u32 == ACTION_FIELD_UNLOCK_ON_PRESS as i32 as u32 {
            if (*keymap_info).features.mods_unlock_on_press {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_UNLOCK_ON_PRESS,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    ctx,
                    (*action).type_0,
                    field,
                    (*keymap_info).keymap.format,
                    b">= 2\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
        }
        if (type_0 as u32 == ACTION_TYPE_MOD_SET as i32 as u32
            || type_0 as u32 == ACTION_TYPE_MOD_LATCH as i32 as u32)
            && field as u32 == ACTION_FIELD_CLEAR_LOCKS as i32 as u32
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LOCK_CLEAR,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_MOD_LATCH as i32 as u32 {
            if field as u32 == ACTION_FIELD_LATCH_TO_LOCK as i32 as u32 {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_LATCH_TO_LOCK,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            }
            if field as u32 == ACTION_FIELD_LATCH_ON_PRESS as i32 as u32 {
                if (*keymap_info).features.mods_latch_on_press {
                    return CheckBooleanFlag(
                        ctx,
                        (*keymap_info).strict,
                        (*action).type_0,
                        field,
                        ACTION_LATCH_ON_PRESS,
                        array_ndx,
                        value,
                        &raw mut (*act).flags,
                    );
                } else {
                    return ReportFormatVersionMismatch(
                        ctx,
                        (*action).type_0,
                        field,
                        (*keymap_info).keymap.format,
                        b">= 2\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
            }
        }
        if type_0 as u32 == ACTION_TYPE_MOD_LOCK as i32 as u32
            && field as u32 == ACTION_FIELD_AFFECT as i32 as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn CheckGroupField(
    mut keymap_info: *const xkb_keymap_info,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    mut flags_inout: *mut xkb_action_flags,
    mut group_rtrn: *mut i32,
) -> xkb_parser_error {
    unsafe {
        let mut spec: *const ExprDef = ::core::ptr::null::<ExprDef>();
        let mut idx: xkb_layout_index_t = 0 as xkb_layout_index_t;
        let mut flags: xkb_action_flags = *flags_inout;
        if !array_ndx.is_null() {
            return ReportActionNotArray(
                (*keymap_info).keymap.ctx,
                action,
                ACTION_FIELD_GROUP,
                (*keymap_info).strict,
            );
        }
        if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32
            || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as i32 as u32
        {
            flags = (flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            spec = (*value).unary.child;
            value_ptr = &raw mut (**value_ptr).unary.child as *mut *mut ExprDef;
        } else {
            flags = (flags as u32 | ACTION_ABSOLUTE_SWITCH as i32 as u32) as xkb_action_flags;
            spec = value;
        }
        let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 != 0;
        let mut pending: bool = 0 != 0;
        let ret: xkb_parser_error =
            ExprResolveGroup(keymap_info, spec, absolute, &raw mut idx, &raw mut pending)
                as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS as i32 as u32 && !pending {
            ReportMismatch(
                (*keymap_info).keymap.ctx,
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
                action,
                ACTION_FIELD_GROUP,
                b"integer\0".as_ptr() as *const i8,
                (*keymap_info).strict,
            );
            return ret;
        }
        if pending {
            flags = (flags as u32 | ACTION_PENDING_COMPUTATION as i32 as u32) as xkb_action_flags;
            let pending_index: darray_size_t = (*(*keymap_info).pending_computations).size;
            darray_append(
                &mut (*(*keymap_info).pending_computations).item,
                &mut (*(*keymap_info).pending_computations).size,
                &mut (*(*keymap_info).pending_computations).alloc,
                pending_computation {
                    expr: *value_ptr,
                    computed: false,
                    value: 0 as u32,
                },
            );
            *value_ptr = ::core::ptr::null_mut::<ExprDef>();
            *group_rtrn = pending_index as i32;
        } else {
            flags =
                (flags as u32 & !(ACTION_PENDING_COMPUTATION as i32) as u32) as xkb_action_flags;
            if flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 == 0 {
                *group_rtrn = idx as i32;
                if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                    *group_rtrn = -*group_rtrn;
                }
            } else {
                *group_rtrn = idx.wrapping_sub(1 as xkb_layout_index_t) as i32;
            }
        }
        *flags_inout = flags;
        return PARSER_SUCCESS;
    }
}
unsafe fn HandleSetLatchLockGroup(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_group_action = &raw mut (*action).group;
        let type_0: xkb_action_type = (*action).type_0;
        if field as u32 == ACTION_FIELD_GROUP as i32 as u32 {
            return CheckGroupField(
                keymap_info,
                (*action).type_0,
                array_ndx,
                value,
                value_ptr,
                &raw mut (*act).flags,
                &raw mut (*act).group,
            );
        }
        if (type_0 as u32 == ACTION_TYPE_GROUP_SET as i32 as u32
            || type_0 as u32 == ACTION_TYPE_GROUP_LATCH as i32 as u32)
            && field as u32 == ACTION_FIELD_CLEAR_LOCKS as i32 as u32
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LOCK_CLEAR,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_GROUP_LATCH as i32 as u32
            && field as u32 == ACTION_FIELD_LATCH_TO_LOCK as i32 as u32
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LATCH_TO_LOCK,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_GROUP_LOCK as i32 as u32
            && field as u32 == ACTION_FIELD_LOCK_ON_RELEASE as i32 as u32
        {
            if (*keymap_info).features.group_lock_on_release {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_LOCK_ON_RELEASE,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    ctx,
                    (*action).type_0,
                    field,
                    (*keymap_info).keymap.format,
                    b">= v2\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleMovePtr(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_pointer_action = &raw mut (*action).ptr;
        if field as u32 == ACTION_FIELD_X as i32 as u32
            || field as u32 == ACTION_FIELD_Y as i32 as u32
        {
            let mut val: i64 = 0 as i64;
            let absolute: bool = (*value).common.type_0 as u32 != STMT_EXPR_NEGATE as i32 as u32
                && (*value).common.type_0 as u32 != STMT_EXPR_UNARY_PLUS as i32 as u32;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if val < i16::MIN as i64 || val > i16::MAX as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "The {} field in the {} action must be in range {}..{}, but got {}. Action definition ignored\n",
                    crate::xkb::utils::CStrDisplay(fieldText(field)),
                    crate::xkb::utils::CStrDisplay(ActionTypeText((*action).type_0)),
                    -32767 as i32 - 1 as i32,
                    32767 as i32,
                    val,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if field as u32 == ACTION_FIELD_X as i32 as u32 {
                if absolute {
                    (*act).flags =
                        ((*act).flags as u32 | ACTION_ABSOLUTE_X as i32 as u32) as xkb_action_flags;
                }
                (*act).x = val as i16;
            } else {
                if absolute {
                    (*act).flags =
                        ((*act).flags as u32 | ACTION_ABSOLUTE_Y as i32 as u32) as xkb_action_flags;
                }
                (*act).y = val as i16;
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_ACCEL as i32 as u32 {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_ACCEL,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandlePtrBtn(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_pointer_button_action = &raw mut (*action).btn;
        if field as u32 == ACTION_FIELD_BUTTON as i32 as u32 {
            let mut btn: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveButton(ctx, value, &raw mut btn) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (range 1..5)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if btn < 0 as i64 || btn > 5 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Button must specify default or be in the range 1..5; Illegal button value {} ignored\n",
                    btn,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).button = btn as u8;
            return PARSER_SUCCESS;
        } else if (*action).type_0 as u32 == ACTION_TYPE_PTR_LOCK as i32 as u32
            && field as u32 == ACTION_FIELD_AFFECT as i32 as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        } else if field as u32 == ACTION_FIELD_COUNT as i32 as u32 {
            let mut val: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if val < 0 as i64 || val > 255 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "The count field must have a value in the range 0..255; Illegal count {} ignored\n",
                    val,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).count = val as u8;
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
static mut ptrDflts: [LookupEntry; 4] = [
    LookupEntry {
        name: b"dfltbtn\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"defaultbutton\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"button\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
unsafe fn HandleSetPtrDflt(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_pointer_default_action = &raw mut (*action).dflt;
        if field as u32 == ACTION_FIELD_AFFECT as i32 as u32 {
            let mut val: u32 = 0 as u32;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveEnum(
                ctx,
                value,
                &raw mut val,
                &raw const ptrDflts as *const LookupEntry,
            ) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"pointer component\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_BUTTON as i32 as u32
            || field as u32 == ACTION_FIELD_VALUE as i32 as u32
        {
            let mut button: *const ExprDef = ::core::ptr::null::<ExprDef>();
            let mut btn: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32
                || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as i32 as u32
            {
                (*act).flags = ((*act).flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32)
                    as xkb_action_flags;
                button = (*value).unary.child;
            } else {
                (*act).flags = ((*act).flags as u32 | ACTION_ABSOLUTE_SWITCH as i32 as u32)
                    as xkb_action_flags;
                button = value;
            }
            if !ExprResolveButton(ctx, button, &raw mut btn) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (range 1..5)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if btn < 0 as i64 || btn > 5 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "New default button value must be in the range 1..5; Illegal default button value {} ignored\n",
                    btn,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if btn == 0 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Cannot set default pointer button to \"default\"; Illegal default button setting ignored\n",
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).value = (if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                -btn
            } else {
                btn
            }) as i8;
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleSwitchScreen(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_switch_screen_action = &raw mut (*action).screen;
        if field as u32 == ACTION_FIELD_SCREEN as i32 as u32 {
            let mut scrn: *const ExprDef = ::core::ptr::null::<ExprDef>();
            let mut val: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32
                || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as i32 as u32
            {
                (*act).flags = ((*act).flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32)
                    as xkb_action_flags;
                scrn = (*value).unary.child;
            } else {
                (*act).flags = ((*act).flags as u32 | ACTION_ABSOLUTE_SWITCH as i32 as u32)
                    as xkb_action_flags;
                scrn = value;
            }
            if !ExprResolveInteger(ctx, scrn, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (-128..127)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            val = if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                -val
            } else {
                val
            };
            if val < i8::MIN as i64 || val > i8::MAX as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Screen index must be in the range {}..{}; Illegal screen value {} ignored\n",
                    -128 as i32,
                    127 as i32,
                    val,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).screen = val as i8;
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_SAME as i32 as u32 {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_SAME_SCREEN,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleSetLockControls(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_controls_action = &raw mut (*action).ctrls;
        if field as u32 == ACTION_FIELD_CONTROLS as i32 as u32 {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            let mut mask: u32 = 0 as u32;
            let offset: u8 = (*keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                ctx,
                value,
                &raw mut mask,
                (&raw const ctrlMaskNames as *const LookupEntry).offset(offset as i32 as isize),
            ) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"controls mask\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            (*act).ctrls = mask as xkb_action_controls;
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_AFFECT as i32 as u32
            && (*action).type_0 as u32 == ACTION_TYPE_CTRL_LOCK as i32 as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleRedirectKey(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let keymap: *const xkb_keymap = &raw const (*keymap_info).keymap;
        let ctx: *mut xkb_context = (*keymap).ctx;
        let act: *mut xkb_redirect_key_action = &raw mut (*action).redirect;
        if field as u32 == ACTION_FIELD_KEYCODE as i32 as u32 {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_IDENT as i32 as u32 {
                let mut valStr: *const i8 = xkb_atom_text(ctx, (*value).ident.ident);
                if !valStr.is_null() && istreq(valStr, b"auto\0".as_ptr() as *const i8) as i32 != 0
                {
                    (*act).keycode = (*keymap_info).keymap.redirect_key_auto;
                    return PARSER_SUCCESS;
                }
            }
            if (*value).common.type_0 as u32 != STMT_EXPR_KEYNAME_LITERAL as i32 as u32 {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"key name\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            let key: *const xkb_key = XkbKeyByName(keymap, (*value).key_name.key_name, 1 != 0);
            if key.is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "RedirectKey field {} cannot resolve {} to a valid key\n",
                    crate::xkb::utils::CStrDisplay(fieldText(field)),
                    crate::xkb::utils::CStrDisplay(KeyNameText(ctx, (*value).key_name.key_name)),
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_VALUE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).keycode = (*key).keycode;
            return PARSER_SUCCESS;
        }
        if field as u32 == ACTION_FIELD_MODIFIERS as i32 as u32
            || field as u32 == ACTION_FIELD_MODS_TO_CLEAR as i32 as u32
        {
            let mut flags: xkb_action_flags = 0 as xkb_action_flags;
            let mut m: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
            let mut r: xkb_parser_error = CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut flags,
                &raw mut m,
            );
            if r as u32 != PARSER_SUCCESS as i32 as u32 {
                return r;
            }
            if flags as u64 != 0 {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"modifier mask\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            (*act).affect |= m;
            if field as u32 == ACTION_FIELD_MODIFIERS as i32 as u32 {
                (*act).mods |= m;
            } else {
                (*act).mods &= !m;
            }
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleUnsupported(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    return PARSER_SUCCESS;
}
unsafe fn HandlePrivate(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: action_field,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        let mut act: *mut xkb_private_action = &raw mut (*action).priv_0;
        if field as u32 == ACTION_FIELD_TYPE as i32 as u32 {
            let mut type_0: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut type_0) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    ACTION_TYPE_PRIVATE,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if type_0 < 0 as i64 || type_0 > 255 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Private action type must be in the range 0..255; Illegal type {} ignored\n",
                    type_0,
                );
                return (if (*keymap_info).strict as u32
                    & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                    != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if type_0 < ACTION_TYPE_PRIVATE as i32 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_INFO,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Private actions of type {} are not supported; Ignored\n",
                    crate::xkb::utils::CStrDisplay(ActionTypeText(type_0 as xkb_action_type)),
                );
                (*act).type_0 = ACTION_TYPE_NONE;
            } else {
                (*act).type_0 = type_0 as xkb_action_type;
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_DATA as i32 as u32 {
            if array_ndx.is_null() {
                let mut val: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
                if !ExprResolveString(ctx, value, &raw mut val) {
                    return ReportMismatch(
                        ctx,
                        XKB_ERROR_WRONG_FIELD_TYPE,
                        (*action).type_0,
                        field,
                        b"string\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
                let mut str: *const i8 = xkb_atom_text(ctx, val);
                let mut len: usize = cstr_len(str);
                if len < 1 as usize || len > ::core::mem::size_of::<[u8; 7]>() as usize {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "A private action has {} data bytes, but got: {}; Illegal data ignored\n",
                        ::core::mem::size_of::<[u8; 7]>(),
                        len,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                std::ptr::write_bytes::<[u8; 7]>(
                    &raw mut (*act).data as *mut u8 as *mut [u8; 7],
                    0u8,
                    1,
                );
                std::ptr::copy_nonoverlapping(
                    str as *const u8,
                    &raw mut (*act).data as *mut u8,
                    len,
                );
                return PARSER_SUCCESS;
            } else {
                let mut ndx: i64 = 0 as i64;
                let mut datum: i64 = 0 as i64;
                if !ExprResolveInteger(ctx, array_ndx, &raw mut ndx) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Array subscript must be integer; Illegal subscript ignored\n",
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if ndx < 0 as i64 || ndx as usize >= ::core::mem::size_of::<[u8; 7]>() as usize {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "The data for a private action is {} bytes long; Attempt to use data[{}] ignored\n",
                        ::core::mem::size_of::<[u8; 7]>(),
                        ndx,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if !ExprResolveInteger(ctx, value, &raw mut datum) {
                    return ReportMismatch(
                        ctx,
                        XKB_ERROR_WRONG_FIELD_TYPE,
                        (*act).type_0,
                        field,
                        b"integer\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
                if datum < 0 as i64 || datum > 255 as i64 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "All data for a private action must be 0..255; Illegal datum {} ignored\n",
                        datum,
                    );
                    return (if (*keymap_info).strict as u32
                        & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                (*act).data[ndx as usize] = datum as u8;
                return PARSER_SUCCESS;
            }
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
static mut handleAction: [actionHandler; 21] = {
    [
        Some(
            HandleNoAction
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleNoAction
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleMovePtr
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePtrBtn
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePtrBtn
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetPtrDflt
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleNoAction
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSwitchScreen
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLockControls
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLockControls
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleRedirectKey
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleUnsupported
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleUnsupported
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePrivate
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    action_field,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        None,
    ]
};
pub unsafe fn HandleActionDef(
    mut keymap_info: *const xkb_keymap_info,
    mut info: *mut ActionsInfo,
    mut mods: *const xkb_mod_set,
    mut def: *mut ExprDef,
    mut action: *mut xkb_action,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = (*keymap_info).keymap.ctx;
        if (*def).common.type_0 as u32 != STMT_EXPR_ACTION_DECL as i32 as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Expected an action definition, found {}\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(stmt_type_to_string((*def).common.type_0)),
            );
            return PARSER_FATAL_ERROR;
        }
        let mut action_name: *const i8 = xkb_atom_text(ctx, (*def).action.name);
        let mut handler_type: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(action_name, &raw mut handler_type) {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action \"{}\"\n",
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                crate::xkb::utils::CStrDisplay(action_name),
            );
            handler_type = ACTION_TYPE_UNKNOWN;
            if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION as i32 as u32 != 0 {
                return PARSER_FATAL_ERROR;
            }
        }
        *action = (*info).actions[handler_type as usize];
        if handler_type as u32 == ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unsupported legacy action type \"{}\".\n",
                XKB_WARNING_UNSUPPORTED_LEGACY_ACTION as i32,
                crate::xkb::utils::CStrDisplay(action_name),
            );
            (*action).type_0 = ACTION_TYPE_NONE;
        }
        let mut ret: xkb_parser_error = PARSER_SUCCESS;
        let mut arg: *mut ExprDef = (*def).action.args as *mut ExprDef;
        while !arg.is_null() {
            let mut value: *const ExprDef = ::core::ptr::null::<ExprDef>();
            let mut value_ptr: *mut *mut ExprDef = ::core::ptr::null_mut::<*mut ExprDef>();
            let mut field: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            let mut arrayRtrn: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            let mut elemRtrn: *const i8 = ::core::ptr::null::<i8>();
            let mut fieldRtrn: *const i8 = ::core::ptr::null::<i8>();
            if (*arg).common.type_0 as u32 == STMT_EXPR_ASSIGN as i32 as u32 {
                field = (*arg).binary.left as *mut ExprDef;
                value = (*arg).binary.right;
                value_ptr = &raw mut (*arg).binary.right as *mut *mut ExprDef;
            } else if (*arg).common.type_0 as u32 == STMT_EXPR_NOT as i32 as u32
                || (*arg).common.type_0 as u32 == STMT_EXPR_INVERT as i32 as u32
            {
                field = (*arg).unary.child as *mut ExprDef;
                value = &raw const constFalse as *const ExprDef;
            } else {
                field = arg;
                value = &raw const constTrue as *const ExprDef;
            }
            if !ExprResolveLhs(
                ctx,
                field,
                &raw mut elemRtrn,
                &raw mut fieldRtrn,
                &raw mut arrayRtrn,
            ) {
                return PARSER_FATAL_ERROR;
            }
            if !elemRtrn.is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Cannot change defaults in an action definition; Ignoring attempt to change \"{}.{}\".\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    crate::xkb::utils::CStrDisplay(elemRtrn),
                    crate::xkb::utils::CStrDisplay(fieldRtrn),
                );
                return PARSER_FATAL_ERROR;
            }
            let mut fieldNdx: action_field = ACTION_FIELD_CLEAR_LOCKS;
            if !stringToField(fieldRtrn, &raw mut fieldNdx) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown field name {} for action {} discarded\n",
                    XKB_ERROR_INVALID_ACTION_FIELD as i32,
                    crate::xkb::utils::CStrDisplay(fieldRtrn),
                    crate::xkb::utils::CStrDisplay(ActionTypeText((*action).type_0)),
                );
                if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as i32 as u32 != 0
                {
                    return PARSER_FATAL_ERROR;
                }
            } else {
                match handleAction[handler_type as usize].expect("non-null function pointer")(
                    keymap_info,
                    mods,
                    action,
                    fieldNdx,
                    arrayRtrn,
                    value,
                    value_ptr,
                ) as u32
                {
                    2 => return PARSER_FATAL_ERROR,
                    1 => {
                        ret = PARSER_RECOVERABLE_ERROR;
                    }
                    _ => {}
                }
            }
            arg = (*arg).common.next as *mut ExprDef;
        }
        return (if (*action).type_0 as u32 == ACTION_TYPE_UNKNOWN as i32 as u32 {
            PARSER_RECOVERABLE_ERROR as i32 as u32
        } else {
            ret as u32
        }) as xkb_parser_error;
    }
}
pub unsafe fn SetDefaultActionField(
    mut keymap_info: *const xkb_keymap_info,
    mut info: *mut ActionsInfo,
    mut mods: *mut xkb_mod_set,
    mut elem: *const i8,
    mut field: *const i8,
    mut array_ndx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    mut merge: merge_mode,
) -> xkb_parser_error {
    unsafe {
        let value: *const ExprDef = *value_ptr;
        let mut action: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(elem, &raw mut action) {
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action \"{}\"\n",
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                crate::xkb::utils::CStrDisplay(elem),
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION as i32 as u32 != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let mut action_field: action_field = ACTION_FIELD_CLEAR_LOCKS;
        if !stringToField(field, &raw mut action_field) {
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action field \"{}\"\n",
                XKB_ERROR_INVALID_ACTION_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as i32 as u32
                != 0
            {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let into: *mut xkb_action = (&raw mut (*info).actions as *mut xkb_action)
            .offset(action as isize) as *mut xkb_action;
        let mut from: xkb_action = *into;
        let ret: xkb_parser_error = handleAction[action as usize]
            .expect("non-null function pointer")(
            keymap_info,
            mods,
            &raw mut from,
            action_field,
            array_ndx,
            value,
            value_ptr,
        ) as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS as i32 as u32 {
            return ret;
        }
        if !action_equal(into, &raw mut from) {
            let replace: bool = merge as u32 != MERGE_AUGMENT as i32 as u32;
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_VERBOSE as i32,
                "Conflicting field \"{}\" for default action \"{}\"; Using {}, ignore {}\n",
                crate::xkb::utils::CStrDisplay(fieldText(action_field)),
                crate::xkb::utils::CStrDisplay(ActionTypeText(action)),
                crate::xkb::utils::CStrDisplay(if replace as i32 != 0 {
                    b"from\0".as_ptr() as *const i8
                } else {
                    b"into\0".as_ptr() as *const i8
                }),
                crate::xkb::utils::CStrDisplay(if replace as i32 != 0 {
                    b"into\0".as_ptr() as *const i8
                } else {
                    b"from\0".as_ptr() as *const i8
                }),
            );
            if replace {
                *into = from;
            }
        }
        return PARSER_SUCCESS;
    }
}
