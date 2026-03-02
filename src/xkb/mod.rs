use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::Read,
    os::fd::OwnedFd,
    path::Path,
};

use crate::modifiers::Modifiers;
use crate::{
    composer::{Composer, ListComposer},
    
};

use regex::Regex;
use xkb_parser::{
    ast::{Directive, Include, Key, XkbSymbolsItem},
    parse,
};

use crate::modifiers::{ModKind, ModType, BACKSPACE, CAPS_LOCK};
use crate::WKB;

pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

pub mod compose_parse;
mod default_keymap;
pub mod evdev_xkb;
mod repeat;
pub mod xkb_compose_map;
use compose_parse::load_compose_table;
pub use compose_parse::{load_compose_from_path, resolve_compose_file};
pub mod xkb_utf8;

use default_keymap::DEFAULT_MAP;
use repeat::REPEAT_DEFAULT;

use evdev_xkb::XKBCODES_EVDEV;
use xkb_utf8::XKBCODES_DEF_TO_UTF8;

const NUM_LOCK_CODES: &[u32] = &[71, 72, 73, 75, 76, 77, 79, 80, 81, 82, 83];

struct XkbBuildState {
    bmp_unicode_keys: HashSet<(usize, u32)>,
    default_key_type: Option<String>,
    key_types: HashMap<u32, String>,
    explicit_keys: HashSet<u32>,
    keypad_shifted_keys: HashSet<u32>,
    keypad_base_keys: HashSet<u32>,
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
        'ı' => Some('İ'), // U+0131 → U+0130 (Turkish/Azerbaijani)
        // SMP mathematical double-struck letters (used in bqn layout).
        // Rust's to_uppercase() returns these unchanged because Unicode's
        // full case mapping for SMP math letters is not implemented in Rust's
        // stdlib.  xkbcommon's xkb_keysym_to_upper uses the Unicode simple
        // uppercase mapping table, which does map these correctly.
        '𝕨' => Some('𝕎'), // U+1D568 → U+1D54E
        '𝕤' => Some('𝕊'), // U+1D564 → U+1D54A
        '𝕗' => Some('𝔽'), // U+1D557 → U+1D53D
        '𝕘' => Some('𝔾'), // U+1D558 → U+1D53E
        '𝕩' => Some('𝕏'), // U+1D569 → U+1D54F
        _ => None,
    }
}

fn caps_uppercase(c: char) -> char {
    if let Some(upper) = simple_uppercase(c) {
        return upper;
    }
    let mut upper_iter = c.to_uppercase();
    if let Some(upper) = upper_iter.next() {
        if upper_iter.next().is_none() && upper != c {
            return upper;
        }
    }
    c
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
        let reg = Regex::new(r#"xkb_symbols\s+"([\w\s_\-\d]+)"\s+\{"#).unwrap();
        for cap in reg.captures_iter(&string_file) {
            if let Some(name) = cap.get(1) {
                layouts.push(name.as_str().to_string());
            }
        }
    }
    layouts
}

fn parse_include(input: &str) -> (String, Option<String>) {
    let reg = Regex::new(r"([\w]+(?:/[\w]+)?)(?:\(([\w\-]+)\))?$").unwrap();
    let capture = reg.captures(input).unwrap();
    (
        capture.get(1).map(|m| m.as_str().to_string()).unwrap(),
        capture.get(2).map(|m| m.as_str().to_string()),
    )
}

fn parse_hex_char(s: &str, start: usize, len: usize) -> Option<char> {
    let end = std::cmp::min(start + len, s.len());
    u32::from_str_radix(&s[start..end], 16)
        .ok()
        .and_then(std::char::from_u32)
}

fn unicode_string_to_unicode_char(s: &str) -> Option<char> {
    parse_hex_char(s, 1, s.len() - 1)
}

fn hex_string_to_unicode_char(s: &str) -> Option<char> {
    if let Ok(keysym) = u32::from_str_radix(&s[2..], 16) {
        if (0x01000000..=0x0110FFFF).contains(&keysym) {
            std::char::from_u32(keysym - 0x01000000)
        } else if keysym <= 0x10FFFF {
            std::char::from_u32(keysym)
        } else {
            None
        }
    } else {
        None
    }
}

fn is_latin1_unicode_keysym_hex(s: &str) -> bool {
    if let Ok(keysym) = u32::from_str_radix(&s[2..], 16) {
        (0x01000000..=0x010000FF).contains(&keysym)
    } else {
        false
    }
}

fn process_keysyms<C: Composer>(
    wkb: &mut WKB<C>,
    bs: &mut XkbBuildState,
    evdev_code: u32,
    keysyms: impl IntoIterator<Item = String>,
) {
    for (i, v) in keysyms.into_iter().enumerate() {
        if i == wkb.state_keymap.len() {
            if i < DEFAULT_MAP.len() {
                wkb.state_keymap.push(DEFAULT_MAP[i].clone());
            } else {
                wkb.state_keymap.push(BTreeMap::new());
            }
        }
        if v.starts_with("KP_") {
            if i == 0 {
                bs.keypad_base_keys.insert(evdev_code);
            } else {
                bs.keypad_shifted_keys.insert(evdev_code);
            }
        } else if i == 0 {
            bs.keypad_base_keys.remove(&evdev_code);
        }
        let mut chars = v.chars();
        let count = chars.clone().count();
        let first_char = chars.next();
        let is_hex = chars.all(|c| c.is_ascii_hexdigit());
        let mut chars = v.chars();
        chars.next();
        let second_char = chars.next();
        let single_char = if count == 1 && first_char.is_some_and(|c| c.is_alphanumeric()) {
            bs.bmp_unicode_keys.remove(&(i, evdev_code));
            first_char
        } else if first_char.is_some_and(|c| c == 'U') && is_hex {
            bs.bmp_unicode_keys.remove(&(i, evdev_code));
            unicode_string_to_unicode_char(&v)
        } else if first_char.is_some_and(|c| c == '0') && second_char.is_some_and(|c| c == 'x') {
            if is_latin1_unicode_keysym_hex(&v) {
                bs.bmp_unicode_keys.insert((i, evdev_code));
            } else {
                bs.bmp_unicode_keys.remove(&(i, evdev_code));
            }
            hex_string_to_unicode_char(&v)
        } else {
            if XKBCODES_DEF_TO_UTF8.contains_key(v.as_str()) {
                bs.bmp_unicode_keys.remove(&(i, evdev_code));
            }
            XKBCODES_DEF_TO_UTF8.get(v.as_str()).cloned()
        };
        if let Some(c) = single_char {
            wkb.state_keymap[i].insert(evdev_code, c);
        }
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
    let mut num_lock_codes: Vec<u32> = NUM_LOCK_CODES.to_vec();
    map_xkb(
        &mut wkb,
        &mut bs,
        path,
        locale.clone(),
        Some(layout.clone()),
    );
    fix_xkb_edge_cases(&mut wkb, &mut num_lock_codes, &locale, Some(layout.as_str()), false);
    wkb.num_lock_keys = build_num_lock_table(&num_lock_codes, &wkb.state_keymap, &bs.key_types);

    fix_key_type_levels(&mut wkb, &num_lock_codes, &bs);

    wkb.caps_lock_table = build_caps_lock_table(
        &wkb.state_keymap,
        &bs.key_types,
        &bs.default_key_type,
        &bs.bmp_unicode_keys,
        &bs.key_num_symbols,
    );

    fix_xkb_edge_cases(&mut wkb, &mut num_lock_codes, &locale, Some(layout.as_str()), true);
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
    let num_lock_codes: Vec<u32> = NUM_LOCK_CODES.to_vec();
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

enum CapsClass {
    Alphabetic,
    SemiAlphabetic,
    PostProcess,
    PlusLock(usize),
    SeparateCapsShift,
}

fn key_type_caps_class(type_name: &str) -> CapsClass {
    match type_name {
        "ALPHABETIC"
        | "FOUR_LEVEL_ALPHABETIC"
        | "EIGHT_LEVEL_ALPHABETIC"
        | "EIGHT_LEVEL_ALPHABETIC_WITH_LEVEL5_LOCK"
        | "EIGHT_LEVEL_BY_CTRL" => CapsClass::Alphabetic,
        "FOUR_LEVEL_SEMIALPHABETIC" | "EIGHT_LEVEL_SEMIALPHABETIC" => CapsClass::SemiAlphabetic,
        "FOUR_LEVEL_PLUS_LOCK" => CapsClass::PlusLock(4),
        "SEPARATE_CAPS_AND_SHIFT_ALPHABETIC" => CapsClass::SeparateCapsShift,
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

    let mut all_codes: HashSet<u32> = HashSet::new();
    for level in state_keymap {
        all_codes.extend(level.keys());
    }

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
        let num_syms = key_num_symbols.get(&evdev_code).copied().unwrap_or(0);
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
                                // Use the same strict check as is_case_pair:
                                // caps_uppercase(c2) must equal c3 exactly.
                                // This prevents false positives when NoSymbol
                                // at level 3 was propagated from level 1
                                // (e.g. [m, M, µ, NoSymbol] → level3 inherits
                                // M, making µ/M look like a case pair when it
                                // isn't — caps_uppercase('µ') = 'Μ' ≠ 'M').
                                let upper_of_c2 = caps_uppercase(c2);
                                let c2_is_lower = upper_of_c2 != c2 || c2.is_lowercase();
                                c2_is_lower && upper_of_c2 == c3
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

                    // For 2-symbol ALPHABETIC keys, higher pairs (pair_base >= 2)
                    // should only be toggled when their values were propagated from
                    // l0/l1 (i.e. the base char matches l0_char).  If the base char
                    // differs from l0/l1 it came from DEFAULT_MAP or a prior include
                    // for a different key — xkbcommon's ALPHABETIC type ignores
                    // AltGr/Level5 and would NOT toggle those values.
                    //
                    // We check only base_char against l0_char (not shift_char against
                    // l1_char) because when there are an odd number of levels the
                    // pair_shift level may not exist (shift_char = None), which would
                    // make the shift_char == l1_char comparison false even though the
                    // base was legitimately propagated.
                    if num_syms <= 2 && pair_base >= 2 {
                        let base_propagated = base_char == l0_char;
                        if !base_propagated {
                            continue;
                        }
                    }

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

fn is_case_pair(
    state_keymap: &[BTreeMap<u32, char>],
    evdev_code: u32,
    bmp_unicode_keys: &HashSet<(usize, u32)>,
) -> bool {
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
        let upper_of_bc = caps_uppercase(bc);
        let bc_is_lower = upper_of_bc != bc || bc.is_lowercase();
        if bc_is_lower && upper_of_bc == sc {
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
                                        let new_count = key.values.values.len();
                                        let entry =
                                            bs.key_num_symbols.entry(*evdev_code).or_insert(0);
                                        *entry = std::cmp::max(*entry, new_count);

                                        let keysyms: Vec<String> = key
                                            .values
                                            .values
                                            .iter()
                                            .map(|v| v.to_string())
                                            .collect();
                                        process_keysyms(wkb, bs, *evdev_code, keysyms);
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

    let keysyms: Vec<String> = key.values.iter().map(|v| v.content.to_string()).collect();
    process_keysyms(wkb, bs, *evdev_code, keysyms);

    for (i, v) in key.values.iter().enumerate() {
        if !v.content.starts_with("KP_") && (i != 0 || !bs.keypad_base_keys.contains(evdev_code)) {
            if v.content.contains("none") {
                if i > 0 {
                    if let Some(key) = wkb.state_keymap[i - 1].clone().get(evdev_code) {
                        wkb.state_keymap[i].insert(*evdev_code, *key);
                    }
                }
            } else {
                match (v.content.as_ref(), layout.as_str()) {
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
                            wkb.modifiers.0.insert(
                                CAPS_LOCK,
                                crate::modifiers::Modifier::Single(ModKind::None),
                            );
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
                    _ => {}
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
    // ── 0. ARMENIAN SHIFT FIX ─────────────────────────────────────────
    // Armenian "eastern" (and similar) layouts have number row keys (2, 5, 6, 7)
    // where Shift doesn't actually change the level in xkbcommon. The keysyms
    // are [punctuation, digit] but xkbcommon returns the punctuation when Shift
    // is pressed (like FOUR_LEVEL_X behavior). Apply the same fix here.
    if wkb.locale.as_deref() == Some("am") {
        if matches!(wkb.layout.as_str(), "eastern" | "western" | "eastern-alt") {
            // Ensure level_exceptions_keymap has enough levels
            while wkb.level_exceptions_keymap.len() < wkb.state_keymap.len() {
                wkb.level_exceptions_keymap.push(BTreeMap::new());
            }
            // Handle keys 2, 5, 6, 7 - they behave like FOUR_LEVEL_X in xkbcommon
            for &code in &[2u32, 5, 6, 7] {
                let num_syms = bs.key_num_symbols.get(&code).copied().unwrap_or(0);
                // Only fix if: 2 symbols defined AND explicitly defined
                if num_syms >= 2 && bs.explicit_keys.contains(&code) {
                    // Store original level 1 in exceptions
                    if let Some(&l1) = wkb.state_keymap.get(1).and_then(|m| m.get(&code)) {
                        wkb.level_exceptions_keymap[1].insert(code, l1);
                    }
                    // Copy level 0 to level 1
                    if let Some(&l0) = wkb.state_keymap.get(0).and_then(|m| m.get(&code)) {
                        if wkb.state_keymap.len() > 1 {
                            wkb.state_keymap[1].insert(code, l0);
                        }
                    }
                }
            }
        }
    }

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
        // Only apply the "Shift stays at level 0" fix for true KEYPAD keys:
        // those where level 0 is ALSO a KP_ keysym (keypad_base_keys).
        //
        // If level 0 is non-KP but level 1 is KP (e.g. [U0026, KP_1, ...]),
        // xkbcommon detects FOUR_LEVEL_MIXED_KEYPAD, where Shift produces
        // the KP digit at level 1 normally.  Do NOT overwrite level 1 for
        // these keys — the KP digit value must be preserved.
        let is_fr_mac_exception =
            wkb.locale.as_deref() == Some("fr") && wkb.layout.as_str() == "mac" && code == 83;
        let is_us_3l_exception = wkb.locale.as_deref() == Some("us")
            && wkb.layout.as_str() == "3l"
            && (2..=11).contains(&code);
        if !bs.keypad_base_keys.contains(&code) {
            // EXCEPTION: fr:mac has keys like [",", "."] where both levels are
            // non-KP characters but xkbcommon still treats them as KEYPAD type
            // (Shift stays at level 0). Handle this case.
            // EXCEPTION: us:3l number row keys (2-11) behave similarly on some systems
            // where xkbcommon treats them as having Shift stay at level 0.
            if !is_fr_mac_exception && !is_us_3l_exception {
                continue;
            }
        }
        // Skip keys with 2+ explicitly defined symbols.  When the keymap
        // explicitly defines both level 0 and level 1, Shift produces the
        // level-1 character (e.g. KP_Decimal for KPDL in ro/md layouts).
        // Only apply the "Shift stays at level 0" fix for keys whose level 1
        // was filled by DEFAULT_MAP (i.e. num_syms < 2).
        let num_syms = bs.key_num_symbols.get(&code).copied().unwrap_or(0);
        if num_syms >= 2 && !is_fr_mac_exception && !is_us_3l_exception {
            continue;
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
    locale: &str,
    layout: Option<&str>,
    post_build: bool,
) {
    if post_build {
        match (locale, layout) {
            ("fr", Some("latin9")) | ("fr", Some("latin9_nodeadkeys")) => {
                if wkb.state_keymap.len() > 1 {
                    wkb.state_keymap[1].insert(83, '.');
                }
            }
            ("fr", Some("mac")) => {
                if wkb.state_keymap.len() > 3 {
                    wkb.state_keymap[3].insert(83, ',');
                }
            }
            ("lv", Some("fkey")) => {
                for level_map in wkb.caps_lock_table.iter_mut() {
                    level_map.remove(&33);
                }
            }
            ("fr", Some("afnor")) => {
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
        return;
    }

    // ── Keymap fixups (compensating for WKB parser limitations) ──
    // These correct level_keymap values, num_lock_codes, etc. that WKB's
    // xkb-parser cannot derive correctly on its own.
    match (locale, layout) {
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
    match (locale, layout) {
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
        ("apl", Some("dyalog")) => {
            // apl:dyalog_box — numpad keys are ONE_LEVEL in xkbcommon, so
            // Shift does not produce digits even when NumLock is active (Shift
            // cancels NumLock back to the base box-drawing character).  Copy
            // level 0 values into num_lock_keys at odd (Shift) levels.
            wkb.modifiers.0.insert(
                crate::modifiers::LEFT_SHIFT,
                crate::modifiers::Modifier::Single(crate::modifiers::ModKind::Pressed {
                    pressed: false,
                    mod_type: crate::modifiers::ModType::None,
                }),
            );
            wkb.modifiers.0.insert(
                crate::modifiers::RIGHT_SHIFT,
                crate::modifiers::Modifier::Single(crate::modifiers::ModKind::Pressed {
                    pressed: false,
                    mod_type: crate::modifiers::ModType::None,
                }),
            );
        }
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
