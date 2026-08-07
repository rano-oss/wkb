//! Compare WKB named keys against xkbcommon for all keys and layouts,
//! accessing each level directly by index (no modifiers pressed).

use test_case::test_matrix;
use wkb::keysym_to_named_key;
use xkbcommon::xkb::Keycode;

include!("../test_data/layouts.rs");

mod common;
use common::xkb_new_keymap_from_names;

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], [0usize, 1, 2, 3, 4, 5, 6, 7])]
fn keysym(locale: &str, level: usize) {
    for layout in get_all_layouts_for_locale(locale) {
        let xkb = xkb_new_keymap_from_names(locale, &layout);
        let wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();

        for evdev in 0..701 {
            let wkb_key = wkb.level_named_key(evdev, 0, level);
            let xkb_sym = xkb
                .key_get_syms_by_level(Keycode::new(evdev + 8), 0, level as u32)
                .first()
                .map(|k| k.raw())
                .unwrap_or(0);

            if xkb_sym != 0 {
                let xkb_key = keysym_to_named_key(xkb_sym);
                if wkb_key != xkb_key {
                    panic!(
                        "locale={} layout={} evdev={} level={}: wkb={:?} xkb_sym={:#x} xkb_key={:?}",
                        locale, layout, evdev, level, wkb_key, xkb_sym, xkb_key
                    );
                }
            }
        }
    }
}
