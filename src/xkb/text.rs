use crate::xkb::context::{xkb_atom_text, xkb_context_get_buffer};

// Was in text_h module — now at file level
#[derive(Copy, Clone)]
pub struct LookupEntry {
    pub name: &'static str,
    pub value: u32,
}
pub const CONTROL_NAMES_MIN_V2_INDEX: u32 = 0;
pub const CONTROL_NAMES_MIN_V1_INDEX: u32 = 7;
pub const GROUP_LAST_INDEX_NAME: &str = "last";
#[inline]
pub fn format_control_names_offset(format: u32) -> u8 {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        CONTROL_NAMES_MIN_V1_INDEX as i32
    } else {
        CONTROL_NAMES_MIN_V2_INDEX as i32
    }) as u8
}
use crate::xkb::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;

pub const XKB_KEYSYM_NAME_MAX_SIZE: i32 = 31;

pub use crate::xkb::shared_types::{
    format_boolean_controls, xkb_action_controls, xkb_action_type, xkb_mod, xkb_mod_set,
    ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK,
    ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK,
    ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON,
    ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY,
    ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, CONTROL_ALL, CONTROL_ALL_BOOLEAN,
    CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK, CONTROL_AX_TIMEOUT,
    CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP, CONTROL_IGNORE_GROUP_LOCK,
    CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1, CONTROL_OVERLAY2,
    CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6, CONTROL_OVERLAY7,
    CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT,
    XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MOD_NONE, _ACTION_TYPE_NUM_ENTRIES,
};
use crate::xkb::utils::cstr_as_bytes;
pub fn LookupString(tab: &[LookupEntry], string: &str, value_rtrn: &mut u32) -> bool {
    if string.is_empty() {
        return false;
    }
    for entry in tab {
        if entry.name.is_empty() {
            break;
        }
        if entry.name.eq_ignore_ascii_case(string) {
            *value_rtrn = entry.value;
            return true;
        }
    }
    false
}
pub fn LookupValue(tab: &[LookupEntry], value: u32) -> &'static str {
    for entry in tab {
        if entry.name.is_empty() {
            break;
        }
        if entry.value == value {
            return entry.name;
        }
    }
    ""
}
pub static ctrlMaskNames: [LookupEntry; 25] = [
    LookupEntry {
        name: "Overlay3",
        value: CONTROL_OVERLAY3,
    },
    LookupEntry {
        name: "Overlay4",
        value: CONTROL_OVERLAY4,
    },
    LookupEntry {
        name: "Overlay5",
        value: CONTROL_OVERLAY5,
    },
    LookupEntry {
        name: "Overlay6",
        value: CONTROL_OVERLAY6,
    },
    LookupEntry {
        name: "Overlay7",
        value: CONTROL_OVERLAY7,
    },
    LookupEntry {
        name: "Overlay8",
        value: CONTROL_OVERLAY8,
    },
    LookupEntry {
        name: "all",
        value: CONTROL_ALL_BOOLEAN,
    },
    LookupEntry {
        name: "RepeatKeys",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: "Repeat",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: "AutoRepeat",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: "SlowKeys",
        value: CONTROL_SLOW,
    },
    LookupEntry {
        name: "BounceKeys",
        value: CONTROL_DEBOUNCE,
    },
    LookupEntry {
        name: "StickyKeys",
        value: CONTROL_STICKY_KEYS,
    },
    LookupEntry {
        name: "MouseKeys",
        value: CONTROL_MOUSE_KEYS,
    },
    LookupEntry {
        name: "MouseKeysAccel",
        value: CONTROL_MOUSE_KEYS_ACCEL,
    },
    LookupEntry {
        name: "AccessXKeys",
        value: CONTROL_AX,
    },
    LookupEntry {
        name: "AccessXTimeout",
        value: CONTROL_AX_TIMEOUT,
    },
    LookupEntry {
        name: "AccessXFeedback",
        value: CONTROL_AX_FEEDBACK,
    },
    LookupEntry {
        name: "AudibleBell",
        value: CONTROL_BELL,
    },
    LookupEntry {
        name: "IgnoreGroupLock",
        value: CONTROL_IGNORE_GROUP_LOCK,
    },
    LookupEntry {
        name: "Overlay1",
        value: CONTROL_OVERLAY1,
    },
    LookupEntry {
        name: "Overlay2",
        value: CONTROL_OVERLAY2,
    },
    LookupEntry {
        name: "all",
        value: CONTROL_ALL_BOOLEAN_V1,
    },
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static modComponentMaskNames: [LookupEntry; 8] = [
    LookupEntry {
        name: "base",
        value: XKB_STATE_MODS_DEPRESSED,
    },
    LookupEntry {
        name: "latched",
        value: XKB_STATE_MODS_LATCHED,
    },
    LookupEntry {
        name: "locked",
        value: XKB_STATE_MODS_LOCKED,
    },
    LookupEntry {
        name: "effective",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: "compat",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: "any",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static groupComponentMaskNames: [LookupEntry; 7] = [
    LookupEntry {
        name: "base",
        value: XKB_STATE_LAYOUT_DEPRESSED,
    },
    LookupEntry {
        name: "latched",
        value: XKB_STATE_LAYOUT_LATCHED,
    },
    LookupEntry {
        name: "locked",
        value: XKB_STATE_LAYOUT_LOCKED,
    },
    LookupEntry {
        name: "effective",
        value: XKB_STATE_LAYOUT_EFFECTIVE,
    },
    LookupEntry {
        name: "any",
        value: XKB_STATE_LAYOUT_EFFECTIVE,
    },
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static groupMaskNames: [LookupEntry; 3] = [
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "all",
        value: XKB_ALL_GROUPS as u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static buttonNames: [LookupEntry; 7] = [
    LookupEntry {
        name: "Button1",
        value: 1_u32,
    },
    LookupEntry {
        name: "Button2",
        value: 2_u32,
    },
    LookupEntry {
        name: "Button3",
        value: 3_u32,
    },
    LookupEntry {
        name: "Button4",
        value: 4_u32,
    },
    LookupEntry {
        name: "Button5",
        value: 5_u32,
    },
    LookupEntry {
        name: "default",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static useModMapValueNames: [LookupEntry; 5] = [
    LookupEntry {
        name: "LevelOne",
        value: 1_u32,
    },
    LookupEntry {
        name: "Level1",
        value: 1_u32,
    },
    LookupEntry {
        name: "AnyLevel",
        value: 0_u32,
    },
    LookupEntry {
        name: "any",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static actionTypeNames: [LookupEntry; 43] = [
    LookupEntry {
        name: "NoAction",
        value: ACTION_TYPE_NONE,
    },
    LookupEntry {
        name: "VoidAction",
        value: ACTION_TYPE_VOID,
    },
    LookupEntry {
        name: "SetMods",
        value: ACTION_TYPE_MOD_SET,
    },
    LookupEntry {
        name: "LatchMods",
        value: ACTION_TYPE_MOD_LATCH,
    },
    LookupEntry {
        name: "LockMods",
        value: ACTION_TYPE_MOD_LOCK,
    },
    LookupEntry {
        name: "SetGroup",
        value: ACTION_TYPE_GROUP_SET,
    },
    LookupEntry {
        name: "LatchGroup",
        value: ACTION_TYPE_GROUP_LATCH,
    },
    LookupEntry {
        name: "LockGroup",
        value: ACTION_TYPE_GROUP_LOCK,
    },
    LookupEntry {
        name: "MovePtr",
        value: ACTION_TYPE_PTR_MOVE,
    },
    LookupEntry {
        name: "MovePointer",
        value: ACTION_TYPE_PTR_MOVE,
    },
    LookupEntry {
        name: "PtrBtn",
        value: ACTION_TYPE_PTR_BUTTON,
    },
    LookupEntry {
        name: "PointerButton",
        value: ACTION_TYPE_PTR_BUTTON,
    },
    LookupEntry {
        name: "LockPtrBtn",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "LockPtrButton",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "LockPointerButton",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "LockPointerBtn",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "SetPtrDflt",
        value: ACTION_TYPE_PTR_DEFAULT,
    },
    LookupEntry {
        name: "SetPointerDefault",
        value: ACTION_TYPE_PTR_DEFAULT,
    },
    LookupEntry {
        name: "Terminate",
        value: ACTION_TYPE_TERMINATE,
    },
    LookupEntry {
        name: "TerminateServer",
        value: ACTION_TYPE_TERMINATE,
    },
    LookupEntry {
        name: "SwitchScreen",
        value: ACTION_TYPE_SWITCH_VT,
    },
    LookupEntry {
        name: "SetControls",
        value: ACTION_TYPE_CTRL_SET,
    },
    LookupEntry {
        name: "LockControls",
        value: ACTION_TYPE_CTRL_LOCK,
    },
    LookupEntry {
        name: "RedirectKey",
        value: ACTION_TYPE_REDIRECT_KEY,
    },
    LookupEntry {
        name: "Redirect",
        value: ACTION_TYPE_REDIRECT_KEY,
    },
    LookupEntry {
        name: "Private",
        value: ACTION_TYPE_PRIVATE,
    },
    LookupEntry {
        name: "ISOLock",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "ActionMessage",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "MessageAction",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "Message",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub static symInterpretMatchMaskNames: [LookupEntry; 6] = [
    LookupEntry {
        name: "NoneOf",
        value: MATCH_NONE,
    },
    LookupEntry {
        name: "AnyOfOrNone",
        value: MATCH_ANY_OR_NONE,
    },
    LookupEntry {
        name: "AnyOf",
        value: MATCH_ANY,
    },
    LookupEntry {
        name: "AllOf",
        value: MATCH_ALL,
    },
    LookupEntry {
        name: "Exactly",
        value: MATCH_EXACTLY,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub unsafe fn ModIndexText(
    ctx: *mut xkb_context,
    mods: *const xkb_mod_set,
    ndx: u32,
) -> &'static str {
    unsafe {
        if ndx == XKB_MOD_INVALID {
            return "none";
        }
        if ndx == XKB_MOD_NONE {
            return "None";
        }
        if ndx >= (*mods).num_mods {
            return "";
        }
        xkb_atom_text(&(*ctx).atom_table, (*mods).mods[ndx as usize].name)
    }
}
pub fn ActionTypeText(type_0: xkb_action_type) -> &'static str {
    let name: &'static str = LookupValue(&actionTypeNames, type_0);
    if !name.is_empty() {
        name
    } else {
        "Private"
    }
}
pub fn KeysymText(sym: u32) -> String {
    xkb_keysym_get_name(sym)
}
pub fn SIMatchText(type_0: u32) -> &'static str {
    LookupValue(&symInterpretMatchMaskNames, type_0)
}
pub unsafe fn ModMaskText(
    ctx: &xkb_context,
    type_0: u32,
    mods: *const xkb_mod_set,
    mask: u32,
) -> String {
    unsafe {
        if mask == 0_u32 {
            return "none".to_string();
        }
        if mask == MOD_REAL_MASK_ALL {
            return "all".to_string();
        }
        if type_0 == MOD_REAL && mask & !MOD_REAL_MASK_ALL != 0
            || (mask as u64 & !(1_u64 << (*mods).num_mods).wrapping_sub(1_u64) != 0) as i32 as i64
                != 0
        {
            return format!("{:#x}", mask);
        }
        let mut result = String::new();
        let mut mod_0: *const xkb_mod = &raw const (*mods).mods as *const xkb_mod;
        let mut remaining = mask;
        while remaining != 0
            && mod_0 < (&raw const (*mods).mods as *const xkb_mod).offset((*mods).num_mods as isize)
        {
            if remaining & 0x1_u32 != 0 {
                if !result.is_empty() {
                    result.push('+');
                }
                result.push_str(xkb_atom_text(&ctx.atom_table, (*mod_0).name));
            }
            mod_0 = mod_0.offset(1);
            remaining >>= 1_i32;
        }
        result
    }
}

use crate::xkb::keysym::xkb_keysym_get_name;
use crate::xkb::shared_types::*;
