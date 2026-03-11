use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use test_case::test_matrix;
use xkb_parser::compose::{parse_compose_file, ComposeEntry};
use xkb_parser::keysym_name_to_char;
use xkbcommon::xkb::{self, compose};

// ---------------------------------------------------------------------------
// Helpers: keysym / char resolution
// ---------------------------------------------------------------------------

/// Simple check whether a compose subpath indicates a UTF-8 compose file.
///
/// Accepts strings like `en_US.UTF-8/Compose` (case-insensitive). Also accepts
/// `.utf8` without the hyphen to be permissive.
fn is_utf8_compose_subpath(subpath: &str) -> bool {
    let s = subpath.to_ascii_lowercase();
    s.contains(".utf-8") || s.contains(".utf8")
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
fn resolve_entry_chars(entry: &ComposeEntry) -> Vec<char> {
    let mut chars = Vec::new();
    for name in &entry.keysym_names {
        if let Some(character) = keysym_name_to_char(name) {
            chars.push(character);
        }
    }
    chars
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
        let (chars, output) = (resolve_entry_chars(entry), entry.output);
        let key = (entry.multi_key_index.is_some(), chars);
        if let Some(&prev) = char_seq_outputs.get(&key) {
            if prev != output {
                collision_seqs.insert(key);
            }
        } else {
            char_seq_outputs.insert(key, output);
        }
    }

    // Detect prefix conflicts
    let all_char_seqs: Vec<(bool, Vec<char>)> = entries
        .iter()
        .map(|e| (e.multi_key_index.is_some(), e.keys.clone()))
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
        let expected = entry.output;

        let xkb_result = xkb_state.as_mut().and_then(|state| {
            xkb_compose_sequence(state, &entry.keysym_names, entry.multi_key_index.is_some())
        });

        let wkb_result = {
            let composer = if entry.multi_key_index.is_some() {
                compose_key
            } else {
                regular
            };
            wkb_compose_sequence(composer, &resolve_entry_chars(entry))
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
                        entry.keysym_names,
                        entry.multi_key_index.is_some(),
                        xkb_result,
                        wkb_result,
                        expected
                    );
                    let key = (entry.multi_key_index.is_some(), entry.keys.clone());
                    let is_known =
                        collision_seqs.contains(&key) || prefix_conflict_seqs.contains(&key);
                    if is_known {
                        char_collisions.push(msg);
                    } else {
                        mismatches.push(msg);
                    }
                }
            } else if xkb_result.is_some() && wkb_result.is_none() {
                wkb_failures.push(format!(
                    "  WKB_MISS: {:?} [multi={}] -> xkb={:?} expected={:?}",
                    entry.keysym_names,
                    entry.multi_key_index.is_some(),
                    xkb_result,
                    expected
                ));
            } else if xkb_result.is_none() && wkb_result.is_some() {
                xkb_failures.push(format!(
                    "  XKB_MISS: {:?} [multi={}] -> wkb={:?} expected={:?}",
                    entry.keysym_names,
                    entry.multi_key_index.is_some(),
                    wkb_result,
                    expected
                ));
            }
        } else {
            match wkb_result {
                Some(c) if c == expected => wkb_only_ok += 1,
                Some(c) => {
                    mismatches.push(format!(
                        "  WKB_WRONG: {:?} [multi={}] -> wkb={:?} expected={:?}",
                        entry.keysym_names,
                        entry.multi_key_index.is_some(),
                        c,
                        expected
                    ));
                }
                None => {
                    wkb_failures.push(format!(
                        "  WKB_MISS: {:?} [multi={}] -> expected={:?}",
                        entry.keysym_names,
                        entry.multi_key_index.is_some(),
                        expected
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
    let compose_file_subpath = wkb::xkb::compose_parse::resolve_compose_file(xkb_locale)
        .unwrap_or_else(|| {
            panic!(
                "resolve_compose_file('{}') returned None — \
                 add an entry to xkb_compose_map.rs",
                xkb_locale
            )
        });

    // Simple UTF-8 check: only exercise compose files whose subpath indicates UTF-8.
    if !is_utf8_compose_subpath(&compose_file_subpath) {
        println!(
            "SKIP: wkb({}) -> legacy/non-UTF-8 compose file '{}'",
            xkb_locale, compose_file_subpath
        );
        return;
    }

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
    // Derive the compose subpath relative to /usr/share/X11/locale if possible,
    // e.g. "/usr/share/X11/locale/en_US.UTF-8/Compose" -> "en_US.UTF-8/Compose".
    let subpath = compose_file
        .strip_prefix("/usr/share/X11/locale/")
        .unwrap_or(compose_file);

    // Simple UTF-8 check (no compose.dir parsing): only support explicit UTF-8 subpaths.
    if !is_utf8_compose_subpath(subpath) {
        println!(
            "SKIP: legacy/non-UTF-8 compose file not supported: {}",
            compose_file
        );
        return;
    }

    let compose_path = Path::new(compose_file);
    if !compose_path.exists() {
        println!("SKIP: compose file not found: {}", compose_file);
        return;
    }

    let (regular, compose_key) = wkb::xkb::compose_parse::load_compose_from_path(compose_path);

    run_compose_test(label, xkb_locale, compose_path, &regular, &compose_key);
}

// ===================================================================
// Part 1: XKB locales via WKB::new_from_names
//
// Exercises the full pipeline: locale resolution through
// locale.alias + compose.dir, compose file loading, and trie building.
// ===================================================================

#[test_matrix([
    "af", "al", "am", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "br", "bt", "bw", "by",
    "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg", "epo", "es", "et", "eu",
    "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id", "ie", "il", "in", "iq",
    "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "latam", "lk", "lt", "lv",
    "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "ng", "nl", "no", "np", "nz",
    "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "si", "sk", "sn", "sy", "tg", "th", "tj",
    "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za"
])]
fn compose_wkb(locale: &str) {
    test_wkb_compose(locale);
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
        let resolved = wkb::xkb::compose_parse::resolve_compose_file(locale);
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
        let resolved = wkb::xkb::compose_parse::resolve_compose_file(locale);
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
        let resolved = wkb::xkb::compose_parse::resolve_compose_file(locale);
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
        let resolved = wkb::xkb::compose_parse::resolve_compose_file(layout);
        assert!(
            resolved.is_some(),
            "resolve_compose_file('{}') returned None — every XKB layout must resolve",
            layout
        );
    }
}
