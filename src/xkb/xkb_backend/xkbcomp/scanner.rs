use self::parser_h::*;
use super::super::context::xkb_atom_intern;
use super::super::shared_types::*;
use super::super::utils::{parse_dec_u64, parse_hex_u32, parse_hex_u64};

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

// Re-export parse functions from parser module
pub use super::parser::parse;

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

pub use self::parser_h::{
    CBRACE, CBRACKET, COMMA, CPAREN, DECIMAL_DIGIT, DIVIDE, DOT, END_OF_FILE, EQUALS, ERROR_TOK,
    EXCLAM, FLOAT, IDENT, INTEGER, INVERT, KEYNAME, MINUS, OBRACE, OBRACKET, OPAREN, PLUS, SEMI,
    STRING, TIMES,
};
pub use super::super::shared_ast_types::{
    merge_mode, xkb_map_flags, ExprDef, GroupCompatDef, InterpDef, KeyAliasDef, KeyTypeDef,
    KeycodeDef, LedMapDef, LedNameDef, ModMapDef, Statement, SymbolsDef, UnknownStatement, VModDef,
    VarDef, XkbFile, MERGE_DEFAULT,
};
pub use super::messages::{
    XKB_ERROR_INVALID_FILE_ENCODING, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_WARNING_INVALID_ESCAPE_SEQUENCE, XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE,
    XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
};

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
pub fn XkbParseFile(
    ctx: &mut xkb_context,
    file: &std::fs::File,
    file_name: &str,
    map: &str,
) -> Option<Box<XkbFile>> {
    use super::super::utils::MappedFile;

    // Map the file
    let mapped = match MappedFile::new(file) {
        Ok(m) => m,
        Err(e) => {
            log::error!("Couldn't read XKB file {}: {}\n", file_name, e);
            return None;
        }
    };

    XkbParseString(ctx, mapped.as_bytes(), file_name, map)
}
