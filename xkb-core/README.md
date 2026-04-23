# xkb-core

Pure Rust implementation of XKB keymap compilation, keyboard state management,
compose table support, and keysym utilities. This crate is the engine behind
[`wkb`](https://crates.io/crates/wkb) and is not typically used directly.

## What it provides

- XKB keymap parsing and compilation (RMLVO rules, keymap strings)
- Keyboard state tracking (modifiers, groups, LEDs)
- Compose / dead-key table parsing and matching
- Keysym ↔ UTF-8 conversion and case mapping
- XKB registry (RMLVO database) queries

## License

MIT
