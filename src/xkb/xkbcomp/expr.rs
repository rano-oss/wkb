use super::prelude::*;
use crate::xkb::text::{buttonNames, GROUP_LAST_INDEX_NAME};
extern "C" {
    pub fn imaxabs(__n: i64) -> i64;
}
pub use crate::xkb::keymap_priv::XkbModNameToIndex;
pub use crate::xkb::shared_ast_types::stmt_type_to_operator_char;
pub use crate::xkb::shared_types::{MOD_REAL_MASK_ALL, XKB_LEVEL_MAX_IMPL};
use crate::xkb::utils::istrneq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupModMaskPriv {
    pub mods: *const xkb_mod_set,
    pub mod_type: mod_type,
}
pub type IdentLookupFunc = Option<
    unsafe fn(
        *mut xkb_context,
        *const ::core::ffi::c_void,
        xkb_atom_t,
        *mut u32,
        *mut bool,
    ) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_integer_pattern {
    pub prefix: *const i8,
    pub prefix_length: usize,
    pub min: u32,
    pub max: u32,
    pub entries: *const LookupEntry,
    pub pending_entries: *const LookupEntry,
    pub is_mask: bool,
    pub error_id: xkb_message_code,
}
static mut level_name_pattern: named_integer_pattern = named_integer_pattern {
    prefix: std::ptr::null(),
    prefix_length: 0,
    min: 0,
    max: 0,
    entries: std::ptr::null(),
    pending_entries: std::ptr::null(),
    is_mask: false,
    error_id: 0 as xkb_message_code,
};
pub unsafe fn ExprResolveLhs(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut elem_rtrn: *mut *const i8,
    mut field_rtrn: *mut *const i8,
    mut index_rtrn: *mut *mut ExprDef,
) -> bool {
    unsafe {
        match (*expr).common.type_0 as u32 {
            10 => {
                *elem_rtrn = std::ptr::null();
                *field_rtrn = xkb_atom_text(ctx, (*expr).ident.ident);
                *index_rtrn = std::ptr::null_mut();
                return !(*field_rtrn).is_null();
            }
            12 => {
                *elem_rtrn = xkb_atom_text(ctx, (*expr).field_ref.element);
                *field_rtrn = xkb_atom_text(ctx, (*expr).field_ref.field);
                *index_rtrn = std::ptr::null_mut();
                return !(*elem_rtrn).is_null() && !(*field_rtrn).is_null();
            }
            13 => {
                *elem_rtrn = xkb_atom_text(ctx, (*expr).array_ref.element);
                *field_rtrn = xkb_atom_text(ctx, (*expr).array_ref.field);
                *index_rtrn = (*expr).array_ref.entry as *mut ExprDef;
                if (*expr).array_ref.element != XKB_ATOM_NONE as xkb_atom_t
                    && (*elem_rtrn).is_null()
                {
                    return false;
                }
                if (*field_rtrn).is_null() {
                    return false;
                }
                return true;
            }
            _ => {}
        }
        xkb_logf!(
            ctx,
            XKB_LOG_LEVEL_CRITICAL,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Unexpected operator {} in ResolveLhs\n",
            XKB_ERROR_INVALID_XKB_SYNTAX as i32,
            (*expr).common.type_0 as u32,
        );
        return false;
    }
}
unsafe fn SimpleLookup(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut u32,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE as xkb_atom_t {
            return false;
        }
        let mut str: *const i8 = xkb_atom_text(ctx, field);
        let mut entry: *const LookupEntry = priv_0 as *const LookupEntry;
        while !entry.is_null() && !(*entry).name.is_null() {
            if istreq(str, (*entry).name) {
                *val_rtrn = (*entry).value;
                return true;
            }
            entry = entry.offset(1);
        }
        return false;
    }
}
unsafe fn NamedIntegerPatternLookup(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut u32,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE as xkb_atom_t {
            return false;
        }
        let str: *const i8 = xkb_atom_text(ctx, field) as *const i8;
        let pattern: *const named_integer_pattern = priv_0 as *const named_integer_pattern;
        let count: i32 = if istrneq(str, (*pattern).prefix, (*pattern).prefix_length) as i32 != 0 {
            let s_ptr = str.offset((*pattern).prefix_length as isize);
            let s_bytes = std::ffi::CStr::from_ptr(s_ptr).to_bytes();
            let (val_parsed, c) = crate::xkb::utils::parse_dec_u32(s_bytes);
            *(val_rtrn as *mut u32) = val_parsed;
            c
        } else {
            0 as i32
        };
        if count > 0 as i32
            && *str
                .offset((*pattern).prefix_length as isize)
                .offset(count as isize) as i32
                == '\0' as i32
        {
            if *val_rtrn < (*pattern).min || *val_rtrn > (*pattern).max {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} index {} is out of range ({}..{})\n",
                    (*pattern).error_id as u32,
                    crate::xkb::utils::CStrDisplay((*pattern).prefix),
                    *val_rtrn,
                    (*pattern).min,
                    (*pattern).max,
                );
                return false;
            }
            if (*pattern).is_mask {
                *val_rtrn = ((1 as u32) << (*val_rtrn).wrapping_sub((*pattern).min)) as u32;
            }
            return true;
        } else {
            if !(*pattern).entries.is_null()
                && SimpleLookup(
                    ctx,
                    (*pattern).entries as *const ::core::ffi::c_void,
                    field,
                    val_rtrn,
                    std::ptr::null_mut(),
                ) as i32
                    != 0
            {
                return true;
            }
            if !(*pattern).pending_entries.is_null()
                && !pending_rtrn.is_null()
                && SimpleLookup(
                    ctx,
                    (*pattern).pending_entries as *const ::core::ffi::c_void,
                    field,
                    val_rtrn,
                    std::ptr::null_mut(),
                ) as i32
                    != 0
            {
                *pending_rtrn = true;
                return true;
            }
            return false;
        };
    }
}
unsafe fn LookupModMask(
    mut ctx: *mut xkb_context,
    mut priv_0: *const ::core::ffi::c_void,
    mut field: xkb_atom_t,
    mut val_rtrn: *mut xkb_mod_mask_t,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        let mut str: *const i8 = xkb_atom_text(ctx, field);
        if str.is_null() {
            return false;
        }
        if istreq(str, b"all\0".as_ptr() as *const i8) {
            *val_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        if istreq(str, b"none\0".as_ptr() as *const i8) {
            *val_rtrn = 0 as xkb_mod_mask_t;
            return true;
        }
        let mut arg: *const LookupModMaskPriv = priv_0 as *const LookupModMaskPriv;
        let mut mods: *const xkb_mod_set = (*arg).mods;
        let mod_type: mod_type = (*arg).mod_type;
        let ndx: xkb_mod_index_t = XkbModNameToIndex(mods, field, mod_type) as xkb_mod_index_t;
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            return false;
        }
        *val_rtrn = ((1 as u32) << ndx) as xkb_mod_mask_t;
        return true;
    }
}
pub unsafe fn ExprResolveBoolean(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut set_rtrn: *mut bool,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut ident: *const i8 = std::ptr::null();
        match (*expr).common.type_0 as u32 {
            7 => {
                *set_rtrn = (*expr).boolean.set;
                return true;
            }
            4 | 5 | 6 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {} where boolean was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return false;
            }
            10 => {
                ident = xkb_atom_text(ctx, (*expr).ident.ident);
                if !ident.is_null() {
                    if istreq(ident, b"true\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"yes\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"on\0".as_ptr() as *const i8) as i32 != 0
                    {
                        *set_rtrn = true;
                        return true;
                    } else if istreq(ident, b"false\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"no\0".as_ptr() as *const i8) as i32 != 0
                        || istreq(ident, b"off\0".as_ptr() as *const i8) as i32 != 0
                    {
                        *set_rtrn = false;
                        return true;
                    }
                }
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Identifier \"{}\" of type boolean is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    crate::xkb::utils::CStrDisplay(ident),
                );
                return false;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type boolean is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return false;
            }
            24 | 22 => {
                ok = ExprResolveBoolean(ctx, (*expr).unary.child, set_rtrn);
                if ok {
                    *set_rtrn = !*set_rtrn;
                }
                return ok;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 25 | 14 | 11 | 16 | 15 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} of boolean values not permitted\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator {} in ResolveBoolean\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
            }
        }
        return false;
    }
}
unsafe fn ExprResolveIntegerLookup(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut i64,
    mut pending: *mut bool,
    mut lookup: IdentLookupFunc,
    mut lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut l: i64 = 0 as i64;
        let mut r: i64 = 0 as i64;
        let mut u: u32 = 0 as u32;
        let mut left: *mut ExprDef = std::ptr::null_mut();
        let mut right: *mut ExprDef = std::ptr::null_mut();
        match (*expr).common.type_0 as u32 {
            5 => {
                *val_rtrn = (*expr).integer.ival;
                return true;
            }
            4 | 6 | 7 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {} where an int was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return false;
            }
            10 => {
                if lookup.is_some() {
                    ok = lookup.expect("non-null function pointer")(
                        ctx,
                        lookupPriv,
                        (*expr).ident.ident,
                        &raw mut u,
                        pending,
                    );
                }
                if !ok {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                        XKB_ERROR_INVALID_IDENTIFIER as i32,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
                    );
                } else {
                    *val_rtrn = u as i64;
                }
                if !pending.is_null() && *pending as i32 != 0 {
                    return false;
                }
                return ok;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return false;
            }
            17 | 18 | 19 | 20 => {
                left = (*expr).binary.left as *mut ExprDef;
                right = (*expr).binary.right as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveIntegerLookup(
                        ctx, right, &raw mut r, pending, lookup, lookupPriv,
                    )
                {
                    return false;
                }
                match (*expr).common.type_0 as u32 {
                    17 => {
                        let (c2rust_fresh0, c2rust_fresh1) = l.overflowing_add(r);
                        *val_rtrn = c2rust_fresh0;
                        if c2rust_fresh1 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Addition {} + {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return false;
                        }
                    }
                    18 => {
                        let (c2rust_fresh2, c2rust_fresh3) = l.overflowing_sub(r);
                        *val_rtrn = c2rust_fresh2;
                        if c2rust_fresh3 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Substraction {} - {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return false;
                        }
                    }
                    19 => {
                        let (c2rust_fresh4, c2rust_fresh5) = l.overflowing_mul(r);
                        *val_rtrn = c2rust_fresh4;
                        if c2rust_fresh5 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Multiplication {} * {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn,
                            );
                            return false;
                        }
                    }
                    20 => {
                        if r == 0 as i64 {
                            xkb_logf!(
                                ctx,
                                XKB_LOG_LEVEL_ERROR,
                                XKB_LOG_VERBOSITY_MINIMAL as i32,
                                "[XKB-{:03}] Cannot divide by zero: {} / {}\n",
                                XKB_ERROR_INVALID_OPERATION as i32,
                                l,
                                r,
                            );
                            return false;
                        }
                        *val_rtrn = l / r;
                    }
                    _ => {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] {} of integers not permitted\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            crate::xkb::utils::CStrDisplay(stmt_type_to_string(
                                (*expr).common.type_0
                            )),
                        );
                        return false;
                    }
                }
                return true;
            }
            21 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Assignment operator not implemented yet\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                );
            }
            22 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The ! operator cannot be applied to an integer\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                );
                return false;
            }
            24 | 23 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv) {
                    return false;
                }
                *val_rtrn = if (*expr).common.type_0 as u32 == STMT_EXPR_NEGATE as u32 {
                    -l
                } else {
                    !l
                };
                return true;
            }
            25 => {
                left = (*expr).unary.child as *mut ExprDef;
                return ExprResolveIntegerLookup(ctx, left, val_rtrn, pending, lookup, lookupPriv);
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator {} in ResolveInteger\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
            }
        }
        return false;
    }
}
pub unsafe fn ExprResolveInteger(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut i64,
) -> bool {
    unsafe {
        return ExprResolveIntegerLookup(
            ctx,
            expr,
            val_rtrn,
            std::ptr::null_mut(),
            None,
            std::ptr::null(),
        );
    }
}
pub unsafe fn ExprResolveGroup(
    mut keymap_info: *const xkb_keymap_info,
    mut expr: *const ExprDef,
    mut absolute: bool,
    mut group_rtrn: *mut xkb_layout_index_t,
    mut pending: *mut bool,
) -> xkb_parser_error {
    unsafe {
        static mut pendingGroupIndexNames: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: 0 as u32,
            },
            LookupEntry {
                name: std::ptr::null(),
                value: 0 as u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: b"Group\0".as_ptr() as *const i8,
            prefix_length: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: (*keymap_info).features.max_groups as u32,
            entries: &raw const (*keymap_info).lookup.groupIndexNames as *const LookupEntry,
            pending_entries: &raw const pendingGroupIndexNames as *const LookupEntry,
            is_mask: false,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        let mut result: i64 = 0 as i64;
        if !ExprResolveIntegerLookup(
            (*keymap_info).keymap.ctx,
            expr,
            &raw mut result,
            pending,
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        ) {
            return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        if result < absolute as i64 || result > (*keymap_info).features.max_groups as i64 {
            xkb_logf!(
                (*keymap_info).keymap.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Group index {} is out of range ({}..{})\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                result,
                absolute as i32,
                (*keymap_info).features.max_groups,
            );
            return (if (*keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32 != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        *group_rtrn = result as xkb_layout_index_t;
        return PARSER_SUCCESS;
    }
}
pub unsafe fn ExprResolveLevel(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut level_rtrn: *mut xkb_level_index_t,
) -> bool {
    unsafe {
        let mut result: i64 = 0 as i64;
        if !ExprResolveIntegerLookup(
            ctx,
            expr,
            &raw mut result,
            std::ptr::null_mut(),
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const level_name_pattern as *const ::core::ffi::c_void,
        ) {
            return false;
        }
        if result < 1 as i64 || result > XKB_LEVEL_MAX_IMPL as i64 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Shift level {} is out of range (1..{})\n",
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32,
                result,
                2048 as i32,
            );
            return false;
        }
        *level_rtrn = (result - 1 as i64) as xkb_level_index_t;
        return true;
    }
}
pub unsafe fn ExprResolveButton(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut btn_rtrn: *mut i64,
) -> bool {
    unsafe {
        return ExprResolveIntegerLookup(
            ctx,
            expr,
            btn_rtrn,
            std::ptr::null_mut(),
            Some(
                SimpleLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const buttonNames as *const LookupEntry as *const ::core::ffi::c_void,
        );
    }
}
pub unsafe fn ExprResolveString(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut xkb_atom_t,
) -> bool {
    unsafe {
        match (*expr).common.type_0 as u32 {
            4 => {
                *val_rtrn = (*expr).string.str;
                return true;
            }
            5 | 6 | 7 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {}, expected a string\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return false;
            }
            10 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Identifier \"{}\" of type string not found\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
                );
                return false;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type string not found\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return false;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 24 | 22 | 25 | 14 | 11 | 16 | 15 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] {} of strings not permitted\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return false;
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator {} in ResolveString\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
            }
        }
        return false;
    }
}
pub unsafe fn ExprResolveEnum(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut u32,
    mut values: *const LookupEntry,
) -> bool {
    unsafe {
        if (*expr).common.type_0 as u32 != STMT_EXPR_IDENT as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Found a {} where an enumerated value was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
            );
            return false;
        }
        if !SimpleLookup(
            ctx,
            values as *const ::core::ffi::c_void,
            (*expr).ident.ident,
            val_rtrn,
            std::ptr::null_mut(),
        ) {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Illegal identifier {}; expected one of:\n",
                XKB_ERROR_INVALID_IDENTIFIER as i32,
                crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
            );
            while !values.is_null() && !(*values).name.is_null() {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] \t{}\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    crate::xkb::utils::CStrDisplay((*values).name),
                );
                values = values.offset(1);
            }
            return false;
        }
        return true;
    }
}
unsafe fn ExprResolveMaskLookup(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut val_rtrn: *mut u32,
    mut pending: *mut bool,
    mut lookup: IdentLookupFunc,
    mut lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut l: u32 = 0 as u32;
        let mut r: u32 = 0 as u32;
        let mut v: i64 = 0 as i64;
        let mut left: *mut ExprDef = std::ptr::null_mut();
        let mut right: *mut ExprDef = std::ptr::null_mut();
        let mut bogus: *const i8 = std::ptr::null();
        let mut c2rust_current_block_47: u64;
        match (*expr).common.type_0 as u32 {
            5 => {
                if (*expr).integer.ival < 0 as i64 || (*expr).integer.ival > u32::MAX as i64 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Mask {}{:#x} is out of range (0..{:#x})\n",
                        crate::xkb::utils::CStrDisplay(if (*expr).integer.ival < 0 as i64 {
                            b"-\0".as_ptr() as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        }),
                        imaxabs((*expr).integer.ival as i64),
                        4294967295 as u32,
                    );
                    return false;
                }
                *val_rtrn = (*expr).integer.ival as u32;
                return true;
            }
            4 | 6 | 7 | 8 | 9 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Found {} where a mask was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(stmt_type_to_string((*expr).common.type_0)),
                );
                return false;
            }
            10 => {
                ok = lookup.expect("non-null function pointer")(
                    ctx,
                    lookupPriv,
                    (*expr).ident.ident,
                    val_rtrn,
                    pending,
                );
                if !ok {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                        XKB_ERROR_INVALID_IDENTIFIER as i32,
                        crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).ident.ident)),
                    );
                }
                if !pending.is_null() && *pending as i32 != 0 {
                    return false;
                }
                return ok;
            }
            12 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.element)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, (*expr).field_ref.field)),
                );
                return false;
            }
            13 => {
                bogus = b"array reference\0".as_ptr() as *const i8;
                c2rust_current_block_47 = 6716998617863641615;
            }
            11 => {
                c2rust_current_block_47 = 6716998617863641615;
            }
            17 | 18 | 19 | 20 => {
                left = (*expr).binary.left as *mut ExprDef;
                right = (*expr).binary.right as *mut ExprDef;
                if !ExprResolveMaskLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveMaskLookup(ctx, right, &raw mut r, pending, lookup, lookupPriv)
                {
                    return false;
                }
                match (*expr).common.type_0 as u32 {
                    17 => {
                        *val_rtrn = l | r;
                    }
                    18 => {
                        *val_rtrn = l & !r;
                    }
                    19 | 20 => {
                        xkb_logf!(
                            ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Cannot {} masks; Illegal operation ignored\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            crate::xkb::utils::CStrDisplay(
                                if (*expr).common.type_0 as u32 == STMT_EXPR_DIVIDE as u32 {
                                    b"divide\0".as_ptr() as *const i8
                                } else {
                                    b"multiply\0".as_ptr() as *const i8
                                }
                            ),
                        );
                        return false;
                    }
                    _ => {}
                }
                return true;
            }
            21 => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Assignment operator not implemented yet\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                );
                c2rust_current_block_47 = 11626999923138678822;
            }
            24 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return false;
                }
                if v < 0 as i64 || v > u32::MAX as i64 {
                    xkb_logf!(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Mask {}{:#x} is out of range (0..{:#x})\n",
                        crate::xkb::utils::CStrDisplay(if v < 0 as i64 {
                            b"-\0".as_ptr() as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        }),
                        imaxabs(v as i64),
                        4294967295 as u32,
                    );
                    return false;
                }
                *val_rtrn = !(v as u32);
                return true;
            }
            25 | 23 | 22 => {
                left = (*expr).unary.child as *mut ExprDef;
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return false;
                }
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] The '{}' unary operator cannot be used with a mask\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                    (stmt_type_to_operator_char((*expr).common.type_0) as u8 as char),
                );
                return false;
            }
            _ => {
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_CRITICAL,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unknown operator type {} in ResolveMask\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    (*expr).common.type_0 as u32,
                );
                c2rust_current_block_47 = 11626999923138678822;
            }
        }
        match c2rust_current_block_47 {
            11626999923138678822 => {}
            _ => {
                if bogus.is_null() {
                    bogus = b"function use\0".as_ptr() as *const i8;
                }
                xkb_logf!(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Unexpected {} in mask expression; Expression Ignored\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    crate::xkb::utils::CStrDisplay(bogus),
                );
                return false;
            }
        }
        return false;
    }
}
pub unsafe fn ExprResolveMask(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut mask_rtrn: *mut u32,
    mut values: *const LookupEntry,
) -> bool {
    unsafe {
        return ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn,
            std::ptr::null_mut(),
            Some(
                SimpleLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            values as *const ::core::ffi::c_void,
        );
    }
}
pub unsafe fn ExprResolveModMask(
    mut ctx: *mut xkb_context,
    mut expr: *const ExprDef,
    mut mod_type: mod_type,
    mut mods: *const xkb_mod_set,
    mut mask_rtrn: *mut xkb_mod_mask_t,
) -> bool {
    unsafe {
        let mut priv_0: LookupModMaskPriv = LookupModMaskPriv {
            mods: mods,
            mod_type: mod_type,
        };
        return ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn as *mut u32,
            std::ptr::null_mut(),
            Some(
                LookupModMask
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut xkb_mod_mask_t,
                        *mut bool,
                    ) -> bool,
            ),
            &raw mut priv_0 as *const ::core::ffi::c_void,
        );
    }
}
pub unsafe fn ExprResolveMod(
    mut ctx: *mut xkb_context,
    mut def: *const ExprDef,
    mut mod_type: mod_type,
    mut mods: *const xkb_mod_set,
    mut ndx_rtrn: *mut xkb_mod_index_t,
) -> bool {
    unsafe {
        if (*def).common.type_0 as u32 != STMT_EXPR_IDENT as u32 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot resolve virtual modifier: found {} where a virtual modifier name was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                crate::xkb::utils::CStrDisplay(stmt_type_to_string((*def).common.type_0)),
            );
            return false;
        }
        let mut name: xkb_atom_t = (*def).ident.ident;
        let mut ndx: xkb_mod_index_t = XkbModNameToIndex(mods, name, mod_type);
        if ndx == XKB_MOD_INVALID as xkb_mod_index_t {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot resolve virtual modifier: \"{}\" was not previously declared\n",
                XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER as i32,
                crate::xkb::utils::CStrDisplay(xkb_atom_text(ctx, name)),
            );
            return false;
        }
        *ndx_rtrn = ndx;
        return true;
    }
}
pub unsafe fn ExprResolveGroupMask(
    mut keymap_info: *const xkb_keymap_info,
    mut expr: *const ExprDef,
    mut group_rtrn: *mut xkb_layout_mask_t,
    mut pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        static mut pendingGroupMaskNames: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME.as_ptr(),
                value: 0 as u32,
            },
            LookupEntry {
                name: std::ptr::null(),
                value: 0 as u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: b"Group\0".as_ptr() as *const i8,
            prefix_length: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: (*keymap_info).features.max_groups as u32,
            entries: &raw const (*keymap_info).lookup.groupMaskNames as *const LookupEntry,
            pending_entries: &raw const pendingGroupMaskNames as *const LookupEntry,
            is_mask: true,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        return ExprResolveMaskLookup(
            (*keymap_info).keymap.ctx,
            expr,
            group_rtrn as *mut u32,
            pending_rtrn,
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        xkb_atom_t,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        );
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        level_name_pattern = named_integer_pattern {
            prefix: b"Level\0".as_ptr() as *const i8,
            prefix_length: (std::mem::size_of::<[i8; 6]>()).wrapping_sub(1 as usize),
            min: 1 as u32,
            max: XKB_LEVEL_MAX_IMPL as u32,
            entries: std::ptr::null(),
            pending_entries: std::ptr::null(),
            is_mask: false,
            error_id: XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
        }
    }
}
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
