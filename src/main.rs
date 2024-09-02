use wkb::{self, WKB};
use xkbcommon::{
    self,
    xkb::{self, Keycode},
};

fn xkb_new_from_names(locale: String, layout: Option<String>) -> xkb::State {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        &locale,
        &layout.unwrap_or("".to_string()),
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .unwrap();
    xkb::State::new(&keymap)
}

fn test_all_keys(wkb: WKB, xkb: xkb::State) {
    for i in 0..701 {
        let k1 = if let Some(utf8) = wkb.utf8(i) {
            utf8.to_string()
        } else {
            String::new()
        };
        let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
        if k1 != k2 && (k2.len() != 0 && k1.len() == 0) {
            println!(
                "{}: wkb: {}, xkb: {:x}",
                i,
                k1,
                k2.chars().last().unwrap_or_default() as u32
            );
        }
    }
}

fn main() {
    let lang = [
        "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
        "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
        "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
        "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la",
        "lk", "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam",
        "latin", "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se",
        "tg", "th", "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk",
        "trans", "sn",
    ];
    // let lang = [
    //     "af", "al", "am", "ancient", "apl", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "bqn",
    //     "br", "brai", "bt", "bw", "by", "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee",
    //     "eg", "epo", "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu",
    //     "id", "ie", "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la",
    //     "lk", "lt", "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam",
    //     "latin", "ng", "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se",
    //     "tg", "th", "tj", "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk",
    //     "trans", "sn",
    // ];
    for locale in lang {
        println!("{}", locale);
        let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
        for layout in wkb.layouts() {
            // println!("{}", layout);
            let mut xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
            let mut wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout));
            let state_modifiers = wkb.modifiers.evdev_codes();
            // level 2
            let i = state_modifiers[0];
            xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
            wkb.update_key(i, wkb::KeyDirection::Down);
            test_all_keys(wkb, xkb);
        }
    }
}
