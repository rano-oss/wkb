//! Compose table support: iteration, parsing, and layout-to-locale mapping.

// --- Compose table types and iteration (from compose_iter.rs) ---

#[derive(Clone)]
pub struct xkb_compose_table {
    pub refcnt: i32,
    pub format: xkb_compose_format,
    pub flags: xkb_compose_compile_flags,
    pub ctx: xkb_context,
    pub locale: String,
    pub utf8: Vec<i8>,
    pub nodes: Vec<compose_node>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compose_node {
    pub keysym: u32,
    pub lokid: u32,
    pub hikid: u32,
    pub data: ComposeNodeData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ComposeNodeData {
    pub tag: ComposeTag,
    pub internal: ComposeInternal,
    pub leaf: ComposeLeaf,
}
/// Leaf node: bits 0..30 = utf8 index, bit 31 = is_leaf (always true).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComposeLeaf {
    pub utf8_is_leaf: u32,
    pub keysym: u32,
}
impl ComposeLeaf {
    #[inline]
    pub fn utf8(&self) -> u32 {
        self.utf8_is_leaf & 0x7FFF_FFFF
    }
}
/// Internal node: bits 0..30 = pad, bit 31 = is_leaf (always false).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComposeInternal {
    pub _pad_is_leaf: u32,
    pub eqkid: u32,
}
/// Tag-only accessor: bit 31 = is_leaf discriminant.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComposeTag {
    pub _pad_is_leaf: u32,
}
impl ComposeTag {
    #[inline]
    pub fn is_leaf(&self) -> bool {
        (self._pad_is_leaf >> 31) != 0
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_compose_table_entry {
    pub sequence_length: usize,
    pub sequence: *mut u32,
    pub keysym: u32,
    pub utf8: *const i8,
}

pub type xkb_compose_compile_flags = u32;
pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
pub type xkb_compose_format = u32;
pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
pub type xkb_compose_table_iter_t =
    Option<unsafe fn(*mut xkb_compose_table_entry, *mut ::core::ffi::c_void) -> ()>;

unsafe fn for_each_helper(
    table: *mut xkb_compose_table,
    iter: xkb_compose_table_iter_t,
    data: *mut ::core::ffi::c_void,
    syms: *mut u32,
    mut nsyms: usize,
    p: u32,
) {
    unsafe {
        if p == 0 {
            return;
        }
        let node: *const compose_node = (*table).nodes.as_ptr().offset(p as isize);
        for_each_helper(table, iter, data, syms, nsyms, (*node).lokid);
        let c2rust_fresh0 = nsyms;
        nsyms = nsyms.wrapping_add(1);
        *syms.add(c2rust_fresh0) = (*node).keysym;
        if (*node).data.tag.is_leaf() {
            let mut entry: xkb_compose_table_entry = xkb_compose_table_entry {
                sequence_length: nsyms,
                sequence: syms,
                keysym: (*node).data.leaf.keysym,
                utf8: (*table)
                    .utf8
                    .as_ptr()
                    .offset((*node).data.leaf.utf8() as isize) as *mut i8,
            };
            iter.expect("non-null function pointer")(&raw mut entry, data);
        } else {
            for_each_helper(table, iter, data, syms, nsyms, (*node).data.internal.eqkid);
        }
        nsyms = nsyms.wrapping_sub(1);
        for_each_helper(table, iter, data, syms, nsyms, (*node).hikid);
    }
}

pub unsafe fn xkb_compose_table_for_each(
    table: *mut xkb_compose_table,
    iter: xkb_compose_table_iter_t,
    data: *mut ::core::ffi::c_void,
) {
    unsafe {
        if (*table).nodes.len() <= 1 {
            return;
        }
        let mut syms: [u32; 10] = [0; 10];
        for_each_helper(table, iter, data, &raw mut syms as *mut u32, 0_usize, 1_u32);
    }
}

use crate::xkb::shared_types::*;

// --- Compose file resolution (from compose_parse.rs) ---

use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use crate::composer::{ListComposer, Token};
use xkb_parser::parse_compose_file;

const LOCALE_DIR: &str = "/usr/share/X11/locale";

fn lookup_locale_file(
    filename: &str,
    match_index: usize,
    return_index: usize,
    locale: &str,
) -> Option<String> {
    let path = Path::new(LOCALE_DIR).join(filename);
    let file = fs::File::open(path).ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() > match_index && parts.len() > return_index && parts[match_index] == locale {
            return Some(parts[return_index].to_string());
        }
    }
    None
}

fn lookup_compose_dir(locale: &str) -> Option<String> {
    lookup_locale_file("compose.dir", 1, 0, locale)
}

/// Resolve a locale name to the compose file sub-path (relative to
/// `/usr/share/X11/locale/`) that should be used.
///
/// Returns e.g. `Some("en_US.UTF-8/Compose")` for locale `"de"`.
/// Falls back to `"en_US.UTF-8"` when no locale-specific compose file
/// can be found, matching xkbcommon's behaviour.  Returns `None` only
pub fn resolve_compose_file(locale: &str) -> Option<String> {
    if let Some(&mapped_locale) = XKB_COMPOSE_MAP.get(locale) {
        if let Some(compose_file) = lookup_compose_dir(mapped_locale) {
            return Some(compose_file);
        }
    }

    if let Some(compose_file) = lookup_compose_dir(locale) {
        return Some(compose_file);
    }

    if let Some(resolved) = lookup_locale_file("locale.alias", 0, 1, locale) {
        if let Some(dot_pos) = resolved.find('.') {
            let base = &resolved[..dot_pos];
            if !resolved[dot_pos..].eq_ignore_ascii_case(".UTF-8") {
                let utf8_locale = format!("{}.UTF-8", base);
                if let Some(compose_file) = lookup_compose_dir(&utf8_locale) {
                    return Some(compose_file);
                }
            }
        }

        if let Some(compose_file) = lookup_compose_dir(&resolved) {
            return Some(compose_file);
        }
    }

    if locale.len() >= 2 && locale.len() <= 3 && locale.chars().all(|c| c.is_ascii_lowercase()) {
        let upper = locale.to_ascii_uppercase();
        let candidate = format!("{}_{}.UTF-8", locale, upper);
        if let Some(compose_file) = lookup_compose_dir(&candidate) {
            return Some(compose_file);
        }
    }

    lookup_compose_dir("en_US.UTF-8")
}

pub fn load_compose_from_path(path: &Path) -> ListComposer {
    let mut regular = ListComposer::new();

    let entries = parse_compose_file(path);

    for entry in entries {
        let mut tokens: Vec<Token> = Vec::new();
        let mk_idx = entry.multi_key_index;

        for (i, ch) in entry.keys.iter().enumerate() {
            if let Some(idx) = mk_idx {
                if idx == i {
                    tokens.push(Token::Compose);
                }
            }
            tokens.push(Token::Char(*ch));
        }
        regular.insert(&tokens, entry.output);
    }
    regular
}

// --- Layout-to-locale mapping (from xkb_compose_map.rs) ---

use std::collections::BTreeMap;

lazy_static::lazy_static! {
    pub static ref XKB_COMPOSE_MAP: BTreeMap<&'static str, &'static str> = [
        // --- Layouts where XKB name != language code ---
        ("us",    "en_US.UTF-8"),
        ("gb",    "en_GB.UTF-8"),
        ("au",    "en_AU.UTF-8"),
        ("nz",    "en_NZ.UTF-8"),
        ("za",    "en_ZA.UTF-8"),
        ("bw",    "en_BW.UTF-8"),
        ("no",    "nb_NO.UTF-8"),
        ("dk",    "da_DK.UTF-8"),
        ("se",    "sv_SE.UTF-8"),
        ("at",    "de_AT.UTF-8"),
        ("ch",    "de_CH.UTF-8"),
        ("cz",    "cs_CZ.UTF-8"),
        ("gr",    "el_GR.UTF-8"),
        ("rs",    "sr_RS.UTF-8"),
        ("me",    "sr_ME.UTF-8"),
        ("al",    "sq_AL.UTF-8"),
        ("ba",    "bs_BA.UTF-8"),
        ("by",    "be_BY.UTF-8"),
        ("ge",    "ka_GE.UTF-8"),
        ("ua",    "uk_UA.UTF-8"),
        ("jp",    "ja_JP.UTF-8"),
        ("kr",    "ko_KR.UTF-8"),
        ("cn",    "zh_CN.UTF-8"),
        ("tw",    "zh_TW.UTF-8"),
        ("kh",    "km_KH.UTF-8"),
        ("vn",    "vi_VN.UTF-8"),
        ("in",    "hi_IN.UTF-8"),
        ("bd",    "bn_BD.UTF-8"),
        ("lk",    "si_LK.UTF-8"),
        ("np",    "ne_NP.UTF-8"),
        ("pk",    "ur_PK.UTF-8"),
        ("il",    "he_IL.UTF-8"),
        // --- Arabic-script countries ---
        ("ara",   "ar_SA.UTF-8"),
        ("iq",    "ar_IQ.UTF-8"),
        ("ir",    "fa_IR.UTF-8"),
        ("sy",    "ar_SY.UTF-8"),
        ("eg",    "ar_EG.UTF-8"),
        ("dz",    "ar_DZ.UTF-8"),
        ("ma",    "ar_MA.UTF-8"),
        // --- Central Asian ---
        ("kg",    "ky_KG.UTF-8"),
        ("kz",    "kk_KZ.UTF-8"),
        ("tj",    "tg_TJ.UTF-8"),
        ("la",    "lo_LA.UTF-8"),
        ("my",    "ms_MY.UTF-8"),
        // --- Irish / Esperanto / Latin American ---
        ("ie",    "ga_IE.UTF-8"),
        ("epo",   "eo_XX.UTF-8"),
        ("latam", "es_MX.UTF-8"),
    ].into();
}
