/// Shared constants and test matrix for all benchmarks.
///
/// Evdev keycodes used across benchmarks (all backends add +8 for XKB keycodes).
/// Layout cases cover plain, shift, caps-lock, AltGr/level3, numlock variants.

// ── Locales & layouts ──────────────────────────────────────────────────
pub const LAYOUTS: &[(&str, Option<&str>)] = &[
    ("us", None),
    ("de", None),
    ("fr", None),
    ("ru", None),
    ("us", Some("intl")),
];

// ── Evdev key codes ────────────────────────────────────────────────────
pub const KEY_A: u32 = 30;
pub const KEY_B: u32 = 48;
pub const KEY_Z: u32 = 44;
pub const KEY_1: u32 = 2;
pub const KEY_SPACE: u32 = 57;
pub const KEY_ENTER: u32 = 28;
pub const KEY_LEFT_SHIFT: u32 = 42;
pub const KEY_RIGHT_SHIFT: u32 = 54;
pub const KEY_CAPS_LOCK: u32 = 58;
pub const KEY_LEFT_ALT: u32 = 56;
pub const KEY_RIGHT_ALT: u32 = 100; // AltGr on intl layouts
pub const KEY_NUM_LOCK: u32 = 69;
pub const KEY_KP_1: u32 = 79;
pub const KEY_KP_5: u32 = 76;
pub const KEY_LEFT_CTRL: u32 = 29;
pub const KEY_TAB: u32 = 15;
pub const KEY_E: u32 = 18;
pub const KEY_O: u32 = 24;
pub const KEY_U: u32 = 22;
pub const KEY_SEMICOLON: u32 = 39;
pub const KEY_APOSTROPHE: u32 = 40;
pub const KEY_COMMA: u32 = 51;

pub const EVDEV_OFFSET: u32 = 8;

/// A named key-press scenario: (name, sequence of (evdev_code, is_down)).
/// `is_down == true` means press, `false` means release.
pub struct KeyCase {
    pub name: &'static str,
    pub keys: &'static [(u32, bool)],
}

/// All modifier/key combinations to benchmark.
pub const KEY_CASES: &[KeyCase] = &[
    // Plain keys
    KeyCase {
        name: "plain_a",
        keys: &[(KEY_A, true), (KEY_A, false)],
    },
    KeyCase {
        name: "plain_space",
        keys: &[(KEY_SPACE, true), (KEY_SPACE, false)],
    },
    // Shift + key
    KeyCase {
        name: "shift_a",
        keys: &[
            (KEY_LEFT_SHIFT, true),
            (KEY_A, true),
            (KEY_A, false),
            (KEY_LEFT_SHIFT, false),
        ],
    },
    // Caps Lock + key
    KeyCase {
        name: "caps_a",
        keys: &[
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
            (KEY_A, true),
            (KEY_A, false),
            // toggle caps off
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
        ],
    },
    // AltGr / Level3 (evdev 100 = Right Alt = AltGr on intl)
    KeyCase {
        name: "altgr_e",
        keys: &[
            (KEY_RIGHT_ALT, true),
            (KEY_E, true),
            (KEY_E, false),
            (KEY_RIGHT_ALT, false),
        ],
    },
    // NumLock + keypad
    KeyCase {
        name: "numlock_kp5",
        keys: &[
            (KEY_NUM_LOCK, true),
            (KEY_NUM_LOCK, false),
            (KEY_KP_5, true),
            (KEY_KP_5, false),
            // toggle numlock off
            (KEY_NUM_LOCK, true),
            (KEY_NUM_LOCK, false),
        ],
    },
    // Rapid typing: a b z 1 space
    KeyCase {
        name: "rapid_typing",
        keys: &[
            (KEY_A, true),
            (KEY_A, false),
            (KEY_B, true),
            (KEY_B, false),
            (KEY_Z, true),
            (KEY_Z, false),
            (KEY_1, true),
            (KEY_1, false),
            (KEY_SPACE, true),
            (KEY_SPACE, false),
        ],
    },
];

/// Compose sequences to benchmark.
/// Each is a series of keysyms (u32) that form a compose sequence.
/// These are standard X11 compose sequences for en_US.UTF-8.
pub struct ComposeSequence {
    pub name: &'static str,
    pub keysyms: &'static [u32],
    pub expected_char: Option<char>,
}

// Keysym constants for compose sequences
pub const XKB_KEY_MULTI_KEY: u32 = 0xff20; // Multi_key / Compose
pub const XKB_KEY_ACUTE: u32 = 0x00b4; // acute accent
pub const XKB_KEY_APOSTROPHE: u32 = 0x0027;
pub const XKB_KEY_A_LOWER: u32 = 0x0061; // 'a'
pub const XKB_KEY_E_LOWER: u32 = 0x0065; // 'e'
pub const XKB_KEY_O_LOWER: u32 = 0x006f; // 'o'
pub const XKB_KEY_U_LOWER: u32 = 0x0075; // 'u'
pub const XKB_KEY_QUOTEDBL: u32 = 0x0022; // '"'
pub const XKB_KEY_ASCIITILDE: u32 = 0x007e; // '~'
pub const XKB_KEY_N_LOWER: u32 = 0x006e; // 'n'
pub const XKB_KEY_SLASH: u32 = 0x002f; // '/'
pub const XKB_KEY_EQUAL: u32 = 0x003d; // '='
pub const XKB_KEY_S_LOWER: u32 = 0x0073; // 's'
pub const XKB_KEY_LESS: u32 = 0x003c; // '<'
pub const XKB_KEY_3: u32 = 0x0033; // '3'

pub const COMPOSE_SEQUENCES: &[ComposeSequence] = &[
    // Compose + ' + e  →  é
    ComposeSequence {
        name: "compose_acute_e",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_APOSTROPHE, XKB_KEY_E_LOWER],
        expected_char: Some('é'),
    },
    // Compose + " + u  →  ü
    ComposeSequence {
        name: "compose_diaeresis_u",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_QUOTEDBL, XKB_KEY_U_LOWER],
        expected_char: Some('ü'),
    },
    // Compose + ~ + n  →  ñ
    ComposeSequence {
        name: "compose_tilde_n",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_ASCIITILDE, XKB_KEY_N_LOWER],
        expected_char: Some('ñ'),
    },
    // Compose + / + =  →  ≠  (may not exist in all tables)
    ComposeSequence {
        name: "compose_slash_equal",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_SLASH, XKB_KEY_EQUAL],
        expected_char: None, // varies by locale
    },
    // Compose + s + s  →  ß
    ComposeSequence {
        name: "compose_ss",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_S_LOWER, XKB_KEY_S_LOWER],
        expected_char: Some('ß'),
    },
    // Compose + < + 3  →  ♥
    ComposeSequence {
        name: "compose_heart",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_LESS, XKB_KEY_3],
        expected_char: Some('♥'),
    },
];

/// Fixed locale for compose benchmarks.
pub const COMPOSE_LOCALE: &str = "en_US.UTF-8";

/// Number of repeated key-presses for hot-path benchmarks.
pub const HOT_PATH_ITERATIONS: usize = 100;
