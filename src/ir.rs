//! Intermediate representation (IR) for wkb layout data files.
//!
//! This module defines the canonical on-disk format wkb uses to persist
//! pre-compiled keyboard layouts, plus conversions between this IR and the
//! runtime [`KBLayout`] type. Files are serialized as RON text.
//!
//! The IR is bidirectional:
//! - `LayoutFile::from_ron_str` / `LayoutFile::try_from(&KBLayout)` produce a
//!   `LayoutFile` that can be written with [`LayoutFile::to_ron_string`].
//! - `TryFrom<LayoutFile> for KBLayout` rebuilds a runtime layout from a file.
//!
//! See `docs/layout-format.md` for the normative specification.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::composer::{Composer, Token};
use crate::flat_keymap::MAX_LEVELS;
use crate::modifiers::{ModKind, ModType, Modifier, Modifiers};
use crate::named_keys::NamedKey;
use crate::{FlatKeymap, FlatNamedKeyMap, KBLayout, KeyBitSet};

/// Current version of the layout file schema. Files with a different version
/// are rejected by [`LayoutFile::validate`].
pub const FORMAT_VERSION: u32 = 1;

/// Character used to represent the Compose/Multi_key token inside a serialized
/// compose sequence. Reserved: a literal U+00B7 key cannot be represented.
pub const COMPOSE_KEY_CHAR: char = '\u{b7}';

/// A per-layout section mapping layout name -> level -> keycode -> character.
pub type CharSection = BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>;

/// A per-layout section mapping layout name -> level -> keycode -> named key.
pub type NamedSection = BTreeMap<String, BTreeMap<u8, BTreeMap<u32, NamedKey>>>;

/// Modifier bindings: `(keycode, name, [(level, action)])`.
pub type ModifierList = Vec<(u32, String, Vec<(u8, ModAction)>)>;

/// Errors from validating, serializing, or converting layout files.
#[derive(Debug, thiserror::Error)]
pub enum IrError {
    /// The file has an unsupported [`FORMAT_VERSION`].
    #[error("unsupported format version {0}")]
    UnsupportedVersion(u32),
    /// `layout_names` must contain at least one name.
    #[error("layout_names must not be empty")]
    EmptyLayoutNames,
    /// A layout name appears more than once in `layout_names`.
    #[error("duplicate layout name {0:?}")]
    DuplicateLayoutName(String),
    /// A requested layout index does not exist in the runtime instance.
    #[error("invalid layout index {0}")]
    InvalidLayoutIndex(usize),
    /// The format holds exactly one layout per file.
    #[error("expected exactly one layout, found {0}")]
    MultipleLayouts(usize),
    /// A section is keyed by a layout not declared in `layout_names`.
    #[error("layout {0:?} not declared in layout_names")]
    UndeclaredLayout(String),
    /// `num_keys` must be at least 1.
    #[error("invalid num_keys {0}")]
    InvalidNumKeys(u32),
    /// An evdev keycode is outside `0..num_keys`.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModAction {
    Pressed(ModType),
    Lock(ModType),
    Latch(ModType),
    None,
}

/// A persisted keyboard layout.
///
/// Maps are keyed by layout name (always a single entry per file), then by
/// level (`u8`, ascending), then by evdev keycode (`u32`, ascending). Using
/// `BTreeMap` guarantees canonical, deterministic ordering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "")]
pub struct LayoutFile {
    /// Schema version, must equal [`FORMAT_VERSION`].
    pub version: u32,
    /// Declared layout names, exactly one entry.
    pub layout_names: Vec<String>,
    /// Number of evdev keycode slots. All keycodes are `< num_keys`.
    pub num_keys: u32,
    /// Keycodes that repeat. The effective set is `add ∖ remove`.
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub repeat_keys_add: Vec<u32>,
    /// Keycodes excluded from `repeat_keys_add`.
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub repeat_keys_remove: Vec<u32>,
    /// Modifier bindings as `(keycode, name, [(level, action)])`, sorted by keycode.
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub modifiers: ModifierList,
    /// Resolved character per (level, keycode) under base modifiers.
    pub keymap: CharSection,
    /// Character overrides active while Num Lock is locked.
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub num_lock_keys: CharSection,
    /// Character overrides active while Caps Lock is locked.
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub caps_lock_keymap: CharSection,
    /// Raw keysym characters used as a fallback when level resolution fails.
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub level_exceptions_keymap: CharSection,
    /// Named-key identities per (level, keycode); `Unnamed` entries are omitted.
    #[serde(default, skip_serializing_if = "is_empty_map")]
    pub keysym_map: NamedSection,
    /// Compose sequences as `(keys, output)`. Only sequences whose keys are all
    /// reachable in this layout are stored.
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub compose: Vec<(Vec<char>, char)>,
}

fn is_empty_vec<T>(v: &[T]) -> bool {
    v.is_empty()
}

fn is_empty_map<K: Ord, V>(m: &BTreeMap<K, V>) -> bool {
    m.is_empty()
}

impl LayoutFile {
    /// Validate all structural invariants. Called automatically by
    /// [`LayoutFile::to_ron_string`], [`LayoutFile::from_ron_str`], and the
    /// conversions to/from [`KBLayout`].
    pub fn validate(&self) -> Result<(), IrError> {
        if self.version != FORMAT_VERSION {
            return Err(IrError::UnsupportedVersion(self.version));
        }
        if self.layout_names.is_empty() {
            return Err(IrError::EmptyLayoutNames);
        }
        for (i, name) in self.layout_names.iter().enumerate() {
            if self.layout_names[..i].contains(name) {
                return Err(IrError::DuplicateLayoutName(name.clone()));
            }
        }
        if self.layout_names.len() != 1 {
            return Err(IrError::MultipleLayouts(self.layout_names.len()));
        }
        let name = &self.layout_names[0];
        if self.num_keys == 0 {
            return Err(IrError::InvalidNumKeys(self.num_keys));
        }
        for keycode in self.repeat_keys_add.iter().chain(&self.repeat_keys_remove) {
            if *keycode >= self.num_keys {
                return Err(IrError::KeycodeOutOfRange(*keycode, self.num_keys));
            }
        }
        for (keycode, mod_name, actions) in &self.modifiers {
            if *keycode >= self.num_keys {
                return Err(IrError::KeycodeOutOfRange(*keycode, self.num_keys));
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
        for section in [
            &self.keymap,
            &self.num_lock_keys,
            &self.caps_lock_keymap,
            &self.level_exceptions_keymap,
        ] {
            validate_section(section, name, self.num_keys)?;
        }
        validate_section(&self.keysym_map, name, self.num_keys)?;
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
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| IrError::Serialize(e.to_string()))
    }

    /// Deserialize from RON text and validate.
    pub fn from_ron_str(s: &str) -> Result<Self, IrError> {
        let file: Self = ron::from_str(s).map_err(|e| IrError::Deserialize(e.to_string()))?;
        file.validate()?;
        Ok(file)
    }
}

fn validate_section<T>(
    section: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, T>>>,
    name: &str,
    num_keys: u32,
) -> Result<(), IrError> {
    for (layout, levels) in section {
        if layout != name {
            return Err(IrError::UndeclaredLayout(layout.clone()));
        }
        for (level, keys) in levels {
            if *level >= MAX_LEVELS as u8 {
                return Err(IrError::LevelOutOfRange(*level));
            }
            for keycode in keys.keys() {
                if *keycode >= num_keys {
                    return Err(IrError::KeycodeOutOfRange(*keycode, num_keys));
                }
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

        let mut repeat_keys_add = Vec::new();
        for keycode in 0..num_keys {
            if layout.repeat_keys.contains(keycode) {
                repeat_keys_add.push(keycode);
            }
        }

        let mut keymap = BTreeMap::new();
        keymap.insert(name.clone(), flat_to_levels(&layout.state_keymap, num_keys));
        let mut num_lock_keys = BTreeMap::new();
        num_lock_keys.insert(
            name.clone(),
            flat_to_levels(&layout.num_lock_keys, num_keys),
        );
        let mut caps_lock_keymap = BTreeMap::new();
        caps_lock_keymap.insert(
            name.clone(),
            flat_to_levels(&layout.caps_lock_keymap, num_keys),
        );
        #[cfg(feature = "xkb")]
        let mut level_exceptions_keymap = BTreeMap::new();
        #[cfg(feature = "xkb")]
        level_exceptions_keymap.insert(
            name.clone(),
            flat_to_levels(&layout.level_exceptions_keymap, num_keys),
        );
        #[cfg(not(feature = "xkb"))]
        let level_exceptions_keymap = BTreeMap::new();
        let mut keysym_map = BTreeMap::new();
        keysym_map.insert(
            name.clone(),
            named_to_levels(&layout.named_key_map, num_keys),
        );

        let reachable = reachable_chars(layout);
        let compose = compose_from_composer(&layout.composer, &reachable);

        let file = LayoutFile {
            version: FORMAT_VERSION,
            layout_names: vec![name],
            num_keys,
            repeat_keys_add,
            repeat_keys_remove: Vec::new(),
            modifiers: modifiers_from_layout(&layout.modifiers),
            keymap,
            num_lock_keys,
            caps_lock_keymap,
            level_exceptions_keymap,
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

fn flat_to_levels(flat: &FlatKeymap, num_keys: u32) -> BTreeMap<u8, BTreeMap<u32, char>> {
    let mut levels = BTreeMap::new();
    for level in 0..MAX_LEVELS {
        let mut keys = BTreeMap::new();
        for keycode in 0..num_keys {
            if let Some(ch) = flat.get(level, keycode) {
                keys.insert(keycode, ch);
            }
        }
        if !keys.is_empty() {
            levels.insert(level as u8, keys);
        }
    }
    levels
}

fn named_to_levels(flat: &FlatNamedKeyMap, num_keys: u32) -> BTreeMap<u8, BTreeMap<u32, NamedKey>> {
    let mut levels = BTreeMap::new();
    for level in 0..MAX_LEVELS {
        let mut keys = BTreeMap::new();
        for keycode in 0..num_keys {
            let named = flat.get(level, keycode);
            if named != NamedKey::Unnamed {
                keys.insert(keycode, named);
            }
        }
        if !keys.is_empty() {
            levels.insert(level as u8, keys);
        }
    }
    levels
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
    let mod_type = match modifier {
        Modifier::Single(kind) => modkind_type(kind),
        Modifier::Leveled(map) => map.values().next().and_then(modkind_type),
    };
    let name = match keycode {
        42 => "LeftShift",
        54 => "RightShift",
        29 => "LeftControl",
        97 => "RightControl",
        56 => "Alt",
        100 => match modifier {
            Modifier::Single(ModKind::Latch { .. }) => "AltGrLatch",
            Modifier::Single(ModKind::Lock { .. }) => "AltGrLock",
            _ => "AltGr",
        },
        58 => {
            if matches!(modifier, Modifier::Leveled(_)) {
                "Eisu_toggle"
            } else {
                "CapsLock"
            }
        }
        69 => "NumLock",
        70 => "ScrollLock",
        125 => "Super",
        _ => match mod_type {
            Some(ModType::Level2) => "Shift",
            Some(ModType::Level3) => "Level3",
            Some(ModType::Level5) => "Level5",
            Some(ModType::Caps) => "CapsLock",
            Some(ModType::Num) => "NumLock",
            Some(ModType::Scroll) => "ScrollLock",
            Some(ModType::Compose) => "Compose",
            Some(ModType::None) | None => "Modifier",
        },
    };
    name.to_string()
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
        let num_keys = file.num_keys as usize;
        let name = file.layout_names[0].clone();

        let mut repeat_keys = KeyBitSet::new();
        for keycode in &file.repeat_keys_add {
            repeat_keys.insert(*keycode);
        }
        for keycode in &file.repeat_keys_remove {
            repeat_keys.remove(*keycode);
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

        let state_keymap = levels_to_flat(file.keymap.get(&name), num_keys);
        let num_lock_keys = levels_to_flat(file.num_lock_keys.get(&name), num_keys);
        let caps_lock_keymap = levels_to_flat(file.caps_lock_keymap.get(&name), num_keys);
        #[cfg(feature = "xkb")]
        let level_exceptions_keymap =
            levels_to_flat(file.level_exceptions_keymap.get(&name), num_keys);
        let named_key_map = levels_to_named(file.keysym_map.get(&name), num_keys);

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
            level_exceptions_keymap,
        })
    }
}

fn levels_to_flat(
    levels: Option<&BTreeMap<u8, BTreeMap<u32, char>>>,
    num_keys: usize,
) -> FlatKeymap {
    let mut flat = FlatKeymap::new(num_keys);
    if let Some(levels) = levels {
        for (level, keys) in levels {
            let base = (*level as usize) * num_keys;
            for (keycode, ch) in keys {
                flat.data[base + *keycode as usize] = Some(*ch);
            }
        }
    }
    flat
}

fn levels_to_named(
    levels: Option<&BTreeMap<u8, BTreeMap<u32, NamedKey>>>,
    num_keys: usize,
) -> FlatNamedKeyMap {
    let mut flat = FlatNamedKeyMap::new(num_keys);
    if let Some(levels) = levels {
        for (level, keys) in levels {
            let base = (*level as usize) * num_keys;
            for (keycode, named) in keys {
                flat.data[base + *keycode as usize] = *named;
            }
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
