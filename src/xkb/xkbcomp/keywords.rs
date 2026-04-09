pub mod parser_h {
    pub type yytokentype = ::core::ffi::c_int;
    pub const ALTERNATE_GROUP: yytokentype = 77;
    pub const FUNCTION_KEYS: yytokentype = 76;
    pub const KEYPAD_KEYS: yytokentype = 75;
    pub const MODIFIER_KEYS: yytokentype = 74;
    pub const ALPHANUMERIC_KEYS: yytokentype = 73;
    pub const HIDDEN: yytokentype = 72;
    pub const DEFAULT: yytokentype = 71;
    pub const PARTIAL: yytokentype = 70;
    pub const KEYNAME: yytokentype = 65;
    pub const IDENT: yytokentype = 64;
    pub const FLOAT: yytokentype = 63;
    pub const INTEGER: yytokentype = 62;
    pub const DECIMAL_DIGIT: yytokentype = 61;
    pub const STRING: yytokentype = 60;
    pub const INVERT: yytokentype = 55;
    pub const EXCLAM: yytokentype = 54;
    pub const SEMI: yytokentype = 53;
    pub const COMMA: yytokentype = 52;
    pub const DOT: yytokentype = 51;
    pub const CBRACKET: yytokentype = 50;
    pub const OBRACKET: yytokentype = 49;
    pub const CPAREN: yytokentype = 48;
    pub const OPAREN: yytokentype = 47;
    pub const CBRACE: yytokentype = 46;
    pub const OBRACE: yytokentype = 45;
    pub const TIMES: yytokentype = 44;
    pub const DIVIDE: yytokentype = 43;
    pub const MINUS: yytokentype = 42;
    pub const PLUS: yytokentype = 41;
    pub const EQUALS: yytokentype = 40;
    pub const VIRTUAL: yytokentype = 38;
    pub const LOGO: yytokentype = 37;
    pub const SOLID: yytokentype = 36;
    pub const OUTLINE: yytokentype = 35;
    pub const TEXT: yytokentype = 34;
    pub const OVERLAY: yytokentype = 33;
    pub const SECTION: yytokentype = 32;
    pub const ROW: yytokentype = 31;
    pub const KEYS: yytokentype = 30;
    pub const SHAPE: yytokentype = 29;
    pub const INDICATOR: yytokentype = 28;
    pub const MODIFIER_MAP: yytokentype = 27;
    pub const GROUP: yytokentype = 26;
    pub const ALIAS: yytokentype = 25;
    pub const KEY: yytokentype = 24;
    pub const ACTION_TOK: yytokentype = 23;
    pub const INTERPRET: yytokentype = 22;
    pub const TYPE: yytokentype = 21;
    pub const VIRTUAL_MODS: yytokentype = 20;
    pub const ALTERNATE: yytokentype = 14;
    pub const REPLACE: yytokentype = 13;
    pub const AUGMENT: yytokentype = 12;
    pub const OVERRIDE: yytokentype = 11;
    pub const INCLUDE: yytokentype = 10;
    pub const XKB_LAYOUT: yytokentype = 8;
    pub const XKB_SEMANTICS: yytokentype = 7;
    pub const XKB_GEOMETRY: yytokentype = 6;
    pub const XKB_COMPATMAP: yytokentype = 5;
    pub const XKB_SYMBOLS: yytokentype = 4;
    pub const XKB_TYPES: yytokentype = 3;
    pub const XKB_KEYCODES: yytokentype = 2;
    pub const XKB_KEYMAP: yytokentype = 1;
    pub const ERROR_TOK: yytokentype = 255;
    pub const YYUNDEF: yytokentype = 257;
    pub const YYerror: yytokentype = 256;
    pub const END_OF_FILE: yytokentype = 0;
    pub const YYEMPTY: yytokentype = -2;
}

pub use self::parser_h::{
    yytokentype, YYerror, ACTION_TOK, ALIAS, ALPHANUMERIC_KEYS, ALTERNATE, ALTERNATE_GROUP,
    AUGMENT, CBRACE, CBRACKET, COMMA, CPAREN, DECIMAL_DIGIT, DEFAULT, DIVIDE, DOT, END_OF_FILE,
    EQUALS, ERROR_TOK, EXCLAM, FLOAT, FUNCTION_KEYS, GROUP, HIDDEN, IDENT, INCLUDE, INDICATOR,
    INTEGER, INTERPRET, INVERT, KEY, KEYNAME, KEYPAD_KEYS, KEYS, LOGO, MINUS, MODIFIER_KEYS,
    MODIFIER_MAP, OBRACE, OBRACKET, OPAREN, OUTLINE, OVERLAY, OVERRIDE, PARTIAL, PLUS, REPLACE,
    ROW, SECTION, SEMI, SHAPE, SOLID, STRING, TEXT, TIMES, TYPE, VIRTUAL, VIRTUAL_MODS,
    XKB_COMPATMAP, XKB_GEOMETRY, XKB_KEYCODES, XKB_KEYMAP, XKB_LAYOUT, XKB_SEMANTICS, XKB_SYMBOLS,
    XKB_TYPES, YYEMPTY, YYUNDEF,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyword_tok {
    pub name: ::core::ffi::c_int,
    pub tok: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringpool_t {
    pub stringpool_str3: [i8; 4],
    pub stringpool_str4: [i8; 5],
    pub stringpool_str7: [i8; 8],
    pub stringpool_str9: [i8; 5],
    pub stringpool_str10: [i8; 11],
    pub stringpool_str11: [i8; 12],
    pub stringpool_str12: [i8; 13],
    pub stringpool_str13: [i8; 13],
    pub stringpool_str14: [i8; 10],
    pub stringpool_str15: [i8; 11],
    pub stringpool_str17: [i8; 8],
    pub stringpool_str19: [i8; 15],
    pub stringpool_str20: [i8; 11],
    pub stringpool_str21: [i8; 12],
    pub stringpool_str22: [i8; 18],
    pub stringpool_str23: [i8; 14],
    pub stringpool_str24: [i8; 5],
    pub stringpool_str25: [i8; 6],
    pub stringpool_str26: [i8; 22],
    pub stringpool_str27: [i8; 18],
    pub stringpool_str28: [i8; 14],
    pub stringpool_str29: [i8; 10],
    pub stringpool_str30: [i8; 6],
    pub stringpool_str31: [i8; 7],
    pub stringpool_str32: [i8; 8],
    pub stringpool_str33: [i8; 4],
    pub stringpool_str34: [i8; 5],
    pub stringpool_str35: [i8; 16],
    pub stringpool_str36: [i8; 7],
    pub stringpool_str37: [i8; 8],
    pub stringpool_str42: [i8; 8],
    pub stringpool_str43: [i8; 8],
    pub stringpool_str46: [i8; 7],
    pub stringpool_str47: [i8; 18],
    pub stringpool_str52: [i8; 8],
    pub stringpool_str53: [i8; 9],
    pub stringpool_str57: [i8; 8],
    pub stringpool_str62: [i8; 13],
    pub stringpool_str63: [i8; 14],
    pub stringpool_str64: [i8; 10],
    pub stringpool_str66: [i8; 6],
    pub stringpool_str67: [i8; 8],
    pub stringpool_str69: [i8; 10],
    pub stringpool_str71: [i8; 6],
    pub stringpool_str72: [i8; 8],
}
pub const MAX_HASH_VALUE: C2Rust_Unnamed = 72;
pub const MIN_WORD_LENGTH: C2Rust_Unnamed = 3;
pub const MAX_WORD_LENGTH: C2Rust_Unnamed = 21;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const MIN_HASH_VALUE: C2Rust_Unnamed = 3;
pub const TOTAL_KEYWORDS: C2Rust_Unnamed = 45;
static mut wordlist: [keyword_tok; 73] = [keyword_tok { name: 0, tok: 0 }; 73];
static mut gperf_downcase: [::core::ffi::c_uchar; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    123 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    124 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    125 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    126 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    127 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    128 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    129 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    130 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    131 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    132 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    133 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    134 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    135 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    136 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    137 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    138 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    139 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    140 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    141 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    142 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    143 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    144 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    145 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    146 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    147 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    148 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    149 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    150 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    151 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    152 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    153 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    154 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    155 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    156 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    157 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    158 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    159 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    160 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    161 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    162 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    163 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    164 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    165 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    166 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    167 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    168 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    169 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    170 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    171 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    172 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    173 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    174 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    175 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    176 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    177 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    178 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    179 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    180 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    181 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    182 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    183 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    184 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    185 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    186 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    187 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    188 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    189 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    190 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    191 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    192 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    193 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    194 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    195 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    196 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    197 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    198 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    199 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    200 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    201 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    202 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    203 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    204 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    205 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    206 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    207 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    208 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    209 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    210 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    211 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    212 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    213 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    214 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    215 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    216 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    217 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    218 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    219 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    220 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    221 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    222 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    223 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    224 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    225 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    226 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    227 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    228 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    229 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    230 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    231 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    232 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    233 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    234 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    235 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    236 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    237 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    238 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    239 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    240 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    241 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    242 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    243 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    244 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    245 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    246 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    247 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    248 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    249 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    250 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    251 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    252 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    253 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    254 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
unsafe extern "C" fn gperf_case_memcmp(
    mut s1: *const i8,
    mut s2: *const i8,
    mut n: usize,
) -> ::core::ffi::c_int {
    unsafe {
        while n > 0 as usize {
            let c2rust_fresh0 = s1;
            s1 = s1.offset(1);
            let mut c1: ::core::ffi::c_uchar =
                gperf_downcase[*c2rust_fresh0 as ::core::ffi::c_uchar as usize];
            let c2rust_fresh1 = s2;
            s2 = s2.offset(1);
            let mut c2: ::core::ffi::c_uchar =
                gperf_downcase[*c2rust_fresh1 as ::core::ffi::c_uchar as usize];
            if c1 as ::core::ffi::c_int == c2 as ::core::ffi::c_int {
                n = n.wrapping_sub(1);
            } else {
                return c1 as ::core::ffi::c_int - c2 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn keyword_gperf_hash(mut str: *const i8, mut len: usize) -> ::core::ffi::c_uint {
    unsafe {
        static mut asso_values: [::core::ffi::c_uchar; 256] = [
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ];
        let mut hval: ::core::ffi::c_uint = len as ::core::ffi::c_uint;
        let mut c2rust_current_block_2: u64;
        match hval {
            4 | 3 | 2 => {
                c2rust_current_block_2 = 14734602837567950927;
            }
            1 => {
                c2rust_current_block_2 = 17577715908279651303;
            }
            _ => {
                hval = hval.wrapping_add(
                    asso_values[*str.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_uint,
                );
                c2rust_current_block_2 = 14734602837567950927;
            }
        }
        match c2rust_current_block_2 {
            14734602837567950927 => {
                hval = hval.wrapping_add(
                    asso_values[*str.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_uint,
                );
            }
            _ => {}
        }
        hval = hval.wrapping_add(
            asso_values
                [*str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_uint,
        );
        return hval;
    }
}
#[no_mangle]
pub unsafe extern "C" fn keyword_to_token(
    mut string: *const i8,
    mut len: usize,
) -> ::core::ffi::c_int {
    unsafe {
        let mut kt: *const keyword_tok = keyword_gperf_lookup(string, len) as *const keyword_tok;
        if kt.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        return (*kt).tok;
    }
}
static mut stringpool_contents: stringpool_t = unsafe {
    stringpool_t {
        stringpool_str3: ::core::mem::transmute::<[u8; 4], [i8; 4]>(*b"key\0"),
        stringpool_str4: ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"keys\0"),
        stringpool_str7: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"augment\0"),
        stringpool_str9: ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"text\0"),
        stringpool_str10: ::core::mem::transmute::<[u8; 11], [i8; 11]>(*b"xkb_keymap\0"),
        stringpool_str11: ::core::mem::transmute::<[u8; 12], [i8; 12]>(*b"keypad_keys\0"),
        stringpool_str12: ::core::mem::transmute::<[u8; 13], [i8; 13]>(*b"xkb_keycodes\0"),
        stringpool_str13: ::core::mem::transmute::<[u8; 13], [i8; 13]>(*b"xkb_geometry\0"),
        stringpool_str14: ::core::mem::transmute::<[u8; 10], [i8; 10]>(*b"xkb_types\0"),
        stringpool_str15: ::core::mem::transmute::<[u8; 11], [i8; 11]>(*b"xkb_compat\0"),
        stringpool_str17: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"replace\0"),
        stringpool_str19: ::core::mem::transmute::<[u8; 15], [i8; 15]>(*b"xkb_compat_map\0"),
        stringpool_str20: ::core::mem::transmute::<[u8; 11], [i8; 11]>(*b"xkb_layout\0"),
        stringpool_str21: ::core::mem::transmute::<[u8; 12], [i8; 12]>(*b"xkb_symbols\0"),
        stringpool_str22: ::core::mem::transmute::<[u8; 18], [i8; 18]>(*b"xkb_compatibility\0"),
        stringpool_str23: ::core::mem::transmute::<[u8; 14], [i8; 14]>(*b"xkb_semantics\0"),
        stringpool_str24: ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"type\0"),
        stringpool_str25: ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"alias\0"),
        stringpool_str26: ::core::mem::transmute::<[u8; 22], [i8; 22]>(*b"xkb_compatibility_map\0"),
        stringpool_str27: ::core::mem::transmute::<[u8; 18], [i8; 18]>(*b"alphanumeric_keys\0"),
        stringpool_str28: ::core::mem::transmute::<[u8; 14], [i8; 14]>(*b"function_keys\0"),
        stringpool_str29: ::core::mem::transmute::<[u8; 10], [i8; 10]>(*b"alternate\0"),
        stringpool_str30: ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"shape\0"),
        stringpool_str31: ::core::mem::transmute::<[u8; 7], [i8; 7]>(*b"action\0"),
        stringpool_str32: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"section\0"),
        stringpool_str33: ::core::mem::transmute::<[u8; 4], [i8; 4]>(*b"row\0"),
        stringpool_str34: ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"logo\0"),
        stringpool_str35: ::core::mem::transmute::<[u8; 16], [i8; 16]>(*b"alternate_group\0"),
        stringpool_str36: ::core::mem::transmute::<[u8; 7], [i8; 7]>(*b"hidden\0"),
        stringpool_str37: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"virtual\0"),
        stringpool_str42: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"outline\0"),
        stringpool_str43: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"default\0"),
        stringpool_str46: ::core::mem::transmute::<[u8; 7], [i8; 7]>(*b"modmap\0"),
        stringpool_str47: ::core::mem::transmute::<[u8; 18], [i8; 18]>(*b"virtual_modifiers\0"),
        stringpool_str52: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"overlay\0"),
        stringpool_str53: ::core::mem::transmute::<[u8; 9], [i8; 9]>(*b"override\0"),
        stringpool_str57: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"include\0"),
        stringpool_str62: ::core::mem::transmute::<[u8; 13], [i8; 13]>(*b"modifier_map\0"),
        stringpool_str63: ::core::mem::transmute::<[u8; 14], [i8; 14]>(*b"modifier_keys\0"),
        stringpool_str64: ::core::mem::transmute::<[u8; 10], [i8; 10]>(*b"indicator\0"),
        stringpool_str66: ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"group\0"),
        stringpool_str67: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"mod_map\0"),
        stringpool_str69: ::core::mem::transmute::<[u8; 10], [i8; 10]>(*b"interpret\0"),
        stringpool_str71: ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"solid\0"),
        stringpool_str72: ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"partial\0"),
    }
};
unsafe extern "C" fn keyword_gperf_lookup(
    mut str: *const i8,
    mut len: usize,
) -> *const keyword_tok {
    unsafe {
        static mut lengthtable: [::core::ffi::c_uchar; 73] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ];
        if len <= MAX_WORD_LENGTH as ::core::ffi::c_int as usize
            && len >= MIN_WORD_LENGTH as ::core::ffi::c_int as usize
        {
            let mut key: ::core::ffi::c_uint = keyword_gperf_hash(str, len);
            if key <= MAX_HASH_VALUE as ::core::ffi::c_int as ::core::ffi::c_uint {
                if len == lengthtable[key as usize] as usize {
                    let mut s: *const i8 = (&raw const stringpool_contents as *const i8)
                        .offset(wordlist[key as usize].name as isize);
                    if (*str as ::core::ffi::c_uchar as ::core::ffi::c_int
                        ^ *s as ::core::ffi::c_uchar as ::core::ffi::c_int)
                        & !(32 as ::core::ffi::c_int)
                        == 0 as ::core::ffi::c_int
                        && gperf_case_memcmp(str, s, len) == 0
                    {
                        return (&raw const wordlist as *const keyword_tok).offset(key as isize)
                            as *const keyword_tok;
                    }
                }
            }
        }
        return ::core::ptr::null::<keyword_tok>();
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        wordlist = [
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str3 as usize
                    as ::core::ffi::c_int,
                tok: KEY as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str4 as usize
                    as ::core::ffi::c_int,
                tok: KEYS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str7 as usize
                    as ::core::ffi::c_int,
                tok: AUGMENT as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str9 as usize
                    as ::core::ffi::c_int,
                tok: TEXT as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str10 as usize
                    as ::core::ffi::c_int,
                tok: XKB_KEYMAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str11 as usize
                    as ::core::ffi::c_int,
                tok: KEYPAD_KEYS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str12 as usize
                    as ::core::ffi::c_int,
                tok: XKB_KEYCODES as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str13 as usize
                    as ::core::ffi::c_int,
                tok: XKB_GEOMETRY as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str14 as usize
                    as ::core::ffi::c_int,
                tok: XKB_TYPES as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str15 as usize
                    as ::core::ffi::c_int,
                tok: XKB_COMPATMAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str17 as usize
                    as ::core::ffi::c_int,
                tok: REPLACE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str19 as usize
                    as ::core::ffi::c_int,
                tok: XKB_COMPATMAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str20 as usize
                    as ::core::ffi::c_int,
                tok: XKB_LAYOUT as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str21 as usize
                    as ::core::ffi::c_int,
                tok: XKB_SYMBOLS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str22 as usize
                    as ::core::ffi::c_int,
                tok: XKB_COMPATMAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str23 as usize
                    as ::core::ffi::c_int,
                tok: XKB_SEMANTICS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str24 as usize
                    as ::core::ffi::c_int,
                tok: TYPE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str25 as usize
                    as ::core::ffi::c_int,
                tok: ALIAS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str26 as usize
                    as ::core::ffi::c_int,
                tok: XKB_COMPATMAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str27 as usize
                    as ::core::ffi::c_int,
                tok: ALPHANUMERIC_KEYS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str28 as usize
                    as ::core::ffi::c_int,
                tok: FUNCTION_KEYS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str29 as usize
                    as ::core::ffi::c_int,
                tok: ALTERNATE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str30 as usize
                    as ::core::ffi::c_int,
                tok: SHAPE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str31 as usize
                    as ::core::ffi::c_int,
                tok: ACTION_TOK as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str32 as usize
                    as ::core::ffi::c_int,
                tok: SECTION as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str33 as usize
                    as ::core::ffi::c_int,
                tok: ROW as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str34 as usize
                    as ::core::ffi::c_int,
                tok: LOGO as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str35 as usize
                    as ::core::ffi::c_int,
                tok: ALTERNATE_GROUP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str36 as usize
                    as ::core::ffi::c_int,
                tok: HIDDEN as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str37 as usize
                    as ::core::ffi::c_int,
                tok: VIRTUAL as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str42 as usize
                    as ::core::ffi::c_int,
                tok: OUTLINE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str43 as usize
                    as ::core::ffi::c_int,
                tok: DEFAULT as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str46 as usize
                    as ::core::ffi::c_int,
                tok: MODIFIER_MAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str47 as usize
                    as ::core::ffi::c_int,
                tok: VIRTUAL_MODS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str52 as usize
                    as ::core::ffi::c_int,
                tok: OVERLAY as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str53 as usize
                    as ::core::ffi::c_int,
                tok: OVERRIDE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str57 as usize
                    as ::core::ffi::c_int,
                tok: INCLUDE as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str62 as usize
                    as ::core::ffi::c_int,
                tok: MODIFIER_MAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str63 as usize
                    as ::core::ffi::c_int,
                tok: MODIFIER_KEYS as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str64 as usize
                    as ::core::ffi::c_int,
                tok: INDICATOR as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str66 as usize
                    as ::core::ffi::c_int,
                tok: GROUP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str67 as usize
                    as ::core::ffi::c_int,
                tok: MODIFIER_MAP as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str69 as usize
                    as ::core::ffi::c_int,
                tok: INTERPRET as ::core::ffi::c_int,
            },
            keyword_tok {
                name: -1 as ::core::ffi::c_int,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str71 as usize
                    as ::core::ffi::c_int,
                tok: SOLID as ::core::ffi::c_int,
            },
            keyword_tok {
                name: &raw mut (*::core::ptr::null_mut::<stringpool_t>()).stringpool_str72 as usize
                    as ::core::ffi::c_int,
                tok: PARTIAL as ::core::ffi::c_int,
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
