pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
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
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    pub const XKB_ATOM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::darray_h::darray_size_t;
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        pub fn strndup(
            __string: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
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
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atom_h::{xkb_atom_t, XKB_ATOM_NONE};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::stdlib_h::{calloc, free, realloc};
use self::string_h::{memset, strlen, strncmp, strndup};
pub use self::types_h::{__uint32_t, __uint8_t};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atom_table {
    pub index: *mut xkb_atom_t,
    pub index_size: size_t,
    pub strings: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *mut ::core::ffi::c_char,
}
#[inline]
unsafe extern "C" fn hash_buf(mut string: *const ::core::ffi::c_char, mut len: size_t) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = 2166136261 as uint32_t;
        let mut i: size_t = 0 as size_t;
        while i < len.wrapping_add(1 as size_t).wrapping_div(2 as size_t) {
            hash ^= *string.offset(i as isize) as uint8_t as uint32_t;
            hash = hash.wrapping_mul(0x1000193 as uint32_t);
            hash ^= *string.offset(len.wrapping_sub(1 as size_t).wrapping_sub(i) as isize)
                as uint8_t as uint32_t;
            hash = hash.wrapping_mul(0x1000193 as uint32_t);
            i = i.wrapping_add(1);
        }
        return hash;
    }
}
#[no_mangle]
pub unsafe extern "C" fn atom_table_new() -> *mut atom_table {
    unsafe {
        let mut table: *mut atom_table =
            calloc(1 as size_t, ::core::mem::size_of::<atom_table>() as size_t) as *mut atom_table;
        if table.is_null() {
            return ::core::ptr::null_mut::<atom_table>();
        }
        (*table).strings.item = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*table).strings.size = 0 as darray_size_t;
        (*table).strings.alloc = 0 as darray_size_t;
        (*table).strings.size = (*table).strings.size.wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = (*table).strings.size;
        if __need > (*table).strings.alloc {
            (*table).strings.alloc = darray_next_alloc(
                (*table).strings.alloc,
                __need,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
            );
            (*table).strings.item = realloc(
                (*table).strings.item as *mut ::core::ffi::c_void,
                ((*table).strings.alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
            ) as *mut *mut ::core::ffi::c_char;
        }
        let ref mut c2rust_fresh0 = *(*table)
            .strings
            .item
            .offset((*table).strings.size.wrapping_sub(1 as darray_size_t) as isize);
        *c2rust_fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*table).index_size = 4 as size_t;
        (*table).index = calloc(
            (*table).index_size,
            ::core::mem::size_of::<xkb_atom_t>() as size_t,
        ) as *mut xkb_atom_t;
        return table;
    }
}
#[no_mangle]
pub unsafe extern "C" fn atom_table_free(mut table: *mut atom_table) {
    unsafe {
        if table.is_null() {
            return;
        }
        let mut string: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        if !(*table).strings.item.is_null() {
            string = (*table)
                .strings
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ::core::ffi::c_char;
            while string
                < (*table).strings.item.offset((*table).strings.size as isize)
                    as *mut *mut ::core::ffi::c_char
            {
                free(*string as *mut ::core::ffi::c_void);
                string = string.offset(1);
            }
        }
        free((*table).strings.item as *mut ::core::ffi::c_void);
        (*table).strings.item = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*table).strings.size = 0 as darray_size_t;
        (*table).strings.alloc = 0 as darray_size_t;
        free((*table).index as *mut ::core::ffi::c_void);
        free(table as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn atom_table_size(mut table: *mut atom_table) -> darray_size_t {
    unsafe {
        return (*table).strings.size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn atom_text(
    mut table: *mut atom_table,
    mut atom: xkb_atom_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        if atom < (*table).strings.size {
        } else {
            __assert_fail(
                b"atom < darray_size(table->strings)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/atom.c\0".as_ptr() as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                b"const char *atom_text(struct atom_table *, xkb_atom_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        return *(*table).strings.item.offset(atom as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn atom_intern(
    mut table: *mut atom_table,
    mut string: *const ::core::ffi::c_char,
    mut len: size_t,
    mut add: bool,
) -> xkb_atom_t {
    unsafe {
        if (*table).strings.size as size_t
            > (*table)
                .index_size
                .wrapping_div(5 as size_t)
                .wrapping_mul(4 as size_t)
        {
            (*table).index_size = (*table).index_size.wrapping_mul(2 as size_t);
            (*table).index = realloc(
                (*table).index as *mut ::core::ffi::c_void,
                (*table)
                    .index_size
                    .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as size_t),
            ) as *mut xkb_atom_t;
            memset(
                (*table).index as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*table)
                    .index_size
                    .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as size_t),
            );
            let mut j: darray_size_t = 1 as darray_size_t;
            while j < (*table).strings.size {
                let mut s: *const ::core::ffi::c_char = *(*table).strings.item.offset(j as isize);
                let mut hash: uint32_t = hash_buf(s, strlen(s));
                let mut i: size_t = 0 as size_t;
                while i < (*table).index_size {
                    let mut index_pos: size_t = (hash as size_t).wrapping_add(i)
                        & (*table).index_size.wrapping_sub(1 as size_t);
                    if !(index_pos == 0 as size_t) {
                        let mut atom: xkb_atom_t = *(*table).index.offset(index_pos as isize);
                        if atom == XKB_ATOM_NONE as xkb_atom_t {
                            *(*table).index.offset(index_pos as isize) = j as xkb_atom_t;
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
        }
        let mut hash_0: uint32_t = hash_buf(string, len);
        let mut i_0: size_t = 0 as size_t;
        while i_0 < (*table).index_size {
            let mut index_pos_0: size_t = (hash_0 as size_t).wrapping_add(i_0)
                & (*table).index_size.wrapping_sub(1 as size_t);
            if !(index_pos_0 == 0 as size_t) {
                let mut existing_atom: xkb_atom_t = *(*table).index.offset(index_pos_0 as isize);
                if existing_atom == XKB_ATOM_NONE as xkb_atom_t {
                    if add {
                        let mut new_atom: xkb_atom_t = (*table).strings.size as xkb_atom_t;
                        (*table).strings.size =
                            (*table).strings.size.wrapping_add(1 as darray_size_t);
                        let mut __need: darray_size_t = (*table).strings.size;
                        if __need > (*table).strings.alloc {
                            (*table).strings.alloc = darray_next_alloc(
                                (*table).strings.alloc,
                                __need,
                                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
                            );
                            (*table).strings.item = realloc(
                                (*table).strings.item as *mut ::core::ffi::c_void,
                                ((*table).strings.alloc as size_t).wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
                                ),
                            )
                                as *mut *mut ::core::ffi::c_char;
                        }
                        let ref mut c2rust_fresh1 =
                            *(*table).strings.item.offset(
                                (*table).strings.size.wrapping_sub(1 as darray_size_t) as isize,
                            );
                        *c2rust_fresh1 = strndup(string, len);
                        *(*table).index.offset(index_pos_0 as isize) = new_atom;
                        return new_atom;
                    } else {
                        return XKB_ATOM_NONE as xkb_atom_t;
                    }
                }
                let mut existing_value: *const ::core::ffi::c_char =
                    *(*table).strings.item.offset(existing_atom as isize);
                if strncmp(existing_value, string, len) == 0 as ::core::ffi::c_int
                    && *existing_value.offset(len as isize) as ::core::ffi::c_int == '\0' as i32
                {
                    return existing_atom;
                }
            }
            i_0 = i_0.wrapping_add(1);
        }
        if (b"couldn't find an empty slot during probing\0".as_ptr() as *const ::core::ffi::c_char)
            .is_null()
        {
        } else {
            __assert_fail(
                b"!\"couldn't find an empty slot during probing\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../src/atom.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_uint,
                b"xkb_atom_t atom_intern(struct atom_table *, const char *, size_t, _Bool)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        return XKB_ATOM_NONE as xkb_atom_t;
    }
}
