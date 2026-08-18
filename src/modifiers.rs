use std::collections::BTreeMap;

const MAX_MOD_SLOTS: usize = 32;

pub(crate) const MOD_SHIFT: u32 = 1 << 0;
pub(crate) const MOD_CAPS_LOCK: u32 = 1 << 1;
pub(crate) const MOD_CTRL: u32 = 1 << 2;
pub(crate) const MOD_ALT: u32 = 1 << 3; // Mod1
pub(crate) const MOD_NUM_LOCK: u32 = 1 << 4; // Mod2
pub(crate) const MOD_SCROLL_LOCK: u32 = 1 << 5;
pub(crate) const MOD_LOGO: u32 = 1 << 6; // Mod4
pub(crate) const MOD_ALTGR: u32 = 1 << 7; // Mod5

pub(crate) const MODIFIER_MAPPING: [(u32, u32); 10] = [
    (LEFT_SHIFT, MOD_SHIFT),
    (RIGHT_SHIFT, MOD_SHIFT),
    (CAPS_LOCK, MOD_CAPS_LOCK),
    (LEFT_CTRL, MOD_CTRL),
    (RIGHT_CTRL, MOD_CTRL),
    (ALT, MOD_ALT),
    (NUM_LOCK, MOD_NUM_LOCK),
    (SCROLL_LOCK, MOD_SCROLL_LOCK),
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RawModifiers {
    pub depressed: u32,
    pub latched: u32,
    pub locked: u32,
    pub layout: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LedState {
    pub num_lock: bool,
    pub caps_lock: bool,
    pub scroll_lock: bool,
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

#[derive(Debug, Clone, Copy)]
pub enum ModKind {
    Press { pressed: bool },
    Lock { pressed: bool, locked: u8 },
    UnlockOnPress { pressed: bool, locked: bool },
    Latch { pressed: bool, latched: bool },
}

impl ModKind {
    pub fn update(&mut self, key_direction: KeyDirection) {
        match self {
            ModKind::Press { ref mut pressed } => match key_direction {
                KeyDirection::Down => *pressed = true,
                KeyDirection::Up => *pressed = false,
            },
            ModKind::Lock {
                ref mut pressed,
                ref mut locked,
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
            ModKind::UnlockOnPress { pressed, locked } => match key_direction {
                KeyDirection::Down => {
                    *pressed = true;
                    *locked = !*locked;
                }
                KeyDirection::Up => *pressed = false,
            },
            ModKind::Latch {
                ref mut pressed,
                ref mut latched,
            } => match key_direction {
                KeyDirection::Down => {
                    *pressed = true;
                    *latched = !*latched;
                }
                KeyDirection::Up => {
                    *pressed = false;
                }
            },
        }
    }

    fn unlatch(&mut self) {
        if let ModKind::Latch {
            pressed: _,
            latched,
        } = self
        {
            *latched = false
        }
    }

    pub fn pressed(&self) -> bool {
        match self {
            ModKind::Press { pressed, .. }
            | ModKind::Lock { pressed, .. }
            | ModKind::UnlockOnPress { pressed, .. }
            | ModKind::Latch { pressed, .. } => *pressed,
        }
    }

    fn depressed(&self) -> bool {
        match self {
            ModKind::Press { pressed } => *pressed,
            ModKind::Lock { pressed, .. } => *pressed,
            ModKind::Latch { pressed, latched } => *pressed && *latched,
            ModKind::UnlockOnPress { pressed, locked } => *pressed && *locked,
        }
    }

    pub fn locked(&self) -> bool {
        match self {
            ModKind::Lock { locked, .. } => *locked > 0,
            ModKind::UnlockOnPress { locked, .. } => *locked,
            _ => false,
        }
    }

    pub fn latched(&self) -> bool {
        match self {
            ModKind::Latch { latched, .. } => *latched,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StateModifier {
    pub(crate) mod_type: ModType,
    pub(crate) kind: ModKind,
}

impl StateModifier {
    pub(crate) fn has_mod_type(&self, mod_type: ModType) -> bool {
        self.mod_type == mod_type
    }

    pub fn unlatch(&mut self) {
        self.kind.unlatch();
    }

    pub fn update(&mut self, key_direction: KeyDirection) {
        self.kind.update(key_direction);
    }
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(StateModifier),
    Leveled(BTreeMap<u8, StateModifier>),
}

impl Modifier {
    pub(crate) fn for_each(&self, mut f: impl FnMut(&StateModifier)) {
        match self {
            Self::Single(sm) => f(sm),
            Self::Leveled(map) => map.values().for_each(f),
        }
    }

    pub(crate) fn for_each_mut(&mut self, mut f: impl FnMut(&mut StateModifier)) {
        match self {
            Self::Single(mk) => f(mk),
            Self::Leveled(map) => map.values_mut().for_each(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    /// Flat array of (evdev_code, Modifier) pairs. Typically 10-20 entries.
    pub(crate) entries: Vec<(u32, Modifier)>,
    raw: RawModifiers,
}

impl Default for Modifiers {
    fn default() -> Self {
        let single = |mod_type, kind| Modifier::Single(StateModifier { mod_type, kind });
        let press = |mod_type| single(mod_type, ModKind::Press { pressed: false });
        let lock = |mod_type| {
            single(
                mod_type,
                ModKind::Lock {
                    pressed: false,
                    locked: 0,
                },
            )
        };
        let entries = vec![
            (LEFT_CTRL, press(ModType::None)),
            (RIGHT_CTRL, press(ModType::None)),
            (LEFT_SHIFT, press(ModType::Level2)),
            (RIGHT_SHIFT, press(ModType::Level2)),
            (ALT, press(ModType::None)),
            (ALTGR, press(ModType::None)),
            (LOGO, press(ModType::None)),
            (CAPS_LOCK, lock(ModType::Caps)),
            (NUM_LOCK, lock(ModType::Num)),
            (SCROLL_LOCK, lock(ModType::Scroll)),
        ];
        Self {
            entries,
            raw: RawModifiers::default(),
        }
    }
}

impl Modifiers {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_MOD_SLOTS),
            raw: RawModifiers::default(),
        }
    }

    #[inline]
    pub fn get(&self, evdev_code: u32) -> Option<&Modifier> {
        self.entries
            .iter()
            .find(|(c, _)| *c == evdev_code)
            .map(|(_, m)| m)
    }

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

        self.rebuild_raw();
    }

    #[inline]
    fn effective(&self) -> u32 {
        self.raw.depressed | self.raw.latched | self.raw.locked
    }

    pub fn active_mod_type(&self, mod_type: ModType) -> bool {
        match mod_type {
            ModType::None => self.effective() & (MOD_CTRL | MOD_ALT | MOD_LOGO) != 0,
            ModType::Level2 => self.effective() & MOD_SHIFT != 0,
            ModType::Level3 => self.effective() & MOD_ALTGR != 0,
            ModType::Level5 => self.effective() & MOD_SCROLL_LOCK != 0,
            ModType::Caps => self.raw.locked & MOD_CAPS_LOCK != 0,
            ModType::Num => self.raw.locked & MOD_NUM_LOCK != 0,
            ModType::Scroll => self.raw.locked & MOD_SCROLL_LOCK != 0,
            ModType::Compose => self.entries.iter().any(|(_, modifier)| {
                let mut active = false;
                modifier.for_each(|state_modifier| {
                    active |= state_modifier.mod_type == ModType::Compose
                        && state_modifier.kind.pressed();
                });
                active
            }),
        }
    }

    #[inline(always)]
    pub fn active_none_and_levels(
        &self,
    ) -> (bool, bool, bool, bool) {
        let effective =
            self.raw.depressed | self.raw.latched | self.raw.locked;
    
        (
            effective & (MOD_CTRL | MOD_ALT | MOD_LOGO) != 0,
            effective & MOD_SHIFT != 0,
            effective & MOD_ALTGR != 0,
            effective & MOD_SCROLL_LOCK != 0,
        )
    }

    #[inline]
    pub fn caps_locked(&self) -> bool {
        self.raw.locked & MOD_CAPS_LOCK != 0
    }

    #[inline]
    pub fn num_locked(&self) -> bool {
        self.raw.locked & MOD_NUM_LOCK != 0
    }

    pub fn unlatch(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|(_, modifier)| modifier.for_each_mut(|sm| sm.unlatch()));
        self.raw.latched = 0;
    }

    #[inline]
    pub fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let position = match self.entries.iter().position(|(c, _)| *c == evdev_code) {
            Some(p) => p,
            None => return false,
        };
        let is_leveled = matches!(&self.entries[position].1, Modifier::Leveled(_));
        if is_leveled {
            let (_, level2, level3, level5) = self.active_none_and_levels();

            let level = level_index(level5, level3, level2) as u8;

            let Modifier::Leveled(levels) = &mut self.entries[position].1 else {
                unreachable!();
            };

            let Some(modifier) = levels.get_mut(&level) else {
                return false;
            };

            modifier.update(key_direction);
        } else {
            let Modifier::Single(modifier) = &mut self.entries[position].1 else {
                unreachable!();
            };

            modifier.update(key_direction);
        }

        self.rebuild_raw();
        true
    }

    pub fn state(&self, layout_index: usize) -> RawModifiers {
        RawModifiers {
            layout: layout_index as u32,
            ..self.raw
        }
    }

    /// Install aggregate state received from a Wayland compositor or copied
    /// across WKB layouts.
    ///
    /// This deliberately does not modify physical-key state in `entries`.
    pub(crate) fn update(&mut self, depressed: u32, latched: u32, locked: u32) {
        self.raw.depressed = depressed;
        self.raw.latched = latched;
        self.raw.locked = locked;
    }

    fn rebuild_raw(&mut self) {
        let layout = self.raw.layout;
        let mut raw = RawModifiers {
            layout,
            ..RawModifiers::default()
        };

        for (code, modifier) in &self.entries {
            modifier.for_each(|state_modifier| {
                let mask = modifier_mask(*code, state_modifier.mod_type);

                if mask == 0 {
                    return;
                }

                if state_modifier.kind.depressed() {
                    raw.depressed |= mask;
                }

                if state_modifier.kind.latched() {
                    raw.latched |= mask;
                }

                if state_modifier.kind.locked() {
                    raw.locked |= mask;
                }
            });
        }
        self.raw = raw;
    }

    pub(crate) fn leds_state(&self) -> LedState {
        LedState {
            num_lock: self.raw.locked & MOD_NUM_LOCK != 0,
            caps_lock: self.raw.locked & MOD_CAPS_LOCK != 0,
            scroll_lock: self.raw.locked & MOD_SCROLL_LOCK != 0,
        }
    }
}

fn modifier_mask(code: u32, mod_type: ModType) -> u32 {
    match mod_type {
        ModType::Level2 => MOD_SHIFT,
        ModType::Level3 => MOD_ALTGR,
        ModType::Level5 => MOD_SCROLL_LOCK,
        ModType::Caps => MOD_CAPS_LOCK,
        ModType::Num => MOD_NUM_LOCK,
        ModType::Scroll => MOD_SCROLL_LOCK,
        ModType::Compose => 0,

        // None is used for Ctrl, Alt and Logo. Preserve their distinct
        // protocol masks using the physical keycode.
        ModType::None => MODIFIER_MAPPING
            .iter()
            .find(|(mapped_code, _)| *mapped_code == code)
            .map_or(0, |(_, mask)| *mask),
    }
}

#[inline(always)]
pub fn level_index(level5: bool, level3: bool, level2: bool) -> usize {
    ((level5 as usize) << 2) | ((level3 as usize) << 1) | (level2 as usize)
}
