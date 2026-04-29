//! Human-readable RON serialization format for WKB.
//!
//! Converts between the internal flat-array representation and a sparse,
//! named representation that humans can read and edit.

use crate::bitset::{KeyBitSet, BITSET_WORDS};
use crate::composer::{Composer, Token, TrieNode};
use crate::flat_keymap::{FlatKeymap, MAX_LEVELS};
use crate::keysyms;
use crate::modifiers::{ModKind, ModType, Modifier, Modifiers};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ── Default keys ─────────────────────────────────────────────────────
//
// Invariant key tables: keys identical (or >99% identical) across all 594 XKB layouts.
// Omitted from .ron files when they match. Repopulated on deserialize.

/// Keys that are invariant (or >99% invariant) across all XKB layouts.
/// Format: (evdev_code, &[(level, char)]).
/// During RON serialization, entries matching this table are omitted.
/// During RON deserialization, these are pre-populated before applying overrides.
static INVARIANT_CHARS: &[(u32, &[(u8, char)])] = &[
    (
        1,
        &[
            (0, '\u{001B}'),
            (1, '\u{001B}'),
            (2, '\u{001B}'),
            (3, '\u{001B}'),
            (4, '\u{001B}'),
            (5, '\u{001B}'),
            (6, '\u{001B}'),
            (7, '\u{001B}'),
        ],
    ),
    (
        14,
        &[
            (0, '\u{0008}'),
            (1, '\u{0008}'),
            (2, '\u{0008}'),
            (3, '\u{0008}'),
            (4, '\u{0008}'),
            (5, '\u{0008}'),
            (6, '\u{0008}'),
            (7, '\u{0008}'),
        ],
    ),
    (
        15,
        &[
            (0, '\u{0009}'),
            (2, '\u{0009}'),
            (4, '\u{0009}'),
            (6, '\u{0009}'),
        ],
    ),
    (
        28,
        &[
            (0, '\u{000D}'),
            (1, '\u{000D}'),
            (2, '\u{000D}'),
            (3, '\u{000D}'),
            (4, '\u{000D}'),
            (5, '\u{000D}'),
            (6, '\u{000D}'),
            (7, '\u{000D}'),
        ],
    ),
    (55, &[(0, '*'), (4, '*')]),
    (57, &[(0, ' ')]),
    (71, &[(1, '7')]),
    (72, &[(1, '8')]),
    (73, &[(1, '9')]),
    (74, &[(0, '-'), (4, '-')]),
    (
        78,
        &[(0, '+'), (1, '+'), (2, '+'), (4, '+'), (5, '+'), (6, '+')],
    ),
    (80, &[(1, '2')]),
    (81, &[(1, '3')]),
    (
        96,
        &[
            (0, '\u{000D}'),
            (1, '\u{000D}'),
            (2, '\u{000D}'),
            (3, '\u{000D}'),
            (4, '\u{000D}'),
            (5, '\u{000D}'),
            (6, '\u{000D}'),
            (7, '\u{000D}'),
        ],
    ),
    (98, &[(0, '/'), (4, '/')]),
    (
        101,
        &[
            (0, '\u{000A}'),
            (1, '\u{000A}'),
            (2, '\u{000A}'),
            (3, '\u{000A}'),
            (4, '\u{000A}'),
            (5, '\u{000A}'),
            (6, '\u{000A}'),
            (7, '\u{000A}'),
        ],
    ),
    (
        111,
        &[
            (0, '\u{007F}'),
            (1, '\u{007F}'),
            (2, '\u{007F}'),
            (3, '\u{007F}'),
            (4, '\u{007F}'),
            (5, '\u{007F}'),
            (6, '\u{007F}'),
            (7, '\u{007F}'),
        ],
    ),
    (
        117,
        &[
            (0, '='),
            (1, '='),
            (2, '='),
            (3, '='),
            (4, '='),
            (5, '='),
            (6, '='),
            (7, '='),
        ],
    ),
    (
        118,
        &[
            (0, '±'),
            (1, '±'),
            (2, '±'),
            (3, '±'),
            (4, '±'),
            (5, '±'),
            (6, '±'),
            (7, '±'),
        ],
    ),
    (
        121,
        &[
            (0, '.'),
            (1, '.'),
            (2, '.'),
            (3, '.'),
            (4, '.'),
            (5, '.'),
            (6, '.'),
            (7, '.'),
        ],
    ),
    (
        179,
        &[
            (0, '('),
            (1, '('),
            (2, '('),
            (3, '('),
            (4, '('),
            (5, '('),
            (6, '('),
            (7, '('),
        ],
    ),
    (
        180,
        &[
            (0, ')'),
            (1, ')'),
            (2, ')'),
            (3, ')'),
            (4, ')'),
            (5, ')'),
            (6, ')'),
            (7, ')'),
        ],
    ),
    (
        434,
        &[
            (0, '$'),
            (1, '$'),
            (2, '$'),
            (3, '$'),
            (4, '$'),
            (5, '$'),
            (6, '$'),
            (7, '$'),
        ],
    ),
    (
        435,
        &[
            (0, '€'),
            (1, '€'),
            (2, '€'),
            (3, '€'),
            (4, '€'),
            (5, '€'),
            (6, '€'),
            (7, '€'),
        ],
    ),
    (
        497,
        &[
            (0, '⠁'),
            (1, '⠁'),
            (2, '⠁'),
            (3, '⠁'),
            (4, '⠁'),
            (5, '⠁'),
            (6, '⠁'),
            (7, '⠁'),
        ],
    ),
    (
        498,
        &[
            (0, '⠂'),
            (1, '⠂'),
            (2, '⠂'),
            (3, '⠂'),
            (4, '⠂'),
            (5, '⠂'),
            (6, '⠂'),
            (7, '⠂'),
        ],
    ),
    (
        499,
        &[
            (0, '⠄'),
            (1, '⠄'),
            (2, '⠄'),
            (3, '⠄'),
            (4, '⠄'),
            (5, '⠄'),
            (6, '⠄'),
            (7, '⠄'),
        ],
    ),
    (
        500,
        &[
            (0, '⠈'),
            (1, '⠈'),
            (2, '⠈'),
            (3, '⠈'),
            (4, '⠈'),
            (5, '⠈'),
            (6, '⠈'),
            (7, '⠈'),
        ],
    ),
    (
        501,
        &[
            (0, '⠐'),
            (1, '⠐'),
            (2, '⠐'),
            (3, '⠐'),
            (4, '⠐'),
            (5, '⠐'),
            (6, '⠐'),
            (7, '⠐'),
        ],
    ),
    (
        502,
        &[
            (0, '⠠'),
            (1, '⠠'),
            (2, '⠠'),
            (3, '⠠'),
            (4, '⠠'),
            (5, '⠠'),
            (6, '⠠'),
            (7, '⠠'),
        ],
    ),
    (
        503,
        &[
            (0, '⡀'),
            (1, '⡀'),
            (2, '⡀'),
            (3, '⡀'),
            (4, '⡀'),
            (5, '⡀'),
            (6, '⡀'),
            (7, '⡀'),
        ],
    ),
    (
        504,
        &[
            (0, '⢀'),
            (1, '⢀'),
            (2, '⢀'),
            (3, '⢀'),
            (4, '⢀'),
            (5, '⢀'),
            (6, '⢀'),
            (7, '⢀'),
        ],
    ),
    (
        506,
        &[
            (0, '⠁'),
            (1, '⠁'),
            (2, '⠁'),
            (3, '⠁'),
            (4, '⠁'),
            (5, '⠁'),
            (6, '⠁'),
            (7, '⠁'),
        ],
    ),
    (
        512,
        &[
            (0, '0'),
            (1, '0'),
            (2, '0'),
            (3, '0'),
            (4, '0'),
            (5, '0'),
            (6, '0'),
            (7, '0'),
        ],
    ),
    (
        513,
        &[
            (0, '1'),
            (1, '1'),
            (2, '1'),
            (3, '1'),
            (4, '1'),
            (5, '1'),
            (6, '1'),
            (7, '1'),
        ],
    ),
    (
        514,
        &[
            (0, '2'),
            (1, '2'),
            (2, '2'),
            (3, '2'),
            (4, '2'),
            (5, '2'),
            (6, '2'),
            (7, '2'),
        ],
    ),
    (
        515,
        &[
            (0, '3'),
            (1, '3'),
            (2, '3'),
            (3, '3'),
            (4, '3'),
            (5, '3'),
            (6, '3'),
            (7, '3'),
        ],
    ),
    (
        516,
        &[
            (0, '4'),
            (1, '4'),
            (2, '4'),
            (3, '4'),
            (4, '4'),
            (5, '4'),
            (6, '4'),
            (7, '4'),
        ],
    ),
    (
        517,
        &[
            (0, '5'),
            (1, '5'),
            (2, '5'),
            (3, '5'),
            (4, '5'),
            (5, '5'),
            (6, '5'),
            (7, '5'),
        ],
    ),
    (
        518,
        &[
            (0, '6'),
            (1, '6'),
            (2, '6'),
            (3, '6'),
            (4, '6'),
            (5, '6'),
            (6, '6'),
            (7, '6'),
        ],
    ),
    (
        519,
        &[
            (0, '7'),
            (1, '7'),
            (2, '7'),
            (3, '7'),
            (4, '7'),
            (5, '7'),
            (6, '7'),
            (7, '7'),
        ],
    ),
    (
        520,
        &[
            (0, '8'),
            (1, '8'),
            (2, '8'),
            (3, '8'),
            (4, '8'),
            (5, '8'),
            (6, '8'),
            (7, '8'),
        ],
    ),
    (
        521,
        &[
            (0, '9'),
            (1, '9'),
            (2, '9'),
            (3, '9'),
            (4, '9'),
            (5, '9'),
            (6, '9'),
            (7, '9'),
        ],
    ),
    (
        522,
        &[
            (0, '*'),
            (1, '*'),
            (2, '*'),
            (3, '*'),
            (4, '*'),
            (5, '*'),
            (6, '*'),
            (7, '*'),
        ],
    ),
    (
        523,
        &[
            (0, '#'),
            (1, '#'),
            (2, '#'),
            (3, '#'),
            (4, '#'),
            (5, '#'),
            (6, '#'),
            (7, '#'),
        ],
    ),
];

/// Invariant keysyms: keys whose keysym is the same across all layouts at given levels.
/// Format: (evdev, &[(level, keysym)]).
/// During RON serialization of keysym_map, entries matching this table are omitted.
static INVARIANT_KEYSYMS: &[(u32, &[(u8, u32)])] = &[
    (1, &[(0, 0xff1b /*Escape*/)]),
    (14, &[(0, 0xff08 /*BackSpace*/), (1, 0xff08 /*BackSpace*/)]),
    (15, &[(0, 0xff09 /*Tab*/), (1, 0xfe20 /*ISO_Left_Tab*/)]),
    (28, &[(0, 0xff0d /*Return*/)]),
    (29, &[(0, 0xffe3 /*Control_L*/)]),
    (42, &[(0, 0xffe1 /*Shift_L*/)]),
    (54, &[(0, 0xffe2 /*Shift_R*/)]),
    (
        59,
        &[
            (0, 0xffbe /*F1*/),
            (1, 0xffbe /*F1*/),
            (2, 0xffbe /*F1*/),
            (3, 0xffbe /*F1*/),
            (4, 0x1008fe01 /*XF86Switch_VT_1*/),
        ],
    ),
    (
        60,
        &[
            (0, 0xffbf /*F2*/),
            (1, 0xffbf /*F2*/),
            (2, 0xffbf /*F2*/),
            (3, 0xffbf /*F2*/),
            (4, 0x1008fe02 /*XF86Switch_VT_2*/),
        ],
    ),
    (
        61,
        &[
            (0, 0xffc0 /*F3*/),
            (1, 0xffc0 /*F3*/),
            (2, 0xffc0 /*F3*/),
            (3, 0xffc0 /*F3*/),
            (4, 0x1008fe03 /*XF86Switch_VT_3*/),
        ],
    ),
    (
        62,
        &[
            (0, 0xffc1 /*F4*/),
            (1, 0xffc1 /*F4*/),
            (2, 0xffc1 /*F4*/),
            (3, 0xffc1 /*F4*/),
            (4, 0x1008fe04 /*XF86Switch_VT_4*/),
        ],
    ),
    (
        63,
        &[
            (0, 0xffc2 /*F5*/),
            (1, 0xffc2 /*F5*/),
            (2, 0xffc2 /*F5*/),
            (3, 0xffc2 /*F5*/),
            (4, 0x1008fe05 /*XF86Switch_VT_5*/),
        ],
    ),
    (
        64,
        &[
            (0, 0xffc3 /*F6*/),
            (1, 0xffc3 /*F6*/),
            (2, 0xffc3 /*F6*/),
            (3, 0xffc3 /*F6*/),
            (4, 0x1008fe06 /*XF86Switch_VT_6*/),
        ],
    ),
    (
        65,
        &[
            (0, 0xffc4 /*F7*/),
            (1, 0xffc4 /*F7*/),
            (2, 0xffc4 /*F7*/),
            (3, 0xffc4 /*F7*/),
            (4, 0x1008fe07 /*XF86Switch_VT_7*/),
        ],
    ),
    (
        66,
        &[
            (0, 0xffc5 /*F8*/),
            (1, 0xffc5 /*F8*/),
            (2, 0xffc5 /*F8*/),
            (3, 0xffc5 /*F8*/),
            (4, 0x1008fe08 /*XF86Switch_VT_8*/),
        ],
    ),
    (
        67,
        &[
            (0, 0xffc6 /*F9*/),
            (1, 0xffc6 /*F9*/),
            (2, 0xffc6 /*F9*/),
            (3, 0xffc6 /*F9*/),
            (4, 0x1008fe09 /*XF86Switch_VT_9*/),
        ],
    ),
    (
        68,
        &[
            (0, 0xffc7 /*F10*/),
            (1, 0xffc7 /*F10*/),
            (2, 0xffc7 /*F10*/),
            (3, 0xffc7 /*F10*/),
            (4, 0x1008fe0a /*XF86Switch_VT_10*/),
        ],
    ),
    (84, &[(0, 0xfe03 /*ISO_Level3_Shift*/)]),
    (85, &[(0, 0x1008ffa9 /*XF86TouchpadToggle*/)]),
    (
        87,
        &[
            (0, 0xffc8 /*L1*/),
            (1, 0xffc8 /*L1*/),
            (2, 0xffc8 /*L1*/),
            (3, 0xffc8 /*L1*/),
            (4, 0x1008fe0b /*XF86Switch_VT_11*/),
        ],
    ),
    (
        88,
        &[
            (0, 0xffc9 /*L2*/),
            (1, 0xffc9 /*L2*/),
            (2, 0xffc9 /*L2*/),
            (3, 0xffc9 /*L2*/),
            (4, 0x1008fe0c /*XF86Switch_VT_12*/),
        ],
    ),
    (90, &[(0, 0xff26 /*Katakana*/)]),
    (91, &[(0, 0xff25 /*Hiragana*/)]),
    (92, &[(0, 0xff23 /*Henkan_Mode*/)]),
    (93, &[(0, 0xff27 /*Hiragana_Katakana*/)]),
    (94, &[(0, 0xff22 /*Muhenkan*/)]),
    (99, &[(0, 0xff61 /*SunPrint_Screen*/)]),
    (101, &[(0, 0xff0a /*Linefeed*/)]),
    (102, &[(0, 0xff50 /*Home*/)]),
    (103, &[(0, 0xff52 /*Up*/)]),
    (104, &[(0, 0xff55 /*SunPageUp*/)]),
    (105, &[(0, 0xff51 /*Left*/)]),
    (106, &[(0, 0xff53 /*Right*/)]),
    (107, &[(0, 0xff57 /*End*/)]),
    (108, &[(0, 0xff54 /*Down*/)]),
    (109, &[(0, 0xff56 /*SunPageDown*/)]),
    (110, &[(0, 0xff63 /*Insert*/)]),
    (111, &[(0, 0xffff /*Delete*/)]),
    (113, &[(0, 0x1008ff12 /*XF86AudioMute*/)]),
    (114, &[(0, 0x1008ff11 /*XF86AudioLowerVolume*/)]),
    (115, &[(0, 0x1008ff13 /*XF86AudioRaiseVolume*/)]),
    (116, &[(0, 0x1008ff2a /*XF86PowerOff*/)]),
    (118, &[(0, 0xb1 /*plusminus*/)]),
    (119, &[(0, 0xff13 /*Pause*/), (1, 0xff6b /*Break*/)]),
    (120, &[(0, 0x1008ff4a /*XF86LaunchA*/)]),
    (
        121,
        &[(0, 0xffae /*KP_Decimal*/), (1, 0xffae /*KP_Decimal*/)],
    ),
    (122, &[(0, 0xff31 /*Hangul*/)]),
    (123, &[(0, 0xff34 /*Hangul_Hanja*/)]),
    (125, &[(0, 0xffeb /*Super_L*/)]),
    (127, &[(0, 0xff67 /*Menu*/)]),
    (128, &[(0, 0xff69 /*SunStop*/)]),
    (129, &[(0, 0xff66 /*SunAgain*/)]),
    (130, &[(0, 0x1005ff70 /*SunProps*/)]),
    (131, &[(0, 0xff65 /*Undo*/)]),
    (132, &[(0, 0x1005ff71 /*SunFront*/)]),
    (133, &[(0, 0x1008ff57 /*XF86Copy*/)]),
    (134, &[(0, 0x1008ff6b /*XF86Open*/)]),
    (135, &[(0, 0x1008ff6d /*XF86Paste*/)]),
    (136, &[(0, 0xff68 /*SunFind*/)]),
    (137, &[(0, 0x1008ff58 /*XF86Cut*/)]),
    (138, &[(0, 0xff6a /*Help*/)]),
    (139, &[(0, 0x1008ff65 /*XF86MenuKB*/)]),
    (140, &[(0, 0x1008ff1d /*XF86Calculator*/)]),
    (142, &[(0, 0x1008ff2f /*XF86Sleep*/)]),
    (143, &[(0, 0x1008ff2b /*XF86WakeUp*/)]),
    (144, &[(0, 0x1008ff5d /*XF86Explorer*/)]),
    (145, &[(0, 0x1008ff7b /*XF86Send*/)]),
    (147, &[(0, 0x1008ff8a /*XF86Xfer*/)]),
    (148, &[(0, 0x1008ff41 /*XF86Launch1*/)]),
    (149, &[(0, 0x1008ff42 /*XF86Launch2*/)]),
    (150, &[(0, 0x1008ff2e /*XF86WWW*/)]),
    (151, &[(0, 0x1008ff5a /*XF86DOS*/)]),
    (152, &[(0, 0x1008ff2d /*XF86ScreenSaver*/)]),
    (153, &[(0, 0x1008ff74 /*XF86RotateWindows*/)]),
    (154, &[(0, 0x1008ff7f /*XF86TaskPane*/)]),
    (155, &[(0, 0x1008ff19 /*XF86Mail*/)]),
    (156, &[(0, 0x1008ff30 /*XF86Favorites*/)]),
    (157, &[(0, 0x1008ff33 /*XF86MyComputer*/)]),
    (158, &[(0, 0x1008ff26 /*XF86Back*/)]),
    (159, &[(0, 0x1008ff27 /*XF86Forward*/)]),
    (161, &[(0, 0x1008ff2c /*XF86Eject*/)]),
    (162, &[(0, 0x1008ff2c /*XF86Eject*/)]),
    (163, &[(0, 0x1008ff17 /*XF86AudioNext*/)]),
    (
        164,
        &[
            (0, 0x1008ff14 /*XF86AudioPlay*/),
            (1, 0x1008ff31 /*XF86AudioPause*/),
        ],
    ),
    (165, &[(0, 0x1008ff16 /*XF86AudioPrev*/)]),
    (
        166,
        &[
            (0, 0x1008ff15 /*XF86AudioStop*/),
            (1, 0x1008ff2c /*XF86Eject*/),
        ],
    ),
    (167, &[(0, 0x1008ff1c /*XF86AudioRecord*/)]),
    (168, &[(0, 0x1008ff3e /*XF86AudioRewind*/)]),
    (169, &[(0, 0x1008ff6e /*XF86Phone*/)]),
    (171, &[(0, 0x1008ff81 /*XF86Tools*/)]),
    (172, &[(0, 0x1008ff18 /*XF86HomePage*/)]),
    (173, &[(0, 0x1008ff73 /*XF86Reload*/)]),
    (174, &[(0, 0x1008ff56 /*XF86Close*/)]),
    (177, &[(0, 0x1008ff78 /*XF86ScrollUp*/)]),
    (178, &[(0, 0x1008ff79 /*XF86ScrollDown*/)]),
    (179, &[(0, 0x28 /*parenleft*/)]),
    (180, &[(0, 0x29 /*parenright*/)]),
    (181, &[(0, 0x1008ff68 /*XF86New*/)]),
    (182, &[(0, 0xff66 /*SunAgain*/)]),
    (183, &[(0, 0x1008ff81 /*XF86Tools*/)]),
    (184, &[(0, 0x1008ff45 /*XF86Launch5*/)]),
    (185, &[(0, 0x1008ff46 /*XF86Launch6*/)]),
    (186, &[(0, 0x1008ff47 /*XF86Launch7*/)]),
    (187, &[(0, 0x1008ff48 /*XF86Launch8*/)]),
    (188, &[(0, 0x1008ff49 /*XF86Launch9*/)]),
    (190, &[(0, 0x1008ffb2 /*XF86AudioMicMute*/)]),
    (191, &[(0, 0x1008ffa9 /*XF86TouchpadToggle*/)]),
    (192, &[(0, 0x1008ffb0 /*XF86TouchpadOn*/)]),
    (
        193,
        &[
            (0, 0x1008ffb1 /*XF86TouchpadOff*/),
            (1, 0x10081247 /*XF86Assistant*/),
        ],
    ),
    (195, &[(0, 0xfe11 /*ISO_Level5_Shift*/)]),
    (196, &[(1, 0xffe9 /*Alt_L*/)]),
    (197, &[(1, 0xffe7 /*Meta_L*/)]),
    (198, &[(1, 0xffeb /*Super_L*/)]),
    (200, &[(0, 0x1008ff14 /*XF86AudioPlay*/)]),
    (201, &[(0, 0x1008ff31 /*XF86AudioPause*/)]),
    (202, &[(0, 0x1008ff43 /*XF86Launch3*/)]),
    (203, &[(0, 0x1008ff44 /*XF86Launch4*/)]),
    (204, &[(0, 0x1008ff4b /*XF86LaunchB*/)]),
    (205, &[(0, 0x1008ffa7 /*XF86Suspend*/)]),
    (206, &[(0, 0x1008ff56 /*XF86Close*/)]),
    (207, &[(0, 0x1008ff14 /*XF86AudioPlay*/)]),
    (208, &[(0, 0x1008ff97 /*XF86AudioForward*/)]),
    (210, &[(0, 0xff61 /*SunPrint_Screen*/)]),
    (212, &[(0, 0x1008ff8f /*XF86WebCam*/)]),
    (213, &[(0, 0x1008ffb6 /*XF86AudioPreset*/)]),
    (215, &[(0, 0x1008ff19 /*XF86Mail*/)]),
    (216, &[(0, 0x1008ff8e /*XF86Messenger*/)]),
    (217, &[(0, 0x1008ff1b /*XF86Search*/)]),
    (218, &[(0, 0x1008ff5f /*XF86Go*/)]),
    (219, &[(0, 0x1008ff3c /*XF86Finance*/)]),
    (220, &[(0, 0x1008ff5e /*XF86Game*/)]),
    (221, &[(0, 0x1008ff36 /*XF86Shop*/)]),
    (223, &[(0, 0xff69 /*SunStop*/)]),
    (224, &[(0, 0x1008ff03 /*XF86MonBrightnessDown*/)]),
    (225, &[(0, 0x1008ff02 /*XF86MonBrightnessUp*/)]),
    (226, &[(0, 0x1008ff32 /*XF86AudioMedia*/)]),
    (227, &[(0, 0x1008ff59 /*XF86Display*/)]),
    (228, &[(0, 0x1008ff04 /*XF86KbdLightOnOff*/)]),
    (229, &[(0, 0x1008ff06 /*XF86KbdBrightnessDown*/)]),
    (230, &[(0, 0x1008ff05 /*XF86KbdBrightnessUp*/)]),
    (231, &[(0, 0x1008ff7b /*XF86Send*/)]),
    (232, &[(0, 0x1008ff72 /*XF86Reply*/)]),
    (233, &[(0, 0x1008ff90 /*XF86MailForward*/)]),
    (234, &[(0, 0x1008ff77 /*XF86Save*/)]),
    (235, &[(0, 0x1008ff5b /*XF86Documents*/)]),
    (236, &[(0, 0x1008ff93 /*XF86Battery*/)]),
    (237, &[(0, 0x1008ff94 /*XF86Bluetooth*/)]),
    (238, &[(0, 0x1008ff95 /*XF86WLAN*/)]),
    (239, &[(0, 0x1008ff96 /*XF86UWB*/)]),
    (241, &[(0, 0x1008fe22 /*XF86Next_VMode*/)]),
    (242, &[(0, 0x1008fe23 /*XF86Prev_VMode*/)]),
    (243, &[(0, 0x1008ff07 /*XF86MonBrightnessCycle*/)]),
    (244, &[(0, 0x100810f4 /*XF86MonBrightnessAuto*/)]),
    (245, &[(0, 0x100810f5 /*XF86DisplayOff*/)]),
    (246, &[(0, 0x1008ffb4 /*XF86WWAN*/)]),
    (247, &[(0, 0x1008ffb5 /*XF86RFKill*/)]),
    (248, &[(0, 0x1008ffb2 /*XF86AudioMicMute*/)]),
    (352, &[(0, 0x10081160 /*XF86OK*/)]),
    (353, &[(0, 0x1008ffa0 /*XF86Select*/)]),
    (354, &[(0, 0x10081162 /*XF86GoTo*/)]),
    (355, &[(0, 0x1008ff55 /*XF86Clear*/)]),
    (357, &[(0, 0x1008ff6c /*XF86Option*/)]),
    (358, &[(0, 0x10081166 /*XF86Info*/)]),
    (359, &[(0, 0x1008ff9f /*XF86Time*/)]),
    (360, &[(0, 0x10081168 /*XF86VendorLogo*/)]),
    (362, &[(0, 0x1008116a /*XF86MediaSelectProgramGuide*/)]),
    (363, &[(0, 0x10081270 /*XF86NextFavorite*/)]),
    (364, &[(0, 0x1008ff30 /*XF86Favorites*/)]),
    (365, &[(0, 0x1008116a /*XF86MediaSelectProgramGuide*/)]),
    (366, &[(0, 0x1008116e /*XF86MediaSelectHome*/)]),
    (368, &[(0, 0x10081170 /*XF86MediaLanguageMenu*/)]),
    (369, &[(0, 0x10081171 /*XF86MediaTitleMenu*/)]),
    (370, &[(0, 0x1008ff9a /*XF86Subtitle*/)]),
    (371, &[(0, 0x1008ff9c /*XF86CycleAngle*/)]),
    (372, &[(0, 0x1008ffb8 /*XF86FullScreen*/)]),
    (373, &[(0, 0x10081175 /*XF86AudioChannelMode*/)]),
    (374, &[(0, 0x1008ffb3 /*XF86Keyboard*/)]),
    (375, &[(0, 0x10081177 /*XF86AspectRatio*/)]),
    (376, &[(0, 0x10081178 /*XF86MediaSelectPC*/)]),
    (377, &[(0, 0x10081179 /*XF86MediaSelectTV*/)]),
    (378, &[(0, 0x1008117a /*XF86MediaSelectCable*/)]),
    (379, &[(0, 0x1008117b /*XF86MediaSelectVCR*/)]),
    (380, &[(0, 0x1008117c /*XF86MediaSelectVCRPlus*/)]),
    (381, &[(0, 0x1008117d /*XF86MediaSelectSatellite*/)]),
    (383, &[(0, 0x1008ff53 /*XF86MediaSelectCD*/)]),
    (384, &[(0, 0x10081180 /*XF86MediaSelectTape*/)]),
    (385, &[(0, 0x10081181 /*XF86MediaSelectRadio*/)]),
    (386, &[(0, 0x10081182 /*XF86MediaSelectTuner*/)]),
    (387, &[(0, 0x10081183 /*XF86MediaPlayer*/)]),
    (388, &[(0, 0x10081184 /*XF86MediaSelectTeletext*/)]),
    (389, &[(0, 0x10081185 /*XF86MediaSelectDVD*/)]),
    (390, &[(0, 0x10081186 /*XF86MediaSelectAuxiliary*/)]),
    (392, &[(0, 0x10081188 /*XF86Audio*/)]),
    (393, &[(0, 0x1008ff87 /*XF86Video*/)]),
    (396, &[(0, 0x1008ff1e /*XF86Memo*/)]),
    (397, &[(0, 0x1008ff20 /*XF86Calendar*/)]),
    (398, &[(0, 0x1008ffa3 /*XF86Red*/)]),
    (399, &[(0, 0x1008ffa4 /*XF86Green*/)]),
    (400, &[(0, 0x1008ffa5 /*XF86Yellow*/)]),
    (401, &[(0, 0x1008ffa6 /*XF86Blue*/)]),
    (402, &[(0, 0x10081192 /*XF86ChannelUp*/)]),
    (403, &[(0, 0x10081193 /*XF86ChannelDown*/)]),
    (409, &[(0, 0x10081199 /*XF86MediaPlaySlow*/)]),
    (410, &[(0, 0x1008ff99 /*XF86AudioRandomPlay*/)]),
    (411, &[(0, 0x1008119b /*XF86Break*/)]),
    (413, &[(0, 0x1008119d /*XF86NumberEntryMode*/)]),
    (416, &[(0, 0x100811a0 /*XF86VideoPhone*/)]),
    (417, &[(0, 0x1008ff5e /*XF86Game*/)]),
    (418, &[(0, 0x1008ff8b /*XF86ZoomIn*/)]),
    (419, &[(0, 0x1008ff8c /*XF86ZoomOut*/)]),
    (420, &[(0, 0x100811a4 /*XF86ZoomReset*/)]),
    (421, &[(0, 0x1008ff89 /*XF86Word*/)]),
    (422, &[(0, 0x100811a6 /*XF86Editor*/)]),
    (423, &[(0, 0x1008ff5c /*XF86Excel*/)]),
    (424, &[(0, 0x100811a8 /*XF86GraphicsEditor*/)]),
    (425, &[(0, 0x100811a9 /*XF86Presentation*/)]),
    (426, &[(0, 0x100811aa /*XF86Database*/)]),
    (427, &[(0, 0x1008ff69 /*XF86News*/)]),
    (428, &[(0, 0x100811ac /*XF86Voicemail*/)]),
    (429, &[(0, 0x100811ad /*XF86Addressbook*/)]),
    (430, &[(0, 0x1008ff8e /*XF86Messenger*/)]),
    (431, &[(0, 0x100811af /*XF86DisplayToggle*/)]),
    (432, &[(0, 0x100811b0 /*XF86SpellCheck*/)]),
    (433, &[(0, 0x1008ff61 /*XF86LogOff*/)]),
    (434, &[(0, 0x24 /*dollar*/)]),
    (435, &[(0, 0x20ac /*EuroSign*/)]),
    (436, &[(0, 0x1008ff9d /*XF86FrameBack*/)]),
    (437, &[(0, 0x1008ff9e /*XF86FrameForward*/)]),
    (438, &[(0, 0x100811b6 /*XF86ContextMenu*/)]),
    (439, &[(0, 0x100811b7 /*XF86MediaRepeat*/)]),
    (440, &[(0, 0x100811b8 /*XF8610ChannelsUp*/)]),
    (441, &[(0, 0x100811b9 /*XF8610ChannelsDown*/)]),
    (442, &[(0, 0x100811ba /*XF86Images*/)]),
    (444, &[(0, 0x100811bc /*XF86NotificationCenter*/)]),
    (445, &[(0, 0x100811bd /*XF86PickupPhone*/)]),
    (446, &[(0, 0x100811be /*XF86HangupPhone*/)]),
    (464, &[(0, 0x100811d0 /*XF86Fn*/)]),
    (465, &[(0, 0x100811d1 /*XF86Fn_Esc*/)]),
    (485, &[(0, 0x100811e5 /*XF86FnRightShift*/)]),
    (497, &[(0, 0xfff1 /*braille_dot_1*/)]),
    (498, &[(0, 0xfff2 /*braille_dot_2*/)]),
    (499, &[(0, 0xfff3 /*braille_dot_3*/)]),
    (500, &[(0, 0xfff4 /*braille_dot_4*/)]),
    (501, &[(0, 0xfff5 /*braille_dot_5*/)]),
    (502, &[(0, 0xfff6 /*braille_dot_6*/)]),
    (503, &[(0, 0xfff7 /*braille_dot_7*/)]),
    (504, &[(0, 0xfff8 /*braille_dot_8*/)]),
    (505, &[(0, 0xfff9 /*braille_dot_9*/)]),
    (506, &[(0, 0xfff1 /*braille_dot_1*/)]),
    (512, &[(0, 0x10081200 /*XF86Numeric0*/)]),
    (513, &[(0, 0x10081201 /*XF86Numeric1*/)]),
    (514, &[(0, 0x10081202 /*XF86Numeric2*/)]),
    (515, &[(0, 0x10081203 /*XF86Numeric3*/)]),
    (516, &[(0, 0x10081204 /*XF86Numeric4*/)]),
    (517, &[(0, 0x10081205 /*XF86Numeric5*/)]),
    (518, &[(0, 0x10081206 /*XF86Numeric6*/)]),
    (519, &[(0, 0x10081207 /*XF86Numeric7*/)]),
    (520, &[(0, 0x10081208 /*XF86Numeric8*/)]),
    (521, &[(0, 0x10081209 /*XF86Numeric9*/)]),
    (522, &[(0, 0x1008120a /*XF86NumericStar*/)]),
    (523, &[(0, 0x1008120b /*XF86NumericPound*/)]),
    (524, &[(0, 0x1008120c /*XF86NumericA*/)]),
    (525, &[(0, 0x1008120d /*XF86NumericB*/)]),
    (526, &[(0, 0x1008120e /*XF86NumericC*/)]),
    (527, &[(0, 0x1008120f /*XF86NumericD*/)]),
    (528, &[(0, 0x10081210 /*XF86CameraFocus*/)]),
    (529, &[(0, 0x10081211 /*XF86WPSButton*/)]),
    (530, &[(0, 0x1008ffa9 /*XF86TouchpadToggle*/)]),
    (531, &[(0, 0x1008ffb0 /*XF86TouchpadOn*/)]),
    (532, &[(0, 0x1008ffb1 /*XF86TouchpadOff*/)]),
    (533, &[(0, 0x10081215 /*XF86CameraZoomIn*/)]),
    (534, &[(0, 0x10081216 /*XF86CameraZoomOut*/)]),
    (535, &[(0, 0x10081217 /*XF86CameraUp*/)]),
    (536, &[(0, 0x10081218 /*XF86CameraDown*/)]),
    (537, &[(0, 0x10081219 /*XF86CameraLeft*/)]),
    (538, &[(0, 0x1008121a /*XF86CameraRight*/)]),
    (539, &[(0, 0x1008121b /*XF86AttendantOn*/)]),
    (540, &[(0, 0x1008121c /*XF86AttendantOff*/)]),
    (541, &[(0, 0x1008121d /*XF86AttendantToggle*/)]),
    (542, &[(0, 0x1008121e /*XF86LightsToggle*/)]),
    (560, &[(0, 0x10081230 /*XF86ALSToggle*/)]),
    (561, &[(0, 0x1008ffb7 /*XF86RotationLockToggle*/)]),
    (562, &[(0, 0x10081232 /*XF86RefreshRateToggle*/)]),
    (576, &[(0, 0x10081240 /*XF86Buttonconfig*/)]),
    (577, &[(0, 0x10081241 /*XF86Taskmanager*/)]),
    (578, &[(0, 0x10081242 /*XF86Journal*/)]),
    (579, &[(0, 0x10081243 /*XF86ControlPanel*/)]),
    (580, &[(0, 0x10081244 /*XF86AppSelect*/)]),
    (581, &[(0, 0x10081245 /*XF86Screensaver*/)]),
    (582, &[(0, 0x10081246 /*XF86VoiceCommand*/)]),
    (583, &[(0, 0x10081247 /*XF86Assistant*/)]),
    (584, &[(0, 0xfe08 /*ISO_Next_Group*/)]),
    (585, &[(0, 0x10081249 /*XF86EmojiPicker*/)]),
    (586, &[(0, 0x1008124a /*XF86Dictate*/)]),
    (587, &[(0, 0x1008124b /*XF86CameraAccessEnable*/)]),
    (588, &[(0, 0x1008124c /*XF86CameraAccessDisable*/)]),
    (589, &[(0, 0x1008124d /*XF86CameraAccessToggle*/)]),
    (590, &[(0, 0x1008124e /*XF86Accessibility*/)]),
    (591, &[(0, 0x1008124f /*XF86DoNotDisturb*/)]),
    (592, &[(0, 0x10081250 /*XF86BrightnessMin*/)]),
    (593, &[(0, 0x10081251 /*XF86BrightnessMax*/)]),
    (608, &[(0, 0x10081260 /*XF86KbdInputAssistPrev*/)]),
    (609, &[(0, 0x10081261 /*XF86KbdInputAssistNext*/)]),
    (610, &[(0, 0x10081262 /*XF86KbdInputAssistPrevgroup*/)]),
    (611, &[(0, 0x10081263 /*XF86KbdInputAssistNextgroup*/)]),
    (612, &[(0, 0x10081264 /*XF86KbdInputAssistAccept*/)]),
    (613, &[(0, 0x10081265 /*XF86KbdInputAssistCancel*/)]),
    (614, &[(0, 0x10081266 /*XF86RightUp*/)]),
    (615, &[(0, 0x10081267 /*XF86RightDown*/)]),
    (616, &[(0, 0x10081268 /*XF86LeftUp*/)]),
    (617, &[(0, 0x10081269 /*XF86LeftDown*/)]),
    (618, &[(0, 0x1008126a /*XF86RootMenu*/)]),
    (619, &[(0, 0x1008126b /*XF86MediaTopMenu*/)]),
    (620, &[(0, 0x1008126c /*XF86Numeric11*/)]),
    (621, &[(0, 0x1008126d /*XF86Numeric12*/)]),
    (622, &[(0, 0x1008126e /*XF86AudioDesc*/)]),
    (623, &[(0, 0x1008126f /*XF863DMode*/)]),
    (624, &[(0, 0x10081270 /*XF86NextFavorite*/)]),
    (625, &[(0, 0x10081271 /*XF86StopRecord*/)]),
    (626, &[(0, 0x10081272 /*XF86PauseRecord*/)]),
    (627, &[(0, 0x10081273 /*XF86VOD*/)]),
    (628, &[(0, 0x10081274 /*XF86Unmute*/)]),
    (629, &[(0, 0x10081275 /*XF86FastReverse*/)]),
    (630, &[(0, 0x10081276 /*XF86SlowReverse*/)]),
    (631, &[(0, 0x10081277 /*XF86Data*/)]),
    (632, &[(0, 0x10081278 /*XF86OnScreenKeyboard*/)]),
    (633, &[(0, 0x10081279 /*XF86PrivacyScreenToggle*/)]),
    (634, &[(0, 0x1008127a /*XF86SelectiveScreenshot*/)]),
    (635, &[(0, 0x1008127b /*XF86NextElement*/)]),
    (636, &[(0, 0x1008127c /*XF86PreviousElement*/)]),
    (637, &[(0, 0x1008127d /*XF86AutopilotEngageToggle*/)]),
    (638, &[(0, 0x1008127e /*XF86MarkWaypoint*/)]),
    (639, &[(0, 0x1008127f /*XF86Sos*/)]),
    (640, &[(0, 0x10081280 /*XF86NavChart*/)]),
    (641, &[(0, 0x10081281 /*XF86FishingChart*/)]),
    (642, &[(0, 0x10081282 /*XF86SingleRangeRadar*/)]),
    (643, &[(0, 0x10081283 /*XF86DualRangeRadar*/)]),
    (644, &[(0, 0x10081284 /*XF86RadarOverlay*/)]),
    (645, &[(0, 0x10081285 /*XF86TraditionalSonar*/)]),
    (646, &[(0, 0x10081286 /*XF86ClearvuSonar*/)]),
    (647, &[(0, 0x10081287 /*XF86SidevuSonar*/)]),
    (648, &[(0, 0x10081288 /*XF86NavInfo*/)]),
    (649, &[(0, 0x1008ff3b /*XF86BrightnessAdjust*/)]),
    (656, &[(0, 0x10081290 /*XF86Macro1*/)]),
    (657, &[(0, 0x10081291 /*XF86Macro2*/)]),
    (658, &[(0, 0x10081292 /*XF86Macro3*/)]),
    (659, &[(0, 0x10081293 /*XF86Macro4*/)]),
    (660, &[(0, 0x10081294 /*XF86Macro5*/)]),
    (661, &[(0, 0x10081295 /*XF86Macro6*/)]),
    (662, &[(0, 0x10081296 /*XF86Macro7*/)]),
    (663, &[(0, 0x10081297 /*XF86Macro8*/)]),
    (664, &[(0, 0x10081298 /*XF86Macro9*/)]),
    (665, &[(0, 0x10081299 /*XF86Macro10*/)]),
    (666, &[(0, 0x1008129a /*XF86Macro11*/)]),
    (667, &[(0, 0x1008129b /*XF86Macro12*/)]),
    (668, &[(0, 0x1008129c /*XF86Macro13*/)]),
    (669, &[(0, 0x1008129d /*XF86Macro14*/)]),
    (670, &[(0, 0x1008129e /*XF86Macro15*/)]),
    (671, &[(0, 0x1008129f /*XF86Macro16*/)]),
    (672, &[(0, 0x100812a0 /*XF86Macro17*/)]),
    (673, &[(0, 0x100812a1 /*XF86Macro18*/)]),
    (674, &[(0, 0x100812a2 /*XF86Macro19*/)]),
    (675, &[(0, 0x100812a3 /*XF86Macro20*/)]),
    (676, &[(0, 0x100812a4 /*XF86Macro21*/)]),
    (677, &[(0, 0x100812a5 /*XF86Macro22*/)]),
    (678, &[(0, 0x100812a6 /*XF86Macro23*/)]),
    (679, &[(0, 0x100812a7 /*XF86Macro24*/)]),
    (680, &[(0, 0x100812a8 /*XF86Macro25*/)]),
    (681, &[(0, 0x100812a9 /*XF86Macro26*/)]),
    (682, &[(0, 0x100812aa /*XF86Macro27*/)]),
    (683, &[(0, 0x100812ab /*XF86Macro28*/)]),
    (684, &[(0, 0x100812ac /*XF86Macro29*/)]),
    (685, &[(0, 0x100812ad /*XF86Macro30*/)]),
    (688, &[(0, 0x100812b0 /*XF86MacroRecordStart*/)]),
    (689, &[(0, 0x100812b1 /*XF86MacroRecordStop*/)]),
    (690, &[(0, 0x100812b2 /*XF86MacroPresetCycle*/)]),
    (691, &[(0, 0x100812b3 /*XF86MacroPreset1*/)]),
    (692, &[(0, 0x100812b4 /*XF86MacroPreset2*/)]),
    (693, &[(0, 0x100812b5 /*XF86MacroPreset3*/)]),
    (696, &[(0, 0x100812b8 /*XF86KbdLcdMenu1*/)]),
    (697, &[(0, 0x100812b9 /*XF86KbdLcdMenu2*/)]),
    (698, &[(0, 0x100812ba /*XF86KbdLcdMenu3*/)]),
    (699, &[(0, 0x100812bb /*XF86KbdLcdMenu4*/)]),
    (700, &[(0, 0x100812bc /*XF86KbdLcdMenu5*/)]),
];

/// Check if (evdev, level, char) matches an invariant entry.
/// Returns true if this entry should be omitted from RON.
/// A key is invariant if:
/// - It matches an entry in INVARIANT_CHARS at (evdev, level, char), OR
/// - It's in INVARIANT_CHARS at level 0 and the char matches level 0's char
///   (key produces same char at all levels → invariant at all levels)
fn is_invariant_char(evdev: u32, level: u8, ch: char) -> bool {
    for &(e, levels) in INVARIANT_CHARS {
        if e == evdev {
            // Direct match
            if levels.iter().any(|&(l, c)| l == level && c == ch) {
                return true;
            }
            // If level-0 char matches and this key is in the table, it's invariant at all levels
            if let Some(&(_, l0_ch)) = levels.iter().find(|&&(l, _)| l == 0) {
                if ch == l0_ch {
                    return true;
                }
            }
            return false;
        }
    }
    false
}

/// Check if (evdev, level, keysym) matches an invariant keysym entry.
/// Only exact (evdev, level, sym) matches are filtered — no level-0 extrapolation
/// because XKB doesn't necessarily define keysyms at all levels.
fn is_invariant_keysym(evdev: u32, level: u8, sym: u32) -> bool {
    for &(e, levels) in INVARIANT_KEYSYMS {
        if e == evdev {
            return levels.iter().any(|&(l, s)| l == level && s == sym);
        }
    }
    false
}

/// Default repeat keys — based on the standard XKB repeat set shared by >99% of layouts.
/// In .ron files only additions/removals from this set are stored.
#[rustfmt::skip]
static DEFAULT_REPEAT_KEYS: &[u32] = &[
    1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,
    30,31,32,33,34,35,36,37,38,39,40,41,43,44,45,46,47,48,49,50,51,52,53,55,57,
    59,60,61,62,63,64,65,66,67,68,70,71,72,73,74,75,76,77,78,79,80,81,82,83,
    85,86,87,88,90,91,92,93,94,96,98,99,101,102,103,104,105,106,107,108,109,110,111,
    113,114,115,116,117,118,119,120,121,122,123,127,128,129,130,131,132,133,134,135,
    136,137,138,139,140,142,143,144,145,147,148,149,150,151,152,153,154,155,156,157,
    158,159,161,162,163,164,165,166,167,168,169,171,172,173,174,177,178,179,180,181,
    182,183,184,185,186,187,188,190,191,192,193,200,201,202,203,204,205,206,207,208,
    210,212,213,215,216,217,218,219,220,221,223,224,225,226,227,228,229,230,231,232,
    233,234,235,236,237,238,239,241,242,243,244,245,246,247,248,352,353,354,355,357,
    358,359,360,362,363,364,365,366,368,369,370,371,372,373,374,375,376,377,378,379,
    380,381,383,384,385,386,387,388,389,390,392,393,396,397,398,399,400,401,402,403,
    409,410,411,413,416,417,418,419,420,421,422,423,424,425,426,427,428,429,430,431,
    432,433,434,435,436,437,438,439,440,441,442,444,445,446,464,465,485,497,498,499,
    500,501,502,503,504,505,506,512,513,514,515,516,517,518,519,520,521,522,523,524,
    525,526,527,528,529,530,531,532,533,534,535,536,537,538,539,540,541,542,560,561,
    562,576,577,578,579,580,581,582,583,585,586,587,588,589,590,591,592,593,608,609,
    610,611,612,613,614,615,616,617,618,619,620,621,622,623,624,625,626,627,628,629,
    630,631,632,633,634,635,636,637,638,639,640,641,642,643,644,645,646,647,648,649,
    656,657,658,659,660,661,662,663,664,665,666,667,668,669,670,671,672,673,674,675,
    676,677,678,679,680,681,682,683,684,685,688,689,690,691,692,693,696,697,698,699,700,
];

/// Modifier kind without runtime state (pressed/locked/latched are always default in saved files).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ReadableModKind {
    Pressed(ModType),
    Lock(ModType),
    Latch(ModType),
}

/// Well-known evdev code → human-readable modifier key name.
/// Falls back to the keysym name (friendly-translated) if provided.
fn modifier_key_name(evdev: u32, keysym_name: Option<&str>) -> String {
    match evdev {
        29 => "LeftCtrl".into(),
        42 => "LeftShift".into(),
        54 => "RightShift".into(),
        56 => keysym_name.unwrap_or("LeftAlt").into(),
        58 => keysym_name.unwrap_or("CapsLock").into(),
        69 => "NumLock".into(),
        70 => "ScrollLock".into(),
        97 => "RightCtrl".into(),
        100 => keysym_name.unwrap_or("AltGr").into(),
        125 => keysym_name.unwrap_or("LeftSuper").into(),
        other => match keysym_name {
            Some(name) => name.to_string(),
            None => format!("Key{}", other),
        },
    }
}

/// Human-readable intermediate format for RON serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ReadableWKB {
    layout_names: Vec<String>,
    num_keys: usize,
    repeat_keys_add: Vec<u32>,
    repeat_keys_remove: Vec<u32>,
    modifiers: Vec<(u32, String, Vec<(u8, ReadableModKind)>)>,
    keymap: BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    num_lock_keys: BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    caps_lock_keymap: BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    #[cfg(feature = "xkb")]
    level_exceptions_keymap: BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    keysym_map: BTreeMap<String, BTreeMap<u8, BTreeMap<u32, String>>>,
    compose: Vec<(Vec<String>, char)>,
}

// ── KeyBitSet ↔ Vec<u32> ──────────────────────────────────────────────

fn bitset_to_vec(bs: &KeyBitSet) -> Vec<u32> {
    let mut out = Vec::new();
    for code in 0u32..(BITSET_WORDS as u32 * 64) {
        if bs.contains(code) {
            out.push(code);
        }
    }
    out
}

fn flat_keymap_to_map(
    km: &FlatKeymap,
    layout_names: &[String],
    omit_invariants: bool,
    reachable: Option<&[usize]>,
) -> BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>> {
    let mut result = BTreeMap::new();
    for (li, name) in layout_names.iter().enumerate() {
        let mut levels = BTreeMap::new();
        let level_iter: Vec<usize> = match reachable {
            Some(r) => r.to_vec(),
            None => (0..MAX_LEVELS).collect(),
        };
        for level in level_iter {
            let mut keys = BTreeMap::new();
            for evdev in 0..km.num_keys as u32 {
                if let Some(ch) = km.get(li, level, evdev) {
                    if omit_invariants {
                        // Skip entries matching the invariant char table
                        if is_invariant_char(evdev, level as u8, ch) {
                            continue;
                        }
                    }
                    keys.insert(evdev, ch);
                }
            }
            if !keys.is_empty() {
                levels.insert(level as u8, keys);
            }
        }
        if !levels.is_empty() {
            result.insert(name.clone(), levels);
        }
    }
    result
}

fn map_to_flat_keymap(
    map: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    layout_names: &[String],
    num_keys: usize,
) -> FlatKeymap {
    let num_layouts = layout_names.len();
    let mut km = FlatKeymap::new(num_keys, num_layouts);

    for (name, levels) in map {
        if let Some(li) = layout_names.iter().position(|n| n == name) {
            for (&level, keys) in levels {
                for (&evdev, &ch) in keys {
                    km.set(li, level as usize, evdev, ch);
                }
            }
        }
    }
    km
}

fn map_to_flat_keymap_with_defaults(
    map: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    layout_names: &[String],
    num_keys: usize,
) -> FlatKeymap {
    let mut km = map_to_flat_keymap(map, layout_names, num_keys);
    // Restore invariant chars for keys omitted from RON.
    // Only populate at levels that exist in the RON for this layout
    // (i.e., levels that are reachable for the keyboard).
    for (li, name) in layout_names.iter().enumerate() {
        let active_levels: Vec<u8> = map
            .get(name)
            .map(|levels| levels.keys().copied().collect())
            .unwrap_or_default();
        // Always include level 0
        let mut lvls: Vec<u8> = active_levels;
        if !lvls.contains(&0) {
            lvls.push(0);
        }

        for &(evdev, levels) in INVARIANT_CHARS {
            if (evdev as usize) >= num_keys {
                continue;
            }
            let l0_ch = levels.iter().find(|&&(l, _)| l == 0).map(|&(_, c)| c);
            // Populate at all active levels with level-0 char
            if let Some(ch) = l0_ch {
                for &lvl in &lvls {
                    if km.get(li, lvl as usize, evdev).is_none() {
                        km.set(li, lvl as usize, evdev, ch);
                    }
                }
            }
            // Apply specific level overrides
            for &(level, ch) in levels {
                if lvls.contains(&level) && km.get(li, level as usize, evdev).is_none() {
                    km.set(li, level as usize, evdev, ch);
                }
            }
        }
    }
    km
}

fn keysym_name(sym: u32) -> String {
    if sym == 0 {
        return "NoSymbol".to_string();
    }
    if (0x0100_0000..=0x0110_ffff).contains(&sym) {
        return format!("{:#010x}", sym);
    }
    if let Some(name) = crate::keysyms::keysym_get_name(sym) {
        name.to_string()
    } else {
        format!("{:#010x}", sym)
    }
}

fn keysym_from_name_or_hex(name: &str) -> u32 {
    if let Some(sym) = crate::keysyms::keysym_from_name(name) {
        return sym;
    }
    // Try hex literal (0x01000042 etc.)
    if let Some(hex) = name.strip_prefix("0x") {
        if let Ok(v) = u32::from_str_radix(hex, 16) {
            return v;
        }
    }
    0
}

fn flat_keysym_map_to_map(
    km: &crate::FlatKeysymMap,
    layout_names: &[String],
) -> BTreeMap<String, BTreeMap<u8, BTreeMap<u32, String>>> {
    let mut result = BTreeMap::new();
    for (li, name) in layout_names.iter().enumerate() {
        let mut levels = BTreeMap::new();
        for level in 0..MAX_LEVELS {
            let mut keys = BTreeMap::new();
            for evdev in 0..km.num_keys as u32 {
                let sym = km.get(li, level, evdev);
                if sym != 0 {
                    // Skip invariant keysyms
                    if is_invariant_keysym(evdev, level as u8, sym) {
                        continue;
                    }
                    keys.insert(evdev, keysym_name(sym));
                }
            }
            if !keys.is_empty() {
                levels.insert(level as u8, keys);
            }
        }
        if !levels.is_empty() {
            result.insert(name.clone(), levels);
        }
    }
    result
}

fn map_to_flat_keysym_map(
    map: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, String>>>,
    layout_names: &[String],
    num_keys: usize,
) -> crate::FlatKeysymMap {
    let num_layouts = layout_names.len();
    let mut km = crate::FlatKeysymMap::new(num_keys, num_layouts);
    // First apply RON data
    for (name, levels) in map {
        if let Some(li) = layout_names.iter().position(|n| n == name) {
            for (&level, keys) in levels {
                for (&evdev, sym_name) in keys {
                    let sym = keysym_from_name_or_hex(sym_name);
                    if sym != 0 {
                        km.set(li, level as usize, evdev, sym);
                    }
                }
            }
        }
    }
    // Restore invariant keysyms only at their specific listed levels.
    // These are keysyms present in ALL layouts, so restore unconditionally.
    for li in 0..num_layouts {
        for &(evdev, levels) in INVARIANT_KEYSYMS {
            if (evdev as usize) >= num_keys {
                continue;
            }
            for &(level, sym) in levels {
                if km.get(li, level as usize, evdev) == 0 {
                    km.set(li, level as usize, evdev, sym);
                }
            }
        }
    }
    km
}

// ── Friendly keysym name aliases ──────────────────────────────────────
// Replace XKB-internal names with user-friendly equivalents.

/// XKB keysym name → friendly name for RON output.
fn friendly_keysym_name(xkb_name: &str) -> &str {
    match xkb_name {
        // Modifier keys
        "Shift_L" => "LeftShift",
        "Shift_R" => "RightShift",
        "Control_L" => "LeftCtrl",
        "Control_R" => "RightCtrl",
        "Alt_L" => "LeftAlt",
        "Alt_R" => "RightAlt",
        "Super_L" => "LeftSuper",
        "Super_R" => "RightSuper",
        "Meta_L" => "LeftMeta",
        "Meta_R" => "RightMeta",
        "Hyper_L" => "LeftHyper",
        "Hyper_R" => "RightHyper",
        "Caps_Lock" => "CapsLock",
        "Num_Lock" => "NumLock",
        "Scroll_Lock" => "ScrollLock",
        // ISO level modifiers
        "ISO_Level2_Latch" => "ShiftLatch",
        "ISO_Level3_Shift" => "AltGr",
        "ISO_Level3_Latch" => "AltGrLatch",
        "ISO_Level3_Lock" => "AltGrLock",
        "ISO_Level5_Shift" => "Level5",
        "ISO_Level5_Latch" => "Level5Latch",
        "ISO_Level5_Lock" => "Level5Lock",
        // ISO group switching
        "ISO_Next_Group" => "NextLayout",
        "ISO_Prev_Group" => "PrevLayout",
        "ISO_Group_Shift" => "LayoutShift",
        "ISO_Group_Latch" => "LayoutLatch",
        "ISO_Group_Lock" => "LayoutLock",
        "ISO_First_Group" => "FirstLayout",
        "ISO_First_Group_Lock" => "FirstLayoutLock",
        "ISO_Last_Group" => "LastLayout",
        "ISO_Last_Group_Lock" => "LastLayoutLock",
        "ISO_Next_Group_Lock" => "NextLayoutLock",
        "ISO_Prev_Group_Lock" => "PrevLayoutLock",
        // Other ISO
        "ISO_Left_Tab" => "LeftTab",
        "ISO_Enter" => "Enter",
        "ISO_Lock" => "Lock",
        // Navigation
        "KP_Enter" => "NumpadEnter",
        "KP_Add" => "NumpadAdd",
        "KP_Subtract" => "NumpadSubtract",
        "KP_Multiply" => "NumpadMultiply",
        "KP_Divide" => "NumpadDivide",
        "KP_Decimal" => "NumpadDecimal",
        "KP_Separator" => "NumpadSeparator",
        "KP_Equal" => "NumpadEqual",
        _ => xkb_name,
    }
}

// ── Composer ↔ sequence list ──────────────────────────────────────────

fn composer_to_sequences(composer: &Composer) -> Vec<(Vec<String>, char)> {
    let mut sequences = Vec::new();
    let mut path = Vec::new();
    walk_trie(&composer.nodes, 0, &mut path, &mut sequences);
    sequences
}

fn walk_trie(
    nodes: &[TrieNode],
    idx: usize,
    path: &mut Vec<String>,
    out: &mut Vec<(Vec<String>, char)>,
) {
    let node = &nodes[idx];
    if let Some(ch) = node.emit {
        out.push((path.clone(), ch));
    }
    for &(key, child_idx) in &node.children {
        let token_str = if key == 0 {
            "·".to_string()
        } else if let Some(c) = char::from_u32(key) {
            c.to_string()
        } else {
            format!("U+{:04X}", key)
        };
        path.push(token_str);
        walk_trie(nodes, child_idx as usize, path, out);
        path.pop();
    }
}

fn sequences_to_composer(sequences: &[(Vec<String>, char)]) -> Composer {
    let mut composer = Composer::new();
    for (seq, output) in sequences {
        let tokens: Vec<Token> = seq
            .iter()
            .map(|s| {
                if s == "·" {
                    Token::Compose
                } else {
                    let mut chars = s.chars();
                    let c = chars.next().unwrap_or('\0');
                    Token::Char(c)
                }
            })
            .collect();
        composer.insert(&tokens, *output);
    }
    composer
}

// ── Modifiers ↔ readable representation ──────────────────────────────

fn mod_kind_to_readable(mk: &ModKind) -> ReadableModKind {
    match mk {
        ModKind::Pressed { mod_type, .. } => ReadableModKind::Pressed(*mod_type),
        ModKind::Lock { mod_type, .. } => ReadableModKind::Lock(*mod_type),
        ModKind::Latch { mod_type, .. } => ReadableModKind::Latch(*mod_type),
        ModKind::None => ReadableModKind::Pressed(ModType::None),
    }
}

fn readable_to_mod_kind(rmk: &ReadableModKind) -> ModKind {
    match rmk {
        ReadableModKind::Pressed(mt) => ModKind::Pressed {
            pressed: false,
            mod_type: *mt,
        },
        ReadableModKind::Lock(mt) => ModKind::Lock {
            pressed: false,
            locked: 0,
            mod_type: *mt,
        },
        ReadableModKind::Latch(mt) => ModKind::Latch {
            pressed: false,
            latched: false,
            mod_type: *mt,
        },
    }
}

fn modifiers_to_readable(
    mods: &Modifiers,
    wkb: &crate::WKB,
) -> Vec<(u32, String, Vec<(u8, ReadableModKind)>)> {
    let mut out = Vec::new();
    for (evdev, modifier) in mods.iter() {
        // Skip modifiers that have no level effect (Pressed(None), ModKind::None)
        let is_all_none = match modifier {
            Modifier::Single(mk) => matches!(
                mk,
                ModKind::None
                    | ModKind::Pressed {
                        mod_type: ModType::None,
                        ..
                    }
            ),
            Modifier::Leveled(map) => map.values().all(|mk| {
                matches!(
                    mk,
                    ModKind::None
                        | ModKind::Pressed {
                            mod_type: ModType::None,
                            ..
                        }
                )
            }),
        };
        if is_all_none {
            continue;
        }
        // For unknown keys, derive keysym at level 0 layout 0 and use friendly name
        let keysym_fallback = {
            let sym = wkb.level_keysym(*evdev, 0, 0);
            if sym != 0 {
                keysyms::keysym_get_name(sym).map(|n| friendly_keysym_name(n).to_string())
            } else {
                None
            }
        };
        let name = modifier_key_name(*evdev, keysym_fallback.as_deref());
        let entries = match modifier {
            Modifier::Single(mk) => vec![(0u8, mod_kind_to_readable(mk))],
            Modifier::Leveled(map) => map
                .iter()
                .map(|(&level, mk)| (level, mod_kind_to_readable(mk)))
                .collect(),
        };
        out.push((*evdev, name, entries));
    }
    out
}

fn readable_to_modifiers(entries: &[(u32, String, Vec<(u8, ReadableModKind)>)]) -> Modifiers {
    let mut mods = Modifiers::new();
    for (evdev, _name, kinds) in entries {
        let modifier = if kinds.len() == 1 && kinds[0].0 == 0 {
            Modifier::Single(readable_to_mod_kind(&kinds[0].1))
        } else {
            let map: BTreeMap<u8, ModKind> = kinds
                .iter()
                .map(|(level, rmk)| (*level, readable_to_mod_kind(rmk)))
                .collect();
            Modifier::Leveled(map)
        };
        mods.set_modifier(*evdev, modifier);
    }
    mods
}

// ── WKB ↔ ReadableWKB ────────────────────────────────────────────────

impl ReadableWKB {
    pub(crate) fn from_wkb(wkb: &crate::WKB) -> Self {
        let actual = bitset_to_vec(&wkb.repeat_keys);
        let default_set: std::collections::HashSet<u32> =
            DEFAULT_REPEAT_KEYS.iter().copied().collect();
        let actual_set: std::collections::HashSet<u32> = actual.iter().copied().collect();
        let mut repeat_keys_add: Vec<u32> = actual_set.difference(&default_set).copied().collect();
        let mut repeat_keys_remove: Vec<u32> =
            default_set.difference(&actual_set).copied().collect();
        repeat_keys_add.sort();
        repeat_keys_remove.sort();

        // Compute reachable levels from modifier keys.
        // Level index is a 3-bit field: bit0=Shift, bit1=Level3(AltGr), bit2=Level5.
        let has_mod = |target: crate::modifiers::ModType| -> bool {
            wkb.modifiers.entries.iter().any(|(_, modifier)| {
                let mod_kind_has = |mk: &crate::modifiers::ModKind| -> bool {
                    match mk {
                        crate::modifiers::ModKind::Pressed { mod_type, .. }
                        | crate::modifiers::ModKind::Lock { mod_type, .. }
                        | crate::modifiers::ModKind::Latch { mod_type, .. } => *mod_type == target,
                        crate::modifiers::ModKind::None => false,
                    }
                };
                match modifier {
                    crate::modifiers::Modifier::Single(mk) => mod_kind_has(mk),
                    crate::modifiers::Modifier::Leveled(map) => map.values().any(mod_kind_has),
                }
            })
        };
        let has_level3 = has_mod(crate::modifiers::ModType::Level3);
        let has_level5 = has_mod(crate::modifiers::ModType::Level5);
        let reachable: Vec<usize> = (0..MAX_LEVELS)
            .filter(|&lvl| (lvl & 2 == 0 || has_level3) && (lvl & 4 == 0 || has_level5))
            .collect();

        ReadableWKB {
            layout_names: wkb.layout_names.clone(),
            num_keys: wkb.state_keymap.num_keys,
            repeat_keys_add,
            repeat_keys_remove,
            modifiers: modifiers_to_readable(&wkb.modifiers, wkb),
            keymap: flat_keymap_to_map(
                &wkb.state_keymap,
                &wkb.layout_names,
                true,
                Some(&reachable),
            ),
            num_lock_keys: flat_keymap_to_map(
                &wkb.num_lock_keys,
                &wkb.layout_names,
                false,
                Some(&reachable),
            ),
            caps_lock_keymap: flat_keymap_to_map(
                &wkb.caps_lock_keymap,
                &wkb.layout_names,
                false,
                Some(&reachable),
            ),
            #[cfg(feature = "xkb")]
            level_exceptions_keymap: flat_keymap_to_map(
                &wkb.level_exceptions_keymap,
                &wkb.layout_names,
                false,
                Some(&reachable),
            ),
            keysym_map: flat_keysym_map_to_map(&wkb.keysym_map, &wkb.layout_names),
            compose: composer_to_sequences(&wkb.composer),
        }
    }

    pub(crate) fn to_wkb(self) -> crate::WKB {
        let layout_names = self.layout_names;
        let num_keys = self.num_keys;
        let state_keymap = map_to_flat_keymap_with_defaults(&self.keymap, &layout_names, num_keys);
        let num_lock_keys = map_to_flat_keymap(&self.num_lock_keys, &layout_names, num_keys);
        let caps_lock_keymap = map_to_flat_keymap(&self.caps_lock_keymap, &layout_names, num_keys);
        #[cfg(feature = "xkb")]
        let level_exceptions_keymap =
            map_to_flat_keymap(&self.level_exceptions_keymap, &layout_names, num_keys);
        let modifiers = readable_to_modifiers(&self.modifiers);

        // Reconstruct repeat keys from defaults + add/remove diffs
        let mut repeat_set: std::collections::HashSet<u32> =
            DEFAULT_REPEAT_KEYS.iter().copied().collect();
        for &code in &self.repeat_keys_add {
            repeat_set.insert(code);
        }
        for &code in &self.repeat_keys_remove {
            repeat_set.remove(&code);
        }
        let mut repeat_keys = KeyBitSet::new();
        for code in repeat_set {
            repeat_keys.insert(code);
        }
        let keysym_map = map_to_flat_keysym_map(&self.keysym_map, &layout_names, num_keys);
        let wkb = crate::WKB {
            repeat_keys,
            composer: sequences_to_composer(&self.compose),
            modifiers,
            state_keymap,
            num_lock_keys,
            caps_lock_keymap,
            current_layout_idx: 0,
            layout_names,
            keysym_map,
            #[cfg(feature = "xkb")]
            level_exceptions_keymap,
        };
        wkb
    }
}

// ── Custom compact RON formatter ─────────────────────────────────────

/// Keyboard visual groups for formatting. Each group is a named set of evdev codes.
struct KeyGroup {
    codes: &'static [u32],
}

// Main keyboard rows
const ROW_NUMBER: KeyGroup = KeyGroup {
    codes: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
};
const ROW_QWERTY: KeyGroup = KeyGroup {
    codes: &[15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28],
};
const ROW_HOME: KeyGroup = KeyGroup {
    codes: &[29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43],
};
const ROW_BOTTOM: KeyGroup = KeyGroup {
    codes: &[42, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54],
};
const ROW_SPACE: KeyGroup = KeyGroup {
    codes: &[56, 57, 86, 100, 125],
};

// Function keys
const ROW_FKEYS: KeyGroup = KeyGroup {
    codes: &[59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 87, 88],
};

// Navigation cluster
const ROW_NAV_TOP: KeyGroup = KeyGroup {
    codes: &[110, 102, 104], // Insert, Home, PageUp
};
const ROW_NAV_BOT: KeyGroup = KeyGroup {
    codes: &[111, 107, 109], // Delete, End, PageDown
};
const ROW_ARROWS: KeyGroup = KeyGroup {
    codes: &[103, 105, 108, 106], // Up, Left, Down, Right
};

// Numpad
const ROW_NUMPAD_TOP: KeyGroup = KeyGroup {
    codes: &[69, 98, 55, 74], // NumLock, KP/, KP*, KP-
};
const ROW_NUMPAD_789: KeyGroup = KeyGroup {
    codes: &[71, 72, 73], // KP7, KP8, KP9
};
const ROW_NUMPAD_456: KeyGroup = KeyGroup {
    codes: &[75, 76, 77, 78], // KP4, KP5, KP6, KP+
};
const ROW_NUMPAD_123: KeyGroup = KeyGroup {
    codes: &[79, 80, 81], // KP1, KP2, KP3
};
const ROW_NUMPAD_0: KeyGroup = KeyGroup {
    codes: &[82, 83, 96], // KP0, KP., KPEnter
};

// Misc keys
const ROW_MISC: KeyGroup = KeyGroup {
    codes: &[70, 99, 101, 117, 118, 119, 121], // ScrollLock, SysRq, LineFeed, KP=, KP±, Pause, KPComma
};

const ALL_GROUPS: &[&KeyGroup] = &[
    &ROW_NUMBER,
    &ROW_QWERTY,
    &ROW_HOME,
    &ROW_BOTTOM,
    &ROW_SPACE,
    &ROW_FKEYS,
    &ROW_NAV_TOP,
    &ROW_NAV_BOT,
    &ROW_ARROWS,
    &ROW_NUMPAD_TOP,
    &ROW_NUMPAD_789,
    &ROW_NUMPAD_456,
    &ROW_NUMPAD_123,
    &ROW_NUMPAD_0,
    &ROW_MISC,
];

/// Check which group an evdev code belongs to.
fn key_group_for(evdev: u32) -> Option<usize> {
    for (i, group) in ALL_GROUPS.iter().enumerate() {
        if group.codes.contains(&evdev) {
            return Some(i);
        }
    }
    None
}

/// Escape a char for RON char literal (single-quoted).
fn ron_char(c: char) -> String {
    match c {
        '\'' => "'\\''".to_string(),
        '\\' => "'\\\\'".to_string(),
        '\t' => "'\\t'".to_string(),
        '\n' => "'\\n'".to_string(),
        '\r' => "'\\r'".to_string(),
        '\0' => "'\\0'".to_string(),
        c if c.is_control() => format!("'\\u{{{:04X}}}'", c as u32),
        c => format!("'{}'", c),
    }
}

/// Escape a string for RON string literal (double-quoted).
fn ron_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c if c.is_control() => {
                use std::fmt::Write;
                let _ = write!(out, "\\u{{{:04X}}}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

impl ReadableWKB {
    /// Format as a compact, human-readable RON string.
    pub(crate) fn format_ron(&self) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        out.push_str("(\n");

        // layout_names
        out.push_str("    layout_names: [");
        for (i, name) in self.layout_names.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&ron_string(name));
        }
        out.push_str("],\n");

        // num_keys
        let _ = writeln!(out, "    num_keys: {},", self.num_keys);

        // repeat_keys — add/remove diffs from built-in defaults
        if !self.repeat_keys_add.is_empty() {
            out.push_str("    repeat_keys_add: [\n");
            Self::write_u32_list_by_group(&mut out, &self.repeat_keys_add);
            out.push_str("    ],\n");
        } else {
            out.push_str("    repeat_keys_add: [],\n");
        }
        if !self.repeat_keys_remove.is_empty() {
            out.push_str("    repeat_keys_remove: [\n");
            Self::write_u32_list_by_group(&mut out, &self.repeat_keys_remove);
            out.push_str("    ],\n");
        } else {
            out.push_str("    repeat_keys_remove: [],\n");
        }

        // modifiers — one entry per line
        out.push_str("    modifiers: [\n");
        for (evdev, name, kinds) in &self.modifiers {
            let _ = write!(out, "        ({}, {}, [", evdev, ron_string(name));
            for (i, (level, rmk)) in kinds.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let kind_str = match rmk {
                    ReadableModKind::Pressed(mt) => format!("({}, Pressed({:?}))", level, mt),
                    ReadableModKind::Lock(mt) => format!("({}, Lock({:?}))", level, mt),
                    ReadableModKind::Latch(mt) => format!("({}, Latch({:?}))", level, mt),
                };
                out.push_str(&kind_str);
            }
            out.push_str("]),\n");
        }
        out.push_str("    ],\n");

        // keymap
        Self::write_char_map(&mut out, "keymap", &self.keymap);

        // num_lock_keys
        Self::write_char_map(&mut out, "num_lock_keys", &self.num_lock_keys);

        // caps_lock_keymap
        Self::write_char_map(&mut out, "caps_lock_keymap", &self.caps_lock_keymap);

        // level_exceptions_keymap (diff only)
        #[cfg(feature = "xkb")]
        Self::write_char_map(
            &mut out,
            "level_exceptions_keymap",
            &self.level_exceptions_keymap,
        );

        // keysym_map
        Self::write_keysym_map(&mut out, "keysym_map", &self.keysym_map);

        // compose — one sequence per line
        out.push_str("    compose: [\n");
        for (seq, ch) in &self.compose {
            out.push_str("        ([");
            for (i, s) in seq.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&ron_string(s));
            }
            out.push_str("], ");
            out.push_str(&ron_char(*ch));
            out.push_str("),\n");
        }
        out.push_str("    ],\n");

        out.push_str(")\n");
        out
    }

    /// Write a list of u32 values grouped by keyboard regions.
    fn write_u32_list_by_group(out: &mut String, values: &[u32]) {
        use std::fmt::Write;
        let num_groups = ALL_GROUPS.len();
        let mut groups: Vec<Vec<u32>> = vec![Vec::new(); num_groups];
        let mut overflow: Vec<u32> = Vec::new();

        for &v in values {
            if let Some(g) = key_group_for(v) {
                groups[g].push(v);
            } else {
                overflow.push(v);
            }
        }

        for group in &groups {
            if group.is_empty() {
                continue;
            }
            out.push_str("        ");
            for (i, &v) in group.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}", v);
            }
            out.push_str(",\n");
        }
        if !overflow.is_empty() {
            // Break overflow into lines of ~20 values
            for chunk in overflow.chunks(20) {
                out.push_str("        ");
                for (i, &v) in chunk.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    let _ = write!(out, "{}", v);
                }
                out.push_str(",\n");
            }
        }
    }

    /// Write a char-valued keymap field with keyboard-group visual layout.
    fn write_char_map(
        out: &mut String,
        field: &str,
        map: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, char>>>,
    ) {
        use std::fmt::Write;
        let _ = writeln!(out, "    {}: {{", field);
        for (layout, levels) in map {
            let _ = writeln!(out, "        {}: {{", ron_string(layout));
            for (&level, keys) in levels {
                let _ = write!(out, "            {}: {{", level);
                Self::write_char_entries_by_group(out, keys);
                out.push_str("            },\n");
            }
            out.push_str("        },\n");
        }
        out.push_str("    },\n");
    }

    fn write_keysym_map(
        out: &mut String,
        field: &str,
        map: &BTreeMap<String, BTreeMap<u8, BTreeMap<u32, String>>>,
    ) {
        use std::fmt::Write;
        let _ = writeln!(out, "    {}: {{", field);
        for (layout, levels) in map {
            let _ = writeln!(out, "        {}: {{", ron_string(layout));
            for (&level, keys) in levels {
                let _ = write!(out, "            {}: {{", level);
                if keys.is_empty() {
                    out.push('\n');
                } else {
                    out.push('\n');
                    // Group entries for readability, 10 per line
                    let entries: Vec<_> = keys.iter().collect();
                    for chunk in entries.chunks(10) {
                        out.push_str("                ");
                        for (i, (&evdev, sym_name)) in chunk.iter().enumerate() {
                            if i > 0 {
                                out.push_str(", ");
                            }
                            let _ = write!(out, "{}: {}", evdev, ron_string(sym_name));
                        }
                        out.push_str(",\n");
                    }
                }
                out.push_str("            },\n");
            }
            out.push_str("        },\n");
        }
        out.push_str("    },\n");
    }

    /// Write a keysym-valued map field with keyboard-group visual layout.
    /// Write BTreeMap<u32, char> entries grouped by keyboard visual layout.
    fn write_char_entries_by_group(out: &mut String, keys: &BTreeMap<u32, char>) {
        use std::fmt::Write;
        if keys.is_empty() {
            out.push('\n');
            return;
        }

        let num_groups = ALL_GROUPS.len();
        let mut groups: Vec<Vec<(u32, char)>> = vec![Vec::new(); num_groups];
        let mut overflow: Vec<(u32, char)> = Vec::new();

        for (&evdev, &ch) in keys {
            if let Some(g) = key_group_for(evdev) {
                groups[g].push((evdev, ch));
            } else {
                overflow.push((evdev, ch));
            }
        }

        out.push('\n');

        // Track which section we're in for blank-line separators
        // Groups 0-4: main keyboard, 5: fkeys, 6-8: nav, 9-13: numpad, 14: misc
        let mut last_section = None;
        for (gi, group) in groups.iter().enumerate() {
            if group.is_empty() {
                continue;
            }
            let section = if gi <= 4 {
                0 // main
            } else if gi == 5 {
                1 // fkeys
            } else if gi <= 8 {
                2 // nav
            } else if gi <= 13 {
                3 // numpad
            } else {
                4 // misc
            };
            if let Some(prev) = last_section {
                if section != prev {
                    // Empty line between sections
                    out.push('\n');
                }
            }
            last_section = Some(section);

            out.push_str("                ");
            for (i, (evdev, ch)) in group.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "{}: {}", evdev, ron_char(*ch));
            }
            out.push_str(",\n");
        }
        if !overflow.is_empty() {
            if last_section.is_some() {
                out.push('\n');
            }
            for chunk in overflow.chunks(15) {
                out.push_str("                ");
                for (i, (evdev, ch)) in chunk.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    let _ = write!(out, "{}: {}", evdev, ron_char(*ch));
                }
                out.push_str(",\n");
            }
        }
    }
}

impl crate::WKB {
    /// Serialize this WKB instance to a human-readable RON string.
    pub fn to_ron(&self) -> Result<String, ron::Error> {
        let readable = ReadableWKB::from_wkb(self);
        Ok(readable.format_ron())
    }

    /// Deserialize a WKB instance from a RON string.
    pub fn from_ron(s: &str) -> Result<Self, ron::error::SpannedError> {
        let readable: ReadableWKB = ron::from_str(s)?;
        Ok(readable.to_wkb())
    }
}
