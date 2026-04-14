// use f128; // f128 is unstable, replaced with f64
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_time {
    pub seconds: i64,
    pub nanoseconds: i64,
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
    pub elapsed: i64,
    pub stdev: i64,
}
extern "C" {
    pub fn bench_start2(bench: *mut bench);
    pub fn bench_stop2(bench: *mut bench);
    pub fn bench_elapsed(bench: *const bench, result: *mut bench_time);
    pub fn predictPerturbed(t1: *const bench_time, t2: *const bench_time, est: *mut estimate);
}
pub const DEFAULT_OUTPUT_KEYMAP_FORMAT: xkb_keymap_format = XKB_KEYMAP_USE_ORIGINAL_FORMAT;
use crate::xkb::shared_types::{xkb_keymap_format, XKB_KEYMAP_USE_ORIGINAL_FORMAT};
extern "C" {
    pub fn xkb_keymap_parse_format(raw: *const i8) -> xkb_keymap_format;
    pub fn xkb_keymap_get_format_label(format: xkb_keymap_format) -> *const i8;
}
use crate::xkb::context::{xkb_context_new, xkb_context_unref};
use crate::xkb::keymap::{
    xkb_keymap_get_as_string2, xkb_keymap_new_from_file, xkb_keymap_new_from_names2,
    xkb_keymap_unref,
};
pub use crate::xkb::shared_types::O_WRONLY;
pub use crate::xkb::shared_types::{no_argument, option, required_argument};
pub use crate::xkb::shared_types::{STDERR_FILENO, STDOUT_FILENO};
pub use crate::xkb::utils::getopt_long;
use crate::xkb::utils::open;
use crate::xkb::utils::optarg;
pub use crate::xkb::utils::{close, dup, dup2};
use libc::{
    atof, atoi, exit, fclose, fflush, fopen, free, perror, EXIT_FAILURE, EXIT_SUCCESS, FILE,
};
extern "C" {
    pub static stderr: *mut libc::FILE;
    pub static stdout: *mut libc::FILE;
}
use crate::xkb::shared_types::{
    DEFAULT_XKB_LAYOUT, DEFAULT_XKB_MODEL, DEFAULT_XKB_OPTIONS, DEFAULT_XKB_RULES,
    DEFAULT_XKB_VARIANT,
};
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
pub type options = u32;
pub const DEFAULT_ITERATIONS: i32 = 3000 as i32;
pub const DEFAULT_STDEV: ::core::ffi::c_double = 0.05f64;
unsafe fn usage(mut fp: *mut FILE, mut argv: *mut *mut i8) {
    unsafe {
        let use_stderr = fp == stderr;
        let msg = format!(
            "Usage: {} [OPTIONS]\n\nBenchmark compilation of the given RMLVO\n\nOptions:\n --help\n    Print this help and exit\n --iter\n    Exact number of iterations to run\n --stdev\n    Minimal relative standard deviation (percentage) to reach.\n    (default: {})\nNote: --iter and --stdev are mutually exclusive.\n\nXKB-specific options:\n --input-format <format>\n    The keymap format to use for parsing (default: '{}')\n --output-format <format>\n    The keymap format to use for serializing (default: same as input)\n --pretty\n    Enable pretty-printing in keymap serialization\n --keep-unused\n    Keep unused bits in keymap serialization\n --explicit-values\n    Force serializing explicit values\n --keymap\n    Load the corresponding XKB file, ignore RMLVO options.\n --rules <rules>\n    The XKB ruleset (default: '{}')\n --model <model>\n    The XKB model (default: '{}')\n --layout <layout>\n    The XKB layout (default: '{}')\n --variant <variant>\n    The XKB layout variant (default: '{}')\n --options <options>\n    The XKB options (default: '{}')\n\n",
            crate::xkb::utils::CStrDisplay(*argv.offset(0 as i32 as isize)),
            DEFAULT_STDEV * 100 as i32 as ::core::ffi::c_double,
            crate::xkb::utils::CStrDisplay(xkb_keymap_get_format_label(XKB_KEYMAP_FORMAT_TEXT_V2)),
            crate::xkb::utils::CStrDisplay(DEFAULT_XKB_RULES.as_ptr()),
            crate::xkb::utils::CStrDisplay(DEFAULT_XKB_MODEL.as_ptr()),
            crate::xkb::utils::CStrDisplay(DEFAULT_XKB_LAYOUT.as_ptr()),
            crate::xkb::utils::CStrDisplay(if !DEFAULT_XKB_VARIANT.is_null() {
                std::ptr::null()
            } else {
                b"<none>\0".as_ptr() as *const i8
            }),
            crate::xkb::utils::CStrDisplay(if !DEFAULT_XKB_OPTIONS.is_null() {
                std::ptr::null()
            } else {
                b"<none>\0".as_ptr() as *const i8
            }),
        );
        if use_stderr {
            eprint!("{}", msg);
        } else {
            print!("{}", msg);
        }
    }
}
unsafe fn load_keymap(
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
                eprintln!(
                    "ERROR: cannot open file: {}",
                    crate::xkb::utils::CStrDisplay(keymap_path),
                );
                return std::ptr::null_mut();
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
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut stdout_old: i32 = 0;
        let mut stdout_new: i32 = 0;
        let mut stderr_old: i32 = 0;
        let mut stderr_new: i32 = 0;
        let mut total_elapsed: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut context: *mut xkb_context = std::ptr::null_mut();
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
        let mut explicit_iterations: bool = false;
        let mut ret: i32 = 0 as i32;
        let mut keymap_path: *mut i8 = std::ptr::null_mut();
        let mut rmlvo: xkb_rule_names = xkb_rule_names {
            rules: DEFAULT_XKB_RULES.as_ptr(),
            model: DEFAULT_XKB_MODEL.as_ptr(),
            layout: std::ptr::null(),
            variant: std::ptr::null(),
            options: std::ptr::null(),
        };
        let mut max_iterations: u32 = DEFAULT_ITERATIONS as u32;
        let mut stdev: ::core::ffi::c_double = DEFAULT_STDEV;
        static mut opts: [option; 15] = [
            option {
                name: b"help\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: 'h' as i32,
            },
            option {
                name: b"input-format\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_KEYMAP_INPUT_FORMAT as i32,
            },
            option {
                name: b"output-format\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_KEYMAP_OUTPUT_FORMAT as i32,
            },
            option {
                name: b"pretty\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: OPT_KEYMAP_PRETTY as i32,
            },
            option {
                name: b"keep-unused\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: OPT_KEYMAP_KEEP_UNUSED as i32,
            },
            option {
                name: b"explicit-values\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: std::ptr::null_mut(),
                val: OPT_KEYMAP_EXPLICIT_VALUES as i32,
            },
            option {
                name: b"keymap\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_KEYMAP as i32,
            },
            option {
                name: b"rules\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_RULES as i32,
            },
            option {
                name: b"model\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_MODEL as i32,
            },
            option {
                name: b"layout\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_LAYOUT as i32,
            },
            option {
                name: b"variant\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_VARIANT as i32,
            },
            option {
                name: b"options\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_OPTION as i32,
            },
            option {
                name: b"iter\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_ITERATIONS as i32,
            },
            option {
                name: b"stdev\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: std::ptr::null_mut(),
                val: OPT_STDEV as i32,
            },
            option {
                name: std::ptr::null(),
                has_arg: 0 as i32,
                flag: std::ptr::null_mut(),
                val: 0 as i32,
            },
        ];
        loop {
            let mut c: i32 = 0;
            let mut option_index: i32 = 0 as i32;
            c = getopt_long(
                argc,
                argv,
                b"h\0".as_ptr() as *const i8,
                &raw mut opts as *mut option,
                &raw mut option_index,
            );
            if c == -1 as i32 {
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
                        eprintln!(
                            "ERROR: invalid --input-format: \"{}\"",
                            crate::xkb::utils::CStrDisplay(optarg),
                        );
                        usage(stderr, argv);
                        exit(2);
                    }
                }
                1 => {
                    keymap_output_format = xkb_keymap_parse_format(optarg);
                    if keymap_output_format as u64 == 0 {
                        eprintln!(
                            "ERROR: invalid --output-format: \"{}\"",
                            crate::xkb::utils::CStrDisplay(optarg),
                        );
                        usage(stderr, argv);
                        exit(2);
                    }
                }
                2 => {
                    serialize_flags = (serialize_flags as u32 | XKB_KEYMAP_SERIALIZE_PRETTY as u32)
                        as xkb_keymap_serialize_flags;
                }
                3 => {
                    serialize_flags = (serialize_flags as u32
                        | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as u32)
                        as xkb_keymap_serialize_flags;
                }
                4 => {
                    serialize_flags = (serialize_flags as u32
                        | XKB_KEYMAP_SERIALIZE_EXPLICIT as u32)
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
                    if max_iterations == 0 as u32 {
                        usage(stderr, argv);
                        exit(2);
                    }
                    let max_iterations_raw: i32 = atoi(optarg) as i32;
                    if max_iterations_raw <= 0 as i32 {
                        max_iterations = DEFAULT_ITERATIONS as u32;
                    } else {
                        max_iterations = max_iterations_raw as u32;
                    }
                    explicit_iterations = true;
                }
                12 => {
                    if explicit_iterations {
                        usage(stderr, argv);
                        exit(2);
                    }
                    stdev = atof(optarg) / 100 as i32 as ::core::ffi::c_double;
                    if stdev <= 0 as i32 as ::core::ffi::c_double {
                        stdev = DEFAULT_STDEV;
                    }
                    max_iterations = 0 as u32;
                }
                _ => {
                    usage(stderr, argv);
                    exit(2);
                }
            }
        }
        if rmlvo.layout.is_null() || *rmlvo.layout == 0 {
            if !rmlvo.variant.is_null() && *rmlvo.variant as i32 != 0 {
                eprintln!("Error: a variant requires a layout");
                return 2;
            }
            rmlvo.layout = DEFAULT_XKB_LAYOUT.as_ptr();
            rmlvo.variant = std::ptr::null();
        }
        context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if context.is_null() {
            exit(1 as i32);
        }
        let mut keymap: *mut xkb_keymap = load_keymap(
            context,
            keymap_path,
            &raw mut rmlvo,
            keymap_input_format,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        );
        if keymap.is_null() {
            eprintln!("ERROR: Cannot compile keymap.");
            ret = EXIT_FAILURE;
        } else {
            fflush(stdout);
            stdout_old = dup(STDOUT_FILENO);
            stdout_new = open(b"/dev/null\0".as_ptr() as *const i8, O_WRONLY);
            if stdout_old == -1 as i32
                || stdout_new == -1 as i32
                || dup2(stdout_new, STDOUT_FILENO) == -1 as i32
            {
                perror(b"Stdout error\0".as_ptr() as *const i8);
                exit(EXIT_FAILURE);
            }
            close(stdout_new);
            fflush(stderr);
            stderr_old = dup(STDERR_FILENO);
            stderr_new = open(b"/dev/null\0".as_ptr() as *const i8, O_WRONLY);
            if stderr_old == -1 as i32
                || stderr_new == -1 as i32
                || dup2(stderr_new, STDERR_FILENO) == -1 as i32
            {
                perror(b"Stderr error\0".as_ptr() as *const i8);
                exit(EXIT_FAILURE);
            }
            close(stderr_new);
            if explicit_iterations {
                stdev = 0 as i32 as ::core::ffi::c_double;
                bench_start2(&raw mut bench);
                let mut i: u32 = 0 as u32;
                while i < max_iterations {
                    let mut s: *mut i8 =
                        xkb_keymap_get_as_string2(keymap, keymap_output_format, serialize_flags);
                    free(s as *mut ::core::ffi::c_void);
                    i = i.wrapping_add(1);
                }
                bench_stop2(&raw mut bench);
                bench_elapsed(&raw mut bench, &raw mut elapsed);
                est.elapsed = ((elapsed.nanoseconds + 1000000000 as i64 * elapsed.seconds)
                    / max_iterations as i64) as i64;
                est.stdev = 0 as i64;
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
                max_iterations = 1 as u32;
                bench_start2(&raw mut _bench);
                let mut s_0: *mut i8 =
                    xkb_keymap_get_as_string2(keymap, keymap_output_format, serialize_flags);
                free(s_0 as *mut ::core::ffi::c_void);
                bench_stop2(&raw mut _bench);
                bench_elapsed(&raw mut _bench, &raw mut _t1);
                loop {
                    bench_start2(&raw mut _bench);
                    let mut k: u32 = 0 as u32;
                    while k < (2 as u32).wrapping_mul(max_iterations) {
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
                        < (if 0 as i32 as ::core::ffi::c_double
                            > stdev * est.elapsed as ::core::ffi::c_double
                        {
                            0 as i32 as ::core::ffi::c_double
                        } else {
                            stdev * est.elapsed as ::core::ffi::c_double
                        }) as i64
                    {
                        est.elapsed /= max_iterations as i64;
                        est.stdev /= max_iterations as i64;
                        elapsed = _t2;
                        max_iterations = max_iterations.wrapping_mul(2 as u32);
                        break;
                    } else {
                        max_iterations = max_iterations.wrapping_mul(2 as u32);
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
                eprintln!(
                    "mean: {} \u{00B5}s; compiled {} keymaps in {}.{:06}s",
                    est.elapsed / 1000 as i64,
                    max_iterations,
                    total_elapsed.seconds,
                    total_elapsed.nanoseconds / 1000 as i64,
                );
            } else {
                eprintln!(
                    "mean: {} \u{00B5}s; stdev: {}% (target: {}%); last run: compiled {} keymaps in {}.{:06}s; total time: {}.{:06}s",
                    est.elapsed / 1000 as i64,
                    (est.stdev as f64) * (100.0 as f64)
                        / (est.elapsed as f64),
                    stdev * 100 as i32 as ::core::ffi::c_double,
                    max_iterations,
                    elapsed.seconds,
                    elapsed.nanoseconds / 1000 as i64,
                    total_elapsed.seconds,
                    total_elapsed.nanoseconds / 1000 as i64,
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
            (args_ptrs.len() - 1) as i32,
            args_ptrs.as_mut_ptr() as *mut *mut i8,
        ) as i32)
    }
}
use crate::xkb::shared_types::*;
