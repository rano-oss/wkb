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
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
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
        pub text_next: size_t,
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
    use super::__stddef_size_t_h::size_t;
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
    extern "C" {
        pub fn xkb_components_names_from_rules(
            context: *mut xkb_context,
            rmlvo_in: *const xkb_rule_names,
            rmlvo_out: *mut xkb_rule_names,
            components_out: *mut xkb_component_names,
        ) -> bool;
        pub fn xkb_context_unref(context: *mut xkb_context);
    }
}
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    use super::context_h::xkb_context;
    extern "C" {
        pub fn test_init();
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
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
                    __ASSERT_FUNCTION.as_ptr(),
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
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::stdbool_h::false_0;
    use super::string_h::strcmp;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
    pub const NULL_0: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod assert_h {
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 40] = unsafe {
        ::core::mem::transmute::<[u8; 40], [::core::ffi::c_char; 40]>(
            *b"_Bool streq(const char *, const char *)\0",
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
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn setenv(
            __name: *const ::core::ffi::c_char,
            __value: *const ::core::ffi::c_char,
            __replace: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub mod string_h {
    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod test_config_h {
    pub const TEST_XKB_CONFIG_ROOT: [::core::ffi::c_char; 41] = unsafe {
        ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
            *b"/home/rano/Public/libxkbcommon/test/data\0",
        )
    };
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::{NULL, NULL_0};
pub use self::__stddef_size_t_h::size_t;
pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::{free, setenv};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_config_h::TEST_XKB_CONFIG_ROOT;
pub use self::test_h::{
    test_context_flags, test_get_context, test_init, CONTEXT_ALLOW_ENVIRONMENT_NAMES,
    CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::utils_h::{streq, streq_not_null, streq_null};
pub use self::xkbcommon_h::{
    xkb_component_names, xkb_components_names_from_rules, xkb_context_unref, xkb_log_level,
    xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
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
    pub should_fail: bool,
}
unsafe extern "C" fn test_rules(mut ctx: *mut xkb_context, mut data: *mut test_data) -> bool {
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
                b"Expecting: %s\t%s\t%s\t%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*data).keycodes,
                (*data).types,
                (*data).compat,
                (*data).symbols,
            );
        }
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
        if xkb_components_names_from_rules(
            ctx,
            &raw const rmlvo,
            ::core::ptr::null_mut::<xkb_rule_names>(),
            &raw mut kccgst,
        ) {
            fprintf(
                stderr,
                b"Received : %s\t%s\t%s\t%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                kccgst.keycodes,
                kccgst.types,
                kccgst.compatibility,
                kccgst.symbols,
            );
        } else {
            fprintf(
                stderr,
                b"Received : FAILURE\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return (*data).should_fail;
        }
        let passed: bool = streq_not_null(kccgst.keycodes, (*data).keycodes) as ::core::ffi::c_int
            != 0
            && streq_not_null(kccgst.types, (*data).types) as ::core::ffi::c_int != 0
            && streq_not_null(kccgst.compatibility, (*data).compat) as ::core::ffi::c_int != 0
            && streq_not_null(kccgst.symbols, (*data).symbols) as ::core::ffi::c_int != 0
            && streq_null(kccgst.geometry, (*data).geometry) as ::core::ffi::c_int != 0;
        free(kccgst.keycodes as *mut ::core::ffi::c_void);
        free(kccgst.types as *mut ::core::ffi::c_void);
        free(kccgst.compatibility as *mut ::core::ffi::c_void);
        free(kccgst.symbols as *mut ::core::ffi::c_void);
        free(kccgst.geometry as *mut ::core::ffi::c_void);
        return passed;
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
        test_init();
        setenv(
            b"XKB_CONFIG_ROOT\0".as_ptr() as *const ::core::ffi::c_char,
            TEST_XKB_CONFIG_ROOT.as_ptr(),
            1 as ::core::ffi::c_int,
        );
        ctx = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test1: test_data = test_data {
            rules: b"inc-src-simple\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
            symbols: b"my_symbols\0".as_ptr() as *const ::core::ffi::c_char,
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: false,
        };
        if test_rules(ctx, &raw mut test1) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test1)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test2: test_data = test_data {
            rules: b"inc-src-nested\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
            symbols: b"my_symbols\0".as_ptr() as *const ::core::ffi::c_char,
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: false,
        };
        if test_rules(ctx, &raw mut test2) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test2)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                106 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test3: test_data = test_data {
            rules: b"inc-src-looped\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: ::core::ptr::null::<::core::ffi::c_char>(),
            types: ::core::ptr::null::<::core::ffi::c_char>(),
            compat: ::core::ptr::null::<::core::ffi::c_char>(),
            symbols: ::core::ptr::null::<::core::ffi::c_char>(),
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: true_0 != 0,
        };
        if test_rules(ctx, &raw mut test3) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test3)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                115 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test4: test_data = test_data {
            rules: b"inc-src-before-after\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"before_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
            symbols: b"default_symbols\0".as_ptr() as *const ::core::ffi::c_char,
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: false,
        };
        if test_rules(ctx, &raw mut test4) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test4)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test5: test_data = test_data {
            rules: b"inc-src-options\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"my_variant\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"option11,my_option,colon:opt,option111\0".as_ptr()
                as *const ::core::ffi::c_char,
            keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"default_compat+substring+group(bla)|some:compat\0".as_ptr()
                as *const ::core::ffi::c_char,
            symbols: b"my_symbols+extra_variant+altwin(menu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: false,
        };
        if test_rules(ctx, &raw mut test5) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test5)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test6: test_data = test_data {
            rules: b"inc-src-loop-twice\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
            symbols: b"my_symbols\0".as_ptr() as *const ::core::ffi::c_char,
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: false,
        };
        if test_rules(ctx, &raw mut test6) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test6)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test7: test_data = test_data {
            rules: b"inc-no-newline\0".as_ptr() as *const ::core::ffi::c_char,
            model: ::core::ptr::null::<::core::ffi::c_char>(),
            layout: ::core::ptr::null::<::core::ffi::c_char>(),
            variant: ::core::ptr::null::<::core::ffi::c_char>(),
            options: ::core::ptr::null::<::core::ffi::c_char>(),
            keycodes: ::core::ptr::null::<::core::ffi::c_char>(),
            types: ::core::ptr::null::<::core::ffi::c_char>(),
            compat: ::core::ptr::null::<::core::ffi::c_char>(),
            symbols: ::core::ptr::null::<::core::ffi::c_char>(),
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: true_0 != 0,
        };
        if test_rules(ctx, &raw mut test7) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test7)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut test8: test_data = test_data {
            rules: b"inc-src-relative-path\0".as_ptr() as *const ::core::ffi::c_char,
            model: b"my_model\0".as_ptr() as *const ::core::ffi::c_char,
            layout: b"my_layout\0".as_ptr() as *const ::core::ffi::c_char,
            variant: b"\0".as_ptr() as *const ::core::ffi::c_char,
            options: b"\0".as_ptr() as *const ::core::ffi::c_char,
            keycodes: b"my_keycodes\0".as_ptr() as *const ::core::ffi::c_char,
            types: b"default_types\0".as_ptr() as *const ::core::ffi::c_char,
            compat: b"default_compat\0".as_ptr() as *const ::core::ffi::c_char,
            symbols: b"my_symbols\0".as_ptr() as *const ::core::ffi::c_char,
            geometry: ::core::ptr::null::<::core::ffi::c_char>(),
            should_fail: false,
        };
        if test_rules(ctx, &raw mut test8) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"test_rules(ctx, &test8)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/rules-file-includes.c\0".as_ptr() as *const ::core::ffi::c_char,
                163 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_context_unref(ctx);
        return 0 as ::core::ffi::c_int;
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
