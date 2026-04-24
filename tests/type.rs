//! kbvm type-test runner: parse map.xkb, replay key events, compare WKB vs xkbcommon.
//!
//! Test data from https://github.com/mahkoh/kbvm (MIT licensed).
//! Only tests that xkbcommon can handle are included.
//!
//! For each test we:
//! 1. Parse the keymap with xkbcommon and replay events
//! 2. Parse the same keymap with WKB and replay events
//! 3. Compare char output and modifier state between the two

use std::collections::HashMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use wkb::testing::WKBTestExt;
use xkbcommon::xkb;

const TEST_DATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_files");
const EVDEV_OFFSET: u32 = 8;

fn parse_generated_keycodes() -> HashMap<String, u32> {
    let path = Path::new(TEST_DATA_DIR).join("type-include/keycodes/generated");
    let content = std::fs::read_to_string(&path).expect("read generated keycodes");
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('<') {
            if let Some((name, rest)) = rest.split_once('>') {
                let rest = rest.trim().strip_prefix('=').unwrap_or("").trim();
                let rest = rest.strip_suffix(';').unwrap_or(rest).trim();
                if let Ok(code) = rest.parse::<u32>() {
                    map.insert(name.to_string(), code);
                }
            }
        }
    }
    map
}

fn collect_type_test_cases() -> Vec<PathBuf> {
    let base = Path::new(TEST_DATA_DIR).join("type-tests");
    let mut cases = Vec::new();
    collect_dirs(&base, "map.xkb", &mut cases);
    cases.sort();
    cases
}

fn collect_dirs(dir: &Path, filename: &str, out: &mut Vec<PathBuf>) {
    if dir.join(filename).exists() {
        out.push(dir.to_path_buf());
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_dirs(&path, filename, out);
            }
        }
    }
}

fn build_full_keymap(case_dir: &Path) -> String {
    let map_content = std::fs::read_to_string(case_dir.join("map.xkb")).expect("read map.xkb");
    let keycodes_path = Path::new(TEST_DATA_DIR).join("type-include/keycodes/generated");
    let keycodes = std::fs::read_to_string(&keycodes_path).expect("read keycodes");
    let inner = keycodes
        .trim()
        .strip_prefix("default xkb_keycodes {")
        .and_then(|s| s.strip_suffix("};"))
        .unwrap_or(&keycodes);
    map_content.replace(r#"include "generated""#, inner.trim())
}

fn keysym_char(ks: xkb::Keysym) -> Option<char> {
    let ch = xkb::keysym_to_utf32(ks);
    if ch == 0 {
        None
    } else {
        char::from_u32(ch)
    }
}

fn case_name(case_dir: &Path) -> String {
    case_dir
        .strip_prefix(Path::new(TEST_DATA_DIR).join("type-tests"))
        .unwrap_or(case_dir)
        .display()
        .to_string()
}

/// Replay key events through xkbcommon and WKB, comparing output at each step.
fn run_type_test(case_dir: &Path, key_names: &HashMap<String, u32>) -> Result<(), String> {
    let keymap_str = build_full_keymap(case_dir);
    let name = case_name(case_dir);

    // Parse with xkbcommon
    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let xkb_keymap = xkb::Keymap::new_from_string(
        &ctx,
        keymap_str,
        xkb::KEYMAP_FORMAT_TEXT_V1,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .ok_or_else(|| format!("{name}: xkbcommon failed to parse"))?;
    let mut xkb_state = xkb::State::new(&xkb_keymap);

    // Parse with WKB using xkbcommon's fully-expanded keymap string
    // (the original minimal keymap may lack types/compat sections that WKB needs)
    let full_keymap_str = xkb_keymap.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    let mut wkb = wkb::WKB::new_from_string(full_keymap_str);

    let input = std::fs::read_to_string(case_dir.join("input.txt"))
        .map_err(|e| format!("{name}: read input.txt: {e}"))?;

    let mut diffs = Vec::new();

    for line in input.lines() {
        let stripped = line
            .split_once('#')
            .map(|(pre, _)| pre)
            .unwrap_or(line)
            .trim();
        if stripped.is_empty() {
            continue;
        }

        let (command, arg) = match stripped.split_once(' ') {
            Some((c, a)) => (c.trim(), a.trim()),
            None => (stripped, ""),
        };

        let get_kc = |key_name: &str| -> Result<u32, String> {
            key_names
                .get(key_name)
                .copied()
                .ok_or_else(|| format!("{name}: unknown key: {key_name}"))
        };

        match command {
            "down" | "up" | "both" => {
                let xkb_kc_raw = get_kc(arg)?;
                let xkb_kc = xkb::Keycode::new(xkb_kc_raw);

                if xkb_kc_raw < EVDEV_OFFSET {
                    continue;
                }
                let evdev_code = xkb_kc_raw - EVDEV_OFFSET;

                let do_down = command == "down" || command == "both";
                let do_up = command == "up" || command == "both";

                if do_down {
                    // Get char from xkbcommon (before update_key)
                    let xkb_syms = xkb_state.key_get_syms(xkb_kc);
                    let xkb_char = xkb_syms.first().and_then(|&s| keysym_char(s));

                    // Get char from WKB
                    let wkb_char = wkb.utf8(evdev_code);

                    if xkb_char != wkb_char {
                        diffs.push(format!(
                            "  {command} {arg}: char mismatch: \
                             xkbcommon={xkb_char:?} wkb={wkb_char:?}"
                        ));
                    }

                    // Update both states
                    xkb_state.update_key(xkb_kc, xkb::KeyDirection::Down);
                    wkb.update_key(evdev_code, wkb::KeyDirection::Down);

                    // Compare modifier state
                    let xkb_mods = (
                        xkb_state.serialize_mods(xkb::STATE_MODS_DEPRESSED),
                        xkb_state.serialize_mods(xkb::STATE_MODS_LATCHED),
                        xkb_state.serialize_mods(xkb::STATE_MODS_LOCKED),
                    );
                    let (wkb_dep, wkb_lat, wkb_lock, _) = wkb.modifiers_state();

                    if xkb_mods != (wkb_dep, wkb_lat, wkb_lock) {
                        diffs.push(format!(
                            "  {command} {arg} (after down): mods mismatch: \
                             xkbcommon=({:#010x},{:#010x},{:#010x}) \
                             wkb=({:#010x},{:#010x},{:#010x})",
                            xkb_mods.0, xkb_mods.1, xkb_mods.2, wkb_dep, wkb_lat, wkb_lock
                        ));
                    }
                }

                if do_up {
                    xkb_state.update_key(xkb_kc, xkb::KeyDirection::Up);
                    wkb.update_key(evdev_code, wkb::KeyDirection::Up);

                    let xkb_mods = (
                        xkb_state.serialize_mods(xkb::STATE_MODS_DEPRESSED),
                        xkb_state.serialize_mods(xkb::STATE_MODS_LATCHED),
                        xkb_state.serialize_mods(xkb::STATE_MODS_LOCKED),
                    );
                    let (wkb_dep, wkb_lat, wkb_lock, _) = wkb.modifiers_state();

                    if xkb_mods != (wkb_dep, wkb_lat, wkb_lock) {
                        diffs.push(format!(
                            "  {command} {arg} (after up): mods mismatch: \
                             xkbcommon=({:#010x},{:#010x},{:#010x}) \
                             wkb=({:#010x},{:#010x},{:#010x})",
                            xkb_mods.0, xkb_mods.1, xkb_mods.2, wkb_dep, wkb_lat, wkb_lock
                        ));
                    }
                }
            }
            "repeat" => {}
            _ => return Err(format!("{name}: unknown command: {command}")),
        }
    }

    if diffs.is_empty() {
        Ok(())
    } else {
        let mut msg = format!("{name}: WKB vs xkbcommon differences:\n");
        for d in &diffs {
            writeln!(msg, "{d}").unwrap();
        }
        Err(msg)
    }
}

#[test]
fn kbvm_type_tests() {
    let key_names = parse_generated_keycodes();
    let cases = collect_type_test_cases();
    assert!(!cases.is_empty(), "No type test cases found");

    let mut failures = Vec::new();

    for case in &cases {
        if let Err(e) = run_type_test(case, &key_names) {
            failures.push(e);
        }
    }

    if !failures.is_empty() {
        let mut msg = format!(
            "{} of {} kbvm type tests failed (WKB vs xkbcommon mismatch):\n\n",
            failures.len(),
            cases.len()
        );
        for f in &failures {
            writeln!(msg, "{f}").unwrap();
        }
        panic!("{msg}");
    }

    eprintln!(
        "All {} kbvm type tests passed (WKB matches xkbcommon)",
        cases.len()
    );
}
