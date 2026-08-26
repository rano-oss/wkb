use crate::modifiers::{level_index, Modifiers};
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

    #[inline(always)]
    pub(crate) fn populated_levels(&self) -> usize {
        self.data.len() / self.num_keys.max(1)
    }

    #[inline(always)]
    pub(crate) fn level_for_modifiers(&self, modifiers: &Modifiers) -> usize {
        let (_, level2, level3, level5) = modifiers.active_none_and_levels();
        let levels = self.populated_levels();
        level_index(
            level5 && levels > 4,
            level3 && levels > 2,
            level2 && levels > 1,
        )
    }
}

impl FlatMap<NamedKey> {
    #[inline]
    pub(crate) fn get_with_fallback(&self, level: usize, evdev_code: u32) -> NamedKey {
        let at = self.get(level, evdev_code);
        if at != NamedKey::Unnamed || level == 0 {
            return at;
        }
        for l in (0..level).rev() {
            let named = self.get(l, evdev_code);
            if named != NamedKey::Unnamed {
                return named;
            }
        }
        NamedKey::Unnamed
    }
}

/// Flat keymap: `MAX_LEVELS` planes of `num_keys` slots.
/// Index: `level * num_keys + evdev_code`.
pub(crate) type FlatKeymap = FlatMap<Option<char>>;
/// Flat named-key map: same shape as `FlatKeymap` but stores [`NamedKey`] values.
/// [`NamedKey::Unnamed`] means no named key is mapped.
pub(crate) type FlatNamedKeyMap = FlatMap<NamedKey>;
