use crate::xkb::context::xkb_context_include_path_get_system_path;
use crate::xkb::atom::{atom_table_size, atom_intern, atom_text};

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
pub use crate::xkb::shared_types::{RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT};
pub use crate::xkb::utils::isempty;
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::shared_types::{DEFAULT_XKB_LAYOUT, DEFAULT_XKB_MODEL, DEFAULT_XKB_RULES};
use crate::xkb::utils::cstr_len;
use libc::{getenv};
extern "C" {
    pub fn secure_getenv(name: *const i8) -> *mut i8;
}

/// Macro that formats a Rust format string into a stack buffer, then calls `xkb_log`.
/// This uses `core::fmt::Write` instead of C `snprintf`.
/// Usage: `xkb_logf!(ctx, level, verbosity, "format {}", arg)`
#[macro_export]
macro_rules! xkb_logf {
    ($ctx:expr, $level:expr, $verb:expr, $($arg:tt)*) => {{
        let mut _xkb_log_buf = [0u8; 2048];
        {
            let mut _w = crate::xkb::utils::LogBuf::new(&mut _xkb_log_buf[..2047]);
            let _ = core::fmt::Write::write_fmt(&mut _w, format_args!($($arg)*));
        }
        // _xkb_log_buf[2047] is always 0 (zero-initialized), so NUL-terminated
        crate::xkb::context_priv::xkb_log(
            $ctx, $level, $verb,
            _xkb_log_buf.as_ptr() as *const i8
        )
    }};
}

pub unsafe fn xkb_context_getenv(mut ctx: *mut xkb_context, mut name: *const i8) -> *mut i8 {
    unsafe {
        if (*ctx).use_secure_getenv() {
            return secure_getenv(name);
        } else {
            return getenv(name);
        };
    }
}
pub unsafe fn xkb_context_init_includes(mut ctx: *mut xkb_context) -> bool {
    unsafe {
        if (*ctx).pending_default_includes() {
            if (*ctx).failed_includes.size == 0 as darray_size_t {
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
                    return 0 != 0;
                }
                (*ctx).set_pending_default_includes((0 != 0) as bool);
            } else {
                return 0 != 0;
            }
        }
        return 1 != 0;
    }
}
pub unsafe fn xkb_context_num_failed_include_paths(mut ctx: *mut xkb_context) -> darray_size_t {
    unsafe {
        return if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).failed_includes.size
        } else {
            0 as darray_size_t
        };
    }
}
pub unsafe fn xkb_context_failed_include_path_get(
    mut ctx: *mut xkb_context,
    mut idx: darray_size_t,
) -> *const i8 {
    unsafe {
        if idx >= xkb_context_num_failed_include_paths(ctx) {
            return ::core::ptr::null::<i8>();
        }
        return *(*ctx).failed_includes.item.offset(idx as isize);
    }
}

pub unsafe fn xkb_atom_table_size(mut ctx: *mut xkb_context) -> darray_size_t {
    unsafe {
        return atom_table_size((*ctx).atom_table);
    }
}
pub unsafe fn xkb_atom_lookup(mut ctx: *mut xkb_context, mut string: *const i8) -> xkb_atom_t {
    unsafe {
        return atom_intern((*ctx).atom_table, string, cstr_len(string), 0 != 0);
    }
}
pub unsafe fn xkb_atom_intern(
    mut ctx: *mut xkb_context,
    mut string: *const i8,
    mut len: usize,
) -> xkb_atom_t {
    unsafe {
        return atom_intern((*ctx).atom_table, string, len, 1 != 0);
    }
}
pub unsafe fn xkb_atom_text(mut ctx: *mut xkb_context, mut atom: xkb_atom_t) -> *const i8 {
    unsafe {
        return atom_text((*ctx).atom_table, atom);
    }
}
pub unsafe fn xkb_log(
    mut ctx: *mut xkb_context,
    mut level: xkb_log_level,
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
pub unsafe fn xkb_context_get_buffer(mut ctx: *mut xkb_context, mut size: usize) -> *mut i8 {
    unsafe {
        let mut rtrn: *mut i8 = ::core::ptr::null_mut::<i8>();
        if size >= ::core::mem::size_of::<[i8; 2048]>() as usize {
            return ::core::ptr::null_mut::<i8>();
        }
        if (::core::mem::size_of::<[i8; 2048]>() as usize).wrapping_sub((*ctx).text_next as usize)
            <= size
        {
            (*ctx).text_next = 0 as usize;
        }
        rtrn =
            (&raw mut (*ctx).text_buffer as *mut i8).offset((*ctx).text_next as isize) as *mut i8;
        (*ctx).text_next = (*ctx).text_next.wrapping_add(size);
        return rtrn;
    }
}
unsafe fn xkb_context_get_default_rules(mut ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(ctx, b"XKB_DEFAULT_RULES\0".as_ptr() as *const i8);
        }
        return if !env.is_null() {
            env
        } else {
            DEFAULT_XKB_RULES.as_ptr()
        };
    }
}
unsafe fn xkb_context_get_default_model(mut ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(ctx, b"XKB_DEFAULT_MODEL\0".as_ptr() as *const i8);
        }
        return if !env.is_null() {
            env
        } else {
            DEFAULT_XKB_MODEL.as_ptr()
        };
    }
}
unsafe fn xkb_context_get_default_layout(mut ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(ctx, b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const i8);
        }
        return if !env.is_null() {
            env
        } else {
            DEFAULT_XKB_LAYOUT.as_ptr()
        };
    }
}
unsafe fn xkb_context_get_default_variant(mut ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        let mut layout: *const i8 =
            xkb_context_getenv(ctx, b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const i8);
        if !layout.is_null() && (*ctx).use_environment_names() as i32 != 0 {
            env = xkb_context_getenv(ctx, b"XKB_DEFAULT_VARIANT\0".as_ptr() as *const i8);
        }
        return if !env.is_null() {
            env
        } else {
            ::core::ptr::null::<i8>()
        };
    }
}
unsafe fn xkb_context_get_default_options(mut ctx: *mut xkb_context) -> *const i8 {
    unsafe {
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        if (*ctx).use_environment_names() {
            env = xkb_context_getenv(ctx, b"XKB_DEFAULT_OPTIONS\0".as_ptr() as *const i8);
        }
        return if !env.is_null() {
            env
        } else {
            ::core::ptr::null::<i8>()
        };
    }
}
pub unsafe fn xkb_context_sanitize_rule_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *mut xkb_rule_names,
) -> RMLVO {
    unsafe {
        let mut modified: RMLVO = 0 as RMLVO;
        if isempty((*rmlvo).rules) {
            (*rmlvo).rules = xkb_context_get_default_rules(ctx);
            modified = (modified as u32 | RMLVO_RULES as i32 as u32) as RMLVO;
        }
        if isempty((*rmlvo).model) {
            (*rmlvo).model = xkb_context_get_default_model(ctx);
            modified = (modified as u32 | RMLVO_MODEL as i32 as u32) as RMLVO;
        }
        if isempty((*rmlvo).layout) {
            (*rmlvo).layout = xkb_context_get_default_layout(ctx);
            modified = (modified as u32 | RMLVO_LAYOUT as i32 as u32) as RMLVO;
            let variant: *const i8 = xkb_context_get_default_variant(ctx) as *const i8;
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
            modified = (modified as u32 | RMLVO_VARIANT as i32 as u32) as RMLVO;
        }
        if (*rmlvo).options.is_null() {
            (*rmlvo).options = xkb_context_get_default_options(ctx);
            modified = (modified as u32 | RMLVO_OPTIONS as i32 as u32) as RMLVO;
        }
        return modified;
    }
}
use crate::xkb::shared_types::*;
use crate::xkb::context::xkb_context_include_path_append_default;
