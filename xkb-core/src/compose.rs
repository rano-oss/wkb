//! Compose table support: iteration, parsing, and layout-to-locale mapping.

// --- Compose table types and iteration (from compose_iter.rs) ---

#[derive(Clone)]
pub(crate) struct xkb_compose_table {
    pub(crate) refcnt: i32,
    pub(crate) format: xkb_compose_format,
    pub(crate) flags: xkb_compose_compile_flags,
    pub(crate) ctx: xkb_context,
    pub(crate) locale: String,
    pub(crate) utf8: Vec<i8>,
    pub(crate) nodes: Vec<compose_node>,
}
#[derive(Copy, Clone)]

pub(crate) struct compose_node {
    pub(crate) keysym: u32,
    pub(crate) lokid: u32,
    pub(crate) hikid: u32,
    pub(crate) data: ComposeNodeData,
}
#[derive(Copy, Clone)]

pub(crate) union ComposeNodeData {
    pub(crate) tag: ComposeTag,
    pub(crate) internal: ComposeInternal,
    pub(crate) leaf: ComposeLeaf,
}
/// Leaf node: bits 0..30 = utf8 index, bit 31 = is_leaf (always true).
#[derive(Copy, Clone)]

pub(crate) struct ComposeLeaf {
    pub(crate) utf8_is_leaf: u32,
    pub(crate) keysym: u32,
}
impl ComposeLeaf {
    #[inline]
    pub(crate) fn utf8(&self) -> u32 {
        self.utf8_is_leaf & 0x7FFF_FFFF
    }
}
/// Internal node: bits 0..30 = pad, bit 31 = is_leaf (always false).
#[derive(Copy, Clone)]

pub(crate) struct ComposeInternal {
    pub(crate) _pad_is_leaf: u32,
    pub(crate) eqkid: u32,
}
/// Tag-only accessor: bit 31 = is_leaf discriminant.
#[derive(Copy, Clone)]

pub(crate) struct ComposeTag {
    pub(crate) _pad_is_leaf: u32,
}
impl ComposeTag {
    #[inline]
    pub(crate) fn is_leaf(&self) -> bool {
        (self._pad_is_leaf >> 31) != 0
    }
}
#[derive(Copy, Clone)]

pub(crate) struct xkb_compose_table_entry {
    pub(crate) sequence_length: usize,
    pub(crate) sequence: *mut u32,
    pub(crate) keysym: u32,
    pub(crate) utf8: *const i8,
}

pub(crate) type xkb_compose_compile_flags = u32;
pub(crate) const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
pub(crate) type xkb_compose_format = u32;
pub(crate) const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;

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
    use crate::keysym_utf::keysym_to_utf32;
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
    let utf32 = keysym_to_utf32(ks);
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

// ════════════════════════════════════════════════════════════════════════
// Public compose API — keysym-based table + state machine
// ════════════════════════════════════════════════════════════════════════

/// Status of the compose state machine after feeding a keysym.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ComposeStatus {
    /// No compose sequence in progress.
    Nothing = 0,
    /// A compose sequence is in progress.
    Composing = 1,
    /// A complete compose sequence was matched.
    Composed = 2,
    /// The compose sequence was cancelled (no match).
    Cancelled = 3,
}

/// Result of feeding a keysym to the compose state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposeFeedResult {
    Ignored,
    Accepted,
}

/// A node in the keysym-based ternary search trie.
#[derive(Clone)]
struct TstNode {
    keysym: u32,
    lo: u32, // index of lo child (0 = none)
    hi: u32, // index of hi child (0 = none)
    eq: u32, // index of eq child (0 = none)
    // Leaf data (valid when is_leaf is true)
    is_leaf: bool,
    leaf_keysym: u32,
    leaf_utf8: String,
}

/// A compiled compose table built from a locale's Compose file.
/// Uses a keysym-based ternary search trie, matching libxkbcommon's structure.
pub struct ComposeTable {
    nodes: Vec<TstNode>,
}

impl ComposeTable {
    /// Create a compose table from a locale string (e.g. "en_US.UTF-8" or "de").
    /// Returns `None` if the compose file cannot be resolved or parsed.
    pub fn new_from_locale(locale: &str) -> Option<Self> {
        let subpath = resolve_compose_file(locale)?;
        let path = Path::new(LOCALE_DIR).join(&subpath);
        Some(Self::new_from_file(&path))
    }

    /// Create a compose table from a Compose file path.
    pub fn new_from_file(path: &Path) -> Self {
        use crate::keysym::xkb_keysym_from_name;
        use crate::keysym_utf::keysym_to_utf32;
        use crate::shared_types::XKB_KEYSYM_NO_FLAGS;

        let mut table = ComposeTable {
            // Index 0 is reserved as "null" sentinel
            nodes: vec![TstNode {
                keysym: 0,
                lo: 0,
                hi: 0,
                eq: 0,
                is_leaf: false,
                leaf_keysym: 0,
                leaf_utf8: String::new(),
            }],
        };

        // Parse the raw compose file to get keysym-name-based entries
        let raw_entries = parse_compose_file_raw(path);

        for entry in &raw_entries {
            // Convert keysym names to keysym values
            let mut keysyms = Vec::new();
            let mut valid = true;
            for name in &entry.keysym_names {
                let ks = xkb_keysym_from_name(name.as_bytes(), XKB_KEYSYM_NO_FLAGS);
                if ks == 0 {
                    valid = false;
                    break;
                }
                keysyms.push(ks);
            }
            if !valid || keysyms.is_empty() {
                continue;
            }

            // Resolve output keysym
            let out_ks = if let Some(ref name) = entry.output_keysym_name {
                xkb_keysym_from_name(name.as_bytes(), XKB_KEYSYM_NO_FLAGS)
            } else {
                0
            };

            // Resolve output UTF-8
            let out_utf8 = if !entry.output_string.is_empty() {
                entry.output_string.clone()
            } else if out_ks != 0 {
                let cp = keysym_to_utf32(out_ks);
                if cp != 0 {
                    char::from_u32(cp).map_or(String::new(), |c| c.to_string())
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            table.insert(&keysyms, out_ks, &out_utf8);
        }

        table
    }

    fn insert(&mut self, keysyms: &[u32], out_keysym: u32, out_utf8: &str) {
        if keysyms.is_empty() {
            return;
        }
        let cur = self.insert_path(keysyms);
        self.nodes[cur].is_leaf = true;
        self.nodes[cur].leaf_keysym = out_keysym;
        self.nodes[cur].leaf_utf8 = out_utf8.to_string();
    }

    fn insert_path(&mut self, keysyms: &[u32]) -> usize {
        let mut node_idx = 1; // root is at index 1 (0 is null sentinel)

        // Ensure root exists
        if self.nodes.len() < 2 {
            self.nodes.push(TstNode {
                keysym: keysyms[0],
                lo: 0,
                hi: 0,
                eq: 0,
                is_leaf: false,
                leaf_keysym: 0,
                leaf_utf8: String::new(),
            });
        }

        for (i, &ks) in keysyms.iter().enumerate() {
            node_idx = self.tst_insert(node_idx, ks);
            if i + 1 < keysyms.len() {
                // Move to eq child for next keysym in sequence
                if self.nodes[node_idx].eq == 0 {
                    let new_idx = self.nodes.len();
                    self.nodes.push(TstNode {
                        keysym: keysyms[i + 1],
                        lo: 0,
                        hi: 0,
                        eq: 0,
                        is_leaf: false,
                        leaf_keysym: 0,
                        leaf_utf8: String::new(),
                    });
                    self.nodes[node_idx].eq = new_idx as u32;
                }
                node_idx = self.nodes[node_idx].eq as usize;
            }
        }
        node_idx
    }

    fn tst_insert(&mut self, start: usize, ks: u32) -> usize {
        let mut idx = start;
        loop {
            let node_ks = self.nodes[idx].keysym;
            if node_ks == 0 {
                // Uninitialized node — claim it
                self.nodes[idx].keysym = ks;
                return idx;
            }
            if ks < node_ks {
                if self.nodes[idx].lo == 0 {
                    let new_idx = self.nodes.len();
                    self.nodes.push(TstNode {
                        keysym: ks,
                        lo: 0,
                        hi: 0,
                        eq: 0,
                        is_leaf: false,
                        leaf_keysym: 0,
                        leaf_utf8: String::new(),
                    });
                    self.nodes[idx].lo = new_idx as u32;
                    return new_idx;
                }
                idx = self.nodes[idx].lo as usize;
            } else if ks > node_ks {
                if self.nodes[idx].hi == 0 {
                    let new_idx = self.nodes.len();
                    self.nodes.push(TstNode {
                        keysym: ks,
                        lo: 0,
                        hi: 0,
                        eq: 0,
                        is_leaf: false,
                        leaf_keysym: 0,
                        leaf_utf8: String::new(),
                    });
                    self.nodes[idx].hi = new_idx as u32;
                    return new_idx;
                }
                idx = self.nodes[idx].hi as usize;
            } else {
                return idx; // exact match
            }
        }
    }

    /// Create a new compose state from this table.
    pub fn new_state(&self) -> ComposeState<'_> {
        ComposeState {
            table: self,
            node_idx: if self.nodes.len() > 1 { 1 } else { 0 },
            status: ComposeStatus::Nothing,
            result_keysym: 0,
            result_utf8: String::new(),
        }
    }
}

/// Compose state machine that navigates a `ComposeTable`.
pub struct ComposeState<'a> {
    table: &'a ComposeTable,
    node_idx: usize,
    status: ComposeStatus,
    result_keysym: u32,
    result_utf8: String,
}

impl<'a> ComposeState<'a> {
    /// Feed a keysym to the compose state machine.
    pub fn feed(&mut self, keysym: u32) -> ComposeFeedResult {
        // Modifier keysyms and special keysyms are ignored
        if is_modifier_or_non_printable(keysym) {
            return ComposeFeedResult::Ignored;
        }

        if self.status == ComposeStatus::Composed || self.status == ComposeStatus::Cancelled {
            // Auto-reset after composed/cancelled
            self.reset();
        }

        if self.node_idx == 0 || self.table.nodes.len() <= 1 {
            self.status = ComposeStatus::Nothing;
            return ComposeFeedResult::Ignored;
        }

        // If we're at Nothing state, start from root
        let search_from = if self.status == ComposeStatus::Nothing {
            1 // root
        } else {
            // We're composing — search_from is the eq child of current matched node
            let eq = self.table.nodes[self.node_idx].eq as usize;
            if eq == 0 {
                self.status = ComposeStatus::Cancelled;
                return ComposeFeedResult::Accepted;
            }
            eq
        };

        // Search for this keysym in the TST level
        match self.tst_lookup(search_from, keysym) {
            Some(found_idx) => {
                self.node_idx = found_idx;
                if self.table.nodes[found_idx].is_leaf {
                    self.status = ComposeStatus::Composed;
                    self.result_keysym = self.table.nodes[found_idx].leaf_keysym;
                    self.result_utf8 = self.table.nodes[found_idx].leaf_utf8.clone();
                } else {
                    self.status = ComposeStatus::Composing;
                }
                ComposeFeedResult::Accepted
            }
            None => {
                if self.status == ComposeStatus::Nothing {
                    // Not in a sequence, keysym doesn't start any sequence
                    ComposeFeedResult::Ignored
                } else {
                    self.status = ComposeStatus::Cancelled;
                    ComposeFeedResult::Accepted
                }
            }
        }
    }

    fn tst_lookup(&self, start: usize, keysym: u32) -> Option<usize> {
        let mut idx = start;
        loop {
            if idx == 0 || idx >= self.table.nodes.len() {
                return None;
            }
            let node = &self.table.nodes[idx];
            if keysym < node.keysym {
                idx = node.lo as usize;
            } else if keysym > node.keysym {
                idx = node.hi as usize;
            } else {
                return Some(idx);
            }
        }
    }

    /// Get the current compose status.
    pub fn status(&self) -> ComposeStatus {
        self.status
    }

    /// Get the composed keysym (valid when status is `Composed`).
    pub fn keysym(&self) -> u32 {
        if self.status == ComposeStatus::Composed {
            self.result_keysym
        } else {
            0 // XKB_KEY_NoSymbol
        }
    }

    /// Get the composed UTF-8 string (valid when status is `Composed`).
    pub fn utf8(&self) -> &str {
        if self.status == ComposeStatus::Composed {
            &self.result_utf8
        } else {
            ""
        }
    }

    /// Reset the compose state machine.
    pub fn reset(&mut self) {
        self.node_idx = if self.table.nodes.len() > 1 { 1 } else { 0 };
        self.status = ComposeStatus::Nothing;
        self.result_keysym = 0;
        self.result_utf8.clear();
    }
}

/// Check if a keysym is a modifier or non-printable key that should be
/// ignored by the compose state machine (matching libxkbcommon behavior).
fn is_modifier_or_non_printable(ks: u32) -> bool {
    // XKB modifier keysyms: Shift, Control, Caps_Lock, Meta, Alt, Super, Hyper
    // Range: 0xFFE1..=0xFFEE
    if (0xFFE1..=0xFFEE).contains(&ks) {
        return true;
    }
    // Misc function keys that don't produce text: 0xFF00..=0xFF1F
    // (but allow some like Return=0xFF0D, Tab=0xFF09, Escape=0xFF1B, Delete=0xFFFF, BackSpace=0xFF08)
    false
}

// ════════════════════════════════════════════════════════════════════════
// Raw compose file parser — preserves keysym names for TST construction
// ════════════════════════════════════════════════════════════════════════

struct RawComposeEntry {
    keysym_names: Vec<String>,
    output_string: String,
    output_keysym_name: Option<String>,
}

fn parse_compose_file_raw(path: &Path) -> Vec<RawComposeEntry> {
    let mut out = Vec::new();
    parse_compose_file_raw_impl(path, &mut out);
    out
}

fn parse_compose_file_raw_impl(path: &Path, out: &mut Vec<RawComposeEntry>) {
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

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
                parse_compose_file_raw_impl(&resolved, out);
            }
            continue;
        }

        if let Some(entry) = parse_raw_rule_line(trimmed) {
            out.push(entry);
        }
    }
}

fn parse_raw_rule_line(line: &str) -> Option<RawComposeEntry> {
    let colon_pos = line.find(':')?;
    let lhs = &line[..colon_pos];
    let rhs = line[colon_pos + 1..].trim();

    // Strip trailing comment
    let rhs = if let Some(hash) = rhs.find('#') {
        rhs[..hash].trim()
    } else {
        rhs
    };

    // Parse LHS: sequence of <keysym_name>
    let mut keysym_names = Vec::new();
    let mut pos = 0;
    let lhs_bytes = lhs.as_bytes();
    while pos < lhs_bytes.len() {
        if lhs_bytes[pos] == b'<' {
            let end = lhs[pos..].find('>')? + pos;
            let name = &lhs[pos + 1..end];
            keysym_names.push(name.to_string());
            pos = end + 1;
        } else {
            pos += 1;
        }
    }

    if keysym_names.is_empty() {
        return None;
    }

    // Parse RHS: "string" [keysym_name]
    let (output_string, output_keysym_name) = parse_raw_rhs(rhs);

    Some(RawComposeEntry {
        keysym_names,
        output_string,
        output_keysym_name,
    })
}

fn parse_raw_rhs(rhs: &str) -> (String, Option<String>) {
    let rhs = rhs.trim();
    if rhs.starts_with('"') {
        if let Some(end_quote) = rhs[1..].find('"') {
            let s = &rhs[1..end_quote + 1];
            // Unescape basic sequences
            let output_string = unescape_compose_string(s);
            let after = rhs[end_quote + 2..].trim();
            let keysym_name = if !after.is_empty() {
                after.split_whitespace().next().map(|s| s.to_string())
            } else {
                None
            };
            return (output_string, keysym_name);
        }
    }
    // Bare keysym name
    let name = rhs.split_whitespace().next();
    (String::new(), name.map(|s| s.to_string()))
}

fn unescape_compose_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some('"') => out.push('"'),
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('0') => { /* null — skip */ }
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}
