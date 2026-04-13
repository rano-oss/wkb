use crate::xkb::utils::{darray_append, darray_appends, darray_free};
use crate::xkb_logf;

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
pub mod text_h {
    pub use crate::xkb::text::LookupEntry;
    pub type C2Rust_Unnamed_16 = u32;
    pub const CONTROL_NAMES_MIN_V2_INDEX: C2Rust_Unnamed_16 = 0;
    pub const CONTROL_NAMES_MIN_V1_INDEX: C2Rust_Unnamed_16 = 7;
    pub const GROUP_LAST_INDEX_NAME: [i8; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"last\0") };
    #[inline]
    pub unsafe fn format_control_names_offset(mut format: xkb_keymap_format) -> u8 {
        return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as u32 {
            CONTROL_NAMES_MIN_V1_INDEX as ::core::ffi::c_int
        } else {
            CONTROL_NAMES_MIN_V2_INDEX as ::core::ffi::c_int
        }) as u8;
    }

    use crate::xkb::shared_types::{xkb_keymap_format, XKB_KEYMAP_FORMAT_TEXT_V1};
    pub use crate::xkb::text::{ActionTypeText, KeyNameText, KeysymText};
}
pub mod xkbcomp_priv_h {
    pub use crate::xkb::shared_ast_types::{
        pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
        xkb_parser_error, xkb_parser_strict_flags, XkbcompFeatures, XkbcompLookup,
        PARSER_FATAL_ERROR, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
        PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
        PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
        PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
        PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS,
        PARSER_V1_LAX_FLAGS, PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
    };
    pub type C2Rust_Unnamed_17 = XkbcompLookup;
    pub type C2Rust_Unnamed_18 = XkbcompFeatures;
    pub use crate::xkb::xkbcomp::compat::CompileCompatMap;
    pub use crate::xkb::xkbcomp::keycodes::CompileKeycodes;
    pub use crate::xkb::xkbcomp::symbols::CompileSymbols;
    pub use crate::xkb::xkbcomp::types::CompileKeyTypes;
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = 8;
}
pub mod ast_build_h {
    pub use crate::xkb::xkbcomp::ast_build::FreeStmt;
}
pub mod expr_h {

    pub use crate::xkb::xkbcomp::expr::{ExprResolveGroup, ExprResolveGroupMask};
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

use self::ast_build_h::FreeStmt;
pub use crate::xkb::shared_ast_types::{
    _ParseCommon, stmt_type, xkb_file_type, xkb_file_type_to_string, xkb_map_flags,
    C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef,
    ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString,
    ExprUnary, ParseCommon, XkbFile, _FILE_TYPE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES,
    FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
    MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR,
    MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, STMT_ALIAS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN,
    STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF,
    STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT,
    STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY,
    STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT,
    STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP,
    STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
use self::expr_h::{ExprResolveGroup, ExprResolveGroupMask};
pub use self::limits_h::CHAR_BIT;
pub use crate::xkb::messages::{
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
    format_control_names_offset, ActionTypeText, C2Rust_Unnamed_16, KeyNameText, KeysymText,
    LookupEntry, CONTROL_NAMES_MIN_V1_INDEX, CONTROL_NAMES_MIN_V2_INDEX, GROUP_LAST_INDEX_NAME,
};
pub use crate::xkb::utils::{is_aligned, memdup};
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_error, xkb_parser_strict_flags, C2Rust_Unnamed_17, C2Rust_Unnamed_18,
    PARSER_FATAL_ERROR, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
    PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
    PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use crate::xkb::shared_types::darray_size_t;
pub use crate::xkb::shared_types::{
    areOverlappingOverlaysSupported, format_max_groups, format_max_overlays,
    isGroupLockOnReleaseSupported, isModsLatchOnPressSupported, isModsUnLockOnPressSupported,
    mod_type, real_mod_index, xkb_action, xkb_action_controls, xkb_action_count_t,
    xkb_action_flags, xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group,
    xkb_group_action, xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias,
    xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level,
    xkb_match_operation, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t,
    xkb_overlay_mask_t, xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action,
    xkb_private_action, xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret,
    C2Rust_Unnamed_1, C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_14,
    C2Rust_Unnamed_15, C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5,
    C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch,
    XkbKeyNumLevels, _ACTION_TYPE_NUM_ENTRIES, _XKB_MOD_INDEX_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
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
    DEFAULT_INTERPRET_KEY_REPEAT, DEFAULT_INTERPRET_VMOD, DEFAULT_INTERPRET_VMODMAP,
    DEFAULT_KEY_REPEAT, DEFAULT_KEY_VMODMAP, EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT,
    EXPLICIT_SYMBOLS, EXPLICIT_TYPES, EXPLICIT_VMODMAP, FALLBACK_INTERPRET_KEY_REPEAT,
    FALLBACK_INTERPRET_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL,
    MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MAX_ACTIONS_PER_LEVEL, MOD_BOTH,
    MOD_REAL, MOD_VIRT, XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MAX_GROUPS_X11, XKB_MOD_INDEX_CAPS,
    XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1, XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3,
    XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5, XKB_MOD_INDEX_SHIFT, XKB_OVERLAY_MAX,
    XKB_OVERLAY_MAX_X11,
};
pub use crate::xkb::state::mod_mask_get_effective;
pub use crate::xkb::xkbcomp::compat::CompileCompatMap;
pub use crate::xkb::xkbcomp::keycodes::CompileKeycodes;
pub use crate::xkb::xkbcomp::symbols::CompileSymbols;
pub use crate::xkb::xkbcomp::types::CompileKeyTypes;
use libc::{calloc, free, realloc};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_sym_interprets {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *const xkb_sym_interpret,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_action,
}
pub const GROUP_MASK_NAME_LAST: C2Rust_Unnamed_21 = 3;
pub const GROUP_INDEX_NAME_LAST: C2Rust_Unnamed_20 = 1;
pub type compile_file_fn = Option<unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool>;
pub type C2Rust_Unnamed_20 = u32;
pub type C2Rust_Unnamed_21 = u32;
unsafe fn has_unbound_vmods(mut keymap: *mut xkb_keymap, mut mask: xkb_mod_mask_t) -> bool {
    unsafe {
        let mut k: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        k = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
            as *mut xkb_mod;
        while k < (*keymap).mods.num_mods {
            if mask & (1 as xkb_mod_mask_t) << k != 0 && (*mod_0).mapping == 0 as xkb_mod_mask_t {
                return 1 != 0;
            }
            k = k.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        return 0 != 0;
    }
}
#[inline]
unsafe fn ComputeEffectiveMask(mut keymap: *mut xkb_keymap, mut mods: *mut xkb_mods) {
    unsafe {
        let unknown_mods: xkb_mod_mask_t =
            !(((1 as u64) << (*keymap).mods.num_mods).wrapping_sub(1 as u64) as xkb_mod_mask_t);
        (*mods).mask = mod_mask_get_effective(keymap, (*mods).mods) | (*mods).mods & unknown_mods;
    }
}
unsafe fn UpdateActionMods(
    mut keymap: *mut xkb_keymap,
    mut act: *mut xkb_action,
    mut modmap: xkb_mod_mask_t,
) {
    unsafe {
        match (*act).type_0 as u32 {
            2 | 3 | 4 => {
                if (*act).mods.flags as u32 & ACTION_MODS_LOOKUP_MODMAP as ::core::ffi::c_int as u32
                    != 0
                {
                    (*act).mods.mods.mods = modmap;
                }
                ComputeEffectiveMask(keymap, &raw mut (*act).mods.mods);
            }
            _ => {}
        };
    }
}
static mut default_interpret: xkb_sym_interpret = xkb_sym_interpret {
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
};
unsafe fn FindInterpForKey(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut group: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut interprets: *mut xkb_sym_interprets,
) -> bool {
    unsafe {
        let mut syms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let mut num_syms: ::core::ffi::c_int = 0;
        num_syms =
            xkb_keymap_key_get_syms_by_level(keymap, (*key).keycode, group, level, &raw mut syms);
        if num_syms <= 0 as ::core::ffi::c_int {
            return 0 != 0;
        }
        let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while s < num_syms {
            let mut c2rust_current_block_34: u64;
            let mut found: bool = 0 != 0;
            let mut i: darray_size_t = 0 as darray_size_t;
            's_26: loop {
                if !(i < (*keymap).num_sym_interprets) {
                    c2rust_current_block_34 = 7659304154607701039;
                    break;
                }
                let interp: *mut xkb_sym_interpret =
                    (*keymap).sym_interprets.offset(i as isize) as *mut xkb_sym_interpret;
                let mut mods: xkb_mod_mask_t = 0;
                found = 0 != 0;
                if !((*interp).sym != *syms.offset(s as isize)
                    && (*interp).sym != XKB_KEY_NoSymbol as xkb_keysym_t)
                {
                    if (*interp).level_one_only as ::core::ffi::c_int != 0
                        && level != 0 as xkb_level_index_t
                    {
                        mods = 0 as xkb_mod_mask_t;
                    } else {
                        mods = (*key).modmap;
                    }
                    match (*interp).match_0 as u32 {
                        0 => {
                            found = (*interp).mods & mods == 0;
                        }
                        1 => {
                            found = mods == 0 || (*interp).mods & mods != 0;
                        }
                        2 => {
                            found = (*interp).mods & mods != 0;
                        }
                        3 => {
                            found = (*interp).mods & mods == (*interp).mods;
                        }
                        4 => {
                            found = (*interp).mods == mods;
                        }
                        _ => {}
                    }
                    if found as ::core::ffi::c_int != 0
                        && i > 0 as darray_size_t
                        && (*interp).sym == XKB_KEY_NoSymbol as xkb_keysym_t
                    {
                        let mut previous_interp: *mut *const xkb_sym_interpret =
                            ::core::ptr::null_mut::<*const xkb_sym_interpret>();
                        if !(*interprets).item.is_null() {
                            previous_interp =
                                (*interprets).item.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut *const xkb_sym_interpret;
                            while previous_interp
                                < (*interprets).item.offset((*interprets).size as isize)
                                    as *mut *const xkb_sym_interpret
                            {
                                if *previous_interp == interp as *const xkb_sym_interpret {
                                    found = 0 != 0;
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_WARNING,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        "Repeated interpretation ignored for keysym #{} \"{}\" at level {}/group {} on key {}.\n",
                                        s + 1 as ::core::ffi::c_int,
                                        crate::xkb::utils::CStrDisplay(KeysymText((*keymap).ctx, *syms.offset(s as isize))),
                                        level.wrapping_add(1 as xkb_level_index_t),
                                        group.wrapping_add(1 as xkb_layout_index_t),
                                        crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*key).name)),
                                    );
                                    c2rust_current_block_34 = 2209838995503123840;
                                    break 's_26;
                                } else {
                                    previous_interp = previous_interp.offset(1);
                                }
                            }
                        }
                    }
                    if found {
                        darray_append(
                            &mut (*interprets).item,
                            &mut (*interprets).size,
                            &mut (*interprets).alloc,
                            interp,
                        );
                        (*interp).set_required((1 != 0) as bool);
                        c2rust_current_block_34 = 7659304154607701039;
                        break;
                    }
                }
                i = i.wrapping_add(1);
            }
            match c2rust_current_block_34 {
                7659304154607701039 => {
                    if !found {
                        c2rust_current_block_34 = 2209838995503123840;
                    } else {
                        c2rust_current_block_34 = 2989495919056355252;
                    }
                }
                _ => {}
            }
            match c2rust_current_block_34 {
                2209838995503123840 => {
                    darray_append(
                        &mut (*interprets).item,
                        &mut (*interprets).size,
                        &mut (*interprets).alloc,
                        &raw const default_interpret,
                    );
                }
                _ => {}
            }
            s += 1;
        }
        return 1 != 0;
    }
}
unsafe fn ApplyInterpsToKey(mut keymap: *mut xkb_keymap, mut key: *mut xkb_key) -> bool {
    unsafe {
        let mut vmodmap: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut level: xkb_level_index_t = 0;
        let mut interprets: xkb_sym_interprets = xkb_sym_interprets {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<*const xkb_sym_interpret>(),
        };
        let mut actions: C2Rust_Unnamed_19 = C2Rust_Unnamed_19 {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<xkb_action>(),
        };
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while group < (*key).num_groups() {
            if !(*(*key).groups.offset(group as isize)).explicit_actions() {
                level = 0 as xkb_level_index_t;
                while level < XkbKeyNumLevels(key, group) {
                    let mut interp_iter: *mut *const xkb_sym_interpret =
                        ::core::ptr::null_mut::<*const xkb_sym_interpret>();
                    let mut interp: *const xkb_sym_interpret =
                        ::core::ptr::null::<xkb_sym_interpret>();
                    let mut k: usize = 0;
                    interprets.size = 0 as darray_size_t;
                    let found: bool =
                        FindInterpForKey(keymap, key, group, level, &raw mut interprets) as bool;
                    if found {
                        if !interprets.item.is_null() {
                            k = 0 as usize;
                            interp_iter = interprets.item.offset(0 as ::core::ffi::c_int as isize)
                                as *mut *const xkb_sym_interpret;
                            while k < interprets.size as usize {
                                interp = *interp_iter;
                                if group == 0 as xkb_layout_index_t
                                    && level == 0 as xkb_level_index_t
                                {
                                    if (*key).explicit as u32
                                        & EXPLICIT_REPEAT as ::core::ffi::c_int as u32
                                        == 0
                                        && (*interp).repeat() as ::core::ffi::c_int != 0
                                    {
                                        (*key).set_repeats((1 != 0) as bool);
                                    }
                                }
                                if group == 0 as xkb_layout_index_t
                                    && level == 0 as xkb_level_index_t
                                    || !(*interp).level_one_only
                                {
                                    if (*interp).virtual_mod != XKB_MOD_INVALID as xkb_mod_index_t {
                                        vmodmap = (vmodmap as u32
                                            | (1 as u32) << (*interp).virtual_mod)
                                            as xkb_mod_mask_t;
                                    }
                                }
                                match (*interp).num_actions as ::core::ffi::c_int {
                                    0 => {}
                                    1 => {
                                        darray_append(
                                            &mut actions.item,
                                            &mut actions.size,
                                            &mut actions.alloc,
                                            (*interp).a.action,
                                        );
                                    }
                                    _ => {
                                        darray_appends(
                                            &mut actions.item,
                                            &mut actions.size,
                                            &mut actions.alloc,
                                            (*interp).a.actions,
                                            (*interp).num_actions as u32,
                                        );
                                    }
                                }
                                k = k.wrapping_add(1);
                                interp_iter = interp_iter.offset(1);
                            }
                        }
                        if (actions.size != 0) as ::core::ffi::c_int as i64
                            > MAX_ACTIONS_PER_LEVEL as i64
                        {
                            xkb_logf!(
                                (*keymap).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                "Could not append interpret actions to key {}: maximum is {}, got: {}. Dropping excessive actions\n",
                                crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*key).name)),
                                65535 as ::core::ffi::c_int,
                                actions.size,
                            );
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_actions = MAX_ACTIONS_PER_LEVEL as xkb_action_count_t;
                        } else {
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_actions = actions.size as xkb_action_count_t;
                        }
                        match actions.size {
                            0 => {
                                (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .action = xkb_action {
                                    type_0: ACTION_TYPE_NONE,
                                };
                            }
                            1 => {
                                (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .action = *actions.item.offset(0 as ::core::ffi::c_int as isize);
                            }
                            _ => {
                                let ref mut c2rust_fresh0 =
                                    (*(*(*key).groups.offset(group as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                *c2rust_fresh0 = memdup(
                                    actions.item as *const ::core::ffi::c_void,
                                    (*(*(*key).groups.offset(group as isize))
                                        .levels
                                        .offset(level as isize))
                                    .num_actions as usize,
                                    ::core::mem::size_of::<xkb_action>() as usize,
                                )
                                    as *mut xkb_action;
                                if (*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .a
                                .actions
                                .is_null()
                                {
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        "[XKB-{:03}] Could not allocate interpret actions\n",
                                        XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
                                    );
                                    darray_free(
                                        &mut actions.item,
                                        &mut actions.size,
                                        &mut actions.alloc,
                                    );
                                    darray_free(
                                        &mut interprets.item,
                                        &mut interprets.size,
                                        &mut interprets.alloc,
                                    );
                                    return 0 != 0;
                                }
                            }
                        }
                        if !(actions.size == 0 as darray_size_t) {
                            let ref mut c2rust_fresh1 = *(*key).groups.offset(group as isize);
                            (*c2rust_fresh1).set_implicit_actions((1 != 0) as bool);
                        }
                        actions.size = 0 as darray_size_t;
                    }
                    level = level.wrapping_add(1);
                }
                if (*(*key).groups.offset(group as isize)).implicit_actions() {
                    (*key).set_implicit_actions((1 != 0) as bool);
                }
            }
            group = group.wrapping_add(1);
        }
        darray_free(&mut actions.item, &mut actions.size, &mut actions.alloc);
        darray_free(
            &mut interprets.item,
            &mut interprets.size,
            &mut interprets.alloc,
        );
        if (*key).explicit as u32 & EXPLICIT_VMODMAP as ::core::ffi::c_int as u32 == 0 {
            (*key).vmodmap = vmodmap;
        }
        return 1 != 0;
    }
}
#[inline]
unsafe fn is_mod_action(mut action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as u32 == ACTION_TYPE_MOD_SET as ::core::ffi::c_int as u32
            || (*action).type_0 as u32 == ACTION_TYPE_MOD_LATCH as ::core::ffi::c_int as u32
            || (*action).type_0 as u32 == ACTION_TYPE_MOD_LOCK as ::core::ffi::c_int as u32;
    }
}
#[inline]
unsafe fn is_group_action(mut action: *mut xkb_action) -> bool {
    unsafe {
        return (*action).type_0 as u32 == ACTION_TYPE_GROUP_SET as ::core::ffi::c_int as u32
            || (*action).type_0 as u32 == ACTION_TYPE_GROUP_LATCH as ::core::ffi::c_int as u32
            || (*action).type_0 as u32 == ACTION_TYPE_GROUP_LOCK as ::core::ffi::c_int as u32;
    }
}
unsafe fn CheckMultipleActionsCategories(mut keymap: *mut xkb_keymap, mut key: *mut xkb_key) {
    unsafe {
        let mut g: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while g < (*key).num_groups() {
            let mut l: xkb_level_index_t = 0 as xkb_level_index_t;
            while l < XkbKeyNumLevels(key, g) {
                let mut level: *mut xkb_level = (*(*key).groups.offset(g as isize))
                    .levels
                    .offset(l as isize)
                    as *mut xkb_level;
                if !((*level).num_actions as ::core::ffi::c_int <= 1 as ::core::ffi::c_int) {
                    let mut i: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (i as ::core::ffi::c_int) < (*level).num_actions as ::core::ffi::c_int {
                        let mut action1: *mut xkb_action =
                            (*level).a.actions.offset(i as isize) as *mut xkb_action;
                        let mut mod_action: bool = is_mod_action(action1);
                        let mut group_action: bool = is_group_action(action1);
                        if mod_action as ::core::ffi::c_int != 0
                            || group_action as ::core::ffi::c_int != 0
                            || (*action1).type_0 as u32
                                == ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int as u32
                        {
                            let mut j: xkb_action_count_t = (i as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as xkb_action_count_t;
                            while (j as ::core::ffi::c_int)
                                < (*level).num_actions as ::core::ffi::c_int
                            {
                                let mut action2: *mut xkb_action =
                                    (*level).a.actions.offset(j as isize) as *mut xkb_action;
                                if (*action1).type_0 as u32 == (*action2).type_0 as u32
                                    || mod_action as ::core::ffi::c_int != 0
                                        && is_mod_action(action2) as ::core::ffi::c_int != 0
                                    || group_action as ::core::ffi::c_int != 0
                                        && is_group_action(action2) as ::core::ffi::c_int != 0
                                {
                                    let type_0: *const i8 = if mod_action as ::core::ffi::c_int != 0
                                    {
                                        b"modifiers\0".as_ptr() as *const i8
                                    } else if group_action as ::core::ffi::c_int != 0 {
                                        b"group\0".as_ptr() as *const i8
                                    } else {
                                        ActionTypeText((*action1).type_0) as *const i8
                                    };
                                    xkb_logf!(
                                        (*keymap).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                        "Cannot use multiple {} actions in the same level. Action #{} for key {} in group {}/level {} ignored.\n",
                                        crate::xkb::utils::CStrDisplay(type_0),
                                        j as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                        crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*key).name)),
                                        g.wrapping_add(1 as xkb_layout_index_t),
                                        l.wrapping_add(1 as xkb_level_index_t),
                                    );
                                    (*action2).type_0 = ACTION_TYPE_NONE;
                                }
                                j = j.wrapping_add(1);
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                }
                l = l.wrapping_add(1);
            }
            g = g.wrapping_add(1);
        }
    }
}
unsafe fn add_key_aliases(
    mut keymap: *mut xkb_keymap,
    mut min: darray_size_t,
    mut max: darray_size_t,
    mut aliases: *mut xkb_key_alias,
) {
    unsafe {
        let mut alias: darray_size_t = min;
        while alias <= max {
            let entry: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(alias as isize);
            if entry.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
                && entry.c2rust_unnamed.found() as ::core::ffi::c_int != 0
            {
                *aliases = xkb_key_alias {
                    real: entry.alias.real(),
                    alias: alias as xkb_atom_t,
                };
                aliases = aliases.offset(1);
            }
            alias = alias.wrapping_add(1);
        }
    }
}
unsafe fn update_pending_key_fields(mut info: *mut xkb_keymap_info, mut key: *mut xkb_key) -> bool {
    unsafe {
        if (*key).out_of_range_pending_group() {
            let pc: *mut pending_computation = (*(*info).pending_computations)
                .item
                .offset((*key).out_of_range_group_number() as isize)
                as *mut pending_computation;
            if !(*pc).computed {
                let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                match ExprResolveGroup(
                    info,
                    (*pc).expr,
                    1 != 0,
                    &raw mut group,
                    ::core::ptr::null_mut::<bool>(),
                ) as u32
                {
                    0 => {
                        (*pc).computed = 1 != 0;
                        (*pc).value = group.wrapping_sub(1 as xkb_layout_index_t) as u32;
                    }
                    2 => {
                        xkb_logf!(
                            (*info).keymap.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            "[XKB-{:03}] Invalid key redirect group index\n",
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                        );
                        return (*info).strict as u32
                            & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as u32
                            != 0;
                    }
                    _ => {}
                }
            }
            (*key).set_out_of_range_pending_group((0 != 0) as bool);
            (*key).set_out_of_range_group_number(
                (*pc).value as xkb_layout_index_t as xkb_layout_index_t,
            );
        }
        return 1 != 0;
    }
}
unsafe fn update_pending_action_fields(
    mut info: *mut xkb_keymap_info,
    mut keycode: xkb_keycode_t,
    mut act: *mut xkb_action,
) -> bool {
    unsafe {
        match (*act).type_0 as u32 {
            5 | 6 | 7 => {
                if (*act).group.flags as u32
                    & ACTION_PENDING_COMPUTATION as ::core::ffi::c_int as u32
                    != 0
                {
                    let pc: *mut pending_computation = (*(*info).pending_computations)
                        .item
                        .offset((*act).group.group as isize)
                        as *mut pending_computation;
                    if !(*pc).computed {
                        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
                        let absolute: bool = (*act).group.flags as u32
                            & ACTION_ABSOLUTE_SWITCH as ::core::ffi::c_int as u32
                            != 0;
                        match ExprResolveGroup(
                            info,
                            (*pc).expr,
                            absolute,
                            &raw mut group,
                            ::core::ptr::null_mut::<bool>(),
                        ) as u32
                        {
                            2 => {
                                xkb_logf!(
                                    (*info).keymap.ctx,
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                                    "[XKB-{:03}] Invalid action group index\n",
                                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                                );
                                return 0 != 0;
                            }
                            1 => {}
                            _ => {
                                (*pc).computed = 1 != 0;
                                if absolute {
                                    (*pc).value =
                                        group.wrapping_sub(1 as xkb_layout_index_t) as i32 as u32;
                                } else {
                                    (*pc).value = group as u32;
                                    if (*(*pc).expr).common.type_0 as u32
                                        == STMT_EXPR_NEGATE as ::core::ffi::c_int as u32
                                    {
                                        (*pc).value = -((*pc).value as i32) as u32;
                                    }
                                }
                            }
                        }
                    }
                    (*act).group.group = (*pc).value as i32;
                    (*act).group.flags = ((*act).group.flags as u32
                        & !(ACTION_PENDING_COMPUTATION as ::core::ffi::c_int) as u32)
                        as xkb_action_flags;
                }
                return 1 != 0;
            }
            16 => {
                if keycode == XKB_KEYCODE_INVALID as xkb_keycode_t
                    || (*act).redirect.keycode != (*info).keymap.redirect_key_auto
                {
                    return 1 != 0;
                } else {
                    (*act).redirect.keycode = keycode;
                }
                return 1 != 0;
            }
            _ => return 1 != 0,
        };
    }
}
unsafe fn update_pending_led_fields(mut info: *mut xkb_keymap_info, mut led: *mut xkb_led) -> bool {
    unsafe {
        if (*led).pending_groups() {
            let pc: *mut pending_computation = (*(*info).pending_computations)
                .item
                .offset((*led).groups as isize)
                as *mut pending_computation;
            if !(*pc).computed {
                let mut mask: xkb_layout_mask_t = 0 as xkb_layout_mask_t;
                if !ExprResolveGroupMask(
                    info,
                    (*pc).expr,
                    &raw mut mask,
                    ::core::ptr::null_mut::<bool>(),
                ) {
                    xkb_logf!(
                        (*info).keymap.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "[XKB-{:03}] Invalid LED group mask\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as ::core::ffi::c_int,
                    );
                    return 0 != 0;
                }
                (*pc).computed = 1 != 0;
                (*pc).value = mask as u32;
            }
            (*led).set_pending_groups((0 != 0) as bool);
            (*led).groups = (*pc).value as xkb_layout_mask_t;
        }
        return 1 != 0;
    }
}
unsafe fn UpdateDerivedKeymapFields(mut info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let keymap: *mut xkb_keymap = &raw mut (*info).keymap;
        let mut num_key_aliases: darray_size_t = 0 as darray_size_t;
        let mut min_alias: darray_size_t = 0 as darray_size_t;
        let mut max_alias: darray_size_t = 0 as darray_size_t;
        let mut alias: xkb_atom_t = 0 as xkb_atom_t;
        while alias < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
            let entry: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(alias as isize);
            if entry.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
                && entry.c2rust_unnamed.found() as ::core::ffi::c_int != 0
            {
                if num_key_aliases == 0 {
                    min_alias = alias as darray_size_t;
                }
                max_alias = alias as darray_size_t;
                num_key_aliases = num_key_aliases.wrapping_add(1);
            }
            alias = alias.wrapping_add(1);
        }
        if num_key_aliases != 0 {
            let required_space: darray_size_t = (::core::mem::size_of::<xkb_key_alias>() as usize)
                .wrapping_div(::core::mem::size_of::<KeycodeMatch>() as usize)
                .wrapping_mul(num_key_aliases as usize)
                as darray_size_t;
            if min_alias >= required_space {
                add_key_aliases(
                    keymap,
                    min_alias,
                    max_alias,
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases,
                );
                let r: *mut xkb_key_alias = realloc(
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases
                        as *mut ::core::ffi::c_void,
                    (num_key_aliases as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_key_alias>() as usize),
                ) as *mut xkb_key_alias;
                if r.is_null() {
                    return 0 != 0;
                }
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = r;
            } else if (*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .num_key_names
                .wrapping_sub(max_alias)
                .wrapping_sub(1 as darray_size_t)
                > required_space
            {
                let aliases: *mut xkb_key_alias = (*keymap)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .key_names
                    .offset(max_alias as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset(!is_aligned(
                        (*keymap)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .key_names
                            .offset(max_alias as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<xkb_key_alias>() as usize,
                    ) as ::core::ffi::c_int as isize)
                    as *mut xkb_key_alias;
                add_key_aliases(keymap, min_alias, max_alias, aliases);
                std::ptr::copy_nonoverlapping::<xkb_key_alias>(
                    aliases,
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases,
                    num_key_aliases as usize,
                );
                let r_0: *mut xkb_key_alias = realloc(
                    (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases
                        as *mut ::core::ffi::c_void,
                    (num_key_aliases as usize)
                        .wrapping_mul(::core::mem::size_of::<xkb_key_alias>() as usize),
                ) as *mut xkb_key_alias;
                if r_0.is_null() {
                    return 0 != 0;
                }
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = r_0;
            } else {
                let aliases_0: *mut xkb_key_alias = calloc(
                    num_key_aliases as usize,
                    ::core::mem::size_of::<xkb_key_alias>() as usize,
                ) as *mut xkb_key_alias;
                if aliases_0.is_null() {
                    return 0 != 0;
                }
                add_key_aliases(keymap, min_alias, max_alias, aliases_0);
                free((*keymap).c2rust_unnamed.c2rust_unnamed.key_names as *mut ::core::ffi::c_void);
                (*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases = aliases_0;
            }
        }
        (*keymap).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases = num_key_aliases;
        let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            (*keymap).num_groups = if (*keymap).num_groups > (*key).num_groups() {
                (*keymap).num_groups
            } else {
                (*key).num_groups()
            };
            key = key.offset(1);
        }
        let pending_computations: bool =
            !((*(*info).pending_computations).size == 0 as darray_size_t);
        if pending_computations {
            let num_groups: xkb_layout_index_t = if (*keymap).num_groups != 0 {
                (*keymap).num_groups
            } else {
                1 as xkb_layout_index_t
            };
            (*info).lookup.groupIndexNames[GROUP_INDEX_NAME_LAST as ::core::ffi::c_int as usize] =
                LookupEntry {
                    name: GROUP_LAST_INDEX_NAME.as_ptr(),
                    value: num_groups as u32,
                };
            (*info).lookup.groupMaskNames[GROUP_MASK_NAME_LAST as ::core::ffi::c_int as usize] =
                LookupEntry {
                    name: GROUP_LAST_INDEX_NAME.as_ptr(),
                    value: (1 as u32) << num_groups.wrapping_sub(1 as xkb_layout_index_t),
                };
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < (*keymap).num_sym_interprets {
                let interp: *mut xkb_sym_interpret =
                    (*keymap).sym_interprets.offset(i as isize) as *mut xkb_sym_interpret;
                if (*interp).num_actions as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
                    let act: *mut xkb_action = &raw mut (*interp).a.action;
                    if !update_pending_action_fields(
                        info,
                        XKB_KEYCODE_INVALID as xkb_keycode_t,
                        act,
                    ) {
                        return 0 != 0;
                    }
                } else {
                    let mut a: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (a as ::core::ffi::c_int) < (*interp).num_actions as ::core::ffi::c_int {
                        let act_0: *mut xkb_action =
                            (*interp).a.actions.offset(a as isize) as *mut xkb_action;
                        if !update_pending_action_fields(
                            info,
                            XKB_KEYCODE_INVALID as xkb_keycode_t,
                            act_0,
                        ) {
                            return 0 != 0;
                        }
                        a = a.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            if !ApplyInterpsToKey(keymap, key) {
                return 0 != 0;
            }
            CheckMultipleActionsCategories(keymap, key);
            key = key.offset(1);
        }
        let mut idx: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
                as *mut xkb_mod;
            while idx < (*keymap).mods.num_mods {
                if (*key).vmodmap & (1 as xkb_mod_mask_t) << idx != 0 {
                    (*mod_0).mapping |= (*key).modmap;
                }
                idx = idx.wrapping_add(1);
                mod_0 = mod_0.offset(1);
            }
            key = key.offset(1);
        }
        if (*keymap).format as u32 >= XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int as u32 {
            idx = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
            mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
                .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
                as *mut xkb_mod;
            while idx < (*keymap).mods.num_mods {
                let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << idx;
                if (*mod_0).mapping == 0 as xkb_mod_mask_t
                    && (*keymap).mods.explicit_vmods & mask == 0
                {
                    (*mod_0).mapping = mask;
                    (*keymap).mods.explicit_vmods |= mask;
                }
                idx = idx.wrapping_add(1);
                mod_0 = mod_0.offset(1);
            }
        }
        let mut extra_canonical_mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        idx = _XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as xkb_mod_index_t;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as ::core::ffi::c_int as isize)
            as *mut xkb_mod;
        while idx < (*keymap).mods.num_mods {
            extra_canonical_mods |= (*mod_0).mapping;
            idx = idx.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*keymap).canonical_state_mask |= extra_canonical_mods;
        let mut i_0: darray_size_t = 0 as darray_size_t;
        while i_0 < (*keymap).num_types {
            ComputeEffectiveMask(
                keymap,
                &raw mut (*(*keymap).types.offset(i_0 as isize)).mods,
            );
            let mut j: darray_size_t = 0 as darray_size_t;
            while j < (*(*keymap).types.offset(i_0 as isize)).num_entries {
                if has_unbound_vmods(
                    keymap,
                    (*(*(*keymap).types.offset(i_0 as isize))
                        .entries
                        .offset(j as isize))
                    .mods
                    .mods,
                ) {
                    (*(*(*keymap).types.offset(i_0 as isize))
                        .entries
                        .offset(j as isize))
                    .mods
                    .mask = 0 as xkb_mod_mask_t;
                } else {
                    ComputeEffectiveMask(
                        keymap,
                        &raw mut (*(*(*keymap).types.offset(i_0 as isize))
                            .entries
                            .offset(j as isize))
                        .mods,
                    );
                    ComputeEffectiveMask(
                        keymap,
                        &raw mut (*(*(*keymap).types.offset(i_0 as isize))
                            .entries
                            .offset(j as isize))
                        .preserve,
                    );
                }
                j = j.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            if !update_pending_key_fields(info, key) {
                return 0 != 0;
            }
            let mut i_1: xkb_layout_index_t = 0 as xkb_layout_index_t;
            while i_1 < (*key).num_groups() {
                let mut j_0: xkb_level_index_t = 0 as xkb_level_index_t;
                while j_0 < XkbKeyNumLevels(key, i_1) {
                    if (*(*(*key).groups.offset(i_1 as isize))
                        .levels
                        .offset(j_0 as isize))
                    .num_actions as ::core::ffi::c_int
                        <= 1 as ::core::ffi::c_int
                    {
                        let act_1: *mut xkb_action =
                            &raw mut (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .a
                            .action;
                        UpdateActionMods(keymap, act_1, (*key).modmap);
                        if (pending_computations as ::core::ffi::c_int != 0
                            || (*act_1).type_0 as u32
                                == ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int as u32)
                            && !update_pending_action_fields(info, (*key).keycode, act_1)
                        {
                            return 0 != 0;
                        }
                    } else {
                        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
                        while (k as ::core::ffi::c_int)
                            < (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .num_actions as ::core::ffi::c_int
                        {
                            let act_2: *mut xkb_action = (*(*(*key).groups.offset(i_1 as isize))
                                .levels
                                .offset(j_0 as isize))
                            .a
                            .actions
                            .offset(k as isize)
                                as *mut xkb_action;
                            UpdateActionMods(keymap, act_2, (*key).modmap);
                            if (pending_computations as ::core::ffi::c_int != 0
                                || (*act_2).type_0 as u32
                                    == ACTION_TYPE_REDIRECT_KEY as ::core::ffi::c_int as u32)
                                && !update_pending_action_fields(info, (*key).keycode, act_2)
                            {
                                return 0 != 0;
                            }
                            k = k.wrapping_add(1);
                        }
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                i_1 = i_1.wrapping_add(1);
            }
            key = key.offset(1);
        }
        let mut led: *mut xkb_led = ::core::ptr::null_mut::<xkb_led>();
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while led < (&raw mut (*keymap).leds as *mut xkb_led).offset((*keymap).num_leds as isize) {
            ComputeEffectiveMask(keymap, &raw mut (*led).mods);
            if pending_computations as ::core::ffi::c_int != 0
                && !update_pending_led_fields(info, led)
            {
                return 0 != 0;
            }
            led = led.offset(1);
        }
        return 1 != 0;
    }
}
static mut compile_file_fns: [compile_file_fn; 4] = {
    [
        Some(CompileKeycodes as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileKeyTypes as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileCompatMap as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
        Some(CompileSymbols as unsafe fn(*mut XkbFile, *mut xkb_keymap_info) -> bool),
    ]
};
unsafe fn pending_computations_array_free(mut p: *mut pending_computation_array) {
    unsafe {
        let mut pc: *mut pending_computation = ::core::ptr::null_mut::<pending_computation>();
        if !(*p).item.is_null() {
            pc = (*p).item.offset(0 as ::core::ffi::c_int as isize) as *mut pending_computation;
            while pc < (*p).item.offset((*p).size as isize) as *mut pending_computation {
                FreeStmt((*pc).expr as *mut ParseCommon);
                pc = pc.offset(1);
            }
        }
        darray_free(&mut (*p).item, &mut (*p).size, &mut (*p).alloc);
    }
}
pub unsafe fn CompileKeymap(mut file: *mut XkbFile, mut keymap: *mut xkb_keymap) -> bool {
    unsafe {
        let mut files: [*mut XkbFile; 4] = [
            ::core::ptr::null_mut::<XkbFile>(),
            ::core::ptr::null_mut::<XkbFile>(),
            ::core::ptr::null_mut::<XkbFile>(),
            ::core::ptr::null_mut::<XkbFile>(),
        ];
        let mut type_0: xkb_file_type = FILE_TYPE_KEYCODES;
        let mut ctx: *mut xkb_context = (*keymap).ctx;
        file = (*file).defs as *mut XkbFile;
        while !file.is_null() {
            if ((*file).file_type as u32) < FIRST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as u32
                || (*file).file_type as u32 > LAST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as u32
            {
                if (*file).file_type as u32 == FILE_TYPE_GEOMETRY as ::core::ffi::c_int as u32 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_BRIEF as ::core::ffi::c_int,
                        "[XKB-{:03}] Geometry sections are not supported; ignoring\n",
                        XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION as ::core::ffi::c_int,
                    );
                } else {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Cannot define {} in a keymap file\n",
                        crate::xkb::utils::CStrDisplay(xkb_file_type_to_string((*file).file_type)),
                    );
                }
            } else if !files[(*file).file_type as usize].is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "More than one {} section in keymap file; All sections after the first ignored\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string((*file).file_type)),
                );
            } else {
                files[(*file).file_type as usize] = file;
            }
            file = (*file).common.next as *mut XkbFile;
        }
        let mut pending_computations: pending_computation_array = pending_computation_array {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<pending_computation>(),
        };
        let mut info: xkb_keymap_info = xkb_keymap_info {
            keymap: *keymap,
            strict: (if (*keymap).format as u32
                == XKB_KEYMAP_FORMAT_TEXT_V1 as ::core::ffi::c_int as u32
            {
                if (*keymap).flags as u32
                    & XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int as u32
                    != 0
                {
                    PARSER_V1_STRICT_FLAGS as ::core::ffi::c_int
                } else {
                    PARSER_V1_LAX_FLAGS as ::core::ffi::c_int
                }
            } else if (*keymap).flags as u32
                & XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int as u32
                != 0
            {
                PARSER_V2_STRICT_FLAGS as ::core::ffi::c_int
            } else {
                PARSER_V2_LAX_FLAGS as ::core::ffi::c_int
            }) as xkb_parser_strict_flags,
            features: C2Rust_Unnamed_18 {
                max_groups: format_max_groups((*keymap).format),
                max_overlays: format_max_overlays((*keymap).format),
                controls_name_offset: format_control_names_offset((*keymap).format),
                group_lock_on_release: isGroupLockOnReleaseSupported((*keymap).format),
                mods_unlock_on_press: isModsUnLockOnPressSupported((*keymap).format),
                mods_latch_on_press: isModsLatchOnPressSupported((*keymap).format),
                overlapping_overlays: areOverlappingOverlaysSupported((*keymap).format),
            },
            lookup: C2Rust_Unnamed_17 {
                groupIndexNames: [
                    LookupEntry {
                        name: b"first\0".as_ptr() as *const i8,
                        value: 1 as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME.as_ptr()
                        } else {
                            ::core::ptr::null::<i8>()
                        },
                        value: (*keymap).num_groups as u32,
                    },
                    LookupEntry {
                        name: ::core::ptr::null::<i8>(),
                        value: 0 as u32,
                    },
                ],
                groupMaskNames: [
                    LookupEntry {
                        name: b"none\0".as_ptr() as *const i8,
                        value: 0 as u32,
                    },
                    LookupEntry {
                        name: b"first\0".as_ptr() as *const i8,
                        value: 0x1 as u32,
                    },
                    LookupEntry {
                        name: b"all\0".as_ptr() as *const i8,
                        value: XKB_ALL_GROUPS as u32,
                    },
                    LookupEntry {
                        name: if (*keymap).num_groups != 0 {
                            GROUP_LAST_INDEX_NAME.as_ptr()
                        } else {
                            ::core::ptr::null::<i8>()
                        },
                        value: if (*keymap).num_groups != 0
                            && (*keymap).num_groups <= XKB_MAX_GROUPS as xkb_layout_index_t
                        {
                            (1 as u32) << (*keymap).num_groups.wrapping_sub(1 as xkb_layout_index_t)
                        } else {
                            0 as u32
                        },
                    },
                    LookupEntry {
                        name: ::core::ptr::null::<i8>(),
                        value: 0 as u32,
                    },
                ],
            },
            pending_computations: &raw mut pending_computations,
        };
        type_0 = FIRST_KEYMAP_FILE_TYPE;
        while type_0 as u32 <= LAST_KEYMAP_FILE_TYPE as ::core::ffi::c_int as u32 {
            if files[type_0 as usize].is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Component {} not provided in keymap\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(type_0)),
                );
            } else {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Compiling {} \"{}\"\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(type_0)),
                    crate::xkb::utils::CStrDisplay(safe_map_name(files[type_0 as usize])),
                );
            }
            let ok: bool = compile_file_fns[type_0 as usize].expect("non-null function pointer")(
                files[type_0 as usize],
                &raw mut info,
            ) as bool;
            if !ok {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Failed to compile {}\n",
                    crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(type_0)),
                );
                *keymap = info.keymap;
                pending_computations_array_free(&raw mut pending_computations);
                return 0 != 0;
            }
            type_0 += 1;
        }
        let ok_0: bool = UpdateDerivedKeymapFields(&raw mut info) as bool;
        *keymap = info.keymap;
        pending_computations_array_free(&raw mut pending_computations);
        return ok_0;
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        default_interpret = {
            let mut init = xkb_sym_interpret {
                repeat_required: [0; 1],
                sym: XKB_KEY_NoSymbol as xkb_keysym_t,
                match_0: MATCH_ANY_OR_NONE,
                mods: 0 as xkb_mod_mask_t,
                virtual_mod: DEFAULT_INTERPRET_VMOD as u32 as xkb_mod_index_t,
                level_one_only: false,
                num_actions: 0 as xkb_action_count_t,
                a: C2Rust_Unnamed_1 {
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                },
            };
            init.set_repeat(DEFAULT_INTERPRET_KEY_REPEAT as ::core::ffi::c_int != 0);
            init.set_required(false);
            init
        }
    }
}
use crate::xkb::keymap::xkb_keymap_key_get_syms_by_level;
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
