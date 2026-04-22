use std::{collections::BTreeMap, fmt};

// Max modifier slots — keymaps typically have 10-20 modifiers
const MAX_MOD_SLOTS: usize = 32;

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
pub const BACKSPACE: u32 = 14;
pub const TAB: u32 = 15;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
                    // if *latched {
                    //     // Already latched - pressing again deactivates (toggle off)
                    //     *pressed = false;
                    //     *latched = false;
                    // } else {
                    //     // Not latched - activate it
                    //     *pressed = true;
                    //     *latched = true;
                    // }
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

    pub fn pressed(&self) -> &bool {
        match self {
            ModKind::Pressed {
                pressed,
                mod_type: _,
            } => pressed,
            ModKind::Lock {
                pressed,
                locked: _,
                mod_type: _,
            } => pressed,
            ModKind::Latch {
                pressed,
                latched: _,
                mod_type: _,
            } => pressed,
            ModKind::None => &false,
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
    entries: Vec<(u32, Modifier)>,
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (code, modifier) in &self.entries {
            write!(f, "code {}: ", code)?;
            match modifier {
                Modifier::Single(mod_kind) => {
                    write!(f, "{:?}", mod_kind)?;
                }
                Modifier::Leveled(map) => {
                    writeln!(f, "[")?;
                    for (index, mod_kind) in map {
                        writeln!(f, "   index {}: {:?}, ", index, mod_kind)?;
                    }
                    write!(f, "]")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
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

    pub fn insert(&mut self, evdev_code: u32, mod_kind: ModKind, level: u8) {
        if let Some((_, existing)) = self.entries.iter_mut().find(|(c, _)| *c == evdev_code) {
            match existing {
                Modifier::Single(ex_kind) => {
                    if level == 0 {
                        *ex_kind = mod_kind;
                    } else {
                        let mut map = BTreeMap::new();
                        map.insert(0, ex_kind.clone());
                        map.insert(level, mod_kind);
                        *existing = Modifier::Leveled(map);
                    }
                }
                Modifier::Leveled(map) => {
                    map.insert(level, mod_kind);
                }
            }
        } else {
            if level == 0 {
                self.entries.push((evdev_code, Modifier::Single(mod_kind)));
            } else {
                let mut map = BTreeMap::new();
                map.insert(level, mod_kind);
                self.entries.push((evdev_code, Modifier::Leveled(map)));
            }
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

    /// Compute level2/3/5 bits in a single scan.
    /// Returns (level2, level3, level5).
    #[inline]
    pub fn level_bits(&self) -> (bool, bool, bool) {
        let mut l2 = false;
        let mut l3 = false;
        let mut l5 = false;

        for (_, modifier) in &self.entries {
            match modifier {
                Modifier::Single(mk) => {
                    Self::check_mod_kind_levels(mk, &mut l2, &mut l3, &mut l5);
                }
                Modifier::Leveled(map) => {
                    for mk in map.values() {
                        Self::check_mod_kind_levels(mk, &mut l2, &mut l3, &mut l5);
                    }
                }
            }
        }
        (l2, l3, l5)
    }

    #[inline(always)]
    fn check_mod_kind_levels(mk: &ModKind, l2: &mut bool, l3: &mut bool, l5: &mut bool) {
        match mk {
            ModKind::Pressed {
                pressed: true,
                mod_type,
            } => match mod_type {
                ModType::Level2 => *l2 = true,
                ModType::Level3 => *l3 = true,
                ModType::Level5 => *l5 = true,
                _ => {}
            },
            ModKind::Lock {
                locked, mod_type, ..
            } if *locked > 0 => match mod_type {
                ModType::Level2 => *l2 = true,
                ModType::Level3 => *l3 = true,
                ModType::Level5 => *l5 = true,
                _ => {}
            },
            ModKind::Latch {
                latched: true,
                mod_type,
                ..
            } => match mod_type {
                ModType::Level2 => *l2 = true,
                ModType::Level3 => *l3 = true,
                ModType::Level5 => *l5 = true,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn level5(&self) -> bool {
        self.active_mod_type(ModType::Level5)
    }

    pub fn level3(&self) -> bool {
        self.active_mod_type(ModType::Level3)
    }

    pub fn level2(&self) -> bool {
        self.active_mod_type(ModType::Level2)
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

    pub fn pressed(&self, evdev_code: u32) -> bool {
        self.get(evdev_code).is_some_and(|modifier| match modifier {
            Modifier::Single(mod_kind) => *mod_kind.pressed(),
            Modifier::Leveled(map) => map.values().any(|mod_kind| *mod_kind.pressed()),
        })
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

    pub fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let (l2, l3, l5) = self.level_bits();
        let level = level_index(l5, l3, l2) as u8;
        if let Some((_, modifier)) = self.entries.iter_mut().find(|(c, _)| *c == evdev_code) {
            match modifier {
                Modifier::Single(mod_kind) => {
                    mod_kind.update(key_direction);
                }
                Modifier::Leveled(map) => {
                    if let Some(mod_kind) = map.get_mut(&level) {
                        mod_kind.update(key_direction);
                    } else if let Some(mod_kind) = map.get_mut(&0) {
                        mod_kind.update(key_direction);
                    } else {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

pub fn level_index(level5: bool, level3: bool, level2: bool) -> usize {
    match (level5, level3, level2) {
        (true, true, true) => 7,
        (true, true, false) => 6,
        (true, false, true) => 5,
        (true, false, false) => 4,
        (false, true, true) => 3,
        (false, true, false) => 2,
        (false, false, true) => 1,
        (false, false, false) => 0,
    }
}
