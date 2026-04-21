//! Minimal binary using only the xkbcommon-dl (dlopen) backend.
//! Build with: cargo build --example bench_size_xkbcommon_dl --release
//! Measure with: size target/release/examples/bench_size_xkbcommon_dl

#[path = "../benches/common.rs"]
mod common;
use common::*;
use std::ffi::CString;
use std::hint::black_box;
use std::os::raw::c_char;
use std::ptr;

fn main() {
    let mut checksum: u64 = 0;

    let xkb = xkbcommon_dl::xkbcommon_handle();
    let ctx =
        unsafe { (xkb.xkb_context_new)(xkbcommon_dl::xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };

    for &(locale, variant) in LAYOUTS {
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
        let mut buf = [0u8; 64];

        for case in KEY_CASES {
            for &(code, down) in case.keys {
                let xkb_kc = code + EVDEV_OFFSET;
                let dir = if down {
                    xkbcommon_dl::xkb_key_direction::XKB_KEY_DOWN
                } else {
                    xkbcommon_dl::xkb_key_direction::XKB_KEY_UP
                };
                unsafe { (xkb.xkb_state_update_key)(st, xkb_kc, dir) };
                if down {
                    let n = unsafe {
                        (xkb.xkb_state_key_get_utf8)(
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
            (xkb.xkb_state_unref)(st);
            (xkb.xkb_keymap_unref)(km);
        }
    }

    // Compose
    let xkb_compose = xkbcommon_dl::xkbcommon_compose_handle();
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
        for seq in COMPOSE_SEQUENCES {
            for &ks in seq.keysyms {
                let _ = unsafe { (xkb_compose.xkb_compose_state_feed)(state, ks) };
                let status = unsafe { (xkb_compose.xkb_compose_state_get_status)(state) };
                checksum = checksum.wrapping_add(status as u64);
            }
            unsafe { (xkb_compose.xkb_compose_state_reset)(state) };
        }
        unsafe {
            (xkb_compose.xkb_compose_state_unref)(state);
            (xkb_compose.xkb_compose_table_unref)(table);
        }
    }

    unsafe { (xkb.xkb_context_unref)(ctx) };

    black_box(checksum);
}
