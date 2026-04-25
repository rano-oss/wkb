use std::time::Instant;

fn main() {
    use xkb_core::rust_types::{Context, RuleNames};

    let t0 = Instant::now();

    let t_ctx = Instant::now();
    let ctx = Context::new().expect("ctx");
    eprintln!("1. context_new:        {:?}", t_ctx.elapsed());

    let t_km = Instant::now();
    let rules = RuleNames::evdev("us".to_string(), None);
    let keymap = ctx.keymap_from_names(&rules).expect("keymap");
    eprintln!("2. keymap_from_names:  {:?}", t_km.elapsed());

    let t_st = Instant::now();
    let _state = keymap.new_state().expect("state");
    eprintln!("3. single new_state:   {:?}", t_st.elapsed());

    let t_st24 = Instant::now();
    for _ in 0..24 {
        let _s = keymap.new_state();
    }
    eprintln!("4. 24x new_state:      {:?}", t_st24.elapsed());

    let min_kc = keymap.min_keycode().max(8);
    let max_kc = keymap.max_keycode();
    let num_keys = (max_kc - min_kc + 1) as usize;

    let t_iter = Instant::now();
    for _ in 0..8 {
        if let Some(st) = keymap.new_state() {
            for kc in min_kc..=max_kc {
                let _ = st.key_get_utf8(kc);
            }
        }
    }
    eprintln!(
        "5. 8x(new_state+{}x get_utf8): {:?}",
        num_keys,
        t_iter.elapsed()
    );

    // Simulate populate_lock (caps): 8 levels x (new_state + toggle + iterate)
    let t_lock = Instant::now();
    for _ in 0..8 {
        if let Some(mut st) = keymap.new_state() {
            st.update_key(66, xkb_core::XKB_KEY_DOWN); // caps lock X11 keycode
            st.update_key(66, xkb_core::XKB_KEY_UP);
            for kc in min_kc..=max_kc {
                let _ = st.key_get_utf8(kc);
            }
        }
    }
    eprintln!("6. populate_lock(caps): {:?}", t_lock.elapsed());

    // level_exceptions: 8 levels x keycodes x key_get_syms_by_level
    let t_exc = Instant::now();
    for lvl in 0..8u32 {
        for kc in min_kc..=max_kc {
            let _ = keymap.key_get_syms_by_level(kc, 0, lvl);
        }
    }
    eprintln!("7. level_exceptions:   {:?}", t_exc.elapsed());

    let t_compose = Instant::now();
    let resolved = xkb_core::compose::resolve_compose_file("en_US.UTF-8");
    if let Some(subpath) = resolved {
        let path = std::path::Path::new("/usr/share/X11/locale").join(&subpath);
        let _ = xkb_core::compose::parse_compose_file(&path);
    }
    eprintln!("8. compose_table:      {:?}", t_compose.elapsed());

    // Full WKB setup for comparison
    let t_wkb = Instant::now();
    let _wkb: wkb::WKB = wkb::WKB::new_from_names("", "", "us", "", None).unwrap();
    eprintln!("\nFull WKB new_from_names: {:?}", t_wkb.elapsed());

    // With explicit layout (skip get_all_layouts)
    let t_wkb2 = Instant::now();
    let _wkb2: wkb::WKB = wkb::WKB::new_from_names("", "", "us", "", None).unwrap();
    eprintln!("WKB with explicit layout: {:?}", t_wkb2.elapsed());

    eprintln!("Total profiling time:   {:?}", t0.elapsed());
}
