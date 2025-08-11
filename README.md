# WKB(Wayland Keyboard)

This is a start to a rewrite of xkb/xkbcommon for wayland.

Goals: 
- A complete replacement for the used parts of xkb. Meaning just evdev and no geometry support etc.
- Creating a new more intuitive keyboard format that is serializable(TOML, JSON, etc)

Todo list(tentative):
- [x] Keymap
- [x] Keymap functions 
- [ ] Composing
- [x] Testing with xkb 
- [x] Repeat keys
- [x] Level keys
- [x] Modifier support
- [ ] Fuzzing of some kind?
- [x] Modifier support
- [x] Level keys tests
- [x] State keys tests
- [ ] Num_lock, Caps_lock tests
- [ ] Add wayland specific functions

Performance improvements(?):
- [ ] use phf instead of static hashmaps
