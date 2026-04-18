use std::env::VarError;

use crate::xkb::atom::{atom_intern, atom_table_new, atom_text};

pub use crate::xkb::messages::{
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
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
};
use crate::xkb::shared_types::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use crate::xkb::shared_types::{EACCES, ENOMEM, ENOTDIR};
pub use crate::xkb::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use crate::xkb::shared_types::{R_OK, X_OK};
// isempty no longer needed — String fields use .is_empty()
use crate::xkb::utils::cstr_as_bytes;
use crate::xkb::utils::cstr_len;
/// Macro that formats a message and prints to stderr.
/// Usage: `xkb_logf!(ctx, level, verbosity, "format {}", arg)`
/// The ctx, level, and verbosity args are accepted for compatibility but
/// XKB logging macro — dispatches to the `log` crate based on level.
/// The ctx and verbosity arguments are accepted for compatibility but ignored.
#[macro_export]
macro_rules! xkb_logf {
    ($ctx:expr, $level:expr, $verb:expr, $($arg:tt)*) => {
        log::log!(
            match $level {
                10 | 20 => log::Level::Error,
                30 => log::Level::Warn,
                40 => log::Level::Info,
                _ => log::Level::Debug,
            },
            $($arg)*
        )
    };
}
unsafe fn context_include_path_append(ctx: *mut xkb_context, path: &str) -> i32 {
    unsafe {
        let is_dir = std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false);
        if is_dir {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_INFO,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Include path added: {}\n",
                path,
            );
            (*ctx).includes.push(path.to_string());
            return 1_i32;
        }
        if !path.is_empty() {
            (*ctx).failed_includes.push(path.to_string());
        }
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_INFO,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "Include path failed: \"{}\"\n",
            path,
        );
        0_i32
    }
}
pub unsafe fn xkb_context_include_path_append(ctx: *mut xkb_context, path: *const i8) -> i32 {
    unsafe {
        if xkb_context_init_includes(ctx) as i32 != 0 {
            let path_str = std::ffi::CStr::from_ptr(path).to_str().unwrap_or("");
            context_include_path_append(ctx, path_str)
        } else {
            0_i32
        }
    }
}
pub fn xkb_context_include_path_get_extra_path(_ctx: *mut xkb_context) -> String {
    match xkb_context_getenv("XKB_CONFIG_EXTRA_PATH") {
        Ok(extra) => extra,
        Err(_) => cstr_const_to_string(&DFLT_XKB_CONFIG_EXTRA_PATH),
    }
}

pub fn xkb_context_include_path_get_unversioned_extensions_path(_ctx: *mut xkb_context) -> String {
    match xkb_context_getenv("XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => cstr_const_to_string(&DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH),
    }
}

pub fn xkb_context_include_path_get_versioned_extensions_path(_ctx: *mut xkb_context) -> String {
    match xkb_context_getenv("XKB_CONFIG_VERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => cstr_const_to_string(&DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH),
    }
}
/// Convert a null-terminated `[i8]` constant to a Rust `String`.
fn cstr_const_to_string(bytes: &[i8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    // SAFETY: these are ASCII path constants from shared_types
    String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u8, end) })
        .into_owned()
}

fn add_direct_subdirectories(
    ctx: *mut xkb_context,
    path: &str,
    extensions: &mut Vec<String>,
    versioned_count: usize,
    versioned_path_length: usize,
) -> i32 {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(e) => {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Include extensions path failed: {} ({})\n",
                path,
                e,
            );
            return 0_i32;
        }
    };

    // The +1 accounts for the '/' separator between the base path and entry name
    let name_offset = if versioned_path_length > 0 {
        versioned_path_length + 1
    } else {
        0
    };

    for entry in dir.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == "." || name_str == ".." {
            continue;
        }
        let full_path = format!("{}/{}", path, name_str);
        // Check if it's a directory
        if !std::fs::metadata(&full_path)
            .map(|m| m.is_dir())
            .unwrap_or(false)
        {
            continue;
        }
        // Check if already in versioned list
        let mut duplicate = false;
        for i in 0..versioned_count {
            if name_offset <= extensions[i].len() && *name_str == extensions[i][name_offset..] {
                duplicate = true;
                break;
            }
        }
        if duplicate {
            continue;
        }
        extensions.push(full_path);
    }

    let mut ret = 0_i32;
    // Sort the newly added entries and append as include paths
    if extensions.len() > versioned_count {
        extensions[versioned_count..].sort();
        for i in versioned_count..extensions.len() {
            // SAFETY: ctx is a valid pointer, context_include_path_append only dereferences ctx
            ret |= unsafe { context_include_path_append(ctx, &extensions[i]) };
        }
    }

    ret
}

pub fn xkb_context_include_path_get_system_path(_ctx: *mut xkb_context) -> String {
    match xkb_context_getenv("XKB_CONFIG_ROOT") {
        Ok(root) => root,
        Err(_) => cstr_const_to_string(&DFLT_XKB_CONFIG_ROOT),
    }
}

pub unsafe fn xkb_context_include_path_append_default(ctx: *mut xkb_context) -> i32 {
    unsafe {
        let mut ret: i32 = 0_i32;
        let home = xkb_context_getenv("HOME");
        let xdg = xkb_context_getenv("XDG_CONFIG_HOME");
        if let Ok(ref xdg) = xdg {
            ret |= context_include_path_append(ctx, &format!("{}/xkb", xdg));
        } else if let Ok(ref home) = home {
            ret |= context_include_path_append(ctx, &format!("{}/.config/xkb", home));
        }
        if let Ok(ref home) = home {
            ret |= context_include_path_append(ctx, &format!("{}/.xkb", home));
        }
        let extra = xkb_context_include_path_get_extra_path(ctx);
        ret |= context_include_path_append(ctx, &extra);

        let mut extensions: Vec<String> = Vec::new();
        let versioned_path = xkb_context_include_path_get_versioned_extensions_path(ctx);
        let mut versioned_path_length: usize = 0;
        if !versioned_path.is_empty() {
            ret |= add_direct_subdirectories(ctx, &versioned_path, &mut extensions, 0, 0);
            versioned_path_length = versioned_path.len();
        }
        let unversioned_path = xkb_context_include_path_get_unversioned_extensions_path(ctx);
        if !unversioned_path.is_empty() {
            let versioned_count = extensions.len();
            ret |= add_direct_subdirectories(
                ctx,
                &unversioned_path,
                &mut extensions,
                versioned_count,
                versioned_path_length,
            );
        }
        // extensions is Vec<String>, dropped automatically

        let root = xkb_context_include_path_get_system_path(ctx);
        let has_root: bool = context_include_path_append(ctx, &root) != 0;
        ret |= has_root as i32;
        if !has_root && !root.is_empty() {
            let legacy = cstr_const_to_string(&DFLT_XKB_LEGACY_ROOT);
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                "/usr/share/X11/xkb",
                root,
            );
            ret |= context_include_path_append(ctx, &legacy);
        }
        ret
    }
}

pub unsafe fn xkb_context_include_path_clear(ctx: *mut xkb_context) {
    unsafe {
        (*ctx).includes.clear();
        (*ctx).failed_includes.clear();
        (*ctx).pending_default_includes = false;
    }
}

pub unsafe fn xkb_context_num_include_paths(ctx: *mut xkb_context) -> u32 {
    unsafe {
        if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).includes.len() as u32
        } else {
            0_u32
        }
    }
}
pub unsafe fn xkb_context_include_path_get(ctx: *mut xkb_context, idx: u32) -> String {
    unsafe {
        if idx >= xkb_context_num_include_paths(ctx) {
            return "".to_string();
        }
        (&*ctx).includes.get(idx as usize).unwrap().clone()
    }
}
pub unsafe fn xkb_context_unref(ctx: *mut xkb_context) {
    unsafe {
        if ctx.is_null() {
            return;
        }
        (*ctx).refcnt -= 1;
        if (*ctx).refcnt > 0_i32 {
            return;
        }
        xkb_context_include_path_clear(ctx);
        // atom_table is owned, dropped automatically when xkb_context is dropped
    }
}
fn log_level_to_prefix(level: u32) -> &'static str {
    match level {
        50 => "xkbcommon: DEBUG: ",
        40 => "xkbcommon: INFO: ",
        30 => "xkbcommon: WARNING: ",
        20 => "xkbcommon: ERROR: ",
        10 => "xkbcommon: CRITICAL: ",
        _ => "",
    }
}
unsafe fn default_log_fn(_ctx: *mut xkb_context, level: u32, msg: *const i8) {
    unsafe {
        let prefix = log_level_to_prefix(level);
        if !prefix.is_empty() {
            eprint!("{}", prefix);
        }
        eprint!(
            "{}",
            std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(msg))
        );
    }
}
unsafe fn log_level(level: *const i8) -> u32 {
    unsafe {
        let (val, consumed) = crate::xkb::utils::cstr_parse_long(level);
        if consumed > 0 {
            let after = *level.add(consumed);
            if after as i32 == '\0' as i32
                || matches!(after as u8, b' ' | b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r')
            {
                return val as u32;
            }
        }
        if cstr_as_bytes(level)
            .get(..4)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"crit"))
        {
            return XKB_LOG_LEVEL_CRITICAL;
        }
        if cstr_as_bytes(level)
            .get(..3)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"err"))
        {
            return XKB_LOG_LEVEL_ERROR;
        }
        if cstr_as_bytes(level)
            .get(..4)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"warn"))
        {
            return XKB_LOG_LEVEL_WARNING;
        }
        if cstr_as_bytes(level)
            .get(..4)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"info"))
        {
            return XKB_LOG_LEVEL_INFO;
        }
        if cstr_as_bytes(level)
            .get(..5)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"debug"))
            || cstr_as_bytes(level)
                .get(..3)
                .is_some_and(|s| s.eq_ignore_ascii_case(b"dbg"))
        {
            return XKB_LOG_LEVEL_DEBUG;
        }
        XKB_LOG_LEVEL_ERROR
    }
}
unsafe fn log_verbosity(verbosity: *const i8) -> i32 {
    unsafe {
        let (val, consumed) = crate::xkb::utils::cstr_parse_long(verbosity);
        if consumed > 0 {
            return val as i32;
        }
        XKB_LOG_VERBOSITY_DEFAULT
    }
}
pub unsafe fn xkb_context_new(flags: xkb_context_flags) -> xkb_context {
    unsafe {
        let mut ctx = xkb_context {
            refcnt: 1,
            log_fn: Some(default_log_fn as unsafe fn(*mut xkb_context, u32, *const i8) -> ()),
            log_level: XKB_LOG_LEVEL_ERROR,
            log_verbosity: XKB_LOG_VERBOSITY_DEFAULT,
            names_dflt: xkb_rule_names {
                rules: std::ffi::CString::new("").unwrap(),
                model: std::ffi::CString::new("").unwrap(),
                layout: std::ffi::CString::new("").unwrap(),
                variant: std::ffi::CString::new("").unwrap(),
                options: std::ffi::CString::new("").unwrap(),
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
        if flags & !XKB_CONTEXT_FLAGS != 0 {
            xkb_logf!(
                &mut ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Invalid context flags: 0x{:x}\n",
                flags & !XKB_CONTEXT_FLAGS,
            );
            // Return a dummy/default — caller should check flags before calling
            // In practice this path is very rare
            return ctx;
        }
        ctx.use_environment_names = flags & XKB_CONTEXT_NO_ENVIRONMENT_NAMES == 0;
        ctx.use_secure_getenv = flags & XKB_CONTEXT_NO_SECURE_GETENV == 0;
        ctx.pending_default_includes = flags & XKB_CONTEXT_NO_DEFAULT_INCLUDES == 0;
        let env = xkb_context_getenv("XKB_LOG_LEVEL");
        if let Ok(env) = env {
            let cenv = std::ffi::CString::new(env).unwrap();
            xkb_context_set_log_level(&raw mut ctx, log_level(cenv.as_ptr()));
        }
        let env = xkb_context_getenv("XKB_LOG_VERBOSITY");
        if let Ok(env) = env {
            let cenv = std::ffi::CString::new(env).unwrap();
            xkb_context_set_log_verbosity(&raw mut ctx, log_verbosity(cenv.as_ptr()));
        }
        ctx
    }
}

pub unsafe fn xkb_context_set_log_level(ctx: *mut xkb_context, level: u32) {
    unsafe {
        (*ctx).log_level = level;
    }
}
pub fn xkb_context_get_log_verbosity(ctx: &xkb_context) -> i32 {
    ctx.log_verbosity
}

pub unsafe fn xkb_context_set_log_verbosity(ctx: *mut xkb_context, verbosity: i32) {
    unsafe {
        (*ctx).log_verbosity = verbosity;
    }
}

// --- Merged from context_priv.rs ---

pub fn xkb_context_getenv(name: &str) -> Result<String, VarError> {
    std::env::var(name)
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
                        xkb_context_include_path_get_system_path(ctx),
                    );
                    return false;
                }
                (*ctx).pending_default_includes = false;
            } else {
                return false;
            }
        }
        true
    }
}
pub unsafe fn xkb_context_num_failed_include_paths(ctx: *mut xkb_context) -> u32 {
    unsafe {
        if xkb_context_init_includes(ctx) as i32 != 0 {
            (*ctx).failed_includes.len() as u32
        } else {
            0_u32
        }
    }
}
pub unsafe fn xkb_context_failed_include_path_get(ctx: *mut xkb_context, idx: u32) -> String {
    unsafe {
        if idx >= xkb_context_num_failed_include_paths(ctx) {
            return "".to_string();
        }
        (&*ctx).failed_includes.get(idx as usize).unwrap().clone()
    }
}

pub unsafe fn xkb_atom_lookup(ctx: *mut xkb_context, string: *const i8) -> u32 {
    unsafe {
        let bytes = std::slice::from_raw_parts(string as *const u8, cstr_len(string));
        atom_intern(&mut (*ctx).atom_table, bytes, false)
    }
}
pub fn xkb_atom_intern_bytes(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
/// # Safety
/// ctx must be valid.
pub unsafe fn xkb_atom_intern(ctx: *mut xkb_context, bytes: &[u8]) -> u32 {
    unsafe { atom_intern(&mut (*ctx).atom_table, bytes, true) }
}
pub fn xkb_atom_text(atom_table: &atom_table, atom: u32) -> &str {
    atom_text(atom_table, atom)
}

pub unsafe fn xkb_log(ctx: *mut xkb_context, level: u32, verbosity: i32, msg: *const i8) {
    unsafe {
        if (*ctx).log_level < level || (*ctx).log_verbosity < verbosity {
            return;
        }
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, msg);
    }
}
pub unsafe fn xkb_context_get_buffer(ctx: &mut xkb_context, size: usize) -> *mut i8 {
    unsafe {
        if size >= std::mem::size_of::<[i8; 2048]>() {
            return std::ptr::null_mut();
        }
        if (std::mem::size_of::<[i8; 2048]>()).wrapping_sub(ctx.text_next) <= size {
            ctx.text_next = 0_usize;
        }
        let rtrn: *mut i8 = (&raw mut ctx.text_buffer as *mut i8).add(ctx.text_next) as *mut i8;
        ctx.text_next = ctx.text_next.wrapping_add(size);
        rtrn
    }
}
pub unsafe fn xkb_context_sanitize_rule_names(
    ctx: &xkb_context,
    rmlvo: *mut xkb_rule_names,
) -> RMLVO {
    unsafe {
        let mut modified: RMLVO = 0 as RMLVO;
        let rmlvo = &mut *rmlvo;
        if rmlvo.rules.as_bytes().is_empty() {
            let env = if ctx.use_environment_names {
                xkb_context_getenv("XKB_DEFAULT_RULES")
            } else {
                Err(VarError::NotPresent)
            };
            rmlvo.rules = match env {
                Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
                Err(_) => std::ffi::CString::new("evdev").unwrap(),
            };
            modified = (modified as u32 | RMLVO_RULES) as RMLVO;
        }
        if rmlvo.model.as_bytes().is_empty() {
            let env = if ctx.use_environment_names {
                xkb_context_getenv("XKB_DEFAULT_MODEL")
            } else {
                Err(VarError::NotPresent)
            };
            rmlvo.model = match env {
                Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
                Err(_) => std::ffi::CString::new("pc105").unwrap(),
            };
            modified = (modified as u32 | RMLVO_MODEL) as RMLVO;
        }
        if rmlvo.layout.as_bytes().is_empty() {
            {
                let env = if ctx.use_environment_names {
                    xkb_context_getenv("XKB_DEFAULT_LAYOUT")
                } else {
                    Err(VarError::NotPresent)
                };
                rmlvo.layout = match env {
                    Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
                    Err(_) => std::ffi::CString::new("us").unwrap(),
                };
            }
            modified = (modified as u32 | RMLVO_LAYOUT) as RMLVO;
            let variant: std::ffi::CString = {
                let layout = xkb_context_getenv("XKB_DEFAULT_LAYOUT");
                let default_variant = xkb_context_getenv("XKB_DEFAULT_VARIANT");
                match (layout, ctx.use_environment_names, default_variant) {
                    (Ok(_), true, Ok(default_variant)) => {
                        std::ffi::CString::new(default_variant).unwrap_or_default()
                    }
                    (_, _, _) => std::ffi::CString::new("").unwrap(),
                }
            };
            if !rmlvo.variant.as_bytes().is_empty() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Layout not provided, but variant set to \"{}\": ignoring variant and using defaults for both: layout=\"{}\", variant=\"{}\".\n",
                    rmlvo.variant.to_str().unwrap_or(""),
                    rmlvo.layout.to_str().unwrap_or(""),
                    variant.to_str().unwrap_or(""),
                );
            }
            rmlvo.variant = variant;
            modified = (modified as u32 | RMLVO_VARIANT) as RMLVO;
        }
        if rmlvo.options.as_bytes().is_empty() {
            if ctx.use_environment_names {
                let env = xkb_context_getenv("XKB_DEFAULT_OPTIONS");
                rmlvo.options = match env {
                    Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
                    Err(_) => std::ffi::CString::new("").unwrap(),
                };
            } else {
                rmlvo.options = std::ffi::CString::new("").unwrap();
            };
            modified = (modified as u32 | RMLVO_OPTIONS) as RMLVO;
        }
        modified
    }
}

use crate::xkb::shared_types::*;
