use super::prelude::*;
use crate::xkb::shared_types::*;
use crate::xkb::text::{buttonNames, GROUP_LAST_INDEX_NAME};

pub use crate::xkb::keymap::XkbModNameToIndex;
pub use crate::xkb::shared_ast_types::stmt_type_to_operator_char;
use crate::xkb::shared_ast_types::{ExprKind, OptBoxRaw};

pub struct LookupModMaskPriv<'a> {
    pub mods: &'a xkb_mod_set,
    pub mod_type: u32,
}

/// Safe replacement for the IdentLookupFunc + *const c_void pair.
pub enum IdentLookup<'a> {
    None,
    Simple(&'a [LookupEntry]),
    NamedPattern(&'a NamedIntegerPattern<'a>),
    ModMask(&'a LookupModMaskPriv<'a>),
}

pub struct NamedIntegerPattern<'a> {
    pub prefix: &'static str,
    pub min: u32,
    pub max: u32,
    pub entries: &'a [LookupEntry],
    pub pending_entries: &'a [LookupEntry],
    pub is_mask: bool,
    pub error_id: u32,
}

static LEVEL_NAME_PATTERN_ENTRIES: [LookupEntry; 1] = [LookupEntry { name: "", value: 0 }];

fn simple_lookup(ctx: &xkb_context, entries: &[LookupEntry], field: u32) -> Option<u32> {
    if field == XKB_ATOM_NONE {
        return None;
    }
    let s: &str = xkb_atom_text(&ctx.atom_table, field);
    for entry in entries {
        if entry.name.is_empty() {
            break;
        }
        if s.eq_ignore_ascii_case(entry.name) {
            return Some(entry.value);
        }
    }
    None
}

fn named_integer_pattern_lookup(
    ctx: &xkb_context,
    pattern: &NamedIntegerPattern,
    field: u32,
    pending_rtrn: Option<&mut bool>,
) -> Option<u32> {
    if field == XKB_ATOM_NONE {
        return None;
    }
    let str_bytes: &str = xkb_atom_text(&ctx.atom_table, field);
    let prefix = pattern.prefix;
    let count: i32 = if str_bytes
        .as_bytes()
        .get(..prefix.len())
        .is_some_and(|s| s.eq_ignore_ascii_case(prefix.as_bytes()))
    {
        let suffix = &str_bytes.as_bytes()[prefix.len()..];
        let (val_parsed, c) = crate::xkb::utils::parse_dec_u32(suffix);
        // Return parsed value via count mechanism
        let _ = val_parsed;
        c
    } else {
        0_i32
    };

    if count > 0_i32 && prefix.len() + count as usize == str_bytes.len() {
        // Re-parse to get the value
        let suffix = &str_bytes.as_bytes()[prefix.len()..];
        let (val, _) = crate::xkb::utils::parse_dec_u32(suffix);
        if val < pattern.min || val > pattern.max {
            log::error!(
                "[XKB-{:03}] {} index {} is out of range ({}..{})\n",
                { pattern.error_id },
                pattern.prefix,
                val,
                pattern.min,
                pattern.max
            );
            return None;
        }
        let result = if pattern.is_mask {
            1_u32 << val.wrapping_sub(pattern.min)
        } else {
            val
        };
        Some(result)
    } else {
        if let Some(val) = simple_lookup(ctx, pattern.entries, field) {
            return Some(val);
        }
        if let Some(pending) = pending_rtrn {
            if let Some(val) = simple_lookup(ctx, pattern.pending_entries, field) {
                *pending = true;
                return Some(val);
            }
        }
        None
    }
}

fn lookup_mod_mask(ctx: &xkb_context, priv_0: &LookupModMaskPriv, field: u32) -> Option<u32> {
    let s: &str = xkb_atom_text(&ctx.atom_table, field);
    if s.is_empty() {
        return None;
    }
    if s.eq_ignore_ascii_case("all") {
        return Some(MOD_REAL_MASK_ALL);
    }
    if s.eq_ignore_ascii_case("none") {
        return Some(0_u32);
    }
    let ndx: u32 = XkbModNameToIndex(priv_0.mods, field, priv_0.mod_type);
    if ndx == XKB_MOD_INVALID {
        return None;
    }
    Some(1_u32 << ndx)
}

/// Dispatch a lookup based on the IdentLookup variant.
/// Returns Some(value) on success. Sets `pending` to true if applicable.
fn ident_lookup(
    ctx: &xkb_context,
    lookup: &IdentLookup,
    field: u32,
    pending: Option<&mut bool>,
) -> Option<u32> {
    match lookup {
        IdentLookup::None => None,
        IdentLookup::Simple(entries) => simple_lookup(ctx, entries, field),
        IdentLookup::NamedPattern(pattern) => {
            named_integer_pattern_lookup(ctx, pattern, field, pending)
        }
        IdentLookup::ModMask(priv_0) => lookup_mod_mask(ctx, priv_0, field),
    }
}

pub fn ExprResolveLhs<'a>(
    ctx: &'a xkb_context,
    expr: &'a ExprDef,
    elem_rtrn: &mut &'static str,
    field_rtrn: &mut &'static str,
    index_rtrn: &mut Option<&'a ExprDef>,
) -> bool {
    match expr.common.type_0 {
        10 => {
            let ExprKind::Ident(ident) = &expr.kind else {
                unreachable!()
            };
            *elem_rtrn = "";
            // SAFETY: atom table strings live for the lifetime of the context,
            // which outlives all callers. We extend to 'static to avoid borrow
            // conflicts at call sites where info.ctx is borrowed immutably here
            // but info needs to be borrowed mutably later.
            *field_rtrn = unsafe {
                std::mem::transmute::<&str, &'static str>(xkb_atom_text(&ctx.atom_table, *ident))
            };
            *index_rtrn = None;
            return !(*field_rtrn).is_empty();
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr.kind else {
                unreachable!()
            };
            *elem_rtrn = unsafe {
                std::mem::transmute::<&str, &'static str>(xkb_atom_text(&ctx.atom_table, *element))
            };
            *field_rtrn = unsafe {
                std::mem::transmute::<&str, &'static str>(xkb_atom_text(&ctx.atom_table, *field))
            };
            *index_rtrn = None;
            return !(*elem_rtrn).is_empty() && !(*field_rtrn).is_empty();
        }
        13 => {
            let ExprKind::ArrayRef {
                element,
                field,
                entry,
            } = &expr.kind
            else {
                unreachable!()
            };
            *elem_rtrn = unsafe {
                std::mem::transmute::<&str, &'static str>(xkb_atom_text(&ctx.atom_table, *element))
            };
            *field_rtrn = unsafe {
                std::mem::transmute::<&str, &'static str>(xkb_atom_text(&ctx.atom_table, *field))
            };
            *index_rtrn = entry.as_ref().map(|b| &**b);
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
    log::error!(
        "[XKB-{:03}] Unexpected operator {} in ResolveLhs\n",
        XKB_ERROR_INVALID_XKB_SYNTAX as i32,
        { expr.common.type_0 }
    );
    false
}

pub fn ExprResolveBoolean(ctx: &xkb_context, expr: &ExprDef, set_rtrn: &mut bool) -> bool {
    let ok: bool;
    #[allow(unused_assignments)]
    let mut ident: &str = "";
    match expr.common.type_0 {
        7 => {
            let ExprKind::Boolean(set) = &expr.kind else {
                unreachable!()
            };
            *set_rtrn = *set;
            return true;
        }
        4 | 5 | 6 | 8 | 9 => {
            log::error!(
                "[XKB-{:03}] Found {} where boolean was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string(expr.common.type_0)
            );
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr.kind else {
                unreachable!()
            };
            ident = xkb_atom_text(&ctx.atom_table, *ident_atom);
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
                } else if ident.eq_ignore_ascii_case("no") || ident.eq_ignore_ascii_case("off") {
                    *set_rtrn = false;
                    return true;
                }
            }
            log::error!(
                "[XKB-{:03}] Identifier \"{}\" of type boolean is unknown\n",
                XKB_ERROR_INVALID_IDENTIFIER as i32,
                ident
            );
            return false;
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr.kind else {
                unreachable!()
            };
            log::error!(
                "[XKB-{:03}] Default \"{}.{}\" of type boolean is unknown\n",
                XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                xkb_atom_text(&ctx.atom_table, *element),
                xkb_atom_text(&ctx.atom_table, *field)
            );
            return false;
        }
        24 | 22 => {
            let ExprKind::Unary { child, .. } = &expr.kind else {
                unreachable!()
            };
            let child_ref = child.as_deref().unwrap();
            ok = ExprResolveBoolean(ctx, child_ref, set_rtrn);
            if ok {
                *set_rtrn = !*set_rtrn;
            }
            return ok;
        }
        17 | 18 | 19 | 20 | 21 | 23 | 25 | 14 | 11 | 16 | 15 => {
            log::error!(
                "[XKB-{:03}] {} of boolean values not permitted\n",
                XKB_ERROR_INVALID_OPERATION as i32,
                stmt_type_to_string(expr.common.type_0)
            );
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator {} in ResolveBoolean\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.common.type_0 }
            );
        }
    }
    false
}

fn ExprResolveIntegerLookup(
    ctx: &xkb_context,
    expr: &ExprDef,
    val_rtrn: &mut i64,
    pending: *mut bool,
    lookup: &IdentLookup,
) -> bool {
    let mut ok: bool = false;
    let mut l: i64 = 0_i64;
    let mut r: i64 = 0_i64;
    match expr.common.type_0 {
        5 => {
            let ExprKind::Integer(ival) = &expr.kind else {
                unreachable!()
            };
            *val_rtrn = *ival;
            return true;
        }
        4 | 6 | 7 | 8 | 9 => {
            log::error!(
                "[XKB-{:03}] Found {} where an int was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string(expr.common.type_0)
            );
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr.kind else {
                unreachable!()
            };
            let mut pending_local = false;
            let pending_ref = if pending.is_null() {
                None
            } else {
                Some(&mut pending_local)
            };
            if let Some(u) = ident_lookup(ctx, lookup, *ident_atom, pending_ref) {
                *val_rtrn = u as i64;
                ok = true;
            }
            if !pending.is_null() {
                unsafe {
                    *pending = pending_local;
                }
            }
            if !ok {
                log::error!(
                    "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    xkb_atom_text(&ctx.atom_table, *ident_atom)
                );
            }
            if !pending.is_null() && unsafe { *pending } {
                return false;
            }
            return ok;
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr.kind else {
                unreachable!()
            };
            log::error!(
                "[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                xkb_atom_text(&ctx.atom_table, *element),
                xkb_atom_text(&ctx.atom_table, *field)
            );
            return false;
        }
        17..=20 => {
            let ExprKind::Binary {
                left: bleft,
                right: bright,
                ..
            } = &expr.kind
            else {
                unreachable!()
            };
            let left = bleft.as_deref().unwrap();
            let right = bright.as_deref().unwrap();
            if !ExprResolveIntegerLookup(ctx, left, &mut l, std::ptr::null_mut(), lookup)
                || !ExprResolveIntegerLookup(ctx, right, &mut r, std::ptr::null_mut(), lookup)
            {
                return false;
            }
            match expr.common.type_0 {
                17 => {
                    let (v, overflow) = l.overflowing_add(r);
                    *val_rtrn = v;
                    if overflow {
                        log::error!(
                            "[XKB-{:03}] Addition {} + {} has an invalid mathematical result: {}\n",
                            XKB_ERROR_INTEGER_OVERFLOW as i32,
                            l,
                            r,
                            *val_rtrn
                        );
                        return false;
                    }
                }
                18 => {
                    let (v, overflow) = l.overflowing_sub(r);
                    *val_rtrn = v;
                    if overflow {
                        log::error!("[XKB-{:03}] Substraction {} - {} has an invalid mathematical result: {}\n",
                            XKB_ERROR_INTEGER_OVERFLOW as i32, l, r, *val_rtrn);
                        return false;
                    }
                }
                19 => {
                    let (v, overflow) = l.overflowing_mul(r);
                    *val_rtrn = v;
                    if overflow {
                        log::error!("[XKB-{:03}] Multiplication {} * {} has an invalid mathematical result: {}\n",
                            XKB_ERROR_INTEGER_OVERFLOW as i32, l, r, *val_rtrn);
                        return false;
                    }
                }
                20 => {
                    if r == 0_i64 {
                        log::error!(
                            "[XKB-{:03}] Cannot divide by zero: {} / {}\n",
                            XKB_ERROR_INVALID_OPERATION as i32,
                            l,
                            r
                        );
                        return false;
                    }
                    *val_rtrn = l / r;
                }
                _ => {
                    log::error!(
                        "[XKB-{:03}] {} of integers not permitted\n",
                        XKB_ERROR_INVALID_OPERATION as i32,
                        stmt_type_to_string(expr.common.type_0)
                    );
                    return false;
                }
            }
            return true;
        }
        21 => {
            log::error!(
                "[XKB-{:03}] Assignment operator not implemented yet\n",
                XKB_ERROR_INVALID_OPERATION as i32
            );
        }
        22 => {
            log::error!(
                "[XKB-{:03}] The ! operator cannot be applied to an integer\n",
                XKB_ERROR_INVALID_OPERATION as i32
            );
            return false;
        }
        24 | 23 => {
            let ExprKind::Unary { child, .. } = &expr.kind else {
                unreachable!()
            };
            let left = child.as_deref().unwrap();
            if !ExprResolveIntegerLookup(ctx, left, &mut l, std::ptr::null_mut(), lookup) {
                return false;
            }
            *val_rtrn = if expr.common.type_0 == STMT_EXPR_NEGATE {
                -l
            } else {
                !l
            };
            return true;
        }
        25 => {
            let ExprKind::Unary { child, .. } = &expr.kind else {
                unreachable!()
            };
            let left = child.as_deref().unwrap();
            return ExprResolveIntegerLookup(ctx, left, val_rtrn, std::ptr::null_mut(), lookup);
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator {} in ResolveInteger\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.common.type_0 }
            );
        }
    }
    false
}

pub fn ExprResolveInteger(ctx: &xkb_context, expr: &ExprDef, val_rtrn: &mut i64) -> bool {
    ExprResolveIntegerLookup(
        ctx,
        expr,
        val_rtrn,
        std::ptr::null_mut(),
        &IdentLookup::None,
    )
}

pub fn ExprResolveGroup(
    keymap_info: &mut xkb_keymap_info,
    expr: &ExprDef,
    absolute: bool,
    group_rtrn: &mut u32,
    pending: &mut bool,
) -> u32 {
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
    let group_name_pattern = NamedIntegerPattern {
        prefix: "Group",
        min: 1_u32,
        max: keymap_info.features.max_groups,
        entries: &keymap_info.lookup.groupIndexNames,
        pending_entries: &PENDING_GROUP_INDEX_NAMES,
        is_mask: false,
        error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
    };
    let lookup = IdentLookup::NamedPattern(&group_name_pattern);
    let ctx = keymap_info.ctx();
    let mut result: i64 = 0_i64;
    if !ExprResolveIntegerLookup(ctx, expr, &mut result, pending as *mut bool, &lookup) {
        return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as u32;
    }
    if result < absolute as i64 || result > keymap_info.features.max_groups as i64 {
        log::error!(
            "[XKB-{:03}] Group index {} is out of range ({}..{})\n",
            { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
            result,
            absolute as i32,
            keymap_info.features.max_groups
        );
        return (if keymap_info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
            PARSER_FATAL_ERROR as i32
        } else {
            PARSER_RECOVERABLE_ERROR as i32
        }) as u32;
    }
    *group_rtrn = result as u32;
    PARSER_SUCCESS
}

pub fn ExprResolveLevel(ctx: &xkb_context, expr: &ExprDef, level_rtrn: &mut u32) -> bool {
    let pattern = NamedIntegerPattern {
        prefix: "Level",
        min: 1_u32,
        max: XKB_LEVEL_MAX_IMPL as u32,
        entries: &LEVEL_NAME_PATTERN_ENTRIES,
        pending_entries: &LEVEL_NAME_PATTERN_ENTRIES,
        is_mask: false,
        error_id: XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL,
    };
    let lookup = IdentLookup::NamedPattern(&pattern);
    let mut result: i64 = 0_i64;
    if !ExprResolveIntegerLookup(ctx, expr, &mut result, std::ptr::null_mut(), &lookup) {
        return false;
    }
    if result < 1_i64 || result > XKB_LEVEL_MAX_IMPL as i64 {
        log::error!(
            "[XKB-{:03}] Shift level {} is out of range (1..{})\n",
            XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL as i32,
            result,
            2048_i32
        );
        return false;
    }
    *level_rtrn = (result - 1_i64) as u32;
    true
}

pub fn ExprResolveButton(ctx: &xkb_context, expr: &ExprDef, btn_rtrn: &mut i64) -> bool {
    let lookup = IdentLookup::Simple(&buttonNames);
    ExprResolveIntegerLookup(ctx, expr, btn_rtrn, std::ptr::null_mut(), &lookup)
}

pub fn ExprResolveString(ctx: &xkb_context, expr: &ExprDef, val_rtrn: &mut u32) -> bool {
    match expr.common.type_0 {
        4 => {
            let ExprKind::String(s) = &expr.kind else {
                unreachable!()
            };
            *val_rtrn = *s;
            return true;
        }
        5..=9 => {
            log::error!(
                "[XKB-{:03}] Found {}, expected a string\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string(expr.common.type_0)
            );
            return false;
        }
        10 => {
            log::error!(
                "[XKB-{:03}] Identifier \"{}\" of type string not found\n",
                XKB_ERROR_INVALID_IDENTIFIER as i32,
                xkb_atom_text(&ctx.atom_table, {
                    let ExprKind::Ident(id) = &expr.kind else {
                        unreachable!()
                    };
                    *id
                })
            );
            return false;
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr.kind else {
                unreachable!()
            };
            log::error!(
                "[XKB-{:03}] Default \"{}.{}\" of type string not found\n",
                XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                xkb_atom_text(&ctx.atom_table, *element),
                xkb_atom_text(&ctx.atom_table, *field)
            );
            return false;
        }
        17 | 18 | 19 | 20 | 21 | 23 | 24 | 22 | 25 | 14 | 11 | 16 | 15 => {
            log::error!(
                "[XKB-{:03}] {} of strings not permitted\n",
                XKB_ERROR_INVALID_XKB_SYNTAX as i32,
                stmt_type_to_string(expr.common.type_0)
            );
            return false;
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator {} in ResolveString\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.common.type_0 }
            );
        }
    }
    false
}

pub fn ExprResolveEnum(
    ctx: &xkb_context,
    expr: &ExprDef,
    val_rtrn: &mut u32,
    values: &[LookupEntry],
) -> bool {
    if expr.common.type_0 != STMT_EXPR_IDENT {
        log::error!(
            "[XKB-{:03}] Found a {} where an enumerated value was expected\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(expr.common.type_0)
        );
        return false;
    }
    let ExprKind::Ident(ident_atom) = &expr.kind else {
        unreachable!()
    };
    if let Some(val) = simple_lookup(ctx, values, *ident_atom) {
        *val_rtrn = val;
        return true;
    }
    log::error!(
        "[XKB-{:03}] Illegal identifier {}; expected one of:\n",
        XKB_ERROR_INVALID_IDENTIFIER as i32,
        xkb_atom_text(&ctx.atom_table, *ident_atom)
    );
    for entry in values {
        if entry.name.is_empty() {
            break;
        }
        log::error!(
            "[XKB-{:03}] \t{}\n",
            XKB_ERROR_INVALID_IDENTIFIER as i32,
            entry.name
        );
    }
    false
}

fn ExprResolveMaskLookup(
    ctx: &xkb_context,
    expr: &ExprDef,
    val_rtrn: &mut u32,
    pending: *mut bool,
    lookup: &IdentLookup,
) -> bool {
    let ok: bool;
    let mut l: u32 = 0_u32;
    let mut r: u32 = 0_u32;
    let mut v: i64 = 0_i64;
    let mut bogus: Option<&str> = None;
    let c2rust_current_block_47: u64;
    match expr.common.type_0 {
        5 => {
            let ExprKind::Integer(ival) = &expr.kind else {
                unreachable!()
            };
            if *ival < 0_i64 || *ival > u32::MAX as i64 {
                log::error!(
                    "Mask {}{:#x} is out of range (0..{:#x})\n",
                    if *ival < 0_i64 { "-" } else { "" },
                    ival.abs(),
                    4294967295_u32
                );
                return false;
            }
            *val_rtrn = *ival as u32;
            return true;
        }
        4 | 6 | 7 | 8 | 9 => {
            log::error!(
                "[XKB-{:03}] Found {} where a mask was expected\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                stmt_type_to_string(expr.common.type_0)
            );
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr.kind else {
                unreachable!()
            };
            let mut pending_local = false;
            let pending_ref = if pending.is_null() {
                None
            } else {
                Some(&mut pending_local)
            };
            if let Some(val) = ident_lookup(ctx, lookup, *ident_atom, pending_ref) {
                *val_rtrn = val;
                ok = true;
            } else {
                ok = false;
            }
            if !pending.is_null() {
                unsafe {
                    *pending = pending_local;
                }
            }
            if !ok {
                log::error!(
                    "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    xkb_atom_text(&ctx.atom_table, *ident_atom)
                );
            }
            if !pending.is_null() && unsafe { *pending } {
                return false;
            }
            return ok;
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr.kind else {
                unreachable!()
            };
            log::error!(
                "[XKB-{:03}] Default \"{}.{}\" of type int is unknown\n",
                XKB_ERROR_INVALID_EXPRESSION_TYPE as i32,
                xkb_atom_text(&ctx.atom_table, *element),
                xkb_atom_text(&ctx.atom_table, *field)
            );
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
            } = &expr.kind
            else {
                unreachable!()
            };
            let left = bleft.as_deref().unwrap();
            let right = bright.as_deref().unwrap();
            if !ExprResolveMaskLookup(ctx, left, &mut l, std::ptr::null_mut(), lookup)
                || !ExprResolveMaskLookup(ctx, right, &mut r, std::ptr::null_mut(), lookup)
            {
                return false;
            }
            match expr.common.type_0 {
                17 => {
                    *val_rtrn = l | r;
                }
                18 => {
                    *val_rtrn = l & !r;
                }
                19 | 20 => {
                    log::error!(
                        "[XKB-{:03}] Cannot {} masks; Illegal operation ignored\n",
                        XKB_ERROR_INVALID_OPERATION as i32,
                        if expr.common.type_0 == STMT_EXPR_DIVIDE {
                            "divide"
                        } else {
                            "multiply"
                        }
                    );
                    return false;
                }
                _ => {}
            }
            return true;
        }
        21 => {
            log::error!(
                "[XKB-{:03}] Assignment operator not implemented yet\n",
                XKB_ERROR_INVALID_OPERATION as i32
            );
            c2rust_current_block_47 = 11626999923138678822;
        }
        24 => {
            let ExprKind::Unary { child, .. } = &expr.kind else {
                unreachable!()
            };
            let left = child.as_deref().unwrap();
            if !ExprResolveIntegerLookup(ctx, left, &mut v, std::ptr::null_mut(), lookup) {
                return false;
            }
            if v < 0_i64 || v > u32::MAX as i64 {
                log::error!(
                    "Mask {}{:#x} is out of range (0..{:#x})\n",
                    if v < 0_i64 { "-" } else { "" },
                    v.abs(),
                    4294967295_u32
                );
                return false;
            }
            *val_rtrn = !(v as u32);
            return true;
        }
        25 | 23 | 22 => {
            let ExprKind::Unary { child, .. } = &expr.kind else {
                unreachable!()
            };
            let left = child.as_deref().unwrap();
            if !ExprResolveIntegerLookup(ctx, left, &mut v, std::ptr::null_mut(), lookup) {
                return false;
            }
            log::error!(
                "[XKB-{:03}] The '{}' unary operator cannot be used with a mask\n",
                XKB_ERROR_INVALID_OPERATION as i32,
                (stmt_type_to_operator_char(expr.common.type_0) as u8 as char)
            );
            return false;
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator type {} in ResolveMask\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.common.type_0 }
            );
            c2rust_current_block_47 = 11626999923138678822;
        }
    }
    match c2rust_current_block_47 {
        11626999923138678822 => {}
        _ => {
            if bogus.is_none() {
                bogus = Some("function use");
            }
            log::error!(
                "[XKB-{:03}] Unexpected {} in mask expression; Expression Ignored\n",
                XKB_ERROR_WRONG_FIELD_TYPE as i32,
                bogus.unwrap_or("unknown")
            );
            return false;
        }
    }
    false
}

pub fn ExprResolveMask(
    ctx: &xkb_context,
    expr: &ExprDef,
    mask_rtrn: &mut u32,
    values: &[LookupEntry],
) -> bool {
    let lookup = IdentLookup::Simple(values);
    ExprResolveMaskLookup(ctx, expr, mask_rtrn, std::ptr::null_mut(), &lookup)
}

pub fn ExprResolveModMask(
    ctx: &xkb_context,
    expr: &ExprDef,
    mod_type: u32,
    mods: &xkb_mod_set,
    mask_rtrn: &mut u32,
) -> bool {
    let priv_0 = LookupModMaskPriv { mods, mod_type };
    let lookup = IdentLookup::ModMask(&priv_0);
    ExprResolveMaskLookup(ctx, expr, mask_rtrn, std::ptr::null_mut(), &lookup)
}

pub fn ExprResolveMod(
    ctx: &xkb_context,
    def: &ExprDef,
    mod_type: u32,
    mods: &xkb_mod_set,
    ndx_rtrn: &mut u32,
) -> bool {
    if def.common.type_0 != STMT_EXPR_IDENT {
        log::error!(
            "[XKB-{:03}] Cannot resolve virtual modifier: found {} where a virtual modifier name was expected\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(def.common.type_0)
        );
        return false;
    }
    let ExprKind::Ident(ident_atom) = &def.kind else {
        unreachable!()
    };
    let name: u32 = *ident_atom;
    let ndx: u32 = XkbModNameToIndex(mods, name, mod_type);
    if ndx == XKB_MOD_INVALID {
        log::error!(
            "[XKB-{:03}] Cannot resolve virtual modifier: \"{}\" was not previously declared\n",
            XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER as i32,
            xkb_atom_text(&ctx.atom_table, name)
        );
        return false;
    }
    *ndx_rtrn = ndx;
    true
}

pub fn ExprResolveGroupMask(
    keymap_info: &mut xkb_keymap_info,
    expr: &ExprDef,
    group_rtrn: &mut u32,
    pending_rtrn: &mut bool,
) -> bool {
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
    let group_name_pattern = NamedIntegerPattern {
        prefix: "Group",
        min: 1_u32,
        max: keymap_info.features.max_groups,
        entries: &keymap_info.lookup.groupMaskNames,
        pending_entries: &PENDING_GROUP_MASK_NAMES,
        is_mask: true,
        error_id: XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_,
    };
    let lookup = IdentLookup::NamedPattern(&group_name_pattern);
    let ctx = keymap_info.ctx();
    ExprResolveMaskLookup(ctx, expr, group_rtrn, pending_rtrn as *mut bool, &lookup)
}
