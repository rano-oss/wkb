// use std::path::Path;

use wkb::WKB;

fn main() {
    let file_name = std::env::args().nth(1).expect("USAGE: debug <FILE>");
    // let wkb = WKB::new_from_names("no".to_string(), None);
    let wkb = WKB::new_from_names(file_name, None);
    // // keymap.map(Path::new("test/"), file_name, None);
    // keymap.map(Path::new("/usr/share/X11/xkb/symbols/"), "no".to_string(), None);
    // let key = keymap.key(31);|
    // println!("{:?}", key);
    // println!("{:?}", wkb.layouts());
    // println!("{:?}", wkb.layouts().len());
    // println!("{:?}", wkb.level_keymap());
}
