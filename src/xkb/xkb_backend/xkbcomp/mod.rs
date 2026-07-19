// XKB compiler modules from libxkbcommon

pub mod messages {
    pub use super::super::shared_types::*;
}
pub mod parser;
pub mod symbols;
// ── Prelude: shared imports used by xkbcomp modules ──

pub use super::keymap::xkb_atom_text;
pub use super::keymap::xkb_escape_map_name;

pub use super::shared_types::{
    merge_mode, pending_computation, safe_map_name, stmt_type_to_string, xkb_keymap_info, ExprDef,
    IncludeStmt, ReportBadType, Statement, VModDef, VarDef, XkbFile, XkbcompFeatures,
    XkbcompLookup, FILE_TYPE_COMPAT, FILE_TYPE_KEYCODES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES,
    MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, PARSER_FATAL_ERROR,
    PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
    PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
    PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ASSIGN, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST,
    STMT_EXPR_IDENT, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_UNARY_PLUS,
    STMT_UNKNOWN_COMPOUND,
};

pub use super::shared_types::xkb_keymap;

pub use super::keymap::{ActionTypeText, KeysymText};
pub use super::shared_types::LookupEntry;

pub use self::parser::{ExceedsIncludeMaxDepth, ProcessIncludeFile};

use super::shared_types::xkb_context;
use super::shared_types::{xkb_component_names, xkb_rule_names};
pub fn xkb_components_from_rules_names(
    ctx: &mut xkb_context,
    rmlvo: &xkb_rule_names,
    out: &mut xkb_component_names,
    explicit_layouts: &mut u32,
) -> bool {
    self::parser::xkb_components_from_rules_names(ctx, rmlvo, out, explicit_layouts)
}

pub use self::messages::XKB_ERROR_KEYMAP_COMPILATION_FAILED;
use self::parser::CompileKeymap;
use self::parser::XkbFileFromComponents;
use self::parser::XkbParseString;
pub use super::shared_types::{xkb_file_type_to_string, FILE_TYPE_KEYMAP};

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
