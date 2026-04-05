pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn is_surrogate(mut cp: uint32_t) -> bool {
        unsafe {
            return cp >= 0xd800 as uint32_t && cp <= 0xdfff as uint32_t;
        }
    }
    use super::stdint_uintn_h::uint32_t;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
pub use self::types_h::{__uint32_t, __uint8_t};
pub use self::utils_h::is_surrogate;
#[no_mangle]
pub unsafe extern "C" fn utf32_to_utf8(
    mut unichar: uint32_t,
    mut buffer: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut count: ::core::ffi::c_int = 0;
        let mut shift: ::core::ffi::c_int = 0;
        let mut length: ::core::ffi::c_int = 0;
        let mut head: uint8_t = 0;
        if unichar <= 0x7f as uint32_t {
            *buffer.offset(0 as ::core::ffi::c_int as isize) = unichar as ::core::ffi::c_char;
            *buffer.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
            return 2 as ::core::ffi::c_int;
        } else {
            if unichar <= 0x7ff as uint32_t {
                length = 2 as ::core::ffi::c_int;
                head = 0xc0 as uint8_t;
            } else {
                if is_surrogate(unichar) {
                    c2rust_current_block = 5841877160196952624;
                } else if unichar <= 0xffff as uint32_t {
                    length = 3 as ::core::ffi::c_int;
                    head = 0xe0 as uint8_t;
                    c2rust_current_block = 17860125682698302841;
                } else if unichar <= 0x10ffff as uint32_t {
                    length = 4 as ::core::ffi::c_int;
                    head = 0xf0 as uint8_t;
                    c2rust_current_block = 17860125682698302841;
                } else {
                    c2rust_current_block = 5841877160196952624;
                }
                match c2rust_current_block {
                    17860125682698302841 => {}
                    _ => {
                        *buffer.offset(0 as ::core::ffi::c_int as isize) =
                            '\0' as i32 as ::core::ffi::c_char;
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            count = length - 1 as ::core::ffi::c_int;
            shift = 0 as ::core::ffi::c_int;
            while count > 0 as ::core::ffi::c_int {
                *buffer.offset(count as isize) =
                    (0x80 as uint32_t | unichar >> shift & 0x3f as uint32_t) as ::core::ffi::c_char;
                count -= 1;
                shift += 6 as ::core::ffi::c_int;
            }
            *buffer.offset(0 as ::core::ffi::c_int as isize) =
                (head as uint32_t | unichar >> shift & 0x3f as uint32_t) as ::core::ffi::c_char;
            *buffer.offset(length as isize) = '\0' as i32 as ::core::ffi::c_char;
            return length + 1 as ::core::ffi::c_int;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_utf8(
    mut ss: *const ::core::ffi::c_char,
    mut len: size_t,
) -> bool {
    unsafe {
        let mut i: size_t = 0 as size_t;
        let mut tail_bytes: size_t = 0 as size_t;
        let mut s: *const uint8_t = ss as *const uint8_t;
        while i < len {
            if *s.offset(i as isize) as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int {
                tail_bytes = 0 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int >= 0xc2 as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 0xdf as ::core::ffi::c_int
            {
                tail_bytes = 1 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int {
                i = i.wrapping_add(1);
                if i >= len
                    || !(*s.offset(i as isize) as ::core::ffi::c_int >= 0xa0 as ::core::ffi::c_int
                        && *s.offset(i as isize) as ::core::ffi::c_int
                            <= 0xbf as ::core::ffi::c_int)
                {
                    return false_0 != 0;
                }
                tail_bytes = 1 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int >= 0xe1 as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 0xec as ::core::ffi::c_int
            {
                tail_bytes = 2 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int == 0xed as ::core::ffi::c_int {
                i = i.wrapping_add(1);
                if i >= len
                    || !(*s.offset(i as isize) as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                        && *s.offset(i as isize) as ::core::ffi::c_int
                            <= 0x9f as ::core::ffi::c_int)
                {
                    return false_0 != 0;
                }
                tail_bytes = 1 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int >= 0xee as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 0xef as ::core::ffi::c_int
            {
                tail_bytes = 2 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int {
                i = i.wrapping_add(1);
                if i >= len
                    || !(*s.offset(i as isize) as ::core::ffi::c_int >= 0x90 as ::core::ffi::c_int
                        && *s.offset(i as isize) as ::core::ffi::c_int
                            <= 0xbf as ::core::ffi::c_int)
                {
                    return false_0 != 0;
                }
                tail_bytes = 2 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int >= 0xf1 as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 0xf3 as ::core::ffi::c_int
            {
                tail_bytes = 3 as size_t;
            } else if *s.offset(i as isize) as ::core::ffi::c_int == 0xf4 as ::core::ffi::c_int {
                i = i.wrapping_add(1);
                if i >= len
                    || !(*s.offset(i as isize) as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                        && *s.offset(i as isize) as ::core::ffi::c_int
                            <= 0x8f as ::core::ffi::c_int)
                {
                    return false_0 != 0;
                }
                tail_bytes = 2 as size_t;
            } else {
                return false_0 != 0;
            }
            i = i.wrapping_add(1);
            while i < len
                && tail_bytes > 0 as size_t
                && *s.offset(i as isize) as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 0xbf as ::core::ffi::c_int
            {
                i = i.wrapping_add(1);
                tail_bytes = tail_bytes.wrapping_sub(1);
            }
            if tail_bytes != 0 as size_t {
                return false_0 != 0;
            }
        }
        return true_0 != 0;
    }
}
