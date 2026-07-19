// XKB compiler modules from libxkbcommon

pub mod action;
pub mod ast_build;
pub mod compat;
pub mod expr;
pub mod include;
pub mod keycodes;
pub mod keymap;
pub mod messages {
    pub use super::super::shared_types::*;
}
pub mod parser;
pub mod rules;
pub mod scanner;
pub mod symbols;
pub mod types;
// ── Prelude: shared imports used by xkbcomp modules ──

pub use super::context::xkb_atom_text;
pub use super::keymap::xkb_escape_map_name;

pub use self::messages::{
    XKB_ERROR_ALLOCATION_ERROR, XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY,
    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE, XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT,
    XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD, XKB_ERROR_INVALID_EXPRESSION_TYPE,
    XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_MODMAP_ENTRY, XKB_ERROR_INVALID_OPERATION,
    XKB_ERROR_INVALID_REAL_MODIFIER, XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT,
    XKB_ERROR_INVALID_VALUE, XKB_ERROR_INVALID_XKB_SYNTAX, XKB_ERROR_OVERLAPPING_OVERLAY,
    XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER, XKB_ERROR_UNKNOWN_ACTION_TYPE,
    XKB_ERROR_UNKNOWN_DEFAULT_FIELD, XKB_ERROR_UNKNOWN_FIELD, XKB_ERROR_UNKNOWN_OPERATOR,
    XKB_ERROR_UNKNOWN_STATEMENT, XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_, XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX,
    XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL, XKB_ERROR_WRONG_FIELD_TYPE, XKB_ERROR_WRONG_STATEMENT_TYPE,
    XKB_WARNING_CANNOT_INFER_KEY_TYPE, XKB_WARNING_CONFLICTING_KEY_ACTION,
    XKB_WARNING_CONFLICTING_KEY_FIELDS, XKB_WARNING_CONFLICTING_KEY_NAME,
    XKB_WARNING_CONFLICTING_KEY_SYMBOL, XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES, XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES, XKB_WARNING_CONFLICTING_MODMAP,
    XKB_WARNING_DUPLICATE_ENTRY, XKB_WARNING_EXTRA_SYMBOLS_IGNORED,
    XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT, XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX,
    XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE, XKB_WARNING_NON_BASE_GROUP_NAME,
    XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE, XKB_WARNING_UNDEFINED_KEYCODE,
    XKB_WARNING_UNDEFINED_KEY_TYPE, XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL,
    XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION, XKB_WARNING_UNSUPPORTED_LEGACY_ACTION,
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD,
};

pub use super::shared_ast_types::{
    merge_mode, pending_computation, safe_map_name, stmt_type_to_string, xkb_keymap_info, ExprDef,
    ExprKind, IncludeStmt, ReportBadType, Statement, VModDef, VarDef, XkbFile, XkbcompFeatures,
    XkbcompLookup, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_KEYCODES, FILE_TYPE_SYMBOLS,
    FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MERGE_AUGMENT, MERGE_DEFAULT,
    MERGE_OVERRIDE, MERGE_REPLACE, PARSER_FATAL_ERROR, PARSER_NO_FIELD_TYPE_MISMATCH,
    PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_UNKNOWN_ACTION,
    PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ASSIGN, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST,
    STMT_EXPR_IDENT, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_UNARY_PLUS,
    STMT_UNKNOWN_COMPOUND,
};

pub use super::shared_types::{
    xkb_action_controls, xkb_action_flags, xkb_explicit_components, xkb_keymap,
    xkb_overlay_index_t, xkb_overlay_mask_t, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X,
    ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK,
    ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE,
    ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION, ACTION_SAME_SCREEN,
    ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET,
    ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE,
    ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE,
    ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_UNLOCK_ON_PRESS, DEFAULT_INTERPRET_KEY_REPEAT,
    DEFAULT_INTERPRET_VMOD, EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS,
    EXPLICIT_TYPES, EXPLICIT_VMODMAP, MATCH_ALL, MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY,
    MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, _ACTION_TYPE_NUM_ENTRIES,
};

pub use super::text::{ActionTypeText, KeysymText, LookupEntry};

pub use self::expr::{
    ExprResolveBoolean, ExprResolveEnum, ExprResolveGroup, ExprResolveLhs, ExprResolveModMask,
    ExprResolveString,
};

pub use self::include::{ExceedsIncludeMaxDepth, ProcessIncludeFile};

pub use self::types::{HandleVModDef, InitVMods, MergeModSets};

use super::shared_types::xkb_context;
use super::shared_types::{xkb_component_names, xkb_rule_names};
pub fn xkb_components_from_rules_names(
    ctx: &mut xkb_context,
    rmlvo: &xkb_rule_names,
    out: &mut xkb_component_names,
    explicit_layouts: &mut u32,
) -> bool {
    self::rules::xkb_components_from_rules_names(ctx, rmlvo, out, explicit_layouts)
}

use self::ast_build::XkbFileFromComponents;
use self::keymap::CompileKeymap;
pub use self::messages::XKB_ERROR_KEYMAP_COMPILATION_FAILED;
use self::scanner::XkbParseString;
pub use super::shared_ast_types::{xkb_file_type_to_string, FILE_TYPE_KEYMAP};

pub use super::shared_types::format_max_groups;

fn compile_keymap_file(keymap: &mut xkb_keymap, file: &mut XkbFile) -> bool {
    if file.file_type != FILE_TYPE_KEYMAP {
        log::error!(
            "[XKB-{:03}] Cannot compile a {} file alone into a keymap\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32,
            xkb_file_type_to_string(file.file_type)
        );
        return false;
    }
    if !CompileKeymap(file, keymap) {
        log::error!(
            "[XKB-{:03}] Failed to compile keymap\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
        );
        return false;
    }
    true
}
pub fn text_v1_keymap_new_from_names(keymap: &mut xkb_keymap, rmlvo: &xkb_rule_names) -> bool {
    let mut ok: bool;
    let mut kccgst: xkb_component_names = xkb_component_names::default();

    log::debug!(
        "Compiling from RMLVO: rules '{}', model '{}', layout '{}', variant '{}', options '{}'\n",
        rmlvo.rules.to_str().unwrap_or(""),
        rmlvo.model.to_str().unwrap_or(""),
        rmlvo.layout.to_str().unwrap_or(""),
        rmlvo.variant.to_str().unwrap_or(""),
        rmlvo.options.to_str().unwrap_or("")
    );
    ok = xkb_components_from_rules_names(
        &mut keymap.ctx,
        rmlvo,
        &mut kccgst,
        &mut keymap.num_groups,
    );
    if !ok {
        log::error!("[XKB-{:03}] Couldn't look up rules '{}', model '{}', layout '{}', variant '{}', options '{}'\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32,
        &rmlvo.rules.to_str().unwrap_or(""),
        &rmlvo.model.to_str().unwrap_or(""),
        &rmlvo.layout.to_str().unwrap_or(""),
        &rmlvo.variant.to_str().unwrap_or(""),
        &rmlvo.options.to_str().unwrap_or(""));
        return false;
    }
    let max_groups: u32 = format_max_groups(keymap.format);
    if keymap.num_groups > max_groups {
        keymap.num_groups = max_groups;
    }
    // Safe conversion of Vec<i8> fields to string slices for logging
    fn vec_i8_to_str(v: &[i8]) -> String {
        let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
        v[..end].iter().map(|&b| b as u8 as char).collect()
    }
    log::debug!(
        "Compiling from KcCGST: keycodes '{}', types '{}', compat '{}', symbols '{}'\n",
        vec_i8_to_str(&kccgst.keycodes),
        vec_i8_to_str(&kccgst.types),
        vec_i8_to_str(&kccgst.compatibility),
        vec_i8_to_str(&kccgst.symbols),
    );
    let file_opt: Option<Box<XkbFile>> = XkbFileFromComponents(&mut keymap.ctx, &kccgst);
    drop(kccgst);
    let Some(mut file) = file_opt else {
        log::error!(
            "[XKB-{:03}] Failed to generate parsed XKB file from components\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
        );
        return false;
    };
    ok = compile_keymap_file(keymap, &mut file);
    ok
}
pub fn text_v1_keymap_new_from_string(keymap: &mut xkb_keymap, input: &[u8]) -> bool {
    let Some(mut xkb_file) = XkbParseString(&mut keymap.ctx, input, "(input string)", "") else {
        log::error!(
            "[XKB-{:03}] Failed to parse input xkb string\n",
            XKB_ERROR_KEYMAP_COMPILATION_FAILED as i32
        );
        return false;
    };
    let ok: bool = compile_keymap_file(keymap, &mut xkb_file);
    ok
}
