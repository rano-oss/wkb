pub const MAP_FAILED: *mut ::core::ffi::c_void = -1 as i32 as *mut ::core::ffi::c_void;

extern "C" {
    pub fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: usize,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: i64,
    ) -> *mut ::core::ffi::c_void;
    pub fn munmap(__addr: *mut ::core::ffi::c_void, __len: usize) -> i32;
}
pub const PROT_READ: i32 = 0x1 as i32;
pub const MAP_SHARED: i32 = 0x1 as i32;

pub use crate::xkb::shared_types::timespec;
use crate::xkb::shared_types::xkb_error_code;
pub use crate::xkb::shared_types::__S_IFMT;
pub unsafe fn open_file(mut path: *const i8) -> *mut FILE {
    unsafe {
        if path.is_null() {
            return std::ptr::null_mut();
        }
        let mut fd: i32 = open(path, O_RDONLY);
        if fd < 0 as i32 {
            return std::ptr::null_mut();
        }
        let mut stat_buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut err: i32 = fstat(fd, &raw mut stat_buf);
        if err != 0 as i32 || !(stat_buf.st_mode & __S_IFMT as u32 == 0o100000 as u32) {
            close(fd);
            return std::ptr::null_mut();
        }
        let mut fp: *mut FILE = fdopen(fd, b"rb\0".as_ptr() as *const i8);
        if fp.is_null() {
            close(fd);
        }
        return fp;
    }
}
static LOWER_MAP: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];
pub fn to_lower(mut c: i8) -> i8 {
    return LOWER_MAP[c as usize] as i8;
}
pub unsafe fn istrcmp(mut a: *const i8, mut b: *const i8) -> i32 {
    unsafe {
        let mut i: usize = 0 as usize;
        loop {
            if to_lower(*a.offset(i as isize)) as i32 != to_lower(*b.offset(i as isize)) as i32 {
                return to_lower(*a.offset(i as isize)) as i32
                    - to_lower(*b.offset(i as isize)) as i32;
            }
            if *a.offset(i as isize) == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        return 0 as i32;
    }
}
pub unsafe fn istrncmp(mut a: *const i8, mut b: *const i8, mut n: usize) -> i32 {
    unsafe {
        let mut i: usize = 0 as usize;
        while i < n {
            if to_lower(*a.offset(i as isize)) as i32 != to_lower(*b.offset(i as isize)) as i32 {
                return to_lower(*a.offset(i as isize)) as i32
                    - to_lower(*b.offset(i as isize)) as i32;
            }
            if *a.offset(i as isize) == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        return 0 as i32;
    }
}

// New Rust file utilities
#[inline]
pub unsafe fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
        let mut swapped: *mut ::core::ffi::c_void = *original;
        *original = std::ptr::null_mut::<core::ffi::c_void>();
        return swapped;
    }
}

extern "C" {
    pub fn __errno_location() -> *mut i32;
}

use libc::{fdopen, FILE};
use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::path::Path;

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

/// Open a file and verify it's a regular file
pub fn open_regular_file(path: &Path) -> io::Result<File> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not a regular file",
        ));
    }
    Ok(file)
}

/// Open a file from a C string path
pub unsafe fn open_file_from_cstr(path: *const i8) -> io::Result<File> {
    if path.is_null() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "null path"));
    }
    let path_str = std::ffi::CStr::from_ptr(path)
        .to_str()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid UTF-8 in path"))?;
    open_regular_file(Path::new(path_str))
}

// Safe Rust string utilities

/// Convert a character to lowercase using the same lookup table as the C code
#[inline]
pub fn to_lower_char(c: u8) -> u8 {
    LOWER_MAP[c as usize]
}

/// Case-insensitive string comparison (safe Rust version)
/// Returns 0 if strings are equal, negative if a < b, positive if a > b
pub fn str_case_cmp(a: &str, b: &str) -> i32 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let min_len = a_bytes.len().min(b_bytes.len());

    for i in 0..min_len {
        let diff = to_lower_char(a_bytes[i]) as i32 - to_lower_char(b_bytes[i]) as i32;
        if diff != 0 {
            return diff;
        }
    }

    // If one string is a prefix of the other, the shorter one is "less"
    (a_bytes.len() as i32) - (b_bytes.len() as i32)
}

/// Case-insensitive string equality check (safe Rust version)
pub fn str_case_eq(a: &str, b: &str) -> bool {
    str_case_cmp(a, b) == 0
}

/// Case-insensitive string comparison with length limit (safe Rust version)
/// Returns 0 if first n characters are equal, negative if a < b, positive if a > b
pub fn str_case_ncmp(a: &str, b: &str, n: usize) -> i32 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let min_len = a_bytes.len().min(b_bytes.len()).min(n);

    for i in 0..min_len {
        let diff = to_lower_char(a_bytes[i]) as i32 - to_lower_char(b_bytes[i]) as i32;
        if diff != 0 {
            return diff;
        }
    }

    // If we've compared n chars and they're all equal, strings are equal for this comparison
    if min_len == n {
        return 0;
    }

    // Otherwise, if one string is shorter, it's "less"
    (a_bytes.len() as i32) - (b_bytes.len() as i32)
}

/// Case-insensitive string equality check with length limit (safe Rust version)
pub fn str_case_neq(a: &str, b: &str, n: usize) -> bool {
    str_case_ncmp(a, b, n) == 0
}

// --- libc replacement helpers ---

/// Safe replacement for libc `strlen`. Returns the length of a C string.
/// # Safety: `s` must point to a valid null-terminated C string.
#[inline]
pub unsafe fn cstr_len(s: *const i8) -> usize {
    unsafe { std::ffi::CStr::from_ptr(s).to_bytes().len() }
}

/// Null-safe version of `cstr_len`. Returns 0 if `s` is null.
/// # Safety: if non-null, `s` must point to a valid null-terminated C string.
#[inline]
pub unsafe fn cstr_len_safe(s: *const i8) -> usize {
    unsafe {
        if s.is_null() {
            0
        } else {
            std::ffi::CStr::from_ptr(s).to_bytes().len()
        }
    }
}

/// Safe replacement for libc `strcmp`. Returns <0, 0, or >0.
/// # Safety: both pointers must point to valid null-terminated C strings.
#[inline]
pub unsafe fn cstr_cmp(s1: *const i8, s2: *const i8) -> i32 {
    unsafe {
        let a = std::ffi::CStr::from_ptr(s1);
        let b = std::ffi::CStr::from_ptr(s2);
        a.cmp(&b) as i32
    }
}

/// Safe replacement for libc `strncmp`. Compares at most `n` bytes.
/// # Safety: both pointers must point to valid C strings (or at least `n` readable bytes).
#[inline]
pub unsafe fn cstr_ncmp(s1: *const i8, s2: *const i8, n: usize) -> i32 {
    unsafe {
        let a = std::slice::from_raw_parts(s1 as *const u8, n);
        let b = std::slice::from_raw_parts(s2 as *const u8, n);
        // Compare byte-by-byte, stopping at null like C strncmp
        for i in 0..n {
            if a[i] == 0 && b[i] == 0 {
                return 0;
            }
            if a[i] != b[i] {
                return a[i] as i32 - b[i] as i32;
            }
            if a[i] == 0 || b[i] == 0 {
                return a[i] as i32 - b[i] as i32;
            }
        }
        0
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

/// Display wrapper for `*const i8` C strings in Rust format strings.
/// Replaces `%s` format specifier usage. Reads the C string and writes it as UTF-8.
pub struct CStrDisplay(pub *const i8);

impl core::fmt::Display for CStrDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_null() {
            return f.write_str("(null)");
        }
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(self.0);
            let bytes = cstr.to_bytes();
            match core::str::from_utf8(bytes) {
                Ok(s) => f.write_str(s),
                Err(_) => f.write_str("<invalid utf8>"),
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_case_cmp() {
        assert_eq!(str_case_cmp("hello", "hello"), 0);
        assert_eq!(str_case_cmp("hello", "HELLO"), 0);
        assert_eq!(str_case_cmp("Hello", "hElLo"), 0);
        assert!(str_case_cmp("abc", "abd") < 0);
        assert!(str_case_cmp("abd", "abc") > 0);
        assert!(str_case_cmp("abc", "abcd") < 0);
        assert!(str_case_cmp("abcd", "abc") > 0);
    }

    #[test]
    fn test_str_case_eq() {
        assert!(str_case_eq("hello", "hello"));
        assert!(str_case_eq("hello", "HELLO"));
        assert!(str_case_eq("Hello", "hElLo"));
        assert!(!str_case_eq("abc", "abd"));
        assert!(!str_case_eq("abc", "abcd"));
    }

    #[test]
    fn test_str_case_ncmp() {
        assert_eq!(str_case_ncmp("hello", "hello", 5), 0);
        assert_eq!(str_case_ncmp("hello", "HELLO", 5), 0);
        assert_eq!(str_case_ncmp("helloworld", "HELLO", 5), 0);
        assert_eq!(str_case_ncmp("hello", "helloworld", 5), 0);
        assert!(str_case_ncmp("abc", "abd", 3) < 0);
        assert!(str_case_ncmp("abd", "abc", 3) > 0);
        assert_eq!(str_case_ncmp("abc", "abd", 2), 0);
    }

    #[test]
    fn test_str_case_neq() {
        assert!(str_case_neq("hello", "hello", 5));
        assert!(str_case_neq("hello", "HELLO", 5));
        assert!(str_case_neq("helloworld", "HELLO", 5));
        assert!(str_case_neq("hello", "helloworld", 5));
        assert!(!str_case_neq("abc", "abd", 3));
        assert!(str_case_neq("abc", "abd", 2));
    }

    #[test]
    fn test_to_lower_char() {
        assert_eq!(to_lower_char(b'A'), b'a');
        assert_eq!(to_lower_char(b'Z'), b'z');
        assert_eq!(to_lower_char(b'a'), b'a');
        assert_eq!(to_lower_char(b'z'), b'z');
        assert_eq!(to_lower_char(b'0'), b'0');
        assert_eq!(to_lower_char(b'9'), b'9');
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
        extern "C" {
            fn strerror(__errnum: i32) -> *mut i8;
        }
        unsafe {
            let p = strerror(self.0);
            if p.is_null() {
                return f.write_str("Unknown error");
            }
            CStrDisplay(p as *const i8).fmt(f)
        }
    }
}

/// Safe replacement for C `memchr`. Searches `len` bytes starting at `ptr` for byte `c`.
/// Returns pointer to first occurrence, or null if not found.
#[inline]
pub unsafe fn byte_memchr(ptr: *const i8, c: u8, len: usize) -> *const i8 {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    match slice.iter().position(|&b| b == c) {
        Some(i) => unsafe { ptr.add(i) },
        None => core::ptr::null(),
    }
}

// ---------------------------------------------------------------------------
// darray helpers — replace libc realloc/free for the c2rust darray pattern
// ---------------------------------------------------------------------------

/// Compute next darray capacity. Replaces the duplicated `darray_next_alloc` in ~29 files.
/// Same growth strategy as the C original: start at 4, double until >= need.
#[inline]
pub fn darray_next_alloc(alloc: u32, need: u32) -> u32 {
    let mut a = if alloc == 0 { 4 } else { alloc };
    while a < need {
        a = a.wrapping_mul(2);
    }
    a
}

/// Grow a darray's backing allocation if `need > *alloc_ref`.
/// Uses `std::alloc` (which on Linux IS the system allocator, compatible with libc malloc/free).
///
/// # Safety
/// `*item_ptr` must be null (for first alloc) or a pointer previously allocated by
/// `std::alloc::alloc`/`std::alloc::realloc` or libc `malloc`/`realloc` (same allocator on Linux).
/// `T` must be a `Copy` type (all darray element types are `#[derive(Copy, Clone)]`).
#[inline]
pub unsafe fn darray_growalloc<T>(item_ptr: &mut *mut T, alloc_ref: &mut u32, need: u32) {
    if need <= *alloc_ref {
        return;
    }
    let new_alloc = darray_next_alloc(*alloc_ref, need);
    let new_size_bytes = (new_alloc as usize)
        .checked_mul(core::mem::size_of::<T>())
        .expect("darray_growalloc: overflow");
    let align = core::mem::align_of::<T>();
    let new_ptr = if (*item_ptr).is_null() {
        let layout = core::alloc::Layout::from_size_align_unchecked(new_size_bytes, align);
        std::alloc::alloc(layout) as *mut T
    } else {
        let old_size_bytes = (*alloc_ref as usize) * core::mem::size_of::<T>();
        let old_layout = core::alloc::Layout::from_size_align_unchecked(old_size_bytes, align);
        std::alloc::realloc(*item_ptr as *mut u8, old_layout, new_size_bytes) as *mut T
    };
    if new_ptr.is_null() {
        // Original C code didn't check either — abort like the C version would.
        std::process::abort();
    }
    *item_ptr = new_ptr;
    *alloc_ref = new_alloc;
}

/// Append one element to a darray. Replaces the ~8 line "Pattern A" expand.
///
/// # Safety
/// Same requirements as `darray_growalloc`. `val` is copied into the array.
#[inline]
pub unsafe fn darray_append<T: Copy>(
    item_ptr: &mut *mut T,
    size_ref: &mut u32,
    alloc_ref: &mut u32,
    val: T,
) {
    *size_ref = size_ref.wrapping_add(1);
    darray_growalloc(item_ptr, alloc_ref, *size_ref);
    *(*item_ptr).offset((*size_ref).wrapping_sub(1) as isize) = val;
}

/// Free a darray's items and reset size/alloc to 0.
/// Uses `std::alloc::dealloc` (same allocator as libc free on Linux).
///
/// # Safety
/// `*item_ptr` must be null or a pointer from `std::alloc`/libc `malloc`.
#[inline]
pub unsafe fn darray_free<T>(item_ptr: &mut *mut T, size_ref: &mut u32, alloc_ref: &mut u32) {
    if !(*item_ptr).is_null() && *alloc_ref > 0 {
        let size_bytes = (*alloc_ref as usize) * core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        let layout = core::alloc::Layout::from_size_align_unchecked(size_bytes, align);
        std::alloc::dealloc(*item_ptr as *mut u8, layout);
    }
    *item_ptr = core::ptr::null_mut();
    *size_ref = 0;
    *alloc_ref = 0;
}

/// Resize a darray to `new_size`, growing allocation if needed.
/// Does NOT zero-fill new elements (use `darray_resize_zero` for that).
///
/// # Safety
/// Same requirements as `darray_growalloc`.
#[inline]
pub unsafe fn darray_resize<T>(
    item_ptr: &mut *mut T,
    size_ref: &mut u32,
    alloc_ref: &mut u32,
    new_size: u32,
) {
    if new_size > *alloc_ref {
        darray_growalloc(item_ptr, alloc_ref, new_size);
    }
    *size_ref = new_size;
}

/// Resize a darray to `new_size`, zero-filling any new elements (Pattern C).
///
/// # Safety
/// Same requirements as `darray_growalloc`.
#[inline]
pub unsafe fn darray_resize_zero<T>(
    item_ptr: &mut *mut T,
    size_ref: &mut u32,
    alloc_ref: &mut u32,
    new_size: u32,
) {
    let old_size = *size_ref;
    darray_resize(item_ptr, size_ref, alloc_ref, new_size);
    if new_size > old_size {
        core::ptr::write_bytes(
            (*item_ptr).offset(old_size as isize) as *mut u8,
            0u8,
            (new_size - old_size) as usize * core::mem::size_of::<T>(),
        );
    }
}

/// Append `count` items from `src` to a darray. Grows allocation if needed.
/// After this call, `size` is increased by `count` and items are copied.
///
/// # Safety
/// - `src` must be valid for reading `count` items of type `T`.
/// - Same requirements as `darray_growalloc` for item_ptr/alloc_ref.
#[inline]
pub unsafe fn darray_appends<T: Copy>(
    item_ptr: &mut *mut T,
    size_ref: &mut u32,
    alloc_ref: &mut u32,
    src: *const T,
    count: u32,
) {
    let old_size = *size_ref;
    let new_size = old_size.wrapping_add(count);
    if new_size > *alloc_ref {
        darray_growalloc(item_ptr, alloc_ref, new_size);
    }
    *size_ref = new_size;
    core::ptr::copy_nonoverlapping(src, (*item_ptr).offset(old_size as isize), count as usize);
}

/// Append `count` items from `src` to a darray, plus a NUL terminator (zero byte).
/// Used for char/i8 darray patterns where the C code does `size += count + 1`,
/// copies `count` bytes, then writes `\0` at the end.
///
/// # Safety
/// - `src` must be valid for reading `count` bytes.
/// - Same requirements as `darray_growalloc`.
#[inline]
pub unsafe fn darray_appends_nul(
    item_ptr: &mut *mut i8,
    size_ref: &mut u32,
    alloc_ref: &mut u32,
    src: *const i8,
    count: u32,
) {
    let old_size = *size_ref;
    let new_size = old_size.wrapping_add(count).wrapping_add(1);
    if new_size > *alloc_ref {
        darray_growalloc(item_ptr, alloc_ref, new_size);
    }
    *size_ref = new_size;
    core::ptr::copy_nonoverlapping(src, (*item_ptr).offset(old_size as isize), count as usize);
    *(*item_ptr).offset(new_size.wrapping_sub(1) as isize) = 0;
}

// ── utils_h functions (moved from duplicated pub mod utils_h blocks) ─

#[inline]
pub unsafe fn istreq(s1: *const i8, s2: *const i8) -> bool {
    istrcmp(s1, s2) == 0
}

#[inline]
pub unsafe fn istrneq(s1: *const i8, s2: *const i8, len: usize) -> bool {
    istrncmp(s1, s2, len) == 0
}

#[inline]
pub unsafe fn strdup_safe(s: *const i8) -> *mut i8 {
    cstr_dup(s)
}

#[inline]
pub unsafe fn isempty(s: *const i8) -> bool {
    s.is_null() || *s == 0
}

#[inline]
pub unsafe fn streq(s1: *const i8, s2: *const i8) -> bool {
    assert!(!s1.is_null() && !s2.is_null(), "s1 && s2");
    std::ffi::CStr::from_ptr(s1) == std::ffi::CStr::from_ptr(s2)
}

#[inline]
pub unsafe fn streq_null(s1: *const i8, s2: *const i8) -> bool {
    if s1.is_null() || s2.is_null() {
        return s1 == s2;
    }
    streq(s1, s2)
}

#[inline]
pub unsafe fn streq_not_null(s1: *const i8, s2: *const i8) -> bool {
    if s1.is_null() || s2.is_null() {
        return false;
    }
    streq(s1, s2)
}

#[inline]
pub unsafe fn strempty(s: *const i8) -> *const i8 {
    if !s.is_null() {
        s
    } else {
        b"\0".as_ptr() as *const i8
    }
}

#[inline]
pub fn is_space(ch: i8) -> bool {
    ch as i32 == ' ' as i32 || (ch as i32 >= '\t' as i32 && ch as i32 <= '\r' as i32)
}

#[inline]
pub fn is_ascii(ch: i8) -> bool {
    ch as i32 & !(0x7f) == 0
}

#[inline]
pub fn is_graph(ch: i8) -> bool {
    ch as i32 > ' ' as i32 && (ch as i32) < 0x7f
}

#[inline]
pub fn is_alpha(ch: i8) -> bool {
    (ch as i32 >= 'a' as i32 && ch as i32 <= 'z' as i32)
        || (ch as i32 >= 'A' as i32 && ch as i32 <= 'Z' as i32)
}

#[inline]
pub fn is_digit(ch: i8) -> bool {
    ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32
}

#[inline]
pub fn is_alnum(ch: i8) -> bool {
    is_alpha(ch) || is_digit(ch)
}

#[inline]
pub fn is_xdigit(ch: i8) -> bool {
    (ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32)
        || (ch as i32 >= 'a' as i32 && ch as i32 <= 'f' as i32)
        || (ch as i32 >= 'A' as i32 && ch as i32 <= 'F' as i32)
}

#[inline]
pub fn is_valid_char(cp: u32) -> bool {
    cp != 0
}

#[inline]
pub fn is_aligned(pointer: *const ::core::ffi::c_void, byte_count: usize) -> bool {
    (pointer as usize).wrapping_rem(byte_count) == 0
}

#[inline]
pub fn one_bit_set(x: u32) -> i32 {
    (x != 0 && x & x.wrapping_sub(1) == 0) as i32
}

#[inline]
pub unsafe fn memdup(
    mem: *const ::core::ffi::c_void,
    nmemb: usize,
    size: usize,
) -> *mut ::core::ffi::c_void {
    let p: *mut ::core::ffi::c_void = libc::calloc(nmemb, size);
    if !p.is_null() {
        std::ptr::copy_nonoverlapping(mem as *const u8, p as *mut u8, nmemb.wrapping_mul(size));
    }
    p
}

#[inline]
pub unsafe fn strcpy_safe(mut dest: *mut i8, size: usize, src: *const i8) -> *mut i8 {
    if dest.is_null() || size == 0 || src.is_null() {
        return std::ptr::null_mut();
    }
    let limit: *const i8 = dest.offset(size as isize).offset(-1);
    let mut s = src;
    while dest < limit as *mut i8 && *s != 0 {
        let c = *s;
        s = s.offset(1);
        *dest = c;
        dest = dest.offset(1);
    }
    *dest = 0;
    if *s != 0 {
        std::ptr::null_mut()
    } else {
        dest
    }
}

#[inline]
pub unsafe fn check_eaccess(path: *const i8, mode: i32) -> bool {
    extern "C" {
        fn eaccess(__name: *const i8, __type: i32) -> i32;
    }
    unsafe { eaccess(path, mode) == 0 }
}

pub unsafe fn xkb_check_versioned_struct_size_(
    mut v1_size: usize,
    mut min_size: usize,
    mut lib_size: usize,
    mut caller_size: usize,
    mut caller_data: *const ::core::ffi::c_void,
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
            (caller_data as *const ::core::ffi::c_uchar).offset(lib_size as isize);
        let mut end: *const ::core::ffi::c_uchar =
            (caller_data as *const ::core::ffi::c_uchar).offset(caller_size as isize);
        while p < end {
            let c2rust_fresh1 = p;
            p = p.offset(1);
            if *c2rust_fresh1 != 0 {
                return XKB_ERROR_ABI_FORWARD_COMPAT;
            }
        }
        return XKB_SUCCESS;
    }
}

// === Number parsing utilities (consolidated from utils_numbers_h) ===

#[inline]
pub unsafe fn popcount32(mut x: u32) -> u32 {
    (x as u64).count_ones() as i32 as u32
}

#[inline]
pub unsafe fn next_pow2(mut x: u32) -> u32 {
    if x <= 1 as u32 {
        return 1 as u32;
    }
    (1 as u32)
        << (::core::mem::size_of::<u32>() as usize)
            .wrapping_mul(8 as usize)
            .wrapping_sub(x.wrapping_sub(1 as u32).leading_zeros() as i32 as usize)
}

#[inline]
pub unsafe fn parse_dec_to_uint32_t(mut s: *const i8, mut len: usize, mut out: *mut u32) -> i32 {
    unsafe {
        let mut result: u32 = 0 as u32;
        let mut i: usize = 0;
        i = 0 as usize;
        while i < len
            && ((*s.offset(i as isize) as i32 - '0' as i32) as ::core::ffi::c_uchar as u32)
                < 10 as u32
            && result <= (4294967295 as u32).wrapping_div(10 as u32)
            && result.wrapping_mul(10 as u32)
                <= (4294967295 as u32).wrapping_sub(
                    (*s.offset(i as isize) as i32 - '0' as i32) as ::core::ffi::c_uchar as u32,
                )
        {
            result = result
                .wrapping_mul(10 as u32)
                .wrapping_add((*s.offset(i as isize) as i32 - '0' as i32) as u32);
            i = i.wrapping_add(1);
        }
        *out = result as u32;
        return if i >= len
            || (*s.offset(i as isize) as i32 - '0' as i32) as ::core::ffi::c_uchar as u32
                >= 10 as u32
        {
            i as i32
        } else {
            -1 as i32
        };
    }
}

#[inline]
pub unsafe fn parse_dec_to_uint64_t(
    mut s: *const i8,
    mut len: usize,
    mut out: *mut u64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut result: u64 = 0 as u64;
        let mut i: usize = 0;
        i = 0 as usize;
        while i < len
            && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar
                as u32)
                < 10 as u32
            && result <= (18446744073709551615 as u64).wrapping_div(10 as u64)
            && result.wrapping_mul(10 as u64)
                <= (18446744073709551615 as u64).wrapping_sub(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                        as ::core::ffi::c_uchar as u64,
                )
        {
            result = result
                .wrapping_mul(10 as u64)
                .wrapping_add((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u64);
            i = i.wrapping_add(1);
        }
        *out = result as u64;
        return if i >= len
            || (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar
                as u32
                >= 10 as u32
        {
            i as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
}

pub static mut digits__: [::core::ffi::c_uchar; 256] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 10, 11, 12, 13, 14, 15,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 10, 11, 12, 13, 14, 15, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

#[inline]
pub unsafe fn parse_hex_to_uint32_t(
    mut s: *const i8,
    mut len: usize,
    mut out: *mut u32,
) -> ::core::ffi::c_int {
    unsafe {
        let mut result: u32 = 0 as u32;
        let mut i: usize = 0 as usize;
        while i < len
            && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32) < 16 as u32
            && result <= 4294967295 as u32 >> 4 as ::core::ffi::c_int
        {
            result = result.wrapping_mul(16 as u32).wrapping_add(
                digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32,
            );
            i = i.wrapping_add(1);
        }
        *out = result as u32;
        return if i >= len || !is_xdigit(*s.offset(i as isize)) {
            i as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
}

#[inline]
pub unsafe fn parse_hex_to_uint64_t(
    mut s: *const i8,
    mut len: usize,
    mut out: *mut u64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut result: u64 = 0 as u64;
        let mut i: usize = 0 as usize;
        while i < len
            && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32) < 16 as u32
            && result <= 18446744073709551615 as u64 >> 4 as ::core::ffi::c_int
        {
            result = result.wrapping_mul(16 as u64).wrapping_add(
                digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u64,
            );
            i = i.wrapping_add(1);
        }
        *out = result as u64;
        return if i >= len || !is_xdigit(*s.offset(i as isize)) {
            i as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
}

// ── Consolidated extern "C" declarations ──────────────────────────────
// (previously scattered across per-file pub mod xxx_h blocks)

use crate::xkb::shared_types::{dirent, stat, O_RDONLY};

// stat_h
extern "C" {
    pub fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    pub fn mkdir(__path: *const i8, __mode: u32) -> i32;
}
extern "C" {
    #[link_name = "stat"]
    pub fn xkb_stat(__file: *const i8, __buf: *mut stat) -> i32;
}

// include_locale_h
extern "C" {
    pub fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}

// unistd_h
extern "C" {
    pub fn eaccess(__name: *const i8, __type: i32) -> i32;
    pub fn close(__fd: i32) -> i32;
    pub fn dup(__fd: i32) -> i32;
    pub fn dup2(__fd: i32, __fd2: i32) -> i32;
}

// fcntl_h (variadic — must stay extern "C")
extern "C" {
    pub fn open(__file: *const i8, __oflag: i32, ...) -> i32;
}

// include_dirent_h
extern "C" {
    pub type __dirstream;
}
pub type DIR = __dirstream;
extern "C" {
    pub fn closedir(__dirp: *mut DIR) -> i32;
    pub fn opendir(__name: *const i8) -> *mut DIR;
    pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
}

// getopt_core_h
extern "C" {
    pub static mut optarg: *mut i8;
    pub static mut optind: i32;
}

// getopt_ext_h
use crate::xkb::shared_types::option;
extern "C" {
    pub fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
}
