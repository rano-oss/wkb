pub use super::scanner::parser_h::{
    YYerror, ACTION_TOK, ALIAS, ALPHANUMERIC_KEYS, ALTERNATE, ALTERNATE_GROUP, AUGMENT, CBRACE,
    CBRACKET, COMMA, CPAREN, DECIMAL_DIGIT, DEFAULT, DIVIDE, DOT, END_OF_FILE, EQUALS, ERROR_TOK,
    EXCLAM, FLOAT, FUNCTION_KEYS, GROUP, HIDDEN, IDENT, INCLUDE, INDICATOR, INTEGER, INTERPRET,
    INVERT, KEY, KEYNAME, KEYPAD_KEYS, KEYS, LOGO, MINUS, MODIFIER_KEYS, MODIFIER_MAP, OBRACE,
    OBRACKET, OPAREN, OUTLINE, OVERLAY, OVERRIDE, PARTIAL, PLUS, REPLACE, ROW, SECTION, SEMI,
    SHAPE, SOLID, STRING, TEXT, TIMES, TYPE, VIRTUAL, VIRTUAL_MODS, XKB_COMPATMAP, XKB_GEOMETRY,
    XKB_KEYCODES, XKB_KEYMAP, XKB_LAYOUT, XKB_SEMANTICS, XKB_SYMBOLS, XKB_TYPES, YYEMPTY, YYUNDEF,
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

static LENGTHTABLE: [u8; 73] = [
    0, 0, 0, 3, 4, 0, 0, 7, 0, 4, 10, 11, 12, 12, 9, 10, 0, 7, 0, 14, 10, 11, 17, 13, 4, 5, 21, 17,
    13, 9, 5, 6, 7, 3, 4, 15, 6, 7, 0, 0, 0, 0, 7, 7, 0, 0, 6, 17, 0, 0, 0, 0, 7, 8, 0, 0, 0, 7, 0,
    0, 0, 0, 12, 13, 9, 0, 5, 7, 0, 9, 0, 5, 7,
];

static WORDLIST: [keyword_tok; 73] = [
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str3) as i32,
        tok: KEY,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str4) as i32,
        tok: KEYS,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str7) as i32,
        tok: AUGMENT,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str9) as i32,
        tok: TEXT,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str10) as i32,
        tok: XKB_KEYMAP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str11) as i32,
        tok: KEYPAD_KEYS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str12) as i32,
        tok: XKB_KEYCODES,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str13) as i32,
        tok: XKB_GEOMETRY,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str14) as i32,
        tok: XKB_TYPES,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str15) as i32,
        tok: XKB_COMPATMAP,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str17) as i32,
        tok: REPLACE,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str19) as i32,
        tok: XKB_COMPATMAP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str20) as i32,
        tok: XKB_LAYOUT,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str21) as i32,
        tok: XKB_SYMBOLS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str22) as i32,
        tok: XKB_COMPATMAP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str23) as i32,
        tok: XKB_SEMANTICS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str24) as i32,
        tok: TYPE,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str25) as i32,
        tok: ALIAS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str26) as i32,
        tok: XKB_COMPATMAP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str27) as i32,
        tok: ALPHANUMERIC_KEYS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str28) as i32,
        tok: FUNCTION_KEYS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str29) as i32,
        tok: ALTERNATE,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str30) as i32,
        tok: SHAPE,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str31) as i32,
        tok: ACTION_TOK,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str32) as i32,
        tok: SECTION,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str33) as i32,
        tok: ROW,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str34) as i32,
        tok: LOGO,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str35) as i32,
        tok: ALTERNATE_GROUP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str36) as i32,
        tok: HIDDEN,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str37) as i32,
        tok: VIRTUAL,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str42) as i32,
        tok: OUTLINE,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str43) as i32,
        tok: DEFAULT,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str46) as i32,
        tok: MODIFIER_MAP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str47) as i32,
        tok: VIRTUAL_MODS,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str52) as i32,
        tok: OVERLAY,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str53) as i32,
        tok: OVERRIDE,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str57) as i32,
        tok: INCLUDE,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str62) as i32,
        tok: MODIFIER_MAP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str63) as i32,
        tok: MODIFIER_KEYS,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str64) as i32,
        tok: INDICATOR,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str66) as i32,
        tok: GROUP,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str67) as i32,
        tok: MODIFIER_MAP,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str69) as i32,
        tok: INTERPRET,
    },
    keyword_tok {
        name: -1_i32,
        tok: 0,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str71) as i32,
        tok: SOLID,
    },
    keyword_tok {
        name: std::mem::offset_of!(stringpool_t, stringpool_str72) as i32,
        tok: PARTIAL,
    },
];

unsafe fn gperf_case_memcmp(mut s1: *const i8, mut s2: *const i8, mut n: usize) -> i32 {
    unsafe {
        while n > 0_usize {
            let c2rust_fresh0 = s1;
            s1 = s1.offset(1);
            let c1: u8 = GPERF_DOWNCASE[*c2rust_fresh0 as u8 as usize];
            let c2rust_fresh1 = s2;
            s2 = s2.offset(1);
            let c2: u8 = GPERF_DOWNCASE[*c2rust_fresh1 as u8 as usize];
            if c1 as i32 == c2 as i32 {
                n = n.wrapping_sub(1);
            } else {
                return c1 as i32 - c2 as i32;
            }
        }
        0_i32
    }
}
#[inline]
unsafe fn keyword_gperf_hash(str: *const i8, len: usize) -> u32 {
    unsafe {
        let mut hval: u32 = len as u32;
        let c2rust_current_block_2: u64;
        match hval {
            2..=4 => {
                c2rust_current_block_2 = 14734602837567950927;
            }
            1 => {
                c2rust_current_block_2 = 17577715908279651303;
            }
            _ => {
                hval = hval
                    .wrapping_add(ASSO_VALUES[*str.offset(4_i32 as isize) as u8 as usize] as u32);
                c2rust_current_block_2 = 14734602837567950927;
            }
        }
        if c2rust_current_block_2 == 14734602837567950927 {
            hval =
                hval.wrapping_add(ASSO_VALUES[*str.offset(1_i32 as isize) as u8 as usize] as u32);
        }
        hval = hval.wrapping_add(ASSO_VALUES[*str.offset(0_i32 as isize) as u8 as usize] as u32);
        hval
    }
}
pub unsafe fn keyword_to_token(string: *const i8, len: usize) -> i32 {
    unsafe {
        let kt: *const keyword_tok = keyword_gperf_lookup(string, len) as *const keyword_tok;
        if kt.is_null() {
            return -1_i32;
        }
        (*kt).tok
    }
}
static STRINGPOOL_CONTENTS: stringpool_t = unsafe {
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
unsafe fn keyword_gperf_lookup(str: *const i8, len: usize) -> *const keyword_tok {
    unsafe {
        if len <= MAX_WORD_LENGTH as usize && len >= MIN_WORD_LENGTH as usize {
            let key: u32 = keyword_gperf_hash(str, len);
            if key <= MAX_HASH_VALUE && len == LENGTHTABLE[key as usize] as usize {
                let s: *const i8 = (&raw const STRINGPOOL_CONTENTS as *const i8)
                    .offset(WORDLIST[key as usize].name as isize);
                if (*str as u8 as i32 ^ *s as u8 as i32) & !32_i32 == 0_i32
                    && gperf_case_memcmp(str, s, len) == 0
                {
                    return &WORDLIST[key as usize] as *const keyword_tok;
                }
            }
        }
        std::ptr::null()
    }
}
