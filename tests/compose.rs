use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use wkb::xkb::xkb_utf8::XKBCODES_DEF_TO_UTF8;
use xkbcommon::xkb::{self, compose};

// ---------------------------------------------------------------------------
// Helpers: keysym / char resolution
// ---------------------------------------------------------------------------

/// Resolve a keysym name to a char the same way wkb's load_compose_table does.
fn resolve_keysym_to_char(name: &str) -> Option<char> {
    if let Some(&c) = XKBCODES_DEF_TO_UTF8.get(name) {
        return Some(c);
    }
    if name.len() == 1 {
        return Some(name.chars().next().unwrap());
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

/// Resolve a keysym name to a UTF-8 character via xkbcommon.
fn resolve_output_via_keysym(name: &str) -> Option<char> {
    let keysym = xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS);
    if keysym.raw() == 0 {
        return None;
    }
    let utf = xkb::keysym_to_utf8(keysym);
    if utf.is_empty() {
        return None;
    }
    utf.chars().next()
}

// ---------------------------------------------------------------------------
// Compose file parsing (test-side, mirrors the main library logic)
// ---------------------------------------------------------------------------

/// A parsed compose entry.
#[derive(Debug, Clone)]
struct ComposeEntry {
    keysym_names: Vec<String>,
    is_multi_key: bool,
    output: Option<char>,
    output_keysym_name: Option<String>,
}

impl ComposeEntry {
    fn resolved_output(&self) -> Option<char> {
        if let Some(c) = self.output {
            return Some(c);
        }
        self.output_keysym_name
            .as_deref()
            .and_then(resolve_output_via_keysym)
    }
}

fn parse_compose_file(path: &Path) -> Vec<ComposeEntry> {
    let mut entries = Vec::new();
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return entries,
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
        if trimmed.starts_with("include") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let include_path = &trimmed[start + 1..start + 1 + end];
                    let included = parse_compose_file(Path::new(include_path));
                    entries.extend(included);
                }
            }
            continue;
        }
        if !trimmed.starts_with('<') {
            continue;
        }
        if let Some(entry) = parse_compose_line(trimmed) {
            entries.push(entry);
        }
    }

    entries
}

fn parse_compose_line(line: &str) -> Option<ComposeEntry> {
    let (keys_part, value_part) = line.split_once(':')?;
    let keys_str = keys_part.trim();
    let value_str = value_part.trim();

    let mut keysym_names: Vec<String> = Vec::new();
    let mut is_multi_key = false;

    for token in keys_str.split_whitespace() {
        let name = token.trim_start_matches('<').trim_end_matches('>');
        if name == "Multi_key" {
            if keysym_names.is_empty() {
                is_multi_key = true;
            } else {
                return None;
            }
            continue;
        }
        keysym_names.push(name.to_string());
    }

    if keysym_names.is_empty() {
        return None;
    }

    let (output, output_keysym_name) = parse_output(value_str);

    if output.is_none() && output_keysym_name.is_none() {
        return None;
    }

    Some(ComposeEntry {
        keysym_names,
        is_multi_key,
        output,
        output_keysym_name,
    })
}

fn parse_output(value_str: &str) -> (Option<char>, Option<String>) {
    let mut output_char = None;
    let mut keysym_name = None;

    if value_str.starts_with('"') {
        let rest = &value_str[1..];
        if let Some(end_quote) = rest.find('"') {
            let inner = &rest[..end_quote];
            if !inner.starts_with('\\') && !inner.is_empty() {
                output_char = inner.chars().next();
            }
            let after_quote = rest[end_quote + 1..].trim();
            keysym_name = parse_keysym_name(after_quote);
        }
    }

    (output_char, keysym_name)
}

fn parse_keysym_name(after_quote: &str) -> Option<String> {
    let trimmed = after_quote.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let name = trimmed.split_whitespace().next()?;
    if name.starts_with('#') {
        return None;
    }
    Some(name.to_string())
}

// ---------------------------------------------------------------------------
// Compose sequence testers
// ---------------------------------------------------------------------------

/// Feed a sequence of keysyms to an xkb compose state.
fn xkb_compose_sequence(
    compose_state: &mut compose::State,
    keysym_names: &[String],
    is_multi_key: bool,
) -> Option<char> {
    compose_state.reset();
    if is_multi_key {
        let multi_keysym = xkb::keysym_from_name("Multi_key", xkb::KEYSYM_NO_FLAGS);
        compose_state.feed(multi_keysym);
    }
    for name in keysym_names {
        let keysym = xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS);
        if keysym.raw() == 0 {
            compose_state.reset();
            return None;
        }
        compose_state.feed(keysym);
    }
    if compose_state.status() == compose::Status::Composed {
        compose_state.utf8().and_then(|s| s.chars().next())
    } else {
        None
    }
}

/// Feed a sequence of chars to a wkb ListComposer clone.
fn wkb_compose_sequence(composer: &wkb::composer::ListComposer, chars: &[char]) -> Option<char> {
    use wkb::composer::{ComposeState, Composer};
    let mut c = composer.clone();
    let mut result = None;
    for &ch in chars {
        match c.feed(ch) {
            ComposeState::Finished(out) => {
                result = Some(out);
            }
            ComposeState::Idle(_) | ComposeState::Composing(_) => {}
            ComposeState::Cancelled(_) => {
                return None;
            }
        }
    }
    result
}

/// Resolve a compose entry's keysym names to chars for wkb.
fn resolve_entry_chars(entry: &ComposeEntry) -> Option<Vec<char>> {
    let mut chars = Vec::new();
    for name in &entry.keysym_names {
        chars.push(resolve_keysym_to_char(name)?);
    }
    Some(chars)
}

// ---------------------------------------------------------------------------
// Core test logic
// ---------------------------------------------------------------------------

/// Run compose tests for a compose file loaded from a pair of composers.
///
/// `label` is used for log/assertion messages.
/// `xkb_locale` is the locale to pass to xkbcommon for cross-checking.
/// `compose_path` is the path to the compose file to parse entries from.
/// `regular` and `compose_key` are the wkb composers to test against.
fn run_compose_test(
    label: &str,
    xkb_locale: &str,
    compose_path: &Path,
    regular: &wkb::composer::ListComposer,
    compose_key: &wkb::composer::ListComposer,
) {
    if !compose_path.exists() {
        println!("SKIP: compose file not found: {}", compose_path.display());
        return;
    }

    let entries = parse_compose_file(compose_path);
    if entries.is_empty() {
        println!("SKIP: no entries in {}", compose_path.display());
        return;
    }

    // Build xkb compose state for cross-checking
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let xkb_state = compose::Table::new_from_locale(
        &context,
        OsStr::new(xkb_locale),
        compose::COMPILE_NO_FLAGS,
    )
    .ok()
    .map(|table| compose::State::new(&table, compose::STATE_NO_FLAGS));

    let has_xkb = xkb_state.is_some();
    let mut xkb_state = xkb_state;

    println!(
        "{}: {} total entries, xkb={}",
        label,
        entries.len(),
        if has_xkb { "yes" } else { "no" },
    );

    // Detect char-sequence collisions
    let mut char_seq_outputs: HashMap<(bool, Vec<char>), char> = HashMap::new();
    let mut collision_seqs: std::collections::HashSet<(bool, Vec<char>)> = Default::default();
    for entry in &entries {
        if let (Some(chars), Some(output)) = (resolve_entry_chars(entry), entry.resolved_output()) {
            let key = (entry.is_multi_key, chars);
            if let Some(&prev) = char_seq_outputs.get(&key) {
                if prev != output {
                    collision_seqs.insert(key);
                }
            } else {
                char_seq_outputs.insert(key, output);
            }
        }
    }

    // Detect prefix conflicts
    let all_char_seqs: Vec<(bool, Vec<char>)> = entries
        .iter()
        .filter_map(|e| resolve_entry_chars(e).map(|chars| (e.is_multi_key, chars)))
        .collect();
    let mut prefix_conflict_seqs: std::collections::HashSet<(bool, Vec<char>)> = Default::default();
    for a in &all_char_seqs {
        for b in &all_char_seqs {
            if a.0 == b.0 && b.1.len() > a.1.len() && b.1.starts_with(&a.1) {
                prefix_conflict_seqs.insert(a.clone());
                prefix_conflict_seqs.insert(b.clone());
            }
        }
    }

    let mut xkb_ok = 0usize;
    let mut wkb_ok = 0usize;
    let mut both_ok = 0usize;
    let mut both_match = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    let mut char_collisions: Vec<String> = Vec::new();
    let mut wkb_failures: Vec<String> = Vec::new();
    let mut xkb_failures: Vec<String> = Vec::new();
    let mut wkb_only_ok = 0usize;

    for entry in &entries {
        let expected = match entry.resolved_output() {
            Some(c) => c,
            None => continue,
        };

        let xkb_result = xkb_state
            .as_mut()
            .and_then(|state| xkb_compose_sequence(state, &entry.keysym_names, entry.is_multi_key));

        let wkb_result = if let Some(chars) = resolve_entry_chars(entry) {
            let composer = if entry.is_multi_key {
                compose_key
            } else {
                regular
            };
            wkb_compose_sequence(composer, &chars)
        } else {
            None
        };

        if has_xkb {
            if xkb_result.is_some() {
                xkb_ok += 1;
            }
            if wkb_result.is_some() {
                wkb_ok += 1;
            }

            if xkb_result.is_some() && wkb_result.is_some() {
                both_ok += 1;
                if xkb_result == wkb_result {
                    both_match += 1;
                } else {
                    let msg = format!(
                        "  MISMATCH: {:?} [multi={}] -> xkb={:?} wkb={:?} expected={:?}",
                        entry.keysym_names, entry.is_multi_key, xkb_result, wkb_result, expected
                    );
                    let is_known = resolve_entry_chars(entry)
                        .map(|chars| {
                            let key = (entry.is_multi_key, chars);
                            collision_seqs.contains(&key) || prefix_conflict_seqs.contains(&key)
                        })
                        .unwrap_or(false);
                    if is_known {
                        char_collisions.push(msg);
                    } else {
                        mismatches.push(msg);
                    }
                }
            } else if xkb_result.is_some() && wkb_result.is_none() {
                wkb_failures.push(format!(
                    "  WKB_MISS: {:?} [multi={}] -> xkb={:?} expected={:?}",
                    entry.keysym_names, entry.is_multi_key, xkb_result, expected
                ));
            } else if xkb_result.is_none() && wkb_result.is_some() {
                xkb_failures.push(format!(
                    "  XKB_MISS: {:?} [multi={}] -> wkb={:?} expected={:?}",
                    entry.keysym_names, entry.is_multi_key, wkb_result, expected
                ));
            }
        } else {
            match wkb_result {
                Some(c) if c == expected => wkb_only_ok += 1,
                Some(c) => {
                    mismatches.push(format!(
                        "  WKB_WRONG: {:?} [multi={}] -> wkb={:?} expected={:?}",
                        entry.keysym_names, entry.is_multi_key, c, expected
                    ));
                }
                None => {
                    wkb_failures.push(format!(
                        "  WKB_MISS: {:?} [multi={}] -> expected={:?}",
                        entry.keysym_names, entry.is_multi_key, expected
                    ));
                }
            }
        }
    }

    if has_xkb {
        println!(
            "  xkb_ok={}, wkb_ok={}, both_ok={}, both_match={}",
            xkb_ok, wkb_ok, both_ok, both_match
        );
    } else {
        println!("  wkb_only_ok={}", wkb_only_ok);
    }
    println!(
        "  mismatches={}, known_collisions={}, prefix_conflicts={}, wkb_miss={}, xkb_miss={}",
        mismatches.len(),
        char_collisions.len(),
        prefix_conflict_seqs.len(),
        wkb_failures.len(),
        xkb_failures.len()
    );

    for m in mismatches.iter().take(20) {
        println!("{}", m);
    }
    if mismatches.len() > 20 {
        println!("  ... and {} more mismatches", mismatches.len() - 20);
    }
    for m in char_collisions.iter().take(5) {
        println!("  (known collision) {}", m);
    }
    if char_collisions.len() > 5 {
        println!(
            "  ... and {} more known collisions",
            char_collisions.len() - 5
        );
    }
    for m in wkb_failures.iter().take(10) {
        println!("{}", m);
    }
    if wkb_failures.len() > 10 {
        println!("  ... and {} more wkb failures", wkb_failures.len() - 10);
    }

    assert_eq!(
        mismatches.len(),
        0,
        "{}: {} true mismatches (excluding {} known collisions)",
        label,
        mismatches.len(),
        char_collisions.len()
    );

    assert_eq!(
        wkb_failures.len(),
        0,
        "{}: {} entries where compose was expected but wkb did not produce output",
        label,
        wkb_failures.len()
    );
}

// ---------------------------------------------------------------------------
// Test via WKB::new_from_names — exercises the full locale-to-compose
// resolution pipeline.
// ---------------------------------------------------------------------------

/// Test compose for an XKB locale created via `WKB::new_from_names`.
///
/// 1. Creates a full WKB instance (keymap + compose tables).
/// 2. Resolves the expected compose file via `resolve_compose_file`.
/// 3. Parses the compose file and tests every entry against the WKB's
///    composers, cross-checked with xkbcommon.
fn test_wkb_compose(xkb_locale: &str) {
    let compose_file_subpath = wkb::xkb::resolve_compose_file(xkb_locale).unwrap_or_else(|| {
        panic!(
            "resolve_compose_file('{}') returned None — \
                 add an entry to xkb_compose_map.rs",
            xkb_locale
        )
    });

    let wkb = wkb::WKB::new_from_names(xkb_locale.to_string(), None);

    let compose_path = Path::new("/usr/share/X11/locale").join(&compose_file_subpath);
    println!(
        "Testing XKB locale '{}' -> compose file '{}'",
        xkb_locale, compose_file_subpath
    );

    // Determine the xkbcommon locale for cross-checking.
    // Try the UTF-8 full locale first; for locales already containing
    // a dot (like full locale names) use as-is.
    let xkb_locale_full = if xkb_locale.contains('.') {
        xkb_locale.to_string()
    } else {
        // Derive from compose file subpath: e.g. "en_US.UTF-8/Compose" → "en_US.UTF-8"
        compose_file_subpath
            .strip_suffix("/Compose")
            .unwrap_or(xkb_locale)
            .to_string()
    };

    run_compose_test(
        &format!("wkb({})", xkb_locale),
        &xkb_locale_full,
        &compose_path,
        &wkb.regular_composer,
        &wkb.compose_key_composer,
    );
}

/// Test compose for a compose file loaded directly via
/// `load_compose_from_path`.  Used for compose files not reachable
/// through any short XKB locale name.
fn test_compose_file_direct(label: &str, xkb_locale: &str, compose_file: &str) {
    let compose_path = Path::new(compose_file);
    if !compose_path.exists() {
        println!("SKIP: compose file not found: {}", compose_file);
        return;
    }

    let (regular, compose_key) = wkb::xkb::load_compose_from_path(compose_path);

    run_compose_test(label, xkb_locale, compose_path, &regular, &compose_key);
}

// ===================================================================
// Part 1: XKB locales via WKB::new_from_names
//
// These tests exercise the full pipeline: locale resolution through
// locale.alias + compose.dir, compose file loading, and trie building.
//
// Each test covers a unique compose file reachable through a short
// XKB locale name.
// ===================================================================

// --- en_US.UTF-8/Compose (the most common; many locales map here) ---
// Locales resolving via locale.alias + UTF-8 fallback:

#[test]
fn compose_wkb_de() {
    test_wkb_compose("de");
}

#[test]
fn compose_wkb_fr() {
    test_wkb_compose("fr");
}

#[test]
fn compose_wkb_es() {
    test_wkb_compose("es");
}

#[test]
fn compose_wkb_it() {
    test_wkb_compose("it");
}

#[test]
fn compose_wkb_nl() {
    test_wkb_compose("nl");
}

#[test]
fn compose_wkb_pl() {
    test_wkb_compose("pl");
}

#[test]
fn compose_wkb_ro() {
    test_wkb_compose("ro");
}

#[test]
fn compose_wkb_tr() {
    test_wkb_compose("tr");
}

#[test]
fn compose_wkb_hr() {
    test_wkb_compose("hr");
}

#[test]
fn compose_wkb_hu() {
    test_wkb_compose("hu");
}

#[test]
fn compose_wkb_sk() {
    test_wkb_compose("sk");
}

#[test]
fn compose_wkb_is() {
    test_wkb_compose("is");
}

#[test]
fn compose_wkb_lt() {
    test_wkb_compose("lt");
}

#[test]
fn compose_wkb_lv() {
    test_wkb_compose("lv");
}

#[test]
fn compose_wkb_mk() {
    test_wkb_compose("mk");
}

#[test]
fn compose_wkb_mt() {
    test_wkb_compose("mt");
}

#[test]
fn compose_wkb_si() {
    test_wkb_compose("si");
}

#[test]
fn compose_wkb_bg() {
    test_wkb_compose("bg");
}

#[test]
fn compose_wkb_af() {
    test_wkb_compose("af");
}

#[test]
fn compose_wkb_az() {
    test_wkb_compose("az");
}

#[test]
fn compose_wkb_be() {
    test_wkb_compose("be");
}

#[test]
fn compose_wkb_br() {
    test_wkb_compose("br");
}

#[test]
fn compose_wkb_ca() {
    test_wkb_compose("ca");
}

#[test]
fn compose_wkb_et() {
    test_wkb_compose("et");
}

#[test]
fn compose_wkb_eu() {
    test_wkb_compose("eu");
}

#[test]
fn compose_wkb_fo() {
    test_wkb_compose("fo");
}

#[test]
fn compose_wkb_id() {
    test_wkb_compose("id");
}

#[test]
fn compose_wkb_ml() {
    test_wkb_compose("ml");
}

#[test]
fn compose_wkb_ph() {
    test_wkb_compose("ph");
}

#[test]
fn compose_wkb_tg() {
    test_wkb_compose("tg");
}

#[test]
fn compose_wkb_uz() {
    test_wkb_compose("uz");
}

// Locales resolved via xkb_compose_map (XKB name ≠ language code):

#[test]
fn compose_wkb_us() {
    test_wkb_compose("us");
}

#[test]
fn compose_wkb_gb() {
    test_wkb_compose("gb");
}

#[test]
fn compose_wkb_au() {
    test_wkb_compose("au");
}

#[test]
fn compose_wkb_nz() {
    test_wkb_compose("nz");
}

#[test]
fn compose_wkb_za() {
    test_wkb_compose("za");
}

#[test]
fn compose_wkb_bw() {
    test_wkb_compose("bw");
}

#[test]
fn compose_wkb_no() {
    test_wkb_compose("no");
}

#[test]
fn compose_wkb_dk() {
    test_wkb_compose("dk");
}

#[test]
fn compose_wkb_se() {
    test_wkb_compose("se");
}

#[test]
fn compose_wkb_at() {
    test_wkb_compose("at");
}

#[test]
fn compose_wkb_ch() {
    test_wkb_compose("ch");
}

#[test]
fn compose_wkb_al() {
    test_wkb_compose("al");
}

#[test]
fn compose_wkb_ba() {
    test_wkb_compose("ba");
}

#[test]
fn compose_wkb_by() {
    test_wkb_compose("by");
}

#[test]
fn compose_wkb_ge() {
    test_wkb_compose("ge");
}

#[test]
fn compose_wkb_ua() {
    test_wkb_compose("ua");
}

#[test]
fn compose_wkb_in() {
    test_wkb_compose("in");
}

#[test]
fn compose_wkb_bd() {
    test_wkb_compose("bd");
}

#[test]
fn compose_wkb_np() {
    test_wkb_compose("np");
}

#[test]
fn compose_wkb_pk() {
    test_wkb_compose("pk");
}

#[test]
fn compose_wkb_il() {
    test_wkb_compose("il");
}

#[test]
fn compose_wkb_vn() {
    test_wkb_compose("vn");
}

#[test]
fn compose_wkb_my() {
    test_wkb_compose("my");
}

#[test]
fn compose_wkb_ie() {
    test_wkb_compose("ie");
}

#[test]
fn compose_wkb_epo() {
    test_wkb_compose("epo");
}

#[test]
fn compose_wkb_latam() {
    test_wkb_compose("latam");
}

// Arabic-script countries (via xkb_compose_map):

#[test]
fn compose_wkb_ara() {
    test_wkb_compose("ara");
}

#[test]
fn compose_wkb_iq() {
    test_wkb_compose("iq");
}

#[test]
fn compose_wkb_ir() {
    test_wkb_compose("ir");
}

#[test]
fn compose_wkb_sy() {
    test_wkb_compose("sy");
}

#[test]
fn compose_wkb_eg() {
    test_wkb_compose("eg");
}

#[test]
fn compose_wkb_dz() {
    test_wkb_compose("dz");
}

#[test]
fn compose_wkb_ma() {
    test_wkb_compose("ma");
}

// Central Asian (via xkb_compose_map):

#[test]
fn compose_wkb_kg() {
    test_wkb_compose("kg");
}

#[test]
fn compose_wkb_kz() {
    test_wkb_compose("kz");
}

#[test]
fn compose_wkb_tj() {
    test_wkb_compose("tj");
}

#[test]
fn compose_wkb_la() {
    test_wkb_compose("la");
}

// Countries with en_US.UTF-8 fallback (via xkb_compose_map):

#[test]
fn compose_wkb_bt() {
    test_wkb_compose("bt");
}

#[test]
fn compose_wkb_cd() {
    test_wkb_compose("cd");
}

#[test]
fn compose_wkb_cm() {
    test_wkb_compose("cm");
}

#[test]
fn compose_wkb_gh() {
    test_wkb_compose("gh");
}

#[test]
fn compose_wkb_gn() {
    test_wkb_compose("gn");
}

#[test]
fn compose_wkb_ke() {
    test_wkb_compose("ke");
}

#[test]
fn compose_wkb_md() {
    test_wkb_compose("md");
}

#[test]
fn compose_wkb_mm() {
    test_wkb_compose("mm");
}

#[test]
fn compose_wkb_mn() {
    test_wkb_compose("mn");
}

#[test]
fn compose_wkb_mv() {
    test_wkb_compose("mv");
}

#[test]
fn compose_wkb_ng() {
    test_wkb_compose("ng");
}

#[test]
fn compose_wkb_sn() {
    test_wkb_compose("sn");
}

#[test]
fn compose_wkb_tm() {
    test_wkb_compose("tm");
}

#[test]
fn compose_wkb_tz() {
    test_wkb_compose("tz");
}

// --- fi_FI.UTF-8/Compose ---

#[test]
fn compose_wkb_fi() {
    test_wkb_compose("fi");
}

// --- pt_PT.UTF-8/Compose ---

#[test]
fn compose_wkb_pt() {
    test_wkb_compose("pt");
}

// --- ru_RU.UTF-8/Compose (includes en_US.UTF-8) ---

#[test]
fn compose_wkb_ru() {
    test_wkb_compose("ru");
}

// --- am_ET.UTF-8/Compose ---

#[test]
fn compose_wkb_am() {
    test_wkb_compose("am");
}

// --- th_TH.UTF-8/Compose (includes en_US.UTF-8) ---

#[test]
fn compose_wkb_th() {
    test_wkb_compose("th");
}

// --- cs_CZ.UTF-8/Compose (via xkb_compose_map: cz → cs_CZ.UTF-8) ---

#[test]
fn compose_wkb_cz() {
    test_wkb_compose("cz");
}

// --- el_GR.UTF-8/Compose (via xkb_compose_map: gr → el_GR.UTF-8) ---

#[test]
fn compose_wkb_gr() {
    test_wkb_compose("gr");
}

// --- sr_RS.UTF-8/Compose (via xkb_compose_map: rs → sr_RS.UTF-8) ---

#[test]
fn compose_wkb_rs() {
    test_wkb_compose("rs");
}

// --- sr_RS.UTF-8/Compose (via xkb_compose_map: me → sr_ME.UTF-8) ---

#[test]
fn compose_wkb_me() {
    test_wkb_compose("me");
}

// --- ja_JP.UTF-8/Compose (via xkb_compose_map: jp → ja_JP.UTF-8) ---

#[test]
fn compose_wkb_jp() {
    test_wkb_compose("jp");
}

// --- ko_KR.UTF-8/Compose (via xkb_compose_map: kr → ko_KR.UTF-8) ---

#[test]
fn compose_wkb_kr() {
    test_wkb_compose("kr");
}

// --- km_KH.UTF-8/Compose (via xkb_compose_map: kh → km_KH.UTF-8) ---

#[test]
fn compose_wkb_kh() {
    test_wkb_compose("kh");
}

// --- zh_CN.UTF-8/Compose (via xkb_compose_map: cn → zh_CN.UTF-8) ---

#[test]
fn compose_wkb_cn() {
    test_wkb_compose("cn");
}

// --- zh_TW.UTF-8/Compose (via xkb_compose_map: tw → zh_TW.UTF-8) ---

#[test]
fn compose_wkb_tw() {
    test_wkb_compose("tw");
}

// --- iso8859-4/Compose (ee → locale.alias → ee_EE.ISO8859-4, no UTF-8 variant) ---

#[test]
fn compose_wkb_ee() {
    test_wkb_compose("ee");
}

// --- lk → si_LK.UTF-8 (via xkb_compose_map) ---

#[test]
fn compose_wkb_lk() {
    test_wkb_compose("lk");
}

// ===================================================================
// Part 2: Direct compose file tests
//
// These cover compose files not reachable through any short XKB
// locale name.  They load the compose file directly via
// load_compose_from_path and test all entries.
// ===================================================================

// --- UTF-8 compose files ---

#[test]
fn compose_direct_en_us() {
    test_compose_file_direct(
        "en_US.UTF-8",
        "en_US.UTF-8",
        "/usr/share/X11/locale/en_US.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_el_gr() {
    test_compose_file_direct(
        "el_GR.UTF-8",
        "el_GR.UTF-8",
        "/usr/share/X11/locale/el_GR.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_fi_fi() {
    test_compose_file_direct(
        "fi_FI.UTF-8",
        "fi_FI.UTF-8",
        "/usr/share/X11/locale/fi_FI.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_am_et() {
    test_compose_file_direct(
        "am_ET.UTF-8",
        "am_ET.UTF-8",
        "/usr/share/X11/locale/am_ET.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_sr_rs() {
    test_compose_file_direct(
        "sr_RS.UTF-8",
        "sr_RS.UTF-8",
        "/usr/share/X11/locale/sr_RS.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_pt_br() {
    test_compose_file_direct(
        "pt_BR.UTF-8",
        "pt_BR.UTF-8",
        "/usr/share/X11/locale/pt_BR.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_cs_cz() {
    test_compose_file_direct(
        "cs_CZ.UTF-8",
        "cs_CZ.UTF-8",
        "/usr/share/X11/locale/cs_CZ.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_pt_pt() {
    test_compose_file_direct(
        "pt_PT.UTF-8",
        "pt_PT.UTF-8",
        "/usr/share/X11/locale/pt_PT.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_km_kh() {
    test_compose_file_direct(
        "km_KH.UTF-8",
        "km_KH.UTF-8",
        "/usr/share/X11/locale/km_KH.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_ja_jp() {
    test_compose_file_direct(
        "ja_JP.UTF-8",
        "ja_JP.UTF-8",
        "/usr/share/X11/locale/ja_JP.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_ko_kr() {
    test_compose_file_direct(
        "ko_KR.UTF-8",
        "ko_KR.UTF-8",
        "/usr/share/X11/locale/ko_KR.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_ru_ru() {
    test_compose_file_direct(
        "ru_RU.UTF-8",
        "ru_RU.UTF-8",
        "/usr/share/X11/locale/ru_RU.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_th_th() {
    test_compose_file_direct(
        "th_TH.UTF-8",
        "th_TH.UTF-8",
        "/usr/share/X11/locale/th_TH.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_zh_cn() {
    test_compose_file_direct(
        "zh_CN.UTF-8",
        "zh_CN.UTF-8",
        "/usr/share/X11/locale/zh_CN.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_zh_hk() {
    test_compose_file_direct(
        "zh_HK.UTF-8",
        "zh_HK.UTF-8",
        "/usr/share/X11/locale/zh_HK.UTF-8/Compose",
    );
}

#[test]
fn compose_direct_zh_tw() {
    test_compose_file_direct(
        "zh_TW.UTF-8",
        "zh_TW.UTF-8",
        "/usr/share/X11/locale/zh_TW.UTF-8/Compose",
    );
}

// --- Legacy (non-UTF-8) compose files ---

#[test]
fn compose_direct_iso8859_1() {
    test_compose_file_direct(
        "iso8859-1",
        "af_ZA.ISO8859-1",
        "/usr/share/X11/locale/iso8859-1/Compose",
    );
}

#[test]
fn compose_direct_iso8859_2() {
    test_compose_file_direct(
        "iso8859-2",
        "bs_BA.ISO8859-2",
        "/usr/share/X11/locale/iso8859-2/Compose",
    );
}

#[test]
fn compose_direct_iso8859_3() {
    test_compose_file_direct(
        "iso8859-3",
        "eo_XX.ISO8859-3",
        "/usr/share/X11/locale/iso8859-3/Compose",
    );
}

#[test]
fn compose_direct_iso8859_4() {
    test_compose_file_direct(
        "iso8859-4",
        "ee_EE.ISO8859-4",
        "/usr/share/X11/locale/iso8859-4/Compose",
    );
}

#[test]
fn compose_direct_iso8859_7() {
    test_compose_file_direct(
        "iso8859-7",
        "el_GR.ISO8859-7",
        "/usr/share/X11/locale/iso8859-7/Compose",
    );
}

#[test]
fn compose_direct_iso8859_9() {
    test_compose_file_direct(
        "iso8859-9",
        "tr_TR.ISO8859-9",
        "/usr/share/X11/locale/iso8859-9/Compose",
    );
}

#[test]
fn compose_direct_iso8859_9e() {
    test_compose_file_direct(
        "iso8859-9e",
        "az_AZ.ISO8859-9E",
        "/usr/share/X11/locale/iso8859-9e/Compose",
    );
}

#[test]
fn compose_direct_iso8859_13() {
    test_compose_file_direct(
        "iso8859-13",
        "et_EE.ISO8859-13",
        "/usr/share/X11/locale/iso8859-13/Compose",
    );
}

#[test]
fn compose_direct_iso8859_14() {
    test_compose_file_direct(
        "iso8859-14",
        "br_FR.ISO8859-14",
        "/usr/share/X11/locale/iso8859-14/Compose",
    );
}

#[test]
fn compose_direct_iso8859_15() {
    test_compose_file_direct(
        "iso8859-15",
        "br_FR.ISO8859-15",
        "/usr/share/X11/locale/iso8859-15/Compose",
    );
}

#[test]
fn compose_direct_koi8_c() {
    test_compose_file_direct(
        "koi8-c",
        "az_AZ.KOI8-C",
        "/usr/share/X11/locale/koi8-c/Compose",
    );
}

#[test]
fn compose_direct_vi_vn_tcvn() {
    test_compose_file_direct(
        "vi_VN.tcvn",
        "vi_VN.TCVN",
        "/usr/share/X11/locale/vi_VN.tcvn/Compose",
    );
}

#[test]
fn compose_direct_vi_vn_viscii() {
    test_compose_file_direct(
        "vi_VN.viscii",
        "vi_VN.VISCII",
        "/usr/share/X11/locale/vi_VN.viscii/Compose",
    );
}

// ===================================================================
// Part 3: Locale resolution smoke tests
//
// Verify that resolve_compose_file returns the expected compose file
// for various locale names (short XKB names, full locale names, and
// xkb_compose_map entries).
// ===================================================================

#[test]
fn compose_resolution_short_names() {
    // Short XKB names resolved via locale.alias + UTF-8 fallback
    let cases: &[(&str, &str)] = &[
        ("de", "en_US.UTF-8/Compose"),
        ("fr", "en_US.UTF-8/Compose"),
        ("es", "en_US.UTF-8/Compose"),
        ("it", "en_US.UTF-8/Compose"),
        ("fi", "fi_FI.UTF-8/Compose"),
        ("pt", "pt_PT.UTF-8/Compose"),
        ("ru", "ru_RU.UTF-8/Compose"),
        ("am", "am_ET.UTF-8/Compose"),
        ("th", "th_TH.UTF-8/Compose"),
    ];

    for &(locale, expected) in cases {
        let resolved = wkb::xkb::resolve_compose_file(locale);
        assert_eq!(
            resolved.as_deref(),
            Some(expected),
            "resolve_compose_file('{}') should return '{}'",
            locale,
            expected
        );
    }
}

#[test]
fn compose_resolution_xkb_compose_map() {
    // XKB layout names resolved via the hardcoded xkb_compose_map
    let cases: &[(&str, &str)] = &[
        ("us", "en_US.UTF-8/Compose"),
        ("gb", "en_US.UTF-8/Compose"),
        ("no", "en_US.UTF-8/Compose"),
        ("dk", "en_US.UTF-8/Compose"),
        ("se", "en_US.UTF-8/Compose"),
        ("at", "en_US.UTF-8/Compose"),
        ("ch", "en_US.UTF-8/Compose"),
        ("cz", "cs_CZ.UTF-8/Compose"),
        ("gr", "el_GR.UTF-8/Compose"),
        ("rs", "sr_RS.UTF-8/Compose"),
        ("me", "sr_RS.UTF-8/Compose"),
        ("jp", "ja_JP.UTF-8/Compose"),
        ("kr", "ko_KR.UTF-8/Compose"),
        ("cn", "zh_CN.UTF-8/Compose"),
        ("tw", "zh_TW.UTF-8/Compose"),
        ("kh", "km_KH.UTF-8/Compose"),
        ("ua", "en_US.UTF-8/Compose"),
        ("in", "en_US.UTF-8/Compose"),
        ("il", "en_US.UTF-8/Compose"),
        ("ara", "en_US.UTF-8/Compose"),
        ("ir", "en_US.UTF-8/Compose"),
        ("vn", "en_US.UTF-8/Compose"),
        ("latam", "en_US.UTF-8/Compose"),
        ("epo", "en_US.UTF-8/Compose"),
    ];

    for &(locale, expected) in cases {
        let resolved = wkb::xkb::resolve_compose_file(locale);
        assert_eq!(
            resolved.as_deref(),
            Some(expected),
            "resolve_compose_file('{}') should return '{}'",
            locale,
            expected
        );
    }
}

#[test]
fn compose_resolution_full_locale_names() {
    // Full locale names should match compose.dir directly
    let cases: &[(&str, &str)] = &[
        ("en_US.UTF-8", "en_US.UTF-8/Compose"),
        ("de_DE.UTF-8", "en_US.UTF-8/Compose"),
        ("fi_FI.UTF-8", "fi_FI.UTF-8/Compose"),
        ("pt_PT.UTF-8", "pt_PT.UTF-8/Compose"),
        ("pt_BR.UTF-8", "pt_BR.UTF-8/Compose"),
        ("ru_RU.UTF-8", "ru_RU.UTF-8/Compose"),
        ("el_GR.UTF-8", "el_GR.UTF-8/Compose"),
        ("cs_CZ.UTF-8", "cs_CZ.UTF-8/Compose"),
        ("sr_RS.UTF-8", "sr_RS.UTF-8/Compose"),
        ("am_ET.UTF-8", "am_ET.UTF-8/Compose"),
        ("ja_JP.UTF-8", "ja_JP.UTF-8/Compose"),
        ("ko_KR.UTF-8", "ko_KR.UTF-8/Compose"),
        ("km_KH.UTF-8", "km_KH.UTF-8/Compose"),
        ("th_TH.UTF-8", "th_TH.UTF-8/Compose"),
        ("zh_CN.UTF-8", "zh_CN.UTF-8/Compose"),
        ("zh_HK.UTF-8", "zh_HK.UTF-8/Compose"),
        ("zh_TW.UTF-8", "zh_TW.UTF-8/Compose"),
    ];

    for &(locale, expected) in cases {
        let resolved = wkb::xkb::resolve_compose_file(locale);
        assert_eq!(
            resolved.as_deref(),
            Some(expected),
            "resolve_compose_file('{}') should return '{}'",
            locale,
            expected
        );
    }
}

#[test]
fn compose_resolution_every_xkb_layout() {
    // Every XKB layout file that is a real keyboard layout should
    // resolve to some compose file (no None allowed).
    let xkb_layouts = [
        "af", "al", "am", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "br", "bt", "bw", "by",
        "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg", "epo", "es", "et", "eu",
        "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id", "ie", "il", "in", "iq",
        "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "latam", "lk", "lt", "lv",
        "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "ng", "nl", "no", "np", "nz",
        "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "si", "sk", "sn", "sy", "tg", "th", "tj",
        "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za",
    ];

    for &layout in &xkb_layouts {
        let resolved = wkb::xkb::resolve_compose_file(layout);
        assert!(
            resolved.is_some(),
            "resolve_compose_file('{}') returned None — every XKB layout must resolve",
            layout
        );
    }
}
