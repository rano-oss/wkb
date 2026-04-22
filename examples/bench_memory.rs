//! Memory benchmark — run under valgrind massif or just measure RSS.
//!
//! Usage:
//!   cargo build --example bench_memory --release
//!   valgrind --tool=massif --pages-as-heap=yes ./target/release/examples/bench_memory
//!   ms_print massif.out.<pid>
//!
//! Or for quick RSS measurement:
//!   /usr/bin/time -v ./target/release/examples/bench_memory 2>&1 | grep "Maximum resident"

#[path = "../benches/common.rs"]
mod common;

use common::*;
use std::ffi::CString;
use std::hint::black_box;
use std::os::raw::c_char;
use std::ptr;

fn get_rss_kb() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
}

fn print_rss(label: &str) {
    if let Some(rss) = get_rss_kb() {
        println!("{label:40} RSS: {rss:>8} kB");
    }
}

fn run_workload_wkb() -> u64 {
    let mut checksum: u64 = 0;

    print_rss("wkb/before_setup");

    for &(locale, variant) in LAYOUTS {
        let layout = variant.map(String::from);
        let mut wb = wkb::xkb::new_from_names(locale.to_string(), layout);

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
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
    }

    // Compose workload
    if let Some(subpath) = xkb_core::compose::resolve_compose_file(COMPOSE_LOCALE) {
        let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
        let mut composer = wkb::xkb::load_compose_from_path(&path);
        use wkb::composer::Composer;
        for seq in COMPOSE_SEQUENCES {
            for _ in 0..HOT_PATH_ITERATIONS {
                for &ks in seq.keysyms {
                    if let Some(ch) = xkb_core::keysym_utf::keysym_to_char(ks) {
                        let st = composer.feed(wkb::composer::Token::Char(ch));
                        checksum = checksum.wrapping_add(format!("{st:?}").len() as u64);
                    }
                }
            }
        }
    }

    print_rss("wkb/after_workload");
    checksum
}

fn run_workload_xkbcommon() -> u64 {
    use xkbcommon::xkb;
    let mut checksum: u64 = 0;

    print_rss("xkbcommon/before_setup");

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
            for _ in 0..HOT_PATH_ITERATIONS {
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
    }

    // Compose workload
    let locale_os = std::ffi::OsStr::new(COMPOSE_LOCALE);
    if let Ok(table) =
        xkb::compose::Table::new_from_locale(&ctx, locale_os, xkb::compose::COMPILE_NO_FLAGS)
    {
        let mut state = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);
        for seq in COMPOSE_SEQUENCES {
            for _ in 0..HOT_PATH_ITERATIONS {
                for &ks in seq.keysyms {
                    let _ = state.feed(xkb::Keysym::new(ks));
                    checksum = checksum.wrapping_add(state.status() as u64);
                }
                state.reset();
            }
        }
    }

    print_rss("xkbcommon/after_workload");
    checksum
}

fn run_workload_xkbcommon_dl() -> u64 {
    let mut checksum: u64 = 0;

    print_rss("xkbcommon-dl/before_setup");

    let xkb = xkbcommon_dl::xkbcommon_handle();
    let ctx =
        unsafe { (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };

    for &(locale, variant) in LAYOUTS {
        let c_rules = CString::new("evdev").unwrap();
        let c_layout = CString::new(locale).unwrap();
        let c_variant = variant.map(|v| CString::new(v).unwrap());

        let names = xkbcommon_dl::xkb_rule_names {
            rules: c_rules.as_ptr(),
            model: ptr::null(),
            layout: c_layout.as_ptr(),
            variant: c_variant.as_ref().map_or(ptr::null(), |v| v.as_ptr()),
            options: ptr::null(),
        };

        let km = unsafe {
            (xkb.xkb_keymap_new_from_names)(
                ctx,
                &names,
                xkbcommon_dl::xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
            )
        };
        let st = unsafe { (xkb.xkb_state_new)(km) };
        let mut buf = [0u8; 64];

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                for &(code, down) in case.keys {
                    let xkb_kc = code + EVDEV_OFFSET;
                    let dir = if down {
                        xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                    } else {
                        xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                    };
                    unsafe { (xkb.xkb_state_update_key)(st, xkb_kc, dir) };
                    if down {
                        let n = unsafe {
                            (xkb.xkb_state_key_get_utf8)(
                                st,
                                xkb_kc,
                                buf.as_mut_ptr() as *mut c_char,
                                buf.len(),
                            )
                        };
                        checksum = checksum.wrapping_add(n as u64);
                    }
                }
            }
        }

        unsafe {
            (xkb.xkb_state_unref)(st);
            (xkb.xkb_keymap_unref)(km);
        }
    }

    // Compose workload
    let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
    let c_locale = CString::new(COMPOSE_LOCALE).unwrap();
    let table = unsafe {
        (xkb_compose.xkb_compose_table_new_from_locale)(
            ctx,
            c_locale.as_ptr(),
            xkbcommon_dl::xkb_compose_compile_flags::XKB_COMPOSE_COMPILE_NO_FLAGS,
        )
    };
    if !table.is_null() {
        let state = unsafe {
            (xkb_compose.xkb_compose_state_new)(
                table,
                xkbcommon_dl::xkb_compose_state_flags::XKB_COMPOSE_STATE_NO_FLAGS,
            )
        };
        for seq in COMPOSE_SEQUENCES {
            for _ in 0..HOT_PATH_ITERATIONS {
                for &ks in seq.keysyms {
                    let _ = unsafe { (xkb_compose.xkb_compose_state_feed)(state, ks) };
                    let status = unsafe { (xkb_compose.xkb_compose_state_get_status)(state) };
                    checksum = checksum.wrapping_add(status as u64);
                }
                unsafe { (xkb_compose.xkb_compose_state_reset)(state) };
            }
        }
        unsafe {
            (xkb_compose.xkb_compose_state_unref)(state);
            (xkb_compose.xkb_compose_table_unref)(table);
        }
    }

    unsafe { (xkb.xkb_context_unref)(ctx) };

    print_rss("xkbcommon-dl/after_workload");
    checksum
}

fn run_workload_xkbcommon_compat() -> u64 {
    use xkb_core::rust_types::{Context, RuleNames};
    let mut checksum: u64 = 0;

    print_rss("xkbcommon-compat/before_setup");

    let ctx = Context::new().expect("xkb-core context");

    for &(locale, variant) in LAYOUTS {
        let rmlvo = RuleNames {
            rules: "evdev".to_string(),
            model: String::new(),
            layout: locale.to_string(),
            variant: variant.unwrap_or("").to_string(),
            options: String::new(),
        };
        let km = ctx.keymap_from_names(&rmlvo).expect("keymap");
        let mut st = km.new_state().expect("state");

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                for &(code, down) in case.keys {
                    let xkb_kc = code + EVDEV_OFFSET;
                    let dir = if down {
                        xkb_core::XKB_KEY_DOWN
                    } else {
                        xkb_core::XKB_KEY_UP
                    };
                    st.update_key(xkb_kc, dir);
                    if down {
                        let s = st.key_get_utf8(xkb_kc);
                        checksum = checksum.wrapping_add(s.len() as u64);
                    }
                }
            }
        }
    }

    // Compose workload
    if let Some(table) = xkb_core::compose::ComposeTable::new_from_locale(COMPOSE_LOCALE) {
        let mut state = table.new_state();
        for seq in COMPOSE_SEQUENCES {
            for _ in 0..HOT_PATH_ITERATIONS {
                for &ks in seq.keysyms {
                    let _ = state.feed(ks);
                    checksum = checksum.wrapping_add(state.status() as u64);
                }
                state.reset();
            }
        }
    }

    print_rss("xkbcommon-compat/after_workload");
    checksum
}

fn main() {
    println!("=== Memory Benchmark ===\n");

    print_rss("baseline");
    println!();

    let c1 = run_workload_wkb();
    println!("  wkb checksum: {c1}\n");

    let c2 = run_workload_xkbcommon();
    println!("  xkbcommon checksum: {c2}\n");

    let c3 = run_workload_xkbcommon_dl();
    println!("  xkbcommon-dl checksum: {c3}\n");

    let c4 = run_workload_xkbcommon_compat();
    println!("  xkbcommon-compat checksum: {c4}\n");

    black_box((c1, c2, c3, c4));

    println!("=== Done ===");
}
