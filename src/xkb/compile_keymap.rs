// use f128; // f128 is unstable, replaced with f64
pub mod types_h {
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
}
pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct option {
        pub name: *const i8,
        pub has_arg: ::core::ffi::c_int,
        pub flag: *mut ::core::ffi::c_int,
        pub val: ::core::ffi::c_int,
    }
    pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        pub fn getopt_long(
            ___argc: ::core::ffi::c_int,
            ___argv: *const *mut i8,
            __shortopts: *const i8,
            __longopts: *const option,
            __longind: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
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
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [i8; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [i8; 1],
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
        pub _unused2: [i8; 8],
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const i8,
        pub model: *const i8,
        pub layout: *const i8,
        pub variant: *const i8,
        pub options: *const i8,
    }
    pub type xkb_context_flags = ::core::ffi::c_uint;
    pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
    pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
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
    use super::FILE_h::FILE;
    extern "C" {
        pub type xkb_context;
        pub type xkb_keymap;
        pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
        pub fn xkb_context_unref(context: *mut xkb_context);
        pub fn xkb_keymap_new_from_names2(
            context: *mut xkb_context,
            names: *const xkb_rule_names,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_new_from_file(
            context: *mut xkb_context,
            file: *mut FILE,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_get_as_string2(
            keymap: *mut xkb_keymap,
            format: xkb_keymap_format,
            flags: xkb_keymap_serialize_flags,
        ) -> *mut i8;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct estimate {
        pub elapsed: ::core::ffi::c_longlong,
        pub stdev: ::core::ffi::c_longlong,
    }
    extern "C" {
        pub fn bench_start2(bench: *mut bench);
        pub fn bench_stop2(bench: *mut bench);
        pub fn bench_elapsed(bench: *const bench, result: *mut bench_time);
        pub fn predictPerturbed(t1: *const bench_time, t2: *const bench_time, est: *mut estimate);
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stdout: *mut FILE;
        pub static mut stderr: *mut FILE;
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        pub fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
        pub fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
        pub fn fprintf(__stream: *mut FILE, __format: *const i8, ...) -> ::core::ffi::c_int;
        pub fn perror(__s: *const i8);
    }
}
pub mod keymap_formats_h {
    pub const DEFAULT_OUTPUT_KEYMAP_FORMAT: xkb_keymap_format = XKB_KEYMAP_USE_ORIGINAL_FORMAT;
    use super::xkbcommon_h::{xkb_keymap_format, XKB_KEYMAP_USE_ORIGINAL_FORMAT};
    extern "C" {
        pub fn xkb_keymap_parse_format(raw: *const i8) -> xkb_keymap_format;
        pub fn xkb_keymap_get_format_label(format: xkb_keymap_format) -> *const i8;
    }
}
pub mod config_h {
    pub const DEFAULT_XKB_LAYOUT: [i8; 3] =
        unsafe { ::core::mem::transmute::<[u8; 3], [i8; 3]>(*b"us\0") };
    pub const DEFAULT_XKB_MODEL: [i8; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"pc105\0") };
    pub const DEFAULT_XKB_OPTIONS: *mut ::core::ffi::c_void = NULL;
    pub const DEFAULT_XKB_RULES: [i8; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"evdev\0") };
    pub const DEFAULT_XKB_VARIANT: *mut ::core::ffi::c_void = NULL;
    pub const EXIT_INVALID_USAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::__stddef_null_h::NULL;
}
pub mod fcntl_linux_h {
    pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
}
pub mod fcntl_h {
    extern "C" {
        pub fn open(__file: *const i8, __oflag: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    }
}
pub mod getopt_core_h {
    extern "C" {
        pub static mut optarg: *mut i8;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod stdlib_h {
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn atof(__nptr: *const i8) -> ::core::ffi::c_double;
        pub fn atoi(__nptr: *const i8) -> ::core::ffi::c_int;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
pub mod unistd_h {
    pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    extern "C" {
        pub fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        pub fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        pub fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::bench_h::{
    bench, bench_elapsed, bench_start2, bench_stop2, bench_time, estimate, predictPerturbed,
};
pub use self::config_h::{
    DEFAULT_XKB_LAYOUT, DEFAULT_XKB_MODEL, DEFAULT_XKB_OPTIONS, DEFAULT_XKB_RULES,
    DEFAULT_XKB_VARIANT, EXIT_INVALID_USAGE,
};
use self::fcntl_h::open;
pub use self::fcntl_linux_h::O_WRONLY;
use self::getopt_core_h::optarg;
pub use self::getopt_ext_h::{getopt_long, no_argument, option, required_argument};
pub use self::keymap_formats_h::{
    xkb_keymap_get_format_label, xkb_keymap_parse_format, DEFAULT_OUTPUT_KEYMAP_FORMAT,
};
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::{fclose, fflush, fopen, fprintf, perror, stderr, stdout};
pub use self::stdlib_h::{atof, atoi, exit, free, EXIT_FAILURE, EXIT_SUCCESS};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::unistd_h::{close, dup, dup2, STDERR_FILENO, STDOUT_FILENO};
pub use self::xkbcommon_h::{
    xkb_context, xkb_context_flags, xkb_context_new, xkb_context_unref, xkb_keymap,
    xkb_keymap_compile_flags, xkb_keymap_format, xkb_keymap_get_as_string2,
    xkb_keymap_new_from_file, xkb_keymap_new_from_names2, xkb_keymap_serialize_flags,
    xkb_keymap_unref, xkb_rule_names, XKB_CONTEXT_NO_DEFAULT_INCLUDES,
    XKB_CONTEXT_NO_ENVIRONMENT_NAMES, XKB_CONTEXT_NO_FLAGS, XKB_CONTEXT_NO_SECURE_GETENV,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_KEYMAP_SERIALIZE_EXPLICIT, XKB_KEYMAP_SERIALIZE_KEEP_UNUSED,
    XKB_KEYMAP_SERIALIZE_NO_FLAGS, XKB_KEYMAP_SERIALIZE_PRETTY, XKB_KEYMAP_USE_ORIGINAL_FORMAT,
};
pub use self::FILE_h::FILE;
pub const OPT_STDEV: options = 12;
pub const OPT_ITERATIONS: options = 11;
pub const OPT_OPTION: options = 10;
pub const OPT_VARIANT: options = 9;
pub const OPT_LAYOUT: options = 8;
pub const OPT_MODEL: options = 7;
pub const OPT_RULES: options = 6;
pub const OPT_KEYMAP: options = 5;
pub const OPT_KEYMAP_EXPLICIT_VALUES: options = 4;
pub const OPT_KEYMAP_KEEP_UNUSED: options = 3;
pub const OPT_KEYMAP_PRETTY: options = 2;
pub const OPT_KEYMAP_OUTPUT_FORMAT: options = 1;
pub const OPT_KEYMAP_INPUT_FORMAT: options = 0;
pub type options = ::core::ffi::c_uint;
pub const DEFAULT_ITERATIONS: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
pub const DEFAULT_STDEV: ::core::ffi::c_double = 0.05f64;
unsafe extern "C" fn usage(mut fp: *mut FILE, mut argv: *mut *mut i8) {
    unsafe {
        fprintf(
            fp,
            b"Usage: %s [OPTIONS]\n\nBenchmark compilation of the given RMLVO\n\nOptions:\n --help\n    Print this help and exit\n --iter\n    Exact number of iterations to run\n --stdev\n    Minimal relative standard deviation (percentage) to reach.\n    (default: %f)\nNote: --iter and --stdev are mutually exclusive.\n\nXKB-specific options:\n --input-format <format>\n    The keymap format to use for parsing (default: '%s')\n --output-format <format>\n    The keymap format to use for serializing (default: same as input)\n --pretty\n    Enable pretty-printing in keymap serialization\n --keep-unused\n    Keep unused bits in keymap serialization\n --explicit-values\n    Force serializing explicit values\n --keymap\n    Load the corresponding XKB file, ignore RMLVO options.\n --rules <rules>\n    The XKB ruleset (default: '%s')\n --model <model>\n    The XKB model (default: '%s')\n --layout <layout>\n    The XKB layout (default: '%s')\n --variant <variant>\n    The XKB layout variant (default: '%s')\n --options <options>\n    The XKB options (default: '%s')\n\n\0"
                .as_ptr() as *const i8,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            DEFAULT_STDEV * 100 as ::core::ffi::c_int as ::core::ffi::c_double,
            xkb_keymap_get_format_label(XKB_KEYMAP_FORMAT_TEXT_V2),
            DEFAULT_XKB_RULES.as_ptr(),
            DEFAULT_XKB_MODEL.as_ptr(),
            DEFAULT_XKB_LAYOUT.as_ptr(),
            if !DEFAULT_XKB_VARIANT.is_null() {
                ::core::ptr::null::<i8>()
            } else {
                b"<none>\0".as_ptr() as *const i8
            },
            if !DEFAULT_XKB_OPTIONS.is_null() {
                ::core::ptr::null::<i8>()
            } else {
                b"<none>\0".as_ptr() as *const i8
            },
        );
    }
}
unsafe extern "C" fn load_keymap(
    mut ctx: *mut xkb_context,
    mut keymap_path: *const i8,
    mut rmlvo: *const xkb_rule_names,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        if !keymap_path.is_null() {
            let mut file: *mut FILE = fopen(keymap_path, b"r\0".as_ptr() as *const i8) as *mut FILE;
            if file.is_null() {
                fprintf(
                    stderr,
                    b"ERROR: cannot open file: %s\n\0".as_ptr() as *const i8,
                    keymap_path,
                );
                return ::core::ptr::null_mut::<xkb_keymap>();
            }
            let mut keymap: *mut xkb_keymap =
                xkb_keymap_new_from_file(ctx, file, format, XKB_KEYMAP_COMPILE_NO_FLAGS);
            fclose(file);
            return keymap;
        } else {
            return xkb_keymap_new_from_names2(ctx, rmlvo, format, XKB_KEYMAP_COMPILE_NO_FLAGS);
        };
    }
}
unsafe fn main_0(mut argc: ::core::ffi::c_int, mut argv: *mut *mut i8) -> ::core::ffi::c_int {
    unsafe {
        let mut stdout_old: ::core::ffi::c_int = 0;
        let mut stdout_new: ::core::ffi::c_int = 0;
        let mut stderr_old: ::core::ffi::c_int = 0;
        let mut stderr_new: ::core::ffi::c_int = 0;
        let mut total_elapsed: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut context: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
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
        let mut elapsed: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut est: estimate = estimate {
            elapsed: 0,
            stdev: 0,
        };
        let mut keymap_input_format: xkb_keymap_format = XKB_KEYMAP_FORMAT_TEXT_V2;
        let mut keymap_output_format: xkb_keymap_format = DEFAULT_OUTPUT_KEYMAP_FORMAT;
        let mut serialize_flags: xkb_keymap_serialize_flags = XKB_KEYMAP_SERIALIZE_NO_FLAGS;
        let mut explicit_iterations: bool = false_0 != 0;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut keymap_path: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut rmlvo: xkb_rule_names = xkb_rule_names {
            rules: DEFAULT_XKB_RULES.as_ptr(),
            model: DEFAULT_XKB_MODEL.as_ptr(),
            layout: ::core::ptr::null::<i8>(),
            variant: ::core::ptr::null::<i8>(),
            options: ::core::ptr::null::<i8>(),
        };
        let mut max_iterations: ::core::ffi::c_uint = DEFAULT_ITERATIONS as ::core::ffi::c_uint;
        let mut stdev: ::core::ffi::c_double = DEFAULT_STDEV;
        static mut opts: [option; 15] = [
            option {
                name: b"help\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'h' as i32,
            },
            option {
                name: b"input-format\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_KEYMAP_INPUT_FORMAT as ::core::ffi::c_int,
            },
            option {
                name: b"output-format\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_KEYMAP_OUTPUT_FORMAT as ::core::ffi::c_int,
            },
            option {
                name: b"pretty\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_KEYMAP_PRETTY as ::core::ffi::c_int,
            },
            option {
                name: b"keep-unused\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_KEYMAP_KEEP_UNUSED as ::core::ffi::c_int,
            },
            option {
                name: b"explicit-values\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_KEYMAP_EXPLICIT_VALUES as ::core::ffi::c_int,
            },
            option {
                name: b"keymap\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_KEYMAP as ::core::ffi::c_int,
            },
            option {
                name: b"rules\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_RULES as ::core::ffi::c_int,
            },
            option {
                name: b"model\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_MODEL as ::core::ffi::c_int,
            },
            option {
                name: b"layout\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_LAYOUT as ::core::ffi::c_int,
            },
            option {
                name: b"variant\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_VARIANT as ::core::ffi::c_int,
            },
            option {
                name: b"options\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_OPTION as ::core::ffi::c_int,
            },
            option {
                name: b"iter\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_ITERATIONS as ::core::ffi::c_int,
            },
            option {
                name: b"stdev\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_STDEV as ::core::ffi::c_int,
            },
            option {
                name: ::core::ptr::null::<i8>(),
                has_arg: 0 as ::core::ffi::c_int,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 0 as ::core::ffi::c_int,
            },
        ];
        loop {
            let mut c: ::core::ffi::c_int = 0;
            let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            c = getopt_long(
                argc,
                argv,
                b"h\0".as_ptr() as *const i8,
                &raw mut opts as *mut option,
                &raw mut option_index,
            );
            if c == -1 as ::core::ffi::c_int {
                break;
            }
            match c {
                104 => {
                    usage(stdout, argv);
                    exit(EXIT_SUCCESS);
                }
                0 => {
                    keymap_input_format = xkb_keymap_parse_format(optarg);
                    if keymap_input_format as u64 == 0 {
                        fprintf(
                            stderr,
                            b"ERROR: invalid --input-format: \"%s\"\n\0".as_ptr() as *const i8,
                            optarg,
                        );
                        usage(stderr, argv);
                        exit(EXIT_INVALID_USAGE);
                    }
                }
                1 => {
                    keymap_output_format = xkb_keymap_parse_format(optarg);
                    if keymap_output_format as u64 == 0 {
                        fprintf(
                            stderr,
                            b"ERROR: invalid --output-format: \"%s\"\n\0".as_ptr() as *const i8,
                            optarg,
                        );
                        usage(stderr, argv);
                        exit(EXIT_INVALID_USAGE);
                    }
                }
                2 => {
                    serialize_flags = (serialize_flags as ::core::ffi::c_uint
                        | XKB_KEYMAP_SERIALIZE_PRETTY as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as xkb_keymap_serialize_flags;
                }
                3 => {
                    serialize_flags = (serialize_flags as ::core::ffi::c_uint
                        | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as ::core::ffi::c_int
                            as ::core::ffi::c_uint)
                        as xkb_keymap_serialize_flags;
                }
                4 => {
                    serialize_flags = (serialize_flags as ::core::ffi::c_uint
                        | XKB_KEYMAP_SERIALIZE_EXPLICIT as ::core::ffi::c_int
                            as ::core::ffi::c_uint)
                        as xkb_keymap_serialize_flags;
                }
                5 => {
                    keymap_path = optarg;
                }
                6 => {
                    rmlvo.rules = optarg;
                }
                7 => {
                    rmlvo.model = optarg;
                }
                8 => {
                    rmlvo.layout = optarg;
                }
                9 => {
                    rmlvo.variant = optarg;
                }
                10 => {
                    rmlvo.options = optarg;
                }
                11 => {
                    if max_iterations == 0 as ::core::ffi::c_uint {
                        usage(stderr, argv);
                        exit(EXIT_INVALID_USAGE);
                    }
                    let max_iterations_raw: ::core::ffi::c_int = atoi(optarg) as ::core::ffi::c_int;
                    if max_iterations_raw <= 0 as ::core::ffi::c_int {
                        max_iterations = DEFAULT_ITERATIONS as ::core::ffi::c_uint;
                    } else {
                        max_iterations = max_iterations_raw as ::core::ffi::c_uint;
                    }
                    explicit_iterations = true_0 != 0;
                }
                12 => {
                    if explicit_iterations {
                        usage(stderr, argv);
                        exit(EXIT_INVALID_USAGE);
                    }
                    stdev = atof(optarg) / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
                    if stdev <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        stdev = DEFAULT_STDEV;
                    }
                    max_iterations = 0 as ::core::ffi::c_uint;
                }
                _ => {
                    usage(stderr, argv);
                    exit(EXIT_INVALID_USAGE);
                }
            }
        }
        if rmlvo.layout.is_null() || *rmlvo.layout == 0 {
            if !rmlvo.variant.is_null() && *rmlvo.variant as ::core::ffi::c_int != 0 {
                fprintf(
                    stderr,
                    b"Error: a variant requires a layout\n\0".as_ptr() as *const i8,
                );
                return EXIT_INVALID_USAGE;
            }
            rmlvo.layout = DEFAULT_XKB_LAYOUT.as_ptr();
            rmlvo.variant = ::core::ptr::null::<i8>();
        }
        context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if context.is_null() {
            exit(1 as ::core::ffi::c_int);
        }
        let mut keymap: *mut xkb_keymap = load_keymap(
            context,
            keymap_path,
            &raw mut rmlvo,
            keymap_input_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        );
        if keymap.is_null() {
            fprintf(
                stderr,
                b"ERROR: Cannot compile keymap.\n\0".as_ptr() as *const i8,
            );
            ret = EXIT_FAILURE;
        } else {
            fflush(stdout);
            stdout_old = dup(STDOUT_FILENO);
            stdout_new = open(b"/dev/null\0".as_ptr() as *const i8, O_WRONLY);
            if stdout_old == -1 as ::core::ffi::c_int
                || stdout_new == -1 as ::core::ffi::c_int
                || dup2(stdout_new, STDOUT_FILENO) == -1 as ::core::ffi::c_int
            {
                perror(b"Stdout error\0".as_ptr() as *const i8);
                exit(EXIT_FAILURE);
            }
            close(stdout_new);
            fflush(stderr);
            stderr_old = dup(STDERR_FILENO);
            stderr_new = open(b"/dev/null\0".as_ptr() as *const i8, O_WRONLY);
            if stderr_old == -1 as ::core::ffi::c_int
                || stderr_new == -1 as ::core::ffi::c_int
                || dup2(stderr_new, STDERR_FILENO) == -1 as ::core::ffi::c_int
            {
                perror(b"Stderr error\0".as_ptr() as *const i8);
                exit(EXIT_FAILURE);
            }
            close(stderr_new);
            if explicit_iterations {
                stdev = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                bench_start2(&raw mut bench);
                let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                while i < max_iterations {
                    let mut s: *mut i8 =
                        xkb_keymap_get_as_string2(keymap, keymap_output_format, serialize_flags);
                    free(s as *mut ::core::ffi::c_void);
                    i = i.wrapping_add(1);
                }
                bench_stop2(&raw mut bench);
                bench_elapsed(&raw mut bench, &raw mut elapsed);
                est.elapsed = ((elapsed.nanoseconds
                    + 1000000000 as ::core::ffi::c_long * elapsed.seconds)
                    / max_iterations as ::core::ffi::c_long)
                    as ::core::ffi::c_longlong;
                est.stdev = 0 as ::core::ffi::c_longlong;
            } else {
                bench_start2(&raw mut bench);
                let mut _bench: bench = bench {
                    start: bench_time {
                        seconds: 0,
                        nanoseconds: 0,
                    },
                    stop: bench_time {
                        seconds: 0,
                        nanoseconds: 0,
                    },
                };
                let mut _t1: bench_time = bench_time {
                    seconds: 0,
                    nanoseconds: 0,
                };
                let mut _t2: bench_time = bench_time {
                    seconds: 0,
                    nanoseconds: 0,
                };
                max_iterations = 1 as ::core::ffi::c_uint;
                bench_start2(&raw mut _bench);
                let mut s_0: *mut i8 =
                    xkb_keymap_get_as_string2(keymap, keymap_output_format, serialize_flags);
                free(s_0 as *mut ::core::ffi::c_void);
                bench_stop2(&raw mut _bench);
                bench_elapsed(&raw mut _bench, &raw mut _t1);
                loop {
                    bench_start2(&raw mut _bench);
                    let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    while k < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                        let mut s_1: *mut i8 = xkb_keymap_get_as_string2(
                            keymap,
                            keymap_output_format,
                            serialize_flags,
                        );
                        free(s_1 as *mut ::core::ffi::c_void);
                        k = k.wrapping_add(1);
                    }
                    bench_stop2(&raw mut _bench);
                    bench_elapsed(&raw mut _bench, &raw mut _t2);
                    predictPerturbed(&raw mut _t1, &raw mut _t2, &raw mut est);
                    if est.stdev
                        < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            > stdev * est.elapsed as ::core::ffi::c_double
                        {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            stdev * est.elapsed as ::core::ffi::c_double
                        }) as ::core::ffi::c_longlong
                    {
                        est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                        est.stdev /= max_iterations as ::core::ffi::c_longlong;
                        elapsed = _t2;
                        max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                        break;
                    } else {
                        max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                        _t1 = _t2;
                    }
                }
                bench_stop2(&raw mut bench);
            }
            fflush(stdout);
            dup2(stdout_old, STDOUT_FILENO);
            close(stdout_old);
            fflush(stderr);
            dup2(stderr_old, STDERR_FILENO);
            close(stderr_old);
            xkb_keymap_unref(keymap);
            total_elapsed = bench_time {
                seconds: 0,
                nanoseconds: 0,
            };
            bench_elapsed(&raw mut bench, &raw mut total_elapsed);
            if explicit_iterations {
                fprintf(
                    stderr,
                    b"mean: %lld \xC2\xB5s; compiled %u keymaps in %ld.%06lds\n\0".as_ptr()
                        as *const i8,
                    est.elapsed / 1000 as ::core::ffi::c_longlong,
                    max_iterations,
                    total_elapsed.seconds,
                    total_elapsed.nanoseconds / 1000 as ::core::ffi::c_long,
                );
            } else {
                fprintf(
                    stderr,
                    b"mean: %lld \xC2\xB5s; stdev: %Lf%% (target: %f%%); last run: compiled %u keymaps in %ld.%06lds; total time: %ld.%06lds\n\0"
                        .as_ptr() as *const i8,
                    est.elapsed / 1000 as ::core::ffi::c_longlong,
                    (est.stdev as f64) * (100.0 as f64)
                        / (est.elapsed as f64),
                    stdev * 100 as ::core::ffi::c_int as ::core::ffi::c_double,
                    max_iterations,
                    elapsed.seconds,
                    elapsed.nanoseconds / 1000 as ::core::ffi::c_long,
                    total_elapsed.seconds,
                    total_elapsed.nanoseconds / 1000 as ::core::ffi::c_long,
                );
            }
        }
        xkb_context_unref(context);
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
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut i8,
        ) as i32)
    }
}
