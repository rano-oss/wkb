use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::{self, File},
    io::{self, BufRead, Read},
    os::fd::OwnedFd,
    path::Path,
};

use crate::modifiers::Modifiers;
use crate::{
    composer::{Composer, ListComposer},
    modifiers::{Modifier, LEFT_SHIFT, RIGHT_SHIFT},
};

use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};

use crate::modifiers::{ModKind, ModType, BACKSPACE, CAPS_LOCK};
use crate::WKB;

pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

mod default_keymap;
pub mod evdev_xkb;
mod repeat;
pub mod xkb_compose_map;
pub mod xkb_utf8;

use default_keymap::DEFAULT_MAP;
use repeat::REPEAT_DEFAULT;

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
    /// Keycodes explicitly encountered during XKB parsing (from key
    /// definitions in xkb_symbols sections, including included files).
    /// Used by `build_repeat_keys` to distinguish keys that were actually
    /// defined in the keymap from those filled in by `DEFAULT_MAP`.
    /// Only explicitly-defined keys should be added to the repeat set
    /// beyond what `REPEAT_DEFAULT` already provides, because xkbcommon
    /// reports `key_repeats() == false` for keycodes that don't exist
    /// in the compiled keymap.
    explicit_keys: HashSet<u32>,
    /// Evdev keycodes whose definitions contain at least one KP_* keysym
    /// at a level > 0 but NOT at level 0. xkbcommon's `FindAutomaticType`
    /// assigns the KEYPAD key type to 2-symbol keys that contain a keypad
    /// keysym, which changes the modifier-to-level mapping: Shift stays
    /// at Level 1, only NumLock activates Level 2.
    ///
    /// Keys where level 0 is ALSO a KP_* keysym (e.g. `[KP_Delete, KP_Decimal]`)
    /// are excluded because xkbcommon expands those to FOUR_LEVEL_KEYPAD
    /// where Shift→Level 2 is correct.
    keypad_shifted_keys: HashSet<u32>,
    /// Evdev keycodes whose level-0 symbol is a KP_* keysym.
    keypad_base_keys: HashSet<u32>,
    /// Number of symbols explicitly defined for each key (evdev code).
    /// Used by `fix_key_type_levels` to distinguish 1-symbol ONE_LEVEL keys
    /// (where DEFAULT_MAP digit values at level 1 should be overwritten)
    /// from 2-symbol keys whose level 1 was intentionally defined.
    key_num_symbols: HashMap<u32, usize>,
}

impl XkbBuildState {
    fn new() -> Self {
        Self {
            bmp_unicode_keys: HashSet::new(),
            default_key_type: None,
            key_types: HashMap::new(),
            explicit_keys: HashSet::new(),
            keypad_shifted_keys: HashSet::new(),
            keypad_base_keys: HashSet::new(),
            key_num_symbols: HashMap::new(),
        }
    }
}

/// Unicode simple uppercase mapping for characters where Rust's
/// `char::to_uppercase()` (full case mapping) expands to multiple
/// codepoints or doesn't map at all.
///
/// Note: Mathematical Alphanumeric Symbols (U+1D400–U+1D7FF) are NOT
/// included here because xkbcommon's `xkb_keysym_to_upper` does not
/// uppercase them (its case tables don't cover SMP mathematical symbols).
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
        _ => None,
    }
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
                layouts.push(src.name.content.to_string());
            }
            _ => {}
        });
    } else {
        let re = Regex::new(r#"xkb_symbols\s+"([\w\s_\-\d]+)"\s+\{"#).unwrap();
        for cap in re.captures_iter(&string_file) {
            if let Some(name) = cap.get(1) {
                layouts.push(name.as_str().to_string());
            }
        }
    }
    layouts
}

fn parse_include(input: &str) -> (String, Option<String>) {
    let re = Regex::new(r"([\w]+(?:/[\w]+)?)(?:\(([\w\-]+)\))?$").unwrap();
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
    let mut num_lock_codes: Vec<u32> = vec![
        71, 72, 73, // 7, 8, 9
        75, 76, 77, // 4, 5, 6
        79, 80, 81, // 1, 2, 3
        82, 83, // 0, .
    ];
    map_xkb(
        &mut wkb,
        &mut bs,
        path,
        locale.clone(),
        Some(layout.clone()),
    );
    fix_xkb_edge_cases(&mut wkb, &mut num_lock_codes, locale, Some(layout));
    wkb.num_lock_keys = build_num_lock_table(&num_lock_codes, &wkb.state_keymap, &bs.key_types);

    // apl:dyalog_box — numpad keys are ONE_LEVEL in xkbcommon, so
    // Shift does not produce digits even when NumLock is active (Shift
    // cancels NumLock back to the base box-drawing character).  Copy
    // level 0 values into num_lock_keys at odd (Shift) levels.
    if wkb.locale.as_deref() == Some("apl") && matches!(wkb.layout.as_str(), "dyalog") {
        wkb.modifiers.0.insert(
            LEFT_SHIFT,
            Modifier::Single(ModKind::Pressed {
                pressed: false,
                mod_type: ModType::None,
            }),
        );
        wkb.modifiers.0.insert(
            RIGHT_SHIFT,
            Modifier::Single(ModKind::Pressed {
                pressed: false,
                mod_type: ModType::None,
            }),
        );
    }
    fix_key_type_levels(&mut wkb, &num_lock_codes, &bs);

    // Post-fix_key_type_levels fixups for specific layouts
    match (wkb.locale.as_deref(), wkb.layout.as_str()) {
        (Some("fr"), "latin9") | (Some("fr"), "latin9_nodeadkeys") => {
            if wkb.state_keymap.len() > 1 {
                wkb.state_keymap[1].insert(83, '.');
            }
        }
        (Some("fr"), "mac") => {
            if wkb.state_keymap.len() > 3 {
                wkb.state_keymap[3].insert(83, ',');
            }
        }
        _ => {}
    }

    wkb.caps_lock_table = build_caps_lock_table(
        &wkb.state_keymap,
        &bs.key_types,
        &bs.default_key_type,
        &bs.bmp_unicode_keys,
        &bs.key_num_symbols,
    );

    // Post-build_caps_lock_table fixups for specific layouts
    match (wkb.locale.as_deref(), wkb.layout.as_str()) {
        (Some("fr"), "afnor") => {
            // fr:afnor: keys 16 and 24 are incorrectly detected as FOUR_LEVEL_SEMIALPHABETIC
            // instead of EIGHT_LEVEL_ALPHABETIC. Fix caps_lock_table levels 4 and 5.
            for code in [16, 24] {
                if wkb.state_keymap.len() > 4 {
                    if let Some(&c4) = wkb.state_keymap[4].get(&code) {
                        if wkb.caps_lock_table.len() > 4 {
                            wkb.caps_lock_table[4].insert(code, c4);
                        }
                    }
                }
                if wkb.state_keymap.len() > 5 {
                    if let Some(&c5) = wkb.state_keymap[5].get(&code) {
                        if wkb.caps_lock_table.len() > 5 {
                            wkb.caps_lock_table[5].insert(code, c5);
                        }
                    }
                }
            }
        }
        _ => {}
    }

    wkb.repeat_keys = build_repeat_keys(&wkb.state_keymap, &wkb.modifiers, &bs.explicit_keys);
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
    let num_lock_codes: Vec<u32> = vec![
        71, 72, 73, // 7, 8, 9
        75, 76, 77, // 4, 5, 6
        79, 80, 81, // 1, 2, 3
        82, 83, // 0, .
    ];
    wkb.num_lock_keys = build_num_lock_table(&num_lock_codes, &wkb.state_keymap, &bs.key_types);

    fix_key_type_levels(&mut wkb, &num_lock_codes, &bs);
    wkb.caps_lock_table = build_caps_lock_table(
        &wkb.state_keymap,
        &bs.key_types,
        &bs.default_key_type,
        &bs.bmp_unicode_keys,
        &bs.key_num_symbols,
    );
    wkb.repeat_keys = build_repeat_keys(&wkb.state_keymap, &wkb.modifiers, &bs.explicit_keys);
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
    state_keymap: &[BTreeMap<u32, char>],
    key_types: &HashMap<u32, String>,
    _default_key_type: &Option<String>,
    bmp_unicode_keys: &HashSet<(usize, u32)>,
    key_num_symbols: &HashMap<u32, usize>,
) -> Vec<BTreeMap<u32, char>> {
    if state_keymap.len() < 2 {
        return Vec::new();
    }

    let num_levels = state_keymap.len();
    let mut table: Vec<BTreeMap<u32, char>> = vec![BTreeMap::new(); num_levels];

    // Collect all evdev codes that appear in any level
    let mut all_codes: HashSet<u32> = HashSet::new();
    for level in state_keymap {
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
                if let Some(&c) = state_keymap[level].get(&evdev_code) {
                    flagged_chars.insert(c);
                }
            }
        }
        if !flagged_chars.is_empty() {
            for level in 0..num_levels {
                if !effective_bmp.contains(&(level, evdev_code)) {
                    if let Some(&c) = state_keymap[level].get(&evdev_code) {
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
        let key_type_name = key_types.get(&evdev_code).cloned();
        let caps_class = match key_types.get(&evdev_code) {
            Some(name) => key_type_caps_class(name),
            None => {
                // Auto-detect: replicate xkbcommon's FindAutomaticType logic.
                // The key type depends on the key's OWN symbol count, not the
                // total number of levels in the keymap.
                //
                //  2-symbol case pair → ALPHABETIC (Lock toggles at all levels)
                //  4-symbol case pair at levels 0-1 AND 2-3 → FOUR_LEVEL_ALPHABETIC
                //  4-symbol case pair at levels 0-1 only → FOUR_LEVEL_SEMIALPHABETIC
                //  otherwise → PostProcess (uppercase post-processing)
                let num_syms = key_num_symbols.get(&evdev_code).copied().unwrap_or(0);
                if is_case_pair(state_keymap, evdev_code, &effective_bmp) {
                    if num_syms <= 2 {
                        // 2-symbol ALPHABETIC: Lock toggles Shift bit at all levels.
                        // AltGr/Level5 are not in the type's modifier set, so the
                        // key always behaves as if only Shift+Lock are active.
                        CapsClass::Alphabetic
                    } else {
                        // 4+ symbols: check if levels 2-3 also form a case pair
                        // → FOUR_LEVEL_ALPHABETIC (toggle at all levels)
                        // Otherwise → FOUR_LEVEL_SEMIALPHABETIC (toggle 0-1 only,
                        // post-process uppercase at 2+)
                        //
                        // Check if levels 2-3 also form a case pair, matching
                        // xkbcommon's FindAutomaticType logic.  xkbcommon checks
                        // xkb_keysym_is_lower(sym[2]) && xkb_keysym_is_upper(sym[3])
                        // without requiring that levels 2-3 differ from 0-1.
                        // Keys like [q, Q, q, Q] (explicitly defined with 4
                        // identical-to-base symbols) are correctly detected as
                        // FOUR_LEVEL_ALPHABETIC by xkbcommon.
                        let has_level23_case_pair = if num_syms >= 4 && state_keymap.len() > 3 {
                            let l2 = state_keymap[2].get(&evdev_code).copied();
                            let l3 = state_keymap[3].get(&evdev_code).copied();
                            if let (Some(c2), Some(c3)) = (l2, l3) {
                                let c2_lower = caps_uppercase(c2) != c2 || c2.is_lowercase();
                                let c3_upper =
                                    c3.to_lowercase().next() != Some(c3) || c3.is_uppercase();
                                c2 != c3 && c2_lower && c3_upper
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if has_level23_case_pair {
                            CapsClass::Alphabetic
                        } else {
                            CapsClass::SemiAlphabetic
                        }
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
                //
                // For 2-symbol ALPHABETIC keys (e.g. [q, Q]) in layouts with
                // more than 2 levels, AltGr/Level5 are NOT in the type's
                // modifier set.  CapsLock always behaves as if only Shift+Lock
                // are active — so even levels get the shift char and odd levels
                // get the base char, regardless of the AltGr/Level5 state.
                //
                // When a pair has identical values (from propagation) or the
                // pair partner doesn't exist, fall back to the level 0-1 values.
                let l0_char = state_keymap[0].get(&evdev_code).copied();
                let l1_char = if num_levels > 1 {
                    state_keymap[1].get(&evdev_code).copied()
                } else {
                    None
                };

                for pair_base in (0..num_levels).step_by(2) {
                    let pair_shift = pair_base + 1;
                    let base_char = state_keymap[pair_base].get(&evdev_code).copied();
                    let shift_char = if pair_shift < num_levels {
                        state_keymap[pair_shift].get(&evdev_code).copied()
                    } else {
                        None
                    };

                    match (base_char, shift_char) {
                        (Some(bc), Some(sc)) if bc != sc => {
                            table[pair_base].insert(evdev_code, sc);
                            table[pair_shift].insert(evdev_code, bc);
                        }
                        (Some(_bc), Some(_sc)) => {
                            // Pair values are identical (propagated from 0-1).
                            // Use level 0-1 values for the toggle.
                            if let (Some(b), Some(s)) = (l0_char, l1_char) {
                                if b != s {
                                    table[pair_base].insert(evdev_code, s);
                                    table[pair_shift].insert(evdev_code, b);
                                }
                            }
                        }
                        (Some(_bc), None) if pair_shift >= num_levels => {
                            // Unpaired level (odd number of levels).
                            // Use the level-1 value (shift char) for this
                            // even level, matching the CapsLock toggle.
                            if let Some(s) = l1_char {
                                if Some(s) != base_char {
                                    table[pair_base].insert(evdev_code, s);
                                }
                            }
                        }
                        (None, Some(sc)) => {
                            table[pair_base].insert(evdev_code, sc);
                        }
                        (Some(bc), None) => {
                            if pair_shift < num_levels {
                                table[pair_shift].insert(evdev_code, bc);
                            }
                        }
                        _ => {}
                    }
                }
            }
            CapsClass::SemiAlphabetic => {
                // FOUR_LEVEL_SEMIALPHABETIC / EIGHT_LEVEL_SEMIALPHABETIC:
                //   Lock toggles at "base" pairs (0-1, 4-5) where the
                //   higher modifier (LevelFive) is active but LevelThree
                //   is not.  Lock is preserved (post-process uppercase) at
                //   "AltGr" pairs (2-3, 6-7).
                //
                // Pattern by pair index (pair_base / 2):
                //   even pair index (0, 2, …) → toggle (swap pair values)
                //   odd  pair index (1, 3, …) → post-process uppercase
                for pair_base in (0..num_levels).step_by(2) {
                    let pair_shift = pair_base + 1;
                    let pair_index = pair_base / 2;
                    let is_toggle_pair = pair_index % 2 == 0;

                    if is_toggle_pair {
                        // Toggle (swap) at this pair
                        let bc = state_keymap[pair_base].get(&evdev_code).copied();
                        let sc = if pair_shift < num_levels {
                            state_keymap[pair_shift].get(&evdev_code).copied()
                        } else {
                            None
                        };
                        match (bc, sc) {
                            (Some(b), Some(s)) if b != s => {
                                table[pair_base].insert(evdev_code, s);
                                table[pair_shift].insert(evdev_code, b);
                            }
                            (None, Some(s)) => {
                                table[pair_base].insert(evdev_code, s);
                            }
                            (Some(b), None) if pair_shift < num_levels => {
                                table[pair_shift].insert(evdev_code, b);
                            }
                            _ => {}
                        }
                    } else {
                        // Post-process uppercase at this pair
                        for level in pair_base..std::cmp::min(pair_base + 2, num_levels) {
                            if effective_bmp.contains(&(level, evdev_code)) {
                                continue;
                            }
                            if let Some(&c) = state_keymap[level].get(&evdev_code) {
                                let upper = caps_uppercase(c);
                                if upper != c {
                                    table[level].insert(evdev_code, upper);
                                }
                            }
                        }
                    }
                }
            }
            CapsClass::PlusLock(target_level) => {
                // Lock redirects level 0 to target_level; other levels unaffected
                if target_level < num_levels {
                    if let Some(&tc) = state_keymap[target_level].get(&evdev_code) {
                        let base_char = state_keymap[0].get(&evdev_code).copied();
                        if base_char != Some(tc) {
                            table[0].insert(evdev_code, tc);
                        }
                    }
                }
            }
            CapsClass::SeparateCapsShift => {
                // SEPARATE_CAPS_AND_SHIFT_ALPHABETIC type:
                //   map[Lock] = Level4;              preserve[Lock] = Lock;
                //   map[Lock+LevelThree] = Level3;   preserve[Lock+LevelThree] = Lock;
                //   map[Shift+Lock+LevelThree] = Level3;  (no preserve)
                //
                // Level 0: Lock → Level4 (index 3), Lock preserved → post-process
                if num_levels > 3 {
                    if let Some(&tc) = state_keymap[3].get(&evdev_code) {
                        let upper = caps_uppercase(tc);
                        let base_char = state_keymap[0].get(&evdev_code).copied();
                        if base_char != Some(upper) {
                            table[0].insert(evdev_code, upper);
                        }
                    }
                }
                // Level 1 (Shift): Shift+Lock not mapped → defaults to Level1 (index 0)
                if let Some(&bc) = state_keymap[0].get(&evdev_code) {
                    let shift_char = state_keymap[1].get(&evdev_code).copied();
                    if shift_char != Some(bc) {
                        table[1].insert(evdev_code, bc);
                    }
                }
                // Level 2 (AltGr): Lock+AltGr → Level3 (index 2), Lock preserved
                // → post-process uppercase on level 2 value
                if num_levels > 2 {
                    if !effective_bmp.contains(&(2, evdev_code)) {
                        if let Some(&c) = state_keymap[2].get(&evdev_code) {
                            let upper = caps_uppercase(c);
                            let l2_char = state_keymap[2].get(&evdev_code).copied();
                            if l2_char != Some(upper) {
                                table[2].insert(evdev_code, upper);
                            }
                        }
                    }
                }
                // Level 3 (AltGr+Shift): Shift+Lock+AltGr → Level3 (index 2),
                // Lock NOT preserved → use level 2 value directly
                if num_levels > 3 {
                    if let Some(&c2) = state_keymap[2].get(&evdev_code) {
                        let l3_char = state_keymap[3].get(&evdev_code).copied();
                        if l3_char != Some(c2) {
                            table[3].insert(evdev_code, c2);
                        }
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
                    if let Some(&c) = state_keymap[level].get(&evdev_code) {
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
    state_keymap: &[BTreeMap<u32, char>],
    evdev_code: u32,
    bmp_unicode_keys: &HashSet<(usize, u32)>,
) -> bool {
    // If either level 0 or level 1 keysym is a Latin-1 Unicode keysym
    // (0x01000000–0x010000FF from hex notation), xkbcommon's
    // xkb_keysym_to_upper/to_lower returns it unchanged → not a case pair.
    if bmp_unicode_keys.contains(&(0, evdev_code)) || bmp_unicode_keys.contains(&(1, evdev_code)) {
        return false;
    }
    if state_keymap.len() < 2 {
        return false;
    }
    let base_char = state_keymap[0].get(&evdev_code).copied();
    let shift_char = state_keymap[1].get(&evdev_code).copied();
    if let (Some(bc), Some(sc)) = (base_char, shift_char) {
        if bc == sc {
            return false;
        }
        // Match xkbcommon 1.11+'s type inference:
        //   xkb_keysym_is_lower  → checks Unicode General_Category Ll
        //   xkb_keysym_is_upper_or_title → checks Unicode General_Category Lu/Lt
        //
        // For bc_is_lower we use caps_uppercase(bc) != bc, which covers
        // both standard to_uppercase and the simple_uppercase fallback table.
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
    WKB {
        layouts,
        layout,
        locale,
        regular_composer,
        compose_key_composer,
        state_keymap: Vec::with_capacity(8),
        pressed_keys: HashSet::new(),
        repeat_keys: HashSet::new(),
        modifiers: Modifiers::default(),
        num_lock_keys: Vec::new(),
        caps_lock_table: Vec::new(),
        level_exceptions_keymap: Vec::new(),
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
                            // Record this keycode as explicitly defined
                            bs.explicit_keys.insert(*evdev_code);
                            // Track whether this key definition contains an
                            // explicit per-key type (type[Group1] = "...").
                            let mut has_explicit_type = false;
                            // Track whether we've already processed a KeyNames
                            // group for this key.  Multi-group definitions like
                            //   key <AE01> { [1, exclam], [kana_NU] };
                            // produce multiple KeyNames entries — one per group.
                            // xkbcommon only uses Group 1 by default, so we must
                            // skip Group 2+ to avoid overwriting level 0 values.
                            let mut key_names_processed = false;
                            values.iter().for_each(|v| {
                                if let xkb_parser::ast::KeyValue::KeyDefs(key_defs) = v {
                                    // Per-key type declaration: type[Group1] = "..."
                                    if let xkb_parser::ast::KeyDef::TypeDef(td) = key_defs {
                                        bs.key_types
                                            .insert(*evdev_code, td.content.content.to_string());
                                        has_explicit_type = true;
                                    }
                                    if let xkb_parser::ast::KeyDef::SymbolDef(key) = key_defs {
                                        // Track symbol count for auto-type detection.
                                        let new_count = key.values.values.len();
                                        let entry =
                                            bs.key_num_symbols.entry(*evdev_code).or_insert(0);
                                        *entry = std::cmp::max(*entry, new_count);

                                        for (i, v) in key.values.values.iter().enumerate() {
                                            if i == wkb.state_keymap.len() {
                                                if i < DEFAULT_MAP.len() {
                                                    wkb.state_keymap.push(DEFAULT_MAP[i].clone());
                                                } else {
                                                    wkb.state_keymap.push(BTreeMap::new());
                                                }
                                            }
                                            // Track keys containing KP_* keysyms for
                                            // KEYPAD-type level adjustment.
                                            if v.starts_with("KP_") {
                                                if i == 0 {
                                                    bs.keypad_base_keys.insert(*evdev_code);
                                                } else {
                                                    bs.keypad_shifted_keys.insert(*evdev_code);
                                                }
                                            } else if i == 0 {
                                                bs.keypad_base_keys.remove(evdev_code);
                                            }
                                            let mut chars = v.chars();
                                            let count = chars.clone().count();
                                            let first_char = chars.next();
                                            let is_hex = chars.all(|c| c.is_ascii_hexdigit());
                                            let mut chars = v.chars();
                                            chars.next();
                                            let second_char = chars.next();
                                            let single_char = if count == 1
                                                && first_char.is_some_and(|c| c.is_alphanumeric())
                                            {
                                                // Standard single-char keysym (e.g. 'a') — not a BMP Unicode keysym.
                                                bs.bmp_unicode_keys.remove(&(i, *evdev_code));
                                                first_char
                                            } else if first_char.is_some_and(|c| c == 'U') && is_hex
                                            {
                                                // Uxxxx notation: xkbcommon normalises codepoints ≤ 0xFF to named
                                                // keysyms (uppercasable) and > 0xFF to Unicode keysyms where
                                                // xkb_keysym_to_upper works correctly — so never flag these.
                                                if is_latin1_unicode_keysym_u(v) {
                                                    bs.bmp_unicode_keys.insert((i, *evdev_code));
                                                } else {
                                                    bs.bmp_unicode_keys.remove(&(i, *evdev_code));
                                                }
                                                unicode_string_to_unicode_char(v)
                                            } else if first_char.is_some_and(|c| c == '0')
                                                && second_char.is_some_and(|c| c == 'x')
                                            {
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
                                            if let Some(char) = single_char {
                                                wkb.state_keymap[i].insert(*evdev_code, char);
                                            }
                                        }
                                    }
                                } else if let xkb_parser::ast::KeyValue::KeyNames(key) = v {
                                    if !key_names_processed {
                                        key_names_processed = true;
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
            let value = *wkb.state_keymap[0].get(&BACKSPACE).unwrap();
            wkb.state_keymap[1].insert(CAPS_LOCK, value);
            // Neutralize CAPS_LOCK modifier so is_caps_lock_modifier() returns false
            wkb.modifiers
                .0
                .insert(CAPS_LOCK, crate::modifiers::Modifier::Single(ModKind::None));
        } else if key.values.first().is_some_and(|k| k.content == "Tab") {
            wkb.modifiers
                .0
                .insert(CAPS_LOCK, crate::modifiers::Modifier::Single(ModKind::None));
        }
    }
    // Track how many symbols are explicitly defined for this key.
    let new_count = key.values.len();
    let entry = bs.key_num_symbols.entry(*evdev_code).or_insert(0);
    *entry = std::cmp::max(*entry, new_count);

    for (i, v) in key.values.iter().enumerate() {
        if i == wkb.state_keymap.len() {
            if i < DEFAULT_MAP.len() {
                wkb.state_keymap.push(DEFAULT_MAP[i].clone());
            } else {
                wkb.state_keymap.push(BTreeMap::new());
            }
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
        // Track keys that contain KP_* keysyms so we can apply
        // KEYPAD-type level adjustments later (Shift stays at Level 1).
        if v.content.starts_with("KP_") {
            if i == 0 {
                bs.keypad_base_keys.insert(*evdev_code);
            } else {
                bs.keypad_shifted_keys.insert(*evdev_code);
            }
        } else if i == 0 {
            // Position 0 overridden with a non-KP_ keysym (e.g. an
            // include defined [KP_Home, KP_7] but this section
            // redefines it as [U250C, KP_7]).  Clear the stale entry
            // so fix_key_type_levels can detect the KEYPAD auto-type.
            bs.keypad_base_keys.remove(evdev_code);
        }
        if let Some(single_char) = single_char {
            wkb.state_keymap[i].insert(*evdev_code, single_char);
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
                        wkb.modifiers
                            .0
                            .insert(CAPS_LOCK, crate::modifiers::Modifier::Single(ModKind::None));
                    }
                }
                ("Shift_L", _) | ("Shift_R", _) => {
                    wkb.modifiers.insert(
                        *evdev_code,
                        ModKind::Pressed {
                            pressed: false,
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
                            if let Some(key) = wkb.state_keymap[i - 1].clone().get(evdev_code) {
                                wkb.state_keymap[i].insert(*evdev_code, *key);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Build the NumLock lookup table from the list of NumLock-affected key codes,
/// the fully-populated level_keymap, and the per-key type map.
///
/// Returns a `Vec<BTreeMap<u32, char>>` indexed by base_level (0–7).
/// For each level, the BTreeMap maps evdev_code → char that should be
/// returned when NumLock is active at that modifier combination.
///
/// Shift-cancels-NumLock: in xkbcommon's KEYPAD / FOUR_LEVEL_MIXED_KEYPAD
/// types, pressing Shift while NumLock is active reverts to Level1
/// (navigation).  We model this by leaving Shift-active levels empty in
/// the table so that `utf8()` can detect the cancellation and return the
/// base (level 0) value instead.
///
/// Key type determines behavior:
/// - `FOUR_LEVEL_MIXED_KEYPAD`:
///     - No Shift, no Level3  → digit (level 1)
///     - Shift, no Level3     → empty (Shift cancels NumLock)
///     - Level3 (±Shift)      → level3_val or digit (Shift does NOT cancel here)
/// - Standard / other types:
///     - No Shift             → digit (level 1)
///     - Shift                → empty (Shift cancels NumLock)
fn build_num_lock_table(
    num_lock_codes: &[u32],
    state_keymap: &[BTreeMap<u32, char>],
    key_types: &HashMap<u32, String>,
) -> Vec<BTreeMap<u32, char>> {
    if num_lock_codes.is_empty() || state_keymap.is_empty() {
        return Vec::new();
    }

    let num_levels = state_keymap.len();
    let mut table: Vec<BTreeMap<u32, char>> = vec![BTreeMap::new(); num_levels];

    for &evdev_code in num_lock_codes {
        let is_mixed_keypad = key_types
            .get(&evdev_code)
            .is_some_and(|t| t == "FOUR_LEVEL_MIXED_KEYPAD");

        // Digit value (XKB Level2 = WKB level 1)
        let digit = state_keymap
            .get(1)
            .and_then(|m| m.get(&evdev_code).copied())
            .or_else(|| DEFAULT_MAP.get(1).and_then(|m| m.get(&evdev_code).copied()));

        if is_mixed_keypad {
            // Level3 value (XKB Level3 = WKB level 2), but only if it
            // was explicitly defined — propagated copies match level 0.
            let l0 = state_keymap
                .get(0)
                .and_then(|m| m.get(&evdev_code).copied());
            let l2 = state_keymap
                .get(2)
                .and_then(|m| m.get(&evdev_code).copied());
            let level3_val = if l2.is_some() && l2 != l0 { l2 } else { None };

            for level in 0..num_levels {
                let has_shift = (level & 1) != 0;
                let has_level3 = (level & 2) != 0;
                if has_level3 {
                    // Level3 active: Shift does NOT cancel NumLock.
                    // Use level3_val (alt digit) or fall back to digit.
                    if let Some(v) = level3_val {
                        table[level].insert(evdev_code, v);
                    } else if let Some(v) = digit {
                        table[level].insert(evdev_code, v);
                    }
                } else if !has_shift {
                    // No Level3, no Shift: NumLock → digit
                    if let Some(v) = digit {
                        table[level].insert(evdev_code, v);
                    }
                }
                // !has_level3 && has_shift: Shift cancels NumLock → no entry
            }
        } else {
            // Standard KEYPAD: NumLock → digit, but Shift cancels NumLock.
            // Level3/Level5 are not part of the KEYPAD type modifier set,
            // so only Shift (bit 0) matters for cancellation.
            if let Some(v) = digit {
                for level in 0..num_levels {
                    let has_shift = (level & 1) != 0;
                    if !has_shift {
                        table[level].insert(evdev_code, v);
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

/// Build the set of repeat keys at runtime.
///
/// In xkbcommon, a key repeats if it **exists** in the compiled keymap
/// and its level-0 keysym is not a modifier.  WKB cannot fully track
/// key existence because infrastructure keys (ESC, Tab, Enter, F-keys,
/// numpad, multimedia keys, etc.) are resolved from `DEFAULT_MAP`
/// rather than from XKB keysym parsing.  Only the main keyboard area
/// keys (number row, letter rows, symbol keys — evdev codes 2–13,
/// 16–27, 30–41, 44–53) are reliably recorded in `explicit_keys`.
///
/// Strategy:
/// 1. Start with `REPEAT_DEFAULT` which covers every standard repeating
///    keycode (infrastructure + main keyboard area).
/// 2. Remove "layout-specific" keys (the main keyboard area) that were
///    **not** explicitly parsed — these don't exist in the compiled
///    keymap for fragment/overlay layouts.
/// 3. Add explicitly-parsed keys that aren't already present.
/// 4. Remove modifier keys (checking level-0 only, matching xkbcommon).
fn build_repeat_keys(
    state_keymap: &[BTreeMap<u32, char>],
    modifiers: &Modifiers,
    explicit_keys: &HashSet<u32>,
) -> HashSet<u32> {
    /// Keys in the main keyboard area whose existence varies by layout.
    /// Fragment/overlay layouts (e.g. `apl:level3`, `jp:henkan`) don't
    /// define these, so they should not repeat.  Full layouts (us:basic,
    /// de:basic, …) always define them via includes.
    fn is_layout_specific(code: u32) -> bool {
        matches!(code, 2..=13 | 16..=27 | 30..=41 | 44..=53)
    }

    let mut repeat = REPEAT_DEFAULT.clone();

    // Remove layout-specific keys that weren't explicitly parsed.
    repeat.retain(|&code| !is_layout_specific(code) || explicit_keys.contains(&code));

    // Add explicitly-parsed keys that produce characters (have entries
    // in level_keymap).  This handles keys not in REPEAT_DEFAULT that
    // are real character keys in certain layouts (e.g. KEY_RO / 89 in
    // Brazilian layouts, KEY_YEN / 124 in French ergol).  Keys that
    // were parsed but only have function/modifier keysyms (no character
    // mapping) are NOT added — they don't repeat in xkbcommon either.
    for &code in explicit_keys {
        let has_char = state_keymap.iter().any(|m| m.contains_key(&code));
        if has_char {
            repeat.insert(code);
        }
    }

    // Remove keys that act as modifiers at level 0.
    // ModKind::None means the modifier slot was neutralised (e.g. CAPS
    // remapped to BackSpace) — those keys DO repeat.
    //
    // Exception: Scroll Lock (70) repeats in xkbcommon despite being a
    // lock modifier — the default compat section sets `repeat = True`
    // for Scroll_Lock, unlike Caps_Lock and Num_Lock.
    //
    // Keys that only have modifier roles at levels > 0 (Leveled map
    // without a level-0 entry) keep repeating, matching xkbcommon's
    // behaviour of checking only the level-0 keysym.
    for (&code, modifier) in &modifiers.0 {
        if code == crate::modifiers::SCROLL_LOCK {
            continue;
        }
        let is_modifier_at_level0 = match modifier {
            crate::modifiers::Modifier::Single(ModKind::None) => false,
            crate::modifiers::Modifier::Single(_) => true,
            crate::modifiers::Modifier::Leveled(map) => {
                // Only suppress repeat if level 0 is present and not None
                map.get(&0).is_some_and(|mk| !matches!(mk, ModKind::None))
            }
        };
        if is_modifier_at_level0 {
            repeat.remove(&code);
        }
    }

    repeat
}

/// Adjust `level_keymap` entries for keys whose XKB key type uses a
/// non-standard modifier-to-level mapping.
///
/// xkbcommon assigns key types that change how modifiers select levels:
///
/// - **KEYPAD** (auto-detected for 2-symbol keys containing a KP_* keysym):
///   Shift stays at Level 1; only NumLock activates Level 2.
///
/// - **FOUR_LEVEL_X** (explicit `type = "FOUR_LEVEL_X"`):
///   `map[Shift] = Level1`, `map[LevelThree] = Level2`,
///   `map[Shift+LevelThree] = Level3`, `map[Control+Alt] = Level4`.
///
/// - **ALPHABETIC** (explicit `type = "ALPHABETIC"`):
///   Only Shift and Lock affect the level; AltGr / Level5 have no effect.
///
/// WKB's level index assumes: 0=None, 1=Shift, 2=AltGr, 3=Shift+AltGr,
/// so we rearrange the per-key values to match xkbcommon's resolution.
///
/// Must be called **after** `build_num_lock_table` (which needs the
/// original Level 2 digit values) and **after** `fix_xkb_edge_cases`
/// (which fills DEFAULT_MAP fallbacks).
fn fix_key_type_levels<C: Composer>(wkb: &mut WKB<C>, num_lock_codes: &[u32], bs: &XkbBuildState) {
    // ── 1. KEYPAD / ONE_LEVEL keys ──────────────────────────────────
    //
    // xkbcommon auto-detects the key type from the keysym content:
    //
    //  • 1-symbol key → ONE_LEVEL: Shift has no effect.
    //  • 2-symbol key with mixed KP_/non-KP_ keysyms → KEYPAD:
    //        Shift stays at Level 1, only NumLock activates Level 2.
    //  • 2-symbol key where BOTH syms are KP_ → expanded to
    //        FOUR_LEVEL_KEYPAD when LevelThree is available:
    //        Shift → Level 2 (standard mapping, no fix needed).
    //
    // For ONE_LEVEL keys in `num_lock_codes` whose level 1 was filled
    // by DEFAULT_MAP (not by the parser), copy level 0 → level 1 so
    // that Shift produces the same character as the unmodified press.
    for &code in num_lock_codes {
        let key_type = bs.key_types.get(&code).map(|s| s.as_str());
        match key_type {
            // These explicit types map Shift → Level2, matching WKB's default.
            Some("FOUR_LEVEL_MIXED_KEYPAD") | Some("FOUR_LEVEL_KEYPAD") => continue,
            // FOUR_LEVEL_X is handled in the next section.
            Some("FOUR_LEVEL_X") => continue,
            _ => {
                let num_syms = bs.key_num_symbols.get(&code).copied().unwrap_or(0);
                // Only fix keys whose level 1 was NOT explicitly defined
                // during parsing (i.e. came from DEFAULT_MAP). Skip keys
                // explicitly defined in the keymap, even if they have a
                // single symbol — those are handled at runtime via
                // shift_numlock_keys (Shift stays at level 0).
                if num_syms < 2 && !bs.explicit_keys.contains(&code) {
                    if let Some(value) = wkb.state_keymap.get(0).and_then(|m| m.get(&code).copied())
                    {
                        if wkb.state_keymap.len() > 1 {
                            wkb.state_keymap[1].insert(code, value);
                        }
                    }
                }
            }
        }
    }

    // ── 2. FOUR_LEVEL_X / KEYPAD-shifted keys ───────────────────────
    // For these keys, Shift doesn't change level in xkbcommon.
    // We store the original level 1 value in level_exceptions_keymap,
    // and copy level 0 value to state_keymap[1] for the corrected behavior.
    // Ensure level_exceptions_keymap has enough levels
    while wkb.level_exceptions_keymap.len() < wkb.state_keymap.len() {
        wkb.level_exceptions_keymap.push(BTreeMap::new());
    }

    // Handle FOUR_LEVEL_X keys
    for (&code, type_name) in &bs.key_types {
        if type_name != "FOUR_LEVEL_X" {
            continue;
        }
        // Save original values FIRST
        let orig_l0 = wkb.state_keymap.get(0).and_then(|m| m.get(&code)).copied();
        let orig_l1 = wkb.state_keymap.get(1).and_then(|m| m.get(&code)).copied();
        let orig_l2 = wkb.state_keymap.get(2).and_then(|m| m.get(&code)).copied();
        let orig_l3 = wkb.state_keymap.get(3).and_then(|m| m.get(&code)).copied();

        // Store original values in exceptions
        if let Some(l1) = orig_l1 {
            wkb.level_exceptions_keymap[1].insert(code, l1);
        }
        if let Some(l2) = orig_l2 {
            wkb.level_exceptions_keymap[2].insert(code, l2);
        }
        if let Some(l3) = orig_l3 {
            wkb.level_exceptions_keymap[3].insert(code, l3);
        }

        // Fix state_keymap for FOUR_LEVEL_X mapping:
        // - level 1: copy from original level 0 (Shift maps to Level1)
        // - level 2: copy from original level 1 (AltGr maps to Level2)
        // - level 3: copy from original level 2 (Shift+AltGr maps to Level3)
        if let Some(l0) = orig_l0 {
            if wkb.state_keymap.len() > 1 {
                wkb.state_keymap[1].insert(code, l0);
            }
        }
        if let Some(l1) = orig_l1 {
            if wkb.state_keymap.len() > 2 {
                wkb.state_keymap[2].insert(code, l1);
            }
        }
        if let Some(l2) = orig_l2 {
            if wkb.state_keymap.len() > 3 {
                wkb.state_keymap[3].insert(code, l2);
            }
        }
    }

    // Handle KEYPAD-shifted keys (auto-detected KEYPAD type)
    // Only fix keys that have the actual KEYPAD type (2 levels, Shift stays at level 0)
    // Skip FOUR_LEVEL_KEYPAD (4 levels, normal Shift behavior)
    // Skip keys with any explicit type
    for &code in &bs.keypad_shifted_keys {
        // Skip if has explicit type - xkbcommon handles these correctly
        if bs.key_types.get(&code).is_some() {
            continue;
        }
        // No explicit type - this is a true auto-detected KEYPAD key
        // For these, Shift stays at level 0
        // Store original level 1 value in exceptions
        // Skip keys with explicit KEYPAD types - these have normal Shift behavior
        if let Some(key_type) = bs.key_types.get(&code) {
            if key_type.contains("KEYPAD") || key_type.contains("FOUR_LEVEL") {
                continue;
            }
        }
        // Skip keys with explicit KEYPAD types - these have normal Shift behavior
        if let Some(key_type) = bs.key_types.get(&code) {
            if key_type.contains("KEYPAD") || key_type.contains("FOUR_LEVEL") {
                continue;
            }
        }
        // Skip keys with explicit KEYPAD types - these have normal Shift behavior
        if let Some(key_type) = bs.key_types.get(&code) {
            if key_type.contains("KEYPAD") || key_type.contains("FOUR_LEVEL_KEYPAD") {
                continue;
            }
        }
        // Store original level 1 value in exceptions
        if let Some(&l1) = wkb.state_keymap.get(1).and_then(|m| m.get(&code)) {
            wkb.level_exceptions_keymap[1].insert(code, l1);
        }
        // Copy level 0 to level 1 in state_keymap (corrected behavior)
        if let Some(&l0) = wkb.state_keymap.get(0).and_then(|m| m.get(&code)) {
            if wkb.state_keymap.len() > 1 {
                wkb.state_keymap[1].insert(code, l0);
            }
        }
    }

    // ── 3. ALPHABETIC keys ──────────────────────────────────────────
    // Only Shift and Lock affect the level; AltGr / Level5 are ignored.
    // Even levels (AltGr off) → Level 1, odd levels → Level 2.
    for (&code, type_name) in &bs.key_types {
        if type_name != "ALPHABETIC" {
            continue;
        }
        let l0 = wkb.state_keymap.get(0).and_then(|m| m.get(&code).copied());
        let l1 = wkb.state_keymap.get(1).and_then(|m| m.get(&code).copied());

        if let Some(v) = l0 {
            for i in (2..wkb.state_keymap.len()).step_by(2) {
                wkb.state_keymap[i].insert(code, v);
            }
        }
        if let Some(v) = l1 {
            for i in (3..wkb.state_keymap.len()).step_by(2) {
                wkb.state_keymap[i].insert(code, v);
            }
        }
    }
}

fn fix_xkb_edge_cases<C: Composer>(
    wkb: &mut WKB<C>,
    num_lock_codes: &mut Vec<u32>,
    locale: String,
    layout: Option<String>,
) {
    // ── Keymap fixups (compensating for WKB parser limitations) ──
    // These correct level_keymap values, num_lock_codes, etc. that WKB's
    // xkb-parser cannot derive correctly on its own.
    match (locale.as_str(), &layout.as_deref()) {
        // de:ru* key swap
        ("de", Some("ru")) | ("de", Some("ru-recom")) | ("de", Some("ru-translit")) => {
            for i in 0..2 {
                let map = wkb.state_keymap[i].clone();
                let t21 = map.get(&21);
                let t44 = map.get(&44);
                wkb.state_keymap[i].insert(21, *t44.unwrap());
                wkb.state_keymap[i].insert(44, *t21.unwrap());
            }
        }
        _ => {}
    };
    match (locale.as_str(), &layout.as_deref()) {
        // bqn: level_keymap fixups (caps metadata now auto-derived)
        ("bqn", Some("bqn")) => {
            wkb.state_keymap[1].insert(22, '⊔');
            wkb.state_keymap[1].insert(32, '↕');
            wkb.state_keymap[1].insert(36, '∘');
            wkb.state_keymap[1].insert(46, '↓');
            wkb.state_keymap[1].insert(57, '‿');
        }
        // fr:bepo_afnor: level_keymap resize
        ("fr", Some("bepo_afnor")) => {
            wkb.state_keymap.resize(4, BTreeMap::new());
        }
        // pl:glagolica: level_keymap fixup
        ("pl", Some("glagolica")) => {
            let value = *wkb.state_keymap[0].get(&40).unwrap();
            wkb.state_keymap[1].insert(40, value);
        }
        // apl: copy level0 to level1 for non-alphabetic keys
        ("apl", Some("apl2")) | ("apl", Some("aplplusII")) => {
            for i in 16..=25 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[1].insert(i, value);
            }
            for i in 30..=38 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[1].insert(i, value);
            }
            for i in 44..=50 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[1].insert(i, value);
            }
        }
        // ie:ogam level_keymap fixups
        ("ie", Some("ogam")) => {
            wkb.state_keymap[1].insert(43, '\u{1680}');
        }
        ("ie", Some("ogam_is434")) => {
            for (code, value) in wkb.state_keymap[2].clone() {
                if wkb.state_keymap[3].get(&code).is_none() {
                    wkb.state_keymap[3].insert(code, value);
                }
            }
            for (code, value) in wkb.state_keymap[0].clone() {
                if [2, 3, 4, 6, 7, 9, 10, 11, 12, 13].contains(&code) {
                    wkb.state_keymap[2].insert(code, value);
                    let value = *wkb.state_keymap[1].get(&code).unwrap();
                    wkb.state_keymap[3].insert(code, value);
                }
            }
        }
        // si: level_keymap copies
        ("si", Some(_)) => {
            let value = *wkb.state_keymap[0].get(&41).unwrap();
            wkb.state_keymap[2].insert(41, value);
            let value = *wkb.state_keymap[1].get(&41).unwrap();
            wkb.state_keymap[3].insert(41, value);
        }
        // se:rus level_keymap copies
        ("se", Some("rus")) => {
            let value = *wkb.state_keymap[0].get(&13).unwrap();
            wkb.state_keymap[2].insert(13, value);
            let value = *wkb.state_keymap[1].get(&13).unwrap();
            wkb.state_keymap[3].insert(13, value);
            for i in 16..=27 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[2].insert(i, value);
                let value = *wkb.state_keymap[1].get(&i).unwrap();
                wkb.state_keymap[3].insert(i, value);
            }
            for i in 30..=41 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[2].insert(i, value);
                let value = *wkb.state_keymap[1].get(&i).unwrap();
                wkb.state_keymap[3].insert(i, value);
            }
            for i in 43..=50 {
                let value = *wkb.state_keymap[0].get(&i).unwrap();
                wkb.state_keymap[2].insert(i, value);
                let value = *wkb.state_keymap[1].get(&i).unwrap();
                wkb.state_keymap[3].insert(i, value);
            }
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[2].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[3].insert(86, value);
        }
        // us:3l* level_keymap fixups
        ("us", Some("3l")) => {
            wkb.state_keymap[1].insert(2, '!');
            wkb.state_keymap[1].insert(3, '@');
            wkb.state_keymap[1].insert(4, '#');
            wkb.state_keymap[1].insert(5, '$');
            wkb.state_keymap[1].insert(6, '%');
            wkb.state_keymap[1].insert(7, '^');
            wkb.state_keymap[1].insert(8, '&');
            wkb.state_keymap[1].insert(9, '*');
            wkb.state_keymap[1].insert(10, '(');
            wkb.state_keymap[1].insert(11, ')');
            wkb.state_keymap[3].insert(2, '!');
            wkb.state_keymap[3].insert(3, '@');
            wkb.state_keymap[3].insert(4, '#');
            wkb.state_keymap[3].insert(5, '$');
            wkb.state_keymap[3].insert(6, '%');
            wkb.state_keymap[3].insert(7, '^');
            wkb.state_keymap[3].insert(8, '&');
            wkb.state_keymap[3].insert(9, '*');
            wkb.state_keymap[3].insert(10, '(');
            wkb.state_keymap[3].insert(11, ')');
            let value = *wkb.state_keymap[0].get(&15).unwrap();
            wkb.state_keymap[1].insert(15, value);
            wkb.state_keymap[2].insert(15, value);
            wkb.state_keymap[3].insert(15, value);
            wkb.state_keymap[4].insert(15, value);
            let value = *wkb.state_keymap[0].get(&58).unwrap();
            wkb.state_keymap[1].insert(58, value);
            wkb.state_keymap[3].insert(58, value);
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[4].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[5].insert(86, value);
            let value = *wkb.state_keymap[2].get(&86).unwrap();
            wkb.state_keymap[6].insert(86, value);
        }
        ("us", Some("3l-cros")) => {
            wkb.state_keymap[1].insert(2, '!');
            wkb.state_keymap[1].insert(3, '@');
            wkb.state_keymap[1].insert(4, '#');
            wkb.state_keymap[1].insert(5, '$');
            wkb.state_keymap[1].insert(6, '%');
            wkb.state_keymap[1].insert(7, '^');
            wkb.state_keymap[1].insert(8, '&');
            wkb.state_keymap[1].insert(9, '*');
            wkb.state_keymap[1].insert(10, '(');
            wkb.state_keymap[1].insert(11, ')');
            wkb.state_keymap[3].insert(2, '!');
            wkb.state_keymap[3].insert(3, '@');
            wkb.state_keymap[3].insert(4, '#');
            wkb.state_keymap[3].insert(5, '$');
            wkb.state_keymap[3].insert(6, '%');
            wkb.state_keymap[3].insert(7, '^');
            wkb.state_keymap[3].insert(8, '&');
            wkb.state_keymap[3].insert(9, '*');
            wkb.state_keymap[3].insert(10, '(');
            wkb.state_keymap[3].insert(11, ')');
            let value = *wkb.state_keymap[0].get(&15).unwrap();
            wkb.state_keymap[1].insert(15, value);
            wkb.state_keymap[2].insert(15, value);
            wkb.state_keymap[3].insert(15, value);
            wkb.state_keymap[4].insert(15, value);
            let value = *wkb.state_keymap[0].get(&58).unwrap();
            wkb.state_keymap[1].insert(58, value);
            wkb.state_keymap[3].insert(58, value);
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[4].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[5].insert(86, value);
            let value = *wkb.state_keymap[2].get(&86).unwrap();
            wkb.state_keymap[6].insert(86, value);
            let value = *wkb.state_keymap[0].get(&125).unwrap();
            wkb.state_keymap[1].insert(125, value);
            wkb.state_keymap[3].insert(125, value);
        }
        ("us", Some("3l-emacs")) => {
            wkb.state_keymap[1].insert(2, '!');
            wkb.state_keymap[1].insert(3, '@');
            wkb.state_keymap[1].insert(4, '#');
            wkb.state_keymap[1].insert(5, '$');
            wkb.state_keymap[1].insert(6, '%');
            wkb.state_keymap[1].insert(7, '^');
            wkb.state_keymap[1].insert(8, '&');
            wkb.state_keymap[1].insert(9, '*');
            wkb.state_keymap[1].insert(10, '(');
            wkb.state_keymap[1].insert(11, ')');
            wkb.state_keymap[3].insert(2, '!');
            wkb.state_keymap[3].insert(3, '@');
            wkb.state_keymap[3].insert(4, '#');
            wkb.state_keymap[3].insert(5, '$');
            wkb.state_keymap[3].insert(6, '%');
            wkb.state_keymap[3].insert(7, '^');
            wkb.state_keymap[3].insert(8, '&');
            wkb.state_keymap[3].insert(9, '*');
            wkb.state_keymap[3].insert(10, '(');
            wkb.state_keymap[3].insert(11, ')');
            let value = *wkb.state_keymap[0].get(&15).unwrap();
            wkb.state_keymap[1].insert(15, value);
            wkb.state_keymap[3].insert(15, value);
            let value = *wkb.state_keymap[0].get(&86).unwrap();
            wkb.state_keymap[4].insert(86, value);
            let value = *wkb.state_keymap[1].get(&86).unwrap();
            wkb.state_keymap[5].insert(86, value);
            let value = *wkb.state_keymap[2].get(&86).unwrap();
            wkb.state_keymap[6].insert(86, value);
        }
        // fr:bepo_latin9 — now handled generically by fix_key_type_levels (FOUR_LEVEL_X)
        // fr/be:oss_latin9 — now handled generically by fix_key_type_levels (FOUR_LEVEL_X)
        // fr:mac — now handled generically by fix_key_type_levels (KEYPAD auto-detect)
        // fr:afnor level_keymap fixups
        ("fr", Some("afnor")) => {
            while wkb.state_keymap.len() < 6 {
                wkb.state_keymap.push(BTreeMap::new());
            }
            for i in 0..2 {
                for (code, value) in &wkb.state_keymap[i].clone() {
                    if wkb.state_keymap[i + 4].get(&code).is_none() {
                        wkb.state_keymap[i + 4].insert(*code, *value);
                    }
                }
            }
            wkb.state_keymap[4].insert(86, '<');
            wkb.state_keymap[5].insert(55, '⋅');
            wkb.state_keymap[5].insert(74, '−');
            wkb.state_keymap[5].insert(86, '>');
            wkb.state_keymap[5].insert(98, '∕');
        }
        // de:T3 level_keymap fixups (caps metadata now auto-derived)
        ("de", Some("T3")) => {
            wkb.state_keymap[3].insert(2, 'ʹ');
            wkb.state_keymap[3].insert(3, 'ʺ');
            wkb.state_keymap[3].insert(4, 'ʿ');
            wkb.state_keymap[3].insert(5, 'ʾ');
            wkb.state_keymap[3].insert(6, 'ˁ');
            wkb.state_keymap[3].insert(7, 'ˀ');
            wkb.state_keymap[3].insert(8, '{');
            wkb.state_keymap[3].insert(9, '}');
            wkb.state_keymap[3].insert(10, '[');
            wkb.state_keymap[3].insert(11, ']');
            wkb.state_keymap[3].insert(12, 'ʻ');
            wkb.state_keymap[3].insert(13, '¬');
            wkb.state_keymap[3].insert(15, '\t');
            wkb.state_keymap[3].insert(16, '\u{30d}');
            wkb.state_keymap[3].insert(27, '@');
            wkb.state_keymap[3].insert(30, '\u{329}');
            wkb.state_keymap[3].insert(35, '\u{332}');
            wkb.state_keymap[3].insert(38, '\u{338}');
            wkb.state_keymap[3].insert(39, '°');
            wkb.state_keymap[3].insert(40, '′');
            wkb.state_keymap[3].insert(41, '|');
            wkb.state_keymap[3].insert(43, '″');
            wkb.state_keymap[3].insert(44, '«');
            wkb.state_keymap[3].insert(45, '»');
            wkb.state_keymap[3].insert(46, '―');
            wkb.state_keymap[3].insert(47, '‹');
            wkb.state_keymap[3].insert(48, '›');
            wkb.state_keymap[3].insert(49, '–');
            wkb.state_keymap[3].insert(50, '—');
            wkb.state_keymap[3].insert(51, '$');
            wkb.state_keymap[3].insert(52, '#');
            wkb.state_keymap[3].insert(53, '‑');
            wkb.state_keymap[3].insert(57, '\u{a0}');
        }
        // Extra level_keymap push for braille/apl/kr/jp layouts
        ("brai", Some("home_row"))
        | ("brai", Some("right_hand"))
        | ("brai", Some("keypad"))
        | ("apl", Some("common"))
        | ("kr", Some("hw_keys"))
        | ("jp", Some("hztg_escape"))
        | ("brai", Some("right_hand_invert")) => {
            wkb.state_keymap.push(BTreeMap::from([
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
            num_lock_codes.push(2);
            num_lock_codes.push(5);
            num_lock_codes.push(6);
            num_lock_codes.push(7);
        }
        ("ru", Some("ruintl_en")) => {
            wkb.state_keymap[3].insert(36, 'Ø');
        }
        ("cm", Some("azerty")) => {
            num_lock_codes.push(2);
            num_lock_codes.push(3);
            num_lock_codes.push(4);
            num_lock_codes.push(5);
            num_lock_codes.push(6);
            num_lock_codes.push(7);
            num_lock_codes.push(8);
            num_lock_codes.push(9);
            num_lock_codes.push(10);
            num_lock_codes.push(11);
        }
        ("la", Some("stea")) => num_lock_codes.retain(|k| k != &83),
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
        | ("de", Some("adnw"))
        | ("de", Some("noted")) => {
            num_lock_codes.clear();
        }
        _ => {}
    }
    if wkb.state_keymap.len() > 1 {
        for (code, value) in wkb.state_keymap[0].clone() {
            if wkb.state_keymap[1].get(&code).is_none() {
                wkb.state_keymap[1].insert(code, value);
            }
        }
    }
    for i in 0..wkb.state_keymap.len() {
        let next_large = std::cmp::min(i + 2, wkb.state_keymap.len() - 1);
        let next_small = std::cmp::min(i + 1, wkb.state_keymap.len() - 1);
        let next = std::cmp::max(next_small, next_large);
        if next > i {
            for (code, value) in &wkb.state_keymap[i].clone() {
                if wkb.state_keymap[next].get(&code).is_none() {
                    wkb.state_keymap[next].insert(*code, *value);
                }
            }
        }
    }
    for i in 0..std::cmp::min(wkb.state_keymap.len(), DEFAULT_MAP.len()) {
        for (code, value) in &DEFAULT_MAP[i] {
            if wkb.state_keymap[i].get(&code).is_none() {
                wkb.state_keymap[i].insert(*code, *value);
            }
        }
    }
}
