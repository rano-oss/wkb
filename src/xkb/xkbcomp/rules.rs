pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;

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
use crate::xkb::utils::parse_dec_u32;
pub use crate::xkb::xkbcomp::include::{
    expand_path_str, FindFileInXkbPath, MERGE_AUGMENT_PREFIX, MERGE_OVERRIDE_PREFIX,
    MERGE_REPLACE_PREFIX,
};

/// Appends bytes from `src` to the Vec<i8>.
#[inline]
fn vec_append_nul_terminated(v: &mut Vec<i8>, src: &[u8]) {
    v.extend(src.iter().map(|&b| b as i8));
}

/// Index-based sval for scanner input. Used in lvalue/rule to avoid
/// lifetime issues across include boundaries. Reconstruct sval via to_sval().
#[derive(Copy, Clone, Default)]
struct SvalIdx {
    start: usize,
    end: usize,
}
impl SvalIdx {
    const EMPTY: SvalIdx = SvalIdx { start: 0, end: 0 };
    #[inline]
    fn to_sval<'a>(&self, input: &'a [u8]) -> sval<'a> {
        if self.start >= self.end || self.start >= input.len() {
            sval::EMPTY
        } else {
            sval {
                data: &input[self.start..self.end.min(input.len())],
            }
        }
    }
    #[inline]
    fn len(&self) -> usize {
        self.end - self.start
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

pub struct matcher<'a> {
    pub ctx: &'a mut xkb_context,
    pub rmlvo: rule_names<'a>,
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
pub struct rule {
    pub mlvo_value_at_pos: [SvalIdx; 4],
    pub match_type_at_pos: [mlvo_match_type; 4],
    pub kccgst_value_at_pos: [SvalIdx; 5],
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
pub struct mapping {
    pub mlvo_at_pos: [rules_mlvo; 4],
    pub num_mlvo: mlvo_index_t,
    pub defined_mlvo_mask: mlvo_mask_t,
    pub layout: LayoutIdx,
    pub active_or_candidates_mask: u32,
    pub kccgst_at_pos: [rules_kccgst; 5],
    pub num_kccgst: kccgst_index_t,
    pub defined_kccgst_mask: u8,
}
#[derive(Copy, Clone)]
pub enum LayoutIdx {
    Single {
        layout_idx: u32,
        variant_idx: u32,
    },
    Range {
        layout_idx_min: u32,
        layout_idx_max: u32,
    },
    Index {
        layout_idx_min: u32,
        layout_idx_max: u32,
    },
}
impl Default for LayoutIdx {
    fn default() -> Self {
        LayoutIdx::Single {
            layout_idx: 0,
            variant_idx: 0,
        }
    }
}
impl LayoutIdx {
    fn layout_idx_min(&self) -> u32 {
        match self {
            LayoutIdx::Range { layout_idx_min, .. } | LayoutIdx::Index { layout_idx_min, .. } => {
                *layout_idx_min
            }
            _ => panic!("expected Range or Index"),
        }
    }
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
    pub name: Vec<u8>,
    pub elements: Vec<Vec<u8>>,
}
#[derive(Copy, Clone)]
pub struct lvalue {
    pub string: SvalIdx,
}
#[derive(Clone)]
pub struct rule_names<'a> {
    pub model: matched_sval<'a>,
    pub layouts: Vec<matched_sval<'a>>,
    pub variants: Vec<matched_sval<'a>>,
    pub options: Vec<matched_sval<'a>>,
}
#[derive(Copy, Clone, Default)]
pub struct matched_sval<'a> {
    pub sval: sval<'a>,
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

pub struct LayoutIndexName {
    pub name: *const i8,
    pub length: i32,
    pub range: layout_index_ranges,
}
pub type layout_index_ranges = u32;

impl Default for mapping {
    fn default() -> Self {
        mapping {
            mlvo_at_pos: [0; 4],
            num_mlvo: 0,
            defined_mlvo_mask: 0,
            layout: LayoutIdx::default(),
            active_or_candidates_mask: 0,
            kccgst_at_pos: [0; 5],
            num_kccgst: 0,
            defined_kccgst_mask: 0,
        }
    }
}
impl Default for rule {
    fn default() -> Self {
        rule {
            mlvo_value_at_pos: [SvalIdx::EMPTY; 4],
            match_type_at_pos: [0; 4],
            kccgst_value_at_pos: [SvalIdx::EMPTY; 5],
            num_mlvo_values: 0,
            num_kccgst_values: 0,
            skip: false,
        }
    }
}
impl Default for lvalue {
    fn default() -> Self {
        lvalue {
            string: SvalIdx::EMPTY,
        }
    }
}
impl Default for kccgst_buffer {
    fn default() -> Self {
        kccgst_buffer {
            buffer: Vec::new(),
            slices: Vec::new(),
        }
    }
}
impl Default for rule_names<'_> {
    fn default() -> Self {
        rule_names {
            model: matched_sval::default(),
            layouts: Vec::new(),
            variants: Vec::new(),
            options: Vec::new(),
        }
    }
}
impl<'a> matcher<'a> {
    fn new(ctx: &'a mut xkb_context) -> Self {
        matcher {
            ctx,
            rmlvo: rule_names::default(),
            val: lvalue::default(),
            groups: Vec::new(),
            mapping: mapping::default(),
            rule: rule::default(),
            pending_kccgst: kccgst_buffer::default(),
            kccgst: std::array::from_fn(|_| Vec::new()),
        }
    }
}
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
fn lex(s: &mut scanner, val: &mut lvalue) -> rules_token {
    loop {
        while s.chr(' ' as i32 as i8) as i32 != 0
            || s.chr('\t' as i32 as i8) as i32 != 0
            || s.chr('\r' as i32 as i8) as i32 != 0
        {}
        if s.str_match(b"//") {
            s.skip_to_eol();
        }
        if s.eol() {
            while s.eol() {
                s.next_byte();
            }
            return TOK_END_OF_LINE;
        }
        if !s.chr('\\' as i32 as i8) {
            break;
        }
        s.chr('\r' as i32 as i8);
        if !s.eol() {
            let loc: scanner_loc = s.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: illegal new line escape; must appear at end of line\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc.line,
                loc.column
            );
            return TOK_ERROR;
        }
        s.next_byte();
    }
    if s.eof() {
        return TOK_END_OF_FILE;
    }
    s.token_pos = s.pos;
    if s.chr('!' as i32 as i8) {
        return TOK_BANG;
    }
    if s.chr('=' as i32 as i8) {
        return TOK_EQUALS;
    }
    if s.chr('*' as i32 as i8) {
        return TOK_WILD_CARD_STAR;
    }
    if s.str_match(b"<none>") {
        return TOK_WILD_CARD_NONE;
    }
    if s.str_match(b"<some>") {
        return TOK_WILD_CARD_SOME;
    }
    if s.str_match(b"<any>") {
        return TOK_WILD_CARD_ANY;
    }
    if s.chr('$' as i32 as i8) {
        val.string = SvalIdx {
            start: s.pos,
            end: s.pos,
        };
        while is_ident(s.peek()) {
            s.next_byte();
            val.string.end += 1;
        }
        if val.string.len() == 0 {
            let loc_0: scanner_loc = s.token_location();
            log::error!(
                "[XKB-{:03}] {}:{}:{}: unexpected character after '$'; expected name\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_0.line,
                loc_0.column
            );
            return TOK_ERROR;
        }
        return TOK_GROUP_NAME;
    }
    if s.str_match(b"include") {
        return TOK_INCLUDE;
    }
    if is_ident(s.peek()) {
        val.string = SvalIdx {
            start: s.pos,
            end: s.pos,
        };
        while is_ident(s.peek()) {
            s.next_byte();
            val.string.end += 1;
        }
        return TOK_IDENTIFIER;
    }
    let loc_1: scanner_loc = s.token_location();
    log::error!(
        "[XKB-{:03}] {}:{}:{}: unrecognized token\n",
        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
        &s.file_name,
        loc_1.line,
        loc_1.column
    );
    TOK_ERROR
}
static RULES_MLVO_SVALS: [&[u8]; 4] = [b"model", b"layout", b"variant", b"option"];
static RULES_KCCGST_SVALS: [&[u8]; 5] = [b"keycodes", b"types", b"compat", b"symbols", b"geometry"];
pub const OPTIONS_MATCH_ALL_GROUPS: i32 = XKB_MAX_GROUPS;
fn strip_spaces<'a>(v: sval<'a>) -> sval<'a> {
    let bytes = v.data;
    let start_trim = bytes
        .iter()
        .position(|&b| !is_space(b as i8))
        .unwrap_or(bytes.len());
    let end_trim = bytes
        .iter()
        .rposition(|&b| !is_space(b as i8))
        .map(|i| i + 1)
        .unwrap_or(start_trim);
    if start_trim >= end_trim {
        sval::EMPTY
    } else {
        sval {
            data: &bytes[start_trim..end_trim],
        }
    }
}

/// Resize a Vec<matched_sval>, zero-filling new elements.
fn vec_resize_zero_matched_sval(v: &mut Vec<matched_sval<'_>>, new_len: usize) {
    if new_len > v.len() {
        v.resize(new_len, matched_sval::default());
    } else {
        v.truncate(new_len);
    }
}

fn split_comma_separated_mlvo<'a>(mlvo: rules_mlvo, s: Option<&'a [u8]>) -> Vec<matched_sval<'a>> {
    let mut arr: Vec<matched_sval<'a>> = Vec::new();
    let Some(bytes) = s else {
        arr.push(matched_sval::default());
        return arr;
    };
    if bytes.is_empty() {
        arr.push(matched_sval::default());
        return arr;
    }
    let mut pos: usize = 0;
    loop {
        let start = pos;
        let mut end = pos;
        let mut val_0 = matched_sval {
            matched: false,
            layout: OPTIONS_MATCH_ALL_GROUPS as u32,
            sval: sval {
                data: &bytes[start..start],
            },
        };
        while pos < bytes.len()
            && bytes[pos] != b','
            && bytes[pos] as i32 != OPTIONS_GROUP_SPECIFIER_PREFIX
        {
            pos += 1;
            end += 1;
        }
        val_0.sval = sval {
            data: &bytes[start..end],
        };
        val_0.sval = strip_spaces(val_0.sval);
        if pos < bytes.len() && bytes[pos] as i32 == OPTIONS_GROUP_SPECIFIER_PREFIX {
            pos += 1;
            let layout_start = pos;
            #[allow(unused_assignments)]
            let mut layout: u32 = XKB_LAYOUT_INVALID;
            let (val_parsed, count) = crate::xkb::utils::parse_dec_u32(&bytes[pos..]);
            layout = val_parsed;
            let count = count as usize;
            if count > 0 {
                pos += count;
                if layout == 0 || layout > XKB_MAX_GROUPS as u32 {
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
                    val_0.layout = layout.wrapping_sub(1);
                }
            }
            let layout_index_end = pos;
            while pos < bytes.len() && bytes[pos] != b',' {
                pos += 1;
            }
            if count == 0 || layout_index_end != pos {
                let layout_spec = std::str::from_utf8(&bytes[layout_start..pos]).unwrap_or("");
                log::error!("[XKB-{:03}] Invalid layout index \"{}\" for the RMLVO component \"{}\"; discarding specifier.\n",
                    { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                    layout_spec,
                    val_0.sval.as_str());
                val_0.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
            }
        }
        arr.push(val_0);
        if pos >= bytes.len() {
            break;
        }
        if bytes[pos] == b',' {
            pos += 1;
        }
    }
    arr
}
fn matcher_new_from_names<'a>(
    ctx: &'a mut xkb_context,
    rmlvo: &'a xkb_rule_names,
) -> Box<matcher<'a>> {
    let mut m = Box::new(matcher::new(ctx));
    let rmlvo_ref = rmlvo;
    m.rmlvo.model.sval = sval {
        data: rmlvo_ref.model.as_bytes(),
    };
    m.rmlvo.model.layout = OPTIONS_MATCH_ALL_GROUPS as u32;
    m.rmlvo.layouts = split_comma_separated_mlvo(
        MLVO_LAYOUT,
        if rmlvo_ref.layout.as_bytes().is_empty() {
            None
        } else {
            Some(rmlvo_ref.layout.as_bytes())
        },
    );
    m.rmlvo.variants = split_comma_separated_mlvo(
        MLVO_VARIANT,
        if rmlvo_ref.variant.as_bytes().is_empty() {
            None
        } else {
            Some(rmlvo_ref.variant.as_bytes())
        },
    );
    m.rmlvo.options = split_comma_separated_mlvo(
        MLVO_OPTION,
        if rmlvo_ref.options.as_bytes().is_empty() {
            None
        } else {
            Some(rmlvo_ref.options.as_bytes())
        },
    );
    if m.rmlvo.layouts.len() > m.rmlvo.variants.len() {
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
        vec_resize_zero_matched_sval(&mut m.rmlvo.variants, m.rmlvo.layouts.len());
    } else if m.rmlvo.layouts.len() < m.rmlvo.variants.len() {
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
        m.rmlvo.variants.truncate(m.rmlvo.layouts.len());
    }
    m
}
fn matcher_free(_m: Box<matcher>) {
    // Box drop handles deallocation
}
fn matcher_group_start_new(m: &mut matcher, name: &[u8]) {
    let group: group = group {
        name: name.to_vec(),
        elements: Vec::new(),
    };
    m.groups.push(group);
}
fn matcher_group_add_element(m: &mut matcher, _s: &mut scanner, element: &[u8]) {
    let last_group = m.groups.last_mut().unwrap();
    last_group.elements.push(element.to_vec());
}
fn matcher_include(
    m: &mut matcher<'_>,
    parent_scanner: &mut scanner,
    include_depth: u32,
    inc: sval,
) {
    if include_depth >= MAX_INCLUDE_DEPTH as u32 {
        let loc: scanner_loc = parent_scanner.token_location();
        log::error!(
            "{}:{}:{}: maximum include depth ({}) exceeded; maybe there is an include loop?\n",
            &parent_scanner.file_name,
            loc.line,
            loc.column,
            MAX_INCLUDE_DEPTH
        );
        return;
    }
    let inc_str = inc.as_str();
    let stmt_file: String =
        match expand_path_str(&parent_scanner.file_name, inc_str, FILE_TYPE_RULES) {
            Err(()) => return,
            Ok(Some(expanded)) => expanded,
            Ok(None) => inc_str.to_string(),
        };
    let expanded = stmt_file != inc_str;

    let absolute_path = stmt_file.starts_with('/');
    let mut offset: u32 = 0;
    let mut file_and_path: Option<(std::fs::File, String)> = if absolute_path {
        std::fs::File::open(&stmt_file)
            .ok()
            .map(|f| (f, stmt_file.clone()))
    } else if expanded {
        None
    } else {
        FindFileInXkbPath(
            &mut *m.ctx,
            &parent_scanner.file_name,
            &stmt_file,
            FILE_TYPE_RULES,
            &mut offset,
            true,
        )
    };

    while let Some((ref open_file, ref path)) = file_and_path {
        let ret: bool = read_rules_file(m, include_depth.wrapping_add(1_u32), open_file, path);
        let path_str = path.clone();
        let _ = file_and_path.take();
        if ret {
            return;
        }
        log::error!(
            "No components returned from included XKB rules \"{}\"\n",
            &path_str
        );
        if absolute_path {
            break;
        }
        offset += 1;
        file_and_path = FindFileInXkbPath(
            &mut *m.ctx,
            &parent_scanner.file_name,
            &stmt_file,
            FILE_TYPE_RULES,
            &mut offset,
            true,
        );
    }
    log::error!("Failed to open included XKB rules \"{}\"\n", &stmt_file);
}
fn matcher_mapping_start_new(m: &mut matcher) {
    let mut i: mlvo_index_t = 0 as mlvo_index_t;
    while (i as i32) < _MLVO_NUM_ENTRIES as i32 as mlvo_index_t as i32 {
        m.mapping.mlvo_at_pos[i as usize] = _MLVO_NUM_ENTRIES;
        i = i.wrapping_add(1);
    }
    let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
    while (i_0 as i32) < _KCCGST_NUM_ENTRIES as i32 as kccgst_index_t as i32 {
        m.mapping.kccgst_at_pos[i_0 as usize] = _KCCGST_NUM_ENTRIES;
        i_0 = i_0.wrapping_add(1);
    }
    m.mapping.layout = LayoutIdx::Single {
        layout_idx: XKB_LAYOUT_INVALID,
        variant_idx: XKB_LAYOUT_INVALID,
    };
    m.mapping.num_kccgst = 0 as kccgst_index_t;
    m.mapping.num_mlvo = m.mapping.num_kccgst as mlvo_index_t;
    m.mapping.defined_mlvo_mask = 0 as mlvo_mask_t;
    m.mapping.defined_kccgst_mask = 0 as u8;
    m.mapping.active_or_candidates_mask = 1_u32;
}
fn parse_layout_int_index(s: &[u8], out: &mut u32) -> i32 {
    // s starts with '[', parse integer between brackets
    if s.len() < 3 {
        return -1_i32;
    }
    let inner = &s[1..]; // skip '['
    let (val, count) = crate::xkb::utils::parse_dec_u32(inner);
    let count: i32 = count;
    if count <= 0_i32
        || (1 + count as usize) >= s.len()
        || s[1 + count as usize] != b']'
        || val == 0_u32
        || val > XKB_MAX_GROUPS as u32
    {
        return -1_i32;
    }
    *out = val.wrapping_sub(1_u32);
    count + 2_i32
}
fn extract_layout_index(s: &[u8], out: &mut u32) -> i32 {
    *out = XKB_LAYOUT_INVALID;
    if s.len() < 3 || s[0] != b'[' {
        return -1_i32;
    }
    if s.len() > 3 && s[1] == b'%' && s[2] == b'i' && s[3] == b']' {
        return 4_i32;
    }
    parse_layout_int_index(s, out)
}
fn extract_mapping_layout_index(s: &[u8], out: &mut u32) -> i32 {
    struct LayoutIndexEntry {
        name: &'static [u8],
        range: layout_index_ranges,
    }
    static NAMES: [LayoutIndexEntry; 4] = [
        LayoutIndexEntry {
            name: b"single]",
            range: LAYOUT_INDEX_SINGLE,
        },
        LayoutIndexEntry {
            name: b"first]",
            range: LAYOUT_INDEX_FIRST,
        },
        LayoutIndexEntry {
            name: b"later]",
            range: LAYOUT_INDEX_LATER,
        },
        LayoutIndexEntry {
            name: b"any]",
            range: LAYOUT_INDEX_ANY,
        },
    ];
    if s.len() < 3 || s[0] != b'[' {
        *out = XKB_LAYOUT_INVALID;
        return -1_i32;
    }
    let after_bracket = &s[1..];
    for entry in &NAMES {
        if after_bracket.starts_with(entry.name) {
            *out = entry.range;
            return (entry.name.len() + 1) as i32;
        }
    }
    *out = XKB_LAYOUT_INVALID;
    parse_layout_int_index(s, out)
}
#[inline]
fn is_mlvo_mask_defined(m: &mut matcher, mlvo: rules_mlvo) -> bool {
    m.mapping.defined_mlvo_mask as u32 & 1_u32 << mlvo != 0
}
fn matcher_mapping_set_mlvo(m: &mut matcher, s: &mut scanner, ident: sval) {
    let ident_bytes = ident.as_bytes();
    let mut mlvo: rules_mlvo = MLVO_MODEL;
    let mut mlvo_bytes: &[u8] = b"";
    while (mlvo as u32) < _MLVO_NUM_ENTRIES {
        mlvo_bytes = RULES_MLVO_SVALS[mlvo as usize];
        if mlvo_bytes.len() <= ident_bytes.len() && &ident_bytes[..mlvo_bytes.len()] == mlvo_bytes {
            break;
        }
        mlvo += 1;
    }
    if mlvo as u32 >= _MLVO_NUM_ENTRIES {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column,
            ident.as_str());
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if is_mlvo_mask_defined(m, mlvo) {
        let loc_0: scanner_loc = s.token_location();
        let mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc_0.line,
            loc_0.column,
            mlvo_str);
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if mlvo_bytes.len() < ident_bytes.len() {
        let mut idx: u32 = 0;
        let remaining = &ident_bytes[mlvo_bytes.len()..];
        let consumed: i32 = extract_mapping_layout_index(remaining, &mut idx);
        if remaining.len() as i32 != consumed {
            let loc_1: scanner_loc = s.token_location();
            let mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" may only be followed by a valid group index; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_1.line,
                loc_1.column,
                mlvo_str);
            m.mapping.active_or_candidates_mask = 0_u32;
            return;
        }
        if mlvo as u32 == MLVO_LAYOUT {
            if let LayoutIdx::Single {
                ref mut layout_idx, ..
            } = m.mapping.layout
            {
                *layout_idx = idx;
            }
        } else if mlvo as u32 == MLVO_VARIANT {
            if let LayoutIdx::Single {
                ref mut variant_idx,
                ..
            } = m.mapping.layout
            {
                *variant_idx = idx;
            }
        } else {
            let loc_2: scanner_loc = s.token_location();
            let mlvo_str = std::str::from_utf8(mlvo_bytes).unwrap_or("");
            log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" cannot be followed by a group index; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_2.line,
                loc_2.column,
                mlvo_str);
            m.mapping.active_or_candidates_mask = 0_u32;
            return;
        }
    } else if mlvo as u32 == MLVO_LAYOUT {
        if let LayoutIdx::Single {
            ref mut layout_idx, ..
        } = m.mapping.layout
        {
            *layout_idx = LAYOUT_INDEX_SINGLE;
        }
    } else if mlvo as u32 == MLVO_VARIANT {
        if let LayoutIdx::Single {
            ref mut variant_idx,
            ..
        } = m.mapping.layout
        {
            *variant_idx = LAYOUT_INDEX_SINGLE;
        }
    }
    if (mlvo as u32 == MLVO_LAYOUT && is_mlvo_mask_defined(m, MLVO_VARIANT) as i32 != 0
        || mlvo as u32 == MLVO_VARIANT && is_mlvo_mask_defined(m, MLVO_LAYOUT) as i32 != 0)
        && {
            if let LayoutIdx::Single {
                layout_idx,
                variant_idx,
            } = m.mapping.layout
            {
                layout_idx != variant_idx
            } else {
                false
            }
        }
    {
        let loc_3: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"layout\" index must be the same as the \"variant\" index\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc_3.line,
            loc_3.column);
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.mlvo_at_pos[m.mapping.num_mlvo as usize] = mlvo;
    m.mapping.defined_mlvo_mask = (m.mapping.defined_mlvo_mask as i32
        | (1_u32 as mlvo_mask_t as i32) << mlvo as u32)
        as mlvo_mask_t;
    m.mapping.num_mlvo = m.mapping.num_mlvo.wrapping_add(1);
}
fn matcher_mapping_set_layout_bounds(m: &mut matcher) {
    let mut idx: u32 = if let LayoutIdx::Single {
        layout_idx,
        variant_idx,
    } = m.mapping.layout
    {
        if layout_idx < variant_idx {
            layout_idx
        } else {
            variant_idx
        }
    } else {
        0 // should not happen, layout is Single at this point
    };
    let c2rust_current_block_17: u64;
    match idx {
        XKB_LAYOUT_INVALID => {
            m.mapping.layout = LayoutIdx::Index {
                layout_idx_min: XKB_LAYOUT_INVALID,
                layout_idx_max: XKB_LAYOUT_INVALID,
            };
            m.mapping.active_or_candidates_mask = 0x1_u32;
            c2rust_current_block_17 = 13056961889198038528;
        }
        4294967293 => {
            let layout_idx_max = (if (32) < m.rmlvo.layouts.len() {
                32
            } else {
                m.rmlvo.layouts.len()
            }) as u32;
            m.mapping.layout = LayoutIdx::Range {
                layout_idx_min: 1_u32,
                layout_idx_max,
            };
            m.mapping.active_or_candidates_mask =
                ((1_u64 << layout_idx_max).wrapping_sub(1_u64) as u32 as u64 & !1_u64) as u32;
            c2rust_current_block_17 = 13056961889198038528;
        }
        4294967294 => {
            let layout_idx_max = (if (32) < m.rmlvo.layouts.len() {
                32
            } else {
                m.rmlvo.layouts.len()
            }) as u32;
            m.mapping.layout = LayoutIdx::Range {
                layout_idx_min: 0_u32,
                layout_idx_max,
            };
            m.mapping.active_or_candidates_mask =
                ((1_u64 << layout_idx_max) as u32 as u64).wrapping_sub(1_u64) as u32;
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
        m.mapping.layout = LayoutIdx::Index {
            layout_idx_min: idx,
            layout_idx_max: idx.wrapping_add(1_u32),
        };
        m.mapping.active_or_candidates_mask = 1_u32 << idx;
    };
}
fn matcher_mapping_set_kccgst(m: &mut matcher, s: &mut scanner, ident: sval) {
    let ident_bytes = ident.as_bytes();
    let mut kccgst: rules_kccgst = KCCGST_KEYCODES;
    let mut kccgst_bytes: &[u8] = b"";
    while (kccgst as u32) < _KCCGST_NUM_ENTRIES {
        kccgst_bytes = RULES_KCCGST_SVALS[kccgst as usize];
        if kccgst_bytes == ident_bytes {
            break;
        }
        kccgst += 1;
    }
    if kccgst as u32 >= _KCCGST_NUM_ENTRIES {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" is not a valid value here; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column,
            ident.as_str());
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if m.mapping.defined_kccgst_mask as u32 & 1_u32 << kccgst as u32 != 0 {
        let loc_0: scanner_loc = s.token_location();
        let kccgst_str = std::str::from_utf8(kccgst_bytes).unwrap_or("");
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: \"{}\" appears twice on the same line; ignoring rule set\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc_0.line,
            loc_0.column,
            kccgst_str);
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.kccgst_at_pos[m.mapping.num_kccgst as usize] = kccgst;
    m.mapping.defined_kccgst_mask =
        (m.mapping.defined_kccgst_mask as i32 | (1_u32 as u8 as i32) << kccgst as u32) as u8;
    m.mapping.num_kccgst = m.mapping.num_kccgst.wrapping_add(1);
}
fn matcher_mapping_verify(m: &mut matcher, s: &mut scanner) -> bool {
    let mut c2rust_current_block: u64;
    if m.mapping.num_mlvo as i32 == 0_i32 {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the left hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc.line,
                loc.column);
    } else if m.mapping.num_kccgst as i32 == 0_i32 {
        let loc_0: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid mapping: must have at least one value on the right hand side; ignoring rule set\n",
                XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                &s.file_name,
                loc_0.line,
                loc_0.column);
    } else {
        if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
            let single_layout_idx = if let LayoutIdx::Single { layout_idx, .. } = m.mapping.layout {
                layout_idx
            } else {
                0
            };
            match single_layout_idx {
                4294967291 => {
                    c2rust_current_block = 4840043166261277618;
                    match c2rust_current_block {
                        14825033830842003582 => {
                            if m.rmlvo.layouts.len() < 2
                                || single_layout_idx >= m.rmlvo.layouts.len() as u32
                            {
                                c2rust_current_block = 436805222042109220;
                            } else {
                                c2rust_current_block = 8831408221741692167;
                            }
                        }
                        _ => {
                            if m.rmlvo.layouts.len() > 1 {
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
                            if m.rmlvo.layouts.len() < 2
                                || single_layout_idx >= m.rmlvo.layouts.len() as u32
                            {
                                c2rust_current_block = 436805222042109220;
                            } else {
                                c2rust_current_block = 8831408221741692167;
                            }
                        }
                        _ => {
                            if m.rmlvo.layouts.len() > 1 {
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
                    let single_variant_idx =
                        if let LayoutIdx::Single { variant_idx, .. } = m.mapping.layout {
                            variant_idx
                        } else {
                            0
                        };
                    match single_variant_idx {
                        4294967291 => {
                            c2rust_current_block = 13345507216710712890;
                            match c2rust_current_block {
                                10338831042980687939 => {
                                    if m.rmlvo.variants.len() < 2
                                        || single_variant_idx >= m.rmlvo.variants.len() as u32
                                    {
                                        c2rust_current_block = 436805222042109220;
                                    } else {
                                        c2rust_current_block = 10652014663920648156;
                                    }
                                }
                                _ => {
                                    if m.rmlvo.variants.len() > 1 {
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
                                    if m.rmlvo.variants.len() < 2
                                        || single_variant_idx >= m.rmlvo.variants.len() as u32
                                    {
                                        c2rust_current_block = 436805222042109220;
                                    } else {
                                        c2rust_current_block = 10652014663920648156;
                                    }
                                }
                                _ => {
                                    if m.rmlvo.variants.len() > 1 {
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
    m.mapping.active_or_candidates_mask = 0_u32;
    false
}
fn matcher_rule_start_new(m: &mut matcher) {
    m.rule = rule::default();
    m.rule.skip = m.mapping.active_or_candidates_mask == 0;
}
fn matcher_rule_set_mlvo_common(
    m: &mut matcher,
    s: &mut scanner,
    ident: SvalIdx,
    match_type: mlvo_match_type,
) {
    if m.rule.num_mlvo_values as i32 >= m.mapping.num_mlvo as i32 {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column);
        m.rule.skip = true;
        return;
    }
    m.rule.match_type_at_pos[m.rule.num_mlvo_values as usize] = match_type;
    m.rule.mlvo_value_at_pos[m.rule.num_mlvo_values as usize] = ident;
    m.rule.num_mlvo_values = m.rule.num_mlvo_values.wrapping_add(1);
}
fn matcher_rule_set_mlvo_wildcard(m: &mut matcher, s: &mut scanner, match_type: mlvo_match_type) {
    let dummy = SvalIdx::EMPTY;
    matcher_rule_set_mlvo_common(m, s, dummy, match_type);
}
fn matcher_rule_set_mlvo_group(m: &mut matcher, s: &mut scanner, ident: SvalIdx) {
    matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_GROUP);
}
fn matcher_rule_set_mlvo(m: &mut matcher, s: &mut scanner, ident: SvalIdx) {
    matcher_rule_set_mlvo_common(m, s, ident, MLVO_MATCH_NORMAL);
}
fn matcher_rule_set_kccgst(m: &mut matcher, s: &mut scanner, ident: SvalIdx) {
    if m.rule.num_kccgst_values as i32 >= m.mapping.num_kccgst as i32 {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: has more values than the mapping line; ignoring rule\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column);
        m.rule.skip = true;
        return;
    }
    m.rule.kccgst_value_at_pos[m.rule.num_kccgst_values as usize] = ident;
    m.rule.num_kccgst_values = m.rule.num_kccgst_values.wrapping_add(1);
}
fn match_group(groups: &[group], group_name: sval, to: sval) -> bool {
    let found_group = groups.iter().find(|g| g.name.as_slice() == group_name.data);
    match found_group {
        None => false,
        Some(group) => {
            for elem in group.elements.iter() {
                if elem.as_slice() == to.data {
                    return true;
                }
            }
            false
        }
    }
}
fn match_value(
    groups: &[group],
    val: sval,
    to: sval,
    match_type: mlvo_match_type,
    wildcard_type: wildcard_match_type,
) -> bool {
    match match_type {
        1 => wildcard_type == WILDCARD_MATCH_ALL || to.len() != 0,
        2 => to.len() == 0,
        3 => to.len() != 0,
        4 => true,
        5 => match_group(groups, val, to),
        _ => svaleq(val, to),
    }
}
fn match_value_and_mark(
    groups: &[group],
    val: sval,
    to: &mut matched_sval,
    match_type: mlvo_match_type,
    wildcard_type: wildcard_match_type,
) -> bool {
    let matched: bool = match_value(groups, val, to.sval, match_type, wildcard_type);
    if matched {
        to.matched = true;
    }
    matched
}
fn expand_rmlvo_in_kccgst_value(
    m: &mut matcher,
    s: &mut scanner,
    value: sval,
    layout_idx: u32,
    expanded: &mut Vec<i8>,
    i: &mut usize,
) -> bool {
    let bytes = value.as_bytes();
    // Handle %i expansion
    if bytes[*i] == b'i'
        && ((*i).wrapping_add(1_usize) == value.len()
            || (bytes[(*i).wrapping_add(1_usize)] as i32 == MERGE_OVERRIDE_PREFIX
                || bytes[(*i).wrapping_add(1_usize)] as i32 == MERGE_AUGMENT_PREFIX
                || bytes[(*i).wrapping_add(1_usize)] as i32 == MERGE_REPLACE_PREFIX))
    {
        if layout_idx == XKB_LAYOUT_INVALID {
            let loc: scanner_loc = s.token_location();
            log::error!("[XKB-{:03}] {}:{}:{}: Invalid %i in %-expansion: there is no corresponding layout nor variant in the MLVO fields of the rules header.\n",
                XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION as i32,
                &s.file_name,
                loc.line,
                loc.column);
        } else {
            *i = (*i).wrapping_add(1);
            let idx_str = format!("{}", layout_idx.wrapping_add(1_u32));
            vec_append_nul_terminated(expanded, idx_str.as_bytes());
            return true;
        }
    } else {
        let mut sfx: i8 = 0;
        let mut pfx: i8 = 0;
        let ch = bytes[*i];
        if ch == b'('
            || ch as i32 == MERGE_OVERRIDE_PREFIX
            || ch as i32 == MERGE_AUGMENT_PREFIX
            || ch as i32 == MERGE_REPLACE_PREFIX
            || ch == b'_'
            || ch == b'-'
        {
            pfx = ch as i8;
            if ch == b'(' {
                sfx = b')' as i8;
            }
            *i = (*i).wrapping_add(1);
            if *i >= value.len() {
                // fall through to error
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
        }

        let c2rust_fresh7 = *i;
        *i = (*i).wrapping_add(1);
        let mlv: rules_mlvo = match bytes[c2rust_fresh7] {
            b'm' => MLVO_MODEL,
            b'l' => MLVO_LAYOUT,
            b'v' => MLVO_VARIANT,
            _ => {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
        };

        let mut idx: u32 = XKB_LAYOUT_INVALID;
        let mut expanded_index: bool = false;
        if *i < value.len() && bytes[*i] == b'[' {
            if mlv as u32 != MLVO_LAYOUT && mlv as u32 != MLVO_VARIANT {
                let loc_0: scanner_loc = s.token_location();
                log::error!("[XKB-{:03}] {}:{}:{}: invalid index in %-expansion; may only index layout or variant\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_0.line,
                    loc_0.column);
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
            let consumed: i32 = extract_layout_index(&bytes[*i..], &mut idx);
            if consumed == -1_i32 {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
            if idx == XKB_LAYOUT_INVALID {
                idx = layout_idx;
                expanded_index = true;
            }
            *i = (*i).wrapping_add(consumed as usize);
        }

        if sfx as i32 != 0_i32 {
            if *i >= value.len() {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
            let c2rust_fresh8 = *i;
            *i = (*i).wrapping_add(1);
            if bytes[c2rust_fresh8] as i8 != sfx {
                let loc_1: scanner_loc = s.token_location();
                log::error!(
                    "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
                    XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                    &s.file_name,
                    loc_1.line,
                    loc_1.column
                );
                return false;
            }
        }

        // Determine which expanded_value to use. Copy the sval (it's Copy).
        // Then set matched afterward.
        enum RmlvoRef {
            Model,
            Layout(usize),
            Variant(usize),
        }
        let ev_ref: Option<RmlvoRef>;

        if mlv as u32 == MLVO_LAYOUT {
            if idx == XKB_LAYOUT_INVALID {
                if m.rmlvo.layouts.len() == 1 {
                    ev_ref = Some(RmlvoRef::Layout(0));
                } else {
                    ev_ref = None;
                }
            } else if idx < m.rmlvo.layouts.len() as u32
                && (expanded_index || m.rmlvo.layouts.len() > 1)
            {
                ev_ref = Some(RmlvoRef::Layout(idx as usize));
            } else {
                ev_ref = None;
            }
        } else if mlv as u32 == MLVO_VARIANT {
            if idx == XKB_LAYOUT_INVALID {
                if m.rmlvo.variants.len() == 1 {
                    ev_ref = Some(RmlvoRef::Variant(0));
                } else {
                    ev_ref = None;
                }
            } else if idx < m.rmlvo.variants.len() as u32
                && (expanded_index || m.rmlvo.variants.len() > 1)
            {
                ev_ref = Some(RmlvoRef::Variant(idx as usize));
            } else {
                ev_ref = None;
            }
        } else if mlv as u32 == MLVO_MODEL {
            ev_ref = Some(RmlvoRef::Model);
        } else {
            ev_ref = None;
        }

        // Get the sval bytes (sval is Copy)
        let ev_sval = match &ev_ref {
            Some(RmlvoRef::Model) => m.rmlvo.model.sval,
            Some(RmlvoRef::Layout(idx)) => m.rmlvo.layouts[*idx].sval,
            Some(RmlvoRef::Variant(idx)) => m.rmlvo.variants[*idx].sval,
            None => sval::EMPTY,
        };

        if ev_ref.is_none() || ev_sval.len() == 0 {
            return true;
        }

        if pfx as i32 != 0_i32 {
            vec_append_nul_terminated(expanded, &[pfx as u8]);
        }
        vec_append_nul_terminated(expanded, ev_sval.as_bytes());
        if sfx as i32 != 0_i32 {
            vec_append_nul_terminated(expanded, &[sfx as u8]);
        }

        // Set matched flag
        match ev_ref {
            Some(RmlvoRef::Model) => m.rmlvo.model.matched = true,
            Some(RmlvoRef::Layout(idx)) => m.rmlvo.layouts[idx].matched = true,
            Some(RmlvoRef::Variant(idx)) => m.rmlvo.variants[idx].matched = true,
            None => {}
        }
        return true;
    }

    let loc_1: scanner_loc = s.token_location();
    log::error!(
        "[XKB-{:03}] {}:{}:{}: invalid %-expansion in value; not used\n",
        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
        &s.file_name,
        loc_1.line,
        loc_1.column
    );
    false
}
fn expand_qualifier_in_kccgst_value(
    m: &mut matcher,
    s: &mut scanner,
    value: sval,
    expanded: &mut Vec<i8>,
    has_layout_idx_range: bool,
    has_separator: bool,
    prefix_idx: u32,
    i: &mut usize,
) {
    let bytes = value.as_bytes();
    if (*i).wrapping_add(3_usize) <= value.len()
        && ((*i).wrapping_add(3_usize) == value.len()
            || bytes[(*i).wrapping_add(3_usize)] as i32 == MERGE_OVERRIDE_PREFIX
            || bytes[(*i).wrapping_add(3_usize)] as i32 == MERGE_AUGMENT_PREFIX
            || bytes[(*i).wrapping_add(3_usize)] as i32 == MERGE_REPLACE_PREFIX)
        && bytes[*i] == b'a'
        && bytes[(*i).wrapping_add(1_usize)] == b'l'
        && bytes[(*i).wrapping_add(2_usize)] == b'l'
    {
        if has_layout_idx_range {
            let loc: scanner_loc = s.token_location();
            log::warn!(
                "{}:{}:{}: Using :all qualifier with indices range is not recommended.\n",
                &s.file_name,
                loc.line,
                loc.column
            );
        }
        vec_append_nul_terminated(expanded, b"1");
        if m.rmlvo.layouts.len() > 1 {
            let prefix_length = expanded
                .len()
                .wrapping_sub(prefix_idx as usize)
                .wrapping_sub(1);
            let mut l: u32 = 1_u32;
            while l
                < (if 32 < m.rmlvo.layouts.len() {
                    32_u32
                } else {
                    m.rmlvo.layouts.len() as u32
                })
            {
                if !has_separator {
                    expanded.push('+' as i32 as i8);
                }
                {
                    let old_size = expanded.len();
                    let new_size = old_size.wrapping_add(prefix_length).wrapping_add(1);
                    expanded.resize(new_size, 0);
                    expanded.copy_within(
                        prefix_idx as usize..prefix_idx as usize + prefix_length,
                        old_size,
                    );
                    expanded.truncate(new_size.wrapping_sub(1));
                }
                let idx_str = format!("{}", l.wrapping_add(1_u32));
                vec_append_nul_terminated(expanded, idx_str.as_bytes());
                l = l.wrapping_add(1);
            }
        }
        *i = (*i).wrapping_add(3_usize);
    }
}
#[inline]
fn concat_kccgst(into: &mut Vec<i8>, from: &[i8]) {
    let from_plus = !from.is_empty()
        && (from[0] as i32 == MERGE_OVERRIDE_PREFIX
            || from[0] as i32 == MERGE_AUGMENT_PREFIX
            || from[0] as i32 == MERGE_REPLACE_PREFIX);
    if from_plus || into.is_empty() {
        into.extend_from_slice(from);
    } else {
        let ch = if into.is_empty() { 0i8 } else { into[0] };
        let into_plus = ch as i32 == MERGE_OVERRIDE_PREFIX
            || ch as i32 == MERGE_AUGMENT_PREFIX
            || ch as i32 == MERGE_REPLACE_PREFIX;
        if into_plus {
            let old_len = into.len();
            into.resize(old_len + from.len(), 0);
            into.copy_within(..old_len, from.len());
            for (i, &b) in from.iter().enumerate() {
                into[i] = b;
            }
        }
    }
}
fn expand_kccgst_value(
    m: &mut matcher,
    s: &mut scanner,
    value: sval,
    layout_idx: u32,
) -> Option<Vec<i8>> {
    let bytes = value.as_bytes();
    let c2rust_current_block: u64;
    let mut expanded: Vec<i8> = Vec::new();
    let mut last_item_idx: u32 = 0;
    let mut has_separator: bool = false;
    let mut i: usize = 0_usize;
    loop {
        if i >= value.len() {
            c2rust_current_block = 10758786907990354186;
            break;
        }
        match bytes[i] {
            b':' => {
                let c2rust_fresh4 = i;
                i = i.wrapping_add(1);
                vec_append_nul_terminated(&mut expanded, &[bytes[c2rust_fresh4]]);
                expand_qualifier_in_kccgst_value(
                    m,
                    s,
                    value,
                    &mut expanded,
                    matches!(m.mapping.layout, LayoutIdx::Range { .. }),
                    has_separator,
                    last_item_idx,
                    &mut i,
                );
            }
            b'%' => {
                i = i.wrapping_add(1);
                if i >= value.len()
                    || !expand_rmlvo_in_kccgst_value(m, s, value, layout_idx, &mut expanded, &mut i)
                {
                    c2rust_current_block = 1032266188497003083;
                    break;
                }
            }
            b if b as i32 == MERGE_OVERRIDE_PREFIX
                || b as i32 == MERGE_AUGMENT_PREFIX
                || b as i32 == MERGE_REPLACE_PREFIX =>
            {
                let c2rust_fresh5 = i;
                i = i.wrapping_add(1);
                vec_append_nul_terminated(&mut expanded, &[bytes[c2rust_fresh5]]);
                last_item_idx = expanded.len().wrapping_sub(1) as u32;
                has_separator = true;
            }
            _ => {
                let c2rust_fresh6 = i;
                i = i.wrapping_add(1);
                vec_append_nul_terminated(&mut expanded, &[bytes[c2rust_fresh6]]);
            }
        }
    }
    match c2rust_current_block {
        1032266188497003083 => None,
        _ => Some(expanded),
    }
}
fn matcher_append_pending_kccgst(m: &mut matcher) -> bool {
    if !matches!(m.mapping.layout, LayoutIdx::Range { .. }) {
        return true;
    }
    let (range_min, range_max) = if let LayoutIdx::Range {
        layout_idx_min,
        layout_idx_max,
    } = m.mapping.layout
    {
        (layout_idx_min, layout_idx_max)
    } else {
        unreachable!()
    };
    let mut i: kccgst_index_t = 0 as kccgst_index_t;
    while (i as i32) < m.mapping.num_kccgst as i32 {
        let kccgst: rules_kccgst = m.mapping.kccgst_at_pos[i as usize];
        let mut layout: u32 = range_min;
        while layout < range_max {
            let mut offset: usize = 0_usize;
            let mut k: u32 = 0;
            while k < m.pending_kccgst.slices.len() as u32 {
                let slice_len = m.pending_kccgst.slices[k as usize].length;
                let slice_kccgst = m.pending_kccgst.slices[k as usize].kccgst;
                let slice_layout = m.pending_kccgst.slices[k as usize].layout;
                if slice_kccgst == kccgst as u32 && slice_layout == layout && slice_len as i32 != 0
                {
                    let from: Vec<i8> =
                        m.pending_kccgst.buffer[offset..offset + slice_len as usize].to_vec();
                    concat_kccgst(&mut m.kccgst[kccgst as usize], &from);
                }
                offset = offset.wrapping_add(slice_len as usize);
                k = k.wrapping_add(1);
            }
            layout = layout.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    m.mapping.layout = LayoutIdx::default();
    true
}
fn matcher_rule_verify(m: &mut matcher, s: &mut scanner) {
    if m.rule.num_mlvo_values as i32 != m.mapping.num_mlvo as i32
        || m.rule.num_kccgst_values as i32 != m.mapping.num_kccgst as i32
    {
        let loc: scanner_loc = s.token_location();
        log::error!("[XKB-{:03}] {}:{}:{}: invalid rule: must have same number of values as mapping line; ignoring rule\n",
            XKB_ERROR_INVALID_RULES_SYNTAX as i32,
            &s.file_name,
            loc.line,
            loc.column);
        m.rule.skip = true;
    }
}
fn matcher_rule_apply_if_matches(m: &mut matcher, s: &mut scanner) {
    let mut candidate_layouts: u32 = m.mapping.active_or_candidates_mask;
    let mut idx: u32;
    let mut i: mlvo_index_t = 0 as mlvo_index_t;
    while (i as i32) < m.mapping.num_mlvo as i32 {
        let mlvo: rules_mlvo = m.mapping.mlvo_at_pos[i as usize];
        let value: sval = m.rule.mlvo_value_at_pos[i as usize].to_sval(s.s);
        let match_type: mlvo_match_type = m.rule.match_type_at_pos[i as usize];
        let mut matched: bool = false;
        if mlvo as u32 == MLVO_MODEL {
            matched = match_value_and_mark(
                &m.groups,
                value,
                &mut m.rmlvo.model,
                match_type,
                WILDCARD_MATCH_ALL,
            );
        } else if let LayoutIdx::Range {
            layout_idx_min,
            layout_idx_max,
        } = m.mapping.layout
        {
            idx = layout_idx_min;
            while idx < layout_idx_max && candidate_layouts != 0 {
                let mask: u32 = 1_u32 << idx;
                if candidate_layouts & mask != 0 {
                    match mlvo as u32 {
                        1 => {
                            if match_value_and_mark(
                                &m.groups,
                                value,
                                &mut m.rmlvo.layouts[idx as usize],
                                match_type,
                                WILDCARD_MATCH_NONEMPTY,
                            ) {
                                matched = true;
                            } else {
                                candidate_layouts &= !mask;
                            }
                        }
                        2 => {
                            if match_value_and_mark(
                                &m.groups,
                                value,
                                &mut m.rmlvo.variants[idx as usize],
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
                            for opt_idx in 0..m.rmlvo.options.len() {
                                let opt = &m.rmlvo.options[opt_idx];
                                if opt.layout as i32 != OPTIONS_MATCH_ALL_GROUPS
                                    && opt.layout != idx
                                {
                                    continue;
                                }
                                if match_value_and_mark(
                                    &m.groups,
                                    value,
                                    &mut m.rmlvo.options[opt_idx],
                                    match_type,
                                    WILDCARD_MATCH_ALL,
                                ) {
                                    matched = true;
                                    found_option = true;
                                    break;
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
            let li = m.mapping.layout.layout_idx_min() as usize;
            match mlvo as u32 {
                1 => {
                    matched = match_value_and_mark(
                        &m.groups,
                        value,
                        &mut m.rmlvo.layouts[li],
                        match_type,
                        WILDCARD_MATCH_NONEMPTY,
                    );
                }
                2 => {
                    matched = match_value_and_mark(
                        &m.groups,
                        value,
                        &mut m.rmlvo.variants[li],
                        match_type,
                        WILDCARD_MATCH_NONEMPTY,
                    );
                }
                _ => {
                    let layout_min = m.mapping.layout.layout_idx_min();
                    for opt_idx in 0..m.rmlvo.options.len() {
                        let opt = &m.rmlvo.options[opt_idx];
                        if opt.layout as i32 != OPTIONS_MATCH_ALL_GROUPS && opt.layout != layout_min
                        {
                            continue;
                        }
                        matched = match_value_and_mark(
                            &m.groups,
                            value,
                            &mut m.rmlvo.options[opt_idx],
                            match_type,
                            WILDCARD_MATCH_ALL,
                        );
                        if matched {
                            break;
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
    if let LayoutIdx::Range {
        layout_idx_min,
        layout_idx_max,
    } = m.mapping.layout
    {
        idx = layout_idx_min;
        while idx < layout_idx_max {
            if candidate_layouts & 1_u32 << idx != 0 {
                let mut i_0: kccgst_index_t = 0 as kccgst_index_t;
                while (i_0 as i32) < m.mapping.num_kccgst as i32 {
                    let kccgst: rules_kccgst = m.mapping.kccgst_at_pos[i_0 as usize];
                    let value_0: sval = m.rule.kccgst_value_at_pos[i_0 as usize].to_sval(s.s);
                    let prev_buffer_length: u32 = m.pending_kccgst.buffer.len() as u32;
                    if let Some(expanded) = expand_kccgst_value(m, s, value_0, idx) {
                        if !expanded.is_empty() {
                            m.pending_kccgst.buffer.extend_from_slice(&expanded);
                        }
                        let length: u32 =
                            (m.pending_kccgst.buffer.len() as u32).wrapping_sub(prev_buffer_length);
                        let slice = kccgst_buffer_slice {
                            length,
                            kccgst,
                            layout: idx,
                        };
                        m.pending_kccgst.slices.push(slice);
                    }
                    i_0 = i_0.wrapping_add(1);
                }
            }
            idx = idx.wrapping_add(1);
        }
    } else if let LayoutIdx::Index { layout_idx_min, .. } = m.mapping.layout {
        let mut i_1: kccgst_index_t = 0 as kccgst_index_t;
        while (i_1 as i32) < m.mapping.num_kccgst as i32 {
            let kccgst_0: rules_kccgst = m.mapping.kccgst_at_pos[i_1 as usize];
            let value_1: sval = m.rule.kccgst_value_at_pos[i_1 as usize].to_sval(s.s);
            if let Some(expanded) = expand_kccgst_value(m, s, value_1, layout_idx_min) {
                if !expanded.is_empty() {
                    concat_kccgst(&mut m.kccgst[kccgst_0 as usize], &expanded);
                }
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
    if !is_mlvo_mask_defined(m, MLVO_OPTION) {
        m.mapping.active_or_candidates_mask &= !candidate_layouts;
    }
}
fn gettok(m: &mut matcher, s: &mut scanner) -> rules_token {
    lex(s, &mut m.val)
}
fn matcher_match(m: &mut matcher, s: &mut scanner, include_depth: u32, _file_name: &str) -> bool {
    let c2rust_current_block: u64;
    let mut tok: rules_token;

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
                    matcher_group_start_new(m, m.val.string.to_sval(s.s).data);
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
                    matcher_include(m, s, include_depth, m.val.string.to_sval(s.s));
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
                    matcher_mapping_set_mlvo(m, s, m.val.string.to_sval(s.s));
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
                        if m.mapping.active_or_candidates_mask != 0 {
                            matcher_mapping_set_mlvo(m, s, m.val.string.to_sval(s.s));
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
                        if m.mapping.active_or_candidates_mask != 0 {
                            matcher_mapping_set_kccgst(m, s, m.val.string.to_sval(s.s));
                        }
                    }
                    if m.mapping.active_or_candidates_mask != 0
                        && matcher_mapping_verify(m, s) as i32 != 0
                    {
                        matcher_mapping_set_layout_bounds(m);
                        if matches!(m.mapping.layout, LayoutIdx::Range { .. }) {
                            m.pending_kccgst.buffer.clear();
                            m.pending_kccgst.slices.clear();
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
                                            if !m.rule.skip {
                                                if m.val.string.len() == 1
                                                    && s.s[m.val.string.start] == b'+'
                                                {
                                                    matcher_rule_set_mlvo_wildcard(
                                                        m,
                                                        s,
                                                        MLVO_MATCH_WILDCARD_SOME,
                                                    );
                                                } else {
                                                    matcher_rule_set_mlvo(m, s, m.val.string);
                                                }
                                            }
                                        }
                                        6 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_LEGACY,
                                                );
                                            }
                                        }
                                        7 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_NONE,
                                                );
                                            }
                                        }
                                        8 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_SOME,
                                                );
                                            }
                                        }
                                        9 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_wildcard(
                                                    m,
                                                    s,
                                                    MLVO_MATCH_WILDCARD_ANY,
                                                );
                                            }
                                        }
                                        3 => {
                                            if !m.rule.skip {
                                                matcher_rule_set_mlvo_group(m, s, m.val.string);
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
                                    if !m.rule.skip {
                                        matcher_rule_set_kccgst(m, s, m.val.string);
                                    }
                                }
                                if !m.rule.skip {
                                    matcher_rule_verify(m, s);
                                }
                                if !m.rule.skip {
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
            matcher_group_add_element(m, s, m.val.string.to_sval(s.s).data);
        }
    }
    match c2rust_current_block {
        13194772801876125100 => true,
        _ => {
            match tok as u32 {
                11 => {}
                _ => {
                    let loc: scanner_loc = s.token_location();
                    log::error!(
                        "[XKB-{:03}] {}:{}:{}: unexpected token\n",
                        XKB_ERROR_INVALID_RULES_SYNTAX as i32,
                        &s.file_name,
                        loc.line,
                        loc.column
                    );
                }
            }
            false
        }
    }
}
fn read_rules_file(
    matcher: &mut matcher<'_>,
    include_depth: u32,
    file: &std::fs::File,
    path: &str,
) -> bool {
    #[allow(unused_assignments)]
    let mut scanner: scanner = scanner::new(&[], "");

    use crate::xkb::utils::MappedFile;

    let mapped = match MappedFile::new(file) {
        Ok(m) => m,
        Err(e) => {
            log::error!("Couldn't read rules file \"{}\": {}\n", path, e);
            return false;
        }
    };

    scanner = scanner::new(mapped.as_bytes(), path);
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
        return false;
    }
    let ret: bool = matcher_match(matcher, &mut scanner, include_depth, path);
    ret
}
fn xkb_resolve_partial_rules(rules: &str, suffix: &str, matcher: &mut matcher<'_>) -> bool {
    let partial_rules = format!("{}{}", rules, suffix);
    if partial_rules.len() >= 60 {
        log::error!(
            "[XKB-{:03}] Cannot load XKB rules \"{}{}\"\n",
            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
            rules,
            suffix
        );
        return false;
    }
    let mut offset: u32 = 0;
    loop {
        let found = FindFileInXkbPath(
            &mut *matcher.ctx,
            "(unknown)",
            &partial_rules,
            FILE_TYPE_RULES,
            &mut offset,
            false,
        );
        let Some((file, path)) = found else { break };
        let ok: bool = read_rules_file(matcher, 0, &file, &path);
        drop(file);
        if !ok {
            log::error!(
                "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                &path
            );
            return false;
        }
        offset += 1;
    }
    true
}
fn xkb_resolve_rules(
    rules: &str,
    matcher: &mut matcher<'_>,
    out: &mut xkb_component_names,
    explicit_layouts: &mut u32,
) -> bool {
    let mut ret: bool = false;
    let mut offset: u32 = 0;
    let rules_str = rules;
    let found = FindFileInXkbPath(
        &mut *matcher.ctx,
        "(unknown)",
        rules_str,
        FILE_TYPE_RULES,
        &mut offset,
        true,
    );
    let Some((file, path)) = found else {
        log::error!(
            "[XKB-{:03}] Cannot load XKB rules \"{}\"\n",
            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
            rules_str
        );
        return false;
    };
    ret = xkb_resolve_partial_rules(rules_str, ".pre", matcher);
    if ret {
        ret = read_rules_file(matcher, 0, &file, &path);
        if !ret {
            log::error!(
                "[XKB-{:03}] Error while parsing XKB rules \"{}\"\n",
                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                &path
            );
        } else {
            ret = xkb_resolve_partial_rules(rules_str, ".post", matcher);
            if ret {
                if matcher.kccgst[KCCGST_KEYCODES as usize].is_empty()
                    || matcher.kccgst[KCCGST_TYPES as usize].is_empty()
                    || matcher.kccgst[KCCGST_COMPAT as usize].is_empty()
                    || matcher.kccgst[KCCGST_SYMBOLS as usize].is_empty()
                {
                    log::error!(
                        "[XKB-{:03}] No components returned from XKB rules \"{}\"\n",
                        XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                        rules_str
                    );
                    ret = false;
                } else {
                    // Transfer ownership of Vec data directly.
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_KEYCODES as usize]);
                        v.push(0);
                        out.keycodes = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_TYPES as usize]);
                        v.push(0);
                        out.types = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_COMPAT as usize]);
                        v.push(0);
                        out.compatibility = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_SYMBOLS as usize]);
                        v.push(0);
                        out.symbols = v;
                    }
                    {
                        let mut v = std::mem::take(&mut matcher.kccgst[KCCGST_GEOMETRY as usize]);
                        v.push(0);
                        out.geometry = v;
                    }
                    if !matcher.rmlvo.model.matched && matcher.rmlvo.model.sval.len() > 0 {
                        log::error!(
                            "[XKB-{:03}] Unrecognized RMLVO model \"{}\" was ignored\n",
                            XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                            matcher.rmlvo.model.sval.as_str()
                        );
                    }
                    for mval in matcher.rmlvo.layouts.iter() {
                        if !mval.matched && mval.sval.len() > 0 {
                            log::error!(
                                "[XKB-{:03}] Unrecognized RMLVO layout \"{}\" was ignored\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                mval.sval.as_str()
                            );
                        }
                    }
                    for mval in matcher.rmlvo.variants.iter() {
                        if !mval.matched && mval.sval.len() > 0 {
                            log::error!(
                                "[XKB-{:03}] Unrecognized RMLVO variant \"{}\" was ignored\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                mval.sval.as_str()
                            );
                        }
                    }
                    for mval in matcher.rmlvo.options.iter() {
                        if !mval.matched && mval.sval.len() > 0 {
                            log::error!(
                                "[XKB-{:03}] Unrecognized RMLVO option \"{}\" was ignored\n",
                                XKB_ERROR_CANNOT_RESOLVE_RMLVO as i32,
                                mval.sval.as_str()
                            );
                        }
                    }
                    if !out.symbols.is_empty() {
                        *explicit_layouts = 1_u32;
                        // Parse symbols string to find explicit layout count
                        let sym_bytes: Vec<u8> = out.symbols.iter().map(|&b| b as u8).collect();
                        let mut pos: usize = 0;
                        loop {
                            match sym_bytes[pos..].iter().position(|&b| b == b':') {
                                None => break,
                                Some(colon_off) => {
                                    pos += colon_off + 1;
                                    if pos >= sym_bytes.len() || sym_bytes[pos] == 0 {
                                        break;
                                    }
                                    let (val_parsed, count) =
                                        crate::xkb::utils::parse_dec_u32(&sym_bytes[pos..]);
                                    let group: u32 = val_parsed;
                                    let count = count as usize;
                                    if count > 0
                                        && pos + count <= sym_bytes.len()
                                        && (sym_bytes.get(pos + count).copied() == Some(0)
                                            || sym_bytes.get(pos + count).map(|&b| b as i32)
                                                == Some(MERGE_OVERRIDE_PREFIX)
                                            || sym_bytes.get(pos + count).map(|&b| b as i32)
                                                == Some(MERGE_AUGMENT_PREFIX)
                                            || sym_bytes.get(pos + count).map(|&b| b as i32)
                                                == Some(MERGE_REPLACE_PREFIX))
                                        && group > 0
                                        && group <= XKB_MAX_GROUPS as u32
                                    {
                                        *explicit_layouts = (*explicit_layouts).max(group);
                                        pos += count;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ret
}
pub fn xkb_components_from_rules_names(
    ctx: &mut xkb_context,
    rmlvo: &xkb_rule_names,
    out: &mut xkb_component_names,
    explicit_layouts: &mut u32,
) -> bool {
    let mut matcher = matcher_new_from_names(ctx, rmlvo);
    let rules_str = rmlvo.rules.to_str().unwrap_or("");
    let ret: bool = xkb_resolve_rules(rules_str, &mut *matcher, out, explicit_layouts);
    drop(matcher);
    ret
}
use crate::xkb::shared_types::*;
