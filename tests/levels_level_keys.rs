//! Compare WKB character output against xkbcommon for all keys and layouts,
//! accessing each level directly by index (no modifiers pressed).

use test_case::test_matrix;
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
fn level_keys(locale: &str, level: usize) {
    for layout in get_all_layouts_for_locale(locale) {
        let xkb = xkb_new_keymap_from_names(locale, &layout);
        let wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        for i in 0..701 {
            let k1 = wkb.level_char(i as u32, 0, level);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, level as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            if k1 != k2 && k2.is_some() && k1.is_some() {
                println!("wkb: {:?}, xkb: {:?} {}", k1, k2, i);
            }
            assert!(k1 == k2 || k2.is_none() || k1.is_none());
        }
    }
}
