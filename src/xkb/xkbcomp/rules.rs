pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;
pub use crate::xkb::utils::is_absolute_path;

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
    XKB_WARNING_CANNOT_INFER_KEY_TYPE, XKB_WARNING_CONFLICTING_KEY_ACTION,
    XKB_WARNING_CONFLICTING_KEY_FIELDS, XKB_WARNING_CONFLICTING_KEY_NAME,
    XKB_WARNING_CONFLICTING_KEY_SYMBOL, XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES, XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
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
pub use crate::xkb::scanner_utils::{scanner, scanner_loc, sval, svaleq, svaleq_prefix};
pub use crate::xkb::shared_ast_types::{
    FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP,
    FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, _FILE_TYPE_NUM_ENTRIES,
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
use crate::xkb::utils::{cstr_as_bytes, cstr_len};
pub use crate::xkb::xkbcomp::include::{
    expand_path, FindFileInXkbPath, MERGE_AUGMENT_PREFIX, MERGE_OVERRIDE_PREFIX,
    MERGE_REPLACE_PREFIX,
};
use libc::{fclose, fopen, FILE};

/// Appends `count` bytes from `src` to the Vec.
#[inline]
fn vec_append_nul_terminated(v: &mut Vec<i8>, src: *const i8, count: u32) {
    v.extend_from_slice(unsafe { std::slice::from_raw_parts(src, count as usize) });
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
    pub layout: LayoutIdxUnion,
    pub active_or_candidates_mask: u32,
    pub kccgst_at_pos: [rules_kccgst; 5],
    pub num_kccgst: kccgst_index_t,
    pub defined_kccgst_mask: kccgst_mask_t,
}
pub type kccgst_mask_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LayoutIdxUnion {
    pub single: LayoutIdxSingle,
    pub range: LayoutIdxRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayoutIdxRange {
    pub layout_idx_min: u32,
    pub layout_idx_max: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayoutIdxSingle {
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
#[derive(Copy, Clone, Default)]
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
pub struct LayoutIndexName {
    pub name: *const i8,
    pub length: i32,
    pub range: layout_index_ranges,
}
pub type layout_index_ranges = u32;
pub type wildcard_match_type = u32;
pub const WILDCARD_MATCH_ALL: wildcard_match_type = 1;
pub const WILDCARD_MATCH_NONEMPTY: wildcard_match_type = 0;
pub const MAX_INCLUDE_DEPTH: i32 = 5_i32;
#[inline]
fn is_space(ch: i8) -> bool {
    matches!(ch as u8, b' ' | b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r')
}
#[inline]
fn is_ident(ch: i8) -> bool {
    (ch as u8).is_ascii_graphic() && ch as i32 != '\\' as i32
}
fn lex(s: *mut scanner, val: *mut lvalue) -> rules_token {
    unsafe {
        loop {
            while (*s).chr(' ' as i32 as i8) as i32 != 0
                || (*s).chr('\t' as i32 as i8) as i32 != 0
                || (*s).chr('\r' as i32 as i8) as i32 != 0
            {}
            if (*s).str_match(
                b"//\0".as_ptr() as *const i8,
                (std::mem::size_of::<[i8; 3]>()).wrapping_sub(1_usize),
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
                let loc: scanner_loc = (*s).token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: illegal new line escape; must appear at end of line\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &(*s).file_name,
                    loc.line,
                    loc.column
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
            (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1_usize),
        ) {
            return TOK_WILD_CARD_NONE;
        }
        if (*s).str_match(
            b"<some>\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1_usize),
        ) {
            return TOK_WILD_CARD_SOME;
        }
        if (*s).str_match(
            b"<any>\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1_usize),
        ) {
            return TOK_WILD_CARD_ANY;
        }
        if (*s).chr('$' as i32 as i8) {
            (*val).string.start = (*s).s.add((*s).pos);
            (*val).string.len = 0_usize;
            while is_ident((*s).peek()) {
                (*s).next_byte();
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            if (*val).string.len == 0_usize {
                let loc_0: scanner_loc = (*s).token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: unexpected character after '$'; expected name\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &(*s).file_name,
                    loc_0.line,
                    loc_0.column
                );
                return TOK_ERROR;
            }
            return TOK_GROUP_NAME;
        }
        if (*s).str_match(
            b"include\0".as_ptr() as *const i8,
            (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1_usize),
        ) {
            return TOK_INCLUDE;
        }
        if is_ident((*s).peek()) {
            (*val).string.start = (*s).s.add((*s).pos);
            (*val).string.len = 0_usize;
            while is_ident((*s).peek()) {
                (*s).next_byte();
                (*val).string.len = (*val).string.len.wrapping_add(1);
            }
            return TOK_IDENTIFIER;
        }
        let loc_1: scanner_loc = (*s).token_location();
        log::error!(
            "[XKB-{:03}] {}:{}:{}: unrecognized token\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &(*s).file_name,
            loc_1.line,
            loc_1.column
        );
        TOK_ERROR
    }
}
// SAFETY: SyncSval is a wrapper to allow sval (which contains *const i8) in a
// static. The wrapped values point to byte-string literals with 'static lifetime
// and are never mutated, so sharing across threads is safe.
struct SyncSval(sval);
unsafe impl Sync for SyncSval {}

static RULES_MLVO_SVALS: [SyncSval; 4] = [
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1),
        start: b"model\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1),
        start: b"layout\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1),
        start: b"variant\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1),
        start: b"option\0".as_ptr() as *const i8,
    }),
];
static RULES_KCCGST_SVALS: [SyncSval; 5] = [
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 9]>()).wrapping_sub(1),
        start: b"keycodes\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1),
        start: b"types\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 7]>()).wrapping_sub(1),
        start: b"compat\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 8]>()).wrapping_sub(1),
        start: b"symbols\0".as_ptr() as *const i8,
    }),
    SyncSval(sval {
        len: (std::mem::size_of::<[i8; 9]>()).wrapping_sub(1),
        start: b"geometry\0".as_ptr() as *const i8,
    }),
];
pub const OPTIONS_MATCH_ALL_GROUPS: i32 = XKB_MAX_GROUPS;
fn strip_spaces(mut v: sval) -> sval {
    unsafe {
        while v.len > 0_usize && is_space(*v.start.offset(0_i32 as isize)) as i32 != 0 {
            v.len = v.len.wrapping_sub(1);
            v.start = v.start.offset(1);
        }
        while v.len > 0_usize && is_space(*v.start.add(v.len.wrapping_sub(1_usize))) as i32 != 0 {
            v.len = v.len.wrapping_sub(1);
        }
        v
    }
}

/// Resize a Vec<matched_sval>, zero-filling new elements.
fn vec_resize_zero_matched_sval(v: &mut Vec<matched_sval>, new_len: usize) {
    if new_len > v.len() {
        v.resize(new_len, matched_sval::default());
    } else {
        v.truncate(new_len);
    }
}

fn split_comma_separated_mlvo(
    _ctx: *mut xkb_context,
    mlvo: rules_mlvo,
    mut s: *const i8,
) -> Vec<matched_sval> {
    unsafe {
        let mut arr: Vec<matched_sval> = Vec::new();
        if s.is_null() {
            let val: matched_sval = {
                let mut init = matched_sval {
                    matched: false,
                    layout: 0,
                    sval: sval {
                        len: 0_usize,
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
                        len: 0_usize,
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
                #[allow(unused_assignments)]
                let mut layout: u32 = XKB_LAYOUT_INVALID;
                let (val_parsed, count) =
                    crate::xkb::utils::parse_dec_u32(std::ffi::CStr::from_ptr(s).to_bytes());
                layout = val_parsed;
                let count: i32 = count;
                if count > 0_i32 {
                    s = s.offset(count as isize);
                    if layout == 0_u32 || layout > XKB_MAX_GROUPS as u32 {
                        log::error!(
                            "[XKB-{:03}] Invalid layout index {} for the RMVLO component: \"{}\"\n",
                            { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                            layout,
                            val_0.sval.as_str()
                        );
                    } else if mlvo != MLVO_OPTION {
                        log::warn!(
                            "Layout index {} is not supported for the RMLVO component: \"{}\"\n",
                            layout,
                            val_0.sval.as_str()
                        );
                    } else {
                        val_0.layout = layout.wrapping_sub(1_u32);
                    }
                }
                let layout_index_end: *const i8 = s;
                while *s as i32 != '\0' as i32 && *s as i32 != ',' as i32 {
                    s = s.offset(1);
                }
                if count <= 0_i32 || layout_index_end != s {
                    log::error!("[XKB-{:03}] Invalid layout index \"{}\" for the RMLVO component \"{}\"; discarding specifier.\n",
                        { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                        std::str::from_utf8_unchecked(std::slice::from_raw_parts(layout_start as *const u8, s.offset_from(layout_start) as usize)),
                        val_0.sval.as_str());
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
        arr
    }
}
fn matcher_new_from_names(ctx: *mut xkb_context, rmlvo: *const xkb_rule_names) -> *mut matcher {
    unsafe {
        // Allocate zeroed memory for matcher, then initialize Vec fields properly.
        let layout = std::alloc::Layout::new::<matcher>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut matcher;
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        let m: *mut matcher = ptr;
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
        (*m).rmlvo.model.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
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
                log::warn!(
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
                    }
                );
            }
            vec_resize_zero_matched_sval(&mut (*m).rmlvo.variants, (*m).rmlvo.layouts.len());
        } else if (*m).rmlvo.layouts.len() < (*m).rmlvo.variants.len() {
            log::error!(
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
                }
            );
            (*m).rmlvo.variants.truncate((*m).rmlvo.layouts.len());
        }
        m
    }
}
fn matcher_free(m: *mut matcher) {
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
fn matcher_group_start_new(m: *mut matcher, name: sval) {
    unsafe {
        let group: group = group {
            name,
            elements: Vec::new(),
        };
        (*m).groups.push(group);
    }
}
fn matcher_group_add_element(m: *mut matcher, _s: *mut scanner, element: sval) {
    unsafe {
        let last_group = (*m).groups.last_mut().unwrap();
        last_group.elements.push(element);
    }
}
fn matcher_include(m: *mut matcher, parent_scanner: *mut scanner, include_depth: u32, inc: sval) {
    unsafe {
        if include_depth >= MAX_INCLUDE_DEPTH as u32 {
            let loc: scanner_loc = (*parent_scanner).token_location();
            log::error!(
                "{}:{}:{}: maximum include depth ({}) exceeded; maybe there is an include loop?\n",
                &(*parent_scanner).file_name,
                loc.line,
                loc.column,
                MAX_INCLUDE_DEPTH
            );
            return;
        }
        let mut stmt_file: *const i8 = inc.start;
        let mut stmt_file_len: usize = inc.len;
        let mut buf: [i8; 4096] = [0; 4096];
        let expanded: isize = expand_path(
            (*m).ctx,
            &(*parent_scanner).file_name,
            stmt_file,
            stmt_file_len,
            FILE_TYPE_RULES,
            &raw mut buf as *mut i8,
            std::mem::size_of::<[i8; 4096]>(),
        ) as isize;
        if expanded < 0_isize {
            return;
        } else if expanded > 0_isize {
            stmt_file = &raw mut buf as *mut i8;
            stmt_file_len = expanded as usize;
        }
        let mut file: *mut FILE;
        let mut offset: u32 = 0_u32;
        let absolute_path: bool = is_absolute_path(stmt_file);
        if absolute_path {
            if expanded == 0 {
                if stmt_file_len < std::mem::size_of::<[i8; 4096]>() {
                    std::ptr::copy_nonoverlapping(
                        stmt_file as *const u8,
                        &raw mut buf as *mut i8 as *mut u8,
                        stmt_file_len,
                    );
                    buf[stmt_file_len] = '\0' as i32 as i8;
                    stmt_file = &raw mut buf as *mut i8;
                } else {
                    log::error!(
                        "[XKB-{:03}] Path is too long: {} > {}, got raw path: {}\n",
                        XKB_ERROR_INVALID_PATH as i32,
                        stmt_file_len,
                        std::mem::size_of::<[i8; 4096]>(),
                        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                            stmt_file as *const u8,
                            stmt_file_len
                        ))
                    );
                    return;
                }
            }
            file = fopen(stmt_file, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        } else if (expanded != 0) as i64 != 0 {
            file = std::ptr::null_mut();
        } else {
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name.as_str(),
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
            let ret: bool = read_rules_file(
                (*m).ctx,
                m,
                include_depth.wrapping_add(1_u32),
                file,
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                    &raw mut buf as *const i8,
                )),
            );
            fclose(file);
            if ret {
                return;
            }
            log::error!(
                "No components returned from included XKB rules \"{}\"\n",
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                    &raw mut buf as *const i8
                ))
            );
            if absolute_path {
                break;
            }
            offset = offset.wrapping_add(1);
            file = FindFileInXkbPath(
                (*m).ctx,
                (*parent_scanner).file_name.as_str(),
                stmt_file,
                stmt_file_len,
                FILE_TYPE_RULES,
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
                &raw mut offset,
                true,
            );
        }
        log::error!(
            "Failed to open included XKB rules \"{}\"\n",
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                stmt_file as *const u8,
                stmt_file_len
            ))
        );
    }
}
fn matcher_mapping_start_new(m: *mut matcher) {
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
        (*m).mapping.layout.single.variant_idx = XKB_LAYOUT_INVALID;
        (*m).mapping.layout.single.layout_idx = (*m).mapping.layout.single.variant_idx;
        (*m).mapping.num_kccgst = 0 as kccgst_index_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_kccgst as mlvo_index_t;
        (*m).mapping.defined_mlvo_mask = 0 as mlvo_mask_t;
        (*m).mapping.defined_kccgst_mask = 0 as kccgst_mask_t;
        (*m).mapping.active_or_candidates_mask = 1_u32;
    }
}
fn parse_layout_int_index(s: *const i8, max_len: usize, out: *mut u32) -> i32 {
    unsafe {
        let slice = std::slice::from_raw_parts(s.offset(1) as *const u8, max_len.wrapping_sub(2));
        let (val, count) = crate::xkb::utils::parse_dec_u32(slice);
        let count: i32 = count;
        if count <= 0_i32
            || *s.offset((1_i32 + count) as isize) as i32 != ']' as i32
            || val == 0_u32
            || val > XKB_MAX_GROUPS as u32
        {
            return -1_i32;
        }
        *out = val.wrapping_sub(1_u32);
        count + 2_i32
    }
}
fn extract_layout_index(s: *const i8, max_len: usize, out: *mut u32) -> i32 {
    unsafe {
        *out = XKB_LAYOUT_INVALID;
        if max_len < 3_usize || *s.offset(0_i32 as isize) as i32 != '[' as i32 {
            return -1_i32;
        }
        if max_len > 3_usize
            && *s.offset(1_i32 as isize) as i32 == '%' as i32
            && *s.offset(2_i32 as isize) as i32 == 'i' as i32
            && *s.offset(3_i32 as isize) as i32 == ']' as i32
        {
            return 4_i32;
        }
        parse_layout_int_index(s, max_len, out)
    }
}
fn extract_mapping_layout_index(s: *const i8, max_len: usize, out: *mut u32) -> i32 {
    unsafe {
        // SAFETY: SyncLayoutIndexName wraps LayoutIndexName (which contains *const i8)
        // to satisfy Sync requirement for static. The pointers are to static byte string literals.
        struct SyncLayoutIndexName(LayoutIndexName);
        unsafe impl Sync for SyncLayoutIndexName {}
        static NAMES: [SyncLayoutIndexName; 4] = [
            SyncLayoutIndexName(LayoutIndexName {
                name: "single]\0".as_ptr() as *const i8,
                length: 7_i32,
                range: LAYOUT_INDEX_SINGLE,
            }),
            SyncLayoutIndexName(LayoutIndexName {
                name: "first]\0".as_ptr() as *const i8,
                length: 6_i32,
                range: LAYOUT_INDEX_FIRST,
            }),
            SyncLayoutIndexName(LayoutIndexName {
                name: "later]\0".as_ptr() as *const i8,
                length: 6_i32,
                range: LAYOUT_INDEX_LATER,
            }),
            SyncLayoutIndexName(LayoutIndexName {
                name: "any]\0".as_ptr() as *const i8,
                length: 4_i32,
                range: LAYOUT_INDEX_ANY,
            }),
        ];
        if max_len < 3_usize || *s.offset(0_i32 as isize) as i32 != '[' as i32 {
            *out = XKB_LAYOUT_INVALID;
            return -1_i32;
        }
        let mut k: u32 = 0_u32;
        while (k as usize)
            < (std::mem::size_of::<[LayoutIndexName; 4]>())
                .wrapping_div(std::mem::size_of::<LayoutIndexName>())
        {
            if cstr_as_bytes(s.offset(1_i32 as isize) as *const i8).starts_with(
                &cstr_as_bytes(NAMES[k as usize].0.name)[..NAMES[k as usize].0.length as usize],
            ) {
                *out = NAMES[k as usize].0.range;
                return NAMES[k as usize].0.length + 1_i32;
            }
            k = k.wrapping_add(1);
        }
        *out = XKB_LAYOUT_INVALID;
        parse_layout_int_index(s, max_len, out)
    }
}
#[inline]
fn is_mlvo_mask_defined(m: *mut matcher, mlvo: rules_mlvo) -> bool {
    unsafe { (*m).mapping.defined_mlvo_mask as u32 & 1_u32 << mlvo != 0 }
}
fn matcher_mapping_set_mlvo(m: *mut matcher, s: *mut scanner, ident: sval) {
    unsafe {
        let mut mlvo: rules_mlvo;
        let mut mlvo_sval: sval = sval {
            len: 0,
            start: std::ptr::null(),
        };
        mlvo = MLVO_MODEL;
        while (mlvo as u32) < _MLVO_NUM_ENTRIES {
            mlvo_sval = RULES_MLVO_SVALS[mlvo as usize].0;
            if svaleq_prefix(mlvo_sval, ident) {
                break;
            }
            mlvo += 1;
        }
        if mlvo as u32 >= _MLVO_NUM_ENTRIES {
            let loc: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc.line,
                loc.column,
                ident.as_str());
            (*m).mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        if is_mlvo_mask_defined(m, mlvo) {
            let loc_0: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc_0.line,
                loc_0.column,
                mlvo_sval.as_str());
            (*m).mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        if mlvo_sval.len < ident.len {
            let mut idx: u32 = 0;
            let consumed: i32 = extract_mapping_layout_index(
                ident.start.add(mlvo_sval.len),
                ident.len.wrapping_sub(mlvo_sval.len),
                &raw mut idx,
            );
            if ident.len.wrapping_sub(mlvo_sval.len) as i32 != consumed {
                let loc_1: scanner_loc = (*s).token_location();
                log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" may only be followed by a valid group index; ignoring rule set\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &(*s).file_name,
                    loc_1.line,
                    loc_1.column,
                    mlvo_sval.as_str());
                (*m).mapping.active_or_candidates_mask = 0_u32;
                return;
            }
            if mlvo as u32 == MLVO_LAYOUT {
                (*m).mapping.layout.single.layout_idx = idx;
            } else if mlvo as u32 == MLVO_VARIANT {
                (*m).mapping.layout.single.variant_idx = idx;
            } else {
                let loc_2: scanner_loc = (*s).token_location();
                log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" cannot be followed by a group index; ignoring rule set\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &(*s).file_name,
                    loc_2.line,
                    loc_2.column,
                    mlvo_sval.as_str());
                (*m).mapping.active_or_candidates_mask = 0_u32;
                return;
            }
        } else if mlvo as u32 == MLVO_LAYOUT {
            (*m).mapping.layout.single.layout_idx = LAYOUT_INDEX_SINGLE;
        } else if mlvo as u32 == MLVO_VARIANT {
            (*m).mapping.layout.single.variant_idx = LAYOUT_INDEX_SINGLE;
        }
        if (mlvo as u32 == MLVO_LAYOUT && is_mlvo_mask_defined(m, MLVO_VARIANT) as i32 != 0
            || mlvo as u32 == MLVO_VARIANT && is_mlvo_mask_defined(m, MLVO_LAYOUT) as i32 != 0)
            && (*m).mapping.layout.single.layout_idx != (*m).mapping.layout.single.variant_idx
        {
            let loc_3: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"layout\" index must be the same as the \"variant\" index\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc_3.line,
                loc_3.column);
            (*m).mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        (*m).mapping.mlvo_at_pos[(*m).mapping.num_mlvo as usize] = mlvo;
        (*m).mapping.defined_mlvo_mask = ((*m).mapping.defined_mlvo_mask as i32
            | (1_u32 as mlvo_mask_t as i32) << mlvo as u32)
            as mlvo_mask_t;
        (*m).mapping.num_mlvo = (*m).mapping.num_mlvo.wrapping_add(1);
    }
}
fn matcher_mapping_set_layout_bounds(m: *mut matcher) {
    unsafe {
        let mut idx: u32 =
            if (*m).mapping.layout.single.layout_idx < (*m).mapping.layout.single.variant_idx {
                (*m).mapping.layout.single.layout_idx
            } else {
                (*m).mapping.layout.single.variant_idx
            };
        let c2rust_current_block_17: u64;
        match idx {
            XKB_LAYOUT_INVALID => {
                (*m).mapping.has_layout_idx_range = false;
                (*m).mapping.layout.range.layout_idx_min = XKB_LAYOUT_INVALID;
                (*m).mapping.layout.range.layout_idx_max = XKB_LAYOUT_INVALID;
                (*m).mapping.active_or_candidates_mask = 0x1_u32;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967293 => {
                (*m).mapping.has_layout_idx_range = true;
                (*m).mapping.layout.range.layout_idx_min = 1_u32;
                (*m).mapping.layout.range.layout_idx_max = (if (32) < (*m).rmlvo.layouts.len() {
                    32
                } else {
                    (*m).rmlvo.layouts.len()
                }) as u32;
                (*m).mapping.active_or_candidates_mask =
                    ((1_u64 << (*m).mapping.layout.range.layout_idx_max).wrapping_sub(1_u64) as u32
                        as u64
                        & !1_u64) as u32;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967294 => {
                (*m).mapping.has_layout_idx_range = true;
                (*m).mapping.layout.range.layout_idx_min = 0_u32;
                (*m).mapping.layout.range.layout_idx_max = (if (32) < (*m).rmlvo.layouts.len() {
                    32
                } else {
                    (*m).rmlvo.layouts.len()
                }) as u32;
                (*m).mapping.active_or_candidates_mask =
                    ((1_u64 << (*m).mapping.layout.range.layout_idx_max) as u32 as u64)
                        .wrapping_sub(1_u64) as u32;
                c2rust_current_block_17 = 13056961889198038528;
            }
            4294967291 | 4294967292 => {
                idx = 0_u32;
                c2rust_current_block_17 = 9641388756612255828;
            }
            _ => {
                c2rust_current_block_17 = 9641388756612255828;
            }
        }
        if c2rust_current_block_17 == 9641388756612255828 {
            (*m).mapping.has_layout_idx_range = false;
            (*m).mapping.layout.range.layout_idx_min = idx;
            (*m).mapping.layout.range.layout_idx_max = idx.wrapping_add(1_u32);
            (*m).mapping.active_or_candidates_mask = 1_u32 << idx;
        };
    }
}
fn matcher_mapping_set_kccgst(m: *mut matcher, s: *mut scanner, ident: sval) {
    unsafe {
        let mut kccgst: rules_kccgst;
        let mut kccgst_sval: sval = sval {
            len: 0,
            start: std::ptr::null(),
        };
        kccgst = KCCGST_KEYCODES;
        while (kccgst as u32) < _KCCGST_NUM_ENTRIES {
            kccgst_sval = RULES_KCCGST_SVALS[kccgst as usize].0;
            if svaleq(RULES_KCCGST_SVALS[kccgst as usize].0, ident) {
                break;
            }
            kccgst += 1;
        }
        if kccgst as u32 >= _KCCGST_NUM_ENTRIES {
            let loc: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc.line,
                loc.column,
                ident.as_str());
            (*m).mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        if (*m).mapping.defined_kccgst_mask as u32 & 1_u32 << kccgst as u32 != 0 {
            let loc_0: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc_0.line,
                loc_0.column,
                kccgst_sval.as_str());
            (*m).mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        (*m).mapping.kccgst_at_pos[(*m).mapping.num_kccgst as usize] = kccgst;
        (*m).mapping.defined_kccgst_mask = ((*m).mapping.defined_kccgst_mask as i32
            | (1_u32 as kccgst_mask_t as i32) << kccgst as u32)
            as kccgst_mask_t;
        (*m).mapping.num_kccgst = (*m).mapping.num_kccgst.wrapping_add(1);
    }
}
fn matcher_mapping_verify(m: *mut matcher, s: *mut scanner) -> bool {
    unsafe {
        let mut c2rust_current_block: u64;
        if (*m).mapping.num_mlvo as i32 == 0_i32 {
            let loc: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the left hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc.line,
                loc.column);
        } else if (*m).mapping.num_kccgst as i32 == 0_i32 {
            let loc_0: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the right hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc_0.line,
                loc_0.column);
        } else {
            if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
                match (*m).mapping.layout.single.layout_idx {
                    4294967291 => {
                        c2rust_current_block = 4840043166261277618;
                        match c2rust_current_block {
                            14825033830842003582 => {
                                if (*m).rmlvo.layouts.len() < 2
                                    || (*m).mapping.layout.single.layout_idx
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
                    4294967292..=4294967294 => {
                        c2rust_current_block = 8831408221741692167;
                    }
                    _ => {
                        c2rust_current_block = 14825033830842003582;
                        match c2rust_current_block {
                            14825033830842003582 => {
                                if (*m).rmlvo.layouts.len() < 2
                                    || (*m).mapping.layout.single.layout_idx
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
                        match (*m).mapping.layout.single.variant_idx {
                            4294967291 => {
                                c2rust_current_block = 13345507216710712890;
                                match c2rust_current_block {
                                    10338831042980687939 => {
                                        if (*m).rmlvo.variants.len() < 2
                                            || (*m).mapping.layout.single.variant_idx
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
                            4294967292..=4294967294 => {
                                c2rust_current_block = 10652014663920648156;
                            }
                            _ => {
                                c2rust_current_block = 10338831042980687939;
                                match c2rust_current_block {
                                    10338831042980687939 => {
                                        if (*m).rmlvo.variants.len() < 2
                                            || (*m).mapping.layout.single.variant_idx
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
        (*m).mapping.active_or_candidates_mask = 0_u32;
        false
    }
}
fn matcher_rule_start_new(m: *mut matcher) {
    unsafe {
        std::ptr::write_bytes::<rule>(&raw mut (*m).rule as *mut rule, 0u8, 1);
        (*m).rule.skip = (*m).mapping.active_or_candidates_mask == 0;
    }
}
fn matcher_rule_set_mlvo_common(
    m: *mut matcher,
    s: *mut scanner,
    ident: sval,
    match_type: mlvo_match_type,
) {
    unsafe {
        if (*m).rule.num_mlvo_values as i32 >= (*m).mapping.num_mlvo as i32 {
            let loc: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc.line,
                loc.column);
            (*m).rule.skip = true;
            return;
        }
        (*m).rule.match_type_at_pos[(*m).rule.num_mlvo_values as usize] = match_type;
        (*m).rule.mlvo_value_at_pos[(*m).rule.num_mlvo_values as usize] = ident;
        (*m).rule.num_mlvo_values = (*m).rule.num_mlvo_values.wrapping_add(1);
    }
}
fn matcher_rule_set_mlvo_wildcard(m: *mut matcher, s: *mut scanner, match_type: mlvo_match_type) {
    unsafe {
        let dummy: sval = sval {
            len: 0_usize,
            start: std::ptr::null(),
        };
        matcher_rule_set_mlvo_common(m, s, dummy, match_type);
    }
}
fn matcher_rule_set_mlvo_group(m: *mut matcher, s: *mut scanner, ident: sval) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
    }
}
fn matcher_rule_set_mlvo(m: *mut matcher, s: *mut scanner, ident: sval) {
    unsafe {
        matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
    }
}
fn matcher_rule_set_kccgst(m: *mut matcher, s: *mut scanner, ident: sval) {
    unsafe {
        if (*m).rule.num_kccgst_values as i32 >= (*m).mapping.num_kccgst as i32 {
            let loc: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc.line,
                loc.column);
            (*m).rule.skip = true;
            return;
        }
        (*m).rule.kccgst_value_at_pos[(*m).rule.num_kccgst_values as usize] = ident;
        (*m).rule.num_kccgst_values = (*m).rule.num_kccgst_values.wrapping_add(1);
    }
}
fn match_group(m: *mut matcher, group_name: sval, to: sval) -> bool {
    unsafe {
        let found_group = (*m).groups.iter().find(|g| svaleq(g.name, group_name));
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
fn match_value(
    m: *mut matcher,
    val: sval,
    to: sval,
    match_type: mlvo_match_type,
    wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        match match_type {
            1 => wildcard_type == WILDCARD_MATCH_ALL || to.len != 0,
            2 => to.len == 0,
            3 => to.len != 0,
            4 => true,
            5 => match_group(m, val, to),
            _ => svaleq(val, to),
        }
    }
}
fn match_value_and_mark(
    m: *mut matcher,
    val: sval,
    to: *mut matched_sval,
    match_type: mlvo_match_type,
    wildcard_type: wildcard_match_type,
) -> bool {
    unsafe {
        let matched: bool = match_value(m, val, (*to).sval, match_type, wildcard_type);
        if matched {
            (*to).matched = true;
        }
        matched
    }
}
fn expand_rmlvo_in_kccgst_value(
    m: *mut matcher,
    s: *mut scanner,
    value: sval,
    layout_idx: u32,
    expanded: *mut Vec<i8>,
    i: *mut usize,
) -> bool {
    unsafe {
        let mut expanded_index: bool;
        let mut c2rust_current_block: u64;
        let str: *const i8 = value.start;
        let mlv: rules_mlvo;
        let mut idx: u32;
        let mut pfx: i8;
        let mut sfx: i8;
        let mut expanded_value: *mut matched_sval;
        if *str.add(*i) as i32 == 'i' as i32
            && ((*i).wrapping_add(1_usize) == value.len
                || (*str.add((*i).wrapping_add(1_usize)) as i32 == MERGE_OVERRIDE_PREFIX
                    || *str.add((*i).wrapping_add(1_usize)) as i32 == MERGE_AUGMENT_PREFIX
                    || *str.add((*i).wrapping_add(1_usize)) as i32 == MERGE_REPLACE_PREFIX))
        {
            if layout_idx == XKB_LAYOUT_INVALID {
                let loc: scanner_loc = (*s).token_location();
                log::error!("[XKB-{:03}] {}:{}:{}: Invalid %i in %-expansion: there is no corresponding layout nor variant in the MLVO fields of the rules header.\n",
                    XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION
                        as i32,
                    &(*s).file_name,
                    loc.line,
                    loc.column);
            } else {
                *i = (*i).wrapping_add(1);
                let mut index_str: [i8; 12] = [0; 12];
                let count: i32 = crate::xkb::utils::snprintf_c(
                    &raw mut index_str as *mut i8,
                    std::mem::size_of::<[i8; 12]>(),
                    format_args!("{}", layout_idx.wrapping_add(1_u32)),
                );
                (*expanded).extend_from_slice(std::slice::from_raw_parts(
                    &raw mut index_str as *mut i8 as *const i8,
                    count as usize,
                ));
                return true;
            }
        } else {
            sfx = 0_i8;
            pfx = sfx;
            if *str.add(*i) as i32 == '(' as i32
                || (*str.add(*i) as i32 == MERGE_OVERRIDE_PREFIX
                    || *str.add(*i) as i32 == MERGE_AUGMENT_PREFIX
                    || *str.add(*i) as i32 == MERGE_REPLACE_PREFIX)
                || *str.add(*i) as i32 == '_' as i32
                || *str.add(*i) as i32 == '-' as i32
            {
                pfx = *str.add(*i);
                if *str.add(*i) as i32 == '(' as i32 {
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
                    match *str.add(c2rust_fresh7) as i32 {
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
                            idx = XKB_LAYOUT_INVALID;
                            expanded_index = false;
                            if *i < value.len && *str.add(*i) as i32 == '[' as i32 {
                                if mlv as u32 != MLVO_LAYOUT && mlv as u32 != MLVO_VARIANT {
                                    let loc_0: scanner_loc = (*s).token_location();
                                    log::error!("[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                                        &(*s).file_name,
                                        loc_0.line,
                                        loc_0.column);
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let consumed: i32 = extract_layout_index(
                                        str.add(*i),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1_i32 {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID {
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
                                    if sfx as i32 != 0_i32 {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.add(c2rust_fresh8) as i32 != sfx as i32 {
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
                                            if mlv as u32 == MLVO_LAYOUT {
                                                if idx == XKB_LAYOUT_INVALID {
                                                    if (*m).rmlvo.layouts.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .layouts
                                                            .as_mut_ptr()
                                                            .offset(0_i32 as isize)
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
                                            } else if mlv as u32 == MLVO_VARIANT {
                                                if idx == XKB_LAYOUT_INVALID {
                                                    if (*m).rmlvo.variants.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .variants
                                                            .as_mut_ptr()
                                                            .offset(0_i32 as isize)
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
                                            } else if mlv as u32 == MLVO_MODEL {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0_usize
                                            {
                                                return true;
                                            }
                                            if pfx as i32 != 0_i32 {
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
                                            if sfx as i32 != 0_i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                            }
                                            (*expanded_value).matched = true;
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
                            idx = XKB_LAYOUT_INVALID;
                            expanded_index = false;
                            if *i < value.len && *str.add(*i) as i32 == '[' as i32 {
                                if mlv as u32 != MLVO_LAYOUT && mlv as u32 != MLVO_VARIANT {
                                    let loc_0: scanner_loc = (*s).token_location();
                                    log::error!("[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                                        &(*s).file_name,
                                        loc_0.line,
                                        loc_0.column);
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let consumed: i32 = extract_layout_index(
                                        str.add(*i),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1_i32 {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID {
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
                                    if sfx as i32 != 0_i32 {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.add(c2rust_fresh8) as i32 != sfx as i32 {
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
                                            if mlv as u32 == MLVO_LAYOUT {
                                                if idx == XKB_LAYOUT_INVALID {
                                                    if (*m).rmlvo.layouts.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .layouts
                                                            .as_mut_ptr()
                                                            .offset(0_i32 as isize)
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
                                            } else if mlv as u32 == MLVO_VARIANT {
                                                if idx == XKB_LAYOUT_INVALID {
                                                    if (*m).rmlvo.variants.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .variants
                                                            .as_mut_ptr()
                                                            .offset(0_i32 as isize)
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
                                            } else if mlv as u32 == MLVO_MODEL {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0_usize
                                            {
                                                return true;
                                            }
                                            if pfx as i32 != 0_i32 {
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
                                            if sfx as i32 != 0_i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                            }
                                            (*expanded_value).matched = true;
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
                            idx = XKB_LAYOUT_INVALID;
                            expanded_index = false;
                            if *i < value.len && *str.add(*i) as i32 == '[' as i32 {
                                if mlv as u32 != MLVO_LAYOUT && mlv as u32 != MLVO_VARIANT {
                                    let loc_0: scanner_loc = (*s).token_location();
                                    log::error!("[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                                        &(*s).file_name,
                                        loc_0.line,
                                        loc_0.column);
                                    c2rust_current_block = 14165246690716487359;
                                } else {
                                    let consumed: i32 = extract_layout_index(
                                        str.add(*i),
                                        value.len.wrapping_sub(*i),
                                        &raw mut idx,
                                    );
                                    if consumed == -1_i32 {
                                        c2rust_current_block = 14165246690716487359;
                                    } else {
                                        if idx == XKB_LAYOUT_INVALID {
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
                                    if sfx as i32 != 0_i32 {
                                        if *i >= value.len {
                                            c2rust_current_block = 14165246690716487359;
                                        } else {
                                            let c2rust_fresh8 = *i;
                                            *i = (*i).wrapping_add(1);
                                            if *str.add(c2rust_fresh8) as i32 != sfx as i32 {
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
                                            if mlv as u32 == MLVO_LAYOUT {
                                                if idx == XKB_LAYOUT_INVALID {
                                                    if (*m).rmlvo.layouts.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .layouts
                                                            .as_mut_ptr()
                                                            .offset(0_i32 as isize)
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
                                            } else if mlv as u32 == MLVO_VARIANT {
                                                if idx == XKB_LAYOUT_INVALID {
                                                    if (*m).rmlvo.variants.len() == 1 {
                                                        expanded_value = (*m)
                                                            .rmlvo
                                                            .variants
                                                            .as_mut_ptr()
                                                            .offset(0_i32 as isize)
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
                                            } else if mlv as u32 == MLVO_MODEL {
                                                expanded_value = &raw mut (*m).rmlvo.model;
                                            }
                                            if expanded_value.is_null()
                                                || (*expanded_value).sval.len == 0_usize
                                            {
                                                return true;
                                            }
                                            if pfx as i32 != 0_i32 {
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
                                            if sfx as i32 != 0_i32 {
                                                vec_append_nul_terminated(
                                                    &mut *expanded,
                                                    &raw const sfx as *const i8,
                                                    1,
                                                );
                                            }
                                            (*expanded_value).matched = true;
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
        let loc_1: scanner_loc = (*s).token_location();
        log::error!(
            "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &(*s).file_name,
            loc_1.line,
            loc_1.column
        );
        false
    }
}
fn expand_qualifier_in_kccgst_value(
    m: *mut matcher,
    s: *mut scanner,
    value: sval,
    expanded: *mut Vec<i8>,
    has_layout_idx_range: bool,
    has_separator: bool,
    prefix_idx: u32,
    i: *mut usize,
) {
    unsafe {
        let str: *const i8 = value.start;
        if ((*i).wrapping_add(3_usize) <= value.len
            || (*str.add((*i).wrapping_add(3_usize)) as i32 == MERGE_OVERRIDE_PREFIX
                || *str.add((*i).wrapping_add(3_usize)) as i32 == MERGE_AUGMENT_PREFIX
                || *str.add((*i).wrapping_add(3_usize)) as i32 == MERGE_REPLACE_PREFIX))
            && *str.add(*i) as i32 == 'a' as i32
            && *str.add((*i).wrapping_add(1_usize)) as i32 == 'l' as i32
            && *str.add((*i).wrapping_add(2_usize)) as i32 == 'l' as i32
        {
            if has_layout_idx_range {
                let loc: scanner_loc = (*s).token_location();
                log::warn!(
                    "{}:{}:{}: Using :all qualifier with indices range is not recommended.\n",
                    &(*s).file_name,
                    loc.line,
                    loc.column
                );
            }
            vec_append_nul_terminated(&mut *expanded, b"1\0".as_ptr() as *const i8, 1);
            if (*m).rmlvo.layouts.len() > 1 {
                let mut layout_index: [i8; 12] = [0; 12];
                let prefix_length = (*expanded)
                    .len()
                    .wrapping_sub(prefix_idx as usize)
                    .wrapping_sub(1);
                let mut l: u32 = 1_u32;
                while l
                    < (if 32 < (*m).rmlvo.layouts.len() {
                        32_u32
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
                            (*expanded).as_mut_ptr().add(old_size),
                            prefix_length,
                        );
                        *(*expanded).as_mut_ptr().add(new_size.wrapping_sub(1)) = 0;
                        (*expanded).truncate(new_size.wrapping_sub(1));
                    }
                    let count: i32 = crate::xkb::utils::snprintf_c(
                        &raw mut layout_index as *mut i8,
                        std::mem::size_of::<[i8; 12]>(),
                        format_args!("{}", l.wrapping_add(1_u32)),
                    );
                    vec_append_nul_terminated(
                        &mut *expanded,
                        &raw mut layout_index as *mut i8 as *const i8,
                        count as u32,
                    );
                    l = l.wrapping_add(1);
                }
            }
            *i = (*i).wrapping_add(3_usize);
        }
    }
}
#[inline]
fn concat_kccgst(into: *mut Vec<i8>, size: u32, from: *const i8) {
    unsafe {
        let from_plus: bool = *from.offset(0_i32 as isize) as i32 == MERGE_OVERRIDE_PREFIX
            || *from.offset(0_i32 as isize) as i32 == MERGE_AUGMENT_PREFIX
            || *from.offset(0_i32 as isize) as i32 == MERGE_REPLACE_PREFIX;
        if from_plus as i32 != 0 || (*into).is_empty() {
            vec_append_nul_terminated(&mut *into, from, size);
        } else {
            let ch: i8 = (if (*into).is_empty() {
                '\0' as i32
            } else {
                *(*into).as_ptr().offset(0_i32 as isize) as i32
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
fn append_expanded_kccgst_value(
    m: *mut matcher,
    s: *mut scanner,
    merge: bool,
    to: *mut Vec<i8>,
    value: sval,
    layout_idx: u32,
) -> bool {
    unsafe {
        let c2rust_current_block: u64;
        let str: *const i8 = value.start;
        let mut expanded: Vec<i8> = Vec::new();
        let mut last_item_idx: u32 = 0;
        let mut has_separator: bool = false;
        let mut i: usize = 0_usize;
        loop {
            if i >= value.len {
                c2rust_current_block = 10758786907990354186;
                break;
            }
            match *str.add(i) as i32 {
                58 => {
                    let c2rust_fresh4 = i;
                    i = i.wrapping_add(1);
                    vec_append_nul_terminated(&mut expanded, str.add(c2rust_fresh4), 1);
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
                    vec_append_nul_terminated(&mut expanded, str.add(c2rust_fresh5), 1);
                    last_item_idx = expanded.len().wrapping_sub(1) as u32;
                    has_separator = true;
                }
                _ => {
                    let c2rust_fresh6 = i;
                    i = i.wrapping_add(1);
                    vec_append_nul_terminated(&mut expanded, str.add(c2rust_fresh6), 1);
                }
            }
        }
        match c2rust_current_block {
            1032266188497003083 => {
                drop(expanded);
                false
            }
            _ => {
                if merge {
                    if !(expanded.is_empty()) {
                        concat_kccgst(to, expanded.len() as u32, expanded.as_ptr());
                    }
                } else if !expanded.is_empty() {
                    (*to).extend_from_slice(&expanded);
                }
                drop(expanded);
                true
            }
        }
    }
}
fn matcher_append_pending_kccgst(m: *mut matcher) -> bool {
    unsafe {
        if !(*m).mapping.has_layout_idx_range {
            return true;
        }
        let mut i: kccgst_index_t = 0 as kccgst_index_t;
        while (i as i32) < (*m).mapping.num_kccgst as i32 {
            let kccgst: rules_kccgst = (*m).mapping.kccgst_at_pos[i as usize];
            let mut layout: u32 = (*m).mapping.layout.range.layout_idx_min;
            while layout < (*m).mapping.layout.range.layout_idx_max {
                let buf: *const kccgst_buffer = &raw mut (*m).pending_kccgst;
                let mut offset: usize = 0_usize;
                let mut k: u32 = 0;
                while k < (*buf).slices.len() as u32 {
                    let slice: &kccgst_buffer_slice = &(&(*buf).slices)[k as usize];
                    if slice.kccgst == kccgst as u32
                        && slice.layout == layout
                        && slice.length as i32 != 0
                    {
                        concat_kccgst(
                            &mut (*m).kccgst[kccgst as usize],
                            slice.length,
                            (*buf).buffer.as_ptr().add(offset),
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
        true
    }
}
fn matcher_rule_verify(m: *mut matcher, s: *mut scanner) {
    unsafe {
        if (*m).rule.num_mlvo_values as i32 != (*m).mapping.num_mlvo as i32
            || (*m).rule.num_kccgst_values as i32 != (*m).mapping.num_kccgst as i32
        {
            let loc: scanner_loc = (*s).token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: must have same number of values as mapping line; ignoring rule\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &(*s).file_name,
                loc.line,
                loc.column);
            (*m).rule.skip = true;
        }
    }
}
fn matcher_rule_apply_if_matches(m: *mut matcher, s: *mut scanner) {
    unsafe {
        let mut candidate_layouts: u32 = (*m).mapping.active_or_candidates_mask;
        let mut idx: u32;
        let mut i: mlvo_index_t = 0 as mlvo_index_t;
        while (i as i32) < (*m).mapping.num_mlvo as i32 {
            let mlvo: rules_mlvo = (*m).mapping.mlvo_at_pos[i as usize];
            let value: sval = (*m).rule.mlvo_value_at_pos[i as usize];
            let match_type: mlvo_match_type = (*m).rule.match_type_at_pos[i as usize];
            let mut to: *mut matched_sval;
            let mut matched: bool = false;
            if mlvo as u32 == MLVO_MODEL {
                to = &raw mut (*m).rmlvo.model;
                matched = match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_ALL);
            } else if (*m).mapping.has_layout_idx_range {
                idx = (*m).mapping.layout.range.layout_idx_min;
                while idx < (*m).mapping.layout.range.layout_idx_max && candidate_layouts != 0 {
                    let mask: u32 = 1_u32 << idx;
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
                                    to = (*m).rmlvo.options.as_mut_ptr().offset(0_i32 as isize)
                                        as *mut matched_sval;
                                    while to
                                        < (*m)
                                            .rmlvo
                                            .options
                                            .as_mut_ptr()
                                            .add((*m).rmlvo.options.len())
                                            as *mut matched_sval
                                    {
                                        if !((*to).layout as i32 != OPTIONS_MATCH_ALL_GROUPS
                                            && (*to).layout != idx)
                                            && match_value_and_mark(
                                                m,
                                                value,
                                                to,
                                                match_type,
                                                WILDCARD_MATCH_ALL,
                                            )
                                        {
                                            matched = true;
                                            found_option = true;
                                            break;
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
                        to = (*m)
                            .rmlvo
                            .layouts
                            .as_mut_ptr()
                            .offset((*m).mapping.layout.range.layout_idx_min as isize)
                            as *mut matched_sval;
                        matched =
                            match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_NONEMPTY);
                    }
                    2 => {
                        to = (*m)
                            .rmlvo
                            .variants
                            .as_mut_ptr()
                            .offset((*m).mapping.layout.range.layout_idx_min as isize)
                            as *mut matched_sval;
                        matched =
                            match_value_and_mark(m, value, to, match_type, WILDCARD_MATCH_NONEMPTY);
                    }
                    _ => {
                        if !(*m).rmlvo.options.is_empty() {
                            to = (*m).rmlvo.options.as_mut_ptr().offset(0_i32 as isize)
                                as *mut matched_sval;
                            while to
                                < (*m)
                                    .rmlvo
                                    .options
                                    .as_mut_ptr()
                                    .add((*m).rmlvo.options.len())
                                    as *mut matched_sval
                            {
                                if !((*to).layout as i32 != OPTIONS_MATCH_ALL_GROUPS
                                    && (*to).layout != (*m).mapping.layout.range.layout_idx_min)
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
            idx = (*m).mapping.layout.range.layout_idx_min;
            while idx < (*m).mapping.layout.range.layout_idx_max {
                if candidate_layouts & 1_u32 << idx != 0 {
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
                            ((*buf).buffer.len() as u32).wrapping_sub(prev_buffer_length);
                        let slice = kccgst_buffer_slice {
                            length,
                            kccgst,
                            layout: idx,
                        };
                        (*buf).slices.push(slice);
                        i_0 = i_0.wrapping_add(1);
                    }
                }
                idx = idx.wrapping_add(1);
            }
        } else {
            let mut i_1: kccgst_index_t = 0 as kccgst_index_t;
            while (i_1 as i32) < (*m).mapping.num_kccgst as i32 {
                let kccgst_0: rules_kccgst = (*m).mapping.kccgst_at_pos[i_1 as usize];
                let value_1: sval = (*m).rule.kccgst_value_at_pos[i_1 as usize];
                append_expanded_kccgst_value(
                    m,
                    s,
                    true,
                    &mut (*m).kccgst[kccgst_0 as usize],
                    value_1,
                    (*m).mapping.layout.range.layout_idx_min,
                );
                i_1 = i_1.wrapping_add(1);
            }
        }
        if !is_mlvo_mask_defined(m, MLVO_OPTION) {
            (*m).mapping.active_or_candidates_mask &= !candidate_layouts;
        }
    }
}
fn gettok(m: *mut matcher, s: *mut scanner) -> rules_token {
    unsafe { lex(s, &raw mut (*m).val) }
}
fn matcher_match(
    m: *mut matcher,
    s: *mut scanner,
    include_depth: u32,
    _string: *const i8,
    _len: usize,
    _file_name: &str,
) -> bool {
    unsafe {
        let c2rust_current_block: u64;
        let mut tok: rules_token;
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
                            if (*m).mapping.active_or_candidates_mask != 0 {
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
                            if (*m).mapping.active_or_candidates_mask != 0 {
                                matcher_mapping_set_kccgst(m, s, (*m).val.string);
                            }
                        }
                        if (*m).mapping.active_or_candidates_mask != 0
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
                                                    if (*m).val.string.len == 1_usize
                                                        && *(*m)
                                                            .val
                                                            .string
                                                            .start
                                                            .offset(0_i32 as isize)
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
            13194772801876125100 => true,
            _ => {
                match tok as u32 {
                    11 => {}
                    _ => {
                        let loc: scanner_loc = (*s).token_location();
                        log::error!(
                            "[XKB-{:03}] {}:{}:{}: unexpected token\n",
                            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                            &(*s).file_name,
                            loc.line,
                            loc.column
                        );
                    }
                }
                false
            }
        }
    }
}
fn read_rules_file(
    _ctx: *mut xkb_context,
    matcher: *mut matcher,
    include_depth: u32,
    file: *mut FILE,
    path: &str,
) -> bool {
    unsafe {
        #[allow(unused_assignments)]
        let mut scanner: scanner = scanner::new(
            std::ptr::null_mut(),
            std::ptr::null(),
            0,
            "",
            std::ptr::null_mut(),
        );

        // Convert FILE* to Rust File and map it
        use crate::xkb::utils::MappedFile;
        use std::fs::File;
        use std::os::unix::io::FromRawFd;

        let fd = libc::fileno(file as *mut libc::FILE);
        if fd < 0 {
            log::error!("Invalid file descriptor\n");
            return false;
        }

        let rust_file = File::from_raw_fd(fd);
        let mapped = match MappedFile::new(&rust_file) {
            Ok(m) => m,
            Err(e) => {
                log::error!("Couldn't read rules file \"{}\": {}\n", path, e);
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
            let loc: scanner_loc = scanner.token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: This could be a file encoding issue. Supported encodings must be backward compatible with ASCII.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &scanner.file_name,
                loc.line,
                loc.column);
            let loc_0: scanner_loc = scanner.token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: E.g. ISO/CEI 8859 and UTF-8 are supported but UTF-16, UTF-32 and CP1026 are not.\n",
                XKB_ERROR_INVALID_FILE_ENCODING as i32,
                &scanner.file_name,
                loc_0.line,
                loc_0.column);
            std::mem::forget(rust_file);
            return false;
        }
        let ret: bool = matcher_match(
            matcher,
            &raw mut scanner,
            include_depth,
            mapped.as_ptr(),
            mapped.len(),
            path,
        );
        std::mem::forget(rust_file);
        ret
    }
}
fn xkb_resolve_partial_rules(
    ctx: *mut xkb_context,
    path: *mut i8,
    path_size: usize,
    rules: *const i8,
    suffix: *const i8,
    matcher: *mut matcher,
) -> bool {
    unsafe {
        let mut partial_rules: [i8; 60] = [0; 60];
        let (_, _trunc) = crate::xkb::utils::snprintf_args(
            &raw mut partial_rules as *mut i8,
            std::mem::size_of::<[i8; 60]>(),
            format_args!(
                "{}{}",
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(rules)),
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(suffix))
            ),
        );
        if _trunc {
            log::error!(
                "[XKB-{:03}] Cannot load XKB rules \"{}{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(rules)),
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(suffix))
            );
            return false;
        }
        let mut offset: u32 = 0_u32;
        let mut file: *mut FILE;
        let len: usize = cstr_len(&raw mut partial_rules as *mut i8);
        loop {
            file = FindFileInXkbPath(
                ctx,
                "(unknown)",
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
            let ok: bool = read_rules_file(
                ctx,
                matcher,
                0_u32,
                file,
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(path)),
            );
            fclose(file);
            if !ok {
                log::error!(
                    "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                    std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(path))
                );
                return false;
            }
            offset = offset.wrapping_add(1);
        }
        true
    }
}
fn xkb_resolve_rules(
    ctx: *mut xkb_context,
    rules: *const i8,
    matcher: *mut matcher,
    out: *mut xkb_component_names,
    explicit_layouts: *mut u32,
) -> bool {
    unsafe {
        let mut mval: *mut matched_sval;
        let mut ret: bool = false;
        let mut offset: u32 = 0_u32;
        let mut path: [i8; 4096] = [0; 4096];
        let file: *mut FILE = FindFileInXkbPath(
            ctx,
            "(unknown)",
            rules,
            cstr_len(rules),
            FILE_TYPE_RULES,
            &raw mut path as *mut i8,
            std::mem::size_of::<[i8; 4096]>(),
            &raw mut offset,
            true,
        ) as *mut FILE;
        if file.is_null() {
            log::error!(
                "[XKB-{:03}] Cannot load XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(rules))
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
                ret = read_rules_file(
                    ctx,
                    matcher,
                    0_u32,
                    file,
                    std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                        &raw mut path as *const i8,
                    )),
                );
                if !ret {
                    log::error!(
                        "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                        XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                        std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                            &raw mut path as *mut i8
                        ))
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
                        if (*matcher).kccgst[KCCGST_KEYCODES as usize].is_empty()
                            || (*matcher).kccgst[KCCGST_TYPES as usize].is_empty()
                            || (*matcher).kccgst[KCCGST_COMPAT as usize].is_empty()
                            || (*matcher).kccgst[KCCGST_SYMBOLS as usize].is_empty()
                        {
                            log::error!(
                                "[XKB-{:03}] No components returned from XKB rules \"{}\"\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                std::str::from_utf8_unchecked(crate::xkb::utils::cstr_as_bytes(
                                    rules
                                ))
                            );
                            ret = false;
                        } else {
                            // Transfer ownership of Vec data directly.
                            // Each Vec is taken (replaced with empty Vec) and nul-terminated.
                            {
                                let mut v = std::mem::take(
                                    &mut (*matcher).kccgst[KCCGST_KEYCODES as usize],
                                );
                                v.push(0);
                                (*out).keycodes = v;
                            }
                            {
                                let mut v =
                                    std::mem::take(&mut (*matcher).kccgst[KCCGST_TYPES as usize]);
                                v.push(0);
                                (*out).types = v;
                            }
                            {
                                let mut v =
                                    std::mem::take(&mut (*matcher).kccgst[KCCGST_COMPAT as usize]);
                                v.push(0);
                                (*out).compatibility = v;
                            }
                            {
                                let mut v =
                                    std::mem::take(&mut (*matcher).kccgst[KCCGST_SYMBOLS as usize]);
                                v.push(0);
                                (*out).symbols = v;
                            }
                            {
                                let mut v = std::mem::take(
                                    &mut (*matcher).kccgst[KCCGST_GEOMETRY as usize],
                                );
                                v.push(0);
                                (*out).geometry = v;
                            }
                            mval = &raw mut (*matcher).rmlvo.model;
                            if !(*mval).matched && (*mval).sval.len > 0_usize {
                                log::error!(
                                    "[XKB-{:03}] Unrecognized RMLVO model \"{}\" was ignored\n",
                                    XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                    (*mval).sval.as_str()
                                );
                            }
                            if !(*matcher).rmlvo.layouts.is_empty() {
                                mval = (*matcher).rmlvo.layouts.as_mut_ptr().offset(0_i32 as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .layouts
                                        .as_mut_ptr()
                                        .add((*matcher).rmlvo.layouts.len())
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched && (*mval).sval.len > 0_usize {
                                        log::error!("[XKB-{:03}] Unrecognized RMLVO layout \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                            (*mval).sval.as_str());
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*matcher).rmlvo.variants.is_empty() {
                                mval = (*matcher)
                                    .rmlvo
                                    .variants
                                    .as_mut_ptr()
                                    .offset(0_i32 as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .variants
                                        .as_mut_ptr()
                                        .add((*matcher).rmlvo.variants.len())
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched && (*mval).sval.len > 0_usize {
                                        log::error!("[XKB-{:03}] Unrecognized RMLVO variant \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                            (*mval).sval.as_str());
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*matcher).rmlvo.options.is_empty() {
                                mval = (*matcher).rmlvo.options.as_mut_ptr().offset(0_i32 as isize)
                                    as *mut matched_sval;
                                while mval
                                    < (*matcher)
                                        .rmlvo
                                        .options
                                        .as_mut_ptr()
                                        .add((*matcher).rmlvo.options.len())
                                        as *mut matched_sval
                                {
                                    if !(*mval).matched && (*mval).sval.len > 0_usize {
                                        log::error!("[XKB-{:03}] Unrecognized RMLVO option \"{}\" was ignored\n",
                                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                            (*mval).sval.as_str());
                                    }
                                    mval = mval.offset(1);
                                }
                            }
                            if !(*out).symbols.is_empty() && !explicit_layouts.is_null() {
                                *explicit_layouts = 1_u32;
                                let mut symbols: *const i8 = (*out).symbols.as_ptr();
                                loop {
                                    symbols = crate::xkb::utils::cstr_chr(symbols, ':' as i32);
                                    if !(!symbols.is_null()
                                        && *symbols.offset(1_isize) as i32 != '\0' as i32)
                                    {
                                        break;
                                    }

                                    symbols = symbols.offset(1);
                                    let (val_parsed, count) = crate::xkb::utils::parse_dec_u32(
                                        std::ffi::CStr::from_ptr(symbols).to_bytes(),
                                    );
                                    let group: u32 = val_parsed;
                                    let count: i32 = count;
                                    if count > 0_i32
                                        && (*symbols.offset(count as isize) as i32 == '\0' as i32
                                            || (*symbols.offset(count as isize) as i32
                                                == MERGE_OVERRIDE_PREFIX
                                                || *symbols.offset(count as isize) as i32
                                                    == MERGE_AUGMENT_PREFIX
                                                || *symbols.offset(count as isize) as i32
                                                    == MERGE_REPLACE_PREFIX))
                                        && group > 0_u32
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
        ret
    }
}
pub fn xkb_components_from_rules_names(
    ctx: *mut xkb_context,
    rmlvo: *const xkb_rule_names,
    out: *mut xkb_component_names,
    explicit_layouts: *mut u32,
) -> bool {
    unsafe {
        let rmlvo_ref = &*rmlvo;
        let matcher: *mut matcher = matcher_new_from_names(ctx, rmlvo);
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
        ret
    }
}
use crate::xkb::shared_types::*;
