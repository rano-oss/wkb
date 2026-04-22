use test_case::test_matrix;
use wkb::{
    modifiers::{level_index, ALTGR, CAPS_LOCK},
    xkb as wkbxkb, KeyDirection, WKB,
};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let variant_str = layout.unwrap_or_default();
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

fn test_all_keys_locale<C: wkb::composer::Composer>(
    wkb: WKB<C>,
    xkb: xkb::State,
    layout: String,
    locale: &str,
) {
    let mut wkb = wkb;
    for i in 0..701 {
        let k1 = wkb.utf8(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i + 8));

        if k1 != k2.chars().last() && !k2.is_empty() {
            let level = level_index(
                wkb.modifiers.level5(),
                wkb.modifiers.level3(),
                wkb.modifiers.level2(),
            );
            println!(
                "locale={} layout={} key={} level={}",
                locale, layout, i, level
            );
            println!("  wkb={:?} xkb={:?}", k1, k2.chars().last());
        }
        assert!(k1 == k2.chars().last() || k2.chars().last().is_none());
    }
}

fn set_level<C: wkb::composer::Composer>(
    wkb: &mut WKB<C>,
    xkb: &mut xkb::State,
    code: u32,
    level: Option<u8>,
) {
    if let Some(level) = level {
        let mut modifiers = Vec::new();
        match level {
            7 => {
                modifiers.push(wkbxkb::level5_code(&wkb.modifiers).unwrap().0);
                modifiers.push(wkbxkb::level3_code(&wkb.modifiers).unwrap().0);
                modifiers.push(wkbxkb::level2_code(&wkb.modifiers).unwrap().0);
            }
            6 => {
                modifiers.push(wkbxkb::level5_code(&wkb.modifiers).unwrap().0);
                modifiers.push(wkbxkb::level3_code(&wkb.modifiers).unwrap().0);
            }
            5 => {
                modifiers.push(wkbxkb::level5_code(&wkb.modifiers).unwrap().0);
                modifiers.push(wkbxkb::level2_code(&wkb.modifiers).unwrap().0);
            }
            4 => {
                modifiers.push(wkbxkb::level5_code(&wkb.modifiers).unwrap().0);
            }
            3 => {
                modifiers.push(wkbxkb::level2_code(&wkb.modifiers).unwrap().0);
                modifiers.push(wkbxkb::level3_code(&wkb.modifiers).unwrap_or((ALTGR, None)).0);
            }
            2 => {
                modifiers.push(wkbxkb::level3_code(&wkb.modifiers).unwrap().0);
            }
            1 => {
                modifiers.push(wkbxkb::level2_code(&wkb.modifiers).unwrap().0);
            }
            _ => {}
        }
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
        }
        wkb.update_key(code, KeyDirection::Down);
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
        for &mod_code in &modifiers {
            wkb.update_key(mod_code, KeyDirection::Down);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Down);
            wkb.update_key(mod_code, KeyDirection::Up);
            xkb.update_key(Keycode::new(mod_code + 8), xkb::KeyDirection::Up);
        }
    } else {
        xkb.update_key(Keycode::new(code + 8), xkb::KeyDirection::Down);
        wkb.update_key(code, KeyDirection::Down);
    }
}

fn set_modifier_level<C: wkb::composer::Composer>(
    wkb: &mut WKB<C>,
    xkb: &mut xkb::State,
    level: usize,
) -> bool {
    match level {
        0 => true,
        1 => {
            if let Some((code, lvl)) = wkbxkb::level2_code(&wkb.modifiers) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        2 => {
            if let Some((code, lvl)) = wkbxkb::level3_code(&wkb.modifiers) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        3 => {
            if let (Some((c3, l3)), Some((c2, l2))) = (
                wkbxkb::level3_code(&wkb.modifiers),
                wkbxkb::level2_code(&wkb.modifiers),
            ) {
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        4 => {
            if let Some((code, lvl)) = wkbxkb::level5_code(&wkb.modifiers) {
                set_level(wkb, xkb, code, lvl);
                true
            } else {
                false
            }
        }
        5 => {
            if let (Some((c5, l5)), Some((c2, l2))) = (
                wkbxkb::level5_code(&wkb.modifiers),
                wkbxkb::level2_code(&wkb.modifiers),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        6 => {
            if let (Some((c5, l5)), Some((c3, l3))) = (
                wkbxkb::level5_code(&wkb.modifiers),
                wkbxkb::level3_code(&wkb.modifiers),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                true
            } else {
                false
            }
        }
        7 => {
            if let (Some((c5, l5)), Some((c3, l3)), Some((c2, l2))) = (
                wkbxkb::level5_code(&wkb.modifiers),
                wkbxkb::level3_code(&wkb.modifiers),
                wkbxkb::level2_code(&wkb.modifiers),
            ) {
                set_level(wkb, xkb, c5, l5);
                set_level(wkb, xkb, c3, l3);
                set_level(wkb, xkb, c2, l2);
                true
            } else {
                false
            }
        }
        _ => false,
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
], 0..8)]
fn caps_lock(locale: &str, level: usize) {
    let wkb_init = wkb::WKB::new_from_names(locale.to_string(), None);
    for layout in wkb_init.layouts() {
        let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
        let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));

        set_modifier_level(&mut wkb, &mut xkb, level);

        // Activate caps lock
        xkb.update_key(Keycode::new(CAPS_LOCK + 8), xkb::KeyDirection::Down);
        wkb.update_key(CAPS_LOCK, wkb::KeyDirection::Down);

        test_all_keys_locale(wkb, xkb, layout, locale);
    }
}
