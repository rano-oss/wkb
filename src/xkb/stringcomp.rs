use c2rust_bitfields;
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
pub mod xkbcommon_h {
    pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    pub type xkb_keymap_serialize_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_SERIALIZE_EXPLICIT: xkb_keymap_serialize_flags = 4;
    pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
    pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
    pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
    pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 4294967295 as xkb_keymap_format;
    extern "C" {
        pub type xkb_context;
        pub type xkb_keymap;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_keymap_new_from_string(
            context: *mut xkb_context,
            string: *const ::core::ffi::c_char,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_get_as_string2(
            keymap: *mut xkb_keymap,
            format: xkb_keymap_format,
            flags: xkb_keymap_serialize_flags,
        ) -> *mut ::core::ffi::c_char;
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
            size_t,
            *mut ::core::ffi::c_void,
        ) -> *mut xkb_keymap,
    >;
    use super::__stddef_size_t_h::size_t;
    use super::xkbcommon_h::{
        xkb_context, xkb_keymap, xkb_keymap_format, xkb_keymap_serialize_flags,
        XKB_KEYMAP_SERIALIZE_NO_FLAGS,
    };
    extern "C" {
        pub fn test_init();
        pub fn test_read_file(path_rel: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_string(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            string: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
        pub fn test_compile_output2(
            ctx: *mut xkb_context,
            input_format: xkb_keymap_format,
            output_format: xkb_keymap_format,
            serialize_flags: xkb_keymap_serialize_flags,
            compile_buffer: test_compile_buffer_t,
            compile_buffer_private: *mut ::core::ffi::c_void,
            test_title: *const ::core::ffi::c_char,
            keymap_str: *const ::core::ffi::c_char,
            keymap_len: size_t,
            rel_path: *const ::core::ffi::c_char,
            update_output_files: bool,
        ) -> bool;
        pub fn test_compile_rules(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            rules: *const ::core::ffi::c_char,
            model: *const ::core::ffi::c_char,
            layout: *const ::core::ffi::c_char,
            variant: *const ::core::ffi::c_char,
            options: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
    }
}
pub mod keymap_compare_h {
    pub type xkb_keymap_compare_property = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_CMP_POSSIBLY_DROPPED: xkb_keymap_compare_property = 4;
    pub const XKB_KEYMAP_CMP_ALL: xkb_keymap_compare_property = 31;
    pub const XKB_KEYMAP_CMP_SYMBOLS: xkb_keymap_compare_property = 16;
    pub const XKB_KEYMAP_CMP_KEYCODES: xkb_keymap_compare_property = 8;
    pub const XKB_KEYMAP_CMP_TYPES: xkb_keymap_compare_property = 4;
    pub const XKB_KEYMAP_CMP_LEDS: xkb_keymap_compare_property = 2;
    pub const XKB_KEYMAP_CMP_MODS: xkb_keymap_compare_property = 1;
    use super::xkbcommon_h::{xkb_context, xkb_keymap};
    extern "C" {
        pub fn xkb_keymap_compare(
            ctx: *mut xkb_context,
            keymap1: *const xkb_keymap,
            keymap2: *const xkb_keymap,
            properties: xkb_keymap_compare_property,
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
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub mod utils_text_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn strip_lines(
            input: *const ::core::ffi::c_char,
            input_length: size_t,
            prefix: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn uncomment(
            input: *const ::core::ffi::c_char,
            input_length: size_t,
            prefix: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
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
pub mod stdlib_h {
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::keymap_compare_h::{
    xkb_keymap_compare, xkb_keymap_compare_property, XKB_KEYMAP_CMP_ALL, XKB_KEYMAP_CMP_KEYCODES,
    XKB_KEYMAP_CMP_LEDS, XKB_KEYMAP_CMP_MODS, XKB_KEYMAP_CMP_POSSIBLY_DROPPED,
    XKB_KEYMAP_CMP_SYMBOLS, XKB_KEYMAP_CMP_TYPES,
};
use self::stdio_h::{fprintf, stderr};
use self::string_h::{strcmp, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    test_compile_buffer_t, test_compile_output2, test_compile_rules, test_compile_string,
    test_context_flags, test_get_context, test_init, test_read_file,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::utils_h::{streq, streq_not_null};
use self::utils_text_h::{strip_lines, uncomment};
pub use self::xkbcommon_h::{
    xkb_context, xkb_context_unref, xkb_keymap, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keymap_get_as_string2, xkb_keymap_new_from_string, xkb_keymap_serialize_flags,
    xkb_keymap_unref, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE,
    XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_SERIALIZE_EXPLICIT,
    XKB_KEYMAP_SERIALIZE_KEEP_UNUSED, XKB_KEYMAP_SERIALIZE_NO_FLAGS, XKB_KEYMAP_SERIALIZE_PRETTY,
    XKB_KEYMAP_USE_ORIGINAL_FORMAT,
};
pub use self::FILE_h::FILE;
pub use self::__stddef_null_h::NULL;
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdlib_h::{exit, free, EXIT_FAILURE, EXIT_SUCCESS};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub keymap1: *const ::core::ffi::c_char,
    pub keymap2: *const ::core::ffi::c_char,
    pub properties: xkb_keymap_compare_property,
    pub same: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub path: *const ::core::ffi::c_char,
    pub format: xkb_keymap_format,
    pub serialize_flags: xkb_keymap_serialize_flags,
}
unsafe extern "C" fn test_keymap_comparison(mut ctx: *mut xkb_context) {
    unsafe {
        static mut tests: [C2Rust_Unnamed; 3] = [
            C2Rust_Unnamed {
                keymap1: b"xkb_keymap {};\0".as_ptr() as *const ::core::ffi::c_char,
                keymap2: b"xkb_keymap {};\0".as_ptr() as *const ::core::ffi::c_char,
                properties: XKB_KEYMAP_CMP_ALL,
                same: true_0 != 0,
            },
            C2Rust_Unnamed {
                keymap1: b"xkb_keymap {\n  xkb_keycodes { <> = 1; };\n};\0".as_ptr()
                    as *const ::core::ffi::c_char,
                keymap2: b"xkb_keymap {};\0".as_ptr() as *const ::core::ffi::c_char,
                properties: XKB_KEYMAP_CMP_ALL,
                same: false_0 != 0,
            },
            C2Rust_Unnamed {
                keymap1: b"xkb_keymap {\n  xkb_keycodes {\n    <a> = 1;\n    <b> = 2;\n  };\n  xkb_compat { include \"caps\" };\n  xkb_types { include \"basic+iso9995\" };\n  xkb_symbols {\n    key <a> { [a, A] };\n    key <b> { [Caps_Lock] };\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                keymap2: b"xkb_keymap {\n  xkb_keycodes {\n    <a> = 1;\n    <b> = 2;\n  };\n  xkb_compat {};\n  xkb_types { include \"basic\" };\n  xkb_symbols {\n    key <b> {\n      [Caps_Lock],\n      [LockMods(modifiers = Lock)],\n      type=\"ONE_LEVEL\"\n    };\n    key <a> { [a, A], type=\"ALPHABETIC\" };\n    virtual_modifiers LevelThree;\n  };\n};\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                properties: (XKB_KEYMAP_CMP_ALL as ::core::ffi::c_int
                    & !(XKB_KEYMAP_CMP_POSSIBLY_DROPPED as ::core::ffi::c_int))
                    as xkb_keymap_compare_property,
                same: true_0 != 0,
            },
        ];
        let mut t: size_t = 0 as size_t;
        while t
            < (::core::mem::size_of::<[C2Rust_Unnamed; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed>() as usize)
        {
            fprintf(
                stderr,
                b"------\n%s: #%zu\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"test_keymap_comparison\0".as_ptr() as *const ::core::ffi::c_char,
                t,
            );
            let keymap1: *mut xkb_keymap =
                test_compile_string(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, tests[t as usize].keymap1)
                    as *mut xkb_keymap;
            if !keymap1.is_null() {
            } else {
                __assert_fail(
                    b"keymap1\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    87 as ::core::ffi::c_uint,
                    b"void test_keymap_comparison(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let keymap2: *mut xkb_keymap =
                test_compile_string(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, tests[t as usize].keymap2)
                    as *mut xkb_keymap;
            if !keymap2.is_null() {
            } else {
                __assert_fail(
                    b"keymap2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    91 as ::core::ffi::c_uint,
                    b"void test_keymap_comparison(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            if xkb_keymap_compare(ctx, keymap1, keymap2, tests[t as usize].properties)
                as ::core::ffi::c_int
                == tests[t as usize].same as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"xkb_keymap_compare(ctx, keymap1, keymap2, tests[t].properties) == tests[t].same\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    93 as ::core::ffi::c_uint,
                    b"void test_keymap_comparison(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap1);
            xkb_keymap_unref(keymap2);
            t = t.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_explicit_actions(mut ctx: *mut xkb_context) {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut original: *mut ::core::ffi::c_char = test_read_file(
            b"keymaps/explicit-actions.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !original.is_null() {
        } else {
            __assert_fail(
                b"original\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut tmp: *mut ::core::ffi::c_char = uncomment(
            original,
            strlen(original),
            b"//\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !tmp.is_null() {
        } else {
            __assert_fail(
                b"tmp\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                107 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut expected: *mut ::core::ffi::c_char = strip_lines(
            tmp,
            strlen(tmp),
            b"//\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(tmp as *mut ::core::ffi::c_void);
        if !expected.is_null() {
        } else {
            __assert_fail(
                b"expected\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        let mut got: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        keymap = test_compile_string(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, original);
        free(original as *mut ::core::ffi::c_void);
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        got = xkb_keymap_get_as_string2(
            keymap,
            XKB_KEYMAP_USE_ORIGINAL_FORMAT,
            (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                as xkb_keymap_serialize_flags,
        );
        xkb_keymap_unref(keymap);
        let __cond: bool = streq_not_null(expected, got) as bool;
        if !__cond {
            fprintf(
                stderr,
                b"Assertion failure: Check output from original. Expected \"%s\", got: \"%s\"\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expected,
                got,
            );
            if __cond as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_uint,
                    b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
        }
        free(got as *mut ::core::ffi::c_void);
        keymap = test_compile_string(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, expected);
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_uint,
                b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        got = xkb_keymap_get_as_string2(
            keymap,
            XKB_KEYMAP_USE_ORIGINAL_FORMAT,
            (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                as xkb_keymap_serialize_flags,
        );
        xkb_keymap_unref(keymap);
        let __cond_0: bool = streq_not_null(expected, got) as bool;
        if !__cond_0 {
            fprintf(
                stderr,
                b"Assertion failure: Check roundtrip. Expected \"%s\", got: \"%s\"\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                expected,
                got,
            );
            if __cond_0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"__cond\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    129 as ::core::ffi::c_uint,
                    b"void test_explicit_actions(struct xkb_context *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
        }
        free(got as *mut ::core::ffi::c_void);
        free(expected as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn compile_string(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut buf: *const ::core::ffi::c_char,
    mut len: size_t,
    mut private: *mut ::core::ffi::c_void,
) -> *mut xkb_keymap {
    unsafe {
        return test_compile_string(context, format, buf);
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
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut dump: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut dump2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            b"xkb_keymap {};\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, \"xkb_keymap {};\", XKB_KEYMAP_FORMAT_TEXT_V1, -1)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                166 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            b"xkb_keymap {};\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            65535 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, \"xkb_keymap {};\", XKB_KEYMAP_FORMAT_TEXT_V1, 0xffff)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        keymap = test_compile_string(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if keymap.is_null() {
        } else {
            __assert_fail(
                b"!keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                172 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        static mut data: [C2Rust_Unnamed_0; 6] = [
            C2Rust_Unnamed_0 {
                path: b"keymaps/stringcomp-v1.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V1,
                serialize_flags: (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                    as xkb_keymap_serialize_flags,
            },
            C2Rust_Unnamed_0 {
                path: b"keymaps/stringcomp-v1-no-prettyfied.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V1,
                serialize_flags: ((XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                    & !(XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int))
                    as xkb_keymap_serialize_flags,
            },
            C2Rust_Unnamed_0 {
                path: b"keymaps/stringcomp-v1-no-flags.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V1,
                serialize_flags: XKB_KEYMAP_SERIALIZE_NO_FLAGS,
            },
            C2Rust_Unnamed_0 {
                path: b"keymaps/stringcomp-v2.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V2,
                serialize_flags: (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                    as xkb_keymap_serialize_flags,
            },
            C2Rust_Unnamed_0 {
                path: b"keymaps/stringcomp-v2-explicit.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V2,
                serialize_flags: (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_EXPLICIT as ::core::ffi::c_int)
                    as xkb_keymap_serialize_flags,
            },
            C2Rust_Unnamed_0 {
                path: b"keymaps/stringcomp-v2-explicit-drop-unused.xkb\0".as_ptr()
                    as *const ::core::ffi::c_char,
                format: XKB_KEYMAP_FORMAT_TEXT_V2,
                serialize_flags: ((XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int
                    | XKB_KEYMAP_SERIALIZE_EXPLICIT as ::core::ffi::c_int)
                    & !(XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int))
                    as xkb_keymap_serialize_flags,
            },
        ];
        let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (k as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_0; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_0>() as usize)
        {
            fprintf(
                stderr,
                b"------\nTest roundtrip of: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                data[k as usize].path,
            );
            let mut original: *mut ::core::ffi::c_char = test_read_file(data[k as usize].path);
            if !original.is_null() {
            } else {
                __assert_fail(
                    b"original\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if test_compile_output2(
                ctx,
                data[k as usize].format,
                4294967295 as xkb_keymap_format,
                data[k as usize].serialize_flags,
                Some(
                    compile_string
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"Round-trip with same serialize flags\0".as_ptr() as *const ::core::ffi::c_char,
                original,
                0 as size_t,
                data[k as usize].path,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output2(ctx, data[k].format, XKB_KEYMAP_USE_ORIGINAL_FORMAT, data[k].serialize_flags, compile_string, NULL, \"Round-trip with same serialize flags\", original, 0 , data[k].path, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    229 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            keymap = xkb_keymap_new_from_string(
                ctx,
                original,
                data[k as usize].format,
                XKB_KEYMAP_COMPILE_STRICT_MODE,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    239 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            free(original as *mut ::core::ffi::c_void);
            let test_serialize_flags: [xkb_keymap_serialize_flags; 2] = [
                (data[k as usize].serialize_flags as ::core::ffi::c_uint
                    & !(XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                        as ::core::ffi::c_uint) as xkb_keymap_serialize_flags,
                (data[k as usize].serialize_flags as ::core::ffi::c_uint
                    | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as xkb_keymap_serialize_flags,
            ];
            let mut f: size_t = 0 as size_t;
            while f
                < (::core::mem::size_of::<[xkb_keymap_serialize_flags; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<xkb_keymap_serialize_flags>() as usize)
            {
                original = xkb_keymap_get_as_string2(
                    keymap,
                    data[k as usize].format,
                    test_serialize_flags[f as usize],
                );
                if !original.is_null() {
                } else {
                    __assert_fail(
                        b"original\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        248 as ::core::ffi::c_uint,
                        b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                let mut keymap2: *mut xkb_keymap = xkb_keymap_new_from_string(
                    ctx,
                    original,
                    data[k as usize].format,
                    XKB_KEYMAP_COMPILE_STRICT_MODE,
                );
                if !keymap2.is_null() {
                } else {
                    __assert_fail(
                        b"keymap2\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        252 as ::core::ffi::c_uint,
                        b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                free(original as *mut ::core::ffi::c_void);
                if xkb_keymap_compare(
                    ctx,
                    keymap,
                    keymap2,
                    (XKB_KEYMAP_CMP_ALL as ::core::ffi::c_int
                        & !(XKB_KEYMAP_CMP_POSSIBLY_DROPPED as ::core::ffi::c_int))
                        as xkb_keymap_compare_property,
                ) as ::core::ffi::c_int
                    != 0
                {
                } else {
                    __assert_fail(
                        b"xkb_keymap_compare(ctx, keymap, keymap2, (XKB_KEYMAP_CMP_ALL & ~(XKB_KEYMAP_CMP_POSSIBLY_DROPPED)))\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                        256 as ::core::ffi::c_uint,
                        b"int main(int, char **)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                xkb_keymap_unref(keymap2);
                f = f.wrapping_add(1);
            }
            let serialize_flags: xkb_keymap_serialize_flags = (if data[k as usize].serialize_flags
                as ::core::ffi::c_uint
                & XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                data[k as usize].serialize_flags as ::core::ffi::c_uint
                    & !(XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int) as ::core::ffi::c_uint
            } else {
                data[k as usize].serialize_flags as ::core::ffi::c_uint
                    | XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int as ::core::ffi::c_uint
            })
                as xkb_keymap_serialize_flags;
            original = xkb_keymap_get_as_string2(keymap, data[k as usize].format, serialize_flags);
            if !original.is_null() {
            } else {
                __assert_fail(
                    b"original\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    272 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            let mut keymap2_0: *mut xkb_keymap = xkb_keymap_new_from_string(
                ctx,
                original,
                data[k as usize].format,
                XKB_KEYMAP_COMPILE_STRICT_MODE,
            );
            if !keymap2_0.is_null() {
            } else {
                __assert_fail(
                    b"keymap2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if xkb_keymap_compare(ctx, keymap, keymap2_0, XKB_KEYMAP_CMP_ALL) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"xkb_keymap_compare(ctx, keymap, keymap2, XKB_KEYMAP_CMP_ALL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap2_0);
            if test_compile_output2(
                ctx,
                data[k as usize].format,
                4294967295 as xkb_keymap_format,
                data[k as usize].serialize_flags,
                Some(
                    compile_string
                        as unsafe extern "C" fn(
                            *mut xkb_context,
                            xkb_keymap_format,
                            *const ::core::ffi::c_char,
                            size_t,
                            *mut ::core::ffi::c_void,
                        ) -> *mut xkb_keymap,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                b"Round-trip with different serialize flags\0".as_ptr()
                    as *const ::core::ffi::c_char,
                original,
                0 as size_t,
                data[k as usize].path,
                update_output_files,
            ) as ::core::ffi::c_int
                != 0
            {
            } else {
                __assert_fail(
                    b"test_compile_output2(ctx, data[k].format, XKB_KEYMAP_USE_ORIGINAL_FORMAT, data[k].serialize_flags, compile_string, NULL, \"Round-trip with different serialize flags\", original, 0 , data[k].path, update_output_files)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    287 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            free(original as *mut ::core::ffi::c_void);
            xkb_keymap_unref(keymap);
            k = k.wrapping_add(1);
        }
        keymap = test_compile_rules(
            ctx,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"ru,ca,de,us\0".as_ptr() as *const ::core::ffi::c_char,
            b",multix,neo,intl\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                295 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        dump = xkb_keymap_get_as_string2(
            keymap,
            XKB_KEYMAP_USE_ORIGINAL_FORMAT,
            (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                as xkb_keymap_serialize_flags,
        );
        if !dump.is_null() {
        } else {
            __assert_fail(
                b"dump\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                298 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        keymap = test_compile_string(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, dump);
        if !keymap.is_null() {
        } else {
            __assert_fail(
                b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        dump2 = xkb_keymap_get_as_string2(
            keymap,
            XKB_KEYMAP_USE_ORIGINAL_FORMAT,
            (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                as xkb_keymap_serialize_flags,
        );
        if !dump2.is_null() {
        } else {
            __assert_fail(
                b"dump2\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if streq(dump, dump2) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"streq(dump, dump2)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                306 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            dump,
            0 as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, dump, 0, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            dump,
            4294967295 as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, dump, -1, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            dump,
            4294967295 as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, dump, XKB_KEYMAP_USE_ORIGINAL_FORMAT, 0)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            dump,
            (XKB_KEYMAP_FORMAT_TEXT_V2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, dump, XKB_KEYMAP_FORMAT_TEXT_V2+1, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                312 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            dump,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, dump, XKB_KEYMAP_FORMAT_TEXT_V1, -1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                313 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_string(
            ctx,
            dump,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            1414 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_string(ctx, dump, XKB_KEYMAP_FORMAT_TEXT_V1, 1414)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                314 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_get_as_string2(
            keymap,
            4294967295 as xkb_keymap_format,
            4294967295 as xkb_keymap_serialize_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_get_as_string2(keymap, XKB_KEYMAP_USE_ORIGINAL_FORMAT, -1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                315 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_get_as_string2(
            keymap,
            4294967295 as xkb_keymap_format,
            3333 as xkb_keymap_serialize_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_get_as_string2(keymap, XKB_KEYMAP_USE_ORIGINAL_FORMAT, 3333)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_get_as_string2(
            keymap,
            0 as xkb_keymap_format,
            (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                as xkb_keymap_serialize_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_get_as_string2(keymap, 0, TEST_KEYMAP_SERIALIZE_FLAGS)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                317 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_get_as_string2(
            keymap,
            4893 as xkb_keymap_format,
            (XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int
                | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int)
                as xkb_keymap_serialize_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_get_as_string2(keymap, 4893, TEST_KEYMAP_SERIALIZE_FLAGS)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/stringcomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                318 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_keymap_unref(keymap);
        free(dump as *mut ::core::ffi::c_void);
        free(dump2 as *mut ::core::ffi::c_void);
        test_keymap_comparison(ctx);
        test_explicit_actions(ctx);
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
