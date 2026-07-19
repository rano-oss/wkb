// Safe parser.rs
// LALR(1) parser for XKB, converted from bison-generated C via c2rust
#![allow(dead_code)]

use self::parser_h::*;
use super::super::context::xkb_atom_intern;
use super::super::keymap::xkb_escape_map_name;
use super::super::keysym::{xkb_keysym_from_name, xkb_keysym_is_deprecated};
use super::super::keysym_utf::utf32_to_keysym;
use super::super::shared_types::*;
use super::super::utils::{
    parse_dec_u64, parse_hex_u32, parse_hex_u64, utf8_next_code_point_safe, INVALID_UTF8_CODE_POINT,
};

use super::super::shared_ast_types::{
    merge_mode, safe_map_name, stmt_type, xkb_map_flags, ExprDef, ExprKind, GroupCompatDef,
    IncludeStmt, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef, ModMapDef,
    Statement, SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES,
    FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN,
    MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL,
    MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_EXPR_ADD, STMT_EXPR_ASSIGN,
    STMT_EXPR_DIVIDE, STMT_EXPR_INVERT, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT,
    STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION,
    _STMT_NUM_VALUES,
};

use super::messages::{
    XKB_ERROR_INVALID_NUMERIC_KEYSYM, XKB_ERROR_INVALID_XKB_SYNTAX,
    XKB_WARNING_DEPRECATED_KEYSYM_NAME, XKB_WARNING_MISSING_DEFAULT_SECTION,
    XKB_WARNING_NUMERIC_KEYSYM, XKB_WARNING_UNRECOGNIZED_KEYSYM,
};

pub use self::parser_h::{YYerror, END_OF_FILE, YYEMPTY, YYUNDEF};

pub const XKB_KEY_VoidSymbol: i32 = 0xffffff_i32;
pub const XKB_KEY_0: i32 = 0x30_i32;
pub const XKB_KEY_section: i32 = 0xa7_i32;
pub const XKB_KEYSYM_MIN: i32 = 0;

// ── YYSYMBOL constants ──────────────────────────────────────────────
pub const YYSYMBOL_MapName: i32 = 148;
pub const YYSYMBOL_OptMapName: i32 = 147;
pub const YYSYMBOL_String: i32 = 146;
pub const YYSYMBOL_Ident: i32 = 145;
pub const YYSYMBOL_KeyCode: i32 = 144;
pub const YYSYMBOL_Integer: i32 = 143;
pub const YYSYMBOL_Float: i32 = 142;
pub const YYSYMBOL_Number: i32 = 141;
pub const YYSYMBOL_SignedNumber: i32 = 140;
pub const YYSYMBOL_KeySymLit: i32 = 139;
pub const YYSYMBOL_KeySym: i32 = 138;
pub const YYSYMBOL_KeySyms: i32 = 137;
pub const YYSYMBOL_NonEmptyKeySyms: i32 = 136;
pub const YYSYMBOL_KeySymList: i32 = 135;
pub const YYSYMBOL_MultiKeySymList: i32 = 134;
pub const YYSYMBOL_Terminal: i32 = 133;
pub const YYSYMBOL_OptTerminal: i32 = 132;
pub const YYSYMBOL_Lhs: i32 = 131;
pub const YYSYMBOL_Action: i32 = 130;
pub const YYSYMBOL_Actions: i32 = 129;
pub const YYSYMBOL_NonEmptyActions: i32 = 128;
pub const YYSYMBOL_ActionList: i32 = 127;
pub const YYSYMBOL_MultiActionList: i32 = 126;
pub const YYSYMBOL_Term: i32 = 125;
pub const YYSYMBOL_Expr: i32 = 124;
pub const YYSYMBOL_ExprList: i32 = 123;
pub const YYSYMBOL_MergeMode: i32 = 122;
pub const YYSYMBOL_OptMergeMode: i32 = 121;
pub const YYSYMBOL_Element: i32 = 120;
pub const YYSYMBOL_FieldSpec: i32 = 119;
pub const YYSYMBOL_DoodadType: i32 = 118;
pub const YYSYMBOL_DoodadDecl: i32 = 117;
pub const YYSYMBOL_Coord: i32 = 116;
pub const YYSYMBOL_CoordList: i32 = 115;
pub const YYSYMBOL_OutlineInList: i32 = 114;
pub const YYSYMBOL_OutlineList: i32 = 113;
pub const YYSYMBOL_OverlayKey: i32 = 112;
pub const YYSYMBOL_OverlayKeyList: i32 = 111;
pub const YYSYMBOL_OverlayDecl: i32 = 110;
pub const YYSYMBOL_Key: i32 = 109;
pub const YYSYMBOL_Keys: i32 = 108;
pub const YYSYMBOL_RowBodyItem: i32 = 107;
pub const YYSYMBOL_RowBody: i32 = 106;
pub const YYSYMBOL_SectionBodyItem: i32 = 105;
pub const YYSYMBOL_SectionBody: i32 = 104;
pub const YYSYMBOL_SectionDecl: i32 = 103;
pub const YYSYMBOL_ShapeDecl: i32 = 102;
pub const YYSYMBOL_UnknownCompoundStatementDecl: i32 = 101;
pub const YYSYMBOL_UnknownDecl: i32 = 100;
pub const YYSYMBOL_LedNameDecl: i32 = 99;
pub const YYSYMBOL_LedMapDecl: i32 = 98;
pub const YYSYMBOL_KeyOrKeySym: i32 = 97;
pub const YYSYMBOL_KeyOrKeySymList: i32 = 96;
pub const YYSYMBOL_ModMapDecl: i32 = 95;
pub const YYSYMBOL_GroupCompatDecl: i32 = 94;
pub const YYSYMBOL_NoSymbolOrActionList: i32 = 93;
pub const YYSYMBOL_MultiKeySymOrActionList: i32 = 92;
pub const YYSYMBOL_SymbolsVarDecl: i32 = 91;
pub const YYSYMBOL_SymbolsBody: i32 = 90;
pub const YYSYMBOL_OptSymbolsBody: i32 = 89;
pub const YYSYMBOL_SymbolsDecl: i32 = 88;
pub const YYSYMBOL_KeyTypeDecl: i32 = 87;
pub const YYSYMBOL_VarDeclList: i32 = 86;
pub const YYSYMBOL_InterpretMatch: i32 = 85;
pub const YYSYMBOL_InterpretDecl: i32 = 84;
pub const YYSYMBOL_VModDef: i32 = 83;
pub const YYSYMBOL_VModDefList: i32 = 82;
pub const YYSYMBOL_VModDecl: i32 = 81;
pub const YYSYMBOL_KeyAliasDecl: i32 = 80;
pub const YYSYMBOL_KeyNameDecl: i32 = 79;
pub const YYSYMBOL_VarDecl: i32 = 78;
pub const YYSYMBOL_Decl: i32 = 77;
pub const YYSYMBOL_DeclList: i32 = 76;
pub const YYSYMBOL_Flag: i32 = 75;
pub const YYSYMBOL_Flags: i32 = 74;
pub const YYSYMBOL_OptFlags: i32 = 73;
pub const YYSYMBOL_FileType: i32 = 72;
pub const YYSYMBOL_XkbMapConfig: i32 = 71;
pub const YYSYMBOL_XkbMapConfigList: i32 = 70;
pub const YYSYMBOL_XkbCompositeType: i32 = 69;
pub const YYSYMBOL_XkbCompositeMap: i32 = 68;
pub const YYSYMBOL_XkbFile: i32 = 67;
pub const YYSYMBOL_YYACCEPT: i32 = 66;
pub const YYSYMBOL_ALTERNATE_GROUP: i32 = 65;
pub const YYSYMBOL_FUNCTION_KEYS: i32 = 64;
pub const YYSYMBOL_KEYPAD_KEYS: i32 = 63;
pub const YYSYMBOL_MODIFIER_KEYS: i32 = 62;
pub const YYSYMBOL_ALPHANUMERIC_KEYS: i32 = 61;
pub const YYSYMBOL_HIDDEN: i32 = 60;
pub const YYSYMBOL_DEFAULT: i32 = 59;
pub const YYSYMBOL_PARTIAL: i32 = 58;
pub const YYSYMBOL_KEYNAME: i32 = 57;
pub const YYSYMBOL_IDENT: i32 = 56;
pub const YYSYMBOL_FLOAT: i32 = 55;
pub const YYSYMBOL_INTEGER: i32 = 54;
pub const YYSYMBOL_DECIMAL_DIGIT: i32 = 53;
pub const YYSYMBOL_STRING: i32 = 52;
pub const YYSYMBOL_INVERT: i32 = 51;
pub const YYSYMBOL_EXCLAM: i32 = 50;
pub const YYSYMBOL_SEMI: i32 = 49;
pub const YYSYMBOL_COMMA: i32 = 48;
pub const YYSYMBOL_DOT: i32 = 47;
pub const YYSYMBOL_CBRACKET: i32 = 46;
pub const YYSYMBOL_OBRACKET: i32 = 45;
pub const YYSYMBOL_CPAREN: i32 = 44;
pub const YYSYMBOL_OPAREN: i32 = 43;
pub const YYSYMBOL_CBRACE: i32 = 42;
pub const YYSYMBOL_OBRACE: i32 = 41;
pub const YYSYMBOL_TIMES: i32 = 40;
pub const YYSYMBOL_DIVIDE: i32 = 39;
pub const YYSYMBOL_MINUS: i32 = 38;
pub const YYSYMBOL_PLUS: i32 = 37;
pub const YYSYMBOL_EQUALS: i32 = 36;
pub const YYSYMBOL_VIRTUAL: i32 = 35;
pub const YYSYMBOL_LOGO: i32 = 34;
pub const YYSYMBOL_SOLID: i32 = 33;
pub const YYSYMBOL_OUTLINE: i32 = 32;
pub const YYSYMBOL_TEXT: i32 = 31;
pub const YYSYMBOL_OVERLAY: i32 = 30;
pub const YYSYMBOL_SECTION: i32 = 29;
pub const YYSYMBOL_ROW: i32 = 28;
pub const YYSYMBOL_KEYS: i32 = 27;
pub const YYSYMBOL_SHAPE: i32 = 26;
pub const YYSYMBOL_INDICATOR: i32 = 25;
pub const YYSYMBOL_MODIFIER_MAP: i32 = 24;
pub const YYSYMBOL_GROUP: i32 = 23;
pub const YYSYMBOL_ALIAS: i32 = 22;
pub const YYSYMBOL_KEY: i32 = 21;
pub const YYSYMBOL_ACTION_TOK: i32 = 20;
pub const YYSYMBOL_INTERPRET: i32 = 19;
pub const YYSYMBOL_TYPE: i32 = 18;
pub const YYSYMBOL_VIRTUAL_MODS: i32 = 17;
pub const YYSYMBOL_ALTERNATE: i32 = 16;
pub const YYSYMBOL_REPLACE: i32 = 15;
pub const YYSYMBOL_AUGMENT: i32 = 14;
pub const YYSYMBOL_OVERRIDE: i32 = 13;
pub const YYSYMBOL_INCLUDE: i32 = 12;
pub const YYSYMBOL_XKB_LAYOUT: i32 = 11;
pub const YYSYMBOL_XKB_SEMANTICS: i32 = 10;
pub const YYSYMBOL_XKB_GEOMETRY: i32 = 9;
pub const YYSYMBOL_XKB_COMPATMAP: i32 = 8;
pub const YYSYMBOL_XKB_SYMBOLS: i32 = 7;
pub const YYSYMBOL_XKB_TYPES: i32 = 6;
pub const YYSYMBOL_XKB_KEYCODES: i32 = 5;
pub const YYSYMBOL_XKB_KEYMAP: i32 = 4;
pub const YYSYMBOL_ERROR_TOK: i32 = 3;
pub const YYSYMBOL_YYUNDEF: i32 = 2;
pub const YYSYMBOL_YYerror: i32 = 1;
pub const YYSYMBOL_YYEOF: i32 = 0;
pub const YYSYMBOL_YYEMPTY: i32 = -2;

pub const YYARGS_MAX: u32 = 5;
pub const YYFINAL: i32 = 16;
pub const YYLAST: i32 = 928;
pub const YYNTOKENS: i32 = 66;
pub const YYMAXUTOK: i32 = 257;
pub const YYINITDEPTH: usize = 200;
pub const YYMAXDEPTH: usize = 10000;
pub const YYPACT_NINF: i32 = -280;
pub const YYENOMEM: i32 = -2;

pub struct parser_param<'a> {
    pub ctx: &'a mut xkb_context,
    pub scanner: &'a mut scanner<'a>,
    pub rtrn: Option<Box<XkbFile>>,
    pub more_maps: bool,
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

pub type yytype_uint8 = u8;
pub type yytype_int8 = i8;

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
        return Some(XKB_KEY_NoSymbol as u32);
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
    if sym != XKB_KEY_NoSymbol as u32 {
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

pub fn _xkbcommon_parse<'a>(param: &mut parser_param<'a>) -> i32 {
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
    yyvs: &mut Vec<YYValue<'a>>,
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
            match stmt {
                YYValue::Stmt(s) => list.push(s),
                _ => {} // null/none → skip
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
            // Decl: OptMergeMode GroupCompatDecl
            let merge = yyvs[sp - 1].as_merge();
            if let YYValue::GroupCompat(mut gc) = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                gc.merge = merge;
                *yyval = YYValue::Stmt(Statement::GroupCompat(gc));
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
                prepended.push(ExprCreateKeySymList(XKB_KEY_NoSymbol as u32));
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
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::GroupCompat(GroupCompatCreate(num, expr));
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
            *yyval = YYValue::LedName(LedNameCreate(num, expr, false));
        }
        90 => {
            // LedNameDecl: VIRTUAL INDICATOR Integer EQUALS Expr SEMI
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::LedName(LedNameCreate(num, expr, true));
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
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"action"));
        }
        131 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"interpret"));
        }
        132 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"type"));
        }
        133 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"key"));
        }
        134 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"group"));
        }
        135 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"modifier_map"));
        }
        136 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"indicator"));
        }
        137 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"shape"));
        }
        138 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"row"));
        }
        139 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"section"));
        }
        140 => {
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"text"));
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
            let expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as u32);
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
            let expr = ExprCreateKeySymList(XKB_KEY_NoSymbol as u32);
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
            *yyval = YYValue::Expr(ExprCreateKeySymList(XKB_KEY_NoSymbol as u32));
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
            if keysym == XKB_KEY_NoSymbol as u32 {
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
                    *yyval = YYValue::Keysym(XKB_KEY_NoSymbol as u32);
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
                *yyval = YYValue::Keysym(XKB_KEY_NoSymbol as u32);
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
                    *yyval = YYValue::Keysym(XKB_KEY_NoSymbol as u32);
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
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, sval.as_bytes()));
        }
        215 => {
            // Ident: DEFAULT
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, b"default"));
        }
        // String 216
        216 => {
            // String: STRING → intern as atom
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Atom(xkb_atom_intern(param.ctx, s.as_bytes()));
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

pub fn parse<'a>(
    mut ctx: &'a mut xkb_context,
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

    if first.is_some() {
        let first_ref = first.as_ref().unwrap();
        log::debug!(
            "[XKB-{:03}] No map in include statement, but \"{}\" contains several; Using first defined map, \"{}\"\n",
            XKB_WARNING_MISSING_DEFAULT_SECTION as i32,
            &scanner.file_name,
            safe_map_name(first_ref)
        );
    }
    first
}

pub fn parse_next<'a>(
    ctx: &'a mut xkb_context,
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

pub fn expr_create(kind: ExprKind) -> Box<ExprDef> {
    Box::new(ExprDef { kind })
}

pub fn ExprCreateKeySymList(sym: u32) -> Box<ExprDef> {
    let mut syms = Vec::new();
    if sym != XKB_KEY_NoSymbol as u32 {
        syms.push(sym);
    }
    expr_create(ExprKind::KeysymList { syms })
}

pub fn ExprAppendKeySymList(mut expr: Box<ExprDef>, sym: u32) -> Box<ExprDef> {
    if sym != XKB_KEY_NoSymbol as u32 {
        if let ExprKind::KeysymList { ref mut syms } = expr.kind {
            syms.push(sym);
        }
    }
    expr
}

pub fn ExprKeySymListAppendString(
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
        if sym == XKB_KEY_NoSymbol as u32 {
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

pub fn KeysymParseString(scanner: &mut scanner, string: &str) -> u32 {
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
        return XKB_KEY_NoSymbol as u32;
    }
    let (cp, cp_len) = utf8_next_code_point_safe(bytes);
    if cp == INVALID_UTF8_CODE_POINT {
        let loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Invalid UTF-8 encoding.\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &scanner.file_name,
            loc.line,
            loc.column);
        return XKB_KEY_NoSymbol as u32;
    } else if cp_len != len {
        let loc = scanner.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: Cannot convert string to single keysym: Expected a single Unicode code point, got: \"{}\".\n",
            XKB_ERROR_INVALID_FILE_ENCODING as i32,
            &scanner.file_name,
            loc.line,
            loc.column,
            string);
        return XKB_KEY_NoSymbol as u32;
    }
    let sym = utf32_to_keysym(cp);
    if sym == XKB_KEY_NoSymbol as u32 {
        let loc = scanner.token_location();
        log::error!("{}:{}:{}: Cannot convert string to single keysym: Unicode code point U+{:04X} has no keysym equivalent.\n",
            &scanner.file_name,
            loc.line,
            loc.column,
            cp);
    }
    sym
}

pub fn KeycodeCreate(name: u32, value: i64) -> Box<KeycodeDef> {
    Box::new(KeycodeDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub fn KeyAliasCreate(alias: u32, real: u32) -> Box<KeyAliasDef> {
    Box::new(KeyAliasDef {
        merge: MERGE_DEFAULT,
        alias,
        real,
    })
}

pub fn VModCreate(name: u32, value: Option<Box<ExprDef>>) -> Box<VModDef> {
    Box::new(VModDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub fn VarCreate(name: Option<Box<ExprDef>>, value: Option<Box<ExprDef>>) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub fn BoolVarCreate(ident: u32, set: bool) -> Box<VarDef> {
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

pub fn InterpCreate(sym: u32, match_0: Option<Box<ExprDef>>) -> Box<InterpDef> {
    Box::new(InterpDef {
        merge: MERGE_DEFAULT,
        sym,
        match_0,
        def: Vec::new(),
    })
}

pub fn KeyTypeCreate(name: u32, body: Vec<VarDef>) -> Box<KeyTypeDef> {
    Box::new(KeyTypeDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub fn SymbolsCreate(keyName: u32, symbols: Vec<VarDef>) -> Box<SymbolsDef> {
    Box::new(SymbolsDef {
        merge: MERGE_DEFAULT,
        keyName,
        symbols,
    })
}

pub fn GroupCompatCreate(group: i64, val: Option<Box<ExprDef>>) -> Box<GroupCompatDef> {
    Box::new(GroupCompatDef {
        merge: MERGE_DEFAULT,
        group,
        def: val,
    })
}

pub fn ModMapCreate(modifier: u32, keys: Vec<ExprDef>) -> Box<ModMapDef> {
    Box::new(ModMapDef {
        merge: MERGE_DEFAULT,
        modifier,
        keys,
    })
}

pub fn LedMapCreate(name: u32, body: Vec<VarDef>) -> Box<LedMapDef> {
    Box::new(LedMapDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub fn LedNameCreate(ndx: i64, name: Option<Box<ExprDef>>, virtual_0: bool) -> Box<LedNameDef> {
    Box::new(LedNameDef {
        merge: MERGE_DEFAULT,
        ndx,
        virtual_0,
        name,
    })
}

pub fn UnknownStatementCreate(type_0: stmt_type, name: &str) -> Box<UnknownStatement> {
    Box::new(UnknownStatement {
        stmt_type: type_0,
        name: name.to_string(),
    })
}

pub fn IncludeCreate(
    _ctx: &mut xkb_context,
    stmt_str: &str,
    mut merge: merge_mode,
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

pub fn XkbFileCreate(
    type_0: u32,
    name: Option<String>,
    defs: Vec<Statement>,
    flags: xkb_map_flags,
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

pub fn XkbFileFromComponents(
    ctx: &mut xkb_context,
    kkctgs: &xkb_component_names,
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

pub fn xkb_file_type_to_string(type_0: u32) -> &'static str {
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

pub fn stmt_type_to_string(type_0: u32) -> &'static str {
    if type_0 >= _STMT_NUM_VALUES {
        return "unknown";
    }
    STMT_TYPE_STRINGS[type_0 as usize]
}

pub fn stmt_type_to_operator_char(type_0: u32) -> char {
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
pub struct sval<'a> {
    pub data: &'a [u8],
}

impl<'a> sval<'a> {
    pub const EMPTY: sval<'static> = sval { data: &[] };

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.data).unwrap_or("")
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn start_ptr(&self) -> *const i8 {
        self.data.as_ptr() as *const i8
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Copy, Clone)]
pub struct scanner_loc {
    pub line: usize,
    pub column: usize,
}

pub struct scanner<'a> {
    pub pos: usize,
    pub s: &'a [u8],
    pub buf: [u8; 1024],
    pub buf_pos: usize,
    pub token_pos: usize,
    pub cached_pos: usize,
    pub cached_loc: scanner_loc,
    pub file_name: String,
}

impl<'a> scanner<'a> {
    pub fn new(s: &'a [u8], file_name: &str) -> Self {
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
    pub fn len(&self) -> usize {
        self.s.len()
    }

    #[inline]
    fn remaining_bytes(&self) -> &[u8] {
        &self.s[self.pos..]
    }

    #[inline]
    pub fn peek(&self) -> i8 {
        if self.pos >= self.s.len() {
            return 0;
        }
        self.s[self.pos] as i8
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.pos >= self.s.len()
    }

    #[inline]
    pub fn eol(&self) -> bool {
        self.peek() == b'\n' as i8
    }

    #[inline]
    pub fn skip_to_eol(&mut self) {
        let rem = self.remaining_bytes();
        match rem.iter().position(|&b| b == b'\n') {
            Some(i) => self.pos += i,
            None => self.pos = self.s.len(),
        }
    }

    #[inline]
    pub fn next_byte(&mut self) -> i8 {
        if self.pos >= self.s.len() {
            return 0;
        }
        let c = self.s[self.pos] as i8;
        self.pos += 1;
        c
    }

    #[inline]
    pub fn chr(&mut self, ch: i8) -> bool {
        if self.peek() != ch {
            return false;
        }
        self.pos += 1;
        true
    }

    #[inline]
    pub fn str_match(&mut self, string: &[u8]) -> bool {
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
    pub fn buf_append(&mut self, ch: u8) -> bool {
        if self.buf_pos + 1 >= self.buf.len() {
            return false;
        }
        self.buf[self.buf_pos] = ch;
        self.buf_pos += 1;
        true
    }

    #[inline]
    pub fn buf_appends(&mut self, s: &[u8]) -> bool {
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

    pub fn buf_appends_str(&mut self, s: &str) -> bool {
        for &b in s.as_bytes() {
            if !self.buf_append(b) {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn buf_appends_code_point(&mut self, c: u32) -> bool {
        if self.buf_pos + 4 <= self.buf.len() {
            let mut count = utf8_h::utf32_to_utf8_safe(c, &mut self.buf[self.buf_pos..]);
            if count == 0 {
                count = utf8_h::utf32_to_utf8_safe(0xfffd, &mut self.buf[self.buf_pos..]);
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
    pub fn oct(&mut self, out: &mut u8) -> bool {
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
    pub fn dec_int64(&mut self, out: &mut i64) -> i32 {
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
    pub fn hex_int64(&mut self, out: &mut i64) -> i32 {
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
    pub fn unicode_code_point(&mut self, out: &mut u32) -> bool {
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
    pub fn check_supported_char_encoding(&mut self) -> bool {
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
    pub fn input_at(&self, pos: usize) -> *const i8 {
        self.s[pos..].as_ptr() as *const i8
    }

    #[inline]
    pub fn input_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.s[start..end]
    }

    pub fn token_location(&mut self) -> scanner_loc {
        let mut line: usize;
        let column: usize;
        let mut line_pos: usize = 0;

        if self.cached_pos > self.token_pos {
            self.cached_pos = 0;
            self.cached_loc.column = 1;
            self.cached_loc.line = 1;
        }

        line = self.cached_loc.line;
        let input = self.s;
        let start = self.cached_pos;
        let end = self.token_pos;

        let mut search_from = start;
        loop {
            match input[search_from..end].iter().position(|&b| b == b'\n') {
                Some(i) => {
                    line += 1;
                    search_from = search_from + i + 1;
                    line_pos = search_from;
                }
                None => break,
            }
        }

        if line == self.cached_loc.line {
            column = self.cached_loc.column + (self.token_pos - self.cached_pos);
        } else {
            column = self.token_pos - line_pos + 1;
        }

        let loc = scanner_loc { line, column };
        self.cached_pos = self.token_pos;
        self.cached_loc = loc;
        loc
    }
}

// ── sval comparison functions (migrated from scanner_utils.rs) ──

#[inline]
pub fn svaleq(s1: sval, s2: sval) -> bool {
    s1.data == s2.data
}

#[inline]
pub fn svaleq_prefix(s1: sval, s2: sval) -> bool {
    s1.data.len() <= s2.data.len() && s1.data == &s2.data[..s1.data.len()]
}

#[inline]
pub fn isvaleq(s1: sval, s2: sval) -> bool {
    s1.data.len() == s2.data.len() && s1.data.eq_ignore_ascii_case(s2.data)
}

pub mod parser_h {
    pub const ALTERNATE_GROUP: i32 = 77;
    pub const FUNCTION_KEYS: i32 = 76;
    pub const KEYPAD_KEYS: i32 = 75;
    pub const MODIFIER_KEYS: i32 = 74;
    pub const ALPHANUMERIC_KEYS: i32 = 73;
    pub const HIDDEN: i32 = 72;
    pub const DEFAULT: i32 = 71;
    pub const PARTIAL: i32 = 70;
    pub const KEYNAME: i32 = 65;
    pub const IDENT: i32 = 64;
    pub const FLOAT: i32 = 63;
    pub const INTEGER: i32 = 62;
    pub const DECIMAL_DIGIT: i32 = 61;
    pub const STRING: i32 = 60;
    pub const INVERT: i32 = 55;
    pub const EXCLAM: i32 = 54;
    pub const SEMI: i32 = 53;
    pub const COMMA: i32 = 52;
    pub const DOT: i32 = 51;
    pub const CBRACKET: i32 = 50;
    pub const OBRACKET: i32 = 49;
    pub const CPAREN: i32 = 48;
    pub const OPAREN: i32 = 47;
    pub const CBRACE: i32 = 46;
    pub const OBRACE: i32 = 45;
    pub const TIMES: i32 = 44;
    pub const DIVIDE: i32 = 43;
    pub const MINUS: i32 = 42;
    pub const PLUS: i32 = 41;
    pub const EQUALS: i32 = 40;
    pub const VIRTUAL: i32 = 38;
    pub const LOGO: i32 = 37;
    pub const SOLID: i32 = 36;
    pub const OUTLINE: i32 = 35;
    pub const TEXT: i32 = 34;
    pub const OVERLAY: i32 = 33;
    pub const SECTION: i32 = 32;
    pub const ROW: i32 = 31;
    pub const KEYS: i32 = 30;
    pub const SHAPE: i32 = 29;
    pub const INDICATOR: i32 = 28;
    pub const MODIFIER_MAP: i32 = 27;
    pub const GROUP: i32 = 26;
    pub const ALIAS: i32 = 25;
    pub const KEY: i32 = 24;
    pub const ACTION_TOK: i32 = 23;
    pub const INTERPRET: i32 = 22;
    pub const TYPE: i32 = 21;
    pub const VIRTUAL_MODS: i32 = 20;
    pub const ALTERNATE: i32 = 14;
    pub const REPLACE: i32 = 13;
    pub const AUGMENT: i32 = 12;
    pub const OVERRIDE: i32 = 11;
    pub const INCLUDE: i32 = 10;
    pub const XKB_LAYOUT: i32 = 8;
    pub const XKB_SEMANTICS: i32 = 7;
    pub const XKB_GEOMETRY: i32 = 6;
    pub const XKB_COMPATMAP: i32 = 5;
    pub const XKB_SYMBOLS: i32 = 4;
    pub const XKB_TYPES: i32 = 3;
    pub const XKB_KEYCODES: i32 = 2;
    pub const XKB_KEYMAP: i32 = 1;
    pub const ERROR_TOK: i32 = 255;
    pub const YYUNDEF: i32 = 257;
    pub const YYerror: i32 = 256;
    pub const END_OF_FILE: i32 = 0;
    pub const YYEMPTY: i32 = -2;
}
pub mod utf8_h {

    /// Native Rust UTF-32 to UTF-8 conversion (replaces C FFI)
    ///
    /// Encode a Unicode code point to UTF-8 into the given buffer.
    /// Returns the number of bytes written (NOT including null terminator), or 0 on failure.
    #[inline]
    pub fn utf32_to_utf8_safe(unichar: u32, buffer: &mut [u8]) -> usize {
        let Some(ch) = char::from_u32(unichar) else {
            return 0;
        };
        let encoded = ch.encode_utf8(&mut buffer[..]);
        encoded.len()
    }
}

// ── Keyword lookup (gperf-generated) ──

pub const MAX_HASH_VALUE: u32 = 72;
pub const MIN_WORD_LENGTH: u32 = 3;
pub const MAX_WORD_LENGTH: u32 = 21;
pub const MIN_HASH_VALUE: u32 = 3;
pub const TOTAL_KEYWORDS: u32 = 45;

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
pub fn keyword_to_token(string: &[u8]) -> i32 {
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
pub enum YYValue<'a> {
    #[default]
    None,
    Num(i64),
    FileType(u32),
    Str(String),
    Sval(sval<'a>),
    Atom(u32),
    Merge(merge_mode),
    MapFlags(xkb_map_flags),
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
    pub fn take(&mut self) -> YYValue<'a> {
        std::mem::take(self)
    }

    pub fn take_expr(&mut self) -> Option<Box<ExprDef>> {
        match std::mem::take(self) {
            YYValue::Expr(e) => Some(e),
            _ => Option::None,
        }
    }
    pub fn take_expr_list(&mut self) -> Vec<Box<ExprDef>> {
        match std::mem::take(self) {
            YYValue::ExprList(v) => v,
            _ => Vec::new(),
        }
    }
    pub fn take_var(&mut self) -> Option<Box<VarDef>> {
        match std::mem::take(self) {
            YYValue::Var(v) => Some(v),
            _ => Option::None,
        }
    }
    pub fn take_var_list(&mut self) -> Vec<Box<VarDef>> {
        match std::mem::take(self) {
            YYValue::VarList(v) => v,
            _ => Vec::new(),
        }
    }
    pub fn take_vmod(&mut self) -> Option<Box<VModDef>> {
        match std::mem::take(self) {
            YYValue::VMod(v) => Some(v),
            _ => Option::None,
        }
    }
    pub fn take_vmod_list(&mut self) -> Vec<Box<VModDef>> {
        match std::mem::take(self) {
            YYValue::VModList(v) => v,
            _ => Vec::new(),
        }
    }
    pub fn take_file(&mut self) -> Option<Box<XkbFile>> {
        match std::mem::take(self) {
            YYValue::File(f) => Some(f),
            _ => Option::None,
        }
    }
    pub fn take_file_list(&mut self) -> Vec<Box<XkbFile>> {
        match std::mem::take(self) {
            YYValue::FileList(v) => v,
            _ => Vec::new(),
        }
    }
    pub fn take_stmt_list(&mut self) -> Vec<Statement> {
        match std::mem::take(self) {
            YYValue::StmtList(v) => v,
            _ => Vec::new(),
        }
    }
    pub fn as_num(&self) -> i64 {
        match self {
            YYValue::Num(n) => *n,
            _ => 0,
        }
    }
    pub fn as_atom(&self) -> u32 {
        match self {
            YYValue::Atom(a) => *a,
            _ => 0,
        }
    }
    pub fn as_merge(&self) -> merge_mode {
        match self {
            YYValue::Merge(m) => *m,
            _ => MERGE_DEFAULT,
        }
    }
    pub fn as_map_flags(&self) -> xkb_map_flags {
        match self {
            YYValue::MapFlags(f) => *f,
            _ => 0,
        }
    }
    pub fn as_file_type(&self) -> u32 {
        match self {
            YYValue::FileType(f) => *f,
            _ => 0,
        }
    }
    pub fn as_keysym(&self) -> u32 {
        match self {
            YYValue::Keysym(k) => *k,
            _ => 0,
        }
    }
    pub fn as_no_sym_or_action_list(&self) -> u32 {
        match self {
            YYValue::NoSymbolOrActionList(n) => *n,
            _ => 0,
        }
    }
    pub fn as_sval(&self) -> sval<'a> {
        match self {
            YYValue::Sval(s) => *s,
            _ => sval::EMPTY,
        }
    }
    pub fn take_str(&mut self) -> String {
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
pub static DECIMAL_SEPARATOR: i8 = '.' as i32 as i8;
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
pub fn _xkbcommon_lex<'a>(
    yylval: &mut YYValue<'a>,
    s: &mut scanner<'a>,
    ctx: &mut xkb_context,
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
        *yylval = YYValue::Atom(xkb_atom_intern(ctx, &keyname_bytes));
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
pub fn XkbParseStringInit<'a>(
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
pub fn XkbParseString(
    ctx: &mut xkb_context,
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

use super::super::context::{
    xkb_context_failed_include_path_get, xkb_context_getenv, xkb_context_num_failed_include_paths,
};
use super::super::context::{xkb_context_include_path_get, xkb_context_num_include_paths};
use super::super::context::{
    xkb_context_include_path_get_extra_path, xkb_context_include_path_get_system_path,
};

pub const INCLUDE_MAX_DEPTH: i32 = 15_i32;
pub const MERGE_OVERRIDE_PREFIX: i32 = '+' as i32;
pub const MERGE_AUGMENT_PREFIX: i32 = '|' as i32;
pub const MERGE_REPLACE_PREFIX: i32 = '^' as i32;

/// Parsed result from one segment of an include statement.
pub struct ParsedIncludeMap {
    pub file: String,
    pub map: String,
    pub extra_data: String,
    pub nextop: char,
}

/// Parse one include map segment from `input`, returning the parsed result
/// and the remaining input (if any). Returns None on parse error.
pub fn ParseIncludeMap(input: &str) -> Option<(ParsedIncludeMap, Option<&str>)> {
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
fn LogIncludePaths(ctx: &mut xkb_context) {
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
pub fn expand_path_str(
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
pub fn FindFileInXkbPath(
    ctx: &mut xkb_context,
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
pub fn ExceedsIncludeMaxDepth(include_depth: u32) -> bool {
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
pub fn ProcessIncludeFile(
    ctx: &mut xkb_context,
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
