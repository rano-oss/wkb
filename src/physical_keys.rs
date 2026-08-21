//! Physical keyboard positions identified by evdev keycode.
//!
//! Names follow the QWERTY / UI Events `code` convention: they describe
//! the key's position, never the character printed on the keycap or the
//! active layout.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum PhysicalKey {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadSubtract,
    Escape,
    Fn,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    LaunchCalculator,
    LaunchTerminal,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    MediaPlay,
    MediaPause,
    Power,
    PowerOff,
    Sleep,
    WakeUp,
    Suspend,
    Hibernate,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    BrightnessUp,
    BrightnessDown,
    KeyboardBrightnessUp,
    KeyboardBrightnessDown,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    ZenkakuHankaku,
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
    /// Evdev code with no known physical position.
    #[default]
    Unidentified,
}

#[rustfmt::skip]
const EVDEV_TO_PHYSICAL: [PhysicalKey; 231] = [
    PhysicalKey::Unidentified, // 0
    PhysicalKey::Escape, // 1
    PhysicalKey::Digit1, // 2
    PhysicalKey::Digit2, // 3
    PhysicalKey::Digit3, // 4
    PhysicalKey::Digit4, // 5
    PhysicalKey::Digit5, // 6
    PhysicalKey::Digit6, // 7
    PhysicalKey::Digit7, // 8
    PhysicalKey::Digit8, // 9
    PhysicalKey::Digit9, // 10
    PhysicalKey::Digit0, // 11
    PhysicalKey::Minus, // 12
    PhysicalKey::Equal, // 13
    PhysicalKey::Backspace, // 14
    PhysicalKey::Tab, // 15
    PhysicalKey::KeyQ, // 16
    PhysicalKey::KeyW, // 17
    PhysicalKey::KeyE, // 18
    PhysicalKey::KeyR, // 19
    PhysicalKey::KeyT, // 20
    PhysicalKey::KeyY, // 21
    PhysicalKey::KeyU, // 22
    PhysicalKey::KeyI, // 23
    PhysicalKey::KeyO, // 24
    PhysicalKey::KeyP, // 25
    PhysicalKey::BracketLeft, // 26
    PhysicalKey::BracketRight, // 27
    PhysicalKey::Enter, // 28
    PhysicalKey::ControlLeft, // 29
    PhysicalKey::KeyA, // 30
    PhysicalKey::KeyS, // 31
    PhysicalKey::KeyD, // 32
    PhysicalKey::KeyF, // 33
    PhysicalKey::KeyG, // 34
    PhysicalKey::KeyH, // 35
    PhysicalKey::KeyJ, // 36
    PhysicalKey::KeyK, // 37
    PhysicalKey::KeyL, // 38
    PhysicalKey::Semicolon, // 39
    PhysicalKey::Quote, // 40
    PhysicalKey::Backquote, // 41
    PhysicalKey::ShiftLeft, // 42
    PhysicalKey::Backslash, // 43
    PhysicalKey::KeyZ, // 44
    PhysicalKey::KeyX, // 45
    PhysicalKey::KeyC, // 46
    PhysicalKey::KeyV, // 47
    PhysicalKey::KeyB, // 48
    PhysicalKey::KeyN, // 49
    PhysicalKey::KeyM, // 50
    PhysicalKey::Comma, // 51
    PhysicalKey::Period, // 52
    PhysicalKey::Slash, // 53
    PhysicalKey::ShiftRight, // 54
    PhysicalKey::NumpadMultiply, // 55
    PhysicalKey::AltLeft, // 56
    PhysicalKey::Space, // 57
    PhysicalKey::CapsLock, // 58
    PhysicalKey::F1, // 59
    PhysicalKey::F2, // 60
    PhysicalKey::F3, // 61
    PhysicalKey::F4, // 62
    PhysicalKey::F5, // 63
    PhysicalKey::F6, // 64
    PhysicalKey::F7, // 65
    PhysicalKey::F8, // 66
    PhysicalKey::F9, // 67
    PhysicalKey::F10, // 68
    PhysicalKey::NumLock, // 69
    PhysicalKey::ScrollLock, // 70
    PhysicalKey::Numpad7, // 71
    PhysicalKey::Numpad8, // 72
    PhysicalKey::Numpad9, // 73
    PhysicalKey::NumpadSubtract, // 74
    PhysicalKey::Numpad4, // 75
    PhysicalKey::Numpad5, // 76
    PhysicalKey::Numpad6, // 77
    PhysicalKey::NumpadAdd, // 78
    PhysicalKey::Numpad1, // 79
    PhysicalKey::Numpad2, // 80
    PhysicalKey::Numpad3, // 81
    PhysicalKey::Numpad0, // 82
    PhysicalKey::NumpadDecimal, // 83
    PhysicalKey::Unidentified, // 84
    PhysicalKey::Lang5, // 85
    PhysicalKey::IntlBackslash, // 86
    PhysicalKey::F11, // 87
    PhysicalKey::F12, // 88
    PhysicalKey::IntlRo, // 89
    PhysicalKey::Lang3, // 90
    PhysicalKey::Lang4, // 91
    PhysicalKey::Convert, // 92
    PhysicalKey::KanaMode, // 93
    PhysicalKey::NonConvert, // 94
    PhysicalKey::Unidentified, // 95
    PhysicalKey::NumpadEnter, // 96
    PhysicalKey::ControlRight, // 97
    PhysicalKey::NumpadDivide, // 98
    PhysicalKey::PrintScreen, // 99
    PhysicalKey::AltRight, // 100
    PhysicalKey::Unidentified, // 101
    PhysicalKey::Home, // 102
    PhysicalKey::ArrowUp, // 103
    PhysicalKey::PageUp, // 104
    PhysicalKey::ArrowLeft, // 105
    PhysicalKey::ArrowRight, // 106
    PhysicalKey::End, // 107
    PhysicalKey::ArrowDown, // 108
    PhysicalKey::PageDown, // 109
    PhysicalKey::Insert, // 110
    PhysicalKey::Delete, // 111
    PhysicalKey::Unidentified, // 112
    PhysicalKey::AudioVolumeMute, // 113
    PhysicalKey::AudioVolumeDown, // 114
    PhysicalKey::AudioVolumeUp, // 115
    PhysicalKey::Power, // 116
    PhysicalKey::NumpadEqual, // 117
    PhysicalKey::Unidentified, // 118
    PhysicalKey::Pause, // 119
    PhysicalKey::Unidentified, // 120
    PhysicalKey::NumpadComma, // 121
    PhysicalKey::Lang1, // 122
    PhysicalKey::Lang2, // 123
    PhysicalKey::IntlYen, // 124
    PhysicalKey::SuperLeft, // 125
    PhysicalKey::SuperRight, // 126
    PhysicalKey::ContextMenu, // 127
    PhysicalKey::Unidentified, // 128
    PhysicalKey::Again, // 129
    PhysicalKey::Props, // 130
    PhysicalKey::Undo, // 131
    PhysicalKey::Unidentified, // 132
    PhysicalKey::Copy, // 133
    PhysicalKey::Open, // 134
    PhysicalKey::Paste, // 135
    PhysicalKey::Find, // 136
    PhysicalKey::Cut, // 137
    PhysicalKey::Help, // 138
    PhysicalKey::Unidentified, // 139
    PhysicalKey::LaunchCalculator, // 140
    PhysicalKey::Unidentified, // 141
    PhysicalKey::Sleep, // 142
    PhysicalKey::WakeUp, // 143
    PhysicalKey::Unidentified, // 144
    PhysicalKey::Unidentified, // 145
    PhysicalKey::Unidentified, // 146
    PhysicalKey::Unidentified, // 147
    PhysicalKey::Unidentified, // 148
    PhysicalKey::Unidentified, // 149
    PhysicalKey::BrowserHome, // 150
    PhysicalKey::Unidentified, // 151
    PhysicalKey::Unidentified, // 152
    PhysicalKey::Unidentified, // 153
    PhysicalKey::Unidentified, // 154
    PhysicalKey::LaunchMail, // 155
    PhysicalKey::BrowserFavorites, // 156
    PhysicalKey::Unidentified, // 157
    PhysicalKey::BrowserBack, // 158
    PhysicalKey::BrowserForward, // 159
    PhysicalKey::Unidentified, // 160
    PhysicalKey::Eject, // 161
    PhysicalKey::Unidentified, // 162
    PhysicalKey::MediaTrackNext, // 163
    PhysicalKey::MediaPlayPause, // 164
    PhysicalKey::MediaTrackPrevious, // 165
    PhysicalKey::MediaStop, // 166
    PhysicalKey::Unidentified, // 167
    PhysicalKey::Unidentified, // 168
    PhysicalKey::Unidentified, // 169
    PhysicalKey::Unidentified, // 170
    PhysicalKey::Unidentified, // 171
    PhysicalKey::BrowserHome, // 172
    PhysicalKey::BrowserRefresh, // 173
    PhysicalKey::Unidentified, // 174
    PhysicalKey::Unidentified, // 175
    PhysicalKey::Unidentified, // 176
    PhysicalKey::Unidentified, // 177
    PhysicalKey::Unidentified, // 178
    PhysicalKey::NumpadParenLeft, // 179
    PhysicalKey::NumpadParenRight, // 180
    PhysicalKey::Unidentified, // 181
    PhysicalKey::Unidentified, // 182
    PhysicalKey::F13, // 183
    PhysicalKey::F14, // 184
    PhysicalKey::F15, // 185
    PhysicalKey::F16, // 186
    PhysicalKey::F17, // 187
    PhysicalKey::F18, // 188
    PhysicalKey::F19, // 189
    PhysicalKey::F20, // 190
    PhysicalKey::F21, // 191
    PhysicalKey::F22, // 192
    PhysicalKey::F23, // 193
    PhysicalKey::F24, // 194
    PhysicalKey::Unidentified, // 195
    PhysicalKey::Unidentified, // 196
    PhysicalKey::Unidentified, // 197
    PhysicalKey::Unidentified, // 198
    PhysicalKey::Unidentified, // 199
    PhysicalKey::Unidentified, // 200
    PhysicalKey::Unidentified, // 201
    PhysicalKey::Unidentified, // 202
    PhysicalKey::Unidentified, // 203
    PhysicalKey::Unidentified, // 204
    PhysicalKey::Suspend, // 205
    PhysicalKey::Unidentified, // 206
    PhysicalKey::MediaPlay, // 207
    PhysicalKey::Unidentified, // 208
    PhysicalKey::Unidentified, // 209
    PhysicalKey::Unidentified, // 210
    PhysicalKey::Unidentified, // 211
    PhysicalKey::Unidentified, // 212
    PhysicalKey::Unidentified, // 213
    PhysicalKey::Unidentified, // 214
    PhysicalKey::Unidentified, // 215
    PhysicalKey::Unidentified, // 216
    PhysicalKey::BrowserSearch, // 217
    PhysicalKey::Unidentified, // 218
    PhysicalKey::Unidentified, // 219
    PhysicalKey::Unidentified, // 220
    PhysicalKey::Unidentified, // 221
    PhysicalKey::Unidentified, // 222
    PhysicalKey::Unidentified, // 223
    PhysicalKey::BrightnessDown, // 224
    PhysicalKey::BrightnessUp, // 225
    PhysicalKey::MediaSelect, // 226
    PhysicalKey::Unidentified, // 227
    PhysicalKey::Unidentified, // 228
    PhysicalKey::KeyboardBrightnessDown, // 229
    PhysicalKey::KeyboardBrightnessUp, // 230
];

impl PhysicalKey {
    /// Map a raw Linux/evdev keycode to a physical key position.
    ///
    /// Unknown codes return [`PhysicalKey::Unidentified`]. This mapping
    /// depends only on the evdev code — never on layout, modifiers, or compose.
    #[inline]
    pub fn from_evdev(evdev_code: u32) -> Self {
        let idx = evdev_code as usize;
        if idx < EVDEV_TO_PHYSICAL.len() {
            EVDEV_TO_PHYSICAL[idx]
        } else {
            PhysicalKey::Unidentified
        }
    }
}
