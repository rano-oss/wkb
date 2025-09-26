use std::{
    backtrace::Backtrace,
    collections::{BTreeMap, HashMap},
    fmt,
};

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
                    *latched = true;
                }
                KeyDirection::Up => {
                    *pressed = false;
                }
            },
            ModKind::None => {}
        }
    }

    fn unlatch(&mut self) {
        match self {
            ModKind::Latch {
                pressed,
                latched,
                mod_type: _,
            } => *latched = *pressed,
            _ => {}
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

    pub fn get_modkind_from_modtype(&self, mod_type: ModType) -> Option<ModKind> {
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
pub struct Modifiers(pub BTreeMap<u32, Modifier>);

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (code, modifier) in &self.0 {
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
        Self(BTreeMap::from([
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
                    mod_type: ModType::None,
                }),
            ),
            (
                NUM_LOCK,
                Modifier::Single(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::None,
                }),
            ),
            (
                SCROLL_LOCK,
                Modifier::Single(ModKind::Lock {
                    pressed: false,
                    locked: 0,
                    mod_type: ModType::None,
                }),
            ),
        ]))
    }
}

impl Modifiers {
    pub fn insert(&mut self, evdev_code: u32, mod_kind: ModKind, level: u8) {
        match self.0.get_mut(&evdev_code) {
            Some(Modifier::Single(existing)) => {
                if level == 0 {
                    *existing = mod_kind.clone();
                } else {
                    let mut map = BTreeMap::new();
                    map.insert(0, existing.clone());
                    map.insert(level, mod_kind);
                    *self.0.get_mut(&evdev_code).unwrap() = Modifier::Leveled(map);
                }
            }
            Some(Modifier::Leveled(map)) => {
                map.insert(level, mod_kind);
            }
            None => {
                if level == 0 {
                    self.0.insert(evdev_code, Modifier::Single(mod_kind));
                } else {
                    let mut map = BTreeMap::new();
                    map.insert(level, mod_kind);
                    self.0.insert(evdev_code, Modifier::Leveled(map));
                }
            }
        }
    }

    pub fn level(&self, mod_type: ModType) -> bool {
        self.0
            .values()
            .find(|modifier| match modifier {
                Modifier::Single(mod_kind) => {
                    if let Some(mod_kind) = mod_kind.get_modkind_from_modtype(mod_type) {
                        mod_kind.is_active()
                    } else {
                        false
                    }
                }
                Modifier::Leveled(map) => map.values().any(|mod_kind| {
                    if let Some(mod_kind) = mod_kind.get_modkind_from_modtype(mod_type) {
                        mod_kind.is_active()
                    } else {
                        false
                    }
                }),
            })
            .is_some()
    }

    pub fn level5(&self) -> bool {
        self.level(ModType::Level5)
    }

    pub fn level3(&self) -> bool {
        self.level(ModType::Level3)
    }

    pub fn level2(&self) -> bool {
        self.level(ModType::Level2)
    }

    pub fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)> {
        self.0.iter().find_map(|(code, modifier)| match modifier {
            Modifier::Single(mod_kind) => {
                if mod_kind.get_modkind_from_modtype(mod_type).is_some() {
                    Some((*code, None))
                } else {
                    None
                }
            }
            Modifier::Leveled(map) => {
                for (level, mod_kind) in map {
                    if mod_kind.get_modkind_from_modtype(mod_type).is_some() {
                        return Some((*code, Some(*level)));
                    }
                }
                None
            }
        })
    }

    pub fn level2_code(&self) -> Option<(u32, Option<u8>)> {
        self.level_code(ModType::Level2)
    }

    pub fn level3_code(&self) -> Option<(u32, Option<u8>)> {
        self.level_code(ModType::Level3)
    }

    pub fn level5_code(&self) -> Option<(u32, Option<u8>)> {
        self.level_code(ModType::Level5)
    }

    pub fn unlatch(&mut self) {
        self.0.values_mut().for_each(|modifier| match modifier {
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
        self.0
            .get(&evdev_code)
            .is_some_and(|modifier| match modifier {
                Modifier::Single(mod_kind) => *mod_kind.pressed(),
                Modifier::Leveled(map) => map.values().any(|mod_kind| *mod_kind.pressed()),
            })
    }

    pub fn locked(&self, evdev_code: u32) -> bool {
        self.0
            .get(&evdev_code)
            .is_some_and(|modifier| match modifier {
                Modifier::Single(mod_kind) => mod_kind.locked(),
                Modifier::Leveled(map) => map.values().any(|mod_kind| mod_kind.locked()),
            })
    }

    pub fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let level = level_index(self.level5(), self.level3(), self.level2()) as u8;
        if let Some(modifier) = self.0.get_mut(&evdev_code) {
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
