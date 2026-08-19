use std::collections::BTreeMap;

pub(crate) use super::keymap::xkb_mod_name_to_index;
use super::keymap::GROUP_LAST_INDEX_NAME;
use super::keysym::xkb_keysym_is_keypad;
use super::keysym::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
use super::parser::{exceeds_include_max_depth, process_include_file};
pub(crate) use super::parser::{KeyAliasDef, KeycodeDef, ModMapDef, NamedVarDef};

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
macro_rules! include_file {
    ($ki:expr, $info:expr, $stmt:expr, $type:expr) => {
        match process_include_file(&mut $ki.keymap.ctx, $stmt, $type) {
            Some(file) => file,
            None => {
                $info.error_count += 10;
                return false;
            }
        }
    };
}

#[derive(Default)]
pub(crate) struct SymbolsBuilder {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) explicit_group: Option<u32>,
    pub(crate) max_groups: u32,
    pub(crate) keys: Vec<Option<KeyInfo>>,
    pub(crate) default_key: KeyInfo,
    pub(crate) default_actions: ActionsInfo,
    pub(crate) group_names: Vec<u32>,
    pub(crate) modmaps: Vec<(ModMapTarget, ModMapEntry)>,
    pub(crate) mods: XkbModSet,
    pub(crate) star_atom: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ModMapTarget {
    Key(u32),
    Symbol(u32),
}

#[derive(Copy, Clone)]
pub(crate) struct ModMapEntry {
    pub(crate) merge: MergeMode,
    pub(crate) modifier: u32,
}
#[derive(Clone, Default)]
pub(crate) struct KeyInfo {
    pub(crate) name: u32,
    pub(crate) vmodmap: Option<u32>,
    pub(crate) default_type: Option<u32>,
    pub(crate) repeat: Option<bool>,
    pub(crate) out_of_range: Option<OutOfRangeInfo>,
    pub(crate) groups: Vec<GroupInfo>,
    pub(crate) merge: MergeMode,
}
#[derive(Clone, Copy, Default)]
pub(crate) struct OutOfRangeInfo {
    pub(crate) policy: u32,
    pub(crate) number: u32,
    pub(crate) pending: bool,
}

impl KeyInfo {
    fn has_any_field(&self) -> bool {
        self.default_type.is_some()
            || self.vmodmap.is_some()
            || self.repeat.is_some()
            || self.out_of_range.is_some()
    }
}
#[derive(Clone, Default)]
pub(crate) struct GroupInfo {
    pub(crate) levels: Vec<XkbLevel>,
    pub(crate) explicit_syms: bool,
    pub(crate) explicit_acts: bool,
    pub(crate) type_0: Option<u32>,
}

#[derive(Clone, Copy)]
enum GroupField {
    Syms,
    Acts,
}

impl GroupInfo {
    fn field_is_set(&self, field: GroupField) -> bool {
        match field {
            GroupField::Syms => self.explicit_syms,
            GroupField::Acts => self.explicit_acts,
        }
    }

    fn has_any_field(&self) -> bool {
        self.explicit_syms || self.explicit_acts || self.type_0.is_some()
    }
}

impl SymbolsBuilder {
    fn new(ki: &mut XkbKeymapInfo<'_>, include_depth: u32, mods: &XkbModSet) -> Self {
        let star_atom = ki.keymap.ctx.atom_intern(b"*");
        let mut builder = Self {
            include_depth,
            max_groups: XKB_MAX_GROUPS,
            default_key: KeyInfo {
                name: star_atom,
                ..Default::default()
            },
            star_atom,
            ..Default::default()
        };
        init_actions_info(&mut builder.default_actions);
        init_vmods(&mut builder.mods, mods, include_depth > 0);
        builder
    }

    fn add_key(&mut self, ki: &mut XkbKeymapInfo<'_>, key: &mut KeyInfo) -> bool {
        if let Some(alias) = ki
            .keymap
            .key_names
            .get(key.name as usize)
            .copied()
            .filter(|entry| entry.found && entry.is_alias)
        {
            key.name = alias.index;
        }
        let slot = match ki.keymap.key_names.get(key.name as usize) {
            Some(entry) if entry.found && !entry.is_alias => entry.index as usize,
            _ => {
                init_key_info_with_atom(key, self.star_atom);
                return true;
            }
        };
        if slot >= self.keys.len() {
            self.keys.resize_with(slot + 1, || None);
        }
        if let Some(existing) = &mut self.keys[slot] {
            merge_keys(ki, self.star_atom, existing, key)
        } else {
            self.keys[slot] = Some(std::mem::take(key));
            init_key_info_with_atom(key, self.star_atom);
            true
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
        let mut errors = 0;
        for (key_idx, key) in self.keys.iter_mut().enumerate() {
            if key
                .as_mut()
                .is_some_and(|key| !copy_symbols_def_to_keymap(keymap, key_idx, key))
            {
                errors += 1;
            }
        }
        let start = if keymap.num_keys_low == 0 {
            0
        } else {
            keymap.min_key_code as usize
        };
        for &(target, modmap) in &self.modmaps {
            let key = match target {
                ModMapTarget::Symbol(symbol) => find_key_by_symbol(keymap, start, symbol)
                    .and_then(|index| keymap.keys.get_mut(index)),
                ModMapTarget::Key(name) => keymap.key_by_name_mut(name, true),
            };
            let Some(key) = key else {
                errors += 1;
                continue;
            };
            if modmap.modifier != XKB_MOD_NONE {
                key.modmap |= 1 << modmap.modifier;
            }
        }
        self.error_count += errors;
    }

    fn merge_from(&mut self, ki: &mut XkbKeymapInfo<'_>, from: &mut Self, merge: MergeMode) {
        if from.error_count > 0 {
            self.error_count += from.error_count;
            return;
        }

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
                key.merge = merge;

                if !self.add_key(ki, &mut key) {
                    self.error_count += 1;
                }
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
            self.error_count += 10;
            return false;
        }
        let mut included = Self::new(ki, self.include_depth + 1, &self.mods);
        let include_statements = &mut *includes;
        for statement in include_statements {
            let Some(mut file) =
                process_include_file(&mut ki.keymap.ctx, statement, FileType::Symbols)
            else {
                self.error_count += 10;
                return false;
            };
            let mut next = Self::new(ki, self.include_depth + 1, &included.mods);
            next.explicit_group = if !statement.modifier.is_empty() {
                statement
                    .modifier
                    .parse::<i32>()
                    .ok()
                    .and_then(|group| group.checked_sub(1))
                    .and_then(|group| ((group as u32) < self.max_groups).then_some(group as u32))
                    .or(self.explicit_group)
            } else if ki.keymap.num_groups != 0 && next.include_depth == 1 {
                Some(0)
            } else {
                self.explicit_group
            };
            next.compile_file(ki, &mut file);
            included.merge_from(ki, &mut next, statement.merge);
        }
        if let Some(first) = includes.first() {
            self.merge_from(ki, &mut included, first.merge);
        }
        self.error_count == 0
    }

    fn compile_file(&mut self, ki: &mut XkbKeymapInfo<'_>, file: &mut XkbFile) {
        for statement in &mut file.defs {
            let valid = match statement {
                Statement::Include(includes) => self.include(ki, includes),
                Statement::Symbols(definition) => self.compile_key(ki, definition),
                Statement::Var(variable) => self.compile_global(ki, variable),
                Statement::VMod(vmod) => handle_vmod_def(&mut ki.keymap.ctx, &mut self.mods, vmod),
                Statement::ModMap(definition) => self.compile_modmap(ki, definition),
                Statement::Unknown => ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0,
                _ => false,
            };
            if !valid {
                self.error_count += 1;
            }
            if self.error_count > 10 {
                break;
            }
        }
    }

    fn compile_key(&mut self, ki: &mut XkbKeymapInfo<'_>, stmt: &mut NamedVarDef) -> bool {
        // Clone scalar fields from default_key, deep-copy groups
        let dk = &self.default_key;
        let mut keyi = dk.clone();
        keyi.merge = stmt.merge;
        keyi.name = stmt.name;
        if self.compile_key_body(ki, &mut stmt.body, &mut keyi) {
            set_explicit_group(self, &mut keyi);
            if self.add_key(ki, &mut keyi) {
                return true;
            }
        }
        self.error_count += 1;
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
        let ret: bool;
        let lhs = some_or_false!(expr_resolve_lhs(stmt.name.as_ref().unwrap()));
        let elem_atom = lhs.element;
        let field_atom = lhs.field;
        let array_ndx_opt = lhs.index;
        let elem = ki.keymap.ctx.atom_text(elem_atom).to_owned();
        let field = ki.keymap.ctx.atom_text(field_atom).to_owned();
        if !elem.is_empty() && elem.eq_ignore_ascii_case("key") {
            let mut temp: KeyInfo = KeyInfo::default();
            init_key_info_with_atom(&mut temp, ki.keymap.ctx.atom_intern(b"*"));
            temp.merge = if temp.merge == MergeMode::Replace {
                MergeMode::Override
            } else {
                stmt.merge
            };
            ret = set_symbols_field(ki, self, &mut temp, &field, array_ndx_opt, &mut stmt.value);
            let mut dk = std::mem::take(&mut self.default_key);
            merge_keys(ki, self.star_atom, &mut dk, &mut temp);
            self.default_key = dk;
        } else if elem.is_empty()
            && (field.eq_ignore_ascii_case("name") || field.eq_ignore_ascii_case("groupname"))
        {
            ret = set_group_name(
                ki,
                self,
                array_ndx_opt,
                stmt.value.as_ref().unwrap(),
                stmt.merge,
            );
        } else if elem.is_empty()
            && [
                "groupswrap",
                "wrapgroups",
                "groupsclamp",
                "clampgroups",
                "groupsredirect",
                "redirectgroups",
                "allownone",
            ]
            .iter()
            .any(|name| field.eq_ignore_ascii_case(name))
        {
            ret = true;
        } else if !elem.is_empty() {
            ret = {
                set_default_action_field(
                    ki,
                    &mut self.default_actions,
                    &elem,
                    &field,
                    array_ndx_opt,
                    &mut stmt.value,
                    stmt.merge,
                ) != ParseStatus::Fatal
            };
        } else {
            return ki.strict & PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS == 0;
        }
        ret
    }

    fn compile_key_body(
        &mut self,
        ki: &mut XkbKeymapInfo<'_>,
        defs: &mut [VarDef],
        keyi: &mut KeyInfo,
    ) -> bool {
        let mut all_valid_entries: bool = true;
        for def in defs.iter_mut() {
            let field_owned: String;
            let field: &str;
            let mut array_ndx_opt: Option<&ExprKind> = None;
            let mut ok: bool = true;
            if let Some(name) = def.name.as_ref() {
                if let Some(lhs) = expr_resolve_lhs(name) {
                    array_ndx_opt = lhs.index;
                    let elem = ki.keymap.ctx.atom_text(lhs.element);
                    field_owned = ki.keymap.ctx.atom_text(lhs.field).to_owned();
                    field = &field_owned;
                    if !elem.is_empty() {
                        ok = false;
                    }
                } else {
                    field_owned = String::new();
                    field = &field_owned;
                    ok = false;
                }
            } else if def.value.is_none() || !is_action_list_value(def.value.as_ref().unwrap()) {
                field = "symbols";
            } else {
                field = "actions";
            }
            if def.value.is_none() {
                ok = false;
            }
            if !ok || !set_symbols_field(ki, self, keyi, field, array_ndx_opt, &mut def.value) {
                all_valid_entries = false;
            }
        }
        all_valid_entries
    }
}

fn is_action_list_value(value: &ExprKind) -> bool {
    match value {
        ExprKind::ActionList { actions } => match actions.first() {
            None => true,
            Some(first) => matches!(first, ExprKind::ActionList { .. } | ExprKind::Action { .. }),
        },
        _ => false,
    }
}

/// Extract child expressions from an ActionList container node, or return a single-element slice.
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
fn merge_groups(into: &mut GroupInfo, from: &mut GroupInfo, clobber: bool) -> bool {
    if let Some(from_type) = from.type_0 {
        if into.type_0.is_none() || clobber {
            into.type_0 = Some(from_type);
        }
    }
    if from.levels.is_empty() {
        *from = GroupInfo::default();
        return true;
    }
    if into.levels.is_empty() {
        from.type_0 = into.type_0;
        *into = std::mem::take(from);
        return true;
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
        if from_actions_count == into.levels.len() as u32 {
            into.explicit_acts = false;
        }
        if from.explicit_acts {
            into.explicit_acts = true;
        }
    }
    true
}
fn merge_keys(
    _ki: &XkbKeymapInfo<'_>,
    star_atom: u32,
    into: &mut KeyInfo,
    from: &mut KeyInfo,
) -> bool {
    let clobber: bool = from.merge != MergeMode::Augment;
    if from.merge == MergeMode::Replace {
        std::mem::swap(into, from);
        init_key_info_with_atom(from, star_atom);
        return true;
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
    true
}
fn group_index(
    ki: &mut XkbKeymapInfo<'_>,
    info: &SymbolsBuilder,
    key: &mut KeyInfo,
    index: Option<&ExprKind>,
    field: GroupField,
) -> Option<usize> {
    let index = match index {
        Some(expr) => {
            let mut group = 0;
            let mut pending = false;
            if expr_resolve_group(ki, expr, false, &mut group, &mut pending) != ParseStatus::Success
            {
                return None;
            }
            group.checked_sub(1)? as usize
        }
        None => key
            .groups
            .iter()
            .position(|group| !group.field_is_set(field))
            .unwrap_or(key.groups.len()),
    };
    if index >= info.max_groups as usize {
        return None;
    }
    if index >= key.groups.len() {
        key.groups.resize_with(index + 1, GroupInfo::default);
    }
    Some(index)
}
fn add_symbols_to_key(
    ki: &mut XkbKeymapInfo<'_>,
    info: &SymbolsBuilder,
    key: &mut KeyInfo,
    array_index: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let Some(group_index) = group_index(ki, info, key, array_index, GroupField::Syms) else {
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
fn add_actions_to_key(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut SymbolsBuilder,
    key: &mut KeyInfo,
    array_index: Option<&ExprKind>,
    value: &mut ExprKind,
) -> bool {
    let Some(group_index) = group_index(ki, info, key, array_index, GroupField::Acts) else {
        return false;
    };

    let group = &mut key.groups[group_index];

    if matches!(value, ExprKind::EmptyList) {
        group.explicit_acts = true;
        return true;
    }

    if !matches!(value, ExprKind::ActionList { .. }) || group.explicit_acts {
        return false;
    }

    let ExprKind::ActionList { actions: nodes } = value else {
        unreachable!();
    };

    group.levels.resize_with(nodes.len(), XkbLevel::default);

    group.explicit_acts = true;

    let mut used_levels = 0;

    for (level_index, node) in nodes.iter_mut().enumerate() {
        let ExprKind::ActionList { actions } = node else {
            return false;
        };

        let level = &mut group.levels[level_index];
        level.action = None;

        for expression in actions {
            let mut action = XkbAction::None;

            match handle_action_def(ki, &mut info.default_actions, expression, &mut action) {
                ParseStatus::Fatal => return false,
                ParseStatus::Success if !matches!(action, XkbAction::None) => {
                    level.action = Some(action);
                    break;
                }
                _ => {}
            }
        }

        if level.action.is_some() || !level.syms.is_empty() {
            used_levels = level_index + 1;
        }
    }

    group.levels.truncate(used_levels);
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
    field: &str,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<ExprKind>,
) -> bool {
    let mapped_field = match parse_symbols_field(field) {
        Some(f) => f,
        None => return ki.strict & PARSER_NO_UNKNOWN_KEY_FIELDS == 0,
    };

    match mapped_field {
        SymbolsField::Type => {
            let mut ndx: u32 = 0;
            let val = some_or_false!(expr_resolve_string(value_opt.as_ref().unwrap()));
            if let Some(array_ndx) = array_ndx {
                let mut _pending = false;
                if expr_resolve_group(ki, array_ndx, false, &mut ndx, &mut _pending)
                    != ParseStatus::Success
                {
                    return false;
                }
                ndx -= 1;
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
            return add_symbols_to_key(ki, info, keyi, array_ndx, value_opt.as_ref().unwrap());
        }
        SymbolsField::Actions => {
            return add_actions_to_key(ki, info, keyi, array_ndx, value_opt.as_mut().unwrap());
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
            let mut grp: u32 = 0;
            let mut pending: bool = false;
            if expr_resolve_group(
                ki,
                value_opt.as_ref().unwrap(),
                false,
                &mut grp,
                &mut pending,
            ) != ParseStatus::Success
                && !pending
            {
                return false;
            }
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
    let array_ndx = match array_ndx {
        Some(a) => a,
        None => {
            return false;
        }
    };
    let mut group: u32 = 0;
    let mut _pending: bool = false;
    if { expr_resolve_group(ki, array_ndx, false, &mut group, &mut _pending) }
        != ParseStatus::Success
    {
        return false;
    }
    let mut name = some_or_false!(expr_resolve_string(value));
    let group_to_use = match info.explicit_group {
        None => group.wrapping_sub(1),
        Some(explicit_group) if group.wrapping_sub(1) == 0 => explicit_group,
        Some(_) => return false,
    };
    if group_to_use >= info.group_names.len() as u32 {
        info.group_names.resize((group_to_use as usize) + 1, 0_u32);
    } else {
        let old_name: u32 = info.group_names[group_to_use as usize];
        if old_name != XKB_ATOM_NONE && old_name != name {
            name = if merge != MergeMode::Augment {
                name
            } else {
                old_name
            };
        }
    }
    info.group_names[group_to_use as usize] = name;
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
#[inline]
fn first_symbol(group: &GroupInfo, level: usize) -> u32 {
    group
        .levels
        .get(level)
        .and_then(|level| level.syms.first())
        .copied()
        .unwrap_or(XKB_KEY_NO_SYMBOL)
}
fn find_automatic_type(ctx: &mut XkbContext, group: &GroupInfo) -> u32 {
    let width = group.levels.len();
    match width {
        0 | 1 => ctx.atom_intern(b"ONE_LEVEL"),
        2 => {
            let first = first_symbol(group, 0);
            let second = first_symbol(group, 1);
            if xkb_keysym_is_lower(first) && xkb_keysym_is_upper_or_title(second) {
                ctx.atom_intern(b"ALPHABETIC")
            } else if xkb_keysym_is_keypad(first) || xkb_keysym_is_keypad(second) {
                ctx.atom_intern(b"KEYPAD")
            } else {
                ctx.atom_intern(b"TWO_LEVEL")
            }
        }
        3 | 4 => {
            let first = first_symbol(group, 0);
            let second = first_symbol(group, 1);
            if xkb_keysym_is_lower(first) && xkb_keysym_is_upper_or_title(second) {
                let third = first_symbol(group, 2);
                let fourth = first_symbol(group, 3);
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
fn copy_symbols_def_to_keymap(keymap: &mut XkbKeymap, key_idx: usize, keyi: &mut KeyInfo) -> bool {
    keymap.keys[key_idx].num_groups = 0;
    if !keyi.groups.is_empty() {
        for (idx, groupi) in keyi.groups.iter().enumerate() {
            let has_explicit_type = keyi.default_type.is_some() || groupi.type_0.is_some();
            if !groupi.levels.is_empty() || has_explicit_type {
                keymap.keys[key_idx].num_groups = (idx as u32) + 1;
            }
        }
    }

    if keymap.keys[key_idx].num_groups == 0 {
        if !keyi.has_any_field() {
            return false;
        }
    } else {
        let num_groups = keymap.keys[key_idx].num_groups as usize;
        keyi.groups.resize_with(num_groups, Default::default);

        for i in 1..keyi.groups.len() {
            if !keyi.groups[i].has_any_field() {
                keyi.groups[i] = keyi.groups[0].clone();
            }
        }

        keymap.keys[key_idx].groups = (0..num_groups).map(|_| XkbGroup::default()).collect();

        for i in 0..keyi.groups.len() as u32 {
            let type_idx = find_type_for_group(keymap, keyi, i as usize) as u32;

            if keymap.types[type_idx as usize].num_levels
                < keyi.groups[i as usize].levels.len() as u32
            {
                for lvl_idx in keymap.types[type_idx as usize].num_levels as usize
                    ..keyi.groups[i as usize].levels.len()
                {
                    keyi.groups[i as usize].levels[lvl_idx].syms.clear();
                    keyi.groups[i as usize].levels[lvl_idx].action = None;
                }
            }

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
                        let orig = leveli.syms.len();
                        leveli.syms.reserve(orig);
                        for i in 0..orig {
                            leveli.syms.push(xkb_keysym_to_upper(leveli.syms[i]));
                        }
                    }
                }
            }
            keymap.keys[key_idx].groups[i].levels = std::mem::take(&mut groupi.levels);
            if groupi.explicit_acts {
                keymap.keys[key_idx].groups[i].explicit_actions = true;
            }
        }

        if let Some(oor) = keyi.out_of_range {
            keymap.keys[key_idx].out_of_range_pending_group = oor.pending;
            keymap.keys[key_idx].out_of_range_group_number = oor.number;
            keymap.keys[key_idx].out_of_range_group_policy = oor.policy;
        }
    }

    if let Some(vmodmap) = keyi.vmodmap {
        keymap.keys[key_idx].vmodmap = vmodmap;
        keymap.keys[key_idx].explicit_vmodmap = true;
    }

    if let Some(repeat) = keyi.repeat {
        keymap.keys[key_idx].repeats = repeat;
        keymap.keys[key_idx].explicit_repeat = true;
    }

    true
}
fn find_key_by_symbol(keymap: &XkbKeymap, start: usize, sym: u32) -> Option<usize> {
    for ki in start..keymap.num_keys.min(keymap.keys.len() as u32) as usize {
        let key = &keymap.keys[ki];
        for gi in 0..key.num_groups.min(key.groups.len() as u32) {
            let g = &key.groups[gi as usize];
            let num_levels = keymap
                .types
                .get(g.type_idx as usize)
                .map_or(0, |t| t.num_levels);
            for li in 0..num_levels.min(g.levels.len() as u32) {
                for &s in &g.levels[li as usize].syms {
                    if s == sym {
                        return Some(ki);
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn compile_symbols(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut builder = SymbolsBuilder::new(keymap_info, 0, &mods);
    if let Some(file) = file {
        builder.compile_file(keymap_info, file);
    }
    if builder.error_count != 0 {
        return false;
    }
    builder.finish(keymap_info.keymap);
    true
}
use super::keysym::xkb_keysym_to_upper;
use super::parser::*;
#[derive(Default)]
pub(crate) struct KeyTypesInfo {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) types: Vec<KeyTypeInfo>,
    type_index: Vec<Option<usize>>,
    pub(crate) mods: XkbModSet,
}

#[derive(Clone, Default)]
pub(crate) struct KeyTypeInfo {
    pub(crate) merge: MergeMode,
    pub(crate) modifiers_set: bool,
    pub(crate) def: XkbKeyType,
}
fn key_types_info(include_depth: u32, mods: &XkbModSet) -> KeyTypesInfo {
    let mut info = KeyTypesInfo {
        include_depth,
        ..Default::default()
    };
    init_vmods(&mut info.mods, mods, include_depth > 0);
    info
}
fn add_key_type(info: &mut KeyTypesInfo, new: &mut KeyTypeInfo) {
    let name = new.def.name as usize;
    if name >= info.type_index.len() {
        info.type_index.resize(name + 1, None);
    }
    if let Some(index) = info.type_index[name] {
        let existing = &mut info.types[index];
        if new.merge != MergeMode::Augment {
            std::mem::swap(existing, new);
        }
        return;
    }
    info.type_index[name] = Some(info.types.len());
    info.types.push(std::mem::take(new));
}
fn merge_included_key_types(into: &mut KeyTypesInfo, from: &mut KeyTypesInfo, merge: MergeMode) {
    if from.error_count > 0 {
        into.error_count += from.error_count;
        return;
    }
    merge_mod_sets(&mut into.mods, &from.mods, merge);
    if into.types.is_empty() {
        into.types = std::mem::take(&mut from.types);
        into.type_index = std::mem::take(&mut from.type_index);
    } else {
        for mut type_0 in from.types.drain(..) {
            type_0.merge = merge;
            add_key_type(into, &mut type_0);
        }
        from.type_index.clear();
    }
}
fn handle_include_key_types(
    ki: &mut XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    includes: &mut [IncludeStmt],
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    let mut included = key_types_info(info.include_depth.wrapping_add(1), &info.mods);
    for stmt in includes.iter() {
        let mut file = include_file!(ki, info, stmt, FileType::Types);
        let mut next = key_types_info(info.include_depth.wrapping_add(1), &included.mods);
        handle_key_types_file(ki, &mut next, &mut file);
        merge_included_key_types(&mut included, &mut next, stmt.merge);
    }
    if let Some(first) = includes.first() {
        merge_included_key_types(info, &mut included, first.merge);
    }
    info.error_count == 0
}
fn add_map_entry(type_0: &mut KeyTypeInfo, new: &XkbKeyTypeEntry) {
    type_0.def.num_levels = type_0.def.num_levels.max(new.level + 1);
    if let Some(old) = type_0
        .def
        .entries
        .iter_mut()
        .find(|entry| entry.mods.mods == new.mods.mods)
    {
        old.level = new.level;
    } else {
        type_0.def.entries.push(*new);
    }
}
fn set_key_type_field(
    ki: &XkbKeymapInfo<'_>,
    info: &mut KeyTypesInfo,
    type_0: &mut KeyTypeInfo,
    field: &str,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if field.eq_ignore_ascii_case("modifiers") {
        if array_ndx.is_some() || type_0.modifiers_set {
            return false;
        }
        type_0.def.mods.mods = some_or_false!(expr_resolve_mod_mask(
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
        return ki.strict & PARSER_NO_UNKNOWN_TYPE_FIELDS == 0;
    }
    let Some(array_ndx) = array_ndx else {
        return false;
    };
    let mods = some_or_false!(expr_resolve_mod_mask(
        &ki.keymap.ctx,
        array_ndx,
        MOD_BOTH,
        &info.mods
    )) & type_0.def.mods.mods;
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
            .def
            .entries
            .iter_mut()
            .find(|entry| entry.mods.mods == mods)
        {
            entry.preserve.mods = preserve;
        } else {
            type_0.def.entries.push(XkbKeyTypeEntry {
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
    type_0: &mut KeyTypeInfo,
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
        ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0
    } else {
        !field.is_empty() && ki.strict & PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS == 0
    }
}
fn handle_key_types_file(ki: &mut XkbKeymapInfo<'_>, info: &mut KeyTypesInfo, file: &mut XkbFile) {
    for stmt in &mut file.defs {
        let ok = match stmt {
            Statement::Include(incl) => handle_include_key_types(ki, info, incl),
            Statement::KeyType(def) => {
                let mut type_0 = KeyTypeInfo {
                    merge: def.merge,
                    def: XkbKeyType {
                        name: def.name,
                        num_levels: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                if !handle_key_type_body(ki, info, &def.body, &mut type_0) {
                    false
                } else {
                    add_key_type(info, &mut type_0);
                    true
                }
            }
            Statement::Var(var) => handle_type_global_var(ki, var),
            Statement::VMod(vmod) => handle_vmod_def(&mut ki.keymap.ctx, &mut info.mods, vmod),
            Statement::Unknown => ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0,
            _ => false,
        };
        info.error_count += i32::from(!ok);
        if info.error_count > 10 {
            break;
        }
    }
}
fn copy_key_types_to_keymap(ki: &mut XkbKeymapInfo<'_>, info: &mut KeyTypesInfo) {
    ki.keymap.types = if info.types.is_empty() {
        vec![XkbKeyType {
            name: ki.keymap.ctx.atom_intern(b"ONE_LEVEL"),
            num_levels: 1,
            ..Default::default()
        }]
    } else {
        info.types.drain(..).map(|def| def.def).collect()
    };
    ki.keymap.mods = info.mods;
}
pub(crate) fn compile_key_types(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mods = keymap_info.keymap.mods;
    let mut info = key_types_info(0, &mods);
    if let Some(file) = file {
        handle_key_types_file(keymap_info, &mut info, file);
    }
    if info.error_count == 0 {
        copy_key_types_to_keymap(keymap_info, &mut info);
        true
    } else {
        false
    }
}

// ── Virtual modifier functions (migrated from vmod.rs) ──

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
    let mut mapping: u32 = 0;
    if let Some(value_ref) = stmt.value.as_ref() {
        let value = some_or_false!(expr_resolve_mod_mask(ctx, value_ref, MOD_REAL, mods));
        mapping = value;
    }
    for vmod in 0..mods.num_mods as usize {
        if mods.mods[vmod].name == stmt.name {
            if mods.mods[vmod].type_0 != MOD_VIRT {
                return false;
            }
            let mask: u32 = 1_u32 << vmod;
            if stmt.value.is_none() {
                return true;
            } else if mods.explicit_vmods & mask == 0 {
                mods.mods[vmod].mapping = mapping;
            } else if mods.mods[vmod].mapping != mapping {
                let clobber: bool = stmt.merge != MergeMode::Augment;
                let use_0: u32 = if clobber {
                    mapping
                } else {
                    mods.mods[vmod].mapping
                };
                mods.mods[vmod].mapping = use_0;
            }
            mods.explicit_vmods |= mask;
            return true;
        }
    }
    if mods.num_mods >= XKB_MAX_MODS {
        return false;
    }
    mods.mods[mods.num_mods as usize].name = stmt.name;
    mods.mods[mods.num_mods as usize].type_0 = MOD_VIRT;
    mods.mods[mods.num_mods as usize].mapping = mapping;
    if stmt.value.is_some() {
        let mask_0: u32 = 1_u32 << mods.num_mods;
        mods.explicit_vmods |= mask_0;
    }
    mods.num_mods += 1;
    true
}
#[derive(Default)]
pub(crate) struct KeyNamesInfo {
    pub(crate) error_count: i32,
    pub(crate) include_depth: u32,
    pub(crate) keycodes: KeycodeStore,
}

#[derive(Clone, Default)]
pub(crate) struct KeycodeStore {
    pub(crate) low: Vec<u32>,
    pub(crate) high: BTreeMap<u32, u32>,
    pub(crate) names: Vec<KeycodeMatch>,
}

fn name_slot(store: &mut KeycodeStore, name: u32) -> &mut KeycodeMatch {
    if name as usize >= store.names.len() {
        store.names.resize(name as usize + 1, Default::default());
    }
    &mut store.names[name as usize]
}

fn key_name(store: &KeycodeStore, keycode: u32) -> u32 {
    if keycode <= XKB_KEYCODE_MAX_CONTIGUOUS {
        store.low.get(keycode as usize).copied().unwrap_or(0)
    } else {
        store.high.get(&keycode).copied().unwrap_or(0)
    }
}

fn set_key_name(store: &mut KeycodeStore, keycode: u32, name: u32) {
    if keycode <= XKB_KEYCODE_MAX_CONTIGUOUS {
        if keycode as usize >= store.low.len() {
            store.low.resize(keycode as usize + 1, 0);
        }
        store.low[keycode as usize] = name;
    } else {
        store.high.insert(keycode, name);
    }
    *name_slot(store, name) = KeycodeMatch {
        found: true,
        index: keycode,
        ..Default::default()
    };
}

fn remove_key_name(store: &mut KeycodeStore, name: u32) {
    let binding = store.names.get(name as usize).copied().unwrap_or_default();
    if !binding.found || binding.is_alias {
        return;
    }
    if binding.index <= XKB_KEYCODE_MAX_CONTIGUOUS {
        if store.low.get(binding.index as usize) == Some(&name) {
            store.low[binding.index as usize] = 0;
            while store.low.last() == Some(&0) {
                store.low.pop();
            }
        }
    } else if store.high.get(&binding.index) == Some(&name) {
        store.high.remove(&binding.index);
    }
    store.names[name as usize].found = false;
}

fn add_key_name(info: &mut KeyNamesInfo, keycode: u32, name: u32, merge: MergeMode) -> bool {
    let clobber = merge != MergeMode::Augment;
    let binding = info
        .keycodes
        .names
        .get(name as usize)
        .copied()
        .unwrap_or_default();
    if binding.found {
        if binding.is_alias {
            if !clobber {
                return true;
            }
            info.keycodes.names[name as usize].found = false;
        } else if binding.index != keycode {
            if !clobber {
                return true;
            }
            remove_key_name(&mut info.keycodes, name);
        }
    }

    let old_name = key_name(&info.keycodes, keycode);
    if old_name == name {
        return true;
    }
    if old_name != 0 {
        if !clobber {
            return true;
        }
        info.keycodes.names[old_name as usize].found = false;
    }
    set_key_name(&mut info.keycodes, keycode, name);
    true
}

fn merge_keycode_stores(into: &mut KeyNamesInfo, from: &mut KeyNamesInfo, merge: MergeMode) {
    if into.keycodes.low.is_empty()
        && into.keycodes.high.is_empty()
        && into.keycodes.names.is_empty()
    {
        into.keycodes = std::mem::take(&mut from.keycodes);
        return;
    }
    for (keycode, &name) in from.keycodes.low.iter().enumerate() {
        if name != 0 && !add_key_name(into, keycode as u32, name, merge) {
            into.error_count += 1;
        }
    }
    for (&keycode, &name) in &from.keycodes.high {
        if !add_key_name(into, keycode, name, merge) {
            into.error_count += 1;
        }
    }
    for (alias, binding) in from.keycodes.names.iter().copied().enumerate() {
        if binding.found && binding.is_alias {
            handle_alias_def(
                into,
                &KeyAliasDef {
                    merge,
                    alias: alias as u32,
                    real: binding.index,
                },
            );
        }
    }
}

fn handle_include_keycodes(
    info: &mut KeyNamesInfo,
    includes: &mut [IncludeStmt],
    ki: &mut XkbKeymapInfo<'_>,
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        info.error_count += 10;
        return false;
    }
    let mut included = key_names_info(0);
    for stmt in includes.iter() {
        let mut file = include_file!(ki, info, stmt, FileType::Keycodes);
        let mut next = key_names_info(info.include_depth + 1);
        handle_keycodes_file(&mut next, &mut file, ki);
        if next.error_count == 0 {
            merge_keycode_stores(&mut included, &mut next, stmt.merge);
        } else {
            included.error_count += next.error_count;
        }
    }
    if included.error_count != 0 {
        info.error_count += included.error_count;
    } else if let Some(first) = includes.first() {
        merge_keycode_stores(info, &mut included, first.merge);
    }
    info.error_count == 0
}

fn key_names_info(include_depth: u32) -> KeyNamesInfo {
    KeyNamesInfo {
        include_depth,
        ..Default::default()
    }
}

fn handle_keycode_def(info: &mut KeyNamesInfo, stmt: &KeycodeDef) -> bool {
    (0..=XKB_KEYCODE_MAX as i64).contains(&stmt.value)
        && add_key_name(info, stmt.value as u32, stmt.name, stmt.merge)
}

fn handle_alias_def(info: &mut KeyNamesInfo, def: &KeyAliasDef) {
    let old = info
        .keycodes
        .names
        .get(def.alias as usize)
        .copied()
        .unwrap_or_default();
    if old.found {
        if def.merge == MergeMode::Augment {
            return;
        }
        if !old.is_alias {
            remove_key_name(&mut info.keycodes, def.alias);
        }
    }
    *name_slot(&mut info.keycodes, def.alias) = KeycodeMatch {
        found: true,
        is_alias: def.real != 0,
        index: def.real,
        ..Default::default()
    };
}

fn handle_key_name_var(ki: &mut XkbKeymapInfo<'_>, stmt: &VarDef) -> bool {
    let lhs = some_or_false!(expr_resolve_lhs(stmt.name.as_ref().unwrap()));
    let element = ki.keymap.ctx.atom_text(lhs.element);
    let field = ki.keymap.ctx.atom_text(lhs.field);
    if !element.is_empty() {
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if !field.eq_ignore_ascii_case("minimum") && !field.eq_ignore_ascii_case("maximum") {
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if lhs.index.is_some() {
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    expr_resolve_integer(&ki.keymap.ctx, stmt.value.as_ref().unwrap())
        .is_some_and(|value| (0..=u32::MAX as i64).contains(&value))
        || ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0
}

fn handle_keycodes_file(info: &mut KeyNamesInfo, file: &mut XkbFile, ki: &mut XkbKeymapInfo<'_>) {
    for statement in &mut file.defs {
        let valid = match statement {
            Statement::Include(includes) => handle_include_keycodes(info, includes, ki),
            Statement::Keycode(keycode) => handle_keycode_def(info, keycode),
            Statement::KeyAlias(alias) => {
                handle_alias_def(info, alias);
                true
            }
            Statement::Var(variable) => handle_key_name_var(ki, variable),
            Statement::Unknown => ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0,
            _ => false,
        };
        if !valid {
            info.error_count += 1;
        }
        if info.error_count > 10 {
            break;
        }
    }
}

fn finish_keycodes(keymap: &mut XkbKeymap, store: &mut KeycodeStore) {
    if store.low.is_empty() && store.high.is_empty() {
        keymap.min_key_code = 8;
        keymap.max_key_code = 255;
        keymap.num_keys_low = 256;
        keymap.num_keys = 256;
        keymap.keys = (0..256)
            .map(|keycode| XkbKey {
                keycode,
                ..Default::default()
            })
            .collect();
        return;
    }

    let low_min = store
        .low
        .iter()
        .position(|name| *name != 0)
        .map(|i| i as u32);
    let high_min = store.high.first_key_value().map(|(&keycode, _)| keycode);
    keymap.min_key_code = low_min.into_iter().chain(high_min).min().unwrap();
    keymap.max_key_code = store
        .high
        .last_key_value()
        .map_or_else(|| store.low.len() as u32 - 1, |(&keycode, _)| keycode);
    keymap.num_keys_low = store.low.len() as u32;
    keymap.num_keys = keymap.num_keys_low + store.high.len() as u32;
    keymap.keys = (0..keymap.num_keys_low)
        .map(|keycode| XkbKey {
            keycode,
            ..Default::default()
        })
        .chain(store.high.keys().map(|&keycode| XkbKey {
            keycode,
            ..Default::default()
        }))
        .collect();

    let original = store.names.clone();
    for (keycode, &name) in store.low.iter().enumerate().filter(|(_, name)| **name != 0) {
        store.names[name as usize] = KeycodeMatch {
            found: true,
            index: keycode as u32,
            ..Default::default()
        };
    }
    for (offset, &name) in store.high.values().enumerate() {
        store.names[name as usize] = KeycodeMatch {
            found: true,
            index: keymap.num_keys_low + offset as u32,
            ..Default::default()
        };
    }
    for (name, binding) in original.into_iter().enumerate() {
        if binding.found && binding.is_alias {
            let real = store
                .names
                .get(binding.index as usize)
                .copied()
                .unwrap_or_default();
            store.names[name] = if real.found && !real.is_alias {
                KeycodeMatch {
                    found: true,
                    is_alias: true,
                    index: binding.index,
                    ..Default::default()
                }
            } else {
                KeycodeMatch::default()
            };
        }
    }
    keymap.key_names = std::mem::take(&mut store.names);
}

pub(crate) fn compile_keycodes(
    file: Option<&mut XkbFile>,
    keymap_info: &mut XkbKeymapInfo<'_>,
) -> bool {
    let mut info = key_names_info(0);
    if let Some(file) = file {
        handle_keycodes_file(&mut info, file, keymap_info);
    }
    if info.error_count != 0 {
        return false;
    }
    finish_keycodes(keymap_info.keymap, &mut info.keycodes);
    true
}
struct Lhs<'a> {
    element: u32,
    field: u32,
    index: Option<&'a ExprKind>,
}

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
        ExprKind::Ident(atom) => match ctx.atom_text(*atom) {
            value
                if ["true", "yes", "on"]
                    .iter()
                    .any(|candidate| value.eq_ignore_ascii_case(candidate)) =>
            {
                Some(true)
            }
            value
                if ["false", "no", "off"]
                    .iter()
                    .any(|candidate| value.eq_ignore_ascii_case(candidate)) =>
            {
                Some(false)
            }
            _ => None,
        },
        ExprKind::Unary {
            child,
            op: UnaryOp::Not | UnaryOp::Invert,
        } => expr_resolve_boolean(ctx, child.as_deref()?).map(|value| !value),
        _ => None,
    }
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
            let left = eval_integer(left.as_deref()?, lookup)?;
            let right = eval_integer(right.as_deref()?, lookup)?;
            match op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Subtract => left.checked_sub(right),
                BinaryOp::Multiply => left.checked_mul(right),
                BinaryOp::Divide => left.checked_div(right),
                BinaryOp::Assign => unreachable!(),
            }
        }
        ExprKind::Unary { child, op } => {
            let value = eval_integer(child.as_deref()?, lookup)?;
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

pub(crate) fn expr_resolve_group(
    keymap_info: &mut XkbKeymapInfo<'_>,
    expr: &ExprKind,
    absolute: bool,
    group: &mut u32,
    pending: &mut bool,
) -> ParseStatus {
    if let ExprKind::Ident(atom) = expr {
        if keymap_info
            .keymap
            .ctx
            .atom_text(*atom)
            .eq_ignore_ascii_case(GROUP_LAST_INDEX_NAME)
            && keymap_info.keymap.num_groups == 0
        {
            *pending = true;
            return report_mismatch(keymap_info.strict);
        }
    }
    let max = XKB_MAX_GROUPS;
    let num_groups = keymap_info.keymap.num_groups;
    let ctx = &keymap_info.keymap.ctx;
    let Some(value) = eval_integer(expr, &|atom| {
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
    }) else {
        return report_mismatch(keymap_info.strict);
    };
    if value > max as i64 || (absolute && value < 1) || (!absolute && value < -(max as i64)) {
        return report_mismatch(keymap_info.strict);
    }
    *group = value as u32;
    ParseStatus::Success
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
    match ctx.atom_text(*atom) {
        value
            if ["true", "yes", "on"]
                .iter()
                .any(|candidate| value.eq_ignore_ascii_case(candidate)) =>
        {
            Some(Some(true))
        }
        value
            if ["false", "no", "off"]
                .iter()
                .any(|candidate| value.eq_ignore_ascii_case(candidate)) =>
        {
            Some(Some(false))
        }
        value if value.eq_ignore_ascii_case("default") => Some(None),
        _ => None,
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
            let left = expr_resolve_mod_mask(ctx, left.as_deref()?, mod_type, mods)?;
            let right = expr_resolve_mod_mask(ctx, right.as_deref()?, mod_type, mods)?;
            match op {
                BinaryOp::Add => Some(left | right),
                BinaryOp::Subtract => Some(left & !right),
                _ => None,
            }
        }
        ExprKind::Unary {
            op: UnaryOp::Invert,
            child,
        } => expr_resolve_mod_mask(ctx, child.as_deref()?, mod_type, mods).map(|value| !value),
        _ => None,
    }
}

pub(crate) type ActionsInfo = [XkbAction; 3];

pub(crate) fn init_actions_info(info: &mut ActionsInfo) {
    *info = [
        XkbAction::GroupSet(Default::default()),
        XkbAction::GroupLatch(Default::default()),
        XkbAction::GroupLock(Default::default()),
    ];
}

#[inline]
fn report_mismatch(strict: u32) -> ParseStatus {
    if strict & PARSER_NO_FIELD_TYPE_MISMATCH != 0 {
        ParseStatus::Fatal
    } else {
        ParseStatus::Recoverable
    }
}

fn action_index(name: &str) -> Option<usize> {
    ["SetGroup", "LatchGroup", "LockGroup"]
        .iter()
        .position(|candidate| name.eq_ignore_ascii_case(candidate))
}

fn action_group(action: &mut XkbAction) -> &mut XkbGroupAction {
    match action {
        XkbAction::GroupSet(group) | XkbAction::GroupLatch(group) | XkbAction::GroupLock(group) => {
            group
        }
        XkbAction::None => unreachable!(),
    }
}

fn set_action_group(
    keymap_info: &mut XkbKeymapInfo<'_>,
    action: &mut XkbAction,
    value: &mut ExprKind,
) -> ParseStatus {
    let (absolute, negative, inner) = match value {
        ExprKind::Unary {
            op: UnaryOp::Negate,
            child,
        } => (false, true, child.as_deref_mut().unwrap()),
        ExprKind::Unary {
            op: UnaryOp::Plus,
            child,
        } => (false, false, child.as_deref_mut().unwrap()),
        _ => (true, false, &mut *value),
    };
    let mut group = 0;
    let mut pending = false;
    let status = expr_resolve_group(keymap_info, inner, absolute, &mut group, &mut pending);
    if status != ParseStatus::Success && !pending {
        return status;
    }
    let action = action_group(action);
    action.flags.set(ActionFlags::ABSOLUTE_SWITCH, absolute);
    if pending {
        action.flags.insert(ActionFlags::PENDING_COMPUTATION);
        action.group = if absolute {
            0
        } else if negative {
            -1
        } else {
            1
        };
    } else {
        action.flags.remove(ActionFlags::PENDING_COMPUTATION);
        action.group = if absolute {
            group.wrapping_sub(1) as i32
        } else if negative {
            -(group as i32)
        } else {
            group as i32
        };
    }
    ParseStatus::Success
}

fn set_action_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    action: &mut XkbAction,
    field: &str,
    index: Option<&ExprKind>,
    value: &mut ExprKind,
) -> ParseStatus {
    if index.is_some() {
        return report_mismatch(keymap_info.strict);
    }
    if field.eq_ignore_ascii_case("group") {
        return set_action_group(keymap_info, action, value);
    }
    let flag = if field.eq_ignore_ascii_case("clearLocks")
        && matches!(action, XkbAction::GroupSet(_) | XkbAction::GroupLatch(_))
    {
        ActionFlags::LOCK_CLEAR
    } else if field.eq_ignore_ascii_case("latchToLock")
        && matches!(action, XkbAction::GroupLatch(_))
    {
        ActionFlags::LATCH_TO_LOCK
    } else if field.eq_ignore_ascii_case("lockOnRelease")
        && matches!(action, XkbAction::GroupLock(_))
    {
        if keymap_info.keymap.format < XKB_KEYMAP_FORMAT_TEXT_V2 {
            return ParseStatus::Success;
        }
        ActionFlags::LOCK_ON_RELEASE
    } else {
        return if keymap_info.strict & PARSER_NO_ILLEGAL_ACTION_FIELDS != 0 {
            ParseStatus::Fatal
        } else {
            ParseStatus::Success
        };
    };
    let Some(enabled) = expr_resolve_boolean(&keymap_info.keymap.ctx, value) else {
        return report_mismatch(keymap_info.strict);
    };
    action_group(action).flags.set(flag, enabled);
    ParseStatus::Success
}

fn action_arg(
    keymap_info: &mut XkbKeymapInfo<'_>,
    action: &mut XkbAction,
    arg: &mut ExprKind,
) -> ParseStatus {
    let mut truth;
    let (field_expr, value) = match arg {
        ExprKind::Binary {
            op: BinaryOp::Assign,
            left,
            right,
        } => (left.as_deref().unwrap(), right.as_deref_mut().unwrap()),
        ExprKind::Unary {
            op: UnaryOp::Not | UnaryOp::Invert,
            child,
        } => {
            truth = ExprKind::Boolean(false);
            (child.as_deref().unwrap(), &mut truth)
        }
        field => {
            truth = ExprKind::Boolean(true);
            (&*field, &mut truth)
        }
    };
    let Some(lhs) = expr_resolve_lhs(field_expr) else {
        return ParseStatus::Fatal;
    };
    if lhs.element != XKB_ATOM_NONE {
        return ParseStatus::Fatal;
    }
    let field = keymap_info.keymap.ctx.atom_text(lhs.field).to_owned();
    if !(field.eq_ignore_ascii_case("group")
        || field.eq_ignore_ascii_case("clearLocks")
        || field.eq_ignore_ascii_case("latchToLock")
        || field.eq_ignore_ascii_case("lockOnRelease"))
    {
        return if keymap_info.strict & PARSER_NO_UNKNOWN_ACTION_FIELDS != 0 {
            ParseStatus::Fatal
        } else {
            ParseStatus::Recoverable
        };
    }
    set_action_field(keymap_info, action, &field, lhs.index, value)
}

pub(crate) fn handle_action_def(
    keymap_info: &mut XkbKeymapInfo<'_>,
    defaults: &mut ActionsInfo,
    def: &mut ExprKind,
    action: &mut XkbAction,
) -> ParseStatus {
    let ExprKind::Action { name, args } = def else {
        return ParseStatus::Fatal;
    };
    let name = keymap_info.keymap.ctx.atom_text(*name);
    let Some(index) = action_index(name) else {
        *action = XkbAction::None;
        return ParseStatus::Success;
    };
    *action = defaults[index];
    let mut result = ParseStatus::Success;
    for arg in args {
        match action_arg(keymap_info, action, arg) {
            ParseStatus::Fatal => return ParseStatus::Fatal,
            ParseStatus::Recoverable => result = ParseStatus::Recoverable,
            ParseStatus::Success => {}
        }
    }
    result
}

pub(crate) fn set_default_action_field(
    keymap_info: &mut XkbKeymapInfo<'_>,
    defaults: &mut ActionsInfo,
    element: &str,
    field: &str,
    index: Option<&ExprKind>,
    value: &mut Option<ExprKind>,
    merge: MergeMode,
) -> ParseStatus {
    let Some(action_index) = action_index(element) else {
        return ParseStatus::Success;
    };
    let mut action = defaults[action_index];
    let status = set_action_field(
        keymap_info,
        &mut action,
        field,
        index,
        value.as_mut().unwrap(),
    );
    if status == ParseStatus::Success && merge != MergeMode::Augment {
        defaults[action_index] = action;
    }
    status
}
