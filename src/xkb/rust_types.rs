//! Rust-native wrapper types for XKB FFI structures
//!
//! These types use String instead of *const c_char for safer, more idiomatic Rust code.
//! They provide conversion methods to/from the C FFI types.

use std::ffi::CString;

/// Rust-native version of xkb_rule_names
#[derive(Debug, Clone, Default)]
pub struct RuleNames {
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: String,
    pub options: String,
}

impl RuleNames {
    /// Create new RuleNames with all fields set to given values
    pub fn new(
        rules: String,
        model: String,
        layout: String,
        variant: String,
        options: String,
    ) -> Self {
        Self {
            rules,
            model,
            layout,
            variant,
            options,
        }
    }

    /// Create RuleNames for evdev with given layout and variant
    pub fn evdev(layout: String, variant: Option<String>) -> Self {
        Self {
            rules: "evdev".to_string(),
            model: "".to_string(),
            layout,
            variant: variant.unwrap_or_default(),
            options: "".to_string(),
        }
    }

    /// Convert to C FFI structure compatible with keymap module
    ///
    /// Returns tuple of (C struct, CStrings that must be kept alive)
    /// The CStrings must outlive the C struct since it contains pointers to them.
    pub fn to_c_keymap(
        &self,
    ) -> (
        crate::xkb::shared_types::xkb_rule_names,
        RuleNamesCStrings,
    ) {
        let rules_c = CString::new(self.rules.as_str()).unwrap_or_default();
        let model_c = CString::new(self.model.as_str()).unwrap_or_default();
        let layout_c = CString::new(self.layout.as_str()).unwrap_or_default();
        let variant_c = CString::new(self.variant.as_str()).unwrap_or_default();
        let options_c = CString::new(self.options.as_str()).unwrap_or_default();

        let c_struct = crate::xkb::shared_types::xkb_rule_names {
            rules: rules_c.as_ptr(),
            model: model_c.as_ptr(),
            layout: layout_c.as_ptr(),
            variant: variant_c.as_ptr(),
            options: options_c.as_ptr(),
        };

        let strings = RuleNamesCStrings {
            rules: rules_c,
            model: model_c,
            layout: layout_c,
            variant: variant_c,
            options: options_c,
        };

        (c_struct, strings)
    }
}

/// Holder for CStrings that back the C FFI structure
/// Must be kept alive as long as the C struct is in use
pub struct RuleNamesCStrings {
    pub rules: CString,
    pub model: CString,
    pub layout: CString,
    pub variant: CString,
    pub options: CString,
}

// ============================================================================
// Safe RAII Wrappers for XKB FFI Types
// ============================================================================

/// Safe wrapper around xkb_context with automatic cleanup
pub struct Context {
    ptr: *mut super::context::context_h::xkb_context,
}

impl Context {
    /// Create a new XKB context
    pub fn new() -> Option<Self> {
        unsafe {
            use crate::xkb::shared_types::XKB_CONTEXT_NO_FLAGS;
            let ptr = super::context::xkb_context_new(XKB_CONTEXT_NO_FLAGS);
            if ptr.is_null() {
                None
            } else {
                Some(Context { ptr })
            }
        }
    }

    /// Get raw pointer (for FFI calls)
    pub fn as_ptr(&self) -> *mut super::context::context_h::xkb_context {
        self.ptr
    }

    /// Create a keymap from RMLVO names
    pub fn keymap_from_names(&self, rules: &RuleNames) -> Option<Keymap> {
        unsafe {
            use crate::xkb::shared_types::XKB_KEYMAP_COMPILE_NO_FLAGS;

            let (rmlvo_c, _strings) = rules.to_c_keymap();
            let keymap_ptr = super::keymap::xkb_keymap_new_from_names(
                self.ptr as *mut _,
                &rmlvo_c as *const _,
                XKB_KEYMAP_COMPILE_NO_FLAGS,
            );

            if keymap_ptr.is_null() {
                None
            } else {
                Some(Keymap { ptr: keymap_ptr })
            }
        }
    }

    /// Create a keymap from a keymap string
    pub fn keymap_from_string(&self, keymap_str: &str) -> Option<Keymap> {
        unsafe {
            use crate::xkb::shared_types::{
                XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_FORMAT_TEXT_V1,
            };

            let keymap_cstr = CString::new(keymap_str).ok()?;
            let keymap_ptr = super::keymap::xkb_keymap_new_from_string(
                self.ptr as *mut _,
                keymap_cstr.as_ptr(),
                XKB_KEYMAP_FORMAT_TEXT_V1,
                XKB_KEYMAP_COMPILE_NO_FLAGS,
            );

            if keymap_ptr.is_null() {
                None
            } else {
                Some(Keymap { ptr: keymap_ptr })
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            super::context::xkb_context_unref(self.ptr);
        }
    }
}

/// Safe wrapper around xkb_keymap with automatic cleanup
pub struct Keymap {
    ptr: *mut super::keymap::keymap_h::xkb_keymap,
}

impl Keymap {
    /// Get raw pointer (for FFI calls)
    pub fn as_ptr(&self) -> *mut super::keymap::keymap_h::xkb_keymap {
        self.ptr
    }

    /// Get minimum keycode
    pub fn min_keycode(&self) -> u32 {
        unsafe { super::keymap::xkb_keymap_min_keycode(self.ptr) }
    }

    /// Get maximum keycode
    pub fn max_keycode(&self) -> u32 {
        unsafe { super::keymap::xkb_keymap_max_keycode(self.ptr) }
    }

    /// Get number of levels for a key
    pub fn num_levels_for_key(&self, keycode: u32, layout: u32) -> u32 {
        unsafe { super::keymap::xkb_keymap_num_levels_for_key(self.ptr, keycode, layout) }
    }

    /// Get keysyms for a key at a specific level
    pub fn key_get_syms_by_level(&self, keycode: u32, layout: u32, level: u32) -> &[u32] {
        unsafe {
            let mut syms_ptr: *const u32 = std::ptr::null();
            let num_syms = super::keymap::xkb_keymap_key_get_syms_by_level(
                self.ptr,
                keycode,
                layout,
                level,
                &mut syms_ptr as *mut _,
            );

            if num_syms > 0 && !syms_ptr.is_null() {
                std::slice::from_raw_parts(syms_ptr, num_syms as usize)
            } else {
                &[]
            }
        }
    }

    /// Get number of modifiers in the keymap
    pub fn num_mods(&self) -> u32 {
        unsafe { super::keymap::xkb_keymap_num_mods(self.ptr) }
    }

    /// Get modifier name by index
    pub fn mod_get_name(&self, idx: u32) -> Option<String> {
        unsafe {
            let name_ptr = super::keymap::xkb_keymap_mod_get_name(self.ptr, idx);
            if name_ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Get modifier mask by name
    pub fn mod_get_mask(&self, name: &str) -> u32 {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).unwrap_or_default();
            super::keymap::xkb_keymap_mod_get_mask(self.ptr, name_cstr.as_ptr())
        }
    }

    /// Check if a key can repeat
    pub fn key_repeats(&self, keycode: u32) -> bool {
        unsafe { super::keymap::xkb_keymap_key_repeats(self.ptr, keycode) != 0 }
    }

    /// Get modifier maps for a key (returns (modmap, vmodmap) or None if key doesn't exist)
    pub fn key_get_mods(&self, keycode: u32) -> Option<(u32, u32)> {
        unsafe {
            let key = super::keymap::XkbKey(self.ptr, keycode);
            if key.is_null() {
                None
            } else {
                Some(((*key).modmap, (*key).vmodmap))
            }
        }
    }

    /// Iterate over all keycodes in the keymap
    ///
    /// Returns an iterator that yields (keycode, evdev_code) pairs.
    /// evdev_code is keycode - 8 (the standard offset for evdev)
    pub fn keycodes(&self) -> KeycodeIter {
        KeycodeIter {
            current: self.min_keycode(),
            max: self.max_keycode(),
            evdev_offset: 8,
        }
    }

    /// Get keycode range as a Range
    pub fn keycode_range(&self) -> std::ops::RangeInclusive<u32> {
        self.min_keycode()..=self.max_keycode()
    }

    /// Convert keysym to character using the keysym_utf module
    pub fn keysym_to_char(keysym: u32) -> Option<char> {
        super::keysym_utf::keysym_to_char(keysym)
    }

    /// Create a new state for this keymap
    pub fn new_state(&self) -> Option<State> {
        unsafe {
            let state_ptr =
                super::state::xkb_state_new(self.ptr as *mut super::state::keymap_h::xkb_keymap);
            if state_ptr.is_null() {
                None
            } else {
                Some(State { ptr: state_ptr })
            }
        }
    }

    /// Get number of layouts in the keymap
    pub fn num_layouts(&self) -> u32 {
        unsafe { super::keymap::xkb_keymap_num_layouts(self.ptr) }
    }

    /// Get layout name by index
    pub fn layout_get_name(&self, idx: u32) -> Option<String> {
        unsafe {
            let name_ptr = super::keymap::xkb_keymap_layout_get_name(self.ptr, idx);
            if name_ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Get layout index by name
    pub fn layout_get_index(&self, name: &str) -> Option<u32> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).ok()?;
            let idx = super::keymap::xkb_keymap_layout_get_index(self.ptr, name_cstr.as_ptr());
            if idx == u32::MAX {
                None
            } else {
                Some(idx)
            }
        }
    }

    /// Get number of LEDs in the keymap
    pub fn num_leds(&self) -> u32 {
        unsafe { super::keymap::xkb_keymap_num_leds(self.ptr) }
    }

    /// Get LED name by index
    pub fn led_get_name(&self, idx: u32) -> Option<String> {
        unsafe {
            let name_ptr = super::keymap::xkb_keymap_led_get_name(self.ptr, idx);
            if name_ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Get LED index by name
    pub fn led_get_index(&self, name: &str) -> Option<u32> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).ok()?;
            let idx = super::keymap::xkb_keymap_led_get_index(self.ptr, name_cstr.as_ptr());
            if idx == u32::MAX {
                None
            } else {
                Some(idx)
            }
        }
    }

    /// Get key name by keycode
    pub fn key_get_name(&self, keycode: u32) -> Option<String> {
        unsafe {
            let name_ptr = super::keymap::xkb_keymap_key_get_name(self.ptr, keycode);
            if name_ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Get keycode by key name
    pub fn key_by_name(&self, name: &str) -> Option<u32> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).ok()?;
            let keycode = super::keymap::xkb_keymap_key_by_name(self.ptr, name_cstr.as_ptr());
            if keycode == 0 {
                None
            } else {
                Some(keycode)
            }
        }
    }

    /// Get number of layouts for a specific key
    pub fn num_layouts_for_key(&self, keycode: u32) -> u32 {
        unsafe { super::keymap::xkb_keymap_num_layouts_for_key(self.ptr, keycode) }
    }

    /// Get modifier index by name
    pub fn mod_get_index(&self, name: &str) -> Option<u32> {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).ok()?;
            let idx = super::keymap::xkb_keymap_mod_get_index(self.ptr, name_cstr.as_ptr());
            if idx == u32::MAX {
                None
            } else {
                Some(idx)
            }
        }
    }

    /// Get all layout names as a Vec
    pub fn get_all_layouts(&self) -> Vec<String> {
        let num_layouts = self.num_layouts();
        (0..num_layouts)
            .filter_map(|idx| self.layout_get_name(idx))
            .collect()
    }

    /// Get all modifier names as a Vec
    pub fn get_all_mods(&self) -> Vec<String> {
        let num_mods = self.num_mods();
        (0..num_mods)
            .filter_map(|idx| self.mod_get_name(idx))
            .collect()
    }

    /// Get all LED names as a Vec
    pub fn get_all_leds(&self) -> Vec<String> {
        let num_leds = self.num_leds();
        (0..num_leds)
            .filter_map(|idx| self.led_get_name(idx))
            .collect()
    }
}

/// Iterator over keycode ranges in a keymap
pub struct KeycodeIter {
    current: u32,
    max: u32,
    evdev_offset: u32,
}

impl Iterator for KeycodeIter {
    type Item = (u32, u32); // (xkb_keycode, evdev_code)

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.max {
            let keycode = self.current;
            let evdev_code = keycode - self.evdev_offset;
            self.current += 1;
            Some((keycode, evdev_code))
        } else {
            None
        }
    }
}

impl ExactSizeIterator for KeycodeIter {
    fn len(&self) -> usize {
        if self.current <= self.max {
            (self.max - self.current + 1) as usize
        } else {
            0
        }
    }
}

impl Drop for Keymap {
    fn drop(&mut self) {
        unsafe {
            super::keymap::xkb_keymap_unref(self.ptr);
        }
    }
}

/// Safe wrapper around xkb_state with automatic cleanup
pub struct State {
    ptr: *mut super::state::xkb_state,
}

impl State {
    /// Get raw pointer (for FFI calls)
    pub fn as_ptr(&self) -> *mut super::state::xkb_state {
        self.ptr
    }

    /// Update key state (press or release)
    pub fn update_key(&mut self, keycode: u32, direction: super::common::xkb_key_direction) {
        unsafe {
            super::state::xkb_state_update_key(self.ptr, keycode, direction);
        }
    }

    /// Get UTF-8 string for a key
    pub fn key_get_utf8(&self, keycode: u32) -> String {
        unsafe {
            let mut buffer: [u8; 32] = [0; 32];
            let size = super::state::xkb_state_key_get_utf8(
                self.ptr,
                keycode,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len(),
            );

            if size > 0 && (size as usize) < buffer.len() {
                String::from_utf8_lossy(&buffer[..size as usize]).into_owned()
            } else {
                String::new()
            }
        }
    }

    /// Press a key (convenience wrapper for update_key with KEY_DOWN)
    pub fn key_down(&mut self, keycode: u32) {
        self.update_key(keycode, super::common::XKB_KEY_DOWN);
    }

    /// Release a key (convenience wrapper for update_key with KEY_UP)
    pub fn key_up(&mut self, keycode: u32) {
        self.update_key(keycode, super::common::XKB_KEY_UP);
    }

    /// Get the character for a key as Option<char>
    pub fn key_get_char(&self, keycode: u32) -> Option<char> {
        self.key_get_utf8(keycode).chars().next()
    }

    /// Get keysym for a key in the current state
    pub fn key_get_one_sym(&self, keycode: u32) -> u32 {
        unsafe { super::state::xkb_state_key_get_one_sym(self.ptr, keycode) }
    }

    /// Get all keysyms for a key in the current state
    pub fn key_get_syms(&self, keycode: u32) -> &[u32] {
        unsafe {
            let mut syms_ptr: *const u32 = std::ptr::null();
            let num_syms =
                super::state::xkb_state_key_get_syms(self.ptr, keycode, &mut syms_ptr as *mut _);

            if num_syms > 0 && !syms_ptr.is_null() {
                std::slice::from_raw_parts(syms_ptr, num_syms as usize)
            } else {
                &[]
            }
        }
    }

    /// Get active layout index
    pub fn serialize_layout(&self, component: u32) -> u32 {
        unsafe { super::state::xkb_state_serialize_layout(self.ptr, component) }
    }

    /// Get active modifiers mask
    pub fn serialize_mods(&self, component: u32) -> u32 {
        unsafe { super::state::xkb_state_serialize_mods(self.ptr, component) }
    }

    /// Check if a modifier is active
    pub fn mod_name_is_active(&self, name: &str, state_type: u32) -> bool {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).unwrap_or_default();
            super::state::xkb_state_mod_name_is_active(self.ptr, name_cstr.as_ptr(), state_type) > 0
        }
    }

    /// Check if a modifier index is active
    pub fn mod_index_is_active(&self, idx: u32, state_type: u32) -> bool {
        unsafe { super::state::xkb_state_mod_index_is_active(self.ptr, idx, state_type) > 0 }
    }

    /// Check if a layout is active
    pub fn layout_name_is_active(&self, name: &str, state_type: u32) -> bool {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).unwrap_or_default();
            super::state::xkb_state_layout_name_is_active(self.ptr, name_cstr.as_ptr(), state_type)
                > 0
        }
    }

    /// Check if a layout index is active
    pub fn layout_index_is_active(&self, idx: u32, state_type: u32) -> bool {
        unsafe { super::state::xkb_state_layout_index_is_active(self.ptr, idx, state_type) > 0 }
    }

    /// Check if a LED is active
    pub fn led_name_is_active(&self, name: &str) -> bool {
        unsafe {
            let name_cstr = std::ffi::CString::new(name).unwrap_or_default();
            super::state::xkb_state_led_name_is_active(self.ptr, name_cstr.as_ptr()) > 0
        }
    }

    /// Check if a LED index is active
    pub fn led_index_is_active(&self, idx: u32) -> bool {
        unsafe { super::state::xkb_state_led_index_is_active(self.ptr, idx) > 0 }
    }

    /// Update state from modifier/layout masks (e.g., from Wayland compositor)
    pub fn update_mask(
        &mut self,
        depressed_mods: u32,
        latched_mods: u32,
        locked_mods: u32,
        depressed_layout: u32,
        latched_layout: u32,
        locked_layout: u32,
    ) -> u32 {
        unsafe {
            super::state::xkb_state_update_mask(
                self.ptr,
                depressed_mods,
                latched_mods,
                locked_mods,
                depressed_layout,
                latched_layout,
                locked_layout,
            )
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            super::state::xkb_state_unref(self.ptr);
        }
    }
}

// ============================================================================
// Registry (rxkb) Wrappers for Layout Enumeration
// ============================================================================

/// Safe wrapper around rxkb_context for keyboard layout registry
pub struct RxkbContext {
    ptr: *mut super::registry_list::xkbregistry_h::rxkb_context,
}

impl RxkbContext {
    /// Create a new registry context
    pub fn new() -> Option<Self> {
        unsafe {
            let ptr = super::registry_list::xkbregistry_h::rxkb_context_new(
                super::registry_list::xkbregistry_h::RXKB_CONTEXT_NO_FLAGS,
            );
            if ptr.is_null() {
                None
            } else {
                Some(RxkbContext { ptr })
            }
        }
    }

    /// Load default registry paths
    pub fn include_path_append_default(&self) {
        unsafe {
            super::registry_list::xkbregistry_h::rxkb_context_include_path_append_default(self.ptr);
        }
    }

    /// Parse the registry for the given ruleset (typically "evdev")
    pub fn parse(&self, ruleset: &str) -> bool {
        unsafe {
            let ruleset_cstr = std::ffi::CString::new(ruleset).unwrap_or_default();
            super::registry_list::xkbregistry_h::rxkb_context_parse(self.ptr, ruleset_cstr.as_ptr())
        }
    }

    /// Get the first layout in the registry
    pub fn layout_first(&self) -> Option<RxkbLayout> {
        unsafe {
            let ptr = super::registry_list::xkbregistry_h::rxkb_layout_first(self.ptr);
            if ptr.is_null() {
                None
            } else {
                Some(RxkbLayout { ptr })
            }
        }
    }
}

impl Drop for RxkbContext {
    fn drop(&mut self) {
        unsafe {
            super::registry_list::xkbregistry_h::rxkb_context_unref(self.ptr);
        }
    }
}

/// Safe wrapper around rxkb_layout for keyboard layout information
pub struct RxkbLayout {
    ptr: *mut super::registry_list::xkbregistry_h::rxkb_layout,
}

impl RxkbLayout {
    /// Get the layout name (e.g., "us", "de", "fr")
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            let name_ptr = super::registry_list::xkbregistry_h::rxkb_layout_get_name(self.ptr);
            if name_ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Get the layout variant (e.g., "dvorak", "colemak"), returns None for base layout
    pub fn get_variant(&self) -> Option<String> {
        unsafe {
            let variant_ptr =
                super::registry_list::xkbregistry_h::rxkb_layout_get_variant(self.ptr);
            if variant_ptr.is_null() {
                None
            } else {
                let variant = std::ffi::CStr::from_ptr(variant_ptr)
                    .to_string_lossy()
                    .to_string();
                if variant.is_empty() {
                    None
                } else {
                    Some(variant)
                }
            }
        }
    }

    /// Get the next layout in the registry
    pub fn next(&self) -> Option<RxkbLayout> {
        unsafe {
            let ptr = super::registry_list::xkbregistry_h::rxkb_layout_next(self.ptr);
            if ptr.is_null() {
                None
            } else {
                Some(RxkbLayout { ptr })
            }
        }
    }
}

// ============================================================================
// Compose Table Wrappers for Dead Key Sequences
// ============================================================================

/// Safe wrapper around xkb_compose_table for dead key composition
///
/// Note: The underlying type is opaque to avoid private struct imports
pub struct ComposeTable {
    ptr: *mut ::core::ffi::c_void,
}

impl ComposeTable {
    /// Create a new compose table from locale
    pub fn new_from_locale(ctx: &Context, locale: &str) -> Option<Self> {
        unsafe {
            let locale_cstr = std::ffi::CString::new(locale).ok()?;
            let ctx_cast = ctx.as_ptr() as *mut super::compile_compose::context_h::xkb_context;

            let ptr =
                super::compile_compose::xkbcommon_compose_h::xkb_compose_table_new_from_locale(
                    ctx_cast,
                    locale_cstr.as_ptr(),
                    super::compile_compose::xkbcommon_compose_h::XKB_COMPOSE_COMPILE_NO_FLAGS,
                );

            if ptr.is_null() {
                None
            } else {
                Some(ComposeTable {
                    ptr: ptr as *mut ::core::ffi::c_void,
                })
            }
        }
    }

    /// Get raw pointer for FFI calls (needed for compose iteration)
    pub fn as_ptr(&self) -> *mut ::core::ffi::c_void {
        self.ptr
    }
}

impl Drop for ComposeTable {
    fn drop(&mut self) {
        unsafe {
            super::compile_compose::xkbcommon_compose_h::xkb_compose_table_unref(
                self.ptr as *mut _,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_names_evdev() {
        let names = RuleNames::evdev("us".to_string(), None);
        assert_eq!(names.rules, "evdev");
        assert_eq!(names.layout, "us");
        assert_eq!(names.variant, "");
    }

    #[test]
    fn test_rule_names_to_c() {
        let names = RuleNames::evdev("us".to_string(), Some("dvorak".to_string()));
        let (c_struct, _strings) = names.to_c_keymap();

        unsafe {
            use std::ffi::CStr;
            assert_eq!(CStr::from_ptr(c_struct.rules).to_str().unwrap(), "evdev");
            assert_eq!(CStr::from_ptr(c_struct.layout).to_str().unwrap(), "us");
            assert_eq!(CStr::from_ptr(c_struct.variant).to_str().unwrap(), "dvorak");
        }
    }

    #[test]
    fn test_context_new() {
        let ctx = Context::new();
        assert!(ctx.is_some());
    }

    #[test]
    fn test_context_keymap() {
        let ctx = Context::new().expect("Failed to create context");
        let rules = RuleNames::evdev("us".to_string(), None);
        let keymap = ctx.keymap_from_names(&rules);
        assert!(keymap.is_some());
    }

    #[test]
    fn test_keymap_keycodes() {
        let ctx = Context::new().expect("Failed to create context");
        let rules = RuleNames::evdev("us".to_string(), None);
        let keymap = ctx
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        let min = keymap.min_keycode();
        let max = keymap.max_keycode();
        assert!(min < max);
        assert!(min >= 8); // evdev starts at 8
    }
}
use crate::xkb::shared_types::*;
