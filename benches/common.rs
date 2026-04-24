/// Shared constants and test matrix for all benchmarks.
///
/// Evdev keycodes used across benchmarks (all backends add +8 for XKB keycodes).

// ── Locales & layouts ──────────────────────────────────────────────────
/// Primary layout — used for all key cases.
pub const PRIMARY_LAYOUT: (&str, Option<&str>) = ("us", None);

/// Extra layouts — only tested with layout-sensitive cases (AltGr, punctuation).
pub const EXTRA_LAYOUTS: &[(&str, Option<&str>)] = &[("de", None), ("us", Some("intl"))];

/// All layouts combined — used by example binaries (memory/size benchmarks).
pub const LAYOUTS: &[(&str, Option<&str>)] = &[
    ("us", None),
    ("de", None),
    ("fr", None),
    ("ru", None),
    ("us", Some("intl")),
];

/// Number of repeated key-presses for memory/size benchmark workloads.
pub const HOT_PATH_ITERATIONS: usize = 100;

/// Cases that produce different results across layouts (AltGr, punctuation, etc.)
pub const LAYOUT_SENSITIVE_CASES: &[&str] = &["altgr_e", "semicolon", "shift_1", "plain_a"];

// ── Evdev key codes ────────────────────────────────────────────────────
pub const KEY_A: u32 = 30;
pub const KEY_B: u32 = 48;
pub const KEY_Z: u32 = 44;
pub const KEY_1: u32 = 2;
pub const KEY_SPACE: u32 = 57;
pub const KEY_LEFT_SHIFT: u32 = 42;
pub const KEY_RIGHT_SHIFT: u32 = 54;
pub const KEY_CAPS_LOCK: u32 = 58;
pub const KEY_LEFT_ALT: u32 = 56;
pub const KEY_RIGHT_ALT: u32 = 100;
pub const KEY_NUM_LOCK: u32 = 69;
pub const KEY_KP_5: u32 = 76;
pub const KEY_LEFT_CTRL: u32 = 29;
pub const KEY_E: u32 = 18;
pub const KEY_F1: u32 = 59;
pub const KEY_SEMICOLON: u32 = 39;
pub const KEY_TAB: u32 = 15;
pub const KEY_SCROLL_LOCK: u32 = 70;

pub const EVDEV_OFFSET: u32 = 8;

/// A named key-press scenario: (name, sequence of (evdev_code, is_down)).
pub struct KeyCase {
    pub name: &'static str,
    pub keys: &'static [(u32, bool)],
}

/// Key cases covering hot paths and edge paths.
pub const KEY_CASES: &[KeyCase] = &[
    // ── Hot paths ──────────────────────────────────────────────────────
    KeyCase {
        name: "plain_a",
        keys: &[(KEY_A, true), (KEY_A, false)],
    },
    KeyCase {
        name: "shift_a",
        keys: &[
            (KEY_LEFT_SHIFT, true),
            (KEY_A, true),
            (KEY_A, false),
            (KEY_LEFT_SHIFT, false),
        ],
    },
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
    // ── Modifier toggles ───────────────────────────────────────────────
    KeyCase {
        name: "caps_a",
        keys: &[
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
            (KEY_A, true),
            (KEY_A, false),
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
        ],
    },
    KeyCase {
        name: "altgr_e",
        keys: &[
            (KEY_RIGHT_ALT, true),
            (KEY_E, true),
            (KEY_E, false),
            (KEY_RIGHT_ALT, false),
        ],
    },
    KeyCase {
        name: "numlock_kp5",
        keys: &[
            (KEY_NUM_LOCK, true),
            (KEY_NUM_LOCK, false),
            (KEY_KP_5, true),
            (KEY_KP_5, false),
            (KEY_NUM_LOCK, true),
            (KEY_NUM_LOCK, false),
        ],
    },
    // ── Edge paths ─────────────────────────────────────────────────────
    KeyCase {
        name: "ctrl_a",
        keys: &[
            (KEY_LEFT_CTRL, true),
            (KEY_A, true),
            (KEY_A, false),
            (KEY_LEFT_CTRL, false),
        ],
    },
    KeyCase {
        name: "ctrl_shift_a",
        keys: &[
            (KEY_LEFT_CTRL, true),
            (KEY_LEFT_SHIFT, true),
            (KEY_A, true),
            (KEY_A, false),
            (KEY_LEFT_SHIFT, false),
            (KEY_LEFT_CTRL, false),
        ],
    },
    KeyCase {
        name: "f1",
        keys: &[(KEY_F1, true), (KEY_F1, false)],
    },
    KeyCase {
        name: "caps_shift_a",
        keys: &[
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
            (KEY_LEFT_SHIFT, true),
            (KEY_A, true),
            (KEY_A, false),
            (KEY_LEFT_SHIFT, false),
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
        ],
    },
    KeyCase {
        name: "shift_1",
        keys: &[
            (KEY_LEFT_SHIFT, true),
            (KEY_1, true),
            (KEY_1, false),
            (KEY_LEFT_SHIFT, false),
        ],
    },
    KeyCase {
        name: "semicolon",
        keys: &[(KEY_SEMICOLON, true), (KEY_SEMICOLON, false)],
    },
    KeyCase {
        name: "rapid_modifiers",
        keys: &[
            (KEY_LEFT_SHIFT, true),
            (KEY_LEFT_SHIFT, false),
            (KEY_LEFT_CTRL, true),
            (KEY_LEFT_CTRL, false),
            (KEY_LEFT_ALT, true),
            (KEY_LEFT_ALT, false),
            (KEY_CAPS_LOCK, true),
            (KEY_CAPS_LOCK, false),
            (KEY_NUM_LOCK, true),
            (KEY_NUM_LOCK, false),
        ],
    },
];

/// Compose sequences to benchmark.
pub struct ComposeSequence {
    pub name: &'static str,
    pub keysyms: &'static [u32],
    pub expected_char: Option<char>,
}

// Keysym constants for compose sequences
pub const XKB_KEY_MULTI_KEY: u32 = 0xff20;
pub const XKB_KEY_APOSTROPHE: u32 = 0x0027;
pub const XKB_KEY_E_LOWER: u32 = 0x0065;
pub const XKB_KEY_U_LOWER: u32 = 0x0075;
pub const XKB_KEY_QUOTEDBL: u32 = 0x0022;
pub const XKB_KEY_ASCIITILDE: u32 = 0x007e;
pub const XKB_KEY_N_LOWER: u32 = 0x006e;
pub const XKB_KEY_SLASH: u32 = 0x002f;
pub const XKB_KEY_EQUAL: u32 = 0x003d;
pub const XKB_KEY_S_LOWER: u32 = 0x0073;
pub const XKB_KEY_LESS: u32 = 0x003c;
pub const XKB_KEY_3: u32 = 0x0033;
pub const XKB_KEY_O_LOWER: u32 = 0x006f;
pub const XKB_KEY_A_LOWER: u32 = 0x0061;
pub const XKB_KEY_ASCIICIRCUM: u32 = 0x005e;
pub const XKB_KEY_C_LOWER: u32 = 0x0063;

pub const COMPOSE_SEQUENCES: &[ComposeSequence] = &[
    ComposeSequence {
        name: "acute_e",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_APOSTROPHE, XKB_KEY_E_LOWER],
        expected_char: Some('é'),
    },
    ComposeSequence {
        name: "diaeresis_u",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_QUOTEDBL, XKB_KEY_U_LOWER],
        expected_char: Some('ü'),
    },
    ComposeSequence {
        name: "tilde_n",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_ASCIITILDE, XKB_KEY_N_LOWER],
        expected_char: Some('ñ'),
    },
    ComposeSequence {
        name: "ss",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_S_LOWER, XKB_KEY_S_LOWER],
        expected_char: Some('ß'),
    },
    ComposeSequence {
        name: "heart",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_LESS, XKB_KEY_3],
        expected_char: Some('♥'),
    },
    // Edge: circumflex + vowels
    ComposeSequence {
        name: "circumflex_o",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_ASCIICIRCUM, XKB_KEY_O_LOWER],
        expected_char: Some('ô'),
    },
    // Edge: cedilla
    ComposeSequence {
        name: "cedilla_c",
        keysyms: &[XKB_KEY_MULTI_KEY, XKB_KEY_COMMA, XKB_KEY_C_LOWER],
        expected_char: Some('ç'),
    },
];

pub const XKB_KEY_COMMA: u32 = 0x002c;

/// Fixed locale for compose benchmarks.
pub const COMPOSE_LOCALE: &str = "en_US.UTF-8";
