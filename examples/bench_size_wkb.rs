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
    let keymap = levels(&[
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
    ]);
    let num_lock_keys = levels(&[(0, 76, '5')]);
    let caps_lock_keymap = levels(&[(0, 30, 'A')]);
    let caps_num_lock_keys = levels(&[(0, 30, 'A')]);

    LayoutFile {
        version: wkb::ir::FORMAT_VERSION,
        layout: "layout".to_string(),
        repeat_keys: vec![30, 48],
        modifiers: vec![
            (42, vec![(0, ModAction::Press(ModType::Level2))]),
            (54, vec![(0, ModAction::Press(ModType::Level2))]),
            (58, vec![(0, ModAction::Lock(ModType::Caps))]),
            (100, vec![(0, ModAction::Press(ModType::Level3))]),
            (69, vec![(0, ModAction::Lock(ModType::Num))]),
            (29, vec![(0, ModAction::Press(ModType::None))]),
            (56, vec![(0, ModAction::Press(ModType::None))]),
        ],
        keymap,
        num_lock_keys,
        caps_lock_keymap,
        keysym_map: BTreeMap::new(),
        compose: Vec::new(),
        caps_num_lock_keys
    }
}

fn main() {
    let file = synthetic_layout();
    let ron = file.to_ron_string().unwrap();
    let loaded = wkb::ir::LayoutFile::from_ron_str(&ron).unwrap();
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
