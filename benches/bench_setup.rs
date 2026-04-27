mod common;

use common::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ffi::CString;
use std::ptr;
use std::time::Duration;

fn cfg() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(50))
        .measurement_time(Duration::from_millis(300))
        .sample_size(10)
}

fn bench_setup_no_compose(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup/no_compose");
    let locale = "us";

    group.bench_function("wkb", |b| {
        // Temporarily unset locale env vars so compose is not loaded
        let saved = std::env::var("LC_ALL").ok();
        std::env::set_var("LC_ALL", "C");
        b.iter(|| {
            let wkb: wkb::WKB =
                wkb::WKB::new_from_names("", "", black_box(locale), "", None).unwrap();
            black_box(wkb);
        });
        if let Some(v) = saved {
            std::env::set_var("LC_ALL", v);
        } else {
            std::env::remove_var("LC_ALL");
        }
    });

    group.bench_function("xkbcommon", |b| {
        use xkbcommon::xkb;
        b.iter(|| {
            let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
            let km = xkb::Keymap::new_from_names(
                &ctx,
                "evdev",
                "",
                black_box(locale),
                "",
                None,
                xkb::KEYMAP_COMPILE_NO_FLAGS,
            )
            .expect("keymap");
            let st = xkb::State::new(&km);
            let _ = black_box((ctx, km, st));
        });
    });

    group.bench_function("xkbcommon-dl", |b| {
        let xkb = xkbcommon_dl::xkbcommon_handle();
        b.iter(|| {
            let ctx = unsafe {
                (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS)
            };
            let rmlvo = xkbcommon_dl::xkb_rule_names {
                rules: c"evdev".as_ptr(),
                model: ptr::null(),
                layout: c"us".as_ptr(),
                variant: ptr::null(),
                options: ptr::null(),
            };
            let km = unsafe {
                (xkb.xkb_keymap_new_from_names)(
                    ctx,
                    &rmlvo,
                    xkbcommon_dl::xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
                )
            };
            let st = unsafe { (xkb.xkb_state_new)(km) };
            black_box((ctx, km, st));
            unsafe {
                (xkb.xkb_state_unref)(st);
                (xkb.xkb_keymap_unref)(km);
                (xkb.xkb_context_unref)(ctx);
            }
        });
    });

    group.finish();
}

fn bench_setup_with_compose(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup/with_compose");
    let locale = "us";

    group.bench_function("wkb", |b| {
        // Ensure compose locale resolves
        std::env::set_var("LC_ALL", COMPOSE_LOCALE);
        b.iter(|| {
            let wkb: wkb::WKB =
                wkb::WKB::new_from_names("", "", black_box(locale), "", None).unwrap();
            black_box(wkb);
        });
    });

    group.bench_function("xkbcommon", |b| {
        use xkbcommon::xkb;
        b.iter(|| {
            let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
            let km = xkb::Keymap::new_from_names(
                &ctx,
                "evdev",
                "",
                black_box(locale),
                "",
                None,
                xkb::KEYMAP_COMPILE_NO_FLAGS,
            )
            .expect("keymap");
            let st = xkb::State::new(&km);
            let locale_os = std::ffi::OsStr::new(COMPOSE_LOCALE);
            let table = xkb::compose::Table::new_from_locale(
                &ctx,
                locale_os,
                xkb::compose::COMPILE_NO_FLAGS,
            );
            let cs = table
                .as_ref()
                .map(|t| xkb::compose::State::new(t, xkb::compose::STATE_NO_FLAGS));
            black_box(&table);
            black_box(&cs);
            let _ = black_box((ctx, km, st));
        });
    });

    group.bench_function("xkbcommon-dl", |b| {
        let xkb = xkbcommon_dl::xkbcommon_handle();
        let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
        let c_locale = CString::new(COMPOSE_LOCALE).unwrap();
        b.iter(|| {
            let ctx = unsafe {
                (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS)
            };
            let rmlvo = xkbcommon_dl::xkb_rule_names {
                rules: c"evdev".as_ptr(),
                model: ptr::null(),
                layout: c"us".as_ptr(),
                variant: ptr::null(),
                options: ptr::null(),
            };
            let km = unsafe {
                (xkb.xkb_keymap_new_from_names)(
                    ctx,
                    &rmlvo,
                    xkbcommon_dl::xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
                )
            };
            let st = unsafe { (xkb.xkb_state_new)(km) };
            let table = unsafe {
                (xkb_compose.xkb_compose_table_new_from_locale)(
                    ctx,
                    c_locale.as_ptr(),
                    xkbcommon_dl::xkb_compose_compile_flags::XKB_COMPOSE_COMPILE_NO_FLAGS,
                )
            };
            let cs = if !table.is_null() {
                unsafe {
                    (xkb_compose.xkb_compose_state_new)(
                        table,
                        xkbcommon_dl::xkb_compose_state_flags::XKB_COMPOSE_STATE_NO_FLAGS,
                    )
                }
            } else {
                ptr::null_mut()
            };
            black_box((ctx, km, st, table, cs));
            if !cs.is_null() {
                unsafe { (xkb_compose.xkb_compose_state_unref)(cs) };
            }
            if !table.is_null() {
                unsafe { (xkb_compose.xkb_compose_table_unref)(table) };
            }
            unsafe {
                (xkb.xkb_state_unref)(st);
                (xkb.xkb_keymap_unref)(km);
                (xkb.xkb_context_unref)(ctx);
            }
        });
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = cfg();
    targets = bench_setup_no_compose, bench_setup_with_compose,
}
criterion_main!(benches);
