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

/// xkbcommon client path: compositor state update + client state sync.
macro_rules! bench_xkb_client {
    ($group:expr, $bid:expr, $locale:expr, $variant:expr, $case:expr, $body:expr) => {{
        let (_ctx, _km, mut comp_st, mut client_st) = xkbcommon_dual_setup($locale, $variant);
        let case_keys = $case.keys;
        $group.bench_function(BenchmarkId::new("xkbcommon", &$bid), |b| {
            b.iter(|| {
                for &(code, down) in case_keys {
                    xkb_update_key(&mut comp_st, code, down);
                    sync_xkb_client_state(&comp_st, &mut client_st);
                    #[allow(clippy::redundant_closure_call)]
                    ($body)(&mut client_st, code, down);
                }
            });
        });
    }};
}

/// xkbcommon-dl client path: compositor state update + client state sync.
macro_rules! bench_dl_client {
    ($group:expr, $bid:expr, $locale:expr, $variant:expr, $case:expr, $body:expr) => {{
        let (xkb, ctx, km, comp_st, client_st) = xkbcommon_dl_dual_setup($locale, $variant);
        let case_keys = $case.keys;
        $group.bench_function(BenchmarkId::new("xkbcommon-dl", &$bid), |b| {
            b.iter(|| {
                for &(code, down) in case_keys {
                    xkb_update_key_dl(xkb, comp_st, code, down);
                    sync_xkb_client_state_dl(xkb, comp_st, client_st);
                    #[allow(clippy::redundant_closure_call)]
                    ($body)(xkb, client_st, code, down);
                }
            });
        });
        unsafe {
            (xkb.xkb_state_unref)(client_st);
            (xkb.xkb_state_unref)(comp_st);
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
            let (_, _, mut comp_st) = xkbcommon_setup(locale, variant);
            let case_keys = case.keys;

            group.bench_function(BenchmarkId::new("wkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut comp_st, code, down);
                        sync_client_modifiers(&mut wb, &comp_st);
                        black_box(code);
                    }
                });
            });

            let mut wb = wkb_noxkb_setup(locale, variant);
            group.bench_function(BenchmarkId::new("wkb-noxkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut comp_st, code, down);
                        sync_client_modifiers(&mut wb, &comp_st);
                        black_box(code);
                    }
                });
            });

            bench_xkb_client!(group, bid, locale, variant, case, |_client_st: &mut xkbcommon::xkb::State, code, _down| {
                black_box(code);
            });

            bench_dl_client!(
                group,
                bid,
                locale,
                variant,
                case,
                |_xkb: &xkbcommon_dl::XkbCommon, _client_st: *mut xkbcommon_dl::xkb_state, code, _down| {
                    black_box(code);
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
            let (_, _, mut comp_st) = xkbcommon_setup(locale, variant);
            let case_keys = case.keys;

            group.bench_function(BenchmarkId::new("wkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut comp_st, code, down);
                        sync_client_modifiers(&mut wb, &comp_st);
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
                        xkb_update_key(&mut comp_st, code, down);
                        sync_client_modifiers(&mut wb, &comp_st);
                        if down {
                            black_box(wb.key_char(black_box(code)));
                        }
                    }
                });
            });

            bench_xkb_client!(group, bid, locale, variant, case, |client_st: &mut xkbcommon::xkb::State, code, down| {
                use xkbcommon::xkb;
                if down {
                    let kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                    black_box(client_st.key_get_utf8(black_box(kc)));
                }
            });

            {
                let (xkb, ctx, km, comp_st, client_st) = xkbcommon_dl_dual_setup(locale, variant);
                let case_keys = case.keys;
                let mut buf = [0u8; 64];
                group.bench_function(BenchmarkId::new("xkbcommon-dl", &bid), |b| {
                    b.iter(|| {
                        for &(code, down) in case_keys {
                            xkb_update_key_dl(xkb, comp_st, code, down);
                            sync_xkb_client_state_dl(xkb, comp_st, client_st);
                            if down {
                                let kc = code + EVDEV_OFFSET;
                                black_box(unsafe {
                                    (xkb.xkb_state_key_get_utf8)(
                                        client_st,
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
                    (xkb.xkb_state_unref)(client_st);
                    (xkb.xkb_state_unref)(comp_st);
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
            let (_, _, mut comp_st) = xkbcommon_setup(locale, variant);
            let case_keys = case.keys;

            group.bench_function(BenchmarkId::new("wkb", &bid), |b| {
                b.iter(|| {
                    for &(code, down) in case_keys {
                        xkb_update_key(&mut comp_st, code, down);
                        sync_client_modifiers(&mut wb, &comp_st);
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
                        xkb_update_key(&mut comp_st, code, down);
                        sync_client_modifiers(&mut wb, &comp_st);
                        if down {
                            black_box(wb.named_key(black_box(code)));
                        }
                    }
                });
            });

            bench_xkb_client!(group, bid, locale, variant, case, |client_st: &mut xkbcommon::xkb::State, code, down| {
                use xkbcommon::xkb;
                if down {
                    let kc = xkb::Keycode::new(code + EVDEV_OFFSET);
                    black_box(client_st.key_get_one_sym(black_box(kc)));
                }
            });

            bench_dl_client!(group, bid, locale, variant, case, |xkb: &xkbcommon_dl::XkbCommon, client_st: *mut xkbcommon_dl::xkb_state, code, down| {
                if down {
                    let kc = code + EVDEV_OFFSET;
                    black_box(unsafe { (xkb.xkb_state_key_get_one_sym)(client_st, black_box(kc)) });
                }
            });
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
