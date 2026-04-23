# wkb — Wayland Keyboard

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
*Benchmarks are generated automatically by CI. Run `cargo bench` locally or push to `main` to update.*
<!-- BENCHMARK_END -->

## Scope and Limitations

WKB targets the subset of XKB used by Wayland clients and compositors.
Geometry descriptions and other X11-only features are intentionally out of
scope. A future native keyboard format (TOML/RON) is planned but not yet
available.

## License

MIT
