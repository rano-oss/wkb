use super::keymap::mod_mask_get_effective;
pub(crate) use super::parse_xkb::{xkb_file_from_components, xkb_parse_string};
pub(crate) use super::symbols::{compile_key_types, compile_keycodes, compile_symbols};
use crate::xkb::keymap::xkb_mod_name_to_index;
pub(crate) const INCLUDE_MAX_DEPTH: i32 = 15_i32;
fn is_merge_prefix(byte: u8) -> bool {
    matches!(byte, b'+' | b'|' | b'^')
}
fn directory_for_include(type_0: FileType) -> &'static str {
    match type_0 {
        FileType::Keycodes => "keycodes",
        FileType::Types => "types",
        FileType::Symbols => "symbols",
        FileType::Rules => "rules",
        _ => "",
    }
}
pub(crate) fn find_file_in_xkb_path(
    ctx: &mut XkbContext,
    name: &str,
    type_0: FileType,
    offset: &mut u32,
) -> Option<Vec<u8>> {
    let type_dir = directory_for_include(type_0);
    let path_count = ctx.includes.len() as u32;
    for i in *offset..path_count {
        let path = format!("{}/{}/{}", ctx.includes[i as usize], type_dir, name);
        if let Some(data) = read_file_cached(&path) {
            *offset = i;
            return Some(data);
        }
    }
    None
}
pub(crate) fn exceeds_include_max_depth(include_depth: u32) -> bool {
    include_depth >= INCLUDE_MAX_DEPTH as u32
}
pub(crate) fn process_include_file(
    ctx: &mut XkbContext,
    stmt: &IncludeStmt,
    file_type: FileType,
) -> Option<XkbFile> {
    let mut offset = 0;
    let mut candidate = None;
    loop {
        let file_data = if stmt.file.starts_with('/') {
            (offset == 0)
                .then(|| read_file_cached(&stmt.file))
                .flatten()
        } else {
            find_file_in_xkb_path(ctx, &stmt.file, file_type, &mut offset)
        };
        let Some(file_data) = file_data else { break };
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
#[inline]
fn compute_effective_mask_with(mod_set: &XkbModSet, mods: &mut XkbMods) {
    let unknown_mods: u32 = !((1_u64 << mod_set.num_mods).wrapping_sub(1_u64) as u32);
    mods.mask = mod_mask_get_effective(mod_set, mods.mods) | mods.mods & unknown_mods;
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
    let vmods = named_vmods.map(|(name, syms)| (mod_index_by_name(keymap, name), syms));
    for key in &mut keymap.keys {
        let first_sym = key
            .groups
            .first()
            .and_then(|group| group.levels.first())
            .and_then(|level| level.syms.first())
            .copied()
            .unwrap_or(XKB_KEY_NO_SYMBOL);
        key.repeats.get_or_insert_with(|| {
            first_sym != XKB_KEY_NO_SYMBOL && !is_modifier_keysym(first_sym)
        });
        if key.vmodmap.is_none() {
            let mut vmodmap = 0;
            let level_one_syms = key
                .groups
                .first()
                .and_then(|group| group.levels.first())
                .map_or(&[][..], |level| level.syms.as_slice());
            for &sym in level_one_syms {
                for &(index, candidates) in &vmods {
                    if let Some(index) = index.filter(|_| candidates.contains(&sym)) {
                        vmodmap |= 1 << index;
                    }
                }
            }
            key.vmodmap = Some(vmodmap);
        }
        for group in &mut key.groups {
            for level in &mut group.levels {
                if !group.explicit_actions && level.action.is_none() {
                    level.action = level.syms.iter().copied().find_map(wkb_group_action);
                }
            }
        }
    }
}
fn update_derived_keymap_fields(info: &mut XkbKeymap) -> bool {
    let keymap: &mut XkbKeymap = &mut *info;
    keymap.key_names = Vec::new();
    keymap.num_groups = keymap.keys[keymap.min_key_code as usize..]
        .iter()
        .fold(keymap.num_groups, |max, key| max.max(key.num_groups));
    apply_wkb_compat(info);
    update_mod_mappings(info);
    compute_type_entry_masks(info);
    update_key_action_fields(info);
    true
}
fn update_mod_mappings(info: &mut XkbKeymap) {
    let keymap = &mut *info;
    for key in keymap.keys.iter().skip(keymap.min_key_code as usize) {
        for idx in _XKB_MOD_INDEX_NUM_ENTRIES as usize..keymap.mods.num_mods as usize {
            if key.vmodmap.unwrap_or(0) & 1 << idx != 0 {
                keymap.mods.mods[idx].mapping |= key.modmap;
            }
        }
    }
    if !keymap.strict {
        for idx in _XKB_MOD_INDEX_NUM_ENTRIES as usize..keymap.mods.num_mods as usize {
            let mask: u32 = 1 << idx as u32;
            if keymap.mods.mods[idx].mapping == 0 && keymap.mods.explicit_vmods & mask == 0 {
                keymap.mods.mods[idx].mapping = mask;
                keymap.mods.explicit_vmods |= mask;
            }
        }
    }
}
fn has_unbound_vmods(mod_set: &XkbModSet, mods: &XkbMods) -> bool {
    (_XKB_MOD_INDEX_NUM_ENTRIES..mod_set.num_mods)
        .any(|k| mods.mods & 1 << k != 0 && mod_set.mods[k as usize].mapping == 0)
}
fn compute_type_entry_masks(info: &mut XkbKeymap) {
    let keymap = &mut *info;
    let mods = keymap.mods;
    for type_ in &mut keymap.types {
        compute_effective_mask_with(&mods, &mut type_.mods);
        for entry in &mut type_.entries {
            if has_unbound_vmods(&mods, &entry.mods) {
                entry.mods.mask = 0;
            } else {
                compute_effective_mask_with(&mods, &mut entry.mods);
                compute_effective_mask_with(&mods, &mut entry.preserve);
            }
        }
    }
}
fn update_key_action_fields(info: &mut XkbKeymap) {
    let last = info.num_groups.saturating_sub(1);
    for key in info.keys.iter_mut().skip(info.min_key_code as usize) {
        if let Some(out_of_range) = &mut key.out_of_range {
            if out_of_range.pending {
                out_of_range.pending = false;
                out_of_range.number = last;
            }
        }
        for group in &mut key.groups {
            for level in &mut group.levels {
                let Some(
                    XkbAction::GroupSet(action)
                    | XkbAction::GroupLatch(action)
                    | XkbAction::GroupLock(action),
                ) = &mut level.action
                else {
                    continue;
                };
                if action.flags.contains(ActionFlags::PENDING_COMPUTATION) {
                    action.group = if action.flags.contains(ActionFlags::ABSOLUTE_SWITCH) {
                        last as i32
                    } else {
                        action.group.signum() * info.num_groups as i32
                    };
                    action.flags.remove(ActionFlags::PENDING_COMPUTATION);
                }
            }
        }
    }
}
pub(crate) fn compile_keymap(file: &mut XkbFile, keymap: &mut XkbKeymap) -> bool {
    let mut parts: [Option<XkbFile>; 3] = std::array::from_fn(|_| None);
    for stmt in &mut file.defs {
        let Statement::XkbFile(sub_file) = stmt else {
            continue;
        };
        let index = sub_file.file_type as usize;
        if index < parts.len() && parts[index].is_none() {
            parts[index] = Some(std::mem::take(sub_file));
        }
    }
    compile_keycodes(parts[0].as_mut(), keymap)
        && compile_key_types(parts[1].as_mut(), keymap)
        && compile_symbols(parts[2].as_mut(), keymap)
        && update_derived_keymap_fields(keymap)
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum RuleScope {
    Single,
    First,
    Later,
    Any,
    Index(usize),
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum RuleInput {
    Model,
    Layout,
    Variant,
    Option,
}
struct RuleMapping {
    inputs: Vec<RuleInput>,
    outputs: Vec<usize>,
    targets: Vec<Option<usize>>,
    active: Vec<bool>,
    deferred: bool,
    has_option: bool,
    pending: Vec<(usize, usize, Vec<u8>)>,
}
pub(crate) struct Matcher<'a> {
    pub(crate) ctx: &'a mut XkbContext,
    model: &'a str,
    layouts: Vec<&'a str>,
    variants: Vec<&'a str>,
    options: Vec<(&'a str, Option<usize>)>,
    groups: HashMap<String, Vec<String>>,
    kccgst: [Vec<u8>; 5],
}
fn rule_tokens(line: &str) -> Vec<&str> {
    line.split_once("//")
        .map_or(line, |(line, _)| line)
        .split_whitespace()
        .collect()
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
            if inputs.contains(&input) {
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
            RuleInput::Option => self.options.iter().any(|&(value, layout)| {
                (layout.is_none() || layout == target) && self.pattern_matches(pattern, value, true)
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
                Some(prefix @ (b'_' | b'-')) => {
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
        Some(out)
    }
}
#[inline]
fn concat_kccgst(into: &mut Vec<u8>, from: &[u8]) {
    if from.first().is_some_and(|byte| is_merge_prefix(*byte)) || into.is_empty() {
        into.extend_from_slice(from);
    } else if into.first().is_some_and(|byte| is_merge_prefix(*byte)) {
        into.splice(..0, from.iter().copied());
    }
}
fn parse_rules_file(matcher: &mut Matcher<'_>, data: &[u8]) -> bool {
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
            (value, layout)
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
    if !parse_rules_file(matcher, &data)
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
                .map_or(true, |byte| is_merge_prefix(*byte))
            && (1..=XKB_MAX_GROUPS).contains(&group)
        {
            *explicit_layouts = (*explicit_layouts).max(group);
            pos += count as usize;
        }
    }
    true
}
use lasso::Key as _;
use std::collections::HashMap;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: u32 = 2;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: u32 = 1;
pub(crate) const XKB_LAYOUT_OUT_OF_RANGE_WRAP: u32 = 0;
#[derive(Clone, Debug, Default)] #[rustfmt::skip] pub(crate) struct XkbRuleNames { pub(crate) rules: String, pub(crate) model: String, pub(crate) layout: String, pub(crate) variant: String, pub(crate) options: String }
#[derive(Clone)] #[rustfmt::skip] pub(crate) struct XkbContext { pub(crate) includes: Vec<String>, pub(crate) atom_table: lasso::Rodeo }
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
pub(crate) fn read_file_cached(path: &str) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}
#[derive(Clone)] #[rustfmt::skip] pub(crate) struct XkbKeymap { pub(crate) ctx: XkbContext, pub(crate) strict: bool, pub(crate) min_key_code: u32, pub(crate) keys: Vec<XkbKey>, pub(crate) key_names: Vec<u32>, pub(crate) types: Vec<XkbKeyType>, pub(crate) mods: XkbModSet, pub(crate) num_groups: u32, pub(crate) group_names: Vec<u32> }
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
#[derive(Copy, Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbModSet { pub(crate) mods: [XkbMod; 32], pub(crate) num_mods: u32, pub(crate) explicit_vmods: u32 }
#[derive(Copy, Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbMod { pub(crate) name: u32, pub(crate) type_0: u32, pub(crate) mapping: u32 }
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
bitflags::bitflags! {
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ActionFlags: u32 {
    const LOCK_CLEAR            = 1;
    const LATCH_TO_LOCK         = 2;
    const ABSOLUTE_SWITCH       = 32;
    const LOCK_ON_RELEASE       = 1024;
    const PENDING_COMPUTATION   = 8192; } }
#[derive(Copy, Clone, Default, PartialEq, Eq)] #[rustfmt::skip] pub struct XkbGroupAction { pub flags: ActionFlags, pub group: i32 }
#[derive(Copy, Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbMods { pub(crate) mods: u32, pub(crate) mask: u32 }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbKeyType { pub(crate) name: u32, pub(crate) modifiers_set: bool, pub(crate) mods: XkbMods, pub(crate) num_levels: u32, pub(crate) entries: Vec<XkbKeyTypeEntry> }
#[derive(Copy, Clone)] #[rustfmt::skip] pub(crate) struct XkbKeyTypeEntry { pub(crate) level: u32, pub(crate) mods: XkbMods, pub(crate) preserve: XkbMods }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbKey { pub(crate) modmap: u32, pub(crate) vmodmap: Option<u32>, pub(crate) repeats: Option<bool>, pub(crate) out_of_range: Option<OutOfRangeInfo>, pub(crate) num_groups: u32, pub(crate) groups: Vec<XkbGroup> }
#[derive(Clone, Copy, Default)] #[rustfmt::skip] pub(crate) struct OutOfRangeInfo { pub(crate) policy: u32, pub(crate) number: u32, pub(crate) pending: bool }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbGroup { pub(crate) explicit_actions: bool, pub(crate) type_idx: u32, pub(crate) levels: Vec<XkbLevel> }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbLevel { pub(crate) syms: Vec<u32>, pub(crate) action: Option<XkbAction> }
pub(crate) const XKB_MAX_GROUPS: u32 = 32;
pub(crate) const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;
pub(crate) const DFLT_XKB_CONFIG_EXTRA_PATH: &str = "/usr/local/etc/xkb";
pub(crate) const DFLT_XKB_CONFIG_ROOT: &str = "/usr/share/xkeyboard-config-2";
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";
pub(crate) const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub(crate) const XKB_KEYSYM_CASE_INSENSITIVE: u32 = 1;
pub(crate) const XKB_KEYSYM_MAX: u32 = 0x1fffffff;
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbComponentNames { pub(crate) keycodes: Vec<u8>, pub(crate) symbols: Vec<u8>, pub(crate) types: Vec<u8> }
pub(crate) const XKB_ATOM_NONE: u32 = 0;
pub const XKB_MOD_NONE: u32 = 0xffffffff;
pub(crate) const _XKB_MOD_INDEX_NUM_ENTRIES: u32 = 8;
pub(crate) const XKB_KEYCODE_MAX_CONTIGUOUS: u32 = 0xfff;
pub(crate) const XKB_LEVEL_MAX_IMPL: u32 = 2048;
pub(crate) const XKB_MAX_MODS: u32 = 32;
impl XkbKeymap {
    #[inline]
    pub(crate) fn get_key(&self, kc: u32) -> Option<&XkbKey> {
        if kc < self.min_key_code {
            None
        } else {
            self.keys.get(kc as usize)
        }
    }
    pub(crate) fn key_index_by_name(&self, name: u32, aliases: bool) -> Option<usize> {
        const ALIAS: u32 = 1 << 31;
        let binding = *self.key_names.get(name as usize)?;
        let binding = if binding & ALIAS != 0 && aliases {
            *self.key_names.get((binding & !ALIAS) as usize)?
        } else {
            binding
        };
        (binding != 0 && binding & ALIAS == 0).then_some(binding as usize - 1)
    }
    #[inline]
    pub(crate) fn key_by_name_mut(&mut self, name: u32, aliases: bool) -> Option<&mut XkbKey> {
        let idx = self.key_index_by_name(name, aliases)?;
        self.keys.get_mut(idx)
    }
}
pub(crate) const XKB_KEY_NO_SYMBOL: u32 = 0;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum FileType {
    Keycodes = 0,
    Types = 1,
    Symbols = 2,
    #[default]
    Keymap,
    Rules,
    Ignored,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(crate) enum MergeMode {
    #[default]
    Default = 0,
    Augment = 1,
    Override = 2,
    Replace = 3,
}
#[derive(Clone)] #[rustfmt::skip] pub(crate) struct IncludeStmt { pub(crate) merge: MergeMode, pub(crate) file: String, pub(crate) map: String, pub(crate) modifier: String }
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
        left: Box<ExprKind>,
        right: Box<ExprKind>,
    },
    Unary {
        op: UnaryOp,
        child: Box<ExprKind>,
    },
}
#[rustfmt::skip] pub(crate) struct VarDef { pub(crate) merge: MergeMode, pub(crate) name: Option<ExprKind>, pub(crate) value: Option<ExprKind> }
#[rustfmt::skip] pub(crate) struct VModDef { pub(crate) merge: MergeMode, pub(crate) name: u32, pub(crate) value: Option<ExprKind> }
#[derive(Copy, Clone)] #[rustfmt::skip] pub(crate) struct KeycodeDef { pub(crate) merge: MergeMode, pub(crate) name: u32, pub(crate) value: i64 }
#[derive(Copy, Clone)] #[rustfmt::skip] pub(crate) struct KeyAliasDef { pub(crate) merge: MergeMode, pub(crate) alias: u32, pub(crate) real: u32 }
#[rustfmt::skip] pub(crate) struct NamedVarDef { pub(crate) merge: MergeMode, pub(crate) name: u32, pub(crate) body: Vec<VarDef> }
#[rustfmt::skip] pub(crate) struct ModMapDef { pub(crate) merge: MergeMode, pub(crate) modifier: u32, pub(crate) keys: Vec<ExprKind> }
pub(crate) const MAP_HAS_MAP_FLAGS: u32 = 2;
pub(crate) const MAP_IS_DEFAULT: u32 = 1;
pub(crate) enum Statement {
    Include(Vec<IncludeStmt>),
    Keycode(KeycodeDef),
    KeyAlias(KeyAliasDef),
    Var(VarDef),
    KeyType(NamedVarDef),
    VMods(Vec<VModDef>),
    Symbols(NamedVarDef),
    ModMap(ModMapDef),
    Unknown,
    XkbFile(XkbFile),
}
#[derive(Default)]
pub(crate) struct XkbFile {
    pub(crate) name: String,
    pub(crate) defs: Vec<Statement>,
    pub(crate) file_type: FileType,
    pub(crate) flags: u32,
}
fn parse_u32(s: &[u8], radix: u32) -> (u32, i32) {
    let valid = |byte: &u8| {
        if radix == 10 {
            byte.is_ascii_digit()
        } else {
            byte.is_ascii_hexdigit()
        }
    };
    let count = s.iter().take_while(|byte| valid(byte)).count();
    let value = std::str::from_utf8(&s[..count])
        .ok()
        .and_then(|digits| u32::from_str_radix(digits, radix).ok());
    value.map_or((0, -1), |value| (value, count as i32))
}
pub(crate) fn parse_dec_u32(s: &[u8]) -> (u32, i32) {
    parse_u32(s, 10)
}
pub(crate) fn parse_hex_u32(s: &[u8]) -> (u32, i32) {
    parse_u32(s, 16)
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
