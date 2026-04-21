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

pub struct compose_node {
    pub keysym: u32,
    pub lokid: u32,
    pub hikid: u32,
    pub data: ComposeNodeData,
}
#[derive(Copy, Clone)]

pub union ComposeNodeData {
    pub tag: ComposeTag,
    pub internal: ComposeInternal,
    pub leaf: ComposeLeaf,
}
/// Leaf node: bits 0..30 = utf8 index, bit 31 = is_leaf (always true).
#[derive(Copy, Clone)]

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

pub struct ComposeInternal {
    pub _pad_is_leaf: u32,
    pub eqkid: u32,
}
/// Tag-only accessor: bit 31 = is_leaf discriminant.
#[derive(Copy, Clone)]

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

use crate::shared_types::*;

// --- Compose file resolution (from compose_parse.rs) ---

use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

const LOCALE_DIR: &str = "/usr/share/X11/locale";

// --- Hand-rolled Compose file parser (replaces xkb-parser crate) ---

/// A parsed Compose file entry.
pub struct ComposeEntry {
    pub keys: Vec<char>,
    pub keysym_names: Vec<String>,
    pub multi_key_index: Option<usize>,
    pub output: char,
}

/// Resolve an XKB keysym name to its Unicode character using our existing
/// keysym database.
pub fn keysym_name_to_char(name: &str) -> Option<char> {
    use crate::keysym::xkb_keysym_from_name;
    use crate::keysym_utf::xkb_keysym_to_utf32;
    use crate::shared_types::{XKB_KEY_NoSymbol, XKB_KEYSYM_NO_FLAGS};

    let ks = xkb_keysym_from_name(name.as_bytes(), XKB_KEYSYM_NO_FLAGS);
    if ks == XKB_KEY_NoSymbol as u32 {
        // Try U<hex> notation
        if let Some(hex) = name.strip_prefix('U') {
            if !hex.is_empty() && hex.len() <= 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return u32::from_str_radix(hex, 16).ok().and_then(char::from_u32);
            }
        }
        // Single printable ASCII char
        if name.len() == 1 {
            return name.chars().next();
        }
        return None;
    }
    let utf32 = xkb_keysym_to_utf32(ks);
    if utf32 == 0 {
        return None;
    }
    char::from_u32(utf32)
}

/// Parse a Compose file at the given path, recursively handling `include` directives.
pub fn parse_compose_file(path: &Path) -> Vec<ComposeEntry> {
    let mut out = Vec::new();
    parse_compose_file_impl(path, &mut out);
    out
}

fn parse_compose_file_impl(path: &Path, out: &mut Vec<ComposeEntry>) {
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Handle include directives: include "path"
        if let Some(rest) = trimmed.strip_prefix("include") {
            let rest = rest.trim();
            if let Some(include_str) = rest.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                if include_str.is_empty() {
                    continue;
                }
                let include_path = Path::new(include_str);
                let resolved = if include_path.is_absolute() {
                    include_path.to_path_buf()
                } else if let Some(parent) = path.parent() {
                    parent.join(include_path)
                } else {
                    include_path.to_path_buf()
                };
                parse_compose_file_impl(&resolved, out);
            }
            continue;
        }

        // Parse rule lines: <key1> <key2> ... : "output" [keysym]
        if let Some(entry) = parse_rule_line(trimmed) {
            out.push(entry);
        }
    }
}

/// Parse a single rule line like `<Multi_key> <a> <e> : "æ" ae`
fn parse_rule_line(line: &str) -> Option<ComposeEntry> {
    // Split on ':'
    let colon_pos = line.find(':')?;
    let lhs = &line[..colon_pos];
    let rhs = line[colon_pos + 1..].trim();

    // Strip trailing comment from rhs
    let rhs = if let Some(hash) = rhs.find('#') {
        rhs[..hash].trim()
    } else {
        rhs
    };

    // Parse LHS: sequence of <keysym_name> tokens
    let mut keys = Vec::new();
    let mut keysym_names = Vec::new();
    let mut multi_key_index: Option<usize> = None;
    let mut pos = 0;
    let lhs_bytes = lhs.as_bytes();
    while pos < lhs_bytes.len() {
        if lhs_bytes[pos] == b'<' {
            let end = lhs[pos..].find('>')? + pos;
            let name = &lhs[pos + 1..end];
            if name.eq_ignore_ascii_case("Multi_key") {
                if multi_key_index.is_none() {
                    multi_key_index = Some(keys.len());
                }
            } else {
                let ch = keysym_name_to_char(name)?;
                keys.push(ch);
                keysym_names.push(name.to_string());
            }
            pos = end + 1;
        } else {
            pos += 1;
        }
    }

    if keys.is_empty() {
        return None;
    }

    // Parse RHS: "string" [keysym_name] or keysym_name
    let output = parse_rhs_value(rhs)?;

    Some(ComposeEntry {
        keys,
        keysym_names,
        multi_key_index,
        output,
    })
}

/// Parse the RHS value: `"string" [keysym]` or bare `keysym_name`
fn parse_rhs_value(rhs: &str) -> Option<char> {
    let rhs = rhs.trim();
    if rhs.starts_with('"') {
        // Quoted string: extract first char
        let end_quote = rhs[1..].find('"')? + 1;
        let s = &rhs[1..end_quote];
        if !s.is_empty() && !s.starts_with('\\') {
            if let Some(ch) = s.chars().next() {
                if !ch.is_ascii_digit() {
                    return Some(ch);
                }
            }
        }
        // Fall back to keysym name after the quoted string
        let after = rhs[end_quote + 1..].trim();
        if !after.is_empty() {
            let name = after.split_whitespace().next()?;
            return keysym_name_to_char(name);
        }
        // Try to get first char from the string as fallback
        s.chars().next()
    } else {
        // Bare keysym name
        let name = rhs.split_whitespace().next()?;
        keysym_name_to_char(name)
    }
}

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

// --- Layout-to-locale mapping (from xkb_compose_map.rs) ---

use std::collections::BTreeMap;

use std::sync::LazyLock;

static XKB_COMPOSE_MAP: LazyLock<BTreeMap<&'static str, &'static str>> = LazyLock::new(|| {
    [
        // --- Layouts where XKB name != language code ---
        ("us", "en_US.UTF-8"),
        ("gb", "en_GB.UTF-8"),
        ("au", "en_AU.UTF-8"),
        ("nz", "en_NZ.UTF-8"),
        ("za", "en_ZA.UTF-8"),
        ("bw", "en_BW.UTF-8"),
        ("no", "nb_NO.UTF-8"),
        ("dk", "da_DK.UTF-8"),
        ("se", "sv_SE.UTF-8"),
        ("at", "de_AT.UTF-8"),
        ("ch", "de_CH.UTF-8"),
        ("cz", "cs_CZ.UTF-8"),
        ("gr", "el_GR.UTF-8"),
        ("rs", "sr_RS.UTF-8"),
        ("me", "sr_ME.UTF-8"),
        ("al", "sq_AL.UTF-8"),
        ("ba", "bs_BA.UTF-8"),
        ("by", "be_BY.UTF-8"),
        ("ge", "ka_GE.UTF-8"),
        ("ua", "uk_UA.UTF-8"),
        ("jp", "ja_JP.UTF-8"),
        ("kr", "ko_KR.UTF-8"),
        ("cn", "zh_CN.UTF-8"),
        ("tw", "zh_TW.UTF-8"),
        ("kh", "km_KH.UTF-8"),
        ("vn", "vi_VN.UTF-8"),
        ("in", "hi_IN.UTF-8"),
        ("bd", "bn_BD.UTF-8"),
        ("lk", "si_LK.UTF-8"),
        ("np", "ne_NP.UTF-8"),
        ("pk", "ur_PK.UTF-8"),
        ("il", "he_IL.UTF-8"),
        // --- Arabic-script countries ---
        ("ara", "ar_SA.UTF-8"),
        ("iq", "ar_IQ.UTF-8"),
        ("ir", "fa_IR.UTF-8"),
        ("sy", "ar_SY.UTF-8"),
        ("eg", "ar_EG.UTF-8"),
        ("dz", "ar_DZ.UTF-8"),
        ("ma", "ar_MA.UTF-8"),
        // --- Central Asian ---
        ("kg", "ky_KG.UTF-8"),
        ("kz", "kk_KZ.UTF-8"),
        ("tj", "tg_TJ.UTF-8"),
        ("la", "lo_LA.UTF-8"),
        ("my", "ms_MY.UTF-8"),
        // --- Irish / Esperanto / Latin American ---
        ("ie", "ga_IE.UTF-8"),
        ("epo", "eo_XX.UTF-8"),
        ("latam", "es_MX.UTF-8"),
    ]
    .into()
});
