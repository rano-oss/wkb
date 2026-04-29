use crate::context::xkb_atom_intern;
use crate::shared_types::*;

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

pub use crate::xkbcomp::keywords::keyword_to_token;

pub use self::parser_h::{
    CBRACE, CBRACKET, COMMA, CPAREN, DECIMAL_DIGIT, DIVIDE, DOT, END_OF_FILE, EQUALS, ERROR_TOK,
    EXCLAM, FLOAT, IDENT, INTEGER, INVERT, KEYNAME, MINUS, OBRACE, OBRACKET, OPAREN, PLUS, SEMI,
    STRING, TIMES,
};
pub use crate::messages::{
    XKB_ERROR_INVALID_FILE_ENCODING, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_WARNING_INVALID_ESCAPE_SEQUENCE, XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE,
    XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
};
pub use crate::scanner_utils::{scanner, sval};
pub use crate::shared_ast_types::{
    merge_mode, xkb_map_flags, ExprDef, GroupCompatDef, InterpDef, KeyAliasDef, KeyTypeDef,
    KeycodeDef, LedMapDef, LedNameDef, ModMapDef, Statement, SymbolsDef, UnknownStatement, VModDef,
    VarDef, XkbFile, MERGE_DEFAULT,
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
    use crate::utils::MappedFile;

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
