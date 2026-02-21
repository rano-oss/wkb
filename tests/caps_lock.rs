use test_case::test_matrix;
use wkb::{self, modifiers::CAPS_LOCK};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

mod common;
use common::{key_range, set_level, single_key, test_all_keys_locale, xkb_new_from_names};

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

        if wkb.level_keymap.len() <= level {
            continue;
        }

        // Skip levels that require modifiers we don't have
        match level {
            0 => {}
            1 => {
                if wkb.modifiers.level2_code().is_none() {
                    continue;
                }
            }
            2 => {
                if wkb.modifiers.level3_code().is_none() {
                    continue;
                }
            }
            3 => {
                if wkb.modifiers.level3_code().is_none() {
                    continue;
                }
            }
            4 => {
                if wkb.modifiers.level5_code().is_none() {
                    continue;
                }
            }
            5 => {
                if wkb.modifiers.level5_code().is_none() {
                    continue;
                }
            }
            6 => {
                if wkb.modifiers.level5_code().is_none() || wkb.modifiers.level3_code().is_none() {
                    continue;
                }
            }
            7 => {
                if wkb.modifiers.level5_code().is_none() || wkb.modifiers.level3_code().is_none() {
                    continue;
                }
            }
            _ => continue,
        }

        // Activate the appropriate modifier level
        match level {
            0 => {}
            1 => {
                let (code, lvl) = wkb.modifiers.level2_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            2 => {
                let (code, lvl) = wkb.modifiers.level3_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            3 => {
                let (code, lvl) = wkb.modifiers.level3_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
                let (code, lvl) = wkb.modifiers.level2_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            4 => {
                let (code, lvl) = wkb.modifiers.level5_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            5 => {
                let (code, lvl) = wkb.modifiers.level5_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
                let (code, lvl) = wkb.modifiers.level2_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            6 => {
                let (code, lvl) = wkb.modifiers.level5_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
                let (code, lvl) = wkb.modifiers.level3_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            7 => {
                let (code, lvl) = wkb.modifiers.level5_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
                let (code, lvl) = wkb.modifiers.level3_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
                let (code, lvl) = wkb.modifiers.level2_code().unwrap();
                set_level(&mut wkb, &mut xkb, code, lvl);
            }
            _ => {}
        }

        // Activate caps lock
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, wkb::KeyDirection::Down);

        let exceptions: &[(&str, &str, common::KeyRange)] = match level {
            1 => &[
                ("am", "eastern", single_key(2)),
                ("am", "eastern", key_range(5, 7)),
                ("am", "western", single_key(2)),
                ("am", "western", key_range(5, 7)),
                ("fr", "oss_latin9", single_key(55)),
                ("fr", "bepo_latin9", single_key(55)),
                ("fr", "mac", single_key(83)),
                ("be", "oss_latin9", single_key(55)),
                ("apl", "dyalog", key_range(71, 83)),
                ("apl", "dyalog_box", key_range(71, 83)),
            ],
            _ => &[],
        };

        test_all_keys_locale(wkb, xkb, layout, locale, exceptions);
    }
}
