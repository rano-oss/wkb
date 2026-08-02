use crate::named_keys::NamedKey;

/// Maximum number of shift levels.
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
}

impl<T: FlatMapValue> FlatMap<T> {
    pub(crate) fn new(num_keys: usize) -> Self {
        Self {
            data: vec![T::empty(); MAX_LEVELS * num_keys],
            num_keys,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, level: usize, evdev_code: u32) -> T {
        let k = evdev_code as usize;
        if k < self.num_keys {
            let idx = level * self.num_keys + k;
            self.data[idx]
        } else {
            T::empty()
        }
    }
}

/// Flat keymap: `MAX_LEVELS` planes of `num_keys` slots.
/// Index: `level * num_keys + evdev_code`.
pub(crate) type FlatKeymap = FlatMap<Option<char>>;
/// Flat named-key map: same shape as `FlatKeymap` but stores [`NamedKey`] values.
/// [`NamedKey::Unnamed`] means no named key is mapped.
pub(crate) type FlatNamedKeyMap = FlatMap<NamedKey>;
