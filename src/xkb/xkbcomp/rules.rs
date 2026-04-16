use crate::xkb::context::xkb_context_sanitize_rule_names;
use crate::xkb_logf;

pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;
pub use crate::xkb::utils::is_absolute_path;

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
pub use crate::xkb::rmlvo::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_option,
};
pub use crate::xkb::scanner_utils::{scanner, scanner_loc, sval, svaleq, svaleq_prefix};
pub use crate::xkb::shared_ast_types::{
    _FILE_TYPE_NUM_ENTRIES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID,
    FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES,
    FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE,
};
pub use crate::xkb::shared_types::XKB_MAX_GROUPS;
pub use crate::xkb::shared_types::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use crate::xkb::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
use crate::xkb::utils::{cstr_as_bytes, cstr_len, cstr_len_safe};
pub use crate::xkb::xkbcomp::include::{
    expand_path, FindFileInXkbPath, MERGE_AUGMENT_PREFIX, MERGE_OVERRIDE_PREFIX,
    MERGE_REPLACE_PREFIX,
};
use libc::{fclose, fopen, FILE};

/// Appends `count` bytes from `src` to the Vec.
#[inline]
unsafe fn vec_append_nul_terminated(v: &mut Vec<i8>, src: *const i8, count: u32) {
    v.extend_from_slice(std::slice::from_raw_parts(src, count as usize));
}

#[derive(Clone)]
pub struct matcher {
    pub ctx: *mut xkb_context,
    pub rmlvo: rule_names,
    pub val: lvalue,
    pub groups: Vec<group>,
    pub mapping: mapping,
    pub rule: rule,
    pub pending_kccgst: kccgst_buffer,
    pub kccgst: [Vec<i8>; 5],
}
#[derive(Clone)]
pub struct kccgst_buffer {
    pub buffer: Vec<i8>,
    pub slices: Vec<kccgst_buffer_slice>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kccgst_buffer_slice {
    pub length: u32,
    pub kccgst: rules_kccgst,
    pub layout: u32,
}
pub type rules_kccgst = u32;
pub const _KCCGST_NUM_ENTRIES: rules_kccgst = 5;
pub const KCCGST_GEOMETRY: rules_kccgst = 4;
pub const KCCGST_SYMBOLS: rules_kccgst = 3;
pub const KCCGST_COMPAT: rules_kccgst = 2;
pub const KCCGST_TYPES: rules_kccgst = 1;
pub const KCCGST_KEYCODES: rules_kccgst = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub mlvo_value_at_pos: [sval; 4],
    pub match_type_at_pos: [mlvo_match_type; 4],
    pub kccgst_value_at_pos: [sval; 5],
    pub num_mlvo_values: mlvo_index_t,
    pub num_kccgst_values: kccgst_index_t,
    pub skip: bool,
}
pub type kccgst_index_t = u8;
pub type mlvo_index_t = u8;
pub type mlvo_match_type = u32;
pub const MLVO_MATCH_GROUP: mlvo_match_type = 5;
pub const MLVO_MATCH_WILDCARD_ANY: mlvo_match_type = 4;
pub const MLVO_MATCH_WILDCARD_SOME: mlvo_match_type = 3;
pub const MLVO_MATCH_WILDCARD_NONE: mlvo_match_type = 2;
pub const MLVO_MATCH_WILDCARD_LEGACY: mlvo_match_type = 1;
pub const MLVO_MATCH_NORMAL: mlvo_match_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapping {
    pub mlvo_at_pos: [rules_mlvo; 4],
    pub num_mlvo: mlvo_index_t,
    pub defined_mlvo_mask: mlvo_mask_t,
    pub has_layout_idx_range: bool,
    pub c2rust_unnamed: C2Rust_Unnamed_3,
    pub c2rust_unnamed_0: C2Rust_Unnamed_2,
    pub kccgst_at_pos: [rules_kccgst; 5],
    pub num_kccgst: kccgst_index_t,
    pub defined_kccgst_mask: kccgst_mask_t,
}
pub type kccgst_mask_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub active: u32,
    pub layouts_candidates_mask: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub layout_idx_min: u32,
    pub layout_idx_max: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub layout_idx: u32,
    pub variant_idx: u32,
}
pub type mlvo_mask_t = u8;
pub type rules_mlvo = u32;
pub const _MLVO_NUM_ENTRIES: rules_mlvo = 4;
pub const MLVO_OPTION: rules_mlvo = 3;
pub const MLVO_VARIANT: rules_mlvo = 2;
pub const MLVO_LAYOUT: rules_mlvo = 1;
pub const MLVO_MODEL: rules_mlvo = 0;
#[derive(Clone)]
pub struct group {
    pub name: sval,
    pub elements: Vec<sval>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union lvalue {
    pub string: sval,
}
#[derive(Clone)]
pub struct rule_names {
    pub model: matched_sval,
    pub layouts: Vec<matched_sval>,
    pub variants: Vec<matched_sval>,
    pub options: Vec<matched_sval>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matched_sval {
    pub sval: sval,
    pub matched: bool,
    pub layout: u32,
}
pub const TOK_ERROR: rules_token = 11;
pub type rules_token = u32;
pub const TOK_INCLUDE: rules_token = 10;
pub const TOK_WILD_CARD_ANY: rules_token = 9;
pub const TOK_WILD_CARD_SOME: rules_token = 8;
pub const TOK_WILD_CARD_NONE: rules_token = 7;
pub const TOK_WILD_CARD_STAR: rules_token = 6;
pub const TOK_EQUALS: rules_token = 5;
pub const TOK_BANG: rules_token = 4;
pub const TOK_GROUP_NAME: rules_token = 3;
pub const TOK_IDENTIFIER: rules_token = 2;
pub const TOK_END_OF_LINE: rules_token = 1;
pub const TOK_END_OF_FILE: rules_token = 0;
pub const LAYOUT_INDEX_FIRST: layout_index_ranges = 4294967292;
pub const LAYOUT_INDEX_SINGLE: layout_index_ranges = 4294967291;
pub const LAYOUT_INDEX_ANY: layout_index_ranges = 4294967294;
pub const LAYOUT_INDEX_LATER: layout_index_ranges = 4294967293;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub name: *const i8,
    pub length: i32,
    pub range: layout_index_ranges,
}
pub type layout_index_ranges = u32;
pub type wildcard_match_type = u32;
pub const WILDCARD_MATCH_ALL: wildcard_match_type = 1;
pub const WILDCARD_MATCH_NONEMPTY: wildcard_match_type = 0;
pub const MAX_INCLUDE_DEPTH: i32 = 5 as i32;
#[inline]
fn is_space(ch: i8) -> bool {
    matches!(ch as u8, b' ' | b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r')
}
#[inline]
unsafe fn is_ident(mut ch: i8) -> bool {
    return (ch as u8).is_ascii_graphic() && ch as i32 != '\\' as i32;
}
unsafe fn lex(mut s: *mut scanner, mut val: *mut lvalue) -> rules_token {
    unsafe {
        loop {
            while (*s).chr(' ' as i32 as i8) as i32 != 0
                || (*s).chr('\t' as i32 as i8) as i32 != 0
                || (*s).chr('\r' as i32 as i8) as i32 != 0
            {}
            if (*s).str_match(
                b"//\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 3]>()).wrapping_sub(1 as usize),
            ) {
                (*s).skip_to_eol();
            }
            if (*s).eol() {
                while (*s).eol() {
                    (*s).next_byte();
                }
                return TOK_END_OF_LINE;
            }
            if !(*s).chr('\\' as i32 as i8) {
                break;
            }
            (*s).chr('\r' as i32 as i8);
            if !(*s).eol() {
                let mut loc: scanner_loc = (*s).token_location();
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: illegal new line escape; must appear at end of line\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc.line,
                    loc.column,
                );
                return TOK_ERROR;
            }
            (*s).next_byte();
        }
        if (*s).eof() {
            return TOK_END_OF_FILE;
        }
        (*s).token_pos = (*s).pos;
        if (*s).chr('!' as i32 as i8) {
            return TOK_BANG;
        }
        if (*s).chr('=' as i32 as i8) {
            return TOK_EQUALS;
        }
        if (*s).chr('*' as i32 as i8) {
            return TOK_WILD_CARD_STAR;
        }
        if (*s).str_match(
            b"<none>\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_NONE;
        }
        if (*s).str_match(
            b"<some>\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_SOME;
        }
        if (*s).str_match(
            b"<any>\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
        ) {
            return TOK_WILD_CARD_ANY;
        }
        if (*s).chr('$' as i32 as i8) {
            (*val).string.start = (*s).s.offset((*s).pos as isize);
            (*val).string.len = 0 as usize;
            while is_ident((*s).peek()) {
                (*s).next_byte();
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            if (*val).string.len == 0 as usize {
                let mut loc_0: scanner_loc = (*s).token_location();
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: unexpected character after '$'; expected name\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_0.line,
                    loc_0.column,
                );
                return TOK_ERROR;
            }
            return TOK_GROUP_NAME;
        }
        if (*s).str_match(
            b"include\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize),
        ) {
            return TOK_INCLUDE;
        }
        if is_ident((*s).peek()) {
            (*val).string.start = (*s).s.offset((*s).pos as isize);
            (*val).string.len = 0 as usize;
            while is_ident((*s).peek()) {
                (*s).next_byte();
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            return TOK_IDENTIFIER;
        }
        let mut loc_1: scanner_loc = (*s).token_location();
        xkb_logf!(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] {}:{}:{}: unrecognized token\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            crate::xkb::utils::CStrDisplay((*s).file_name),
            loc_1.line,
            loc_1.column,
        );
        return TOK_ERROR;
    }
}
static mut rules_mlvo_svals: [sval; 4] = [sval {
    len: 0,
    start: std::ptr::null(),
}; 4];
static mut rules_kccgst_svals: [sval; 5] = [sval {
    len: 0,
    start: std::ptr::null(),
}; 5];
pub const OPTIONS_MATCH_ALL_GROUPS: i32 = XKB_MAX_GROUPS;
unsafe fn strip_spaces(mut v: sval) -> sval {
    unsafe {
        while v.len > 0 as usize && is_space(*v.start.offset(0 as i32 as isize)) as i32 != 0 {
            v.len = v.len.wrapping_sub(1);
            v.start = v.start.offset(1);
        }
        while v.len > 0 as usize
            && is_space(*v.start.offset(v.len.wrapping_sub(1 as usize) as isize)) as i32 != 0
        {
            v.len = v.len.wrapping_sub(1);
        }
        return v;
    }
}

/// Resize a Vec<matched_sval>, zero-filling new elements.
fn vec_resize_zero_matched_sval(v: &mut Vec<matched_sval>, new_len: usize) {
    if new_len > v.len() {
        v.resize(new_len, unsafe { std::mem::zeroed::<matched_sval>() });
    } else {
        v.truncate(new_len);
    }
}

unsafe fn split_comma_separated_mlvo(
    mut ctx: *mut xkb_context,
    mut mlvo: rules_mlvo,
    mut s: *const i8,
) -> Vec<matched_sval> {
    unsafe {
        let mut arr: Vec<matched_sval> = Vec::new();
        if s.is_null() {
            let mut val: matched_sval = {
                let mut init = matched_sval {
                    matched: false,
                    layout: 0,
                    sval: sval {
                        len: 0 as usize,
                        start: std::ptr::null(),
                    },
                };
                init.matched = false;
                init.layout = 0;
                init
            };
            arr.push(val);
            return arr;
        }
        loop {
            let mut val_0: matched_sval = {
                let mut init = matched_sval {
                    matched: false,
                    layout: 0,
                    sval: sval {
                        len: 0 as usize,
                        start: s,
                    },
                };
                init.matched = false;
                init.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
                init
            };
            while *s as i32 != '\0' as i32
                && *s as i32 != ',' as i32
                && *s as i32 != OPTIONS_GROUP_SPECIFIER_PREFIX
            {
                s = s.offset(1);
                val_0.sval.len = val_0.sval.len.wrapping_add(1);
            }
            val_0.sval = strip_spaces(val_0.sval);
            if *s as i32 == OPTIONS_GROUP_SPECIFIER_PREFIX {
                s = s.offset(1);
                let layout_start: *const i8 = s;
                let mut layout: u32 = XKB_LAYOUT_INVALID as u32;
                let (val_parsed, count) =
                    crate::xkb::utils::parse_dec_u32(std::ffi::CStr::from_ptr(s).to_bytes());
                layout = val_parsed;
                let mut count: i32 = count;
                if count > 0 as i32 {
                    s = s.offset(count as isize);
                    if layout == 0 as u32 || layout > XKB_MAX_GROUPS as u32 {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Invalid layout index {} for the RMVLO component: \"{}\"\n",
                            XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                            layout,
                            crate::xkb::utils::CStrNDisplay(
                                val_0.sval.len as usize,
                                val_0.sval.start
                            ),
                        );
                    } else if mlvo as u32 != MLVO_OPTION as u32 {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "Layout index {} is not supported for the RMLVO component: \"{}\"\n",
                            layout,
                            crate::xkb::utils::CStrNDisplay(
                                val_0.sval.len as usize,
                                val_0.sval.start
                            ),
                        );
                    } else {
                        val_0.layout = layout.wrapping_sub(1 as u32);
                    }
                }
                let layout_index_end: *const i8 = s;
                while *s as i32 != '\0' as i32 && *s as i32 != ',' as i32 {
                    s = s.offset(1);
                }
                if count <= 0 as i32 || layout_index_end != s {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Invalid layout index \"{}\" for the RMLVO component \"{}\"; discarding specifier.\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                        crate::xkb::utils::CStrNDisplay(s.offset_from(layout_start) as i64
                            as usize, layout_start),
                        crate::xkb::utils::CStrNDisplay(val_0.sval.len as usize, val_0.sval.start),
                    );
                    val_0.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
                }
            }
            arr.push(val_0);
            if *s as i32 == '\0' as i32 {
                break;
            }
            if *s as i32 == ',' as i32 {
                s = s.offset(1);
            }
        }
        return arr;
    }
}
unsafe fn matcher_new_from_rmlvo(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut rules: *mut *const i8,
) -> *mut matcher {
    unsafe {
        // Allocate zeroed memory for matcher, then initialize Vec fields properly.
        // We use alloc_zeroed to avoid the stack-based zeroed() panic in debug mode.
        let layout = std::alloc::Layout::new::<matcher>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut matcher;
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        let mut m: *mut matcher = ptr;
        // Vec fields cannot be safely zeroed — write proper empty Vecs
        std::ptr::write(&raw mut (*m).rmlvo.layouts, Vec::new());
        std::ptr::write(&raw mut (*m).rmlvo.variants, Vec::new());
        std::ptr::write(&raw mut (*m).rmlvo.options, Vec::new());
        std::ptr::write(&raw mut (*m).pending_kccgst.buffer, Vec::new());
        std::ptr::write(&raw mut (*m).pending_kccgst.slices, Vec::new());
        std::ptr::write(&raw mut (*m).groups, Vec::new());
        std::ptr::write(&raw mut (*m).kccgst, std::array::from_fn(|_| Vec::new()));
        (*m).ctx = std::ptr::addr_of!((*rmlvo).ctx) as *mut xkb_context;
        let ptr_to_cstring = |p: *const i8| -> std::ffi::CString {
            if p.is_null() {
                std::ffi::CString::new("").unwrap()
            } else {
                std::ffi::CStr::from_ptr(p).to_owned()
            }
        };
        let mut names: xkb_rule_names = xkb_rule_names {
            rules: ptr_to_cstring((*rmlvo).rules),
            model: ptr_to_cstring((*rmlvo).model),
            layout: if (*rmlvo).layouts.is_empty() {
                std::ffi::CString::new("").unwrap()
            } else {
                std::ffi::CString::new("x").unwrap()
            },
            variant: if (*rmlvo).layouts.is_empty() {
                std::ffi::CString::new("").unwrap()
            } else {
                std::ffi::CString::new("x").unwrap()
            },
            options: if (*rmlvo).options.is_empty() {
                std::ffi::CString::new("").unwrap()
            } else {
                std::ffi::CString::new("x").unwrap()
            },
        };
        let changed: RMLVO =
            xkb_context_sanitize_rule_names(&(*rmlvo).ctx, &raw mut names) as RMLVO;
        if changed as u32 & RMLVO_RULES as u32 != 0 {
            *rules = names.rules.as_ptr();
        } else {
            *rules = (*rmlvo).rules;
        }
        if changed as u32 & RMLVO_MODEL as u32 != 0 {
            (*m).rmlvo.model.sval.start = names.model.as_ptr();
        } else {
            (*m).rmlvo.model.sval.start = (*rmlvo).model;
        }
        (*m).rmlvo.model.sval.len = cstr_len_safe((*rmlvo).model);
        (*m).rmlvo.model.layout = OPTIONS_MATCH_ALL_GROUPS as u32 as u32;
        if changed as u32 & RMLVO_LAYOUT as u32 != 0 {
            (*m).rmlvo.layouts = split_comma_separated_mlvo(
                std::ptr::addr_of!((*rmlvo).ctx) as *mut xkb_context,
                MLVO_LAYOUT,
                names.layout.as_ptr(),
            );
            (*m).rmlvo.variants = split_comma_separated_mlvo(
                std::ptr::addr_of!((*rmlvo).ctx) as *mut xkb_context,
                MLVO_VARIANT,
                names.variant.as_ptr(),
            );
            if (*m).rmlvo.layouts.len() > (*m).rmlvo.variants.len() {
                if !names.variant.as_bytes().is_empty() {
                    xkb_logf!(
                        (*m).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "More layouts than variants: \"{}\" vs. \"{}\".\n",
                        if !names.layout.as_bytes().is_empty() {
                            names.layout.to_str().unwrap_or("")
                        } else {
                            "(none)"
                        },
                        if !names.variant.as_bytes().is_empty() {
                            names.variant.to_str().unwrap_or("")
                        } else {
                            "(none)"
                        },
                    );
                }
                vec_resize_zero_matched_sval(&mut (*m).rmlvo.variants, (*m).rmlvo.layouts.len());
            } else if (*m).rmlvo.layouts.len() < (*m).rmlvo.variants.len() {
                xkb_logf!(
                    (*m).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Less layouts than variants: \"{}\" vs. \"{}\".\n",
                    if !names.layout.as_bytes().is_empty() {
                        names.layout.to_str().unwrap_or("")
                    } else {
                        "(none)"
                    },
                    if !names.variant.as_bytes().is_empty() {
                        names.variant.to_str().unwrap_or("")
                    } else {
                        "(none)"
                    },
                );
                (*m).rmlvo.variants.truncate((*m).rmlvo.layouts.len());
            }
        } else {
            for layout in (*rmlvo).layouts.iter() {
                let mut val: matched_sval = {
                    let mut init = matched_sval {
                        matched: false,
                        layout: 0,
                        sval: sval {
                            len: cstr_len_safe(layout.layout),
                            start: layout.layout,
                        },
                    };
                    init.matched = false;
                    init.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
                    init
                };
                (*m).rmlvo.layouts.push(val);
                val.sval.start = layout.variant;
                val.sval.len = cstr_len_safe(layout.variant);
                (*m).rmlvo.variants.push(val);
            }
        }
        if changed as u32 & RMLVO_OPTIONS as u32 != 0 {
            (*m).rmlvo.options = split_comma_separated_mlvo(
                std::ptr::addr_of!((*rmlvo).ctx) as *mut xkb_context,
                MLVO_OPTION,
                names.options.as_ptr(),
            );
        } else {
            for option in (*rmlvo).options.iter() {
                let val_0: matched_sval = {
                    let mut init = matched_sval {
                        matched: false,
                        layout: 0,
                        sval: sval {
                            len: cstr_len_safe(option.option),
                            start: option.option,
                        },
                    };
                    init.matched = false;
                    init.layout = if option.layout == XKB_LAYOUT_INVALID as u32 {
                        OPTIONS_MATCH_ALL_GROUPS as u32
                    } else {
                        option.layout
                    };
                    init
                };
                (*m).rmlvo.options.push(val_0);
            }
        }
        return m;
    }
}
unsafe fn matcher_new_from_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *const xkb_rule_names,
) -> *mut matcher {
    unsafe {
        // Allocate zeroed memory for matcher, then initialize Vec fields properly.
        let layout = std::alloc::Layout::new::<matcher>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut matcher;
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        let mut m: *mut matcher = ptr;
        // Vec fields cannot be safely zeroed — write proper empty Vecs
        std::ptr::write(&raw mut (*m).rmlvo.layouts, Vec::new());
        std::ptr::write(&raw mut (*m).rmlvo.variants, Vec::new());
        std::ptr::write(&raw mut (*m).rmlvo.options, Vec::new());
        std::ptr::write(&raw mut (*m).pending_kccgst.buffer, Vec::new());
        std::ptr::write(&raw mut (*m).pending_kccgst.slices, Vec::new());
        std::ptr::write(&raw mut (*m).groups, Vec::new());
        std::ptr::write(&raw mut (*m).kccgst, std::array::from_fn(|_| Vec::new()));
        (*m).ctx = ctx;
        let rmlvo_ref = &*rmlvo;
        (*m).rmlvo.model.sval.start = if rmlvo_ref.model.as_bytes().is_empty() {
            std::ptr::null()
        } else {
            rmlvo_ref.model.as_ptr()
        };
        (*m).rmlvo.model.sval.len = rmlvo_ref.model.as_bytes().len();
        (*m).rmlvo.model.layout = OPTIONS_MATCH_ALL_GROUPS as u32 as u32;
        (*m).rmlvo.layouts = split_comma_separated_mlvo(
            ctx,
            MLVO_LAYOUT,
            if rmlvo_ref.layout.as_bytes().is_empty() {
                std::ptr::null()
            } else {
                rmlvo_ref.layout.as_ptr()
            },
        );
        (*m).rmlvo.variants = split_comma_separated_mlvo(
            ctx,
            MLVO_VARIANT,
            if rmlvo_ref.variant.as_bytes().is_empty() {
                std::ptr::null()
            } else {
                rmlvo_ref.variant.as_ptr()
            },
        );
        (*m).rmlvo.options = split_comma_separated_mlvo(
            ctx,
            MLVO_OPTION,
            if rmlvo_ref.options.as_bytes().is_empty() {
                std::ptr::null()
            } else {
                rmlvo_ref.options.as_ptr()
            },
        );
        if (*m).rmlvo.layouts.len() > (*m).rmlvo.variants.len() {
            if !rmlvo_ref.variant.as_bytes().is_empty() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "More layouts than variants: \"{}\" vs. \"{}\".\n",
                    if !rmlvo_ref.layout.as_bytes().is_empty() {
                        rmlvo_ref.layout.to_str().unwrap_or("")
                    } else {
                        "(none)"
                    },
                    if !rmlvo_ref.variant.as_bytes().is_empty() {
                        rmlvo_ref.variant.to_str().unwrap_or("")
                    } else {
                        "(none)"
                    },
                );
            }
            vec_resize_zero_matched_sval(&mut (*m).rmlvo.variants, (*m).rmlvo.layouts.len());
        } else if (*m).rmlvo.layouts.len() < (*m).rmlvo.variants.len() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Less layouts than variants: \"{}\" vs. \"{}\".\n",
                if !rmlvo_ref.layout.as_bytes().is_empty() {
                    rmlvo_ref.layout.to_str().unwrap_or("")
                } else {
                    "(none)"
                },
                if !rmlvo_ref.variant.as_bytes().is_empty() {
                    rmlvo_ref.variant.to_str().unwrap_or("")
                } else {
                    "(none)"
                },
            );
            (*m).rmlvo.variants.truncate((*m).rmlvo.layouts.len());
        }
        return m;
    }
}
unsafe fn matcher_free(mut m: *mut matcher) {
    unsafe {
        if m.is_null() {
            return;
        }
        // Drop Vec fields manually before deallocating
        std::ptr::drop_in_place(&raw mut (*m).groups);
        std::ptr::drop_in_place(&raw mut (*m).pending_kccgst.slices);
        std::ptr::drop_in_place(&raw mut (*m).rmlvo.layouts);
        std::ptr::drop_in_place(&raw mut (*m).rmlvo.variants);
        std::ptr::drop_in_place(&raw mut (*m).rmlvo.options);
        std::ptr::drop_in_place(&raw mut (*m).pending_kccgst.buffer);
        std::ptr::drop_in_place(&raw mut (*m).kccgst);
        // Deallocate (was allocated with alloc_zeroed, not Box)
        let layout = std::alloc::Layout::new::<matcher>();
        std::alloc::dealloc(m as *mut u8, layout);
    }
}
unsafe fn matcher_group_start_new(mut m: *mut matcher, mut name: sval) {
    unsafe {
        let group: group = group {
            name: name,
            elements: Vec::new(),
        };
        (&mut (*m).groups).push(group);
    }
}
unsafe fn matcher_group_add_element(mut m: *mut matcher, mut s: *mut scanner, mut element: sval) {
    unsafe {
        let last_group = (&mut (*m).groups).last_mut().unwrap();
        last_group.elements.push(element);
    }
}
unsafe fn matcher_include(
    mut m: *mut matcher,
    mut parent_scanner: *mut scanner,
    mut include_depth: u32,
    mut inc: sval,
) {
    unsafe {
        if include_depth >= MAX_INCLUDE_DEPTH as u32 {
            let mut loc: scanner_loc = (*parent_scanner).token_location();
            xkb_logf!(
                (*parent_scanner).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}:{}:{}: maximum include depth ({}) exceeded; maybe there is an include loop?\n",
                crate::xkb::utils::CStrDisplay((*parent_scanner).file_name),
                loc.line,
                loc.column,
                MAX_INCLUDE_DEPTH,
            );
            return;
        }
        let mut stmt_file: *const i8 = inc.start;
        let mut stmt_file_len: usize = inc.len;
        let mut buf: [i8; 4096] = [
            0 as i32 as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let expanded: isize = expand_path(
            (*m).ctx,
            (*parent_scanner).file_name,
            stmt_file,
            stmt_file_len,
            FILE_TYPE_RULES,
            &raw mut buf as *mut i8,
            std::mem::size_of::<[i8; 4096]>(),
        ) as isize;
        if expanded < 0 as isize {
            return;
        } else if expanded > 0 as isize {
            stmt_file = &raw mut buf as *mut i8;
            stmt_file_len = expanded as usize;
        }
        let mut file: *mut FILE = std::ptr::null_mut();
        let mut offset: u32 = 0 as u32;
        let absolute_path: bool = is_absolute_path(stmt_file) as bool;
        if absolute_path {
            if expanded == 0 {
                if stmt_file_len < std::mem::size_of::<[i8; 4096]>() {
                    std::ptr::copy_nonoverlapping(
                        stmt_file as *const u8,
                        &raw mut buf as *mut i8 as *mut u8,
                        stmt_file_len,
                    );
                    buf[stmt_file_len as usize] = '\0' as i32 as i8;
                    stmt_file = &raw mut buf as *mut i8;
                } else {
                    xkb_logf!(
                        (*m).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Path is too long: {} > {}, got raw path: {}\n",
                        XKB_ERROR_INVALID_PATH as i32,
                        stmt_file_len,
                        std::mem::size_of::<[i8; 4096]>(),
                        crate::xkb::utils::CStrNDisplay(stmt_file_len as usize, stmt_file),
                    );
                    return;
                }
            } else {
            }
            file = fopen(stmt_file, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        } else if (expanded != 0) as i64 != 0 {
            file = std::ptr::null_mut();
        } else {
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name,
                stmt_file,
                stmt_file_len,
                FILE_TYPE_RULES,
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                &raw mut offset,
                true,
            );
        }
        while !file.is_null() {
            let mut ret: bool = read_rules_file(
                (*m).ctx,
                m,
                include_depth.wrapping_add(1 as u32),
                file,
                &raw mut buf as *mut i8,
            );
            fclose(file);
            if ret {
                return;
            }
            xkb_logf!(
                (*m).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "No components returned from included XKB rules \"{}\"\n",
                crate::xkb::utils::CStrDisplay(&raw mut buf as *mut i8),
            );
            if absolute_path {
                break;
            }
            offset = offset.wrapping_add(1);
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name,
                stmt_file,
                stmt_file_len,
                FILE_TYPE_RULES,
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                &raw mut offset,
                true,
            );
        }
        xkb_logf!(
            (*m).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "Failed to open included XKB rules \"{}\"\n",
            crate::xkb::utils::CStrNDisplay(stmt_file_len as usize, stmt_file),
        );
    }
}
unsafe fn matcher_mapping_start_new(mut m: *mut matcher) {
    unsafe {
        let mut i: mlvo_index_t = 0 as mlvo_index_t;
        while (i as i32) < _MLVO_NUM_ENTRIES as i32 as mlvo_index_t as i32 {
            (*m).mapping.mlvo_at_pos[i as usize] = _MLVO_NUM_ENTRIES;
            i = i.wrapping_add(1);
        }
        let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
        while (i_0 as i32) < _KCCGST_NUM_ENTRIES as i32 as kccgst_index_t as i32 {
            (*m).mapping.kccgst_at_pos[i_0 as usize] = _KCCGST_NUM_ENTRIES;
            i_0 = i_0.wrapping_add(1);
        }
        (*m).mapping.has_layout_idx_range = false;
        (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx = XKB_LAYOUT_INVALID as u32;
        (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx =
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx;
        (*m).mapping.num_kccgst = 0 as kccgst_index_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_kccgst as mlvo_index_t;
        (*m).mapping.defined_mlvo_mask = 0 as mlvo_mask_t;
        (*m).mapping.defined_kccgst_mask = 0 as kccgst_mask_t;
        (*m).mapping.c2rust_unnamed_0.active = 1 as u32;
    }
}
unsafe fn parse_layout_int_index(mut s: *const i8, mut max_len: usize, mut out: *mut u32) -> i32 {
    unsafe {
        let slice = std::slice::from_raw_parts(s.offset(1) as *const u8, max_len.wrapping_sub(2));
        let (val, count) = crate::xkb::utils::parse_dec_u32(slice);
        let count: i32 = count;
        if count <= 0 as i32
            || *s.offset((1 as i32 + count) as isize) as i32 != ']' as i32
            || val == 0 as u32
            || val > XKB_MAX_GROUPS as u32
        {
            return -1 as i32;
        }
        *out = val.wrapping_sub(1 as u32) as u32;
        return count + 2 as i32;
    }
}
unsafe fn extract_layout_index(mut s: *const i8, mut max_len: usize, mut out: *mut u32) -> i32 {
    unsafe {
        *out = XKB_LAYOUT_INVALID as u32;
        if max_len < 3 as usize || *s.offset(0 as i32 as isize) as i32 != '[' as i32 {
            return -1 as i32;
        }
        if max_len > 3 as usize
            && *s.offset(1 as i32 as isize) as i32 == '%' as i32
            && *s.offset(2 as i32 as isize) as i32 == 'i' as i32
            && *s.offset(3 as i32 as isize) as i32 == ']' as i32
        {
            return 4 as i32;
        }
        return parse_layout_int_index(s, max_len, out);
    }
}
unsafe fn extract_mapping_layout_index(
    mut s: *const i8,
    mut max_len: usize,
    mut out: *mut u32,
) -> i32 {
    unsafe {
        static mut names: [C2Rust_Unnamed_7; 4] = [
            C2Rust_Unnamed_7 {
                name: b"single]\0".as_ptr() as *const i8,
                length: 7 as i32,
                range: LAYOUT_INDEX_SINGLE,
            },
            C2Rust_Unnamed_7 {
                name: b"first]\0".as_ptr() as *const i8,
                length: 6 as i32,
                range: LAYOUT_INDEX_FIRST,
            },
            C2Rust_Unnamed_7 {
                name: b"later]\0".as_ptr() as *const i8,
                length: 6 as i32,
                range: LAYOUT_INDEX_LATER,
            },
            C2Rust_Unnamed_7 {
                name: b"any]\0".as_ptr() as *const i8,
                length: 4 as i32,
                range: LAYOUT_INDEX_ANY,
            },
        ];
        if max_len < 3 as usize || *s.offset(0 as i32 as isize) as i32 != '[' as i32 {
            *out = XKB_LAYOUT_INVALID as u32;
            return -1 as i32;
        }
        let mut k: u32 = 0 as u32;
        while (k as usize)
            < (std::mem::size_of::<[C2Rust_Unnamed_7; 4]>())
                .wrapping_div(std::mem::size_of::<C2Rust_Unnamed_7>())
        {
            if cstr_as_bytes(s.offset(1 as i32 as isize) as *const i8).starts_with(
                &cstr_as_bytes(names[k as usize].name)[..names[k as usize].length as usize],
            ) {
                *out = names[k as usize].range as u32;
                return names[k as usize].length + 1 as i32;
            }
            k = k.wrapping_add(1);
        }
        *out = XKB_LAYOUT_INVALID as u32;
        return parse_layout_int_index(s, max_len, out);
    }
}
#[inline]
unsafe fn is_mlvo_mask_defined(mut m: *mut matcher, mut mlvo: rules_mlvo) -> bool {
    unsafe {
        return (*m).mapping.defined_mlvo_mask as u32 & (1 as u32) << mlvo as u32 != 0;
    }
}
unsafe fn matcher_mapping_set_mlvo(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        let mut mlvo: rules_mlvo = MLVO_MODEL;
        let mut mlvo_sval: sval = sval {
            len: 0,
            start: std::ptr::null(),
        };
        mlvo = MLVO_MODEL;
        while (mlvo as u32) < _MLVO_NUM_ENTRIES as u32 {
            mlvo_sval = rules_mlvo_svals[mlvo as usize];
            if svaleq_prefix(mlvo_sval, ident) {
                break;
            }
            mlvo += 1;
        }
        if mlvo as u32 >= _MLVO_NUM_ENTRIES as u32 {
            let mut loc: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
                crate::xkb::utils::CStrNDisplay(ident.len as usize, ident.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
            return;
        }
        if is_mlvo_mask_defined(m, mlvo) {
            let mut loc_0: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
                crate::xkb::utils::CStrNDisplay(mlvo_sval.len as usize, mlvo_sval.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
            return;
        }
        if mlvo_sval.len < ident.len {
            let mut idx: u32 = 0;
            let mut consumed: i32 = extract_mapping_layout_index(
                ident.start.offset(mlvo_sval.len as isize),
                ident.len.wrapping_sub(mlvo_sval.len),
                &raw mut idx,
            );
            if ident.len.wrapping_sub(mlvo_sval.len) as i32 != consumed {
                let mut loc_1: scanner_loc = (*s).token_location();
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" may only be followed by a valid group index; ignoring rule set\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_1.line,
                    loc_1.column,
                    crate::xkb::utils::CStrNDisplay(mlvo_sval.len as usize, mlvo_sval.start),
                );
                (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
                return;
            }
            if mlvo as u32 == MLVO_LAYOUT as u32 {
                (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx = idx;
            } else if mlvo as u32 == MLVO_VARIANT as u32 {
                (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx = idx;
            } else {
                let mut loc_2: scanner_loc = (*s).token_location();
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" cannot be followed by a group index; ignoring rule set\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc_2.line,
                    loc_2.column,
                    crate::xkb::utils::CStrNDisplay(mlvo_sval.len as usize, mlvo_sval.start),
                );
                (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
                return;
            }
        } else if mlvo as u32 == MLVO_LAYOUT as u32 {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx =
                LAYOUT_INDEX_SINGLE as u32 as u32;
        } else if mlvo as u32 == MLVO_VARIANT as u32 {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx =
                LAYOUT_INDEX_SINGLE as u32 as u32;
        }
        if (mlvo as u32 == MLVO_LAYOUT as u32 && is_mlvo_mask_defined(m, MLVO_VARIANT) as i32 != 0
            || mlvo as u32 == MLVO_VARIANT as u32
                && is_mlvo_mask_defined(m, MLVO_LAYOUT) as i32 != 0)
            && (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                != (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        {
            let mut loc_3: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"layout\" index must be the same as the \"variant\" index\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_3.line,
                loc_3.column,
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
            return;
        }
        (*m).mapping.mlvo_at_pos[(*m).mapping.num_mlvo as usize] = mlvo;
        (*m).mapping.defined_mlvo_mask = ((*m).mapping.defined_mlvo_mask as i32
            | (1 as u32 as mlvo_mask_t as i32) << mlvo as u32)
            as mlvo_mask_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_mlvo.wrapping_add(1);
    }
}
unsafe fn matcher_mapping_set_layout_bounds(mut m: *mut matcher) {
    unsafe {
        let mut idx: u32 = if (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
            < (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
        } else {
            (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx
        };
        let mut c2rust_current_block_17: u64;
        match idx {
            XKB_LAYOUT_INVALID => {
                (*m).mapping.has_layout_idx_range = false;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min =
                    XKB_LAYOUT_INVALID as u32;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    XKB_LAYOUT_INVALID as u32;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask = 0x1 as u32;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967293 => {
                (*m).mapping.has_layout_idx_range = true;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min = 1 as u32;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    (if (32) < (*m).rmlvo.layouts.len() {
                        32
                    } else {
                        (*m).rmlvo.layouts.len()
                    }) as u32;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    (((1 as u64) << (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max)
                        .wrapping_sub(1 as u64) as u32 as u64
                        & !(1 as u64)) as u32;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967294 => {
                (*m).mapping.has_layout_idx_range = true;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min = 0 as u32;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    (if (32) < (*m).rmlvo.layouts.len() {
                        32
                    } else {
                        (*m).rmlvo.layouts.len()
                    }) as u32;
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask =
                    (((1 as u64) << (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max)
                        as u32 as u64)
                        .wrapping_sub(1 as u64) as u32;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967291 | 4294967292 => {
                idx = 0 as u32;
                c2rust_current_block_17 = 9641388756612255828;
            }
            _ => {
                c2rust_current_block_17 = 9641388756612255828;
            }
        }
        match c2rust_current_block_17 {
            9641388756612255828 => {
                (*m).mapping.has_layout_idx_range = false;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min = idx;
                (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max =
                    idx.wrapping_add(1 as u32);
                (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask = ((1 as u32) << idx) as u32;
            }
            _ => {}
        };
    }
}
unsafe fn matcher_mapping_set_kccgst(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        let mut kccgst: rules_kccgst = KCCGST_KEYCODES;
        let mut kccgst_sval: sval = sval {
            len: 0,
            start: std::ptr::null(),
        };
        kccgst = KCCGST_KEYCODES;
        while (kccgst as u32) < _KCCGST_NUM_ENTRIES as u32 {
            kccgst_sval = rules_kccgst_svals[kccgst as usize];
            if svaleq(rules_kccgst_svals[kccgst as usize], ident) {
                break;
            }
            kccgst += 1;
        }
        if kccgst as u32 >= _KCCGST_NUM_ENTRIES as u32 {
            let mut loc: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
                crate::xkb::utils::CStrNDisplay(ident.len as usize, ident.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
            return;
        }
        if (*m).mapping.defined_kccgst_mask as u32 & (1 as u32) << kccgst as u32 != 0 {
            let mut loc_0: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
                crate::xkb::utils::CStrNDisplay(kccgst_sval.len as usize, kccgst_sval.start),
            );
            (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
            return;
        }
        (*m).mapping.kccgst_at_pos[(*m).mapping.num_kccgst as usize] = kccgst;
        (*m).mapping.defined_kccgst_mask = ((*m).mapping.defined_kccgst_mask as i32
            | (1 as u32 as kccgst_mask_t as i32) << kccgst as u32)
            as kccgst_mask_t;
        (*m).mapping.num_kccgst = (*m).mapping.num_kccgst.wrapping_add(1);
    }
}
unsafe fn matcher_mapping_verify(mut m: *mut matcher, mut s: *mut scanner) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        if (*m).mapping.num_mlvo as i32 == 0 as i32 {
            let mut loc: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the left hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
        } else if (*m).mapping.num_kccgst as i32 == 0 as i32 {
            let mut loc_0: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the right hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc_0.line,
                loc_0.column,
            );
        } else {
            if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
                match (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx {
                    4294967291 => {
                        c2rust_current_block = 4840043166261277618;
                        match c2rust_current_block {
                            14825033830842003582 => {
                                if (*m).rmlvo.layouts.len() < 2
                                    || (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                                        >= (*m).rmlvo.layouts.len() as u32
                                {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                            _ => {
                                if (*m).rmlvo.layouts.len() > 1 {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                        }
                    }
                    4294967294 | 4294967293 | 4294967292 => {
                        c2rust_current_block = 8831408221741692167;
                    }
                    _ => {
                        c2rust_current_block = 14825033830842003582;
                        match c2rust_current_block {
                            14825033830842003582 => {
                                if (*m).rmlvo.layouts.len() < 2
                                    || (*m).mapping.c2rust_unnamed.c2rust_unnamed.layout_idx
                                        >= (*m).rmlvo.layouts.len() as u32
                                {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                            _ => {
                                if (*m).rmlvo.layouts.len() > 1 {
                                    c2rust_current_block = 436805222042109220;
                                } else {
                                    c2rust_current_block = 8831408221741692167;
                                }
                            }
                        }
                    }
                }
            } else {
                c2rust_current_block = 8831408221741692167;
            }
            match c2rust_current_block {
                436805222042109220 => {}
                _ => {
                    if is_mlvo_mask_defined(m, MLVO_VARIANT) {
                        match (*m).mapping.c2rust_unnamed.c2rust_unnamed.variant_idx {
                            4294967291 => {
                                c2rust_current_block = 13345507216710712890;
                                match c2rust_current_block {
                                    10338831042980687939 => {
                                        if (*m).rmlvo.variants.len() < 2
                                            || (*m)
                                                .mapping
                                                .c2rust_unnamed
                                                .c2rust_unnamed
                                                .variant_idx
                                                >= (*m).rmlvo.variants.len() as u32
                                        {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                    _ => {
                                        if (*m).rmlvo.variants.len() > 1 {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                }
                            }
                            4294967294 | 4294967293 | 4294967292 => {
                                c2rust_current_block = 10652014663920648156;
                            }
                            _ => {
                                c2rust_current_block = 10338831042980687939;
                                match c2rust_current_block {
                                    10338831042980687939 => {
                                        if (*m).rmlvo.variants.len() < 2
                                            || (*m)
                                                .mapping
                                                .c2rust_unnamed
                                                .c2rust_unnamed
                                                .variant_idx
                                                >= (*m).rmlvo.variants.len() as u32
                                        {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                    _ => {
                                        if (*m).rmlvo.variants.len() > 1 {
                                            c2rust_current_block = 436805222042109220;
                                        } else {
                                            c2rust_current_block = 10652014663920648156;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        c2rust_current_block = 10652014663920648156;
                    }
                    match c2rust_current_block {
                        436805222042109220 => {}
                        _ => return true,
                    }
                }
            }
        }
        (*m).mapping.c2rust_unnamed_0.active = 0 as u32;
        return false;
    }
}
unsafe fn matcher_rule_start_new(mut m: *mut matcher) {
    unsafe {
        std::ptr::write_bytes::<rule>(&raw mut (*m).rule as *mut rule, 0u8, 1);
        (*m).rule.skip = (*m).mapping.c2rust_unnamed_0.active == 0;
    }
}
unsafe fn matcher_rule_set_mlvo_common(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut ident: sval,
    mut match_type: mlvo_match_type,
) {
    unsafe {
        if (*m).rule.num_mlvo_values as i32 >= (*m).mapping.num_mlvo as i32 {
            let mut loc: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            (*m).rule.skip = true;
            return;
        }
        (*m).rule.match_type_at_pos[(*m).rule.num_mlvo_values as usize] = match_type;
        (*m).rule.mlvo_value_at_pos[(*m).rule.num_mlvo_values as usize] = ident;
        (*m).rule.num_mlvo_values = (*m).rule.num_mlvo_values.wrapping_add(1);
    }
}
unsafe fn matcher_rule_set_mlvo_wildcard(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut match_type: mlvo_match_type,
) {
    unsafe {
        let mut dummy: sval = sval {
            len: 0 as usize,
            start: std::ptr::null(),
        };
        matcher_rule_set_mlvo_common(m, s, dummy, match_type);
    }
}
unsafe fn matcher_rule_set_mlvo_group(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
    }
}
unsafe fn matcher_rule_set_mlvo(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
    }
}
unsafe fn matcher_rule_set_kccgst(mut m: *mut matcher, mut s: *mut scanner, mut ident: sval) {
    unsafe {
        if (*m).rule.num_kccgst_values as i32 >= (*m).mapping.num_kccgst as i32 {
            let mut loc: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            (*m).rule.skip = true;
            return;
        }
        (*m).rule.kccgst_value_at_pos[(*m).rule.num_kccgst_values as usize] = ident;
        (*m).rule.num_kccgst_values = (*m).rule.num_kccgst_values.wrapping_add(1);
    }
}
unsafe fn match_group(mut m: *mut matcher, mut group_name: sval, mut to: sval) -> bool {
    unsafe {
        let found_group = (&(*m).groups).iter().find(|g| svaleq(g.name, group_name));
        match found_group {
            None => false,
            Some(group) => {
                for elem in group.elements.iter() {
                    if svaleq(to, *elem) {
                        return true;
                    }
                }
                false
            }
        }
    }
}
unsafe fn match_value(
    mut m: *mut matcher,
    mut val: sval,
    mut to: sval,
    mut match_type: mlvo_match_type,
    mut wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        match match_type as u32 {
            1 => {
                return wildcard_type as u32 == WILDCARD_MATCH_ALL as u32 || to.len != 0;
            }
            2 => return to.len == 0,
            3 => return to.len != 0,
            4 => return true,
            5 => return match_group(m, val, to),
            _ => {
                return svaleq(val, to);
            }
        };
    }
}
unsafe fn match_value_and_mark(
    mut m: *mut matcher,
    mut val: sval,
    mut to: *mut matched_sval,
    mut match_type: mlvo_match_type,
    mut wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        let mut matched: bool = match_value(m, val, (*to).sval, match_type, wildcard_type);
        if matched {
            (*to).matched = (true) as bool;
        }
        return matched;
    }
}
unsafe fn expand_rmlvo_in_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut value: sval,
    mut layout_idx: u32,
    mut expanded: *mut Vec<i8>,
    mut i: *mut usize,
) -> bool {
    unsafe {
        let mut expanded_index: bool = false;
        let mut c2rust_current_block: u64;
        let mut str: *const i8 = value.start;
        let mut mlv: rules_mlvo = MLVO_MODEL;
        let mut idx: u32 = 0;
        let mut pfx: i8 = 0;
        let mut sfx: i8 = 0;
        let mut expanded_value: *mut matched_sval = std::ptr::null_mut();
        if *str.offset(*i as isize) as i32 == 'i' as i32
            && ((*i).wrapping_add(1 as usize) == value.len
                || (*str.offset((*i).wrapping_add(1 as usize) as isize) as i32
                    == MERGE_OVERRIDE_PREFIX
                    || *str.offset((*i).wrapping_add(1 as usize) as isize) as i32
                        == MERGE_AUGMENT_PREFIX
                    || *str.offset((*i).wrapping_add(1 as usize) as isize) as i32
                        == MERGE_REPLACE_PREFIX))
        {
            if layout_idx == XKB_LAYOUT_INVALID as u32 {
                let mut loc: scanner_loc = (*s).token_location();
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {}:{}:{}: Invalid %i in %-expansion: there is no corresponding layout nor variant in the MLVO fields of the rules header.\n",
                    XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION
                        as i32,
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc.line,
                    loc.column,
                );
            } else {
                *i = (*i).wrapping_add(1);
                let mut index_str: [i8; 12] = [0; 12];
                let mut count: i32 = crate::xkb::utils::snprintf_c(
                    &raw mut index_str as *mut i8,
                    std::mem::size_of::<[i8; 12]>(),
                    format_args!("{}", layout_idx.wrapping_add(1 as u32)),
                );
                (*expanded).extend_from_slice(std::slice::from_raw_parts(
                    &raw mut index_str as *mut i8 as *const i8,
                    count as usize,
                ));
                return true;
            }
        } else {
            sfx = 0 as i8;
            pfx = sfx;
            if *str.offset(*i as isize) as i32 == '(' as i32
                || (*str.offset(*i as isize) as i32 == MERGE_OVERRIDE_PREFIX
                    || *str.offset(*i as isize) as i32 == MERGE_AUGMENT_PREFIX
                    || *str.offset(*i as isize) as i32 == MERGE_REPLACE_PREFIX)
                || *str.offset(*i as isize) as i32 == '_' as i32
                || *str.offset(*i as isize) as i32 == '-' as i32
            {
                pfx = *str.offset(*i as isize);
                if *str.offset(*i as isize) as i32 == '(' as i32 {
                    sfx = ')' as i32 as i8;
                }
                *i = (*i).wrapping_add(1);
                if *i >= value.len {
                    c2rust_current_block = 14165246690716487359;
                } else {
                    c2rust_current_block = 17478428563724192186;
                }
            } else {
                c2rust_current_block = 17478428563724192186;
            }
            match c2rust_current_block {
                14165246690716487359 => {}
                _ => {
                    let c2rust_fresh7 = *i;
                    *i = (*i).wrapping_add(1);
                    match *str.offset(c2rust_fresh7 as isize) as i32 {
                        109 => {
                            c2rust_current_block = 1495343238628690102;
                            match c2rust_current_block {
                                17806387538889038492 => {
                                    mlv = MLVO_VARIANT;
                                }
                                14869399783518996101 => {
                                    mlv = MLVO_LAYOUT;
                                }
                                _ => {
                                    mlv = MLVO_MODEL;
                                }
                            }
                            idx = XKB_LAYOUT_INVALID as u32;
                            expanded_index = false;
                            if *i < value.len && *str.offset(*i as isize) as i32 == '[' as i32 {
                                if mlv as u32 != MLVO_LAYOUT as u32
                                    && mlv as u32 != MLVO_VARIANT as u32
                                {
                                    let mut loc_0: scanner_loc = (*s).token_location();
                                    xkb_logf!(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                                        crate::xkb::utils::CStrDisplay((*s).file_name),
                                        loc_0.line,
                                        loc_0.column,
                                    );
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let mut consumed: i32 = extract_layout_index(
                                        str.offset(*i as isize),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1 as i32 {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID as u32 {
                                            idx = layout_idx;
                                            expanded_index = true;
                                        }
                                        *i = (*i).wrapping_add(consumed as usize);
                                        c2rust_current_block = 10758786907990354186;
                                    }
                                }
                            } else {
                                c2rust_current_block = 10758786907990354186;
                            }
                            match c2rust_current_block {
                                14165246690716487359 => {}
                                _ => {
                                    if sfx as i32 != 0 as i32 {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.offset(c2rust_fresh8 as isize) as i32
                                                != sfx as i32
                                            {
                                                c2rust_current_block = 14165246690716487359;
                                            } else {
                                                c2rust_current_block = 2122094917359643297;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 2122094917359643297;
                                    }
                                    match c2rust_current_block {
                                        14165246690716487359 => {}
                                        _ => {
                                            expanded_value = std::ptr::null_mut();
                                            if mlv as u32 == MLVO_LAYOUT as u32 {
                                                if idx == XKB_LAYOUT_INVALID as u32 {
                                                    if (*m).rmlvo.layouts.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .layouts
                                                            .as_mut_ptr()
                                                            .offset(0 as i32 as isize)
                                                            as *mut matched_sval;
                                                    }
                                                } else if idx < (*m).rmlvo.layouts.len() as u32
                                                    && (expanded_index as i32 != 0
                                                        || (*m).rmlvo.layouts.len() > 1)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .layouts
                                                        .as_mut_ptr()
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32 == MLVO_VARIANT as u32 {
                                                if idx == XKB_LAYOUT_INVALID as u32 {
                                                    if (*m).rmlvo.variants.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .variants
                                                            .as_mut_ptr()
                                                            .offset(0 as i32 as isize)
                                                            as *mut matched_sval;
                                                    }
                                                } else if idx < (*m).rmlvo.variants.len() as u32
                                                    && (expanded_index as i32 != 0
                                                        || (*m).rmlvo.variants.len() > 1)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .variants
                                                        .as_mut_ptr()
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32 == MLVO_MODEL as u32 {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return true;
                                            }
                                            if pfx as i32 != 0 as i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const pfx as *const i8,
                                                    1,
                                                );
                                            }
                                            vec_append_nul_terminated(
                                                &mut *expanded,
                                                (*expanded_value).sval.start,
                                                (*expanded_value).sval.len as u32,
                                            );
                                            if sfx as i32 != 0 as i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                            }
                                            (*expanded_value).matched = (true) as bool;
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                        108 => {
                            c2rust_current_block = 14869399783518996101;
                            match c2rust_current_block {
                                17806387538889038492 => {
                                    mlv = MLVO_VARIANT;
                                }
                                14869399783518996101 => {
                                    mlv = MLVO_LAYOUT;
                                }
                                _ => {
                                    mlv = MLVO_MODEL;
                                }
                            }
                            idx = XKB_LAYOUT_INVALID as u32;
                            expanded_index = false;
                            if *i < value.len && *str.offset(*i as isize) as i32 == '[' as i32 {
                                if mlv as u32 != MLVO_LAYOUT as u32
                                    && mlv as u32 != MLVO_VARIANT as u32
                                {
                                    let mut loc_0: scanner_loc = (*s).token_location();
                                    xkb_logf!(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                                        crate::xkb::utils::CStrDisplay((*s).file_name),
                                        loc_0.line,
                                        loc_0.column,
                                    );
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let mut consumed: i32 = extract_layout_index(
                                        str.offset(*i as isize),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1 as i32 {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID as u32 {
                                            idx = layout_idx;
                                            expanded_index = true;
                                        }
                                        *i = (*i).wrapping_add(consumed as usize);
                                        c2rust_current_block = 10758786907990354186;
                                    }
                                }
                            } else {
                                c2rust_current_block = 10758786907990354186;
                            }
                            match c2rust_current_block {
                                14165246690716487359 => {}
                                _ => {
                                    if sfx as i32 != 0 as i32 {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.offset(c2rust_fresh8 as isize) as i32
                                                != sfx as i32
                                            {
                                                c2rust_current_block = 14165246690716487359;
                                            } else {
                                                c2rust_current_block = 2122094917359643297;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 2122094917359643297;
                                    }
                                    match c2rust_current_block {
                                        14165246690716487359 => {}
                                        _ => {
                                            expanded_value = std::ptr::null_mut();
                                            if mlv as u32 == MLVO_LAYOUT as u32 {
                                                if idx == XKB_LAYOUT_INVALID as u32 {
                                                    if (*m).rmlvo.layouts.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .layouts
                                                            .as_mut_ptr()
                                                            .offset(0 as i32 as isize)
                                                            as *mut matched_sval;
                                                    }
                                                } else if idx < (*m).rmlvo.layouts.len() as u32
                                                    && (expanded_index as i32 != 0
                                                        || (*m).rmlvo.layouts.len() > 1)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .layouts
                                                        .as_mut_ptr()
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32 == MLVO_VARIANT as u32 {
                                                if idx == XKB_LAYOUT_INVALID as u32 {
                                                    if (*m).rmlvo.variants.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .variants
                                                            .as_mut_ptr()
                                                            .offset(0 as i32 as isize)
                                                            as *mut matched_sval;
                                                    }
                                                } else if idx < (*m).rmlvo.variants.len() as u32
                                                    && (expanded_index as i32 != 0
                                                        || (*m).rmlvo.variants.len() > 1)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .variants
                                                        .as_mut_ptr()
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32 == MLVO_MODEL as u32 {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return true;
                                            }
                                            if pfx as i32 != 0 as i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const pfx as *const i8,
                                                    1,
                                                );
                                            }
                                            vec_append_nul_terminated(
                                                &mut *expanded,
                                                (*expanded_value).sval.start,
                                                (*expanded_value).sval.len as u32,
                                            );
                                            if sfx as i32 != 0 as i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                            }
                                            (*expanded_value).matched = (true) as bool;
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                        118 => {
                            c2rust_current_block = 17806387538889038492;
                            match c2rust_current_block {
                                17806387538889038492 => {
                                    mlv = MLVO_VARIANT;
                                }
                                14869399783518996101 => {
                                    mlv = MLVO_LAYOUT;
                                }
                                _ => {
                                    mlv = MLVO_MODEL;
                                }
                            }
                            idx = XKB_LAYOUT_INVALID as u32;
                            expanded_index = false;
                            if *i < value.len && *str.offset(*i as isize) as i32 == '[' as i32 {
                                if mlv as u32 != MLVO_LAYOUT as u32
                                    && mlv as u32 != MLVO_VARIANT as u32
                                {
                                    let mut loc_0: scanner_loc = (*s).token_location();
                                    xkb_logf!(
                                        (*s).ctx,
                                        XKB_LOG_LEVEL_ERROR,
                                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                                        "[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                                        crate::xkb::utils::CStrDisplay((*s).file_name),
                                        loc_0.line,
                                        loc_0.column,
                                    );
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let mut consumed: i32 = extract_layout_index(
                                        str.offset(*i as isize),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1 as i32 {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID as u32 {
                                            idx = layout_idx;
                                            expanded_index = true;
                                        }
                                        *i = (*i).wrapping_add(consumed as usize);
                                        c2rust_current_block = 10758786907990354186;
                                    }
                                }
                            } else {
                                c2rust_current_block = 10758786907990354186;
                            }
                            match c2rust_current_block {
                                14165246690716487359 => {}
                                _ => {
                                    if sfx as i32 != 0 as i32 {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.offset(c2rust_fresh8 as isize) as i32
                                                != sfx as i32
                                            {
                                                c2rust_current_block = 14165246690716487359;
                                            } else {
                                                c2rust_current_block = 2122094917359643297;
                                            }
                                        }
                                    } else {
                                        c2rust_current_block = 2122094917359643297;
                                    }
                                    match c2rust_current_block {
                                        14165246690716487359 => {}
                                        _ => {
                                            expanded_value = std::ptr::null_mut();
                                            if mlv as u32 == MLVO_LAYOUT as u32 {
                                                if idx == XKB_LAYOUT_INVALID as u32 {
                                                    if (*m).rmlvo.layouts.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .layouts
                                                            .as_mut_ptr()
                                                            .offset(0 as i32 as isize)
                                                            as *mut matched_sval;
                                                    }
                                                } else if idx < (*m).rmlvo.layouts.len() as u32
                                                    && (expanded_index as i32 != 0
                                                        || (*m).rmlvo.layouts.len() > 1)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .layouts
                                                        .as_mut_ptr()
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32 == MLVO_VARIANT as u32 {
                                                if idx == XKB_LAYOUT_INVALID as u32 {
                                                    if (*m).rmlvo.variants.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .variants
                                                            .as_mut_ptr()
                                                            .offset(0 as i32 as isize)
                                                            as *mut matched_sval;
                                                    }
                                                } else if idx < (*m).rmlvo.variants.len() as u32
                                                    && (expanded_index as i32 != 0
                                                        || (*m).rmlvo.variants.len() > 1)
                                                {
                                                    expanded_value = (*m)
                                                        .rmlvo
                                                        .variants
                                                        .as_mut_ptr()
                                                        .offset(idx as isize)
                                                        as *mut matched_sval;
                                                }
                                            } else if mlv as u32 == MLVO_MODEL as u32 {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0 as usize
                                            {
                                                return true;
                                            }
                                            if pfx as i32 != 0 as i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const pfx as *const i8,
                                                    1,
                                                );
                                            }
                                            vec_append_nul_terminated(
                                                &mut *expanded,
                                                (*expanded_value).sval.start,
                                                (*expanded_value).sval.len as u32,
                                            );
                                            if sfx as i32 != 0 as i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                            }
                                            (*expanded_value).matched = (true) as bool;
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let mut loc_1: scanner_loc = (*s).token_location();
        xkb_logf!(
            (*s).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            crate::xkb::utils::CStrDisplay((*s).file_name),
            loc_1.line,
            loc_1.column,
        );
        return false;
    }
}
unsafe fn expand_qualifier_in_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut value: sval,
    mut expanded: *mut Vec<i8>,
    mut has_layout_idx_range: bool,
    mut has_separator: bool,
    mut prefix_idx: u32,
    mut i: *mut usize,
) {
    unsafe {
        let mut str: *const i8 = value.start;
        if ((*i).wrapping_add(3 as usize) <= value.len
            || (*str.offset((*i).wrapping_add(3 as usize) as isize) as i32
                == MERGE_OVERRIDE_PREFIX
                || *str.offset((*i).wrapping_add(3 as usize) as isize) as i32
                    == MERGE_AUGMENT_PREFIX
                || *str.offset((*i).wrapping_add(3 as usize) as isize) as i32
                    == MERGE_REPLACE_PREFIX))
            && *str.offset(*i as isize) as i32 == 'a' as i32
            && *str.offset((*i).wrapping_add(1 as usize) as isize) as i32 == 'l' as i32
            && *str.offset((*i).wrapping_add(2 as usize) as isize) as i32 == 'l' as i32
        {
            if has_layout_idx_range {
                let mut loc: scanner_loc = (*s).token_location();
                xkb_logf!(
                    (*s).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_DETAILED as i32,
                    "{}:{}:{}: Using :all qualifier with indices range is not recommended.\n",
                    crate::xkb::utils::CStrDisplay((*s).file_name),
                    loc.line,
                    loc.column,
                );
            }
            vec_append_nul_terminated(&mut *expanded, b"1\0".as_ptr() as *const i8, 1);
            if (*m).rmlvo.layouts.len() > 1 {
                let mut layout_index: [i8; 12] = [0; 12];
                let prefix_length = (*expanded)
                    .len()
                    .wrapping_sub(prefix_idx as usize)
                    .wrapping_sub(1);
                let mut l: u32 = 1 as u32;
                while l
                    < (if 32 < (*m).rmlvo.layouts.len() {
                        32 as u32
                    } else {
                        (*m).rmlvo.layouts.len() as u32
                    })
                {
                    if !has_separator {
                        (*expanded).push('+' as i32 as i8);
                    }
                    {
                        let old_size = (*expanded).len();
                        let new_size = old_size.wrapping_add(prefix_length).wrapping_add(1);
                        (*expanded).resize(new_size, 0);
                        std::ptr::copy_nonoverlapping(
                            (*expanded).as_ptr().offset(prefix_idx as isize),
                            (*expanded).as_mut_ptr().offset(old_size as isize),
                            prefix_length as usize,
                        );
                        *(*expanded)
                            .as_mut_ptr()
                            .offset(new_size.wrapping_sub(1) as isize) = 0;
                        (*expanded).truncate(new_size.wrapping_sub(1));
                    }
                    let mut count: i32 = crate::xkb::utils::snprintf_c(
                        &raw mut layout_index as *mut i8,
                        std::mem::size_of::<[i8; 12]>(),
                        format_args!("{}", l.wrapping_add(1 as u32)),
                    );
                    vec_append_nul_terminated(
                        &mut *expanded,
                        &raw mut layout_index as *mut i8 as *const i8,
                        count as u32,
                    );
                    l = l.wrapping_add(1);
                }
            }
            *i = (*i).wrapping_add(3 as usize);
        }
    }
}
#[inline]
unsafe fn concat_kccgst(mut into: *mut Vec<i8>, mut size: u32, mut from: *const i8) {
    unsafe {
        let from_plus: bool = *from.offset(0 as i32 as isize) as i32 == MERGE_OVERRIDE_PREFIX
            || *from.offset(0 as i32 as isize) as i32 == MERGE_AUGMENT_PREFIX
            || *from.offset(0 as i32 as isize) as i32 == MERGE_REPLACE_PREFIX;
        if from_plus as i32 != 0 || (*into).len() == 0 {
            vec_append_nul_terminated(&mut *into, from, size as u32);
        } else {
            let ch: i8 = (if (*into).len() == 0 {
                '\0' as i32
            } else {
                *(*into).as_ptr().offset(0 as i32 as isize) as i32
            }) as i8;
            let into_plus: bool = ch as i32 == MERGE_OVERRIDE_PREFIX
                || ch as i32 == MERGE_AUGMENT_PREFIX
                || ch as i32 == MERGE_REPLACE_PREFIX;
            if into_plus {
                // Insert `from` data at the beginning of the Vec
                let from_slice = std::slice::from_raw_parts(from, size as usize);
                let old_len = (*into).len();
                (*into).resize(old_len + size as usize, 0);
                // Shift existing content to the right
                std::ptr::copy(
                    (*into).as_ptr(),
                    (*into).as_mut_ptr().offset(size as isize),
                    old_len,
                );
                // Copy `from` data to the beginning
                std::ptr::copy_nonoverlapping(
                    from_slice.as_ptr(),
                    (*into).as_mut_ptr(),
                    size as usize,
                );
            }
        };
    }
}
unsafe fn append_expanded_kccgst_value(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut merge: bool,
    mut to: *mut Vec<i8>,
    mut value: sval,
    mut layout_idx: u32,
) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut str: *const i8 = value.start;
        let mut expanded: Vec<i8> = Vec::new();
        let mut last_item_idx: u32 = 0;
        let mut has_separator: bool = false;
        let mut i: usize = 0 as usize;
        loop {
            if !(i < value.len) {
                c2rust_current_block = 10758786907990354186;
                break;
            }
            match *str.offset(i as isize) as i32 {
                58 => {
                    let c2rust_fresh4 = i;
                    i = i.wrapping_add(1);
                    vec_append_nul_terminated(&mut expanded, str.offset(c2rust_fresh4 as isize), 1);
                    expand_qualifier_in_kccgst_value(
                        m,
                        s,
                        value,
                        &raw mut expanded,
                        (*m).mapping.has_layout_idx_range,
                        has_separator,
                        last_item_idx,
                        &raw mut i,
                    );
                }
                37 => {
                    i = i.wrapping_add(1);
                    if i >= value.len
                        || !expand_rmlvo_in_kccgst_value(
                            m,
                            s,
                            value,
                            layout_idx,
                            &raw mut expanded,
                            &raw mut i,
                        )
                    {
                        c2rust_current_block = 1032266188497003083;
                        break;
                    }
                }
                MERGE_OVERRIDE_PREFIX | MERGE_AUGMENT_PREFIX | MERGE_REPLACE_PREFIX => {
                    let c2rust_fresh5 = i;
                    i = i.wrapping_add(1);
                    vec_append_nul_terminated(&mut expanded, str.offset(c2rust_fresh5 as isize), 1);
                    last_item_idx = expanded.len().wrapping_sub(1) as u32;
                    has_separator = true;
                }
                _ => {
                    let c2rust_fresh6 = i;
                    i = i.wrapping_add(1);
                    vec_append_nul_terminated(&mut expanded, str.offset(c2rust_fresh6 as isize), 1);
                }
            }
        }
        match c2rust_current_block {
            1032266188497003083 => {
                drop(expanded);
                return false;
            }
            _ => {
                if merge {
                    if !(expanded.len() == 0) {
                        concat_kccgst(to, expanded.len() as u32, expanded.as_ptr());
                    }
                } else if expanded.len() > 0 {
                    (*to).extend_from_slice(&expanded);
                }
                drop(expanded);
                return true;
            }
        };
    }
}
unsafe fn matcher_append_pending_kccgst(mut m: *mut matcher) -> bool {
    unsafe {
        if !(*m).mapping.has_layout_idx_range {
            return true;
        }
        let mut i: kccgst_index_t = 0 as kccgst_index_t;
        while (i as i32) < (*m).mapping.num_kccgst as i32 {
            let kccgst: rules_kccgst = (*m).mapping.kccgst_at_pos[i as usize];
            let mut layout: u32 = (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
            while layout < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max {
                let buf: *const kccgst_buffer = &raw mut (*m).pending_kccgst;
                let mut offset: usize = 0 as usize;
                let mut k: u32 = 0;
                while k < (&(*buf).slices).len() as u32 {
                    let slice: &kccgst_buffer_slice = &(&(*buf).slices)[k as usize];
                    if slice.kccgst as u32 == kccgst as u32
                        && slice.layout == layout
                        && slice.length as i32 != 0
                    {
                        concat_kccgst(
                            &mut (*m).kccgst[kccgst as usize],
                            slice.length,
                            (*buf).buffer.as_ptr().offset(offset as isize),
                        );
                    }
                    offset = offset.wrapping_add(slice.length as usize);
                    k = k.wrapping_add(1);
                }
                layout = layout.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*m).mapping.has_layout_idx_range = false;
        return true;
    }
}
unsafe fn matcher_rule_verify(mut m: *mut matcher, mut s: *mut scanner) {
    unsafe {
        if (*m).rule.num_mlvo_values as i32 != (*m).mapping.num_mlvo as i32
            || (*m).rule.num_kccgst_values as i32 != (*m).mapping.num_kccgst as i32
        {
            let mut loc: scanner_loc = (*s).token_location();
            xkb_logf!(
                (*s).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: invalid rule: must have same number of values as mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                crate::xkb::utils::CStrDisplay((*s).file_name),
                loc.line,
                loc.column,
            );
            (*m).rule.skip = true;
        }
    }
}
unsafe fn matcher_rule_apply_if_matches(mut m: *mut matcher, mut s: *mut scanner) {
    unsafe {
        let mut candidate_layouts: u32 = (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask;
        let mut idx: u32 = 0;
        let mut i: mlvo_index_t = 0 as mlvo_index_t;
        while (i as i32) < (*m).mapping.num_mlvo as i32 {
            let mut mlvo: rules_mlvo = (*m).mapping.mlvo_at_pos[i as usize];
            let mut value: sval = (*m).rule.mlvo_value_at_pos[i as usize];
            let mut match_type: mlvo_match_type = (*m).rule.match_type_at_pos[i as usize];
            let mut to: *mut matched_sval = std::ptr::null_mut();
            let mut matched: bool = false;
            if mlvo as u32 == MLVO_MODEL as u32 {
                to = &raw mut (*m).rmlvo.model;
                matched = match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_ALL);
            } else if (*m).mapping.has_layout_idx_range {
                idx = (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
                while idx < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max
                    && candidate_layouts != 0
                {
                    let mask: u32 = (1 as u32) << idx;
                    if candidate_layouts & mask != 0 {
                        match mlvo as u32 {
                            1 => {
                                to = (*m).rmlvo.layouts.as_mut_ptr().offset(idx as isize)
                                    as *mut matched_sval;
                                if match_value_and_mark(
                                    m,
                                    value,
                                    to,
                                    match_type,
                                    WILDCARD_MATCH_NONEMPTY,
                                ) {
                                    matched = true;
                                } else {
                                    candidate_layouts &= !mask;
                                }
                            }
                            2 => {
                                to = (*m).rmlvo.variants.as_mut_ptr().offset(idx as isize)
                                    as *mut matched_sval;
                                if match_value_and_mark(
                                    m,
                                    value,
                                    to,
                                    match_type,
                                    WILDCARD_MATCH_NONEMPTY,
                                ) {
                                    matched = true;
                                } else {
                                    candidate_layouts &= !mask;
                                }
                            }
                            _ => {
                                let mut found_option: bool = false;
                                if !(*m).rmlvo.options.is_empty() {
                                    to = (*m).rmlvo.options.as_mut_ptr().offset(0 as i32 as isize)
                                        as *mut matched_sval;
                                    while to
                                        < (*m)
                                            .rmlvo
                                            .options
                                            .as_mut_ptr()
                                            .offset((*m).rmlvo.options.len() as isize)
                                            as *mut matched_sval
                                    {
                                        if !((*to).layout as i32 != OPTIONS_MATCH_ALL_GROUPS
                                            && (*to).layout != idx)
                                        {
                                            if match_value_and_mark(
                                                m,
                                                value,
                                                to,
                                                match_type,
                                                WILDCARD_MATCH_ALL,
                                            ) {
                                                matched = true;
                                                found_option = true;
                                                break;
                                            }
                                        }
                                        to = to.offset(1);
                                    }
                                }
                                if !found_option {
                                    candidate_layouts &= !mask;
                                }
                            }
                        }
                    }
                    idx = idx.wrapping_add(1);
                }
            } else {
                match mlvo as u32 {
                    1 => {
                        to = (*m).rmlvo.layouts.as_mut_ptr().offset(
                            (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min as isize,
                        ) as *mut matched_sval;
                        matched =
                            match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_NONEMPTY);
                    }
                    2 => {
                        to = (*m).rmlvo.variants.as_mut_ptr().offset(
                            (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min as isize,
                        ) as *mut matched_sval;
                        matched =
                            match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_NONEMPTY);
                    }
                    _ => {
                        if !(*m).rmlvo.options.is_empty() {
                            to = (*m).rmlvo.options.as_mut_ptr().offset(0 as i32 as isize)
                                as *mut matched_sval;
                            while to
                                < (*m)
                                    .rmlvo
                                    .options
                                    .as_mut_ptr()
                                    .offset((*m).rmlvo.options.len() as isize)
                                    as *mut matched_sval
                            {
                                if !((*to).layout as i32 != OPTIONS_MATCH_ALL_GROUPS
                                    && (*to).layout
                                        != (*m)
                                            .mapping
                                            .c2rust_unnamed
                                            .c2rust_unnamed_0
                                            .layout_idx_min)
                                {
                                    matched = match_value_and_mark(
                                        m,
                                        value,
                                        to,
                                        match_type,
                                        WILDCARD_MATCH_ALL,
                                    );
                                    if matched {
                                        break;
                                    }
                                }
                                to = to.offset(1);
                            }
                        }
                    }
                }
            }
            if !matched {
                return;
            }
            i = i.wrapping_add(1);
        }
        if (*m).mapping.has_layout_idx_range {
            idx = (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min;
            while idx < (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_max {
                if candidate_layouts & (1 as u32) << idx != 0 {
                    let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
                    while (i_0 as i32) < (*m).mapping.num_kccgst as i32 {
                        let kccgst: rules_kccgst = (*m).mapping.kccgst_at_pos[i_0 as usize];
                        let value_0: sval = (*m).rule.kccgst_value_at_pos[i_0 as usize];
                        let buf: *mut kccgst_buffer = &raw mut (*m).pending_kccgst;
                        let prev_buffer_length: u32 = (*buf).buffer.len() as u32;
                        append_expanded_kccgst_value(
                            m,
                            s,
                            false,
                            &raw mut (*buf).buffer,
                            value_0,
                            idx,
                        );
                        let length: u32 =
                            ((*buf).buffer.len() as u32).wrapping_sub(prev_buffer_length) as u32;
                        let slice = kccgst_buffer_slice {
                            length,
                            kccgst,
                            layout: idx,
                        };
                        (&mut (*buf).slices).push(slice);
                        i_0 = i_0.wrapping_add(1);
                    }
                }
                idx = idx.wrapping_add(1);
            }
        } else {
            let mut i_1: kccgst_index_t = 0 as kccgst_index_t;
            while (i_1 as i32) < (*m).mapping.num_kccgst as i32 {
                let mut kccgst_0: rules_kccgst = (*m).mapping.kccgst_at_pos[i_1 as usize];
                let mut value_1: sval = (*m).rule.kccgst_value_at_pos[i_1 as usize];
                append_expanded_kccgst_value(
                    m,
                    s,
                    true,
                    &mut (*m).kccgst[kccgst_0 as usize],
                    value_1,
                    (*m).mapping.c2rust_unnamed.c2rust_unnamed_0.layout_idx_min,
                );
                i_1 = i_1.wrapping_add(1);
            }
        }
        if !is_mlvo_mask_defined(m, MLVO_OPTION) {
            (*m).mapping.c2rust_unnamed_0.layouts_candidates_mask &= !candidate_layouts;
        }
    }
}
unsafe fn gettok(mut m: *mut matcher, mut s: *mut scanner) -> rules_token {
    unsafe {
        return lex(s, &raw mut (*m).val);
    }
}
unsafe fn matcher_match(
    mut m: *mut matcher,
    mut s: *mut scanner,
    mut include_depth: u32,
    mut string: *const i8,
    mut len: usize,
    mut file_name: *const i8,
) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut tok: rules_token = TOK_END_OF_FILE;
        if m.is_null() {
            return false;
        }
        '_initial: loop {
            tok = gettok(m, s);
            match tok as u32 {
                4 => {}
                1 => {
                    continue;
                }
                0 => {
                    c2rust_current_block = 13194772801876125100;
                    break;
                }
                _ => {
                    c2rust_current_block = 2196083827608010402;
                    break;
                }
            }
            loop {
                tok = gettok(m, s);
                match tok as u32 {
                    3 => {
                        matcher_group_start_new(m, (*m).val.string);
                        tok = gettok(m, s);
                        match tok as u32 {
                            5 => {
                                break;
                            }
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                    }
                    10 => {
                        tok = gettok(m, s);
                        match tok as u32 {
                            2 => {}
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                        matcher_include(m, s, include_depth, (*m).val.string);
                        tok = gettok(m, s);
                        match tok as u32 {
                            1 => {
                                continue '_initial;
                            }
                            _ => {
                                c2rust_current_block = 2196083827608010402;
                                break '_initial;
                            }
                        }
                    }
                    2 => {
                        matcher_mapping_start_new(m);
                        matcher_mapping_set_mlvo(m, s, (*m).val.string);
                        loop {
                            tok = gettok(m, s);
                            match tok as u32 {
                                2 => {}
                                5 => {
                                    break;
                                }
                                _ => {
                                    c2rust_current_block = 2196083827608010402;
                                    break '_initial;
                                }
                            }
                            if (*m).mapping.c2rust_unnamed_0.active != 0 {
                                matcher_mapping_set_mlvo(m, s, (*m).val.string);
                            }
                        }
                        loop {
                            tok = gettok(m, s);
                            match tok as u32 {
                                2 => {}
                                1 => {
                                    break;
                                }
                                _ => {
                                    c2rust_current_block = 2196083827608010402;
                                    break '_initial;
                                }
                            }
                            if (*m).mapping.c2rust_unnamed_0.active != 0 {
                                matcher_mapping_set_kccgst(m, s, (*m).val.string);
                            }
                        }
                        if (*m).mapping.c2rust_unnamed_0.active != 0
                            && matcher_mapping_verify(m, s) as i32 != 0
                        {
                            matcher_mapping_set_layout_bounds(m);
                            if (*m).mapping.has_layout_idx_range {
                                (*m).pending_kccgst.buffer.clear();
                                (*m).pending_kccgst.slices.clear();
                            }
                        }
                        loop {
                            tok = gettok(m, s);
                            match tok as u32 {
                                4 => {
                                    matcher_append_pending_kccgst(m);
                                    break;
                                }
                                1 => {}
                                0 => {
                                    matcher_append_pending_kccgst(m);
                                    c2rust_current_block = 13194772801876125100;
                                    break '_initial;
                                }
                                _ => {
                                    matcher_rule_start_new(m);
                                    loop {
                                        match tok as u32 {
                                            2 => {
                                                if !(*m).rule.skip {
                                                    if (*m).val.string.len == 1 as usize
                                                        && *(*m)
                                                            .val
                                                            .string
                                                            .start
                                                            .offset(0 as i32 as isize)
                                                            as i32
                                                            == '+' as i32
                                                    {
                                                        matcher_rule_set_mlvo_wildcard(
                                                            m,
                                                            s,
                                                            MLVO_MATCH_WILDCARD_SOME,
                                                        );
                                                    } else {
                                                        matcher_rule_set_mlvo(
                                                            m,
                                                            s,
                                                            (*m).val.string,
                                                        );
                                                    }
                                                }
                                            }
                                            6 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_LEGACY,
                                                    );
                                                }
                                            }
                                            7 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_NONE,
                                                    );
                                                }
                                            }
                                            8 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_SOME,
                                                    );
                                                }
                                            }
                                            9 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_ANY,
                                                    );
                                                }
                                            }
                                            3 => {
                                                if !(*m).rule.skip {
                                                    matcher_rule_set_mlvo_group(
                                                        m,
                                                        s,
                                                        (*m).val.string,
                                                    );
                                                }
                                            }
                                            5 => {
                                                break;
                                            }
                                            _ => {
                                                c2rust_current_block = 2196083827608010402;
                                                break '_initial;
                                            }
                                        }
                                        tok = gettok(m, s);
                                    }
                                    loop {
                                        tok = gettok(m, s);
                                        match tok as u32 {
                                            2 => {}
                                            1 => {
                                                break;
                                            }
                                            _ => {
                                                c2rust_current_block = 2196083827608010402;
                                                break '_initial;
                                            }
                                        }
                                        if !(*m).rule.skip {
                                            matcher_rule_set_kccgst(m, s, (*m).val.string);
                                        }
                                    }
                                    if !(*m).rule.skip {
                                        matcher_rule_verify(m, s);
                                    }
                                    if !(*m).rule.skip {
                                        matcher_rule_apply_if_matches(m, s);
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        c2rust_current_block = 2196083827608010402;
                        break '_initial;
                    }
                }
            }
            loop {
                tok = gettok(m, s);
                match tok as u32 {
                    2 => {}
                    1 => {
                        break;
                    }
                    _ => {
                        c2rust_current_block = 2196083827608010402;
                        break '_initial;
                    }
                }
                matcher_group_add_element(m, s, (*m).val.string);
            }
        }
        match c2rust_current_block {
            13194772801876125100 => return true,
            _ => {
                match tok as u32 {
                    11 => {}
                    _ => {
                        let mut loc: scanner_loc = (*s).token_location();
                        xkb_logf!(
                            (*s).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {}:{}:{}: unexpected token\n",
                            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                            crate::xkb::utils::CStrDisplay((*s).file_name),
                            loc.line,
                            loc.column,
                        );
                    }
                }
                return false;
            }
        };
    }
}
unsafe fn read_rules_file(
    mut ctx: *mut xkb_context,
    mut matcher: *mut matcher,
    mut include_depth: u32,
    mut file: *mut FILE,
    mut path: *const i8,
) -> bool {
    unsafe {
        let mut ret: bool = false;
        let mut scanner: scanner = scanner::new(
            std::ptr::null_mut(),
            std::ptr::null(),
            0,
            std::ptr::null(),
            std::ptr::null_mut(),
        );

        // Convert FILE* to Rust File and map it
        use crate::xkb::utils::MappedFile;
        use std::fs::File;
        use std::os::unix::io::FromRawFd;

        let fd = libc::fileno(file as *mut libc::FILE);
        if fd < 0 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Invalid file descriptor\n",
            );
            return false;
        }

        let rust_file = File::from_raw_fd(fd);
        let mapped = match MappedFile::new(&rust_file) {
            Ok(m) => m,
            Err(e) => {
                let err_msg = std::ffi::CString::new(e.to_string())
                    .unwrap_or_else(|_| std::ffi::CString::new("unknown error").unwrap());
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Couldn't read rules file \"{}\": {}\n",
                    crate::xkb::utils::CStrDisplay(path),
                    crate::xkb::utils::CStrDisplay(err_msg.as_ptr()),
                );
                std::mem::forget(rust_file);
                return false;
            }
        };

        scanner = scanner::new(
            (*matcher).ctx,
            mapped.as_ptr(),
            mapped.len(),
            path,
            std::ptr::null_mut::<core::ffi::c_void>(),
        );
        if !scanner.check_supported_char_encoding() {
            let mut loc: scanner_loc = scanner.token_location();
            xkb_logf!(
                scanner.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                crate::xkb::utils::CStrDisplay(scanner.file_name),
                loc.line,
                loc.column,
            );
            let mut loc_0: scanner_loc = scanner.token_location();
            xkb_logf!(
                scanner.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}:{}:{}: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                crate::xkb::utils::CStrDisplay(scanner.file_name),
                loc_0.line,
                loc_0.column,
            );
            std::mem::forget(rust_file);
            return false;
        }
        ret = matcher_match(
            matcher,
            &raw mut scanner,
            include_depth,
            mapped.as_ptr(),
            mapped.len(),
            path,
        );
        std::mem::forget(rust_file);
        return ret;
    }
}
unsafe fn xkb_resolve_partial_rules(
    mut ctx: *mut xkb_context,
    mut path: *mut i8,
    mut path_size: usize,
    mut rules: *const i8,
    mut suffix: *const i8,
    mut matcher: *mut matcher,
) -> bool {
    unsafe {
        let mut partial_rules: [i8; 60] = [0; 60];
        let (_, _trunc) = crate::xkb::utils::snprintf_args(
            &raw mut partial_rules as *mut i8,
            std::mem::size_of::<[i8; 60]>(),
            format_args!(
                "{}{}",
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(suffix)
            ),
        );
        if _trunc {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot load XKB rules \"{}{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(suffix),
            );
            return false;
        }
        let mut offset: u32 = 0 as u32;
        let mut file: *mut FILE = std::ptr::null_mut();
        let len: usize = cstr_len(&raw mut partial_rules as *mut i8) as usize;
        loop {
            file = FindFileInXkbPath(
                ctx,
                b"(unknown)\0".as_ptr() as *const i8,
                &raw mut partial_rules as *mut i8,
                len,
                FILE_TYPE_RULES,
                path,
                path_size,
                &raw mut offset,
                false,
            );
            if file.is_null() {
                break;
            }
            let ok: bool = read_rules_file(ctx, matcher, 0 as u32, file, path) as bool;
            fclose(file);
            if !ok {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                    crate::xkb::utils::CStrDisplay(path),
                );
                return false;
            }
            offset = offset.wrapping_add(1);
        }
        return true;
    }
}
unsafe fn xkb_resolve_rules(
    mut ctx: *mut xkb_context,
    mut rules: *const i8,
    mut matcher: *mut matcher,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut u32,
) -> bool {
    unsafe {
        let mut mval: *mut matched_sval = std::ptr::null_mut();
        let mut ret: bool = false;
        let mut offset: u32 = 0 as u32;
        let mut path: [i8; 4096] = [0; 4096];
        let file: *mut FILE = FindFileInXkbPath(
            ctx,
            b"(unknown)\0".as_ptr() as *const i8,
            rules,
            cstr_len(rules),
            FILE_TYPE_RULES,
            &raw mut path as *mut i8,
            std::mem::size_of::<[i8; 4096]>(),
            &raw mut offset,
            true,
        ) as *mut FILE;
        if file.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot load XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                crate::xkb::utils::CStrDisplay(rules),
            );
        } else {
            ret = xkb_resolve_partial_rules(
                ctx,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                rules,
                b".pre\0".as_ptr() as *const i8,
                matcher,
            );
            if ret {
                ret = read_rules_file(ctx, matcher, 0 as u32, file, &raw mut path as *mut i8);
                if !ret {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                        XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                        crate::xkb::utils::CStrDisplay(&raw mut path as *mut i8),
                    );
                } else {
                    ret = xkb_resolve_partial_rules(
                        ctx,
                        &raw mut path as *mut i8,
                        std::mem::size_of::<[i8; 4096]>(),
                        rules,
                        b".post\0".as_ptr() as *const i8,
                        matcher,
                    );
                    if ret {
                        if (*matcher).kccgst[KCCGST_KEYCODES as usize].len() == 0
                            || (*matcher).kccgst[KCCGST_TYPES as usize].len() == 0
                            || (*matcher).kccgst[KCCGST_COMPAT as usize].len() == 0
                            || (*matcher).kccgst[KCCGST_SYMBOLS as usize].len() == 0
                        {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] No components returned from XKB rules \"{}\"\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                crate::xkb::utils::CStrDisplay(rules),
                            );
                            ret = false;
                        } else {
                            // Transfer ownership of Vec data to raw pointers.
                            // Each Vec is taken (replaced with empty Vec), nul-terminated,
                            // and leaked so the caller owns the allocation as a C string.
                            {
                                let mut v = std::mem::take(
                                    &mut (*matcher).kccgst[KCCGST_KEYCODES as usize],
                                );
                                v.push(0);
                                (*out).keycodes = Vec::leak(v).as_mut_ptr();
                            }
                            {
                                let mut v =
                                    std::mem::take(&mut (*matcher).kccgst[KCCGST_TYPES as usize]);
                                v.push(0);
                                (*out).types = Vec::leak(v).as_mut_ptr();
                            }
                            {
                                let mut v =
                                    std::mem::take(&mut (*matcher).kccgst[KCCGST_COMPAT as usize]);
                                v.push(0);
                                (*out).compatibility = Vec::leak(v).as_mut_ptr();
                            }
                            {
                                let mut v =
                                    std::mem::take(&mut (*matcher).kccgst[KCCGST_SYMBOLS as usize]);
                                v.push(0);
                                (*out).symbols = Vec::leak(v).as_mut_ptr();
                            }
                            {
                                let mut v = std::mem::take(
                                    &mut (*matcher).kccgst[KCCGST_GEOMETRY as usize],
                                );
                                v.push(0);
                                (*out).geometry = Vec::leak(v).as_mut_ptr();
                            }
                            mval = &raw mut (*matcher).rmlvo.model;
                            if !(*mval).matched && (*mval).sval.len > 0 as usize {
                                xkb_logf!(
                                    (*matcher).ctx,
                                    XKB_LOG_LEVEL_ERROR,
                                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                                    "[XKB-{:03}] Unrecognized RMLVO model \"{}\" was ignored\n",
                                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                    crate::xkb::utils::CStrNDisplay(
                                        (*mval).sval.len as usize,
                                        (*mval).sval.start
                                    ),
                                );
                            }
                            if !(*matcher).rmlvo.layouts.is_empty() {
                                mval = (*matcher)
                                    .rmlvo
                                    .layouts
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .layouts
                                        .as_mut_ptr()
                                        .offset((*matcher).rmlvo.layouts.len() as isize)
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched && (*mval).sval.len > 0 as usize {
                                        xkb_logf!(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                                            "[XKB-{:03}] Unrecognized RMLVO layout \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                            crate::xkb::utils::CStrNDisplay((*mval).sval.len as usize, (*mval).sval.start),
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*matcher).rmlvo.variants.is_empty() {
                                mval = (*matcher)
                                    .rmlvo
                                    .variants
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .variants
                                        .as_mut_ptr()
                                        .offset((*matcher).rmlvo.variants.len() as isize)
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched && (*mval).sval.len > 0 as usize {
                                        xkb_logf!(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                                            "[XKB-{:03}] Unrecognized RMLVO variant \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                            crate::xkb::utils::CStrNDisplay((*mval).sval.len as usize, (*mval).sval.start),
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*matcher).rmlvo.options.is_empty() {
                                mval = (*matcher)
                                    .rmlvo
                                    .options
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .options
                                        .as_mut_ptr()
                                        .offset((*matcher).rmlvo.options.len() as isize)
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched && (*mval).sval.len > 0 as usize {
                                        xkb_logf!(
                                            (*matcher).ctx,
                                            XKB_LOG_LEVEL_ERROR,
                                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                                            "[XKB-{:03}] Unrecognized RMLVO option \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                            crate::xkb::utils::CStrNDisplay((*mval).sval.len as usize, (*mval).sval.start),
                                        );
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*out).symbols.is_null() && !explicit_layouts.is_null() {
                                *explicit_layouts = 1 as u32;
                                let mut symbols: *const i8 = (*out).symbols;
                                loop {
                                    symbols = crate::xkb::utils::cstr_chr(symbols, ':' as i32);
                                    if !(!symbols.is_null()
                                        && *symbols.offset(1 as isize) as i32 != '\0' as i32)
                                    {
                                        break;
                                    }
                                    let mut group: u32 = 0 as u32;
                                    symbols = symbols.offset(1);
                                    let (val_parsed, count) = crate::xkb::utils::parse_dec_u32(
                                        std::ffi::CStr::from_ptr(symbols).to_bytes(),
                                    );
                                    group = val_parsed;
                                    let count: i32 = count;
                                    if count > 0 as i32
                                        && (*symbols.offset(count as isize) as i32 == '\0' as i32
                                            || (*symbols.offset(count as isize) as i32
                                                == MERGE_OVERRIDE_PREFIX
                                                || *symbols.offset(count as isize) as i32
                                                    == MERGE_AUGMENT_PREFIX
                                                || *symbols.offset(count as isize) as i32
                                                    == MERGE_REPLACE_PREFIX))
                                        && group > 0 as u32
                                        && group <= XKB_MAX_GROUPS as u32
                                    {
                                        *explicit_layouts = if *explicit_layouts > group {
                                            *explicit_layouts
                                        } else {
                                            group
                                        };
                                        symbols = symbols.offset(count as isize);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !file.is_null() {
            fclose(file);
        }
        return ret;
    }
}
pub unsafe fn xkb_components_from_rmlvo_builder(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut u32,
) -> bool {
    unsafe {
        let mut rules: *const i8 = (*rmlvo).rules;
        let mut matcher: *mut matcher = matcher_new_from_rmlvo(rmlvo, &raw mut rules);
        if matcher.is_null() {
            return false;
        }
        let ret: bool = xkb_resolve_rules(
            std::ptr::addr_of!((*rmlvo).ctx) as *mut xkb_context,
            rules,
            matcher,
            out,
            explicit_layouts,
        ) as bool;
        matcher_free(matcher);
        return ret;
    }
}
pub unsafe fn xkb_components_from_rules_names(
    mut ctx: *mut xkb_context,
    mut rmlvo: *const xkb_rule_names,
    mut out: *mut xkb_component_names,
    mut explicit_layouts: *mut u32,
) -> bool {
    unsafe {
        let rmlvo_ref = &*rmlvo;
        let mut matcher: *mut matcher = matcher_new_from_names(ctx, rmlvo);
        if matcher.is_null() {
            return false;
        }
        let ret: bool = xkb_resolve_rules(
            ctx,
            if rmlvo_ref.rules.as_bytes().is_empty() {
                std::ptr::null()
            } else {
                rmlvo_ref.rules.as_ptr()
            },
            matcher,
            out,
            explicit_layouts,
        ) as bool;
        matcher_free(matcher);
        return ret;
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        rules_kccgst_svals = [
            sval {
                len: (std::mem::size_of::<[i8; 9]>()).wrapping_sub(1 as usize),
                start: b"keycodes\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
                start: b"types\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
                start: b"compat\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize),
                start: b"symbols\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 9]>()).wrapping_sub(1 as usize),
                start: b"geometry\0".as_ptr() as *const i8,
            },
        ];
        rules_mlvo_svals = [
            sval {
                len: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
                start: b"model\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
                start: b"layout\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1 as usize),
                start: b"variant\0".as_ptr() as *const i8,
            },
            sval {
                len: (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1 as usize),
                start: b"option\0".as_ptr() as *const i8,
            },
        ];
    }
}
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
