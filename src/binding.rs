use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Binding<T> {
    Single(T),
    Leveled(BTreeMap<u8, T>),
}

impl<T: Copy> Binding<T> {
    pub(crate) fn get(&self, level: usize) -> Option<T> {
        match self {
            Self::Single(value) => Some(*value),
            Self::Leveled(levels) => {
                levels.get(&u8::try_from(level).ok()?).copied()
            }
        }
    }

    pub(crate) fn for_each(&self, mut f: impl FnMut(&T)) {
        match self {
            Self::Single(sm) => f(&sm),
            Self::Leveled(map) => map.values().for_each(f),
        }
    }

    pub(crate) fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
        match self {
            Self::Single(mk) => f(mk),
            Self::Leveled(map) => map.values_mut().for_each(f),
        }
    }
}
