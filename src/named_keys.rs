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
