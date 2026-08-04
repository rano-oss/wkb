# wkb Layout File Format

This is the normative specification for the on-disk format used by `wkb` to
persist pre-compiled keyboard layouts. Files are serialized as **KDL** text and
read/written through the `wkb::ir` module.

## Purpose

The runtime representation ([`KBLayout`]) is an in-memory, pre-compiled form of
a keyboard layout. A layout file is the serialized version of one such layout.
It is produced by `KBLayout -> LayoutFile` (`WKB::export_layout`) and consumed
by `LayoutFile -> KBLayout` (`WKB::new_from_layouts`, which takes a list of
files for multi-group layouts), enabling wkb to load
layouts without compiling XKB at runtime.

## Ground rules

1. **One layout per file.** `layout_names` holds exactly one name, and every
   section map is keyed by exactly that name.
2. **Versioned schema.** Every file starts with `version`, which must equal the
   current `FORMAT_VERSION` (1). Files with any other version are rejected.
3. **Canonical ordering.** All maps are sorted: by layout name, then by level
   (ascending), then by evdev keycode (ascending). `repeat_keys` and
   `modifiers` are sorted ascending. `compose` is sorted lexicographically.
   Serializing the same logical file always yields byte-identical output.
4. **Minimal redundancy.** Empty sections and empty level maps are omitted
   entirely. `Unnamed` keysym entries and `None` entries are never stored.
5. **Validation.** Loading (`LayoutFile::from_kdl_str`, `validate`, and both
   conversions) enforces every invariant listed under [Validation].

## Top-level structure

```kdl
// wkb keyboard layout (KDL format)
version 1
layout "English (US)"
num_keys 701

repeat_keys 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 \
     21 22 23 24 25 26 27 28 30 31 32 33 34 35 36 37 38 39 40 \
     ...

modifier 29 "LeftControl" 0 "Pressed"
modifier 42 "LeftShift" 0 "Pressed(Level2)"
modifier 58 "CapsLock" 0 "Lock(Caps)"

keymap {
    level 0 "1"="\u{1b}" "2"="1" "3"="2" "16"="q" "17"="w" ...
    level 1 "1"="\u{1b}" "2"="!" "16"="Q" "17"="W" ...
}

keysym_map {
    level 0 "1"="Escape" "2"="1" "16"="q" ...
}

compose "·" "a" "e" "æ"
compose "·" " " "(" "˘"
```

A comment line `// wkb keyboard layout (KDL format)` begins the file. The top
level is a sequence of nodes:

| Node | Kind | Meaning |
|------|------|---------|
| `version` | `u32` argument | Schema version, must be `1`. |
| `layout` | quoted string argument | The single layout name. |
| `num_keys` | `u32` argument | Number of evdev keycode slots; every keycode is `< num_keys`. |
| `repeat_keys` | `u32` arguments | Keycodes that repeat. Wrapped at 20 per line with a trailing `\` continuation. |
| `modifier` | mixed arguments | Modifier binding; see [Modifiers]. |
| `keymap` | node with children | Resolved character per (level, keycode) under base modifiers. |
| `num_lock_keys` | node with children | Character overrides while Num Lock is locked. |
| `caps_lock_keymap` | node with children | Character overrides while Caps Lock is locked. |
| `keysym_map` | node with children | Named-key identities; `Unnamed` entries are omitted. |
| `compose` | string arguments | One compose sequence per node; see [Compose sequences]. |

The `keymap`/`num_lock_keys`/`caps_lock_keymap`/`keysym_map` nodes each contain
one `level <n>` child line per populated level. Entries on a level line are
quoted property pairs `"keycode"="value"` — the numeric keycodes are always
quoted, because bare numeric property keys are not valid KDL. Within a level
line the keycode is the primary sort key, giving a canonical order. Empty
sections (including a layout whose section has no populated levels) are omitted
from the output entirely.

## Strings and escaping

Quoted strings are used for the layout name, modifier names, character values,
keysym names, and compose characters. The writer escapes characters that KDL
forbids literally:

- `\` and `"` are backslash-escaped (`\\`, `\"`).
- `\n`, `\r`, `\t`, `\b`, `\f` use the short escapes.
- All other control characters (e.g. `\u{1b}` for Escape) and Unicode bidi
  direction controls (U+200E–200F, U+202A–202E, U+2066–2069) plus U+FEFF are
  written as `\u{hex}` escapes.

The parser accepts the same set, so serialization and parsing round-trip
byte-for-byte (verified across all generated fixtures).

## Modifiers

Each modifier binding is a `modifier` node with arguments
`keycode name (level action)*`. `name` is human-readable metadata only — it is
ignored when loading. `action` is the `ModAction` string:

```kdl
modifier 42 "LeftShift" 0 "Pressed(Level2)"
modifier 58 "CapsLock" 0 "Lock(Caps)"
modifier 69 "NumLock" 0 "Lock(Num)"
modifier 100 "AltGr" 0 "Pressed(Level3)"
```

`ModAction` mirrors the runtime modifier state machine:

```rust
pub enum ModAction {
    Pressed(ModType),  // held key selects a level
    Lock(ModType),     // toggles a lock (Caps, Num)
    Latch(ModType),    // one-shot latch
    None,
}
```

`ModType` is one of `None`, `Level2`, `Level3`, `Level5`, `Compose`, `Caps`,
`Num`, `Scroll`.

A single `(0, action)` binding becomes a `Single` modifier; anything else
becomes a leveled modifier.

## Compose sequences

Each `compose` node is one sequence: the characters typed after pressing
Compose, followed by the resulting character as the final argument.

The Compose/Multi_key token is represented by the reserved character
`U+00B7 MIDDLE DOT` (`·`). A literal `·` cannot be used as a compose input key.
It is used exactly as the compose marker, matching X11 `<Multi_key>`.

```kdl
compose "·" "a" "e" "æ"
```

Files store only **reachable** sequences: a sequence is kept iff every
non-marker character in `keys` is a character the layout can produce (appears
in `keymap`, `caps_lock_keymap`, or `num_lock_keys`). This mirrors the runtime
composer filtering and keeps files free of redundant entries.

## Validation

On load, the following are enforced (each maps to an `IrError` variant):

- `version` equals `FORMAT_VERSION`, else `UnsupportedVersion`.
- `layout_names` is non-empty (`EmptyLayoutNames`), contains no duplicates
  (`DuplicateLayoutName`), and has exactly one entry (`MultipleLayouts`).
- Every section map key is declared in `layout_names`
  (`UndeclaredLayout`).
- `num_keys >= 1` (`InvalidNumKeys`).
- Every keycode is `< num_keys` (`KeycodeOutOfRange`).
- Every level is `< 8` (`LevelOutOfRange`).
- Modifier names are non-empty (`EmptyModifierName`) and actions non-empty
  (`EmptyModifierActions`).
- Compose sequences are non-empty (`EmptyComposeSequence`) and contain no NUL
  (`NullComposeKey` / `NullComposeOutput`).

## Canonical example

```kdl
// wkb keyboard layout (KDL format)
version 1
layout "us"
num_keys 701

repeat_keys 1 2 3 \
     4 5 6
modifier 42 "LeftShift" 0 "Pressed(Level2)"

keymap {
    level 0 "16"="q" "17"="w" "30"="a" "31"="s"
    level 1 "16"="Q" "17"="W" "30"="A" "31"="S"
}

compose "·" "a" "e" "æ"
```

Sections that would be empty (`num_lock_keys`, `caps_lock_keymap`,
`keysym_map`, `compose`) are omitted from the output.

## Compatibility

The format is the successor of the RON files that previously lived in
`ron_layouts/`. It preserves their section layout and value encodings, adds a
`version` header, replaces raw keysym-name strings in `keysym_map` with the
compiled `NamedKey` enum, drops the xkb-only `level_exceptions_keymap` section,
and applies the ground rules above. KDL was chosen over RON for its compact,
line-oriented serialization: the canonical `us.kdl` is roughly one third the
size of its RON equivalent. Fixtures are regenerated into `kdl_layouts/` by the
`gen_layouts` example and are gitignored.
