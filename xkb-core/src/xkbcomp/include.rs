use crate::context::{
    xkb_context_failed_include_path_get, xkb_context_getenv, xkb_context_num_failed_include_paths,
};
use crate::context::{xkb_context_include_path_get, xkb_context_num_include_paths};
use crate::context::{
    xkb_context_include_path_get_extra_path, xkb_context_include_path_get_system_path,
};

pub const INCLUDE_MAX_DEPTH: i32 = 15_i32;
pub const MERGE_OVERRIDE_PREFIX: i32 = '+' as i32;
pub const MERGE_AUGMENT_PREFIX: i32 = '|' as i32;
pub const MERGE_REPLACE_PREFIX: i32 = '^' as i32;

pub use crate::messages::{
    XKB_ERROR_INCLUDED_FILE_NOT_FOUND,
    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE, XKB_ERROR_INVALID_INCLUDED_FILE, XKB_ERROR_INVALID_PATH,
    XKB_ERROR_RECURSIVE_INCLUDE,
};
pub use crate::shared_ast_types::{
    xkb_file_type_to_string, IncludeStmt,
    XkbFile, MAP_IS_DEFAULT,
};
use crate::xkbcomp::scanner::XkbParseFile;

/// Parsed result from one segment of an include statement.
pub struct ParsedIncludeMap {
    pub file: String,
    pub map: String,
    pub extra_data: String,
    pub nextop: char,
}

/// Parse one include map segment from `input`, returning the parsed result
/// and the remaining input (if any). Returns None on parse error.
pub fn ParseIncludeMap(input: &str) -> Option<(ParsedIncludeMap, Option<&str>)> {
    // Split at merge-mode prefix (+, |, ^)
    let (segment, nextop, rest) = if let Some(pos) = input.find(&['+', '|', '^'][..]) {
        let op = input.as_bytes()[pos] as char;
        (&input[..pos], op, Some(&input[pos + 1..]))
    } else {
        (input, '\0', None)
    };

    // Split off extra_data after ':'
    let (segment, extra_data) = if let Some(pos) = segment.find(':') {
        (&segment[..pos], segment[pos + 1..].to_string())
    } else {
        (segment, String::new())
    };

    // Parse file(map) pattern
    let (file, map) = if let Some(pos) = segment.find('(') {
        if pos == 0 {
            return None; // starts with '(' — invalid
        }
        let rest_paren = &segment[pos + 1..];
        if !rest_paren.ends_with(')') || rest_paren.is_empty() {
            return None;
        }
        let map_str = &rest_paren[..rest_paren.len() - 1];
        (segment[..pos].to_string(), map_str.to_string())
    } else {
        (segment.to_string(), String::new())
    };

    // Validate nextop
    if nextop != '\0' && nextop != '+' && nextop != '|' && nextop != '^' {
        return None;
    }

    Some((
        ParsedIncludeMap {
            file,
            map,
            extra_data,
            nextop,
        },
        rest,
    ))
}
static XKB_FILE_TYPE_INCLUDE_DIRS: [&str; 7] = [
    "keycodes", "types", "compat", "symbols", "geometry", "keymap", "rules",
];
fn DirectoryForInclude(type_0: u32) -> &'static str {
    if type_0 as usize >= XKB_FILE_TYPE_INCLUDE_DIRS.len() {
        ""
    } else {
        XKB_FILE_TYPE_INCLUDE_DIRS[type_0 as usize]
    }
}
fn LogIncludePaths(ctx: &mut xkb_context) {
    let num = xkb_context_num_include_paths(ctx);
    if num > 0_u32 {
        log::error!(
            "[XKB-{:03}] {} include paths searched:\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            num
        );
        for i in 0..num {
            log::error!(
                "[XKB-{:03}] \t{}\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_include_path_get(ctx, i)
            );
        }
    } else {
        log::error!(
            "[XKB-{:03}] There are no include paths to search\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32
        );
    }
    let num_failed = xkb_context_num_failed_include_paths(ctx);
    if num_failed > 0_u32 {
        log::error!(
            "[XKB-{:03}] {} include paths could not be added:\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            num_failed
        );
        for i in 0..num_failed {
            log::error!(
                "[XKB-{:03}] \t{}\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_failed_include_path_get(ctx, i)
            );
        }
    }
}
/// Expand `%H`, `%S`, `%E`, `%%` in the given name string.
/// Returns `Some(expanded)` on success, `None` on error.
fn expand_percent(parent_file_name: &str, type_dir: &str, name: &str) -> Option<String> {
    let max_len = 4096usize;
    let mut result = String::new();
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => result.push('%'),
                Some('H') => match xkb_context_getenv("HOME") {
                    Ok(home) => result.push_str(&home),
                    Err(_) => {
                        log::error!("{}: %H was used in an include statement, but the HOME environment variable is not set\n",
                                parent_file_name);
                        return None;
                    }
                },
                Some('S') => {
                    let sys = xkb_context_include_path_get_system_path();
                    result.push_str(&sys);
                    result.push('/');
                    result.push_str(type_dir);
                }
                Some('E') => {
                    let extra = xkb_context_include_path_get_extra_path();
                    result.push_str(&extra);
                    result.push('/');
                    result.push_str(type_dir);
                }
                Some(other) => {
                    log::error!(
                        "[XKB-{:03}] {}: unknown % format ({}) in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        parent_file_name,
                        other
                    );
                    return None;
                }
                None => {
                    log::error!(
                        "[XKB-{:03}] {}: trailing %% in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        parent_file_name
                    );
                    return None;
                }
            }
        } else {
            result.push(c);
        }
        if result.len() > max_len {
            log::error!(
                "[XKB-{:03}] {}: include path after expansion is too long\n",
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                parent_file_name
            );
            return None;
        }
    }
    Some(result)
}
/// Expand `%`-sequences in `name`. Returns:
/// - `Ok(None)` if no `%` found (no expansion needed)
/// - `Ok(Some(expanded))` if expansion succeeded
/// - `Err(())` on error
pub fn expand_path_str(
    parent_file_name: &str,
    name: &str,
    file_type: u32,
) -> Result<Option<String>, ()> {
    // Find first '%'
    let k = match name.find('%') {
        Some(pos) => pos,
        None => return Ok(None),
    };
    let type_dir = DirectoryForInclude(file_type);
    let prefix = &name[..k];
    let rest = &name[k..];
    match expand_percent(parent_file_name, type_dir, rest) {
        Some(expanded) => {
            let mut result = String::with_capacity(prefix.len() + expanded.len());
            result.push_str(prefix);
            result.push_str(&expanded);
            Ok(Some(result))
        }
        None => Err(()),
    }
}
pub fn FindFileInXkbPath(
    ctx: &mut xkb_context,
    _parent_file_name: &str,
    name: &str,
    type_0: u32,
    offset: &mut u32,
    required: bool,
) -> Option<(std::fs::File, String)> {
    let type_dir = DirectoryForInclude(type_0);
    let mut i: u32 = *offset;
    while i < xkb_context_num_include_paths(ctx) {
        let path = format!(
            "{}/{}/{}",
            xkb_context_include_path_get(ctx, i),
            type_dir,
            name
        );
        if path.len() >= 4096 {
            log::error!(
                "[XKB-{:03}] Path is too long: expected max length of {}, got: {}\n",
                XKB_ERROR_INVALID_PATH as i32,
                4096,
                &path
            );
        } else if let Ok(file) = std::fs::File::open(&path) {
            *offset = i;
            return Some((file, path));
        }
        i += 1;
    }
    if required && *offset == 0 {
        log::error!(
            "[XKB-{:03}] Couldn't find file \"{}/{}\" in include paths\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            type_dir,
            name
        );
        LogIncludePaths(ctx);
    }
    None
}
pub fn ExceedsIncludeMaxDepth(include_depth: u32) -> bool {
    if include_depth >= INCLUDE_MAX_DEPTH as u32 {
        log::error!(
            "[XKB-{:03}] Exceeded include depth threshold ({})",
            XKB_ERROR_RECURSIVE_INCLUDE as i32,
            15_i32
        );
        true
    } else {
        false
    }
}
pub fn ProcessIncludeFile(
    ctx: &mut xkb_context,
    stmt: &IncludeStmt,
    file_type: u32,
) -> Option<Box<XkbFile>> {
    let mut xkb_file: Option<Box<XkbFile>> = None;
    let mut candidate: Option<Box<XkbFile>> = None;

    // Expand %-sequences in the file name
    let stmt_file: String = match expand_path_str("(unknown)", &stmt.file, file_type) {
        Err(()) => return None,
        Ok(Some(expanded)) => expanded,
        Ok(None) => stmt.file.clone(),
    };
    let expanded = stmt_file != stmt.file;

    let map_str = if stmt.map.is_empty() { "" } else { &stmt.map };

    let absolute_path = stmt_file.starts_with('/');
    let mut offset: u32 = 0;
    let mut file_and_path: Option<(std::fs::File, String)> = if absolute_path {
        std::fs::File::open(&stmt_file)
            .ok()
            .map(|f| (f, stmt_file.clone()))
    } else if expanded {
        // Expanded but not absolute — don't search include paths
        None
    } else {
        FindFileInXkbPath(ctx, "(unknown)", &stmt_file, file_type, &mut offset, true)
    };

    while let Some((ref open_file, ref _path)) = file_and_path {
        if let Some(parsed) = XkbParseFile(ctx, open_file, &stmt.file, map_str) {
            // Drop the file (closes it)
            let _ = file_and_path.take();

            if parsed.file_type != file_type {
                log::error!("[XKB-{:03}] Include file of wrong type (expected {}, got {}); Include file \"{}\" ignored\n",
                    XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                    xkb_file_type_to_string(file_type),
                    xkb_file_type_to_string(parsed.file_type),
                    &stmt.file);
                // parsed drops automatically
            } else if !stmt.map.is_empty() || parsed.flags != 0 && MAP_IS_DEFAULT as i32 != 0 {
                xkb_file = Some(parsed);
                break;
            } else if candidate.is_none() {
                candidate = Some(parsed);
            }
            // else: parsed drops automatically (was FreeXkbFile)
        } else {
            // Drop the file (closes it)
            let _ = file_and_path.take();
        }
        if absolute_path {
            break;
        }
        offset += 1;
        file_and_path =
            FindFileInXkbPath(ctx, "(unknown)", &stmt_file, file_type, &mut offset, true);
    }

    if xkb_file.is_none() {
        xkb_file = candidate;
    }
    // else: candidate drops automatically

    if xkb_file.is_none() {
        if !stmt.map.is_empty() {
            log::error!(
                "[XKB-{:03}] Couldn't process include statement for '{}({})'\n",
                XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                &stmt.file,
                &stmt.map
            );
        } else {
            log::error!(
                "[XKB-{:03}] Couldn't process include statement for '{}'\n",
                XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                &stmt.file
            );
        }
    }
    xkb_file
}
use crate::shared_types::*;
