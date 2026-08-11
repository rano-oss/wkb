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
    Press,
    Lock,
    Latch,
    None,
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

#[derive(Debug, Clone, Copy, Default)]
struct ModState {
    pressed: bool,
    locked: u8,
    latched: bool,
}

#[derive(Debug, Clone)]
pub struct StateModifier {
    pub(crate) mod_type: ModType,
    pub(crate) kind: ModKind,
    state: ModState,
}

impl StateModifier {
    pub(crate) fn new(mod_type: ModType, kind: ModKind) -> Self {
        Self {
            mod_type,
            kind,
            state: ModState::default(),
        }
    }

    fn update(&mut self, key_direction: KeyDirection) {
        match (self.kind, key_direction) {
            (ModKind::Press, KeyDirection::Down) => self.state.pressed = true,
            (ModKind::Press, KeyDirection::Up) => self.state.pressed = false,
            (ModKind::Lock, KeyDirection::Down) => {
                self.state.pressed = true;
                if self.state.locked == 0 {
                    self.state.locked = 2;
                }
            }
            (ModKind::Lock, KeyDirection::Up) => {
                self.state.pressed = false;
                if self.state.locked != 0 {
                    self.state.locked -= 1;
                }
            }
            (ModKind::Latch, KeyDirection::Down) => {
                self.state.pressed = true;
                self.state.latched = !self.state.latched;
            }
            (ModKind::Latch, KeyDirection::Up) => self.state.pressed = false,
            (ModKind::None, _) => {}
        }
    }

    fn update_from_state(&mut self, pressed: bool, locked: bool, latched: bool) {
        self.state.pressed = pressed;
        self.state.locked = locked as u8;
        self.state.latched = latched;
    }

    fn unlatch(&mut self) {
        if self.kind == ModKind::Latch {
            self.state.latched = false;
        }
    }

    fn state(&self) -> (bool, bool, bool) {
        (
            self.state.pressed,
            self.kind == ModKind::Lock && self.state.locked > 0,
            self.kind == ModKind::Latch && self.state.latched,
        )
    }

    fn pressed(&self) -> bool {
        self.kind != ModKind::None && self.state.pressed
    }

    fn locked(&self) -> bool {
        self.kind == ModKind::Lock && self.state.locked > 0
    }

    fn active(&self) -> bool {
        match self.kind {
            ModKind::Press => self.state.pressed,
            ModKind::Lock => self.state.locked > 0,
            ModKind::Latch => self.state.latched,
            ModKind::None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GroupKind {
    Set,
    Latch,
    Lock { on_release: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Group {
    id: u8,
    kind: GroupKind,
    clear_locks: bool,
    latch_to_lock: bool,
}

/// High bit set in `Group.id` means the value is a relative group delta
/// (wrapped into 7 bits, sign-extended at runtime); otherwise it is an
/// absolute 0-based layout index.
pub(crate) const GROUP_RELATIVE_MARKER: u8 = 0x80;

impl Group {
    pub(crate) fn new(id: u8, kind: GroupKind, clear_locks: bool, latch_to_lock: bool) -> Self {
        Self {
            id,
            kind,
            clear_locks,
            latch_to_lock,
        }
    }

    #[cfg(test)]
    pub(crate) fn kind(&self) -> GroupKind {
        self.kind
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
struct PressedGroup {
    keycode: u32,
    action: Group,
    target: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct GroupState {
    locked: usize,
    latched: Option<usize>,
    pressed: Vec<PressedGroup>,
}

impl GroupState {
    pub(crate) fn effective(&self) -> usize {
        self.pressed
            .last()
            .map(|pressed| pressed.target)
            .or(self.latched)
            .unwrap_or(self.locked)
    }

    pub(crate) fn set_layout(&mut self, layout: usize) {
        self.locked = layout;
        self.latched = None;
        self.pressed.clear();
    }

    pub(crate) fn press(
        &mut self,
        keycode: u32,
        action: Group,
        current: usize,
        num_layouts: usize,
    ) -> usize {
        let Some(target) = action.resolve(current, num_layouts) else {
            return self.effective();
        };

        if action.clear_locks {
            self.locked = 0;
        }

        match action.kind {
            GroupKind::Set | GroupKind::Latch => {
                self.pressed.retain(|pressed| pressed.keycode != keycode);
                self.pressed.push(PressedGroup {
                    keycode,
                    action,
                    target,
                });
            }
            GroupKind::Lock { on_release: false } => self.locked = target,
            GroupKind::Lock { on_release: true } => {
                self.pressed.retain(|pressed| pressed.keycode != keycode);
                self.pressed.push(PressedGroup {
                    keycode,
                    action,
                    target,
                });
            }
        }
        self.effective()
    }

    pub(crate) fn release(&mut self, keycode: u32) -> usize {
        let Some(index) = self
            .pressed
            .iter()
            .rposition(|pressed| pressed.keycode == keycode)
        else {
            return self.effective();
        };
        let pressed = self.pressed.remove(index);
        match pressed.action.kind {
            GroupKind::Set => {}
            GroupKind::Latch => {
                if pressed.action.latch_to_lock && self.latched == Some(pressed.target) {
                    self.locked = pressed.target;
                    self.latched = None;
                } else {
                    self.latched = Some(pressed.target);
                }
            }
            GroupKind::Lock { on_release: true } => self.locked = pressed.target,
            GroupKind::Lock { on_release: false } => {}
        }
        self.effective()
    }

    pub(crate) fn unlatch(&mut self) -> usize {
        self.latched = None;
        self.effective()
    }
}

#[derive(Debug, Clone, Default)]
pub struct KeyEffect {
    pub modifier: Option<StateModifier>,
    pub(crate) group: Option<Group>,
}

impl KeyEffect {
    pub fn from_modifier(modifier: StateModifier) -> Self {
        Self {
            modifier: Some(modifier),
            group: None,
        }
    }

    pub fn mod_kind_from_mod_type(&self, mod_type: ModType) -> Option<&ModKind> {
        let state_modifier = self.modifier.as_ref()?;
        if state_modifier.mod_type == mod_type {
            Some(&state_modifier.kind)
        } else {
            None
        }
    }

    pub fn active_mod_kind(&self) -> Option<&ModType> {
        let state_modifier = self.modifier.as_ref()?;
        if state_modifier.active() {
            Some(&state_modifier.mod_type)
        } else {
            None
        }
    }

    pub fn state(&self) -> (bool, bool, bool) {
        self.modifier
            .as_ref()
            .map(StateModifier::state)
            .unwrap_or_default()
    }

    /// Whether this effect's modifier component is currently pressed.
    pub fn pressed(&self) -> bool {
        self.modifier.as_ref().is_some_and(StateModifier::pressed)
    }

    pub fn update(&mut self, key_direction: KeyDirection) {
        if let Some(modifier) = &mut self.modifier {
            modifier.update(key_direction);
        }
    }

    pub fn update_from_state(&mut self, pressed: bool, locked: bool, latched: bool) {
        if let Some(modifier) = &mut self.modifier {
            modifier.update_from_state(pressed, locked, latched);
        }
    }

    pub(crate) fn group(&self) -> Option<Group> {
        self.group
    }

    pub fn unlatch(&mut self) {
        if let Some(modifier) = &mut self.modifier {
            modifier.unlatch();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(KeyEffect),
    Leveled(BTreeMap<u8, KeyEffect>),
}

impl Modifier {
    fn for_each(&self, mut f: impl FnMut(&KeyEffect)) {
        match self {
            Modifier::Single(mk) => f(mk),
            Modifier::Leveled(map) => map.values().for_each(f),
        }
    }

    fn for_each_mut(&mut self, mut f: impl FnMut(&mut KeyEffect)) {
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
        let single = |mod_type, kind| {
            Modifier::Single(KeyEffect::from_modifier(StateModifier::new(mod_type, kind)))
        };
        let entries = vec![
            (LEFT_CTRL, single(ModType::None, ModKind::Press)),
            (RIGHT_CTRL, single(ModType::None, ModKind::Press)),
            (LEFT_SHIFT, single(ModType::Level2, ModKind::Press)),
            (RIGHT_SHIFT, single(ModType::Level2, ModKind::Press)),
            (ALT, single(ModType::None, ModKind::Press)),
            (ALTGR, single(ModType::None, ModKind::Press)),
            (LOGO, single(ModType::None, ModKind::Press)),
            (CAPS_LOCK, single(ModType::Caps, ModKind::Lock)),
            (NUM_LOCK, single(ModType::Num, ModKind::Lock)),
            (SCROLL_LOCK, single(ModType::Scroll, ModKind::Lock)),
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
                    .modifier
                    .as_ref()
                    .is_some_and(|modifier| modifier.mod_type == mod_type && modifier.locked())
            });
            found
        })
    }

    #[inline]
    pub(crate) fn update_key(
        &mut self,
        evdev_code: u32,
        key_direction: KeyDirection,
    ) -> (bool, Option<Group>) {
        let pos = match self.entries.iter().position(|(c, _)| *c == evdev_code) {
            Some(p) => p,
            None => return (false, None),
        };
        let mut group = None;
        if key_direction == KeyDirection::Down {
            // The modifier component is selected from the state before the
            // press. XKB group actions, however, use the level after the key's
            // own modifier contribution has been applied (for example the
            // second key in a Ctrl+Shift group toggle).
            let (_, l2, l3, l5) = self.active_none_and_levels();
            let level = level_index(l5, l3, l2) as u8;
            if let Modifier::Leveled(map) = &mut self.entries[pos].1 {
                let target = if map.contains_key(&level) { level } else { 0 };
                if let Some(effect) = map.get_mut(&target) {
                    effect.update(key_direction);
                } else {
                    return (false, None);
                }
            } else if let Modifier::Single(effect) = &mut self.entries[pos].1 {
                effect.update(key_direction);
            }

            let (_, l2, l3, l5) = self.active_none_and_levels();
            let level = level_index(l5, l3, l2) as u8;
            group = match &self.entries[pos].1 {
                Modifier::Single(effect) => effect.group(),
                Modifier::Leveled(map) => map
                    .get(&level)
                    .or_else(|| map.get(&0))
                    .and_then(KeyEffect::group),
            };
        } else if let Modifier::Leveled(map) = &mut self.entries[pos].1 {
            // Release the level that is actually pressed. A press activates
            // exactly one level's effect, so scanning for the pressed effect
            // targets the same level without extra bookkeeping.
            let pressed_level = map
                .iter()
                .find(|(_, effect)| effect.pressed())
                .map(|(level, _)| *level);
            if let Some(level) = pressed_level {
                if let Some(effect) = map.get_mut(&level) {
                    effect.update(key_direction);
                }
            }
        } else if let Modifier::Single(effect) = &mut self.entries[pos].1 {
            effect.update(key_direction);
        }
        (true, group)
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
fn inherit_effect(dst: &mut KeyEffect, src: &KeyEffect) {
    if let (Some(dst), Some(src)) = (&mut dst.modifier, &src.modifier) {
        if dst.kind == src.kind {
            dst.state = src.state;
        }
    }
}
