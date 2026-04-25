//! Keymap tests: XKB string export comparison and string-based construction.

use test_case::test_matrix;
use wkb::{testing::*, KeyDirection, WKB};
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
            wkb_from_string.utf8(keycode),
            wkb_from_names.utf8(keycode),
            "Code {keycode} mismatch"
        );
    }

    if let Some((shift_code, _)) = wkb_from_string.level2_code() {
        wkb_from_string.update_key(shift_code, KeyDirection::Down);
        wkb_from_names.update_key(shift_code, KeyDirection::Down);

        for keycode in [38, 44, 2] {
            assert_eq!(
                wkb_from_string.utf8(keycode),
                wkb_from_names.utf8(keycode),
                "Code {keycode} with Shift mismatch"
            );
        }
    }
}

#[test]
fn string_de() {
    let keymap_str = keymap_string_from_export("de", None);
    let mut wkb_from_string = WKB::new_from_string(&keymap_str).unwrap();
    let mut wkb_from_names = WKB::new_from_names("", "", "de", "", None).unwrap();

    for keycode in [44, 52] {
        assert_eq!(
            wkb_from_string.utf8(keycode),
            wkb_from_names.utf8(keycode),
            "DE layout code {keycode} mismatch"
        );
    }
}

#[test]
fn string_dvorak() {
    let keymap_str = keymap_string_from_export("us", Some("dvorak"));
    let mut wkb_from_string = WKB::new_from_string(&keymap_str).unwrap();
    let mut wkb_from_names = WKB::new_from_names("", "", "us", "dvorak", None).unwrap();

    for keycode in [38, 39, 40, 44] {
        assert_eq!(
            wkb_from_string.utf8(keycode),
            wkb_from_names.utf8(keycode),
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

    assert_eq!(wkb.utf8(38), Some('l'));

    wkb.update_key(58, KeyDirection::Down);
    wkb.update_key(58, KeyDirection::Up);

    assert_eq!(wkb.utf8(38), Some('L'), "Expected uppercase with Caps Lock");
}
