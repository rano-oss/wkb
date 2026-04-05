//! Test layout enumeration
//!
//! This test suite verifies that the layouts() function returns a valid list
//! of layout variants for each locale, and that each variant can be loaded.

use test_case::test_matrix;
use wkbxkb as wkb;
use xkbcommon::xkb;

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let variant_str = layout.unwrap_or_else(|| String::new());
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        &locale,
        &variant_str,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    xkb::State::new(&keymap)
}

/// Test that layouts() returns a valid list of variants that can be loaded
/// Note: We can't directly compare with xkbcommon because it doesn't expose
/// a variant enumeration API. The KeymapLayouts iterator returns layout names
/// (e.g., "English (US)"), not variant names (e.g., "dvorak", "colemak").
#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
])]
fn layouts_enumeration(locale: &str) {
    // Get wkb layouts (these are variant names like "dvorak", "colemak", etc.)
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    let layouts = wkb.layouts();

    // Verify we have at least one layout variant
    assert!(
        !layouts.is_empty(),
        "Locale {} should have at least one layout variant",
        locale
    );

    // The first variant is typically "" (empty string for base layout)
    // This is just a sanity check that we got a list of strings
    assert!(
        layouts.iter().all(|l| l == "" || !l.is_empty()),
        "All layouts should be empty string or valid variant names for locale {}",
        locale
    );

    // Verify each returned variant can be loaded successfully with xkbcommon
    // This ensures wkbxkb is returning valid variant names
    for variant in &layouts {
        // Try to create an xkb keymap with this locale and variant
        let xkb_state = xkb_new_from_names(locale.to_string(), Some(variant.clone()));

        // If we got here without panicking, the variant is valid
        // Verify we can create a wkb instance too
        let wkb_variant = wkb::WKB::new_from_names(locale.to_string(), Some(variant.clone()));

        // Verify the loaded layout matches what we requested
        assert_eq!(
            wkb_variant.current_layout(),
            *variant,
            "Loaded layout should match requested layout for locale={} layout={}",
            locale,
            variant
        );

        // Drop xkb_state to ensure we're testing clean creation each time
        drop(xkb_state);
    }
}
