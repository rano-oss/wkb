pub mod util_list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct list {
        pub prev: *mut list,
        pub next: *mut list,
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
use self::assert_h::__assert_fail;
pub use self::util_list_h::list;
#[no_mangle]
pub unsafe extern "C" fn list_init(mut list: *mut list) {
    unsafe {
        (*list).prev = list;
        (*list).next = list;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_insert(mut list: *mut list, mut elm: *mut list) {
    unsafe {
        if !(*list).next.is_null() && !(*list).prev.is_null()
            || (b"list->next|prev is NULL, possibly missing list_init()\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(list->next != NULL && list->prev != NULL) || !\"list->next|prev is NULL, possibly missing list_init()\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../src/util-list.c\0".as_ptr() as *const ::core::ffi::c_char,
                27 as ::core::ffi::c_uint,
                b"void list_insert(struct list *, struct list *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if (*elm).next.is_null() && (*elm).prev.is_null()
            || list_empty(elm) as ::core::ffi::c_int != 0
            || (b"elm->next|prev is not NULL, list node used twice?\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((elm->next == NULL && elm->prev == NULL) || list_empty(elm)) || !\"elm->next|prev is not NULL, list node used twice?\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../src/util-list.c\0".as_ptr() as *const ::core::ffi::c_char,
                29 as ::core::ffi::c_uint,
                b"void list_insert(struct list *, struct list *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*elm).prev = list;
        (*elm).next = (*list).next;
        (*list).next = elm;
        (*(*elm).next).prev = elm;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_append(mut list: *mut list, mut elm: *mut list) {
    unsafe {
        if !(*list).next.is_null() && !(*list).prev.is_null()
            || (b"list->next|prev is NULL, possibly missing list_init()\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(list->next != NULL && list->prev != NULL) || !\"list->next|prev is NULL, possibly missing list_init()\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../src/util-list.c\0".as_ptr() as *const ::core::ffi::c_char,
                41 as ::core::ffi::c_uint,
                b"void list_append(struct list *, struct list *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if (*elm).next.is_null() && (*elm).prev.is_null()
            || list_empty(elm) as ::core::ffi::c_int != 0
            || (b"elm->next|prev is not NULL, list node used twice?\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((elm->next == NULL && elm->prev == NULL) || list_empty(elm)) || !\"elm->next|prev is not NULL, list node used twice?\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../src/util-list.c\0".as_ptr() as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_uint,
                b"void list_append(struct list *, struct list *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*elm).next = list;
        (*elm).prev = (*list).prev;
        (*list).prev = elm;
        (*(*elm).prev).next = elm;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_remove(mut elm: *mut list) {
    unsafe {
        if !(*elm).next.is_null() && !(*elm).prev.is_null()
            || (b"list->next|prev is NULL, possibly missing list_init()\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(elm->next != NULL && elm->prev != NULL) || !\"list->next|prev is NULL, possibly missing list_init()\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../src/util-list.c\0".as_ptr() as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_uint,
                b"void list_remove(struct list *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*(*elm).prev).next = (*elm).next;
        (*(*elm).next).prev = (*elm).prev;
        (*elm).next = ::core::ptr::null_mut::<list>();
        (*elm).prev = ::core::ptr::null_mut::<list>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_empty(mut list: *const list) -> bool {
    unsafe {
        if !(*list).next.is_null() && !(*list).prev.is_null()
            || (b"list->next|prev is NULL, possibly missing list_init()\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"(list->next != NULL && list->prev != NULL) || !\"list->next|prev is NULL, possibly missing list_init()\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../src/util-list.c\0".as_ptr() as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"_Bool list_empty(const struct list *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        return (*list).next == list as *mut list;
    }
}
#[no_mangle]
pub unsafe extern "C" fn list_is_last(mut list: *const list, mut elm: *const list) -> bool {
    unsafe {
        return (*elm).next == list as *mut list;
    }
}
