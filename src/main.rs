use std::path::Path;

use wkb::WKB;

// use unicode_normalization::char::compose;

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
    // let a = unicode_normalization::char::compose('a', '́');
    // let b = unicode_normalization::char::compose('o', '\u{0311}');
    // let c = unicode_normalization::char::compose('a', '\u{0306}');
    // let d = unicode_normalization::char::compose(c.unwrap(), '\u{0301}');
    //println!("{:?}{:?}{:?}{:?}", a, b, c, d);
    //println!("{:?}", unicode_normalization::char::is_combining_mark('a'));
    // println!(
    //     "{:?}",
    //     unicode_normalization::char::is_combining_mark('\u{0344}')
    // );
}
