//! Intermediate representation (IR) for wkb layout data files.
//!
//! [`LayoutFile`] is the canonical on-disk (RON) form of a [`KBLayout`]. It is
//! bidirectional: [`LayoutFile::from_ron_str`] / `TryFrom<&KBLayout>` produce a
//! serializable file, and `TryFrom<LayoutFile>` rebuilds the runtime layout.
//! See `docs/layout-format.md` for the normative specification.
//!
//! The IR mirrors the serialized RON document one-to-one: `version`, a single
//! `layout` name, `repeat_keys`, `modifiers`, per-level section maps
//! (`keymap`, `num_lock_keys`, `caps_lock_keymap`, `caps_num_lock_keys`,
//! `keysym_map`), and a `compose` table.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::composer::{Composer, Token};
use crate::flat_keymap::{FlatMap, FlatMapValue, MAX_LEVELS};
use crate::modifiers::{KeyEffect, ModKind, ModType, Modifier, Modifiers};
use crate::named_keys::NamedKey;
use crate::{FlatKeymap, FlatNamedKeyMap, KBLayout, KeyBitSet};

pub use crate::modifiers::{LatchVariant, LockFlags};

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

/// A section: level -> keycode -> character (used by `keymap`,
/// `num_lock_keys`, `caps_lock_keymap`).
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
    Lock(
        ModType,
        #[serde(default, skip_serializing_if = "LockFlags::is_empty")] LockFlags,
    ),
    Latch(
        ModType,
        #[serde(default, skip_serializing_if = "LatchVariant::is_on_release")] LatchVariant,
    ),
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
    /// Modifier bindings as `(keycode, [(level, action)])`, sorted by keycode.
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
    /// Character overrides active while both Num Lock and Caps Lock are locked.
    #[serde(default)]
    pub caps_num_lock_keys: CharSection,
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
            &self.caps_num_lock_keys,
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
    let _ = writeln!(out, "// wkb keyboard layout (RON format)");
    let _ = writeln!(out, "(");
    let _ = writeln!(out, "    version: {},", file.version);
    let _ = writeln!(out, "    layout: {},", ron_value(&file.layout));
    if !file.repeat_keys.is_empty() {
        write_integer_list(&mut out, "repeat_keys", &file.repeat_keys);
    }
    if !file.modifiers.is_empty() {
        write_modifier_entries(&mut out, &file.modifiers);
    }
    write_char_section(&mut out, "keymap", &file.keymap);
    write_char_section(&mut out, "num_lock_keys", &file.num_lock_keys);
    write_char_section(&mut out, "caps_lock_keymap", &file.caps_lock_keymap);
    write_char_section(&mut out, "caps_num_lock_keys", &file.caps_num_lock_keys);
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

fn write_modifier_entries(out: &mut String, entries: &ModifierList) {
    let _ = writeln!(out, "    modifiers: [");
    for entry in entries {
        let value = ron_value(entry).replace("r#None", "None");
        let _ = writeln!(out, "        {value},");
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
                let newline = keys_per_line.map_or(true, |n| {
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
        let file = LayoutFile {
            version: FORMAT_VERSION,
            layout: layout.name.clone(),
            repeat_keys: (0..num_keys)
                .filter(|&k| layout.repeat_keys.contains(k))
                .collect(),
            modifiers: modifiers_from_layout(&layout.modifiers),
            keymap: char_section(&layout.state_keymap),
            num_lock_keys: plain_char_section(&layout.num_lock_keys),
            caps_lock_keymap: plain_char_section(&layout.caps_lock_keymap),
            caps_num_lock_keys: plain_char_section(&layout.caps_num_lock_keys),
            keysym_map: named_section(&layout.named_key_map),
            compose: compose_from_composer(&layout.composer, &reachable_chars(layout)),
        };
        file.validate()?;
        Ok(file)
    }
}

/// Characters this layout can produce, for filtering the compose table.
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

/// Level with its most significant set bit cleared (1 → 0, 2 → 0, 3 → 1,
/// 4 → 0, 5 → 1, 6 → 2, 7 → 3). A key's level index is accumulated from three
/// independent modifier bits (Level5, Level3, Level2), so `drop_top(level)` is
/// the parent plane the key resolves to when the extra bit is unconsumed:
/// AltGr+Shift (level 3) falls back to Shift (level 1), Level5 (level 4) to
/// the base plane, and so on.
fn drop_top(level: u8) -> u8 {
    debug_assert!(level < MAX_LEVELS as u8);
    if level == 0 {
        return 0;
    }
    let bit = 1u8 << (7 - level.leading_zeros());
    level & !bit
}

/// Number of leading level planes in `flat` that carry distinct output.
///
/// The three modifier bits make higher planes resolve to the same characters
/// as lower ones when a key ignores that bit, so trailing planes that repeat a
/// prefix of the sequence are redundant and omitted from the serialized file.
fn effective_levels<T: FlatMapValue + PartialEq>(flat: &FlatMap<T>) -> usize {
    let mut n = MAX_LEVELS;
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

fn planes_equal<T: FlatMapValue + PartialEq>(
    flat: &FlatMap<T>,
    a: usize,
    b: usize,
    count: usize,
) -> bool {
    let nk = flat.num_keys;
    let len = count * nk;
    flat.data[a * nk..a * nk + len] == flat.data[b * nk..b * nk + len]
}

/// Invariant named keys by (level, evdev keycode). The serializer omits any
/// `keysym_map` entry equal to this default and the loader re-seeds it before
/// applying the file's entries, so an explicitly `Unnamed` entry clears one.
/// Only genuinely layout-varying slots (Caps Lock, Num Lock, Right Control,
/// ...) end up stored in files.
const LEVEL0_DEFAULTS: &[(u32, NamedKey)] = &[
    (1, NamedKey::Escape),
    (14, NamedKey::Backspace),
    (15, NamedKey::Tab),
    (28, NamedKey::Enter),
    (29, NamedKey::LeftControl),
    (42, NamedKey::LeftShift),
    (54, NamedKey::RightShift),
    (56, NamedKey::LeftAlt),
    (57, NamedKey::Space),
    (59, NamedKey::F1),
    (60, NamedKey::F2),
    (61, NamedKey::F3),
    (62, NamedKey::F4),
    (63, NamedKey::F5),
    (64, NamedKey::F6),
    (65, NamedKey::F7),
    (66, NamedKey::F8),
    (67, NamedKey::F9),
    (68, NamedKey::F10),
    (70, NamedKey::ScrollLock),
    (71, NamedKey::Home),
    (72, NamedKey::ArrowUp),
    (73, NamedKey::PageUp),
    (75, NamedKey::ArrowLeft),
    (77, NamedKey::ArrowRight),
    (79, NamedKey::End),
    (80, NamedKey::ArrowDown),
    (81, NamedKey::PageDown),
    (82, NamedKey::Insert),
    (83, NamedKey::Delete),
    (87, NamedKey::F11),
    (88, NamedKey::F12),
    (90, NamedKey::Katakana),
    (91, NamedKey::Hiragana),
    (96, NamedKey::Enter),
    (99, NamedKey::PrintScreen),
    (102, NamedKey::Home),
    (103, NamedKey::ArrowUp),
    (104, NamedKey::PageUp),
    (105, NamedKey::ArrowLeft),
    (106, NamedKey::ArrowRight),
    (107, NamedKey::End),
    (108, NamedKey::ArrowDown),
    (109, NamedKey::PageDown),
    (110, NamedKey::Insert),
    (111, NamedKey::Delete),
    (113, NamedKey::VolumeMute),
    (114, NamedKey::VolumeDown),
    (115, NamedKey::VolumeUp),
    (116, NamedKey::PowerOff),
    (119, NamedKey::Pause),
    (123, NamedKey::HangulHanja),
    (125, NamedKey::LeftSuper),
    (126, NamedKey::RightSuper),
    (127, NamedKey::ContextMenu),
    (140, NamedKey::LaunchCalculator),
    (142, NamedKey::Sleep),
    (143, NamedKey::WakeUp),
    (155, NamedKey::LaunchMail),
    (158, NamedKey::BrowserBack),
    (159, NamedKey::BrowserForward),
    (163, NamedKey::MediaNextTrack),
    (164, NamedKey::MediaPlay),
    (165, NamedKey::MediaPreviousTrack),
    (166, NamedKey::MediaStop),
    (172, NamedKey::BrowserHome),
    (200, NamedKey::MediaPlay),
    (201, NamedKey::MediaPause),
    (205, NamedKey::Suspend),
    (207, NamedKey::MediaPlay),
    (210, NamedKey::PrintScreen),
    (215, NamedKey::LaunchMail),
    (224, NamedKey::BrightnessDown),
    (225, NamedKey::BrightnessUp),
    (229, NamedKey::KeyboardBrightnessDown),
    (230, NamedKey::KeyboardBrightnessUp),
];

/// Invariant level-1 named keys (Shift plane): navigation/function keys and
/// the system modifier aliases XKB emits on the second level.
const LEVEL1_DEFAULTS: &[(u32, NamedKey)] = &[
    (14, NamedKey::Backspace),
    (15, NamedKey::Tab),
    (59, NamedKey::F1),
    (60, NamedKey::F2),
    (61, NamedKey::F3),
    (62, NamedKey::F4),
    (63, NamedKey::F5),
    (64, NamedKey::F6),
    (65, NamedKey::F7),
    (66, NamedKey::F8),
    (67, NamedKey::F9),
    (68, NamedKey::F10),
    (87, NamedKey::F11),
    (88, NamedKey::F12),
    (99, NamedKey::SysReq),
    (164, NamedKey::MediaPause),
    (196, NamedKey::LeftAlt),
    (197, NamedKey::LeftMeta),
    (198, NamedKey::LeftSuper),
    (199, NamedKey::LeftHyper),
];

/// Invariant F1-F12 row, repeated on the AltGr (level 2) and AltGr+Shift
/// (level 3) planes in every layout.
const FN_KEYS_DEFAULTS: &[(u32, NamedKey)] = &[
    (59, NamedKey::F1),
    (60, NamedKey::F2),
    (61, NamedKey::F3),
    (62, NamedKey::F4),
    (63, NamedKey::F5),
    (64, NamedKey::F6),
    (65, NamedKey::F7),
    (66, NamedKey::F8),
    (67, NamedKey::F9),
    (68, NamedKey::F10),
    (87, NamedKey::F11),
    (88, NamedKey::F12),
];

fn default_named_key(level: u8, keycode: u32) -> NamedKey {
    let table = match level {
        0 => LEVEL0_DEFAULTS,
        1 => LEVEL1_DEFAULTS,
        2 | 3 => FN_KEYS_DEFAULTS,
        _ => return NamedKey::Unnamed,
    };
    table
        .iter()
        .find(|(code, _)| *code == keycode)
        .map_or(NamedKey::Unnamed, |(_, key)| *key)
}

/// Seed the per-level named-key defaults into `flat`. The loader runs this
/// before applying the file's own entries, so an explicit `Unnamed` entry
/// stored in a file clears a default.
fn seed_named_defaults(flat: &mut FlatNamedKeyMap) {
    const LEVELS: &[(u8, &[(u32, NamedKey)])] = &[
        (0, LEVEL0_DEFAULTS),
        (1, LEVEL1_DEFAULTS),
        (2, FN_KEYS_DEFAULTS),
        (3, FN_KEYS_DEFAULTS),
    ];
    for &(level, table) in LEVELS {
        let start = level as usize * flat.num_keys;
        for (keycode, key) in table {
            flat.data[start + *keycode as usize] = *key;
        }
    }
}

/// Convert `num_levels` leading level planes of a flat map to a per-level
/// delta map. Level 0 keeps every populated slot; higher planes only keep
/// entries whose resolved output differs from their parent plane (see
/// [`drop_top`]). The loader re-derives the omitted entries, so the round-trip
/// stays exact while redundant AltGr/Level5 planes (identical to a lower plane
/// for most keys) disappear from the serialized file.
fn to_levels<T: FlatMapValue + PartialEq, V>(
    flat: &FlatMap<T>,
    project: impl Fn(T) -> Option<V>,
    base: impl Fn(u8, u32) -> T,
    num_levels: usize,
) -> BTreeMap<u8, BTreeMap<u32, V>> {
    (0..num_levels)
        .filter_map(|level| {
            let level = level as u8;
            let keys: BTreeMap<_, _> = (0..flat.num_keys)
                .filter_map(|keycode| {
                    let value = flat.data[level as usize * flat.num_keys + keycode];
                    (value != base(level, keycode as u32))
                        .then(|| project(value))
                        .flatten()
                        .map(|v| (keycode as u32, v))
                })
                .collect();
            (!keys.is_empty()).then_some((level, keys))
        })
        .collect()
}

/// Base character keymap: level 0 in full, higher planes delta-compressed
/// against the resolved parent plane.
fn char_section(flat: &FlatKeymap) -> CharSection {
    to_levels(
        flat,
        |value| value,
        |level, keycode| {
            if level > 0 {
                flat.get(drop_top(level) as usize, keycode)
            } else {
                None
            }
        },
        effective_levels(flat),
    )
}

/// Plain character section without delta compression: every populated slot at
/// every level is stored. Used for the Num Lock / Caps Lock override maps,
/// where an entry means "this level overrides the base keymap" and there is no
/// inheritance between planes.
fn plain_char_section(flat: &FlatKeymap) -> CharSection {
    to_levels(flat, |value| value, |_, _| None, effective_levels(flat))
}

/// Named-key section. Entries equal to the per-level [`default_named_key`]
/// value are omitted and re-seeded on load, so an explicit `Unnamed` entry
/// stored in a file clears a default. All levels are walked so a plane whose
/// resolved names differ from the defaults still round-trips exactly.
fn named_section(flat: &FlatNamedKeyMap) -> NamedSection {
    to_levels(
        flat,
        |key| Some(key),
        |level, keycode| default_named_key(level, keycode),
        MAX_LEVELS,
    )
}

fn modifiers_from_layout(modifiers: &Modifiers) -> ModifierList {
    let mut out: Vec<_> = modifiers
        .iter()
        .map(|(keycode, modifier)| (*keycode, actions_from_modifier(modifier)))
        .collect();
    out.sort_by_key(|(keycode, _)| *keycode);
    out
}

fn actions_from_modifier(modifier: &Modifier) -> Vec<(u8, ModAction)> {
    match modifier {
        Modifier::Single(effect) => vec![(0, modaction_from_effect(effect))],
        Modifier::Leveled(map) => map
            .iter()
            .map(|(level, effect)| (*level, modaction_from_effect(effect)))
            .collect(),
    }
}

fn modaction_from_effect(effect: &KeyEffect) -> ModAction {
    effect
        .modifier
        .as_ref()
        .map(|state| modaction_from_modkind(state.mod_type, &state.kind))
        .unwrap_or(ModAction::Press(ModType::None))
}

fn modaction_from_modkind(mod_type: ModType, kind: &ModKind) -> ModAction {
    match kind {
        ModKind::Press => ModAction::Press(mod_type),
        ModKind::Lock(variant) => ModAction::Lock(mod_type, *variant),
        ModKind::Latch(variant) => ModAction::Latch(mod_type, *variant),
    }
}

/// Depth-first walk of the composer trie emitting reachable, sorted sequences.
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

        let mut repeat_keys = KeyBitSet::new();
        file.repeat_keys.iter().for_each(|k| repeat_keys.insert(*k));

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

        let composer = composer_from_compose(&file.compose);

        // The `keymap` plane count drives how deeply the delta-compressed maps
        // are materialized; planes beyond it stay empty, matching the level
        // range the file was serialized with.
        let top = file
            .keymap
            .keys()
            .map(|&level| level as usize + 1)
            .max()
            .unwrap_or(1);

        let state_keymap = from_levels_resolved(&file.keymap, top, num_keys, Some);
        let named_key_map = from_named_levels(&file.keysym_map, num_keys);
        let num_lock_keys = from_sparse_levels(&file.num_lock_keys, num_keys, Some);
        let caps_lock_keymap = from_sparse_levels(&file.caps_lock_keymap, num_keys, Some);
        let caps_num_lock_keys = from_sparse_levels(&file.caps_num_lock_keys, num_keys, Some);

        Ok(KBLayout {
            name: file.layout,
            repeat_keys,
            composer,
            modifiers,
            state_keymap,
            num_lock_keys,
            caps_lock_keymap,
            named_key_map,
            #[cfg(feature = "xkb")]
            level_exceptions_keymap: FlatKeymap::new(num_keys),
            caps_num_lock_keys,
        })
    }
}

/// Un-flatten the delta-compressed base keymap. Each plane inherits its parent
/// plane ([`drop_top`]) before the file's own overrides are applied, so an
/// omitted `(level, key)` resolves exactly as it did before serialization.
fn from_levels_resolved<T: FlatMapValue + PartialEq, V: Copy>(
    levels: &BTreeMap<u8, BTreeMap<u32, V>>,
    top: usize,
    num_keys: usize,
    reconstruct: impl Fn(V) -> T,
) -> FlatMap<T> {
    let mut flat = FlatMap::new(num_keys);
    for level in 0..top.min(MAX_LEVELS) {
        if level > 0 {
            let parent = drop_top(level as u8) as usize * num_keys;
            let start = level * num_keys;
            let parent_plane = flat.data[parent..parent + num_keys].to_vec();
            flat.data[start..start + num_keys].copy_from_slice(&parent_plane);
        }
        if let Some(keys) = levels.get(&(level as u8)) {
            let start = level * num_keys;
            for (keycode, value) in keys {
                flat.data[start + *keycode as usize] = reconstruct(*value);
            }
        }
    }
    flat
}

/// Un-flatten the `keysym_map`. Every level starts from the invariant per-level
/// defaults ([`default_named_key`]), seeded by [`seed_named_defaults`], and the
/// file's own entries (including explicit `Unnamed` clears) are applied on top.
fn from_named_levels(levels: &NamedSection, num_keys: usize) -> FlatNamedKeyMap {
    let mut flat = FlatMap::new(num_keys);
    if !levels.is_empty() {
        seed_named_defaults(&mut flat);
    }
    for (level, keys) in levels {
        let start = (*level as usize) * num_keys;
        for (keycode, key) in keys {
            flat.data[start + *keycode as usize] = *key;
        }
    }
    flat
}

/// Un-flatten a Num Lock / Caps Lock override section sparsely: only the
/// declared levels and keys are written and every other plane stays empty, so
/// these maps keep meaning "override with exactly what is listed".
fn from_sparse_levels<T: FlatMapValue, V: Copy>(
    levels: &BTreeMap<u8, BTreeMap<u32, V>>,
    num_keys: usize,
    reconstruct: impl Fn(V) -> T,
) -> FlatMap<T> {
    let mut flat = FlatMap::new(num_keys);
    for (level, keys) in levels {
        let start = (*level as usize) * num_keys;
        for (keycode, value) in keys {
            flat.data[start + *keycode as usize] = reconstruct(*value);
        }
    }
    flat
}

fn modkind_from_modaction(action: ModAction) -> KeyEffect {
    let (mod_type, kind) = match action {
        ModAction::Press(t) => (t, ModKind::Press),
        ModAction::Lock(t, variant) => (t, ModKind::Lock(variant)),
        ModAction::Latch(t, variant) => (t, ModKind::Latch(variant)),
    };
    KeyEffect::modifier(mod_type, kind)
}

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
