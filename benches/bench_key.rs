mod common;

use common::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::time::Duration;
use wkb::testing::{KeyDirection, WKBTestExt};

fn cfg() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(50))
        .measurement_time(Duration::from_millis(200))
        .sample_size(10)
}

// ── Setup helpers ──────────────────────────────────────────────────────

fn wkb_setup(locale: &str, variant: Option<&str>) -> wkb::WKB {
    wkb::WKB::new_from_names("", "", locale, variant.unwrap_or(""), None).unwrap()
}

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

/// Iterate over (layout_id, locale, variant) for a given case.
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

// ── Macros to reduce per-impl boilerplate ──────────────────────────────

macro_rules! bench_wkb {
    ($group:expr, $bid:expr, $locale:expr, $variant:expr, $case:expr, $body:expr) => {{
        let mut wb = wkb_setup($locale, $variant);
        let case_keys = $case.keys;
        $group.bench_function(BenchmarkId::new("wkb", &$bid), |b| {
            b.iter(|| {
                for &(code, down) in case_keys {
                    let dir = if down {
                        KeyDirection::Down
                    } else {
                        KeyDirection::Up
                    };
                    #[allow(clippy::redundant_closure_call)]
                    ($body)(&mut wb, code, down, dir);
                }
            });
        });
    }};
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

// ── key/update ─────────────────────────────────────────────────────────

fn bench_key_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("key/update");

    for case in KEY_CASES {
        for (lid, locale, variant) in layouts_for_case(case.name) {
            let bid = format!("{lid}/{}", case.name);

            bench_wkb!(
                group,
                bid,
                locale,
                variant,
                case,
                |wb: &mut wkb::WKB, code: u32, _down: bool, dir: KeyDirection| {
                    black_box(wb.update_key(black_box(code), dir));
                }
            );

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

// ── key/get_utf8 ───────────────────────────────────────────────────────

fn bench_key_get_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("key/get_utf8");

    for case in KEY_CASES {
        for (lid, locale, variant) in layouts_for_case(case.name) {
            let bid = format!("{lid}/{}", case.name);

            bench_wkb!(
                group,
                bid,
                locale,
                variant,
                case,
                |wb: &mut wkb::WKB, code: u32, down: bool, dir: KeyDirection| {
                    wb.update_key(code, dir);
                    if down {
                        black_box(wb.key_char(black_box(code)));
                    }
                }
            );

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

// ── key/get_sym ────────────────────────────────────────────────────────

fn bench_key_get_sym(c: &mut Criterion) {
    let mut group = c.benchmark_group("key/get_sym");

    for case in KEY_CASES {
        for (lid, locale, variant) in layouts_for_case(case.name) {
            let bid = format!("{lid}/{}", case.name);

            bench_wkb!(
                group,
                bid,
                locale,
                variant,
                case,
                |wb: &mut wkb::WKB, code: u32, down: bool, dir: KeyDirection| {
                    wb.update_key(code, dir);
                    if down {
                        black_box(wb.key_char(black_box(code)));
                    }
                }
            );

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
        bench_key_update,
        bench_key_get_utf8,
        bench_key_get_sym,
}
criterion_main!(benches);
