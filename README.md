# WKB(Wayland Keyboard)

This is a start to a rewrite of xkb/xkbcommon for wayland.

Goals: 
- A complete replacement for the used parts of xkb. Meaning just evdev and no geometry support etc.
- Creating a new more intuitive keyboard format that is serializable(TOML, JSON, etc)

Todo list(tentative):
- [x] Keymap
- [x] Keymap functions 
- [x] Level keys
- [x] Repeat keys
- [x] Modifier support
- [x] Level keys tests
- [x] State keys tests
- [ ] Num_lock tests
- [ ] Caps_lock tests
- [x] Composing
  - [x] Compose Key
  - [x] Compose Automagic
  - [x] Tests
- [x] Add wayland specific functions
- [x] Lightbulbs
- [ ] New keyboard format

Performance improvements(?):
- [ ] use phf instead of static hashmaps
