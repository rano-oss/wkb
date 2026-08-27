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
    pub(crate) num_levels: usize,
}

impl<T: FlatMapValue> FlatMap<T> {
    pub(crate) fn with_levels(num_keys: usize, num_levels: usize) -> Self {
        let num_levels = num_levels.clamp(1, MAX_LEVELS);
        Self {
            data: vec![T::empty(); num_levels * num_keys],
            num_keys,
            num_levels,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, level: usize, evdev_code: u32) -> T {
        let k = evdev_code as usize;
        if k < self.num_keys && level < self.num_levels {
            self.data[level * self.num_keys + k]
        } else {
            T::empty()
        }
    }

    #[inline(always)]
    pub(crate) fn level_for_modifiers(&self, modifiers: &Modifiers) -> usize {
        let (level2, level3, level5) = modifiers.active_levels();
        let n = self.num_levels;
        level_index(
            level5 && n > 4,
            level3 && n > 2,
            level2 && n > 1,
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
