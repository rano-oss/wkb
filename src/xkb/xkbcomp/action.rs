use super::prelude::*;
pub use crate::xkb::keymap::action_equal;
use crate::xkb::text::{actionTypeNames, ctrlMaskNames, LookupString, LookupValue};
pub use crate::xkb::xkbcomp::expr::{ExprResolveButton, ExprResolveInteger, ExprResolveMask};
#[derive(Copy, Clone)]
pub struct ActionsInfo {
    pub actions: [xkb_action; 21],
}

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
    fn(
        &mut xkb_keymap_info,
        &xkb_mod_set,
        &mut xkb_action,
        u32,
        Option<&ExprDef>,
        &ExprDef,
        *mut *mut ExprDef,
    ) -> u32,
>;
// SAFETY: These are read-only constants; the raw pointers in other ExprKind variants are irrelevant
// because these use Boolean which contains no pointers.
struct SyncExprDef(ExprDef);
unsafe impl Sync for SyncExprDef {}

static CONST_TRUE: SyncExprDef = SyncExprDef(ExprDef {
    common: ParseCommon {
        next: std::ptr::null_mut(),
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    kind: ExprKind::Boolean(true),
});
static CONST_FALSE: SyncExprDef = SyncExprDef(ExprDef {
    common: ParseCommon {
        next: std::ptr::null_mut(),
        type_0: STMT_EXPR_BOOLEAN_LITERAL,
    },
    kind: ExprKind::Boolean(false),
});
pub fn InitActionsInfo(keymap: &xkb_keymap, info: &mut ActionsInfo) {
    let mut type_0: u32 = ACTION_TYPE_NONE;
    while (type_0 as u32) < _ACTION_TYPE_NUM_ENTRIES {
        info.actions[type_0 as usize] = xkb_action::from_type(type_0);
        type_0 += 1;
    }
    info.actions[ACTION_TYPE_PTR_DEFAULT as usize]
        .as_dflt_mut()
        .flags = 0;
    info.actions[ACTION_TYPE_PTR_DEFAULT as usize]
        .as_dflt_mut()
        .value = 1_i8;
    info.actions[ACTION_TYPE_PTR_MOVE as usize]
        .as_ptr_mut()
        .flags = ACTION_ACCEL;
    info.actions[ACTION_TYPE_SWITCH_VT as usize]
        .as_screen_mut()
        .flags = ACTION_SAME_SCREEN;
    info.actions[ACTION_TYPE_REDIRECT_KEY as usize]
        .as_redirect_mut()
        .keycode = keymap.redirect_key_auto;
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
fn stringToActionType(str: &str, type_rtrn: &mut u32) -> bool {
    let mut type_0: u32 = 0_u32;
    let ret: bool = LookupString(&actionTypeNames, str, &mut type_0);
    *type_rtrn = type_0 as u32;
    ret
}
fn stringToField(str: &str, field_rtrn: &mut u32) -> bool {
    let mut field: u32 = 0_u32;
    let ret: bool = LookupString(&FIELD_STRINGS, str, &mut field);
    *field_rtrn = field;
    ret
}
fn fieldText(field: u32) -> &'static str {
    LookupValue(&FIELD_STRINGS, field)
}
#[inline]
fn ReportMismatch(code: u32, action: u32, field: u32, type_0: &str, strict: u32) -> u32 {
    log::error!(
        "[XKB-{:03}] Value of {} field must be of type {}; Action {} definition ignored\n",
        { code },
        fieldText(field),
        type_0,
        ActionTypeText(action)
    );
    (if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
        PARSER_FATAL_ERROR as i32
    } else {
        PARSER_RECOVERABLE_ERROR as i32
    }) as u32
}
#[inline]
fn ReportFormatVersionMismatch(
    action: u32,
    field: u32,
    format: u32,
    versions: &str,
    strict: u32,
) -> u32 {
    log::error!("[XKB-{:03}] Field {} for an action of type {} requires keymap text format {},  but got: {}; Action definition ignored\n",
        XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT as i32,
        fieldText(field),
        ActionTypeText(action),
        versions,
        { format });
    (if strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
        PARSER_FATAL_ERROR as i32
    } else {
        PARSER_SUCCESS as i32
    }) as u32
}
#[inline]
fn ReportIllegal(action: u32, field: u32, strict: u32) -> u32 {
    log::error!(
        "[XKB-{:03}] Field {} is not defined for an action of type {}; Action definition ignored\n",
        XKB_ERROR_INVALID_ACTION_FIELD as i32,
        fieldText(field),
        ActionTypeText(action)
    );
    (if strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
        PARSER_FATAL_ERROR as i32
    } else {
        PARSER_SUCCESS as i32
    }) as u32
}
#[inline]
fn ReportActionNotArray(action: u32, field: u32, strict: u32) -> u32 {
    log::error!(
        "[XKB-{:03}] The {} field in the {} action is not an array; Action definition ignored\n",
        XKB_ERROR_WRONG_FIELD_TYPE as i32,
        fieldText(field),
        ActionTypeText(action)
    );
    (if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
        PARSER_FATAL_ERROR as i32
    } else {
        PARSER_RECOVERABLE_ERROR as i32
    }) as u32
}
fn HandleNoAction(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    _array_ndx: Option<&ExprDef>,
    _value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    log::error!("[XKB-{:03}] The \"{}\" action takes no argument, but got \"{}\" field; Action definition ignored\n",
        XKB_ERROR_INVALID_ACTION_FIELD as i32,
        ActionTypeText(action.action_type()),
        fieldText(field));
    (if keymap_info.strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
        PARSER_FATAL_ERROR as i32
    } else {
        PARSER_SUCCESS as i32
    }) as u32
}
fn CheckBooleanFlag(
    ctx: &xkb_context,
    strict: u32,
    action: u32,
    field: u32,
    flag: xkb_action_flags,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    flags_inout: &mut xkb_action_flags,
) -> u32 {
    let mut set: bool = false;
    if array_ndx.is_some() {
        return ReportActionNotArray(action, field, strict);
    }
    if !ExprResolveBoolean(ctx, value, &mut set) {
        return ReportMismatch(XKB_ERROR_WRONG_FIELD_TYPE, action, field, "boolean", strict);
    }
    if set {
        *flags_inout = (*flags_inout | flag) as xkb_action_flags;
    } else {
        *flags_inout = (*flags_inout & !flag) as xkb_action_flags;
    }
    PARSER_SUCCESS
}
fn CheckModifierField(
    ctx: &xkb_context,
    strict: u32,
    mods: &xkb_mod_set,
    action: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    flags_inout: &mut xkb_action_flags,
    mods_rtrn: &mut u32,
) -> u32 {
    if array_ndx.is_some() {
        return ReportActionNotArray(action, ACTION_FIELD_MODIFIERS, strict);
    }
    if value.common.type_0 == STMT_EXPR_IDENT {
        let ident = if let ExprKind::Ident(id) = &value.kind {
            *id
        } else {
            unreachable!()
        };
        let valStr: &str = xkb_atom_text(&ctx.atom_table, ident);
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
            XKB_ERROR_WRONG_FIELD_TYPE,
            action,
            ACTION_FIELD_MODIFIERS,
            "modifier mask",
            strict,
        );
    }
    *flags_inout = (*flags_inout & !(ACTION_MODS_LOOKUP_MODMAP as i32) as u32) as xkb_action_flags;
    PARSER_SUCCESS
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
fn CheckAffectField(
    ctx: &xkb_context,
    strict: u32,
    action: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    flags_inout: &mut xkb_action_flags,
) -> u32 {
    if array_ndx.is_some() {
        return ReportActionNotArray(action, ACTION_FIELD_AFFECT, strict);
    }
    let mut flags: u32 = 0_u32;
    if !ExprResolveEnum(ctx, value, &mut flags, &LOCK_WHICH) {
        return ReportMismatch(
            XKB_ERROR_WRONG_FIELD_TYPE,
            action,
            ACTION_FIELD_AFFECT,
            "lock, unlock, both, neither",
            strict,
        );
    }
    *flags_inout = (*flags_inout
        & !(ACTION_LOCK_NO_LOCK as i32 | ACTION_LOCK_NO_UNLOCK as i32) as u32)
        as xkb_action_flags;
    *flags_inout = (*flags_inout | flags as xkb_action_flags) as xkb_action_flags;
    PARSER_SUCCESS
}
fn HandleSetLatchLockMods(
    keymap_info: &mut xkb_keymap_info,
    mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let act = action.as_mods_mut();
    let type_0: u32 = act.type_0;
    if field == ACTION_FIELD_MODIFIERS {
        return CheckModifierField(
            ctx,
            keymap_info.strict,
            mods,
            type_0,
            array_ndx,
            value,
            &mut act.flags,
            &mut act.mods.mods,
        );
    }
    if field == ACTION_FIELD_UNLOCK_ON_PRESS {
        if keymap_info.features.mods_unlock_on_press {
            return CheckBooleanFlag(
                ctx,
                keymap_info.strict,
                type_0,
                field,
                ACTION_UNLOCK_ON_PRESS,
                array_ndx,
                value,
                &mut act.flags,
            );
        } else {
            return ReportFormatVersionMismatch(
                type_0,
                field,
                keymap_info.keymap_ref().format,
                ">= 2",
                keymap_info.strict,
            );
        }
    }
    if (type_0 as u32 == ACTION_TYPE_MOD_SET || type_0 as u32 == ACTION_TYPE_MOD_LATCH)
        && field == ACTION_FIELD_CLEAR_LOCKS
    {
        return CheckBooleanFlag(
            ctx,
            keymap_info.strict,
            type_0,
            field,
            ACTION_LOCK_CLEAR,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    if type_0 as u32 == ACTION_TYPE_MOD_LATCH {
        if field == ACTION_FIELD_LATCH_TO_LOCK {
            return CheckBooleanFlag(
                ctx,
                keymap_info.strict,
                type_0,
                field,
                ACTION_LATCH_TO_LOCK,
                array_ndx,
                value,
                &mut act.flags,
            );
        }
        if field == ACTION_FIELD_LATCH_ON_PRESS {
            if keymap_info.features.mods_latch_on_press {
                return CheckBooleanFlag(
                    ctx,
                    keymap_info.strict,
                    type_0,
                    field,
                    ACTION_LATCH_ON_PRESS,
                    array_ndx,
                    value,
                    &mut act.flags,
                );
            } else {
                return ReportFormatVersionMismatch(
                    type_0,
                    field,
                    keymap_info.keymap_ref().format,
                    ">= 2",
                    keymap_info.strict,
                );
            }
        }
    }
    if type_0 as u32 == ACTION_TYPE_MOD_LOCK && field == ACTION_FIELD_AFFECT {
        return CheckAffectField(
            ctx,
            keymap_info.strict,
            type_0,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn CheckGroupField(
    keymap_info: &mut xkb_keymap_info,
    action: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    mut value_ptr: *mut *mut ExprDef,
    flags_inout: &mut xkb_action_flags,
    group_rtrn: &mut i32,
) -> u32 {
    unsafe {
        let spec: *const ExprDef;
        let mut idx: u32 = 0_u32;
        let mut flags: xkb_action_flags = *flags_inout;
        if array_ndx.is_some() {
            return ReportActionNotArray(action, ACTION_FIELD_GROUP, keymap_info.strict);
        }
        if value.common.type_0 == STMT_EXPR_NEGATE || value.common.type_0 == STMT_EXPR_UNARY_PLUS {
            flags = (flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            spec = if let ExprKind::Unary { child, .. } = &value.kind {
                child.raw()
            } else {
                unreachable!()
            };
            // SAFETY: Option<Box<ExprDef>> has the same layout as *mut ExprDef
            // (null-pointer optimisation).  Casting the field address lets us
            // read/null the field through value_ptr without changing the handler
            // signature.  Setting to null is equivalent to .take() + into_raw
            // (intentionally leaks the Box, ownership moves to pending_computations).
            value_ptr = if let ExprKind::Unary { ref mut child, .. } = (**value_ptr).kind {
                child as *mut Option<Box<ExprDef>> as *mut *mut ExprDef
            } else {
                unreachable!()
            };
        } else {
            flags = (flags as u32 | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
            spec = value as *const ExprDef;
        }
        let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH != 0;
        let mut pending: bool = false;
        let ret: u32 =
            ExprResolveGroup(keymap_info, &*spec, absolute, &mut idx, &mut pending) as u32;
        if ret as u32 != PARSER_SUCCESS && !pending {
            ReportMismatch(
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
                action,
                ACTION_FIELD_GROUP,
                "integer",
                keymap_info.strict,
            );
            return ret;
        }
        if pending {
            flags = (flags as u32 | ACTION_PENDING_COMPUTATION) as xkb_action_flags;
            let pending_index: u32 = keymap_info.pending_computations.len() as u32;
            keymap_info.pending_computations.push(pending_computation {
                expr: box_from_raw(*value_ptr),
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
                if value.common.type_0 == STMT_EXPR_NEGATE {
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
fn HandleSetLatchLockGroup(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0: u32 = action.action_type();
    if field == ACTION_FIELD_GROUP {
        let act = action.as_group_mut();
        return CheckGroupField(
            keymap_info,
            type_0,
            array_ndx,
            value,
            value_ptr,
            &mut act.flags,
            &mut act.group,
        );
    }
    let act = action.as_group_mut();
    if (type_0 as u32 == ACTION_TYPE_GROUP_SET || type_0 as u32 == ACTION_TYPE_GROUP_LATCH)
        && field == ACTION_FIELD_CLEAR_LOCKS
    {
        return CheckBooleanFlag(
            ctx,
            keymap_info.strict,
            type_0,
            field,
            ACTION_LOCK_CLEAR,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    if type_0 as u32 == ACTION_TYPE_GROUP_LATCH && field == ACTION_FIELD_LATCH_TO_LOCK {
        return CheckBooleanFlag(
            ctx,
            keymap_info.strict,
            type_0,
            field,
            ACTION_LATCH_TO_LOCK,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    if type_0 as u32 == ACTION_TYPE_GROUP_LOCK && field == ACTION_FIELD_LOCK_ON_RELEASE {
        if keymap_info.features.group_lock_on_release {
            return CheckBooleanFlag(
                ctx,
                keymap_info.strict,
                type_0,
                field,
                ACTION_LOCK_ON_RELEASE,
                array_ndx,
                value,
                &mut act.flags,
            );
        } else {
            return ReportFormatVersionMismatch(
                type_0,
                field,
                keymap_info.keymap_ref().format,
                ">= v2",
                keymap_info.strict,
            );
        }
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandleMovePtr(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_ptr_mut();
    if field == ACTION_FIELD_X || field == ACTION_FIELD_Y {
        let mut val: i64 = 0_i64;
        let absolute: bool =
            value.common.type_0 != STMT_EXPR_NEGATE && value.common.type_0 != STMT_EXPR_UNARY_PLUS;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if !ExprResolveInteger(ctx, value, &mut val) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "integer",
                keymap_info.strict,
            );
        }
        if val < i16::MIN as i64 || val > i16::MAX as i64 {
            log::error!("The {} field in the {} action must be in range {}..{}, but got {}. Action definition ignored\n",
                fieldText(field),
                ActionTypeText(type_0),
                -32767_i32 - 1_i32,
                32767_i32,
                val);
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        if field == ACTION_FIELD_X {
            if absolute {
                act.flags = (act.flags | ACTION_ABSOLUTE_X) as xkb_action_flags;
            }
            act.x = val as i16;
        } else {
            if absolute {
                act.flags = (act.flags | ACTION_ABSOLUTE_Y) as xkb_action_flags;
            }
            act.y = val as i16;
        }
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_ACCEL {
        return CheckBooleanFlag(
            ctx,
            keymap_info.strict,
            type_0,
            field,
            ACTION_ACCEL,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandlePtrBtn(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_btn_mut();
    if field == ACTION_FIELD_BUTTON {
        let mut btn: i64 = 0_i64;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if !ExprResolveButton(ctx, value, &mut btn) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "integer (range 1..5)",
                keymap_info.strict,
            );
        }
        if !(0_i64..=5_i64).contains(&btn) {
            log::error!("Button must specify default or be in the range 1..5; Illegal button value {} ignored\n",
                btn);
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        act.button = btn as u8;
        return PARSER_SUCCESS;
    } else if type_0 == ACTION_TYPE_PTR_LOCK && field == ACTION_FIELD_AFFECT {
        return CheckAffectField(
            ctx,
            keymap_info.strict,
            type_0,
            array_ndx,
            value,
            &mut act.flags,
        );
    } else if field == ACTION_FIELD_COUNT {
        let mut val: i64 = 0_i64;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if !ExprResolveInteger(ctx, value, &mut val) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "integer",
                keymap_info.strict,
            );
        }
        if !(0_i64..=255_i64).contains(&val) {
            log::error!(
                "The count field must have a value in the range 0..255; Illegal count {} ignored\n",
                val
            );
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        act.count = val as u8;
        return PARSER_SUCCESS;
    }
    ReportIllegal(type_0, field, keymap_info.strict)
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
fn HandleSetPtrDflt(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_dflt_mut();
    if field == ACTION_FIELD_AFFECT {
        let mut val: u32 = 0_u32;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if !ExprResolveEnum(ctx, value, &mut val, &PTR_DFLTS) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "pointer component",
                keymap_info.strict,
            );
        }
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_BUTTON || field == ACTION_FIELD_VALUE {
        let button: &ExprDef;
        let mut btn: i64 = 0_i64;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if value.common.type_0 == STMT_EXPR_NEGATE || value.common.type_0 == STMT_EXPR_UNARY_PLUS {
            act.flags = (act.flags & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            button = if let ExprKind::Unary { child, .. } = &value.kind {
                unsafe { &*child.raw() }
            } else {
                unreachable!()
            };
        } else {
            act.flags = (act.flags | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
            button = value;
        }
        if !ExprResolveButton(ctx, button, &mut btn) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "integer (range 1..5)",
                keymap_info.strict,
            );
        }
        if !(0_i64..=5_i64).contains(&btn) {
            log::error!("New default button value must be in the range 1..5; Illegal default button value {} ignored\n",
                btn);
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        if btn == 0_i64 {
            log::error!("Cannot set default pointer button to \"default\"; Illegal default button setting ignored\n");
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        act.value = (if value.common.type_0 == STMT_EXPR_NEGATE {
            -btn
        } else {
            btn
        }) as i8;
        return PARSER_SUCCESS;
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandleSwitchScreen(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_screen_mut();
    if field == ACTION_FIELD_SCREEN {
        let scrn: &ExprDef;
        let mut val: i64 = 0_i64;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if value.common.type_0 == STMT_EXPR_NEGATE || value.common.type_0 == STMT_EXPR_UNARY_PLUS {
            act.flags = (act.flags & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            scrn = if let ExprKind::Unary { child, .. } = &value.kind {
                unsafe { &*child.raw() }
            } else {
                unreachable!()
            };
        } else {
            act.flags = (act.flags | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
            scrn = value;
        }
        if !ExprResolveInteger(ctx, scrn, &mut val) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "integer (-128..127)",
                keymap_info.strict,
            );
        }
        val = if value.common.type_0 == STMT_EXPR_NEGATE {
            -val
        } else {
            val
        };
        if val < i8::MIN as i64 || val > i8::MAX as i64 {
            log::error!(
                "Screen index must be in the range {}..{}; Illegal screen value {} ignored\n",
                -128_i32,
                127_i32,
                val
            );
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        act.screen = val as i8;
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_SAME {
        return CheckBooleanFlag(
            ctx,
            keymap_info.strict,
            type_0,
            field,
            ACTION_SAME_SCREEN,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandleSetLockControls(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_ctrls_mut();
    if field == ACTION_FIELD_CONTROLS {
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        let mut mask: u32 = 0_u32;
        let offset: u8 = keymap_info.features.controls_name_offset;
        if !ExprResolveMask(ctx, value, &mut mask, &ctrlMaskNames[offset as usize..]) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "controls mask",
                keymap_info.strict,
            );
        }
        act.ctrls = mask as xkb_action_controls;
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_AFFECT && type_0 == ACTION_TYPE_CTRL_LOCK {
        return CheckAffectField(
            ctx,
            keymap_info.strict,
            type_0,
            array_ndx,
            value,
            &mut act.flags,
        );
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandleRedirectKey(
    keymap_info: &mut xkb_keymap_info,
    mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let type_0 = action.action_type();
    let act = action.as_redirect_mut();
    if field == ACTION_FIELD_KEYCODE {
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if value.common.type_0 == STMT_EXPR_IDENT {
            let ident = if let ExprKind::Ident(id) = &value.kind {
                *id
            } else {
                unreachable!()
            };
            let valStr: &str = xkb_atom_text(&keymap_info.keymap_ref().ctx.atom_table, ident);
            if !valStr.is_empty() && valStr.eq_ignore_ascii_case("auto") {
                act.keycode = keymap_info.keymap_ref().redirect_key_auto;
                return PARSER_SUCCESS;
            }
        }
        if value.common.type_0 != STMT_EXPR_KEYNAME_LITERAL {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "key name",
                keymap_info.strict,
            );
        }
        let key_name_val = if let ExprKind::KeyName(kn) = &value.kind {
            *kn
        } else {
            unreachable!()
        };
        let keymap: *const xkb_keymap = keymap_info.keymap;
        let key: *const xkb_key = unsafe { XkbKeyByName(keymap, key_name_val, true) };
        if key.is_null() {
            log::error!(
                "RedirectKey field {} cannot resolve <{}> to a valid key\n",
                fieldText(field),
                xkb_atom_text(&keymap_info.keymap_ref().ctx.atom_table, key_name_val)
            );
            return (if keymap_info.strict & PARSER_NO_FIELD_VALUE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        act.keycode = unsafe { (*key).keycode };
        return PARSER_SUCCESS;
    }
    if field == ACTION_FIELD_MODIFIERS || field == ACTION_FIELD_MODS_TO_CLEAR {
        let mut flags: xkb_action_flags = 0 as xkb_action_flags;
        let mut m: u32 = 0_u32;
        let ctx: &xkb_context = keymap_info.ctx();
        let r: u32 = CheckModifierField(
            ctx,
            keymap_info.strict,
            mods,
            type_0,
            array_ndx,
            value,
            &mut flags,
            &mut m,
        );
        if r as u32 != PARSER_SUCCESS {
            return r;
        }
        if flags as u64 != 0 {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                type_0,
                field,
                "modifier mask",
                keymap_info.strict,
            );
        }
        act.affect |= m;
        if field == ACTION_FIELD_MODIFIERS {
            act.mods |= m;
        } else {
            act.mods &= !m;
        }
        return PARSER_SUCCESS;
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandleUnsupported(
    _keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    _action: &mut xkb_action,
    _field: u32,
    _array_ndx: Option<&ExprDef>,
    _value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    PARSER_SUCCESS
}
fn HandlePrivate(
    keymap_info: &mut xkb_keymap_info,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: &ExprDef,
    _value_ptr: *mut *mut ExprDef,
) -> u32 {
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_priv_mut();
    if field == ACTION_FIELD_TYPE {
        let mut type_0: i64 = 0_i64;
        if array_ndx.is_some() {
            return ReportActionNotArray(ACTION_TYPE_PRIVATE, field, keymap_info.strict);
        }
        if !ExprResolveInteger(ctx, value, &mut type_0) {
            return ReportMismatch(
                XKB_ERROR_WRONG_FIELD_TYPE,
                ACTION_TYPE_PRIVATE,
                field,
                "integer",
                keymap_info.strict,
            );
        }
        if !(0_i64..=255_i64).contains(&type_0) {
            log::error!(
                "Private action type must be in the range 0..255; Illegal type {} ignored\n",
                type_0
            );
            return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        if type_0 < ACTION_TYPE_PRIVATE as i64 {
            log::info!(
                "Private actions of type {} are not supported; Ignored\n",
                ActionTypeText(type_0 as u32)
            );
            act.type_0 = ACTION_TYPE_NONE;
        } else {
            act.type_0 = type_0 as u32;
        }
        return PARSER_SUCCESS;
    } else if field == ACTION_FIELD_DATA {
        if array_ndx.is_none() {
            let mut val: u32 = XKB_ATOM_NONE;
            if !ExprResolveString(ctx, value, &mut val) {
                return ReportMismatch(
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    act.type_0,
                    field,
                    "string",
                    keymap_info.strict,
                );
            }
            let str_bytes: &str = xkb_atom_text(&ctx.atom_table, val);
            let len: usize = str_bytes.len();
            if len < 1_usize || len > std::mem::size_of::<[u8; 7]>() {
                log::warn!(
                    "A private action has {} data bytes, but got: {}; Illegal data ignored\n",
                    std::mem::size_of::<[u8; 7]>(),
                    len
                );
                return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as u32;
            }
            unsafe {
                std::ptr::write_bytes::<[u8; 7]>(
                    &raw mut act.data as *mut u8 as *mut [u8; 7],
                    0u8,
                    1,
                );
                std::ptr::copy_nonoverlapping(
                    str_bytes.as_bytes().as_ptr(),
                    &raw mut act.data as *mut u8,
                    len,
                );
            }
            return PARSER_SUCCESS;
        } else {
            let array_ndx = array_ndx.unwrap();
            let mut ndx: i64 = 0_i64;
            let mut datum: i64 = 0_i64;
            if !ExprResolveInteger(ctx, array_ndx, &mut ndx) {
                log::error!("Array subscript must be integer; Illegal subscript ignored\n");
                return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as u32;
            }
            if ndx < 0_i64 || ndx as usize >= std::mem::size_of::<[u8; 7]>() {
                log::error!("The data for a private action is {} bytes long; Attempt to use data[{}] ignored\n",
                    std::mem::size_of::<[u8; 7]>(),
                    ndx);
                return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as u32;
            }
            if !ExprResolveInteger(ctx, value, &mut datum) {
                return ReportMismatch(
                    XKB_ERROR_WRONG_FIELD_TYPE,
                    act.type_0,
                    field,
                    "integer",
                    keymap_info.strict,
                );
            }
            if !(0_i64..=255_i64).contains(&datum) {
                log::error!(
                    "All data for a private action must be 0..255; Illegal datum {} ignored\n",
                    datum
                );
                return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                    PARSER_FATAL_ERROR as i32
                } else {
                    PARSER_RECOVERABLE_ERROR as i32
                }) as u32;
            }
            act.data[ndx as usize] = datum as u8;
            return PARSER_SUCCESS;
        }
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
static HANDLE_ACTION: [actionHandler; 21] = {
    [
        Some(
            HandleNoAction
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleNoAction
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockMods
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockMods
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockMods
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockGroup
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockGroup
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockGroup
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleMovePtr
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandlePtrBtn
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandlePtrBtn
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetPtrDflt
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleNoAction
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSwitchScreen
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLockControls
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleSetLockControls
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleRedirectKey
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleUnsupported
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandleUnsupported
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        Some(
            HandlePrivate
                as fn(
                    &mut xkb_keymap_info,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    &ExprDef,
                    *mut *mut ExprDef,
                ) -> u32,
        ),
        None,
    ]
};
pub fn HandleActionDef(
    keymap_info: &mut xkb_keymap_info,
    info: &mut ActionsInfo,
    mods: &xkb_mod_set,
    def: &mut ExprDef,
    action: &mut xkb_action,
) -> u32 {
    let ctx: &xkb_context = unsafe { &(*keymap_info.keymap).ctx };
    if def.common.type_0 != STMT_EXPR_ACTION_DECL {
        log::error!(
            "[XKB-{:03}] Expected an action definition, found {}\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(def.common.type_0)
        );
        return PARSER_FATAL_ERROR;
    }
    let (action_name_atom, action_args) = if let ExprKind::Action { name, ref mut args } = def.kind
    {
        (name, args as *mut Vec<ExprDef>)
    } else {
        unreachable!()
    };
    let action_name: &str = xkb_atom_text(&ctx.atom_table, action_name_atom);
    let mut handler_type: u32 = ACTION_TYPE_NONE;
    if !stringToActionType(action_name, &mut handler_type) {
        log::error!(
            "[XKB-{:03}] Unknown action \"{}\"\n",
            XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
            action_name
        );
        handler_type = ACTION_TYPE_UNKNOWN;
        if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION != 0 {
            return PARSER_FATAL_ERROR;
        }
    }
    *action = info.actions[handler_type as usize];
    if handler_type as u32 == ACTION_TYPE_UNSUPPORTED_LEGACY {
        log::warn!(
            "[XKB-{:03}] Unsupported legacy action type \"{}\".\n",
            XKB_WARNING_UNSUPPORTED_LEGACY_ACTION as i32,
            action_name
        );
        action.set_none();
    }
    let mut ret: u32 = PARSER_SUCCESS;
    // SAFETY: we need raw pointer to action_args (Vec inside def) because
    // we pass &mut keymap_info and &mut action to the handler, and the
    // handler signature requires *mut *mut ExprDef for ownership transfer.
    // The action_args pointer is into def which is a separate allocation.
    for arg in unsafe { &mut *action_args } {
        let value: &ExprDef;
        let mut value_ptr: *mut *mut ExprDef = std::ptr::null_mut();
        let field: *mut ExprDef;
        let mut arrayRtrn_opt: Option<&ExprDef> = None;
        let mut elemRtrn: &str = "";
        let mut fieldRtrn: &str = "";
        if arg.common.type_0 == STMT_EXPR_ASSIGN {
            let (left_ptr, right_ref) = if let ExprKind::Binary {
                ref mut left,
                ref mut right,
                ..
            } = arg.kind
            {
                // SAFETY: Option<Box<ExprDef>> has same layout as *mut ExprDef
                (
                    left.raw(),
                    right as *mut Option<Box<ExprDef>> as *mut *mut ExprDef,
                )
            } else {
                unreachable!()
            };
            field = left_ptr;
            value = unsafe { &**right_ref };
            value_ptr = right_ref;
        } else if arg.common.type_0 == STMT_EXPR_NOT || arg.common.type_0 == STMT_EXPR_INVERT {
            field = if let ExprKind::Unary { child, .. } = &arg.kind {
                child.raw()
            } else {
                unreachable!()
            };
            value = &CONST_FALSE.0;
        } else {
            field = arg as *mut ExprDef;
            value = &CONST_TRUE.0;
        }
        if !ExprResolveLhs(
            ctx,
            unsafe { &*field },
            &mut elemRtrn,
            &mut fieldRtrn,
            &mut arrayRtrn_opt,
        ) {
            return PARSER_FATAL_ERROR;
        }
        if !elemRtrn.is_empty() {
            log::error!("[XKB-{:03}] Cannot change defaults in an action definition; Ignoring attempt to change \"{}.{}\".\n",
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                elemRtrn,
                fieldRtrn);
            return PARSER_FATAL_ERROR;
        }
        let mut fieldNdx: u32 = ACTION_FIELD_CLEAR_LOCKS;
        if !stringToField(fieldRtrn, &mut fieldNdx) {
            log::error!(
                "[XKB-{:03}] Unknown field name {} for action {} discarded\n",
                XKB_ERROR_INVALID_ACTION_FIELD as i32,
                fieldRtrn,
                ActionTypeText(action.action_type())
            );
            if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
                return PARSER_FATAL_ERROR;
            }
        } else {
            match HANDLE_ACTION[handler_type as usize].expect("non-null function pointer")(
                keymap_info,
                mods,
                action,
                fieldNdx,
                arrayRtrn_opt,
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
    }
    (if action.action_type() == ACTION_TYPE_UNKNOWN {
        PARSER_RECOVERABLE_ERROR
    } else {
        ret as u32
    }) as u32
}
pub fn SetDefaultActionField(
    keymap_info: &mut xkb_keymap_info,
    info: &mut ActionsInfo,
    mods: &mut xkb_mod_set,
    elem: &str,
    field: &str,
    array_ndx: Option<&ExprDef>,
    value_ptr: *mut *mut ExprDef,
    merge: merge_mode,
) -> u32 {
    unsafe {
        let value: &ExprDef = &**value_ptr;
        let mut action: u32 = ACTION_TYPE_NONE;
        if !stringToActionType(elem, &mut action) {
            log::error!(
                "[XKB-{:03}] Unknown action \"{}\"\n",
                XKB_ERROR_UNKNOWN_ACTION_TYPE as i32,
                elem
            );
            return (if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        let mut action_field: u32 = ACTION_FIELD_CLEAR_LOCKS;
        if !stringToField(field, &mut action_field) {
            log::error!(
                "[XKB-{:03}] Unknown action field \"{}\"\n",
                XKB_ERROR_INVALID_ACTION_FIELD as i32,
                field
            );
            return (if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as u32;
        }
        let into: &mut xkb_action = &mut info.actions[action as usize];
        let mut from: xkb_action = *into;
        let ret: u32 = HANDLE_ACTION[action as usize].expect("non-null function pointer")(
            keymap_info,
            mods,
            &mut from,
            action_field,
            array_ndx,
            value,
            value_ptr,
        ) as u32;
        if ret as u32 != PARSER_SUCCESS {
            return ret;
        }
        if !action_equal(into, &from) {
            let replace: bool = merge != MERGE_AUGMENT;
            log::warn!(
                "Conflicting field \"{}\" for default action \"{}\"; Using {}, ignore {}\n",
                fieldText(action_field),
                ActionTypeText(action),
                if replace { "from" } else { "into" },
                if replace { "into" } else { "from" }
            );
            if replace {
                *into = from;
            }
        }
        PARSER_SUCCESS
    }
}
use crate::xkb::shared_types::*;
