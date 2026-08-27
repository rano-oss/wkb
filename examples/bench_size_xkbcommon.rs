//! Minimal binary using only the xkbcommon (linked) backend.
//! Build with: cargo build --example bench_size_xkbcommon --release
//! Measure with: size target/release/examples/bench_size_xkbcommon

#[path = "../benches/common.rs"]
mod common;
use common::*;
use std::hint::black_box;

fn main() {
    use xkbcommon::xkb;
    let mut checksum: u64 = 0;

    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    for &(locale, variant) in LAYOUTS {
        let km = xkb::Keymap::new_from_names(
            &ctx,
            "evdev",
            "",
            locale,
            variant.unwrap_or(""),
            None,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )
        .expect("keymap");
        let mut st = xkb::State::new(&km);

        for case in KEY_CASES {
            for &(code, down) in case.keys {
                let xkb_kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                let dir = if down {
                    xkb::KeyDirection::Down
                } else {
                    xkb::KeyDirection::Up
                };
                st.update_key(xkb_kc, dir);
                if down {
                    let s = st.key_get_utf8(xkb_kc);
                    checksum = checksum.wrapping_add(s.len() as u64);
                }
            }
        }
    }

    // Compose workload through the keymap state + compose state.
    let locale_os = std::ffi::OsStr::new(COMPOSE_LOCALE);
    if let Ok(table) =
        xkb::compose::Table::new_from_locale(&ctx, locale_os, xkb::compose::COMPILE_NO_FLAGS)
    {
        let km = xkb::Keymap::new_from_names(
            &ctx,
            "evdev",
            "",
            "us",
            "",
            None,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )
        .expect("keymap");
        let mut comp_st = xkb::State::new(&km);
        let mut client_st = xkb::State::new(&km);
        let mut compose = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);
        let compose_kc = xkb::Keycode::new(COMPOSE_KEY + EVDEV_OFFSET);
        for case in COMPOSE_CASES {
            let c = xkb_feed_compose(
                &mut comp_st,
                &mut client_st,
                &mut compose,
                case.keys,
                compose_kc,
            );
            checksum = checksum.wrapping_add(c.map_or(0, |c| c as u64));
        }
    }

    black_box(checksum);
}
