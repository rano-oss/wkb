use crate::xkb::context_priv::{xkb_atom_intern, xkb_atom_text};
use crate::xkb::keysym::xkb_keysym_is_keypad;
use crate::xkb::keysym_case_mappings::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
use crate::xkb_logf;
use c2rust_bitfields;

pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::text::{ActionTypeText, KeyNameText, KeysymText, LookupEntry, ModIndexText};
use crate::xkb::xkbcomp::expr::{
    ExprResolveBoolean, ExprResolveEnum, ExprResolveGroup, ExprResolveLhs, ExprResolveModMask,
    ExprResolveString,
};

pub use crate::xkb::keymap::clear_level;
pub use crate::xkb::keymap_priv::{
    XkbEscapeMapName, XkbLevelsSameActions, XkbLevelsSameSyms, XkbModNameToIndex,
};
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
pub use crate::xkb::shared_ast_types::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, ModMapDef, ParseCommon, SymbolsDef,
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
pub use crate::xkb::shared_ast_types::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_error, xkb_parser_strict_flags, FreeXkbFile, XkbcompFeatures, XkbcompLookup,
    PARSER_FATAL_ERROR, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
    PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
    PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use crate::xkb::shared_types::{
    mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_14, C2Rust_Unnamed_15,
    C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6,
    C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, XkbKeyByName,
    XkbKeyNumLevels, _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X,
    ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK,
    ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE,
    ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION, ACTION_SAME_SCREEN,
    ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK,
    ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK,
    ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON,
    ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY,
    ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
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
    MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT,
    XKB_MOD_NONE, XKB_OVERLAY_INVALID,
};
pub use crate::xkb::shared_types::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use crate::xkb::utils::_steal;
use crate::xkb::utils::cstr_free;
use crate::xkb::utils::cstr_len;
use crate::xkb::utils::darray_append;
pub use crate::xkb::utils::{istrcmp, istreq, istrncmp, istrneq, memdup, strdup_safe};
pub use crate::xkb::utils::{next_pow2, parse_dec_to_uint64_t, popcount32};
pub use crate::xkb::xkbcomp::action::{
    ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField,
};
use crate::xkb::xkbcomp::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
use crate::xkb::xkbcomp::vmod::{HandleVModDef, InitVMods, MergeModSets};
use libc::{abort, atoi, calloc, free, realloc};
#[derive(Clone)]
pub struct SymbolsInfo {
    pub name: *mut i8,
    pub errorCount: i32,
    pub include_depth: u32,
    pub explicit_group: xkb_layout_index_t,
    pub max_groups: xkb_layout_index_t,
    pub keys: Vec<KeyInfo>,
    pub default_key: KeyInfo,
    pub default_actions: ActionsInfo,
    pub group_names: Vec<xkb_atom_t>,
    pub modmaps: Vec<ModMapEntry>,
    pub mods: xkb_mod_set,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModMapEntry {
    pub merge: merge_mode,
    pub haveSymbol: bool,
    pub modifier: xkb_mod_index_t,
    pub u: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_19 {
    pub keyName: xkb_atom_t,
    pub keySym: xkb_keysym_t,
}
#[derive(Clone, BitfieldStruct)]
#[repr(C)]
pub struct KeyInfo {
    pub name: xkb_atom_t,
    pub vmodmap: xkb_mod_mask_t,
    pub default_type: xkb_atom_t,
    pub out_of_range_group_number: xkb_layout_index_t,
    pub groups: Vec<GroupInfo>,
    #[bitfield(
        name = "out_of_range_group_policy",
        ty = "xkb_layout_out_of_range_policy",
        bits = "0..=7"
    )]
    #[bitfield(name = "defined", ty = "key_field", bits = "8..=23")]
    #[bitfield(name = "merge", ty = "merge_mode", bits = "24..=31")]
    #[bitfield(name = "repeat", ty = "key_repeat", bits = "32..=39")]
    #[bitfield(name = "out_of_range_pending_group", ty = "bool", bits = "40..=40")]
    #[bitfield(name = "overlays_clear", ty = "bool", bits = "41..=41")]
    pub out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear:
        [u8; 6],
    pub overlays_alloc: xkb_overlay_index_t,
    pub overlays: xkb_overlay_mask_t,
    pub c2rust_unnamed: C2Rust_Unnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_21 {
    pub overlay_key: *const xkb_key,
    pub overlays_keys: *mut *const xkb_key,
}
pub type key_repeat = u32;
pub const _KEY_REPEAT_NUM_ENTRIES: key_repeat = 3;
pub const KEY_REPEAT_NO: key_repeat = 2;
pub const KEY_REPEAT_YES: key_repeat = 1;
pub const KEY_REPEAT_UNDEFINED: key_repeat = 0;
pub type key_field = u32;
pub const KEY_FIELD_ALL: key_field = 31;
pub const KEY_FIELD_OVERLAY: key_field = 16;
pub const KEY_FIELD_VMODMAP: key_field = 8;
pub const KEY_FIELD_GROUPINFO: key_field = 4;
pub const KEY_FIELD_DEFAULT_TYPE: key_field = 2;
pub const KEY_FIELD_REPEAT: key_field = 1;
#[derive(Clone)]
pub struct GroupInfo {
    pub levels: Vec<xkb_level>,
    pub defined: group_field,
    pub type_0: xkb_atom_t,
}
pub type group_field = u32;
pub const GROUP_FIELD_TYPE: group_field = 4;
pub const GROUP_FIELD_ACTS: group_field = 2;
pub const GROUP_FIELD_SYMS: group_field = 1;

/// Resize a Vec<T> to `new_len`, zero-initializing any new elements.
/// If `new_len` < current len, the Vec is truncated.
/// WARNING: Only safe for types where all-zeros is a valid representation.
/// For types containing Vec/String/Box, use resize_groups_zero or similar.
unsafe fn darray_resize_zero_vec<T>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        let ptr = v.as_mut_ptr().add(old_len);
        std::ptr::write_bytes(ptr, 0, new_len - old_len);
        v.set_len(new_len);
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

/// Resize a Vec<GroupInfo> to `new_len`, properly initializing new elements.
/// Unlike darray_resize_zero_vec, this correctly initializes Vec fields in GroupInfo.
unsafe fn resize_groups_zero(v: &mut Vec<GroupInfo>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        // Zero-fill the raw memory first (for scalar fields)
        let ptr = v.as_mut_ptr().add(old_len);
        std::ptr::write_bytes(ptr, 0, new_len - old_len);
        v.set_len(new_len);
        // Now properly initialize Vec fields in new elements
        for i in old_len..new_len {
            std::ptr::write(&raw mut (*v.as_mut_ptr().add(i)).levels, Vec::new());
        }
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

unsafe fn StealLevelInfo(mut into: *mut xkb_level, mut from: *mut xkb_level) {
    unsafe {
        clear_level(into);
        if (*from).num_syms as i32 > 1 as i32 {
            (*into).s.syms = _steal(&raw mut (*from).s.syms as *mut ::core::ffi::c_void)
                as *mut xkb_keysym_t as *mut xkb_keysym_t;
        } else {
            (*into).s.sym = (*from).s.sym;
        }
        (*into).num_syms = (*from).num_syms;
        (*from).num_syms = 0 as xkb_keysym_count_t;
        if (*from).num_actions as i32 > 1 as i32 {
            (*into).a.actions = _steal(&raw mut (*from).a.actions as *mut ::core::ffi::c_void)
                as *mut xkb_action as *mut xkb_action;
        } else {
            (*into).a.action = (*from).a.action;
        }
        (*into).num_actions = (*from).num_actions;
        (*from).num_actions = 0 as xkb_action_count_t;
    }
}
unsafe fn InitGroupInfo(mut groupi: *mut GroupInfo) {
    unsafe {
        std::ptr::write(
            groupi,
            GroupInfo {
                levels: Vec::new(),
                defined: 0 as group_field,
                type_0: 0,
            },
        );
    }
}
unsafe fn ClearGroupInfo(mut groupi: *mut GroupInfo) {
    unsafe {
        for leveli in (*groupi).levels.iter_mut() {
            clear_level(leveli as *mut xkb_level);
        }
        (*groupi).levels.clear();
    }
}
unsafe fn CopyGroupInfo(mut to: *mut GroupInfo, mut from: *const GroupInfo) {
    unsafe {
        (*to).defined = (*from).defined;
        (*to).type_0 = (*from).type_0;
        (*to).levels = Vec::new();
        if !(*from).levels.is_empty() {
            (*to).levels.extend_from_slice(&(*from).levels);
        }
        let mut j: xkb_level_index_t = 0 as xkb_level_index_t;
        while j < (&(*to).levels).len() as xkb_level_index_t {
            if (&(*from).levels)[j as usize].num_syms as i32 > 1 as i32 {
                let ref mut c2rust_fresh0 = (&mut (*to).levels)[j as usize].s.syms;
                *c2rust_fresh0 = memdup(
                    (&(*from).levels)[j as usize].s.syms as *const ::core::ffi::c_void,
                    (&(*from).levels)[j as usize].num_syms as usize,
                    std::mem::size_of::<xkb_keysym_t>(),
                ) as *mut xkb_keysym_t;
            }
            if (&(*from).levels)[j as usize].num_actions as i32 > 1 as i32 {
                let ref mut c2rust_fresh1 = (&mut (*to).levels)[j as usize].a.actions;
                *c2rust_fresh1 = memdup(
                    (&(*from).levels)[j as usize].a.actions as *const ::core::ffi::c_void,
                    (&(*from).levels)[j as usize].num_actions as usize,
                    std::mem::size_of::<xkb_action>(),
                ) as *mut xkb_action;
            }
            j = j.wrapping_add(1);
        }
    }
}
unsafe fn InitKeyInfo(mut ctx: *mut xkb_context, mut keyi: *mut KeyInfo) {
    unsafe {
        std::ptr::write_bytes::<KeyInfo>(keyi as *mut KeyInfo, 0u8, 1);
        (*keyi).name = xkb_atom_intern(
            ctx,
            b"*\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 2]>()).wrapping_sub(1 as usize),
        );
        (*keyi).set_out_of_range_group_policy(
            XKB_LAYOUT_OUT_OF_RANGE_WRAP as xkb_layout_out_of_range_policy,
        );
        std::ptr::write(&raw mut (*keyi).groups, Vec::new());
        (*keyi).c2rust_unnamed.overlay_key = std::ptr::null();
    }
}
unsafe fn ClearKeyInfo(mut keyi: *mut KeyInfo) {
    unsafe {
        for groupi in (*keyi).groups.iter_mut() {
            ClearGroupInfo(groupi as *mut GroupInfo);
        }
        (*keyi).groups.clear();
        if (*keyi).overlays_alloc != 0 {
            free((*keyi).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void);
        }
    }
}
unsafe fn InitSymbolsInfo(
    mut info: *mut SymbolsInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        std::ptr::write_bytes::<SymbolsInfo>(info as *mut SymbolsInfo, 0u8, 1);
        std::ptr::write(&raw mut (*info).keys, Vec::new());
        std::ptr::write(&raw mut (*info).group_names, Vec::new());
        std::ptr::write(&raw mut (*info).modmaps, Vec::new());
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).include_depth = include_depth;
        (*info).keymap_info = keymap_info;
        (*info).max_groups = (*keymap_info).features.max_groups;
        InitKeyInfo((*keymap_info).keymap.ctx, &raw mut (*info).default_key);
        InitActionsInfo(
            &raw const (*keymap_info).keymap,
            &raw mut (*info).default_actions,
        );
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0 as u32);
        (*info).explicit_group = XKB_LAYOUT_INVALID as xkb_layout_index_t;
    }
}
unsafe fn ClearSymbolsInfo(mut info: *mut SymbolsInfo) {
    unsafe {
        cstr_free((*info).name);
        for keyi in (*info).keys.iter_mut() {
            ClearKeyInfo(keyi as *mut KeyInfo);
        }
        (*info).keys.clear();
        (*info).group_names.clear();
        (*info).modmaps.clear();
        ClearKeyInfo(&raw mut (*info).default_key);
    }
}
unsafe fn KeyInfoText(mut info: *mut SymbolsInfo, mut keyi: *mut KeyInfo) -> *const i8 {
    unsafe {
        return KeyNameText((*info).ctx, (*keyi).name);
    }
}
unsafe fn MergeGroups(
    mut info: *mut SymbolsInfo,
    mut into: *mut GroupInfo,
    mut from: *mut GroupInfo,
    mut clobber: bool,
    mut report: bool,
    mut group: xkb_layout_index_t,
    mut key_name: xkb_atom_t,
) -> bool {
    unsafe {
        if (*into).type_0 != (*from).type_0 {
            if !((*from).type_0 == XKB_ATOM_NONE as xkb_atom_t) {
                if (*into).type_0 == XKB_ATOM_NONE as xkb_atom_t {
                    (*into).type_0 = (*from).type_0;
                } else {
                    let mut use_0: xkb_atom_t = if clobber as i32 != 0 {
                        (*from).type_0
                    } else {
                        (*into).type_0
                    };
                    let mut ignore: xkb_atom_t = if clobber as i32 != 0 {
                        (*into).type_0
                    } else {
                        (*from).type_0
                    };
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Multiple definitions for group {} type of key {}; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS
                                as i32,
                            group.wrapping_add(1 as xkb_layout_index_t),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, key_name)),
                            crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, use_0)),
                            crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, ignore)),
                        );
                    }
                    (*into).type_0 = use_0;
                }
            }
        }
        (*into).defined = ((*into).defined as u32
            | (*from).defined as u32 & GROUP_FIELD_TYPE as u32)
            as group_field;
        if (*from).levels.len() == 0 {
            InitGroupInfo(from);
            return true;
        }
        if (*into).levels.len() == 0 {
            (*from).type_0 = (*into).type_0;
            std::ptr::write(into, std::ptr::read(from));
            InitGroupInfo(from);
            return true;
        }
        let levels_in_both: darray_size_t = if (*into).levels.len() < (*from).levels.len() {
            (*into).levels.len()
        } else {
            (*from).levels.len()
        } as darray_size_t;
        let mut fromKeysymsCount: darray_size_t = 0 as darray_size_t;
        let mut fromActionsCount: darray_size_t = 0 as darray_size_t;
        let mut i: darray_size_t = 0 as darray_size_t;
        while i < levels_in_both {
            let intoLevel: *mut xkb_level =
                &mut (&mut (*into).levels)[i as usize] as *mut xkb_level;
            let fromLevel: *mut xkb_level =
                &mut (&mut (*from).levels)[i as usize] as *mut xkb_level;
            let fromHasNoKeysym: bool = (*fromLevel).num_syms as i32 == 0 as i32;
            let fromHasNoAction: bool = (*fromLevel).num_actions as i32 == 0 as i32;
            if !(fromHasNoKeysym as i32 != 0 && fromHasNoAction as i32 != 0) {
                let intoHasNoKeysym: bool = (*intoLevel).num_syms as i32 == 0 as i32;
                let intoHasNoAction: bool = (*intoLevel).num_actions as i32 == 0 as i32;
                if intoHasNoKeysym as i32 != 0 && intoHasNoAction as i32 != 0 {
                    StealLevelInfo(intoLevel, fromLevel);
                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                    fromActionsCount = fromActionsCount.wrapping_add(1);
                } else {
                    if !XkbLevelsSameSyms(fromLevel, intoLevel) {
                        if report as i32 != 0
                            && !(intoHasNoKeysym as i32 != 0 || fromHasNoKeysym as i32 != 0)
                        {
                            xkb_logf!(
                                (*info).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Multiple symbols for level {}/group {} on key {}; Using {}, ignoring {}\n",
                                XKB_WARNING_CONFLICTING_KEY_SYMBOL as i32,
                                i.wrapping_add(1 as darray_size_t),
                                group.wrapping_add(1 as xkb_layout_index_t),
                                crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, key_name)),
                                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                                    b"from\0".as_ptr() as *const i8
                                } else {
                                    b"to\0".as_ptr() as *const i8
                                }),
                                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                                    b"to\0".as_ptr() as *const i8
                                } else {
                                    b"from\0".as_ptr() as *const i8
                                }),
                            );
                        }
                        if !fromHasNoKeysym {
                            if clobber {
                                if ((*fromLevel).num_syms as i32 > 1 as i32) as i64 != 0 {
                                    if ((*intoLevel).num_syms as i32 > 1 as i32) as i32 as i64 != 0
                                    {
                                        free((*intoLevel).s.syms as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).s.syms = (*fromLevel).s.syms;
                                    (*intoLevel).num_syms = (*fromLevel).num_syms;
                                    (*fromLevel).num_syms = 0 as xkb_keysym_count_t;
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                } else if (*fromLevel).s.sym != XKB_KEY_NoSymbol as xkb_keysym_t {
                                    if ((*intoLevel).num_syms as i32 > 1 as i32) as i32 as i64 != 0
                                    {
                                        free((*intoLevel).s.syms as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).s.sym = (*fromLevel).s.sym;
                                    (*intoLevel).num_syms = 1 as xkb_keysym_count_t;
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                }
                            } else if !(((*intoLevel).num_syms as i32 > 1 as i32) as i32 as i64
                                != 0)
                            {
                                if (*intoLevel).s.sym == XKB_KEY_NoSymbol as xkb_keysym_t {
                                    if ((*fromLevel).num_syms as i32 > 1 as i32) as i32 as i64 != 0
                                    {
                                        (*intoLevel).s.syms = (*fromLevel).s.syms;
                                        (*intoLevel).num_syms = (*fromLevel).num_syms;
                                        (*fromLevel).num_syms = 0 as xkb_keysym_count_t;
                                    } else {
                                        (*intoLevel).s.sym = (*fromLevel).s.sym;
                                        (*intoLevel).num_syms = 1 as xkb_keysym_count_t;
                                    }
                                    fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
                                }
                            }
                        }
                    }
                    if !XkbLevelsSameActions(intoLevel, fromLevel) {
                        if report as i32 != 0
                            && !(intoHasNoAction as i32 != 0 || fromHasNoAction as i32 != 0)
                        {
                            if ((*intoLevel).num_actions as i32 > 1 as i32) as i64 != 0 {
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] Multiple actions for level {}/group {} on key {}; {}\n",
                                    XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                                    i.wrapping_add(1 as darray_size_t),
                                    group.wrapping_add(1 as xkb_layout_index_t),
                                    crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, key_name)),
                                    crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                                        b"Using from, ignoring to\0".as_ptr()
                                            as *const i8
                                    } else {
                                        b"Using to, ignoring from\0".as_ptr()
                                            as *const i8
                                    }),
                                );
                            } else {
                                let mut use_1: *mut xkb_action = std::ptr::null_mut();
                                let mut ignore_0: *mut xkb_action = std::ptr::null_mut();
                                use_1 = if clobber as i32 != 0 {
                                    &raw mut (*fromLevel).a.action
                                } else {
                                    &raw mut (*intoLevel).a.action
                                };
                                ignore_0 = if clobber as i32 != 0 {
                                    &raw mut (*intoLevel).a.action
                                } else {
                                    &raw mut (*fromLevel).a.action
                                };
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] Multiple actions for level {}/group {} on key {}; Using {}, ignoring {}\n",
                                    XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                                    i.wrapping_add(1 as darray_size_t),
                                    group.wrapping_add(1 as xkb_layout_index_t),
                                    crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, key_name)),
                                    crate::xkb::utils::CStrDisplay(ActionTypeText((*use_1).type_0)),
                                    crate::xkb::utils::CStrDisplay(ActionTypeText((*ignore_0).type_0)),
                                );
                            }
                        }
                        if !fromHasNoAction {
                            if clobber {
                                if ((*fromLevel).num_actions as i32 > 1 as i32) as i64 != 0 {
                                    if ((*intoLevel).num_actions as i32 > 1 as i32) as i32 as i64
                                        != 0
                                    {
                                        free((*intoLevel).a.actions as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).a.actions = (*fromLevel).a.actions;
                                    (*intoLevel).num_actions = (*fromLevel).num_actions;
                                    (*fromLevel).num_actions = 0 as xkb_action_count_t;
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                } else if (*fromLevel).a.action.type_0 as u32
                                    != ACTION_TYPE_NONE as u32
                                {
                                    if ((*intoLevel).num_actions as i32 > 1 as i32) as i32 as i64
                                        != 0
                                    {
                                        free((*intoLevel).a.actions as *mut ::core::ffi::c_void);
                                    }
                                    (*intoLevel).a.action = (*fromLevel).a.action;
                                    (*intoLevel).num_actions = 1 as xkb_action_count_t;
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                }
                            } else if !(((*intoLevel).num_actions as i32 > 1 as i32) as i32 as i64
                                != 0)
                            {
                                if (*intoLevel).a.action.type_0 as u32 == ACTION_TYPE_NONE as u32 {
                                    if ((*fromLevel).num_actions as i32 > 1 as i32) as i32 as i64
                                        != 0
                                    {
                                        (*intoLevel).a.actions = (*fromLevel).a.actions;
                                        (*intoLevel).num_actions = (*fromLevel).num_actions;
                                        (*fromLevel).num_actions = 0 as xkb_action_count_t;
                                    } else {
                                        (*intoLevel).a.action = (*fromLevel).a.action;
                                        (*intoLevel).num_actions = 1 as xkb_action_count_t;
                                    }
                                    fromActionsCount = fromActionsCount.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let mut level_idx: darray_size_t = levels_in_both;
        while level_idx < (&(*from).levels).len() as darray_size_t {
            let level_val = (&(*from).levels)[level_idx as usize];
            (&mut (*into).levels).push(level_val);
            (&mut (*from).levels)[level_idx as usize].num_syms = 0 as xkb_keysym_count_t;
            (&mut (*from).levels)[level_idx as usize].num_actions = 0 as xkb_action_count_t;
            fromKeysymsCount = fromKeysymsCount.wrapping_add(1);
            fromActionsCount = fromActionsCount.wrapping_add(1);
            level_idx = level_idx.wrapping_add(1);
        }
        if fromKeysymsCount != 0 {
            if fromKeysymsCount == (*into).levels.len() as darray_size_t {
                (*into).defined =
                    ((*into).defined as u32 & !(GROUP_FIELD_SYMS as i32) as u32) as group_field;
            }
            (*into).defined = ((*into).defined as u32
                | (*from).defined as u32 & GROUP_FIELD_SYMS as u32)
                as group_field;
        }
        if fromActionsCount != 0 {
            if fromActionsCount == (*into).levels.len() as darray_size_t {
                (*into).defined =
                    ((*into).defined as u32 & !(GROUP_FIELD_ACTS as i32) as u32) as group_field;
            }
            (*into).defined = ((*into).defined as u32
                | (*from).defined as u32 & GROUP_FIELD_ACTS as u32)
                as group_field;
        }
        return true;
    }
}
unsafe fn UseNewKeyField(
    mut field: key_field,
    mut old: key_field,
    mut new: key_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut key_field,
) -> bool {
    unsafe {
        if old as u32 & field as u32 == 0 {
            return new as u32 & field as u32 != 0;
        }
        if new as u32 & field as u32 != 0 {
            if report {
                *collide = (*collide as u32 | field as u32) as key_field;
            }
            return clobber;
        }
        return false;
    }
}
unsafe fn overlays_get(
    mut info: *const KeyInfo,
    mut bit: xkb_overlay_index_t,
    mut key_out: *mut *const xkb_key,
) -> bool {
    unsafe {
        if bit as i32
            >= (std::mem::size_of::<xkb_overlay_mask_t>()).wrapping_mul(8 as usize)
                as xkb_overlay_index_t as i32
        {
            return false;
        }
        let mask: xkb_overlay_mask_t = ((1 as u32) << bit as i32) as xkb_overlay_mask_t;
        if (*info).overlays as i32 & mask as i32 == 0 {
            return false;
        }
        if !key_out.is_null() {
            if (*info).overlays_alloc == 0 {
                *key_out = (*info).c2rust_unnamed.overlay_key;
            } else {
                let low: xkb_overlay_mask_t = ((*info).overlays as u32
                    & (mask as u32).wrapping_sub(1 as u32))
                    as xkb_overlay_mask_t;
                let index: xkb_overlay_index_t = popcount32(low as u32) as xkb_overlay_index_t;
                *key_out = *(*info).c2rust_unnamed.overlays_keys.offset(index as isize);
            }
        }
        return true;
    }
}
unsafe fn overlays_insert(
    mut keyi: *mut KeyInfo,
    mut bit: xkb_overlay_index_t,
    mut key: *const xkb_key,
) -> bool {
    unsafe {
        if bit as i32
            >= (std::mem::size_of::<xkb_overlay_mask_t>()).wrapping_mul(8 as usize)
                as xkb_overlay_index_t as i32
        {
            return false;
        }
        let mask: xkb_overlay_mask_t = ((1 as u32) << bit as i32) as xkb_overlay_mask_t;
        if (*keyi).overlays as i32 & mask as i32 != 0 && !(*keyi).overlays_clear() {
            if (*keyi).overlays_alloc == 0 {
                (*keyi).c2rust_unnamed.overlay_key = key;
                if key.is_null() {
                    (*keyi).set_overlays_clear((true) as bool);
                }
            } else {
                let low: xkb_overlay_mask_t = ((*keyi).overlays as i32
                    & (mask as u32).wrapping_sub(1 as u32) as xkb_overlay_mask_t as i32)
                    as xkb_overlay_mask_t;
                let index: xkb_overlay_index_t = popcount32(low as u32) as xkb_overlay_index_t;
                let ref mut c2rust_fresh4 =
                    *(*keyi).c2rust_unnamed.overlays_keys.offset(index as isize);
                *c2rust_fresh4 = key;
            }
            return true;
        }
        if (*keyi).overlays == 0 || (*keyi).overlays_clear() as i32 != 0 && key.is_null() {
            (*keyi).c2rust_unnamed.overlay_key = key;
            (*keyi).overlays = ((*keyi).overlays as i32 | mask as i32) as xkb_overlay_mask_t;
            (*keyi).set_overlays_clear(key.is_null() as bool);
        } else if (*keyi).overlays_alloc == 0 {
            let overlays: xkb_overlay_mask_t =
                ((*keyi).overlays as i32 | mask as i32) as xkb_overlay_mask_t;
            let alloc: xkb_overlay_index_t = popcount32(overlays as u32) as xkb_overlay_index_t;
            let tmp: *mut *const xkb_key =
                calloc(alloc as usize, std::mem::size_of::<*const xkb_key>())
                    as *mut *const xkb_key;
            if tmp.is_null() {
                return false;
            }
            if !(*keyi).overlays_clear() {
                let low_0: xkb_overlay_mask_t = (overlays as i32
                    & ((*keyi).overlays as u32).wrapping_sub(1 as u32) as xkb_overlay_mask_t as i32)
                    as xkb_overlay_mask_t;
                let idx: xkb_overlay_index_t = popcount32(low_0 as u32) as xkb_overlay_index_t;
                let ref mut c2rust_fresh5 = *tmp.offset(idx as isize);
                *c2rust_fresh5 = (*keyi).c2rust_unnamed.overlay_key;
            }
            let low_1: xkb_overlay_mask_t = (overlays as i32
                & (mask as u32).wrapping_sub(1 as u32) as xkb_overlay_mask_t as i32)
                as xkb_overlay_mask_t;
            let idx_0: xkb_overlay_index_t = popcount32(low_1 as u32) as xkb_overlay_index_t;
            let ref mut c2rust_fresh6 = *tmp.offset(idx_0 as isize);
            *c2rust_fresh6 = key;
            (*keyi).c2rust_unnamed.overlays_keys = tmp;
            (*keyi).overlays_alloc = alloc;
            (*keyi).overlays = overlays;
            (*keyi).set_overlays_clear((false) as bool);
        } else {
            let overlays_0: xkb_overlay_mask_t =
                ((*keyi).overlays as i32 | mask as i32) as xkb_overlay_mask_t;
            let count: xkb_overlay_index_t = popcount32(overlays_0 as u32) as xkb_overlay_index_t;
            if count as i32 > (*keyi).overlays_alloc as i32 {
                let alloc_0: xkb_overlay_index_t = next_pow2(count as u32) as xkb_overlay_index_t;
                let tmp_0: *mut *const xkb_key = realloc(
                    (*keyi).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void,
                    (alloc_0 as usize).wrapping_mul(std::mem::size_of::<*const xkb_key>()),
                ) as *mut *const xkb_key;
                if tmp_0.is_null() {
                    return false;
                }
                (*keyi).c2rust_unnamed.overlays_keys = tmp_0;
                (*keyi).overlays_alloc = alloc_0;
            }
            let low_2: xkb_overlay_mask_t = (overlays_0 as i32
                & (mask as u32).wrapping_sub(1 as u32) as xkb_overlay_mask_t as i32)
                as xkb_overlay_mask_t;
            let index_0: xkb_overlay_index_t = popcount32(low_2 as u32) as xkb_overlay_index_t;
            if index_0 as i32 >= (*keyi).overlays_alloc as i32 {
                eprintln!(
                    "Critical Error: Reached unreachable line in {} at {}",
                    "../src/xkbcomp/symbols.c", 654
                );
                abort();
            }
            if (index_0 as u32) < (count as u32).wrapping_sub(1 as u32) {
                std::ptr::copy(
                    (*keyi)
                        .c2rust_unnamed
                        .overlays_keys
                        .offset(index_0 as i32 as isize),
                    (*keyi)
                        .c2rust_unnamed
                        .overlays_keys
                        .offset(index_0 as i32 as isize)
                        .offset(1 as i32 as isize),
                    (count as u32)
                        .wrapping_sub(1 as u32)
                        .wrapping_sub(index_0 as u32) as usize,
                );
            }
            let ref mut c2rust_fresh7 = *(*keyi)
                .c2rust_unnamed
                .overlays_keys
                .offset(index_0 as isize);
            *c2rust_fresh7 = key;
            (*keyi).overlays = overlays_0;
        }
        return true;
    }
}
unsafe fn merge_overlays(
    mut info: *mut SymbolsInfo,
    mut into: *mut KeyInfo,
    mut from: *mut KeyInfo,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut key_field,
) -> bool {
    unsafe {
        if !((*from).defined() as i32 & KEY_FIELD_OVERLAY as i32 == 0) {
            if (*into).defined() as i32 & KEY_FIELD_OVERLAY as i32 == 0 {
                (*into).overlays = (*from).overlays;
                (*into).c2rust_unnamed.overlays_keys = (*from).c2rust_unnamed.overlays_keys;
                (*from).c2rust_unnamed.overlays_keys = std::ptr::null_mut();
                (*into).overlays_alloc = (*from).overlays_alloc;
                (*from).overlays_alloc = 0 as xkb_overlay_index_t;
                (*into).set_defined((*into).defined() | KEY_FIELD_OVERLAY as i32 as key_field);
            } else if (*into).overlays_clear() as i32 != 0 && (*from).overlays_clear() as i32 != 0 {
                (*into).overlays =
                    ((*into).overlays as i32 | (*from).overlays as i32) as xkb_overlay_mask_t;
            } else if (*(*info).keymap_info).features.overlapping_overlays {
                let result_mask: xkb_overlay_mask_t =
                    ((*into).overlays as i32 | (*from).overlays as i32) as xkb_overlay_mask_t;
                let count: xkb_overlay_index_t =
                    popcount32(result_mask as u32) as xkb_overlay_index_t;
                if count as i32 == 0 as i32 {
                    eprintln!(
                        "Critical Error: Reached unreachable line in {} at {}",
                        "../src/xkbcomp/symbols.c", 696
                    );
                    abort();
                }
                let mut dest: *mut KeyInfo = into;
                let mut src: *mut KeyInfo = from;
                if !((*into).overlays_alloc as i32 >= count as i32
                    && (*into).overlays_alloc as i32 >= (*from).overlays_alloc as i32)
                {
                    if (*from).overlays_alloc as i32 >= count as i32 {
                        dest = from;
                        src = into;
                        clobber = !clobber;
                    } else if count as i32 > 1 as i32 {
                        if ((*into).overlays_alloc as i32) < (*from).overlays_alloc as i32 {
                            dest = from;
                            src = into;
                            clobber = !clobber;
                        }
                        if (*dest).overlays_alloc == 0 {
                            let mut tmp: *mut *const xkb_key =
                                calloc(count as usize, std::mem::size_of::<*const xkb_key>())
                                    as *mut *const xkb_key;
                            if tmp.is_null() {
                                return false;
                            }
                            if !(*dest).c2rust_unnamed.overlay_key.is_null() {
                                let ref mut c2rust_fresh2 = *tmp.offset(0 as i32 as isize);
                                *c2rust_fresh2 = (*dest).c2rust_unnamed.overlay_key;
                            }
                            (*dest).c2rust_unnamed.overlays_keys = tmp;
                            (*dest).overlays_alloc = count;
                        } else {
                            let mut tmp_0: *mut *const xkb_key = realloc(
                                (*dest).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void,
                                (count as usize)
                                    .wrapping_mul(std::mem::size_of::<*const xkb_key>() as usize),
                            )
                                as *mut *const xkb_key;
                            if tmp_0.is_null() {
                                return false;
                            }
                            (*dest).c2rust_unnamed.overlays_keys = tmp_0;
                            (*dest).overlays_alloc = count;
                        }
                    }
                }
                let mut remaining: xkb_overlay_mask_t = (*src).overlays;
                let mut src_keys: *mut *const xkb_key = if (*src).overlays_clear() as i32 != 0 {
                    std::ptr::null_mut()
                } else if (*src).overlays_alloc as i32 != 0 {
                    (*src).c2rust_unnamed.overlays_keys
                } else {
                    &raw mut (*src).c2rust_unnamed.overlay_key
                };
                while remaining != 0 {
                    let lsb: xkb_overlay_mask_t = (remaining as i32
                        & (!(remaining as i32) as u32).wrapping_add(1 as u32) as xkb_overlay_mask_t
                            as i32)
                        as xkb_overlay_mask_t;
                    let bit: xkb_overlay_index_t =
                        popcount32((lsb as u32).wrapping_sub(1 as u32)) as xkb_overlay_index_t;
                    remaining = (remaining as i32 & !(lsb as i32)) as xkb_overlay_mask_t;
                    if !(*src).overlays_clear()
                        && (*src).overlays_alloc == 0
                        && remaining as i32 != 0
                    {
                        eprintln!(
                            "Critical Error: Reached unreachable line in {} at {}",
                            "../src/xkbcomp/symbols.c", 758
                        );
                        abort();
                    }
                    let mut src_key: *const xkb_key = if !src_keys.is_null() {
                        let c2rust_fresh3 = src_keys;
                        src_keys = src_keys.offset(1);
                        *c2rust_fresh3
                    } else {
                        std::ptr::null()
                    };
                    let mut dest_key: *const xkb_key = std::ptr::null();
                    let conflict: bool = overlays_get(dest, bit, &raw mut dest_key) as bool;
                    if conflict {
                        if dest_key == src_key {
                            continue;
                        }
                        if report {
                            *collide = (*collide as u32 | KEY_FIELD_OVERLAY as u32) as key_field;
                        }
                    }
                    if (!conflict || clobber as i32 != 0) && !overlays_insert(dest, bit, src_key) {
                        return false;
                    }
                }
                if into != dest {
                    if (*into).overlays_alloc != 0 {
                        free((*into).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void);
                    }
                    (*into).overlays = (*from).overlays;
                    (*into).set_overlays_clear((*from).overlays_clear() as bool);
                    (*into).overlays_alloc = (*from).overlays_alloc;
                    if (*from).overlays_alloc != 0 {
                        (*into).c2rust_unnamed.overlays_keys = (*from).c2rust_unnamed.overlays_keys;
                        (*from).c2rust_unnamed.overlays_keys = std::ptr::null_mut();
                        (*from).overlays_alloc = 0 as xkb_overlay_index_t;
                    } else {
                        (*into).c2rust_unnamed.overlay_key = (*from).c2rust_unnamed.overlay_key;
                    }
                }
            } else {
                if (*into).overlays as i32 == (*from).overlays as i32
                    && (*into).overlays_clear() as i32 == (*from).overlays_clear() as i32
                    && (*into).c2rust_unnamed.overlay_key == (*from).c2rust_unnamed.overlay_key
                {
                    return true;
                }
                if (*into).overlays as i32 & (*from).overlays as i32 == 0 {
                    if (*into).overlays_clear() {
                        (*into).overlays = (*from).overlays;
                        (*into).set_overlays_clear((*from).overlays_clear() as bool);
                        (*into).overlays_alloc = (*from).overlays_alloc;
                        (*into).c2rust_unnamed.overlay_key = (*from).c2rust_unnamed.overlay_key;
                        return true;
                    } else if (*from).overlays_clear() {
                        return true;
                    }
                }
                if report {
                    *collide = (*collide as u32 | KEY_FIELD_OVERLAY as u32) as key_field;
                }
                if clobber {
                    (*into).overlays = (*from).overlays;
                    (*into).set_overlays_clear((*from).overlays_clear() as bool);
                    (*into).overlays_alloc = (*from).overlays_alloc;
                    (*into).c2rust_unnamed.overlay_key = (*from).c2rust_unnamed.overlay_key;
                }
            }
        }
        return true;
    }
}
unsafe fn MergeKeys(
    mut info: *mut SymbolsInfo,
    mut into: *mut KeyInfo,
    mut from: *mut KeyInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut i: xkb_layout_index_t = 0;
        let mut groups_in_both: xkb_layout_index_t = 0;
        let mut collide: key_field = 0 as key_field;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let clobber: bool = (*from).merge() as i32 != MERGE_AUGMENT as i32;
        let report: bool = same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32;
        if (*from).merge() as i32 == MERGE_REPLACE as i32 {
            ClearKeyInfo(into);
            std::ptr::write(into, std::ptr::read(from));
            InitKeyInfo((*info).ctx, from);
            return true;
        }
        groups_in_both = (if (*into).groups.len() < (*from).groups.len() {
            (*into).groups.len()
        } else {
            (*from).groups.len()
        }) as xkb_layout_index_t;
        i = 0 as xkb_layout_index_t;
        while i < groups_in_both {
            MergeGroups(
                info,
                &mut (&mut (*into).groups)[i as usize] as *mut GroupInfo,
                &mut (&mut (*from).groups)[i as usize] as *mut GroupInfo,
                clobber,
                report,
                i,
                (*into).name,
            );
            i = i.wrapping_add(1);
        }
        i = groups_in_both;
        while i < (&(*from).groups).len() as xkb_layout_index_t {
            let group_val = std::ptr::read(&(&(*from).groups)[i as usize]);
            (&mut (*into).groups).push(group_val);
            InitGroupInfo(&mut (&mut (*from).groups)[i as usize] as *mut GroupInfo);
            i = i.wrapping_add(1);
        }
        if UseNewKeyField(
            KEY_FIELD_VMODMAP,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).vmodmap = (*from).vmodmap;
            (*into).set_defined((*into).defined() | KEY_FIELD_VMODMAP as i32 as key_field);
        }
        if UseNewKeyField(
            KEY_FIELD_REPEAT,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).set_repeat((*from).repeat() as key_repeat);
            (*into).set_defined((*into).defined() | KEY_FIELD_REPEAT as i32 as key_field);
        }
        if UseNewKeyField(
            KEY_FIELD_DEFAULT_TYPE,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).default_type = (*from).default_type;
            (*into).set_defined((*into).defined() | KEY_FIELD_DEFAULT_TYPE as i32 as key_field);
        }
        if UseNewKeyField(
            KEY_FIELD_GROUPINFO,
            (*into).defined(),
            (*from).defined(),
            clobber,
            report,
            &raw mut collide,
        ) {
            (*into).set_out_of_range_pending_group((*from).out_of_range_pending_group() as bool);
            (*into).set_out_of_range_group_policy(
                (*from).out_of_range_group_policy() as xkb_layout_out_of_range_policy
            );
            (*into).out_of_range_group_number = (*from).out_of_range_group_number;
            (*into).set_defined((*into).defined() | KEY_FIELD_GROUPINFO as i32 as key_field);
        }
        if !merge_overlays(info, into, from, clobber, report, &raw mut collide) {
            return false;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Symbol map for key {} redefined; Using {} definition for conflicting fields\n",
                XKB_WARNING_CONFLICTING_KEY_FIELDS as i32,
                crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*into).name)),
                crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                    b"first\0".as_ptr() as *const i8
                } else {
                    b"last\0".as_ptr() as *const i8
                }),
            );
        }
        ClearKeyInfo(from);
        InitKeyInfo((*info).ctx, from);
        return true;
    }
}
unsafe fn XkbResolveKeyAlias(mut keymap: *const xkb_keymap, mut name: xkb_atom_t) -> xkb_atom_t {
    unsafe {
        if name < (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names {
            let match_0: KeycodeMatch = *(*keymap)
                .c2rust_unnamed
                .c2rust_unnamed
                .key_names
                .offset(name as isize);
            if match_0.c2rust_unnamed.found() as i32 != 0
                && match_0.c2rust_unnamed.is_alias() as i32 != 0
            {
                return match_0.alias.real();
            }
        }
        return name;
    }
}
unsafe fn AddKeySymbols(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        (*keyi).name = XkbResolveKeyAlias(&raw const (*(*info).keymap_info).keymap, (*keyi).name);
        for iter in (*info).keys.iter_mut() {
            if iter.name == (*keyi).name {
                return MergeKeys(info, iter as *mut KeyInfo, keyi, same_file);
            }
        }
        (*info).keys.push(std::ptr::read(keyi));
        InitKeyInfo((*info).ctx, keyi);
        return true;
    }
}
unsafe fn AddModMapEntry(mut info: *mut SymbolsInfo, mut new: *mut ModMapEntry) -> bool {
    unsafe {
        let mut clobber: bool = (*new).merge as u32 != MERGE_AUGMENT as u32;
        for old in (*info).modmaps.iter_mut() {
            if (*new).haveSymbol as i32 != old.haveSymbol as i32
                || (*new).haveSymbol as i32 != 0 && (*new).u.keySym != old.u.keySym
                || !(*new).haveSymbol && (*new).u.keyName != old.u.keyName
            {
                continue;
            }
            if (*new).modifier == old.modifier {
                return true;
            }
            let use_0: xkb_mod_index_t = if clobber as i32 != 0 {
                (*new).modifier
            } else {
                old.modifier
            };
            let ignore: xkb_mod_index_t = if clobber as i32 != 0 {
                old.modifier
            } else {
                (*new).modifier
            };
            if (*new).haveSymbol {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Symbol \"{}\" added to modifier map for multiple modifiers; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_MODMAP as i32,
                    crate::xkb::utils::CStrDisplay(KeysymText((*info).ctx, (*new).u.keySym)),
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, use_0)),
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, ignore)),
                );
            } else {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Key \"{}\" added to modifier map for multiple modifiers; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_MODMAP as i32,
                    crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*new).u.keyName)),
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, use_0)),
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, ignore)),
                );
            }
            old.modifier = use_0;
            return true;
        }
        (*info).modmaps.push(*new);
        return true;
    }
}
unsafe fn MergeIncludedSymbols(
    mut into: *mut SymbolsInfo,
    mut from: *mut SymbolsInfo,
    mut merge: merge_mode,
) {
    unsafe {
        let mut group_names_in_both: xkb_layout_index_t = 0;
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
        group_names_in_both = (if (&(*into).group_names).len() < (&(*from).group_names).len() {
            (&(*into).group_names).len()
        } else {
            (&(*from).group_names).len()
        }) as xkb_layout_index_t;
        let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while i < group_names_in_both {
            if !((&(*from).group_names)[i as usize] == 0) {
                if !(merge as u32 == MERGE_AUGMENT as u32
                    && (&(*into).group_names)[i as usize] != 0)
                {
                    (&mut (*into).group_names)[i as usize] = (&(*from).group_names)[i as usize];
                }
            }
            i = i.wrapping_add(1);
        }
        if group_names_in_both < (&(*from).group_names).len() as xkb_layout_index_t {
            for gn_idx in group_names_in_both as usize..(&(*from).group_names).len() {
                (&mut (*into).group_names).push((&(*from).group_names)[gn_idx]);
            }
        }
        if (*into).keys.is_empty() {
            std::mem::swap(&mut (*into).keys, &mut (*from).keys);
        } else {
            for keyi in (*from).keys.iter_mut() {
                keyi.set_merge(merge as merge_mode);
                if !AddKeySymbols(into, keyi as *mut KeyInfo, false) {
                    (*into).errorCount += 1;
                }
            }
        }
        if (*into).modmaps.is_empty() {
            std::mem::swap(&mut (*into).modmaps, &mut (*from).modmaps);
        } else {
            for mm in (*from).modmaps.iter_mut() {
                mm.merge = merge;
                if !AddModMapEntry(into, mm as *mut ModMapEntry) {
                    (*into).errorCount += 1;
                }
            }
        };
    }
}
unsafe fn HandleIncludeSymbols(mut info: *mut SymbolsInfo, mut include: *mut IncludeStmt) -> bool {
    unsafe {
        let mut included: SymbolsInfo = SymbolsInfo {
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: Vec::new(),
            default_key: KeyInfo {
                name: 0,
                vmodmap: 0,
                default_type: 0,
                out_of_range_group_number: 0,
                groups: Vec::new(),
                out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                overlays_alloc: 0,
                overlays: 0,
                c2rust_unnamed: C2Rust_Unnamed_21 {
                    overlay_key: std::ptr::null(),
                },
            },
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            group_names: Vec::new(),
            modmaps: Vec::new(),
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: std::ptr::null_mut(),
            keymap_info: std::ptr::null(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as i32;
            return false;
        }
        InitSymbolsInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as u32),
            &raw mut (*info).mods,
        );
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: SymbolsInfo = SymbolsInfo {
                name: std::ptr::null_mut(),
                errorCount: 0,
                include_depth: 0,
                explicit_group: 0,
                max_groups: 0,
                keys: Vec::new(),
                default_key: KeyInfo {
                    name: 0,
                    vmodmap: 0,
                    default_type: 0,
                    out_of_range_group_number: 0,
                    groups: Vec::new(),
                    out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                    overlays_alloc: 0,
                    overlays: 0,
                    c2rust_unnamed: C2Rust_Unnamed_21 {
                        overlay_key: std::ptr::null(),
                    },
                },
                default_actions: ActionsInfo {
                    actions: [xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    }; 21],
                },
                group_names: Vec::new(),
                modmaps: Vec::new(),
                mods: xkb_mod_set {
                    mods: [xkb_mod {
                        name: 0,
                        type_0: 0 as mod_type,
                        mapping: 0,
                    }; 32],
                    num_mods: 0,
                    explicit_vmods: 0,
                },
                ctx: std::ptr::null_mut(),
                keymap_info: std::ptr::null(),
            };
            let mut file: *mut XkbFile = std::ptr::null_mut();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_SYMBOLS,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearSymbolsInfo(&raw mut included);
                return false;
            }
            InitSymbolsInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
                &raw mut included.mods,
            );
            if !(*stmt).modifier.is_null() {
                next_incl.explicit_group =
                    (atoi((*stmt).modifier) - 1 as i32) as xkb_layout_index_t;
                if next_incl.explicit_group >= (*info).max_groups {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Cannot set explicit group to {} - must be between 1..{}; Ignoring group number\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                        next_incl.explicit_group.wrapping_add(1 as xkb_layout_index_t),
                        (*info).max_groups,
                    );
                    next_incl.explicit_group = (*info).explicit_group;
                }
            } else if (*(*info).keymap_info).keymap.num_groups != 0 as xkb_layout_index_t
                && next_incl.include_depth == 1 as u32
            {
                next_incl.explicit_group = 0 as xkb_layout_index_t;
            } else {
                next_incl.explicit_group = (*info).explicit_group;
            }
            HandleSymbolsFile(&raw mut next_incl, file);
            MergeIncludedSymbols(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearSymbolsInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedSymbols(info, &raw mut included, (*include).merge);
        ClearSymbolsInfo(&raw mut included);
        return (*info).errorCount == 0 as i32;
    }
}
unsafe fn GetGroupIndex(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut field: group_field,
    mut ndx_rtrn: *mut xkb_layout_index_t,
) -> bool {
    unsafe {
        let mut name: *const i8 = if field as u32 == GROUP_FIELD_SYMS as u32 {
            b"symbols\0".as_ptr() as *const i8
        } else {
            b"actions\0".as_ptr() as *const i8
        };
        if arrayNdx.is_null() {
            let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
            let mut groupi: *mut GroupInfo = std::ptr::null_mut();
            if !(*keyi).groups.is_empty() {
                i = 0 as xkb_layout_index_t;
                groupi = (*keyi).groups.as_mut_ptr();
                while i < (*keyi).groups.len() as xkb_layout_index_t {
                    if (*groupi).defined as u32 & field as u32 == 0 {
                        *ndx_rtrn = i;
                        return true;
                    }
                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }
            if i >= (*info).max_groups {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Too many groups of {} for key {} (max {}); Ignoring {} defined for extra groups\n",
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                    crate::xkb::utils::CStrDisplay(name),
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                    (*info).max_groups,
                    crate::xkb::utils::CStrDisplay(name),
                );
                return false;
            }
            resize_groups_zero(&mut (*keyi).groups, (*keyi).groups.len().wrapping_add(1));
            *ndx_rtrn = (*keyi).groups.len().wrapping_sub(1) as xkb_layout_index_t;
            return true;
        }
        if ExprResolveGroup(
            (*info).keymap_info,
            arrayNdx,
            false,
            ndx_rtrn,
            std::ptr::null_mut(),
        ) as u32
            != PARSER_SUCCESS as u32
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Illegal group index for {} of key {}\nDefinition with non-integer array index ignored\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                crate::xkb::utils::CStrDisplay(name),
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }
        *ndx_rtrn = (*ndx_rtrn).wrapping_sub(1);
        if *ndx_rtrn >= (*keyi).groups.len() as xkb_layout_index_t {
            resize_groups_zero(&mut (*keyi).groups, (*ndx_rtrn).wrapping_add(1) as usize);
        }
        return true;
    }
}
unsafe fn AddSymbolsToKey(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ndx: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if !GetGroupIndex(info, keyi, arrayNdx, GROUP_FIELD_SYMS, &raw mut ndx) {
            return false;
        }
        let mut groupi: *mut GroupInfo = &mut (&mut (*keyi).groups)[ndx as usize] as *mut GroupInfo;
        if (*value).common.type_0 as u32 == STMT_EXPR_EMPTY_LIST as u32 {
            (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_SYMS as u32) as group_field;
            return true;
        }
        if (*value).common.type_0 as u32 != STMT_EXPR_KEYSYM_LIST as u32 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Expected a list of symbols, found {}; Ignoring symbols for group {} of {}\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(stmt_type_to_string((*value).common.type_0)),
                ndx.wrapping_add(1 as xkb_layout_index_t),
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }
        if (*groupi).defined as u32 & GROUP_FIELD_SYMS as u32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Symbols for key {}, group {} already defined; Ignoring duplicate definition\n",
                XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY as i32,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                ndx.wrapping_add(1 as xkb_layout_index_t),
            );
            return false;
        }
        let mut nLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut nonEmptyLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut keysymList: *mut ExprKeysymList = value as *mut ExprKeysymList;
        while !keysymList.is_null() {
            nLevels = nLevels.wrapping_add(1);
            if (*keysymList).syms.size > 0 as darray_size_t {
                nonEmptyLevels = nLevels;
            }
            keysymList = (*keysymList).common.next as *mut ExprKeysymList;
        }
        if nonEmptyLevels < nLevels {
            nLevels = nonEmptyLevels;
        }
        if ((*groupi).levels.len() as xkb_level_index_t) < nLevels {
            darray_resize_zero_vec(&mut (*groupi).levels, nLevels as usize);
        }
        (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_SYMS as u32) as group_field;
        let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut keysymList_0: *mut ExprKeysymList = value as *mut ExprKeysymList;
        while !keysymList_0.is_null() && level < nLevels {
            let mut leveli: *mut xkb_level =
                &mut (&mut (*groupi).levels)[level as usize] as *mut xkb_level;
            if ((*keysymList_0).syms.size > 65535 as darray_size_t) as i64 != 0 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key {} has too many keysyms for group {}, level {}; expected max {}, got: {}\n",
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                    ndx.wrapping_add(1 as xkb_layout_index_t),
                    level.wrapping_add(1 as xkb_level_index_t),
                    65535 as i32,
                    (*keysymList_0).syms.size,
                );
                return false;
            }
            (*leveli).num_syms = (*keysymList_0).syms.size as xkb_keysym_count_t;
            match (*leveli).num_syms as i32 {
                0 => {
                    (*leveli).s.sym = XKB_KEY_NoSymbol as xkb_keysym_t;
                }
                1 => {
                    (*leveli).s.sym = *(*keysymList_0).syms.item.offset(0 as i32 as isize);
                }
                _ => {
                    if (*keysymList_0).syms.size > 0 as darray_size_t {
                        (*keysymList_0).syms.alloc = (*keysymList_0).syms.size;
                        (*keysymList_0).syms.item = realloc(
                            (*keysymList_0).syms.item as *mut ::core::ffi::c_void,
                            ((*keysymList_0).syms.alloc as usize)
                                .wrapping_mul(std::mem::size_of::<xkb_keysym_t>()),
                        ) as *mut xkb_keysym_t;
                    }
                    (*leveli).s.syms = (*keysymList_0).syms.item;
                    if !std::ptr::null_mut::<u8>().is_null() {
                        *(std::ptr::null_mut() as *mut darray_size_t) = (*keysymList_0).syms.size;
                    }
                    (*keysymList_0).syms.item = std::ptr::null_mut();
                    (*keysymList_0).syms.size = 0 as darray_size_t;
                    (*keysymList_0).syms.alloc = 0 as darray_size_t;
                    let mut k: xkb_keysym_count_t = 0 as xkb_keysym_count_t;
                    while (k as i32) < (*leveli).num_syms as i32 {
                        k = k.wrapping_add(1);
                    }
                }
            }
            keysymList_0 = (*keysymList_0).common.next as *mut ExprKeysymList;
            level = level.wrapping_add(1);
        }
        return true;
    }
}
unsafe fn AddActionsToKey(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        let mut ndx: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if !GetGroupIndex(info, keyi, arrayNdx, GROUP_FIELD_ACTS, &raw mut ndx) {
            return false;
        }
        let mut groupi: *mut GroupInfo = &mut (&mut (*keyi).groups)[ndx as usize] as *mut GroupInfo;
        if (*value).common.type_0 as u32 == STMT_EXPR_EMPTY_LIST as u32 {
            (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_ACTS as u32) as group_field;
            return true;
        }
        if (*value).common.type_0 as u32 != STMT_EXPR_ACTION_LIST as u32 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_CRITICAL,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Bad expression type ({}) for action list value; Ignoring actions for group {} of {}\n",
                XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                (*value).common.type_0 as u32,
                ndx,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }
        if (*groupi).defined as u32 & GROUP_FIELD_ACTS as u32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_CRITICAL,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Actions for key {}, group {} already defined\n",
                XKB_WARNING_CONFLICTING_KEY_ACTION as i32,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                ndx,
            );
            return false;
        }
        let mut nLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut p: *mut ParseCommon = &raw mut (*value).common;
        while !p.is_null() {
            nLevels = nLevels.wrapping_add(1);
            p = (*p).next as *mut ParseCommon;
        }
        if ((*groupi).levels.len() as xkb_level_index_t) < nLevels {
            darray_resize_zero_vec(&mut (*groupi).levels, nLevels as usize);
        }
        (*groupi).defined = ((*groupi).defined as u32 | GROUP_FIELD_ACTS as u32) as group_field;
        let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut nonEmptyLevels: xkb_level_index_t = 0 as xkb_level_index_t;
        let mut actionList: *mut ExprActionList = value as *mut ExprActionList;
        while !actionList.is_null() {
            let mut c2rust_current_block_102: u64;
            let mut leveli: *mut xkb_level =
                &mut (&mut (*groupi).levels)[level as usize] as *mut xkb_level;
            let mut num_actions: u32 = 0 as u32;
            let mut act: *mut ExprDef = (*actionList).actions as *mut ExprDef;
            while !act.is_null() {
                num_actions = num_actions.wrapping_add(1);
                act = (*act).common.next as *mut ExprDef;
            }
            if (num_actions > 65535 as u32) as i64 != 0 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Key {} has too many actions for group {}, level {}; expected max {}, got: {}\n",
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                    ndx.wrapping_add(1 as xkb_layout_index_t),
                    level.wrapping_add(1 as xkb_level_index_t),
                    65535 as i32,
                    num_actions,
                );
                return false;
            }
            let mut actions: Vec<xkb_action> = Vec::new();
            let mut act_0: *mut ExprDef = (*actionList).actions as *mut ExprDef;
            loop {
                if act_0.is_null() {
                    c2rust_current_block_102 = 1134115459065347084;
                    break;
                }
                let mut toAct: xkb_action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                let r: xkb_parser_error = HandleActionDef(
                    (*info).keymap_info,
                    &raw mut (*info).default_actions,
                    &raw mut (*info).mods,
                    act_0,
                    &raw mut toAct,
                ) as xkb_parser_error;
                if r as u32 != PARSER_SUCCESS as u32 {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Illegal action definition for {}; Action for group {}/level {} ignored\n",
                        XKB_ERROR_INVALID_VALUE as i32,
                        crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                        ndx.wrapping_add(1 as xkb_layout_index_t),
                        level.wrapping_add(1 as xkb_level_index_t),
                    );
                    if r as u32 == PARSER_FATAL_ERROR as u32 {
                        drop(actions);
                        return false;
                    } else {
                        toAct.type_0 = ACTION_TYPE_NONE;
                    }
                }
                if !(toAct.type_0 as u32 == ACTION_TYPE_NONE as u32) {
                    if (num_actions == 1 as u32) as i64 != 0 {
                        (*leveli).num_actions = 1 as xkb_action_count_t;
                        (*leveli).a.action = toAct;
                        c2rust_current_block_102 = 1829140360157350833;
                        break;
                    } else {
                        actions.push(toAct);
                    }
                }
                act_0 = (*act_0).common.next as *mut ExprDef;
            }
            match c2rust_current_block_102 {
                1134115459065347084 => {
                    if actions.is_empty() {
                        (*leveli).num_actions = 0 as xkb_action_count_t;
                    } else if (actions.len() > 1) as i64 != 0 {
                        (*leveli).num_actions = actions.len() as xkb_action_count_t;
                        // Shrink to fit and steal the buffer
                        actions.shrink_to_fit();
                        let ptr = actions.as_mut_ptr();
                        let len = actions.len();
                        std::mem::forget(actions);
                        (*leveli).a.actions = ptr;
                        actions = Vec::new();
                    } else {
                        (*leveli).num_actions = 1 as xkb_action_count_t;
                        (*leveli).a.action = actions[0];
                        drop(actions);
                        actions = Vec::new();
                    }
                }
                _ => {}
            }
            if (*leveli).num_actions as i32 > 0 as i32 || (*leveli).num_syms as i32 > 0 as i32 {
                nonEmptyLevels = level.wrapping_add(1 as xkb_level_index_t);
            }
            actionList = (*actionList).common.next as *mut ExprActionList;
            level = level.wrapping_add(1);
        }
        if nonEmptyLevels < nLevels {
            if nonEmptyLevels > 0 as xkb_level_index_t {
                (*groupi).levels.truncate(nonEmptyLevels as usize);
            } else {
                (*groupi).levels.clear();
            }
        }
        return true;
    }
}
static mut repeatEntries: [LookupEntry; 8] = [
    LookupEntry {
        name: b"true\0".as_ptr() as *const i8,
        value: KEY_REPEAT_YES as u32,
    },
    LookupEntry {
        name: b"yes\0".as_ptr() as *const i8,
        value: KEY_REPEAT_YES as u32,
    },
    LookupEntry {
        name: b"on\0".as_ptr() as *const i8,
        value: KEY_REPEAT_YES as u32,
    },
    LookupEntry {
        name: b"false\0".as_ptr() as *const i8,
        value: KEY_REPEAT_NO as u32,
    },
    LookupEntry {
        name: b"no\0".as_ptr() as *const i8,
        value: KEY_REPEAT_NO as u32,
    },
    LookupEntry {
        name: b"off\0".as_ptr() as *const i8,
        value: KEY_REPEAT_NO as u32,
    },
    LookupEntry {
        name: b"default\0".as_ptr() as *const i8,
        value: KEY_REPEAT_UNDEFINED as u32,
    },
    LookupEntry {
        name: std::ptr::null(),
        value: 0 as u32,
    },
];
unsafe fn ExprResolveOverlayEntry(
    mut keymap_info: *const xkb_keymap_info,
    mut field: *const i8,
    mut arrayNdx: *const ExprDef,
    mut expr: *const ExprDef,
    mut keyi: *mut KeyInfo,
    mut overlay_rtrn: *mut xkb_overlay_index_t,
    mut key_rtrn: *mut *const xkb_key,
) -> bool {
    unsafe {
        if !arrayNdx.is_null() {
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Overlay field \"{}\" in {} does not support array index; ignored\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(field),
                crate::xkb::utils::CStrDisplay(KeyNameText(
                    (*keymap_info).keymap.ctx,
                    (*keyi).name
                )),
            );
            return false;
        }
        let prefix: usize = (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize);
        let len: usize = cstr_len(field.offset(prefix as isize)) as usize;
        let mut raw_overlay: i64 = XKB_OVERLAY_INVALID as i64;
        if parse_dec_to_uint64_t(
            field.offset(prefix as isize),
            len,
            &raw mut raw_overlay as *mut u64,
        ) != len as i32
            || raw_overlay < 1 as i64
            || raw_overlay > (*keymap_info).features.max_overlays as i64
        {
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unsupported overlay index \"{}\" field for {}: expected 1..{}, got: {}; ignored\n",
                XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX as i32,
                crate::xkb::utils::CStrDisplay(field),
                crate::xkb::utils::CStrDisplay(KeyNameText((*keymap_info).keymap.ctx, (*keyi).name)),
                (*keymap_info).features.max_overlays as i32,
                raw_overlay,
            );
            return false;
        }
        *overlay_rtrn =
            (raw_overlay as xkb_overlay_index_t as i32 - 1 as i32) as xkb_overlay_index_t;
        match (*expr).common.type_0 as u32 {
            8 => {
                *key_rtrn = XkbKeyByName(
                    &raw const (*keymap_info).keymap,
                    (*expr).key_name.key_name,
                    false,
                );
                if (*key_rtrn).is_null() {
                    xkb_logf!(
                        (*keymap_info).keymap.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unknown key \"{}\" for field {} in {}\n",
                        XKB_WARNING_UNDEFINED_KEYCODE as i32,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(
                            (*keymap_info).keymap.ctx,
                            (*expr).key_name.key_name
                        )),
                        crate::xkb::utils::CStrDisplay(field),
                        crate::xkb::utils::CStrDisplay(KeyNameText(
                            (*keymap_info).keymap.ctx,
                            (*keyi).name
                        )),
                    );
                    return false;
                }
                return true;
            }
            10 => {
                let id: *const i8 =
                    xkb_atom_text((*keymap_info).keymap.ctx, (*expr).ident.ident) as *const i8;
                if !id.is_null() && istreq(id, b"none\0".as_ptr() as *const i8) as i32 != 0 {
                    *key_rtrn = std::ptr::null();
                    return true;
                } else if !id.is_null() && istreq(id, b"any\0".as_ptr() as *const i8) as i32 != 0 {
                    *key_rtrn = std::ptr::null();
                    *overlay_rtrn = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
                    return true;
                }
                xkb_logf!(
                    (*keymap_info).keymap.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unsupported overlay value \"{}\" for field {} in {}\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::CStrDisplay(id),
                    crate::xkb::utils::CStrDisplay(field),
                    crate::xkb::utils::CStrDisplay(KeyNameText(
                        (*keymap_info).keymap.ctx,
                        (*keyi).name
                    )),
                );
                return false;
            }
            _ => {
                xkb_logf!(
                    (*keymap_info).keymap.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Expected {} for field \"{}\" in {}, got: {}\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string(STMT_EXPR_KEYNAME_LITERAL)),
                    crate::xkb::utils::CStrDisplay(field),
                    crate::xkb::utils::CStrDisplay(KeyNameText(
                        (*keymap_info).keymap.ctx,
                        (*keyi).name
                    )),
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return false;
            }
        };
    }
}
unsafe fn SetSymbolsField(
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
    mut field: *const i8,
    mut arrayNdx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if istreq(field, b"type\0".as_ptr() as *const i8) {
            let mut ndx: xkb_layout_index_t = 0 as xkb_layout_index_t;
            let mut val: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
            if !ExprResolveString((*info).ctx, value, &raw mut val) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The type field of a key symbol map must be a string; Ignoring illegal type definition\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                );
                return false;
            }
            if arrayNdx.is_null() {
                (*keyi).default_type = val;
                (*keyi).set_defined((*keyi).defined() | KEY_FIELD_DEFAULT_TYPE as i32 as key_field);
            } else if ExprResolveGroup(
                (*info).keymap_info,
                arrayNdx,
                false,
                &raw mut ndx,
                std::ptr::null_mut(),
            ) as u32
                != PARSER_SUCCESS as u32
            {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal group index for type of key {}; Definition with non-integer array index ignored\n",
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            } else {
                ndx = ndx.wrapping_sub(1);
                if ndx >= (*keyi).groups.len() as xkb_layout_index_t {
                    resize_groups_zero(&mut (*keyi).groups, (ndx as usize).wrapping_add(1));
                }
                (&mut (*keyi).groups)[ndx as usize].type_0 = val;
                let ref mut c2rust_fresh8 = (&mut (*keyi).groups)[ndx as usize].defined;
                *c2rust_fresh8 = (*c2rust_fresh8 as u32 | GROUP_FIELD_TYPE as u32) as group_field;
            }
        } else if istreq(field, b"symbols\0".as_ptr() as *const i8) {
            return AddSymbolsToKey(info, keyi, arrayNdx, value);
        } else if istreq(field, b"actions\0".as_ptr() as *const i8) {
            return AddActionsToKey(info, keyi, arrayNdx, value);
        } else if istreq(field, b"vmods\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"virtualmods\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"virtualmodifiers\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut mask: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
            if !ExprResolveModMask(
                (*info).ctx,
                value,
                MOD_VIRT,
                &raw mut (*info).mods,
                &raw mut mask,
            ) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Expected a virtual modifier mask, found {}; Ignoring virtual modifiers definition for key {}\n",
                    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*value).common.type_0)),
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).vmodmap = mask;
            (*keyi).set_defined((*keyi).defined() | KEY_FIELD_VMODMAP as i32 as key_field);
        } else if istreq(field, b"locking\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"lock\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"locks\0".as_ptr() as *const i8) as i32 != 0
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Key behaviors not supported; Ignoring locking specification for key {}\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
        } else if istreq(field, b"radiogroup\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"permanentradiogroup\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"allownone\0".as_ptr() as *const i8) as i32 != 0
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Radio groups not supported; Ignoring radio group specification for key {}\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
        } else if istrneq(
            b"permanentoverlay\0".as_ptr() as *const i8,
            field,
            (std::mem::size_of::<[i8; 17]>()).wrapping_sub(1 as usize),
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Permanent overlays not supported; Ignoring overlay specification for key {}\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
        } else if istrneq(
            b"overlay\0".as_ptr() as *const i8,
            field,
            (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize),
        ) {
            let mut overlay: xkb_overlay_index_t = XKB_OVERLAY_INVALID as xkb_overlay_index_t;
            let mut key: *const xkb_key = std::ptr::null();
            if !ExprResolveOverlayEntry(
                (*info).keymap_info,
                field,
                arrayNdx,
                *value_ptr,
                keyi,
                &raw mut overlay,
                &raw mut key,
            ) {
                return false;
            }
            if overlay as i32 == XKB_OVERLAY_INVALID {
                return true;
            } else if !key.is_null() && (*key).name == (*keyi).name {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as i32,
                    "Cannot overlay a key to itself; Ignoring overlay {} specification for key {}\n",
                    overlay as i32 + 1 as i32,
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
            } else {
                let mut prev: *const xkb_key = std::ptr::null();
                if overlays_get(keyi, overlay, &raw mut prev) {
                    if key != prev {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Conflicting overlays defined in key {}; use overlay{}={}, ignore overlay{}={}\n",
                            XKB_WARNING_CONFLICTING_KEY_FIELDS as i32,
                            crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                            overlay as i32 + 1 as i32,
                            crate::xkb::utils::CStrDisplay(if !prev.is_null() {
                                KeyNameText((*info).ctx, (*prev).name)
                            } else {
                                b"none\0".as_ptr() as *const i8
                            }),
                            overlay as i32 + 1 as i32,
                            crate::xkb::utils::CStrDisplay(if !key.is_null() {
                                KeyNameText((*info).ctx, (*key).name)
                            } else {
                                b"none\0".as_ptr() as *const i8
                            }),
                        );
                    }
                } else if (*(*info).keymap_info).features.overlapping_overlays {
                    if !overlays_insert(keyi, overlay, key) {
                        return false;
                    }
                    (*keyi).set_defined((*keyi).defined() | KEY_FIELD_OVERLAY as i32 as key_field);
                } else {
                    let mask_0: xkb_overlay_mask_t =
                        ((1 as u32) << overlay as i32) as xkb_overlay_mask_t;
                    if (*keyi).overlays == 0 || (*keyi).overlays_clear() as i32 != 0 {
                        if !key.is_null() {
                            (*keyi).overlays = mask_0;
                            (*keyi).set_overlays_clear((false) as bool);
                            (*keyi).c2rust_unnamed.overlay_key = key;
                        } else {
                            (*keyi).overlays =
                                ((*keyi).overlays as i32 | mask_0 as i32) as xkb_overlay_mask_t;
                            (*keyi).set_overlays_clear((true) as bool);
                        }
                        (*keyi)
                            .set_defined((*keyi).defined() | KEY_FIELD_OVERLAY as i32 as key_field);
                    } else if (*keyi).overlays != 0 {
                        if !key.is_null() {
                            xkb_logf!(
                                (*info).ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Overlapping overlays are not allowed in {}; use overlay{}={}, ignore overlay{}={}\n",
                                XKB_ERROR_OVERLAPPING_OVERLAY as i32,
                                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                                (*keyi).overlays as i32,
                                crate::xkb::utils::CStrDisplay(KeyNameText(
                                    (*info).ctx,
                                    (*(*keyi).c2rust_unnamed.overlay_key).name,
                                )),
                                overlay as i32 + 1 as i32,
                                crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*key).name)),
                            );
                            return (*(*info).keymap_info).strict as u32
                                & PARSER_NO_FIELD_VALUE_MISMATCH as u32
                                == 0;
                        }
                    }
                }
            }
        } else if istreq(field, b"repeating\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"repeats\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"repeat\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut val_0: u32 = 0 as u32;
            if !ExprResolveEnum(
                (*info).ctx,
                value,
                &raw mut val_0,
                &raw const repeatEntries as *const LookupEntry,
            ) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal repeat setting for {}; Non-boolean repeat setting ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).set_repeat(val_0 as key_repeat as key_repeat);
            (*keyi).set_defined((*keyi).defined() | KEY_FIELD_REPEAT as i32 as key_field);
        } else if istreq(field, b"groupswrap\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"wrapgroups\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut set: bool = false;
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal groupsWrap setting for {}; Non-boolean value ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).set_out_of_range_group_policy(
                (if set as i32 != 0 {
                    XKB_LAYOUT_OUT_OF_RANGE_WRAP as i32
                } else {
                    XKB_LAYOUT_OUT_OF_RANGE_CLAMP as i32
                }) as xkb_layout_out_of_range_policy
                    as xkb_layout_out_of_range_policy,
            );
            (*keyi).set_defined((*keyi).defined() | KEY_FIELD_GROUPINFO as i32 as key_field);
        } else if istreq(field, b"groupsclamp\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"clampgroups\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut set_0: bool = false;
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set_0) {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal groupsClamp setting for {}; Non-boolean value ignored\n",
                    XKB_ERROR_INVALID_VALUE as i32,
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            (*keyi).set_out_of_range_group_policy(
                (if set_0 as i32 != 0 {
                    XKB_LAYOUT_OUT_OF_RANGE_CLAMP as i32
                } else {
                    XKB_LAYOUT_OUT_OF_RANGE_WRAP as i32
                }) as xkb_layout_out_of_range_policy
                    as xkb_layout_out_of_range_policy,
            );
            (*keyi).set_defined((*keyi).defined() | KEY_FIELD_GROUPINFO as i32 as key_field);
        } else if istreq(field, b"groupsredirect\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"redirectgroups\0".as_ptr() as *const i8) as i32 != 0
        {
            let mut grp: xkb_layout_index_t = 0 as xkb_layout_index_t;
            let mut pending: bool = false;
            if ExprResolveGroup(
                (*info).keymap_info,
                value,
                false,
                &raw mut grp,
                &raw mut pending,
            ) as u32
                != PARSER_SUCCESS as u32
                && !pending
            {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal group index for redirect of key {}; Definition with non-integer group ignored\n",
                    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                    crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                );
                return false;
            }
            if pending {
                (*keyi).set_out_of_range_pending_group((true) as bool);
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
                *value_ptr = std::ptr::null_mut();
                (*keyi).out_of_range_group_number = pending_index as xkb_layout_index_t;
            } else {
                (*keyi).set_out_of_range_pending_group((false) as bool);
                (*keyi).out_of_range_group_number = grp.wrapping_sub(1 as xkb_layout_index_t);
            }
            (*keyi).set_out_of_range_group_policy(
                XKB_LAYOUT_OUT_OF_RANGE_REDIRECT as xkb_layout_out_of_range_policy,
            );
            (*keyi).set_defined((*keyi).defined() | KEY_FIELD_GROUPINFO as i32 as key_field);
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown field \"{}\" in a key; definition ignored\n",
                XKB_ERROR_UNKNOWN_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_KEY_FIELDS as u32 == 0;
        }
        return true;
    }
}
unsafe fn SetGroupName(
    mut info: *mut SymbolsInfo,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
    mut merge: merge_mode,
) -> bool {
    unsafe {
        if arrayNdx.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] You must specify an index when specifying a group name; Group name definition without array subscript ignored\n",
                XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX as i32,
            );
            return false;
        }
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if ExprResolveGroup(
            (*info).keymap_info,
            arrayNdx,
            false,
            &raw mut group,
            std::ptr::null_mut(),
        ) as u32
            != PARSER_SUCCESS as u32
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Illegal index in group name definition; Definition with non-integer array index ignored\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
            );
            return false;
        }
        let mut name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, value, &raw mut name) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Group name must be a string; Illegal name for group {} ignored\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                group,
            );
            return false;
        }
        let mut group_to_use: xkb_layout_index_t = 0;
        if (*info).explicit_group == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            group_to_use = group.wrapping_sub(1 as xkb_layout_index_t);
        } else if group.wrapping_sub(1 as xkb_layout_index_t) == 0 as xkb_layout_index_t {
            group_to_use = (*info).explicit_group;
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] An explicit group was specified for the '{}' map, but it provides a name for a group other than Group1 ({}); Ignoring group name '{}'\n",
                XKB_WARNING_NON_BASE_GROUP_NAME as i32,
                crate::xkb::utils::CStrDisplay((*info).name),
                group,
                crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, name)),
            );
            return false;
        }
        if group_to_use >= (*info).group_names.len() as xkb_layout_index_t {
            (*info)
                .group_names
                .resize((group_to_use as usize).wrapping_add(1), 0 as xkb_atom_t);
        } else {
            let old_name: xkb_atom_t = (&(*info).group_names)[group_to_use as usize];
            if old_name != XKB_ATOM_NONE as xkb_atom_t && old_name != name {
                let replace: bool = merge as u32 != MERGE_AUGMENT as u32;
                let use_0: xkb_atom_t = if replace as i32 != 0 { name } else { old_name };
                let ignore: xkb_atom_t = if replace as i32 != 0 { old_name } else { name };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple definitions of group {} name in map '{}'; Using '{}', ignoring '{}'\n",
                    group_to_use,
                    crate::xkb::utils::CStrDisplay((*info).name),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, use_0)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, ignore)),
                );
                name = use_0;
            }
        }
        (&mut (*info).group_names)[group_to_use as usize] = name;
        return true;
    }
}
unsafe fn HandleGlobalVar(mut info: *mut SymbolsInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const i8 = std::ptr::null();
        let mut field: *const i8 = std::ptr::null();
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        let mut ret: bool = false;
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        }
        if !elem.is_null() && istreq(elem, b"key\0".as_ptr() as *const i8) as i32 != 0 {
            let mut temp: KeyInfo = {
                let mut init = KeyInfo {
                    out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                    name: 0 as xkb_atom_t,
                    vmodmap: 0,
                    default_type: 0,
                    out_of_range_group_number: 0,
                    groups: Vec::new(),
                    overlays_alloc: 0,
                    overlays: 0,
                    c2rust_unnamed: C2Rust_Unnamed_21 {
                        overlay_key: std::ptr::null(),
                    },
                };
                init.set_out_of_range_group_policy(XKB_LAYOUT_OUT_OF_RANGE_WRAP);
                init.set_defined(0 as key_field);
                init.set_merge(MERGE_DEFAULT);
                init.set_repeat(KEY_REPEAT_UNDEFINED);
                init.set_out_of_range_pending_group(false);
                init.set_overlays_clear(false);
                init
            };
            InitKeyInfo((*info).ctx, &raw mut temp);
            temp.set_merge(
                (if temp.merge() as i32 == MERGE_REPLACE as i32 {
                    MERGE_OVERRIDE as u32
                } else {
                    (*stmt).merge as u32
                }) as merge_mode as merge_mode,
            );
            ret = SetSymbolsField(info, &raw mut temp, field, arrayNdx, &raw mut (*stmt).value);
            MergeKeys(info, &raw mut (*info).default_key, &raw mut temp, true);
        } else if elem.is_null()
            && (istreq(field, b"name\0".as_ptr() as *const i8) as i32 != 0
                || istreq(field, b"groupname\0".as_ptr() as *const i8) as i32 != 0)
        {
            ret = SetGroupName(info, arrayNdx, (*stmt).value as *mut ExprDef, (*stmt).merge);
        } else if elem.is_null()
            && (istreq(field, b"groupswrap\0".as_ptr() as *const i8) as i32 != 0
                || istreq(field, b"wrapgroups\0".as_ptr() as *const i8) as i32 != 0)
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Global \"groupswrap\" not supported; Ignored\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if elem.is_null()
            && (istreq(field, b"groupsclamp\0".as_ptr() as *const i8) as i32 != 0
                || istreq(field, b"clampgroups\0".as_ptr() as *const i8) as i32 != 0)
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Global \"groupsclamp\" not supported; Ignored\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if elem.is_null()
            && (istreq(field, b"groupsredirect\0".as_ptr() as *const i8) as i32 != 0
                || istreq(field, b"redirectgroups\0".as_ptr() as *const i8) as i32 != 0)
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Global \"groupsredirect\" not supported; Ignored\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if elem.is_null() && istreq(field, b"allownone\0".as_ptr() as *const i8) as i32 != 0
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Radio groups not supported; Ignoring \"allownone\" specification\n",
                XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD as i32,
            );
            ret = true;
        } else if !elem.is_null() {
            ret = SetDefaultActionField(
                (*info).keymap_info,
                &raw mut (*info).default_actions,
                &raw mut (*info).mods,
                elem,
                field,
                arrayNdx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as u32
                != PARSER_FATAL_ERROR as u32;
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
                & PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS as u32
                == 0;
        }
        return ret;
    }
}
unsafe fn HandleSymbolsBody(
    mut info: *mut SymbolsInfo,
    mut def: *mut VarDef,
    mut keyi: *mut KeyInfo,
) -> bool {
    unsafe {
        let mut all_valid_entries: bool = true;
        while !def.is_null() {
            let mut field: *const i8 = std::ptr::null();
            let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
            let mut ok: bool = true;
            if (*def).name.is_null() {
                if (*def).value.is_null() as i64 != 0
                    || (*(*def).value).common.type_0 as u32 != STMT_EXPR_ACTION_LIST as u32
                {
                    field = b"symbols\0".as_ptr() as *const i8;
                } else {
                    field = b"actions\0".as_ptr() as *const i8;
                }
                arrayNdx = std::ptr::null_mut();
            } else {
                let mut elem: *const i8 = std::ptr::null();
                ok = ExprResolveLhs(
                    (*info).ctx,
                    (*def).name,
                    &raw mut elem,
                    &raw mut field,
                    &raw mut arrayNdx,
                );
                if ok as i32 != 0 && !elem.is_null() {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(field),
                    );
                    ok = false;
                }
            }
            if (*def).value.is_null() as i64 != 0 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Could not allocate the value of field \"{}\". Statement ignored.\n",
                    XKB_ERROR_ALLOCATION_ERROR as i32,
                    crate::xkb::utils::CStrDisplay(field),
                );
                ok = false;
            }
            if !ok || !SetSymbolsField(info, keyi, field, arrayNdx, &raw mut (*def).value) {
                all_valid_entries = false;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return all_valid_entries;
    }
}
unsafe fn SetExplicitGroup(mut info: *mut SymbolsInfo, mut keyi: *mut KeyInfo) -> bool {
    unsafe {
        let mut i: xkb_layout_index_t = 0;
        let mut groupi: *mut GroupInfo = std::ptr::null_mut();
        let mut warn: bool = false;
        if (*info).explicit_group == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return true;
        }
        if !(&(*keyi).groups).is_empty() {
            i = 1 as xkb_layout_index_t;
            while i < (&(*keyi).groups).len() as xkb_layout_index_t {
                if (&(*keyi).groups)[i as usize].defined as u64 != 0 {
                    warn = true;
                    ClearGroupInfo(&mut (&mut (*keyi).groups)[i as usize] as *mut GroupInfo);
                    InitGroupInfo(&mut (&mut (*keyi).groups)[i as usize] as *mut GroupInfo);
                }
                i = i.wrapping_add(1);
            }
        }
        if warn {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] For the map {} the explicit group {} is specified, but key {} has more than one group defined; All groups except first one will be ignored\n",
                XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE as i32,
                crate::xkb::utils::CStrDisplay((*info).name),
                (*info).explicit_group.wrapping_add(1 as xkb_layout_index_t),
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
        }
        darray_resize_zero_vec(
            &mut (*keyi).groups,
            ((*info).explicit_group as usize).wrapping_add(1),
        );
        if (*info).explicit_group > 0 as xkb_layout_index_t {
            let swapped = std::ptr::read(&(&(*keyi).groups)[0]);
            std::ptr::write(
                &mut (&mut (*keyi).groups)[(*info).explicit_group as usize],
                swapped,
            );
            InitGroupInfo(&mut (&mut (*keyi).groups)[0] as *mut GroupInfo);
        }
        return true;
    }
}
unsafe fn HandleSymbolsDef(mut info: *mut SymbolsInfo, mut stmt: *mut SymbolsDef) -> bool {
    unsafe {
        let mut keyi: KeyInfo = KeyInfo {
            name: 0,
            vmodmap: 0,
            default_type: 0,
            out_of_range_group_number: 0,
            groups: Vec::new(),
            out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
            overlays_alloc: 0,
            overlays: 0,
            c2rust_unnamed: C2Rust_Unnamed_21 {
                overlay_key: std::ptr::null(),
            },
        };
        keyi = (*info).default_key.clone();
        keyi.groups = Vec::new();
        if !(*info).default_key.groups.is_empty() {
            // Shallow copy the GroupInfo structs (bitwise), then deep-copy inner pointers
            keyi.groups
                .extend_from_slice(&(&(*info).default_key.groups));
        }
        let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while i < keyi.groups.len() as xkb_layout_index_t {
            CopyGroupInfo(
                &mut keyi.groups[i as usize] as *mut GroupInfo,
                &(&(*info).default_key.groups)[i as usize] as *const GroupInfo,
            );
            i = i.wrapping_add(1);
        }
        keyi.set_merge((*stmt).merge as merge_mode);
        keyi.name = (*stmt).keyName;
        if HandleSymbolsBody(info, (*stmt).symbols, &raw mut keyi) as i32 != 0
            && SetExplicitGroup(info, &raw mut keyi) as i32 != 0
            && AddKeySymbols(info, &raw mut keyi, true) as i32 != 0
        {
            return true;
        }
        ClearKeyInfo(&raw mut keyi);
        (*info).errorCount += 1;
        return false;
    }
}
unsafe fn HandleModMapDef(mut info: *mut SymbolsInfo, mut def: *mut ModMapDef) -> bool {
    unsafe {
        let mut tmp: ModMapEntry = ModMapEntry {
            merge: MERGE_DEFAULT,
            haveSymbol: false,
            modifier: 0,
            u: C2Rust_Unnamed_19 { keyName: 0 },
        };
        let mut ndx: xkb_mod_index_t = 0;
        let mut ok: bool = false;
        let mut ctx: *mut xkb_context = (*info).ctx;
        let mut modifier_name: *const i8 = xkb_atom_text(ctx, (*def).modifier);
        if istreq(modifier_name, b"none\0".as_ptr() as *const i8) {
            ndx = XKB_MOD_NONE as xkb_mod_index_t;
        } else {
            ndx = XkbModNameToIndex(&raw mut (*info).mods, (*def).modifier, MOD_REAL);
            if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Illegal modifier map definition; Ignoring map for non-modifier \"{}\"\n",
                    XKB_ERROR_INVALID_REAL_MODIFIER as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*def).modifier)),
                );
                return false;
            }
        }
        ok = true;
        tmp.modifier = ndx;
        tmp.merge = (*def).merge;
        let mut c2rust_current_block_19: u64;
        let mut key: *mut ExprDef = (*def).keys as *mut ExprDef;
        while !key.is_null() {
            if (*key).common.type_0 as u32 == STMT_EXPR_KEYNAME_LITERAL as u32 {
                tmp.haveSymbol = false;
                tmp.u.keyName = (*key).key_name.key_name;
                c2rust_current_block_19 = 5601891728916014340;
            } else if (*key).common.type_0 as u32 == STMT_EXPR_KEYSYM_LITERAL as u32 {
                if (*key).keysym.keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    c2rust_current_block_19 = 13536709405535804910;
                } else {
                    tmp.haveSymbol = true;
                    tmp.u.keySym = (*key).keysym.keysym;
                    c2rust_current_block_19 = 5601891728916014340;
                }
            } else {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Modmap entries may contain only key names or keysyms; Illegal definition for {} modifier ignored\n",
                    XKB_ERROR_INVALID_MODMAP_ENTRY as i32,
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, tmp.modifier)),
                );
                c2rust_current_block_19 = 13536709405535804910;
            }
            match c2rust_current_block_19 {
                5601891728916014340 => {
                    ok = AddModMapEntry(info, &raw mut tmp) as i32 != 0 && ok as i32 != 0;
                }
                _ => {}
            }
            key = (*key).common.next as *mut ExprDef;
        }
        return ok;
    }
}
unsafe fn HandleSymbolsFile(mut info: *mut SymbolsInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        cstr_free((*info).name);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
                1 => {
                    ok = HandleIncludeSymbols(info, stmt as *mut IncludeStmt);
                }
                30 => {
                    ok = HandleSymbolsDef(info, stmt as *mut SymbolsDef);
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef((*info).ctx, &raw mut (*info).mods, stmt as *mut VModDef);
                }
                31 => {
                    ok = HandleModMapDef(info, stmt as *mut ModMapDef);
                }
                35 | 36 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported symbols {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        crate::xkb::utils::CStrDisplay(
                            if (*stmt).type_0 as u32 == STMT_UNKNOWN_COMPOUND as u32 {
                                b"compound\0".as_ptr() as *const i8
                            } else {
                                b"declaration\0".as_ptr() as *const i8
                            }
                        ),
                        crate::xkb::utils::CStrDisplay((*(stmt as *mut UnknownStatement)).name),
                    );
                    ok = (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_STATEMENTS as u32
                        == 0;
                }
                _ => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Symbols files may not include other types; Ignoring {}\n",
                        XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
                        crate::xkb::utils::CStrDisplay(stmt_type_to_string((*stmt).type_0)),
                    );
                    ok = false;
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
                    "[XKB-{:03}] Abandoning symbols file \"{}\"\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn FindKeyForSymbol(mut keymap: *mut xkb_keymap, mut sym: xkb_keysym_t) -> *mut xkb_key {
    unsafe {
        let mut got_one_group: bool = false;
        let mut group: xkb_layout_index_t = 0 as xkb_layout_index_t;
        loop {
            let mut level: xkb_level_index_t = 0 as xkb_level_index_t;
            got_one_group = false;
            let mut got_one_level: bool = false;
            loop {
                got_one_level = false;
                let mut key: *mut xkb_key = std::ptr::null_mut();
                key = (*keymap).keys.offset(
                    (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                        0 as xkb_keycode_t
                    } else {
                        (*keymap).min_key_code
                    }) as isize,
                );
                while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
                    if group < (*key).num_groups() && level < XkbKeyNumLevels(key, group) {
                        got_one_level = true;
                        got_one_group = got_one_level;
                        let num_syms: xkb_keysym_count_t =
                            (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .num_syms;
                        if num_syms as i32 > 1 as i32 {
                            let mut k: xkb_keysym_count_t = 0 as xkb_keysym_count_t;
                            while (k as i32) < num_syms as i32 {
                                if *(*(*(*key).groups.offset(group as isize))
                                    .levels
                                    .offset(level as isize))
                                .s
                                .syms
                                .offset(k as isize)
                                    == sym
                                {
                                    return key;
                                }
                                k = k.wrapping_add(1);
                            }
                        } else if num_syms as i32 != 0
                            && (*(*(*key).groups.offset(group as isize))
                                .levels
                                .offset(level as isize))
                            .s
                            .sym == sym
                        {
                            return key;
                        }
                    }
                    key = key.offset(1);
                }
                level = level.wrapping_add(1);
                if !got_one_level {
                    break;
                }
            }
            group = group.wrapping_add(1);
            if !got_one_group {
                break;
            }
        }
        return std::ptr::null_mut();
    }
}
unsafe fn FindAutomaticType(mut ctx: *mut xkb_context, mut groupi: *mut GroupInfo) -> xkb_atom_t {
    unsafe {
        let mut sym0: xkb_keysym_t = 0;
        let mut sym1: xkb_keysym_t = 0;
        let width: xkb_level_index_t = (*groupi).levels.len() as xkb_level_index_t;
        if width == 1 as xkb_level_index_t || width <= 0 as xkb_level_index_t {
            return xkb_atom_intern(
                ctx,
                b"ONE_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
            );
        }
        sym0 = if (*(*groupi).levels.as_ptr().offset(0 as i32 as isize)).num_syms as i32 == 0 as i32
        {
            XKB_KEY_NoSymbol as xkb_keysym_t
        } else if (*(*groupi).levels.as_ptr().offset(0 as i32 as isize)).num_syms as i32 == 1 as i32
        {
            (*(*groupi).levels.as_ptr().offset(0 as i32 as isize)).s.sym
        } else {
            *(*(*groupi).levels.as_ptr().offset(0 as i32 as isize))
                .s
                .syms
                .offset(0 as i32 as isize)
        };
        sym1 = if (*(*groupi).levels.as_ptr().offset(1 as i32 as isize)).num_syms as i32 == 0 as i32
        {
            XKB_KEY_NoSymbol as xkb_keysym_t
        } else if (*(*groupi).levels.as_ptr().offset(1 as i32 as isize)).num_syms as i32 == 1 as i32
        {
            (*(*groupi).levels.as_ptr().offset(1 as i32 as isize)).s.sym
        } else {
            *(*(*groupi).levels.as_ptr().offset(1 as i32 as isize))
                .s
                .syms
                .offset(0 as i32 as isize)
        };
        if width == 2 as xkb_level_index_t {
            if xkb_keysym_is_lower(sym0) as i32 != 0
                && xkb_keysym_is_upper_or_title(sym1) as i32 != 0
            {
                return xkb_atom_intern(
                    ctx,
                    b"ALPHABETIC\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 11]>()).wrapping_sub(1 as usize),
                );
            }
            if xkb_keysym_is_keypad(sym0) as i32 != 0 || xkb_keysym_is_keypad(sym1) as i32 != 0 {
                return xkb_atom_intern(
                    ctx,
                    b"KEYPAD\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
                );
            }
            return xkb_atom_intern(
                ctx,
                b"TWO_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
            );
        }
        if width <= 4 as xkb_level_index_t {
            if xkb_keysym_is_lower(sym0) as i32 != 0
                && xkb_keysym_is_upper_or_title(sym1) as i32 != 0
            {
                let mut sym2: xkb_keysym_t = 0;
                let mut sym3: xkb_keysym_t = 0;
                sym2 = if (*(*groupi).levels.as_ptr().offset(2 as i32 as isize)).num_syms as i32
                    == 0 as i32
                {
                    XKB_KEY_NoSymbol as xkb_keysym_t
                } else if (*(*groupi).levels.as_ptr().offset(2 as i32 as isize)).num_syms as i32
                    == 1 as i32
                {
                    (*(*groupi).levels.as_ptr().offset(2 as i32 as isize)).s.sym
                } else {
                    *(*(*groupi).levels.as_ptr().offset(2 as i32 as isize))
                        .s
                        .syms
                        .offset(0 as i32 as isize)
                };
                sym3 = if width == 4 as xkb_level_index_t {
                    if (*(*groupi).levels.as_ptr().offset(3 as i32 as isize)).num_syms as i32
                        == 0 as i32
                    {
                        XKB_KEY_NoSymbol as xkb_keysym_t
                    } else if (*(*groupi).levels.as_ptr().offset(3 as i32 as isize)).num_syms as i32
                        == 1 as i32
                    {
                        (*(*groupi).levels.as_ptr().offset(3 as i32 as isize)).s.sym
                    } else {
                        *(*(*groupi).levels.as_ptr().offset(3 as i32 as isize))
                            .s
                            .syms
                            .offset(0 as i32 as isize)
                    }
                } else {
                    XKB_KEY_NoSymbol as xkb_keysym_t
                };
                if xkb_keysym_is_lower(sym2) as i32 != 0
                    && xkb_keysym_is_upper_or_title(sym3) as i32 != 0
                {
                    return xkb_atom_intern(
                        ctx,
                        b"FOUR_LEVEL_ALPHABETIC\0".as_ptr() as *const i8,
                        (std::mem::size_of::<[i8; 22]>()).wrapping_sub(1 as usize),
                    );
                }
                return xkb_atom_intern(
                    ctx,
                    b"FOUR_LEVEL_SEMIALPHABETIC\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 26]>()).wrapping_sub(1 as usize),
                );
            }
            if xkb_keysym_is_keypad(sym0) as i32 != 0 || xkb_keysym_is_keypad(sym1) as i32 != 0 {
                return xkb_atom_intern(
                    ctx,
                    b"FOUR_LEVEL_KEYPAD\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 18]>()).wrapping_sub(1 as usize),
                );
            }
            return xkb_atom_intern(
                ctx,
                b"FOUR_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 11]>()).wrapping_sub(1 as usize),
            );
        }
        return XKB_ATOM_NONE as xkb_atom_t;
    }
}
unsafe fn FindTypeForGroup(
    mut keymap: *mut xkb_keymap,
    mut keyi: *mut KeyInfo,
    mut group: xkb_layout_index_t,
    mut explicit_type: *mut bool,
) -> *const xkb_key_type {
    unsafe {
        let mut i: darray_size_t = 0;
        let mut groupi: *mut GroupInfo =
            &mut (&mut (*keyi).groups)[group as usize] as *mut GroupInfo;
        let mut type_name: xkb_atom_t = (*groupi).type_0;
        *explicit_type = true;
        if type_name == XKB_ATOM_NONE as xkb_atom_t {
            if (*keyi).default_type != XKB_ATOM_NONE as xkb_atom_t {
                type_name = (*keyi).default_type;
            } else {
                type_name = FindAutomaticType((*keymap).ctx, groupi);
                if type_name != XKB_ATOM_NONE as xkb_atom_t {
                    *explicit_type = false;
                }
            }
        }
        if type_name == XKB_ATOM_NONE as xkb_atom_t {
            xkb_logf!(
                (*keymap).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Couldn't find an automatic type for key '{}' group {} with {} levels; Using the default type\n",
                XKB_WARNING_CANNOT_INFER_KEY_TYPE as i32,
                crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*keyi).name)),
                group.wrapping_add(1 as xkb_layout_index_t),
                (*groupi).levels.len(),
            );
        } else {
            i = 0;
            i = 0 as darray_size_t;
            while i < (*keymap).num_types {
                if (*(*keymap).types.offset(i as isize)).name == type_name {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i >= (*keymap).num_types {
                xkb_logf!(
                    (*keymap).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The type \"{}\" for key '{}' group {} was not previously defined; Using the default type\n",
                    XKB_WARNING_UNDEFINED_KEY_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*keymap).ctx, type_name)),
                    crate::xkb::utils::CStrDisplay(KeyNameText((*keymap).ctx, (*keyi).name)),
                    group.wrapping_add(1 as xkb_layout_index_t),
                );
            } else {
                (*(*keymap).types.offset(i as isize)).required = true;
                return (*keymap).types.offset(i as isize) as *mut xkb_key_type;
            }
        }
        (*(*keymap).types.offset(0 as i32 as isize)).required = true;
        return (*keymap).types.offset(0 as i32 as isize) as *mut xkb_key_type;
    }
}
unsafe fn CopySymbolsDefToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
    mut keyi: *mut KeyInfo,
) -> bool {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
        let mut groupi: *mut GroupInfo = std::ptr::null_mut();
        let mut i: xkb_layout_index_t = 0;

        // The name is guaranteed to be real and not an alias, so 'false' is safe here
        key = XkbKeyByName(keymap, (*keyi).name, false);
        if key.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_DETAILED as i32,
                "[XKB-{:03}] Key {} not found in keycodes; Symbols ignored\n",
                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
            );
            return false;
        }

        // Find the range of groups we need
        (*key).set_num_groups(0);
        if !(*keyi).groups.is_empty() {
            i = 0;
            groupi = (*keyi).groups.as_mut_ptr();
            while i < (*keyi).groups.len() as xkb_layout_index_t {
                // Skip groups that have no levels and no explicit type
                let has_explicit_type = ((*keyi).defined() as i32 & KEY_FIELD_DEFAULT_TYPE as i32
                    != 0)
                    || ((*groupi).defined as u32 & GROUP_FIELD_TYPE as u32 != 0);
                if (*groupi).levels.len() > 0 || has_explicit_type {
                    (*key).set_num_groups(i.wrapping_add(1));
                }
                if has_explicit_type {
                    (*key).explicit =
                        ((*key).explicit as u32 | EXPLICIT_TYPES as u32) as xkb_explicit_components;
                }
                i = i.wrapping_add(1);
                groupi = groupi.offset(1);
            }
        }

        if (*key).num_groups() <= 0 {
            // A key with no group may still have other fields defined
            if (*keyi).defined() as i32 != 0 {
                // goto key_fields
            } else {
                return false;
            }
        } else {
            // Resize groups array
            let __need: usize = (*key).num_groups() as usize;
            resize_groups_zero(&mut (*keyi).groups, __need);

            // If there are empty groups between non-empty ones, fill them with data from the first group
            if !(*keyi).groups.is_empty() {
                // Must use raw pointers because CopyGroupInfo takes ptrs and we need group0 as const while mutating others
                let groups_ptr = (*keyi).groups.as_mut_ptr();
                let groups_len = (*keyi).groups.len();
                i = 1;
                while i < groups_len as xkb_layout_index_t {
                    if (*groups_ptr.add(i as usize)).defined == 0 {
                        CopyGroupInfo(groups_ptr.add(i as usize), groups_ptr as *const GroupInfo);
                    }
                    i = i.wrapping_add(1);
                }
            }

            (*key).groups = calloc(
                (*key).num_groups() as usize,
                std::mem::size_of::<xkb_group>(),
            ) as *mut xkb_group;

            // Find and assign the groups' types in the keymap
            if !(*keyi).groups.is_empty() {
                i = 0;
                groupi = (*keyi).groups.as_mut_ptr();
                while i < (*keyi).groups.len() as xkb_layout_index_t {
                    let mut explicit_type = false;
                    let type_0: *const xkb_key_type =
                        FindTypeForGroup(keymap, keyi, i, &raw mut explicit_type);

                    // Always have as many levels as the type specifies
                    if (*type_0).num_levels < (*groupi).levels.len() as darray_size_t {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_BRIEF as i32,
                            "[XKB-{:03}] Type \"{}\" has {} levels, but {} has {} levels; Ignoring extra symbols\n",
                            XKB_WARNING_EXTRA_SYMBOLS_IGNORED as i32,
                            crate::xkb::utils::CStrDisplay(xkb_atom_text((*keymap).ctx, (*type_0).name)),
                            (*type_0).num_levels,
                            crate::xkb::utils::CStrDisplay(KeyInfoText(info, keyi)),
                            (*groupi).levels.len(),
                        );

                        for lvl_idx in (*type_0).num_levels as usize..(*groupi).levels.len() {
                            clear_level(&mut (&mut (*groupi).levels)[lvl_idx] as *mut xkb_level);
                        }
                    }

                    // Resize levels array to match type
                    let __need_levels: usize = (*type_0).num_levels as usize;
                    darray_resize_zero_vec(&mut (*groupi).levels, __need_levels);

                    (*(*key).groups.offset(i as isize)).set_explicit_type(explicit_type);
                    (*(*key).groups.offset(i as isize)).type_0 = type_0;

                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            // Copy levels
            if !(*keyi).groups.is_empty() {
                i = 0;
                groupi = (*keyi).groups.as_mut_ptr();
                while i < (*keyi).groups.len() as xkb_layout_index_t {
                    // Compute the capitalization transformation of the keysyms
                    let mut leveli: *mut xkb_level = std::ptr::null_mut();
                    if !(*groupi).levels.is_empty() {
                        leveli = (*groupi).levels.as_mut_ptr();
                        while leveli < (*groupi).levels.as_mut_ptr().add((*groupi).levels.len()) {
                            match (*leveli).num_syms {
                                0 => {
                                    (*leveli).c2rust_unnamed.upper =
                                        XKB_KEY_NoSymbol as xkb_keysym_t;
                                }
                                1 => {
                                    (*leveli).c2rust_unnamed.upper =
                                        xkb_keysym_to_upper((*leveli).s.sym);
                                }
                                _ => {
                                    // Multiple keysyms: check if there is any cased keysym
                                    (*leveli).c2rust_unnamed.has_upper = false;
                                    let mut k: xkb_keysym_count_t = 0;
                                    while k < (*leveli).num_syms {
                                        let upper: xkb_keysym_t = xkb_keysym_to_upper(
                                            *(*leveli).s.syms.offset(k as isize),
                                        );
                                        if upper != *(*leveli).s.syms.offset(k as isize) {
                                            (*leveli).c2rust_unnamed.has_upper = true;
                                            break;
                                        }
                                        k = k.wrapping_add(1);
                                    }
                                    if (*leveli).c2rust_unnamed.has_upper {
                                        // Some cased keysyms: store the transformation result
                                        (*leveli).s.syms = realloc(
                                            (*leveli).s.syms as *mut ::core::ffi::c_void,
                                            (2 * (*leveli).num_syms as usize)
                                                .wrapping_mul(std::mem::size_of::<xkb_keysym_t>()),
                                        )
                                            as *mut xkb_keysym_t;
                                        if (*leveli).s.syms.is_null() {
                                            return false;
                                        }
                                        let mut k: xkb_keysym_count_t = 0;
                                        while k < (*leveli).num_syms {
                                            *(*leveli)
                                                .s
                                                .syms
                                                .offset(((*leveli).num_syms + k) as isize) =
                                                xkb_keysym_to_upper(
                                                    *(*leveli).s.syms.offset(k as isize),
                                                );
                                            k = k.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                            leveli = leveli.offset(1);
                        }
                    }

                    // Copy the level (steal from Vec)
                    if (*groupi).levels.is_empty() {
                        (*(*key).groups.offset(i as isize)).levels = std::ptr::null_mut();
                    } else {
                        let levels_ptr = (*groupi).levels.as_mut_ptr();
                        let levels_len = (*groupi).levels.len();
                        // Forget the Vec so it doesn't free the buffer
                        std::ptr::write(&raw mut (*groupi).levels, Vec::new());
                        (*(*key).groups.offset(i as isize)).levels = levels_ptr;
                    }

                    if (*(*(*key).groups.offset(i as isize)).type_0).num_levels > 1
                        || (*(*(*key).groups.offset(i as isize)).levels.offset(0)).num_syms > 0
                    {
                        (*(*key).groups.offset(i as isize)).set_explicit_symbols(true);
                        (*key).explicit = ((*key).explicit as u32 | EXPLICIT_SYMBOLS as u32)
                            as xkb_explicit_components;
                    }
                    if (*groupi).defined as u32 & GROUP_FIELD_ACTS as u32 != 0 {
                        (*(*key).groups.offset(i as isize)).set_explicit_actions(true);
                        (*key).explicit = ((*key).explicit as u32 | EXPLICIT_INTERP as u32)
                            as xkb_explicit_components;
                    }
                    if (*(*key).groups.offset(i as isize)).explicit_type() {
                        (*key).explicit = ((*key).explicit as u32 | EXPLICIT_TYPES as u32)
                            as xkb_explicit_components;
                    }

                    i = i.wrapping_add(1);
                    groupi = groupi.offset(1);
                }
            }

            (*key).set_out_of_range_pending_group((*keyi).out_of_range_pending_group());
            (*key).set_out_of_range_group_number((*keyi).out_of_range_group_number);
            (*key).set_out_of_range_group_policy((*keyi).out_of_range_group_policy());
        }

        // key_fields:
        if (*keyi).defined() as i32 & KEY_FIELD_VMODMAP as i32 != 0 {
            (*key).vmodmap = (*keyi).vmodmap;
            (*key).explicit =
                ((*key).explicit as u32 | EXPLICIT_VMODMAP as u32) as xkb_explicit_components;
        }

        if (*keyi).repeat() != KEY_REPEAT_UNDEFINED as key_repeat {
            (*key).set_repeats((*keyi).repeat() == KEY_REPEAT_YES as key_repeat);
            (*key).explicit =
                ((*key).explicit as u32 | EXPLICIT_REPEAT as u32) as xkb_explicit_components;
        }

        if ((*keyi).defined() as i32 & KEY_FIELD_OVERLAY as i32 != 0)
            && (*keyi).overlays != 0
            && !(*keyi).overlays_clear()
        {
            (*key).overlays = (*keyi).overlays;
            if (*keyi).overlays_alloc != 0 {
                // Remove empty entries
                let mut remaining: xkb_overlay_mask_t = (*key).overlays;
                let mut overlays_keys: *mut *const xkb_key = (*keyi).c2rust_unnamed.overlays_keys;
                while remaining != 0 {
                    // isolate lowest set bit
                    let lsb: xkb_overlay_mask_t = remaining & (!remaining).wrapping_add(1);
                    remaining = remaining & !lsb;
                    if !(*overlays_keys).is_null() {
                        overlays_keys = overlays_keys.offset(1);
                    } else {
                        // drop current null value
                        (*key).overlays &= !lsb;
                    }
                }

                if (*key).overlays != 0 {
                    // Steal
                    (*key).c2rust_unnamed.overlays_keys = (*keyi).c2rust_unnamed.overlays_keys;
                    (*keyi).c2rust_unnamed.overlays_keys = ::core::ptr::null_mut();
                    (*keyi).overlays_alloc = 0;

                    (*key).set_overlays_inline(false);
                    (*key).explicit = ((*key).explicit as u32 | EXPLICIT_OVERLAY as u32)
                        as xkb_explicit_components;
                }
            } else {
                (*key).c2rust_unnamed.overlay_key = (*keyi).c2rust_unnamed.overlay_key;
                (*key).set_overlays_inline(true);
                (*key).explicit =
                    ((*key).explicit as u32 | EXPLICIT_OVERLAY as u32) as xkb_explicit_components;
            }
        }

        return true;
    }
}

unsafe fn CopyModMapDefToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut SymbolsInfo,
    mut entry: *mut ModMapEntry,
) -> bool {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
        if !(*entry).haveSymbol {
            key = XkbKeyByName(keymap, (*entry).u.keyName, true);
            if key.is_null() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "[XKB-{:03}] Key {} not found in keycodes; Modifier map entry for {} not updated\n",
                    XKB_WARNING_UNDEFINED_KEYCODE as i32,
                    crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*entry).u.keyName)),
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, (*entry).modifier)),
                );
                return false;
            }
        } else {
            key = FindKeyForSymbol(keymap, (*entry).u.keySym);
            if key.is_null() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "[XKB-{:03}] Key \"{}\" not found in symbol map; Modifier map entry for {} not updated\n",
                    XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL as i32,
                    crate::xkb::utils::CStrDisplay(KeysymText((*info).ctx, (*entry).u.keySym)),
                    crate::xkb::utils::CStrDisplay(ModIndexText((*info).ctx, &raw mut (*info).mods, (*entry).modifier)),
                );
                return false;
            }
        }
        if (*entry).modifier != XKB_MOD_NONE as xkb_mod_index_t {
            (*key).modmap =
                ((*key).modmap as u32 | (1 as u32) << (*entry).modifier) as xkb_mod_mask_t;
        }
        return true;
    }
}
unsafe fn CopySymbolsToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut SymbolsInfo) -> bool {
    unsafe {
        (*keymap).symbols_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).symbols_section_name);
        (*keymap).mods = (*info).mods;
        (*keymap).num_group_names = (*info).group_names.len() as xkb_layout_index_t;
        // Steal the group_names Vec's buffer
        let mut gn = std::mem::take(&mut (*info).group_names);
        if gn.is_empty() {
            (*keymap).group_names = std::ptr::null_mut();
        } else {
            gn.shrink_to_fit();
            (*keymap).group_names = gn.as_mut_ptr();
            std::mem::forget(gn);
        }
        for keyi in (*info).keys.iter_mut() {
            if !CopySymbolsDefToKeymap(keymap, info, keyi as *mut KeyInfo) {
                (*info).errorCount += 1;
            }
        }
        if xkb_context_get_log_verbosity((*keymap).ctx) > 3 as i32 {
            let mut key: *mut xkb_key = std::ptr::null_mut();
            key = (*keymap).keys.offset(
                (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                    0 as xkb_keycode_t
                } else {
                    (*keymap).min_key_code
                }) as isize,
            );
            while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
                if !((*key).name == XKB_ATOM_NONE as xkb_atom_t) {
                    if ((*key).num_groups() as i32) < 1 as i32 {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_INFO,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "No symbols defined for {}\n",
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*key).name)),
                        );
                    }
                }
                key = key.offset(1);
            }
        }
        for mm in (*info).modmaps.iter_mut() {
            if !CopyModMapDefToKeymap(keymap, info, mm as *mut ModMapEntry) {
                (*info).errorCount += 1;
            }
        }
        return true;
    }
}
pub unsafe fn CompileSymbols(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: SymbolsInfo = SymbolsInfo {
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            explicit_group: 0,
            max_groups: 0,
            keys: Vec::new(),
            default_key: KeyInfo {
                name: 0,
                vmodmap: 0,
                default_type: 0,
                out_of_range_group_number: 0,
                groups: Vec::new(),
                out_of_range_group_policy_defined_merge_repeat_out_of_range_pending_group_overlays_clear: [0; 6],
                overlays_alloc: 0,
                overlays: 0,
                c2rust_unnamed: C2Rust_Unnamed_21 {
                    overlay_key: std::ptr::null(),
                },
            },
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            group_names: Vec::new(),
            modmaps: Vec::new(),
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            ctx: std::ptr::null_mut(),
            keymap_info: std::ptr::null(),
        };
        InitSymbolsInfo(
            &raw mut info,
            keymap_info,
            0 as u32,
            &raw mut (*keymap_info).keymap.mods,
        );
        if !file.is_null() {
            HandleSymbolsFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopySymbolsToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearSymbolsInfo(&raw mut info);
                return true;
            }
        }
        ClearSymbolsInfo(&raw mut info);
        return false;
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::keysym_case_mappings::xkb_keysym_to_upper;
use crate::xkb::shared_types::*;
