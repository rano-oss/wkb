//! Minimal binary using only the wkb backend (standalone, no XKB).
//! Build with: cargo build --example bench_size_wkb --release --no-default-features
//! Measure with: size target/release/examples/bench_size_wkb
//!           or: cargo bloat --example bench_size_wkb --release -n 20

use std::collections::BTreeMap;
use std::hint::black_box;

#[path = "../benches/common.rs"]
mod common;
use common::*;
use wkb::ir::{LayoutFile, ModAction};
use wkb::{ModType, WKB};

fn levels(entries: &[(u8, u32, char)]) -> BTreeMap<u8, BTreeMap<u32, char>> {
    let mut out: BTreeMap<u8, BTreeMap<u32, char>> = BTreeMap::new();
    for &(level, keycode, ch) in entries {
        out.entry(level).or_default().insert(keycode, ch);
    }
    out
}

fn synthetic_layout() -> LayoutFile {
    let mut keymap = BTreeMap::new();
    keymap.insert(
        "layout".to_string(),
        levels(&[
            (0, 2, '1'),
            (0, 15, '\t'),
            (0, 18, 'e'),
            (0, 30, 'a'),
            (0, 39, ';'),
            (0, 44, 'z'),
            (0, 48, 'b'),
            (0, 57, ' '),
            (0, 59, '\u{f000}'),
            (0, 76, '5'),
            (1, 2, '!'),
            (1, 30, 'A'),
            (1, 39, ':'),
            (3, 18, '\u{20ac}'),
        ]),
    );
    let mut num_lock_keys = BTreeMap::new();
    num_lock_keys.insert("layout".to_string(), levels(&[(0, 76, '5')]));
    let mut caps_lock_keymap = BTreeMap::new();
    caps_lock_keymap.insert("layout".to_string(), levels(&[(0, 30, 'A')]));

    LayoutFile {
        version: wkb::ir::FORMAT_VERSION,
        layout_names: vec!["layout".to_string()],
        num_keys: 128,
        repeat_keys: vec![30, 48],
        modifiers: vec![
            (
                42,
                "LeftShift".to_string(),
                vec![(0, ModAction::Pressed(ModType::Level2))],
            ),
            (
                54,
                "RightShift".to_string(),
                vec![(0, ModAction::Pressed(ModType::Level2))],
            ),
            (
                58,
                "CapsLock".to_string(),
                vec![(0, ModAction::Lock(ModType::Caps))],
            ),
            (
                100,
                "AltGr".to_string(),
                vec![(0, ModAction::Pressed(ModType::Level3))],
            ),
            (
                69,
                "NumLock".to_string(),
                vec![(0, ModAction::Lock(ModType::Num))],
            ),
            (
                29,
                "LeftControl".to_string(),
                vec![(0, ModAction::Pressed(ModType::None))],
            ),
            (
                56,
                "Alt".to_string(),
                vec![(0, ModAction::Pressed(ModType::None))],
            ),
        ],
        keymap,
        num_lock_keys,
        caps_lock_keymap,
        keysym_map: BTreeMap::new(),
        compose: Vec::new(),
    }
}

fn main() {
    let file = synthetic_layout();
    let kdl = file.to_kdl_string().unwrap();
    let loaded = wkb::ir::LayoutFile::from_kdl_str(&kdl).unwrap();
    let mut wkb = WKB::new_from_layouts(vec![loaded]).unwrap();

    let mut checksum: u64 = 0;
    for case in KEY_CASES {
        for &(code, down) in case.keys {
            let result = if down {
                wkb.press_key(code)
            } else {
                wkb.release_key(code)
            };
            black_box(result);
            if down {
                if let Some(ch) = wkb.key_char(code) {
                    checksum = checksum.wrapping_add(ch as u64);
                }
            }
        }
    }
    black_box(checksum);
}
