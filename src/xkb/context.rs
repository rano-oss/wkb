use crate::xkb::atom::{atom_intern, atom_table_new, atom_text, atom_text_bytes};

pub use crate::xkb::messages::{
    xkb_log_verbosity, xkb_message_code, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
    XKB_ERROR_ABI_BACKWARD_COMPAT_, XKB_ERROR_ABI_FORWARD_COMPAT_,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR, XKB_ERROR_CANNOT_RESOLVE_RMLVO,
    XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY, XKB_ERROR_EXPECTED_ARRAY_ENTRY,
    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE, XKB_ERROR_INCLUDED_FILE_NOT_FOUND,
    XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT, XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT,
    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE, XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD,
    XKB_ERROR_INVALID_COMPOSE_LOCALE, XKB_ERROR_INVALID_COMPOSE_SYNTAX,
    XKB_ERROR_INVALID_EXPRESSION_TYPE, XKB_ERROR_INVALID_FILE_ENCODING,
    XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
    XKB_ERROR_INVALID_INCLUDE_STATEMENT, XKB_ERROR_INVALID_MODMAP_ENTRY,
    XKB_ERROR_INVALID_NUMERIC_KEYSYM, XKB_ERROR_INVALID_OPERATION, XKB_ERROR_INVALID_PATH,
    XKB_ERROR_INVALID_REAL_MODIFIER, XKB_ERROR_INVALID_RULES_SYNTAX,
    XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT, XKB_ERROR_INVALID_VALUE, XKB_ERROR_INVALID_XKB_SYNTAX,
    XKB_ERROR_KEYMAP_COMPILATION_FAILED, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_ERROR_OVERLAPPING_OVERLAY,
    XKB_ERROR_RECURSIVE_INCLUDE, XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION,
    XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER, XKB_ERROR_UNKNOWN_ACTION_TYPE,
    XKB_ERROR_UNKNOWN_DEFAULT_FIELD, XKB_ERROR_UNKNOWN_FIELD, XKB_ERROR_UNKNOWN_OPERATOR,
    XKB_ERROR_UNKNOWN_STATEMENT, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_, XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX,
    XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL, XKB_ERROR_WRONG_FIELD_TYPE, XKB_ERROR_WRONG_STATEMENT_TYPE,
    XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE, XKB_LOG_VERBOSITY_DEFAULT,
    XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL, XKB_LOG_VERBOSITY_SILENT,
    XKB_LOG_VERBOSITY_VERBOSE, XKB_WARNING_CANNOT_INFER_KEY_TYPE,
    XKB_WARNING_CONFLICTING_KEY_ACTION, XKB_WARNING_CONFLICTING_KEY_FIELDS,
    XKB_WARNING_CONFLICTING_KEY_NAME, XKB_WARNING_CONFLICTING_KEY_SYMBOL,
    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS, XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY, XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES, XKB_WARNING_CONFLICTING_MODMAP,
    XKB_WARNING_DEPRECATED_KEYSYM, XKB_WARNING_DEPRECATED_KEYSYM_NAME, XKB_WARNING_DUPLICATE_ENTRY,
    XKB_WARNING_EXTRA_SYMBOLS_IGNORED, XKB_WARNING_ILLEGAL_KEYCODE_ALIAS,
    XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT, XKB_WARNING_INVALID_ESCAPE_SEQUENCE,
    XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE, XKB_WARNING_MISSING_DEFAULT_SECTION,
    XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX, XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE,
    XKB_WARNING_NON_BASE_GROUP_NAME, XKB_WARNING_NUMERIC_KEYSYM,
    XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE, XKB_WARNING_UNDEFINED_KEYCODE,
    XKB_WARNING_UNDEFINED_KEY_TYPE, XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
    XKB_WARNING_UNRECOGNIZED_KEYSYM, XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL,
    XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION, XKB_WARNING_UNSUPPORTED_LEGACY_ACTION,
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD,
};
pub use crate::xkb::shared_types::dirent;
pub use crate::xkb::shared_types::stat;
pub use crate::xkb::shared_types::timespec;
pub use crate::xkb::shared_types::__S_IFMT;
use crate::xkb::shared_types::{
    DEFAULT_XKB_LAYOUT, DEFAULT_XKB_MODEL, DEFAULT_XKB_RULES, DFLT_XKB_CONFIG_EXTRA_PATH,
    DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use crate::xkb::shared_types::{EACCES, ENOMEM, ENOTDIR};
pub use crate::xkb::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use crate::xkb::shared_types::{R_OK, X_OK};
use crate::xkb::utils::__errno_location;
use crate::xkb::utils::cstr_dup;
use crate::xkb::utils::isempty;
use crate::xkb::utils::xkb_stat;
use crate::xkb::utils::{check_eaccess, cstr_as_bytes, istrneq, strdup_safe};
use crate::xkb::utils::{closedir, opendir, readdir, DIR};
use crate::xkb::utils::{cstr_cmp, cstr_free, cstr_len};
use libc::{free, getenv, qsort};

extern "C" {
    pub fn secure_getenv(name: *const i8) -> *mut i8;
}

/// Macro that formats a message and prints to stderr.
/// Usage: `xkb_logf!(ctx, level, verbosity, "format {}", arg)`
/// The ctx, level, and verbosity args are accepted for compatibility but
/// level/verbosity filtering is not currently implemented.
#[macro_export]
macro_rules! xkb_logf {
    ($ctx:expr, $level:expr, $verb:expr, $($arg:tt)*) => {{
        eprint!($($arg)*);
    }};
}
unsafe fn context_include_path_append(ctx: *mut xkb_context, mut path: *const i8) -> i32 {
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
pub unsafe fn xkb_context_include_path_append(ctx: *mut xkb_context, mut path: *const i8) -> i32 {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            context_include_path_append(ctx, path)
        } else {
            0 as i32
        };
    }
}
pub unsafe fn xkb_context_include_path_get_extra_path(ctx: *mut xkb_context) -> *const i8 {
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
    ctx: *mut xkb_context,
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
    ctx: *mut xkb_context,
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
    ctx: *mut xkb_context,
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
                    if cstr_as_bytes(name) == b"." || cstr_as_bytes(name) == b".." {
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
                            if cstr_as_bytes(name) == cstr_as_bytes(prev_name) {
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
pub unsafe fn xkb_context_include_path_get_system_path(ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let root: *const i8 = xkb_context_getenv(ctx, b"XKB_CONFIG_ROOT\0".as_ptr() as *const i8);
        return if !root.is_null() {
            root
        } else {
            DFLT_XKB_CONFIG_ROOT.as_ptr()
        };
    }
}
pub unsafe fn xkb_context_include_path_append_default(ctx: *mut xkb_context) -> i32 {
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

pub unsafe fn xkb_context_include_path_clear(ctx: *mut xkb_context) {
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

pub unsafe fn xkb_context_num_include_paths(ctx: *mut xkb_context) -> u32 {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).includes.len() as u32
        } else {
            0 as u32
        };
    }
}
pub unsafe fn xkb_context_include_path_get(ctx: *mut xkb_context, mut idx: u32) -> *const i8 {
    unsafe {
        if idx >= xkb_context_num_include_paths(ctx) {
            return std::ptr::null();
        }
        return *(*ctx).includes.as_ptr().add(idx as usize);
    }
}
pub unsafe fn xkb_context_ref(ctx: *mut xkb_context) -> *mut xkb_context {
    unsafe {
        (*ctx).refcnt += 1;
        return ctx;
    }
}
pub unsafe fn xkb_context_unref(ctx: *mut xkb_context) {
    unsafe {
        if ctx.is_null() {
            return;
        }
        (*ctx).refcnt -= 1;
        if (*ctx).refcnt > 0 as i32 {
            return;
        }
        xkb_context_include_path_clear(ctx);
        // atom_table is owned, dropped automatically when xkb_context is dropped
    }
}
unsafe fn log_level_to_prefix(mut level: u32) -> *const i8 {
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
unsafe fn default_log_fn(ctx: *mut xkb_context, mut level: u32, mut msg: *const i8) {
    unsafe {
        let mut prefix: *const i8 = log_level_to_prefix(level);
        if !prefix.is_null() {
            eprint!("{}", crate::xkb::utils::CStrDisplay(prefix));
        }
        eprint!("{}", crate::xkb::utils::CStrDisplay(msg));
    }
}
unsafe fn log_level(mut level: *const i8) -> u32 {
    unsafe {
        let (val, consumed) = crate::xkb::utils::cstr_parse_long(level);
        if consumed > 0 {
            let after = *level.offset(consumed as isize);
            if after as i32 == '\0' as i32
                || matches!(after as u8, b' ' | b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r')
            {
                return val as u32;
            }
        }
        if istrneq(b"crit", cstr_as_bytes(level), 4) {
            return XKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(b"err", cstr_as_bytes(level), 3) {
            return XKB_LOG_LEVEL_ERROR;
        }
        if istrneq(b"warn", cstr_as_bytes(level), 4) {
            return XKB_LOG_LEVEL_WARNING;
        }
        if istrneq(b"info", cstr_as_bytes(level), 4) {
            return XKB_LOG_LEVEL_INFO;
        }
        if istrneq(b"debug", cstr_as_bytes(level), 5) || istrneq(b"dbg", cstr_as_bytes(level), 3) {
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
pub unsafe fn xkb_context_new(mut flags: xkb_context_flags) -> xkb_context {
    unsafe {
        let mut env: *const i8 = std::ptr::null();
        let mut ctx = xkb_context {
            refcnt: 1,
            log_fn: Some(default_log_fn as unsafe fn(*mut xkb_context, u32, *const i8) -> ()),
            log_level: XKB_LOG_LEVEL_ERROR,
            log_verbosity: XKB_LOG_VERBOSITY_DEFAULT as i32,
            names_dflt: xkb_rule_names {
                rules: std::ptr::null(),
                model: std::ptr::null(),
                layout: std::ptr::null(),
                variant: std::ptr::null(),
                options: std::ptr::null(),
            },
            includes: Vec::new(),
            failed_includes: Vec::new(),
            atom_table: atom_table_new(),
            text_buffer: [0i8; 2048],
            text_next: 0,
            use_environment_names: false,
            use_secure_getenv: false,
            pending_default_includes: false,
        };
        static mut XKB_CONTEXT_FLAGS: xkb_context_flags = (XKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
            | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32
            | XKB_CONTEXT_NO_SECURE_GETENV as i32)
            as xkb_context_flags;
        if flags as u32 & !(XKB_CONTEXT_FLAGS as u32) != 0 {
            xkb_logf!(
                &mut ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Invalid context flags: 0x{:x}\n",
                flags as u32 & !(XKB_CONTEXT_FLAGS as u32),
            );
            // Return a dummy/default — caller should check flags before calling
            // In practice this path is very rare
            return ctx;
        }
        ctx.use_environment_names = flags as u32 & XKB_CONTEXT_NO_ENVIRONMENT_NAMES as u32 == 0;
        ctx.use_secure_getenv = flags as u32 & XKB_CONTEXT_NO_SECURE_GETENV as u32 == 0;
        ctx.pending_default_includes = flags as u32 & XKB_CONTEXT_NO_DEFAULT_INCLUDES as u32 == 0;
        env = xkb_context_getenv(&raw mut ctx, b"XKB_LOG_LEVEL\0".as_ptr() as *const i8);
        if !env.is_null() {
            xkb_context_set_log_level(&raw mut ctx, log_level(env));
        }
        env = xkb_context_getenv(&raw mut ctx, b"XKB_LOG_VERBOSITY\0".as_ptr() as *const i8);
        if !env.is_null() {
            xkb_context_set_log_verbosity(&raw mut ctx, log_verbosity(env));
        }
        return ctx;
    }
}

pub unsafe fn xkb_context_set_log_level(ctx: *mut xkb_context, mut level: u32) {
    unsafe {
        (*ctx).log_level = level;
    }
}
pub unsafe fn xkb_context_get_log_verbosity(ctx: *mut xkb_context) -> i32 {
    unsafe {
        return (*ctx).log_verbosity;
    }
}

pub unsafe fn xkb_context_set_log_verbosity(ctx: *mut xkb_context, mut verbosity: i32) {
    unsafe {
        (*ctx).log_verbosity = verbosity;
    }
}

// --- Merged from context_priv.rs ---

pub unsafe fn xkb_context_getenv(ctx: *mut xkb_context, mut name: *const i8) -> *mut i8 {
    unsafe {
        if (*ctx).use_secure_getenv {
            return secure_getenv(name);
        } else {
            return getenv(name);
        };
    }
}
pub unsafe fn xkb_context_init_includes(ctx: *mut xkb_context) -> bool {
    unsafe {
        if (*ctx).pending_default_includes {
            if (*ctx).failed_includes.is_empty() {
                if xkb_context_include_path_append_default(ctx) == 0 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Failed to add any default include path (system path: {})\n",
                        XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                        crate::xkb::utils::CStrDisplay(xkb_context_include_path_get_system_path(
                            ctx
                        )),
                    );
                    return false;
                }
                (*ctx).pending_default_includes = false;
            } else {
                return false;
            }
        }
        return true;
    }
}
pub unsafe fn xkb_context_num_failed_include_paths(ctx: *mut xkb_context) -> u32 {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).failed_includes.len() as u32
        } else {
            0 as u32
        };
    }
}
pub unsafe fn xkb_context_failed_include_path_get(
    ctx: *mut xkb_context,
    mut idx: u32,
) -> *const i8 {
    unsafe {
        if idx >= xkb_context_num_failed_include_paths(ctx) {
            return std::ptr::null();
        }
        return *(*ctx).failed_includes.as_ptr().add(idx as usize);
    }
}

pub unsafe fn xkb_atom_lookup(ctx: *mut xkb_context, mut string: *const i8) -> u32 {
    unsafe {
        return atom_intern(&mut (*ctx).atom_table, string, cstr_len(string), false);
    }
}
pub unsafe fn xkb_atom_intern(ctx: *mut xkb_context, mut string: *const i8, mut len: usize) -> u32 {
    unsafe {
        return atom_intern(&mut (*ctx).atom_table, string, len, true);
    }
}
pub unsafe fn xkb_atom_text(ctx: *mut xkb_context, mut atom: u32) -> *const i8 {
    unsafe {
        return atom_text(&(*ctx).atom_table, atom);
    }
}
pub unsafe fn xkb_atom_text_bytes<'a>(ctx: *mut xkb_context, atom: u32) -> &'a [u8] {
    atom_text_bytes(&(*ctx).atom_table, atom)
}
pub unsafe fn xkb_log(
    ctx: *mut xkb_context,
    mut level: u32,
    mut verbosity: i32,
    mut msg: *const i8,
) {
    unsafe {
        if ((*ctx).log_level as u32) < level as u32 || (*ctx).log_verbosity < verbosity {
            return;
        }
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, msg);
    }
}
pub unsafe fn xkb_context_get_buffer(ctx: *mut xkb_context, mut size: usize) -> *mut i8 {
    unsafe {
        let mut rtrn: *mut i8 = std::ptr::null_mut();
        if size >= std::mem::size_of::<[i8; 2048]>() {
            return std::ptr::null_mut();
        }
        if (std::mem::size_of::<[i8; 2048]>()).wrapping_sub((*ctx).text_next as usize) <= size {
            (*ctx).text_next = 0 as usize;
        }
        rtrn =
            (&raw mut (*ctx).text_buffer as *mut i8).offset((*ctx).text_next as isize) as *mut i8;
        (*ctx).text_next = (*ctx).text_next.wrapping_add(size);
        return rtrn;
    }
}
pub unsafe fn xkb_context_sanitize_rule_names(
    ctx: *mut xkb_context,
    mut rmlvo: *mut xkb_rule_names,
) -> RMLVO {
    unsafe {
        let mut modified: RMLVO = 0 as RMLVO;
        if isempty((*rmlvo).rules) {
            let mut env: *const i8 = std::ptr::null();
            if (*ctx).use_environment_names {
                env = xkb_context_getenv(ctx, b"XKB_DEFAULT_RULES\0".as_ptr() as *const i8);
            }
            (*rmlvo).rules = if !env.is_null() {
                env
            } else {
                DEFAULT_XKB_RULES.as_ptr()
            };
            modified = (modified as u32 | RMLVO_RULES as u32) as RMLVO;
        }
        if isempty((*rmlvo).model) {
            let mut env: *const i8 = std::ptr::null();
            if (*ctx).use_environment_names {
                env = xkb_context_getenv(ctx, b"XKB_DEFAULT_MODEL\0".as_ptr() as *const i8);
            }
            (*rmlvo).model = if !env.is_null() {
                env
            } else {
                DEFAULT_XKB_MODEL.as_ptr()
            };
            modified = (modified as u32 | RMLVO_MODEL as u32) as RMLVO;
        }
        if isempty((*rmlvo).layout) {
            {
                let mut env: *const i8 = std::ptr::null();
                if (*ctx).use_environment_names {
                    env = xkb_context_getenv(ctx, b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const i8);
                }
                (*rmlvo).layout = if !env.is_null() {
                    env
                } else {
                    DEFAULT_XKB_LAYOUT.as_ptr()
                };
            }
            modified = (modified as u32 | RMLVO_LAYOUT as u32) as RMLVO;
            let variant: *const i8 = {
                let mut env: *const i8 = std::ptr::null();
                let mut layout: *const i8 =
                    xkb_context_getenv(ctx, b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const i8);
                if !layout.is_null() && (*ctx).use_environment_names {
                    env = xkb_context_getenv(ctx, b"XKB_DEFAULT_VARIANT\0".as_ptr() as *const i8);
                }
                if !env.is_null() {
                    env
                } else {
                    std::ptr::null()
                }
            };
            if !isempty((*rmlvo).variant) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Layout not provided, but variant set to \"{}\": ignoring variant and using defaults for both: layout=\"{}\", variant=\"{}\".\n",
                    crate::xkb::utils::CStrDisplay((*rmlvo).variant),
                    crate::xkb::utils::CStrDisplay((*rmlvo).layout),
                    crate::xkb::utils::CStrDisplay(if !variant.is_null() {
                        variant
                    } else {
                        b"\0".as_ptr() as *const i8
                    }),
                );
            }
            (*rmlvo).variant = variant;
            modified = (modified as u32 | RMLVO_VARIANT as u32) as RMLVO;
        }
        if (*rmlvo).options.is_null() {
            let mut env: *const i8 = std::ptr::null();
            if (*ctx).use_environment_names {
                env = xkb_context_getenv(ctx, b"XKB_DEFAULT_OPTIONS\0".as_ptr() as *const i8);
            }
            (*rmlvo).options = if !env.is_null() {
                env
            } else {
                std::ptr::null()
            };
            modified = (modified as u32 | RMLVO_OPTIONS as u32) as RMLVO;
        }
        return modified;
    }
}

use crate::xkb::shared_types::*;
