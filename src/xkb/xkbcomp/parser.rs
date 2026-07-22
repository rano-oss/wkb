// LALR(1) parser for XKB, converted from bison-generated C via c2rust
use super::super::keymap::xkb_escape_map_name;
use super::super::keymap::xkb_keymap_key_get_syms_by_level_ref;
use super::super::keysym::utf32_to_keysym;
use super::super::keysym::xkb_keysym_from_name;
use super::super::shared_types::*;
use super::super::shared_types::{
    parse_dec_u32, parse_dec_u64, parse_hex_u32, parse_hex_u64, utf8_next_code_point_safe,
    INVALID_UTF8_CODE_POINT,
};

use super::super::shared_types::{
    ExprKind, IncludeStmt, InterpDef, KeyAliasDef, KeyTypeDef, KeycodeDef, LedMapDef, LedNameDef,
    ModMapDef, Statement, SymbolsDef, UnknownStatement, VModDef, VarDef, XkbFile, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS,
    FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC,
    MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN,
    MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_EXPR_ADD,
    STMT_EXPR_ASSIGN, STMT_EXPR_DIVIDE, STMT_EXPR_INVERT, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS,
};

use super::super::keymap::action_type_text;
pub(crate) use super::super::keymap::mod_mask_get_effective;
use super::super::keymap::{format_control_names_offset, GROUP_LAST_INDEX_NAME};
pub(crate) use super::symbols::compile_compat_map;
pub(crate) use super::symbols::compile_key_types;
pub(crate) use super::symbols::compile_keycodes;
pub(crate) use super::symbols::compile_symbols;
use super::symbols::{expr_resolve_group, expr_resolve_group_mask};

pub(crate) const XKB_KEY_VOID_SYMBOL: i32 = 0xffffff_i32;
pub(crate) const XKB_KEY_0: i32 = 0x30;
pub(crate) const XKB_KEY_SECTION: i32 = 0xa7_i32;
pub(crate) const XKB_KEYSYM_MIN: i32 = 0;

// ── YYSYMBOL constants ──────────────────────────────────────────────
pub(crate) const YYSYMBOL_YYUNDEF: i32 = 2;
pub(crate) const YYSYMBOL_YYERROR: i32 = 1;
pub(crate) const YYSYMBOL_YYEOF: i32 = 0;
pub(crate) const YYSYMBOL_YYEMPTY: i32 = -2;

pub(crate) const YYFINAL: i32 = 16;
pub(crate) const YYLAST: i32 = 928;
pub(crate) const YYNTOKENS: i32 = 66;
pub(crate) const YYMAXUTOK: i32 = 257;
pub(crate) const YYINITDEPTH: usize = 200;
pub(crate) const YYMAXDEPTH: usize = 10000;
pub(crate) const YYPACT_NINF: i32 = -280;
pub(crate) const YYENOMEM: i32 = -2;

pub(crate) struct ParserParam<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    pub(crate) scanner: &'a mut Scanner<'a>,
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
static YYR1: [u8; 220] = [
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
static YYR2: [i8; 220] = [
    0, 2, 1, 1, 1, 7, 1, 1, 1, 2, 0, 7, 1, 1, 1, 1, 1, 1, 0, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 0,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 2, 3, 4, 5, 3, 3, 1, 1, 3, 6, 3, 1, 2, 0, 6,
    6, 1, 0, 3, 1, 3, 3, 1, 2, 1, 3, 5, 3, 5, 3, 4, 2, 0, 5, 6, 3, 1, 1, 1, 6, 5, 6, 5, 6, 6, 6, 6,
    2, 1, 5, 1, 1, 1, 1, 2, 1, 5, 1, 3, 1, 1, 3, 6, 3, 1, 3, 3, 1, 3, 5, 3, 3, 1, 5, 6, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 3, 1, 0, 3, 3, 3, 3, 3, 1, 2, 2, 2,
    2, 1, 4, 1, 1, 3, 3, 3, 1, 1, 3, 1, 3, 1, 2, 4, 1, 3, 4, 6, 1, 0, 1, 1, 1, 1, 3, 3, 1, 1, 3, 3,
    1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
];
// ── Helper functions ────────────────────────────────────────────────

fn _xkbcommon_error(param: &mut ParserParam, _msg: &str) {
    let _loc: ScannerLoc = param.scanner.token_location();
}

fn resolve_keysym(name: Sval) -> Option<u32> {
    let name_bytes = name.as_bytes();
    let name_str = name.as_str();

    if name_str.eq_ignore_ascii_case("any") || name_str.eq_ignore_ascii_case("nosymbol") {
        return Some(XKB_KEY_NO_SYMBOL as u32);
    }
    if name_str.eq_ignore_ascii_case("none") || name_str.eq_ignore_ascii_case("voidsymbol") {
        return Some(XKB_KEY_VOID_SYMBOL as u32);
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
        return Some(sym);
    }
    None
}

fn yypcontext_expected_tokens(yyssp: &[i16], ssp: usize, yyarg: &mut [i32], yyargn: usize) -> i32 {
    let mut yycount = 0;
    let yyn = YYPACT[yyssp[ssp] as usize] as i32;
    if yyn != YYPACT_NINF {
        let yyxbegin = if yyn < 0 { -yyn } else { 0 };
        let yychecklim = YYLAST - yyn + 1;
        let yyxend = if yychecklim < YYNTOKENS {
            yychecklim
        } else {
            YYNTOKENS
        };
        let mut yyx = yyxbegin;
        while yyx < yyxend {
            if YYCHECK[(yyx + yyn) as usize] as i32 == yyx && yyx != YYSYMBOL_YYERROR {
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

// ── Main parser function ────────────────────────────────────────────

pub(crate) fn _xkbcommon_parse<'a>(param: &mut ParserParam<'a>) -> i32 {
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
            } else if yychar == YYERROR {
                yychar = YYUNDEF;
                // goto yyerrlab1
                yyerrstatus = 3;
                // Error recovery: pop until we find a state that accepts error token
                loop {
                    yyn = YYPACT[yystate as usize] as i32;
                    if yyn != YYPACT_NINF {
                        yyn += YYSYMBOL_YYERROR;
                        if (0..=YYLAST).contains(&yyn)
                            && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYERROR
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
                                yyn += YYSYMBOL_YYERROR;
                                if (0..=YYLAST).contains(&yyn)
                                    && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYERROR
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
                    yyn += YYSYMBOL_YYERROR;
                    if (0..=YYLAST).contains(&yyn)
                        && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYERROR
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
                    yyn += YYSYMBOL_YYERROR;
                    if (0..=YYLAST).contains(&yyn)
                        && YYCHECK[yyn as usize] as i32 == YYSYMBOL_YYERROR
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

#[inline(always)]
fn yy_file_type<'a>(yyval: &mut YYValue<'a>, ft: u32) {
    *yyval = YYValue::FileType(ft);
}
#[inline(always)]
fn yy_flags<'a>(yyval: &mut YYValue<'a>, f: u32) {
    *yyval = YYValue::MapFlags(f);
}
#[inline(always)]
fn yy_atom<'a>(yyval: &mut YYValue<'a>, ctx: &mut &mut XkbContext, bytes: &[u8]) {
    *yyval = YYValue::Atom(atom_intern(&mut ctx.atom_table, bytes));
}
#[inline(always)]
fn yy_merge<'a>(yyval: &mut YYValue<'a>, m: u32) {
    *yyval = YYValue::Merge(m);
}
#[inline(always)]
fn yy_bin_expr<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize, op: u32) {
    let left = yyvs[sp - 2].take_expr();
    let right = yyvs[sp].take_expr();
    *yyval = YYValue::Expr(expr_create(ExprKind::Binary { op, left, right }));
}
#[inline(always)]
fn yy_unary_expr<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize, op: u32) {
    let child = yyvs[sp].take_expr();
    *yyval = YYValue::Expr(expr_create(ExprKind::Unary { op, child }));
}
#[inline(always)]
fn yy_list_push<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize, sp_off: usize) {
    let item = yyvs[sp].take_expr();
    let mut list = yyvs[sp - sp_off].take_expr_list();
    if let Some(e) = item {
        list.push(e);
    }
    *yyval = YYValue::ExprList(list);
}
#[inline(always)]
fn yy_list_single<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize) {
    let item = yyvs[sp].take_expr();
    let mut list = Vec::new();
    if let Some(e) = item {
        list.push(e);
    }
    *yyval = YYValue::ExprList(list);
}

macro_rules! yy_merge_decl {
    ($yyval:expr, $yyvs:expr, $sp:expr, $variant:ident, $stmt_variant:ident) => {
        let merge_mode = $yyvs[$sp - 1].as_merge();
        if let YYValue::$variant(mut item) = std::mem::replace(&mut $yyvs[$sp], YYValue::None) {
            item.merge = merge_mode;
            *$yyval = YYValue::Stmt(Statement::$stmt_variant(item));
        } else {
            *$yyval = YYValue::None;
        }
    };
}

/// Execute a reduction rule. Returns true on success, false on error (YYERROR).
fn execute_reduction<'a>(
    yyn: i32,
    yyvs: &mut [YYValue<'a>],
    sp: usize,
    yyval: &mut YYValue<'a>,
    param: &mut ParserParam<'a>,
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
            *yyval = YYValue::File(xkb_file_create(
                file_type,
                if name.is_empty() { None } else { Some(name) },
                defs,
                flags,
            ));
        }
        6 => {
            yy_file_type(yyval, FILE_TYPE_KEYMAP);
        }
        7 => {
            yy_file_type(yyval, FILE_TYPE_KEYMAP);
        }
        8 => {
            yy_file_type(yyval, FILE_TYPE_KEYMAP);
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
            *yyval = YYValue::File(xkb_file_create(
                file_type,
                if name.is_empty() { None } else { Some(name) },
                stmts,
                flags,
            ));
        }
        12 => {
            yy_file_type(yyval, FILE_TYPE_KEYCODES);
        }
        13 => {
            yy_file_type(yyval, FILE_TYPE_TYPES);
        }
        14 => {
            yy_file_type(yyval, FILE_TYPE_COMPAT);
        }
        15 => {
            yy_file_type(yyval, FILE_TYPE_SYMBOLS);
        }
        16 => {
            yy_file_type(yyval, FILE_TYPE_GEOMETRY);
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
            yy_flags(yyval, MAP_IS_PARTIAL);
        }
        22 => {
            yy_flags(yyval, MAP_IS_DEFAULT);
        }
        23 => {
            yy_flags(yyval, MAP_IS_HIDDEN);
        }
        24 => {
            yy_flags(yyval, MAP_HAS_ALPHANUMERIC);
        }
        25 => {
            yy_flags(yyval, MAP_HAS_MODIFIER);
        }
        26 => {
            yy_flags(yyval, MAP_HAS_KEYPAD);
        }
        27 => {
            yy_flags(yyval, MAP_HAS_FN);
        }
        28 => {
            yy_flags(yyval, MAP_IS_ALTGR);
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
            yy_merge_decl!(yyval, yyvs, sp, Interp, Interp);
        }
        34 => {
            // Decl: OptMergeMode KeyNameDecl
            yy_merge_decl!(yyval, yyvs, sp, Keycode, Keycode);
        }
        35 => {
            // Decl: OptMergeMode KeyAliasDecl
            yy_merge_decl!(yyval, yyvs, sp, KeyAlias, KeyAlias);
        }
        36 => {
            // Decl: OptMergeMode KeyTypeDecl
            yy_merge_decl!(yyval, yyvs, sp, KeyType, KeyType);
        }
        37 => {
            // Decl: OptMergeMode SymbolsDecl
            yy_merge_decl!(yyval, yyvs, sp, Symbols, Symbols);
        }
        38 => {
            // Decl: OptMergeMode ModMapDecl
            yy_merge_decl!(yyval, yyvs, sp, ModMask, ModMap);
        }
        39 => {
            if let YYValue::GroupCompat = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::GroupCompat(()));
            } else {
                *yyval = YYValue::None;
            }
        }
        40 => {
            // Decl: OptMergeMode LedMapDecl
            yy_merge_decl!(yyval, yyvs, sp, LedMap, LedMap);
        }
        41 => {
            // Decl: OptMergeMode LedNameDecl
            yy_merge_decl!(yyval, yyvs, sp, LedName, LedName);
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
            if let Some(inc) = include_create(param.ctx, &s, merge) {
                *yyval = YYValue::Stmt(Statement::Include(inc));
            } else {
                *yyval = YYValue::None;
            }
        }
        48 => {
            // VarDecl: Lhs EQUALS Expr SEMI
            let lhs = yyvs[sp - 3].take_expr();
            let val = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Var(var_create(lhs, val));
        }
        49 => {
            // VarDecl: Ident SEMI
            let atom = yyvs[sp - 1].as_atom();
            *yyval = YYValue::Var(bool_var_create(atom, true));
        }
        50 => {
            // VarDecl: EXCLAM Ident SEMI
            let atom = yyvs[sp - 1].as_atom();
            *yyval = YYValue::Var(bool_var_create(atom, false));
        }
        51 => {
            // KeyNameDecl: KEYNAME EQUALS KeyCode SEMI
            let atom = yyvs[sp - 3].as_atom();
            let num = yyvs[sp - 1].as_num();
            *yyval = YYValue::Keycode(keycode_create(atom, num));
        }
        52 => {
            // KeyAliasDecl: ALIAS KEYNAME EQUALS KEYNAME SEMI
            let alias = yyvs[sp - 3].as_atom();
            let real = yyvs[sp - 1].as_atom();
            *yyval = YYValue::KeyAlias(key_alias_create(alias, real));
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
            *yyval = YYValue::VMod(vmod_create(atom, None));
        }
        57 => {
            // VModDef: Ident EQUALS Expr
            let atom = yyvs[sp - 2].as_atom();
            let expr = yyvs[sp].take_expr();
            *yyval = YYValue::VMod(vmod_create(atom, expr));
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
            *yyval = YYValue::Interp(interp_create(keysym, expr));
        }
        60 => {
            // InterpretMatch: KeySym
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Interp(interp_create(keysym, None));
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
            *yyval = YYValue::KeyType(key_type_create(
                atom,
                vardefs.into_iter().map(|b| *b).collect(),
            ));
        }
        64 => {
            // SymbolsDecl: KEY KEYNAME OBRACE OptSymbolsBody CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Symbols(symbols_create(
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
            *yyval = YYValue::Var(var_create(lhs, val));
        }
        70 => {
            // SymbolsVarDecl: Lhs EQUALS MultiKeySymOrActionList
            let lhs = yyvs[sp - 2].take_expr();
            // MultiKeySymOrActionList is an ExprList or Expr
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(var_create(lhs, val));
        }
        71 => {
            // SymbolsVarDecl: Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Var(bool_var_create(atom, true));
        }
        72 => {
            // SymbolsVarDecl: EXCLAM Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Var(bool_var_create(atom, false));
        }
        73 => {
            // SymbolsVarDecl: Expr
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(var_create(None, val));
        }
        74 => {
            // MultiKeySymOrActionList: OBRACKET MultiKeySymList CBRACKET (yylen=3)
            let list = yyvs[sp - 1].take_expr_list();
            let exprs: Vec<ExprKind> = list.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        75 => {
            // MultiKeySymOrActionList: NoSymbolOrActionList OBRACKET MultiKeySymList CBRACKET COMMA (yylen=5)
            let mut list = yyvs[sp - 1].take_expr_list(); // sp-1 = MultiKeySymList = offset(-1)
            let count = yyvs[sp - 3].as_no_sym_or_action_list(); // sp-3 = NoSymbolOrActionList = offset(-3)
                                                                 // Prepend 'count' NoSymbol keysym lists
            let mut prepended: Vec<Box<ExprKind>> = Vec::new();
            for _ in 0..count {
                prepended.push(expr_create_key_sym_list(XKB_KEY_NO_SYMBOL as u32));
            }
            prepended.append(&mut list);
            let exprs: Vec<ExprKind> = prepended.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        76 => {
            // MultiKeySymOrActionList: OBRACKET MultiActionList CBRACKET (yylen=3)
            let list = yyvs[sp - 1].take_expr_list();
            let exprs: Vec<ExprKind> = list.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        77 => {
            // MultiKeySymOrActionList: NoSymbolOrActionList OBRACKET MultiActionList CBRACKET COMMA (yylen=5)
            let mut list = yyvs[sp - 1].take_expr_list();
            let count = yyvs[sp - 3].as_no_sym_or_action_list();
            let mut prepended: Vec<Box<ExprKind>> = Vec::new();
            for _ in 0..count {
                prepended.push(expr_create(ExprKind::ActionList {
                    actions: Vec::new(),
                }));
            }
            prepended.append(&mut list);
            let exprs: Vec<ExprKind> = prepended.into_iter().map(|b| *b).collect();
            *yyval = YYValue::Expr(expr_create(ExprKind::ActionList { actions: exprs }));
        }
        78 => {
            // NoSymbolOrActionList: NoSymbol (produces EmptyList expr)
            *yyval = YYValue::Expr(expr_create(ExprKind::EmptyList));
        }
        79 => {
            // NoSymbolOrActionList: NoSymbolOrActionList COMMA NoSymbol COMMA (yylen=4)
            let prev = yyvs[sp - 3].as_no_sym_or_action_list();
            *yyval = YYValue::NoSymbolOrActionList(prev + 1);
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
            *yyval = YYValue::GroupCompat;
        }
        83 => {
            // ModMapDecl: MODIFIER_MAP Ident OBRACE KeyOrKeySymList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let list = yyvs[sp - 2].take_expr_list();
            *yyval = YYValue::ModMask(mod_map_create(atom, list.into_iter().map(|b| *b).collect()));
        }
        84 => {
            // KeyOrKeySymList: KeyOrKeySymList COMMA KeyOrKeySym
            yy_list_push(yyval, yyvs, sp, 2);
        }
        85 => {
            // KeyOrKeySymList: KeyOrKeySym
            yy_list_single(yyval, yyvs, sp);
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
            *yyval = YYValue::LedMap(led_map_create(
                atom,
                vardefs.into_iter().map(|b| *b).collect(),
            ));
        }
        89 => {
            // LedNameDecl: INDICATOR Integer EQUALS Expr SEMI
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::LedName(led_name_create(num, expr));
        }
        90 => {
            // LedNameDecl: VIRTUAL INDICATOR Integer EQUALS Expr SEMI
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::LedName(led_name_create(num, expr));
        }
        91 => {
            // UnknownDecl: Ident Lhs EQUALS Expr SEMI
            // Drop expr values (geometry not supported)
            let _ = yyvs[sp - 3].take_expr();
            let _ = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Unknown(unknown_statement_create());
        }
        92 => {
            // UnknownCompoundStatementDecl: Ident Lhs OBRACE VarDeclList CBRACE SEMI
            let _ = yyvs[sp - 4].take_expr();
            let _ = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Unknown(unknown_statement_create());
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
            yy_atom(yyval, &mut param.ctx, b"action");
        }
        131 => {
            yy_atom(yyval, &mut param.ctx, b"interpret");
        }
        132 => {
            yy_atom(yyval, &mut param.ctx, b"type");
        }
        133 => {
            yy_atom(yyval, &mut param.ctx, b"key");
        }
        134 => {
            yy_atom(yyval, &mut param.ctx, b"group");
        }
        135 => {
            yy_atom(yyval, &mut param.ctx, b"modifier_map");
        }
        136 => {
            yy_atom(yyval, &mut param.ctx, b"indicator");
        }
        137 => {
            yy_atom(yyval, &mut param.ctx, b"shape");
        }
        138 => {
            yy_atom(yyval, &mut param.ctx, b"row");
        }
        139 => {
            yy_atom(yyval, &mut param.ctx, b"section");
        }
        140 => {
            yy_atom(yyval, &mut param.ctx, b"text");
        }
        // MergeMode rules 141-147
        141 => {
            *yyval = YYValue::Merge(yyvs[sp].as_merge());
        }
        142 => {
            yy_merge(yyval, MERGE_DEFAULT);
        }
        143 => {
            yy_merge(yyval, MERGE_DEFAULT);
        }
        144 => {
            yy_merge(yyval, MERGE_AUGMENT);
        }
        145 => {
            yy_merge(yyval, MERGE_OVERRIDE);
        }
        146 => {
            yy_merge(yyval, MERGE_REPLACE);
        }
        147 => {
            let _loc = param.scanner.token_location();
            yy_merge(yyval, MERGE_DEFAULT);
        }
        // ExprList rules 148-150
        148 => {
            // ExprList: ExprList COMMA Expr
            yy_list_push(yyval, yyvs, sp, 2);
        }
        149 => {
            // ExprList: Expr
            yy_list_single(yyval, yyvs, sp);
        }
        150 => {
            // ExprList: empty
            *yyval = YYValue::ExprList(Vec::new());
        }
        // Expr rules 151-165
        151 => {
            yy_bin_expr(yyval, yyvs, sp, STMT_EXPR_DIVIDE);
        }
        152 => {
            yy_bin_expr(yyval, yyvs, sp, STMT_EXPR_ADD);
        }
        153 => {
            yy_bin_expr(yyval, yyvs, sp, STMT_EXPR_SUBTRACT);
        }
        154 => {
            yy_bin_expr(yyval, yyvs, sp, STMT_EXPR_MULTIPLY);
        }
        155 => {
            yy_bin_expr(yyval, yyvs, sp, STMT_EXPR_ASSIGN);
        }
        156 => {
            // Expr: Term
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        157 => {
            yy_unary_expr(yyval, yyvs, sp, STMT_EXPR_NEGATE);
        }
        158 => {
            yy_unary_expr(yyval, yyvs, sp, STMT_EXPR_UNARY_PLUS);
        }
        159 => {
            yy_unary_expr(yyval, yyvs, sp, STMT_EXPR_NOT);
        }
        160 => {
            yy_unary_expr(yyval, yyvs, sp, STMT_EXPR_INVERT);
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
            let actions: Vec<ExprKind> = actions_expr_list.into_iter().map(|b| *b).collect();
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
            let actions: Vec<ExprKind> = actions_expr_list.into_iter().map(|b| *b).collect();
            let action_list_expr = expr_create(ExprKind::ActionList { actions });
            *yyval = YYValue::ExprList(vec![action_list_expr]);
        }
        169 => {
            // MultiActionList: KeySymList (initial single element)
            yy_list_single(yyval, yyvs, sp);
        }
        // NonEmptyActions rules 170-171
        170 => {
            // NonEmptyActions: NonEmptyActions COMMA Action
            yy_list_push(yyval, yyvs, sp, 2);
        }
        171 => {
            // NonEmptyActions: Action
            yy_list_single(yyval, yyvs, sp);
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
            let expr = expr_create_key_sym_list(keysym);
            let mut list = yyvs[sp - 2].take_expr_list();
            list.push(expr);
            *yyval = YYValue::ExprList(list);
        }
        187 => {
            // MultiKeySymList: MultiKeySymList COMMA KeySymList (expr variant)
            yy_list_push(yyval, yyvs, sp, 2);
        }
        188 => {
            // MultiKeySymList: KeySymList (keysym)
            let keysym = yyvs[sp].as_keysym();
            let expr = expr_create_key_sym_list(keysym);
            *yyval = YYValue::ExprList(vec![expr]);
        }
        189 => {
            // MultiKeySymList: KeySymList (expr)
            yy_list_single(yyval, yyvs, sp);
        }
        // KeySymList rules 190-197
        190 => {
            // NonEmptyKeySyms: NonEmptyKeySyms COMMA KeySym
            let expr = yyvs[sp - 2].take_expr().unwrap();
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(expr_append_key_sym_list(expr, keysym));
        }
        191 => {
            // NonEmptyKeySyms: NonEmptyKeySyms COMMA STRING
            let expr = yyvs[sp - 2].take_expr().unwrap();
            let s = yyvs[sp].take_str();
            match expr_key_sym_list_append_string(param.scanner, expr, &s) {
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
            *yyval = YYValue::Expr(expr_create_key_sym_list(keysym));
        }
        193 => {
            // KeySyms: STRING (single string keysym)
            let s = yyvs[sp].take_str();
            let expr = expr_create_key_sym_list(XKB_KEY_NO_SYMBOL as u32);
            match expr_key_sym_list_append_string(param.scanner, expr, &s) {
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
            let expr = expr_create_key_sym_list(XKB_KEY_NO_SYMBOL as u32);
            match expr_key_sym_list_append_string(param.scanner, expr, &s) {
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
            *yyval = YYValue::Expr(expr_create_key_sym_list(XKB_KEY_NO_SYMBOL as u32));
        }
        // KeySym rules 198-203
        198 => {
            // KeySymLit: KeySym (passthrough)
            *yyval = YYValue::Keysym(yyvs[sp].as_keysym());
        }
        199 => {
            // KeySym: STRING → parse string as keysym
            let s = yyvs[sp].take_str();
            let keysym = keysym_parse_string(param.scanner, &s);
            if keysym == XKB_KEY_NO_SYMBOL as u32 {
                return false;
            }
            *yyval = YYValue::Keysym(keysym);
        }
        200 => {
            // KeySym: IDENT → resolve keysym name
            let sval = yyvs[sp].as_sval();
            match resolve_keysym(sval) {
                Some(sym) => {
                    *yyval = YYValue::Keysym(sym);
                }
                None => {
                    let _loc = param.scanner.token_location();
                    *yyval = YYValue::Keysym(XKB_KEY_NO_SYMBOL as u32);
                }
            }
        }
        201 => {
            // KeySym: SECTION
            *yyval = YYValue::Keysym(XKB_KEY_SECTION as u32);
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
                let _loc = param.scanner.token_location();
                *yyval = YYValue::Keysym(XKB_KEY_NO_SYMBOL as u32);
            } else {
                if num <= XKB_KEYSYM_MAX as i64 {
                    let keysym = num as u32;
                    *yyval = YYValue::Keysym(keysym);
                } else {
                    let _loc = param.scanner.token_location();
                    *yyval = YYValue::Keysym(XKB_KEY_NO_SYMBOL as u32);
                }
                let _loc = param.scanner.token_location();
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
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, sval.as_bytes()));
        }
        215 => {
            // Ident: DEFAULT
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, b"default"));
        }
        // String 216
        216 => {
            // String: STRING → intern as atom
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Atom(atom_intern(&mut param.ctx.atom_table, s.as_bytes()));
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
    mut scanner: &'a mut Scanner<'a>,
    map: &str,
) -> Option<Box<XkbFile>> {
    let mut first: Option<Box<XkbFile>> = None;

    loop {
        let mut param = ParserParam {
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

    if let Some(_first_ref) = first.as_ref() {}
    first
}

// ── AST builder functions (merged from ast_build.rs) ──

pub(crate) fn expr_create(kind: ExprKind) -> Box<ExprKind> {
    Box::new(kind)
}

pub(crate) fn expr_create_key_sym_list(sym: u32) -> Box<ExprKind> {
    let mut syms = Vec::new();
    if sym != XKB_KEY_NO_SYMBOL as u32 {
        syms.push(sym);
    }
    expr_create(ExprKind::KeysymList { syms })
}

pub(crate) fn expr_append_key_sym_list(mut expr: Box<ExprKind>, sym: u32) -> Box<ExprKind> {
    if sym != XKB_KEY_NO_SYMBOL as u32 {
        if let ExprKind::KeysymList { ref mut syms } = *expr {
            syms.push(sym);
        }
    }
    expr
}

pub(crate) fn expr_key_sym_list_append_string(
    scanner: &mut Scanner,
    mut expr: Box<ExprKind>,
    string: &str,
) -> Option<Box<ExprKind>> {
    let bytes = string.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;
    loop {
        if idx >= len {
            return Some(expr);
        }
        let (cp, cp_len) = utf8_next_code_point_safe(&bytes[idx..]);
        if cp == INVALID_UTF8_CODE_POINT {
            let _loc = scanner.token_location();
            return None;
        }
        let sym = utf32_to_keysym(cp);
        if sym == XKB_KEY_NO_SYMBOL as u32 {
            let _loc = scanner.token_location();
            return None;
        }
        if let ExprKind::KeysymList { ref mut syms } = *expr {
            syms.push(sym);
        }
        idx += cp_len;
    }
}

pub(crate) fn keysym_parse_string(scanner: &mut Scanner, string: &str) -> u32 {
    let bytes = string.as_bytes();
    let len = bytes.len();
    if len == 0 {
        let _loc = scanner.token_location();
        return XKB_KEY_NO_SYMBOL as u32;
    }
    let (cp, cp_len) = utf8_next_code_point_safe(bytes);
    if cp == INVALID_UTF8_CODE_POINT {
        let _loc = scanner.token_location();
        return XKB_KEY_NO_SYMBOL as u32;
    } else if cp_len != len {
        let _loc = scanner.token_location();
        return XKB_KEY_NO_SYMBOL as u32;
    }
    let sym = utf32_to_keysym(cp);
    if sym == XKB_KEY_NO_SYMBOL as u32 {
        let _loc = scanner.token_location();
    }
    sym
}

pub(crate) fn keycode_create(name: u32, value: i64) -> Box<KeycodeDef> {
    Box::new(KeycodeDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub(crate) fn key_alias_create(alias: u32, real: u32) -> Box<KeyAliasDef> {
    Box::new(KeyAliasDef {
        merge: MERGE_DEFAULT,
        alias,
        real,
    })
}

pub(crate) fn vmod_create(name: u32, value: Option<Box<ExprKind>>) -> Box<VModDef> {
    Box::new(VModDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub(crate) fn var_create(name: Option<Box<ExprKind>>, value: Option<Box<ExprKind>>) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name,
        value,
    })
}

pub(crate) fn bool_var_create(ident: u32, set: bool) -> Box<VarDef> {
    Box::new(VarDef {
        merge: MERGE_DEFAULT,
        name: Some(Box::new(ExprKind::Ident(ident))),
        value: Some(Box::new(ExprKind::Boolean(set))),
    })
}

pub(crate) fn interp_create(sym: u32, match_0: Option<Box<ExprKind>>) -> Box<InterpDef> {
    Box::new(InterpDef {
        merge: MERGE_DEFAULT,
        sym,
        match_0,
        def: Vec::new(),
    })
}

pub(crate) fn key_type_create(name: u32, body: Vec<VarDef>) -> Box<KeyTypeDef> {
    Box::new(KeyTypeDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub(crate) fn symbols_create(key_name: u32, symbols: Vec<VarDef>) -> Box<SymbolsDef> {
    Box::new(SymbolsDef {
        merge: MERGE_DEFAULT,
        key_name,
        symbols,
    })
}

pub(crate) fn mod_map_create(modifier: u32, keys: Vec<ExprKind>) -> Box<ModMapDef> {
    Box::new(ModMapDef {
        merge: MERGE_DEFAULT,
        modifier,
        keys,
    })
}

pub(crate) fn led_map_create(name: u32, body: Vec<VarDef>) -> Box<LedMapDef> {
    Box::new(LedMapDef {
        merge: MERGE_DEFAULT,
        name,
        body,
    })
}

pub(crate) fn led_name_create(ndx: i64, name: Option<Box<ExprKind>>) -> Box<LedNameDef> {
    Box::new(LedNameDef {
        merge: MERGE_DEFAULT,
        ndx,
        name,
    })
}

pub(crate) fn unknown_statement_create() -> Box<UnknownStatement> {
    Box::new(UnknownStatement {})
}

pub(crate) fn include_create(
    _ctx: &mut XkbContext,
    stmt_str: &str,
    mut merge: u32,
) -> Option<Box<IncludeStmt>> {
    let mut items: Vec<Box<IncludeStmt>> = Vec::new();
    let mut remaining: Option<&str> = Some(stmt_str);

    loop {
        let input = match remaining {
            Some(s) if !s.is_empty() => s,
            _ => break,
        };

        let (parsed, rest) = match parse_include_map(input) {
            Some(r) => r,
            None => {
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

pub(crate) fn xkb_file_create(
    type_0: u32,
    name: Option<String>,
    defs: Vec<Statement>,
    flags: u32,
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

pub(crate) fn xkb_file_from_components(
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
        let include = include_create(ctx, component_str, MERGE_DEFAULT)?;
        let defs = vec![Statement::Include(include)];
        let file = xkb_file_create(type_0, None, defs, 0);
        file_stmts.push(Statement::XkbFile(file));
        type_0 += 1;
    }
    Some(xkb_file_create(FILE_TYPE_KEYMAP, None, file_stmts, 0))
}

// ── Scanner types (migrated from scanner_utils.rs) ──

#[derive(Copy, Clone, Default)]
pub(crate) struct Sval<'a> {
    pub(crate) data: &'a [u8],
}

impl<'a> Sval<'a> {
    pub(crate) const EMPTY: Sval<'static> = Sval { data: &[] };

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
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ScannerLoc {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

pub(crate) struct Scanner<'a> {
    pub(crate) pos: usize,
    pub(crate) s: &'a [u8],
    pub(crate) buf: [u8; 1024],
    pub(crate) buf_pos: usize,
    pub(crate) token_pos: usize,
    pub(crate) cached_pos: usize,
    pub(crate) cached_loc: ScannerLoc,
    pub(crate) file_name: String,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(s: &'a [u8], file_name: &str) -> Self {
        Scanner {
            pos: 0,
            s,
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: ScannerLoc { line: 1, column: 1 },
            file_name: file_name.to_string(),
        }
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
        if self.str_match(b"\xEF\xBB\xBF") || self.s.len() < 2 {
            return true;
        }
        let input = self.s;
        if input[0] == 0 || input[1] == 0 {
            return false;
        }
        if !input[0].is_ascii() {
            return false;
        }
        true
    }

    #[inline]
    pub(crate) fn input_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.s[start..end]
    }

    pub(crate) fn token_location(&mut self) -> ScannerLoc {
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

        let loc = ScannerLoc { line, column };
        self.cached_pos = self.token_pos;
        self.cached_loc = loc;
        loc
    }
}

// ── sval comparison functions (migrated from scanner_utils.rs) ──

#[inline]
pub(crate) fn svaleq(s1: Sval, s2: Sval) -> bool {
    s1.data == s2.data
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
pub(crate) const YYERROR: i32 = 256;
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
    Sval(Sval<'a>),
    Atom(u32),
    Merge(u32),
    MapFlags(u32),
    Keysym(u32),
    NoSymbolOrActionList(u32),
    Expr(Box<ExprKind>),
    ExprList(Vec<Box<ExprKind>>),
    Var(Box<VarDef>),
    VarList(Vec<Box<VarDef>>),
    VMod(Box<VModDef>),
    VModList(Vec<Box<VModDef>>),
    Interp(Box<InterpDef>),
    KeyType(Box<KeyTypeDef>),
    Symbols(Box<SymbolsDef>),
    ModMask(Box<ModMapDef>),
    GroupCompat,
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
    pub(crate) fn take_expr(&mut self) -> Option<Box<ExprKind>> {
        match std::mem::take(self) {
            YYValue::Expr(e) => Some(e),
            _ => None,
        }
    }
    pub(crate) fn take_expr_list(&mut self) -> Vec<Box<ExprKind>> {
        match std::mem::take(self) {
            YYValue::ExprList(v) => v,
            _ => Vec::new(),
        }
    }
    pub(crate) fn take_var(&mut self) -> Option<Box<VarDef>> {
        match std::mem::take(self) {
            YYValue::Var(v) => Some(v),
            _ => None,
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
            _ => None,
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
            _ => None,
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
    pub(crate) fn as_merge(&self) -> u32 {
        match self {
            YYValue::Merge(m) => *m,
            _ => MERGE_DEFAULT,
        }
    }
    pub(crate) fn as_map_flags(&self) -> u32 {
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
    pub(crate) fn as_sval(&self) -> Sval<'a> {
        match self {
            YYValue::Sval(s) => *s,
            _ => Sval::EMPTY,
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
fn number(s: &mut Scanner, out: &mut i64, out_tok: &mut i32) -> bool {
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
    s: &mut Scanner<'a>,
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
                        let _loc = s.token_location();
                    }
                } else if s.oct(&mut o) && o != 0 {
                    s.buf_append(o);
                } else if s.pos > start_pos {
                    let _loc_0 = s.token_location();
                } else {
                    let _loc_1 = s.token_location();
                }
            } else {
                let c = s.next_byte();
                s.buf_append(c as u8);
            }
        }
        if !s.buf_append(0) || !s.chr(b'"' as i8) {
            let _loc_2 = s.token_location();
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
            let _loc_3 = s.token_location();
            return ERROR_TOK;
        }
        let len: usize = s.pos - s.token_pos - 2;
        let keyname_bytes: Vec<u8> = s
            .input_slice(s.token_pos + 1, s.token_pos + 1 + len)
            .to_vec();
        *yylval = YYValue::Atom(atom_intern(&mut ctx.atom_table, &keyname_bytes));
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
        *yylval = YYValue::Sval(Sval {
            data: &s.s[s.token_pos..s.pos],
        });
        return IDENT;
    }
    let mut num_val: i64 = 0;
    if number(s, &mut num_val, &mut tok) {
        *yylval = YYValue::Num(num_val);
        if tok == ERROR_TOK {
            let _loc_4 = s.token_location();
            return ERROR_TOK;
        }
        return tok;
    }
    let _loc_5 = s.token_location();
    ERROR_TOK
}
pub(crate) fn xkb_parse_string_init<'a>(
    sc: &mut Scanner<'a>,
    input: &'a [u8],
    file_name: &str,
    _map: &str,
) -> bool {
    *sc = Scanner::new(input, file_name);
    if !sc.check_supported_char_encoding() {
        let _loc = sc.token_location();
        let _loc_0 = sc.token_location();
        return false;
    }
    true
}
pub(crate) fn xkb_parse_string(
    ctx: &mut XkbContext,
    input: &[u8],
    file_name: &str,
    map: &str,
) -> Option<Box<XkbFile>> {
    let mut sc = Scanner::new(&[], "");
    if !xkb_parse_string_init(&mut sc, input, file_name, map) {
        return None;
    }
    parse(ctx, &mut sc, map)
}

// ── Include file processing (merged from include.rs) ──

use super::super::keymap::{xkb_context_getenv, xkb_context_num_failed_include_paths};
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
pub(crate) fn parse_include_map(input: &str) -> Option<(ParsedIncludeMap, Option<&str>)> {
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
fn directory_for_include(type_0: u32) -> &'static str {
    if type_0 as usize >= XKB_FILE_TYPE_INCLUDE_DIRS.len() {
        ""
    } else {
        XKB_FILE_TYPE_INCLUDE_DIRS[type_0 as usize]
    }
}
fn log_include_paths(ctx: &mut XkbContext) {
    let num = xkb_context_num_include_paths(ctx);
    if num > 0 {
        for _i in 0..num {}
    }
    let num_failed = xkb_context_num_failed_include_paths(ctx);
    if num_failed > 0 {
        for _i in 0..num_failed {}
    }
}
/// Expand `%H`, `%S`, `%E`, `%%` in the given name string.
/// Returns `Some(expanded)` on success, `None` on error.
fn expand_percent(_parent_file_name: &str, type_dir: &str, name: &str) -> Option<String> {
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
                Some(_other) => {
                    return None;
                }
                None => {
                    return None;
                }
            }
        } else {
            result.push(c);
        }
        if result.len() > max_len {
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
    let type_dir = directory_for_include(file_type);
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
pub(crate) fn find_file_in_xkb_path(
    ctx: &mut XkbContext,
    _parent_file_name: &str,
    name: &str,
    type_0: u32,
    offset: &mut u32,
    required: bool,
) -> Option<(std::sync::Arc<Vec<u8>>, String)> {
    let type_dir = directory_for_include(type_0);
    let mut i: u32 = *offset;
    while i < xkb_context_num_include_paths(ctx) {
        let path = format!(
            "{}/{}/{}",
            xkb_context_include_path_get(ctx, i),
            type_dir,
            name
        );
        if path.len() >= 4096 {
        } else if let Some(data) = super::super::shared_types::read_file_cached(&path) {
            *offset = i;
            return Some((data, path));
        }
        i += 1;
    }
    if required && *offset == 0 {
        log_include_paths(ctx);
    }
    None
}
pub(crate) fn exceeds_include_max_depth(include_depth: u32) -> bool {
    include_depth >= INCLUDE_MAX_DEPTH as u32
}
pub(crate) fn process_include_file(
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
        find_file_in_xkb_path(ctx, "(unknown)", &stmt_file, file_type, &mut offset, true)
    };

    while let Some((ref file_data, ref _path)) = file_and_path {
        if let Some(parsed) = xkb_parse_string(ctx, file_data, &stmt.file, map_str) {
            let _ = file_and_path.take();

            if parsed.file_type != file_type {
                // parsed drops automatically
            } else if !stmt.map.is_empty() || parsed.flags != 0 && MAP_IS_DEFAULT != 0 {
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
            find_file_in_xkb_path(ctx, "(unknown)", &stmt_file, file_type, &mut offset, true);
    }

    if xkb_file.is_none() {
        xkb_file = candidate;
    }
    // else: candidate drops automatically

    if xkb_file.is_none() && !stmt.map.is_empty() {}
    xkb_file
}

pub(crate) const GROUP_MASK_NAME_LAST: u32 = 3;
pub(crate) const GROUP_INDEX_NAME_LAST: u32 = 1;
pub(crate) type CompileFileFn =
    Option<for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool>;
#[inline]
fn compute_effective_mask(keymap: &XkbKeymap, mods: &mut XkbMods) {
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
fn update_action_mods(keymap: &XkbKeymap, act: &mut XkbAction, modmap: u32) {
    match act {
        XkbAction::ModSet(m) | XkbAction::ModLatch(m) | XkbAction::ModLock(m) => {
            if m.flags & ACTION_MODS_LOOKUP_MODMAP != 0 {
                m.mods.mods = modmap;
            }
            compute_effective_mask(keymap, &mut m.mods);
        }
        _ => {}
    }
}
fn default_interpret() -> XkbSymInterpret {
    XkbSymInterpret {
        sym: XKB_KEY_NO_SYMBOL as u32,
        match_0: MATCH_ANY_OR_NONE,
        mods: 0,
        virtual_mod: DEFAULT_INTERPRET_VMOD,
        level_one_only: false,
        repeat: DEFAULT_INTERPRET_KEY_REPEAT != 0,
        required: false,
        num_actions: 0,
        action: XkbAction::None,
        actions: Vec::new(), // Note: XkbAction is Copy, so Vec::new() is zero-alloc (no heap until push)
    }
}
/// Returns interp indices into `keymap.sym_interprets`, or `usize::MAX` for default interprets.
fn find_interp_for_key(
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
    let _key_name = keymap.keys[key_idx].name;
    let num_syms = syms.len() as i32;
    let mut s: i32 = 0;
    while s < num_syms {
        let mut found: bool = false;
        let mut use_default: bool = false;
        let mut i: u32 = 0;
        's_26: loop {
            if i >= keymap.sym_interprets.len() as u32 {
                use_default = true;
                break;
            }
            let interp = &keymap.sym_interprets[i as usize];
            let mods: u32;
            if !(interp.sym != syms[s as usize] && interp.sym != XKB_KEY_NO_SYMBOL as u32) {
                if interp.level_one_only && level != 0 {
                    mods = 0;
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
                if found
                    && i > 0
                    && interp.sym == XKB_KEY_NO_SYMBOL as u32
                    && !interp_indices.is_empty()
                {
                    for &prev_idx in interp_indices.iter() {
                        if prev_idx == i as usize {
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
            i += 1;
        }
        if use_default {
            // usize::MAX signals "use default interpret"
            interp_indices.push(usize::MAX);
        }
        s += 1;
    }
    true
}
fn apply_interps_to_key(keymap: &mut XkbKeymap, key_idx: usize) -> bool {
    let mut vmodmap: u32 = 0;
    let mut level: u32;
    let mut interp_indices: Vec<usize> = Vec::with_capacity(4);
    let mut actions: Vec<XkbAction> = Vec::with_capacity(4);
    let num_groups = keymap.keys[key_idx].num_groups;
    let mut group: u32 = 0;
    while group < num_groups {
        if !keymap.keys[key_idx].groups[group as usize].explicit_actions {
            level = 0;
            let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], group);
            while level < num_levels {
                interp_indices.clear();
                let found: bool =
                    find_interp_for_key(keymap, key_idx, group, level, &mut interp_indices);
                if found {
                    let default_interp = default_interpret();
                    let key_explicit = keymap.keys[key_idx].explicit;
                    let _key_name = keymap.keys[key_idx].name;
                    for &idx in interp_indices.iter() {
                        let interp = if idx == usize::MAX {
                            &default_interp
                        } else {
                            &keymap.sym_interprets[idx]
                        };
                        if group == 0
                            && level == 0
                            && key_explicit & EXPLICIT_REPEAT == 0
                            && interp.repeat
                        {
                            keymap.keys[key_idx].repeats = true;
                        }
                        if (group == 0 && level == 0 || !interp.level_one_only)
                            && interp.virtual_mod != XKB_MOD_INVALID
                        {
                            vmodmap |= 1 << interp.virtual_mod;
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
                level += 1;
            }
            if keymap.keys[key_idx].groups[group as usize].implicit_actions {
                keymap.keys[key_idx].implicit_actions = true;
            }
        }
        group += 1;
    }
    if keymap.keys[key_idx].explicit & EXPLICIT_VMODMAP == 0 {
        keymap.keys[key_idx].vmodmap = vmodmap;
    }
    true
}
#[inline]
fn is_mod_action(action: &XkbAction) -> bool {
    matches!(
        action,
        XkbAction::ModSet(_) | XkbAction::ModLatch(_) | XkbAction::ModLock(_)
    )
}
#[inline]
fn is_group_action(action: &XkbAction) -> bool {
    matches!(
        action,
        XkbAction::GroupSet(_) | XkbAction::GroupLatch(_) | XkbAction::GroupLock(_)
    )
}
fn check_multiple_actions_categories(keymap: &mut XkbKeymap, key_idx: usize) {
    let num_groups = keymap.keys[key_idx].num_groups;
    let _key_name = keymap.keys[key_idx].name;
    let mut g: u32 = 0;
    while g < num_groups {
        let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], g);
        let mut l: u32 = 0;
        while l < num_levels {
            let level: &mut XkbLevel =
                &mut keymap.keys[key_idx].groups[g as usize].levels[l as usize];
            if level.actions.len() > 1 {
                let mut i: u16 = 0_u16;
                while (i as usize) < level.actions.len() {
                    let mod_action: bool = is_mod_action(&level.actions[i as usize]);
                    let group_action: bool = is_group_action(&level.actions[i as usize]);
                    let is_redirect =
                        matches!(level.actions[i as usize], XkbAction::RedirectKey(_));
                    if mod_action || group_action || is_redirect {
                        let mut j: u16 = (i as i32 + 1) as u16;
                        while (j as usize) < level.actions.len() {
                            let same_action = std::mem::discriminant(&level.actions[i as usize])
                                == std::mem::discriminant(&level.actions[j as usize]);
                            if same_action
                                || mod_action && is_mod_action(&level.actions[j as usize])
                                || group_action && is_group_action(&level.actions[j as usize])
                            {
                                let _type_0: &str = if mod_action {
                                    "modifiers"
                                } else if group_action {
                                    "group"
                                } else {
                                    action_type_text(ACTION_TYPE_REDIRECT_KEY)
                                };
                                level.actions[j as usize].set_none();
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
            }
            l += 1;
        }
        g += 1;
    }
}
fn add_key_aliases(keymap: &XkbKeymap, min: u32, max: u32, aliases: &mut Vec<XkbKeyAlias>) {
    let mut alias: u32 = min;
    while alias <= max {
        let entry: KeycodeMatch = keymap.key_names[alias as usize];
        if entry.is_alias && entry.found {
            aliases.push(XkbKeyAlias {});
        }
        alias += 1;
    }
}
fn update_pending_key_fields(info: &mut XkbKeymapInfo<'_>, key_idx: usize) -> bool {
    if info.keymap.keys[key_idx].out_of_range_pending_group {
        let idx = info.keymap.keys[key_idx].out_of_range_group_number as usize;
        if !info.pending_computations[idx].computed {
            // Temporarily take the expr out to avoid borrow conflict with info
            let expr_box = info.pending_computations[idx].expr.take().unwrap();
            let mut group: u32 = 0;
            let mut _pending = false;
            let resolve_ret = expr_resolve_group(info, &expr_box, true, &mut group, &mut _pending);
            info.pending_computations[idx].expr = Some(expr_box);
            match resolve_ret {
                0 => {
                    info.pending_computations[idx].computed = true;
                    info.pending_computations[idx].value = group.wrapping_sub(1);
                }
                2 => {
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
    match act {
        XkbAction::GroupSet(g) | XkbAction::GroupLatch(g) | XkbAction::GroupLock(g) => {
            if g.flags & ACTION_PENDING_COMPUTATION != 0 {
                let pc_idx = g.group as usize;
                if !info.pending_computations[pc_idx].computed {
                    let mut group: u32 = 0;
                    let absolute: bool = g.flags & ACTION_ABSOLUTE_SWITCH != 0;
                    let mut _pending = false;
                    let expr_box = info.pending_computations[pc_idx].expr.take().unwrap();
                    let resolve_ret =
                        expr_resolve_group(info, &expr_box, absolute, &mut group, &mut _pending);
                    info.pending_computations[pc_idx].expr = Some(expr_box);
                    match resolve_ret {
                        2 => {
                            return false;
                        }
                        1 => {}
                        _ => {
                            info.pending_computations[pc_idx].computed = true;
                            if absolute {
                                info.pending_computations[pc_idx].value = group.wrapping_sub(1);
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
                g.group = info.pending_computations[pc_idx].value as i32;
                g.flags = g.flags & !ACTION_PENDING_COMPUTATION;
            }
            true
        }
        XkbAction::RedirectKey(r) => {
            if keycode == XKB_KEYCODE_INVALID || r.keycode != info.keymap.redirect_key_auto {
                return true;
            } else {
                r.keycode = keycode;
            }
            true
        }
        _ => true,
    }
}
fn update_derived_keymap_fields(info: &mut XkbKeymapInfo<'_>) -> bool {
    let keymap: &mut XkbKeymap = &mut *info.keymap;
    build_key_aliases(keymap);
    compute_max_num_groups(keymap);
    let pending_computations: bool = !info.pending_computations.is_empty();
    if pending_computations {
        update_group_lookup_entries(info);
        if update_pending_sym_interpret_actions(info).is_err() {
            return false;
        }
    }
    if apply_interps_and_check_actions(info).is_err() {
        return false;
    }
    update_mod_mappings(info);
    compute_type_entry_masks(info);
    if update_key_action_fields(info, pending_computations).is_err() {
        return false;
    }
    compute_led_effective_masks(info);
    if pending_computations && resolve_pending_led_groups(info).is_err() {
        return false;
    }
    true
}

fn build_key_aliases(keymap: &mut XkbKeymap) {
    let mut num_key_aliases: u32 = 0;
    let mut min_alias: u32 = 0;
    let mut max_alias: u32 = 0;
    for (alias, entry) in keymap.key_names.iter().enumerate() {
        if entry.is_alias && entry.found {
            if num_key_aliases == 0 {
                min_alias = alias as u32;
            }
            max_alias = alias as u32;
            num_key_aliases += 1;
        }
    }
    if num_key_aliases != 0 {
        let mut aliases: Vec<XkbKeyAlias> = Vec::with_capacity(num_key_aliases as usize);
        add_key_aliases(keymap, min_alias, max_alias, &mut aliases);
        keymap.key_aliases = aliases;
    }
    keymap.key_names = Vec::new();
}

fn compute_max_num_groups(keymap: &mut XkbKeymap) {
    let start_idx = if keymap.num_keys_low == 0 {
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
        ki += 1;
    }
}

fn update_group_lookup_entries(info: &mut XkbKeymapInfo<'_>) {
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
        value: 1 << num_groups.wrapping_sub(1),
    };
}

fn update_pending_sym_interpret_actions(info: &mut XkbKeymapInfo<'_>) -> Result<(), ()> {
    let mut i: u32 = 0;
    while (i as usize) < info.keymap.sym_interprets.len() {
        let num_actions = info.keymap.sym_interprets[i as usize].num_actions;
        if num_actions as i32 <= 1_i32 {
            let mut action = info.keymap.sym_interprets[i as usize].action;
            if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, &mut action) {
                return Err(());
            }
            info.keymap.sym_interprets[i as usize].action = action;
        } else {
            let mut a: u16 = 0_u16;
            while (a as i32) < num_actions as i32 {
                let mut action = info.keymap.sym_interprets[i as usize].actions[a as usize];
                if !update_pending_action_fields(info, XKB_KEYCODE_INVALID, &mut action) {
                    return Err(());
                }
                info.keymap.sym_interprets[i as usize].actions[a as usize] = action;
                a += 1;
            }
        }
        i += 1;
    }
    Ok(())
}

fn apply_interps_and_check_actions(info: &mut XkbKeymapInfo<'_>) -> Result<(), ()> {
    let keymap = &mut *info.keymap;
    let start_idx = if keymap.num_keys_low == 0 {
        0_u32
    } else {
        keymap.min_key_code
    };
    let mut ki: u32 = start_idx;
    while ki < keymap.num_keys {
        if !apply_interps_to_key(keymap, ki as usize) {
            return Err(());
        }
        check_multiple_actions_categories(keymap, ki as usize);
        ki += 1;
    }
    Ok(())
}

fn update_mod_mappings(info: &mut XkbKeymapInfo<'_>) {
    let keymap = &mut *info.keymap;
    let start_idx = if keymap.num_keys_low == 0 {
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
            if key_vmodmap & 1 << idx != 0 {
                keymap.mods.mods[idx as usize].mapping |= key_modmap;
            }
            idx += 1;
        }
        ki += 1;
    }
    if keymap.format >= XKB_KEYMAP_FORMAT_TEXT_V2 {
        let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
        while idx < keymap.mods.num_mods {
            let mask: u32 = 1 << idx;
            if keymap.mods.mods[idx as usize].mapping == 0 && keymap.mods.explicit_vmods & mask == 0
            {
                keymap.mods.mods[idx as usize].mapping = mask;
                keymap.mods.explicit_vmods |= mask;
            }
            idx += 1;
        }
    }
    let mut extra_canonical_mods: u32 = 0;
    {
        let mut idx: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
        while idx < keymap.mods.num_mods {
            extra_canonical_mods |= keymap.mods.mods[idx as usize].mapping;
            idx += 1;
        }
    }
    keymap.canonical_state_mask |= extra_canonical_mods;
}

fn has_unbound_vmods(keymap: &XkbKeymap, mods: &XkbMods) -> bool {
    let mut k: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
    while k < keymap.mods.num_mods {
        if mods.mods & 1 << k != 0 && keymap.mods.mods[k as usize].mapping == 0 {
            return true;
        }
        k += 1;
    }
    false
}

fn compute_type_entry_masks(info: &mut XkbKeymapInfo<'_>) {
    let keymap = &mut *info.keymap;
    let mut i_0: u32 = 0;
    while (i_0 as usize) < keymap.types.len() {
        compute_effective_mask_with(&keymap.mods, &mut keymap.types[i_0 as usize].mods);
        let mut j: u32 = 0;
        while j < keymap.types[i_0 as usize].entries.len() as u32 {
            if has_unbound_vmods(keymap, &keymap.types[i_0 as usize].entries[j as usize].mods) {
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
            j += 1;
        }
        i_0 += 1;
    }
}

fn update_key_action_fields(
    info: &mut XkbKeymapInfo<'_>,
    pending_computations: bool,
) -> Result<(), ()> {
    let start_idx = if info.keymap.num_keys_low == 0 {
        0_u32
    } else {
        info.keymap.min_key_code
    };
    let mut ki: u32 = start_idx;
    while ki < info.keymap.num_keys {
        if !update_pending_key_fields(info, ki as usize) {
            return Err(());
        }
        let key_num_groups = info.keymap.keys[ki as usize].num_groups;
        let key_modmap = info.keymap.keys[ki as usize].modmap;
        let key_keycode = info.keymap.keys[ki as usize].keycode;
        let mut i_1: u32 = 0;
        while i_1 < key_num_groups {
            let num_levels = {
                let key = &info.keymap.keys[ki as usize];
                info.keymap.types[key.groups[i_1 as usize].type_idx as usize].num_levels
            };
            let mut j_0: u32 = 0;
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
                        update_action_mods(&*info.keymap, &mut act, key_modmap);
                        if (pending_computations || matches!(act, XkbAction::RedirectKey(_)))
                            && !update_pending_action_fields(info, key_keycode, &mut act)
                        {
                            return Err(());
                        }
                        info.keymap.keys[ki as usize].groups[i_1 as usize].levels[j_0 as usize]
                            .actions[0] = act;
                    }
                } else {
                    let mut k: u16 = 0_u16;
                    while (k as usize) < num_actions {
                        let mut act = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                            [j_0 as usize]
                            .actions[k as usize];
                        update_action_mods(&*info.keymap, &mut act, key_modmap);
                        if (pending_computations || matches!(act, XkbAction::RedirectKey(_)))
                            && !update_pending_action_fields(info, key_keycode, &mut act)
                        {
                            return Err(());
                        }
                        info.keymap.keys[ki as usize].groups[i_1 as usize].levels[j_0 as usize]
                            .actions[k as usize] = act;
                        k += 1;
                    }
                }
                j_0 += 1;
            }
            i_1 += 1;
        }
        ki += 1;
    }
    Ok(())
}

fn compute_led_effective_masks(info: &mut XkbKeymapInfo<'_>) {
    let keymap = &mut *info.keymap;
    let mut led_idx: u32 = 0;
    while led_idx < keymap.num_leds {
        compute_effective_mask_with(&keymap.mods, &mut keymap.leds[led_idx as usize].mods);
        led_idx += 1;
    }
}

fn resolve_pending_led_groups(info: &mut XkbKeymapInfo<'_>) -> Result<(), ()> {
    let mut led_idx: u32 = 0;
    while led_idx < info.keymap.num_leds {
        if info.keymap.leds[led_idx as usize].pending_groups {
            let groups_idx = info.keymap.leds[led_idx as usize].groups as usize;
            if !info.pending_computations[groups_idx].computed {
                let expr_box = info.pending_computations[groups_idx].expr.take().unwrap();
                let mut mask: u32 = 0;
                let mut _pending = false;
                let resolved = expr_resolve_group_mask(info, &expr_box, &mut mask, &mut _pending);
                info.pending_computations[groups_idx].expr = Some(expr_box);
                if !resolved {
                    return Err(());
                }
                info.pending_computations[groups_idx].computed = true;
                info.pending_computations[groups_idx].value = mask;
            }
            let value = info.pending_computations[groups_idx].value;
            info.keymap.leds[led_idx as usize].pending_groups = false;
            info.keymap.leds[led_idx as usize].groups = value;
        }
        led_idx += 1;
    }
    Ok(())
}
static COMPILE_FILE_FNS: [CompileFileFn; 4] = {
    [
        Some(compile_keycodes as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
        Some(compile_key_types as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
        Some(
            compile_compat_map as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool,
        ),
        Some(compile_symbols as for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool),
    ]
};
fn pending_computations_array_free(p: &mut Vec<PendingComputation>) {
    for pc in p.iter_mut() {
        pc.expr.take(); // Drop handles cleanup
    }
    p.clear();
}
pub(crate) fn compile_keymap(file: &mut XkbFile, keymap: &mut XkbKeymap) -> bool {
    let mut file_indices: [Option<usize>; 4] = [None; 4];
    for (idx, stmt) in file.defs.iter().enumerate() {
        if let Statement::XkbFile(ref sub_file) = stmt {
            if sub_file.file_type > LAST_KEYMAP_FILE_TYPE {
                if sub_file.file_type == FILE_TYPE_GEOMETRY {}
            } else if file_indices[sub_file.file_type as usize].is_some() {
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
            max_groups: XKB_MAX_GROUPS,
            max_overlays: XKB_OVERLAY_MAX,
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
                    value: 1,
                },
                LookupEntry {
                    name: if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    value: km_num_groups,
                },
                LookupEntry { name: "", value: 0 },
            ],
            group_mask_names: [
                LookupEntry {
                    name: "none",
                    value: 0,
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
                        1 << km_num_groups.wrapping_sub(1_u32)
                    } else {
                        0_u32
                    },
                },
                LookupEntry { name: "", value: 0 },
            ],
        },
        pending_computations: Vec::new(),
    };
    let mut type_0: u32 = FIRST_KEYMAP_FILE_TYPE;
    while type_0 <= LAST_KEYMAP_FILE_TYPE {
        if let Some(idx) = file_indices[type_0 as usize] {
            let _sub_name = if let Statement::XkbFile(ref sub_file) = file.defs[idx] {
                if sub_file.name.is_empty() {
                    "(unnamed map)"
                } else {
                    &sub_file.name
                }
            } else {
                ""
            };
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
            pending_computations_array_free(&mut info.pending_computations);
            return false;
        }
        type_0 += 1;
    }
    let ok_0: bool = update_derived_keymap_fields(&mut info) as bool;
    pending_computations_array_free(&mut info.pending_computations);
    ok_0
}
pub(crate) const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;

/// Appends bytes from `src` to the Vec<i8>.
#[inline]
fn vec_append_nul_terminated(v: &mut Vec<i8>, src: &[u8]) {
    v.extend(src.iter().map(|&b| b as i8));
}

/// Index-based sval for scanner input. Used in Lvalue/rule to avoid
/// lifetime issues across include boundaries. Reconstruct sval via to_sval().
#[derive(Copy, Clone, Default)]
pub(crate) struct SvalIdx {
    start: usize,
    end: usize,
}
impl SvalIdx {
    const EMPTY: SvalIdx = SvalIdx { start: 0, end: 0 };
    #[inline]
    fn to_sval<'a>(&self, input: &'a [u8]) -> Sval<'a> {
        if self.start >= self.end || self.start >= input.len() {
            Sval::EMPTY
        } else {
            Sval {
                data: &input[self.start..self.end.min(input.len())],
            }
        }
    }
    #[inline]
    fn len(&self) -> usize {
        self.end - self.start
    }
}

pub(crate) struct Matcher<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    pub(crate) rmlvo: RuleNames<'a>,
    pub(crate) val: Lvalue,
    pub(crate) groups: Vec<Group>,
    pub(crate) mapping: Mapping,
    pub(crate) rule: Rule,
    pub(crate) pending_kccgst: KccgstBuffer,
    pub(crate) kccgst: [Vec<i8>; 5],
}
#[derive(Clone, Default)]
pub(crate) struct KccgstBuffer {
    pub(crate) buffer: Vec<i8>,
    pub(crate) slices: Vec<KccgstBufferSlice>,
}
#[derive(Copy, Clone)]
pub(crate) struct KccgstBufferSlice {
    pub(crate) length: u32,
    pub(crate) kccgst: u32,
    pub(crate) layout: u32,
}
pub(crate) const _KCCGST_NUM_ENTRIES: u32 = 5;
pub(crate) const KCCGST_GEOMETRY: u32 = 4;
pub(crate) const KCCGST_SYMBOLS: u32 = 3;
pub(crate) const KCCGST_COMPAT: u32 = 2;
pub(crate) const KCCGST_TYPES: u32 = 1;
pub(crate) const KCCGST_KEYCODES: u32 = 0;
#[derive(Copy, Clone)]
pub(crate) struct Rule {
    pub(crate) mlvo_value_at_pos: [SvalIdx; 4],
    pub(crate) match_type_at_pos: [u32; 4],
    pub(crate) kccgst_value_at_pos: [SvalIdx; 5],
    pub(crate) num_mlvo_values: u8,
    pub(crate) num_kccgst_values: u8,
    pub(crate) skip: bool,
}
pub(crate) const MLVO_MATCH_GROUP: u32 = 5;
pub(crate) const MLVO_MATCH_WILDCARD_ANY: u32 = 4;
pub(crate) const MLVO_MATCH_WILDCARD_SOME: u32 = 3;
pub(crate) const MLVO_MATCH_WILDCARD_NONE: u32 = 2;
pub(crate) const MLVO_MATCH_WILDCARD_LEGACY: u32 = 1;
pub(crate) const MLVO_MATCH_NORMAL: u32 = 0;
#[derive(Copy, Clone, Default)]
pub(crate) struct Mapping {
    pub(crate) mlvo_at_pos: [u32; 4],
    pub(crate) num_mlvo: u8,
    pub(crate) defined_mlvo_mask: u8,
    pub(crate) layout: LayoutIdx,
    pub(crate) active_or_candidates_mask: u32,
    pub(crate) kccgst_at_pos: [u32; 5],
    pub(crate) num_kccgst: u8,
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
pub(crate) const _MLVO_NUM_ENTRIES: u32 = 4;
pub(crate) const MLVO_OPTION: u32 = 3;
pub(crate) const MLVO_VARIANT: u32 = 2;
pub(crate) const MLVO_LAYOUT: u32 = 1;
pub(crate) const MLVO_MODEL: u32 = 0;
#[derive(Clone)]
pub(crate) struct Group {
    pub(crate) name: Vec<u8>,
    pub(crate) elements: Vec<Vec<u8>>,
}
#[derive(Copy, Clone)]
pub(crate) struct Lvalue {
    pub(crate) string: SvalIdx,
}
#[derive(Clone, Default)]
pub(crate) struct RuleNames<'a> {
    pub(crate) model: MatchedSval<'a>,
    pub(crate) layouts: Vec<MatchedSval<'a>>,
    pub(crate) variants: Vec<MatchedSval<'a>>,
    pub(crate) options: Vec<MatchedSval<'a>>,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct MatchedSval<'a> {
    pub(crate) sval: Sval<'a>,
    pub(crate) matched: bool,
    pub(crate) layout: u32,
}
pub(crate) const TOK_ERROR: u32 = 11;
pub(crate) const TOK_INCLUDE: u32 = 10;
pub(crate) const TOK_WILD_CARD_ANY: u32 = 9;
pub(crate) const TOK_WILD_CARD_SOME: u32 = 8;
pub(crate) const TOK_WILD_CARD_NONE: u32 = 7;
pub(crate) const TOK_WILD_CARD_STAR: u32 = 6;
pub(crate) const TOK_EQUALS: u32 = 5;
pub(crate) const TOK_BANG: u32 = 4;
pub(crate) const TOK_GROUP_NAME: u32 = 3;
pub(crate) const TOK_IDENTIFIER: u32 = 2;
pub(crate) const TOK_END_OF_LINE: u32 = 1;
pub(crate) const TOK_END_OF_FILE: u32 = 0;
pub(crate) const LAYOUT_INDEX_FIRST: u32 = 4294967292;
pub(crate) const LAYOUT_INDEX_SINGLE: u32 = 4294967291;
pub(crate) const LAYOUT_INDEX_ANY: u32 = 4294967294;
pub(crate) const LAYOUT_INDEX_LATER: u32 = 4294967293;

impl Default for Rule {
    fn default() -> Self {
        Rule {
            mlvo_value_at_pos: [SvalIdx::EMPTY; 4],
            match_type_at_pos: [0; 4],
            kccgst_value_at_pos: [SvalIdx::EMPTY; 5],
            num_mlvo_values: 0,
            num_kccgst_values: 0,
            skip: false,
        }
    }
}
impl Default for Lvalue {
    fn default() -> Self {
        Lvalue {
            string: SvalIdx::EMPTY,
        }
    }
}
impl<'a> Matcher<'a> {
    fn new(ctx: &'a mut XkbContext) -> Self {
        Matcher {
            ctx,
            rmlvo: RuleNames::default(),
            val: Lvalue::default(),
            groups: Vec::new(),
            mapping: Mapping::default(),
            rule: Rule::default(),
            pending_kccgst: KccgstBuffer::default(),
            kccgst: std::array::from_fn(|_| Vec::new()),
        }
    }
}
pub(crate) const WILDCARD_MATCH_ALL: u32 = 1;
pub(crate) const WILDCARD_MATCH_NONEMPTY: u32 = 0;
pub(crate) const MAX_INCLUDE_DEPTH: i32 = 5_i32;
#[inline]
fn is_ident(ch: i8) -> bool {
    (ch as u8).is_ascii_graphic() && ch as i32 != '\\' as i32
}
fn lex(s: &mut Scanner, val: &mut Lvalue) -> u32 {
    loop {
        while s.chr(b' ' as i8) as i32 != 0
            || s.chr(b'\t' as i8) as i32 != 0
            || s.chr(b'\r' as i8) as i32 != 0
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
        s.chr(b'\r' as i8);
        if !s.eol() {
            let _loc: ScannerLoc = s.token_location();
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
            let _loc_0: ScannerLoc = s.token_location();
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
    let _loc_1: ScannerLoc = s.token_location();
    TOK_ERROR
}
static RULES_MLVO_SVALS: [&[u8]; 4] = [b"model", b"layout", b"variant", b"option"];
static RULES_KCCGST_SVALS: [&[u8]; 5] = [b"keycodes", b"types", b"compat", b"symbols", b"geometry"];
pub(crate) const OPTIONS_MATCH_ALL_GROUPS: u32 = XKB_MAX_GROUPS;
fn strip_spaces<'a>(v: Sval<'a>) -> Sval<'a> {
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
        Sval::EMPTY
    } else {
        Sval {
            data: &bytes[start_trim..end_trim],
        }
    }
}

/// Resize a Vec<MatchedSval>, zero-filling new elements.
fn vec_resize_zero_matched_sval(v: &mut Vec<MatchedSval<'_>>, new_len: usize) {
    if new_len > v.len() {
        v.resize(new_len, MatchedSval::default());
    } else {
        v.truncate(new_len);
    }
}

fn split_comma_separated_mlvo<'a>(mlvo: u32, s: Option<&'a [u8]>) -> Vec<MatchedSval<'a>> {
    let mut arr: Vec<MatchedSval<'a>> = Vec::new();
    let Some(bytes) = s else {
        arr.push(MatchedSval::default());
        return arr;
    };
    if bytes.is_empty() {
        arr.push(MatchedSval::default());
        return arr;
    }
    let mut pos: usize = 0;
    loop {
        let start = pos;
        let mut end = pos;
        let mut val_0 = MatchedSval {
            matched: false,
            layout: OPTIONS_MATCH_ALL_GROUPS as u32,
            sval: Sval {
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
        val_0.sval = Sval {
            data: &bytes[start..end],
        };
        val_0.sval = strip_spaces(val_0.sval);
        if pos < bytes.len() && bytes[pos] as i32 == OPTIONS_GROUP_SPECIFIER_PREFIX {
            pos += 1;
            let layout_start = pos;
            let (val_parsed, count) = parse_dec_u32(&bytes[pos..]);
            let layout: u32 = val_parsed;
            let count = count as usize;
            if count > 0 {
                pos += count;
                if layout == 0 || layout > XKB_MAX_GROUPS as u32 {
                } else if mlvo != MLVO_OPTION {
                } else {
                    val_0.layout -= 1;
                }
            }
            let layout_index_end = pos;
            while pos < bytes.len() && bytes[pos] != b',' {
                pos += 1;
            }
            if count == 0 || layout_index_end != pos {
                let _layout_spec = std::str::from_utf8(&bytes[layout_start..pos]).unwrap_or("");
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
) -> Box<Matcher<'a>> {
    let mut m = Box::new(Matcher::new(ctx));
    let rmlvo_ref = rmlvo;
    m.rmlvo.model.sval = Sval {
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
        vec_resize_zero_matched_sval(&mut m.rmlvo.variants, m.rmlvo.layouts.len());
    } else if m.rmlvo.layouts.len() < m.rmlvo.variants.len() {
        m.rmlvo.variants.truncate(m.rmlvo.layouts.len());
    }
    m
}
fn matcher_group_start_new(m: &mut Matcher, name: &[u8]) {
    let group: Group = Group {
        name: name.to_vec(),
        elements: Vec::new(),
    };
    m.groups.push(group);
}
fn matcher_group_add_element(m: &mut Matcher, _s: &mut Scanner, element: &[u8]) {
    let last_group = m.groups.last_mut().unwrap();
    last_group.elements.push(element.to_vec());
}
fn matcher_include(
    m: &mut Matcher<'_>,
    parent_scanner: &mut Scanner,
    include_depth: u32,
    inc: Sval,
) {
    if include_depth >= MAX_INCLUDE_DEPTH as u32 {
        let _loc: ScannerLoc = parent_scanner.token_location();
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
        find_file_in_xkb_path(
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
        let _path_str = path.clone();
        let _ = file_and_path.take();
        if ret {
            return;
        }
        if absolute_path {
            break;
        }
        offset += 1;
        file_and_path = find_file_in_xkb_path(
            &mut *m.ctx,
            &parent_scanner.file_name,
            &stmt_file,
            FILE_TYPE_RULES,
            &mut offset,
            true,
        );
    }
}
fn matcher_mapping_start_new(m: &mut Matcher) {
    let mut i: u8 = 0_u8;
    while (i as i32) < _MLVO_NUM_ENTRIES as i32 as u8 as i32 {
        m.mapping.mlvo_at_pos[i as usize] = _MLVO_NUM_ENTRIES;
        i += 1;
    }
    let mut i_0: u8 = 0_u8;
    while (i_0 as i32) < _KCCGST_NUM_ENTRIES as i32 as u8 as i32 {
        m.mapping.kccgst_at_pos[i_0 as usize] = _KCCGST_NUM_ENTRIES;
        i_0 += 1;
    }
    m.mapping.layout = LayoutIdx::Single {
        layout_idx: XKB_LAYOUT_INVALID,
        variant_idx: XKB_LAYOUT_INVALID,
    };
    m.mapping.num_kccgst = 0_u8;
    m.mapping.num_mlvo = m.mapping.num_kccgst;
    m.mapping.defined_mlvo_mask = 0_u8;
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
        range: u32,
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
fn is_mlvo_mask_defined(m: &mut Matcher, mlvo: u32) -> bool {
    m.mapping.defined_mlvo_mask as u32 & 1 << mlvo != 0
}
fn matcher_mapping_set_mlvo(m: &mut Matcher, s: &mut Scanner, ident: Sval) {
    let ident_bytes = ident.as_bytes();
    let mut mlvo: u32 = MLVO_MODEL;
    let mut mlvo_bytes: &[u8] = b"";
    while mlvo < _MLVO_NUM_ENTRIES {
        mlvo_bytes = RULES_MLVO_SVALS[mlvo as usize];
        if mlvo_bytes.len() <= ident_bytes.len() && &ident_bytes[..mlvo_bytes.len()] == mlvo_bytes {
            break;
        }
        mlvo += 1;
    }
    if mlvo >= _MLVO_NUM_ENTRIES {
        let _loc: ScannerLoc = s.token_location();
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if is_mlvo_mask_defined(m, mlvo) {
        let _loc_0: ScannerLoc = s.token_location();
        let _mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if mlvo_bytes.len() < ident_bytes.len() {
        let mut idx: u32 = 0;
        let remaining = &ident_bytes[mlvo_bytes.len()..];
        let consumed: i32 = extract_mapping_layout_index(remaining, &mut idx);
        if remaining.len() as i32 != consumed {
            let _loc_1: ScannerLoc = s.token_location();
            let _mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
            m.mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        if mlvo == MLVO_LAYOUT {
            if let LayoutIdx::Single {
                ref mut layout_idx, ..
            } = m.mapping.layout
            {
                *layout_idx = idx;
            }
        } else if mlvo == MLVO_VARIANT {
            if let LayoutIdx::Single {
                ref mut variant_idx,
                ..
            } = m.mapping.layout
            {
                *variant_idx = idx;
            }
        } else {
            let _loc_2: ScannerLoc = s.token_location();
            let _mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
            m.mapping.active_or_candidates_mask = 0_u32;
            return;
        }
    } else if mlvo == MLVO_LAYOUT {
        if let LayoutIdx::Single {
            ref mut layout_idx, ..
        } = m.mapping.layout
        {
            *layout_idx = LAYOUT_INDEX_SINGLE;
        }
    } else if mlvo == MLVO_VARIANT {
        if let LayoutIdx::Single {
            ref mut variant_idx,
            ..
        } = m.mapping.layout
        {
            *variant_idx = LAYOUT_INDEX_SINGLE;
        }
    }
    if (mlvo == MLVO_LAYOUT && is_mlvo_mask_defined(m, MLVO_VARIANT)
        || mlvo == MLVO_VARIANT && is_mlvo_mask_defined(m, MLVO_LAYOUT))
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
        let _loc_3: ScannerLoc = s.token_location();
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.mlvo_at_pos[m.mapping.num_mlvo as usize] = mlvo;
    m.mapping.defined_mlvo_mask =
        (m.mapping.defined_mlvo_mask as i32 | (1_u32 as u8 as i32) << mlvo) as u8;
    m.mapping.num_mlvo = m.mapping.num_mlvo + 1;
}
fn matcher_mapping_set_layout_bounds(m: &mut Matcher) {
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
        };
        m.mapping.active_or_candidates_mask = 1 << idx;
    };
}
fn matcher_mapping_set_kccgst(m: &mut Matcher, s: &mut Scanner, ident: Sval) {
    let ident_bytes = ident.as_bytes();
    let mut kccgst: u32 = KCCGST_KEYCODES;
    let mut kccgst_bytes: &[u8] = b"";
    while kccgst < _KCCGST_NUM_ENTRIES {
        kccgst_bytes = RULES_KCCGST_SVALS[kccgst as usize];
        if kccgst_bytes == ident_bytes {
            break;
        }
        kccgst += 1;
    }
    if kccgst >= _KCCGST_NUM_ENTRIES {
        let _loc: ScannerLoc = s.token_location();
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if m.mapping.defined_kccgst_mask as u32 & 1 << kccgst != 0 {
        let _loc_0: ScannerLoc = s.token_location();
        let _kccgst_str = std::str::from_utf8(kccgst_bytes).unwrap_or("");
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.kccgst_at_pos[m.mapping.num_kccgst as usize] = kccgst;
    m.mapping.defined_kccgst_mask =
        (m.mapping.defined_kccgst_mask as i32 | (1_u32 as u8 as i32) << kccgst) as u8;
    m.mapping.num_kccgst = m.mapping.num_kccgst + 1;
}
fn fn_layout_or_variant_valid(rmlvo_len: usize, idx: u32) -> bool {
    match idx {
        4294967291 => rmlvo_len <= 1,
        4294967292..=4294967294 => true,
        _ => rmlvo_len >= 2 && (idx as usize) < rmlvo_len,
    }
}

fn matcher_mapping_verify(m: &mut Matcher, s: &mut Scanner) -> bool {
    if m.mapping.num_mlvo == 0 {
        let _loc: ScannerLoc = s.token_location();
    } else if m.mapping.num_kccgst == 0 {
        let _loc_0: ScannerLoc = s.token_location();
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
fn matcher_rule_start_new(m: &mut Matcher) {
    m.rule = Rule::default();
    m.rule.skip = m.mapping.active_or_candidates_mask == 0;
}
fn matcher_rule_set_mlvo_common(m: &mut Matcher, s: &mut Scanner, ident: SvalIdx, match_type: u32) {
    if m.rule.num_mlvo_values as i32 >= m.mapping.num_mlvo as i32 {
        let _loc: ScannerLoc = s.token_location();
        m.rule.skip = true;
        return;
    }
    m.rule.match_type_at_pos[m.rule.num_mlvo_values as usize] = match_type;
    m.rule.mlvo_value_at_pos[m.rule.num_mlvo_values as usize] = ident;
    m.rule.num_mlvo_values = m.rule.num_mlvo_values + 1;
}
fn matcher_rule_set_mlvo_wildcard(m: &mut Matcher, s: &mut Scanner, match_type: u32) {
    let dummy = SvalIdx::EMPTY;
    matcher_rule_set_mlvo_common(m, s, dummy, match_type);
}
fn matcher_rule_set_mlvo_group(m: &mut Matcher, s: &mut Scanner, ident: SvalIdx) {
    matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
}
fn matcher_rule_set_mlvo(m: &mut Matcher, s: &mut Scanner, ident: SvalIdx) {
    matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
}
fn matcher_rule_set_kccgst(m: &mut Matcher, s: &mut Scanner, ident: SvalIdx) {
    if m.rule.num_kccgst_values as i32 >= m.mapping.num_kccgst as i32 {
        let _loc: ScannerLoc = s.token_location();
        m.rule.skip = true;
        return;
    }
    m.rule.kccgst_value_at_pos[m.rule.num_kccgst_values as usize] = ident;
    m.rule.num_kccgst_values = m.rule.num_kccgst_values + 1;
}
fn match_group(groups: &[Group], group_name: Sval, to: Sval) -> bool {
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
fn match_value(groups: &[Group], val: Sval, to: Sval, match_type: u32, wildcard_type: u32) -> bool {
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
    groups: &[Group],
    val: Sval,
    to: &mut MatchedSval,
    match_type: u32,
    wildcard_type: u32,
) -> bool {
    let matched: bool = match_value(groups, val, to.sval, match_type, wildcard_type);
    if matched {
        to.matched = true;
    }
    matched
}
fn expand_rmlvo_in_kccgst_value(
    m: &mut Matcher,
    s: &mut Scanner,
    value: Sval,
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
            let _loc: ScannerLoc = s.token_location();
        } else {
            *i += 1;
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
            *i += 1;
            if *i >= value.len() {
                // fall through to error
                let _loc_1: ScannerLoc = s.token_location();
                return false;
            }
        }

        let mlv: u32 = match bytes[*i] {
            b'm' => MLVO_MODEL,
            b'l' => MLVO_LAYOUT,
            b'v' => MLVO_VARIANT,
            _ => {
                let _loc_1: ScannerLoc = s.token_location();
                return false;
            }
        };
        *i += 1;

        let mut idx: u32 = XKB_LAYOUT_INVALID;
        let mut expanded_index: bool = false;
        if *i < value.len() && bytes[*i] == b'[' {
            if mlv != MLVO_LAYOUT && mlv != MLVO_VARIANT {
                let _loc_0: ScannerLoc = s.token_location();
                let _loc_1: ScannerLoc = s.token_location();
                return false;
            }
            let consumed: i32 = extract_layout_index(&bytes[*i..], &mut idx);
            if consumed == -1_i32 {
                let _loc_1: ScannerLoc = s.token_location();
                return false;
            }
            if idx == XKB_LAYOUT_INVALID {
                idx = layout_idx;
                expanded_index = true;
            }
            *i = (*i).wrapping_add(consumed as usize);
        }

        if sfx != 0 {
            if *i >= value.len() {
                let _loc_1: ScannerLoc = s.token_location();
                return false;
            }
            let ch = bytes[*i] as i8;
            *i += 1;
            if ch != sfx {
                let _loc_1: ScannerLoc = s.token_location();
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

        if mlv == MLVO_LAYOUT {
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
        } else if mlv == MLVO_VARIANT {
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
        } else if mlv == MLVO_MODEL {
            ev_ref = Some(RmlvoRef::Model);
        } else {
            ev_ref = None;
        }

        // Get the sval bytes (sval is Copy)
        let ev_sval = match &ev_ref {
            Some(RmlvoRef::Model) => m.rmlvo.model.sval,
            Some(RmlvoRef::Layout(idx)) => m.rmlvo.layouts[*idx].sval,
            Some(RmlvoRef::Variant(idx)) => m.rmlvo.variants[*idx].sval,
            None => Sval::EMPTY,
        };

        if ev_ref.is_none() || ev_sval.is_empty() {
            return true;
        }

        if pfx != 0 {
            vec_append_nul_terminated(expanded, &[pfx as u8]);
        }
        vec_append_nul_terminated(expanded, ev_sval.as_bytes());
        if sfx != 0 {
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

    let _loc_1: ScannerLoc = s.token_location();
    false
}
fn expand_qualifier_in_kccgst_value(
    m: &mut Matcher,
    s: &mut Scanner,
    value: Sval,
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
            let _loc: ScannerLoc = s.token_location();
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
                    let new_size = old_size.wrapping_add(prefix_length) + 1;
                    expanded.resize(new_size, 0);
                    expanded.copy_within(
                        prefix_idx as usize..prefix_idx as usize + prefix_length,
                        old_size,
                    );
                    expanded.truncate(new_size - 1);
                }
                let idx_str = format!("{}", l.wrapping_add(1_u32));
                vec_append_nul_terminated(expanded, idx_str.as_bytes());
                l += 1;
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
    m: &mut Matcher,
    s: &mut Scanner,
    value: Sval,
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
                vec_append_nul_terminated(&mut expanded, &[bytes[i]]);
                i += 1;
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
                i += 1;
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
                vec_append_nul_terminated(&mut expanded, &[bytes[i]]);
                i += 1;
                last_item_idx = (expanded.len() - 1) as u32;
                has_separator = true;
            }
            _ => {
                vec_append_nul_terminated(&mut expanded, &[bytes[i]]);
                i += 1;
            }
        }
    }
    if invalid {
        None
    } else {
        Some(expanded)
    }
}
fn matcher_append_pending_kccgst(m: &mut Matcher) -> bool {
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
    let mut i: u8 = 0_u8;
    while (i as i32) < m.mapping.num_kccgst as i32 {
        let kccgst: u32 = m.mapping.kccgst_at_pos[i as usize];
        let mut layout: u32 = range_min;
        while layout < range_max {
            let mut offset: usize = 0_usize;
            let mut k: u32 = 0;
            while k < m.pending_kccgst.slices.len() as u32 {
                let slice_len = m.pending_kccgst.slices[k as usize].length;
                let slice_kccgst = m.pending_kccgst.slices[k as usize].kccgst;
                let slice_layout = m.pending_kccgst.slices[k as usize].layout;
                if slice_kccgst == kccgst && slice_layout == layout && slice_len != 0 {
                    let from: Vec<i8> =
                        m.pending_kccgst.buffer[offset..offset + slice_len as usize].to_vec();
                    concat_kccgst(&mut m.kccgst[kccgst as usize], &from);
                }
                offset = offset.wrapping_add(slice_len as usize);
                k += 1;
            }
            layout += 1;
        }
        i += 1;
    }
    m.mapping.layout = LayoutIdx::default();
    true
}
fn matcher_rule_verify(m: &mut Matcher, s: &mut Scanner) {
    if m.rule.num_mlvo_values as i32 != m.mapping.num_mlvo as i32
        || m.rule.num_kccgst_values as i32 != m.mapping.num_kccgst as i32
    {
        let _loc: ScannerLoc = s.token_location();
        m.rule.skip = true;
    }
}
fn matcher_rule_apply_if_matches(m: &mut Matcher, s: &mut Scanner) {
    let mut candidate_layouts: u32 = m.mapping.active_or_candidates_mask;
    let mut idx: u32;
    let mut i: u8 = 0_u8;
    while (i as i32) < m.mapping.num_mlvo as i32 {
        let mlvo: u32 = m.mapping.mlvo_at_pos[i as usize];
        let value: Sval = m.rule.mlvo_value_at_pos[i as usize].to_sval(s.s);
        let match_type: u32 = m.rule.match_type_at_pos[i as usize];
        let mut matched: bool = false;
        if mlvo == MLVO_MODEL {
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
                let mask: u32 = 1 << idx;
                if candidate_layouts & mask != 0 {
                    match mlvo {
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
                                if opt.layout != OPTIONS_MATCH_ALL_GROUPS && opt.layout != idx {
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
                idx += 1;
            }
        } else {
            let li = m.mapping.layout.layout_idx_min() as usize;
            match mlvo {
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
                        if opt.layout != OPTIONS_MATCH_ALL_GROUPS && opt.layout != layout_min {
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
        i += 1;
    }
    if let LayoutIdx::Range {
        layout_idx_min,
        layout_idx_max,
    } = m.mapping.layout
    {
        idx = layout_idx_min;
        while idx < layout_idx_max {
            if candidate_layouts & 1 << idx != 0 {
                let mut i_0: u8 = 0_u8;
                while (i_0 as i32) < m.mapping.num_kccgst as i32 {
                    let kccgst: u32 = m.mapping.kccgst_at_pos[i_0 as usize];
                    let value_0: Sval = m.rule.kccgst_value_at_pos[i_0 as usize].to_sval(s.s);
                    let prev_buffer_length: u32 = m.pending_kccgst.buffer.len() as u32;
                    if let Some(expanded) = expand_kccgst_value(m, s, value_0, idx) {
                        if !expanded.is_empty() {
                            m.pending_kccgst.buffer.extend_from_slice(&expanded);
                        }
                        let length: u32 =
                            (m.pending_kccgst.buffer.len() as u32).wrapping_sub(prev_buffer_length);
                        let slice = KccgstBufferSlice {
                            length,
                            kccgst,
                            layout: idx,
                        };
                        m.pending_kccgst.slices.push(slice);
                    }
                    i_0 += 1;
                }
            }
            idx += 1;
        }
    } else if let LayoutIdx::Index { layout_idx_min, .. } = m.mapping.layout {
        let mut i_1: u8 = 0_u8;
        while (i_1 as i32) < m.mapping.num_kccgst as i32 {
            let kccgst_0: u32 = m.mapping.kccgst_at_pos[i_1 as usize];
            let value_1: Sval = m.rule.kccgst_value_at_pos[i_1 as usize].to_sval(s.s);
            if let Some(expanded) = expand_kccgst_value(m, s, value_1, layout_idx_min) {
                if !expanded.is_empty() {
                    concat_kccgst(&mut m.kccgst[kccgst_0 as usize], &expanded);
                }
            }
            i_1 += 1;
        }
    }
    if !is_mlvo_mask_defined(m, MLVO_OPTION) {
        m.mapping.active_or_candidates_mask &= !candidate_layouts;
    }
}
fn gettok(m: &mut Matcher, s: &mut Scanner) -> u32 {
    lex(s, &mut m.val)
}
fn matcher_match(m: &mut Matcher, s: &mut Scanner, include_depth: u32, _file_name: &str) -> bool {
    let mut eof_ok = false;
    let mut tok: u32;

    '_initial: loop {
        tok = gettok(m, s);
        match tok {
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
            match tok {
                3 => {
                    matcher_group_start_new(m, m.val.string.to_sval(s.s).data);
                    tok = gettok(m, s);
                    match tok {
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
                    match tok {
                        2 => {}
                        _ => {
                            break '_initial;
                        }
                    }
                    matcher_include(m, s, include_depth, m.val.string.to_sval(s.s));
                    tok = gettok(m, s);
                    match tok {
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
                        match tok {
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
                        match tok {
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
                    if m.mapping.active_or_candidates_mask != 0 && matcher_mapping_verify(m, s) {
                        matcher_mapping_set_layout_bounds(m);
                        if matches!(m.mapping.layout, LayoutIdx::Range { .. }) {
                            m.pending_kccgst.buffer.clear();
                            m.pending_kccgst.slices.clear();
                        }
                    }
                    loop {
                        tok = gettok(m, s);
                        match tok {
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
                                    match tok {
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
                                    match tok {
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
            match tok {
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
        match tok {
            11 => {}
            _ => {
                let _loc: ScannerLoc = s.token_location();
            }
        }
        false
    }
}
fn read_rules_file(
    matcher: &mut Matcher<'_>,
    include_depth: u32,
    file_data: &[u8],
    path: &str,
) -> bool {
    let mut scanner = Scanner::new(file_data, path);
    if !scanner.check_supported_char_encoding() {
        let _loc: ScannerLoc = scanner.token_location();
        let _loc_0: ScannerLoc = scanner.token_location();
        return false;
    }
    let ret: bool = matcher_match(matcher, &mut scanner, include_depth, path);
    ret
}
fn xkb_resolve_partial_rules(rules: &str, suffix: &str, matcher: &mut Matcher<'_>) -> bool {
    let partial_rules = format!("{}{}", rules, suffix);
    if partial_rules.len() >= 60 {
        return false;
    }
    let mut offset: u32 = 0;
    loop {
        let found = find_file_in_xkb_path(
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
            return false;
        }
        offset += 1;
    }
    true
}
fn xkb_resolve_rules(
    rules: &str,
    matcher: &mut Matcher<'_>,
    out: &mut XkbComponentNames,
    explicit_layouts: &mut u32,
) -> bool {
    let mut ret: bool;
    let mut offset: u32 = 0;
    let rules_str = rules;
    let found = find_file_in_xkb_path(
        &mut *matcher.ctx,
        "(unknown)",
        rules_str,
        FILE_TYPE_RULES,
        &mut offset,
        true,
    );
    let Some((file_data, path)) = found else {
        return false;
    };
    ret = xkb_resolve_partial_rules(rules_str, ".pre", matcher);
    if ret {
        ret = read_rules_file(matcher, 0, &file_data, &path);
    }
    if ret {
        ret = xkb_resolve_partial_rules(rules_str, ".post", matcher);
        if ret {
            if matcher.kccgst[KCCGST_KEYCODES as usize].is_empty()
                || matcher.kccgst[KCCGST_TYPES as usize].is_empty()
                || matcher.kccgst[KCCGST_COMPAT as usize].is_empty()
                || matcher.kccgst[KCCGST_SYMBOLS as usize].is_empty()
            {
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
                for mval in matcher.rmlvo.layouts.iter() {
                    if !mval.matched && !mval.sval.is_empty() {}
                }
                for mval in matcher.rmlvo.variants.iter() {
                    if !mval.matched && !mval.sval.is_empty() {}
                }
                for mval in matcher.rmlvo.options.iter() {
                    if !mval.matched && !mval.sval.is_empty() {}
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
    xkb_resolve_rules(rules_str, &mut matcher, out, explicit_layouts)
}
