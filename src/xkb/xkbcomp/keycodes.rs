use crate::xkb::context_priv::xkb_atom_text;
use crate::xkb_logf;

use crate::xkb::text::KeyNameText;
use crate::xkb::xkbcomp::expr::{ExprResolveInteger, ExprResolveLhs, ExprResolveString};

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
    ExprUnary, IncludeStmt, KeyAliasDef, KeycodeDef, LedNameDef, ParseCommon, UnknownStatement,
    VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES,
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
pub use crate::xkb::shared_ast_types::{
    pending_computation, safe_map_name, xkb_keymap_info, xkb_parser_strict_flags, FreeXkbFile,
    ReportBadType, ReportNotArray, XkbcompFeatures, XkbcompLookup, PARSER_NO_FIELD_TYPE_MISMATCH,
    PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS,
    PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
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
    XKB_KEYCODE_MAX_CONTIGUOUS, XKB_MAX_LEDS,
};
pub use crate::xkb::utils::_steal;
use crate::xkb::utils::cstr_free;
pub use crate::xkb::utils::{istrcmp, istreq, strdup_safe};
use crate::xkb::xkbcomp::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
use libc::calloc;
#[derive(Clone)]
pub struct KeyNamesInfo {
    pub name: *mut i8,
    pub errorCount: i32,
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
#[derive(Clone)]
pub struct KeycodeStore {
    pub min: xkb_keycode_t,
    pub low: Vec<xkb_atom_t>,
    pub high: Vec<HighKeycodeEntry>,
    pub names: Vec<KeycodeMatch>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighKeycodeEntry {
    pub keycode: xkb_keycode_t,
    pub name: xkb_atom_t,
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
#[inline]
unsafe fn keycode_store_init(mut store: *mut KeycodeStore) {
    unsafe {
        (*store).low = Vec::new();
        (*store).high = Vec::new();
        (*store).names = Vec::new();
        (*store).min = XKB_KEYCODE_INVALID as xkb_keycode_t;
    }
}
#[inline]
unsafe fn keycode_store_free(mut store: *mut KeycodeStore) {
    unsafe {
        (&mut (*store).low).clear();
        (&mut (*store).high).clear();
        (&mut (*store).names).clear();
    }
}
#[inline]
unsafe fn keycode_store_update_key(
    mut store: *mut KeycodeStore,
    mut match_0: KeycodeMatch,
    mut name: xkb_atom_t,
) {
    unsafe {
        if (!match_0.c2rust_unnamed.found() || match_0.c2rust_unnamed.is_alias() as i32 != 0) as i64
            != 0
        {
            return;
        } else if match_0.key.low() {
            (&mut (*store).low)[match_0.key.index() as usize] = name;
        } else {
            (&mut (*store).high)[match_0.key.index() as usize].name = name;
        }
        if name >= (&(*store).names).len() as xkb_atom_t {
            darray_resize_zero_vec(&mut (*store).names, (name as usize).wrapping_add(1));
        }
        (&mut (*store).names)[name as usize] = match_0;
    }
}
unsafe fn keycode_store_insert_key(
    mut store: *mut KeycodeStore,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
) -> bool {
    unsafe {
        if name >= (&(*store).names).len() as xkb_atom_t {
            darray_resize_zero_vec(&mut (*store).names, (name as usize).wrapping_add(1));
        }
        if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            if kc >= (&(*store).low).len() as xkb_keycode_t {
                darray_resize_zero_vec(&mut (*store).low, (kc as usize).wrapping_add(1));
            }
            (&mut (*store).low)[kc as usize] = name;
            if kc < (*store).min {
                (*store).min = kc;
            }
            (&mut (*store).names)[name as usize] = KeycodeMatch {
                key: {
                    let mut init = C2Rust_Unnamed_7 {
                        found_low_is_alias_index: [0; 4],
                    };
                    init.set_found(true);
                    init.set_low(true);
                    init.set_is_alias(false);
                    init.set_index(kc as darray_size_t);
                    init
                },
            };
        } else {
            let idx: darray_size_t = (&(*store).high).len() as darray_size_t;
            if idx != 0
                && (&(*store).high)[(idx.wrapping_sub(1 as darray_size_t)) as usize].keycode > kc
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
                    let entry: &HighKeycodeEntry = &(&(*store).high)[mid as usize];
                    if entry.keycode < kc {
                        lower = mid.wrapping_add(1 as darray_size_t);
                    } else if entry.keycode > kc {
                        upper = mid;
                    } else {
                    }
                }
                {
                    let high_ptr = (*store).high.as_mut_ptr();
                    let high_len = (&(*store).high).len() as darray_size_t;
                    let mut entry_0: *mut HighKeycodeEntry = high_ptr.offset(lower as isize);
                    while entry_0 < high_ptr.offset(high_len as isize) {
                        let ref mut c2rust_fresh4 =
                            (&mut (*store).names)[(*entry_0).name as usize].key;
                        (*c2rust_fresh4).set_index((*c2rust_fresh4).index() + 1 as darray_size_t);
                        entry_0 = entry_0.offset(1);
                    }
                }
                let __index: darray_size_t = lower;
                (&mut (*store).high).insert(
                    __index as usize,
                    HighKeycodeEntry {
                        keycode: kc,
                        name: name,
                    },
                );
                (&mut (*store).names)[name as usize] = KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true);
                        init.set_low(false);
                        init.set_is_alias(false);
                        init.set_index(lower);
                        init
                    },
                };
            } else {
                (&mut (*store).high).push(HighKeycodeEntry {
                    keycode: kc,
                    name: name,
                });
                (&mut (*store).names)[name as usize] = KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true);
                        init.set_low(false);
                        init.set_is_alias(false);
                        init.set_index(idx);
                        init
                    },
                };
            }
            if (&(*store).low).len() == 0 {
                (*store).min = (&(*store).high)[0].keycode;
            }
        }
        return true;
    }
}
#[inline]
unsafe fn keycode_store_insert_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        if alias >= (&(*store).names).len() as xkb_atom_t {
            darray_resize_zero_vec(&mut (*store).names, (alias as usize).wrapping_add(1));
        }
        (&mut (*store).names)[alias as usize] = KeycodeMatch {
            alias: {
                let mut init = C2Rust_Unnamed_6 {
                    found_c2rust_unnamed_is_alias_real: [0; 4],
                };
                init.set_found(true);
                init.set_c2rust_unnamed(true);
                init.set_is_alias(real != 0);
                init.set_real(real);
                init
            },
        };
        return true;
    }
}
#[inline]
unsafe fn keycode_store_update_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        let ref mut c2rust_fresh3 = (&mut (*store).names)[alias as usize].alias;
        (*c2rust_fresh3).set_real(real as xkb_atom_t);
        return true;
    }
}
#[inline]
unsafe fn keycode_store_delete_name(mut store: *const KeycodeStore, mut name: xkb_atom_t) {
    unsafe {
        if (name as usize) < (&(*store).names).len() {
            let ref mut c2rust_fresh5 =
                (&mut (*(store as *mut KeycodeStore)).names)[name as usize].c2rust_unnamed;
            (*c2rust_fresh5).set_found((false) as bool);
        }
    }
}
unsafe fn keycode_store_delete_key(mut store: *mut KeycodeStore, match_0: KeycodeMatch) {
    unsafe {
        if (!match_0.c2rust_unnamed.found() || match_0.c2rust_unnamed.is_alias() as i32 != 0) as i64
            != 0
        {
            return;
        } else if match_0.key.low() {
            let low_name = (&(*store).low)[match_0.key.index() as usize];
            let ref mut c2rust_fresh1 = (&mut (*store).names)[low_name as usize].c2rust_unnamed;
            (*c2rust_fresh1).set_found((false) as bool);
            if match_0.key.index().wrapping_add(1 as u32) == (&(*store).low).len() as darray_size_t
            {
                if (*store).min == match_0.key.index() as xkb_keycode_t {
                    (&mut (*store).low).clear();
                } else {
                    let mut idx: darray_size_t = match_0.key.index();
                    while idx > 0 as darray_size_t {
                        if (&(*store).low)[(idx.wrapping_sub(1 as darray_size_t)) as usize]
                            != XKB_ATOM_NONE as xkb_atom_t
                        {
                            (&mut (*store).low).truncate(idx as usize);
                            break;
                        } else {
                            idx = idx.wrapping_sub(1);
                        }
                    }
                }
            } else {
                (&mut (*store).low)[match_0.key.index() as usize] = XKB_ATOM_NONE as xkb_atom_t;
            }
        } else {
            let high_name = (&(*store).high)[match_0.key.index() as usize].name;
            let ref mut c2rust_fresh2 = (&mut (*store).names)[high_name as usize].c2rust_unnamed;
            (*c2rust_fresh2).set_found((false) as bool);
            let __index: darray_size_t = match_0.key.index();
            (&mut (*store).high).remove(__index as usize);
            {
                let names_ptr = (*store).names.as_mut_ptr();
                let names_len = (&(*store).names).len();
                let mut entry: *mut KeycodeMatch = names_ptr;
                while entry < names_ptr.add(names_len) {
                    if (*entry).c2rust_unnamed.found() as i32 != 0
                        && !(*entry).c2rust_unnamed.is_alias()
                        && !(*entry).key.low()
                        && (*entry).key.index() as i32 > match_0.key.index() as i32
                    {
                        (*entry)
                            .key
                            .set_index((*entry).key.index() - 1 as darray_size_t);
                    }
                    entry = entry.offset(1);
                }
            }
        }
        if (&(*store).low).len() == 0 {
            (*store).min = if (&(*store).high).len() == 0 {
                XKB_KEYCODE_INVALID as xkb_keycode_t
            } else {
                (&(*store).high)[0].keycode
            };
        } else {
            let mut kc: xkb_keycode_t = (*store).min;
            while kc < (&(*store).low).len() as xkb_keycode_t {
                if (&(*store).low)[kc as usize] != XKB_ATOM_NONE as xkb_atom_t {
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
        if !match_0.c2rust_unnamed.found() || match_0.c2rust_unnamed.is_alias() as i32 != 0 {
            return XKB_KEYCODE_INVALID as xkb_keycode_t;
        } else if match_0.key.low() {
            return match_0.key.index() as xkb_keycode_t;
        } else {
            return (&(*store).high)[match_0.key.index() as usize].keycode;
        };
    }
}
#[inline]
unsafe fn keycode_store_get_key_name(
    mut store: *const KeycodeStore,
    mut match_0: KeycodeMatch,
) -> xkb_atom_t {
    unsafe {
        if !match_0.c2rust_unnamed.found() || match_0.c2rust_unnamed.is_alias() as i32 != 0 {
            return XKB_ATOM_NONE as xkb_atom_t;
        } else if match_0.key.low() {
            return (&(*store).low)[match_0.key.index() as usize];
        } else {
            return (&(*store).high)[match_0.key.index() as usize].name;
        };
    }
}
unsafe fn keycode_store_lookup_keycode(
    mut store: *const KeycodeStore,
    mut kc: xkb_keycode_t,
) -> KeycodeMatch {
    unsafe {
        if kc < (&(*store).low).len() as xkb_keycode_t {
            return KeycodeMatch {
                key: {
                    let mut init = C2Rust_Unnamed_7 {
                        found_low_is_alias_index: [0; 4],
                    };
                    init.set_found(true);
                    init.set_low(true);
                    init.set_is_alias(false);
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
                    init.set_found(false);
                    init.set_c2rust_unnamed(false);
                    init
                },
            };
        }
        let mut lower: darray_size_t = 0 as darray_size_t;
        let mut upper: darray_size_t = (&(*store).high).len() as darray_size_t;
        while lower < upper {
            let mid: darray_size_t = lower.wrapping_add(
                upper
                    .wrapping_sub(1 as darray_size_t)
                    .wrapping_sub(lower)
                    .wrapping_div(2 as darray_size_t),
            );
            let entry: &HighKeycodeEntry = &(&(*store).high)[mid as usize];
            if entry.keycode < kc {
                lower = mid.wrapping_add(1 as darray_size_t);
            } else if entry.keycode > kc {
                upper = mid;
            } else {
                return KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true);
                        init.set_low(false);
                        init.set_is_alias(false);
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
                init.set_found(false);
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
        if name >= (&(*store).names).len() as xkb_atom_t {
            return KeycodeMatch {
                c2rust_unnamed: {
                    let mut init = C2Rust_Unnamed_8 {
                        found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                    };
                    init.set_found(false);
                    init.set_c2rust_unnamed(false);
                    init
                },
            };
        } else {
            return (&(*store).names)[name as usize];
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
        return std::ptr::null_mut();
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
        let mut old: *mut LedNameInfo = std::ptr::null_mut();
        let replace: bool = (*new).merge as u32 != MERGE_AUGMENT as u32;
        old = FindLedByName(info, (*new).name, &raw mut old_idx);
        if !old.is_null() {
            if old_idx == new_idx {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Multiple indicators named \"{}\"; Identical definitions ignored\n",
                        crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    );
                }
                return true;
            }
            if report {
                let mut use_0: xkb_led_index_t = if replace as i32 != 0 {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                };
                let mut ignore: xkb_led_index_t = if replace as i32 != 0 {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple indicators named {}; Using {}, ignoring {}\n",
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    use_0,
                    ignore,
                );
            }
            if replace {
                (*old).name = XKB_ATOM_NONE as xkb_atom_t;
            } else {
                return true;
            }
        }
        if new_idx >= (*info).num_led_names {
            (*info).num_led_names = new_idx.wrapping_add(1 as xkb_led_index_t);
        }
        old = (&raw mut (*info).led_names as *mut LedNameInfo).offset(new_idx as isize)
            as *mut LedNameInfo;
        if (*old).name != XKB_ATOM_NONE as xkb_atom_t {
            if report {
                let use_1: xkb_atom_t = if replace as i32 != 0 {
                    (*new).name
                } else {
                    (*old).name
                };
                let ignore_0: xkb_atom_t = if replace as i32 != 0 {
                    (*old).name
                } else {
                    (*new).name
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple names for indicator {}; Using {}, ignoring {}\n",
                    new_idx.wrapping_add(1 as xkb_led_index_t),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, use_1)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, ignore_0)),
                );
            }
            if replace {
                *old = *new;
            }
            return true;
        }
        *old = *new;
        return true;
    }
}
unsafe fn ClearKeyNamesInfo(mut info: *mut KeyNamesInfo) {
    unsafe {
        cstr_free((*info).name);
        keycode_store_free(&raw mut (*info).keycodes);
    }
}
unsafe fn InitKeyNamesInfo(
    mut info: *mut KeyNamesInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
) {
    unsafe {
        std::ptr::write(
            info,
            KeyNamesInfo {
                name: std::ptr::null_mut(),
                errorCount: 0,
                include_depth: 0,
                keycodes: KeycodeStore {
                    min: XKB_KEYCODE_INVALID as xkb_keycode_t,
                    low: Vec::new(),
                    high: Vec::new(),
                    names: Vec::new(),
                },
                led_names: [LedNameInfo {
                    merge: MERGE_DEFAULT,
                    name: 0,
                }; 32],
                num_led_names: 0,
                ctx: std::ptr::null_mut(),
                keymap_info: std::ptr::null(),
            },
        );
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
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
            let clobber: bool = merge as u32 != MERGE_AUGMENT as u32;
            if match_name.c2rust_unnamed.is_alias() {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Key name {} already assigned to an alias; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"key\0".as_ptr() as *const i8
                        } else {
                            b"alias\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"alias\0".as_ptr() as *const i8
                        } else {
                            b"key\0".as_ptr() as *const i8
                        }),
                    );
                }
                if clobber {
                    keycode_store_delete_name(&raw mut (*info).keycodes, name);
                    match_name.c2rust_unnamed.set_found((false) as bool);
                } else {
                    return true;
                }
            } else {
                let old_kc: xkb_keycode_t =
                    keycode_store_get_keycode(&raw mut (*info).keycodes, match_name)
                        as xkb_keycode_t;
                if old_kc != kc {
                    if report {
                        let use_0: xkb_keycode_t = if clobber as i32 != 0 { kc } else { old_kc };
                        let ignore: xkb_keycode_t = if clobber as i32 != 0 { old_kc } else { kc };
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Key name {} assigned to multiple keys; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                            use_0,
                            ignore,
                        );
                    }
                    if clobber {
                        keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                    } else {
                        return true;
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
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Multiple identical key name definitions; Later occurrences of \"{} = {}\" ignored\n",
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, old_name)),
                        kc,
                    );
                }
                return true;
            }
            let clobber_0: bool = merge as u32 != MERGE_AUGMENT as u32;
            if report {
                let kname: *const i8 = KeyNameText((*info).ctx, name) as *const i8;
                let old_kname: *const i8 = KeyNameText((*info).ctx, old_name) as *const i8;
                let use_1: *const i8 = if clobber_0 as i32 != 0 {
                    kname
                } else {
                    old_kname
                };
                let ignore_0: *const i8 = if clobber_0 as i32 != 0 {
                    old_kname
                } else {
                    kname
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot add keycode\n",
                XKB_ERROR_ALLOCATION_ERROR as i32,
            );
            return false;
        }
        return true;
    }
}
unsafe fn MergeKeycodeStores(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (&(*into).keycodes.low).len() == 0
            && (&(*into).keycodes.high).len() == 0
            && (&(*into).keycodes.names).len() == 0
        {
            (*into).keycodes = std::mem::replace(
                &mut (*from).keycodes,
                KeycodeStore {
                    min: XKB_KEYCODE_INVALID as xkb_keycode_t,
                    low: Vec::new(),
                    high: Vec::new(),
                    names: Vec::new(),
                },
            );
        } else {
            let mut kc: xkb_keycode_t = (*from).keycodes.min;
            while kc < (&(*from).keycodes.low).len() as xkb_keycode_t {
                let name: xkb_atom_t = (&(*from).keycodes.low)[kc as usize];
                if !(name == XKB_ATOM_NONE as xkb_atom_t) {
                    if !AddKeyName(into, kc, name, merge, report) {
                        (*into).errorCount += 1;
                    }
                }
                kc = kc.wrapping_add(1);
            }
            {
                let high_ptr = (*from).keycodes.high.as_ptr();
                let high_len = (&(*from).keycodes.high).len();
                let mut new: *const HighKeycodeEntry = high_ptr;
                while new < high_ptr.add(high_len) {
                    if !AddKeyName(into, (*new).keycode, (*new).name, merge, report) {
                        (*into).errorCount += 1;
                    }
                    new = new.offset(1);
                }
            }
            {
                let names_ptr = (*from).keycodes.names.as_ptr();
                let names_len = (&(*from).keycodes.names).len();
                let mut match_0: *const KeycodeMatch = std::ptr::null();
                let mut alias: xkb_atom_t = 0;
                if names_len > 0 {
                    alias = 0 as xkb_atom_t;
                    match_0 = names_ptr;
                    while (alias as usize) < names_len {
                        if !(!(*match_0).c2rust_unnamed.found()
                            || !(*match_0).c2rust_unnamed.is_alias())
                        {
                            let def: KeyAliasDef = KeyAliasDef {
                                common: _ParseCommon {
                                    next: std::ptr::null_mut(),
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
        if (*from).errorCount > 0 as i32 {
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
                    if !AddLedName(into, false, ledi, idx, report) {
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
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: 0,
                low: Vec::new(),
                high: Vec::new(),
                names: Vec::new(),
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: std::ptr::null_mut(),
            keymap_info: std::ptr::null(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as i32;
            return false;
        }
        InitKeyNamesInfo(&raw mut included, (*info).keymap_info, 0 as u32);
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyNamesInfo = KeyNamesInfo {
                name: std::ptr::null_mut(),
                errorCount: 0,
                include_depth: 0,
                keycodes: KeycodeStore {
                    min: 0,
                    low: Vec::new(),
                    high: Vec::new(),
                    names: Vec::new(),
                },
                led_names: [LedNameInfo {
                    merge: MERGE_DEFAULT,
                    name: 0,
                }; 32],
                num_led_names: 0,
                ctx: std::ptr::null_mut(),
                keymap_info: std::ptr::null(),
            };
            let mut file: *mut XkbFile = std::ptr::null_mut();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_KEYCODES,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearKeyNamesInfo(&raw mut included);
                return false;
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
        return (*info).errorCount == 0 as i32;
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Illegal keycode {}: must be between 0..{}; Key ignored\n",
                (*stmt).value,
                (0xffffffff as u32).wrapping_sub(1 as u32),
            );
            return false;
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
            let clobber: bool = (*def).merge as u32 != MERGE_AUGMENT as u32;
            if match_name.c2rust_unnamed.is_alias() {
                if (*def).real == match_name.alias.real() {
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Alias of {} for {} declared more than once; First definition ignored\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).real)),
                        );
                    }
                } else {
                    let use_0: xkb_atom_t = if clobber as i32 != 0 {
                        (*def).real
                    } else {
                        match_name.alias.real()
                    };
                    let ignore: xkb_atom_t = if clobber as i32 != 0 {
                        match_name.alias.real()
                    } else {
                        (*def).real
                    };
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Multiple definitions for alias {}; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, use_0)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, ignore)),
                        );
                    }
                    keycode_store_update_alias(&raw mut (*info).keycodes, (*def).alias, use_0);
                }
                return true;
            } else {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Alias name {} already assigned to a real key; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"alias\0".as_ptr() as *const i8
                        } else {
                            b"key\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"key\0".as_ptr() as *const i8
                        } else {
                            b"alias\0".as_ptr() as *const i8
                        }),
                    );
                }
                if clobber {
                    keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                } else {
                    return true;
                }
            }
        }
        return keycode_store_insert_alias(&raw mut (*info).keycodes, (*def).alias, (*def).real);
    }
}
unsafe fn HandleKeyNameVar(mut info: *mut KeyNamesInfo, mut stmt: *mut VarDef) -> bool {
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
        }
        if !elem.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot set global defaults for \"{}\" element; Assignment to \"{}.{}\" ignored\n",
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as u32
                == 0;
        }
        if !istreq(field, b"minimum\0".as_ptr() as *const i8)
            && !istreq(field, b"maximum\0".as_ptr() as *const i8)
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as u32
                == 0;
        }
        if !arrayNdx.is_null() {
            ReportNotArray(
                (*info).ctx,
                b"keycodes\0".as_ptr() as *const i8,
                field,
                b"defaults\0".as_ptr() as *const i8,
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                == 0;
        }
        let mut val: i64 = 0 as i64;
        if !ExprResolveInteger((*info).ctx, (*stmt).value, &raw mut val)
            || val < 0 as i64
            || val > u32::MAX as i64
        {
            ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"keycodes\0".as_ptr() as *const i8,
                field,
                b"defaults\0".as_ptr() as *const i8,
                b"integer 0..0xfffffffe\0".as_ptr() as *const i8,
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                == 0;
        }
        return true;
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
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Illegal indicator index ({}) specified; must be between 1 .. {}; Ignored\n",
                (*def).ndx,
                (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8 as usize) as xkb_led_index_t,
            );
            return false;
        }
        let mut name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, (*def).name, &raw mut name) {
            let mut buf: [i8; 20] = [0; 20];
            crate::xkb::utils::snprintf_args(
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 20]>(),
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
            true,
            &raw mut ledi,
            ((*def).ndx as xkb_led_index_t).wrapping_sub(1 as xkb_led_index_t),
            report,
        );
    }
}
unsafe fn HandleKeycodesFile(mut info: *mut KeyNamesInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let report_same_file: bool = verbosity > 0 as i32;
        let report_include: bool = verbosity > 7 as i32;
        cstr_free((*info).name);
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
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported keycodes {} statement \"{}\"; Ignoring\n",
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
                        "Keycode files may define key and indicator names only; Ignoring {}\n",
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
        if (&(*info).keycodes.low).len() == 0 && (&(*info).keycodes.high).len() == 0 {
            (*keymap).min_key_code = 8 as xkb_keycode_t;
            (*keymap).max_key_code = 255 as xkb_keycode_t;
            (*keymap).num_keys_low = (*keymap).max_key_code.wrapping_add(1 as xkb_keycode_t);
            (*keymap).num_keys = (*keymap).num_keys_low;
        } else {
            (*keymap).min_key_code = (*info).keycodes.min;
            (*keymap).max_key_code = if (&(*info).keycodes.high).len() == 0 {
                ((&(*info).keycodes.low).len() as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t)
            } else {
                (&(*info).keycodes.high)[(&(*info).keycodes.high).len().wrapping_sub(1)].keycode
            };
            (*keymap).num_keys_low = (&(*info).keycodes.low).len() as xkb_keycode_t;
            (*keymap).num_keys = (*keymap)
                .num_keys_low
                .wrapping_add((&(*info).keycodes.high).len() as xkb_keycode_t);
        }
        let keys: *mut xkb_key =
            calloc((*keymap).num_keys as usize, std::mem::size_of::<xkb_key>()) as *mut xkb_key;
        if keys.is_null() {
            (*keymap).num_keys = 0 as xkb_keycode_t;
            (*keymap).max_key_code = XKB_KEYCODE_INVALID as xkb_keycode_t;
            (*keymap).min_key_code = (*keymap).max_key_code;
            return false;
        }
        let mut kc: xkb_keycode_t = (*keymap).min_key_code;
        while kc < (*keymap).num_keys_low {
            (*keys.offset(kc as isize)).keycode = kc;
            kc = kc.wrapping_add(1);
        }
        let mut kc_0: xkb_keycode_t = (*info).keycodes.min;
        while kc_0 < (&(*info).keycodes.low).len() as xkb_keycode_t {
            (*keys.offset(kc_0 as isize)).name = (&(*info).keycodes.low)[kc_0 as usize];
            kc_0 = kc_0.wrapping_add(1);
        }
        let mut idx: xkb_keycode_t = (*keymap).num_keys_low;
        {
            let high_ptr = (*info).keycodes.high.as_ptr();
            let high_len = (&(*info).keycodes.high).len();
            let mut entry: *const HighKeycodeEntry = high_ptr;
            while entry < high_ptr.add(high_len) {
                (*keys.offset(idx as isize)).keycode = (*entry).keycode;
                (*keys.offset(idx as isize)).name = (*entry).name;
                idx = idx.wrapping_add(1);
                entry = entry.offset(1);
            }
        }
        (*keymap).keys = keys;
        return true;
    }
}
unsafe fn CopyKeycodeNameLUT(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
    unsafe {
        let names_len = (&(*info).keycodes.names).len();
        let names_ptr = (*info).keycodes.names.as_mut_ptr();
        {
            let mut match_0: *mut KeycodeMatch = std::ptr::null_mut();
            let mut name: xkb_atom_t = 0;
            if names_len > 0 {
                name = 0 as xkb_atom_t;
                match_0 = names_ptr;
                while (name as usize) < names_len {
                    if (*match_0).c2rust_unnamed.found() {
                        if (*match_0).c2rust_unnamed.is_alias() {
                            let match_real: KeycodeMatch = keycode_store_lookup_name(
                                &raw mut (*info).keycodes,
                                (*match_0).alias.real(),
                            )
                                as KeycodeMatch;
                            if !match_real.c2rust_unnamed.found() {
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_DETAILED as i32,
                                    "[XKB-{:03}] Attempt to alias {} to non-existent key {}; Ignored\n",
                                    XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                    crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                                    crate::xkb::utils::CStrDisplay(KeyNameText(
                                        (*info).ctx,
                                        (*match_0).alias.real()
                                    )),
                                );
                                (*match_0).c2rust_unnamed.set_found((false) as bool);
                            } else {
                            }
                        } else if !(*match_0).key.low() {
                            (*match_0).key.set_index(
                                (*match_0).key.index()
                                    + (*keymap).num_keys_low as u32 as darray_size_t,
                            );
                        }
                    }
                    name = name.wrapping_add(1);
                    match_0 = match_0.offset(1);
                }
            }
        }
        if names_len > 0 {
            // Shrink the Vec to exact size, then steal the allocation
            (&mut (*info).keycodes.names).shrink_to_fit();
            let stolen_ptr = (*info).keycodes.names.as_mut_ptr();
            let stolen_len = (&(*info).keycodes.names).len();
            // Prevent Vec from freeing the allocation
            std::mem::forget(std::mem::replace(&mut (*info).keycodes.names, Vec::new()));
            (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names = stolen_len as darray_size_t;
            (*keymap).c2rust_unnamed.c2rust_unnamed.key_names = stolen_ptr;
        } else {
            (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names = 0 as darray_size_t;
            (*keymap).c2rust_unnamed.c2rust_unnamed.key_names = std::ptr::null_mut();
        }
        return true;
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
        return true;
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
            return false;
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
        return true;
    }
}
pub unsafe fn CompileKeycodes(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: KeyNamesInfo = KeyNamesInfo {
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: 0,
                low: Vec::new(),
                high: Vec::new(),
                names: Vec::new(),
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: std::ptr::null_mut(),
            keymap_info: std::ptr::null(),
        };
        InitKeyNamesInfo(&raw mut info, keymap_info, 0 as u32);
        if !file.is_null() {
            HandleKeycodesFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopyKeyNamesInfoToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearKeyNamesInfo(&raw mut info);
                return true;
            }
        }
        ClearKeyNamesInfo(&raw mut info);
        return false;
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
