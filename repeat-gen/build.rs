use std::{
    env,
    fs::File,
    io::{LineWriter, Write},
    path::Path,
};

use xkbcommon::xkb::{self, Keycode};

fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("repeat.rs");
    let file = File::create(&dest_path)?;
    let mut file = LineWriter::new(file);
    file.write_all(b"use std::collections::{HashMap, HashSet};\n")?;
    file.write_all(b"\n")?;
    file.write_all(b"lazy_static::lazy_static! {\n")?;
    file.write_all(
        b"    pub static ref REPEAT_KEYS: HashMap<&'static str, HashMap<&'static str, HashSet<u32>>> =[\n"
    )?;
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
    for locale in lang {
        file.write_all(b"        (\"")?;
        file.write_all(locale.as_bytes())?;
        file.write_all(b"\", [\n")?;
        let wkb = wkb::WKB::new_from_names(locale.to_string(), None);
        for layout in wkb.layouts() {
            file.write_all(b"            (\"")?;
            file.write_all(layout.as_bytes())?;
            file.write_all(b"\", [\n")?;
            file.write_all(b"                ")?;
            let xkb = xkb_new_from_names(locale.to_string(), Some(layout.to_owned()));
            for i in 0..701 {
                if xkb.key_repeats(Keycode::new(i as u32 + 8)) {
                    file.write_all(i.to_string().as_bytes())?;
                    file.write_all(b",")?;
                }
            }
            file.write_all(b"].into()),\n")?;
        }
        file.write_all(b"        ].into()),\n")?;
    }
    file.write_all(b"    ].into();\n")?;
    file.write_all(b"}")?;
    file.flush()?;
    Ok(())
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
