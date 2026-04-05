pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_char {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut ::core::ffi::c_char,
    }
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: size_t,
    ) -> darray_size_t {
        unsafe {
            if (need as size_t)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as size_t)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }
    use super::__stddef_size_t_h::size_t;
    use super::assert_h::__assert_fail;
}
pub mod utils_text_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct text_line {
        pub start: *const ::core::ffi::c_char,
        pub length: size_t,
    }
    use super::__stddef_size_t_h::size_t;
}
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn rand() -> ::core::ffi::c_int;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::darray_h::{darray_char, darray_next_alloc, darray_size_t};
pub use self::stdbool_h::{false_0, true_0};
use self::stdlib_h::{rand, realloc};
use self::string_h::{memcpy, strchr, strlen, strstr};
pub use self::utils_text_h::text_line;
#[no_mangle]
pub unsafe extern "C" fn strip_lines(
    mut input: *const ::core::ffi::c_char,
    mut input_length: size_t,
    mut prefix: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut buf: darray_char = darray_char {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let prefix_len: size_t = strlen(prefix) as size_t;
        let mut start: *const ::core::ffi::c_char = input;
        let mut end: *const ::core::ffi::c_char = input.offset(input_length as isize);
        let mut next: *const ::core::ffi::c_char = strstr(start, prefix);
        let mut count: size_t = 0;
        while start < end && !next.is_null() {
            count = next.offset_from(start) as ::core::ffi::c_long as size_t;
            next = start.offset(count as isize).offset(prefix_len as isize);
            let mut i: size_t = 0;
            i = count;
            while i > 0 as size_t {
                if *start.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                    != ' ' as i32
                    && *start.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                        != '\t' as i32
                {
                    break;
                }
                i = i.wrapping_sub(1);
            }
            let mut dropped: bool = false_0 != 0;
            if i == 0 as size_t
                || *start.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                    == '\n' as i32
            {
                count = i;
                dropped = true_0 != 0;
            }
            let mut __count: darray_size_t = count as darray_size_t;
            let mut __oldSize: darray_size_t = buf.size;
            buf.size = __oldSize.wrapping_add(__count);
            let mut __need: darray_size_t = buf.size;
            if __need > buf.alloc {
                buf.alloc = darray_next_alloc(
                    buf.alloc,
                    __need,
                    ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                );
                buf.item = realloc(
                    buf.item as *mut ::core::ffi::c_void,
                    (buf.alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                ) as *mut ::core::ffi::c_char;
            }
            memcpy(
                buf.item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
                start as *const ::core::ffi::c_void,
                (__count as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            if next >= end {
                start = end;
                break;
            } else {
                start = strchr(next, 0xa as ::core::ffi::c_int);
                if start.is_null() {
                    start = end;
                    break;
                } else {
                    if dropped {
                        start = start.offset(1);
                    }
                    next = strstr(start, prefix);
                }
            }
        }
        if start < end {
            count = end.offset_from(start) as ::core::ffi::c_long as size_t;
            let mut __count_0: darray_size_t = count as darray_size_t;
            let mut __oldSize_0: darray_size_t = buf.size;
            buf.size = __oldSize_0.wrapping_add(__count_0);
            let mut __need_0: darray_size_t = buf.size;
            if __need_0 > buf.alloc {
                buf.alloc = darray_next_alloc(
                    buf.alloc,
                    __need_0,
                    ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                );
                buf.item = realloc(
                    buf.item as *mut ::core::ffi::c_void,
                    (buf.alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                ) as *mut ::core::ffi::c_char;
            }
            memcpy(
                buf.item.offset(__oldSize_0 as isize) as *mut ::core::ffi::c_void,
                start as *const ::core::ffi::c_void,
                (__count_0 as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
        }
        buf.size = buf.size.wrapping_add(1 as darray_size_t);
        let mut __need_1: darray_size_t = buf.size;
        if __need_1 > buf.alloc {
            buf.alloc = darray_next_alloc(
                buf.alloc,
                __need_1,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            );
            buf.item = realloc(
                buf.item as *mut ::core::ffi::c_void,
                (buf.alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
        }
        *buf.item
            .offset(buf.size.wrapping_sub(1 as darray_size_t) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
        return buf.item;
    }
}
#[no_mangle]
pub unsafe extern "C" fn uncomment(
    mut input: *const ::core::ffi::c_char,
    mut input_length: size_t,
    mut prefix: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut buf: darray_char = darray_char {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let prefix_len: size_t = strlen(prefix) as size_t;
        let mut start: *const ::core::ffi::c_char = input;
        let mut end: *const ::core::ffi::c_char = input.offset(input_length as isize);
        let mut next: *const ::core::ffi::c_char = strstr(start, prefix);
        let mut count: size_t = 0;
        while start < end && !next.is_null() {
            count = next.offset_from(start) as ::core::ffi::c_long as size_t;
            let mut __count: darray_size_t = count as darray_size_t;
            let mut __oldSize: darray_size_t = buf.size;
            buf.size = __oldSize.wrapping_add(__count);
            let mut __need: darray_size_t = buf.size;
            if __need > buf.alloc {
                buf.alloc = darray_next_alloc(
                    buf.alloc,
                    __need,
                    ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                );
                buf.item = realloc(
                    buf.item as *mut ::core::ffi::c_void,
                    (buf.alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                ) as *mut ::core::ffi::c_char;
            }
            memcpy(
                buf.item.offset(__oldSize as isize) as *mut ::core::ffi::c_void,
                start as *const ::core::ffi::c_void,
                (__count as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
            start = start.offset(count.wrapping_add(prefix_len) as isize);
            if start >= end {
                break;
            }
            next = strchr(start, 0xa as ::core::ffi::c_int);
            if next.is_null() {
                break;
            }
            next = strstr(next, prefix);
        }
        if start < end {
            count = end.offset_from(start) as ::core::ffi::c_long as size_t;
            let mut __count_0: darray_size_t = count as darray_size_t;
            let mut __oldSize_0: darray_size_t = buf.size;
            buf.size = __oldSize_0.wrapping_add(__count_0);
            let mut __need_0: darray_size_t = buf.size;
            if __need_0 > buf.alloc {
                buf.alloc = darray_next_alloc(
                    buf.alloc,
                    __need_0,
                    ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                );
                buf.item = realloc(
                    buf.item as *mut ::core::ffi::c_void,
                    (buf.alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
                ) as *mut ::core::ffi::c_char;
            }
            memcpy(
                buf.item.offset(__oldSize_0 as isize) as *mut ::core::ffi::c_void,
                start as *const ::core::ffi::c_void,
                (__count_0 as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
        }
        buf.size = buf.size.wrapping_add(1 as darray_size_t);
        let mut __need_1: darray_size_t = buf.size;
        if __need_1 > buf.alloc {
            buf.alloc = darray_next_alloc(
                buf.alloc,
                __need_1,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            );
            buf.item = realloc(
                buf.item as *mut ::core::ffi::c_void,
                (buf.alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
        }
        *buf.item
            .offset(buf.size.wrapping_sub(1 as darray_size_t) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
        return buf.item;
    }
}
#[no_mangle]
pub unsafe extern "C" fn split_lines(
    mut input: *const ::core::ffi::c_char,
    mut input_length: size_t,
    mut output: *mut text_line,
    mut output_length: size_t,
) -> size_t {
    unsafe {
        let mut start: *const ::core::ffi::c_char = input;
        let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut l: size_t = 0;
        let mut i: size_t = 0 as size_t;
        l = 0 as size_t;
        while i < input_length && l < output_length && *start as ::core::ffi::c_int != '\0' as i32 {
            next = strchr(start, 0xa as ::core::ffi::c_int);
            let ref mut c2rust_fresh0 = (*output.offset(l as isize)).start;
            *c2rust_fresh0 = start;
            if next.is_null() {
                let c2rust_fresh1 = l;
                l = l.wrapping_add(1);
                (*output.offset(c2rust_fresh1 as isize)).length = strlen(start);
                break;
            } else {
                (*output.offset(l as isize)).length =
                    (next.offset_from(start) as ::core::ffi::c_long as size_t)
                        .wrapping_add(1 as size_t);
                start = next.offset(1 as ::core::ffi::c_int as isize);
                i = i.wrapping_add((*output.offset(l as isize)).length);
                l = l.wrapping_add(1);
            }
        }
        return l;
    }
}
#[no_mangle]
pub unsafe extern "C" fn concat_lines(
    mut lines: *mut text_line,
    mut length: size_t,
    mut sep: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
) -> size_t {
    unsafe {
        let mut out: *mut ::core::ffi::c_char = output;
        let mut sep_len: size_t = strlen(sep);
        let mut i: size_t = 0 as size_t;
        while i < length {
            if i > 0 as size_t {
                memcpy(
                    out as *mut ::core::ffi::c_void,
                    sep as *const ::core::ffi::c_void,
                    sep_len,
                );
                out = out.offset(sep_len as isize);
            }
            memcpy(
                out as *mut ::core::ffi::c_void,
                (*lines.offset(i as isize)).start as *const ::core::ffi::c_void,
                (*lines.offset(i as isize)).length,
            );
            out = out.offset((*lines.offset(i as isize)).length as isize);
            i = i.wrapping_add(1);
        }
        *out = '\0' as i32 as ::core::ffi::c_char;
        return out.offset_from(output) as ::core::ffi::c_long as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn shuffle_lines(
    mut lines: *mut text_line,
    mut length: size_t,
    mut output: *mut ::core::ffi::c_char,
) -> size_t {
    unsafe {
        if length < 2147483647 as ::core::ffi::c_int as size_t {
        } else {
            __assert_fail(
                b"length < RAND_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/utils-text.c\0".as_ptr() as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_uint,
                b"size_t shuffle_lines(struct text_line *, size_t, char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut out: *mut ::core::ffi::c_char = output;
        if length > 1 as size_t {
            let mut i: size_t = length.wrapping_sub(1 as size_t);
            while i > 0 as size_t {
                let mut j: size_t = (rand() as size_t).wrapping_rem(i.wrapping_add(1 as size_t));
                let mut tmp: text_line = *lines.offset(j as isize);
                *lines.offset(j as isize) = *lines.offset(i as isize);
                *lines.offset(i as isize) = tmp;
                memcpy(
                    out as *mut ::core::ffi::c_void,
                    (*lines.offset(i as isize)).start as *const ::core::ffi::c_void,
                    (*lines.offset(i as isize)).length,
                );
                out = out.offset((*lines.offset(i as isize)).length as isize);
                if *out.offset(-1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\n' as i32
                {
                    *out.offset(0 as ::core::ffi::c_int as isize) =
                        '\n' as i32 as ::core::ffi::c_char;
                    out = out.offset(1);
                }
                i = i.wrapping_sub(1);
            }
        }
        return out.offset_from(output) as ::core::ffi::c_long as size_t;
    }
}
