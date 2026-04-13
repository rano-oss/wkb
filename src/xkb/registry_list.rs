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
    pub use crate::xkb::registry::{
        rxkb_context, rxkb_context_include_path_append, rxkb_context_include_path_append_default,
        rxkb_context_new, rxkb_context_parse, rxkb_context_set_log_level, rxkb_context_unref,
        rxkb_iso3166_code, rxkb_iso3166_code_get_code, rxkb_iso3166_code_next, rxkb_iso639_code,
        rxkb_iso639_code_get_code, rxkb_iso639_code_next, rxkb_layout, rxkb_layout_first,
        rxkb_layout_get_brief, rxkb_layout_get_description, rxkb_layout_get_iso3166_first,
        rxkb_layout_get_iso639_first, rxkb_layout_get_name, rxkb_layout_get_variant,
        rxkb_layout_next, rxkb_model, rxkb_model_first, rxkb_model_get_description,
        rxkb_model_get_name, rxkb_model_get_vendor, rxkb_model_next, rxkb_option,
        rxkb_option_first, rxkb_option_get_brief, rxkb_option_get_description,
        rxkb_option_get_name, rxkb_option_group, rxkb_option_group_allows_multiple,
        rxkb_option_group_first, rxkb_option_group_get_description, rxkb_option_group_get_name,
        rxkb_option_group_next, rxkb_option_is_layout_specific, rxkb_option_next,
    };
}
use crate::xkb::shared_types::*;
use crate::xkb::utils::{optarg, optind};
pub use crate::xkb::utils::getopt_long;
pub use crate::xkb::shared_types::{no_argument, option, required_argument};
pub use crate::xkb::utils::setlocale;
pub use crate::xkb::shared_types::LC_ALL;
pub use crate::xkb::shared_types::__LC_ALL;
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
use crate::xkb::shared_types::DEFAULT_XKB_RULES;
use libc::{EXIT_FAILURE, EXIT_SUCCESS, FILE};
extern "C" {
    pub static stderr: *mut libc::FILE;
    pub static stdout: *mut libc::FILE;
}
unsafe fn usage(mut progname: *const i8, mut fp: *mut FILE) {
    unsafe {
        let use_stderr = fp == stderr;
        let msg = format!(
            "Usage: {} [OPTIONS] [/path/to/xkb_base_directory [/path2]...]\n\nOptions:\n  --verbose, -v .......... Increase verbosity, use multiple times for debugging output\n  --ruleset=foo .......... Load the 'foo' ruleset\n  --skip-default-paths ... Do not load the default XKB paths\n  --load-exotic .......... Load the exotic (extra) rulesets\n  --help ................. Print this help and exit\n\nTrailing arguments are treated as XKB base directory installations.\n",
            crate::xkb::utils::CStrDisplay(progname),
        );
        if use_stderr {
            eprint!("{}", msg);
        } else {
            print!("{}", msg);
        }
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
        let mut load_defaults: bool = 1 != 0;
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
                    load_defaults = 0 != 0;
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
                    return 2;
                }
            }
        }
        if optind < argc {
            flags = (flags as u32 | RXKB_CONTEXT_NO_DEFAULT_INCLUDES as i32 as u32)
                as rxkb_context_flags;
        }
        ctx = rxkb_context_new(flags);
        if ctx.is_null() {
            eprintln!("Failed to create a registry context");
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
                        eprintln!(
                            "Failed to append include path '{}'",
                            crate::xkb::utils::CStrDisplay(*argv.offset(i as isize)),
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
                                eprintln!("Failed to include default paths.");
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
                        eprintln!("Failed to parse XKB descriptions.");
                    } else {
                        print!("models:\n");
                        m = rxkb_model_first(ctx);
                        while !m.is_null() {
                            let mut vendor: *const i8 = rxkb_model_get_vendor(m);
                            print!(
                                "- name: {}\n  vendor: {}\n  description: {}\n",
                                crate::xkb::utils::CStrDisplay(rxkb_model_get_name(m)),
                                if !vendor.is_null() {
                                    crate::xkb::utils::CStrDisplay(vendor)
                                } else {
                                    crate::xkb::utils::CStrDisplay(b"''\0".as_ptr() as *const i8)
                                },
                                crate::xkb::utils::CStrDisplay(rxkb_model_get_description(m)),
                            );
                            m = rxkb_model_next(m);
                        }
                        print!("\n");
                        print!("layouts:\n");
                        l = rxkb_layout_first(ctx);
                        while !l.is_null() {
                            let mut iso639: *mut rxkb_iso639_code =
                                ::core::ptr::null_mut::<rxkb_iso639_code>();
                            let mut iso3166: *mut rxkb_iso3166_code =
                                ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            let mut variant: *const i8 = rxkb_layout_get_variant(l);
                            let mut brief: *const i8 = rxkb_layout_get_brief(l);
                            print!(
                                "- layout: '{}'\n  variant: '{}'\n  brief: '{}'\n  description: {}\n",
                                crate::xkb::utils::CStrDisplay(rxkb_layout_get_name(l)),
                                if !variant.is_null() {
                                    crate::xkb::utils::CStrDisplay(variant)
                                } else {
                                    crate::xkb::utils::CStrDisplay(b"\0".as_ptr() as *const i8)
                                },
                                if !brief.is_null() {
                                    crate::xkb::utils::CStrDisplay(brief)
                                } else {
                                    crate::xkb::utils::CStrDisplay(b"''\0".as_ptr() as *const i8)
                                },
                                crate::xkb::utils::CStrDisplay(rxkb_layout_get_description(l)),
                            );
                            print!("  iso639: [");
                            iso639 = rxkb_layout_get_iso639_first(l);
                            if !iso639.is_null() {
                                let mut sep: &str = "";
                                while !iso639.is_null() {
                                    print!(
                                        "{}'{}'",
                                        sep,
                                        crate::xkb::utils::CStrDisplay(rxkb_iso639_code_get_code(
                                            iso639
                                        )),
                                    );
                                    iso639 = rxkb_iso639_code_next(iso639);
                                    sep = ", ";
                                }
                            }
                            print!("]\n");
                            print!("  iso3166: [");
                            iso3166 = rxkb_layout_get_iso3166_first(l);
                            if !iso3166.is_null() {
                                let mut sep_0: &str = "";
                                while !iso3166.is_null() {
                                    print!(
                                        "{}'{}'",
                                        sep_0,
                                        crate::xkb::utils::CStrDisplay(rxkb_iso3166_code_get_code(
                                            iso3166
                                        )),
                                    );
                                    iso3166 = rxkb_iso3166_code_next(iso3166);
                                    sep_0 = ", ";
                                }
                            }
                            print!("]\n");
                            l = rxkb_layout_next(l);
                        }
                        print!("\n");
                        print!("option_groups:\n");
                        g = rxkb_option_group_first(ctx);
                        while !g.is_null() {
                            let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
                            print!(
                                "- name: '{}'\n  description: {}\n  allows_multiple: {}\n  options:\n",
                                crate::xkb::utils::CStrDisplay(rxkb_option_group_get_name(g)),
                                crate::xkb::utils::CStrDisplay(rxkb_option_group_get_description(g)),
                                if rxkb_option_group_allows_multiple(g)
                                    as i32 != 0
                                {
                                    "true"
                                } else {
                                    "false"
                                },
                            );
                            o = rxkb_option_first(g);
                            while !o.is_null() {
                                let mut brief_0: *const i8 = rxkb_option_get_brief(o);
                                print!(
                                    "  - name: '{}'\n    brief: '{}'\n    description: '{}'\n    layout-specific: {}\n",
                                    crate::xkb::utils::CStrDisplay(rxkb_option_get_name(o)),
                                    if !brief_0.is_null() {
                                        crate::xkb::utils::CStrDisplay(brief_0)
                                    } else {
                                        crate::xkb::utils::CStrDisplay(b"\0".as_ptr() as *const i8)
                                    },
                                    crate::xkb::utils::CStrDisplay(rxkb_option_get_description(o)),
                                    if rxkb_option_is_layout_specific(o) as i32
                                        != 0
                                    {
                                        "true"
                                    } else {
                                        "false"
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
