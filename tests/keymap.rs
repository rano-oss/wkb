//! Keymap tests: XKB string export comparison and string-based construction.

use test_case::test_matrix;
use wkb::{testing::*, WKB};
use xkbcommon::xkb;

// ── Helpers ─────────────────────────────────────────────────────────────

/// Normalize a keysym for comparison: convert to the character it produces.
/// This makes legacy/Unicode keysym encodings and dead-vs-regular variants equal
/// when they produce the same character (e.g. dead_greek and Greek_alpha both → α).
fn normalize_keysym(ks: u32) -> u32 {
    // First try xkbcommon's utf32 (handles legacy keysyms, Unicode keysyms, KP keys)
    let utf32 = unsafe { xkbcommon::xkb::ffi::xkb_keysym_to_utf32(ks) };
    if utf32 != 0 {
        return utf32;
    }
    // Fall back to our own table (handles dead keysyms → char)
    if let Some(ch) = xkb_core::keysym_utf::keysym_to_char(ks) {
        return ch as u32;
    }
    ks
}

/// Parse both keymap strings and compare them structurally: for every keycode,
/// check that the same keysyms are produced at every level in every group.
/// Keysyms are normalized to their character before comparison, since the same
/// character can be represented by different keysym encodings (legacy vs Unicode,
/// dead vs regular).
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

    for kc_raw in min..=max {
        let kc = xkb::Keycode::new(kc_raw);
        for layout in 0..num_layouts_wkb {
            let nl = km_wkb
                .num_levels_for_key(kc, layout)
                .min(km_xkb.num_levels_for_key(kc, layout));

            for level in 0..nl {
                let syms_wkb = km_wkb.key_get_syms_by_level(kc, layout, level);
                let syms_xkb = km_xkb.key_get_syms_by_level(kc, layout, level);

                if syms_wkb.is_empty() || syms_xkb.is_empty() {
                    continue;
                }

                let norm_wkb: Vec<u32> =
                    syms_wkb.iter().map(|s| normalize_keysym(s.raw())).collect();
                let norm_xkb: Vec<u32> =
                    syms_xkb.iter().map(|s| normalize_keysym(s.raw())).collect();

                assert_eq!(
                    norm_wkb, norm_xkb,
                    "[{layout_name}] keycode {} layout {layout} level {level}: syms differ\n  wkb: {:?}\n  xkb: {:?}",
                    kc_raw, syms_wkb, syms_xkb,
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
    let wkb = WKB::new_from_names("", "", locale, layout.as_deref().unwrap_or(""), None).unwrap();
    wkb.as_xkb_string()
        .expect("WKB should have xkb_keymap stored")
        .to_string()
}

// ── Export tests (test_matrix over all locales) ─────────────────────────

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn export_matches_xkbcommon(locale: &str) {
    let wkb_str = get_wkb_string(locale, None);
    let xkb_str = get_xkbcommon_string(locale, None);
    compare_keymaps_functionally(&wkb_str, &xkb_str, locale);
}

#[test]
fn export_us_intl_matches_xkbcommon() {
    let wkb_str = get_wkb_string("us", Some("intl"));
    let xkb_str = get_xkbcommon_string("us", Some("intl"));
    compare_keymaps_functionally(&wkb_str, &xkb_str, "us_intl");
}

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn export_round_trip(locale: &str) {
    let wkb = WKB::new_from_names("", "", locale, "", None).unwrap();
    let exported = wkb.as_xkb_string().expect("should have keymap");

    let wkb2 = WKB::new_from_string(&exported).unwrap();
    let exported2 = wkb2.as_xkb_string().expect("round-trip should have keymap");

    compare_keymaps_functionally(&exported, &exported2, &format!("{locale}_roundtrip"));
}

// ── All-variants export test ────────────────────────────────────────────

/// For every locale, iterate ALL layout variants (including base), create the
/// wkb keymap, export to XKB string, feed that string to xkbcommon, and compare
/// against an xkbcommon instance built directly from RMLVO names.
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn export_all_variants_match_xkbcommon(locale: &str) {
    let variants = wkb::testing::get_all_layouts_for_locale(locale);

    let mut failures: Vec<String> = Vec::new();

    for variant in &variants {
        let label = if variant.is_empty() {
            locale.to_string()
        } else {
            format!("{locale}({variant})")
        };

        // Build wkb, export to XKB string
        let wkb = match WKB::new_from_names("", "", locale, variant, None) {
            Ok(w) => w,
            Err(e) => {
                failures.push(format!("{label}: wkb construction failed: {e}"));
                continue;
            }
        };
        let wkb_str = match wkb.as_xkb_string() {
            Some(s) => s.to_string(),
            None => {
                failures.push(format!("{label}: as_xkb_string() returned None"));
                continue;
            }
        };

        // Parse wkb's exported string via xkbcommon
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let km_from_wkb = match xkb::Keymap::new_from_string(
            &ctx,
            wkb_str,
            xkb::KEYMAP_FORMAT_TEXT_V1,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        ) {
            Some(km) => km,
            None => {
                failures.push(format!("{label}: xkbcommon rejected wkb's XKB string"));
                continue;
            }
        };

        // Build xkbcommon directly from RMLVO
        let km_from_rmlvo = match xkb::Keymap::new_from_names(
            &ctx,
            "evdev",
            "",
            locale,
            variant,
            None,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        ) {
            Some(km) => km,
            None => {
                failures.push(format!("{label}: xkbcommon failed to compile from names"));
                continue;
            }
        };

        // Compare keysyms at every keycode × layout × level
        let min: u32 = km_from_wkb
            .min_keycode()
            .raw()
            .max(km_from_rmlvo.min_keycode().raw());
        let max: u32 = km_from_wkb
            .max_keycode()
            .raw()
            .min(km_from_rmlvo.max_keycode().raw());

        let num_layouts_wkb = km_from_wkb.num_layouts();
        let num_layouts_rmlvo = km_from_rmlvo.num_layouts();
        if num_layouts_wkb != num_layouts_rmlvo {
            failures.push(format!(
                "{label}: layout count mismatch: wkb={num_layouts_wkb} xkb={num_layouts_rmlvo}"
            ));
            continue;
        }

        let mut variant_ok = true;
        for kc_raw in min..=max {
            let kc = xkb::Keycode::new(kc_raw);
            for layout in 0..num_layouts_wkb {
                let nl = km_from_wkb
                    .num_levels_for_key(kc, layout)
                    .min(km_from_rmlvo.num_levels_for_key(kc, layout));

                for level in 0..nl {
                    let syms_wkb = km_from_wkb.key_get_syms_by_level(kc, layout, level);
                    let syms_rmlvo = km_from_rmlvo.key_get_syms_by_level(kc, layout, level);

                    if syms_wkb.is_empty() || syms_rmlvo.is_empty() {
                        continue;
                    }

                    let norm_wkb: Vec<u32> =
                        syms_wkb.iter().map(|s| normalize_keysym(s.raw())).collect();
                    let norm_rmlvo: Vec<u32> = syms_rmlvo
                        .iter()
                        .map(|s| normalize_keysym(s.raw()))
                        .collect();

                    if norm_wkb != norm_rmlvo {
                        failures.push(format!(
                            "{label}: keycode {kc_raw} layout {layout} level {level}: \
                             wkb={syms_wkb:?} xkb={syms_rmlvo:?}"
                        ));
                        variant_ok = false;
                        break; // one mismatch per variant is enough detail
                    }
                }
                if !variant_ok {
                    break;
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "Failures for locale {locale} ({} variants tested, {} failed):\n{}",
        variants.len(),
        failures.len(),
        failures.join("\n")
    );
}

// ── String construction tests ───────────────────────────────────────────

/// Get an XKB keymap string using wkb's own serializer (no external tools needed).
fn keymap_string_from_export(locale: &str, variant: Option<&str>) -> String {
    let wkb = WKB::new_from_names("", "", locale, variant.unwrap_or(""), None).unwrap();
    wkb.as_xkb_string()
        .expect("WKB should have xkb_keymap stored")
        .to_string()
}

#[test]
fn string_us() {
    let keymap_str = keymap_string_from_export("us", None);
    let mut wkb_from_string = WKB::new_from_string(&keymap_str).unwrap();
    let mut wkb_from_names = WKB::new_from_names("", "", "us", "", None).unwrap();

    for keycode in [38, 44, 2, 3, 4] {
        assert_eq!(
            wkb_from_string.key_char(keycode),
            wkb_from_names.key_char(keycode),
            "Code {keycode} mismatch"
        );
    }

    if let Some((shift_code, _)) = wkb_from_string.level2_code() {
        wkb_from_string.update_key(shift_code, KeyDirection::Down);
        wkb_from_names.update_key(shift_code, KeyDirection::Down);

        for keycode in [38, 44, 2] {
            assert_eq!(
                wkb_from_string.key_char(keycode),
                wkb_from_names.key_char(keycode),
                "Code {keycode} with Shift mismatch"
            );
        }
    }
}

#[test]
fn string_de() {
    let keymap_str = keymap_string_from_export("de", None);
    let wkb_from_string = WKB::new_from_string(&keymap_str).unwrap();
    let wkb_from_names = WKB::new_from_names("", "", "de", "", None).unwrap();

    for keycode in [44, 52] {
        assert_eq!(
            wkb_from_string.key_char(keycode),
            wkb_from_names.key_char(keycode),
            "DE layout code {keycode} mismatch"
        );
    }
}

#[test]
fn string_dvorak() {
    let keymap_str = keymap_string_from_export("us", Some("dvorak"));
    let wkb_from_string = WKB::new_from_string(&keymap_str).unwrap();
    let wkb_from_names = WKB::new_from_names("", "", "us", "dvorak", None).unwrap();

    for keycode in [38, 39, 40, 44] {
        assert_eq!(
            wkb_from_string.key_char(keycode),
            wkb_from_names.key_char(keycode),
            "Dvorak code {keycode} mismatch"
        );
    }
}

#[test]
fn string_modifiers() {
    let keymap_str = keymap_string_from_export("us", None);
    let wkb = WKB::new_from_string(&keymap_str).unwrap();

    assert!(
        wkb.level2_code().is_some(),
        "Level2 (Shift) should be detected"
    );
    // US layout has no AltGr (Level3) — RALT produces Alt_R, not ISO_Level3_Shift.
    // Layouts like dk, de, fr have Level3 on physical RALT via level3(ralt_switch).
    assert!(
        wkb.level3_code().is_none(),
        "US layout should NOT have Level3 (no AltGr)"
    );
}

#[test]
fn string_modifiers_dk() {
    let keymap_str = keymap_string_from_export("dk", None);
    let wkb = WKB::new_from_string(&keymap_str).unwrap();

    assert!(
        wkb.level2_code().is_some(),
        "Level2 (Shift) should be detected"
    );
    // Danish layout has AltGr on physical RALT (evdev 100) via level3(ralt_switch).
    let l3 = wkb.level3_code();
    assert!(l3.is_some(), "dk layout should have Level3 (AltGr)");
    assert_eq!(l3.unwrap().0, 100, "Level3 should be on evdev 100 (RALT)");
}

#[test]
fn string_caps_lock() {
    let keymap_str = keymap_string_from_export("us", None);
    let mut wkb = WKB::new_from_string(&keymap_str).unwrap();

    assert_eq!(wkb.key_char(38), Some('l'));

    wkb.update_key(58, KeyDirection::Down);
    wkb.update_key(58, KeyDirection::Up);

    assert_eq!(
        wkb.key_char(38),
        Some('L'),
        "Expected uppercase with Caps Lock"
    );
}
