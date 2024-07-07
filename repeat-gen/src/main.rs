include!(concat!(env!("OUT_DIR"), "/repeat.rs"));

fn main() {
    let layout = REPEAT_KEYS.get("no");
    if let Some(layout) = layout {
        let rep = layout.get("colemak");
        println!("{:?}", rep);
    }
}
