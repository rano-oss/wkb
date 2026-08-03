//! Generate wkb layout data files for every XKB layout and variant.
//!
//! Takes no arguments. Reads the XKB registry (`rules/evdev.xml`) to enumerate
//! all `(layout, variant)` pairs, compiles each one, and writes the canonical
//! `wkb::ir` RON format into `ron_layouts/`, mirroring the naming convention
//! `<layout>.<variant>.ron` (base layout: `<layout>.ron`).
//!
//! ```sh
//! cargo run --example gen_layouts
//! ```

use std::path::PathBuf;

use wkb::WKB;

const OUT_DIR: &str = "ron_layouts";

fn main() {
    let layouts = wkb::list_layouts();
    println!(
        "found {} layout(s)/variant(s) in the XKB registry",
        layouts.len()
    );

    let out_dir = PathBuf::from(OUT_DIR);
    std::fs::create_dir_all(&out_dir).expect("failed to create output directory");

    let mut written = 0usize;
    let mut skipped = 0usize;
    for (layout, variant) in &layouts {
        let path = out_dir.join(file_name(layout, variant));
        let text = match generate(layout, variant) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("skipped {} ({}): {err}", layout, variant);
                skipped += 1;
                continue;
            }
        };
        std::fs::write(&path, text).expect("failed to write output file");
        written += 1;
    }

    println!("wrote {written} file(s) to {OUT_DIR}/, skipped {skipped}");
}

fn generate(layout: &str, variant: &str) -> Result<String, String> {
    let wkb = WKB::new_from_names("", "", layout, variant, None).map_err(|err| err.to_string())?;
    let file = wkb.export_layout(0).map_err(|err| err.to_string())?;
    file.to_ron_string().map_err(|err| err.to_string())
}

fn file_name(layout: &str, variant: &str) -> String {
    if variant.is_empty() {
        format!("{layout}.ron")
    } else {
        format!("{layout}.{variant}.ron")
    }
}
