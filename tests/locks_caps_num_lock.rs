#![cfg(feature = "compositor")]
//! Caps + num lock combined: hold both locks while reaching each level state
//! and compare all keys against xkbcommon.

use test_case::test_matrix;
use wkb::WKB;

include!("../test_data/layouts.rs");

mod common;
use common::{activate_locks, set_modifier_level, test_all_keys, xkb_new_from_names};

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], [0usize, 1, 2, 3, 4, 5, 6, 7])]
fn caps_num_lock(locale: &str, level: usize) {
    for layout in get_all_layouts_for_locale(locale) {
        let mut xkb = xkb_new_from_names(locale, &layout);
        let mut wkb = WKB::new_from_names("", "", locale, &layout, None).unwrap();
        set_modifier_level(&mut wkb, &mut xkb, level);
        activate_locks(&mut wkb, &mut xkb, 3);
        test_all_keys(wkb, xkb, layout, locale);
    }
}
