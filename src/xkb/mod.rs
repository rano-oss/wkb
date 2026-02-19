use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::{self, File},
    io::{self, BufRead, Read},
    os::fd::OwnedFd,
    path::Path,
};

use crate::composer::{Composer, ListComposer};
use crate::modifiers::Modifiers;
use crate::repeat::REPEAT_DEFAULT;
use crate::REPEAT_KEYS;

use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};

use crate::default_keymap::DEFAULT_MAP;
use crate::modifiers::{ModKind, ModType, BACKSPACE, CAPS_LOCK, TAB};
use crate::WKB;

pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

pub mod evdev_xkb;
pub mod xkb_compose_map;
pub mod xkb_utf8;

use evdev_xkb::XKBCODES_EVDEV;
use xkb_compose_map::XKB_COMPOSE_MAP;
use xkb_utf8::XKBCODES_DEF_TO_UTF8;

const LOCALE_DIR: &str = "/usr/share/X11/locale";

/// Temporary state used only during XKB keymap construction.
/// These fields are consumed by `build_caps_lock_table` and then discarded.
struct XkbBuildState {
    /// Per-(level, evdev_code) pairs whose keysyms originated from BMP
    /// Unicode keysyms (XKB keysym range 0x01000000–0x0100FFFF, i.e.
    /// `Uxxxx` or `0x0100xxxx` notation). xkbcommon does NOT apply caps
    /// lock post-process uppercasing to these keysyms, unlike standard
    /// named keysyms that map to the same codepoints. Tracked per-level
    /// because a single key can mix standard and Unicode keysyms across
    /// levels (e.g. `z` at level 0, `U0A01` at level 1).
    bmp_unicode_keys: HashSet<(usize, u32)>,
    /// Global default key type for this layout (from `key.type[group1] = "..."`).
    default_key_type: Option<String>,
    /// Per-key explicit type declarations (from `type[Group1] = "..."` within keys).
    key_types: HashMap<u32, String>,
}

impl XkbBuildState {
    fn new() -> Self {
        Self {
            bmp_unicode_keys: HashSet::new(),
            default_key_type: None,
            key_types: HashMap::new(),
        }
    }
}

/// Unicode simple uppercase mapping for characters where Rust's
/// `char::to_uppercase()` (full case mapping) expands to multiple
/// codepoints or doesn't map at all. xkbcommon's `xkb_keysym_to_upper`
/// uses simple mapping from UnicodeData.txt, which includes mathematical
/// alphanumeric symbols that Rust's standard library doesn't handle.
fn simple_uppercase(c: char) -> Option<char> {
    match c {
        'ß' => Some('ẞ'), // U+00DF → U+1E9E
        'ﬀ' => None,      // U+FB00: no simple uppercase
        'ﬁ' => None,      // U+FB01: no simple uppercase
        'ﬂ' => None,      // U+FB02: no simple uppercase
        'ﬃ' => None,      // U+FB03: no simple uppercase
        'ﬄ' => None,      // U+FB04: no simple uppercase
        'ﬅ' => None,      // U+FB05: no simple uppercase
        'ﬆ' => None,      // U+FB06: no simple uppercase
        _ => math_uppercase(c),
    }
}

/// Uppercase mapping for Mathematical Alphanumeric Symbols (U+1D400–U+1D7FF).
///
/// Rust's `char::to_uppercase()` doesn't handle these because they have
/// `<font>` decomposition type, but xkbcommon's `xkb_keysym_to_upper`
/// uses the Simple_Uppercase_Mapping from UnicodeData.txt which does
/// include them. Each style (bold, italic, double-struck, etc.) has
/// lowercase letters at a fixed offset from their uppercase counterparts,
/// except where pre-existing Unicode characters replace some uppercase
/// slots (e.g., ℂ U+2102 instead of 𝔺).
fn math_uppercase(c: char) -> Option<char> {
    let cp = c as u32;
    // Mathematical Bold: a-z 1D41A–1D433 → A-Z 1D400–1D419
    if (0x1D41A..=0x1D433).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Italic: a-z 1D44E–1D467 → A-Z 1D434–1D44D
    // (h at 1D455 is replaced by ℎ U+210E, but lowercase h maps to 1D43B)
    if (0x1D44E..=0x1D467).contains(&cp) {
        let upper = cp - 26;
        return char::from_u32(upper);
    }
    // Mathematical Bold Italic: a-z 1D482–1D49B → A-Z 1D468–1D481
    if (0x1D482..=0x1D49B).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Script: a-z 1D4B6–1D4CF → A-Z 1D49C–1D4B5
    // (gaps: B→ℬ U+212C, E→ℰ U+2130, F→ℱ U+2131, H→ℋ U+210B,
    //  I→ℐ U+2110, L→ℒ U+2112, M→ℳ U+2133, R→ℛ U+211B,
    //  e→ℯ U+212F, g→ℊ U+210A, o→ℴ U+2134)
    if (0x1D4B6..=0x1D4CF).contains(&cp) {
        let idx = cp - 0x1D4B6; // 0-based index (a=0..z=25)
        let upper = match idx {
            1 => 0x212C,  // b→ℬ (B is at U+212C)
            4 => 0x2130,  // e→ℰ (E is at U+2130)
            5 => 0x2131,  // f→ℱ (F is at U+2131)
            7 => 0x210B,  // h→ℋ (H is at U+210B)
            8 => 0x2110,  // i→ℐ (I is at U+2110)
            11 => 0x2112, // l→ℒ (L is at U+2112)
            12 => 0x2133, // m→ℳ (M is at U+2133)
            17 => 0x211B, // r→ℛ (R is at U+211B)
            _ => cp - 26,
        };
        return char::from_u32(upper);
    }
    // Mathematical Bold Script: a-z 1D4EA–1D503 → A-Z 1D4D0–1D4E9
    if (0x1D4EA..=0x1D503).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Fraktur: a-z 1D51E–1D537 → A-Z 1D504–1D51D
    // (gaps: C→ℭ U+212D, H→ℌ U+210C, I→ℑ U+2111, R→ℜ U+211C, Z→ℨ U+2128)
    if (0x1D51E..=0x1D537).contains(&cp) {
        let idx = cp - 0x1D51E;
        let upper = match idx {
            2 => 0x212D,  // c→ℭ
            7 => 0x210C,  // h→ℌ
            8 => 0x2111,  // i→ℑ
            17 => 0x211C, // r→ℜ
            25 => 0x2128, // z→ℨ
            _ => cp - 26,
        };
        return char::from_u32(upper);
    }
    // Mathematical Double-Struck: a-z 1D552–1D56B → A-Z 1D538–1D551
    // (gaps: C→ℂ U+2102, H→ℍ U+210D, N→ℕ U+2115, P→ℙ U+2119,
    //  Q→ℚ U+211A, R→ℝ U+211D, Z→ℤ U+2124)
    if (0x1D552..=0x1D56B).contains(&cp) {
        let idx = cp - 0x1D552;
        let upper = match idx {
            2 => 0x2102,  // c→ℂ
            7 => 0x210D,  // h→ℍ
            13 => 0x2115, // n→ℕ
            15 => 0x2119, // p→ℙ
            16 => 0x211A, // q→ℚ
            17 => 0x211D, // r→ℝ
            25 => 0x2124, // z→ℤ
            _ => cp - 26,
        };
        return char::from_u32(upper);
    }
    // Mathematical Bold Fraktur: a-z 1D586–1D59F → A-Z 1D56C–1D585
    if (0x1D586..=0x1D59F).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Sans-Serif: a-z 1D5BA–1D5D3 → A-Z 1D5A0–1D5B9
    if (0x1D5BA..=0x1D5D3).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Sans-Serif Bold: a-z 1D5EE–1D607 → A-Z 1D5D4–1D5ED
    if (0x1D5EE..=0x1D607).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Sans-Serif Italic: a-z 1D622–1D63B → A-Z 1D608–1D621
    if (0x1D622..=0x1D63B).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Sans-Serif Bold Italic: a-z 1D656–1D66F → A-Z 1D63C–1D655
    if (0x1D656..=0x1D66F).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Monospace: a-z 1D68A–1D6A3 → A-Z 1D670–1D689
    if (0x1D68A..=0x1D6A3).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Bold Greek: α-ω 1D6C2–1D6DA → Α-Ω 1D6A8–1D6C0
    // (offset 26 for 25 letters, ϑ at 1D6DD, ϰ at 1D6DE etc. are extras)
    if (0x1D6C2..=0x1D6DA).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Italic Greek: α-ω 1D6FC–1D714 → Α-Ω 1D6E2–1D6FA
    if (0x1D6FC..=0x1D714).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Bold Italic Greek: α-ω 1D736–1D74E → Α-Ω 1D71C–1D734
    if (0x1D736..=0x1D74E).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Sans-Serif Bold Greek: α-ω 1D770–1D788 → Α-Ω 1D756–1D76E
    if (0x1D770..=0x1D788).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    // Mathematical Sans-Serif Bold Italic Greek: α-ω 1D7AA–1D7C2 → Α-Ω 1D790–1D7A8
    if (0x1D7AA..=0x1D7C2).contains(&cp) {
        return char::from_u32(cp - 26);
    }
    None
}

/// Apply uppercase for caps lock post-processing.
/// Uses Unicode simple case mapping (single codepoint only), matching
/// xkbcommon's `xkb_keysym_to_upper` behavior.
fn caps_uppercase(c: char) -> char {
    // Try standard uppercase — use it if it maps to a single codepoint
    // that is different from the input.
    let mut upper_iter = c.to_uppercase();
    if let Some(upper) = upper_iter.next() {
        if upper_iter.next().is_none() && upper != c {
            return upper;
        }
    }
    // Fallback: Unicode simple case mapping for chars where Rust's full
    // case mapping expands to multiple codepoints, returns the same char
    // (e.g., mathematical symbols), or a single uppercase codepoint
    // exists (e.g., ß → ẞ, 𝕨 → 𝕎).
    simple_uppercase(c).unwrap_or(c)
}

/// Look up a locale name in `compose.dir` and return the compose file
/// sub-path (e.g. `"en_US.UTF-8/Compose"`).  Returns `None` when no
/// entry matches.
fn lookup_compose_dir(locale: &str) -> Option<String> {
    let compose_dir_path = Path::new(LOCALE_DIR).join("compose.dir");
    let file = fs::File::open(compose_dir_path).ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == locale {
            return Some(parts[0].to_string());
        }
    }
    None
}

/// Resolve a short or partial locale name through
/// `/usr/share/X11/locale/locale.alias`.
///
/// For example `"de"` → `"de_DE.ISO8859-1"`, `"ru"` → `"ru_RU.UTF-8"`.
fn resolve_locale_alias(locale: &str) -> Option<String> {
    let alias_path = Path::new(LOCALE_DIR).join("locale.alias");
    let file = fs::File::open(alias_path).ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == locale {
            return Some(parts[1].to_string());
        }
    }
    None
}

/// Load a compose table from a locale name by resolving it through
/// `locale.alias` and `compose.dir`.
///
/// Resolution order:
/// 1. Exact match in `compose.dir` (handles full locale names like
///    `"en_US.UTF-8"`).
/// 2. Resolve through `locale.alias` (handles short names like `"de"`
///    → `"de_DE.ISO8859-1"`).  When the alias is a non-UTF-8 locale,
///    **prefer the UTF-8 sibling** (e.g. `"de_DE.UTF-8"`) over the
///    legacy locale so that modern compose tables are used.
/// 3. Construct `"{lang}_{LANG}.UTF-8"` from a short code and try
///    that in `compose.dir` (covers locales where locale.alias points
///    nowhere useful).
fn load_compose_table(locale: &str) -> Option<(ListComposer, ListComposer)> {
    let compose_file_path = resolve_compose_file(locale)?;
    let full_path = Path::new(LOCALE_DIR).join(&compose_file_path);
    Some(load_compose_from_path(&full_path))
}

/// Resolve a locale name to the compose file sub-path (relative to
/// `/usr/share/X11/locale/`) that should be used.
///
/// Returns e.g. `Some("en_US.UTF-8/Compose")` for locale `"de"`.
/// Falls back to `"en_US.UTF-8"` when no locale-specific compose file
/// can be found, matching xkbcommon's behaviour.  Returns `None` only
/// if even the `en_US.UTF-8` fallback is missing (broken system).
///
/// This is the same resolution logic used by [`load_compose_table`],
/// exposed publicly so tests can verify which compose file a given
/// locale maps to.
pub fn resolve_compose_file(locale: &str) -> Option<String> {
    // 0. Hardcoded XKB layout name → compose.dir locale mapping
    //    (covers layout names that locale.alias cannot resolve)
    if let Some(&mapped_locale) = XKB_COMPOSE_MAP.get(locale) {
        if let Some(compose_file) = lookup_compose_dir(mapped_locale) {
            return Some(compose_file);
        }
    }

    // 1. Direct match
    if let Some(compose_file) = lookup_compose_dir(locale) {
        return Some(compose_file);
    }

    // 2. Resolve through locale.alias
    if let Some(resolved) = resolve_locale_alias(locale) {
        // 2a. If the alias resolved to a non-UTF-8 locale, try the
        //     UTF-8 variant first so we get modern compose tables.
        //     e.g. "de_DE.ISO8859-1" → "de_DE.UTF-8"
        if let Some(dot_pos) = resolved.find('.') {
            let base = &resolved[..dot_pos];
            if !resolved[dot_pos..].eq_ignore_ascii_case(".UTF-8") {
                let utf8_locale = format!("{}.UTF-8", base);
                if let Some(compose_file) = lookup_compose_dir(&utf8_locale) {
                    return Some(compose_file);
                }
            }
        }

        // 2b. Fall back to the alias locale itself
        if let Some(compose_file) = lookup_compose_dir(&resolved) {
            return Some(compose_file);
        }
    }

    // 3. Heuristic: construct "{lang}_{LANG}.UTF-8" from a 2-3 letter code
    //    e.g. "de" → "de_DE.UTF-8", "fr" → "fr_FR.UTF-8"
    if locale.len() >= 2 && locale.len() <= 3 && locale.chars().all(|c| c.is_ascii_lowercase()) {
        let upper = locale.to_ascii_uppercase();
        let candidate = format!("{}_{}.UTF-8", locale, upper);
        if let Some(compose_file) = lookup_compose_dir(&candidate) {
            return Some(compose_file);
        }
    }

    // 4. Final fallback: use en_US.UTF-8, matching xkbcommon behavior.
    //    xkbcommon falls back to en_US.UTF-8 for any valid locale that
    //    has no compose.dir entry, so we do the same rather than
    //    maintaining a hardcoded list of fallback entries.
    lookup_compose_dir("en_US.UTF-8")
}

/// Resolve a keysym name to a character.
///
/// Checks [`XKBCODES_DEF_TO_UTF8`] first, then handles single-character
/// names (e.g. `a`, `Z`), and finally `Uxxxx` Unicode codepoint names
/// (e.g. `U0226` → U+0226 Ȧ).
fn resolve_keysym_char(name: &str) -> Option<char> {
    if let Some(&c) = XKBCODES_DEF_TO_UTF8.get(name) {
        return Some(c);
    }
    if name.len() == 1 {
        return name.chars().next();
    }
    // Handle Uxxxx / Uxxxxx / Uxxxxxx Unicode codepoint keysyms
    if name.starts_with('U') && name.len() >= 5 && name.len() <= 7 {
        let hex = &name[1..];
        if hex.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Ok(cp) = u32::from_str_radix(hex, 16) {
                return char::from_u32(cp);
            }
        }
    }
    None
}

/// Load a compose file from the given path into a pair of
/// [`ListComposer`]s (regular, compose-key).
///
/// Handles `include` directives recursively, skips entries where
/// `Multi_key` appears in a non-first position, and falls back to
/// resolving the keysym name after the quoted output string via
/// [`XKBCODES_DEF_TO_UTF8`] when the quoted string contains an
/// octal/hex escape or is empty.
pub fn load_compose_from_path(path: &Path) -> (ListComposer, ListComposer) {
    let mut regular = ListComposer::new();
    let mut compose_key = ListComposer::new();
    parse_compose_into(path, &mut regular, &mut compose_key);
    fix_compose_trie(&mut regular);
    fix_compose_trie(&mut compose_key);
    (regular, compose_key)
}

/// Post-process a compose trie to fix known issues that arise from
/// flattening keysym-based compose entries into char-based entries.
///
/// 1. **Prefix conflicts**: when a shorter sequence (e.g. `U, comma`)
///    has an emit value but also has children leading to longer
///    sequences (e.g. `U, comma, E`), remove the shorter entry so
///    that `feed()` keeps composing toward the longer match.  This
///    matches xkbcommon's behaviour.
///
/// 2. **Keysym alias collision**: the keysyms `U223C` and `approximate`
///    both resolve to char `∼` (U+223C).  The compose file maps
///    `<Multi_key> <U223C> <slash>` → `≁` and
///    `<Multi_key> <approximate> <slash>` → `≇`.  With last-wins
///    insert semantics `≇` wins, but xkbcommon keeps them as separate
///    keysym sequences and produces `≁` for the `U223C` keysym.
///    Re-insert the xkbcommon-preferred entry so it wins.
fn fix_compose_trie(composer: &mut ListComposer) {
    // Fix 1: remove emit on entries that are prefixes of longer
    // sequences (i.e. the node has children).  Walk every node: if it
    // has both emit and children, clear emit.
    // We need to use indices because we can't borrow mutably while
    // iterating, but the node vec length is fixed after parsing.
    for i in 0..composer.nodes.len() {
        if !composer.nodes[i].next.is_empty() && composer.nodes[i].emit.is_some() {
            composer.nodes[i].emit = None;
        }
    }

    // Fix 2: keysym alias collision — force the xkbcommon-preferred
    // value for `∼ /` → `≁` (U+2241 NOT TILDE).
    composer.insert(&['∼', '/'], '≁');
}

/// Recursively parse a compose file into the given composers.
fn parse_compose_into(path: &Path, regular: &mut ListComposer, compose_key: &mut ListComposer) {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Handle include directives recursively
        if trimmed.starts_with("include") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let include_path = &trimmed[start + 1..start + 1 + end];
                    parse_compose_into(Path::new(include_path), regular, compose_key);
                }
            }
            continue;
        }

        if !trimmed.starts_with('<') {
            continue;
        }

        // Parse: <key1> <key2> ... : "output" keysym_name # comment
        let (keys_part, value_part) = match trimmed.split_once(':') {
            Some(pair) => pair,
            None => continue,
        };

        let keys_str = keys_part.trim();
        let value_str = value_part.trim();

        let mut keys: Vec<char> = Vec::new();
        let mut is_multi_key = false;
        let mut skip = false;

        for token in keys_str.split_whitespace() {
            let name = token.trim_start_matches('<').trim_end_matches('>');
            if name == "Multi_key" {
                if keys.is_empty() {
                    is_multi_key = true;
                } else {
                    // Multi_key in a non-first position creates a hybrid
                    // sequence we cannot represent.  Skip the entry.
                    skip = true;
                    break;
                }
                continue;
            }
            if let Some(c) = resolve_keysym_char(name) {
                keys.push(c);
            } else {
                // Unresolvable keysym — skip entry
                keys.clear();
                break;
            }
        }

        if skip || keys.is_empty() {
            continue;
        }

        // Parse output: try quoted string first, fall back to keysym name
        let out_char = parse_compose_output(value_str);

        if let Some(out) = out_char {
            if is_multi_key {
                compose_key.insert(&keys, out);
            } else {
                regular.insert(&keys, out);
            }
        }
    }
}

/// Extract the output character from the value part of a compose line.
///
/// Handles formats like:
///   `"á"  aacute # LATIN SMALL LETTER A WITH ACUTE`
///   `"\305"  Aring`
///   `"á"`
///
/// Prefers the character from the quoted string when it is plain UTF-8.
/// Falls back to resolving the keysym name (the token after the closing
/// quote) via [`XKBCODES_DEF_TO_UTF8`].
fn parse_compose_output(value_str: &str) -> Option<char> {
    if !value_str.starts_with('"') {
        // No quoted string — try the whole value as a keysym name
        let name = value_str.trim();
        return resolve_keysym_char(name);
    }

    let rest = &value_str[1..];
    let end_quote = rest.find('"')?;
    let inner = &rest[..end_quote];

    // Accept the quoted character if it is NOT an octal/hex escape
    if !inner.is_empty() && !inner.starts_with('\\') {
        if let Some(c) = inner.chars().next() {
            return Some(c);
        }
    }

    // Fall back to the keysym name after the closing quote
    let after_quote = rest[end_quote + 1..].trim();
    if after_quote.is_empty() || after_quote.starts_with('#') {
        return None;
    }
    let keysym_name = after_quote.split_whitespace().next()?;
    if keysym_name.starts_with('#') {
        return None;
    }
    resolve_keysym_char(keysym_name)
}

fn read_layouts(path: &Path, locale: Option<String>, fd: Option<OwnedFd>) -> Vec<String> {
    let mut string_file = String::new();
    if let Some(fd) = fd {
        let mut file = File::from(fd);
        let _ = file.read_to_string(&mut string_file);
    } else if let Some(locale) = locale {
        if let Ok(content) = std::fs::read_to_string(&path.join(locale)) {
            string_file = content;
        }
    }
    let mut layouts = Vec::new();
    if let Ok(xkb) = parse(&string_file) {
        xkb.definitions.iter().for_each(|d| match &d.directive {
            Directive::XkbSymbols(src) => {
                let name = src.name.content.to_string();
                if ![
                    "sun_type6",
                    "sun_type6_de",
                    "sun_type6_fr",
                    "sun_type6_suncompat",
                    "sun_type7_suncompat",
                    "suncompat",
                    "sun_type7",
                ]
                .contains(&name.as_str())
                {
                    layouts.push(src.name.content.to_string());
                }
            }
            _ => {}
        });
    } else {
        let re = Regex::new(r#"xkb_symbols\s+"([\w\s_\-\d]+)"\s+\{"#).unwrap();
        for cap in re.captures_iter(&string_file) {
            if let Some(name) = cap.get(1) {
                let name = name.as_str();
                if ![
                    "sun_type6",
                    "sun_type6_de",
                    "sun_type6_fr",
                    "sun_type6_suncompat",
                    "sun_type7_suncompat",
                    "suncompat",
                    "sun_type7",
                ]
                .contains(&name)
                {
                    layouts.push(name.to_string());
                }
            }
        }
    }
    layouts
}

fn parse_include(input: &str) -> (String, Option<String>) {
    let re = Regex::new(r"([\w]+)(?:\(([\w\-]+)\))?$").unwrap();
    let capture = re.captures(input).unwrap();
    (
        capture.get(1).map(|m| m.as_str().to_string()).unwrap(),
        capture.get(2).map(|m| m.as_str().to_string()),
    )
}

fn unicode_string_to_unicode_char(s: &str) -> Option<char> {
    let number = &s[1..];

    u32::from_str_radix(number, 16)
        .ok()
        .and_then(std::char::from_u32)
}

/// Check whether a `Uxxxx` keysym string represents a Latin-1 Unicode keysym
/// that xkbcommon would NOT normalise to a named keysym.
///
/// In practice this never returns `true`: xkbcommon normalises `Uxxxx` with
/// codepoint ≤ 0xFF to the named keysym (which IS uppercased), and codepoints
/// > 0xFF become Unicode keysyms where `xkb_keysym_to_upper` works correctly.
/// So `Uxxxx` notation never produces a "non-uppercasable" keysym.
fn is_latin1_unicode_keysym_u(_s: &str) -> bool {
    false
}

fn hex_string_to_unicode_char(s: &str) -> Option<char> {
    let split_pos = s.char_indices().nth_back(4).unwrap().0;
    let number = &s[split_pos..];

    u32::from_str_radix(number, 16)
        .ok()
        .and_then(std::char::from_u32)
}

/// Check whether a `0x...` hex keysym string represents a Latin-1 Unicode
/// keysym (keysym value in range 0x01000000–0x010000FF).
///
/// xkbcommon's `xkb_keysym_to_upper` returns these keysyms **unchanged**
/// because `XConvertCase` for Unicode keysyms in the Latin-1 codepoint range
/// (≤ 0xFF) does not perform case conversion. Codepoints > 0xFF are handled
/// correctly by `xkb_keysym_to_upper`, so only the Latin-1 range needs to be
/// excluded from post-process uppercasing.
fn is_latin1_unicode_keysym_hex(s: &str) -> bool {
    if let Ok(keysym) = u32::from_str_radix(&s[2..], 16) {
        (0x01000000..=0x010000FF).contains(&keysym)
    } else {
        false
    }
}

pub fn new_from_names(locale: String, layout: Option<String>) -> WKB<ListComposer> {
    let path = Path::new(XKB_SYMBOLS_PATH);
    let layouts = read_layouts(path, Some(locale.clone()), None);
    let layout = if let Some(layout) = layout {
        layout
    } else {
        layouts[0].clone()
    };
    let (regular_composer, compose_key_composer) =
        load_compose_table(&locale).unwrap_or_else(|| (ListComposer::new(), ListComposer::new()));

    let mut wkb = new_wkb(
        layouts,
        layout.clone(),
        Some(locale.clone()),
        regular_composer,
        compose_key_composer,
    );
    let mut bs = XkbBuildState::new();
    map_xkb(
        &mut wkb,
        &mut bs,
        path,
        locale.clone(),
        Some(layout.clone()),
    );
    fix_xkb_edge_cases(&mut wkb, locale, Some(layout));
    wkb.caps_lock_table = build_caps_lock_table(
        &wkb.level_keymap,
        &bs.key_types,
        &bs.default_key_type,
        &bs.bmp_unicode_keys,
    );
    wkb
}

pub fn new_from_string(string: String) -> WKB<ListComposer> {
    let mut wkb = new_wkb(
        Vec::new(),
        "".to_string(),
        None,
        ListComposer::new(),
        ListComposer::new(),
    );
    let mut bs = XkbBuildState::new();
    map_xkb_from_str(
        &mut wkb,
        &mut bs,
        &string,
        Path::new(XKB_SYMBOLS_PATH),
        None,
        None,
    );
    wkb.caps_lock_table = build_caps_lock_table(
        &wkb.level_keymap,
        &bs.key_types,
        &bs.default_key_type,
        &bs.bmp_unicode_keys,
    );
    wkb
}

/// How caps lock affects a key, derived from its XKB key type.
enum CapsClass {
    /// Lock toggles the level2 bit at all levels (ALPHABETIC types).
    /// Levels 0↔1, 2↔3, 4↔5, 6↔7.
    Alphabetic,
    /// Lock toggles at levels 0-1, post-process uppercase at levels 2+
    /// (SEMIALPHABETIC types).
    SemiAlphabetic,
    /// Lock not consumed — post-process uppercase at all levels
    /// (TWO_LEVEL, FOUR_LEVEL, ONE_LEVEL, etc.).
    PostProcess,
    /// Lock redirects level 0 to a specific target level; other levels
    /// are unaffected (FOUR_LEVEL_PLUS_LOCK).
    PlusLock(usize),
    /// Lock redirects level 0 to level 3 (with post-process), shift+lock
    /// to level 0 (SEPARATE_CAPS_AND_SHIFT_ALPHABETIC).
    SeparateCapsShift,
}

/// Determine the caps lock behavior class for a key type name.
fn key_type_caps_class(type_name: &str) -> CapsClass {
    match type_name {
        // Fully alphabetic: Lock toggles level at all modifier combinations
        "ALPHABETIC"
        | "FOUR_LEVEL_ALPHABETIC"
        | "EIGHT_LEVEL_ALPHABETIC"
        | "EIGHT_LEVEL_ALPHABETIC_WITH_LEVEL5_LOCK"
        | "EIGHT_LEVEL_BY_CTRL" => CapsClass::Alphabetic,

        // Semi-alphabetic: Lock toggles at base levels, preserved at level3+
        "FOUR_LEVEL_SEMIALPHABETIC" | "EIGHT_LEVEL_SEMIALPHABETIC" => CapsClass::SemiAlphabetic,

        // Lock → Level5 (index 4); other levels unaffected
        "FOUR_LEVEL_PLUS_LOCK" => CapsClass::PlusLock(4),

        // Lock → Level4 (index 3) + post-process; Shift+Lock → Level1 (index 0)
        "SEPARATE_CAPS_AND_SHIFT_ALPHABETIC" => CapsClass::SeparateCapsShift,

        // Everything else: Lock not in modifiers, no level toggle
        _ => CapsClass::PostProcess,
    }
}

/// Build the caps lock table from level_keymap and key type information.
///
/// For each key at each level, determines what char caps lock should produce:
///   - **Alphabetic keys**: toggle the level2 bit (0↔1, 2↔3, …)
///   - **Semialphabetic keys**: toggle at levels 0-1, uppercase at levels 2+
///   - **PlusLock keys**: redirect level 0 to a target level (e.g. level 4)
///   - **SeparateCapsShift keys**: redirect level 0→3, level 1→0
///   - **Non-alphabetic keys**: uppercase post-processing
///   - **No type info**: auto-detect from symbols (level0/level1 case pair)
///
/// `bmp_unicode_keys` tracks (level, evdev_code) entries whose keysyms
/// originated from Latin-1 Unicode keysyms (0x01000000–0x010000FF) specified
/// via `0x...` hex notation.  xkbcommon's `xkb_keysym_to_upper` returns these
/// unchanged, so they must be excluded from both case-pair detection and
/// post-process uppercasing.
fn build_caps_lock_table(
    level_keymap: &[BTreeMap<u32, char>],
    key_types: &HashMap<u32, String>,
    _default_key_type: &Option<String>,
    bmp_unicode_keys: &HashSet<(usize, u32)>,
) -> Vec<BTreeMap<u32, char>> {
    if level_keymap.len() < 2 {
        return Vec::new();
    }

    let num_levels = level_keymap.len();
    let mut table: Vec<BTreeMap<u32, char>> = vec![BTreeMap::new(); num_levels];

    // Collect all evdev codes that appear in any level
    let mut all_codes: HashSet<u32> = HashSet::new();
    for level in level_keymap {
        all_codes.extend(level.keys());
    }

    // Propagate bmp_unicode_keys: if (level_X, key) is flagged and another
    // level_Y has the same char value for the same key, treat level_Y as
    // also non-uppercasable.  This handles keys defined with fewer keysyms
    // than the layout has levels (e.g. ONE_LEVEL keys whose value leaks
    // into higher levels via default maps).
    let mut effective_bmp = bmp_unicode_keys.clone();
    for &evdev_code in &all_codes {
        // Collect chars from flagged levels for this key
        let mut flagged_chars: HashSet<char> = HashSet::new();
        for level in 0..num_levels {
            if bmp_unicode_keys.contains(&(level, evdev_code)) {
                if let Some(&c) = level_keymap[level].get(&evdev_code) {
                    flagged_chars.insert(c);
                }
            }
        }
        if !flagged_chars.is_empty() {
            for level in 0..num_levels {
                if !effective_bmp.contains(&(level, evdev_code)) {
                    if let Some(&c) = level_keymap[level].get(&evdev_code) {
                        if flagged_chars.contains(&c) {
                            effective_bmp.insert((level, evdev_code));
                        }
                    }
                }
            }
        }
    }

    for &evdev_code in &all_codes {
        // Use only per-key types (already includes the scoped default_key_type
        // applied during parsing). Keys without an entry fall back to auto-detect.
        let caps_class = match key_types.get(&evdev_code) {
            Some(name) => key_type_caps_class(name),
            None => {
                // Auto-detect: match xkbcommon's default type inference.
                // If level0 is lowercase and level1 is uppercase, it's a case pair.
                // For 2-level keys → ALPHABETIC, for 4+ level keys → SEMIALPHABETIC.
                if is_case_pair(level_keymap, evdev_code, &effective_bmp) {
                    if num_levels <= 2 {
                        CapsClass::Alphabetic
                    } else {
                        CapsClass::SemiAlphabetic
                    }
                } else {
                    CapsClass::PostProcess
                }
            }
        };

        match caps_class {
            CapsClass::Alphabetic => {
                // Lock toggles the level2 bit at all levels
                // level0 ↔ level1, level2 ↔ level3, level4 ↔ level5, level6 ↔ level7
                for pair_base in (0..num_levels).step_by(2) {
                    let pair_shift = pair_base + 1;
                    if pair_shift >= num_levels {
                        break;
                    }
                    let base_char = level_keymap[pair_base].get(&evdev_code).copied();
                    let shift_char = level_keymap[pair_shift].get(&evdev_code).copied();
                    match (base_char, shift_char) {
                        (Some(bc), Some(sc)) if bc != sc => {
                            table[pair_base].insert(evdev_code, sc);
                            table[pair_shift].insert(evdev_code, bc);
                        }
                        // One level has a char but the other doesn't (e.g.
                        // modifier at base, printable at shift): still toggle
                        // so that caps lock at the missing level produces the
                        // existing level's char.
                        (None, Some(sc)) => {
                            table[pair_base].insert(evdev_code, sc);
                        }
                        (Some(bc), None) => {
                            table[pair_shift].insert(evdev_code, bc);
                        }
                        _ => {}
                    }
                }
            }
            CapsClass::SemiAlphabetic => {
                // Toggle at levels 0-1
                let base_char = level_keymap[0].get(&evdev_code).copied();
                let shift_char = level_keymap[1].get(&evdev_code).copied();
                match (base_char, shift_char) {
                    (Some(bc), Some(sc)) if bc != sc => {
                        table[0].insert(evdev_code, sc);
                        table[1].insert(evdev_code, bc);
                    }
                    (None, Some(sc)) => {
                        table[0].insert(evdev_code, sc);
                    }
                    (Some(bc), None) => {
                        table[1].insert(evdev_code, bc);
                    }
                    _ => {}
                }
                // Post-process uppercase at levels 2+
                // Skip individual (level, key) entries from BMP Unicode keysyms.
                for level in 2..num_levels {
                    if effective_bmp.contains(&(level, evdev_code)) {
                        continue;
                    }
                    if let Some(&c) = level_keymap[level].get(&evdev_code) {
                        let upper = caps_uppercase(c);
                        if upper != c {
                            table[level].insert(evdev_code, upper);
                        }
                    }
                }
            }
            CapsClass::PlusLock(target_level) => {
                // Lock redirects level 0 to target_level; other levels unaffected
                if target_level < num_levels {
                    if let Some(&tc) = level_keymap[target_level].get(&evdev_code) {
                        let base_char = level_keymap[0].get(&evdev_code).copied();
                        if base_char != Some(tc) {
                            table[0].insert(evdev_code, tc);
                        }
                    }
                }
            }
            CapsClass::SeparateCapsShift => {
                // Lock at level 0 → level 3 (index 3), with post-process
                if num_levels > 3 {
                    if let Some(&tc) = level_keymap[3].get(&evdev_code) {
                        let upper = caps_uppercase(tc);
                        let base_char = level_keymap[0].get(&evdev_code).copied();
                        if base_char != Some(upper) {
                            table[0].insert(evdev_code, upper);
                        }
                    }
                }
                // Shift+Lock → level 0 (base)
                if let Some(&bc) = level_keymap[0].get(&evdev_code) {
                    let shift_char = level_keymap[1].get(&evdev_code).copied();
                    if shift_char != Some(bc) {
                        table[1].insert(evdev_code, bc);
                    }
                }
            }
            CapsClass::PostProcess => {
                // Lock not consumed, apply uppercase post-processing at all levels.
                // Skip individual (level, key) entries whose keysyms originated
                // from Latin-1 Unicode keysyms (0x01000000–0x010000FF via hex
                // notation): xkbcommon's xkb_keysym_to_upper returns these
                // unchanged, so post-process uppercasing has no effect.
                for level in 0..num_levels {
                    if effective_bmp.contains(&(level, evdev_code)) {
                        continue;
                    }
                    if let Some(&c) = level_keymap[level].get(&evdev_code) {
                        let upper = caps_uppercase(c);
                        if upper != c {
                            table[level].insert(evdev_code, upper);
                        }
                    }
                }
            }
        }
    }

    // Trim trailing empty levels
    while table.last().is_some_and(|m| m.is_empty()) {
        table.pop();
    }

    table
}

/// Check if a key's level0 and level1 form a lowercase/uppercase pair.
///
/// Uses Unicode `is_lowercase()` / `is_uppercase()` properties rather than
/// `to_uppercase()` mapping, matching xkbcommon's `xkb_keysym_is_lower` /
/// `xkb_keysym_is_upper` heuristic. This correctly handles locale-specific
/// case pairs like Turkish `i` / `İ` where `'i'.to_uppercase()` gives `'I'`,
/// not `'İ'`.
///
/// Keys whose keysyms originated from Latin-1 Unicode keysyms
/// (0x01000000–0x010000FF, specified via `0x...` hex notation) are never
/// considered case pairs, because xkbcommon's `xkb_keysym_to_upper` /
/// `xkb_keysym_to_lower` return these keysyms unchanged, so they are
/// never detected as lowercase or uppercase during type inference.
/// Unicode keysyms for codepoints > 0xFF are handled correctly by
/// `xkb_keysym_to_upper` and CAN form case pairs.
fn is_case_pair(
    level_keymap: &[BTreeMap<u32, char>],
    evdev_code: u32,
    bmp_unicode_keys: &HashSet<(usize, u32)>,
) -> bool {
    // If either level 0 or level 1 keysym is a Latin-1 Unicode keysym
    // (0x01000000–0x010000FF from hex notation), xkbcommon's
    // xkb_keysym_to_upper/to_lower returns it unchanged → not a case pair.
    if bmp_unicode_keys.contains(&(0, evdev_code)) || bmp_unicode_keys.contains(&(1, evdev_code)) {
        return false;
    }
    if level_keymap.len() < 2 {
        return false;
    }
    let base_char = level_keymap[0].get(&evdev_code).copied();
    let shift_char = level_keymap[1].get(&evdev_code).copied();
    if let (Some(bc), Some(sc)) = (base_char, shift_char) {
        if bc == sc {
            return false;
        }
        // Match xkbcommon 1.11+'s type inference:
        //   xkb_keysym_is_lower  → checks Unicode General_Category Ll
        //   xkb_keysym_is_upper_or_title → checks Unicode General_Category Lu/Lt
        //
        // For bc_is_lower we use caps_uppercase(bc) != bc, which covers
        // both standard to_uppercase and the math_uppercase / simple_uppercase
        // fallback tables for SMP mathematical symbols.
        //
        // For sc_is_upper we first try to_lowercase (mapping-based, works for
        // BMP characters and matches older xkbcommon behaviour), then fall back
        // to the Unicode Lu property via char::is_uppercase() for SMP
        // characters where Rust's to_lowercase() returns the input unchanged.
        let bc_is_lower = caps_uppercase(bc) != bc;
        let sc_is_upper = {
            let mut lower_iter = sc.to_lowercase();
            if let Some(lower) = lower_iter.next() {
                // Don't require single-char mapping: İ → i+\u{307} is
                // still "upper" (xkbcommon uses simple lowercase mapping).
                if lower != sc {
                    true
                } else {
                    // to_lowercase returned the char unchanged — this happens
                    // for SMP characters (e.g. mathematical symbols) where
                    // Rust doesn't implement the mapping.  Fall back to the
                    // Unicode General_Category Lu property, matching
                    // xkbcommon 1.11+'s xkb_keysym_is_upper_or_title.
                    sc.is_uppercase()
                }
            } else {
                false
            }
        };
        if bc_is_lower && sc_is_upper {
            return true;
        }
    }
    false
}

fn new_wkb(
    layouts: Vec<String>,
    layout: String,
    locale: Option<String>,
    regular_composer: ListComposer,
    compose_key_composer: ListComposer,
) -> WKB<ListComposer> {
    let repeat_keys = if let Some(locale) = &locale {
        if let Some(locale_map) = REPEAT_KEYS.get(locale.as_str()) {
            if let Some(layout_set) = locale_map.get(layout.as_str()) {
                layout_set.clone()
            } else {
                REPEAT_DEFAULT.clone()
            }
        } else {
            REPEAT_DEFAULT.clone()
        }
    } else {
        REPEAT_DEFAULT.clone()
    };
    WKB {
        layouts,
        layout,
        locale,
        regular_composer,
        compose_key_composer,
        level_keymap: Vec::with_capacity(8),
        pressed_keys: HashSet::new(),
        repeat_keys,
        modifiers: Modifiers::default(),
        num_lock_keys: vec![
            71, 72, 73, // 7, 8, 9
            75, 76, 77, // 4, 5, 6
            79, 80, 81, // 1, 2, 3
            82, 83, // 0, .
        ],
        remap: HashMap::new(),
        caps_lock_table: Vec::new(),
        right_left_shift_caps: false,
    }
}

fn map_xkb<C: Composer>(
    wkb: &mut WKB<C>,
    bs: &mut XkbBuildState,
    path: &Path,
    locale: String,
    layout: Option<String>,
) {
    let Ok(file) = std::fs::read_to_string(&path.join(locale.clone())) else {
        return;
    };
    map_xkb_from_str(wkb, bs, &file, path, Some(locale), layout);
}

fn map_xkb_from_str<C: Composer>(
    wkb: &mut WKB<C>,
    bs: &mut XkbBuildState,
    file: &str,
    path: &Path,
    locale: Option<String>,
    layout: Option<String>,
) {
    let file = file.replace("symbols=[", "[");
    let Ok(xkb) = parse(&file) else {
        return;
    };
    xkb.definitions.iter().for_each(|d| {
        if let Directive::XkbSymbols(src) = &d.directive {
            let layout = layout.clone().unwrap_or(wkb.current_layout());
            if src.name.content == layout {
                src.value.iter().for_each(|si| {
                    if let XkbSymbolsItem::Include(Include { name }) = si {
                        let (locale, layout) = parse_include(name);
                        // Save/restore default_key_type around includes so that
                        // key.type from an included section doesn't leak into
                        // the including section (and vice versa).
                        let saved_type = bs.default_key_type.take();
                        if layout.is_none() {
                            map_xkb(wkb, bs, path, locale, Some("basic".to_string()));
                        } else {
                            map_xkb(wkb, bs, path, locale, layout);
                        }
                        bs.default_key_type = saved_type;
                    } else if let XkbSymbolsItem::KeyType(kt) = si {
                        // Global default key type: key.type[group1] = "..."
                        bs.default_key_type = Some(kt.name.content.to_string());
                    } else if let XkbSymbolsItem::Key(Key {
                        mode: _,
                        id,
                        values,
                    }) = si
                    {
                        if let Some(evdev_code) = XKBCODES_EVDEV.get(id.content) {
                            // Track whether this key definition contains an
                            // explicit per-key type (type[Group1] = "...").
                            let mut has_explicit_type = false;
                            values.iter().for_each(|v| {
                                if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
                                    // Per-key type declaration: type[Group1] = "..."
                                    if let xkb_parser::ast::KeyDef::TypeDef(td) = key_defs {
                                        bs.key_types
                                            .insert(*evdev_code, td.content.content.to_string());
                                        has_explicit_type = true;
                                    }
                                    if let xkb_parser::ast::KeyDef::SymbolDef(key) = key_defs {
                                        for (i, v) in key.values.values.iter().enumerate() {
                                            if i == wkb.level_keymap.len() {
                                                if i < DEFAULT_MAP.len() {
                                                    wkb.level_keymap.push(DEFAULT_MAP[i].clone());
                                                } else {
                                                    wkb.level_keymap.push(BTreeMap::new());
                                                }
                                            }
                                            let single_char =
                                                XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned();
                                            if let Some(char) = single_char {
                                                // Named keysym via KeyDefs path — not a
                                                // BMP Unicode keysym; clear any stale
                                                // entry from an earlier include.
                                                bs.bmp_unicode_keys.remove(&(i, *evdev_code));
                                                wkb.level_keymap[i].insert(*evdev_code, char);
                                            }
                                        }
                                    }
                                } else if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                    map_keys_and_modifiers(
                                        wkb,
                                        bs,
                                        key,
                                        evdev_code,
                                        layout.clone(),
                                        locale.clone().unwrap_or_default(),
                                        id.content.to_owned(),
                                    );
                                }
                            });
                            // After all values for this key are processed,
                            // apply current default_key_type if this key
                            // definition didn't include an explicit per-key
                            // type. Always override any stale entry from a
                            // previous include, since the key is being
                            // (re)defined in the current section.
                            if !has_explicit_type {
                                if let Some(ref dt) = bs.default_key_type {
                                    bs.key_types.insert(*evdev_code, dt.clone());
                                }
                            }
                        }
                    }
                })
            }
        }
    });
}

fn map_keys_and_modifiers<C: Composer>(
    wkb: &mut WKB<C>,
    bs: &mut XkbBuildState,
    key: &xkb_parser::ast::KeyNames,
    evdev_code: &u32,
    layout: String,
    locale: String,
    id: String,
) {
    if id == "CAPS" {
        if key.values.first().is_some_and(|k| k.content == "BackSpace") {
            wkb.remap.insert(*evdev_code, BACKSPACE);
            let value = *wkb.level_keymap[0].get(&BACKSPACE).unwrap();
            wkb.level_keymap[1].insert(CAPS_LOCK, value);
            // Neutralize CAPS_LOCK modifier so is_caps_lock_modifier() returns false
            wkb.modifiers
                .0
                .insert(CAPS_LOCK, crate::modifiers::Modifier::Single(ModKind::None));
        } else if key.values.first().is_some_and(|k| k.content == "Tab") {
            wkb.remap.insert(*evdev_code, TAB);
            wkb.modifiers
                .0
                .insert(CAPS_LOCK, crate::modifiers::Modifier::Single(ModKind::None));
        }
    }
    for (i, v) in key.values.iter().enumerate() {
        if i == wkb.level_keymap.len() {
            if i < DEFAULT_MAP.len() {
                wkb.level_keymap.push(DEFAULT_MAP[i].clone());
            } else {
                wkb.level_keymap.push(BTreeMap::new());
            }
        }
        if let Some(remap_key_code) = XKBCODES_EVDEV.get(v.content) {
            wkb.remap.insert(*evdev_code, *remap_key_code);
        }
        let mut chars = v.chars();
        let count = chars.clone().count();
        let first_char = chars.next();
        let is_hex = chars.all(|c| c.is_ascii_hexdigit());
        let mut chars = v.chars();
        chars.next();
        let second_char = chars.next();
        let single_char = if count == 1 && first_char.is_some_and(|c| c.is_alphanumeric()) {
            // Standard single-char keysym (e.g. 'a') — not a BMP Unicode keysym.
            bs.bmp_unicode_keys.remove(&(i, *evdev_code));
            first_char
        } else if first_char.is_some_and(|c| c == 'U') && is_hex {
            // Uxxxx notation: xkbcommon normalises codepoints ≤ 0xFF to named
            // keysyms (uppercasable) and > 0xFF to Unicode keysyms where
            // xkb_keysym_to_upper works correctly — so never flag these.
            if is_latin1_unicode_keysym_u(v) {
                bs.bmp_unicode_keys.insert((i, *evdev_code));
            } else {
                bs.bmp_unicode_keys.remove(&(i, *evdev_code));
            }
            unicode_string_to_unicode_char(v)
        } else if first_char.is_some_and(|c| c == '0') && second_char.is_some_and(|c| c == 'x') {
            // 0x... hex notation: only Latin-1 Unicode keysyms (0x01000000–
            // 0x010000FF) are non-uppercasable; codepoints > 0xFF are fine.
            if is_latin1_unicode_keysym_hex(v) {
                bs.bmp_unicode_keys.insert((i, *evdev_code));
            } else {
                bs.bmp_unicode_keys.remove(&(i, *evdev_code));
            }
            hex_string_to_unicode_char(v)
        } else {
            // Named keysym from lookup table (e.g. "ccedilla") — not a
            // BMP Unicode keysym even if it maps to the same codepoint.
            if XKBCODES_DEF_TO_UTF8.contains_key(v.as_ref()) {
                bs.bmp_unicode_keys.remove(&(i, *evdev_code));
            }
            XKBCODES_DEF_TO_UTF8.get(v.as_ref()).cloned()
        };
        if let Some(single_char) = single_char {
            wkb.level_keymap[i].insert(*evdev_code, single_char);
        } else {
            match (v.content, layout.as_str()) {
                ("Eisu_toggle", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::None,
                        },
                        1,
                    );
                }
                ("Control_L", _) => {
                    if id == "CAPS" {
                        wkb.remap.insert(*evdev_code, 29);
                        wkb.modifiers
                            .0
                            .insert(CAPS_LOCK, crate::modifiers::Modifier::Single(ModKind::None));
                    }
                }
                ("Shift_L", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level2,
                        },
                        i as u8,
                    );
                }
                ("Shift_R", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level2,
                        },
                        i as u8,
                    );
                }
                ("Shift_Lock", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level2,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level3_Shift", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level3_Lock", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level3_Latch", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Latch {
                            pressed: false,
                            latched: false,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level5_Shift", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Level5,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level5_Lock", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Lock {
                            pressed: false,
                            locked: 0,
                            mod_type: ModType::Level5,
                        },
                        i as u8,
                    );
                }
                ("ISO_Level5_Latch", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Latch {
                            pressed: false,
                            latched: false,
                            mod_type: ModType::Level5,
                        },
                        i as u8,
                    );
                }
                ("Multi_key", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Compose,
                        },
                        i as u8,
                    );
                }
                (_, "rshift_both_shiftlock") | (_, "lshift_both_shiftlock") => {
                    wkb.right_left_shift_caps = true;
                }
                (_, "bksl_switch") => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
                            mod_type: ModType::Level3,
                        },
                        i as u8,
                    );
                }
                (_, "caps_switch") => {
                    if locale == "level3" {
                        wkb.modifiers.insert(
                            *evdev_code,
                            ModKind::Pressed {
                                pressed: false,
                                mod_type: ModType::Level3,
                            },
                            i as u8,
                        );
                    } else {
                        wkb.modifiers.insert(
                            *evdev_code,
                            ModKind::Pressed {
                                pressed: false,
                                mod_type: ModType::Level5,
                            },
                            i as u8,
                        );
                    };
                }
                _ => {
                    if v.contains("none") {
                        if i > 0 {
                            if let Some(key) = wkb.level_keymap[i - 1].clone().get(evdev_code) {
                                wkb.level_keymap[i].insert(*evdev_code, *key);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn fix_xkb_edge_cases<C: Composer>(wkb: &mut WKB<C>, locale: String, layout: Option<String>) {
    // ── Keymap fixups (compensating for WKB parser limitations) ──
    // These correct level_keymap values, num_lock_keys, etc. that WKB's
    // xkb-parser cannot derive correctly on its own.
    match (locale.as_str(), &layout.as_deref()) {
        // de:ru* key swap
        ("de", Some("ru")) | ("de", Some("ru-recom")) | ("de", Some("ru-translit")) => {
            for i in 0..2 {
                let map = wkb.level_keymap[i].clone();
                let t21 = map.get(&21);
                let t44 = map.get(&44);
                wkb.level_keymap[i].insert(21, *t44.unwrap());
                wkb.level_keymap[i].insert(44, *t21.unwrap());
            }
        }
        _ => {}
    };
    match (locale.as_str(), &layout.as_deref()) {
        // bqn: level_keymap fixups (caps metadata now auto-derived)
        ("bqn", Some("bqn")) => {
            wkb.level_keymap[1].insert(22, '⊔');
            wkb.level_keymap[1].insert(32, '↕');
            wkb.level_keymap[1].insert(36, '∘');
            wkb.level_keymap[1].insert(46, '↓');
            wkb.level_keymap[1].insert(57, '‿');
        }
        // fr:bepo_afnor: level_keymap resize
        ("fr", Some("bepo_afnor")) => {
            wkb.level_keymap.resize(4, BTreeMap::new());
        }
        // pl:glagolica: level_keymap fixup
        ("pl", Some("glagolica")) => {
            let value = *wkb.level_keymap[0].get(&40).unwrap();
            wkb.level_keymap[1].insert(40, value);
        }
        // apl: copy level0 to level1 for non-alphabetic keys
        ("apl", Some("apl2")) | ("apl", Some("aplplusII")) => {
            for i in 16..=25 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[1].insert(i, value);
            }
            for i in 30..=38 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[1].insert(i, value);
            }
            for i in 44..=50 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[1].insert(i, value);
            }
        }
        // ie:ogam level_keymap fixups
        ("ie", Some("ogam")) => {
            wkb.level_keymap[1].insert(43, '\u{1680}');
        }
        ("ie", Some("ogam_is434")) => {
            for (code, value) in wkb.level_keymap[2].clone() {
                if wkb.level_keymap[3].get(&code).is_none() {
                    wkb.level_keymap[3].insert(code, value);
                }
            }
            for (code, value) in wkb.level_keymap[0].clone() {
                if [2, 3, 4, 6, 7, 9, 10, 11, 12, 13].contains(&code) {
                    wkb.level_keymap[2].insert(code, value);
                    let value = *wkb.level_keymap[1].get(&code).unwrap();
                    wkb.level_keymap[3].insert(code, value);
                }
            }
        }
        // si: level_keymap copies
        ("si", Some(_)) => {
            let value = *wkb.level_keymap[0].get(&41).unwrap();
            wkb.level_keymap[2].insert(41, value);
            let value = *wkb.level_keymap[1].get(&41).unwrap();
            wkb.level_keymap[3].insert(41, value);
        }
        // se:rus level_keymap copies
        ("se", Some("rus")) => {
            let value = *wkb.level_keymap[0].get(&13).unwrap();
            wkb.level_keymap[2].insert(13, value);
            let value = *wkb.level_keymap[1].get(&13).unwrap();
            wkb.level_keymap[3].insert(13, value);
            for i in 16..=27 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[2].insert(i, value);
                let value = *wkb.level_keymap[1].get(&i).unwrap();
                wkb.level_keymap[3].insert(i, value);
            }
            for i in 30..=41 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[2].insert(i, value);
                let value = *wkb.level_keymap[1].get(&i).unwrap();
                wkb.level_keymap[3].insert(i, value);
            }
            for i in 43..=50 {
                let value = *wkb.level_keymap[0].get(&i).unwrap();
                wkb.level_keymap[2].insert(i, value);
                let value = *wkb.level_keymap[1].get(&i).unwrap();
                wkb.level_keymap[3].insert(i, value);
            }
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[2].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[3].insert(86, value);
        }
        // us:3l* level_keymap fixups
        ("us", Some("3l")) => {
            wkb.level_keymap[1].insert(2, '!');
            wkb.level_keymap[1].insert(3, '@');
            wkb.level_keymap[1].insert(4, '#');
            wkb.level_keymap[1].insert(5, '$');
            wkb.level_keymap[1].insert(6, '%');
            wkb.level_keymap[1].insert(7, '^');
            wkb.level_keymap[1].insert(8, '&');
            wkb.level_keymap[1].insert(9, '*');
            wkb.level_keymap[1].insert(10, '(');
            wkb.level_keymap[1].insert(11, ')');
            wkb.level_keymap[3].insert(2, '!');
            wkb.level_keymap[3].insert(3, '@');
            wkb.level_keymap[3].insert(4, '#');
            wkb.level_keymap[3].insert(5, '$');
            wkb.level_keymap[3].insert(6, '%');
            wkb.level_keymap[3].insert(7, '^');
            wkb.level_keymap[3].insert(8, '&');
            wkb.level_keymap[3].insert(9, '*');
            wkb.level_keymap[3].insert(10, '(');
            wkb.level_keymap[3].insert(11, ')');
            let value = *wkb.level_keymap[0].get(&15).unwrap();
            wkb.level_keymap[1].insert(15, value);
            wkb.level_keymap[2].insert(15, value);
            wkb.level_keymap[3].insert(15, value);
            wkb.level_keymap[4].insert(15, value);
            let value = *wkb.level_keymap[0].get(&58).unwrap();
            wkb.level_keymap[1].insert(58, value);
            wkb.level_keymap[3].insert(58, value);
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[4].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[5].insert(86, value);
            let value = *wkb.level_keymap[2].get(&86).unwrap();
            wkb.level_keymap[6].insert(86, value);
        }
        ("us", Some("3l-cros")) => {
            wkb.level_keymap[1].insert(2, '!');
            wkb.level_keymap[1].insert(3, '@');
            wkb.level_keymap[1].insert(4, '#');
            wkb.level_keymap[1].insert(5, '$');
            wkb.level_keymap[1].insert(6, '%');
            wkb.level_keymap[1].insert(7, '^');
            wkb.level_keymap[1].insert(8, '&');
            wkb.level_keymap[1].insert(9, '*');
            wkb.level_keymap[1].insert(10, '(');
            wkb.level_keymap[1].insert(11, ')');
            wkb.level_keymap[3].insert(2, '!');
            wkb.level_keymap[3].insert(3, '@');
            wkb.level_keymap[3].insert(4, '#');
            wkb.level_keymap[3].insert(5, '$');
            wkb.level_keymap[3].insert(6, '%');
            wkb.level_keymap[3].insert(7, '^');
            wkb.level_keymap[3].insert(8, '&');
            wkb.level_keymap[3].insert(9, '*');
            wkb.level_keymap[3].insert(10, '(');
            wkb.level_keymap[3].insert(11, ')');
            let value = *wkb.level_keymap[0].get(&15).unwrap();
            wkb.level_keymap[1].insert(15, value);
            wkb.level_keymap[2].insert(15, value);
            wkb.level_keymap[3].insert(15, value);
            wkb.level_keymap[4].insert(15, value);
            let value = *wkb.level_keymap[0].get(&58).unwrap();
            wkb.level_keymap[1].insert(58, value);
            wkb.level_keymap[3].insert(58, value);
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[4].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[5].insert(86, value);
            let value = *wkb.level_keymap[2].get(&86).unwrap();
            wkb.level_keymap[6].insert(86, value);
            let value = *wkb.level_keymap[0].get(&125).unwrap();
            wkb.level_keymap[1].insert(125, value);
            wkb.level_keymap[3].insert(125, value);
        }
        ("us", Some("3l-emacs")) => {
            wkb.level_keymap[1].insert(2, '!');
            wkb.level_keymap[1].insert(3, '@');
            wkb.level_keymap[1].insert(4, '#');
            wkb.level_keymap[1].insert(5, '$');
            wkb.level_keymap[1].insert(6, '%');
            wkb.level_keymap[1].insert(7, '^');
            wkb.level_keymap[1].insert(8, '&');
            wkb.level_keymap[1].insert(9, '*');
            wkb.level_keymap[1].insert(10, '(');
            wkb.level_keymap[1].insert(11, ')');
            wkb.level_keymap[3].insert(2, '!');
            wkb.level_keymap[3].insert(3, '@');
            wkb.level_keymap[3].insert(4, '#');
            wkb.level_keymap[3].insert(5, '$');
            wkb.level_keymap[3].insert(6, '%');
            wkb.level_keymap[3].insert(7, '^');
            wkb.level_keymap[3].insert(8, '&');
            wkb.level_keymap[3].insert(9, '*');
            wkb.level_keymap[3].insert(10, '(');
            wkb.level_keymap[3].insert(11, ')');
            let value = *wkb.level_keymap[0].get(&15).unwrap();
            wkb.level_keymap[1].insert(15, value);
            wkb.level_keymap[3].insert(15, value);
            let value = *wkb.level_keymap[0].get(&86).unwrap();
            wkb.level_keymap[4].insert(86, value);
            let value = *wkb.level_keymap[1].get(&86).unwrap();
            wkb.level_keymap[5].insert(86, value);
            let value = *wkb.level_keymap[2].get(&86).unwrap();
            wkb.level_keymap[6].insert(86, value);
        }
        // fr:bepo_latin9 level_keymap fixups
        ("fr", Some("bepo_latin9")) => {
            let value = *wkb.level_keymap[2].get(&55).unwrap();
            wkb.level_keymap[3].insert(55, value);
            let value = *wkb.level_keymap[2].get(&98).unwrap();
            wkb.level_keymap[3].insert(98, value);
        }
        // fr/be:oss_latin9 level_keymap fixups
        ("fr", Some("oss_latin9")) | ("be", Some("oss_latin9")) => {
            let value = *wkb.level_keymap[2].get(&55).unwrap();
            wkb.level_keymap[3].insert(55, value);
            let value = *wkb.level_keymap[2].get(&98).unwrap();
            wkb.level_keymap[3].insert(98, value);
        }
        // fr:mac level_keymap fixup
        ("fr", Some("mac")) => {
            let value = *wkb.level_keymap[0].get(&83).unwrap();
            wkb.level_keymap[3].insert(83, value);
        }
        // fr:afnor level_keymap fixups
        ("fr", Some("afnor")) => {
            for i in 0..2 {
                for (code, value) in &wkb.level_keymap[i].clone() {
                    if wkb.level_keymap[i + 4].get(&code).is_none() {
                        wkb.level_keymap[i + 4].insert(*code, *value);
                    }
                }
            }
            wkb.level_keymap[4].insert(86, '<');
            wkb.level_keymap[5].insert(55, '⋅');
            wkb.level_keymap[5].insert(74, '−');
            wkb.level_keymap[5].insert(86, '>');
            wkb.level_keymap[5].insert(98, '∕');
        }
        // de:T3 level_keymap fixups (caps metadata now auto-derived)
        ("de", Some("T3")) => {
            wkb.level_keymap[3].insert(2, 'ʹ');
            wkb.level_keymap[3].insert(3, 'ʺ');
            wkb.level_keymap[3].insert(4, 'ʿ');
            wkb.level_keymap[3].insert(5, 'ʾ');
            wkb.level_keymap[3].insert(6, 'ˁ');
            wkb.level_keymap[3].insert(7, 'ˀ');
            wkb.level_keymap[3].insert(8, '{');
            wkb.level_keymap[3].insert(9, '}');
            wkb.level_keymap[3].insert(10, '[');
            wkb.level_keymap[3].insert(11, ']');
            wkb.level_keymap[3].insert(12, 'ʻ');
            wkb.level_keymap[3].insert(13, '¬');
            wkb.level_keymap[3].insert(15, '\t');
            wkb.level_keymap[3].insert(16, '\u{30d}');
            wkb.level_keymap[3].insert(27, '@');
            wkb.level_keymap[3].insert(30, '\u{329}');
            wkb.level_keymap[3].insert(35, '\u{332}');
            wkb.level_keymap[3].insert(38, '\u{338}');
            wkb.level_keymap[3].insert(39, '°');
            wkb.level_keymap[3].insert(40, '′');
            wkb.level_keymap[3].insert(41, '|');
            wkb.level_keymap[3].insert(43, '″');
            wkb.level_keymap[3].insert(44, '«');
            wkb.level_keymap[3].insert(45, '»');
            wkb.level_keymap[3].insert(46, '―');
            wkb.level_keymap[3].insert(47, '‹');
            wkb.level_keymap[3].insert(48, '›');
            wkb.level_keymap[3].insert(49, '–');
            wkb.level_keymap[3].insert(50, '—');
            wkb.level_keymap[3].insert(51, '$');
            wkb.level_keymap[3].insert(52, '#');
            wkb.level_keymap[3].insert(53, '‑');
            wkb.level_keymap[3].insert(57, '\u{a0}');
        }
        // Extra level_keymap push for braille/apl/kr/jp layouts
        ("brai", Some("home_row"))
        | ("brai", Some("right_hand"))
        | ("brai", Some("keypad"))
        | ("apl", Some("common"))
        | ("apl", Some("dyalog_box"))
        | ("kr", Some("hw_keys"))
        | ("jp", Some("hztg_escape"))
        | ("brai", Some("right_hand_invert")) => {
            wkb.level_keymap.push(BTreeMap::from([
                (43, '|'),
                (71, '7'),
                (72, '8'),
                (73, '9'),
                (75, '4'),
                (76, '5'),
                (77, '6'),
                (78, '+'),
                (79, '1'),
                (80, '2'),
                (81, '3'),
                (82, '0'),
                (83, '.'),
                (86, '>'),
            ]));
        }
        // Num lock key fixups
        ("am", Some("eastern")) | ("am", Some("western")) | ("am", Some("eastern-alt")) => {
            wkb.num_lock_keys.push(2);
            wkb.num_lock_keys.push(5);
            wkb.num_lock_keys.push(6);
            wkb.num_lock_keys.push(7);
        }
        ("ru", Some("ruintl_en")) => {
            wkb.level_keymap[3].insert(36, 'Ø');
        }
        ("cm", Some("azerty")) => {
            wkb.num_lock_keys.push(2);
            wkb.num_lock_keys.push(3);
            wkb.num_lock_keys.push(4);
            wkb.num_lock_keys.push(5);
            wkb.num_lock_keys.push(6);
            wkb.num_lock_keys.push(7);
            wkb.num_lock_keys.push(8);
            wkb.num_lock_keys.push(9);
            wkb.num_lock_keys.push(10);
            wkb.num_lock_keys.push(11);
        }
        ("la", Some("stea")) => wkb.num_lock_keys.retain(|k| k != &83),
        ("de", Some("neo_base"))
        | ("de", Some("neo"))
        | ("de", Some("neo_qwerty_base"))
        | ("de", Some("neo_qwerty"))
        | ("de", Some("neo_qwertz_base"))
        | ("de", Some("neo_qwertz"))
        | ("de", Some("bone_base"))
        | ("de", Some("bone_eszett_home_base"))
        | ("de", Some("bone_eszett_home"))
        | ("de", Some("bone"))
        | ("de", Some("koy_base"))
        | ("de", Some("koy"))
        | ("de", Some("adnw_base"))
        | ("de", Some("adnw")) => {
            wkb.num_lock_keys = Vec::new();
        }
        _ => {}
    }
    if wkb.level_keymap.len() > 1 {
        for (code, value) in wkb.level_keymap[0].clone() {
            if wkb.level_keymap[1].get(&code).is_none() {
                wkb.level_keymap[1].insert(code, value);
            }
        }
    }
    for i in 0..wkb.level_keymap.len() {
        let next_large = std::cmp::min(i + 2, wkb.level_keymap.len() - 1);
        let next_small = std::cmp::min(i + 1, wkb.level_keymap.len() - 1);
        let next = std::cmp::max(next_small, next_large);
        if next > i {
            for (code, value) in &wkb.level_keymap[i].clone() {
                if wkb.level_keymap[next].get(&code).is_none() {
                    wkb.level_keymap[next].insert(*code, *value);
                }
            }
        }
    }
    for i in 0..std::cmp::min(wkb.level_keymap.len(), DEFAULT_MAP.len()) {
        for (code, value) in &DEFAULT_MAP[i] {
            if wkb.level_keymap[i].get(&code).is_none() {
                wkb.level_keymap[i].insert(*code, *value);
            }
        }
    }
}
