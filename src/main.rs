// use std::path::Path;

// use wkb::WKB;
use xkbcommon::xkb::{self, Keycode};

fn main() {
    // let hx = hex::decode("01D513");
    // match hx {
    //     Ok(s) => {
    //         println!("{:?}", s);
    //     }
    //     _ => {}
    // }
    // let u = string_to_unicode_char("U1D513");
    // println!("{:?}", u);
    // let file_name = std::env::args().nth(1).expect("USAGE: debug <FILE>");
    // let wkb = WKB::new_from_names("keypad".to_string(), None);
    // let wkb = WKB::new_from_names(file_name, None);
    // // keymap.map(Path::new("test/"), file_name, None);
    // keymap.map(Path::new("/usr/share/X11/xkb/symbols/"), "no".to_string(), None);
    // let key = keymap.key(31);|
    // println!("{:?}", key);
    // println!("{:?}", wkb.layouts());
    // println!("{:?}", wkb.layouts().len());
    // println!("{:?}", wkb.level_keymap());

    let lang = [
        // "af", "al", "am", "ancient", "apl", // "ara",
        // "at", "au", "az", "ba", "bd", //"be",
        // "bg", "bqn", // "br",
        // "bt", "bw", "by", //"ca",
        // "cd", //"ch",
        // "cm", "cn",
        // "cz",
        "de", "dk", "dz", "ee", "eg",
        "epo",
        // "es", "et", "eu", "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id", "ie",
        // "il", "in", "iq", "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "lk", "lt",
        // "lv", "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "latam", "latin", "ng",
        // "nl", "no", "np", "nz", "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "tg", "th", "tj",
        // "tm", "tr", "tw", "tz", "ua", "us", "uz", "vn", "za", "si", "sk", "trans", "sn",
    ];
    let level = 0;
    for locale in lang {
        println!("{}", locale);
        let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
        // println!("{:?}", wkb.level_keymap());
        for layout in wkb.layouts() {
            // println!("{:?}", layout);
            let wkb = wkb::WKB::new_from_names(locale.to_string(), Some(layout.clone()));
            let xkb = xkb_new_from_names(locale.to_string(), Some(layout));
            for i in 1..701 {
                let k1 = wkb.level_key(i as u32, level);
                let mut k2 = xkb
                    .key_get_syms_by_level(Keycode::new(i as u32 + 8), 0, level as u32)
                    .first()
                    .map(|k| k.key_char().unwrap_or_default());
                if k2.unwrap_or_default() == '\0' {
                    k2 = None;
                }
                if k1 != k2 && k2 != None {
                    println!("{:?}, {:?}, {}", k1, k2, i);
                }
            }
        }
    }
}

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
