use crate::xkb::context_priv::{xkb_atom_intern, xkb_atom_text};
use crate::xkb_logf;

use crate::xkb::text::ModMaskText;
use crate::xkb::xkbcomp::expr::{
    ExprResolveLevel, ExprResolveLhs, ExprResolveModMask, ExprResolveString,
};

pub use crate::xkb::keymap_priv::XkbEscapeMapName;
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
    xkb_map_flags, ExprAction, ExprActionList, ExprArrayRef, ExprBinary, ExprBoolean, ExprDef,
    ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym, ExprKeysymList, ExprString,
    ExprUnary, IncludeStmt, KeyTypeDef, ParseCommon, UnknownStatement, VModDef, VarDef, XkbFile,
    _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES,
    FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
    MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR,
    MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
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
pub use crate::xkb::shared_ast_types::{
    pending_computation, safe_map_name, xkb_keymap_info, xkb_parser_strict_flags, FreeXkbFile,
    ReportBadType, ReportShouldBeArray, XkbcompFeatures, XkbcompLookup,
    PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS,
    PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
    PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
    PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_V1_LAX_FLAGS, PARSER_V1_STRICT_FLAGS,
    PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
pub use crate::xkb::shared_types::darray_size_t;
pub use crate::xkb::shared_types::{
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
pub use crate::xkb::shared_types::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use crate::xkb::utils::_steal;
use crate::xkb::utils::cstr_free;
// darray_resize_zero_vec is used instead of darray_resize_zero for Vec fields
pub use crate::xkb::utils::{istrcmp, istreq, strdup_safe};
use crate::xkb::xkbcomp::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
use crate::xkb::xkbcomp::vmod::{HandleVModDef, InitVMods, MergeModSets};
use libc::calloc;
#[derive(Clone)]
pub struct KeyTypesInfo {
    pub name: *mut i8,
    pub errorCount: i32,
    pub include_depth: u32,
    pub types: Vec<KeyTypeInfo>,
    pub mods: xkb_mod_set,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Clone)]
pub struct KeyTypeInfo {
    pub defined: type_field,
    pub merge: merge_mode,
    pub name: xkb_atom_t,
    pub mods: xkb_mod_mask_t,
    pub num_levels: xkb_level_index_t,
    pub entries: Vec<xkb_key_type_entry>,
    pub level_names: Vec<xkb_atom_t>,
}
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
        (*info).name = std::ptr::null_mut();
        (*info).errorCount = 0;
        (*info).include_depth = 0;
        (*info).types = Vec::new();
        (*info).mods = std::mem::zeroed();
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0 as u32);
    }
}
unsafe fn ClearKeyTypeInfo(mut type_0: *mut KeyTypeInfo) {
    unsafe {
        (*type_0).entries.clear();
        (*type_0).level_names.clear();
    }
}
unsafe fn ClearKeyTypesInfo(mut info: *mut KeyTypesInfo) {
    unsafe {
        cstr_free((*info).name);
        for type_0 in (*info).types.iter_mut() {
            ClearKeyTypeInfo(type_0 as *mut KeyTypeInfo);
        }
        (*info).types.clear();
    }
}
unsafe fn FindMatchingKeyType(
    mut info: *mut KeyTypesInfo,
    mut name: xkb_atom_t,
) -> *mut KeyTypeInfo {
    unsafe {
        for type_0 in (*info).types.iter_mut() {
            if type_0.name == name {
                return type_0 as *mut KeyTypeInfo;
            }
        }
        return std::ptr::null_mut();
    }
}
unsafe fn AddKeyType(
    mut info: *mut KeyTypesInfo,
    mut new: *mut KeyTypeInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut old: *mut KeyTypeInfo = std::ptr::null_mut();
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        old = FindMatchingKeyType(info, (*new).name);
        if !old.is_null() {
            if (*new).merge as u32 != MERGE_AUGMENT as u32 {
                if same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32 {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Multiple definitions of the {} key type; Earlier definition ignored\n",
                        XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS
                            as i32,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    );
                }
                ClearKeyTypeInfo(old);
                *old = (*new).clone();
                (*new).entries = Vec::new();
                (*new).level_names = Vec::new();
                return true;
            }
            if same_file {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "[XKB-{:03}] Multiple definitions of the {} key type; Later definition ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                );
            }
            ClearKeyTypeInfo(new);
            return true;
        }
        (*info).types.push((*new).clone());
        return true;
    }
}
unsafe fn MergeIncludedKeyTypes(
    mut into: *mut KeyTypesInfo,
    mut from: *mut KeyTypesInfo,
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
        if (*into).types.len() == 0 {
            (*into).types = std::mem::take(&mut (*from).types);
        } else {
            let mut type_0: *mut KeyTypeInfo = std::ptr::null_mut();
            for i in 0..(*from).types.len() {
                type_0 = (*from).types.as_mut_ptr().add(i);
                (*type_0).merge = merge;
                if !AddKeyType(into, type_0, false) {
                    (*into).errorCount += 1;
                }
            }
            (*from).types.clear();
        };
    }
}
unsafe fn HandleIncludeKeyTypes(
    mut info: *mut KeyTypesInfo,
    mut include: *mut IncludeStmt,
) -> bool {
    unsafe {
        let mut included: KeyTypesInfo = KeyTypesInfo {
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            types: Vec::new(),
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
                name: std::ptr::null_mut(),
                errorCount: 0,
                include_depth: 0,
                types: Vec::new(),
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
                FILE_TYPE_TYPES,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearKeyTypesInfo(&raw mut included);
                return false;
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
        return (*info).errorCount == 0 as i32;
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The modifiers field of a key type is not an array; Illegal array subscript ignored\n",
            );
            return false;
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Key type mask field must be a modifier mask; Key type definition ignored\n",
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as i32,
            );
            return false;
        }
        if (*type_0).defined as u32 & TYPE_FIELD_MASK as u32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Multiple modifier mask definitions for key type {}; Using {}, ignoring {}\n",
                crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*type_0).name)),
                crate::xkb::utils::CStrDisplay(TypeMaskTxt(info, type_0)),
                crate::xkb::utils::CStrDisplay(ModMaskText(
                    (*info).ctx,
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    mods
                )),
            );
            return false;
        }
        (*type_0).mods = mods;
        return true;
    }
}
unsafe fn FindMatchingMapEntry(
    mut type_0: *mut KeyTypeInfo,
    mut mods: xkb_mod_mask_t,
) -> *mut xkb_key_type_entry {
    unsafe {
        for entry in (*type_0).entries.iter_mut() {
            if entry.mods.mods == mods {
                return entry as *mut xkb_key_type_entry;
            }
        }
        return std::ptr::null_mut();
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
        let mut old: *mut xkb_key_type_entry = std::ptr::null_mut();
        old = FindMatchingMapEntry(type_0, (*new).mods.mods);
        if !old.is_null() {
            if report as i32 != 0 && (*old).level != (*new).level {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Multiple map entries for {} in {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                    crate::xkb::utils::CStrDisplay(MapEntryTxt(info, new)),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                    (if clobber as i32 != 0 {
                        (*new).level
                    } else {
                        (*old).level
                    })
                    .wrapping_add(1 as xkb_level_index_t),
                    (if clobber as i32 != 0 {
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
                    XKB_LOG_VERBOSITY_VERBOSE as i32,
                    "[XKB-{:03}] Multiple occurrences of map[{}]= {} in {}; Ignored\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY as i32,
                    crate::xkb::utils::CStrDisplay(MapEntryTxt(info, new)),
                    (*new).level.wrapping_add(1 as xkb_level_index_t),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                );
                return true;
            }
            if clobber {
                if (*new).level >= (*type_0).num_levels {
                    (*type_0).num_levels = (*new).level.wrapping_add(1 as xkb_level_index_t);
                }
                (*old).level = (*new).level;
            }
            return true;
        }
        if (*new).level >= (*type_0).num_levels {
            (*type_0).num_levels = (*new).level.wrapping_add(1 as xkb_level_index_t);
        }
        (*type_0).entries.push(*new);
        return true;
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
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Map entry for modifiers not used by type {}; Using {} instead of {}\n",
                XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Level specifications in a key type must be integer; Ignoring malformed level specification\n",
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32,
            );
            return false;
        }
        entry.preserve.mods = 0 as xkb_mod_mask_t;
        return AddMapEntry(info, type_0, &raw mut entry, true, true);
    }
}
unsafe fn AddPreserve(
    mut info: *mut KeyTypesInfo,
    mut type_0: *mut KeyTypeInfo,
    mut mods: xkb_mod_mask_t,
    mut preserve_mods: xkb_mod_mask_t,
) -> bool {
    unsafe {
        let mut entry: *mut xkb_key_type_entry = std::ptr::null_mut();
        let mut new: xkb_key_type_entry = xkb_key_type_entry {
            level: 0,
            mods: xkb_mods { mods: 0, mask: 0 },
            preserve: xkb_mods { mods: 0, mask: 0 },
        };
        for e in (*type_0).entries.iter_mut() {
            if e.mods.mods != mods {
                continue;
            } else {
                if e.preserve.mods == 0 as xkb_mod_mask_t {
                    e.preserve.mods = preserve_mods;
                    return true;
                }
                if e.preserve.mods == preserve_mods {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_VERBOSE as i32,
                        "[XKB-{:03}] Identical definitions for preserve[{}] in {}; Ignored\n",
                        XKB_WARNING_DUPLICATE_ENTRY as i32,
                        crate::xkb::utils::CStrDisplay(ModMaskText(
                            (*info).ctx,
                            MOD_BOTH,
                            &raw mut (*info).mods,
                            mods
                        )),
                        crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                    );
                    return true;
                }
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as i32,
                    "[XKB-{:03}] Multiple definitions for preserve[{}] in {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES
                        as i32,
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
                        e.preserve.mods,
                    )),
                );
                e.preserve.mods = preserve_mods;
                return true;
            }
        }
        new.level = 0 as xkb_level_index_t;
        new.mods.mods = mods;
        new.preserve.mods = preserve_mods;
        (*type_0).entries.push(new);
        return true;
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
            let mut before: *const i8 = std::ptr::null();
            let mut after: *const i8 = std::ptr::null();
            before = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods);
            mods &= (*type_0).mods;
            after = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods);
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Preserve entry for modifiers not used by the {} type; Index {} converted to {}\n",
                XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE as i32,
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Preserve value in a key type is not a modifier mask; Ignoring preserve[{}] in type {}\n",
                XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as i32,
                crate::xkb::utils::CStrDisplay(ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, mods)),
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
            );
            return false;
        }
        if preserve_mods & !mods != 0 {
            let mut before_0: *const i8 = std::ptr::null();
            let mut after_0: *const i8 = std::ptr::null();
            before_0 = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, preserve_mods);
            preserve_mods &= mods;
            after_0 = ModMaskText((*info).ctx, MOD_BOTH, &raw mut (*info).mods, preserve_mods);
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_BRIEF as i32,
                "[XKB-{:03}] Illegal value for preserve[{}] in type {}; Converted {} to {}\n",
                XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT as i32,
                crate::xkb::utils::CStrDisplay(ModMaskText(
                    (*info).ctx,
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    mods
                )),
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
        if level >= (*type_0).level_names.len() as xkb_level_index_t {
            darray_resize_zero_vec(&mut (*type_0).level_names, (level as usize).wrapping_add(1));
        } else {
            if *(*type_0).level_names.as_ptr().add(level as usize) == name {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_VERBOSE as i32,
                    "[XKB-{:03}] Duplicate names for level {} of key type {}; Ignored\n",
                    XKB_WARNING_DUPLICATE_ENTRY as i32,
                    level.wrapping_add(1 as xkb_level_index_t),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                );
                return true;
            }
            if *(*type_0).level_names.as_ptr().add(level as usize) != XKB_ATOM_NONE as xkb_atom_t {
                let mut old: *const i8 = std::ptr::null();
                let mut new: *const i8 = std::ptr::null();
                old = xkb_atom_text(
                    (*info).ctx,
                    *(*type_0).level_names.as_ptr().add(level as usize),
                );
                new = xkb_atom_text((*info).ctx, name);
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_BRIEF as i32,
                    "[XKB-{:03}] Multiple names for level {} of key type {}; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES as i32,
                    level.wrapping_add(1 as xkb_level_index_t),
                    crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
                    crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 { new } else { old }),
                    crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 { old } else { new }),
                );
                if !clobber {
                    return true;
                }
            }
        }
        *(*type_0).level_names.as_mut_ptr().add(level as usize) = name;
        return true;
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Non-string name for level {} in key type {}; Ignoring illegal level name definition\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                level.wrapping_add(1 as xkb_level_index_t),
                crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*type_0).name)),
            );
            return false;
        }
        return AddLevelName(info, type_0, level, level_name, true);
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
        let mut ok: bool = false;
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
        } else if istreq(field, b"levelname\0".as_ptr() as *const i8) as i32 != 0
            || istreq(field, b"level_name\0".as_ptr() as *const i8) as i32 != 0
        {
            type_field = TYPE_FIELD_LEVEL_NAME;
            ok = SetLevelName(info, type_0, arrayNdx, value);
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown field \"{}\" in key type \"{}\"; Definition ignored\n",
                XKB_ERROR_UNKNOWN_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
                crate::xkb::utils::CStrDisplay(TypeTxt(info, type_0)),
            );
            ok = (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_TYPE_FIELDS as u32 == 0;
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
        let mut ok: bool = true;
        let mut elem: *const i8 = std::ptr::null();
        let mut field: *const i8 = std::ptr::null();
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        while !def.is_null() {
            if !ExprResolveLhs(
                (*info).ctx,
                (*def).name,
                &raw mut elem,
                &raw mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_null() {
                if istreq(elem, b"type\0".as_ptr() as *const i8) {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Support for changing the default type has been removed; Statement \"{}.{}\" ignored.\n",
                        XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT as i32,
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(field),
                    );
                } else {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Cannot set global defaults for \"{}\" element within a key type statement: move statements to the global file scope. Assignment to \"{}.{}\" ignored.\n",
                        XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(elem),
                        crate::xkb::utils::CStrDisplay(field),
                    );
                    ok = false;
                }
            } else if !SetKeyTypeField(info, type_0, field, arrayNdx, (*def).value as *mut ExprDef)
            {
                ok = false;
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
            entries: Vec::new(),
            level_names: Vec::new(),
        };
        if !HandleKeyTypeBody(info, (*def).body, &raw mut type_0)
            || !AddKeyType(info, &raw mut type_0, true)
        {
            (*info).errorCount += 1;
            ClearKeyTypeInfo(&raw mut type_0);
            return false;
        }
        return true;
    }
}
unsafe fn HandleGlobalVar(mut info: *mut KeyTypesInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const i8 = std::ptr::null();
        let mut field: *const i8 = std::ptr::null();
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        } else if !elem.is_null() && istreq(elem, b"type\0".as_ptr() as *const i8) as i32 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Support for changing the default type has been removed; Statement ignored\n",
                XKB_ERROR_WRONG_STATEMENT_TYPE as i32,
            );
            return true;
        } else if !elem.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown element \"{}\"; Value for field \"{}.{}\" ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_STATEMENTS as u32 == 0;
        } else if !field.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS as u32
                == 0;
        }
        return false;
    }
}
unsafe fn HandleKeyTypesFile(mut info: *mut KeyTypesInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        cstr_free((*info).name);
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
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported types {} statement \"{}\"; Ignoring\n",
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
                        "[XKB-{:03}] Key type files may not include other declarations; Ignoring {}\n",
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
                    "[XKB-{:03}] Abandoning keytypes file \"{}\"\n",
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
unsafe fn CopyKeyTypesToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut KeyTypesInfo) -> bool {
    unsafe {
        let num_types: darray_size_t = if (*info).types.len() == 0 {
            1 as darray_size_t
        } else {
            (*info).types.len() as darray_size_t
        };
        let mut types: *mut xkb_key_type =
            calloc(num_types as usize, std::mem::size_of::<xkb_key_type>()) as *mut xkb_key_type;
        if types.is_null() {
            return false;
        }
        if (*info).types.len() == 0 {
            let mut type_0: *mut xkb_key_type =
                types.offset(0 as i32 as isize) as *mut xkb_key_type;
            (*type_0).mods.mods = 0 as xkb_mod_mask_t;
            (*type_0).num_levels = 1 as xkb_level_index_t;
            (*type_0).entries = std::ptr::null_mut();
            (*type_0).num_entries = 0 as darray_size_t;
            (*type_0).name = xkb_atom_intern(
                (*keymap).ctx,
                b"ONE_LEVEL\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
            );
            (*type_0).level_names = std::ptr::null_mut();
            (*type_0).num_level_names = 0 as xkb_level_index_t;
            (*type_0).required = true;
        } else {
            let canonical_types: [xkb_atom_t; 4] = [
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"ONE_LEVEL\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
                ),
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"TWO_LEVEL\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 10]>()).wrapping_sub(1 as usize),
                ),
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"ALPHABETIC\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 11]>()).wrapping_sub(1 as usize),
                ),
                xkb_atom_intern(
                    (*keymap).ctx,
                    b"KEYPAD\0".as_ptr() as *const i8,
                    (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
                ),
            ];
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < num_types {
                let mut def: *mut KeyTypeInfo = (*info).types.as_mut_ptr().add(i as usize);
                let mut type_1: *mut xkb_key_type = types.offset(i as isize) as *mut xkb_key_type;
                (*type_1).name = (*def).name;
                (*type_1).mods.mods = (*def).mods;
                (*type_1).num_levels = (*def).num_levels;
                // Steal level_names Vec buffer
                let mut ln_vec = std::mem::take(&mut (*def).level_names);
                (*type_1).num_level_names = ln_vec.len() as xkb_level_index_t;
                if ln_vec.is_empty() {
                    (*type_1).level_names = std::ptr::null_mut();
                } else {
                    ln_vec.shrink_to_fit();
                    (*type_1).level_names = ln_vec.as_mut_ptr();
                    std::mem::forget(ln_vec);
                }
                // Steal entries Vec buffer
                let mut ent_vec = std::mem::take(&mut (*def).entries);
                (*type_1).num_entries = ent_vec.len() as darray_size_t;
                if ent_vec.is_empty() {
                    (*type_1).entries = std::ptr::null_mut();
                } else {
                    ent_vec.shrink_to_fit();
                    (*type_1).entries = ent_vec.as_mut_ptr();
                    std::mem::forget(ent_vec);
                }
                (*type_1).required = false;
                if (*type_1).num_levels <= 2 as xkb_level_index_t {
                    let mut t: u8 = 0 as u8;
                    while (t as i32)
                        < (std::mem::size_of::<[xkb_atom_t; 4]>())
                            .wrapping_div(std::mem::size_of::<xkb_atom_t>())
                            as u8 as i32
                    {
                        if (*type_1).name == canonical_types[t as usize] {
                            (*type_1).required = true;
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
        return true;
    }
}
pub unsafe fn CompileKeyTypes(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: KeyTypesInfo = KeyTypesInfo {
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            types: Vec::new(),
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
        InitKeyTypesInfo(
            &raw mut info,
            keymap_info,
            0 as u32,
            &raw mut (*keymap_info).keymap.mods,
        );
        if !file.is_null() {
            HandleKeyTypesFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopyKeyTypesToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearKeyTypesInfo(&raw mut info);
                return true;
            }
        }
        ClearKeyTypesInfo(&raw mut info);
        return false;
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
