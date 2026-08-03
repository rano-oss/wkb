//! Generate a wkb layout data file from XKB RMLVO names.
//!
//! The output is the canonical `wkb::ir` RON format, ready to be loaded back
//! with [`WKB::new_from_layout`].
//!
//! Usage:
//! ```sh
//! cargo run --example gen_layouts -- <output.ron> [layout] [variant]
//! cargo run --example gen_layouts -- af.ron af
//! cargo run --example gen_layouts -- de.ron de nodeadkeys
//! ```

use wkb::WKB;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: gen_layouts <output.ron> [layout] [variant]");
        std::process::exit(2);
    }
    let path = &args[1];
    let layout = args.get(2).map(String::as_str).unwrap_or("us");
    let variant = args.get(3).map(String::as_str).unwrap_or("");

    let wkb = WKB::new_from_names("", "", layout, variant, None)
        .expect("failed to compile keymap from RMLVO names");
    let file = wkb.export_layout(0).expect("failed to export layout");
    let text = file.to_ron_string().expect("failed to serialize layout");
    let len = text.len();

    std::fs::write(path, text).expect("failed to write output file");
    println!("wrote {path} ({len} bytes)");
}
