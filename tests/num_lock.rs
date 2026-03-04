use test_case::test_matrix;
use wkb::{self, modifiers::NUM_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{set_modifier_level, test_all_keys, xkb_new_from_names};

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], 0..8)]
fn num_lock(locale: &str, level: usize) {
    let wkb_init = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb_init.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));

        if wkb.state_keymap.len() <= level {
            continue;
        }

        if !set_modifier_level(&mut wkb, &mut xkb, level) {
            continue;
        }

        // Activate num lock
        xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);

        test_all_keys(wkb, xkb, layout);
    }
}
