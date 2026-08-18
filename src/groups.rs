use std::collections::BTreeSet;

use crate::KeyDirection;

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

/// One complete group activation rule.
///
/// `key` owns the action. `with` contains the additional keys which must
/// already be held when `key` is pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub key: u32,
    pub with: Vec<u32>,
    pub action: GroupKind,
}

impl Group {
    pub fn new(key: u32, action: GroupKind) -> Self {
        Self {
            key,
            with: Vec::new(),
            action,
        }
    }

    pub fn with_keys(key: u32, with: impl IntoIterator<Item = u32>, action: GroupKind) -> Self {
        let mut with = with.into_iter().collect::<Vec<_>>();
        with.sort_unstable();
        with.dedup();

        Self { key, with, action }
    }

    fn matches(&self, code: u32, pressed: &BTreeSet<u32>) -> bool {
        self.key == code && self.with.iter().all(|key| pressed.contains(key))
    }

    fn contains(&self, code: u32) -> bool {
        self.key == code || self.with.contains(&code)
    }
}

#[derive(Debug, Clone, Copy)]
struct ActiveGroup {
    rule: usize,
    interrupted: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Groups {
    pub(crate) entries: Vec<Group>,
    pressed: BTreeSet<u32>,
    active: Vec<ActiveGroup>,
    latched_action: Option<GroupChange>,
    base: i32,
    latched: i32,
    locked: i32,
}

impl Groups {
    pub fn new(mut entries: Vec<Group>) -> Self {
        for entry in &mut entries {
            entry.with.sort_unstable();
            entry.with.dedup();
        }

        entries.dedup();

        Self {
            entries,
            ..Self::default()
        }
    }

    pub fn set(&mut self, group: Group) {
        self.entries
            .retain(|entry| entry.key != group.key || entry.with != group.with);
        self.entries.push(group);
    }

    pub fn remove_key(&mut self, code: u32) -> bool {
        let before = self.entries.len();
        self.entries.retain(|entry| entry.key != code);
        before != self.entries.len()
    }

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
        true
    }

    pub fn update(
        &mut self,
        code: u32,
        direction: KeyDirection,
        consumes_latch: bool,
        layouts: usize,
    ) -> usize {
        match direction {
            KeyDirection::Down => self.key_down(code, consumes_latch),
            KeyDirection::Up => self.key_up(code),
        }

        self.effective(layouts)
    }

    fn key_down(&mut self, code: u32, consumes_latch: bool) {
        // Ignore key-repeat presses for group state.
        if !self.pressed.insert(code) {
            return;
        }

        let selected = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, group)| group.matches(code, &self.pressed))
            // Prefer the most specific matching combination.
            .max_by_key(|(_, group)| group.with.len())
            .map(|(index, group)| (index, group.action));

        let selected_is_latch = selected.is_some_and(|(_, action)| {
            matches!(
                action,
                GroupKind::LatchOnPress(_)
                    | GroupKind::LatchOnRelease(_)
                    | GroupKind::LatchToLockOnPress(_)
                    | GroupKind::LatchToLockOnRelease(_)
            )
        });

        if consumes_latch && !selected_is_latch {
            self.latched = 0;
            self.latched_action = None;
        }

        // A different key interrupts pending tap/release actions.
        for active in &mut self.active {
            let group = &self.entries[active.rule];

            if !group.contains(code) {
                active.interrupted |= matches!(
                    group.action,
                    GroupKind::Tap(_) | GroupKind::LockOnRelease(_)
                );
            }
        }

        if let Some((rule, action)) = selected {
            self.down(action);
            self.active.push(ActiveGroup {
                rule,
                interrupted: false,
            });
        }
    }

    fn key_up(&mut self, code: u32) {
        if !self.pressed.remove(&code) {
            return;
        }

        // Release actions belong to the rule's owning key, not its required
        // combination keys.
        let mut index = 0;

        while index < self.active.len() {
            let active = self.active[index];

            if self.entries[active.rule].key == code {
                self.active.remove(index);
                self.up(self.entries[active.rule].action, active.interrupted);
            } else {
                index += 1;
            }
        }
    }

    fn down(&mut self, action: GroupKind) {
        match action {
            GroupKind::Press(delta) => {
                self.base += i32::from(delta);
            }
            GroupKind::LockOnPress(change) => {
                change.apply(&mut self.locked);
            }
            GroupKind::LatchOnPress(change) => {
                self.latch(change);
            }
            GroupKind::LatchToLockOnPress(change) => {
                self.latch_or_lock(change);
            }
            _ => {}
        }
    }

    fn up(&mut self, action: GroupKind, interrupted: bool) {
        match action {
            GroupKind::Press(delta) => {
                self.base -= i32::from(delta);
            }
            GroupKind::Tap(change) if !interrupted => {
                change.apply(&mut self.locked);
            }
            GroupKind::LockOnRelease(change) if !interrupted => {
                change.apply(&mut self.locked);
            }
            GroupKind::LatchOnRelease(change) => {
                self.latch(change);
            }
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
            self.latched = 0;
            self.latched_action = None;
            change.apply(&mut self.locked);
        } else {
            self.latch(change);
        }
    }
}
