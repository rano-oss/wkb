use test_case::test_matrix;
use xkbcommon::xkb::{self, Keycode};

fn xkb_new_keymap_from_names(locale: &str, layout: &str) -> xkb::Keymap {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_names(
        &context,
        "evdev",
        "pc105",
        locale,
        layout,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap()
}

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], 0..8)]
fn keysym(locale: &str, level: usize) {
    for layout in wkb::testing::get_all_layouts_for_locale(locale) {
        let xkb = xkb_new_keymap_from_names(locale, &layout);
        let wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();

        for evdev in 0..701 {
            let wkb_sym = wkb.level_keysym(evdev, 0, level);
            let xkb_sym = xkb
                .key_get_syms_by_level(Keycode::new(evdev + 8), 0, level as u32)
                .first()
                .map(|k| k.raw())
                .unwrap_or(0);

            if wkb_sym != xkb_sym && xkb_sym != 0 {
                let wkb_name = wkb::keysyms::keysym_get_name(wkb_sym).unwrap_or("?");
                let xkb_name = wkb::keysyms::keysym_get_name(xkb_sym).unwrap_or("?");
                panic!(
                    "locale={} layout={} evdev={} level={}: wkb={:#x}({}) xkb={:#x}({})",
                    locale, layout, evdev, level, wkb_sym, wkb_name, xkb_sym, xkb_name
                );
            }
            assert!(wkb_sym == xkb_sym || xkb_sym == 0);
        }
    }
}
