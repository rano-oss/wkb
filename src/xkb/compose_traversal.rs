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
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod types_h {
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
}
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_char {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut ::core::ffi::c_char,
    }
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
    pub type xkb_keysym_t = uint32_t;
    use super::context_h::xkb_context;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);
        pub fn xkb_context_set_log_verbosity(
            context: *mut xkb_context,
            verbosity: ::core::ffi::c_int,
        );
    }
}
pub mod table_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_compose_table {
        pub refcnt: ::core::ffi::c_int,
        pub format: xkb_compose_format,
        pub flags: xkb_compose_compile_flags,
        pub ctx: *mut xkb_context,
        pub locale: *mut ::core::ffi::c_char,
        pub utf8: darray_char,
        pub nodes: C2Rust_Unnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_1 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut compose_node,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct compose_node {
        pub keysym: xkb_keysym_t,
        pub lokid: uint32_t,
        pub hikid: uint32_t,
        pub c2rust_unnamed: C2Rust_Unnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_2 {
        pub c2rust_unnamed: C2Rust_Unnamed_5,
        pub internal: C2Rust_Unnamed_4,
        pub leaf: C2Rust_Unnamed_3,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_3 {
        #[bitfield(name = "utf8", ty = "uint32_t", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub utf8_is_leaf: [u8; 4],
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        #[bitfield(name = "_pad", ty = "uint32_t", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub _pad_is_leaf: [u8; 4],
        pub eqkid: uint32_t,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        #[bitfield(name = "_pad", ty = "uint32_t", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub _pad_is_leaf: [u8; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_compose_table_entry {
        pub sequence_length: size_t,
        pub sequence: *mut xkb_keysym_t,
        pub keysym: xkb_keysym_t,
        pub utf8: *const ::core::ffi::c_char,
    }
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::darray_h::{darray_char, darray_size_t};
    use super::stdint_uintn_h::uint32_t;
    use super::xkbcommon_compose_h::{xkb_compose_compile_flags, xkb_compose_format};
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod xkbcommon_compose_h {
    pub type xkb_compose_compile_flags = ::core::ffi::c_uint;
    pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
    pub type xkb_compose_format = ::core::ffi::c_uint;
    pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
    use super::context_h::xkb_context;
    use super::table_h::{xkb_compose_table, xkb_compose_table_entry};
    use super::FILE_h::FILE;
    extern "C" {
        pub type xkb_compose_table_iterator;
        pub fn xkb_compose_table_new_from_file(
            context: *mut xkb_context,
            file: *mut FILE,
            locale: *const ::core::ffi::c_char,
            format: xkb_compose_format,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;
        pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);
        pub fn xkb_compose_table_iterator_new(
            table: *mut xkb_compose_table,
        ) -> *mut xkb_compose_table_iterator;
        pub fn xkb_compose_table_iterator_free(iter: *mut xkb_compose_table_iterator);
        pub fn xkb_compose_table_iterator_next(
            iter: *mut xkb_compose_table_iterator,
        ) -> *mut xkb_compose_table_entry;
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
pub mod compose_iter_h {
    pub type xkb_compose_table_iter_t =
        Option<unsafe extern "C" fn(*mut xkb_compose_table_entry, *mut ::core::ffi::c_void) -> ()>;
    use super::table_h::{xkb_compose_table, xkb_compose_table_entry};
    extern "C" {
        pub fn xkb_compose_table_for_each(
            table: *mut xkb_compose_table,
            iter: xkb_compose_table_iter_t,
            data: *mut ::core::ffi::c_void,
        );
    }
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
    use super::context_h::xkb_context;
    use super::messages_codes_h::XKB_LOG_VERBOSITY_MINIMAL;
    use super::xkbcommon_h::{
        xkb_context_set_log_level, xkb_context_set_log_verbosity, XKB_LOG_LEVEL_CRITICAL,
    };
    extern "C" {
        pub fn test_get_path(path_rel: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn test_get_context(flags: test_context_flags) -> *mut xkb_context;
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
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn perror(__s: *const ::core::ffi::c_char);
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
pub mod stdlib_h {
    extern "C" {
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::bench_h::{bench, bench_elapsed_str, bench_start, bench_stop, bench_time};
pub use self::compose_iter_h::{xkb_compose_table_for_each, xkb_compose_table_iter_t};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_char, darray_size_t};
pub use self::internal::__va_list_tag;
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use self::stdint_uintn_h::uint32_t;
use self::stdio_h::{fclose, fopen, fprintf, perror, stderr};
use self::stdlib_h::free;
use self::string_h::strcmp;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::table_h::{
    compose_node, xkb_compose_table, xkb_compose_table_entry, C2Rust_Unnamed_1, C2Rust_Unnamed_2,
    C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5,
};
pub use self::test_h::{
    test_context_flags, test_get_context, test_get_path, xkb_enable_quiet_logging,
    CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG,
};
pub use self::types_h::{__off64_t, __off_t, __uint32_t, __uint64_t};
pub use self::xkbcommon_compose_h::{
    xkb_compose_compile_flags, xkb_compose_format, xkb_compose_table_iterator,
    xkb_compose_table_iterator_free, xkb_compose_table_iterator_new,
    xkb_compose_table_iterator_next, xkb_compose_table_new_from_file, xkb_compose_table_unref,
    XKB_COMPOSE_COMPILE_NO_FLAGS, XKB_COMPOSE_FORMAT_TEXT_V1,
};
pub use self::xkbcommon_h::{
    xkb_context_set_log_level, xkb_context_set_log_verbosity, xkb_context_unref, xkb_keysym_t,
    xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
    XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
pub const BENCHMARK_ITERATIONS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn compose_fn(
    mut entry: *mut xkb_compose_table_entry,
    mut data: *mut ::core::ffi::c_void,
) {
    unsafe {
        if !entry.is_null() {
        } else {
            __assert_fail(
                b"entry\0".as_ptr() as *const ::core::ffi::c_char,
                b"../bench/compose-traversal.c\0".as_ptr() as *const ::core::ffi::c_char,
                22 as ::core::ffi::c_uint,
                b"void compose_fn(struct xkb_compose_table_entry *, void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut table: *mut xkb_compose_table = ::core::ptr::null_mut::<xkb_compose_table>();
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
        let mut use_foreach_impl: bool = argc > 1 as ::core::ffi::c_int
            && strcmp(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"foreach\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        ctx = test_get_context(CONTEXT_NO_FLAG);
        if !ctx.is_null() {
        } else {
            __assert_fail(
                b"ctx\0".as_ptr() as *const ::core::ffi::c_char,
                b"../bench/compose-traversal.c\0".as_ptr() as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        path =
            test_get_path(b"locale/en_US.UTF-8/Compose\0".as_ptr() as *const ::core::ffi::c_char);
        file = fopen(path, b"rb\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if file.is_null() {
            perror(path);
            free(path as *mut ::core::ffi::c_void);
            xkb_context_unref(ctx);
            return -1 as ::core::ffi::c_int;
        }
        free(path as *mut ::core::ffi::c_void);
        xkb_enable_quiet_logging(ctx);
        table = xkb_compose_table_new_from_file(
            ctx,
            file,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            XKB_COMPOSE_FORMAT_TEXT_V1,
            XKB_COMPOSE_COMPILE_NO_FLAGS,
        );
        fclose(file);
        if !table.is_null() {
        } else {
            __assert_fail(
                b"table\0".as_ptr() as *const ::core::ffi::c_char,
                b"../bench/compose-traversal.c\0".as_ptr() as *const ::core::ffi::c_char,
                61 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        bench_start(&raw mut bench);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < BENCHMARK_ITERATIONS {
            if use_foreach_impl {
                xkb_compose_table_for_each(
                    table,
                    Some(
                        compose_fn
                            as unsafe extern "C" fn(
                                *mut xkb_compose_table_entry,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    NULL,
                );
            } else {
                let mut iter: *mut xkb_compose_table_iterator =
                    ::core::ptr::null_mut::<xkb_compose_table_iterator>();
                let mut entry: *mut xkb_compose_table_entry =
                    ::core::ptr::null_mut::<xkb_compose_table_entry>();
                iter = xkb_compose_table_iterator_new(table);
                loop {
                    entry = xkb_compose_table_iterator_next(iter);
                    if entry.is_null() {
                        break;
                    }
                    if !entry.is_null() {
                    } else {
                        __assert_fail(
                            b"entry\0".as_ptr() as *const ::core::ffi::c_char,
                            b"../bench/compose-traversal.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            72 as ::core::ffi::c_uint,
                            b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                }
                xkb_compose_table_iterator_free(iter);
            }
            i += 1;
        }
        bench_stop(&raw mut bench);
        xkb_compose_table_unref(table);
        elapsed = bench_elapsed_str(&raw mut bench);
        fprintf(
            stderr,
            b"traversed %d compose tables in %ss\n\0".as_ptr() as *const ::core::ffi::c_char,
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
