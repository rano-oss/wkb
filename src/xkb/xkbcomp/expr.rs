use crate::xkb_logf;
pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
}

pub mod xkbcommon_errors_h {
    pub type xkb_error_code = i32;
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
        XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
        XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED,
        XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_overlay_index_t = u8;
    pub const XKB_LEVEL_MAX_IMPL: i32 = 2048 as i32;
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    pub type C2Rust_Unnamed_13 = DarrayKeysym;
    pub use crate::xkb::xkbcomp::ast_build::{stmt_type_to_operator_char, stmt_type_to_string};
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
pub mod text_h {
    pub use crate::xkb::text::{buttonNames, LookupEntry};
    pub const GROUP_LAST_INDEX_NAME: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"last\0") };
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
pub mod inttypes_h {
    extern "C" {
        pub fn imaxabs(__n: i64) -> i64;
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
    pub unsafe fn istrneq(mut s1: *const i8, mut s2: *const i8, mut len: usize) -> bool {
        unsafe {
            return istrncmp(s1, s2, len) == 0 as i32;
        }
    }

    pub use crate::xkb::utils::{istrcmp, istrncmp};
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe fn parse_dec_to_uint32_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut u32,
    ) -> i32 {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as i32 - '0' as i32) as ::core::ffi::c_uchar as u32)
                    < 10 as u32
                && result <= (4294967295 as u32).wrapping_div(10 as u32)
                && result.wrapping_mul(10 as u32)
                    <= (4294967295 as u32).wrapping_sub(
                        (*s.offset(i as isize) as i32 - '0' as i32) as ::core::ffi::c_uchar as u32,
                    )
            {
                result = result
                    .wrapping_mul(10 as u32)
                    .wrapping_add((*s.offset(i as isize) as i32 - '0' as i32) as u32);
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len
                || (*s.offset(i as isize) as i32 - '0' as i32) as ::core::ffi::c_uchar as u32
                    >= 10 as u32
            {
                i as i32
            } else {
                -1 as i32
            };
        }
    }
}

pub use self::ast_h::{
    _ParseCommon, stmt_type, stmt_type_to_operator_char, stmt_type_to_string, C2Rust_Unnamed_13,
    ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef,
    ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary,
    ParseCommon, _STMT_NUM_VALUES, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
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
pub use self::context_h::{xkb_atom_text, xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::internal::__va_list_tag;
use self::inttypes_h::imaxabs;
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
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT,
    XKB_LEVEL_MAX_IMPL,
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
pub use self::text_h::{buttonNames, LookupEntry, GROUP_LAST_INDEX_NAME};
pub use self::utils_h::{istrcmp, istreq, istrncmp, istrneq};
pub use self::utils_numbers_h::parse_dec_to_uint32_t;
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t, xkb_layout_index_t,
    xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_level_index_t,
    xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names, xkb_state_component,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS,
    XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED,
    XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE,
    XKB_STATE_MODS_LATCHED, XKB_STATE_MODS_LOCKED,
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
pub use crate::xkb::keymap_priv::XkbModNameToIndex;
pub use crate::xkb::shared_types::darray_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupModMaskPriv {
    pub mods: *const xkb_mod_set,
    pub mod_type: mod_type,
}
pub type IdentLookupFunc = Option<
    unsafe fn(
        *mut xkb_context,
        *const ::core::ffi::c_void,
        xkb_atom_t,
        *mut u32,
        *mut bool,
    ) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_integer_pattern {
    pub prefix: *const i8,
    pub prefix_length: usize,
    pub min: u32,
    pub max: u32,
    pub entries: *const LookupEntry,
    pub pending_entries: *const LookupEntry,
    pub is_mask: bool,
    pub error_id: xkb_message_code,
}
static mut level_name_pattern: named_integer_pattern = named_integer_pattern {
    prefix: ::core::ptr::null::<i8>(),
    prefix_length: 0,
    min: 0,
    max: 0,
    entries: ::core::ptr::null::<LookupEntry>(),
    pending_entries: ::core::ptr::null::<LookupEntry>(),
    is_mask: false,
    error_id: 0 as xkb_message_code,
};
pub unsafe fn ExprResolveLhs(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut elem_rtrn: *mut *const i8,
    mut field_rtrn: *mut *const i8,
    mut index_rtrn: *mut *mut ExprDef,
) -> bool {
    unsafe {
        match (*expr).common.type_0 as u32 {
            10 => {
                *elem_rtrn = ::core::ptr::null::<i8>();
                *field_rtrn = xkb_atom_text(ctx, (*expr).ident.ident);
                *index_rtrn = ::core::ptr::null_mut::<ExprDef>();
                return !(*field_rtrn).is_null();
            }
            12 => {
                *elem_rtrn = xkb_atom_text(ctx, (*expr).field_ref.element);
                *field_rtrn = xkb_atom_text(ctx, (*expr).field_ref.field);
                *index_rtrn = ::core::ptr::null_mut::<ExprDef>();
                return !(*elem_rtrn).is_null() && !(*field_rtrn).is_null();
            }
            13 => {
                *elem_rtrn = xkb_atom_text(ctx, (*expr).array_ref.element);
                *field_rtrn = xkb_atom_text(ctx, (*expr).array_ref.field);
                *index_rtrn = (*expr).array_ref.entry as *mut ExprDef;
                if (*expr).array_ref.element != XKB_ATOM_NONE as xkb_atom_t
                    && (*elem_rtrn).is_null()
                {
                    return 0 != 0;
                }
                if (*field_rtrn).is_null() {
                    return 0 != 0;
                }
                return 1 != 0;
            }
            _ => {}
        }
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_CRITICAL,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Unexpected operator {} in ResolveLhs\n",
            XKB_ERROR_INVALID_XKB_SYNTAX as i32,
            (*expr).common.type_0 as u32,
        );
        return 0 != 0;
    }
}
unsafe fn SimpleLookup(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut u32,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE as xkb_atom_t {
            return 0 != 0;
        }
        let mut str: *const i8 = xkb_atom_text(ctx, field);
        let mut entry: *const LookupEntry = priv_0 as *const LookupEntry;
        while !entry.is_null() && !(*entry).name.is_null() {
            if istreq(str, (*entry).name) {
                *val_rtrn = (*entry).value;
                return 1 != 0;
            }
            entry = entry.offset(1);
        }
        return 0 != 0;
    }
}
unsafe fn NamedIntegerPatternLookup(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut u32,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE as xkb_atom_t {
            return 0 != 0;
        }
        let str: *const i8 = xkb_atom_text(ctx, field) as *const i8;
        let pattern: *const named_integer_pattern = priv_0 as *const named_integer_pattern;
        let count: i32 = if istrneq(str, (*pattern).prefix, (*pattern).prefix_length) as i32 != 0 {
            parse_dec_to_uint32_t(
                str.offset((*pattern).prefix_length as isize),
                usize::MAX as usize,
                val_rtrn as *mut u32,
            ) as i32
        } else {
            0 as i32
        };
        if count > 0 as i32
            && *str
                .offset((*pattern).prefix_length as isize)
                .offset(count as isize) as i32
                == '\0' as i32
        {
            if *val_rtrn < (*pattern).min || *val_rtrn > (*pattern).max {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} index {} is out of range ({}..{})\n",
                    (*pattern).error_id as u32,
                    crate::xkb::utils::CStrDisplay((*pattern).prefix),
                    *val_rtrn,
                    (*pattern).min,
                    (*pattern).max,
                );
                return 0 != 0;
            }
            if (*pattern).is_mask {
                *val_rtrn = ((1 as u32) << (*val_rtrn).wrapping_sub((*pattern).min)) as u32;
            }
            return 1 != 0;
        } else {
            if !(*pattern).entries.is_null()
                && SimpleLookup(
                    ctx,
                    (*pattern).entries as *const ::core::ffi::c_void,
                    field,
                    val_rtrn,
                    ::core::ptr::null_mut::<bool>(),
                ) as i32
                    != 0
            {
                return 1 != 0;
            }
            if !(*pattern).pending_entries.is_null()
                && !pending_rtrn.is_null()
                && SimpleLookup(
                    ctx,
                    (*pattern).pending_entries as *const ::core::ffi::c_void,
                    field,
                    val_rtrn,
                    ::core::ptr::null_mut::<bool>(),
                ) as i32
                    != 0
            {
                *pending_rtrn = 1 != 0;
                return 1 != 0;
            }
            return 0 != 0;
        };
    }
}
unsafe fn LookupModMask(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut xkb_mod_mask_t,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        let mut str: *const i8 = xkb_atom_text(ctx, field);
        if str.is_null() {
            return 0 != 0;
        }
        if istreq(str, b"all\0".as_ptr() as *const i8) {
            *val_rtrn = MOD_REAL_MASK_ALL;
            return 1 != 0;
        }
        if istreq(str, b"none\0".as_ptr() as *const i8) {
            *val_rtrn = 0 as xkb_mod_mask_t;
            return 1 != 0;
        }
        let mut arg: *const LookupModMaskPriv = priv_0 as *const LookupModMaskPriv;
        let mut mods: *const xkb_mod_set = (*arg).mods;
        let mod_type: mod_type = (*arg).mod_type;
        let ndx: xkb_mod_index_t = XkbModNameToIndex(mods, field, mod_type) as xkb_mod_index_t;
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            return 0 != 0;
        }
        *val_rtrn = ((1 as u32) << ndx) as xkb_mod_mask_t;
        return 1 != 0;
    }
}
pub unsafe fn ExprResolveBoolean(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut set_rtrn: *mut bool,
) -> bool {
    unsafe {
        let mut ok: bool = 0 != 0;
        let mut ident: *const i8 = ::core::ptr::null::<i8>();
        match (*expr).common.type_0 as u32 {
            7 => {
                *set_rtrn = (*expr).boolean.set;
                return 1 != 0;
            }
            4 | 5 | 6 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {} where boolean was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return 0 != 0;
            }
            10 => {
                ident = xkb_atom_text(ctx, (*expr).ident.ident);
                if !ident.is_null() {
                    if istreq(ident, b"true\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"yes\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"on\0".as_ptr() as *const i8) as i32 != 0
                    {
                        *set_rtrn = 1 != 0;
                        return 1 != 0;
                    } else if istreq(ident, b"false\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"no\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"off\0".as_ptr() as *const i8) as i32 != 0
                    {
                        *set_rtrn = 0 != 0;
                        return 1 != 0;
                    }
                }
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Identifier \"{}\" of type boolean is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    crate::xkb::utils::CStrDisplay(ident),
                );
                return 0 != 0;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type boolean is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return 0 != 0;
            }
            24 | 22 => {
                ok = ExprResolveBoolean(ctx, (*expr).unary.child, set_rtrn);
                if ok {
                    *set_rtrn = !*set_rtrn;
                }
                return ok;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 25 | 14 | 11 | 16 | 15 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} of boolean values not permitted\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator {} in ResolveBoolean\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
            }
        }
        return 0 != 0;
    }
}
unsafe fn ExprResolveIntegerLookup(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut i64,
    mut pending: *mut bool,
    mut lookup: IdentLookupFunc,
    mut lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = 0 != 0;
        let mut l: i64 = 0 as i64;
        let mut r: i64 = 0 as i64;
        let mut u: u32 = 0 as u32;
        let mut left: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut right: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        match (*expr).common.type_0 as u32 {
            5 => {
                *val_rtrn = (*expr).integer.ival;
                return 1 != 0;
            }
            4 | 6 | 7 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {} where an int was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return 0 != 0;
            }
            10 => {
                if lookup.is_some() {
                    ok = lookup.expect("non-null function pointer")(
                        ctx,
                        lookupPriv,
                        (*expr).ident.ident,
                        &raw mut u,
                        pending,
                    );
                }
                if !ok {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                        XKB_ERROR_INVALID_IDENTIFIER as i32,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
                    );
                } else {
                    *val_rtrn = u as i64;
                }
                if !pending.is_null() && *pending as i32 != 0 {
                    return 0 != 0;
                }
                return ok;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return 0 != 0;
            }
            17 | 18 | 19 | 20 => {
                left = (*expr).binary.left as *mut ExprDef;
                right = (*expr).binary.right as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveIntegerLookup(
                        ctx, right, &raw mut r, pending, lookup, lookupPriv,
                    )
                {
                    return 0 != 0;
                }
                match (*expr).common.type_0 as u32 {
                    17 => {
                        let (c2rust_fresh0, c2rust_fresh1) = l.overflowing_add(r);
                        *val_rtrn = c2rust_fresh0;
                        if c2rust_fresh1 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Addition {} + {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return 0 != 0;
                        }
                    }
                    18 => {
                        let (c2rust_fresh2, c2rust_fresh3) = l.overflowing_sub(r);
                        *val_rtrn = c2rust_fresh2;
                        if c2rust_fresh3 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Substraction {} - {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return 0 != 0;
                        }
                    }
                    19 => {
                        let (c2rust_fresh4, c2rust_fresh5) = l.overflowing_mul(r);
                        *val_rtrn = c2rust_fresh4;
                        if c2rust_fresh5 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Multiplication {} * {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return 0 != 0;
                        }
                    }
                    20 => {
                        if r == 0 as i64 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Cannot divide by zero: {} / {}\n",
                                XKB_ERROR_INVALID_OPERATION as i32,
                                l,
                                r,
                            );
                            return 0 != 0;
                        }
                        *val_rtrn = l / r;
                    }
                    _ => {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {} of integers not permitted\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            crate::xkb::utils::CStrDisplay(stmt_type_to_string(
                                (*expr).common.type_0
                            )),
                        );
                        return 0 != 0;
                    }
                }
                return 1 != 0;
            }
            21 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Assignment operator not implemented yet\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                );
            }
            22 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The ! operator cannot be applied to an integer\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                );
                return 0 != 0;
            }
            24 | 23 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv) {
                    return 0 != 0;
                }
                *val_rtrn = if (*expr).common.type_0 as u32 == STMT_EXPR_NEGATE as i32 as u32 {
                    -l
                } else {
                    !l
                };
                return 1 != 0;
            }
            25 => {
                left = (*expr).unary.child as *mut ExprDef;
                return ExprResolveIntegerLookup(ctx, left, val_rtrn, pending, lookup, lookupPriv);
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator {} in ResolveInteger\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
            }
        }
        return 0 != 0;
    }
}
pub unsafe fn ExprResolveInteger(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut i64,
) -> bool {
    unsafe {
        return ExprResolveIntegerLookup(
            ctx,
            expr,
            val_rtrn,
            ::core::ptr::null_mut::<bool>(),
            None,
            ::core::ptr::null::<::core::ffi::c_void>(),
        );
    }
}
pub unsafe fn ExprResolveGroup(
    mut keymap_info: *const xkb_keymap_info,
    mut expr: *const ExprDef,
    mut absolute: bool,
    mut group_rtrn: *mut xkb_layout_index_t,
    mut pending: *mut bool,
) -> xkb_parser_error {
    unsafe {
        static mut pendingGroupIndexNames: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: 0 as u32,
            },
            LookupEntry {
                name: ::core::ptr::null::<i8>(),
                value: 0 as u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: b"Group\0".as_ptr() as *const i8,
            prefix_length: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: (*keymap_info).features.max_groups as u32,
            entries: &raw const (*keymap_info).lookup.groupIndexNames as *const LookupEntry,
            pending_entries: &raw const pendingGroupIndexNames as *const LookupEntry,
            is_mask: 0 != 0,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        let mut result: i64 = 0 as i64;
        if !ExprResolveIntegerLookup(
            (*keymap_info).keymap.ctx,
            expr,
            &raw mut result,
            pending,
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        ) {
            return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                != 0
            {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        if result < absolute as i32 as i64 || result > (*keymap_info).features.max_groups as i64 {
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Group index {} is out of range ({}..{})\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                result,
                absolute as i32,
                (*keymap_info).features.max_groups,
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as i32 as u32
                != 0
            {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        *group_rtrn = result as xkb_layout_index_t;
        return PARSER_SUCCESS;
    }
}
pub unsafe fn ExprResolveLevel(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut level_rtrn: *mut xkb_level_index_t,
) -> bool {
    unsafe {
        let mut result: i64 = 0 as i64;
        if !ExprResolveIntegerLookup(
            ctx,
            expr,
            &raw mut result,
            ::core::ptr::null_mut::<bool>(),
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const level_name_pattern as *const ::core::ffi::c_void,
        ) {
            return 0 != 0;
        }
        if result < 1 as i64 || result > XKB_LEVEL_MAX_IMPL as i64 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Shift level {} is out of range (1..{})\n",
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32,
                result,
                2048 as i32,
            );
            return 0 != 0;
        }
        *level_rtrn = (result - 1 as i64) as xkb_level_index_t;
        return 1 != 0;
    }
}
pub unsafe fn ExprResolveButton(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut btn_rtrn: *mut i64,
) -> bool {
    unsafe {
        return ExprResolveIntegerLookup(
            ctx,
            expr,
            btn_rtrn,
            ::core::ptr::null_mut::<bool>(),
            Some(
                SimpleLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const buttonNames as *const LookupEntry as *const ::core::ffi::c_void,
        );
    }
}
pub unsafe fn ExprResolveString(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut xkb_atom_t,
) -> bool {
    unsafe {
        match (*expr).common.type_0 as u32 {
            4 => {
                *val_rtrn = (*expr).string.str;
                return 1 != 0;
            }
            5 | 6 | 7 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {}, expected a string\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return 0 != 0;
            }
            10 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Identifier \"{}\" of type string not found\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
                );
                return 0 != 0;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type string not found\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return 0 != 0;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 24 | 22 | 25 | 14 | 11 | 16 | 15 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} of strings not permitted\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return 0 != 0;
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator {} in ResolveString\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
            }
        }
        return 0 != 0;
    }
}
pub unsafe fn ExprResolveEnum(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut u32,
    mut values: *const LookupEntry,
) -> bool {
    unsafe {
        if (*expr).common.type_0 as u32 != STMT_EXPR_IDENT as i32 as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Found a {} where an enumerated value was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
            );
            return 0 != 0;
        }
        if !SimpleLookup(
            ctx,
            values as *const ::core::ffi::c_void,
            (*expr).ident.ident,
            val_rtrn,
            ::core::ptr::null_mut::<bool>(),
        ) {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Illegal identifier {}; expected one of:\n",
                XKB_ERROR_INVALID_IDENTIFIER as i32,
                crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
            );
            while !values.is_null() && !(*values).name.is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] \t{}\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    crate::xkb::utils::CStrDisplay((*values).name),
                );
                values = values.offset(1);
            }
            return 0 != 0;
        }
        return 1 != 0;
    }
}
unsafe fn ExprResolveMaskLookup(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut u32,
    mut pending: *mut bool,
    mut lookup: IdentLookupFunc,
    mut lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = 0 != 0;
        let mut l: u32 = 0 as u32;
        let mut r: u32 = 0 as u32;
        let mut v: i64 = 0 as i64;
        let mut left: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut right: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut bogus: *const i8 = ::core::ptr::null::<i8>();
        let mut c2rust_current_block_47: u64;
        match (*expr).common.type_0 as u32 {
            5 => {
                if (*expr).integer.ival < 0 as i64 || (*expr).integer.ival > u32::MAX as i64 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Mask {}{:#x} is out of range (0..{:#x})\n",
                        crate::xkb::utils::CStrDisplay(if (*expr).integer.ival < 0 as i64 {
                            b"-\0".as_ptr() as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        }),
                        imaxabs((*expr).integer.ival as i64),
                        4294967295 as u32,
                    );
                    return 0 != 0;
                }
                *val_rtrn = (*expr).integer.ival as u32;
                return 1 != 0;
            }
            4 | 6 | 7 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {} where a mask was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return 0 != 0;
            }
            10 => {
                ok = lookup.expect("non-null function pointer")(
                    ctx,
                    lookupPriv,
                    (*expr).ident.ident,
                    val_rtrn,
                    pending,
                );
                if !ok {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                        XKB_ERROR_INVALID_IDENTIFIER as i32,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
                    );
                }
                if !pending.is_null() && *pending as i32 != 0 {
                    return 0 != 0;
                }
                return ok;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return 0 != 0;
            }
            13 => {
                bogus = b"array reference\0".as_ptr() as *const i8;
                c2rust_current_block_47 = 6716998617863641615;
            }
            11 => {
                c2rust_current_block_47 = 6716998617863641615;
            }
            17 | 18 | 19 | 20 => {
                left = (*expr).binary.left as *mut ExprDef;
                right = (*expr).binary.right as *mut ExprDef;
                if !ExprResolveMaskLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveMaskLookup(ctx, right, &raw mut r, pending, lookup, lookupPriv)
                {
                    return 0 != 0;
                }
                match (*expr).common.type_0 as u32 {
                    17 => {
                        *val_rtrn = l | r;
                    }
                    18 => {
                        *val_rtrn = l & !r;
                    }
                    19 | 20 => {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Cannot {} masks; Illegal operation ignored\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            crate::xkb::utils::CStrDisplay(
                                if (*expr).common.type_0 as u32 == STMT_EXPR_DIVIDE as i32 as u32 {
                                    b"divide\0".as_ptr() as *const i8
                                } else {
                                    b"multiply\0".as_ptr() as *const i8
                                }
                            ),
                        );
                        return 0 != 0;
                    }
                    _ => {}
                }
                return 1 != 0;
            }
            21 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Assignment operator not implemented yet\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                );
                c2rust_current_block_47 = 11626999923138678822;
            }
            24 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return 0 != 0;
                }
                if v < 0 as i64 || v > u32::MAX as i64 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Mask {}{:#x} is out of range (0..{:#x})\n",
                        crate::xkb::utils::CStrDisplay(if v < 0 as i64 {
                            b"-\0".as_ptr() as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        }),
                        imaxabs(v as i64),
                        4294967295 as u32,
                    );
                    return 0 != 0;
                }
                *val_rtrn = !(v as u32);
                return 1 != 0;
            }
            25 | 23 | 22 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return 0 != 0;
                }
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The '{}' unary operator cannot be used with a mask\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                    (stmt_type_to_operator_char((*expr).common.type_0) as u8 as char),
                );
                return 0 != 0;
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator type {} in ResolveMask\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
                c2rust_current_block_47 = 11626999923138678822;
            }
        }
        match c2rust_current_block_47 {
            11626999923138678822 => {}
            _ => {
                if bogus.is_null() {
                    bogus = b"function use\0".as_ptr() as *const i8;
                }
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unexpected {} in mask expression; Expression Ignored\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(bogus),
                );
                return 0 != 0;
            }
        }
        return 0 != 0;
    }
}
pub unsafe fn ExprResolveMask(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut mask_rtrn: *mut u32,
    mut values: *const LookupEntry,
) -> bool {
    unsafe {
        return ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn,
            ::core::ptr::null_mut::<bool>(),
            Some(
                SimpleLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            values as *const ::core::ffi::c_void,
        );
    }
}
pub unsafe fn ExprResolveModMask(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut mod_type: mod_type,
    mut mods: *const xkb_mod_set,
    mut mask_rtrn: *mut xkb_mod_mask_t,
) -> bool {
    unsafe {
        let mut priv_0: LookupModMaskPriv = LookupModMaskPriv {
            mods: mods,
            mod_type: mod_type,
        };
        return ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn as *mut u32,
            ::core::ptr::null_mut::<bool>(),
            Some(
                LookupModMask
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut xkb_mod_mask_t,
                        *mut bool,
                    ) -> bool,
            ),
            &raw mut priv_0 as *const ::core::ffi::c_void,
        );
    }
}
pub unsafe fn ExprResolveMod(
    mut ctx: *mut xkb_context,
    mut def: *const ExprDef,
    mut mod_type: mod_type,
    mut mods: *const xkb_mod_set,
    mut ndx_rtrn: *mut xkb_mod_index_t,
) -> bool {
    unsafe {
        if (*def).common.type_0 as u32 != STMT_EXPR_IDENT as i32 as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot resolve virtual modifier: found {} where a virtual modifier name was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(stmt_type_to_string((*def).common.type_0)),
            );
            return 0 != 0;
        }
        let mut name: xkb_atom_t = (*def).ident.ident;
        let mut ndx: xkb_mod_index_t = XkbModNameToIndex(mods, name, mod_type);
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot resolve virtual modifier: \"{}\" was not previously declared\n",
                XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER as i32,
                crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, name)),
            );
            return 0 != 0;
        }
        *ndx_rtrn = ndx;
        return 1 != 0;
    }
}
pub unsafe fn ExprResolveGroupMask(
    mut keymap_info: *const xkb_keymap_info,
    mut expr: *const ExprDef,
    mut group_rtrn: *mut xkb_layout_mask_t,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        static mut pendingGroupMaskNames: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: 0 as u32,
            },
            LookupEntry {
                name: ::core::ptr::null::<i8>(),
                value: 0 as u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: b"Group\0".as_ptr() as *const i8,
            prefix_length: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: (*keymap_info).features.max_groups as u32,
            entries: &raw const (*keymap_info).lookup.groupMaskNames as *const LookupEntry,
            pending_entries: &raw const pendingGroupMaskNames as *const LookupEntry,
            is_mask: 1 != 0,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        return ExprResolveMaskLookup(
            (*keymap_info).keymap.ctx,
            expr,
            group_rtrn as *mut u32,
            pending_rtrn,
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        );
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        level_name_pattern = named_integer_pattern {
            prefix: b"Level\0".as_ptr() as *const i8,
            prefix_length: (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: XKB_LEVEL_MAX_IMPL as u32,
            entries: ::core::ptr::null::<LookupEntry>(),
            pending_entries: ::core::ptr::null::<LookupEntry>(),
            is_mask: 0 != 0,
            error_id: XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
        }
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
