use crate::xkb_logf;
pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
    pub type __builtin_va_list = [__va_list_tag; 1];
}

pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: u64,
        pub st_ino: u64,
        pub st_nlink: u64,
        pub st_mode: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub __pad0: i32,
        pub st_rdev: u64,
        pub st_size: i64,
        pub st_blksize: i64,
        pub st_blocks: i64,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [i64; 3],
    }
    use super::struct_timespec_h::timespec;
}
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: u16,
        pub d_type: ::core::ffi::c_uchar,
        pub d_name: [i8; 256],
    }
}
pub mod include_dirent_h {
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        pub type __dirstream;
        pub fn closedir(__dirp: *mut DIR) -> i32;
        pub fn opendir(__name: *const i8) -> *mut DIR;
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod context_h {
    pub use crate::xkb::context_priv::{xkb_context_getenv, xkb_context_init_includes};
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
}
pub mod atom_h {
    pub use crate::xkb::atom::{atom_table_free, atom_table_new};
    pub use crate::xkb::shared_types::atom_table;
}

pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::{
        xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
        XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    };
    pub type xkb_context_flags = u32;
    pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
    pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
}
pub mod messages_codes_h {
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub type xkb_log_verbosity = i32;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        pub fn stat(__file: *const i8, __buf: *mut stat) -> i32;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn istrneq(mut s1: *const i8, mut s2: *const i8, mut len: usize) -> bool {
        unsafe {
            return istrncmp(s1, s2, len) == 0 as i32;
        }
    }
    #[inline]
    pub unsafe fn strdup_safe(mut s: *const i8) -> *mut i8 {
        unsafe { cstr_dup(s) }
    }
    #[inline]
    pub fn is_space(mut ch: i8) -> bool {
        return ch as i32 == ' ' as i32 || ch as i32 >= '\t' as i32 && ch as i32 <= '\r' as i32;
    }
    #[inline]
    pub unsafe fn check_eaccess(mut path: *const i8, mut mode: i32) -> bool {
        unsafe {
            if eaccess(path, mode) != 0 as i32 {
                return 0 != 0;
            }
            return 1 != 0;
        }
    }

    use super::unistd_h::eaccess;
    use crate::xkb::utils::cstr_dup;
    pub use crate::xkb::utils::istrncmp;
}
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut i32;
    }
}
pub mod bits_stat_h {
    pub const __S_IFMT: i32 = 0o170000 as i32;
}
pub mod unistd_h {
    pub const R_OK: i32 = 4 as i32;
    pub const X_OK: i32 = 1 as i32;
    extern "C" {
        pub fn eaccess(__name: *const i8, __type: i32) -> i32;
    }
}
pub mod errno_base_h {
    pub const ENOMEM: i32 = 12 as i32;
    pub const EACCES: i32 = 13 as i32;
    pub const ENOTDIR: i32 = 20 as i32;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;

use self::atom_h::{atom_table_free, atom_table_new};
pub use self::bits_stat_h::__S_IFMT;
pub use self::context_h::{
    xkb_context, xkb_context_getenv, xkb_context_init_includes, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
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
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::unistd_h::{eaccess, R_OK, X_OK};
pub use self::utils_h::{check_eaccess, is_space, istrncmp, istrneq, strdup_safe};
pub use self::xkbcommon_h::{
    xkb_context_flags, xkb_log_level, xkb_rule_names, XKB_CONTEXT_NO_DEFAULT_INCLUDES,
    XKB_CONTEXT_NO_ENVIRONMENT_NAMES, XKB_CONTEXT_NO_FLAGS, XKB_CONTEXT_NO_SECURE_GETENV,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING,
};
pub use crate::xkb::shared_types::{darray_size_t, darray_string};
use crate::xkb::shared_types::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
use crate::xkb::utils::cstr_dup;
use crate::xkb::utils::{cstr_cmp, cstr_len, darray_append, darray_free};
use libc::{calloc, free, qsort, strtol};
unsafe fn context_include_path_append(mut ctx: *mut xkb_context, mut path: *const i8) -> i32 {
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
        let mut err: i32 = ENOMEM;
        let mut tmp: *mut i8 = cstr_dup(path);
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
            if err != 0 as i32 {
                err = *__errno_location();
            } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
                err = ENOTDIR;
            } else if !check_eaccess(path, R_OK | X_OK) {
                err = EACCES;
            } else {
                darray_append(
                    &mut (*ctx).includes.item,
                    &mut (*ctx).includes.size,
                    &mut (*ctx).includes.alloc,
                    tmp,
                );
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_INFO,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Include path added: {}\n",
                    crate::xkb::utils::CStrDisplay(tmp),
                );
                return 1 as i32;
            }
        }
        if !tmp.is_null() {
            darray_append(
                &mut (*ctx).failed_includes.item,
                &mut (*ctx).failed_includes.size,
                &mut (*ctx).failed_includes.alloc,
                tmp,
            );
        }
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_INFO,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "Include path failed: \"{}\" ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        return 0 as i32;
    }
}
pub unsafe fn xkb_context_include_path_append(
    mut ctx: *mut xkb_context,
    mut path: *const i8,
) -> i32 {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            context_include_path_append(ctx, path)
        } else {
            0 as i32
        };
    }
}
pub unsafe fn xkb_context_include_path_get_extra_path(mut ctx: *mut xkb_context) -> *const i8 {
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

pub unsafe fn xkb_context_include_path_get_unversioned_extensions_path(
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

pub unsafe fn xkb_context_include_path_get_versioned_extensions_path(
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
) -> i32 {
    unsafe {
        return cstr_cmp(*(a as *mut *mut i8), *(b as *mut *mut i8));
    }
}
unsafe fn add_direct_subdirectories(
    mut ctx: *mut xkb_context,
    mut path: *const i8,
    mut extensions: *mut darray_string,
    mut versioned_count: darray_size_t,
    mut versioned_path_length: usize,
) -> i32 {
    unsafe {
        let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut path_buf: [i8; 4096] = [0; 4096];
        let mut c2rust_current_block: u64;
        let mut ret: i32 = 0 as i32;
        let mut err: i32 = ENOMEM;
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
        if err != 0 as i32 {
            err = *__errno_location();
        } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
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
                    if cstr_cmp(name, b".\0".as_ptr() as *const i8) == 0 as i32
                        || cstr_cmp(name, b"..\0".as_ptr() as *const i8) == 0 as i32
                    {
                        continue;
                    }
                    let (_, _trunc) = crate::xkb::utils::snprintf_args(
                        &raw mut path_buf as *mut i8,
                        ::core::mem::size_of::<[i8; 4096]>() as usize,
                        format_args!(
                            "{}/{}",
                            crate::xkb::utils::CStrDisplay(path),
                            crate::xkb::utils::CStrDisplay(name)
                        ),
                    );
                    if _trunc {
                        err = ENOMEM;
                        c2rust_current_block = 9563249396912231495;
                        break;
                    } else {
                        if stat(&raw mut path_buf as *mut i8, &raw mut stat_buf) != 0 as i32
                            || !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32)
                        {
                            continue;
                        }
                        let mut i: darray_size_t = 0 as darray_size_t;
                        while i < versioned_count {
                            let prev_name: *const i8 = (*(*extensions).item.offset(i as isize))
                                .offset(versioned_path_length as isize);
                            if cstr_cmp(name, prev_name) == 0 as i32 {
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
                            darray_append(
                                &mut (*extensions).item,
                                &mut (*extensions).size,
                                &mut (*extensions).alloc,
                                ext_path,
                            );
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
                                            -> i32,
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
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_DEBUG,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "Include extensions path failed: {} ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        if !dir.is_null() {
            closedir(dir);
        }
        return ret;
    }
}
pub unsafe fn xkb_context_include_path_get_system_path(mut ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let root: *const i8 = xkb_context_getenv(ctx, b"XKB_CONFIG_ROOT\0".as_ptr() as *const i8);
        return if !root.is_null() {
            root
        } else {
            DFLT_XKB_CONFIG_ROOT.as_ptr()
        };
    }
}
pub unsafe fn xkb_context_include_path_append_default(mut ctx: *mut xkb_context) -> i32 {
    unsafe {
        let mut ret: i32 = 0 as i32;
        let home: *const i8 = xkb_context_getenv(ctx, b"HOME\0".as_ptr() as *const i8);
        let xdg: *const i8 = xkb_context_getenv(ctx, b"XDG_CONFIG_HOME\0".as_ptr() as *const i8);
        if !xdg.is_null() {
            let user_path =
                std::ffi::CString::new(format!("{}/xkb", crate::xkb::utils::CStrDisplay(xdg)))
                    .unwrap();
            ret |= context_include_path_append(ctx, user_path.as_ptr());
        } else if !home.is_null() {
            let user_path = std::ffi::CString::new(format!(
                "{}/.config/xkb",
                crate::xkb::utils::CStrDisplay(home)
            ))
            .unwrap();
            ret |= context_include_path_append(ctx, user_path.as_ptr());
        }
        if !home.is_null() {
            let user_path =
                std::ffi::CString::new(format!("{}/.xkb", crate::xkb::utils::CStrDisplay(home)))
                    .unwrap();
            ret |= context_include_path_append(ctx, user_path.as_ptr());
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
            versioned_path_length = cstr_len(extensions_path);
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
            ext_path = extensions.item.offset(0 as i32 as isize) as *mut *mut i8;
            while ext_path < extensions.item.offset(extensions.size as isize) as *mut *mut i8 {
                free(*ext_path as *mut ::core::ffi::c_void);
                ext_path = ext_path.offset(1);
            }
        }
        darray_free(
            &mut extensions.item,
            &mut extensions.size,
            &mut extensions.alloc,
        );
        let root: *const i8 = xkb_context_include_path_get_system_path(ctx) as *const i8;
        let has_root: bool = context_include_path_append(ctx, root) != 0;
        ret |= has_root as i32;
        if !has_root && *root.offset(0 as i32 as isize) as i32 != '\0' as i32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                crate::xkb::utils::CStrDisplay(b"/usr/share/X11/xkb\0".as_ptr() as *const i8),
                crate::xkb::utils::CStrDisplay(root),
            );
            ret |= context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT.as_ptr());
        }
        return ret;
    }
}

pub unsafe fn xkb_context_include_path_clear(mut ctx: *mut xkb_context) {
    unsafe {
        let mut path: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
        if !(*ctx).includes.item.is_null() {
            path = (*ctx).includes.item.offset(0 as i32 as isize) as *mut *mut i8;
            while path < (*ctx).includes.item.offset((*ctx).includes.size as isize) as *mut *mut i8
            {
                free(*path as *mut ::core::ffi::c_void);
                path = path.offset(1);
            }
        }
        darray_free(
            &mut (*ctx).includes.item,
            &mut (*ctx).includes.size,
            &mut (*ctx).includes.alloc,
        );
        if !(*ctx).failed_includes.item.is_null() {
            path = (*ctx).failed_includes.item.offset(0 as i32 as isize) as *mut *mut i8;
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
        darray_free(
            &mut (*ctx).failed_includes.item,
            &mut (*ctx).failed_includes.size,
            &mut (*ctx).failed_includes.alloc,
        );
        (*ctx).set_pending_default_includes((0 != 0) as bool);
    }
}

pub unsafe fn xkb_context_include_path_reset_defaults(mut ctx: *mut xkb_context) -> i32 {
    unsafe {
        xkb_context_include_path_clear(ctx);
        return xkb_context_include_path_append_default(ctx);
    }
}
pub unsafe fn xkb_context_num_include_paths(mut ctx: *mut xkb_context) -> u32 {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).includes.size as u32
        } else {
            0 as u32
        };
    }
}
pub unsafe fn xkb_context_include_path_get(mut ctx: *mut xkb_context, mut idx: u32) -> *const i8 {
    unsafe {
        if idx >= xkb_context_num_include_paths(ctx) {
            return ::core::ptr::null::<i8>();
        }
        return *(*ctx).includes.item.offset(idx as isize);
    }
}
pub unsafe fn xkb_context_ref(mut ctx: *mut xkb_context) -> *mut xkb_context {
    unsafe {
        (*ctx).refcnt += 1;
        return ctx;
    }
}
pub unsafe fn xkb_context_unref(mut ctx: *mut xkb_context) {
    unsafe {
        if ctx.is_null() || {
            (*ctx).refcnt -= 1;
            (*ctx).refcnt > 0 as i32
        } {
            return;
        }
        free((*ctx).x11_atom_cache);
        xkb_context_include_path_clear(ctx);
        atom_table_free((*ctx).atom_table);
        free(ctx as *mut ::core::ffi::c_void);
    }
}
unsafe fn log_level_to_prefix(mut level: xkb_log_level) -> *const i8 {
    match level as u32 {
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
unsafe fn default_log_fn(mut ctx: *mut xkb_context, mut level: xkb_log_level, mut msg: *const i8) {
    unsafe {
        let mut prefix: *const i8 = log_level_to_prefix(level);
        if !prefix.is_null() {
            eprint!("{}", crate::xkb::utils::CStrDisplay(prefix));
        }
        eprint!("{}", crate::xkb::utils::CStrDisplay(msg));
    }
}
unsafe fn log_level(mut level: *const i8) -> xkb_log_level {
    unsafe {
        let mut endptr: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut lvl: xkb_log_level = 0 as xkb_log_level;
        *__errno_location() = 0 as i32;
        lvl = strtol(level, &raw mut endptr, 10 as i32) as xkb_log_level;
        if *__errno_location() == 0 as i32
            && (*endptr.offset(0 as i32 as isize) as i32 == '\0' as i32
                || is_space(*endptr.offset(0 as i32 as isize)) as i32 != 0)
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
        ) as i32
            != 0
            || istrneq(
                b"dbg\0".as_ptr() as *const i8,
                level,
                (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
            ) as i32
                != 0
        {
            return XKB_LOG_LEVEL_DEBUG;
        }
        return XKB_LOG_LEVEL_ERROR;
    }
}
unsafe fn log_verbosity(mut verbosity: *const i8) -> i32 {
    unsafe {
        *__errno_location() = 0 as i32;
        let v: i64 = strtol(verbosity, ::core::ptr::null_mut::<*mut i8>(), 10 as i32) as i64;
        if *__errno_location() == 0 as i32 {
            return v as i32;
        }
        return XKB_LOG_VERBOSITY_DEFAULT as i32;
    }
}
pub unsafe fn xkb_context_new(mut flags: xkb_context_flags) -> *mut xkb_context {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        let mut ctx: *mut xkb_context =
            calloc(1 as usize, ::core::mem::size_of::<xkb_context>() as usize) as *mut xkb_context;
        if ctx.is_null() {
            return ::core::ptr::null_mut::<xkb_context>();
        }
        (*ctx).refcnt = 1 as i32;
        (*ctx).log_fn =
            Some(default_log_fn as unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ())
                as Option<unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ()>;
        (*ctx).log_level = XKB_LOG_LEVEL_ERROR;
        (*ctx).log_verbosity = XKB_LOG_VERBOSITY_DEFAULT as i32;
        static mut XKB_CONTEXT_FLAGS: xkb_context_flags = (XKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
            | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32
            | XKB_CONTEXT_NO_SECURE_GETENV as i32)
            as xkb_context_flags;
        if flags as u32 & !(XKB_CONTEXT_FLAGS as u32) != 0 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Invalid context flags: 0x{:x}\n",
                flags as u32 & !(XKB_CONTEXT_FLAGS as u32),
            );
            free(ctx as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xkb_context>();
        }
        (*ctx).set_use_environment_names(
            (flags as u32 & XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32 as u32 == 0) as bool,
        );
        (*ctx).set_use_secure_getenv(
            (flags as u32 & XKB_CONTEXT_NO_SECURE_GETENV as i32 as u32 == 0) as bool,
        );
        (*ctx).set_pending_default_includes(
            (flags as u32 & XKB_CONTEXT_NO_DEFAULT_INCLUDES as i32 as u32 == 0) as bool,
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
        (*ctx).x11_atom_cache = std::ptr::null_mut::<core::ffi::c_void>();
        return ctx;
    }
}

pub unsafe fn xkb_context_set_log_fn(
    mut ctx: *mut xkb_context,
    mut log_fn: Option<unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ()>,
) {
    unsafe {
        (*ctx).log_fn = (if log_fn.is_some() {
            log_fn as Option<unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ()>
        } else {
            Some(default_log_fn as unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ())
        })
            as Option<unsafe fn(*mut xkb_context, xkb_log_level, *const i8) -> ()>;
    }
}

pub unsafe fn xkb_context_get_log_level(mut ctx: *mut xkb_context) -> xkb_log_level {
    unsafe {
        return (*ctx).log_level;
    }
}

pub unsafe fn xkb_context_set_log_level(mut ctx: *mut xkb_context, mut level: xkb_log_level) {
    unsafe {
        (*ctx).log_level = level;
    }
}
pub unsafe fn xkb_context_get_log_verbosity(mut ctx: *mut xkb_context) -> i32 {
    unsafe {
        return (*ctx).log_verbosity;
    }
}

pub unsafe fn xkb_context_set_log_verbosity(mut ctx: *mut xkb_context, mut verbosity: i32) {
    unsafe {
        (*ctx).log_verbosity = verbosity;
    }
}

pub unsafe fn xkb_context_get_user_data(mut ctx: *mut xkb_context) -> *mut ::core::ffi::c_void {
    unsafe {
        if !ctx.is_null() {
            return (*ctx).user_data;
        }
        return std::ptr::null_mut::<core::ffi::c_void>();
    }
}

pub unsafe fn xkb_context_set_user_data(
    mut ctx: *mut xkb_context,
    mut user_data: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*ctx).user_data = user_data;
    }
}
