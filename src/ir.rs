//! See `docs/layout-format.md`.

use crate::ir_tables::{
    default_level0_char, evdev_named, DEFAULT_MODIFIER_KEYS, MOD_KEY_NAMED, MOD_TYPE_NAMED,
    STANDARD_NAMED, STANDARD_NO_REPEAT, STANDARD_REPEAT_MAX,
};

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

#[cfg(feature = "client")]
use crate::composer::{Composer, Token};
use crate::flat_keymap::{FlatMap, FlatMapValue, MAX_LEVELS};
use crate::modifiers::{ModKind, ModType, Modifier, Modifiers, StateModifier};
use crate::named_keys::NamedKey;
use crate::{FlatKeymap, FlatNamedKeyMap, KBLayout, KeyBitSet};
#[cfg(feature = "xkb")]
use crate::keysym_to_named_key;

/// Current version of the layout file schema. Files with a different version
/// are rejected by [`LayoutFile::validate`].
pub const FORMAT_VERSION: u32 = 1;

/// Number of evdev keycode slots, fixed at compile time. Every keycode in a
/// layout file is `< NUM_KEYS`. Chosen well above the current maximum keycode
/// in xkb-data registries (a few above 700) so newer registries validate too.
pub const NUM_KEYS: u32 = 1024;

/// Character used to represent the Compose/Multi_key token inside a serialized
/// compose sequence. Reserved: a literal U+00B7 key cannot be represented.
pub const COMPOSE_KEY_CHAR: char = '\u{b7}';

pub type CharSection = BTreeMap<u8, BTreeMap<u32, char>>;

/// A section mapping level -> keycode -> named key (`keysym_map`).
pub type NamedSection = BTreeMap<u8, BTreeMap<u32, NamedKey>>;

/// Modifier bindings: `(keycode, [(level, action)])`.
pub type ModifierList = Vec<(u32, Vec<(u8, ModAction)>)>;

/// Errors from validating, serializing, or converting layout files.
#[derive(Debug, thiserror::Error)]
pub enum IrError {
    #[error("unsupported format version {0}")]
    UnsupportedVersion(u32),
    #[error("layout name must not be empty")]
    EmptyLayoutName,
    #[error("invalid layout index {0}")]
    InvalidLayoutIndex(usize),
    #[error("keycode {0} out of range (num_keys={1})")]
    KeycodeOutOfRange(u32, u32),
    #[error("level {0} out of range (max 8)")]
    LevelOutOfRange(u8),
    #[error("modifier at keycode {0} has no actions")]
    EmptyModifierActions(u32),
    #[error("empty compose sequence")]
    EmptyComposeSequence,
    #[error("compose output is NUL")]
    NullComposeOutput,
    #[error("compose sequence contains NUL")]
    NullComposeKey,
    #[error("serialization error: {0}")]
    Serialize(String),
    #[error("deserialization error: {0}")]
    Deserialize(String),
}

/// One modifier action, mirroring the runtime [`ModKind`] in a serializable
/// form. The `ModType` argument follows the surrounding XKB convention, e.g.
/// `Press(Level2)`, `Lock(Caps)`, `Lock(Num)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModAction {
    Press(ModType),
    Lock(ModType),
    UnlockOnPress(ModType),
    Latch(ModType),
}

const DEFAULT_MODIFIERS: &[(u32, ModAction)] = &[
    (29, ModAction::Press(ModType::None)),
    (97, ModAction::Press(ModType::None)),
    (58, ModAction::Lock(ModType::Caps)),
    (69, ModAction::Lock(ModType::Num)),
];

impl ModAction {
    fn mod_type(self) -> ModType {
        match self {
            Self::Press(t) | Self::Lock(t) | Self::UnlockOnPress(t) | Self::Latch(t) => t,
        }
    }
}

/// On-disk RON layout (see `docs/layout-format.md`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LayoutFile {
    pub version: u32,
    pub layout: String,
    #[serde(default)]
    pub repeat_keys: Vec<u32>,
    #[serde(default)]
    pub repeat_remove: Vec<u32>,
    #[serde(default)]
    pub modifiers: ModifierList,
    pub keymap: CharSection,
    #[serde(default)]
    pub num_lock_keys: CharSection,
    #[serde(default)]
    pub caps_lock_keymap: CharSection,
    #[serde(default)]
    pub keysym_map: NamedSection,
    #[serde(default)]
    pub compose: Vec<(Vec<char>, char)>,
}

impl LayoutFile {
    /// Validate all structural invariants. Called automatically by
    /// [`LayoutFile::to_ron_string`], [`LayoutFile::from_ron_str`], and the
    /// conversions to/from [`KBLayout`].
    pub fn validate(&self) -> Result<(), IrError> {
        if self.version != FORMAT_VERSION {
            return Err(IrError::UnsupportedVersion(self.version));
        }
        if self.layout.is_empty() {
            return Err(IrError::EmptyLayoutName);
        }
        for keycode in self
            .repeat_keys
            .iter()
            .chain(&self.repeat_remove)
        {
            if *keycode >= NUM_KEYS {
                return Err(IrError::KeycodeOutOfRange(*keycode, NUM_KEYS));
            }
        }
        for (keycode, actions) in &self.modifiers {
            if *keycode >= NUM_KEYS {
                return Err(IrError::KeycodeOutOfRange(*keycode, NUM_KEYS));
            }
            if actions.is_empty() {
                return Err(IrError::EmptyModifierActions(*keycode));
            }
            for (level, _) in actions {
                if *level >= MAX_LEVELS as u8 {
                    return Err(IrError::LevelOutOfRange(*level));
                }
            }
        }
        for section in [
            &self.keymap,
            &self.num_lock_keys,
            &self.caps_lock_keymap,
        ] {
            validate_section(section)?;
        }
        validate_section(&self.keysym_map)?;
        for (keys, output) in &self.compose {
            if keys.is_empty() {
                return Err(IrError::EmptyComposeSequence);
            }
            if *output == '\0' {
                return Err(IrError::NullComposeOutput);
            }
            if keys.contains(&'\0') {
                return Err(IrError::NullComposeKey);
            }
        }
        Ok(())
    }

    /// Serialize to canonical RON text. Fails on invalid input.
    pub fn to_ron_string(&self) -> Result<String, IrError> {
        self.validate()?;
        Ok(serialize_to_ron(self))
    }

    /// Deserialize from RON text and validate.
    pub fn from_ron_str(s: &str) -> Result<Self, IrError> {
        let file: Self = ron::from_str(s).map_err(|e| IrError::Deserialize(e.to_string()))?;
        file.validate()?;
        Ok(file)
    }
}

// --- RON serialization ---

/// How many repeat-key codes per wrapped line.
const RON_WRAP_WIDTH: usize = 20;

/// Keycodes per wrapped line in char-keyed sections; lines break when a keycode
/// exceeds a multiple of this value.
const RON_KEYS_PER_LINE: usize = 14;

/// Serialize a value in RON's compact spaced form (arrays inline).
fn ron_value<T: Serialize>(value: &T) -> String {
    ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::new().compact_arrays(true)).unwrap()
}

fn serialize_to_ron(file: &LayoutFile) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "(");
    let _ = writeln!(out, "    version: {},", file.version);
    let _ = writeln!(out, "    layout: {},", ron_value(&file.layout));
    if !file.repeat_remove.is_empty() {
        write_integer_list(&mut out, "repeat_remove", &file.repeat_remove);
    } else if !file.repeat_keys.is_empty() {
        write_integer_list(&mut out, "repeat_keys", &file.repeat_keys);
    }
    if !file.modifiers.is_empty() {
        write_entries(&mut out, "modifiers", &file.modifiers);
    }
    write_char_section(&mut out, "keymap", &file.keymap);
    write_char_section(&mut out, "num_lock_keys", &file.num_lock_keys);
    write_char_section(&mut out, "caps_lock_keymap", &file.caps_lock_keymap);
    write_named_section(&mut out, "keysym_map", &file.keysym_map);
    if !file.compose.is_empty() {
        write_entries(&mut out, "compose", &file.compose);
    }
    let _ = writeln!(out, ")");
    out
}

fn write_ron_char(out: &mut String, c: char) {
    out.push('\'');
    out.extend(c.escape_debug());
    out.push('\'');
}

/// Write a `u32` list as one array, wrapping at [`RON_WRAP_WIDTH`] per line.
fn write_integer_list(out: &mut String, name: &str, values: &[u32]) {
    let _ = write!(out, "    {name}: [");
    for (i, chunk) in values.chunks(RON_WRAP_WIDTH).enumerate() {
        let _ = write!(
            out,
            "{}{}",
            if i > 0 { ",\n        " } else { "" },
            chunk
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let _ = writeln!(out, "],");
}

/// Write a slice of RON-serializable tuples, one per line.
fn write_entries<T: Serialize>(out: &mut String, name: &str, entries: &[T]) {
    let _ = writeln!(out, "    {name}: [");
    for entry in entries {
        let _ = writeln!(out, "        {},", ron_value(entry));
    }
    let _ = writeln!(out, "    ],");
}

fn write_char_section(out: &mut String, name: &str, section: &CharSection) {
    write_section(out, name, section, Some(RON_KEYS_PER_LINE), |out, value| {
        write_ron_char(out, *value);
    });
}

fn write_named_section(out: &mut String, name: &str, section: &NamedSection) {
    write_section(out, name, section, None, |out, key| {
        out.push_str(&ron_value(key));
    });
}

/// Write a level-keyed section as nested maps, wrapping per `keys_per_line`.
fn write_section<T>(
    out: &mut String,
    name: &str,
    section: &BTreeMap<u8, BTreeMap<u32, T>>,
    keys_per_line: Option<usize>,
    write_value: impl Fn(&mut String, &T),
) {
    if section.is_empty() {
        return;
    }
    let _ = writeln!(out, "    {name}: {{");
    let indent = "            ";
    for (level, keys) in section {
        let _ = writeln!(out, "        {level}: {{");
        let mut prev: Option<u32> = None;
        for (keycode, value) in keys {
            if let Some(p) = prev {
                out.push(',');
                let newline = keys_per_line.is_none_or(|n| {
                    keycode.saturating_sub(1) as usize / n != p.saturating_sub(1) as usize / n
                });
                out.push_str(if newline { "\n            " } else { " " });
            } else {
                out.push_str(indent);
            }
            prev = Some(*keycode);
            let _ = write!(out, "{keycode}: ");
            write_value(out, value);
        }
        let _ = writeln!(out, ",");
        let _ = writeln!(out, "        }},");
    }
    let _ = writeln!(out, "    }},");
}

fn validate_section<T>(section: &BTreeMap<u8, BTreeMap<u32, T>>) -> Result<(), IrError> {
    for (level, keys) in section {
        if *level >= MAX_LEVELS as u8 {
            return Err(IrError::LevelOutOfRange(*level));
        }
        for keycode in keys.keys() {
            if *keycode >= NUM_KEYS {
                return Err(IrError::KeycodeOutOfRange(*keycode, NUM_KEYS));
            }
        }
    }
    Ok(())
}

impl TryFrom<&KBLayout> for LayoutFile {
    type Error = IrError;

    fn try_from(layout: &KBLayout) -> Result<Self, IrError> {
        let num_keys = layout.state_keymap.num_keys as u32;
        let hinted_keymap = keymap_with_named_char_hints(&layout.state_keymap, &layout.named_key_map);
        let keymap = normalized_char_section(&hinted_keymap, CharNorm::Keymap);
        let max_level = keymap.keys().max().copied().unwrap_or(0);
        let modifiers = normalized_modifiers(&layout.modifiers, max_level, &keymap);
        let derived = derived_repeat_keys(&modifiers, &keymap);
        let actual: BTreeSet<u32> = (0..num_keys)
            .filter(|&k| layout.repeat_keys.contains(k))
            .collect();
        let repeat_remove: Vec<u32> = derived
            .into_iter()
            .filter(|k| !actual.contains(k))
            .collect();
        let state_keymap = &layout.state_keymap;
        let file = LayoutFile {
            version: FORMAT_VERSION,
            layout: layout.name.clone(),
            repeat_keys: Vec::new(),
            repeat_remove,
            modifiers,
            keymap,
            num_lock_keys: normalized_char_section(
                &layout.num_lock_keys,
                CharNorm::Override(state_keymap),
            ),
            caps_lock_keymap: normalized_char_section(
                &layout.caps_lock_keymap,
                CharNorm::Override(state_keymap),
            ),
            keysym_map: normalized_named_section(&layout.named_key_map, &hinted_keymap, &layout.modifiers),
            compose: compose_section(layout),
        };
        file.validate()?;
        Ok(file)
    }
}

fn compose_section(layout: &KBLayout) -> Vec<(Vec<char>, char)> {
    #[cfg(feature = "client")]
    {
        let mut reachable: Vec<char> = layout
            .state_keymap
            .data
            .iter()
            .chain(&layout.caps_lock_keymap.data)
            .chain(&layout.num_lock_keys.data)
            .filter_map(|ch| *ch)
            .collect();
        reachable.sort_unstable();
        reachable.dedup();
        compose_from_composer(&layout.composer, &reachable)
    }
    #[cfg(not(feature = "client"))]
    {
        let _ = layout;
        Vec::new()
    }
}

fn effective_levels<T: FlatMapValue + PartialEq>(flat: &FlatMap<T>) -> usize {
    let mut n = flat.num_levels;
    while n > 1 {
        let half = n / 2;
        if planes_equal(flat, 0, half, half) {
            n = half;
        } else {
            break;
        }
    }
    n
}

fn planes_equal<T: FlatMapValue + PartialEq>(flat: &FlatMap<T>, a: usize, b: usize, count: usize) -> bool {
    let nk = flat.num_keys;
    let len = count * nk;
    flat.data[a * nk..a * nk + len] == flat.data[b * nk..b * nk + len]
}

enum CharNorm<'a> {
    Keymap,
    Override(&'a FlatKeymap),
}

fn set_keymap_char(map: &mut FlatKeymap, level: usize, keycode: u32, ch: char) {
    let k = keycode as usize;
    if k < map.num_keys && level < map.num_levels {
        map.data[level * map.num_keys + k] = Some(ch);
    }
}

/// When XKB binds `Delete` without a character, export `\u{7f}` so named identity
/// roundtrips through `keymap` instead of `keysym_map`.
fn keymap_with_named_char_hints(
    state_keymap: &FlatKeymap,
    named_key_map: &FlatNamedKeyMap,
) -> FlatKeymap {
    let mut out = state_keymap.clone();
    for level in 0..out.num_levels.min(named_key_map.num_levels) {
        for keycode in 0..out.num_keys {
            let kc = keycode as u32;
            if out.get(level, kc).is_some() {
                continue;
            }
            if named_key_map.get(level, kc) == NamedKey::Delete {
                set_keymap_char(&mut out, level, kc, '\u{7f}');
            }
        }
    }
    out
}

fn char_redundant_at_level(flat: &FlatKeymap, level: usize, keycode: u32, ch: char) -> bool {
    if level == 0 {
        return default_level0_char(keycode) == Some(ch);
    }
    (0..level).any(|l| flat.get(l, keycode) == Some(ch))
}

fn normalized_char_section(flat: &FlatKeymap, mode: CharNorm<'_>) -> CharSection {
    let num_levels = effective_levels(flat);
    let mut section = BTreeMap::new();
    for level in 0..num_levels {
        let keys: BTreeMap<_, _> = (0..flat.num_keys)
            .filter_map(|kc| {
                let kc = kc as u32;
                let ch = flat.get(level, kc)?;
                let skip = match mode {
                    CharNorm::Keymap => char_redundant_at_level(flat, level, kc, ch),
                    CharNorm::Override(base) => base.get(level, kc) == Some(ch),
                };
                (!skip).then_some((kc, ch))
            })
            .collect();
        if !keys.is_empty() {
            section.insert(level as u8, keys);
        }
    }
    section
}

fn fill_char_section(levels: &CharSection, num_keys: usize, num_levels: usize, inherit: bool) -> FlatKeymap {
    let mut flat = FlatKeymap::with_levels(num_keys, num_levels);
    for (level, keys) in levels {
        let base = (*level as usize) * num_keys;
        for (keycode, ch) in keys {
            flat.data[base + *keycode as usize] = Some(*ch);
        }
    }
    if inherit {
        for level in 1..num_levels {
            for keycode in 0..num_keys {
                let idx = level * num_keys + keycode;
                if flat.data[idx].is_none() {
                    flat.data[idx] = flat.data[keycode];
                }
            }
        }
        for keycode in 0..num_keys {
            if flat.data[keycode].is_none() {
                flat.data[keycode] = default_level0_char(keycode as u32);
            }
        }
    }
    flat
}

fn expand_keymap_section(levels: &CharSection, num_keys: usize) -> FlatKeymap {
    let max_level = levels.keys().max().copied().unwrap_or(0) as usize;
    fill_char_section(levels, num_keys, max_level + 1, true)
}

fn expand_override_section(levels: &CharSection, num_keys: usize, num_levels: usize) -> FlatKeymap {
    fill_char_section(levels, num_keys, num_levels, false)
}

fn named_from_modifier(keycode: u32, modifier: &Modifier) -> NamedKey {
    let mut named = NamedKey::Unnamed;
    modifier.for_each(|state| {
        if named != NamedKey::Unnamed {
            return;
        }
        let t = state.mod_type;
        named = MOD_TYPE_NAMED
            .iter()
            .find(|(mt, _)| *mt == t)
            .map(|(_, n)| *n)
            .or_else(|| {
                MOD_KEY_NAMED
                    .iter()
                    .find(|(mt, kc, _)| *mt == t && *kc == keycode)
                    .map(|(_, _, n)| *n)
            })
            .unwrap_or(NamedKey::Unnamed);
    });
    named
}

fn char_to_named_key(ch: char) -> NamedKey {
    match ch {
        '\u{8}' => NamedKey::Backspace,
        '\t' => NamedKey::Tab,
        '\r' | '\n' => NamedKey::Enter,
        '\u{1b}' => NamedKey::Escape,
        '\u{7f}' => NamedKey::Delete,
        ' ' => NamedKey::Space,
        #[cfg(feature = "xkb")]
        _ => keysym_to_named_key(0x0100_0000 | ch as u32),
        #[cfg(not(feature = "xkb"))]
        _ => NamedKey::Unnamed,
    }
}

fn default_named_key(
    level: usize,
    keycode: u32,
    keymap: &FlatKeymap,
    modifiers: &Modifiers,
) -> NamedKey {
    keymap
        .get(level, keycode)
        .and_then(|ch| {
            let n = char_to_named_key(ch);
            (n != NamedKey::Unnamed).then_some(n)
        })
        .or_else(|| {
            STANDARD_NAMED
                .iter()
                .find(|(l, k, _)| *l == level as u8 && *k == keycode)
                .map(|(_, _, n)| *n)
        })
        .or_else(|| modifiers.get(keycode).map(|m| named_from_modifier(keycode, m)))
        .filter(|n| *n != NamedKey::Unnamed)
        .unwrap_or_else(|| evdev_named(keycode))
}

fn derive_named_key_map(
    num_keys: usize,
    num_levels: usize,
    keymap: &FlatKeymap,
    modifiers: &Modifiers,
) -> FlatNamedKeyMap {
    let mut flat = FlatNamedKeyMap::with_levels(num_keys, num_levels);
    for level in 0..num_levels {
        for keycode in 0..num_keys {
            flat.data[level * num_keys + keycode] =
                default_named_key(level, keycode as u32, keymap, modifiers);
        }
    }
    flat
}

fn normalized_named_section(
    flat: &FlatNamedKeyMap,
    keymap: &FlatKeymap,
    modifiers: &Modifiers,
) -> NamedSection {
    let num_levels = effective_levels(flat);
    let mut section = BTreeMap::new();
    for level in 0..num_levels {
        let keys: BTreeMap<_, _> = (0..flat.num_keys)
            .filter_map(|kc| {
                let kc = kc as u32;
                let named = flat.get(level, kc);
                if named == NamedKey::Unnamed {
                    return None;
                }
                let default = default_named_key(level, kc, keymap, modifiers);
                (named != default).then_some((kc, named))
            })
            .collect();
        if !keys.is_empty() {
            section.insert(level as u8, keys);
        }
    }
    section
}

fn apply_named_overrides(flat: &mut FlatNamedKeyMap, overrides: &NamedSection) {
    for (level, keys) in overrides {
        let base = (*level as usize) * flat.num_keys;
        for (keycode, named) in keys {
            flat.data[base + *keycode as usize] = *named;
        }
    }
}

fn key_in_section(section: &CharSection, keycode: u32) -> bool {
    section.values().any(|keys| keys.contains_key(&keycode))
}

fn level_modifier_reachable(mod_type: ModType, max_level: u8) -> bool {
    match mod_type {
        ModType::Level2 => max_level >= 1,
        ModType::Level3 => max_level >= 2,
        ModType::Level5 => max_level >= 4,
        _ => true,
    }
}

fn normalized_modifiers(
    modifiers: &Modifiers,
    max_level: u8,
    keymap: &CharSection,
) -> ModifierList {
    let has_altgr = modifiers.iter().any(|(k, m)| {
        *k == 100
            && actions_from_modifier(m)
                .iter()
                .any(|(_, a)| *a == ModAction::Press(ModType::Level3))
    });
    let mut out = Vec::new();
    for (keycode, modifier) in modifiers.iter() {
        let actions: Vec<_> = actions_from_modifier(modifier)
            .into_iter()
            .filter(|(_, a)| level_modifier_reachable(a.mod_type(), max_level))
            .collect();
        if actions.is_empty() {
            continue;
        }
        if DEFAULT_MODIFIERS
            .iter()
            .find(|(k, _)| *k == *keycode)
            .is_some_and(|(_, d)| actions == [(0, *d)])
            && !key_in_section(keymap, *keycode)
        {
            continue;
        }
        if *keycode == 84 && has_altgr && actions == [(0, ModAction::Press(ModType::Level3))] {
            continue;
        }
        out.push((*keycode, actions));
    }
    out.sort_by_key(|(keycode, _)| *keycode);
    out
}

fn derived_repeat_keys(modifiers: &ModifierList, keymap: &CharSection) -> BTreeSet<u32> {
    let modifier_codes: BTreeSet<u32> = modifiers.iter().map(|(k, _)| *k).collect();
    let mut out = BTreeSet::new();
    for keycode in 1..=STANDARD_REPEAT_MAX {
        if !STANDARD_NO_REPEAT.contains(&keycode) && !modifier_codes.contains(&keycode) {
            out.insert(keycode);
        }
    }
    for keys in keymap.values() {
        for &keycode in keys.keys() {
            if !modifier_codes.contains(&keycode) {
                out.insert(keycode);
            }
        }
    }
    out
}

fn resolve_repeat_keys(file: &LayoutFile) -> KeyBitSet {
    let mut repeat_keys = KeyBitSet::default();
    if !file.repeat_keys.is_empty() {
        for &keycode in &file.repeat_keys {
            repeat_keys.insert(keycode);
        }
        return repeat_keys;
    }
    let mut set = derived_repeat_keys(&file.modifiers, &file.keymap);
    for &keycode in &file.repeat_remove {
        set.remove(&keycode);
    }
    for keycode in set {
        repeat_keys.insert(keycode);
    }
    repeat_keys
}

fn actions_from_modifier(modifier: &Modifier) -> Vec<(u8, ModAction)> {
    match modifier {
        Modifier::Single(kind) => vec![(0, modaction_from_state_modifier(kind))],
        Modifier::Leveled(map) => map
            .iter()
            .map(|(level, kind)| (*level, modaction_from_state_modifier(kind)))
            .collect(),
    }
}

fn modaction_from_state_modifier(kind: &StateModifier) -> ModAction {
    match kind.kind {
        ModKind::Press { .. } => ModAction::Press(kind.mod_type),
        ModKind::Lock { .. } => ModAction::Lock(kind.mod_type),
        ModKind::UnlockOnPress { .. } => ModAction::UnlockOnPress(kind.mod_type),
        ModKind::Latch { .. } => ModAction::Latch(kind.mod_type),
    }
}

#[cfg(feature = "client")]
/// Depth-first walk of the composer trie emitting reachable, sorted sequences.
fn compose_from_composer(composer: &Composer, reachable: &[char]) -> Vec<(Vec<char>, char)> {
    let mut out = Vec::new();
    let mut path = Vec::new();
    dfs_compose(composer, 0, &mut path, &mut out, reachable);
    out.sort();
    out
}

#[cfg(feature = "client")]
fn dfs_compose(
    composer: &Composer,
    node: u32,
    path: &mut Vec<char>,
    out: &mut Vec<(Vec<char>, char)>,
    reachable: &[char],
) {
    let node = &composer.nodes[node as usize];
    if let Some(emit) = node.emit {
        let reachable = path
            .iter()
            .all(|ch| *ch == COMPOSE_KEY_CHAR || reachable.binary_search(ch).is_ok());
        if reachable {
            out.push((path.clone(), emit));
        }
    }
    for &(key, child) in &node.children {
        path.push(if key == 0 {
            COMPOSE_KEY_CHAR
        } else {
            char::from_u32(key).unwrap_or('\u{fffd}')
        });
        dfs_compose(composer, child, path, out, reachable);
        path.pop();
    }
}

impl TryFrom<LayoutFile> for KBLayout {
    type Error = IrError;

    fn try_from(file: LayoutFile) -> Result<Self, IrError> {
        file.validate()?;
        let num_keys = NUM_KEYS as usize;

        let repeat_keys = resolve_repeat_keys(&file);

        let mut modifiers = Modifiers::new();
        for (keycode, actions) in &file.modifiers {
            let modifier = match actions.as_slice() {
                [(0, action)] => Modifier::Single(modkind_from_modaction(*action)),
                _ => Modifier::Leveled(
                    actions
                        .iter()
                        .map(|(level, action)| (*level, modkind_from_modaction(*action)))
                        .collect(),
                ),
            };
            modifiers.set_modifier(*keycode, modifier);
        }
        for &(keycode, mod_type, lock) in DEFAULT_MODIFIER_KEYS {
            if modifiers.get(keycode).is_none() && !key_in_section(&file.keymap, keycode) {
                let action = if lock {
                    ModAction::Lock(mod_type)
                } else {
                    ModAction::Press(mod_type)
                };
                modifiers.set_modifier(keycode, Modifier::Single(modkind_from_modaction(action)));
            }
        }

        #[cfg(feature = "client")]
        let composer = composer_from_compose(&file.compose);

        let state_keymap = expand_keymap_section(&file.keymap, num_keys);
        let num_levels = state_keymap.num_levels;
        let num_lock_keys = expand_override_section(&file.num_lock_keys, num_keys, num_levels);
        let caps_lock_keymap = expand_override_section(&file.caps_lock_keymap, num_keys, num_levels);
        let mut named_key_map =
            derive_named_key_map(num_keys, num_levels, &state_keymap, &modifiers);
        apply_named_overrides(&mut named_key_map, &file.keysym_map);

        Ok(KBLayout {
            name: file.layout,
            repeat_keys,
            #[cfg(feature = "client")]
            composer,
            modifiers,
            state_keymap,
            num_lock_keys,
            caps_lock_keymap,
            named_key_map,
            #[cfg(feature = "xkb")]
            caps_num_lock_keys: FlatKeymap::with_levels(num_keys, num_levels),
            #[cfg(feature = "xkb")]
            level_exceptions_keymap: FlatKeymap::with_levels(num_keys, 1),
        })
    }
}

#[rustfmt::skip]
fn modkind_from_modaction(action: ModAction) -> StateModifier {
    match action {
        ModAction::Press(t) => StateModifier { kind: ModKind::Press { pressed: false }, mod_type: t },
        ModAction::Lock(t) => StateModifier { kind: ModKind::Lock { pressed: false, locked: 0 }, mod_type: t },
        ModAction::UnlockOnPress(t) => StateModifier {
            kind: ModKind::UnlockOnPress {
                pressed: false,
                locked: false,
            },
            mod_type: t,
        },
        ModAction::Latch(t) => StateModifier { kind: ModKind::Latch { pressed: false, latched: false }, mod_type: t },
    }
}

#[cfg(feature = "client")]
fn composer_from_compose(sequences: &[(Vec<char>, char)]) -> Composer {
    let mut composer = Composer::new();
    for (keys, output) in sequences {
        let tokens: Vec<Token> = keys
            .iter()
            .map(|&ch| {
                if ch == COMPOSE_KEY_CHAR {
                    Token::Compose
                } else {
                    Token::Char(ch)
                }
            })
            .collect();
        composer.insert(&tokens, *output);
    }
    composer
}
