use super::prelude::*;
pub use crate::xkb::keymap::action_equal;
use crate::xkb::text::{actionTypeNames, ctrlMaskNames, LookupString, LookupValue};
pub use crate::xkb::xkbcomp::expr::{ExprResolveButton, ExprResolveInteger, ExprResolveMask};
#[derive(Copy, Clone)]
pub struct ActionsInfo {
    pub actions: [xkb_action; 21],
}

// impl ActionsInfo {
//     pub fn new(mut keymap: &xkb_keymap) {
//         let mut type_0 = ACTION_TYPE_NONE;
//         let actions: [xkb_action; 21];
//         for a in &actions {
//             a.type_0
//         }
//         while type_0 < _ACTION_TYPE_NUM_ENTRIES {
//             actions[type_0 as usize].type_0 = type_0;
//             type_0 += 1;
//         }
//         actions[ACTION_TYPE_PTR_DEFAULT as usize].dflt.flags = 0 as xkb_action_flags;
//         actions[ACTION_TYPE_PTR_DEFAULT as usize].dflt.value = 1 as i8;
//         actions[ACTION_TYPE_PTR_MOVE as usize].ptr.flags = ACTION_ACCEL;
//         actions[ACTION_TYPE_SWITCH_VT as usize].screen.flags = ACTION_SAME_SCREEN;
//         actions[ACTION_TYPE_REDIRECT_KEY as usize].redirect.keycode = (*keymap).redirect_key_auto;
//     }
// }
pub const ACTION_FIELD_LATCH_ON_PRESS: u32 = 25;
pub const ACTION_FIELD_UNLOCK_ON_PRESS: u32 = 24;
pub const ACTION_FIELD_LOCK_ON_RELEASE: u32 = 23;
pub const ACTION_FIELD_MODS_TO_CLEAR: u32 = 22;
pub const ACTION_FIELD_KEYCODE: u32 = 21;
pub const ACTION_FIELD_DEVICE: u32 = 20;
pub const ACTION_FIELD_DATA: u32 = 19;
pub const ACTION_FIELD_SAME: u32 = 18;
pub const ACTION_FIELD_SCREEN: u32 = 17;
pub const ACTION_FIELD_COUNT: u32 = 16;
pub const ACTION_FIELD_TYPE: u32 = 15;
pub const ACTION_FIELD_CONTROLS: u32 = 14;
pub const ACTION_FIELD_VALUE: u32 = 13;
pub const ACTION_FIELD_BUTTON: u32 = 12;
pub const ACTION_FIELD_ACCEL: u32 = 11;
pub const ACTION_FIELD_Y: u32 = 10;
pub const ACTION_FIELD_X: u32 = 9;
pub const ACTION_FIELD_GROUP: u32 = 8;
pub const ACTION_FIELD_MODIFIERS: u32 = 7;
pub const ACTION_FIELD_INCREMENT: u32 = 6;
pub const ACTION_FIELD_AFFECT: u32 = 5;
pub const ACTION_FIELD_DEFAULT: u32 = 4;
pub const ACTION_FIELD_REPORT: u32 = 3;
pub const ACTION_FIELD_GEN_KEY_EVENT: u32 = 2;
pub const ACTION_FIELD_LATCH_TO_LOCK: u32 = 1;
pub const ACTION_FIELD_CLEAR_LOCKS: u32 = 0;
pub type actionHandler = Option<
    unsafe fn(
        *const xkb_keymap_info,
        *const xkb_mod_set,
        *mut xkb_action,
        u32,
        *const ExprDef,
        *const ExprDef,
        *mut *mut ExprDef,
    ) -> xkb_parser_error,
>;
// SAFETY: ExprBoolean contains *mut _ParseCommon which is always null and never dereferenced mutably.
// These are read-only constants used only via `&raw const`.
struct SyncExprBoolean(ExprBoolean);
unsafe impl Sync for SyncExprBoolean {}

static CONST_TRUE: SyncExprBoolean = SyncExprBoolean(ExprBoolean {
    common: _ParseCommon {
        next: std::ptr::null_mut(),
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: true,
});
static CONST_FALSE: SyncExprBoolean = SyncExprBoolean(ExprBoolean {
    common: _ParseCommon {
        next: std::ptr::null_mut(),
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: false,
});
pub unsafe fn InitActionsInfo(keymap: *const xkb_keymap, info: *mut ActionsInfo) {
    unsafe {
        let mut type_0: xkb_action_type = ACTION_TYPE_NONE;
        while (type_0 as u32) < _ACTION_TYPE_NUM_ENTRIES {
            (*info).actions[type_0 as usize].type_0 = type_0;
            type_0 += 1;
        }
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as usize].dflt.flags = 0 as xkb_action_flags;
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as usize].dflt.value = 1_i8;
        (*info).actions[ACTION_TYPE_PTR_MOVE as usize].ptr.flags = ACTION_ACCEL;
        (*info).actions[ACTION_TYPE_SWITCH_VT as usize].screen.flags = ACTION_SAME_SCREEN;
        (*info).actions[ACTION_TYPE_REDIRECT_KEY as usize]
            .redirect
            .keycode = (*keymap).redirect_key_auto;
    }
}
static FIELD_STRINGS: [LookupEntry; 37] = [
    LookupEntry {
        name: "clearLocks",
        value: ACTION_FIELD_CLEAR_LOCKS,
    },
    LookupEntry {
        name: "latchToLock",
        value: ACTION_FIELD_LATCH_TO_LOCK,
    },
    LookupEntry {
        name: "genKeyEvent",
        value: ACTION_FIELD_GEN_KEY_EVENT,
    },
    LookupEntry {
        name: "generateKeyEvent",
        value: ACTION_FIELD_GEN_KEY_EVENT,
    },
    LookupEntry {
        name: "report",
        value: ACTION_FIELD_REPORT,
    },
    LookupEntry {
        name: "default",
        value: ACTION_FIELD_DEFAULT,
    },
    LookupEntry {
        name: "affect",
        value: ACTION_FIELD_AFFECT,
    },
    LookupEntry {
        name: "increment",
        value: ACTION_FIELD_INCREMENT,
    },
    LookupEntry {
        name: "modifiers",
        value: ACTION_FIELD_MODIFIERS,
    },
    LookupEntry {
        name: "mods",
        value: ACTION_FIELD_MODIFIERS,
    },
    LookupEntry {
        name: "group",
        value: ACTION_FIELD_GROUP,
    },
    LookupEntry {
        name: "x",
        value: ACTION_FIELD_X,
    },
    LookupEntry {
        name: "y",
        value: ACTION_FIELD_Y,
    },
    LookupEntry {
        name: "accel",
        value: ACTION_FIELD_ACCEL,
    },
    LookupEntry {
        name: "accelerate",
        value: ACTION_FIELD_ACCEL,
    },
    LookupEntry {
        name: "repeat",
        value: ACTION_FIELD_ACCEL,
    },
    LookupEntry {
        name: "button",
        value: ACTION_FIELD_BUTTON,
    },
    LookupEntry {
        name: "value",
        value: ACTION_FIELD_VALUE,
    },
    LookupEntry {
        name: "controls",
        value: ACTION_FIELD_CONTROLS,
    },
    LookupEntry {
        name: "ctrls",
        value: ACTION_FIELD_CONTROLS,
    },
    LookupEntry {
        name: "type",
        value: ACTION_FIELD_TYPE,
    },
    LookupEntry {
        name: "count",
        value: ACTION_FIELD_COUNT,
    },
    LookupEntry {
        name: "screen",
        value: ACTION_FIELD_SCREEN,
    },
    LookupEntry {
        name: "same",
        value: ACTION_FIELD_SAME,
    },
    LookupEntry {
        name: "sameServer",
        value: ACTION_FIELD_SAME,
    },
    LookupEntry {
        name: "data",
        value: ACTION_FIELD_DATA,
    },
    LookupEntry {
        name: "device",
        value: ACTION_FIELD_DEVICE,
    },
    LookupEntry {
        name: "dev",
        value: ACTION_FIELD_DEVICE,
    },
    LookupEntry {
        name: "key",
        value: ACTION_FIELD_KEYCODE,
    },
    LookupEntry {
        name: "keycode",
        value: ACTION_FIELD_KEYCODE,
    },
    LookupEntry {
        name: "kc",
        value: ACTION_FIELD_KEYCODE,
    },
    LookupEntry {
        name: "clearmods",
        value: ACTION_FIELD_MODS_TO_CLEAR,
    },
    LookupEntry {
        name: "clearmodifiers",
        value: ACTION_FIELD_MODS_TO_CLEAR,
    },
    LookupEntry {
        name: "lockOnRelease",
        value: ACTION_FIELD_LOCK_ON_RELEASE,
    },
    LookupEntry {
        name: "unlockOnPress",
        value: ACTION_FIELD_UNLOCK_ON_PRESS,
    },
    LookupEntry {
        name: "latchOnPress",
        value: ACTION_FIELD_LATCH_ON_PRESS,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
unsafe fn stringToActionType(str: &str, type_rtrn: *mut xkb_action_type) -> bool {
    unsafe {
        let mut type_0: u32 = 0_u32;
        let ret: bool = LookupString(&actionTypeNames, str, &mut type_0);
        *type_rtrn = type_0 as xkb_action_type;
        ret
    }
}
unsafe fn stringToField(str: &str, field_rtrn: *mut u32) -> bool {
    unsafe {
        let mut field: u32 = 0_u32;
        let ret: bool = LookupString(&FIELD_STRINGS, str, &mut field);
        *field_rtrn = field;
        ret
    }
}
unsafe fn fieldText(field: u32) -> &'static str {
    unsafe { LookupValue(&FIELD_STRINGS, field) }
}
#[inline]
unsafe fn ReportMismatch(
    _ctx: *mut xkb_context,
    code: xkb_message_code,
    action: xkb_action_type,
    field: u32,
    type_0: *const i8,
    strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Value of {} field must be of type {}; Action {} definition ignored\n",
            { code },
            fieldText(field),
            crate::xkb::utils::CStrDisplay(type_0),
            ActionTypeText(action),
        );
        (if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error
    }
}
#[inline]
unsafe fn ReportFormatVersionMismatch(
    _ctx: *mut xkb_context,
    action: xkb_action_type,
    field: u32,
    format: u32,
    versions: *const i8,
    strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Field {} for an action of type {} requires keymap text format {},  but got: {}; Action definition ignored\n",
            XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT as i32,
            fieldText(field),
            ActionTypeText(action),
            crate::xkb::utils::CStrDisplay(versions),
            { format },
        );
        (if strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error
    }
}
#[inline]
unsafe fn ReportIllegal(
    _ctx: *mut xkb_context,
    action: xkb_action_type,
    field: u32,
    strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Field {} is not defined for an action of type {}; Action definition ignored\n",
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            fieldText(field),
            ActionTypeText(action),
        );
        (if strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error
    }
}
#[inline]
unsafe fn ReportActionNotArray(
    _ctx: *mut xkb_context,
    action: xkb_action_type,
    field: u32,
    strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] The {} field in the {} action is not an array; Action definition ignored\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            fieldText(field),
            ActionTypeText(action),
        );
        (if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error
    }
}
unsafe fn HandleNoAction(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    _array_ndx: *const ExprDef,
    _value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            (*(*keymap_info).keymap).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] The \"{}\" action takes no argument, but got \"{}\" field; Action definition ignored\n",
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            ActionTypeText((*action).type_0),
            fieldText(field),
        );
        (if (*keymap_info).strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error
    }
}
unsafe fn CheckBooleanFlag(
    ctx: *mut xkb_context,
    strict: xkb_parser_strict_flags,
    action: xkb_action_type,
    field: u32,
    flag: xkb_action_flags,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        let mut set: bool = false;
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, field, strict);
        }
        if !ExprResolveBoolean(ctx, value, &raw mut set) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                field,
                b"boolean\0".as_ptr() as *const i8,
                strict,
            );
        }
        if set {
            *flags_inout = (*flags_inout | flag) as xkb_action_flags;
        } else {
            *flags_inout = (*flags_inout & !flag) as xkb_action_flags;
        }
        PARSER_SUCCESS
    }
}
unsafe fn CheckModifierField(
    ctx: *mut xkb_context,
    strict: xkb_parser_strict_flags,
    mods: *const xkb_mod_set,
    action: xkb_action_type,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    flags_inout: *mut xkb_action_flags,
    mods_rtrn: *mut u32,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_MODIFIERS, strict);
        }
        if (*value).common.type_0 == STMT_EXPR_IDENT {
            let valStr: &str = xkb_atom_text(&(*ctx).atom_table, (*value).ident.ident);
            if !valStr.is_empty()
                && (valStr.eq_ignore_ascii_case("usemodmapmods")
                    || valStr.eq_ignore_ascii_case("modmapmods"))
            {
                *mods_rtrn = 0_u32;
                *flags_inout = (*flags_inout | ACTION_MODS_LOOKUP_MODMAP) as xkb_action_flags;
                return PARSER_SUCCESS;
            }
        }
        if !ExprResolveModMask(ctx, value, MOD_BOTH, mods, mods_rtrn) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_MODIFIERS,
                b"modifier mask\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout =
            (*flags_inout & !(ACTION_MODS_LOOKUP_MODMAP as i32) as u32) as xkb_action_flags;
        PARSER_SUCCESS
    }
}
static LOCK_WHICH: [LookupEntry; 5] = [
    LookupEntry {
        name: "both",
        value: 0_u32,
    },
    LookupEntry {
        name: "lock",
        value: ACTION_LOCK_NO_UNLOCK,
    },
    LookupEntry {
        name: "neither",
        value: (ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32,
    },
    LookupEntry {
        name: "unlock",
        value: ACTION_LOCK_NO_LOCK,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
unsafe fn CheckAffectField(
    ctx: *mut xkb_context,
    strict: xkb_parser_strict_flags,
    action: xkb_action_type,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_AFFECT, strict);
        }
        let mut flags: u32 = 0_u32;
        if !ExprResolveEnum(ctx, value, &raw mut flags, &LOCK_WHICH) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_AFFECT,
                b"lock, unlock, both, neither\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout = (*flags_inout
            & !(ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32)
            as xkb_action_flags;
        *flags_inout = (*flags_inout | flags as xkb_action_flags) as xkb_action_flags;
        PARSER_SUCCESS
    }
}
unsafe fn HandleSetLatchLockMods(
    keymap_info: *const xkb_keymap_info,
    mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_mod_action = &raw mut (*action).mods;
        let type_0: xkb_action_type = (*action).type_0;
        if field == ACTION_FIELD_MODIFIERS {
            return CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
                &raw mut (*act).mods.mods,
            );
        }
        if field == ACTION_FIELD_UNLOCK_ON_PRESS {
            if (*keymap_info).features.mods_unlock_on_press {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_UNLOCK_ON_PRESS,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    ctx,
                    (*action).type_0,
                    field,
                    (*(*keymap_info).keymap).format,
                    b">= 2\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
        }
        if (type_0 as u32 == ACTION_TYPE_MOD_SET || type_0 as u32 == ACTION_TYPE_MOD_LATCH)
            && field == ACTION_FIELD_CLEAR_LOCKS
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LOCK_CLEAR,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_MOD_LATCH {
            if field == ACTION_FIELD_LATCH_TO_LOCK {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_LATCH_TO_LOCK,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            }
            if field == ACTION_FIELD_LATCH_ON_PRESS {
                if (*keymap_info).features.mods_latch_on_press {
                    return CheckBooleanFlag(
                        ctx,
                        (*keymap_info).strict,
                        (*action).type_0,
                        field,
                        ACTION_LATCH_ON_PRESS,
                        array_ndx,
                        value,
                        &raw mut (*act).flags,
                    );
                } else {
                    return ReportFormatVersionMismatch(
                        ctx,
                        (*action).type_0,
                        field,
                        (*(*keymap_info).keymap).format,
                        b">= 2\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
            }
        }
        if type_0 as u32 == ACTION_TYPE_MOD_LOCK && field == ACTION_FIELD_AFFECT {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn CheckGroupField(
    keymap_info: *const xkb_keymap_info,
    action: xkb_action_type,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    flags_inout: *mut xkb_action_flags,
    group_rtrn: *mut i32,
) -> xkb_parser_error {
    unsafe {
        let spec: *const ExprDef;
        let mut idx: u32 = 0_u32;
        let mut flags: xkb_action_flags = *flags_inout;
        if !array_ndx.is_null() {
            return ReportActionNotArray(
                &raw mut (*(*keymap_info).keymap).ctx,
                action,
                ACTION_FIELD_GROUP,
                (*keymap_info).strict,
            );
        }
        if (*value).common.type_0 == STMT_EXPR_NEGATE
            || (*value).common.type_0 == STMT_EXPR_UNARY_PLUS
        {
            flags = (flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            spec = (*value).unary.child;
            value_ptr = &raw mut (**value_ptr).unary.child as *mut *mut ExprDef;
        } else {
            flags = (flags as u32 | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
            spec = value;
        }
        let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH != 0;
        let mut pending: bool = false;
        let ret: xkb_parser_error =
            ExprResolveGroup(keymap_info, spec, absolute, &raw mut idx, &raw mut pending)
                as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS && !pending {
            ReportMismatch(
                &raw mut (*(*keymap_info).keymap).ctx,
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
                action,
                ACTION_FIELD_GROUP,
                b"integer\0".as_ptr() as *const i8,
                (*keymap_info).strict,
            );
            return ret;
        }
        if pending {
            flags = (flags as u32 | ACTION_PENDING_COMPUTATION) as xkb_action_flags;
            let pending_index: u32 = (&*(*keymap_info).pending_computations).len() as u32;
            (&mut *(*keymap_info).pending_computations).push(pending_computation {
                expr: *value_ptr,
                computed: false,
                value: 0_u32,
            });
            *value_ptr = std::ptr::null_mut();
            *group_rtrn = pending_index as i32;
        } else {
            flags =
                (flags as u32 & !(ACTION_PENDING_COMPUTATION as i32) as u32) as xkb_action_flags;
            if flags as u32 & ACTION_ABSOLUTE_SWITCH == 0 {
                *group_rtrn = idx as i32;
                if (*value).common.type_0 == STMT_EXPR_NEGATE {
                    *group_rtrn = -*group_rtrn;
                }
            } else {
                *group_rtrn = idx.wrapping_sub(1_u32) as i32;
            }
        }
        *flags_inout = flags;
        PARSER_SUCCESS
    }
}
unsafe fn HandleSetLatchLockGroup(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_group_action = &raw mut (*action).group;
        let type_0: xkb_action_type = (*action).type_0;
        if field == ACTION_FIELD_GROUP {
            return CheckGroupField(
                keymap_info,
                (*action).type_0,
                array_ndx,
                value,
                value_ptr,
                &raw mut (*act).flags,
                &raw mut (*act).group,
            );
        }
        if (type_0 as u32 == ACTION_TYPE_GROUP_SET || type_0 as u32 == ACTION_TYPE_GROUP_LATCH)
            && field == ACTION_FIELD_CLEAR_LOCKS
        {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LOCK_CLEAR,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_GROUP_LATCH && field == ACTION_FIELD_LATCH_TO_LOCK {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_LATCH_TO_LOCK,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        if type_0 as u32 == ACTION_TYPE_GROUP_LOCK && field == ACTION_FIELD_LOCK_ON_RELEASE {
            if (*keymap_info).features.group_lock_on_release {
                return CheckBooleanFlag(
                    ctx,
                    (*keymap_info).strict,
                    (*action).type_0,
                    field,
                    ACTION_LOCK_ON_RELEASE,
                    array_ndx,
                    value,
                    &raw mut (*act).flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    ctx,
                    (*action).type_0,
                    field,
                    (*(*keymap_info).keymap).format,
                    b">= v2\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn HandleMovePtr(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_pointer_action = &raw mut (*action).ptr;
        if field == ACTION_FIELD_X || field == ACTION_FIELD_Y {
            let mut val: i64 = 0_i64;
            let absolute: bool = (*value).common.type_0 != STMT_EXPR_NEGATE
                && (*value).common.type_0 != STMT_EXPR_UNARY_PLUS;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if val < i16::MIN as i64 || val > i16::MAX as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "The {} field in the {} action must be in range {}..{}, but got {}. Action definition ignored\n",
                    fieldText(field),
                    ActionTypeText((*action).type_0),
                    -32767_i32 - 1_i32,
                    32767_i32,
                    val,
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if field == ACTION_FIELD_X {
                if absolute {
                    (*act).flags = ((*act).flags | ACTION_ABSOLUTE_X) as xkb_action_flags;
                }
                (*act).x = val as i16;
            } else {
                if absolute {
                    (*act).flags = ((*act).flags | ACTION_ABSOLUTE_Y) as xkb_action_flags;
                }
                (*act).y = val as i16;
            }
            return PARSER_SUCCESS;
        } else if field == ACTION_FIELD_ACCEL {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_ACCEL,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn HandlePtrBtn(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_pointer_button_action = &raw mut (*action).btn;
        if field == ACTION_FIELD_BUTTON {
            let mut btn: i64 = 0_i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveButton(ctx, value, &raw mut btn) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (range 1..5)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if !(0_i64..=5_i64).contains(&btn) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Button must specify default or be in the range 1..5; Illegal button value {} ignored\n",
                    btn,
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).button = btn as u8;
            return PARSER_SUCCESS;
        } else if (*action).type_0 == ACTION_TYPE_PTR_LOCK && field == ACTION_FIELD_AFFECT {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        } else if field == ACTION_FIELD_COUNT {
            let mut val: i64 = 0_i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if !(0_i64..=255_i64).contains(&val) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "The count field must have a value in the range 0..255; Illegal count {} ignored\n",
                    val,
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).count = val as u8;
            return PARSER_SUCCESS;
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
static PTR_DFLTS: [LookupEntry; 4] = [
    LookupEntry {
        name: "dfltbtn",
        value: 1_u32,
    },
    LookupEntry {
        name: "defaultbutton",
        value: 1_u32,
    },
    LookupEntry {
        name: "button",
        value: 1_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
unsafe fn HandleSetPtrDflt(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_pointer_default_action = &raw mut (*action).dflt;
        if field == ACTION_FIELD_AFFECT {
            let mut val: u32 = 0_u32;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveEnum(ctx, value, &raw mut val, &PTR_DFLTS) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"pointer component\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            return PARSER_SUCCESS;
        } else if field == ACTION_FIELD_BUTTON || field == ACTION_FIELD_VALUE {
            let button: *const ExprDef;
            let mut btn: i64 = 0_i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 == STMT_EXPR_NEGATE
                || (*value).common.type_0 == STMT_EXPR_UNARY_PLUS
            {
                (*act).flags =
                    ((*act).flags & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
                button = (*value).unary.child;
            } else {
                (*act).flags = ((*act).flags | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
                button = value;
            }
            if !ExprResolveButton(ctx, button, &raw mut btn) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (range 1..5)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if !(0_i64..=5_i64).contains(&btn) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "New default button value must be in the range 1..5; Illegal default button value {} ignored\n",
                    btn,
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if btn == 0_i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Cannot set default pointer button to \"default\"; Illegal default button setting ignored\n",
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).value = (if (*value).common.type_0 == STMT_EXPR_NEGATE {
                -btn
            } else {
                btn
            }) as i8;
            return PARSER_SUCCESS;
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn HandleSwitchScreen(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_switch_screen_action = &raw mut (*action).screen;
        if field == ACTION_FIELD_SCREEN {
            let scrn: *const ExprDef;
            let mut val: i64 = 0_i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 == STMT_EXPR_NEGATE
                || (*value).common.type_0 == STMT_EXPR_UNARY_PLUS
            {
                (*act).flags =
                    ((*act).flags & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
                scrn = (*value).unary.child;
            } else {
                (*act).flags = ((*act).flags | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
                scrn = value;
            }
            if !ExprResolveInteger(ctx, scrn, &raw mut val) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"integer (-128..127)\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            val = if (*value).common.type_0 == STMT_EXPR_NEGATE {
                -val
            } else {
                val
            };
            if val < i8::MIN as i64 || val > i8::MAX as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Screen index must be in the range {}..{}; Illegal screen value {} ignored\n",
                    -128_i32,
                    127_i32,
                    val,
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).screen = val as i8;
            return PARSER_SUCCESS;
        } else if field == ACTION_FIELD_SAME {
            return CheckBooleanFlag(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                field,
                ACTION_SAME_SCREEN,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn HandleSetLockControls(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_controls_action = &raw mut (*action).ctrls;
        if field == ACTION_FIELD_CONTROLS {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            let mut mask: u32 = 0_u32;
            let offset: u8 = (*keymap_info).features.controls_name_offset;
            if !ExprResolveMask(ctx, value, &raw mut mask, &ctrlMaskNames[offset as usize..]) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"controls mask\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            (*act).ctrls = mask as xkb_action_controls;
            return PARSER_SUCCESS;
        } else if field == ACTION_FIELD_AFFECT && (*action).type_0 == ACTION_TYPE_CTRL_LOCK {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn HandleRedirectKey(
    keymap_info: *const xkb_keymap_info,
    mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let keymap: *const xkb_keymap = (*keymap_info).keymap;
        let ctx: *mut xkb_context = (&raw const (*keymap).ctx) as *mut xkb_context;
        let act: *mut xkb_redirect_key_action = &raw mut (*action).redirect;
        if field == ACTION_FIELD_KEYCODE {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 == STMT_EXPR_IDENT {
                let valStr: &str = xkb_atom_text(&(*ctx).atom_table, (*value).ident.ident);
                if !valStr.is_empty() && valStr.eq_ignore_ascii_case("auto") {
                    (*act).keycode = (*(*keymap_info).keymap).redirect_key_auto;
                    return PARSER_SUCCESS;
                }
            }
            if (*value).common.type_0 != STMT_EXPR_KEYNAME_LITERAL {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"key name\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            let key: *const xkb_key = XkbKeyByName(keymap, (*value).key_name.key_name, true);
            if key.is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "RedirectKey field {} cannot resolve <{}> to a valid key\n",
                    fieldText(field),
                    xkb_atom_text(&(*ctx).atom_table, (*value).key_name.key_name),
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_VALUE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).keycode = (*key).keycode;
            return PARSER_SUCCESS;
        }
        if field == ACTION_FIELD_MODIFIERS || field == ACTION_FIELD_MODS_TO_CLEAR {
            let mut flags: xkb_action_flags = 0 as xkb_action_flags;
            let mut m: u32 = 0_u32;
            let r: xkb_parser_error = CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut flags,
                &raw mut m,
            );
            if r as u32 != PARSER_SUCCESS {
                return r;
            }
            if flags as u64 != 0 {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    (*action).type_0,
                    field,
                    b"modifier mask\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            (*act).affect |= m;
            if field == ACTION_FIELD_MODIFIERS {
                (*act).mods |= m;
            } else {
                (*act).mods &= !m;
            }
            return PARSER_SUCCESS;
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
unsafe fn HandleUnsupported(
    _keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    _action: *mut xkb_action,
    _field: u32,
    _array_ndx: *const ExprDef,
    _value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    PARSER_SUCCESS
}
unsafe fn HandlePrivate(
    keymap_info: *const xkb_keymap_info,
    _mods: *const xkb_mod_set,
    action: *mut xkb_action,
    field: u32,
    array_ndx: *const ExprDef,
    value: *const ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let act: *mut xkb_private_action = &raw mut (*action).priv_0;
        if field == ACTION_FIELD_TYPE {
            let mut type_0: i64 = 0_i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveInteger(ctx, value, &raw mut type_0) {
                return ReportMismatch(
                    ctx,
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    ACTION_TYPE_PRIVATE,
                    field,
                    b"integer\0".as_ptr() as *const i8,
                    (*keymap_info).strict,
                );
            }
            if !(0_i64..=255_i64).contains(&type_0) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Private action type must be in the range 0..255; Illegal type {} ignored\n",
                    type_0,
                );
                return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if type_0 < ACTION_TYPE_PRIVATE as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_INFO,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Private actions of type {} are not supported; Ignored\n",
                    ActionTypeText(type_0 as xkb_action_type),
                );
                (*act).type_0 = ACTION_TYPE_NONE;
            } else {
                (*act).type_0 = type_0 as xkb_action_type;
            }
            return PARSER_SUCCESS;
        } else if field == ACTION_FIELD_DATA {
            if array_ndx.is_null() {
                let mut val: u32 = XKB_ATOM_NONE;
                if !ExprResolveString(ctx, value, &raw mut val) {
                    return ReportMismatch(
                        ctx,
                        XKB_ERROR_WRONG_FIELD_TYPE,
                        (*action).type_0,
                        field,
                        b"string\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
                let str_bytes: &str = xkb_atom_text(&(*ctx).atom_table, val);
                let len: usize = str_bytes.len();
                if len < 1_usize || len > std::mem::size_of::<[u8; 7]>() {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "A private action has {} data bytes, but got: {}; Illegal data ignored\n",
                        std::mem::size_of::<[u8; 7]>(),
                        len,
                    );
                    return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                std::ptr::write_bytes::<[u8; 7]>(
                    &raw mut (*act).data as *mut u8 as *mut [u8; 7],
                    0u8,
                    1,
                );
                std::ptr::copy_nonoverlapping(
                    str_bytes.as_bytes().as_ptr(),
                    &raw mut (*act).data as *mut u8,
                    len,
                );
                return PARSER_SUCCESS;
            } else {
                let mut ndx: i64 = 0_i64;
                let mut datum: i64 = 0_i64;
                if !ExprResolveInteger(ctx, array_ndx, &raw mut ndx) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Array subscript must be integer; Illegal subscript ignored\n",
                    );
                    return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if ndx < 0_i64 || ndx as usize >= std::mem::size_of::<[u8; 7]>() {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "The data for a private action is {} bytes long; Attempt to use data[{}] ignored\n",
                        std::mem::size_of::<[u8; 7]>(),
                        ndx,
                    );
                    return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if !ExprResolveInteger(ctx, value, &raw mut datum) {
                    return ReportMismatch(
                        ctx,
                        XKB_ERROR_WRONG_FIELD_TYPE,
                        (*act).type_0,
                        field,
                        b"integer\0".as_ptr() as *const i8,
                        (*keymap_info).strict,
                    );
                }
                if !(0_i64..=255_i64).contains(&datum) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "All data for a private action must be 0..255; Illegal datum {} ignored\n",
                        datum,
                    );
                    return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                (*act).data[ndx as usize] = datum as u8;
                return PARSER_SUCCESS;
            }
        }
        ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict)
    }
}
static HANDLE_ACTION: [actionHandler; 21] = {
    [
        Some(
            HandleNoAction
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleNoAction
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockMods
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLatchLockGroup
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleMovePtr
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePtrBtn
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePtrBtn
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetPtrDflt
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleNoAction
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSwitchScreen
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLockControls
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleSetLockControls
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleRedirectKey
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleUnsupported
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandleUnsupported
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        Some(
            HandlePrivate
                as unsafe fn(
                    *const xkb_keymap_info,
                    *const xkb_mod_set,
                    *mut xkb_action,
                    u32,
                    *const ExprDef,
                    *const ExprDef,
                    *mut *mut ExprDef,
                ) -> xkb_parser_error,
        ),
        None,
    ]
};
pub unsafe fn HandleActionDef(
    keymap_info: *const xkb_keymap_info,
    info: *mut ActionsInfo,
    mods: *const xkb_mod_set,
    def: *mut ExprDef,
    action: *mut xkb_action,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        if (*def).common.type_0 != STMT_EXPR_ACTION_DECL {
            println!(
                "{:?} {} {} [XKB-{{:03}}] Expected an action definition, found {{}}\n {} {:?}",
                ctx,
                XKB_LOG_LEVEL_ERROR,
                { XKB_LOG_VERBOSITY_MINIMAL },
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string((*def).common.type_0)
            );
            return PARSER_FATAL_ERROR;
        }
        let action_name: &str = xkb_atom_text(&(*ctx).atom_table, (*def).action.name);
        let mut handler_type: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(action_name, &raw mut handler_type) {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action \"{}\"\n",
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                action_name,
            );
            handler_type = ACTION_TYPE_UNKNOWN;
            if (*keymap_info).strict & PARSER_NO_UNKNOWN_ACTION != 0 {
                return PARSER_FATAL_ERROR;
            }
        }
        *action = (*info).actions[handler_type as usize];
        if handler_type as u32 == ACTION_TYPE_UNSUPPORTED_LEGACY {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unsupported legacy action type \"{}\".\n",
                XKB_WARNING_UNSUPPORTED_LEGACY_ACTION as i32,
                action_name,
            );
            (*action).type_0 = ACTION_TYPE_NONE;
        }
        let mut ret: xkb_parser_error = PARSER_SUCCESS;
        let mut arg: *mut ExprDef = (*def).action.args as *mut ExprDef;
        while !arg.is_null() {
            let value: *const ExprDef;
            let mut value_ptr: *mut *mut ExprDef = std::ptr::null_mut();
            let field: *mut ExprDef;
            let mut arrayRtrn: *mut ExprDef = std::ptr::null_mut();
            let mut elemRtrn: &str = "";
            let mut fieldRtrn: &str = "";
            if (*arg).common.type_0 == STMT_EXPR_ASSIGN {
                field = (*arg).binary.left as *mut ExprDef;
                value = (*arg).binary.right;
                value_ptr = &raw mut (*arg).binary.right as *mut *mut ExprDef;
            } else if (*arg).common.type_0 == STMT_EXPR_NOT
                || (*arg).common.type_0 == STMT_EXPR_INVERT
            {
                field = (*arg).unary.child as *mut ExprDef;
                value = &raw const CONST_FALSE.0 as *const ExprDef;
            } else {
                field = arg;
                value = &raw const CONST_TRUE.0 as *const ExprDef;
            }
            if !ExprResolveLhs(
                ctx,
                field,
                &mut elemRtrn,
                &mut fieldRtrn,
                &raw mut arrayRtrn,
            ) {
                return PARSER_FATAL_ERROR;
            }
            if !elemRtrn.is_empty() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Cannot change defaults in an action definition; Ignoring attempt to change \"{}.{}\".\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    elemRtrn,
                    fieldRtrn,
                );
                return PARSER_FATAL_ERROR;
            }
            let mut fieldNdx: u32 = ACTION_FIELD_CLEAR_LOCKS;
            if !stringToField(fieldRtrn, &raw mut fieldNdx) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown field name {} for action {} discarded\n",
                    XKB_ERROR_INVALID_ACTION_FIELD as i32,
                    fieldRtrn,
                    ActionTypeText((*action).type_0),
                );
                if (*keymap_info).strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
                    return PARSER_FATAL_ERROR;
                }
            } else {
                match HANDLE_ACTION[handler_type as usize].expect("non-null function pointer")(
                    keymap_info,
                    mods,
                    action,
                    fieldNdx,
                    arrayRtrn,
                    value,
                    value_ptr,
                ) as u32
                {
                    2 => return PARSER_FATAL_ERROR,
                    1 => {
                        ret = PARSER_RECOVERABLE_ERROR;
                    }
                    _ => {}
                }
            }
            arg = (*arg).common.next as *mut ExprDef;
        }
        (if (*action).type_0 == ACTION_TYPE_UNKNOWN {
            PARSER_RECOVERABLE_ERROR
        } else {
            ret as u32
        }) as xkb_parser_error
    }
}
pub unsafe fn SetDefaultActionField(
    keymap_info: *const xkb_keymap_info,
    info: *mut ActionsInfo,
    mods: *mut xkb_mod_set,
    elem: &str,
    field: &str,
    array_ndx: *mut ExprDef,
    value_ptr: *mut *mut ExprDef,
    merge: merge_mode,
) -> xkb_parser_error {
    unsafe {
        let value: *const ExprDef = *value_ptr;
        let mut action: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(elem, &raw mut action) {
            xkb_logf!(
                (*(*keymap_info).keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action \"{}\"\n",
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                elem,
            );
            return (if (*keymap_info).strict & PARSER_NO_UNKNOWN_ACTION != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let mut action_field: u32 = ACTION_FIELD_CLEAR_LOCKS;
        if !stringToField(field, &raw mut action_field) {
            xkb_logf!(
                (*(*keymap_info).keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action field \"{}\"\n",
                XKB_ERROR_INVALID_ACTION_FIELD as i32,
                field,
            );
            return (if (*keymap_info).strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let into: *mut xkb_action = (&raw mut (*info).actions as *mut xkb_action)
            .offset(action as isize) as *mut xkb_action;
        let mut from: xkb_action = *into;
        let ret: xkb_parser_error = HANDLE_ACTION[action as usize]
            .expect("non-null function pointer")(
            keymap_info,
            mods,
            &raw mut from,
            action_field,
            array_ndx,
            value,
            value_ptr,
        ) as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS {
            return ret;
        }
        if !action_equal(into, &raw mut from) {
            let replace: bool = merge != MERGE_AUGMENT;
            xkb_logf!(
                (*(*keymap_info).keymap).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_VERBOSE as i32,
                "Conflicting field \"{}\" for default action \"{}\"; Using {}, ignore {}\n",
                fieldText(action_field),
                ActionTypeText(action),
                crate::xkb::utils::CStrDisplay(if replace as i32 != 0 {
                    b"from\0".as_ptr() as *const i8
                } else {
                    b"into\0".as_ptr() as *const i8
                }),
                crate::xkb::utils::CStrDisplay(if replace as i32 != 0 {
                    b"into\0".as_ptr() as *const i8
                } else {
                    b"from\0".as_ptr() as *const i8
                }),
            );
            if replace {
                *into = from;
            }
        }
        PARSER_SUCCESS
    }
}
use crate::xkb::shared_types::*;
