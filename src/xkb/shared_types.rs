//! Shared type definitions used across multiple modules.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

// ── xkbcommon public types ───────────────────────────────────────────

pub(crate) const XKB_LOG_LEVEL_DEBUG: u32 = 50;
pub(crate) const XKB_LOG_LEVEL_INFO: u32 = 40;
pub(crate) const XKB_LOG_LEVEL_WARNING: u32 = 30;
pub(crate) const XKB_LOG_LEVEL_ERROR: u32 = 20;
pub(crate) const XKB_LOG_LEVEL_CRITICAL: u32 = 10;

pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: u32 = 2;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: u32 = 1;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_WRAP: u32 = 0;

pub(crate) const XKB_STATE_CONTROLS: u32 = 512;
pub(crate) const XKB_STATE_LEDS: u32 = 256;
pub(crate) const XKB_STATE_LAYOUT_EFFECTIVE: u32 = 128;
pub(crate) const XKB_STATE_LAYOUT_LOCKED: u32 = 64;
pub(crate) const XKB_STATE_LAYOUT_LATCHED: u32 = 32;
pub(crate) const XKB_STATE_LAYOUT_DEPRESSED: u32 = 16;
pub(crate) const XKB_STATE_MODS_EFFECTIVE: u32 = 8;
pub(crate) const XKB_STATE_MODS_LOCKED: u32 = 4;
pub(crate) const XKB_STATE_MODS_LATCHED: u32 = 2;
pub(crate) const XKB_STATE_MODS_DEPRESSED: u32 = 1;

pub(crate) const XKB_KEYMAP_FORMAT_TEXT_V2: u32 = 2;
pub(crate) const XKB_KEYMAP_FORMAT_TEXT_V1: u32 = 1;

pub(crate) const XKB_KEYMAP_COMPILE_STRICT_MODE: u32 = 1;
pub(crate) const XKB_KEYMAP_COMPILE_NO_FLAGS: u32 = 0;

pub(crate) const XKB_LAYOUT_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_MOD_INVALID: u32 = 0xffffffff;

// ── xkb_rule_names ──────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub(crate) struct xkb_rule_names {
    pub(crate) rules: std::ffi::CString,
    pub(crate) model: std::ffi::CString,
    pub(crate) layout: std::ffi::CString,
    pub(crate) variant: std::ffi::CString,
    pub(crate) options: std::ffi::CString,
}

impl Default for xkb_rule_names {
    fn default() -> Self {
        Self {
            rules: std::ffi::CString::new("").unwrap(),
            model: std::ffi::CString::new("").unwrap(),
            layout: std::ffi::CString::new("").unwrap(),
            variant: std::ffi::CString::new("").unwrap(),
            options: std::ffi::CString::new("").unwrap(),
        }
    }
}

// ── Opaque types ────────────────────────────────────────────────────

/// Atom table - string interning system
pub(crate) struct atom_table {
    /// Hash index for O(1) lookups (open addressing, linear probing)
    index: Vec<u32>,
    /// Interned strings. Index 0 is None (XKB_ATOM_NONE).
    strings: Vec<Option<String>>,
}

impl Clone for atom_table {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            strings: self.strings.clone(),
        }
    }
}

impl std::fmt::Debug for atom_table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("atom_table").finish()
    }
}

/// FNV-1a hash function for byte slices
fn hash_buf(bytes: &[u8]) -> u32 {
    let len = bytes.len();
    let mut hash: u32 = 2166136261;
    for i in 0..len.div_ceil(2) {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(0x1000193);
        hash ^= bytes[len - 1 - i] as u32;
        hash = hash.wrapping_mul(0x1000193);
    }
    hash
}

/// Create new atom table with pre-allocated capacity
pub(crate) fn atom_table_new() -> atom_table {
    atom_table {
        index: vec![0; 1024],
        strings: {
            let mut v = Vec::with_capacity(512);
            v.push(None);
            v
        },
    }
}

/// Get number of atoms in table
pub(crate) fn atom_table_size(table: &atom_table) -> u32 {
    table.strings.len() as u32
}

// ── xkb_context ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub(crate) struct xkb_context {
    pub(crate) refcnt: i32,
    pub(crate) log_level: u32,
    pub(crate) log_verbosity: i32,
    pub(crate) names_dflt: xkb_rule_names,
    pub(crate) includes: Vec<String>,
    pub(crate) failed_includes: Vec<String>,
    pub(crate) atom_table: atom_table,
    pub(crate) use_environment_names: bool,
    pub(crate) use_secure_getenv: bool,
    pub(crate) pending_default_includes: bool,
}

thread_local! {
    /// Thread-local file cache shared across all xkb_context instances.
    /// Survives context clones and keymap compilations within the same thread.
    static FILE_CACHE: RefCell<HashMap<String, Arc<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// Read a file from the thread-local cache, or read from disk and cache it.
pub(crate) fn read_file_cached(path: &str) -> Option<Arc<Vec<u8>>> {
    FILE_CACHE
        .with(|cache| {
            let cache = cache.borrow();
            cache.get(path).cloned()
        })
        .or_else(|| {
            use std::io::Read;
            let mut file = std::fs::File::open(path).ok()?;
            let mut data = Vec::new();
            file.read_to_end(&mut data).ok()?;
            let arc = Arc::new(data);
            FILE_CACHE.with(|cache| {
                cache.borrow_mut().insert(path.to_string(), arc.clone());
            });
            Some(arc)
        })
}

// ── keymap_h types (from keymap_priv.rs) ────────────────────────────

pub(crate) struct xkb_keymap {
    pub(crate) ctx: xkb_context,
    pub(crate) flags: u32,
    pub(crate) format: u32,
    pub(crate) num_leds: u32,
    pub(crate) leds: [xkb_led; 32],
    pub(crate) min_key_code: u32,
    pub(crate) max_key_code: u32,
    pub(crate) num_keys: u32,
    pub(crate) num_keys_low: u32,
    pub(crate) keys: Vec<xkb_key>,
    pub(crate) key_names: Vec<KeycodeMatch>,
    pub(crate) key_aliases: Vec<xkb_key_alias>,
    pub(crate) types: Vec<xkb_key_type>,
    pub(crate) sym_interprets: Vec<xkb_sym_interpret>,
    pub(crate) mods: xkb_mod_set,
    pub(crate) canonical_state_mask: u32,
    pub(crate) redirect_key_auto: u32,
    pub(crate) num_groups: u32,
    pub(crate) group_names: Vec<u32>,
    pub(crate) keycodes_section_name: String,
    pub(crate) symbols_section_name: String,
    pub(crate) types_section_name: String,
    pub(crate) compat_section_name: String,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct xkb_mod_set {
    pub(crate) mods: [xkb_mod; 32],
    pub(crate) num_mods: u32,
    pub(crate) explicit_vmods: u32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct xkb_mod {
    pub(crate) name: u32,
    pub(crate) type_0: u32,
    pub(crate) mapping: u32,
}

pub(crate) const MOD_BOTH: u32 = 3;
pub(crate) const MOD_VIRT: u32 = 2;
pub(crate) const MOD_REAL: u32 = 1;

#[derive(Clone)]
pub(crate) struct xkb_sym_interpret {
    pub(crate) sym: u32,
    pub(crate) match_0: u32,
    pub(crate) mods: u32,
    pub(crate) virtual_mod: u32,
    pub(crate) level_one_only: bool,
    pub(crate) repeat: bool,
    pub(crate) required: bool,
    pub(crate) num_actions: u16,
    pub(crate) action: xkb_action,
    pub(crate) actions: Vec<xkb_action>,
}

#[derive(Copy, Clone, Default)]
pub(crate) enum xkb_action {
    #[default]
    None,
    Void,
    ModSet(xkb_mod_action),
    ModLatch(xkb_mod_action),
    ModLock(xkb_mod_action),
    GroupSet(xkb_group_action),
    GroupLatch(xkb_group_action),
    GroupLock(xkb_group_action),
    PtrMove(xkb_pointer_action),
    PtrButton(xkb_pointer_button_action),
    PtrLock(xkb_pointer_button_action),
    PtrDefault(xkb_pointer_default_action),
    Terminate,
    SwitchVt(xkb_switch_screen_action),
    CtrlSet(xkb_controls_action),
    CtrlLock(xkb_controls_action),
    RedirectKey(xkb_redirect_key_action),
    UnsupportedLegacy,
    Unknown,
    Private(xkb_private_action),
    Internal(xkb_internal_action),
}

impl xkb_action {
    pub(crate) fn action_type(&self) -> u32 {
        match self {
            xkb_action::None => ACTION_TYPE_NONE,
            xkb_action::Void => ACTION_TYPE_VOID,
            xkb_action::ModSet(_) => ACTION_TYPE_MOD_SET,
            xkb_action::ModLatch(_) => ACTION_TYPE_MOD_LATCH,
            xkb_action::ModLock(_) => ACTION_TYPE_MOD_LOCK,
            xkb_action::GroupSet(_) => ACTION_TYPE_GROUP_SET,
            xkb_action::GroupLatch(_) => ACTION_TYPE_GROUP_LATCH,
            xkb_action::GroupLock(_) => ACTION_TYPE_GROUP_LOCK,
            xkb_action::PtrMove(_) => ACTION_TYPE_PTR_MOVE,
            xkb_action::PtrButton(_) => ACTION_TYPE_PTR_BUTTON,
            xkb_action::PtrLock(_) => ACTION_TYPE_PTR_LOCK,
            xkb_action::PtrDefault(_) => ACTION_TYPE_PTR_DEFAULT,
            xkb_action::Terminate => ACTION_TYPE_TERMINATE,
            xkb_action::SwitchVt(_) => ACTION_TYPE_SWITCH_VT,
            xkb_action::CtrlSet(_) => ACTION_TYPE_CTRL_SET,
            xkb_action::CtrlLock(_) => ACTION_TYPE_CTRL_LOCK,
            xkb_action::RedirectKey(_) => ACTION_TYPE_REDIRECT_KEY,
            xkb_action::UnsupportedLegacy => ACTION_TYPE_UNSUPPORTED_LEGACY,
            xkb_action::Unknown => ACTION_TYPE_UNKNOWN,
            xkb_action::Private(_) => ACTION_TYPE_PRIVATE,
            xkb_action::Internal(_) => ACTION_TYPE_INTERNAL,
        }
    }

    pub(crate) fn from_type(t: u32) -> Self {
        match t {
            ACTION_TYPE_NONE => xkb_action::None,
            ACTION_TYPE_VOID => xkb_action::Void,
            ACTION_TYPE_MOD_SET => xkb_action::ModSet(xkb_mod_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_MOD_LATCH => xkb_action::ModLatch(xkb_mod_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_MOD_LOCK => xkb_action::ModLock(xkb_mod_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_GROUP_SET => xkb_action::GroupSet(xkb_group_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_GROUP_LATCH => xkb_action::GroupLatch(xkb_group_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_GROUP_LOCK => xkb_action::GroupLock(xkb_group_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_PTR_MOVE => xkb_action::PtrMove(xkb_pointer_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_PTR_BUTTON => xkb_action::PtrButton(xkb_pointer_button_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_PTR_LOCK => xkb_action::PtrLock(xkb_pointer_button_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_PTR_DEFAULT => xkb_action::PtrDefault(xkb_pointer_default_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_TERMINATE => xkb_action::Terminate,
            ACTION_TYPE_SWITCH_VT => xkb_action::SwitchVt(xkb_switch_screen_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_CTRL_SET => xkb_action::CtrlSet(xkb_controls_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_CTRL_LOCK => xkb_action::CtrlLock(xkb_controls_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_REDIRECT_KEY => xkb_action::RedirectKey(xkb_redirect_key_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_UNSUPPORTED_LEGACY => xkb_action::UnsupportedLegacy,
            ACTION_TYPE_UNKNOWN => xkb_action::Unknown,
            ACTION_TYPE_PRIVATE => xkb_action::Private(xkb_private_action {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_INTERNAL => xkb_action::Internal(xkb_internal_action {
                type_0: t,
                ..Default::default()
            }),
            _ => xkb_action::None,
        }
    }

    pub(crate) fn set_none(&mut self) {
        *self = xkb_action::None;
    }

    pub(crate) fn as_mods(&self) -> &xkb_mod_action {
        match self {
            xkb_action::ModSet(m) | xkb_action::ModLatch(m) | xkb_action::ModLock(m) => m,
            _ => panic!("not a mod action"),
        }
    }
    pub(crate) fn as_mods_mut(&mut self) -> &mut xkb_mod_action {
        match self {
            xkb_action::ModSet(m) | xkb_action::ModLatch(m) | xkb_action::ModLock(m) => m,
            _ => panic!("not a mod action"),
        }
    }

    pub(crate) fn as_group(&self) -> &xkb_group_action {
        match self {
            xkb_action::GroupSet(g) | xkb_action::GroupLatch(g) | xkb_action::GroupLock(g) => g,
            _ => panic!("not a group action"),
        }
    }
    pub(crate) fn as_group_mut(&mut self) -> &mut xkb_group_action {
        match self {
            xkb_action::GroupSet(g) | xkb_action::GroupLatch(g) | xkb_action::GroupLock(g) => g,
            _ => panic!("not a group action"),
        }
    }

    pub fn as_ctrls(&self) -> &xkb_controls_action {
        match self {
            xkb_action::CtrlSet(c) | xkb_action::CtrlLock(c) => c,
            _ => panic!("not a ctrls action"),
        }
    }
    pub fn as_ctrls_mut(&mut self) -> &mut xkb_controls_action {
        match self {
            xkb_action::CtrlSet(c) | xkb_action::CtrlLock(c) => c,
            _ => panic!("not a ctrls action"),
        }
    }

    pub fn as_ptr(&self) -> &xkb_pointer_action {
        match self {
            xkb_action::PtrMove(p) => p,
            _ => panic!("not a ptr action"),
        }
    }
    pub fn as_ptr_mut(&mut self) -> &mut xkb_pointer_action {
        match self {
            xkb_action::PtrMove(p) => p,
            _ => panic!("not a ptr action"),
        }
    }

    pub fn as_btn(&self) -> &xkb_pointer_button_action {
        match self {
            xkb_action::PtrButton(b) | xkb_action::PtrLock(b) => b,
            _ => panic!("not a btn action"),
        }
    }
    pub fn as_btn_mut(&mut self) -> &mut xkb_pointer_button_action {
        match self {
            xkb_action::PtrButton(b) | xkb_action::PtrLock(b) => b,
            _ => panic!("not a btn action"),
        }
    }

    pub fn as_dflt(&self) -> &xkb_pointer_default_action {
        match self {
            xkb_action::PtrDefault(d) => d,
            _ => panic!("not a dflt action"),
        }
    }
    pub fn as_dflt_mut(&mut self) -> &mut xkb_pointer_default_action {
        match self {
            xkb_action::PtrDefault(d) => d,
            _ => panic!("not a dflt action"),
        }
    }

    pub fn as_screen(&self) -> &xkb_switch_screen_action {
        match self {
            xkb_action::SwitchVt(s) => s,
            _ => panic!("not a screen action"),
        }
    }
    pub fn as_screen_mut(&mut self) -> &mut xkb_switch_screen_action {
        match self {
            xkb_action::SwitchVt(s) => s,
            _ => panic!("not a screen action"),
        }
    }

    pub fn as_redirect(&self) -> &xkb_redirect_key_action {
        match self {
            xkb_action::RedirectKey(r) => r,
            _ => panic!("not a redirect action"),
        }
    }
    pub fn as_redirect_mut(&mut self) -> &mut xkb_redirect_key_action {
        match self {
            xkb_action::RedirectKey(r) => r,
            _ => panic!("not a redirect action"),
        }
    }

    pub fn as_priv(&self) -> &xkb_private_action {
        match self {
            xkb_action::Private(p) => p,
            _ => panic!("not a priv action"),
        }
    }
    pub fn as_priv_mut(&mut self) -> &mut xkb_private_action {
        match self {
            xkb_action::Private(p) => p,
            _ => panic!("not a priv action"),
        }
    }

    pub fn as_internal(&self) -> &xkb_internal_action {
        match self {
            xkb_action::Internal(i) => i,
            _ => panic!("not an internal action"),
        }
    }
    pub fn as_internal_mut(&mut self) -> &mut xkb_internal_action {
        match self {
            xkb_action::Internal(i) => i,
            _ => panic!("not an internal action"),
        }
    }

    pub fn flags(&self) -> xkb_action_flags {
        match self {
            xkb_action::ModSet(m) | xkb_action::ModLatch(m) | xkb_action::ModLock(m) => m.flags,
            xkb_action::GroupSet(g) | xkb_action::GroupLatch(g) | xkb_action::GroupLock(g) => {
                g.flags
            }
            xkb_action::CtrlSet(c) | xkb_action::CtrlLock(c) => c.flags,
            xkb_action::PtrMove(p) => p.flags,
            xkb_action::PtrButton(b) | xkb_action::PtrLock(b) => b.flags,
            xkb_action::PtrDefault(d) => d.flags,
            xkb_action::SwitchVt(s) => s.flags,
            xkb_action::RedirectKey(_) => 0,
            xkb_action::Internal(i) => i.flags,
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct xkb_internal_action {
    pub type_0: u32,
    pub flags: u32,
    pub clear_latched_mods: u32,
}

pub const INTERNAL_BREAKS_MOD_LATCH: u32 = 2;
pub const INTERNAL_BREAKS_GROUP_LATCH: u32 = 1;

pub const _ACTION_TYPE_NUM_ENTRIES: u32 = 21;
pub const ACTION_TYPE_INTERNAL: u32 = 20;
pub const ACTION_TYPE_PRIVATE: u32 = 19;
pub const ACTION_TYPE_UNKNOWN: u32 = 18;
pub const ACTION_TYPE_UNSUPPORTED_LEGACY: u32 = 17;
pub const ACTION_TYPE_REDIRECT_KEY: u32 = 16;
pub const ACTION_TYPE_CTRL_LOCK: u32 = 15;
pub const ACTION_TYPE_CTRL_SET: u32 = 14;
pub const ACTION_TYPE_SWITCH_VT: u32 = 13;
pub const ACTION_TYPE_TERMINATE: u32 = 12;
pub const ACTION_TYPE_PTR_DEFAULT: u32 = 11;
pub const ACTION_TYPE_PTR_LOCK: u32 = 10;
pub const ACTION_TYPE_PTR_BUTTON: u32 = 9;
pub const ACTION_TYPE_PTR_MOVE: u32 = 8;
pub const ACTION_TYPE_GROUP_LOCK: u32 = 7;
pub const ACTION_TYPE_GROUP_LATCH: u32 = 6;
pub const ACTION_TYPE_GROUP_SET: u32 = 5;
pub const ACTION_TYPE_MOD_LOCK: u32 = 4;
pub const ACTION_TYPE_MOD_LATCH: u32 = 3;
pub const ACTION_TYPE_MOD_SET: u32 = 2;
pub const ACTION_TYPE_VOID: u32 = 1;
pub const ACTION_TYPE_NONE: u32 = 0;

#[derive(Copy, Clone, Default)]
pub struct xkb_private_action {
    pub type_0: u32,
    pub data: [u8; 7],
}

#[derive(Copy, Clone, Default)]
pub struct xkb_redirect_key_action {
    pub type_0: u32,
    pub keycode: u32,
    pub affect: u32,
    pub mods: u32,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_pointer_button_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub count: u8,
    pub button: u8,
}

pub type xkb_action_flags = u32;
pub const ACTION_PENDING_COMPUTATION: xkb_action_flags = 8192;
pub const ACTION_LATCH_ON_PRESS: xkb_action_flags = 4096;
pub const ACTION_UNLOCK_ON_PRESS: xkb_action_flags = 2048;
pub const ACTION_LOCK_ON_RELEASE: xkb_action_flags = 1024;
pub const ACTION_SAME_SCREEN: xkb_action_flags = 512;
pub const ACTION_ACCEL: xkb_action_flags = 256;
pub const ACTION_ABSOLUTE_Y: xkb_action_flags = 128;
pub const ACTION_ABSOLUTE_X: xkb_action_flags = 64;
pub const ACTION_ABSOLUTE_SWITCH: xkb_action_flags = 32;
pub const ACTION_MODS_LOOKUP_MODMAP: xkb_action_flags = 16;
pub const ACTION_LOCK_NO_UNLOCK: xkb_action_flags = 8;
pub const ACTION_LOCK_NO_LOCK: xkb_action_flags = 4;
pub const ACTION_LATCH_TO_LOCK: xkb_action_flags = 2;
pub const ACTION_LOCK_CLEAR: xkb_action_flags = 1;

#[derive(Copy, Clone, Default)]
pub struct xkb_pointer_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_switch_screen_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub screen: i8,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_pointer_default_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub value: i8,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_controls_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub ctrls: xkb_action_controls,
}

pub type xkb_action_controls = u32;
pub const CONTROL_ALL_BOOLEAN: xkb_action_controls = 2088447;
pub const CONTROL_ALL_BOOLEAN_V1: xkb_action_controls = 2087943;
pub const CONTROL_ALL: xkb_action_controls = 2088959;
pub const CONTROL_ALL_V1: xkb_action_controls = 2088455;
pub const CONTROL_IGNORE_GROUP_LOCK: xkb_action_controls = 1048576;
pub const CONTROL_BELL: xkb_action_controls = 524288;
pub const CONTROL_AX_FEEDBACK: xkb_action_controls = 262144;
pub const CONTROL_AX_TIMEOUT: xkb_action_controls = 131072;
pub const CONTROL_AX: xkb_action_controls = 65536;
pub const CONTROL_MOUSE_KEYS_ACCEL: xkb_action_controls = 32768;
pub const CONTROL_MOUSE_KEYS: xkb_action_controls = 16384;
pub const CONTROL_DEBOUNCE: xkb_action_controls = 4096;
pub const CONTROL_SLOW: xkb_action_controls = 2048;
pub const CONTROL_REPEAT: xkb_action_controls = 1024;
pub const CONTROL_GROUPS_WRAP: xkb_action_controls = 512;
pub const CONTROL_OVERLAY8: xkb_action_controls = 256;
pub const CONTROL_OVERLAY7: xkb_action_controls = 128;
pub const CONTROL_OVERLAY6: xkb_action_controls = 64;
pub const CONTROL_OVERLAY5: xkb_action_controls = 32;
pub const CONTROL_OVERLAY4: xkb_action_controls = 16;
pub const CONTROL_OVERLAY3: xkb_action_controls = 8;
pub const CONTROL_OVERLAY2: xkb_action_controls = 4;
pub const CONTROL_OVERLAY1: xkb_action_controls = 2;
pub const CONTROL_STICKY_KEYS: xkb_action_controls = 1;

#[derive(Copy, Clone, Default)]
pub(crate) struct xkb_group_action {
    pub(crate) type_0: u32,
    pub(crate) flags: xkb_action_flags,
    pub(crate) group: i32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct xkb_mod_action {
    pub(crate) type_0: u32,
    pub(crate) flags: xkb_action_flags,
    pub(crate) mods: xkb_mods,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct xkb_mods {
    pub(crate) mods: u32,
    pub(crate) mask: u32,
}

pub const MATCH_EXACTLY: u32 = 4;
pub const MATCH_ALL: u32 = 3;
pub const MATCH_ANY: u32 = 2;
pub const MATCH_ANY_OR_NONE: u32 = 1;
pub const MATCH_NONE: u32 = 0;

#[derive(Clone)]
pub(crate) struct xkb_key_type {
    pub(crate) name: u32,
    pub(crate) mods: xkb_mods,
    pub(crate) required: bool,
    pub(crate) num_levels: u32,
    pub(crate) level_names: Vec<u32>,
    pub(crate) entries: Vec<xkb_key_type_entry>,
}

#[derive(Copy, Clone)]
pub(crate) struct xkb_key_type_entry {
    pub(crate) level: u32,
    pub(crate) mods: xkb_mods,
    pub(crate) preserve: xkb_mods,
}

#[derive(Copy, Clone)]
pub(crate) struct xkb_key_alias {
    pub(crate) real: u32,
    pub(crate) alias: u32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct KeycodeMatch {
    pub(crate) found: bool,
    pub(crate) low: bool,
    pub(crate) is_alias: bool,
    pub(crate) index: u32,
}

#[derive(Clone, Default)]
pub(crate) struct xkb_key {
    pub(crate) keycode: u32,
    pub(crate) name: u32,
    pub(crate) explicit: xkb_explicit_components,
    pub(crate) modmap: u32,
    pub(crate) vmodmap: u32,
    pub(crate) overlays: xkb_overlay_mask_t,
    pub(crate) repeats: bool,
    pub(crate) implicit_actions: bool,
    pub(crate) out_of_range_pending_group: bool,
    pub(crate) out_of_range_group_policy: u32,
    pub(crate) out_of_range_group_number: u32,
    pub(crate) num_groups: u32,
    pub(crate) groups: Vec<xkb_group>,
    pub(crate) overlay_keys: Vec<u32>,
}

#[derive(Clone, Default)]
pub(crate) struct xkb_group {
    pub(crate) explicit_symbols: bool,
    pub(crate) explicit_actions: bool,
    pub(crate) implicit_actions: bool,
    pub(crate) explicit_type: bool,
    pub(crate) type_idx: u32,
    pub(crate) levels: Vec<xkb_level>,
}

#[derive(Clone, Default)]
pub(crate) struct xkb_level {
    pub(crate) upper: u32,
    pub(crate) has_upper: bool,
    pub(crate) syms: Vec<u32>,
    pub(crate) actions: Vec<xkb_action>,
}

pub(crate) type xkb_keysym_count_t = u16;
pub(crate) type xkb_overlay_mask_t = u8;
pub(crate) type xkb_overlay_index_t = u8;

pub(crate) type xkb_explicit_components = u32;
pub(crate) const EXPLICIT_OVERLAY: xkb_explicit_components = 32;
pub(crate) const EXPLICIT_REPEAT: xkb_explicit_components = 16;
pub(crate) const EXPLICIT_VMODMAP: xkb_explicit_components = 8;
pub(crate) const EXPLICIT_TYPES: xkb_explicit_components = 4;
pub(crate) const EXPLICIT_INTERP: xkb_explicit_components = 2;
pub(crate) const EXPLICIT_SYMBOLS: xkb_explicit_components = 1;

#[derive(Copy, Clone, Default)]
pub(crate) struct xkb_led {
    pub(crate) name: u32,
    pub(crate) which_groups: u32,
    pub(crate) pending_groups: bool,
    pub(crate) groups: u32,
    pub(crate) which_mods: u32,
    pub(crate) mods: xkb_mods,
    pub(crate) ctrls: xkb_action_controls,
}

pub(crate) const XKB_MAX_GROUPS: i32 = 32_i32;
pub(crate) const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;

// ── Additional xkbcommon types ──────────────────────────────────────

pub(crate) type xkb_keymap_serialize_flags = u32;
pub(crate) const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;

pub(crate) type xkb_led_mask_t = u32;

// ── C constants ─────────────────────────────────────────────────────

pub(crate) const CHAR_BIT: i32 = 8;
pub(crate) const UINT16_MAX: i32 = 65535;

pub(crate) const XKB_MAX_LEDS: u32 =
    (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(CHAR_BIT as usize) as u32;
pub(crate) const MAX_ACTIONS_PER_LEVEL: i32 = UINT16_MAX;

// ── config_h constants ──────────────────────────────────────────────

pub(crate) const DEFAULT_XKB_RULES: &str = "evdev";

pub(crate) const DFLT_XKB_CONFIG_EXTRA_PATH: &str = "/usr/local/etc/xkb";
pub(crate) const DFLT_XKB_CONFIG_ROOT: &str = "/usr/share/xkeyboard-config-2";
pub(crate) const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config.d";
pub(crate) const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config-2.d";
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";

// ── xkbcommon_h types (moved from duplicated pub(crate) mod xkbcommon_h blocks) ─

pub(crate) type xkb_context_flags = u32;
pub(crate) const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub(crate) const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub(crate) const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub(crate) const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;

pub(crate) type xkb_key_direction = u32;
pub(crate) const XKB_KEY_UP: xkb_key_direction = 0;
pub(crate) const XKB_KEY_DOWN: xkb_key_direction = 1;
pub(crate) const XKB_KEY_REPEATED: xkb_key_direction = 2;

pub(crate) type xkb_event_type = u32;
pub(crate) const XKB_EVENT_TYPE_KEY_DOWN: xkb_event_type = 1;
pub(crate) const XKB_EVENT_TYPE_KEY_REPEATED: xkb_event_type = 2;
pub(crate) const XKB_EVENT_TYPE_KEY_UP: xkb_event_type = 3;
pub(crate) const XKB_EVENT_TYPE_COMPONENTS_CHANGE: xkb_event_type = 4;

pub(crate) type xkb_consumed_mode = u32;
pub(crate) const XKB_CONSUMED_MODE_XKB: xkb_consumed_mode = 0;

pub(crate) type xkb_keysym_flags = u32;
pub(crate) const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
pub(crate) const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;

pub(crate) type xkb_state_match = u32;
pub(crate) const XKB_STATE_MATCH_ANY: xkb_state_match = 1;
pub(crate) const XKB_STATE_MATCH_NON_EXCLUSIVE: xkb_state_match = 65536;

pub(crate) type xkb_a11y_flags = u32;
pub(crate) const XKB_A11Y_NO_FLAGS: xkb_a11y_flags = 0;
pub(crate) const XKB_A11Y_LATCH_TO_LOCK: xkb_a11y_flags = 1;
pub(crate) const XKB_A11Y_LATCH_SIMULTANEOUS_KEYS: xkb_a11y_flags = 2;

pub(crate) type xkb_keyboard_control_flags = u32;
pub(crate) const XKB_KEYBOARD_CONTROL_NO_FLAGS: xkb_keyboard_control_flags = 0;

pub(crate) const XKB_KEYCODE_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_KEYCODE_MAX: u32 = 0xffffffff_u32.wrapping_sub(1);
pub(crate) const XKB_LED_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_LEVEL_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_KEYMAP_USE_ORIGINAL_FORMAT: u32 = 0xffffffff;

pub(crate) const XKB_KEYSYM_MAX: i32 = 0x1fffffff;

#[derive(Clone, Default)]
pub(crate) struct xkb_component_names {
    pub(crate) keycodes: Vec<i8>,
    pub(crate) compatibility: Vec<i8>,
    pub(crate) geometry: Vec<i8>,
    pub(crate) symbols: Vec<i8>,
    pub(crate) types: Vec<i8>,
}

#[derive(Copy, Clone)]
pub(crate) struct xkb_state_components_update {
    pub(crate) size: usize,
    pub(crate) components: u32,
    pub(crate) affect_latched_mods: u32,
    pub(crate) latched_mods: u32,
    pub(crate) affect_locked_mods: u32,
    pub(crate) locked_mods: u32,
    pub(crate) latched_layout: i32,
    pub(crate) locked_layout: i32,
    pub(crate) affect_controls: xkb_keyboard_control_flags,
    pub(crate) controls: xkb_keyboard_control_flags,
}

#[derive(Copy, Clone)]
pub(crate) struct xkb_layout_policy_update {
    pub(crate) size: usize,
    pub(crate) policy: u32,
    pub(crate) redirect: u32,
}

#[derive(Copy, Clone)]
pub(crate) struct xkb_state_update {
    pub(crate) size: usize,
    pub(crate) components: *const xkb_state_components_update,
    pub(crate) layout_policy: *const xkb_layout_policy_update,
}

pub(crate) const XKB_ATOM_NONE: u32 = 0;

/// Get text for an atom as a string slice.
pub(crate) fn atom_text(table: &atom_table, atom: u32) -> &str {
    if (atom as usize) >= table.strings.len() {
        return "";
    }
    match &table.strings[atom as usize] {
        Some(s) => s.as_str(),
        None => "",
    }
}

/// Look up an existing atom without mutating the table.
pub(crate) fn atom_lookup_ref(table: &atom_table, input_bytes: &[u8]) -> u32 {
    let hash = hash_buf(input_bytes);
    for i in 0..table.index.len() {
        let index_pos = ((hash as usize) + i) & (table.index.len() - 1);
        if index_pos == 0 {
            continue;
        }
        let existing_atom = table.index[index_pos];
        if existing_atom == XKB_ATOM_NONE {
            return XKB_ATOM_NONE;
        }
        if let Some(ref existing) = table.strings[existing_atom as usize] {
            if existing.as_bytes() == input_bytes {
                return existing_atom;
            }
        }
    }
    XKB_ATOM_NONE
}

/// Intern a string or look up existing atom
pub(crate) fn atom_intern(table: &mut atom_table, input_bytes: &[u8], add: bool) -> u32 {
    let t = table;

    // Resize hash table if load factor > 0.8
    if t.strings.len() > t.index.len() * 4 / 5 {
        let new_size = t.index.len() * 2;
        t.index = vec![0; new_size];

        // Rehash all strings (skip index 0)
        for j in 1..t.strings.len() {
            if let Some(ref s) = t.strings[j] {
                let s_bytes = s.as_bytes();
                let hash = hash_buf(s_bytes);

                for i in 0..t.index.len() {
                    let index_pos = ((hash as usize) + i) & (t.index.len() - 1);
                    if index_pos == 0 {
                        continue;
                    }
                    if t.index[index_pos] == XKB_ATOM_NONE {
                        t.index[index_pos] = j as u32;
                        break;
                    }
                }
            }
        }
    }

    // Look up or insert string
    let hash = hash_buf(input_bytes);

    for i in 0..t.index.len() {
        let index_pos = ((hash as usize) + i) & (t.index.len() - 1);
        if index_pos == 0 {
            continue;
        }

        let existing_atom = t.index[index_pos];

        // Empty slot - not found
        if existing_atom == XKB_ATOM_NONE {
            if add {
                let new_atom = t.strings.len() as u32;

                let new_string = String::from_utf8(input_bytes.to_vec())
                    .expect("atom string is not valid UTF-8");
                t.strings.push(Some(new_string));

                // Update hash table
                t.index[index_pos] = new_atom;

                return new_atom;
            } else {
                return XKB_ATOM_NONE;
            }
        }

        // Check if existing string matches
        if let Some(ref existing) = t.strings[existing_atom as usize] {
            if existing.as_bytes() == input_bytes {
                return existing_atom;
            }
        }
    }

    // Should never reach here - hash table is kept sparse enough
    panic!("couldn't find an empty slot during probing");
}

// ── keymap_h types & constants (moved from duplicated pub(crate) mod keymap_h blocks) ─

pub(crate) type real_mod_index = u32;

pub(crate) const FALLBACK_INTERPRET_KEY_REPEAT: u32 = 0;
pub(crate) const DEFAULT_INTERPRET_KEY_REPEAT: u32 = 1;
pub(crate) const DEFAULT_KEY_REPEAT: u32 = 0;

pub(crate) const FALLBACK_INTERPRET_VMODMAP: u32 = 0;
pub(crate) const DEFAULT_INTERPRET_VMODMAP: u32 = 0;
pub(crate) const DEFAULT_INTERPRET_VMOD: u32 = 4294967295;
pub(crate) const DEFAULT_KEY_VMODMAP: u32 = 0;

pub(crate) const XKB_MOD_ALL: u32 = 4294967295;
pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub(crate) const XKB_MOD_INDEX_SHIFT: real_mod_index = 0;
pub(crate) const XKB_MOD_INDEX_CAPS: real_mod_index = 1;
pub(crate) const XKB_MOD_INDEX_CTRL: real_mod_index = 2;
pub(crate) const XKB_MOD_INDEX_MOD1: real_mod_index = 3;
pub(crate) const XKB_MOD_INDEX_MOD2: real_mod_index = 4;
pub(crate) const XKB_MOD_INDEX_MOD3: real_mod_index = 5;
pub(crate) const XKB_MOD_INDEX_MOD4: real_mod_index = 6;
pub(crate) const XKB_MOD_INDEX_MOD5: real_mod_index = 7;
pub(crate) const _XKB_MOD_INDEX_NUM_ENTRIES: real_mod_index = 8;

pub(crate) const XKB_MAX_GROUPS_X11: i32 = 4;
pub(crate) const XKB_ALL_GROUPS: u64 = ((1u64) << XKB_MAX_GROUPS).wrapping_sub(1u64);

pub(crate) const XKB_OVERLAY_MAX_X11: i32 = 2;
pub(crate) const XKB_OVERLAY_MAX: u64 =
    (std::mem::size_of::<xkb_overlay_mask_t>() as u64).wrapping_mul(CHAR_BIT as u64);
pub(crate) const XKB_OVERLAY1_CONTROLS_OFFSET: i32 = 1;
pub(crate) const XKB_OVERLAY_INVALID: i32 = 255;

pub(crate) const XKB_KEYCODE_MAX_CONTIGUOUS: i32 = 0xfff;
pub(crate) const XKB_LEVEL_MAX_IMPL: i32 = 2048;
pub(crate) const XKB_MAX_MODS: u32 =
    (std::mem::size_of::<u32>()).wrapping_mul(CHAR_BIT as usize) as u32;

// ── Safe methods on xkb_keymap ──────────────────────────────────────

impl xkb_keymap {
    /// Look up a key by keycode. Safe wrapper around the old `XkbKey` function.
    #[inline]
    pub(crate) fn get_key(&self, kc: u32) -> Option<&xkb_key> {
        if kc < self.min_key_code || kc > self.max_key_code {
            None
        } else if kc < self.num_keys_low {
            Some(&self.keys[kc as usize])
        } else {
            let mut lower: u32 = self.num_keys_low;
            let mut upper: u32 = self.num_keys;
            while lower < upper {
                let mid: u32 = lower.wrapping_add(
                    upper
                        .wrapping_sub(1_u32)
                        .wrapping_sub(lower)
                        .wrapping_div(2_u32),
                );
                let key = &self.keys[mid as usize];
                if key.keycode < kc {
                    lower = mid.wrapping_add(1_u32);
                } else if key.keycode > kc {
                    upper = mid;
                } else {
                    return Some(key);
                }
            }
            None
        }
    }

    /// Safe wrapper around `XkbKeyNumLevels`.
    #[inline]
    pub(crate) fn key_num_levels(&self, key: &xkb_key, layout: u32) -> u32 {
        let group = &key.groups[layout as usize];
        self.types[group.type_idx as usize].num_levels
    }

    /// Safe wrapper around `XkbKeyByName`. Looks up a key by atom name using the key_names table.
    #[inline]
    pub(crate) fn key_by_name(&self, name: u32, use_aliases: bool) -> Option<&xkb_key> {
        if (name as usize) < self.key_names.len() {
            let match_0 = self.key_names[name as usize];
            if match_0.found {
                if !match_0.is_alias {
                    return Some(&self.keys[match_0.index as usize]);
                } else if use_aliases {
                    let alias_match = self.key_names[match_0.index as usize];
                    return Some(&self.keys[alias_match.index as usize]);
                }
            }
        }
        None
    }

    /// Mutable version of `key_by_name`.
    #[inline]
    pub(crate) fn key_by_name_mut(&mut self, name: u32, use_aliases: bool) -> Option<&mut xkb_key> {
        if (name as usize) < self.key_names.len() {
            let match_0 = self.key_names[name as usize];
            if match_0.found {
                if !match_0.is_alias {
                    return Some(&mut self.keys[match_0.index as usize]);
                } else if use_aliases {
                    let alias_match = self.key_names[match_0.index as usize];
                    return Some(&mut self.keys[alias_match.index as usize]);
                }
            }
        }
        None
    }

    /// Safe wrapper: look up a key by keycode and resolve the level for a given layout+level index.
    /// Returns `None` if the key doesn't exist, layout is invalid, or level is out of range.
    #[inline]
    pub(crate) fn get_key_level<'a>(
        &'a self,
        key: &'a xkb_key,
        layout: u32,
        level: u32,
    ) -> Option<&'a xkb_level> {
        let layout = super::keymap::XkbWrapGroupIntoRange(
            layout as i32,
            key.num_groups,
            key.out_of_range_group_policy,
            key.out_of_range_group_number,
        );
        if layout == XKB_LAYOUT_INVALID {
            return None;
        }
        if level >= self.key_num_levels(key, layout) {
            return None;
        }
        Some(&key.groups[layout as usize].levels[level as usize])
    }
}

// ── Inline helpers ──────────────────────────────────────────────────

#[inline]
pub(crate) fn entry_is_active(entry: &xkb_key_type_entry) -> bool {
    entry.mods.mods == 0_u32 || entry.mods.mask != 0_u32
}

#[inline]
pub(crate) fn format_max_overlays(format: u32) -> xkb_overlay_index_t {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        XKB_OVERLAY_MAX_X11 as usize
    } else {
        XKB_OVERLAY_MAX as usize
    }) as xkb_overlay_index_t
}

#[inline]
pub(crate) fn format_max_groups(format: u32) -> u32 {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        XKB_MAX_GROUPS_X11
    } else {
        XKB_MAX_GROUPS
    }) as u32
}

#[inline]
pub(crate) fn isModsUnLockOnPressSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub(crate) fn isGroupLockOnReleaseSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub(crate) fn isModsLatchOnPressSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub(crate) fn areOverlappingOverlaysSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

// Error codes (from xkbcommon_errors_h)
pub(crate) type xkb_error_code = i32;
pub(crate) const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX: xkb_error_code = 237;
pub(crate) const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK: xkb_error_code = 60;
pub(crate) const XKB_KEY_NoSymbol: i32 = 0;

// ── errno_base_h ──────────────────────────────────────────────────────
pub(crate) const ENOMEM: i32 = 12;
pub(crate) const EACCES: i32 = 13;
pub(crate) const ENOTDIR: i32 = 20;

// ── locale_h ──────────────────────────────────────────────────────────
pub(crate) const __LC_CTYPE: i32 = 0;
pub(crate) const __LC_ALL: i32 = 6;

// ── unistd_h ──────────────────────────────────────────────────────────
pub(crate) const R_OK: i32 = 4;
pub(crate) const X_OK: i32 = 1;

// NOTE: DIR type alias and __dirstream extern type are in utils.rs
// (because __dirstream is an extern type that must be declared alongside its extern block)

// ── enums_h ───────────────────────────────────────────────────────────
pub(crate) const XKB_COMPOSE_FEED_RESULT_VALUES: u32 = 3;
pub(crate) const XKB_COMPOSE_STATUS_VALUES: u32 = 15;
pub(crate) const XKB_COMPOSE_STATE_FLAGS_VALUES: u32 = 0;
pub(crate) const XKB_COMPOSE_FORMAT_VALUES: u32 = 2;
pub(crate) const XKB_COMPOSE_COMPILE_FLAGS_VALUES: u32 = 0;
pub(crate) const XKB_CONSUMED_MODE_VALUES: u32 = 3;
pub(crate) const XKB_STATE_MATCH_VALUES: u32 = 65539;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES: u32 = 7;
pub(crate) const XKB_KEY_DIRECTION_VALUES: u32 = 7;
pub(crate) const XKB_A11Y_FLAGS_VALUES: u32 = 3;
pub(crate) const XKB_EVENTS_FLAGS_VALUES: u32 = 0;
pub(crate) const XKB_KEYBOARD_CONTROL_FLAGS_VALUES: u32 = 511;
pub(crate) const XKB_STATE_COMPONENT_VALUES: u32 = 1023;
pub(crate) const XKB_EVENT_TYPE_VALUES: u32 = 30;
pub(crate) const XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES: u32 = 3;
pub(crate) const XKB_KEYMAP_SERIALIZE_FLAGS_VALUES: u32 = 7;
pub(crate) const XKB_KEYMAP_FORMAT_VALUES: u32 = 6;
pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: u32 = 1;
pub(crate) const XKB_CONTEXT_FLAGS_VALUES: u32 = 7;
pub(crate) const XKB_KEYSYM_FLAGS_VALUES: u32 = 1;
pub(crate) const XKB_RMLVO_BUILDER_FLAGS_VALUES: u32 = 0;

// ── rmlvo_h (RMLVO enum) ─────────────────────────────────────────────
pub(crate) type RMLVO = u32;
pub(crate) const RMLVO_OPTIONS: RMLVO = 16;
pub(crate) const RMLVO_VARIANT: RMLVO = 8;
pub(crate) const RMLVO_LAYOUT: RMLVO = 4;
pub(crate) const RMLVO_MODEL: RMLVO = 2;
pub(crate) const RMLVO_RULES: RMLVO = 1;

// ── rules_h ───────────────────────────────────────────────────────────
pub(crate) const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;

// ── Message codes (from messages.rs) ─────────────────────────────────────

pub const XKB_LOG_VERBOSITY_DEFAULT: i32 = 0;

pub(crate) const _XKB_LOG_MESSAGE_MIN_CODE: u32 = 34;
pub(crate) const _XKB_LOG_MESSAGE_MAX_CODE: u32 = 971;

pub(crate) const XKB_ERROR_MALFORMED_NUMBER_LITERAL: u32 = 34;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES: u32 = 43;
pub(crate) const XKB_ERROR_INTEGER_OVERFLOW: u32 = 52;
pub(crate) const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_: u32 = 60;
pub(crate) const XKB_ERROR_EXPECTED_ARRAY_ENTRY: u32 = 77;
pub(crate) const XKB_ERROR_INVALID_NUMERIC_KEYSYM: u32 = 82;
pub(crate) const XKB_WARNING_UNRECOGNIZED_KEYSYM: u32 = 107;
pub(crate) const XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER: u32 = 123;
pub(crate) const XKB_ERROR_INSUFFICIENT_BUFFER_SIZE: u32 = 134;
pub(crate) const XKB_ERROR_WRONG_STATEMENT_TYPE: u32 = 150;
pub(crate) const XKB_ERROR_INVALID_PATH: u32 = 161;
pub(crate) const XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION: u32 = 172;
pub(crate) const XKB_WARNING_CANNOT_INFER_KEY_TYPE: u32 = 183;
pub(crate) const XKB_WARNING_INVALID_ESCAPE_SEQUENCE: u32 = 193;
pub(crate) const XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT: u32 = 195;
pub(crate) const XKB_ERROR_INVALID_INCLUDE_STATEMENT: u32 = 203;
pub(crate) const XKB_ERROR_INVALID_MODMAP_ENTRY: u32 = 206;
pub(crate) const XKB_ERROR_UNKNOWN_STATEMENT: u32 = 222;
pub(crate) const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_: u32 = 237;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES: u32 = 239;
pub(crate) const XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT: u32 = 254;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY: u32 = 266;
pub(crate) const XKB_WARNING_UNDEFINED_KEY_TYPE: u32 = 286;
pub(crate) const XKB_WARNING_DEPRECATED_KEYSYM_NAME: u32 = 302;
pub(crate) const XKB_WARNING_NON_BASE_GROUP_NAME: u32 = 305;
pub(crate) const XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL: u32 = 312;
pub(crate) const XKB_ERROR_INCLUDED_FILE_NOT_FOUND: u32 = 338;
pub(crate) const XKB_ERROR_UNKNOWN_OPERATOR: u32 = 345;
pub(crate) const XKB_ERROR_OVERLAPPING_OVERLAY: u32 = 355;
pub(crate) const XKB_WARNING_UNSUPPORTED_LEGACY_ACTION: u32 = 362;
pub(crate) const XKB_WARNING_DUPLICATE_ENTRY: u32 = 378;
pub(crate) const XKB_ERROR_RECURSIVE_INCLUDE: u32 = 386;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS: u32 = 407;
pub(crate) const XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE: u32 = 428;
pub(crate) const XKB_WARNING_MISSING_DEFAULT_SECTION: u32 = 433;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_SYMBOL: u32 = 461;
pub(crate) const XKB_ERROR_INVALID_OPERATION: u32 = 478;
pub(crate) const XKB_WARNING_NUMERIC_KEYSYM: u32 = 489;
pub(crate) const XKB_WARNING_EXTRA_SYMBOLS_IGNORED: u32 = 516;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_NAME: u32 = 523;
pub(crate) const XKB_ERROR_INVALID_FILE_ENCODING: u32 = 542;
pub(crate) const XKB_ERROR_ALLOCATION_ERROR: u32 = 550;
pub(crate) const XKB_ERROR_INVALID_ACTION_FIELD: u32 = 563;
pub(crate) const XKB_ERROR_WRONG_FIELD_TYPE: u32 = 578;
pub(crate) const XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX: u32 = 588;
pub(crate) const XKB_ERROR_CANNOT_RESOLVE_RMLVO: u32 = 595;
pub(crate) const XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE: u32 = 607;
pub(crate) const XKB_ERROR_INVALID_REAL_MODIFIER: u32 = 623;
pub const XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH: u32 = 632;
pub(crate) const XKB_ERROR_UNKNOWN_DEFAULT_FIELD: u32 = 639;
pub(crate) const XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE: u32 = 645;
pub(crate) const XKB_ERROR_INVALID_INCLUDED_FILE: u32 = 661;
pub(crate) const XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE: u32 = 700;
pub(crate) const XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD: u32 = 711;
pub(crate) const XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT: u32 = 742;
pub(crate) const XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION: u32 = 762;
pub(crate) const XKB_ERROR_INVALID_XKB_SYNTAX: u32 = 769;
pub(crate) const XKB_WARNING_UNDEFINED_KEYCODE: u32 = 770;
pub(crate) const XKB_ERROR_INVALID_EXPRESSION_TYPE: u32 = 784;
pub(crate) const XKB_ERROR_INVALID_VALUE: u32 = 796;
pub(crate) const XKB_WARNING_CONFLICTING_MODMAP: u32 = 800;
pub(crate) const XKB_ERROR_UNKNOWN_FIELD: u32 = 812;
pub(crate) const XKB_ERROR_KEYMAP_COMPILATION_FAILED: u32 = 822;
pub(crate) const XKB_ERROR_UNKNOWN_ACTION_TYPE: u32 = 844;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_ACTION: u32 = 883;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS: u32 = 893;
pub(crate) const XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY: u32 = 901;
pub(crate) const XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX: u32 = 903;
pub(crate) const XKB_WARNING_CONFLICTING_KEY_FIELDS: u32 = 935;
pub(crate) const XKB_ERROR_INVALID_IDENTIFIER: u32 = 949;
pub(crate) const XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL: u32 = 965;
pub(crate) const XKB_ERROR_INVALID_RULES_SYNTAX: u32 = 967;
pub(crate) const XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE: u32 = 971;

// ── LookupEntry (moved from keymap.rs) ────────────────────────────

#[derive(Copy, Clone)]
pub(crate) struct LookupEntry {
    pub(crate) name: &'static str,
    pub(crate) value: u32,
}

// ── Shared AST type definitions (merged from shared_ast_types.rs) ──

pub(crate) type xkb_message_code = u32;
pub(crate) type xkb_log_verbosity = i32;
pub(crate) const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;

// ── File type enum ──────────────────────────────────────────────────

pub(crate) const FILE_TYPE_INVALID: u32 = 7;
pub(crate) const _FILE_TYPE_NUM_ENTRIES: u32 = 7;
pub(crate) const FILE_TYPE_RULES: u32 = 6;
pub(crate) const FILE_TYPE_KEYMAP: u32 = 5;
pub(crate) const FILE_TYPE_GEOMETRY: u32 = 4;
pub(crate) const LAST_KEYMAP_FILE_TYPE: u32 = 3;
pub(crate) const FIRST_KEYMAP_FILE_TYPE: u32 = 0;
pub(crate) const FILE_TYPE_SYMBOLS: u32 = 3;
pub(crate) const FILE_TYPE_COMPAT: u32 = 2;
pub(crate) const FILE_TYPE_TYPES: u32 = 1;
pub(crate) const FILE_TYPE_KEYCODES: u32 = 0;

// ── Statement type enum ─────────────────────────────────────────────

pub(crate) type stmt_type = u32;
pub(crate) const _STMT_NUM_VALUES: stmt_type = 37;
pub(crate) const STMT_UNKNOWN_COMPOUND: stmt_type = 36;
pub(crate) const STMT_UNKNOWN_DECLARATION: stmt_type = 35;
pub(crate) const STMT_LED_NAME: stmt_type = 34;
pub(crate) const STMT_LED_MAP: stmt_type = 33;
pub(crate) const STMT_GROUP_COMPAT: stmt_type = 32;
pub(crate) const STMT_MODMAP: stmt_type = 31;
pub(crate) const STMT_SYMBOLS: stmt_type = 30;
pub(crate) const STMT_VMOD: stmt_type = 29;
pub(crate) const STMT_INTERP: stmt_type = 28;
pub(crate) const STMT_TYPE: stmt_type = 27;
pub(crate) const STMT_VAR: stmt_type = 26;
pub(crate) const STMT_EXPR_UNARY_PLUS: stmt_type = 25;
pub(crate) const STMT_EXPR_INVERT: stmt_type = 24;
pub(crate) const STMT_EXPR_NEGATE: stmt_type = 23;
pub(crate) const STMT_EXPR_NOT: stmt_type = 22;
pub(crate) const STMT_EXPR_ASSIGN: stmt_type = 21;
pub(crate) const STMT_EXPR_DIVIDE: stmt_type = 20;
pub(crate) const STMT_EXPR_MULTIPLY: stmt_type = 19;
pub(crate) const STMT_EXPR_SUBTRACT: stmt_type = 18;
pub(crate) const STMT_EXPR_ADD: stmt_type = 17;
pub(crate) const STMT_EXPR_ACTION_LIST: stmt_type = 16;
pub(crate) const STMT_EXPR_KEYSYM_LIST: stmt_type = 15;
pub(crate) const STMT_EXPR_EMPTY_LIST: stmt_type = 14;
pub(crate) const STMT_EXPR_ARRAY_REF: stmt_type = 13;
pub(crate) const STMT_EXPR_FIELD_REF: stmt_type = 12;
pub(crate) const STMT_EXPR_ACTION_DECL: stmt_type = 11;
pub(crate) const STMT_EXPR_IDENT: stmt_type = 10;
pub(crate) const STMT_EXPR_KEYSYM_LITERAL: stmt_type = 9;
pub(crate) const STMT_EXPR_KEYNAME_LITERAL: stmt_type = 8;
pub(crate) const STMT_EXPR_BOOLEAN_LITERAL: stmt_type = 7;
pub(crate) const STMT_EXPR_FLOAT_LITERAL: stmt_type = 6;
pub(crate) const STMT_EXPR_INTEGER_LITERAL: stmt_type = 5;
pub(crate) const STMT_EXPR_STRING_LITERAL: stmt_type = 4;
pub(crate) const STMT_ALIAS: stmt_type = 3;
pub(crate) const STMT_KEYCODE: stmt_type = 2;
pub(crate) const STMT_INCLUDE: stmt_type = 1;
pub(crate) const STMT_UNKNOWN: stmt_type = 0;

// ── Merge mode enum ─────────────────────────────────────────────────

pub(crate) type merge_mode = u32;
pub(crate) const _MERGE_MODE_NUM_ENTRIES: merge_mode = 4;
pub(crate) const MERGE_REPLACE: merge_mode = 3;
pub(crate) const MERGE_OVERRIDE: merge_mode = 2;
pub(crate) const MERGE_AUGMENT: merge_mode = 1;
pub(crate) const MERGE_DEFAULT: merge_mode = 0;

// ── Core AST node types ─────────────────────────────────────────────

#[derive(Clone)]

pub(crate) struct _IncludeStmt {
    pub(crate) merge: merge_mode,
    pub(crate) stmt: String,
    pub(crate) file: String,
    pub(crate) map: String,
    pub(crate) modifier: String,
    pub(crate) next_incl: Option<Box<_IncludeStmt>>,
}
pub(crate) type IncludeStmt = _IncludeStmt;

// ── Expression types ────────────────────────────────────────────────

/// Expression AST node.
pub(crate) struct ExprDef {
    pub(crate) kind: ExprKind,
}

/// The discriminated payload of an expression node.
pub(crate) enum ExprKind {
    String(u32),
    Integer(i64),
    Float,
    Boolean(bool),
    KeyName(u32),
    KeySym(u32),
    Ident(u32),
    FieldRef {
        element: u32,
        field: u32,
    },
    ArrayRef {
        element: u32,
        field: u32,
        entry: Option<Box<ExprDef>>,
    },
    Action {
        name: u32,
        args: Vec<ExprDef>,
    },
    ActionList {
        actions: Vec<ExprDef>,
    },
    KeysymList {
        syms: Vec<u32>,
    },
    EmptyList,
    Binary {
        op: stmt_type,
        left: Option<Box<ExprDef>>,
        right: Option<Box<ExprDef>>,
    },
    Unary {
        op: stmt_type,
        child: Option<Box<ExprDef>>,
    },
}

impl ExprDef {
    pub(crate) fn stmt_type(&self) -> stmt_type {
        Self::stmt_type_for_kind(&self.kind)
    }

    pub(crate) fn stmt_type_for_kind(kind: &ExprKind) -> stmt_type {
        match kind {
            ExprKind::String(_) => STMT_EXPR_STRING_LITERAL,
            ExprKind::Integer(_) => STMT_EXPR_INTEGER_LITERAL,
            ExprKind::Float => STMT_EXPR_FLOAT_LITERAL,
            ExprKind::Boolean(_) => STMT_EXPR_BOOLEAN_LITERAL,
            ExprKind::KeyName(_) => STMT_EXPR_KEYNAME_LITERAL,
            ExprKind::KeySym(_) => STMT_EXPR_KEYSYM_LITERAL,
            ExprKind::Ident(_) => STMT_EXPR_IDENT,
            ExprKind::FieldRef { .. } => STMT_EXPR_FIELD_REF,
            ExprKind::ArrayRef { .. } => STMT_EXPR_ARRAY_REF,
            ExprKind::Action { .. } => STMT_EXPR_ACTION_DECL,
            ExprKind::ActionList { .. } => STMT_EXPR_ACTION_LIST,
            ExprKind::KeysymList { .. } => STMT_EXPR_KEYSYM_LIST,
            ExprKind::EmptyList => STMT_EXPR_EMPTY_LIST,
            ExprKind::Binary { op, .. } => *op,
            ExprKind::Unary { op, .. } => *op,
        }
    }
}

// Re-export ast_build functions used by consumers via ast_h
pub(crate) use super::xkbcomp::parser::{
    stmt_type_to_operator_char, stmt_type_to_string, xkb_file_type_to_string,
};

// ── Statement definition types ──────────────────────────────────────

pub(crate) struct VarDef {
    pub(crate) merge: merge_mode,
    pub(crate) name: Option<Box<ExprDef>>,
    pub(crate) value: Option<Box<ExprDef>>,
}

pub(crate) struct VModDef {
    pub(crate) merge: merge_mode,
    pub(crate) name: u32,
    pub(crate) value: Option<Box<ExprDef>>,
}

#[derive(Copy, Clone)]

pub(crate) struct KeycodeDef {
    pub(crate) merge: merge_mode,
    pub(crate) name: u32,
    pub(crate) value: i64,
}

#[derive(Copy, Clone)]

pub(crate) struct KeyAliasDef {
    pub(crate) merge: merge_mode,
    pub(crate) alias: u32,
    pub(crate) real: u32,
}

pub(crate) struct KeyTypeDef {
    pub(crate) merge: merge_mode,
    pub(crate) name: u32,
    pub(crate) body: Vec<VarDef>,
}

pub(crate) struct SymbolsDef {
    pub(crate) merge: merge_mode,
    pub(crate) keyName: u32,
    pub(crate) symbols: Vec<VarDef>,
}

pub(crate) struct ModMapDef {
    pub(crate) merge: merge_mode,
    pub(crate) modifier: u32,
    pub(crate) keys: Vec<ExprDef>,
}

pub(crate) struct GroupCompatDef {
    pub(crate) merge: merge_mode,
    pub(crate) group: i64,
    pub(crate) def: Option<Box<ExprDef>>,
}

pub(crate) struct InterpDef {
    pub(crate) merge: merge_mode,
    pub(crate) sym: u32,
    pub(crate) match_0: Option<Box<ExprDef>>,
    pub(crate) def: Vec<VarDef>,
}

pub(crate) struct LedNameDef {
    pub(crate) merge: merge_mode,
    pub(crate) virtual_0: bool,
    pub(crate) ndx: i64,
    pub(crate) name: Option<Box<ExprDef>>,
}

pub(crate) struct LedMapDef {
    pub(crate) merge: merge_mode,
    pub(crate) name: u32,
    pub(crate) body: Vec<VarDef>,
}

#[derive(Clone)]

pub(crate) struct UnknownStatement {
    pub(crate) stmt_type: stmt_type,
    pub(crate) name: String,
}

// ── Map flags and XkbFile ───────────────────────────────────────────

pub(crate) type xkb_map_flags = u32;
pub(crate) const MAP_IS_ALTGR: xkb_map_flags = 128;
pub(crate) const MAP_HAS_FN: xkb_map_flags = 64;
pub(crate) const MAP_HAS_KEYPAD: xkb_map_flags = 32;
pub(crate) const MAP_HAS_MODIFIER: xkb_map_flags = 16;
pub(crate) const MAP_HAS_ALPHANUMERIC: xkb_map_flags = 8;
pub(crate) const MAP_IS_HIDDEN: xkb_map_flags = 4;
pub(crate) const MAP_IS_PARTIAL: xkb_map_flags = 2;
pub(crate) const MAP_IS_DEFAULT: xkb_map_flags = 1;

pub(crate) enum Statement {
    Include(Box<IncludeStmt>),
    Keycode(Box<KeycodeDef>),
    KeyAlias(Box<KeyAliasDef>),
    Expr(Box<ExprDef>),
    Var(Box<VarDef>),
    KeyType(Box<KeyTypeDef>),
    Interp(Box<InterpDef>),
    VMod(Box<VModDef>),
    Symbols(Box<SymbolsDef>),
    ModMap(Box<ModMapDef>),
    GroupCompat(Box<GroupCompatDef>),
    LedMap(Box<LedMapDef>),
    LedName(Box<LedNameDef>),
    Unknown(Box<UnknownStatement>),
    XkbFile(Box<XkbFile>),
}

impl Statement {
    pub(crate) fn merge(&self) -> merge_mode {
        match self {
            Statement::Include(s) => s.merge,
            Statement::Keycode(s) => s.merge,
            Statement::KeyAlias(s) => s.merge,
            Statement::Var(s) => s.merge,
            Statement::KeyType(s) => s.merge,
            Statement::Interp(s) => s.merge,
            Statement::VMod(s) => s.merge,
            Statement::Symbols(s) => s.merge,
            Statement::ModMap(s) => s.merge,
            Statement::GroupCompat(s) => s.merge,
            Statement::LedMap(s) => s.merge,
            Statement::LedName(s) => s.merge,
            Statement::Unknown(_) | Statement::Expr(_) | Statement::XkbFile(_) => 0,
        }
    }

    pub(crate) fn stmt_type(&self) -> stmt_type {
        match self {
            Statement::Include(_) => STMT_INCLUDE,
            Statement::Keycode(_) => STMT_KEYCODE,
            Statement::KeyAlias(_) => STMT_ALIAS,
            Statement::Expr(e) => e.stmt_type(),
            Statement::Var(_) => STMT_VAR,
            Statement::KeyType(_) => STMT_TYPE,
            Statement::Interp(_) => STMT_INTERP,
            Statement::VMod(_) => STMT_VMOD,
            Statement::Symbols(_) => STMT_SYMBOLS,
            Statement::ModMap(_) => STMT_MODMAP,
            Statement::GroupCompat(_) => STMT_GROUP_COMPAT,
            Statement::LedMap(_) => STMT_LED_MAP,
            Statement::LedName(_) => STMT_LED_NAME,
            Statement::Unknown(_) => STMT_UNKNOWN,
            Statement::XkbFile(_) => 0,
        }
    }
}

pub(crate) struct XkbFile {
    pub(crate) name: String,
    pub(crate) defs: Vec<Statement>,
    pub(crate) file_type: u32,
    pub(crate) flags: xkb_map_flags,
}

// ── xkbcomp_priv types (parser/keymap info) ─────────────────────────

pub(crate) const PARSER_FATAL_ERROR: u32 = 2;
pub(crate) const PARSER_RECOVERABLE_ERROR: u32 = 1;
pub(crate) const PARSER_SUCCESS: u32 = 0;

pub(crate) const PARSER_V2_LAX_FLAGS: u32 = 0;
pub(crate) const PARSER_V2_STRICT_FLAGS: u32 = 16383;
pub(crate) const PARSER_V1_LAX_FLAGS: u32 = 16379;
pub(crate) const PARSER_V1_STRICT_FLAGS: u32 = 16383;
pub(crate) const PARSER_NO_ILLEGAL_ACTION_FIELDS: u32 = 8192;
pub(crate) const PARSER_NO_UNKNOWN_ACTION_FIELDS: u32 = 4096;
pub(crate) const PARSER_NO_UNKNOWN_ACTION: u32 = 2048;
pub(crate) const PARSER_NO_UNKNOWN_KEY_FIELDS: u32 = 1024;
pub(crate) const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: u32 = 512;
pub(crate) const PARSER_NO_UNKNOWN_LED_FIELDS: u32 = 256;
pub(crate) const PARSER_NO_UNKNOWN_INTERPRET_FIELDS: u32 = 128;
pub(crate) const PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS: u32 = 64;
pub(crate) const PARSER_NO_UNKNOWN_TYPE_FIELDS: u32 = 32;
pub(crate) const PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS: u32 = 16;
pub(crate) const PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS: u32 = 8;
pub(crate) const PARSER_NO_FIELD_VALUE_MISMATCH: u32 = 4;
pub(crate) const PARSER_NO_FIELD_TYPE_MISMATCH: u32 = 2;
pub(crate) const PARSER_NO_UNKNOWN_STATEMENTS: u32 = 1;
pub(crate) const PARSER_NO_STRICT_FLAGS: u32 = 0;

pub(crate) struct pending_computation {
    pub(crate) expr: Option<Box<ExprDef>>,
    pub(crate) computed: bool,
    pub(crate) value: u32,
}

pub(crate) struct xkb_keymap_info<'a> {
    pub(crate) keymap: &'a mut xkb_keymap,
    pub(crate) strict: u32,
    pub(crate) features: XkbcompFeatures,
    pub(crate) lookup: XkbcompLookup,
    pub(crate) pending_computations: Vec<pending_computation>,
}

impl<'a> xkb_keymap_info<'a> {
    pub(crate) fn keymap_ref(&self) -> &xkb_keymap {
        &*self.keymap
    }

    pub(crate) fn keymap_mut(&mut self) -> &mut xkb_keymap {
        &mut *self.keymap
    }

    pub(crate) fn ctx(&self) -> &xkb_context {
        &self.keymap.ctx
    }

    pub(crate) fn ctx_mut(&mut self) -> &mut xkb_context {
        &mut self.keymap.ctx
    }
}

#[derive(Copy, Clone)]

pub(crate) struct XkbcompLookup {
    pub(crate) groupIndexNames: [LookupEntry; 3],
    pub(crate) groupMaskNames: [LookupEntry; 5],
}

#[derive(Copy, Clone)]

pub(crate) struct XkbcompFeatures {
    pub(crate) max_groups: u32,
    pub(crate) max_overlays: xkb_overlay_index_t,
    pub(crate) controls_name_offset: u8,
    pub(crate) group_lock_on_release: bool,
    pub(crate) mods_unlock_on_press: bool,
    pub(crate) mods_latch_on_press: bool,
    pub(crate) overlapping_overlays: bool,
}

// ── Inline helper functions (were duplicated in every xkbcomp_priv_h module) ──

pub(crate) const false_0: i32 = 0;

#[inline]
pub(crate) fn safe_map_name(file: &XkbFile) -> &str {
    if file.name.is_empty() {
        "(unnamed map)"
    } else {
        &file.name
    }
}

#[inline]
pub(crate) fn ReportNotArray(type_0: &str, field: &str, name: &str) -> bool {
    log::error!(
        "[XKB-{:03}] The {} {} field is not an array; Ignoring illegal assignment in {}\n",
        XKB_ERROR_WRONG_FIELD_TYPE as i32,
        type_0,
        field,
        name
    );
    false
}

#[inline]
pub(crate) fn ReportBadType(
    code: xkb_message_code,
    type_0: &str,
    field: &str,
    name: &str,
    wanted: &str,
) -> bool {
    log::error!(
        "[XKB-{:03}] The {} {} field must be a {}; Ignoring illegal assignment in {}\n",
        { code },
        type_0,
        field,
        wanted,
        name
    );
    false
}

#[inline]
pub(crate) fn ReportBadField(type_0: &str, field: &str, name: &str) -> bool {
    log::error!(
        "Unknown {} field \"{}\" in {}; Ignoring assignment to unknown field in {}\n",
        type_0,
        field,
        name,
        name
    );
    false
}

#[inline]
pub(crate) fn ReportShouldBeArray(type_0: &str, field: &str, name: &str) -> bool {
    log::error!(
        "[XKB-{:03}] Missing subscript for {} {}; Ignoring illegal assignment in {}\n",
        XKB_ERROR_EXPECTED_ARRAY_ENTRY as i32,
        type_0,
        field,
        name
    );
    false
}

// ── Utility functions (merged from utils.rs) ──

/// Case-insensitive comparison of two byte slices (like C `strcasecmp`).
/// Returns <0, 0, or >0.
#[inline]
pub(crate) fn istrcmp(a: &[u8], b: &[u8]) -> i32 {
    let n = a.len().min(b.len());
    for i in 0..n {
        let al = a[i].to_ascii_lowercase();
        let bl = b[i].to_ascii_lowercase();
        if al != bl {
            return al as i32 - bl as i32;
        }
    }
    (a.len() as i32) - (b.len() as i32)
}

/// Stack buffer writer implementing `core::fmt::Write`.
pub(crate) struct LogBuf<'a> {
    buf: &'a mut [u8],
    pub(crate) pos: usize,
    pub(crate) truncated: bool,
}

impl<'a> LogBuf<'a> {
    #[inline]
    pub(crate) fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            truncated: false,
        }
    }
}

impl<'a> core::fmt::Write for LogBuf<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let space = self.buf.len() - self.pos;
        let n = bytes.len().min(space);
        if n < bytes.len() {
            self.truncated = true;
        }
        self.buf[self.pos..self.pos + n].copy_from_slice(&bytes[..n]);
        self.pos += n;
        Ok(())
    }
}

/// Parse decimal digits from a byte slice into u32.
pub(crate) fn parse_dec_u32(s: &[u8]) -> (u32, i32) {
    let mut result: u32 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = s[i].wrapping_sub(b'0');
        if d >= 10 {
            break;
        }
        if result > u32::MAX / 10 || result * 10 > u32::MAX - d as u32 {
            return (result, -1);
        }
        result = result * 10 + d as u32;
        i += 1;
    }
    if i < s.len() && s[i].wrapping_sub(b'0') < 10 {
        return (result, -1);
    }
    (result, i as i32)
}

/// Parse decimal digits from a byte slice into u64.
pub(crate) fn parse_dec_u64(s: &[u8]) -> (u64, i32) {
    let mut result: u64 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = s[i].wrapping_sub(b'0');
        if d >= 10 {
            break;
        }
        if result > u64::MAX / 10 || result * 10 > u64::MAX - d as u64 {
            return (result, -1);
        }
        result = result * 10 + d as u64;
        i += 1;
    }
    if i < s.len() && s[i].wrapping_sub(b'0') < 10 {
        return (result, -1);
    }
    (result, i as i32)
}

// ── UTF-8 decoding (migrated from utf8_decoding.rs) ──

/// Invalid UTF-8 code point marker
pub(crate) const INVALID_UTF8_CODE_POINT: u32 = u32::MAX;

/// Decode next UTF-8 code point from byte slice.
pub(crate) fn utf8_next_code_point_safe(bytes: &[u8]) -> (u32, usize) {
    if bytes.is_empty() {
        return (INVALID_UTF8_CODE_POINT, 0);
    }
    let b0 = bytes[0];
    let (len, mut cp) = match b0 {
        0x00..=0x7F => return (b0 as u32, 1),
        0xC2..=0xDF => (2, (b0 as u32) & 0x1F),
        0xE0..=0xEF => (3, (b0 as u32) & 0x0F),
        0xF0..=0xF4 => (4, (b0 as u32) & 0x07),
        _ => return (INVALID_UTF8_CODE_POINT, 0),
    };
    if len > bytes.len() {
        return (INVALID_UTF8_CODE_POINT, 0);
    }
    for &byte in bytes.iter().take(len).skip(1) {
        if (byte & 0xC0) != 0x80 {
            return (INVALID_UTF8_CODE_POINT, 0);
        }
        cp = (cp << 6) | ((byte as u32) & 0x3F);
    }
    if (len == 2 && cp < 0x80)
        || (len == 3 && cp < 0x800)
        || (len == 4 && cp < 0x10000)
        || (0xD800..=0xDFFF).contains(&cp)
        || cp > 0x10FFFF
    {
        return (INVALID_UTF8_CODE_POINT, 0);
    }
    (cp, len)
}

/// Convert a hex digit byte to its numeric value (0-15), or 0xff if invalid.
#[inline]
fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'F' => b - b'A' + 10,
        b'a'..=b'f' => b - b'a' + 10,
        _ => 0xff,
    }
}

/// Parse hex digits from a byte slice into u32.
pub(crate) fn parse_hex_u32(s: &[u8]) -> (u32, i32) {
    let mut result: u32 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = hex_val(s[i]);
        if d >= 16 {
            break;
        }
        if result > u32::MAX >> 4 {
            return (result, -1);
        }
        result = result * 16 + d as u32;
        i += 1;
    }
    if i < s.len() && hex_val(s[i]) < 16 {
        return (result, -1);
    }
    (result, i as i32)
}

/// Parse hex digits from a byte slice into u64.
pub(crate) fn parse_hex_u64(s: &[u8]) -> (u64, i32) {
    let mut result: u64 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        let d = hex_val(s[i]);
        if d >= 16 {
            break;
        }
        if result > u64::MAX >> 4 {
            return (result, -1);
        }
        result = result * 16 + d as u64;
        i += 1;
    }
    if i < s.len() && hex_val(s[i]) < 16 {
        return (result, -1);
    }
    (result, i as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_table_basic() {
        let mut table = atom_table_new();

        // Initially should have 1 atom (NONE at index 0)
        assert_eq!(atom_table_size(&table), 1);

        // Intern a string
        let atom1 = atom_intern(&mut table, b"hello", true);
        assert_ne!(atom1, XKB_ATOM_NONE);
        assert_eq!(atom_table_size(&table), 2);

        // Look up same string again
        let atom2 = atom_intern(&mut table, b"hello", false);
        assert_eq!(atom1, atom2);
        assert_eq!(atom_table_size(&table), 2); // No new atom

        // Intern different string
        let atom3 = atom_intern(&mut table, b"world", true);
        assert_ne!(atom3, atom1);
        assert_eq!(atom_table_size(&table), 3);

        // Get text back
        assert_eq!(atom_text(&table, atom1), "hello");
    }

    #[test]
    fn test_atom_not_found() {
        let mut table = atom_table_new();
        let atom = atom_intern(&mut table, b"test", false);
        assert_eq!(atom, XKB_ATOM_NONE);
    }

    #[test]
    fn test_utf8_next_code_point_ascii() {
        let (cp, len) = utf8_next_code_point_safe(b"ABC");
        assert_eq!((cp, len), (b'A' as u32, 1));
    }

    #[test]
    fn test_utf8_next_code_point_multibyte() {
        assert_eq!(utf8_next_code_point_safe(&[0xE2, 0x82, 0xAC]), (0x20AC, 3));
        assert_eq!(
            utf8_next_code_point_safe(&[0xF0, 0x9F, 0x98, 0x80]),
            (0x1F600, 4)
        );
    }

    #[test]
    fn test_utf8_next_code_point_invalid() {
        assert_eq!(
            utf8_next_code_point_safe(&[0xE2, 0xFF, 0xAC]),
            (INVALID_UTF8_CODE_POINT, 0)
        );
        assert_eq!(
            utf8_next_code_point_safe(&[0xE2, 0x82]),
            (INVALID_UTF8_CODE_POINT, 0)
        );
        assert_eq!(utf8_next_code_point_safe(&[]), (INVALID_UTF8_CODE_POINT, 0));
    }
}
