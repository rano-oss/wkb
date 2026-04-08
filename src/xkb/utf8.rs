//! Native Rust UTF-8 utilities
//!
//! This module provides UTF-8 validation and conversion utilities.
//! Converted from C FFI to idiomatic Rust.

/// Check if a Unicode codepoint is a UTF-16 surrogate
///
/// Surrogates (U+D800 to U+DFFF) are not valid Unicode scalar values
/// and cannot be encoded in UTF-8.
#[inline]
pub fn is_surrogate(cp: u32) -> bool {
    (0xD800..=0xDFFF).contains(&cp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_surrogate() {
        assert!(!is_surrogate(0xD7FF)); // Just before range
        assert!(is_surrogate(0xD800)); // Start of range
        assert!(is_surrogate(0xDFFF)); // End of range
        assert!(!is_surrogate(0xE000)); // Just after range
        assert!(!is_surrogate(0x0041)); // ASCII 'A'
    }
}
