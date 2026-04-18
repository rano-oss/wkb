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
#[inline]
pub unsafe fn _steal(ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
        let swapped: *mut ::core::ffi::c_void = *original;
        *original = std::ptr::null_mut::<core::ffi::c_void>();
        swapped
    }
}

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

/// Safe replacement for libc `strcmp`. Returns <0, 0, or >0.
/// # Safety: both pointers must point to valid null-terminated C strings.
#[inline]
pub unsafe fn cstr_cmp(s1: *const i8, s2: *const i8) -> i32 {
    unsafe {
        let a = std::ffi::CStr::from_ptr(s1);
        let b = std::ffi::CStr::from_ptr(s2);
        a.cmp(b) as i32
    }
}

/// Stack buffer writer implementing `core::fmt::Write`.
/// Used by the `xkb_logf!` and `rxkb_logf!` macros and `snprintf_args` to replace C `snprintf`.
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

/// Write formatted args into a raw C buffer, NUL-terminated.
/// Returns (bytes_written, truncated). The buffer is always NUL-terminated if size > 0.
/// On truncation, writes as much as fits and sets truncated=true.
///
/// This replaces C `snprintf` and `snprintf_safe`.
///
/// # Safety
/// `buf` must point to at least `size` writable bytes.
#[inline]
pub unsafe fn snprintf_args(
    buf: *mut i8,
    size: usize,
    args: core::fmt::Arguments<'_>,
) -> (usize, bool) {
    if size == 0 {
        return (0, true);
    }
    let slice = std::slice::from_raw_parts_mut(buf as *mut u8, size);
    let mut w = LogBuf::new(&mut slice[..size - 1]);
    let _ = core::fmt::Write::write_fmt(&mut w, args);
    let pos = w.pos;
    let truncated = w.truncated;
    slice[pos] = 0; // NUL terminate
    (pos, truncated)
}

/// Like C `snprintf`: format into buffer, return the total formatted length
/// (even if truncated). Always NUL-terminates if size > 0.
/// Returns -1 on error (never happens with Rust formatting).
///
/// Use this when callers need the would-be length (e.g. public API return values).
/// For internal use where you just need truncation detection, use `snprintf_args`.
///
/// # Safety
/// `buf` must point to `size` writable bytes.
pub unsafe fn snprintf_c(buf: *mut i8, size: usize, args: core::fmt::Arguments<'_>) -> i32 {
    use core::fmt::Write;
    // Format into a String to get the true full length
    let mut s = String::new();
    let _ = Write::write_fmt(&mut s, args);
    let full_len = s.len();
    if size > 0 {
        let copy_len = full_len.min(size - 1);
        std::ptr::copy_nonoverlapping(s.as_ptr(), buf as *mut u8, copy_len);
        *(buf as *mut u8).add(copy_len) = 0;
    }
    full_len as i32
}

/// Display wrapper for precision-limited C strings (`%.*s` replacement).
/// Reads at most `len` bytes from the pointer, stopping at NUL.
pub struct CStrNDisplay(pub usize, pub *const i8);

impl core::fmt::Display for CStrNDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.1.is_null() {
            return f.write_str("(null)");
        }
        unsafe {
            let ptr = self.1 as *const u8;
            let mut len = 0;
            while len < self.0 && *ptr.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(ptr, len);
            match core::str::from_utf8(slice) {
                Ok(s) => f.write_str(s),
                Err(_) => f.write_str("<invalid utf8>"),
            }
        }
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

/// Safe replacement for `strndup`. Duplicates up to `n` bytes of a C string
/// using Rust's allocator. Returns a `*mut i8` allocated via `CString::into_raw()`.
/// Returns null if `s` is null.
///
/// # Safety
/// If non-null, `s` must point to at least `n` readable bytes.
#[inline]
pub unsafe fn cstr_ndup(s: *const i8, n: usize) -> *mut i8 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    // Find actual length (stop at null or n bytes)
    let mut len = 0;
    while len < n && *s.add(len) != 0 {
        len += 1;
    }
    let slice = std::slice::from_raw_parts(s as *const u8, len);
    match std::ffi::CString::new(slice) {
        Ok(cstring) => cstring.into_raw(),
        Err(_) => std::ptr::null_mut(), // interior null (shouldn't happen since we stopped at null)
    }
}

/// Frees a C string that was allocated by `cstr_dup`, `cstr_ndup`, or `strdup_safe`.
/// Safe to call with null (no-op). Counterpart to `CString::into_raw()`.
///
/// # Safety
/// `s` must be null or a pointer previously returned by `cstr_dup`/`cstr_ndup`/`strdup_safe`.
#[inline]
pub unsafe fn cstr_free(s: *mut i8) {
    if !s.is_null() {
        drop(std::ffi::CString::from_raw(s));
    }
}

/// Like C `strchr`: find first occurrence of byte `c` in C string `s`.
/// Returns pointer to the byte, or null if not found.
///
/// # Safety
/// `s` must point to a valid NUL-terminated C string.
pub unsafe fn cstr_chr(s: *const i8, c: i32) -> *mut i8 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = s;
    let target = c as i8;
    loop {
        if *p == target {
            return p as *mut i8;
        }
        if *p == 0 {
            return std::ptr::null_mut();
        }
        p = p.offset(1);
    }
}

/// Like C `strpbrk`: find first occurrence of any byte from `accept` in C string `s`.
/// Returns pointer to the byte, or null if not found.
///
/// # Safety
/// `s` and `accept` must point to valid NUL-terminated C strings.
pub unsafe fn cstr_pbrk(s: *const i8, accept: *const i8) -> *mut i8 {
    if s.is_null() || accept.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = s;
    while *p != 0 {
        let mut a = accept;
        while *a != 0 {
            if *p == *a {
                return p as *mut i8;
            }
            a = a.offset(1);
        }
        p = p.offset(1);
    }
    std::ptr::null_mut()
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

#[inline]
pub unsafe fn strdup_safe(s: *const i8) -> *mut i8 {
    cstr_dup(s)
}

#[inline]
pub unsafe fn isempty(s: *const i8) -> bool {
    s.is_null() || *s == 0
}

/// Null-safe case-sensitive equality of two C strings.
#[inline]
pub unsafe fn streq_null(s1: *const i8, s2: *const i8) -> bool {
    if s1.is_null() || s2.is_null() {
        return s1 == s2;
    }
    unsafe { cstr_as_bytes(s1) == cstr_as_bytes(s2) }
}

/// Both-non-null case-sensitive equality of two C strings.
#[inline]
pub unsafe fn streq_not_null(s1: *const i8, s2: *const i8) -> bool {
    if s1.is_null() || s2.is_null() {
        return false;
    }
    unsafe { cstr_as_bytes(s1) == cstr_as_bytes(s2) }
}

pub unsafe fn is_absolute_path(path: *const i8) -> bool {
    !path.is_null() && *path == b'/' as i8
}

pub unsafe fn xkb_check_versioned_struct_size_(
    v1_size: usize,
    min_size: usize,
    lib_size: usize,
    caller_size: usize,
    caller_data: *const ::core::ffi::c_void,
) -> xkb_error_code {
    use crate::xkb::shared_types::{
        XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
        XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_SUCCESS,
    };
    unsafe {
        if caller_size < v1_size {
            return XKB_ERROR_ABI_INVALID_STRUCT_SIZE;
        }
        if caller_size < min_size {
            return XKB_ERROR_ABI_BACKWARD_COMPAT;
        }
        if caller_size <= lib_size {
            return XKB_SUCCESS;
        }
        let mut p: *const ::core::ffi::c_uchar =
            (caller_data as *const ::core::ffi::c_uchar).add(lib_size);
        let end: *const ::core::ffi::c_uchar =
            (caller_data as *const ::core::ffi::c_uchar).add(caller_size);
        while p < end {
            let c2rust_fresh1 = p;
            p = p.offset(1);
            if *c2rust_fresh1 != 0 {
                return XKB_ERROR_ABI_FORWARD_COMPAT;
            }
        }
        XKB_SUCCESS
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

/// Parse leading decimal integer from a C string.
/// Returns `(value, bytes_consumed)`. On empty/non-digit input returns `(0, 0)`.
pub unsafe fn cstr_parse_long(s: *const i8) -> (i64, usize) {
    unsafe {
        if s.is_null() {
            return (0, 0);
        }
        let mut i: usize = 0;
        let neg = *s.offset(0) as u8 == b'-';
        let pos = *s.offset(0) as u8 == b'+';
        if neg || pos {
            i = 1;
        }
        let start = i;
        let mut result: i64 = 0;
        while (*s.add(i) as u8).is_ascii_digit() {
            result = result
                .wrapping_mul(10)
                .wrapping_add((*s.add(i) as u8 - b'0') as i64);
            i += 1;
        }
        if i == start {
            return (0, 0); // no digits consumed
        }
        if neg {
            result = -result;
        }
        (result, i)
    }
}

/// Simple atoi replacement: parse entire C string as decimal i32, return 0 on failure.
pub unsafe fn cstr_atoi(s: *const i8) -> i32 {
    unsafe { cstr_parse_long(s).0 as i32 }
}
