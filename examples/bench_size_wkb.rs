//! Minimal binary using only the wkb backend.
//! Build with: cargo build --example bench_size_wkb --release
//! Measure with: size target/release/examples/bench_size_wkb
//!           or: cargo bloat --example bench_size_wkb --release -n 20

#[path = "../benches/common.rs"]
mod common;
use common::*;
use std::hint::black_box;
use wkb::testing::composer_feed;

fn main() {
    let mut checksum: u64 = 0;

    for &(locale, variant) in LAYOUTS {
        let mut wb = wkb::WKB::new_from_names("", "", locale, variant.unwrap_or(""), None).unwrap();

        for case in KEY_CASES {
            for &(code, down) in case.keys {
                if down {
                    let result = wb.press_key(code);
                    if let Some(ch) = wb.key_char(code) {
                        checksum = checksum.wrapping_add(ch as u64);
                    }
                    black_box(result);
                } else {
                    let result = wb.release_key(code);
                    black_box(result);
                }
            }
        }
    }

    if let Some(subpath) = xkb_core::compose::resolve_compose_file(COMPOSE_LOCALE) {
        let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
        let mut composer = wkb::testing::compose_parse::load_compose_from_path(&path);
        for seq in COMPOSE_SEQUENCES {
            for &ks in seq.keysyms {
                if let Some(ch) = xkb_core::keysym_utf::keysym_to_char(ks) {
                    let _ = composer_feed(&mut composer, wkb::testing::Token::Char(ch));
                    checksum = checksum.wrapping_add(1);
                }
            }
        }
    }

    black_box(checksum);
}
