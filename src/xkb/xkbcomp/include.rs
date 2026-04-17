use crate::xkb::context::{
    xkb_context_failed_include_path_get, xkb_context_getenv, xkb_context_num_failed_include_paths,
};
use crate::xkb::context::{xkb_context_include_path_get, xkb_context_num_include_paths};
use crate::xkb::context::{
    xkb_context_include_path_get_extra_path, xkb_context_include_path_get_system_path,
};
use crate::xkb_logf;

pub const INCLUDE_MAX_DEPTH: i32 = 15_i32;
pub const MERGE_OVERRIDE_PREFIX: i32 = '+' as i32;
pub const MERGE_AUGMENT_PREFIX: i32 = '|' as i32;
pub const MERGE_REPLACE_PREFIX: i32 = '^' as i32;
pub static mut MERGE_MODE_PREFIXES: [i8; 4] = [
    MERGE_OVERRIDE_PREFIX as i8,
    MERGE_AUGMENT_PREFIX as i8,
    MERGE_REPLACE_PREFIX as i8,
    0_i32 as i8,
];

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
use crate::xkb::utils::cstr_dup;
use crate::xkb::utils::is_absolute_path;
use crate::xkb::utils::{cstr_free, cstr_len};
use crate::xkb::xkbcomp::scanner::XkbParseFile;
use libc::{fclose, fopen, FILE};
pub unsafe fn ParseIncludeMap(
    str_inout: *mut *mut i8,
    file_rtrn: *mut *mut i8,
    map_rtrn: *mut *mut i8,
    nextop_rtrn: *mut i8,
    extra_data: *mut *mut i8,
) -> bool {
    unsafe {
        let mut tmp: *mut i8;
        let mut str: *mut i8;
        let mut next: *mut i8;
        str = *str_inout;
        next = crate::xkb::utils::cstr_pbrk(str, &raw const MERGE_MODE_PREFIXES as *const i8);
        if !next.is_null() {
            *nextop_rtrn = *next;
            let c2rust_fresh2 = next;
            next = next.offset(1);
            *c2rust_fresh2 = '\0' as i32 as i8;
        } else {
            *nextop_rtrn = '\0' as i32 as i8;
            next = std::ptr::null_mut();
        }
        tmp = crate::xkb::utils::cstr_chr(str, ':' as i32);
        if !tmp.is_null() {
            let c2rust_fresh3 = tmp;
            tmp = tmp.offset(1);
            *c2rust_fresh3 = '\0' as i32 as i8;
            *extra_data = cstr_dup(tmp);
        } else {
            *extra_data = std::ptr::null_mut();
        }
        tmp = crate::xkb::utils::cstr_chr(str, '(' as i32);
        if tmp.is_null() {
            *file_rtrn = cstr_dup(str);
            *map_rtrn = std::ptr::null_mut();
        } else if *str.offset(0_i32 as isize) as i32 == '(' as i32 {
            cstr_free(*extra_data);
            return false;
        } else {
            let c2rust_fresh4 = tmp;
            tmp = tmp.offset(1);
            *c2rust_fresh4 = '\0' as i32 as i8;
            *file_rtrn = cstr_dup(str);
            str = tmp;
            tmp = crate::xkb::utils::cstr_chr(str, ')' as i32);
            if tmp.is_null() || *tmp.offset(1_i32 as isize) as i32 != '\0' as i32 {
                cstr_free(*file_rtrn);
                cstr_free(*extra_data);
                return false;
            }
            let c2rust_fresh5 = tmp;
            // dead store removed: tmp = tmp.offset(1);
            *c2rust_fresh5 = '\0' as i32 as i8;
            *map_rtrn = cstr_dup(str);
        }
        if *nextop_rtrn as i32 == '\0' as i32 {
            *str_inout = std::ptr::null_mut();
        } else if *nextop_rtrn as i32 == MERGE_OVERRIDE_PREFIX
            || *nextop_rtrn as i32 == MERGE_AUGMENT_PREFIX
            || *nextop_rtrn as i32 == MERGE_REPLACE_PREFIX
        {
            *str_inout = next;
        } else {
            return false;
        }
        true
    }
}
static mut XKB_FILE_TYPE_INCLUDE_DIRS: [*const i8; 7] = [
    b"keycodes\0".as_ptr() as *const i8,
    b"types\0".as_ptr() as *const i8,
    b"compat\0".as_ptr() as *const i8,
    b"symbols\0".as_ptr() as *const i8,
    b"geometry\0".as_ptr() as *const i8,
    b"keymap\0".as_ptr() as *const i8,
    b"rules\0".as_ptr() as *const i8,
];
unsafe fn DirectoryForInclude(type_0: u32) -> *const i8 {
    unsafe {
        if type_0 >= _FILE_TYPE_NUM_ENTRIES {
            return b"\0".as_ptr() as *const i8;
        }
        XKB_FILE_TYPE_INCLUDE_DIRS[type_0 as usize]
    }
}
unsafe fn LogIncludePaths(ctx: *mut xkb_context) {
    unsafe {
        if xkb_context_num_include_paths(ctx) > 0_u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {} include paths searched:\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_num_include_paths(ctx),
            );
            let mut i: u32 = 0_u32;
            while i < xkb_context_num_include_paths(ctx) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] \t{}\n",
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                    xkb_context_include_path_get(ctx, i),
                );
                i = i.wrapping_add(1);
            }
        } else {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] There are no include paths to search\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
            );
        }
        if xkb_context_num_failed_include_paths(ctx) > 0_u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {} include paths could not be added:\n",
                XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                xkb_context_num_failed_include_paths(ctx),
            );
            let mut i_0: u32 = 0_u32;
            while i_0 < xkb_context_num_failed_include_paths(ctx) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] \t{}\n",
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                    xkb_context_failed_include_path_get(ctx, i_0),
                );
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
}
unsafe fn expand_percent(
    ctx: *mut xkb_context,
    parent_file_name: *const i8,
    typeDir: *const i8,
    buf: *mut i8,
    buf_size: usize,
    name: *const i8,
    name_len: usize,
) -> usize {
    unsafe {
        let mut s = scanner::new(ctx, name, name_len, parent_file_name, std::ptr::null_mut());
        s.buf_pos = 0;
        while !s.eof() && !s.eol() {
            if s.chr(b'%' as i8) {
                if s.chr(b'%' as i8) {
                    s.buf_append(b'%');
                } else if s.chr(b'H' as i8) {
                    let home = xkb_context_getenv("HOME");
                    match &home {
                        Ok(home) => {
                            if !s.buf_appends(home.as_ptr()) {
                                let loc = s.token_location();
                                xkb_logf!(
                                    s.ctx,
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] {}:{}:{}: include path after expanding %H is too long\n",
                                    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                                    crate::xkb::utils::CStrDisplay(s.file_name),
                                    loc.line,
                                    loc.column,
                                );
                                return 0;
                            }
                        }
                        Err(_) => todo!(),
                    }
                    if home.is_err() {
                        let loc = s.token_location();
                        xkb_logf!(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "{}:{}:{}: %H was used in an include statement, but the HOME environment variable is not set\n",
                            crate::xkb::utils::CStrDisplay(s.file_name),
                            loc.line,
                            loc.column,
                        );
                        return 0;
                    }
                } else if s.chr(b'S' as i8) {
                    let default_root_str = xkb_context_include_path_get_system_path(ctx);
                    let default_root_c = std::ffi::CString::new(default_root_str).unwrap();
                    if !s.buf_appends(default_root_c.as_ptr() as *const u8)
                        || !s.buf_append(b'/')
                        || !s.buf_appends(typeDir as *const u8)
                    {
                        let loc = s.token_location();
                        xkb_logf!(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: include path after expanding %S is too long\n",
                            XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                            crate::xkb::utils::CStrDisplay(s.file_name),
                            loc.line,
                            loc.column,
                        );
                        return 0;
                    }
                } else if s.chr(b'E' as i8) {
                    let default_root_str = xkb_context_include_path_get_extra_path(ctx);
                    let default_root_c = std::ffi::CString::new(default_root_str).unwrap();
                    if !s.buf_appends(default_root_c.as_ptr() as *const u8)
                        || !s.buf_append(b'/')
                        || !s.buf_appends(typeDir as *const u8)
                    {
                        let loc = s.token_location();
                        xkb_logf!(
                            s.ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: include path after expanding %E is too long\n",
                            XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                            crate::xkb::utils::CStrDisplay(s.file_name),
                            loc.line,
                            loc.column,
                        );
                        return 0;
                    }
                } else {
                    let loc = s.token_location();
                    xkb_logf!(
                        s.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] {}:{}:{}: unknown % format ({}) in include statement\n",
                        XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                        crate::xkb::utils::CStrDisplay(s.file_name),
                        loc.line,
                        loc.column,
                        (s.peek() as u8 as char),
                    );
                    return 0;
                }
            } else {
                let c = s.next_byte();
                s.buf_append(c as u8);
            }
        }
        if !s.buf_append(0) {
            let loc = s.token_location();
            xkb_logf!(
                s.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: include path is too long; max: {}\n",
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                crate::xkb::utils::CStrDisplay(s.file_name),
                loc.line,
                loc.column,
                std::mem::size_of::<[i8; 1024]>(),
            );
            return 0;
        }
        if s.buf_pos > buf_size {
            let loc = s.token_location();
            xkb_logf!(
                s.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: include path is too long: {} > {}\n",
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
                crate::xkb::utils::CStrDisplay(s.file_name),
                loc.line,
                loc.column,
                s.buf_pos,
                buf_size,
            );
            return 0;
        }
        std::ptr::copy_nonoverlapping(&raw mut s.buf as *const u8, buf as *mut u8, s.buf_pos);
        s.buf_pos
    }
}
pub unsafe fn expand_path(
    ctx: *mut xkb_context,
    parent_file_name: *const i8,
    name: *const i8,
    name_len: usize,
    type_0: u32,
    buf: *mut i8,
    buf_size: usize,
) -> isize {
    unsafe {
        let c2rust_current_block: u64;
        let mut k: usize;
        k = 0_usize;
        loop {
            if k >= name_len {
                c2rust_current_block = 17179679302217393232;
                break;
            }
            if *name.add(k) as i32 == '%' as i32 {
                c2rust_current_block = 15593259132448327734;
                break;
            }
            k = k.wrapping_add(1);
        }
        match c2rust_current_block {
            17179679302217393232 => 0_isize,
            _ => {
                if (k >= buf_size) as i64 != 0 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Path is too long: {} > {}, got raw path: {}\n",
                        XKB_ERROR_INVALID_PATH as i32,
                        k,
                        buf_size,
                        crate::xkb::utils::CStrNDisplay(name_len, name),
                    );
                    return -1_i32 as isize;
                }
                std::ptr::copy_nonoverlapping(name as *const u8, buf as *mut u8, k);
                let typeDir: *const i8 = DirectoryForInclude(type_0);
                let mut count: usize = expand_percent(
                    ctx,
                    parent_file_name,
                    typeDir,
                    buf.add(k),
                    buf_size.wrapping_sub(k),
                    name.add(k),
                    name_len.wrapping_sub(k),
                );
                if count == 0 {
                    return -1_i32 as isize;
                }
                count = count.wrapping_add(k);
                count as isize - 1_isize
            }
        }
    }
}
pub unsafe fn FindFileInXkbPath(
    ctx: *mut xkb_context,
    _parent_file_name: *const i8,
    name: *const i8,
    name_len: usize,
    type_0: u32,
    buf: *mut i8,
    buf_size: usize,
    offset: *mut u32,
    required: bool,
) -> *mut FILE {
    unsafe {
        let c2rust_current_block: u64;
        let mut file: *mut FILE = std::ptr::null_mut();
        let name_buffer: *mut i8 = std::ptr::null_mut();
        let typeDir: *const i8 = DirectoryForInclude(type_0);
        let mut i: u32 = *offset;
        loop {
            if i >= xkb_context_num_include_paths(ctx) {
                c2rust_current_block = 8515828400728868193;
                break;
            }
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                buf,
                buf_size,
                format_args!(
                    "{}/{}/{}",
                    xkb_context_include_path_get(ctx, i),
                    crate::xkb::utils::CStrDisplay(typeDir),
                    crate::xkb::utils::CStrNDisplay(name_len, name)
                ),
            );
            if _trunc {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Path is too long: expected max length of {}, got: {}/{}/{}\n",
                    XKB_ERROR_INVALID_PATH as i32,
                    buf_size,
                    xkb_context_include_path_get(ctx, i),
                    crate::xkb::utils::CStrDisplay(typeDir),
                    crate::xkb::utils::CStrNDisplay(name_len, name),
                );
            } else {
                file = fopen(buf, b"rb\0".as_ptr() as *const i8) as *mut FILE;
                if !file.is_null() {
                    *offset = i;
                    c2rust_current_block = 17619028831370153636;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        match c2rust_current_block {
            8515828400728868193 if required as i32 != 0 && *offset == 0_u32 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Couldn't find file \"{}/{}\" in include paths\n",
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                    crate::xkb::utils::CStrDisplay(typeDir),
                    crate::xkb::utils::CStrNDisplay(name_len, name),
                );
                LogIncludePaths(ctx);
            }
            _ => {}
        }
        cstr_free(name_buffer);
        file
    }
}
pub unsafe fn ExceedsIncludeMaxDepth(_ctx: *mut xkb_context, include_depth: u32) -> bool {
    if include_depth >= INCLUDE_MAX_DEPTH as u32 {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Exceeded include depth threshold ({})",
            XKB_ERROR_RECURSIVE_INCLUDE as i32,
            15_i32,
        );
        true
    } else {
        false
    }
}
pub unsafe fn ProcessIncludeFile(
    ctx: *mut xkb_context,
    stmt: *const IncludeStmt,
    file_type: u32,
    path: *mut i8,
    path_size: usize,
) -> *mut XkbFile {
    unsafe {
        let mut xkb_file: *mut XkbFile = std::ptr::null_mut();
        let mut candidate: *mut XkbFile = std::ptr::null_mut();
        let mut stmt_file = (*stmt).file;
        let mut stmt_file_len: usize = cstr_len(stmt_file);
        let expanded: isize = expand_path(
            ctx,
            b"(unknown)\0".as_ptr() as *const i8,
            stmt_file,
            stmt_file_len,
            file_type,
            path,
            path_size,
        );
        if expanded < 0_isize {
            return std::ptr::null_mut();
        } else if expanded > 0_isize {
            stmt_file = path;
            stmt_file_len = expanded as usize;
        }
        let mut file: *mut FILE;
        let mut offset: u32 = 0_u32;
        let absolute_path: bool = is_absolute_path(stmt_file);
        if absolute_path {
            file = fopen(stmt_file, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        } else if (expanded != 0) as i64 != 0 {
            file = std::ptr::null_mut();
        } else {
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                stmt_file,
                stmt_file_len,
                file_type,
                path,
                path_size,
                &raw mut offset,
                true,
            );
        }
        while !file.is_null() {
            xkb_file = XkbParseFile(ctx, file, (*stmt).file, (*stmt).map);
            fclose(file);
            if !xkb_file.is_null() {
                if (*xkb_file).file_type as u32 != file_type {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Include file of wrong type (expected {}, got {}); Include file \"{}\" ignored\n",
                        XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                        crate::xkb::utils::CStrDisplay(xkb_file_type_to_string(file_type)),
                        crate::xkb::utils::CStrDisplay(xkb_file_type_to_string((*xkb_file).file_type)),
                        crate::xkb::utils::CStrDisplay((*stmt).file),
                    );
                    FreeXkbFile(xkb_file);
                    xkb_file = std::ptr::null_mut();
                } else if !(*stmt).map.is_null()
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
            if absolute_path {
                break;
            }
            offset = offset.wrapping_add(1);
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                stmt_file,
                stmt_file_len,
                file_type,
                path,
                path_size,
                &raw mut offset,
                true,
            );
        }
        if xkb_file.is_null() {
            xkb_file = candidate;
        } else {
            FreeXkbFile(candidate);
        }
        if xkb_file.is_null() {
            if !(*stmt).map.is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Couldn't process include statement for '{}({})'\n",
                    XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                    crate::xkb::utils::CStrDisplay((*stmt).file),
                    crate::xkb::utils::CStrDisplay((*stmt).map),
                );
            } else {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Couldn't process include statement for '{}'\n",
                    XKB_ERROR_INVALID_INCLUDED_FILE as i32,
                    crate::xkb::utils::CStrDisplay((*stmt).file),
                );
            }
        }
        xkb_file
    }
}
use crate::xkb::shared_types::*;
