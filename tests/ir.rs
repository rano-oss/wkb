//! Tests for the `wkb::ir` layout file format.

use std::collections::BTreeMap;

use wkb::ir::{self, IrError, LayoutFile, ModAction};
use wkb::{ModType, NamedKey, WKB};

include!("../test_data/layouts.rs");

const COMPOSE: char = ir::COMPOSE_KEY_CHAR;

fn sample_file() -> LayoutFile {
    let mut level0 = BTreeMap::new();
    level0.insert(0u32, 'a');
    level0.insert(1u32, 'b');
    let mut keymap = BTreeMap::new();
    keymap.insert(0u8, level0);

    LayoutFile {
        version: ir::FORMAT_VERSION,
        layout: "us".to_string(),
        repeat_keys: vec![1, 2, 3],
        modifiers: vec![(42, vec![(0, ModAction::Press(ModType::Level2))])],
        keymap,
        num_lock_keys: BTreeMap::new(),
        caps_lock_keymap: BTreeMap::new(),
        caps_num_lock_keys: BTreeMap::new(),
        keysym_map: BTreeMap::new(),
        compose: vec![(vec![COMPOSE, 'a', 'e'], 'æ')],
    }
}

#[test]
fn ron_roundtrip() {
    let file = sample_file();
    let text = file.to_ron_string().unwrap();
    let back = LayoutFile::from_ron_str(&text).unwrap();
    assert_eq!(back, file);
}

#[test]
fn serialization_is_deterministic() {
    let file = sample_file();
    let a = file.to_ron_string().unwrap();
    let b = file.to_ron_string().unwrap();
    assert_eq!(a, b);
}

#[test]
fn ron_output_matches_suggestion_shape() {
    let file = sample_file();
    let text = file.to_ron_string().unwrap();
    assert!(text.starts_with("// wkb keyboard layout (RON format)\n"));
    assert!(text.contains("    version: 1,\n"));
    assert!(text.contains("    layout: \"us\",\n"));
    assert!(!text.contains("num_keys"));
    assert!(text.contains("repeat_keys: [1, 2, 3],\n"));
    assert!(text.contains("modifiers: [\n"));
    assert!(text.contains("        (42, [(0, Press(Level2))]),\n"));
    assert!(text.contains("    keymap: {\n"));
    assert!(text.contains("        0: {\n"));
    assert!(text.contains("            0: 'a', 1: 'b',\n"));
    assert!(text.contains("    compose: [\n"));
    assert!(text.contains("        (['·', 'a', 'e'], 'æ'),\n"));
    // Redundant (empty) sections are omitted.
    assert!(!text.contains("num_lock_keys"));
}

#[test]
fn char_section_wraps_every_14_keys() {
    let mut level = BTreeMap::new();
    for keycode in 1..=30u32 {
        level.insert(keycode, 'a');
    }
    let mut keymap = BTreeMap::new();
    keymap.insert(0u8, level);
    let file = LayoutFile {
        keymap,
        ..sample_file()
    };
    let text = file.to_ron_string().unwrap();
    assert!(text.contains(
        "1: 'a', 2: 'a', 3: 'a', 4: 'a', 5: 'a', 6: 'a', 7: 'a', 8: 'a', 9: 'a', 10: 'a', 11: 'a', 12: 'a', 13: 'a', 14: 'a',\n            15: 'a', 16: 'a', 17: 'a', 18: 'a', 19: 'a', 20: 'a', 21: 'a', 22: 'a', 23: 'a', 24: 'a', 25: 'a', 26: 'a', 27: 'a', 28: 'a',\n            29: 'a', 30: 'a',\n"
    ));
}

#[test]
fn named_section_stays_one_per_line() {
    let mut level = BTreeMap::new();
    level.insert(0u32, NamedKey::LeftControl);
    level.insert(1u32, NamedKey::LeftShift);
    let mut keysym_map = BTreeMap::new();
    keysym_map.insert(0u8, level);
    let file = LayoutFile {
        keysym_map,
        ..sample_file()
    };
    let text = file.to_ron_string().unwrap();
    assert!(text.contains("0: LeftControl,\n"));
    assert!(text.contains("1: LeftShift,\n"));
}

#[test]
fn rejects_unknown_version() {
    let mut file = sample_file();
    file.version = 99;
    assert!(matches!(
        file.validate(),
        Err(IrError::UnsupportedVersion(99))
    ));
    assert!(file.to_ron_string().is_err());

    let text = sample_file().to_ron_string().unwrap().replacen(
        &format!("version: {},", ir::FORMAT_VERSION),
        "version: 99,",
        1,
    );
    assert!(matches!(
        LayoutFile::from_ron_str(&text),
        Err(IrError::UnsupportedVersion(99))
    ));
}

#[test]
fn rejects_empty_layout_name() {
    let mut file = sample_file();
    file.layout.clear();
    assert!(matches!(file.validate(), Err(IrError::EmptyLayoutName)));
}

#[test]
fn rejects_keycode_out_of_range() {
    let mut file = sample_file();
    file.repeat_keys.push(ir::NUM_KEYS); // 1024 is out of range
    assert!(matches!(
        file.validate(),
        Err(IrError::KeycodeOutOfRange(ir::NUM_KEYS, ir::NUM_KEYS))
    ));
}

#[test]
fn rejects_level_out_of_range() {
    let mut file = sample_file();
    file.modifiers[0]
        .1
        .push((8, ModAction::Press(ModType::None))); // MAX_LEVELS = 8
    assert!(matches!(file.validate(), Err(IrError::LevelOutOfRange(8))));
}

#[test]
fn rejects_empty_compose_sequence() {
    let mut file = sample_file();
    file.compose.push((Vec::new(), 'x'));
    assert!(matches!(
        file.validate(),
        Err(IrError::EmptyComposeSequence)
    ));
}

#[test]
fn rejects_empty_modifier_actions() {
    let mut file = sample_file();
    file.modifiers[0].1.clear();
    assert!(matches!(
        file.validate(),
        Err(IrError::EmptyModifierActions(42))
    ));
}

#[test]
fn ron_fixtures_roundtrip() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ron_layouts");
    if !dir.exists() {
        return; // gitignored fixtures, regenerated by `gen_layouts`
    }
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("ron") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        let file = LayoutFile::from_ron_str(&text)
            .unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()));
        let re = file.to_ron_string().unwrap();
        assert_eq!(re, text, "roundtrip mismatch for {}", path.display());
        checked += 1;
    }
    assert!(checked > 0, "no .ron fixtures found");
}

#[test]
fn xkb_roundtrip_is_exact() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let first = wkb.export_layout(0).unwrap();

    let text = first.to_ron_string().unwrap();
    let parsed = LayoutFile::from_ron_str(&text).unwrap();
    assert_eq!(parsed, first);

    let wkb2 = WKB::new_from_layouts(vec![parsed]).unwrap();
    let second = wkb2.export_layout(0).unwrap();
    assert_eq!(second, first);
}

#[test]
fn xkb_roundtrip_preserves_behavior() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    let mut wkb2 = WKB::new_from_layouts(vec![file]).unwrap();

    // 'a' key (evdev 38) under plain state.
    assert_eq!(wkb2.key_char(38), wkb.key_char(38));
    // Caps-locked 'a' resolves through the caps override path.
    wkb2.update_modifiers(0, 0, 1, 0); // locked = CapsLock bit
    let mut wkb_caps = WKB::new_from_names("", "", "us", "", None).unwrap();
    wkb_caps.update_modifiers(0, 0, 1, 0);
    assert_eq!(wkb2.key_char(38), wkb_caps.key_char(38));
}

#[test]
#[cfg(feature = "compositor")]
fn xkb_export_omits_compose() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    assert!(file.compose.is_empty());
}

#[test]
#[cfg(feature = "compositor")]
fn layout_file_compose_ignored_on_import() {
    let mut file = sample_file();
    file.compose = vec![(vec![COMPOSE, 'a', 'e'], 'æ')];
    let wkb = WKB::new_from_layouts(vec![file]).unwrap();
    let exported = wkb.export_layout(0).unwrap();
    assert!(exported.compose.is_empty());
}

#[test]
fn repeat_set_survives_roundtrip() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    assert!(!file.repeat_keys.is_empty());

    let wkb2 = WKB::new_from_layouts(vec![file]).unwrap();
    for code in [38u32, 1, 57] {
        assert_eq!(wkb2.key_repeats(code), wkb.key_repeats(code));
    }
}

#[test]
fn modifiers_are_deterministic() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    assert!(!file.modifiers.is_empty());

    let a = file.to_ron_string().unwrap();
    let wkb2 = WKB::new_from_layouts(vec![file]).unwrap();
    let b = wkb2.export_layout(0).unwrap().to_ron_string().unwrap();
    assert_eq!(a, b);
}

#[test]
fn new_from_layouts_supports_multiple_groups() {
    let us = WKB::new_from_names("", "", "us", "", None).unwrap();
    let de = WKB::new_from_names("", "", "de", "", None).unwrap();
    let mut wkb = WKB::new_from_layouts(vec![
        us.export_layout(0).unwrap(),
        de.export_layout(0).unwrap(),
    ])
    .unwrap();

    assert_eq!(wkb.num_layouts(), 2);
    assert_eq!(wkb.layout_name(0), Some("English (US)"));
    assert_eq!(wkb.layout_name(1), Some("German"));

    // evdev 30 = KEY_A in both layouts.
    assert_eq!(wkb.key_char(30), Some('a'));
    // evdev 21 is 'y' on US.
    assert_eq!(wkb.key_char(21), Some('y'));

    // Switch to the German group and re-check.
    wkb.set_layout(1).unwrap();
    assert_eq!(wkb.active_layout_idx(), 1);
    assert_eq!(wkb.key_char(30), Some('a'));
    assert_eq!(wkb.key_char(21), Some('z')); // QWERTZ: the 'y' key produces 'z'
}

#[test]
fn list_layouts_finds_the_registry() {
    let layouts = list_layouts();
    assert!(!layouts.is_empty(), "XKB registry should enumerate layouts");
    assert!(layouts.iter().any(|(name, _)| name == "us"));
    // Every pair is directly consumable by new_from_names.
    let (name, variant) = layouts
        .iter()
        .find(|(name, _)| name == "us")
        .expect("us layout present");
    WKB::new_from_names("", "", name, variant, None).expect("us layout compiles");
}
