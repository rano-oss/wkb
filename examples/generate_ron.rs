//! Generate RON files for all XKB layouts discovered from evdev.lst.
//!
//! Usage: cargo run --example generate_ron -- [output_dir]
//!
//! Defaults to `ron_layouts/` in the current directory.
//! Each file is named `{layout}.ron` or `{layout}.{variant}.ron`.

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Parse evdev.lst and return (layouts, variants) where variants are (variant_name, parent_layout).
fn parse_evdev_lst(path: &Path) -> (Vec<String>, Vec<(String, String)>) {
    let file = fs::File::open(path).expect("failed to open evdev.lst");
    let reader = BufReader::new(file);
    let mut section = String::new();
    let mut layouts = Vec::new();
    let mut variants = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("! layout") {
            section = "layout".into();
            continue;
        }
        if line.starts_with("! variant") {
            section = "variant".into();
            continue;
        }
        if line.starts_with('!') {
            section.clear();
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if section == "layout" {
            if let Some(name) = trimmed.split_whitespace().next() {
                layouts.push(name.to_string());
            }
        } else if section == "variant" {
            // Format: "variant_name  layout: description"
            let mut parts = trimmed.splitn(2, char::is_whitespace);
            if let Some(variant_name) = parts.next() {
                if let Some(rest) = parts.next() {
                    let rest = rest.trim();
                    if let Some(layout) = rest.split(':').next() {
                        variants.push((variant_name.to_string(), layout.trim().to_string()));
                    }
                }
            }
        }
    }

    (layouts, variants)
}

fn main() {
    let output_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("ron_layouts"));

    fs::create_dir_all(&output_dir).expect("failed to create output directory");

    let evdev_lst = Path::new("/usr/share/X11/xkb/rules/evdev.lst");
    let (layouts, variants) = parse_evdev_lst(evdev_lst);

    println!(
        "Found {} layouts and {} variants",
        layouts.len(),
        variants.len()
    );

    let mut success = 0u32;
    let mut failed = 0u32;

    // Generate base layouts
    for layout in &layouts {
        match wkb::WKB::new_from_names("", "", layout, "", None) {
            Ok(wkb) => match wkb.to_ron() {
                Ok(ron_str) => {
                    let path = output_dir.join(format!("{layout}.ron"));
                    fs::write(&path, &ron_str).expect("failed to write RON file");
                    let kb = ron_str.len() / 1024;
                    println!("  {layout}.ron ({kb} KB)");
                    success += 1;
                }
                Err(e) => {
                    eprintln!("  SKIP {layout}: serialize error: {e}");
                    failed += 1;
                }
            },
            Err(e) => {
                eprintln!("  SKIP {layout}: {e}");
                failed += 1;
            }
        }
    }

    // Generate variants
    for (variant, parent_layout) in &variants {
        match wkb::WKB::new_from_names("", "", parent_layout, variant, None) {
            Ok(wkb) => match wkb.to_ron() {
                Ok(ron_str) => {
                    let path = output_dir.join(format!("{parent_layout}.{variant}.ron"));
                    fs::write(&path, &ron_str).expect("failed to write RON file");
                    let kb = ron_str.len() / 1024;
                    println!("  {parent_layout}.{variant}.ron ({kb} KB)");
                    success += 1;
                }
                Err(e) => {
                    eprintln!("  SKIP {parent_layout}.{variant}: serialize error: {e}");
                    failed += 1;
                }
            },
            Err(e) => {
                eprintln!("  SKIP {parent_layout}.{variant}: {e}");
                failed += 1;
            }
        }
    }

    println!("\nDone: {success} generated, {failed} failed");
    println!("Output: {}", output_dir.display());
}
