//! Tests for the `wkb::ir` layout file format.

use std::collections::BTreeMap;

use wkb::ir::{self, IrError, LayoutFile, ModAction};
use wkb::{ModType, WKB};

const COMPOSE: char = ir::COMPOSE_KEY_CHAR;

fn sample_file() -> LayoutFile {
    let mut keymap = BTreeMap::new();
    let mut level0 = BTreeMap::new();
    level0.insert(0u32, 'a');
    level0.insert(1u32, 'b');
    let mut levels = BTreeMap::new();
    levels.insert(0u8, level0);
    keymap.insert("us".to_string(), levels);

    LayoutFile {
        version: ir::FORMAT_VERSION,
        layout_names: vec!["us".to_string()],
        num_keys: 128,
        repeat_keys_add: vec![1, 2, 3],
        repeat_keys_remove: Vec::new(),
        modifiers: vec![(
            42,
            "LeftShift".to_string(),
            vec![(0, ModAction::Pressed(ModType::Level2))],
        )],
        keymap,
        num_lock_keys: BTreeMap::new(),
        caps_lock_keymap: BTreeMap::new(),
        keysym_map: BTreeMap::new(),
        compose: vec![(vec![COMPOSE, 'a', 'e'], 'æ')],
    }
}

#[test]
fn serde_roundtrip() {
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
    assert!(text.starts_with("(\n"));
    assert!(text.contains("layout_names: [\n        \"us\",\n    ],"));
    assert!(text.contains("(42, \"LeftShift\", [\n            (0, Pressed(Level2)),\n        ])"));
    assert!(text.contains("'·'"));
    assert!(text.contains("'æ'"));
    assert!(text.contains("compose: ["));
    // Redundant (empty) sections are omitted.
    assert!(!text.contains("num_lock_keys:"));
    assert!(!text.contains("repeat_keys_remove:"));
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

    let text = sample_file()
        .to_ron_string()
        .unwrap()
        .replace(&format!("version: {}", ir::FORMAT_VERSION), "version: 99");
    assert!(matches!(
        LayoutFile::from_ron_str(&text),
        Err(IrError::UnsupportedVersion(99))
    ));
}

#[test]
fn rejects_empty_layout_names() {
    let mut file = sample_file();
    file.layout_names.clear();
    assert!(matches!(file.validate(), Err(IrError::EmptyLayoutNames)));
}

#[test]
fn rejects_multiple_layouts() {
    let mut file = sample_file();
    file.layout_names.push("de".to_string());
    assert!(matches!(file.validate(), Err(IrError::MultipleLayouts(2))));
}

#[test]
fn rejects_undeclared_layout_in_section() {
    let mut file = sample_file();
    file.keymap.insert("de".to_string(), BTreeMap::new());
    assert!(matches!(
        file.validate(),
        Err(IrError::UndeclaredLayout(name)) if name == "de"
    ));
}

#[test]
fn rejects_keycode_out_of_range() {
    let mut file = sample_file();
    file.repeat_keys_add.push(128); // num_keys = 128, so 128 is out of range
    assert!(matches!(
        file.validate(),
        Err(IrError::KeycodeOutOfRange(128, 128))
    ));
}

#[test]
fn rejects_level_out_of_range() {
    let mut file = sample_file();
    file.modifiers[0].2.push((8, ModAction::None)); // MAX_LEVELS = 8
    assert!(matches!(file.validate(), Err(IrError::LevelOutOfRange(8))));
}

#[test]
fn rejects_invalid_num_keys() {
    let mut file = sample_file();
    file.num_keys = 0;
    assert!(matches!(file.validate(), Err(IrError::InvalidNumKeys(0))));
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
    file.modifiers[0].2.clear();
    assert!(matches!(
        file.validate(),
        Err(IrError::EmptyModifierActions(42))
    ));
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
fn exported_compose_is_reachable_filtered() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    if file.compose.is_empty() {
        return; // no compose data available in this environment
    }

    let mut reachable = Vec::new();
    for levels in file.keymap.values() {
        for keys in levels.values() {
            reachable.extend(keys.values().copied());
        }
    }
    for levels in file.caps_lock_keymap.values() {
        for keys in levels.values() {
            reachable.extend(keys.values().copied());
        }
    }
    for levels in file.num_lock_keys.values() {
        for keys in levels.values() {
            reachable.extend(keys.values().copied());
        }
    }
    reachable.sort_unstable();
    reachable.dedup();

    for (keys, _) in &file.compose {
        for ch in keys {
            if *ch == COMPOSE {
                continue;
            }
            assert!(
                reachable.binary_search(ch).is_ok(),
                "compose key {ch:?} is not reachable in the layout"
            );
        }
    }
}

#[test]
fn repeat_set_survives_roundtrip() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    assert!(!file.repeat_keys_add.is_empty());

    let wkb2 = WKB::new_from_layouts(vec![file]).unwrap();
    for code in [38u32, 1, 57] {
        assert_eq!(wkb2.key_repeats(code), wkb.key_repeats(code));
    }
}

#[test]
fn modifier_names_are_deterministic() {
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
    let layouts = wkb::list_layouts();
    assert!(!layouts.is_empty(), "XKB registry should enumerate layouts");
    assert!(layouts.iter().any(|(name, _)| name == "us"));
    // Every pair is directly consumable by new_from_names.
    let (name, variant) = layouts
        .iter()
        .find(|(name, _)| name == "us")
        .expect("us layout present");
    WKB::new_from_names("", "", name, variant, None).expect("us layout compiles");
}
