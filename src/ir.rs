//! Intermediate representation (IR) for wkb layout data files.
//!
//! [`LayoutFile`] is the canonical on-disk (KDL) form of a [`KBLayout`]. It is
//! bidirectional: [`LayoutFile::from_kdl_str`] / `TryFrom<&KBLayout>` produce a
//! serializable file, and `TryFrom<LayoutFile>` rebuilds the runtime layout.
//! See `docs/layout-format.md` for the normative specification.

use std::collections::BTreeMap;

use kdl::KdlDocument;

use crate::composer::{Composer, Token};
use crate::flat_keymap::{FlatMap, FlatMapValue, MAX_LEVELS};
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

/// One parsed `modifier` node: `(keycode, name, [(level, action)])`.
type ParsedModifier = (u32, String, Vec<(u8, ModAction)>);

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
    /// KDL serialization failed.
    #[error("serialization error: {0}")]
    Serialize(String),
    /// KDL deserialization failed.
    #[error("deserialization error: {0}")]
    Deserialize(String),
}

/// One modifier action, mirroring the runtime [`ModKind`] in a serializable
/// form. The `ModType` argument follows the surrounding XKB convention, e.g.
/// `Pressed(Level2)`, `Lock(Caps)`, `Lock(Num)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutFile {
    /// Schema version, must equal [`FORMAT_VERSION`].
    pub version: u32,
    /// Declared layout names, exactly one entry.
    pub layout_names: Vec<String>,
    /// Number of evdev keycode slots. All keycodes are `< num_keys`.
    pub num_keys: u32,
    /// Keycodes that repeat.
    pub repeat_keys: Vec<u32>,
    /// Modifier bindings as `(keycode, name, [(level, action)])`, sorted by keycode.
    pub modifiers: ModifierList,
    /// Resolved character per (level, keycode) under base modifiers.
    pub keymap: CharSection,
    /// Character overrides active while Num Lock is locked.
    pub num_lock_keys: CharSection,
    /// Character overrides active while Caps Lock is locked.
    pub caps_lock_keymap: CharSection,
    /// Named-key identities per (level, keycode); `Unnamed` entries are omitted.
    pub keysym_map: NamedSection,
    /// Compose sequences as `(keys, output)`. Only sequences whose keys are all
    /// reachable in this layout are stored.
    pub compose: Vec<(Vec<char>, char)>,
}

impl LayoutFile {
    /// Validate all structural invariants. Called automatically by
    /// [`LayoutFile::to_kdl_string`], [`LayoutFile::from_kdl_str`], and the
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
        for keycode in &self.repeat_keys {
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
        for section in [&self.keymap, &self.num_lock_keys, &self.caps_lock_keymap] {
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

    /// Serialize to canonical KDL text. Fails on invalid input.
    pub fn to_kdl_string(&self) -> Result<String, IrError> {
        self.validate()?;
        Ok(serialize_to_kdl(self))
    }

    /// Deserialize from KDL text and validate.
    pub fn from_kdl_str(s: &str) -> Result<Self, IrError> {
        let doc: KdlDocument = s
            .parse()
            .map_err(|e: kdl::KdlError| IrError::Deserialize(e.to_string()))?;
        let file = parse_kdl_document(&doc)?;
        file.validate()?;
        Ok(file)
    }
}

// ---------------------------------------------------------------------------
// KDL serialization
// ---------------------------------------------------------------------------

/// Wrap a sequence of integer arguments across lines using KDL line
/// continuation, keeping each line short enough to stay readable.
const KDL_WRAP_WIDTH: usize = 20;

fn serialize_to_kdl(file: &LayoutFile) -> String {
    let mut blocks: Vec<String> = Vec::new();

    let mut header = format!("version {}\n", file.version);
    push_string_node(&mut header, "layout", &file.layout_names[0]);
    header.push_str(&format!("num_keys {}", file.num_keys));
    blocks.push(header);

    if !file.repeat_keys.is_empty() {
        blocks.push(integer_node_block("repeat_keys", &file.repeat_keys));
    }
    if !file.modifiers.is_empty() {
        blocks.push(modifier_block(&file.modifiers));
    }
    let name = &file.layout_names[0];
    if section_has_content(&file.keymap, name) {
        blocks.push(char_section_block("keymap", &file.keymap, name));
    }
    if section_has_content(&file.num_lock_keys, name) {
        blocks.push(char_section_block(
            "num_lock_keys",
            &file.num_lock_keys,
            name,
        ));
    }
    if section_has_content(&file.caps_lock_keymap, name) {
        blocks.push(char_section_block(
            "caps_lock_keymap",
            &file.caps_lock_keymap,
            name,
        ));
    }
    if section_has_content(&file.keysym_map, name) {
        blocks.push(named_section_block("keysym_map", &file.keysym_map, name));
    }
    if !file.compose.is_empty() {
        blocks.push(compose_block(&file.compose));
    }

    let mut out = String::from("// wkb keyboard layout (KDL format)\n");
    out.push_str(&blocks.join("\n\n"));
    out.push('\n');
    out
}

fn push_string_node(out: &mut String, node: &str, value: &str) {
    out.push_str(node);
    out.push(' ');
    write_kdl_string(out, value);
    out.push('\n');
}

/// Write `s` as a KDL quoted string, escaping any character that would be
/// ambiguous or invalid inside quotes.
fn write_kdl_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' | '"' => {
                out.push('\\');
                out.push(c);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if c.is_control() || is_kdl_disallowed(c) => {
                use std::fmt::Write as _;
                write!(out, "\\u{{{:x}}}", c as u32).unwrap();
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Code points KDL forbids in quoted strings unless escaped. Mirrors the
/// `kdl` crate's `is_disallowed_unicode` (control chars, Unicode bidi
/// direction controls, and the byte-order mark).
fn is_kdl_disallowed(c: char) -> bool {
    matches!(
        c,
        '\u{0000}'..='\u{0008}'
            | '\u{000e}'..='\u{001f}'
            | '\u{007f}'..='\u{009f}'
            | '\u{200e}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}'
            | '\u{feff}'
    )
}

fn integer_node_block(node: &str, values: &[u32]) -> String {
    let mut out = String::new();
    out.push_str(node);
    for (i, value) in values.iter().enumerate() {
        if i > 0 && i % KDL_WRAP_WIDTH == 0 {
            out.push_str(" \\\n    ");
        }
        out.push(' ');
        out.push_str(&value.to_string());
    }
    out
}

fn modifier_block(modifiers: &ModifierList) -> String {
    let mut out = String::new();
    for (keycode, name, actions) in modifiers {
        out.push_str("modifier ");
        out.push_str(&keycode.to_string());
        out.push(' ');
        write_kdl_string(&mut out, name);
        for (level, action) in actions {
            out.push(' ');
            out.push_str(&level.to_string());
            out.push(' ');
            write_kdl_string(&mut out, &mod_action_str(*action));
        }
        out.push('\n');
    }
    out.pop();
    out
}

fn mod_action_str(action: ModAction) -> String {
    match action {
        ModAction::Pressed(t) => action_str("Pressed", t),
        ModAction::Lock(t) => action_str("Lock", t),
        ModAction::Latch(t) => action_str("Latch", t),
        ModAction::None => "None".to_string(),
    }
}

fn action_str(variant: &str, mod_type: ModType) -> String {
    if mod_type == ModType::None {
        variant.to_string()
    } else {
        format!("{variant}({mod_type:?})")
    }
}

fn char_section_block(node_name: &str, section: &CharSection, name: &str) -> String {
    section_block(node_name, section, name, |out, value| {
        write_kdl_string(out, &value.to_string());
    })
}

fn named_section_block(node_name: &str, section: &NamedSection, name: &str) -> String {
    section_block(node_name, section, name, |out, value| {
        write_kdl_string(out, &format!("{value:?}"));
    })
}

/// True if `section` has any keyed level data under `name`.
fn section_has_content<T>(
    section: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, T>>>,
    name: &str,
) -> bool {
    section.get(name).is_some_and(|levels| !levels.is_empty())
}

fn section_block<T>(
    node_name: &str,
    section: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, T>>>,
    name: &str,
    write_value: impl Fn(&mut String, &T),
) -> String {
    let mut out = String::new();
    out.push_str(node_name);
    out.push_str(" {\n");
    for (level, keys) in &section[name] {
        out.push_str("    level ");
        out.push_str(&level.to_string());
        for (keycode, value) in keys {
            out.push_str(" \"");
            out.push_str(&keycode.to_string());
            out.push_str("\"=");
            write_value(&mut out, value);
        }
        out.push('\n');
    }
    out.push('}');
    out
}

fn compose_block(compose: &[(Vec<char>, char)]) -> String {
    let mut out = String::new();
    for (keys, output) in compose {
        out.push_str("compose");
        for ch in keys {
            out.push(' ');
            write_kdl_string(&mut out, &ch.to_string());
        }
        out.push(' ');
        write_kdl_string(&mut out, &output.to_string());
        out.push('\n');
    }
    out.pop();
    out
}

// ---------------------------------------------------------------------------
// KDL parsing
// ---------------------------------------------------------------------------

fn kdl_err(msg: impl Into<String>) -> IrError {
    IrError::Deserialize(msg.into())
}

fn kdl_u32(node: &kdl::KdlNode, node_name: &str) -> Result<u32, IrError> {
    node.entries()
        .first()
        .and_then(|entry| entry.value().as_integer())
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| kdl_err(format!("{node_name:?} requires a u32 argument")))
}

fn kdl_string<'a>(node: &'a kdl::KdlNode, node_name: &str) -> Result<&'a str, IrError> {
    node.entries()
        .first()
        .and_then(|entry| entry.value().as_string())
        .ok_or_else(|| kdl_err(format!("{node_name:?} requires a string argument")))
}

fn kdl_u32_list(node: &kdl::KdlNode, node_name: &str) -> Result<Vec<u32>, IrError> {
    let mut out = Vec::new();
    for entry in node.entries() {
        let value = entry
            .value()
            .as_integer()
            .and_then(|value| u32::try_from(value).ok())
            .ok_or_else(|| kdl_err(format!("{node_name:?} requires u32 arguments")))?;
        out.push(value);
    }
    Ok(out)
}

fn parse_kdl_modifier(node: &kdl::KdlNode) -> Result<ParsedModifier, IrError> {
    let entries = node.entries();
    if entries.len() < 4 || (entries.len() - 2) % 2 != 0 {
        return Err(kdl_err(
            "modifier expects keycode, name, then level/action pairs",
        ));
    }
    let keycode = entries[0]
        .value()
        .as_integer()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| kdl_err("modifier keycode must be a u32"))?;
    let name = entries[1]
        .value()
        .as_string()
        .ok_or_else(|| kdl_err("modifier name must be a string"))?
        .to_string();

    let mut actions = Vec::with_capacity((entries.len() - 2) / 2);
    let mut i = 2;
    while i < entries.len() {
        let level = entries[i]
            .value()
            .as_integer()
            .and_then(|value| u8::try_from(value).ok())
            .ok_or_else(|| kdl_err("modifier level must be a u8"))?;
        let action = entries[i + 1]
            .value()
            .as_string()
            .and_then(parse_kdl_mod_action)
            .ok_or_else(|| kdl_err("modifier action must be a valid action string"))?;
        actions.push((level, action));
        i += 2;
    }
    Ok((keycode, name, actions))
}

fn parse_kdl_mod_action(s: &str) -> Option<ModAction> {
    let (variant, mod_type) = match s.split_once('(') {
        Some((variant, rest)) => {
            let mod_type = parse_kdl_mod_type(rest.strip_suffix(')')?)?;
            (variant, Some(mod_type))
        }
        None => (s, None),
    };
    match variant {
        "Pressed" => Some(ModAction::Pressed(mod_type.unwrap_or(ModType::None))),
        "Lock" => Some(ModAction::Lock(mod_type.unwrap_or(ModType::None))),
        "Latch" => Some(ModAction::Latch(mod_type.unwrap_or(ModType::None))),
        "None" => Some(ModAction::None),
        _ => None,
    }
}

fn parse_kdl_mod_type(s: &str) -> Option<ModType> {
    match s {
        "None" => Some(ModType::None),
        "Level2" => Some(ModType::Level2),
        "Level3" => Some(ModType::Level3),
        "Level5" => Some(ModType::Level5),
        "Compose" => Some(ModType::Compose),
        "Caps" => Some(ModType::Caps),
        "Num" => Some(ModType::Num),
        "Scroll" => Some(ModType::Scroll),
        _ => None,
    }
}

fn parse_kdl_document(doc: &KdlDocument) -> Result<LayoutFile, IrError> {
    let mut version = None;
    let mut layout_names = Vec::new();
    let mut num_keys = None;
    let mut repeat_keys = Vec::new();
    let mut modifiers = Vec::new();
    let mut keymap: Option<BTreeMap<u8, BTreeMap<u32, char>>> = None;
    let mut num_lock_keys: Option<BTreeMap<u8, BTreeMap<u32, char>>> = None;
    let mut caps_lock_keymap: Option<BTreeMap<u8, BTreeMap<u32, char>>> = None;
    let mut keysym_map: Option<BTreeMap<u8, BTreeMap<u32, NamedKey>>> = None;
    let mut compose = Vec::new();

    for node in doc.nodes() {
        match node.name().value() {
            "version" => version = Some(kdl_u32(node, "version")?),
            "layout" => layout_names.push(kdl_string(node, "layout")?.to_string()),
            "num_keys" => num_keys = Some(kdl_u32(node, "num_keys")?),
            "repeat_keys" => repeat_keys = kdl_u32_list(node, "repeat_keys")?,
            "modifier" => modifiers.push(parse_kdl_modifier(node)?),
            "keymap" => keymap = Some(parse_kdl_levels(node, "keymap", parse_kdl_char)?),
            "num_lock_keys" => {
                num_lock_keys = Some(parse_kdl_levels(node, "num_lock_keys", parse_kdl_char)?)
            }
            "caps_lock_keymap" => {
                caps_lock_keymap = Some(parse_kdl_levels(node, "caps_lock_keymap", parse_kdl_char)?)
            }
            "keysym_map" => {
                keysym_map = Some(parse_kdl_levels(node, "keysym_map", parse_kdl_named_key)?)
            }
            "compose" => compose.push(parse_kdl_compose(node)?),
            other => return Err(kdl_err(format!("unknown node {other:?}"))),
        }
    }

    let name = layout_names
        .first()
        .ok_or_else(|| kdl_err("missing node \"layout\""))?;
    let keymap = keymap
        .filter(|levels| !levels.is_empty())
        .map(|levels| CharSection::from([(name.clone(), levels)]))
        .unwrap_or_default();
    let num_lock_keys = num_lock_keys
        .filter(|levels| !levels.is_empty())
        .map(|levels| CharSection::from([(name.clone(), levels)]))
        .unwrap_or_default();
    let caps_lock_keymap = caps_lock_keymap
        .filter(|levels| !levels.is_empty())
        .map(|levels| CharSection::from([(name.clone(), levels)]))
        .unwrap_or_default();
    let keysym_map = keysym_map
        .filter(|levels| !levels.is_empty())
        .map(|levels| NamedSection::from([(name.clone(), levels)]))
        .unwrap_or_default();

    Ok(LayoutFile {
        version: version.ok_or_else(|| kdl_err("missing node \"version\""))?,
        layout_names,
        num_keys: num_keys.ok_or_else(|| kdl_err("missing node \"num_keys\""))?,
        repeat_keys,
        modifiers,
        keymap,
        num_lock_keys,
        caps_lock_keymap,
        keysym_map,
        compose,
    })
}

fn parse_kdl_levels<T>(
    node: &kdl::KdlNode,
    node_name: &str,
    parse_value: impl Fn(&kdl::KdlEntry) -> Result<T, IrError>,
) -> Result<BTreeMap<u8, BTreeMap<u32, T>>, IrError> {
    let children = node
        .children()
        .ok_or_else(|| kdl_err(format!("{node_name:?} requires a children block")))?;
    let mut levels = BTreeMap::new();
    for child in children.nodes() {
        if child.name().value() != "level" {
            return Err(kdl_err(format!(
                "{node_name:?} children must be level nodes"
            )));
        }
        let entries = child.entries();
        let level = entries
            .first()
            .filter(|entry| entry.name().is_none())
            .and_then(|entry| entry.value().as_integer())
            .and_then(|value| u8::try_from(value).ok())
            .ok_or_else(|| kdl_err(format!("{node_name:?} level node requires a level u8")))?;
        let mut keys = BTreeMap::new();
        for entry in &entries[1..] {
            let keycode = entry
                .name()
                .and_then(|name| name.value().parse::<u32>().ok())
                .ok_or_else(|| {
                    kdl_err(format!("{node_name:?} level expects keycode properties"))
                })?;
            let value = parse_value(entry)?;
            keys.insert(keycode, value);
        }
        levels.insert(level, keys);
    }
    Ok(levels)
}

fn parse_kdl_char(entry: &kdl::KdlEntry) -> Result<char, IrError> {
    let s = entry
        .value()
        .as_string()
        .ok_or_else(|| kdl_err("level value must be a single-character string"))?;
    let mut chars = s.chars();
    let ch = chars
        .next()
        .ok_or_else(|| kdl_err("level value must be a single char"))?;
    if chars.next().is_some() {
        return Err(kdl_err("level value must be a single char"));
    }
    Ok(ch)
}

fn parse_kdl_named_key(entry: &kdl::KdlEntry) -> Result<NamedKey, IrError> {
    let s = entry
        .value()
        .as_string()
        .ok_or_else(|| kdl_err("level value must be a named-key string"))?;
    named_key_from_str(s).ok_or_else(|| kdl_err(format!("unknown named key {s:?}")))
}

fn parse_kdl_compose(node: &kdl::KdlNode) -> Result<(Vec<char>, char), IrError> {
    let mut args = Vec::new();
    for entry in node.entries() {
        let s = entry
            .value()
            .as_string()
            .ok_or_else(|| kdl_err("compose requires string arguments"))?;
        let mut chars = s.chars();
        let ch = chars
            .next()
            .ok_or_else(|| kdl_err("compose argument must be a single char"))?;
        if chars.next().is_some() {
            return Err(kdl_err("compose argument must be a single char"));
        }
        args.push(ch);
    }
    if args.len() < 2 {
        return Err(kdl_err("compose requires at least one key and an output"));
    }
    let output = args.pop().expect("args has at least two entries");
    Ok((args, output))
}

/// Parse a `NamedKey` from its canonical name (the serde variant name, e.g.
/// `Escape`, `ArrowUp`).
fn named_key_from_str(s: &str) -> Option<NamedKey> {
    use serde::de::Deserialize as _;
    NamedKey::deserialize(serde::de::value::StrDeserializer::<serde::de::value::Error>::new(s)).ok()
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

        let mut repeat_keys = Vec::new();
        for keycode in 0..num_keys {
            if layout.repeat_keys.contains(keycode) {
                repeat_keys.push(keycode);
            }
        }

        let keymap = char_section(&name, &layout.state_keymap);
        let num_lock_keys = char_section(&name, &layout.num_lock_keys);
        let caps_lock_keymap = char_section(&name, &layout.caps_lock_keymap);
        let keysym_map = named_section(&name, &layout.named_key_map);

        let reachable = reachable_chars(layout);
        let compose = compose_from_composer(&layout.composer, &reachable);

        let file = LayoutFile {
            version: FORMAT_VERSION,
            layout_names: vec![name],
            num_keys,
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

fn char_section(name: &str, flat: &FlatKeymap) -> CharSection {
    BTreeMap::from([(name.to_string(), to_levels(flat, |value| value))])
}

fn named_section(name: &str, flat: &FlatNamedKeyMap) -> NamedSection {
    BTreeMap::from([(
        name.to_string(),
        to_levels(flat, |key| (key != NamedKey::Unnamed).then_some(key)),
    )])
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
        let num_keys = file.num_keys as usize;
        let name = file.layout_names[0].clone();

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

        let state_keymap = from_levels(file.keymap.get(&name), num_keys, Some);
        let num_lock_keys = from_levels(file.num_lock_keys.get(&name), num_keys, Some);
        let caps_lock_keymap = from_levels(file.caps_lock_keymap.get(&name), num_keys, Some);
        let named_key_map = from_levels(file.keysym_map.get(&name), num_keys, |key| key);

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
    levels: Option<&BTreeMap<u8, BTreeMap<u32, V>>>,
    num_keys: usize,
    reconstruct: impl Fn(V) -> T,
) -> FlatMap<T> {
    let mut flat = FlatMap::new(num_keys);
    if let Some(levels) = levels {
        for (level, keys) in levels {
            let base = (*level as usize) * num_keys;
            for (keycode, value) in keys {
                flat.data[base + *keycode as usize] = reconstruct(*value);
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
