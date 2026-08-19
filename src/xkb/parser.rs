use super::keymap::mod_mask_get_effective;
use super::keymap::xkb_escape_map_name;
use super::keymap::GROUP_LAST_INDEX_NAME;
use super::keysym::xkb_keysym_from_name;
use super::parser_tables::*;
pub(crate) use super::symbols::compile_key_types;
pub(crate) use super::symbols::compile_keycodes;
pub(crate) use super::symbols::compile_symbols;
use super::symbols::expr_resolve_group;
use crate::xkb::keymap::xkb_mod_name_to_index;
use crate::xkb::keysym::codepoint_to_keysym;

pub(crate) const XKB_KEY_VOID_SYMBOL: i32 = 0xffffff_i32;
pub(crate) const XKB_KEY_0: i32 = 0x30;
pub(crate) const XKB_KEY_SECTION: i32 = 0xa7_i32;
pub(crate) const XKB_KEYSYM_MIN: i32 = 0;

pub(crate) const YYINITDEPTH: usize = 200;
pub(crate) const YYMAXDEPTH: usize = 10000;

pub(crate) struct ParserParam<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    pub(crate) scanner: &'a mut Scanner<'a>,
    pub(crate) rtrn: Option<Box<XkbFile>>,
    pub(crate) more_maps: bool,
}

fn resolve_keysym(name: Sval) -> Option<u32> {
    let name_bytes = name.data;
    let name_str = std::str::from_utf8(name.data).unwrap_or("");

    if name_str.eq_ignore_ascii_case("any") || name_str.eq_ignore_ascii_case("nosymbol") {
        return Some(XKB_KEY_NO_SYMBOL);
    }
    if name_str.eq_ignore_ascii_case("none") || name_str.eq_ignore_ascii_case("voidsymbol") {
        return Some(XKB_KEY_VOID_SYMBOL as u32);
    }

    if name.data.len() >= 30 {
        return None;
    }

    // Build null-terminated buffer for xkb_keysym_from_name
    let mut buf = [0u8; 32];
    buf[..name.data.len()].copy_from_slice(name_bytes);
    buf[name.data.len()] = 0;
    let buf_slice = &buf[..name.data.len() + 1];

    xkb_keysym_from_name(buf_slice, XKB_KEYSYM_NO_FLAGS)
}

pub(crate) fn _xkbcommon_parse<'a>(param: &mut ParserParam<'a>) -> i32 {
    let mut lookahead = YYEMPTY;
    let mut lookahead_value = YYValue::None;

    let mut states = Vec::with_capacity(YYINITDEPTH);
    let mut values = Vec::with_capacity(YYINITDEPTH);

    states.push(0);
    values.push(YYValue::None);

    loop {
        if states.len() >= YYMAXDEPTH {
            return 2;
        }

        let state = &STATES[*states.last().unwrap() as usize];

        if lookahead == YYEMPTY
            && (state.has_terminal_transitions() || matches!(state.default_action(), Action::Error))
        {
            lookahead = _xkbcommon_lex(&mut lookahead_value, param.scanner, param.ctx);
        }

        let action = (lookahead >= 0)
            .then(|| state.explicit_action(lookahead as Symbol))
            .flatten()
            .unwrap_or_else(|| state.default_action());

        match action {
            Action::Accept => return 0,

            Action::Error => return 1,

            Action::Shift(next) => {
                states.push(next);
                values.push(std::mem::replace(&mut lookahead_value, YYValue::None));
                lookahead = YYEMPTY;
            }

            Action::Reduce(rule_id) => {
                let rule = &RULES[rule_id as usize];
                let rhs_len = rule.rhs_len() as usize;
                let top = values.len() - 1;
                let mut result = YYValue::None;

                if !execute_reduction(rule_id as i32, &mut values, top, &mut result, param) {
                    return 1;
                }

                // A complete top-level map is returned before parsing
                // another map from the same component file.
                if matches!(rule_id, 2 | 3) {
                    return 0;
                }

                states.truncate(states.len() - rhs_len);
                values.truncate(values.len() - rhs_len);

                let previous = *states.last().unwrap();
                states.push(rule.next_state(previous));
                values.push(result);
            }
        }
    }
}
#[inline(always)]
fn yy_atom<'a>(yyval: &mut YYValue<'a>, ctx: &mut &mut XkbContext, bytes: &[u8]) {
    *yyval = YYValue::Atom(ctx.atom_intern(bytes));
}
#[inline(always)]
fn yy_bin_expr<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize, op: BinaryOp) {
    let left = yyvs[sp - 2].take_expr();
    let right = yyvs[sp].take_expr();
    *yyval = YYValue::Expr(ExprKind::Binary {
        op,
        left: left.map(Box::new),
        right: right.map(Box::new),
    });
}
#[inline(always)]
fn yy_unary_expr<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize, op: UnaryOp) {
    let child = yyvs[sp].take_expr();
    *yyval = YYValue::Expr(ExprKind::Unary {
        op,
        child: child.map(Box::new),
    });
}
#[inline(always)]
fn yy_list_push<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize, sp_off: usize) {
    let item = yyvs[sp].take_expr();
    let mut list = yyvs[sp - sp_off].take_expr_list();
    if let Some(e) = item {
        list.push(e);
    }
    *yyval = YYValue::ExprList(list);
}
#[inline(always)]
fn yy_list_single<'a>(yyval: &mut YYValue<'a>, yyvs: &mut [YYValue<'a>], sp: usize) {
    let item = yyvs[sp].take_expr();
    let mut list = Vec::new();
    if let Some(e) = item {
        list.push(e);
    }
    *yyval = YYValue::ExprList(list);
}

macro_rules! yy_merge_decl {
    ($yyval:expr, $yyvs:expr, $sp:expr, $variant:ident, $stmt_variant:ident) => {
        let merge_mode = $yyvs[$sp - 1].as_merge();
        if let YYValue::$variant(mut item) = std::mem::replace(&mut $yyvs[$sp], YYValue::None) {
            item.merge = merge_mode;
            *$yyval = YYValue::Stmt(Statement::$stmt_variant(item));
        } else {
            *$yyval = YYValue::None;
        }
    };
}

fn execute_reduction<'a>(
    yyn: i32,
    yyvs: &mut [YYValue<'a>],
    sp: usize,
    yyval: &mut YYValue<'a>,
    param: &mut ParserParam<'a>,
) -> bool {
    match yyn {
        2 | 3 => {
            param.rtrn = yyvs[sp].take_file();
            param.more_maps = param.rtrn.is_some();
        }
        4 => {
            param.rtrn = None;
            *yyval = YYValue::None;
            param.more_maps = false;
        }
        5 => {
            let file_type = yyvs[sp - 5].as_file_type();
            let name = yyvs[sp - 4].take_str();
            let files = yyvs[sp - 2].take_file_list();
            let flags = yyvs[sp - 6].as_map_flags();
            let defs: Vec<Statement> = files.into_iter().map(Statement::XkbFile).collect();
            *yyval = YYValue::File(xkb_file_create(
                file_type,
                if name.is_empty() { None } else { Some(name) },
                defs,
                flags,
            ));
        }
        6..=8 => {
            *yyval = YYValue::FileType(FileType::Keymap);
        }
        9 => {
            let file = yyvs[sp].take_file();
            let mut list = yyvs[sp - 1].take_file_list();
            if let Some(f) = file {
                list.push(*f);
            }
            *yyval = YYValue::FileList(list);
        }
        10 => {
            *yyval = YYValue::FileList(Vec::new());
        }
        11 => {
            let file_type = yyvs[sp - 5].as_file_type();
            let name = yyvs[sp - 4].take_str();
            let stmts = yyvs[sp - 2].take_stmt_list();
            let flags = yyvs[sp - 6].as_map_flags();
            *yyval = YYValue::File(xkb_file_create(
                file_type,
                if name.is_empty() { None } else { Some(name) },
                stmts,
                flags,
            ));
        }
        12..=16 => {
            *yyval = YYValue::FileType(
                [
                    FileType::Keycodes,
                    FileType::Types,
                    FileType::Compat,
                    FileType::Symbols,
                    FileType::Geometry,
                ][yyn as usize - 12],
            )
        }
        17 | 20 => {
            *yyval = YYValue::MapFlags(yyvs[sp].as_map_flags());
        }
        18 => {
            *yyval = YYValue::MapFlags(0);
        }
        19 => {
            let f = yyvs[sp - 1].as_map_flags() | yyvs[sp].as_map_flags();
            *yyval = YYValue::MapFlags(f);
        }
        21..=28 => {
            *yyval = YYValue::MapFlags(if yyn == 22 {
                MAP_IS_DEFAULT
            } else {
                MAP_HAS_MAP_FLAGS
            });
        }
        29 => {
            let stmt = std::mem::replace(&mut yyvs[sp], YYValue::None);
            let mut list = yyvs[sp - 1].take_stmt_list();
            if let YYValue::Stmt(s) = stmt {
                list.push(s);
            }
            *yyval = YYValue::StmtList(list);
        }
        30 => {
            let merge = yyvs[sp - 1].as_merge();
            let mut vmods = yyvs[sp].take_vmod_list();
            for v in &mut vmods {
                v.merge = merge;
            }
            let mut list = yyvs[sp - 2].take_stmt_list();
            for v in vmods {
                list.push(Statement::VMod(v));
            }
            *yyval = YYValue::StmtList(list);
        }
        31 => {
            *yyval = YYValue::StmtList(Vec::new());
        }
        32 => {
            let merge = yyvs[sp - 1].as_merge();
            if let Some(mut var) = yyvs[sp].take_var() {
                var.merge = merge;
                *yyval = YYValue::Stmt(Statement::Var(var));
            } else {
                *yyval = YYValue::None;
            }
        }
        33 => *yyval = YYValue::None,
        34 => {
            yy_merge_decl!(yyval, yyvs, sp, Keycode, Keycode);
        }
        35 => {
            yy_merge_decl!(yyval, yyvs, sp, KeyAlias, KeyAlias);
        }
        36 => {
            yy_merge_decl!(yyval, yyvs, sp, KeyType, KeyType);
        }
        37 => {
            yy_merge_decl!(yyval, yyvs, sp, Symbols, Symbols);
        }
        38 => {
            yy_merge_decl!(yyval, yyvs, sp, ModMask, ModMap);
        }
        40..=44 | 93..=123 | 181 => *yyval = YYValue::None,
        39 | 45 | 46 => {
            if let YYValue::Unknown = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::Unknown);
            } else {
                *yyval = YYValue::None;
            }
        }
        47 => {
            let merge = yyvs[sp - 1].as_merge();
            let s = yyvs[sp].take_str();
            if let Some(inc) = include_create(&s, merge) {
                *yyval = YYValue::Stmt(Statement::Include(inc));
            } else {
                *yyval = YYValue::None;
            }
        }
        48 | 69 | 70 => {
            let trailing = usize::from(yyn == 48);
            let lhs = yyvs[sp - 2 - trailing].take_expr();
            let val = yyvs[sp - trailing].take_expr();
            *yyval = YYValue::Var(var_create(lhs, val));
        }
        49 | 71 => {
            let atom = yyvs[sp - usize::from(yyn == 49)].as_atom();
            *yyval = YYValue::Var(bool_var_create(atom, true));
        }
        50 | 72 => {
            let atom = yyvs[sp - usize::from(yyn == 50)].as_atom();
            *yyval = YYValue::Var(bool_var_create(atom, false));
        }
        51 => {
            let atom = yyvs[sp - 3].as_atom();
            let num = yyvs[sp - 1].as_num();
            *yyval = YYValue::Keycode(keycode_create(atom, num));
        }
        52 => {
            let alias = yyvs[sp - 3].as_atom();
            let real = yyvs[sp - 1].as_atom();
            *yyval = YYValue::KeyAlias(key_alias_create(alias, real));
        }
        53 => {
            let list = yyvs[sp - 1].take_vmod_list();
            *yyval = YYValue::VModList(list);
        }
        54 | 55 => {
            let vmod = yyvs[sp].take_vmod();
            let mut list = if yyn == 54 {
                yyvs[sp - 2].take_vmod_list()
            } else {
                Vec::new()
            };
            if let Some(v) = vmod {
                list.push(v);
            }
            *yyval = YYValue::VModList(list);
        }
        56 => {
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::VMod(vmod_create(atom, None));
        }
        57 => {
            let atom = yyvs[sp - 2].as_atom();
            let expr = yyvs[sp].take_expr();
            *yyval = YYValue::VMod(vmod_create(atom, expr));
        }
        58 => {
            if let YYValue::Interp(mut interp) = std::mem::replace(&mut yyvs[sp - 4], YYValue::None)
            {
                let vardefs = yyvs[sp - 2].take_var_list();
                interp.def = vardefs;
                *yyval = YYValue::Interp(interp);
            } else {
                *yyval = YYValue::None;
            }
        }
        59 | 60 => {
            *yyval = YYValue::Interp(InterpDef { def: Vec::new() });
        }
        61 | 67 | 68 => {
            let var = yyvs[sp].take_var();
            let mut list = match yyn {
                61 => yyvs[sp - 1].take_var_list(),
                67 => yyvs[sp - 2].take_var_list(),
                _ => Vec::new(),
            };
            if let Some(v) = var {
                list.push(v);
            }
            *yyval = YYValue::VarList(list);
        }
        62 | 66 => {
            *yyval = YYValue::VarList(Vec::new());
        }
        63 => {
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::KeyType(key_type_create(atom, vardefs));
        }
        64 => {
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Symbols(symbols_create(atom, vardefs));
        }
        65 => {
            let list = yyvs[sp].take_var_list();
            *yyval = YYValue::VarList(list);
        }
        73 => {
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(var_create(None, val));
        }
        74 | 76 | 172 => {
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(ExprKind::ActionList { actions: list });
        }
        75 => {
            let mut list = yyvs[sp - 1].take_expr_list(); // sp-1 = MultiKeySymList = offset(-1)
            let count = yyvs[sp - 3].as_no_sym_or_action_list(); // sp-3 = NoSymbolOrActionList = offset(-3)
            let mut prepended: Vec<ExprKind> = Vec::new();
            for _ in 0..count {
                prepended.push(expr_create_key_sym_list(XKB_KEY_NO_SYMBOL));
            }
            prepended.append(&mut list);
            *yyval = YYValue::Expr(ExprKind::ActionList { actions: prepended });
        }
        77 => {
            let mut list = yyvs[sp - 1].take_expr_list();
            let count = yyvs[sp - 3].as_no_sym_or_action_list();
            let mut prepended: Vec<ExprKind> = Vec::new();
            for _ in 0..count {
                prepended.push(ExprKind::ActionList {
                    actions: Vec::new(),
                });
            }
            prepended.append(&mut list);
            *yyval = YYValue::Expr(ExprKind::ActionList { actions: prepended });
        }
        78 => {
            *yyval = YYValue::Expr(ExprKind::EmptyList);
        }
        79 => {
            let prev = yyvs[sp - 3].as_no_sym_or_action_list();
            *yyval = YYValue::NoSymbolOrActionList(prev + 1);
        }
        80 => {
            *yyval = YYValue::NoSymbolOrActionList(1);
        }
        81 => {
            *yyval = YYValue::NoSymbolOrActionList(0);
        }
        82 => {
            *yyval = YYValue::Unknown;
        }
        83 => {
            let atom = yyvs[sp - 4].as_atom();
            let list = yyvs[sp - 2].take_expr_list();
            *yyval = YYValue::ModMask(mod_map_create(atom, list));
        }
        84 | 148 | 170 | 187 => {
            yy_list_push(yyval, yyvs, sp, 2);
        }
        85 | 149 | 169 | 171 | 189 => {
            yy_list_single(yyval, yyvs, sp);
        }
        86 | 185 => {
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::KeyName(atom));
        }
        87 => {
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(ExprKind::KeySym(keysym));
        }
        88 => {
            *yyval = YYValue::LedMap(());
        }
        89 | 90 => {
            *yyval = YYValue::LedName(());
        }
        91 => {
            let _ = yyvs[sp - 3].take_expr();
            let _ = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Unknown;
        }
        92 => {
            let _ = yyvs[sp - 4].take_expr();
            let _ = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Unknown;
        }
        124..=127 | 209 => *yyval = YYValue::Num(0),
        128 | 129 => {
            *yyval = YYValue::Atom(yyvs[sp].as_atom());
        }
        130..=140 => yy_atom(
            yyval,
            &mut param.ctx,
            [
                &b"action"[..],
                b"interpret",
                b"type",
                b"key",
                b"group",
                b"modifier_map",
                b"indicator",
                b"shape",
                b"row",
                b"section",
                b"text",
            ][yyn as usize - 130],
        ),
        141 => {
            *yyval = YYValue::Merge(yyvs[sp].as_merge());
        }
        142..=147 => {
            *yyval = YYValue::Merge(match yyn {
                144 => MergeMode::Augment,
                145 => MergeMode::Override,
                146 => MergeMode::Replace,
                _ => MergeMode::Default,
            })
        }
        150 => {
            *yyval = YYValue::ExprList(Vec::new());
        }
        151..=155 => yy_bin_expr(
            yyval,
            yyvs,
            sp,
            [
                BinaryOp::Divide,
                BinaryOp::Add,
                BinaryOp::Subtract,
                BinaryOp::Multiply,
                BinaryOp::Assign,
            ][yyn as usize - 151],
        ),
        156 | 161 | 163 | 164 | 173 | 180 | 196 => {
            *yyval = std::mem::replace(&mut yyvs[sp], YYValue::None);
        }
        157..=160 => yy_unary_expr(
            yyval,
            yyvs,
            sp,
            [
                UnaryOp::Negate,
                UnaryOp::Plus,
                UnaryOp::Not,
                UnaryOp::Invert,
            ][yyn as usize - 157],
        ),
        162 | 175 => {
            let name = yyvs[sp - 3].as_atom();
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(ExprKind::Action { name, args: list });
        }
        165 | 194 => {
            *yyval = std::mem::replace(&mut yyvs[sp - 1], YYValue::None);
        }
        166 | 167 => {
            let item = if yyn == 166 {
                Some(ExprKind::ActionList {
                    actions: yyvs[sp].take_expr_list(),
                })
            } else {
                yyvs[sp].take_expr()
            };
            let mut list = yyvs[sp - 2].take_expr_list();
            if let Some(e) = item {
                list.push(e);
            }
            *yyval = YYValue::ExprList(list);
        }
        168 => {
            let actions_expr_list = yyvs[sp].take_expr_list();
            let action_list_expr = ExprKind::ActionList {
                actions: actions_expr_list,
            };
            *yyval = YYValue::ExprList(vec![action_list_expr]);
        }
        174 => {
            *yyval = YYValue::Expr(ExprKind::ActionList {
                actions: Vec::new(),
            });
        }
        176 => {
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::Ident(atom));
        }
        177 => {
            let element = yyvs[sp - 2].as_atom();
            let field = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::FieldRef {
                element,
                field,
                index: None,
            });
        }
        178 => {
            let field = yyvs[sp - 3].as_atom();
            let entry = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Expr(ExprKind::FieldRef {
                element: XKB_ATOM_NONE,
                field,
                index: entry.map(Box::new),
            });
        }
        179 => {
            let element = yyvs[sp - 5].as_atom();
            let field = yyvs[sp - 3].as_atom();
            let entry = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Expr(ExprKind::FieldRef {
                element,
                field,
                index: entry.map(Box::new),
            });
        }
        182 => {
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::String(atom));
        }
        183 => {
            let num = yyvs[sp].as_num();
            *yyval = YYValue::Expr(ExprKind::Integer(num));
        }
        184 => {
            *yyval = YYValue::Expr(ExprKind::Float);
        }
        186 => {
            let keysym = yyvs[sp].as_keysym();
            let expr = expr_create_key_sym_list(keysym);
            let mut list = yyvs[sp - 2].take_expr_list();
            list.push(expr);
            *yyval = YYValue::ExprList(list);
        }
        188 => {
            let keysym = yyvs[sp].as_keysym();
            let expr = expr_create_key_sym_list(keysym);
            *yyval = YYValue::ExprList(vec![expr]);
        }
        190 => {
            let mut expr = yyvs[sp - 2].take_expr().unwrap();
            let keysym = yyvs[sp].as_keysym();
            if keysym != XKB_KEY_NO_SYMBOL {
                if let ExprKind::KeysymList { ref mut syms } = expr {
                    syms.push(keysym);
                }
            }
            *yyval = YYValue::Expr(expr);
        }
        192 => {
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(expr_create_key_sym_list(keysym));
        }
        191 | 193 | 195 => {
            let s = yyvs[sp].take_str();
            let expr = if yyn == 191 {
                yyvs[sp - 2].take_expr().unwrap()
            } else {
                expr_create_key_sym_list(XKB_KEY_NO_SYMBOL)
            };
            let Some(expr) = expr_key_sym_list_append_string(expr, &s) else {
                return false;
            };
            *yyval = YYValue::Expr(expr);
        }
        197 => {
            *yyval = YYValue::Expr(expr_create_key_sym_list(XKB_KEY_NO_SYMBOL));
        }
        198 => {
            *yyval = YYValue::Keysym(yyvs[sp].as_keysym());
        }
        199 => {
            let s = yyvs[sp].take_str();
            match keysym_parse_string(&s) {
                Some(keysym) => *yyval = YYValue::Keysym(keysym),
                None => return false,
            }
        }
        200 => {
            *yyval =
                YYValue::Keysym(resolve_keysym(yyvs[sp].as_sval()).unwrap_or(XKB_KEY_NO_SYMBOL))
        }
        201 => {
            *yyval = YYValue::Keysym(XKB_KEY_SECTION as u32);
        }
        202 => {
            let num = yyvs[sp].as_num();
            *yyval = YYValue::Keysym((XKB_KEY_0 as u32).wrapping_add(num as u32));
        }
        203 => {
            let num = yyvs[sp].as_num();
            *yyval = YYValue::Keysym(
                if (XKB_KEYSYM_MIN as i64..=XKB_KEYSYM_MAX as i64).contains(&num) {
                    num as u32
                } else {
                    XKB_KEY_NO_SYMBOL
                },
            );
        }
        204 => {
            *yyval = YYValue::Num(-yyvs[sp].as_num());
        }
        205..=208 | 210..=213 => {
            *yyval = YYValue::Num(yyvs[sp].as_num());
        }
        214 => {
            let sval = yyvs[sp].as_sval();
            *yyval = YYValue::Atom(param.ctx.atom_intern(sval.data));
        }
        215 => {
            *yyval = YYValue::Atom(param.ctx.atom_intern(b"default"));
        }
        216 => {
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Atom(param.ctx.atom_intern(s.as_bytes()));
        }
        217 | 219 => {
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Str(s);
        }
        218 => {
            *yyval = YYValue::Str(String::new());
        }

        _ => {}
    }
    true
}

pub(crate) fn parse<'a>(
    mut ctx: &'a mut XkbContext,
    mut scanner: &'a mut Scanner<'a>,
    map: &str,
) -> Option<Box<XkbFile>> {
    let mut first: Option<Box<XkbFile>> = None;

    loop {
        let mut param = ParserParam {
            ctx,
            scanner,
            rtrn: None,
            more_maps: false,
        };

        let ret = _xkbcommon_parse(&mut param);
        // Recover ctx and scanner from param before it's dropped
        ctx = param.ctx;
        scanner = param.scanner;

        if ret != 0 {
            return None;
        }
        if !param.more_maps {
            return param.rtrn.or(first);
        }
        let file = param.rtrn?;
        if (!map.is_empty() && map == file.name)
            || (map.is_empty() && file.flags & MAP_IS_DEFAULT != 0)
        {
            return Some(file);
        }
        if map.is_empty() && first.is_none() {
            first = Some(file);
        }
    }
}

// ── AST builder functions (merged from ast_build.rs) ──

pub(crate) fn expr_create_key_sym_list(sym: u32) -> ExprKind {
    let mut syms = Vec::new();
    if sym != XKB_KEY_NO_SYMBOL {
        syms.push(sym);
    }
    ExprKind::KeysymList { syms }
}

pub(crate) fn expr_key_sym_list_append_string(
    mut expr: ExprKind,
    string: &str,
) -> Option<ExprKind> {
    for ch in string.chars() {
        let sym = codepoint_to_keysym(ch as u32).unwrap_or(0);
        if sym == XKB_KEY_NO_SYMBOL {
            return None;
        }
        if let ExprKind::KeysymList { ref mut syms } = expr {
            syms.push(sym);
        }
    }
    Some(expr)
}

pub(crate) fn keysym_parse_string(string: &str) -> Option<u32> {
    let mut chars = string.chars();
    let sym = codepoint_to_keysym(chars.next()? as u32).unwrap_or(0);
    chars.next().is_none().then_some(sym)
}

macro_rules! default_merge_constructors {
    ($(fn $name:ident($($field:ident: $field_type:ty),*) -> $result:ident;)*) => {$(
        pub(crate) fn $name($($field: $field_type),*) -> $result {
            $result { merge: MergeMode::Default, $($field),* }
        }
    )*};
}

default_merge_constructors! {
    fn keycode_create(name: u32, value: i64) -> KeycodeDef;
    fn key_alias_create(alias: u32, real: u32) -> KeyAliasDef;
    fn vmod_create(name: u32, value: Option<ExprKind>) -> VModDef;
    fn var_create(name: Option<ExprKind>, value: Option<ExprKind>) -> VarDef;
    fn key_type_create(name: u32, body: Vec<VarDef>) -> NamedVarDef;
    fn symbols_create(name: u32, body: Vec<VarDef>) -> NamedVarDef;
    fn mod_map_create(modifier: u32, keys: Vec<ExprKind>) -> ModMapDef;
}

pub(crate) fn bool_var_create(ident: u32, set: bool) -> VarDef {
    VarDef {
        merge: MergeMode::Default,
        name: Some(ExprKind::Ident(ident)),
        value: Some(ExprKind::Boolean(set)),
    }
}

pub(crate) fn include_create(stmt_str: &str, mut merge: MergeMode) -> Option<Vec<IncludeStmt>> {
    let mut items: Vec<IncludeStmt> = Vec::new();
    let mut remaining: Option<&str> = Some(stmt_str);

    while let Some(input) = remaining.filter(|s| !s.is_empty()) {
        let (segment, nextop, rest) = match input.find(['+', '|', '^']) {
            Some(pos) => (
                &input[..pos],
                input.as_bytes()[pos],
                Some(&input[pos + 1..]),
            ),
            None => (input, 0, None),
        };
        let (segment, modifier) = segment.split_once(':').unwrap_or((segment, ""));
        let (file, map) = match segment.split_once('(') {
            Some(("", _)) => return None,
            Some((file, map)) => (file, map.strip_suffix(')')?),
            None => (segment, ""),
        };

        if file.is_empty() {
            remaining = rest;
            continue;
        }

        items.push(IncludeStmt {
            merge,
            file: file.into(),
            map: map.into(),
            modifier: modifier.into(),
        });

        merge = match nextop {
            b'|' => MergeMode::Augment,
            b'^' => MergeMode::Replace,
            _ => MergeMode::Override,
        };

        remaining = rest;
    }

    (!items.is_empty()).then_some(items)
}

pub(crate) fn xkb_file_create(
    type_0: FileType,
    name: Option<String>,
    defs: Vec<Statement>,
    flags: u32,
) -> Box<XkbFile> {
    let mut name_str = name.unwrap_or_default();
    xkb_escape_map_name(&mut name_str);
    Box::new(XkbFile {
        file_type: type_0,
        name: name_str,
        defs,
        flags,
    })
}

pub(crate) fn xkb_file_from_components(kkctgs: &XkbComponentNames) -> Option<Box<XkbFile>> {
    let mut file_stmts: Vec<Statement> = Vec::new();
    for (type_0, component_bytes) in [
        (FileType::Keycodes, &kkctgs.keycodes),
        (FileType::Types, &kkctgs.types),
        (FileType::Symbols, &kkctgs.symbols),
    ] {
        let end = component_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(component_bytes.len());
        let component_str = std::str::from_utf8(&component_bytes[..end]).unwrap_or("");
        let defs = vec![Statement::Include(include_create(
            component_str,
            MergeMode::Default,
        )?)];
        let file = xkb_file_create(type_0, None, defs, 0);
        file_stmts.push(Statement::XkbFile(*file));
    }
    Some(xkb_file_create(FileType::Keymap, None, file_stmts, 0))
}

#[derive(Copy, Clone, Default)]
pub(crate) struct Sval<'a> {
    pub(crate) data: &'a [u8],
}

pub(crate) struct Scanner<'a> {
    pub(crate) pos: usize,
    pub(crate) s: &'a [u8],
    pub(crate) buf: [u8; 1024],
    pub(crate) buf_pos: usize,
    pub(crate) token_pos: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(s: &'a [u8]) -> Self {
        Scanner {
            pos: 0,
            s,
            buf: [0; 1024],
            buf_pos: 0,
            token_pos: 0,
        }
    }

    #[inline]
    fn remaining_bytes(&self) -> &[u8] {
        &self.s[self.pos..]
    }

    #[inline]
    pub(crate) fn peek(&self) -> u8 {
        self.s.get(self.pos).copied().unwrap_or(0)
    }

    #[inline]
    pub(crate) fn eof(&self) -> bool {
        self.pos >= self.s.len()
    }

    #[inline]
    pub(crate) fn eol(&self) -> bool {
        self.peek() == b'\n'
    }

    #[inline]
    pub(crate) fn skip_to_eol(&mut self) {
        self.pos += self
            .remaining_bytes()
            .iter()
            .position(|&b| b == b'\n')
            .unwrap_or(self.s.len() - self.pos);
    }

    #[inline]
    pub(crate) fn next_byte(&mut self) -> u8 {
        if let Some(&byte) = self.s.get(self.pos) {
            self.pos += 1;
            byte
        } else {
            0
        }
    }

    #[inline]
    pub(crate) fn chr(&mut self, ch: u8) -> bool {
        if self.peek() != ch {
            return false;
        }
        self.pos += 1;
        true
    }

    #[inline]
    pub(crate) fn str_match(&mut self, string: &[u8]) -> bool {
        if self.remaining_bytes().starts_with(string) {
            self.pos += string.len();
            true
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn buf_append(&mut self, ch: u8) -> bool {
        if self.buf_pos + 1 >= self.buf.len() {
            return false;
        }
        self.buf[self.buf_pos] = ch;
        self.buf_pos += 1;
        true
    }

    #[inline]
    pub(crate) fn buf_appends_code_point(&mut self, c: u32) -> bool {
        let Some(buffer) = self.buf.get_mut(self.buf_pos..self.buf_pos + 4) else {
            return false;
        };
        let count = char::from_u32(c)
            .unwrap_or(char::REPLACEMENT_CHARACTER)
            .encode_utf8(buffer)
            .len();
        self.buf_pos += count;
        true
    }

    #[inline]
    pub(crate) fn oct(&mut self) -> Option<u8> {
        let mut i: u8 = 0;
        let mut c: u8 = 0;
        while self.peek() >= b'0' && self.peek() <= b'7' && (i as i32) < 4 {
            if (c as i32) < 0o40 {
                c = (c as i32 * 8 + self.next_byte() as i32 - b'0' as i32) as u8;
            } else {
                self.next_byte();
                return None;
            }
            i += 1;
        }
        (i > 0).then_some(c)
    }

    #[inline]
    pub(crate) fn unicode_code_point(&mut self) -> Option<u32> {
        if !self.chr(b'{') {
            return None;
        }
        let remaining = self.remaining_bytes();
        let (cp, count) = parse_hex_u32(remaining);
        if count > 0 {
            self.pos += count as usize;
        }
        let last_valid = self.pos;
        while !self.eof() && !self.eol() && self.peek() != b'"' && self.peek() != b'}' {
            self.next_byte();
        }
        if self.chr(b'}') {
            return (count > 0 && self.pos == last_valid + 1 && cp <= 0x10ffff).then_some(cp);
        }
        self.pos = last_valid;
        None
    }

    #[inline]
    pub(crate) fn check_supported_char_encoding(&mut self) -> bool {
        if self.str_match(b"\xEF\xBB\xBF") || self.s.len() < 2 {
            return true;
        }
        self.s[0] != 0 && self.s[1] != 0 && self.s[0].is_ascii()
    }

    #[inline]
    pub(crate) fn input_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.s[start..end]
    }
}

macro_rules! tokens { ($($name:ident = $value:literal),* $(,)?) => { $(pub(crate) const $name: i32 = $value;)* }; }
#[rustfmt::skip]
tokens! {
    ALTERNATE_GROUP = 65, FUNCTION_KEYS = 64, KEYPAD_KEYS = 63, MODIFIER_KEYS = 62, ALPHANUMERIC_KEYS = 61, HIDDEN = 60, DEFAULT = 59, PARTIAL = 58,
    KEYNAME = 57, IDENT = 56, FLOAT = 55, INTEGER = 54, DECIMAL_DIGIT = 53, STRING = 52, INVERT = 51, EXCLAM = 50,
    SEMI = 49, COMMA = 48, DOT = 47, CBRACKET = 46, OBRACKET = 45, CPAREN = 44, OPAREN = 43, CBRACE = 42,
    OBRACE = 41, TIMES = 40, DIVIDE = 39, MINUS = 38, PLUS = 37, EQUALS = 36, VIRTUAL = 35, LOGO = 34,
    SOLID = 33, OUTLINE = 32, TEXT = 31, OVERLAY = 30, SECTION = 29, ROW = 28, KEYS = 27, SHAPE = 26,
    INDICATOR = 25, MODIFIER_MAP = 24, GROUP = 23, ALIAS = 22, KEY = 21, ACTION_TOK = 20, INTERPRET = 19, TYPE = 18,
    VIRTUAL_MODS = 17, ALTERNATE = 16, REPLACE = 15, AUGMENT = 14, OVERRIDE = 13, INCLUDE = 12, XKB_LAYOUT = 11, XKB_SEMANTICS = 10,
    XKB_GEOMETRY = 9, XKB_COMPATMAP = 8, XKB_SYMBOLS = 7, XKB_TYPES = 6, XKB_KEYCODES = 5, XKB_KEYMAP = 4, ERROR_TOK = 3, END_OF_FILE = 0,
}
pub(crate) const YYEMPTY: i32 = -2;

// ── Keyword lookup ──

static KEYWORDS: &[(&[u8], i32)] = &[
    (b"key", KEY),
    (b"keys", KEYS),
    (b"augment", AUGMENT),
    (b"text", TEXT),
    (b"xkb_keymap", XKB_KEYMAP),
    (b"keypad_keys", KEYPAD_KEYS),
    (b"xkb_keycodes", XKB_KEYCODES),
    (b"xkb_geometry", XKB_GEOMETRY),
    (b"xkb_types", XKB_TYPES),
    (b"xkb_compat", XKB_COMPATMAP),
    (b"replace", REPLACE),
    (b"xkb_compat_map", XKB_COMPATMAP),
    (b"xkb_layout", XKB_LAYOUT),
    (b"xkb_symbols", XKB_SYMBOLS),
    (b"xkb_compatibility", XKB_COMPATMAP),
    (b"xkb_semantics", XKB_SEMANTICS),
    (b"type", TYPE),
    (b"alias", ALIAS),
    (b"xkb_compatibility_map", XKB_COMPATMAP),
    (b"alphanumeric_keys", ALPHANUMERIC_KEYS),
    (b"function_keys", FUNCTION_KEYS),
    (b"alternate", ALTERNATE),
    (b"shape", SHAPE),
    (b"action", ACTION_TOK),
    (b"section", SECTION),
    (b"row", ROW),
    (b"logo", LOGO),
    (b"alternate_group", ALTERNATE_GROUP),
    (b"hidden", HIDDEN),
    (b"virtual", VIRTUAL),
    (b"outline", OUTLINE),
    (b"default", DEFAULT),
    (b"modmap", MODIFIER_MAP),
    (b"virtual_modifiers", VIRTUAL_MODS),
    (b"overlay", OVERLAY),
    (b"override", OVERRIDE),
    (b"include", INCLUDE),
    (b"modifier_map", MODIFIER_MAP),
    (b"modifier_keys", MODIFIER_KEYS),
    (b"indicator", INDICATOR),
    (b"group", GROUP),
    (b"mod_map", MODIFIER_MAP),
    (b"interpret", INTERPRET),
    (b"solid", SOLID),
    (b"partial", PARTIAL),
];

// ── YYValue: safe replacement for the YYSTYPE union ──

/// Safe parser value stack type, replacing the old YYSTYPE union.
/// Each variant owns its data. `Default` produces `None`.
#[derive(Default)]
pub(crate) enum YYValue<'a> {
    #[default]
    None,
    Num(i64),
    FileType(FileType),
    Str(String),
    Sval(Sval<'a>),
    Atom(u32),
    Merge(MergeMode),
    MapFlags(u32),
    Keysym(u32),
    NoSymbolOrActionList(u32),
    Expr(ExprKind),
    ExprList(Vec<ExprKind>),
    Var(VarDef),
    VarList(Vec<VarDef>),
    VMod(VModDef),
    VModList(Vec<VModDef>),
    Interp(InterpDef),
    KeyType(NamedVarDef),
    Symbols(NamedVarDef),
    ModMask(ModMapDef),
    LedMap(()),
    LedName(()),
    Keycode(KeycodeDef),
    KeyAlias(KeyAliasDef),
    Unknown,
    File(Box<XkbFile>),
    FileList(Vec<XkbFile>),
    Stmt(Statement),
    StmtList(Vec<Statement>),
}

macro_rules! yy_take {
    ($($name:ident: $variant:ident($value:ident) -> $result:ty = $fallback:expr => $mapped:expr;)*) => {$(
        pub(crate) fn $name(&mut self) -> $result {
            match std::mem::take(self) { YYValue::$variant($value) => $mapped, _ => $fallback }
        }
    )*};
}

macro_rules! yy_as {
    ($($name:ident: $variant:ident($value:ident) -> $result:ty = $fallback:expr;)*) => {$(
        pub(crate) fn $name(&self) -> $result {
            match self { YYValue::$variant($value) => *$value, _ => $fallback }
        }
    )*};
}

impl<'a> YYValue<'a> {
    yy_take! {
        take_expr: Expr(v) -> Option<ExprKind> = None => Some(v);
        take_expr_list: ExprList(v) -> Vec<ExprKind> = Vec::new() => v;
        take_var: Var(v) -> Option<VarDef> = None => Some(v);
        take_var_list: VarList(v) -> Vec<VarDef> = Vec::new() => v;
        take_vmod: VMod(v) -> Option<VModDef> = None => Some(v);
        take_vmod_list: VModList(v) -> Vec<VModDef> = Vec::new() => v;
        take_file: File(v) -> Option<Box<XkbFile>> = None => Some(v);
        take_file_list: FileList(v) -> Vec<XkbFile> = Vec::new() => v;
        take_stmt_list: StmtList(v) -> Vec<Statement> = Vec::new() => v;
        take_str: Str(v) -> String = String::new() => v;
    }
    yy_as! {
        as_num: Num(v) -> i64 = 0;
        as_atom: Atom(v) -> u32 = 0;
        as_merge: Merge(v) -> MergeMode = MergeMode::Default;
        as_map_flags: MapFlags(v) -> u32 = 0;
        as_file_type: FileType(v) -> FileType = FileType::Keycodes;
        as_keysym: Keysym(v) -> u32 = 0;
        as_no_sym_or_action_list: NoSymbolOrActionList(v) -> u32 = 0;
        as_sval: Sval(v) -> Sval<'a> = Sval { data: &[] };
    }
}

fn number(s: &mut Scanner) -> Option<(i64, i32)> {
    let hex = s.str_match(b"0x");
    let (value, count) = if hex {
        parse_hex_u64(s.remaining_bytes())
    } else {
        parse_dec_u64(s.remaining_bytes())
    };
    if count < 0 || value > i64::MAX as u64 {
        return Some((0, ERROR_TOK));
    }
    if count == 0 {
        return None;
    }
    s.pos += count as usize;
    if hex {
        return Some((value as i64, INTEGER));
    }
    let token = if s.chr(b'.') {
        let (fraction, count) = parse_dec_u64(s.remaining_bytes());
        if count < 0 || fraction > i64::MAX as u64 {
            return Some((0, ERROR_TOK));
        }
        s.pos += count as usize;
        FLOAT
    } else if count == 1 {
        DECIMAL_DIGIT
    } else {
        INTEGER
    };
    Some((value as i64, token))
}

/// Lex one token and write the semantic value into `yylval`.
pub(crate) fn _xkbcommon_lex<'a>(
    yylval: &mut YYValue<'a>,
    s: &mut Scanner<'a>,
    ctx: &mut XkbContext,
) -> i32 {
    loop {
        while s.peek().is_ascii_whitespace() {
            s.next_byte();
        }
        if s.str_match(b"\xE2\x80\x8E") || s.str_match(b"\xE2\x80\x8F") {
            continue;
        }
        if !(s.str_match(b"//") || s.chr(b'#')) {
            break;
        }
        s.skip_to_eol();
    }
    if s.eof() {
        return END_OF_FILE;
    }
    s.token_pos = s.pos;
    s.buf_pos = 0;
    if s.chr(b'"') {
        while !s.eof() && !s.eol() && s.peek() != b'"' {
            if s.chr(b'\\') {
                match s.next_byte() {
                    b'\\' => s.buf_append(b'\\'),
                    b'"' => s.buf_append(b'"'),
                    b'n' => s.buf_append(b'\n'),
                    b't' => s.buf_append(b'\t'),
                    b'r' => s.buf_append(b'\r'),
                    b'b' => s.buf_append(b'\x08'),
                    b'f' => s.buf_append(b'\x0c'),
                    b'v' => s.buf_append(b'\x0b'),
                    b'e' => s.buf_append(b'\x1b'),
                    b'u' => s
                        .unicode_code_point()
                        .is_some_and(|cp| cp != 0 && s.buf_appends_code_point(cp)),
                    b'0'..=b'7' => {
                        s.pos -= 1;
                        s.oct()
                            .is_some_and(|octal| octal != 0 && s.buf_append(octal))
                    }
                    other => other != 0 && s.buf_append(other),
                };
            } else {
                let c = s.next_byte();
                s.buf_append(c);
            }
        }
        if !s.buf_append(0) || !s.chr(b'"') {
            return ERROR_TOK;
        }
        // Convert buffer to String (exclude null terminator)
        let buf_len = s.buf_pos.saturating_sub(1);
        let string = String::from_utf8_lossy(&s.buf[..buf_len]).into_owned();
        *yylval = YYValue::Str(string);
        return STRING;
    }
    if s.chr(b'<') {
        while s.peek().is_ascii_graphic() && s.peek() != b'>' {
            s.next_byte();
        }
        if !s.chr(b'>') {
            return ERROR_TOK;
        }
        let len: usize = s.pos - s.token_pos - 2;
        let keyname_bytes = s.input_slice(s.token_pos + 1, s.token_pos + 1 + len);
        *yylval = YYValue::Atom(ctx.atom_intern(keyname_bytes));
        return KEYNAME;
    }
    let punctuation = match s.peek() {
        b';' => SEMI,
        b'{' => OBRACE,
        b'}' => CBRACE,
        b'=' => EQUALS,
        b'[' => OBRACKET,
        b']' => CBRACKET,
        b'(' => OPAREN,
        b')' => CPAREN,
        b'.' => DOT,
        b',' => COMMA,
        b'+' => PLUS,
        b'-' => MINUS,
        b'*' => TIMES,
        b'/' => DIVIDE,
        b'!' => EXCLAM,
        b'~' => INVERT,
        _ => -1,
    };
    if punctuation >= 0 {
        s.next_byte();
        return punctuation;
    }
    if s.peek().is_ascii_alphabetic() || s.peek() == b'_' {
        while s.peek().is_ascii_alphanumeric() || s.peek() == b'_' {
            s.next_byte();
        }
        let word = s.input_slice(s.token_pos, s.pos);
        let tok = KEYWORDS
            .iter()
            .find_map(|&(keyword, token)| word.eq_ignore_ascii_case(keyword).then_some(token))
            .unwrap_or(-1);
        if tok >= 0 {
            return tok;
        }
        *yylval = YYValue::Sval(Sval {
            data: &s.s[s.token_pos..s.pos],
        });
        return IDENT;
    }
    if let Some((number, tok)) = number(s) {
        *yylval = YYValue::Num(number);
        return tok;
    }
    ERROR_TOK
}
pub(crate) fn xkb_parse_string(
    ctx: &mut XkbContext,
    input: &[u8],
    map: &str,
) -> Option<Box<XkbFile>> {
    let mut sc = Scanner::new(input);
    if !sc.check_supported_char_encoding() {
        return None;
    }
    parse(ctx, &mut sc, map)
}

// ── Include file processing (merged from include.rs) ──

use super::keymap::getenv_or;
use super::keymap::xkb_context_num_include_paths;

pub(crate) const INCLUDE_MAX_DEPTH: i32 = 15_i32;
fn is_merge_prefix(byte: u8) -> bool {
    matches!(byte, b'+' | b'|' | b'^')
}
static XKB_FILE_TYPE_INCLUDE_DIRS: [&str; 7] = [
    "keycodes", "types", "compat", "symbols", "geometry", "keymap", "rules",
];
fn directory_for_include(type_0: FileType) -> &'static str {
    XKB_FILE_TYPE_INCLUDE_DIRS
        .get(type_0 as usize)
        .copied()
        .unwrap_or("")
}
/// Expand `%H`, `%S`, `%E`, `%%` in the given name string.
/// Returns `Some(expanded)` on success, `None` on error.
fn expand_percent(type_dir: &str, name: &str) -> Option<String> {
    let max_len = 4096usize;
    let mut result = String::new();
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next()? {
                '%' => result.push('%'),
                'H' => result.push_str(&std::env::var("HOME").ok()?),
                'S' => {
                    let sys = getenv_or("XKB_CONFIG_ROOT", DFLT_XKB_CONFIG_ROOT);
                    result.push_str(&sys);
                    result.push('/');
                    result.push_str(type_dir);
                }
                'E' => {
                    let extra = getenv_or("XKB_CONFIG_EXTRA_PATH", DFLT_XKB_CONFIG_EXTRA_PATH);
                    result.push_str(&extra);
                    result.push('/');
                    result.push_str(type_dir);
                }
                _ => return None,
            }
        } else {
            result.push(c);
        }
        if result.len() > max_len {
            return None;
        }
    }
    Some(result)
}
/// Expand `%`-sequences in `name`. Returns:
/// - `Ok(None)` if no `%` found (no expansion needed)
/// - `Ok(Some(expanded))` if expansion succeeded
/// - `Err(())` on error
pub(crate) fn expand_path_str(name: &str, file_type: FileType) -> Result<Option<String>, ()> {
    let Some(k) = name.find('%') else {
        return Ok(None);
    };
    let expanded = expand_percent(directory_for_include(file_type), &name[k..]).ok_or(())?;
    Ok(Some(format!("{}{}", &name[..k], expanded)))
}

fn include_path<'a>(
    name: &'a str,
    file_type: FileType,
) -> Option<(std::borrow::Cow<'a, str>, bool)> {
    match expand_path_str(name, file_type) {
        Ok(Some(path)) => Some((std::borrow::Cow::Owned(path), true)),
        Ok(None) => Some((std::borrow::Cow::Borrowed(name), false)),
        Err(()) => None,
    }
}
pub(crate) fn find_file_in_xkb_path(
    ctx: &mut XkbContext,
    name: &str,
    type_0: FileType,
    offset: &mut u32,
) -> Option<std::sync::Arc<Vec<u8>>> {
    let type_dir = directory_for_include(type_0);
    let path_count = xkb_context_num_include_paths(ctx);
    for i in *offset..path_count {
        let path = format!("{}/{}/{}", ctx.includes[i as usize], type_dir, name);
        if path.len() < 4096 {
            if let Some(data) = read_file_cached(&path) {
                *offset = i;
                return Some(data);
            }
        }
    }
    None
}

fn find_include_file(
    ctx: &mut XkbContext,
    name: &str,
    file_type: FileType,
    expanded: bool,
    offset: &mut u32,
) -> Option<std::sync::Arc<Vec<u8>>> {
    if name.starts_with('/') {
        if *offset == 0 {
            read_file_cached(name)
        } else {
            None
        }
    } else if expanded {
        None
    } else {
        find_file_in_xkb_path(ctx, name, file_type, offset)
    }
}

pub(crate) fn exceeds_include_max_depth(include_depth: u32) -> bool {
    include_depth >= INCLUDE_MAX_DEPTH as u32
}
pub(crate) fn process_include_file(
    ctx: &mut XkbContext,
    stmt: &IncludeStmt,
    file_type: FileType,
) -> Option<Box<XkbFile>> {
    let (stmt_file, expanded) = include_path(&stmt.file, file_type)?;

    let mut offset = 0;
    let mut candidate = None;
    while let Some(file_data) = find_include_file(ctx, &stmt_file, file_type, expanded, &mut offset)
    {
        if let Some(parsed) = xkb_parse_string(ctx, &file_data, &stmt.map) {
            if parsed.file_type == file_type {
                if !stmt.map.is_empty() || parsed.flags != 0 {
                    return Some(parsed);
                }
                if candidate.is_none() {
                    candidate = Some(parsed);
                }
            }
        }
        offset += 1;
    }
    candidate
}

pub(crate) type CompileFileFn = for<'a> fn(Option<&mut XkbFile>, &mut XkbKeymapInfo<'a>) -> bool;
/// Version that takes the mod_set separately to allow calling on fields of keymap.
#[inline]
fn compute_effective_mask_with(mod_set: &XkbModSet, mods: &mut XkbMods) {
    let unknown_mods: u32 = !((1_u64 << mod_set.num_mods).wrapping_sub(1_u64) as u32);
    mods.mask = mod_mask_get_effective(mod_set, mods.mods) | mods.mods & unknown_mods;
}
fn update_pending_key_fields(info: &mut XkbKeymapInfo<'_>, key_idx: usize) -> bool {
    if info.keymap.keys[key_idx].out_of_range_pending_group {
        let idx = info.keymap.keys[key_idx].out_of_range_group_number as usize;
        if !info.pending_computations[idx].computed {
            // Temporarily take the expr out to avoid borrow conflict with info
            let expr_box = info.pending_computations[idx].expr.take().unwrap();
            let mut group: u32 = 0;
            let mut _pending = false;
            let resolve_ret = expr_resolve_group(info, &expr_box, true, &mut group, &mut _pending);
            info.pending_computations[idx].expr = Some(expr_box);
            match resolve_ret {
                ParseStatus::Success => {
                    info.pending_computations[idx].computed = true;
                    info.pending_computations[idx].value = group.wrapping_sub(1);
                }
                ParseStatus::Fatal => {
                    return info.strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0;
                }
                _ => {}
            }
        }
        info.keymap.keys[key_idx].out_of_range_pending_group = false;
        info.keymap.keys[key_idx].out_of_range_group_number = info.pending_computations[idx].value;
    }
    true
}
fn update_pending_action_fields(info: &mut XkbKeymapInfo<'_>, act: &mut XkbAction) -> bool {
    match act {
        XkbAction::GroupSet(g) | XkbAction::GroupLatch(g) | XkbAction::GroupLock(g) => {
            if g.flags.contains(ActionFlags::PENDING_COMPUTATION) {
                let pc_idx = g.group as usize;
                if !info.pending_computations[pc_idx].computed {
                    let mut group: u32 = 0;
                    let absolute: bool = g.flags.contains(ActionFlags::ABSOLUTE_SWITCH);
                    let mut _pending = false;
                    let expr_box = info.pending_computations[pc_idx].expr.take().unwrap();
                    let resolve_ret =
                        expr_resolve_group(info, &expr_box, absolute, &mut group, &mut _pending);
                    info.pending_computations[pc_idx].expr = Some(expr_box);
                    match resolve_ret {
                        ParseStatus::Fatal => {
                            return false;
                        }
                        ParseStatus::Recoverable => {}
                        _ => {
                            info.pending_computations[pc_idx].computed = true;
                            if absolute {
                                info.pending_computations[pc_idx].value = group.wrapping_sub(1);
                            } else {
                                info.pending_computations[pc_idx].value = group;
                                if info.pending_computations[pc_idx].expr.as_ref().is_some_and(
                                    |expr| {
                                        matches!(
                                            expr,
                                            ExprKind::Unary {
                                                op: UnaryOp::Negate,
                                                ..
                                            }
                                        )
                                    },
                                ) {
                                    info.pending_computations[pc_idx].value =
                                        -(info.pending_computations[pc_idx].value as i32) as u32;
                                }
                            }
                        }
                    }
                }
                g.group = info.pending_computations[pc_idx].value as i32;
                g.flags &= !ActionFlags::PENDING_COMPUTATION;
            }
            true
        }
        _ => true,
    }
}
fn mod_index_by_name(keymap: &XkbKeymap, name: &str) -> Option<u32> {
    keymap.mods.mods[..keymap.mods.num_mods as usize]
        .iter()
        .position(|modifier| {
            keymap
                .ctx
                .atom_text(modifier.name)
                .eq_ignore_ascii_case(name)
        })
        .map(|index| index as u32)
}

pub(crate) fn wkb_group_action(sym: u32) -> Option<XkbAction> {
    let relative = |group| XkbGroupAction {
        group,
        ..Default::default()
    };
    let absolute = |group| XkbGroupAction {
        flags: ActionFlags::ABSOLUTE_SWITCH,
        group,
    };
    match sym {
        0xff7e => Some(XkbAction::GroupSet(relative(1))), // Mode_switch
        0xfe06 => Some(XkbAction::GroupLatch(absolute(1))), // ISO_Group_Latch
        0xfe08 => Some(XkbAction::GroupLock(relative(1))), // ISO_Next_Group
        0xfe0a => Some(XkbAction::GroupLock(relative(-1))), // ISO_Prev_Group
        0xfe0c => Some(XkbAction::GroupLock(absolute(0))), // ISO_First_Group
        0xfe0e => Some(XkbAction::GroupLock(absolute(1))), // ISO_Last_Group
        _ => None,
    }
}

fn is_modifier_keysym(sym: u32) -> bool {
    matches!(sym, 0xff2d..=0xff30 | 0xff7e | 0xff7f | 0xffe1..=0xffee | 0xfe01..=0xfe13)
}

/// Apply the tiny subset of xkb_compat semantics consumed by WKB.
///
/// The full compatibility compiler builds runtime actions, controls and LEDs.
/// WKB discards all of those. Deriving the observable pieces from keysyms avoids
/// loading and compiling the entire compat include tree.
fn apply_wkb_compat(keymap: &mut XkbKeymap) {
    let named_vmods = [
        ("NumLock", &[0xff7f][..]),
        ("LevelThree", &[0xfe03, 0xfe04, 0xfe05][..]),
        ("LevelFive", &[0xfe11, 0xfe12, 0xfe13][..]),
        ("Alt", &[0xffe9, 0xffea][..]),
        ("Meta", &[0xffe7, 0xffe8][..]),
        ("Super", &[0xffeb, 0xffec][..]),
        ("Hyper", &[0xffed, 0xffee][..]),
        ("ScrollLock", &[0xff14][..]),
    ];
    let vmods: Vec<(u32, &[u32])> = named_vmods
        .into_iter()
        .filter_map(|(name, syms)| mod_index_by_name(keymap, name).map(|index| (index, syms)))
        .collect();

    for key in &mut keymap.keys {
        let first_sym = key
            .groups
            .first()
            .and_then(|group| group.levels.first())
            .and_then(|level| level.syms.first())
            .copied()
            .unwrap_or(XKB_KEY_NO_SYMBOL);
        if !key.explicit_repeat {
            key.repeats = first_sym != XKB_KEY_NO_SYMBOL && !is_modifier_keysym(first_sym);
        }

        let mut vmodmap = 0;
        if !key.explicit_vmodmap {
            let level_one_syms = key
                .groups
                .first()
                .and_then(|group| group.levels.first())
                .map_or(&[][..], |level| level.syms.as_slice());
            for &sym in level_one_syms {
                for &(index, candidates) in &vmods {
                    if candidates.contains(&sym) {
                        vmodmap |= 1 << index;
                    }
                }
            }
        }
        for group in &mut key.groups {
            for level in &mut group.levels {
                if !group.explicit_actions && level.action.is_none() {
                    level.action = level.syms.iter().copied().find_map(wkb_group_action);
                }
            }
        }
        if !key.explicit_vmodmap {
            key.vmodmap = vmodmap;
        }
    }
}

fn update_derived_keymap_fields(info: &mut XkbKeymapInfo<'_>) -> bool {
    let keymap: &mut XkbKeymap = &mut *info.keymap;
    keymap.key_names = Vec::new();
    let start_idx = if keymap.num_keys_low == 0 {
        0
    } else {
        keymap.min_key_code
    };
    keymap.num_groups = keymap.keys[start_idx as usize..keymap.num_keys as usize]
        .iter()
        .fold(keymap.num_groups, |max, key| max.max(key.num_groups));
    let pending_computations: bool = !info.pending_computations.is_empty();
    if pending_computations {
        let num_groups = info.keymap.num_groups.max(1);
        info.lookup.group_index_names[1] = lookup_entry(GROUP_LAST_INDEX_NAME, num_groups);
    }
    apply_wkb_compat(info.keymap);
    update_mod_mappings(info);
    compute_type_entry_masks(info);
    if update_key_action_fields(info, pending_computations).is_err() {
        return false;
    }
    true
}

fn update_mod_mappings(info: &mut XkbKeymapInfo<'_>) {
    let keymap = &mut *info.keymap;
    let start_idx = if keymap.num_keys_low == 0 {
        0_u32
    } else {
        keymap.min_key_code
    };
    for ki in start_idx..keymap.num_keys {
        let key_vmodmap = keymap.keys[ki as usize].vmodmap;
        let key_modmap = keymap.keys[ki as usize].modmap;
        for idx in _XKB_MOD_INDEX_NUM_ENTRIES as usize..keymap.mods.num_mods as usize {
            if key_vmodmap & 1 << idx as u32 != 0 {
                keymap.mods.mods[idx].mapping |= key_modmap;
            }
        }
    }
    if keymap.format >= XKB_KEYMAP_FORMAT_TEXT_V2 {
        for idx in _XKB_MOD_INDEX_NUM_ENTRIES as usize..keymap.mods.num_mods as usize {
            let mask: u32 = 1 << idx as u32;
            if keymap.mods.mods[idx].mapping == 0 && keymap.mods.explicit_vmods & mask == 0 {
                keymap.mods.mods[idx].mapping = mask;
                keymap.mods.explicit_vmods |= mask;
            }
        }
    }
}

fn has_unbound_vmods(keymap: &XkbKeymap, mods: &XkbMods) -> bool {
    (_XKB_MOD_INDEX_NUM_ENTRIES..keymap.mods.num_mods)
        .any(|k| mods.mods & 1 << k != 0 && keymap.mods.mods[k as usize].mapping == 0)
}

fn compute_type_entry_masks(info: &mut XkbKeymapInfo<'_>) {
    let keymap = &mut *info.keymap;
    for i_0 in 0..keymap.types.len() {
        compute_effective_mask_with(&keymap.mods, &mut keymap.types[i_0].mods);
        for j in 0..keymap.types[i_0].entries.len() {
            if has_unbound_vmods(keymap, &keymap.types[i_0].entries[j].mods) {
                keymap.types[i_0].entries[j].mods.mask = 0_u32;
            } else {
                compute_effective_mask_with(&keymap.mods, &mut keymap.types[i_0].entries[j].mods);
                compute_effective_mask_with(
                    &keymap.mods,
                    &mut keymap.types[i_0].entries[j].preserve,
                );
            }
        }
    }
}

fn update_key_action_fields(
    info: &mut XkbKeymapInfo<'_>,
    pending_computations: bool,
) -> Result<(), ()> {
    let start_idx = if info.keymap.num_keys_low == 0 {
        0_u32
    } else {
        info.keymap.min_key_code
    };
    for ki in start_idx..info.keymap.num_keys {
        if !update_pending_key_fields(info, ki as usize) {
            return Err(());
        }
        if !pending_computations {
            continue;
        }
        let key_num_groups = info.keymap.keys[ki as usize].num_groups;
        for i_1 in 0..key_num_groups {
            let num_levels = {
                let key = &info.keymap.keys[ki as usize];
                info.keymap.types[key.groups[i_1 as usize].type_idx as usize].num_levels
            };
            for j_0 in 0..num_levels {
                let action =
                    info.keymap.keys[ki as usize].groups[i_1 as usize].levels[j_0 as usize].action;
                if let Some(mut act) = action {
                    if !update_pending_action_fields(info, &mut act) {
                        return Err(());
                    }
                    info.keymap.keys[ki as usize].groups[i_1 as usize].levels[j_0 as usize]
                        .action = Some(act);
                }
            }
        }
    }
    Ok(())
}

static COMPILE_FILE_FNS: [(FileType, CompileFileFn); 3] = [
    (FileType::Keycodes, compile_keycodes),
    (FileType::Types, compile_key_types),
    (FileType::Symbols, compile_symbols),
];
pub(crate) fn compile_keymap(file: &mut XkbFile, keymap: &mut XkbKeymap) -> bool {
    let mut file_indices: [Option<usize>; 4] = [None; 4];
    for (idx, stmt) in file.defs.iter().enumerate() {
        let Statement::XkbFile(ref sub_file) = stmt else {
            continue;
        };
        if sub_file.file_type as usize <= FileType::Symbols as usize
            && file_indices[sub_file.file_type as usize].is_none()
        {
            file_indices[sub_file.file_type as usize] = Some(idx);
        }
    }
    let km_format = keymap.format;
    let km_num_groups = keymap.num_groups;
    let mut info = XkbKeymapInfo {
        keymap,
        strict: if km_format == XKB_KEYMAP_FORMAT_TEXT_V1 {
            PARSER_V1_LAX_FLAGS
        } else {
            PARSER_V2_LAX_FLAGS
        },
        features: XkbcompFeatures {
            max_groups: XKB_MAX_GROUPS,
            max_overlays: XKB_OVERLAY_MAX,
            group_lock_on_release: km_format >= XKB_KEYMAP_FORMAT_TEXT_V2,
            overlapping_overlays: km_format >= XKB_KEYMAP_FORMAT_TEXT_V2,
        },
        lookup: XkbcompLookup {
            group_index_names: [
                lookup_entry("first", 1),
                lookup_entry(
                    if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    km_num_groups,
                ),
                lookup_entry("", 0),
            ],
        },
        pending_computations: Vec::new(),
    };
    for (file_type, compile) in COMPILE_FILE_FNS {
        let file_arg: Option<&mut XkbFile> = file_indices[file_type as usize].map(|idx| {
            if let Statement::XkbFile(ref mut sub_file) = file.defs[idx] {
                sub_file
            } else {
                unreachable!()
            }
        });
        if !compile(file_arg, &mut info) {
            return false;
        }
    }
    update_derived_keymap_fields(&mut info)
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum RuleScope {
    Single,
    First,
    Later,
    Any,
    Index(usize),
}

#[derive(Clone, Copy)]
enum RuleInput {
    Model,
    Layout,
    Variant,
    Option,
}

const MAX_INCLUDE_DEPTH: u32 = 5;

struct RuleMapping {
    inputs: Vec<RuleInput>,
    outputs: Vec<usize>,
    targets: Vec<Option<usize>>,
    active: Vec<bool>,
    deferred: bool,
    has_option: bool,
    pending: Vec<(usize, usize, Vec<u8>)>,
}

struct RuleOption<'a> {
    value: &'a str,
    layout: Option<usize>,
}

pub(crate) struct Matcher<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    model: &'a str,
    layouts: Vec<&'a str>,
    variants: Vec<&'a str>,
    options: Vec<RuleOption<'a>>,
    groups: HashMap<String, Vec<String>>,
    kccgst: [Vec<u8>; 5],
}

fn rule_tokens(line: &str) -> Vec<&str> {
    let bytes = line.as_bytes();
    let mut tokens = Vec::new();
    let mut pos = 0;
    while pos < bytes.len() {
        while bytes.get(pos).is_some_and(u8::is_ascii_whitespace) {
            pos += 1;
        }
        if pos == bytes.len() || bytes[pos..].starts_with(b"//") {
            break;
        }
        if bytes[pos] == b'=' {
            tokens.push("=");
            pos += 1;
            continue;
        }
        let start = pos;
        while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() && bytes[pos] != b'=' {
            pos += 1;
        }
        tokens.push(&line[start..pos]);
    }
    tokens
}

fn rule_scope(field: &str, name: &str) -> Option<RuleScope> {
    let suffix = field.strip_prefix(name)?;
    match suffix {
        "" | "[single]" => Some(RuleScope::Single),
        "[first]" => Some(RuleScope::First),
        "[later]" => Some(RuleScope::Later),
        "[any]" => Some(RuleScope::Any),
        _ => suffix
            .strip_prefix('[')?
            .strip_suffix(']')?
            .parse::<usize>()
            .ok()
            .and_then(|index| index.checked_sub(1))
            .map(RuleScope::Index),
    }
}

fn output_index(name: &str) -> Option<usize> {
    ["keycodes", "types", "compat", "symbols", "geometry"]
        .iter()
        .position(|candidate| *candidate == name)
}

impl RuleMapping {
    fn parse(tokens: &[&str], layouts: usize, variants: usize) -> Option<Self> {
        let equals = tokens.iter().position(|token| *token == "=")?;
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut layout_scope = None;
        let mut variant_scope = None;
        for &field in &tokens[..equals] {
            let input = if field == "model" {
                RuleInput::Model
            } else if field == "option" {
                RuleInput::Option
            } else if field.starts_with("layout") {
                let scope = rule_scope(field, "layout")?;
                layout_scope = Some(scope);
                RuleInput::Layout
            } else if field.starts_with("variant") {
                let scope = rule_scope(field, "variant")?;
                variant_scope = Some(scope);
                RuleInput::Variant
            } else {
                return None;
            };
            if inputs
                .iter()
                .any(|old| std::mem::discriminant(old) == std::mem::discriminant(&input))
            {
                return None;
            }
            inputs.push(input);
        }
        for &field in &tokens[equals + 1..] {
            let output = output_index(field)?;
            if outputs.contains(&output) {
                return None;
            }
            outputs.push(output);
        }
        if inputs.is_empty()
            || outputs.is_empty()
            || layout_scope.zip(variant_scope).is_some_and(|(a, b)| a != b)
        {
            return None;
        }
        let scope = layout_scope.or(variant_scope);
        let valid = |scope, len| match scope {
            RuleScope::Single => len <= 1,
            RuleScope::Index(index) => len >= 2 && index < len,
            _ => len > 0,
        };
        if layout_scope.is_some_and(|scope| !valid(scope, layouts))
            || variant_scope.is_some_and(|scope| !valid(scope, variants))
        {
            return None;
        }
        let targets = match scope {
            None => vec![None],
            Some(RuleScope::Single | RuleScope::First) => vec![Some(0)],
            Some(RuleScope::Later) => (1..layouts).map(Some).collect(),
            Some(RuleScope::Any) => (0..layouts).map(Some).collect(),
            Some(RuleScope::Index(index)) => vec![Some(index)],
        };
        let deferred = matches!(scope, Some(RuleScope::Later | RuleScope::Any));
        let has_option = inputs
            .iter()
            .any(|input| matches!(input, RuleInput::Option));
        Some(Self {
            inputs,
            outputs,
            active: vec![true; targets.len()],
            targets,
            deferred,
            has_option,
            pending: Vec::new(),
        })
    }

    fn flush(&mut self, matcher: &mut Matcher<'_>) {
        if self.deferred {
            self.pending.sort_by_key(|entry| entry.0);
        }
        for (_, output, value) in self.pending.drain(..) {
            concat_kccgst(&mut matcher.kccgst[output], &value);
        }
    }
}

impl Matcher<'_> {
    fn pattern_matches(&self, pattern: &str, value: &str, empty_wildcard: bool) -> bool {
        match pattern {
            "*" => empty_wildcard || !value.is_empty(),
            "+" | "<some>" => !value.is_empty(),
            "<none>" => value.is_empty(),
            "<any>" => true,
            _ if pattern.starts_with('$') => self
                .groups
                .get(&pattern[1..])
                .is_some_and(|group| group.iter().any(|element| element == value)),
            _ => pattern == value,
        }
    }

    fn input_matches(&self, input: RuleInput, pattern: &str, target: Option<usize>) -> bool {
        match input {
            RuleInput::Model => self.pattern_matches(pattern, self.model, true),
            RuleInput::Layout => target
                .and_then(|index| self.layouts.get(index))
                .is_some_and(|value| self.pattern_matches(pattern, value, false)),
            RuleInput::Variant => target
                .and_then(|index| self.variants.get(index))
                .is_some_and(|value| self.pattern_matches(pattern, value, false)),
            RuleInput::Option => self.options.iter().any(|option| {
                (option.layout.is_none() || option.layout == target)
                    && self.pattern_matches(pattern, option.value, true)
            }),
        }
    }

    fn apply_rule(&mut self, mapping: &mut RuleMapping, tokens: &[&str]) {
        let Some(equals) = tokens.iter().position(|token| *token == "=") else {
            return;
        };
        let (patterns, values) = (&tokens[..equals], &tokens[equals + 1..]);
        if patterns.len() != mapping.inputs.len() || values.len() != mapping.outputs.len() {
            return;
        }
        for target_pos in 0..mapping.targets.len() {
            if !mapping.active[target_pos] {
                continue;
            }
            let target = mapping.targets[target_pos];
            if !mapping
                .inputs
                .iter()
                .zip(patterns)
                .all(|(&input, &pattern)| self.input_matches(input, pattern, target))
            {
                continue;
            }
            for (&output, &value) in mapping.outputs.iter().zip(values) {
                if !matches!(output, 0 | 1 | 3) {
                    continue;
                }
                if let Some(expanded) = self.expand(value, target) {
                    if mapping.deferred {
                        mapping
                            .pending
                            .push((target.unwrap_or(0), output, expanded));
                    } else {
                        concat_kccgst(&mut self.kccgst[output], &expanded);
                    }
                }
            }
            if !mapping.has_option {
                mapping.active[target_pos] = false;
            }
        }
    }

    fn expand(&self, value: &str, target: Option<usize>) -> Option<Vec<u8>> {
        let bytes = value.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut pos = 0;
        while pos < bytes.len() {
            if bytes[pos] != b'%' {
                out.push(bytes[pos]);
                pos += 1;
                continue;
            }
            pos += 1;
            if bytes.get(pos) == Some(&b'i') {
                out.extend_from_slice((target? + 1).to_string().as_bytes());
                pos += 1;
                continue;
            }
            let (prefix, suffix) = match bytes.get(pos).copied() {
                Some(b'(') => {
                    pos += 1;
                    (Some(b'('), Some(b')'))
                }
                Some(prefix @ (b'_' | b'-' | b'+' | b'|' | b'^')) => {
                    pos += 1;
                    (Some(prefix), None)
                }
                _ => (None, None),
            };
            let values = match bytes.get(pos).copied()? {
                b'm' => None,
                b'l' => Some(&self.layouts),
                b'v' => Some(&self.variants),
                _ => return None,
            };
            let model = (values.is_none()).then_some(self.model);
            pos += 1;
            let mut selected = None;
            if bytes.get(pos) == Some(&b'[') {
                let end = bytes[pos..].iter().position(|byte| *byte == b']')? + pos;
                let index = &value[pos + 1..end];
                selected = if index == "%i" {
                    target
                } else {
                    index.parse::<usize>().ok()?.checked_sub(1)
                };
                pos = end + 1;
            }
            if let Some(suffix) = suffix {
                if bytes.get(pos) != Some(&suffix) {
                    return None;
                }
                pos += 1;
            }
            let replacement = if let Some(model) = model {
                model
            } else {
                let values = values.unwrap();
                let index = selected.or_else(|| (values.len() == 1).then_some(0));
                index
                    .and_then(|index| values.get(index).copied())
                    .unwrap_or("")
            };
            if !replacement.is_empty() {
                if let Some(prefix) = prefix {
                    out.push(prefix);
                }
                out.extend_from_slice(replacement.as_bytes());
                if let Some(suffix) = suffix {
                    out.push(suffix);
                }
            }
        }
        Some(expand_all_groups(&out, self.layouts.len().max(1)))
    }
}

fn expand_all_groups(value: &[u8], groups: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(value.len());
    let mut start = 0;
    while start < value.len() {
        let next = value[start + 1..]
            .iter()
            .position(|byte| is_merge_prefix(*byte))
            .map_or(value.len(), |offset| start + offset + 1);
        let item = &value[start..next];
        if let Some(base) = item.strip_suffix(b":all") {
            out.extend_from_slice(base);
            out.push(b':');
            out.extend_from_slice(b"1");
            let body = base
                .first()
                .filter(|byte| is_merge_prefix(**byte))
                .map_or(base, |_| &base[1..]);
            for group in 2..=groups {
                out.push(b'+');
                out.extend_from_slice(body);
                out.push(b':');
                out.extend_from_slice(group.to_string().as_bytes());
            }
        } else {
            out.extend_from_slice(item);
        }
        start = next;
    }
    out
}

#[inline]
fn concat_kccgst(into: &mut Vec<u8>, from: &[u8]) {
    if from.first().is_some_and(|byte| is_merge_prefix(*byte)) || into.is_empty() {
        into.extend_from_slice(from);
    } else if into.first().is_some_and(|byte| is_merge_prefix(*byte)) {
        into.splice(..0, from.iter().copied());
    }
}

fn parse_rules_file(matcher: &mut Matcher<'_>, data: &[u8], depth: u32) -> bool {
    let Ok(text) = std::str::from_utf8(data) else {
        return false;
    };
    let mut logical = String::new();
    let mut mapping: Option<RuleMapping> = None;
    for raw in text.lines().chain(std::iter::once("")) {
        let raw = raw.trim_end_matches('\r');
        if let Some(part) = raw.trim_end().strip_suffix('\\') {
            logical.push_str(part);
            logical.push(' ');
            continue;
        }
        logical.push_str(raw);
        let tokens = rule_tokens(&logical);
        if tokens.first() == Some(&"!") {
            if let Some(mut old) = mapping.take() {
                old.flush(matcher);
            }
            if tokens.get(1).is_some_and(|token| token.starts_with('$')) {
                if tokens.get(2) == Some(&"=") {
                    matcher.groups.insert(
                        tokens[1][1..].to_owned(),
                        tokens[3..].iter().map(|s| (*s).to_owned()).collect(),
                    );
                }
            } else if tokens.get(1) == Some(&"include") {
                if let Some(name) = tokens.get(2) {
                    include_rules(matcher, name, depth);
                }
            } else {
                mapping =
                    RuleMapping::parse(&tokens[1..], matcher.layouts.len(), matcher.variants.len());
            }
        } else if let Some(mapping) = &mut mapping {
            matcher.apply_rule(mapping, &tokens);
        }
        logical.clear();
    }
    if let Some(mut mapping) = mapping {
        mapping.flush(matcher);
    }
    true
}

fn include_rules(matcher: &mut Matcher<'_>, name: &str, depth: u32) {
    if depth >= MAX_INCLUDE_DEPTH {
        return;
    }
    let Some((name, expanded)) = include_path(name, FileType::Rules) else {
        return;
    };
    let mut offset = 0;
    while let Some(data) =
        find_include_file(matcher.ctx, &name, FileType::Rules, expanded, &mut offset)
    {
        if parse_rules_file(matcher, &data, depth + 1) {
            return;
        }
        offset += 1;
    }
}

pub fn matcher_new_from_names<'a>(ctx: &'a mut XkbContext, rmlvo: &'a XkbRuleNames) -> Matcher<'a> {
    let split = |value: &'a str| {
        let value = value.strip_suffix(',').unwrap_or(value);
        if value.is_empty() {
            vec![""]
        } else {
            value.split(',').map(str::trim).collect()
        }
    };
    let layouts = split(&rmlvo.layout);
    let mut variants = split(&rmlvo.variant);
    variants.resize(layouts.len(), "");
    let options = split(&rmlvo.options)
        .into_iter()
        .map(|option| {
            let (value, layout) =
                option
                    .rsplit_once('!')
                    .map_or((option, None), |(value, layout)| {
                        (
                            value,
                            layout
                                .parse::<usize>()
                                .ok()
                                .and_then(|index| index.checked_sub(1)),
                        )
                    });
            RuleOption { value, layout }
        })
        .collect();
    Matcher {
        ctx,
        model: &rmlvo.model,
        layouts,
        variants,
        options,
        groups: HashMap::new(),
        kccgst: std::array::from_fn(|_| Vec::new()),
    }
}

fn partial_rules(rules: &str, suffix: &str, matcher: &mut Matcher<'_>) -> bool {
    let name = format!("{rules}{suffix}");
    if name.len() >= 60 {
        return false;
    }
    let mut offset = 0;
    while let Some(data) = find_file_in_xkb_path(matcher.ctx, &name, FileType::Rules, &mut offset) {
        if !parse_rules_file(matcher, &data, 0) {
            return false;
        }
        offset += 1;
    }
    true
}

pub fn xkb_resolve_rules(
    rules: &str,
    matcher: &mut Matcher<'_>,
    out: &mut XkbComponentNames,
    explicit_layouts: &mut u32,
) -> bool {
    let mut offset = 0;
    let Some(data) = find_file_in_xkb_path(matcher.ctx, rules, FileType::Rules, &mut offset) else {
        return false;
    };
    if !partial_rules(rules, ".pre", matcher)
        || !parse_rules_file(matcher, &data, 0)
        || !partial_rules(rules, ".post", matcher)
        || [0, 1, 3]
            .into_iter()
            .any(|index| matcher.kccgst[index].is_empty())
    {
        return false;
    }
    for (index, target) in [
        (0, &mut out.keycodes),
        (1, &mut out.types),
        (3, &mut out.symbols),
    ] {
        let source = &mut matcher.kccgst[index];
        *target = std::mem::take(source);
        target.push(0);
    }
    *explicit_layouts = 1;
    let mut pos = 0;
    while let Some(colon) = out.symbols[pos..].iter().position(|byte| *byte == b':') {
        pos += colon + 1;
        let (group, count) = parse_dec_u32(&out.symbols[pos..]);
        if count > 0
            && out
                .symbols
                .get(pos + count as usize)
                .is_some_and(|byte| *byte == 0 || is_merge_prefix(*byte))
            && (1..=XKB_MAX_GROUPS).contains(&group)
        {
            *explicit_layouts = (*explicit_layouts).max(group);
            pos += count as usize;
        }
    }
    true
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use lasso::Key as _;

pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: u32 = 2;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: u32 = 1;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_WRAP: u32 = 0;

pub(crate) const XKB_KEYMAP_FORMAT_TEXT_V2: u32 = 2;
pub(crate) const XKB_KEYMAP_FORMAT_TEXT_V1: u32 = 1;

#[derive(Clone, Debug, Default)]
pub(crate) struct XkbRuleNames {
    pub(crate) rules: String,
    pub(crate) model: String,
    pub(crate) layout: String,
    pub(crate) variant: String,
    pub(crate) options: String,
}

#[derive(Clone)]
pub(crate) struct XkbContext {
    pub(crate) includes: Vec<String>,
    pub(crate) failed_includes: Vec<String>,
    pub(crate) atom_table: lasso::Rodeo,
    pub(crate) pending_default_includes: bool,
}

impl XkbContext {
    pub(crate) fn atom_text(&self, atom: u32) -> &str {
        if atom == XKB_ATOM_NONE {
            return "";
        }
        let key = lasso::Key::try_from_usize((atom - 1) as usize).expect("invalid atom key");
        self.atom_table.try_resolve(&key).unwrap_or("")
    }

    pub(crate) fn atom_intern(&mut self, bytes: &[u8]) -> u32 {
        let text = std::str::from_utf8(bytes).expect("atom string is not valid UTF-8");
        self.atom_table.get_or_intern(text).into_usize() as u32 + 1
    }
}

thread_local! {
    /// Thread-local file cache shared across all XkbContext instances.
    /// Survives context clones and keymap compilations within the same thread.
    static FILE_CACHE: RefCell<HashMap<String, Arc<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// Read a file from the thread-local cache, or read from disk and cache it.
pub(crate) fn read_file_cached(path: &str) -> Option<Arc<Vec<u8>>> {
    FILE_CACHE
        .with(|cache| {
            let cache = cache.borrow();
            cache.get(path).cloned()
        })
        .or_else(|| {
            let arc = Arc::new(std::fs::read(path).ok()?);
            FILE_CACHE.with(|cache| {
                cache.borrow_mut().insert(path.to_string(), arc.clone());
            });
            Some(arc)
        })
}

#[derive(Clone)]
pub(crate) struct XkbKeymap {
    pub(crate) ctx: XkbContext,
    pub(crate) format: u32,
    pub(crate) min_key_code: u32,
    pub(crate) max_key_code: u32,
    pub(crate) num_keys: u32,
    pub(crate) num_keys_low: u32,
    pub(crate) keys: Vec<XkbKey>,
    pub(crate) key_names: Vec<KeycodeMatch>,
    pub(crate) types: Vec<XkbKeyType>,
    pub(crate) mods: XkbModSet,
    pub(crate) num_groups: u32,
    pub(crate) group_names: Vec<u32>,
}

impl XkbKeymap {
    pub(crate) fn mod_get_mask(&self, name: &str) -> u32 {
        let Some(key) = self.ctx.atom_table.get(name) else {
            return 0;
        };
        let atom = key.into_usize() as u32 + 1;
        xkb_mod_name_to_index(&self.mods, atom, MOD_BOTH)
            .filter(|&idx| idx < self.mods.num_mods)
            .map(|idx| self.mods.mods[idx as usize].mapping)
            .unwrap_or(0)
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbModSet {
    pub(crate) mods: [XkbMod; 32],
    pub(crate) num_mods: u32,
    pub(crate) explicit_vmods: u32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbMod {
    pub(crate) name: u32,
    pub(crate) type_0: u32,
    pub(crate) mapping: u32,
}

pub(crate) const MOD_BOTH: u32 = 3;
pub(crate) const MOD_VIRT: u32 = 2;
pub(crate) const MOD_REAL: u32 = 1;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum XkbAction {
    #[default]
    None,
    GroupSet(XkbGroupAction),
    GroupLatch(XkbGroupAction),
    GroupLock(XkbGroupAction),
}

pub const ACTION_TYPE_GROUP_LOCK: u32 = 7;
pub const ACTION_TYPE_GROUP_LATCH: u32 = 6;
pub const ACTION_TYPE_GROUP_SET: u32 = 5;

bitflags::bitflags! {
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ActionFlags: u32 {
        const LOCK_CLEAR            = 1;
        const LATCH_TO_LOCK         = 2;
        const ABSOLUTE_SWITCH       = 32;
        const LOCK_ON_RELEASE       = 1024;
        const PENDING_COMPUTATION   = 8192;
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct XkbGroupAction {
    pub flags: ActionFlags,
    pub group: i32,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbMods {
    pub(crate) mods: u32,
    pub(crate) mask: u32,
}

#[derive(Clone, Default)]
pub(crate) struct XkbKeyType {
    pub(crate) name: u32,
    pub(crate) mods: XkbMods,
    pub(crate) num_levels: u32,
    pub(crate) entries: Vec<XkbKeyTypeEntry>,
}

#[derive(Copy, Clone)]
pub(crate) struct XkbKeyTypeEntry {
    pub(crate) level: u32,
    pub(crate) mods: XkbMods,
    pub(crate) preserve: XkbMods,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct KeycodeMatch {
    pub(crate) found: bool,
    pub(crate) low: bool,
    pub(crate) is_alias: bool,
    pub(crate) index: u32,
}

#[derive(Clone, Default)]
pub(crate) struct XkbKey {
    pub(crate) keycode: u32,
    pub(crate) name: u32,
    pub(crate) explicit_repeat: bool,
    pub(crate) explicit_vmodmap: bool,
    pub(crate) modmap: u32,
    pub(crate) vmodmap: u32,
    pub(crate) repeats: bool,
    pub(crate) out_of_range_pending_group: bool,
    pub(crate) out_of_range_group_policy: u32,
    pub(crate) out_of_range_group_number: u32,
    pub(crate) num_groups: u32,
    pub(crate) groups: Vec<XkbGroup>,
}

#[derive(Clone, Default)]
pub(crate) struct XkbGroup {
    pub(crate) explicit_actions: bool,
    pub(crate) type_idx: u32,
    pub(crate) levels: Vec<XkbLevel>,
}

#[derive(Clone, Default)]
pub(crate) struct XkbLevel {
    pub(crate) syms: Vec<u32>,
    pub(crate) action: Option<XkbAction>,
}

pub(crate) const XKB_MAX_GROUPS: u32 = 32;
pub(crate) const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;
pub(crate) const DFLT_XKB_CONFIG_EXTRA_PATH: &str = "/usr/local/etc/xkb";
pub(crate) const DFLT_XKB_CONFIG_ROOT: &str = "/usr/share/xkeyboard-config-2";
pub(crate) const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config.d";
pub(crate) const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config-2.d";
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";

pub(crate) const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub(crate) const XKB_KEYSYM_CASE_INSENSITIVE: u32 = 1;

pub(crate) const XKB_KEYCODE_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_KEYCODE_MAX: u32 = 0xffffffff_u32.wrapping_sub(1);
pub(crate) const XKB_KEYSYM_MAX: u32 = 0x1fffffff;

#[derive(Clone, Default)]
pub(crate) struct XkbComponentNames {
    pub(crate) keycodes: Vec<u8>,
    pub(crate) symbols: Vec<u8>,
    pub(crate) types: Vec<u8>,
}

pub(crate) const XKB_ATOM_NONE: u32 = 0;

pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub(crate) const _XKB_MOD_INDEX_NUM_ENTRIES: u32 = 8;
pub(crate) const XKB_OVERLAY_MAX: u8 = 8;
pub(crate) const XKB_OVERLAY_INVALID: u8 = 255;
pub(crate) const XKB_KEYCODE_MAX_CONTIGUOUS: u32 = 0xfff;
pub(crate) const XKB_LEVEL_MAX_IMPL: u32 = 2048;
pub(crate) const XKB_MAX_MODS: u32 = 32;
// ── Safe methods on XkbKeymap ──────────────────────────────────────

impl XkbKeymap {
    /// Look up a key by keycode. Safe wrapper around the old `XkbKey` function.
    #[inline]
    pub(crate) fn get_key(&self, kc: u32) -> Option<&XkbKey> {
        if kc < self.min_key_code || kc > self.max_key_code {
            None
        } else if kc < self.num_keys_low {
            Some(&self.keys[kc as usize])
        } else {
            self.keys[self.num_keys_low as usize..self.num_keys as usize]
                .binary_search_by(|key| key.keycode.cmp(&kc))
                .ok()
                .map(|i| &self.keys[self.num_keys_low as usize + i])
        }
    }

    fn key_index_by_name(&self, name: u32, aliases: bool) -> Option<usize> {
        let found = *self.key_names.get(name as usize)?;
        match (found.found, found.is_alias && aliases) {
            (true, true) => Some(self.key_names.get(found.index as usize)?.index as usize),
            (true, false) if !found.is_alias => Some(found.index as usize),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn key_by_name(&self, name: u32, aliases: bool) -> Option<&XkbKey> {
        self.key_index_by_name(name, aliases)
            .and_then(|idx| self.keys.get(idx))
    }

    #[inline]
    pub(crate) fn key_by_name_mut(&mut self, name: u32, aliases: bool) -> Option<&mut XkbKey> {
        let idx = self.key_index_by_name(name, aliases)?;
        self.keys.get_mut(idx)
    }
}

// Error codes (from xkbcommon_errors_h)
pub(crate) const XKB_KEY_NO_SYMBOL: u32 = 0;

#[derive(Copy, Clone)]
pub(crate) struct LookupEntry {
    pub(crate) name: &'static str,
    pub(crate) value: u32,
}

pub(crate) const fn lookup_entry(name: &'static str, value: u32) -> LookupEntry {
    LookupEntry { name, value }
}

// ── File type enum ──────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum FileType {
    Keycodes = 0,
    Types = 1,
    Compat = 2,
    Symbols = 3,
    Geometry = 4,
    Keymap = 5,
    Rules = 6,
}

// ── Merge mode enum ─────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(crate) enum MergeMode {
    #[default]
    Default = 0,
    Augment = 1,
    Override = 2,
    Replace = 3,
}

// ── Core AST node types ─────────────────────────────────────────────
#[derive(Clone)]
pub(crate) struct IncludeStmt {
    pub(crate) merge: MergeMode,
    pub(crate) file: String,
    pub(crate) map: String,
    pub(crate) modifier: String,
}

// ── Expression types ────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum BinaryOp {
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum UnaryOp {
    Not,
    Invert,
    Negate,
    Plus,
}

/// Expression AST node — the discriminated payload.
pub(crate) enum ExprKind {
    String(u32),
    Integer(i64),
    Float,
    Boolean(bool),
    KeyName(u32),
    KeySym(u32),
    Ident(u32),
    FieldRef {
        element: u32,
        field: u32,
        index: Option<Box<ExprKind>>,
    },
    Action {
        name: u32,
        args: Vec<ExprKind>,
    },
    ActionList {
        actions: Vec<ExprKind>,
    },
    KeysymList {
        syms: Vec<u32>,
    },
    EmptyList,
    Binary {
        op: BinaryOp,
        left: Option<Box<ExprKind>>,
        right: Option<Box<ExprKind>>,
    },
    Unary {
        op: UnaryOp,
        child: Option<Box<ExprKind>>,
    },
}

// ── Statement definition types ──────────────────────────────────────
pub(crate) struct VarDef {
    pub(crate) merge: MergeMode,
    pub(crate) name: Option<ExprKind>,
    pub(crate) value: Option<ExprKind>,
}

pub(crate) struct VModDef {
    pub(crate) merge: MergeMode,
    pub(crate) name: u32,
    pub(crate) value: Option<ExprKind>,
}

#[derive(Copy, Clone)]
pub(crate) struct KeycodeDef {
    pub(crate) merge: MergeMode,
    pub(crate) name: u32,
    pub(crate) value: i64,
}

#[derive(Copy, Clone)]
pub(crate) struct KeyAliasDef {
    pub(crate) merge: MergeMode,
    pub(crate) alias: u32,
    pub(crate) real: u32,
}

pub(crate) struct NamedVarDef {
    pub(crate) merge: MergeMode,
    pub(crate) name: u32,
    pub(crate) body: Vec<VarDef>,
}
pub(crate) struct ModMapDef {
    pub(crate) merge: MergeMode,
    pub(crate) modifier: u32,
    pub(crate) keys: Vec<ExprKind>,
}
pub(crate) struct InterpDef {
    pub(crate) def: Vec<VarDef>,
}

pub(crate) const MAP_HAS_MAP_FLAGS: u32 = 2;
pub(crate) const MAP_IS_DEFAULT: u32 = 1;

pub(crate) enum Statement {
    Include(Vec<IncludeStmt>),
    Keycode(KeycodeDef),
    KeyAlias(KeyAliasDef),
    Var(VarDef),
    KeyType(NamedVarDef),
    VMod(VModDef),
    Symbols(NamedVarDef),
    ModMap(ModMapDef),
    Unknown,
    XkbFile(XkbFile),
}

pub(crate) struct XkbFile {
    pub(crate) name: String,
    pub(crate) defs: Vec<Statement>,
    pub(crate) file_type: FileType,
    pub(crate) flags: u32,
}

// ── xkbcomp_priv types (parser/keymap info) ─────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ParseStatus {
    Success = 0,
    Recoverable = 1,
    Fatal = 2,
}

pub(crate) const PARSER_V2_LAX_FLAGS: u32 = 0;
pub(crate) const PARSER_V1_LAX_FLAGS: u32 = 16379;
pub(crate) const PARSER_NO_ILLEGAL_ACTION_FIELDS: u32 = 8192;
pub(crate) const PARSER_NO_UNKNOWN_ACTION_FIELDS: u32 = 4096;
pub(crate) const PARSER_NO_UNKNOWN_KEY_FIELDS: u32 = 1024;
pub(crate) const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: u32 = 512;
pub(crate) const PARSER_NO_UNKNOWN_TYPE_FIELDS: u32 = 32;
pub(crate) const PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS: u32 = 16;
pub(crate) const PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS: u32 = 8;
pub(crate) const PARSER_NO_FIELD_VALUE_MISMATCH: u32 = 4;
pub(crate) const PARSER_NO_FIELD_TYPE_MISMATCH: u32 = 2;
pub(crate) const PARSER_NO_UNKNOWN_STATEMENTS: u32 = 1;
pub(crate) struct PendingComputation {
    pub(crate) expr: Option<ExprKind>,
    pub(crate) computed: bool,
    pub(crate) value: u32,
}

pub(crate) struct XkbKeymapInfo<'a> {
    pub(crate) keymap: &'a mut XkbKeymap,
    pub(crate) strict: u32,
    pub(crate) features: XkbcompFeatures,
    pub(crate) lookup: XkbcompLookup,
    pub(crate) pending_computations: Vec<PendingComputation>,
}

#[derive(Copy, Clone)]
pub(crate) struct XkbcompLookup {
    pub(crate) group_index_names: [LookupEntry; 3],
}

#[derive(Copy, Clone)]
pub(crate) struct XkbcompFeatures {
    pub(crate) max_groups: u32,
    pub(crate) max_overlays: u8,
    pub(crate) group_lock_on_release: bool,
    pub(crate) overlapping_overlays: bool,
}

fn digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'F' => b - b'A' + 10,
        b'a'..=b'f' => b - b'a' + 10,
        _ => 0xff,
    }
}

fn parse_uint(s: &[u8], radix: u64, max: u64) -> (u64, i32) {
    let mut value = 0;
    for (i, &byte) in s.iter().enumerate() {
        let digit = digit(byte) as u64;
        if digit >= radix {
            return (value, i as i32);
        }
        if value > (max - digit) / radix {
            return (value, -1);
        }
        value = value * radix + digit;
    }
    (value, s.len() as i32)
}

pub(crate) fn parse_dec_u32(s: &[u8]) -> (u32, i32) {
    let (value, count) = parse_uint(s, 10, u32::MAX as u64);
    (value as u32, count)
}
pub(crate) fn parse_dec_u64(s: &[u8]) -> (u64, i32) {
    parse_uint(s, 10, u64::MAX)
}
pub(crate) fn parse_hex_u32(s: &[u8]) -> (u32, i32) {
    let (value, count) = parse_uint(s, 16, u32::MAX as u64);
    (value as u32, count)
}
fn parse_hex_u64(s: &[u8]) -> (u64, i32) {
    parse_uint(s, 16, u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::xkb_parse_string;
    use crate::xkb::keymap::xkb_context_new;

    #[test]
    fn parser_preserves_the_next_map_token() {
        let input = br#"
            xkb_symbols "first" {};
            xkb_symbols "second" {};
        "#;
        let mut ctx = xkb_context_new();

        let file = xkb_parse_string(&mut ctx, input, "second")
            .expect("second map should remain parseable");

        assert_eq!(file.name, "second");
    }
}
