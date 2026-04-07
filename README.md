# WKB(Wayland Keyboard)

This is a start to a rewrite of xkb/xkbcommon for wayland.

Goals: 
- A complete replacement for the used parts of xkb. Meaning just evdev and no geometry support etc.
- Creating a new more intuitive keyboard format that is serializable(TOML, RON, etc)

## Features

### XKB Keymap Import
WKB can parse XKB keymap strings, enabling interoperability with existing Wayland compositors and clients:

```rust
use wkb::WKB;

// Receive keymap from Wayland compositor
let keymap_string = "xkb_keymap { ... }";
let wkb = WKB::new_from_string(keymap_string.to_string());

// Use normally
let char = wkb.utf8(keycode);
```

This allows WKB to be a drop-in replacement for xkbcommon in Wayland clients today.

Todo list(tentative):
- [x] Keymap
- [x] Keymap functions 
- [x] Level keys
- [x] Repeat keys
- [x] Modifier support
- [x] Level keys tests
- [x] State keys tests
- [x] Num_lock tests
- [x] Caps_lock tests
- [x] Combined modifiers tests, edge case tests for all modifiers
- [x] Layout comparison test, light test, etc
- [x] Try reaching edge case parity for exisisting tests
- [x] Handle cases where no keys should be returned
- [x] Test cases where no keys should be returned
- [x] Compare Modifiers State between xkb and wkb test
- [x] XKB keymap string import (`new_from_string()`)
- [ ] New WKB keyboard format (TOML, RON, etc)
- [ ] WKB keymap export (for native format)
- [x] Composing
  - [x] Compose Key
  - [x] Compose Automagic
  - [x] Tests
- [x] Add wayland specific functions
- [x] Lightbulbs
- [ ] Benchmarks
