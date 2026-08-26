mod common;

use common::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::ptr;
use std::time::Duration;

fn cfg() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(10))
        .measurement_time(Duration::from_secs(1))
        .sample_size(20)
}

fn without_compose<T>(f: impl FnOnce() -> T) -> T {
    let saved = ["LC_ALL", "LC_CTYPE", "LANG"].map(|name| (name, std::env::var(name).ok()));
    for (name, _) in &saved {
        unsafe { std::env::remove_var(name) };
    }
    let result = f();
    for (name, value) in saved {
        unsafe {
            match value {
                Some(value) => std::env::set_var(name, value),
                None => std::env::remove_var(name),
            }
        }
    }
    result
}

fn bench_setup_no_compose(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup/no_compose");
    let locale = "us";
    let multi_layout = "us,de,fr,ru";

    for l in multi_layout.split(',') {
        ensure_layout_file(l, None);
    }

    group.bench_function("wkb", |b| {
        without_compose(|| {
            b.iter(|| {
                let wkb: wkb::WKB =
                    wkb::WKB::new_from_names("", "", black_box(locale), "", None).unwrap();
                black_box(wkb);
            });
        });
    });

    group.bench_function("wkb-noxkb", |b| {
        without_compose(|| {
            b.iter(|| {
                let file = load_layout_file(black_box(locale), None);
                let wkb = wkb::WKB::new_from_layouts(vec![file]).unwrap();
                black_box(wkb);
            });
        });
    });

    let keymap = without_compose(|| {
        wkb::WKB::new_from_names("", "", locale, "", None)
            .unwrap()
            .as_xkb_string()
            .unwrap()
    });
    group.bench_function("wkb_xkb_string", |b| {
        without_compose(|| {
            b.iter(|| {
                let wkb = wkb::WKB::new_from_string(black_box(&keymap)).unwrap();
                black_box(wkb);
            });
        });
    });

    group.bench_function("wkb_multilayout", |b| {
        without_compose(|| {
            b.iter(|| {
                let wkb =
                    wkb::WKB::new_from_names("", "", black_box(multi_layout), "", None).unwrap();
                black_box(wkb);
            });
        });
    });

    group.bench_function("wkb-noxkb-multilayout", |b| {
        without_compose(|| {
            b.iter(|| {
                let files = multi_layout
                    .split(',')
                    .map(|l| load_layout_file(l, None))
                    .collect();
                let wkb = wkb::WKB::new_from_layouts(files).unwrap();
                black_box(wkb);
            });
        });
    });

    let multi_keymap = without_compose(|| {
        wkb::WKB::new_from_names("", "", multi_layout, "", None)
            .unwrap()
            .as_xkb_string()
            .unwrap()
    });
    group.bench_function("wkb_multilayout_xkb_string", |b| {
        without_compose(|| {
            b.iter(|| {
                let wkb = wkb::WKB::new_from_string(black_box(&multi_keymap)).unwrap();
                black_box(wkb);
            });
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

criterion_group! {
    name = benches;
    config = cfg();
    targets = bench_setup_no_compose,
}
criterion_main!(benches);
