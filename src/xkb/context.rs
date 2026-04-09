pub mod internal {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod types_h {
    pub type __uint64_t = u64;
    pub type __dev_t = ::core::ffi::c_ulong;
    pub type __uid_t = ::core::ffi::c_uint;
    pub type __gid_t = ::core::ffi::c_uint;
    pub type __ino_t = ::core::ffi::c_ulong;
    pub type __ino64_t = ::core::ffi::c_ulong;
    pub type __mode_t = ::core::ffi::c_uint;
    pub type __nlink_t = ::core::ffi::c_ulong;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
    pub type __time_t = ::core::ffi::c_long;
    pub type __blksize_t = ::core::ffi::c_long;
    pub type __blkcnt_t = ::core::ffi::c_long;
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: __ino64_t,
        pub d_off: __off64_t,
        pub d_reclen: ::core::ffi::c_ushort,
        pub d_type: ::core::ffi::c_uchar,
        pub d_name: [i8; 256],
    }
    use super::types_h::{__ino64_t, __off64_t};
}
pub mod include_dirent_h {
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        pub type __dirstream;
        pub fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
        pub fn opendir(__name: *const i8) -> *mut DIR;
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
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
pub mod stdio_h {
    pub type va_list = __gnuc_va_list;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    use super::FILE_h::FILE;

    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(__stream: *mut FILE, __format: *const i8, ...) -> ::core::ffi::c_int;
        pub fn vfprintf(
            __s: *mut FILE,
            __format: *const i8,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
        pub fn vsnprintf(
            __s: *mut i8,
            __maxlen: usize,
            __format: *const i8,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
        pub fn vasprintf(
            __ptr: *mut *mut i8,
            __f: *const i8,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
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
                *const i8,
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
        pub text_buffer: [i8; 2048],
        pub text_next: usize,
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
        pub item: *mut *mut i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut i8,
    }

    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_context_getenv(ctx: *mut xkb_context, name: *const i8) -> *mut i8;
        pub fn xkb_context_init_includes(ctx: *mut xkb_context) -> bool;
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const i8,
            ...
        );
    }
}
pub mod atom_h {
    extern "C" {
        pub type atom_table;
        pub fn atom_table_new() -> *mut atom_table;
        pub fn atom_table_free(table: *mut atom_table);
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_string {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut i8,
    }
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }
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
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_context_flags = ::core::ffi::c_uint;
    pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
    pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
}
pub mod messages_codes_h {
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
pub mod stdlib_h {
    pub type __compar_fn_t = Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;

    extern "C" {
        pub fn strtol(
            __nptr: *const i8,
            __endptr: *mut *mut i8,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn qsort(
            __base: *mut ::core::ffi::c_void,
            __nmemb: usize,
            __size: usize,
            __compar: __compar_fn_t,
        );
    }
}
pub mod string_h {

    extern "C" {
        pub fn strcmp(__s1: *const i8, __s2: *const i8) -> ::core::ffi::c_int;
        pub fn strdup(__s: *const i8) -> *mut i8;
        pub fn strlen(__s: *const i8) -> usize;
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut i8;
    }
}
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        pub fn stat(__file: *const i8, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn istrneq(mut s1: *const i8, mut s2: *const i8, mut len: usize) -> bool {
        unsafe {
            return istrncmp(s1, s2, len) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn strdup_safe(mut s: *const i8) -> *mut i8 {
        unsafe {
            return if !s.is_null() {
                strdup(s)
            } else {
                ::core::ptr::null_mut::<i8>()
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn is_space(mut ch: i8) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int == ' ' as i32
                || ch as ::core::ffi::c_int >= '\t' as i32
                    && ch as ::core::ffi::c_int <= '\r' as i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn check_eaccess(
        mut path: *const i8,
        mut mode: ::core::ffi::c_int,
    ) -> bool {
        unsafe {
            if eaccess(path, mode) != 0 as ::core::ffi::c_int {
                return false_0 != 0;
            }
            return true_0 != 0;
        }
    }
    #[inline]
    pub unsafe extern "C" fn snprintf_safe(
        mut buf: *mut i8,
        mut sz: usize,
        mut format: *const i8,
        mut c2rust_args: ...
    ) -> bool {
        unsafe {
            let mut ap: ::core::ffi::VaList;
            let mut rc: ::core::ffi::c_int = 0;
            ap = c2rust_args.clone();
            rc = vsnprintf(buf, sz, format, ap);
            return rc >= 0 as ::core::ffi::c_int && (rc as usize) < sz;
        }
    }
    #[inline]
    pub unsafe extern "C" fn vasprintf_safe(
        mut fmt: *const i8,
        mut args: ::core::ffi::VaList,
    ) -> *mut i8 {
        unsafe {
            let mut str: *mut i8 = ::core::ptr::null_mut::<i8>();
            let mut len: ::core::ffi::c_int = 0;
            len = vasprintf(&raw mut str, fmt, args);
            if len == -1 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<i8>();
            }
            return str;
        }
    }
    #[inline]
    pub unsafe extern "C" fn asprintf_safe(mut fmt: *const i8, mut c2rust_args: ...) -> *mut i8 {
        unsafe {
            let mut args: ::core::ffi::VaList;
            let mut str: *mut i8 = ::core::ptr::null_mut::<i8>();
            args = c2rust_args.clone();
            str = vasprintf_safe(fmt, args);
            return str;
        }
    }

    use super::stdbool_h::{false_0, true_0};
    use super::stdio_h::{vasprintf, vsnprintf};
    use super::string_h::strdup;
    use super::unistd_h::eaccess;
    extern "C" {
        pub fn istrncmp(a: *const i8, b: *const i8, n: usize) -> ::core::ffi::c_int;
    }
}
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
    }
}
pub mod bits_stat_h {
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod unistd_h {
    pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        pub fn eaccess(__name: *const i8, __type: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod config_h {
    pub const DFLT_XKB_CONFIG_EXTRA_PATH: [i8; 19] =
        unsafe { ::core::mem::transmute::<[u8; 19], [i8; 19]>(*b"/usr/local/etc/xkb\0") };
    pub const DFLT_XKB_CONFIG_ROOT: [i8; 30] = unsafe {
        ::core::mem::transmute::<[u8; 30], [i8; 30]>(*b"/usr/share/xkeyboard-config-2\0")
    };
    pub const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: [i8; 30] = unsafe {
        ::core::mem::transmute::<[u8; 30], [i8; 30]>(*b"/usr/share/xkeyboard-config.d\0")
    };
    pub const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: [i8; 32] = unsafe {
        ::core::mem::transmute::<[u8; 32], [i8; 32]>(*b"/usr/share/xkeyboard-config-2.d\0")
    };
    pub const DFLT_XKB_LEGACY_ROOT: [i8; 19] =
        unsafe { ::core::mem::transmute::<[u8; 19], [i8; 19]>(*b"/usr/share/X11/xkb\0") };
}
pub mod errno_base_h {
    pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::NULL;

use self::atom_h::{atom_table_free, atom_table_new};
pub use self::bits_stat_h::__S_IFMT;
pub use self::config_h::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use self::context_h::{
    xkb_context, xkb_context_getenv, xkb_context_init_includes, xkb_log, C2Rust_Unnamed,
    C2Rust_Unnamed_0,
};
pub use self::darray_h::{darray_next_alloc, darray_size_t, darray_string};
pub use self::dirent_h::dirent;
pub use self::errno_base_h::{EACCES, ENOMEM, ENOTDIR};
use self::errno_h::__errno_location;
pub use self::include_dirent_h::{__dirstream, closedir, opendir, readdir, DIR};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
use self::stat_h::stat;
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdio_h::{fprintf, stderr, va_list, vasprintf, vfprintf, vsnprintf};
pub use self::stdlib_h::{__compar_fn_t, calloc, free, qsort, realloc, strtol};
use self::string_h::{strcmp, strdup, strerror, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino64_t, __ino_t, __mode_t, __nlink_t, __off64_t,
    __off_t, __syscall_slong_t, __time_t, __uid_t, __uint64_t,
};
pub use self::unistd_h::{eaccess, R_OK, X_OK};
pub use self::utils_h::{
    asprintf_safe, check_eaccess, is_space, istrncmp, istrneq, snprintf_safe, strdup_safe,
    vasprintf_safe,
};
pub use self::xkbcommon_h::{
    xkb_context_flags, xkb_log_level, xkb_rule_names, XKB_CONTEXT_NO_DEFAULT_INCLUDES,
    XKB_CONTEXT_NO_ENVIRONMENT_NAMES, XKB_CONTEXT_NO_FLAGS, XKB_CONTEXT_NO_SECURE_GETENV,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING,
};
pub use self::FILE_h::FILE;
unsafe extern "C" fn context_include_path_append(
    mut ctx: *mut xkb_context,
    mut path: *const i8,
) -> ::core::ffi::c_int {
    unsafe {
        let mut stat_buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut err: ::core::ffi::c_int = ENOMEM;
        let mut tmp: *mut i8 = strdup(path);
        if !tmp.is_null() {
            stat_buf = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            err = stat(path, &raw mut stat_buf);
            if err != 0 as ::core::ffi::c_int {
                err = *__errno_location();
            } else if !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
                err = ENOTDIR;
            } else if !check_eaccess(path, R_OK | X_OK) {
                err = EACCES;
            } else {
                (*ctx).includes.size = (*ctx).includes.size.wrapping_add(1 as darray_size_t);
                let mut __need: darray_size_t = (*ctx).includes.size;
                if __need > (*ctx).includes.alloc {
                    (*ctx).includes.alloc = darray_next_alloc(
                        (*ctx).includes.alloc,
                        __need,
                        ::core::mem::size_of::<*mut i8>() as usize,
                    );
                    (*ctx).includes.item = realloc(
                        (*ctx).includes.item as *mut ::core::ffi::c_void,
                        ((*ctx).includes.alloc as usize)
                            .wrapping_mul(::core::mem::size_of::<*mut i8>() as usize),
                    ) as *mut *mut i8;
                }
                let ref mut c2rust_fresh0 = *(*ctx)
                    .includes
                    .item
                    .offset((*ctx).includes.size.wrapping_sub(1 as darray_size_t) as isize);
                *c2rust_fresh0 = tmp;
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_INFO,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Include path added: %s\n\0".as_ptr() as *const i8,
                    tmp,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        if !tmp.is_null() {
            (*ctx).failed_includes.size =
                (*ctx).failed_includes.size.wrapping_add(1 as darray_size_t);
            let mut __need_0: darray_size_t = (*ctx).failed_includes.size;
            if __need_0 > (*ctx).failed_includes.alloc {
                (*ctx).failed_includes.alloc = darray_next_alloc(
                    (*ctx).failed_includes.alloc,
                    __need_0,
                    ::core::mem::size_of::<*mut i8>() as usize,
                );
                (*ctx).failed_includes.item = realloc(
                    (*ctx).failed_includes.item as *mut ::core::ffi::c_void,
                    ((*ctx).failed_includes.alloc as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut i8>() as usize),
                ) as *mut *mut i8;
            }
            let ref mut c2rust_fresh1 = *(*ctx)
                .failed_includes
                .item
                .offset((*ctx).failed_includes.size.wrapping_sub(1 as darray_size_t) as isize);
            *c2rust_fresh1 = tmp;
        }
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_INFO,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"Include path failed: \"%s\" (%s)\n\0".as_ptr() as *const i8,
            path,
            strerror(err),
        );
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_append(
    mut ctx: *mut xkb_context,
    mut path: *const i8,
) -> ::core::ffi::c_int {
    unsafe {
        return if xkb_context_init_includes(ctx) as ::core::ffi::c_int != 0 {
            context_include_path_append(ctx, path)
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_get_extra_path(
    mut ctx: *mut xkb_context,
) -> *const i8 {
    unsafe {
        let extra: *const i8 =
            xkb_context_getenv(ctx, b"XKB_CONFIG_EXTRA_PATH\0".as_ptr() as *const i8);
        return if !extra.is_null() {
            extra
        } else {
            DFLT_XKB_CONFIG_EXTRA_PATH.as_ptr()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_get_unversioned_extensions_path(
    mut ctx: *mut xkb_context,
) -> *const i8 {
    unsafe {
        let mut ext: *const i8 = xkb_context_getenv(
            ctx,
            b"XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const i8,
        );
        return if !ext.is_null() {
            ext
        } else {
            DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.as_ptr()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_get_versioned_extensions_path(
    mut ctx: *mut xkb_context,
) -> *const i8 {
    unsafe {
        let mut ext: *const i8 = xkb_context_getenv(
            ctx,
            b"XKB_CONFIG_VERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const i8,
        );
        return if !ext.is_null() {
            ext
        } else {
            DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.as_ptr()
        };
    }
}
unsafe extern "C" fn compare_str(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return strcmp(*(a as *mut *mut i8), *(b as *mut *mut i8));
    }
}
unsafe extern "C" fn add_direct_subdirectories(
    mut ctx: *mut xkb_context,
    mut path: *const i8,
    mut extensions: *mut darray_string,
    mut versioned_count: darray_size_t,
    mut versioned_path_length: usize,
) -> ::core::ffi::c_int {
    unsafe {
        let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut path_buf: [i8; 4096] = [0; 4096];
        let mut c2rust_current_block: u64;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut err: ::core::ffi::c_int = ENOMEM;
        let mut dir: *mut DIR = ::core::ptr::null_mut::<DIR>();
        let mut stat_buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        err = stat(path, &raw mut stat_buf);
        if err != 0 as ::core::ffi::c_int {
            err = *__errno_location();
        } else if !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
            err = ENOTDIR;
        } else if !check_eaccess(path, R_OK | X_OK) {
            err = EACCES;
        } else {
            dir = opendir(path);
            if dir.is_null() {
                err = EACCES;
            } else {
                entry = ::core::ptr::null_mut::<dirent>();
                path_buf = ::core::mem::transmute::<
                    [u8; 4096],
                    [i8; 4096],
                >(
                    *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                );
                versioned_path_length = versioned_path_length.wrapping_add(1);
                's_62: loop {
                    entry = readdir(dir);
                    if entry.is_null() {
                        c2rust_current_block = 14434620278749266018;
                        break;
                    }
                    let mut name: *const i8 = &raw mut (*entry).d_name as *mut i8;
                    if strcmp(name, b".\0".as_ptr() as *const i8) == 0 as ::core::ffi::c_int
                        || strcmp(name, b"..\0".as_ptr() as *const i8) == 0 as ::core::ffi::c_int
                    {
                        continue;
                    }
                    if !snprintf_safe(
                        &raw mut path_buf as *mut i8,
                        ::core::mem::size_of::<[i8; 4096]>() as usize,
                        b"%s/%s\0".as_ptr() as *const i8,
                        path,
                        name,
                    ) {
                        err = ENOMEM;
                        c2rust_current_block = 9563249396912231495;
                        break;
                    } else {
                        if stat(&raw mut path_buf as *mut i8, &raw mut stat_buf)
                            != 0 as ::core::ffi::c_int
                            || !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                        {
                            continue;
                        }
                        let mut i: darray_size_t = 0 as darray_size_t;
                        while i < versioned_count {
                            let prev_name: *const i8 = (*(*extensions).item.offset(i as isize))
                                .offset(versioned_path_length as isize);
                            if strcmp(name, prev_name) == 0 as ::core::ffi::c_int {
                                continue 's_62;
                            }
                            i = i.wrapping_add(1);
                        }
                        let mut ext_path: *mut i8 = strdup_safe(&raw mut path_buf as *mut i8);
                        if ext_path.is_null() {
                            err = ENOMEM;
                            c2rust_current_block = 9563249396912231495;
                            break;
                        } else {
                            (*extensions).size =
                                (*extensions).size.wrapping_add(1 as darray_size_t);
                            let mut __need: darray_size_t = (*extensions).size;
                            if __need > (*extensions).alloc {
                                (*extensions).alloc = darray_next_alloc(
                                    (*extensions).alloc,
                                    __need,
                                    ::core::mem::size_of::<*mut i8>() as usize,
                                );
                                (*extensions).item = realloc(
                                    (*extensions).item as *mut ::core::ffi::c_void,
                                    ((*extensions).alloc as usize)
                                        .wrapping_mul(::core::mem::size_of::<*mut i8>() as usize),
                                )
                                    as *mut *mut i8;
                            }
                            let ref mut c2rust_fresh2 =
                                *(*extensions)
                                    .item
                                    .offset((*extensions).size.wrapping_sub(1 as darray_size_t)
                                        as isize);
                            *c2rust_fresh2 = ext_path;
                        }
                    }
                }
                match c2rust_current_block {
                    9563249396912231495 => {}
                    _ => {
                        closedir(dir);
                        if (*extensions).size > versioned_count {
                            qsort(
                                (*extensions).item.offset(versioned_count as isize)
                                    as *mut ::core::ffi::c_void,
                                (*extensions).size.wrapping_sub(versioned_count) as usize,
                                ::core::mem::size_of::<*mut i8>() as usize,
                                Some(
                                    compare_str
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                ),
                            );
                            let mut ext_path_0: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
                            if !(*extensions).item.is_null() {
                                ext_path_0 = (*extensions).item.offset(versioned_count as isize)
                                    as *mut *mut i8;
                                while ext_path_0
                                    < (*extensions).item.offset((*extensions).size as isize)
                                        as *mut *mut i8
                                {
                                    ret |= context_include_path_append(ctx, *ext_path_0);
                                    ext_path_0 = ext_path_0.offset(1);
                                }
                            }
                        }
                        return ret;
                    }
                }
            }
        }
        xkb_log(
            ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
            b"Include extensions path failed: %s (%s)\n\0".as_ptr() as *const i8,
            path,
            strerror(err),
        );
        if !dir.is_null() {
            closedir(dir);
        }
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_get_system_path(
    mut ctx: *mut xkb_context,
) -> *const i8 {
    unsafe {
        let root: *const i8 = xkb_context_getenv(ctx, b"XKB_CONFIG_ROOT\0".as_ptr() as *const i8);
        return if !root.is_null() {
            root
        } else {
            DFLT_XKB_CONFIG_ROOT.as_ptr()
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_append_default(
    mut ctx: *mut xkb_context,
) -> ::core::ffi::c_int {
    unsafe {
        let mut user_path: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let home: *const i8 = xkb_context_getenv(ctx, b"HOME\0".as_ptr() as *const i8);
        let xdg: *const i8 = xkb_context_getenv(ctx, b"XDG_CONFIG_HOME\0".as_ptr() as *const i8);
        if !xdg.is_null() {
            user_path = asprintf_safe(b"%s/xkb\0".as_ptr() as *const i8, xdg);
            if !user_path.is_null() {
                ret |= context_include_path_append(ctx, user_path);
                free(user_path as *mut ::core::ffi::c_void);
            }
        } else if !home.is_null() {
            user_path = asprintf_safe(b"%s/.config/xkb\0".as_ptr() as *const i8, home);
            if !user_path.is_null() {
                ret |= context_include_path_append(ctx, user_path);
                free(user_path as *mut ::core::ffi::c_void);
            }
        }
        if !home.is_null() {
            user_path = asprintf_safe(b"%s/.xkb\0".as_ptr() as *const i8, home);
            if !user_path.is_null() {
                ret |= context_include_path_append(ctx, user_path);
                free(user_path as *mut ::core::ffi::c_void);
            }
        }
        let extra: *const i8 = xkb_context_include_path_get_extra_path(ctx) as *const i8;
        ret |= context_include_path_append(ctx, extra);
        let mut extensions: darray_string = darray_string {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<*mut i8>(),
        };
        let mut extensions_path: *const i8 =
            xkb_context_include_path_get_versioned_extensions_path(ctx);
        let mut versioned_path_length: usize = 0 as usize;
        if !extensions_path.is_null() {
            ret |= add_direct_subdirectories(
                ctx,
                extensions_path,
                &raw mut extensions,
                0 as darray_size_t,
                0 as usize,
            );
            versioned_path_length = strlen(extensions_path);
        }
        extensions_path = xkb_context_include_path_get_unversioned_extensions_path(ctx);
        if !extensions_path.is_null() {
            ret |= add_direct_subdirectories(
                ctx,
                extensions_path,
                &raw mut extensions,
                extensions.size,
                versioned_path_length,
            );
        }
        let mut ext_path: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
        if !extensions.item.is_null() {
            ext_path = extensions.item.offset(0 as ::core::ffi::c_int as isize) as *mut *mut i8;
            while ext_path < extensions.item.offset(extensions.size as isize) as *mut *mut i8 {
                free(*ext_path as *mut ::core::ffi::c_void);
                ext_path = ext_path.offset(1);
            }
        }
        free(extensions.item as *mut ::core::ffi::c_void);
        extensions.item = ::core::ptr::null_mut::<*mut i8>();
        extensions.size = 0 as darray_size_t;
        extensions.alloc = 0 as darray_size_t;
        let root: *const i8 = xkb_context_include_path_get_system_path(ctx) as *const i8;
        let has_root: bool = context_include_path_append(ctx, root) != 0;
        ret |= has_root as ::core::ffi::c_int;
        if !has_root
            && *root.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Root include path failed; fallback to \"%s\". The setup is probably misconfigured. Please ensure that \"%s\" is available in the environment.\n\0"
                    .as_ptr() as *const i8,
                b"/usr/share/X11/xkb\0".as_ptr() as *const i8,
                root,
            );
            ret |= context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT.as_ptr());
        }
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_clear(mut ctx: *mut xkb_context) {
    unsafe {
        let mut path: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
        if !(*ctx).includes.item.is_null() {
            path = (*ctx)
                .includes
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut *mut i8;
            while path < (*ctx).includes.item.offset((*ctx).includes.size as isize) as *mut *mut i8
            {
                free(*path as *mut ::core::ffi::c_void);
                path = path.offset(1);
            }
        }
        free((*ctx).includes.item as *mut ::core::ffi::c_void);
        (*ctx).includes.item = ::core::ptr::null_mut::<*mut i8>();
        (*ctx).includes.size = 0 as darray_size_t;
        (*ctx).includes.alloc = 0 as darray_size_t;
        if !(*ctx).failed_includes.item.is_null() {
            path = (*ctx)
                .failed_includes
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut *mut i8;
            while path
                < (*ctx)
                    .failed_includes
                    .item
                    .offset((*ctx).failed_includes.size as isize) as *mut *mut i8
            {
                free(*path as *mut ::core::ffi::c_void);
                path = path.offset(1);
            }
        }
        free((*ctx).failed_includes.item as *mut ::core::ffi::c_void);
        (*ctx).failed_includes.item = ::core::ptr::null_mut::<*mut i8>();
        (*ctx).failed_includes.size = 0 as darray_size_t;
        (*ctx).failed_includes.alloc = 0 as darray_size_t;
        (*ctx).set_pending_default_includes((false_0 != 0) as bool);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_reset_defaults(
    mut ctx: *mut xkb_context,
) -> ::core::ffi::c_int {
    unsafe {
        xkb_context_include_path_clear(ctx);
        return xkb_context_include_path_append_default(ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_num_include_paths(
    mut ctx: *mut xkb_context,
) -> ::core::ffi::c_uint {
    unsafe {
        return if xkb_context_init_includes(ctx) as ::core::ffi::c_int != 0 {
            (*ctx).includes.size as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_include_path_get(
    mut ctx: *mut xkb_context,
    mut idx: ::core::ffi::c_uint,
) -> *const i8 {
    unsafe {
        if idx >= xkb_context_num_include_paths(ctx) {
            return ::core::ptr::null::<i8>();
        }
        return *(*ctx).includes.item.offset(idx as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_ref(mut ctx: *mut xkb_context) -> *mut xkb_context {
    unsafe {
        (*ctx).refcnt += 1;
        return ctx;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_unref(mut ctx: *mut xkb_context) {
    unsafe {
        if ctx.is_null() || {
            (*ctx).refcnt -= 1;
            (*ctx).refcnt > 0 as ::core::ffi::c_int
        } {
            return;
        }
        free((*ctx).x11_atom_cache);
        xkb_context_include_path_clear(ctx);
        atom_table_free((*ctx).atom_table);
        free(ctx as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn log_level_to_prefix(mut level: xkb_log_level) -> *const i8 {
    unsafe {
        match level as ::core::ffi::c_uint {
            50 => return b"xkbcommon: DEBUG: \0".as_ptr() as *const i8,
            40 => return b"xkbcommon: INFO: \0".as_ptr() as *const i8,
            30 => return b"xkbcommon: WARNING: \0".as_ptr() as *const i8,
            20 => return b"xkbcommon: ERROR: \0".as_ptr() as *const i8,
            10 => {
                return b"xkbcommon: CRITICAL: \0".as_ptr() as *const i8;
            }
            _ => return ::core::ptr::null::<i8>(),
        };
    }
}
unsafe extern "C" fn default_log_fn(
    mut ctx: *mut xkb_context,
    mut level: xkb_log_level,
    mut fmt: *const i8,
    mut args: ::core::ffi::VaList,
) {
    unsafe {
        let mut prefix: *const i8 = log_level_to_prefix(level);
        if !prefix.is_null() {
            fprintf(stderr, b"%s\0".as_ptr() as *const i8, prefix);
        }
        vfprintf(stderr, fmt, args);
    }
}
unsafe extern "C" fn log_level(mut level: *const i8) -> xkb_log_level {
    unsafe {
        let mut endptr: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut lvl: xkb_log_level = 0 as xkb_log_level;
        *__errno_location() = 0 as ::core::ffi::c_int;
        lvl = strtol(level, &raw mut endptr, 10 as ::core::ffi::c_int) as xkb_log_level;
        if *__errno_location() == 0 as ::core::ffi::c_int
            && (*endptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
                || is_space(*endptr.offset(0 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    != 0)
        {
            return lvl;
        }
        if istrneq(
            b"crit\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(
            b"err\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_ERROR;
        }
        if istrneq(
            b"warn\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_WARNING;
        }
        if istrneq(
            b"info\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_INFO;
        }
        if istrneq(
            b"debug\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
        ) as ::core::ffi::c_int
            != 0
            || istrneq(
                b"dbg\0".as_ptr() as *const i8,
                level,
                (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
            ) as ::core::ffi::c_int
                != 0
        {
            return XKB_LOG_LEVEL_DEBUG;
        }
        return XKB_LOG_LEVEL_ERROR;
    }
}
unsafe extern "C" fn log_verbosity(mut verbosity: *const i8) -> ::core::ffi::c_int {
    unsafe {
        *__errno_location() = 0 as ::core::ffi::c_int;
        let v: ::core::ffi::c_long = strtol(
            verbosity,
            ::core::ptr::null_mut::<*mut i8>(),
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_long;
        if *__errno_location() == 0 as ::core::ffi::c_int {
            return v as ::core::ffi::c_int;
        }
        return XKB_LOG_VERBOSITY_DEFAULT as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_new(mut flags: xkb_context_flags) -> *mut xkb_context {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        let mut ctx: *mut xkb_context =
            calloc(1 as usize, ::core::mem::size_of::<xkb_context>() as usize) as *mut xkb_context;
        if ctx.is_null() {
            return ::core::ptr::null_mut::<xkb_context>();
        }
        (*ctx).refcnt = 1 as ::core::ffi::c_int;
        (*ctx).log_fn = Some(
            default_log_fn
                as unsafe extern "C" fn(
                    *mut xkb_context,
                    xkb_log_level,
                    *const i8,
                    ::core::ffi::VaList,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut xkb_context,
                    xkb_log_level,
                    *const i8,
                    ::core::ffi::VaList,
                ) -> (),
            >;
        (*ctx).log_level = XKB_LOG_LEVEL_ERROR;
        (*ctx).log_verbosity = XKB_LOG_VERBOSITY_DEFAULT as ::core::ffi::c_int;
        static mut XKB_CONTEXT_FLAGS: xkb_context_flags = (XKB_CONTEXT_NO_DEFAULT_INCLUDES
            as ::core::ffi::c_int
            | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as ::core::ffi::c_int
            | XKB_CONTEXT_NO_SECURE_GETENV as ::core::ffi::c_int)
            as xkb_context_flags;
        if flags as ::core::ffi::c_uint & !(XKB_CONTEXT_FLAGS as ::core::ffi::c_uint) != 0 {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Invalid context flags: 0x%x\n\0".as_ptr() as *const i8,
                flags as ::core::ffi::c_uint & !(XKB_CONTEXT_FLAGS as ::core::ffi::c_uint),
            );
            free(ctx as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xkb_context>();
        }
        (*ctx).set_use_environment_names(
            (flags as ::core::ffi::c_uint
                & XKB_CONTEXT_NO_ENVIRONMENT_NAMES as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0) as bool,
        );
        (*ctx).set_use_secure_getenv(
            (flags as ::core::ffi::c_uint
                & XKB_CONTEXT_NO_SECURE_GETENV as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0) as bool,
        );
        (*ctx).set_pending_default_includes(
            (flags as ::core::ffi::c_uint
                & XKB_CONTEXT_NO_DEFAULT_INCLUDES as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0) as bool,
        );
        (*ctx).includes.item = ::core::ptr::null_mut::<*mut i8>();
        (*ctx).includes.size = 0 as darray_size_t;
        (*ctx).includes.alloc = 0 as darray_size_t;
        (*ctx).failed_includes.item = ::core::ptr::null_mut::<*mut i8>();
        (*ctx).failed_includes.size = 0 as darray_size_t;
        (*ctx).failed_includes.alloc = 0 as darray_size_t;
        env = xkb_context_getenv(ctx, b"XKB_LOG_LEVEL\0".as_ptr() as *const i8);
        if !env.is_null() {
            xkb_context_set_log_level(ctx, log_level(env));
        }
        env = xkb_context_getenv(ctx, b"XKB_LOG_VERBOSITY\0".as_ptr() as *const i8);
        if !env.is_null() {
            xkb_context_set_log_verbosity(ctx, log_verbosity(env));
        }
        (*ctx).atom_table = atom_table_new();
        if (*ctx).atom_table.is_null() {
            xkb_context_unref(ctx);
            return ::core::ptr::null_mut::<xkb_context>();
        }
        (*ctx).x11_atom_cache = NULL;
        return ctx;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_set_log_fn(
    mut ctx: *mut xkb_context,
    mut log_fn: Option<
        unsafe extern "C" fn(*mut xkb_context, xkb_log_level, *const i8, ::core::ffi::VaList) -> (),
    >,
) {
    unsafe {
        (*ctx).log_fn = (if log_fn.is_some() {
            log_fn
                as Option<
                    unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_log_level,
                        *const i8,
                        ::core::ffi::VaList,
                    ) -> (),
                >
        } else {
            Some(
                default_log_fn
                    as unsafe extern "C" fn(
                        *mut xkb_context,
                        xkb_log_level,
                        *const i8,
                        ::core::ffi::VaList,
                    ) -> (),
            )
        })
            as Option<
                unsafe extern "C" fn(
                    *mut xkb_context,
                    xkb_log_level,
                    *const i8,
                    ::core::ffi::VaList,
                ) -> (),
            >;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_get_log_level(mut ctx: *mut xkb_context) -> xkb_log_level {
    unsafe {
        return (*ctx).log_level;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_set_log_level(
    mut ctx: *mut xkb_context,
    mut level: xkb_log_level,
) {
    unsafe {
        (*ctx).log_level = level;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_get_log_verbosity(
    mut ctx: *mut xkb_context,
) -> ::core::ffi::c_int {
    unsafe {
        return (*ctx).log_verbosity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_set_log_verbosity(
    mut ctx: *mut xkb_context,
    mut verbosity: ::core::ffi::c_int,
) {
    unsafe {
        (*ctx).log_verbosity = verbosity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_get_user_data(
    mut ctx: *mut xkb_context,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if !ctx.is_null() {
            return (*ctx).user_data;
        }
        return NULL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_context_set_user_data(
    mut ctx: *mut xkb_context,
    mut user_data: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*ctx).user_data = user_data;
    }
}
