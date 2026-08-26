mod common;

use common::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::os::raw::c_char;
use std::time::Duration;

fn cfg() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(10))
        .measurement_time(Duration::from_secs(1))
        .sample_size(50)
}

macro_rules! bench_xkb {
    ($group:expr, $bid:expr, $locale:expr, $variant:expr, $case:expr, $body:expr) => {{
        use xkbcommon::xkb;
        let (_ctx, _km, mut st) = xkbcommon_setup($locale, $variant);
        let case_keys = $case.keys;
        $group.bench_function(BenchmarkId::new("xkbcommon", &$bid), |b| {
            b.iter(|| {
                for &(code, down) in case_keys {
                    let kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                    let dir = if down {
                        xkb::KeyDirection::Down
                    } else {
                        xkb::KeyDirection::Up
                    };
                    #[allow(clippy::redundant_closure_call)]
                    ($body)(&mut st, kc, down, dir);
                }
            });
        });
    }};
}

macro_rules! bench_dl {
    ($group:expr, $bid:expr, $locale:expr, $variant:expr, $case:expr, $body:expr) => {{
        let (xkb, ctx, km, st) = xkbcommon_dl_setup($locale, $variant);
        let case_keys = $case.keys;
        $group.bench_function(BenchmarkId::new("xkbcommon-dl", &$bid), |b| {
            b.iter(|| {
                for &(code, down) in case_keys {
                    let kc = code + EVDEV_OFFSET;
                    let dir = if down {
                        xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                    } else {
                        xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                    };
                    #[allow(clippy::redundant_closure_call)]
                    ($body)(xkb, st, kc, down, dir);
                }
            });
        });
        unsafe {
            (xkb.xkb_state_unref)(st);
            (xkb.xkb_keymap_unref)(km);
            (xkb.xkb_context_unref)(ctx);
        }
    }};
}

fn bench_client_update_modifiers(c: &mut Criterion) {
    let mut group = c.benchmark_group("client/update_modifiers");
    ensure_noxkb_fixtures();

    for case in KEY_CASES {
        for (lid, locale, variant) in layouts_for_case(case.name) {
            let bid = format!("{lid}/{}", case.name);
            let mut wb = wkb_setup(locale, variant);
            let (_, _, mut xkb_st) = xkbcommon_setup(locale, variant);
            let case_keys = case.keys;

            group.bench_function(BenchmarkId::new("wkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut xkb_st, code, down);
                        sync_client_modifiers(&mut wb, &xkb_st);
                        black_box(code);
                    }
                });
            });

            let mut wb = wkb_noxkb_setup(locale, variant);
            group.bench_function(BenchmarkId::new("wkb-noxkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut xkb_st, code, down);
                        sync_client_modifiers(&mut wb, &xkb_st);
                        black_box(code);
                    }
                });
            });

            // xkbcommon has no client-side update_modifiers; compare compositor-side
            // state update (what drives modifier sync over the wire).
            bench_xkb!(
                group,
                bid,
                locale,
                variant,
                case,
                |st: &mut xkbcommon::xkb::State,
                 kc: xkbcommon::xkb::Keycode,
                 _down: bool,
                 dir: xkbcommon::xkb::KeyDirection| {
                    black_box(st.update_key(kc, dir));
                }
            );

            bench_dl!(
                group,
                bid,
                locale,
                variant,
                case,
                |xkb: &xkbcommon_dl::XkbCommon,
                 st: *mut xkbcommon_dl::xkb_state,
                 kc: u32,
                 _down: bool,
                 dir: xkbcommon_dl::xkb_key_direction| {
                    black_box(unsafe { (xkb.xkb_state_update_key)(st, kc, dir) });
                }
            );
        }
    }

    group.finish();
}

fn bench_client_get_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("client/get_char");
    ensure_noxkb_fixtures();

    for case in KEY_CASES {
        for (lid, locale, variant) in layouts_for_case(case.name) {
            let bid = format!("{lid}/{}", case.name);
            let mut wb = wkb_setup(locale, variant);
            let (_, _, mut xkb_st) = xkbcommon_setup(locale, variant);
            let case_keys = case.keys;

            group.bench_function(BenchmarkId::new("wkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut xkb_st, code, down);
                        sync_client_modifiers(&mut wb, &xkb_st);
                        if down {
                            black_box(wb.key_char(black_box(code)));
                        }
                    }
                });
            });

            let mut wb = wkb_noxkb_setup(locale, variant);
            group.bench_function(BenchmarkId::new("wkb-noxkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut xkb_st, code, down);
                        sync_client_modifiers(&mut wb, &xkb_st);
                        if down {
                            black_box(wb.key_char(black_box(code)));
                        }
                    }
                });
            });

            bench_xkb!(
                group,
                bid,
                locale,
                variant,
                case,
                |st: &mut xkbcommon::xkb::State,
                 kc: xkbcommon::xkb::Keycode,
                 down: bool,
                 dir: xkbcommon::xkb::KeyDirection| {
                    st.update_key(kc, dir);
                    if down {
                        black_box(st.key_get_utf8(black_box(kc)));
                    }
                }
            );

            {
                let (xkb, ctx, km, st) = xkbcommon_dl_setup(locale, variant);
                let case_keys = case.keys;
                let mut buf = [0u8; 64];
                group.bench_function(BenchmarkId::new("xkbcommon-dl", &bid), |b| {
                    b.iter(|| {
                        for &(code, down) in case_keys {
                            let kc = code + EVDEV_OFFSET;
                            let dir = if down {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                            } else {
                                xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                            };
                            unsafe { (xkb.xkb_state_update_key)(st, kc, dir) };
                            if down {
                                black_box(unsafe {
                                    (xkb.xkb_state_key_get_utf8)(
                                        st,
                                        black_box(kc),
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
        }
    }

    group.finish();
}

fn bench_client_get_sym(c: &mut Criterion) {
    let mut group = c.benchmark_group("client/get_sym");
    ensure_noxkb_fixtures();

    for case in KEY_CASES {
        for (lid, locale, variant) in layouts_for_case(case.name) {
            let bid = format!("{lid}/{}", case.name);
            let mut wb = wkb_setup(locale, variant);
            let (_, _, mut xkb_st) = xkbcommon_setup(locale, variant);
            let case_keys = case.keys;

            group.bench_function(BenchmarkId::new("wkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut xkb_st, code, down);
                        sync_client_modifiers(&mut wb, &xkb_st);
                        if down {
                            black_box(wb.named_key(black_box(code)));
                        }
                    }
                });
            });

            let mut wb = wkb_noxkb_setup(locale, variant);
            group.bench_function(BenchmarkId::new("wkb-noxkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut xkb_st, code, down);
                        sync_client_modifiers(&mut wb, &xkb_st);
                        if down {
                            black_box(wb.named_key(black_box(code)));
                        }
                    }
                });
            });

            bench_xkb!(
                group,
                bid,
                locale,
                variant,
                case,
                |st: &mut xkbcommon::xkb::State,
                 kc: xkbcommon::xkb::Keycode,
                 down: bool,
                 dir: xkbcommon::xkb::KeyDirection| {
                    st.update_key(kc, dir);
                    if down {
                        black_box(st.key_get_one_sym(black_box(kc)));
                    }
                }
            );

            bench_dl!(
                group,
                bid,
                locale,
                variant,
                case,
                |xkb: &xkbcommon_dl::XkbCommon,
                 st: *mut xkbcommon_dl::xkb_state,
                 kc: u32,
                 down: bool,
                 dir: xkbcommon_dl::xkb_key_direction| {
                    unsafe { (xkb.xkb_state_update_key)(st, kc, dir) };
                    if down {
                        black_box(unsafe { (xkb.xkb_state_key_get_one_sym)(st, black_box(kc)) });
                    }
                }
            );
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = cfg();
    targets =
        bench_client_update_modifiers,
        bench_client_get_char,
        bench_client_get_sym,
}
criterion_main!(benches);
