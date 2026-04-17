use crate::xkb::context::{xkb_atom_text_bytes, xkb_context_get_buffer};

// Was in text_h module — now at file level
#[derive(Copy, Clone)]
pub struct LookupEntry {
    pub name: &'static [u8],
    pub value: u32,
}
pub const CONTROL_NAMES_MIN_V2_INDEX: u32 = 0;
pub const CONTROL_NAMES_MIN_V1_INDEX: u32 = 7;
pub const GROUP_LAST_INDEX_NAME: &[u8] = b"last";
#[inline]
pub unsafe fn format_control_names_offset(format: u32) -> u8 {
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
pub unsafe fn LookupString(tab: *const LookupEntry, string: &[u8], value_rtrn: *mut u32) -> bool {
    unsafe {
        if string.is_empty() {
            return false;
        }
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(&(*entry).name).is_empty() {
            if (*entry).name.eq_ignore_ascii_case(string) {
                *value_rtrn = (*entry).value;
                return true;
            }
            entry = entry.offset(1);
        }
        false
    }
}
pub unsafe fn LookupValue(tab: *const LookupEntry, value: u32) -> &'static [u8] {
    unsafe {
        let mut entry: *const LookupEntry = tab as *const LookupEntry;
        while !(&(*entry).name).is_empty() {
            if (*entry).value == value {
                return (*entry).name;
            }
            entry = entry.offset(1);
        }
        b""
    }
}
pub static mut ctrlMaskNames: [LookupEntry; 25] = [
    LookupEntry {
        name: b"Overlay3",
        value: CONTROL_OVERLAY3,
    },
    LookupEntry {
        name: b"Overlay4",
        value: CONTROL_OVERLAY4,
    },
    LookupEntry {
        name: b"Overlay5",
        value: CONTROL_OVERLAY5,
    },
    LookupEntry {
        name: b"Overlay6",
        value: CONTROL_OVERLAY6,
    },
    LookupEntry {
        name: b"Overlay7",
        value: CONTROL_OVERLAY7,
    },
    LookupEntry {
        name: b"Overlay8",
        value: CONTROL_OVERLAY8,
    },
    LookupEntry {
        name: b"all",
        value: CONTROL_ALL_BOOLEAN,
    },
    LookupEntry {
        name: b"RepeatKeys",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: b"Repeat",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: b"AutoRepeat",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: b"SlowKeys",
        value: CONTROL_SLOW,
    },
    LookupEntry {
        name: b"BounceKeys",
        value: CONTROL_DEBOUNCE,
    },
    LookupEntry {
        name: b"StickyKeys",
        value: CONTROL_STICKY_KEYS,
    },
    LookupEntry {
        name: b"MouseKeys",
        value: CONTROL_MOUSE_KEYS,
    },
    LookupEntry {
        name: b"MouseKeysAccel",
        value: CONTROL_MOUSE_KEYS_ACCEL,
    },
    LookupEntry {
        name: b"AccessXKeys",
        value: CONTROL_AX,
    },
    LookupEntry {
        name: b"AccessXTimeout",
        value: CONTROL_AX_TIMEOUT,
    },
    LookupEntry {
        name: b"AccessXFeedback",
        value: CONTROL_AX_FEEDBACK,
    },
    LookupEntry {
        name: b"AudibleBell",
        value: CONTROL_BELL,
    },
    LookupEntry {
        name: b"IgnoreGroupLock",
        value: CONTROL_IGNORE_GROUP_LOCK,
    },
    LookupEntry {
        name: b"Overlay1",
        value: CONTROL_OVERLAY1,
    },
    LookupEntry {
        name: b"Overlay2",
        value: CONTROL_OVERLAY2,
    },
    LookupEntry {
        name: b"all",
        value: CONTROL_ALL_BOOLEAN_V1,
    },
    LookupEntry {
        name: b"none",
        value: 0_u32,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub static mut modComponentMaskNames: [LookupEntry; 8] = [
    LookupEntry {
        name: b"base",
        value: XKB_STATE_MODS_DEPRESSED,
    },
    LookupEntry {
        name: b"latched",
        value: XKB_STATE_MODS_LATCHED,
    },
    LookupEntry {
        name: b"locked",
        value: XKB_STATE_MODS_LOCKED,
    },
    LookupEntry {
        name: b"effective",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: b"compat",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: b"any",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: b"none",
        value: 0_u32,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub static mut groupComponentMaskNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"base",
        value: XKB_STATE_LAYOUT_DEPRESSED,
    },
    LookupEntry {
        name: b"latched",
        value: XKB_STATE_LAYOUT_LATCHED,
    },
    LookupEntry {
        name: b"locked",
        value: XKB_STATE_LAYOUT_LOCKED,
    },
    LookupEntry {
        name: b"effective",
        value: XKB_STATE_LAYOUT_EFFECTIVE,
    },
    LookupEntry {
        name: b"any",
        value: XKB_STATE_LAYOUT_EFFECTIVE,
    },
    LookupEntry {
        name: b"none",
        value: 0_u32,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub static mut groupMaskNames: [LookupEntry; 3] = [LookupEntry {
    name: b"",
    value: 0,
}; 3];
pub static mut buttonNames: [LookupEntry; 7] = [
    LookupEntry {
        name: b"Button1",
        value: 1_u32,
    },
    LookupEntry {
        name: b"Button2",
        value: 2_u32,
    },
    LookupEntry {
        name: b"Button3",
        value: 3_u32,
    },
    LookupEntry {
        name: b"Button4",
        value: 4_u32,
    },
    LookupEntry {
        name: b"Button5",
        value: 5_u32,
    },
    LookupEntry {
        name: b"default",
        value: 0_u32,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub static mut useModMapValueNames: [LookupEntry; 5] = [
    LookupEntry {
        name: b"LevelOne",
        value: 1_u32,
    },
    LookupEntry {
        name: b"Level1",
        value: 1_u32,
    },
    LookupEntry {
        name: b"AnyLevel",
        value: 0_u32,
    },
    LookupEntry {
        name: b"any",
        value: 0_u32,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub static mut actionTypeNames: [LookupEntry; 43] = [
    LookupEntry {
        name: b"NoAction",
        value: ACTION_TYPE_NONE,
    },
    LookupEntry {
        name: b"VoidAction",
        value: ACTION_TYPE_VOID,
    },
    LookupEntry {
        name: b"SetMods",
        value: ACTION_TYPE_MOD_SET,
    },
    LookupEntry {
        name: b"LatchMods",
        value: ACTION_TYPE_MOD_LATCH,
    },
    LookupEntry {
        name: b"LockMods",
        value: ACTION_TYPE_MOD_LOCK,
    },
    LookupEntry {
        name: b"SetGroup",
        value: ACTION_TYPE_GROUP_SET,
    },
    LookupEntry {
        name: b"LatchGroup",
        value: ACTION_TYPE_GROUP_LATCH,
    },
    LookupEntry {
        name: b"LockGroup",
        value: ACTION_TYPE_GROUP_LOCK,
    },
    LookupEntry {
        name: b"MovePtr",
        value: ACTION_TYPE_PTR_MOVE,
    },
    LookupEntry {
        name: b"MovePointer",
        value: ACTION_TYPE_PTR_MOVE,
    },
    LookupEntry {
        name: b"PtrBtn",
        value: ACTION_TYPE_PTR_BUTTON,
    },
    LookupEntry {
        name: b"PointerButton",
        value: ACTION_TYPE_PTR_BUTTON,
    },
    LookupEntry {
        name: b"LockPtrBtn",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: b"LockPtrButton",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: b"LockPointerButton",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: b"LockPointerBtn",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: b"SetPtrDflt",
        value: ACTION_TYPE_PTR_DEFAULT,
    },
    LookupEntry {
        name: b"SetPointerDefault",
        value: ACTION_TYPE_PTR_DEFAULT,
    },
    LookupEntry {
        name: b"Terminate",
        value: ACTION_TYPE_TERMINATE,
    },
    LookupEntry {
        name: b"TerminateServer",
        value: ACTION_TYPE_TERMINATE,
    },
    LookupEntry {
        name: b"SwitchScreen",
        value: ACTION_TYPE_SWITCH_VT,
    },
    LookupEntry {
        name: b"SetControls",
        value: ACTION_TYPE_CTRL_SET,
    },
    LookupEntry {
        name: b"LockControls",
        value: ACTION_TYPE_CTRL_LOCK,
    },
    LookupEntry {
        name: b"RedirectKey",
        value: ACTION_TYPE_REDIRECT_KEY,
    },
    LookupEntry {
        name: b"Redirect",
        value: ACTION_TYPE_REDIRECT_KEY,
    },
    LookupEntry {
        name: b"Private",
        value: ACTION_TYPE_PRIVATE,
    },
    LookupEntry {
        name: b"ISOLock",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"ActionMessage",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"MessageAction",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"Message",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"LockDeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"LockDevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"LockDevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"LockDeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DeviceValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DevVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DeviceVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"DevValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub static mut symInterpretMatchMaskNames: [LookupEntry; 6] = [
    LookupEntry {
        name: b"NoneOf",
        value: MATCH_NONE,
    },
    LookupEntry {
        name: b"AnyOfOrNone",
        value: MATCH_ANY_OR_NONE,
    },
    LookupEntry {
        name: b"AnyOf",
        value: MATCH_ANY,
    },
    LookupEntry {
        name: b"AllOf",
        value: MATCH_ALL,
    },
    LookupEntry {
        name: b"Exactly",
        value: MATCH_EXACTLY,
    },
    LookupEntry {
        name: b"",
        value: 0_u32,
    },
];
pub unsafe fn ModIndexText(
    ctx: *mut xkb_context,
    mods: *const xkb_mod_set,
    ndx: u32,
) -> &'static [u8] {
    unsafe {
        if ndx == XKB_MOD_INVALID {
            return b"none";
        }
        if ndx == XKB_MOD_NONE {
            return b"None";
        }
        if ndx >= (*mods).num_mods {
            return b"";
        }
        xkb_atom_text_bytes(&(*ctx).atom_table, (*mods).mods[ndx as usize].name)
    }
}
pub unsafe fn ActionTypeText(type_0: xkb_action_type) -> &'static [u8] {
    unsafe {
        let name: &'static [u8] =
            LookupValue(&raw const actionTypeNames as *const LookupEntry, type_0);
        if !name.is_empty() {
            name
        } else {
            b"Private"
        }
    }
}
pub unsafe fn KeysymText(mut ctx: xkb_context, sym: u32) -> &'static [u8] {
    unsafe {
        let buffer: *mut i8 = xkb_context_get_buffer(&mut ctx, XKB_KEYSYM_NAME_MAX_SIZE as usize);
        xkb_keysym_get_name(sym, buffer, XKB_KEYSYM_NAME_MAX_SIZE as usize);
        cstr_as_bytes(buffer)
    }
}
pub unsafe fn KeyNameText(mut ctx: xkb_context, name: u32) -> &'static [u8] {
    unsafe {
        let atom_table = &ctx.atom_table.clone();
        let sname: &[u8] = xkb_atom_text_bytes(atom_table, name);
        let sname_str = std::str::from_utf8(sname).unwrap_or("");
        let len: usize = sname_str.len().wrapping_add(3_usize);
        let buf: *mut i8 = xkb_context_get_buffer(&mut ctx, len);
        let (written, _) =
            crate::xkb::utils::snprintf_args(buf, len, format_args!("<{}>", sname_str));
        std::slice::from_raw_parts(buf as *const u8, written)
    }
}
pub unsafe fn SIMatchText(type_0: u32) -> &'static [u8] {
    unsafe {
        LookupValue(
            &raw const symInterpretMatchMaskNames as *const LookupEntry,
            type_0,
        )
    }
}
pub unsafe fn ModMaskText(
    mut ctx: xkb_context,
    type_0: u32,
    mods: *const xkb_mod_set,
    mut mask: u32,
) -> &'static [u8] {
    unsafe {
        let mut buf: [i8; 1024] = [0; 1024];
        let mut pos: usize = 0_usize;
        let mut mod_0: *const xkb_mod;
        if mask == 0_u32 {
            return b"none";
        }
        if mask == MOD_REAL_MASK_ALL {
            return b"all";
        }
        if type_0 == MOD_REAL && mask & !MOD_REAL_MASK_ALL != 0
            || (mask as u64 & !(1_u64 << (*mods).num_mods).wrapping_sub(1_u64) != 0) as i32 as i64
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
                if mask & 0x1_u32 != 0 {
                    let (written, trunc) = crate::xkb::utils::snprintf_args(
                        (&raw mut buf as *mut i8).add(pos),
                        (std::mem::size_of::<[i8; 1024]>()).wrapping_sub(pos),
                        format_args!(
                            "{}{}",
                            if pos == 0_usize { "" } else { "+" },
                            crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(
                                &ctx.atom_table,
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
                mask >>= 1_i32;
            }
        }
        let dst = xkb_context_get_buffer(&mut ctx, pos);
        std::ptr::copy_nonoverlapping(&raw mut buf as *const u8, dst as *mut u8, pos);
        std::slice::from_raw_parts(dst as *const u8, pos)
    }
}

unsafe fn c2rust_run_static_initializers() {
    unsafe {
        groupMaskNames = [
            LookupEntry {
                name: b"none",
                value: 0_u32,
            },
            LookupEntry {
                name: b"all",
                value: XKB_ALL_GROUPS as u32,
            },
            LookupEntry {
                name: b"",
                value: 0_u32,
            },
        ]
    }
}
use crate::xkb::keysym::xkb_keysym_get_name;
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
