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
        // € (U+20AC): E2 82 AC
        assert_eq!(utf8_next_code_point_safe(&[0xE2, 0x82, 0xAC]), (0x20AC, 3));
        // 😀 (U+1F600): F0 9F 98 80
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
