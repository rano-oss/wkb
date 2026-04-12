pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}
pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct option {
        pub name: *const i8,
        pub has_arg: i32,
        pub flag: *mut i32,
        pub val: i32,
    }
    pub const no_argument: i32 = 0 as i32;
    pub const required_argument: i32 = 1 as i32;
    extern "C" {
        pub fn getopt_long(
            ___argc: i32,
            ___argv: *const *mut i8,
            __shortopts: *const i8,
            __longopts: *const option,
            __longind: *mut i32,
        ) -> i32;
    }
}

pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: i32,
        pub _IO_read_ptr: *mut i8,
        pub _IO_read_end: *mut i8,
        pub _IO_read_base: *mut i8,
        pub _IO_write_base: *mut i8,
        pub _IO_write_ptr: *mut i8,
        pub _IO_write_end: *mut i8,
        pub _IO_buf_base: *mut i8,
        pub _IO_buf_end: *mut i8,
        pub _IO_save_base: *mut i8,
        pub _IO_backup_base: *mut i8,
        pub _IO_save_end: *mut i8,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: i32,
        #[bitfield(name = "_flags2", ty = "i32", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [i8; 1],
        pub _old_offset: i64,
        pub _cur_column: u16,
        pub _vtable_offset: i8,
        pub _shortbuf: [i8; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: i64,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: i32,
        pub _unused3: i32,
        pub _total_written: u64,
        pub _unused2: [i8; 8],
    }
    pub type _IO_lock_t = ();
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
    pub use crate::xkb::shared_types::*;
}
pub mod atom_h {
    pub use crate::xkb::shared_types::*;

    extern "C" {
        pub type atom_table;
    }
}

pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_context_flags = u32;
    pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
    pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
    pub use crate::xkb::context::{xkb_context_new, xkb_context_unref};
}
pub mod table_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_compose_table {
        pub refcnt: i32,
        pub format: xkb_compose_format,
        pub flags: xkb_compose_compile_flags,
        pub ctx: *mut xkb_context,
        pub locale: *mut i8,
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
        pub lokid: u32,
        pub hikid: u32,
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
        #[bitfield(name = "utf8", ty = "u32", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub utf8_is_leaf: [u8; 4],
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        #[bitfield(name = "_pad", ty = "u32", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub _pad_is_leaf: [u8; 4],
        pub eqkid: u32,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        #[bitfield(name = "_pad", ty = "u32", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub _pad_is_leaf: [u8; 4],
    }
    use super::context_h::xkb_context;
    use super::xkbcommon_compose_h::{xkb_compose_compile_flags, xkb_compose_format};
    use super::xkbcommon_h::xkb_keysym_t;
    use crate::xkb::shared_types::{darray_char, darray_size_t};
}
pub mod xkbcommon_compose_h {
    pub type xkb_compose_compile_flags = u32;
    pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
    pub type xkb_compose_format = u32;
    pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
    use super::context_h::xkb_context;
    use super::table_h::xkb_compose_table;
    use super::FILE_h::FILE;
    extern "C" {
        pub fn xkb_compose_table_new_from_locale(
            context: *mut xkb_context,
            locale: *const i8,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;
        pub fn xkb_compose_table_new_from_file(
            context: *mut xkb_context,
            file: *mut FILE,
            locale: *const i8,
            format: xkb_compose_format,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;
        pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stdout: *mut FILE;
        pub static mut stderr: *mut FILE;
        pub fn fclose(__stream: *mut FILE) -> i32;
        pub fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
        pub fn perror(__s: *const i8);
    }
}
pub mod dump_h {
    use super::table_h::xkb_compose_table;
    use super::FILE_h::FILE;
    extern "C" {
        pub fn xkb_compose_table_dump(file: *mut FILE, table: *mut xkb_compose_table) -> bool;
    }
}
pub mod tools_common_h {
    use super::context_h::xkb_context;
    use super::FILE_h::FILE;
    extern "C" {
        pub fn tools_enable_verbose_logging(ctx: *mut xkb_context);
        pub fn is_pipe_or_regular_file(fd: i32) -> bool;
        pub fn tools_read_stdin() -> *mut FILE;
    }
}
pub mod include_locale_h {
    pub const LC_CTYPE: i32 = __LC_CTYPE;
    pub const LC_ALL: i32 = __LC_ALL;
    use super::locale_h::{__LC_ALL, __LC_CTYPE};
    extern "C" {
        pub fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    }
}
pub mod getopt_core_h {
    extern "C" {
        pub static mut optarg: *mut i8;
        pub static mut optind: i32;
    }
}
pub mod stdlib_h {
    pub const EXIT_FAILURE: i32 = 1 as i32;
    pub const EXIT_SUCCESS: i32 = 0 as i32;
    extern "C" {
        pub fn exit(__status: i32) -> !;
    }
}
pub mod unistd_h {
    pub const STDIN_FILENO: i32 = 0 as i32;
}
pub mod utils_h {
    #[inline]
    pub unsafe fn isempty(mut s: *const i8) -> bool {
        unsafe {
            return s.is_null() || *s.offset(0 as i32 as isize) as i32 == '\0' as i32;
        }
    }
}
pub mod locale_h {
    pub const __LC_CTYPE: i32 = 0 as i32;
    pub const __LC_ALL: i32 = 6 as i32;
}

pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
use self::dump_h::xkb_compose_table_dump;
use self::getopt_core_h::{optarg, optind};
pub use self::getopt_ext_h::{getopt_long, no_argument, option, required_argument};
pub use self::include_locale_h::{setlocale, LC_ALL, LC_CTYPE};
pub use self::internal::__va_list_tag;
pub use self::locale_h::{__LC_ALL, __LC_CTYPE};
use self::stdio_h::{fclose, fopen, perror, stderr, stdout};
pub use self::stdlib_h::{exit, EXIT_FAILURE, EXIT_SUCCESS};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::table_h::{
    compose_node, xkb_compose_table, C2Rust_Unnamed_1, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5,
};
use self::tools_common_h::{
    is_pipe_or_regular_file, tools_enable_verbose_logging, tools_read_stdin,
};
pub use self::unistd_h::STDIN_FILENO;
pub use self::utils_h::isempty;
pub use self::xkbcommon_compose_h::{
    xkb_compose_compile_flags, xkb_compose_format, xkb_compose_table_new_from_file,
    xkb_compose_table_new_from_locale, xkb_compose_table_unref, XKB_COMPOSE_COMPILE_NO_FLAGS,
    XKB_COMPOSE_FORMAT_TEXT_V1,
};
pub use self::xkbcommon_h::{
    xkb_context_flags, xkb_context_new, xkb_context_unref, xkb_keysym_t, xkb_log_level,
    xkb_rule_names, XKB_CONTEXT_NO_DEFAULT_INCLUDES, XKB_CONTEXT_NO_ENVIRONMENT_NAMES,
    XKB_CONTEXT_NO_FLAGS, XKB_CONTEXT_NO_SECURE_GETENV, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
pub use crate::xkb::shared_types::{darray_char, darray_size_t};
use crate::xkb::utils::cstr_cmp;
pub const OPT_TEST: options = 3;
pub const OPT_LOCALE: options = 2;
pub const OPT_FILE: options = 1;
pub const OPT_VERBOSE: options = 0;
pub type options = u32;
unsafe fn usage(mut fp: *mut FILE, mut progname: *mut i8) {
    unsafe {
        let use_stderr = fp == stderr;
        let msg1 = format!(
            "Usage: {} [--help] [--verbose] [--locale LOCALE] [--test] [FILE]\n",
            crate::xkb::utils::CStrDisplay(progname),
        );
        let msg2 = "\nCompile a Compose file and print it\n\nOptions:\n --help\n    Print this help and exit\n --verbose\n    Enable verbose debugging output\n --file FILE\n    Specify a Compose file to load.\n    DEPRECATED: use the positional argument instead.\n --locale LOCALE\n    Specify the locale directly, instead of relying on the environment variables\n    LC_ALL, LC_TYPE and LANG.\n --test\n    Test compilation but do not print the Compose file.\n";
        if use_stderr {
            eprint!("{}{}", msg1, msg2);
        } else {
            print!("{}{}", msg1, msg2);
        }
    }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut locale: *const i8 = ::core::ptr::null::<i8>();
        let mut path: *const i8 = ::core::ptr::null::<i8>();
        let mut format: xkb_compose_format = XKB_COMPOSE_FORMAT_TEXT_V1;
        let mut verbose: bool = 0 != 0;
        let mut test: bool = 0 != 0;
        static mut opts: [option; 6] = [
            option {
                name: b"help\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: 'h' as i32,
            },
            option {
                name: b"verbose\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: OPT_VERBOSE as i32,
            },
            option {
                name: b"file\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: OPT_FILE as i32,
            },
            option {
                name: b"locale\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: OPT_LOCALE as i32,
            },
            option {
                name: b"test\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: OPT_TEST as i32,
            },
            option {
                name: ::core::ptr::null::<i8>(),
                has_arg: 0 as i32,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: 0 as i32,
            },
        ];
        setlocale(LC_ALL, b"\0".as_ptr() as *const i8);
        locale = setlocale(LC_CTYPE, ::core::ptr::null::<i8>());
        if locale.is_null() {
            locale = b"C\0".as_ptr() as *const i8;
        }
        loop {
            let mut opt: i32 = 0;
            let mut option_index: i32 = 0 as i32;
            opt = getopt_long(
                argc,
                argv as *const *mut i8,
                b"h\0".as_ptr() as *const i8,
                &raw mut opts as *mut option,
                &raw mut option_index,
            );
            if opt == -1 as i32 {
                break;
            }
            match opt {
                0 => {
                    verbose = 1 != 0;
                }
                1 => {
                    path = optarg;
                    eprintln!("WARNING: the flag --file is deprecated");
                }
                2 => {
                    locale = optarg;
                }
                3 => {
                    test = 1 != 0;
                }
                104 => {
                    usage(stdout, *argv.offset(0 as i32 as isize));
                    return EXIT_SUCCESS;
                }
                _ => {
                    usage(stderr, *argv.offset(0 as i32 as isize));
                    return 2;
                }
            }
        }
        if locale.is_null() {
            eprintln!("ERROR: Cannot determine the locale.");
            usage(stderr, *argv.offset(0 as i32 as isize));
            return 2;
        }
        if optind < argc && !isempty(*argv.offset(optind as isize)) {
            if !path.is_null() {
                eprintln!("ERROR: Path already provided via the flag: --file");
                usage(stderr, *argv.offset(0 as i32 as isize));
                exit(2);
            }
            let c2rust_fresh0 = optind;
            optind = optind + 1;
            path = *argv.offset(c2rust_fresh0 as isize);
            if optind < argc {
                eprintln!("ERROR: Too many positional arguments");
                usage(stderr, *argv.offset(0 as i32 as isize));
                exit(2);
            }
        } else if is_pipe_or_regular_file(STDIN_FILENO) {
            path = b"-\0".as_ptr() as *const i8;
        }
        let mut ctx: *mut xkb_context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if ctx.is_null() {
            eprintln!("ERROR: Couldn't create xkb context");
            return EXIT_FAILURE;
        }
        if verbose {
            tools_enable_verbose_logging(ctx);
        }
        let mut ret: i32 = EXIT_FAILURE;
        let mut compose_table: *mut xkb_compose_table =
            ::core::ptr::null_mut::<xkb_compose_table>();
        if !path.is_null() {
            let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
            if isempty(path) as i32 != 0 || cstr_cmp(path, b"-\0".as_ptr() as *const i8) == 0 as i32
            {
                file = tools_read_stdin();
            } else {
                file = fopen(path, b"rb\0".as_ptr() as *const i8) as *mut FILE;
            }
            if file.is_null() {
                perror(path);
                c2rust_current_block = 13276448805914789491;
            } else {
                compose_table = xkb_compose_table_new_from_file(
                    ctx,
                    file,
                    locale,
                    format,
                    XKB_COMPOSE_COMPILE_NO_FLAGS,
                );
                fclose(file);
                if compose_table.is_null() {
                    eprintln!(
                        "ERROR: Couldn't create compose from file: {}",
                        crate::xkb::utils::CStrDisplay(path),
                    );
                    c2rust_current_block = 11742132266850903425;
                } else {
                    c2rust_current_block = 1854459640724737493;
                }
            }
        } else {
            compose_table =
                xkb_compose_table_new_from_locale(ctx, locale, XKB_COMPOSE_COMPILE_NO_FLAGS);
            if compose_table.is_null() {
                eprintln!(
                    "ERROR: Couldn't create compose from locale \"{}\"",
                    crate::xkb::utils::CStrDisplay(locale),
                );
                c2rust_current_block = 11742132266850903425;
            } else {
                c2rust_current_block = 1854459640724737493;
            }
        }
        match c2rust_current_block {
            1854459640724737493 => {
                if test {
                    ret = EXIT_SUCCESS;
                } else {
                    ret = if xkb_compose_table_dump(stdout, compose_table) as i32 != 0 {
                        EXIT_SUCCESS
                    } else {
                        EXIT_FAILURE
                    };
                }
                c2rust_current_block = 11742132266850903425;
            }
            _ => {}
        }
        match c2rust_current_block {
            11742132266850903425 => {
                xkb_compose_table_unref(compose_table);
            }
            _ => {}
        }
        xkb_context_unref(ctx);
        return ret;
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
    let mut args_ptrs: Vec<*mut i8> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut i8)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as i32,
            args_ptrs.as_mut_ptr() as *mut *mut i8,
        ) as i32)
    }
}
