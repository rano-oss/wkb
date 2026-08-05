# wkb Layout File Format

This is the normative specification for the on-disk format used by `wkb` to
persist pre-compiled keyboard layouts. Files are serialized as **RON** text and
read/written through the `wkb::ir` module.

## Purpose

The runtime representation ([`KBLayout`]) is an in-memory, pre-compiled form of
a keyboard layout. A layout file is the serialized version of one such layout.
It is produced by `KBLayout -> LayoutFile` (`WKB::export_layout`) and consumed
by `LayoutFile -> KBLayout` (`WKB::new_from_layouts`, which takes a list of
files for multi-group layouts), enabling wkb to load layouts without compiling
XKB at runtime.

## Ground rules

1. **One layout per file.** A file holds exactly one layout, named by the
   `layout` field. Multi-group instances are produced by loading several files.
2. **Versioned schema.** Every file starts with `version`, which must equal the
   current `FORMAT_VERSION` (1). Files with any other version are rejected.
3. **Compile-time keycode count.** `NUM_KEYS` (701) is a code constant, not a
   file field. Every keycode in a file is `< NUM_KEYS` and the runtime layout
   is always sized to `NUM_KEYS` slots.
4. **Canonical ordering.** All maps are sorted by level (ascending), then by
   evdev keycode (ascending). `repeat_keys` and `modifiers` are sorted
   ascending by keycode. `compose` is sorted lexicographically. Serializing the
   same logical file always yields byte-identical output.
5. **Minimal redundancy.** Empty sections are omitted entirely. `Unnamed`
   keysym entries and `None` entries are never stored.
6. **Validation.** Loading (`LayoutFile::from_ron_str`, `validate`, and both
   conversions) enforces every invariant listed under [Validation].

## Top-level structure

```ron
// wkb keyboard layout (RON format)
(
    version: 1,
    layout: "English (US)",
    repeat_keys: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
        43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 55, 57],
    modifiers: [
        (29, "LeftControl", [(0, Pressed(None))]),
        (42, "LeftShift", [(0, Pressed(Level2))]),
        (58, "CapsLock", [(0, Lock(Caps))]),
    ],
    keymap: {
        0: {
            1: '\u{1b}', 2: '1', 3: '2', 4: '3', 5: '4', 6: '5', 7: '6', 8: '7',
            9: '8', 10: '9', 11: '0', 12: '-', 13: '=', 14: '\u{7f}', 15: '\t',
            16: 'q',
        },
        1: {
            1: '\u{1b}', 2: '!', 3: '@', 4: '#', 5: '$', 6: '%', 7: '^', 8: '&',
            9: '*', 10: '(', 11: ')', 12: '_', 13: '+', 14: '\u{7f}', 15: '\t',
            16: 'Q',
        },
    },
    keysym_map: {
        0: {
            1: Escape,
            14: Backspace,
            15: Tab,
        },
    },
    compose: [
        (['·', 'a', 'e'], 'æ'),
        (['·', ' ', '(', 'x'], 'x'),
    ],
)
```

A comment line `// wkb keyboard layout (RON format)` begins the file. The root
is a RON struct with the following fields:

| Field | Kind | Meaning |
|-------|------|---------|
| `version` | `u32` | Schema version, must be `1`. |
| `layout` | string | The single layout name. |
| `repeat_keys` | array of `u32` | Keycodes that repeat. Wrapped at 20 per line. |
| `modifiers` | array of tuples | Modifier bindings; see [Modifiers]. |
| `keymap` | nested maps | Resolved character per (level, keycode) under base modifiers. |
| `num_lock_keys` | nested maps | Character overrides while Num Lock is locked. |
| `caps_lock_keymap` | nested maps | Character overrides while Caps Lock is locked. |
| `keysym_map` | nested maps | Named-key identities; `Unnamed` entries are omitted. |
| `compose` | array of tuples | Compose sequences; see [Compose sequences]. |

Every section that would be empty is omitted from the output (and defaults to
empty on load).

## Sections

`keymap`, `num_lock_keys`, `caps_lock_keymap`, and `keysym_map` share one
shape: an outer map from level to an inner map from keycode to value. The
char-keyed sections (`keymap`, `num_lock_keys`, `caps_lock_keymap`) start a new
line when the keycode exceeds a multiple of 14 (so keys 1-14, 15-28, ... each
share a line), while `keysym_map` puts one key per line:

```ron
keymap: {
    0: {
        1: '\u{1b}', 2: '1', 3: '2', 4: '3', 5: '4', 6: '5', 7: '6', 8: '7',
        9: '8', 10: '9', 11: '0', 12: '-', 13: '=', 14: '\u{7f}',
        15: '\t', 16: 'q',
    },
}

keysym_map: {
    0: {
        1: Escape,
        14: Backspace,
        15: Tab,
    },
}
```

`keymap`/`num_lock_keys`/`caps_lock_keymap` values are single-character RON
char literals; `keysym_map` values are `NamedKey` names written as bare
identifiers (e.g. `Escape`, `ArrowUp`). Levels with no populated keycodes are
omitted.

`repeat_keys` is a single array of keycodes, wrapped at 20 per line:

```ron
repeat_keys: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40],
```

## Strings and escaping

Strings (the layout name and modifier names) and char literals (character and
compose values) use RON's Rust-style escaping:

- `\` and the delimiter (`"` for strings, `'` for chars) are backslash-escaped.
- `\n`, `\r`, `\t` use the short escapes.
- All other control characters (e.g. `\u{1b}` for Escape) and U+FEFF are
  written as `\u{hex}` escapes.

The parser accepts the same set, so serialization and parsing round-trip
byte-for-byte (verified across all generated fixtures).

## Modifiers

Modifier bindings are a list of tuples `(keycode, name, [(level, action)])`,
one per line. `name` is human-readable metadata only — it is ignored when
loading. `action` is the `ModAction`:

```ron
modifiers: [
    (42, "LeftShift", [(0, Pressed(Level2))]),
    (58, "CapsLock", [(0, Lock(Caps))]),
    (69, "NumLock", [(0, Lock(Num))]),
    (100, "AltGr", [(0, Pressed(Level3))]),
],
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

Compose sequences are a list of tuples `(keys, output)`, one per line. `keys`
is a list of characters typed after pressing Compose; `output` is the resulting
character.

The Compose/Multi_key token is represented by the reserved character
`U+00B7 MIDDLE DOT` (`·`). A literal `·` cannot be used as a compose input key.
It is used exactly as the compose marker, matching X11 `<Multi_key>`.

```ron
compose: [
    (['·', 'a', 'e'], 'æ'),
],
```

Files store only **reachable** sequences: a sequence is kept iff every
non-marker character in `keys` is a character the layout can produce (appears
in `keymap`, `caps_lock_keymap`, or `num_lock_keys`). This mirrors the runtime
composer filtering and keeps files free of redundant entries.

## Validation

On load, the following are enforced (each maps to an `IrError` variant):

- `version` equals `FORMAT_VERSION`, else `UnsupportedVersion`.
- `layout` is non-empty (`EmptyLayoutName`).
- Every keycode is `< NUM_KEYS` (`KeycodeOutOfRange`). Applies to
  `repeat_keys`, modifier keycodes, and all section keycodes.
- Every level is `< 8` (`LevelOutOfRange`).
- Modifier names are non-empty (`EmptyModifierName`) and actions non-empty
  (`EmptyModifierActions`).
- Compose sequences are non-empty (`EmptyComposeSequence`) and contain no NUL
  (`NullComposeKey` / `NullComposeOutput`).

## Canonical example

```ron
// wkb keyboard layout (RON format)
(
    version: 1,
    layout: "us",
    repeat_keys: [1, 2, 3, 4, 5, 6],
    modifiers: [
        (42, "LeftShift", [(0, Pressed(Level2))]),
    ],
    keymap: {
        0: {
            16: 'q',
            17: 'w',
            30: 'a',
            31: 's',
        },
        1: {
            16: 'Q',
            17: 'W',
            30: 'A',
            31: 'S',
        },
    },
    compose: [
        (['·', 'a', 'e'], 'æ'),
    ],
)
```

Sections that would be empty (`num_lock_keys`, `caps_lock_keymap`,
`keysym_map`, `compose`) are omitted from the output.

## Compatibility

The format is the third revision of wkb's on-disk format. It follows the RON
files of the original release and the second-revision KDL format, preserving
their section layout and value encodings while applying the ground rules
above. Compared to the KDL revision: `num_keys` was replaced by the `NUM_KEYS`
compile-time constant, `layout_names` collapsed to a single `layout` name,
`modifiers`/`compose` became plain lists instead of nested child blocks, and
lists are emitted compactly (repeat keys on a few wrapped lines, one compose
sequence per line) rather than one element per line. Fixtures are regenerated
into `ron_layouts/` by the `gen_layouts` example and are gitignored.
