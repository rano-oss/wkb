pub mod types_h {
    pub type __uint64_t = u64;
    pub type __dev_t = u64;
    pub type __uid_t = u32;
    pub type __gid_t = u32;
    pub type __ino_t = u64;
    pub type __mode_t = u32;
    pub type __nlink_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
    pub type __time_t = i64;
    pub type __blksize_t = i64;
    pub type __blkcnt_t = i64;
    pub type __syscall_slong_t = i64;
}
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: i32,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}

pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: i32,
        pub _IO_read_ptr: *mut i8,
        pub _IO_read_end: *mut i8,
        pub _IO_read_base: *mut i8,
        pub _IO_write_base: *mut i8,
        pub _IO_write_ptr: *mut i8,
        pub _IO_write_end: *mut i8,
        pub _IO_buf_base: *mut i8,
        pub _IO_buf_end: *mut i8,
        pub _IO_save_base: *mut i8,
        pub _IO_backup_base: *mut i8,
        pub _IO_save_end: *mut i8,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: i32,
        #[bitfield(name = "_flags2", ty = "i32", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [i8; 1],
        pub _old_offset: __off_t,
        pub _cur_column: u16,
        pub _vtable_offset: i8,
        pub _shortbuf: [i8; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: i32,
        pub _unused3: i32,
        pub _total_written: __uint64_t,
        pub _unused2: [i8; 8],
    }
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        pub fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
        pub fn fileno(__stream: *mut FILE) -> i32;
    }
}
pub mod mman_h {
    pub const MAP_FAILED: *mut ::core::ffi::c_void = -1 as i32 as *mut ::core::ffi::c_void;

    use super::types_h::__off64_t;
    extern "C" {
        pub fn mmap(
            __addr: *mut ::core::ffi::c_void,
            __len: usize,
            __prot: i32,
            __flags: i32,
            __fd: i32,
            __offset: __off64_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn munmap(__addr: *mut ::core::ffi::c_void, __len: usize) -> i32;
    }
}
pub mod bits_stat_h {
    pub const __S_IFMT: i32 = 0o170000 as i32;
}
pub mod fcntl_linux_h {
    pub const O_RDONLY: i32 = 0 as i32;
}
pub mod fcntl_h {
    extern "C" {
        pub fn open(__file: *const i8, __oflag: i32, ...) -> i32;
    }
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod unistd_h {
    extern "C" {
        pub fn close(__fd: i32) -> i32;
    }
}
pub mod mman_linux_h {
    pub const PROT_READ: i32 = 0x1 as i32;
    pub const MAP_SHARED: i32 = 0x1 as i32;
}
pub use self::__stddef_null_h::NULL;

pub use self::bits_stat_h::__S_IFMT;
use self::fcntl_h::open;
pub use self::fcntl_linux_h::O_RDONLY;
pub use self::mman_h::{mmap, munmap, MAP_FAILED};
pub use self::mman_linux_h::{MAP_SHARED, PROT_READ};
use self::stat_h::fstat;
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::fdopen;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, __uint64_t,
};
use self::unistd_h::close;
pub use self::FILE_h::FILE;
pub unsafe fn open_file(mut path: *const i8) -> *mut FILE {
    unsafe {
        if path.is_null() {
            return ::core::ptr::null_mut::<FILE>();
        }
        let mut fd: i32 = open(path, O_RDONLY);
        if fd < 0 as i32 {
            return ::core::ptr::null_mut::<FILE>();
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
        if err != 0 as i32 || !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
            close(fd);
            return ::core::ptr::null_mut::<FILE>();
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
