use super::keymap::mod_mask_get_effective;
use super::keymap::xkb_escape_map_name;
use super::keymap::GROUP_LAST_INDEX_NAME;
use super::keysym::xkb_keysym_from_name;
use super::parser_tables::*;
pub(crate) use super::symbols::compile_compat_map;
pub(crate) use super::symbols::compile_key_types;
pub(crate) use super::symbols::compile_keycodes;
pub(crate) use super::symbols::compile_symbols;
use super::symbols::{expr_resolve_group, expr_resolve_group_mask};
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

// ── Helper functions ────────────────────────────────────────────────

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

// ── Main parser function ────────────────────────────────────────────

/// Error recovery: try to shift the error token in the current state, otherwise
/// pop states until we find one that can (which the current grammar never does).
fn recover<'a>(
    states: &mut Vec<u16>,
    values: &mut Vec<YYValue<'a>>,
    yylval: &mut YYValue<'a>,
) -> bool {
    loop {
        let state = &STATES[*states.last().unwrap() as usize];
        if let Some(Action::Shift(next)) = state.explicit_action(SYM_ERROR) {
            states.push(next);
            values.push(std::mem::replace(yylval, YYValue::None));
            return true;
        }
        if states.len() == 1 {
            return false;
        }
        states.pop();
        values.pop();
    }
}

pub(crate) fn _xkbcommon_parse<'a>(param: &mut ParserParam<'a>) -> i32 {
    let mut yychar: i32 = YYEMPTY; // lookahead symbol (internal), or YYEMPTY when none
    let mut yylval: YYValue<'a> = YYValue::None;
    let mut yyerrstatus: i32 = 0;

    let mut states = Vec::with_capacity(YYINITDEPTH);
    let mut values = Vec::with_capacity(YYINITDEPTH);
    states.push(0);
    values.push(YYValue::None);

    'main_loop: loop {
        if states.len() >= YYMAXDEPTH {
            return 2;
        }

        let yystate = *states.last().unwrap();
        let state = &STATES[yystate as usize];
        if yychar == YYEMPTY
            && (state.has_terminal_transitions() || matches!(state.default_action(), Action::Error))
        {
            yychar = _xkbcommon_lex(&mut yylval, param.scanner, param.ctx);
        }

        // Look up the action for the lookahead symbol in the current state.
        let action = (yychar >= 0)
            .then(|| state.explicit_action(yychar as Symbol))
            .flatten()
            .unwrap_or_else(|| state.default_action());

        match action {
            Action::Accept => return 0,
            Action::Shift(next) => {
                if yyerrstatus != 0 {
                    yyerrstatus -= 1;
                }
                states.push(next);
                values.push(std::mem::replace(&mut yylval, YYValue::None));
                yychar = YYEMPTY;
            }
            Action::Reduce(rule_id) => {
                let rule = &RULES[rule_id as usize];
                let yylen = rule.rhs_len() as usize;
                let mut yyval = YYValue::None;

                let top = values.len() - 1;
                let reduce_ok =
                    execute_reduction(rule_id as i32, &mut values, top, &mut yyval, param);
                if !reduce_ok {
                    states.truncate(states.len() - yylen);
                    values.truncate(values.len() - yylen);

                    yyerrstatus = 3;
                    if !recover(&mut states, &mut values, &mut yylval) {
                        return 1;
                    }
                    continue 'main_loop;
                }

                // A complete top-level map returns before lexing the next one.
                if matches!(rule_id, 2 | 3) {
                    return 0;
                }

                states.truncate(states.len() - yylen);
                values.truncate(values.len() - yylen);
                states.push(rule.next_state(*states.last().unwrap()));
                values.push(yyval);
            }
            Action::Error => {
                if yyerrstatus == 3 && yychar == END_OF_FILE {
                    return 1;
                }
                if yyerrstatus == 3 && yychar > END_OF_FILE {
                    yylval = YYValue::None;
                    yychar = YYEMPTY;
                }

                yyerrstatus = 3;
                if !recover(&mut states, &mut values, &mut yylval) {
                    return 1;
                }
                continue 'main_loop;
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

/// Execute a reduction rule. Returns true on success, false on error (YYERROR).
fn execute_reduction<'a>(
    yyn: i32,
    yyvs: &mut [YYValue<'a>],
    sp: usize,
    yyval: &mut YYValue<'a>,
    param: &mut ParserParam<'a>,
) -> bool {
    match yyn {
        2 | 3 => {
            // XkbFile: XkbCompositeMap
            param.rtrn = yyvs[sp].take_file();
            param.more_maps = param.rtrn.is_some();
            // yyval is dead here since we continue the loop; leave as None
        }
        4 => {
            // XkbFile: END_OF_FILE
            param.rtrn = None;
            *yyval = YYValue::None;
            param.more_maps = false;
        }
        5 => {
            // XkbCompositeMap: OptFlags XkbCompositeType OptMapName OBRACE XkbMapConfigList CBRACE SEMI
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
            // XkbMapConfigList: XkbMapConfigList XkbMapConfig
            let file = yyvs[sp].take_file();
            let mut list = yyvs[sp - 1].take_file_list();
            if let Some(f) = file {
                list.push(*f);
            }
            *yyval = YYValue::FileList(list);
        }
        10 => {
            // XkbMapConfigList: empty
            *yyval = YYValue::FileList(Vec::new());
        }
        11 => {
            // XkbMapConfig: OptFlags FileType OptMapName OBRACE DeclList CBRACE SEMI
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
            // Rule 22 is the `default` keyword. The other map keywords
            // (partial, hidden, alphanumeric_keys, ...) are declaration
            // metadata that the compiler ignores; they only need to keep
            // the flags word nonzero so include resolution treats the map
            // as explicitly flagged.
            *yyval = YYValue::MapFlags(if yyn == 22 {
                MAP_IS_DEFAULT
            } else {
                MAP_HAS_MAP_FLAGS
            });
        }
        29 => {
            // DeclList: DeclList Decl
            let stmt = std::mem::replace(&mut yyvs[sp], YYValue::None);
            let mut list = yyvs[sp - 1].take_stmt_list();
            if let YYValue::Stmt(s) = stmt {
                list.push(s);
            }
            *yyval = YYValue::StmtList(list);
        }
        30 => {
            // DeclList: DeclList OptMergeMode VModDecl
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
            // DeclList: empty
            *yyval = YYValue::StmtList(Vec::new());
        }
        32 => {
            // Decl: OptMergeMode VarDecl
            let merge = yyvs[sp - 1].as_merge();
            if let Some(mut var) = yyvs[sp].take_var() {
                var.merge = merge;
                *yyval = YYValue::Stmt(Statement::Var(var));
            } else {
                *yyval = YYValue::None;
            }
        }
        33 => {
            // Decl: OptMergeMode InterpretDecl
            yy_merge_decl!(yyval, yyvs, sp, Interp, Interp);
        }
        34 => {
            // Decl: OptMergeMode KeyNameDecl
            yy_merge_decl!(yyval, yyvs, sp, Keycode, Keycode);
        }
        35 => {
            // Decl: OptMergeMode KeyAliasDecl
            yy_merge_decl!(yyval, yyvs, sp, KeyAlias, KeyAlias);
        }
        36 => {
            // Decl: OptMergeMode KeyTypeDecl
            yy_merge_decl!(yyval, yyvs, sp, KeyType, KeyType);
        }
        37 => {
            // Decl: OptMergeMode SymbolsDecl
            yy_merge_decl!(yyval, yyvs, sp, Symbols, Symbols);
        }
        38 => {
            // Decl: OptMergeMode ModMapDecl
            yy_merge_decl!(yyval, yyvs, sp, ModMask, ModMap);
        }
        39 => {
            if let YYValue::GroupCompat = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::GroupCompat);
            } else {
                *yyval = YYValue::None;
            }
        }
        40 => {
            // Decl: OptMergeMode LedMapDecl
            yy_merge_decl!(yyval, yyvs, sp, LedMap, LedMap);
        }
        41 => {
            // Decl: OptMergeMode LedNameDecl
            yy_merge_decl!(yyval, yyvs, sp, LedName, LedName);
        }
        42..=44 | 93..=123 | 181 => *yyval = YYValue::None,
        45 | 46 => {
            // Decl: OptMergeMode UnknownDecl
            if let YYValue::Unknown = std::mem::replace(&mut yyvs[sp], YYValue::None) {
                *yyval = YYValue::Stmt(Statement::Unknown);
            } else {
                *yyval = YYValue::None;
            }
        }
        47 => {
            // Decl: MergeMode STRING
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
            // KeyNameDecl: KEYNAME EQUALS KeyCode SEMI
            let atom = yyvs[sp - 3].as_atom();
            let num = yyvs[sp - 1].as_num();
            *yyval = YYValue::Keycode(keycode_create(atom, num));
        }
        52 => {
            // KeyAliasDecl: ALIAS KEYNAME EQUALS KEYNAME SEMI
            let alias = yyvs[sp - 3].as_atom();
            let real = yyvs[sp - 1].as_atom();
            *yyval = YYValue::KeyAlias(key_alias_create(alias, real));
        }
        53 => {
            // VModDecl: VIRTUAL_MODS VModDefList SEMI
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
            // VModDef: Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::VMod(vmod_create(atom, None));
        }
        57 => {
            // VModDef: Ident EQUALS Expr
            let atom = yyvs[sp - 2].as_atom();
            let expr = yyvs[sp].take_expr();
            *yyval = YYValue::VMod(vmod_create(atom, expr));
        }
        58 => {
            // InterpretDecl: INTERPRET InterpretMatch OBRACE VarDeclList CBRACE SEMI
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
            let offset = 2 * usize::from(yyn == 59);
            let keysym = yyvs[sp - offset].as_keysym();
            let expr = (yyn == 59).then(|| yyvs[sp].take_expr()).flatten();
            *yyval = YYValue::Interp(InterpDef {
                merge: MergeMode::Default,
                sym: keysym,
                match_0: expr,
                def: Vec::new(),
            });
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
            // VarDeclList: empty
            *yyval = YYValue::VarList(Vec::new());
        }
        63 => {
            // KeyTypeDecl: TYPE String OBRACE VarDeclList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::KeyType(key_type_create(atom, vardefs));
        }
        64 => {
            // SymbolsDecl: KEY KEYNAME OBRACE OptSymbolsBody CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Symbols(symbols_create(atom, vardefs));
        }
        65 => {
            // OptSymbolsBody: SymbolsBody
            let list = yyvs[sp].take_var_list();
            *yyval = YYValue::VarList(list);
        }
        73 => {
            // SymbolsVarDecl: Expr
            let val = yyvs[sp].take_expr();
            *yyval = YYValue::Var(var_create(None, val));
        }
        74 | 76 | 172 => {
            // MultiKeySymOrActionList: OBRACKET MultiKeySymList CBRACKET (yylen=3)
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(ExprKind::ActionList { actions: list });
        }
        75 => {
            // MultiKeySymOrActionList: NoSymbolOrActionList OBRACKET MultiKeySymList CBRACKET COMMA (yylen=5)
            let mut list = yyvs[sp - 1].take_expr_list(); // sp-1 = MultiKeySymList = offset(-1)
            let count = yyvs[sp - 3].as_no_sym_or_action_list(); // sp-3 = NoSymbolOrActionList = offset(-3)
                                                                 // Prepend 'count' NoSymbol keysym lists
            let mut prepended: Vec<ExprKind> = Vec::new();
            for _ in 0..count {
                prepended.push(expr_create_key_sym_list(XKB_KEY_NO_SYMBOL));
            }
            prepended.append(&mut list);
            *yyval = YYValue::Expr(ExprKind::ActionList { actions: prepended });
        }
        77 => {
            // MultiKeySymOrActionList: NoSymbolOrActionList OBRACKET MultiActionList CBRACKET COMMA (yylen=5)
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
            // NoSymbolOrActionList: NoSymbol (produces EmptyList expr)
            *yyval = YYValue::Expr(ExprKind::EmptyList);
        }
        79 => {
            // NoSymbolOrActionList: NoSymbolOrActionList COMMA NoSymbol COMMA (yylen=4)
            let prev = yyvs[sp - 3].as_no_sym_or_action_list();
            *yyval = YYValue::NoSymbolOrActionList(prev + 1);
        }
        80 => {
            // NoSymbolOrActionList: ... (yylen=2)
            *yyval = YYValue::NoSymbolOrActionList(1);
        }
        81 => {
            // NoSymbolOrActionList: empty
            *yyval = YYValue::NoSymbolOrActionList(0);
        }
        82 => {
            // GroupCompatDecl: GROUP Integer EQUALS Expr SEMI
            *yyval = YYValue::GroupCompat;
        }
        83 => {
            // ModMapDecl: MODIFIER_MAP Ident OBRACE KeyOrKeySymList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let list = yyvs[sp - 2].take_expr_list();
            *yyval = YYValue::ModMask(mod_map_create(atom, list));
        }
        84 | 148 | 170 | 187 => {
            // KeyOrKeySymList: KeyOrKeySymList COMMA KeyOrKeySym
            yy_list_push(yyval, yyvs, sp, 2);
        }
        85 | 149 | 169 | 171 | 189 => {
            // KeyOrKeySymList: KeyOrKeySym
            yy_list_single(yyval, yyvs, sp);
        }
        86 | 185 => {
            // KeyOrKeySym: KEYNAME
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::KeyName(atom));
        }
        87 => {
            // KeyOrKeySym: KeySym
            let keysym = yyvs[sp].as_keysym();
            *yyval = YYValue::Expr(ExprKind::KeySym(keysym));
        }
        88 => {
            // LedMapDecl: INDICATOR String OBRACE VarDeclList CBRACE SEMI
            let atom = yyvs[sp - 4].as_atom();
            let vardefs = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::LedMap(led_map_create(atom, vardefs));
        }
        89 | 90 => {
            // LedNameDecl: INDICATOR Integer EQUALS Expr SEMI
            let num = yyvs[sp - 3].as_num();
            let expr = yyvs[sp - 1].take_expr();
            *yyval = YYValue::LedName(led_name_create(num, expr));
        }
        91 => {
            // UnknownDecl: Ident Lhs EQUALS Expr SEMI
            // Drop expr values (geometry not supported)
            let _ = yyvs[sp - 3].take_expr();
            let _ = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Unknown;
        }
        92 => {
            // UnknownCompoundStatementDecl: Ident Lhs OBRACE VarDeclList CBRACE SEMI
            let _ = yyvs[sp - 4].take_expr();
            let _ = yyvs[sp - 2].take_var_list();
            *yyval = YYValue::Unknown;
        }
        // Geometry is parsed for compatibility but has no semantic representation.
        124..=127 | 209 => *yyval = YYValue::Num(0),
        // FieldSpec / Element rules 128-140
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
        // MergeMode rules 141-147
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
        // ExprList rules 148-150
        150 => {
            // ExprList: empty
            *yyval = YYValue::ExprList(Vec::new());
        }
        // Expr rules 151-165
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
            // Expr: Term
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
            // Term: Action OPAREN ExprList CPAREN
            let name = yyvs[sp - 3].as_atom();
            let list = yyvs[sp - 1].take_expr_list();
            *yyval = YYValue::Expr(ExprKind::Action { name, args: list });
        }
        165 | 194 => {
            // Term: OPAREN Expr CPAREN
            *yyval = std::mem::replace(&mut yyvs[sp - 1], YYValue::None);
        }
        // MultiActionList rules 166-167
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
            // MultiActionList: ActionList (initial single element)
            let actions_expr_list = yyvs[sp].take_expr_list();
            let action_list_expr = ExprKind::ActionList {
                actions: actions_expr_list,
            };
            *yyval = YYValue::ExprList(vec![action_list_expr]);
        }
        174 => {
            // ActionList: empty (yylen=0 means nothing on stack)
            *yyval = YYValue::Expr(ExprKind::ActionList {
                actions: Vec::new(),
            });
        }
        176 => {
            // Lhs: Ident
            let atom = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::Ident(atom));
        }
        177 => {
            // Lhs: Ident DOT FieldSpec
            let element = yyvs[sp - 2].as_atom();
            let field = yyvs[sp].as_atom();
            *yyval = YYValue::Expr(ExprKind::FieldRef { element, field });
        }
        178 => {
            // Lhs: Ident OBRACKET Expr CBRACKET
            let field = yyvs[sp - 3].as_atom();
            let entry = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Expr(ExprKind::ArrayRef {
                element: XKB_ATOM_NONE,
                field,
                entry: entry.map(Box::new),
            });
        }
        179 => {
            // Lhs: Ident DOT Ident OBRACKET Expr CBRACKET
            let element = yyvs[sp - 5].as_atom();
            let field = yyvs[sp - 3].as_atom();
            let entry = yyvs[sp - 1].take_expr();
            *yyval = YYValue::Expr(ExprKind::ArrayRef {
                element,
                field,
                entry: entry.map(Box::new),
            });
        }
        // Terminal rules 182-185
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
            // MultiKeySymList: MultiKeySymList COMMA KeySymList
            let keysym = yyvs[sp].as_keysym();
            let expr = expr_create_key_sym_list(keysym);
            let mut list = yyvs[sp - 2].take_expr_list();
            list.push(expr);
            *yyval = YYValue::ExprList(list);
        }
        188 => {
            // MultiKeySymList: KeySymList (keysym)
            let keysym = yyvs[sp].as_keysym();
            let expr = expr_create_key_sym_list(keysym);
            *yyval = YYValue::ExprList(vec![expr]);
        }
        190 => {
            // NonEmptyKeySyms: NonEmptyKeySyms COMMA KeySym
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
            // KeySyms: KeySym
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
            // KeySymList: empty → NoSymbol
            *yyval = YYValue::Expr(expr_create_key_sym_list(XKB_KEY_NO_SYMBOL));
        }
        // KeySym rules 198-203
        198 => {
            // KeySymLit: KeySym (passthrough)
            *yyval = YYValue::Keysym(yyvs[sp].as_keysym());
        }
        199 => {
            // KeySym: STRING → parse string as keysym
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
            // KeySym: SECTION
            *yyval = YYValue::Keysym(XKB_KEY_SECTION as u32);
        }
        202 => {
            // KeySym: DECIMAL_DIGIT
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
        // SignedNumber / Number rules 204-208
        204 => {
            *yyval = YYValue::Num(-yyvs[sp].as_num());
        }
        205..=208 | 210..=213 => {
            *yyval = YYValue::Num(yyvs[sp].as_num());
        }
        // Ident 214
        214 => {
            let sval = yyvs[sp].as_sval();
            *yyval = YYValue::Atom(param.ctx.atom_intern(sval.data));
        }
        215 => {
            // Ident: DEFAULT
            *yyval = YYValue::Atom(param.ctx.atom_intern(b"default"));
        }
        // String 216
        216 => {
            // String: STRING → intern as atom
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Atom(param.ctx.atom_intern(s.as_bytes()));
        }
        // OptMapName / MapName 217-219
        217 | 219 => {
            // MapName: STRING
            let s = yyvs[sp].take_str();
            *yyval = YYValue::Str(s);
        }
        218 => {
            // OptMapName: empty
            *yyval = YYValue::Str(String::new());
        }

        _ => {}
    }
    true
}

// ── Public API ──────────────────────────────────────────────────────

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
    fn key_type_create(name: u32, body: Vec<VarDef>) -> KeyTypeDef;
    fn symbols_create(name: u32, body: Vec<VarDef>) -> SymbolsDef;
    fn mod_map_create(modifier: u32, keys: Vec<ExprKind>) -> ModMapDef;
    fn led_map_create(name: u32, body: Vec<VarDef>) -> LedMapDef;
    fn led_name_create(ndx: i64, name: Option<ExprKind>) -> LedNameDef;
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
        (FileType::Compat, &kkctgs.compatibility),
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
    KeyType(KeyTypeDef),
    Symbols(SymbolsDef),
    ModMask(ModMapDef),
    GroupCompat,
    LedMap(LedMapDef),
    LedName(LedNameDef),
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
use super::keymap::{xkb_context_include_path_get, xkb_context_num_include_paths};

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
pub(crate) fn find_file_in_xkb_path(
    ctx: &mut XkbContext,
    name: &str,
    type_0: FileType,
    offset: &mut u32,
) -> Option<(std::sync::Arc<Vec<u8>>, String)> {
    let type_dir = directory_for_include(type_0);
    for i in *offset..xkb_context_num_include_paths(ctx) {
        let path = format!(
            "{}/{}/{}",
            xkb_context_include_path_get(ctx, i),
            type_dir,
            name
        );
        if path.len() < 4096 {
            if let Some(data) = read_file_cached(&path) {
                *offset = i;
                return Some((data, path));
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
) -> Option<(std::sync::Arc<Vec<u8>>, String)> {
    if name.starts_with('/') {
        if *offset == 0 {
            read_file_cached(name).map(|data| (data, name.to_owned()))
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
    let stmt_file = match expand_path_str(&stmt.file, file_type) {
        Err(()) => return None,
        Ok(Some(expanded)) => expanded,
        Ok(None) => stmt.file.clone(),
    };
    let expanded = stmt_file != stmt.file;

    let mut offset = 0;
    let mut candidate = None;
    while let Some((file_data, _)) =
        find_include_file(ctx, &stmt_file, file_type, expanded, &mut offset)
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
#[inline]
fn compute_effective_mask(keymap: &XkbKeymap, mods: &mut XkbMods) {
    let unknown_mods: u32 = !((1_u64 << keymap.mods.num_mods).wrapping_sub(1_u64) as u32);
    mods.mask = mod_mask_get_effective(keymap, mods.mods) | mods.mods & unknown_mods;
}
/// Version that takes the mod_set separately to allow calling on fields of keymap.
#[inline]
fn compute_effective_mask_with(mod_set: &XkbModSet, mods: &mut XkbMods) {
    let unknown_mods: u32 = !((1_u64 << mod_set.num_mods).wrapping_sub(1_u64) as u32);
    // Inline mod_mask_get_effective logic
    let mut mask: u32 = mods.mods & MOD_REAL_MASK_ALL;
    for i in _XKB_MOD_INDEX_NUM_ENTRIES..mod_set.num_mods {
        if mods.mods & (1 << i) != 0 {
            mask |= mod_set.mods[i as usize].mapping;
        }
    }
    mods.mask = mask | mods.mods & unknown_mods;
}
fn update_action_mods(keymap: &XkbKeymap, act: &mut XkbAction, modmap: u32) {
    match act {
        XkbAction::ModSet(m) | XkbAction::ModLatch(m) | XkbAction::ModLock(m) => {
            if m.flags.contains(ActionFlags::MODS_LOOKUP_MODMAP) {
                m.mods.mods = modmap;
            }
            compute_effective_mask(keymap, &mut m.mods);
        }
        _ => {}
    }
}
fn default_interpret() -> XkbSymInterpret {
    XkbSymInterpret {
        sym: XKB_KEY_NO_SYMBOL,
        match_0: MATCH_ANY_OR_NONE,
        mods: 0,
        virtual_mod: DEFAULT_INTERPRET_VMOD,
        level_one_only: false,
        repeat: DEFAULT_INTERPRET_KEY_REPEAT != 0,
        actions: Vec::new(),
    }
}
/// Pre-computed index mapping keysym → matching interpret indices (sorted).
/// Speeds up find_interp_for_key by avoiding an O(N_interps) scan per key/group/level/sym.
struct InterpIndex {
    /// Indices of wildcard interprets (sym == XKB_KEY_NO_SYMBOL), sorted.
    wildcards: Vec<usize>,
    /// Indices grouped by exact sym match, each group sorted.
    by_sym: std::collections::HashMap<u32, Vec<usize>>,
}

fn build_interp_index(interps: &[XkbSymInterpret]) -> InterpIndex {
    let mut wildcards = Vec::new();
    let mut by_sym: std::collections::HashMap<u32, Vec<usize>> = std::collections::HashMap::new();
    for (i, interp) in interps.iter().enumerate() {
        if interp.sym == XKB_KEY_NO_SYMBOL {
            wildcards.push(i);
        } else {
            by_sym.entry(interp.sym).or_default().push(i);
        }
    }
    InterpIndex { wildcards, by_sym }
}

/// Returns indices into the compiler-local interpretations, or `usize::MAX` for defaults.
fn find_interp_for_key(
    keymap: &mut XkbKeymap,
    sym_interprets: &[XkbSymInterpret],
    key_idx: usize,
    group: u32,
    level: u32,
    interp_indices: &mut Vec<usize>,
    interp_index: &InterpIndex,
) -> bool {
    let syms_ref = keymap.keys[key_idx]
        .groups
        .get(group as usize)
        .and_then(|group| group.levels.get(level as usize))
        .map_or(&[][..], |level| level.syms.as_slice());

    if syms_ref.is_empty() {
        return false;
    }
    // Copy syms to stack to release borrow on keymap (most keys have 1-2 syms)
    let mut syms_buf = [0u32; 8];
    let num_syms = syms_ref.len().min(8);
    syms_buf[..num_syms].copy_from_slice(&syms_ref[..num_syms]);
    let syms = &syms_buf[..num_syms];
    let key_modmap = keymap.keys[key_idx].modmap;
    for &cur_sym in syms {
        let mut candidates = interp_index.wildcards.clone();
        if let Some(exact) = interp_index.by_sym.get(&cur_sym) {
            candidates.extend(exact);
            candidates.sort_unstable();
        }
        let mut selected = None;
        for i in candidates {
            let interp = &sym_interprets[i];
            let mods = if interp.level_one_only && level != 0 {
                0
            } else {
                key_modmap
            };
            let matched = match interp.match_0 {
                0 => interp.mods & mods == 0,
                1 => mods == 0 || interp.mods & mods != 0,
                2 => interp.mods & mods != 0,
                3 => interp.mods & mods == interp.mods,
                4 => interp.mods == mods,
                _ => false,
            };
            if matched {
                selected = Some(i);
                break;
            }
        }
        match selected {
            Some(i)
                if sym_interprets[i].sym == XKB_KEY_NO_SYMBOL && interp_indices.contains(&i) =>
            {
                interp_indices.push(usize::MAX)
            }
            Some(i) => interp_indices.push(i),
            None => interp_indices.push(usize::MAX),
        }
    }
    true
}
fn apply_interps_to_key(
    keymap: &mut XkbKeymap,
    sym_interprets: &[XkbSymInterpret],
    key_idx: usize,
    interp_index: &InterpIndex,
) {
    let mut vmodmap: u32 = 0;
    let mut interp_indices: Vec<usize> = Vec::with_capacity(4);
    let mut actions: Vec<XkbAction> = Vec::with_capacity(4);
    let num_groups = keymap.keys[key_idx].num_groups;
    for group in 0..num_groups {
        if !keymap.keys[key_idx].groups[group as usize].explicit_actions {
            let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], group);
            for level in 0..num_levels {
                interp_indices.clear();
                let found: bool = find_interp_for_key(
                    keymap,
                    sym_interprets,
                    key_idx,
                    group,
                    level,
                    &mut interp_indices,
                    interp_index,
                );
                if found {
                    let default_interp = default_interpret();
                    for &idx in interp_indices.iter() {
                        let interp = if idx == usize::MAX {
                            &default_interp
                        } else {
                            &sym_interprets[idx]
                        };
                        if group == 0
                            && level == 0
                            && !keymap.keys[key_idx].explicit_repeat
                            && interp.repeat
                        {
                            keymap.keys[key_idx].repeats = true;
                        }
                        if (group == 0 && level == 0 || !interp.level_one_only)
                            && interp.virtual_mod != XKB_MOD_INVALID
                        {
                            vmodmap |= 1 << interp.virtual_mod;
                        }
                        actions.extend_from_slice(&interp.actions);
                    }
                    if (actions.len() as u32 != 0) as i64 > MAX_ACTIONS_PER_LEVEL as i64 {
                        actions.truncate(MAX_ACTIONS_PER_LEVEL as usize);
                    }
                    keymap.keys[key_idx].groups[group as usize].levels[level as usize].actions =
                        std::mem::take(&mut actions);
                }
            }
        }
    }
    if !keymap.keys[key_idx].explicit_vmodmap {
        keymap.keys[key_idx].vmodmap = vmodmap;
    }
}
fn action_category(action: &XkbAction) -> u8 {
    match action {
        XkbAction::ModSet(_) | XkbAction::ModLatch(_) | XkbAction::ModLock(_) => 1,
        XkbAction::GroupSet(_) | XkbAction::GroupLatch(_) | XkbAction::GroupLock(_) => 2,
        _ => 0,
    }
}
fn check_multiple_actions_categories(keymap: &mut XkbKeymap, key_idx: usize) {
    let num_groups = keymap.keys[key_idx].num_groups;
    for g in 0..num_groups as usize {
        let num_levels = keymap.key_num_levels(&keymap.keys[key_idx], g as u32);
        for l in 0..num_levels as usize {
            let level: &mut XkbLevel = &mut keymap.keys[key_idx].groups[g].levels[l];
            if level.actions.len() > 1 {
                for i in 0..level.actions.len() {
                    let category = action_category(&level.actions[i]);
                    if category != 0 {
                        for j in (i + 1)..level.actions.len() {
                            let same_action = std::mem::discriminant(&level.actions[i])
                                == std::mem::discriminant(&level.actions[j]);
                            if same_action || category == action_category(&level.actions[j]) {
                                level.actions[j] = XkbAction::None;
                            }
                        }
                    }
                }
            }
        }
    }
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
        info.lookup.group_mask_names[3] =
            lookup_entry(GROUP_LAST_INDEX_NAME, 1 << num_groups.wrapping_sub(1));
        if update_pending_sym_interpret_actions(info).is_err() {
            return false;
        }
    }
    let interp_index = build_interp_index(&info.sym_interprets);
    for ki in 0..info.keymap.num_keys as usize {
        apply_interps_to_key(info.keymap, &info.sym_interprets, ki, &interp_index);
        check_multiple_actions_categories(info.keymap, ki);
    }
    update_mod_mappings(info);
    compute_type_entry_masks(info);
    if update_key_action_fields(info, pending_computations).is_err() {
        return false;
    }
    for led in &mut info.keymap.leds[..info.keymap.num_leds as usize] {
        compute_effective_mask_with(&info.keymap.mods, &mut led.mods);
    }
    if pending_computations && resolve_pending_led_groups(info).is_err() {
        return false;
    }
    true
}

fn update_pending_sym_interpret_actions(info: &mut XkbKeymapInfo<'_>) -> Result<(), ()> {
    for i in 0..info.sym_interprets.len() {
        for a in 0..info.sym_interprets[i].actions.len() {
            let mut action = info.sym_interprets[i].actions[a];
            if !update_pending_action_fields(info, &mut action) {
                return Err(());
            }
            info.sym_interprets[i].actions[a] = action;
        }
    }
    Ok(())
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
        let key_num_groups = info.keymap.keys[ki as usize].num_groups;
        let key_modmap = info.keymap.keys[ki as usize].modmap;
        for i_1 in 0..key_num_groups {
            let num_levels = {
                let key = &info.keymap.keys[ki as usize];
                info.keymap.types[key.groups[i_1 as usize].type_idx as usize].num_levels
            };
            for j_0 in 0..num_levels {
                let num_actions = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                    [j_0 as usize]
                    .actions
                    .len();
                for k in 0..num_actions {
                    let mut act = info.keymap.keys[ki as usize].groups[i_1 as usize].levels
                        [j_0 as usize]
                        .actions[k];
                    update_action_mods(&*info.keymap, &mut act, key_modmap);
                    if pending_computations && !update_pending_action_fields(info, &mut act) {
                        return Err(());
                    }
                    info.keymap.keys[ki as usize].groups[i_1 as usize].levels[j_0 as usize]
                        .actions[k] = act;
                }
            }
        }
    }
    Ok(())
}

fn resolve_pending_led_groups(info: &mut XkbKeymapInfo<'_>) -> Result<(), ()> {
    for led_idx in 0..info.keymap.num_leds {
        if info.keymap.leds[led_idx as usize].pending_groups {
            let groups_idx = info.keymap.leds[led_idx as usize].groups as usize;
            if !info.pending_computations[groups_idx].computed {
                let expr_box = info.pending_computations[groups_idx].expr.take().unwrap();
                let mut mask: u32 = 0;
                let resolved = expr_resolve_group_mask(info, &expr_box, &mut mask, &mut false);
                info.pending_computations[groups_idx].expr = Some(expr_box);
                if !resolved {
                    return Err(());
                }
                info.pending_computations[groups_idx].computed = true;
                info.pending_computations[groups_idx].value = mask;
            }
            let value = info.pending_computations[groups_idx].value;
            info.keymap.leds[led_idx as usize].pending_groups = false;
            info.keymap.leds[led_idx as usize].groups = value;
        }
    }
    Ok(())
}
static COMPILE_FILE_FNS: [CompileFileFn; 4] = [
    compile_keycodes,
    compile_key_types,
    compile_compat_map,
    compile_symbols,
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
    let km_flags = keymap.flags;
    let km_num_groups = keymap.num_groups;
    let mut info = XkbKeymapInfo {
        keymap,
        strict: (if km_format == XKB_KEYMAP_FORMAT_TEXT_V1 {
            if km_flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
                PARSER_V1_STRICT_FLAGS as i32
            } else {
                PARSER_V1_LAX_FLAGS as i32
            }
        } else if km_flags & XKB_KEYMAP_COMPILE_STRICT_MODE != 0 {
            PARSER_V2_STRICT_FLAGS as i32
        } else {
            PARSER_V2_LAX_FLAGS as i32
        }) as u32,
        features: XkbcompFeatures {
            max_groups: XKB_MAX_GROUPS,
            max_overlays: XKB_OVERLAY_MAX,
            controls_name_offset: (if km_format == XKB_KEYMAP_FORMAT_TEXT_V1 {
                7
            } else {
                0
            }),
            group_lock_on_release: km_format >= XKB_KEYMAP_FORMAT_TEXT_V2,
            mods_unlock_on_press: km_format >= XKB_KEYMAP_FORMAT_TEXT_V2,
            mods_latch_on_press: km_format >= XKB_KEYMAP_FORMAT_TEXT_V2,
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
            group_mask_names: [
                lookup_entry("none", 0),
                lookup_entry("first", 0x1_u32),
                lookup_entry("all", XKB_ALL_GROUPS as u32),
                lookup_entry(
                    if km_num_groups != 0 {
                        GROUP_LAST_INDEX_NAME
                    } else {
                        ""
                    },
                    if km_num_groups != 0 && km_num_groups <= XKB_MAX_GROUPS {
                        1 << km_num_groups.wrapping_sub(1_u32)
                    } else {
                        0_u32
                    },
                ),
                lookup_entry("", 0),
            ],
        },
        pending_computations: Vec::new(),
        sym_interprets: Vec::new(),
    };
    for (type_0, compile) in COMPILE_FILE_FNS.into_iter().enumerate() {
        let file_arg: Option<&mut XkbFile> = file_indices[type_0].map(|idx| {
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
    let ok_0: bool = update_derived_keymap_fields(&mut info);
    if ok_0 {
        for key in &mut info.keymap.keys {
            for group in &mut key.groups {
                for level in &mut group.levels {
                    level.actions = Vec::new();
                }
            }
        }
    }
    ok_0
}
/// Index-based sval for scanner input. Used in rules to avoid
/// lifetime issues across include boundaries. Reconstruct sval via as_sval().
#[derive(Copy, Clone, Default)]
pub(crate) struct SvalIdx {
    start: usize,
    end: usize,
}
impl SvalIdx {
    const EMPTY: SvalIdx = SvalIdx { start: 0, end: 0 };
    #[inline]
    fn as_sval<'a>(&self, input: &'a [u8]) -> Sval<'a> {
        if self.start >= self.end || self.start >= input.len() {
            Sval { data: &[] }
        } else {
            Sval {
                data: &input[self.start..self.end.min(input.len())],
            }
        }
    }
    #[inline]
    fn len(&self) -> usize {
        self.end - self.start
    }
}

pub(crate) struct Matcher<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    pub(crate) rmlvo: RuleNames<'a>,
    pub(crate) val: SvalIdx,
    pub(crate) groups: Vec<Group>,
    pub(crate) mapping: Mapping,
    pub(crate) rule: Rule,
    pub(crate) pending_kccgst: [Vec<(u32, Vec<u8>)>; 5],
    pub(crate) kccgst: [Vec<u8>; 5],
}
pub(crate) const _KCCGST_NUM_ENTRIES: u32 = 5;
pub(crate) const KCCGST_GEOMETRY: u32 = 4;
#[derive(Copy, Clone, Default)]
pub(crate) struct Rule {
    pub(crate) mlvo_value_at_pos: [SvalIdx; 4],
    pub(crate) match_type_at_pos: [u32; 4],
    pub(crate) kccgst_value_at_pos: [SvalIdx; 5],
    pub(crate) num_mlvo_values: u8,
    pub(crate) num_kccgst_values: u8,
    pub(crate) skip: bool,
}
pub(crate) const MLVO_MATCH_GROUP: u32 = 5;
pub(crate) const MLVO_MATCH_WILDCARD_ANY: u32 = 4;
pub(crate) const MLVO_MATCH_WILDCARD_SOME: u32 = 3;
pub(crate) const MLVO_MATCH_WILDCARD_NONE: u32 = 2;
pub(crate) const MLVO_MATCH_WILDCARD_LEGACY: u32 = 1;
pub(crate) const MLVO_MATCH_NORMAL: u32 = 0;
#[derive(Copy, Clone, Default)]
pub(crate) struct Mapping {
    pub(crate) mlvo_at_pos: [u32; 4],
    pub(crate) num_mlvo: u8,
    pub(crate) defined_mlvo_mask: u8,
    pub(crate) layout: LayoutIdx,
    pub(crate) active_or_candidates_mask: u32,
    pub(crate) kccgst_at_pos: [u32; 5],
    pub(crate) num_kccgst: u8,
    pub(crate) defined_kccgst_mask: u8,
}
#[derive(Copy, Clone)]
pub(crate) enum LayoutIdx {
    Single {
        layout_idx: u32,
        variant_idx: u32,
    },
    Range {
        layout_idx_min: u32,
        layout_idx_max: u32,
    },
    Index {
        layout_idx_min: u32,
    },
}
impl Default for LayoutIdx {
    fn default() -> Self {
        LayoutIdx::Single {
            layout_idx: 0,
            variant_idx: 0,
        }
    }
}
pub(crate) const _MLVO_NUM_ENTRIES: u32 = 4;
pub(crate) const MLVO_OPTION: u32 = 3;
pub(crate) const MLVO_VARIANT: u32 = 2;
pub(crate) const MLVO_LAYOUT: u32 = 1;
pub(crate) const MLVO_MODEL: u32 = 0;
#[derive(Clone)]
pub(crate) struct Group {
    pub(crate) name: Vec<u8>,
    pub(crate) elements: Vec<Vec<u8>>,
}
#[derive(Clone, Default)]
pub(crate) struct RuleNames<'a> {
    pub(crate) model: MatchedSval<'a>,
    pub(crate) layouts: Vec<MatchedSval<'a>>,
    pub(crate) variants: Vec<MatchedSval<'a>>,
    pub(crate) options: Vec<MatchedSval<'a>>,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct MatchedSval<'a> {
    pub(crate) sval: Sval<'a>,
    pub(crate) matched: bool,
    pub(crate) layout: u32,
}
pub(crate) const TOK_ERROR: u32 = 11;
pub(crate) const TOK_INCLUDE: u32 = 10;
pub(crate) const TOK_WILD_CARD_ANY: u32 = 9;
pub(crate) const TOK_WILD_CARD_SOME: u32 = 8;
pub(crate) const TOK_WILD_CARD_NONE: u32 = 7;
pub(crate) const TOK_WILD_CARD_STAR: u32 = 6;
pub(crate) const TOK_EQUALS: u32 = 5;
pub(crate) const TOK_BANG: u32 = 4;
pub(crate) const TOK_GROUP_NAME: u32 = 3;
pub(crate) const TOK_IDENTIFIER: u32 = 2;
pub(crate) const TOK_END_OF_LINE: u32 = 1;
pub(crate) const TOK_END_OF_FILE: u32 = 0;
pub(crate) const LAYOUT_INDEX_FIRST: u32 = 4294967292;
pub(crate) const LAYOUT_INDEX_SINGLE: u32 = 4294967291;
pub(crate) const LAYOUT_INDEX_ANY: u32 = 4294967294;
pub(crate) const LAYOUT_INDEX_LATER: u32 = 4294967293;

impl<'a> Matcher<'a> {
    fn new(ctx: &'a mut XkbContext) -> Self {
        Matcher {
            ctx,
            rmlvo: RuleNames::default(),
            val: SvalIdx::default(),
            groups: Vec::new(),
            mapping: Mapping::default(),
            rule: Rule::default(),
            pending_kccgst: std::array::from_fn(|_| Vec::new()),
            kccgst: std::array::from_fn(|_| Vec::new()),
        }
    }
}
pub(crate) const WILDCARD_MATCH_ALL: u32 = 1;
pub(crate) const WILDCARD_MATCH_NONEMPTY: u32 = 0;
pub(crate) const MAX_INCLUDE_DEPTH: i32 = 5_i32;
#[inline]
fn is_ident(ch: u8) -> bool {
    ch.is_ascii_graphic() && ch != b'\\'
}
fn lex(s: &mut Scanner, val: &mut SvalIdx) -> u32 {
    loop {
        while s.chr(b' ') as i32 != 0 || s.chr(b'\t') as i32 != 0 || s.chr(b'\r') as i32 != 0 {}
        if s.str_match(b"//") {
            s.skip_to_eol();
        }
        if s.eol() {
            while s.eol() {
                s.next_byte();
            }
            return TOK_END_OF_LINE;
        }
        if !s.chr(b'\\') {
            break;
        }
        s.chr(b'\r');
        if !s.eol() {
            return TOK_ERROR;
        }
        s.next_byte();
    }
    if s.eof() {
        return TOK_END_OF_FILE;
    }
    s.token_pos = s.pos;
    if s.chr(b'!') {
        return TOK_BANG;
    }
    if s.chr(b'=') {
        return TOK_EQUALS;
    }
    if s.chr(b'*') {
        return TOK_WILD_CARD_STAR;
    }
    if s.str_match(b"<none>") {
        return TOK_WILD_CARD_NONE;
    }
    if s.str_match(b"<some>") {
        return TOK_WILD_CARD_SOME;
    }
    if s.str_match(b"<any>") {
        return TOK_WILD_CARD_ANY;
    }
    if s.chr(b'$') {
        *val = SvalIdx {
            start: s.pos,
            end: s.pos,
        };
        while is_ident(s.peek()) {
            s.next_byte();
            val.end += 1;
        }
        if val.len() == 0 {
            return TOK_ERROR;
        }
        return TOK_GROUP_NAME;
    }
    if s.str_match(b"include") {
        return TOK_INCLUDE;
    }
    if is_ident(s.peek()) {
        *val = SvalIdx {
            start: s.pos,
            end: s.pos,
        };
        while is_ident(s.peek()) {
            s.next_byte();
            val.end += 1;
        }
        return TOK_IDENTIFIER;
    }
    TOK_ERROR
}
static RULES_MLVO_SVALS: [&[u8]; 4] = [b"model", b"layout", b"variant", b"option"];
static RULES_KCCGST_SVALS: [&[u8]; 5] = [b"keycodes", b"types", b"compat", b"symbols", b"geometry"];
pub(crate) const OPTIONS_MATCH_ALL_GROUPS: u32 = XKB_MAX_GROUPS;
fn strip_spaces<'a>(v: Sval<'a>) -> Sval<'a> {
    Sval {
        data: v.data.trim_ascii(),
    }
}

fn split_comma_separated_mlvo(mlvo: u32, bytes: &[u8]) -> Vec<MatchedSval<'_>> {
    if bytes.is_empty() {
        return vec![MatchedSval::default()];
    }
    let bytes = if bytes.last() == Some(&b',') {
        &bytes[..bytes.len() - 1]
    } else {
        bytes
    };
    bytes
        .split(|&byte| byte == b',')
        .map(|part| {
            let bang = part.iter().position(|&byte| byte == b'!');
            let mut value = MatchedSval {
                sval: strip_spaces(Sval {
                    data: &part[..bang.unwrap_or(part.len())],
                }),
                layout: OPTIONS_MATCH_ALL_GROUPS,
                matched: false,
            };
            if let Some(bang) = bang {
                let suffix = &part[bang + 1..];
                let (layout, count) = parse_dec_u32(suffix);
                if count > 0
                    && count as usize == suffix.len()
                    && (1..=XKB_MAX_GROUPS).contains(&layout)
                    && mlvo == MLVO_OPTION
                {
                    value.layout -= 1;
                }
            }
            value
        })
        .collect()
}
fn matcher_new_from_names<'a>(ctx: &'a mut XkbContext, rmlvo: &'a XkbRuleNames) -> Matcher<'a> {
    let mut m = Matcher::new(ctx);
    m.rmlvo.model.sval = Sval {
        data: rmlvo.model.as_bytes(),
    };
    m.rmlvo.model.layout = OPTIONS_MATCH_ALL_GROUPS;
    m.rmlvo.layouts = split_comma_separated_mlvo(MLVO_LAYOUT, rmlvo.layout.as_bytes());
    m.rmlvo.variants = split_comma_separated_mlvo(MLVO_VARIANT, rmlvo.variant.as_bytes());
    m.rmlvo.options = split_comma_separated_mlvo(MLVO_OPTION, rmlvo.options.as_bytes());
    m.rmlvo
        .variants
        .resize(m.rmlvo.layouts.len(), MatchedSval::default());
    m
}
fn matcher_include(m: &mut Matcher<'_>, include_depth: u32, inc: Sval) {
    if include_depth >= MAX_INCLUDE_DEPTH as u32 {
        return;
    }
    let inc_str = std::str::from_utf8(inc.data).unwrap_or("");
    let stmt_file: String = match expand_path_str(inc_str, FileType::Rules) {
        Err(()) => return,
        Ok(Some(expanded)) => expanded,
        Ok(None) => inc_str.to_string(),
    };
    let expanded = stmt_file != inc_str;

    let mut offset: u32 = 0;
    while let Some((file_data, _)) =
        find_include_file(m.ctx, &stmt_file, FileType::Rules, expanded, &mut offset)
    {
        if read_rules_file(m, include_depth + 1, &file_data) {
            return;
        }
        offset += 1;
    }
}
fn matcher_mapping_start_new(m: &mut Matcher) {
    for i in 0.._MLVO_NUM_ENTRIES as usize {
        m.mapping.mlvo_at_pos[i] = _MLVO_NUM_ENTRIES;
    }
    for i_0 in 0.._KCCGST_NUM_ENTRIES as usize {
        m.mapping.kccgst_at_pos[i_0] = _KCCGST_NUM_ENTRIES;
    }
    m.mapping.layout = LayoutIdx::Single {
        layout_idx: XKB_LAYOUT_INVALID,
        variant_idx: XKB_LAYOUT_INVALID,
    };
    m.mapping.num_kccgst = 0_u8;
    m.mapping.num_mlvo = m.mapping.num_kccgst;
    m.mapping.defined_mlvo_mask = 0_u8;
    m.mapping.defined_kccgst_mask = 0_u8;
    m.mapping.active_or_candidates_mask = 1_u32;
}
fn parse_layout_int_index(s: &[u8], out: &mut u32) -> i32 {
    // s starts with '[', parse integer between brackets
    if s.len() < 3 {
        return -1_i32;
    }
    let inner = &s[1..]; // skip '['
    let (val, count) = parse_dec_u32(inner);
    let count: i32 = count;
    if count <= 0_i32
        || (1 + count as usize) >= s.len()
        || s[1 + count as usize] != b']'
        || val == 0_u32
        || val > XKB_MAX_GROUPS
    {
        return -1_i32;
    }
    *out = val.wrapping_sub(1_u32);
    count + 2_i32
}
fn extract_layout_index(s: &[u8], out: &mut u32) -> i32 {
    *out = XKB_LAYOUT_INVALID;
    if s.len() < 3 || s[0] != b'[' {
        return -1_i32;
    }
    if s.len() > 3 && s[1] == b'%' && s[2] == b'i' && s[3] == b']' {
        return 4_i32;
    }
    parse_layout_int_index(s, out)
}
fn extract_mapping_layout_index(s: &[u8], out: &mut u32) -> i32 {
    struct LayoutIndexEntry {
        name: &'static [u8],
        range: u32,
    }
    static NAMES: [LayoutIndexEntry; 4] = [
        LayoutIndexEntry {
            name: b"single]",
            range: LAYOUT_INDEX_SINGLE,
        },
        LayoutIndexEntry {
            name: b"first]",
            range: LAYOUT_INDEX_FIRST,
        },
        LayoutIndexEntry {
            name: b"later]",
            range: LAYOUT_INDEX_LATER,
        },
        LayoutIndexEntry {
            name: b"any]",
            range: LAYOUT_INDEX_ANY,
        },
    ];
    if s.len() < 3 || s[0] != b'[' {
        *out = XKB_LAYOUT_INVALID;
        return -1_i32;
    }
    let after_bracket = &s[1..];
    for entry in &NAMES {
        if after_bracket.starts_with(entry.name) {
            *out = entry.range;
            return (entry.name.len() + 1) as i32;
        }
    }
    *out = XKB_LAYOUT_INVALID;
    parse_layout_int_index(s, out)
}
#[inline]
fn is_mlvo_mask_defined(m: &mut Matcher, mlvo: u32) -> bool {
    m.mapping.defined_mlvo_mask as u32 & 1 << mlvo != 0
}
fn matcher_mapping_set_mlvo(m: &mut Matcher, ident: Sval) {
    let Some((mlvo, name)) = RULES_MLVO_SVALS
        .iter()
        .enumerate()
        .find(|(_, name)| ident.data.starts_with(name))
    else {
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    };
    let mlvo = mlvo as u32;
    if is_mlvo_mask_defined(m, mlvo) {
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    let indexed = name.len() < ident.data.len();
    if matches!(mlvo, MLVO_LAYOUT | MLVO_VARIANT) {
        let mut idx = LAYOUT_INDEX_SINGLE;
        if indexed {
            let remaining = &ident.data[name.len()..];
            if extract_mapping_layout_index(remaining, &mut idx) != remaining.len() as i32 {
                m.mapping.active_or_candidates_mask = 0_u32;
                return;
            }
        }
        if let LayoutIdx::Single {
            layout_idx,
            variant_idx,
        } = &mut m.mapping.layout
        {
            *if mlvo == MLVO_LAYOUT {
                layout_idx
            } else {
                variant_idx
            } = idx;
        }
    } else if indexed {
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    if (mlvo == MLVO_LAYOUT && is_mlvo_mask_defined(m, MLVO_VARIANT)
        || mlvo == MLVO_VARIANT && is_mlvo_mask_defined(m, MLVO_LAYOUT))
        && {
            if let LayoutIdx::Single {
                layout_idx,
                variant_idx,
            } = m.mapping.layout
            {
                layout_idx != variant_idx
            } else {
                false
            }
        }
    {
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.mlvo_at_pos[m.mapping.num_mlvo as usize] = mlvo;
    m.mapping.defined_mlvo_mask = (m.mapping.defined_mlvo_mask as i32 | (1i32 << mlvo)) as u8;
    m.mapping.num_mlvo += 1;
}
fn matcher_mapping_set_layout_bounds(m: &mut Matcher) {
    let idx = if let LayoutIdx::Single {
        layout_idx,
        variant_idx,
    } = m.mapping.layout
    {
        layout_idx.min(variant_idx)
    } else {
        0
    };
    match idx {
        XKB_LAYOUT_INVALID => {
            m.mapping.layout = LayoutIdx::Index {
                layout_idx_min: XKB_LAYOUT_INVALID,
            };
            m.mapping.active_or_candidates_mask = 1;
        }
        LAYOUT_INDEX_LATER | LAYOUT_INDEX_ANY => {
            let layout_idx_min = u32::from(idx == LAYOUT_INDEX_LATER);
            let layout_idx_max = m.rmlvo.layouts.len().min(XKB_MAX_GROUPS as usize) as u32;
            m.mapping.layout = LayoutIdx::Range {
                layout_idx_min,
                layout_idx_max,
            };
            m.mapping.active_or_candidates_mask =
                (1_u64.wrapping_shl(layout_idx_max).wrapping_sub(1) as u32) << layout_idx_min;
        }
        LAYOUT_INDEX_SINGLE | LAYOUT_INDEX_FIRST => {
            m.mapping.layout = LayoutIdx::Index { layout_idx_min: 0 };
            m.mapping.active_or_candidates_mask = 1;
        }
        _ => {
            m.mapping.layout = LayoutIdx::Index {
                layout_idx_min: idx,
            };
            m.mapping.active_or_candidates_mask = 1 << idx;
        }
    }
}
fn matcher_mapping_set_kccgst(m: &mut Matcher, ident: Sval) {
    let Some(kccgst) = RULES_KCCGST_SVALS
        .iter()
        .position(|name| *name == ident.data)
        .map(|index| index as u32)
    else {
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    };
    if m.mapping.defined_kccgst_mask as u32 & 1 << kccgst != 0 {
        m.mapping.active_or_candidates_mask = 0_u32;
        return;
    }
    m.mapping.kccgst_at_pos[m.mapping.num_kccgst as usize] = kccgst;
    m.mapping.defined_kccgst_mask = (m.mapping.defined_kccgst_mask as i32 | (1i32 << kccgst)) as u8;
    m.mapping.num_kccgst += 1;
}
fn fn_layout_or_variant_valid(rmlvo_len: usize, idx: u32) -> bool {
    match idx {
        LAYOUT_INDEX_SINGLE => rmlvo_len <= 1,
        LAYOUT_INDEX_FIRST..=LAYOUT_INDEX_ANY => true,
        _ => rmlvo_len >= 2 && (idx as usize) < rmlvo_len,
    }
}

fn matcher_mapping_verify(m: &mut Matcher) -> bool {
    if m.mapping.num_mlvo == 0 || m.mapping.num_kccgst == 0 {
        m.mapping.active_or_candidates_mask = 0_u32;
        return false;
    }
    if is_mlvo_mask_defined(m, MLVO_LAYOUT) {
        let single_layout_idx = if let LayoutIdx::Single { layout_idx, .. } = m.mapping.layout {
            layout_idx
        } else {
            0
        };
        if !fn_layout_or_variant_valid(m.rmlvo.layouts.len(), single_layout_idx) {
            m.mapping.active_or_candidates_mask = 0_u32;
            return false;
        }
    }
    if is_mlvo_mask_defined(m, MLVO_VARIANT) {
        let single_variant_idx = if let LayoutIdx::Single { variant_idx, .. } = m.mapping.layout {
            variant_idx
        } else {
            0
        };
        if !fn_layout_or_variant_valid(m.rmlvo.variants.len(), single_variant_idx) {
            m.mapping.active_or_candidates_mask = 0_u32;
            return false;
        }
    }
    true
}
fn matcher_rule_start_new(m: &mut Matcher) {
    m.rule = Rule::default();
    m.rule.skip = m.mapping.active_or_candidates_mask == 0;
}
fn matcher_rule_set_mlvo_common(m: &mut Matcher, ident: SvalIdx, match_type: u32) {
    if m.rule.num_mlvo_values as i32 >= m.mapping.num_mlvo as i32 {
        m.rule.skip = true;
        return;
    }
    m.rule.match_type_at_pos[m.rule.num_mlvo_values as usize] = match_type;
    m.rule.mlvo_value_at_pos[m.rule.num_mlvo_values as usize] = ident;
    m.rule.num_mlvo_values += 1;
}
fn matcher_rule_set_kccgst(m: &mut Matcher, ident: SvalIdx) {
    if m.rule.num_kccgst_values as i32 >= m.mapping.num_kccgst as i32 {
        m.rule.skip = true;
        return;
    }
    m.rule.kccgst_value_at_pos[m.rule.num_kccgst_values as usize] = ident;
    m.rule.num_kccgst_values += 1;
}
fn match_group(groups: &[Group], group_name: Sval, to: Sval) -> bool {
    let found_group = groups.iter().find(|g| g.name.as_slice() == group_name.data);
    match found_group {
        None => false,
        Some(group) => {
            for elem in group.elements.iter() {
                if elem.as_slice() == to.data {
                    return true;
                }
            }
            false
        }
    }
}
fn match_value(groups: &[Group], val: Sval, to: Sval, match_type: u32, wildcard_type: u32) -> bool {
    match match_type {
        1 => wildcard_type == WILDCARD_MATCH_ALL || !to.data.is_empty(),
        2 => to.data.is_empty(),
        3 => !to.data.is_empty(),
        4 => true,
        5 => match_group(groups, val, to),
        _ => val.data == to.data,
    }
}
fn match_value_and_mark(
    groups: &[Group],
    val: Sval,
    to: &mut MatchedSval,
    match_type: u32,
    wildcard_type: u32,
) -> bool {
    let matched: bool = match_value(groups, val, to.sval, match_type, wildcard_type);
    if matched {
        to.matched = true;
    }
    matched
}
fn expand_rmlvo_in_kccgst_value(
    m: &mut Matcher,
    value: Sval,
    layout_idx: u32,
    expanded: &mut Vec<u8>,
    i: &mut usize,
) -> bool {
    let bytes = value.data;
    if bytes[*i] == b'i'
        && ((*i).wrapping_add(1_usize) == value.data.len()
            || is_merge_prefix(bytes[(*i).wrapping_add(1_usize)]))
    {
        if layout_idx == XKB_LAYOUT_INVALID {
            return false;
        }
        *i += 1;
        expanded.extend_from_slice(layout_idx.wrapping_add(1).to_string().as_bytes());
        return true;
    }

    let mut sfx = 0;
    let mut pfx = 0;
    let ch = bytes[*i];
    if matches!(ch, b'(' | b'_' | b'-') || is_merge_prefix(ch) {
        pfx = ch;
        if ch == b'(' {
            sfx = b')';
        }
        *i += 1;
        if *i >= value.data.len() {
            return false;
        }
    }

    let mlv = match bytes[*i] {
        b'm' => MLVO_MODEL,
        b'l' => MLVO_LAYOUT,
        b'v' => MLVO_VARIANT,
        _ => return false,
    };
    *i += 1;

    let mut idx = XKB_LAYOUT_INVALID;
    let mut expanded_index = false;
    if *i < value.data.len() && bytes[*i] == b'[' {
        if mlv == MLVO_MODEL {
            return false;
        }
        let consumed = extract_layout_index(&bytes[*i..], &mut idx);
        if consumed < 0 {
            return false;
        }
        if idx == XKB_LAYOUT_INVALID {
            idx = layout_idx;
            expanded_index = true;
        }
        *i += consumed as usize;
    }

    if sfx != 0 {
        if bytes.get(*i) != Some(&sfx) {
            return false;
        }
        *i += 1;
    }

    let selected_index = |len: usize| match idx {
        XKB_LAYOUT_INVALID if len == 1 => Some(0),
        XKB_LAYOUT_INVALID => None,
        _ if idx < len as u32 && (expanded_index || len > 1) => Some(idx as usize),
        _ => None,
    };
    let target =
        match mlv {
            MLVO_MODEL => Some(&mut m.rmlvo.model),
            MLVO_LAYOUT => selected_index(m.rmlvo.layouts.len())
                .and_then(|index| m.rmlvo.layouts.get_mut(index)),
            MLVO_VARIANT => selected_index(m.rmlvo.variants.len())
                .and_then(|index| m.rmlvo.variants.get_mut(index)),
            _ => None,
        };
    let Some(target) = target else {
        return true;
    };
    if target.sval.data.is_empty() {
        return true;
    }

    if pfx != 0 {
        expanded.push(pfx);
    }
    expanded.extend_from_slice(target.sval.data);
    if sfx != 0 {
        expanded.push(sfx);
    }
    target.matched = true;
    true
}
#[allow(clippy::too_many_arguments)]
fn expand_qualifier_in_kccgst_value(
    m: &mut Matcher,
    value: Sval,
    expanded: &mut Vec<u8>,
    has_separator: bool,
    prefix_idx: u32,
    i: &mut usize,
) {
    let bytes = value.data;
    if (*i).wrapping_add(3_usize) <= value.data.len()
        && ((*i).wrapping_add(3_usize) == value.data.len()
            || is_merge_prefix(bytes[(*i).wrapping_add(3_usize)]))
        && bytes[*i] == b'a'
        && bytes[(*i).wrapping_add(1_usize)] == b'l'
        && bytes[(*i).wrapping_add(2_usize)] == b'l'
    {
        expanded.push(b'1');
        if m.rmlvo.layouts.len() > 1 {
            let prefix_length = expanded
                .len()
                .wrapping_sub(prefix_idx as usize)
                .wrapping_sub(1);
            let max_l = if 32 < m.rmlvo.layouts.len() {
                32_u32
            } else {
                m.rmlvo.layouts.len() as u32
            };
            for l in 1..max_l {
                if !has_separator {
                    expanded.push(b'+');
                }
                {
                    let old_size = expanded.len();
                    let new_size = old_size.wrapping_add(prefix_length) + 1;
                    expanded.resize(new_size, 0);
                    expanded.copy_within(
                        prefix_idx as usize..prefix_idx as usize + prefix_length,
                        old_size,
                    );
                    expanded.truncate(new_size - 1);
                }
                let idx_str = format!("{}", l.wrapping_add(1_u32));
                expanded.extend_from_slice(idx_str.as_bytes());
            }
        }
        *i = (*i).wrapping_add(3_usize);
    }
}
#[inline]
fn concat_kccgst(into: &mut Vec<u8>, from: &[u8]) {
    let from_plus = from.first().is_some_and(|&byte| is_merge_prefix(byte));
    if from_plus || into.is_empty() {
        into.extend_from_slice(from);
    } else {
        let into_plus = into.first().is_some_and(|&byte| is_merge_prefix(byte));
        if into_plus {
            let old_len = into.len();
            into.resize(old_len + from.len(), 0);
            into.copy_within(..old_len, from.len());
            for (i, &b) in from.iter().enumerate() {
                into[i] = b;
            }
        }
    }
}
fn expand_kccgst_value(m: &mut Matcher, value: Sval, layout_idx: u32) -> Option<Vec<u8>> {
    let bytes = value.data;
    let mut expanded = Vec::new();
    let mut last_item_idx: u32 = 0;
    let mut has_separator: bool = false;
    let mut invalid = false;
    let mut i: usize = 0_usize;
    loop {
        if i >= value.data.len() {
            break;
        }
        match bytes[i] {
            b':' => {
                expanded.push(bytes[i]);
                i += 1;
                expand_qualifier_in_kccgst_value(
                    m,
                    value,
                    &mut expanded,
                    has_separator,
                    last_item_idx,
                    &mut i,
                );
            }
            b'%' => {
                i += 1;
                if i >= value.data.len()
                    || !expand_rmlvo_in_kccgst_value(m, value, layout_idx, &mut expanded, &mut i)
                {
                    invalid = true;
                    break;
                }
            }
            b if is_merge_prefix(b) => {
                expanded.push(bytes[i]);
                i += 1;
                last_item_idx = (expanded.len() - 1) as u32;
                has_separator = true;
            }
            _ => {
                expanded.push(bytes[i]);
                i += 1;
            }
        }
    }
    if invalid {
        None
    } else {
        Some(expanded)
    }
}
fn matcher_append_pending_kccgst(m: &mut Matcher) {
    let LayoutIdx::Range {
        layout_idx_min,
        layout_idx_max,
    } = m.mapping.layout
    else {
        return;
    };
    for i in 0..m.mapping.num_kccgst as usize {
        let kccgst: u32 = m.mapping.kccgst_at_pos[i];
        if kccgst == KCCGST_GEOMETRY {
            continue;
        }
        for layout in layout_idx_min..layout_idx_max {
            for (slice_layout, value) in &m.pending_kccgst[kccgst as usize] {
                if *slice_layout == layout {
                    concat_kccgst(&mut m.kccgst[kccgst as usize], value);
                }
            }
        }
    }
    m.mapping.layout = LayoutIdx::default();
}
fn matcher_mlvo_matches(
    m: &mut Matcher,
    mlvo: u32,
    value: Sval,
    match_type: u32,
    layout: u32,
) -> bool {
    let (groups, rmlvo) = (&m.groups, &mut m.rmlvo);
    match mlvo {
        MLVO_MODEL => match_value_and_mark(
            groups,
            value,
            &mut rmlvo.model,
            match_type,
            WILDCARD_MATCH_ALL,
        ),
        MLVO_LAYOUT => match_value_and_mark(
            groups,
            value,
            &mut rmlvo.layouts[layout as usize],
            match_type,
            WILDCARD_MATCH_NONEMPTY,
        ),
        MLVO_VARIANT => match_value_and_mark(
            groups,
            value,
            &mut rmlvo.variants[layout as usize],
            match_type,
            WILDCARD_MATCH_NONEMPTY,
        ),
        MLVO_OPTION => rmlvo
            .options
            .iter_mut()
            .filter(|option| {
                matches!(option.layout, OPTIONS_MATCH_ALL_GROUPS) || option.layout == layout
            })
            .any(|option| {
                match_value_and_mark(groups, value, option, match_type, WILDCARD_MATCH_ALL)
            }),
        _ => false,
    }
}
fn matcher_rule_apply_if_matches(m: &mut Matcher, s: &mut Scanner) {
    let mut candidate_layouts = m.mapping.active_or_candidates_mask;
    for i in 0..m.mapping.num_mlvo as usize {
        let mlvo = m.mapping.mlvo_at_pos[i];
        let value = m.rule.mlvo_value_at_pos[i].as_sval(s.s);
        let match_type = m.rule.match_type_at_pos[i];
        if mlvo == MLVO_MODEL {
            if !matcher_mlvo_matches(m, mlvo, value, match_type, 0) {
                return;
            }
            continue;
        }
        let matched = match m.mapping.layout {
            LayoutIdx::Range {
                layout_idx_min,
                layout_idx_max,
            } => {
                let mut matched = false;
                for layout in layout_idx_min..layout_idx_max {
                    let mask = 1 << layout;
                    if candidate_layouts & mask == 0 {
                        continue;
                    }
                    if matcher_mlvo_matches(m, mlvo, value, match_type, layout) {
                        matched = true;
                    } else {
                        candidate_layouts &= !mask;
                    }
                }
                matched
            }
            LayoutIdx::Index { layout_idx_min } => {
                matcher_mlvo_matches(m, mlvo, value, match_type, layout_idx_min)
            }
            LayoutIdx::Single { .. } => false,
        };
        if !matched {
            return;
        }
    }
    match m.mapping.layout {
        LayoutIdx::Range {
            layout_idx_min,
            layout_idx_max,
        } => {
            for layout in layout_idx_min..layout_idx_max {
                if candidate_layouts & 1 << layout != 0 {
                    apply_kccgst(m, s, layout, true);
                }
            }
        }
        LayoutIdx::Index { layout_idx_min } => apply_kccgst(m, s, layout_idx_min, false),
        LayoutIdx::Single { .. } => {}
    }
    if !is_mlvo_mask_defined(m, MLVO_OPTION) {
        m.mapping.active_or_candidates_mask &= !candidate_layouts;
    }
}
fn apply_kccgst(m: &mut Matcher, s: &Scanner, layout: u32, pending: bool) {
    for i in 0..m.mapping.num_kccgst as usize {
        let kccgst = m.mapping.kccgst_at_pos[i];
        if kccgst == KCCGST_GEOMETRY {
            continue;
        }
        let value = m.rule.kccgst_value_at_pos[i].as_sval(s.s);
        if let Some(expanded) = expand_kccgst_value(m, value, layout) {
            if pending {
                m.pending_kccgst[kccgst as usize].push((layout, expanded));
            } else if !expanded.is_empty() {
                concat_kccgst(&mut m.kccgst[kccgst as usize], &expanded);
            }
        }
    }
}
fn gettok(m: &mut Matcher, s: &mut Scanner) -> u32 {
    lex(s, &mut m.val)
}
fn matcher_match(m: &mut Matcher, s: &mut Scanner, include_depth: u32) -> bool {
    let mut have_bang = false;
    loop {
        if !have_bang {
            match gettok(m, s) {
                TOK_END_OF_LINE => continue,
                TOK_END_OF_FILE => return true,
                TOK_BANG => {}
                _ => return false,
            }
        }
        have_bang = false;
        match gettok(m, s) {
            TOK_GROUP_NAME => {
                m.groups.push(Group {
                    name: m.val.as_sval(s.s).data.to_vec(),
                    elements: Vec::new(),
                });
                if gettok(m, s) != TOK_EQUALS {
                    return false;
                }
                loop {
                    match gettok(m, s) {
                        TOK_IDENTIFIER => m
                            .groups
                            .last_mut()
                            .unwrap()
                            .elements
                            .push(m.val.as_sval(s.s).data.to_vec()),
                        TOK_END_OF_LINE => break,
                        _ => return false,
                    }
                }
            }
            TOK_INCLUDE => {
                if gettok(m, s) != TOK_IDENTIFIER {
                    return false;
                }
                matcher_include(m, include_depth, m.val.as_sval(s.s));
                if gettok(m, s) != TOK_END_OF_LINE {
                    return false;
                }
            }
            TOK_IDENTIFIER => {
                matcher_mapping_start_new(m);
                matcher_mapping_set_mlvo(m, m.val.as_sval(s.s));
                loop {
                    match gettok(m, s) {
                        TOK_IDENTIFIER if m.mapping.active_or_candidates_mask != 0 => {
                            matcher_mapping_set_mlvo(m, m.val.as_sval(s.s))
                        }
                        TOK_IDENTIFIER => {}
                        TOK_EQUALS => break,
                        _ => return false,
                    }
                }
                loop {
                    match gettok(m, s) {
                        TOK_IDENTIFIER if m.mapping.active_or_candidates_mask != 0 => {
                            matcher_mapping_set_kccgst(m, m.val.as_sval(s.s))
                        }
                        TOK_IDENTIFIER => {}
                        TOK_END_OF_LINE => break,
                        _ => return false,
                    }
                }
                if m.mapping.active_or_candidates_mask != 0 && matcher_mapping_verify(m) {
                    matcher_mapping_set_layout_bounds(m);
                    if matches!(m.mapping.layout, LayoutIdx::Range { .. }) {
                        m.pending_kccgst.iter_mut().for_each(Vec::clear);
                    }
                }
                loop {
                    let mut tok = gettok(m, s);
                    match tok {
                        TOK_BANG => {
                            matcher_append_pending_kccgst(m);
                            have_bang = true;
                            break;
                        }
                        TOK_END_OF_LINE => continue,
                        TOK_END_OF_FILE => {
                            matcher_append_pending_kccgst(m);
                            return true;
                        }
                        _ => {}
                    }
                    matcher_rule_start_new(m);
                    loop {
                        match tok {
                            TOK_IDENTIFIER if !m.rule.skip => {
                                if m.val.as_sval(s.s).data == b"+" {
                                    matcher_rule_set_mlvo_common(
                                        m,
                                        SvalIdx::EMPTY,
                                        MLVO_MATCH_WILDCARD_SOME,
                                    );
                                } else {
                                    matcher_rule_set_mlvo_common(m, m.val, MLVO_MATCH_NORMAL);
                                }
                            }
                            TOK_WILD_CARD_STAR..=TOK_WILD_CARD_ANY if !m.rule.skip => {
                                matcher_rule_set_mlvo_common(
                                    m,
                                    SvalIdx::EMPTY,
                                    [
                                        MLVO_MATCH_WILDCARD_LEGACY,
                                        MLVO_MATCH_WILDCARD_NONE,
                                        MLVO_MATCH_WILDCARD_SOME,
                                        MLVO_MATCH_WILDCARD_ANY,
                                    ]
                                        [tok as usize - TOK_WILD_CARD_STAR as usize],
                                );
                            }
                            TOK_GROUP_NAME if !m.rule.skip => {
                                matcher_rule_set_mlvo_common(m, m.val, MLVO_MATCH_GROUP)
                            }
                            TOK_IDENTIFIER
                            | TOK_WILD_CARD_STAR..=TOK_WILD_CARD_ANY
                            | TOK_GROUP_NAME => {}
                            TOK_EQUALS => break,
                            _ => return false,
                        }
                        tok = gettok(m, s);
                    }
                    loop {
                        match gettok(m, s) {
                            TOK_IDENTIFIER if !m.rule.skip => matcher_rule_set_kccgst(m, m.val),
                            TOK_IDENTIFIER => {}
                            TOK_END_OF_LINE => break,
                            _ => return false,
                        }
                    }
                    if !m.rule.skip {
                        m.rule.skip = m.rule.num_mlvo_values != m.mapping.num_mlvo
                            || m.rule.num_kccgst_values != m.mapping.num_kccgst;
                        if !m.rule.skip {
                            matcher_rule_apply_if_matches(m, s);
                        }
                    }
                }
            }
            _ => return false,
        }
    }
}
fn read_rules_file(matcher: &mut Matcher<'_>, include_depth: u32, file_data: &[u8]) -> bool {
    let mut scanner = Scanner::new(file_data);
    scanner.check_supported_char_encoding() && matcher_match(matcher, &mut scanner, include_depth)
}
fn xkb_resolve_partial_rules(rules: &str, suffix: &str, matcher: &mut Matcher<'_>) -> bool {
    let partial_rules = format!("{rules}{suffix}");
    if partial_rules.len() >= 60 {
        return false;
    }
    let mut offset = 0;
    while let Some((file_data, _)) =
        find_file_in_xkb_path(matcher.ctx, &partial_rules, FileType::Rules, &mut offset)
    {
        if !read_rules_file(matcher, 0, &file_data) {
            return false;
        }
        offset += 1;
    }
    true
}
fn xkb_resolve_rules(
    rules: &str,
    matcher: &mut Matcher<'_>,
    out: &mut XkbComponentNames,
    explicit_layouts: &mut u32,
) -> bool {
    let mut offset = 0;
    let Some((file_data, _)) =
        find_file_in_xkb_path(matcher.ctx, rules, FileType::Rules, &mut offset)
    else {
        return false;
    };
    if !xkb_resolve_partial_rules(rules, ".pre", matcher)
        || !read_rules_file(matcher, 0, &file_data)
        || !xkb_resolve_partial_rules(rules, ".post", matcher)
        || matcher.kccgst[..KCCGST_GEOMETRY as usize]
            .iter()
            .any(Vec::is_empty)
    {
        return false;
    }
    for (source, target) in matcher.kccgst.iter_mut().zip([
        &mut out.keycodes,
        &mut out.types,
        &mut out.compatibility,
        &mut out.symbols,
    ]) {
        *target = std::mem::take(source);
        target.push(0);
    }
    *explicit_layouts = 1;
    let mut pos = 0;
    while let Some(colon) = out.symbols[pos..].iter().position(|&byte| byte == b':') {
        pos += colon + 1;
        if out.symbols.get(pos).copied().unwrap_or(0) == 0 {
            break;
        }
        let (group, count) = parse_dec_u32(&out.symbols[pos..]);
        let count = count as usize;
        if count > 0
            && out
                .symbols
                .get(pos + count)
                .is_some_and(|&byte| byte == 0 || is_merge_prefix(byte))
            && (1..=XKB_MAX_GROUPS).contains(&group)
        {
            *explicit_layouts = (*explicit_layouts).max(group);
            pos += count;
        }
    }
    true
}
pub(crate) fn xkb_components_from_rules_names(
    ctx: &mut XkbContext,
    rmlvo: &XkbRuleNames,
    out: &mut XkbComponentNames,
    explicit_layouts: &mut u32,
) -> bool {
    let mut matcher = matcher_new_from_names(ctx, rmlvo);
    xkb_resolve_rules(&rmlvo.rules, &mut matcher, out, explicit_layouts)
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use lasso::Key as _;

// ── xkbcommon public types ───────────────────────────────────────────
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: u32 = 2;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: u32 = 1;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_WRAP: u32 = 0;

pub(crate) const XKB_STATE_LAYOUT_EFFECTIVE: u32 = 128;
pub(crate) const XKB_STATE_LAYOUT_LOCKED: u32 = 64;
pub(crate) const XKB_STATE_LAYOUT_LATCHED: u32 = 32;
pub(crate) const XKB_STATE_LAYOUT_DEPRESSED: u32 = 16;
pub(crate) const XKB_STATE_MODS_EFFECTIVE: u32 = 8;
pub(crate) const XKB_STATE_MODS_LOCKED: u32 = 4;
pub(crate) const XKB_STATE_MODS_LATCHED: u32 = 2;
pub(crate) const XKB_STATE_MODS_DEPRESSED: u32 = 1;

pub(crate) const XKB_KEYMAP_FORMAT_TEXT_V2: u32 = 2;
pub(crate) const XKB_KEYMAP_FORMAT_TEXT_V1: u32 = 1;

pub(crate) const XKB_KEYMAP_COMPILE_STRICT_MODE: u32 = 1;
pub(crate) const XKB_KEYMAP_COMPILE_NO_FLAGS: u32 = 0;

pub(crate) const XKB_LAYOUT_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_MOD_INVALID: u32 = 0xffffffff;

// ── XkbRuleNames ──────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub(crate) struct XkbRuleNames {
    pub(crate) rules: String,
    pub(crate) model: String,
    pub(crate) layout: String,
    pub(crate) variant: String,
    pub(crate) options: String,
}

// ── XkbContext ─────────────────────────────────────────────────────

#[derive(Clone)]
pub(crate) struct XkbContext {
    pub(crate) includes: Vec<String>,
    pub(crate) failed_includes: Vec<String>,
    pub(crate) atom_table: lasso::Rodeo,
    pub(crate) use_environment_names: bool,
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
            use std::io::Read;
            let mut file = std::fs::File::open(path).ok()?;
            let mut data = Vec::new();
            file.read_to_end(&mut data).ok()?;
            let arc = Arc::new(data);
            FILE_CACHE.with(|cache| {
                cache.borrow_mut().insert(path.to_string(), arc.clone());
            });
            Some(arc)
        })
}

// ── keymap_h types (from keymap_priv.rs) ────────────────────────────
#[derive(Clone)]
pub(crate) struct XkbKeymap {
    pub(crate) ctx: XkbContext,
    pub(crate) flags: u32,
    pub(crate) format: u32,
    pub(crate) num_leds: u32,
    pub(crate) leds: [XkbLed; 32],
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

#[derive(Clone, Default)]
pub(crate) struct XkbSymInterpret {
    pub(crate) sym: u32,
    pub(crate) match_0: u32,
    pub(crate) mods: u32,
    pub(crate) virtual_mod: u32,
    pub(crate) level_one_only: bool,
    pub(crate) repeat: bool,
    pub(crate) actions: Vec<XkbAction>,
}

#[derive(Clone, Copy, Default)]
pub(crate) enum XkbAction {
    #[default]
    None,
    Void,
    ModSet(XkbModAction),
    ModLatch(XkbModAction),
    ModLock(XkbModAction),
    GroupSet(XkbGroupAction),
    GroupLatch(XkbGroupAction),
    GroupLock(XkbGroupAction),
    CtrlSet(XkbControlsAction),
    CtrlLock(XkbControlsAction),
    Unknown,
    Private(XkbPrivateAction),
    Internal(XkbInternalAction),
}

#[derive(Copy, Clone, Default)]
pub struct XkbInternalAction {
    pub flags: u32,
    pub clear_latched_mods: u32,
}

pub const _ACTION_TYPE_NUM_ENTRIES: u32 = 21;
pub const ACTION_TYPE_INTERNAL: u32 = 20;
pub const ACTION_TYPE_PRIVATE: u32 = 19;
pub const ACTION_TYPE_UNKNOWN: u32 = 18;
pub const ACTION_TYPE_UNSUPPORTED_LEGACY: u32 = 17;
pub const ACTION_TYPE_REDIRECT_KEY: u32 = 16;
pub const ACTION_TYPE_CTRL_LOCK: u32 = 15;
pub const ACTION_TYPE_CTRL_SET: u32 = 14;
pub const ACTION_TYPE_SWITCH_VT: u32 = 13;
pub const ACTION_TYPE_TERMINATE: u32 = 12;
pub const ACTION_TYPE_PTR_DEFAULT: u32 = 11;
pub const ACTION_TYPE_PTR_LOCK: u32 = 10;
pub const ACTION_TYPE_PTR_BUTTON: u32 = 9;
pub const ACTION_TYPE_PTR_MOVE: u32 = 8;
pub const ACTION_TYPE_GROUP_LOCK: u32 = 7;
pub const ACTION_TYPE_GROUP_LATCH: u32 = 6;
pub const ACTION_TYPE_GROUP_SET: u32 = 5;
pub const ACTION_TYPE_MOD_LOCK: u32 = 4;
pub const ACTION_TYPE_MOD_LATCH: u32 = 3;
pub const ACTION_TYPE_MOD_SET: u32 = 2;
pub const ACTION_TYPE_VOID: u32 = 1;
pub const ACTION_TYPE_NONE: u32 = 0;

#[derive(Copy, Clone, Default)]
pub struct XkbPrivateAction {
    pub data: [u8; 7],
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ActionFlags: u32 {
        const LOCK_CLEAR            = 1;
        const LATCH_TO_LOCK         = 2;
        const LOCK_NO_LOCK          = 4;
        const LOCK_NO_UNLOCK        = 8;
        const MODS_LOOKUP_MODMAP    = 16;
        const ABSOLUTE_SWITCH       = 32;
        const LOCK_ON_RELEASE       = 1024;
        const UNLOCK_ON_PRESS       = 2048;
        const LATCH_ON_PRESS        = 4096;
        const PENDING_COMPUTATION   = 8192;
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub(crate) struct ControlsFlags: u32 {
        const STICKY_KEYS      = 1;
        const OVERLAY1         = 2;
        const OVERLAY2         = 4;
        const OVERLAY3         = 8;
        const OVERLAY4         = 16;
        const OVERLAY5         = 32;
        const OVERLAY6         = 64;
        const OVERLAY7         = 128;
        const OVERLAY8         = 256;
        const REPEAT           = 1024;
        const SLOW             = 2048;
        const DEBOUNCE         = 4096;
        const MOUSE_KEYS       = 16384;
        const MOUSE_KEYS_ACCEL = 32768;
        const AX               = 65536;
        const AX_TIMEOUT       = 131072;
        const AX_FEEDBACK      = 262144;
        const BELL             = 524288;
        const IGNORE_GROUP_LOCK = 1048576;
        const ALL_BOOLEAN      = 2088447;
        const ALL_BOOLEAN_V1   = 2087943;
    }
}

#[derive(Copy, Clone, Default)]
pub struct XkbControlsAction {
    pub flags: ActionFlags,
    pub ctrls: ControlsFlags,
}

#[derive(Copy, Clone, Default)]
pub struct XkbGroupAction {
    pub flags: ActionFlags,
    pub group: i32,
}

#[derive(Clone, Default, Copy)]
pub(crate) struct XkbModAction {
    pub(crate) flags: ActionFlags,
    pub(crate) mods: XkbMods,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbMods {
    pub(crate) mods: u32,
    pub(crate) mask: u32,
}

pub const MATCH_EXACTLY: u32 = 4;
pub const MATCH_ALL: u32 = 3;
pub const MATCH_ANY: u32 = 2;
pub const MATCH_ANY_OR_NONE: u32 = 1;
pub const MATCH_NONE: u32 = 0;

#[derive(Clone)]
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
    pub(crate) actions: Vec<XkbAction>,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct XkbLed {
    pub(crate) name: u32,
    pub(crate) which_groups: u32,
    pub(crate) pending_groups: bool,
    pub(crate) groups: u32,
    pub(crate) which_mods: u32,
    pub(crate) mods: XkbMods,
    pub(crate) ctrls: ControlsFlags,
}

pub(crate) const XKB_MAX_GROUPS: u32 = 32;
pub(crate) const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;

// ── Additional xkbcommon types ──────────────────────────────────────
pub(crate) const XKB_MAX_LEDS: u32 = 32;
pub(crate) const MAX_ACTIONS_PER_LEVEL: i32 = 65535;

pub(crate) const DFLT_XKB_CONFIG_EXTRA_PATH: &str = "/usr/local/etc/xkb";
pub(crate) const DFLT_XKB_CONFIG_ROOT: &str = "/usr/share/xkeyboard-config-2";
pub(crate) const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config.d";
pub(crate) const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: &str =
    "/usr/share/xkeyboard-config-2.d";
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";

// ── xkbcommon_h types (moved from duplicated pub(crate) mod xkbcommon_h blocks) ─

pub(crate) const XKB_CONTEXT_NO_FLAGS: u32 = 0;
pub(crate) const XKB_CONTEXT_NO_DEFAULT_INCLUDES: u32 = 1;
pub(crate) const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: u32 = 2;
pub(crate) const XKB_CONTEXT_NO_SECURE_GETENV: u32 = 4;

pub(crate) const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub(crate) const XKB_KEYSYM_CASE_INSENSITIVE: u32 = 1;

pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: u32 = 1;

pub(crate) const XKB_KEYCODE_INVALID: u32 = 0xffffffff;
pub(crate) const XKB_KEYCODE_MAX: u32 = 0xffffffff_u32.wrapping_sub(1);
pub(crate) const XKB_KEYSYM_MAX: u32 = 0x1fffffff;

#[derive(Clone, Default)]
pub(crate) struct XkbComponentNames {
    pub(crate) keycodes: Vec<u8>,
    pub(crate) compatibility: Vec<u8>,
    pub(crate) symbols: Vec<u8>,
    pub(crate) types: Vec<u8>,
}

pub(crate) const XKB_ATOM_NONE: u32 = 0;

pub(crate) const DEFAULT_INTERPRET_KEY_REPEAT: u32 = 1;
pub(crate) const DEFAULT_INTERPRET_VMOD: u32 = 4294967295;
pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub(crate) const _XKB_MOD_INDEX_NUM_ENTRIES: u32 = 8;
pub(crate) const XKB_ALL_GROUPS: u64 = 4294967295;
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

    /// Safe wrapper around `XkbKeyNumLevels`.
    #[inline]
    pub(crate) fn key_num_levels(&self, key: &XkbKey, layout: u32) -> u32 {
        let group = &key.groups[layout as usize];
        self.types[group.type_idx as usize].num_levels
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
    },
    ArrayRef {
        element: u32,
        field: u32,
        entry: Option<Box<ExprKind>>,
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
pub(crate) type KeyTypeDef = NamedVarDef;
pub(crate) type SymbolsDef = NamedVarDef;
pub(crate) type LedMapDef = NamedVarDef;

pub(crate) struct ModMapDef {
    pub(crate) merge: MergeMode,
    pub(crate) modifier: u32,
    pub(crate) keys: Vec<ExprKind>,
}
pub(crate) struct InterpDef {
    pub(crate) merge: MergeMode,
    pub(crate) sym: u32,
    pub(crate) match_0: Option<ExprKind>,
    pub(crate) def: Vec<VarDef>,
}

pub(crate) struct LedNameDef {
    pub(crate) merge: MergeMode,
    pub(crate) ndx: i64,
    pub(crate) name: Option<ExprKind>,
}

pub(crate) const MAP_HAS_MAP_FLAGS: u32 = 2;
pub(crate) const MAP_IS_DEFAULT: u32 = 1;

pub(crate) enum Statement {
    Include(Vec<IncludeStmt>),
    Keycode(KeycodeDef),
    KeyAlias(KeyAliasDef),
    Var(VarDef),
    KeyType(KeyTypeDef),
    Interp(InterpDef),
    VMod(VModDef),
    Symbols(SymbolsDef),
    ModMap(ModMapDef),
    GroupCompat,
    LedMap(LedMapDef),
    LedName(LedNameDef),
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
pub(crate) const PARSER_V2_STRICT_FLAGS: u32 = 16383;
pub(crate) const PARSER_V1_LAX_FLAGS: u32 = 16379;
pub(crate) const PARSER_V1_STRICT_FLAGS: u32 = 16383;
pub(crate) const PARSER_NO_ILLEGAL_ACTION_FIELDS: u32 = 8192;
pub(crate) const PARSER_NO_UNKNOWN_ACTION_FIELDS: u32 = 4096;
pub(crate) const PARSER_NO_UNKNOWN_ACTION: u32 = 2048;
pub(crate) const PARSER_NO_UNKNOWN_KEY_FIELDS: u32 = 1024;
pub(crate) const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: u32 = 512;
pub(crate) const PARSER_NO_UNKNOWN_LED_FIELDS: u32 = 256;
pub(crate) const PARSER_NO_UNKNOWN_INTERPRET_FIELDS: u32 = 128;
pub(crate) const PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS: u32 = 64;
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
    pub(crate) sym_interprets: Vec<XkbSymInterpret>,
}

#[derive(Copy, Clone)]
pub(crate) struct XkbcompLookup {
    pub(crate) group_index_names: [LookupEntry; 3],
    pub(crate) group_mask_names: [LookupEntry; 5],
}

#[derive(Copy, Clone)]
pub(crate) struct XkbcompFeatures {
    pub(crate) max_groups: u32,
    pub(crate) max_overlays: u8,
    pub(crate) controls_name_offset: u8,
    pub(crate) group_lock_on_release: bool,
    pub(crate) mods_unlock_on_press: bool,
    pub(crate) mods_latch_on_press: bool,
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
        let mut ctx = xkb_context_new(0);

        let file = xkb_parse_string(&mut ctx, input, "second")
            .expect("second map should remain parseable");

        assert_eq!(file.name, "second");
    }
}
