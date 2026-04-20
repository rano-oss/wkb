use crate::xkb::context::{
    xkb_context_failed_include_path_get, xkb_context_getenv, xkb_context_num_failed_include_paths,
};
use crate::xkb::context::{xkb_context_include_path_get, xkb_context_num_include_paths};
use crate::xkb::context::{
    xkb_context_include_path_get_extra_path, xkb_context_include_path_get_system_path,
};

pub const INCLUDE_MAX_DEPTH: i32 = 15_i32;
pub const MERGE_OVERRIDE_PREFIX: i32 = '+' as i32;
pub const MERGE_AUGMENT_PREFIX: i32 = '|' as i32;
pub const MERGE_REPLACE_PREFIX: i32 = '^' as i32;

pub use crate::xkb::messages::{
    _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
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
    XKB_WARNING_CANNOT_INFER_KEY_TYPE,
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
pub use crate::xkb::scanner_utils::{scanner, scanner_loc};
use crate::xkb::shared_ast_types::FreeXkbFile;
pub use crate::xkb::shared_ast_types::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, xkb_file_type_to_string, xkb_map_flags,
    IncludeStmt, ParseCommon, XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES,
    FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER,
    MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT,
    MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
    STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL,
    STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL,
    STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL,
    STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS,
    STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME,
    STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
use crate::xkb::xkbcomp::scanner::XkbParseFile;

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
        if !rest_paren.ends_with(')') || rest_paren.len() < 1 {
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
    "keycodes",
    "types",
    "compat",
    "symbols",
    "geometry",
    "keymap",
    "rules",
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
        log::error!("[XKB-{:03}] {} include paths searched:\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            num);
        for i in 0..num {
            log::error!("[XKB-{:03}] \t{}\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_include_path_get(ctx, i));
        }
    } else {
        log::error!("[XKB-{:03}] There are no include paths to search\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32);
    }
    let num_failed = xkb_context_num_failed_include_paths(ctx);
    if num_failed > 0_u32 {
        log::error!("[XKB-{:03}] {} include paths could not be added:\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            num_failed);
        for i in 0..num_failed {
            log::error!("[XKB-{:03}] \t{}\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_failed_include_path_get(ctx, i));
        }
    }
}
/// Expand `%H`, `%S`, `%E`, `%%` in the given name string.
/// Returns `Some(expanded)` on success, `None` on error.
fn expand_percent(
    parent_file_name: &str,
    type_dir: &str,
    name: &str,
) -> Option<String> {
    let max_len = 4096usize;
    let mut result = String::new();
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => result.push('%'),
                Some('H') => {
                    match xkb_context_getenv("HOME") {
                        Ok(home) => result.push_str(&home),
                        Err(_) => {
                            log::error!("{}: %H was used in an include statement, but the HOME environment variable is not set\n",
                                parent_file_name);
                            return None;
                        }
                    }
                }
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
                    log::error!("[XKB-{:03}] {}: unknown % format ({}) in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        parent_file_name,
                        other);
                    return None;
                }
                None => {
                    log::error!("[XKB-{:03}] {}: trailing %% in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        parent_file_name);
                    return None;
                }
            }
        } else {
            result.push(c);
        }
        if result.len() > max_len {
            log::error!("[XKB-{:03}] {}: include path after expansion is too long\n",
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                parent_file_name);
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
            log::error!("[XKB-{:03}] Path is too long: expected max length of {}, got: {}\n",
                XKB_ERROR_INVALID_PATH as i32,
                4096,
                &path);
        } else if let Ok(file) = std::fs::File::open(&path) {
            *offset = i;
            return Some((file, path));
        }
        i += 1;
    }
    if required && *offset == 0 {
        log::error!("[XKB-{:03}] Couldn't find file \"{}/{}\" in include paths\n",
            XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            type_dir,
            name);
        LogIncludePaths(ctx);
    }
    None
}
pub fn ExceedsIncludeMaxDepth(include_depth: u32) -> bool {
    if include_depth >= INCLUDE_MAX_DEPTH as u32 {
        log::error!("[XKB-{:03}] Exceeded include depth threshold ({})",
            XKB_ERROR_RECURSIVE_INCLUDE as i32,
            15_i32);
        true
    } else {
        false
    }
}
pub fn ProcessIncludeFile(
    ctx: &mut xkb_context,
    stmt: &IncludeStmt,
    file_type: u32,
) -> *mut XkbFile {
    let mut xkb_file: *mut XkbFile = std::ptr::null_mut();
    let mut candidate: *mut XkbFile = std::ptr::null_mut();

    // Expand %-sequences in the file name
    let stmt_file: String = match expand_path_str("(unknown)", &stmt.file, file_type) {
        Err(()) => return std::ptr::null_mut(),
        Ok(Some(expanded)) => expanded,
        Ok(None) => stmt.file.clone(),
    };
    let expanded = stmt_file != stmt.file;

    let map_cstr = if stmt.map.is_empty() {
        None
    } else {
        Some(std::ffi::CString::new(stmt.map.as_str()).unwrap())
    };
    let map_ptr = map_cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());

    let absolute_path = stmt_file.starts_with('/');
    let ctx_ptr: *mut xkb_context = ctx;
    let mut offset: u32 = 0;
    let mut file_and_path: Option<(std::fs::File, String)> = if absolute_path {
        std::fs::File::open(&stmt_file).ok().map(|f| (f, stmt_file.clone()))
    } else if expanded {
        // Expanded but not absolute — don't search include paths
        None
    } else {
        FindFileInXkbPath(
            ctx,
            "(unknown)",
            &stmt_file,
            file_type,
            &mut offset,
            true,
        )
    };

    while let Some((ref open_file, ref _path)) = file_and_path {
        xkb_file = unsafe { XkbParseFile(ctx_ptr, open_file, &stmt.file, map_ptr) };
        // Drop the file (closes it)
        let _ = file_and_path.take();

        if !xkb_file.is_null() {
            unsafe {
                if (*xkb_file).file_type as u32 != file_type {
                    log::error!("[XKB-{:03}] Include file of wrong type (expected {}, got {}); Include file \"{}\" ignored\n",
                        XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                        xkb_file_type_to_string(file_type),
                        xkb_file_type_to_string((*xkb_file).file_type),
                        &stmt.file);
                    FreeXkbFile(xkb_file);
                    xkb_file = std::ptr::null_mut();
                } else if !stmt.map.is_empty()
                    || (*xkb_file).flags as u32 != 0 && MAP_IS_DEFAULT as i32 != 0
                {
                    break;
                } else if candidate.is_null() {
                    candidate = xkb_file;
                    xkb_file = std::ptr::null_mut();
                } else {
                    FreeXkbFile(xkb_file);
                    xkb_file = std::ptr::null_mut();
                }
            }
        }
        if absolute_path {
            break;
        }
        offset += 1;
        file_and_path = FindFileInXkbPath(
            unsafe { &mut *ctx_ptr },
            "(unknown)",
            &stmt_file,
            file_type,
            &mut offset,
            true,
        );
    }

    unsafe {
        if xkb_file.is_null() {
            xkb_file = candidate;
        } else {
            FreeXkbFile(candidate);
        }
        if xkb_file.is_null() {
            if !stmt.map.is_empty() {
                log::error!("[XKB-{:03}] Couldn't process include statement for '{}({})'\n",
                    XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                    &stmt.file,
                    &stmt.map);
            } else {
                log::error!("[XKB-{:03}] Couldn't process include statement for '{}'\n",
                    XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                    &stmt.file);
            }
        }
        xkb_file
    }
}
use crate::xkb::shared_types::*;
