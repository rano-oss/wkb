mod common;

use common::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::ffi::CString;
use std::hint::black_box;
use std::time::Duration;
use wkb::WKB;

fn cfg() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(10))
        .measurement_time(Duration::from_secs(1))
        .sample_size(50)
}

fn bench_compose_feed(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/feed");

    ensure_layout_file("us", None);

    for case in COMPOSE_CASES {
        // wkb: compiled from the XKB registry, compose key set explicitly.
        {
            let mut wb = {
                let saved_lc_all = std::env::var("LC_ALL").ok();
                unsafe { std::env::set_var("LC_ALL", COMPOSE_LOCALE) };
                let mut wb = WKB::new_from_names("", "", "us", "", None).unwrap();
                match saved_lc_all {
                    Some(v) => unsafe { std::env::set_var("LC_ALL", v) },
                    None => unsafe { std::env::remove_var("LC_ALL") },
                };
                wb.set_compose_key(COMPOSE_KEY);
                wb
            };
            let (_, _, mut comp_st) = xkbcommon_setup("us", None);
            group.bench_function(BenchmarkId::new("wkb", case.name), |b| {
                b.iter(|| {
                    black_box(wkb_feed_compose(&mut wb, &mut comp_st, case.keys));
                });
            });
        }

        // wkb-noxkb: rebuilt from the precompiled RON layout (no XKB compile).
        {
            let mut wb = WKB::new_from_layouts(vec![load_layout_file("us", None)]).unwrap();
            wb.set_compose_key(COMPOSE_KEY);
            let (_, _, mut comp_st) = xkbcommon_setup("us", None);
            group.bench_function(BenchmarkId::new("wkb-noxkb", case.name), |b| {
                b.iter(|| {
                    black_box(wkb_feed_compose(&mut wb, &mut comp_st, case.keys));
                });
            });
        }

        // xkbcommon: compositor key events + client state sync + compose feed.
        {
            use xkbcommon::xkb;
            let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
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
            let table = xkb::compose::Table::new_from_locale(
                &ctx,
                std::ffi::OsStr::new(COMPOSE_LOCALE),
                xkb::compose::COMPILE_NO_FLAGS,
            )
            .expect("compose table");
            let mut compose = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);
            let compose_kc = xkb::Keycode::new(COMPOSE_KEY + EVDEV_OFFSET);
            group.bench_function(BenchmarkId::new("xkbcommon", case.name), |b| {
                b.iter(|| {
                    black_box(xkb_feed_compose(
                        &mut comp_st,
                        &mut client_st,
                        &mut compose,
                        case.keys,
                        compose_kc,
                    ));
                });
            });
        }

        // xkbcommon-dl: same Wayland client flow through the dynamic-loader FFI.
        {
            let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
            let (xkb, ctx, km, comp_st, client_st) = xkbcommon_dl_dual_setup("us", None);
            let c_locale = CString::new(COMPOSE_LOCALE).unwrap();
            let table = unsafe {
                (xkb_compose.xkb_compose_table_new_from_locale)(
                    ctx,
                    c_locale.as_ptr(),
                    xkbcommon_dl::xkb_compose_compile_flags::XKB_COMPOSE_COMPILE_NO_FLAGS,
                )
            };
            let cs = unsafe {
                (xkb_compose.xkb_compose_state_new)(
                    table,
                    xkbcommon_dl::xkb_compose_state_flags::XKB_COMPOSE_STATE_NO_FLAGS,
                )
            };
            let compose_kc = COMPOSE_KEY + EVDEV_OFFSET;
            let mut utf8_buf = [0u8; 64];
            group.bench_function(BenchmarkId::new("xkbcommon-dl", case.name), |b| {
                b.iter(|| {
                    black_box(xkb_feed_compose_dl(
                        xkb,
                        xkb_compose,
                        comp_st,
                        client_st,
                        cs,
                        case.keys,
                        compose_kc,
                        &mut utf8_buf,
                    ));
                });
            });
            unsafe {
                (xkb_compose.xkb_compose_state_unref)(cs);
                (xkb_compose.xkb_compose_table_unref)(table);
                (xkb.xkb_state_unref)(client_st);
                (xkb.xkb_state_unref)(comp_st);
                (xkb.xkb_keymap_unref)(km);
                (xkb.xkb_context_unref)(ctx);
            }
        }
    }

    group.finish();
}

fn bench_compose_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/load");
    let path = std::path::Path::new(COMPOSE_FILE);
    if !path.exists() {
        println!("SKIP: compose file not found: {COMPOSE_FILE}");
        group.finish();
        return;
    }

    // wkb: full parse into a composer trie, bypassing the canonical-path cache.
    group.bench_function("wkb_cold_parse", |b| {
        b.iter(|| {
            black_box(wkb::load_compose_from_path_uncached(black_box(path)));
        });
    });

    // wkb: cached by canonical path — only the first call parses.
    group.bench_function("wkb_cached", |b| {
        b.iter(|| {
            black_box(wkb::load_compose_from_path(black_box(path)));
        });
    });

    // xkbcommon: compose table from locale (fresh table each call).
    {
        use xkbcommon::xkb;
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        group.bench_function("xkbcommon", |b| {
            b.iter(|| {
                let table = xkb::compose::Table::new_from_locale(
                    &ctx,
                    std::ffi::OsStr::new(COMPOSE_LOCALE),
                    xkb::compose::COMPILE_NO_FLAGS,
                );
                let _ = black_box(table);
            });
        });
    }

    // xkbcommon-dl: compose table from locale via the dynamic-loader FFI.
    {
        let xkb = xkbcommon_dl::xkbcommon_handle();
        let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
        let ctx =
            unsafe { (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };
        let c_locale = CString::new(COMPOSE_LOCALE).unwrap();
        group.bench_function("xkbcommon-dl", |b| {
            b.iter(|| {
                let table = unsafe {
                    (xkb_compose.xkb_compose_table_new_from_locale)(
                        ctx,
                        c_locale.as_ptr(),
                        xkbcommon_dl::xkb_compose_compile_flags::XKB_COMPOSE_COMPILE_NO_FLAGS,
                    )
                };
                if !table.is_null() {
                    unsafe { (xkb_compose.xkb_compose_table_unref)(table) };
                }
                black_box(table);
            });
        });
        unsafe { (xkb.xkb_context_unref)(ctx) };
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = cfg();
    targets = bench_compose_feed, bench_compose_load,
}
criterion_main!(benches);
