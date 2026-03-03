use test_case::test_matrix;
use wkb::{self, modifiers::CAPS_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{set_modifier_level, test_all_keys_locale, xkb_new_from_names};

#[test_matrix([
    "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk",
    "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
    "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn"
], 0..8)]
fn caps_lock(locale: &str, level: usize) {
    let wkb_init = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb_init.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));

        // Skip layouts where CAPS_LOCK has been remapped (not a true caps lock).
        // Pressing a remapped CAPS_LOCK in xkbcommon has side effects that
        // diverge from WKB, making the comparison meaningless.
        if !wkb.modifiers.caps_active()
            && !matches!(
                wkb.modifiers.0.get(&CAPS_LOCK),
                Some(wkb::modifiers::Modifier::Single(
                    wkb::modifiers::ModKind::Lock {
                        mod_type: wkb::modifiers::ModType::None,
                        ..
                    }
                ))
            )
        {
            continue;
        }

        if wkb.state_keymap.len() <= level {
            continue;
        }

        if !set_modifier_level(&mut wkb, &mut xkb, level) {
            continue;
        }

        // Activate caps lock
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, wkb::KeyDirection::Down);

        test_all_keys_locale(wkb, xkb, layout, locale);
    }
}
