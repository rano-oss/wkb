use test_case::test_matrix;
use wkb;
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::Keymap {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        &locale,
        &layout.unwrap_or("".to_string()),
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
fn level_keys(locale: &str, level: usize) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout));
        for i in 0..701 {
            let k1 = wkb.level_key(i as u32, level);
            let mut k2 = xkb
                .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, level as u32)
                .first()
                .map(|k| k.key_char().unwrap_or_default());
            if k2.unwrap_or_default() == '\0' {
                k2 = None;
            }
            assert!(k1 == k2 || k2 == None);
        }
    }
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
fn repeat_keys(locale: &str) {
    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb.layouts() {
        let xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout));
        for i in 0..701 {
            assert!(xkb.key_repeats(Keycode::new(i as u32 + 8)) == wkb.key_repeats(i));
        }
    }
}
