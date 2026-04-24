# xkb-core

[![Crates.io](https://img.shields.io/crates/v/xkb-core.svg)](https://crates.io/crates/xkb-core)
[![Documentation](https://docs.rs/xkb-core/badge.svg)](https://docs.rs/xkb-core)
[![License](https://img.shields.io/crates/l/xkb-core.svg)](https://github.com/rano-oss/wkb/blob/main/LICENSE)

Pure Rust implementation of XKB keymap compilation, keyboard state management,
compose table support, and keysym utilities. This crate is the engine behind
[`wayland-keyboard`](https://crates.io/crates/wayland-keyboard) and is not typically used directly.

## What it provides

- XKB keymap parsing and compilation (RMLVO rules, keymap strings)
- Keyboard state tracking (modifiers, groups, LEDs)
- Compose / dead-key table parsing and matching
- Keysym ↔ UTF-8 conversion and case mapping
- XKB registry (RMLVO database) queries

## License

MIT
