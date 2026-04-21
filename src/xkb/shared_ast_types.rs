//! Shared AST type definitions used across xkbcomp modules.
//!
//! These types were previously duplicated in every file's local `mod ast_h` blocks
//! (a c2rust artifact). This module provides canonical definitions so that all modules
//! can share the same Rust types.

use crate::xkb::shared_types::{xkb_context, xkb_keymap, xkb_overlay_index_t};
use crate::xkb::text::LookupEntry;

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

#[derive(Clone)]

pub struct _IncludeStmt {
    pub merge: merge_mode,
    pub stmt: String,
    pub file: String,
    pub map: String,
    pub modifier: String,
    pub next_incl: Option<Box<_IncludeStmt>>,
}
pub type IncludeStmt = _IncludeStmt;

// ── Expression types ────────────────────────────────────────────────

/// Expression AST node.
pub struct ExprDef {
    pub kind: ExprKind,
}

/// The discriminated payload of an expression node.
pub enum ExprKind {
    String(u32),
    Integer(i64),
    Float,
    Boolean(bool),
    KeyName(u32),
    KeySym(u32),
    Ident(u32),
    FieldRef {
        element: u32,
        field: u32,
    },
    ArrayRef {
        element: u32,
        field: u32,
        entry: Option<Box<ExprDef>>,
    },
    Action {
        name: u32,
        args: Vec<ExprDef>,
    },
    ActionList {
        actions: Vec<ExprDef>,
    },
    KeysymList {
        syms: Vec<u32>,
    },
    EmptyList,
    Binary {
        op: stmt_type,
        left: Option<Box<ExprDef>>,
        right: Option<Box<ExprDef>>,
    },
    Unary {
        op: stmt_type,
        child: Option<Box<ExprDef>>,
    },
}

impl ExprDef {
    /// Returns the `stmt_type` discriminant for backward compatibility.
    pub fn stmt_type(&self) -> stmt_type {
        Self::stmt_type_for_kind(&self.kind)
    }

    /// Returns the `stmt_type` for a given ExprKind (usable before construction).
    pub fn stmt_type_for_kind(kind: &ExprKind) -> stmt_type {
        match kind {
            ExprKind::String(_) => STMT_EXPR_STRING_LITERAL,
            ExprKind::Integer(_) => STMT_EXPR_INTEGER_LITERAL,
            ExprKind::Float => STMT_EXPR_FLOAT_LITERAL,
            ExprKind::Boolean(_) => STMT_EXPR_BOOLEAN_LITERAL,
            ExprKind::KeyName(_) => STMT_EXPR_KEYNAME_LITERAL,
            ExprKind::KeySym(_) => STMT_EXPR_KEYSYM_LITERAL,
            ExprKind::Ident(_) => STMT_EXPR_IDENT,
            ExprKind::FieldRef { .. } => STMT_EXPR_FIELD_REF,
            ExprKind::ArrayRef { .. } => STMT_EXPR_ARRAY_REF,
            ExprKind::Action { .. } => STMT_EXPR_ACTION_DECL,
            ExprKind::ActionList { .. } => STMT_EXPR_ACTION_LIST,
            ExprKind::KeysymList { .. } => STMT_EXPR_KEYSYM_LIST,
            ExprKind::EmptyList => STMT_EXPR_EMPTY_LIST,
            ExprKind::Binary { op, .. } => *op,
            ExprKind::Unary { op, .. } => *op,
        }
    }
}

// Re-export ast_build functions used by consumers via ast_h
pub use crate::xkb::xkbcomp::ast_build::{
    stmt_type_to_operator_char, stmt_type_to_string, xkb_file_type_to_string,
};

// ── Statement definition types ──────────────────────────────────────

pub struct VarDef {
    pub merge: merge_mode,
    pub name: Option<Box<ExprDef>>,
    pub value: Option<Box<ExprDef>>,
}

pub struct VModDef {
    pub merge: merge_mode,
    pub name: u32,
    pub value: Option<Box<ExprDef>>,
}

#[derive(Copy, Clone)]

pub struct KeycodeDef {
    pub merge: merge_mode,
    pub name: u32,
    pub value: i64,
}

#[derive(Copy, Clone)]

pub struct KeyAliasDef {
    pub merge: merge_mode,
    pub alias: u32,
    pub real: u32,
}

pub struct KeyTypeDef {
    pub merge: merge_mode,
    pub name: u32,
    pub body: Vec<VarDef>,
}

pub struct SymbolsDef {
    pub merge: merge_mode,
    pub keyName: u32,
    pub symbols: Vec<VarDef>,
}

pub struct ModMapDef {
    pub merge: merge_mode,
    pub modifier: u32,
    pub keys: Vec<ExprDef>,
}

pub struct GroupCompatDef {
    pub merge: merge_mode,
    pub group: i64,
    pub def: Option<Box<ExprDef>>,
}

pub struct InterpDef {
    pub merge: merge_mode,
    pub sym: u32,
    pub match_0: Option<Box<ExprDef>>,
    pub def: Vec<VarDef>,
}

pub struct LedNameDef {
    pub merge: merge_mode,
    pub virtual_0: bool,
    pub ndx: i64,
    pub name: Option<Box<ExprDef>>,
}

pub struct LedMapDef {
    pub merge: merge_mode,
    pub name: u32,
    pub body: Vec<VarDef>,
}

#[derive(Clone)]

pub struct UnknownStatement {
    pub stmt_type: stmt_type,
    pub name: String,
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

/// Type-safe enum replacing the ParseCommon linked list for XkbFile.defs.
/// Each variant owns the statement data (no raw pointers needed).
pub enum Statement {
    Include(Box<IncludeStmt>),
    Keycode(Box<KeycodeDef>),
    KeyAlias(Box<KeyAliasDef>),
    Expr(Box<ExprDef>),
    Var(Box<VarDef>),
    KeyType(Box<KeyTypeDef>),
    Interp(Box<InterpDef>),
    VMod(Box<VModDef>),
    Symbols(Box<SymbolsDef>),
    ModMap(Box<ModMapDef>),
    GroupCompat(Box<GroupCompatDef>),
    LedMap(Box<LedMapDef>),
    LedName(Box<LedNameDef>),
    Unknown(Box<UnknownStatement>),
    /// Sub-XkbFile (for FILE_TYPE_KEYMAP defs that are themselves XkbFiles)
    XkbFile(Box<XkbFile>),
}

impl Statement {
    /// Get the merge mode from a statement (most types have one).
    pub fn merge(&self) -> merge_mode {
        match self {
            Statement::Include(s) => s.merge,
            Statement::Keycode(s) => s.merge,
            Statement::KeyAlias(s) => s.merge,
            Statement::Var(s) => s.merge,
            Statement::KeyType(s) => s.merge,
            Statement::Interp(s) => s.merge,
            Statement::VMod(s) => s.merge,
            Statement::Symbols(s) => s.merge,
            Statement::ModMap(s) => s.merge,
            Statement::GroupCompat(s) => s.merge,
            Statement::LedMap(s) => s.merge,
            Statement::LedName(s) => s.merge,
            Statement::Unknown(_) | Statement::Expr(_) | Statement::XkbFile(_) => 0,
        }
    }

    /// Get the stmt_type discriminant.
    pub fn stmt_type(&self) -> stmt_type {
        match self {
            Statement::Include(_) => STMT_INCLUDE,
            Statement::Keycode(_) => STMT_KEYCODE,
            Statement::KeyAlias(_) => STMT_ALIAS,
            Statement::Expr(e) => e.stmt_type(),
            Statement::Var(_) => STMT_VAR,
            Statement::KeyType(_) => STMT_TYPE,
            Statement::Interp(_) => STMT_INTERP,
            Statement::VMod(_) => STMT_VMOD,
            Statement::Symbols(_) => STMT_SYMBOLS,
            Statement::ModMap(_) => STMT_MODMAP,
            Statement::GroupCompat(_) => STMT_GROUP_COMPAT,
            Statement::LedMap(_) => STMT_LED_MAP,
            Statement::LedName(_) => STMT_LED_NAME,
            Statement::Unknown(_) => STMT_UNKNOWN,
            Statement::XkbFile(_) => 0, // XkbFile used as sub-file
        }
    }
}

pub struct XkbFile {
    pub name: String,
    pub defs: Vec<Statement>,
    pub file_type: u32,
    pub flags: xkb_map_flags,
}

// ── xkbcomp_priv types (parser/keymap info) ─────────────────────────

pub const PARSER_FATAL_ERROR: u32 = 2;
pub const PARSER_RECOVERABLE_ERROR: u32 = 1;
pub const PARSER_SUCCESS: u32 = 0;

pub const PARSER_V2_LAX_FLAGS: u32 = 0;
pub const PARSER_V2_STRICT_FLAGS: u32 = 16383;
pub const PARSER_V1_LAX_FLAGS: u32 = 16379;
pub const PARSER_V1_STRICT_FLAGS: u32 = 16383;
pub const PARSER_NO_ILLEGAL_ACTION_FIELDS: u32 = 8192;
pub const PARSER_NO_UNKNOWN_ACTION_FIELDS: u32 = 4096;
pub const PARSER_NO_UNKNOWN_ACTION: u32 = 2048;
pub const PARSER_NO_UNKNOWN_KEY_FIELDS: u32 = 1024;
pub const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: u32 = 512;
pub const PARSER_NO_UNKNOWN_LED_FIELDS: u32 = 256;
pub const PARSER_NO_UNKNOWN_INTERPRET_FIELDS: u32 = 128;
pub const PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS: u32 = 64;
pub const PARSER_NO_UNKNOWN_TYPE_FIELDS: u32 = 32;
pub const PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS: u32 = 16;
pub const PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS: u32 = 8;
pub const PARSER_NO_FIELD_VALUE_MISMATCH: u32 = 4;
pub const PARSER_NO_FIELD_TYPE_MISMATCH: u32 = 2;
pub const PARSER_NO_UNKNOWN_STATEMENTS: u32 = 1;
pub const PARSER_NO_STRICT_FLAGS: u32 = 0;

pub struct pending_computation {
    pub expr: Option<Box<ExprDef>>,
    pub computed: bool,
    pub value: u32,
}

pub struct xkb_keymap_info<'a> {
    pub keymap: &'a mut xkb_keymap,
    pub strict: u32,
    pub features: XkbcompFeatures,
    pub lookup: XkbcompLookup,
    pub pending_computations: Vec<pending_computation>,
}

impl<'a> xkb_keymap_info<'a> {
    /// Safe accessor: get a shared reference to the keymap.
    pub fn keymap_ref(&self) -> &xkb_keymap {
        &*self.keymap
    }

    /// Safe accessor: get a mutable reference to the keymap.
    pub fn keymap_mut(&mut self) -> &mut xkb_keymap {
        &mut *self.keymap
    }

    /// Safe accessor: get a shared reference to the context through the keymap.
    pub fn ctx(&self) -> &xkb_context {
        &self.keymap.ctx
    }

    /// Safe accessor: get a mutable reference to the context through the keymap.
    pub fn ctx_mut(&mut self) -> &mut xkb_context {
        &mut self.keymap.ctx
    }
}

/// Lookup tables for group names/masks (was C2Rust_Unnamed_14 in xkbcomp_priv_h).
#[derive(Copy, Clone)]

pub struct XkbcompLookup {
    pub groupIndexNames: [LookupEntry; 3],
    pub groupMaskNames: [LookupEntry; 5],
}

/// Feature flags for keymap compilation (was C2Rust_Unnamed_15 in xkbcomp_priv_h).
#[derive(Copy, Clone)]

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
pub fn safe_map_name(file: &XkbFile) -> &str {
    if file.name.is_empty() {
        "(unnamed map)"
    } else {
        &file.name
    }
}

#[inline]
pub fn ReportNotArray(type_0: &str, field: &str, name: &str) -> bool {
    log::error!(
        "[XKB-{:03}] The {} {} field is not an array; Ignoring illegal assignment in {}\n",
        XKB_ERROR_WRONG_FIELD_TYPE as i32,
        type_0,
        field,
        name
    );
    false
}

#[inline]
pub fn ReportBadType(
    code: xkb_message_code,
    type_0: &str,
    field: &str,
    name: &str,
    wanted: &str,
) -> bool {
    log::error!(
        "[XKB-{:03}] The {} {} field must be a {}; Ignoring illegal assignment in {}\n",
        { code },
        type_0,
        field,
        wanted,
        name
    );
    false
}

#[inline]
pub fn ReportBadField(type_0: &str, field: &str, name: &str) -> bool {
    log::error!(
        "Unknown {} field \"{}\" in {}; Ignoring assignment to unknown field in {}\n",
        type_0,
        field,
        name,
        name
    );
    false
}

#[inline]
pub fn ReportShouldBeArray(type_0: &str, field: &str, name: &str) -> bool {
    log::error!(
        "[XKB-{:03}] Missing subscript for {} {}; Ignoring illegal assignment in {}\n",
        XKB_ERROR_EXPECTED_ARRAY_ENTRY as i32,
        type_0,
        field,
        name
    );
    false
}
