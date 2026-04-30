//! Find keys that always produce the same character across ALL XKB layouts.
//! These keys can be omitted from .ron files (assumed as defaults).
//!
//! Usage: cargo run --release --example find_invariant_keys

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    let evdev_lst = Path::new("/usr/share/X11/xkb/rules/evdev.lst");
    let (layouts, variants) = parse_evdev_lst(evdev_lst);

    // For each (evdev, level), track: HashMap<Option<char>, count>
    // Key: (evdev_code, level), Value: map of char -> how many layouts have it
    let mut key_char_counts: HashMap<(u32, usize), HashMap<Option<char>, u32>> = HashMap::new();
    let mut key_sym_counts: HashMap<(u32, usize), HashMap<u32, u32>> = HashMap::new();
    let mut total_layouts = 0u32;

    let mut entries: Vec<(&str, &str)> = layouts.iter().map(|l| (l.as_str(), "")).collect();
    for (v, p) in &variants {
        entries.push((p.as_str(), v.as_str()));
    }

    for (layout, variant) in &entries {
        let wkb = match wkb::WKB::new_from_names("", "", layout, variant, None) {
            Ok(w) => w,
            Err(_) => continue,
        };
        total_layouts += 1;

        // Only check layout 0
        for evdev in 0u32..701 {
            for level in 0..8usize {
                let ch = wkb.level_key_char(evdev, 0, level);
                *key_char_counts
                    .entry((evdev, level))
                    .or_default()
                    .entry(ch)
                    .or_default() += 1;

                let sym = wkb.level_keysym(evdev, 0, level);
                *key_sym_counts
                    .entry((evdev, level))
                    .or_default()
                    .entry(sym)
                    .or_default() += 1;
            }
        }

        if total_layouts % 50 == 0 {
            eprint!("\r  Compiled {} layouts...", total_layouts);
        }
    }
    eprintln!("\r  Compiled {} layouts total.", total_layouts);

    // Find keys where ALL layouts agree on the same char (level 0 only for simplicity first)
    println!(
        "\n=== Invariant keys (same char in ALL {} layouts) ===",
        total_layouts
    );
    println!("Format: evdev level -> char (or None) [keysym]");

    let mut invariant_chars: BTreeMap<(u32, usize), (Option<char>, u32)> = BTreeMap::new();

    for (&(evdev, level), char_map) in &key_char_counts {
        // Get the count of None entries
        let none_count = char_map.get(&None).copied().unwrap_or(0);
        let some_entries: Vec<_> = char_map.iter().filter(|(&k, _)| k.is_some()).collect();

        // If there's exactly one non-None char value, and it appears in >=99% of
        // layouts that DO produce a char for this key (ignore None layouts)
        if some_entries.len() == 1 {
            let (&ch, &count) = some_entries[0];
            let total_with_char = total_layouts - none_count;
            if total_with_char > 0 && count as f64 / total_with_char as f64 >= 0.99 {
                let sym_map = &key_sym_counts[&(evdev, level)];
                // For keysyms, also ignore 0 entries (no keysym)
                let non_zero_syms: Vec<_> = sym_map.iter().filter(|(&k, _)| k != 0).collect();
                let sym = if non_zero_syms.len() == 1 {
                    *non_zero_syms[0].0
                } else {
                    0
                };
                invariant_chars.insert((evdev, level), (ch, sym));
            }
        }
    }

    // Group by evdev for readability
    let mut by_evdev: BTreeMap<u32, Vec<(usize, Option<char>, u32)>> = BTreeMap::new();
    for (&(evdev, level), &(ch, sym)) in &invariant_chars {
        by_evdev.entry(evdev).or_default().push((level, ch, sym));
    }

    // Print keys that are invariant at level 0 with Some(char)
    println!("\n--- Level 0 invariant chars ---");
    for (&evdev, levels) in &by_evdev {
        for &(level, ch, sym) in levels {
            if level == 0 {
                let ch_str = match ch {
                    Some(c) if c.is_control() => format!("U+{:04X}", c as u32),
                    Some(c) => format!("'{}'", c),
                    None => "None".into(),
                };
                let sym_name = wkb::keysyms::keysym_get_name(sym).unwrap_or("?");
                println!("  evdev {:>3} -> {} [{}]", evdev, ch_str, sym_name);
            }
        }
    }

    // Print keys invariant at ALL levels (same across all layouts for every level)
    println!("\n--- Keys invariant at ALL levels (0-7) ---");
    for (&evdev, levels) in &by_evdev {
        if levels.len() == 8 {
            // Check if all levels have the same char as level 0
            let level0_ch = levels.iter().find(|(l, _, _)| *l == 0).map(|(_, c, _)| *c);
            let all_same = levels.iter().all(|(_, c, _)| Some(*c) == level0_ch);
            let ch = level0_ch.unwrap_or(None);
            let ch_str = match ch {
                Some(c) if c.is_control() => format!("U+{:04X}", c as u32),
                Some(c) => format!("'{}'", c),
                None => "None".into(),
            };
            if all_same {
                println!("  evdev {:>3} -> {} (all levels identical)", evdev, ch_str);
            } else {
                let parts: Vec<String> = levels
                    .iter()
                    .map(|(l, c, _)| {
                        let cs = match c {
                            Some(ch) if ch.is_control() => format!("U+{:04X}", *ch as u32),
                            Some(ch) => format!("'{}'", ch),
                            None => "None".into(),
                        };
                        format!("L{}={}", l, cs)
                    })
                    .collect();
                println!("  evdev {:>3} -> {}", evdev, parts.join(" "));
            }
        }
    }

    // Count how many keys are NOT invariant at level 0 (these are the layout-specific ones)
    let mut non_invariant_l0 = 0;
    let mut invariant_l0_some = 0;
    let mut invariant_l0_none = 0;
    for evdev in 0u32..701 {
        match invariant_chars.get(&(evdev, 0)) {
            Some((Some(_), _)) => invariant_l0_some += 1,
            Some((None, _)) => invariant_l0_none += 1,
            None => non_invariant_l0 += 1,
        }
    }
    println!("\n--- Summary (level 0) ---");
    println!("  Invariant with char: {}", invariant_l0_some);
    println!("  Invariant None:      {}", invariant_l0_none);
    println!("  Layout-specific:     {}", non_invariant_l0);

    // Also find keys that are almost-invariant (same in >95% of layouts)
    println!("\n--- Almost-invariant level 0 (>99% same) ---");
    let threshold = (total_layouts as f64 * 0.99) as u32;
    for evdev in 0u32..701 {
        if invariant_chars.contains_key(&(evdev, 0)) {
            continue;
        }
        if let Some(char_map) = key_char_counts.get(&(evdev, 0)) {
            if let Some((&ch, &count)) = char_map.iter().max_by_key(|(_, c)| **c) {
                if count >= threshold {
                    let ch_str = match ch {
                        Some(c) if c.is_control() => format!("U+{:04X}", c as u32),
                        Some(c) => format!("'{}'", c),
                        None => "None".into(),
                    };
                    let exceptions = total_layouts - count;
                    println!(
                        "  evdev {:>3} -> {} ({} exceptions out of {})",
                        evdev, ch_str, exceptions, total_layouts
                    );
                }
            }
        }
    }

    // Output Rust table for DEFAULT_KEYSYMS: keys always None in state_keymap with invariant keysyms
    println!("\n=== Rust DEFAULT_KEYSYMS table (evdev, level0_keysym) ===");
    println!("/// Keys that always produce None in state_keymap across all layouts.");
    println!("/// Their keysyms are identical in all layouts at level 0.");
    println!("/// On deserialize, these are repopulated into keysym_map at all levels.");
    println!("static DEFAULT_KEYSYMS: &[(u32, u32)] = &[");
    for evdev in 0u32..701 {
        // Check: invariant None at all levels in state_keymap
        let all_none = (0..8).all(|level| {
            invariant_chars
                .get(&(evdev, level))
                .map(|(ch, _)| ch.is_none())
                .unwrap_or(false)
        });
        if !all_none {
            continue;
        }

        // Get the invariant keysym at level 0
        if let Some(sym_map) = key_sym_counts.get(&(evdev, 0)) {
            let non_zero: Vec<_> = sym_map.iter().filter(|(&k, _)| k != 0).collect();
            if non_zero.len() == 1 {
                let (&sym, _) = non_zero[0];
                let name = wkb::keysyms::keysym_get_name(sym).unwrap_or("?");
                println!("    ({}, 0x{:04x}), // {}", evdev, sym, name);
            }
        }
    }
    println!("];");

    // Output INVARIANT_CHARS table: keys that produce chars and are invariant (or almost) across layouts
    // Include keys at >=99% threshold
    let threshold99 = (total_layouts as f64 * 0.99) as u32;
    println!("\n=== Rust INVARIANT_CHARS table ===");
    println!("/// Keys invariant (or almost-invariant >99%) across all layouts.");
    println!("/// Format: (evdev, [(level, char)]) — omit levels where char is None.");
    println!("/// During RON serialization, these are omitted. During deserialization, they're pre-populated.");
    println!("pub(crate) static INVARIANT_CHARS: &[(u32, &[(u8, char)])] = &[");

    for evdev in 0u32..701 {
        // Check if this evdev is invariant or almost-invariant at level 0
        let is_invariant_l0 = invariant_chars
            .get(&(evdev, 0))
            .map(|(ch, _)| ch.is_some())
            .unwrap_or(false);
        let is_almost_l0 = if !is_invariant_l0 {
            key_char_counts
                .get(&(evdev, 0))
                .map(|m| {
                    m.iter()
                        .filter(|(ch, _)| ch.is_some())
                        .max_by_key(|(_, c)| **c)
                        .map(|(_, &c)| c >= threshold99)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        } else {
            false
        };

        if !is_invariant_l0 && !is_almost_l0 {
            continue;
        }

        // Get the majority char at each level
        let mut level_chars: Vec<(u8, char)> = Vec::new();
        for level in 0..8usize {
            if let Some(char_map) = key_char_counts.get(&(evdev, level)) {
                // Find majority char (most common across layouts)
                if let Some((&ch, &count)) = char_map
                    .iter()
                    .filter(|(ch, _)| ch.is_some())
                    .max_by_key(|(_, c)| **c)
                {
                    if count >= threshold99 {
                        if let Some(c) = ch {
                            level_chars.push((level as u8, c));
                        }
                    }
                }
            }
        }

        if level_chars.is_empty() {
            continue;
        }

        let parts: Vec<String> = level_chars
            .iter()
            .map(|(l, c)| {
                if c.is_control() || *c == '\'' || *c == '\\' {
                    format!("({}, '\\u{{{:04X}}}')", l, *c as u32)
                } else {
                    format!("({}, '{}')", l, c)
                }
            })
            .collect();
        println!("    ({}, &[{}]),", evdev, parts.join(", "));
    }
    println!("];");

    // Also output INVARIANT_KEYSYMS for keys with chars (keysym at each level)
    println!("\n=== Rust INVARIANT_KEYSYMS_WITH_CHARS table ===");
    println!("/// Keysyms for invariant char-producing keys, per level.");
    println!("pub(crate) static INVARIANT_KEYSYMS_WITH_CHARS: &[(u32, &[(u8, u32)])] = &[");
    for evdev in 0u32..701 {
        let is_invariant_l0 = invariant_chars
            .get(&(evdev, 0))
            .map(|(ch, _)| ch.is_some())
            .unwrap_or(false);
        let is_almost_l0 = if !is_invariant_l0 {
            key_char_counts
                .get(&(evdev, 0))
                .map(|m| {
                    m.iter()
                        .filter(|(ch, _)| ch.is_some())
                        .max_by_key(|(_, c)| **c)
                        .map(|(_, &c)| c >= threshold99)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        } else {
            false
        };

        if !is_invariant_l0 && !is_almost_l0 {
            continue;
        }

        let mut level_syms: Vec<(u8, u32)> = Vec::new();
        for level in 0..8usize {
            if let Some(sym_map) = key_sym_counts.get(&(evdev, level)) {
                if let Some((&sym, &count)) = sym_map
                    .iter()
                    .filter(|(&s, _)| s != 0)
                    .max_by_key(|(_, c)| **c)
                {
                    if count >= threshold99 {
                        level_syms.push((level as u8, sym));
                    }
                }
            }
        }

        if level_syms.is_empty() {
            continue;
        }

        let parts: Vec<String> = level_syms
            .iter()
            .map(|(l, s)| {
                let name = wkb::keysyms::keysym_get_name(*s).unwrap_or("?");
                format!("({}, 0x{:04x}/*{}*/)", l, s, name)
            })
            .collect();
        println!("    ({}, &[{}]),", evdev, parts.join(", "));
    }
    println!("];");

    // Output COMPLETE INVARIANT_KEYSYMS table (all evdev codes, all levels)
    // Strict: keysym must be the same in ALL 594 layouts (not just those that have it)
    println!("\n=== Rust INVARIANT_KEYSYMS (complete) ===");
    println!("static INVARIANT_KEYSYMS: &[(u32, &[(u8, u32)])] = &[");
    for evdev in 0u32..701 {
        let mut level_syms: Vec<(u8, u32)> = Vec::new();
        for level in 0..8usize {
            if let Some(sym_map) = key_sym_counts.get(&(evdev, level)) {
                // Exactly one keysym value (no 0 entries) and it covers all layouts
                if sym_map.len() == 1 {
                    let (&sym, &count) = sym_map.iter().next().unwrap();
                    if sym != 0 && count == total_layouts {
                        level_syms.push((level as u8, sym));
                    }
                }
            }
        }
        if level_syms.is_empty() {
            continue;
        }
        let parts: Vec<String> = level_syms
            .iter()
            .map(|(l, s)| {
                let name = wkb::keysyms::keysym_get_name(*s).unwrap_or("?");
                format!("({}, 0x{:08x}/*{}*/)", l, s, name)
            })
            .collect();
        println!("    ({}, &[{}]),", evdev, parts.join(", "));
    }
    println!("];");
}
