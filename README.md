# wkb — Wayland Keyboard

[![Crates.io](https://img.shields.io/crates/v/wayland-keyboard.svg)](https://crates.io/crates/wayland-keyboard)
[![Documentation](https://docs.rs/wayland-keyboard/badge.svg)](https://docs.rs/wayland-keyboard)
[![License](https://img.shields.io/crates/l/wayland-keyboard.svg)](https://github.com/rano-oss/wkb/blob/main/LICENSE)
[![Test Status](https://img.shields.io/github/actions/workflow/status/rano-oss/wkb/tests.yml?branch=main&event=push&label=tests)](https://github.com/rano-oss/wkb/actions)

A lightweight, pure Rust keyboard handling library for Wayland. WKB is a
drop-in alternative to `xkbcommon` that compiles XKB keymaps, tracks modifier
and compose state, and maps evdev key codes to characters — all without C
dependencies.

## Features

- **XKB keymap import** — parse keymap strings received from Wayland
  compositors via `wl_keyboard`.
- **XKB keymap export** — serialize keymaps back to XKB v1 text format via
  `as_xkb_string()`.
- **Full modifier support** — Shift, Ctrl, Alt, AltGr, Caps Lock, Num Lock,
  Scroll Lock, and multi-level keys.
- **Compose sequences** — built-in compose key and automatic compose handling.
- **LED state** — query Caps/Num/Scroll Lock indicator state.
- **Repeat info** — query whether a key repeats.
- **Lightweight** — no C FFI, no `unsafe`, minimal dependencies.

## Quick Start

```toml
[dependencies]
wayland-keyboard = "0.1"
```

```rust
use wkb::{WKB, KeyDirection};

// Build from an XKB keymap string (e.g. received from a Wayland compositor)
let keymap_string = std::fs::read_to_string("/path/to/keymap").unwrap();
let mut wkb = WKB::new_from_string(keymap_string);

// Process a key press (evdev code 38 = 'a' on US layout)
let (ch, is_modifier) = wkb.key(38, KeyDirection::Down);
assert_eq!(ch, Some('a'));
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `xkb` | yes | XKB keymap compilation via the `xkb-core` crate |
| `compose` | yes | Compose-key / dead-key sequence support |
| `testing` | no | Exposes internal helpers for integration tests (not public API) |

## Benchmarks

<!-- BENCHMARK_START -->
*Last updated: 2026-04-25 (automated via CI)*

### Speed

| Benchmark | wkb | xkbcommon | xkbcommon-dl | vs xkbcommon |
|-----------|-----|-----------|--------------|-------------|
| Full setup | 14.43 ms | 4.99 ms | 4.99 ms | 2.9x slower |
| Key update | 38 ns | 200 ns | 203 ns | **5.3x faster** |
| Get UTF-8 | 125 ns | 388 ns | 309 ns | **3.1x faster** |
| Get keysym | 120 ns | 244 ns | 248 ns | **2.0x faster** |
| Compose setup | 7.72 ms | 3.53 ms | 2.33 ms | 2.2x slower |
| Compose feed | 23 ns | 45 ns | 46 ns | **2.0x faster** |

<!-- BENCHMARK_END -->

## Scope and Limitations

WKB targets the subset of XKB used by Wayland clients and compositors.
Geometry descriptions and other X11-only features are intentionally out of
scope. A future native keyboard format (TOML/RON) is planned but not yet
available.

## License

MIT
