//! Minimal binary using only the wkb backend.
//! Build with: cargo build --example bench_size_wkb --release
//! Measure with: size target/release/examples/bench_size_wkb
//!           or: cargo bloat --example bench_size_wkb --release -n 20

#[path = "../benches/common.rs"]
mod common;
use common::*;
use std::hint::black_box;

fn main() {
    let mut checksum: u64 = 0;

    for &(locale, variant) in LAYOUTS {
        let layout = variant.map(String::from);
        let mut wb = wkb::xkb::new_from_names(locale.to_string(), layout);

        for case in KEY_CASES {
            for &(code, down) in case.keys {
                let dir = if down {
                    wkb::modifiers::KeyDirection::Down
                } else {
                    wkb::modifiers::KeyDirection::Up
                };
                wb.update_key(code, dir);
                if down {
                    if let Some(ch) = wb.utf8(code) {
                        checksum = checksum.wrapping_add(ch as u64);
                    }
                }
            }
        }
    }

    if let Some(subpath) = xkb_core::compose::resolve_compose_file(COMPOSE_LOCALE) {
        let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
        let mut composer = wkb::xkb::load_compose_from_path(&path);
        use wkb::composer::Composer;
        for seq in COMPOSE_SEQUENCES {
            for &ks in seq.keysyms {
                if let Some(ch) = xkb_core::keysym_utf::keysym_to_char(ks) {
                    let _ = composer.feed(wkb::composer::Token::Char(ch));
                    checksum = checksum.wrapping_add(1);
                }
            }
        }
    }

    black_box(checksum);
}
