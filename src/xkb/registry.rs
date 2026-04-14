use crate::xkb::messages::{XKB_ERROR_INVALID_PATH, XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH};
use crate::xkb::rmlvo::strdup_safe;
use crate::xkb::util_list::{list, list_append, list_empty, list_init, list_is_last, list_remove};
use crate::xkb::utils::{
    __errno_location, _steal, check_eaccess, closedir, cstr_cmp, cstr_dup, cstr_free, cstr_len,
    is_space, istrneq, opendir, readdir, streq, streq_null, xkb_stat, DIR,
};
use libc::{free, getenv, qsort, strtol};

pub type rxkb_log_level = u32;
pub const RXKB_LOG_LEVEL_DEBUG: rxkb_log_level = 50;
pub const RXKB_LOG_LEVEL_INFO: rxkb_log_level = 40;
pub const RXKB_LOG_LEVEL_WARNING: rxkb_log_level = 30;
pub const RXKB_LOG_LEVEL_ERROR: rxkb_log_level = 20;
pub const RXKB_LOG_LEVEL_CRITICAL: rxkb_log_level = 10;
pub type rxkb_popularity = u32;
pub const RXKB_POPULARITY_EXOTIC: rxkb_popularity = 2;
pub const RXKB_POPULARITY_STANDARD: rxkb_popularity = 1;
pub type rxkb_context_flags = u32;
pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;
extern "C" {
    pub fn secure_getenv(name: *const i8) -> *mut i8;
}
#[repr(C)]
pub struct rxkb_context {
    pub base: rxkb_object,
    pub context_state: context_state,
    pub load_extra_rules_files: bool,
    pub use_secure_getenv: bool,
    pub models: list,
    pub layouts: list,
    pub option_groups: list,
    pub includes: Vec<*mut i8>,
    pub log_fn: Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>,
    pub log_level: rxkb_log_level,
    pub userdata: *mut ::core::ffi::c_void,
}
pub type context_state = u32;
pub const CONTEXT_FAILED: context_state = 2;
pub const CONTEXT_PARSED: context_state = 1;
pub const CONTEXT_NEW: context_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_object {
    pub parent: *mut rxkb_object,
    pub refcount: u32,
    pub link: list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_model {
    pub base: rxkb_object,
    pub name: *mut i8,
    pub vendor: *mut i8,
    pub description: *mut i8,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_layout {
    pub base: rxkb_object,
    pub name: *mut i8,
    pub brief: *mut i8,
    pub description: *mut i8,
    pub variant: *mut i8,
    pub popularity: rxkb_popularity,
    pub iso639s: list,
    pub iso3166s: list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_option_group {
    pub base: rxkb_object,
    pub allow_multiple: bool,
    pub options: list,
    pub name: *mut i8,
    pub description: *mut i8,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_option {
    pub base: rxkb_object,
    pub name: *mut i8,
    pub brief: *mut i8,
    pub description: *mut i8,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_iso639_code {
    pub base: rxkb_object,
    pub code: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_iso3166_code {
    pub base: rxkb_object,
    pub code: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_item {
    pub name: *mut i8,
    pub description: *mut i8,
    pub brief: *mut i8,
    pub vendor: *mut i8,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}

macro_rules! rxkb_logf {
    ($ctx:expr, $level:expr, $($arg:tt)*) => {{
        let mut _rxkb_log_buf = [0u8; 2048];
        {
            let mut _w = crate::xkb::utils::LogBuf::new(&mut _rxkb_log_buf[..2047]);
            let _ = core::fmt::Write::write_fmt(&mut _w, format_args!($($arg)*));
        }
        rxkb_log($ctx, $level, _rxkb_log_buf.as_ptr() as *const i8)
    }};
}

unsafe fn rxkb_log(mut ctx: *mut rxkb_context, mut level: rxkb_log_level, mut msg: *const i8) {
    unsafe {
        if ((*ctx).log_level as u32) < level as u32 {
            return;
        }
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, msg);
    }
}
unsafe fn rxkb_object_init(mut object: *mut rxkb_object, mut parent: *mut rxkb_object) {
    unsafe {
        (*object).refcount = 1 as u32;
        (*object).parent = parent;
        list_init(&raw mut (*object).link);
    }
}
unsafe fn rxkb_object_ref(mut object: *mut rxkb_object) -> *mut ::core::ffi::c_void {
    unsafe {
        (*object).refcount = (*object).refcount.wrapping_add(1);
        return object as *mut ::core::ffi::c_void;
    }
}
pub unsafe fn rxkb_layout_get_iso639_first(mut layout: *mut rxkb_layout) -> *mut rxkb_iso639_code {
    unsafe {
        let mut code: *mut rxkb_iso639_code = std::ptr::null_mut();
        if !list_empty(&raw mut (*layout).iso639s) {
            code = ((*layout).iso639s.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        }
        return code;
    }
}
pub unsafe fn rxkb_iso639_code_next(mut code: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        let mut next: *mut rxkb_iso639_code = std::ptr::null_mut();
        let mut layout: *mut rxkb_layout = std::ptr::null_mut();
        layout = ((*code).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*layout).iso639s, &raw mut (*code).base.link) {
            return std::ptr::null_mut();
        }
        next = ((*code).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        return next;
    }
}

pub unsafe fn rxkb_iso639_code_unref(mut object: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            free((*object).code as *mut ::core::ffi::c_void);
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}
#[inline]
unsafe fn rxkb_iso639_code_create(mut parent: *mut rxkb_object) -> *mut rxkb_iso639_code {
    unsafe {
        let mut t: *mut rxkb_iso639_code =
            Box::into_raw(Box::new(std::mem::zeroed::<rxkb_iso639_code>()));
        rxkb_object_init(&raw mut (*t).base, parent);
        return t;
    }
}
pub unsafe fn rxkb_iso639_code_get_code(mut object: *mut rxkb_iso639_code) -> *const i8 {
    unsafe {
        return (*object).code;
    }
}
pub unsafe fn rxkb_layout_get_iso3166_first(
    mut layout: *mut rxkb_layout,
) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut code: *mut rxkb_iso3166_code = std::ptr::null_mut();
        if !list_empty(&raw mut (*layout).iso3166s) {
            code = ((*layout).iso3166s.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        }
        return code;
    }
}
pub unsafe fn rxkb_iso3166_code_next(mut code: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut next: *mut rxkb_iso3166_code = std::ptr::null_mut();
        let mut layout: *mut rxkb_layout = std::ptr::null_mut();
        layout = ((*code).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*layout).iso3166s, &raw mut (*code).base.link) {
            return std::ptr::null_mut();
        }
        next = ((*code).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        return next;
    }
}

pub unsafe fn rxkb_iso3166_code_unref(
    mut object: *mut rxkb_iso3166_code,
) -> *mut rxkb_iso3166_code {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            free((*object).code as *mut ::core::ffi::c_void);
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}

#[inline]
unsafe fn rxkb_iso3166_code_create(mut parent: *mut rxkb_object) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut t: *mut rxkb_iso3166_code =
            Box::into_raw(Box::new(std::mem::zeroed::<rxkb_iso3166_code>()));
        rxkb_object_init(&raw mut (*t).base, parent);
        return t;
    }
}
pub unsafe fn rxkb_iso3166_code_get_code(mut object: *mut rxkb_iso3166_code) -> *const i8 {
    unsafe {
        return (*object).code;
    }
}

pub unsafe fn rxkb_option_unref(mut object: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            // rxkb_option_destroy inlined
            free((*object).name as *mut ::core::ffi::c_void);
            free((*object).brief as *mut ::core::ffi::c_void);
            free((*object).description as *mut ::core::ffi::c_void);
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}

pub unsafe fn rxkb_option_get_name(mut object: *mut rxkb_option) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_option_get_brief(mut object: *mut rxkb_option) -> *const i8 {
    unsafe {
        return (*object).brief;
    }
}
pub unsafe fn rxkb_option_get_description(mut object: *mut rxkb_option) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}

pub unsafe fn rxkb_option_is_layout_specific(mut object: *mut rxkb_option) -> bool {
    unsafe {
        return (*object).layout_specific;
    }
}
pub unsafe fn rxkb_option_next(mut o: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        let mut parent: *mut rxkb_option_group = std::ptr::null_mut();
        let mut next: *mut rxkb_option = std::ptr::null_mut();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        if list_is_last(&raw mut (*parent).options, &raw mut (*o).base.link) {
            return std::ptr::null_mut();
        }
        return next;
    }
}
pub unsafe fn rxkb_option_first(mut parent: *mut rxkb_option_group) -> *mut rxkb_option {
    unsafe {
        let mut o: *mut rxkb_option = std::ptr::null_mut();
        if !list_empty(&raw mut (*parent).options) {
            o = ((*parent).options.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option as *mut rxkb_option;
        }
        return o;
    }
}
unsafe fn rxkb_layout_destroy(mut l: *mut rxkb_layout) {
    unsafe {
        let mut iso639: *mut rxkb_iso639_code = std::ptr::null_mut();
        let mut tmp_639: *mut rxkb_iso639_code = std::ptr::null_mut();
        let mut iso3166: *mut rxkb_iso3166_code = std::ptr::null_mut();
        let mut tmp_3166: *mut rxkb_iso3166_code = std::ptr::null_mut();
        free((*l).name as *mut ::core::ffi::c_void);
        free((*l).brief as *mut ::core::ffi::c_void);
        free((*l).description as *mut ::core::ffi::c_void);
        free((*l).variant as *mut ::core::ffi::c_void);
        iso639 = std::ptr::null_mut();
        tmp_639 = std::ptr::null_mut();
        iso639 = ((*l).iso639s.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        tmp_639 = ((*iso639).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        while &raw mut (*iso639).base.link != &raw mut (*l).iso639s {
            rxkb_iso639_code_unref(iso639);
            iso639 = tmp_639;
            tmp_639 = ((*iso639).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        }
        iso3166 = std::ptr::null_mut();
        tmp_3166 = std::ptr::null_mut();
        iso3166 = ((*l).iso3166s.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        tmp_3166 = ((*iso3166).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        while &raw mut (*iso3166).base.link != &raw mut (*l).iso3166s {
            rxkb_iso3166_code_unref(iso3166);
            iso3166 = tmp_3166;
            tmp_3166 = ((*iso3166).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        }
    }
}

pub unsafe fn rxkb_layout_unref(mut object: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_layout_destroy(object);
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}
#[inline]
unsafe fn rxkb_layout_create(mut parent: *mut rxkb_object) -> *mut rxkb_layout {
    unsafe {
        let mut t: *mut rxkb_layout = Box::into_raw(Box::new(std::mem::zeroed::<rxkb_layout>()));
        rxkb_object_init(&raw mut (*t).base, parent);
        return t;
    }
}
pub unsafe fn rxkb_layout_get_name(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_layout_get_brief(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).brief;
    }
}
pub unsafe fn rxkb_layout_get_description(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}
pub unsafe fn rxkb_layout_get_variant(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).variant;
    }
}

pub unsafe fn rxkb_layout_first(mut parent: *mut rxkb_context) -> *mut rxkb_layout {
    unsafe {
        let mut o: *mut rxkb_layout = std::ptr::null_mut();
        if !list_empty(&raw mut (*parent).layouts) {
            o = ((*parent).layouts.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_layout as *mut rxkb_layout;
        }
        return o;
    }
}
pub unsafe fn rxkb_layout_next(mut o: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        let mut parent: *mut rxkb_context = std::ptr::null_mut();
        let mut next: *mut rxkb_layout = std::ptr::null_mut();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*parent).layouts, &raw mut (*o).base.link) {
            return std::ptr::null_mut();
        }
        return next;
    }
}

pub unsafe fn rxkb_model_unref(mut object: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            // rxkb_model_destroy inlined
            free((*object).name as *mut ::core::ffi::c_void);
            free((*object).vendor as *mut ::core::ffi::c_void);
            free((*object).description as *mut ::core::ffi::c_void);
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}
pub unsafe fn rxkb_model_get_name(mut object: *mut rxkb_model) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_model_get_vendor(mut object: *mut rxkb_model) -> *const i8 {
    unsafe {
        return (*object).vendor;
    }
}
pub unsafe fn rxkb_model_get_description(mut object: *mut rxkb_model) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}

pub unsafe fn rxkb_model_next(mut o: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        let mut parent: *mut rxkb_context = std::ptr::null_mut();
        let mut next: *mut rxkb_model = std::ptr::null_mut();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        if list_is_last(&raw mut (*parent).models, &raw mut (*o).base.link) {
            return std::ptr::null_mut();
        }
        return next;
    }
}
pub unsafe fn rxkb_model_first(mut parent: *mut rxkb_context) -> *mut rxkb_model {
    unsafe {
        let mut o: *mut rxkb_model = std::ptr::null_mut();
        if !list_empty(&raw mut (*parent).models) {
            o = ((*parent).models.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
                as *mut rxkb_model;
        }
        return o;
    }
}
pub unsafe fn rxkb_option_group_allows_multiple(mut g: *mut rxkb_option_group) -> bool {
    unsafe {
        return (*g).allow_multiple;
    }
}

pub unsafe fn rxkb_option_group_unref(
    mut object: *mut rxkb_option_group,
) -> *mut rxkb_option_group {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            // rxkb_option_group_destroy inlined
            {
                let mut o: *mut rxkb_option = std::ptr::null_mut();
                let mut otmp: *mut rxkb_option = std::ptr::null_mut();
                free((*object).name as *mut ::core::ffi::c_void);
                free((*object).description as *mut ::core::ffi::c_void);
                o = ((*object).options.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_option as *mut rxkb_option;
                otmp = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_option as *mut rxkb_option;
                while &raw mut (*o).base.link != &raw mut (*object).options {
                    rxkb_option_unref(o);
                    o = otmp;
                    otmp = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                        as *mut rxkb_option as *mut rxkb_option;
                }
            }
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}
pub unsafe fn rxkb_option_group_get_name(mut object: *mut rxkb_option_group) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_option_group_get_description(mut object: *mut rxkb_option_group) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}

pub unsafe fn rxkb_option_group_first(mut parent: *mut rxkb_context) -> *mut rxkb_option_group {
    unsafe {
        let mut o: *mut rxkb_option_group = std::ptr::null_mut();
        if !list_empty(&raw mut (*parent).option_groups) {
            o = ((*parent).option_groups.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option_group as *mut rxkb_option_group;
        }
        return o;
    }
}
pub unsafe fn rxkb_option_group_next(mut o: *mut rxkb_option_group) -> *mut rxkb_option_group {
    unsafe {
        let mut parent: *mut rxkb_context = std::ptr::null_mut();
        let mut next: *mut rxkb_option_group = std::ptr::null_mut();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        if list_is_last(&raw mut (*parent).option_groups, &raw mut (*o).base.link) {
            return std::ptr::null_mut();
        }
        return next;
    }
}
unsafe fn rxkb_context_destroy(mut ctx: *mut rxkb_context) {
    unsafe {
        let mut m: *mut rxkb_model = std::ptr::null_mut();
        let mut mtmp: *mut rxkb_model = std::ptr::null_mut();
        let mut l: *mut rxkb_layout = std::ptr::null_mut();
        let mut ltmp: *mut rxkb_layout = std::ptr::null_mut();
        let mut og: *mut rxkb_option_group = std::ptr::null_mut();
        let mut ogtmp: *mut rxkb_option_group = std::ptr::null_mut();
        let mut path: *mut *mut i8 = std::ptr::null_mut();
        m = std::ptr::null_mut();
        mtmp = std::ptr::null_mut();
        m = ((*ctx).models.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        mtmp = ((*m).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        while &raw mut (*m).base.link != &raw mut (*ctx).models {
            rxkb_model_unref(m);
            m = mtmp;
            mtmp = ((*m).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
                as *mut rxkb_model;
        }
        l = std::ptr::null_mut();
        ltmp = std::ptr::null_mut();
        l = ((*ctx).layouts.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        ltmp = ((*l).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        while &raw mut (*l).base.link != &raw mut (*ctx).layouts {
            rxkb_layout_unref(l);
            l = ltmp;
            ltmp = ((*l).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_layout as *mut rxkb_layout;
        }
        og = std::ptr::null_mut();
        ogtmp = std::ptr::null_mut();
        og = ((*ctx).option_groups.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        ogtmp = ((*og).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        while &raw mut (*og).base.link != &raw mut (*ctx).option_groups {
            rxkb_option_group_unref(og);
            og = ogtmp;
            ogtmp = ((*og).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option_group as *mut rxkb_option_group;
        }
        for &p in &(*ctx).includes {
            cstr_free(p);
        }
        (*ctx).includes.clear();
    }
}

pub unsafe fn rxkb_context_unref(mut object: *mut rxkb_context) -> *mut rxkb_context {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_context_destroy(object);
            list_remove(&raw mut (*object).base.link);
            drop(Box::from_raw(object));
        }
        return std::ptr::null_mut();
    }
}

unsafe fn rxkb_context_getenv(mut ctx: *mut rxkb_context, mut name: *const i8) -> *mut i8 {
    unsafe {
        if (*ctx).use_secure_getenv {
            return secure_getenv(name);
        } else {
            return getenv(name);
        };
    }
}
pub unsafe fn rxkb_context_set_log_level(mut ctx: *mut rxkb_context, mut level: rxkb_log_level) {
    unsafe {
        (*ctx).log_level = level;
    }
}
unsafe fn log_level_to_prefix(mut level: rxkb_log_level) -> *const i8 {
    match level as u32 {
        50 => return b"xkbregistry: DEBUG: \0".as_ptr() as *const i8,
        40 => return b"xkbregistry: INFO: \0".as_ptr() as *const i8,
        30 => {
            return b"xkbregistry: WARNING: \0".as_ptr() as *const i8;
        }
        20 => return b"xkbregistry: ERROR: \0".as_ptr() as *const i8,
        10 => {
            return b"xkbregistry: CRITICAL: \0".as_ptr() as *const i8;
        }
        _ => return std::ptr::null(),
    };
}
unsafe fn default_log_fn(
    mut ctx: *mut rxkb_context,
    mut level: rxkb_log_level,
    mut msg: *const i8,
) {
    unsafe {
        let mut prefix: *const i8 = log_level_to_prefix(level);
        if !prefix.is_null() {
            eprint!("{}", crate::xkb::utils::CStrDisplay(prefix));
        }
        eprint!("{}", crate::xkb::utils::CStrDisplay(msg));
    }
}
unsafe fn log_level(mut level: *const i8) -> rxkb_log_level {
    unsafe {
        let mut endptr: *mut i8 = std::ptr::null_mut();
        let mut lvl: rxkb_log_level = 0 as rxkb_log_level;
        *__errno_location() = 0 as i32;
        lvl = strtol(level, &raw mut endptr, 10 as i32) as rxkb_log_level;
        if *__errno_location() == 0 as i32
            && (*endptr.offset(0 as i32 as isize) as i32 == '\0' as i32
                || is_space(*endptr.offset(0 as i32 as isize)) as i32 != 0)
        {
            return lvl;
        }
        if istrneq(
            b"crit\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(
            b"err\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_ERROR;
        }
        if istrneq(
            b"warn\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_WARNING;
        }
        if istrneq(
            b"info\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_INFO;
        }
        if istrneq(
            b"debug\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
        ) as i32
            != 0
            || istrneq(
                b"dbg\0".as_ptr() as *const i8,
                level,
                (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1 as usize),
            ) as i32
                != 0
        {
            return RXKB_LOG_LEVEL_DEBUG;
        }
        return RXKB_LOG_LEVEL_ERROR;
    }
}
pub unsafe fn rxkb_context_new(mut flags: rxkb_context_flags) -> *mut rxkb_context {
    unsafe {
        // rxkb_context_create inlined
        let mut ctx: *mut rxkb_context = {
            let layout = std::alloc::Layout::new::<rxkb_context>();
            let ptr = std::alloc::alloc_zeroed(layout) as *mut rxkb_context;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            std::ptr::write(&raw mut (*ptr).includes, Vec::new());
            let t: *mut rxkb_context = Box::into_raw(Box::from_raw(ptr));
            rxkb_object_init(&raw mut (*t).base, std::ptr::null_mut());
            t
        };
        let mut env: *const i8 = std::ptr::null();
        if ctx.is_null() {
            return std::ptr::null_mut();
        }
        (*ctx).context_state = CONTEXT_NEW;
        (*ctx).load_extra_rules_files = flags as u32 & RXKB_CONTEXT_LOAD_EXOTIC_RULES as u32 != 0;
        (*ctx).use_secure_getenv = flags as u32 & RXKB_CONTEXT_NO_SECURE_GETENV as u32 == 0;
        (*ctx).log_fn =
            Some(default_log_fn as unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ())
                as Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>;
        (*ctx).log_level = RXKB_LOG_LEVEL_ERROR;
        env = rxkb_context_getenv(ctx, b"RXKB_LOG_LEVEL\0".as_ptr() as *const i8);
        if !env.is_null() {
            rxkb_context_set_log_level(ctx, log_level(env));
        }
        static mut RXKB_CONTEXT_FLAGS: rxkb_context_flags =
            (RXKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
                | RXKB_CONTEXT_LOAD_EXOTIC_RULES as i32
                | RXKB_CONTEXT_NO_SECURE_GETENV as i32) as rxkb_context_flags;
        if flags as u32 & !(RXKB_CONTEXT_FLAGS as u32) != 0 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "{}: Invalid context flags: 0x{:x}\n",
                crate::xkb::utils::CStrDisplay(b"rxkb_context_new\0".as_ptr() as *const i8),
                flags as u32 & !(RXKB_CONTEXT_FLAGS as u32),
            );
            drop(Box::from_raw(ctx));
            return std::ptr::null_mut();
        }
        list_init(&raw mut (*ctx).models);
        list_init(&raw mut (*ctx).layouts);
        list_init(&raw mut (*ctx).option_groups);
        if flags as u32 & RXKB_CONTEXT_NO_DEFAULT_INCLUDES as u32 == 0
            && !rxkb_context_include_path_append_default(ctx)
        {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "[XKB-{:03}] Failed to add any default include path (default system path: {})\n",
                XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                crate::xkb::utils::CStrDisplay(
                    b"/usr/share/xkeyboard-config-2\0".as_ptr() as *const i8
                ),
            );
            rxkb_context_unref(ctx);
            return std::ptr::null_mut();
        }
        return ctx;
    }
}

pub unsafe fn rxkb_context_include_path_append(
    mut ctx: *mut rxkb_context,
    mut path: *const i8,
) -> bool {
    unsafe {
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
        let mut rules: [i8; 4096] = [0; 4096];
        let mut tmp: *mut i8 = std::ptr::null_mut();
        let mut err: i32 = 0 as i32;
        if (*ctx).context_state as u32 != CONTEXT_NEW as u32 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
        } else {
            stat_buf = stat {
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
            err = xkb_stat(path, &raw mut stat_buf);
            if err != 0 as i32 {
                err = *__errno_location();
            } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
                err = ENOTDIR;
            } else if !check_eaccess(path, R_OK | X_OK) {
                err = EACCES;
            } else {
                rules = [0; 4096];
                let (_, _trunc) = crate::xkb::utils::snprintf_args(
                    &raw mut rules as *mut i8,
                    std::mem::size_of::<[i8; 4096]>(),
                    format_args!(
                        "{}/rules/{}.xml",
                        crate::xkb::utils::CStrDisplay(path),
                        crate::xkb::utils::CStrDisplay(DEFAULT_XKB_RULES.as_ptr())
                    ),
                );
                if _trunc {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        "[XKB-{:03}] Path is too long: expected max length of {}, got: {}/rules/{}.xml\n",
                        XKB_ERROR_INVALID_PATH as i32,
                        std::mem::size_of::<[i8; 4096]>(),
                        crate::xkb::utils::CStrDisplay(path),
                        crate::xkb::utils::CStrDisplay(b"evdev\0".as_ptr() as *const i8),
                    );
                } else {
                    tmp = cstr_dup(path);
                    if tmp.is_null() {
                        err = ENOMEM;
                    } else {
                        (&mut (*ctx).includes).push(tmp);
                        rxkb_logf!(
                            ctx,
                            RXKB_LOG_LEVEL_INFO,
                            "Include path added: {}\n",
                            crate::xkb::utils::CStrDisplay(tmp),
                        );
                        return true;
                    }
                }
            }
        }
        rxkb_logf!(
            ctx,
            RXKB_LOG_LEVEL_INFO,
            "Include path failed: \"{}\" ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        return false;
    }
}
unsafe extern "C" fn compare_str(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> i32 {
    unsafe {
        return cstr_cmp(*(a as *mut *mut i8), *(b as *mut *mut i8));
    }
}
unsafe fn add_direct_subdirectories(
    mut ctx: *mut rxkb_context,
    mut path: *const i8,
    extensions: &mut Vec<*mut i8>,
    mut versioned_count: u32,
    mut versioned_path_length: usize,
) -> i32 {
    unsafe {
        let mut entry: *mut dirent = std::ptr::null_mut();
        let mut path_buf: [i8; 4096] = [0; 4096];
        let mut c2rust_current_block: u64;
        let mut ret: i32 = 0 as i32;
        let mut err: i32 = ENOMEM;
        let mut dir: *mut DIR = std::ptr::null_mut();
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
        err = xkb_stat(path, &raw mut stat_buf);
        if err != 0 as i32 {
            err = *__errno_location();
        } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
            err = ENOTDIR;
        } else if !check_eaccess(path, R_OK | X_OK) {
            err = EACCES;
        } else {
            dir = opendir(path);
            if dir.is_null() {
                err = EACCES;
            } else {
                entry = std::ptr::null_mut();
                path_buf = ::core::mem::transmute::<
                    [u8; 4096],
                    [i8; 4096],
                >(
                    *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                );
                versioned_path_length = versioned_path_length.wrapping_add(1);
                's_62: loop {
                    entry = readdir(dir);
                    if entry.is_null() {
                        c2rust_current_block = 14434620278749266018;
                        break;
                    }
                    let mut name: *const i8 = &raw mut (*entry).d_name as *mut i8;
                    if cstr_cmp(name, b".\0".as_ptr() as *const i8) == 0 as i32
                        || cstr_cmp(name, b"..\0".as_ptr() as *const i8) == 0 as i32
                    {
                        continue;
                    }
                    let (_, _trunc) = crate::xkb::utils::snprintf_args(
                        &raw mut path_buf as *mut i8,
                        std::mem::size_of::<[i8; 4096]>(),
                        format_args!(
                            "{}/{}",
                            crate::xkb::utils::CStrDisplay(path),
                            crate::xkb::utils::CStrDisplay(name)
                        ),
                    );
                    if _trunc {
                        err = ENOMEM;
                        c2rust_current_block = 17009998909239196508;
                        break;
                    } else {
                        if xkb_stat(&raw mut path_buf as *mut i8, &raw mut stat_buf) != 0 as i32
                            || !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32)
                        {
                            continue;
                        }
                        let mut i: u32 = 0 as u32;
                        while i < versioned_count {
                            let prev_name: *const i8 =
                                (extensions[i as usize]).offset(versioned_path_length as isize);
                            if cstr_cmp(name, prev_name) == 0 as i32 {
                                continue 's_62;
                            }
                            i = i.wrapping_add(1);
                        }
                        let mut ext_path: *mut i8 = strdup_safe(&raw mut path_buf as *mut i8);
                        if ext_path.is_null() {
                            err = ENOMEM;
                            c2rust_current_block = 17009998909239196508;
                            break;
                        } else {
                            extensions.push(ext_path);
                        }
                    }
                }
                match c2rust_current_block {
                    17009998909239196508 => {}
                    _ => {
                        closedir(dir);
                        if extensions.len() > versioned_count as usize {
                            qsort(
                                extensions.as_mut_ptr().add(versioned_count as usize)
                                    as *mut ::core::ffi::c_void,
                                extensions.len().wrapping_sub(versioned_count as usize),
                                std::mem::size_of::<*mut i8>(),
                                Some(
                                    compare_str
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        )
                                            -> i32,
                                ),
                            );
                            for i in (versioned_count as usize)..extensions.len() {
                                ret |= rxkb_context_include_path_append(ctx, extensions[i]) as i32;
                            }
                        }
                        return ret;
                    }
                }
            }
        }
        rxkb_logf!(
            ctx,
            RXKB_LOG_LEVEL_DEBUG,
            "Include extensions path failed: {} ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        if !dir.is_null() {
            closedir(dir);
        }
        return ret;
    }
}
pub unsafe fn rxkb_context_include_path_append_default(mut ctx: *mut rxkb_context) -> bool {
    unsafe {
        let mut user_path: [i8; 4096] = [0; 4096];
        let mut ret: i32 = if false { 1 } else { 0 };
        if (*ctx).context_state as u32 != CONTEXT_NEW as u32 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
            return false;
        }
        let home: *const i8 = rxkb_context_getenv(ctx, b"HOME\0".as_ptr() as *const i8);
        let xdg: *const i8 = rxkb_context_getenv(ctx, b"XDG_CONFIG_HOME\0".as_ptr() as *const i8);
        if !xdg.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                format_args!("{}/xkb", crate::xkb::utils::CStrDisplay(xdg)),
            );
            if !_trunc {
                ret = ret as i32
                    | rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        } else if !home.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                format_args!("{}/.config/xkb", crate::xkb::utils::CStrDisplay(home)),
            );
            if !_trunc {
                ret = ret as i32
                    | rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        }
        if !home.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                format_args!("{}/.xkb", crate::xkb::utils::CStrDisplay(home)),
            );
            if !_trunc {
                ret = ret as i32
                    | rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        }
        let extra: *const i8 =
            rxkb_context_getenv(ctx, b"XKB_CONFIG_EXTRA_PATH\0".as_ptr() as *const i8);
        ret = ret as i32
            | rxkb_context_include_path_append(
                ctx,
                if !extra.is_null() {
                    extra
                } else {
                    DFLT_XKB_CONFIG_EXTRA_PATH.as_ptr()
                },
            ) as i32;
        let mut extensions: Vec<*mut i8> = Vec::new();
        let mut extensions_path: *const i8 = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_VERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const i8,
        );
        if extensions_path.is_null() {
            extensions_path = DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.as_ptr();
        }
        let mut versioned_path_length: usize = 0 as usize;
        if !extensions_path.is_null() {
            ret = ret as i32
                | add_direct_subdirectories(
                    ctx,
                    extensions_path,
                    &mut extensions,
                    0 as u32,
                    0 as usize,
                );
            versioned_path_length = cstr_len(extensions_path);
        }
        extensions_path = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const i8,
        );
        if extensions_path.is_null() {
            extensions_path = DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.as_ptr();
        }
        if !extensions_path.is_null() {
            let ext_count = extensions.len() as u32;
            ret = ret as i32
                | add_direct_subdirectories(
                    ctx,
                    extensions_path,
                    &mut extensions,
                    ext_count,
                    versioned_path_length,
                );
        }
        for &p in &extensions {
            cstr_free(p);
        }
        drop(extensions);
        let root: *const i8 = rxkb_context_getenv(ctx, b"XKB_CONFIG_ROOT\0".as_ptr() as *const i8);
        let has_root: bool = rxkb_context_include_path_append(
            ctx,
            if !root.is_null() {
                root
            } else {
                DFLT_XKB_CONFIG_ROOT.as_ptr()
            },
        ) as bool;
        ret = ret as i32 | has_root as i32;
        if !has_root && (root.is_null() || *root.offset(0 as i32 as isize) as i32 != '\0' as i32) {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_WARNING,
                "Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                crate::xkb::utils::CStrDisplay(b"/usr/share/X11/xkb\0".as_ptr() as *const i8),
                crate::xkb::utils::CStrDisplay(if root.is_null() {
                    b"/usr/share/xkeyboard-config-2\0".as_ptr()
                        as *const i8
                } else {
                    root
                }),
            );
            ret = ret as i32
                | rxkb_context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT.as_ptr()) as i32;
        }
        return ret != 0;
    }
}

pub unsafe fn rxkb_context_parse(mut ctx: *mut rxkb_context, mut ruleset: *const i8) -> bool {
    unsafe {
        let mut path: *mut i8;
        let mut success: bool = false;
        if (*ctx).context_state as u32 != CONTEXT_NEW as u32 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "parse must only be called on a new context\n",
            );
            return false;
        }
        if !(&(*ctx).includes).is_empty() {
            let mut idx = (&(*ctx).includes).len();
            while idx > 0 {
                idx -= 1;
                path = (&(*ctx).includes)[idx];
                let mut rules: [i8; 4096] = [0; 4096];
                let (_, _trunc) = crate::xkb::utils::snprintf_args(
                    &raw mut rules as *mut i8,
                    std::mem::size_of::<[i8; 4096]>(),
                    format_args!(
                        "{}/rules/{}.xml",
                        crate::xkb::utils::CStrDisplay(path),
                        crate::xkb::utils::CStrDisplay(ruleset)
                    ),
                );
                if !_trunc {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_DEBUG,
                        "Parsing {}\n",
                        crate::xkb::utils::CStrDisplay(&raw mut rules as *mut i8),
                    );
                    if parse(ctx, &raw mut rules as *mut i8, RXKB_POPULARITY_STANDARD) {
                        success = true;
                    }
                }
                if (*ctx).load_extra_rules_files as i32 != 0 {
                    let (_, _trunc) = crate::xkb::utils::snprintf_args(
                        &raw mut rules as *mut i8,
                        std::mem::size_of::<[i8; 4096]>(),
                        format_args!(
                            "{}/rules/{}.extras.xml",
                            crate::xkb::utils::CStrDisplay(path),
                            crate::xkb::utils::CStrDisplay(ruleset)
                        ),
                    );
                    if !_trunc {
                        rxkb_logf!(
                            ctx,
                            RXKB_LOG_LEVEL_DEBUG,
                            "Parsing {}\n",
                            crate::xkb::utils::CStrDisplay(&raw mut rules as *mut i8),
                        );
                        if parse(ctx, &raw mut rules as *mut i8, RXKB_POPULARITY_EXOTIC) {
                            success = true;
                        }
                    }
                }
            }
        }
        (*ctx).context_state = (if success as i32 != 0 {
            CONTEXT_PARSED as i32
        } else {
            CONTEXT_FAILED as i32
        }) as context_state;
        return success;
    }
}

/// Get an attribute value by name from an xmloxide element node.
fn get_attr<'a>(
    doc: &'a xmloxide::Document,
    node: xmloxide::tree::NodeId,
    name: &str,
) -> Option<&'a str> {
    doc.attributes(node)
        .iter()
        .find(|a| a.name == name)
        .map(|a| a.value.as_str())
}

#[inline]
fn is_node(doc: &xmloxide::Document, node: xmloxide::tree::NodeId, name: &str) -> bool {
    doc.is_element(node) && doc.node_name(node) == Some(name)
}
fn extract_text(doc: &xmloxide::Document, node: xmloxide::tree::NodeId) -> *mut i8 {
    for child in doc.children(node) {
        if let Some(text) = doc.node_text(child) {
            if !text.is_empty() {
                return match std::ffi::CString::new(text) {
                    Ok(cs) => cs.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                };
            }
        }
    }
    std::ptr::null_mut()
}
unsafe fn config_item_free(mut config: *mut config_item) {
    unsafe {
        free((*config).name as *mut ::core::ffi::c_void);
        free((*config).description as *mut ::core::ffi::c_void);
        free((*config).brief as *mut ::core::ffi::c_void);
        free((*config).vendor as *mut ::core::ffi::c_void);
    }
}
unsafe fn parse_config_item(
    mut ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    parent: xmloxide::tree::NodeId,
    mut config: *mut config_item,
) -> bool {
    unsafe {
        for ci in doc.children(parent) {
            if is_node(doc, ci, "configItem") {
                if let Some(raw_popularity) = get_attr(doc, ci, "popularity") {
                    if raw_popularity == "standard" {
                        (*config).popularity = RXKB_POPULARITY_STANDARD;
                    } else if raw_popularity == "exotic" {
                        (*config).popularity = RXKB_POPULARITY_EXOTIC;
                    } else {
                        rxkb_logf!(
                            ctx,
                            RXKB_LOG_LEVEL_ERROR,
                            "xml: invalid popularity attribute: expected 'standard' or 'exotic', got: '{}'\n",
                            raw_popularity,
                        );
                    }
                }
                if let Some(raw_layout_specific) = get_attr(doc, ci, "layout-specific") {
                    if raw_layout_specific == "true" {
                        (*config).layout_specific = true;
                    }
                }
                for node in doc.children(ci) {
                    if is_node(doc, node, "name") {
                        (*config).name = extract_text(doc, node);
                    } else if is_node(doc, node, "description") {
                        (*config).description = extract_text(doc, node);
                    } else if is_node(doc, node, "shortDescription") {
                        (*config).brief = extract_text(doc, node);
                    } else if is_node(doc, node, "vendor") {
                        (*config).vendor = extract_text(doc, node);
                    }
                }
                if (*config).name.is_null() || cstr_len((*config).name) == 0 {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        "xml: missing required element 'name'\n",
                    );
                    config_item_free(config);
                    return false;
                }
                return true;
            }
        }
        return false;
    }
}
unsafe fn parse_model(
    mut ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    model: xmloxide::tree::NodeId,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, doc, model, &raw mut config) {
            let mut m: *mut rxkb_model = std::ptr::null_mut();
            m = std::ptr::null_mut();
            m = ((*ctx).models.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
                as *mut rxkb_model;
            while &raw mut (*m).base.link != &raw mut (*ctx).models {
                if streq((*m).name, config.name) {
                    config_item_free(&raw mut config);
                    return;
                }
                m = ((*m).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_model as *mut rxkb_model;
            }
            m = Box::into_raw(Box::new(std::mem::zeroed::<rxkb_model>()));
            rxkb_object_init(&raw mut (*m).base, &raw mut (*ctx).base);
            (*m).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*m).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*m).vendor =
                _steal(&raw mut config.vendor as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*m).popularity = config.popularity;
            list_append(&raw mut (*ctx).models, &raw mut (*m).base.link);
        }
    }
}
unsafe fn parse_language_list(
    doc: &xmloxide::Document,
    language_list: xmloxide::tree::NodeId,
    mut layout: *mut rxkb_layout,
) {
    unsafe {
        let mut code: *mut rxkb_iso639_code = std::ptr::null_mut();
        for node in doc.children(language_list) {
            if is_node(doc, node, "iso639Id") {
                let mut str: *mut i8 = extract_text(doc, node);
                let mut parent: *mut rxkb_object = std::ptr::null_mut();
                if str.is_null() || cstr_len(str) != 3 as usize {
                    free(str as *mut ::core::ffi::c_void);
                } else {
                    parent = &raw mut (*layout).base;
                    code = rxkb_iso639_code_create(parent);
                    (*code).code = str;
                    list_append(&raw mut (*layout).iso639s, &raw mut (*code).base.link);
                }
            }
        }
    }
}
unsafe fn parse_country_list(
    doc: &xmloxide::Document,
    country_list: xmloxide::tree::NodeId,
    mut layout: *mut rxkb_layout,
) {
    unsafe {
        let mut code: *mut rxkb_iso3166_code = std::ptr::null_mut();
        for node in doc.children(country_list) {
            if is_node(doc, node, "iso3166Id") {
                let mut str: *mut i8 = extract_text(doc, node);
                let mut parent: *mut rxkb_object = std::ptr::null_mut();
                if str.is_null() || cstr_len(str) != 2 as usize {
                    free(str as *mut ::core::ffi::c_void);
                } else {
                    parent = &raw mut (*layout).base;
                    code = rxkb_iso3166_code_create(parent);
                    (*code).code = str;
                    list_append(&raw mut (*layout).iso3166s, &raw mut (*code).base.link);
                }
            }
        }
    }
}
unsafe fn parse_variant(
    mut ctx: *mut rxkb_context,
    mut l: *mut rxkb_layout,
    doc: &xmloxide::Document,
    variant: xmloxide::tree::NodeId,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, doc, variant, &raw mut config) {
            let mut v: *mut rxkb_layout = std::ptr::null_mut();
            let mut exists: bool = false;
            v = std::ptr::null_mut();
            v = ((*ctx).layouts.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
                as *mut rxkb_layout;
            while &raw mut (*v).base.link != &raw mut (*ctx).layouts {
                if streq_null((*v).variant, config.name) as i32 != 0
                    && streq((*v).name, (*l).name) as i32 != 0
                {
                    exists = true;
                    break;
                } else {
                    v = ((*v).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                        as *mut rxkb_layout as *mut rxkb_layout;
                }
            }
            if !exists {
                v = rxkb_layout_create(&raw mut (*ctx).base);
                list_init(&raw mut (*v).iso639s);
                list_init(&raw mut (*v).iso3166s);
                (*v).name = cstr_dup((*l).name);
                (*v).variant =
                    _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
                (*v).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                    as *mut i8 as *mut i8;
                (*v).brief = if config.brief.is_null() {
                    strdup_safe((*l).brief)
                } else {
                    _steal(&raw mut config.brief as *mut ::core::ffi::c_void) as *mut i8
                };
                (*v).popularity = config.popularity;
                list_append(&raw mut (*ctx).layouts, &raw mut (*v).base.link);
                for ci in doc.children(variant) {
                    if is_node(doc, ci, "configItem") {
                        let mut found_language_list: bool = false;
                        let mut found_country_list: bool = false;
                        for node in doc.children(ci) {
                            if is_node(doc, node, "languageList") {
                                parse_language_list(doc, node, v);
                                found_language_list = true;
                            }
                            if is_node(doc, node, "countryList") {
                                parse_country_list(doc, node, v);
                                found_country_list = true;
                            }
                        }
                        if !found_language_list {
                            let mut x: *mut rxkb_iso639_code = std::ptr::null_mut();
                            x = std::ptr::null_mut();
                            x = ((*l).iso639s.next as *mut i8).offset(-(16 as u64 as isize))
                                as *mut rxkb_iso639_code
                                as *mut rxkb_iso639_code;
                            while &raw mut (*x).base.link != &raw mut (*l).iso639s {
                                let mut code: *mut rxkb_iso639_code =
                                    rxkb_iso639_code_create(&raw mut (*v).base);
                                (*code).code = cstr_dup((*x).code);
                                list_append(&raw mut (*v).iso639s, &raw mut (*code).base.link);
                                x = ((*x).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                                    as *mut rxkb_iso639_code
                                    as *mut rxkb_iso639_code;
                            }
                        }
                        if !found_country_list {
                            let mut x_0: *mut rxkb_iso3166_code = std::ptr::null_mut();
                            x_0 = std::ptr::null_mut();
                            x_0 = ((*l).iso3166s.next as *mut i8).offset(-(16 as u64 as isize))
                                as *mut rxkb_iso3166_code
                                as *mut rxkb_iso3166_code;
                            while &raw mut (*x_0).base.link != &raw mut (*l).iso3166s {
                                let mut code_0: *mut rxkb_iso3166_code =
                                    rxkb_iso3166_code_create(&raw mut (*v).base);
                                (*code_0).code = cstr_dup((*x_0).code);
                                list_append(&raw mut (*v).iso3166s, &raw mut (*code_0).base.link);
                                x_0 = ((*x_0).base.link.next as *mut i8)
                                    .offset(-(16 as u64 as isize))
                                    as *mut rxkb_iso3166_code
                                    as *mut rxkb_iso3166_code;
                            }
                        }
                    }
                }
            } else {
                config_item_free(&raw mut config);
            }
        }
    }
}
unsafe fn parse_layout(
    mut ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    layout: xmloxide::tree::NodeId,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity: popularity,
            layout_specific: false,
        };
        let mut l: *mut rxkb_layout = std::ptr::null_mut();
        let mut exists: bool = false;
        if !parse_config_item(ctx, doc, layout, &raw mut config) {
            return;
        }
        l = std::ptr::null_mut();
        l = ((*ctx).layouts.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        while &raw mut (*l).base.link != &raw mut (*ctx).layouts {
            if streq((*l).name, config.name) as i32 != 0 && (*l).variant.is_null() {
                exists = true;
                break;
            } else {
                l = ((*l).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_layout as *mut rxkb_layout;
            }
        }
        if !exists {
            l = rxkb_layout_create(&raw mut (*ctx).base);
            list_init(&raw mut (*l).iso639s);
            list_init(&raw mut (*l).iso3166s);
            (*l).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*l).variant = std::ptr::null_mut();
            (*l).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*l).brief =
                _steal(&raw mut config.brief as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*l).popularity = config.popularity;
            list_append(&raw mut (*ctx).layouts, &raw mut (*l).base.link);
        } else {
            config_item_free(&raw mut config);
        }
        for node in doc.children(layout) {
            if is_node(doc, node, "variantList") {
                // parse_variant_list inlined
                for vnode in doc.children(node) {
                    if is_node(doc, vnode, "variant") {
                        parse_variant(ctx, l, doc, vnode, popularity);
                    }
                }
            }
            if !exists && is_node(doc, node, "configItem") {
                for ll in doc.children(node) {
                    if is_node(doc, ll, "languageList") {
                        parse_language_list(doc, ll, l);
                    }
                    if is_node(doc, ll, "countryList") {
                        parse_country_list(doc, ll, l);
                    }
                }
            }
        }
    }
}
unsafe fn parse_option(
    mut ctx: *mut rxkb_context,
    mut group: *mut rxkb_option_group,
    doc: &xmloxide::Document,
    option: xmloxide::tree::NodeId,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, doc, option, &raw mut config) {
            let mut o: *mut rxkb_option = std::ptr::null_mut();
            o = std::ptr::null_mut();
            o = ((*group).options.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_option
                as *mut rxkb_option;
            while &raw mut (*o).base.link != &raw mut (*group).options {
                if streq((*o).name, config.name) {
                    config_item_free(&raw mut config);
                    return;
                }
                o = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_option as *mut rxkb_option;
            }
            o = Box::into_raw(Box::new(std::mem::zeroed::<rxkb_option>()));
            rxkb_object_init(&raw mut (*o).base, &raw mut (*group).base);
            (*o).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*o).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*o).popularity = config.popularity;
            (*o).layout_specific = config.layout_specific;
            list_append(&raw mut (*group).options, &raw mut (*o).base.link);
        }
    }
}
unsafe fn parse_group(
    mut ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    group: xmloxide::tree::NodeId,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity: popularity,
            layout_specific: false,
        };
        let mut g: *mut rxkb_option_group = std::ptr::null_mut();
        let mut exists: bool = false;
        if !parse_config_item(ctx, doc, group, &raw mut config) {
            return;
        }
        g = std::ptr::null_mut();
        g = ((*ctx).option_groups.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        while &raw mut (*g).base.link != &raw mut (*ctx).option_groups {
            if streq((*g).name, config.name) {
                exists = true;
                break;
            } else {
                g = ((*g).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_option_group as *mut rxkb_option_group;
            }
        }
        if !exists {
            g = Box::into_raw(Box::new(std::mem::zeroed::<rxkb_option_group>()));
            rxkb_object_init(&raw mut (*g).base, &raw mut (*ctx).base);
            (*g).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*g).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*g).popularity = config.popularity;
            if let Some(multiple) = get_attr(doc, group, "allowMultipleSelection") {
                if multiple == "true" {
                    (*g).allow_multiple = true;
                }
            }
            list_init(&raw mut (*g).options);
            list_append(&raw mut (*ctx).option_groups, &raw mut (*g).base.link);
        } else {
            config_item_free(&raw mut config);
        }
        for node in doc.children(group) {
            if is_node(doc, node, "option") {
                parse_option(ctx, g, doc, node, popularity);
            }
        }
    }
}
const XKBCONFIG_DTD: &str = "\
<!ELEMENT xkbConfigRegistry (modelList?, layoutList?, optionList?)>\n\
<!ATTLIST xkbConfigRegistry version CDATA \"1.1\">\n\
<!ELEMENT modelList (model*)>\n\
<!ELEMENT model (configItem)>\n\
<!ELEMENT layoutList (layout*)>\n\
<!ELEMENT layout (configItem,  variantList?)>\n\
<!ELEMENT optionList (group*)>\n\
<!ELEMENT variantList (variant*)>\n\
<!ELEMENT variant (configItem)>\n\
<!ELEMENT group (configItem, option*)>\n\
<!ATTLIST group allowMultipleSelection (true|false) \"false\">\n\
<!ELEMENT option (configItem)>\n\
<!ELEMENT configItem (name, shortDescription?, description?, vendor?, countryList?, languageList?, hwList?)>\n\
<!ATTLIST configItem layout-specific (true|false) \"false\">\n\
<!ATTLIST configItem popularity (standard|exotic) #IMPLIED>\n\
<!ELEMENT name (#PCDATA)>\n\
<!ELEMENT shortDescription (#PCDATA)>\n\
<!ELEMENT description (#PCDATA)>\n\
<!ELEMENT vendor (#PCDATA)>\n\
<!ELEMENT countryList (iso3166Id+)>\n\
<!ELEMENT iso3166Id (#PCDATA)>\n\
<!ELEMENT languageList (iso639Id+)>\n\
<!ELEMENT iso639Id (#PCDATA)>\n\
<!ELEMENT hwList (hwId+)>\n\
<!ELEMENT hwId (#PCDATA)>";

unsafe fn parse(
    mut ctx: *mut rxkb_context,
    mut path: *const i8,
    mut popularity: rxkb_popularity,
) -> bool {
    unsafe {
        if !check_eaccess(path, R_OK) {
            return false;
        }
        // Convert C path to Rust &str
        let path_cstr = std::ffi::CStr::from_ptr(path);
        let path_str = match path_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let mut doc = match xmloxide::Document::parse_file(path_str) {
            Ok(d) => d,
            Err(_) => return false,
        };
        // validate_xml inlined
        {
            let dtd = match xmloxide::validation::dtd::parse_dtd(XKBCONFIG_DTD) {
                Ok(dtd) => dtd,
                Err(_) => {
                    rxkb_logf!(ctx, RXKB_LOG_LEVEL_ERROR, "Failed to load DTD\n",);
                    return false;
                }
            };
            let result = xmloxide::validation::dtd::validate(&mut doc, &dtd);
            if !result.is_valid {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_ERROR,
                    "XML error: failed to validate document at {}\n",
                    crate::xkb::utils::CStrDisplay(path),
                );
                return false;
            }
        }
        let root = match doc.root_element() {
            Some(r) => r,
            None => return false,
        };
        // parse_rules_xml inlined (with parse_model_list, parse_layout_list, parse_option_list inlined)
        for node in doc.children(root) {
            if is_node(&doc, node, "modelList") {
                for mnode in doc.children(node) {
                    if is_node(&doc, mnode, "model") {
                        parse_model(ctx, &doc, mnode, popularity);
                    }
                }
            } else if is_node(&doc, node, "layoutList") {
                for lnode in doc.children(node) {
                    if is_node(&doc, lnode, "layout") {
                        parse_layout(ctx, &doc, lnode, popularity);
                    }
                }
            } else if is_node(&doc, node, "optionList") {
                for onode in doc.children(node) {
                    if is_node(&doc, onode, "group") {
                        parse_group(ctx, &doc, onode, popularity);
                    }
                }
            }
        }
        return true;
    }
}
use crate::xkb::shared_types::*;
