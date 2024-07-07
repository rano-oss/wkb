# WKB(Wayland Keyboard)

This is a start to a rewrite of xkb/xkbcommon for wayland.

Goals: 
- A complete replacement for the used parts of xkb. Meaning just evdev and no geometry support etc.
- Creating a new more intuitive keyboard format that is serializable(TOML, JSON, etc)

Todo list(tentative):
- [x] Keymap
- [x] Keymap functions 
- [ ] Compositing
- [ ] fuzzing with xkb 
- [ ] use phf instead of static hashmaps
- [x] Repeat keys
