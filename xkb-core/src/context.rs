use std::env::VarError;

use crate::atom::{atom_intern, atom_table_new, atom_text};

pub use crate::messages::{XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_LOG_VERBOSITY_DEFAULT};
use crate::shared_types::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use crate::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
fn context_include_path_append(ctx: &mut xkb_context, path: &str) -> i32 {
    let is_dir = std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false);
    if is_dir {
        log::info!("Include path added: {}\n", path);
        ctx.includes.push(path.to_string());
        return 1_i32;
    }
    if !path.is_empty() {
        ctx.failed_includes.push(path.to_string());
    }
    log::info!("Include path failed: \"{}\"\n", path);
    0_i32
}

pub fn xkb_context_include_path_get_extra_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_EXTRA_PATH") {
        Ok(extra) => extra,
        Err(_) => DFLT_XKB_CONFIG_EXTRA_PATH.to_string(),
    }
}

pub fn xkb_context_include_path_get_unversioned_extensions_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.to_string(),
    }
}

pub fn xkb_context_include_path_get_versioned_extensions_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_VERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.to_string(),
    }
}
/// Convert a null-terminated `[i8]` constant to a Rust `String`.
fn add_direct_subdirectories(
    ctx: &mut xkb_context,
    path: &str,
    extensions: &mut Vec<String>,
    versioned_count: usize,
    versioned_path_length: usize,
) -> i32 {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(e) => {
            log::debug!("Include extensions path failed: {} ({})\n", path, e);
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
        for ext in extensions.iter().take(versioned_count) {
            if name_offset <= ext.len() && *name_str == ext[name_offset..] {
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
        for ext in extensions.iter().skip(versioned_count) {
            ret |= context_include_path_append(ctx, ext);
        }
    }

    ret
}

pub fn xkb_context_include_path_get_system_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_ROOT") {
        Ok(root) => root,
        Err(_) => DFLT_XKB_CONFIG_ROOT.to_string(),
    }
}

pub fn xkb_context_include_path_append_default(ctx: &mut xkb_context) -> i32 {
    {
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
        let extra = xkb_context_include_path_get_extra_path();
        ret |= context_include_path_append(ctx, &extra);

        let mut extensions: Vec<String> = Vec::new();
        let versioned_path = xkb_context_include_path_get_versioned_extensions_path();
        let mut versioned_path_length: usize = 0;
        if !versioned_path.is_empty() {
            ret |= add_direct_subdirectories(ctx, &versioned_path, &mut extensions, 0, 0);
            versioned_path_length = versioned_path.len();
        }
        let unversioned_path = xkb_context_include_path_get_unversioned_extensions_path();
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

        let root = xkb_context_include_path_get_system_path();
        let has_root: bool = context_include_path_append(ctx, &root) != 0;
        ret |= has_root as i32;
        if !has_root && !root.is_empty() {
            let legacy = DFLT_XKB_LEGACY_ROOT.to_string();
            log::warn!("Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                "/usr/share/X11/xkb",
                root);
            ret |= context_include_path_append(ctx, &legacy);
        }
        ret
    }
}

pub fn xkb_context_include_path_clear(ctx: &mut xkb_context) {
    ctx.includes.clear();
    ctx.failed_includes.clear();
    ctx.pending_default_includes = false;
}

pub fn xkb_context_num_include_paths(ctx: &mut xkb_context) -> u32 {
    if xkb_context_init_includes(ctx) {
        ctx.includes.len() as u32
    } else {
        0_u32
    }
}
pub fn xkb_context_include_path_get(ctx: &mut xkb_context, idx: u32) -> String {
    if idx >= xkb_context_num_include_paths(ctx) {
        return "".to_string();
    }
    ctx.includes.get(idx as usize).unwrap().clone()
}

fn log_level_from_str(level: &str) -> u32 {
    let bytes = level.as_bytes();
    // Try parsing as integer first
    if let Ok(val) = level.trim().parse::<i64>() {
        return val as u32;
    }
    if bytes
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"crit"))
    {
        return XKB_LOG_LEVEL_CRITICAL;
    }
    if bytes
        .get(..3)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"err"))
    {
        return XKB_LOG_LEVEL_ERROR;
    }
    if bytes
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"warn"))
    {
        return XKB_LOG_LEVEL_WARNING;
    }
    if bytes
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"info"))
    {
        return XKB_LOG_LEVEL_INFO;
    }
    if bytes
        .get(..5)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"debug"))
        || bytes
            .get(..3)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"dbg"))
    {
        return XKB_LOG_LEVEL_DEBUG;
    }
    XKB_LOG_LEVEL_ERROR
}
fn log_verbosity_from_str(verbosity: &str) -> i32 {
    if let Ok(val) = verbosity.trim().parse::<i64>() {
        return val as i32;
    }
    XKB_LOG_VERBOSITY_DEFAULT
}
pub fn xkb_context_new(flags: xkb_context_flags) -> xkb_context {
    let mut ctx = xkb_context {
        refcnt: 1,
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
        use_environment_names: false,
        use_secure_getenv: false,
        pending_default_includes: false,
    };
    const XKB_CONTEXT_ALL_FLAGS: xkb_context_flags = (XKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
        | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32
        | XKB_CONTEXT_NO_SECURE_GETENV as i32)
        as xkb_context_flags;
    if flags & !XKB_CONTEXT_ALL_FLAGS != 0 {
        log::error!(
            "Invalid context flags: 0x{:x}\n",
            flags & !XKB_CONTEXT_ALL_FLAGS
        );
        return ctx;
    }
    ctx.use_environment_names = flags & XKB_CONTEXT_NO_ENVIRONMENT_NAMES == 0;
    ctx.use_secure_getenv = flags & XKB_CONTEXT_NO_SECURE_GETENV == 0;
    ctx.pending_default_includes = flags & XKB_CONTEXT_NO_DEFAULT_INCLUDES == 0;
    if let Ok(env) = xkb_context_getenv("XKB_LOG_LEVEL") {
        ctx.log_level = log_level_from_str(&env);
    }
    if let Ok(env) = xkb_context_getenv("XKB_LOG_VERBOSITY") {
        ctx.log_verbosity = log_verbosity_from_str(&env);
    }
    ctx
}

pub fn xkb_context_get_log_verbosity(ctx: &xkb_context) -> i32 {
    ctx.log_verbosity
}

// --- Merged from context_priv.rs ---

pub fn xkb_context_getenv(name: &str) -> Result<String, VarError> {
    std::env::var(name)
}
pub fn xkb_context_init_includes(ctx: &mut xkb_context) -> bool {
    if ctx.pending_default_includes {
        if ctx.failed_includes.is_empty() {
            if xkb_context_include_path_append_default(ctx) == 0 {
                log::error!(
                    "[XKB-{:03}] Failed to add any default include path (system path: {})\n",
                    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                    xkb_context_include_path_get_system_path()
                );
                return false;
            }
            ctx.pending_default_includes = false;
        } else {
            return false;
        }
    }
    true
}
pub fn xkb_context_num_failed_include_paths(ctx: &mut xkb_context) -> u32 {
    if xkb_context_init_includes(ctx) {
        ctx.failed_includes.len() as u32
    } else {
        0_u32
    }
}
pub fn xkb_context_failed_include_path_get(ctx: &mut xkb_context, idx: u32) -> String {
    if idx >= xkb_context_num_failed_include_paths(ctx) {
        return "".to_string();
    }
    ctx.failed_includes.get(idx as usize).unwrap().clone()
}

pub fn xkb_atom_intern_bytes(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
pub fn xkb_atom_intern(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
pub fn xkb_atom_intern_ref(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
pub fn xkb_atom_text(atom_table: &atom_table, atom: u32) -> &str {
    atom_text(atom_table, atom)
}

pub fn xkb_context_sanitize_rule_names(ctx: &xkb_context, rmlvo: &mut xkb_rule_names) -> RMLVO {
    let mut modified: RMLVO = 0 as RMLVO;
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
            log::warn!("Layout not provided, but variant set to \"{}\": ignoring variant and using defaults for both: layout=\"{}\", variant=\"{}\".\n",
                    rmlvo.variant.to_str().unwrap_or(""),
                    rmlvo.layout.to_str().unwrap_or(""),
                    variant.to_str().unwrap_or(""));
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

use crate::shared_types::*;
