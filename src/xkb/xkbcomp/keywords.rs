
pub use super::scanner::parser_h::{
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
    pub name: i32,
    pub tok: i32,
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
pub type C2Rust_Unnamed = u32;
pub const MIN_HASH_VALUE: C2Rust_Unnamed = 3;
pub const TOTAL_KEYWORDS: C2Rust_Unnamed = 45;
static mut wordlist: [keyword_tok; 73] = [keyword_tok { name: 0, tok: 0 }; 73];
static mut gperf_downcase: [::core::ffi::c_uchar; 256] = [
    0 as i32 as ::core::ffi::c_uchar,
    1 as i32 as ::core::ffi::c_uchar,
    2 as i32 as ::core::ffi::c_uchar,
    3 as i32 as ::core::ffi::c_uchar,
    4 as i32 as ::core::ffi::c_uchar,
    5 as i32 as ::core::ffi::c_uchar,
    6 as i32 as ::core::ffi::c_uchar,
    7 as i32 as ::core::ffi::c_uchar,
    8 as i32 as ::core::ffi::c_uchar,
    9 as i32 as ::core::ffi::c_uchar,
    10 as i32 as ::core::ffi::c_uchar,
    11 as i32 as ::core::ffi::c_uchar,
    12 as i32 as ::core::ffi::c_uchar,
    13 as i32 as ::core::ffi::c_uchar,
    14 as i32 as ::core::ffi::c_uchar,
    15 as i32 as ::core::ffi::c_uchar,
    16 as i32 as ::core::ffi::c_uchar,
    17 as i32 as ::core::ffi::c_uchar,
    18 as i32 as ::core::ffi::c_uchar,
    19 as i32 as ::core::ffi::c_uchar,
    20 as i32 as ::core::ffi::c_uchar,
    21 as i32 as ::core::ffi::c_uchar,
    22 as i32 as ::core::ffi::c_uchar,
    23 as i32 as ::core::ffi::c_uchar,
    24 as i32 as ::core::ffi::c_uchar,
    25 as i32 as ::core::ffi::c_uchar,
    26 as i32 as ::core::ffi::c_uchar,
    27 as i32 as ::core::ffi::c_uchar,
    28 as i32 as ::core::ffi::c_uchar,
    29 as i32 as ::core::ffi::c_uchar,
    30 as i32 as ::core::ffi::c_uchar,
    31 as i32 as ::core::ffi::c_uchar,
    32 as i32 as ::core::ffi::c_uchar,
    33 as i32 as ::core::ffi::c_uchar,
    34 as i32 as ::core::ffi::c_uchar,
    35 as i32 as ::core::ffi::c_uchar,
    36 as i32 as ::core::ffi::c_uchar,
    37 as i32 as ::core::ffi::c_uchar,
    38 as i32 as ::core::ffi::c_uchar,
    39 as i32 as ::core::ffi::c_uchar,
    40 as i32 as ::core::ffi::c_uchar,
    41 as i32 as ::core::ffi::c_uchar,
    42 as i32 as ::core::ffi::c_uchar,
    43 as i32 as ::core::ffi::c_uchar,
    44 as i32 as ::core::ffi::c_uchar,
    45 as i32 as ::core::ffi::c_uchar,
    46 as i32 as ::core::ffi::c_uchar,
    47 as i32 as ::core::ffi::c_uchar,
    48 as i32 as ::core::ffi::c_uchar,
    49 as i32 as ::core::ffi::c_uchar,
    50 as i32 as ::core::ffi::c_uchar,
    51 as i32 as ::core::ffi::c_uchar,
    52 as i32 as ::core::ffi::c_uchar,
    53 as i32 as ::core::ffi::c_uchar,
    54 as i32 as ::core::ffi::c_uchar,
    55 as i32 as ::core::ffi::c_uchar,
    56 as i32 as ::core::ffi::c_uchar,
    57 as i32 as ::core::ffi::c_uchar,
    58 as i32 as ::core::ffi::c_uchar,
    59 as i32 as ::core::ffi::c_uchar,
    60 as i32 as ::core::ffi::c_uchar,
    61 as i32 as ::core::ffi::c_uchar,
    62 as i32 as ::core::ffi::c_uchar,
    63 as i32 as ::core::ffi::c_uchar,
    64 as i32 as ::core::ffi::c_uchar,
    97 as i32 as ::core::ffi::c_uchar,
    98 as i32 as ::core::ffi::c_uchar,
    99 as i32 as ::core::ffi::c_uchar,
    100 as i32 as ::core::ffi::c_uchar,
    101 as i32 as ::core::ffi::c_uchar,
    102 as i32 as ::core::ffi::c_uchar,
    103 as i32 as ::core::ffi::c_uchar,
    104 as i32 as ::core::ffi::c_uchar,
    105 as i32 as ::core::ffi::c_uchar,
    106 as i32 as ::core::ffi::c_uchar,
    107 as i32 as ::core::ffi::c_uchar,
    108 as i32 as ::core::ffi::c_uchar,
    109 as i32 as ::core::ffi::c_uchar,
    110 as i32 as ::core::ffi::c_uchar,
    111 as i32 as ::core::ffi::c_uchar,
    112 as i32 as ::core::ffi::c_uchar,
    113 as i32 as ::core::ffi::c_uchar,
    114 as i32 as ::core::ffi::c_uchar,
    115 as i32 as ::core::ffi::c_uchar,
    116 as i32 as ::core::ffi::c_uchar,
    117 as i32 as ::core::ffi::c_uchar,
    118 as i32 as ::core::ffi::c_uchar,
    119 as i32 as ::core::ffi::c_uchar,
    120 as i32 as ::core::ffi::c_uchar,
    121 as i32 as ::core::ffi::c_uchar,
    122 as i32 as ::core::ffi::c_uchar,
    91 as i32 as ::core::ffi::c_uchar,
    92 as i32 as ::core::ffi::c_uchar,
    93 as i32 as ::core::ffi::c_uchar,
    94 as i32 as ::core::ffi::c_uchar,
    95 as i32 as ::core::ffi::c_uchar,
    96 as i32 as ::core::ffi::c_uchar,
    97 as i32 as ::core::ffi::c_uchar,
    98 as i32 as ::core::ffi::c_uchar,
    99 as i32 as ::core::ffi::c_uchar,
    100 as i32 as ::core::ffi::c_uchar,
    101 as i32 as ::core::ffi::c_uchar,
    102 as i32 as ::core::ffi::c_uchar,
    103 as i32 as ::core::ffi::c_uchar,
    104 as i32 as ::core::ffi::c_uchar,
    105 as i32 as ::core::ffi::c_uchar,
    106 as i32 as ::core::ffi::c_uchar,
    107 as i32 as ::core::ffi::c_uchar,
    108 as i32 as ::core::ffi::c_uchar,
    109 as i32 as ::core::ffi::c_uchar,
    110 as i32 as ::core::ffi::c_uchar,
    111 as i32 as ::core::ffi::c_uchar,
    112 as i32 as ::core::ffi::c_uchar,
    113 as i32 as ::core::ffi::c_uchar,
    114 as i32 as ::core::ffi::c_uchar,
    115 as i32 as ::core::ffi::c_uchar,
    116 as i32 as ::core::ffi::c_uchar,
    117 as i32 as ::core::ffi::c_uchar,
    118 as i32 as ::core::ffi::c_uchar,
    119 as i32 as ::core::ffi::c_uchar,
    120 as i32 as ::core::ffi::c_uchar,
    121 as i32 as ::core::ffi::c_uchar,
    122 as i32 as ::core::ffi::c_uchar,
    123 as i32 as ::core::ffi::c_uchar,
    124 as i32 as ::core::ffi::c_uchar,
    125 as i32 as ::core::ffi::c_uchar,
    126 as i32 as ::core::ffi::c_uchar,
    127 as i32 as ::core::ffi::c_uchar,
    128 as i32 as ::core::ffi::c_uchar,
    129 as i32 as ::core::ffi::c_uchar,
    130 as i32 as ::core::ffi::c_uchar,
    131 as i32 as ::core::ffi::c_uchar,
    132 as i32 as ::core::ffi::c_uchar,
    133 as i32 as ::core::ffi::c_uchar,
    134 as i32 as ::core::ffi::c_uchar,
    135 as i32 as ::core::ffi::c_uchar,
    136 as i32 as ::core::ffi::c_uchar,
    137 as i32 as ::core::ffi::c_uchar,
    138 as i32 as ::core::ffi::c_uchar,
    139 as i32 as ::core::ffi::c_uchar,
    140 as i32 as ::core::ffi::c_uchar,
    141 as i32 as ::core::ffi::c_uchar,
    142 as i32 as ::core::ffi::c_uchar,
    143 as i32 as ::core::ffi::c_uchar,
    144 as i32 as ::core::ffi::c_uchar,
    145 as i32 as ::core::ffi::c_uchar,
    146 as i32 as ::core::ffi::c_uchar,
    147 as i32 as ::core::ffi::c_uchar,
    148 as i32 as ::core::ffi::c_uchar,
    149 as i32 as ::core::ffi::c_uchar,
    150 as i32 as ::core::ffi::c_uchar,
    151 as i32 as ::core::ffi::c_uchar,
    152 as i32 as ::core::ffi::c_uchar,
    153 as i32 as ::core::ffi::c_uchar,
    154 as i32 as ::core::ffi::c_uchar,
    155 as i32 as ::core::ffi::c_uchar,
    156 as i32 as ::core::ffi::c_uchar,
    157 as i32 as ::core::ffi::c_uchar,
    158 as i32 as ::core::ffi::c_uchar,
    159 as i32 as ::core::ffi::c_uchar,
    160 as i32 as ::core::ffi::c_uchar,
    161 as i32 as ::core::ffi::c_uchar,
    162 as i32 as ::core::ffi::c_uchar,
    163 as i32 as ::core::ffi::c_uchar,
    164 as i32 as ::core::ffi::c_uchar,
    165 as i32 as ::core::ffi::c_uchar,
    166 as i32 as ::core::ffi::c_uchar,
    167 as i32 as ::core::ffi::c_uchar,
    168 as i32 as ::core::ffi::c_uchar,
    169 as i32 as ::core::ffi::c_uchar,
    170 as i32 as ::core::ffi::c_uchar,
    171 as i32 as ::core::ffi::c_uchar,
    172 as i32 as ::core::ffi::c_uchar,
    173 as i32 as ::core::ffi::c_uchar,
    174 as i32 as ::core::ffi::c_uchar,
    175 as i32 as ::core::ffi::c_uchar,
    176 as i32 as ::core::ffi::c_uchar,
    177 as i32 as ::core::ffi::c_uchar,
    178 as i32 as ::core::ffi::c_uchar,
    179 as i32 as ::core::ffi::c_uchar,
    180 as i32 as ::core::ffi::c_uchar,
    181 as i32 as ::core::ffi::c_uchar,
    182 as i32 as ::core::ffi::c_uchar,
    183 as i32 as ::core::ffi::c_uchar,
    184 as i32 as ::core::ffi::c_uchar,
    185 as i32 as ::core::ffi::c_uchar,
    186 as i32 as ::core::ffi::c_uchar,
    187 as i32 as ::core::ffi::c_uchar,
    188 as i32 as ::core::ffi::c_uchar,
    189 as i32 as ::core::ffi::c_uchar,
    190 as i32 as ::core::ffi::c_uchar,
    191 as i32 as ::core::ffi::c_uchar,
    192 as i32 as ::core::ffi::c_uchar,
    193 as i32 as ::core::ffi::c_uchar,
    194 as i32 as ::core::ffi::c_uchar,
    195 as i32 as ::core::ffi::c_uchar,
    196 as i32 as ::core::ffi::c_uchar,
    197 as i32 as ::core::ffi::c_uchar,
    198 as i32 as ::core::ffi::c_uchar,
    199 as i32 as ::core::ffi::c_uchar,
    200 as i32 as ::core::ffi::c_uchar,
    201 as i32 as ::core::ffi::c_uchar,
    202 as i32 as ::core::ffi::c_uchar,
    203 as i32 as ::core::ffi::c_uchar,
    204 as i32 as ::core::ffi::c_uchar,
    205 as i32 as ::core::ffi::c_uchar,
    206 as i32 as ::core::ffi::c_uchar,
    207 as i32 as ::core::ffi::c_uchar,
    208 as i32 as ::core::ffi::c_uchar,
    209 as i32 as ::core::ffi::c_uchar,
    210 as i32 as ::core::ffi::c_uchar,
    211 as i32 as ::core::ffi::c_uchar,
    212 as i32 as ::core::ffi::c_uchar,
    213 as i32 as ::core::ffi::c_uchar,
    214 as i32 as ::core::ffi::c_uchar,
    215 as i32 as ::core::ffi::c_uchar,
    216 as i32 as ::core::ffi::c_uchar,
    217 as i32 as ::core::ffi::c_uchar,
    218 as i32 as ::core::ffi::c_uchar,
    219 as i32 as ::core::ffi::c_uchar,
    220 as i32 as ::core::ffi::c_uchar,
    221 as i32 as ::core::ffi::c_uchar,
    222 as i32 as ::core::ffi::c_uchar,
    223 as i32 as ::core::ffi::c_uchar,
    224 as i32 as ::core::ffi::c_uchar,
    225 as i32 as ::core::ffi::c_uchar,
    226 as i32 as ::core::ffi::c_uchar,
    227 as i32 as ::core::ffi::c_uchar,
    228 as i32 as ::core::ffi::c_uchar,
    229 as i32 as ::core::ffi::c_uchar,
    230 as i32 as ::core::ffi::c_uchar,
    231 as i32 as ::core::ffi::c_uchar,
    232 as i32 as ::core::ffi::c_uchar,
    233 as i32 as ::core::ffi::c_uchar,
    234 as i32 as ::core::ffi::c_uchar,
    235 as i32 as ::core::ffi::c_uchar,
    236 as i32 as ::core::ffi::c_uchar,
    237 as i32 as ::core::ffi::c_uchar,
    238 as i32 as ::core::ffi::c_uchar,
    239 as i32 as ::core::ffi::c_uchar,
    240 as i32 as ::core::ffi::c_uchar,
    241 as i32 as ::core::ffi::c_uchar,
    242 as i32 as ::core::ffi::c_uchar,
    243 as i32 as ::core::ffi::c_uchar,
    244 as i32 as ::core::ffi::c_uchar,
    245 as i32 as ::core::ffi::c_uchar,
    246 as i32 as ::core::ffi::c_uchar,
    247 as i32 as ::core::ffi::c_uchar,
    248 as i32 as ::core::ffi::c_uchar,
    249 as i32 as ::core::ffi::c_uchar,
    250 as i32 as ::core::ffi::c_uchar,
    251 as i32 as ::core::ffi::c_uchar,
    252 as i32 as ::core::ffi::c_uchar,
    253 as i32 as ::core::ffi::c_uchar,
    254 as i32 as ::core::ffi::c_uchar,
    255 as i32 as ::core::ffi::c_uchar,
];
use crate::xkb::shared_types::*;
unsafe fn gperf_case_memcmp(
    mut s1: *const i8,
    mut s2: *const i8,
    mut n: usize,
) -> i32 {
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
            if c1 as i32 == c2 as i32 {
                n = n.wrapping_sub(1);
            } else {
                return c1 as i32 - c2 as i32;
            }
        }
        return 0 as i32;
    }
}
#[inline]
unsafe fn keyword_gperf_hash(mut str: *const i8, mut len: usize) -> u32 {
    unsafe {
        static mut asso_values: [::core::ffi::c_uchar; 256] = [
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            36 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            1 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            20 as i32 as ::core::ffi::c_uchar,
            35 as i32 as ::core::ffi::c_uchar,
            20 as i32 as ::core::ffi::c_uchar,
            50 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            36 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            1 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            20 as i32 as ::core::ffi::c_uchar,
            35 as i32 as ::core::ffi::c_uchar,
            20 as i32 as ::core::ffi::c_uchar,
            50 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
            73 as i32 as ::core::ffi::c_uchar,
        ];
        let mut hval: u32 = len as u32;
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
                    asso_values[*str.offset(4 as i32 as isize)
                        as ::core::ffi::c_uchar as usize] as u32,
                );
                c2rust_current_block_2 = 14734602837567950927;
            }
        }
        match c2rust_current_block_2 {
            14734602837567950927 => {
                hval = hval.wrapping_add(
                    asso_values[*str.offset(1 as i32 as isize)
                        as ::core::ffi::c_uchar as usize] as u32,
                );
            }
            _ => {}
        }
        hval = hval.wrapping_add(
            asso_values
                [*str.offset(0 as i32 as isize) as ::core::ffi::c_uchar as usize]
                as u32,
        );
        return hval;
    }
}
pub unsafe fn keyword_to_token(mut string: *const i8, mut len: usize) -> i32 {
    unsafe {
        let mut kt: *const keyword_tok = keyword_gperf_lookup(string, len) as *const keyword_tok;
        if kt.is_null() {
            return -1 as i32;
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
unsafe fn keyword_gperf_lookup(mut str: *const i8, mut len: usize) -> *const keyword_tok {
    unsafe {
        static mut lengthtable: [::core::ffi::c_uchar; 73] = [
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            3 as i32 as ::core::ffi::c_uchar,
            4 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            4 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            11 as i32 as ::core::ffi::c_uchar,
            12 as i32 as ::core::ffi::c_uchar,
            12 as i32 as ::core::ffi::c_uchar,
            9 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            14 as i32 as ::core::ffi::c_uchar,
            10 as i32 as ::core::ffi::c_uchar,
            11 as i32 as ::core::ffi::c_uchar,
            17 as i32 as ::core::ffi::c_uchar,
            13 as i32 as ::core::ffi::c_uchar,
            4 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            21 as i32 as ::core::ffi::c_uchar,
            17 as i32 as ::core::ffi::c_uchar,
            13 as i32 as ::core::ffi::c_uchar,
            9 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            6 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            3 as i32 as ::core::ffi::c_uchar,
            4 as i32 as ::core::ffi::c_uchar,
            15 as i32 as ::core::ffi::c_uchar,
            6 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            6 as i32 as ::core::ffi::c_uchar,
            17 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            8 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            12 as i32 as ::core::ffi::c_uchar,
            13 as i32 as ::core::ffi::c_uchar,
            9 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            9 as i32 as ::core::ffi::c_uchar,
            0 as i32 as ::core::ffi::c_uchar,
            5 as i32 as ::core::ffi::c_uchar,
            7 as i32 as ::core::ffi::c_uchar,
        ];
        if len <= MAX_WORD_LENGTH as i32 as usize
            && len >= MIN_WORD_LENGTH as i32 as usize
        {
            let mut key: u32 = keyword_gperf_hash(str, len);
            if key <= MAX_HASH_VALUE as i32 as u32 {
                if len == lengthtable[key as usize] as usize {
                    let mut s: *const i8 = (&raw const stringpool_contents as *const i8)
                        .offset(wordlist[key as usize].name as isize);
                    if (*str as ::core::ffi::c_uchar as i32
                        ^ *s as ::core::ffi::c_uchar as i32)
                        & !(32 as i32)
                        == 0 as i32
                        && gperf_case_memcmp(str, s, len) == 0
                    {
                        return (&raw const wordlist as *const keyword_tok).offset(key as isize)
                            as *const keyword_tok;
                    }
                }
            }
        }
        return std::ptr::null();
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        wordlist = [
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str3 as usize
                    as i32,
                tok: KEY as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str4 as usize
                    as i32,
                tok: KEYS as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str7 as usize
                    as i32,
                tok: AUGMENT as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str9 as usize
                    as i32,
                tok: TEXT as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str10 as usize
                    as i32,
                tok: XKB_KEYMAP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str11 as usize
                    as i32,
                tok: KEYPAD_KEYS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str12 as usize
                    as i32,
                tok: XKB_KEYCODES as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str13 as usize
                    as i32,
                tok: XKB_GEOMETRY as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str14 as usize
                    as i32,
                tok: XKB_TYPES as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str15 as usize
                    as i32,
                tok: XKB_COMPATMAP as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str17 as usize
                    as i32,
                tok: REPLACE as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str19 as usize
                    as i32,
                tok: XKB_COMPATMAP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str20 as usize
                    as i32,
                tok: XKB_LAYOUT as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str21 as usize
                    as i32,
                tok: XKB_SYMBOLS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str22 as usize
                    as i32,
                tok: XKB_COMPATMAP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str23 as usize
                    as i32,
                tok: XKB_SEMANTICS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str24 as usize
                    as i32,
                tok: TYPE as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str25 as usize
                    as i32,
                tok: ALIAS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str26 as usize
                    as i32,
                tok: XKB_COMPATMAP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str27 as usize
                    as i32,
                tok: ALPHANUMERIC_KEYS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str28 as usize
                    as i32,
                tok: FUNCTION_KEYS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str29 as usize
                    as i32,
                tok: ALTERNATE as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str30 as usize
                    as i32,
                tok: SHAPE as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str31 as usize
                    as i32,
                tok: ACTION_TOK as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str32 as usize
                    as i32,
                tok: SECTION as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str33 as usize
                    as i32,
                tok: ROW as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str34 as usize
                    as i32,
                tok: LOGO as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str35 as usize
                    as i32,
                tok: ALTERNATE_GROUP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str36 as usize
                    as i32,
                tok: HIDDEN as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str37 as usize
                    as i32,
                tok: VIRTUAL as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str42 as usize
                    as i32,
                tok: OUTLINE as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str43 as usize
                    as i32,
                tok: DEFAULT as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str46 as usize
                    as i32,
                tok: MODIFIER_MAP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str47 as usize
                    as i32,
                tok: VIRTUAL_MODS as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str52 as usize
                    as i32,
                tok: OVERLAY as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str53 as usize
                    as i32,
                tok: OVERRIDE as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str57 as usize
                    as i32,
                tok: INCLUDE as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str62 as usize
                    as i32,
                tok: MODIFIER_MAP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str63 as usize
                    as i32,
                tok: MODIFIER_KEYS as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str64 as usize
                    as i32,
                tok: INDICATOR as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str66 as usize
                    as i32,
                tok: GROUP as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str67 as usize
                    as i32,
                tok: MODIFIER_MAP as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str69 as usize
                    as i32,
                tok: INTERPRET as i32,
            },
            keyword_tok {
                name: -1 as i32,
                tok: 0,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str71 as usize
                    as i32,
                tok: SOLID as i32,
            },
            keyword_tok {
                name: &raw mut (*std::ptr::null_mut::<stringpool_t>()).stringpool_str72 as usize
                    as i32,
                tok: PARTIAL as i32,
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
