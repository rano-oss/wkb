# WKB(Wayland Keyboard)

This is a start to a rewrite of xkb/xkbcommon for wayland.

Goals: 
- A complete replacement for the used parts of xkb. Meaning just evdev and no geometry support etc.
- Creating a new more intuitive keyboard format that is serializable(TOML, RON, etc)

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
- [ ] Combined modifiers tests, edge case tests for all modifiers
- [ ] Try reaching edge case parity for exisisting tests
- [ ] Handle cases where no keys should be returned
- [x] Compare Modifiers State between xkb and wkb test
- [ ] Keymap handling/storage
- [x] Composing
  - [x] Compose Key
  - [x] Compose Automagic
  - [x] Tests
- [x] Add wayland specific functions
- [x] Lightbulbs
- [ ] New keyboard format
- [ ] Benchmarks
- [ ] Check if remapping is needed

Performance improvements(?):
- [ ] use phf instead of static hashmaps
