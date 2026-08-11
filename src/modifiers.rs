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
    Lock(LockFlags),
    Latch(LatchVariant),
    None,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct LockFlags: u8 {
        const LOCK_ON_RELEASE = 1 << 0;
        const UNLOCK_ON_PRESS = 1 << 1;
        const TAP = 1 << 2;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LatchVariant {
    OnPress,
    #[default]
    OnRelease,
}

impl LatchVariant {
    pub(crate) fn is_on_release(&self) -> bool {
        *self == Self::OnRelease
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

#[derive(Debug, Clone, Copy, Default)]
struct ModState {
    pressed: bool,
    locked: bool,
    lock_changed: bool,
    latched: bool,
    tap_used: bool,
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
        let down = key_direction == KeyDirection::Down;
        match self.kind {
            ModKind::Press => self.state.pressed = down,
            ModKind::Lock(flags) if flags.contains(LockFlags::TAP) => {
                self.state.pressed = down;
                if down {
                    self.state.tap_used = false;
                } else if !self.state.tap_used {
                    self.state.locked = !self.state.locked;
                }
            }
            ModKind::Lock(flags) if down => {
                self.state.pressed = true;
                self.state.lock_changed = false;
                if self.state.locked && flags.contains(LockFlags::UNLOCK_ON_PRESS) {
                    self.state.locked = false;
                    self.state.lock_changed = true;
                } else if !self.state.locked && !flags.contains(LockFlags::LOCK_ON_RELEASE) {
                    self.state.locked = true;
                    self.state.lock_changed = true;
                }
            }
            ModKind::Lock(flags) => {
                self.state.pressed = false;
                if !self.state.lock_changed {
                    if self.state.locked && !flags.contains(LockFlags::UNLOCK_ON_PRESS) {
                        self.state.locked = false;
                    } else if !self.state.locked && flags.contains(LockFlags::LOCK_ON_RELEASE) {
                        self.state.locked = true;
                    }
                }
            }
            ModKind::Latch(LatchVariant::OnPress) => {
                self.state.pressed = down;
                if down {
                    self.state.latched = !self.state.latched;
                }
            }
            ModKind::Latch(LatchVariant::OnRelease) if down => {
                self.state.pressed = !std::mem::take(&mut self.state.latched);
            }
            ModKind::Latch(LatchVariant::OnRelease) => {
                if std::mem::take(&mut self.state.pressed) {
                    self.state.latched = true;
                }
            }
            ModKind::None => {}
        }
    }

    fn update_from_state(&mut self, pressed: bool, locked: bool, latched: bool) {
        self.state.pressed = pressed;
        self.state.locked = locked;
        self.state.lock_changed = false;
        self.state.latched = latched;
    }

    fn unlatch(&mut self) {
        if matches!(self.kind, ModKind::Latch(_)) {
            self.state.latched = false;
        }
    }

    fn use_tap(&mut self) {
        if matches!(self.kind, ModKind::Lock(flags) if flags.contains(LockFlags::TAP))
            && self.state.pressed
        {
            self.state.tap_used = true;
        }
    }

    fn state(&self) -> (bool, bool, bool) {
        (
            self.state.pressed,
            matches!(self.kind, ModKind::Lock(_)) && self.state.locked,
            matches!(self.kind, ModKind::Latch(_)) && self.state.latched,
        )
    }

    fn pressed(&self) -> bool {
        self.kind != ModKind::None && self.state.pressed
    }

    fn locked(&self) -> bool {
        matches!(self.kind, ModKind::Lock(_)) && self.state.locked
    }

    fn active(&self) -> bool {
        match self.kind {
            ModKind::Press => self.state.pressed,
            ModKind::Lock(flags) if flags.contains(LockFlags::LOCK_ON_RELEASE) => {
                self.state.pressed || self.state.locked
            }
            ModKind::Lock(_) => self.state.locked,
            ModKind::Latch(LatchVariant::OnPress) => self.state.latched,
            ModKind::Latch(LatchVariant::OnRelease) => self.state.pressed || self.state.latched,
            ModKind::None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Group {
    id: u8,
    kind: ModKind,
    clear_locks: bool,
    latch_to_lock: bool,
}

/// High bit set in `Group.id` means the value is a relative group delta
/// (wrapped into 7 bits, sign-extended at runtime); otherwise it is an
/// absolute 0-based layout index.
pub(crate) const GROUP_RELATIVE_MARKER: u8 = 0x80;

impl Group {
    pub(crate) fn new(id: u8, kind: ModKind, clear_locks: bool, latch_to_lock: bool) -> Self {
        Self {
            id,
            kind,
            clear_locks,
            latch_to_lock,
        }
    }

    pub(crate) fn relative(delta: i8, kind: ModKind) -> Option<Self> {
        (-64..=63).contains(&delta).then(|| {
            Self::new(
                GROUP_RELATIVE_MARKER | ((delta as u8) & 0x7f),
                kind,
                false,
                false,
            )
        })
    }

    #[cfg(test)]
    pub(crate) fn kind(&self) -> ModKind {
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
    tap_used: bool,
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
            .iter()
            .rev()
            .find(|pressed| {
                !matches!(pressed.action.kind, ModKind::Lock(flags) if flags.contains(LockFlags::TAP))
            })
            .map(|pressed| pressed.target)
            .or(self.latched)
            .unwrap_or(self.locked)
    }

    pub(crate) fn set_layout(&mut self, layout: usize) {
        self.locked = layout;
        self.latched = None;
        self.pressed.clear();
    }

    pub(crate) fn update_key(
        &mut self,
        keycode: u32,
        direction: KeyDirection,
        action: Option<Group>,
        is_modifier: bool,
        current: usize,
        num_layouts: usize,
    ) -> usize {
        if direction == KeyDirection::Up {
            return self.release(keycode);
        }

        for pressed in &mut self.pressed {
            if pressed.keycode != keycode
                && matches!(pressed.action.kind, ModKind::Lock(flags) if flags.contains(LockFlags::TAP))
            {
                pressed.tap_used = true;
            }
        }
        if !is_modifier {
            self.latched = None;
        }

        let Some(action) = action else {
            return self.effective();
        };
        let Some(target) = action.resolve(current, num_layouts) else {
            return self.effective();
        };

        if action.clear_locks {
            self.locked = 0;
        }

        match action.kind {
            ModKind::Lock(flags)
                if !flags.intersects(LockFlags::LOCK_ON_RELEASE | LockFlags::TAP) =>
            {
                self.locked = target;
            }
            ModKind::None => {}
            _ => {
                self.pressed.retain(|pressed| pressed.keycode != keycode);
                self.pressed.push(PressedGroup {
                    keycode,
                    action,
                    target,
                    tap_used: false,
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
            ModKind::Press | ModKind::None => {}
            ModKind::Latch(_) => {
                if pressed.action.latch_to_lock && self.latched == Some(pressed.target) {
                    self.locked = pressed.target;
                    self.latched = None;
                } else {
                    self.latched = Some(pressed.target);
                }
            }
            ModKind::Lock(flags) if flags.contains(LockFlags::TAP) && !pressed.tap_used => {
                self.locked = pressed.target;
            }
            ModKind::Lock(flags)
                if flags.contains(LockFlags::LOCK_ON_RELEASE)
                    && !flags.contains(LockFlags::TAP) =>
            {
                self.locked = pressed.target;
            }
            ModKind::Lock(_) => {}
        }
        self.effective()
    }
}

#[derive(Debug, Clone, Default)]
pub struct KeyEffect {
    pub modifier: Option<StateModifier>,
    pub(crate) group: Option<Group>,
}

impl KeyEffect {
    pub(crate) fn modifier(mod_type: ModType, kind: ModKind) -> Self {
        Self {
            modifier: Some(StateModifier::new(mod_type, kind)),
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

    fn use_tap(&mut self) {
        if let Some(modifier) = &mut self.modifier {
            modifier.use_tap();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Single(KeyEffect),
    Leveled(BTreeMap<u8, KeyEffect>),
}

impl Modifier {
    fn effect(&self, level: u8) -> Option<&KeyEffect> {
        match self {
            Modifier::Single(effect) => Some(effect),
            Modifier::Leveled(map) => map.get(&level).or_else(|| map.get(&0)),
        }
    }

    fn effect_mut(&mut self, level: u8) -> Option<&mut KeyEffect> {
        match self {
            Modifier::Single(effect) => Some(effect),
            Modifier::Leveled(map) => {
                let level = if map.contains_key(&level) { level } else { 0 };
                map.get_mut(&level)
            }
        }
    }

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

    fn level_for(&self, mod_type: ModType) -> Option<Option<u8>> {
        match self {
            Modifier::Single(effect) => effect
                .mod_kind_from_mod_type(mod_type)
                .is_some()
                .then_some(None),
            Modifier::Leveled(map) => map.iter().find_map(|(level, effect)| {
                effect
                    .mod_kind_from_mod_type(mod_type)
                    .is_some()
                    .then_some(Some(*level))
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
        let single = |mod_type, kind| Modifier::Single(KeyEffect::modifier(mod_type, kind));
        let lock = ModKind::Lock(LockFlags::empty());
        let entries = vec![
            (LEFT_CTRL, single(ModType::None, ModKind::Press)),
            (RIGHT_CTRL, single(ModType::None, ModKind::Press)),
            (LEFT_SHIFT, single(ModType::Level2, ModKind::Press)),
            (RIGHT_SHIFT, single(ModType::Level2, ModKind::Press)),
            (ALT, single(ModType::None, ModKind::Press)),
            (ALTGR, single(ModType::None, ModKind::Press)),
            (LOGO, single(ModType::None, ModKind::Press)),
            (CAPS_LOCK, single(ModType::Caps, lock)),
            (NUM_LOCK, single(ModType::Num, lock)),
            (SCROLL_LOCK, single(ModType::Scroll, lock)),
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

    pub(crate) fn level_code(&self, mod_type: ModType) -> Option<(u32, Option<u8>)> {
        self.iter()
            .find_map(|(code, modifier)| modifier.level_for(mod_type).map(|level| (*code, level)))
    }

    /// Insert or replace a modifier for the given evdev code.
    pub fn set_modifier(&mut self, evdev_code: u32, modifier: Modifier) {
        if let Some((_, existing)) = self.entries.iter_mut().find(|(c, _)| *c == evdev_code) {
            *existing = modifier;
        } else {
            self.entries.push((evdev_code, modifier));
        }
    }

    pub(crate) fn set_group(&mut self, evdev_code: u32, group: Group) {
        if let Some(modifier) = self.get_mut(evdev_code) {
            modifier.for_each_mut(|effect| effect.group = Some(group));
        } else {
            self.set_modifier(
                evdev_code,
                Modifier::Single(KeyEffect {
                    group: Some(group),
                    ..KeyEffect::default()
                }),
            );
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

    fn active_level(&self) -> u8 {
        let (_, level2, level3, level5) = self.active_none_and_levels();
        level_index(level5, level3, level2) as u8
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
        if key_direction == KeyDirection::Down {
            self.entries
                .iter_mut()
                .filter(|(code, _)| *code != evdev_code)
                .for_each(|(_, modifier)| modifier.for_each_mut(KeyEffect::use_tap));
        }

        let pos = match self.entries.iter().position(|(c, _)| *c == evdev_code) {
            Some(p) => p,
            None => return (false, None),
        };
        if key_direction == KeyDirection::Down {
            // The modifier component is selected from the state before the
            // press. XKB group actions, however, use the level after the key's
            // own modifier contribution has been applied (for example the
            // second key in a Ctrl+Shift group toggle).
            let level = self.active_level();
            let Some(effect) = self.entries[pos].1.effect_mut(level) else {
                return (false, None);
            };
            effect.update(key_direction);

            let level = self.active_level();
            let group = self.entries[pos].1.effect(level).and_then(KeyEffect::group);
            return (true, group);
        }

        if let Modifier::Leveled(map) = &mut self.entries[pos].1 {
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
        (true, None)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latch_variants_activate_on_their_named_edge() {
        let mut on_press =
            StateModifier::new(ModType::Level3, ModKind::Latch(LatchVariant::OnPress));
        on_press.update(KeyDirection::Down);
        assert_eq!(on_press.state(), (true, false, true));

        let mut on_release =
            StateModifier::new(ModType::Level3, ModKind::Latch(LatchVariant::OnRelease));
        on_release.update(KeyDirection::Down);
        assert_eq!(on_release.state(), (true, false, false));
        on_release.update(KeyDirection::Up);
        assert_eq!(on_release.state(), (false, false, true));
    }

    #[test]
    fn lock_flags_use_and_combine_their_named_edges() {
        let mut unlock =
            StateModifier::new(ModType::Level2, ModKind::Lock(LockFlags::UNLOCK_ON_PRESS));
        unlock.update(KeyDirection::Down);
        unlock.update(KeyDirection::Up);
        assert!(unlock.locked());
        unlock.update(KeyDirection::Down);
        assert!(!unlock.locked());

        let mut release =
            StateModifier::new(ModType::Level2, ModKind::Lock(LockFlags::LOCK_ON_RELEASE));
        release.update(KeyDirection::Down);
        assert!(!release.locked());
        assert!(release.active());
        release.update(KeyDirection::Up);
        assert!(release.locked());

        let mut combined = StateModifier::new(
            ModType::Level2,
            ModKind::Lock(LockFlags::LOCK_ON_RELEASE | LockFlags::UNLOCK_ON_PRESS),
        );
        combined.update(KeyDirection::Down);
        assert!(!combined.locked());
        combined.update(KeyDirection::Up);
        assert!(combined.locked());
        combined.update(KeyDirection::Down);
        assert!(!combined.locked());
        combined.update(KeyDirection::Up);
        assert!(!combined.locked());
    }

    #[test]
    fn tap_lock_is_momentary_when_used() {
        let mut tap = StateModifier::new(ModType::Level2, ModKind::Lock(LockFlags::TAP));
        tap.update(KeyDirection::Down);
        assert!(!tap.active());
        assert!(!tap.locked());
        tap.use_tap();
        tap.update(KeyDirection::Up);
        assert!(!tap.active());

        tap.update(KeyDirection::Down);
        tap.update(KeyDirection::Up);
        assert!(tap.locked());
    }

    #[test]
    fn tap_group_locks_only_when_unused() {
        let tap = Group::new(
            GROUP_RELATIVE_MARKER | 1,
            ModKind::Lock(LockFlags::TAP),
            false,
            false,
        );
        let mut state = GroupState::default();

        assert_eq!(
            state.update_key(42, KeyDirection::Down, Some(tap), true, 0, 2),
            0
        );
        assert_eq!(
            state.update_key(30, KeyDirection::Down, None, false, 1, 2),
            0
        );
        assert_eq!(state.update_key(42, KeyDirection::Up, None, true, 1, 2), 0);

        assert_eq!(
            state.update_key(42, KeyDirection::Down, Some(tap), true, 0, 2),
            0
        );
        assert_eq!(state.update_key(42, KeyDirection::Up, None, true, 1, 2), 1);
    }
}
