//! Intermediate representation (IR) for wkb layout data files.
//!
//! [`LayoutFile`] is the canonical on-disk (RON) form of a [`KBLayout`]. It is
//! bidirectional: [`LayoutFile::from_ron_str`] / `TryFrom<&KBLayout>` produce a
//! serializable file, and `TryFrom<LayoutFile>` rebuilds the runtime layout.
//! See `docs/layout-format.md` for the normative specification.
//!
//! The IR mirrors the serialized RON document one-to-one: `version`, a single
//! `layout` name, `repeat_keys`, `modifiers`, per-level section maps
//! (`keymap`, `num_lock_keys`, `caps_lock_keymap`, `keysym_map`), and a
//! `compose` table.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::composer::{Composer, Token};
use crate::flat_keymap::{FlatMap, FlatMapValue, MAX_LEVELS};
use crate::modifiers::{ModKind, ModType, Modifier, Modifiers};
use crate::named_keys::NamedKey;
use crate::{FlatKeymap, FlatNamedKeyMap, KBLayout, KeyBitSet};

/// Current version of the layout file schema. Files with a different version
/// are rejected by [`LayoutFile::validate`].
pub const FORMAT_VERSION: u32 = 1;

/// Number of evdev keycode slots, fixed at compile time. Every keycode in a
/// layout file is `< NUM_KEYS`.
pub const NUM_KEYS: u32 = 701;

/// Character used to represent the Compose/Multi_key token inside a serialized
/// compose sequence. Reserved: a literal U+00B7 key cannot be represented.
pub const COMPOSE_KEY_CHAR: char = '\u{b7}';

/// A section: level -> keycode -> character (used by `keymap`,
/// `num_lock_keys`, `caps_lock_keymap`).
pub type CharSection = BTreeMap<u8, BTreeMap<u32, char>>;

/// A section mapping level -> keycode -> named key (`keysym_map`).
pub type NamedSection = BTreeMap<u8, BTreeMap<u32, NamedKey>>;

/// Modifier bindings: `(keycode, name, [(level, action)])`.
pub type ModifierList = Vec<(u32, String, Vec<(u8, ModAction)>)>;

/// Errors from validating, serializing, or converting layout files.
#[derive(Debug, thiserror::Error)]
pub enum IrError {
    /// The file has an unsupported [`FORMAT_VERSION`].
    #[error("unsupported format version {0}")]
    UnsupportedVersion(u32),
    /// `layout` is empty.
    #[error("layout name must not be empty")]
    EmptyLayoutName,
    /// The requested layout index does not exist in the runtime instance.
    #[error("invalid layout index {0}")]
    InvalidLayoutIndex(usize),
    /// An evdev keycode is outside `0..NUM_KEYS`.
    #[error("keycode {0} out of range (num_keys={1})")]
    KeycodeOutOfRange(u32, u32),
    /// A level is at or above the maximum supported level.
    #[error("level {0} out of range (max 8)")]
    LevelOutOfRange(u8),
    /// A modifier binding has an empty name.
    #[error("empty modifier name at keycode {0}")]
    EmptyModifierName(u32),
    /// A modifier binding has no actions.
    #[error("modifier at keycode {0} has no actions")]
    EmptyModifierActions(u32),
    /// A compose sequence has no keys.
    #[error("empty compose sequence")]
    EmptyComposeSequence,
    /// A compose output is the NUL character.
    #[error("compose output is NUL")]
    NullComposeOutput,
    /// A compose sequence contains the NUL character.
    #[error("compose sequence contains NUL")]
    NullComposeKey,
    /// RON serialization failed.
    #[error("serialization error: {0}")]
    Serialize(String),
    /// RON deserialization failed.
    #[error("deserialization error: {0}")]
    Deserialize(String),
}

/// One modifier action, mirroring the runtime [`ModKind`] in a serializable
/// form. The `ModType` argument follows the surrounding XKB convention, e.g.
/// `Pressed(Level2)`, `Lock(Caps)`, `Lock(Num)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModAction {
    Pressed(ModType),
    Lock(ModType),
    Latch(ModType),
    None,
}

/// A persisted keyboard layout, mirroring the serialized RON document.
///
/// Maps are keyed by level (`u8`, ascending), then by evdev keycode (`u32`,
/// ascending). Using `BTreeMap` guarantees canonical, deterministic ordering.
/// Serialization is hand-rolled (`to_ron_string`) so empty sections are
/// omitted and lists are wrapped readably; deserialization uses the `ron`
/// crate, so fields that may be absent default to empty.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LayoutFile {
    /// Schema version, must equal [`FORMAT_VERSION`].
    pub version: u32,
    /// The single layout name.
    pub layout: String,
    /// Keycodes that repeat.
    #[serde(default)]
    pub repeat_keys: Vec<u32>,
    /// Modifier bindings as `(keycode, name, [(level, action)])`, sorted by keycode.
    #[serde(default)]
    pub modifiers: ModifierList,
    /// Resolved character per (level, keycode) under base modifiers.
    pub keymap: CharSection,
    /// Character overrides active while Num Lock is locked.
    #[serde(default)]
    pub num_lock_keys: CharSection,
    /// Character overrides active while Caps Lock is locked.
    #[serde(default)]
    pub caps_lock_keymap: CharSection,
    /// Named-key identities per (level, keycode); `Unnamed` entries are omitted.
    #[serde(default)]
    pub keysym_map: NamedSection,
    /// Compose sequences as `(keys, output)`. Only sequences whose keys are all
    /// reachable in this layout are stored.
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
        for keycode in &self.repeat_keys {
            if *keycode >= NUM_KEYS {
                return Err(IrError::KeycodeOutOfRange(*keycode, NUM_KEYS));
            }
        }
        for (keycode, mod_name, actions) in &self.modifiers {
            if *keycode >= NUM_KEYS {
                return Err(IrError::KeycodeOutOfRange(*keycode, NUM_KEYS));
            }
            if mod_name.is_empty() {
                return Err(IrError::EmptyModifierName(*keycode));
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
        for section in [&self.keymap, &self.num_lock_keys, &self.caps_lock_keymap] {
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

// ---------------------------------------------------------------------------
// RON serialization
// ---------------------------------------------------------------------------

/// How many repeat-key codes per wrapped line.
const RON_WRAP_WIDTH: usize = 20;

/// How many keycodes per wrapped line in char-keyed sections; lines break when
/// the keycode exceeds a multiple of this value.
const RON_KEYS_PER_LINE: usize = 14;

fn serialize_to_ron(file: &LayoutFile) -> String {
    let mut out = String::new();
    out.push_str("// wkb keyboard layout (RON format)\n(\n");
    out.push_str("    version: ");
    out.push_str(&file.version.to_string());
    out.push_str(",\n");
    out.push_str("    layout: ");
    write_ron_string(&mut out, &file.layout);
    out.push_str(",\n");

    if !file.repeat_keys.is_empty() {
        write_integer_list(&mut out, "repeat_keys", &file.repeat_keys);
    }
    if !file.modifiers.is_empty() {
        write_modifiers(&mut out, &file.modifiers);
    }
    write_char_section(&mut out, "keymap", &file.keymap);
    write_char_section(&mut out, "num_lock_keys", &file.num_lock_keys);
    write_char_section(&mut out, "caps_lock_keymap", &file.caps_lock_keymap);
    write_named_section(&mut out, "keysym_map", &file.keysym_map);
    if !file.compose.is_empty() {
        write_compose(&mut out, &file.compose);
    }

    out.push(')');
    out.push('\n');
    out
}

/// Write `s` as a RON string literal.
fn write_ron_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() || c == '\u{feff}' => {
                use std::fmt::Write as _;
                write!(out, "\\u{{{:x}}}", c as u32).unwrap();
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Write `c` as a RON char literal.
fn write_ron_char(out: &mut String, c: char) {
    match c {
        '\'' => out.push_str("'\\''"),
        '\\' => out.push_str("'\\\\'"),
        '\n' => out.push_str("'\\n'"),
        '\r' => out.push_str("'\\r'"),
        '\t' => out.push_str("'\\t'"),
        c if c.is_control() || c == '\u{feff}' => {
            use std::fmt::Write as _;
            write!(out, "'\\u{{{:x}}}'", c as u32).unwrap();
        }
        c => {
            out.push('\'');
            out.push(c);
            out.push('\'');
        }
    }
}

/// Write a `u32` list as one array, wrapping at [`RON_WRAP_WIDTH`] per line.
fn write_integer_list(out: &mut String, name: &str, values: &[u32]) {
    out.push_str("    ");
    out.push_str(name);
    out.push_str(": [");
    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            out.push(',');
            if i % RON_WRAP_WIDTH == 0 {
                out.push_str("\n        ");
            } else {
                out.push(' ');
            }
        }
        out.push_str(&value.to_string());
    }
    out.push_str("],\n");
}

/// Write the modifier bindings as a one-per-line list of tuples.
fn write_modifiers(out: &mut String, modifiers: &ModifierList) {
    out.push_str("    modifiers: [\n");
    for (keycode, name, actions) in modifiers {
        out.push_str("        (");
        out.push_str(&keycode.to_string());
        out.push_str(", ");
        write_ron_string(out, name);
        out.push_str(", [");
        for (i, (level, action)) in actions.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push('(');
            out.push_str(&level.to_string());
            out.push_str(", ");
            write_mod_action(out, *action);
            out.push(')');
        }
        out.push_str("]),\n");
    }
    out.push_str("    ],\n");
}

fn write_mod_action(out: &mut String, action: ModAction) {
    match action {
        ModAction::Pressed(mod_type) => write_action(out, "Pressed", mod_type),
        ModAction::Lock(mod_type) => write_action(out, "Lock", mod_type),
        ModAction::Latch(mod_type) => write_action(out, "Latch", mod_type),
        ModAction::None => out.push_str("None"),
    }
}

fn write_action(out: &mut String, variant: &str, mod_type: ModType) {
    out.push_str(variant);
    out.push('(');
    out.push_str(&format!("{mod_type:?}"));
    out.push(')');
}

fn write_char_section(out: &mut String, name: &str, section: &CharSection) {
    write_section(out, name, section, Some(RON_KEYS_PER_LINE), |out, value| {
        write_ron_char(out, *value);
    });
}

fn write_named_section(out: &mut String, name: &str, section: &NamedSection) {
    write_section(out, name, section, None, |out, key| {
        out.push_str(&format!("{key:?}"));
    });
}

/// Write a level-keyed section as nested maps. Char-keyed sections break the
/// line when a keycode exceeds a multiple of `keys_per_line`; named sections
/// put one key per line.
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
    out.push_str("    ");
    out.push_str(name);
    out.push_str(": {\n");
    for (level, keys) in section {
        out.push_str("        ");
        out.push_str(&level.to_string());
        out.push_str(": {\n");
        let indent = "            ";
        let mut prev_keycode: Option<u32> = None;
        for (keycode, value) in keys {
            if let Some(prev) = prev_keycode {
                out.push(',');
                let newline = match keys_per_line {
                    Some(n) => {
                        let prev_block = prev.saturating_sub(1) as usize / n;
                        let block = keycode.saturating_sub(1) as usize / n;
                        block != prev_block
                    }
                    None => true,
                };
                if newline {
                    out.push('\n');
                    out.push_str(indent);
                } else {
                    out.push(' ');
                }
            } else {
                out.push_str(indent);
            }
            prev_keycode = Some(*keycode);
            out.push_str(&keycode.to_string());
            out.push_str(": ");
            write_value(out, value);
        }
        out.push_str(",\n");
        out.push_str("        },\n");
    }
    out.push_str("    },\n");
}

/// Write compose sequences as one `(['key', ...], output)` tuple per line.
fn write_compose(out: &mut String, compose: &[(Vec<char>, char)]) {
    out.push_str("    compose: [\n");
    for (keys, output) in compose {
        out.push_str("        ([");
        for (i, ch) in keys.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            write_ron_char(out, *ch);
        }
        out.push_str("], ");
        write_ron_char(out, *output);
        out.push_str("),\n");
    }
    out.push_str("    ],\n");
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

// ---------------------------------------------------------------------------
// KBLayout -> LayoutFile (generation)
// ---------------------------------------------------------------------------

impl TryFrom<&KBLayout> for LayoutFile {
    type Error = IrError;

    fn try_from(layout: &KBLayout) -> Result<Self, IrError> {
        let num_keys = layout.state_keymap.num_keys as u32;
        let name = layout.name.clone();

        let mut repeat_keys = Vec::new();
        for keycode in 0..num_keys {
            if layout.repeat_keys.contains(keycode) {
                repeat_keys.push(keycode);
            }
        }

        let keymap = char_section(&layout.state_keymap);
        let num_lock_keys = char_section(&layout.num_lock_keys);
        let caps_lock_keymap = char_section(&layout.caps_lock_keymap);
        let keysym_map = named_section(&layout.named_key_map);

        let reachable = reachable_chars(layout);
        let compose = compose_from_composer(&layout.composer, &reachable);

        let file = LayoutFile {
            version: FORMAT_VERSION,
            layout: name,
            repeat_keys,
            modifiers: modifiers_from_layout(&layout.modifiers),
            keymap,
            num_lock_keys,
            caps_lock_keymap,
            keysym_map,
            compose,
        };
        file.validate()?;
        Ok(file)
    }
}

/// The set of characters this layout can produce, used to filter the compose
/// table so the file only keeps reachable (non-redundant) sequences.
fn reachable_chars(layout: &KBLayout) -> Vec<char> {
    let mut reachable: Vec<char> = layout
        .state_keymap
        .data
        .iter()
        .chain(layout.caps_lock_keymap.data.iter())
        .chain(layout.num_lock_keys.data.iter())
        .filter_map(|ch| *ch)
        .collect();
    reachable.sort_unstable();
    reachable.dedup();
    reachable
}

/// Convert a flat keymap to a per-level map, keeping only populated slots.
fn to_levels<T: FlatMapValue, V>(
    flat: &FlatMap<T>,
    project: impl Fn(T) -> Option<V>,
) -> BTreeMap<u8, BTreeMap<u32, V>> {
    let mut levels = BTreeMap::new();
    for level in 0..MAX_LEVELS {
        let mut keys = BTreeMap::new();
        for keycode in 0..flat.num_keys as u32 {
            if let Some(value) = project(flat.get(level, keycode)) {
                keys.insert(keycode, value);
            }
        }
        if !keys.is_empty() {
            levels.insert(level as u8, keys);
        }
    }
    levels
}

fn char_section(flat: &FlatKeymap) -> CharSection {
    to_levels(flat, |value| value)
}

fn named_section(flat: &FlatNamedKeyMap) -> NamedSection {
    to_levels(flat, |key| (key != NamedKey::Unnamed).then_some(key))
}

fn modifiers_from_layout(modifiers: &Modifiers) -> ModifierList {
    let mut out: Vec<_> = modifiers
        .iter()
        .map(|(keycode, modifier)| {
            (
                *keycode,
                modifier_name(*keycode, modifier),
                actions_from_modifier(modifier),
            )
        })
        .collect();
    out.sort_by_key(|(keycode, _, _)| *keycode);
    out
}

fn actions_from_modifier(modifier: &Modifier) -> Vec<(u8, ModAction)> {
    match modifier {
        Modifier::Single(kind) => vec![(0, modaction_from_modkind(kind))],
        Modifier::Leveled(map) => map
            .iter()
            .map(|(level, kind)| (*level, modaction_from_modkind(kind)))
            .collect(),
    }
}

fn modaction_from_modkind(kind: &ModKind) -> ModAction {
    match kind {
        ModKind::Pressed { mod_type, .. } => ModAction::Pressed(*mod_type),
        ModKind::Lock { mod_type, .. } => ModAction::Lock(*mod_type),
        ModKind::Latch { mod_type, .. } => ModAction::Latch(*mod_type),
        ModKind::None => ModAction::None,
    }
}

/// Best-effort human-readable name for a modifier binding. The name is
/// metadata only: it is ignored when loading.
fn modifier_name(keycode: u32, modifier: &Modifier) -> String {
    let fallback = |mod_type: Option<ModType>| match mod_type {
        Some(ModType::Level2) => "Shift",
        Some(ModType::Level3) => "Level3",
        Some(ModType::Level5) => "Level5",
        Some(ModType::Caps) => "CapsLock",
        Some(ModType::Num) => "NumLock",
        Some(ModType::Scroll) => "ScrollLock",
        Some(ModType::Compose) => "Compose",
        _ => "Modifier",
    };
    match keycode {
        29 => "LeftControl",
        42 => "LeftShift",
        54 => "RightShift",
        56 => "Alt",
        58 if matches!(modifier, Modifier::Leveled(_)) => "Eisu_toggle",
        58 => "CapsLock",
        69 => "NumLock",
        70 => "ScrollLock",
        97 => "RightControl",
        100 => match modifier {
            Modifier::Single(ModKind::Latch { .. }) => "AltGrLatch",
            Modifier::Single(ModKind::Lock { .. }) => "AltGrLock",
            _ => "AltGr",
        },
        125 => "Super",
        _ => fallback(modkind_type(match modifier {
            Modifier::Single(kind) => kind,
            Modifier::Leveled(map) => map.values().next().unwrap_or(&ModKind::None),
        })),
    }
    .to_string()
}

fn modkind_type(kind: &ModKind) -> Option<ModType> {
    match kind {
        ModKind::Pressed { mod_type, .. }
        | ModKind::Lock { mod_type, .. }
        | ModKind::Latch { mod_type, .. } => Some(*mod_type),
        ModKind::None => None,
    }
}

/// Depth-first walk of the composer trie emitting reachable sequences in
/// canonical (sorted) order.
fn compose_from_composer(composer: &Composer, reachable: &[char]) -> Vec<(Vec<char>, char)> {
    let mut out = Vec::new();
    let mut path = Vec::new();
    dfs_compose(composer, 0, &mut path, &mut out, reachable);
    out.sort();
    out
}

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
        if key == 0 {
            path.push(COMPOSE_KEY_CHAR);
        } else {
            path.push(char::from_u32(key).unwrap_or('\u{fffd}'));
        }
        dfs_compose(composer, child, path, out, reachable);
        path.pop();
    }
}

// ---------------------------------------------------------------------------
// LayoutFile -> KBLayout (loading)
// ---------------------------------------------------------------------------

impl TryFrom<LayoutFile> for KBLayout {
    type Error = IrError;

    fn try_from(file: LayoutFile) -> Result<Self, IrError> {
        file.validate()?;
        let num_keys = NUM_KEYS as usize;
        let name = file.layout;

        let mut repeat_keys = KeyBitSet::new();
        for keycode in &file.repeat_keys {
            repeat_keys.insert(*keycode);
        }

        let mut modifiers = Modifiers::new();
        for (keycode, _, actions) in &file.modifiers {
            let modifier = if actions.len() == 1 && actions[0].0 == 0 {
                Modifier::Single(modkind_from_modaction(actions[0].1))
            } else {
                let map: BTreeMap<u8, ModKind> = actions
                    .iter()
                    .map(|(level, action)| (*level, modkind_from_modaction(*action)))
                    .collect();
                Modifier::Leveled(map)
            };
            modifiers.set_modifier(*keycode, modifier);
        }

        let composer = composer_from_compose(&file.compose);

        let state_keymap = from_levels(&file.keymap, num_keys, Some);
        let num_lock_keys = from_levels(&file.num_lock_keys, num_keys, Some);
        let caps_lock_keymap = from_levels(&file.caps_lock_keymap, num_keys, Some);
        let named_key_map = from_levels(&file.keysym_map, num_keys, |key| key);

        Ok(KBLayout {
            name,
            repeat_keys,
            composer,
            modifiers,
            state_keymap,
            num_lock_keys,
            caps_lock_keymap,
            named_key_map,
            #[cfg(feature = "xkb")]
            level_exceptions_keymap: FlatKeymap::new(num_keys),
        })
    }
}

/// Un-flatten a per-level map back into a single `FlatMap`.
fn from_levels<T: FlatMapValue, V: Copy>(
    levels: &BTreeMap<u8, BTreeMap<u32, V>>,
    num_keys: usize,
    reconstruct: impl Fn(V) -> T,
) -> FlatMap<T> {
    let mut flat = FlatMap::new(num_keys);
    for (level, keys) in levels {
        let base = (*level as usize) * num_keys;
        for (keycode, value) in keys {
            flat.data[base + *keycode as usize] = reconstruct(*value);
        }
    }
    flat
}

fn modkind_from_modaction(action: ModAction) -> ModKind {
    match action {
        ModAction::Pressed(mod_type) => ModKind::Pressed {
            pressed: false,
            mod_type,
        },
        ModAction::Lock(mod_type) => ModKind::Lock {
            pressed: false,
            locked: 0,
            mod_type,
        },
        ModAction::Latch(mod_type) => ModKind::Latch {
            pressed: false,
            latched: false,
            mod_type,
        },
        ModAction::None => ModKind::None,
    }
}

/// Build a composer trie from serialized `(keys, output)` sequences.
fn composer_from_compose(sequences: &[(Vec<char>, char)]) -> Composer {
    let mut composer = Composer::new();
    for (keys, output) in sequences {
        let mut tokens: Vec<Token> = Vec::with_capacity(keys.len());
        for ch in keys {
            if *ch == COMPOSE_KEY_CHAR {
                tokens.push(Token::Compose);
            } else {
                tokens.push(Token::Char(*ch));
            }
        }
        composer.insert(&tokens, *output);
    }
    composer
}
