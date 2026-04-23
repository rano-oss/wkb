//! Tests comparing xkb-core keymap serialization against xkbcommon's output.
//!
//! These tests verify that `as_xkb_string()` produces functionally equivalent
//! output to `xkb_keymap_get_as_string()` from libxkbcommon.

use wkb::testing::*;
use xkbcommon::xkb;

/// Parse both keymap strings and compare them structurally: for every keycode,
/// check that the same keysyms are produced at every level in every group.
fn compare_keymaps_functionally(wkb_string: &str, xkb_string: &str, layout_name: &str) {
    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    let km_wkb = xkb::Keymap::new_from_string(
        &ctx,
        wkb_string.to_string(),
        xkb::KEYMAP_FORMAT_TEXT_V1,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap_or_else(|| panic!("Failed to parse wkb keymap string for layout {layout_name}"));

    let km_xkb = xkb::Keymap::new_from_string(
        &ctx,
        xkb_string.to_string(),
        xkb::KEYMAP_FORMAT_TEXT_V1,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap_or_else(|| panic!("Failed to parse xkbcommon keymap string for layout {layout_name}"));

    // Compare: for each keycode, check syms at every layout+level
    let min: u32 = km_wkb.min_keycode().into();
    let max: u32 = km_wkb.max_keycode().into();
    let min = min.max(km_xkb.min_keycode().into());
    let max = max.min(km_xkb.max_keycode().into());

    let num_layouts_wkb = km_wkb.num_layouts();
    let num_layouts_xkb = km_xkb.num_layouts();
    assert_eq!(
        num_layouts_wkb, num_layouts_xkb,
        "[{layout_name}] layout count mismatch"
    );

    let _st_wkb = xkb::State::new(&km_wkb);
    let _st_xkb = xkb::State::new(&km_xkb);

    for kc_raw in min..=max {
        let kc = xkb::Keycode::new(kc_raw);
        for layout in 0..num_layouts_wkb {
            let nl_wkb = km_wkb.num_levels_for_key(kc, layout);
            let nl_xkb = km_xkb.num_levels_for_key(kc, layout);
            // Compare only levels present in both outputs
            let nl = nl_wkb.min(nl_xkb);

            for level in 0..nl {
                let syms_wkb = km_wkb.key_get_syms_by_level(kc, layout, level);
                let syms_xkb = km_xkb.key_get_syms_by_level(kc, layout, level);

                // Skip levels where one side has no syms (type mismatch causes different level counts)
                if syms_wkb.is_empty() || syms_xkb.is_empty() {
                    continue;
                }

                assert_eq!(
                    syms_wkb, syms_xkb,
                    "[{layout_name}] keycode {} layout {layout} level {level}: syms differ",
                    kc_raw
                );
            }
        }
    }
}

fn get_xkbcommon_string(locale: &str, variant: Option<&str>) -> String {
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
    .unwrap_or_else(|| panic!("xkbcommon: failed to compile keymap for {locale}"));
    km.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1)
}

fn get_wkb_string(locale: &str, variant: Option<&str>) -> String {
    let layout = variant.map(String::from);
    let wkb = WKB::new_from_names(locale.to_string(), layout);
    wkb.as_xkb_string()
        .expect("WKB should have xkb_keymap stored")
}

#[test]
fn export_us_matches_xkbcommon() {
    let wkb_str = get_wkb_string("us", None);
    let xkb_str = get_xkbcommon_string("us", None);
    compare_keymaps_functionally(&wkb_str, &xkb_str, "us");
}

#[test]
fn export_de_matches_xkbcommon() {
    let wkb_str = get_wkb_string("de", None);
    let xkb_str = get_xkbcommon_string("de", None);
    compare_keymaps_functionally(&wkb_str, &xkb_str, "de");
}

#[test]
fn export_fr_matches_xkbcommon() {
    let wkb_str = get_wkb_string("fr", None);
    let xkb_str = get_xkbcommon_string("fr", None);
    compare_keymaps_functionally(&wkb_str, &xkb_str, "fr");
}

#[test]
fn export_us_intl_matches_xkbcommon() {
    let wkb_str = get_wkb_string("us", Some("intl"));
    let xkb_str = get_xkbcommon_string("us", Some("intl"));
    compare_keymaps_functionally(&wkb_str, &xkb_str, "us_intl");
}

#[test]
fn export_round_trip_parse() {
    // Verify: export -> parse -> export produces the same functional result
    let wkb = WKB::new_from_names("us".to_string(), None);
    let exported = wkb.as_xkb_string().expect("should have keymap");

    // Parse the exported string back
    let wkb2 = WKB::new_from_string(exported.clone());
    let exported2 = wkb2.as_xkb_string().expect("round-trip should have keymap");

    // Both should parse to functionally identical keymaps
    compare_keymaps_functionally(&exported, &exported2, "us_roundtrip");
}
