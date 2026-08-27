use crate::{KeyBitSet, KeyDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroupChange {
    Absolute(u8),
    Relative(i8),
}

impl GroupChange {
    #[inline]
    fn apply(self, value: &mut i32) {
        match self {
            Self::Absolute(group) => *value = group.into(),
            Self::Relative(delta) => *value += i32::from(delta),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroupKind {
    Press(i8),
    Tap(GroupChange),
    LockOnPress(GroupChange),
    LockOnRelease(GroupChange),
    LatchOnPress(GroupChange),
    LatchOnRelease(GroupChange),
    LatchToLockOnPress(GroupChange),
    LatchToLockOnRelease(GroupChange),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group {
    pub keys: Vec<u32>,
    pub action: GroupKind,
}

impl Group {
    #[inline]
    fn contains(&self, code: u32) -> bool {
        self.keys.contains(&code)
    }

    #[inline]
    fn active(&self, pressed: &KeyBitSet) -> bool {
        self.keys.iter().all(|key| pressed.contains(*key))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Groups {
    pub(crate) entries: Vec<Group>,
    active: Vec<(usize, bool)>,
    pressed: KeyBitSet,
    group_keys: KeyBitSet,
    latched_action: Option<GroupChange>,
    base: i32,
    latched: i32,
    locked: i32,
}

impl Groups {
    pub fn new(entries: Vec<Group>) -> Self {
        let mut group_keys = KeyBitSet::default();
        for &key in entries.iter().flat_map(|group| &group.keys) {
            group_keys.insert(key);
        }
        Self {
            entries,
            group_keys,
            ..Self::default()
        }
    }

    #[inline]
    pub fn effective(&self, layouts: usize) -> usize {
        if layouts == 0 {
            return 0;
        }

        (self.base + self.latched + self.locked).rem_euclid(layouts as i32) as usize
    }

    pub fn set_layout(&mut self, layout: usize, layouts: usize) -> bool {
        if layout >= layouts {
            return false;
        }

        self.base = 0;
        self.latched = 0;
        self.locked = layout as i32;
        self.latched_action = None;
        self.active.clear();
        self.pressed = KeyBitSet::default();
        true
    }

    pub(crate) fn set_key(&mut self, key: u32, action: GroupKind) {
        self.group_keys.insert(key);

        if let Some(group) = self
            .entries
            .iter_mut()
            .find(|group| group.keys.as_slice() == [key])
        {
            group.action = action;
        } else {
            self.entries.push(Group {
                keys: vec![key],
                action,
            });
        }
    }

    pub fn update(
        &mut self,
        code: u32,
        direction: KeyDirection,
        consumes_latch: bool,
        layouts: usize,
    ) -> usize {
        let changed = match direction {
            KeyDirection::Down => self.pressed.insert(code),
            KeyDirection::Up => self.pressed.remove(code),
        };

        if !changed {
            return self.effective(layouts);
        }

        if !self.group_keys.contains(code) {
            if direction == KeyDirection::Down {
                if consumes_latch {
                    self.clear_latch();
                }

                for (idx, interrupted) in &mut self.active {
                    *interrupted |= release_action(self.entries[*idx].action);
                }
            }

            return self.effective(layouts);
        }

        match direction {
            KeyDirection::Down => {
                let latches = self.entries.iter().enumerate().any(|(idx, group)| {
                    group.contains(code)
                        && group.active(&self.pressed)
                        && !self.active.iter().any(|(active, _)| *active == idx)
                        && latch_action(group.action)
                });

                if consumes_latch && !latches {
                    self.clear_latch();
                }

                for (idx, interrupted) in &mut self.active {
                    let group = &self.entries[*idx];

                    *interrupted |= !group.contains(code) && release_action(group.action);
                }

                for idx in 0..self.entries.len() {
                    let group = &self.entries[idx];

                    if !group.contains(code)
                        || !group.active(&self.pressed)
                        || self.active.iter().any(|(active, _)| *active == idx)
                    {
                        continue;
                    }

                    let action = group.action;
                    self.down(action);
                    self.active.push((idx, false));
                }
            }

            KeyDirection::Up => {
                let mut idx = 0;

                while idx < self.active.len() {
                    let entry = self.active[idx].0;

                    if self.entries[entry].active(&self.pressed) {
                        idx += 1;
                        continue;
                    }

                    let (entry, interrupted) = self.active.swap_remove(idx);
                    self.up(self.entries[entry].action, interrupted);
                }
            }
        }

        self.effective(layouts)
    }

    #[inline]
    fn clear_latch(&mut self) {
        self.latched = 0;
        self.latched_action = None;
    }

    fn down(&mut self, action: GroupKind) {
        match action {
            GroupKind::Press(delta) => self.base += i32::from(delta),
            GroupKind::LockOnPress(change) => change.apply(&mut self.locked),
            GroupKind::LatchOnPress(change) => self.latch(change),
            GroupKind::LatchToLockOnPress(change) => {
                self.latch_or_lock(change);
            }
            _ => {}
        }
    }

    fn up(&mut self, action: GroupKind, interrupted: bool) {
        match action {
            GroupKind::Press(delta) => self.base -= i32::from(delta),

            GroupKind::Tap(change) | GroupKind::LockOnRelease(change) if !interrupted => {
                change.apply(&mut self.locked);
            }

            GroupKind::LatchOnRelease(change) => self.latch(change),

            GroupKind::LatchToLockOnRelease(change) => {
                self.latch_or_lock(change);
            }

            _ => {}
        }
    }

    fn latch(&mut self, change: GroupChange) {
        self.latched = 0;
        change.apply(&mut self.latched);
        self.latched_action = Some(change);
    }

    fn latch_or_lock(&mut self, change: GroupChange) {
        if self.latched_action == Some(change) {
            self.clear_latch();
            change.apply(&mut self.locked);
        } else {
            self.latch(change);
        }
    }
}

#[inline]
fn release_action(action: GroupKind) -> bool {
    matches!(action, GroupKind::Tap(_) | GroupKind::LockOnRelease(_))
}

#[inline]
fn latch_action(action: GroupKind) -> bool {
    matches!(
        action,
        GroupKind::LatchOnPress(_)
            | GroupKind::LatchOnRelease(_)
            | GroupKind::LatchToLockOnPress(_)
            | GroupKind::LatchToLockOnRelease(_)
    )
}
