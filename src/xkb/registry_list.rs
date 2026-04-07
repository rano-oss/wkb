pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct option {
        pub name: *const ::core::ffi::c_char,
        pub has_arg: ::core::ffi::c_int,
        pub flag: *mut ::core::ffi::c_int,
        pub val: ::core::ffi::c_int,
    }
    pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        pub fn getopt_long(
            ___argc: ::core::ffi::c_int,
            ___argv: *const *mut ::core::ffi::c_char,
            __shortopts: *const ::core::ffi::c_char,
            __longopts: *const option,
            __longind: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
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
pub mod xkbregistry_h {
    pub type rxkb_context_flags = ::core::ffi::c_uint;
    pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
    pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
    pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
    pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;
    pub type rxkb_log_level = ::core::ffi::c_uint;
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
        pub fn rxkb_context_parse(
            ctx: *mut rxkb_context,
            ruleset: *const ::core::ffi::c_char,
        ) -> bool;
        pub fn rxkb_context_unref(ctx: *mut rxkb_context) -> *mut rxkb_context;
        pub fn rxkb_context_include_path_append(
            ctx: *mut rxkb_context,
            path: *const ::core::ffi::c_char,
        ) -> bool;
        pub fn rxkb_context_include_path_append_default(ctx: *mut rxkb_context) -> bool;
        pub fn rxkb_model_first(ctx: *mut rxkb_context) -> *mut rxkb_model;
        pub fn rxkb_model_next(m: *mut rxkb_model) -> *mut rxkb_model;
        pub fn rxkb_model_get_name(m: *mut rxkb_model) -> *const ::core::ffi::c_char;
        pub fn rxkb_model_get_description(m: *mut rxkb_model) -> *const ::core::ffi::c_char;
        pub fn rxkb_model_get_vendor(m: *mut rxkb_model) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_first(ctx: *mut rxkb_context) -> *mut rxkb_layout;
        pub fn rxkb_layout_next(l: *mut rxkb_layout) -> *mut rxkb_layout;
        pub fn rxkb_layout_get_name(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_variant(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_brief(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_description(l: *mut rxkb_layout) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_group_first(ctx: *mut rxkb_context) -> *mut rxkb_option_group;
        pub fn rxkb_option_group_next(g: *mut rxkb_option_group) -> *mut rxkb_option_group;
        pub fn rxkb_option_group_get_name(m: *mut rxkb_option_group) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_group_get_description(
            m: *mut rxkb_option_group,
        ) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_group_allows_multiple(g: *mut rxkb_option_group) -> bool;
        pub fn rxkb_option_first(group: *mut rxkb_option_group) -> *mut rxkb_option;
        pub fn rxkb_option_next(o: *mut rxkb_option) -> *mut rxkb_option;
        pub fn rxkb_option_get_name(o: *mut rxkb_option) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_get_brief(o: *mut rxkb_option) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_get_description(o: *mut rxkb_option) -> *const ::core::ffi::c_char;
        pub fn rxkb_option_is_layout_specific(o: *mut rxkb_option) -> bool;
        pub fn rxkb_iso639_code_get_code(
            iso639: *mut rxkb_iso639_code,
        ) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_iso639_first(layout: *mut rxkb_layout) -> *mut rxkb_iso639_code;
        pub fn rxkb_iso639_code_next(iso639: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code;
        pub fn rxkb_iso3166_code_get_code(
            iso3166: *mut rxkb_iso3166_code,
        ) -> *const ::core::ffi::c_char;
        pub fn rxkb_layout_get_iso3166_first(layout: *mut rxkb_layout) -> *mut rxkb_iso3166_code;
        pub fn rxkb_iso3166_code_next(iso3166: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code;
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stdout: *mut FILE;
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    }
}
pub mod include_locale_h {
    pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
    use super::locale_h::__LC_ALL;
    extern "C" {
        pub fn setlocale(
            __category: ::core::ffi::c_int,
            __locale: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
    }
}
pub mod config_h {
    pub const DEFAULT_XKB_RULES: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"evdev\0") };
    pub const EXIT_INVALID_USAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
}
pub mod assert_h {
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 23] = unsafe {
        ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"int main(int, char **)\0")
    };
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod getopt_core_h {
    extern "C" {
        pub static mut optarg: *mut ::core::ffi::c_char;
        pub static mut optind: ::core::ffi::c_int;
    }
}
pub mod locale_h {
    pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdlib_h {
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;
pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
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
unsafe extern "C" fn usage(mut progname: *const ::core::ffi::c_char, mut fp: *mut FILE) {
    unsafe {
        fprintf(
            fp,
            b"Usage: %s [OPTIONS] [/path/to/xkb_base_directory [/path2]...]\n\nOptions:\n  --verbose, -v .......... Increase verbosity, use multiple times for debugging output\n  --ruleset=foo .......... Load the 'foo' ruleset\n  --skip-default-paths ... Do not load the default XKB paths\n  --load-exotic .......... Load the exotic (extra) rulesets\n  --help ................. Print this help and exit\n\nTrailing arguments are treated as XKB base directory installations.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            progname,
        );
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut ctx: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut m: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        let mut l: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut g: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut flags: rxkb_context_flags = RXKB_CONTEXT_NO_FLAGS;
        let mut load_defaults: bool = true_0 != 0;
        let mut verbosity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ruleset: *const ::core::ffi::c_char = DEFAULT_XKB_RULES.as_ptr();
        static mut opts: [option; 6] = [
            option {
                name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'h' as i32,
            },
            option {
                name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'v' as i32,
            },
            option {
                name: b"load-exotic\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'e' as i32,
            },
            option {
                name: b"skip-default-paths\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'd' as i32,
            },
            option {
                name: b"ruleset\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'r' as i32,
            },
            option {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                has_arg: 0 as ::core::ffi::c_int,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 0 as ::core::ffi::c_int,
            },
        ];
        setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
        loop {
            let mut c: ::core::ffi::c_int = 0;
            let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            c = getopt_long(
                argc,
                argv,
                b"hev\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const opts as *const option,
                &raw mut option_index,
            );
            if c == -1 as ::core::ffi::c_int {
                break;
            }
            match c {
                104 => {
                    usage(*argv.offset(0 as ::core::ffi::c_int as isize), stdout);
                    return 0 as ::core::ffi::c_int;
                }
                100 => {
                    load_defaults = false_0 != 0;
                }
                101 => {
                    flags = (flags as ::core::ffi::c_uint
                        | RXKB_CONTEXT_LOAD_EXOTIC_RULES as ::core::ffi::c_int
                            as ::core::ffi::c_uint)
                        as rxkb_context_flags;
                }
                114 => {
                    ruleset = optarg;
                }
                118 => {
                    verbosity += 1;
                }
                _ => {
                    usage(*argv.offset(0 as ::core::ffi::c_int as isize), stderr);
                    return EXIT_INVALID_USAGE;
                }
            }
        }
        if optind < argc {
            flags = (flags as ::core::ffi::c_uint
                | RXKB_CONTEXT_NO_DEFAULT_INCLUDES as ::core::ffi::c_int as ::core::ffi::c_uint)
                as rxkb_context_flags;
        }
        ctx = rxkb_context_new(flags);
        if ctx.is_null() {
            fprintf(
                stderr,
                b"Failed to create a registry context\n\0".as_ptr() as *const ::core::ffi::c_char,
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
                let mut i: ::core::ffi::c_int = optind;
                loop {
                    if !(i < argc) {
                        c2rust_current_block = 1118134448028020070;
                        break;
                    }
                    if !rxkb_context_include_path_append(ctx, *argv.offset(i as isize)) {
                        fprintf(
                            stderr,
                            b"Failed to append include path '%s'\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
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
                                    b"Failed to include default paths.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
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
                            b"Failed to parse XKB descriptions.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        printf(b"models:\n\0".as_ptr() as *const ::core::ffi::c_char);
                        m = rxkb_model_first(ctx);
                        if !m.is_null() {
                        } else {
                            __assert_fail(
                                b"m\0".as_ptr() as *const ::core::ffi::c_char,
                                b"../tools/registry-list.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                132 as ::core::ffi::c_uint,
                                __ASSERT_FUNCTION.as_ptr(),
                            );
                        };
                        while !m.is_null() {
                            let mut vendor: *const ::core::ffi::c_char = rxkb_model_get_vendor(m);
                            printf(
                                b"- name: %s\n  vendor: %s\n  description: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                rxkb_model_get_name(m),
                                if !vendor.is_null() {
                                    vendor
                                } else {
                                    b"''\0".as_ptr() as *const ::core::ffi::c_char
                                },
                                rxkb_model_get_description(m),
                            );
                            m = rxkb_model_next(m);
                        }
                        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                        printf(b"layouts:\n\0".as_ptr() as *const ::core::ffi::c_char);
                        l = rxkb_layout_first(ctx);
                        if !l.is_null() {
                        } else {
                            __assert_fail(
                                b"l\0".as_ptr() as *const ::core::ffi::c_char,
                                b"../tools/registry-list.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                147 as ::core::ffi::c_uint,
                                __ASSERT_FUNCTION.as_ptr(),
                            );
                        };
                        while !l.is_null() {
                            let mut iso639: *mut rxkb_iso639_code =
                                ::core::ptr::null_mut::<rxkb_iso639_code>();
                            let mut iso3166: *mut rxkb_iso3166_code =
                                ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            let mut variant: *const ::core::ffi::c_char =
                                rxkb_layout_get_variant(l);
                            let mut brief: *const ::core::ffi::c_char = rxkb_layout_get_brief(l);
                            printf(
                                b"- layout: '%s'\n  variant: '%s'\n  brief: '%s'\n  description: %s\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                rxkb_layout_get_name(l),
                                if !variant.is_null() {
                                    variant
                                } else {
                                    b"\0".as_ptr() as *const ::core::ffi::c_char
                                },
                                if !brief.is_null() {
                                    brief
                                } else {
                                    b"''\0".as_ptr() as *const ::core::ffi::c_char
                                },
                                rxkb_layout_get_description(l),
                            );
                            printf(b"  iso639: [\0".as_ptr() as *const ::core::ffi::c_char);
                            iso639 = rxkb_layout_get_iso639_first(l);
                            if !iso639.is_null() {
                                let mut sep: *const ::core::ffi::c_char =
                                    b"\0".as_ptr() as *const ::core::ffi::c_char;
                                while !iso639.is_null() {
                                    printf(
                                        b"%s'%s'\0".as_ptr() as *const ::core::ffi::c_char,
                                        sep,
                                        rxkb_iso639_code_get_code(iso639),
                                    );
                                    iso639 = rxkb_iso639_code_next(iso639);
                                    sep = b", \0".as_ptr() as *const ::core::ffi::c_char;
                                }
                            }
                            printf(b"]\n\0".as_ptr() as *const ::core::ffi::c_char);
                            printf(b"  iso3166: [\0".as_ptr() as *const ::core::ffi::c_char);
                            iso3166 = rxkb_layout_get_iso3166_first(l);
                            if !iso3166.is_null() {
                                let mut sep_0: *const ::core::ffi::c_char =
                                    b"\0".as_ptr() as *const ::core::ffi::c_char;
                                while !iso3166.is_null() {
                                    printf(
                                        b"%s'%s'\0".as_ptr() as *const ::core::ffi::c_char,
                                        sep_0,
                                        rxkb_iso3166_code_get_code(iso3166),
                                    );
                                    iso3166 = rxkb_iso3166_code_next(iso3166);
                                    sep_0 = b", \0".as_ptr() as *const ::core::ffi::c_char;
                                }
                            }
                            printf(b"]\n\0".as_ptr() as *const ::core::ffi::c_char);
                            l = rxkb_layout_next(l);
                        }
                        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                        printf(b"option_groups:\n\0".as_ptr() as *const ::core::ffi::c_char);
                        g = rxkb_option_group_first(ctx);
                        if !g.is_null() {
                        } else {
                            __assert_fail(
                                b"g\0".as_ptr() as *const ::core::ffi::c_char,
                                b"../tools/registry-list.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                190 as ::core::ffi::c_uint,
                                __ASSERT_FUNCTION.as_ptr(),
                            );
                        };
                        while !g.is_null() {
                            let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
                            printf(
                                b"- name: '%s'\n  description: %s\n  allows_multiple: %s\n  options:\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                rxkb_option_group_get_name(g),
                                rxkb_option_group_get_description(g),
                                if rxkb_option_group_allows_multiple(g)
                                    as ::core::ffi::c_int != 0
                                {
                                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                            o = rxkb_option_first(g);
                            if !o.is_null() {
                            } else {
                                __assert_fail(
                                    b"o\0".as_ptr() as *const ::core::ffi::c_char,
                                    b"../tools/registry-list.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    203 as ::core::ffi::c_uint,
                                    __ASSERT_FUNCTION.as_ptr(),
                                );
                            };
                            while !o.is_null() {
                                let mut brief_0: *const ::core::ffi::c_char =
                                    rxkb_option_get_brief(o);
                                printf(
                                    b"  - name: '%s'\n    brief: '%s'\n    description: '%s'\n    layout-specific: %s\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    rxkb_option_get_name(o),
                                    if !brief_0.is_null() {
                                        brief_0
                                    } else {
                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                    rxkb_option_get_description(o),
                                    if rxkb_option_is_layout_specific(o) as ::core::ffi::c_int
                                        != 0
                                    {
                                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"false\0".as_ptr() as *const ::core::ffi::c_char
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
