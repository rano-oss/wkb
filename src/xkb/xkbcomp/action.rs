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
static mut constTrue: ExprBoolean = ExprBoolean {
    common: _ParseCommon {
        next: std::ptr::null_mut(),
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: true,
};
static mut constFalse: ExprBoolean = ExprBoolean {
    common: _ParseCommon {
        next: std::ptr::null_mut(),
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    set: false,
};
pub unsafe fn InitActionsInfo(mut keymap: *const xkb_keymap, mut info: *mut ActionsInfo) {
    unsafe {
        let mut type_0: xkb_action_type = ACTION_TYPE_NONE;
        while (type_0 as u32) < _ACTION_TYPE_NUM_ENTRIES as u32 {
            (*info).actions[type_0 as usize].type_0 = type_0;
            type_0 += 1;
        }
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as usize].dflt.flags = 0 as xkb_action_flags;
        (*info).actions[ACTION_TYPE_PTR_DEFAULT as usize].dflt.value = 1 as i8;
        (*info).actions[ACTION_TYPE_PTR_MOVE as usize].ptr.flags = ACTION_ACCEL;
        (*info).actions[ACTION_TYPE_SWITCH_VT as usize].screen.flags = ACTION_SAME_SCREEN;
        (*info).actions[ACTION_TYPE_REDIRECT_KEY as usize]
            .redirect
            .keycode = (*keymap).redirect_key_auto;
    }
}
static mut fieldStrings: [LookupEntry; 37] = [
    LookupEntry {
        name: b"clearLocks",
        value: ACTION_FIELD_CLEAR_LOCKS as u32,
    },
    LookupEntry {
        name: b"latchToLock",
        value: ACTION_FIELD_LATCH_TO_LOCK as u32,
    },
    LookupEntry {
        name: b"genKeyEvent",
        value: ACTION_FIELD_GEN_KEY_EVENT as u32,
    },
    LookupEntry {
        name: b"generateKeyEvent",
        value: ACTION_FIELD_GEN_KEY_EVENT as u32,
    },
    LookupEntry {
        name: b"report",
        value: ACTION_FIELD_REPORT as u32,
    },
    LookupEntry {
        name: b"default",
        value: ACTION_FIELD_DEFAULT as u32,
    },
    LookupEntry {
        name: b"affect",
        value: ACTION_FIELD_AFFECT as u32,
    },
    LookupEntry {
        name: b"increment",
        value: ACTION_FIELD_INCREMENT as u32,
    },
    LookupEntry {
        name: b"modifiers",
        value: ACTION_FIELD_MODIFIERS as u32,
    },
    LookupEntry {
        name: b"mods",
        value: ACTION_FIELD_MODIFIERS as u32,
    },
    LookupEntry {
        name: b"group",
        value: ACTION_FIELD_GROUP as u32,
    },
    LookupEntry {
        name: b"x",
        value: ACTION_FIELD_X as u32,
    },
    LookupEntry {
        name: b"y",
        value: ACTION_FIELD_Y as u32,
    },
    LookupEntry {
        name: b"accel",
        value: ACTION_FIELD_ACCEL as u32,
    },
    LookupEntry {
        name: b"accelerate",
        value: ACTION_FIELD_ACCEL as u32,
    },
    LookupEntry {
        name: b"repeat",
        value: ACTION_FIELD_ACCEL as u32,
    },
    LookupEntry {
        name: b"button",
        value: ACTION_FIELD_BUTTON as u32,
    },
    LookupEntry {
        name: b"value",
        value: ACTION_FIELD_VALUE as u32,
    },
    LookupEntry {
        name: b"controls",
        value: ACTION_FIELD_CONTROLS as u32,
    },
    LookupEntry {
        name: b"ctrls",
        value: ACTION_FIELD_CONTROLS as u32,
    },
    LookupEntry {
        name: b"type",
        value: ACTION_FIELD_TYPE as u32,
    },
    LookupEntry {
        name: b"count",
        value: ACTION_FIELD_COUNT as u32,
    },
    LookupEntry {
        name: b"screen",
        value: ACTION_FIELD_SCREEN as u32,
    },
    LookupEntry {
        name: b"same",
        value: ACTION_FIELD_SAME as u32,
    },
    LookupEntry {
        name: b"sameServer",
        value: ACTION_FIELD_SAME as u32,
    },
    LookupEntry {
        name: b"data",
        value: ACTION_FIELD_DATA as u32,
    },
    LookupEntry {
        name: b"device",
        value: ACTION_FIELD_DEVICE as u32,
    },
    LookupEntry {
        name: b"dev",
        value: ACTION_FIELD_DEVICE as u32,
    },
    LookupEntry {
        name: b"key",
        value: ACTION_FIELD_KEYCODE as u32,
    },
    LookupEntry {
        name: b"keycode",
        value: ACTION_FIELD_KEYCODE as u32,
    },
    LookupEntry {
        name: b"kc",
        value: ACTION_FIELD_KEYCODE as u32,
    },
    LookupEntry {
        name: b"clearmods",
        value: ACTION_FIELD_MODS_TO_CLEAR as u32,
    },
    LookupEntry {
        name: b"clearmodifiers",
        value: ACTION_FIELD_MODS_TO_CLEAR as u32,
    },
    LookupEntry {
        name: b"lockOnRelease",
        value: ACTION_FIELD_LOCK_ON_RELEASE as u32,
    },
    LookupEntry {
        name: b"unlockOnPress",
        value: ACTION_FIELD_UNLOCK_ON_PRESS as u32,
    },
    LookupEntry {
        name: b"latchOnPress",
        value: ACTION_FIELD_LATCH_ON_PRESS as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
unsafe fn stringToActionType(str: &[u8], mut type_rtrn: *mut xkb_action_type) -> bool {
    unsafe {
        let mut type_0: u32 = 0 as u32;
        let ret: bool = LookupString(
            &raw const actionTypeNames as *const LookupEntry,
            str,
            &raw mut type_0,
        ) as bool;
        *type_rtrn = type_0 as xkb_action_type;
        return ret;
    }
}
unsafe fn stringToField(str: &[u8], mut field_rtrn: *mut u32) -> bool {
    unsafe {
        let mut field: u32 = 0 as u32;
        let ret: bool = LookupString(
            &raw const fieldStrings as *const LookupEntry,
            str,
            &raw mut field,
        ) as bool;
        *field_rtrn = field as u32;
        return ret;
    }
}
unsafe fn fieldText(mut field: u32) -> &'static [u8] {
    unsafe {
        return LookupValue(&raw const fieldStrings as *const LookupEntry, field as u32);
    }
}
#[inline]
unsafe fn ReportMismatch(
    mut ctx: *mut xkb_context,
    mut code: xkb_message_code,
    mut action: xkb_action_type,
    mut field: u32,
    mut type_0: *const i8,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Value of {} field must be of type {}; Action {} definition ignored\n",
            code as u32,
            crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
            crate::xkb::utils::CStrDisplay(type_0),
            crate::xkb::utils::ByteSliceDisplay(ActionTypeText(action)),
        );
        return (if strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe fn ReportFormatVersionMismatch(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: u32,
    mut format: u32,
    mut versions: *const i8,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Field {} for an action of type {} requires keymap text format {},  but got: {}; Action definition ignored\n",
            XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT as i32,
            crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
            crate::xkb::utils::ByteSliceDisplay(ActionTypeText(action)),
            crate::xkb::utils::CStrDisplay(versions),
            format as u32,
        );
        return (if strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe fn ReportIllegal(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: u32,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Field {} is not defined for an action of type {}; Action definition ignored\n",
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
            crate::xkb::utils::ByteSliceDisplay(ActionTypeText(action)),
        );
        return (if strict as u32 & PARSER_NO_ILLEGAL_ACTION_FIELDS as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
#[inline]
unsafe fn ReportActionNotArray(
    mut ctx: *mut xkb_context,
    mut action: xkb_action_type,
    mut field: u32,
    mut strict: xkb_parser_strict_flags,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] The {} field in the {} action is not an array; Action definition ignored\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
            crate::xkb::utils::ByteSliceDisplay(ActionTypeText(action)),
        );
        return (if strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as xkb_parser_error;
    }
}
unsafe fn HandleNoAction(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        xkb_logf!(
            (*(*keymap_info).keymap).ctx,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] The \"{}\" action takes no argument, but got \"{}\" field; Action definition ignored\n",
            XKB_ERROR_INVALID_ACTION_FIELD as i32,
            crate::xkb::utils::ByteSliceDisplay(ActionTypeText((*action).type_0)),
            crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
        );
        return (if (*keymap_info).strict as u32 & PARSER_NO_ILLEGAL_ACTION_FIELDS as u32 != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_SUCCESS as i32
        }) as xkb_parser_error;
    }
}
unsafe fn CheckBooleanFlag(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut action: xkb_action_type,
    mut field: u32,
    mut flag: xkb_action_flags,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
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
            *flags_inout = (*flags_inout as u32 | flag as u32) as xkb_action_flags;
        } else {
            *flags_inout = (*flags_inout as u32 & !(flag as u32)) as xkb_action_flags;
        }
        return PARSER_SUCCESS;
    }
}
unsafe fn CheckModifierField(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut mods: *const xkb_mod_set,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
    mut mods_rtrn: *mut u32,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_MODIFIERS, strict);
        }
        if (*value).common.type_0 as u32 == STMT_EXPR_IDENT as u32 {
            let valStr: &[u8] = xkb_atom_text_bytes(&(*ctx).atom_table, (*value).ident.ident);
            if !valStr.is_empty()
                && (istreq(valStr, b"usemodmapmods") || istreq(valStr, b"modmapmods"))
            {
                *mods_rtrn = 0 as u32;
                *flags_inout =
                    (*flags_inout as u32 | ACTION_MODS_LOOKUP_MODMAP as u32) as xkb_action_flags;
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
            (*flags_inout as u32 & !(ACTION_MODS_LOOKUP_MODMAP as i32) as u32) as xkb_action_flags;
        return PARSER_SUCCESS;
    }
}
static mut lockWhich: [LookupEntry; 5] = [
    LookupEntry {
        name: b"both",
        value: 0 as u32,
    },
    LookupEntry {
        name: b"lock",
        value: ACTION_LOCK_NO_UNLOCK as u32,
    },
    LookupEntry {
        name: b"neither",
        value: (ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32,
    },
    LookupEntry {
        name: b"unlock",
        value: ACTION_LOCK_NO_LOCK as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
unsafe fn CheckAffectField(
    mut ctx: *mut xkb_context,
    mut strict: xkb_parser_strict_flags,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut flags_inout: *mut xkb_action_flags,
) -> xkb_parser_error {
    unsafe {
        if !array_ndx.is_null() {
            return ReportActionNotArray(ctx, action, ACTION_FIELD_AFFECT, strict);
        }
        let mut flags: u32 = 0 as u32;
        if !ExprResolveEnum(
            ctx,
            value,
            &raw mut flags,
            &raw const lockWhich as *const LookupEntry,
        ) {
            return ReportMismatch(
                ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                action,
                ACTION_FIELD_AFFECT,
                b"lock, unlock, both, neither\0".as_ptr() as *const i8,
                strict,
            );
        }
        *flags_inout = (*flags_inout as u32
            & !(ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32)
            as xkb_action_flags;
        *flags_inout = (*flags_inout as u32 | flags as xkb_action_flags as u32) as xkb_action_flags;
        return PARSER_SUCCESS;
    }
}
unsafe fn HandleSetLatchLockMods(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_mod_action = &raw mut (*action).mods;
        let type_0: xkb_action_type = (*action).type_0;
        if field as u32 == ACTION_FIELD_MODIFIERS as u32 {
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
        if field as u32 == ACTION_FIELD_UNLOCK_ON_PRESS as u32 {
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
        if (type_0 as u32 == ACTION_TYPE_MOD_SET as u32
            || type_0 as u32 == ACTION_TYPE_MOD_LATCH as u32)
            && field as u32 == ACTION_FIELD_CLEAR_LOCKS as u32
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
        if type_0 as u32 == ACTION_TYPE_MOD_LATCH as u32 {
            if field as u32 == ACTION_FIELD_LATCH_TO_LOCK as u32 {
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
            if field as u32 == ACTION_FIELD_LATCH_ON_PRESS as u32 {
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
        if type_0 as u32 == ACTION_TYPE_MOD_LOCK as u32
            && field as u32 == ACTION_FIELD_AFFECT as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn CheckGroupField(
    mut keymap_info: *const xkb_keymap_info,
    mut action: xkb_action_type,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    mut flags_inout: *mut xkb_action_flags,
    mut group_rtrn: *mut i32,
) -> xkb_parser_error {
    unsafe {
        let mut spec: *const ExprDef = std::ptr::null();
        let mut idx: u32 = 0 as u32;
        let mut flags: xkb_action_flags = *flags_inout;
        if !array_ndx.is_null() {
            return ReportActionNotArray(
                &raw mut (*(*keymap_info).keymap).ctx,
                action,
                ACTION_FIELD_GROUP,
                (*keymap_info).strict,
            );
        }
        if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as u32
            || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as u32
        {
            flags = (flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            spec = (*value).unary.child;
            value_ptr = &raw mut (**value_ptr).unary.child as *mut *mut ExprDef;
        } else {
            flags = (flags as u32 | ACTION_ABSOLUTE_SWITCH as u32) as xkb_action_flags;
            spec = value;
        }
        let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 != 0;
        let mut pending: bool = false;
        let ret: xkb_parser_error =
            ExprResolveGroup(keymap_info, spec, absolute, &raw mut idx, &raw mut pending)
                as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS as u32 && !pending {
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
            flags = (flags as u32 | ACTION_PENDING_COMPUTATION as u32) as xkb_action_flags;
            let pending_index: u32 = (&*(*keymap_info).pending_computations).len() as u32;
            (&mut *(*keymap_info).pending_computations).push(pending_computation {
                expr: *value_ptr,
                computed: false,
                value: 0 as u32,
            });
            *value_ptr = std::ptr::null_mut();
            *group_rtrn = pending_index as i32;
        } else {
            flags =
                (flags as u32 & !(ACTION_PENDING_COMPUTATION as i32) as u32) as xkb_action_flags;
            if flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 == 0 {
                *group_rtrn = idx as i32;
                if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as u32 {
                    *group_rtrn = -*group_rtrn;
                }
            } else {
                *group_rtrn = idx.wrapping_sub(1 as u32) as i32;
            }
        }
        *flags_inout = flags;
        return PARSER_SUCCESS;
    }
}
unsafe fn HandleSetLatchLockGroup(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_group_action = &raw mut (*action).group;
        let type_0: xkb_action_type = (*action).type_0;
        if field as u32 == ACTION_FIELD_GROUP as u32 {
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
        if (type_0 as u32 == ACTION_TYPE_GROUP_SET as u32
            || type_0 as u32 == ACTION_TYPE_GROUP_LATCH as u32)
            && field as u32 == ACTION_FIELD_CLEAR_LOCKS as u32
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
        if type_0 as u32 == ACTION_TYPE_GROUP_LATCH as u32
            && field as u32 == ACTION_FIELD_LATCH_TO_LOCK as u32
        {
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
        if type_0 as u32 == ACTION_TYPE_GROUP_LOCK as u32
            && field as u32 == ACTION_FIELD_LOCK_ON_RELEASE as u32
        {
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
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleMovePtr(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_pointer_action = &raw mut (*action).ptr;
        if field as u32 == ACTION_FIELD_X as u32 || field as u32 == ACTION_FIELD_Y as u32 {
            let mut val: i64 = 0 as i64;
            let absolute: bool = (*value).common.type_0 as u32 != STMT_EXPR_NEGATE as u32
                && (*value).common.type_0 as u32 != STMT_EXPR_UNARY_PLUS as u32;
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
                    crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
                    crate::xkb::utils::ByteSliceDisplay(ActionTypeText((*action).type_0)),
                    -32767 as i32 - 1 as i32,
                    32767 as i32,
                    val,
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if field as u32 == ACTION_FIELD_X as u32 {
                if absolute {
                    (*act).flags =
                        ((*act).flags as u32 | ACTION_ABSOLUTE_X as u32) as xkb_action_flags;
                }
                (*act).x = val as i16;
            } else {
                if absolute {
                    (*act).flags =
                        ((*act).flags as u32 | ACTION_ABSOLUTE_Y as u32) as xkb_action_flags;
                }
                (*act).y = val as i16;
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_ACCEL as u32 {
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
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandlePtrBtn(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_pointer_button_action = &raw mut (*action).btn;
        if field as u32 == ACTION_FIELD_BUTTON as u32 {
            let mut btn: i64 = 0 as i64;
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
            if btn < 0 as i64 || btn > 5 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Button must specify default or be in the range 1..5; Illegal button value {} ignored\n",
                    btn,
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).button = btn as u8;
            return PARSER_SUCCESS;
        } else if (*action).type_0 as u32 == ACTION_TYPE_PTR_LOCK as u32
            && field as u32 == ACTION_FIELD_AFFECT as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        } else if field as u32 == ACTION_FIELD_COUNT as u32 {
            let mut val: i64 = 0 as i64;
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
            if val < 0 as i64 || val > 255 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "The count field must have a value in the range 0..255; Illegal count {} ignored\n",
                    val,
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).count = val as u8;
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
static mut ptrDflts: [LookupEntry; 4] = [
    LookupEntry {
        name: b"dfltbtn",
        value: 1 as u32,
    },
    LookupEntry {
        name: b"defaultbutton",
        value: 1 as u32,
    },
    LookupEntry {
        name: b"button",
        value: 1 as u32,
    },
    LookupEntry {
        name: b"",
        value: 0 as u32,
    },
];
unsafe fn HandleSetPtrDflt(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_pointer_default_action = &raw mut (*action).dflt;
        if field as u32 == ACTION_FIELD_AFFECT as u32 {
            let mut val: u32 = 0 as u32;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if !ExprResolveEnum(
                ctx,
                value,
                &raw mut val,
                &raw const ptrDflts as *const LookupEntry,
            ) {
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
        } else if field as u32 == ACTION_FIELD_BUTTON as u32
            || field as u32 == ACTION_FIELD_VALUE as u32
        {
            let mut button: *const ExprDef = std::ptr::null();
            let mut btn: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as u32
                || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as u32
            {
                (*act).flags = ((*act).flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32)
                    as xkb_action_flags;
                button = (*value).unary.child;
            } else {
                (*act).flags =
                    ((*act).flags as u32 | ACTION_ABSOLUTE_SWITCH as u32) as xkb_action_flags;
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
            if btn < 0 as i64 || btn > 5 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "New default button value must be in the range 1..5; Illegal default button value {} ignored\n",
                    btn,
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            if btn == 0 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Cannot set default pointer button to \"default\"; Illegal default button setting ignored\n",
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).value = (if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as u32 {
                -btn
            } else {
                btn
            }) as i8;
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleSwitchScreen(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_switch_screen_action = &raw mut (*action).screen;
        if field as u32 == ACTION_FIELD_SCREEN as u32 {
            let mut scrn: *const ExprDef = std::ptr::null();
            let mut val: i64 = 0 as i64;
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as u32
                || (*value).common.type_0 as u32 == STMT_EXPR_UNARY_PLUS as u32
            {
                (*act).flags = ((*act).flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32)
                    as xkb_action_flags;
                scrn = (*value).unary.child;
            } else {
                (*act).flags =
                    ((*act).flags as u32 | ACTION_ABSOLUTE_SWITCH as u32) as xkb_action_flags;
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
            val = if (*value).common.type_0 as u32 == STMT_EXPR_NEGATE as u32 {
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
                    -128 as i32,
                    127 as i32,
                    val,
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).screen = val as i8;
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_SAME as u32 {
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
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleSetLockControls(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_controls_action = &raw mut (*action).ctrls;
        if field as u32 == ACTION_FIELD_CONTROLS as u32 {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            let mut mask: u32 = 0 as u32;
            let offset: u8 = (*keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                ctx,
                value,
                &raw mut mask,
                (&raw const ctrlMaskNames as *const LookupEntry).offset(offset as i32 as isize),
            ) {
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
        } else if field as u32 == ACTION_FIELD_AFFECT as u32
            && (*action).type_0 as u32 == ACTION_TYPE_CTRL_LOCK as u32
        {
            return CheckAffectField(
                ctx,
                (*keymap_info).strict,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut (*act).flags,
            );
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleRedirectKey(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let keymap: *const xkb_keymap = (*keymap_info).keymap;
        let ctx: *mut xkb_context = (&raw const (*keymap).ctx) as *mut xkb_context;
        let act: *mut xkb_redirect_key_action = &raw mut (*action).redirect;
        if field as u32 == ACTION_FIELD_KEYCODE as u32 {
            if !array_ndx.is_null() {
                return ReportActionNotArray(ctx, (*action).type_0, field, (*keymap_info).strict);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_IDENT as u32 {
                let valStr: &[u8] = xkb_atom_text_bytes(&(*ctx).atom_table, (*value).ident.ident);
                if !valStr.is_empty() && istreq(valStr, b"auto") {
                    (*act).keycode = (*(*keymap_info).keymap).redirect_key_auto;
                    return PARSER_SUCCESS;
                }
            }
            if (*value).common.type_0 as u32 != STMT_EXPR_KEYNAME_LITERAL as u32 {
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
                    "RedirectKey field {} cannot resolve {} to a valid key\n",
                    crate::xkb::utils::ByteSliceDisplay(fieldText(field)),
                    crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                        (*ctx).clone(),
                        (*value).key_name.key_name
                    )),
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_VALUE_MISMATCH as u32 != 0
                {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as xkb_parser_error;
            }
            (*act).keycode = (*key).keycode;
            return PARSER_SUCCESS;
        }
        if field as u32 == ACTION_FIELD_MODIFIERS as u32
            || field as u32 == ACTION_FIELD_MODS_TO_CLEAR as u32
        {
            let mut flags: xkb_action_flags = 0 as xkb_action_flags;
            let mut m: u32 = 0 as u32;
            let mut r: xkb_parser_error = CheckModifierField(
                ctx,
                (*keymap_info).strict,
                mods,
                (*action).type_0,
                array_ndx,
                value,
                &raw mut flags,
                &raw mut m,
            );
            if r as u32 != PARSER_SUCCESS as u32 {
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
            if field as u32 == ACTION_FIELD_MODIFIERS as u32 {
                (*act).mods |= m;
            } else {
                (*act).mods &= !m;
            }
            return PARSER_SUCCESS;
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
unsafe fn HandleUnsupported(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    return PARSER_SUCCESS;
}
unsafe fn HandlePrivate(
    mut keymap_info: *const xkb_keymap_info,
    mut mods: *const xkb_mod_set,
    mut action: *mut xkb_action,
    mut field: u32,
    mut array_ndx: *const ExprDef,
    mut value: *const ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        let mut act: *mut xkb_private_action = &raw mut (*action).priv_0;
        if field as u32 == ACTION_FIELD_TYPE as u32 {
            let mut type_0: i64 = 0 as i64;
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
            if type_0 < 0 as i64 || type_0 > 255 as i64 {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Private action type must be in the range 0..255; Illegal type {} ignored\n",
                    type_0,
                );
                return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0
                {
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
                    crate::xkb::utils::ByteSliceDisplay(ActionTypeText(type_0 as xkb_action_type)),
                );
                (*act).type_0 = ACTION_TYPE_NONE;
            } else {
                (*act).type_0 = type_0 as xkb_action_type;
            }
            return PARSER_SUCCESS;
        } else if field as u32 == ACTION_FIELD_DATA as u32 {
            if array_ndx.is_null() {
                let mut val: u32 = XKB_ATOM_NONE as u32;
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
                let str_bytes: &[u8] = xkb_atom_text_bytes(&(*ctx).atom_table, val);
                let mut len: usize = str_bytes.len();
                if len < 1 as usize || len > std::mem::size_of::<[u8; 7]>() {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "A private action has {} data bytes, but got: {}; Illegal data ignored\n",
                        std::mem::size_of::<[u8; 7]>(),
                        len,
                    );
                    return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                        != 0
                    {
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
                    str_bytes.as_ptr(),
                    &raw mut (*act).data as *mut u8,
                    len,
                );
                return PARSER_SUCCESS;
            } else {
                let mut ndx: i64 = 0 as i64;
                let mut datum: i64 = 0 as i64;
                if !ExprResolveInteger(ctx, array_ndx, &raw mut ndx) {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Array subscript must be integer; Illegal subscript ignored\n",
                    );
                    return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                if ndx < 0 as i64 || ndx as usize >= std::mem::size_of::<[u8; 7]>() {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "The data for a private action is {} bytes long; Attempt to use data[{}] ignored\n",
                        std::mem::size_of::<[u8; 7]>(),
                        ndx,
                    );
                    return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                        != 0
                    {
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
                if datum < 0 as i64 || datum > 255 as i64 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "All data for a private action must be 0..255; Illegal datum {} ignored\n",
                        datum,
                    );
                    return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                        != 0
                    {
                        PARSER_FATAL_ERROR as i32
                    } else {
                        PARSER_RECOVERABLE_ERROR as i32
                    }) as xkb_parser_error;
                }
                (*act).data[ndx as usize] = datum as u8;
                return PARSER_SUCCESS;
            }
        }
        return ReportIllegal(ctx, (*action).type_0, field, (*keymap_info).strict);
    }
}
static mut handleAction: [actionHandler; 21] = {
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
    mut keymap_info: *const xkb_keymap_info,
    mut info: *mut ActionsInfo,
    mut mods: *const xkb_mod_set,
    mut def: *mut ExprDef,
    mut action: *mut xkb_action,
) -> xkb_parser_error {
    unsafe {
        let ctx: *mut xkb_context = &raw mut (*(*keymap_info).keymap).ctx;
        if (*def).common.type_0 as u32 != STMT_EXPR_ACTION_DECL as u32 {
            println!(
                "{:?} {} {} {} {} {:?}",
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Expected an action definition, found {}\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string((*def).common.type_0)
            );
            return PARSER_FATAL_ERROR;
        }
        let action_name: &[u8] = xkb_atom_text_bytes(&(*ctx).atom_table, (*def).action.name);
        let mut handler_type: xkb_action_type = ACTION_TYPE_NONE;
        if !stringToActionType(action_name, &raw mut handler_type) {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unknown action \"{}\"\n",
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                crate::xkb::utils::ByteSliceDisplay(action_name),
            );
            handler_type = ACTION_TYPE_UNKNOWN;
            if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION as u32 != 0 {
                return PARSER_FATAL_ERROR;
            }
        }
        *action = (*info).actions[handler_type as usize];
        if handler_type as u32 == ACTION_TYPE_UNSUPPORTED_LEGACY as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unsupported legacy action type \"{}\".\n",
                XKB_WARNING_UNSUPPORTED_LEGACY_ACTION as i32,
                crate::xkb::utils::ByteSliceDisplay(action_name),
            );
            (*action).type_0 = ACTION_TYPE_NONE;
        }
        let mut ret: xkb_parser_error = PARSER_SUCCESS;
        let mut arg: *mut ExprDef = (*def).action.args as *mut ExprDef;
        while !arg.is_null() {
            let mut value: *const ExprDef = std::ptr::null();
            let mut value_ptr: *mut *mut ExprDef = std::ptr::null_mut();
            let mut field: *mut ExprDef = std::ptr::null_mut();
            let mut arrayRtrn: *mut ExprDef = std::ptr::null_mut();
            let mut elemRtrn: &[u8] = b"";
            let mut fieldRtrn: &[u8] = b"";
            if (*arg).common.type_0 as u32 == STMT_EXPR_ASSIGN as u32 {
                field = (*arg).binary.left as *mut ExprDef;
                value = (*arg).binary.right;
                value_ptr = &raw mut (*arg).binary.right as *mut *mut ExprDef;
            } else if (*arg).common.type_0 as u32 == STMT_EXPR_NOT as u32
                || (*arg).common.type_0 as u32 == STMT_EXPR_INVERT as u32
            {
                field = (*arg).unary.child as *mut ExprDef;
                value = &raw const constFalse as *const ExprDef;
            } else {
                field = arg;
                value = &raw const constTrue as *const ExprDef;
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
                    crate::xkb::utils::ByteSliceDisplay(elemRtrn),
                    crate::xkb::utils::ByteSliceDisplay(fieldRtrn),
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
                    crate::xkb::utils::ByteSliceDisplay(fieldRtrn),
                    crate::xkb::utils::ByteSliceDisplay(ActionTypeText((*action).type_0)),
                );
                if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as u32 != 0 {
                    return PARSER_FATAL_ERROR;
                }
            } else {
                match handleAction[handler_type as usize].expect("non-null function pointer")(
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
        return (if (*action).type_0 as u32 == ACTION_TYPE_UNKNOWN as u32 {
            PARSER_RECOVERABLE_ERROR as u32
        } else {
            ret as u32
        }) as xkb_parser_error;
    }
}
pub unsafe fn SetDefaultActionField(
    mut keymap_info: *const xkb_keymap_info,
    mut info: *mut ActionsInfo,
    mut mods: *mut xkb_mod_set,
    elem: &[u8],
    field: &[u8],
    mut array_ndx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    mut merge: merge_mode,
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
                crate::xkb::utils::ByteSliceDisplay(elem),
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION as u32 != 0 {
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
                crate::xkb::utils::ByteSliceDisplay(field),
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_UNKNOWN_ACTION_FIELDS as u32 != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        let into: *mut xkb_action = (&raw mut (*info).actions as *mut xkb_action)
            .offset(action as isize) as *mut xkb_action;
        let mut from: xkb_action = *into;
        let ret: xkb_parser_error = handleAction[action as usize]
            .expect("non-null function pointer")(
            keymap_info,
            mods,
            &raw mut from,
            action_field,
            array_ndx,
            value,
            value_ptr,
        ) as xkb_parser_error;
        if ret as u32 != PARSER_SUCCESS as u32 {
            return ret;
        }
        if !action_equal(into, &raw mut from) {
            let replace: bool = merge as u32 != MERGE_AUGMENT as u32;
            xkb_logf!(
                (*(*keymap_info).keymap).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_VERBOSE as i32,
                "Conflicting field \"{}\" for default action \"{}\"; Using {}, ignore {}\n",
                crate::xkb::utils::ByteSliceDisplay(fieldText(action_field)),
                crate::xkb::utils::ByteSliceDisplay(ActionTypeText(action)),
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
        return PARSER_SUCCESS;
    }
}
use crate::xkb::shared_types::*;
