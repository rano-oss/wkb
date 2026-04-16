//! Shared AST type definitions used across xkbcomp modules.
//!
//! These types were previously duplicated in every file's local `mod ast_h` blocks
//! (a c2rust artifact). This module provides canonical definitions so that all modules
//! can share the same Rust types.

use crate::xkb::shared_types::{xkb_context, xkb_keymap, xkb_overlay_index_t};
use crate::xkb::text::LookupEntry;
use crate::xkb_logf;

// message_code types needed by Report* inline functions
pub type xkb_message_code = u32;
pub const XKB_ERROR_WRONG_FIELD_TYPE: xkb_message_code = 578;
pub const XKB_ERROR_EXPECTED_ARRAY_ENTRY: xkb_message_code = 77;
pub type xkb_log_verbosity = i32;
pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
pub const XKB_LOG_LEVEL_ERROR: u32 = 20;

// ── File type enum ──────────────────────────────────────────────────

pub const FILE_TYPE_INVALID: u32 = 7;
pub const _FILE_TYPE_NUM_ENTRIES: u32 = 7;
pub const FILE_TYPE_RULES: u32 = 6;
pub const FILE_TYPE_KEYMAP: u32 = 5;
pub const FILE_TYPE_GEOMETRY: u32 = 4;
pub const LAST_KEYMAP_FILE_TYPE: u32 = 3;
pub const FIRST_KEYMAP_FILE_TYPE: u32 = 0;
pub const FILE_TYPE_SYMBOLS: u32 = 3;
pub const FILE_TYPE_COMPAT: u32 = 2;
pub const FILE_TYPE_TYPES: u32 = 1;
pub const FILE_TYPE_KEYCODES: u32 = 0;

// ── Statement type enum ─────────────────────────────────────────────

pub type stmt_type = u32;
pub const _STMT_NUM_VALUES: stmt_type = 37;
pub const STMT_UNKNOWN_COMPOUND: stmt_type = 36;
pub const STMT_UNKNOWN_DECLARATION: stmt_type = 35;
pub const STMT_LED_NAME: stmt_type = 34;
pub const STMT_LED_MAP: stmt_type = 33;
pub const STMT_GROUP_COMPAT: stmt_type = 32;
pub const STMT_MODMAP: stmt_type = 31;
pub const STMT_SYMBOLS: stmt_type = 30;
pub const STMT_VMOD: stmt_type = 29;
pub const STMT_INTERP: stmt_type = 28;
pub const STMT_TYPE: stmt_type = 27;
pub const STMT_VAR: stmt_type = 26;
pub const STMT_EXPR_UNARY_PLUS: stmt_type = 25;
pub const STMT_EXPR_INVERT: stmt_type = 24;
pub const STMT_EXPR_NEGATE: stmt_type = 23;
pub const STMT_EXPR_NOT: stmt_type = 22;
pub const STMT_EXPR_ASSIGN: stmt_type = 21;
pub const STMT_EXPR_DIVIDE: stmt_type = 20;
pub const STMT_EXPR_MULTIPLY: stmt_type = 19;
pub const STMT_EXPR_SUBTRACT: stmt_type = 18;
pub const STMT_EXPR_ADD: stmt_type = 17;
pub const STMT_EXPR_ACTION_LIST: stmt_type = 16;
pub const STMT_EXPR_KEYSYM_LIST: stmt_type = 15;
pub const STMT_EXPR_EMPTY_LIST: stmt_type = 14;
pub const STMT_EXPR_ARRAY_REF: stmt_type = 13;
pub const STMT_EXPR_FIELD_REF: stmt_type = 12;
pub const STMT_EXPR_ACTION_DECL: stmt_type = 11;
pub const STMT_EXPR_IDENT: stmt_type = 10;
pub const STMT_EXPR_KEYSYM_LITERAL: stmt_type = 9;
pub const STMT_EXPR_KEYNAME_LITERAL: stmt_type = 8;
pub const STMT_EXPR_BOOLEAN_LITERAL: stmt_type = 7;
pub const STMT_EXPR_FLOAT_LITERAL: stmt_type = 6;
pub const STMT_EXPR_INTEGER_LITERAL: stmt_type = 5;
pub const STMT_EXPR_STRING_LITERAL: stmt_type = 4;
pub const STMT_ALIAS: stmt_type = 3;
pub const STMT_KEYCODE: stmt_type = 2;
pub const STMT_INCLUDE: stmt_type = 1;
pub const STMT_UNKNOWN: stmt_type = 0;

// ── Merge mode enum ─────────────────────────────────────────────────

pub type merge_mode = u32;
pub const _MERGE_MODE_NUM_ENTRIES: merge_mode = 4;
pub const MERGE_REPLACE: merge_mode = 3;
pub const MERGE_OVERRIDE: merge_mode = 2;
pub const MERGE_AUGMENT: merge_mode = 1;
pub const MERGE_DEFAULT: merge_mode = 0;

// ── Core AST node types ─────────────────────────────────────────────

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ParseCommon {
    pub next: *mut _ParseCommon,
    pub type_0: stmt_type,
}
pub type ParseCommon = _ParseCommon;

#[derive(Clone)]
#[repr(C)]
pub struct _IncludeStmt {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub stmt: *mut i8,
    pub file: *mut i8,
    pub map: *mut i8,
    pub modifier: *mut i8,
    pub next_incl: *mut _IncludeStmt,
}
pub type IncludeStmt = _IncludeStmt;

// ── Expression types ────────────────────────────────────────────────

#[derive(Copy, Clone)]
#[repr(C)]
pub union ExprDef {
    pub common: ParseCommon,
    pub ident: ExprIdent,
    pub string: ExprString,
    pub boolean: ExprBoolean,
    pub integer: ExprInteger,
    pub key_name: ExprKeyName,
    pub keysym: ExprKeySym,
    pub binary: ExprBinary,
    pub unary: ExprUnary,
    pub field_ref: ExprFieldRef,
    pub array_ref: ExprArrayRef,
    pub action: ExprAction,
    pub actions: ExprActionList,
    /// Placeholder for ExprKeysymList (non-Copy due to Vec).
    /// Access via `*(ptr as *mut ExprKeysymList)` instead.
    pub keysym_list_padding: ExprKeysymListPadding,
}

/// Padding type that keeps ExprDef union large enough for ExprKeysymList.
/// ExprKeysymList contains a Vec and is not Copy, so it cannot be a union variant directly.
/// Access ExprKeysymList only through `*mut ExprKeysymList` pointer casts.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprKeysymListPadding {
    pub _pad: [u8; std::mem::size_of::<ExprKeysymList>()],
}

#[derive(Clone)]
#[repr(C)]
pub struct ExprKeysymList {
    pub common: ParseCommon,
    pub syms: Vec<u32>,
}

// Re-export ast_build functions used by consumers via ast_h
pub use crate::xkb::xkbcomp::ast_build::{
    stmt_type_to_operator_char, stmt_type_to_string, xkb_file_type_to_string, FreeXkbFile,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprActionList {
    pub common: ParseCommon,
    pub actions: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprAction {
    pub common: ParseCommon,
    pub name: u32,
    pub args: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprArrayRef {
    pub common: ParseCommon,
    pub element: u32,
    pub field: u32,
    pub entry: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprFieldRef {
    pub common: ParseCommon,
    pub element: u32,
    pub field: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprUnary {
    pub common: ParseCommon,
    pub child: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprBinary {
    pub common: ParseCommon,
    pub left: *mut ExprDef,
    pub right: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprKeySym {
    pub common: ParseCommon,
    pub keysym: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprKeyName {
    pub common: ParseCommon,
    pub key_name: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprInteger {
    pub common: ParseCommon,
    pub ival: i64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprBoolean {
    pub common: ParseCommon,
    pub set: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprString {
    pub common: ParseCommon,
    pub str: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprIdent {
    pub common: ParseCommon,
    pub ident: u32,
}

// ── Statement definition types ──────────────────────────────────────

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VarDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub name: *mut ExprDef,
    pub value: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VModDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub name: u32,
    pub value: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeycodeDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub name: u32,
    pub value: i64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyAliasDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub alias: u32,
    pub real: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyTypeDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub name: u32,
    pub body: *mut VarDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolsDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub keyName: u32,
    pub symbols: *mut VarDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModMapDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub modifier: u32,
    pub keys: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupCompatDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub group: i64,
    pub def: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub sym: u32,
    pub match_0: *mut ExprDef,
    pub def: *mut VarDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedNameDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub virtual_0: bool,
    pub ndx: i64,
    pub name: *mut ExprDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedMapDef {
    pub common: ParseCommon,
    pub merge: merge_mode,
    pub name: u32,
    pub body: *mut VarDef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnknownStatement {
    pub common: ParseCommon,
    pub name: *mut i8,
}

// ── Map flags and XkbFile ───────────────────────────────────────────

pub type xkb_map_flags = u32;
pub const MAP_IS_ALTGR: xkb_map_flags = 128;
pub const MAP_HAS_FN: xkb_map_flags = 64;
pub const MAP_HAS_KEYPAD: xkb_map_flags = 32;
pub const MAP_HAS_MODIFIER: xkb_map_flags = 16;
pub const MAP_HAS_ALPHANUMERIC: xkb_map_flags = 8;
pub const MAP_IS_HIDDEN: xkb_map_flags = 4;
pub const MAP_IS_PARTIAL: xkb_map_flags = 2;
pub const MAP_IS_DEFAULT: xkb_map_flags = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XkbFile {
    pub common: ParseCommon,
    pub name: *mut i8,
    pub defs: *mut ParseCommon,
    pub file_type: u32,
    pub flags: xkb_map_flags,
}

// ── xkbcomp_priv types (parser/keymap info) ─────────────────────────

pub type xkb_parser_error = u32;
pub const PARSER_FATAL_ERROR: xkb_parser_error = 2;
pub const PARSER_RECOVERABLE_ERROR: xkb_parser_error = 1;
pub const PARSER_SUCCESS: xkb_parser_error = 0;

pub type xkb_parser_strict_flags = u32;
pub const PARSER_V2_LAX_FLAGS: xkb_parser_strict_flags = 0;
pub const PARSER_V2_STRICT_FLAGS: xkb_parser_strict_flags = 16383;
pub const PARSER_V1_LAX_FLAGS: xkb_parser_strict_flags = 16379;
pub const PARSER_V1_STRICT_FLAGS: xkb_parser_strict_flags = 16383;
pub const PARSER_NO_ILLEGAL_ACTION_FIELDS: xkb_parser_strict_flags = 8192;
pub const PARSER_NO_UNKNOWN_ACTION_FIELDS: xkb_parser_strict_flags = 4096;
pub const PARSER_NO_UNKNOWN_ACTION: xkb_parser_strict_flags = 2048;
pub const PARSER_NO_UNKNOWN_KEY_FIELDS: xkb_parser_strict_flags = 1024;
pub const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: xkb_parser_strict_flags = 512;
pub const PARSER_NO_UNKNOWN_LED_FIELDS: xkb_parser_strict_flags = 256;
pub const PARSER_NO_UNKNOWN_INTERPRET_FIELDS: xkb_parser_strict_flags = 128;
pub const PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS: xkb_parser_strict_flags = 64;
pub const PARSER_NO_UNKNOWN_TYPE_FIELDS: xkb_parser_strict_flags = 32;
pub const PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS: xkb_parser_strict_flags = 16;
pub const PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS: xkb_parser_strict_flags = 8;
pub const PARSER_NO_FIELD_VALUE_MISMATCH: xkb_parser_strict_flags = 4;
pub const PARSER_NO_FIELD_TYPE_MISMATCH: xkb_parser_strict_flags = 2;
pub const PARSER_NO_UNKNOWN_STATEMENTS: xkb_parser_strict_flags = 1;
pub const PARSER_NO_STRICT_FLAGS: xkb_parser_strict_flags = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pending_computation {
    pub expr: *mut ExprDef,
    pub computed: bool,
    pub value: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_keymap_info {
    pub keymap: *mut xkb_keymap,
    pub strict: xkb_parser_strict_flags,
    pub features: XkbcompFeatures,
    pub lookup: XkbcompLookup,
    pub pending_computations: *mut Vec<pending_computation>,
}

/// Lookup tables for group names/masks (was C2Rust_Unnamed_14 in xkbcomp_priv_h).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XkbcompLookup {
    pub groupIndexNames: [LookupEntry; 3],
    pub groupMaskNames: [LookupEntry; 5],
}

/// Feature flags for keymap compilation (was C2Rust_Unnamed_15 in xkbcomp_priv_h).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XkbcompFeatures {
    pub max_groups: u32,
    pub max_overlays: xkb_overlay_index_t,
    pub controls_name_offset: u8,
    pub group_lock_on_release: bool,
    pub mods_unlock_on_press: bool,
    pub mods_latch_on_press: bool,
    pub overlapping_overlays: bool,
}

// ── Inline helper functions (were duplicated in every xkbcomp_priv_h module) ──

pub const false_0: i32 = 0;

#[inline]
pub unsafe fn safe_map_name(mut file: *mut XkbFile) -> *const i8 {
    unsafe {
        return if !(*file).name.is_null() {
            (*file).name as *const i8
        } else {
            b"(unnamed map)\0".as_ptr() as *const i8
        };
    }
}

#[inline]
pub unsafe fn ReportNotArray(
    mut ctx: *mut xkb_context,
    type_0: &[u8],
    field: &[u8],
    name: &[u8],
) -> bool {
    xkb_logf!(
        ctx,
        XKB_LOG_LEVEL_ERROR,
        XKB_LOG_VERBOSITY_MINIMAL as i32,
        "[XKB-{:03}] The {} {} field is not an array; Ignoring illegal assignment in {}\n",
        XKB_ERROR_WRONG_FIELD_TYPE as i32,
        crate::xkb::utils::ByteSliceDisplay(type_0),
        crate::xkb::utils::ByteSliceDisplay(field),
        crate::xkb::utils::ByteSliceDisplay(name),
    );
    return false;
}

#[inline]
pub unsafe fn ReportBadType(
    mut ctx: *mut xkb_context,
    mut code: xkb_message_code,
    type_0: &[u8],
    field: &[u8],
    name: &[u8],
    wanted: &[u8],
) -> bool {
    xkb_logf!(
        ctx,
        XKB_LOG_LEVEL_ERROR,
        XKB_LOG_VERBOSITY_MINIMAL as i32,
        "[XKB-{:03}] The {} {} field must be a {}; Ignoring illegal assignment in {}\n",
        code as u32,
        crate::xkb::utils::ByteSliceDisplay(type_0),
        crate::xkb::utils::ByteSliceDisplay(field),
        crate::xkb::utils::ByteSliceDisplay(wanted),
        crate::xkb::utils::ByteSliceDisplay(name),
    );
    return false;
}

#[inline]
pub unsafe fn ReportBadField(
    mut ctx: *mut xkb_context,
    type_0: &[u8],
    field: &[u8],
    name: &[u8],
) -> bool {
    xkb_logf!(
        ctx,
        XKB_LOG_LEVEL_ERROR,
        XKB_LOG_VERBOSITY_MINIMAL as i32,
        "Unknown {} field \"{}\" in {}; Ignoring assignment to unknown field in {}\n",
        crate::xkb::utils::ByteSliceDisplay(type_0),
        crate::xkb::utils::ByteSliceDisplay(field),
        crate::xkb::utils::ByteSliceDisplay(name),
        crate::xkb::utils::ByteSliceDisplay(name),
    );
    return false;
}

#[inline]
pub unsafe fn ReportShouldBeArray(
    mut ctx: *mut xkb_context,
    type_0: &[u8],
    field: &[u8],
    name: &[u8],
) -> bool {
    xkb_logf!(
        ctx,
        XKB_LOG_LEVEL_ERROR,
        XKB_LOG_VERBOSITY_MINIMAL as i32,
        "[XKB-{:03}] Missing subscript for {} {}; Ignoring illegal assignment in {}\n",
        XKB_ERROR_EXPECTED_ARRAY_ENTRY as i32,
        crate::xkb::utils::ByteSliceDisplay(type_0),
        crate::xkb::utils::ByteSliceDisplay(field),
        crate::xkb::utils::ByteSliceDisplay(name),
    );
    return false;
}
