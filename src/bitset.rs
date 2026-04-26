/// Maximum evdev key code we support (768 covers all standard keycodes).
pub(crate) const BITSET_WORDS: usize = 12; // 12 * 64 = 768 bits

/// Compact bitset for tracking key states. Covers evdev codes 0..767.
#[derive(Debug, Clone)]
pub(crate) struct KeyBitSet {
    bits: [u64; BITSET_WORDS],
}

impl KeyBitSet {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            bits: [0; BITSET_WORDS],
        }
    }

    #[inline(always)]
    pub(crate) fn contains(&self, key: u32) -> bool {
        let k = key as usize;
        if k < BITSET_WORDS * 64 {
            self.bits[k >> 6] & (1u64 << (k & 63)) != 0
        } else {
            false
        }
    }

    #[inline(always)]
    pub(crate) fn insert(&mut self, key: u32) {
        let k = key as usize;
        if k < BITSET_WORDS * 64 {
            self.bits[k >> 6] |= 1u64 << (k & 63);
        }
    }
}
