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
use crate::xkb::shared_types::xkb_context;
use crate::xkb::shared_types::xkb_keysym_t;
use crate::xkb::shared_types::{darray_char, darray_size_t};
use libc::FILE;
pub type xkb_compose_compile_flags = u32;
pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
pub type xkb_compose_format = u32;
pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
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
extern "C" {
    pub fn xkb_compose_table_dump(file: *mut FILE, table: *mut xkb_compose_table) -> bool;
}
extern "C" {
    pub fn tools_enable_verbose_logging(ctx: *mut xkb_context);
    pub fn is_pipe_or_regular_file(fd: i32) -> bool;
    pub fn tools_read_stdin() -> *mut FILE;
}

pub use crate::xkb::shared_types::STDIN_FILENO;
pub use crate::xkb::shared_types::{no_argument, option, required_argument};
pub use crate::xkb::shared_types::{__LC_ALL, __LC_CTYPE};
pub use crate::xkb::shared_types::{LC_ALL, LC_CTYPE};
use crate::xkb::utils::cstr_cmp;
pub use crate::xkb::utils::getopt_long;
pub use crate::xkb::utils::isempty;
pub use crate::xkb::utils::setlocale;
use crate::xkb::utils::{optarg, optind};
use libc::{exit, fclose, fopen, perror, EXIT_FAILURE, EXIT_SUCCESS};
extern "C" {
    pub static stderr: *mut libc::FILE;
    pub static stdout: *mut libc::FILE;
}
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
        let mut locale: *const i8 = std::ptr::null();
        let mut path: *const i8 = std::ptr::null();
        let mut format: xkb_compose_format = XKB_COMPOSE_FORMAT_TEXT_V1;
        let mut verbose: bool = false;
        let mut test: bool = false;
        static mut opts: [option; 6] = [
            option {
                name: b"help\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: 'h' as i32,
            },
            option {
                name: b"verbose\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: OPT_VERBOSE as i32,
            },
            option {
                name: b"file\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_FILE as i32,
            },
            option {
                name: b"locale\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_LOCALE as i32,
            },
            option {
                name: b"test\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: OPT_TEST as i32,
            },
            option {
                name: std::ptr::null(),
                has_arg: 0 as i32,
                flag: std::ptr::null_mut(),
                val: 0 as i32,
            },
        ];
        setlocale(LC_ALL, b"\0".as_ptr() as *const i8);
        locale = setlocale(LC_CTYPE, std::ptr::null());
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
                    verbose = true;
                }
                1 => {
                    path = optarg;
                    eprintln!("WARNING: the flag --file is deprecated");
                }
                2 => {
                    locale = optarg;
                }
                3 => {
                    test = true;
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
            std::ptr::null_mut();
        if !path.is_null() {
            let mut file: *mut FILE = std::ptr::null_mut();
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
use crate::xkb::context::{xkb_context_new, xkb_context_unref};
use crate::xkb::shared_types::*;
