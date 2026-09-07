//! Client-only IR tests for compose table export/import.

#![cfg(feature = "client")]

use wkb::ir::{self, LayoutFile};
use wkb::WKB;

const COMPOSE: char = ir::COMPOSE_KEY_CHAR;

#[test]
fn exported_compose_is_stable_on_roundtrip() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    if file.compose.is_empty() {
        return; // no compose data available in this environment
    }

    let compose = file.compose;
    let wkb2 = WKB::new_from_layouts(vec![file]).unwrap();
    assert_eq!(wkb2.export_layout(0).unwrap().compose, compose);
}

#[test]
fn layout_file_compose_roundtrip() {
    let mut level0 = std::collections::BTreeMap::new();
    level0.insert(30u32, 'a');
    level0.insert(31u32, 'e');
    let mut keymap = std::collections::BTreeMap::new();
    keymap.insert(0u8, level0);

    let file = LayoutFile {
        version: ir::FORMAT_VERSION,
        layout: "test".to_string(),
        repeat_keys: vec![],
        repeat_remove: vec![],
        modifiers: vec![],
        keymap,
        num_lock_keys: Default::default(),
        caps_lock_keymap: Default::default(),
        keysym_map: Default::default(),
        compose: vec![(vec![COMPOSE, 'a', 'e'], 'æ')],
    };

    let wkb = WKB::new_from_layouts(vec![file]).unwrap();
    let exported = wkb.export_layout(0).unwrap();
    assert_eq!(exported.compose, vec![(vec![COMPOSE, 'a', 'e'], 'æ')]);
}
