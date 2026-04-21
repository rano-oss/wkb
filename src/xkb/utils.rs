use crate::xkb::shared_types::xkb_error_code;

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

/// Convert a `*const i8` C string to `&[u8]` (without the null terminator).
/// # Safety: `s` must point to a valid null-terminated C string.
#[inline]
pub unsafe fn cstr_as_bytes<'a>(s: *const i8) -> &'a [u8] {
    unsafe { std::ffi::CStr::from_ptr(s).to_bytes() }
}

// New Rust file utilities

/// Returns the last OS error number (replacement for `*__errno_location()`).
pub fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

use memmap2::Mmap;
use std::fs::File;
use std::io;

/// Memory-mapped file wrapper with automatic cleanup
pub struct MappedFile {
    mmap: Mmap,
}

impl MappedFile {
    /// Create a new memory-mapped file
    pub fn new(file: &File) -> io::Result<Self> {
        let mmap = unsafe { Mmap::map(file)? };
        Ok(MappedFile { mmap })
    }

    /// Get the mapped data as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    /// Get the mapped data as a C string pointer (for FFI compatibility)
    pub fn as_ptr(&self) -> *const i8 {
        self.mmap.as_ptr() as *const i8
    }

    /// Get the size of the mapped file
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    /// Check if the mapped file is empty
    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }
}

// --- libc replacement helpers ---

/// Safe replacement for libc `strlen`. Returns the length of a C string.
/// # Safety: `s` must point to a valid null-terminated C string.
#[inline]
pub unsafe fn cstr_len(s: *const i8) -> usize {
    unsafe { std::ffi::CStr::from_ptr(s).to_bytes().len() }
}

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

/// Safe replacement for libc `strdup`. Duplicates a C string using Rust's allocator.
/// Returns a `*mut i8` allocated via `CString::into_raw()`.
/// Returns null if `s` is null.
///
/// # Safety
/// If non-null, `s` must point to a valid null-terminated C string.
///
/// # Deallocation
/// The returned pointer should be freed with `drop(CString::from_raw(ptr))`,
/// or with `free()` on Linux where the default global allocator is the system allocator.
#[inline]
pub unsafe fn cstr_dup(s: *const i8) -> *mut i8 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = std::ffi::CStr::from_ptr(s);
    std::ffi::CString::from(cstr).into_raw()
}

/// Frees a C string that was allocated by `cstr_dup`.
/// Safe to call with null (no-op). Counterpart to `CString::into_raw()`.
///
/// # Safety
/// `s` must be null or a pointer previously returned by `cstr_dup`.
#[inline]
pub unsafe fn cstr_free(s: *mut i8) {
    if !s.is_null() {
        drop(std::ffi::CString::from_raw(s));
    }
}

/// Like C `stpcpy`: copy C string `src` to `dst`, return pointer to the NUL terminator in dst.
///
/// # Safety
/// `dst` must have enough space for the copy. Both must be valid C strings.
pub unsafe fn cstr_pcpy(dst: *mut i8, src: *const i8) -> *mut i8 {
    if src.is_null() || dst.is_null() {
        return dst;
    }
    let mut d = dst;
    let mut s = src;
    while *s != 0 {
        *d = *s;
        d = d.offset(1);
        s = s.offset(1);
    }
    *d = 0;
    d
}

/// Safe wrapper around C `strerror`. Returns a display-able error message.

pub struct StrerrorDisplay(pub i32);

impl core::fmt::Display for StrerrorDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", std::io::Error::from_raw_os_error(self.0))
    }
}

// ── utils_h functions (moved from duplicated pub mod utils_h blocks) ─

pub fn is_absolute_path_bytes(path: &[u8]) -> bool {
    path.first() == Some(&b'/')
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
