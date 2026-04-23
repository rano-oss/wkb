pub use composer::{ComposeState, Composer, ListComposer, Token};
#[doc(hidden)]
pub mod composer;
use modifiers::{level_index, Modifiers, *};
pub use modifiers::{KeyDirection, ModType};
#[doc(hidden)]
pub mod modifiers;
#[doc(hidden)]
pub mod testing;
#[doc(hidden)]
pub mod xkb;

/// Maximum number of shift levels.
const MAX_LEVELS: usize = 8;

/// Maximum evdev key code we support (768 covers all standard keycodes).
const BITSET_WORDS: usize = 12; // 12 * 64 = 768 bits

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

    #[inline(always)]
    pub(crate) fn remove(&mut self, key: u32) {
        let k = key as usize;
        if k < BITSET_WORDS * 64 {
            self.bits[k >> 6] &= !(1u64 << (k & 63));
        }
    }
}

/// Flat keymap: `MAX_LEVELS` planes of `num_keys` slots.
/// Index: `level * num_keys + evdev_code`.
#[derive(Debug, Clone)]
pub(crate) struct FlatKeymap {
    pub(crate) data: Vec<Option<char>>,
    pub(crate) num_keys: usize,
}

impl FlatKeymap {
    pub(crate) fn new(num_keys: usize) -> Self {
        Self {
            data: vec![None; MAX_LEVELS * num_keys],
            num_keys,
        }
    }

    #[inline]
    pub(crate) fn num_levels(&self) -> usize {
        MAX_LEVELS
    }

    #[inline(always)]
    pub(crate) fn get(&self, level: usize, evdev_code: u32) -> Option<char> {
        let k = evdev_code as usize;
        if k < self.num_keys {
            let idx = level * self.num_keys + k;
            self.data[idx]
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn set(&mut self, level: usize, evdev_code: u32, ch: char) {
        let k = evdev_code as usize;
        if k < self.num_keys {
            let idx = level * self.num_keys + k;
            self.data[idx] = Some(ch);
        }
    }
}

const MODIFIER_MAPPING: [(u32, u32); 9] = [
    (LEFT_SHIFT, 1),
    (RIGHT_SHIFT, 1),
    (CAPS_LOCK, 2),
    (LEFT_CTRL, 4),
    (RIGHT_CTRL, 4),
    (ALT, 8),
    (NUM_LOCK, 16),
    (LOGO, 64),
    (ALTGR, 128),
];

#[derive(Debug, Clone)]
pub struct WKB<C: Composer> {
    pub(crate) layouts: Vec<String>,
    pub(crate) layout: String,
    // pub(crate) locale: Option<String>,
    pub(crate) pressed_keys: KeyBitSet,
    pub(crate) repeat_keys: KeyBitSet,
    pub(crate) composer: C,
    pub(crate) modifiers: Modifiers,
    pub(crate) state_keymap: FlatKeymap,
    pub(crate) num_lock_keys: FlatKeymap,
    pub(crate) caps_lock_keymap: FlatKeymap,
    pub(crate) level_exceptions_keymap: FlatKeymap,
}

impl WKB<ListComposer> {
    /// Create WKB instance from RMLVO names (Rules, Model, Layout, Variant, Options)
    pub fn new_from_names(locale: String, layout: Option<String>) -> Self {
        xkb::new_from_names(locale, layout)
    }

    /// Create WKB instance from XKB keymap string
    pub fn new_from_string(string: String) -> Self {
        xkb::new_from_string(string)
    }
}

impl<C: Composer> WKB<C> {
    /// Reset all transient input state: compose sequence and pressed keys.
    /// Call on wl_keyboard.leave or when focus changes.
    pub fn reset_state(&mut self) {
        self.composer.reset();
        self.pressed_keys = KeyBitSet::new();
    }

    pub fn modifiers_state(&self) -> (u32, u32, u32, u32) {
        let mut depressed = 0;
        let mut latched = 0;
        let mut locked = 0;
        let group = 0;
        for (code, bit) in MODIFIER_MAPPING {
            if let Some(Modifier::Single(mk)) = self.modifiers.get(code) {
                match mk {
                    ModKind::Pressed { pressed: true, .. } => depressed |= bit,
                    ModKind::Lock {
                        pressed, locked: l, ..
                    } => {
                        if *pressed {
                            depressed |= bit;
                        }
                        if *l > 0 {
                            locked |= bit;
                        }
                    }
                    ModKind::Latch {
                        pressed,
                        latched: is_latched,
                        ..
                    } => {
                        if *pressed {
                            depressed |= bit;
                        }
                        if *is_latched {
                            latched |= bit;
                        }
                    }
                    _ => {}
                }
            }
        }
        (depressed, latched, locked, group)
    }

    pub fn leds_state(&self) -> u32 {
        let mut leds = 0;
        if self.modifiers.locked_with_type(NUM_LOCK, ModType::Num) {
            leds |= 1;
        }
        if self.modifiers.locked_with_type(CAPS_LOCK, ModType::Caps) {
            leds |= 2;
        }
        if self
            .modifiers
            .locked_with_type(SCROLL_LOCK, ModType::Scroll)
        {
            leds |= 4;
        }
        leds
    }

    pub fn update_modifiers(&mut self, depressed: u32, latched: u32, locked: u32, group: u32) {
        if let Some(l) = self.layouts.get(group as usize) {
            self.layout = l.clone();
        }
        for (code, bit) in MODIFIER_MAPPING {
            let is_depressed = (depressed & bit) != 0;
            let is_locked = (locked & bit) != 0;
            let is_latched = (latched & bit) != 0;

            if let Some(m) = self.modifiers.get_mut(code) {
                if let Modifier::Single(mk) = m {
                    match mk {
                        ModKind::Pressed { pressed, .. } => *pressed = is_depressed,
                        ModKind::Lock {
                            pressed, locked, ..
                        } => {
                            *pressed = is_depressed;
                            *locked = if is_locked { 1 } else { 0 };
                        }
                        ModKind::Latch {
                            pressed, latched, ..
                        } => {
                            *pressed = is_depressed;
                            *latched = is_latched;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[inline]
    pub fn level_key(&self, evdev_code: u32, level_index: usize) -> Option<char> {
        self.level_exceptions_keymap
            .get(level_index, evdev_code)
            .or_else(|| self.state_keymap.get(level_index, evdev_code))
    }

    #[inline]
    pub fn num_levels(&self) -> usize {
        self.state_keymap.num_levels()
    }

    pub fn key_repeats(&self, evdev_code: u32) -> bool {
        self.repeat_keys.contains(evdev_code)
    }

    #[inline]
    pub fn utf8(&mut self, evdev_code: u32) -> Option<char> {
        let (none_active, level2, level3, level5) = self.modifiers.active_none_and_levels();
        if none_active {
            return None;
        }
        let nk = self.state_keymap.num_keys;
        let level5 = level5 && self.state_keymap.data.len() > 4 * nk;
        let level3 = level3 && self.state_keymap.data.len() > 2 * nk;
        let level2 = level2 && self.state_keymap.data.len() > 1 * nk;
        let base_level = level_index(level5, level3, level2);

        if self.modifiers.locked(NUM_LOCK) {
            if let Some(key) = self.num_lock_keys.get(base_level, evdev_code) {
                return Some(key);
            }
        }

        if self.modifiers.locked(CAPS_LOCK) {
            if let Some(c) = self.caps_lock_keymap.get(base_level, evdev_code) {
                return Some(c);
            }
        }

        self.state_keymap.get(base_level, evdev_code)
    }

    pub fn update_key(&mut self, evdev_code: u32, key_direction: KeyDirection) -> bool {
        let is_modifier = self.modifiers.set_state(evdev_code, key_direction);
        if !is_modifier {
            if key_direction == KeyDirection::Down {
                self.modifiers.unlatch();
            }
            match key_direction {
                KeyDirection::Up => {
                    self.pressed_keys.remove(evdev_code);
                }
                KeyDirection::Down => {
                    self.pressed_keys.insert(evdev_code);
                }
            };
        }
        is_modifier
    }

    pub fn key(&mut self, evdev_code: u32, key_direction: KeyDirection) -> (Option<char>, bool) {
        let is_modifier = self.update_key(evdev_code, key_direction);
        let utf8 = if key_direction == KeyDirection::Down && !is_modifier {
            self.utf8(evdev_code)
        } else {
            None
        };
        (utf8, is_modifier)
    }

    #[cfg(feature = "compose")]
    pub fn key_compose(
        &mut self,
        evdev_code: u32,
        key_direction: KeyDirection,
    ) -> (Option<ComposeState>, bool) {
        let is_modifier = self.update_key(evdev_code, key_direction);
        let compose_state = if key_direction == KeyDirection::Down
            && is_modifier
            && self.modifiers.active_mod_type(ModType::Compose)
        {
            Some(self.composer.feed(Token::Compose))
        } else if key_direction == KeyDirection::Down && !is_modifier {
            self.utf8(evdev_code)
                .map(|c| self.composer.feed(Token::Char(c)))
        } else {
            None
        };
        (compose_state, is_modifier)
    }

    pub fn layouts(&self) -> Vec<String> {
        self.layouts.clone()
    }

    pub fn current_layout(&self) -> String {
        self.layout.clone()
    }
}
