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
pub mod types_h {
    pub type __uint64_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
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
        pub _mode: i32,
        pub _unused3: i32,
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
pub mod xkbregistry_h {
    pub type rxkb_context_flags = u32;
    pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
    pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
    pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
    pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;
    pub type rxkb_log_level = u32;
    pub const RXKB_LOG_LEVEL_DEBUG: rxkb_log_level = 50;
    pub const RXKB_LOG_LEVEL_INFO: rxkb_log_level = 40;
    pub const RXKB_LOG_LEVEL_WARNING: rxkb_log_level = 30;
    pub const RXKB_LOG_LEVEL_ERROR: rxkb_log_level = 20;
    pub const RXKB_LOG_LEVEL_CRITICAL: rxkb_log_level = 10;
    extern "C" {
        pub type rxkb_context;
        pub type rxkb_model;
        pub type rxkb_layout;
        pub type rxkb_option_group;
        pub type rxkb_option;
        pub type rxkb_iso639_code;
        pub type rxkb_iso3166_code;
        pub fn rxkb_context_new(flags: rxkb_context_flags) -> *mut rxkb_context;
        pub fn rxkb_context_set_log_level(ctx: *mut rxkb_context, level: rxkb_log_level);
        pub fn rxkb_context_parse(ctx: *mut rxkb_context, ruleset: *const i8) -> bool;
        pub fn rxkb_context_unref(ctx: *mut rxkb_context) -> *mut rxkb_context;
        pub fn rxkb_context_include_path_append(ctx: *mut rxkb_context, path: *const i8) -> bool;
        pub fn rxkb_context_include_path_append_default(ctx: *mut rxkb_context) -> bool;
        pub fn rxkb_model_first(ctx: *mut rxkb_context) -> *mut rxkb_model;
        pub fn rxkb_model_next(m: *mut rxkb_model) -> *mut rxkb_model;
        pub fn rxkb_model_get_name(m: *mut rxkb_model) -> *const i8;
        pub fn rxkb_model_get_description(m: *mut rxkb_model) -> *const i8;
        pub fn rxkb_model_get_vendor(m: *mut rxkb_model) -> *const i8;
        pub fn rxkb_layout_first(ctx: *mut rxkb_context) -> *mut rxkb_layout;
        pub fn rxkb_layout_next(l: *mut rxkb_layout) -> *mut rxkb_layout;
        pub fn rxkb_layout_get_name(l: *mut rxkb_layout) -> *const i8;
        pub fn rxkb_layout_get_variant(l: *mut rxkb_layout) -> *const i8;
        pub fn rxkb_layout_get_brief(l: *mut rxkb_layout) -> *const i8;
        pub fn rxkb_layout_get_description(l: *mut rxkb_layout) -> *const i8;
        pub fn rxkb_option_group_first(ctx: *mut rxkb_context) -> *mut rxkb_option_group;
        pub fn rxkb_option_group_next(g: *mut rxkb_option_group) -> *mut rxkb_option_group;
        pub fn rxkb_option_group_get_name(m: *mut rxkb_option_group) -> *const i8;
        pub fn rxkb_option_group_get_description(m: *mut rxkb_option_group) -> *const i8;
        pub fn rxkb_option_group_allows_multiple(g: *mut rxkb_option_group) -> bool;
        pub fn rxkb_option_first(group: *mut rxkb_option_group) -> *mut rxkb_option;
        pub fn rxkb_option_next(o: *mut rxkb_option) -> *mut rxkb_option;
        pub fn rxkb_option_get_name(o: *mut rxkb_option) -> *const i8;
        pub fn rxkb_option_get_brief(o: *mut rxkb_option) -> *const i8;
        pub fn rxkb_option_get_description(o: *mut rxkb_option) -> *const i8;
        pub fn rxkb_option_is_layout_specific(o: *mut rxkb_option) -> bool;
        pub fn rxkb_iso639_code_get_code(iso639: *mut rxkb_iso639_code) -> *const i8;
        pub fn rxkb_layout_get_iso639_first(layout: *mut rxkb_layout) -> *mut rxkb_iso639_code;
        pub fn rxkb_iso639_code_next(iso639: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code;
        pub fn rxkb_iso3166_code_get_code(iso3166: *mut rxkb_iso3166_code) -> *const i8;
        pub fn rxkb_layout_get_iso3166_first(layout: *mut rxkb_layout) -> *mut rxkb_iso3166_code;
        pub fn rxkb_iso3166_code_next(iso3166: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code;
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stdout: *mut FILE;
        pub static mut stderr: *mut FILE;
        pub fn fprintf(__stream: *mut FILE, __format: *const i8, ...) -> i32;
        pub fn printf(__format: *const i8, ...) -> i32;
    }
}
pub mod include_locale_h {
    pub const LC_ALL: i32 = __LC_ALL;
    use super::locale_h::__LC_ALL;
    extern "C" {
        pub fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    }
}
pub mod config_h {
    pub const DEFAULT_XKB_RULES: [i8; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"evdev\0") };
    pub const EXIT_INVALID_USAGE: i32 = 2 as i32;
}
pub mod getopt_core_h {
    extern "C" {
        pub static mut optarg: *mut i8;
        pub static mut optind: i32;
    }
}
pub mod locale_h {
    pub const __LC_ALL: i32 = 6 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdlib_h {
    pub const EXIT_FAILURE: i32 = 1 as i32;
    pub const EXIT_SUCCESS: i32 = 0 as i32;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub use self::__stddef_null_h::NULL;
pub use self::config_h::{DEFAULT_XKB_RULES, EXIT_INVALID_USAGE};
use self::getopt_core_h::{optarg, optind};
pub use self::getopt_ext_h::{getopt_long, no_argument, option, required_argument};
pub use self::include_locale_h::{setlocale, LC_ALL};
pub use self::locale_h::__LC_ALL;
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::{fprintf, printf, stderr, stdout};
pub use self::stdlib_h::{EXIT_FAILURE, EXIT_SUCCESS};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __uint64_t};
pub use self::xkbregistry_h::{
    rxkb_context, rxkb_context_flags, rxkb_context_include_path_append,
    rxkb_context_include_path_append_default, rxkb_context_new, rxkb_context_parse,
    rxkb_context_set_log_level, rxkb_context_unref, rxkb_iso3166_code, rxkb_iso3166_code_get_code,
    rxkb_iso3166_code_next, rxkb_iso639_code, rxkb_iso639_code_get_code, rxkb_iso639_code_next,
    rxkb_layout, rxkb_layout_first, rxkb_layout_get_brief, rxkb_layout_get_description,
    rxkb_layout_get_iso3166_first, rxkb_layout_get_iso639_first, rxkb_layout_get_name,
    rxkb_layout_get_variant, rxkb_layout_next, rxkb_log_level, rxkb_model, rxkb_model_first,
    rxkb_model_get_description, rxkb_model_get_name, rxkb_model_get_vendor, rxkb_model_next,
    rxkb_option, rxkb_option_first, rxkb_option_get_brief, rxkb_option_get_description,
    rxkb_option_get_name, rxkb_option_group, rxkb_option_group_allows_multiple,
    rxkb_option_group_first, rxkb_option_group_get_description, rxkb_option_group_get_name,
    rxkb_option_group_next, rxkb_option_is_layout_specific, rxkb_option_next,
    RXKB_CONTEXT_LOAD_EXOTIC_RULES, RXKB_CONTEXT_NO_DEFAULT_INCLUDES, RXKB_CONTEXT_NO_FLAGS,
    RXKB_CONTEXT_NO_SECURE_GETENV, RXKB_LOG_LEVEL_CRITICAL, RXKB_LOG_LEVEL_DEBUG,
    RXKB_LOG_LEVEL_ERROR, RXKB_LOG_LEVEL_INFO, RXKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
unsafe fn usage(mut progname: *const i8, mut fp: *mut FILE) {
    unsafe {
        fprintf(
            fp,
            b"Usage: %s [OPTIONS] [/path/to/xkb_base_directory [/path2]...]\n\nOptions:\n  --verbose, -v .......... Increase verbosity, use multiple times for debugging output\n  --ruleset=foo .......... Load the 'foo' ruleset\n  --skip-default-paths ... Do not load the default XKB paths\n  --load-exotic .......... Load the exotic (extra) rulesets\n  --help ................. Print this help and exit\n\nTrailing arguments are treated as XKB base directory installations.\n\0"
                .as_ptr() as *const i8,
            progname,
        );
    }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: i32 = 1 as i32;
        let mut ctx: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut m: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        let mut l: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut g: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut flags: rxkb_context_flags = RXKB_CONTEXT_NO_FLAGS;
        let mut load_defaults: bool = true_0 != 0;
        let mut verbosity: i32 = 0 as i32;
        let mut ruleset: *const i8 = DEFAULT_XKB_RULES.as_ptr();
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
                val: 'v' as i32,
            },
            option {
                name: b"load-exotic\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: 'e' as i32,
            },
            option {
                name: b"skip-default-paths\0".as_ptr() as *const i8,
                has_arg: no_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: 'd' as i32,
            },
            option {
                name: b"ruleset\0".as_ptr() as *const i8,
                has_arg: required_argument,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: 'r' as i32,
            },
            option {
                name: ::core::ptr::null::<i8>(),
                has_arg: 0 as i32,
                flag: ::core::ptr::null::<i32>() as *mut i32,
                val: 0 as i32,
            },
        ];
        setlocale(LC_ALL, b"\0".as_ptr() as *const i8);
        loop {
            let mut c: i32 = 0;
            let mut option_index: i32 = 0 as i32;
            c = getopt_long(
                argc,
                argv,
                b"hev\0".as_ptr() as *const i8,
                &raw const opts as *const option,
                &raw mut option_index,
            );
            if c == -1 as i32 {
                break;
            }
            match c {
                104 => {
                    usage(*argv.offset(0 as i32 as isize), stdout);
                    return 0 as i32;
                }
                100 => {
                    load_defaults = false_0 != 0;
                }
                101 => {
                    flags = (flags as u32 | RXKB_CONTEXT_LOAD_EXOTIC_RULES as i32 as u32)
                        as rxkb_context_flags;
                }
                114 => {
                    ruleset = optarg;
                }
                118 => {
                    verbosity += 1;
                }
                _ => {
                    usage(*argv.offset(0 as i32 as isize), stderr);
                    return EXIT_INVALID_USAGE;
                }
            }
        }
        if optind < argc {
            flags = (flags as u32 | RXKB_CONTEXT_NO_DEFAULT_INCLUDES as i32 as u32)
                as rxkb_context_flags;
        }
        ctx = rxkb_context_new(flags);
        if ctx.is_null() {
            fprintf(
                stderr,
                b"Failed to create a registry context\n\0".as_ptr() as *const i8,
            );
            rc = EXIT_FAILURE;
        } else {
            match verbosity {
                0 => {
                    rxkb_context_set_log_level(ctx, RXKB_LOG_LEVEL_ERROR);
                }
                1 => {
                    rxkb_context_set_log_level(ctx, RXKB_LOG_LEVEL_INFO);
                }
                _ => {
                    rxkb_context_set_log_level(ctx, RXKB_LOG_LEVEL_DEBUG);
                }
            }
            if optind < argc {
                let mut i: i32 = optind;
                loop {
                    if !(i < argc) {
                        c2rust_current_block = 1118134448028020070;
                        break;
                    }
                    if !rxkb_context_include_path_append(ctx, *argv.offset(i as isize)) {
                        fprintf(
                            stderr,
                            b"Failed to append include path '%s'\n\0".as_ptr() as *const i8,
                            *argv.offset(i as isize),
                        );
                        c2rust_current_block = 13721776737462220230;
                        break;
                    } else {
                        i += 1;
                    }
                }
                match c2rust_current_block {
                    13721776737462220230 => {}
                    _ => {
                        if load_defaults {
                            if !rxkb_context_include_path_append_default(ctx) {
                                fprintf(
                                    stderr,
                                    b"Failed to include default paths.\n\0".as_ptr() as *const i8,
                                );
                                c2rust_current_block = 13721776737462220230;
                            } else {
                                c2rust_current_block = 4090602189656566074;
                            }
                        } else {
                            c2rust_current_block = 4090602189656566074;
                        }
                    }
                }
            } else {
                c2rust_current_block = 4090602189656566074;
            }
            match c2rust_current_block {
                13721776737462220230 => {}
                _ => {
                    if !rxkb_context_parse(ctx, ruleset) {
                        fprintf(
                            stderr,
                            b"Failed to parse XKB descriptions.\n\0".as_ptr() as *const i8,
                        );
                    } else {
                        printf(b"models:\n\0".as_ptr() as *const i8);
                        m = rxkb_model_first(ctx);
                        while !m.is_null() {
                            let mut vendor: *const i8 = rxkb_model_get_vendor(m);
                            printf(
                                b"- name: %s\n  vendor: %s\n  description: %s\n\0".as_ptr()
                                    as *const i8,
                                rxkb_model_get_name(m),
                                if !vendor.is_null() {
                                    vendor
                                } else {
                                    b"''\0".as_ptr() as *const i8
                                },
                                rxkb_model_get_description(m),
                            );
                            m = rxkb_model_next(m);
                        }
                        printf(b"\n\0".as_ptr() as *const i8);
                        printf(b"layouts:\n\0".as_ptr() as *const i8);
                        l = rxkb_layout_first(ctx);
                        while !l.is_null() {
                            let mut iso639: *mut rxkb_iso639_code =
                                ::core::ptr::null_mut::<rxkb_iso639_code>();
                            let mut iso3166: *mut rxkb_iso3166_code =
                                ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            let mut variant: *const i8 = rxkb_layout_get_variant(l);
                            let mut brief: *const i8 = rxkb_layout_get_brief(l);
                            printf(
                                b"- layout: '%s'\n  variant: '%s'\n  brief: '%s'\n  description: %s\n\0"
                                    .as_ptr() as *const i8,
                                rxkb_layout_get_name(l),
                                if !variant.is_null() {
                                    variant
                                } else {
                                    b"\0".as_ptr() as *const i8
                                },
                                if !brief.is_null() {
                                    brief
                                } else {
                                    b"''\0".as_ptr() as *const i8
                                },
                                rxkb_layout_get_description(l),
                            );
                            printf(b"  iso639: [\0".as_ptr() as *const i8);
                            iso639 = rxkb_layout_get_iso639_first(l);
                            if !iso639.is_null() {
                                let mut sep: *const i8 = b"\0".as_ptr() as *const i8;
                                while !iso639.is_null() {
                                    printf(
                                        b"%s'%s'\0".as_ptr() as *const i8,
                                        sep,
                                        rxkb_iso639_code_get_code(iso639),
                                    );
                                    iso639 = rxkb_iso639_code_next(iso639);
                                    sep = b", \0".as_ptr() as *const i8;
                                }
                            }
                            printf(b"]\n\0".as_ptr() as *const i8);
                            printf(b"  iso3166: [\0".as_ptr() as *const i8);
                            iso3166 = rxkb_layout_get_iso3166_first(l);
                            if !iso3166.is_null() {
                                let mut sep_0: *const i8 = b"\0".as_ptr() as *const i8;
                                while !iso3166.is_null() {
                                    printf(
                                        b"%s'%s'\0".as_ptr() as *const i8,
                                        sep_0,
                                        rxkb_iso3166_code_get_code(iso3166),
                                    );
                                    iso3166 = rxkb_iso3166_code_next(iso3166);
                                    sep_0 = b", \0".as_ptr() as *const i8;
                                }
                            }
                            printf(b"]\n\0".as_ptr() as *const i8);
                            l = rxkb_layout_next(l);
                        }
                        printf(b"\n\0".as_ptr() as *const i8);
                        printf(b"option_groups:\n\0".as_ptr() as *const i8);
                        g = rxkb_option_group_first(ctx);
                        while !g.is_null() {
                            let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
                            printf(
                                b"- name: '%s'\n  description: %s\n  allows_multiple: %s\n  options:\n\0"
                                    .as_ptr() as *const i8,
                                rxkb_option_group_get_name(g),
                                rxkb_option_group_get_description(g),
                                if rxkb_option_group_allows_multiple(g)
                                    as i32 != 0
                                {
                                    b"true\0".as_ptr() as *const i8
                                } else {
                                    b"false\0".as_ptr() as *const i8
                                },
                            );
                            o = rxkb_option_first(g);
                            while !o.is_null() {
                                let mut brief_0: *const i8 = rxkb_option_get_brief(o);
                                printf(
                                    b"  - name: '%s'\n    brief: '%s'\n    description: '%s'\n    layout-specific: %s\n\0"
                                        .as_ptr() as *const i8,
                                    rxkb_option_get_name(o),
                                    if !brief_0.is_null() {
                                        brief_0
                                    } else {
                                        b"\0".as_ptr() as *const i8
                                    },
                                    rxkb_option_get_description(o),
                                    if rxkb_option_is_layout_specific(o) as i32
                                        != 0
                                    {
                                        b"true\0".as_ptr() as *const i8
                                    } else {
                                        b"false\0".as_ptr() as *const i8
                                    },
                                );
                                o = rxkb_option_next(o);
                            }
                            g = rxkb_option_group_next(g);
                        }
                        rc = EXIT_SUCCESS;
                    }
                }
            }
        }
        rxkb_context_unref(ctx);
        return rc;
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
