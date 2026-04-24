mod common;

use common::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::ffi::CString;
use std::time::Duration;
use wkb::testing::ListComposerTestExt;

fn cfg() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(50))
        .measurement_time(Duration::from_millis(200))
        .sample_size(10)
}

fn bench_compose_table_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/table_creation");
    let locale = COMPOSE_LOCALE;

    group.bench_function("wkb", |b| {
        b.iter(|| {
            let resolved = xkb_core::compose::resolve_compose_file(black_box(locale));
            if let Some(subpath) = resolved {
                let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
                let composer = wkb::testing::compose_parse::load_compose_from_path(&path);
                black_box(composer);
            }
        });
    });

    group.bench_function("xkbcommon", |b| {
        use xkbcommon::xkb;
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let locale_os = std::ffi::OsStr::new(locale);
        b.iter(|| {
            let table = xkb::compose::Table::new_from_locale(
                &ctx,
                locale_os,
                xkb::compose::COMPILE_NO_FLAGS,
            );
            let _ = black_box(table);
        });
    });

    group.bench_function("xkbcommon-dl", |b| {
        let xkb = xkbcommon_dl::xkbcommon_handle();
        let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
        let ctx =
            unsafe { (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };
        let c_locale = CString::new(locale).unwrap();
        b.iter(|| {
            let table = unsafe {
                (xkb_compose.xkb_compose_table_new_from_locale)(
                    ctx,
                    c_locale.as_ptr(),
                    xkbcommon_dl::xkb_compose_compile_flags::XKB_COMPOSE_COMPILE_NO_FLAGS,
                )
            };
            black_box(table);
            if !table.is_null() {
                unsafe { (xkb_compose.xkb_compose_table_unref)(table) };
            }
        });
        unsafe { (xkb.xkb_context_unref)(ctx) };
    });

    group.finish();
}

fn bench_compose_state_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/state_creation");
    let locale = COMPOSE_LOCALE;

    {
        let resolved = xkb_core::compose::resolve_compose_file(locale);
        let path = resolved.map(|s| {
            std::path::Path::new("/usr/share/X11/locale")
                .join(&s)
                .to_path_buf()
        });
        group.bench_function("wkb", |b| {
            b.iter(|| {
                if let Some(ref p) = path {
                    let composer = wkb::testing::compose_parse::load_compose_from_path(p);
                    black_box(composer);
                }
            });
        });
    }

    {
        use xkbcommon::xkb;
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let locale_os = std::ffi::OsStr::new(locale);
        let table =
            xkb::compose::Table::new_from_locale(&ctx, locale_os, xkb::compose::COMPILE_NO_FLAGS)
                .expect("compose table");
        group.bench_function("xkbcommon", |b| {
            b.iter(|| {
                let state = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);
                black_box(state);
            });
        });
    }

    {
        let xkb = xkbcommon_dl::xkbcommon_handle();
        let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
        let ctx =
            unsafe { (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };
        let c_locale = CString::new(locale).unwrap();
        let table = unsafe {
            (xkb_compose.xkb_compose_table_new_from_locale)(
                ctx,
                c_locale.as_ptr(),
                xkbcommon_dl::xkb_compose_compile_flags::XKB_COMPOSE_COMPILE_NO_FLAGS,
            )
        };
        group.bench_function("xkbcommon-dl", |b| {
            b.iter(|| {
                let state = unsafe {
                    (xkb_compose.xkb_compose_state_new)(
                        table,
                        xkbcommon_dl::xkb_compose_state_flags::XKB_COMPOSE_STATE_NO_FLAGS,
                    )
                };
                black_box(state);
                if !state.is_null() {
                    unsafe { (xkb_compose.xkb_compose_state_unref)(state) };
                }
            });
        });
        unsafe {
            (xkb_compose.xkb_compose_table_unref)(table);
            (xkb.xkb_context_unref)(ctx);
        };
    }

    group.finish();
}

fn bench_compose_feed(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/feed");

    for seq in COMPOSE_SEQUENCES {
        {
            let resolved = xkb_core::compose::resolve_compose_file(COMPOSE_LOCALE);
            let path = resolved.map(|s| {
                std::path::Path::new("/usr/share/X11/locale")
                    .join(&s)
                    .to_path_buf()
            });
            if let Some(ref p) = path {
                let mut composer = wkb::testing::compose_parse::load_compose_from_path(p);
                let tokens: Vec<wkb::testing::Token> = seq
                    .keysyms
                    .iter()
                    .filter_map(|&ks| {
                        xkb_core::keysym_utf::keysym_to_char(ks).map(wkb::testing::Token::Char)
                    })
                    .collect();
                group.bench_with_input(BenchmarkId::new("wkb", seq.name), &tokens, |b, tokens| {
                    b.iter(|| {
                        for token in tokens {
                            black_box(composer.feed(*token));
                        }
                    });
                });
            }
        }

        {
            use xkbcommon::xkb;
            let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
            let locale_os = std::ffi::OsStr::new(COMPOSE_LOCALE);
            if let Ok(table) = xkb::compose::Table::new_from_locale(
                &ctx,
                locale_os,
                xkb::compose::COMPILE_NO_FLAGS,
            ) {
                let mut state = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);
                group.bench_with_input(
                    BenchmarkId::new("xkbcommon", seq.name),
                    &seq.keysyms,
                    |b, keysyms| {
                        b.iter(|| {
                            for &ks in *keysyms {
                                black_box(state.feed(xkb::Keysym::new(ks)));
                            }
                            state.reset();
                        });
                    },
                );
            }
        }

        {
            let xkb = xkbcommon_dl::xkbcommon_handle();
            let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
            let ctx = unsafe {
                (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS)
            };
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
                group.bench_with_input(
                    BenchmarkId::new("xkbcommon-dl", seq.name),
                    &seq.keysyms,
                    |b, keysyms| {
                        b.iter(|| {
                            for &ks in *keysyms {
                                black_box(unsafe {
                                    (xkb_compose.xkb_compose_state_feed)(state, ks)
                                });
                            }
                            unsafe { (xkb_compose.xkb_compose_state_reset)(state) };
                        });
                    },
                );
                unsafe {
                    (xkb_compose.xkb_compose_state_unref)(state);
                    (xkb_compose.xkb_compose_table_unref)(table);
                    (xkb.xkb_context_unref)(ctx);
                };
            }
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = cfg();
    targets =
        bench_compose_table_creation,
        bench_compose_state_creation,
        bench_compose_feed,
}
criterion_main!(benches);
