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
        super::keymap::xkbcommon_h::xkb_rule_names,
        RuleNamesCStrings,
    ) {
        let rules_c = CString::new(self.rules.as_str()).unwrap_or_default();
        let model_c = CString::new(self.model.as_str()).unwrap_or_default();
        let layout_c = CString::new(self.layout.as_str()).unwrap_or_default();
        let variant_c = CString::new(self.variant.as_str()).unwrap_or_default();
        let options_c = CString::new(self.options.as_str()).unwrap_or_default();

        let c_struct = super::keymap::xkbcommon_h::xkb_rule_names {
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
            use super::context::xkbcommon_h::XKB_CONTEXT_NO_FLAGS;
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
            use super::keymap::xkbcommon_h::XKB_KEYMAP_COMPILE_NO_FLAGS;

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
            use super::keymap::xkbcommon_h::{
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
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            super::state::xkb_state_unref(self.ptr);
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
