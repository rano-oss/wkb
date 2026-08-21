pub(crate) use super::keymap::xkb_mod_name_to_index;
use super::keysym::xkb_keysym_is_keypad;
use super::keysym::{xkb_keysym_is_lower, xkb_keysym_is_upper_or_title};
use super::parse_xkb::Stream;
use super::parser::{exceeds_include_max_depth, process_include_stream, XKB_MULTI_SYMBOL_LEVEL};
pub(crate) use super::parser::{ModMapDef, NamedVarDef};
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
pub(crate) struct SymbolsBuilder {
    pub(crate) include_depth: u32,
    pub(crate) explicit_group: Option<u32>,
    pub(crate) keys: Vec<Option<KeyInfo>>,
    pub(crate) default_key: KeyInfo,
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
type KeyInfo = XkbKey;
type GroupInfo = XkbGroup;
pub(crate) enum CompileInput<'a, 'src> {
    Stream(Option<&'a mut Stream<'src>>),
    Includes(&'a mut [IncludeStmt]),
}
fn compile_stream<T>(
    keymap: &mut XkbKeymap,
    state: &mut T,
    stream: &mut Stream<'_>,
    mut compile: impl FnMut(&mut XkbKeymap, &mut T, &mut Statement<'_>) -> bool,
) -> bool {
    while let Ok(Some(mut statement)) = stream.next_statement(&mut keymap.ctx) {
        if !compile(keymap, state, &mut statement) {
            return false;
        }
    }
    matches!(stream.next_statement(&mut keymap.ctx), Ok(None))
}
impl GroupInfo {
    fn has_any_field(&self) -> bool {
        self.explicit_syms || self.type_idx != 0
    }
}
impl SymbolsBuilder {
    fn new(ki: &mut XkbKeymap, include_depth: u32, mods: &XkbModSet) -> Self {
        let star_atom = ki.ctx.atom_intern(b"*");
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
    fn add_key(&mut self, ki: &mut XkbKeymap, key: &mut KeyInfo, merge: MergeMode) {
        let Some(slot) = ki.key_index_by_name(key.name, true) else {
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
                finalize_key(keymap, key);
                keymap.keys[key_idx] = std::mem::take(key);
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
    fn merge_from(&mut self, ki: &mut XkbKeymap, from: &mut Self, merge: MergeMode) {
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
    fn include(&mut self, ki: &mut XkbKeymap, includes: &mut [IncludeStmt]) -> bool {
        if exceeds_include_max_depth(self.include_depth) {
            return false;
        }
        let mut included = Self::new(ki, self.include_depth + 1, &self.mods);
        let include_statements = &mut *includes;
        for statement in include_statements {
            let Some(file) = process_include_stream(&mut ki.ctx, statement, FileType::Symbols)
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
            } else if ki.num_groups != 0 && next.include_depth == 1 {
                Some(0)
            } else {
                self.explicit_group
            };
            if !next.compile_stream(ki, &mut file.stream()) {
                return false;
            }
            included.merge_from(ki, &mut next, statement.merge);
        }
        if let Some(first) = includes.first() {
            self.merge_from(ki, &mut included, first.merge);
        }
        true
    }
    fn compile_stream(&mut self, ki: &mut XkbKeymap, stream: &mut Stream<'_>) -> bool {
        compile_stream(ki, self, stream, |ki, this, statement| {
            this.compile_statement(ki, statement)
        })
    }
    fn compile_statement(&mut self, ki: &mut XkbKeymap, statement: &mut Statement<'_>) -> bool {
        match statement {
            Statement::Include(includes) => self.include(ki, includes),
            Statement::Symbols(definition) => self.compile_key(ki, definition),
            Statement::Var(variable) => self.compile_global(ki, variable),
            Statement::VMods(vmods) => vmods
                .iter()
                .all(|vmod| handle_vmod_def(&mut ki.ctx, &mut self.mods, vmod)),
            Statement::ModMap(definition) => self.compile_modmap(ki, definition),
            Statement::Unknown => true,
            _ => false,
        }
    }
    fn compile_key(&mut self, ki: &mut XkbKeymap, stmt: &mut NamedVarDef<'_>) -> bool {
        let dk = &self.default_key;
        let mut keyi = dk.clone();
        keyi.name = stmt.name;
        if self.compile_key_body(ki, &mut Stream::new(stmt.body), &mut keyi) {
            set_explicit_group(self, &mut keyi);
            self.add_key(ki, &mut keyi, stmt.merge);
            return true;
        }
        false
    }
    fn compile_modmap(&mut self, ki: &mut XkbKeymap, def: &mut ModMapDef) -> bool {
        let modifier_name: &str = ki.ctx.atom_text(def.modifier);
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
    fn compile_global(&mut self, ki: &mut XkbKeymap, stmt: &mut VarDef) -> bool {
        let lhs = stmt.name.as_ref().unwrap();
        if lhs.element == Element::Key {
            if matches!(
                lhs.field,
                Field::Name
                    | Field::Modifiers
                    | Field::LevelName
                    | Field::Map
                    | Field::Preserve
                    | Field::Other
            ) {
                return true;
            }
            let mut temp = KeyInfo {
                name: self.star_atom,
                ..Default::default()
            };
            let valid = set_symbols_field(
                ki,
                self,
                &mut temp,
                lhs.field,
                lhs.index.as_ref(),
                &mut stmt.value,
            );
            let mut dk = std::mem::take(&mut self.default_key);
            merge_keys(self.star_atom, &mut dk, &mut temp, stmt.merge);
            self.default_key = dk;
            valid
        } else if lhs.element == Element::None && lhs.field == Field::Name {
            set_group_name(
                ki,
                self,
                lhs.index.as_ref(),
                stmt.value.as_ref().unwrap(),
                stmt.merge,
            )
        } else {
            true
        }
    }
    fn compile_key_body(
        &mut self,
        ki: &mut XkbKeymap,
        body: &mut Stream<'_>,
        keyi: &mut KeyInfo,
    ) -> bool {
        let mut all_valid_entries: bool = true;
        loop {
            let mut def = match body.next_var(&mut ki.ctx) {
                Ok(Some(def)) => def,
                Ok(None) => return all_valid_entries,
                Err(()) => return false,
            };
            let (field, index) = if let Some(lhs) = &def.name {
                if lhs.element != Element::None {
                    all_valid_entries = false;
                    continue;
                }
                (Some(lhs.field), lhs.index.as_ref())
            } else if matches!(&def.value, Some(ExprKind::Actions)) {
                (Some(Field::Actions), None)
            } else {
                (Some(Field::Symbols), None)
            };
            let valid = match (field, def.value.is_some()) {
                (Some(field), true) => {
                    set_symbols_field(ki, self, keyi, field, index, &mut def.value)
                }
                (None, true) => true,
                _ => false,
            };
            if !valid {
                all_valid_entries = false;
            }
        }
    }
}
fn init_key_info_with_atom(keyi: &mut KeyInfo, star_atom: u32) {
    *keyi = KeyInfo {
        name: star_atom,
        ..Default::default()
    };
}
fn merge_groups(into: &mut GroupInfo, from: &mut GroupInfo, clobber: bool) {
    if from.type_idx != 0 && (into.type_idx == 0 || clobber) {
        into.type_idx = from.type_idx;
    }
    let source = std::mem::take(&mut from.levels);
    into.levels
        .resize_with(into.levels.len().max(source.len()), Default::default);
    let mut changed = 0;
    for (dst, mut src) in into.levels.iter_mut().zip(source) {
        if src.sym != 0 && (dst.sym == 0 || clobber) {
            dst.sym = src.sym;
            changed += 1;
        }
        if src.action.is_some() && (dst.action.is_none() || clobber) {
            dst.action = src.action.take();
        }
    }
    if changed == into.levels.len() {
        into.explicit_syms = from.explicit_syms;
    } else if from.explicit_syms {
        into.explicit_syms = true;
    }
}
fn merge_keys(star_atom: u32, into: &mut KeyInfo, from: &mut KeyInfo, merge: MergeMode) {
    let clobber = merge != MergeMode::Augment;
    if merge == MergeMode::Replace {
        std::mem::swap(into, from);
        init_key_info_with_atom(from, star_atom);
        return;
    }
    let source = std::mem::take(&mut from.groups);
    into.groups
        .resize_with(into.groups.len().max(source.len()), Default::default);
    for (dst, mut src) in into.groups.iter_mut().zip(source) {
        merge_groups(dst, &mut src, clobber);
    }
    if from.vmodmap.is_some() && (into.vmodmap.is_none() || clobber) {
        into.vmodmap = from.vmodmap;
    }
    if from.repeat.is_some() && (into.repeat.is_none() || clobber) {
        into.repeat = from.repeat;
    }
    if from.default_type != 0 && (into.default_type == 0 || clobber) {
        into.default_type = from.default_type;
    }
    init_key_info_with_atom(from, star_atom);
}
fn group_index(ki: &mut XkbKeymap, key: &mut KeyInfo, index: Option<&ExprKind>) -> Option<usize> {
    let index = match index {
        Some(expr) => {
            let (group, _) = expr_resolve_group(ki, expr, false)?;
            group.checked_sub(1)? as usize
        }
        None => key
            .groups
            .iter()
            .position(|group| !group.explicit_syms)
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
    ki: &mut XkbKeymap,
    key: &mut KeyInfo,
    array_index: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    let Some(group_index) = group_index(ki, key, array_index) else {
        return false;
    };
    let group = &mut key.groups[group_index];
    let ExprKind::Symbols(syms) = value else {
        return false;
    };
    if group.explicit_syms {
        return false;
    }
    let level_count = syms
        .iter()
        .rposition(|&sym| sym != 0)
        .map_or(0, |index| index + 1)
        .max(
            syms.iter()
                .rposition(|&sym| sym == XKB_MULTI_SYMBOL_LEVEL)
                .map_or(0, |index| index + 1),
        );
    group.levels.resize_with(level_count, XkbLevel::default);
    group.explicit_syms = true;
    for (level, &sym) in syms.iter().take(level_count).enumerate() {
        if sym == XKB_MULTI_SYMBOL_LEVEL {
            let key_name = {
                let name = ki.ctx.atom_text(key.name);
                if name.is_empty() {
                    format!("keycode {}", key.name)
                } else {
                    name.to_owned()
                }
            };
            ki.multi_symbol_error = Some((key_name, level + 1));
            return false;
        }
        group.levels[level].sym = sym;
    }
    true
}
fn set_symbols_field(
    ki: &mut XkbKeymap,
    info: &mut SymbolsBuilder,
    keyi: &mut KeyInfo,
    mapped_field: Field,
    array_ndx: Option<&ExprKind>,
    value_opt: &mut Option<ExprKind>,
) -> bool {
    match mapped_field {
        Field::Type => {
            let val = some_or_false!(expr_resolve_string(value_opt.as_ref().unwrap()));
            if let Some(array_ndx) = array_ndx {
                let ndx = some_or_false!(expr_resolve_group(ki, array_ndx, false)).0 - 1;
                if ndx >= keyi.groups.len() as u32 {
                    keyi.groups
                        .resize_with((ndx as usize) + 1, Default::default);
                }
                keyi.groups[ndx as usize].type_idx = val;
            } else {
                keyi.default_type = val;
            }
        }
        Field::Symbols => {
            return add_symbols_to_key(ki, keyi, array_ndx, value_opt.as_ref().unwrap());
        }
        Field::Actions => {
            return true;
        }
        Field::Vmods => {
            let val = value_opt.as_ref().unwrap();
            let mask = some_or_false!(expr_resolve_mod_mask(&ki.ctx, val, MOD_VIRT, &info.mods));
            keyi.vmodmap = Some(mask);
        }
        Field::Ignored => {}
        Field::Repeat => {
            keyi.repeat = some_or_false!(expr_resolve_repeat(&ki.ctx, value_opt.as_ref().unwrap()));
        }
        _ => return true,
    }
    true
}
fn set_group_name(
    ki: &mut XkbKeymap,
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
            .map(|level| level.sym)
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
fn find_type_for_group(keymap: &mut XkbKeymap, default_type: u32, group: &GroupInfo) -> usize {
    let name = match if group.type_idx != 0 {
        group.type_idx
    } else {
        default_type
    } {
        0 => find_automatic_type(&mut keymap.ctx, group),
        name => name,
    };
    keymap
        .types
        .iter()
        .position(|key_type| key_type.name == name)
        .unwrap_or(0)
}
fn finalize_key(keymap: &mut XkbKeymap, key: &mut KeyInfo) {
    let num_groups = key
        .groups
        .iter()
        .rposition(|group| !group.levels.is_empty() || key.default_type != 0 || group.type_idx != 0)
        .map_or(0, |index| index + 1);
    if num_groups == 0 {
        return;
    }
    key.groups.resize_with(num_groups, Default::default);
    for i in 1..key.groups.len() {
        if !key.groups[i].has_any_field() {
            key.groups[i] = key.groups[0].clone();
        }
    }
    let default_type = key.default_type;
    for group in &mut key.groups {
        group.type_idx = find_type_for_group(keymap, default_type, group) as u32;
        group.levels.resize_with(
            keymap.types[group.type_idx as usize].num_levels as usize,
            Default::default,
        );
    }
}
fn find_key_by_symbol(keymap: &XkbKeymap, start: usize, sym: u32) -> Option<usize> {
    (start..keymap.keys.len()).find(|&index| {
        let key = &keymap.keys[index];
        key.groups
            .iter()
            .flat_map(|group| &group.levels)
            .any(|level| level.sym == sym)
    })
}
pub(crate) fn compile_symbols(input: CompileInput<'_, '_>, keymap: &mut XkbKeymap) -> bool {
    let mods = keymap.mods;
    let mut builder = SymbolsBuilder::new(keymap, 0, &mods);
    let valid = match input {
        CompileInput::Stream(stream) => {
            stream.is_none_or(|stream| builder.compile_stream(keymap, stream))
        }
        CompileInput::Includes(includes) => builder.include(keymap, includes),
    };
    if !valid {
        return false;
    }
    builder.finish(keymap);
    true
}
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
    ki: &mut XkbKeymap,
    info: &mut KeyTypesInfo,
    includes: &mut [IncludeStmt],
) -> bool {
    if exceeds_include_max_depth(info.include_depth) {
        return false;
    }
    let mut included = key_types_info(info.include_depth.wrapping_add(1), &info.mods);
    for stmt in includes.iter() {
        let Some(file) = process_include_stream(&mut ki.ctx, stmt, FileType::Types) else {
            return false;
        };
        let mut next = key_types_info(info.include_depth.wrapping_add(1), &included.mods);
        if !compile_stream(ki, &mut next, &mut file.stream(), handle_key_type_statement) {
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
    ki: &XkbKeymap,
    info: &mut KeyTypesInfo,
    type_0: &mut XkbKeyType,
    field: Field,
    array_ndx: Option<&ExprKind>,
    value: &ExprKind,
) -> bool {
    if field == Field::Modifiers {
        if array_ndx.is_some() || type_0.modifiers_set {
            return false;
        }
        type_0.mods.mods =
            some_or_false!(expr_resolve_mod_mask(&ki.ctx, value, MOD_BOTH, &info.mods));
        type_0.modifiers_set = true;
        return true;
    }
    if field == Field::LevelName {
        return true;
    }
    if !matches!(field, Field::Map | Field::Preserve) {
        return !ki.strict;
    }
    let Some(array_ndx) = array_ndx else {
        return false;
    };
    let mods = some_or_false!(expr_resolve_mod_mask(
        &ki.ctx, array_ndx, MOD_BOTH, &info.mods
    )) & type_0.mods.mods;
    if field == Field::Map {
        add_map_entry(
            type_0,
            &XkbKeyTypeEntry {
                level: some_or_false!(expr_resolve_level(&ki.ctx, value)),
                mods: XkbMods { mods, mask: 0 },
                preserve: XkbMods::default(),
            },
        );
    } else {
        let preserve =
            some_or_false!(expr_resolve_mod_mask(&ki.ctx, value, MOD_BOTH, &info.mods)) & mods;
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
    ki: &mut XkbKeymap,
    info: &mut KeyTypesInfo,
    body: &[u8],
    type_0: &mut XkbKeyType,
) -> bool {
    let mut vars = Stream::new(body);
    loop {
        let def = match vars.next_var(&mut ki.ctx) {
            Ok(Some(def)) => def,
            Ok(None) => return true,
            Err(()) => return false,
        };
        let Some(lhs) = def.name.as_ref() else {
            return false;
        };
        if !(lhs.element == Element::Type
            || lhs.element == Element::None
                && def.value.as_ref().is_some_and(|value| {
                    set_key_type_field(ki, info, type_0, lhs.field, lhs.index.as_ref(), value)
                }))
        {
            return false;
        }
    }
}
fn handle_type_global_var(ki: &XkbKeymap, stmt: &VarDef) -> bool {
    let lhs = some_or_false!(stmt.name.as_ref());
    lhs.element == Element::Type || !ki.strict
}
fn handle_key_type_statement(
    ki: &mut XkbKeymap,
    info: &mut KeyTypesInfo,
    stmt: &mut Statement<'_>,
) -> bool {
    match stmt {
        Statement::Include(incl) => handle_include_key_types(ki, info, incl),
        Statement::KeyType(def) => {
            let mut type_0 = XkbKeyType {
                name: def.name,
                num_levels: 1,
                ..Default::default()
            };
            if !handle_key_type_body(ki, info, def.body, &mut type_0) {
                false
            } else {
                add_key_type(info, type_0, def.merge);
                true
            }
        }
        Statement::Var(var) => handle_type_global_var(ki, var),
        Statement::VMods(vmods) => vmods
            .iter()
            .all(|vmod| handle_vmod_def(&mut ki.ctx, &mut info.mods, vmod)),
        Statement::Unknown => !ki.strict,
        _ => false,
    }
}
pub(crate) fn compile_key_types(input: CompileInput<'_, '_>, keymap: &mut XkbKeymap) -> bool {
    let mut info = key_types_info(0, &keymap.mods);
    let valid = match input {
        CompileInput::Stream(stream) => stream.is_none_or(|stream| {
            compile_stream(keymap, &mut info, stream, handle_key_type_statement)
        }),
        CompileInput::Includes(includes) => handle_include_key_types(keymap, &mut info, includes),
    };
    if !valid {
        return false;
    }
    if info.types.is_empty() {
        info.types.push(XkbKeyType {
            name: keymap.ctx.atom_intern(b"ONE_LEVEL"),
            num_levels: 1,
            ..Default::default()
        });
    }
    keymap.types = info.types;
    keymap.mods = info.mods;
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
struct KeyNamesInfo {
    depth: u32,
    codes: Vec<u32>,
    names: Vec<u32>,
    aliases: Vec<(u32, u32)>,
}
const KEY_ALIAS: u32 = 1 << 31;
fn add_key_name(info: &mut KeyNamesInfo, code: u32, name: u32, clobber: bool) -> bool {
    if code > XKB_KEYCODE_MAX_CONTIGUOUS {
        return false;
    }
    if !clobber
        && info
            .names
            .get(name as usize)
            .is_some_and(|value| *value != 0)
    {
        return true;
    }
    info.codes
        .resize(info.codes.len().max(code as usize + 1), 0);
    info.names
        .resize(info.names.len().max(name as usize + 1), 0);
    if let Some(old) = info
        .codes
        .get(code as usize)
        .copied()
        .filter(|old| *old != 0)
    {
        info.names[old as usize] = 0;
    }
    info.codes[code as usize] = name;
    info.names[name as usize] = code + 1;
    true
}
fn compile_keycode_statement(
    ki: &mut XkbKeymap,
    info: &mut KeyNamesInfo,
    statement: &mut Statement<'_>,
) -> bool {
    match statement {
        Statement::Include(includes) => {
            if exceeds_include_max_depth(info.depth) {
                return false;
            }
            info.depth += 1;
            for include in includes {
                let Some(file) = process_include_stream(&mut ki.ctx, include, FileType::Keycodes)
                else {
                    return false;
                };
                if !compile_stream(ki, info, &mut file.stream(), compile_keycode_statement) {
                    return false;
                }
            }
            info.depth -= 1;
            true
        }
        Statement::Keycode(def) => u32::try_from(def.value).ok().is_some_and(|code| {
            add_key_name(info, code, def.name, def.merge != MergeMode::Augment)
        }),
        Statement::KeyAlias(def) => {
            info.aliases.push((def.alias, def.real));
            true
        }
        Statement::Var(_) | Statement::Unknown => true,
        _ => false,
    }
}
pub(crate) fn compile_keycodes(input: CompileInput<'_, '_>, keymap: &mut XkbKeymap) -> bool {
    let mut info = KeyNamesInfo::default();
    let valid = match input {
        CompileInput::Stream(stream) => stream.is_none_or(|stream| {
            compile_stream(keymap, &mut info, stream, compile_keycode_statement)
        }),
        CompileInput::Includes(includes) => compile_keycode_statement(
            keymap,
            &mut info,
            &mut Statement::Include(includes.to_vec()),
        ),
    };
    if !valid {
        return false;
    }
    info.codes.resize(info.codes.len().max(256), 0);
    keymap.min_key_code = info.codes.iter().position(|name| *name != 0).unwrap_or(8) as u32;
    keymap.keys.resize_with(info.codes.len(), XkbKey::default);
    for (alias, real) in info.aliases {
        info.names
            .resize(info.names.len().max(alias as usize + 1), 0);
        if info
            .names
            .get(real as usize)
            .is_some_and(|value| *value != 0)
        {
            info.names[alias as usize] = KEY_ALIAS | real;
        }
    }
    keymap.key_names = info.names;
    true
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
    let ExprKind::Scalar(expr) = expr else {
        return None;
    };
    let mut value = 0i64;
    for &(add, term) in &expr.terms {
        let term = match term {
            Scalar::Integer(value) => value,
            Scalar::Ident(atom) => lookup(atom)?,
        };
        value = if add {
            value.checked_add(term)?
        } else {
            value.checked_sub(term)?
        };
    }
    Some(if expr.invert { !value } else { value })
}
fn named_number(name: &str, prefix: &str, max: u32) -> Option<i64> {
    let suffix = name.get(prefix.len()..)?;
    name.get(..prefix.len())?
        .eq_ignore_ascii_case(prefix)
        .then_some(())?;
    let value = suffix.parse::<u32>().ok()?;
    (1..=max).contains(&value).then_some(value as i64)
}
pub(crate) fn expr_resolve_level(ctx: &XkbContext, expr: &ExprKind) -> Option<u32> {
    let value = eval_integer(expr, &|atom| {
        named_number(ctx.atom_text(atom), "Level", XKB_LEVEL_MAX_IMPL)
    })?;
    (1..=XKB_LEVEL_MAX_IMPL as i64)
        .contains(&value)
        .then_some(value as u32 - 1)
}
fn expr_resolve_group(
    keymap_info: &XkbKeymap,
    expr: &ExprKind,
    absolute: bool,
) -> Option<(u32, bool)> {
    if let ExprKind::Scalar(ScalarExpr {
        terms,
        invert: false,
    }) = expr
    {
        let Some((true, Scalar::Ident(atom))) = terms.first().copied().filter(|_| terms.len() == 1)
        else {
            return eval_group(keymap_info, expr, absolute);
        };
        if keymap_info.ctx.atom_text(atom).eq_ignore_ascii_case("last")
            && keymap_info.num_groups == 0
        {
            return Some((0, true));
        }
    }
    eval_group(keymap_info, expr, absolute)
}
fn eval_group(keymap_info: &XkbKeymap, expr: &ExprKind, absolute: bool) -> Option<(u32, bool)> {
    let max = XKB_MAX_GROUPS;
    let num_groups = keymap_info.num_groups;
    let ctx = &keymap_info.ctx;
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
pub(crate) fn expr_resolve_string(expr: &ExprKind) -> Option<u32> {
    if let ExprKind::String(value) = expr {
        Some(*value)
    } else {
        None
    }
}
fn expr_resolve_repeat(ctx: &XkbContext, expr: &ExprKind) -> Option<Option<bool>> {
    let ExprKind::Scalar(ScalarExpr {
        terms,
        invert: false,
    }) = expr
    else {
        return None;
    };
    let (true, Scalar::Ident(atom)) = terms.first().copied().filter(|_| terms.len() == 1)? else {
        return None;
    };
    let value = ctx.atom_text(atom);
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
    let ExprKind::Scalar(expr) = expr else {
        return None;
    };
    let mut mask = 0;
    for &(add, term) in &expr.terms {
        let value = match term {
            Scalar::Integer(value) => u32::try_from(value).ok()?,
            Scalar::Ident(atom) => match ctx.atom_text(atom) {
                value if value.eq_ignore_ascii_case("all") => MOD_REAL_MASK_ALL,
                value if value.eq_ignore_ascii_case("none") => 0,
                _ => 1 << xkb_mod_name_to_index(mods, atom, mod_type)?,
            },
        };
        if add {
            mask |= value
        } else {
            mask &= !value
        }
    }
    Some(if expr.invert { !mask } else { mask })
}
