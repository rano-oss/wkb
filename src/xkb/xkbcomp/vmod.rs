use crate::xkb::context::xkb_atom_text;

pub use crate::xkb::shared_ast_types::{
    merge_mode, OptBoxRaw, _ParseCommon, stmt_type, ExprDef, ExprKind, ParseCommon, VModDef,
    MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL,
    STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN,
    STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF,
    STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT,
    STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY,
    STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT,
    STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP,
    STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD, _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES,
};
pub use crate::xkb::shared_types::{
    xkb_mod, xkb_mod_set, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_MODS,
};
use crate::xkb::text::ModMaskText;
use crate::xkb::xkbcomp::expr::ExprResolveModMask;
pub fn InitVMods(info: &mut xkb_mod_set, mods: &xkb_mod_set, reset: bool) {
    *info = *mods;
    if !reset {
        return;
    }
    for vmod in 0..info.num_mods as usize {
        info.mods[vmod].mapping = 0_u32;
    }
    info.explicit_vmods = 0_u32;
}
pub fn MergeModSets(
    ctx: &mut xkb_context,
    into: &mut xkb_mod_set,
    from: &xkb_mod_set,
    merge: merge_mode,
) {
    let clobber: bool = merge != MERGE_AUGMENT;
    for vmod in 0..from.num_mods as usize {
        let mod_0 = &from.mods[vmod];
        let mask: u32 = 1_u32 << vmod;
        if mod_0.type_0 != MOD_VIRT {
        } else if into.mods[vmod].type_0 == 0_u32 {
            into.mods[vmod] = *mod_0;
            if from.explicit_vmods & mask != 0 {
                into.explicit_vmods |= mask;
            }
        } else {
            if from.explicit_vmods & mask == 0 {
            } else if into.explicit_vmods & mask == 0 {
                into.mods[vmod].mapping = mod_0.mapping;
                into.explicit_vmods |= mask;
            } else if mod_0.mapping != into.mods[vmod].mapping {
                let use_0: u32 = if clobber {
                    mod_0.mapping
                } else {
                    into.mods[vmod].mapping
                };
                let ignore: u32 = if clobber {
                    into.mods[vmod].mapping
                } else {
                    mod_0.mapping
                };
                log::warn!(
                    "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                    xkb_atom_text(&ctx.atom_table, mod_0.name),
                    ModMaskText(ctx, MOD_REAL, from, use_0),
                    ModMaskText(ctx, MOD_REAL, from, ignore)
                );
                into.mods[vmod].mapping = use_0;
            }
        }
    }
    into.num_mods = from.num_mods;
}
pub fn HandleVModDef(ctx: &mut xkb_context, mods: &mut xkb_mod_set, stmt: &VModDef) -> bool {
    let mut mapping: u32 = 0_u32;
    if stmt.value.is_some()
        && !unsafe {
            ExprResolveModMask(
                ctx as *mut _,
                stmt.value.raw(),
                MOD_REAL,
                mods as *const _,
                &raw mut mapping,
            )
        }
    {
        log::error!(
            "Declaration of {} ignored\n",
            xkb_atom_text(&ctx.atom_table, stmt.name)
        );
        return false;
    }
    for vmod in 0..mods.num_mods as usize {
        if mods.mods[vmod].name == stmt.name {
            if mods.mods[vmod].type_0 != MOD_VIRT {
                log::error!("Can't add a virtual modifier named \"{}\"; there is already a non-virtual modifier with this name! Ignored\n",
                    xkb_atom_text(&ctx.atom_table, mods.mods[vmod].name));
                return false;
            }
            let mask: u32 = 1_u32 << vmod;
            if stmt.value.is_none() {
                return true;
            } else if mods.explicit_vmods & mask == 0 {
                mods.mods[vmod].mapping = mapping;
            } else if mods.mods[vmod].mapping != mapping {
                let clobber: bool = stmt.merge != MERGE_AUGMENT;
                let use_0: u32 = if clobber {
                    mapping
                } else {
                    mods.mods[vmod].mapping
                };
                let ignore: u32 = if clobber {
                    mods.mods[vmod].mapping
                } else {
                    mapping
                };
                log::warn!(
                    "Virtual modifier {} mapping defined multiple times; Using {}, ignoring {}\n",
                    xkb_atom_text(&ctx.atom_table, stmt.name),
                    ModMaskText(ctx, MOD_REAL, mods, use_0),
                    ModMaskText(ctx, MOD_REAL, mods, ignore)
                );
                mods.mods[vmod].mapping = use_0;
            }
            mods.explicit_vmods |= mask;
            return true;
        }
    }
    if mods.num_mods >= XKB_MAX_MODS {
        log::error!(
            "Cannot define virtual modifier {}: too many modifiers defined (maximum {})\n",
            xkb_atom_text(&ctx.atom_table, stmt.name),
            (std::mem::size_of::<u32>()).wrapping_mul(8_usize) as u32
        );
        return false;
    }
    mods.mods[mods.num_mods as usize].name = stmt.name;
    mods.mods[mods.num_mods as usize].type_0 = MOD_VIRT;
    mods.mods[mods.num_mods as usize].mapping = mapping;
    if stmt.value.is_some() {
        let mask_0: u32 = 1_u32 << mods.num_mods;
        mods.explicit_vmods |= mask_0;
    }
    mods.num_mods = mods.num_mods.wrapping_add(1);
    true
}
use crate::xkb::shared_types::*;
