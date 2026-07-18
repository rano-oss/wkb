//! Semantic names for keys that are not represented solely by produced text.
//!
//! `NamedKey` describes the logical identity of a key. It does not describe
//! its physical location or necessarily the text produced by pressing it.
//!
//! For example, the Space key may have:
//!
//! - `NamedKey::Space` as its semantic identity
//! - U+0020 SPACE, U+00A0 NO-BREAK SPACE, or another character as text output
//!
//! Ordinary character keys normally use `NamedKey::Unnamed`.
//!
//! KP variants (KP_Enter, KP_Delete, etc.) collapse to their main key equivalents.
//! ISO_Left_Tab collapses to Tab, ISO_Enter collapses to Enter.
//! Dead keys and character-producing keys map to `Unnamed`.

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[non_exhaustive]
pub enum NamedKey {
    /// The key has no separate semantic name.
    ///
    /// This is used for ordinary character-producing keys, dead keys,
    /// and any key without a distinct functional identity.
    #[default]
    Unnamed,

    // Navigation and editing (15)
    Space,
    Enter,
    Tab,
    Backspace,
    Escape,
    Delete,
    Insert,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    PageUp,
    PageDown,

    // Function keys (35)
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,

    // Modifiers with L/R distinction (12)
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftMeta,
    RightMeta,
    LeftSuper,
    RightSuper,
    LeftHyper,
    RightHyper,

    // Locks (3)
    CapsLock,
    NumLock,
    ScrollLock,

    // System (4)
    PrintScreen,
    Pause,
    SysReq,
    ContextMenu,

    // Power (6)
    Power,
    PowerOff,
    Sleep,
    WakeUp,
    Suspend,
    Hibernate,

    // Media (8)
    MediaPlay,
    MediaPause,
    MediaStop,
    MediaNextTrack,
    MediaPreviousTrack,
    VolumeUp,
    VolumeDown,
    VolumeMute,

    // Browser (4)
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserHome,

    // Launch (3)
    LaunchMail,
    LaunchCalculator,
    LaunchTerminal,

    // Display (4)
    BrightnessUp,
    BrightnessDown,
    KeyboardBrightnessUp,
    KeyboardBrightnessDown,

    // Japanese input (7)
    KanjiMode,
    Hiragana,
    Katakana,
    Romaji,
    ZenkakuHankaku,
    EisuToggle,

    // Korean input (1)
    HangulHanja,
}

/// Map an XKB keysym value to a [`NamedKey`].
///
/// KP variants collapse to their main key equivalents (KP_Enter → Enter).
/// ISO_Left_Tab → Tab, ISO_Enter → Enter.
/// Dead keys and character-producing keys map to `Unnamed`.
#[inline]
pub fn keysym_to_named_key(keysym: u32) -> NamedKey {
    match keysym {
        // Navigation and editing
        0x0020 => NamedKey::Space,
        0xff09 => NamedKey::Tab,
        0xff08 => NamedKey::Backspace,
        0xff0d => NamedKey::Enter,
        0xff1b => NamedKey::Escape,
        0xffff => NamedKey::Delete,
        0xff63 => NamedKey::Insert,
        0xff51 => NamedKey::ArrowLeft,
        0xff53 => NamedKey::ArrowRight,
        0xff52 => NamedKey::ArrowUp,
        0xff54 => NamedKey::ArrowDown,
        0xff50 => NamedKey::Home,
        0xff57 => NamedKey::End,
        0xff55 => NamedKey::PageUp,
        0xff56 => NamedKey::PageDown,

        // Function keys
        0xffbe => NamedKey::F1,
        0xffbf => NamedKey::F2,
        0xffc0 => NamedKey::F3,
        0xffc1 => NamedKey::F4,
        0xffc2 => NamedKey::F5,
        0xffc3 => NamedKey::F6,
        0xffc4 => NamedKey::F7,
        0xffc5 => NamedKey::F8,
        0xffc6 => NamedKey::F9,
        0xffc7 => NamedKey::F10,
        0xffc8 => NamedKey::F11,
        0xffc9 => NamedKey::F12,
        0xffca => NamedKey::F13,
        0xffcb => NamedKey::F14,
        0xffcc => NamedKey::F15,
        0xffcd => NamedKey::F16,
        0xffce => NamedKey::F17,
        0xffcf => NamedKey::F18,
        0xffd0 => NamedKey::F19,
        0xffd1 => NamedKey::F20,
        0xffd2 => NamedKey::F21,
        0xffd3 => NamedKey::F22,
        0xffd4 => NamedKey::F23,
        0xffd5 => NamedKey::F24,
        0xffd6 => NamedKey::F25,
        0xffd7 => NamedKey::F26,
        0xffd8 => NamedKey::F27,
        0xffd9 => NamedKey::F28,
        0xffda => NamedKey::F29,
        0xffdb => NamedKey::F30,
        0xffdc => NamedKey::F31,
        0xffdd => NamedKey::F32,
        0xffde => NamedKey::F33,
        0xffdf => NamedKey::F34,
        0xffe0 => NamedKey::F35,

        // Modifiers
        0xffe1 => NamedKey::LeftShift,
        0xffe2 => NamedKey::RightShift,
        0xffe3 => NamedKey::LeftControl,
        0xffe4 => NamedKey::RightControl,
        0xffe9 => NamedKey::LeftAlt,
        0xffea => NamedKey::RightAlt,
        0xffe7 => NamedKey::LeftMeta,
        0xffe8 => NamedKey::RightMeta,
        0xffeb => NamedKey::LeftSuper,
        0xffec => NamedKey::RightSuper,
        0xffed => NamedKey::LeftHyper,
        0xffee => NamedKey::RightHyper,

        // Locks
        0xffe5 => NamedKey::CapsLock,
        0xff7f => NamedKey::NumLock,
        0xff14 => NamedKey::ScrollLock,

        // System
        0xff61 => NamedKey::PrintScreen,
        0xff13 => NamedKey::Pause,
        0xff15 => NamedKey::SysReq,
        0xff67 => NamedKey::ContextMenu,

        // Power (XF86)
        0x1008ff21 => NamedKey::Power,
        0x1008ff2a => NamedKey::PowerOff,
        0x1008ff2f => NamedKey::Sleep,
        0x1008ff2b => NamedKey::WakeUp,
        0x1008ffa7 => NamedKey::Suspend,
        0x1008ffa8 => NamedKey::Hibernate,

        // Media (XF86)
        0x1008ff14 => NamedKey::MediaPlay,
        0x1008ff31 => NamedKey::MediaPause,
        0x1008ff15 => NamedKey::MediaStop,
        0x1008ff17 => NamedKey::MediaNextTrack,
        0x1008ff16 => NamedKey::MediaPreviousTrack,
        0x1008ff13 => NamedKey::VolumeUp,
        0x1008ff11 => NamedKey::VolumeDown,
        0x1008ff12 => NamedKey::VolumeMute,

        // Browser (XF86)
        0x1008ff26 => NamedKey::BrowserBack,
        0x1008ff27 => NamedKey::BrowserForward,
        0x1008ff29 => NamedKey::BrowserRefresh,
        0x1008ff18 => NamedKey::BrowserHome,

        // Launch (XF86)
        0x1008ff19 => NamedKey::LaunchMail,
        0x1008ff1d => NamedKey::LaunchCalculator,
        0x1008ff80 => NamedKey::LaunchTerminal,

        // Display (XF86)
        0x1008ff02 => NamedKey::BrightnessUp,
        0x1008ff03 => NamedKey::BrightnessDown,
        0x1008ff05 => NamedKey::KeyboardBrightnessUp,
        0x1008ff06 => NamedKey::KeyboardBrightnessDown,

        // Japanese input
        0xff21 => NamedKey::KanjiMode,
        0xff25 => NamedKey::Hiragana,
        0xff26 => NamedKey::Katakana,
        0xff24 => NamedKey::Romaji,
        0xff2a => NamedKey::ZenkakuHankaku,
        0xff30 => NamedKey::EisuToggle,

        // Korean input
        0xff34 => NamedKey::HangulHanja,

        // KP → main key collapse
        0xff80 => NamedKey::Space,      // KP_Space
        0xff8d => NamedKey::Enter,      // KP_Enter
        0xff89 => NamedKey::Tab,        // KP_Tab
        0xff9f => NamedKey::Delete,     // KP_Delete
        0xff9e => NamedKey::Insert,     // KP_Insert
        0xff95 => NamedKey::Home,       // KP_Home
        0xff9c => NamedKey::End,        // KP_End
        0xff9a => NamedKey::PageUp,     // KP_Prior
        0xff9b => NamedKey::PageDown,   // KP_Next
        0xff97 => NamedKey::ArrowUp,    // KP_Up
        0xff99 => NamedKey::ArrowDown,  // KP_Down
        0xff96 => NamedKey::ArrowLeft,  // KP_Left
        0xff98 => NamedKey::ArrowRight, // KP_Right

        // ISO → main key collapse
        0xfe20 => NamedKey::Tab,   // ISO_Left_Tab
        0xfe34 => NamedKey::Enter, // ISO_Enter

        // Dead keys → Unnamed (handled internally by compose)
        0xfe50..=0xfe8d => NamedKey::Unnamed,

        // Everything else (character keys, KP digits, etc.) → Unnamed
        _ => NamedKey::Unnamed,
    }
}
