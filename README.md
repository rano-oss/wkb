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

// Compositor: build from an XKB keymap string (e.g. received from wl_keyboard.keymap)
let keymap_string = std::fs::read_to_string("/path/to/keymap").unwrap();
let mut wkb = WKB::new_from_string(&keymap_string).unwrap();

// Key down / up update modifier and group state (compositor feature)
let changes = wkb.press_key(30); // evdev code 30 = KeyA
wkb.release_key(30);

// Query current modifier state
let mods = wkb.raw_modifiers();
println!("ctrl depressed={}", mods.depressed);
```

### Client usage (winit, etc.)

Clients receive modifier state from the compositor via `wl_keyboard.modifiers`.
Use `update_modifiers` and drive compose with `compose` — there is no
`press_key` / `release_key` on the client feature.

```rust,no_run
use wkb::WKB;

let mut wkb = WKB::new_from_string(&keymap_string).unwrap();

// After wl_keyboard.modifiers
wkb.update_modifiers(depressed, latched, locked, group);

// On key press: lookup + compose
let ch = wkb.key_char(30);
if let Some(wkb::ComposeState::Finished(c)) = wkb.compose(30) {
    println!("composed: {}", c);
}
```

### Key Event API

| Method | Role | Mutates state | Use case |
|--------|------|--------------|----------|
| `press_key(evdev)` | compositor | yes | Key down — updates modifiers/groups |
| `release_key(evdev)` | compositor | yes | Key up — updates modifiers/groups |
| `update_modifiers(...)` | client | yes | Apply `wl_keyboard.modifiers` |
| `compose(evdev)` | client | yes | Advance compose sequence on key down |
| `key_char(evdev)` | both | no | Character under current modifiers |
| `physical_key(evdev)` | both | no | Physical position from evdev alone |
| `logical_key(evdev)` | both | no | Logical identity under layout + modifiers |

Compositor `press_key` / `release_key` return [`StateChanges`](https://docs.rs/wayland-keyboard/latest/wkb/struct.StateChanges.html).
Keycodes are always raw Linux/evdev codes.

### Compositor Usage

```rust,no_run
use wkb::WKB;

// Build from RMLVO names (compositor side)
let wkb = WKB::new_from_names("evdev", "pc105", "us,de", "dvorak,", None).unwrap();

// Serialize to XKB string for wl_keyboard.keymap
let xkb_string = wkb.as_xkb_string();

// Switch layouts via group index (no re-parsing needed)
// wkb.set_layout(1).unwrap(); // switch to German
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `client` | yes | Client role — `compose`, `update_modifiers` (mutually exclusive with `compositor`) |
| `compositor` | no | Compositor role — `press_key`, `release_key` |
| `xkb` | yes | XKB keymap compilation |
| `compose` | yes | Compose-key / dead-key sequence support |

## Benchmarks

<!-- BENCHMARK_START -->
*Last updated: 2026-08-27 (automated via CI)*

### Speed

| Benchmark | wkb | wkb-noxkb | xkbcommon | xkbcommon-dl | vs xkbcommon |
|-----------|-----|-----------|-----------|--------------|-------------|
| Setup (compositor) | 1.04 ms | 168.5 µs | 1.62 ms | 1.64 ms | **1.6x faster** |
| Setup (client + compose) | 1.10 ms | 172.5 µs | 3.20 ms | 3.22 ms | **2.9x faster** |
| Compositor key update | 112 ns | 112 ns | 126 ns | 125 ns | **1.1x faster** |
| Compositor get char | 117 ns | 119 ns | 222 ns | 205 ns | **1.9x faster** |
| Compositor get keysym | 118 ns | 118 ns | 173 ns | 170 ns | **1.5x faster** |
| Client update_modifiers | 189 ns | 189 ns | 379 ns | 375 ns | **2.0x faster** |
| Client get char | 196 ns | 196 ns | 475 ns | 450 ns | **2.4x faster** |
| Client get keysym | 197 ns | 198 ns | 421 ns | 406 ns | **2.1x faster** |
| Compose feed | 405 ns | 403 ns | 882 ns | 836 ns | **2.2x faster** |

### Memory

| Library | Peak RSS |
|---------|----------|
| wkb | 5.0 MB |
| wkb-noxkb | 3.9 MB |
| xkbcommon | 5.0 MB |
| xkbcommon-dl | 5.0 MB |

### Binary Size

Sizes for xkbcommon and xkbcommon-dl include the dynamically-linked `libxkbcommon.so`.

| Binary | Size (stripped) |
|--------|----------------|
| wkb (no XKB) | 708 KB |
| wkb (with XKB) | 637 KB |
| xkbcommon | 730 KB |
| xkbcommon-dl | 763 KB |

<!-- BENCHMARK_END -->

## Scope and Limitations

WKB targets the subset of XKB used by Wayland clients and compositors.
Geometry descriptions and other X11-only features are intentionally out of
scope.

## License

MIT
