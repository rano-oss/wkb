use super::prelude::*;
use crate::xkb::shared_types::*;
use crate::xkb::text::{buttonNames, GROUP_LAST_INDEX_NAME};

pub use crate::xkb::keymap::XkbModNameToIndex;
pub use crate::xkb::shared_ast_types::stmt_type_to_operator_char;
use crate::xkb::shared_ast_types::{ExprKind, OptBoxRaw};
use crate::xkb::utils::cstr_as_bytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupModMaskPriv {
    pub mods: *const xkb_mod_set,
    pub mod_type: u32,
}
pub type IdentLookupFunc = Option<
    unsafe fn(*mut xkb_context, *const ::core::ffi::c_void, u32, *mut u32, *mut bool) -> bool,
>;
#[derive(Copy, Clone)]
pub struct named_integer_pattern {
    pub prefix: &'static str,
    pub min: u32,
    pub max: u32,
    pub entries: *const LookupEntry,
    pub pending_entries: *const LookupEntry,
    pub is_mask: bool,
    pub error_id: u32,
}
// SAFETY: LEVEL_NAME_PATTERN is never modified and all pointers are either null or
// point to static data. Only accessed in single-threaded parser context.
struct SyncNamedIntegerPattern(named_integer_pattern);
unsafe impl Sync for SyncNamedIntegerPattern {}

static LEVEL_NAME_PATTERN: SyncNamedIntegerPattern =
    SyncNamedIntegerPattern(named_integer_pattern {
        prefix: "Level",
        min: 1_u32,
        max: XKB_LEVEL_MAX_IMPL as u32,
        entries: std::ptr::null(),
        pending_entries: std::ptr::null(),
        is_mask: false,
        error_id: XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
    });
pub unsafe fn ExprResolveLhs<'a>(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    elem_rtrn: &mut &'a str,
    field_rtrn: &mut &'a str,
    index_rtrn: *mut *mut ExprDef,
) -> bool {
    unsafe {
        match (*expr).common.type_0 {
            10 => {
                let ExprKind::Ident(ident) = &(*expr).kind else {
                    unreachable!()
                };
                *elem_rtrn = "";
                *field_rtrn = xkb_atom_text(&(*ctx).atom_table, *ident);
                *index_rtrn = std::ptr::null_mut();
                return !(*field_rtrn).is_empty();
            }
            12 => {
                let ExprKind::FieldRef { element, field } = &(*expr).kind else {
                    unreachable!()
                };
                *elem_rtrn = xkb_atom_text(&(*ctx).atom_table, *element);
                *field_rtrn = xkb_atom_text(&(*ctx).atom_table, *field);
                *index_rtrn = std::ptr::null_mut();
                return !(*elem_rtrn).is_empty() && !(*field_rtrn).is_empty();
            }
            13 => {
                let ExprKind::ArrayRef {
                    element,
                    field,
                    entry,
                } = &(*expr).kind
                else {
                    unreachable!()
                };
                *elem_rtrn = xkb_atom_text(&(*ctx).atom_table, *element);
                *field_rtrn = xkb_atom_text(&(*ctx).atom_table, *field);
                *index_rtrn = entry.raw();
                if *element != XKB_ATOM_NONE && (*elem_rtrn).is_empty() {
                    return false;
                }
                if (*field_rtrn).is_empty() {
                    return false;
                }
                return true;
            }
            _ => {}
        }
        log::error!("[XKB-{:03}] Unexpected operator {} in ResolveLhs\n",
            XKB_ERROR_INVALID_XKB_SYNTAX as i32,
            { (*expr).common.type_0 });
        false
    }
}
unsafe fn SimpleLookup(
    ctx: *mut xkb_context,
    priv_0: *const ::core::ffi::c_void,
    field: u32,
    val_rtrn: *mut u32,
    _pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE {
            return false;
        }
        let str: &str = xkb_atom_text(&(*ctx).atom_table, field);
        let mut entry: *const LookupEntry = priv_0 as *const LookupEntry;
        while !entry.is_null() && !(&(*entry).name).is_empty() {
            if str.eq_ignore_ascii_case((*entry).name) {
                *val_rtrn = (*entry).value;
                return true;
            }
            entry = entry.offset(1);
        }
        false
    }
}
unsafe fn NamedIntegerPatternLookup(
    ctx: *mut xkb_context,
    priv_0: *const ::core::ffi::c_void,
    field: u32,
    val_rtrn: *mut u32,
    pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        if priv_0.is_null() || field == XKB_ATOM_NONE {
            return false;
        }
        let str_bytes: &str = xkb_atom_text(&(*ctx).atom_table, field);
        let pattern: *const named_integer_pattern = priv_0 as *const named_integer_pattern;
        let prefix = (*pattern).prefix;
        let count: i32 = if str_bytes
            .as_bytes()
            .get(..prefix.len())
            .is_some_and(|s| s.eq_ignore_ascii_case(prefix.as_bytes()))
        {
            let suffix = &str_bytes.as_bytes()[prefix.len()..];
            let (val_parsed, c) = crate::xkb::utils::parse_dec_u32(suffix);
            *(val_rtrn as *mut u32) = val_parsed;
            c
        } else {
            0_i32
        };
        if count > 0_i32 && prefix.len() + count as usize == str_bytes.len() {
            if *val_rtrn < (*pattern).min || *val_rtrn > (*pattern).max {
                log::error!("[XKB-{:03}] {} index {} is out of range ({}..{})\n",
                    { (*pattern).error_id },
                    (*pattern).prefix,
                    *val_rtrn,
                    (*pattern).min,
                    (*pattern).max);
                return false;
            }
            if (*pattern).is_mask {
                *val_rtrn = 1_u32 << (*val_rtrn).wrapping_sub((*pattern).min);
            }
            true
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
            false
        }
    }
}
unsafe fn LookupModMask(
    ctx: *mut xkb_context,
    priv_0: *const ::core::ffi::c_void,
    field: u32,
    val_rtrn: *mut u32,
    _pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        let str: &str = xkb_atom_text(&(*ctx).atom_table, field);
        if str.is_empty() {
            return false;
        }
        if str.eq_ignore_ascii_case("all") {
            *val_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        if str.eq_ignore_ascii_case("none") {
            *val_rtrn = 0_u32;
            return true;
        }
        let arg: *const LookupModMaskPriv = priv_0 as *const LookupModMaskPriv;
        let mods: *const xkb_mod_set = (*arg).mods;
        let mod_type: u32 = (*arg).mod_type;
        let ndx: u32 = XkbModNameToIndex(mods, field, mod_type);
        if ndx == XKB_MOD_INVALID {
            return false;
        }
        *val_rtrn = 1_u32 << ndx;
        true
    }
}
pub unsafe fn ExprResolveBoolean(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    set_rtrn: *mut bool,
) -> bool {
    unsafe {
        let ok: bool;
        #[allow(unused_assignments)]
        let mut ident: &str = "";
        match (*expr).common.type_0 {
            7 => {
                let ExprKind::Boolean(set) = &(*expr).kind else {
                    unreachable!()
                };
                *set_rtrn = *set;
                return true;
            }
            4 | 5 | 6 | 8 | 9 => {
                log::error!("[XKB-{:03}] Found {} where boolean was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    stmt_type_to_string((*expr).common.type_0));
                return false;
            }
            10 => {
                let ExprKind::Ident(ident_atom) = &(*expr).kind else {
                    unreachable!()
                };
                ident = xkb_atom_text(&(*ctx).atom_table, *ident_atom);
                if !ident.is_empty() {
                    if ident.eq_ignore_ascii_case("true")
                        || ident.eq_ignore_ascii_case("yes")
                        || ident.eq_ignore_ascii_case("on")
                    {
                        *set_rtrn = true;
                        return true;
                    } else if ident.eq_ignore_ascii_case("false") {
                        *set_rtrn = false;
                        return true;
                    } else if ident.eq_ignore_ascii_case("no") || ident.eq_ignore_ascii_case("off")
                    {
                        *set_rtrn = false;
                        return true;
                    }
                }
                log::error!("[XKB-{:03}] Identifier \"{}\" of type boolean is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    ident);
                return false;
            }
            12 => {
                let ExprKind::FieldRef { element, field } = &(*expr).kind else {
                    unreachable!()
                };
                log::error!("[XKB-{:03}] Default \"{}.{}\" of type boolean is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    xkb_atom_text(&(*ctx).atom_table, *element),
                    xkb_atom_text(&(*ctx).atom_table, *field));
                return false;
            }
            24 | 22 => {
                let ExprKind::Unary { child, .. } = &(*expr).kind else {
                    unreachable!()
                };
                ok = ExprResolveBoolean(ctx, child.raw(), set_rtrn);
                if ok {
                    *set_rtrn = !*set_rtrn;
                }
                return ok;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 25 | 14 | 11 | 16 | 15 => {
                log::error!("[XKB-{:03}] {} of boolean values not permitted\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                    stmt_type_to_string((*expr).common.type_0));
            }
            _ => {
                log::error!("[XKB-{:03}] Unknown operator {} in ResolveBoolean\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    { (*expr).common.type_0 });
            }
        }
        false
    }
}
unsafe fn ExprResolveIntegerLookup(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    val_rtrn: *mut i64,
    pending: *mut bool,
    lookup: IdentLookupFunc,
    lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let mut ok: bool = false;
        let mut l: i64 = 0_i64;
        let mut r: i64 = 0_i64;
        let mut u: u32 = 0_u32;
        let left: *mut ExprDef;
        let right: *mut ExprDef;
        match (*expr).common.type_0 {
            5 => {
                let ExprKind::Integer(ival) = &(*expr).kind else {
                    unreachable!()
                };
                *val_rtrn = *ival;
                return true;
            }
            4 | 6 | 7 | 8 | 9 => {
                log::error!("[XKB-{:03}] Found {} where an int was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    stmt_type_to_string((*expr).common.type_0));
                return false;
            }
            10 => {
                let ExprKind::Ident(ident_atom) = &(*expr).kind else {
                    unreachable!()
                };
                if lookup.is_some() {
                    ok = lookup.expect("non-null function pointer")(
                        ctx,
                        lookupPriv,
                        *ident_atom,
                        &raw mut u,
                        pending,
                    );
                }
                if !ok {
                    log::error!("[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                        XKB_ERROR_INVALID_IDENTIFIER as i32,
                        xkb_atom_text(&(*ctx).atom_table, *ident_atom));
                } else {
                    *val_rtrn = u as i64;
                }
                if !pending.is_null() && *pending as i32 != 0 {
                    return false;
                }
                return ok;
            }
            12 => {
                let ExprKind::FieldRef { element, field } = &(*expr).kind else {
                    unreachable!()
                };
                log::error!("[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    xkb_atom_text(&(*ctx).atom_table, *element),
                    xkb_atom_text(&(*ctx).atom_table, *field));
                return false;
            }
            17..=20 => {
                let ExprKind::Binary {
                    left: bleft,
                    right: bright,
                    ..
                } = &(*expr).kind
                else {
                    unreachable!()
                };
                left = bleft.raw();
                right = bright.raw();
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveIntegerLookup(
                        ctx, right, &raw mut r, pending, lookup, lookupPriv,
                    )
                {
                    return false;
                }
                match (*expr).common.type_0 {
                    17 => {
                        let (c2rust_fresh0, c2rust_fresh1) = l.overflowing_add(r);
                        *val_rtrn = c2rust_fresh0;
                        if c2rust_fresh1 {
                            log::error!("[XKB-{:03}] Addition {} + {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn);
                            return false;
                        }
                    }
                    18 => {
                        let (c2rust_fresh2, c2rust_fresh3) = l.overflowing_sub(r);
                        *val_rtrn = c2rust_fresh2;
                        if c2rust_fresh3 {
                            log::error!("[XKB-{:03}] Substraction {} - {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn);
                            return false;
                        }
                    }
                    19 => {
                        let (c2rust_fresh4, c2rust_fresh5) = l.overflowing_mul(r);
                        *val_rtrn = c2rust_fresh4;
                        if c2rust_fresh5 {
                            log::error!("[XKB-{:03}] Multiplication {} * {} has an invalid mathematical result: {}\n",
                                XKB_ERROR_INTEGER_OVERFLOW as i32,
                                l,
                                r,
                                *val_rtrn);
                            return false;
                        }
                    }
                    20 => {
                        if r == 0_i64 {
                            log::error!("[XKB-{:03}] Cannot divide by zero: {} / {}\n",
                                XKB_ERROR_INVALID_OPERATION as i32,
                                l,
                                r);
                            return false;
                        }
                        *val_rtrn = l / r;
                    }
                    _ => {
                        log::error!("[XKB-{:03}] {} of integers not permitted\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            stmt_type_to_string((*expr).common.type_0));
                        return false;
                    }
                }
                return true;
            }
            21 => {
                log::error!("[XKB-{:03}] Assignment operator not implemented yet\n",
                    XKB_ERROR_INVALID_OPERATION as i32);
            }
            22 => {
                log::error!("[XKB-{:03}] The ! operator cannot be applied to an integer\n",
                    XKB_ERROR_INVALID_OPERATION as i32);
                return false;
            }
            24 | 23 => {
                let ExprKind::Unary { child, .. } = &(*expr).kind else {
                    unreachable!()
                };
                left = child.raw();
                if !ExprResolveIntegerLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv) {
                    return false;
                }
                *val_rtrn = if (*expr).common.type_0 == STMT_EXPR_NEGATE {
                    -l
                } else {
                    !l
                };
                return true;
            }
            25 => {
                let ExprKind::Unary { child, .. } = &(*expr).kind else {
                    unreachable!()
                };
                left = child.raw();
                return ExprResolveIntegerLookup(ctx, left, val_rtrn, pending, lookup, lookupPriv);
            }
            _ => {
                log::error!("[XKB-{:03}] Unknown operator {} in ResolveInteger\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    { (*expr).common.type_0 });
            }
        }
        false
    }
}
pub unsafe fn ExprResolveInteger(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    val_rtrn: *mut i64,
) -> bool {
    unsafe {
        ExprResolveIntegerLookup(
            ctx,
            expr,
            val_rtrn,
            std::ptr::null_mut(),
            None,
            std::ptr::null(),
        )
    }
}
pub unsafe fn ExprResolveGroup(
    keymap_info: *mut xkb_keymap_info,
    expr: *const ExprDef,
    absolute: bool,
    group_rtrn: *mut u32,
    pending: *mut bool,
) -> xkb_parser_error {
    unsafe {
        static PENDING_GROUP_INDEX_NAMES: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME,
                value: 0_u32,
            },
            LookupEntry {
                name: "",
                value: 0_u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: "Group",
            min: 1_u32,
            max: (*keymap_info).features.max_groups,
            entries: &raw const (*keymap_info).lookup.groupIndexNames as *const LookupEntry,
            pending_entries: &PENDING_GROUP_INDEX_NAMES as *const LookupEntry,
            is_mask: false,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        let mut result: i64 = 0_i64;
        if !ExprResolveIntegerLookup(
            &raw mut (*(*keymap_info).keymap).ctx,
            expr,
            &raw mut result,
            pending,
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        u32,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        ) {
            return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        if result < absolute as i64 || result > (*keymap_info).features.max_groups as i64 {
            log::error!("[XKB-{:03}] Group index {} is out of range ({}..{})\n",
                { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                result,
                absolute as i32,
                (*keymap_info).features.max_groups);
            return (if (*keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
                PARSER_FATAL_ERROR as i32
            } else {
                PARSER_RECOVERABLE_ERROR as i32
            }) as xkb_parser_error;
        }
        *group_rtrn = result as u32;
        PARSER_SUCCESS
    }
}
pub unsafe fn ExprResolveLevel(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    level_rtrn: *mut u32,
) -> bool {
    unsafe {
        let mut result: i64 = 0_i64;
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
                        u32,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &LEVEL_NAME_PATTERN.0 as *const _ as *const ::core::ffi::c_void,
        ) {
            return false;
        }
        if result < 1_i64 || result > XKB_LEVEL_MAX_IMPL as i64 {
            log::error!("[XKB-{:03}] Shift level {} is out of range (1..{})\n",
                XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32,
                result,
                2048_i32);
            return false;
        }
        *level_rtrn = (result - 1_i64) as u32;
        true
    }
}
pub unsafe fn ExprResolveButton(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    btn_rtrn: *mut i64,
) -> bool {
    unsafe {
        ExprResolveIntegerLookup(
            ctx,
            expr,
            btn_rtrn,
            std::ptr::null_mut(),
            Some(
                SimpleLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        u32,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &buttonNames as *const _ as *const ::core::ffi::c_void,
        )
    }
}
pub unsafe fn ExprResolveString(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    val_rtrn: *mut u32,
) -> bool {
    unsafe {
        match (*expr).common.type_0 {
            4 => {
                let ExprKind::String(s) = &(*expr).kind else {
                    unreachable!()
                };
                *val_rtrn = *s;
                return true;
            }
            5..=9 => {
                log::error!("[XKB-{:03}] Found {}, expected a string\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    stmt_type_to_string((*expr).common.type_0));
                return false;
            }
            10 => {
                log::error!("[XKB-{:03}] Identifier \"{}\" of type string not found\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    xkb_atom_text(&(*ctx).atom_table, {
                        let ExprKind::Ident(id) = &(*expr).kind else {
                            unreachable!()
                        };
                        *id
                    }));
                return false;
            }
            12 => {
                let ExprKind::FieldRef { element, field } = &(*expr).kind else {
                    unreachable!()
                };
                log::error!("[XKB-{:03}] Default \"{}.{}\" of type string not found\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    xkb_atom_text(&(*ctx).atom_table, *element),
                    xkb_atom_text(&(*ctx).atom_table, *field));
                return false;
            }
            17 | 18 | 19 | 20 | 21 | 23 | 24 | 22 | 25 | 14 | 11 | 16 | 15 => {
                log::error!("[XKB-{:03}] {} of strings not permitted\n",
                    XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                    stmt_type_to_string((*expr).common.type_0));
                return false;
            }
            _ => {
                log::error!("[XKB-{:03}] Unknown operator {} in ResolveString\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    { (*expr).common.type_0 });
            }
        }
        false
    }
}
pub unsafe fn ExprResolveEnum(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    val_rtrn: *mut u32,
    values: &[LookupEntry],
) -> bool {
    unsafe {
        if (*expr).common.type_0 != STMT_EXPR_IDENT {
            log::error!("[XKB-{:03}] Found a {} where an enumerated value was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string((*expr).common.type_0));
            return false;
        }
        let ExprKind::Ident(ident_atom) = &(*expr).kind else {
            unreachable!()
        };
        if !SimpleLookup(
            ctx,
            values.as_ptr() as *const ::core::ffi::c_void,
            *ident_atom,
            val_rtrn,
            std::ptr::null_mut(),
        ) {
            log::error!("[XKB-{:03}] Illegal identifier {}; expected one of:\n",
                XKB_ERROR_INVALID_IDENTIFIER as i32,
                xkb_atom_text(&(*ctx).atom_table, *ident_atom));
            for entry in values {
                if entry.name.is_empty() {
                    break;
                }
                log::error!("[XKB-{:03}] \t{}\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    entry.name);
            }
            return false;
        }
        true
    }
}
unsafe fn ExprResolveMaskLookup(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    val_rtrn: *mut u32,
    pending: *mut bool,
    lookup: IdentLookupFunc,
    lookupPriv: *const ::core::ffi::c_void,
) -> bool {
    unsafe {
        let ok: bool;
        let mut l: u32 = 0_u32;
        let mut r: u32 = 0_u32;
        let mut v: i64 = 0_i64;
        let left: *mut ExprDef;
        let right: *mut ExprDef;
        let mut bogus: Option<&str> = None;
        let c2rust_current_block_47: u64;
        match (*expr).common.type_0 {
            5 => {
                let ExprKind::Integer(ival) = &(*expr).kind else {
                    unreachable!()
                };
                if *ival < 0_i64 || *ival > u32::MAX as i64 {
                    log::error!("Mask {}{:#x} is out of range (0..{:#x})\n",
                        if *ival < 0_i64 { "-" } else { "" },
                        ival.abs(),
                        4294967295_u32);
                    return false;
                }
                *val_rtrn = *ival as u32;
                return true;
            }
            4 | 6 | 7 | 8 | 9 => {
                log::error!("[XKB-{:03}] Found {} where a mask was expected\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    stmt_type_to_string((*expr).common.type_0));
                return false;
            }
            10 => {
                let ExprKind::Ident(ident_atom) = &(*expr).kind else {
                    unreachable!()
                };
                ok = lookup.expect("non-null function pointer")(
                    ctx,
                    lookupPriv,
                    *ident_atom,
                    val_rtrn,
                    pending,
                );
                if !ok {
                    log::error!("[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                        XKB_ERROR_INVALID_IDENTIFIER as i32,
                        xkb_atom_text(&(*ctx).atom_table, *ident_atom));
                }
                if !pending.is_null() && *pending as i32 != 0 {
                    return false;
                }
                return ok;
            }
            12 => {
                let ExprKind::FieldRef { element, field } = &(*expr).kind else {
                    unreachable!()
                };
                log::error!("[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                    xkb_atom_text(&(*ctx).atom_table, *element),
                    xkb_atom_text(&(*ctx).atom_table, *field));
                return false;
            }
            13 => {
                bogus = Some("array reference");
                c2rust_current_block_47 = 6716998617863641615;
            }
            11 => {
                c2rust_current_block_47 = 6716998617863641615;
            }
            17..=20 => {
                let ExprKind::Binary {
                    left: bleft,
                    right: bright,
                    ..
                } = &(*expr).kind
                else {
                    unreachable!()
                };
                left = bleft.raw();
                right = bright.raw();
                if !ExprResolveMaskLookup(ctx, left, &raw mut l, pending, lookup, lookupPriv)
                    || !ExprResolveMaskLookup(ctx, right, &raw mut r, pending, lookup, lookupPriv)
                {
                    return false;
                }
                match (*expr).common.type_0 {
                    17 => {
                        *val_rtrn = l | r;
                    }
                    18 => {
                        *val_rtrn = l & !r;
                    }
                    19 | 20 => {
                        log::error!("[XKB-{:03}] Cannot {} masks; Illegal operation ignored\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            if (*expr).common.type_0 == STMT_EXPR_DIVIDE {
                                "divide"
                            } else {
                                "multiply"
                            });
                        return false;
                    }
                    _ => {}
                }
                return true;
            }
            21 => {
                log::error!("[XKB-{:03}] Assignment operator not implemented yet\n",
                    XKB_ERROR_INVALID_OPERATION as i32);
                c2rust_current_block_47 = 11626999923138678822;
            }
            24 => {
                let ExprKind::Unary { child, .. } = &(*expr).kind else {
                    unreachable!()
                };
                left = child.raw();
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return false;
                }
                if v < 0_i64 || v > u32::MAX as i64 {
                    log::error!("Mask {}{:#x} is out of range (0..{:#x})\n",
                        if v < 0_i64 { "-" } else { "" },
                        v.abs(),
                        4294967295_u32);
                    return false;
                }
                *val_rtrn = !(v as u32);
                return true;
            }
            25 | 23 | 22 => {
                let ExprKind::Unary { child, .. } = &(*expr).kind else {
                    unreachable!()
                };
                left = child.raw();
                if !ExprResolveIntegerLookup(ctx, left, &raw mut v, pending, lookup, lookupPriv) {
                    return false;
                }
                log::error!("[XKB-{:03}] The '{}' unary operator cannot be used with a mask\n",
                    XKB_ERROR_INVALID_OPERATION as i32,
                    (stmt_type_to_operator_char((*expr).common.type_0) as u8 as char));
                return false;
            }
            _ => {
                log::error!("[XKB-{:03}] Unknown operator type {} in ResolveMask\n",
                    XKB_ERROR_UNKNOWN_OPERATOR as i32,
                    { (*expr).common.type_0 });
                c2rust_current_block_47 = 11626999923138678822;
            }
        }
        match c2rust_current_block_47 {
            11626999923138678822 => {}
            _ => {
                if bogus.is_none() {
                    bogus = Some("function use");
                }
                log::error!("[XKB-{:03}] Unexpected {} in mask expression; Expression Ignored\n",
                    XKB_ERROR_WRONG_FIELD_TYPE as i32,
                    bogus.unwrap_or("unknown"));
                return false;
            }
        }
        false
    }
}
pub unsafe fn ExprResolveMask(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    mask_rtrn: *mut u32,
    values: &[LookupEntry],
) -> bool {
    unsafe {
        ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn,
            std::ptr::null_mut(),
            Some(
                SimpleLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        u32,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            values.as_ptr() as *const ::core::ffi::c_void,
        )
    }
}
pub unsafe fn ExprResolveModMask(
    ctx: *mut xkb_context,
    expr: *const ExprDef,
    mod_type: u32,
    mods: *const xkb_mod_set,
    mask_rtrn: *mut u32,
) -> bool {
    unsafe {
        let mut priv_0: LookupModMaskPriv = LookupModMaskPriv { mods, mod_type };
        ExprResolveMaskLookup(
            ctx,
            expr,
            mask_rtrn as *mut u32,
            std::ptr::null_mut(),
            Some(
                LookupModMask
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        u32,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw mut priv_0 as *const ::core::ffi::c_void,
        )
    }
}
pub unsafe fn ExprResolveMod(
    ctx: *mut xkb_context,
    def: *const ExprDef,
    mod_type: u32,
    mods: *const xkb_mod_set,
    ndx_rtrn: *mut u32,
) -> bool {
    unsafe {
        if (*def).common.type_0 != STMT_EXPR_IDENT {
            log::error!("[XKB-{:03}] Cannot resolve virtual modifier: found {} where a virtual modifier name was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string((*def).common.type_0));
            return false;
        }
        let ExprKind::Ident(ident_atom) = &(*def).kind else {
            unreachable!()
        };
        let name: u32 = *ident_atom;
        let ndx: u32 = XkbModNameToIndex(mods, name, mod_type);
        if ndx == XKB_MOD_INVALID {
            log::error!("[XKB-{:03}] Cannot resolve virtual modifier: \"{}\" was not previously declared\n",
                XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER as i32,
                xkb_atom_text(&(*ctx).atom_table, name));
            return false;
        }
        *ndx_rtrn = ndx;
        true
    }
}
pub unsafe fn ExprResolveGroupMask(
    keymap_info: *mut xkb_keymap_info,
    expr: *const ExprDef,
    group_rtrn: *mut u32,
    pending_rtrn: *mut bool,
) -> bool {
    unsafe {
        static PENDING_GROUP_MASK_NAMES: [LookupEntry; 2] = [
            LookupEntry {
                name: GROUP_LAST_INDEX_NAME,
                value: 0_u32,
            },
            LookupEntry {
                name: "",
                value: 0_u32,
            },
        ];
        let group_name_pattern: named_integer_pattern = named_integer_pattern {
            prefix: "Group",
            min: 1_u32,
            max: (*keymap_info).features.max_groups,
            entries: &raw const (*keymap_info).lookup.groupMaskNames as *const LookupEntry,
            pending_entries: &PENDING_GROUP_MASK_NAMES as *const LookupEntry,
            is_mask: true,
            error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
        };
        ExprResolveMaskLookup(
            &raw mut (*(*keymap_info).keymap).ctx,
            expr,
            group_rtrn as *mut u32,
            pending_rtrn,
            Some(
                NamedIntegerPatternLookup
                    as unsafe fn(
                        *mut xkb_context,
                        *const ::core::ffi::c_void,
                        u32,
                        *mut u32,
                        *mut bool,
                    ) -> bool,
            ),
            &raw const group_name_pattern as *const ::core::ffi::c_void,
        )
    }
}
