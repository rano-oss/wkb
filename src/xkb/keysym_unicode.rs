pub mod types_h {
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
    pub type __pid_t = ::core::ffi::c_int;
}
pub mod stdint_uintn_h {
    pub type u32 = __uint32_t;
    use super::types_h::__uint32_t;
}

pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
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
pub mod xkbcommon_h {
    pub type xkb_keysym_t = u32;
    pub type xkb_keysym_flags = ::core::ffi::c_uint;
    pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;
    pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;

    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_keysym_get_name(
            keysym: xkb_keysym_t,
            buffer: *mut ::core::ffi::c_char,
            size: usize,
        ) -> ::core::ffi::c_int;
        pub fn xkb_keysym_from_name(
            name: *const ::core::ffi::c_char,
            flags: xkb_keysym_flags,
        ) -> xkb_keysym_t;
        pub fn xkb_keysym_to_utf8(
            keysym: xkb_keysym_t,
            buffer: *mut ::core::ffi::c_char,
            size: usize,
        ) -> ::core::ffi::c_int;
        pub fn xkb_keysym_to_utf32(keysym: xkb_keysym_t) -> u32;
        pub fn xkb_utf32_to_keysym(codepoint: u32) -> xkb_keysym_t;
    }
}
pub mod sys_types_h {
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}
pub mod stdio_h {

    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn perror(__s: *const ::core::ffi::c_char);
    }
}
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        pub fn fork() -> __pid_t;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn is_surrogate(mut cp: u32) -> bool {
        unsafe {
            return cp >= 0xd800 as u32 && cp <= 0xdfff as u32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn is_digit(mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= '0' as i32
                && ch as ::core::ffi::c_int <= '9' as i32;
        }
    }
    use super::stdint_uintn_h::u32;
}
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        pub fn waitpid(
            __pid: __pid_t,
            __stat_loc: *mut ::core::ffi::c_int,
            __options: ::core::ffi::c_int,
        ) -> __pid_t;
    }
}
pub mod assert_h {
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 48] = unsafe {
        ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
            *b"void test_unicode_keysyms_consistency(u32, u32)\0",
        )
    };
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod stdlib_h {
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn strtoul(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;
        pub fn strtoull(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulonglong;
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod string_h {
    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod test_h {
    pub const TEST_SETUP_FAILURE: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
    extern "C" {
        pub fn test_init();
    }
}
pub mod keysym_h {
    pub const XKB_KEYSYM_UNICODE_OFFSET: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    pub const XKB_KEYSYM_NAME_MAX_SIZE: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
    pub const XKB_KEYSYM_UTF8_MAX_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
pub use self::keysym_h::{
    XKB_KEYSYM_NAME_MAX_SIZE, XKB_KEYSYM_UNICODE_OFFSET, XKB_KEYSYM_UTF8_MAX_SIZE,
};
pub use self::stdint_uintn_h::u32;
use self::stdio_h::{fprintf, perror, snprintf, stderr};
pub use self::stdlib_h::{exit, strtoul, strtoull, EXIT_FAILURE, EXIT_SUCCESS};
use self::string_h::strcmp;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::sys_types_h::pid_t;
pub use self::test_h::{test_init, TEST_SETUP_FAILURE};
pub use self::types_h::{__off64_t, __off_t, __pid_t, __uint32_t, __uint64_t};
use self::unistd_h::fork;
pub use self::utils_h::{is_digit, is_surrogate};
use self::wait_h::waitpid;
pub use self::xkbcommon_h::{
    xkb_keysym_flags, xkb_keysym_from_name, xkb_keysym_get_name, xkb_keysym_t, xkb_keysym_to_utf32,
    xkb_keysym_to_utf8, xkb_utf32_to_keysym, XKB_KEYSYM_CASE_INSENSITIVE, XKB_KEYSYM_NO_FLAGS,
};
pub use self::FILE_h::FILE;
unsafe extern "C" fn test_unicode_keysyms_consistency(mut start: u32, mut end: u32) {
    unsafe {
        let mut buffer: [::core::ffi::c_char; 31] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut utf8: [::core::ffi::c_char; 5] =
            [0 as ::core::ffi::c_int as ::core::ffi::c_char, 0, 0, 0, 0];
        let mut cp: u32 = start;
        while cp <= end {
            let mut unicode: xkb_keysym_t =
                (XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t).wrapping_add(cp as xkb_keysym_t);
            let mut canonical: xkb_keysym_t = xkb_utf32_to_keysym(cp);
            let mut count: ::core::ffi::c_int = xkb_keysym_get_name(
                unicode,
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
            );
            if count > 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"count > 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                    45 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
            if cp == 0 as u32 || is_surrogate(cp) as ::core::ffi::c_int != 0 {
                if canonical == 0 as xkb_keysym_t {
                } else {
                    __assert_fail(
                        b"canonical == XKB_KEY_NoSymbol\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        52 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                if xkb_keysym_to_utf32(unicode) == 0 as u32 {
                } else {
                    __assert_fail(
                        b"xkb_keysym_to_utf32(unicode) == 0\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        54 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                let mut end_ptr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if cp == 0 as u32 {
                    if strtoull(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        &raw mut end_ptr,
                        16 as ::core::ffi::c_int,
                    ) == unicode as ::core::ffi::c_ulonglong
                    {
                    } else {
                        __assert_fail(
                            b"strtoull(buffer, &end_ptr, 16) == unicode\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            58 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                } else {
                    if count == 5 as ::core::ffi::c_int
                        && buffer[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 'U' as i32
                    {
                    } else {
                        __assert_fail(
                            b"count == 5 && buffer[0] == 'U'\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            61 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                    if strtoull(
                        (&raw mut buffer as *mut ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                        &raw mut end_ptr,
                        16 as ::core::ffi::c_int,
                    ) == cp as ::core::ffi::c_ulonglong
                    {
                    } else {
                        __assert_fail(
                            b"strtoull(buffer + 1, &end_ptr, 16) == cp\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            62 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                }
                if *end_ptr as ::core::ffi::c_int == '\0' as i32 {
                } else {
                    __assert_fail(
                        b"*end_ptr == '\\0'\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        64 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                let mut ks: xkb_keysym_t = xkb_keysym_from_name(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    XKB_KEYSYM_NO_FLAGS,
                );
                if ks == unicode {
                } else {
                    __assert_fail(
                        b"ks == unicode\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        67 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                snprintf(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                    b"U%X\0".as_ptr() as *const ::core::ffi::c_char,
                    cp,
                );
                ks = xkb_keysym_from_name(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    XKB_KEYSYM_NO_FLAGS,
                );
                if (cp == 0 as u32 && ks == 0 as xkb_keysym_t) as ::core::ffi::c_int
                    ^ (is_surrogate(cp) as ::core::ffi::c_int != 0 && ks == unicode)
                        as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"(cp == 0 && ks == XKB_KEY_NoSymbol) ^ (is_surrogate(cp) && ks == unicode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        72 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                count = xkb_keysym_to_utf8(
                    unicode,
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                );
                if count == 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"count == 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        75 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
            } else {
                let __cond: bool = (canonical == unicode) as ::core::ffi::c_int
                    ^ (0x20 as u32 <= cp
                        && cp <= 0x100 as u32
                        && cp != 0x7f as u32
                        && canonical == cp
                        || canonical != unicode
                            && canonical != 0 as xkb_keysym_t
                            && (canonical != cp || canonical == 0x20ac as xkb_keysym_t))
                        as ::core::ffi::c_int
                    != 0;
                if !__cond {
                    fprintf(
                        stderr,
                        b"Assertion failure: Code point: U+%04X, Unicode: %#x, canonical: %#x\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        cp,
                        unicode,
                        canonical,
                    );
                    if __cond as ::core::ffi::c_int != 0 {
                    } else {
                        __assert_fail(
                            b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            89 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                }
                if xkb_keysym_to_utf32(unicode) == cp {
                } else {
                    __assert_fail(
                        b"xkb_keysym_to_utf32(unicode) == cp\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        91 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                if xkb_keysym_to_utf32(canonical) == cp {
                } else {
                    __assert_fail(
                        b"xkb_keysym_to_utf32(canonical) == cp\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        92 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                let mut ks_0: xkb_keysym_t = xkb_keysym_from_name(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    XKB_KEYSYM_NO_FLAGS,
                );
                if (unicode != canonical && ks_0 == canonical) as ::core::ffi::c_int
                    ^ (ks_0 == unicode) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"(unicode != canonical && ks == canonical) ^ (ks == unicode)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        95 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                if !(buffer[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'U' as i32
                    && count > 4 as ::core::ffi::c_int
                    && is_digit(buffer[1 as ::core::ffi::c_int as usize]) as ::core::ffi::c_int
                        != 0)
                {
                    snprintf(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                        b"U%X\0".as_ptr() as *const ::core::ffi::c_char,
                        cp,
                    );
                    ks_0 = xkb_keysym_from_name(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        XKB_KEYSYM_NO_FLAGS,
                    );
                    if (unicode != canonical && ks_0 == canonical) as ::core::ffi::c_int
                        ^ (ks_0 == unicode) as ::core::ffi::c_int
                        != 0
                    {
                    } else {
                        __assert_fail(
                            b"(unicode != canonical && ks == canonical) ^ (ks == unicode)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            104 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                }
                if snprintf(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                    b"%#x\0".as_ptr() as *const ::core::ffi::c_char,
                    unicode,
                ) == 9 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"snprintf(buffer, sizeof(buffer), \"%#\"PRIx32, unicode) == 9\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        107 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                ks_0 = xkb_keysym_from_name(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    XKB_KEYSYM_NO_FLAGS,
                );
                if ks_0 == unicode {
                } else {
                    __assert_fail(
                        b"ks == unicode\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        109 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                count = xkb_keysym_to_utf8(
                    unicode,
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                );
                if count > 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"count > 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                        112 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                };
                if canonical != unicode {
                    let count2: ::core::ffi::c_int = xkb_keysym_to_utf8(
                        canonical,
                        &raw mut utf8 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize,
                    ) as ::core::ffi::c_int;
                    if count2 == count {
                    } else {
                        __assert_fail(
                            b"count2 == count\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                    if strcmp(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        &raw mut utf8 as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"strcmp(buffer, utf8) == 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            117 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                    count = xkb_keysym_get_name(
                        canonical,
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                    );
                    if count > 0 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"count > 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            120 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                    if xkb_keysym_from_name(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        XKB_KEYSYM_NO_FLAGS,
                    ) == canonical
                    {
                    } else {
                        __assert_fail(
                            b"xkb_keysym_from_name(buffer, XKB_KEYSYM_NO_FLAGS) == canonical\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            121 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                    if snprintf(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as usize,
                        b"%#x\0".as_ptr() as *const ::core::ffi::c_char,
                        canonical,
                    ) > 2 as ::core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"snprintf(buffer, sizeof(buffer), \"%#\"PRIx32, canonical) > 2\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            122 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                    if xkb_keysym_from_name(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        XKB_KEYSYM_NO_FLAGS,
                    ) == canonical
                    {
                    } else {
                        __assert_fail(
                            b"xkb_keysym_from_name(buffer, XKB_KEYSYM_NO_FLAGS) == canonical\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"../test/keysym-unicode.c\0".as_ptr() as *const ::core::ffi::c_char,
                            123 as ::core::ffi::c_uint,
                            __ASSERT_FUNCTION.as_ptr(),
                        );
                    };
                }
            }
            cp = cp.wrapping_add(1);
        }
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut NUM_PROCESSES: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
        if argc > 1 as ::core::ffi::c_int {
            NUM_PROCESSES = strtoul(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            );
        }
        if NUM_PROCESSES == 0 as ::core::ffi::c_ulong || NUM_PROCESSES > 32 as ::core::ffi::c_ulong
        {
            NUM_PROCESSES = 4 as ::core::ffi::c_ulong;
        }
        let max_codepoint: u32 = 0x10ffff as u32;
        let chunk: u32 = (max_codepoint as ::core::ffi::c_ulong).wrapping_div(NUM_PROCESSES) as u32;
        let vla = NUM_PROCESSES as usize;
        let mut pids: Vec<pid_t> = ::std::vec::from_elem(0, vla);
        let mut i: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
        while i < NUM_PROCESSES {
            let mut pid: pid_t = fork() as pid_t;
            if pid == 0 as ::core::ffi::c_int {
                let start: u32 = i.wrapping_mul(chunk as ::core::ffi::c_ulong) as u32;
                let end: u32 = if i == NUM_PROCESSES.wrapping_sub(1 as ::core::ffi::c_ulong) {
                    max_codepoint
                } else {
                    start.wrapping_add(chunk)
                };
                test_unicode_keysyms_consistency(start, end);
                exit(EXIT_SUCCESS);
            } else if pid > 0 as ::core::ffi::c_int {
                *pids.as_mut_ptr().offset(i as isize) = pid;
            } else {
                perror(b"fork\0".as_ptr() as *const ::core::ffi::c_char);
                return TEST_SETUP_FAILURE;
            }
            i = i.wrapping_add(1);
        }
        let mut exit_code: ::core::ffi::c_int = EXIT_SUCCESS;
        let mut i_0: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
        while i_0 < NUM_PROCESSES {
            let mut status: ::core::ffi::c_int = 0;
            if waitpid(
                *pids.as_mut_ptr().offset(i_0 as isize),
                &raw mut status,
                0 as ::core::ffi::c_int,
            ) == -1 as ::core::ffi::c_int
            {
                perror(b"waitpid\0".as_ptr() as *const ::core::ffi::c_char);
                exit_code = EXIT_FAILURE;
            } else if status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut code: ::core::ffi::c_int =
                    (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
                if code != EXIT_SUCCESS {
                    fprintf(
                        stderr,
                        b"Child PID %d exited with code %d\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *pids.as_mut_ptr().offset(i_0 as isize),
                        code,
                    );
                    exit_code = EXIT_FAILURE;
                }
            } else if ((status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_schar as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Child PID %d terminated by signal %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *pids.as_mut_ptr().offset(i_0 as isize),
                    status & 0x7f as ::core::ffi::c_int,
                );
                exit_code = EXIT_FAILURE;
            }
            i_0 = i_0.wrapping_add(1);
        }
        return exit_code;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
