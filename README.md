# WKB(Wayland Keyboard)

This is a start to a rewrite of xkb/xkbcommon for wayland.

Goals: 
- A complete replacement for the used parts of xkb. Meaning just evdev and no geometry support etc.
- Creating a new more intuitive keyboard format that is serializable(TOML, JSON, etc)

Todo list(tentative):
- [x] Keymap
- [x] Keymap functions 
- [ ] Compositing
- [x] Testing with xkb 
- [x] Repeat keys
- [x] Level keys
- [x] Modifier support
- [ ] Fuzzing of some kind?
- [ ] Caps Lock support
- [ ] Num Lock support
- [ ] Scroll Lock support

Performance improvements(?):
- [ ] use phf instead of static hashmaps
