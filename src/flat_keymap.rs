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

    #[inline]
    pub(crate) fn num_levels(&self) -> usize {
        MAX_LEVELS
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

/// Flat keysym map: same layout as `FlatKeymap` but stores `u32` keysym values.
/// Index: `(layout * MAX_LEVELS + level) * num_keys + evdev_code`.
/// `0` means no keysym (KEY_NoSymbol).
#[derive(Debug, Clone)]
pub(crate) struct FlatKeysymMap {
    pub(crate) data: Vec<u32>,
    pub(crate) num_keys: usize,
    pub(crate) num_layouts: usize,
}

impl FlatKeysymMap {
    pub(crate) fn new(num_keys: usize, num_layouts: usize) -> Self {
        Self {
            data: vec![0; num_layouts * MAX_LEVELS * num_keys],
            num_keys,
            num_layouts,
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, layout: usize, level: usize, evdev_code: u32) -> u32 {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx]
        } else {
            0
        }
    }

    #[inline]
    pub(crate) fn set(&mut self, layout: usize, level: usize, evdev_code: u32, sym: u32) {
        let k = evdev_code as usize;
        if k < self.num_keys && layout < self.num_layouts {
            let idx = (layout * MAX_LEVELS + level) * self.num_keys + k;
            self.data[idx] = sym;
        }
    }
}
