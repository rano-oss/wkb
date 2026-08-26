//! Minimal binary using only the wkb backend (with XKB compilation).
//! Build with: cargo build --example bench_size_wkb_xkb --release
//! Measure with: size target/release/examples/bench_size_wkb_xkb
//!           or: cargo bloat --example bench_size_wkb_xkb --release -n 20

#[path = "../benches/common.rs"]
mod common;
use common::*;
use std::hint::black_box;
use wkb::WKB;

fn main() {
    let mut checksum: u64 = 0;

    for &(locale, variant) in LAYOUTS {
        let wb = WKB::new_from_names("", "", locale, variant.unwrap_or(""), None).unwrap();

        for case in KEY_CASES {
            for &(code, down) in case.keys {
                if down {
                    if let Some(ch) = wb.key_char(code) {
                        checksum = checksum.wrapping_add(ch as u64);
                    }
                }
            }
        }
    }

    unsafe { std::env::set_var("LC_ALL", COMPOSE_LOCALE) };
    let mut wb = WKB::new_from_names("", "", "us", "", None).unwrap();
    wb.set_compose_key(COMPOSE_KEY);
    for case in COMPOSE_CASES {
        for &(code, down) in case.keys {
            if down {
                let result = wb.compose(code);
                if let Some(wkb::ComposeState::Finished(c)) = &result {
                    checksum = checksum.wrapping_add(*c as u64);
                }
                black_box(result);
            }
        }
    }

    black_box(checksum);
}
