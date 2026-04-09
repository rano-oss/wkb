pub mod types_h {
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
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
    pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    extern "C" {
        pub type xkb_context;
        pub type xkb_keymap;
        pub fn xkb_context_unref(context: *mut xkb_context);
    }
}
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    pub type test_compile_buffer_t = Option<
        unsafe extern "C" fn(
            *mut xkb_context,
            xkb_keymap_format,
            *const ::core::ffi::c_char,
            usize,
            *mut ::core::ffi::c_void,
        ) -> *mut xkb_keymap,
    >;

    use super::xkbcommon_h::{
        xkb_context, xkb_keymap, xkb_keymap_compile_flags, xkb_keymap_format,
    };
    extern "C" {
        pub fn test_init();
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_buffer2(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
            buf: *const ::core::ffi::c_char,
            len: usize,
        ) -> *mut xkb_keymap;
        pub fn test_compile_output(
            ctx: *mut xkb_context,
            input_format: xkb_keymap_format,
            output_format: xkb_keymap_format,
            compile_buffer_0: test_compile_buffer_t,
            compile_buffer_private: *mut ::core::ffi::c_void,
            test_title: *const ::core::ffi::c_char,
            keymap_str: *const ::core::ffi::c_char,
            keymap_len: usize,
            rel_path: *const ::core::ffi::c_char,
            update_output_files: bool,
        ) -> bool;
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
pub mod string_h {

    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> usize;
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
    use super::assert_h::__assert_fail;
    use super::string_h::strcmp;
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
pub mod stdlib_h {
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

use self::assert_h::__assert_fail;
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::{fprintf, stderr};
pub use self::stdlib_h::{exit, EXIT_FAILURE, EXIT_SUCCESS};
use self::string_h::strlen;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    test_compile_buffer2, test_compile_buffer_t, test_compile_output, test_context_flags,
    test_get_context, test_init, CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::utils_h::streq;
pub use self::xkbcommon_h::{
    xkb_context, xkb_context_unref, xkb_keymap, xkb_keymap_compile_flags, xkb_keymap_format,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keymap_test_data {
    pub keymap: *const ::core::ffi::c_char,
    pub expected: *const ::core::ffi::c_char,
    pub skip: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub name: *const ::core::ffi::c_char,
    pub flags: xkb_keymap_compile_flags,
    pub fail: bool,
}
static mut modes: [C2Rust_Unnamed; 2] = [
    C2Rust_Unnamed {
        name: b"strict\0".as_ptr() as *const ::core::ffi::c_char,
        flags: (XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int
            | XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int)
            as xkb_keymap_compile_flags,
        fail: true_0 != 0,
    },
    C2Rust_Unnamed {
        name: b"lax\0".as_ptr() as *const ::core::ffi::c_char,
        flags: (XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int
            & !(XKB_KEYMAP_COMPILE_STRICT_MODE as ::core::ffi::c_int))
            as xkb_keymap_compile_flags,
        fail: false_0 != 0,
    },
];
unsafe extern "C" fn compile_buffer(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut buf: *const ::core::ffi::c_char,
    mut len: usize,
    mut private: *mut ::core::ffi::c_void,
) -> *mut xkb_keymap {
    unsafe {
        return test_compile_buffer2(
            context,
            format,
            *(private as *mut xkb_keymap_compile_flags),
            buf,
            len,
        );
    }
}
unsafe extern "C" fn test_unknown_declaration_statements(
    mut ctx: *mut xkb_context,
    mut update_files: bool,
) {
    unsafe {
        static mut tests: [keymap_test_data; 16] = [
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx \"1\" = \"xxx\"; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx 1 = yyy; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx 1.23 = Group1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx <> = 1 + 1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx \"1\" = \"xxx\"; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx 1 = yyy; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx 1.23 = Group1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx <> = 1 + 1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx \"1\" = \"xxx\"; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx 1 = yyy; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx 1.23 = Group1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx <> = 1 + 1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx \"1\" = \"xxx\"; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx 1 = yyy; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx 1.23 = Group1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx <> = 1 + 1; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_unknown_declaration_statements\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut m: usize = 0 as usize;
            while m
                < (::core::mem::size_of::<[C2Rust_Unnamed; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
            {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                usize,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const (*(&raw const modes as *const C2Rust_Unnamed).offset(m as isize))
                        .flags as *mut ::core::ffi::c_void,
                    modes[m as usize].name,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    if modes[m as usize].fail as ::core::ffi::c_int != 0 {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    } else {
                        tests[k as usize].expected
                    },
                    update_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output( ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&modes[m].flags, modes[m].name, tests[k].keymap, strlen(tests[k].keymap), (modes[m].fail ? NULL : tests[k].expected), update_files )\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/lenient-mode.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        154 as ::core::ffi::c_uint,
                        b"void test_unknown_declaration_statements(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_unknown_compound_statements(
    mut ctx: *mut xkb_context,
    mut update_files: bool,
) {
    unsafe {
        static mut tests: [keymap_test_data; 10] = [
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx {}; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx \"yyy\" {}; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx.yyy = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx { yyy = 1; }; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx 1 { yyy[1] = true; }; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx.yyy = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx 1.23 {}; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx.yyy = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx <> { a[<>] = {}; }; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx.yyy = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_unknown_compound_statements\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut m: usize = 0 as usize;
            while m
                < (::core::mem::size_of::<[C2Rust_Unnamed; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
            {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                usize,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const (*(&raw const modes as *const C2Rust_Unnamed).offset(m as isize))
                        .flags as *mut ::core::ffi::c_void,
                    modes[m as usize].name,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    if modes[m as usize].fail as ::core::ffi::c_int != 0 {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    } else {
                        tests[k as usize].expected
                    },
                    update_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output( ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&modes[m].flags, modes[m].name, tests[k].keymap, strlen(tests[k].keymap), (modes[m].fail ? NULL : tests[k].expected), update_files )\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/lenient-mode.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        232 as ::core::ffi::c_uint,
                        b"void test_unknown_compound_statements(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_unknown_fields(mut ctx: *mut xkb_context, mut update_files: bool) {
    unsafe {
        static mut tests: [keymap_test_data; 8] = [
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_keycodes { xxx = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { xxx = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_types { type \"1\" { xxx = 0; }; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-types-unknown-type-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { xxx = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { interpret A { xxx = 0; }; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-interpret-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { indicator \"1\" { xxx = 0; }; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-indicator-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { xxx = 0; }; };\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { xxx = 0, [none], [NoAction()] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-key-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_unknown_fields\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut m: usize = 0 as usize;
            while m
                < (::core::mem::size_of::<[C2Rust_Unnamed; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
            {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                usize,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const (*(&raw const modes as *const C2Rust_Unnamed).offset(m as isize))
                        .flags as *mut ::core::ffi::c_void,
                    modes[m as usize].name,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    if modes[m as usize].fail as ::core::ffi::c_int != 0 {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    } else {
                        tests[k as usize].expected
                    },
                    update_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output( ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&modes[m].flags, modes[m].name, tests[k].keymap, strlen(tests[k].keymap), (modes[m].fail ? NULL : tests[k].expected), update_files )\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/lenient-mode.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        306 as ::core::ffi::c_uint,
                        b"void test_unknown_fields(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_unknown_values(mut ctx: *mut xkb_context, mut update_files: bool) {
    unsafe {
        static mut tests: [keymap_test_data; 1] = [
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes {\n    minimum = xxx;\n    maximum = \"x\";\n    maximum[0] = 1;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_unknown_values\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut m: usize = 0 as usize;
            while m
                < (::core::mem::size_of::<[C2Rust_Unnamed; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
            {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                usize,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const (*(&raw const modes as *const C2Rust_Unnamed).offset(m as isize))
                        .flags as *mut ::core::ffi::c_void,
                    modes[m as usize].name,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    if modes[m as usize].fail as ::core::ffi::c_int != 0 {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    } else {
                        tests[k as usize].expected
                    },
                    update_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output( ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&modes[m].flags, modes[m].name, tests[k].keymap, strlen(tests[k].keymap), (modes[m].fail ? NULL : tests[k].expected), update_files )\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/lenient-mode.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        380 as ::core::ffi::c_uint,
                        b"void test_unknown_values(struct xkb_context *, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_actions(mut ctx: *mut xkb_context, mut update_files: bool) {
    unsafe {
        static mut tests: [keymap_test_data; 22] = [
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { SetMods.xxx = 0; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { SetMods.mods = xxx; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { SetMods.mods = \"0\"; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { SetMods.mods[0] = 0; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_compat { XXX.xxx = 0; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_compat { interpret A { action = XXX(); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-interpret-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_compat { interpret A { action = XXX(xxx=0); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-interpret-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_compat { interpret A { action = SetMods(xxx=0); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-action-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_compat { interpret A { action = SetMods(mods=xxx); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-action-value.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_compat { interpret A { action = SetMods(mods=\"0\"); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-action-value.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_compat { interpret A { action = SetMods(mods[0]=0); }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-compat-unknown-action-value.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { SetMods.xxx = 0; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { SetMods.mods = xxx; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { SetMods.mods = \"0\"; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { SetMods.mods[0] = 0; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap { xkb_symbols { XXX.xxx = 0; }; };\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/optional-components-none.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { [none], [XXX()] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-key-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { [none], [XXX(xxx=0)] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-key-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { [none], [SetMods(xxx=0)] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-action-field.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { [none], [SetMods(mods=xxx)] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-action-value.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { [none], [SetMods(mods=\"0\")] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-action-value.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
            keymap_test_data {
                keymap: b"default xkb_keymap {\n  xkb_keycodes { <> = 1; };\n  xkb_symbols { key <> { [none], [SetMods(mods[0]=0)] }; };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected: b"keymaps/lax-symbols-unknown-action-value.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                skip: false,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[keymap_test_data; 22]>() as usize)
                .wrapping_div(::core::mem::size_of::<keymap_test_data>() as usize)
        {
            fprintf(
                stderr,
                b"------\n*** %s: #%u ***\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_actions\0".as_ptr() as *const ::core::ffi::c_char,
                k,
            );
            let mut m: usize = 0 as usize;
            while m
                < (::core::mem::size_of::<[C2Rust_Unnamed; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
            {
                if test_compile_output(
                    ctx,
                    XKB_KEYMAP_FORMAT_TEXT_V2,
                    4294967295 as xkb_keymap_format,
                    Some(
                        compile_buffer
                            as unsafe extern "C" fn(
                                *mut xkb_context,
                                xkb_keymap_format,
                                *const ::core::ffi::c_char,
                                usize,
                                *mut ::core::ffi::c_void,
                            ) -> *mut xkb_keymap,
                    ),
                    &raw const (*(&raw const modes as *const C2Rust_Unnamed).offset(m as isize))
                        .flags as *mut ::core::ffi::c_void,
                    modes[m as usize].name,
                    tests[k as usize].keymap,
                    strlen(tests[k as usize].keymap),
                    if modes[m as usize].fail as ::core::ffi::c_int != 0 {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    } else {
                        tests[k as usize].expected
                    },
                    update_files,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"test_compile_output( ctx, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_USE_ORIGINAL_FORMAT, compile_buffer, (void*)&modes[m].flags, modes[m].name, tests[k].keymap, strlen(tests[k].keymap), (modes[m].fail ? NULL : tests[k].expected), update_files )\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/lenient-mode.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        538 as ::core::ffi::c_uint,
                        b"void test_actions(struct xkb_context *, _Bool)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut update_output_files: bool = false_0 != 0;
        if argc > 1 as ::core::ffi::c_int {
            if streq(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"update\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                update_output_files = true_0 != 0;
            } else {
                fprintf(
                    stderr,
                    b"ERROR: unsupported argument: \"%s\".\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                );
                exit(EXIT_FAILURE);
            }
        }
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        test_unknown_declaration_statements(ctx, update_output_files);
        test_unknown_compound_statements(ctx, update_output_files);
        test_unknown_fields(ctx, update_output_files);
        test_unknown_values(ctx, update_output_files);
        test_actions(ctx, update_output_files);
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
