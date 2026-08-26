#![cfg(feature = "client")]
//! Full-flow compose integration tests.
//!
//! Drives `WKB::compose` and compares the final produced character against xkbcommon.

use std::path::Path;
use std::sync::Mutex;
use wkb::WKB;
use xkbcommon::xkb::{self, Keycode};

const EVDEV_OFFSET: u32 = 8;
const COMPOSE_LOCALE: &str = "en_US.UTF-8";
const COMPOSE_FILE: &str = "/usr/share/X11/locale/en_US.UTF-8/Compose";

/// Guard for env-var mutations (LC_ALL) during WKB construction.
/// `set_var` / `remove_var` are process-wide and not thread-safe,
/// so parallel tests must serialize around them.
static ENV_LOCK: Mutex<()> = Mutex::new(());

// ── Keycodes (identical on the custom keymap and the standard US layout) ──
const COMPOSE_KEY: u32 = 119; // Menu key, keycode 127
const SHIFT: u32 = 42;
const APOSTROPHE: u32 = 40; // ' / "
const GRAVE: u32 = 41; // ` / ~
const COMMA: u32 = 51; // , / <
const KEY_3: u32 = 4;
const KEY_6: u32 = 7;
const E: u32 = 18;
const U: u32 = 22;
const N: u32 = 49;
const S: u32 = 31;
const O: u32 = 24;
const C: u32 = 46;

struct ComposeCase {
    name: &'static str,
    keys: &'static [(u32, bool)],
    expected: char,
}

/// Compose sequences (Shift applied via `update_modifiers` where needed).
const COMPOSE_CASES: &[ComposeCase] = &[
    ComposeCase {
        name: "acute_e",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (APOSTROPHE, true),
            (APOSTROPHE, false),
            (E, true),
            (E, false),
        ],
        expected: 'é',
    },
    ComposeCase {
        name: "diaeresis_u",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (SHIFT, true),
            (APOSTROPHE, true),
            (APOSTROPHE, false),
            (SHIFT, false),
            (U, true),
            (U, false),
        ],
        expected: 'ü',
    },
    ComposeCase {
        name: "tilde_n",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (SHIFT, true),
            (GRAVE, true),
            (GRAVE, false),
            (SHIFT, false),
            (N, true),
            (N, false),
        ],
        expected: 'ñ',
    },
    ComposeCase {
        name: "ss",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (S, true),
            (S, false),
            (S, true),
            (S, false),
        ],
        expected: 'ß',
    },
    ComposeCase {
        name: "circumflex_o",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (SHIFT, true),
            (KEY_6, true),
            (KEY_6, false),
            (SHIFT, false),
            (O, true),
            (O, false),
        ],
        expected: 'ô',
    },
    ComposeCase {
        name: "cedilla_c",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (COMMA, true),
            (COMMA, false),
            (C, true),
            (C, false),
        ],
        expected: 'ç',
    },
    ComposeCase {
        name: "heart",
        keys: &[
            (COMPOSE_KEY, true),
            (COMPOSE_KEY, false),
            (SHIFT, true),
            (COMMA, true),
            (COMMA, false),
            (SHIFT, false),
            (KEY_3, true),
            (KEY_3, false),
        ],
        expected: '♥',
    },
];

const MOD_SHIFT: u32 = 1;

/// Feed a compose sequence via `update_modifiers` + `compose`.
fn wkb_compose_char(wkb: &mut WKB, keys: &[(u32, bool)]) -> Option<char> {
    let mut mods;
    let mut final_char = None;
    for &(evdev, down) in keys {
        if evdev == SHIFT {
            mods = if down { MOD_SHIFT } else { 0 };
            wkb.update_modifiers(mods, 0, 0, 0);
            continue;
        }
        if down {
            let result = wkb.compose(evdev);
            if let Some(wkb::ComposeState::Finished(c)) = &result {
                final_char = Some(*c);
            }
        }
    }
    final_char
}

/// Feed the same key events through xkbcommon's keymap state + compose state.
/// Returns the final composed character. Modifier keysyms are not fed to the
/// compose state (matching compositor behavior).
///
/// `compose_kc` designates a keycode to treat as the compose key (feeding the
/// `Multi_key` keysym), mirroring `WKB::set_compose_key`.
fn xkb_compose_char(
    state: &mut xkb::State,
    compose_state: &mut xkb::compose::State,
    keys: &[(u32, bool)],
    compose_kc: Option<Keycode>,
) -> Option<char> {
    let mut final_char = None;
    for &(evdev, down) in keys {
        let kc = Keycode::new(evdev + EVDEV_OFFSET);
        let dir = if down {
            xkb::KeyDirection::Down
        } else {
            xkb::KeyDirection::Up
        };
        state.update_key(kc, dir);
        if !down {
            continue;
        }
        let sym = state.key_get_one_sym(kc);
        if is_modifier_keysym(sym.raw()) {
            continue;
        }
        let feed = if sym.raw() == XKB_KEY_MULTI_KEY || Some(kc) == compose_kc {
            xkb::Keysym::new(XKB_KEY_MULTI_KEY)
        } else {
            sym
        };
        compose_state.feed(feed);
        if compose_state.status() == xkb::compose::Status::Composed {
            final_char = compose_state.utf8().and_then(|s| s.chars().next());
        }
    }
    final_char
}

const XKB_KEY_MULTI_KEY: u32 = 0xff20;

fn is_modifier_keysym(keysym: u32) -> bool {
    (0xffe1..=0xffee).contains(&keysym) || keysym == 0xff7f // Shift..Hyper, NumLock
}

fn skip_unless_compose_file() -> bool {
    if Path::new(COMPOSE_FILE).exists() {
        true
    } else {
        println!("SKIP: compose file not found: {COMPOSE_FILE}");
        false
    }
}

/// Minimal keymap with a `Multi_key` key plus the letter/punctuation keys the
/// compose cases need. Used to exercise the auto-detected compose key path.
fn keymap_with_multi_key() -> &'static str {
    "xkb_keymap {
    xkb_keycodes {
        minimum = 8;
        maximum = 255;
        <COMP> = 127;
        <AC01> = 38;
        <AD03> = 26;
        <AD07> = 30;
        <AB06> = 57;
        <AC02> = 39;
        <AD09> = 32;
        <AB03> = 54;
        <AC11> = 48;
        <AB08> = 59;
        <AE03> = 12;
        <AE06> = 15;
        <TLDE> = 49;
        <LFSH> = 50;
    };
    xkb_types {
        type \"ONE_LEVEL\" {
            modifiers = None;
            map[None] = Level1;
            level_name[Level1] = \"Base\";
        };
        type \"TWO_LEVEL\" {
            modifiers = Shift;
            map[None] = Level1;
            map[Shift] = Level2;
            level_name[Level1] = \"Base\";
            level_name[Level2] = \"Shift\";
        };
    };
    xkb_compat {
        interpret Shift_L+AnyOfOrNone(all) {
            action = SetMods(modifiers=Shift);
        };
    };
    xkb_symbols {
        key <COMP> { type= \"ONE_LEVEL\", [ Multi_key ] };
        key <AC01> { type= \"TWO_LEVEL\", [ a, A ] };
        key <AD03> { type= \"TWO_LEVEL\", [ e, E ] };
        key <AD07> { type= \"TWO_LEVEL\", [ u, U ] };
        key <AB06> { type= \"TWO_LEVEL\", [ n, N ] };
        key <AC02> { type= \"TWO_LEVEL\", [ s, S ] };
        key <AD09> { type= \"TWO_LEVEL\", [ o, O ] };
        key <AB03> { type= \"TWO_LEVEL\", [ c, C ] };
        key <AC11> { type= \"TWO_LEVEL\", [ apostrophe, quotedbl ] };
        key <AB08> { type= \"TWO_LEVEL\", [ comma, less ] };
        key <AE03> { type= \"TWO_LEVEL\", [ 3, numbersign ] };
        key <AE06> { type= \"TWO_LEVEL\", [ 6, asciicircum ] };
        key <TLDE> { type= \"TWO_LEVEL\", [ grave, asciitilde ] };
        key <LFSH> { [ Shift_L ] };
        modifier_map Shift { <LFSH> };
    };
};"
}

/// Build a WKB (and xkbcommon keymap + compose state) for the given keymap
/// string, then compare the final composed char for every compose case.
fn compare_compose_flow(label: &str, keymap_str: &str, wkb: &mut WKB) {
    if !skip_unless_compose_file() {
        return;
    }

    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_string(
        &ctx,
        keymap_str.to_string(),
        xkb::KEYMAP_FORMAT_TEXT_V1,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .expect("xkbcommon keymap");
    let mut state = xkb::State::new(&keymap);

    let table = xkb::compose::Table::new_from_locale(
        &ctx,
        std::ffi::OsStr::new(COMPOSE_LOCALE),
        xkb::compose::COMPILE_NO_FLAGS,
    )
    .expect("xkbcommon compose table");
    let mut compose_state = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);

    for case in COMPOSE_CASES {
        let wkb_char = wkb_compose_char(wkb, case.keys);
        let xkb_char = xkb_compose_char(&mut state, &mut compose_state, case.keys, None);
        compose_state.reset();

        assert_eq!(
            wkb_char,
            xkb_char,
            "{label}/{}: wkb={:?} xkb={:?} (expected {:?})",
            case.name,
            wkb_char,
            xkb_char,
            Some(case.expected)
        );
        assert_eq!(
            wkb_char,
            Some(case.expected),
            "{label}/{}: wkb produced wrong char",
            case.name
        );
    }
}

/// Auto-detect path: keymap maps the Menu key to `Multi_key`, so WKB detects
/// the compose key from the keymap and `compose` advances the sequence.
#[test]
fn compose_auto_detected_multi_key() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    let saved_lc_all = std::env::var("LC_ALL").ok();
    unsafe { std::env::set_var("LC_ALL", COMPOSE_LOCALE) };
    let mut wkb = WKB::new_from_string(keymap_with_multi_key()).unwrap();
    restore_env(&saved_lc_all);

    compare_compose_flow("auto-detect", keymap_with_multi_key(), &mut wkb);
}

/// Explicit path: the standard US layout has no `Multi_key` key, so the test
/// designates one via `WKB::set_compose_key`.
#[test]
fn compose_set_compose_key() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    let saved_lc_all = std::env::var("LC_ALL").ok();
    unsafe { std::env::set_var("LC_ALL", COMPOSE_LOCALE) };
    let mut wkb = WKB::new_from_names("", "", "us", "", None).unwrap();
    wkb.set_compose_key(COMPOSE_KEY);
    restore_env(&saved_lc_all);

    // xkbcommon needs the same keymap; there is no Multi_key key, so the
    // designated compose keycode is translated to the Multi_key keysym.
    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap = xkb::Keymap::new_from_names(
        &ctx,
        "evdev",
        "",
        "us",
        "",
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .expect("xkbcommon keymap");
    let mut state = xkb::State::new(&keymap);
    let table = xkb::compose::Table::new_from_locale(
        &ctx,
        std::ffi::OsStr::new(COMPOSE_LOCALE),
        xkb::compose::COMPILE_NO_FLAGS,
    )
    .expect("xkbcommon compose table");
    let mut compose_state = xkb::compose::State::new(&table, xkb::compose::STATE_NO_FLAGS);
    let compose_kc = Keycode::new(COMPOSE_KEY + EVDEV_OFFSET);

    for case in COMPOSE_CASES {
        let wkb_char = wkb_compose_char(&mut wkb, case.keys);
        let xkb_char =
            xkb_compose_char(&mut state, &mut compose_state, case.keys, Some(compose_kc));
        compose_state.reset();

        assert_eq!(
            wkb_char,
            xkb_char,
            "set_compose_key/{}: wkb={:?} xkb={:?} (expected {:?})",
            case.name,
            wkb_char,
            xkb_char,
            Some(case.expected)
        );
        assert_eq!(
            wkb_char,
            Some(case.expected),
            "set_compose_key/{}: wkb produced wrong char",
            case.name
        );
    }
}

fn restore_env(saved: &Option<String>) {
    match saved {
        Some(v) => unsafe { std::env::set_var("LC_ALL", v) },
        None => unsafe { std::env::remove_var("LC_ALL") },
    }
}
