# Changelog

All notable changes to `wayland-keyboard` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Removed duplicate `reset_compose`; use [`WKB::leave`] to clear in-progress compose.
- [`ComposeState`] carries all per-key compose data: `Idle(char)`, `ComposeKey(buf)`
  (`·` in buf), `Composing(buf)`, `Finished(char)`, `Cancelled`. No separate
  preview/active query methods.
- Extended evdev physical-key table through code 633 (single lookup table).

### Added

- `full` feature (both `compositor` and `client`) for integration tests.
- `tests/compositor_client.rs` — compositor modifier state drives client
  `key_char` / `compose`.

## [0.3.2] - 2026-08-25

### Added

- Role features: **`client`** (default) and **`compositor`** — mutually exclusive.
  Client: compose trie, `compose`, `leave`; compositor: full
  `press_key` / `release_key` state machine.
- `tests/dead_keys.rs` — pure compose vs dead-key char vs compose-trie vs xkbcommon sticky.

### Changed

- `compositor` and `client` cannot be enabled together (`compile_error`).
- Client role has no `press_key` / `release_key`; use `update_modifiers` + `compose`.
- Compositor `press_key` / `release_key` return [`StateChanges`].
- Compose state on the layout being left is reset when switching groups via `set_layout`, `update_modifiers`, or group-switch key handling.

## [0.3.0] - 2026-08-21

Version 0.3.0 is a breaking compositor-facing API change. Physical and
logical key identity are now separate, public keycodes are raw evdev
codes, and key processing reports whether modifiers or LEDs actually
changed.

### Added

- Added [`PhysicalKey`](https://docs.rs/wayland-keyboard/latest/wkb/enum.PhysicalKey.html)
  for layout-independent physical positions (`KeyA`…`KeyZ`, digits,
  punctuation, navigation, modifiers, numpad, function, and media keys).
- Added [`LogicalKey`](https://docs.rs/wayland-keyboard/latest/wkb/enum.LogicalKey.html)
  (`Character`, `Named`, `Unidentified`) resolved from the current layout
  and modifier state.
- Added `WKB::physical_key()` and `WKB::logical_key()`.
- Added [`StateChanges`](https://docs.rs/wayland-keyboard/latest/wkb/struct.StateChanges.html)
  with `modifiers_updated` and `leds_updated` flags.

### Changed

- `KeyResult` now carries `physical_key`, `logical_key`, compose state,
  `is_modifier`, `modifiers_updated`, and `leds_updated`. The old `key:
  NamedKey` field is removed.
- `press_key`, `release_key`, and `repeat_key` report actual modifier and
  LED changes by comparing state before and after the operation.
- `update_modifiers` now returns `StateChanges`.
- Named keys such as Escape and arrows stay named while Ctrl, Alt, or Logo
  are held. Modifiers only change a key when its compiled XKB type selects
  a different level.

### Removed

- Removed `KeyResult.key`.

## [0.2.0] - 2026-08-20

Version 0.2.0 is a breaking release. It replaces much of the 0.1 API and
introduces a self-contained XKB compiler, precompiled human-readable layouts,
multi-layout support, and a unified key-event API.

### Added

- Added a self-contained, pure Rust XKB parser and compiler behind the default
  `xkb` feature. The separate `xkb-core` runtime dependency is no longer
  required.
- Added XKB keymap construction from RMLVO names through
  `WKB::new_from_names()` and from XKB text through `WKB::new_from_string()`.
- Added XKB v1 keymap serialization through `WKB::as_xkb_string()`.
- Added the human-readable RON layout format and its public intermediate
  representation in `wkb::ir`.
- Added `LayoutFile::from_ron_str()`, `LayoutFile::to_ron_string()`, and
  `LayoutFile::validate()`.
- Added `WKB::new_from_layouts()` for applications that ship precompiled
  layouts without the XKB compiler.
- Added `WKB::export_layout()` for exporting compiled layouts to the portable
  layout representation.
- Added multi-layout keymaps, explicit layout selection, and group switching.
- Added `NamedKey` so non-character keys retain a stable symbolic identity.
- Added `RawModifiers` and the `shift()`, `ctrl()`, `alt()`, `logo()`,
  `caps_lock()`, and `num_lock()` convenience queries.
- Added the structured `LedState` result for Caps Lock, Num Lock, and Scroll
  Lock indicators.
- Added separate `press_key()`, `release_key()`, and `repeat_key()` event
  methods returning `KeyResult`.
- Added compose-state results for idle, in-progress, completed, and cancelled
  compose sequences.
- Added support for Level5 modifiers, modifier latching, group actions, and
  combined Caps Lock and Num Lock mappings.

### Changed

- Increased the minimum supported Rust version from 1.70 to 1.82.
- Changed `WKB` from a composer-generic type to a concrete keyboard state
  type.
- Changed `WKB::new_from_names()` to accept RMLVO components and return a
  `Result<WKB, XkbError>`.
- Changed `WKB::new_from_string()` to borrow XKB text and return a
  `Result<WKB, XkbError>`.
- Replaced the combined `key()` and `key_compose()` interface with the
  explicit key-event methods.
- Replaced raw keysym results with `NamedKey` in `KeyResult` and named-key
  lookup methods.
- Changed `leds_state()` from an integer bitmask to `LedState`.
- Replaced the old modifier-state tuple interface with `RawModifiers` and
  dedicated modifier queries.
- Changed `repeat_key()` to mutate compose state and therefore require
  `&mut self`.
- Reworked keymaps into flat lookup tables and compose sequences into a compact
  trie, reducing runtime work and allocation.
- Made XKB compiler-only dependencies optional and tied them to the `xkb`
  feature.

### Removed

- Removed the public `testing` feature and `wkb::testing` module.
- Removed the `xkb-core` workspace crate and dependency.
- Removed `WKB`'s public composer type parameter.
- Removed the old `key()`, `key_compose()`, `utf8()`, `level_key()`,
  `num_levels()`, `layouts()`, and `current_layout()` APIs.
- Removed the old LED bitmask constants from the primary LED-state API.

### Fixed

- Improved modifier press, release, lock, latch, and serialization behavior.
- Improved Caps Lock, Num Lock, AltGr, Level5, and combined lock handling.
- Improved XKB include processing, keysym conversion, key type compilation,
  group actions, and multi-layout state transitions.
- Improved compose parsing and state handling, including repeated keys and
  cancelled sequences.
- Improved XKB keymap serialization compatibility.

### Migration from 0.1

Construction and key processing now return structured results:

```rust
use wkb::WKB;

let keymap = std::fs::read_to_string("/path/to/keymap")?;
let mut keyboard = WKB::new_from_string(&keymap)?;

let result = keyboard.press_key(30);
let character = keyboard.key_char(30);

println!("key={:?} char={character:?}", result.key);

keyboard.release_key(30);

let modifiers = keyboard.raw_modifiers();
let leds = keyboard.leds_state();

println!(
    "shift={} caps={} depressed={:#x}",
    keyboard.shift(),
    leds.caps_lock,
    modifiers.depressed,
);

# Ok::<(), Box<dyn std::error::Error>>(())
```

Applications that do not need runtime XKB compilation can disable default
features and construct `WKB` from precompiled `LayoutFile` values.

## [0.1.0] - 2026-04-23

- Initial crates.io release.
- Added XKB keymap loading, keyboard state tracking, modifier handling,
  compose sequences, LED state, repeat information, and XKB serialization.

[0.2.0]: https://github.com/rano-oss/wkb/compare/abf2491d2aa6dba9d0008aa473a331fa53a78395...v0.2.0
[0.1.0]: https://github.com/rano-oss/wkb/commit/abf2491d2aa6dba9d0008aa473a331fa53a78395
