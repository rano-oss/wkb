/// Case-insensitive comparison of two byte slices (like C `strcasecmp`).
/// Returns <0, 0, or >0.
#[inline]
pub fn istrcmp(a: &[u8], b: &[u8]) -> i32 {
    let n = a.len().min(b.len());
    for i in 0..n {
        let al = a[i].to_ascii_lowercase();
        let bl = b[i].to_ascii_lowercase();
        if al != bl {
            return al as i32 - bl as i32;
        }
    }
    // shorter slice is "less than" longer
    (a.len() as i32) - (b.len() as i32)
}

// New Rust file utilities

// --- libc replacement helpers ---

/// Stack buffer writer implementing `core::fmt::Write`.
/// Used by the `xkb_logf!` and `rxkb_logf!` macros to replace C `snprintf`.
pub struct LogBuf<'a> {
    buf: &'a mut [u8],
    pub pos: usize,
    pub truncated: bool,
}

impl<'a> LogBuf<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            truncated: false,
        }
    }
}

impl<'a> core::fmt::Write for LogBuf<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let space = self.buf.len() - self.pos;
        let n = bytes.len().min(space);
        if n < bytes.len() {
            self.truncated = true;
        }
        self.buf[self.pos..self.pos + n].copy_from_slice(&bytes[..n]);
        self.pos += n;
        Ok(())
    }
}

// === Number parsing utilities ===
// Safe Rust replacements for C-style parse functions.
// Return count of bytes consumed (positive) or -1 on overflow.

/// Parse decimal digits from a byte slice into u32.
/// Returns (value, count). count > 0 on success, -1 on overflow.
pub fn parse_dec_u32(s: &[u8]) -> (u32, i32) {
    let mut result: u32 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = s[i].wrapping_sub(b'0');
        if d >= 10 {
            break;
        }
        // overflow check
        if result > u32::MAX / 10 || result * 10 > u32::MAX - d as u32 {
            // there are more digits but we can't fit them
            return (result, -1);
        }
        result = result * 10 + d as u32;
        i += 1;
    }
    // If we stopped before the end AND the next char is still a digit, overflow
    if i < s.len() && s[i].wrapping_sub(b'0') < 10 {
        return (result, -1);
    }
    (result, i as i32)
}

/// Parse decimal digits from a byte slice into u64.
/// Returns (value, count). count > 0 on success, -1 on overflow.
pub fn parse_dec_u64(s: &[u8]) -> (u64, i32) {
    let mut result: u64 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = s[i].wrapping_sub(b'0');
        if d >= 10 {
            break;
        }
        if result > u64::MAX / 10 || result * 10 > u64::MAX - d as u64 {
            return (result, -1);
        }
        result = result * 10 + d as u64;
        i += 1;
    }
    if i < s.len() && s[i].wrapping_sub(b'0') < 10 {
        return (result, -1);
    }
    (result, i as i32)
}

// ── UTF-8 decoding (migrated from utf8_decoding.rs) ──

/// Invalid UTF-8 code point marker
pub const INVALID_UTF8_CODE_POINT: u32 = u32::MAX;

/// Decode next UTF-8 code point from byte slice.
/// Returns (code_point, bytes_consumed) or (INVALID_UTF8_CODE_POINT, 0) on error.
pub fn utf8_next_code_point_safe(bytes: &[u8]) -> (u32, usize) {
    if bytes.is_empty() {
        return (INVALID_UTF8_CODE_POINT, 0);
    }
    // Determine sequence length from leading byte
    let b0 = bytes[0];
    let (len, mut cp) = match b0 {
        0x00..=0x7F => return (b0 as u32, 1),
        0xC2..=0xDF => (2, (b0 as u32) & 0x1F),
        0xE0..=0xEF => (3, (b0 as u32) & 0x0F),
        0xF0..=0xF4 => (4, (b0 as u32) & 0x07),
        _ => return (INVALID_UTF8_CODE_POINT, 0),
    };
    if len > bytes.len() {
        return (INVALID_UTF8_CODE_POINT, 0);
    }
    for &byte in bytes.iter().take(len).skip(1) {
        if (byte & 0xC0) != 0x80 {
            return (INVALID_UTF8_CODE_POINT, 0);
        }
        cp = (cp << 6) | ((byte as u32) & 0x3F);
    }
    // Reject overlong encodings and surrogates
    if (len == 2 && cp < 0x80)
        || (len == 3 && cp < 0x800)
        || (len == 4 && cp < 0x10000)
        || (0xD800..=0xDFFF).contains(&cp)
        || cp > 0x10FFFF
    {
        return (INVALID_UTF8_CODE_POINT, 0);
    }
    (cp, len)
}

/// Convert a hex digit byte to its numeric value (0-15), or 0xff if invalid.
#[inline]
fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'F' => b - b'A' + 10,
        b'a'..=b'f' => b - b'a' + 10,
        _ => 0xff,
    }
}

/// Parse hex digits from a byte slice into u32.
/// Returns (value, count). count > 0 on success, -1 on overflow.
pub fn parse_hex_u32(s: &[u8]) -> (u32, i32) {
    let mut result: u32 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = hex_val(s[i]);
        if d >= 16 {
            break;
        }
        if result > u32::MAX >> 4 {
            return (result, -1);
        }
        result = result * 16 + d as u32;
        i += 1;
    }
    if i < s.len() && hex_val(s[i]) < 16 {
        return (result, -1);
    }
    (result, i as i32)
}

/// Parse hex digits from a byte slice into u64.
/// Returns (value, count). count > 0 on success, -1 on overflow.
pub fn parse_hex_u64(s: &[u8]) -> (u64, i32) {
    let mut result: u64 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = hex_val(s[i]);
        if d >= 16 {
            break;
        }
        if result > u64::MAX >> 4 {
            return (result, -1);
        }
        result = result * 16 + d as u64;
        i += 1;
    }
    if i < s.len() && hex_val(s[i]) < 16 {
        return (result, -1);
    }
    (result, i as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_next_code_point_ascii() {
        let (cp, len) = utf8_next_code_point_safe(b"ABC");
        assert_eq!((cp, len), (b'A' as u32, 1));
    }

    #[test]
    fn test_utf8_next_code_point_multibyte() {
        assert_eq!(utf8_next_code_point_safe(&[0xE2, 0x82, 0xAC]), (0x20AC, 3));
        assert_eq!(
            utf8_next_code_point_safe(&[0xF0, 0x9F, 0x98, 0x80]),
            (0x1F600, 4)
        );
    }

    #[test]
    fn test_utf8_next_code_point_invalid() {
        assert_eq!(
            utf8_next_code_point_safe(&[0xE2, 0xFF, 0xAC]),
            (INVALID_UTF8_CODE_POINT, 0)
        );
        assert_eq!(
            utf8_next_code_point_safe(&[0xE2, 0x82]),
            (INVALID_UTF8_CODE_POINT, 0)
        );
        assert_eq!(utf8_next_code_point_safe(&[]), (INVALID_UTF8_CODE_POINT, 0));
    }
}
