use crate::xkb::shared_types::xkb_context;
use crate::xkb::shared_types::*;
use crate::xkb::shared_types::{xkb_component_names, xkb_rule_names};
pub fn xkb_components_from_rules_names(
    ctx: *mut xkb_context,
    rmlvo: *const xkb_rule_names,
    out: *mut xkb_component_names,
    explicit_layouts: *mut u32,
) -> bool {
    crate::xkb::xkbcomp::rules::xkb_components_from_rules_names(
        ctx,
        rmlvo,
        out as *mut _,
        explicit_layouts,
    )
}

pub use crate::xkb::messages::{
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
    XKB_WARNING_CANNOT_INFER_KEY_TYPE, XKB_WARNING_CONFLICTING_KEY_ACTION,
    XKB_WARNING_CONFLICTING_KEY_FIELDS, XKB_WARNING_CONFLICTING_KEY_NAME,
    XKB_WARNING_CONFLICTING_KEY_SYMBOL, XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES, XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
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
use crate::xkb::shared_ast_types::FreeXkbFile;
pub use crate::xkb::shared_ast_types::{
    _ParseCommon, stmt_type, xkb_file_type_to_string, xkb_map_flags, ParseCommon, XkbFile,
    FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP,
    FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER,
    MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, STMT_ALIAS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN,
    STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF,
    STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT,
    STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY,
    STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT,
    STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP,
    STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD, _FILE_TYPE_NUM_ENTRIES, _STMT_NUM_VALUES,
};
pub use crate::xkb::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
use crate::xkb::xkbcomp::ast_build::XkbFileFromComponents;
use crate::xkb::xkbcomp::keymap::CompileKeymap;
use crate::xkb::xkbcomp::scanner::XkbParseString;

pub use crate::xkb::shared_types::{
    format_max_groups, xkb_action, xkb_action_controls, xkb_action_flags, xkb_controls_action,
    xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action, xkb_key,
    xkb_key_alias, xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keymap_serialize_flags,
    xkb_keysym_count_t, xkb_led, xkb_level, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods,
    xkb_overlay_mask_t, xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action,
    xkb_private_action, xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret,
    KeycodeMatch, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL,
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
    XKB_MAX_GROUPS_X11, _ACTION_TYPE_NUM_ENTRIES,
};

fn compile_keymap_file(keymap: *mut xkb_keymap, file: *mut XkbFile) -> bool {
    unsafe {
        if (*file).file_type != FILE_TYPE_KEYMAP {
            log::error!(
                "[XKB-{:03}] Cannot compile a {} file alone into a keymap\n",
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32,
                xkb_file_type_to_string((*file).file_type)
            );
            return false;
        }
        if !CompileKeymap(&mut *file, &mut *keymap) {
            log::error!(
                "[XKB-{:03}] Failed to compile keymap\n",
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
            );
            return false;
        }
        true
    }
}
pub fn text_v1_keymap_new_from_names(
    keymap: *mut xkb_keymap,
    rmlvo: *const xkb_rule_names,
) -> bool {
    unsafe {
        let mut ok: bool;
        let mut kccgst: xkb_component_names = xkb_component_names::default();

        log::debug!("Compiling from RMLVO: rules '{}', model '{}', layout '{}', variant '{}', options '{}'\n",
            (*rmlvo).rules.to_str().unwrap_or(""),
            (*rmlvo).model.to_str().unwrap_or(""),
            (*rmlvo).layout.to_str().unwrap_or(""),
            (*rmlvo).variant.to_str().unwrap_or(""),
            (*rmlvo).options.to_str().unwrap_or(""));
        ok = xkb_components_from_rules_names(
            &raw mut (*keymap).ctx,
            rmlvo,
            &raw mut kccgst,
            &raw mut (*keymap).num_groups,
        );
        if !ok {
            log::error!("[XKB-{:03}] Couldn't look up rules '{}', model '{}', layout '{}', variant '{}', options '{}'\n",
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32,
            &(*rmlvo).rules.to_str().unwrap_or(""),
            &(*rmlvo).model.to_str().unwrap_or(""),
            &(*rmlvo).layout.to_str().unwrap_or(""),
            &(*rmlvo).variant.to_str().unwrap_or(""),
            &(*rmlvo).options.to_str().unwrap_or(""));
            return false;
        }
        let max_groups: u32 = format_max_groups((*keymap).format);
        if (*keymap).num_groups > max_groups {
            (*keymap).num_groups = max_groups;
        }
        log::debug!(
            "Compiling from KcCGST: keycodes '{}', types '{}', compat '{}', symbols '{}'\n",
            std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                kccgst.keycodes.as_ptr()
            )),
            std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(kccgst.types.as_ptr())),
            std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                kccgst.compatibility.as_ptr()
            )),
            std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                kccgst.symbols.as_ptr()
            ))
        );
        let file: *mut XkbFile = XkbFileFromComponents(&raw mut (*keymap).ctx, &raw mut kccgst);
        drop(kccgst);
        if file.is_null() {
            log::error!(
                "[XKB-{:03}] Failed to generate parsed XKB file from components\n",
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
            );
            return false;
        }
        ok = compile_keymap_file(keymap, file);
        FreeXkbFile(file);
        ok
    }
}
pub fn text_v1_keymap_new_from_string(
    keymap: *mut xkb_keymap,
    string: *const i8,
    len: usize,
) -> bool {
    unsafe {
        let xkb_file: *mut XkbFile = XkbParseString(
            &raw mut (*keymap).ctx,
            string,
            len,
            "(input string)",
            std::ptr::null(),
        );
        if xkb_file.is_null() {
            log::error!(
                "[XKB-{:03}] Failed to parse input xkb string\n",
                XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
            );
            return false;
        }
        let ok: bool = compile_keymap_file(keymap, xkb_file);
        FreeXkbFile(xkb_file);
        ok
    }
}
