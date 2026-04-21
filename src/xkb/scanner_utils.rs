// Canonical scanner utility types and functions (consolidated from scanner_utils_h blocks)
use crate::xkb::shared_types::*;

#[derive(Copy, Clone)]
pub struct sval<'a> {
    pub data: &'a [u8],
}

impl<'a> Default for sval<'a> {
    fn default() -> Self {
        sval { data: &[] }
    }
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
    pub ctx: *mut xkb_context,
}

impl<'a> scanner<'a> {
    pub fn new(ctx: *mut xkb_context, s: &'a [u8], file_name: &str) -> Self {
        scanner {
            pos: 0,
            s,
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: scanner_loc { line: 1, column: 1 },
            file_name: file_name.to_string(),
            ctx,
        }
    }

    /// The full input as a byte slice.
    #[inline]
    fn input_bytes(&self) -> &[u8] {
        self.s
    }

    /// The length of the input.
    #[inline]
    pub fn len(&self) -> usize {
        self.s.len()
    }

    /// The remaining input as a byte slice.
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
        let input = &self.s[self.pos..self.pos + len];
        if input != string {
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
            let mut count = crate::xkb::xkbcomp::scanner::utf8_h::utf32_to_utf8_safe(
                c,
                &mut self.buf[self.buf_pos..],
            );
            if count == 0 {
                count = crate::xkb::xkbcomp::scanner::utf8_h::utf32_to_utf8_safe(
                    0xfffd,
                    &mut self.buf[self.buf_pos..],
                );
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
        let (val, count) = crate::xkb::utils::parse_dec_u64(remaining);
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
        let (val, count) = crate::xkb::utils::parse_hex_u64(remaining);
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
        let (cp, count) = crate::xkb::utils::parse_hex_u32(remaining);
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
        use crate::xkb::messages::XKB_ERROR_INVALID_FILE_ENCODING;

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

    /// Get a pointer into the input at the given position.
    /// Used by callers that need a pointer into the input at a specific position.
    #[inline]
    pub fn input_at(&self, pos: usize) -> *const i8 {
        self.s[pos..].as_ptr() as *const i8
    }

    /// Get a mutable reference to the context.
    /// # Safety invariant: self.ctx must be a valid, non-null pointer.
    #[inline]
    pub fn ctx_mut(&mut self) -> &mut xkb_context {
        // SAFETY: scanner.ctx is always set to a valid pointer in scanner::new()
        // and remains valid for the scanner's lifetime.
        unsafe { &mut *self.ctx }
    }

    /// Get a slice of the input from `start` to `end` as bytes.
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

        // Count newlines between cached_pos and token_pos
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

// ── sval comparison functions ──

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
