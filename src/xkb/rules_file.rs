pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod types_h {
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
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
pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_context {
        pub refcnt: ::core::ffi::c_int,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: ::core::ffi::c_int,
        pub user_data: *mut ::core::ffi::c_void,
        pub names_dflt: xkb_rule_names,
        pub includes: C2Rust_Unnamed_0,
        pub failed_includes: C2Rust_Unnamed,
        pub atom_table: *mut atom_table,
        pub x11_atom_cache: *mut ::core::ffi::c_void,
        pub text_buffer: [::core::ffi::c_char; 2048],
        pub text_next: usize,
        #[bitfield(name = "use_environment_names", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "use_secure_getenv", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "pending_default_includes", ty = "bool", bits = "2..=2")]
        pub use_environment_names_use_secure_getenv_pending_default_includes: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }

    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
}
pub mod atom_h {
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
}
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const ::core::ffi::c_char,
        pub model: *const ::core::ffi::c_char,
        pub layout: *const ::core::ffi::c_char,
        pub variant: *const ::core::ffi::c_char,
        pub options: *const ::core::ffi::c_char,
    }
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_layout_index_t = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_component_names {
        pub keycodes: *mut ::core::ffi::c_char,
        pub compatibility: *mut ::core::ffi::c_char,
        pub geometry: *mut ::core::ffi::c_char,
        pub symbols: *mut ::core::ffi::c_char,
        pub types: *mut ::core::ffi::c_char,
    }
    use super::context_h::xkb_context;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_components_names_from_rules(
            context: *mut xkb_context,
            rmlvo_in: *const xkb_rule_names,
            rmlvo_out: *mut xkb_rule_names,
            components_out: *mut xkb_component_names,
        ) -> bool;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_context_include_path_append(
            context: *mut xkb_context,
            path: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    use super::context_h::xkb_context;
    extern "C" {
        pub fn test_init();
        pub fn test_get_path(path_rel: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: usize,
        ) -> ::core::ffi::c_int;
    }
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
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn streq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if !s1.is_null() && !s2.is_null() {
            } else {
                __assert_fail(
                    b"s1 && s2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../src/utils.h\0".as_ptr() as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_uint,
                    b"_Bool streq(const char *, const char *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return s1 == s2;
            }
            return streq(s1, s2);
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_not_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return false_0 != 0;
            }
            return streq(s1, s2);
        }
    }
    use super::assert_h::__assert_fail;
    use super::stdbool_h::false_0;
    use super::string_h::strcmp;
}
pub mod rules_h {
    use super::context_h::xkb_context;
    use super::xkbcommon_h::{xkb_component_names, xkb_layout_index_t, xkb_rule_names};
    extern "C" {
        pub fn xkb_components_from_rules_names(
            ctx: *mut xkb_context,
            rmlvo: *const xkb_rule_names,
            out: *mut xkb_component_names,
            explicit_layouts: *mut xkb_layout_index_t,
        ) -> bool;
    }
}
pub mod stdlib_h {
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
pub mod keymap_h {
    pub const XKB_MAX_GROUPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::keymap_h::XKB_MAX_GROUPS;
use self::rules_h::xkb_components_from_rules_names;
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_uintn_h::u32;
use self::stdio_h::{fprintf, snprintf, stderr};
pub use self::stdlib_h::{free, EXIT_SUCCESS};
use self::string_h::{memcpy, strncmp};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    test_context_flags, test_get_context, test_get_path, test_init,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint32_t, __uint64_t};
pub use self::utils_h::{streq, streq_not_null, streq_null};
pub use self::xkbcommon_h::{
    xkb_component_names, xkb_components_names_from_rules, xkb_context_include_path_append,
    xkb_context_unref, xkb_layout_index_t, xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_data {
    pub rules: *const ::core::ffi::c_char,
    pub model: *const ::core::ffi::c_char,
    pub layout: *const ::core::ffi::c_char,
    pub variant: *const ::core::ffi::c_char,
    pub options: *const ::core::ffi::c_char,
    pub keycodes: *const ::core::ffi::c_char,
    pub types: *const ::core::ffi::c_char,
    pub compat: *const ::core::ffi::c_char,
    pub symbols: *const ::core::ffi::c_char,
    pub geometry: *const ::core::ffi::c_char,
    pub explicit_layouts: xkb_layout_index_t,
    pub should_fail: bool,
}
unsafe extern "C" fn test_rules(mut ctx: *mut xkb_context, mut data: *const test_data) -> bool {
    unsafe {
        fprintf(
            stderr,
            b"\n\nChecking : %s\t%s\t%s\t%s\t%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*data).rules,
            (*data).model,
            (*data).layout,
            (*data).variant,
            (*data).options,
        );
        if (*data).should_fail {
            fprintf(
                stderr,
                b"Expecting: FAILURE\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"Expecting: %s\t%s\t%s\t%s\t%s\t%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*data).keycodes,
                (*data).types,
                (*data).compat,
                (*data).symbols,
                (*data).geometry,
                (*data).explicit_layouts,
            );
        }
        let mut passed: ::core::ffi::c_int = if true_0 != 0 { 1 } else { 0 };
        let mut explicit_layouts: xkb_layout_index_t = 0 as xkb_layout_index_t;
        let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while k < 2 as ::core::ffi::c_int {
            let mut ok: bool = false;
            let rmlvo: xkb_rule_names = xkb_rule_names {
                rules: (*data).rules,
                model: (*data).model,
                layout: (*data).layout,
                variant: (*data).variant,
                options: (*data).options,
            };
            let mut kccgst: xkb_component_names = xkb_component_names {
                keycodes: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                compatibility: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                geometry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                symbols: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                types: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            };
            if k == 0 as ::core::ffi::c_int {
                ok = xkb_components_from_rules_names(
                    ctx,
                    &raw const rmlvo,
                    &raw mut kccgst,
                    &raw mut explicit_layouts,
                );
            } else {
                ok = xkb_components_names_from_rules(
                    ctx,
                    &raw const rmlvo,
                    ::core::ptr::null_mut::<xkb_rule_names>(),
                    &raw mut kccgst,
                );
            }
            if ok {
                fprintf(
                    stderr,
                    b"Received : %s\t%s\t%s\t%s\t%s\t%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                    kccgst.keycodes,
                    kccgst.types,
                    kccgst.compatibility,
                    kccgst.symbols,
                    kccgst.geometry,
                    explicit_layouts,
                );
            } else {
                fprintf(
                    stderr,
                    b"Received : FAILURE\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return (*data).should_fail;
            }
            passed = passed as ::core::ffi::c_int
                & (streq_not_null(kccgst.keycodes, (*data).keycodes) as ::core::ffi::c_int != 0
                    && streq_not_null(kccgst.types, (*data).types) as ::core::ffi::c_int != 0
                    && streq_not_null(kccgst.compatibility, (*data).compat) as ::core::ffi::c_int
                        != 0
                    && streq_not_null(kccgst.symbols, (*data).symbols) as ::core::ffi::c_int != 0
                    && streq_null(kccgst.geometry, (*data).geometry) as ::core::ffi::c_int != 0
                    && explicit_layouts == (*data).explicit_layouts)
                    as ::core::ffi::c_int;
            free(kccgst.keycodes as *mut ::core::ffi::c_void);
            free(kccgst.types as *mut ::core::ffi::c_void);
            free(kccgst.compatibility as *mut ::core::ffi::c_void);
            free(kccgst.symbols as *mut ::core::ffi::c_void);
            free(kccgst.geometry as *mut ::core::ffi::c_void);
            k += 1;
        }
        return passed != 0;
    }
}
unsafe extern "C" fn test_encodings(mut ctx: *mut xkb_context) {
    unsafe {
        static mut tests: [test_data; 4] = [
            test_data {
                rules: b"utf-8_with_bom\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat|some:compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"my_symbols+extra_variant\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"utf-16le_with_bom\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat|some:compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"my_symbols+extra_variant\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true_0 != 0,
            },
            test_data {
                rules: b"utf-16be_with_bom\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat|some:compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"my_symbols+extra_variant\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true_0 != 0,
            },
            test_data {
                rules: b"utf-32be\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat|some:compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"my_symbols+extra_variant\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true_0 != 0,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_encodings\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_uint,
                    b"void test_encodings(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_strict_decimal_groups(mut ctx: *mut xkb_context) {
    unsafe {
        static mut tests: [test_data; 2] = [
            test_data {
                rules: b"invalid-group-index\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"1,2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"default_symbols+default_symbols:2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: true_0 != 0,
            },
            test_data {
                rules: b"invalid-group-qualifier\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"1,2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"default_symbols+default_symbols:+2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_strict_decimal_groups\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_uint,
                    b"void test_strict_decimal_groups(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_simple(mut ctx: *mut xkb_context) {
    unsafe {
        static mut tests: [test_data; 8] = [
            test_data {
                rules: b"simple\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat|some:compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"my_symbols+extra_variant\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"simple\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"foo\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"default_symbols\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"groups\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"foo\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"something(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"default_symbols\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"groups\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"foo\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ar\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"bar\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"my_symbols+(bar)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"simple\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"my_layout,second_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: ::core::ptr::null::<::core::ffi::c_char>(),
                types: ::core::ptr::null::<::core::ffi::c_char>(),
                compat: ::core::ptr::null::<::core::ffi::c_char>(),
                symbols: ::core::ptr::null::<::core::ffi::c_char>(),
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 0,
                should_fail: true_0 != 0,
            },
            test_data {
                rules: b"index\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"br,al,cn,az\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"some:opt\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"default_symbols+extra:1+extra:2+extra:3+extra:4\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 4 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"multiple-options\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"option3,option1,colon:opt,option11\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat+some:compat+group(bla)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                symbols: b"my_symbols+extra_variant+compose(foo)+keypad(bar)+altwin(menu)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"merge-mode-replace\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"us,de\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"replace:first\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+us+de:2^level3(ralt_alt)|empty\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_simple\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    281 as ::core::ffi::c_uint,
                    b"void test_simple(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_wild_card(mut ctx: *mut xkb_context) {
    unsafe {
        static mut tests: [test_data; 16] = [
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+a(1)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+a(1)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"a,\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1,\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+a(1)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b",b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",2\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"+b(2):2\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1,\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+a(1)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",2\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"+b(2):2\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: ::core::ptr::null::<::core::ffi::c_char>(),
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: ::core::ptr::null::<::core::ffi::c_char>(),
                variant: b"1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b",\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1,2\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: true,
            },
            test_data {
                rules: b"wildcard\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: true,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_wild_card\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    319 as ::core::ffi::c_uint,
                    b"void test_wild_card(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_extended_wilcards(mut ctx: *mut xkb_context) {
    unsafe {
        static mut tests: [test_data; 15] = [
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l10:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l20:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v2\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l30(v2):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l2:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l40(v1):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v2\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l40(v2):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l3\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l50:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l3\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l50(v1):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l3\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v2\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l50(v2):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l4\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l4:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l4\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l4(v1):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l4\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v2\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l4(v20):1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l1,l1,l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",v1,v2,\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l10:1+l20:2+l30(v2):3+l2:4\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 4 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l2,l2,l3,l3\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v1,v2,,v1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l40(v1):1+l40(v2):2+l50:3+l50(v1):4\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 4 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"extended-wild-cards\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"l3,l4,l4,l4\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"v2,,v1,v2\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l50(v2):1+l4:2+l4(v1):3+l4(v20):4\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 4 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 15]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_extended_wilcards\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    361 as ::core::ffi::c_uint,
                    b"void test_extended_wilcards(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_layout_index_ranges(
    mut ctx: *mut xkb_context,
    mut too_much_layouts: *const ::core::ffi::c_char,
    mut too_much_symbols: *const ::core::ffi::c_char,
) {
    unsafe {
        let tests: [test_data; 41] = [
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"A\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_e\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"E+layout_e\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"1\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a(1)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_c\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"C:1+z:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_d\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"D\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a+b:2\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"a,b\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",c\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a+b(c):2\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_e,layout_a\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"e:1+x:2\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_a,layout_b,layout_c,layout_d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a:1+y:2+layout_c:3+layout_d:4+z:3\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 4 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_a,layout_b,layout_c,layout_d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: b"extra,,,extra\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a:1+y:2+layout_c:3+layout_d(extra):4+z:3+foo:1|bar:1+foo:4|bar:4\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 4 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_a,layout_b,layout_c,layout_d,layout_e\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a:1+y:2+layout_c:3+layout_d:4+layout_e:5+z:3\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 5 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: b"layout_a,layout_b,layout_c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"option_3,option_2,option_1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"a:1+y:2+layout_c:3+z:3+III:2+JJJ:2+HHH:3+KKK:3+LLL+OOO:2+MMM:3+NNN:3\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"special_indices\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 3 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"special_indices-limit\0".as_ptr() as *const ::core::ffi::c_char,
                model: ::core::ptr::null::<::core::ffi::c_char>(),
                layout: too_much_layouts,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: too_much_symbols,
                geometry: if strncmp(
                    b"special_indices-limit\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 32 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ar\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+ara+inet(evdev)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ben\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"probhat\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+in(ben_probhat)+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"ataritt\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"xfree68_vndr/ataritt(us)+es+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"ataritt\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete+japan\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"xfree68_vndr/ataritt(us)+jp+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"olpc\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"us\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+olpc(olpc)+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"olpc\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"olpc+us(olpc)+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"olpc\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+olpc(olpc)+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete+japan\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"olpc+jp+inet(evdev)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete+japan\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+jp+inet(evdev)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"xxx\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete+japan\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+jp(xxx)+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+es+inet(evdev)\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"xxx\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+es(xxx)+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"de\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"neo\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwertz)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete+caps(caps_lock):1+misc(assign_shift_left_action):1+level5(level5_lock):1\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+de(neo)+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"br\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"misc:typo,misc:apl\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+br+inet(evdev)+apl(level3):1+typo(base):1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ar,pt\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+ara+pt:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"pt,ar\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+pt+ara:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ben,gb\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"probhat,\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+in(ben_probhat)+gb:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"gb,ben\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",probhat\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+gb+in(ben):2+in(ben_probhat):2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"whatever\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ben,ar\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"probhat,\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+in(ben_probhat)+ara:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"ataritt\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp,es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+jp+es:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"ataritt\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"es,jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+es+jp:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"olpc\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp,es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+olpc(olpc)+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+jp+es:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"olpc\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"es,jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+olpc(olpc)+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+es+jp:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp,es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+jp+es:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"jp,es\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"xxx,yyy\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+jp(xxx)+es(yyy):2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"latin,jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+latin+jp:2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"latin,jp\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b"xxx,yyy\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+latin(xxx)+jp(yyy):2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"gb,de\0".as_ptr() as *const ::core::ffi::c_char,
                variant: b",neo\0".as_ptr() as *const ::core::ffi::c_char,
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete+caps(caps_lock):2+misc(assign_shift_left_action):2+level5(level5_lock):2\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+gb+de(neo):2+inet(evdev)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"ca,br\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"misc:typo,misc:apl\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev+aliases(qwerty)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+ca+br:2+inet(evdev)+apl(level3):1+apl(level3):2+typo(base):1+typo(base):2\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: if strncmp(
                    b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
                    b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as usize,
                ) == 0 as ::core::ffi::c_int
                {
                    b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null::<::core::ffi::c_char>()
                },
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 41]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_layout_index_ranges\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    465 as ::core::ffi::c_uint,
                    b"void test_layout_index_ranges(struct xkb_context *, const char *, const char *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_extended_layout_indices(mut ctx: *mut xkb_context) {
    unsafe {
        let mut layouts: [::core::ffi::c_char; 416] = [
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
        let mut count: usize = 0 as usize;
        let mut l: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while l < XKB_MAX_GROUPS as xkb_layout_index_t {
            count = count.wrapping_add(snprintf(
                (&raw mut layouts as *mut ::core::ffi::c_char).offset(count as isize)
                    as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 416]>() as usize).wrapping_sub(count),
                b"x%u,\0".as_ptr() as *const ::core::ffi::c_char,
                l.wrapping_add(1 as xkb_layout_index_t),
            ) as usize);
            l = l.wrapping_add(1);
        }
        count = count.wrapping_sub(1);
        layouts[count as usize] = '\0' as i32 as ::core::ffi::c_char;
        static mut symbols_prefix: [::core::ffi::c_char; 7] =
            unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"pc+x1+\0") };
        static mut symbols_suffix: [::core::ffi::c_char; 13] = unsafe {
            ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"+inet(evdev)\0")
        };
        let mut symbols: [::core::ffi::c_char; 820] = [
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
        memcpy(
            &raw mut symbols as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw const symbols_prefix as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize,
        );
        count = (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
            .wrapping_sub(1 as usize) as usize;
        let mut l_0: xkb_layout_index_t = 1 as xkb_layout_index_t;
        while l_0 < XKB_MAX_GROUPS as xkb_layout_index_t {
            count = count.wrapping_add(snprintf(
                (&raw mut symbols as *mut ::core::ffi::c_char).offset(count as isize)
                    as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 820]>() as usize).wrapping_sub(count),
                b"x%u:%u+\0".as_ptr() as *const ::core::ffi::c_char,
                l_0.wrapping_add(1 as xkb_layout_index_t),
                l_0.wrapping_add(1 as xkb_layout_index_t),
            ) as usize);
            l_0 = l_0.wrapping_add(1);
        }
        memcpy(
            (&raw mut symbols as *mut ::core::ffi::c_char)
                .offset(count as isize)
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_void,
            &raw const symbols_suffix as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 13]>() as usize,
        );
        let test: test_data = test_data {
            rules: b"evdev-modern\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
            layout: &raw mut layouts as *mut ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: b"evdev+aliases(qwerty)\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
            symbols: &raw mut symbols as *mut ::core::ffi::c_char,
            geometry: b"pc(pc104)\0".as_ptr() as *const ::core::ffi::c_char,
            explicit_layouts: XKB_MAX_GROUPS as xkb_layout_index_t,
            should_fail: false,
        };
        if test_rules(ctx, &raw const test) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                508 as ::core::ffi::c_uint,
                b"void test_extended_layout_indices(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe extern "C" fn test_all_qualifier(
    mut ctx: *mut xkb_context,
    mut too_much_layouts: *const ::core::ffi::c_char,
    mut too_much_symbols: *const ::core::ffi::c_char,
) {
    unsafe {
        let tests: [test_data; 5] = [
            test_data {
                rules: b"all_qualifier\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"layout_a,layout_b,layout_a,layout_b,layout_c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"symbols_a:1+symbols_b:2+symbols_a:3+symbols_b:4+symbols_c:5+extra_option:1+extra_option:2+extra_option:3+extra_option:4+extra_option:5\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 5 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"all_qualifier\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"layout_x,layout_a,layout_b,layout_c,layout_d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"base:1+base:2+base:3+base:4+base:5+symbols_a:2+symbols_b:3+default_symbols:4+default_symbols:5\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 5 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"all_qualifier-limit\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: too_much_layouts,
                variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"default_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: too_much_symbols,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: XKB_MAX_GROUPS as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"all_qualifier\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"layout_a,layout_b,layout_a,layout_b,layout_c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: b"extra1,,,,\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"symbols_a:1+symbols_b:2+symbols_a:3+symbols_b:4+symbols_c:5+extra_symbols:1+extra_symbols:2+extra_symbols:3+extra_symbols:4+extra_symbols:5+extra_option:1+extra_option:2+extra_option:3+extra_option:4+extra_option:5\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 5 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"all_qualifier\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"layout_a,layout_b,layout_a,layout_b,layout_c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                variant: b"extra2,,extra3,,\0".as_ptr() as *const ::core::ffi::c_char,
                options: b"my_option\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"my_types\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"my_compat\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"symbols_a:1+symbols_b:2+symbols_a:3+symbols_b:4+symbols_c:5+extra_symbols1:1+extra_symbols2:1+extra_symbols2:2+extra_symbols2:3+extra_symbols2:4+extra_symbols2:5+extra_symbols2:1+extra_symbols2:2+extra_symbols2:3+extra_symbols2:4+extra_symbols2:5+extra_symbols1:3+extra_option:1+extra_option:2+extra_option:3+extra_option:4+extra_option:5\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 5 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_all_qualifier\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    602 as ::core::ffi::c_uint,
                    b"void test_all_qualifier(struct xkb_context *, const char *, const char *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_layout_specific_options(mut ctx: *mut xkb_context) {
    unsafe {
        let tests: [test_data; 12] = [
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1,opt2,opt3,opt4,opt5,opt6,opt7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+s1:1+s3:1+s7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1,opt2,opt3,opt4,opt5,opt6,opt7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l2:1+s1:1+s2:1+s3:1+s4:1+s7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!,opt2!1x,opt3!x,opt4!x1,opt5!!,opt6!+,opt7!|\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+s1:1+s3:1+s7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!1,opt2!1,opt3!1,opt4!1,opt5!1,opt6!1,opt7!1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+s1:1+s3:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!1,opt2!1,opt3!1,opt4!1,opt5!1,opt6!1,opt7!1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l2:1+s1:1+s2:1+s3:1+s4:1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!2,opt2!2,opt3!2,opt4!2,opt5!2,opt6!2,opt7!2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!2,opt2!2,opt3!2,opt4!2,opt5!2,opt6!2,opt7!2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l2:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1,opt2,opt3,opt4,opt5,opt6,opt7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2+s1:1+s3:1+s3:2+s4:2+s5:1+s6:2+s7\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!3,opt2!3,opt3!3,opt4!3,opt5!3,opt6!3,opt7!3\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!1,opt2!1,opt3!1,opt4!1,opt5!1,opt6!1,opt7!1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2+s1:1+s3:1+s5:1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!2,opt2!2,opt3!2,opt4!2,opt5!2,opt6!2,opt7!2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2+s3:2+s4:2+s6:2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"layout-specific-options\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt1!1,opt1!2,opt2!1,opt2!2,opt3!1,opt3!2,opt4!1,opt4!2,opt5!1,opt5!2,opt6!1,opt6!2,opt7!1,opt7!2\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2+s1:1+s3:1+s3:2+s4:2+s5:1+s6:2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 12]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_layout_specific_options\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    732 as ::core::ffi::c_uint,
                    b"void test_layout_specific_options(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_partial_rules(mut ctx: *mut xkb_context) {
    unsafe {
        let tests_1: [test_data; 4] = [
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"m1\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"m1\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"m1+l1:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt:a!1\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2+extra:2+a:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"m1\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt:a!1\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"m1\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"m1+l1:1+l2:2+extra:2+a:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[test_data; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u (1 include path) ***\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"test_partial_rules\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            if test_rules(
                ctx,
                (&raw const tests_1 as *const test_data).offset(k as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests_1[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    783 as ::core::ffi::c_uint,
                    b"void test_partial_rules(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k = k.wrapping_add(1);
        }
        ctx = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                792 as ::core::ffi::c_uint,
                b"void test_partial_rules(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut extra: *mut ::core::ffi::c_char =
            test_get_path(b"extra\0".as_ptr() as *const ::core::ffi::c_char);
        if !extra.is_null() {
        } else {
            __assert_fail(
                b"extra\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                794 as ::core::ffi::c_uint,
                b"void test_partial_rules(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        xkb_context_include_path_append(ctx, extra);
        free(extra as *mut ::core::ffi::c_void);
        let tests_2: [test_data; 4] = [
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"m1\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: ::core::ptr::null::<::core::ffi::c_char>(),
                keycodes: b"m1+m1-ter\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"m1+l1:1\0".as_ptr() as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 1 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt:a!1\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"pc+l1:1+l2:2+extra:2+a:1+extra-bis:2+a-bis:1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
            test_data {
                rules: b"partial\0".as_ptr() as *const ::core::ffi::c_char,
                model: b"m1\0".as_ptr() as *const ::core::ffi::c_char,
                layout: b"l1,l2\0".as_ptr() as *const ::core::ffi::c_char,
                variant: ::core::ptr::null::<::core::ffi::c_char>(),
                options: b"opt:a!1\0".as_ptr() as *const ::core::ffi::c_char,
                keycodes: b"m1+m1-ter\0".as_ptr() as *const ::core::ffi::c_char,
                types: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                compat: b"complete\0".as_ptr() as *const ::core::ffi::c_char,
                symbols: b"m1+l1:1+l2:2+extra:2+a:1+extra-bis:2+a-bis:1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                geometry: ::core::ptr::null::<::core::ffi::c_char>(),
                explicit_layouts: 2 as xkb_layout_index_t,
                should_fail: false,
            },
        ];
        let mut k_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k_0 as usize)
            < (::core::mem::size_of::<[test_data; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u (2 include paths) ***\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"test_partial_rules\0".as_ptr() as *const ::core::ffi::c_char,
                k_0,
            );
            if test_rules(
                ctx,
                (&raw const tests_2 as *const test_data).offset(k_0 as isize) as *const test_data,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_rules(ctx, &tests_2[k])\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                    838 as ::core::ffi::c_uint,
                    b"void test_partial_rules(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            k_0 = k_0.wrapping_add(1);
        }
        xkb_context_unref(ctx);
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
        test_init();
        ctx = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file.c\0".as_ptr() as *const ::core::ffi::c_char,
                851 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        test_encodings(ctx);
        test_strict_decimal_groups(ctx);
        test_simple(ctx);
        test_wild_card(ctx);
        test_extended_wilcards(ctx);
        let mut too_much_layouts: [::core::ffi::c_char; 429] = [
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut too_much_symbols: [::core::ffi::c_char; 448] = [
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
        let mut i: usize = 0 as usize;
        let mut l: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while l <= XKB_MAX_GROUPS as xkb_layout_index_t {
            i = i.wrapping_add(snprintf(
                (&raw mut too_much_layouts as *mut ::core::ffi::c_char).offset(i as isize)
                    as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 429]>() as usize).wrapping_sub(i),
                b"x%u,\0".as_ptr() as *const ::core::ffi::c_char,
                l.wrapping_add(1 as xkb_layout_index_t),
            ) as usize);
            l = l.wrapping_add(1);
        }
        i = i.wrapping_sub(1);
        too_much_layouts[i as usize] = '\0' as i32 as ::core::ffi::c_char;
        i = 0 as usize;
        let mut l_0: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while l_0 < XKB_MAX_GROUPS as xkb_layout_index_t {
            i = i.wrapping_add(snprintf(
                (&raw mut too_much_symbols as *mut ::core::ffi::c_char).offset(i as isize)
                    as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 448]>() as usize).wrapping_sub(i),
                b"x:%u+\0".as_ptr() as *const ::core::ffi::c_char,
                l_0.wrapping_add(1 as xkb_layout_index_t),
            ) as usize);
            l_0 = l_0.wrapping_add(1);
        }
        i = i.wrapping_sub(1);
        too_much_symbols[i as usize] = '\0' as i32 as ::core::ffi::c_char;
        test_layout_index_ranges(
            ctx,
            &raw mut too_much_layouts as *mut ::core::ffi::c_char,
            &raw mut too_much_symbols as *mut ::core::ffi::c_char,
        );
        test_extended_layout_indices(ctx);
        test_all_qualifier(
            ctx,
            &raw mut too_much_layouts as *mut ::core::ffi::c_char,
            &raw mut too_much_symbols as *mut ::core::ffi::c_char,
        );
        test_layout_specific_options(ctx);
        test_partial_rules(ctx);
        xkb_context_unref(ctx);
        return EXIT_SUCCESS;
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
