use c2rust_bitfields;
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
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    extern "C" {
        pub type xkb_context;
        pub type xkb_keymap;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);
        pub fn xkb_context_set_log_verbosity(
            context: *mut xkb_context,
            verbosity: ::core::ffi::c_int,
        );
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    }
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
pub mod test_h {
    pub type test_context_flags = ::core::ffi::c_uint;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    #[inline]
    pub unsafe extern "C" fn xkb_enable_quiet_logging(mut ctx: *mut xkb_context) {
        unsafe {
            xkb_context_set_log_level(ctx, XKB_LOG_LEVEL_CRITICAL);
            xkb_context_set_log_verbosity(ctx, XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int);
        }
    }
    use super::messages_codes_h::XKB_LOG_VERBOSITY_MINIMAL;
    use super::xkbcommon_h::{
        xkb_context, xkb_context_set_log_level, xkb_context_set_log_verbosity, xkb_keymap,
        xkb_keymap_format, xkb_log_level, XKB_LOG_LEVEL_CRITICAL,
    };
    extern "C" {
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
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
pub mod bench_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct bench_time {
        pub seconds: ::core::ffi::c_long,
        pub nanoseconds: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct bench {
        pub start: bench_time,
        pub stop: bench_time,
    }
    extern "C" {
        pub fn bench_start(bench: *mut bench);
        pub fn bench_stop(bench: *mut bench);
        pub fn bench_elapsed_str(bench: *const bench) -> *mut ::core::ffi::c_char;
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
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
use self::assert_h::__assert_fail;
pub use self::bench_h::{bench, bench_elapsed_str, bench_start, bench_stop, bench_time};
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::free;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::test_h::{
    test_compile_rules, test_context_flags, test_get_context, xkb_enable_quiet_logging,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::xkbcommon_h::{
    xkb_context, xkb_context_set_log_level, xkb_context_set_log_verbosity, xkb_context_unref,
    xkb_keymap, xkb_keymap_format, xkb_keymap_unref, xkb_log_level, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
pub const BENCHMARK_ITERATIONS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut bench: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut elapsed: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: ::core::ffi::c_int = 0;
        ctx = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../bench/rulescomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                26 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        xkb_enable_quiet_logging(ctx);
        bench_start(&raw mut bench);
        i = 0 as ::core::ffi::c_int;
        while i < BENCHMARK_ITERATIONS {
            keymap = test_compile_rules(
                ctx,
                XKB_KEYMAP_FORMAT_TEXT_V1,
                b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                b"pc104\0".as_ptr() as *const ::core::ffi::c_char,
                b"us\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !keymap.is_null() {
            } else {
                __assert_fail(
                    b"keymap\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../bench/rulescomp.c\0".as_ptr() as *const ::core::ffi::c_char,
                    34 as ::core::ffi::c_uint,
                    b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            };
            xkb_keymap_unref(keymap);
            i += 1;
        }
        bench_stop(&raw mut bench);
        elapsed = bench_elapsed_str(&raw mut bench);
        fprintf(
            stderr,
            b"compiled %d keymaps in %ss\n\0".as_ptr() as *const ::core::ffi::c_char,
            BENCHMARK_ITERATIONS,
            elapsed,
        );
        free(elapsed as *mut ::core::ffi::c_void);
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
