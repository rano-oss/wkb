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

#[derive(Debug, Clone)]
pub enum ModKind {
    Press { pressed: bool },
    Lock { pressed: bool, locked: u8 },
    Latch { pressed: bool, latched: bool },
    UnlockOnPress { pressed: bool, locked: bool },
    LockOnRelease { pressed: bool, locked: u8 },
    None,
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
            ModKind::UnlockOnPress {
                ref mut pressed,
                ref mut locked,
            } => match key_direction {
                KeyDirection::Down => {
                    *pressed = true;
                    *locked = !*locked;
                }
                KeyDirection::Up => {
                    *pressed = false;
                }
            },
            ModKind::LockOnRelease {
                ref mut pressed,
                ref mut locked,
            } => match key_direction {
                KeyDirection::Down => {
                    *pressed = true;
                    if *locked != 0 {
                        *locked -= 1;
                    }
                }
                KeyDirection::Up => {
                    *pressed = false;
                    if *locked == 0 {
                        *locked = 2;
                    } else {
                        *locked -= 1;
                    }
                }
            },
            ModKind::None => {}
        }
    }

    fn update_from_state(&mut self, pressed: bool, locked: bool, latched: bool) {
        match self {
            ModKind::Press { pressed: p } => *p = pressed,
            ModKind::Lock {
                pressed: p,
                locked: l,
            } => {
                *p = pressed;
                *l = locked as u8;
            }
            ModKind::Latch {
                pressed: p,
                latched: lt,
            } => {
                *p = pressed;
                *lt = latched;
            }
            ModKind::UnlockOnPress {
                pressed: p,
                locked: l,
            } => {
                *p = pressed;
                *l = locked;
            }
            ModKind::LockOnRelease {
                pressed: p,
                locked: l,
            } => {
                *p = pressed;
                *l = locked as u8;
            }
            ModKind::None => {}
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

    pub fn locked(&self) -> bool {
        match self {
            ModKind::UnlockOnPress { locked, .. } => *locked,
            ModKind::Lock { locked, .. } | ModKind::LockOnRelease { locked, .. } => *locked > 0,
            _ => false,
        }
    }

    pub fn latched(&self) -> bool {
        matches!(self, ModKind::Latch { latched, .. } if *latched)
    }

    pub fn pressed(&self) -> bool {
        match self {
            ModKind::Press { pressed }
            | ModKind::Lock { pressed, .. }
            | ModKind::Latch { pressed, .. }
            | ModKind::UnlockOnPress { pressed, .. }
            | ModKind::LockOnRelease { pressed, .. } => *pressed,
            ModKind::None => false,
        }
    }

    pub fn active(&self) -> bool {
        match self {
            ModKind::UnlockOnPress { locked, .. } => *locked,
            ModKind::Lock { locked, .. } | ModKind::LockOnRelease { locked, .. } => *locked > 0,
            ModKind::Press { pressed } => *pressed,
            ModKind::Latch { latched, .. } => *latched,
            ModKind::None => false,
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
pub(crate) struct Group {
    id: u8,
    kind: ModKind,
}

/// High bit set in `Group.id` means the value is a relative group delta
/// (wrapped into 7 bits, sign-extended at runtime); otherwise it is an
/// absolute 0-based layout index.
pub(crate) const GROUP_RELATIVE_MARKER: u8 = 0x80;

impl Group {
    pub(crate) fn new(id: u8, kind: ModKind) -> Self {
        Self { id, kind }
    }

    #[cfg(test)]
    pub(crate) fn kind(&self) -> &ModKind {
        &self.kind
    }

    /// Whether this group switch is a relative delta from the active layout
    /// rather than an absolute target index.
    pub(crate) fn is_relative(&self) -> bool {
        self.id & GROUP_RELATIVE_MARKER != 0
    }

    /// Resolve the target layout index given the currently active layout.
    /// Returns `None` for absolute ids that are out of range.
    pub(crate) fn resolve(&self, current: usize, num_layouts: usize) -> Option<usize> {
        if num_layouts == 0 {
            return None;
        }
        if self.is_relative() {
            let low = self.id & 0x7F;
            let delta = if low & 0x40 != 0 {
                (low as i32) - 0x80
            } else {
                low as i32
            };
            Some(((current as i32 + delta).rem_euclid(num_layouts as i32)) as usize)
        } else {
            (self.id < num_layouts as u8).then_some(self.id as usize)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModifierEffect {
    Modifier(StateModifier),
    Group(Group),
    Dual(StateModifier, Group),
}

impl ModifierEffect {
    pub fn mod_kind_from_mod_type(&self, mod_type: ModType) -> Option<&ModKind> {
        match self {
            ModifierEffect::Modifier(state_modifier) => {
                if state_modifier.mod_type == mod_type {
                    Some(&state_modifier.kind)
                } else {
                    None
                }
            }
            ModifierEffect::Group(_group) => None,
            ModifierEffect::Dual(state_modifier, _group) => {
                if state_modifier.mod_type == mod_type {
                    Some(&state_modifier.kind)
                } else {
                    None
                }
            }
        }
    }

    pub fn active_mod_kind(&self) -> Option<&ModType> {
        match self {
            ModifierEffect::Modifier(state_modifier) => {
                if state_modifier.kind.active() {
                    Some(&state_modifier.mod_type)
                } else {
                    None
                }
            }
            ModifierEffect::Group(_group) => None,
            ModifierEffect::Dual(state_modifier, _group) => {
                if state_modifier.kind.active() {
                    Some(&state_modifier.mod_type)
                } else {
                    None
                }
            }
        }
    }

    pub fn state(&self) -> (bool, bool, bool) {
        match self {
            ModifierEffect::Modifier(state_modifier) => (
                state_modifier.kind.pressed(),
                state_modifier.kind.locked(),
                state_modifier.kind.latched(),
            ),
            ModifierEffect::Group(_group) => (false, false, false),
            ModifierEffect::Dual(state_modifier, _group) => (
                state_modifier.kind.pressed(),
                state_modifier.kind.locked(),
                state_modifier.kind.latched(),
            ),
        }
    }

    /// Whether this effect is currently pressed (either the modifier part,
    /// the group part, or both).
    pub fn pressed(&self) -> bool {
        match self {
            ModifierEffect::Modifier(state_modifier) => state_modifier.kind.pressed(),
            ModifierEffect::Group(group) => group.kind.pressed(),
            ModifierEffect::Dual(state_modifier, group) => {
                state_modifier.kind.pressed() || group.kind.pressed()
            }
        }
    }

    pub fn update(&mut self, key_direction: KeyDirection) {
        match self {
            ModifierEffect::Modifier(state_modifier) => state_modifier.kind.update(key_direction),
            ModifierEffect::Group(group) => group.kind.update(key_direction),
            ModifierEffect::Dual(state_modifier, group) => {
                state_modifier.kind.update(key_direction);
                group.kind.update(key_direction);
            }
        }
    }

    pub fn update_from_state(&mut self, pressed: bool, locked: bool, latched: bool) {
        match self {
            ModifierEffect::Modifier(state_modifier) => state_modifier
                .kind
                .update_from_state(pressed, locked, latched),
            ModifierEffect::Group(_group) => {}
            ModifierEffect::Dual(state_modifier, _group) => {
                state_modifier
                    .kind
                    .update_from_state(pressed, locked, latched);
            }
        }
    }

    /// The group switch carried by this effect, if its group action is
    /// currently active (pressed, latched or locked).
    pub(crate) fn active_group(&self) -> Option<&Group> {
        match self {
            ModifierEffect::Modifier(_) => None,
            ModifierEffect::Group(group) => group.kind.active().then_some(group),
            ModifierEffect::Dual(_, group) => group.kind.active().then_some(group),
        }
    }

    pub fn unlatch(&mut self) {
        match self {
            ModifierEffect::Modifier(state_modifier) => state_modifier.kind.unlatch(),
            ModifierEffect::Group(group) => group.kind.unlatch(),
            ModifierEffect::Dual(state_modifier, group) => {
                state_modifier.kind.unlatch();
                group.kind.unlatch();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(ModifierEffect),
    Leveled(BTreeMap<u8, ModifierEffect>),
}

impl Modifier {
    fn for_each(&self, mut f: impl FnMut(&ModifierEffect)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values().for_each(f),
        }
    }

    fn for_each_mut(&mut self, mut f: impl FnMut(&mut ModifierEffect)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values_mut().for_each(f),
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
        let entries = vec![
            (
                LEFT_CTRL,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::None,
                })),
            ),
            (
                RIGHT_CTRL,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::None,
                })),
            ),
            (
                LEFT_SHIFT,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::Level2,
                })),
            ),
            (
                RIGHT_SHIFT,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::Level2,
                })),
            ),
            (
                ALT,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::None,
                })),
            ),
            (
                ALTGR,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::None,
                })),
            ),
            (
                LOGO,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Press { pressed: false },
                    mod_type: ModType::None,
                })),
            ),
            (
                CAPS_LOCK,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Lock {
                        pressed: false,
                        locked: 0,
                    },
                    mod_type: ModType::Caps,
                })),
            ),
            (
                NUM_LOCK,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Lock {
                        pressed: false,
                        locked: 0,
                    },
                    mod_type: ModType::Num,
                })),
            ),
            (
                SCROLL_LOCK,
                Modifier::Single(ModifierEffect::Modifier(StateModifier {
                    kind: ModKind::Lock {
                        pressed: false,
                        locked: 0,
                    },
                    mod_type: ModType::Scroll,
                })),
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

    /// Active modifier state: bit0=none, bit1=level2, bit2=level3, bit3=level5,
    /// bit4=compose, bit5=caps_locked, bit6=num_locked.
    #[inline]
    fn state_bits(&self) -> u8 {
        let mut state = 0u8;
        for (_, modifier) in &self.entries {
            modifier.for_each(|mk| {
                if let Some(mod_type) = mk.active_mod_kind() {
                    match mod_type {
                        ModType::None => state |= STATE_NONE,
                        ModType::Level2 => state |= STATE_LEVEL2,
                        ModType::Level3 => state |= STATE_LEVEL3,
                        ModType::Level5 => state |= STATE_LEVEL5,
                        ModType::Compose => state |= STATE_COMPOSE,
                        ModType::Caps => state |= STATE_CAPS_LOCKED,
                        ModType::Num => state |= STATE_NUM_LOCKED,
                        ModType::Scroll => {}
                    }
                }
            });
        }
        state
    }

    pub fn active_mod_type(&self, mod_type: ModType) -> bool {
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
    #[inline]
    pub fn active_none_and_levels(&self) -> (bool, bool, bool, bool) {
        let state = self.state_bits();
        (
            state & STATE_NONE != 0,
            state & STATE_LEVEL2 != 0,
            state & STATE_LEVEL3 != 0,
            state & STATE_LEVEL5 != 0,
        )
    }

    /// Return true if Caps Lock is locked.
    #[inline]
    pub fn caps_locked(&self) -> bool {
        self.state_bits() & STATE_CAPS_LOCKED != 0
    }

    /// Return true if Num Lock is locked.
    #[inline]
    pub fn num_locked(&self) -> bool {
        self.state_bits() & STATE_NUM_LOCKED != 0
    }

    pub fn unlatch(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|(_, modifier)| modifier.for_each_mut(|ke| ke.unlatch()));
    }

    pub fn locked_with_type(&self, evdev_code: u32, mod_type: ModType) -> bool {
        self.get(evdev_code).is_some_and(|modifier| {
            let mut found = false;
            modifier.for_each(|me| {
                found |= me
                    .mod_kind_from_mod_type(mod_type)
                    .is_some_and(|mk| mk.locked())
            });
            found
        })
    }

    #[inline]
    pub fn set_state(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let pos = match self.entries.iter().position(|(c, _)| *c == evdev_code) {
            Some(p) => p,
            None => return false,
        };
        if key_direction == KeyDirection::Down {
            // Select the level from the modifier state BEFORE this press so
            // the same level is released on key up (modifier state may have
            // changed by then).
            let (_, l2, l3, l5) = self.active_none_and_levels();
            let level = level_index(l5, l3, l2) as u8;
            if let Modifier::Leveled(map) = &mut self.entries[pos].1 {
                let target = if map.contains_key(&level) { level } else { 0 };
                if let Some(mod_kind) = map.get_mut(&target) {
                    mod_kind.update(key_direction);
                } else {
                    return false;
                }
            } else if let Modifier::Single(mod_kind) = &mut self.entries[pos].1 {
                mod_kind.update(key_direction);
            }
        } else if let Modifier::Leveled(map) = &mut self.entries[pos].1 {
            // Release the level that is actually pressed. A press activates
            // exactly one level's effect, so scanning for the pressed effect
            // targets the same level without extra bookkeeping.
            let pressed_level = map
                .iter()
                .find(|(_, effect)| effect.pressed())
                .map(|(level, _)| *level);
            if let Some(level) = pressed_level {
                if let Some(mod_kind) = map.get_mut(&level) {
                    mod_kind.update(key_direction);
                }
            }
        } else if let Modifier::Single(mod_kind) = &mut self.entries[pos].1 {
            mod_kind.update(key_direction);
        }
        true
    }

    /// If the given key is a group switch and its group action is currently
    /// active (at the level relevant for the active modifiers), return it.
    pub(crate) fn active_group(&self, evdev_code: u32) -> Option<&Group> {
        let modifier = self.get(evdev_code)?;
        match modifier {
            Modifier::Single(effect) => effect.active_group(),
            Modifier::Leveled(map) => {
                let (_, l2, l3, l5) = self.active_none_and_levels();
                let level = level_index(l5, l3, l2) as u8;
                map.get(&level)
                    .or_else(|| map.get(&0))
                    .and_then(|effect| effect.active_group())
            }
        }
    }

    pub fn state(&self, layout_index: usize) -> RawModifiers {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let layout = layout_index as u32;
        for (code, bit) in MODIFIER_MAPPING {
            if let Some(modifier) = self.get(code) {
                modifier.for_each(|mk| {
                    let state = mk.state();
                    if state.0 {
                        depressed |= bit;
                    }
                    if state.1 {
                        locked |= bit;
                    }
                    if state.2 {
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
                m.for_each_mut(|mk| mk.update_from_state(is_depressed, is_locked, is_latched));
            }
        }
    }

    /// Copy the currently-held state (pressed/latched/locked) for each
    /// modifier key present in `from` into this `Modifiers`. Layouts can have
    /// different modifier keys, so only keys present in both keep their state;
    /// keys only in one layout are left as-is. Used to preserve still-held
    /// modifier keys across a layout switch (matches xkbcommon, which keeps
    /// modifier state across group changes).
    pub(crate) fn inherit_state(&mut self, from: &Modifiers) {
        for (code, modifier) in &mut self.entries {
            let Some((_, other)) = from.entries.iter().find(|(c, _)| *c == *code) else {
                continue;
            };
            inherit_modifier(modifier, other);
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

/// Copy the held state of each level from `src` into `dst`, preserving `dst`'s
/// own structure (levels are matched by level index; levels only present in one
/// side are untouched). Used when layouts define a modifier key differently.
fn inherit_modifier(dst: &mut Modifier, src: &Modifier) {
    match (dst, src) {
        (Modifier::Single(dst), Modifier::Single(src)) => inherit_effect(dst, src),
        (Modifier::Leveled(dst), Modifier::Leveled(src)) => {
            for (level, dst_effect) in dst {
                if let Some(src_effect) = src.get(level) {
                    inherit_effect(dst_effect, src_effect);
                }
            }
        }
        _ => {}
    }
}

/// Copy the held state from `src` into `dst`, preserving `dst`'s own modifier
/// type and group identity. Only state fields (pressed/locked/latched) are
/// carried over.
fn inherit_effect(dst: &mut ModifierEffect, src: &ModifierEffect) {
    match (dst, src) {
        (ModifierEffect::Modifier(dst), ModifierEffect::Modifier(src)) => {
            inherit_kind(&mut dst.kind, &src.kind);
        }
        (ModifierEffect::Group(dst), ModifierEffect::Group(src)) => {
            inherit_kind(&mut dst.kind, &src.kind);
        }
        (ModifierEffect::Dual(dst_mod, dst_group), ModifierEffect::Dual(src_mod, src_group)) => {
            inherit_kind(&mut dst_mod.kind, &src_mod.kind);
            inherit_kind(&mut dst_group.kind, &src_group.kind);
        }
        _ => {}
    }
}

/// Copy the pressed/locked/latched state of `src` into `dst` when both are the
/// same `ModKind` variant; otherwise leave `dst` unchanged.
fn inherit_kind(dst: &mut ModKind, src: &ModKind) {
    match (dst, src) {
        (ModKind::Press { pressed }, ModKind::Press { pressed: s }) => *pressed = *s,
        (
            ModKind::Lock { pressed, locked },
            ModKind::Lock {
                pressed: s,
                locked: sl,
            },
        ) => {
            *pressed = *s;
            *locked = *sl;
        }
        (
            ModKind::Latch { pressed, latched },
            ModKind::Latch {
                pressed: s,
                latched: sl,
            },
        ) => {
            *pressed = *s;
            *latched = *sl;
        }
        (
            ModKind::UnlockOnPress { pressed, locked },
            ModKind::UnlockOnPress {
                pressed: s,
                locked: sl,
            },
        ) => {
            *pressed = *s;
            *locked = *sl;
        }
        (
            ModKind::LockOnRelease { pressed, locked },
            ModKind::LockOnRelease {
                pressed: s,
                locked: sl,
            },
        ) => {
            *pressed = *s;
            *locked = *sl;
        }
        (ModKind::None, ModKind::None) => {}
        _ => {}
    }
}
