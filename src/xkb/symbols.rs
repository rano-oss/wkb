pub(crate) use super::keymap::xkb_mod_name_to_index;
use super::keysym::xkb_keysym_is_keypad;
use super::keysym::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
use super::parser::{exceeds_include_max_depth, process_include_file};
pub(crate) use super::parser::{KeyAliasDef, ModMapDef, NamedVarDef};
macro_rules! some_or_false {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => return false,
        }
    };
    ($value:expr, $return:expr) => {
        match $value {
            Some(value) => value,
            None => return $return,
        }
    };
}
#[derive(Default)]
#[rustfmt::skip]
pub(crate) struct SymbolsBuilder { pub(crate) include_depth: u32, pub(crate) explicit_group: Option<u32>, pub(crate) keys: Vec<Option<KeyInfo>>, pub(crate) default_key: KeyInfo, pub(crate) group_names: Vec<u32>, pub(crate) modmaps: Vec<(ModMapTarget, ModMapEntry)>, pub(crate) mods: XkbModSet, pub(crate) star_atom: u32 }
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ModMapTarget {
    Key(u32),
    Symbol(u32),
}
#[derive(Copy, Clone)]
#[rustfmt::skip]
pub(crate) struct ModMapEntry { pub(crate) merge: MergeMode, pub(crate) modifier: u32 }
#[derive(Clone, Default)]
#[rustfmt::skip]
pub(crate) struct KeyInfo { pub(crate) name: u32, pub(crate) vmodmap: Option<u32>, pub(crate) default_type: Option<u32>, pub(crate) repeat: Option<bool>, pub(crate) out_of_range: Option<OutOfRangeInfo>, pub(crate) groups: Vec<GroupInfo> }
impl KeyInfo {
    fn has_any_field(&self) -> bool {
        self.default_type.is_some()
            || self.vmodmap.is_some()
            || self.repeat.is_some()
            || self.out_of_range.is_some()
    }
}
#[derive(Clone, Default)]
#[rustfmt::skip]
pub(crate) struct GroupInfo { pub(crate) levels: Vec<XkbLevel>, pub(crate) explicit_syms: bool, pub(crate) explicit_acts: bool, pub(crate) type_0: Option<u32> }
impl GroupInfo {
    fn has_any_field(&self) -> bool {
        self.explicit_syms || self.explicit_acts || self.type_0.is_some()
    }
}
impl SymbolsBuilder {
    fn new(ki: &mut XkbKeymapInfo<'_>, include_depth: u32, mods: &XkbModSet) -> Self {
        let star_atom = ki.keymap.ctx.atom_intern(b"*");
        let mut builder = Self {
            include_depth,
            default_key: KeyInfo {
                name: star_atom,
                ..Default::default()
            },
            star_atom,
            ..Default::default()
        };
        init_vmods(&mut builder.mods, mods, include_depth > 0);
        builder
    }
    fn add_key(&mut self, ki: &mut XkbKeymapInfo<'_>, key: &mut KeyInfo, merge: MergeMode) {
        let Some(slot) = ki.keymap.key_index_by_name(key.name, true) else {
            init_key_info_with_atom(key, self.star_atom);
            return;
        };
        if slot >= self.keys.len() {
            self.keys.resize_with(slot + 1, || None);
        }
        if let Some(existing) = &mut self.keys[slot] {
            merge_keys(self.star_atom, existing, key, merge);
        } else {
            self.keys[slot] = Some(std::mem::take(key));
            init_key_info_with_atom(key, self.star_atom);
        }
    }
    fn add_modmap(&mut self, target: ModMapTarget, new: ModMapEntry) {
        if let Some((_, old)) = self.modmaps.iter_mut().find(|(key, _)| *key == target) {
            if old.modifier != new.modifier && new.merge != MergeMode::Augment {
                old.modifier = new.modifier;
            }
        } else {
            self.modmaps.push((target, new));
        }
    }
    fn finish(&mut self, keymap: &mut XkbKeymap) {
        keymap.mods = self.mods;
        keymap.group_names = std::mem::take(&mut self.group_names);
        for (key_idx, key) in self.keys.iter_mut().enumerate() {
            if let Some(key) = key {
                copy_symbols_def_to_keymap(keymap, key_idx, key);
            }
        }
        let start = keymap.min_key_code as usize;
        for &(target, modmap) in &self.modmaps {
            let key = match target {
                ModMapTarget::Symbol(symbol) => find_key_by_symbol(keymap, start, symbol)
                    .and_then(|index| keymap.keys.get_mut(index)),
                ModMapTarget::Key(name) => keymap.key_by_name_mut(name, true),
            };
            let Some(key) = key else {
                continue;
            };
            if modmap.modifier != XKB_MOD_NONE {
                key.modmap |= 1 << modmap.modifier;
            }
        }
    }
    fn merge_from(&mut self, ki: &mut XkbKeymapInfo<'_>, from: &mut Self, merge: MergeMode) {
        merge_mod_sets(&mut self.mods, &from.mods, merge);
        for (index, &name) in from.group_names.iter().enumerate() {
            if index >= self.group_names.len() {
                self.group_names.push(name);
            } else if name != 0 && !(merge == MergeMode::Augment && self.group_names[index] != 0) {
                self.group_names[index] = name;
            }
        }
        if self.keys.is_empty() {
            std::mem::swap(&mut self.keys, &mut from.keys);
        } else {
            for mut key in std::mem::take(&mut from.keys).into_iter().flatten() {
                self.add_key(ki, &mut key, merge);
            }
        }
        if self.modmaps.is_empty() {
            std::mem::swap(&mut self.modmaps, &mut from.modmaps);
        } else {
            for (target, mut entry) in std::mem::take(&mut from.modmaps) {
                entry.merge = merge;
                self.add_modmap(target, entry);
            }
        }
    }
    fn include(&mut self, ki: &mut XkbKeymapInfo<'_>, includes: &mut [IncludeStmt]) -> bool {
        if exceeds_include_max_depth(self.include_depth) {
            return false;
        }
        let mut included = Self::new(ki, self.include_depth + 1, &self.mods);
        let include_statements = &mut *includes;
        for statement in include_statements {
            let Some(mut file) =
                process_include_file(&mut ki.keymap.ctx, statement, FileType::Symbols)
            else {
                return false;
            };
            let mut next = Self::new(ki, self.include_depth + 1, &included.mods);
            next.explicit_group = if !statement.modifier.is_empty() {
                statement
                    .modifier
                    .parse::<i32>()
                    .ok()
                    .and_then(|group| group.checked_sub(1))
                    .and_then(|group| ((group as u32) < XKB_MAX_GROUPS).then_some(group as u32))
                    .or(self.explicit_group)
            } else if ki.keymap.num_groups != 0 && next.include_depth == 1 {
                Some(0)
            } else {
                self.explicit_group
            };
            if !next.compile_file(ki, &mut file) {
                return false;
            }
            included.merge_from(ki, &mut next, statement.merge);
        }
        if let Some(first) = includes.first() {
            self.merge_from(ki, &mut included, first.merge);
        }
        true
    }
    fn compile_file(&mut self, ki: &mut XkbKeymapInfo<'_>, file: &mut XkbFile) -> bool {
        for statement in &mut file.defs {
            let valid = match statement {
                Statement::Include(includes) => self.include(ki, includes),
                Statement::Symbols(definition) => self.compile_key(ki, definition),
                Statement::Var(variable) => self.compile_global(ki, variable),
                Statement::VMods(vmods) => vmods
                    .iter()
                    .all(|vmod| handle_vmod_def(&mut ki.keymap.ctx, &mut self.mods, vmod)),
                Statement::ModMap(definition) => self.compile_modmap(ki, definition),
                Statement::Unknown => !ki.keymap.strict,
                _ => false,
            };
            if !valid {
                return false;
            }
        }
        true
    }
    fn compile_key(&mut self, ki: &mut XkbKeymapInfo<'_>, stmt: &mut NamedVarDef) -> bool {
        let dk = &self.default_key;
        let mut keyi = dk.clone();
        keyi.name = stmt.name;
        if self.compile_key_body(ki, &mut stmt.body, &mut keyi) {
            set_explicit_group(self, &mut keyi);
            self.add_key(ki, &mut keyi, stmt.merge);
            return true;
        }
        false
    }
    fn compile_modmap(&mut self, ki: &mut XkbKeymapInfo<'_>, def: &mut ModMapDef) -> bool {
        let modifier_name: &str = ki.keymap.ctx.atom_text(def.modifier);
        let ndx = if modifier_name.eq_ignore_ascii_case("none") {
            XKB_MOD_NONE
        } else {
            match xkb_mod_name_to_index(&self.mods, def.modifier, MOD_REAL) {
                Some(n) => n,
                None => return false,
            }
        };
        for key in def.keys.iter() {
            let target = if let ExprKind::KeyName(kn) = key {
                Some(ModMapTarget::Key(*kn))
            } else if let ExprKind::KeySym(ks) = key {
                (*ks != XKB_KEY_NO_SYMBOL).then_some(ModMapTarget::Symbol(*ks))
            } else {
                None
            };
            if let Some(target) = target {
                self.add_modmap(
                    target,
                    ModMapEntry {
                        merge: def.merge,
                        modifier: ndx,
                    },
                );
            }
        }
        true
    }
    fn compile_global(&mut self, ki: &mut XkbKeymapInfo<'_>, stmt: &mut VarDef) -> bool {
        let lhs = some_or_false!(expr_resolve_lhs(stmt.name.as_ref().unwrap()));
        let elem = ki.keymap.ctx.atom_text(lhs.element);
        let is_key = elem.eq_ignore_ascii_case("key");
        let empty = elem.is_empty();
        let field = ki.keymap.ctx.atom_text(lhs.field);
        let symbols_field = parse_symbols_field(field);
        let group_name =
            field.eq_ignore_ascii_case("name") || field.eq_ignore_ascii_case("groupname");
        let ignored = [
            "groupswrap",
            "wrapgroups",
            "groupsclamp",
            "clampgroups",
            "groupsredirect",
            "redirectgroups",
            "allownone",
        ]
        .iter()
        .any(|name| field.eq_ignore_ascii_case(name));
        if is_key {
            let Some(field) = symbols_field else {
                return !ki.keymap.strict;
            };
            let mut temp = KeyInfo {
                name: self.star_atom,
                ..Default::default()
            };
            let valid = set_symbols_field(ki, self, &mut temp, field, lhs.index, &mut stmt.value);
            let mut dk = std::mem::take(&mut self.default_key);
            merge_keys(self.star_atom, &mut dk, &mut temp, stmt.merge);
            self.default_key = dk;
            valid
        } else if empty && group_name {
            set_group_name(
                ki,
                self,
                lhs.index,
                stmt.value.as_ref().unwrap(),
                stmt.merge,
            )
        } else if ignored || !empty {
            true
        } else {
            !ki.keymap.strict
        }
    }
    fn compile_key_body(
        &mut self,
        ki: &mut XkbKeymapInfo<'_>,
        defs: &mut [VarDef],
        keyi: &mut KeyInfo,
    ) -> bool {
        let mut all_valid_entries: bool = true;
        for def in defs.iter_mut() {
            let (field, index) = if let Some(name) = &def.name {
                let Some(lhs) = expr_resolve_lhs(name) else {
                    all_valid_entries = false;
                    continue;
                };
                if !ki.keymap.ctx.atom_text(lhs.element).is_empty() {
                    all_valid_entries = false;
                    continue;
                }
                (
                    parse_symbols_field(ki.keymap.ctx.atom_text(lhs.field)),
                    lhs.index,
                )
            } else if def.value.as_ref().is_some_and(is_action_list_value) {
                (Some(SymbolsField::Actions), None)
            } else {
                (Some(SymbolsField::Symbols), None)
            };
            let valid = match (field, def.value.is_some()) {
                (Some(field), true) => {
                    set_symbols_field(ki, self, keyi, field, index, &mut def.value)
                }
                (None, true) => !ki.keymap.strict,
                _ => false,
            };
            if !valid {
                all_valid_entries = false;
            }
        }
        all_valid_entries
    }
}
fn is_action_list_value(value: &ExprKind) -> bool {
    matches!(value, ExprKind::ActionList { actions }
        if actions.first().is_none_or(|first|
            matches!(first, ExprKind::ActionList { .. } | ExprKind::Action { .. })))
}
fn collect_expr_list(container: &ExprKind) -> &[ExprKind] {
    match container {
        ExprKind::ActionList { actions } => actions.as_slice(),
        _ => std::slice::from_ref(container),
    }
}
fn init_key_info_with_atom(keyi: &mut KeyInfo, star_atom: u32) {
    *keyi = KeyInfo {
        name: star_atom,
        ..Default::default()
    };
}
fn merge_groups(into: &mut GroupInfo, from: &mut GroupInfo, clobber: bool) {
    if let Some(from_type) = from.type_0 {
        if into.type_0.is_none() || clobber {
            into.type_0 = Some(from_type);
        }
    }
    if from.levels.is_empty() {
        *from = GroupInfo::default();
        return;
    }
    if into.levels.is_empty() {
        from.type_0 = into.type_0;
        *into = std::mem::take(from);
        return;
    }
    let levels_in_both = into.levels.len().min(from.levels.len());
    let mut from_keysyms_count = 0;
    let mut from_actions_count = 0;
    for (into_level, from_level) in into.levels.iter_mut().zip(&mut from.levels) {
        if from_level.syms.is_empty() && from_level.action.is_none() {
            continue;
        }
        if into_level.syms.is_empty() && into_level.action.is_none() {
            into_level.syms = std::mem::take(&mut from_level.syms);
            into_level.action = from_level.action.take();
            from_keysyms_count += 1;
            from_actions_count += 1;
            continue;
        }
        if !from_level.syms.is_empty()
            && from_level.syms != into_level.syms
            && (clobber || into_level.syms.is_empty())
        {
            into_level.syms = std::mem::take(&mut from_level.syms);
            from_keysyms_count += 1;
        }
        if from_level.action.is_some()
            && from_level.action != into_level.action
            && (clobber || into_level.action.is_none())
        {
            into_level.action = from_level.action.take();
            from_actions_count += 1;
        }
    }
    for level in from.levels.drain(levels_in_both..) {
        into.levels.push(level);
        from_keysyms_count += 1;
        from_actions_count += 1;
    }
    if from_keysyms_count != 0 {
        if from_keysyms_count == into.levels.len() as u32 {
            into.explicit_syms = false;
        }
        if from.explicit_syms {
            into.explicit_syms = true;
        }
    }
    if from_actions_count != 0 {
        into.explicit_acts |= from.explicit_acts;
    }
}
fn merge_keys(star_atom: u32, into: &mut KeyInfo, from: &mut KeyInfo, merge: MergeMode) {
    let clobber = merge != MergeMode::Augment;
    if merge == MergeMode::Replace {
        std::mem::swap(into, from);
        init_key_info_with_atom(from, star_atom);
        return;
    }
    let groups_in_both = into.groups.len().min(from.groups.len()) as u32;
    for i in 0..groups_in_both as usize {
        merge_groups(&mut into.groups[i], &mut from.groups[i], clobber);
    }
    for group in from.groups.drain(groups_in_both as usize..) {
        into.groups.push(group);
    }
    if from.vmodmap.is_some() && (into.vmodmap.is_none() || clobber) {
        into.vmodmap = from.vmodmap;
    }
    if from.repeat.is_some() && (into.repeat.is_none() || clobber) {
        into.repeat = from.repeat;
    }
    if from.default_type.is_some() && (into.default_type.is_none() || clobber) {
        into.default_type = from.default_type;
    }
    if from.out_of_range.is_some() && (into.out_of_range.is_none() || clobber) {
        into.out_of_range = from.out_of_range;
    }
    init_key_info_with_atom(from, star_atom);
}
fn group_index(
    ki: &mut XkbKeymapInfo<'_>,
    key: &mut KeyInfo,
    index: Option<&ExprKind>,
    actions: bool,
) -> Option<usize> {
    let index = match index {
        Some(expr) => {
            let (group, _) = expr_resolve_group(ki, expr, false)?;
            group.checked_sub(1)? as usize
        }
        None => key
            .groups
            .iter()
            .position(|group| {
                if actions {
                    !group.explicit_acts
                } else {
                    !group.explicit_syms
                }
            })
            .unwrap_or(key.groups.len()),
    };
    if index >= XKB_MAX_GROUPS as usize {
        return None;
    }
    if index >= key.groups.len() {
        key.groups.resize_with(index + 1, GroupInfo::default);
    }
    Some(index)
}
fn add_symbols_to_key(
    ki: &mut XkbKeymapInfo<'_>,
    key: &mut KeyInfo,
    array_index: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let Some(group_index) = group_index(ki, key, array_index, false) else {
        return false;
    };
    let group = &mut key.groups[group_index];
    if matches!(value, ExprKind::EmptyList) {
        group.explicit_syms = true;
        return true;
    }
    if !matches!(
        value,
        ExprKind::KeysymList { .. } | ExprKind::ActionList { .. }
    ) || group.explicit_syms
    {
        return false;
    }
    let nodes = collect_expr_list(value);
    let level_count = nodes
        .iter()
        .rposition(|node| {
            matches!(
                node,
                ExprKind::KeysymList { syms }
                    if !syms.is_empty()
            )
        })
        .map_or(0, |index| index + 1);
    group.levels.resize_with(level_count, XkbLevel::default);
    group.explicit_syms = true;
    for (level, node) in nodes.iter().take(level_count).enumerate() {
        let ExprKind::KeysymList { syms } = node else {
            return false;
        };
        if syms.len() > u16::MAX as usize {
            return false;
        }
        group.levels[level].syms.clone_from(syms);
    }
    true
}
fn action_group(action: &mut XkbAction) -> &mut XkbGroupAction {
    match action {
        XkbAction::GroupSet(group) | XkbAction::GroupLatch(group) | XkbAction::GroupLock(group) => {
            group
        }
        XkbAction::None => unreachable!(),
    }
}
fn group_action(ki: &mut XkbKeymapInfo<'_>, expr: &ExprKind) -> Option<XkbAction> {
    let ExprKind::Action { name, args } = expr else {
        return None;
    };
    let mut action = match ki.keymap.ctx.atom_text(*name) {
        name if name.eq_ignore_ascii_case("SetGroup") => XkbAction::GroupSet(Default::default()),
        name if name.eq_ignore_ascii_case("LatchGroup") => {
            XkbAction::GroupLatch(Default::default())
        }
        name if name.eq_ignore_ascii_case("LockGroup") => XkbAction::GroupLock(Default::default()),
        _ => return None,
    };
    for arg in args {
        let (field, value) = match arg {
            ExprKind::Binary {
                op: BinaryOp::Assign,
                left,
                right,
            } => (&**left, &**right),
            _ => continue,
        };
        let ExprKind::Ident(field) = field else {
            continue;
        };
        let field = ki.keymap.ctx.atom_text(*field);
        if field.eq_ignore_ascii_case("group") {
            let (absolute, negative, value) = match value {
                ExprKind::Unary {
                    op: UnaryOp::Plus,
                    child,
                } => (false, false, &**child),
                ExprKind::Unary {
                    op: UnaryOp::Negate,
                    child,
                } => (false, true, &**child),
                value => (true, false, value),
            };
            let (group, pending) = expr_resolve_group(ki, value, absolute)?;
            let target = action_group(&mut action);
            target.flags.set(ActionFlags::ABSOLUTE_SWITCH, absolute);
            if pending {
                target.flags.insert(ActionFlags::PENDING_COMPUTATION);
                target.group = if absolute {
                    0
                } else if negative {
                    -1
                } else {
                    1
                };
            } else {
                target.group = if absolute {
                    group.wrapping_sub(1) as i32
                } else if negative {
                    -(group as i32)
                } else {
                    group as i32
                };
            }
        }
    }
    Some(action)
}
fn add_actions_to_key(
    ki: &mut XkbKeymapInfo<'_>,
    key: &mut KeyInfo,
    index: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let Some(group_index) = group_index(ki, key, index, true) else {
        return false;
    };
    let group = &mut key.groups[group_index];
    group.explicit_acts = true;
    let ExprKind::ActionList { actions } = value else {
        return matches!(value, ExprKind::EmptyList);
    };
    group.levels.resize_with(actions.len(), XkbLevel::default);
    for (level, item) in group.levels.iter_mut().zip(actions) {
        let ExprKind::ActionList { actions } = item else {
            return false;
        };
        level.action = actions.iter().find_map(|action| group_action(ki, action));
    }
    true
}
macro_rules! field_parser {
    ($type:ident, $parse:ident { $($variant:ident => [$($name:literal),+]),+ $(,)? }) => {
        #[derive(Clone, Copy)]
        enum $type { $($variant),+ }
        fn $parse(field: &str) -> Option<$type> {
            $(if [$($name),+].iter().any(|name| field.eq_ignore_ascii_case(name)) {
                return Some($type::$variant);
            })+
            None
        }
    };
}
field_parser!(SymbolsField, parse_symbols_field_exact {
    Type => ["type"], Symbols => ["symbols"], Actions => ["actions"],
    Vmods => ["vmods", "virtualmods", "virtualmodifiers"],
    Ignored => ["locking", "lock", "locks", "radiogroup", "permanentradiogroup", "allownone", "overlay"],
    Repeat => ["repeating", "repeats", "repeat"], GroupsWrap => ["groupswrap", "wrapgroups"],
    GroupsClamp => ["groupsclamp", "clampgroups"], GroupsRedirect => ["groupsredirect", "redirectgroups"]
});
fn parse_symbols_field(field: &str) -> Option<SymbolsField> {
    parse_symbols_field_exact(field).or_else(|| {
        if field
            .get(..16)
            .is_some_and(|s| s.eq_ignore_ascii_case("permanentoverlay"))
        {
            Some(SymbolsField::Ignored)
        } else if field
            .get(..7)
            .is_some_and(|s| s.eq_ignore_ascii_case("overlay"))
        {
            Some(SymbolsField::Ignored)
        } else {
            None
        }
    })
}
fn set_symbols_field(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsBuilder,
    keyi: &mut KeyInfo,
    mapped_field: SymbolsField,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<ExprKind>,
) -> bool {
    match mapped_field {
        SymbolsField::Type => {
            let val = some_or_false!(expr_resolve_string(value_opt.as_ref().unwrap()));
            if let Some(array_ndx) = array_ndx {
                let ndx = some_or_false!(expr_resolve_group(ki, array_ndx, false)).0 - 1;
                if ndx >= keyi.groups.len() as u32 {
                    keyi.groups
                        .resize_with((ndx as usize) + 1, Default::default);
                }
                keyi.groups[ndx as usize].type_0 = Some(val);
            } else {
                keyi.default_type = Some(val);
            }
        }
        SymbolsField::Symbols => {
            return add_symbols_to_key(ki, keyi, array_ndx, value_opt.as_ref().unwrap());
        }
        SymbolsField::Actions => {
            return add_actions_to_key(ki, keyi, array_ndx, value_opt.as_ref().unwrap());
        }
        SymbolsField::Vmods => {
            let val = value_opt.as_ref().unwrap();
            let mask = some_or_false!(expr_resolve_mod_mask(
                &ki.keymap.ctx,
                val,
                MOD_VIRT,
                &info.mods
            ));
            keyi.vmodmap = Some(mask);
        }
        SymbolsField::Ignored => {}
        SymbolsField::Repeat => {
            keyi.repeat = some_or_false!(expr_resolve_repeat(
                &ki.keymap.ctx,
                value_opt.as_ref().unwrap()
            ));
        }
        SymbolsField::GroupsWrap | SymbolsField::GroupsClamp => {
            let set = some_or_false!(expr_resolve_boolean(
                &ki.keymap.ctx,
                value_opt.as_ref().unwrap()
            ));
            let wrap = matches!(mapped_field, SymbolsField::GroupsWrap);
            let policy = if set == wrap {
                XKB_LAYOUT_OUT_OF_RANGE_WRAP
            } else {
                XKB_LAYOUT_OUT_OF_RANGE_CLAMP
            };
            match &mut keyi.out_of_range {
                Some(oor) => oor.policy = policy,
                None => {
                    keyi.out_of_range = Some(OutOfRangeInfo {
                        policy,
                        ..Default::default()
                    });
                }
            }
        }
        SymbolsField::GroupsRedirect => {
            let (grp, pending) =
                some_or_false!(expr_resolve_group(ki, value_opt.as_ref().unwrap(), false));
            let number = if pending { 0 } else { grp - 1 };
            keyi.out_of_range = Some(OutOfRangeInfo {
                policy: XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
                number,
                pending,
            });
        }
    }
    true
}
fn set_group_name(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsBuilder,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
    merge: MergeMode,
) -> bool {
    let Some(array_ndx) = array_ndx else {
        return false;
    };
    let group = some_or_false!(expr_resolve_group(ki, array_ndx, false)).0;
    let name = some_or_false!(expr_resolve_string(value));
    let group_to_use = match info.explicit_group {
        None => group.wrapping_sub(1),
        Some(explicit_group) if group.wrapping_sub(1) == 0 => explicit_group,
        Some(_) => return false,
    } as usize;
    if group_to_use >= info.group_names.len() {
        info.group_names.resize(group_to_use + 1, 0);
    }
    let old = &mut info.group_names[group_to_use];
    if *old == XKB_ATOM_NONE || merge != MergeMode::Augment {
        *old = name;
    }
    true
}
fn set_explicit_group(info: &SymbolsBuilder, keyi: &mut KeyInfo) {
    let eg = match info.explicit_group {
        None => return,
        Some(v) => v,
    };
    if !keyi.groups.is_empty() {
        for group in keyi.groups[1..].iter_mut() {
            if group.has_any_field() {
                *group = GroupInfo::default();
            }
        }
    }
    keyi.groups.resize_with((eg as usize) + 1, Default::default);
    if eg > 0 {
        keyi.groups[eg as usize] = std::mem::take(&mut keyi.groups[0]);
    }
}
fn find_automatic_type(ctx: &mut XkbContext, group: &GroupInfo) -> u32 {
    let symbol = |level: usize| {
        group
            .levels
            .get(level)
            .and_then(|level| level.syms.first())
            .copied()
            .unwrap_or(XKB_KEY_NO_SYMBOL)
    };
    let width = group.levels.len();
    match width {
        0 | 1 => ctx.atom_intern(b"ONE_LEVEL"),
        2 => {
            let first = symbol(0);
            let second = symbol(1);
            if xkb_keysym_is_lower(first) && xkb_keysym_is_upper_or_title(second) {
                ctx.atom_intern(b"ALPHABETIC")
            } else if xkb_keysym_is_keypad(first) || xkb_keysym_is_keypad(second) {
                ctx.atom_intern(b"KEYPAD")
            } else {
                ctx.atom_intern(b"TWO_LEVEL")
            }
        }
        3 | 4 => {
            let first = symbol(0);
            let second = symbol(1);
            if xkb_keysym_is_lower(first) && xkb_keysym_is_upper_or_title(second) {
                let third = symbol(2);
                let fourth = symbol(3);
                if xkb_keysym_is_lower(third) && xkb_keysym_is_upper_or_title(fourth) {
                    ctx.atom_intern(b"FOUR_LEVEL_ALPHABETIC")
                } else {
                    ctx.atom_intern(b"FOUR_LEVEL_SEMIALPHABETIC")
                }
            } else if xkb_keysym_is_keypad(first) || xkb_keysym_is_keypad(second) {
                ctx.atom_intern(b"FOUR_LEVEL_KEYPAD")
            } else {
                ctx.atom_intern(b"FOUR_LEVEL")
            }
        }
        _ => XKB_ATOM_NONE,
    }
}
fn find_type_for_group(keymap: &mut XkbKeymap, key: &KeyInfo, group: usize) -> usize {
    let group = &key.groups[group];
    let name = group
        .type_0
        .or(key.default_type)
        .unwrap_or_else(|| find_automatic_type(&mut keymap.ctx, group));
    keymap
        .types
        .iter()
        .position(|key_type| key_type.name == name)
        .unwrap_or(0)
}
fn copy_symbols_def_to_keymap(keymap: &mut XkbKeymap, key_idx: usize, keyi: &mut KeyInfo) {
    let num_groups = keyi
        .groups
        .iter()
        .rposition(|group| {
            !group.levels.is_empty() || keyi.default_type.is_some() || group.type_0.is_some()
        })
        .map_or(0, |index| index + 1);
    keymap.keys[key_idx].num_groups = num_groups as u32;
    if num_groups == 0 && !keyi.has_any_field() {
        return;
    }
    if num_groups != 0 {
        keyi.groups.resize_with(num_groups, Default::default);
        for i in 1..keyi.groups.len() {
            if !keyi.groups[i].has_any_field() {
                keyi.groups[i] = keyi.groups[0].clone();
            }
        }
        keymap.keys[key_idx].groups = vec![XkbGroup::default(); num_groups];
        for i in 0..keyi.groups.len() as u32 {
            let type_idx = find_type_for_group(keymap, keyi, i as usize) as u32;
            let need_levels = keymap.types[type_idx as usize].num_levels as usize;
            keyi.groups[i as usize]
                .levels
                .resize_with(need_levels, Default::default);
            keymap.keys[key_idx].groups[i as usize].type_idx = type_idx;
        }
        for i in 0..keyi.groups.len() {
            let groupi = &mut keyi.groups[i];
            for li in 0..groupi.levels.len() {
                let leveli = &mut groupi.levels[li];
                if leveli.syms.len() > 1 {
                    let has_upper = leveli.syms.iter().any(|&s| xkb_keysym_to_upper(s) != s);
                    if has_upper {
                        let original = leveli.syms.len();
                        leveli.syms.extend_from_within(..);
                        leveli.syms[original..]
                            .iter_mut()
                            .for_each(|sym| *sym = xkb_keysym_to_upper(*sym));
                    }
                }
            }
            keymap.keys[key_idx].groups[i].levels = std::mem::take(&mut groupi.levels);
            keymap.keys[key_idx].groups[i].explicit_actions = groupi.explicit_acts;
        }
        if let Some(oor) = keyi.out_of_range {
            keymap.keys[key_idx].out_of_range = Some(oor);
        }
    }
    if let Some(vmodmap) = keyi.vmodmap {
        keymap.keys[key_idx].vmodmap = Some(vmodmap);
    }
    if let Some(repeat) = keyi.repeat {
        keymap.keys[key_idx].repeats = Some(repeat);
    }
}
fn find_key_by_symbol(keymap: &XkbKeymap, start: usize, sym: u32) -> Option<usize> {
    (start..keymap.keys.len()).find(|&index| {
        let key = &keymap.keys[index];
        key.groups
            .iter()
            .take(key.num_groups as usize)
            .flat_map(|group| &group.levels)
            .any(|level| level.syms.contains(&sym))
    })
}
pub(crate) fn compile_symbols(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut builder = SymbolsBuilder::new(keymap_info, 0, &mods);
    if file.is_some_and(|file| !builder.compile_file(keymap_info, file)) {
        return false;
    }
    builder.finish(keymap_info.keymap);
    true
}
use super::keysym::xkb_keysym_to_upper;
use super::parser::*;
#[derive(Default)]
pub(crate) struct KeyTypesInfo {
    pub(crate) include_depth: u32,
    pub(crate) types: Vec<XkbKeyType>,
    pub(crate) mods: XkbModSet,
}
fn key_types_info(include_depth: u32, mods: &XkbModSet) -> KeyTypesInfo {
    let mut info = KeyTypesInfo {
        include_depth,
        ..Default::default()
    };
    init_vmods(&mut info.mods, mods, include_depth > 0);
    info
}
fn add_key_type(info: &mut KeyTypesInfo, new: XkbKeyType, merge: MergeMode) {
    if let Some(index) = info
        .types
        .iter()
        .position(|existing| existing.name == new.name)
    {
        if merge != MergeMode::Augment {
            info.types[index] = new;
        }
        return;
    }
    info.types.push(new);
}
fn merge_included_key_types(into: &mut KeyTypesInfo, from: &mut KeyTypesInfo, merge: MergeMode) {
    merge_mod_sets(&mut into.mods, &from.mods, merge);
    if into.types.is_empty() {
        into.types = std::mem::take(&mut from.types);
    } else {
        for type_0 in from.types.drain(..) {
            add_key_type(into, type_0, merge);
        }
    }
}
fn handle_include_key_types(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    includes: &mut [IncludeStmt],
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        return false;
    }
    let mut included = key_types_info(info.include_depth.wrapping_add(1), &info.mods);
    for stmt in includes.iter() {
        let Some(mut file) = process_include_file(&mut ki.keymap.ctx, stmt, FileType::Types) else {
            return false;
        };
        let mut next = key_types_info(info.include_depth.wrapping_add(1), &included.mods);
        if !handle_key_types_file(ki, &mut next, &mut file) {
            return false;
        }
        merge_included_key_types(&mut included, &mut next, stmt.merge);
    }
    if let Some(first) = includes.first() {
        merge_included_key_types(info, &mut included, first.merge);
    }
    true
}
fn add_map_entry(type_0: &mut XkbKeyType, new: &XkbKeyTypeEntry) {
    type_0.num_levels = type_0.num_levels.max(new.level + 1);
    if let Some(old) = type_0
        .entries
        .iter_mut()
        .find(|entry| entry.mods.mods == new.mods.mods)
    {
        old.level = new.level;
    } else {
        type_0.entries.push(*new);
    }
}
fn set_key_type_field(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut XkbKeyType,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if field.eq_ignore_ascii_case("modifiers") {
        if array_ndx.is_some() || type_0.modifiers_set {
            return false;
        }
        type_0.mods.mods = some_or_false!(expr_resolve_mod_mask(
            &ki.keymap.ctx,
            value,
            MOD_BOTH,
            &info.mods
        ));
        type_0.modifiers_set = true;
        return true;
    }
    if field.eq_ignore_ascii_case("levelname") || field.eq_ignore_ascii_case("level_name") {
        return true;
    }
    if !(field.eq_ignore_ascii_case("map") || field.eq_ignore_ascii_case("preserve")) {
        return !ki.keymap.strict;
    }
    let Some(array_ndx) = array_ndx else {
        return false;
    };
    let mods = some_or_false!(expr_resolve_mod_mask(
        &ki.keymap.ctx,
        array_ndx,
        MOD_BOTH,
        &info.mods
    )) & type_0.mods.mods;
    if field.eq_ignore_ascii_case("map") {
        add_map_entry(
            type_0,
            &XkbKeyTypeEntry {
                level: some_or_false!(expr_resolve_level(&ki.keymap.ctx, value)),
                mods: XkbMods { mods, mask: 0 },
                preserve: XkbMods::default(),
            },
        );
    } else {
        let preserve = some_or_false!(expr_resolve_mod_mask(
            &ki.keymap.ctx,
            value,
            MOD_BOTH,
            &info.mods
        )) & mods;
        if let Some(entry) = type_0
            .entries
            .iter_mut()
            .find(|entry| entry.mods.mods == mods)
        {
            entry.preserve.mods = preserve;
        } else {
            type_0.entries.push(XkbKeyTypeEntry {
                level: 0,
                mods: XkbMods { mods, mask: 0 },
                preserve: XkbMods {
                    mods: preserve,
                    mask: 0,
                },
            });
        }
    }
    true
}
fn handle_key_type_body(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    defs: &[VarDef],
    type_0: &mut XkbKeyType,
) -> bool {
    defs.iter().all(|def| {
        let Some(lhs) = def.name.as_ref().and_then(expr_resolve_lhs) else {
            return false;
        };
        let elem = ki.keymap.ctx.atom_text(lhs.element);
        elem.eq_ignore_ascii_case("type")
            || elem.is_empty()
                && def.value.as_ref().is_some_and(|value| {
                    set_key_type_field(
                        ki,
                        info,
                        type_0,
                        ki.keymap.ctx.atom_text(lhs.field),
                        lhs.index,
                        value,
                    )
                })
    })
}
fn handle_type_global_var(ki: &XkbKeymapInfo<'_>, stmt: &VarDef) -> bool {
    let lhs = some_or_false!(stmt.name.as_ref().and_then(expr_resolve_lhs));
    let elem = ki.keymap.ctx.atom_text(lhs.element);
    let field = ki.keymap.ctx.atom_text(lhs.field);
    if elem.eq_ignore_ascii_case("type") {
        true
    } else if !elem.is_empty() {
        !ki.keymap.strict
    } else {
        !field.is_empty() && !ki.keymap.strict
    }
}
fn handle_key_types_file(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    file: &mut XkbFile,
) -> bool {
    for stmt in &mut file.defs {
        let ok = match stmt {
            Statement::Include(incl) => handle_include_key_types(ki, info, incl),
            Statement::KeyType(def) => {
                let mut type_0 = XkbKeyType {
                    name: def.name,
                    num_levels: 1,
                    ..Default::default()
                };
                if !handle_key_type_body(ki, info, &def.body, &mut type_0) {
                    false
                } else {
                    add_key_type(info, type_0, def.merge);
                    true
                }
            }
            Statement::Var(var) => handle_type_global_var(ki, var),
            Statement::VMods(vmods) => vmods
                .iter()
                .all(|vmod| handle_vmod_def(&mut ki.keymap.ctx, &mut info.mods, vmod)),
            Statement::Unknown => !ki.keymap.strict,
            _ => false,
        };
        if !ok {
            return false;
        }
    }
    true
}
pub(crate) fn compile_key_types(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut info = key_types_info(0, &mods);
    if file.is_some_and(|file| !handle_key_types_file(keymap_info, &mut info, file)) {
        return false;
    }
    keymap_info.keymap.types = if info.types.is_empty() {
        vec![XkbKeyType {
            name: keymap_info.keymap.ctx.atom_intern(b"ONE_LEVEL"),
            num_levels: 1,
            ..Default::default()
        }]
    } else {
        info.types
    };
    keymap_info.keymap.mods = info.mods;
    true
}
pub(crate) fn init_vmods(info: &mut XkbModSet, mods: &XkbModSet, reset: bool) {
    *info = *mods;
    if !reset {
        return;
    }
    for vmod in 0..info.num_mods as usize {
        info.mods[vmod].mapping = 0;
    }
    info.explicit_vmods = 0;
}
pub(crate) fn merge_mod_sets(into: &mut XkbModSet, from: &XkbModSet, merge: MergeMode) {
    let clobber: bool = merge != MergeMode::Augment;
    for vmod in 0..from.num_mods as usize {
        let mod_0 = &from.mods[vmod];
        let mask: u32 = 1_u32 << vmod;
        if mod_0.type_0 != MOD_VIRT {
            continue;
        }
        if into.mods[vmod].type_0 == 0 {
            into.mods[vmod] = *mod_0;
            if from.explicit_vmods & mask != 0 {
                into.explicit_vmods |= mask;
            }
        } else if from.explicit_vmods & mask != 0 {
            if into.explicit_vmods & mask == 0 {
                into.mods[vmod].mapping = mod_0.mapping;
                into.explicit_vmods |= mask;
            } else if clobber {
                into.mods[vmod].mapping = mod_0.mapping;
            }
        }
    }
    into.num_mods = from.num_mods;
}
pub(crate) fn handle_vmod_def(ctx: &mut XkbContext, mods: &mut XkbModSet, stmt: &VModDef) -> bool {
    let mapping = match &stmt.value {
        Some(value) => some_or_false!(expr_resolve_mod_mask(ctx, value, MOD_REAL, mods)),
        None => 0,
    };
    if let Some(vmod) = (0..mods.num_mods as usize).find(|&i| mods.mods[i].name == stmt.name) {
        if mods.mods[vmod].type_0 != MOD_VIRT {
            return false;
        }
        let mask = 1 << vmod;
        if stmt.value.is_some() {
            if mods.explicit_vmods & mask == 0 || stmt.merge != MergeMode::Augment {
                mods.mods[vmod].mapping = mapping;
            }
            mods.explicit_vmods |= mask;
        }
        return true;
    }
    if mods.num_mods >= XKB_MAX_MODS {
        return false;
    }
    mods.mods[mods.num_mods as usize].name = stmt.name;
    mods.mods[mods.num_mods as usize].type_0 = MOD_VIRT;
    mods.mods[mods.num_mods as usize].mapping = mapping;
    if stmt.value.is_some() {
        mods.explicit_vmods |= 1 << mods.num_mods;
    }
    mods.num_mods += 1;
    true
}
#[derive(Default)]
pub(crate) struct KeyNamesInfo {
    include_depth: u32,
    codes: Vec<u32>,
    names: Vec<u32>,
}
const KEY_ALIAS: u32 = 1 << 31;
fn name_slot(info: &mut KeyNamesInfo, name: u32) -> &mut u32 {
    if name as usize >= info.names.len() {
        info.names.resize(name as usize + 1, 0);
    }
    &mut info.names[name as usize]
}
fn remove_key_name(info: &mut KeyNamesInfo, name: u32) {
    let binding = info.names.get(name as usize).copied().unwrap_or(0);
    if binding != 0 && binding & KEY_ALIAS == 0 {
        let keycode = binding - 1;
        if info.codes.get(keycode as usize) == Some(&name) {
            info.codes[keycode as usize] = 0;
        }
        info.names[name as usize] = 0;
    }
}
fn add_key_name(info: &mut KeyNamesInfo, keycode: u32, name: u32, merge: MergeMode) -> bool {
    if keycode > XKB_KEYCODE_MAX_CONTIGUOUS {
        return false;
    }
    let clobber = merge != MergeMode::Augment;
    let binding = info.names.get(name as usize).copied().unwrap_or(0);
    if binding != 0 && binding != keycode + 1 {
        if !clobber {
            return true;
        }
        remove_key_name(info, name);
    }
    if keycode as usize >= info.codes.len() {
        info.codes.resize(keycode as usize + 1, 0);
    }
    let old = info.codes[keycode as usize];
    if old == name {
        return true;
    }
    if old != 0 {
        if !clobber {
            return true;
        }
        info.names[old as usize] = 0;
    }
    info.codes[keycode as usize] = name;
    *name_slot(info, name) = keycode + 1;
    true
}
fn handle_alias_def(info: &mut KeyNamesInfo, def: &KeyAliasDef) {
    let old = info.names.get(def.alias as usize).copied().unwrap_or(0);
    if old != 0 && def.merge == MergeMode::Augment {
        return;
    }
    if old != 0 && old & KEY_ALIAS == 0 {
        remove_key_name(info, def.alias);
    }
    *name_slot(info, def.alias) = (def.real != 0).then_some(KEY_ALIAS | def.real).unwrap_or(0);
}
fn merge_keycodes(into: &mut KeyNamesInfo, from: &KeyNamesInfo, merge: MergeMode) -> bool {
    for (keycode, &name) in from.codes.iter().enumerate() {
        if name != 0 && !add_key_name(into, keycode as u32, name, merge) {
            return false;
        }
    }
    for (alias, &binding) in from.names.iter().enumerate() {
        if binding & KEY_ALIAS != 0 {
            handle_alias_def(
                into,
                &KeyAliasDef {
                    merge,
                    alias: alias as u32,
                    real: binding & !KEY_ALIAS,
                },
            );
        }
    }
    true
}
fn handle_include_keycodes(
    info: &mut KeyNamesInfo,
    includes: &mut [IncludeStmt],
    ki: &mut XkbKeymapInfo<'_>,
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        return false;
    }
    let mut included = KeyNamesInfo {
        include_depth: info.include_depth + 1,
        ..Default::default()
    };
    for stmt in includes.iter() {
        let Some(mut file) = process_include_file(&mut ki.keymap.ctx, stmt, FileType::Keycodes)
        else {
            return false;
        };
        let mut next = KeyNamesInfo {
            include_depth: included.include_depth,
            ..Default::default()
        };
        if !handle_keycodes_file(&mut next, &mut file, ki)
            || !merge_keycodes(&mut included, &next, stmt.merge)
        {
            return false;
        }
    }
    if let Some(first) = includes.first() {
        return merge_keycodes(info, &included, first.merge);
    }
    true
}
fn handle_key_name_var(ki: &XkbKeymapInfo<'_>, stmt: &VarDef) -> bool {
    let Some(lhs) = stmt.name.as_ref().and_then(expr_resolve_lhs) else {
        return false;
    };
    let field = ki.keymap.ctx.atom_text(lhs.field);
    if lhs.element != 0 || lhs.index.is_some() {
        return !ki.keymap.strict;
    }
    matches!(field.to_ascii_lowercase().as_str(), "minimum" | "maximum")
        && stmt
            .value
            .as_ref()
            .and_then(|value| expr_resolve_integer(&ki.keymap.ctx, value))
            .is_some_and(|value| (0..=XKB_KEYCODE_MAX_CONTIGUOUS as i64).contains(&value))
        || !ki.keymap.strict
}
fn handle_keycodes_file(
    info: &mut KeyNamesInfo,
    file: &mut XkbFile,
    ki: &mut XkbKeymapInfo<'_>,
) -> bool {
    for statement in &mut file.defs {
        let valid = match statement {
            Statement::Include(includes) => handle_include_keycodes(info, includes, ki),
            Statement::Keycode(def) => {
                (0..=XKB_KEYCODE_MAX_CONTIGUOUS as i64).contains(&def.value)
                    && add_key_name(info, def.value as u32, def.name, def.merge)
            }
            Statement::KeyAlias(def) => {
                handle_alias_def(info, def);
                true
            }
            Statement::Var(def) => handle_key_name_var(ki, def),
            Statement::Unknown => !ki.keymap.strict,
            _ => false,
        };
        if !valid {
            return false;
        }
    }
    true
}
fn finish_keycodes(keymap: &mut XkbKeymap, info: &mut KeyNamesInfo) {
    if info.codes.is_empty() {
        info.codes.resize(256, 0);
    }
    keymap.min_key_code = info.codes.iter().position(|&name| name != 0).unwrap_or(8) as u32;
    keymap.keys.resize_with(info.codes.len(), XkbKey::default);
    let aliases: Vec<_> = info
        .names
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, binding)| binding & KEY_ALIAS != 0)
        .collect();
    for (keycode, &name) in info
        .codes
        .iter()
        .enumerate()
        .filter(|(_, name)| **name != 0)
    {
        info.names[name as usize] = keycode as u32 + 1;
    }
    for (name, alias) in aliases {
        if info.names[name] & KEY_ALIAS != 0
            && info
                .names
                .get((alias & !KEY_ALIAS) as usize)
                .map_or(true, |real| *real == 0)
        {
            info.names[name] = 0;
        }
    }
    keymap.key_names = std::mem::take(&mut info.names);
}
pub(crate) fn compile_keycodes(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mut info = KeyNamesInfo::default();
    if file.is_some_and(|file| !handle_keycodes_file(&mut info, file, keymap_info)) {
        return false;
    }
    finish_keycodes(keymap_info.keymap, &mut info);
    true
}
#[rustfmt::skip]
struct Lhs<'a> { element: u32, field: u32, index: Option<&'a ExprKind> }
fn expr_resolve_lhs(expr: &ExprKind) -> Option<Lhs<'_>> {
    match expr {
        ExprKind::Ident(field) if *field != 0 => Some(Lhs {
            element: 0,
            field: *field,
            index: None,
        }),
        ExprKind::FieldRef {
            element,
            field,
            index,
        } if *field != 0 && (*element != 0 || index.is_some()) => Some(Lhs {
            element: *element,
            field: *field,
            index: index.as_deref(),
        }),
        _ => None,
    }
}
pub(crate) fn expr_resolve_boolean(ctx: &XkbContext, expr: &ExprKind) -> Option<bool> {
    match expr {
        ExprKind::Boolean(value) => Some(*value),
        ExprKind::Ident(atom) => named_bool(ctx.atom_text(*atom)),
        ExprKind::Unary {
            child,
            op: UnaryOp::Not | UnaryOp::Invert,
        } => expr_resolve_boolean(ctx, child).map(|value| !value),
        _ => None,
    }
}
fn named_bool(value: &str) -> Option<bool> {
    ["true", "yes", "on"]
        .iter()
        .any(|name| value.eq_ignore_ascii_case(name))
        .then_some(true)
        .or_else(|| {
            ["false", "no", "off"]
                .iter()
                .any(|name| value.eq_ignore_ascii_case(name))
                .then_some(false)
        })
}
fn eval_integer(expr: &ExprKind, lookup: &dyn Fn(u32) -> Option<i64>) -> Option<i64> {
    match expr {
        ExprKind::Integer(value) => Some(*value),
        ExprKind::Ident(atom) => lookup(*atom),
        ExprKind::Binary {
            left,
            right,
            op: op @ (BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide),
        } => {
            let left = eval_integer(left, lookup)?;
            let right = eval_integer(right, lookup)?;
            match op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Subtract => left.checked_sub(right),
                BinaryOp::Multiply => left.checked_mul(right),
                BinaryOp::Divide => left.checked_div(right),
                BinaryOp::Assign => unreachable!(),
            }
        }
        ExprKind::Unary { child, op } => {
            let value = eval_integer(child, lookup)?;
            match op {
                UnaryOp::Invert => Some(!value),
                UnaryOp::Negate => value.checked_neg(),
                UnaryOp::Plus => Some(value),
                UnaryOp::Not => None,
            }
        }
        _ => None,
    }
}
pub(crate) fn expr_resolve_integer(_ctx: &XkbContext, expr: &ExprKind) -> Option<i64> {
    eval_integer(expr, &|_| None)
}
fn named_number(name: &str, prefix: &str, max: u32) -> Option<i64> {
    let suffix = name.get(prefix.len()..)?;
    name.get(..prefix.len())?
        .eq_ignore_ascii_case(prefix)
        .then_some(())?;
    let value = suffix.parse::<u32>().ok()?;
    (1..=max).contains(&value).then_some(value as i64)
}
fn expr_resolve_group(
    keymap_info: &XkbKeymapInfo<'_>,
    expr: &ExprKind,
    absolute: bool,
) -> Option<(u32, bool)> {
    if let ExprKind::Ident(atom) = expr {
        if keymap_info
            .keymap
            .ctx
            .atom_text(*atom)
            .eq_ignore_ascii_case("last")
            && keymap_info.keymap.num_groups == 0
        {
            return Some((0, true));
        }
    }
    let max = XKB_MAX_GROUPS;
    let num_groups = keymap_info.keymap.num_groups;
    let ctx = &keymap_info.keymap.ctx;
    let value = eval_integer(expr, &|atom| {
        let name = ctx.atom_text(atom);
        named_number(name, "Group", max).or_else(|| {
            if name.eq_ignore_ascii_case("first") {
                Some(1)
            } else if name.eq_ignore_ascii_case("last") && num_groups != 0 {
                Some(num_groups as i64)
            } else {
                None
            }
        })
    })?;
    if value > max as i64 || (absolute && value < 1) || (!absolute && value < -(max as i64)) {
        return None;
    }
    Some((value as u32, false))
}
pub(crate) fn expr_resolve_level(ctx: &XkbContext, expr: &ExprKind) -> Option<u32> {
    let value = eval_integer(expr, &|atom| {
        named_number(ctx.atom_text(atom), "Level", XKB_LEVEL_MAX_IMPL)
    })?;
    (1..=XKB_LEVEL_MAX_IMPL as i64)
        .contains(&value)
        .then_some(value as u32 - 1)
}
pub(crate) fn expr_resolve_string(expr: &ExprKind) -> Option<u32> {
    if let ExprKind::String(value) = expr {
        Some(*value)
    } else {
        None
    }
}
fn expr_resolve_repeat(ctx: &XkbContext, expr: &ExprKind) -> Option<Option<bool>> {
    let ExprKind::Ident(atom) = expr else {
        return None;
    };
    let value = ctx.atom_text(*atom);
    if value.eq_ignore_ascii_case("default") {
        Some(None)
    } else {
        named_bool(value).map(Some)
    }
}
fn expr_resolve_mod_mask(
    ctx: &XkbContext,
    expr: &ExprKind,
    mod_type: u32,
    mods: &XkbModSet,
) -> Option<u32> {
    match expr {
        ExprKind::Integer(value) => u32::try_from(*value).ok(),
        ExprKind::Ident(atom) => match ctx.atom_text(*atom) {
            value if value.eq_ignore_ascii_case("all") => Some(MOD_REAL_MASK_ALL),
            value if value.eq_ignore_ascii_case("none") => Some(0),
            _ => xkb_mod_name_to_index(mods, *atom, mod_type).map(|index| 1 << index),
        },
        ExprKind::Binary { left, right, op } => {
            let left = expr_resolve_mod_mask(ctx, left, mod_type, mods)?;
            let right = expr_resolve_mod_mask(ctx, right, mod_type, mods)?;
            match op {
                BinaryOp::Add => Some(left | right),
                BinaryOp::Subtract => Some(left & !right),
                _ => None,
            }
        }
        ExprKind::Unary {
            op: UnaryOp::Invert,
            child,
        } => expr_resolve_mod_mask(ctx, child, mod_type, mods).map(|value| !value),
        _ => None,
    }
}
