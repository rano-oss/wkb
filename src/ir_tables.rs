//! Static tables for [`crate::ir`] sparse export/import.

use crate::modifiers::ModType;
use crate::named_keys::NamedKey;

pub(crate) const STANDARD_REPEAT_MAX: u32 = 700;

#[rustfmt::skip]
pub(crate) const STANDARD_NO_REPEAT: &[u32] = &[
    29, 42, 54, 56, 58, 69, 84, 89, 95, 97, 100, 112, 124, 125, 126, 141, 146, 160, 170, 175,
    176, 189, 194, 195, 196, 197, 198, 199, 209, 211, 214, 222, 240, 249, 250, 251, 252, 253,
    254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271,
    272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289,
    290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307,
    308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343,
    344, 345, 346, 347, 348, 349, 350, 351, 356, 361, 367, 382, 391, 394, 395, 404, 405, 406,
    407, 408, 412, 414, 415, 443, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478,
    479, 480, 481, 482, 483, 484, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 507,
    508, 509, 510, 511, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556,
    557, 558, 559, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 584, 594,
    595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 650, 651, 652, 653, 654,
    655, 686, 687, 694, 695,
];

/// Default level-0 chars omitted from export (numpad/media + universal PC keys).
pub(crate) const DEFAULT_LEVEL0: &[(u32, char)] = &[
    (1, '\u{1b}'), (14, '\u{8}'), (15, '\t'), (28, '\r'), (57, ' '),
    (55, '*'), (74, '-'), (78, '+'), (96, '\r'), (98, '/'), (101, '\n'), (111, '\u{7f}'),
    (117, '='), (118, '±'), (121, '.'), (179, '('), (180, ')'), (434, '$'), (435, '€'),
    (512, '0'), (513, '1'), (514, '2'), (515, '3'), (516, '4'), (517, '5'), (518, '6'),
    (519, '7'), (520, '8'), (521, '9'), (522, '*'), (523, '#'),
];

pub(crate) const STANDARD_NAMED: &[(u8, u32, NamedKey)] = &[
    (0, 71, NamedKey::Home), (0, 72, NamedKey::ArrowUp), (0, 73, NamedKey::PageUp),
    (0, 75, NamedKey::ArrowLeft), (0, 77, NamedKey::ArrowRight), (0, 79, NamedKey::End),
    (0, 80, NamedKey::ArrowDown), (0, 81, NamedKey::PageDown), (0, 82, NamedKey::Insert),
    (0, 83, NamedKey::Delete), (0, 116, NamedKey::PowerOff), (0, 200, NamedKey::MediaPlay),
    (0, 210, NamedKey::PrintScreen), (0, 215, NamedKey::LaunchMail), (1, 99, NamedKey::SysReq),
    (1, 164, NamedKey::MediaPause), (1, 196, NamedKey::LeftAlt), (1, 197, NamedKey::LeftMeta),
    (1, 198, NamedKey::LeftSuper), (1, 199, NamedKey::LeftHyper),
];

pub(crate) const DEFAULT_MODIFIER_KEYS: &[(u32, ModType, bool)] = &[
    (29, ModType::None, false),
    (97, ModType::None, false),
    (58, ModType::Caps, true),
    (69, ModType::Num, true),
];

pub(crate) const MOD_TYPE_NAMED: &[(ModType, NamedKey)] = &[
    (ModType::Caps, NamedKey::CapsLock),
    (ModType::Num, NamedKey::NumLock),
    (ModType::Scroll, NamedKey::ScrollLock),
];

pub(crate) const MOD_KEY_NAMED: &[(ModType, u32, NamedKey)] = &[
    (ModType::Level2, 42, NamedKey::LeftShift),
    (ModType::Level2, 54, NamedKey::RightShift),
    (ModType::None, 29, NamedKey::LeftControl),
    (ModType::None, 97, NamedKey::RightControl),
    (ModType::Level3, 56, NamedKey::LeftAlt),
    (ModType::Level3, 100, NamedKey::RightAlt),
];

#[rustfmt::skip]
pub(crate) const EVDEV_NAMED: &[(u32, NamedKey)] = &[
    (1, NamedKey::Escape), (14, NamedKey::Backspace), (15, NamedKey::Tab), (28, NamedKey::Enter),
    (29, NamedKey::LeftControl), (42, NamedKey::LeftShift), (54, NamedKey::RightShift),
    (56, NamedKey::LeftAlt), (57, NamedKey::Space), (58, NamedKey::CapsLock), (59, NamedKey::F1),
    (60, NamedKey::F2), (61, NamedKey::F3), (62, NamedKey::F4), (63, NamedKey::F5),
    (64, NamedKey::F6), (65, NamedKey::F7), (66, NamedKey::F8), (67, NamedKey::F9),
    (68, NamedKey::F10), (69, NamedKey::NumLock), (70, NamedKey::ScrollLock),
    (85, NamedKey::ZenkakuHankaku), (87, NamedKey::F11), (88, NamedKey::F12),
    (90, NamedKey::Katakana), (91, NamedKey::Hiragana), (92, NamedKey::Romaji),
    (93, NamedKey::KanjiMode), (96, NamedKey::Enter), (97, NamedKey::RightControl),
    (99, NamedKey::PrintScreen), (100, NamedKey::RightAlt), (102, NamedKey::Home),
    (103, NamedKey::ArrowUp), (104, NamedKey::PageUp), (105, NamedKey::ArrowLeft),
    (106, NamedKey::ArrowRight), (107, NamedKey::End), (108, NamedKey::ArrowDown),
    (109, NamedKey::PageDown), (110, NamedKey::Insert), (111, NamedKey::Delete),
    (113, NamedKey::VolumeMute), (114, NamedKey::VolumeDown), (115, NamedKey::VolumeUp),
    (116, NamedKey::Power), (119, NamedKey::Pause), (122, NamedKey::HangulHanja),
    (123, NamedKey::HangulHanja), (125, NamedKey::LeftSuper), (126, NamedKey::RightSuper),
    (127, NamedKey::ContextMenu), (140, NamedKey::LaunchCalculator), (142, NamedKey::Sleep),
    (143, NamedKey::WakeUp), (150, NamedKey::BrowserHome), (155, NamedKey::LaunchMail),
    (158, NamedKey::BrowserBack), (159, NamedKey::BrowserForward), (163, NamedKey::MediaNextTrack),
    (164, NamedKey::MediaPlay), (165, NamedKey::MediaPreviousTrack), (166, NamedKey::MediaStop),
    (172, NamedKey::BrowserHome), (173, NamedKey::BrowserRefresh), (183, NamedKey::F13),
    (184, NamedKey::F14), (185, NamedKey::F15), (186, NamedKey::F16), (187, NamedKey::F17),
    (188, NamedKey::F18), (189, NamedKey::F19), (190, NamedKey::F20), (191, NamedKey::F21),
    (192, NamedKey::F22), (193, NamedKey::F23), (194, NamedKey::F24), (201, NamedKey::MediaPause),
    (205, NamedKey::Suspend), (207, NamedKey::MediaPlay), (224, NamedKey::BrightnessDown),
    (225, NamedKey::BrightnessUp), (229, NamedKey::KeyboardBrightnessDown),
    (230, NamedKey::KeyboardBrightnessUp),
];

pub(crate) fn default_level0_char(keycode: u32) -> Option<char> {
    DEFAULT_LEVEL0
        .iter()
        .find(|(k, _)| *k == keycode)
        .map(|(_, ch)| *ch)
}

pub(crate) fn evdev_named(evdev: u32) -> NamedKey {
    EVDEV_NAMED
        .binary_search_by_key(&evdev, |(k, _)| *k)
        .map(|i| EVDEV_NAMED[i].1)
        .unwrap_or(NamedKey::Unnamed)
}
