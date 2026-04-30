use std::collections::BTreeMap;

const MAX_MOD_SLOTS: usize = 32;

pub(crate) const MOD_SHIFT: u32 = 1;
pub(crate) const MOD_CAPS_LOCK: u32 = 2;
pub(crate) const MOD_CTRL: u32 = 4;
pub(crate) const MOD_ALT: u32 = 8;
pub(crate) const MOD_NUM_LOCK: u32 = 16;
pub(crate) const MOD_LOGO: u32 = 64;
pub(crate) const MOD_ALTGR: u32 = 128;

// State bitfield constants
const STATE_NONE: u8 = 1;
const STATE_LEVEL2: u8 = 2;
const STATE_LEVEL3: u8 = 4;
const STATE_LEVEL5: u8 = 8;
const STATE_COMPOSE: u8 = 16;
const STATE_CAPS_LOCKED: u8 = 32;
const STATE_NUM_LOCKED: u8 = 64;

/// LED bitmask for Num Lock (bit 0).
/// LED indicator state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LedState {
    pub num_lock: bool,
    pub caps_lock: bool,
    pub scroll_lock: bool,
}

/// Raw modifier bitmasks for the Wayland `wl_keyboard.modifiers` protocol event.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RawModifiers {
    /// Depressed modifiers bitmask (keys physically held down).
    pub depressed: u32,
    /// Latched modifiers bitmask (sticky, cleared on next keypress).
    pub latched: u32,
    /// Locked modifiers bitmask (toggled, e.g. Caps Lock).
    pub locked: u32,
    /// Active keyboard layout index.
    pub layout: u32,
}

pub(crate) const MODIFIER_MAPPING: [(u32, u32); 9] = [
    (LEFT_SHIFT, MOD_SHIFT),
    (RIGHT_SHIFT, MOD_SHIFT),
    (CAPS_LOCK, MOD_CAPS_LOCK),
    (LEFT_CTRL, MOD_CTRL),
    (RIGHT_CTRL, MOD_CTRL),
    (ALT, MOD_ALT),
    (NUM_LOCK, MOD_NUM_LOCK),
    (LOGO, MOD_LOGO),
    (ALTGR, MOD_ALTGR),
];

// Key constants
pub const LEFT_CTRL: u32 = 29;
pub const LEFT_SHIFT: u32 = 42;
pub const RIGHT_SHIFT: u32 = 54;
pub const RIGHT_CTRL: u32 = 97;
pub const ALT: u32 = 56;
pub const ALTGR: u32 = 100;
pub const LOGO: u32 = 125;
pub const CAPS_LOCK: u32 = 58;
pub const NUM_LOCK: u32 = 69;
pub const SCROLL_LOCK: u32 = 70;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ModType {
    None,
    Level2,
    Level3,
    Level5,
    Compose,
    Caps,
    Num,
    Scroll,
}

#[derive(Debug, Clone)]
pub enum ModKind {
    Pressed {
        pressed: bool,
        mod_type: ModType,
    },
    Lock {
        pressed: bool,
        locked: u8,
        mod_type: ModType,
    },
    Latch {
        pressed: bool,
        latched: bool,
        mod_type: ModType,
    },
    None,
}

impl ModKind {
    pub fn update(&mut self, key_direction: KeyDirection) {
        match self {
            ModKind::Pressed {
                ref mut pressed,
                mod_type: _,
            } => match key_direction {
                KeyDirection::Down => *pressed = true,
                KeyDirection::Up => *pressed = false,
            },
            ModKind::Lock {
                ref mut pressed,
                ref mut locked,
                mod_type: _,
            } => match key_direction {
                KeyDirection::Down => {
                    *pressed = true;
                    if *locked == 0 {
                        *locked = 2;
                    }
                }
                KeyDirection::Up => {
                    *pressed = false;
                    if *locked != 0 {
                        *locked -= 1;
                    }
                }
            },
            ModKind::Latch {
                ref mut pressed,
                ref mut latched,
                mod_type: _,
            } => match key_direction {
                KeyDirection::Down => {
                    *pressed = true;
                    *latched = !*latched;
                }
                KeyDirection::Up => {
                    *pressed = false;
                }
            },
            ModKind::None => {}
        }
    }

    fn unlatch(&mut self) {
        if let ModKind::Latch {
            pressed: _,
            latched,
            mod_type: _,
        } = self
        {
            *latched = false
        }
    }

    pub fn locked(&self) -> bool {
        matches!(self, ModKind::Lock { locked, .. } if *locked > 0)
    }

    pub(crate) fn has_mod_type(&self, mod_type: ModType) -> bool {
        match self {
            ModKind::Pressed { mod_type: m, .. }
            | ModKind::Lock { mod_type: m, .. }
            | ModKind::Latch { mod_type: m, .. } => *m == mod_type,
            ModKind::None => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(ModKind),
    Leveled(BTreeMap<u8, ModKind>),
}

impl Modifier {
    fn for_each(&self, mut f: impl FnMut(&ModKind)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values().for_each(|mk| f(mk)),
        }
    }

    fn for_each_mut(&mut self, mut f: impl FnMut(&mut ModKind)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values_mut().for_each(|mk| f(mk)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    /// Flat array of (evdev_code, Modifier) pairs. Typically 10-20 entries.
    pub(crate) entries: Vec<(u32, Modifier)>,
    /// Active modifier state: bit0=none, bit1=level2, bit2=level3, bit3=level5, bit4=compose, bit5=caps_locked, bit6=num_locked
    state: u8,
}

impl Default for Modifiers {
    fn default() -> Self {
        let entries = vec![
            (
                LEFT_CTRL,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::None,
                }),
            ),
            (
                RIGHT_CTRL,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::None,
                }),
            ),
            (
                LEFT_SHIFT,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::Level2,
                }),
            ),
            (
                RIGHT_SHIFT,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::Level2,
                }),
            ),
            (
                ALT,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::None,
                }),
            ),
            (
                ALTGR,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::None,
                }),
            ),
            (
                LOGO,
                Modifier::Single(ModKind::Pressed {
                    pressed: false,
                    mod_type: ModType::None,
                }),
            ),
            (
                CAPS_LOCK,
                Modifier::Single(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::Caps,
                }),
            ),
            (
                NUM_LOCK,
                Modifier::Single(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::Num,
                }),
            ),
            (
                SCROLL_LOCK,
                Modifier::Single(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::Scroll,
                }),
            ),
        ];
        Self { entries, state: 0 }
    }
}

impl Modifiers {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_MOD_SLOTS),
            state: 0,
        }
    }

    /// Get a reference to a modifier by evdev code.
    #[inline]
    pub fn get(&self, evdev_code: u32) -> Option<&Modifier> {
        self.entries
            .iter()
            .find(|(c, _)| *c == evdev_code)
            .map(|(_, m)| m)
    }

    /// Get a mutable reference to a modifier by evdev code.
    #[inline]
    pub fn get_mut(&mut self, evdev_code: u32) -> Option<&mut Modifier> {
        self.entries
            .iter_mut()
            .find(|(c, _)| *c == evdev_code)
            .map(|(_, m)| m)
    }

    /// Iterate over all (evdev_code, modifier) pairs.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Modifier)> {
        self.entries.iter().map(|(c, m)| (c, m))
    }

    /// Insert or replace a modifier for the given evdev code.
    pub fn set_modifier(&mut self, evdev_code: u32, modifier: Modifier) {
        if let Some((_, existing)) = self.entries.iter_mut().find(|(c, _)| *c == evdev_code) {
            *existing = modifier;
        } else {
            self.entries.push((evdev_code, modifier));
        }
    }

    pub fn active_mod_type(&self, mod_type: ModType) -> bool {
        match mod_type {
            ModType::None => self.state & STATE_NONE != 0,
            ModType::Level2 => self.state & STATE_LEVEL2 != 0,
            ModType::Level3 => self.state & STATE_LEVEL3 != 0,
            ModType::Level5 => self.state & STATE_LEVEL5 != 0,
            ModType::Compose => self.state & STATE_COMPOSE != 0,
            _ => false,
        }
    }

    /// Check for active None-type modifier AND compute level2/3/5 in a single scan.
    /// Returns (has_active_none, level2, level3, level5).
    #[inline]
    pub fn active_none_and_levels(&self) -> (bool, bool, bool, bool) {
        (
            self.state & STATE_NONE != 0,
            self.state & STATE_LEVEL2 != 0,
            self.state & STATE_LEVEL3 != 0,
            self.state & STATE_LEVEL5 != 0,
        )
    }

    /// Return true if Caps Lock is locked (O(1) from state bitfield).
    #[inline]
    pub fn caps_locked(&self) -> bool {
        self.state & STATE_CAPS_LOCKED != 0
    }

    /// Return true if Num Lock is locked (O(1) from state bitfield).
    #[inline]
    pub fn num_locked(&self) -> bool {
        self.state & STATE_NUM_LOCKED != 0
    }

    fn refresh_state(&mut self) {
        let mut s = 0u8;
        for (_, modifier) in &self.entries {
            modifier.for_each(|mk| Self::accumulate_state(mk, &mut s));
        }
        self.state = s;
    }

    #[inline(always)]
    fn accumulate_state(mk: &ModKind, state: &mut u8) {
        let mod_type = match mk {
            ModKind::Pressed {
                pressed: true,
                mod_type,
            } => mod_type,
            ModKind::Lock {
                locked, mod_type, ..
            } if *locked > 0 => mod_type,
            ModKind::Latch {
                latched: true,
                mod_type,
                ..
            } => mod_type,
            _ => return,
        };
        match mod_type {
            ModType::None => *state |= STATE_NONE,
            ModType::Level2 => *state |= STATE_LEVEL2,
            ModType::Level3 => *state |= STATE_LEVEL3,
            ModType::Level5 => *state |= STATE_LEVEL5,
            ModType::Compose => *state |= STATE_COMPOSE,
            ModType::Caps => *state |= STATE_CAPS_LOCKED,
            ModType::Num => *state |= STATE_NUM_LOCKED,
            ModType::Scroll => {}
        }
    }

    pub fn unlatch(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|(_, modifier)| modifier.for_each_mut(|mk| mk.unlatch()));
        self.refresh_state();
    }

    pub fn locked_with_type(&self, evdev_code: u32, mod_type: ModType) -> bool {
        self.get(evdev_code).is_some_and(|modifier| {
            let mut found = false;
            modifier.for_each(|mk| found |= mk.locked() && mk.has_mod_type(mod_type));
            found
        })
    }

    #[inline]
    pub fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let pos = match self.entries.iter().position(|(c, _)| *c == evdev_code) {
            Some(p) => p,
            None => return false,
        };
        let is_leveled = matches!(&self.entries[pos].1, Modifier::Leveled(_));
        if is_leveled {
            let (_, l2, l3, l5) = self.active_none_and_levels();
            let level = level_index(l5, l3, l2) as u8;
            if let Modifier::Leveled(map) = &mut self.entries[pos].1 {
                if let Some(mod_kind) = map.get_mut(&level) {
                    mod_kind.update(key_direction);
                } else if let Some(mod_kind) = map.get_mut(&0) {
                    mod_kind.update(key_direction);
                } else {
                    return false;
                }
            }
        } else if let Modifier::Single(mod_kind) = &mut self.entries[pos].1 {
            mod_kind.update(key_direction);
        }
        self.refresh_state();
        true
    }

    pub fn state(&self, layout_index: usize) -> RawModifiers {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let layout = layout_index as u32;
        for (code, bit) in MODIFIER_MAPPING {
            if let Some(modifier) = self.get(code) {
                modifier.for_each(|mk| match mk {
                    ModKind::Pressed { pressed: true, .. } => depressed |= bit,
                    ModKind::Lock {
                        pressed: p,
                        locked: l,
                        ..
                    } => {
                        if *p {
                            depressed |= bit;
                        }
                        if *l > 0 {
                            locked |= bit;
                        }
                    }
                    ModKind::Latch {
                        pressed: p,
                        latched: lt,
                        ..
                    } => {
                        if *p {
                            depressed |= bit;
                        }
                        if *lt {
                            latched |= bit;
                        }
                    }
                    _ => {}
                });
            }
        }
        RawModifiers {
            depressed,
            latched,
            locked,
            layout,
        }
    }

    pub(crate) fn update(&mut self, depressed: u32, latched: u32, locked: u32) {
        for (code, bit) in MODIFIER_MAPPING {
            let is_depressed = (depressed & bit) != 0;
            let is_locked = (locked & bit) != 0;
            let is_latched = (latched & bit) != 0;

            if let Some(m) = self.get_mut(code) {
                m.for_each_mut(|mk| match mk {
                    ModKind::Pressed { pressed, .. } => *pressed = is_depressed,
                    ModKind::Lock {
                        pressed, locked, ..
                    } => {
                        *pressed = is_depressed;
                        *locked = if is_locked { 1 } else { 0 };
                    }
                    ModKind::Latch {
                        pressed, latched, ..
                    } => {
                        *pressed = is_depressed;
                        *latched = is_latched;
                    }
                    ModKind::None => {}
                });
            }
        }
        self.refresh_state();
    }

    pub(crate) fn leds_state(&self) -> LedState {
        LedState {
            num_lock: self.locked_with_type(NUM_LOCK, ModType::Num),
            caps_lock: self.locked_with_type(CAPS_LOCK, ModType::Caps),
            scroll_lock: self.locked_with_type(SCROLL_LOCK, ModType::Scroll),
        }
    }
}

#[inline(always)]
pub fn level_index(level5: bool, level3: bool, level2: bool) -> usize {
    ((level5 as usize) << 2) | ((level3 as usize) << 1) | (level2 as usize)
}
