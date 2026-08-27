//! Memory benchmark — measure peak RSS per backend.
//!
//! Backends are distinguished by *construction path*, matching the size
//! benchmarks:
//! - `wkb-noxkb` — rebuilds layouts from precompiled RON files via
//!   [`wkb::WKB::new_from_layouts`] and drives them through the public event
//!   API. Run first, before any XKB compilation happens, so its RSS reflects
//!   the no-XKB usage pattern (the XKB code paths are never exercised).
//! - `wkb` — compiles layouts from the XKB registry via
//!   [`wkb::WKB::new_from_names`].
//! - `xkbcommon`, `xkbcommon-dl`, `xkbcommon-compat` — the C backends.
//!
//! Hot-path workloads use the Wayland **client** model: compositor key events
//! update a reference `xkb::State`, client state is synced via serialized
//! modifiers (`update_modifiers` / `update_mask`), then char lookup or compose
//! feed runs on key down.
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
use wkb::WKB;

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

/// wkb without XKB: rebuild layouts from precompiled RON files and drive
/// them through the public client event API.
fn run_workload_wkb_noxkb() -> u64 {
    let mut checksum: u64 = 0;

    for &(locale, variant) in LAYOUTS {
        ensure_layout_file(locale, variant);
    }

    print_rss("wkb-noxkb/before_setup");

    for &(locale, variant) in LAYOUTS {
        let mut wb = wkb::WKB::new_from_layouts(vec![load_layout_file(locale, variant)]).unwrap();
        let (_, _, mut comp_st) = xkbcommon_setup(locale, variant);

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                checksum = checksum.wrapping_add(checksum_wkb_client_keys(
                    &mut wb,
                    &mut comp_st,
                    case.keys,
                ));
            }
        }
    }

    print_rss("wkb-noxkb/after_workload");
    checksum
}

/// wkb with XKB: compile layouts from the XKB registry.
fn run_workload_wkb_xkb() -> u64 {
    let mut checksum: u64 = 0;

    print_rss("wkb/before_setup");

    for &(locale, variant) in LAYOUTS {
        let mut wb = wkb::WKB::new_from_names("", "", locale, variant.unwrap_or(""), None).unwrap();
        let (_, _, mut comp_st) = xkbcommon_setup(locale, variant);

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                checksum = checksum.wrapping_add(checksum_wkb_client_keys(
                    &mut wb,
                    &mut comp_st,
                    case.keys,
                ));
            }
        }
    }

    unsafe { std::env::set_var("LC_ALL", COMPOSE_LOCALE) };
    let mut wb = WKB::new_from_names("", "", "us", "", None).unwrap();
    wb.set_compose_key(COMPOSE_KEY);
    let (_, _, mut comp_st) = xkbcommon_setup("us", None);
    for case in COMPOSE_CASES {
        for _ in 0..HOT_PATH_ITERATIONS {
            let c = wkb_feed_compose(&mut wb, &mut comp_st, case.keys);
            checksum = checksum.wrapping_add(c.map_or(0, |c| c as u64));
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
        let mut comp_st = xkb::State::new(&km);
        let mut client_st = xkb::State::new(&km);

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                checksum = checksum.wrapping_add(checksum_xkb_client_keys(
                    &mut comp_st,
                    &mut client_st,
                    case.keys,
                ));
            }
        }
    }

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
            for _ in 0..HOT_PATH_ITERATIONS {
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
        let (xkb, ctx, km, comp_st, client_st) = xkbcommon_dl_dual_setup(locale, variant);
        let mut buf = [0u8; 64];

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                checksum = checksum.wrapping_add(checksum_xkb_client_keys_dl(
                    xkb,
                    comp_st,
                    client_st,
                    case.keys,
                    &mut buf,
                ));
            }
        }

        unsafe {
            (xkb.xkb_state_unref)(client_st);
            (xkb.xkb_state_unref)(comp_st);
            (xkb.xkb_keymap_unref)(km);
            (xkb.xkb_context_unref)(ctx);
        }
    }

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
        let (xkb, _ctx, km, comp_st, client_st) = xkbcommon_dl_dual_setup("us", None);
        let cs = unsafe {
            (xkb_compose.xkb_compose_state_new)(
                table,
                xkbcommon_dl::xkb_compose_state_flags::XKB_COMPOSE_STATE_NO_FLAGS,
            )
        };
        let compose_kc = COMPOSE_KEY + EVDEV_OFFSET;
        let mut utf8_buf = [0u8; 64];
        for case in COMPOSE_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                let c = xkb_feed_compose_dl(
                    xkb,
                    xkb_compose,
                    comp_st,
                    client_st,
                    cs,
                    case.keys,
                    compose_kc,
                    &mut utf8_buf,
                );
                checksum = checksum.wrapping_add(c.map_or(0, |c| c as u64));
            }
        }
        unsafe {
            (xkb_compose.xkb_compose_state_unref)(cs);
            (xkb_compose.xkb_compose_table_unref)(table);
            (xkb.xkb_state_unref)(client_st);
            (xkb.xkb_state_unref)(comp_st);
            (xkb.xkb_keymap_unref)(km);
        }
    }

    unsafe { (xkb.xkb_context_unref)(ctx) };

    print_rss("xkbcommon-dl/after_workload");
    checksum
}

fn run_workload_xkbcommon_compat() -> u64 {
    use xkbcommon::xkb;
    let mut checksum: u64 = 0;

    print_rss("xkbcommon-compat/before_setup");

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
        let mut comp_st = xkb::State::new(&km);
        let mut client_st = xkb::State::new(&km);

        for case in KEY_CASES {
            for _ in 0..HOT_PATH_ITERATIONS {
                checksum = checksum.wrapping_add(checksum_xkb_client_keys(
                    &mut comp_st,
                    &mut client_st,
                    case.keys,
                ));
            }
        }
    }

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
            for _ in 0..HOT_PATH_ITERATIONS {
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
    }

    print_rss("xkbcommon-compat/after_workload");
    checksum
}

fn main() {
    println!("=== Memory Benchmark ===\n");

    print_rss("baseline");
    println!();

    let c0 = run_workload_wkb_noxkb();
    println!("  wkb-noxkb checksum: {c0}\n");

    let c1 = run_workload_wkb_xkb();
    println!("  wkb checksum: {c1}\n");

    let c2 = run_workload_xkbcommon();
    println!("  xkbcommon checksum: {c2}\n");

    let c3 = run_workload_xkbcommon_dl();
    println!("  xkbcommon-dl checksum: {c3}\n");

    let c4 = run_workload_xkbcommon_compat();
    println!("  xkbcommon-compat checksum: {c4}\n");

    black_box((c0, c1, c2, c3, c4));

    println!("=== Done ===");
}
