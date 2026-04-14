use crate::xkb::atom::{atom_table_free, atom_table_new};
use crate::xkb::context_priv::{xkb_context_getenv, xkb_context_init_includes};
use crate::xkb_logf;

pub use crate::xkb::messages::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use crate::xkb::shared_types::dirent;
pub use crate::xkb::shared_types::stat;
pub use crate::xkb::shared_types::timespec;
pub use crate::xkb::shared_types::__S_IFMT;
use crate::xkb::shared_types::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use crate::xkb::shared_types::{EACCES, ENOMEM, ENOTDIR};
pub use crate::xkb::shared_types::{R_OK, X_OK};
use crate::xkb::utils::__errno_location;
use crate::xkb::utils::cstr_dup;
pub use crate::xkb::utils::eaccess;
use crate::xkb::utils::xkb_stat;
pub use crate::xkb::utils::{__dirstream, closedir, opendir, readdir, DIR};
pub use crate::xkb::utils::{check_eaccess, is_space, istrncmp, istrneq, strdup_safe};
use crate::xkb::utils::{cstr_cmp, cstr_free, cstr_len};
use libc::{free, qsort};
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
            err = xkb_stat(path, &raw mut stat_buf);
            if err != 0 as i32 {
                err = *__errno_location();
            } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
                err = ENOTDIR;
            } else if !check_eaccess(path, R_OK | X_OK) {
                err = EACCES;
            } else {
                (*ctx).includes.push(tmp);
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
            (*ctx).failed_includes.push(tmp);
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
    mut extensions: *mut Vec<*mut i8>,
    mut versioned_count: u32,
    mut versioned_path_length: usize,
) -> i32 {
    unsafe {
        let mut entry: *mut dirent = std::ptr::null_mut();
        let mut path_buf: [i8; 4096] = [0; 4096];
        let mut c2rust_current_block: u64;
        let mut ret: i32 = 0 as i32;
        let mut err: i32 = ENOMEM;
        let mut dir: *mut DIR = std::ptr::null_mut();
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
        err = xkb_stat(path, &raw mut stat_buf);
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
                entry = std::ptr::null_mut();
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
                        std::mem::size_of::<[i8; 4096]>(),
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
                        if xkb_stat(&raw mut path_buf as *mut i8, &raw mut stat_buf) != 0 as i32
                            || !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32)
                        {
                            continue;
                        }
                        let mut i: u32 = 0 as u32;
                        while i < versioned_count {
                            let prev_name: *const i8 =
                                ((&*extensions)[i as usize]).offset(versioned_path_length as isize);
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
                            (&mut *extensions).push(ext_path);
                        }
                    }
                }
                match c2rust_current_block {
                    9563249396912231495 => {}
                    _ => {
                        closedir(dir);
                        if (&*extensions).len() as u32 > versioned_count {
                            qsort(
                                (*extensions).as_mut_ptr().offset(versioned_count as isize)
                                    as *mut ::core::ffi::c_void,
                                ((&*extensions).len() as u32).wrapping_sub(versioned_count)
                                    as usize,
                                std::mem::size_of::<*mut i8>(),
                                Some(
                                    compare_str
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        )
                                            -> i32,
                                ),
                            );
                            let mut ext_path_0: *mut *mut i8 = std::ptr::null_mut();
                            if !(*extensions).is_empty() {
                                ext_path_0 =
                                    (*extensions).as_mut_ptr().offset(versioned_count as isize)
                                        as *mut *mut i8;
                                while ext_path_0
                                    < (*extensions)
                                        .as_mut_ptr()
                                        .offset((&*extensions).len() as isize)
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
        let mut extensions: Vec<*mut i8> = Vec::new();
        let mut extensions_path: *const i8 =
            xkb_context_include_path_get_versioned_extensions_path(ctx);
        let mut versioned_path_length: usize = 0 as usize;
        if !extensions_path.is_null() {
            ret |= add_direct_subdirectories(
                ctx,
                extensions_path,
                &raw mut extensions,
                0 as u32,
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
                extensions.len() as u32,
                versioned_path_length,
            );
        }
        for ext_item in &extensions {
            cstr_free(*ext_item);
        }
        drop(extensions);
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
        for path in (*ctx).includes.iter() {
            cstr_free(*path);
        }
        (*ctx).includes.clear();
        for path in (*ctx).failed_includes.iter() {
            cstr_free(*path);
        }
        (*ctx).failed_includes.clear();
        (*ctx).pending_default_includes = false;
    }
}

pub unsafe fn xkb_context_num_include_paths(mut ctx: *mut xkb_context) -> u32 {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).includes.len() as u32
        } else {
            0 as u32
        };
    }
}
pub unsafe fn xkb_context_include_path_get(mut ctx: *mut xkb_context, mut idx: u32) -> *const i8 {
    unsafe {
        if idx >= xkb_context_num_include_paths(ctx) {
            return std::ptr::null();
        }
        return *(*ctx).includes.as_ptr().add(idx as usize);
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
        drop(Box::from_raw(ctx));
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
        _ => return std::ptr::null(),
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
        let (val, consumed) = crate::xkb::utils::cstr_parse_long(level);
        if consumed > 0 {
            let after = *level.offset(consumed as isize);
            if after as i32 == '\0' as i32 || is_space(after) {
                return val as xkb_log_level;
            }
        }
        if istrneq(
            b"crit\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(
            b"err\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_ERROR;
        }
        if istrneq(
            b"warn\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_WARNING;
        }
        if istrneq(
            b"info\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 5]>()).wrapping_sub(1 as usize),
        ) {
            return XKB_LOG_LEVEL_INFO;
        }
        if istrneq(
            b"debug\0".as_ptr() as *const i8,
            level,
            (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
        ) as i32
            != 0
            || istrneq(
                b"dbg\0".as_ptr() as *const i8,
                level,
                (std::mem::size_of::<[i8; 4]>()).wrapping_sub(1 as usize),
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
        let (val, consumed) = crate::xkb::utils::cstr_parse_long(verbosity);
        if consumed > 0 {
            return val as i32;
        }
        return XKB_LOG_VERBOSITY_DEFAULT as i32;
    }
}
pub unsafe fn xkb_context_new(mut flags: xkb_context_flags) -> *mut xkb_context {
    unsafe {
        let mut env: *const i8 = std::ptr::null();
        let ctx: *mut xkb_context = {
            // Allocate zeroed memory (all scalar/pointer fields zero-init fine),
            // then write Vec fields which cannot be zero-initialized.
            let layout = std::alloc::Layout::new::<xkb_context>();
            let ptr = std::alloc::alloc_zeroed(layout) as *mut xkb_context;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            std::ptr::write(&raw mut (*ptr).includes, Vec::new());
            std::ptr::write(&raw mut (*ptr).failed_includes, Vec::new());
            ptr
        };
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
            drop(Box::from_raw(ctx));
            return std::ptr::null_mut();
        }
        (*ctx).use_environment_names = flags as u32 & XKB_CONTEXT_NO_ENVIRONMENT_NAMES as u32 == 0;
        (*ctx).use_secure_getenv = flags as u32 & XKB_CONTEXT_NO_SECURE_GETENV as u32 == 0;
        (*ctx).pending_default_includes =
            flags as u32 & XKB_CONTEXT_NO_DEFAULT_INCLUDES as u32 == 0;
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
            return std::ptr::null_mut();
        }
        (*ctx).x11_atom_cache = std::ptr::null_mut::<core::ffi::c_void>();
        return ctx;
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

use crate::xkb::shared_types::*;
