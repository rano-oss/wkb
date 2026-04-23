mod common;

use common::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::time::Duration;

fn fast() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(500))
        .sample_size(20)
}

// ════════════════════════════════════════════════════════════════════════
// COMPAT HELPERS — call xkb-core Rust API directly (same code path as
// xkbcommon-compat, but avoids #[no_mangle] symbol collision with libxkbcommon)
// ════════════════════════════════════════════════════════════════════════

fn compat_setup(
    locale: &str,
    variant: Option<&str>,
) -> (xkb_core::rust_types::Keymap, xkb_core::rust_types::State) {
    use xkb_core::rust_types::{Context, RuleNames};
    let ctx = Context::new().expect("xkb-core context");
    let rmlvo = RuleNames {
        rules: "evdev".to_string(),
        model: String::new(),
        layout: locale.to_string(),
        variant: variant.unwrap_or("").to_string(),
        options: String::new(),
    };
    let km = ctx.keymap_from_names(&rmlvo).expect("xkb-core keymap");
    let st = km.new_state().expect("xkb-core state");
    (km, st)
}

// ════════════════════════════════════════════════════════════════════════
// 1. COMPOSE BENCHMARKS
// ════════════════════════════════════════════════════════════════════════

fn bench_compose_table_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/table_creation");
    let locale = COMPOSE_LOCALE;

    // ── wkb ────────────────────────────────────────────────────────────
    group.bench_function("wkb", |b| {
        b.iter(|| {
            let resolved = xkb_core::compose::resolve_compose_file(black_box(locale));
            if let Some(subpath) = resolved {
                let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
                let composer = wkb::xkb::load_compose_from_path(&path);
                black_box(composer);
            }
        });
    });

    // ── xkbcommon (linked) ─────────────────────────────────────────────
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

    // ── xkbcommon-dl ───────────────────────────────────────────────────
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

    // ── xkbcommon-compat ───────────────────────────────────────────────
    group.bench_function("xkbcommon-compat", |b| {
        b.iter(|| {
            let table = xkb_core::compose::ComposeTable::new_from_locale(black_box(locale));
            black_box(table);
        });
    });

    group.finish();
}

fn bench_compose_state_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/state_creation");
    let locale = COMPOSE_LOCALE;

    // ── wkb ────────────────────────────────────────────────────────────
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
                    let composer = wkb::xkb::load_compose_from_path(p);
                    black_box(composer);
                }
            });
        });
    }

    // ── xkbcommon ──────────────────────────────────────────────────────
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

    // ── xkbcommon-dl ───────────────────────────────────────────────────
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

    // ── xkbcommon-compat ───────────────────────────────────────────────
    {
        let table = xkb_core::compose::ComposeTable::new_from_locale(locale)
            .expect("xkb-core compose table");
        group.bench_function("xkbcommon-compat", |b| {
            b.iter(|| {
                let state = table.new_state();
                black_box(state);
            });
        });
    }

    group.finish();
}

fn bench_compose_feed(c: &mut Criterion) {
    let mut group = c.benchmark_group("compose/feed");

    for seq in COMPOSE_SEQUENCES {
        // ── wkb ────────────────────────────────────────────────────────
        {
            let resolved = xkb_core::compose::resolve_compose_file(COMPOSE_LOCALE);
            let path = resolved.map(|s| {
                std::path::Path::new("/usr/share/X11/locale")
                    .join(&s)
                    .to_path_buf()
            });
            if let Some(ref p) = path {
                let mut composer = wkb::xkb::load_compose_from_path(p);
                use wkb::composer::Composer;
                // Pre-convert keysyms to tokens outside the benchmark loop
                let tokens: Vec<wkb::composer::Token> = seq
                    .keysyms
                    .iter()
                    .filter_map(|&ks| {
                        xkb_core::keysym_utf::keysym_to_char(ks).map(wkb::composer::Token::Char)
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

        // ── xkbcommon ──────────────────────────────────────────────────
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

        // ── xkbcommon-dl ───────────────────────────────────────────────
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

        // ── xkbcommon-compat ───────────────────────────────────────────
        {
            if let Some(table) = xkb_core::compose::ComposeTable::new_from_locale(COMPOSE_LOCALE) {
                let mut state = table.new_state();
                group.bench_with_input(
                    BenchmarkId::new("xkbcommon-compat", seq.name),
                    &seq.keysyms,
                    |b, keysyms| {
                        b.iter(|| {
                            for &ks in *keysyms {
                                black_box(state.feed(ks));
                            }
                            state.reset();
                        });
                    },
                );
            }
        }
    }

    group.finish();
}

// ════════════════════════════════════════════════════════════════════════
// 2. KEY PRESS BENCHMARKS
// ════════════════════════════════════════════════════════════════════════

/// Helper: set up wkb for a given layout.
fn wkb_setup(locale: &str, variant: Option<&str>) -> wkb::WKB<wkb::ListComposer> {
    let layout = variant.map(String::from);
    wkb::xkb::new_from_names(locale.to_string(), layout)
}

/// Helper: set up xkbcommon context + keymap + state.
fn xkbcommon_setup(
    locale: &str,
    variant: Option<&str>,
) -> (
    xkbcommon::xkb::Context,
    xkbcommon::xkb::Keymap,
    xkbcommon::xkb::State,
) {
    use xkbcommon::xkb;
    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let km = xkb::Keymap::new_from_names(
        &ctx,
        "evdev",
        "",
        locale,
        variant.unwrap_or(""),
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .expect("xkbcommon keymap");
    let st = xkb::State::new(&km);
    (ctx, km, st)
}

/// Helper: set up xkbcommon-dl context + keymap + state.
fn xkbcommon_dl_setup(
    locale: &str,
    variant: Option<&str>,
) -> (
    &'static xkbcommon_dl::XkbCommon,
    *mut xkbcommon_dl::xkb_context,
    *mut xkbcommon_dl::xkb_keymap,
    *mut xkbcommon_dl::xkb_state,
) {
    let xkb = xkbcommon_dl::xkbcommon_handle();
    let ctx =
        unsafe { (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };
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
    (xkb, ctx, km, st)
}

fn bench_key_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("key/update");

    for &(locale, variant) in LAYOUTS {
        let layout_id = variant.map_or(locale.to_string(), |v| format!("{locale}_{v}"));

        for case in KEY_CASES {
            let bench_id = format!("{layout_id}/{}", case.name);

            // ── wkb ────────────────────────────────────────────────────
            {
                let mut wb = wkb_setup(locale, variant);
                group.bench_function(BenchmarkId::new("wkb", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let dir = if down {
                                wkb::modifiers::KeyDirection::Down
                            } else {
                                wkb::modifiers::KeyDirection::Up
                            };
                            black_box(wb.update_key(black_box(code), dir));
                        }
                    });
                });
            }

            // ── xkbcommon ──────────────────────────────────────────────
            {
                use xkbcommon::xkb;
                let (_ctx, _km, mut st) = xkbcommon_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                            let dir = if down {
                                xkb::KeyDirection::Down
                            } else {
                                xkb::KeyDirection::Up
                            };
                            black_box(st.update_key(xkb_kc, dir));
                        }
                    });
                });
            }

            // ── xkbcommon-dl ───────────────────────────────────────────
            {
                let (xkb, ctx, km, st) = xkbcommon_dl_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon-dl", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                            } else {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                            };
                            black_box(unsafe { (xkb.xkb_state_update_key)(st, xkb_kc, dir) });
                        }
                    });
                });
                unsafe {
                    (xkb.xkb_state_unref)(st);
                    (xkb.xkb_keymap_unref)(km);
                    (xkb.xkb_context_unref)(ctx);
                }
            }

            // ── xkbcommon-compat ───────────────────────────────────────
            {
                let (_km, mut st) = compat_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon-compat", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkb_core::XKB_KEY_DOWN
                            } else {
                                xkb_core::XKB_KEY_UP
                            };
                            black_box(st.update_key(xkb_kc, dir));
                        }
                    });
                });
            }
        }
    }
    group.finish();
}

fn bench_key_get_sym(c: &mut Criterion) {
    let mut group = c.benchmark_group("key/get_sym");

    for &(locale, variant) in LAYOUTS {
        let layout_id = variant.map_or(locale.to_string(), |v| format!("{locale}_{v}"));

        for case in KEY_CASES {
            let bench_id = format!("{layout_id}/{}", case.name);

            // ── wkb ────────────────────────────────────────────────────
            {
                let mut wb = wkb_setup(locale, variant);
                group.bench_function(BenchmarkId::new("wkb", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let dir = if down {
                                wkb::modifiers::KeyDirection::Down
                            } else {
                                wkb::modifiers::KeyDirection::Up
                            };
                            wb.update_key(code, dir);
                            if down {
                                black_box(wb.utf8(black_box(code)));
                            }
                        }
                    });
                });
            }

            // ── xkbcommon ──────────────────────────────────────────────
            {
                use xkbcommon::xkb;
                let (_ctx, _km, mut st) = xkbcommon_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                            let dir = if down {
                                xkb::KeyDirection::Down
                            } else {
                                xkb::KeyDirection::Up
                            };
                            st.update_key(xkb_kc, dir);
                            if down {
                                black_box(st.key_get_one_sym(black_box(xkb_kc)));
                            }
                        }
                    });
                });
            }

            // ── xkbcommon-dl ───────────────────────────────────────────
            {
                let (xkb, ctx, km, st) = xkbcommon_dl_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon-dl", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                            } else {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                            };
                            unsafe { (xkb.xkb_state_update_key)(st, xkb_kc, dir) };
                            if down {
                                black_box(unsafe {
                                    (xkb.xkb_state_key_get_one_sym)(st, black_box(xkb_kc))
                                });
                            }
                        }
                    });
                });
                unsafe {
                    (xkb.xkb_state_unref)(st);
                    (xkb.xkb_keymap_unref)(km);
                    (xkb.xkb_context_unref)(ctx);
                }
            }

            // ── xkbcommon-compat ───────────────────────────────────────
            {
                let (_km, mut st) = compat_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon-compat", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkb_core::XKB_KEY_DOWN
                            } else {
                                xkb_core::XKB_KEY_UP
                            };
                            st.update_key(xkb_kc, dir);
                            if down {
                                black_box(st.key_get_one_sym(black_box(xkb_kc)));
                            }
                        }
                    });
                });
            }
        }
    }
    group.finish();
}

fn bench_key_get_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("key/get_utf8");

    for &(locale, variant) in LAYOUTS {
        let layout_id = variant.map_or(locale.to_string(), |v| format!("{locale}_{v}"));

        for case in KEY_CASES {
            let bench_id = format!("{layout_id}/{}", case.name);

            // ── wkb ────────────────────────────────────────────────────
            {
                let mut wb = wkb_setup(locale, variant);
                group.bench_function(BenchmarkId::new("wkb", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let dir = if down {
                                wkb::modifiers::KeyDirection::Down
                            } else {
                                wkb::modifiers::KeyDirection::Up
                            };
                            wb.update_key(code, dir);
                            if down {
                                black_box(wb.utf8(black_box(code)));
                            }
                        }
                    });
                });
            }

            // ── xkbcommon ──────────────────────────────────────────────
            {
                use xkbcommon::xkb;
                let (_ctx, _km, mut st) = xkbcommon_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                            let dir = if down {
                                xkb::KeyDirection::Down
                            } else {
                                xkb::KeyDirection::Up
                            };
                            st.update_key(xkb_kc, dir);
                            if down {
                                black_box(st.key_get_utf8(black_box(xkb_kc)));
                            }
                        }
                    });
                });
            }

            // ── xkbcommon-dl ───────────────────────────────────────────
            {
                let (xkb, ctx, km, st) = xkbcommon_dl_setup(locale, variant);
                let mut buf = [0u8; 64];
                group.bench_function(BenchmarkId::new("xkbcommon-dl", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                            } else {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                            };
                            unsafe { (xkb.xkb_state_update_key)(st, xkb_kc, dir) };
                            if down {
                                black_box(unsafe {
                                    (xkb.xkb_state_key_get_utf8)(
                                        st,
                                        black_box(xkb_kc),
                                        buf.as_mut_ptr() as *mut c_char,
                                        buf.len(),
                                    )
                                });
                            }
                        }
                    });
                });
                unsafe {
                    (xkb.xkb_state_unref)(st);
                    (xkb.xkb_keymap_unref)(km);
                    (xkb.xkb_context_unref)(ctx);
                }
            }

            // ── xkbcommon-compat ───────────────────────────────────────
            {
                let (_km, mut st) = compat_setup(locale, variant);
                group.bench_function(BenchmarkId::new("xkbcommon-compat", &bench_id), |b| {
                    b.iter(|| {
                        for &(code, down) in case.keys {
                            let xkb_kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkb_core::XKB_KEY_DOWN
                            } else {
                                xkb_core::XKB_KEY_UP
                            };
                            st.update_key(xkb_kc, dir);
                            if down {
                                black_box(st.key_get_utf8(black_box(xkb_kc)));
                            }
                        }
                    });
                });
            }
        }
    }
    group.finish();
}

// ════════════════════════════════════════════════════════════════════════
// 4. FULL SETUP BENCHMARKS — total cost from zero to ready-to-use
// ════════════════════════════════════════════════════════════════════════

fn bench_full_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_setup");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(2));
    let locale = "us";

    // ── wkb: new_from_names builds keymap + flat tables + compose in one call
    group.bench_function("wkb", |b| {
        b.iter(|| {
            let wkb = wkb_setup(black_box(locale), None);
            black_box(wkb);
        });
    });

    // ── xkbcommon: context + keymap + state + compose_table + compose_state
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
            let compose_state = table
                .as_ref()
                .map(|t| xkb::compose::State::new(t, xkb::compose::STATE_NO_FLAGS));
            black_box(&table);
            black_box((ctx, km, st, compose_state));
        });
    });

    // ── xkbcommon-dl: same via dlopen
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
            // Cleanup
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

    // ── xkbcommon-compat: xkb-core Rust API
    group.bench_function("xkbcommon-compat", |b| {
        b.iter(|| {
            let (km, st) = compat_setup(black_box(locale), None);
            let table =
                xkb_core::compose::ComposeTable::new_from_locale(black_box(COMPOSE_LOCALE));
            let cs = table.as_ref().map(|t| t.new_state());
            black_box(&table);
            black_box((km, st, cs));
        });
    });

    group.finish();
}

// ════════════════════════════════════════════════════════════════════════
// CRITERION ENTRY
// ════════════════════════════════════════════════════════════════════════

criterion_group! {
    name = benches;
    config = fast();
    targets =
        bench_full_setup,
        bench_compose_table_creation,
        bench_compose_state_creation,
        bench_compose_feed,
        bench_key_update,
        bench_key_get_sym,
        bench_key_get_utf8,
}
criterion_main!(benches);
