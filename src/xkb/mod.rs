//! XKB (X Keyboard Extension) support using rust-xkb
//!
//! This module contains the libxkbcommon functionality transpiled from C to Rust.
//! It provides complete XKB keymap compilation and state management for Wayland.
//!
//! Note: This code was originally generated using c2rust. We keep c2rust-bitfields
//! for the bitfield macros but have removed other c2rust dependencies.

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    mutable_transmutes,
    unused_mut,
    unused_assignments,
    unused_variables,
    improper_ctypes,
    improper_ctypes_definitions,
    unknown_lints,
    clippy::all
)]
#![allow(unknown_lints)] // For c2rust attributes that are no longer used

// Core XKB modules
pub mod atom;
pub mod common;
pub mod compile_keymap;
pub mod context;
pub mod context_priv;
pub mod features;
pub mod keymap;
pub mod keymap_compare;
pub mod keymap_formats;
pub mod keymap_priv;
pub mod server_state;
pub mod state;

// Keysym modules
pub mod keysym;
pub mod keysym_case_mappings;
pub mod keysym_unicode;
pub mod keysym_utf;

// Parsing and text processing
pub mod custom_parsers;
pub mod scanner_utils;
pub mod text;

// Utilities
pub mod utf8;
pub mod utf8_decoding;
pub mod util_list;
pub mod utils;
pub mod utils_paths;
pub mod utils_text;

// Compilation modules
pub mod buffercomp;
pub mod filecomp;
pub mod stringcomp;
pub mod xkbcomp; // XKB compiler with all submodules

// Compose support
pub mod compile_compose;
pub mod compose;
pub mod compose_iter;
pub mod compose_parse;
pub mod compose_traversal;
pub mod xkb_compose_map;

// Rules and registry
pub mod registry;
pub mod registry_list;
pub mod rmlvo;
pub mod rules_file;
pub mod rules_file_includes;
pub mod rulescomp;

// Messages and logging
pub mod check_messages;
pub mod log;
pub mod messages;

// Key processing
pub mod key_proc;
pub mod keyseq;

// Extensions
pub mod extensions_directories;

// Lenient mode
pub mod lenient_mode;

// WKB integration functions
use crate::modifiers::*;
use crate::{ListComposer, WKB};
use std::collections::{BTreeMap, HashSet};
use std::ffi::CString;

/// Path to XKB symbols directory
pub const XKB_SYMBOLS_PATH: &str = "/usr/share/X11/xkb/symbols/";

/// Get all available layouts/variants for a given locale
fn get_all_layouts_for_locale(locale: &str) -> Vec<String> {
    use std::ffi::CStr;

    unsafe {
        // Create registry context
        let rxkb_ctx = registry_list::xkbregistry_h::rxkb_context_new(
            registry_list::xkbregistry_h::RXKB_CONTEXT_NO_FLAGS,
        );

        if rxkb_ctx.is_null() {
            // Failed to create registry context, return empty string as default (base layout)
            return vec![String::new()];
        }

        // Load default paths
        registry_list::xkbregistry_h::rxkb_context_include_path_append_default(rxkb_ctx);

        // Parse the registry
        if !registry_list::xkbregistry_h::rxkb_context_parse(
            rxkb_ctx,
            b"evdev\0".as_ptr() as *const i8,
        ) {
            registry_list::xkbregistry_h::rxkb_context_unref(rxkb_ctx);
            return vec![String::new()];
        }

        let mut layouts = Vec::new();

        // Iterate through all layouts
        let mut layout_ptr = registry_list::xkbregistry_h::rxkb_layout_first(rxkb_ctx);

        while !layout_ptr.is_null() {
            let layout_name_ptr = registry_list::xkbregistry_h::rxkb_layout_get_name(layout_ptr);

            if !layout_name_ptr.is_null() {
                let layout_name = CStr::from_ptr(layout_name_ptr).to_string_lossy();

                // Check if this layout matches our locale
                if layout_name == locale {
                    // Get the variant (can be null for base layout)
                    let variant_ptr =
                        registry_list::xkbregistry_h::rxkb_layout_get_variant(layout_ptr);

                    if variant_ptr.is_null() {
                        // Base layout (no variant) - store empty string
                        layouts.push(String::new());
                    } else {
                        let variant_name = CStr::from_ptr(variant_ptr).to_string_lossy();
                        if !variant_name.is_empty() {
                            // Variant layout - store as "locale:variant"
                            layouts.push(variant_name.to_string());
                        }
                    }
                }
            }

            layout_ptr = registry_list::xkbregistry_h::rxkb_layout_next(layout_ptr);
        }

        // Clean up registry context
        registry_list::xkbregistry_h::rxkb_context_unref(rxkb_ctx);

        // If we didn't find any layouts, return empty string as default (base layout)
        if layouts.is_empty() {
            layouts.push(String::new());
        }

        layouts
    }
}

/// Create a new WKB instance from locale and layout names using rust-xkb
pub fn new_from_names(locale: String, layout: Option<String>) -> WKB<ListComposer> {
    unsafe {
        // Import necessary types and constants from rust-xkb modules
        use context::xkbcommon_h::XKB_CONTEXT_NO_FLAGS;
        use keymap::xkbcommon_h::{
            xkb_keymap_compile_flags, xkb_rule_names, XKB_KEYMAP_COMPILE_NO_FLAGS,
        };

        // Get all available layouts for this locale
        let all_layouts = if layout.is_none() {
            get_all_layouts_for_locale(&locale)
        } else {
            // If a specific layout was requested, just use that one
            vec![layout.clone().unwrap()]
        };

        // Create XKB context
        let ctx = context::xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if ctx.is_null() {
            panic!("Failed to create XKB context");
        }

        // Prepare RMLVO names (Rules, Model, Layout, Variant, Options)
        let rules = CString::new("evdev").unwrap();
        let model = CString::new("").unwrap(); // Use empty string instead of pc105
                                               // Layout is always the locale (e.g., "us"), variant is the optional layout parameter (e.g., "dvorak")
        let layout_cstr = CString::new(locale.clone()).unwrap();
        let variant_cstr = CString::new(layout.clone().unwrap_or_else(|| String::new())).unwrap();
        let options_cstr = CString::new("").unwrap();

        let rmlvo = xkb_rule_names {
            rules: rules.as_ptr(),
            model: model.as_ptr(),
            layout: layout_cstr.as_ptr(),
            variant: variant_cstr.as_ptr(),
            options: options_cstr.as_ptr(),
        };

        // Compile keymap from RMLVO names
        let keymap = keymap::xkb_keymap_new_from_names(
            ctx as *mut _,
            &rmlvo as *const _,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        );

        if keymap.is_null() {
            context::xkb_context_unref(ctx);
            panic!("Failed to compile keymap for layout: {:?}", layout);
        }

        // Get keycode range
        let min_keycode = keymap::xkb_keymap_min_keycode(keymap);
        let max_keycode = keymap::xkb_keymap_max_keycode(keymap);

        // XKB supports up to 8 levels (0-7)
        // We need to create 8 levels regardless of how many are actually defined in the keymap,
        // because applications may try to access any level, and XKB has fallback rules for undefined levels.
        const XKB_MAX_LEVELS: usize = 8;

        // First pass: determine the maximum number of levels actually defined in the keymap
        let mut max_defined_levels = 1; // At least level 0
        for keycode in min_keycode..=max_keycode {
            let layout_idx = 0;
            let num_levels = keymap::xkb_keymap_num_levels_for_key(keymap, keycode, layout_idx);
            if num_levels > max_defined_levels {
                max_defined_levels = num_levels;
            }
        }

        // Initialize state_keymap with XKB_MAX_LEVELS levels to support all possible modifier combinations
        let mut state_keymap: Vec<BTreeMap<u32, char>> = vec![BTreeMap::new(); XKB_MAX_LEVELS];
        let mut caps_lock_keymap = vec![BTreeMap::new(); XKB_MAX_LEVELS];
        let mut num_lock_keys = vec![BTreeMap::new(); XKB_MAX_LEVELS];
        let mut level_exceptions_keymap = vec![BTreeMap::new(); XKB_MAX_LEVELS];

        // Note: XKB keycodes for evdev start at 9, but evdev codes start at 0
        // The ESC key is XKB keycode 9 and should map to evdev code 1
        // Therefore: evdev_code = xkb_keycode - 8
        let evdev_offset = 8; // Standard offset to convert XKB keycode to evdev code

        // Build modifiers map from XKB keymap FIRST so we can use it to set modifier states
        let modifiers = build_modifiers_from_keymap(keymap, min_keycode, max_keycode);

        // Populate level_exceptions_keymap by querying keysyms directly from keymap
        // This is used by level_key() to return raw keymap level definitions
        for level in 0..XKB_MAX_LEVELS {
            for keycode in min_keycode..=max_keycode {
                let evdev_code = keycode - evdev_offset;
                let layout_idx = 0;

                // Query keysyms directly at this level
                let mut syms_ptr: *const u32 = std::ptr::null();
                let num_syms = keymap::xkb_keymap_key_get_syms_by_level(
                    keymap,
                    keycode,
                    layout_idx,
                    level as u32,
                    &mut syms_ptr as *mut _,
                );

                if num_syms > 0 && !syms_ptr.is_null() {
                    let keysym = *syms_ptr;
                    let utf32 = keysym_utf::xkb_keysym_to_utf32(keysym);
                    if utf32 != 0 {
                        if let Some(ch) = char::from_u32(utf32) {
                            level_exceptions_keymap[level].insert(evdev_code, ch);
                        }
                    }
                }
            }
        }

        // Build state_keymap by actually simulating modifier combinations with XKB state
        // This ensures we get the correct behavior for keys whose types don't respond to certain modifiers
        // IMPORTANT: Create a fresh state for each level to avoid locking modifier issues
        {
            // Get modifier keycodes from the modifiers we just built
            let level2_keycode = modifiers.level2_code().map(|(code, _)| code + evdev_offset);
            let level3_keycode = modifiers.level3_code().map(|(code, _)| code + evdev_offset);
            let level5_keycode = modifiers.level5_code().map(|(code, _)| code + evdev_offset);

            for level in 0..XKB_MAX_LEVELS {
                // Create a FRESH state for this level to avoid locking modifier issues
                let state_ptr = state::xkb_state_new(keymap as *mut state::keymap_h::xkb_keymap);
                if state_ptr.is_null() {
                    continue;
                }

                // Set up modifiers for this level
                match level {
                    0 => { /* No modifiers */ }
                    1 => {
                        // Level2 (Shift)
                        if let Some(kc) = level2_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    2 => {
                        // Level3
                        if let Some(kc) = level3_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    3 => {
                        // Level3 + Level2 (order matters!)
                        if let Some(kc) = level3_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                        if let Some(kc) = level2_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    4 => {
                        // Level5
                        if let Some(kc) = level5_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    5 => {
                        // Level5 + Level2
                        if let Some(kc) = level5_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                        if let Some(kc) = level2_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    6 => {
                        // Level5 + Level3
                        if let Some(kc) = level5_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                        if let Some(kc) = level3_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    7 => {
                        // Level5 + Level3 + Level2
                        if let Some(kc) = level5_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                        if let Some(kc) = level3_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                        if let Some(kc) = level2_keycode {
                            state::xkb_state_update_key(state_ptr, kc, common::XKB_KEY_DOWN);
                        }
                    }
                    _ => {
                        // Beyond level 7
                        state::xkb_state_unref(state_ptr);
                        continue;
                    }
                }

                // Query each key at this modifier combination
                for keycode in min_keycode..=max_keycode {
                    let evdev_code = keycode - evdev_offset;
                    let layout_idx = 0;

                    // Get the character at this state
                    let mut buffer: [u8; 32] = [0; 32];
                    let size = state::xkb_state_key_get_utf8(
                        state_ptr,
                        keycode,
                        buffer.as_mut_ptr() as *mut i8,
                        buffer.len(),
                    );

                    if size > 0 && (size as usize) < buffer.len() {
                        if let Ok(s) = std::str::from_utf8(&buffer[..size as usize]) {
                            if let Some(ch) = s.chars().next() {
                                // Use the character from state - it already handles fallback internally
                                state_keymap[level].insert(evdev_code, ch);
                            }
                        }
                    } else {
                        // State returned nothing - this can happen for keys whose type doesn't respond
                        // to these modifiers (e.g., numpad with Shift). Fall back to querying the
                        // keymap directly at this level.
                        let mut syms_ptr: *const u32 = std::ptr::null();
                        let num_syms = keymap::xkb_keymap_key_get_syms_by_level(
                            keymap,
                            keycode,
                            layout_idx,
                            level as u32,
                            &mut syms_ptr as *mut _,
                        );

                        if num_syms > 0 && !syms_ptr.is_null() {
                            let keysym = *syms_ptr;
                            let utf32 = keysym_utf::xkb_keysym_to_utf32(keysym);
                            if utf32 != 0 {
                                if let Some(ch) = char::from_u32(utf32) {
                                    state_keymap[level].insert(evdev_code, ch);
                                }
                            }
                        }
                    }
                }

                // Clean up this level's state
                state::xkb_state_unref(state_ptr);
            }
        }

        // Now populate caps_lock_keymap: simulate Caps Lock being active with different modifiers
        {
            // Get the actual Caps Lock keycode from the modifiers map
            // In normal layouts this will be keycode 58 (XKB 66), but in special layouts like Neo
            // the physical caps key might not be mapped to Lock modifier at all
            let caps_lock_xkb_keycode = modifiers
                .level_code(ModType::Caps)
                .map(|(code, _)| code + evdev_offset);

            // Only build caps_lock_keymap if there actually is a caps lock key in this layout
            if let Some(caps_lock_keycode) = caps_lock_xkb_keycode {
                // Create a fresh state for caps lock
                let state_ptr = state::xkb_state_new(keymap as *mut state::keymap_h::xkb_keymap);
                if !state_ptr.is_null() {
                    // Get modifier keycodes
                    let level2_keycode =
                        modifiers.level2_code().map(|(code, _)| code + evdev_offset);
                    let level3_keycode =
                        modifiers.level3_code().map(|(code, _)| code + evdev_offset);
                    let level5_keycode =
                        modifiers.level5_code().map(|(code, _)| code + evdev_offset);

                    // Activate Caps Lock by pressing and releasing it
                    state::xkb_state_update_key(state_ptr, caps_lock_keycode, common::XKB_KEY_DOWN);
                    state::xkb_state_update_key(state_ptr, caps_lock_keycode, common::XKB_KEY_UP);

                    for level in 0..XKB_MAX_LEVELS {
                        // Create a FRESH state for this caps+level combination
                        let caps_state_ptr =
                            state::xkb_state_new(keymap as *mut state::keymap_h::xkb_keymap);
                        if caps_state_ptr.is_null() {
                            continue;
                        }

                        // IMPORTANT: Press level modifiers FIRST, then Caps Lock
                        // This order matters! (e.g., JP layout needs Shift pressed before Caps)
                        match level {
                            0 => { /* Caps only - no other modifiers */ }
                            1 => {
                                // Level2 (Shift) + Caps
                                if let Some(kc) = level2_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            2 => {
                                // Level3 + Caps
                                if let Some(kc) = level3_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            3 => {
                                // Level3 + Level2 + Caps (order matters!)
                                if let Some(kc) = level3_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                                if let Some(kc) = level2_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            4 => {
                                // Level5 + Caps
                                if let Some(kc) = level5_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            5 => {
                                // Level5 + Level2 + Caps
                                if let Some(kc) = level5_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                                if let Some(kc) = level2_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            6 => {
                                // Level5 + Level3 + Caps
                                if let Some(kc) = level5_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                                if let Some(kc) = level3_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            7 => {
                                // Level5 + Level3 + Level2 + Caps
                                if let Some(kc) = level5_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                                if let Some(kc) = level3_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                                if let Some(kc) = level2_keycode {
                                    state::xkb_state_update_key(
                                        caps_state_ptr,
                                        kc,
                                        common::XKB_KEY_DOWN,
                                    );
                                }
                            }
                            _ => {
                                // Beyond level 7
                                state::xkb_state_unref(caps_state_ptr);
                                continue;
                            }
                        }

                        // NOW press Caps Lock (after level modifiers)
                        // Keep it pressed down - don't release!
                        // This is key: caps lock acts as a level shift key in some layouts.
                        state::xkb_state_update_key(
                            caps_state_ptr,
                            caps_lock_keycode,
                            common::XKB_KEY_DOWN,
                        );

                        // Test each key at this level with Caps Lock active
                        for keycode in min_keycode..=max_keycode {
                            let evdev_code = keycode - evdev_offset;

                            let mut buffer: [u8; 32] = [0; 32];
                            let size = state::xkb_state_key_get_utf8(
                                caps_state_ptr,
                                keycode,
                                buffer.as_mut_ptr() as *mut i8,
                                buffer.len(),
                            );

                            if size > 0 && (size as usize) < buffer.len() {
                                if let Ok(s) = std::str::from_utf8(&buffer[..size as usize]) {
                                    if let Some(ch) = s.chars().next() {
                                        // Only store if it's different from the normal state_keymap
                                        if state_keymap.get(level).and_then(|m| m.get(&evdev_code))
                                            != Some(&ch)
                                        {
                                            caps_lock_keymap[level].insert(evdev_code, ch);
                                        }
                                    }
                                }
                            }
                        }

                        // Clean up this caps+level state
                        state::xkb_state_unref(caps_state_ptr);
                    }
                }
            } // Close the if let Some(caps_lock_keycode)
        }

        // Populate num_lock_keys: simulate Num Lock being active with different modifiers
        {
            // Get the actual Num Lock keycode from the modifiers map
            let num_lock_xkb_keycode = modifiers
                .level_code(ModType::Num)
                .map(|(code, _)| code + evdev_offset);

            // Only build num_lock_keys if there actually is a num lock key in this layout
            if let Some(num_lock_keycode) = num_lock_xkb_keycode {
                // Get modifier keycodes
                let level2_keycode = modifiers.level2_code().map(|(code, _)| code + evdev_offset);
                let level3_keycode = modifiers.level3_code().map(|(code, _)| code + evdev_offset);
                let level5_keycode = modifiers.level5_code().map(|(code, _)| code + evdev_offset);

                for level in 0..XKB_MAX_LEVELS {
                    // Create a FRESH state for this num+level combination
                    let num_state_ptr =
                        state::xkb_state_new(keymap as *mut state::keymap_h::xkb_keymap);
                    if num_state_ptr.is_null() {
                        continue;
                    }

                    // Activate Num Lock
                    state::xkb_state_update_key(
                        num_state_ptr,
                        num_lock_keycode,
                        common::XKB_KEY_DOWN,
                    );
                    state::xkb_state_update_key(
                        num_state_ptr,
                        num_lock_keycode,
                        common::XKB_KEY_UP,
                    );

                    // Set up modifiers for this level
                    match level {
                        0 => { /* Num only */ }
                        1 => {
                            // Num + Level2 (Shift)
                            if let Some(kc) = level2_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        2 => {
                            // Num + Level3
                            if let Some(kc) = level3_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        3 => {
                            // Num + Level3 + Level2 (order matters!)
                            if let Some(kc) = level3_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                            if let Some(kc) = level2_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        4 => {
                            // Num + Level5
                            if let Some(kc) = level5_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        5 => {
                            // Num + Level5 + Level2
                            if let Some(kc) = level5_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                            if let Some(kc) = level2_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        6 => {
                            // Num + Level5 + Level3
                            if let Some(kc) = level5_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                            if let Some(kc) = level3_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        7 => {
                            // Num + Level5 + Level3 + Level2
                            if let Some(kc) = level5_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                            if let Some(kc) = level3_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                            if let Some(kc) = level2_keycode {
                                state::xkb_state_update_key(
                                    num_state_ptr,
                                    kc,
                                    common::XKB_KEY_DOWN,
                                );
                            }
                        }
                        _ => {
                            // Beyond level 7
                            state::xkb_state_unref(num_state_ptr);
                            continue;
                        }
                    }

                    // Test each key at this level with Num Lock active
                    for keycode in min_keycode..=max_keycode {
                        let evdev_code = keycode - evdev_offset;

                        let mut buffer: [u8; 32] = [0; 32];
                        let size = state::xkb_state_key_get_utf8(
                            num_state_ptr,
                            keycode,
                            buffer.as_mut_ptr() as *mut i8,
                            buffer.len(),
                        );

                        if size > 0 && (size as usize) < buffer.len() {
                            if let Ok(s) = std::str::from_utf8(&buffer[..size as usize]) {
                                if let Some(ch) = s.chars().next() {
                                    // Only store if it's different from the normal state_keymap
                                    if state_keymap.get(level).and_then(|m| m.get(&evdev_code))
                                        != Some(&ch)
                                    {
                                        num_lock_keys[level].insert(evdev_code, ch);
                                    }
                                }
                            }
                        }
                    }

                    // Clean up this num+level state
                    state::xkb_state_unref(num_state_ptr);
                }
            } // Close the if let Some(num_lock_keycode)
        }

        // Build composer from XKB compose table
        // NOTE: Compose table iteration has memory safety issues in some configurations
        // For now, we use an empty composer. The compose resolution logic is tested separately.
        let _compose_locale = match compose_parse::resolve_compose_file(&locale) {
            Some(path) => {
                // Extract locale from path like "en_US.UTF-8/Compose"
                path.strip_suffix("/Compose")
                    .unwrap_or("en_US.UTF-8")
                    .to_string()
            }
            None => "en_US.UTF-8".to_string(),
        };
        // TODO: Enable when memory safety issues are resolved
        // let composer = build_composer_from_xkb(ctx, &compose_locale);
        let composer = ListComposer::new();

        // Populate repeat_keys: determine which keys are repeatable
        let mut repeat_keys = HashSet::new();
        for keycode in min_keycode..=max_keycode {
            if keymap::xkb_keymap_key_repeats(keymap, keycode) != 0 {
                let evdev_code = keycode - evdev_offset;
                repeat_keys.insert(evdev_code);
            }
        }

        // Clean up
        keymap::xkb_keymap_unref(keymap);
        context::xkb_context_unref(ctx);

        WKB {
            layouts: all_layouts,
            layout: layout.clone().unwrap_or_else(|| locale.clone()),
            locale: Some(locale),
            pressed_keys: HashSet::new(),
            repeat_keys,
            composer,
            modifiers,
            state_keymap,
            num_lock_keys,
            caps_lock_keymap,
            level_exceptions_keymap,
        }
    }
}

/// Build Modifiers struct from XKB keymap by querying modifier mappings
fn build_modifiers_from_keymap(
    keymap: *mut keymap::keymap_h::xkb_keymap,
    min_keycode: u32,
    max_keycode: u32,
) -> Modifiers {
    use std::ffi::CStr;

    unsafe {
        // Start with an EMPTY modifiers map - we'll populate it from the keymap
        // DO NOT use Modifiers::default() because that has hard-coded key mappings
        // that may not match the actual layout (e.g., in neo layout, key 58 is not caps lock)
        let mut modifiers = Modifiers(std::collections::BTreeMap::new());

        // Query all modifiers from the keymap
        let num_mods = keymap::xkb_keymap_num_mods(keymap);

        // Build a map of modifier names to their indices and types
        let mut mod_name_to_type: std::collections::HashMap<String, ModType> =
            std::collections::HashMap::new();

        for mod_idx in 0..num_mods {
            let mod_name_ptr = keymap::xkb_keymap_mod_get_name(keymap, mod_idx);
            if mod_name_ptr.is_null() {
                continue;
            }

            let mod_name = CStr::from_ptr(mod_name_ptr).to_string_lossy().to_string();

            // Map XKB modifier names to our ModType
            let mod_type = match mod_name.as_str() {
                "Shift" => ModType::Level2,
                "ISO_Level3_Shift" | "Mode_switch" | "LevelThree" => ModType::Level3,
                "ISO_Level5_Shift" | "LevelFive" => ModType::Level5,
                "Lock" => ModType::Caps,
                "Mod2" => ModType::Num, // Num_Lock is typically mapped to Mod2
                "Scroll_Lock" | "ScrollLock" => ModType::Scroll,
                _ => ModType::None,
            };

            if mod_type != ModType::None {
                mod_name_to_type.insert(mod_name, mod_type);
            }
        }

        // Now iterate through all keys to find which keys produce which modifiers
        let evdev_offset = 8;
        for keycode in min_keycode..=max_keycode {
            let key = keymap::XkbKey(keymap, keycode);
            if key.is_null() {
                continue;
            }

            let modmap = (*key).modmap;
            let evdev_code = keycode - evdev_offset;

            if modmap == 0 {
                continue; // This key doesn't produce any modifiers
            }

            if modmap == 0 {
                continue; // This key doesn't produce any modifiers
            }

            // Check which modifiers this key produces
            for mod_idx in 0..num_mods {
                let mod_mask = keymap::xkb_keymap_mod_get_mask(
                    keymap,
                    keymap::xkb_keymap_mod_get_name(keymap, mod_idx),
                );

                if (modmap & mod_mask) != 0 {
                    // This key produces this modifier
                    let mod_name_ptr = keymap::xkb_keymap_mod_get_name(keymap, mod_idx);
                    if mod_name_ptr.is_null() {
                        continue;
                    }

                    let mod_name = CStr::from_ptr(mod_name_ptr).to_string_lossy().to_string();

                    if let Some(&mod_type) = mod_name_to_type.get(&mod_name) {
                        let evdev_code = keycode - evdev_offset;

                        // Create appropriate ModKind based on modifier type
                        let mod_kind = match mod_type {
                            ModType::Caps | ModType::Num | ModType::Scroll => ModKind::Lock {
                                pressed: false,
                                locked: 0,
                                mod_type,
                            },
                            _ => ModKind::Pressed {
                                pressed: false,
                                mod_type,
                            },
                        };

                        // Update the modifiers map
                        modifiers.set_modifier(evdev_code, Modifier::Single(mod_kind));
                    }
                }
            }
        }

        modifiers
    }
}

/// Build composer (compose trie) from XKB compose table
fn build_composer_from_xkb(
    ctx: *mut context::context_h::xkb_context,
    locale: &str,
) -> ListComposer {
    use std::ffi::CString;

    unsafe {
        let mut composer = ListComposer::new();

        // For safety, catch any panics during compose table loading
        // Some locales may have invalid or missing compose files
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // Create locale C string
            let locale_cstr = CString::new(locale).unwrap();

            // Cast context pointer to the type expected by compile_compose
            let ctx_cast = ctx as *mut compile_compose::context_h::xkb_context;

            // Create XKB compose table from locale
            let compose_table =
                compile_compose::xkbcommon_compose_h::xkb_compose_table_new_from_locale(
                    ctx_cast,
                    locale_cstr.as_ptr(),
                    compile_compose::xkbcommon_compose_h::XKB_COMPOSE_COMPILE_NO_FLAGS,
                );

            if compose_table.is_null() {
                // No compose table available for this locale
                return None;
            }

            Some(compose_table)
        }));

        let compose_table = match result {
            Ok(Some(table)) => table,
            _ => return composer, // Failed or null, return empty composer
        };

        // Cast compose table to the type expected by compose_iter
        let compose_table_iter = compose_table as *mut compose_iter::xkb_compose_table;

        // Define callback data structure to collect compose sequences
        struct CallbackData {
            composer: *mut ListComposer,
        }

        // Define callback function for compose table iteration
        unsafe extern "C" fn collect_sequences(
            entry: *mut compose_iter::xkb_compose_table_entry,
            data: *mut ::core::ffi::c_void,
        ) {
            // Safety checks
            if entry.is_null() || data.is_null() {
                return;
            }

            let callback_data = data as *mut CallbackData;
            if (*callback_data).composer.is_null() {
                return;
            }
            let composer = &mut *(*callback_data).composer;

            let entry_ref = &*entry;

            // Safety check for sequence
            if entry_ref.sequence.is_null() || entry_ref.sequence_length == 0 {
                return;
            }

            // Build sequence of tokens from keysyms
            let mut tokens = Vec::new();
            for i in 0..entry_ref.sequence_length {
                let keysym = *entry_ref.sequence.offset(i as isize);

                // Convert keysym to UTF-32
                let utf32 = keysym_utf::xkb_keysym_to_utf32(keysym);

                if utf32 != 0 {
                    if let Some(ch) = char::from_u32(utf32) {
                        tokens.push(crate::composer::Token::Char(ch));
                    }
                }
            }

            // Skip if we couldn't convert any keysyms
            if tokens.is_empty() {
                return;
            }

            // Get the output character from UTF-8 string
            if !entry_ref.utf8.is_null() {
                if let Ok(utf8_str) = std::ffi::CStr::from_ptr(entry_ref.utf8).to_str() {
                    if let Some(out_char) = utf8_str.chars().next() {
                        // Insert this sequence into the composer trie
                        composer.insert(&tokens, out_char);
                    }
                }
            }
        }

        // Prepare callback data
        let mut callback_data = CallbackData {
            composer: &mut composer as *mut ListComposer,
        };

        // Try to iterate through compose sequences, catching any errors
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compose_iter::xkb_compose_table_for_each(
                compose_table_iter,
                Some(collect_sequences),
                &mut callback_data as *mut CallbackData as *mut ::core::ffi::c_void,
            );
        }));

        // Clean up compose table
        compile_compose::xkbcommon_compose_h::xkb_compose_table_unref(compose_table);

        composer
    }
}

/// Create a new WKB instance from a keymap string
pub fn new_from_string(_string: String) -> WKB<ListComposer> {
    unimplemented!("new_from_string using rust-xkb is not yet implemented")
}
