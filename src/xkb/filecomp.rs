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
    use super::FILE_h::FILE;
    extern "C" {
        pub type xkb_context;
        pub type xkb_keymap;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_keymap_new_from_file(
            context: *mut xkb_context,
            file: *mut FILE,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    }
}
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    use super::xkbcommon_h::{xkb_context, xkb_keymap, xkb_keymap_format};
    extern "C" {
        pub fn test_init();
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
        pub fn test_compile_file(
            context: *mut xkb_context,
            format: xkb_keymap_format,
            path_rel: *const ::core::ffi::c_char,
        ) -> *mut xkb_keymap;
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stdin: *mut FILE;
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
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
use self::assert_h::__assert_fail;
use self::stdio_h::{fclose, stdin};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    test_compile_file, test_context_flags, test_get_context, test_init,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::xkbcommon_h::{
    xkb_context, xkb_context_unref, xkb_keymap, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keymap_new_from_file, xkb_keymap_unref, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
};
pub use self::FILE_h::FILE;
unsafe extern "C" fn test_file(
    mut ctx: *mut xkb_context,
    mut path_rel: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut keymap: *mut xkb_keymap =
            test_compile_file(ctx, XKB_KEYMAP_FORMAT_TEXT_V1, path_rel);
        if keymap.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        xkb_keymap_unref(keymap);
        return 1 as ::core::ffi::c_int;
    }
}
unsafe fn main_0() -> ::core::ffi::c_int {
    unsafe {
        test_init();
        let mut ctx: *mut xkb_context = test_get_context(CONTEXT_NO_FLAG);
        if test_file(
            ctx,
            b"keymaps/basic.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/basic.xkb\")\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                32 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/comprehensive-plus-geom.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/comprehensive-plus-geom.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                33 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/no-types.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/no-types.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/quartz.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/quartz.xkb\")\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                35 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/no-aliases.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/no-aliases.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                36 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/modmap-none.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/modmap-none.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/invalid-escape-sequence.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
        } else {
            __assert_fail(
                b"test_file(ctx, \"keymaps/invalid-escape-sequence.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/divide-by-zero.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"keymaps/divide-by-zero.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                40 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/syntax-error.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"keymaps/syntax-error.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                41 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/syntax-error2.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"keymaps/syntax-error2.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                42 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/empty-symbol-decl.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"keymaps/empty-symbol-decl.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/invalid-qualified-type-field.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"keymaps/invalid-qualified-type-field.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"keymaps/invalid-qualified-symbols-field.xkb\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"keymaps/invalid-qualified-symbols-field.xkb\")\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if test_file(
            ctx,
            b"does not exist\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
        } else {
            __assert_fail(
                b"!test_file(ctx, \"does not exist\")\0".as_ptr() as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                46 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        fclose(stdin);
        if xkb_keymap_new_from_file(
            ctx,
            ::core::ptr::null_mut::<FILE>(),
            XKB_KEYMAP_FORMAT_TEXT_V1,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_file(ctx, NULL, XKB_KEYMAP_FORMAT_TEXT_V1, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_file(
            ctx,
            stdin,
            0 as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_file(ctx, stdin, 0, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                51 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_file(
            ctx,
            stdin,
            4294967295 as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_file(ctx, stdin, XKB_KEYMAP_USE_ORIGINAL_FORMAT, 0)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                52 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_file(
            ctx,
            stdin,
            1234 as xkb_keymap_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_file(ctx, stdin, 1234, 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                53 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_file(
            ctx,
            stdin,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            4294967295 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_file(ctx, stdin, XKB_KEYMAP_FORMAT_TEXT_V1, -1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        if xkb_keymap_new_from_file(
            ctx,
            stdin,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            1234 as xkb_keymap_compile_flags,
        )
        .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_file(ctx, stdin, XKB_KEYMAP_FORMAT_TEXT_V1, 1234)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../test/filecomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_uint,
                b"int main(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_context_unref(ctx);
        return 0 as ::core::ffi::c_int;
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
