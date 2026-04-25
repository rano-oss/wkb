//! kbvm compile-test runner: parse input.xkb with xkbcommon, verify compilation,
//! test round-trip serialization via WKB's `as_xkb_string()`.
//!
//! Test data from https://github.com/mahkoh/kbvm (MIT licensed).
//!
//! For each test: parse with xkbcommon (using include paths), build WKB from
//! xkbcommon's output, serialize with WKB, and verify the result re-parses.

use std::panic;
use std::path::{Path, PathBuf};

const TEST_DATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_files");

fn collect_xkb_compile_tests() -> Vec<PathBuf> {
    let base = Path::new(TEST_DATA_DIR).join("compile-tests");
    let mut cases = Vec::new();
    collect_dirs_with_file(&base, "input.xkb", &mut cases);
    cases.sort();
    cases
}

fn collect_dirs_with_file(dir: &Path, filename: &str, out: &mut Vec<PathBuf>) {
    if dir.join(filename).exists() {
        out.push(dir.to_path_buf());
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_dirs_with_file(&path, filename, out);
            }
        }
    }
}

fn case_name(case_dir: &Path) -> String {
    case_dir
        .strip_prefix(Path::new(TEST_DATA_DIR).join("compile-tests"))
        .unwrap_or(case_dir)
        .display()
        .to_string()
}

fn run_xkb_compile_test(case_dir: &Path) -> Result<(), String> {
    let name = case_name(case_dir);
    let input = std::fs::read_to_string(case_dir.join("input.xkb"))
        .map_err(|e| format!("{name}: read input.xkb: {e}"))?;

    // Set up xkbcommon context with include paths
    let mut ctx = xkbcommon::xkb::Context::new(xkbcommon::xkb::CONTEXT_NO_DEFAULT_INCLUDES);
    ctx.include_path_append(case_dir);
    ctx.include_path_append(&Path::new(TEST_DATA_DIR).join("compile-include"));
    let extra_includes = case_dir.join("extra-includes");
    if extra_includes.is_dir() {
        ctx.include_path_append(&extra_includes);
    }

    // Parse with xkbcommon
    let keymap = xkbcommon::xkb::Keymap::new_from_string(
        &ctx,
        input,
        xkbcommon::xkb::KEYMAP_FORMAT_TEXT_V1,
        xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .ok_or_else(|| format!("{name}: xkbcommon failed to parse"))?;

    // Build WKB from xkbcommon's fully-expanded keymap
    let xkb_str = keymap.get_as_string(xkbcommon::xkb::KEYMAP_FORMAT_TEXT_V1);
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let wkb_result = panic::catch_unwind(|| wkb::WKB::new_from_string(&xkb_str));
    panic::set_hook(prev_hook);
    let wkb = wkb_result
        .map_err(|_| format!("{name}: WKB panicked"))?
        .map_err(|e| format!("{name}: WKB error: {e}"))?;

    // Round-trip: serialize with WKB and verify xkbcommon can re-parse
    let serialized = wkb
        .as_xkb_string()
        .ok_or_else(|| format!("{name}: WKB serializer returned None"))?;

    let ctx2 = xkbcommon::xkb::Context::new(xkbcommon::xkb::CONTEXT_NO_FLAGS);
    xkbcommon::xkb::Keymap::new_from_string(
        &ctx2,
        serialized.to_string(),
        xkbcommon::xkb::KEYMAP_FORMAT_TEXT_V1,
        xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .ok_or_else(|| format!("{name}: round-trip re-parse failed"))?;

    Ok(())
}

#[test]
fn kbvm_compile_xkb_tests() {
    let cases = collect_xkb_compile_tests();
    assert!(!cases.is_empty(), "No XKB compile test cases found");

    let mut failures = Vec::new();

    for case in &cases {
        if let Err(e) = run_xkb_compile_test(case) {
            failures.push(e);
        }
    }

    if !failures.is_empty() {
        let mut msg = format!(
            "{} of {} kbvm compile tests failed:\n\n",
            failures.len(),
            cases.len()
        );
        for f in &failures {
            msg.push_str(f);
            msg.push('\n');
        }
        panic!("{msg}");
    }

    eprintln!(
        "All {} kbvm compile tests passed (round-trip verified)",
        cases.len()
    );
}
