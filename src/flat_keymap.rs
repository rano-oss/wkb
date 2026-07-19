use crate::named_keys::NamedKey;

pub(crate) const MAX_LEVELS: usize = 8;

pub(crate) trait FlatMapValue: Copy {
    fn empty() -> Self;
}

impl FlatMapValue for Option<char> {
    fn empty() -> Self {
        None
    }
}

impl FlatMapValue for NamedKey {
    fn empty() -> Self {
        NamedKey::Unnamed
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FlatMap<T: FlatMapValue> {
    pub(crate) data: Vec<T>,
    pub(crate) num_keys: usize,
    pub(crate) num_layouts: usize,
}

impl<T: FlatMapValue> FlatMap<T> {
    pub(crate) fn new(num_keys: usize, num_layouts: usize) -> Self {
        Self {
            data: vec![T::empty(); num_layouts * MAX_LEVELS * num_keys],
            num_keys,
            num_layouts,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, layout: usize, level: usize, evdev_code: u32) -> T {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx]
        } else {
            T::empty()
        }
    }

    #[inline]
    pub(crate) fn set(
        &mut self,
        layout: usize,
        level: usize,
        evdev_code: u32,
        value: impl Into<T>,
    ) {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx] = value.into();
        }
    }
}

pub(crate) type FlatKeymap = FlatMap<Option<char>>;
pub(crate) type FlatNamedKeyMap = FlatMap<NamedKey>;
