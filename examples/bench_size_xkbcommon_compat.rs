//! Minimal binary using only the xkbcommon-compat backend.
//! Build with: cargo build --example bench_size_xkbcommon_compat --release
//! Measure with: size target/release/examples/bench_size_xkbcommon_compat

#[path = "../benches/common.rs"]
mod common;
use common::*;
use std::ffi::CString;
use std::hint::black_box;
use std::os::raw::c_char;
use std::ptr;

fn main() {
    let mut checksum: u64 = 0;

    let ctx = xkbcommon_compat::xkb_context_new(0);

    for &(locale, variant) in LAYOUTS {
        let c_rules = CString::new("evdev").unwrap();
        let c_layout = CString::new(locale).unwrap();
        let c_variant = variant.map(|v| CString::new(v).unwrap());

        let names = xkbcommon_compat::xkb_rule_names {
            rules: c_rules.as_ptr(),
            model: ptr::null(),
            layout: c_layout.as_ptr(),
            variant: c_variant.as_ref().map_or(ptr::null(), |v| v.as_ptr()),
            options: ptr::null(),
        };

        let km = unsafe { xkbcommon_compat::xkb_keymap_new_from_names(ctx, &names, 0) };
        let st = unsafe { xkbcommon_compat::xkb_state_new(km) };
        let mut buf = [0u8; 64];

        for case in KEY_CASES {
            for &(code, down) in case.keys {
                let xkb_kc = code + EVDEV_OFFSET;
                let dir = if down { 1u32 } else { 0u32 };
                unsafe { xkbcommon_compat::xkb_state_update_key(st, xkb_kc, dir) };
                if down {
                    let n = unsafe {
                        xkbcommon_compat::xkb_state_key_get_utf8(
                            st,
                            xkb_kc,
                            buf.as_mut_ptr() as *mut c_char,
                            buf.len(),
                        )
                    };
                    checksum = checksum.wrapping_add(n as u64);
                }
            }
        }

        unsafe {
            xkbcommon_compat::xkb_state_unref(st);
            xkbcommon_compat::xkb_keymap_unref(km);
        }
    }

    unsafe { xkbcommon_compat::xkb_context_unref(ctx) };

    black_box(checksum);
}
