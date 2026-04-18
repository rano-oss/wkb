// Canonical scanner utility types and functions (consolidated from scanner_utils_h blocks)
use crate::xkb::shared_types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sval {
    pub len: usize,
    pub start: *const i8,
}

impl sval {
    pub const EMPTY: sval = sval {
        len: 0,
        start: std::ptr::null(),
    };

    /// View the sval as a byte slice. Caller must ensure the pointer is valid.
    #[inline]
    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe {
            if self.start.is_null() || self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.start as *const u8, self.len)
            }
        }
    }

    /// View the sval as a &str. Caller must ensure the pointer is valid and contents are UTF-8.
    #[inline]
    pub unsafe fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct scanner_loc {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone)]
#[repr(C)]
pub struct scanner {
    pub pos: usize,
    pub len: usize,
    pub s: *const i8,
    pub buf: [u8; 1024],
    pub buf_pos: usize,
    pub token_pos: usize,
    pub cached_pos: usize,
    pub cached_loc: scanner_loc,
    pub file_name: String,
    pub ctx: *mut xkb_context,
    pub priv_0: *mut ::core::ffi::c_void,
}

impl scanner {
    pub fn new(
        ctx: *mut xkb_context,
        s: *const i8,
        len: usize,
        file_name: &str,
        priv_0: *mut ::core::ffi::c_void,
    ) -> Self {
        scanner {
            pos: 0,
            len,
            s,
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
            cached_pos: 0,
            cached_loc: scanner_loc { line: 1, column: 1 },
            file_name: file_name.to_string(),
            ctx,
            priv_0,
        }
    }

    /// The remaining input as a byte slice.
    #[inline]
    unsafe fn remaining(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.s.add(self.pos) as *const u8, self.len - self.pos)
        }
    }

    /// The full input as a byte slice.
    #[inline]
    unsafe fn input(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.s as *const u8, self.len) }
    }

    #[inline]
    pub fn peek(&self) -> i8 {
        if self.pos >= self.len {
            return 0;
        }
        unsafe { *self.s.add(self.pos) }
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.pos >= self.len
    }

    #[inline]
    pub fn eol(&self) -> bool {
        self.peek() == b'\n' as i8
    }

    #[inline]
    pub unsafe fn skip_to_eol(&mut self) {
        unsafe {
            let rem = self.remaining();
            match rem.iter().position(|&b| b == b'\n') {
                Some(i) => self.pos += i,
                None => self.pos = self.len,
            }
        }
    }

    #[inline]
    pub fn next_byte(&mut self) -> i8 {
        if self.pos >= self.len {
            return 0;
        }
        let c = unsafe { *self.s.add(self.pos) };
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
    pub unsafe fn str_match(&mut self, string: *const i8, len: usize) -> bool {
        unsafe {
            if self.len - self.pos < len {
                return false;
            }
            let input = std::slice::from_raw_parts(self.s.add(self.pos) as *const u8, len);
            let target = std::slice::from_raw_parts(string as *const u8, len);
            if input != target {
                return false;
            }
            self.pos += len;
            true
        }
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
    pub unsafe fn buf_appends(&mut self, s: *const u8) -> bool {
        unsafe {
            let mut i = 0usize;
            while *s.add(i) != 0 {
                if !self.buf_append(*s.add(i)) {
                    return false;
                }
                i += 1;
            }
            true
        }
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
    pub unsafe fn dec_int64(&mut self, out: *mut i64) -> i32 {
        unsafe {
            let remaining =
                std::slice::from_raw_parts(self.s.add(self.pos) as *const u8, self.len - self.pos);
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
    }

    #[inline]
    pub unsafe fn hex_int64(&mut self, out: *mut i64) -> i32 {
        unsafe {
            let remaining =
                std::slice::from_raw_parts(self.s.add(self.pos) as *const u8, self.len - self.pos);
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
    }

    #[inline]
    pub unsafe fn unicode_code_point(&mut self, out: *mut u32) -> bool {
        unsafe {
            if !self.chr(b'{' as i8) {
                return false;
            }
            let remaining =
                std::slice::from_raw_parts(self.s.add(self.pos) as *const u8, self.len - self.pos);
            let (cp, count) = crate::xkb::utils::parse_hex_u32(remaining);
            if count > 0 {
                self.pos += count as usize;
            }
            let last_valid = self.pos;
            while !self.eof()
                && !self.eol()
                && self.peek() != b'"' as i8
                && self.peek() != b'}' as i8
            {
                self.next_byte();
            }
            if self.chr(b'}' as i8) {
                *out = cp;
                return count > 0 && self.pos == last_valid + 1 && cp <= 0x10ffff;
            }
            self.pos = last_valid;
            false
        }
    }

    #[inline]
    pub unsafe fn check_supported_char_encoding(&mut self) -> bool {
        use crate::xkb::messages::XKB_ERROR_INVALID_FILE_ENCODING;
        unsafe {
            if self.str_match(b"\xEF\xBB\xBF\0".as_ptr() as *const i8, 3) || self.len < 2 {
                return true;
            }
            if *self.s == 0 || *self.s.add(1) == 0 {
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
            if !(*self.s as u8).is_ascii() {
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
        let input = unsafe { self.input() };
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
pub unsafe fn svaleq(s1: sval, s2: sval) -> bool {
    unsafe { s1.as_bytes() == s2.as_bytes() }
}

#[inline]
pub unsafe fn svaleq_prefix(s1: sval, s2: sval) -> bool {
    unsafe {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        b1.len() <= b2.len() && b1 == &b2[..b1.len()]
    }
}

#[inline]
pub unsafe fn isvaleq(s1: sval, s2: sval) -> bool {
    unsafe { s1.len == s2.len && s1.as_bytes().eq_ignore_ascii_case(s2.as_bytes()) }
}
