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
- **Multi-layout keymaps** — supports multiple layouts with group switching.
- **LED state** — query Caps/Num/Scroll Lock indicator state.
- **Repeat info** — query whether a key repeats.
- **Lightweight** — no C FFI, no `unsafe` beyond `Send`/`Sync` impls, minimal
  dependencies.

## Quick Start

```toml
[dependencies]
wayland-keyboard = "0.1"
```

```rust,no_run
use wkb::WKB;

// Build from an XKB keymap string (e.g. received from a Wayland compositor)
let keymap_string = std::fs::read_to_string("/path/to/keymap").unwrap();
let mut wkb = WKB::new_from_string(&keymap_string).unwrap();

// Process a key press (evdev code 38 = 'a' on US layout)
let result = wkb.press_key(38);
println!("keysym: {:#x}, compose: {:?}", result.keysym, result.compose);

// Release the key
let result = wkb.release_key(38);

// Query current modifier state
let mods = wkb.modifiers_state();
println!("ctrl={} alt={} shift={}", mods.ctrl, mods.alt, mods.shift);
```

### Key Event API

| Method | Mutates state | Use case |
|--------|--------------|----------|
| `press_key(evdev)` | yes | Key down — updates modifiers, advances compose |
| `release_key(evdev)` | yes | Key up — updates modifiers |
| `repeat_key(evdev)` | yes | Key repeat — advances compose |
| `key_char(evdev)` | no | Raw character under current modifiers (no compose) |

All three event methods return a [`KeyResult`](https://docs.rs/wayland-keyboard/latest/wkb/struct.KeyResult.html)
containing the keysym, compose state, and whether the key is a modifier.

### Compositor Usage

```rust,no_run
use wkb::WKB;

// Build from RMLVO names (compositor side)
let wkb = WKB::new_from_names("evdev", "pc105", "us,de", "dvorak,", None).unwrap();

// Serialize to XKB string for wl_keyboard.keymap
let xkb_string = wkb.as_xkb_string().unwrap();

// Switch layouts via group index (no re-parsing needed)
// wkb.set_layout(1).unwrap(); // switch to German
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `xkb` | yes | XKB keymap compilation via the `xkb-core` crate |
| `compose` | yes | Compose-key / dead-key sequence support |
| `testing` | no | Exposes internal helpers for integration tests (not public API) |

## Benchmarks

<!-- BENCHMARK_START -->
*Last updated: 2026-04-27 (automated via CI)*

### Memory

| Library | Peak RSS |
|---------|----------|
| wkb | 5.4 MB |
| xkbcommon | 5.6 MB |
| xkbcommon-dl | 5.6 MB |

### Binary Size

Sizes for xkbcommon and xkbcommon-dl include the dynamically-linked `libxkbcommon.so`.

| Binary | Size (stripped) |
|--------|----------------|
| wkb | 1173 KB |
| xkbcommon | 701 KB |
| xkbcommon-dl | 735 KB |

<!-- BENCHMARK_END -->

## Scope and Limitations

WKB targets the subset of XKB used by Wayland clients and compositors.
Geometry descriptions and other X11-only features are intentionally out of
scope.

## License

MIT
