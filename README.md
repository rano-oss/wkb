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
*Last updated: 2026-04-23 (automated via CI)*

### Speed

| Benchmark | wkb | xkbcommon | xkbcommon-dl | vs xkbcommon |
|-----------|-----|-----------|--------------|-------------|
| full setup | 13.85 ms | 4.27 ms | 4.26 ms | 3.2x slower |
| key update | 32 ns | 172 ns | 172 ns | **5.4x faster** |
| key get utf8 | 93 ns | 282 ns | 265 ns | **3.0x faster** |
| key get sym | 92 ns | 208 ns | 207 ns | **2.3x faster** |
| compose table creation | 2.44 ms | 1.85 ms | 1.90 ms | 1.3x slower |
| compose feed | 16 ns | 28 ns | 29 ns | **1.8x faster** |

### Memory

| Library | Peak RSS |
|---------|----------|
| wkb | 11.1 MB |
| xkbcommon | 11.4 MB |
| xkbcommon-dl | 11.4 MB |

### Binary Size

| Binary | Size (stripped) |
|--------|----------------|
| wkb | 1641 KB |
| xkbcommon | 341 KB |
| xkbcommon-dl | 375 KB |

<!-- BENCHMARK_END -->

## Scope and Limitations

WKB targets the subset of XKB used by Wayland clients and compositors.
Geometry descriptions and other X11-only features are intentionally out of
scope. A future native keyboard format (TOML/RON) is planned but not yet
available.

## License

MIT
