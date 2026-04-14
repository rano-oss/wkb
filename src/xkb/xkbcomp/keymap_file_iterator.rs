use crate::xkb_logf;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_file_include {
    pub valid: bool,
    pub explicit_section: bool,
    pub merge: merge_mode,
    pub path: u32,
    pub file: u32,
    pub section: u32,
    pub modifier: u32,
    pub flags: xkb_map_flags,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_file_include_group {
    pub start: u32,
    pub end: u32,
}
#[derive(Clone)]
pub struct xkb_file_section {
    pub name: u32,
    pub file_type: xkb_file_type,
    pub flags: xkb_map_flags,
    pub include_groups: Vec<xkb_file_include_group>,
    pub includes: Vec<xkb_file_include>,
    pub buffer: Vec<i8>,
}
pub type xkb_file_iterator_flags = u32;
pub const XKB_FILE_ITERATOR_NO_INCLUDES: xkb_file_iterator_flags = 2;
pub const XKB_FILE_ITERATOR_FAIL_ON_INCLUDE_ERROR: xkb_file_iterator_flags = 1;
pub const XKB_FILE_ITERATOR_NO_FLAG: xkb_file_iterator_flags = 0;
#[derive(Clone)]
pub struct xkb_file_iterator {
    pub flags: xkb_file_iterator_flags,
    pub finished: bool,
    pub path: *const i8,
    pub map: *const i8,
    pub type_0: xkb_file_type,
    pub scanner: scanner,
    pub section: xkb_file_section,
    pub pending_xkb_file: *mut XkbFile,
    pub pending_section: *mut XkbFile,
    pub ctx: *mut xkb_context,
}
use crate::xkb::scanner_utils::scanner;
use crate::xkb::shared_ast_types::{merge_mode, xkb_file_type, xkb_map_flags, XkbFile};
use crate::xkb::shared_types::xkb_context;

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
pub use crate::xkb::scanner_utils::scanner_loc;
use crate::xkb::shared_ast_types::FreeXkbFile;
pub use crate::xkb::shared_ast_types::{
    _IncludeStmt, _ParseCommon, stmt_type, xkb_file_type_to_string, IncludeStmt, ParseCommon,
    _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT,
    FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES,
    FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
    MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR,
    MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
    MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD,
    STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE,
    STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT,
    STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT,
    STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT,
    STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP,
    STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION,
    STMT_VAR, STMT_VMOD,
};
use crate::xkb::utils::cstr_len;
use crate::xkb::xkbcomp::include::ProcessIncludeFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub flag: xkb_map_flags,
    pub name: *const i8,
}

pub unsafe fn xkb_file_type_name(mut type_0: xkb_file_type) -> *const i8 {
    unsafe {
        if type_0 as u32 > FILE_TYPE_KEYMAP as u32 {
            return b"unknown\0".as_ptr() as *const i8;
        }
        static mut xkb_file_type_strings: [*const i8; 7] = [
            b"keycodes\0".as_ptr() as *const i8,
            b"types\0".as_ptr() as *const i8,
            b"compatibility\0".as_ptr() as *const i8,
            b"symbols\0".as_ptr() as *const i8,
            b"geometry\0".as_ptr() as *const i8,
            b"keymap\0".as_ptr() as *const i8,
            std::ptr::null(),
        ];
        return xkb_file_type_strings[type_0 as usize];
    }
}

pub unsafe fn xkb_file_section_init(mut section: *mut xkb_file_section) {
    unsafe {
        std::ptr::write(&raw mut (*section).include_groups, Vec::new());
        std::ptr::write(&raw mut (*section).includes, Vec::new());
        std::ptr::write(&raw mut (*section).buffer, vec!['\0' as i32 as i8]);
    }
}
unsafe fn xkb_file_section_reset(mut section: *mut xkb_file_section) {
    unsafe {
        (&mut (*section).include_groups).clear();
        (&mut (*section).includes).clear();
        (&mut (*section).buffer).truncate(1);
    }
}

pub unsafe fn xkb_file_section_free(mut section: *mut xkb_file_section) {
    unsafe {
        if section.is_null() {
            return;
        }
        std::ptr::drop_in_place(&raw mut (*section).include_groups);
        std::ptr::drop_in_place(&raw mut (*section).includes);
        std::ptr::drop_in_place(&raw mut (*section).buffer);
    }
}
unsafe fn xkb_file_section_set_meta_data(
    mut ctx: *mut xkb_context,
    mut section: *mut xkb_file_section,
    mut xkb_file: *const XkbFile,
) -> bool {
    unsafe {
        (*section).file_type = (*xkb_file).file_type;
        (*section).flags = (*xkb_file).flags;
        if !(*xkb_file).name.is_null() {
            let mut idx: u32 = (&(*section).buffer).len() as u32;
            let name_len = cstr_len((*xkb_file).name).wrapping_add(1) as usize;
            (&mut (*section).buffer)
                .extend_from_slice(std::slice::from_raw_parts((*xkb_file).name, name_len));
            (*section).name = idx;
        } else {
            (*section).name = 0 as u32;
        }
        return true;
    }
}
unsafe fn xkb_file_section_append_includes(
    mut ctx: *mut xkb_context,
    mut flags: xkb_file_iterator_flags,
    mut section_path: *const i8,
    mut section: *mut xkb_file_section,
    mut file_type: xkb_file_type,
    mut include: *mut IncludeStmt,
) -> bool {
    unsafe {
        let mut group: *mut xkb_file_include_group = std::ptr::null_mut();
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut buf: [i8; 4096] = [0; 4096];
            let mut xkb_file: *mut XkbFile = ProcessIncludeFile(
                ctx,
                stmt,
                file_type,
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            let valid: bool = !xkb_file.is_null();
            if valid as i32 != 0
                || flags as u32 & XKB_FILE_ITERATOR_FAIL_ON_INCLUDE_ERROR as u32 == 0
            {
                let path: u32 = (&(*section).buffer).len() as u32;
                let buf_ptr: *const i8 = &raw mut buf as *const i8;
                let buf_len = cstr_len(buf_ptr).wrapping_add(1) as usize;
                (&mut (*section).buffer)
                    .extend_from_slice(std::slice::from_raw_parts(buf_ptr, buf_len));
                let file: u32 = (&(*section).buffer).len() as u32;
                let file_len = cstr_len((*stmt).file).wrapping_add(1) as usize;
                (&mut (*section).buffer)
                    .extend_from_slice(std::slice::from_raw_parts((*stmt).file, file_len));
                let section_name: u32 =
                    if !(*stmt).map.is_null() || valid as i32 != 0 && !(*xkb_file).name.is_null() {
                        (&(*section).buffer).len() as u32
                    } else {
                        0 as u32
                    };
                if section_name != 0 {
                    let src = if !(*stmt).map.is_null() {
                        (*stmt).map
                    } else {
                        (*xkb_file).name
                    };
                    let src_len = cstr_len(src).wrapping_add(1) as usize;
                    (&mut (*section).buffer)
                        .extend_from_slice(std::slice::from_raw_parts(src, src_len));
                }
                let modifier: u32 = if !(*stmt).modifier.is_null() {
                    (&(*section).buffer).len() as u32
                } else {
                    0 as u32
                };
                if modifier != 0 {
                    let mod_len = cstr_len((*stmt).modifier).wrapping_add(1) as usize;
                    (&mut (*section).buffer)
                        .extend_from_slice(std::slice::from_raw_parts((*stmt).modifier, mod_len));
                }
                let section_flags: xkb_map_flags = (if valid as i32 != 0 {
                    (*xkb_file).flags as u32
                } else {
                    0 as u32
                }) as xkb_map_flags;
                let inc = xkb_file_include {
                    valid,
                    explicit_section: !(*stmt).map.is_null(),
                    merge: (*stmt).merge,
                    path: path,
                    file: file,
                    section: section_name,
                    modifier: modifier,
                    flags: section_flags,
                };
                let idx: u32 = (&(*section).includes).len() as u32;
                (&mut (*section).includes).push(inc);
                if group.is_null() {
                    let group_idx: u32 = (&(*section).include_groups).len() as u32;
                    (&mut (*section).include_groups).push(xkb_file_include_group {
                        start: idx,
                        end: idx,
                    });
                    group = (&mut (*section).include_groups)
                        .as_mut_ptr()
                        .offset(group_idx as isize)
                        as *mut xkb_file_include_group;
                } else {
                    (*group).end = idx;
                }
                FreeXkbFile(xkb_file);
            } else {
                let name: *const i8 =
                    xkb_file_section_get_string(section, (*section).name) as *const i8;
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} include failure in: {}{}{}{}\n",
                    XKB_ERROR_INCLUDED_FILE_NOT_FOUND as i32,
                    crate::xkb::utils::CStrDisplay(xkb_file_type_name(file_type)),
                    crate::xkb::utils::CStrDisplay(section_path),
                    crate::xkb::utils::CStrDisplay(if (*section).name != 0 {
                        b" (section: \"\0".as_ptr() as *const i8
                    } else {
                        b"\0".as_ptr() as *const i8
                    }),
                    crate::xkb::utils::CStrDisplay(name),
                    crate::xkb::utils::CStrDisplay(if (*section).name != 0 {
                        b"\")\0".as_ptr() as *const i8
                    } else {
                        b"\0".as_ptr() as *const i8
                    }),
                );
                FreeXkbFile(xkb_file);
                return false;
            }
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        return true;
    }
}
unsafe fn xkb_file_section_process(
    mut ctx: *mut xkb_context,
    mut flags: xkb_file_iterator_flags,
    mut path: *const i8,
    mut section: *mut xkb_file_section,
    mut xkb_file: *const XkbFile,
) -> bool {
    unsafe {
        let mut ok: bool = true;
        let mut stmt: *mut ParseCommon = (*xkb_file).defs;
        while !stmt.is_null() {
            if (*stmt).type_0 as u32 == STMT_INCLUDE as u32 {
                ok = xkb_file_section_append_includes(
                    ctx,
                    flags,
                    path,
                    section,
                    (*xkb_file).file_type,
                    stmt as *mut IncludeStmt,
                );
                if !ok {
                    break;
                }
            }
            stmt = (*stmt).next as *mut ParseCommon;
        }
        return ok;
    }
}

pub unsafe fn xkb_file_iterator_free(mut iter: *mut xkb_file_iterator) {
    unsafe {
        if iter.is_null() {
            return;
        }
        xkb_file_section_free(&raw mut (*iter).section);
        FreeXkbFile((*iter).pending_xkb_file);
        let layout = std::alloc::Layout::new::<xkb_file_iterator>();
        std::alloc::dealloc(iter as *mut u8, layout);
    }
}

pub unsafe fn xkb_file_section_get_string(
    mut section: *const xkb_file_section,
    mut idx: u32,
) -> *const i8 {
    unsafe {
        return (&(*section).buffer).as_ptr().offset(idx as isize) as *mut i8;
    }
}
use crate::xkb::shared_types::*;
