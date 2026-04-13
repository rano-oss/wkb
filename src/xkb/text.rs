pub mod context_h {
    pub use crate::xkb::context_priv::{xkb_atom_text, xkb_context_get_buffer};
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
}

pub mod xkbcommon_h {
    pub use crate::xkb::keysym::xkb_keysym_get_name;
    pub use crate::xkb::shared_types::{
        xkb_keysym_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
        xkb_state_component, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
        XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
        XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
        XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED,
        XKB_STATE_LEDS, XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub const XKB_ALL_GROUPS: u64 = ((1 as u64) << XKB_MAX_GROUPS).wrapping_sub(1 as u64);
    pub const XKB_MOD_NONE: u32 = 0xffffffff as u32;
    #[inline]
    pub unsafe fn format_boolean_controls(mut format: u32) -> xkb_action_controls {
        return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as i32 as u32 {
            CONTROL_ALL_BOOLEAN_V1 as i32
        } else {
            CONTROL_ALL_BOOLEAN as i32
        }) as xkb_action_controls;
    }
}
pub mod text_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LookupEntry {
        pub name: *const i8,
        pub value: u32,
    }
    pub type C2Rust_Unnamed_1 = u32;
    pub const CONTROL_NAMES_MIN_V2_INDEX: C2Rust_Unnamed_1 = 0;
    pub const CONTROL_NAMES_MIN_V1_INDEX: C2Rust_Unnamed_1 = 7;
    #[inline]
    pub unsafe fn format_control_names_offset(mut format: u32) -> u8 {
        return (if format as u32 == XKB_KEYMAP_FORMAT_TEXT_V1 as i32 as u32 {
            CONTROL_NAMES_MIN_V1_INDEX as i32
        } else {
            CONTROL_NAMES_MIN_V2_INDEX as i32
        }) as u8;
    }
    use super::xkbcommon_h::XKB_KEYMAP_FORMAT_TEXT_V1;
}
pub mod utils_h {
    #[inline]
    pub unsafe fn istreq(mut s1: *const i8, mut s2: *const i8) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as i32;
        }
    }
    #[inline]
    pub unsafe fn strempty(mut s: *const i8) -> *const i8 {
        return if !s.is_null() {
            s
        } else {
            b"\0".as_ptr() as *const i8
        };
    }

    pub use crate::xkb::utils::istrcmp;
}
pub mod keysym_h {
    pub const XKB_KEYSYM_NAME_MAX_SIZE: i32 = 31 as i32;
}

pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{
    xkb_atom_text, xkb_context, xkb_context_get_buffer, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::keymap_h::{
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
pub use self::keysym_h::XKB_KEYSYM_NAME_MAX_SIZE;
pub use self::text_h::{
    format_control_names_offset, C2Rust_Unnamed_1, LookupEntry, CONTROL_NAMES_MIN_V1_INDEX,
    CONTROL_NAMES_MIN_V2_INDEX,
};
pub use self::utils_h::{istrcmp, istreq, strempty};
pub use self::xkbcommon_h::{
    xkb_keysym_get_name, xkb_keysym_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t,
    xkb_rule_names, xkb_state_component, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::utils::cstr_len_safe;
pub unsafe fn LookupString(
    mut tab: *const LookupEntry,
    mut string: *const i8,
    mut value_rtrn: *mut u32,
) -> bool {
    unsafe {
        if string.is_null() {
            return 0 != 0;
        }
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(*entry).name.is_null() {
            if istreq((*entry).name, string) {
                *value_rtrn = (*entry).value as u32;
                return 1 != 0;
            }
            entry = entry.offset(1);
        }
        return 0 != 0;
    }
}
pub unsafe fn LookupValue(mut tab: *const LookupEntry, mut value: u32) -> *const i8 {
    unsafe {
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(*entry).name.is_null() {
            if (*entry).value == value as u32 {
                return (*entry).name;
            }
            entry = entry.offset(1);
        }
        return ::core::ptr::null::<i8>();
    }
}
pub static mut ctrlMaskNames: [LookupEntry; 25] = [
    LookupEntry {
        name: b"Overlay3\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY3 as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay4\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY4 as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay5\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY5 as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay6\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY6 as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay7\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY7 as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay8\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY8 as i32 as u32,
    },
    LookupEntry {
        name: b"all\0".as_ptr() as *const i8,
        value: CONTROL_ALL_BOOLEAN as i32 as u32,
    },
    LookupEntry {
        name: b"RepeatKeys\0".as_ptr() as *const i8,
        value: CONTROL_REPEAT as i32 as u32,
    },
    LookupEntry {
        name: b"Repeat\0".as_ptr() as *const i8,
        value: CONTROL_REPEAT as i32 as u32,
    },
    LookupEntry {
        name: b"AutoRepeat\0".as_ptr() as *const i8,
        value: CONTROL_REPEAT as i32 as u32,
    },
    LookupEntry {
        name: b"SlowKeys\0".as_ptr() as *const i8,
        value: CONTROL_SLOW as i32 as u32,
    },
    LookupEntry {
        name: b"BounceKeys\0".as_ptr() as *const i8,
        value: CONTROL_DEBOUNCE as i32 as u32,
    },
    LookupEntry {
        name: b"StickyKeys\0".as_ptr() as *const i8,
        value: CONTROL_STICKY_KEYS as i32 as u32,
    },
    LookupEntry {
        name: b"MouseKeys\0".as_ptr() as *const i8,
        value: CONTROL_MOUSE_KEYS as i32 as u32,
    },
    LookupEntry {
        name: b"MouseKeysAccel\0".as_ptr() as *const i8,
        value: CONTROL_MOUSE_KEYS_ACCEL as i32 as u32,
    },
    LookupEntry {
        name: b"AccessXKeys\0".as_ptr() as *const i8,
        value: CONTROL_AX as i32 as u32,
    },
    LookupEntry {
        name: b"AccessXTimeout\0".as_ptr() as *const i8,
        value: CONTROL_AX_TIMEOUT as i32 as u32,
    },
    LookupEntry {
        name: b"AccessXFeedback\0".as_ptr() as *const i8,
        value: CONTROL_AX_FEEDBACK as i32 as u32,
    },
    LookupEntry {
        name: b"AudibleBell\0".as_ptr() as *const i8,
        value: CONTROL_BELL as i32 as u32,
    },
    LookupEntry {
        name: b"IgnoreGroupLock\0".as_ptr() as *const i8,
        value: CONTROL_IGNORE_GROUP_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay1\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY1 as i32 as u32,
    },
    LookupEntry {
        name: b"Overlay2\0".as_ptr() as *const i8,
        value: CONTROL_OVERLAY2 as i32 as u32,
    },
    LookupEntry {
        name: b"all\0".as_ptr() as *const i8,
        value: CONTROL_ALL_BOOLEAN_V1 as i32 as u32,
    },
    LookupEntry {
        name: b"none\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
pub static mut modComponentMaskNames: [LookupEntry; 8] = [
    LookupEntry {
        name: b"base\0".as_ptr() as *const i8,
        value: XKB_STATE_MODS_DEPRESSED as i32 as u32,
    },
    LookupEntry {
        name: b"latched\0".as_ptr() as *const i8,
        value: XKB_STATE_MODS_LATCHED as i32 as u32,
    },
    LookupEntry {
        name: b"locked\0".as_ptr() as *const i8,
        value: XKB_STATE_MODS_LOCKED as i32 as u32,
    },
    LookupEntry {
        name: b"effective\0".as_ptr() as *const i8,
        value: XKB_STATE_MODS_EFFECTIVE as i32 as u32,
    },
    LookupEntry {
        name: b"compat\0".as_ptr() as *const i8,
        value: XKB_STATE_MODS_EFFECTIVE as i32 as u32,
    },
    LookupEntry {
        name: b"any\0".as_ptr() as *const i8,
        value: XKB_STATE_MODS_EFFECTIVE as i32 as u32,
    },
    LookupEntry {
        name: b"none\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
pub static mut groupComponentMaskNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"base\0".as_ptr() as *const i8,
        value: XKB_STATE_LAYOUT_DEPRESSED as i32 as u32,
    },
    LookupEntry {
        name: b"latched\0".as_ptr() as *const i8,
        value: XKB_STATE_LAYOUT_LATCHED as i32 as u32,
    },
    LookupEntry {
        name: b"locked\0".as_ptr() as *const i8,
        value: XKB_STATE_LAYOUT_LOCKED as i32 as u32,
    },
    LookupEntry {
        name: b"effective\0".as_ptr() as *const i8,
        value: XKB_STATE_LAYOUT_EFFECTIVE as i32 as u32,
    },
    LookupEntry {
        name: b"any\0".as_ptr() as *const i8,
        value: XKB_STATE_LAYOUT_EFFECTIVE as i32 as u32,
    },
    LookupEntry {
        name: b"none\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
pub static mut groupMaskNames: [LookupEntry; 3] = [LookupEntry {
    name: ::core::ptr::null::<i8>(),
    value: 0,
}; 3];
pub static mut buttonNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"Button1\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"Button2\0".as_ptr() as *const i8,
        value: 2 as u32,
    },
    LookupEntry {
        name: b"Button3\0".as_ptr() as *const i8,
        value: 3 as u32,
    },
    LookupEntry {
        name: b"Button4\0".as_ptr() as *const i8,
        value: 4 as u32,
    },
    LookupEntry {
        name: b"Button5\0".as_ptr() as *const i8,
        value: 5 as u32,
    },
    LookupEntry {
        name: b"default\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
pub static mut useModMapValueNames: [LookupEntry; 5] = [
    LookupEntry {
        name: b"LevelOne\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"Level1\0".as_ptr() as *const i8,
        value: 1 as u32,
    },
    LookupEntry {
        name: b"AnyLevel\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: b"any\0".as_ptr() as *const i8,
        value: 0 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
pub static mut actionTypeNames: [LookupEntry; 43] = [
    LookupEntry {
        name: b"NoAction\0".as_ptr() as *const i8,
        value: ACTION_TYPE_NONE as i32 as u32,
    },
    LookupEntry {
        name: b"VoidAction\0".as_ptr() as *const i8,
        value: ACTION_TYPE_VOID as i32 as u32,
    },
    LookupEntry {
        name: b"SetMods\0".as_ptr() as *const i8,
        value: ACTION_TYPE_MOD_SET as i32 as u32,
    },
    LookupEntry {
        name: b"LatchMods\0".as_ptr() as *const i8,
        value: ACTION_TYPE_MOD_LATCH as i32 as u32,
    },
    LookupEntry {
        name: b"LockMods\0".as_ptr() as *const i8,
        value: ACTION_TYPE_MOD_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"SetGroup\0".as_ptr() as *const i8,
        value: ACTION_TYPE_GROUP_SET as i32 as u32,
    },
    LookupEntry {
        name: b"LatchGroup\0".as_ptr() as *const i8,
        value: ACTION_TYPE_GROUP_LATCH as i32 as u32,
    },
    LookupEntry {
        name: b"LockGroup\0".as_ptr() as *const i8,
        value: ACTION_TYPE_GROUP_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"MovePtr\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_MOVE as i32 as u32,
    },
    LookupEntry {
        name: b"MovePointer\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_MOVE as i32 as u32,
    },
    LookupEntry {
        name: b"PtrBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_BUTTON as i32 as u32,
    },
    LookupEntry {
        name: b"PointerButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_BUTTON as i32 as u32,
    },
    LookupEntry {
        name: b"LockPtrBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"LockPtrButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"LockPointerButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"LockPointerBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"SetPtrDflt\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_DEFAULT as i32 as u32,
    },
    LookupEntry {
        name: b"SetPointerDefault\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PTR_DEFAULT as i32 as u32,
    },
    LookupEntry {
        name: b"Terminate\0".as_ptr() as *const i8,
        value: ACTION_TYPE_TERMINATE as i32 as u32,
    },
    LookupEntry {
        name: b"TerminateServer\0".as_ptr() as *const i8,
        value: ACTION_TYPE_TERMINATE as i32 as u32,
    },
    LookupEntry {
        name: b"SwitchScreen\0".as_ptr() as *const i8,
        value: ACTION_TYPE_SWITCH_VT as i32 as u32,
    },
    LookupEntry {
        name: b"SetControls\0".as_ptr() as *const i8,
        value: ACTION_TYPE_CTRL_SET as i32 as u32,
    },
    LookupEntry {
        name: b"LockControls\0".as_ptr() as *const i8,
        value: ACTION_TYPE_CTRL_LOCK as i32 as u32,
    },
    LookupEntry {
        name: b"RedirectKey\0".as_ptr() as *const i8,
        value: ACTION_TYPE_REDIRECT_KEY as i32 as u32,
    },
    LookupEntry {
        name: b"Redirect\0".as_ptr() as *const i8,
        value: ACTION_TYPE_REDIRECT_KEY as i32 as u32,
    },
    LookupEntry {
        name: b"Private\0".as_ptr() as *const i8,
        value: ACTION_TYPE_PRIVATE as i32 as u32,
    },
    LookupEntry {
        name: b"ISOLock\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"ActionMessage\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"MessageAction\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"Message\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DeviceBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DevBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DevButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DeviceButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"LockDeviceBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"LockDevBtn\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"LockDevButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"LockDeviceButton\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DeviceValuator\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DevVal\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DeviceVal\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: b"DevValuator\0".as_ptr() as *const i8,
        value: ACTION_TYPE_UNSUPPORTED_LEGACY as i32 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
        value: 0 as u32,
    },
];
pub static mut symInterpretMatchMaskNames: [LookupEntry; 6] = [
    LookupEntry {
        name: b"NoneOf\0".as_ptr() as *const i8,
        value: MATCH_NONE as i32 as u32,
    },
    LookupEntry {
        name: b"AnyOfOrNone\0".as_ptr() as *const i8,
        value: MATCH_ANY_OR_NONE as i32 as u32,
    },
    LookupEntry {
        name: b"AnyOf\0".as_ptr() as *const i8,
        value: MATCH_ANY as i32 as u32,
    },
    LookupEntry {
        name: b"AllOf\0".as_ptr() as *const i8,
        value: MATCH_ALL as i32 as u32,
    },
    LookupEntry {
        name: b"Exactly\0".as_ptr() as *const i8,
        value: MATCH_EXACTLY as i32 as u32,
    },
    LookupEntry {
        name: ::core::ptr::null::<i8>(),
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
            return ::core::ptr::null::<i8>();
        }
        return xkb_atom_text(ctx, (*mods).mods[ndx as usize].name);
    }
}
pub unsafe fn ActionTypeText(mut type_0: xkb_action_type) -> *const i8 {
    unsafe {
        let mut name: *const i8 = LookupValue(
            &raw const actionTypeNames as *const LookupEntry,
            type_0 as u32,
        );
        return if !name.is_null() {
            name
        } else {
            b"Private\0".as_ptr() as *const i8
        };
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
        let mut len: usize = cstr_len_safe(sname).wrapping_add(3 as usize);
        let mut buf: *mut i8 = xkb_context_get_buffer(ctx, len);
        crate::xkb::utils::snprintf_args(
            buf,
            len,
            format_args!("<{}>", crate::xkb::utils::CStrDisplay(strempty(sname))),
        );
        return buf;
    }
}
pub unsafe fn SIMatchText(mut type_0: xkb_match_operation) -> *const i8 {
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
            0 as i32 as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut pos: usize = 0 as usize;
        let mut mod_0: *const xkb_mod = ::core::ptr::null::<xkb_mod>();
        if mask == 0 as xkb_mod_mask_t {
            return b"none\0".as_ptr() as *const i8;
        }
        if mask == MOD_REAL_MASK_ALL {
            return b"all\0".as_ptr() as *const i8;
        }
        if type_0 as u32 == MOD_REAL as i32 as u32 && mask & !MOD_REAL_MASK_ALL != 0
            || (mask as u64 & !((1 as u64) << (*mods).num_mods).wrapping_sub(1 as u64) != 0) as i32
                as i64
                != 0
        {
            let (written, _) = crate::xkb::utils::snprintf_args(
                &raw mut buf as *mut i8,
                ::core::mem::size_of::<[i8; 1024]>() as usize,
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
                        (::core::mem::size_of::<[i8; 1024]>() as usize).wrapping_sub(pos),
                        format_args!(
                            "{}{}",
                            if pos == 0 as usize { "" } else { "+" },
                            crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*mod_0).name)),
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

pub unsafe fn LedStateMaskText(
    mut ctx: *mut xkb_context,
    mut lookup: *const LookupEntry,
    mut mask: xkb_state_component,
) -> *const i8 {
    unsafe {
        let mut buf: [i8; 1024] = [0; 1024];
        let mut pos: usize = 0 as usize;
        if mask as u32 == 0 as u32 {
            return b"0\0".as_ptr() as *const i8;
        }
        let mut i: u32 = 0 as u32;
        while mask as u64 != 0 {
            let mut ret: i32 = 0;
            if !(mask as u32 & (1 as u32) << i == 0) {
                mask = (mask as u32 & !((1 as u32) << i)) as xkb_state_component;
                let maskText: *const i8 =
                    LookupValue(lookup as *const LookupEntry, (1 as u32) << i) as *const i8;
                let (written, trunc) = crate::xkb::utils::snprintf_args(
                    (&raw mut buf as *mut i8).offset(pos as isize),
                    (::core::mem::size_of::<[i8; 1024]>() as usize).wrapping_sub(pos),
                    format_args!(
                        "{}{}",
                        if pos == 0 as usize { "" } else { "+" },
                        crate::xkb::utils::CStrDisplay(maskText),
                    ),
                );
                ret = if trunc { -1 } else { written as i32 };
                if ret <= 0 as i32
                    || pos.wrapping_add(ret as usize)
                        >= ::core::mem::size_of::<[i8; 1024]>() as usize
                {
                    break;
                }
                pos = pos.wrapping_add(ret as usize);
            }
            i = i.wrapping_add(1);
        }
        pos = pos.wrapping_add(1);
        let dst = xkb_context_get_buffer(ctx, pos);
        std::ptr::copy_nonoverlapping(&raw mut buf as *const u8, dst as *mut u8, pos);
        return dst as *const i8;
    }
}

pub unsafe fn ControlMaskText(
    mut ctx: *mut xkb_context,
    mut format: u32,
    mut mask: xkb_action_controls,
) -> *const i8 {
    unsafe {
        let mut buf: [i8; 1024] = [0; 1024];
        let mut pos: usize = 0 as usize;
        let all_ctrls: xkb_action_controls = format_boolean_controls(format) as xkb_action_controls;
        mask = (mask as u32 & all_ctrls as u32) as xkb_action_controls;
        if mask as u32 == 0 as u32 {
            return b"none\0".as_ptr() as *const i8;
        }
        if mask as u32 == all_ctrls as u32 {
            return b"all\0".as_ptr() as *const i8;
        }
        let control_names_offset: u8 = format_control_names_offset(format) as u8;
        let mut i: u32 = 0 as u32;
        while mask as u64 != 0 {
            let mut ret: i32 = 0;
            if !(mask as u32 & (1 as u32) << i == 0) {
                mask = (mask as u32 & !((1 as u32) << i)) as xkb_action_controls;
                let maskText: *const i8 = LookupValue(
                    (&raw const ctrlMaskNames as *const LookupEntry)
                        .offset(control_names_offset as i32 as isize),
                    (1 as u32) << i,
                ) as *const i8;
                let (written, trunc) = crate::xkb::utils::snprintf_args(
                    (&raw mut buf as *mut i8).offset(pos as isize),
                    (::core::mem::size_of::<[i8; 1024]>() as usize).wrapping_sub(pos),
                    format_args!(
                        "{}{}",
                        if pos == 0 as usize { "" } else { "+" },
                        crate::xkb::utils::CStrDisplay(maskText),
                    ),
                );
                ret = if trunc { -1 } else { written as i32 };
                if ret <= 0 as i32
                    || pos.wrapping_add(ret as usize)
                        >= ::core::mem::size_of::<[i8; 1024]>() as usize
                {
                    break;
                }
                pos = pos.wrapping_add(ret as usize);
            }
            i = i.wrapping_add(1);
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
                name: b"none\0".as_ptr() as *const i8,
                value: 0 as u32,
            },
            LookupEntry {
                name: b"all\0".as_ptr() as *const i8,
                value: XKB_ALL_GROUPS as u32,
            },
            LookupEntry {
                name: ::core::ptr::null::<i8>(),
                value: 0 as u32,
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
