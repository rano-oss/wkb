//! Shared type definitions used across multiple modules.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use lasso::Key as _;

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

// ── XkbRuleNames ──────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub(crate) struct XkbRuleNames {
    pub(crate) rules: std::ffi::CString,
    pub(crate) model: std::ffi::CString,
    pub(crate) layout: std::ffi::CString,
    pub(crate) variant: std::ffi::CString,
    pub(crate) options: std::ffi::CString,
}

impl Default for XkbRuleNames {
    fn default() -> Self {
        Self::from_strs("", "", "", "", "")
    }
}

impl XkbRuleNames {
    pub(crate) fn from_strs(
        rules: &str,
        model: &str,
        layout: &str,
        variant: &str,
        options: &str,
    ) -> Self {
        Self {
            rules: std::ffi::CString::new(rules).unwrap(),
            model: std::ffi::CString::new(model).unwrap(),
            layout: std::ffi::CString::new(layout).unwrap(),
            variant: std::ffi::CString::new(variant).unwrap(),
            options: std::ffi::CString::new(options).unwrap(),
        }
    }
}

// ── Opaque types ────────────────────────────────────────────────────

/// Atom table — thin wrapper around `lasso::Rodeo` for string interning.
/// Atoms are `u32` keys where `0` is reserved as `XKB_ATOM_NONE`.
pub(crate) struct AtomTable {
    inner: lasso::Rodeo,
}

impl Clone for AtomTable {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for AtomTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AtomTable").finish()
    }
}

/// Create new atom table
pub(crate) fn atom_table_new() -> AtomTable {
    AtomTable {
        inner: lasso::Rodeo::new(),
    }
}

// ── XkbContext ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub(crate) struct XkbContext {
    pub(crate) log_level: u32,
    pub(crate) log_verbosity: i32,
    pub(crate) includes: Vec<String>,
    pub(crate) failed_includes: Vec<String>,
    pub(crate) atom_table: AtomTable,
    pub(crate) use_environment_names: bool,
    pub(crate) use_secure_getenv: bool,
    pub(crate) pending_default_includes: bool,
}

thread_local! {
    /// Thread-local file cache shared across all XkbContext instances.
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

pub(crate) struct XkbKeymap {
    pub(crate) ctx: XkbContext,
    pub(crate) flags: u32,
    pub(crate) format: u32,
    pub(crate) num_leds: u32,
    pub(crate) leds: [XkbLed; 32],
    pub(crate) min_key_code: u32,
    pub(crate) max_key_code: u32,
    pub(crate) num_keys: u32,
    pub(crate) num_keys_low: u32,
    pub(crate) keys: Vec<XkbKey>,
    pub(crate) key_names: Vec<KeycodeMatch>,
    pub(crate) key_aliases: Vec<XkbKeyAlias>,
    pub(crate) types: Vec<XkbKeyType>,
    pub(crate) sym_interprets: Vec<XkbSymInterpret>,
    pub(crate) mods: XkbModSet,
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
pub(crate) struct XkbModSet {
    pub(crate) mods: [XkbMod; 32],
    pub(crate) num_mods: u32,
    pub(crate) explicit_vmods: u32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbMod {
    pub(crate) name: u32,
    pub(crate) type_0: u32,
    pub(crate) mapping: u32,
}

pub(crate) const MOD_BOTH: u32 = 3;
pub(crate) const MOD_VIRT: u32 = 2;
pub(crate) const MOD_REAL: u32 = 1;

#[derive(Clone)]
pub(crate) struct XkbSymInterpret {
    pub(crate) sym: u32,
    pub(crate) match_0: u32,
    pub(crate) mods: u32,
    pub(crate) virtual_mod: u32,
    pub(crate) level_one_only: bool,
    pub(crate) repeat: bool,
    pub(crate) required: bool,
    pub(crate) num_actions: u16,
    pub(crate) action: XkbAction,
    pub(crate) actions: Vec<XkbAction>,
}

#[derive(Copy, Clone, Default)]
pub(crate) enum XkbAction {
    #[default]
    None,
    Void,
    ModSet(XkbModAction),
    ModLatch(XkbModAction),
    ModLock(XkbModAction),
    GroupSet(XkbGroupAction),
    GroupLatch(XkbGroupAction),
    GroupLock(XkbGroupAction),
    PtrMove(XkbPointerAction),
    PtrButton(XkbPointerButtonAction),
    PtrLock(XkbPointerButtonAction),
    PtrDefault(XkbPointerDefaultAction),
    Terminate,
    SwitchVt(XkbSwitchScreenAction),
    CtrlSet(XkbControlsAction),
    CtrlLock(XkbControlsAction),
    RedirectKey(XkbRedirectKeyAction),
    UnsupportedLegacy,
    Unknown,
    Private(XkbPrivateAction),
    Internal(XkbInternalAction),
}

impl XkbAction {
    pub(crate) fn action_type(&self) -> u32 {
        match self {
            XkbAction::None => ACTION_TYPE_NONE,
            XkbAction::Void => ACTION_TYPE_VOID,
            XkbAction::ModSet(_) => ACTION_TYPE_MOD_SET,
            XkbAction::ModLatch(_) => ACTION_TYPE_MOD_LATCH,
            XkbAction::ModLock(_) => ACTION_TYPE_MOD_LOCK,
            XkbAction::GroupSet(_) => ACTION_TYPE_GROUP_SET,
            XkbAction::GroupLatch(_) => ACTION_TYPE_GROUP_LATCH,
            XkbAction::GroupLock(_) => ACTION_TYPE_GROUP_LOCK,
            XkbAction::PtrMove(_) => ACTION_TYPE_PTR_MOVE,
            XkbAction::PtrButton(_) => ACTION_TYPE_PTR_BUTTON,
            XkbAction::PtrLock(_) => ACTION_TYPE_PTR_LOCK,
            XkbAction::PtrDefault(_) => ACTION_TYPE_PTR_DEFAULT,
            XkbAction::Terminate => ACTION_TYPE_TERMINATE,
            XkbAction::SwitchVt(_) => ACTION_TYPE_SWITCH_VT,
            XkbAction::CtrlSet(_) => ACTION_TYPE_CTRL_SET,
            XkbAction::CtrlLock(_) => ACTION_TYPE_CTRL_LOCK,
            XkbAction::RedirectKey(_) => ACTION_TYPE_REDIRECT_KEY,
            XkbAction::UnsupportedLegacy => ACTION_TYPE_UNSUPPORTED_LEGACY,
            XkbAction::Unknown => ACTION_TYPE_UNKNOWN,
            XkbAction::Private(_) => ACTION_TYPE_PRIVATE,
            XkbAction::Internal(_) => ACTION_TYPE_INTERNAL,
        }
    }

    pub(crate) fn from_type(t: u32) -> Self {
        match t {
            ACTION_TYPE_NONE => XkbAction::None,
            ACTION_TYPE_VOID => XkbAction::Void,
            ACTION_TYPE_MOD_SET => XkbAction::ModSet(XkbModAction {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_MOD_LATCH => XkbAction::ModLatch(XkbModAction {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_MOD_LOCK => XkbAction::ModLock(XkbModAction {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_GROUP_SET => XkbAction::GroupSet(XkbGroupAction::default()),
            ACTION_TYPE_GROUP_LATCH => XkbAction::GroupLatch(XkbGroupAction::default()),
            ACTION_TYPE_GROUP_LOCK => XkbAction::GroupLock(XkbGroupAction::default()),
            ACTION_TYPE_PTR_MOVE => XkbAction::PtrMove(XkbPointerAction::default()),
            ACTION_TYPE_PTR_BUTTON => XkbAction::PtrButton(XkbPointerButtonAction::default()),
            ACTION_TYPE_PTR_LOCK => XkbAction::PtrLock(XkbPointerButtonAction::default()),
            ACTION_TYPE_PTR_DEFAULT => XkbAction::PtrDefault(XkbPointerDefaultAction::default()),
            ACTION_TYPE_TERMINATE => XkbAction::Terminate,
            ACTION_TYPE_SWITCH_VT => XkbAction::SwitchVt(XkbSwitchScreenAction::default()),
            ACTION_TYPE_CTRL_SET => XkbAction::CtrlSet(XkbControlsAction::default()),
            ACTION_TYPE_CTRL_LOCK => XkbAction::CtrlLock(XkbControlsAction::default()),
            ACTION_TYPE_REDIRECT_KEY => XkbAction::RedirectKey(XkbRedirectKeyAction::default()),
            ACTION_TYPE_UNSUPPORTED_LEGACY => XkbAction::UnsupportedLegacy,
            ACTION_TYPE_UNKNOWN => XkbAction::Unknown,
            ACTION_TYPE_PRIVATE => XkbAction::Private(XkbPrivateAction {
                type_0: t,
                ..Default::default()
            }),
            ACTION_TYPE_INTERNAL => XkbAction::Internal(XkbInternalAction::default()),
            _ => XkbAction::None,
        }
    }

    pub(crate) fn set_none(&mut self) {
        *self = XkbAction::None;
    }

    pub(crate) fn as_mods(&self) -> &XkbModAction {
        match self {
            XkbAction::ModSet(m) | XkbAction::ModLatch(m) | XkbAction::ModLock(m) => m,
            _ => panic!("not a mod action"),
        }
    }
    pub(crate) fn as_mods_mut(&mut self) -> &mut XkbModAction {
        match self {
            XkbAction::ModSet(m) | XkbAction::ModLatch(m) | XkbAction::ModLock(m) => m,
            _ => panic!("not a mod action"),
        }
    }

    pub(crate) fn as_group(&self) -> &XkbGroupAction {
        match self {
            XkbAction::GroupSet(g) | XkbAction::GroupLatch(g) | XkbAction::GroupLock(g) => g,
            _ => panic!("not a group action"),
        }
    }
    pub(crate) fn as_group_mut(&mut self) -> &mut XkbGroupAction {
        match self {
            XkbAction::GroupSet(g) | XkbAction::GroupLatch(g) | XkbAction::GroupLock(g) => g,
            _ => panic!("not a group action"),
        }
    }

    pub(crate) fn as_ctrls(&self) -> &XkbControlsAction {
        match self {
            XkbAction::CtrlSet(c) | XkbAction::CtrlLock(c) => c,
            _ => panic!("not a ctrls action"),
        }
    }
    pub(crate) fn as_ctrls_mut(&mut self) -> &mut XkbControlsAction {
        match self {
            XkbAction::CtrlSet(c) | XkbAction::CtrlLock(c) => c,
            _ => panic!("not a ctrls action"),
        }
    }

    pub(crate) fn as_ptr(&self) -> &XkbPointerAction {
        match self {
            XkbAction::PtrMove(p) => p,
            _ => panic!("not a ptr action"),
        }
    }
    pub(crate) fn as_ptr_mut(&mut self) -> &mut XkbPointerAction {
        match self {
            XkbAction::PtrMove(p) => p,
            _ => panic!("not a ptr action"),
        }
    }

    pub(crate) fn as_btn(&self) -> &XkbPointerButtonAction {
        match self {
            XkbAction::PtrButton(b) | XkbAction::PtrLock(b) => b,
            _ => panic!("not a btn action"),
        }
    }
    pub(crate) fn as_btn_mut(&mut self) -> &mut XkbPointerButtonAction {
        match self {
            XkbAction::PtrButton(b) | XkbAction::PtrLock(b) => b,
            _ => panic!("not a btn action"),
        }
    }

    pub(crate) fn as_dflt(&self) -> &XkbPointerDefaultAction {
        match self {
            XkbAction::PtrDefault(d) => d,
            _ => panic!("not a dflt action"),
        }
    }
    pub(crate) fn as_dflt_mut(&mut self) -> &mut XkbPointerDefaultAction {
        match self {
            XkbAction::PtrDefault(d) => d,
            _ => panic!("not a dflt action"),
        }
    }

    pub(crate) fn as_screen(&self) -> &XkbSwitchScreenAction {
        match self {
            XkbAction::SwitchVt(s) => s,
            _ => panic!("not a screen action"),
        }
    }
    pub(crate) fn as_screen_mut(&mut self) -> &mut XkbSwitchScreenAction {
        match self {
            XkbAction::SwitchVt(s) => s,
            _ => panic!("not a screen action"),
        }
    }

    pub(crate) fn as_redirect(&self) -> &XkbRedirectKeyAction {
        match self {
            XkbAction::RedirectKey(r) => r,
            _ => panic!("not a redirect action"),
        }
    }
    pub(crate) fn as_redirect_mut(&mut self) -> &mut XkbRedirectKeyAction {
        match self {
            XkbAction::RedirectKey(r) => r,
            _ => panic!("not a redirect action"),
        }
    }

    pub(crate) fn as_priv(&self) -> &XkbPrivateAction {
        match self {
            XkbAction::Private(p) => p,
            _ => panic!("not a priv action"),
        }
    }
    pub(crate) fn as_priv_mut(&mut self) -> &mut XkbPrivateAction {
        match self {
            XkbAction::Private(p) => p,
            _ => panic!("not a priv action"),
        }
    }

    pub(crate) fn as_internal(&self) -> &XkbInternalAction {
        match self {
            XkbAction::Internal(i) => i,
            _ => panic!("not an internal action"),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct XkbInternalAction {
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
pub struct XkbPrivateAction {
    pub type_0: u32,
    pub data: [u8; 7],
}

#[derive(Copy, Clone, Default)]
pub struct XkbRedirectKeyAction {
    pub keycode: u32,
    pub affect: u32,
    pub mods: u32,
}

#[derive(Copy, Clone, Default)]
pub struct XkbPointerButtonAction {
    pub flags: XkbActionFlags,
    pub count: u8,
    pub button: u8,
}

pub type XkbActionFlags = u32;
pub const ACTION_PENDING_COMPUTATION: XkbActionFlags = 8192;
pub const ACTION_LATCH_ON_PRESS: XkbActionFlags = 4096;
pub const ACTION_UNLOCK_ON_PRESS: XkbActionFlags = 2048;
pub const ACTION_LOCK_ON_RELEASE: XkbActionFlags = 1024;
pub const ACTION_SAME_SCREEN: XkbActionFlags = 512;
pub const ACTION_ACCEL: XkbActionFlags = 256;
pub const ACTION_ABSOLUTE_Y: XkbActionFlags = 128;
pub const ACTION_ABSOLUTE_X: XkbActionFlags = 64;
pub const ACTION_ABSOLUTE_SWITCH: XkbActionFlags = 32;
pub const ACTION_MODS_LOOKUP_MODMAP: XkbActionFlags = 16;
pub const ACTION_LOCK_NO_UNLOCK: XkbActionFlags = 8;
pub const ACTION_LOCK_NO_LOCK: XkbActionFlags = 4;
pub const ACTION_LATCH_TO_LOCK: XkbActionFlags = 2;
pub const ACTION_LOCK_CLEAR: XkbActionFlags = 1;

#[derive(Copy, Clone, Default)]
pub struct XkbPointerAction {
    pub flags: XkbActionFlags,
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone, Default)]
pub struct XkbSwitchScreenAction {
    pub flags: XkbActionFlags,
    pub screen: i8,
}

#[derive(Copy, Clone, Default)]
pub struct XkbPointerDefaultAction {
    pub flags: XkbActionFlags,
    pub value: i8,
}

#[derive(Copy, Clone, Default)]
pub struct XkbControlsAction {
    pub flags: XkbActionFlags,
    pub ctrls: u32,
}

pub const CONTROL_ALL_BOOLEAN: u32 = 2088447;
pub const CONTROL_ALL_BOOLEAN_V1: u32 = 2087943;
pub const CONTROL_IGNORE_GROUP_LOCK: u32 = 1048576;
pub const CONTROL_BELL: u32 = 524288;
pub const CONTROL_AX_FEEDBACK: u32 = 262144;
pub const CONTROL_AX_TIMEOUT: u32 = 131072;
pub const CONTROL_AX: u32 = 65536;
pub const CONTROL_MOUSE_KEYS_ACCEL: u32 = 32768;
pub const CONTROL_MOUSE_KEYS: u32 = 16384;
pub const CONTROL_DEBOUNCE: u32 = 4096;
pub const CONTROL_SLOW: u32 = 2048;
pub const CONTROL_REPEAT: u32 = 1024;
pub const CONTROL_OVERLAY8: u32 = 256;
pub const CONTROL_OVERLAY7: u32 = 128;
pub const CONTROL_OVERLAY6: u32 = 64;
pub const CONTROL_OVERLAY5: u32 = 32;
pub const CONTROL_OVERLAY4: u32 = 16;
pub const CONTROL_OVERLAY3: u32 = 8;
pub const CONTROL_OVERLAY2: u32 = 4;
pub const CONTROL_OVERLAY1: u32 = 2;
pub const CONTROL_STICKY_KEYS: u32 = 1;

#[derive(Copy, Clone, Default)]
pub struct XkbGroupAction {
    pub flags: XkbActionFlags,
    pub group: i32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbModAction {
    pub(crate) type_0: u32,
    pub(crate) flags: XkbActionFlags,
    pub(crate) mods: XkbMods,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbMods {
    pub(crate) mods: u32,
    pub(crate) mask: u32,
}

pub const MATCH_EXACTLY: u32 = 4;
pub const MATCH_ALL: u32 = 3;
pub const MATCH_ANY: u32 = 2;
pub const MATCH_ANY_OR_NONE: u32 = 1;
pub const MATCH_NONE: u32 = 0;

#[derive(Clone)]
pub(crate) struct XkbKeyType {
    pub(crate) name: u32,
    pub(crate) mods: XkbMods,
    pub(crate) required: bool,
    pub(crate) num_levels: u32,
    pub(crate) entries: Vec<XkbKeyTypeEntry>,
}

#[derive(Copy, Clone)]
pub(crate) struct XkbKeyTypeEntry {
    pub(crate) level: u32,
    pub(crate) mods: XkbMods,
    pub(crate) preserve: XkbMods,
}

#[derive(Copy, Clone)]
pub(crate) struct XkbKeyAlias {}

#[derive(Copy, Clone, Default)]
pub(crate) struct KeycodeMatch {
    pub(crate) found: bool,
    pub(crate) low: bool,
    pub(crate) is_alias: bool,
    pub(crate) index: u32,
}

#[derive(Clone, Default)]
pub(crate) struct XkbKey {
    pub(crate) keycode: u32,
    pub(crate) name: u32,
    pub(crate) explicit: u32,
    pub(crate) modmap: u32,
    pub(crate) vmodmap: u32,
    pub(crate) overlays: u8,
    pub(crate) repeats: bool,
    pub(crate) implicit_actions: bool,
    pub(crate) out_of_range_pending_group: bool,
    pub(crate) out_of_range_group_policy: u32,
    pub(crate) out_of_range_group_number: u32,
    pub(crate) num_groups: u32,
    pub(crate) groups: Vec<XkbGroup>,
    pub(crate) overlay_keys: Vec<u32>,
}

#[derive(Clone, Default)]
pub(crate) struct XkbGroup {
    pub(crate) explicit_symbols: bool,
    pub(crate) explicit_actions: bool,
    pub(crate) implicit_actions: bool,
    pub(crate) explicit_type: bool,
    pub(crate) type_idx: u32,
    pub(crate) levels: Vec<XkbLevel>,
}

#[derive(Clone, Default)]
pub(crate) struct XkbLevel {
    pub(crate) upper: u32,
    pub(crate) has_upper: bool,
    pub(crate) syms: Vec<u32>,
    pub(crate) actions: Vec<XkbAction>,
}

pub(crate) const EXPLICIT_OVERLAY: u32 = 32;
pub(crate) const EXPLICIT_REPEAT: u32 = 16;
pub(crate) const EXPLICIT_VMODMAP: u32 = 8;
pub(crate) const EXPLICIT_TYPES: u32 = 4;
pub(crate) const EXPLICIT_INTERP: u32 = 2;
pub(crate) const EXPLICIT_SYMBOLS: u32 = 1;

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbLed {
    pub(crate) name: u32,
    pub(crate) which_groups: u32,
    pub(crate) pending_groups: bool,
    pub(crate) groups: u32,
    pub(crate) which_mods: u32,
    pub(crate) mods: XkbMods,
    pub(crate) ctrls: u32,
}

pub(crate) const XKB_MAX_GROUPS: u32 = 32;
pub(crate) const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;

// ── Additional xkbcommon types ──────────────────────────────────────


// ── C constants ─────────────────────────────────────────────────────

pub(crate) const CHAR_BIT: usize = 8;
pub(crate) const UINT16_MAX: i32 = 65535;

pub(crate) const XKB_MAX_LEDS: u32 =
    (std::mem::size_of::<u32>()).wrapping_mul(CHAR_BIT as usize) as u32;
pub(crate) const MAX_ACTIONS_PER_LEVEL: i32 = UINT16_MAX;

// ── config_h constants ──────────────────────────────────────────────

pub(crate) const DFLT_XKB_CONFIG_EXTRA_PATH: &str = "/usr/local/etc/xkb";
pub(crate) const DFLT_XKB_CONFIG_ROOT: &str = "/usr/share/xkeyboard-config-2";
pub(crate) const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config.d";
pub(crate) const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config-2.d";
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";

// ── xkbcommon_h types (moved from duplicated pub(crate) mod xkbcommon_h blocks) ─

pub(crate) const XKB_CONTEXT_NO_FLAGS: u32 = 0;
pub(crate) const XKB_CONTEXT_NO_DEFAULT_INCLUDES: u32 = 1;
pub(crate) const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: u32 = 2;
pub(crate) const XKB_CONTEXT_NO_SECURE_GETENV: u32 = 4;

pub(crate) const XKB_KEY_UP: u32 = 0;
pub(crate) const XKB_KEY_DOWN: u32 = 1;
pub(crate) const XKB_KEY_REPEATED: u32 = 2;

pub(crate) const XKB_EVENT_TYPE_KEY_DOWN: u32 = 1;
pub(crate) const XKB_EVENT_TYPE_KEY_REPEATED: u32 = 2;
pub(crate) const XKB_EVENT_TYPE_KEY_UP: u32 = 3;
pub(crate) const XKB_EVENT_TYPE_COMPONENTS_CHANGE: u32 = 4;

pub(crate) const XKB_CONSUMED_MODE_XKB: u32 = 0;

pub(crate) const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub(crate) const XKB_KEYSYM_CASE_INSENSITIVE: u32 = 1;

pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: u32 = 1;

pub(crate) const XKB_A11Y_NO_FLAGS: u32 = 0;
pub(crate) const XKB_A11Y_LATCH_TO_LOCK: u32 = 1;
pub(crate) const XKB_A11Y_LATCH_SIMULTANEOUS_KEYS: u32 = 2;

pub(crate) const XKB_KEYCODE_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_KEYCODE_MAX: u32 = 0xffffffff_u32.wrapping_sub(1);
pub(crate) const XKB_LEVEL_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_KEYSYM_MAX: i32 = 0x1fffffff;

#[derive(Clone, Default)]
pub(crate) struct XkbComponentNames {
    pub(crate) keycodes: Vec<i8>,
    pub(crate) compatibility: Vec<i8>,
    pub(crate) geometry: Vec<i8>,
    pub(crate) symbols: Vec<i8>,
    pub(crate) types: Vec<i8>,
}

#[derive(Copy, Clone)]
pub(crate) struct XkbStateComponentsUpdate {
    pub(crate) components: u32,
    pub(crate) affect_latched_mods: u32,
    pub(crate) latched_mods: u32,
    pub(crate) affect_locked_mods: u32,
    pub(crate) locked_mods: u32,
    pub(crate) latched_layout: i32,
    pub(crate) locked_layout: i32,
}

pub(crate) const XKB_ATOM_NONE: u32 = 0;

/// Get text for an atom as a string slice.
pub(crate) fn atom_text(table: &AtomTable, atom: u32) -> &str {
    if atom == 0 {
        return "";
    }
    // +1 offset: external atoms start at 1 (0=XKB_ATOM_NONE), lasso Spur keys start at 0
    let key = lasso::Key::try_from_usize((atom - 1) as usize).expect("invalid atom key");
    table.inner.try_resolve(&key).unwrap_or("")
}

/// Look up an existing atom without mutating the table.
pub(crate) fn atom_lookup_ref(table: &AtomTable, input_bytes: &[u8]) -> u32 {
    let s = match std::str::from_utf8(input_bytes) {
        Ok(s) => s,
        Err(_) => return XKB_ATOM_NONE,
    };
    // +1 offset: external atoms start at 1 (0=XKB_ATOM_NONE)
    table
        .inner
        .get(s)
        .map(|k| k.into_usize() as u32 + 1)
        .unwrap_or(XKB_ATOM_NONE)
}

/// Intern a string into the atom table, returning its u32 key.
pub(crate) fn atom_intern(table: &mut AtomTable, input_bytes: &[u8]) -> u32 {
    let s = match std::str::from_utf8(input_bytes) {
        Ok(s) => s,
        Err(_) => panic!("atom string is not valid UTF-8"),
    };
    // +1 offset: external atoms start at 1 (0=XKB_ATOM_NONE)
    table.inner.get_or_intern(s).into_usize() as u32 + 1
}

// ── keymap_h types & constants (moved from duplicated pub(crate) mod keymap_h blocks) ─

pub(crate) const DEFAULT_INTERPRET_KEY_REPEAT: u32 = 1;
pub(crate) const DEFAULT_INTERPRET_VMOD: u32 = 4294967295;

pub(crate) const XKB_MOD_ALL: u32 = 4294967295;
pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub(crate) const XKB_MOD_INDEX_CAPS: u32 = 1;
pub(crate) const _XKB_MOD_INDEX_NUM_ENTRIES: u32 = 8;

pub(crate) const XKB_ALL_GROUPS: u64 = ((1u64) << XKB_MAX_GROUPS).wrapping_sub(1u64);

pub(crate) const XKB_OVERLAY_MAX: u8 =
    std::mem::size_of::<u8>().wrapping_mul(CHAR_BIT as usize) as u8;
pub(crate) const XKB_OVERLAY_INVALID: i32 = 255;

pub(crate) const XKB_KEYCODE_MAX_CONTIGUOUS: i32 = 0xfff;
pub(crate) const XKB_LEVEL_MAX_IMPL: i32 = 2048;
pub(crate) const XKB_MAX_MODS: u32 =
    (std::mem::size_of::<u32>()).wrapping_mul(CHAR_BIT as usize) as u32;

// ── Safe methods on XkbKeymap ──────────────────────────────────────

impl XkbKeymap {
    /// Look up a key by keycode. Safe wrapper around the old `XkbKey` function.
    #[inline]
    pub(crate) fn get_key(&self, kc: u32) -> Option<&XkbKey> {
        if kc < self.min_key_code || kc > self.max_key_code {
            None
        } else if kc < self.num_keys_low {
            Some(&self.keys[kc as usize])
        } else {
            self.keys[self.num_keys_low as usize..self.num_keys as usize]
                .binary_search_by(|key| key.keycode.cmp(&kc))
                .ok()
                .map(|i| &self.keys[self.num_keys_low as usize + i])
        }
    }

    /// Safe wrapper around `XkbKeyNumLevels`.
    #[inline]
    pub(crate) fn key_num_levels(&self, key: &XkbKey, layout: u32) -> u32 {
        let group = &key.groups[layout as usize];
        self.types[group.type_idx as usize].num_levels
    }

    /// Safe wrapper around `XkbKeyByName`. Looks up a key by atom name using the key_names table.
    #[inline]
    pub(crate) fn key_by_name(&self, name: u32, use_aliases: bool) -> Option<&XkbKey> {
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
    pub(crate) fn key_by_name_mut(&mut self, name: u32, use_aliases: bool) -> Option<&mut XkbKey> {
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
        key: &'a XkbKey,
        layout: u32,
        level: u32,
    ) -> Option<&'a XkbLevel> {
        let layout = super::keymap::xkb_wrap_group_into_range(
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

#[inline]
pub(crate) fn entry_is_active(entry: &XkbKeyTypeEntry) -> bool {
    entry.mods.mods == 0 || entry.mods.mask != 0
}

#[inline]
pub(crate) fn is_mods_un_lock_on_press_supported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub(crate) fn is_group_lock_on_release_supported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub(crate) fn is_mods_latch_on_press_supported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

#[inline]
pub(crate) fn are_overlapping_overlays_supported(format: u32) -> bool {
    format >= XKB_KEYMAP_FORMAT_TEXT_V2
}

// Error codes (from xkbcommon_errors_h)
pub(crate) const XKB_KEY_NO_SYMBOL: i32 = 0;

// ── errno_base_h ──────────────────────────────────────────────────────

// ── rmlvo_h (RMLVO enum) ─────────────────────────────────────────────
pub(crate) const RMLVO_OPTIONS: u32 = 16;
pub(crate) const RMLVO_VARIANT: u32 = 8;
pub(crate) const RMLVO_LAYOUT: u32 = 4;
pub(crate) const RMLVO_MODEL: u32 = 2;
pub(crate) const RMLVO_RULES: u32 = 1;

// ── rules_h ───────────────────────────────────────────────────────────
// ── Message codes (from messages.rs) ─────────────────────────────────────

pub const XKB_LOG_VERBOSITY_DEFAULT: i32 = 0;

pub(crate) const _XKB_LOG_MESSAGE_MIN_CODE: u32 = 34;
pub(crate) const _XKB_LOG_MESSAGE_MAX_CODE: u32 = 971;

#[derive(Copy, Clone)]
pub(crate) struct LookupEntry {
    pub(crate) name: &'static str,
    pub(crate) value: u32,
}

// ── Shared AST type definitions (merged from shared_ast_types.rs) ──

// ── File type enum ──────────────────────────────────────────────────

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

pub(crate) const _STMT_NUM_VALUES: u32 = 37;
pub(crate) const STMT_UNKNOWN_COMPOUND: u32 = 36;
pub(crate) const STMT_UNKNOWN_DECLARATION: u32 = 35;
pub(crate) const STMT_EXPR_UNARY_PLUS: u32 = 25;
pub(crate) const STMT_EXPR_INVERT: u32 = 24;
pub(crate) const STMT_EXPR_NEGATE: u32 = 23;
pub(crate) const STMT_EXPR_NOT: u32 = 22;
pub(crate) const STMT_EXPR_ASSIGN: u32 = 21;
pub(crate) const STMT_EXPR_DIVIDE: u32 = 20;
pub(crate) const STMT_EXPR_MULTIPLY: u32 = 19;
pub(crate) const STMT_EXPR_SUBTRACT: u32 = 18;
pub(crate) const STMT_EXPR_ADD: u32 = 17;
pub(crate) const STMT_EXPR_ACTION_LIST: u32 = 16;
pub(crate) const STMT_EXPR_KEYSYM_LIST: u32 = 15;
pub(crate) const STMT_EXPR_EMPTY_LIST: u32 = 14;
pub(crate) const STMT_EXPR_ARRAY_REF: u32 = 13;
pub(crate) const STMT_EXPR_FIELD_REF: u32 = 12;
pub(crate) const STMT_EXPR_ACTION_DECL: u32 = 11;
pub(crate) const STMT_EXPR_IDENT: u32 = 10;
pub(crate) const STMT_EXPR_KEYSYM_LITERAL: u32 = 9;
pub(crate) const STMT_EXPR_KEYNAME_LITERAL: u32 = 8;
pub(crate) const STMT_EXPR_BOOLEAN_LITERAL: u32 = 7;
pub(crate) const STMT_EXPR_FLOAT_LITERAL: u32 = 6;
pub(crate) const STMT_EXPR_INTEGER_LITERAL: u32 = 5;
pub(crate) const STMT_EXPR_STRING_LITERAL: u32 = 4;

// ── Merge mode enum ─────────────────────────────────────────────────

pub(crate) const _MERGE_MODE_NUM_ENTRIES: u32 = 4;
pub(crate) const MERGE_REPLACE: u32 = 3;
pub(crate) const MERGE_OVERRIDE: u32 = 2;
pub(crate) const MERGE_AUGMENT: u32 = 1;
pub(crate) const MERGE_DEFAULT: u32 = 0;

// ── Core AST node types ─────────────────────────────────────────────

#[derive(Clone)]

pub(crate) struct _IncludeStmt {
    pub(crate) merge: u32,
    pub(crate) stmt: String,
    pub(crate) file: String,
    pub(crate) map: String,
    pub(crate) modifier: String,
    pub(crate) next_incl: Option<Box<_IncludeStmt>>,
}

// ── Expression types ────────────────────────────────────────────────

/// Expression AST node.

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
        entry: Option<Box<ExprKind>>,
    },
    Action {
        name: u32,
        args: Vec<ExprKind>,
    },
    ActionList {
        actions: Vec<ExprKind>,
    },
    KeysymList {
        syms: Vec<u32>,
    },
    EmptyList,
    Binary {
        op: u32,
        left: Option<Box<ExprKind>>,
        right: Option<Box<ExprKind>>,
    },
    Unary {
        op: u32,
        child: Option<Box<ExprKind>>,
    },
}

impl ExprKind {
    pub(crate) fn stmt_type(&self) -> u32 {
        Self::stmt_type_for_kind(&self)
    }

    pub(crate) fn stmt_type_for_kind(kind: &ExprKind) -> u32 {
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

// ── Statement definition types ──────────────────────────────────────

pub(crate) struct VarDef {
    pub(crate) merge: u32,
    pub(crate) name: Option<Box<ExprKind>>,
    pub(crate) value: Option<Box<ExprKind>>,
}

pub(crate) struct VModDef {
    pub(crate) merge: u32,
    pub(crate) name: u32,
    pub(crate) value: Option<Box<ExprKind>>,
}

#[derive(Copy, Clone)]
pub(crate) struct KeycodeDef {
    pub(crate) merge: u32,
    pub(crate) name: u32,
    pub(crate) value: i64,
}

#[derive(Copy, Clone)]
pub(crate) struct KeyAliasDef {
    pub(crate) merge: u32,
    pub(crate) alias: u32,
    pub(crate) real: u32,
}

pub(crate) struct KeyTypeDef {
    pub(crate) merge: u32,
    pub(crate) name: u32,
    pub(crate) body: Vec<VarDef>,
}

pub(crate) struct SymbolsDef {
    pub(crate) merge: u32,
    pub(crate) key_name: u32,
    pub(crate) symbols: Vec<VarDef>,
}

pub(crate) struct ModMapDef {
    pub(crate) merge: u32,
    pub(crate) modifier: u32,
    pub(crate) keys: Vec<ExprKind>,
}
pub(crate) struct InterpDef {
    pub(crate) merge: u32,
    pub(crate) sym: u32,
    pub(crate) match_0: Option<Box<ExprKind>>,
    pub(crate) def: Vec<VarDef>,
}

pub(crate) struct LedNameDef {
    pub(crate) merge: u32,
    pub(crate) ndx: i64,
    pub(crate) name: Option<Box<ExprKind>>,
}

pub(crate) struct LedMapDef {
    pub(crate) merge: u32,
    pub(crate) name: u32,
    pub(crate) body: Vec<VarDef>,
}

#[derive(Clone)]
pub(crate) struct UnknownStatement {
    pub(crate) _stmt_type: u32,
    pub(crate) _name: String,
}

// ── Map flags and XkbFile ───────────────────────────────────────────

pub(crate) const MAP_IS_ALTGR: u32 = 128;
pub(crate) const MAP_HAS_FN: u32 = 64;
pub(crate) const MAP_HAS_KEYPAD: u32 = 32;
pub(crate) const MAP_HAS_MODIFIER: u32 = 16;
pub(crate) const MAP_HAS_ALPHANUMERIC: u32 = 8;
pub(crate) const MAP_IS_HIDDEN: u32 = 4;
pub(crate) const MAP_IS_PARTIAL: u32 = 2;
pub(crate) const MAP_IS_DEFAULT: u32 = 1;

pub(crate) enum Statement {
    Include(Box<_IncludeStmt>),
    Keycode(Box<KeycodeDef>),
    KeyAlias(Box<KeyAliasDef>),
    Var(Box<VarDef>),
    KeyType(Box<KeyTypeDef>),
    Interp(Box<InterpDef>),
    VMod(Box<VModDef>),
    Symbols(Box<SymbolsDef>),
    ModMap(Box<ModMapDef>),
    GroupCompat(()),
    LedMap(Box<LedMapDef>),
    LedName(Box<LedNameDef>),
    Unknown(Box<UnknownStatement>),
    XkbFile(Box<XkbFile>),
}

pub(crate) struct XkbFile {
    pub(crate) name: String,
    pub(crate) defs: Vec<Statement>,
    pub(crate) file_type: u32,
    pub(crate) flags: u32,
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
pub(crate) struct PendingComputation {
    pub(crate) expr: Option<Box<ExprKind>>,
    pub(crate) computed: bool,
    pub(crate) value: u32,
}

pub(crate) struct XkbKeymapInfo<'a> {
    pub(crate) keymap: &'a mut XkbKeymap,
    pub(crate) strict: u32,
    pub(crate) features: XkbcompFeatures,
    pub(crate) lookup: XkbcompLookup,
    pub(crate) pending_computations: Vec<PendingComputation>,
}

#[derive(Copy, Clone)]

pub(crate) struct XkbcompLookup {
    pub(crate) group_index_names: [LookupEntry; 3],
    pub(crate) group_mask_names: [LookupEntry; 5],
}

#[derive(Copy, Clone)]

pub(crate) struct XkbcompFeatures {
    pub(crate) max_groups: u32,
    pub(crate) max_overlays: u8,
    pub(crate) controls_name_offset: u8,
    pub(crate) group_lock_on_release: bool,
    pub(crate) mods_unlock_on_press: bool,
    pub(crate) mods_latch_on_press: bool,
    pub(crate) overlapping_overlays: bool,
}

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

macro_rules! impl_parse_dec {
    ($name:ident, $t:ty) => {
        pub(crate) fn $name(s: &[u8]) -> ($t, i32) {
            let mut result: $t = 0;
            let mut i: usize = 0;
            while i < s.len() {
                let d = s[i].wrapping_sub(b'0');
                if d >= 10 {
                    break;
                }
                if result > <$t>::MAX / 10 || result * 10 > <$t>::MAX - d as $t {
                    return (result, -1);
                }
                result = result * 10 + d as $t;
                i += 1;
            }
            if i < s.len() && s[i].wrapping_sub(b'0') < 10 {
                return (result, -1);
            }
            (result, i as i32)
        }
    };
}
impl_parse_dec!(parse_dec_u32, u32);
impl_parse_dec!(parse_dec_u64, u64);

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

macro_rules! impl_parse_hex {
    ($name:ident, $t:ty) => {
        pub(crate) fn $name(s: &[u8]) -> ($t, i32) {
            let mut result: $t = 0;
            let mut i: usize = 0;
            while i < s.len() {
                let d = hex_val(s[i]);
                if d >= 16 {
                    break;
                }
                if result > <$t>::MAX >> 4 {
                    return (result, -1);
                }
                result = result * 16 + d as $t;
                i += 1;
            }
            if i < s.len() && hex_val(s[i]) < 16 {
                return (result, -1);
            }
            (result, i as i32)
        }
    };
}
impl_parse_hex!(parse_hex_u32, u32);
impl_parse_hex!(parse_hex_u64, u64);

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
