use std::path::Path;

use wkb::Keymap;

fn main() {
    // let file_name = std::env::args().nth(1).expect("USAGE: debug <FILE>");
    let mut keymap = Keymap::new();
    // keymap.map(Path::new("test/"), file_name, None);
    keymap.map(Path::new("/usr/share/X11/xkb/symbols/"), "no".to_string(), None);
    let key = keymap.key(31);
    println!("{:?}", key);
    println!("{:?}", keymap);
}
