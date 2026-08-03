# wkb Layout File Format

This is the normative specification for the on-disk format used by `wkb` to
persist pre-compiled keyboard layouts. Files are serialized as RON text and
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
   (ascending), then by evdev keycode (ascending). `repeat_keys_*` and
   `modifiers` are sorted ascending. `compose` is sorted lexicographically.
   Serializing the same logical file always yields byte-identical output.
4. **Minimal redundancy.** Empty sections and empty level maps are omitted
   entirely. `Unnamed` keysym entries and `None` entries are never stored.
5. **Validation.** Loading (`LayoutFile::from_ron_str`, `validate`, and both
   conversions) enforces every invariant listed under [Validation].

## Top-level structure

```ron
(
    version: 1,
    layout_names: ["Dari"],
    num_keys: 701,
    repeat_keys_add: [...],
    repeat_keys_remove: [...],
    modifiers: [...],
    keymap: {...},
    num_lock_keys: {...},
    caps_lock_keymap: {...},
    level_exceptions_keymap: {...},
    keysym_map: {...},
    compose: [...],
)
```

| Field | Type | Meaning |
|-------|------|---------|
| `version` | `u32` | Schema version, must be `1`. |
| `layout_names` | `Vec<String>` | Exactly one layout name. |
| `num_keys` | `u32` | Number of evdev keycode slots; every keycode is `< num_keys`. |
| `repeat_keys_add` | `Vec<u32>` | Keycodes that repeat. |
| `repeat_keys_remove` | `Vec<u32>` | Keycodes excluded from `repeat_keys_add`. |
| `modifiers` | `Vec<(u32, String, Vec<(u8, ModAction)>)>` | Modifier bindings: `(keycode, name, [(level, action)])`. |
| `keymap` | `Map<name, Map<level, Map<keycode, char>>>` | Resolved character per (level, keycode) under base modifiers. |
| `num_lock_keys` | same shape | Character overrides while Num Lock is locked. |
| `caps_lock_keymap` | same shape | Character overrides while Caps Lock is locked. |
| `level_exceptions_keymap` | same shape | Raw keysym characters, fallback when level resolution fails. |
| `keysym_map` | `Map<name, Map<level, Map<keycode, NamedKey>>>` | Named-key identities; `Unnamed` entries are omitted. |
| `compose` | `Vec<(Vec<char>, char)>` | Compose sequences as `(keys, output)`. |

Map shape uses `BTreeMap`, giving the canonical ordering described above. The
effective repeat set is `repeat_keys_add ∖ repeat_keys_remove`; generated files
put the complete set in `repeat_keys_add` and leave `repeat_keys_remove` empty.

## Modifiers

Each modifier binding is a `(keycode, name, actions)` tuple. `name` is
human-readable metadata only — it is ignored when loading. `actions` maps a
level to a `ModAction`:

```ron
(42, "LeftShift", [(0, Pressed(Level2))]),   # shift to level 2
(58, "CapsLock",  [(0, Lock(Caps))]),        # toggle caps lock
(69, "NumLock",   [(0, Lock(Num))]),
(100, "AltGr",    [(0, Pressed(Level3))]),
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

Each entry is `(keys, output)` where `keys` is the sequence of characters
typed after pressing Compose, and `output` is the resulting character.

The Compose/Multi_key token is represented by the reserved character
`U+00B7 MIDDLE DOT` (`·`). A literal `·` cannot be used as a compose input key.
It is used exactly as the compose marker, matching X11 `<Multi_key>`.

Files store only **reachable** sequences: a sequence is kept iff every non-marker
character in `keys` is a character the layout can produce (appears in `keymap`,
`caps_lock_keymap`, or `num_lock_keys`). This mirrors the runtime composer
filtering and keeps files free of redundant entries.

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

```ron
(
    version: 1,
    layout_names: [
        "us",
    ],
    num_keys: 701,
    repeat_keys_add: [
        1,
        2,
        3,
    ],
    modifiers: [
        (42, "LeftShift", [
            (0, Pressed(Level2)),
        ]),
    ],
    keymap: {
        "us": {
            0: {
                16: 'q',
                17: 'w',
            },
        },
    },
    compose: [
        (['·', 'a', 'e'], 'æ'),
    ],
)
```

Sections that would be empty (`num_lock_keys`, `caps_lock_keymap`,
`level_exceptions_keymap`, `keysym_map`, `compose`) are omitted from the output.

## Compatibility

The format is the successor of the prototype RON files that previously lived in
`ron_layouts/`. It preserves their section layout and value encodings, adds a
`version` header, replaces raw keysym-name strings in `keysym_map` with the
compiled `NamedKey` enum, and applies the ground rules above.
