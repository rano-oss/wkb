use std::collections::HashMap;

use crate::{KeyDirection, binding::Binding};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupChange {
    Absolute(u8),
    Relative(i8),
}

impl GroupChange {
    fn apply(self, value: &mut i32) {
        match self {
            Self::Absolute(group) => *value = group.into(),
            Self::Relative(delta) => *value += i32::from(delta),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub type Group = Binding<GroupKind>;

#[derive(Debug, Clone, Default)]
pub struct Groups {
    pub(crate) entries: Vec<(u32, Group)>,
    active: HashMap<u32, (GroupKind, bool)>,
    latched_action: Option<GroupChange>,
    base: i32,
    latched: i32,
    locked: i32,
}

impl Groups {
    pub fn new(entries: Vec<(u32, Group)>) -> Self {
        Self {
            entries,
            ..Self::default()
        }
    }

    pub fn effective(&self, layouts: usize) -> usize {
        if layouts == 0 {
            return 0;
        }

        (self.base + self.latched + self.locked)
            .rem_euclid(layouts as i32) as usize
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
        true
    }

    pub fn update(
        &mut self,
        code: u32,
        direction: KeyDirection,
        level: usize,
        consumes_latch: bool,
        layouts: usize,
    ) -> usize {
        match direction {
            KeyDirection::Down if !self.active.contains_key(&code) => {
                let action = self
                    .entries
                    .iter()
                    .find(|(key, _)| *key == code)
                    .and_then(|(_, group)| group.get(level));
                let is_latch_action = matches!(
                    action,
                    Some(
                        GroupKind::LatchOnPress(_)
                            | GroupKind::LatchOnRelease(_)
                            | GroupKind::LatchToLockOnPress(_)
                            | GroupKind::LatchToLockOnRelease(_)
                    )
                );
                if consumes_latch && !is_latch_action {
                    self.latched = 0;
                    self.latched_action = None;
                }
                self.active
                    .values_mut()
                    .for_each(|(action, interrupted)| {
                        *interrupted |= matches!(
                            action,
                            GroupKind::Tap(_)
                                | GroupKind::LockOnRelease(_)
                        );
                    });
                if let Some(action) = action {
                    self.down(action);
                    self.active.insert(code, (action, false));
                }
            }
            KeyDirection::Up => {
                if let Some((action, interrupted)) =
                    self.active.remove(&code)
                {
                    self.up(action, interrupted);
                }
            }
            _ => {}
        }
    
        self.effective(layouts)
    }

    fn down(&mut self, action: GroupKind) {
        match action {
            GroupKind::Press(delta) => self.base += i32::from(delta),
            GroupKind::LockOnPress(change) => change.apply(&mut self.locked),
            GroupKind::LatchOnPress(change) => self.latch(change),
            GroupKind::LatchToLockOnPress(change) => self.latch_or_lock(change),
            _ => {}
        }
    }

    fn up(&mut self, action: GroupKind, interrupted: bool) {
        match action {
            GroupKind::Press(delta) => self.base -= i32::from(delta),
            GroupKind::Tap(change) if !interrupted => {
                change.apply(&mut self.locked)
            }
            GroupKind::LockOnRelease(change) if !interrupted => {
                change.apply(&mut self.locked)
            }
            GroupKind::LatchOnRelease(change) => self.latch(change),
            GroupKind::LatchToLockOnRelease(change) => {
                self.latch_or_lock(change)
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
            self.latched = 0;
            self.latched_action = None;
            change.apply(&mut self.locked);
        } else {
            self.latch(change);
        }
    }
}
