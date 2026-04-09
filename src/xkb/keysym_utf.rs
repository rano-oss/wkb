//! Native Rust keysym to UTF-8/UTF-32 conversion
//!
//! Converted from C FFI to idiomatic Rust.

use super::utf8::is_surrogate;

// Re-export type alias
pub type Keysym = u32;

// XKB key symbol constants
pub const XKB_KEY_NO_SYMBOL: u32 = 0;
pub const XKB_KEY_BACKSPACE: u32 = 0xff08;
pub const XKB_KEY_CLEAR: u32 = 0xff0b;
pub const XKB_KEY_RETURN: u32 = 0xff0d;
pub const XKB_KEY_ESCAPE: u32 = 0xff1b;
pub const XKB_KEY_DELETE: u32 = 0xffff;
pub const XKB_KEY_KP_SPACE: u32 = 0xff80;
pub const XKB_KEY_KP_TAB: u32 = 0xff89;
pub const XKB_KEY_KP_ENTER: u32 = 0xff8d;
pub const XKB_KEY_KP_EQUAL: u32 = 0xffbd;
pub const XKB_KEY_KP_MULTIPLY: u32 = 0xffaa;
pub const XKB_KEY_KP_9: u32 = 0xffb9;
pub const XKB_KEY_SPACE: u32 = 0x20;
pub const XKB_KEY_XF86_NUMERIC_0: u32 = 0x10081200;
pub const XKB_KEY_XF86_NUMERIC_9: u32 = 0x10081209;
pub const XKB_KEY_XF86_NUMERIC_STAR: u32 = 268964362;
pub const XKB_KEY_XF86_NUMERIC_POUND: u32 = 268964363;

pub const XKB_KEYSYM_UNICODE_OFFSET: u32 = 0x1000000;
pub const XKB_KEYSYM_UNICODE_SURROGATE_MIN: u32 = 0x100d800;
pub const XKB_KEYSYM_UNICODE_SURROGATE_MAX: u32 = 0x100dfff;
pub const XKB_KEYSYM_UNICODE_MAX: u32 = 0x110ffff;

/// Keysym lookup table entry
#[derive(Copy, Clone, Debug)]
struct KeysymEntry {
    keysym: u16,
    ucs: u16,
    deprecated: bool,
}

// Include the static lookup table
// TODO: This is generated from the old C code - can be cleaned up later
const KEYSYM_TABLE: &[KeysymEntry] = &include!("keysym_table_data.rs");

/// Binary search the keysym lookup table
fn bin_search_keysym(keysym: u32) -> Option<u32> {
    let keysym16 = if keysym <= 0xFFFF {
        keysym as u16
    } else {
        return None;
    };

    let idx = KEYSYM_TABLE
        .binary_search_by_key(&keysym16, |entry| entry.keysym)
        .ok()?;
    Some(KEYSYM_TABLE[idx].ucs as u32)
}

/// Convert an XKB keysym to a Unicode character
///
/// Returns `None` if the keysym cannot be converted to Unicode.
pub fn keysym_to_char(keysym: Keysym) -> Option<char> {
    let codepoint = keysym_to_codepoint(keysym)?;
    char::from_u32(codepoint)
}

/// Convert an XKB keysym to a Unicode codepoint (U+XXXX)
///
/// Returns `None` if the keysym cannot be converted.
fn keysym_to_codepoint(keysym: Keysym) -> Option<u32> {
    // ASCII printable and Latin-1
    if (0x20..=0x7E).contains(&keysym) || (0xA0..=0xFF).contains(&keysym) {
        return Some(keysym);
    }

    // Keypad space maps to regular space
    if keysym == XKB_KEY_KP_SPACE {
        return Some(XKB_KEY_SPACE & 0x7F);
    }

    // Special keys that map to ASCII control characters
    if (XKB_KEY_BACKSPACE..=XKB_KEY_CLEAR).contains(&keysym)
        || (XKB_KEY_KP_MULTIPLY..=XKB_KEY_KP_9).contains(&keysym)
        || keysym == XKB_KEY_RETURN
        || keysym == XKB_KEY_ESCAPE
        || keysym == XKB_KEY_DELETE
        || keysym == XKB_KEY_KP_TAB
        || keysym == XKB_KEY_KP_ENTER
        || keysym == XKB_KEY_KP_EQUAL
    {
        return Some(keysym & 0x7F);
    }

    // Reject surrogate pairs
    if (XKB_KEYSYM_UNICODE_SURROGATE_MIN..=XKB_KEYSYM_UNICODE_SURROGATE_MAX).contains(&keysym) {
        return None;
    }

    // Unicode keysyms
    if (XKB_KEYSYM_UNICODE_OFFSET..=XKB_KEYSYM_UNICODE_MAX).contains(&keysym) {
        return Some(keysym - XKB_KEYSYM_UNICODE_OFFSET);
    }

    // XF86 numeric keys
    if (XKB_KEY_XF86_NUMERIC_0..=XKB_KEY_XF86_NUMERIC_9).contains(&keysym) {
        return Some(keysym - XKB_KEY_XF86_NUMERIC_0 + 0x30); // '0' to '9'
    }

    // Special XF86 keys
    match keysym {
        XKB_KEY_XF86_NUMERIC_STAR => return Some(0x2A), // '*'
        XKB_KEY_XF86_NUMERIC_POUND => return Some(0x23), // '#'
        _ => {}
    }

    // Look up in the static table
    bin_search_keysym(keysym)
}

/// Convert a Unicode character to an XKB keysym
///
/// Returns `None` if the character has no corresponding keysym.
pub fn char_to_keysym(ch: char) -> Option<Keysym> {
    codepoint_to_keysym(ch as u32)
}

/// Convert a Unicode codepoint to an XKB keysym
fn codepoint_to_keysym(ucs: u32) -> Option<Keysym> {
    // ASCII printable and Latin-1
    if (0x20..=0x7E).contains(&ucs) || (0xA0..=0xFF).contains(&ucs) {
        return Some(ucs);
    }

    // Special control characters
    if (0x08..=0x0B).contains(&ucs) // BackSpace to Clear
        || ucs == 0x0D // Return
        || ucs == 0x1B
    // Escape
    {
        return Some(ucs | 0xFF00);
    }

    // Delete
    if ucs == 0x7F {
        return Some(XKB_KEY_DELETE);
    }

    // Invalid codepoints
    if ucs == 0 || is_surrogate(ucs) || ucs > 0x10FFFF {
        return None;
    }

    // Search the table for non-deprecated entries
    for entry in KEYSYM_TABLE {
        if entry.ucs as u32 == ucs && !entry.deprecated {
            return Some(entry.keysym as u32);
        }
    }

    // Fallback: encode as Unicode keysym
    Some(ucs | XKB_KEYSYM_UNICODE_OFFSET)
}

/// Convert a keysym to a UTF-8 string
///
/// Returns the UTF-8 encoded string, or an empty string if conversion fails.
pub fn keysym_to_utf8(keysym: Keysym) -> String {
    keysym_to_char(keysym)
        .map(|ch| ch.to_string())
        .unwrap_or_default()
}

// ====================================================================================
// FFI compatibility layer for internal C code (state.rs, scanner.rs, etc.)
// These wrappers maintain the old C API while using native Rust implementation
// ====================================================================================

/// FFI wrapper: Convert keysym to UTF-32 codepoint (C-compatible)
///
/// Returns 0 if conversion fails (matches old C behavior)
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_to_utf32(keysym: u32) -> u32 {
    keysym_to_codepoint(keysym).unwrap_or(0)
}

/// FFI wrapper: Convert keysym to UTF-8 string (C-compatible)
///
/// Writes UTF-8 bytes to `buffer` (must have space for at least 5 bytes).
/// Returns number of bytes written (including null terminator), or:
/// - 0 if conversion fails
/// - -1 if buffer too small
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_to_utf8(
    keysym: u32,
    buffer: *mut i8,
    size: usize,
) -> ::core::ffi::c_int {
    const MAX_UTF8_SIZE: usize = 5;

    if size < MAX_UTF8_SIZE {
        return -1;
    }

    if buffer.is_null() {
        return -1;
    }

    let Some(ch) = keysym_to_char(keysym) else {
        return 0;
    };

    // Encode to UTF-8
    let mut tmp = [0u8; 4];
    let utf8_bytes = ch.encode_utf8(&mut tmp).as_bytes();

    // Copy to buffer
    unsafe {
        std::ptr::copy_nonoverlapping(utf8_bytes.as_ptr(), buffer as *mut u8, utf8_bytes.len());
        // Null terminate
        *buffer.add(utf8_bytes.len()) = 0;
    }

    // Return length + 1 for null terminator (matches old C behavior)
    (utf8_bytes.len() + 1) as ::core::ffi::c_int
}

/// FFI wrapper: Convert UTF-32 codepoint to keysym (C-compatible)
///
/// Returns 0 (XKB_KEY_NO_SYMBOL) if conversion fails
#[no_mangle]
pub unsafe extern "C" fn xkb_utf32_to_keysym(ucs: u32) -> u32 {
    codepoint_to_keysym(ucs).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_conversion() {
        assert_eq!(keysym_to_char(0x41), Some('A'));
        assert_eq!(keysym_to_char(0x20), Some(' '));
        assert_eq!(char_to_keysym('A'), Some(0x41));
    }

    #[test]
    fn test_special_keys() {
        assert_eq!(keysym_to_char(XKB_KEY_RETURN), Some('\r'));
        assert_eq!(keysym_to_char(XKB_KEY_ESCAPE), Some('\x1B'));
    }

    #[test]
    fn test_unicode_keysyms() {
        // U+1F600 GRINNING FACE
        let keysym = 0x1000000 + 0x1F600;
        assert_eq!(keysym_to_char(keysym), Some('😀'));
    }

    #[test]
    fn test_invalid_keysyms() {
        assert_eq!(keysym_to_char(0), None);
        assert_eq!(keysym_to_char(XKB_KEYSYM_UNICODE_SURROGATE_MIN), None);
    }

    #[test]
    fn test_ffi_compat() {
        // Test xkb_keysym_to_utf32
        unsafe {
            assert_eq!(xkb_keysym_to_utf32(0x41), 0x41); // 'A'
            assert_eq!(xkb_keysym_to_utf32(0), 0); // Invalid
        }

        // Test xkb_keysym_to_utf8
        unsafe {
            let mut buf = [0i8; 10];
            let len = xkb_keysym_to_utf8(0x41, buf.as_mut_ptr(), 10);
            assert_eq!(len, 2); // 'A' + null = 2 bytes
            assert_eq!(buf[0], b'A' as i8);
            assert_eq!(buf[1], 0);
        }
    }
}
