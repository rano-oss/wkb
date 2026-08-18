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
            group.bench_function(BenchmarkId::new("wkb", case.name), |b| {
                b.iter(|| {
                    black_box(wkb_feed_compose(&mut wb, case.keys));
                });
            });
        }

        // wkb-noxkb: rebuilt from the precompiled RON layout (no XKB compile).
        {
            let mut wb = WKB::new_from_layouts(vec![load_layout_file("us", None)]).unwrap();
            wb.set_compose_key(COMPOSE_KEY);
            group.bench_function(BenchmarkId::new("wkb-noxkb", case.name), |b| {
                b.iter(|| {
                    black_box(wkb_feed_compose(&mut wb, case.keys));
                });
            });
        }

        // xkbcommon: keymap state + compose state driven by the same events.
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
            let mut state = xkb::State::new(&km);
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
                        &mut state,
                        &mut compose,
                        case.keys,
                        compose_kc,
                    ));
                });
            });
        }

        // xkbcommon-dl: same flow through the dynamic-loader FFI.
        {
            let xkb = xkbcommon_dl::xkbcommon_handle();
            let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
            let ctx = unsafe {
                (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS)
            };
            let c_layout = CString::new("us").unwrap();
            let names = xkbcommon_dl::xkb_rule_names {
                rules: c"evdev".as_ptr(),
                model: std::ptr::null(),
                layout: c_layout.as_ptr(),
                variant: std::ptr::null(),
                options: std::ptr::null(),
            };
            let km = unsafe {
                (xkb.xkb_keymap_new_from_names)(
                    ctx,
                    &names,
                    xkbcommon_dl::xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
                )
            };
            let st = unsafe { (xkb.xkb_state_new)(km) };
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
                    let mut out = None;
                    for &(evdev, down) in case.keys {
                        let kc = evdev + EVDEV_OFFSET;
                        let dir = if down {
                            xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                        } else {
                            xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                        };
                        unsafe { (xkb.xkb_state_update_key)(st, kc, dir) };
                        if !down {
                            continue;
                        }
                        let sym = unsafe { (xkb.xkb_state_key_get_one_sym)(st, kc) };
                        if is_modifier_keysym(sym) {
                            continue;
                        }
                        let feed = if kc == compose_kc {
                            XKB_KEY_MULTI_KEY
                        } else {
                            sym
                        };
                        unsafe { (xkb_compose.xkb_compose_state_feed)(cs, feed) };
                        let status = unsafe { (xkb_compose.xkb_compose_state_get_status)(cs) };
                        if status == xkbcommon_dl::xkb_compose_status::XKB_COMPOSE_COMPOSED {
                            let n = unsafe {
                                (xkb_compose.xkb_compose_state_get_utf8)(
                                    cs,
                                    utf8_buf.as_mut_ptr() as *mut _,
                                    utf8_buf.len(),
                                )
                            };
                            out = std::str::from_utf8(&utf8_buf[..n as usize])
                                .ok()
                                .and_then(|s| s.chars().next());
                        }
                    }
                    black_box(out);
                });
            });
            unsafe {
                (xkb_compose.xkb_compose_state_unref)(cs);
                (xkb_compose.xkb_compose_table_unref)(table);
                (xkb.xkb_state_unref)(st);
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
