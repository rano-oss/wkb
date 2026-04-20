//! Shared type definitions used across multiple modules.

// ── xkbcommon public types ───────────────────────────────────────────

pub const XKB_LOG_LEVEL_DEBUG: u32 = 50;
pub const XKB_LOG_LEVEL_INFO: u32 = 40;
pub const XKB_LOG_LEVEL_WARNING: u32 = 30;
pub const XKB_LOG_LEVEL_ERROR: u32 = 20;
pub const XKB_LOG_LEVEL_CRITICAL: u32 = 10;

pub const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: u32 = 2;
pub const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: u32 = 1;
pub const XKB_LAYOUT_OUT_OF_RANGE_WRAP: u32 = 0;

pub const XKB_STATE_CONTROLS: u32 = 512;
pub const XKB_STATE_LEDS: u32 = 256;
pub const XKB_STATE_LAYOUT_EFFECTIVE: u32 = 128;
pub const XKB_STATE_LAYOUT_LOCKED: u32 = 64;
pub const XKB_STATE_LAYOUT_LATCHED: u32 = 32;
pub const XKB_STATE_LAYOUT_DEPRESSED: u32 = 16;
pub const XKB_STATE_MODS_EFFECTIVE: u32 = 8;
pub const XKB_STATE_MODS_LOCKED: u32 = 4;
pub const XKB_STATE_MODS_LATCHED: u32 = 2;
pub const XKB_STATE_MODS_DEPRESSED: u32 = 1;

pub const XKB_KEYMAP_FORMAT_TEXT_V2: u32 = 2;
pub const XKB_KEYMAP_FORMAT_TEXT_V1: u32 = 1;

pub const XKB_KEYMAP_COMPILE_STRICT_MODE: u32 = 1;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: u32 = 0;

pub const XKB_LAYOUT_INVALID: u32 = 0xffffffff;
pub const XKB_MOD_INVALID: u32 = 0xffffffff;

// ── xkb_rule_names ──────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct xkb_rule_names {
    pub rules: std::ffi::CString,
    pub model: std::ffi::CString,
    pub layout: std::ffi::CString,
    pub variant: std::ffi::CString,
    pub options: std::ffi::CString,
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

// atom_table is defined in atom.rs — re-export it here for unified access
pub use crate::xkb::atom::atom_table;

// ── xkb_context ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct xkb_context {
    pub refcnt: i32,
    pub log_level: u32,
    pub log_verbosity: i32,
    // pub user_data: *mut ::core::ffi::c_void,
    pub names_dflt: xkb_rule_names,
    pub includes: Vec<String>,
    pub failed_includes: Vec<String>,
    pub atom_table: atom_table,
    // pub x11_atom_cache: *mut ::core::ffi::c_void,
    pub use_environment_names: bool,
    pub use_secure_getenv: bool,
    pub pending_default_includes: bool,
}

// ── keymap_h types (from keymap_priv.rs) ────────────────────────────

pub struct xkb_keymap {
    pub ctx: xkb_context,
    pub flags: u32,
    pub format: u32,
    pub num_leds: u32,
    pub leds: [xkb_led; 32],
    pub min_key_code: u32,
    pub max_key_code: u32,
    pub num_keys: u32,
    pub num_keys_low: u32,
    pub keys: Vec<xkb_key>,
    pub key_names: Vec<KeycodeMatch>,
    pub key_aliases: Vec<xkb_key_alias>,
    pub types: Vec<xkb_key_type>,
    pub sym_interprets: Vec<xkb_sym_interpret>,
    pub mods: xkb_mod_set,
    pub canonical_state_mask: u32,
    pub redirect_key_auto: u32,
    pub num_groups: u32,
    pub group_names: Vec<u32>,
    pub keycodes_section_name: String,
    pub symbols_section_name: String,
    pub types_section_name: String,
    pub compat_section_name: String,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_mod_set {
    pub mods: [xkb_mod; 32],
    pub num_mods: u32,
    pub explicit_vmods: u32,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_mod {
    pub name: u32,
    pub type_0: u32,
    pub mapping: u32,
}

pub const MOD_BOTH: u32 = 3;
pub const MOD_VIRT: u32 = 2;
pub const MOD_REAL: u32 = 1;

#[derive(Clone)]
pub struct xkb_sym_interpret {
    pub sym: u32,
    pub match_0: u32,
    pub mods: u32,
    pub virtual_mod: u32,
    pub level_one_only: bool,
    pub repeat: bool,
    pub required: bool,
    pub num_actions: u16,
    pub action: xkb_action,
    pub actions: Vec<xkb_action>,
}

#[derive(Copy, Clone)]
pub enum xkb_action {
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

impl Default for xkb_action {
    fn default() -> Self {
        xkb_action::None
    }
}

impl xkb_action {
    pub fn action_type(&self) -> u32 {
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

    pub fn from_type(t: u32) -> Self {
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

    pub fn set_none(&mut self) {
        *self = xkb_action::None;
    }

    pub fn as_mods(&self) -> &xkb_mod_action {
        match self {
            xkb_action::ModSet(m) | xkb_action::ModLatch(m) | xkb_action::ModLock(m) => m,
            _ => panic!("not a mod action"),
        }
    }
    pub fn as_mods_mut(&mut self) -> &mut xkb_mod_action {
        match self {
            xkb_action::ModSet(m) | xkb_action::ModLatch(m) | xkb_action::ModLock(m) => m,
            _ => panic!("not a mod action"),
        }
    }

    pub fn as_group(&self) -> &xkb_group_action {
        match self {
            xkb_action::GroupSet(g) | xkb_action::GroupLatch(g) | xkb_action::GroupLock(g) => g,
            _ => panic!("not a group action"),
        }
    }
    pub fn as_group_mut(&mut self) -> &mut xkb_group_action {
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
pub struct xkb_group_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub group: i32,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_mod_action {
    pub type_0: u32,
    pub flags: xkb_action_flags,
    pub mods: xkb_mods,
}

#[derive(Copy, Clone, Default)]
pub struct xkb_mods {
    pub mods: u32,
    pub mask: u32,
}

pub const MATCH_EXACTLY: u32 = 4;
pub const MATCH_ALL: u32 = 3;
pub const MATCH_ANY: u32 = 2;
pub const MATCH_ANY_OR_NONE: u32 = 1;
pub const MATCH_NONE: u32 = 0;

#[derive(Clone)]
pub struct xkb_key_type {
    pub name: u32,
    pub mods: xkb_mods,
    pub required: bool,
    pub num_levels: u32,
    pub level_names: Vec<u32>,
    pub entries: Vec<xkb_key_type_entry>,
}

#[derive(Copy, Clone)]
pub struct xkb_key_type_entry {
    pub level: u32,
    pub mods: xkb_mods,
    pub preserve: xkb_mods,
}

#[derive(Copy, Clone)]
pub struct xkb_key_alias {
    pub real: u32,
    pub alias: u32,
}

#[derive(Copy, Clone, Default)]
pub struct KeycodeMatch {
    pub found: bool,
    pub low: bool,
    pub is_alias: bool,
    pub index: u32,
}

#[derive(Clone, Default)]
pub struct xkb_key {
    pub keycode: u32,
    pub name: u32,
    pub explicit: xkb_explicit_components,
    pub modmap: u32,
    pub vmodmap: u32,
    pub overlays: xkb_overlay_mask_t,
    pub repeats: bool,
    pub implicit_actions: bool,
    pub out_of_range_pending_group: bool,
    pub out_of_range_group_policy: u32,
    pub out_of_range_group_number: u32,
    pub num_groups: u32,
    pub groups: Vec<xkb_group>,
    pub overlay_keys: Vec<u32>,
}

#[derive(Clone, Default)]
pub struct xkb_group {
    pub explicit_symbols: bool,
    pub explicit_actions: bool,
    pub implicit_actions: bool,
    pub explicit_type: bool,
    pub type_idx: u32,
    pub levels: Vec<xkb_level>,
}

#[derive(Clone, Default)]
pub struct xkb_level {
    pub upper: u32,
    pub has_upper: bool,
    pub syms: Vec<u32>,
    pub actions: Vec<xkb_action>,
}

pub type xkb_keysym_count_t = u16;
pub type xkb_overlay_mask_t = u8;
pub type xkb_overlay_index_t = u8;

pub type xkb_explicit_components = u32;
pub const EXPLICIT_OVERLAY: xkb_explicit_components = 32;
pub const EXPLICIT_REPEAT: xkb_explicit_components = 16;
pub const EXPLICIT_VMODMAP: xkb_explicit_components = 8;
pub const EXPLICIT_TYPES: xkb_explicit_components = 4;
pub const EXPLICIT_INTERP: xkb_explicit_components = 2;
pub const EXPLICIT_SYMBOLS: xkb_explicit_components = 1;

#[derive(Copy, Clone, Default)]
pub struct xkb_led {
    pub name: u32,
    pub which_groups: u32,
    pub pending_groups: bool,
    pub groups: u32,
    pub which_mods: u32,
    pub mods: xkb_mods,
    pub ctrls: xkb_action_controls,
}

pub const XKB_MAX_GROUPS: i32 = 32_i32;
pub const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;

// ── Additional xkbcommon types ──────────────────────────────────────

pub type xkb_keymap_serialize_flags = u32;
pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;

pub type xkb_led_mask_t = u32;

// ── C constants ─────────────────────────────────────────────────────

pub const CHAR_BIT: i32 = 8;
pub const UINT16_MAX: i32 = 65535;

pub const XKB_MAX_LEDS: u32 =
    (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(CHAR_BIT as usize) as u32;
pub const MAX_ACTIONS_PER_LEVEL: i32 = UINT16_MAX;

// ── config_h constants ──────────────────────────────────────────────

pub const DEFAULT_XKB_RULES: &str = "evdev";

pub const DFLT_XKB_CONFIG_EXTRA_PATH: &str = "/usr/local/etc/xkb";
pub const DFLT_XKB_CONFIG_ROOT: &str = "/usr/share/xkeyboard-config-2";
pub const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: &str = "/usr/share/xkeyboard-config.d";
pub const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: &str = "/usr/share/xkeyboard-config-2.d";
pub const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";

// ── xkbcommon_h types (moved from duplicated pub mod xkbcommon_h blocks) ─

pub type xkb_context_flags = u32;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;

pub type xkb_key_direction = u32;
pub const XKB_KEY_UP: xkb_key_direction = 0;
pub const XKB_KEY_DOWN: xkb_key_direction = 1;
pub const XKB_KEY_REPEATED: xkb_key_direction = 2;

pub type xkb_event_type = u32;
pub const XKB_EVENT_TYPE_KEY_DOWN: xkb_event_type = 1;
pub const XKB_EVENT_TYPE_KEY_REPEATED: xkb_event_type = 2;
pub const XKB_EVENT_TYPE_KEY_UP: xkb_event_type = 3;
pub const XKB_EVENT_TYPE_COMPONENTS_CHANGE: xkb_event_type = 4;

pub type xkb_consumed_mode = u32;
pub const XKB_CONSUMED_MODE_XKB: xkb_consumed_mode = 0;

pub type xkb_keysym_flags = u32;
pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;

pub type xkb_state_match = u32;
pub const XKB_STATE_MATCH_ANY: xkb_state_match = 1;
pub const XKB_STATE_MATCH_NON_EXCLUSIVE: xkb_state_match = 65536;

pub type xkb_a11y_flags = u32;
pub const XKB_A11Y_NO_FLAGS: xkb_a11y_flags = 0;
pub const XKB_A11Y_LATCH_TO_LOCK: xkb_a11y_flags = 1;
pub const XKB_A11Y_LATCH_SIMULTANEOUS_KEYS: xkb_a11y_flags = 2;

pub type xkb_keyboard_control_flags = u32;
pub const XKB_KEYBOARD_CONTROL_NO_FLAGS: xkb_keyboard_control_flags = 0;

pub const XKB_KEYCODE_INVALID: u32 = 0xffffffff;
pub const XKB_KEYCODE_MAX: u32 = 0xffffffff_u32.wrapping_sub(1);
pub const XKB_LED_INVALID: u32 = 0xffffffff;
pub const XKB_LEVEL_INVALID: u32 = 0xffffffff;
pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: u32 = 0xffffffff;

pub const XKB_KEYSYM_MAX: i32 = 0x1fffffff;

#[derive(Clone, Default)]
pub struct xkb_component_names {
    pub keycodes: Vec<i8>,
    pub compatibility: Vec<i8>,
    pub geometry: Vec<i8>,
    pub symbols: Vec<i8>,
    pub types: Vec<i8>,
}

#[derive(Copy, Clone)]
pub struct xkb_state_components_update {
    pub size: usize,
    pub components: u32,
    pub affect_latched_mods: u32,
    pub latched_mods: u32,
    pub affect_locked_mods: u32,
    pub locked_mods: u32,
    pub latched_layout: i32,
    pub locked_layout: i32,
    pub affect_controls: xkb_keyboard_control_flags,
    pub controls: xkb_keyboard_control_flags,
}

#[derive(Copy, Clone)]
pub struct xkb_layout_policy_update {
    pub size: usize,
    pub policy: u32,
    pub redirect: u32,
}

#[derive(Copy, Clone)]
pub struct xkb_state_update {
    pub size: usize,
    pub components: *const xkb_state_components_update,
    pub layout_policy: *const xkb_layout_policy_update,
}

pub const XKB_ATOM_NONE: u32 = 0;

// ── keymap_h types & constants (moved from duplicated pub mod keymap_h blocks) ─

pub type real_mod_index = u32;

pub const FALLBACK_INTERPRET_KEY_REPEAT: u32 = 0;
pub const DEFAULT_INTERPRET_KEY_REPEAT: u32 = 1;
pub const DEFAULT_KEY_REPEAT: u32 = 0;

pub const FALLBACK_INTERPRET_VMODMAP: u32 = 0;
pub const DEFAULT_INTERPRET_VMODMAP: u32 = 0;
pub const DEFAULT_INTERPRET_VMOD: u32 = 4294967295;
pub const DEFAULT_KEY_VMODMAP: u32 = 0;

pub const XKB_MOD_ALL: u32 = 4294967295;
pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub const XKB_MOD_INDEX_SHIFT: real_mod_index = 0;
pub const XKB_MOD_INDEX_CAPS: real_mod_index = 1;
pub const XKB_MOD_INDEX_CTRL: real_mod_index = 2;
pub const XKB_MOD_INDEX_MOD1: real_mod_index = 3;
pub const XKB_MOD_INDEX_MOD2: real_mod_index = 4;
pub const XKB_MOD_INDEX_MOD3: real_mod_index = 5;
pub const XKB_MOD_INDEX_MOD4: real_mod_index = 6;
pub const XKB_MOD_INDEX_MOD5: real_mod_index = 7;
pub const _XKB_MOD_INDEX_NUM_ENTRIES: real_mod_index = 8;

pub const XKB_MAX_GROUPS_X11: i32 = 4;
pub const XKB_ALL_GROUPS: u64 = ((1u64) << XKB_MAX_GROUPS).wrapping_sub(1u64);

pub const XKB_OVERLAY_MAX_X11: i32 = 2;
pub const XKB_OVERLAY_MAX: u64 =
    (std::mem::size_of::<xkb_overlay_mask_t>() as u64).wrapping_mul(CHAR_BIT as u64);
pub const XKB_OVERLAY1_CONTROLS_OFFSET: i32 = 1;
pub const XKB_OVERLAY_INVALID: i32 = 255;

pub const XKB_KEYCODE_MAX_CONTIGUOUS: i32 = 0xfff;
pub const XKB_LEVEL_MAX_IMPL: i32 = 2048;
pub const XKB_MAX_MODS: u32 = (std::mem::size_of::<u32>()).wrapping_mul(CHAR_BIT as usize) as u32;

// ── Safe methods on xkb_keymap ──────────────────────────────────────

impl xkb_keymap {
    /// Look up a key by keycode. Safe wrapper around the old `XkbKey` function.
    #[inline]
    pub fn get_key(&self, kc: u32) -> Option<&xkb_key> {
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
    pub fn key_num_levels(&self, key: &xkb_key, layout: u32) -> u32 {
        let group = &key.groups[layout as usize];
        self.types[group.type_idx as usize].num_levels
    }

    /// Safe wrapper: look up a key by keycode and resolve the level for a given layout+level index.
    /// Returns `None` if the key doesn't exist, layout is invalid, or level is out of range.
    #[inline]
    pub fn get_key_level<'a>(
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
pub unsafe fn XkbKeyNumLevels(keymap: *const xkb_keymap, key: *const xkb_key, layout: u32) -> u32 {
    let group = &(&(*key).groups)[layout as usize];
    (&(*keymap).types)[group.type_idx as usize].num_levels
}

#[inline]
pub unsafe fn XkbKey(keymap: *mut xkb_keymap, kc: u32) -> *const xkb_key {
    unsafe {
        if kc < (*keymap).min_key_code || kc > (*keymap).max_key_code {
            std::ptr::null()
        } else if kc < (*keymap).num_keys_low {
            &(&(*keymap).keys)[kc as usize] as *const xkb_key
        } else {
            let mut lower: u32 = (*keymap).num_keys_low;
            let mut upper: u32 = (*keymap).num_keys;
            while lower < upper {
                let mid: u32 = lower.wrapping_add(
                    upper
                        .wrapping_sub(1_u32)
                        .wrapping_sub(lower)
                        .wrapping_div(2_u32),
                );
                let key: *const xkb_key = &(&(*keymap).keys)[mid as usize] as *const xkb_key;
                if (*key).keycode < kc {
                    lower = mid.wrapping_add(1_u32);
                } else if (*key).keycode > kc {
                    upper = mid;
                } else {
                    return key;
                }
            }
            std::ptr::null()
        }
    }
}

#[inline]
pub unsafe fn XkbKeyByName(
    keymap: *const xkb_keymap,
    name: u32,
    use_aliases: bool,
) -> *mut xkb_key {
    unsafe {
        if (name as usize) < (*keymap).key_names.len() {
            let match_0: KeycodeMatch = (&(*keymap).key_names)[name as usize];
            if match_0.found {
                if !match_0.is_alias {
                    return ((*keymap).keys.as_ptr() as *mut xkb_key).add(match_0.index as usize);
                } else if use_aliases {
                    return ((*keymap).keys.as_ptr() as *mut xkb_key)
                        .add((&(*keymap).key_names)[match_0.index as usize].index as usize);
                }
            }
        }
        std::ptr::null_mut()
    }
}

#[inline]
pub fn entry_is_active(entry: &xkb_key_type_entry) -> bool {
    entry.mods.mods == 0_u32 || entry.mods.mask != 0_u32
}

#[inline]
pub fn format_max_overlays(format: u32) -> xkb_overlay_index_t {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        XKB_OVERLAY_MAX_X11 as usize
    } else {
        XKB_OVERLAY_MAX as usize
    }) as xkb_overlay_index_t
}

#[inline]
pub fn format_max_groups(format: u32) -> u32 {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        XKB_MAX_GROUPS_X11
    } else {
        XKB_MAX_GROUPS
    }) as u32
}

#[inline]
pub fn format_boolean_controls(format: u32) -> xkb_action_controls {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        CONTROL_ALL_BOOLEAN_V1 as i32
    } else {
        CONTROL_ALL_BOOLEAN as i32
    }) as xkb_action_controls
}

#[inline]
pub fn isModsUnLockOnPressSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub fn isGroupLockOnReleaseSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub fn isModsLatchOnPressSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub fn areOverlappingOverlaysSupported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

// Error codes (from xkbcommon_errors_h)
pub type xkb_error_code = i32;
pub const XKB_ERROR_ABI_BACKWARD_COMPAT: xkb_error_code = 914;
pub const XKB_ERROR_ABI_FORWARD_COMPAT: xkb_error_code = 876;
pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE: xkb_error_code = 450;
pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS: xkb_error_code = 371;
pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX: xkb_error_code = 237;
pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY: xkb_error_code = 214;
pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK: xkb_error_code = 60;
pub const XKB_KEY_NoSymbol: i32 = 0;

pub const XKB_SUCCESS: xkb_error_code = 0;
pub const XKB_ERROR_INVALID: xkb_error_code = -1;

// ── errno_base_h ──────────────────────────────────────────────────────
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const ENOTDIR: i32 = 20;

// ── locale_h ──────────────────────────────────────────────────────────
pub const __LC_CTYPE: i32 = 0;
pub const __LC_ALL: i32 = 6;

// ── unistd_h ──────────────────────────────────────────────────────────
pub const R_OK: i32 = 4;
pub const X_OK: i32 = 1;

// NOTE: DIR type alias and __dirstream extern type are in utils.rs
// (because __dirstream is an extern type that must be declared alongside its extern block)

// ── enums_h ───────────────────────────────────────────────────────────
pub const XKB_COMPOSE_FEED_RESULT_VALUES: u32 = 3;
pub const XKB_COMPOSE_STATUS_VALUES: u32 = 15;
pub const XKB_COMPOSE_STATE_FLAGS_VALUES: u32 = 0;
pub const XKB_COMPOSE_FORMAT_VALUES: u32 = 2;
pub const XKB_COMPOSE_COMPILE_FLAGS_VALUES: u32 = 0;
pub const XKB_CONSUMED_MODE_VALUES: u32 = 3;
pub const XKB_STATE_MATCH_VALUES: u32 = 65539;
pub const XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES: u32 = 7;
pub const XKB_KEY_DIRECTION_VALUES: u32 = 7;
pub const XKB_A11Y_FLAGS_VALUES: u32 = 3;
pub const XKB_EVENTS_FLAGS_VALUES: u32 = 0;
pub const XKB_KEYBOARD_CONTROL_FLAGS_VALUES: u32 = 511;
pub const XKB_STATE_COMPONENT_VALUES: u32 = 1023;
pub const XKB_EVENT_TYPE_VALUES: u32 = 30;
pub const XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES: u32 = 3;
pub const XKB_KEYMAP_SERIALIZE_FLAGS_VALUES: u32 = 7;
pub const XKB_KEYMAP_FORMAT_VALUES: u32 = 6;
pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: u32 = 1;
pub const XKB_CONTEXT_FLAGS_VALUES: u32 = 7;
pub const XKB_KEYSYM_FLAGS_VALUES: u32 = 1;
pub const XKB_RMLVO_BUILDER_FLAGS_VALUES: u32 = 0;

// ── rmlvo_h (RMLVO enum) ─────────────────────────────────────────────
pub type RMLVO = u32;
pub const RMLVO_OPTIONS: RMLVO = 16;
pub const RMLVO_VARIANT: RMLVO = 8;
pub const RMLVO_LAYOUT: RMLVO = 4;
pub const RMLVO_MODEL: RMLVO = 2;
pub const RMLVO_RULES: RMLVO = 1;

// ── rules_h ───────────────────────────────────────────────────────────
pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;
