use crate::xkb::context::xkb_atom_text_bytes;
use crate::xkb_logf;

pub use crate::xkb::messages::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use crate::xkb::shared_ast_types::{
    _ParseCommon, merge_mode, stmt_type, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, ParseCommon, VModDef, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS,
    STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF,
    STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST,
    STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL,
    STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL,
    STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL,
    STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP,
    STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN,
    STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use crate::xkb::shared_types::{
    xkb_mod, xkb_mod_set, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_MODS,
};
use crate::xkb::text::ModMaskText;
use crate::xkb::xkbcomp::expr::ExprResolveModMask;
pub unsafe fn InitVMods(mut info: *mut xkb_mod_set, mut mods: *const xkb_mod_set, mut reset: bool) {
    unsafe {
        *info = *mods;
        if !reset {
            return;
        }
        let mut mod_0: *mut xkb_mod = std::ptr::null_mut();
        let mut vmod: u32 = 0 as u32;
        vmod = 0 as u32;
        mod_0 = &raw mut (*info).mods as *mut xkb_mod;
        while vmod < (*info).num_mods {
            (*mod_0).mapping = 0 as u32;
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*info).explicit_vmods = 0 as u32;
    }
}
pub unsafe fn MergeModSets(
    mut ctx: *mut xkb_context,
    mut into: *mut xkb_mod_set,
    mut from: *const xkb_mod_set,
    mut merge: merge_mode,
) {
    unsafe {
        let clobber: bool = merge as u32 != MERGE_AUGMENT as u32;
        let mut vmod: u32 = 0;
        let mut mod_0: *const xkb_mod = std::ptr::null();
        vmod = 0 as u32;
        mod_0 = &raw const (*from).mods as *const xkb_mod;
        while vmod < (*from).num_mods {
            let mask: u32 = (1 as u32) << vmod;
            if (*mod_0).type_0 as u32 != MOD_VIRT as u32 {
            } else if (*into).mods[vmod as usize].type_0 as u32 == 0 as u32 {
                (*into).mods[vmod as usize] = *mod_0;
                if (*from).explicit_vmods & mask != 0 {
                    (*into).explicit_vmods |= mask;
                }
            } else {
                if (*from).explicit_vmods & mask == 0 {
                } else if (*into).explicit_vmods & mask == 0 {
                    (*into).mods[vmod as usize].mapping = (*mod_0).mapping;
                    (*into).explicit_vmods |= mask;
                } else if (*mod_0).mapping != (*into).mods[vmod as usize].mapping {
                    let use_0: u32 = if clobber as i32 != 0 {
                        (*mod_0).mapping
                    } else {
                        (*into).mods[vmod as usize].mapping
                    };
                    let ignore: u32 = if clobber as i32 != 0 {
                        (*into).mods[vmod as usize].mapping
                    } else {
                        (*mod_0).mapping
                    };
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                        crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*ctx).atom_table, (*mod_0).name)),
                        crate::xkb::utils::ByteSliceDisplay(ModMaskText((*ctx).clone(), MOD_REAL, from, use_0)),
                        crate::xkb::utils::ByteSliceDisplay(ModMaskText((*ctx).clone(), MOD_REAL, from, ignore)),
                    );
                    (*into).mods[vmod as usize].mapping = use_0;
                }
            }
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*into).num_mods = (*from).num_mods;
    }
}
pub unsafe fn HandleVModDef(
    mut ctx: *mut xkb_context,
    mut mods: *mut xkb_mod_set,
    mut stmt: *mut VModDef,
) -> bool {
    unsafe {
        let mut mapping: u32 = 0 as u32;
        if !(*stmt).value.is_null() {
            if !ExprResolveModMask(ctx, (*stmt).value, MOD_REAL, mods, &raw mut mapping) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Declaration of {} ignored\n",
                    crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(
                        &(*ctx).atom_table,
                        (*stmt).name
                    )),
                );
                return false;
            }
        }
        let mut vmod: u32 = 0;
        let mut mod_0: *mut xkb_mod = std::ptr::null_mut();
        vmod = 0 as u32;
        mod_0 = &raw mut (*mods).mods as *mut xkb_mod;
        while vmod < (*mods).num_mods {
            if (*mod_0).name == (*stmt).name {
                if (*mod_0).type_0 as u32 != MOD_VIRT as u32 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Can't add a virtual modifier named \"{}\"; there is already a non-virtual modifier with this name! Ignored\n",
                        crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*ctx).atom_table, (*mod_0).name)),
                    );
                    return false;
                }
                let mask: u32 = (1 as u32) << vmod;
                if (*stmt).value.is_null() {
                    return true;
                } else if (*mods).explicit_vmods & mask == 0 {
                    (*mod_0).mapping = mapping;
                } else if (*mod_0).mapping != mapping {
                    let clobber: bool = (*stmt).merge as u32 != MERGE_AUGMENT as u32;
                    let use_0: u32 = if clobber as i32 != 0 {
                        mapping
                    } else {
                        (*mod_0).mapping
                    };
                    let ignore: u32 = if clobber as i32 != 0 {
                        (*mod_0).mapping
                    } else {
                        mapping
                    };
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                        crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(&(*ctx).atom_table, (*stmt).name)),
                        crate::xkb::utils::ByteSliceDisplay(ModMaskText((*ctx).clone(), MOD_REAL, mods, use_0)),
                        crate::xkb::utils::ByteSliceDisplay(ModMaskText((*ctx).clone(), MOD_REAL, mods, ignore)),
                    );
                    (*mod_0).mapping = use_0;
                }
                (*mods).explicit_vmods |= mask;
                return true;
            }
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        if (*mods).num_mods >= XKB_MAX_MODS {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Cannot define virtual modifier {}: too many modifiers defined (maximum {})\n",
                crate::xkb::utils::ByteSliceDisplay(xkb_atom_text_bytes(
                    &(*ctx).atom_table,
                    (*stmt).name
                )),
                (std::mem::size_of::<u32>()).wrapping_mul(8 as usize) as u32,
            );
            return false;
        }
        (*mods).mods[(*mods).num_mods as usize].name = (*stmt).name;
        (*mods).mods[(*mods).num_mods as usize].type_0 = MOD_VIRT;
        (*mods).mods[(*mods).num_mods as usize].mapping = mapping;
        if !(*stmt).value.is_null() {
            let mask_0: u32 = (1 as u32) << (*mods).num_mods;
            (*mods).explicit_vmods |= mask_0;
        }
        (*mods).num_mods = (*mods).num_mods.wrapping_add(1);
        return true;
    }
}
use crate::xkb::shared_types::*;
