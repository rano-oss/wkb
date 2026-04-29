fn main() {
    let layout = std::env::args().nth(1).unwrap_or("be".into());
    let wkb = wkb::WKB::new_from_names("", "", &layout, "", None).unwrap();
    let ron = wkb.to_ron().unwrap();
    println!("{}", ron);
}
