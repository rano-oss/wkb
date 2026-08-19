use super::keymap::mod_mask_get_effective;
pub(crate) use super::parse_xkb::{xkb_file_from_components, xkb_parse_string};
pub(crate) use super::symbols::{compile_key_types, compile_keycodes, compile_symbols};
use crate::xkb::keymap::xkb_mod_name_to_index;

// ── Include file processing (merged from include.rs) ──

use super::keymap::getenv_or;

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
fn include_path<'a>(
    name: &'a str,
    file_type: FileType,
) -> Option<(std::borrow::Cow<'a, str>, bool)> {
    let Some(k) = name.find('%') else {
        return Some((std::borrow::Cow::Borrowed(name), false));
    };
    let suffix = expand_percent(directory_for_include(file_type), &name[k..])?;
    Some((
        std::borrow::Cow::Owned(format!("{}{}", &name[..k], suffix)),
        true,
    ))
}
pub(crate) fn find_file_in_xkb_path(
    ctx: &mut XkbContext,
    name: &str,
    type_0: FileType,
    offset: &mut u32,
) -> Option<std::sync::Arc<Vec<u8>>> {
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
    apply_wkb_compat(info.keymap);
    update_mod_mappings(info);
    compute_type_entry_masks(info);
    update_key_action_fields(info);
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

fn update_key_action_fields(info: &mut XkbKeymapInfo<'_>) {
    let last = info.keymap.num_groups.saturating_sub(1);
    let start_idx = if info.keymap.num_keys_low == 0 {
        0_u32
    } else {
        info.keymap.min_key_code
    };
    for ki in start_idx..info.keymap.num_keys {
        let key = &mut info.keymap.keys[ki as usize];
        if key.out_of_range_pending_group {
            key.out_of_range_pending_group = false;
            key.out_of_range_group_number = last;
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
                        action.group.signum() * info.keymap.num_groups as i32
                    };
                    action.flags.remove(ActionFlags::PENDING_COMPUTATION);
                }
            }
        }
    }
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
    let mut info = XkbKeymapInfo {
        keymap,
        strict: km_format == XKB_KEYMAP_FORMAT_TEXT_V1,
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
    pub(crate) atom_table: lasso::Rodeo,
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

pub(crate) fn read_file_cached(path: &str) -> Option<Arc<Vec<u8>>> {
    std::fs::read(path).ok().map(Arc::new)
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
    pub(crate) is_alias: bool,
    pub(crate) index: u32,
}

#[derive(Clone, Default)]
pub(crate) struct XkbKey {
    pub(crate) keycode: u32,
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
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";

pub(crate) const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub(crate) const XKB_KEYSYM_CASE_INSENSITIVE: u32 = 1;

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
    pub(crate) fn key_by_name_mut(&mut self, name: u32, aliases: bool) -> Option<&mut XkbKey> {
        let idx = self.key_index_by_name(name, aliases)?;
        self.keys.get_mut(idx)
    }
}

// Error codes (from xkbcommon_errors_h)
pub(crate) const XKB_KEY_NO_SYMBOL: u32 = 0;

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
        left: Box<ExprKind>,
        right: Box<ExprKind>,
    },
    Unary {
        op: UnaryOp,
        child: Box<ExprKind>,
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

pub(crate) struct XkbKeymapInfo<'a> {
    pub(crate) keymap: &'a mut XkbKeymap,
    pub(crate) strict: bool,
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
pub(crate) fn parse_hex_u32(s: &[u8]) -> (u32, i32) {
    let (value, count) = parse_uint(s, 16, u32::MAX as u64);
    (value as u32, count)
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
