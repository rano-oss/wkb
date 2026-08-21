//! Logical key identity after layout and modifier resolution.
//!
//! [`LogicalKey`] describes what a key *means* under the current layout and
//! modifier state. It is independent of the key's physical position
//! ([`crate::PhysicalKey`]) and of compose-sequence output.

use crate::named_keys::NamedKey;

/// Logical identity of a key under the current layout and modifiers.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LogicalKey {
    /// A printable character produced by this key.
    Character(char),
    /// A named functional key (Escape, arrows, modifiers, etc.).
    Named(NamedKey),
    /// No usable logical identity for this key under the current state.
    #[default]
    Unidentified,
}
