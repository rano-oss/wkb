use std::collections::BTreeMap;

// Max modifier slots — keymaps typically have 10-20 modifiers
const MAX_MOD_SLOTS: usize = 32;

// ── Public modifier bit constants (match standard XKB evdev indices) ──

/// Shift modifier bitmask (XKB mod index 0).
pub(crate) const MOD_SHIFT: u32 = 1;
/// Caps Lock modifier bitmask (XKB mod index 1).
pub(crate) const MOD_CAPS_LOCK: u32 = 2;
/// Control modifier bitmask (XKB mod index 2).
pub(crate) const MOD_CTRL: u32 = 4;
/// Alt/Mod1 modifier bitmask (XKB mod index 3).
pub(crate) const MOD_ALT: u32 = 8;
/// Num Lock/Mod2 modifier bitmask (XKB mod index 4).
pub(crate) const MOD_NUM_LOCK: u32 = 16;
/// Mod3/ISO Level5 Shift modifier bitmask (XKB mod index 5).
// pub(crate) const _MOD_ISO_LEVEL5_SHIFT: u32 = 32;
/// Logo/Mod4 modifier bitmask (XKB mod index 6).
pub(crate) const MOD_LOGO: u32 = 64;
/// AltGr modifier bitmask (XKB mod index 7).
pub(crate) const MOD_ALTGR: u32 = 128;

/// LED bitmask for Num Lock (bit 0).
/// LED indicator state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LedState {
    pub num_lock: bool,
    pub caps_lock: bool,
    pub scroll_lock: bool,
}

// ── Modifier state ──

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
        match self {
            ModKind::Pressed {
                pressed: _,
                mod_type: _,
            } => false,
            ModKind::Lock {
                pressed: _,
                locked,
                mod_type: _,
            } => locked > &0,
            ModKind::Latch {
                pressed: _,
                latched: _,
                mod_type: _,
            } => false,
            ModKind::None => false,
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            ModKind::Pressed {
                pressed,
                mod_type: _,
            } => *pressed,
            ModKind::Lock {
                pressed: _,
                locked,
                mod_type: _,
            } => locked > &0,
            ModKind::Latch {
                pressed: _,
                latched,
                mod_type: _,
            } => *latched,
            ModKind::None => false,
        }
    }

    pub(crate) fn get_modkind_from_modtype(&self, mod_type: ModType) -> Option<ModKind> {
        match self {
            ModKind::Pressed { mod_type: m_t, .. }
            | ModKind::Lock { mod_type: m_t, .. }
            | ModKind::Latch { mod_type: m_t, .. } => {
                if *m_t == mod_type {
                    Some(self.clone())
                } else {
                    None
                }
            }
            ModKind::None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(ModKind),
    Leveled(BTreeMap<u8, ModKind>),
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    /// Flat array of (evdev_code, Modifier) pairs. Typically 10-20 entries.
    pub(crate) entries: Vec<(u32, Modifier)>,
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
        Self { entries }
    }
}

impl Modifiers {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_MOD_SLOTS),
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
        self.entries.iter().any(|(_, modifier)| match modifier {
            Modifier::Single(mod_kind) => {
                if let Some(mk) = mod_kind.get_modkind_from_modtype(mod_type) {
                    mk.is_active()
                } else {
                    false
                }
            }
            Modifier::Leveled(map) => map.values().any(|mod_kind| {
                if let Some(mk) = mod_kind.get_modkind_from_modtype(mod_type) {
                    mk.is_active()
                } else {
                    false
                }
            }),
        })
    }

    /// Check for active None-type modifier AND compute level2/3/5 in a single scan.
    /// Returns (has_active_none, level2, level3, level5).
    #[inline]
    pub fn active_none_and_levels(&self) -> (bool, bool, bool, bool) {
        let mut none_active = false;
        let mut l2 = false;
        let mut l3 = false;
        let mut l5 = false;

        for (_, modifier) in &self.entries {
            match modifier {
                Modifier::Single(mk) => {
                    Self::check_mod_kind(mk, &mut none_active, &mut l2, &mut l3, &mut l5);
                }
                Modifier::Leveled(map) => {
                    for mk in map.values() {
                        Self::check_mod_kind(mk, &mut none_active, &mut l2, &mut l3, &mut l5);
                    }
                }
            }
        }
        (none_active, l2, l3, l5)
    }

    #[inline(always)]
    fn check_mod_kind(
        mk: &ModKind,
        none_active: &mut bool,
        l2: &mut bool,
        l3: &mut bool,
        l5: &mut bool,
    ) {
        match mk {
            ModKind::Pressed { pressed, mod_type } if *pressed => match mod_type {
                ModType::None => *none_active = true,
                ModType::Level2 => *l2 = true,
                ModType::Level3 => *l3 = true,
                ModType::Level5 => *l5 = true,
                _ => {}
            },
            ModKind::Lock {
                locked, mod_type, ..
            } if *locked > 0 => match mod_type {
                ModType::None => *none_active = true,
                ModType::Level2 => *l2 = true,
                ModType::Level3 => *l3 = true,
                ModType::Level5 => *l5 = true,
                _ => {}
            },
            ModKind::Latch {
                latched, mod_type, ..
            } if *latched => match mod_type {
                ModType::None => *none_active = true,
                ModType::Level2 => *l2 = true,
                ModType::Level3 => *l3 = true,
                ModType::Level5 => *l5 = true,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn unlatch(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|(_, modifier)| match modifier {
                Modifier::Single(mod_kind) => {
                    mod_kind.unlatch();
                }
                Modifier::Leveled(map) => {
                    map.values_mut().for_each(|mod_kind| {
                        mod_kind.unlatch();
                    });
                }
            });
    }

    pub fn locked(&self, evdev_code: u32) -> bool {
        self.get(evdev_code).is_some_and(|modifier| match modifier {
            Modifier::Single(mod_kind) => mod_kind.locked(),
            Modifier::Leveled(map) => map.values().any(|mod_kind| mod_kind.locked()),
        })
    }

    pub fn locked_with_type(&self, evdev_code: u32, mod_type: ModType) -> bool {
        self.get(evdev_code).is_some_and(|modifier| match modifier {
            Modifier::Single(mod_kind) => {
                mod_kind.locked() && mod_kind.get_modkind_from_modtype(mod_type).is_some()
            }
            Modifier::Leveled(map) => map.values().any(|mod_kind| {
                mod_kind.locked() && mod_kind.get_modkind_from_modtype(mod_type).is_some()
            }),
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
        true
    }

    pub fn state(&self, layout_index: usize) -> RawModifiers {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let layout = layout_index as u32;
        for (code, bit) in MODIFIER_MAPPING {
            if let Some(modifier) = self.get(code) {
                let mod_kinds: &[&ModKind] = &match modifier {
                    Modifier::Single(mk) => vec![mk],
                    Modifier::Leveled(map) => map.values().collect(),
                };
                for mk in mod_kinds {
                    match mk {
                        ModKind::Pressed { pressed: true, .. } => depressed |= bit,
                        ModKind::Lock {
                            pressed, locked: l, ..
                        } => {
                            if *pressed {
                                depressed |= bit;
                            }
                            if *l > 0 {
                                locked |= bit;
                            }
                        }
                        ModKind::Latch {
                            pressed,
                            latched: is_latched,
                            ..
                        } => {
                            if *pressed {
                                depressed |= bit;
                            }
                            if *is_latched {
                                latched |= bit;
                            }
                        }
                        _ => {}
                    }
                }
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
                let mod_kinds: Vec<&mut ModKind> = match m {
                    Modifier::Single(mk) => vec![mk],
                    Modifier::Leveled(map) => map.values_mut().collect(),
                };
                for mk in mod_kinds {
                    match mk {
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
                        _ => {}
                    }
                }
            }
        }
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
