//! Keysym ↔ [`NamedKey`] and evdev → XKB key-name mapping tables.

use crate::named_keys::NamedKey;

/// Map an XKB keysym value to a [`NamedKey`].
///
/// KP variants collapse to their main key equivalents (KP_Enter → Enter).
/// ISO_Left_Tab → Tab, ISO_Enter → Enter.
/// Dead keys and character-producing keys map to `Unnamed`.
const KEYSYM_TO_NAMED_KEY: &[(u32, NamedKey)] = &[
    (0x0020, NamedKey::Space),
    (0xfe20, NamedKey::Tab),
    (0xfe34, NamedKey::Enter),
    (0xff08, NamedKey::Backspace),
    (0xff09, NamedKey::Tab),
    (0xff0d, NamedKey::Enter),
    (0xff13, NamedKey::Pause),
    (0xff14, NamedKey::ScrollLock),
    (0xff15, NamedKey::SysReq),
    (0xff1b, NamedKey::Escape),
    (0xff21, NamedKey::KanjiMode),
    (0xff24, NamedKey::Romaji),
    (0xff25, NamedKey::Hiragana),
    (0xff26, NamedKey::Katakana),
    (0xff2a, NamedKey::ZenkakuHankaku),
    (0xff30, NamedKey::EisuToggle),
    (0xff34, NamedKey::HangulHanja),
    (0xff50, NamedKey::Home),
    (0xff51, NamedKey::ArrowLeft),
    (0xff52, NamedKey::ArrowUp),
    (0xff53, NamedKey::ArrowRight),
    (0xff54, NamedKey::ArrowDown),
    (0xff55, NamedKey::PageUp),
    (0xff56, NamedKey::PageDown),
    (0xff57, NamedKey::End),
    (0xff61, NamedKey::PrintScreen),
    (0xff63, NamedKey::Insert),
    (0xff67, NamedKey::ContextMenu),
    (0xff7f, NamedKey::NumLock),
    (0xff80, NamedKey::Space),
    (0xff89, NamedKey::Tab),
    (0xff8d, NamedKey::Enter),
    (0xff95, NamedKey::Home),
    (0xff96, NamedKey::ArrowLeft),
    (0xff97, NamedKey::ArrowUp),
    (0xff98, NamedKey::ArrowRight),
    (0xff99, NamedKey::ArrowDown),
    (0xff9a, NamedKey::PageUp),
    (0xff9b, NamedKey::PageDown),
    (0xff9c, NamedKey::End),
    (0xff9e, NamedKey::Insert),
    (0xff9f, NamedKey::Delete),
    (0xffbe, NamedKey::F1),
    (0xffbf, NamedKey::F2),
    (0xffc0, NamedKey::F3),
    (0xffc1, NamedKey::F4),
    (0xffc2, NamedKey::F5),
    (0xffc3, NamedKey::F6),
    (0xffc4, NamedKey::F7),
    (0xffc5, NamedKey::F8),
    (0xffc6, NamedKey::F9),
    (0xffc7, NamedKey::F10),
    (0xffc8, NamedKey::F11),
    (0xffc9, NamedKey::F12),
    (0xffca, NamedKey::F13),
    (0xffcb, NamedKey::F14),
    (0xffcc, NamedKey::F15),
    (0xffcd, NamedKey::F16),
    (0xffce, NamedKey::F17),
    (0xffcf, NamedKey::F18),
    (0xffd0, NamedKey::F19),
    (0xffd1, NamedKey::F20),
    (0xffd2, NamedKey::F21),
    (0xffd3, NamedKey::F22),
    (0xffd4, NamedKey::F23),
    (0xffd5, NamedKey::F24),
    (0xffd6, NamedKey::F25),
    (0xffd7, NamedKey::F26),
    (0xffd8, NamedKey::F27),
    (0xffd9, NamedKey::F28),
    (0xffda, NamedKey::F29),
    (0xffdb, NamedKey::F30),
    (0xffdc, NamedKey::F31),
    (0xffdd, NamedKey::F32),
    (0xffde, NamedKey::F33),
    (0xffdf, NamedKey::F34),
    (0xffe0, NamedKey::F35),
    (0xffe1, NamedKey::LeftShift),
    (0xffe2, NamedKey::RightShift),
    (0xffe3, NamedKey::LeftControl),
    (0xffe4, NamedKey::RightControl),
    (0xffe5, NamedKey::CapsLock),
    (0xffe7, NamedKey::LeftMeta),
    (0xffe8, NamedKey::RightMeta),
    (0xffe9, NamedKey::LeftAlt),
    (0xffea, NamedKey::RightAlt),
    (0xffeb, NamedKey::LeftSuper),
    (0xffec, NamedKey::RightSuper),
    (0xffed, NamedKey::LeftHyper),
    (0xffee, NamedKey::RightHyper),
    (0xffff, NamedKey::Delete),
    (0x1008ff02, NamedKey::BrightnessUp),
    (0x1008ff03, NamedKey::BrightnessDown),
    (0x1008ff05, NamedKey::KeyboardBrightnessUp),
    (0x1008ff06, NamedKey::KeyboardBrightnessDown),
    (0x1008ff11, NamedKey::VolumeDown),
    (0x1008ff12, NamedKey::VolumeMute),
    (0x1008ff13, NamedKey::VolumeUp),
    (0x1008ff14, NamedKey::MediaPlay),
    (0x1008ff15, NamedKey::MediaStop),
    (0x1008ff16, NamedKey::MediaPreviousTrack),
    (0x1008ff17, NamedKey::MediaNextTrack),
    (0x1008ff18, NamedKey::BrowserHome),
    (0x1008ff19, NamedKey::LaunchMail),
    (0x1008ff1d, NamedKey::LaunchCalculator),
    (0x1008ff21, NamedKey::Power),
    (0x1008ff26, NamedKey::BrowserBack),
    (0x1008ff27, NamedKey::BrowserForward),
    (0x1008ff29, NamedKey::BrowserRefresh),
    (0x1008ff2a, NamedKey::PowerOff),
    (0x1008ff2b, NamedKey::WakeUp),
    (0x1008ff2f, NamedKey::Sleep),
    (0x1008ff31, NamedKey::MediaPause),
    (0x1008ff80, NamedKey::LaunchTerminal),
    (0x1008ffa7, NamedKey::Suspend),
    (0x1008ffa8, NamedKey::Hibernate),
];

/// Map an XKB keysym value to a [`NamedKey`].
///
/// KP variants collapse to their main key equivalents (KP_Enter → Enter).
/// ISO_Left_Tab → Tab, ISO_Enter → Enter.
/// Dead keys and character-producing keys map to `Unnamed`.
#[doc(hidden)]
pub fn keysym_to_named_key(keysym: u32) -> NamedKey {
    if (0xfe50..=0xfe8d).contains(&keysym) {
        return NamedKey::Unnamed;
    }
    KEYSYM_TO_NAMED_KEY
        .binary_search_by_key(&keysym, |&(ks, _)| ks)
        .ok()
        .map(|i| KEYSYM_TO_NAMED_KEY[i].1)
        .unwrap_or(NamedKey::Unnamed)
}

/// Map a [`NamedKey`] back to its canonical XKB keysym value.
///
/// Returns `0` (NoSymbol) for [`NamedKey::Unnamed`] and for character keys
/// that don't have a canonical keysym.
pub(crate) fn named_key_to_keysym(key: NamedKey) -> u32 {
    const CANONICAL_OVERRIDES: &[(NamedKey, u32)] = &[
        (NamedKey::Enter, 0xff0d),
        (NamedKey::Tab, 0xff09),
        (NamedKey::Delete, 0xffff),
    ];
    if let Some(&(_, ks)) = CANONICAL_OVERRIDES.iter().find(|(nk, _)| *nk == key) {
        return ks;
    }
    KEYSYM_TO_NAMED_KEY
        .iter()
        .find(|(_, nk)| *nk == key)
        .map(|(ks, _)| *ks)
        .unwrap_or(0)
}

/// Generate a stable XKB identifier for an evdev code.
pub(crate) fn evdev_to_keyname(evdev: u32) -> String {
    format!("I{:03}", evdev + 8)
}
