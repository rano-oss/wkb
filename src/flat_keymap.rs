use crate::named_keys::NamedKey;

/// Maximum number of shift levels.
pub(crate) const MAX_LEVELS: usize = 8;

/// Flat keymap: `num_layouts * MAX_LEVELS` planes of `num_keys` slots.
/// Index: `(layout * MAX_LEVELS + level) * num_keys + evdev_code`.
#[derive(Debug, Clone)]
pub(crate) struct FlatKeymap {
    pub(crate) data: Vec<Option<char>>,
    pub(crate) num_keys: usize,
    pub(crate) num_layouts: usize,
}

impl FlatKeymap {
    pub(crate) fn new(num_keys: usize, num_layouts: usize) -> Self {
        Self {
            data: vec![None; num_layouts * MAX_LEVELS * num_keys],
            num_keys,
            num_layouts,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, layout: usize, level: usize, evdev_code: u32) -> Option<char> {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx]
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn set(&mut self, layout: usize, level: usize, evdev_code: u32, ch: char) {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx] = Some(ch);
        }
    }
}

/// Flat named-key map: same layout as `FlatKeymap` but stores [`NamedKey`] values.
/// Index: `(layout * MAX_LEVELS + level) * num_keys + evdev_code`.
/// [`NamedKey::Unnamed`] means no named key is mapped.
#[derive(Debug, Clone)]
pub(crate) struct FlatNamedKeyMap {
    pub(crate) data: Vec<NamedKey>,
    pub(crate) num_keys: usize,
    pub(crate) num_layouts: usize,
}

impl FlatNamedKeyMap {
    pub(crate) fn new(num_keys: usize, num_layouts: usize) -> Self {
        Self {
            data: vec![NamedKey::Unnamed; num_layouts * MAX_LEVELS * num_keys],
            num_keys,
            num_layouts,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, layout: usize, level: usize, evdev_code: u32) -> NamedKey {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx]
        } else {
            NamedKey::Unnamed
        }
    }

    #[inline]
    pub(crate) fn set(&mut self, layout: usize, level: usize, evdev_code: u32, key: NamedKey) {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx] = key;
        }
    }
}
