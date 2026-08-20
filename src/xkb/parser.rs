use super::keymap::mod_mask_get_effective;
pub(crate) use super::parse_xkb::{
    include_create, xkb_select_map, xkb_select_owned, OwnedMap, SelectedMap, Stream,
};
use super::symbols::{compile_key_types, compile_keycodes, compile_symbols, CompileInput};
use crate::xkb::keymap::xkb_mod_name_to_index;
use std::sync::Arc;
pub(crate) const INCLUDE_MAX_DEPTH: i32 = 15_i32;
fn directory_for_include(type_0: FileType) -> &'static str {
    match type_0 {
        FileType::Keycodes => "keycodes",
        FileType::Types => "types",
        FileType::Symbols => "symbols",
        _ => "",
    }
}
pub(crate) fn find_file_in_xkb_path(
    ctx: &mut XkbContext,
    name: &str,
    type_0: FileType,
    offset: &mut u32,
) -> Option<Arc<[u8]>> {
    let type_dir = directory_for_include(type_0);
    let path_count = ctx.includes.len() as u32;
    for i in *offset..path_count {
        let path = format!("{}/{}/{}", ctx.includes[i as usize], type_dir, name);
        if let Some(data) = ctx.read_file(&path) {
            *offset = i;
            return Some(data);
        }
    }
    None
}
pub(crate) fn exceeds_include_max_depth(include_depth: u32) -> bool {
    include_depth >= INCLUDE_MAX_DEPTH as u32
}
pub(crate) fn process_include_stream(
    ctx: &mut XkbContext,
    stmt: &IncludeStmt,
    file_type: FileType,
) -> Option<OwnedMap> {
    let mut offset = 0;
    let mut candidate = None;
    loop {
        let data = if stmt.file.starts_with('/') {
            (offset == 0).then(|| ctx.read_file(&stmt.file)).flatten()
        } else {
            find_file_in_xkb_path(ctx, &stmt.file, file_type, &mut offset)
        };
        let Some(data) = data else { break };
        if let Some(parsed) = xkb_select_owned(data, &stmt.map) {
            if parsed.file_type == file_type {
                if !stmt.map.is_empty() || parsed.flags != 0 {
                    return Some(parsed);
                }
                candidate.get_or_insert(parsed);
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
            .map(|level| level.sym)
            .unwrap_or(XKB_KEY_NO_SYMBOL);
        key.repeat.get_or_insert_with(|| {
            first_sym != XKB_KEY_NO_SYMBOL && !is_modifier_keysym(first_sym)
        });
        if key.vmodmap.is_none() {
            let mut vmodmap = 0;
            let level_one_sym = key
                .groups
                .first()
                .and_then(|group| group.levels.first())
                .map(|level| level.sym);
            if let Some(sym) = level_one_sym {
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
                if level.action.is_none() {
                    level.action = wkb_group_action(level.sym);
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
        .fold(keymap.num_groups, |max, key| {
            max.max(key.groups.len() as u32)
        });
    apply_wkb_compat(info);
    update_mod_mappings(info);
    compute_type_entry_masks(info);
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
pub(crate) fn compile_components(
    keycodes: &[u8],
    types: &[u8],
    symbols: &[u8],
    keymap: &mut XkbKeymap,
) -> bool {
    let parse = |bytes: &[u8]| {
        std::str::from_utf8(bytes)
            .ok()
            .and_then(|input| include_create(input, MergeMode::Default))
    };
    let (Some(mut keycodes), Some(mut types), Some(mut symbols)) =
        (parse(keycodes), parse(types), parse(symbols))
    else {
        return false;
    };
    compile_keycodes(CompileInput::Includes(&mut keycodes), keymap)
        && compile_key_types(CompileInput::Includes(&mut types), keymap)
        && compile_symbols(CompileInput::Includes(&mut symbols), keymap)
        && update_derived_keymap_fields(keymap)
}
pub(crate) fn compile_keymap_stream(file: SelectedMap<'_>, keymap: &mut XkbKeymap) -> bool {
    if file.file_type != FileType::Keymap {
        return false;
    }
    let mut parts: [Option<SelectedMap<'_>>; 3] = [None, None, None];
    let mut maps = Stream::new(file.body);
    loop {
        let map = match maps.next_map() {
            Ok(Some(map)) => map,
            Ok(None) => break,
            Err(()) => return false,
        };
        let index = map.file_type as usize;
        if index < parts.len() && parts[index].is_none() {
            parts[index] = Some(map);
        }
    }
    let mut keycodes = parts[0].as_ref().map(|map| Stream::new(map.body));
    let mut types = parts[1].as_ref().map(|map| Stream::new(map.body));
    let mut symbols = parts[2].as_ref().map(|map| Stream::new(map.body));
    compile_keycodes(CompileInput::Stream(keycodes.as_mut()), keymap)
        && compile_key_types(CompileInput::Stream(types.as_mut()), keymap)
        && compile_symbols(CompileInput::Stream(symbols.as_mut()), keymap)
        && update_derived_keymap_fields(keymap)
}
use lasso::Key as _;
use std::collections::HashMap;
#[derive(Clone, Debug, Default)] #[rustfmt::skip] pub(crate) struct XkbRuleNames { pub(crate) layout: String, pub(crate) variant: String, pub(crate) options: String }
#[derive(Clone)] #[rustfmt::skip] pub(crate) struct XkbContext { pub(crate) includes: Vec<String>, pub(crate) atom_table: lasso::Rodeo, pub(crate) files: HashMap<String, Arc<[u8]>> }
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
    pub(crate) fn read_file(&mut self, path: &str) -> Option<Arc<[u8]>> {
        if let Some(data) = self.files.get(path) {
            return Some(data.clone());
        }
        let data: Arc<[u8]> = std::fs::read(path).ok()?.into();
        self.files.insert(path.to_owned(), data.clone());
        Some(data)
    }
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
    const ABSOLUTE_SWITCH       = 32; } }
#[derive(Copy, Clone, Default, PartialEq, Eq)] #[rustfmt::skip] pub struct XkbGroupAction { pub flags: ActionFlags, pub group: i32 }
#[derive(Copy, Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbMods { pub(crate) mods: u32, pub(crate) mask: u32 }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbKeyType { pub(crate) name: u32, pub(crate) modifiers_set: bool, pub(crate) mods: XkbMods, pub(crate) num_levels: u32, pub(crate) entries: Vec<XkbKeyTypeEntry> }
#[derive(Copy, Clone)] #[rustfmt::skip] pub(crate) struct XkbKeyTypeEntry { pub(crate) level: u32, pub(crate) mods: XkbMods, pub(crate) preserve: XkbMods }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbKey { pub(crate) name: u32, pub(crate) default_type: u32, pub(crate) modmap: u32, pub(crate) vmodmap: Option<u32>, pub(crate) repeat: Option<bool>, pub(crate) groups: Vec<XkbGroup> }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbGroup { pub(crate) explicit_syms: bool, pub(crate) type_idx: u32, pub(crate) levels: Vec<XkbLevel> }
#[derive(Clone, Default)] #[rustfmt::skip] pub(crate) struct XkbLevel { pub(crate) sym: u32, pub(crate) action: Option<XkbAction> }
pub(crate) const XKB_MAX_GROUPS: u32 = 32;
pub(crate) const MOD_REAL_MASK_ALL: u32 = 0xff_i32 as u32;
pub(crate) const DFLT_XKB_LEGACY_ROOT: &str = "/usr/share/X11/xkb";
pub(crate) const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub(crate) const XKB_KEYSYM_MAX: u32 = 0x1fffffff;
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
#[derive(Copy, Clone)]
pub(crate) enum Scalar {
    Integer(i64),
    Ident(u32),
}
pub(crate) struct ScalarExpr {
    pub(crate) terms: arrayvec::ArrayVec<(bool, Scalar), 8>,
    pub(crate) invert: bool,
}
pub(crate) enum ExprKind {
    String(u32),
    KeyName(u32),
    KeySym(u32),
    Scalar(ScalarExpr),
    Symbols(Vec<u32>),
    Actions,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Element {
    None,
    Key,
    Type,
    Other,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Field {
    Type,
    Symbols,
    Actions,
    Vmods,
    Repeat,
    Name,
    Modifiers,
    LevelName,
    Map,
    Preserve,
    Ignored,
    Other,
}
#[rustfmt::skip] pub(crate) struct Lhs { pub(crate) element: Element, pub(crate) field: Field, pub(crate) index: Option<ExprKind> }
#[rustfmt::skip] pub(crate) struct VarDef { pub(crate) merge: MergeMode, pub(crate) name: Option<Lhs>, pub(crate) value: Option<ExprKind> }
#[rustfmt::skip] pub(crate) struct VModDef { pub(crate) merge: MergeMode, pub(crate) name: u32, pub(crate) value: Option<ExprKind> }
#[derive(Copy, Clone)] #[rustfmt::skip] pub(crate) struct KeycodeDef { pub(crate) merge: MergeMode, pub(crate) name: u32, pub(crate) value: i64 }
#[derive(Copy, Clone)] #[rustfmt::skip] pub(crate) struct KeyAliasDef { pub(crate) alias: u32, pub(crate) real: u32 }
#[rustfmt::skip] pub(crate) struct NamedVarDef<'a> { pub(crate) merge: MergeMode, pub(crate) name: u32, pub(crate) body: &'a [u8] }
#[rustfmt::skip] pub(crate) struct ModMapDef { pub(crate) merge: MergeMode, pub(crate) modifier: u32, pub(crate) keys: Vec<ExprKind> }
pub(crate) const MAP_IS_DEFAULT: u32 = 1;
pub(crate) enum Statement<'a> {
    Include(Vec<IncludeStmt>),
    Keycode(KeycodeDef),
    KeyAlias(KeyAliasDef),
    Var(VarDef),
    VMods(Vec<VModDef>),
    KeyType(NamedVarDef<'a>),
    Symbols(NamedVarDef<'a>),
    ModMap(ModMapDef),
    Unknown,
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
pub(crate) fn parse_hex_u32(s: &[u8]) -> (u32, i32) {
    parse_u32(s, 16)
}
