use crate::xkb_logf;

pub mod context_h {
    pub use crate::xkb::context_priv::xkb_atom_text;
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
}

pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub const XKB_MAX_MODS: xkb_mod_index_t = (::core::mem::size_of::<xkb_mod_mask_t>() as usize)
        .wrapping_mul(CHAR_BIT as usize)
        as xkb_mod_index_t;
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
pub mod ast_h {
    pub use crate::xkb::shared_ast_types::*;
    pub type C2Rust_Unnamed_1 = DarrayKeysym;
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = 8;}
pub mod text_h {

    pub use crate::xkb::text::ModMaskText;
}
pub mod expr_h {

    pub use crate::xkb::xkbcomp::expr::ExprResolveModMask;
}

pub use self::ast_h::{
    _ParseCommon, merge_mode, stmt_type, C2Rust_Unnamed_1, ExprAction, ExprActionList,
    ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger,
    ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary, ParseCommon, VModDef,
    _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
    MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD,
    STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE,
    STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT,
    STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT,
    STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT,
    STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP,
    STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION,
    STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_atom_text, xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
use self::expr_h::ExprResolveModMask;
pub use self::keymap_h::{
    mod_type, xkb_mod, xkb_mod_set, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_MODS,
};
pub use self::limits_h::CHAR_BIT;
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
use self::text_h::ModMaskText;
pub use crate::xkb::shared_types::darray_size_t;
pub unsafe fn InitVMods(mut info: *mut xkb_mod_set, mut mods: *const xkb_mod_set, mut reset: bool) {
    unsafe {
        *info = *mods;
        if !reset {
            return;
        }
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        let mut vmod: xkb_mod_index_t = 0 as xkb_mod_index_t;
        vmod = 0 as xkb_mod_index_t;
        mod_0 = &raw mut (*info).mods as *mut xkb_mod;
        while vmod < (*info).num_mods {
            (*mod_0).mapping = 0 as xkb_mod_mask_t;
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*info).explicit_vmods = 0 as xkb_mod_mask_t;
    }
}
pub unsafe fn MergeModSets(
    mut ctx: *mut xkb_context,
    mut into: *mut xkb_mod_set,
    mut from: *const xkb_mod_set,
    mut merge: merge_mode,
) {
    unsafe {
        let clobber: bool = merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32;
        let mut vmod: xkb_mod_index_t = 0;
        let mut mod_0: *const xkb_mod = ::core::ptr::null::<xkb_mod>();
        vmod = 0 as xkb_mod_index_t;
        mod_0 = &raw const (*from).mods as *const xkb_mod;
        while vmod < (*from).num_mods {
            let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << vmod;
            if (*mod_0).type_0 as u32 != MOD_VIRT as ::core::ffi::c_int as u32 {
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
                    let use_0: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        (*mod_0).mapping
                    } else {
                        (*into).mods[vmod as usize].mapping
                    };
                    let ignore: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        (*into).mods[vmod as usize].mapping
                    } else {
                        (*mod_0).mapping
                    };
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*mod_0).name)),
                        crate::xkb::utils::CStrDisplay(ModMaskText(ctx, MOD_REAL, from, use_0)),
                        crate::xkb::utils::CStrDisplay(ModMaskText(ctx, MOD_REAL, from, ignore)),
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
        let mut mapping: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if !(*stmt).value.is_null() {
            if !ExprResolveModMask(ctx, (*stmt).value, MOD_REAL, mods, &raw mut mapping) {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    "Declaration of {} ignored\n",
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*stmt).name)),
                );
                return 0 != 0;
            }
        }
        let mut vmod: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        vmod = 0 as xkb_mod_index_t;
        mod_0 = &raw mut (*mods).mods as *mut xkb_mod;
        while vmod < (*mods).num_mods {
            if (*mod_0).name == (*stmt).name {
                if (*mod_0).type_0 as u32 != MOD_VIRT as ::core::ffi::c_int as u32 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Can't add a virtual modifier named \"{}\"; there is already a non-virtual modifier with this name! Ignored\n",
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*mod_0).name)),
                    );
                    return 0 != 0;
                }
                let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << vmod;
                if (*stmt).value.is_null() {
                    return 1 != 0;
                } else if (*mods).explicit_vmods & mask == 0 {
                    (*mod_0).mapping = mapping;
                } else if (*mod_0).mapping != mapping {
                    let clobber: bool =
                        (*stmt).merge as u32 != MERGE_AUGMENT as ::core::ffi::c_int as u32;
                    let use_0: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        mapping
                    } else {
                        (*mod_0).mapping
                    };
                    let ignore: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        (*mod_0).mapping
                    } else {
                        mapping
                    };
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*stmt).name)),
                        crate::xkb::utils::CStrDisplay(ModMaskText(ctx, MOD_REAL, mods, use_0)),
                        crate::xkb::utils::CStrDisplay(ModMaskText(ctx, MOD_REAL, mods, ignore)),
                    );
                    (*mod_0).mapping = use_0;
                }
                (*mods).explicit_vmods |= mask;
                return 1 != 0;
            }
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        if (*mods).num_mods >= XKB_MAX_MODS {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                "Cannot define virtual modifier {}: too many modifiers defined (maximum {})\n",
                crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*stmt).name)),
                (::core::mem::size_of::<xkb_mod_mask_t>() as usize).wrapping_mul(8 as usize)
                    as xkb_mod_index_t,
            );
            return 0 != 0;
        }
        (*mods).mods[(*mods).num_mods as usize].name = (*stmt).name;
        (*mods).mods[(*mods).num_mods as usize].type_0 = MOD_VIRT;
        (*mods).mods[(*mods).num_mods as usize].mapping = mapping;
        if !(*stmt).value.is_null() {
            let mask_0: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << (*mods).num_mods;
            (*mods).explicit_vmods |= mask_0;
        }
        (*mods).num_mods = (*mods).num_mods.wrapping_add(1);
        return 1 != 0;
    }
}
use crate::xkb::shared_types::*;
