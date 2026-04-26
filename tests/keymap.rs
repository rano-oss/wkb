//! Keymap tests: XKB string export comparison and string-based construction.

use test_case::test_matrix;
use wkb::{testing::*, WKB};
use xkbcommon::xkb;

// ── Helpers ─────────────────────────────────────────────────────────────

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

                    if syms_wkb != syms_rmlvo {
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
    assert!(
        wkb.level3_code().is_some(),
        "Level3 (AltGr) should be detected"
    );
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
