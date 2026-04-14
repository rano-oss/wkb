//! UTF-8 decoding utilities - fully rustified
/// Invalid UTF-8 code point marker
pub const INVALID_UTF8_CODE_POINT: u32 = u32::MAX;

/// UTF-8 sequence length lookup table (indexed by leading byte)
const UTF8_SEQUENCE_LENGTH_BY_LEADING_BYTE: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Get UTF-8 sequence length from leading byte - safe Rust version
pub fn utf8_sequence_length_safe(leading_byte: u8) -> u8 {
    UTF8_SEQUENCE_LENGTH_BY_LEADING_BYTE[leading_byte as usize]
}

/// Decode next UTF-8 code point from byte slice
/// Returns (code_point, bytes_consumed) or (INVALID_UTF8_CODE_POINT, 0) on error
pub fn utf8_next_code_point_safe(bytes: &[u8]) -> (u32, usize) {
    if bytes.is_empty() {
        return (INVALID_UTF8_CODE_POINT, 0);
    }

    let len = utf8_sequence_length_safe(bytes[0]) as usize;

    if len == 0 || len > bytes.len() {
        return (INVALID_UTF8_CODE_POINT, 0);
    }

    let mut cp: u32 = match len {
        1 => return (bytes[0] as u32, 1),
        2 => (bytes[0] as u32) & 0x1f,
        3 => (bytes[0] as u32) & 0x0f,
        4 => (bytes[0] as u32) & 0x07,
        _ => return (INVALID_UTF8_CODE_POINT, 0),
    };

    // Decode continuation bytes
    for k in 1..len {
        if (bytes[k] & 0xc0) != 0x80 {
            return (INVALID_UTF8_CODE_POINT, 0);
        }
        cp <<= 6;
        cp |= (bytes[k] as u32) & 0x3f;
    }

    // Reject surrogate pairs
    if (0xD800..=0xDFFF).contains(&cp) {
        return (INVALID_UTF8_CODE_POINT, 0);
    }

    (cp, len)
}

// ============================================================================
// FFI wrappers for raw pointer callers (no longer extern "C")
// ============================================================================

pub unsafe fn utf8_next_code_point(s: *const i8, max_size: usize, size_out: *mut usize) -> u32 {
    if s.is_null() || size_out.is_null() || max_size == 0 {
        if !size_out.is_null() {
            unsafe {
                *size_out = 0;
            }
        }
        return INVALID_UTF8_CODE_POINT;
    }

    unsafe {
        let bytes = std::slice::from_raw_parts(s as *const u8, max_size);
        let (cp, len) = utf8_next_code_point_safe(bytes);
        *size_out = len;
        cp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_sequence_length() {
        // 1-byte sequence (ASCII)
        assert_eq!(utf8_sequence_length_safe(b'A'), 1);
        assert_eq!(utf8_sequence_length_safe(0x00), 1);
        assert_eq!(utf8_sequence_length_safe(0x7F), 1);

        // 2-byte sequence
        assert_eq!(utf8_sequence_length_safe(0xC2), 2);
        assert_eq!(utf8_sequence_length_safe(0xDF), 2);

        // 3-byte sequence
        assert_eq!(utf8_sequence_length_safe(0xE0), 3);
        assert_eq!(utf8_sequence_length_safe(0xEF), 3);

        // 4-byte sequence
        assert_eq!(utf8_sequence_length_safe(0xF0), 4);
        assert_eq!(utf8_sequence_length_safe(0xF4), 4);

        // Invalid leading bytes
        assert_eq!(utf8_sequence_length_safe(0x80), 0);
        assert_eq!(utf8_sequence_length_safe(0xFF), 0);
    }

    #[test]
    fn test_utf8_next_code_point_ascii() {
        let data = b"ABC";
        let (cp, len) = utf8_next_code_point_safe(data);
        assert_eq!(cp, b'A' as u32);
        assert_eq!(len, 1);
    }

    #[test]
    fn test_utf8_next_code_point_multibyte() {
        // € symbol (U+20AC) in UTF-8: E2 82 AC
        let data = &[0xE2, 0x82, 0xAC];
        let (cp, len) = utf8_next_code_point_safe(data);
        assert_eq!(cp, 0x20AC);
        assert_eq!(len, 3);

        // 😀 emoji (U+1F600) in UTF-8: F0 9F 98 80
        let data = &[0xF0, 0x9F, 0x98, 0x80];
        let (cp, len) = utf8_next_code_point_safe(data);
        assert_eq!(cp, 0x1F600);
        assert_eq!(len, 4);
    }

    #[test]
    fn test_utf8_next_code_point_invalid() {
        // Invalid continuation byte
        let data = &[0xE2, 0xFF, 0xAC];
        let (cp, len) = utf8_next_code_point_safe(data);
        assert_eq!(cp, INVALID_UTF8_CODE_POINT);
        assert_eq!(len, 0);

        // Truncated sequence
        let data = &[0xE2, 0x82];
        let (cp, len) = utf8_next_code_point_safe(data);
        assert_eq!(cp, INVALID_UTF8_CODE_POINT);
        assert_eq!(len, 0);

        // Empty input
        let data = &[];
        let (cp, len) = utf8_next_code_point_safe(data);
        assert_eq!(cp, INVALID_UTF8_CODE_POINT);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_ffi_wrapper() {
        unsafe {
            let data = b"A\0";
            let mut size_out = 0;
            let cp = utf8_next_code_point(data.as_ptr() as *const i8, 1, &mut size_out);
            assert_eq!(cp, b'A' as u32);
            assert_eq!(size_out, 1);

            // Test null pointer
            let cp = utf8_next_code_point(std::ptr::null(), 1, &mut size_out);
            assert_eq!(cp, INVALID_UTF8_CODE_POINT);
        }
    }
}
