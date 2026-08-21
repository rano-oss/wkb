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
wayland-keyboard = "0.3"
```

```rust,no_run
use wkb::WKB;

// Build from an XKB keymap string (e.g. received from a Wayland compositor)
let keymap_string = std::fs::read_to_string("/path/to/keymap").unwrap();
let mut wkb = WKB::new_from_string(&keymap_string).unwrap();

// Process a key press (evdev code 30 = physical KeyA)
let result = wkb.press_key(30);
println!(
    "physical={:?} logical={:?} compose={:?}",
    result.physical_key, result.logical_key, result.compose
);

// Release the key
let result = wkb.release_key(30);

// Query current modifier state
let mods = wkb.raw_modifiers();
println!("ctrl depressed={}", mods.depressed);
```

### Key Event API

| Method | Mutates state | Use case |
|--------|--------------|----------|
| `press_key(evdev)` | yes | Key down — updates modifiers, advances compose |
| `release_key(evdev)` | yes | Key up — updates modifiers |
| `repeat_key(evdev)` | yes | Key repeat — advances compose |
| `key_char(evdev)` | no | Raw character under current modifiers (no compose) |
| `physical_key(evdev)` | no | Physical position from the evdev code alone |
| `logical_key(evdev)` | no | Logical identity under layout + modifiers |

All three event methods return a [`KeyResult`](https://docs.rs/wayland-keyboard/latest/wkb/struct.KeyResult.html)
with physical and logical identity, compose state, whether the key is a
modifier, and whether modifiers or LEDs changed. Keycodes are always raw
Linux/evdev codes.

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
| `xkb` | yes | XKB keymap compilation |
| `compose` | yes | Compose-key / dead-key sequence support |

## Benchmarks

<!-- BENCHMARK_START -->
*Last updated: 2026-08-21 (automated via CI)*

### Speed

| Benchmark | wkb | wkb-noxkb | xkbcommon | xkbcommon-dl | vs xkbcommon |
|-----------|-----|-----------|-----------|--------------|-------------|
| Setup (no compose) | 1.35 ms | 1.00 ms | 2.20 ms | 2.19 ms | **1.6x faster** |
| Setup (with compose) | 1.37 ms | 1.01 ms | 4.06 ms | 4.03 ms | **3.0x faster** |
| Key update | 162 ns | 161 ns | 167 ns | 168 ns | ~same |
| Get char | 169 ns | 167 ns | 300 ns | 296 ns | **1.8x faster** |
| Get keysym | 166 ns | 170 ns | 237 ns | 207 ns | **1.4x faster** |
| Compose feed | 448 ns | 461 ns | 512 ns | 495 ns | **1.1x faster** |

### Memory

| Library | Peak RSS |
|---------|----------|
| wkb | 4.4 MB |
| wkb-noxkb | 3.2 MB |
| xkbcommon | 4.7 MB |
| xkbcommon-dl | 4.7 MB |

### Binary Size

Sizes for xkbcommon and xkbcommon-dl include the dynamically-linked `libxkbcommon.so`.

| Binary | Size (stripped) |
|--------|----------------|
| wkb (no XKB) | 714 KB |
| wkb (with XKB) | 633 KB |
| xkbcommon | 730 KB |
| xkbcommon-dl | 763 KB |

<!-- BENCHMARK_END -->

## Scope and Limitations

WKB targets the subset of XKB used by Wayland clients and compositors.
Geometry descriptions and other X11-only features are intentionally out of
scope.

## License

MIT
