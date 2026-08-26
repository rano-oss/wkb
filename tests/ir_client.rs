//! Client-only IR tests for compose table export/import.

#![cfg(feature = "client")]

use wkb::ir::{self, LayoutFile};
use wkb::WKB;

const COMPOSE: char = ir::COMPOSE_KEY_CHAR;

#[test]
fn exported_compose_is_reachable_filtered() {
    let wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    let file = wkb.export_layout(0).unwrap();
    if file.compose.is_empty() {
        return; // no compose data available in this environment
    }

    let mut reachable = Vec::new();
    for keys in file.keymap.values() {
        reachable.extend(keys.values().copied());
    }
    for keys in file.caps_lock_keymap.values() {
        reachable.extend(keys.values().copied());
    }
    for keys in file.num_lock_keys.values() {
        reachable.extend(keys.values().copied());
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
        modifiers: vec![],
        keymap,
        num_lock_keys: Default::default(),
        caps_lock_keymap: Default::default(),
        caps_num_lock_keys: Default::default(),
        keysym_map: Default::default(),
        compose: vec![(vec![COMPOSE, 'a', 'e'], 'æ')],
    };

    let wkb = WKB::new_from_layouts(vec![file]).unwrap();
    let exported = wkb.export_layout(0).unwrap();
    assert_eq!(exported.compose, vec![(vec![COMPOSE, 'a', 'e'], 'æ')]);
}
