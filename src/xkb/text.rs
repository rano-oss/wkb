use crate::xkb::context_priv::{xkb_atom_text, xkb_atom_text_bytes, xkb_context_get_buffer};

// Was in text_h module — now at file level
#[derive(Copy, Clone)]
pub struct LookupEntry {
    pub name: &'static [u8],
    pub value: u32,
}
pub type C2Rust_Unnamed_1 = u32;
pub const CONTROL_NAMES_MIN_V2_INDEX: C2Rust_Unnamed_1 = 0;
pub const CONTROL_NAMES_MIN_V1_INDEX: C2Rust_Unnamed_1 = 7;
pub const GROUP_LAST_INDEX_NAME: &'static [u8] = b"last";
#[inline]
pub unsafe fn format_control_names_offset(mut format: u32) -> u8 {
    return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as u32 {
        CONTROL_NAMES_MIN_V1_INDEX as i32
    } else {
        CONTROL_NAMES_MIN_V2_INDEX as i32
    }) as u8;
}
use crate::xkb::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;

pub const XKB_KEYSYM_NAME_MAX_SIZE: i32 = 31;

pub use crate::xkb::shared_types::{
    format_boolean_controls, mod_type, xkb_action_controls, xkb_action_type, xkb_match_operation,
    xkb_mod, xkb_mod_set, _ACTION_TYPE_NUM_ENTRIES, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET,
    ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL,
    ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE,
    ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK,
    ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE,
    ACTION_TYPE_UNKNOWN, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    MATCH_ALL, MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL,
    MOD_REAL_MASK_ALL, MOD_VIRT, XKB_ALL_GROUPS, XKB_MAX_GROUPS, XKB_MOD_NONE,
};
use crate::xkb::utils::{cstr_as_bytes, istreq};
pub unsafe fn LookupString(
    mut tab: *const LookupEntry,
    string: &[u8],
    mut value_rtrn: *mut u32,
) -> bool {
    unsafe {
        if string.is_empty() {
            return false;
        }
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(&(*entry).name).is_empty() {
            if istreq((*entry).name, string) {
                *value_rtrn = (*entry).value as u32;
                return true;
            }
            entry = entry.offset(1);
        }
        return false;
    }
}
pub unsafe fn LookupValue(mut tab: *const LookupEntry, mut value: u32) -> &'static [u8] {
    unsafe {
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(&(*entry).name).is_empty() {
            if (*entry).value == value as u32 {
                return (*entry).name;
            }
            entry = entry.offset(1);
        }
        return b"";
    }
}
pub static mut ctrlMaskNames: [LookupEntry; 25] = [
    LookupEntry {
        name: b"Overlay3",
        value: CONTROL_OVERLAY3 as u32,
    },
    LookupEntry {
        name: b"Overlay4",
        value: CONTROL_OVERLAY4 as u32,
    },
    LookupEntry {
        name: b"Overlay5",
        value: CONTROL_OVERLAY5 as u32,
    },
    LookupEntry {
        name: b"Overlay6",
        value: CONTROL_OVERLAY6 as u32,
    },
    LookupEntry {
        name: b"Overlay7",
        value: CONTROL_OVERLAY7 as u32,
    },
    LookupEntry {
        name: b"Overlay8",
        value: CONTROL_OVERLAY8 as u32,
    },
    LookupEntry {
        name: b"all",
        value: CONTROL_ALL_BOOLEAN as u32,
    },
    LookupEntry {
        name: b"RepeatKeys",
        value: CONTROL_REPEAT as u32,
    },
    LookupEntry {
        name: b"Repeat",
        value: CONTROL_REPEAT as u32,
    },
    LookupEntry {
        name: b"AutoRepeat",
        value: CONTROL_REPEAT as u32,
    },
    LookupEntry {
        name: b"SlowKeys",
        value: CONTROL_SLOW as u32,
    },
    LookupEntry {
        name: b"BounceKeys",
        value: CONTROL_DEBOUNCE as u32,
    },
    LookupEntry {
        name: b"StickyKeys",
        value: CONTROL_STICKY_KEYS as u32,
    },
    LookupEntry {
        name: b"MouseKeys",
        value: CONTROL_MOUSE_KEYS as u32,
    },
    LookupEntry {
        name: b"MouseKeysAccel",
        value: CONTROL_MOUSE_KEYS_ACCEL as u32,
    },
    LookupEntry {
        name: b"AccessXKeys",
        value: CONTROL_AX as u32,
    },
    LookupEntry {
        name: b"AccessXTimeout",
        value: CONTROL_AX_TIMEOUT as u32,
    },
    LookupEntry {
        name: b"AccessXFeedback",
        value: CONTROL_AX_FEEDBACK as u32,
    },
    LookupEntry {
        name: b"AudibleBell",
        value: CONTROL_BELL as u32,
    },
    LookupEntry {
        name: b"IgnoreGroupLock",
        value: CONTROL_IGNORE_GROUP_LOCK as u32,
    },
    LookupEntry {
        name: b"Overlay1",
        value: CONTROL_OVERLAY1 as u32,
    },
    LookupEntry {
        name: b"Overlay2",
        value: CONTROL_OVERLAY2 as u32,
    },
    LookupEntry {
        name: b"all",
        value: CONTROL_ALL_BOOLEAN_V1 as u32,
    },
    LookupEntry {
        name: b"none",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub static mut modComponentMaskNames: [LookupEntry; 8] = [
    LookupEntry {
        name: b"base",
        value: XKB_STATE_MODS_DEPRESSED as u32,
    },
    LookupEntry {
        name: b"latched",
        value: XKB_STATE_MODS_LATCHED as u32,
    },
    LookupEntry {
        name: b"locked",
        value: XKB_STATE_MODS_LOCKED as u32,
    },
    LookupEntry {
        name: b"effective",
        value: XKB_STATE_MODS_EFFECTIVE as u32,
    },
    LookupEntry {
        name: b"compat",
        value: XKB_STATE_MODS_EFFECTIVE as u32,
    },
    LookupEntry {
        name: b"any",
        value: XKB_STATE_MODS_EFFECTIVE as u32,
    },
    LookupEntry {
        name: b"none",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub static mut groupComponentMaskNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"base",
        value: XKB_STATE_LAYOUT_DEPRESSED as u32,
    },
    LookupEntry {
        name: b"latched",
        value: XKB_STATE_LAYOUT_LATCHED as u32,
    },
    LookupEntry {
        name: b"locked",
        value: XKB_STATE_LAYOUT_LOCKED as u32,
    },
    LookupEntry {
        name: b"effective",
        value: XKB_STATE_LAYOUT_EFFECTIVE as u32,
    },
    LookupEntry {
        name: b"any",
        value: XKB_STATE_LAYOUT_EFFECTIVE as u32,
    },
    LookupEntry {
        name: b"none",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub static mut groupMaskNames: [LookupEntry; 3] = [LookupEntry {
    name: b"",
    value: 0,
}; 3];
pub static mut buttonNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"Button1",
        value: 1 as u32,
    },
    LookupEntry {
        name: b"Button2",
        value: 2 as u32,
    },
    LookupEntry {
        name: b"Button3",
        value: 3 as u32,
    },
    LookupEntry {
        name: b"Button4",
        value: 4 as u32,
    },
    LookupEntry {
        name: b"Button5",
        value: 5 as u32,
    },
    LookupEntry {
        name: b"default",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub static mut useModMapValueNames: [LookupEntry; 5] = [
    LookupEntry {
        name: b"LevelOne",
        value: 1 as u32,
    },
    LookupEntry {
        name: b"Level1",
        value: 1 as u32,
    },
    LookupEntry {
        name: b"AnyLevel",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"any",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub static mut actionTypeNames: [LookupEntry; 43] = [
    LookupEntry {
        name: b"NoAction",
        value: ACTION_TYPE_NONE as u32,
    },
    LookupEntry {
        name: b"VoidAction",
        value: ACTION_TYPE_VOID as u32,
    },
    LookupEntry {
        name: b"SetMods",
        value: ACTION_TYPE_MOD_SET as u32,
    },
    LookupEntry {
        name: b"LatchMods",
        value: ACTION_TYPE_MOD_LATCH as u32,
    },
    LookupEntry {
        name: b"LockMods",
        value: ACTION_TYPE_MOD_LOCK as u32,
    },
    LookupEntry {
        name: b"SetGroup",
        value: ACTION_TYPE_GROUP_SET as u32,
    },
    LookupEntry {
        name: b"LatchGroup",
        value: ACTION_TYPE_GROUP_LATCH as u32,
    },
    LookupEntry {
        name: b"LockGroup",
        value: ACTION_TYPE_GROUP_LOCK as u32,
    },
    LookupEntry {
        name: b"MovePtr",
        value: ACTION_TYPE_PTR_MOVE as u32,
    },
    LookupEntry {
        name: b"MovePointer",
        value: ACTION_TYPE_PTR_MOVE as u32,
    },
    LookupEntry {
        name: b"PtrBtn",
        value: ACTION_TYPE_PTR_BUTTON as u32,
    },
    LookupEntry {
        name: b"PointerButton",
        value: ACTION_TYPE_PTR_BUTTON as u32,
    },
    LookupEntry {
        name: b"LockPtrBtn",
        value: ACTION_TYPE_PTR_LOCK as u32,
    },
    LookupEntry {
        name: b"LockPtrButton",
        value: ACTION_TYPE_PTR_LOCK as u32,
    },
    LookupEntry {
        name: b"LockPointerButton",
        value: ACTION_TYPE_PTR_LOCK as u32,
    },
    LookupEntry {
        name: b"LockPointerBtn",
        value: ACTION_TYPE_PTR_LOCK as u32,
    },
    LookupEntry {
        name: b"SetPtrDflt",
        value: ACTION_TYPE_PTR_DEFAULT as u32,
    },
    LookupEntry {
        name: b"SetPointerDefault",
        value: ACTION_TYPE_PTR_DEFAULT as u32,
    },
    LookupEntry {
        name: b"Terminate",
        value: ACTION_TYPE_TERMINATE as u32,
    },
    LookupEntry {
        name: b"TerminateServer",
        value: ACTION_TYPE_TERMINATE as u32,
    },
    LookupEntry {
        name: b"SwitchScreen",
        value: ACTION_TYPE_SWITCH_VT as u32,
    },
    LookupEntry {
        name: b"SetControls",
        value: ACTION_TYPE_CTRL_SET as u32,
    },
    LookupEntry {
        name: b"LockControls",
        value: ACTION_TYPE_CTRL_LOCK as u32,
    },
    LookupEntry {
        name: b"RedirectKey",
        value: ACTION_TYPE_REDIRECT_KEY as u32,
    },
    LookupEntry {
        name: b"Redirect",
        value: ACTION_TYPE_REDIRECT_KEY as u32,
    },
    LookupEntry {
        name: b"Private",
        value: ACTION_TYPE_PRIVATE as u32,
    },
    LookupEntry {
        name: b"ISOLock",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"ActionMessage",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"MessageAction",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"Message",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"LockDeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"LockDevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"LockDevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"LockDeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DeviceValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DevVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DeviceVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"DevValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub static mut symInterpretMatchMaskNames: [LookupEntry; 6] = [
    LookupEntry {
        name: b"NoneOf",
        value: MATCH_NONE as u32,
    },
    LookupEntry {
        name: b"AnyOfOrNone",
        value: MATCH_ANY_OR_NONE as u32,
    },
    LookupEntry {
        name: b"AnyOf",
        value: MATCH_ANY as u32,
    },
    LookupEntry {
        name: b"AllOf",
        value: MATCH_ALL as u32,
    },
    LookupEntry {
        name: b"Exactly",
        value: MATCH_EXACTLY as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
pub unsafe fn ModIndexText(
    mut ctx: *mut xkb_context,
    mut mods: *const xkb_mod_set,
    mut ndx: xkb_mod_index_t,
) -> *const i8 {
    unsafe {
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            return b"none\0".as_ptr() as *const i8;
        }
        if ndx == XKB_MOD_NONE as xkb_mod_index_t {
            return b"None\0".as_ptr() as *const i8;
        }
        if ndx >= (*mods).num_mods {
            return std::ptr::null();
        }
        return xkb_atom_text(ctx, (*mods).mods[ndx as usize].name);
    }
}
pub unsafe fn ActionTypeText(mut type_0: xkb_action_type) -> &'static [u8] {
    unsafe {
        let name: &'static [u8] = LookupValue(
            &raw const actionTypeNames as *const LookupEntry,
            type_0 as u32,
        );
        return if !name.is_empty() { name } else { b"Private" };
    }
}
pub unsafe fn KeysymText(mut ctx: *mut xkb_context, mut sym: xkb_keysym_t) -> *const i8 {
    unsafe {
        let mut buffer: *mut i8 = xkb_context_get_buffer(ctx, XKB_KEYSYM_NAME_MAX_SIZE as usize);
        xkb_keysym_get_name(sym, buffer, XKB_KEYSYM_NAME_MAX_SIZE as usize);
        return buffer;
    }
}
pub unsafe fn KeyNameText(mut ctx: *mut xkb_context, mut name: xkb_atom_t) -> *const i8 {
    unsafe {
        let mut sname: *const i8 = xkb_atom_text(ctx, name);
        let sname_str = if sname.is_null() {
            ""
        } else {
            std::str::from_utf8(cstr_as_bytes(sname)).unwrap_or("")
        };
        let mut len: usize = sname_str.len().wrapping_add(3 as usize);
        let mut buf: *mut i8 = xkb_context_get_buffer(ctx, len);
        crate::xkb::utils::snprintf_args(buf, len, format_args!("<{}>", sname_str));
        return buf;
    }
}
pub unsafe fn SIMatchText(mut type_0: xkb_match_operation) -> &'static [u8] {
    unsafe {
        return LookupValue(
            &raw const symInterpretMatchMaskNames as *const LookupEntry,
            type_0 as u32,
        );
    }
}
pub unsafe fn ModMaskText(
    mut ctx: *mut xkb_context,
    mut type_0: mod_type,
    mut mods: *const xkb_mod_set,
    mut mask: xkb_mod_mask_t,
) -> *const i8 {
    unsafe {
        let mut buf: [i8; 1024] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut pos: usize = 0 as usize;
        let mut mod_0: *const xkb_mod = std::ptr::null();
        if mask == 0 as xkb_mod_mask_t {
            return b"none\0".as_ptr() as *const i8;
        }
        if mask == MOD_REAL_MASK_ALL {
            return b"all\0".as_ptr() as *const i8;
        }
        if type_0 as u32 == MOD_REAL as u32 && mask & !MOD_REAL_MASK_ALL != 0
            || (mask as u64 & !((1 as u64) << (*mods).num_mods).wrapping_sub(1 as u64) != 0) as i32
                as i64
                != 0
        {
            let (written, _) = crate::xkb::utils::snprintf_args(
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 1024]>(),
                format_args!("{:#x}", mask),
            );
            pos = written;
        } else {
            mod_0 = &raw const (*mods).mods as *const xkb_mod;
            while mask != 0
                && mod_0
                    < (&raw const (*mods).mods as *const xkb_mod).offset((*mods).num_mods as isize)
            {
                if mask & 0x1 as xkb_mod_mask_t != 0 {
                    let (written, trunc) = crate::xkb::utils::snprintf_args(
                        (&raw mut buf as *mut i8).offset(pos as isize),
                        (std::mem::size_of::<[i8; 1024]>()).wrapping_sub(pos),
                        format_args!(
                            "{}{}",
                            if pos == 0 as usize { "" } else { "+" },
                            crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(
                                ctx,
                                (*mod_0).name
                            )),
                        ),
                    );
                    if trunc || written == 0 {
                        break;
                    }
                    pos = pos.wrapping_add(written);
                }
                mod_0 = mod_0.offset(1);
                mask >>= 1 as i32;
            }
        }
        pos = pos.wrapping_add(1);
        let dst = xkb_context_get_buffer(ctx, pos);
        std::ptr::copy_nonoverlapping(&raw mut buf as *const u8, dst as *mut u8, pos);
        return dst as *const i8;
    }
}

unsafe fn c2rust_run_static_initializers() {
    unsafe {
        groupMaskNames = [
            LookupEntry {
                name: b"none",
                value: 0 as u32,
            },
            LookupEntry {
                name: b"all",
                value: XKB_ALL_GROUPS as u32,
            },
            LookupEntry {
                name: b"",
                value: 0 as u32,
            },
        ]
    }
}
use crate::xkb::keysym::xkb_keysym_get_name;
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
