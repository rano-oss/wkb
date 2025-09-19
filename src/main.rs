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

fn test_all_keys(mut wkb: WKB, xkb: xkb::State) {
    for i in 0..701 {
        let k1 = wkb.utf8(i);
        let k2 = xkb.key_get_utf8(Keycode::new(i as u32 + 8));
        if k1 != k2.chars().last() && !k2.is_empty() {
            println!("wkb: {:?}, xkb: {:?}, {}", k1, k2.chars().last(), i);
        }
    }
}

fn main() {
    let lang = "de".to_string();
    let layout = "neo_base".to_string();
    let mut xkb = xkb_new_from_names(lang.clone(), Some(layout.clone()));
    let mut wkb = wkb::WKB::new_from_names(lang, Some(layout));
    // xkb.update_key(Keycode::new(NUM_LOCK as u32 + 8), xkb::KeyDirection::Down);
    // wkb.update_key(NUM_LOCK, wkb::KeyDirection::Down);
    // test_all_keys(wkb.clone(), xkb.clone());
    // level 2
    // xkb.update_key(Keycode::new(LEFT_SHIFT as u32 + 8), xkb::KeyDirection::Down);
    // wkb.update_key(LEFT_SHIFT, wkb::KeyDirection::Down);
    // test_all_keys(wkb, xkb);

    // level 5
    if let Some(i) = wkb.modifiers.level5_code() {
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
    }
    // level 3
    if let Some(i) = wkb.modifiers.level3_code() {
        xkb.update_key(Keycode::new(i as u32 + 8), xkb::KeyDirection::Down);
        wkb.update_key(i, wkb::KeyDirection::Down);
    }
    println!("{:?}", wkb.modifiers);
    test_all_keys(wkb, xkb);
}
