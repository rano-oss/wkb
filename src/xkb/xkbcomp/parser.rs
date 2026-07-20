// Safe parser.rs
#![allow(
    non_upper_case_globals,
    dead_code,
    non_snake_case,
    non_camel_case_types,
    clippy::vec_box,
    clippy::wrong_self_convention,
    clippy::too_many_arguments,
    clippy::type_complexity,
    unused_assignments
)]
// LALR(1) parser for XKB, converted from bison-generated C via c2rust
use super::super::keymap::xkb_escape_map_name;
use super::super::keymap::xkb_keymap_key_get_syms_by_level_ref;
use super::super::keysym::utf32_to_keysym;
use super::super::keysym::{xkb_keysym_from_name, xkb_keysym_is_deprecated};
use super::super::shared_types::*;
use super::super::shared_types::{
    parse_dec_u32, parse_dec_u64, parse_hex_u32, parse_hex_u64, utf8_next_code_point_safe,
    INVALID_UTF8_CODE_POINT,
};

use super::super::shared_types::{
    safe_map_name, ExprDef, ExprKind, GroupCompatDef, IncludeStmt, InterpDef, KeyAliasDef,
    KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, MergeMode, ModMapDef, Statement, StmtType,
    SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, XkbMapFlags, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS,
    FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC,
    MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN,
    MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_EXPR_ADD,
    STMT_EXPR_ASSIGN, STMT_EXPR_DIVIDE, STMT_EXPR_INVERT, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, _STMT_NUM_VALUES,
};

pub(crate) use super::super::keymap::mod_mask_get_effective;
use super::super::keymap::{format_control_names_offset, GROUP_LAST_INDEX_NAME};
use super::super::keymap::{ActionTypeText, KeysymText};
pub(crate) use super::symbols::CompileCompatMap;
pub(crate) use super::symbols::CompileKeyTypes;
pub(crate) use super::symbols::CompileKeycodes;
pub(crate) use super::symbols::CompileSymbols;
use super::symbols::{ExprResolveGroup, ExprResolveGroupMask};

pub(crate) const XKB_KEY_VoidSymbol: i32 = 0xffffff_i32;
pub(crate) const XKB_KEY_0: i32 = 0x30_i32;
pub(crate) const XKB_KEY_section: i32 = 0xa7_i32;
pub(crate) const XKB_KEYSYM_MIN: i32 = 0;

// ── YYSYMBOL constants ──────────────────────────────────────────────
pub(crate) const YYSYMBOL_MapName: i32 = 148;
pub(crate) const YYSYMBOL_OptMapName: i32 = 147;
pub(crate) const YYSYMBOL_String: i32 = 146;
pub(crate) const YYSYMBOL_Ident: i32 = 145;
pub(crate) const YYSYMBOL_KeyCode: i32 = 144;
pub(crate) const YYSYMBOL_Integer: i32 = 143;
pub(crate) const YYSYMBOL_Float: i32 = 142;
pub(crate) const YYSYMBOL_Number: i32 = 141;
pub(crate) const YYSYMBOL_SignedNumber: i32 = 140;
pub(crate) const YYSYMBOL_KeySymLit: i32 = 139;
pub(crate) const YYSYMBOL_KeySym: i32 = 138;
pub(crate) const YYSYMBOL_KeySyms: i32 = 137;
pub(crate) const YYSYMBOL_NonEmptyKeySyms: i32 = 136;
pub(crate) const YYSYMBOL_KeySymList: i32 = 135;
pub(crate) const YYSYMBOL_MultiKeySymList: i32 = 134;
pub(crate) const YYSYMBOL_Terminal: i32 = 133;
pub(crate) const YYSYMBOL_OptTerminal: i32 = 132;
pub(crate) const YYSYMBOL_Lhs: i32 = 131;
pub(crate) const YYSYMBOL_Action: i32 = 130;
pub(crate) const YYSYMBOL_Actions: i32 = 129;
pub(crate) const YYSYMBOL_NonEmptyActions: i32 = 128;
pub(crate) const YYSYMBOL_ActionList: i32 = 127;
pub(crate) const YYSYMBOL_MultiActionList: i32 = 126;
pub(crate) const YYSYMBOL_Term: i32 = 125;
pub(crate) const YYSYMBOL_Expr: i32 = 124;
pub(crate) const YYSYMBOL_ExprList: i32 = 123;
pub(crate) const YYSYMBOL_MergeMode: i32 = 122;
pub(crate) const YYSYMBOL_OptMergeMode: i32 = 121;
pub(crate) const YYSYMBOL_Element: i32 = 120;
pub(crate) const YYSYMBOL_FieldSpec: i32 = 119;
pub(crate) const YYSYMBOL_DoodadType: i32 = 118;
pub(crate) const YYSYMBOL_DoodadDecl: i32 = 117;
pub(crate) const YYSYMBOL_Coord: i32 = 116;
pub(crate) const YYSYMBOL_CoordList: i32 = 115;
pub(crate) const YYSYMBOL_OutlineInList: i32 = 114;
pub(crate) const YYSYMBOL_OutlineList: i32 = 113;
pub(crate) const YYSYMBOL_OverlayKey: i32 = 112;
pub(crate) const YYSYMBOL_OverlayKeyList: i32 = 111;
pub(crate) const YYSYMBOL_OverlayDecl: i32 = 110;
pub(crate) const YYSYMBOL_Key: i32 = 109;
pub(crate) const YYSYMBOL_Keys: i32 = 108;
pub(crate) const YYSYMBOL_RowBodyItem: i32 = 107;
pub(crate) const YYSYMBOL_RowBody: i32 = 106;
pub(crate) const YYSYMBOL_SectionBodyItem: i32 = 105;
pub(crate) const YYSYMBOL_SectionBody: i32 = 104;
pub(crate) const YYSYMBOL_SectionDecl: i32 = 103;
pub(crate) const YYSYMBOL_ShapeDecl: i32 = 102;
pub(crate) const YYSYMBOL_UnknownCompoundStatementDecl: i32 = 101;
pub(crate) const YYSYMBOL_UnknownDecl: i32 = 100;
pub(crate) const YYSYMBOL_LedNameDecl: i32 = 99;
pub(crate) const YYSYMBOL_LedMapDecl: i32 = 98;
pub(crate) const YYSYMBOL_KeyOrKeySym: i32 = 97;
pub(crate) const YYSYMBOL_KeyOrKeySymList: i32 = 96;
pub(crate) const YYSYMBOL_ModMapDecl: i32 = 95;
pub(crate) const YYSYMBOL_GroupCompatDecl: i32 = 94;
pub(crate) const YYSYMBOL_NoSymbolOrActionList: i32 = 93;
pub(crate) const YYSYMBOL_MultiKeySymOrActionList: i32 = 92;
pub(crate) const YYSYMBOL_SymbolsVarDecl: i32 = 91;
pub(crate) const YYSYMBOL_SymbolsBody: i32 = 90;
pub(crate) const YYSYMBOL_OptSymbolsBody: i32 = 89;
pub(crate) const YYSYMBOL_SymbolsDecl: i32 = 88;
pub(crate) const YYSYMBOL_KeyTypeDecl: i32 = 87;
pub(crate) const YYSYMBOL_VarDeclList: i32 = 86;
pub(crate) const YYSYMBOL_InterpretMatch: i32 = 85;
pub(crate) const YYSYMBOL_InterpretDecl: i32 = 84;
pub(crate) const YYSYMBOL_VModDef: i32 = 83;
pub(crate) const YYSYMBOL_VModDefList: i32 = 82;
pub(crate) const YYSYMBOL_VModDecl: i32 = 81;
pub(crate) const YYSYMBOL_KeyAliasDecl: i32 = 80;
pub(crate) const YYSYMBOL_KeyNameDecl: i32 = 79;
pub(crate) const YYSYMBOL_VarDecl: i32 = 78;
pub(crate) const YYSYMBOL_Decl: i32 = 77;
pub(crate) const YYSYMBOL_DeclList: i32 = 76;
pub(crate) const YYSYMBOL_Flag: i32 = 75;
pub(crate) const YYSYMBOL_Flags: i32 = 74;
pub(crate) const YYSYMBOL_OptFlags: i32 = 73;
pub(crate) const YYSYMBOL_FileType: i32 = 72;
pub(crate) const YYSYMBOL_XkbMapConfig: i32 = 71;
pub(crate) const YYSYMBOL_XkbMapConfigList: i32 = 70;
pub(crate) const YYSYMBOL_XkbCompositeType: i32 = 69;
pub(crate) const YYSYMBOL_XkbCompositeMap: i32 = 68;
pub(crate) const YYSYMBOL_XkbFile: i32 = 67;
pub(crate) const YYSYMBOL_YYACCEPT: i32 = 66;
pub(crate) const YYSYMBOL_ALTERNATE_GROUP: i32 = 65;
pub(crate) const YYSYMBOL_FUNCTION_KEYS: i32 = 64;
pub(crate) const YYSYMBOL_KEYPAD_KEYS: i32 = 63;
pub(crate) const YYSYMBOL_MODIFIER_KEYS: i32 = 62;
pub(crate) const YYSYMBOL_ALPHANUMERIC_KEYS: i32 = 61;
pub(crate) const YYSYMBOL_HIDDEN: i32 = 60;
pub(crate) const YYSYMBOL_DEFAULT: i32 = 59;
pub(crate) const YYSYMBOL_PARTIAL: i32 = 58;
pub(crate) const YYSYMBOL_KEYNAME: i32 = 57;
pub(crate) const YYSYMBOL_IDENT: i32 = 56;
pub(crate) const YYSYMBOL_FLOAT: i32 = 55;
pub(crate) const YYSYMBOL_INTEGER: i32 = 54;
pub(crate) const YYSYMBOL_DECIMAL_DIGIT: i32 = 53;
pub(crate) const YYSYMBOL_STRING: i32 = 52;
pub(crate) const YYSYMBOL_INVERT: i32 = 51;
pub(crate) const YYSYMBOL_EXCLAM: i32 = 50;
pub(crate) const YYSYMBOL_SEMI: i32 = 49;
pub(crate) const YYSYMBOL_COMMA: i32 = 48;
pub(crate) const YYSYMBOL_DOT: i32 = 47;
pub(crate) const YYSYMBOL_CBRACKET: i32 = 46;
pub(crate) const YYSYMBOL_OBRACKET: i32 = 45;
pub(crate) const YYSYMBOL_CPAREN: i32 = 44;
pub(crate) const YYSYMBOL_OPAREN: i32 = 43;
pub(crate) const YYSYMBOL_CBRACE: i32 = 42;
pub(crate) const YYSYMBOL_OBRACE: i32 = 41;
pub(crate) const YYSYMBOL_TIMES: i32 = 40;
pub(crate) const YYSYMBOL_DIVIDE: i32 = 39;
pub(crate) const YYSYMBOL_MINUS: i32 = 38;
pub(crate) const YYSYMBOL_PLUS: i32 = 37;
pub(crate) const YYSYMBOL_EQUALS: i32 = 36;
pub(crate) const YYSYMBOL_VIRTUAL: i32 = 35;
pub(crate) const YYSYMBOL_LOGO: i32 = 34;
pub(crate) const YYSYMBOL_SOLID: i32 = 33;
pub(crate) const YYSYMBOL_OUTLINE: i32 = 32;
pub(crate) const YYSYMBOL_TEXT: i32 = 31;
pub(crate) const YYSYMBOL_OVERLAY: i32 = 30;
pub(crate) const YYSYMBOL_SECTION: i32 = 29;
pub(crate) const YYSYMBOL_ROW: i32 = 28;
pub(crate) const YYSYMBOL_KEYS: i32 = 27;
pub(crate) const YYSYMBOL_SHAPE: i32 = 26;
pub(crate) const YYSYMBOL_INDICATOR: i32 = 25;
pub(crate) const YYSYMBOL_MODIFIER_MAP: i32 = 24;
pub(crate) const YYSYMBOL_GROUP: i32 = 23;
pub(crate) const YYSYMBOL_ALIAS: i32 = 22;
pub(crate) const YYSYMBOL_KEY: i32 = 21;
pub(crate) const YYSYMBOL_ACTION_TOK: i32 = 20;
pub(crate) const YYSYMBOL_INTERPRET: i32 = 19;
pub(crate) const YYSYMBOL_TYPE: i32 = 18;
pub(crate) const YYSYMBOL_VIRTUAL_MODS: i32 = 17;
pub(crate) const YYSYMBOL_ALTERNATE: i32 = 16;
pub(crate) const YYSYMBOL_REPLACE: i32 = 15;
pub(crate) const YYSYMBOL_AUGMENT: i32 = 14;
pub(crate) const YYSYMBOL_OVERRIDE: i32 = 13;
pub(crate) const YYSYMBOL_INCLUDE: i32 = 12;
pub(crate) const YYSYMBOL_XKB_LAYOUT: i32 = 11;
pub(crate) const YYSYMBOL_XKB_SEMANTICS: i32 = 10;
pub(crate) const YYSYMBOL_XKB_GEOMETRY: i32 = 9;
pub(crate) const YYSYMBOL_XKB_COMPATMAP: i32 = 8;
pub(crate) const YYSYMBOL_XKB_SYMBOLS: i32 = 7;
pub(crate) const YYSYMBOL_XKB_TYPES: i32 = 6;
pub(crate) const YYSYMBOL_XKB_KEYCODES: i32 = 5;
pub(crate) const YYSYMBOL_XKB_KEYMAP: i32 = 4;
pub(crate) const YYSYMBOL_ERROR_TOK: i32 = 3;
pub(crate) const YYSYMBOL_YYUNDEF: i32 = 2;
pub(crate) const YYSYMBOL_YYerror: i32 = 1;
pub(crate) const YYSYMBOL_YYEOF: i32 = 0;
pub(crate) const YYSYMBOL_YYEMPTY: i32 = -2;

pub(crate) const YYARGS_MAX: u32 = 5;
pub(crate) const YYFINAL: i32 = 16;
pub(crate) const YYLAST: i32 = 928;
pub(crate) const YYNTOKENS: i32 = 66;
pub(crate) const YYMAXUTOK: i32 = 257;
pub(crate) const YYINITDEPTH: usize = 200;
pub(crate) const YYMAXDEPTH: usize = 10000;
pub(crate) const YYPACT_NINF: i32 = -280;
pub(crate) const YYENOMEM: i32 = -2;

pub(crate) struct parser_param<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    pub(crate) scanner: &'a mut scanner<'a>,
    pub(crate) rtrn: Option<Box<XkbFile>>,
    pub(crate) more_maps: bool,
}

static YYTRANSLATE: [i8; 258] = [
    0, 4, 5, 6, 7, 8, 9, 10, 11, 2, 12, 13, 14, 15, 16, 2, 2, 2, 2, 2, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 2, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 51, 2, 2, 2, 2, 52, 53, 54, 55, 56, 57, 2, 2, 2, 2, 58, 59, 60, 61, 62, 63, 64,
    65, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1, 2,
];

fn yysymbol_name(yysymbol: i32) -> &'static str {
    static YY_SNAME: [&str; 150] = [
        "end of file",
        "error",
        "invalid token",
        "invalid token",
        "xkb_keymap",
        "xkb_keycodes",
        "xkb_types",
        "xkb_symbols",
        "xkb_compatibility",
        "xkb_geometry",
        "xkb_semantics",
        "xkb_layout",
        "include",
        "override",
        "augment",
        "replace",
        "alternate",
        "virtual_modifiers",
        "type",
        "interpret",
        "action",
        "key",
        "alias",
        "group",
        "modifier_map",
        "indicator",
        "shape",
        "keys",
        "row",
        "section",
        "overlay",
        "text",
        "outline",
        "solid",
        "logo",
        "virtual",
        "=",
        "+",
        "-",
        "/",
        "*",
        "{",
        "}",
        "(",
        ")",
        "[",
        "]",
        ".",
        ",",
        ";",
        "!",
        "~",
        "string literal",
        "decimal digit",
        "integer literal",
        "float literal",
        "identifier",
        "key name",
        "partial",
        "default",
        "hidden",
        "alphanumeric_keys",
        "modifier_keys",
        "keypad_keys",
        "function_keys",
        "alternate_group",
        "$accept",
        "XkbFile",
        "XkbCompositeMap",
        "XkbCompositeType",
        "XkbMapConfigList",
        "XkbMapConfig",
        "FileType",
        "OptFlags",
        "Flags",
        "Flag",
        "DeclList",
        "Decl",
        "VarDecl",
        "KeyNameDecl",
        "KeyAliasDecl",
        "VModDecl",
        "VModDefList",
        "VModDef",
        "InterpretDecl",
        "InterpretMatch",
        "VarDeclList",
        "KeyTypeDecl",
        "SymbolsDecl",
        "OptSymbolsBody",
        "SymbolsBody",
        "SymbolsVarDecl",
        "MultiKeySymOrActionList",
        "NoSymbolOrActionList",
        "GroupCompatDecl",
        "ModMapDecl",
        "KeyOrKeySymList",
        "KeyOrKeySym",
        "LedMapDecl",
        "LedNameDecl",
        "UnknownDecl",
        "UnknownCompoundStatementDecl",
        "ShapeDecl",
        "SectionDecl",
        "SectionBody",
        "SectionBodyItem",
        "RowBody",
        "RowBodyItem",
        "Keys",
        "Key",
        "OverlayDecl",
        "OverlayKeyList",
        "OverlayKey",
        "OutlineList",
        "OutlineInList",
        "CoordList",
        "Coord",
        "DoodadDecl",
        "DoodadType",
        "FieldSpec",
        "Element",
        "OptMergeMode",
        "MergeMode",
        "ExprList",
        "Expr",
        "Term",
        "MultiActionList",
        "ActionList",
        "NonEmptyActions",
        "Actions",
        "Action",
        "Lhs",
        "OptTerminal",
        "Terminal",
        "MultiKeySymList",
        "KeySymList",
        "NonEmptyKeySyms",
        "KeySyms",
        "KeySym",
        "KeySymLit",
        "SignedNumber",
        "Number",
        "Float",
        "Integer",
        "KeyCode",
        "Ident",
        "String",
        "OptMapName",
        "MapName",
        "",
    ];
    YY_SNAME[yysymbol as usize]
}

static YYPACT: [i16; 384] = [
    7, -280, -280, -280, -280, -280, -280, -280, -280, -280, 32, -280, -280, 578, 847, -280, -280,
    -280, -280, -280, -280, -280, -280, -280, -280, -12, -12, -280, -280, 22, -280, 36, -280, -280,
    463, 10, 53, -280, 458, -280, -280, -280, -280, -280, 57, -280, 25, 34, -280, -280, 64, 59,
    172, -280, 40, 61, 135, 64, 154, 59, -280, 59, 78, -280, -280, -280, 114, 64, 324, 120, -280,
    -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280, -280,
    59, -18, -280, 134, 143, -280, -280, -30, -280, 175, -280, 179, -280, -280, -280, -280, -280,
    182, 190, -280, 197, 213, -280, -280, 248, 222, 263, 234, 237, 261, 135, 258, -280, -280, 276,
    293, -280, -280, -280, 142, 289, 332, 869, 332, -280, 64, -280, 332, -280, -280, 332, 597, 269,
    332, 60, 332, -280, 35, 461, 296, -280, -280, 332, -280, -280, 287, -280, -280, -280, -280,
    -280, -280, -280, -280, -280, -280, 332, 332, 825, 332, 332, 332, -6, 228, -280, -280, -280,
    301, -280, -280, 294, 103, -280, 433, 639, 654, 433, 478, 64, 306, 311, -280, -280, 318, -27,
    313, 233, -280, 13, -280, -280, 285, 696, 319, 96, 37, -280, 45, -280, 330, 59, 326, 59, -280,
    -280, 419, -280, -280, -280, 332, 711, 372, -280, 753, -280, -280, -280, -280, 325, 48, -280,
    418, -280, -280, 332, 332, 332, 332, 332, -280, 332, 332, -280, 322, -280, 323, 331, 520, -280,
    337, 130, 133, -280, -280, 170, -280, -280, -280, 341, 597, 290, -280, -280, 343, 60, -280,
    344, 56, 189, -280, -280, -280, 346, -280, 355, -25, 358, 319, 377, 773, 375, 364, -280, 386,
    368, -280, 370, 332, -280, 869, -280, -38, 433, 253, 253, -280, -280, 433, 266, -280, -280,
    -280, -280, 67, -280, -280, 540, -280, 845, -280, 161, -280, -280, -280, 433, -280, -280, -280,
    -280, -280, 96, -280, -280, -280, -280, 796, 433, 381, -280, 227, -280, 384, -280, -280, -280,
    -280, 30, -280, -280, 332, -280, -280, 208, 582, 239, 242, -280, -280, 180, -280, -280, -280,
    400, 89, -24, 405, -280, 423, 112, -280, -280, 433, -280, -280, -280, -280, -280, -280, -280,
    -280, 332, -280, 113, -280, -280, 403, 425, 384, 117, 427, -24, -280, -280, -280, -280, -280,
    -280,
];
static YYDEFACT: [u8; 384] = [
    18, 4, 21, 22, 23, 24, 25, 26, 27, 28, 0, 2, 3, 0, 17, 20, 1, 6, 12, 13, 15, 14, 16, 7, 8, 218,
    218, 19, 219, 0, 217, 0, 10, 31, 18, 142, 0, 9, 0, 143, 145, 144, 146, 147, 0, 29, 0, 141, 5,
    11, 0, 132, 131, 130, 133, 0, 134, 135, 136, 137, 138, 139, 140, 125, 126, 127, 0, 0, 214, 0,
    215, 32, 34, 35, 30, 33, 36, 37, 39, 38, 40, 41, 45, 46, 42, 43, 44, 0, 176, 129, 0, 128, 47,
    214, 0, 55, 56, 216, 0, 201, 199, 202, 203, 200, 0, 60, 198, 0, 0, 211, 210, 0, 0, 0, 0, 0, 0,
    0, 0, 209, 185, 0, 180, 184, 183, 182, 0, 0, 0, 0, 0, 49, 0, 53, 0, 62, 62, 0, 66, 0, 0, 0, 0,
    62, 0, 0, 0, 50, 62, 0, 213, 212, 0, 62, 132, 131, 133, 134, 135, 136, 137, 139, 140, 0, 0, 0,
    0, 0, 0, 176, 0, 156, 173, 163, 161, 164, 128, 177, 0, 54, 57, 0, 0, 59, 81, 0, 0, 65, 68, 73,
    0, 128, 0, 0, 86, 0, 85, 87, 0, 0, 0, 0, 0, 116, 0, 121, 0, 136, 138, 0, 99, 101, 0, 97, 102,
    100, 0, 0, 0, 51, 0, 158, 161, 157, 174, 0, 0, 171, 0, 159, 160, 150, 0, 0, 0, 0, 178, 0, 0,
    48, 0, 61, 0, 201, 0, 195, 200, 0, 0, 169, 168, 0, 189, 188, 72, 0, 0, 0, 52, 82, 0, 0, 89, 0,
    0, 0, 207, 208, 206, 0, 205, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 91, 0, 150, 172, 0, 165, 0, 149,
    152, 153, 151, 154, 155, 0, 63, 58, 80, 193, 0, 192, 78, 0, 76, 0, 74, 0, 64, 67, 70, 69, 83,
    84, 88, 117, 204, 0, 93, 115, 94, 120, 0, 119, 0, 106, 0, 104, 0, 95, 90, 92, 123, 0, 170, 162,
    0, 179, 194, 0, 0, 0, 0, 167, 166, 0, 196, 187, 186, 0, 0, 0, 0, 103, 0, 0, 113, 175, 148, 191,
    190, 79, 77, 75, 197, 122, 118, 150, 109, 0, 108, 98, 0, 0, 0, 0, 0, 0, 114, 111, 112, 110,
    105, 107,
];
static YYPGOTO: [i16; 83] = [
    -280, -280, -280, -280, -280, 434, -280, 443, -280, 469, -280, -280, -45, -280, -280, -280,
    -280, 356, -280, -280, 51, -280, -280, -280, -280, 244, 251, -280, -280, -280, -280, 249, 466,
    -280, -280, -280, -280, -280, -280, 302, -280, 187, -280, 138, -280, -280, 144, -280, 257,
    -196, 259, 470, -280, -46, -280, -280, -280, -279, 63, 5, 232, -280, -176, 231, -181, -35,
    -280, 474, 247, -280, 240, -280, 500, -182, 236, 291, -280, -44, -280, -37, -23, 528, -280,
];
static YYDEFGOTO: [i16; 83] = [
    0, 10, 11, 25, 34, 12, 26, 13, 14, 15, 35, 45, 241, 72, 73, 74, 94, 95, 75, 104, 181, 76, 77,
    186, 187, 188, 189, 247, 78, 79, 195, 196, 211, 81, 82, 83, 84, 85, 212, 213, 326, 327, 369,
    370, 214, 355, 356, 202, 203, 204, 205, 215, 87, 169, 89, 46, 47, 288, 289, 171, 248, 226, 172,
    173, 227, 174, 121, 175, 251, 300, 252, 347, 197, 106, 269, 270, 123, 124, 152, 176, 125, 29,
    30,
];
static YYTABLE: [i16; 929] = [
    88, 71, 253, 250, 264, 333, 335, 1, 249, 91, 336, 90, 111, 96, 113, -71, 200, 367, 132, 133,
    112, -71, 39, 40, 41, 42, 43, 128, 98, 129, 118, 93, 16, 368, 70, 114, 115, 231, 116, 128, 28,
    129, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 44, 60, 61, 260, 62, 63, 64, 65, 66, 261, 301, 32,
    127, 2, 3, 4, 5, 6, 7, 8, 9, 146, 357, 67, 200, 33, 336, 271, 201, 68, 69, 177, 70, 272, 92,
    273, 375, 99, 285, 93, 88, 274, 70, 96, 286, 107, 315, 88, 210, 191, 48, 190, 274, 334, 49,
    206, 91, 338, 90, 97, 100, 101, 102, 339, 103, 194, 108, 225, 93, 253, 250, 70, 344, 348, 350,
    249, 222, 222, -124, 366, 222, 222, 265, 88, 88, 274, 225, 117, 232, 233, 234, 235, 91, 91, 90,
    90, 254, 266, 267, 268, 239, 88, 373, 376, 126, 360, 301, 381, 374, 377, 91, 301, 90, 336, 88,
    210, 221, 223, 130, 88, 229, 230, 88, 91, 302, 90, 303, 304, 91, 305, 90, 91, 114, 90, 277,
    182, 109, 110, 99, 170, 131, 178, 199, 150, 151, 180, 225, 217, 183, 99, 345, 193, 220, 198,
    97, 109, 110, 99, 88, 134, 218, 245, 101, 102, 306, 103, 307, 191, 135, 190, 364, 136, 100,
    101, 102, 137, 103, 228, 88, 325, 299, 101, 102, 206, 103, 99, 138, 91, 225, 90, 266, 267, 268,
    154, 155, 53, 156, 139, 157, 158, 159, 160, 324, 60, 161, 225, 162, 225, 359, 101, 102, 141,
    103, 232, 233, 234, 235, 352, 232, 233, 234, 235, 236, 143, 225, 67, 144, 280, 88, 325, 259,
    93, 140, 362, 70, 305, 363, 91, 307, 90, 234, 235, 225, 290, 291, 292, 293, 142, 294, 295, 145,
    232, 233, 234, 235, 147, 154, 155, 53, 156, 337, 157, 158, 159, 160, 148, 60, 161, 311, 162,
    232, 233, 234, 235, 192, 163, 164, 149, 153, 165, 216, 166, 262, 184, 219, 237, 323, 238, 167,
    168, 97, 109, 110, 119, 93, 120, 255, 70, 154, 155, 53, 156, 257, 157, 158, 159, 160, 256, 60,
    161, 258, 162, 201, -181, 275, 276, 284, 163, 164, 296, 297, 165, -139, 166, 97, 109, 110, 119,
    -214, 120, 167, 168, 97, 109, 110, 119, 93, 120, 308, 70, 312, 314, 317, 154, 155, 53, 156,
    358, 157, 158, 159, 160, 318, 60, 161, 320, 162, 232, 233, 234, 235, 329, 163, 164, 328, 331,
    322, 332, 166, 282, 351, 232, 233, 234, 235, 167, 168, 97, 109, 110, 119, 93, 120, 330, 70,
    154, 155, 53, 156, 354, 157, 158, 207, 160, 365, 208, 161, 209, 62, 63, 64, 65, 371, 232, 233,
    234, 235, 372, 378, 278, 287, 18, 19, 20, 21, 22, 37, 67, 232, 233, 234, 235, 379, 93, 382, 38,
    70, 154, 155, 53, 156, 27, 157, 158, 207, 160, 179, 208, 161, 209, 62, 63, 64, 65, 154, 155,
    53, 156, 309, 157, 158, 159, 160, 36, 60, 243, 310, 162, 313, 67, 80, 353, 279, 383, 86, 93,
    380, 244, 70, 2, 3, 4, 5, 6, 7, 8, 9, 319, 245, 101, 102, 321, 246, 341, 343, 70, 154, 155, 53,
    156, 122, 157, 158, 159, 160, 346, 60, 243, 342, 162, 105, 349, 31, 0, 316, 0, 154, 155, 53,
    156, 298, 157, 158, 159, 160, 0, 60, 243, 0, 162, 299, 101, 102, 0, 246, 0, 0, 70, 0, 340, 17,
    18, 19, 20, 21, 22, 23, 24, 0, 0, 245, 101, 102, 0, 246, 0, 0, 70, 154, 155, 53, 156, 0, 157,
    158, 159, 160, 0, 60, 243, 0, 162, 0, 154, 155, 53, 156, 0, 157, 158, 159, 160, 361, 60, 161,
    0, 162, 0, 0, 0, 0, 0, 299, 101, 102, 0, 246, 0, 0, 70, 184, 0, 0, 0, 0, 185, 0, 0, 0, 0, 0,
    93, 0, 0, 70, 154, 155, 53, 156, 0, 157, 158, 159, 160, 0, 60, 161, 0, 162, 0, 154, 155, 53,
    156, 0, 157, 158, 159, 160, 240, 60, 161, 0, 162, 0, 0, 0, 67, 0, 0, 0, 0, 0, 93, 242, 0, 70,
    0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 93, 0, 0, 70, 154, 155, 53, 156, 0, 157, 158, 159, 160, 0,
    60, 161, 0, 162, 0, 154, 155, 53, 156, 0, 157, 158, 159, 160, 263, 60, 161, 0, 162, 0, 0, 0,
    67, 0, 0, 0, 0, 0, 93, 281, 0, 70, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 93, 0, 0, 70, 154, 155,
    53, 156, 0, 157, 158, 159, 160, 0, 60, 161, 0, 162, 0, 0, 0, 0, 0, 0, 154, 155, 53, 156, 283,
    157, 158, 159, 160, 324, 60, 161, 67, 162, 0, 0, 0, 0, 93, 0, 0, 70, 0, 154, 155, 53, 156, 0,
    157, 158, 159, 160, 67, 60, 161, 0, 162, 0, 93, 0, 0, 70, 0, 0, 0, 0, 0, 224, 0, 0, 201, 0,
    154, 155, 53, 156, 0, 157, 158, 159, 160, 93, 60, 161, 70, 162, 0, 0, 0, 0, 0, 0, 154, 155, 53,
    156, 224, 157, 158, 159, 160, 0, 60, 161, 0, 162, 0, 0, 0, 0, 93, 0, 0, 70, 0, 165, 154, 155,
    53, 156, 0, 157, 158, 159, 160, 0, 60, 161, 0, 162, 93, 0, 0, 70, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0, 70,
];
static YYCHECK: [i16; 929] = [
    46, 46, 184, 184, 200, 284, 44, 0, 184, 46, 48, 46, 56, 50, 58, 42, 41, 41, 48, 49, 57, 48, 12,
    13, 14, 15, 16, 45, 51, 47, 67, 56, 0, 57, 59, 58, 59, 43, 61, 45, 52, 47, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 26, 42, 28, 29, 42, 31, 32, 33, 34, 35, 48, 244, 41, 87, 58, 59, 60, 61, 62,
    63, 64, 65, 117, 44, 50, 41, 41, 48, 42, 45, 56, 57, 129, 59, 48, 52, 42, 367, 29, 42, 56, 138,
    48, 59, 132, 48, 57, 42, 145, 145, 138, 49, 138, 48, 286, 49, 144, 145, 42, 145, 52, 52, 53,
    54, 48, 56, 57, 57, 165, 56, 303, 303, 59, 305, 307, 322, 303, 163, 164, 52, 42, 167, 168, 38,
    181, 182, 48, 184, 25, 37, 38, 39, 40, 181, 182, 181, 182, 185, 53, 54, 55, 49, 199, 42, 42,
    36, 339, 340, 42, 48, 48, 199, 345, 199, 48, 212, 212, 163, 164, 36, 217, 167, 168, 220, 212,
    46, 212, 48, 46, 217, 48, 217, 220, 207, 220, 209, 136, 53, 54, 29, 128, 49, 130, 143, 53, 54,
    134, 244, 148, 137, 29, 41, 140, 153, 142, 52, 53, 54, 29, 256, 36, 149, 52, 53, 54, 46, 56,
    48, 256, 41, 256, 42, 41, 52, 53, 54, 37, 56, 166, 276, 276, 52, 53, 54, 272, 56, 29, 41, 276,
    286, 276, 53, 54, 55, 18, 19, 20, 21, 36, 23, 24, 25, 26, 27, 28, 29, 303, 31, 305, 52, 53, 54,
    41, 56, 37, 38, 39, 40, 42, 37, 38, 39, 40, 46, 41, 322, 50, 41, 216, 326, 326, 49, 56, 36, 46,
    59, 48, 46, 326, 48, 326, 39, 40, 340, 232, 233, 234, 235, 36, 237, 238, 41, 37, 38, 39, 40,
    49, 18, 19, 20, 21, 46, 23, 24, 25, 26, 41, 28, 29, 257, 31, 37, 38, 39, 40, 57, 37, 38, 36,
    41, 41, 36, 43, 49, 45, 49, 36, 275, 45, 50, 51, 52, 53, 54, 55, 56, 57, 42, 59, 18, 19, 20,
    21, 36, 23, 24, 25, 26, 48, 28, 29, 49, 31, 45, 41, 36, 41, 43, 37, 38, 49, 49, 41, 43, 43, 52,
    53, 54, 55, 43, 57, 50, 51, 52, 53, 54, 55, 56, 57, 49, 59, 49, 49, 48, 18, 19, 20, 21, 336,
    23, 24, 25, 26, 49, 28, 29, 49, 31, 37, 38, 39, 40, 49, 37, 38, 41, 49, 41, 49, 43, 49, 41, 37,
    38, 39, 40, 50, 51, 52, 53, 54, 55, 56, 57, 49, 59, 18, 19, 20, 21, 57, 23, 24, 25, 26, 46, 28,
    29, 30, 31, 32, 33, 34, 49, 37, 38, 39, 40, 36, 57, 42, 44, 5, 6, 7, 8, 9, 34, 50, 37, 38, 39,
    40, 49, 56, 49, 34, 59, 18, 19, 20, 21, 14, 23, 24, 25, 26, 132, 28, 29, 30, 31, 32, 33, 34,
    18, 19, 20, 21, 256, 23, 24, 25, 26, 42, 28, 29, 257, 31, 261, 50, 46, 326, 212, 377, 46, 56,
    374, 41, 59, 58, 59, 60, 61, 62, 63, 64, 65, 272, 52, 53, 54, 274, 56, 303, 305, 59, 18, 19,
    20, 21, 68, 23, 24, 25, 26, 307, 28, 29, 303, 31, 52, 317, 26, -1, 265, -1, 18, 19, 20, 21, 42,
    23, 24, 25, 26, -1, 28, 29, -1, 31, 52, 53, 54, -1, 56, -1, -1, 59, -1, 41, 4, 5, 6, 7, 8, 9,
    10, 11, -1, -1, 52, 53, 54, -1, 56, -1, -1, 59, 18, 19, 20, 21, -1, 23, 24, 25, 26, -1, 28, 29,
    -1, 31, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26, 42, 28, 29, -1, 31, -1, -1, -1, -1, -1, 52, 53,
    54, -1, 56, -1, -1, 59, 45, -1, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, -1, -1, 59, 18, 19, 20,
    21, -1, 23, 24, 25, 26, -1, 28, 29, -1, 31, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26, 42, 28, 29,
    -1, 31, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, 42, -1, 59, -1, -1, -1, -1, -1, 50, -1, -1, -1,
    -1, -1, 56, -1, -1, 59, 18, 19, 20, 21, -1, 23, 24, 25, 26, -1, 28, 29, -1, 31, -1, 18, 19, 20,
    21, -1, 23, 24, 25, 26, 42, 28, 29, -1, 31, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, 42, -1, 59,
    -1, -1, -1, -1, -1, 50, -1, -1, -1, -1, -1, 56, -1, -1, 59, 18, 19, 20, 21, -1, 23, 24, 25, 26,
    -1, 28, 29, -1, 31, -1, -1, -1, -1, -1, -1, 18, 19, 20, 21, 42, 23, 24, 25, 26, 27, 28, 29, 50,
    31, -1, -1, -1, -1, 56, -1, -1, 59, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26, 50, 28, 29, -1, 31,
    -1, 56, -1, -1, 59, -1, -1, -1, -1, -1, 42, -1, -1, 45, -1, 18, 19, 20, 21, -1, 23, 24, 25, 26,
    56, 28, 29, 59, 31, -1, -1, -1, -1, -1, -1, 18, 19, 20, 21, 42, 23, 24, 25, 26, -1, 28, 29, -1,
    31, -1, -1, -1, -1, 56, -1, -1, 59, -1, 41, 18, 19, 20, 21, -1, 23, 24, 25, 26, -1, 28, 29, -1,
    31, 56, -1, -1, 59, 58, 59, 60, 61, 62, 63, 64, 65, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 56, -1, -1, 59,
];
static YYR1: [yytype_uint8; 220] = [
    0, 66, 67, 67, 67, 68, 69, 69, 69, 70, 70, 71, 72, 72, 72, 72, 72, 73, 73, 74, 74, 75, 75, 75,
    75, 75, 75, 75, 75, 76, 76, 76, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77, 77,
    78, 78, 78, 79, 80, 81, 82, 82, 83, 83, 84, 85, 85, 86, 86, 87, 88, 89, 89, 90, 90, 91, 91, 91,
    91, 91, 92, 92, 92, 92, 92, 93, 93, 93, 94, 95, 96, 96, 97, 97, 98, 99, 99, 100, 101, 102, 102,
    103, 104, 104, 105, 105, 105, 105, 105, 106, 106, 107, 107, 108, 108, 109, 109, 110, 111, 111,
    112, 113, 113, 114, 114, 114, 115, 115, 116, 117, 118, 118, 118, 118, 119, 119, 120, 120, 120,
    120, 120, 120, 120, 120, 120, 120, 120, 121, 121, 122, 122, 122, 122, 122, 123, 123, 123, 124,
    124, 124, 124, 124, 124, 125, 125, 125, 125, 125, 125, 125, 125, 125, 126, 126, 126, 126, 127,
    127, 128, 129, 129, 130, 131, 131, 131, 131, 132, 132, 133, 133, 133, 133, 134, 134, 134, 134,
    135, 135, 135, 135, 136, 136, 137, 137, 138, 138, 139, 139, 139, 139, 140, 140, 141, 141, 141,
    142, 143, 143, 144, 144, 145, 145, 146, 147, 147, 148,
];
static YYR2: [yytype_int8; 220] = [
    0, 2, 1, 1, 1, 7, 1, 1, 1, 2, 0, 7, 1, 1, 1, 1, 1, 1, 0, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 0,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 2, 3, 4, 5, 3, 3, 1, 1, 3, 6, 3, 1, 2, 0, 6,
    6, 1, 0, 3, 1, 3, 3, 1, 2, 1, 3, 5, 3, 5, 3, 4, 2, 0, 5, 6, 3, 1, 1, 1, 6, 5, 6, 5, 6, 6, 6, 6,
    2, 1, 5, 1, 1, 1, 1, 2, 1, 5, 1, 3, 1, 1, 3, 6, 3, 1, 3, 3, 1, 3, 5, 3, 3, 1, 5, 6, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 3, 1, 0, 3, 3, 3, 3, 3, 1, 2, 2, 2,
    2, 1, 4, 1, 1, 3, 3, 3, 1, 1, 3, 1, 3, 1, 2, 4, 1, 3, 4, 6, 1, 0, 1, 1, 1, 1, 3, 3, 1, 1, 3, 3,
    1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
];

pub(crate) type yytype_uint8 = u8;
pub(crate) type yytype_int8 = i8;

// ── Helper functions ────────────────────────────────────────────────

fn _xkbcommon_error(param: &mut parser_param, msg: &str) {
    let loc: scanner_loc = param.scanner.token_location();
    log::error!(
        "[XKB-{:03}] {}:{}:{}: {}\n",
        XKB_ERROR_INVALID_XKB_SYNTAX as i32,
        &param.scanner.file_name,
        loc.line,
        loc.column,
        msg
    );
}

fn resolve_keysym(param: &mut parser_param, name: sval) -> Option<u32> {
    let name_bytes = name.as_bytes();
    let name_str = name.as_str();

    if name_str.eq_ignore_ascii_case("any") || name_str.eq_ignore_ascii_case("nosymbol") {
        return Some(XKB_KEY_NO_SYMBOL as u32);
    }
    if name_str.eq_ignore_ascii_case("none") || name_str.eq_ignore_ascii_case("voidsymbol") {
        return Some(XKB_KEY_VoidSymbol as u32);
    }

    if name.len() >= 30 {
        return None;
    }

    // Build null-terminated buffer for xkb_keysym_from_name
    let mut buf = [0u8; 32];
    buf[..name.len()].copy_from_slice(name_bytes);
    buf[name.len()] = 0;
    let buf_slice = &buf[..name.len() + 1];

    let sym: u32 = xkb_keysym_from_name(buf_slice, XKB_KEYSYM_NO_FLAGS);
    if sym != XKB_KEY_NO_SYMBOL as u32 {
        if param.ctx.log_verbosity >= 2 {
            if let Some(ref_name) = xkb_keysym_is_deprecated(sym, buf_slice) {
                let loc: scanner_loc = param.scanner.token_location();
                log::warn!(
                    "[XKB-{:03}] {}:{}:{}: deprecated keysym name \"{}\"; please use \"{}\" instead.\n",
                    XKB_WARNING_DEPRECATED_KEYSYM_NAME as i32,
                    &param.scanner.file_name,
                    loc.line,
                    loc.column,
                    name_str,
                    ref_name
                );
            }
        }
        return Some(sym);
    }
    None
}

fn yypcontext_expected_tokens(yyssp: &[i16], ssp: usize, yyarg: &mut [i32], yyargn: usize) -> i32 {
    let mut yycount: i32 = 0;
    let yyn: i32 = YYPACT[yyssp[ssp] as usize] as i32;
    if yyn != YYPACT_NINF {
        let yyxbegin: i32 = if yyn < 0 { -yyn } else { 0 };
        let yychecklim: i32 = YYLAST - yyn + 1;
        let yyxend: i32 = if yychecklim < YYNTOKENS {
            yychecklim
        } else {
            YYNTOKENS
        };
        let mut yyx = yyxbegin;
        while yyx < yyxend {
            if YYCHECK[(yyx + yyn) as usize] as i32 == yyx && yyx != YYSYMBOL_YYerror {
                if yyargn == 0 {
                    yycount += 1;
                } else if (yycount as usize) == yyargn {
                    return 0;
                } else {
                    yyarg[yycount as usize] = yyx;
                    yycount += 1;
                }
            }
            yyx += 1;
        }
    }
    if yyargn > 0 && yycount == 0 {
        yyarg[0] = YYSYMBOL_YYEMPTY;
    }
    yycount
}

fn yysyntax_error(yyssp: &[i16], ssp: usize, yytoken: i32) -> String {
    let mut yyarg: [i32; 5] = [YYSYMBOL_YYEOF; 5];
    // Count expected tokens
    let mut yycount: i32 = 0;
    if yytoken != YYSYMBOL_YYEMPTY {
        yyarg[0] = yytoken;
        yycount = 1;
        let n = yypcontext_expected_tokens(yyssp, ssp, &mut yyarg[1..], 4);
        if n == YYENOMEM {
            return String::from("syntax error");
        }
        yycount += n;
    }

    let mut msg = String::from("syntax error, unexpected ");
    if yycount > 0 {
        msg.push_str(yysymbol_name(yyarg[0]));
    }
    if yycount > 1 {
        msg.push_str(", expecting ");
        for i in 1..yycount {
            if i > 1 {
                msg.push_str(" or ");
            }
            msg.push_str(yysymbol_name(yyarg[i as usize]));
        }
    }
    msg
}

fn ensure_capacity<'a>(v: &mut Vec<YYValue<'a>>, idx: usize) {
    if idx >= v.len() {
        v.resize_with(idx + 1, || YYValue::None);
    }
}

fn ensure_capacity_i16(v: &mut Vec<i16>, idx: usize) {
    if idx >= v.len() {
        v.resize(idx + 1, 0);
    }
}

// ── Main parser function ────────────────────────────────────────────

pub(crate) fn _xkbcommon_parse<'a>(param: &mut parser_param<'a>) -> i32 {
    let mut yychar: i32;
    let mut yylval: YYValue<'a> = YYValue::None;
    let mut _xkbcommon_nerrs: i32 = 0;
    let mut yystate: i32 = 0;
    let mut yyerrstatus: i32 = 0;

    let mut yyss: Vec<i16> = vec![0; YYINITDEPTH];
    let mut yyvs: Vec<YYValue<'a>> = Vec::with_capacity(YYINITDEPTH);
    for _ in 0..YYINITDEPTH {
        yyvs.push(YYValue::None);
    }

    let mut ssp: usize = 0; // state stack pointer
    let mut sp: usize = 0; // value stack pointer

    let mut yyn: i32;
    let yyresult: i32;
    let mut yytoken: i32;
    let mut yyval: YYValue<'a>;
    let mut yylen: i32;

    yychar = YYEMPTY;

    'main_loop: loop {
        // Ensure stack capacity
        ensure_capacity_i16(&mut yyss, ssp);
        ensure_capacity(&mut yyvs, sp);

        yyss[ssp] = yystate as i16;

        // Check stack overflow
        if ssp >= YYMAXDEPTH - 1 {
            _xkbcommon_error(param, "memory exhausted");
            yyresult = 2;
            break 'main_loop;
        }

        // Accept?
        if yystate == YYFINAL {
            yyresult = 0;
            break 'main_loop;
        }

        // Try to take a shift
        yyn = YYPACT[yystate as usize] as i32;
        if yyn == YYPACT_NINF {
            // goto yydefault
        } else {
            if yychar == YYEMPTY {
                yychar = _xkbcommon_lex(&mut yylval, param.scanner, param.ctx);
            }
            if yychar <= END_OF_FILE {
                yychar = END_OF_FILE;
                yytoken = YYSYMBOL_YYEOF;
            } else if yychar == YYerror {
                yychar = YYUNDEF;
                // goto yyerrlab1
                yyerrstatus = 3;
                // Error recovery: pop until we find a state that accepts error token
                loop {
                    yyn = YYPACT[yystate as usize] as i32;
                    if yyn != YYPACT_NINF {
                        yyn += YYSYMBOL_YYerror;
                        if (0..=YYLAST).contains(&yyn)
                            && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYerror
                        {
                            yyn = YYTABLE[yyn as usize] as i32;
                            if yyn > 0 {
                                break;
                            }
                        }
                    }
                    if ssp == 0 {
                        yyresult = 1;
                        break 'main_loop;
                    }
                    // Drop the value being popped
                    yyvs[sp] = YYValue::None;
                    sp = sp.saturating_sub(1);
                    ssp -= 1;
                    yystate = yyss[ssp] as i32;
                }
                sp += 1;
                ensure_capacity(&mut yyvs, sp);
                yyvs[sp] = std::mem::replace(&mut yylval, YYValue::None);
                yystate = yyn;
                ssp += 1;
                continue 'main_loop;
            } else {
                yytoken = if (0..=YYMAXUTOK).contains(&yychar) {
                    YYTRANSLATE[yychar as usize] as i32
                } else {
                    YYSYMBOL_YYUNDEF
                };
            }

            yyn += yytoken;
            if !(0..=YYLAST).contains(&yyn) || YYCHECK[yyn as usize] as i32 != yytoken {
                // goto yydefault (fall through)
            } else {
                yyn = YYTABLE[yyn as usize] as i32;
                if yyn <= 0 {
                    yyn = -yyn;
                    // goto yyreduce
                    yylen = YYR2[yyn as usize] as i32;
                    yyval = YYValue::None;

                    // Execute reduction
                    let reduce_ok = execute_reduction(yyn, &mut yyvs, sp, &mut yyval, param);
                    if !reduce_ok {
                        // YYERROR or YYABORT from reduction
                        _xkbcommon_nerrs += 1;
                        sp -= yylen as usize;
                        ssp -= yylen as usize;
                        yystate = yyss[ssp] as i32;

                        // Error recovery
                        yyerrstatus = 3;
                        loop {
                            yyn = YYPACT[yystate as usize] as i32;
                            if yyn != YYPACT_NINF {
                                yyn += YYSYMBOL_YYerror;
                                if (0..=YYLAST).contains(&yyn)
                                    && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYerror
                                {
                                    yyn = YYTABLE[yyn as usize] as i32;
                                    if yyn > 0 {
                                        break;
                                    }
                                }
                            }
                            if ssp == 0 {
                                yyresult = 1;
                                break 'main_loop;
                            }
                            yyvs[sp] = YYValue::None;
                            sp = sp.saturating_sub(1);
                            ssp -= 1;
                            yystate = yyss[ssp] as i32;
                        }
                        sp += 1;
                        ensure_capacity(&mut yyvs, sp);
                        yyvs[sp] = std::mem::replace(&mut yylval, YYValue::None);
                        yystate = yyn;
                        ssp += 1;
                        continue 'main_loop;
                    }

                    // Check for rule 3 (YYACCEPT mid-rule)
                    if yyn == 3 {
                        yyresult = 0;
                        break 'main_loop;
                    }

                    sp -= yylen as usize;
                    ssp -= yylen as usize;

                    sp += 1;
                    ensure_capacity(&mut yyvs, sp);
                    yyvs[sp] = yyval;

                    let yylhs: i32 = YYR1[yyn as usize] as i32 - YYNTOKENS;
                    let yyi: i32 = YYPGOTO[yylhs as usize] as i32 + yyss[ssp] as i32;
                    yystate = if (0..=YYLAST).contains(&yyi)
                        && YYCHECK[yyi as usize] as i32 == yyss[ssp] as i32
                    {
                        YYTABLE[yyi as usize] as i32
                    } else {
                        YYDEFGOTO[yylhs as usize] as i32
                    };

                    ssp += 1;
                    continue 'main_loop;
                } else {
                    // Shift
                    if yyerrstatus != 0 {
                        yyerrstatus -= 1;
                    }
                    yystate = yyn;
                    sp += 1;
                    ensure_capacity(&mut yyvs, sp);
                    yyvs[sp] = std::mem::replace(&mut yylval, YYValue::None);
                    yychar = YYEMPTY;
                    ssp += 1;
                    continue 'main_loop;
                }
            }
        }

        // yydefault: use default action
        yyn = YYDEFACT[yystate as usize] as i32;
        if yyn == 0 {
            // Syntax error
            yytoken = if yychar == YYEMPTY {
                YYSYMBOL_YYEMPTY
            } else if (0..=YYMAXUTOK).contains(&yychar) {
                YYTRANSLATE[yychar as usize] as i32
            } else {
                YYSYMBOL_YYUNDEF
            };

            if yyerrstatus == 0 {
                _xkbcommon_nerrs += 1;
                let msg = yysyntax_error(&yyss, ssp, yytoken);
                _xkbcommon_error(param, &msg);
            }

            if yyerrstatus == 3 {
                if yychar <= END_OF_FILE {
                    if yychar == END_OF_FILE {
                        yyresult = 1;
                        break 'main_loop;
                    }
                } else {
                    // Discard lookahead
                    yylval = YYValue::None;
                    yychar = YYEMPTY;
                }
            }

            // yyerrlab1: error recovery
            yyerrstatus = 3;
            loop {
                yyn = YYPACT[yystate as usize] as i32;
                if yyn != YYPACT_NINF {
                    yyn += YYSYMBOL_YYerror;
                    if (0..=YYLAST).contains(&yyn)
                        && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYerror
                    {
                        yyn = YYTABLE[yyn as usize] as i32;
                        if yyn > 0 {
                            break;
                        }
                    }
                }
                if ssp == 0 {
                    yyresult = 1;
                    break 'main_loop;
                }
                yyvs[sp] = YYValue::None;
                sp = sp.saturating_sub(1);
                ssp -= 1;
                yystate = yyss[ssp] as i32;
            }
            sp += 1;
            ensure_capacity(&mut yyvs, sp);
            yyvs[sp] = std::mem::replace(&mut yylval, YYValue::None);
            yystate = yyn;
            ssp += 1;
            continue 'main_loop;
        }

        // yyreduce with default action
        yylen = YYR2[yyn as usize] as i32;
        yyval = YYValue::None;

        let reduce_ok = execute_reduction(yyn, &mut yyvs, sp, &mut yyval, param);
        if !reduce_ok {
            _xkbcommon_nerrs += 1;
            sp -= yylen as usize;
            ssp -= yylen as usize;
            yystate = yyss[ssp] as i32;

            yyerrstatus = 3;
            loop {
                yyn = YYPACT[yystate as usize] as i32;
                if yyn != YYPACT_NINF {
                    yyn += YYSYMBOL_YYerror;
                    if (0..=YYLAST).contains(&yyn)
                        && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYerror
                    {
                        yyn = YYTABLE[yyn as usize] as i32;
                        if yyn > 0 {
                            break;
                        }
                    }
                }
                if ssp == 0 {
                    yyresult = 1;
                    break 'main_loop;
                }
                yyvs[sp] = YYValue::None;
                sp = sp.saturating_sub(1);
                ssp -= 1;
                yystate = yyss[ssp] as i32;
            }
            sp += 1;
            ensure_capacity(&mut yyvs, sp);
            yyvs[sp] = std::mem::replace(&mut yylval, YYValue::None);
            yystate = yyn;
            ssp += 1;
            continue 'main_loop;
        }

        // Rule 3 is YYACCEPT
        if yyn == 3 {
            yyresult = 0;
            break 'main_loop;
        }

        sp -= yylen as usize;
        ssp -= yylen as usize;

        sp += 1;
        ensure_capacity(&mut yyvs, sp);
        yyvs[sp] = yyval;

        let yylhs: i32 = YYR1[yyn as usize] as i32 - YYNTOKENS;
        let yyi: i32 = YYPGOTO[yylhs as usize] as i32 + yyss[ssp] as i32;
        yystate = if (0..=YYLAST).contains(&yyi) && YYCHECK[yyi as usize] as i32 == yyss[ssp] as i32
        {
            YYTABLE[yyi as usize] as i32
        } else {
            YYDEFGOTO[yylhs as usize] as i32
        };

        ssp += 1;
    }

    yyresult
}

/// Execute a reduction rule. Returns true on success, false on error (YYERROR).
fn execute_reduction<'a>(
    yyn: i32,
    yyvs: &mut [YYValue<'a>],
    sp: usize,
    yyval: &mut YYValue<'a>,
    param: &mut parser_param<'a>,
) -> bool {
    match yyn {
        2 => {
            // XkbFile: XkbCompositeMap
            param.rtrn = yyvs[sp].take_file();
            param.more_maps = param.rtrn.is_some();
            // yyval is dead here since we continue the loop; leave as None
        }
        3 => {
            // XkbFile: XkbMapConfig (YYACCEPT after)
            param.rtrn = yyvs[sp].take_file();
            param.more_maps = param.rtrn.is_some();
            // Note: caller checks yyn == 3 for YYACCEPT
        }
        4 => {
            // XkbFile: END_OF_FILE
            param.rtrn = None;
            *yyval = YYValue::None;
            param.more_maps = false;
        }
        5 => {
            // XkbCompositeMap: OptFlags XkbCompositeType OptMapName OBRACE XkbMapConfigList CBRACE SEMI
            let file_type = yyvs[sp - 5].as_file_type();
            let name = yyvs[sp - 4].take_str();
            let files = yyvs[sp - 2].take_file_list();
            let flags = yyvs[sp - 6].as_map_flags();
            let defs: Vec<Statement> = files.into_iter().map(Statement::XkbFile).collect();
            *yyval = YYValue::File(XkbFileCreate(
                file_type,
                if name.is_empty() { None } else { Some(name) },
                defs,
                flags,
            ));
        }
        6 => {
            *yyval = YYValue::FileType(FILE_TYPE_KEYMAP);
        }
        7 => {
            *yyval = YYValue::FileType(FILE_TYPE_KEYMAP);
        }
        8 => {
            *yyval = YYValue::FileType(FILE_TYPE_KEYMAP);
        }
        9 => {
            // XkbMapConfigList: XkbMapConfigList XkbMapConfig
            let file = yyvs[sp].take_file();
            let mut list = yyvs[sp - 1].take_file_list();
            if let Some(f) = file {
                list.push(f);
            }
            *yyval = YYValue::FileList(list);
        }
        10 => {
            // XkbMapConfigList: empty
            *yyval = YYValue::FileList(Vec::new());
        }
        11 => {
            // XkbMapConfig: OptFlags FileType OptMapName OBRACE DeclList CBRACE SEMI
            let file_type = yyvs[sp - 5].as_file_type();
            let name = yyvs[sp - 4].take_str();
            let stmts = yyvs[sp - 2].take_stmt_list();
            let flags = yyvs[sp - 6].as_map_flags();
            *yyval = YYValue::File(XkbFileCreate(
                file_type,
                if name.is_empty() { None } else { Some(name) },
                stmts,
                flags,
            ));
        }
        12 => {
            *yyval = YYValue::FileType(FILE_TYPE_KEYCODES);
        }
        13 => {
            *yyval = YYValue::FileType(FILE_TYPE_TYPES);
        }
        14 => {
            *yyval = YYValue::FileType(FILE_TYPE_COMPAT);
        }
        15 => {
            *yyval = YYValue::FileType(FILE_TYPE_SYMBOLS);
        }
        16 => {
            *yyval = YYValue::FileType(FILE_TYPE_GEOMETRY);
        }
        17 => {
            *yyval = YYValue::MapFlags(yyvs[sp].as_map_flags());
        }
        18 => {
            *yyval = YYValue::MapFlags(0);
        }
        19 => {
            let f = yyvs[sp - 1].as_map_flags() | yyvs[sp].as_map_flags();
            *yyval = YYValue::MapFlags(f);
        }
        20 => {
            *yyval = YYValue::MapFlags(yyvs[sp].as_map_flags());
        }
        21 => {
            *yyval = YYValue::MapFlags(MAP_IS_PARTIAL);
        }
        22 => {
            *yyval = YYValue::MapFlags(MAP_IS_DEFAULT);
        }
        23 => {
            *yyval = YYValue::MapFlags(MAP_IS_HIDDEN);
        }
        24 => {
            *yyval = YYValue::MapFlags(MAP_HAS_ALPHANUMERIC);
        }
        25 => {
            *yyval = YYValue::MapFlags(MAP_HAS_MODIFIER);
        }
        26 => {
            *yyval = YYValue::MapFlags(MAP_HAS_KEYPAD);
        }
        27 => {
            *yyval = YYValue::MapFlags(MAP_HAS_FN);
        }
        28 => {
            *yyval = YYValue::MapFlags(MAP_IS_ALTGR);
        }
        29 => {
            // DeclList: DeclList Decl
            let stmt = std::mem::replace(&mut yyvs[sp], YYValue::None);
            let mut list = yyvs[sp - 1].take_stmt_list();
            if let YYValue::Stmt(s) = stmt {
                list.push(s);
            }
            *yyval = YYValue::StmtList(list);
        }
        30 => {
            // DeclList: DeclList OptMergeMode VModDecl
            let merge = yyvs[sp - 1].as_merge();
            let mut vmods = yyvs[sp].take_vmod_list();
            for v in &mut vmods {
                v.merge = merge;
            }
            let mut list = yyvs[sp - 2].take_stmt_list();
            for v in vmods {
                list.push(Statement::VMod(v));
            }
            *yyval = YYValue::StmtList(list);
        }
        31 => {
            // DeclList: empty
            *yyval = YYValue::StmtList(Vec::new());
        }
        32 => {
            // Decl: OptMergeMode VarDecl
            let merge = yyvs[sp - 1].as_merge();
            if let Some(mut var) = yyvs[sp].take_var() {
                var.merge = merge;
                *yyval = YYValue::Stmt(Statement::Var(var));
            } else {
                *yyval = YYValue::None;
            }
        }
        33 => {
            // Decl: OptMergeMode InterpretDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::Interp(mut interp) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                interp.merge = merge;
                *yyval = YYValue::Stmt(Statement::Interp(interp));
            } else {
                *yyval = YYValue::None;
            }
        }
        34 => {
            // Decl: OptMergeMode KeyNameDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::Keycode(mut kc) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                kc.merge = merge;
                *yyval = YYValue::Stmt(Statement::Keycode(kc));
            } else {
                *yyval = YYValue::None;
            }
        }
        35 => {
            // Decl: OptMergeMode KeyAliasDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::KeyAlias(mut ka) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                ka.merge = merge;
                *yyval = YYValue::Stmt(Statement::KeyAlias(ka));
            } else {
                *yyval = YYValue::None;
            }
        }
        36 => {
            // Decl: OptMergeMode KeyTypeDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::KeyType(mut kt) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                kt.merge = merge;
                *yyval = YYValue::Stmt(Statement::KeyType(kt));
            } else {
                *yyval = YYValue::None;
            }
        }
        37 => {
            // Decl: OptMergeMode SymbolsDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::Symbols(mut sym) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                sym.merge = merge;
                *yyval = YYValue::Stmt(Statement::Symbols(sym));
            } else {
                *yyval = YYValue::None;
            }
        }
        38 => {
            // Decl: OptMergeMode ModMapDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::ModMask(mut mm) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                mm.merge = merge;
                *yyval = YYValue::Stmt(Statement::ModMap(mm));
            } else {
                *yyval = YYValue::None;
            }
        }
        39 => {
            if let YYValue::GroupCompat(_) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::GroupCompat(()));
            } else {
                *yyval = YYValue::None;
            }
        }
        40 => {
            // Decl: OptMergeMode LedMapDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::LedMap(mut lm) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                lm.merge = merge;
                *yyval = YYValue::Stmt(Statement::LedMap(lm));
            } else {
                *yyval = YYValue::None;
            }
        }
        41 => {
            // Decl: OptMergeMode LedNameDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::LedName(mut ln) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                ln.merge = merge;
                *yyval = YYValue::Stmt(Statement::LedName(ln));
            } else {
                *yyval = YYValue::None;
            }
        }
        42..=44 => {
            // ShapeDecl, SectionDecl, DoodadDecl → geometry (ignored)
            *yyval = YYValue::None;
        }
        45 => {
            // Decl: OptMergeMode UnknownDecl
            if let YYValue::Unknown(u) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::Unknown(u));
            } else {
                *yyval = YYValue::None;
            }
        }
        46 => {
            // Decl: OptMergeMode UnknownCompoundStatementDecl
            if let YYValue::Unknown(u) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::Unknown(u));
            } else {
                *yyval = YYValue::None;
            }
        }
        47 => {
            // Decl: MergeMode STRING
            let merge = yyvs[sp - 1].as_merge();
            let s = yyvs[sp].take_str();
            if let Some(inc) = IncludeCreate(param.ctx, &s, merge) {
                *yyval = YYValue::Stmt(Statement::Include(inc));
            } else {
                *yyval = YYValue::None;
            }
        }
        48 => {
            // VarDecl: Lhs EQUALS Expr SEMI
            let lhs = yyvs[sp - 3].take_expr();
            let val = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Var(VarCreate(lhs, val));
        }
        49 => {
            // VarDecl: Ident SEMI
            let atom = yyvs[sp - 1].as_atom();
            *yyval = YYValue::Var(BoolVarCreate(atom, true));
        }
        50 => {
            // VarDecl: EXCLAM Ident SEMI
            let atom = yyvs[sp - 1].as_atom();
            *yyval = YYValue::Var(BoolVarCreate(atom, false));
        }
        51 => {
            // KeyNameDecl: KEYNAME EQUALS KeyCode SEMI
            let atom = yyvs[sp - 3].as_atom();
            let num = yyvs[sp - 1].as_num();
            *yyval = YYValue::Keycode(KeycodeCreate(atom, num));
        }
        52 => {
            // KeyAliasDecl: ALIAS KEYNAME EQUALS KEYNAME SEMI
            let alias = yyvs[sp - 3].as_atom();
            let real = yyvs[sp - 1].as_atom();
            *yyval = YYValue::KeyAlias(KeyAliasCreate(alias, real));
        }
        53 => {
            // VModDecl: VIRTUAL_MODS VModDefList SEMI
            let list = yyvs[sp - 1].take_vmod_list();
            *yyval = YYValue::VModList(list);
        }
        54 => {
            // VModDefList: VModDefList COMMA VModDef
            let vmod = yyvs[sp].take_vmod();
            let mut list = yyvs[sp - 2].take_vmod_list();
            if let Some(v) = vmod {
                list.push(v);
            }
            *yyval = YYValue::VModList(list);
        }
        55 => {
            // VModDefList: VModDef
            let vmod = yyvs[sp].take_vmod();
            let mut list = Vec::new();
            if let Some(v) = vmod {
                list.push(v);
            }
            *yyval = YYValue::VModList(list);
        }
        56 => {
            // VModDef: Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::VMod(VModCreate(atom, None));
        }
        57 => {
            // VModDef: Ident EQUALS Expr
            let atom = yyvs[sp - 2].as_atom();
            let expr = yyvs[sp].take_expr();
            *yyval = YYValue::VMod(VModCreate(atom, expr));
        }
        58 => {
            // InterpretDecl: INTERPRET InterpretMatch OBRACE VarDeclList CBRACE SEMI
            if let YYValue::Interp(mut interp) = std::mem::replace(&mut yyvs[sp - 4], YYValue::None)
            {
                let vardefs = yyvs[sp - 2].take_var_list();
                interp.def = vardefs.into_iter().map(|b| *b).collect();
                *yyval = YYValue::Interp(interp);
            } else {
                *yyval = YYValue::None;
            }
        }
        59 => {
            // InterpretMatch: KeySym PLUS Expr
            let keysym = yyvs[sp - 2].as_keysym();
            let expr = yyvs[sp].take_expr();
            *yyval = YYValue::Interp(InterpCreate(keysym, expr));
        }
        60 => {
            // InterpretMatch: KeySym
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Interp(InterpCreate(keysym, None));
        }
        61 => {
            // VarDeclList: VarDeclList VarDecl
            let var = yyvs[sp].take_var();
            let mut list = yyvs[sp - 1].take_var_list();
            if let Some(v) = var {
                list.push(v);
            }
            *yyval = YYValue::VarList(list);
        }
        62 => {
            // VarDeclList: empty
            *yyval = YYValue::VarList(Vec::new());
        }
        63 => {
            // KeyTypeDecl: TYPE String OBRACE VarDeclList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::KeyType(KeyTypeCreate(
                atom,
                vardefs.into_iter().map(|b| *b).collect(),
            ));
        }
        64 => {
            // SymbolsDecl: KEY KEYNAME OBRACE OptSymbolsBody CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Symbols(SymbolsCreate(
                atom,
                vardefs.into_iter().map(|b| *b).collect(),
            ));
        }
        65 => {
            // OptSymbolsBody: SymbolsBody
            let list = yyvs[sp].take_var_list();
            *yyval = YYValue::VarList(list);
        }
        66 => {
            // OptSymbolsBody: empty
            *yyval = YYValue::VarList(Vec::new());
        }
        67 => {
            // SymbolsBody: SymbolsBody COMMA SymbolsVarDecl
            let var = yyvs[sp].take_var();
            let mut list = yyvs[sp - 2].take_var_list();
            if let Some(v) = var {
                list.push(v);
            }
            *yyval = YYValue::VarList(list);
        }
        68 => {
            // SymbolsBody: SymbolsVarDecl
            let var = yyvs[sp].take_var();
            let mut list = Vec::new();
            if let Some(v) = var {
                list.push(v);
            }
            *yyval = YYValue::VarList(list);
        }
        69 => {
            // SymbolsVarDecl: Lhs EQUALS Expr
            let lhs = yyvs[sp - 2].take_expr();
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(VarCreate(lhs, val));
        }
        70 => {
            // SymbolsVarDecl: Lhs EQUALS MultiKeySymOrActionList
            let lhs = yyvs[sp - 2].take_expr();
            // MultiKeySymOrActionList is an ExprList or Expr
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(VarCreate(lhs, val));
        }
        71 => {
            // SymbolsVarDecl: Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Var(BoolVarCreate(atom, true));
        }
        72 => {
            // SymbolsVarDecl: EXCLAM Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Var(BoolVarCreate(atom, false));
        }
        73 => {
            // SymbolsVarDecl: Expr
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(VarCreate(None, val));
        }
        74 => {
            // MultiKeySymOrActionList: OBRACKET MultiKeySymList CBRACKET (yylen=3)
            let list = yyvs[sp - 1].take_expr_list();
            let exprs: Vec<ExprDef> = list.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        75 => {
            // MultiKeySymOrActionList: NoSymbolOrActionList OBRACKET MultiKeySymList CBRACKET COMMA (yylen=5)
            let mut list = yyvs[sp - 1].take_expr_list(); // sp-1 = MultiKeySymList = offset(-1)
            let count = yyvs[sp - 3].as_no_sym_or_action_list(); // sp-3 = NoSymbolOrActionList = offset(-3)
                                                                 // Prepend 'count' NoSymbol keysym lists
            let mut prepended: Vec<Box<ExprDef>> = Vec::new();
            for _ in 0..count {
                prepended.push(ExprCreateKeySymList(XKB_KEY_NO_SYMBOL as u32));
            }
            prepended.append(&mut list);
            let exprs: Vec<ExprDef> = prepended.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        76 => {
            // MultiKeySymOrActionList: OBRACKET MultiActionList CBRACKET (yylen=3)
            let list = yyvs[sp - 1].take_expr_list();
            let exprs: Vec<ExprDef> = list.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        77 => {
            // MultiKeySymOrActionList: NoSymbolOrActionList OBRACKET MultiActionList CBRACKET COMMA (yylen=5)
            let mut list = yyvs[sp - 1].take_expr_list();
            let count = yyvs[sp - 3].as_no_sym_or_action_list();
            let mut prepended: Vec<Box<ExprDef>> = Vec::new();
            for _ in 0..count {
                prepended.push(expr_create(ExprKind::ActionList {
                    actions: Vec::new(),
                }));
            }
            prepended.append(&mut list);
            let exprs: Vec<ExprDef> = prepended.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        78 => {
            // NoSymbolOrActionList: NoSymbol (produces EmptyList expr)
            *yyval = YYValue::Expr(expr_create(ExprKind::EmptyList));
        }
        79 => {
            // NoSymbolOrActionList: NoSymbolOrActionList COMMA NoSymbol COMMA (yylen=4)
            let prev = yyvs[sp - 3].as_no_sym_or_action_list();
            *yyval = YYValue::NoSymbolOrActionList(prev.wrapping_add(1));
        }
        80 => {
            // NoSymbolOrActionList: ... (yylen=2)
            *yyval = YYValue::NoSymbolOrActionList(1);
        }
        81 => {
            // NoSymbolOrActionList: empty
            *yyval = YYValue::NoSymbolOrActionList(0);
        }
        82 => {
            // GroupCompatDecl: GROUP Integer EQUALS Expr SEMI
            *yyval = YYValue::GroupCompat(GroupCompatCreate());
        }
        83 => {
            // ModMapDecl: MODIFIER_MAP Ident OBRACE KeyOrKeySymList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let list = yyvs[sp - 2].take_expr_list();
            *yyval = YYValue::ModMask(ModMapCreate(atom, list.into_iter().map(|b| *b).collect()));
        }
        84 => {
            // KeyOrKeySymList: KeyOrKeySymList COMMA KeyOrKeySym
            let expr = yyvs[sp].take_expr();
            let mut list = yyvs[sp - 2].take_expr_list();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        85 => {
            // KeyOrKeySymList: KeyOrKeySym
            let expr = yyvs[sp].take_expr();
            let mut list = Vec::new();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        86 => {
            // KeyOrKeySym: KEYNAME
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(expr_create(ExprKind::KeyName(atom)));
        }
        87 => {
            // KeyOrKeySym: KeySym
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(expr_create(ExprKind::KeySym(keysym)));
        }
        88 => {
            // LedMapDecl: INDICATOR String OBRACE VarDeclList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::LedMap(LedMapCreate(
                atom,
                vardefs.into_iter().map(|b| *b).collect(),
            ));
        }
        89 => {
            // LedNameDecl: INDICATOR Integer EQUALS Expr SEMI
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::LedName(LedNameCreate(num, expr));
        }
        90 => {
            // LedNameDecl: VIRTUAL INDICATOR Integer EQUALS Expr SEMI
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::LedName(LedNameCreate(num, expr));
        }
        91 => {
            // UnknownDecl: Ident Lhs EQUALS Expr SEMI
            // Drop expr values (geometry not supported)
            let _ = yyvs[sp - 3].take_expr();
            let _ = yyvs[sp - 1].take_expr();
            let sval = yyvs[sp - 4].as_sval();
            let name_str = sval.as_str();
            *yyval = YYValue::Unknown(UnknownStatementCreate(STMT_UNKNOWN_DECLARATION, name_str));
        }
        92 => {
            // UnknownCompoundStatementDecl: Ident Lhs OBRACE VarDeclList CBRACE SEMI
            let _ = yyvs[sp - 4].take_expr();
            let _ = yyvs[sp - 2].take_var_list();
            let sval = yyvs[sp - 5].as_sval();
            let name_str = sval.as_str();
            *yyval = YYValue::Unknown(UnknownStatementCreate(STMT_UNKNOWN_COMPOUND, name_str));
        }
        // Rules 93-123: Geometry rules → all produce None (geometry not supported)
        93 | 94 | 95 | 96 | 97 | 98 | 100 | 102 | 103 | 104 | 105 | 107 | 108 | 109 | 111 | 112
        | 113 | 114 | 115 | 116 | 117 | 118 => {
            *yyval = YYValue::None;
        }
        99 => {
            // SectionBodyItem: VarDecl → drop it (geometry)
            let _ = yyvs[sp].take_var();
            *yyval = YYValue::None;
        }
        101 => {
            // SectionBodyItem: LedMapDecl → drop it (geometry)
            let _ = std::mem::replace(&mut yyvs[sp], YYValue::None);
            *yyval = YYValue::None;
        }
        106 => {
            // RowBodyItem: VarDecl → drop it (geometry)
            let _ = yyvs[sp].take_var();
            *yyval = YYValue::None;
        }
        110 => {
            // OverlayDecl: ... → drop ExprList (geometry)
            let _ = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::None;
        }
        119 => {
            // CoordList: ... → drop expr (geometry)
            let _ = yyvs[sp].take_expr();
            *yyval = YYValue::None;
        }
        120..=122 => {
            // OutlineInList/OutlineList → geometry, drop expr/null
            let _ = yyvs[sp].take_expr();
            *yyval = YYValue::None;
        }
        123 => {
            // ShapeDecl → drop VarDeclList (geometry)
            let _ = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::None;
        }
        // DoodadType rules 124-127
        124..=127 => {
            *yyval = YYValue::Num(0);
        }
        // FieldSpec / Element rules 128-140
        128 => {
            *yyval = YYValue::Atom(yyvs[sp].as_atom());
        }
        129 => {
            *yyval = YYValue::Atom(yyvs[sp].as_atom());
        }
        130 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"action", true));
        }
        131 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"interpret", true));
        }
        132 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"type", true));
        }
        133 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"key", true));
        }
        134 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"group", true));
        }
        135 => {
            *yyval = YYValue::Atom(atom_intern(
                &mut param.ctx.atom_table,
                b"modifier_map",
                true,
            ));
        }
        136 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"indicator", true));
        }
        137 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"shape", true));
        }
        138 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"row", true));
        }
        139 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"section", true));
        }
        140 => {
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"text", true));
        }
        // MergeMode rules 141-147
        141 => {
            *yyval = YYValue::Merge(yyvs[sp].as_merge());
        }
        142 => {
            *yyval = YYValue::Merge(MERGE_DEFAULT);
        }
        143 => {
            *yyval = YYValue::Merge(MERGE_DEFAULT);
        }
        144 => {
            *yyval = YYValue::Merge(MERGE_AUGMENT);
        }
        145 => {
            *yyval = YYValue::Merge(MERGE_OVERRIDE);
        }
        146 => {
            *yyval = YYValue::Merge(MERGE_REPLACE);
        }
        147 => {
            let loc = param.scanner.token_location();
            log::warn!(
                "{}:{}:{}: ignored unsupported legacy merge mode \"alternate\"\n",
                &param.scanner.file_name,
                loc.line,
                loc.column
            );
            *yyval = YYValue::Merge(MERGE_DEFAULT);
        }
        // ExprList rules 148-150
        148 => {
            // ExprList: ExprList COMMA Expr
            let expr = yyvs[sp].take_expr();
            let mut list = yyvs[sp - 2].take_expr_list();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        149 => {
            // ExprList: Expr
            let expr = yyvs[sp].take_expr();
            let mut list = Vec::new();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        150 => {
            // ExprList: empty
            *yyval = YYValue::ExprList(Vec::new());
        }
        // Expr rules 151-165
        151 => {
            let left = yyvs[sp - 2].take_expr();
            let right = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Binary {
                op: STMT_EXPR_DIVIDE,
                left,
                right,
            }));
        }
        152 => {
            let left = yyvs[sp - 2].take_expr();
            let right = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Binary {
                op: STMT_EXPR_ADD,
                left,
                right,
            }));
        }
        153 => {
            let left = yyvs[sp - 2].take_expr();
            let right = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Binary {
                op: STMT_EXPR_SUBTRACT,
                left,
                right,
            }));
        }
        154 => {
            let left = yyvs[sp - 2].take_expr();
            let right = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Binary {
                op: STMT_EXPR_MULTIPLY,
                left,
                right,
            }));
        }
        155 => {
            let left = yyvs[sp - 2].take_expr();
            let right = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Binary {
                op: STMT_EXPR_ASSIGN,
                left,
                right,
            }));
        }
        156 => {
            // Expr: Term
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        157 => {
            let child = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Unary {
                op: STMT_EXPR_NEGATE,
                child,
            }));
        }
        158 => {
            let child = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Unary {
                op: STMT_EXPR_UNARY_PLUS,
                child,
            }));
        }
        159 => {
            let child = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Unary {
                op: STMT_EXPR_NOT,
                child,
            }));
        }
        160 => {
            let child = yyvs[sp].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::Unary {
                op: STMT_EXPR_INVERT,
                child,
            }));
        }
        161 => {
            // Term: Lhs (passthrough)
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        162 => {
            // Term: Action OPAREN ExprList CPAREN
            let name = yyvs[sp - 3].as_atom();
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(expr_create(ExprKind::Action {
                name,
                args: list.into_iter().map(|b| *b).collect(),
            }));
        }
        163 => {
            // Term: Terminal
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        164 => {
            // Term: OPAREN Expr CPAREN → passthrough the expr
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        165 => {
            // Term: OPAREN Expr CPAREN
            *yyval = std::mem::replace(&mut yyvs[sp - 1], YYValue::None);
        }
        // MultiActionList rules 166-167
        166 => {
            // MultiActionList: MultiActionList COMMA ActionList
            // ActionList at sp produces an ExprList of actions
            // Create an ActionList expr wrapping those actions, then append to the list
            let actions_expr_list = yyvs[sp].take_expr_list();
            let actions: Vec<ExprDef> = actions_expr_list.into_iter().map(|b| *b).collect();
            let action_list_expr = expr_create(ExprKind::ActionList { actions });
            let mut list = yyvs[sp - 2].take_expr_list();
            list.push(action_list_expr);
            *yyval = YYValue::ExprList(list);
        }
        167 => {
            // MultiActionList: MultiActionList COMMA KeySymList
            let keysym_expr = yyvs[sp].take_expr();
            let mut list = yyvs[sp - 2].take_expr_list();
            if let Some(e) = keysym_expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        168 => {
            // MultiActionList: ActionList (initial single element)
            let actions_expr_list = yyvs[sp].take_expr_list();
            let actions: Vec<ExprDef> = actions_expr_list.into_iter().map(|b| *b).collect();
            let action_list_expr = expr_create(ExprKind::ActionList { actions });
            *yyval = YYValue::ExprList(vec![action_list_expr]);
        }
        169 => {
            // MultiActionList: KeySymList (initial single element)
            let expr = yyvs[sp].take_expr();
            let mut list = Vec::new();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        // NonEmptyActions rules 170-171
        170 => {
            // NonEmptyActions: NonEmptyActions COMMA Action
            let expr = yyvs[sp].take_expr();
            let mut list = yyvs[sp - 2].take_expr_list();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        171 => {
            // NonEmptyActions: Action
            let expr = yyvs[sp].take_expr();
            let mut list = Vec::new();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        // Actions / ActionList rules 172-175
        172 => {
            // Actions: OBRACE NonEmptyActions CBRACE
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList {
                actions: list.into_iter().map(|b| *b).collect(),
            }));
        }
        173 => {
            // ActionList: Action
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        174 => {
            // ActionList: empty (yylen=0 means nothing on stack)
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList {
                actions: Vec::new(),
            }));
        }
        175 => {
            // Action: FieldSpec OPAREN ExprList CPAREN
            let name = yyvs[sp - 3].as_atom();
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(expr_create(ExprKind::Action {
                name,
                args: list.into_iter().map(|b| *b).collect(),
            }));
        }
        // Lhs rules 176-179
        176 => {
            // Lhs: Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(expr_create(ExprKind::Ident(atom)));
        }
        177 => {
            // Lhs: Ident DOT FieldSpec
            let element = yyvs[sp - 2].as_atom();
            let field = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(expr_create(ExprKind::FieldRef { element, field }));
        }
        178 => {
            // Lhs: Ident OBRACKET Expr CBRACKET
            let field = yyvs[sp - 3].as_atom();
            let entry = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::ArrayRef {
                element: XKB_ATOM_NONE,
                field,
                entry,
            }));
        }
        179 => {
            // Lhs: Ident DOT Ident OBRACKET Expr CBRACKET
            let element = yyvs[sp - 5].as_atom();
            let field = yyvs[sp - 3].as_atom();
            let entry = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Expr(expr_create(ExprKind::ArrayRef {
                element,
                field,
                entry,
            }));
        }
        // OptTerminal / Terminal 180-181
        180 => {
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        181 => {
            *yyval = YYValue::None;
        }
        // Terminal rules 182-185
        182 => {
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(expr_create(ExprKind::String(atom)));
        }
        183 => {
            let num = yyvs[sp].as_num();
            *yyval = YYValue::Expr(expr_create(ExprKind::Integer(num)));
        }
        184 => {
            *yyval = YYValue::Expr(expr_create(ExprKind::Float));
        }
        185 => {
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(expr_create(ExprKind::KeyName(atom)));
        }
        // MultiKeySymList rules 186-189
        186 => {
            // MultiKeySymList: MultiKeySymList COMMA KeySymList
            let keysym = yyvs[sp].as_keysym();
            let expr = ExprCreateKeySymList(keysym);
            let mut list = yyvs[sp - 2].take_expr_list();
            list.push(expr);
            *yyval = YYValue::ExprList(list);
        }
        187 => {
            // MultiKeySymList: MultiKeySymList COMMA KeySymList (expr variant)
            let expr = yyvs[sp].take_expr();
            let mut list = yyvs[sp - 2].take_expr_list();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        188 => {
            // MultiKeySymList: KeySymList (keysym)
            let keysym = yyvs[sp].as_keysym();
            let expr = ExprCreateKeySymList(keysym);
            *yyval = YYValue::ExprList(vec![expr]);
        }
        189 => {
            // MultiKeySymList: KeySymList (expr)
            let expr = yyvs[sp].take_expr();
            let mut list = Vec::new();
            if let Some(e) = expr {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        // KeySymList rules 190-197
        190 => {
            // NonEmptyKeySyms: NonEmptyKeySyms COMMA KeySym
            let expr = yyvs[sp - 2].take_expr().unwrap();
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(ExprAppendKeySymList(expr, keysym));
        }
        191 => {
            // NonEmptyKeySyms: NonEmptyKeySyms COMMA STRING
            let expr = yyvs[sp - 2].take_expr().unwrap();
            let s = yyvs[sp].take_str();
            match ExprKeySymListAppendString(param.scanner, expr, &s) {
                Some(e) => {
                    *yyval = YYValue::Expr(e);
                }
                None => {
                    return false;
                }
            }
        }
        192 => {
            // KeySyms: KeySym
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(ExprCreateKeySymList(keysym));
        }
        193 => {
            // KeySyms: STRING (single string keysym)
            let s = yyvs[sp].take_str();
            let expr = ExprCreateKeySymList(XKB_KEY_NO_SYMBOL as u32);
            match ExprKeySymListAppendString(param.scanner, expr, &s) {
                Some(e) => {
                    *yyval = YYValue::Expr(e);
                }
                None => {
                    return false;
                }
            }
        }
        194 => {
            // KeySymList: OBRACKET NonEmptyKeySyms CBRACKET
            *yyval = std::mem::replace(&mut yyvs[sp - 1], YYValue::None);
        }
        195 => {
            // KeySymList: STRING (produces keysym list from string)
            let s = yyvs[sp].take_str();
            let expr = ExprCreateKeySymList(XKB_KEY_NO_SYMBOL as u32);
            match ExprKeySymListAppendString(param.scanner, expr, &s) {
                Some(e) => {
                    *yyval = YYValue::Expr(e);
                }
                None => {
                    return false;
                }
            }
        }
        196 => {
            // KeySymList: KeySyms
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        197 => {
            // KeySymList: empty → NoSymbol
            *yyval = YYValue::Expr(ExprCreateKeySymList(XKB_KEY_NO_SYMBOL as u32));
        }
        // KeySym rules 198-203
        198 => {
            // KeySymLit: KeySym (passthrough)
            *yyval = YYValue::Keysym(yyvs[sp].as_keysym());
        }
        199 => {
            // KeySym: STRING → parse string as keysym
            let s = yyvs[sp].take_str();
            let keysym = KeysymParseString(param.scanner, &s);
            if keysym == XKB_KEY_NO_SYMBOL as u32 {
                return false;
            }
            *yyval = YYValue::Keysym(keysym);
        }
        200 => {
            // KeySym: IDENT → resolve keysym name
            let sval = yyvs[sp].as_sval();
            match resolve_keysym(param, sval) {
                Some(sym) => {
                    *yyval = YYValue::Keysym(sym);
                }
                None => {
                    let loc = param.scanner.token_location();
                    log::warn!(
                        "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"{}\"\n",
                        XKB_WARNING_UNRECOGNIZED_KEYSYM as i32,
                        &param.scanner.file_name,
                        loc.line,
                        loc.column,
                        sval.as_str()
                    );
                    *yyval = YYValue::Keysym(XKB_KEY_NO_SYMBOL as u32);
                }
            }
        }
        201 => {
            // KeySym: SECTION
            *yyval = YYValue::Keysym(XKB_KEY_section as u32);
        }
        202 => {
            // KeySym: DECIMAL_DIGIT
            let num = yyvs[sp].as_num();
            *yyval = YYValue::Keysym((XKB_KEY_0 as u32).wrapping_add(num as u32));
        }
        203 => {
            // KeySym: INTEGER (numeric keysym)
            let num = yyvs[sp].as_num();
            if num < XKB_KEYSYM_MIN as i64 {
                let loc = param.scanner.token_location();
                log::warn!(
                    "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"-{:#06x}\" ({})\n",
                    XKB_ERROR_INVALID_NUMERIC_KEYSYM as i32,
                    &param.scanner.file_name,
                    loc.line,
                    loc.column,
                    -num,
                    num
                );
                *yyval = YYValue::Keysym(XKB_KEY_NO_SYMBOL as u32);
            } else {
                if num <= XKB_KEYSYM_MAX as i64 {
                    let keysym = num as u32;
                    if param.ctx.log_verbosity >= 2 {
                        if let Some(ref_name) = xkb_keysym_is_deprecated(keysym, &[]) {
                            let loc = param.scanner.token_location();
                            log::debug!(
                                "[XKB-{:03}] {}:{}:{}: deprecated keysym name \"{:#06x}\"; please use \"{}\" instead.\n",
                                XKB_WARNING_DEPRECATED_KEYSYM_NAME as i32,
                                &param.scanner.file_name,
                                loc.line,
                                loc.column,
                                keysym,
                                ref_name
                            );
                        }
                    }
                    *yyval = YYValue::Keysym(keysym);
                } else {
                    let loc = param.scanner.token_location();
                    log::warn!(
                        "[XKB-{:03}] {}:{}:{}: unrecognized keysym \"{:#06x}\" ({})\n",
                        XKB_ERROR_INVALID_NUMERIC_KEYSYM as i32,
                        &param.scanner.file_name,
                        loc.line,
                        loc.column,
                        num,
                        num
                    );
                    *yyval = YYValue::Keysym(XKB_KEY_NO_SYMBOL as u32);
                }
                let loc = param.scanner.token_location();
                log::debug!(
                    "[XKB-{:03}] {}:{}:{}: numeric keysym \"{:#06x}\" ({})\n",
                    XKB_WARNING_NUMERIC_KEYSYM as i32,
                    &param.scanner.file_name,
                    loc.line,
                    loc.column,
                    num,
                    num
                );
            }
        }
        // SignedNumber / Number rules 204-208
        204 => {
            *yyval = YYValue::Num(-yyvs[sp].as_num());
        }
        205..=208 => {
            *yyval = YYValue::Num(yyvs[sp].as_num());
        }
        // Float 209
        209 => {
            *yyval = YYValue::Num(0);
        }
        // Integer, KeyCode 210-213
        210..=213 => {
            *yyval = YYValue::Num(yyvs[sp].as_num());
        }
        // Ident 214
        214 => {
            let sval = yyvs[sp].as_sval();
            *yyval = YYValue::Atom(atom_intern(
                &mut param.ctx.atom_table,
                sval.as_bytes(),
                true,
            ));
        }
        215 => {
            // Ident: DEFAULT
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"default", true));
        }
        // String 216
        216 => {
            // String: STRING → intern as atom
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, s.as_bytes(), true));
        }
        // OptMapName / MapName 217-219
        217 => {
            // MapName: STRING
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Str(s);
        }
        218 => {
            // OptMapName: empty
            *yyval = YYValue::Str(String::new());
        }
        219 => {
            // MapName: STRING
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Str(s);
        }
        _ => {
            // Default: no action
        }
    }
    true
}

// ── Public API ──────────────────────────────────────────────────────

pub(crate) fn parse<'a>(
    mut ctx: &'a mut XkbContext,
    mut scanner: &'a mut scanner<'a>,
    map: &str,
) -> Option<Box<XkbFile>> {
    let mut first: Option<Box<XkbFile>> = None;

    loop {
        let mut param = parser_param {
            ctx,
            scanner,
            rtrn: None,
            more_maps: false,
        };

        let ret = _xkbcommon_parse(&mut param);
        // Recover ctx and scanner from param before it's dropped
        ctx = param.ctx;
        scanner = param.scanner;

        if ret == 0 && param.more_maps {
            if !map.is_empty() {
                if let Some(ref rtrn) = param.rtrn {
                    if map == rtrn.name {
                        return param.rtrn;
                    }
                }
                // Not the map we want, drop it
            } else if let Some(ref rtrn) = param.rtrn {
                if rtrn.flags & MAP_IS_DEFAULT != 0 {
                    return param.rtrn;
                } else if first.is_none() {
                    first = param.rtrn;
                }
                // else drop param.rtrn
            }
            continue;
        }

        if ret != 0 {
            return None;
        }

        if param.rtrn.is_some() {
            return param.rtrn;
        }

        break;
    }

    if let Some(first_ref) = first.as_ref() {
        log::debug!(
            "[XKB-{:03}] No map in include statement, but \"{}\" contains several; Using first defined map, \"{}\"\n",
            XKB_WARNING_MISSING_DEFAULT_SECTION as i32,
            &scanner.file_name,
            safe_map_name(first_ref)
        );
    }
    first
}

pub(crate) fn parse_next<'a>(
    ctx: &'a mut XkbContext,
    scanner: &'a mut scanner<'a>,
    xkb_file: &mut Option<Box<XkbFile>>,
) -> bool {
    let mut param = parser_param {
        ctx,
        scanner,
        rtrn: None,
        more_maps: false,
    };

    let ret = _xkbcommon_parse(&mut param);
    if ret == 0 && param.more_maps {
        *xkb_file = param.rtrn;
        true
    } else {
        *xkb_file = None;
        ret == 0
    }
}

// ── AST builder functions (merged from ast_build.rs) ──

pub(crate) fn expr_create(kind: ExprKind) -> Box<ExprDef> {
    Box::new(ExprDef { kind })
}

pub(crate) fn ExprCreateKeySymList(sym: u32) -> Box<ExprDef> {
    let mut syms = Vec::new();
    if sym != XKB_KEY_NO_SYMBOL as u32 {
        syms.push(sym);
    }
    expr_create(ExprKind::KeysymList { syms })
}

pub(crate) fn ExprAppendKeySymList(mut expr: Box<ExprDef>, sym: u32) -> Box<ExprDef> {
    if sym != XKB_KEY_NO_SYMBOL as u32 {
        if let ExprKind::KeysymList { ref mut syms } = expr.kind {
            syms.push(sym);
        }
    }
    expr
}

pub(crate) fn ExprKeySymListAppendString(
    scanner: &mut scanner,
    mut expr: Box<ExprDef>,
    string: &str,
) -> Option<Box<ExprDef>> {
    let bytes = string.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;
    let mut idx_cp: usize = 1;
    loop {
        if idx >= len {
            return Some(expr);
        }
        let (cp, cp_len) = utf8_next_code_point_safe(&bytes[idx..]);
        if cp == INVALID_UTF8_CODE_POINT {
            let loc = scanner.token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to keysyms: Invalid UTF-8 encoding starting at byte position {} (code point position: {}).\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &scanner.file_name,
                loc.line,
                loc.column,
                idx + 1,
                idx_cp);
            return None;
        }
        let sym = utf32_to_keysym(cp);
        if sym == XKB_KEY_NO_SYMBOL as u32 {
            let loc = scanner.token_location();
            log::error!("{}:{}:{}: Cannot convert string to keysyms: Unicode code point U+04{:X} has no keysym equivalent(byte position: {}, code point position: {}).\n",
                &scanner.file_name,
                loc.line,
                loc.column,
                cp,
                idx + 1,
                idx_cp);
            return None;
        }
        if let ExprKind::KeysymList { ref mut syms } = expr.kind {
            syms.push(sym);
        }
        idx += cp_len;
        idx_cp += 1;
    }
}

pub(crate) fn KeysymParseString(scanner: &mut scanner, string: &str) -> u32 {
    let bytes = string.as_bytes();
    let len = bytes.len();
    if len == 0 {
        let loc = scanner.token_location();
        log::error!(
            "{}:{}:{}: Cannot convert string to single keysym: empty string.\n",
            &scanner.file_name,
            loc.line,
            loc.column
        );
        return XKB_KEY_NO_SYMBOL as u32;
    }
    let (cp, cp_len) = utf8_next_code_point_safe(bytes);
    if cp == INVALID_UTF8_CODE_POINT {
        let loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Invalid UTF-8 encoding.\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &scanner.file_name,
            loc.line,
            loc.column);
        return XKB_KEY_NO_SYMBOL as u32;
    } else if cp_len != len {
        let loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Expected a single Unicode code point, got: \"{}\".\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &scanner.file_name,
            loc.line,
            loc.column,
            string);
        return XKB_KEY_NO_SYMBOL as u32;
    }
    let sym = utf32_to_keysym(cp);
    if sym == XKB_KEY_NO_SYMBOL as u32 {
        let loc = scanner.token_location();
        log::error!("{}:{}:{}: Cannot convert string to single keysym: Unicode code point U+{:04X} has no keysym equivalent.\n",
            &scanner.file_name,
            loc.line,
            loc.column,
            cp);
    }
    sym
}

pub(crate) fn KeycodeCreate(name: u32, value: i64) -> Box<KeycodeDef> {
    Box::new(KeycodeDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub(crate) fn KeyAliasCreate(alias: u32, real: u32) -> Box<KeyAliasDef> {
    Box::new(KeyAliasDef {
        merge: MERGE_DEFAULT,
        alias,
        real,
    })
}

pub(crate) fn VModCreate(name: u32, value: Option<Box<ExprDef>>) -> Box<VModDef> {
    Box::new(VModDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub(crate) fn VarCreate(name: Option<Box<ExprDef>>, value: Option<Box<ExprDef>>) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub(crate) fn BoolVarCreate(ident: u32, set: bool) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name: Some(Box::new(ExprDef {
            kind: ExprKind::Ident(ident),
        })),
        value: Some(Box::new(ExprDef {
            kind: ExprKind::Boolean(set),
        })),
    })
}

pub(crate) fn InterpCreate(sym: u32, match_0: Option<Box<ExprDef>>) -> Box<InterpDef> {
    Box::new(InterpDef {
        merge: MERGE_DEFAULT,
        sym,
        match_0,
        def: Vec::new(),
    })
}

pub(crate) fn KeyTypeCreate(name: u32, body: Vec<VarDef>) -> Box<KeyTypeDef> {
    Box::new(KeyTypeDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub(crate) fn SymbolsCreate(key_name: u32, symbols: Vec<VarDef>) -> Box<SymbolsDef> {
    Box::new(SymbolsDef {
        merge: MERGE_DEFAULT,
        key_name,
        symbols,
    })
}

pub(crate) fn GroupCompatCreate() -> Box<GroupCompatDef> {
    Box::new(GroupCompatDef {})
}

pub(crate) fn ModMapCreate(modifier: u32, keys: Vec<ExprDef>) -> Box<ModMapDef> {
    Box::new(ModMapDef {
        merge: MERGE_DEFAULT,
        modifier,
        keys,
    })
}

pub(crate) fn LedMapCreate(name: u32, body: Vec<VarDef>) -> Box<LedMapDef> {
    Box::new(LedMapDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub(crate) fn LedNameCreate(ndx: i64, name: Option<Box<ExprDef>>) -> Box<LedNameDef> {
    Box::new(LedNameDef {
        merge: MERGE_DEFAULT,
        ndx,
        name,
    })
}

pub(crate) fn UnknownStatementCreate(type_0: StmtType, name: &str) -> Box<UnknownStatement> {
    Box::new(UnknownStatement {
        stmt_type: type_0,
        name: name.to_string(),
    })
}

pub(crate) fn IncludeCreate(
    _ctx: &mut XkbContext,
    stmt_str: &str,
    mut merge: MergeMode,
) -> Option<Box<IncludeStmt>> {
    let mut items: Vec<Box<IncludeStmt>> = Vec::new();
    let mut remaining: Option<&str> = Some(stmt_str);

    loop {
        let input = match remaining {
            Some(s) if !s.is_empty() => s,
            _ => break,
        };

        let (parsed, rest) = match ParseIncludeMap(input) {
            Some(r) => r,
            None => {
                log::error!(
                    "[XKB-{:03}] Illegal include statement \"{}\"; Ignored\n",
                    XKB_ERROR_INVALID_INCLUDE_STATEMENT as i32,
                    stmt_str
                );
                return None;
            }
        };

        if parsed.file.is_empty() {
            remaining = rest;
            continue;
        }

        items.push(Box::new(IncludeStmt {
            merge,
            stmt: String::new(),
            file: parsed.file,
            map: parsed.map,
            modifier: parsed.extra_data,
            next_incl: None,
        }));

        match parsed.nextop {
            '|' => merge = MERGE_AUGMENT,
            '^' => merge = MERGE_REPLACE,
            _ => merge = MERGE_OVERRIDE,
        }

        remaining = rest;
    }

    // Build linked list in reverse
    let mut first: Option<Box<IncludeStmt>> = None;
    for mut item in items.into_iter().rev() {
        item.next_incl = first;
        first = Some(item);
    }

    if let Some(ref mut f) = first {
        f.stmt = stmt_str.to_string();
    }
    first
}

pub(crate) fn XkbFileCreate(
    type_0: u32,
    name: Option<String>,
    defs: Vec<Statement>,
    flags: XkbMapFlags,
) -> Box<XkbFile> {
    let mut name_str = name.unwrap_or_default();
    xkb_escape_map_name(&mut name_str);
    Box::new(XkbFile {
        file_type: type_0,
        name: name_str,
        defs,
        flags,
    })
}

pub(crate) fn XkbFileFromComponents(
    ctx: &mut XkbContext,
    kkctgs: &XkbComponentNames,
) -> Option<Box<XkbFile>> {
    let components = [
        &kkctgs.keycodes,
        &kkctgs.types,
        &kkctgs.compatibility,
        &kkctgs.symbols,
    ];
    let mut file_stmts: Vec<Statement> = Vec::new();
    let mut type_0 = FIRST_KEYMAP_FILE_TYPE;
    while type_0 <= LAST_KEYMAP_FILE_TYPE {
        let component_bytes: Vec<u8> = components[type_0 as usize]
            .iter()
            .map(|&b| b as u8)
            .collect();
        let end = component_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(component_bytes.len());
        let component_str = std::str::from_utf8(&component_bytes[..end]).unwrap_or("");
        let include = IncludeCreate(ctx, component_str, MERGE_DEFAULT)?;
        let defs = vec![Statement::Include(include)];
        let file = XkbFileCreate(type_0, None, defs, 0);
        file_stmts.push(Statement::XkbFile(file));
        type_0 += 1;
    }
    Some(XkbFileCreate(FILE_TYPE_KEYMAP, None, file_stmts, 0))
}

static XKB_FILE_TYPE_STRINGS: [&str; 7] = [
    "xkb_keycodes",
    "xkb_types",
    "xkb_compatibility",
    "xkb_symbols",
    "xkb_geometry",
    "xkb_keymap",
    "rules",
];

pub(crate) fn xkb_file_type_to_string(type_0: u32) -> &'static str {
    if type_0 as usize >= XKB_FILE_TYPE_STRINGS.len() {
        "unknown"
    } else {
        XKB_FILE_TYPE_STRINGS[type_0 as usize]
    }
}
static STMT_TYPE_STRINGS: [&str; 37] = [
    "unknown statement",
    "include statement",
    "key name definition",
    "key alias definition",
    "string literal expression",
    "integer literal expression",
    "float literal expression",
    "boolean literal expression",
    "key name expression",
    "keysym expression",
    "identifier expression",
    "action declaration expression",
    "field reference expression",
    "array reference expression",
    "empty list expression",
    "keysym list expression",
    "action list expression",
    "addition expression",
    "substraction expression",
    "multiplication expression",
    "division expression",
    "assignment expression",
    "logical negation expression",
    "arithmetic negation expression",
    "bitwise inversion expression",
    "unary plus expression",
    "variable definition",
    "key type definition",
    "symbol interpretation definition",
    "virtual modifiers definition",
    "key symbols definition",
    "modifier map declaration",
    "group declaration",
    "indicator map declaration",
    "indicator name declaration",
    "unknown declaration statement",
    "unknown compound statement",
];

pub(crate) fn stmt_type_to_string(type_0: u32) -> &'static str {
    if type_0 >= _STMT_NUM_VALUES {
        return "unknown";
    }
    STMT_TYPE_STRINGS[type_0 as usize]
}

pub(crate) fn stmt_type_to_operator_char(type_0: u32) -> char {
    match type_0 {
        17 => '+',
        18 => '-',
        19 => '*',
        20 => '/',
        22 => '!',
        23 => '-',
        24 => '~',
        25 => '+',
        _ => '\0',
    }
}
// ── Scanner types (migrated from scanner_utils.rs) ──

#[derive(Copy, Clone, Default)]
pub(crate) struct sval<'a> {
    pub(crate) data: &'a [u8],
}

impl<'a> sval<'a> {
    pub(crate) const EMPTY: sval<'static> = sval { data: &[] };

    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        std::str::from_utf8(self.data).unwrap_or("")
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub(crate) fn start_ptr(&self) -> *const i8 {
        self.data.as_ptr() as *const i8
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Copy, Clone)]
pub(crate) struct scanner_loc {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

pub(crate) struct scanner<'a> {
    pub(crate) pos: usize,
    pub(crate) s: &'a [u8],
    pub(crate) buf: [u8; 1024],
    pub(crate) buf_pos: usize,
    pub(crate) token_pos: usize,
    pub(crate) cached_pos: usize,
    pub(crate) cached_loc: scanner_loc,
    pub(crate) file_name: String,
}

impl<'a> scanner<'a> {
    pub(crate) fn new(s: &'a [u8], file_name: &str) -> Self {
        scanner {
            pos: 0,
            s,
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: scanner_loc { line: 1, column: 1 },
            file_name: file_name.to_string(),
        }
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.s.len()
    }

    #[inline]
    fn remaining_bytes(&self) -> &[u8] {
        &self.s[self.pos..]
    }

    #[inline]
    pub(crate) fn peek(&self) -> i8 {
        if self.pos >= self.s.len() {
            return 0;
        }
        self.s[self.pos] as i8
    }

    #[inline]
    pub(crate) fn eof(&self) -> bool {
        self.pos >= self.s.len()
    }

    #[inline]
    pub(crate) fn eol(&self) -> bool {
        self.peek() == b'\n' as i8
    }

    #[inline]
    pub(crate) fn skip_to_eol(&mut self) {
        let rem = self.remaining_bytes();
        match rem.iter().position(|&b| b == b'\n') {
            Some(i) => self.pos += i,
            None => self.pos = self.s.len(),
        }
    }

    #[inline]
    pub(crate) fn next_byte(&mut self) -> i8 {
        if self.pos >= self.s.len() {
            return 0;
        }
        let c = self.s[self.pos] as i8;
        self.pos += 1;
        c
    }

    #[inline]
    pub(crate) fn chr(&mut self, ch: i8) -> bool {
        if self.peek() != ch {
            return false;
        }
        self.pos += 1;
        true
    }

    #[inline]
    pub(crate) fn str_match(&mut self, string: &[u8]) -> bool {
        let len = string.len();
        if self.s.len() - self.pos < len {
            return false;
        }
        if &self.s[self.pos..self.pos + len] != string {
            return false;
        }
        self.pos += len;
        true
    }

    #[inline]
    pub(crate) fn buf_append(&mut self, ch: u8) -> bool {
        if self.buf_pos + 1 >= self.buf.len() {
            return false;
        }
        self.buf[self.buf_pos] = ch;
        self.buf_pos += 1;
        true
    }

    #[inline]
    pub(crate) fn buf_appends(&mut self, s: &[u8]) -> bool {
        for &b in s {
            if b == 0 {
                break;
            }
            if !self.buf_append(b) {
                return false;
            }
        }
        true
    }

    pub(crate) fn buf_appends_str(&mut self, s: &str) -> bool {
        for &b in s.as_bytes() {
            if !self.buf_append(b) {
                return false;
            }
        }
        true
    }

    #[inline]
    pub(crate) fn buf_appends_code_point(&mut self, c: u32) -> bool {
        if self.buf_pos + 4 <= self.buf.len() {
            let mut count = utf32_to_utf8_safe(c, &mut self.buf[self.buf_pos..]);
            if count == 0 {
                count = utf32_to_utf8_safe(0xfffd, &mut self.buf[self.buf_pos..]);
            }
            if count == 0 {
                return false;
            }
            self.buf_pos += count;
            true
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn oct(&mut self, out: &mut u8) -> bool {
        let mut i: u8 = 0;
        let mut c: u8 = 0;
        while self.peek() as u8 >= b'0' && self.peek() as u8 <= b'7' && (i as i32) < 4 {
            if (c as i32) < 0o40 {
                c = (c as i32 * 8 + self.next_byte() as i32 - b'0' as i32) as u8;
            } else {
                self.next_byte();
                *out = c;
                return false;
            }
            i += 1;
        }
        *out = c;
        i > 0
    }

    #[inline]
    pub(crate) fn dec_int64(&mut self, out: &mut i64) -> i32 {
        let remaining = self.remaining_bytes();
        let (val, count) = parse_dec_u64(remaining);
        if count > 0 {
            if val > i64::MAX as u64 {
                return -1;
            }
            self.pos += count as usize;
            *out = val as i64;
        }
        count
    }

    #[inline]
    pub(crate) fn hex_int64(&mut self, out: &mut i64) -> i32 {
        let remaining = self.remaining_bytes();
        let (val, count) = parse_hex_u64(remaining);
        if count > 0 {
            if val > i64::MAX as u64 {
                return -1;
            }
            self.pos += count as usize;
            *out = val as i64;
        }
        count
    }

    #[inline]
    pub(crate) fn unicode_code_point(&mut self, out: &mut u32) -> bool {
        if !self.chr(b'{' as i8) {
            return false;
        }
        let remaining = self.remaining_bytes();
        let (cp, count) = parse_hex_u32(remaining);
        if count > 0 {
            self.pos += count as usize;
        }
        let last_valid = self.pos;
        while !self.eof() && !self.eol() && self.peek() != b'"' as i8 && self.peek() != b'}' as i8 {
            self.next_byte();
        }
        if self.chr(b'}' as i8) {
            *out = cp;
            return count > 0 && self.pos == last_valid + 1 && cp <= 0x10ffff;
        }
        self.pos = last_valid;
        false
    }

    #[inline]
    pub(crate) fn check_supported_char_encoding(&mut self) -> bool {
        use super::super::shared_types::XKB_ERROR_INVALID_FILE_ENCODING;
        if self.str_match(b"\xEF\xBB\xBF") || self.s.len() < 2 {
            return true;
        }
        let input = self.s;
        if input[0] == 0 || input[1] == 0 {
            let loc = self.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: unexpected NULL character.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &self.file_name,
                loc.line,
                loc.column
            );
            return false;
        }
        if !input[0].is_ascii() {
            let loc = self.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: unexpected non-ASCII character.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &self.file_name,
                loc.line,
                loc.column
            );
            return false;
        }
        true
    }

    #[inline]
    pub(crate) fn input_at(&self, pos: usize) -> *const i8 {
        self.s[pos..].as_ptr() as *const i8
    }

    #[inline]
    pub(crate) fn input_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.s[start..end]
    }

    pub(crate) fn token_location(&mut self) -> scanner_loc {
        let mut line = self.cached_loc.line;
        let mut line_pos: usize = 0;

        if self.cached_pos > self.token_pos {
            self.cached_pos = 0;
            self.cached_loc.column = 1;
            self.cached_loc.line = 1;
        }

        let input = self.s;
        let start = self.cached_pos;
        let end = self.token_pos;

        let mut search_from = start;
        while let Some(i) = input[search_from..end].iter().position(|&b| b == b'\n') {
            line += 1;
            search_from = search_from + i + 1;
            line_pos = search_from;
        }

        let column = if line == self.cached_loc.line {
            self.cached_loc.column + (self.token_pos - self.cached_pos)
        } else {
            self.token_pos - line_pos + 1
        };

        let loc = scanner_loc { line, column };
        self.cached_pos = self.token_pos;
        self.cached_loc = loc;
        loc
    }
}

// ── sval comparison functions (migrated from scanner_utils.rs) ──

#[inline]
pub(crate) fn svaleq(s1: sval, s2: sval) -> bool {
    s1.data == s2.data
}

#[inline]
pub(crate) fn svaleq_prefix(s1: sval, s2: sval) -> bool {
    s1.data.len() <= s2.data.len() && s1.data == &s2.data[..s1.data.len()]
}

#[inline]
pub(crate) fn isvaleq(s1: sval, s2: sval) -> bool {
    s1.data.len() == s2.data.len() && s1.data.eq_ignore_ascii_case(s2.data)
}

pub(crate) const ALTERNATE_GROUP: i32 = 77;
pub(crate) const FUNCTION_KEYS: i32 = 76;
pub(crate) const KEYPAD_KEYS: i32 = 75;
pub(crate) const MODIFIER_KEYS: i32 = 74;
pub(crate) const ALPHANUMERIC_KEYS: i32 = 73;
pub(crate) const HIDDEN: i32 = 72;
pub(crate) const DEFAULT: i32 = 71;
pub(crate) const PARTIAL: i32 = 70;
pub(crate) const KEYNAME: i32 = 65;
pub(crate) const IDENT: i32 = 64;
pub(crate) const FLOAT: i32 = 63;
pub(crate) const INTEGER: i32 = 62;
pub(crate) const DECIMAL_DIGIT: i32 = 61;
pub(crate) const STRING: i32 = 60;
pub(crate) const INVERT: i32 = 55;
pub(crate) const EXCLAM: i32 = 54;
pub(crate) const SEMI: i32 = 53;
pub(crate) const COMMA: i32 = 52;
pub(crate) const DOT: i32 = 51;
pub(crate) const CBRACKET: i32 = 50;
pub(crate) const OBRACKET: i32 = 49;
pub(crate) const CPAREN: i32 = 48;
pub(crate) const OPAREN: i32 = 47;
pub(crate) const CBRACE: i32 = 46;
pub(crate) const OBRACE: i32 = 45;
pub(crate) const TIMES: i32 = 44;
pub(crate) const DIVIDE: i32 = 43;
pub(crate) const MINUS: i32 = 42;
pub(crate) const PLUS: i32 = 41;
pub(crate) const EQUALS: i32 = 40;
pub(crate) const VIRTUAL: i32 = 38;
pub(crate) const LOGO: i32 = 37;
pub(crate) const SOLID: i32 = 36;
pub(crate) const OUTLINE: i32 = 35;
pub(crate) const TEXT: i32 = 34;
pub(crate) const OVERLAY: i32 = 33;
pub(crate) const SECTION: i32 = 32;
pub(crate) const ROW: i32 = 31;
pub(crate) const KEYS: i32 = 30;
pub(crate) const SHAPE: i32 = 29;
pub(crate) const INDICATOR: i32 = 28;
pub(crate) const MODIFIER_MAP: i32 = 27;
pub(crate) const GROUP: i32 = 26;
pub(crate) const ALIAS: i32 = 25;
pub(crate) const KEY: i32 = 24;
pub(crate) const ACTION_TOK: i32 = 23;
pub(crate) const INTERPRET: i32 = 22;
pub(crate) const TYPE: i32 = 21;
pub(crate) const VIRTUAL_MODS: i32 = 20;
pub(crate) const ALTERNATE: i32 = 14;
pub(crate) const REPLACE: i32 = 13;
pub(crate) const AUGMENT: i32 = 12;
pub(crate) const OVERRIDE: i32 = 11;
pub(crate) const INCLUDE: i32 = 10;
pub(crate) const XKB_LAYOUT: i32 = 8;
pub(crate) const XKB_SEMANTICS: i32 = 7;
pub(crate) const XKB_GEOMETRY: i32 = 6;
pub(crate) const XKB_COMPATMAP: i32 = 5;
pub(crate) const XKB_SYMBOLS: i32 = 4;
pub(crate) const XKB_TYPES: i32 = 3;
pub(crate) const XKB_KEYCODES: i32 = 2;
pub(crate) const XKB_KEYMAP: i32 = 1;
pub(crate) const ERROR_TOK: i32 = 255;
pub(crate) const YYUNDEF: i32 = 257;
pub(crate) const YYerror: i32 = 256;
pub(crate) const END_OF_FILE: i32 = 0;
pub(crate) const YYEMPTY: i32 = -2;

/// Native Rust UTF-32 to UTF-8 conversion (replaces C FFI)
///
/// Encode a Unicode code point to UTF-8 into the given buffer.
/// Returns the number of bytes written (NOT including null terminator), or 0 on failure.
#[inline]
pub(crate) fn utf32_to_utf8_safe(unichar: u32, buffer: &mut [u8]) -> usize {
    let Some(ch) = char::from_u32(unichar) else {
        return 0;
    };
    let encoded = ch.encode_utf8(&mut buffer[..]);
    encoded.len()
}
// ── Keyword lookup (gperf-generated) ──

pub(crate) const MAX_HASH_VALUE: u32 = 72;
pub(crate) const MIN_WORD_LENGTH: u32 = 3;
pub(crate) const MAX_WORD_LENGTH: u32 = 21;
pub(crate) const MIN_HASH_VALUE: u32 = 3;
pub(crate) const TOTAL_KEYWORDS: u32 = 45;

static GPERF_DOWNCASE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];

static ASSO_VALUES: [u8; 256] = [
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 0, 73, 5, 36, 0, 10, 1, 15,
    15, 73, 0, 10, 20, 35, 20, 50, 73, 10, 10, 5, 0, 15, 73, 0, 15, 73, 73, 73, 73, 73, 73, 73, 0,
    73, 5, 36, 0, 10, 1, 15, 15, 73, 0, 10, 20, 35, 20, 50, 73, 10, 10, 5, 0, 15, 73, 0, 15, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
    73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
];

/// Keyword string + token pairs, indexed by gperf hash value.
static WORDLIST: [Option<(&[u8], i32)>; 73] = [
    None,                                            // 0
    None,                                            // 1
    None,                                            // 2
    Some((b"key", KEY)),                             // 3
    Some((b"keys", KEYS)),                           // 4
    None,                                            // 5
    None,                                            // 6
    Some((b"augment", AUGMENT)),                     // 7
    None,                                            // 8
    Some((b"text", TEXT)),                           // 9
    Some((b"xkb_keymap", XKB_KEYMAP)),               // 10
    Some((b"keypad_keys", KEYPAD_KEYS)),             // 11
    Some((b"xkb_keycodes", XKB_KEYCODES)),           // 12
    Some((b"xkb_geometry", XKB_GEOMETRY)),           // 13
    Some((b"xkb_types", XKB_TYPES)),                 // 14
    Some((b"xkb_compat", XKB_COMPATMAP)),            // 15
    None,                                            // 16
    Some((b"replace", REPLACE)),                     // 17
    None,                                            // 18
    Some((b"xkb_compat_map", XKB_COMPATMAP)),        // 19
    Some((b"xkb_layout", XKB_LAYOUT)),               // 20
    Some((b"xkb_symbols", XKB_SYMBOLS)),             // 21
    Some((b"xkb_compatibility", XKB_COMPATMAP)),     // 22
    Some((b"xkb_semantics", XKB_SEMANTICS)),         // 23
    Some((b"type", TYPE)),                           // 24
    Some((b"alias", ALIAS)),                         // 25
    Some((b"xkb_compatibility_map", XKB_COMPATMAP)), // 26
    Some((b"alphanumeric_keys", ALPHANUMERIC_KEYS)), // 27
    Some((b"function_keys", FUNCTION_KEYS)),         // 28
    Some((b"alternate", ALTERNATE)),                 // 29
    Some((b"shape", SHAPE)),                         // 30
    Some((b"action", ACTION_TOK)),                   // 31
    Some((b"section", SECTION)),                     // 32
    Some((b"row", ROW)),                             // 33
    Some((b"logo", LOGO)),                           // 34
    Some((b"alternate_group", ALTERNATE_GROUP)),     // 35
    Some((b"hidden", HIDDEN)),                       // 36
    Some((b"virtual", VIRTUAL)),                     // 37
    None,                                            // 38
    None,                                            // 39
    None,                                            // 40
    None,                                            // 41
    Some((b"outline", OUTLINE)),                     // 42
    Some((b"default", DEFAULT)),                     // 43
    None,                                            // 44
    None,                                            // 45
    Some((b"modmap", MODIFIER_MAP)),                 // 46
    Some((b"virtual_modifiers", VIRTUAL_MODS)),      // 47
    None,                                            // 48
    None,                                            // 49
    None,                                            // 50
    None,                                            // 51
    Some((b"overlay", OVERLAY)),                     // 52
    Some((b"override", OVERRIDE)),                   // 53
    None,                                            // 54
    None,                                            // 55
    None,                                            // 56
    Some((b"include", INCLUDE)),                     // 57
    None,                                            // 58
    None,                                            // 59
    None,                                            // 60
    None,                                            // 61
    Some((b"modifier_map", MODIFIER_MAP)),           // 62
    Some((b"modifier_keys", MODIFIER_KEYS)),         // 63
    Some((b"indicator", INDICATOR)),                 // 64
    None,                                            // 65
    Some((b"group", GROUP)),                         // 66
    Some((b"mod_map", MODIFIER_MAP)),                // 67
    None,                                            // 68
    Some((b"interpret", INTERPRET)),                 // 69
    None,                                            // 70
    Some((b"solid", SOLID)),                         // 71
    Some((b"partial", PARTIAL)),                     // 72
];

fn gperf_case_eq(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    for i in 0..s1.len() {
        if GPERF_DOWNCASE[s1[i] as usize] != GPERF_DOWNCASE[s2[i] as usize] {
            return false;
        }
    }
    true
}

fn keyword_gperf_hash(s: &[u8]) -> u32 {
    let mut hval: u32 = s.len() as u32;
    if s.len() >= 5 {
        hval = hval.wrapping_add(ASSO_VALUES[s[4] as usize] as u32);
    }
    if s.len() >= 2 {
        hval = hval.wrapping_add(ASSO_VALUES[s[1] as usize] as u32);
    }
    hval = hval.wrapping_add(ASSO_VALUES[s[0] as usize] as u32);
    hval
}

/// Look up a keyword token from a byte slice. Returns -1 if not found.
pub(crate) fn keyword_to_token(string: &[u8]) -> i32 {
    let len = string.len();
    if len > MAX_WORD_LENGTH as usize || len < MIN_WORD_LENGTH as usize {
        return -1;
    }
    let key = keyword_gperf_hash(string);
    if key > MAX_HASH_VALUE {
        return -1;
    }
    if let Some((kw, tok)) = WORDLIST[key as usize] {
        if len == kw.len() && gperf_case_eq(string, kw) {
            return tok;
        }
    }
    -1
}

// ── YYValue: safe replacement for the YYSTYPE union ──

/// Safe parser value stack type, replacing the old YYSTYPE union.
/// Each variant owns its data. `Default` produces `None`.
#[derive(Default)]
pub(crate) enum YYValue<'a> {
    #[default]
    None,
    Num(i64),
    FileType(u32),
    Str(String),
    Sval(sval<'a>),
    Atom(u32),
    Merge(MergeMode),
    MapFlags(XkbMapFlags),
    Keysym(u32),
    NoSymbolOrActionList(u32),
    Expr(Box<ExprDef>),
    ExprList(Vec<Box<ExprDef>>),
    Var(Box<VarDef>),
    VarList(Vec<Box<VarDef>>),
    VMod(Box<VModDef>),
    VModList(Vec<Box<VModDef>>),
    Interp(Box<InterpDef>),
    KeyType(Box<KeyTypeDef>),
    Symbols(Box<SymbolsDef>),
    ModMask(Box<ModMapDef>),
    GroupCompat(Box<GroupCompatDef>),
    LedMap(Box<LedMapDef>),
    LedName(Box<LedNameDef>),
    Keycode(Box<KeycodeDef>),
    KeyAlias(Box<KeyAliasDef>),
    Unknown(Box<UnknownStatement>),
    File(Box<XkbFile>),
    FileList(Vec<Box<XkbFile>>),
    Stmt(Statement),
    StmtList(Vec<Statement>),
}

// Helper to take a value out and replace with None
impl<'a> YYValue<'a> {
    pub(crate) fn take(&mut self) -> YYValue<'a> {
        std::mem::take(self)
    }

    pub(crate) fn take_expr(&mut self) -> Option<Box<ExprDef>> {
        match std::mem::take(self) {
            YYValue::Expr(e) => Some(e),
            _ => Option::None,
        }
    }
    pub(crate) fn take_expr_list(&mut self) -> Vec<Box<ExprDef>> {
        match std::mem::take(self) {
            YYValue::ExprList(v) => v,
            _ => Vec::new(),
        }
    }
    pub(crate) fn take_var(&mut self) -> Option<Box<VarDef>> {
        match std::mem::take(self) {
            YYValue::Var(v) => Some(v),
            _ => Option::None,
        }
    }
    pub(crate) fn take_var_list(&mut self) -> Vec<Box<VarDef>> {
        match std::mem::take(self) {
            YYValue::VarList(v) => v,
            _ => Vec::new(),
        }
    }
    pub(crate) fn take_vmod(&mut self) -> Option<Box<VModDef>> {
        match std::mem::take(self) {
            YYValue::VMod(v) => Some(v),
            _ => Option::None,
        }
    }
    pub(crate) fn take_vmod_list(&mut self) -> Vec<Box<VModDef>> {
        match std::mem::take(self) {
            YYValue::VModList(v) => v,
            _ => Vec::new(),
        }
    }
    pub(crate) fn take_file(&mut self) -> Option<Box<XkbFile>> {
        match std::mem::take(self) {
            YYValue::File(f) => Some(f),
            _ => Option::None,
        }
    }
    pub(crate) fn take_file_list(&mut self) -> Vec<Box<XkbFile>> {
        match std::mem::take(self) {
            YYValue::FileList(v) => v,
            _ => Vec::new(),
        }
    }
    pub(crate) fn take_stmt_list(&mut self) -> Vec<Statement> {
        match std::mem::take(self) {
            YYValue::StmtList(v) => v,
            _ => Vec::new(),
        }
    }
    pub(crate) fn as_num(&self) -> i64 {
        match self {
            YYValue::Num(n) => *n,
            _ => 0,
        }
    }
    pub(crate) fn as_atom(&self) -> u32 {
        match self {
            YYValue::Atom(a) => *a,
            _ => 0,
        }
    }
    pub(crate) fn as_merge(&self) -> MergeMode {
        match self {
            YYValue::Merge(m) => *m,
            _ => MERGE_DEFAULT,
        }
    }
    pub(crate) fn as_map_flags(&self) -> XkbMapFlags {
        match self {
            YYValue::MapFlags(f) => *f,
            _ => 0,
        }
    }
    pub(crate) fn as_file_type(&self) -> u32 {
        match self {
            YYValue::FileType(f) => *f,
            _ => 0,
        }
    }
    pub(crate) fn as_keysym(&self) -> u32 {
        match self {
            YYValue::Keysym(k) => *k,
            _ => 0,
        }
    }
    pub(crate) fn as_no_sym_or_action_list(&self) -> u32 {
        match self {
            YYValue::NoSymbolOrActionList(n) => *n,
            _ => 0,
        }
    }
    pub(crate) fn as_sval(&self) -> sval<'a> {
        match self {
            YYValue::Sval(s) => *s,
            _ => sval::EMPTY,
        }
    }
    pub(crate) fn take_str(&mut self) -> String {
        match std::mem::take(self) {
            YYValue::Str(s) => s,
            _ => String::new(),
        }
    }
}

/// Check if byte is whitespace (space, HT, LF, VT, FF, CR).
/// Matches C `isspace()` for ASCII range.
#[inline]
fn is_space(ch: i8) -> bool {
    matches!(ch as u8, b' ' | b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r')
}
pub(crate) static DECIMAL_SEPARATOR: i8 = '.' as i32 as i8;
fn number(s: &mut scanner, out: &mut i64, out_tok: &mut i32) -> bool {
    if s.str_match(b"0x") {
        match s.hex_int64(out) {
            -1 => {
                *out_tok = ERROR_TOK;
                true
            }
            0 => false,
            _ => {
                *out_tok = INTEGER;
                true
            }
        }
    } else {
        let mut is_digit_0: bool = false;
        match s.dec_int64(out) {
            -1 => {
                *out_tok = ERROR_TOK;
                return true;
            }
            0 => return false,
            1 => {
                is_digit_0 = true;
            }
            _ => {}
        }
        if s.chr(DECIMAL_SEPARATOR) {
            let mut dec: i64 = 0;
            if s.dec_int64(&mut dec) < 0 {
                *out_tok = ERROR_TOK;
                return true;
            }
            *out_tok = FLOAT;
        } else if is_digit_0 {
            *out_tok = DECIMAL_DIGIT;
        } else {
            *out_tok = INTEGER;
        }
        true
    }
}

/// Lex one token and write the semantic value into `yylval`.
pub(crate) fn _xkbcommon_lex<'a>(
    yylval: &mut YYValue<'a>,
    s: &mut scanner<'a>,
    ctx: &mut XkbContext,
) -> i32 {
    loop {
        while is_space(s.peek()) {
            s.next_byte();
        }
        if s.str_match(b"\xE2\x80\x8E") || s.str_match(b"\xE2\x80\x8F") {
            continue;
        }
        if !(s.str_match(b"//") || s.chr(b'#' as i8)) {
            break;
        }
        s.skip_to_eol();
    }
    if s.eof() {
        return END_OF_FILE;
    }
    s.token_pos = s.pos;
    s.buf_pos = 0;
    if s.chr(b'"' as i8) {
        while !s.eof() && !s.eol() && s.peek() != b'"' as i8 {
            if s.chr(b'\\' as i8) {
                let mut o: u8 = 0;
                let start_pos: usize = s.pos;
                if s.chr(b'\\' as i8) {
                    s.buf_append(b'\\');
                } else if s.chr(b'"' as i8) {
                    s.buf_append(b'"');
                } else if s.chr(b'n' as i8) {
                    s.buf_append(b'\n');
                } else if s.chr(b't' as i8) {
                    s.buf_append(b'\t');
                } else if s.chr(b'r' as i8) {
                    s.buf_append(b'\r');
                } else if s.chr(b'b' as i8) {
                    s.buf_append(b'\x08');
                } else if s.chr(b'f' as i8) {
                    s.buf_append(b'\x0c');
                } else if s.chr(b'v' as i8) {
                    s.buf_append(b'\x0b');
                } else if s.chr(b'e' as i8) {
                    s.buf_append(b'\x1b');
                } else if s.chr(b'u' as i8) {
                    let mut cp: u32 = 0;
                    if s.unicode_code_point(&mut cp) && cp != 0 {
                        s.buf_appends_code_point(cp);
                    } else {
                        let loc = s.token_location();
                        log::warn!("[XKB-{:03}] {}:{}:{}: invalid Unicode escape sequence \"{}\" in string literal\n",
                            XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE
                                as i32,
                            &s.file_name,
                            loc.line,
                            loc.column,
                            std::str::from_utf8(s.input_slice(start_pos.wrapping_sub(1), s.pos)).unwrap_or(""));
                    }
                } else if s.oct(&mut o) && o != 0 {
                    s.buf_append(o);
                } else if s.pos > start_pos {
                    let loc_0 = s.token_location();
                    log::warn!("[XKB-{:03}] {}:{}:{}: invalid octal escape sequence \"{}\" in string literal\n",
                        XKB_WARNING_INVALID_ESCAPE_SEQUENCE as i32,
                        &s.file_name,
                        loc_0.line,
                        loc_0.column,
                        std::str::from_utf8(s.input_slice(start_pos.wrapping_sub(1), s.pos)).unwrap_or(""));
                } else {
                    let loc_1 = s.token_location();
                    log::warn!("[XKB-{:03}] {}:{}:{}: unknown escape sequence \"\\{}\" in string literal\n",
                        XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE
                            as i32,
                        &s.file_name,
                        loc_1.line,
                        loc_1.column,
                        (s.peek() as u8 as char));
                }
            } else {
                let c = s.next_byte();
                s.buf_append(c as u8);
            }
        }
        if !s.buf_append(0) || !s.chr(b'"' as i8) {
            let loc_2 = s.token_location();
            log::error!(
                "{}:{}:{}: unterminated string literal\n",
                &s.file_name,
                loc_2.line,
                loc_2.column
            );
            return ERROR_TOK;
        }
        // Convert buffer to String (exclude null terminator)
        let buf_len = s.buf_pos.saturating_sub(1);
        let string = String::from_utf8_lossy(&s.buf[..buf_len]).into_owned();
        *yylval = YYValue::Str(string);
        return STRING;
    }
    if s.chr(b'<' as i8) {
        while (s.peek() as u8).is_ascii_graphic() && s.peek() != b'>' as i8 {
            s.next_byte();
        }
        if !s.chr(b'>' as i8) {
            let loc_3 = s.token_location();
            log::error!(
                "{}:{}:{}: unterminated key name literal\n",
                &s.file_name,
                loc_3.line,
                loc_3.column
            );
            return ERROR_TOK;
        }
        let len: usize = s.pos - s.token_pos - 2;
        let keyname_bytes: Vec<u8> = s
            .input_slice(s.token_pos + 1, s.token_pos + 1 + len)
            .to_vec();
        *yylval = YYValue::Atom(atom_intern(&mut ctx.atom_table, &keyname_bytes, true));
        return KEYNAME;
    }
    if s.chr(b';' as i8) {
        return SEMI;
    }
    if s.chr(b'{' as i8) {
        return OBRACE;
    }
    if s.chr(b'}' as i8) {
        return CBRACE;
    }
    if s.chr(b'=' as i8) {
        return EQUALS;
    }
    if s.chr(b'[' as i8) {
        return OBRACKET;
    }
    if s.chr(b']' as i8) {
        return CBRACKET;
    }
    if s.chr(b'(' as i8) {
        return OPAREN;
    }
    if s.chr(b')' as i8) {
        return CPAREN;
    }
    if s.chr(b'.' as i8) {
        return DOT;
    }
    if s.chr(b',' as i8) {
        return COMMA;
    }
    if s.chr(b'+' as i8) {
        return PLUS;
    }
    if s.chr(b'-' as i8) {
        return MINUS;
    }
    if s.chr(b'*' as i8) {
        return TIMES;
    }
    if s.chr(b'/' as i8) {
        return DIVIDE;
    }
    if s.chr(b'!' as i8) {
        return EXCLAM;
    }
    if s.chr(b'~' as i8) {
        return INVERT;
    }
    let mut tok: i32 = ERROR_TOK;
    if (s.peek() as u8).is_ascii_alphabetic() || s.peek() == b'_' as i8 {
        while (s.peek() as u8).is_ascii_alphanumeric() || s.peek() == b'_' as i8 {
            s.next_byte();
        }
        tok = keyword_to_token(s.input_slice(s.token_pos, s.pos));
        if tok >= 0 {
            return tok;
        }
        *yylval = YYValue::Sval(sval {
            data: &s.s[s.token_pos..s.pos],
        });
        return IDENT;
    }
    let mut num_val: i64 = 0;
    if number(s, &mut num_val, &mut tok) {
        *yylval = YYValue::Num(num_val);
        if tok == ERROR_TOK {
            let loc_4 = s.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: malformed number literal\n",
                XKB_ERROR_MALFORMED_NUMBER_LITERAL as i32,
                &s.file_name,
                loc_4.line,
                loc_4.column
            );
            return ERROR_TOK;
        }
        return tok;
    }
    let loc_5 = s.token_location();
    log::error!(
        "{}:{}:{}: unrecognized token\n",
        &s.file_name,
        loc_5.line,
        loc_5.column
    );
    ERROR_TOK
}
pub(crate) fn XkbParseStringInit<'a>(
    sc: &mut scanner<'a>,
    input: &'a [u8],
    file_name: &str,
    _map: &str,
) -> bool {
    *sc = scanner::new(input, file_name);
    if !sc.check_supported_char_encoding() {
        let loc = sc.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &sc.file_name,
            loc.line,
            loc.column);
        let loc_0 = sc.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &sc.file_name,
            loc_0.line,
            loc_0.column);
        return false;
    }
    true
}
pub(crate) fn XkbParseString(
    ctx: &mut XkbContext,
    input: &[u8],
    file_name: &str,
    map: &str,
) -> Option<Box<XkbFile>> {
    let mut sc = scanner::new(&[], "");
    if !XkbParseStringInit(&mut sc, input, file_name, map) {
        return None;
    }
    parse(ctx, &mut sc, map)
}

// ── Include file processing (merged from include.rs) ──

use super::super::keymap::{
    xkb_context_failed_include_path_get, xkb_context_getenv, xkb_context_num_failed_include_paths,
};
use super::super::keymap::{xkb_context_include_path_get, xkb_context_num_include_paths};
use super::super::keymap::{
    xkb_context_include_path_get_extra_path, xkb_context_include_path_get_system_path,
};

pub(crate) const INCLUDE_MAX_DEPTH: i32 = 15_i32;
pub(crate) const MERGE_OVERRIDE_PREFIX: i32 = '+' as i32;
pub(crate) const MERGE_AUGMENT_PREFIX: i32 = '|' as i32;
pub(crate) const MERGE_REPLACE_PREFIX: i32 = '^' as i32;

/// Parsed result from one segment of an include statement.
pub(crate) struct ParsedIncludeMap {
    pub(crate) file: String,
    pub(crate) map: String,
    pub(crate) extra_data: String,
    pub(crate) nextop: char,
}

/// Parse one include map segment from `input`, returning the parsed result
/// and the remaining input (if any). Returns None on parse error.
pub(crate) fn ParseIncludeMap(input: &str) -> Option<(ParsedIncludeMap, Option<&str>)> {
    // Split at merge-mode prefix (+, |, ^)
    let (segment, nextop, rest) = if let Some(pos) = input.find(&['+', '|', '^'][..]) {
        let op = input.as_bytes()[pos] as char;
        (&input[..pos], op, Some(&input[pos + 1..]))
    } else {
        (input, '\0', None)
    };

    // Split off extra_data after ':'
    let (segment, extra_data) = if let Some(pos) = segment.find(':') {
        (&segment[..pos], segment[pos + 1..].to_string())
    } else {
        (segment, String::new())
    };

    // Parse file(map) pattern
    let (file, map) = if let Some(pos) = segment.find('(') {
        if pos == 0 {
            return None; // starts with '(' — invalid
        }
        let rest_paren = &segment[pos + 1..];
        if !rest_paren.ends_with(')') || rest_paren.is_empty() {
            return None;
        }
        let map_str = &rest_paren[..rest_paren.len() - 1];
        (segment[..pos].to_string(), map_str.to_string())
    } else {
        (segment.to_string(), String::new())
    };

    // Validate nextop
    if nextop != '\0' && nextop != '+' && nextop != '|' && nextop != '^' {
        return None;
    }

    Some((
        ParsedIncludeMap {
            file,
            map,
            extra_data,
            nextop,
        },
        rest,
    ))
}
static XKB_FILE_TYPE_INCLUDE_DIRS: [&str; 7] = [
    "keycodes", "types", "compat", "symbols", "geometry", "keymap", "rules",
];
fn DirectoryForInclude(type_0: u32) -> &'static str {
    if type_0 as usize >= XKB_FILE_TYPE_INCLUDE_DIRS.len() {
        ""
    } else {
        XKB_FILE_TYPE_INCLUDE_DIRS[type_0 as usize]
    }
}
fn LogIncludePaths(ctx: &mut XkbContext) {
    let num = xkb_context_num_include_paths(ctx);
    if num > 0_u32 {
        log::error!(
            "[XKB-{:03}] {} include paths searched:\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            num
        );
        for i in 0..num {
            log::error!(
                "[XKB-{:03}] \t{}\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_include_path_get(ctx, i)
            );
        }
    } else {
        log::error!(
            "[XKB-{:03}] There are no include paths to search\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32
        );
    }
    let num_failed = xkb_context_num_failed_include_paths(ctx);
    if num_failed > 0_u32 {
        log::error!(
            "[XKB-{:03}] {} include paths could not be added:\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            num_failed
        );
        for i in 0..num_failed {
            log::error!(
                "[XKB-{:03}] \t{}\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_failed_include_path_get(ctx, i)
            );
        }
    }
}
/// Expand `%H`, `%S`, `%E`, `%%` in the given name string.
/// Returns `Some(expanded)` on success, `None` on error.
fn expand_percent(parent_file_name: &str, type_dir: &str, name: &str) -> Option<String> {
    let max_len = 4096usize;
    let mut result = String::new();
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => result.push('%'),
                Some('H') => match xkb_context_getenv("HOME") {
                    Ok(home) => result.push_str(&home),
                    Err(_) => {
                        log::error!("{}: %H was used in an include statement, but the HOME environment variable is not set\n",
                                parent_file_name);
                        return None;
                    }
                },
                Some('S') => {
                    let sys = xkb_context_include_path_get_system_path();
                    result.push_str(&sys);
                    result.push('/');
                    result.push_str(type_dir);
                }
                Some('E') => {
                    let extra = xkb_context_include_path_get_extra_path();
                    result.push_str(&extra);
                    result.push('/');
                    result.push_str(type_dir);
                }
                Some(other) => {
                    log::error!(
                        "[XKB-{:03}] {}: unknown % format ({}) in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        parent_file_name,
                        other
                    );
                    return None;
                }
                None => {
                    log::error!(
                        "[XKB-{:03}] {}: trailing %% in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        parent_file_name
                    );
                    return None;
                }
            }
        } else {
            result.push(c);
        }
        if result.len() > max_len {
            log::error!(
                "[XKB-{:03}] {}: include path after expansion is too long\n",
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                parent_file_name
            );
            return None;
        }
    }
    Some(result)
}
/// Expand `%`-sequences in `name`. Returns:
/// - `Ok(None)` if no `%` found (no expansion needed)
/// - `Ok(Some(expanded))` if expansion succeeded
/// - `Err(())` on error
pub(crate) fn expand_path_str(
    parent_file_name: &str,
    name: &str,
    file_type: u32,
) -> Result<Option<String>, ()> {
    // Find first '%'
    let k = match name.find('%') {
        Some(pos) => pos,
        None => return Ok(None),
    };
    let type_dir = DirectoryForInclude(file_type);
    let prefix = &name[..k];
    let rest = &name[k..];
    match expand_percent(parent_file_name, type_dir, rest) {
        Some(expanded) => {
            let mut result = String::with_capacity(prefix.len() + expanded.len());
            result.push_str(prefix);
            result.push_str(&expanded);
            Ok(Some(result))
        }
        None => Err(()),
    }
}
pub(crate) fn FindFileInXkbPath(
    ctx: &mut XkbContext,
    _parent_file_name: &str,
    name: &str,
    type_0: u32,
    offset: &mut u32,
    required: bool,
) -> Option<(std::sync::Arc<Vec<u8>>, String)> {
    let type_dir = DirectoryForInclude(type_0);
    let mut i: u32 = *offset;
    while i < xkb_context_num_include_paths(ctx) {
        let path = format!(
            "{}/{}/{}",
            xkb_context_include_path_get(ctx, i),
            type_dir,
            name
        );
        if path.len() >= 4096 {
            log::error!(
                "[XKB-{:03}] Path is too long: expected max length of {}, got: {}\n",
                XKB_ERROR_INVALID_PATH as i32,
                4096,
                &path
            );
        } else if let Some(data) = super::super::shared_types::read_file_cached(&path) {
            *offset = i;
            return Some((data, path));
        }
        i += 1;
    }
    if required && *offset == 0 {
        log::error!(
            "[XKB-{:03}] Couldn't find file \"{}/{}\" in include paths\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            type_dir,
            name
        );
        LogIncludePaths(ctx);
    }
    None
}
pub(crate) fn ExceedsIncludeMaxDepth(include_depth: u32) -> bool {
    if include_depth >= INCLUDE_MAX_DEPTH as u32 {
        log::error!(
            "[XKB-{:03}] Exceeded include depth threshold ({})",
            XKB_ERROR_RECURSIVE_INCLUDE as i32,
            15_i32
        );
        true
    } else {
        false
    }
}
pub(crate) fn ProcessIncludeFile(
    ctx: &mut XkbContext,
    stmt: &IncludeStmt,
    file_type: u32,
) -> Option<Box<XkbFile>> {
    let mut xkb_file: Option<Box<XkbFile>> = None;
    let mut candidate: Option<Box<XkbFile>> = None;

    // Expand %-sequences in the file name
    let stmt_file: String = match expand_path_str("(unknown)", &stmt.file, file_type) {
        Err(()) => return None,
        Ok(Some(expanded)) => expanded,
        Ok(None) => stmt.file.clone(),
    };
    let expanded = stmt_file != stmt.file;

    let map_str = if stmt.map.is_empty() { "" } else { &stmt.map };

    let absolute_path = stmt_file.starts_with('/');
    let mut offset: u32 = 0;
    let mut file_and_path: Option<(std::sync::Arc<Vec<u8>>, String)> = if absolute_path {
        super::super::shared_types::read_file_cached(&stmt_file)
            .map(|data| (data, stmt_file.clone()))
    } else if expanded {
        // Expanded but not absolute — don't search include paths
        None
    } else {
        FindFileInXkbPath(ctx, "(unknown)", &stmt_file, file_type, &mut offset, true)
    };

    while let Some((ref file_data, ref _path)) = file_and_path {
        if let Some(parsed) = XkbParseString(ctx, file_data, &stmt.file, map_str) {
            let _ = file_and_path.take();

            if parsed.file_type != file_type {
                log::error!("[XKB-{:03}] Include file of wrong type (expected {}, got {}); Include file \"{}\" ignored\n",
                    XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                    xkb_file_type_to_string(file_type),
                    xkb_file_type_to_string(parsed.file_type),
                    &stmt.file);
                // parsed drops automatically
            } else if !stmt.map.is_empty() || parsed.flags != 0 && MAP_IS_DEFAULT as i32 != 0 {
                xkb_file = Some(parsed);
                break;
            } else if candidate.is_none() {
                candidate = Some(parsed);
            }
            // else: parsed drops automatically (was FreeXkbFile)
        } else {
            // Drop the file (closes it)
            let _ = file_and_path.take();
        }
        if absolute_path {
            break;
        }
        offset += 1;
        file_and_path =
            FindFileInXkbPath(ctx, "(unknown)", &stmt_file, file_type, &mut offset, true);
    }

    if xkb_file.is_none() {
        xkb_file = candidate;
    }
    // else: candidate drops automatically

    if xkb_file.is_none() {
        if !stmt.map.is_empty() {
            log::error!(
                "[XKB-{:03}] Couldn't process include statement for '{}({})'\n",
                XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                &stmt.file,
                &stmt.map
            );
        } else {
            log::error!(
                "[XKB-{:03}] Couldn't process include statement for '{}'\n",
                XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                &stmt.file
            );
        }
    }
    xkb_file
}

pub(crate) const GROUP_MASK_NAME_LAST: u32 = 3;
pub(crate) const GROUP_INDEX_NAME_LAST: u32 = 1;
pub(crate) type compile_file_fn =
    Option<for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool>;
#[inline]
fn ComputeEffectiveMask(keymap: &XkbKeymap, mods: &mut XkbMods) {
    let unknown_mods: u32 = !((1_u64 << keymap.mods.num_mods).wrapping_sub(1_u64) as u32);
    mods.mask = mod_mask_get_effective(keymap, mods.mods) | mods.mods & unknown_mods;
}
/// Version that takes the mod_set separately to allow calling on fields of keymap.
#[inline]
fn compute_effective_mask_with(mod_set: &XkbModSet, mods: &mut XkbMods) {
    let unknown_mods: u32 = !((1_u64 << mod_set.num_mods).wrapping_sub(1_u64) as u32);
    // Inline mod_mask_get_effective logic
    let mut mask: u32 = mods.mods & MOD_REAL_MASK_ALL;
    let mut i: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
    while i < mod_set.num_mods {
        if mods.mods & (1u32 << i) != 0 {
            mask |= mod_set.mods[i as usize].mapping;
        }
        i += 1;
    }
    mods.mask = mask | mods.mods & unknown_mods;
}
fn UpdateActionMods(keymap: &XkbKeymap, act: &mut XkbAction, modmap: u32) {
    if let 2..=4 = act.action_type() {
        if act.as_mods().flags & ACTION_MODS_LOOKUP_MODMAP != 0 {
            act.as_mods_mut().mods.mods = modmap;
        }
        ComputeEffectiveMask(keymap, &mut act.as_mods_mut().mods);
    };
}
fn default_interpret() -> XkbSymInterpret {
    XkbSymInterpret {
        sym: XKB_KEY_NO_SYMBOL as u32,
        match_0: MATCH_ANY_OR_NONE,
        mods: 0,
        virtual_mod: DEFAULT_INTERPRET_VMOD,
        level_one_only: false,
        repeat: DEFAULT_INTERPRET_KEY_REPEAT as i32 != 0,
        required: false,
        num_actions: 0,
        action: XkbAction::None,
        actions: Vec::new(), // Note: XkbAction is Copy, so Vec::new() is zero-alloc (no heap until push)
    }
}
/// Returns interp indices into `keymap.sym_interprets`, or `usize::MAX` for default interprets.
fn FindInterpForKey(
    keymap: &mut XkbKeymap,
    key_idx: usize,
    group: u32,
    level: u32,
    interp_indices: &mut Vec<usize>,
) -> bool {
    let keycode = keymap.keys[key_idx].keycode;
    let syms_ref = xkb_keymap_key_get_syms_by_level_ref(keymap, keycode, group, level);

    if syms_ref.is_empty() {
        return false;
    }
    // Copy syms to stack to release borrow on keymap (most keys have 1-2 syms)
    let mut syms_buf = [0u32; 8];
    let num_syms = syms_ref.len().min(8);
    syms_buf[..num_syms].copy_from_slice(&syms_ref[..num_syms]);
    let syms = &syms_buf[..num_syms];
    let key_modmap = keymap.keys[key_idx].modmap;
    let key_name = keymap.keys[key_idx].name;
    let num_syms = syms.len() as i32;
    let mut s: i32 = 0_i32;
    while s < num_syms {
        let mut found: bool = false;
        let mut use_default: bool = false;
        let mut i: u32 = 0_u32;
        's_26: loop {
            if i >= keymap.sym_interprets.len() as u32 {
                use_default = true;
                break;
            }
            let interp = &keymap.sym_interprets[i as usize];
            let mods: u32;
            found = false;
            if !(interp.sym != syms[s as usize] && interp.sym != XKB_KEY_NO_SYMBOL as u32) {
                if interp.level_one_only as i32 != 0 && level != 0_u32 {
                    mods = 0_u32;
                } else {
                    mods = key_modmap;
                }
                match interp.match_0 {
                    0 => {
                        found = interp.mods & mods == 0;
                    }
                    1 => {
                        found = mods == 0 || interp.mods & mods != 0;
                    }
                    2 => {
                        found = interp.mods & mods != 0;
                    }
                    3 => {
                        found = interp.mods & mods == interp.mods;
                    }
                    4 => {
                        found = interp.mods == mods;
                    }
                    _ => {}
                }
                if found as i32 != 0
                    && i > 0_u32
                    && interp.sym == XKB_KEY_NO_SYMBOL as u32
                    && !interp_indices.is_empty()
                {
                    for &prev_idx in interp_indices.iter() {
                        if prev_idx == i as usize {
                            found = false;
                            log::warn!("Repeated interpretation ignored for keysym #{} \"{}\" at level {}/group {} on key <{}>.\n",
                                    s + 1_i32,
                                    KeysymText(syms[s as usize]),
                                    level.wrapping_add(1_u32),
                                    group.wrapping_add(1_u32),
                                    atom_text(&keymap.ctx.atom_table, key_name));
                            use_default = true;
                            break 's_26;
                        }
                    }
                }
                if found {
                    interp_indices.push(i as usize);
                    keymap.sym_interprets[i as usize].required = true;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if use_default {
            // usize::MAX signals "use default interpret"
            interp_indices.push(usize::MAX);
        }
        s += 1;
    }
    true
}
fn ApplyInterpsToKey(keymap: &mut XkbKeymap, key_idx: usize) -> bool {
    let mut vmodmap: u32 = 0_u32;
    let mut level: u32;
    let mut interp_indices: Vec<usize> = Vec::with_capacity(4);
    let mut actions: Vec<XkbAction> = Vec::with_capacity(4);
    let num_groups = keymap.keys[key_idx].num_groups;
    let mut group: u32 = 0_u32;
    while group < num_groups {
        if !keymap.keys[key_idx].groups[group as usize].explicit_actions {
            level = 0_u32;
            let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], group);
            while level < num_levels {
                interp_indices.clear();
                let found: bool =
                    FindInterpForKey(keymap, key_idx, group, level, &mut interp_indices);
                if found {
                    let default_interp = default_interpret();
                    let key_explicit = keymap.keys[key_idx].explicit;
                    let key_name = keymap.keys[key_idx].name;
                    for &idx in interp_indices.iter() {
                        let interp = if idx == usize::MAX {
                            &default_interp
                        } else {
                            &keymap.sym_interprets[idx]
                        };
                        if group == 0_u32
                            && level == 0_u32
                            && key_explicit & EXPLICIT_REPEAT == 0
                            && interp.repeat
                        {
                            keymap.keys[key_idx].repeats = true;
                        }
                        if (group == 0_u32 && level == 0_u32 || !interp.level_one_only)
                            && interp.virtual_mod != XKB_MOD_INVALID
                        {
                            vmodmap |= 1_u32 << interp.virtual_mod;
                        }
                        match interp.num_actions as i32 {
                            0 => {}
                            1 => {
                                actions.push(interp.action);
                            }
                            _ => {
                                actions.extend_from_slice(&interp.actions);
                            }
                        }
                    }
                    if (actions.len() as u32 != 0) as i64 > MAX_ACTIONS_PER_LEVEL as i64 {
                        log::warn!("Could not append interpret actions to key <{}>: maximum is {}, got: {}. Dropping excessive actions\n",
                            atom_text(&keymap.ctx.atom_table, key_name),
                            65535_i32,
                            actions.len() as u32);
                        actions.truncate(MAX_ACTIONS_PER_LEVEL as usize);
                    }
                    keymap.keys[key_idx].groups[group as usize].levels[level as usize].actions =
                        std::mem::take(&mut actions);
                    if !keymap.keys[key_idx].groups[group as usize].levels[level as usize]
                        .actions
                        .is_empty()
                    {
                        keymap.keys[key_idx].groups[group as usize].implicit_actions = true;
                    }
                }
                level = level.wrapping_add(1);
            }
            if keymap.keys[key_idx].groups[group as usize].implicit_actions {
                keymap.keys[key_idx].implicit_actions = true;
            }
        }
        group = group.wrapping_add(1);
    }
    if keymap.keys[key_idx].explicit & EXPLICIT_VMODMAP == 0 {
        keymap.keys[key_idx].vmodmap = vmodmap;
    }
    true
}
#[inline]
fn is_mod_action(action: &XkbAction) -> bool {
    action.action_type() == ACTION_TYPE_MOD_SET
        || action.action_type() == ACTION_TYPE_MOD_LATCH
        || action.action_type() == ACTION_TYPE_MOD_LOCK
}
#[inline]
fn is_group_action(action: &XkbAction) -> bool {
    action.action_type() == ACTION_TYPE_GROUP_SET
        || action.action_type() == ACTION_TYPE_GROUP_LATCH
        || action.action_type() == ACTION_TYPE_GROUP_LOCK
}
fn CheckMultipleActionsCategories(keymap: &mut XkbKeymap, key_idx: usize) {
    let num_groups = keymap.keys[key_idx].num_groups;
    let key_name = keymap.keys[key_idx].name;
    let mut g: u32 = 0_u32;
    while g < num_groups {
        let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], g);
        let mut l: u32 = 0_u32;
        while l < num_levels {
            let level: &mut XkbLevel =
                &mut keymap.keys[key_idx].groups[g as usize].levels[l as usize];
            if level.actions.len() > 1 {
                let mut i: u16 = 0_u16;
                while (i as usize) < level.actions.len() {
                    let mod_action: bool = is_mod_action(&level.actions[i as usize]);
                    let group_action: bool = is_group_action(&level.actions[i as usize]);
                    let action1_type = level.actions[i as usize].action_type();
                    if mod_action as i32 != 0
                        || group_action as i32 != 0
                        || action1_type == ACTION_TYPE_REDIRECT_KEY
                    {
                        let mut j: u16 = (i as i32 + 1_i32) as u16;
                        while (j as usize) < level.actions.len() {
                            if action1_type == level.actions[j as usize].action_type()
                                || mod_action as i32 != 0
                                    && is_mod_action(&level.actions[j as usize]) as i32 != 0
                                || group_action as i32 != 0
                                    && is_group_action(&level.actions[j as usize]) as i32 != 0
                            {
                                let type_0: &str = if mod_action as i32 != 0 {
                                    "modifiers"
                                } else if group_action as i32 != 0 {
                                    "group"
                                } else {
                                    ActionTypeText(action1_type)
                                };
                                log::error!("Cannot use multiple {} actions in the same level. Action #{} for key <{}> in group {}/level {} ignored.\n",
                                    type_0,
                                    j as i32 + 1_i32,
                                    atom_text(&keymap.ctx.atom_table, key_name),
                                    g.wrapping_add(1_u32),
                                    l.wrapping_add(1_u32));
                                level.actions[j as usize].set_none();
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
fn add_key_aliases(keymap: &XkbKeymap, min: u32, max: u32, aliases: &mut Vec<XkbKeyAlias>) {
    let mut alias: u32 = min;
    while alias <= max {
        let entry: KeycodeMatch = keymap.key_names[alias as usize];
        if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
            aliases.push(XkbKeyAlias {
                real: entry.index,
                alias,
            });
        }
        alias = alias.wrapping_add(1);
    }
}
fn update_pending_key_fields(info: &mut XkbKeymapInfo<'_>, key_idx: usize) -> bool {
    if info.keymap.keys[key_idx].out_of_range_pending_group {
        let idx = info.keymap.keys[key_idx].out_of_range_group_number as usize;
        if !info.pending_computations[idx].computed {
            // Temporarily take the expr out to avoid borrow conflict with info
            let expr_box = info.pending_computations[idx].expr.take().unwrap();
            let mut group: u32 = 0_u32;
            let mut pending_dummy = false;
            let resolve_ret =
                ExprResolveGroup(info, &expr_box, true, &mut group, &mut pending_dummy);
            info.pending_computations[idx].expr = Some(expr_box);
            match resolve_ret {
                0 => {
                    info.pending_computations[idx].computed = true;
                    info.pending_computations[idx].value = group.wrapping_sub(1_u32);
                }
                2 => {
                    log::error!("[XKB-{:03}] Invalid key redirect group index\n", {
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                    });
                    return info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0;
                }
                _ => {}
            }
        }
        info.keymap.keys[key_idx].out_of_range_pending_group = false;
        info.keymap.keys[key_idx].out_of_range_group_number = info.pending_computations[idx].value;
    }
    true
}
fn update_pending_action_fields(
    info: &mut XkbKeymapInfo<'_>,
    keycode: u32,
    act: &mut XkbAction,
) -> bool {
    match act.action_type() {
        5..=7 => {
            if act.as_group().flags & ACTION_PENDING_COMPUTATION != 0 {
                let pc_idx = act.as_group().group as usize;
                if !info.pending_computations[pc_idx].computed {
                    let mut group: u32 = 0_u32;
                    let absolute: bool = act.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0;
                    let mut pending_dummy = false;
                    let expr_box = info.pending_computations[pc_idx].expr.take().unwrap();
                    let resolve_ret =
                        ExprResolveGroup(info, &expr_box, absolute, &mut group, &mut pending_dummy);
                    info.pending_computations[pc_idx].expr = Some(expr_box);
                    match resolve_ret {
                        2 => {
                            log::error!("[XKB-{:03}] Invalid action group index\n", {
                                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                            });
                            return false;
                        }
                        1 => {}
                        _ => {
                            info.pending_computations[pc_idx].computed = true;
                            if absolute {
                                info.pending_computations[pc_idx].value = group.wrapping_sub(1_u32);
                            } else {
                                info.pending_computations[pc_idx].value = group;
                                if info.pending_computations[pc_idx]
                                    .expr
                                    .as_ref()
                                    .unwrap()
                                    .stmt_type()
                                    == STMT_EXPR_NEGATE
                                {
                                    info.pending_computations[pc_idx].value =
                                        -(info.pending_computations[pc_idx].value as i32) as u32;
                                }
                            }
                        }
                    }
                }
                act.as_group_mut().group = info.pending_computations[pc_idx].value as i32;
                act.as_group_mut().flags = (act.as_group().flags
                    & !(ACTION_PENDING_COMPUTATION as i32) as u32)
                    as XkbActionFlags;
            }
            true
        }
        16 => {
            if keycode == XKB_KEYCODE_INVALID
                || act.as_redirect().keycode != info.keymap.redirect_key_auto
            {
                return true;
            } else {
                act.as_redirect_mut().keycode = keycode;
            }
            true
        }
        _ => true,
    }
}
fn UpdateDerivedKeymapFields(info: &mut XkbKeymapInfo<'_>) -> bool {
    let keymap: &mut XkbKeymap = &mut *info.keymap;
    let mut num_key_aliases: u32 = 0_u32;
    let mut min_alias: u32 = 0_u32;
    let mut max_alias: u32 = 0_u32;
    let mut alias: u32 = 0_u32;
    while (alias as usize) < keymap.key_names.len() {
        let entry: KeycodeMatch = keymap.key_names[alias as usize];
        if entry.is_alias as i32 != 0 && entry.found as i32 != 0 {
            if num_key_aliases == 0 {
                min_alias = alias;
            }
            max_alias = alias;
            num_key_aliases = num_key_aliases.wrapping_add(1);
        }
        alias = alias.wrapping_add(1);
    }
    if num_key_aliases != 0 {
        let mut aliases: Vec<XkbKeyAlias> = Vec::with_capacity(num_key_aliases as usize);
        add_key_aliases(keymap, min_alias, max_alias, &mut aliases);
        keymap.key_aliases = aliases;
    }
    // key_names is no longer needed after compilation; drop it
    keymap.key_names = Vec::new();
    {
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            let key_num_groups = keymap.keys[ki as usize].num_groups;
            keymap.num_groups = if keymap.num_groups > key_num_groups {
                keymap.num_groups
            } else {
                key_num_groups
            };
            ki = ki.wrapping_add(1);
        }
    }
    let pending_computations: bool = !info.pending_computations.is_empty();
    if pending_computations {
        let num_groups: u32 = if info.keymap.num_groups != 0 {
            info.keymap.num_groups
        } else {
            1_u32
        };
        info.lookup.group_index_names[GROUP_INDEX_NAME_LAST as usize] = LookupEntry {
            name: GROUP_LAST_INDEX_NAME,
            value: num_groups,
        };
        info.lookup.group_mask_names[GROUP_MASK_NAME_LAST as usize] = LookupEntry {
            name: GROUP_LAST_INDEX_NAME,
            value: 1_u32 << num_groups.wrapping_sub(1_u32),
        };
        let mut i: u32 = 0_u32;
        while (i as usize) < info.keymap.sym_interprets.len() {
            let num_actions = info.keymap.sym_interprets[i as usize].num_actions;
            if num_actions as i32 <= 1_i32 {
                // Extract action, update, write back
                let mut action = info.keymap.sym_interprets[i as usize].action;
                if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, &mut action) {
                    return false;
                }
                info.keymap.sym_interprets[i as usize].action = action;
            } else {
                let mut a: u16 = 0_u16;
                while (a as i32) < num_actions as i32 {
                    let mut action = info.keymap.sym_interprets[i as usize].actions[a as usize];
                    if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, &mut action) {
                        return false;
                    }
                    info.keymap.sym_interprets[i as usize].actions[a as usize] = action;
                    a = a.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    {
        let keymap = &mut *info.keymap;
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            if !ApplyInterpsToKey(keymap, ki as usize) {
                return false;
            }
            CheckMultipleActionsCategories(keymap, ki as usize);
            ki = ki.wrapping_add(1);
        }
    }
    {
        let keymap = &mut *info.keymap;
        let start_idx = if keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < keymap.num_keys {
            let key_vmodmap = keymap.keys[ki as usize].vmodmap;
            let key_modmap = keymap.keys[ki as usize].modmap;
            let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            while idx < keymap.mods.num_mods {
                if key_vmodmap & 1_u32 << idx != 0 {
                    keymap.mods.mods[idx as usize].mapping |= key_modmap;
                }
                idx = idx.wrapping_add(1);
            }
            ki = ki.wrapping_add(1);
        }
    }
    {
        let keymap = &mut *info.keymap;
        if keymap.format >= XKB_KEYMAP_FORMAT_TEXT_V2 {
            let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            while idx < keymap.mods.num_mods {
                let mask: u32 = 1_u32 << idx;
                if keymap.mods.mods[idx as usize].mapping == 0_u32
                    && keymap.mods.explicit_vmods & mask == 0
                {
                    keymap.mods.mods[idx as usize].mapping = mask;
                    keymap.mods.explicit_vmods |= mask;
                }
                idx = idx.wrapping_add(1);
            }
        }
        let mut extra_canonical_mods: u32 = 0_u32;
        {
            let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
            while idx < keymap.mods.num_mods {
                extra_canonical_mods |= keymap.mods.mods[idx as usize].mapping;
                idx = idx.wrapping_add(1);
            }
        }
        keymap.canonical_state_mask |= extra_canonical_mods;
        let mut i_0: u32 = 0_u32;
        while (i_0 as usize) < keymap.types.len() {
            compute_effective_mask_with(&keymap.mods, &mut keymap.types[i_0 as usize].mods);
            let mut j: u32 = 0_u32;
            while j < keymap.types[i_0 as usize].entries.len() as u32 {
                let res = {
                    // has_unbound_vmods inlined
                    let entry_mods = keymap.types[i_0 as usize].entries[j as usize].mods.mods;
                    let mut unbound = false;
                    let mut k: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
                    while k < keymap.mods.num_mods {
                        if entry_mods & 1_u32 << k != 0
                            && keymap.mods.mods[k as usize].mapping == 0_u32
                        {
                            unbound = true;
                            break;
                        }
                        k = k.wrapping_add(1);
                    }
                    unbound
                };
                if res {
                    keymap.types[i_0 as usize].entries[j as usize].mods.mask = 0_u32;
                } else {
                    compute_effective_mask_with(
                        &keymap.mods,
                        &mut keymap.types[i_0 as usize].entries[j as usize].mods,
                    );
                    compute_effective_mask_with(
                        &keymap.mods,
                        &mut keymap.types[i_0 as usize].entries[j as usize].preserve,
                    );
                }
                j = j.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    {
        let start_idx = if info.keymap.num_keys_low == 0_u32 {
            0_u32
        } else {
            info.keymap.min_key_code
        };
        let mut ki: u32 = start_idx;
        while ki < info.keymap.num_keys {
            if !update_pending_key_fields(info, ki as usize) {
                return false;
            }
            let key_num_groups = info.keymap.keys[ki as usize].num_groups;
            let key_modmap = info.keymap.keys[ki as usize].modmap;
            let key_keycode = info.keymap.keys[ki as usize].keycode;
            let mut i_1: u32 = 0_u32;
            while i_1 < key_num_groups {
                let num_levels = {
                    let key = &info.keymap.keys[ki as usize];
                    info.keymap.types[key.groups[i_1 as usize].type_idx as usize].num_levels
                };
                let mut j_0: u32 = 0_u32;
                while j_0 < num_levels {
                    let num_actions = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                        [j_0 as usize]
                        .actions
                        .len();
                    if num_actions <= 1 {
                        if num_actions == 1 {
                            let mut act = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[0];
                            UpdateActionMods(&*info.keymap, &mut act, key_modmap);
                            if (pending_computations as i32 != 0
                                || act.action_type() == ACTION_TYPE_REDIRECT_KEY)
                                && !update_pending_action_fields(info, key_keycode, &mut act)
                            {
                                return false;
                            }
                            info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[0] = act;
                        }
                    } else {
                        let mut k: u16 = 0_u16;
                        while (k as usize) < num_actions {
                            let mut act = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[k as usize];
                            UpdateActionMods(&*info.keymap, &mut act, key_modmap);
                            if (pending_computations as i32 != 0
                                || act.action_type() == ACTION_TYPE_REDIRECT_KEY)
                                && !update_pending_action_fields(info, key_keycode, &mut act)
                            {
                                return false;
                            }
                            info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                                [j_0 as usize]
                                .actions[k as usize] = act;
                            k = k.wrapping_add(1);
                        }
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                i_1 = i_1.wrapping_add(1);
            }
            ki = ki.wrapping_add(1);
        }
    }
    let keymap = &mut *info.keymap;
    let mut led_idx: u32 = 0_u32;
    while led_idx < keymap.num_leds {
        compute_effective_mask_with(&keymap.mods, &mut keymap.leds[led_idx as usize].mods);
        led_idx = led_idx.wrapping_add(1);
    }
    if pending_computations {
        let mut led_idx: u32 = 0_u32;
        while led_idx < info.keymap.num_leds {
            if info.keymap.leds[led_idx as usize].pending_groups {
                let groups_idx = info.keymap.leds[led_idx as usize].groups as usize;
                if !info.pending_computations[groups_idx].computed {
                    let expr_box = info.pending_computations[groups_idx].expr.take().unwrap();
                    let mut mask: u32 = 0_u32;
                    let mut pending_dummy = false;
                    let resolved =
                        ExprResolveGroupMask(info, &expr_box, &mut mask, &mut pending_dummy);
                    info.pending_computations[groups_idx].expr = Some(expr_box);
                    if !resolved {
                        log::error!("[XKB-{:03}] Invalid LED group mask\n", {
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX
                        });
                        return false;
                    }
                    info.pending_computations[groups_idx].computed = true;
                    info.pending_computations[groups_idx].value = mask;
                }
                let value = info.pending_computations[groups_idx].value;
                info.keymap.leds[led_idx as usize].pending_groups = false;
                info.keymap.leds[led_idx as usize].groups = value;
            }
            led_idx = led_idx.wrapping_add(1);
        }
    }
    true
}
static COMPILE_FILE_FNS: [compile_file_fn; 4] = {
    [
        Some(CompileKeycodes as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
        Some(CompileKeyTypes as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
        Some(CompileCompatMap as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
        Some(CompileSymbols as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
    ]
};
fn pending_computations_array_free(p: &mut Vec<PendingComputation>) {
    for pc in p.iter_mut() {
        pc.expr.take(); // Drop handles cleanup
    }
    p.clear();
}
pub(crate) fn CompileKeymap(file: &mut XkbFile, keymap: &mut XkbKeymap) -> bool {
    let mut file_indices: [Option<usize>; 4] = [None; 4];
    for (idx, stmt) in file.defs.iter().enumerate() {
        if let Statement::XkbFile(ref sub_file) = stmt {
            if sub_file.file_type > LAST_KEYMAP_FILE_TYPE {
                if sub_file.file_type == FILE_TYPE_GEOMETRY {
                    log::warn!(
                        "[XKB-{:03}] Geometry sections are not supported; ignoring\n",
                        XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION as i32
                    );
                } else {
                    log::error!(
                        "Cannot define {} in a keymap file\n",
                        xkb_file_type_to_string(sub_file.file_type)
                    );
                }
            } else if file_indices[sub_file.file_type as usize].is_some() {
                log::error!("More than one {} section in keymap file; All sections after the first ignored\n",
                    xkb_file_type_to_string(sub_file.file_type));
            } else {
                file_indices[sub_file.file_type as usize] = Some(idx);
            }
        }
    }
    let km_format = keymap.format;
    let km_flags = keymap.flags;
    let km_num_groups = keymap.num_groups;
    let mut info = XkbKeymapInfo {
        keymap,
        strict: (if km_format == XKB_KEYMAP_FORMAT_TEXT_V1 {
            if km_flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
                PARSER_V1_STRICT_FLAGS as i32
            } else {
                PARSER_V1_LAX_FLAGS as i32
            }
        } else if km_flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
            PARSER_V2_STRICT_FLAGS as i32
        } else {
            PARSER_V2_LAX_FLAGS as i32
        }) as u32,
        features: XkbcompFeatures {
            max_groups: format_max_groups(km_format),
            max_overlays: format_max_overlays(km_format),
            controls_name_offset: format_control_names_offset(km_format),
            group_lock_on_release: is_group_lock_on_release_supported(km_format),
            mods_unlock_on_press: is_mods_un_lock_on_press_supported(km_format),
            mods_latch_on_press: is_mods_latch_on_press_supported(km_format),
            overlapping_overlays: are_overlapping_overlays_supported(km_format),
        },
        lookup: XkbcompLookup {
            group_index_names: [
                LookupEntry {
                    name: "first",
                    value: 1_u32,
                },
                LookupEntry {
                    name: if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    value: km_num_groups,
                },
                LookupEntry {
                    name: "",
                    value: 0_u32,
                },
            ],
            group_mask_names: [
                LookupEntry {
                    name: "none",
                    value: 0_u32,
                },
                LookupEntry {
                    name: "first",
                    value: 0x1_u32,
                },
                LookupEntry {
                    name: "all",
                    value: XKB_ALL_GROUPS as u32,
                },
                LookupEntry {
                    name: if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    value: if km_num_groups != 0 && km_num_groups <= XKB_MAX_GROUPS as u32 {
                        1_u32 << km_num_groups.wrapping_sub(1_u32)
                    } else {
                        0_u32
                    },
                },
                LookupEntry {
                    name: "",
                    value: 0_u32,
                },
            ],
        },
        pending_computations: Vec::new(),
    };
    let mut type_0: u32 = FIRST_KEYMAP_FILE_TYPE;
    while type_0 <= LAST_KEYMAP_FILE_TYPE {
        if file_indices[type_0 as usize].is_none() {
            log::debug!(
                "Component {} not provided in keymap\n",
                xkb_file_type_to_string(type_0)
            );
        } else {
            let idx = file_indices[type_0 as usize].unwrap();
            let sub_name = if let Statement::XkbFile(ref sub_file) = file.defs[idx] {
                safe_map_name(sub_file)
            } else {
                ""
            };
            log::debug!(
                "Compiling {} \"{}\"\n",
                xkb_file_type_to_string(type_0),
                sub_name
            );
        }
        let file_arg: Option<&mut XkbFile> = file_indices[type_0 as usize].map(|idx| {
            if let Statement::XkbFile(ref mut sub_file) = file.defs[idx] {
                &mut **sub_file
            } else {
                unreachable!()
            }
        });
        let ok: bool = COMPILE_FILE_FNS[type_0 as usize].expect("non-null function pointer")(
            file_arg, &mut info,
        ) as bool;
        if !ok {
            log::error!("Failed to compile {}\n", xkb_file_type_to_string(type_0));
            pending_computations_array_free(&mut info.pending_computations);
            return false;
        }
        type_0 += 1;
    }
    let ok_0: bool = UpdateDerivedKeymapFields(&mut info) as bool;
    pending_computations_array_free(&mut info.pending_computations);
    ok_0
}
pub(crate) const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;

/// Appends bytes from `src` to the Vec<i8>.
#[inline]
fn vec_append_nul_terminated(v: &mut Vec<i8>, src: &[u8]) {
    v.extend(src.iter().map(|&b| b as i8));
}

/// Index-based sval for scanner input. Used in lvalue/rule to avoid
/// lifetime issues across include boundaries. Reconstruct sval via to_sval().
#[derive(Copy, Clone, Default)]
pub(crate) struct SvalIdx {
    start: usize,
    end: usize,
}
impl SvalIdx {
    const EMPTY: SvalIdx = SvalIdx { start: 0, end: 0 };
    #[inline]
    fn to_sval<'a>(&self, input: &'a [u8]) -> sval<'a> {
        if self.start >= self.end || self.start >= input.len() {
            sval::EMPTY
        } else {
            sval {
                data: &input[self.start..self.end.min(input.len())],
            }
        }
    }
    #[inline]
    fn len(&self) -> usize {
        self.end - self.start
    }
}

pub(crate) struct matcher<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    pub(crate) rmlvo: rule_names<'a>,
    pub(crate) val: lvalue,
    pub(crate) groups: Vec<group>,
    pub(crate) mapping: mapping,
    pub(crate) rule: rule,
    pub(crate) pending_kccgst: kccgst_buffer,
    pub(crate) kccgst: [Vec<i8>; 5],
}
#[derive(Clone, Default)]
pub(crate) struct kccgst_buffer {
    pub(crate) buffer: Vec<i8>,
    pub(crate) slices: Vec<kccgst_buffer_slice>,
}
#[derive(Copy, Clone)]
pub(crate) struct kccgst_buffer_slice {
    pub(crate) length: u32,
    pub(crate) kccgst: rules_kccgst,
    pub(crate) layout: u32,
}
pub(crate) type rules_kccgst = u32;
pub(crate) const _KCCGST_NUM_ENTRIES: rules_kccgst = 5;
pub(crate) const KCCGST_GEOMETRY: rules_kccgst = 4;
pub(crate) const KCCGST_SYMBOLS: rules_kccgst = 3;
pub(crate) const KCCGST_COMPAT: rules_kccgst = 2;
pub(crate) const KCCGST_TYPES: rules_kccgst = 1;
pub(crate) const KCCGST_KEYCODES: rules_kccgst = 0;
#[derive(Copy, Clone)]
pub(crate) struct rule {
    pub(crate) mlvo_value_at_pos: [SvalIdx; 4],
    pub(crate) match_type_at_pos: [mlvo_match_type; 4],
    pub(crate) kccgst_value_at_pos: [SvalIdx; 5],
    pub(crate) num_mlvo_values: mlvo_index_t,
    pub(crate) num_kccgst_values: kccgst_index_t,
    pub(crate) skip: bool,
}
pub(crate) type kccgst_index_t = u8;
pub(crate) type mlvo_index_t = u8;
pub(crate) type mlvo_match_type = u32;
pub(crate) const MLVO_MATCH_GROUP: mlvo_match_type = 5;
pub(crate) const MLVO_MATCH_WILDCARD_ANY: mlvo_match_type = 4;
pub(crate) const MLVO_MATCH_WILDCARD_SOME: mlvo_match_type = 3;
pub(crate) const MLVO_MATCH_WILDCARD_NONE: mlvo_match_type = 2;
pub(crate) const MLVO_MATCH_WILDCARD_LEGACY: mlvo_match_type = 1;
pub(crate) const MLVO_MATCH_NORMAL: mlvo_match_type = 0;
#[derive(Copy, Clone, Default)]
pub(crate) struct mapping {
    pub(crate) mlvo_at_pos: [rules_mlvo; 4],
    pub(crate) num_mlvo: mlvo_index_t,
    pub(crate) defined_mlvo_mask: mlvo_mask_t,
    pub(crate) layout: LayoutIdx,
    pub(crate) active_or_candidates_mask: u32,
    pub(crate) kccgst_at_pos: [rules_kccgst; 5],
    pub(crate) num_kccgst: kccgst_index_t,
    pub(crate) defined_kccgst_mask: u8,
}
#[derive(Copy, Clone)]
pub(crate) enum LayoutIdx {
    Single {
        layout_idx: u32,
        variant_idx: u32,
    },
    Range {
        layout_idx_min: u32,
        layout_idx_max: u32,
    },
    Index {
        layout_idx_min: u32,
        layout_idx_max: u32,
    },
}
impl Default for LayoutIdx {
    fn default() -> Self {
        LayoutIdx::Single {
            layout_idx: 0,
            variant_idx: 0,
        }
    }
}
impl LayoutIdx {
    fn layout_idx_min(&self) -> u32 {
        match self {
            LayoutIdx::Range { layout_idx_min, .. } | LayoutIdx::Index { layout_idx_min, .. } => {
                *layout_idx_min
            }
            _ => panic!("expected Range or Index"),
        }
    }
}
pub(crate) type mlvo_mask_t = u8;
pub(crate) type rules_mlvo = u32;
pub(crate) const _MLVO_NUM_ENTRIES: rules_mlvo = 4;
pub(crate) const MLVO_OPTION: rules_mlvo = 3;
pub(crate) const MLVO_VARIANT: rules_mlvo = 2;
pub(crate) const MLVO_LAYOUT: rules_mlvo = 1;
pub(crate) const MLVO_MODEL: rules_mlvo = 0;
#[derive(Clone)]
pub(crate) struct group {
    pub(crate) name: Vec<u8>,
    pub(crate) elements: Vec<Vec<u8>>,
}
#[derive(Copy, Clone)]
pub(crate) struct lvalue {
    pub(crate) string: SvalIdx,
}
#[derive(Clone, Default)]
pub(crate) struct rule_names<'a> {
    pub(crate) model: matched_sval<'a>,
    pub(crate) layouts: Vec<matched_sval<'a>>,
    pub(crate) variants: Vec<matched_sval<'a>>,
    pub(crate) options: Vec<matched_sval<'a>>,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct matched_sval<'a> {
    pub(crate) sval: sval<'a>,
    pub(crate) matched: bool,
    pub(crate) layout: u32,
}
pub(crate) const TOK_ERROR: rules_token = 11;
pub(crate) type rules_token = u32;
pub(crate) const TOK_INCLUDE: rules_token = 10;
pub(crate) const TOK_WILD_CARD_ANY: rules_token = 9;
pub(crate) const TOK_WILD_CARD_SOME: rules_token = 8;
pub(crate) const TOK_WILD_CARD_NONE: rules_token = 7;
pub(crate) const TOK_WILD_CARD_STAR: rules_token = 6;
pub(crate) const TOK_EQUALS: rules_token = 5;
pub(crate) const TOK_BANG: rules_token = 4;
pub(crate) const TOK_GROUP_NAME: rules_token = 3;
pub(crate) const TOK_IDENTIFIER: rules_token = 2;
pub(crate) const TOK_END_OF_LINE: rules_token = 1;
pub(crate) const TOK_END_OF_FILE: rules_token = 0;
pub(crate) const LAYOUT_INDEX_FIRST: layout_index_ranges = 4294967292;
pub(crate) const LAYOUT_INDEX_SINGLE: layout_index_ranges = 4294967291;
pub(crate) const LAYOUT_INDEX_ANY: layout_index_ranges = 4294967294;
pub(crate) const LAYOUT_INDEX_LATER: layout_index_ranges = 4294967293;
pub(crate) type layout_index_ranges = u32;

impl Default for rule {
    fn default() -> Self {
        rule {
            mlvo_value_at_pos: [SvalIdx::EMPTY; 4],
            match_type_at_pos: [0; 4],
            kccgst_value_at_pos: [SvalIdx::EMPTY; 5],
            num_mlvo_values: 0,
            num_kccgst_values: 0,
            skip: false,
        }
    }
}
impl Default for lvalue {
    fn default() -> Self {
        lvalue {
            string: SvalIdx::EMPTY,
        }
    }
}
impl<'a> matcher<'a> {
    fn new(ctx: &'a mut XkbContext) -> Self {
        matcher {
            ctx,
            rmlvo: rule_names::default(),
            val: lvalue::default(),
            groups: Vec::new(),
            mapping: mapping::default(),
            rule: rule::default(),
            pending_kccgst: kccgst_buffer::default(),
            kccgst: std::array::from_fn(|_| Vec::new()),
        }
    }
}
pub(crate) type wildcard_match_type = u32;
pub(crate) const WILDCARD_MATCH_ALL: wildcard_match_type = 1;
pub(crate) const WILDCARD_MATCH_NONEMPTY: wildcard_match_type = 0;
pub(crate) const MAX_INCLUDE_DEPTH: i32 = 5_i32;
#[inline]
fn is_ident(ch: i8) -> bool {
    (ch as u8).is_ascii_graphic() && ch as i32 != '\\' as i32
}
fn lex(s: &mut scanner, val: &mut lvalue) -> rules_token {
    loop {
        while s.chr(' ' as i32 as i8) as i32 != 0
            || s.chr('\t' as i32 as i8) as i32 != 0
            || s.chr('\r' as i32 as i8) as i32 != 0
        {}
        if s.str_match(b"//") {
            s.skip_to_eol();
        }
        if s.eol() {
            while s.eol() {
                s.next_byte();
            }
            return TOK_END_OF_LINE;
        }
        if !s.chr('\\' as i32 as i8) {
            break;
        }
        s.chr('\r' as i32 as i8);
        if !s.eol() {
            let loc: scanner_loc = s.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: illegal new line escape; must appear at end of line\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc.line,
                loc.column
            );
            return TOK_ERROR;
        }
        s.next_byte();
    }
    if s.eof() {
        return TOK_END_OF_FILE;
    }
    s.token_pos = s.pos;
    if s.chr('!' as i32 as i8) {
        return TOK_BANG;
    }
    if s.chr('=' as i32 as i8) {
        return TOK_EQUALS;
    }
    if s.chr('*' as i32 as i8) {
        return TOK_WILD_CARD_STAR;
    }
    if s.str_match(b"<none>") {
        return TOK_WILD_CARD_NONE;
    }
    if s.str_match(b"<some>") {
        return TOK_WILD_CARD_SOME;
    }
    if s.str_match(b"<any>") {
        return TOK_WILD_CARD_ANY;
    }
    if s.chr('$' as i32 as i8) {
        val.string = SvalIdx {
            start: s.pos,
            end: s.pos,
        };
        while is_ident(s.peek()) {
            s.next_byte();
            val.string.end += 1;
        }
        if val.string.len() == 0 {
            let loc_0: scanner_loc = s.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: unexpected character after '$'; expected name\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_0.line,
                loc_0.column
            );
            return TOK_ERROR;
        }
        return TOK_GROUP_NAME;
    }
    if s.str_match(b"include") {
        return TOK_INCLUDE;
    }
    if is_ident(s.peek()) {
        val.string = SvalIdx {
            start: s.pos,
            end: s.pos,
        };
        while is_ident(s.peek()) {
            s.next_byte();
            val.string.end += 1;
        }
        return TOK_IDENTIFIER;
    }
    let loc_1: scanner_loc = s.token_location();
    log::error!(
        "[XKB-{:03}] {}:{}:{}: unrecognized token\n",
        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
        &s.file_name,
        loc_1.line,
        loc_1.column
    );
    TOK_ERROR
}
static RULES_MLVO_SVALS: [&[u8]; 4] = [b"model", b"layout", b"variant", b"option"];
static RULES_KCCGST_SVALS: [&[u8]; 5] = [b"keycodes", b"types", b"compat", b"symbols", b"geometry"];
pub(crate) const OPTIONS_MATCH_ALL_GROUPS: i32 = XKB_MAX_GROUPS;
fn strip_spaces<'a>(v: sval<'a>) -> sval<'a> {
    let bytes = v.data;
    let start_trim = bytes
        .iter()
        .position(|&b| !is_space(b as i8))
        .unwrap_or(bytes.len());
    let end_trim = bytes
        .iter()
        .rposition(|&b| !is_space(b as i8))
        .map(|i| i + 1)
        .unwrap_or(start_trim);
    if start_trim >= end_trim {
        sval::EMPTY
    } else {
        sval {
            data: &bytes[start_trim..end_trim],
        }
    }
}

/// Resize a Vec<matched_sval>, zero-filling new elements.
fn vec_resize_zero_matched_sval(v: &mut Vec<matched_sval<'_>>, new_len: usize) {
    if new_len > v.len() {
        v.resize(new_len, matched_sval::default());
    } else {
        v.truncate(new_len);
    }
}

fn split_comma_separated_mlvo<'a>(mlvo: rules_mlvo, s: Option<&'a [u8]>) -> Vec<matched_sval<'a>> {
    let mut arr: Vec<matched_sval<'a>> = Vec::new();
    let Some(bytes) = s else {
        arr.push(matched_sval::default());
        return arr;
    };
    if bytes.is_empty() {
        arr.push(matched_sval::default());
        return arr;
    }
    let mut pos: usize = 0;
    loop {
        let start = pos;
        let mut end = pos;
        let mut val_0 = matched_sval {
            matched: false,
            layout: OPTIONS_MATCH_ALL_GROUPS as u32,
            sval: sval {
                data: &bytes[start..start],
            },
        };
        while pos < bytes.len()
            && bytes[pos] != b','
            && bytes[pos] as i32 != OPTIONS_GROUP_SPECIFIER_PREFIX
        {
            pos += 1;
            end += 1;
        }
        val_0.sval = sval {
            data: &bytes[start..end],
        };
        val_0.sval = strip_spaces(val_0.sval);
        if pos < bytes.len() && bytes[pos] as i32 == OPTIONS_GROUP_SPECIFIER_PREFIX {
            pos += 1;
            let layout_start = pos;
            #[allow(unused_assignments)]
            let mut layout: u32 = XKB_LAYOUT_INVALID;
            let (val_parsed, count) = parse_dec_u32(&bytes[pos..]);
            layout = val_parsed;
            let count = count as usize;
            if count > 0 {
                pos += count;
                if layout == 0 || layout > XKB_MAX_GROUPS as u32 {
                    log::error!(
                        "[XKB-{:03}] Invalid layout index {} for the RMVLO component: \"{}\"\n",
                        { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                        layout,
                        val_0.sval.as_str()
                    );
                } else if mlvo != MLVO_OPTION {
                    log::warn!(
                        "Layout index {} is not supported for the RMLVO component: \"{}\"\n",
                        layout,
                        val_0.sval.as_str()
                    );
                } else {
                    val_0.layout = layout.wrapping_sub(1);
                }
            }
            let layout_index_end = pos;
            while pos < bytes.len() && bytes[pos] != b',' {
                pos += 1;
            }
            if count == 0 || layout_index_end != pos {
                let layout_spec = std::str::from_utf8(&bytes[layout_start..pos]).unwrap_or("");
                log::error!("[XKB-{:03}] Invalid layout index \"{}\" for the RMLVO component \"{}\"; discarding specifier.\n",
                    { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                    layout_spec,
                    val_0.sval.as_str());
                val_0.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
            }
        }
        arr.push(val_0);
        if pos >= bytes.len() {
            break;
        }
        if bytes[pos] == b',' {
            pos += 1;
        }
    }
    arr
}
fn matcher_new_from_names<'a>(
    ctx: &'a mut XkbContext,
    rmlvo: &'a XkbRuleNames,
) -> Box<matcher<'a>> {
    let mut m = Box::new(matcher::new(ctx));
    let rmlvo_ref = rmlvo;
    m.rmlvo.model.sval = sval {
        data: rmlvo_ref.model.as_bytes(),
    };
    m.rmlvo.model.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
    m.rmlvo.layouts = split_comma_separated_mlvo(
        MLVO_LAYOUT,
        if rmlvo_ref.layout.as_bytes().is_empty() {
            None
        } else {
            Some(rmlvo_ref.layout.as_bytes())
        },
    );
    m.rmlvo.variants = split_comma_separated_mlvo(
        MLVO_VARIANT,
        if rmlvo_ref.variant.as_bytes().is_empty() {
            None
        } else {
            Some(rmlvo_ref.variant.as_bytes())
        },
    );
    m.rmlvo.options = split_comma_separated_mlvo(
        MLVO_OPTION,
        if rmlvo_ref.options.as_bytes().is_empty() {
            None
        } else {
            Some(rmlvo_ref.options.as_bytes())
        },
    );
    if m.rmlvo.layouts.len() > m.rmlvo.variants.len() {
        if !rmlvo_ref.variant.as_bytes().is_empty() {
            log::warn!(
                "More layouts than variants: \"{}\" vs. \"{}\".\n",
                if !rmlvo_ref.layout.as_bytes().is_empty() {
                    rmlvo_ref.layout.to_str().unwrap_or("")
                } else {
                    "(none)"
                },
                if !rmlvo_ref.variant.as_bytes().is_empty() {
                    rmlvo_ref.variant.to_str().unwrap_or("")
                } else {
                    "(none)"
                }
            );
        }
        vec_resize_zero_matched_sval(&mut m.rmlvo.variants, m.rmlvo.layouts.len());
    } else if m.rmlvo.layouts.len() < m.rmlvo.variants.len() {
        log::error!(
            "Less layouts than variants: \"{}\" vs. \"{}\".\n",
            if !rmlvo_ref.layout.as_bytes().is_empty() {
                rmlvo_ref.layout.to_str().unwrap_or("")
            } else {
                "(none)"
            },
            if !rmlvo_ref.variant.as_bytes().is_empty() {
                rmlvo_ref.variant.to_str().unwrap_or("")
            } else {
                "(none)"
            }
        );
        m.rmlvo.variants.truncate(m.rmlvo.layouts.len());
    }
    m
}
fn matcher_group_start_new(m: &mut matcher, name: &[u8]) {
    let group: group = group {
        name: name.to_vec(),
        elements: Vec::new(),
    };
    m.groups.push(group);
}
fn matcher_group_add_element(m: &mut matcher, _s: &mut scanner, element: &[u8]) {
    let last_group = m.groups.last_mut().unwrap();
    last_group.elements.push(element.to_vec());
}
fn matcher_include(
    m: &mut matcher<'_>,
    parent_scanner: &mut scanner,
    include_depth: u32,
    inc: sval,
) {
    if include_depth >= MAX_INCLUDE_DEPTH as u32 {
        let loc: scanner_loc = parent_scanner.token_location();
        log::error!(
            "{}:{}:{}: maximum include depth ({}) exceeded; maybe there is an include loop?\n",
            &parent_scanner.file_name,
            loc.line,
            loc.column,
            MAX_INCLUDE_DEPTH
        );
        return;
    }
    let inc_str = inc.as_str();
    let stmt_file: String =
        match expand_path_str(&parent_scanner.file_name, inc_str, FILE_TYPE_RULES) {
            Err(()) => return,
            Ok(Some(expanded)) => expanded,
            Ok(None) => inc_str.to_string(),
        };
    let expanded = stmt_file != inc_str;

    let absolute_path = stmt_file.starts_with('/');
    let mut offset: u32 = 0;
    let mut file_and_path: Option<(std::sync::Arc<Vec<u8>>, String)> = if absolute_path {
        read_file_cached(&stmt_file).map(|data| (data, stmt_file.clone()))
    } else if expanded {
        None
    } else {
        FindFileInXkbPath(
            &mut *m.ctx,
            &parent_scanner.file_name,
            &stmt_file,
            FILE_TYPE_RULES,
            &mut offset,
            true,
        )
    };

    while let Some((ref file_data, ref path)) = file_and_path {
        let ret: bool = read_rules_file(m, include_depth.wrapping_add(1_u32), file_data, path);
        let path_str = path.clone();
        let _ = file_and_path.take();
        if ret {
            return;
        }
        log::error!(
            "No components returned from included XKB rules \"{}\"\n",
            &path_str
        );
        if absolute_path {
            break;
        }
        offset += 1;
        file_and_path = FindFileInXkbPath(
            &mut *m.ctx,
            &parent_scanner.file_name,
            &stmt_file,
            FILE_TYPE_RULES,
            &mut offset,
            true,
        );
    }
    log::error!("Failed to open included XKB rules \"{}\"\n", &stmt_file);
}
fn matcher_mapping_start_new(m: &mut matcher) {
    let mut i: mlvo_index_t = 0 as mlvo_index_t;
    while (i as i32) < _MLVO_NUM_ENTRIES as i32 as mlvo_index_t as i32 {
        m.mapping.mlvo_at_pos[i as usize] = _MLVO_NUM_ENTRIES;
        i = i.wrapping_add(1);
    }
    let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
    while (i_0 as i32) < _KCCGST_NUM_ENTRIES as i32 as kccgst_index_t as i32 {
        m.mapping.kccgst_at_pos[i_0 as usize] = _KCCGST_NUM_ENTRIES;
        i_0 = i_0.wrapping_add(1);
    }
    m.mapping.layout = LayoutIdx::Single {
        layout_idx: XKB_LAYOUT_INVALID,
        variant_idx: XKB_LAYOUT_INVALID,
    };
    m.mapping.num_kccgst = 0 as kccgst_index_t;
    m.mapping.num_mlvo = m.mapping.num_kccgst as mlvo_index_t;
    m.mapping.defined_mlvo_mask = 0 as mlvo_mask_t;
    m.mapping.defined_kccgst_mask = 0_u8;
    m.mapping.active_or_candidates_mask = 1_u32;
}
fn parse_layout_int_index(s: &[u8], out: &mut u32) -> i32 {
    // s starts with '[', parse integer between brackets
    if s.len() < 3 {
        return -1_i32;
    }
    let inner = &s[1..]; // skip '['
    let (val, count) = parse_dec_u32(inner);
    let count: i32 = count;
    if count <= 0_i32
        || (1 + count as usize) >= s.len()
        || s[1 + count as usize] != b']'
        || val == 0_u32
        || val > XKB_MAX_GROUPS as u32
    {
        return -1_i32;
    }
    *out = val.wrapping_sub(1_u32);
    count + 2_i32
}
fn extract_layout_index(s: &[u8], out: &mut u32) -> i32 {
    *out = XKB_LAYOUT_INVALID;
    if s.len() < 3 || s[0] != b'[' {
        return -1_i32;
    }
    if s.len() > 3 && s[1] == b'%' && s[2] == b'i' && s[3] == b']' {
        return 4_i32;
    }
    parse_layout_int_index(s, out)
}
fn extract_mapping_layout_index(s: &[u8], out: &mut u32) -> i32 {
    struct LayoutIndexEntry {
        name: &'static [u8],
        range: layout_index_ranges,
    }
    static NAMES: [LayoutIndexEntry; 4] = [
        LayoutIndexEntry {
            name: b"single]",
            range: LAYOUT_INDEX_SINGLE,
        },
        LayoutIndexEntry {
            name: b"first]",
            range: LAYOUT_INDEX_FIRST,
        },
        LayoutIndexEntry {
            name: b"later]",
            range: LAYOUT_INDEX_LATER,
        },
        LayoutIndexEntry {
            name: b"any]",
            range: LAYOUT_INDEX_ANY,
        },
    ];
    if s.len() < 3 || s[0] != b'[' {
        *out = XKB_LAYOUT_INVALID;
        return -1_i32;
    }
    let after_bracket = &s[1..];
    for entry in &NAMES {
        if after_bracket.starts_with(entry.name) {
            *out = entry.range;
            return (entry.name.len() + 1) as i32;
        }
    }
    *out = XKB_LAYOUT_INVALID;
    parse_layout_int_index(s, out)
}
#[inline]
fn is_mlvo_mask_defined(m: &mut matcher, mlvo: rules_mlvo) -> bool {
    m.mapping.defined_mlvo_mask as u32 & 1_u32 << mlvo != 0
}
fn matcher_mapping_set_mlvo(m: &mut matcher, s: &mut scanner, ident: sval) {
    let ident_bytes = ident.as_bytes();
    let mut mlvo: rules_mlvo = MLVO_MODEL;
    let mut mlvo_bytes: &[u8] = b"";
    while (mlvo as u32) < _MLVO_NUM_ENTRIES {
        mlvo_bytes = RULES_MLVO_SVALS[mlvo as usize];
        if mlvo_bytes.len() <= ident_bytes.len() && &ident_bytes[..mlvo_bytes.len()] == mlvo_bytes {
            break;
        }
        mlvo += 1;
    }
    if mlvo as u32 >= _MLVO_NUM_ENTRIES {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column,
            ident.as_str());
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if is_mlvo_mask_defined(m, mlvo) {
        let loc_0: scanner_loc = s.token_location();
        let mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc_0.line,
            loc_0.column,
            mlvo_str);
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if mlvo_bytes.len() < ident_bytes.len() {
        let mut idx: u32 = 0;
        let remaining = &ident_bytes[mlvo_bytes.len()..];
        let consumed: i32 = extract_mapping_layout_index(remaining, &mut idx);
        if remaining.len() as i32 != consumed {
            let loc_1: scanner_loc = s.token_location();
            let mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" may only be followed by a valid group index; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_1.line,
                loc_1.column,
                mlvo_str);
            m.mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        if mlvo as u32 == MLVO_LAYOUT {
            if let LayoutIdx::Single {
                ref mut layout_idx, ..
            } = m.mapping.layout
            {
                *layout_idx = idx;
            }
        } else if mlvo as u32 == MLVO_VARIANT {
            if let LayoutIdx::Single {
                ref mut variant_idx,
                ..
            } = m.mapping.layout
            {
                *variant_idx = idx;
            }
        } else {
            let loc_2: scanner_loc = s.token_location();
            let mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" cannot be followed by a group index; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_2.line,
                loc_2.column,
                mlvo_str);
            m.mapping.active_or_candidates_mask = 0_u32;
            return;
        }
    } else if mlvo as u32 == MLVO_LAYOUT {
        if let LayoutIdx::Single {
            ref mut layout_idx, ..
        } = m.mapping.layout
        {
            *layout_idx = LAYOUT_INDEX_SINGLE;
        }
    } else if mlvo as u32 == MLVO_VARIANT {
        if let LayoutIdx::Single {
            ref mut variant_idx,
            ..
        } = m.mapping.layout
        {
            *variant_idx = LAYOUT_INDEX_SINGLE;
        }
    }
    if (mlvo as u32 == MLVO_LAYOUT && is_mlvo_mask_defined(m, MLVO_VARIANT) as i32 != 0
        || mlvo as u32 == MLVO_VARIANT && is_mlvo_mask_defined(m, MLVO_LAYOUT) as i32 != 0)
        && {
            if let LayoutIdx::Single {
                layout_idx,
                variant_idx,
            } = m.mapping.layout
            {
                layout_idx != variant_idx
            } else {
                false
            }
        }
    {
        let loc_3: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"layout\" index must be the same as the \"variant\" index\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc_3.line,
            loc_3.column);
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.mlvo_at_pos[m.mapping.num_mlvo as usize] = mlvo;
    m.mapping.defined_mlvo_mask = (m.mapping.defined_mlvo_mask as i32
        | (1_u32 as mlvo_mask_t as i32) << mlvo as u32)
        as mlvo_mask_t;
    m.mapping.num_mlvo = m.mapping.num_mlvo.wrapping_add(1);
}
fn matcher_mapping_set_layout_bounds(m: &mut matcher) {
    let mut idx: u32 = if let LayoutIdx::Single {
        layout_idx,
        variant_idx,
    } = m.mapping.layout
    {
        if layout_idx < variant_idx {
            layout_idx
        } else {
            variant_idx
        }
    } else {
        0 // should not happen, layout is Single at this point
    };
    let mut is_index_case: bool = false;
    match idx {
        XKB_LAYOUT_INVALID => {
            m.mapping.layout = LayoutIdx::Index {
                layout_idx_min: XKB_LAYOUT_INVALID,
                layout_idx_max: XKB_LAYOUT_INVALID,
            };
            m.mapping.active_or_candidates_mask = 0x1_u32;
        }
        4294967293 => {
            let layout_idx_max = (if (32) < m.rmlvo.layouts.len() {
                32
            } else {
                m.rmlvo.layouts.len()
            }) as u32;
            m.mapping.layout = LayoutIdx::Range {
                layout_idx_min: 1_u32,
                layout_idx_max,
            };
            m.mapping.active_or_candidates_mask =
                ((1_u64 << layout_idx_max).wrapping_sub(1_u64) as u32 as u64 & !1_u64) as u32;
        }
        4294967294 => {
            let layout_idx_max = (if (32) < m.rmlvo.layouts.len() {
                32
            } else {
                m.rmlvo.layouts.len()
            }) as u32;
            m.mapping.layout = LayoutIdx::Range {
                layout_idx_min: 0_u32,
                layout_idx_max,
            };
            m.mapping.active_or_candidates_mask =
                ((1_u64 << layout_idx_max) as u32 as u64).wrapping_sub(1_u64) as u32;
        }
        4294967291 | 4294967292 => {
            idx = 0_u32;
            is_index_case = true;
        }
        _ => {
            is_index_case = true;
        }
    }
    if is_index_case {
        m.mapping.layout = LayoutIdx::Index {
            layout_idx_min: idx,
            layout_idx_max: idx.wrapping_add(1_u32),
        };
        m.mapping.active_or_candidates_mask = 1_u32 << idx;
    };
}
fn matcher_mapping_set_kccgst(m: &mut matcher, s: &mut scanner, ident: sval) {
    let ident_bytes = ident.as_bytes();
    let mut kccgst: rules_kccgst = KCCGST_KEYCODES;
    let mut kccgst_bytes: &[u8] = b"";
    while (kccgst as u32) < _KCCGST_NUM_ENTRIES {
        kccgst_bytes = RULES_KCCGST_SVALS[kccgst as usize];
        if kccgst_bytes == ident_bytes {
            break;
        }
        kccgst += 1;
    }
    if kccgst as u32 >= _KCCGST_NUM_ENTRIES {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column,
            ident.as_str());
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if m.mapping.defined_kccgst_mask as u32 & 1_u32 << kccgst as u32 != 0 {
        let loc_0: scanner_loc = s.token_location();
        let kccgst_str = std::str::from_utf8(kccgst_bytes).unwrap_or("");
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc_0.line,
            loc_0.column,
            kccgst_str);
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.kccgst_at_pos[m.mapping.num_kccgst as usize] = kccgst;
    m.mapping.defined_kccgst_mask =
        (m.mapping.defined_kccgst_mask as i32 | (1_u32 as u8 as i32) << kccgst as u32) as u8;
    m.mapping.num_kccgst = m.mapping.num_kccgst.wrapping_add(1);
}
fn fn_layout_or_variant_valid(rmlvo_len: usize, idx: u32) -> bool {
    match idx {
        4294967291 => rmlvo_len <= 1,
        4294967292..=4294967294 => true,
        _ => rmlvo_len >= 2 && (idx as usize) < rmlvo_len,
    }
}

fn matcher_mapping_verify(m: &mut matcher, s: &mut scanner) -> bool {
    if m.mapping.num_mlvo as i32 == 0_i32 {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the left hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc.line,
                loc.column);
    } else if m.mapping.num_kccgst as i32 == 0_i32 {
        let loc_0: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the right hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_0.line,
                loc_0.column);
    } else {
        if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
            let single_layout_idx = if let LayoutIdx::Single { layout_idx, .. } = m.mapping.layout {
                layout_idx
            } else {
                0
            };
            if !fn_layout_or_variant_valid(m.rmlvo.layouts.len(), single_layout_idx) {
                m.mapping.active_or_candidates_mask = 0_u32;
                return false;
            }
        }
        if is_mlvo_mask_defined(m, MLVO_VARIANT) {
            let single_variant_idx = if let LayoutIdx::Single { variant_idx, .. } = m.mapping.layout
            {
                variant_idx
            } else {
                0
            };
            if !fn_layout_or_variant_valid(m.rmlvo.variants.len(), single_variant_idx) {
                m.mapping.active_or_candidates_mask = 0_u32;
                return false;
            }
        }
        return true;
    }
    m.mapping.active_or_candidates_mask = 0_u32;
    false
}
fn matcher_rule_start_new(m: &mut matcher) {
    m.rule = rule::default();
    m.rule.skip = m.mapping.active_or_candidates_mask == 0;
}
fn matcher_rule_set_mlvo_common(
    m: &mut matcher,
    s: &mut scanner,
    ident: SvalIdx,
    match_type: mlvo_match_type,
) {
    if m.rule.num_mlvo_values as i32 >= m.mapping.num_mlvo as i32 {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column);
        m.rule.skip = true;
        return;
    }
    m.rule.match_type_at_pos[m.rule.num_mlvo_values as usize] = match_type;
    m.rule.mlvo_value_at_pos[m.rule.num_mlvo_values as usize] = ident;
    m.rule.num_mlvo_values = m.rule.num_mlvo_values.wrapping_add(1);
}
fn matcher_rule_set_mlvo_wildcard(m: &mut matcher, s: &mut scanner, match_type: mlvo_match_type) {
    let dummy = SvalIdx::EMPTY;
    matcher_rule_set_mlvo_common(m, s, dummy, match_type);
}
fn matcher_rule_set_mlvo_group(m: &mut matcher, s: &mut scanner, ident: SvalIdx) {
    matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
}
fn matcher_rule_set_mlvo(m: &mut matcher, s: &mut scanner, ident: SvalIdx) {
    matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
}
fn matcher_rule_set_kccgst(m: &mut matcher, s: &mut scanner, ident: SvalIdx) {
    if m.rule.num_kccgst_values as i32 >= m.mapping.num_kccgst as i32 {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column);
        m.rule.skip = true;
        return;
    }
    m.rule.kccgst_value_at_pos[m.rule.num_kccgst_values as usize] = ident;
    m.rule.num_kccgst_values = m.rule.num_kccgst_values.wrapping_add(1);
}
fn match_group(groups: &[group], group_name: sval, to: sval) -> bool {
    let found_group = groups.iter().find(|g| g.name.as_slice() == group_name.data);
    match found_group {
        None => false,
        Some(group) => {
            for elem in group.elements.iter() {
                if elem.as_slice() == to.data {
                    return true;
                }
            }
            false
        }
    }
}
fn match_value(
    groups: &[group],
    val: sval,
    to: sval,
    match_type: mlvo_match_type,
    wildcard_type: wildcard_match_type,
) -> bool {
    match match_type {
        1 => wildcard_type == WILDCARD_MATCH_ALL || !to.is_empty(),
        2 => to.is_empty(),
        3 => !to.is_empty(),
        4 => true,
        5 => match_group(groups, val, to),
        _ => svaleq(val, to),
    }
}
fn match_value_and_mark(
    groups: &[group],
    val: sval,
    to: &mut matched_sval,
    match_type: mlvo_match_type,
    wildcard_type: wildcard_match_type,
) -> bool {
    let matched: bool = match_value(groups, val, to.sval, match_type, wildcard_type);
    if matched {
        to.matched = true;
    }
    matched
}
fn expand_rmlvo_in_kccgst_value(
    m: &mut matcher,
    s: &mut scanner,
    value: sval,
    layout_idx: u32,
    expanded: &mut Vec<i8>,
    i: &mut usize,
) -> bool {
    let bytes = value.as_bytes();
    // Handle %i expansion
    if bytes[*i] == b'i'
        && ((*i).wrapping_add(1_usize) == value.len()
            || (bytes[(*i).wrapping_add(1_usize)] as i32 == MERGE_OVERRIDE_PREFIX
                || bytes[(*i).wrapping_add(1_usize)] as i32 == MERGE_AUGMENT_PREFIX
                || bytes[(*i).wrapping_add(1_usize)] as i32 == MERGE_REPLACE_PREFIX))
    {
        if layout_idx == XKB_LAYOUT_INVALID {
            let loc: scanner_loc = s.token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: Invalid %i in %-expansion: there is no corresponding layout nor variant in the MLVO fields of the rules header.\n",
                XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION as i32,
                &s.file_name,
                loc.line,
                loc.column);
        } else {
            *i = (*i).wrapping_add(1);
            let idx_str = format!("{}", layout_idx.wrapping_add(1_u32));
            vec_append_nul_terminated(expanded, idx_str.as_bytes());
            return true;
        }
    } else {
        let mut sfx: i8 = 0;
        let mut pfx: i8 = 0;
        let ch = bytes[*i];
        if ch == b'('
            || ch as i32 == MERGE_OVERRIDE_PREFIX
            || ch as i32 == MERGE_AUGMENT_PREFIX
            || ch as i32 == MERGE_REPLACE_PREFIX
            || ch == b'_'
            || ch == b'-'
        {
            pfx = ch as i8;
            if ch == b'(' {
                sfx = b')' as i8;
            }
            *i = (*i).wrapping_add(1);
            if *i >= value.len() {
                // fall through to error
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
        }

        let c2rust_fresh7 = *i;
        *i = (*i).wrapping_add(1);
        let mlv: rules_mlvo = match bytes[c2rust_fresh7] {
            b'm' => MLVO_MODEL,
            b'l' => MLVO_LAYOUT,
            b'v' => MLVO_VARIANT,
            _ => {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
        };

        let mut idx: u32 = XKB_LAYOUT_INVALID;
        let mut expanded_index: bool = false;
        if *i < value.len() && bytes[*i] == b'[' {
            if mlv as u32 != MLVO_LAYOUT && mlv as u32 != MLVO_VARIANT {
                let loc_0: scanner_loc = s.token_location();
                log::error!("[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_0.line,
                    loc_0.column);
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
            let consumed: i32 = extract_layout_index(&bytes[*i..], &mut idx);
            if consumed == -1_i32 {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
            if idx == XKB_LAYOUT_INVALID {
                idx = layout_idx;
                expanded_index = true;
            }
            *i = (*i).wrapping_add(consumed as usize);
        }

        if sfx as i32 != 0_i32 {
            if *i >= value.len() {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
            let c2rust_fresh8 = *i;
            *i = (*i).wrapping_add(1);
            if bytes[c2rust_fresh8] as i8 != sfx {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
        }

        // Determine which expanded_value to use. Copy the sval (it's Copy).
        // Then set matched afterward.
        enum RmlvoRef {
            Model,
            Layout(usize),
            Variant(usize),
        }
        let ev_ref: Option<RmlvoRef>;

        if mlv as u32 == MLVO_LAYOUT {
            if idx == XKB_LAYOUT_INVALID {
                if m.rmlvo.layouts.len() == 1 {
                    ev_ref = Some(RmlvoRef::Layout(0));
                } else {
                    ev_ref = None;
                }
            } else if idx < m.rmlvo.layouts.len() as u32
                && (expanded_index || m.rmlvo.layouts.len() > 1)
            {
                ev_ref = Some(RmlvoRef::Layout(idx as usize));
            } else {
                ev_ref = None;
            }
        } else if mlv as u32 == MLVO_VARIANT {
            if idx == XKB_LAYOUT_INVALID {
                if m.rmlvo.variants.len() == 1 {
                    ev_ref = Some(RmlvoRef::Variant(0));
                } else {
                    ev_ref = None;
                }
            } else if idx < m.rmlvo.variants.len() as u32
                && (expanded_index || m.rmlvo.variants.len() > 1)
            {
                ev_ref = Some(RmlvoRef::Variant(idx as usize));
            } else {
                ev_ref = None;
            }
        } else if mlv as u32 == MLVO_MODEL {
            ev_ref = Some(RmlvoRef::Model);
        } else {
            ev_ref = None;
        }

        // Get the sval bytes (sval is Copy)
        let ev_sval = match &ev_ref {
            Some(RmlvoRef::Model) => m.rmlvo.model.sval,
            Some(RmlvoRef::Layout(idx)) => m.rmlvo.layouts[*idx].sval,
            Some(RmlvoRef::Variant(idx)) => m.rmlvo.variants[*idx].sval,
            None => sval::EMPTY,
        };

        if ev_ref.is_none() || ev_sval.is_empty() {
            return true;
        }

        if pfx as i32 != 0_i32 {
            vec_append_nul_terminated(expanded, &[pfx as u8]);
        }
        vec_append_nul_terminated(expanded, ev_sval.as_bytes());
        if sfx as i32 != 0_i32 {
            vec_append_nul_terminated(expanded, &[sfx as u8]);
        }

        // Set matched flag
        match ev_ref {
            Some(RmlvoRef::Model) => m.rmlvo.model.matched = true,
            Some(RmlvoRef::Layout(idx)) => m.rmlvo.layouts[idx].matched = true,
            Some(RmlvoRef::Variant(idx)) => m.rmlvo.variants[idx].matched = true,
            None => {}
        }
        return true;
    }

    let loc_1: scanner_loc = s.token_location();
    log::error!(
        "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
        &s.file_name,
        loc_1.line,
        loc_1.column
    );
    false
}
fn expand_qualifier_in_kccgst_value(
    m: &mut matcher,
    s: &mut scanner,
    value: sval,
    expanded: &mut Vec<i8>,
    has_layout_idx_range: bool,
    has_separator: bool,
    prefix_idx: u32,
    i: &mut usize,
) {
    let bytes = value.as_bytes();
    if (*i).wrapping_add(3_usize) <= value.len()
        && ((*i).wrapping_add(3_usize) == value.len()
            || bytes[(*i).wrapping_add(3_usize)] as i32 == MERGE_OVERRIDE_PREFIX
            || bytes[(*i).wrapping_add(3_usize)] as i32 == MERGE_AUGMENT_PREFIX
            || bytes[(*i).wrapping_add(3_usize)] as i32 == MERGE_REPLACE_PREFIX)
        && bytes[*i] == b'a'
        && bytes[(*i).wrapping_add(1_usize)] == b'l'
        && bytes[(*i).wrapping_add(2_usize)] == b'l'
    {
        if has_layout_idx_range {
            let loc: scanner_loc = s.token_location();
            log::warn!(
                "{}:{}:{}: Using :all qualifier with indices range is not recommended.\n",
                &s.file_name,
                loc.line,
                loc.column
            );
        }
        vec_append_nul_terminated(expanded, b"1");
        if m.rmlvo.layouts.len() > 1 {
            let prefix_length = expanded
                .len()
                .wrapping_sub(prefix_idx as usize)
                .wrapping_sub(1);
            let mut l: u32 = 1_u32;
            while l
                < (if 32 < m.rmlvo.layouts.len() {
                    32_u32
                } else {
                    m.rmlvo.layouts.len() as u32
                })
            {
                if !has_separator {
                    expanded.push('+' as i32 as i8);
                }
                {
                    let old_size = expanded.len();
                    let new_size = old_size.wrapping_add(prefix_length).wrapping_add(1);
                    expanded.resize(new_size, 0);
                    expanded.copy_within(
                        prefix_idx as usize..prefix_idx as usize + prefix_length,
                        old_size,
                    );
                    expanded.truncate(new_size.wrapping_sub(1));
                }
                let idx_str = format!("{}", l.wrapping_add(1_u32));
                vec_append_nul_terminated(expanded, idx_str.as_bytes());
                l = l.wrapping_add(1);
            }
        }
        *i = (*i).wrapping_add(3_usize);
    }
}
#[inline]
fn concat_kccgst(into: &mut Vec<i8>, from: &[i8]) {
    let from_plus = !from.is_empty()
        && (from[0] as i32 == MERGE_OVERRIDE_PREFIX
            || from[0] as i32 == MERGE_AUGMENT_PREFIX
            || from[0] as i32 == MERGE_REPLACE_PREFIX);
    if from_plus || into.is_empty() {
        into.extend_from_slice(from);
    } else {
        let ch = if into.is_empty() { 0i8 } else { into[0] };
        let into_plus = ch as i32 == MERGE_OVERRIDE_PREFIX
            || ch as i32 == MERGE_AUGMENT_PREFIX
            || ch as i32 == MERGE_REPLACE_PREFIX;
        if into_plus {
            let old_len = into.len();
            into.resize(old_len + from.len(), 0);
            into.copy_within(..old_len, from.len());
            for (i, &b) in from.iter().enumerate() {
                into[i] = b;
            }
        }
    }
}
fn expand_kccgst_value(
    m: &mut matcher,
    s: &mut scanner,
    value: sval,
    layout_idx: u32,
) -> Option<Vec<i8>> {
    let bytes = value.as_bytes();
    let mut expanded: Vec<i8> = Vec::new();
    let mut last_item_idx: u32 = 0;
    let mut has_separator: bool = false;
    let mut invalid = false;
    let mut i: usize = 0_usize;
    loop {
        if i >= value.len() {
            break;
        }
        match bytes[i] {
            b':' => {
                let c2rust_fresh4 = i;
                i = i.wrapping_add(1);
                vec_append_nul_terminated(&mut expanded, &[bytes[c2rust_fresh4]]);
                expand_qualifier_in_kccgst_value(
                    m,
                    s,
                    value,
                    &mut expanded,
                    matches!(m.mapping.layout, LayoutIdx::Range { .. }),
                    has_separator,
                    last_item_idx,
                    &mut i,
                );
            }
            b'%' => {
                i = i.wrapping_add(1);
                if i >= value.len()
                    || !expand_rmlvo_in_kccgst_value(m, s, value, layout_idx, &mut expanded, &mut i)
                {
                    invalid = true;
                    break;
                }
            }
            b if b as i32 == MERGE_OVERRIDE_PREFIX
                || b as i32 == MERGE_AUGMENT_PREFIX
                || b as i32 == MERGE_REPLACE_PREFIX =>
            {
                let c2rust_fresh5 = i;
                i = i.wrapping_add(1);
                vec_append_nul_terminated(&mut expanded, &[bytes[c2rust_fresh5]]);
                last_item_idx = expanded.len().wrapping_sub(1) as u32;
                has_separator = true;
            }
            _ => {
                let c2rust_fresh6 = i;
                i = i.wrapping_add(1);
                vec_append_nul_terminated(&mut expanded, &[bytes[c2rust_fresh6]]);
            }
        }
    }
    if invalid {
        None
    } else {
        Some(expanded)
    }
}
fn matcher_append_pending_kccgst(m: &mut matcher) -> bool {
    if !matches!(m.mapping.layout, LayoutIdx::Range { .. }) {
        return true;
    }
    let (range_min, range_max) = if let LayoutIdx::Range {
        layout_idx_min,
        layout_idx_max,
    } = m.mapping.layout
    {
        (layout_idx_min, layout_idx_max)
    } else {
        unreachable!()
    };
    let mut i: kccgst_index_t = 0 as kccgst_index_t;
    while (i as i32) < m.mapping.num_kccgst as i32 {
        let kccgst: rules_kccgst = m.mapping.kccgst_at_pos[i as usize];
        let mut layout: u32 = range_min;
        while layout < range_max {
            let mut offset: usize = 0_usize;
            let mut k: u32 = 0;
            while k < m.pending_kccgst.slices.len() as u32 {
                let slice_len = m.pending_kccgst.slices[k as usize].length;
                let slice_kccgst = m.pending_kccgst.slices[k as usize].kccgst;
                let slice_layout = m.pending_kccgst.slices[k as usize].layout;
                if slice_kccgst == kccgst as u32 && slice_layout == layout && slice_len as i32 != 0
                {
                    let from: Vec<i8> =
                        m.pending_kccgst.buffer[offset..offset + slice_len as usize].to_vec();
                    concat_kccgst(&mut m.kccgst[kccgst as usize], &from);
                }
                offset = offset.wrapping_add(slice_len as usize);
                k = k.wrapping_add(1);
            }
            layout = layout.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    m.mapping.layout = LayoutIdx::default();
    true
}
fn matcher_rule_verify(m: &mut matcher, s: &mut scanner) {
    if m.rule.num_mlvo_values as i32 != m.mapping.num_mlvo as i32
        || m.rule.num_kccgst_values as i32 != m.mapping.num_kccgst as i32
    {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: must have same number of values as mapping line; ignoring rule\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column);
        m.rule.skip = true;
    }
}
fn matcher_rule_apply_if_matches(m: &mut matcher, s: &mut scanner) {
    let mut candidate_layouts: u32 = m.mapping.active_or_candidates_mask;
    let mut idx: u32;
    let mut i: mlvo_index_t = 0 as mlvo_index_t;
    while (i as i32) < m.mapping.num_mlvo as i32 {
        let mlvo: rules_mlvo = m.mapping.mlvo_at_pos[i as usize];
        let value: sval = m.rule.mlvo_value_at_pos[i as usize].to_sval(s.s);
        let match_type: mlvo_match_type = m.rule.match_type_at_pos[i as usize];
        let mut matched: bool = false;
        if mlvo as u32 == MLVO_MODEL {
            matched = match_value_and_mark(
                &m.groups,
                value,
                &mut m.rmlvo.model,
                match_type,
                WILDCARD_MATCH_ALL,
            );
        } else if let LayoutIdx::Range {
            layout_idx_min,
            layout_idx_max,
        } = m.mapping.layout
        {
            idx = layout_idx_min;
            while idx < layout_idx_max && candidate_layouts != 0 {
                let mask: u32 = 1_u32 << idx;
                if candidate_layouts & mask != 0 {
                    match mlvo as u32 {
                        1 => {
                            if match_value_and_mark(
                                &m.groups,
                                value,
                                &mut m.rmlvo.layouts[idx as usize],
                                match_type,
                                WILDCARD_MATCH_NONEMPTY,
                            ) {
                                matched = true;
                            } else {
                                candidate_layouts &= !mask;
                            }
                        }
                        2 => {
                            if match_value_and_mark(
                                &m.groups,
                                value,
                                &mut m.rmlvo.variants[idx as usize],
                                match_type,
                                WILDCARD_MATCH_NONEMPTY,
                            ) {
                                matched = true;
                            } else {
                                candidate_layouts &= !mask;
                            }
                        }
                        _ => {
                            let mut found_option: bool = false;
                            for opt_idx in 0..m.rmlvo.options.len() {
                                let opt = &m.rmlvo.options[opt_idx];
                                if opt.layout as i32 != OPTIONS_MATCH_ALL_GROUPS
                                    && opt.layout != idx
                                {
                                    continue;
                                }
                                if match_value_and_mark(
                                    &m.groups,
                                    value,
                                    &mut m.rmlvo.options[opt_idx],
                                    match_type,
                                    WILDCARD_MATCH_ALL,
                                ) {
                                    matched = true;
                                    found_option = true;
                                    break;
                                }
                            }
                            if !found_option {
                                candidate_layouts &= !mask;
                            }
                        }
                    }
                }
                idx = idx.wrapping_add(1);
            }
        } else {
            let li = m.mapping.layout.layout_idx_min() as usize;
            match mlvo as u32 {
                1 => {
                    matched = match_value_and_mark(
                        &m.groups,
                        value,
                        &mut m.rmlvo.layouts[li],
                        match_type,
                        WILDCARD_MATCH_NONEMPTY,
                    );
                }
                2 => {
                    matched = match_value_and_mark(
                        &m.groups,
                        value,
                        &mut m.rmlvo.variants[li],
                        match_type,
                        WILDCARD_MATCH_NONEMPTY,
                    );
                }
                _ => {
                    let layout_min = m.mapping.layout.layout_idx_min();
                    for opt_idx in 0..m.rmlvo.options.len() {
                        let opt = &m.rmlvo.options[opt_idx];
                        if opt.layout as i32 != OPTIONS_MATCH_ALL_GROUPS && opt.layout != layout_min
                        {
                            continue;
                        }
                        matched = match_value_and_mark(
                            &m.groups,
                            value,
                            &mut m.rmlvo.options[opt_idx],
                            match_type,
                            WILDCARD_MATCH_ALL,
                        );
                        if matched {
                            break;
                        }
                    }
                }
            }
        }
        if !matched {
            return;
        }
        i = i.wrapping_add(1);
    }
    if let LayoutIdx::Range {
        layout_idx_min,
        layout_idx_max,
    } = m.mapping.layout
    {
        idx = layout_idx_min;
        while idx < layout_idx_max {
            if candidate_layouts & 1_u32 << idx != 0 {
                let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
                while (i_0 as i32) < m.mapping.num_kccgst as i32 {
                    let kccgst: rules_kccgst = m.mapping.kccgst_at_pos[i_0 as usize];
                    let value_0: sval = m.rule.kccgst_value_at_pos[i_0 as usize].to_sval(s.s);
                    let prev_buffer_length: u32 = m.pending_kccgst.buffer.len() as u32;
                    if let Some(expanded) = expand_kccgst_value(m, s, value_0, idx) {
                        if !expanded.is_empty() {
                            m.pending_kccgst.buffer.extend_from_slice(&expanded);
                        }
                        let length: u32 =
                            (m.pending_kccgst.buffer.len() as u32).wrapping_sub(prev_buffer_length);
                        let slice = kccgst_buffer_slice {
                            length,
                            kccgst,
                            layout: idx,
                        };
                        m.pending_kccgst.slices.push(slice);
                    }
                    i_0 = i_0.wrapping_add(1);
                }
            }
            idx = idx.wrapping_add(1);
        }
    } else if let LayoutIdx::Index { layout_idx_min, .. } = m.mapping.layout {
        let mut i_1: kccgst_index_t = 0 as kccgst_index_t;
        while (i_1 as i32) < m.mapping.num_kccgst as i32 {
            let kccgst_0: rules_kccgst = m.mapping.kccgst_at_pos[i_1 as usize];
            let value_1: sval = m.rule.kccgst_value_at_pos[i_1 as usize].to_sval(s.s);
            if let Some(expanded) = expand_kccgst_value(m, s, value_1, layout_idx_min) {
                if !expanded.is_empty() {
                    concat_kccgst(&mut m.kccgst[kccgst_0 as usize], &expanded);
                }
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
    if !is_mlvo_mask_defined(m, MLVO_OPTION) {
        m.mapping.active_or_candidates_mask &= !candidate_layouts;
    }
}
fn gettok(m: &mut matcher, s: &mut scanner) -> rules_token {
    lex(s, &mut m.val)
}
fn matcher_match(m: &mut matcher, s: &mut scanner, include_depth: u32, _file_name: &str) -> bool {
    let mut eof_ok = false;
    let mut tok: rules_token;

    '_initial: loop {
        tok = gettok(m, s);
        match tok as u32 {
            4 => {}
            1 => {
                continue;
            }
            0 => {
                eof_ok = true;
                break;
            }
            _ => {
                break;
            }
        }
        loop {
            tok = gettok(m, s);
            match tok as u32 {
                3 => {
                    matcher_group_start_new(m, m.val.string.to_sval(s.s).data);
                    tok = gettok(m, s);
                    match tok as u32 {
                        5 => {
                            break;
                        }
                        _ => {
                            break '_initial;
                        }
                    }
                }
                10 => {
                    tok = gettok(m, s);
                    match tok as u32 {
                        2 => {}
                        _ => {
                            break '_initial;
                        }
                    }
                    matcher_include(m, s, include_depth, m.val.string.to_sval(s.s));
                    tok = gettok(m, s);
                    match tok as u32 {
                        1 => {
                            continue '_initial;
                        }
                        _ => {
                            break '_initial;
                        }
                    }
                }
                2 => {
                    matcher_mapping_start_new(m);
                    matcher_mapping_set_mlvo(m, s, m.val.string.to_sval(s.s));
                    loop {
                        tok = gettok(m, s);
                        match tok as u32 {
                            2 => {}
                            5 => {
                                break;
                            }
                            _ => {
                                break '_initial;
                            }
                        }
                        if m.mapping.active_or_candidates_mask != 0 {
                            matcher_mapping_set_mlvo(m, s, m.val.string.to_sval(s.s));
                        }
                    }
                    loop {
                        tok = gettok(m, s);
                        match tok as u32 {
                            2 => {}
                            1 => {
                                break;
                            }
                            _ => {
                                break '_initial;
                            }
                        }
                        if m.mapping.active_or_candidates_mask != 0 {
                            matcher_mapping_set_kccgst(m, s, m.val.string.to_sval(s.s));
                        }
                    }
                    if m.mapping.active_or_candidates_mask != 0
                        && matcher_mapping_verify(m, s) as i32 != 0
                    {
                        matcher_mapping_set_layout_bounds(m);
                        if matches!(m.mapping.layout, LayoutIdx::Range { .. }) {
                            m.pending_kccgst.buffer.clear();
                            m.pending_kccgst.slices.clear();
                        }
                    }
                    loop {
                        tok = gettok(m, s);
                        match tok as u32 {
                            4 => {
                                matcher_append_pending_kccgst(m);
                                break;
                            }
                            1 => {}
                            0 => {
                                matcher_append_pending_kccgst(m);
                                eof_ok = true;
                                break '_initial;
                            }
                            _ => {
                                matcher_rule_start_new(m);
                                loop {
                                    match tok as u32 {
                                        2 => {
                                            if !m.rule.skip {
                                                if m.val.string.len() == 1
                                                    && s.s[m.val.string.start] == b'+'
                                                {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_SOME,
                                                    );
                                                } else {
                                                    matcher_rule_set_mlvo(m, s, m.val.string);
                                                }
                                            }
                                        }
                                        6 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_LEGACY,
                                                );
                                            }
                                        }
                                        7 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_NONE,
                                                );
                                            }
                                        }
                                        8 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_SOME,
                                                );
                                            }
                                        }
                                        9 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_ANY,
                                                );
                                            }
                                        }
                                        3 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_group(m, s, m.val.string);
                                            }
                                        }
                                        5 => {
                                            break;
                                        }
                                        _ => {
                                            break '_initial;
                                        }
                                    }
                                    tok = gettok(m, s);
                                }
                                loop {
                                    tok = gettok(m, s);
                                    match tok as u32 {
                                        2 => {}
                                        1 => {
                                            break;
                                        }
                                        _ => {
                                            break '_initial;
                                        }
                                    }
                                    if !m.rule.skip {
                                        matcher_rule_set_kccgst(m, s, m.val.string);
                                    }
                                }
                                if !m.rule.skip {
                                    matcher_rule_verify(m, s);
                                }
                                if !m.rule.skip {
                                    matcher_rule_apply_if_matches(m, s);
                                }
                            }
                        }
                    }
                }
                _ => {
                    break '_initial;
                }
            }
        }
        loop {
            tok = gettok(m, s);
            match tok as u32 {
                2 => {}
                1 => {
                    break;
                }
                _ => {
                    break '_initial;
                }
            }
            matcher_group_add_element(m, s, m.val.string.to_sval(s.s).data);
        }
    }
    if eof_ok {
        true
    } else {
        match tok as u32 {
            11 => {}
            _ => {
                let loc: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: unexpected token\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc.line,
                    loc.column
                );
            }
        }
        false
    }
}
fn read_rules_file(
    matcher: &mut matcher<'_>,
    include_depth: u32,
    file_data: &[u8],
    path: &str,
) -> bool {
    #[allow(unused_assignments)]
    let mut scanner: scanner = scanner::new(&[], "");

    scanner = scanner::new(file_data, path);
    if !scanner.check_supported_char_encoding() {
        let loc: scanner_loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &scanner.file_name,
                loc.line,
                loc.column);
        let loc_0: scanner_loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &scanner.file_name,
                loc_0.line,
                loc_0.column);
        return false;
    }
    let ret: bool = matcher_match(matcher, &mut scanner, include_depth, path);
    ret
}
fn xkb_resolve_partial_rules(rules: &str, suffix: &str, matcher: &mut matcher<'_>) -> bool {
    let partial_rules = format!("{}{}", rules, suffix);
    if partial_rules.len() >= 60 {
        log::error!(
            "[XKB-{:03}] Cannot load XKB rules \"{}{}\"\n",
            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
            rules,
            suffix
        );
        return false;
    }
    let mut offset: u32 = 0;
    loop {
        let found = FindFileInXkbPath(
            &mut *matcher.ctx,
            "(unknown)",
            &partial_rules,
            FILE_TYPE_RULES,
            &mut offset,
            false,
        );
        let Some((file_data, path)) = found else {
            break;
        };
        let ok: bool = read_rules_file(matcher, 0, &file_data, &path);
        drop(file_data);
        if !ok {
            log::error!(
                "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                &path
            );
            return false;
        }
        offset += 1;
    }
    true
}
fn xkb_resolve_rules(
    rules: &str,
    matcher: &mut matcher<'_>,
    out: &mut XkbComponentNames,
    explicit_layouts: &mut u32,
) -> bool {
    let mut ret: bool;
    let mut offset: u32 = 0;
    let rules_str = rules;
    let found = FindFileInXkbPath(
        &mut *matcher.ctx,
        "(unknown)",
        rules_str,
        FILE_TYPE_RULES,
        &mut offset,
        true,
    );
    let Some((file_data, path)) = found else {
        log::error!(
            "[XKB-{:03}] Cannot load XKB rules \"{}\"\n",
            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
            rules_str
        );
        return false;
    };
    ret = xkb_resolve_partial_rules(rules_str, ".pre", matcher);
    if ret {
        ret = read_rules_file(matcher, 0, &file_data, &path);
        if !ret {
            log::error!(
                "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                &path
            );
        } else {
            ret = xkb_resolve_partial_rules(rules_str, ".post", matcher);
            if ret {
                if matcher.kccgst[KCCGST_KEYCODES as usize].is_empty()
                    || matcher.kccgst[KCCGST_TYPES as usize].is_empty()
                    || matcher.kccgst[KCCGST_COMPAT as usize].is_empty()
                    || matcher.kccgst[KCCGST_SYMBOLS as usize].is_empty()
                {
                    log::error!(
                        "[XKB-{:03}] No components returned from XKB rules \"{}\"\n",
                        XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                        rules_str
                    );
                    ret = false;
                } else {
                    // Transfer ownership of Vec data directly.
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_KEYCODES as usize]);
                        v.push(0);
                        out.keycodes = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_TYPES as usize]);
                        v.push(0);
                        out.types = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_COMPAT as usize]);
                        v.push(0);
                        out.compatibility = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_SYMBOLS as usize]);
                        v.push(0);
                        out.symbols = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_GEOMETRY as usize]);
                        v.push(0);
                        out.geometry = v;
                    }
                    if !matcher.rmlvo.model.matched && !matcher.rmlvo.model.sval.is_empty() {
                        log::error!(
                            "[XKB-{:03}] Unrecognized RMLVO model \"{}\" was ignored\n",
                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                            matcher.rmlvo.model.sval.as_str()
                        );
                    }
                    for mval in matcher.rmlvo.layouts.iter() {
                        if !mval.matched && !mval.sval.is_empty() {
                            log::error!(
                                "[XKB-{:03}] Unrecognized RMLVO layout \"{}\" was ignored\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                mval.sval.as_str()
                            );
                        }
                    }
                    for mval in matcher.rmlvo.variants.iter() {
                        if !mval.matched && !mval.sval.is_empty() {
                            log::error!(
                                "[XKB-{:03}] Unrecognized RMLVO variant \"{}\" was ignored\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                mval.sval.as_str()
                            );
                        }
                    }
                    for mval in matcher.rmlvo.options.iter() {
                        if !mval.matched && !mval.sval.is_empty() {
                            log::error!(
                                "[XKB-{:03}] Unrecognized RMLVO option \"{}\" was ignored\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                mval.sval.as_str()
                            );
                        }
                    }
                    if !out.symbols.is_empty() {
                        *explicit_layouts = 1_u32;
                        // Parse symbols string to find explicit layout count
                        let sym_bytes: Vec<u8> = out.symbols.iter().map(|&b| b as u8).collect();
                        let mut pos: usize = 0;
                        loop {
                            match sym_bytes[pos..].iter().position(|&b| b == b':') {
                                None => break,
                                Some(colon_off) => {
                                    pos += colon_off + 1;
                                    if pos >= sym_bytes.len() || sym_bytes[pos] == 0 {
                                        break;
                                    }
                                    let (val_parsed, count) = parse_dec_u32(&sym_bytes[pos..]);
                                    let group: u32 = val_parsed;
                                    let count = count as usize;
                                    if count > 0
                                        && pos + count <= sym_bytes.len()
                                        && (sym_bytes.get(pos + count).copied() == Some(0)
                                            || sym_bytes.get(pos + count).map(|&b| b as i32)
                                                == Some(MERGE_OVERRIDE_PREFIX)
                                            || sym_bytes.get(pos + count).map(|&b| b as i32)
                                                == Some(MERGE_AUGMENT_PREFIX)
                                            || sym_bytes.get(pos + count).map(|&b| b as i32)
                                                == Some(MERGE_REPLACE_PREFIX))
                                        && group > 0
                                        && group <= XKB_MAX_GROUPS as u32
                                    {
                                        *explicit_layouts = (*explicit_layouts).max(group);
                                        pos += count;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ret
}
pub(crate) fn xkb_components_from_rules_names(
    ctx: &mut XkbContext,
    rmlvo: &XkbRuleNames,
    out: &mut XkbComponentNames,
    explicit_layouts: &mut u32,
) -> bool {
    let mut matcher = matcher_new_from_names(ctx, rmlvo);
    let rules_str = rmlvo.rules.to_str().unwrap_or("");
    let ret: bool = xkb_resolve_rules(rules_str, &mut matcher, out, explicit_layouts);
    drop(matcher);
    ret
}
