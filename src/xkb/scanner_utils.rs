// Canonical scanner utility types and functions (consolidated from scanner_utils_h blocks)
use crate::xkb::shared_types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sval {
    pub len: usize,
    pub start: *const i8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct darray_sval {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut sval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct scanner_loc {
    pub line: usize,
    pub column: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct scanner {
    pub pos: usize,
    pub len: usize,
    pub s: *const i8,
    pub buf: [i8; 1024],
    pub buf_pos: usize,
    pub token_pos: usize,
    pub cached_pos: usize,
    pub cached_loc: scanner_loc,
    pub file_name: *const i8,
    pub ctx: *mut xkb_context,
    pub priv_0: *mut ::core::ffi::c_void,
}

pub unsafe fn scanner_token_location(mut s: *mut scanner) -> scanner_loc {
    unsafe {
        let mut line: usize = 0;
        let mut column: usize = 0;
        let mut line_pos: usize = 0 as usize;
        if (*s).cached_pos > (*s).token_pos {
            (*s).cached_pos = 0 as usize;
            (*s).cached_loc.column = 1 as usize;
            (*s).cached_loc.line = (*s).cached_loc.column;
        }
        line = (*s).cached_loc.line;
        let mut ptr: *const i8 = (*s).s.offset((*s).cached_pos as isize);
        let mut last: *const i8 = (*s).s.offset((*s).token_pos as isize);
        loop {
            ptr = crate::xkb::utils::byte_memchr(ptr, b'\n', last.offset_from(ptr) as i64 as usize);
            if ptr.is_null() {
                break;
            }
            line = line.wrapping_add(1);
            ptr = ptr.offset(1);
            line_pos = ptr.offset_from((*s).s) as i64 as usize;
        }
        if line == (*s).cached_loc.line {
            column = (*s)
                .cached_loc
                .column
                .wrapping_add((*s).token_pos.wrapping_sub((*s).cached_pos));
        } else {
            column = (*s)
                .token_pos
                .wrapping_sub(line_pos)
                .wrapping_add(1 as usize);
        }
        let mut loc: scanner_loc = scanner_loc {
            line: line,
            column: column,
        };
        (*s).cached_pos = (*s).token_pos;
        (*s).cached_loc = loc;
        return loc;
    }
}

#[inline]
pub unsafe fn scanner_init(
    mut s: *mut scanner,
    mut ctx: *mut xkb_context,
    mut string: *const i8,
    mut len: usize,
    mut file_name: *const i8,
    mut priv_0: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*s).s = string;
        (*s).len = len;
        (*s).pos = 0 as usize;
        (*s).token_pos = 0 as usize;
        (*s).cached_pos = 0 as usize;
        (*s).cached_loc.column = 1 as usize;
        (*s).cached_loc.line = (*s).cached_loc.column;
        (*s).file_name = file_name;
        (*s).ctx = ctx;
        (*s).priv_0 = priv_0;
    }
}

#[inline]
pub unsafe fn scanner_peek(mut s: *mut scanner) -> i8 {
    unsafe {
        if ((*s).pos >= (*s).len) as ::core::ffi::c_int as i64 != 0 {
            return '\0' as i32 as i8;
        }
        return *(*s).s.offset((*s).pos as isize);
    }
}

#[inline]
pub unsafe fn scanner_eof(mut s: *mut scanner) -> bool {
    unsafe {
        return (*s).pos >= (*s).len;
    }
}

#[inline]
pub unsafe fn scanner_eol(mut s: *mut scanner) -> bool {
    unsafe {
        return scanner_peek(s) as ::core::ffi::c_int == '\n' as i32;
    }
}

#[inline]
pub unsafe fn scanner_skip_to_eol(mut s: *mut scanner) {
    unsafe {
        let mut nl: *const i8 = crate::xkb::utils::byte_memchr(
            (*s).s.offset((*s).pos as isize),
            b'\n',
            (*s).len.wrapping_sub((*s).pos),
        );
        let new_pos: usize = if !nl.is_null() {
            nl.offset_from((*s).s) as i64 as usize
        } else {
            (*s).len
        };
        (*s).pos = new_pos;
    }
}

#[inline]
pub unsafe fn scanner_next(mut s: *mut scanner) -> i8 {
    unsafe {
        if scanner_eof(s) as ::core::ffi::c_int as i64 != 0 {
            return '\0' as i32 as i8;
        }
        let c2rust_fresh0 = (*s).pos;
        (*s).pos = (*s).pos.wrapping_add(1);
        return *(*s).s.offset(c2rust_fresh0 as isize);
    }
}

#[inline]
pub unsafe fn scanner_chr(mut s: *mut scanner, mut ch: i8) -> bool {
    unsafe {
        if (scanner_peek(s) as ::core::ffi::c_int != ch as ::core::ffi::c_int) as ::core::ffi::c_int
            as i64
            != 0
        {
            return false;
        }
        (*s).pos = (*s).pos.wrapping_add(1);
        return true;
    }
}

#[inline]
pub unsafe fn scanner_str(mut s: *mut scanner, mut string: *const i8, mut len: usize) -> bool {
    unsafe {
        if (*s).len.wrapping_sub((*s).pos) < len {
            return false;
        }
        if std::slice::from_raw_parts((*s).s.offset((*s).pos as isize) as *const u8, len)
            != std::slice::from_raw_parts(string as *const u8, len)
        {
            return false;
        }
        (*s).pos = (*s).pos.wrapping_add(len);
        return true;
    }
}

#[inline]
pub unsafe fn scanner_buf_append(mut s: *mut scanner, mut ch: i8) -> bool {
    unsafe {
        if (*s).buf_pos.wrapping_add(1 as usize) >= ::core::mem::size_of::<[i8; 1024]>() as usize {
            return false;
        }
        let c2rust_fresh1 = (*s).buf_pos;
        (*s).buf_pos = (*s).buf_pos.wrapping_add(1);
        (*s).buf[c2rust_fresh1 as usize] = ch;
        return true;
    }
}

#[inline]
pub unsafe fn scanner_buf_appends(mut s: *mut scanner, mut str: *const i8) -> bool {
    unsafe {
        let mut i: usize = 0 as usize;
        while *str.offset(i as isize) as i32 != 0 {
            if !scanner_buf_append(s, *str.offset(i as isize)) {
                return false;
            }
            i = i.wrapping_add(1);
        }
        return true;
    }
}

#[inline]
pub unsafe fn scanner_buf_appends_code_point(mut s: *mut scanner, mut c: u32) -> bool {
    unsafe {
        if (*s).buf_pos.wrapping_add(5 as usize) <= ::core::mem::size_of::<[i8; 1024]>() as usize {
            let mut count: ::core::ffi::c_int = crate::xkb::xkbcomp::scanner::utf8_h::utf32_to_utf8(
                c,
                (&raw mut (*s).buf as *mut i8).offset((*s).buf_pos as isize),
            );
            if count == 0 as ::core::ffi::c_int {
                count = crate::xkb::xkbcomp::scanner::utf8_h::utf32_to_utf8(
                    0xfffd as u32,
                    (&raw mut (*s).buf as *mut i8).offset((*s).buf_pos as isize),
                );
            }
            if count == 0 as ::core::ffi::c_int {
                return false;
            }
            (*s).buf_pos = (*s)
                .buf_pos
                .wrapping_add((count - 1 as ::core::ffi::c_int) as usize);
            return true;
        } else {
            return false;
        };
    }
}

#[inline]
pub unsafe fn scanner_oct(mut s: *mut scanner, mut out: *mut u8) -> bool {
    unsafe {
        let mut i: u8 = 0 as u8;
        let mut c: u8 = 0 as u8;
        while scanner_peek(s) as ::core::ffi::c_int >= '0' as i32
            && scanner_peek(s) as ::core::ffi::c_int <= '7' as i32
            && (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
        {
            if (c as ::core::ffi::c_int) < 0o40 as ::core::ffi::c_int {
                c = (c as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                    + scanner_next(s) as ::core::ffi::c_int
                    - '0' as i32) as u8;
            } else {
                scanner_next(s);
                *out = c;
                return false;
            }
            i = i.wrapping_add(1);
        }
        *out = c;
        return i as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
    }
}

#[inline]
pub unsafe fn scanner_dec_int64(mut s: *mut scanner, mut out: *mut i64) -> ::core::ffi::c_int {
    unsafe {
        let mut val: u64 = 0 as u64;
        let count: ::core::ffi::c_int = crate::xkb::utils::parse_dec_to_uint64_t(
            (*s).s.offset((*s).pos as isize),
            (*s).len.wrapping_sub((*s).pos),
            &raw mut val,
        ) as ::core::ffi::c_int;
        if count > 0 as ::core::ffi::c_int {
            if val > i64::MAX as u64 {
                return -1 as ::core::ffi::c_int;
            }
            (*s).pos = (*s).pos.wrapping_add(count as usize);
            *out = val as i64;
        }
        return count;
    }
}

#[inline]
pub unsafe fn scanner_hex_int64(mut s: *mut scanner, mut out: *mut i64) -> ::core::ffi::c_int {
    unsafe {
        let mut val: u64 = 0 as u64;
        let count: ::core::ffi::c_int = crate::xkb::utils::parse_hex_to_uint64_t(
            (*s).s.offset((*s).pos as isize),
            (*s).len.wrapping_sub((*s).pos),
            &raw mut val,
        ) as ::core::ffi::c_int;
        if count > 0 as ::core::ffi::c_int {
            if val > i64::MAX as u64 {
                return -1 as ::core::ffi::c_int;
            }
            (*s).pos = (*s).pos.wrapping_add(count as usize);
            *out = val as i64;
        }
        return count;
    }
}

#[inline]
pub unsafe fn scanner_unicode_code_point(mut s: *mut scanner, mut out: *mut u32) -> bool {
    unsafe {
        if !scanner_chr(s, '{' as i32 as i8) {
            return false;
        }
        let mut cp: u32 = 0 as u32;
        let count: ::core::ffi::c_int = crate::xkb::utils::parse_hex_to_uint32_t(
            (*s).s.offset((*s).pos as isize),
            (*s).len.wrapping_sub((*s).pos),
            &raw mut cp,
        ) as ::core::ffi::c_int;
        if count > 0 as ::core::ffi::c_int {
            (*s).pos = (*s).pos.wrapping_add(count as usize);
        }
        let last_valid: usize = (*s).pos;
        while !scanner_eof(s)
            && !scanner_eol(s)
            && scanner_peek(s) as ::core::ffi::c_int != '"' as i32
            && scanner_peek(s) as ::core::ffi::c_int != '}' as i32
        {
            scanner_next(s);
        }
        if scanner_chr(s, '}' as i32 as i8) {
            *out = cp;
            return count > 0 as ::core::ffi::c_int
                && (*s).pos == last_valid.wrapping_add(1 as usize)
                && cp <= 0x10ffff as u32;
        }
        (*s).pos = last_valid;
        return false;
    }
}

#[inline]
pub unsafe fn scanner_check_supported_char_encoding(mut s: *mut scanner) -> bool {
    use crate::xkb::messages::{XKB_ERROR_INVALID_FILE_ENCODING, XKB_LOG_VERBOSITY_MINIMAL};
    unsafe {
        if scanner_str(s, b"\xEF\xBB\xBF\0".as_ptr() as *const i8, 3 as usize) as ::core::ffi::c_int
            != 0
            || (*s).len < 2 as usize
        {
            return true;
        }
        if *(*s).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
            || *(*s).s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        {
            let mut loc: scanner_loc = scanner_token_location(s);
            crate::xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: unexpected NULL character.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            return false;
        }
        if !crate::xkb::utils::is_ascii(*(*s).s.offset(0 as ::core::ffi::c_int as isize)) {
            let mut loc_0: scanner_loc = scanner_token_location(s);
            crate::xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "[XKB-{:03}] {}:{}:{}: unexpected non-ASCII character.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as ::core::ffi::c_int,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
            );
            return false;
        }
        return true;
    }
}

#[inline]
pub unsafe fn svaleq(mut s1: sval, mut s2: sval) -> bool {
    unsafe {
        return s1.len == s2.len
            && std::slice::from_raw_parts(s1.start as *const u8, s1.len)
                == std::slice::from_raw_parts(s2.start as *const u8, s1.len);
    }
}

#[inline]
pub unsafe fn svaleq_prefix(mut s1: sval, mut s2: sval) -> bool {
    unsafe {
        return s1.len <= s2.len
            && std::slice::from_raw_parts(s1.start as *const u8, s1.len)
                == std::slice::from_raw_parts(s2.start as *const u8, s1.len);
    }
}

#[inline]
pub unsafe fn isvaleq(mut s1: sval, mut s2: sval) -> bool {
    unsafe {
        return s1.len == s2.len
            && crate::xkb::utils::istrncmp(s1.start, s2.start, s1.len) == 0 as ::core::ffi::c_int;
    }
}
