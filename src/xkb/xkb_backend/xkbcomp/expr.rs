use super::super::shared_types::*;
use super::super::text::{
    actionTypeNames, buttonNames, ctrlMaskNames, LookupString, LookupValue, GROUP_LAST_INDEX_NAME,
};
use super::*;

pub use super::super::keymap::action_equal;
pub use super::super::keymap::XkbModNameToIndex;
pub use super::super::shared_ast_types::stmt_type_to_operator_char;
use super::super::shared_ast_types::ExprKind;

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
        let (val_parsed, c) = super::super::utils::parse_dec_u32(suffix);
        // Return parsed value via count mechanism
        let _ = val_parsed;
        c
    } else {
        0_i32
    };

    if count > 0_i32 && prefix.len() + count as usize == str_bytes.len() {
        // Re-parse to get the value
        let suffix = &str_bytes.as_bytes()[prefix.len()..];
        let (val, _) = super::super::utils::parse_dec_u32(suffix);
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
    expr: &'a ExprDef,
    elem_rtrn: &mut u32,
    field_rtrn: &mut u32,
    index_rtrn: &mut Option<&'a ExprDef>,
) -> bool {
    match expr.stmt_type() {
        10 => {
            let ExprKind::Ident(ident) = &expr.kind else {
                unreachable!()
            };
            *elem_rtrn = XKB_ATOM_NONE;
            *field_rtrn = *ident;
            *index_rtrn = None;
            return *field_rtrn != XKB_ATOM_NONE;
        }
        12 => {
            let ExprKind::FieldRef { element, field } = &expr.kind else {
                unreachable!()
            };
            *elem_rtrn = *element;
            *field_rtrn = *field;
            *index_rtrn = None;
            return *elem_rtrn != XKB_ATOM_NONE && *field_rtrn != XKB_ATOM_NONE;
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
            *elem_rtrn = *element;
            *field_rtrn = *field;
            *index_rtrn = entry.as_ref().map(|b| &**b);
            if *element != XKB_ATOM_NONE && *elem_rtrn == XKB_ATOM_NONE {
                return false;
            }
            if *field_rtrn == XKB_ATOM_NONE {
                return false;
            }
            return true;
        }
        _ => {}
    }
    log::error!(
        "[XKB-{:03}] Unexpected operator {} in ResolveLhs\n",
        XKB_ERROR_INVALID_XKB_SYNTAX as i32,
        { expr.stmt_type() }
    );
    false
}

pub fn ExprResolveBoolean(ctx: &xkb_context, expr: &ExprDef, set_rtrn: &mut bool) -> bool {
    let ok: bool;
    #[allow(unused_assignments)]
    let mut ident: &str = "";
    match expr.stmt_type() {
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
                stmt_type_to_string(expr.stmt_type())
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
                stmt_type_to_string(expr.stmt_type())
            );
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator {} in ResolveBoolean\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.stmt_type() }
            );
        }
    }
    false
}

fn ExprResolveIntegerLookup(
    ctx: &xkb_context,
    expr: &ExprDef,
    val_rtrn: &mut i64,
    pending: Option<&mut bool>,
    lookup: &IdentLookup,
) -> bool {
    let mut ok: bool = false;
    let mut l: i64 = 0_i64;
    let mut r: i64 = 0_i64;
    match expr.stmt_type() {
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
                stmt_type_to_string(expr.stmt_type())
            );
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr.kind else {
                unreachable!()
            };
            let mut pending_local = false;
            let pending_ref = if pending.is_some() {
                Some(&mut pending_local)
            } else {
                None
            };
            if let Some(u) = ident_lookup(ctx, lookup, *ident_atom, pending_ref) {
                *val_rtrn = u as i64;
                ok = true;
            }
            if !ok {
                log::error!(
                    "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    xkb_atom_text(&ctx.atom_table, *ident_atom)
                );
            }
            if let Some(p) = pending {
                *p = pending_local;
                if pending_local {
                    return false;
                }
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
            if !ExprResolveIntegerLookup(ctx, left, &mut l, None, lookup)
                || !ExprResolveIntegerLookup(ctx, right, &mut r, None, lookup)
            {
                return false;
            }
            match expr.stmt_type() {
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
                        stmt_type_to_string(expr.stmt_type())
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
            if !ExprResolveIntegerLookup(ctx, left, &mut l, None, lookup) {
                return false;
            }
            *val_rtrn = if expr.stmt_type() == STMT_EXPR_NEGATE {
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
            return ExprResolveIntegerLookup(ctx, left, val_rtrn, None, lookup);
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator {} in ResolveInteger\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.stmt_type() }
            );
        }
    }
    false
}

pub fn ExprResolveInteger(ctx: &xkb_context, expr: &ExprDef, val_rtrn: &mut i64) -> bool {
    ExprResolveIntegerLookup(ctx, expr, val_rtrn, None, &IdentLookup::None)
}

pub fn ExprResolveGroup(
    keymap_info: &mut xkb_keymap_info<'_>,
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
    if !ExprResolveIntegerLookup(ctx, expr, &mut result, Some(pending), &lookup) {
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
    if !ExprResolveIntegerLookup(ctx, expr, &mut result, None, &lookup) {
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
    ExprResolveIntegerLookup(ctx, expr, btn_rtrn, None, &lookup)
}

pub fn ExprResolveString(ctx: &xkb_context, expr: &ExprDef, val_rtrn: &mut u32) -> bool {
    match expr.stmt_type() {
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
                stmt_type_to_string(expr.stmt_type())
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
                stmt_type_to_string(expr.stmt_type())
            );
            return false;
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator {} in ResolveString\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.stmt_type() }
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
    if expr.stmt_type() != STMT_EXPR_IDENT {
        log::error!(
            "[XKB-{:03}] Found a {} where an enumerated value was expected\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(expr.stmt_type())
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
    pending: Option<&mut bool>,
    lookup: &IdentLookup,
) -> bool {
    let ok: bool;
    let mut l: u32 = 0_u32;
    let mut r: u32 = 0_u32;
    let mut v: i64 = 0_i64;
    let mut bogus: Option<&str> = None;
    let c2rust_current_block_47: u64;
    match expr.stmt_type() {
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
                stmt_type_to_string(expr.stmt_type())
            );
            return false;
        }
        10 => {
            let ExprKind::Ident(ident_atom) = &expr.kind else {
                unreachable!()
            };
            let mut pending_local = false;
            let pending_ref = if pending.is_some() {
                Some(&mut pending_local)
            } else {
                None
            };
            if let Some(val) = ident_lookup(ctx, lookup, *ident_atom, pending_ref) {
                *val_rtrn = val;
                ok = true;
            } else {
                ok = false;
            }
            if !ok {
                log::error!(
                    "[XKB-{:03}] Identifier \"{}\" of type int is unknown\n",
                    XKB_ERROR_INVALID_IDENTIFIER as i32,
                    xkb_atom_text(&ctx.atom_table, *ident_atom)
                );
            }
            if let Some(p) = pending {
                *p = pending_local;
                if pending_local {
                    return false;
                }
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
            if !ExprResolveMaskLookup(ctx, left, &mut l, None, lookup)
                || !ExprResolveMaskLookup(ctx, right, &mut r, None, lookup)
            {
                return false;
            }
            match expr.stmt_type() {
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
                        if expr.stmt_type() == STMT_EXPR_DIVIDE {
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
            if !ExprResolveIntegerLookup(ctx, left, &mut v, None, lookup) {
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
            if !ExprResolveIntegerLookup(ctx, left, &mut v, None, lookup) {
                return false;
            }
            log::error!(
                "[XKB-{:03}] The '{}' unary operator cannot be used with a mask\n",
                XKB_ERROR_INVALID_OPERATION as i32,
                (stmt_type_to_operator_char(expr.stmt_type()) as u8 as char)
            );
            return false;
        }
        _ => {
            log::error!(
                "[XKB-{:03}] Unknown operator type {} in ResolveMask\n",
                XKB_ERROR_UNKNOWN_OPERATOR as i32,
                { expr.stmt_type() }
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
    ExprResolveMaskLookup(ctx, expr, mask_rtrn, None, &lookup)
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
    ExprResolveMaskLookup(ctx, expr, mask_rtrn, None, &lookup)
}

pub fn ExprResolveMod(
    ctx: &xkb_context,
    def: &ExprDef,
    mod_type: u32,
    mods: &xkb_mod_set,
    ndx_rtrn: &mut u32,
) -> bool {
    if def.stmt_type() != STMT_EXPR_IDENT {
        log::error!(
            "[XKB-{:03}] Cannot resolve virtual modifier: found {} where a virtual modifier name was expected\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(def.stmt_type())
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
    keymap_info: &mut xkb_keymap_info<'_>,
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
    ExprResolveMaskLookup(ctx, expr, group_rtrn, Some(pending_rtrn), &lookup)
}
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
/// A value passed to an action handler.  Combines what used to be two separate
/// parameters (`value: &ExprDef` and `value_ptr: Option<&mut Option<Box<ExprDef>>>`).
pub enum ActionValue<'v> {
    /// A borrowed reference to a constant or non-ownable ExprDef (e.g. const_true).
    Borrowed(&'v ExprDef),
    /// A mutable reference to an owned ExprDef that can be `.take()`-en.
    Owned(&'v mut Option<Box<ExprDef>>),
}

impl<'v> ActionValue<'v> {
    /// Get a shared reference to the underlying ExprDef.
    #[inline]
    pub fn get(&self) -> &ExprDef {
        match self {
            ActionValue::Borrowed(e) => e,
            ActionValue::Owned(opt) => opt.as_deref().unwrap(),
        }
    }
    /// Take ownership of the ExprDef (only possible for Owned variant).
    #[inline]
    pub fn take(&mut self) -> Option<Box<ExprDef>> {
        match self {
            ActionValue::Borrowed(_) => None,
            ActionValue::Owned(opt) => opt.take(),
        }
    }
    /// Rebind to a child slot (for Owned variant navigating into Unary child).
    #[inline]
    pub fn rebind_to_child(self) -> ActionValue<'v> {
        match self {
            ActionValue::Owned(opt) => {
                if let ExprKind::Unary { ref mut child, .. } = opt.as_mut().unwrap().kind {
                    ActionValue::Owned(child)
                } else {
                    unreachable!()
                }
            }
            other => other,
        }
    }
}

pub type actionHandler = Option<
    for<'a> fn(
        &mut xkb_keymap_info<'a>,
        &xkb_mod_set,
        &mut xkb_action,
        u32,
        Option<&ExprDef>,
        ActionValue<'_>,
    ) -> u32,
>;
// Constant true/false ExprDef values used in HandleActionDef
fn const_true_expr() -> ExprDef {
    ExprDef {
        kind: ExprKind::Boolean(true),
    }
}
fn const_false_expr() -> ExprDef {
    ExprDef {
        kind: ExprKind::Boolean(false),
    }
}
pub fn InitActionsInfo(keymap: &xkb_keymap, info: &mut ActionsInfo) {
    let mut type_0: u32 = ACTION_TYPE_NONE;
    while type_0 < _ACTION_TYPE_NUM_ENTRIES {
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
    *type_rtrn = type_0;
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
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    _array_ndx: Option<&ExprDef>,
    _value: ActionValue<'_>,
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
    if value.stmt_type() == STMT_EXPR_IDENT {
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
    keymap_info: &mut xkb_keymap_info<'_>,
    mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
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
    if (type_0 == ACTION_TYPE_MOD_SET || type_0 == ACTION_TYPE_MOD_LATCH)
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
    if type_0 == ACTION_TYPE_MOD_LATCH {
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
    if type_0 == ACTION_TYPE_MOD_LOCK && field == ACTION_FIELD_AFFECT {
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
    keymap_info: &mut xkb_keymap_info<'_>,
    action: u32,
    array_ndx: Option<&ExprDef>,
    mut value: ActionValue<'_>,
    flags_inout: &mut xkb_action_flags,
    group_rtrn: &mut i32,
) -> u32 {
    let mut idx: u32 = 0_u32;
    let mut flags: xkb_action_flags = *flags_inout;
    if array_ndx.is_some() {
        return ReportActionNotArray(action, ACTION_FIELD_GROUP, keymap_info.strict);
    }
    // If the value is a unary negate/plus, rebind to child and record negate.
    let is_negate = value.get().stmt_type() == STMT_EXPR_NEGATE;
    let is_unary = is_negate || value.get().stmt_type() == STMT_EXPR_UNARY_PLUS;
    if is_unary {
        flags = (flags as u32 & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
        // Rebind value to the child field inside the unary expr
        // (for ownership transfer to pending_computations if needed)
        value = value.rebind_to_child();
    } else {
        flags = (flags as u32 | ACTION_ABSOLUTE_SWITCH) as xkb_action_flags;
    }
    let spec_holder = value.get();
    let absolute: bool = flags as u32 & ACTION_ABSOLUTE_SWITCH != 0;
    let mut pending: bool = false;
    let ret: u32 =
        ExprResolveGroup(keymap_info, spec_holder, absolute, &mut idx, &mut pending) as u32;
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
            expr: value.take(),
            computed: false,
            value: 0_u32,
        });
        *group_rtrn = pending_index as i32;
    } else {
        flags = (flags as u32 & !(ACTION_PENDING_COMPUTATION as i32) as u32) as xkb_action_flags;
        if flags as u32 & ACTION_ABSOLUTE_SWITCH == 0 {
            *group_rtrn = idx as i32;
            if is_negate {
                *group_rtrn = -*group_rtrn;
            }
        } else {
            *group_rtrn = idx.wrapping_sub(1_u32) as i32;
        }
    }
    *flags_inout = flags;
    PARSER_SUCCESS
}
fn HandleSetLatchLockGroup(
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
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
            &mut act.flags,
            &mut act.group,
        );
    }
    let value = value.get();
    let act = action.as_group_mut();
    if (type_0 == ACTION_TYPE_GROUP_SET || type_0 == ACTION_TYPE_GROUP_LATCH)
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
    if type_0 == ACTION_TYPE_GROUP_LATCH && field == ACTION_FIELD_LATCH_TO_LOCK {
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
    if type_0 == ACTION_TYPE_GROUP_LOCK && field == ACTION_FIELD_LOCK_ON_RELEASE {
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
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_ptr_mut();
    if field == ACTION_FIELD_X || field == ACTION_FIELD_Y {
        let mut val: i64 = 0_i64;
        let absolute: bool =
            value.stmt_type() != STMT_EXPR_NEGATE && value.stmt_type() != STMT_EXPR_UNARY_PLUS;
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
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
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
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
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
        if value.stmt_type() == STMT_EXPR_NEGATE || value.stmt_type() == STMT_EXPR_UNARY_PLUS {
            act.flags = (act.flags & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            button = if let ExprKind::Unary { child, .. } = &value.kind {
                child.as_deref().unwrap()
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
        act.value = (if value.stmt_type() == STMT_EXPR_NEGATE {
            -btn
        } else {
            btn
        }) as i8;
        return PARSER_SUCCESS;
    }
    ReportIllegal(type_0, field, keymap_info.strict)
}
fn HandleSwitchScreen(
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let ctx: &xkb_context = keymap_info.ctx();
    let type_0 = action.action_type();
    let act = action.as_screen_mut();
    if field == ACTION_FIELD_SCREEN {
        let scrn: &ExprDef;
        let mut val: i64 = 0_i64;
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if value.stmt_type() == STMT_EXPR_NEGATE || value.stmt_type() == STMT_EXPR_UNARY_PLUS {
            act.flags = (act.flags & !(ACTION_ABSOLUTE_SWITCH as i32) as u32) as xkb_action_flags;
            scrn = if let ExprKind::Unary { child, .. } = &value.kind {
                child.as_deref().unwrap()
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
        val = if value.stmt_type() == STMT_EXPR_NEGATE {
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
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
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
    keymap_info: &mut xkb_keymap_info<'_>,
    mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
    let type_0 = action.action_type();
    let act = action.as_redirect_mut();
    if field == ACTION_FIELD_KEYCODE {
        if array_ndx.is_some() {
            return ReportActionNotArray(type_0, field, keymap_info.strict);
        }
        if value.stmt_type() == STMT_EXPR_IDENT {
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
        if value.stmt_type() != STMT_EXPR_KEYNAME_LITERAL {
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
        let key = keymap_info.keymap_ref().key_by_name(key_name_val, true);
        if let Some(key) = key {
            act.keycode = key.keycode;
            return PARSER_SUCCESS;
        } else {
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
    _keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    _action: &mut xkb_action,
    _field: u32,
    _array_ndx: Option<&ExprDef>,
    _value: ActionValue<'_>,
) -> u32 {
    PARSER_SUCCESS
}
fn HandlePrivate(
    keymap_info: &mut xkb_keymap_info<'_>,
    _mods: &xkb_mod_set,
    action: &mut xkb_action,
    field: u32,
    array_ndx: Option<&ExprDef>,
    value: ActionValue<'_>,
) -> u32 {
    let value = value.get();
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
            act.data = [0u8; 7];
            act.data[..len].copy_from_slice(&str_bytes.as_bytes()[..len]);
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
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleNoAction
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockMods
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockMods
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockMods
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockGroup
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockGroup
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLatchLockGroup
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleMovePtr
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandlePtrBtn
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandlePtrBtn
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetPtrDflt
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleNoAction
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSwitchScreen
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLockControls
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleSetLockControls
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleRedirectKey
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleUnsupported
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandleUnsupported
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        Some(
            HandlePrivate
                as fn(
                    &mut xkb_keymap_info<'_>,
                    &xkb_mod_set,
                    &mut xkb_action,
                    u32,
                    Option<&ExprDef>,
                    ActionValue<'_>,
                ) -> u32,
        ),
        None,
    ]
};
pub fn HandleActionDef(
    keymap_info: &mut xkb_keymap_info<'_>,
    info: &mut ActionsInfo,
    mods: &xkb_mod_set,
    def: &mut ExprDef,
    action: &mut xkb_action,
) -> u32 {
    if def.stmt_type() != STMT_EXPR_ACTION_DECL {
        log::error!(
            "[XKB-{:03}] Expected an action definition, found {}\n",
            XKB_ERROR_WRONG_FIELD_TYPE as i32,
            stmt_type_to_string(def.stmt_type())
        );
        return PARSER_FATAL_ERROR;
    }
    // Extract action name atom (Copy type, no borrow held)
    let action_name_atom = if let ExprKind::Action { name, .. } = &def.kind {
        *name
    } else {
        unreachable!()
    };
    let action_name: &str = xkb_atom_text(&keymap_info.keymap.ctx.atom_table, action_name_atom);
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
    if handler_type == ACTION_TYPE_UNSUPPORTED_LEGACY {
        log::warn!(
            "[XKB-{:03}] Unsupported legacy action type \"{}\".\n",
            XKB_WARNING_UNSUPPORTED_LEGACY_ACTION as i32,
            action_name
        );
        action.set_none();
    }
    let mut ret: u32 = PARSER_SUCCESS;
    let const_true = const_true_expr();
    let const_false = const_false_expr();
    // Get mutable access to the args Vec
    let args = if let ExprKind::Action { ref mut args, .. } = def.kind {
        args
    } else {
        unreachable!()
    };
    for arg in args.iter_mut() {
        let av: ActionValue<'_>;
        let field_ref: &ExprDef;
        let mut arrayRtrn_opt: Option<&ExprDef> = None;
        let mut elemRtrn_atom: u32 = 0;
        let mut fieldRtrn_atom: u32 = 0;
        if arg.stmt_type() == STMT_EXPR_ASSIGN {
            if let ExprKind::Binary {
                ref left,
                ref mut right,
                ..
            } = arg.kind
            {
                field_ref = left.as_deref().unwrap();
                av = ActionValue::Owned(right);
            } else {
                unreachable!()
            }
        } else if arg.stmt_type() == STMT_EXPR_NOT || arg.stmt_type() == STMT_EXPR_INVERT {
            field_ref = if let ExprKind::Unary { ref child, .. } = arg.kind {
                child.as_deref().unwrap()
            } else {
                unreachable!()
            };
            av = ActionValue::Borrowed(&const_false);
        } else {
            field_ref = &*arg;
            av = ActionValue::Borrowed(&const_true);
        }
        if !ExprResolveLhs(
            field_ref,
            &mut elemRtrn_atom,
            &mut fieldRtrn_atom,
            &mut arrayRtrn_opt,
        ) {
            return PARSER_FATAL_ERROR;
        }
        let elemRtrn = xkb_atom_text(&keymap_info.keymap.ctx.atom_table, elemRtrn_atom);
        let fieldRtrn = xkb_atom_text(&keymap_info.keymap.ctx.atom_table, fieldRtrn_atom);
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
                av,
            ) {
                2 => return PARSER_FATAL_ERROR,
                1 => {
                    ret = PARSER_RECOVERABLE_ERROR;
                }
                _ => {}
            }
        }
    }
    if action.action_type() == ACTION_TYPE_UNKNOWN {
        PARSER_RECOVERABLE_ERROR
    } else {
        ret
    }
}
pub fn SetDefaultActionField(
    keymap_info: &mut xkb_keymap_info<'_>,
    info: &mut ActionsInfo,
    mods: &mut xkb_mod_set,
    elem: &str,
    field: &str,
    array_ndx: Option<&ExprDef>,
    value_rtrn: &mut Option<Box<ExprDef>>,
    merge: merge_mode,
) -> u32 {
    let av = ActionValue::Owned(value_rtrn);
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
        av,
    );
    if ret != PARSER_SUCCESS {
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
