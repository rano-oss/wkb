mod common;

use common::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
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

fn wkb_setup(locale: &str, variant: Option<&str>) -> wkb::WKB {
    wkb::WKB::new_from_names("", "", locale, variant.unwrap_or(""), None).unwrap()
}

fn wkb_noxkb_setup(locale: &str, variant: Option<&str>) -> wkb::WKB {
    wkb::WKB::new_from_layouts(vec![load_layout_file(locale, variant)]).unwrap()
}

fn ensure_noxkb_fixtures() {
    let (pl, pv) = PRIMARY_LAYOUT;
    ensure_layout_file(pl, pv);
    for &(l, v) in EXTRA_LAYOUTS {
        ensure_layout_file(l, v);
    }
}

fn layouts_for_case(case_name: &str) -> Vec<(String, &'static str, Option<&'static str>)> {
    let (pl, pv) = PRIMARY_LAYOUT;
    let mut out = vec![(pv.map_or(pl.to_string(), |v| format!("{pl}_{v}")), pl, pv)];
    if LAYOUT_SENSITIVE_CASES.contains(&case_name) {
        for &(l, v) in EXTRA_LAYOUTS {
            out.push((v.map_or(l.to_string(), |vv| format!("{l}_{vv}")), l, v));
        }
    }
    out
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
