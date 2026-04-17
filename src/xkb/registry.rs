use crate::xkb::messages::{XKB_ERROR_INVALID_PATH, XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH};
use crate::xkb::utils::{
    _steal, cstr_as_bytes, cstr_cmp, cstr_dup, cstr_free, cstr_len, istrneq, last_errno,
    strdup_safe, streq, streq_null,
};
use libc::{free, getenv};

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

/// Rust replacement for glibc `secure_getenv`.
/// Returns NULL if the process is running with elevated privileges (setuid/setgid).
unsafe fn secure_getenv(name: *const i8) -> *mut i8 {
    unsafe {
        if libc::getuid() != libc::geteuid() || libc::getgid() != libc::getegid() {
            return std::ptr::null_mut();
        }
        getenv(name)
    }
}
#[repr(C)]
pub struct rxkb_context {
    pub base: rxkb_object,
    pub context_state: context_state,
    pub load_extra_rules_files: bool,
    pub use_secure_getenv: bool,
    pub models: Vec<*mut rxkb_model>,
    pub layouts: Vec<*mut rxkb_layout>,
    pub option_groups: Vec<*mut rxkb_option_group>,
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
#[repr(C)]
pub struct rxkb_layout {
    pub base: rxkb_object,
    pub name: *mut i8,
    pub brief: *mut i8,
    pub description: *mut i8,
    pub variant: *mut i8,
    pub popularity: rxkb_popularity,
    pub iso639s: Vec<*mut rxkb_iso639_code>,
    pub iso3166s: Vec<*mut rxkb_iso3166_code>,
}
#[repr(C)]
pub struct rxkb_option_group {
    pub base: rxkb_object,
    pub allow_multiple: bool,
    pub options: Vec<*mut rxkb_option>,
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

unsafe fn rxkb_log(ctx: *mut rxkb_context, level: rxkb_log_level, msg: *const i8) {
    unsafe {
        if (*ctx).log_level < level {
            return;
        }
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, msg);
    }
}
unsafe fn rxkb_object_init(object: *mut rxkb_object, parent: *mut rxkb_object) {
    unsafe {
        (*object).refcount = 1_u32;
        (*object).parent = parent;
    }
}
pub unsafe fn rxkb_layout_get_iso639_first(layout: *mut rxkb_layout) -> *mut rxkb_iso639_code {
    unsafe {
        (*layout)
            .iso639s
            .first()
            .copied()
            .unwrap_or(std::ptr::null_mut())
    }
}
pub unsafe fn rxkb_iso639_code_next(code: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        let layout = (*code).base.parent as *mut rxkb_layout;
        let vec = &(*layout).iso639s;
        if let Some(idx) = vec.iter().position(|&p| p == code) {
            vec.get(idx + 1).copied().unwrap_or(std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}

pub unsafe fn rxkb_iso639_code_unref(object: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            free((*object).code as *mut ::core::ffi::c_void);
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}
#[inline]
unsafe fn rxkb_iso639_code_create(parent: *mut rxkb_object) -> *mut rxkb_iso639_code {
    unsafe {
        let t: *mut rxkb_iso639_code =
            Box::into_raw(Box::new(std::mem::zeroed::<rxkb_iso639_code>()));
        rxkb_object_init(&raw mut (*t).base, parent);
        t
    }
}
pub unsafe fn rxkb_iso639_code_get_code(object: *mut rxkb_iso639_code) -> *const i8 {
    unsafe { (*object).code }
}
pub unsafe fn rxkb_layout_get_iso3166_first(layout: *mut rxkb_layout) -> *mut rxkb_iso3166_code {
    unsafe {
        (*layout)
            .iso3166s
            .first()
            .copied()
            .unwrap_or(std::ptr::null_mut())
    }
}
pub unsafe fn rxkb_iso3166_code_next(code: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code {
    unsafe {
        let layout = (*code).base.parent as *mut rxkb_layout;
        let vec = &(*layout).iso3166s;
        if let Some(idx) = vec.iter().position(|&p| p == code) {
            vec.get(idx + 1).copied().unwrap_or(std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}

pub unsafe fn rxkb_iso3166_code_unref(object: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            free((*object).code as *mut ::core::ffi::c_void);
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}

#[inline]
unsafe fn rxkb_iso3166_code_create(parent: *mut rxkb_object) -> *mut rxkb_iso3166_code {
    unsafe {
        let t: *mut rxkb_iso3166_code =
            Box::into_raw(Box::new(std::mem::zeroed::<rxkb_iso3166_code>()));
        rxkb_object_init(&raw mut (*t).base, parent);
        t
    }
}
pub unsafe fn rxkb_iso3166_code_get_code(object: *mut rxkb_iso3166_code) -> *const i8 {
    unsafe { (*object).code }
}

pub unsafe fn rxkb_option_unref(object: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            // rxkb_option_destroy inlined
            free((*object).name as *mut ::core::ffi::c_void);
            free((*object).brief as *mut ::core::ffi::c_void);
            free((*object).description as *mut ::core::ffi::c_void);
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}

pub unsafe fn rxkb_option_get_name(object: *mut rxkb_option) -> *const i8 {
    unsafe { (*object).name }
}
pub unsafe fn rxkb_option_get_brief(object: *mut rxkb_option) -> *const i8 {
    unsafe { (*object).brief }
}
pub unsafe fn rxkb_option_get_description(object: *mut rxkb_option) -> *const i8 {
    unsafe { (*object).description }
}

pub unsafe fn rxkb_option_is_layout_specific(object: *mut rxkb_option) -> bool {
    unsafe { (*object).layout_specific }
}
pub unsafe fn rxkb_option_next(o: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        let parent = (*o).base.parent as *mut rxkb_option_group;
        let vec = &(*parent).options;
        if let Some(idx) = vec.iter().position(|&p| p == o) {
            vec.get(idx + 1).copied().unwrap_or(std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}
pub unsafe fn rxkb_option_first(parent: *mut rxkb_option_group) -> *mut rxkb_option {
    unsafe {
        (*parent)
            .options
            .first()
            .copied()
            .unwrap_or(std::ptr::null_mut())
    }
}
unsafe fn rxkb_layout_destroy(l: *mut rxkb_layout) {
    unsafe {
        free((*l).name as *mut ::core::ffi::c_void);
        free((*l).brief as *mut ::core::ffi::c_void);
        free((*l).description as *mut ::core::ffi::c_void);
        free((*l).variant as *mut ::core::ffi::c_void);
        for &code in &(*l).iso639s {
            rxkb_iso639_code_unref(code);
        }
        (*l).iso639s.clear();
        for &code in &(*l).iso3166s {
            rxkb_iso3166_code_unref(code);
        }
        (*l).iso3166s.clear();
    }
}

pub unsafe fn rxkb_layout_unref(object: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            rxkb_layout_destroy(object);
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}
#[inline]
unsafe fn rxkb_layout_create(parent: *mut rxkb_object) -> *mut rxkb_layout {
    unsafe {
        let layout = std::alloc::Layout::new::<rxkb_layout>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut rxkb_layout;
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        std::ptr::write(&raw mut (*ptr).iso639s, Vec::new());
        std::ptr::write(&raw mut (*ptr).iso3166s, Vec::new());
        let t: *mut rxkb_layout = Box::into_raw(Box::from_raw(ptr));
        rxkb_object_init(&raw mut (*t).base, parent);
        t
    }
}
pub unsafe fn rxkb_layout_get_name(object: *mut rxkb_layout) -> *const i8 {
    unsafe { (*object).name }
}
pub unsafe fn rxkb_layout_get_brief(object: *mut rxkb_layout) -> *const i8 {
    unsafe { (*object).brief }
}
pub unsafe fn rxkb_layout_get_description(object: *mut rxkb_layout) -> *const i8 {
    unsafe { (*object).description }
}
pub unsafe fn rxkb_layout_get_variant(object: *mut rxkb_layout) -> *const i8 {
    unsafe { (*object).variant }
}

pub unsafe fn rxkb_layout_first(parent: *mut rxkb_context) -> *mut rxkb_layout {
    unsafe {
        (*parent)
            .layouts
            .first()
            .copied()
            .unwrap_or(std::ptr::null_mut())
    }
}
pub unsafe fn rxkb_layout_next(o: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        let parent = (*o).base.parent as *mut rxkb_context;
        let vec = &(*parent).layouts;
        if let Some(idx) = vec.iter().position(|&p| p == o) {
            vec.get(idx + 1).copied().unwrap_or(std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}

pub unsafe fn rxkb_model_unref(object: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            // rxkb_model_destroy inlined
            free((*object).name as *mut ::core::ffi::c_void);
            free((*object).vendor as *mut ::core::ffi::c_void);
            free((*object).description as *mut ::core::ffi::c_void);
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}
pub unsafe fn rxkb_model_get_name(object: *mut rxkb_model) -> *const i8 {
    unsafe { (*object).name }
}
pub unsafe fn rxkb_model_get_vendor(object: *mut rxkb_model) -> *const i8 {
    unsafe { (*object).vendor }
}
pub unsafe fn rxkb_model_get_description(object: *mut rxkb_model) -> *const i8 {
    unsafe { (*object).description }
}

pub unsafe fn rxkb_model_next(o: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        let parent = (*o).base.parent as *mut rxkb_context;
        let vec = &(*parent).models;
        if let Some(idx) = vec.iter().position(|&p| p == o) {
            vec.get(idx + 1).copied().unwrap_or(std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}
pub unsafe fn rxkb_model_first(parent: *mut rxkb_context) -> *mut rxkb_model {
    unsafe {
        (*parent)
            .models
            .first()
            .copied()
            .unwrap_or(std::ptr::null_mut())
    }
}
pub unsafe fn rxkb_option_group_allows_multiple(g: *mut rxkb_option_group) -> bool {
    unsafe { (*g).allow_multiple }
}

pub unsafe fn rxkb_option_group_unref(object: *mut rxkb_option_group) -> *mut rxkb_option_group {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            // rxkb_option_group_destroy inlined
            free((*object).name as *mut ::core::ffi::c_void);
            free((*object).description as *mut ::core::ffi::c_void);
            for &o in &(*object).options {
                rxkb_option_unref(o);
            }
            (*object).options.clear();
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}
pub unsafe fn rxkb_option_group_get_name(object: *mut rxkb_option_group) -> *const i8 {
    unsafe { (*object).name }
}
pub unsafe fn rxkb_option_group_get_description(object: *mut rxkb_option_group) -> *const i8 {
    unsafe { (*object).description }
}

pub unsafe fn rxkb_option_group_first(parent: *mut rxkb_context) -> *mut rxkb_option_group {
    unsafe {
        (*parent)
            .option_groups
            .first()
            .copied()
            .unwrap_or(std::ptr::null_mut())
    }
}
pub unsafe fn rxkb_option_group_next(o: *mut rxkb_option_group) -> *mut rxkb_option_group {
    unsafe {
        let parent = (*o).base.parent as *mut rxkb_context;
        let vec = &(*parent).option_groups;
        if let Some(idx) = vec.iter().position(|&p| p == o) {
            vec.get(idx + 1).copied().unwrap_or(std::ptr::null_mut())
        } else {
            std::ptr::null_mut()
        }
    }
}
unsafe fn rxkb_context_destroy(ctx: *mut rxkb_context) {
    unsafe {
        for &m in &(*ctx).models {
            rxkb_model_unref(m);
        }
        (*ctx).models.clear();
        for &l in &(*ctx).layouts {
            rxkb_layout_unref(l);
        }
        (*ctx).layouts.clear();
        for &og in &(*ctx).option_groups {
            rxkb_option_group_unref(og);
        }
        (*ctx).option_groups.clear();
        for &p in &(*ctx).includes {
            cstr_free(p);
        }
        (*ctx).includes.clear();
    }
}

pub unsafe fn rxkb_context_unref(object: *mut rxkb_context) -> *mut rxkb_context {
    unsafe {
        if object.is_null() {
            return std::ptr::null_mut();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0_u32 {
            rxkb_context_destroy(object);
            drop(Box::from_raw(object));
        }
        std::ptr::null_mut()
    }
}

unsafe fn rxkb_context_getenv(ctx: *mut rxkb_context, name: *const i8) -> *mut i8 {
    unsafe {
        if (*ctx).use_secure_getenv {
            secure_getenv(name)
        } else {
            getenv(name)
        }
    }
}
pub unsafe fn rxkb_context_set_log_level(ctx: *mut rxkb_context, level: rxkb_log_level) {
    unsafe {
        (*ctx).log_level = level;
    }
}
unsafe fn log_level_to_prefix(level: rxkb_log_level) -> *const i8 {
    match level {
        50 => b"xkbregistry: DEBUG: \0".as_ptr() as *const i8,
        40 => b"xkbregistry: INFO: \0".as_ptr() as *const i8,
        30 => b"xkbregistry: WARNING: \0".as_ptr() as *const i8,
        20 => b"xkbregistry: ERROR: \0".as_ptr() as *const i8,
        10 => b"xkbregistry: CRITICAL: \0".as_ptr() as *const i8,
        _ => std::ptr::null(),
    }
}
unsafe fn default_log_fn(_ctx: *mut rxkb_context, level: rxkb_log_level, msg: *const i8) {
    unsafe {
        let prefix: *const i8 = log_level_to_prefix(level);
        if !prefix.is_null() {
            eprint!("{}", crate::xkb::utils::CStrDisplay(prefix));
        }
        eprint!("{}", crate::xkb::utils::CStrDisplay(msg));
    }
}
unsafe fn log_level(level: *const i8) -> rxkb_log_level {
    unsafe {
        let (val, consumed) = crate::xkb::utils::cstr_parse_long(level);
        if consumed > 0 {
            let after = *level.add(consumed);
            if after as i32 == '\0' as i32
                || matches!(after as u8, b' ' | b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r')
            {
                return val as rxkb_log_level;
            }
        }
        if istrneq(
            b"crit",
            cstr_as_bytes(level),
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1_usize),
        ) {
            return RXKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(
            b"err",
            cstr_as_bytes(level),
            (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1_usize),
        ) {
            return RXKB_LOG_LEVEL_ERROR;
        }
        if istrneq(
            b"warn",
            cstr_as_bytes(level),
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1_usize),
        ) {
            return RXKB_LOG_LEVEL_WARNING;
        }
        if istrneq(
            b"info",
            cstr_as_bytes(level),
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1_usize),
        ) {
            return RXKB_LOG_LEVEL_INFO;
        }
        if istrneq(
            b"debug",
            cstr_as_bytes(level),
            (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1_usize),
        ) as i32
            != 0
            || istrneq(
                b"dbg",
                cstr_as_bytes(level),
                (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1_usize),
            ) as i32
                != 0
        {
            return RXKB_LOG_LEVEL_DEBUG;
        }
        RXKB_LOG_LEVEL_ERROR
    }
}
pub unsafe fn rxkb_context_new(flags: rxkb_context_flags) -> *mut rxkb_context {
    unsafe {
        // rxkb_context_create inlined
        let ctx: *mut rxkb_context = {
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

        if ctx.is_null() {
            return std::ptr::null_mut();
        }
        (*ctx).context_state = CONTEXT_NEW;
        (*ctx).load_extra_rules_files = flags & RXKB_CONTEXT_LOAD_EXOTIC_RULES != 0;
        (*ctx).use_secure_getenv = flags & RXKB_CONTEXT_NO_SECURE_GETENV == 0;
        (*ctx).log_fn =
            Some(default_log_fn as unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ())
                as Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>;
        (*ctx).log_level = RXKB_LOG_LEVEL_ERROR;
        let env: *const i8 = rxkb_context_getenv(ctx, b"RXKB_LOG_LEVEL\0".as_ptr() as *const i8);
        if !env.is_null() {
            rxkb_context_set_log_level(ctx, log_level(env));
        }
        static RXKB_CONTEXT_FLAGS: rxkb_context_flags = (RXKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
            | RXKB_CONTEXT_LOAD_EXOTIC_RULES as i32
            | RXKB_CONTEXT_NO_SECURE_GETENV as i32)
            as rxkb_context_flags;
        if flags & !RXKB_CONTEXT_FLAGS != 0 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "{}: Invalid context flags: 0x{:x}\n",
                crate::xkb::utils::CStrDisplay(b"rxkb_context_new\0".as_ptr() as *const i8),
                flags & !RXKB_CONTEXT_FLAGS,
            );
            drop(Box::from_raw(ctx));
            return std::ptr::null_mut();
        }
        std::ptr::write(&raw mut (*ctx).models, Vec::new());
        std::ptr::write(&raw mut (*ctx).layouts, Vec::new());
        std::ptr::write(&raw mut (*ctx).option_groups, Vec::new());
        if flags & RXKB_CONTEXT_NO_DEFAULT_INCLUDES == 0
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
        ctx
    }
}

pub unsafe fn rxkb_context_include_path_append(ctx: *mut rxkb_context, path: *const i8) -> bool {
    unsafe {
        #[allow(unused_assignments)]
        let mut rules: [i8; 4096] = { std::mem::zeroed() };
        let tmp: *mut i8;
        let mut err: i32 = 0_i32;
        if (*ctx).context_state != CONTEXT_NEW {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
        } else {
            // Use std::fs::metadata instead of xkb_stat
            let path_str = match std::ffi::CStr::from_ptr(path).to_str() {
                Ok(s) => s,
                Err(_) => {
                    err = libc::EINVAL;
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_INFO,
                        "Include path failed: \"{}\" (invalid UTF-8)\n",
                        crate::xkb::utils::CStrDisplay(path),
                    );
                    return false;
                }
            };
            match std::fs::metadata(path_str) {
                Err(e) => {
                    err = e.raw_os_error().unwrap_or(0);
                }
                Ok(m) if !m.is_dir() => {
                    err = ENOTDIR;
                }
                Ok(_) => {
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
                            (*ctx).includes.push(tmp);
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
        }
        rxkb_logf!(
            ctx,
            RXKB_LOG_LEVEL_INFO,
            "Include path failed: \"{}\" ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        false
    }
}
unsafe fn add_direct_subdirectories(
    ctx: *mut rxkb_context,
    path: *const i8,
    extensions: &mut Vec<*mut i8>,
    versioned_count: u32,
    mut versioned_path_length: usize,
) -> i32 {
    unsafe {
        let mut ret: i32 = 0_i32;

        // Convert C path to Rust str
        let path_str = match std::ffi::CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(_) => {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_DEBUG,
                    "Include extensions path failed: {} (invalid UTF-8)\n",
                    crate::xkb::utils::CStrDisplay(path),
                );
                return ret;
            }
        };

        // Check path is a directory
        let meta = match std::fs::metadata(path_str) {
            Ok(m) => m,
            Err(e) => {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_DEBUG,
                    "Include extensions path failed: {} ({})\n",
                    crate::xkb::utils::CStrDisplay(path),
                    crate::xkb::utils::StrerrorDisplay(e.raw_os_error().unwrap_or(0)),
                );
                return ret;
            }
        };
        if !meta.is_dir() {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_DEBUG,
                "Include extensions path failed: {} ({})\n",
                crate::xkb::utils::CStrDisplay(path),
                crate::xkb::utils::StrerrorDisplay(ENOTDIR),
            );
            return ret;
        }

        // Read directory entries
        let dir_entries = match std::fs::read_dir(path_str) {
            Ok(d) => d,
            Err(_) => {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_DEBUG,
                    "Include extensions path failed: {} ({})\n",
                    crate::xkb::utils::CStrDisplay(path),
                    crate::xkb::utils::StrerrorDisplay(EACCES),
                );
                return ret;
            }
        };

        versioned_path_length = versioned_path_length.wrapping_add(1);

        for entry in dir_entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let name = entry.file_name();
            let name_str = match name.to_str() {
                Some(s) => s,
                None => continue,
            };
            if name_str == "." || name_str == ".." {
                continue;
            }

            // Check if subdirectory
            let entry_path = entry.path();
            let entry_meta = match std::fs::metadata(&entry_path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if !entry_meta.is_dir() {
                continue;
            }

            // Check for duplicates against versioned entries
            let mut is_dup = false;
            for i in 0..(versioned_count as usize) {
                let prev_name_ptr = (extensions[i]).add(versioned_path_length);
                let prev_name = cstr_as_bytes(prev_name_ptr);
                if name_str.as_bytes() == prev_name {
                    is_dup = true;
                    break;
                }
            }
            if is_dup {
                continue;
            }

            // Build full path and strdup it
            let full_path = match std::ffi::CString::new(entry_path.to_string_lossy().as_bytes()) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let ext_path = strdup_safe(full_path.as_ptr());
            if ext_path.is_null() {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_DEBUG,
                    "Include extensions path failed: {} ({})\n",
                    crate::xkb::utils::CStrDisplay(path),
                    crate::xkb::utils::StrerrorDisplay(ENOMEM),
                );
                return ret;
            }
            extensions.push(ext_path);
        }

        // Sort new entries
        if extensions.len() > versioned_count as usize {
            extensions[versioned_count as usize..].sort_by(|a, b| cstr_cmp(*a, *b).cmp(&0));
            for i in (versioned_count as usize)..extensions.len() {
                ret |= rxkb_context_include_path_append(ctx, extensions[i]) as i32;
            }
        }

        ret
    }
}
pub unsafe fn rxkb_context_include_path_append_default(ctx: *mut rxkb_context) -> bool {
    unsafe {
        let mut user_path: [i8; 4096] = [0; 4096];
        let mut ret: i32 = if false { 1 } else { 0 };
        if (*ctx).context_state != CONTEXT_NEW {
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
                ret |= rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        } else if !home.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                format_args!("{}/.config/xkb", crate::xkb::utils::CStrDisplay(home)),
            );
            if !_trunc {
                ret |= rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        }
        if !home.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                format_args!("{}/.xkb", crate::xkb::utils::CStrDisplay(home)),
            );
            if !_trunc {
                ret |= rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        }
        let extra: *const i8 =
            rxkb_context_getenv(ctx, b"XKB_CONFIG_EXTRA_PATH\0".as_ptr() as *const i8);
        ret |= rxkb_context_include_path_append(
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
        let mut versioned_path_length: usize = 0_usize;
        if !extensions_path.is_null() {
            ret |= add_direct_subdirectories(ctx, extensions_path, &mut extensions, 0_u32, 0_usize);
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
            ret |= add_direct_subdirectories(
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
        );
        ret |= has_root as i32;
        if !has_root && (root.is_null() || *root.offset(0_i32 as isize) as i32 != '\0' as i32) {
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
            ret |= rxkb_context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT.as_ptr()) as i32;
        }
        ret != 0
    }
}

pub unsafe fn rxkb_context_parse(ctx: *mut rxkb_context, ruleset: *const i8) -> bool {
    unsafe {
        let mut path: *mut i8;
        let mut success: bool = false;
        if (*ctx).context_state != CONTEXT_NEW {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "parse must only be called on a new context\n",
            );
            return false;
        }
        if !(*ctx).includes.is_empty() {
            let mut idx = (*ctx).includes.len();
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
        success
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
unsafe fn config_item_free(config: *mut config_item) {
    unsafe {
        free((*config).name as *mut ::core::ffi::c_void);
        free((*config).description as *mut ::core::ffi::c_void);
        free((*config).brief as *mut ::core::ffi::c_void);
        free((*config).vendor as *mut ::core::ffi::c_void);
    }
}
unsafe fn parse_config_item(
    ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    parent: xmloxide::tree::NodeId,
    config: *mut config_item,
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
        false
    }
}
unsafe fn parse_model(
    ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    model: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, doc, model, &raw mut config) {
            for &m in &(*ctx).models {
                if streq(cstr_as_bytes((*m).name), cstr_as_bytes(config.name)) {
                    config_item_free(&raw mut config);
                    return;
                }
            }
            let m = Box::into_raw(Box::new(std::mem::zeroed::<rxkb_model>()));
            rxkb_object_init(&raw mut (*m).base, &raw mut (*ctx).base);
            (*m).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*m).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*m).vendor =
                _steal(&raw mut config.vendor as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*m).popularity = config.popularity;
            (*ctx).models.push(m);
        }
    }
}
unsafe fn parse_language_list(
    doc: &xmloxide::Document,
    language_list: xmloxide::tree::NodeId,
    layout: *mut rxkb_layout,
) {
    unsafe {
        for node in doc.children(language_list) {
            if is_node(doc, node, "iso639Id") {
                let str: *mut i8 = extract_text(doc, node);
                if str.is_null() || cstr_len(str) != 3_usize {
                    free(str as *mut ::core::ffi::c_void);
                } else {
                    let parent: *mut rxkb_object = &raw mut (*layout).base;
                    let code = rxkb_iso639_code_create(parent);
                    (*code).code = str;
                    (*layout).iso639s.push(code);
                }
            }
        }
    }
}
unsafe fn parse_country_list(
    doc: &xmloxide::Document,
    country_list: xmloxide::tree::NodeId,
    layout: *mut rxkb_layout,
) {
    unsafe {
        for node in doc.children(country_list) {
            if is_node(doc, node, "iso3166Id") {
                let str: *mut i8 = extract_text(doc, node);
                if str.is_null() || cstr_len(str) != 2_usize {
                    free(str as *mut ::core::ffi::c_void);
                } else {
                    let parent: *mut rxkb_object = &raw mut (*layout).base;
                    let code = rxkb_iso3166_code_create(parent);
                    (*code).code = str;
                    (*layout).iso3166s.push(code);
                }
            }
        }
    }
}
unsafe fn parse_variant(
    ctx: *mut rxkb_context,
    l: *mut rxkb_layout,
    doc: &xmloxide::Document,
    variant: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, doc, variant, &raw mut config) {
            let mut exists: bool = false;
            for &v in &(*ctx).layouts {
                if streq_null((*v).variant, config.name)
                    && streq(cstr_as_bytes((*v).name), cstr_as_bytes((*l).name))
                {
                    exists = true;
                    break;
                }
            }
            if !exists {
                let v = rxkb_layout_create(&raw mut (*ctx).base);
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
                (*ctx).layouts.push(v);
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
                            for &x in &(*l).iso639s {
                                let code: *mut rxkb_iso639_code =
                                    rxkb_iso639_code_create(&raw mut (*v).base);
                                (*code).code = cstr_dup((*x).code);
                                (*v).iso639s.push(code);
                            }
                        }
                        if !found_country_list {
                            for &x in &(*l).iso3166s {
                                let code: *mut rxkb_iso3166_code =
                                    rxkb_iso3166_code_create(&raw mut (*v).base);
                                (*code).code = cstr_dup((*x).code);
                                (*v).iso3166s.push(code);
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
    ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    layout: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity,
            layout_specific: false,
        };
        let mut l: *mut rxkb_layout = std::ptr::null_mut();
        let mut exists: bool = false;
        if !parse_config_item(ctx, doc, layout, &raw mut config) {
            return;
        }
        for &el in &(*ctx).layouts {
            if streq(cstr_as_bytes((*el).name), cstr_as_bytes(config.name))
                && (*el).variant.is_null()
            {
                exists = true;
                l = el;
                break;
            }
        }
        if !exists {
            l = rxkb_layout_create(&raw mut (*ctx).base);
            (*l).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*l).variant = std::ptr::null_mut();
            (*l).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*l).brief =
                _steal(&raw mut config.brief as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*l).popularity = config.popularity;
            (*ctx).layouts.push(l);
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
    ctx: *mut rxkb_context,
    group: *mut rxkb_option_group,
    doc: &xmloxide::Document,
    option: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, doc, option, &raw mut config) {
            for &o in &(*group).options {
                if streq(cstr_as_bytes((*o).name), cstr_as_bytes(config.name)) {
                    config_item_free(&raw mut config);
                    return;
                }
            }
            let o = Box::into_raw(Box::new(std::mem::zeroed::<rxkb_option>()));
            rxkb_object_init(&raw mut (*o).base, &raw mut (*group).base);
            (*o).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*o).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*o).popularity = config.popularity;
            (*o).layout_specific = config.layout_specific;
            (*group).options.push(o);
        }
    }
}
unsafe fn parse_group(
    ctx: *mut rxkb_context,
    doc: &xmloxide::Document,
    group: xmloxide::tree::NodeId,
    popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: std::ptr::null_mut(),
            description: std::ptr::null_mut(),
            brief: std::ptr::null_mut(),
            vendor: std::ptr::null_mut(),
            popularity,
            layout_specific: false,
        };
        let mut g: *mut rxkb_option_group = std::ptr::null_mut();
        let mut exists: bool = false;
        if !parse_config_item(ctx, doc, group, &raw mut config) {
            return;
        }
        for &el in &(*ctx).option_groups {
            if streq(cstr_as_bytes((*el).name), cstr_as_bytes(config.name)) {
                exists = true;
                g = el;
                break;
            }
        }
        if !exists {
            g = {
                let layout = std::alloc::Layout::new::<rxkb_option_group>();
                let ptr = std::alloc::alloc_zeroed(layout) as *mut rxkb_option_group;
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                std::ptr::write(&raw mut (*ptr).options, Vec::new());
                Box::into_raw(Box::from_raw(ptr))
            };
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
            (*ctx).option_groups.push(g);
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

unsafe fn parse(ctx: *mut rxkb_context, path: *const i8, popularity: rxkb_popularity) -> bool {
    unsafe {
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
        true
    }
}
use crate::xkb::shared_types::*;
