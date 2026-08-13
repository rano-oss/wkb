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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModKind {
    Press {
        pressed: bool,
    },
    Lock {
        pressed: bool,
        locked: u8,
    },
    LockOnRelease {
        pressed: bool,
        locked: bool,
    },
    UnlockOnPress {
        pressed: bool,
        locked: bool,
    },
    LockOnReleaseUnlockOnPress {
        pressed: bool,
        locked: bool,
        lock: bool
    },
    TapLock {
        pressed: bool,
        locked: bool,
        tapped: bool,
    },
    Latch {
        pressed: bool,
        latched: bool,
    },
    LatchOnPress {
        pressed: bool,
        latched: bool,
    },
    LatchToLockOnPress {
        pressed: bool,
        latched: bool,
        locked: bool,
    },
    LatchToLockOnRelease {
        pressed: bool,
        latched: bool,
        locked: bool,
    },
}

impl ModKind {
    pub(crate) fn update(&mut self, key_direction: KeyDirection) {
        match self {
            ModKind::Press { pressed } => *pressed = key_direction == KeyDirection::Down,
            ModKind::Lock { pressed, locked } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                        if *locked == 0 {
                            *locked = 2;
                        }
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                        if *locked != 0 {
                            *locked -= 1;
                        }
                    },
                }
            },
            ModKind::LockOnRelease { pressed, locked } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                        *locked = !*locked;
                    },
                }
            },
            ModKind::UnlockOnPress { pressed, locked } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                        *locked = !*locked;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                    },
                }
            },
            ModKind::LockOnReleaseUnlockOnPress { pressed, locked, lock } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                        if *locked {
                            *locked = false;
                        }
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                        if !*lock {
                            *locked = true;
                            *lock = true;
                        } else {
                            *lock = false;
                        }
                    },
                }
            },
            ModKind::TapLock { pressed, locked, tapped } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                        *tapped = true;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                        if *tapped {
                            *locked = !*locked;
                        }
                        *tapped = false;
                    },
                }
            },
            ModKind::Latch { pressed, latched } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                        *latched = !*latched;
                    },
                }
            },
            ModKind::LatchOnPress { pressed, latched } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                        *latched = !*latched;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                    },
                }
            },
            ModKind::LatchToLockOnPress { pressed, latched, locked } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                        if *latched {
                            *locked = !*locked;
                        }
                        *latched = !*latched;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                    },
                }
            },
            ModKind::LatchToLockOnRelease { pressed, latched, locked } => {
                match key_direction {
                    KeyDirection::Down => {
                        *pressed = true;
                    },
                    KeyDirection::Up => {
                        *pressed = false;
                        if *latched {
                            *locked = !*locked;
                        }
                        *latched = !*latched;
                    },
                }
            },
        }
    }

    pub(crate) fn update_from_state(&mut self, s_pressed: bool, s_locked: bool, s_latched: bool) {
        match self {
            ModKind::Press { pressed } => *pressed = s_pressed,
            ModKind::Lock { pressed, locked } => {
                *pressed = s_pressed;
                *locked = s_locked as u8 + s_pressed as u8;
            },
            ModKind::LockOnRelease { pressed, locked } => {
                *pressed = s_pressed;
                *locked = s_locked;
            },
            ModKind::UnlockOnPress { pressed, locked } => {
                *pressed = s_pressed;
                *locked = s_locked;
            },
            ModKind::LockOnReleaseUnlockOnPress { pressed, locked, lock } => {
                *pressed = s_pressed;
                *locked = s_locked;
                *lock = s_locked;
            },
            ModKind::TapLock { pressed, locked, tapped } => {
                *pressed = s_pressed;
                *locked = s_locked;
                *tapped = s_pressed;
            },
            ModKind::Latch { pressed, latched } => {
                *pressed = s_pressed;
                *latched = s_latched;
            },
            ModKind::LatchOnPress { pressed, latched } => {
                *pressed = s_pressed;
                *latched = s_latched;
            },
            ModKind::LatchToLockOnPress { pressed, latched, locked } => {
                *pressed = s_pressed;
                *latched = s_latched;
                *locked = s_locked;
            },
            ModKind::LatchToLockOnRelease { pressed, latched, locked } => {
                *pressed = s_pressed;
                *latched = s_latched;
                *locked = s_locked;
            },
        }
    }
    
    pub(crate) fn pressed(&self) -> bool {
        match self {
            ModKind::Press { pressed } |
            ModKind::Lock { pressed, .. } |
            ModKind::LockOnRelease { pressed, .. } |
            ModKind::UnlockOnPress { pressed, .. } |
            ModKind::LockOnReleaseUnlockOnPress { pressed, .. } |
            ModKind::TapLock { pressed, .. } |
            ModKind::Latch { pressed, .. } |
            ModKind::LatchOnPress { pressed, .. } |
            ModKind::LatchToLockOnPress { pressed, .. } |
            ModKind::LatchToLockOnRelease { pressed, .. } => *pressed,
        }
    }

    pub(crate) fn depressed(&self) -> bool {
        match self {
            Self::UnlockOnPress {
                pressed,
                locked,
            } => *pressed && *locked,
    
            Self::LockOnReleaseUnlockOnPress {
                pressed,
                lock,
                ..
            } => *pressed && *lock,
    
            Self::TapLock { .. } => false,
    
            _ => self.pressed(),
        }
    }
    
    pub(crate) fn locked(&self) -> bool {
        match self {
            ModKind::Lock { locked, .. } => *locked > 0,
            ModKind::LockOnRelease { locked, .. } |
            ModKind::UnlockOnPress { locked, .. } |
            ModKind::LockOnReleaseUnlockOnPress { locked, .. } |
            ModKind::TapLock { locked, .. } |
            ModKind::LatchToLockOnPress { locked, .. } |
            ModKind::LatchToLockOnRelease { locked, .. } => *locked,
            _ => false,
        }
    }

    pub(crate) fn latched(&self) -> bool {
        match self {
            ModKind::Latch { latched, .. } |
            ModKind::LatchOnPress { latched, .. } |
            ModKind::LatchToLockOnPress { latched, .. } |
            ModKind::LatchToLockOnRelease { latched, .. } => *latched,
            _ => false,
        }
    }

    pub(crate) fn unlatch(&mut self) {
        match self {
            Self::Latch { latched, .. } => *latched = false,
            _ => {}
        }
    }

    pub(crate) fn untap(&mut self) {
        match self {
            Self::TapLock { tapped, .. } => {
                if *tapped {
                    *tapped = false;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
pub struct StateModifier {
    pub(crate) mod_type: ModType,
    pub(crate) kind: ModKind,
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(StateModifier),
    Leveled(BTreeMap<u8, StateModifier>),
}

impl Modifier {
    fn update(&mut self, key_direction: KeyDirection, level: u8) {
        match self {
            Modifier::Single(state_modifier) => state_modifier.kind.update(key_direction),
            Modifier::Leveled(levels) => match key_direction {
                KeyDirection::Down => {
                    if let Some(modifier) = levels.get_mut(&level) {
                        modifier.kind.update(KeyDirection::Down);
                    }
                }
                KeyDirection::Up => {
                    for modifier in levels.values_mut() {
                        if modifier.kind.pressed() {
                            modifier.kind.update(KeyDirection::Up);
                        }
                    }
                }
            },
        }
    }

    fn for_each(&self, mut f: impl FnMut(&StateModifier)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values().for_each(f),
        }
    }

    fn for_each_mut(&mut self, mut f: impl FnMut(&mut StateModifier)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values_mut().for_each(f),
        }
    }

    fn level_for(&self, this_mod_type: ModType) -> Option<Option<u8>> {
        match self {
            Modifier::Single(StateModifier { mod_type, .. }) => if mod_type.eq(&this_mod_type) {
                Some(None)
            } else {
                None
            },
            Modifier::Leveled(map) => map.iter().find_map(|(level, StateModifier { mod_type, .. })| {
                if mod_type.eq(&this_mod_type) {
                    Some(Some(*level))
                } else {
                    None
                }
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    /// Flat array of (evdev_code, Modifier) pairs. Typically 10-20 entries.
    pub(crate) entries: Vec<(u32, Modifier)>,
}

impl Default for Modifiers {
    fn default() -> Self {
        let single = |mod_type, kind| Modifier::Single(StateModifier { mod_type, kind });
        let press = ModKind::Press { pressed: false };
        let lock = ModKind::Lock { pressed: false, locked: 0 };
        let entries = vec![
            (LEFT_CTRL, single(ModType::None, press)),
            (RIGHT_CTRL, single(ModType::None, press)),
            (LEFT_SHIFT, single(ModType::Level2, press)),
            (RIGHT_SHIFT, single(ModType::Level2, press)),
            (ALT, single(ModType::None, press)),
            (ALTGR, single(ModType::None, press)),
            (LOGO, single(ModType::None, press)),
            (CAPS_LOCK, single(ModType::Caps, lock)),
            (NUM_LOCK, single(ModType::Num, lock)),
            (SCROLL_LOCK, single(ModType::Scroll, lock)),
        ];
        Self { entries }
    }
}

impl Modifiers {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_MOD_SLOTS),
        }
    }

    /// Get a reference to a modifier by evdev code.
    pub(crate) fn get(&self, evdev_code: u32) -> Option<&Modifier> {
        self.entries
            .iter()
            .find(|(c, _)| *c == evdev_code)
            .map(|(_, m)| m)
    }

    /// Get a mutable reference to a modifier by evdev code.
    pub(crate) fn get_mut(&mut self, evdev_code: u32) -> Option<&mut Modifier> {
        self.entries
            .iter_mut()
            .find(|(c, _)| *c == evdev_code)
            .map(|(_, m)| m)
    }

    /// Iterate over all (evdev_code, modifier) pairs.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&u32, &Modifier)> {
        self.entries.iter().map(|(c, m)| (c, m))
    }

    pub(crate) fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)> {
        self.iter()
            .find_map(|(code, modifier)| modifier.level_for(mod_type).map(|level| (*code, level)))
    }

    /// Insert or replace a modifier for the given evdev code.
    pub(crate) fn set_modifier(&mut self, evdev_code: u32, modifier: Modifier) {
        if let Some((_, existing)) = self.entries.iter_mut().find(|(c, _)| *c == evdev_code) {
            *existing = modifier;
        } else {
            self.entries.push((evdev_code, modifier));
        }
    }

    /// Active modifier state: bit0=none, bit1=level2, bit2=level3, bit3=level5,
    /// bit4=compose, bit5=caps_locked, bit6=num_locked.
    fn state_bits(&self) -> u8 {
        let mut state = 0;
        for (_, modifier) in &self.entries {
            modifier.for_each(|sm| {
                let active = match sm.mod_type {
                    ModType::Caps | ModType::Num | ModType::Scroll => {
                        sm.kind.locked()
                    }
                    _ => {
                        sm.kind.depressed()
                            || sm.kind.latched()
                            || sm.kind.locked()
                    }
                };
                if !active {
                    return;
                }
                state |= match sm.mod_type {
                    ModType::None => STATE_NONE,
                    ModType::Level2 => STATE_LEVEL2,
                    ModType::Level3 => STATE_LEVEL3,
                    ModType::Level5 => STATE_LEVEL5,
                    ModType::Compose => STATE_COMPOSE,
                    ModType::Caps => STATE_CAPS_LOCKED,
                    ModType::Num => STATE_NUM_LOCKED,
                    ModType::Scroll => 0,
                };
            });
        }
    
        state
    }

    pub(crate) fn active_mod_type(&self, mod_type: ModType) -> bool {
        let state = self.state_bits();
        match mod_type {
            ModType::None => state & STATE_NONE != 0,
            ModType::Level2 => state & STATE_LEVEL2 != 0,
            ModType::Level3 => state & STATE_LEVEL3 != 0,
            ModType::Level5 => state & STATE_LEVEL5 != 0,
            ModType::Compose => state & STATE_COMPOSE != 0,
            _ => false,
        }
    }

    /// Check for active None-type modifier AND compute level2/3/5 in a single scan.
    /// Returns (has_active_none, level2, level3, level5).
    pub(crate) fn active_none_and_levels(&self) -> (bool, bool, bool, bool) {
        let state = self.state_bits();
        (
            state & STATE_NONE != 0,
            state & STATE_LEVEL2 != 0,
            state & STATE_LEVEL3 != 0,
            state & STATE_LEVEL5 != 0,
        )
    }

    /// Return true if Caps Lock is locked.
    pub(crate) fn caps_locked(&self) -> bool {
        self.state_bits() & STATE_CAPS_LOCKED != 0
    }

    /// Return true if Num Lock is locked.
    pub(crate) fn num_locked(&self) -> bool {
        self.state_bits() & STATE_NUM_LOCKED != 0
    }

    pub(crate) fn unlatch_except(&mut self, evdev_code: u32) {
        for (code, modifier) in &mut self.entries {
            if *code != evdev_code {
                modifier.for_each_mut(|modifier| {
                    modifier.kind.unlatch();
                    modifier.kind.untap();
                });
            }
        }
    }

    pub(crate) fn locked_with_type(&self, evdev_code: u32, mod_type: ModType) -> bool {
        self.get(evdev_code).is_some_and(|modifier| {
            let mut found = false;
            modifier.for_each(|sm| 
                found |= sm.mod_type == mod_type && sm.kind.locked()
            );
            found
        })
    }

    #[inline]
    pub(crate) fn update_key(
        &mut self,
        evdev_code: u32,
        key_direction: KeyDirection,
    ) -> bool {
        let (_, level2, level3, level5) = self.active_none_and_levels();
        let level = level_index(level5, level3, level2) as u8;
        self.entries.iter_mut().find_map(|(code, modifier)| if *code == evdev_code {
            modifier.update(key_direction, level);
            Some(true)
        } else {
            None
        }).is_some()
    }

    pub(crate) fn state(&self, layout_index: usize) -> RawModifiers {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let layout = layout_index as u32;
        for (code, bit) in MODIFIER_MAPPING {
            if let Some(modifier) = self.get(code) {
                modifier.for_each(|mk| {
                    if mk.kind.depressed() {
                        depressed |= bit;
                    }
                    if mk.kind.locked() {
                        locked |= bit;
                    }
                    if mk.kind.latched() {
                        latched |= bit;
                    }
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
                m.for_each_mut(|mk| mk.kind.update_from_state(is_depressed, is_locked, is_latched));
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
