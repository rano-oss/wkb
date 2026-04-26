# xkb-core

[![Crates.io](https://img.shields.io/crates/v/xkb-core.svg)](https://crates.io/crates/xkb-core)
[![Documentation](https://docs.rs/xkb-core/badge.svg)](https://docs.rs/xkb-core)
[![License](https://img.shields.io/crates/l/xkb-core.svg)](https://github.com/rano-oss/wkb/blob/main/LICENSE)

Pure Rust implementation of XKB keymap compilation and keysym utilities.
This crate is the parsing engine behind
[`wayland-keyboard`](https://crates.io/crates/wayland-keyboard) (wkb) and is
not typically used directly.

## What it provides

- XKB keymap parsing and compilation (RMLVO rules, keymap strings)
- Compose / dead-key table parsing
- Keysym-to-Unicode conversion and case mapping
- XKB registry (RMLVO database) queries

## Note

xkb-core is used **only at construction time** — for parsing XKB files into
wkb's flat lookup tables. It is not used at runtime for key event processing.

## License

MIT
