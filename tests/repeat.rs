use test_case::test_matrix;
use xkbcommon::xkb::Keycode;

include!("common/layouts.rs");

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
])]
fn repeat_keys(locale: &str) {
    for layout in get_all_layouts_for_locale(locale) {
        let xkb = xkb_new_keymap_from_names(locale, &layout);
        let wkb = wkb::WKB::new_from_names("", "", locale, &layout, None).unwrap();
        for i in 0..701 {
            assert!(xkb.key_repeats(Keycode::new(i + 8)) == wkb.key_repeats(i));
        }
    }
}
