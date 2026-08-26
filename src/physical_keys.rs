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
    MediaRecord,
    MediaRewind,
    MediaFastForward,
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
    DisplayToggleIntExt,
    KeyboardBacklightToggle,
    MailSend,
    MailReply,
    MailForward,
    MicrophoneMuteToggle,
    ZoomToggle,
    LaunchControlPanel,
    SelectTask,
    LaunchScreenSaver,
    LaunchAssistant,
    KeyboardLayoutSelect,
    PrivacyScreenToggle,
    ShowAllWindows,
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
const EVDEV_TO_PHYSICAL: [PhysicalKey; 634] = [
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
    PhysicalKey::ShowAllWindows, // 120
    PhysicalKey::NumpadComma, // 121
    PhysicalKey::Lang1, // 122
    PhysicalKey::Lang2, // 123
    PhysicalKey::IntlYen, // 124
    PhysicalKey::SuperLeft, // 125
    PhysicalKey::SuperRight, // 126
    PhysicalKey::ContextMenu, // 127
    PhysicalKey::BrowserStop, // 128
    PhysicalKey::Again, // 129
    PhysicalKey::Props, // 130
    PhysicalKey::Undo, // 131
    PhysicalKey::Select, // 132
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
    PhysicalKey::LaunchApp1, // 144
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
    PhysicalKey::MediaRecord, // 167
    PhysicalKey::MediaRewind, // 168
    PhysicalKey::Unidentified, // 169
    PhysicalKey::Unidentified, // 170
    PhysicalKey::MediaSelect, // 171
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
    PhysicalKey::MediaPause, // 201
    PhysicalKey::Unidentified, // 202
    PhysicalKey::Unidentified, // 203
    PhysicalKey::Unidentified, // 204
    PhysicalKey::Suspend, // 205
    PhysicalKey::Unidentified, // 206
    PhysicalKey::MediaPlay, // 207
    PhysicalKey::MediaFastForward, // 208
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
    PhysicalKey::DisplayToggleIntExt, // 227
    PhysicalKey::KeyboardBacklightToggle, // 228
    PhysicalKey::KeyboardBrightnessDown, // 229
    PhysicalKey::KeyboardBrightnessUp, // 230
    PhysicalKey::MailSend, // 231
    PhysicalKey::MailReply, // 232
    PhysicalKey::MailForward, // 233
    PhysicalKey::Unidentified, // 234
    PhysicalKey::Unidentified, // 235
    PhysicalKey::Unidentified, // 236
    PhysicalKey::Unidentified, // 237
    PhysicalKey::Unidentified, // 238
    PhysicalKey::Unidentified, // 239
    PhysicalKey::Unidentified, // 240
    PhysicalKey::Unidentified, // 241
    PhysicalKey::Unidentified, // 242
    PhysicalKey::Unidentified, // 243
    PhysicalKey::Unidentified, // 244
    PhysicalKey::Unidentified, // 245
    PhysicalKey::Unidentified, // 246
    PhysicalKey::Unidentified, // 247
    PhysicalKey::MicrophoneMuteToggle, // 248
    PhysicalKey::Unidentified, // 249
    PhysicalKey::Unidentified, // 250
    PhysicalKey::Unidentified, // 251
    PhysicalKey::Unidentified, // 252
    PhysicalKey::Unidentified, // 253
    PhysicalKey::Unidentified, // 254
    PhysicalKey::Unidentified, // 255
    PhysicalKey::Unidentified, // 256
    PhysicalKey::Unidentified, // 257
    PhysicalKey::Unidentified, // 258
    PhysicalKey::Unidentified, // 259
    PhysicalKey::Unidentified, // 260
    PhysicalKey::Unidentified, // 261
    PhysicalKey::Unidentified, // 262
    PhysicalKey::Unidentified, // 263
    PhysicalKey::Unidentified, // 264
    PhysicalKey::Unidentified, // 265
    PhysicalKey::Unidentified, // 266
    PhysicalKey::Unidentified, // 267
    PhysicalKey::Unidentified, // 268
    PhysicalKey::Unidentified, // 269
    PhysicalKey::Unidentified, // 270
    PhysicalKey::Unidentified, // 271
    PhysicalKey::Unidentified, // 272
    PhysicalKey::Unidentified, // 273
    PhysicalKey::Unidentified, // 274
    PhysicalKey::Unidentified, // 275
    PhysicalKey::Unidentified, // 276
    PhysicalKey::Unidentified, // 277
    PhysicalKey::Unidentified, // 278
    PhysicalKey::Unidentified, // 279
    PhysicalKey::Unidentified, // 280
    PhysicalKey::Unidentified, // 281
    PhysicalKey::Unidentified, // 282
    PhysicalKey::Unidentified, // 283
    PhysicalKey::Unidentified, // 284
    PhysicalKey::Unidentified, // 285
    PhysicalKey::Unidentified, // 286
    PhysicalKey::Unidentified, // 287
    PhysicalKey::Unidentified, // 288
    PhysicalKey::Unidentified, // 289
    PhysicalKey::Unidentified, // 290
    PhysicalKey::Unidentified, // 291
    PhysicalKey::Unidentified, // 292
    PhysicalKey::Unidentified, // 293
    PhysicalKey::Unidentified, // 294
    PhysicalKey::Unidentified, // 295
    PhysicalKey::Unidentified, // 296
    PhysicalKey::Unidentified, // 297
    PhysicalKey::Unidentified, // 298
    PhysicalKey::Unidentified, // 299
    PhysicalKey::Unidentified, // 300
    PhysicalKey::Unidentified, // 301
    PhysicalKey::Unidentified, // 302
    PhysicalKey::Unidentified, // 303
    PhysicalKey::Unidentified, // 304
    PhysicalKey::Unidentified, // 305
    PhysicalKey::Unidentified, // 306
    PhysicalKey::Unidentified, // 307
    PhysicalKey::Unidentified, // 308
    PhysicalKey::Unidentified, // 309
    PhysicalKey::Unidentified, // 310
    PhysicalKey::Unidentified, // 311
    PhysicalKey::Unidentified, // 312
    PhysicalKey::Unidentified, // 313
    PhysicalKey::Unidentified, // 314
    PhysicalKey::Unidentified, // 315
    PhysicalKey::Unidentified, // 316
    PhysicalKey::Unidentified, // 317
    PhysicalKey::Unidentified, // 318
    PhysicalKey::Unidentified, // 319
    PhysicalKey::Unidentified, // 320
    PhysicalKey::Unidentified, // 321
    PhysicalKey::Unidentified, // 322
    PhysicalKey::Unidentified, // 323
    PhysicalKey::Unidentified, // 324
    PhysicalKey::Unidentified, // 325
    PhysicalKey::Unidentified, // 326
    PhysicalKey::Unidentified, // 327
    PhysicalKey::Unidentified, // 328
    PhysicalKey::Unidentified, // 329
    PhysicalKey::Unidentified, // 330
    PhysicalKey::Unidentified, // 331
    PhysicalKey::Unidentified, // 332
    PhysicalKey::Unidentified, // 333
    PhysicalKey::Unidentified, // 334
    PhysicalKey::Unidentified, // 335
    PhysicalKey::Unidentified, // 336
    PhysicalKey::Unidentified, // 337
    PhysicalKey::Unidentified, // 338
    PhysicalKey::Unidentified, // 339
    PhysicalKey::Unidentified, // 340
    PhysicalKey::Unidentified, // 341
    PhysicalKey::Unidentified, // 342
    PhysicalKey::Unidentified, // 343
    PhysicalKey::Unidentified, // 344
    PhysicalKey::Unidentified, // 345
    PhysicalKey::Unidentified, // 346
    PhysicalKey::Unidentified, // 347
    PhysicalKey::Unidentified, // 348
    PhysicalKey::Unidentified, // 349
    PhysicalKey::Unidentified, // 350
    PhysicalKey::Unidentified, // 351
    PhysicalKey::Unidentified, // 352
    PhysicalKey::Unidentified, // 353
    PhysicalKey::Unidentified, // 354
    PhysicalKey::Unidentified, // 355
    PhysicalKey::Unidentified, // 356
    PhysicalKey::Unidentified, // 357
    PhysicalKey::Unidentified, // 358
    PhysicalKey::Unidentified, // 359
    PhysicalKey::Unidentified, // 360
    PhysicalKey::Unidentified, // 361
    PhysicalKey::Unidentified, // 362
    PhysicalKey::Unidentified, // 363
    PhysicalKey::Unidentified, // 364
    PhysicalKey::Unidentified, // 365
    PhysicalKey::Unidentified, // 366
    PhysicalKey::Unidentified, // 367
    PhysicalKey::Unidentified, // 368
    PhysicalKey::Unidentified, // 369
    PhysicalKey::Unidentified, // 370
    PhysicalKey::Unidentified, // 371
    PhysicalKey::ZoomToggle, // 372
    PhysicalKey::Unidentified, // 373
    PhysicalKey::Unidentified, // 374
    PhysicalKey::Unidentified, // 375
    PhysicalKey::Unidentified, // 376
    PhysicalKey::Unidentified, // 377
    PhysicalKey::Unidentified, // 378
    PhysicalKey::Unidentified, // 379
    PhysicalKey::Unidentified, // 380
    PhysicalKey::Unidentified, // 381
    PhysicalKey::Unidentified, // 382
    PhysicalKey::Unidentified, // 383
    PhysicalKey::Unidentified, // 384
    PhysicalKey::Unidentified, // 385
    PhysicalKey::Unidentified, // 386
    PhysicalKey::Unidentified, // 387
    PhysicalKey::Unidentified, // 388
    PhysicalKey::Unidentified, // 389
    PhysicalKey::Unidentified, // 390
    PhysicalKey::Unidentified, // 391
    PhysicalKey::Unidentified, // 392
    PhysicalKey::Unidentified, // 393
    PhysicalKey::Unidentified, // 394
    PhysicalKey::Unidentified, // 395
    PhysicalKey::Unidentified, // 396
    PhysicalKey::Unidentified, // 397
    PhysicalKey::Unidentified, // 398
    PhysicalKey::Unidentified, // 399
    PhysicalKey::Unidentified, // 400
    PhysicalKey::Unidentified, // 401
    PhysicalKey::Unidentified, // 402
    PhysicalKey::Unidentified, // 403
    PhysicalKey::Unidentified, // 404
    PhysicalKey::Unidentified, // 405
    PhysicalKey::Unidentified, // 406
    PhysicalKey::Unidentified, // 407
    PhysicalKey::Unidentified, // 408
    PhysicalKey::Unidentified, // 409
    PhysicalKey::Unidentified, // 410
    PhysicalKey::Unidentified, // 411
    PhysicalKey::Unidentified, // 412
    PhysicalKey::Unidentified, // 413
    PhysicalKey::Unidentified, // 414
    PhysicalKey::Unidentified, // 415
    PhysicalKey::Unidentified, // 416
    PhysicalKey::Unidentified, // 417
    PhysicalKey::Unidentified, // 418
    PhysicalKey::Unidentified, // 419
    PhysicalKey::Unidentified, // 420
    PhysicalKey::Unidentified, // 421
    PhysicalKey::Unidentified, // 422
    PhysicalKey::Unidentified, // 423
    PhysicalKey::Unidentified, // 424
    PhysicalKey::Unidentified, // 425
    PhysicalKey::Unidentified, // 426
    PhysicalKey::Unidentified, // 427
    PhysicalKey::Unidentified, // 428
    PhysicalKey::Unidentified, // 429
    PhysicalKey::Unidentified, // 430
    PhysicalKey::Unidentified, // 431
    PhysicalKey::Unidentified, // 432
    PhysicalKey::Unidentified, // 433
    PhysicalKey::Unidentified, // 434
    PhysicalKey::Unidentified, // 435
    PhysicalKey::Unidentified, // 436
    PhysicalKey::Unidentified, // 437
    PhysicalKey::Unidentified, // 438
    PhysicalKey::Unidentified, // 439
    PhysicalKey::Unidentified, // 440
    PhysicalKey::Unidentified, // 441
    PhysicalKey::Unidentified, // 442
    PhysicalKey::Unidentified, // 443
    PhysicalKey::Unidentified, // 444
    PhysicalKey::Unidentified, // 445
    PhysicalKey::Unidentified, // 446
    PhysicalKey::Unidentified, // 447
    PhysicalKey::Unidentified, // 448
    PhysicalKey::Unidentified, // 449
    PhysicalKey::Unidentified, // 450
    PhysicalKey::Unidentified, // 451
    PhysicalKey::Unidentified, // 452
    PhysicalKey::Unidentified, // 453
    PhysicalKey::Unidentified, // 454
    PhysicalKey::Unidentified, // 455
    PhysicalKey::Unidentified, // 456
    PhysicalKey::Unidentified, // 457
    PhysicalKey::Unidentified, // 458
    PhysicalKey::Unidentified, // 459
    PhysicalKey::Unidentified, // 460
    PhysicalKey::Unidentified, // 461
    PhysicalKey::Unidentified, // 462
    PhysicalKey::Unidentified, // 463
    PhysicalKey::Unidentified, // 464
    PhysicalKey::Unidentified, // 465
    PhysicalKey::Unidentified, // 466
    PhysicalKey::Unidentified, // 467
    PhysicalKey::Unidentified, // 468
    PhysicalKey::Unidentified, // 469
    PhysicalKey::Unidentified, // 470
    PhysicalKey::Unidentified, // 471
    PhysicalKey::Unidentified, // 472
    PhysicalKey::Unidentified, // 473
    PhysicalKey::Unidentified, // 474
    PhysicalKey::Unidentified, // 475
    PhysicalKey::Unidentified, // 476
    PhysicalKey::Unidentified, // 477
    PhysicalKey::Unidentified, // 478
    PhysicalKey::Unidentified, // 479
    PhysicalKey::Unidentified, // 480
    PhysicalKey::Unidentified, // 481
    PhysicalKey::Unidentified, // 482
    PhysicalKey::Unidentified, // 483
    PhysicalKey::Unidentified, // 484
    PhysicalKey::Unidentified, // 485
    PhysicalKey::Unidentified, // 486
    PhysicalKey::Unidentified, // 487
    PhysicalKey::Unidentified, // 488
    PhysicalKey::Unidentified, // 489
    PhysicalKey::Unidentified, // 490
    PhysicalKey::Unidentified, // 491
    PhysicalKey::Unidentified, // 492
    PhysicalKey::Unidentified, // 493
    PhysicalKey::Unidentified, // 494
    PhysicalKey::Unidentified, // 495
    PhysicalKey::Unidentified, // 496
    PhysicalKey::Unidentified, // 497
    PhysicalKey::Unidentified, // 498
    PhysicalKey::Unidentified, // 499
    PhysicalKey::Unidentified, // 500
    PhysicalKey::Unidentified, // 501
    PhysicalKey::Unidentified, // 502
    PhysicalKey::Unidentified, // 503
    PhysicalKey::Unidentified, // 504
    PhysicalKey::Unidentified, // 505
    PhysicalKey::Unidentified, // 506
    PhysicalKey::Unidentified, // 507
    PhysicalKey::Unidentified, // 508
    PhysicalKey::Unidentified, // 509
    PhysicalKey::Unidentified, // 510
    PhysicalKey::Unidentified, // 511
    PhysicalKey::Unidentified, // 512
    PhysicalKey::Unidentified, // 513
    PhysicalKey::Unidentified, // 514
    PhysicalKey::Unidentified, // 515
    PhysicalKey::Unidentified, // 516
    PhysicalKey::Unidentified, // 517
    PhysicalKey::Unidentified, // 518
    PhysicalKey::Unidentified, // 519
    PhysicalKey::Unidentified, // 520
    PhysicalKey::Unidentified, // 521
    PhysicalKey::Unidentified, // 522
    PhysicalKey::Unidentified, // 523
    PhysicalKey::Unidentified, // 524
    PhysicalKey::Unidentified, // 525
    PhysicalKey::Unidentified, // 526
    PhysicalKey::Unidentified, // 527
    PhysicalKey::Unidentified, // 528
    PhysicalKey::Unidentified, // 529
    PhysicalKey::Unidentified, // 530
    PhysicalKey::Unidentified, // 531
    PhysicalKey::Unidentified, // 532
    PhysicalKey::Unidentified, // 533
    PhysicalKey::Unidentified, // 534
    PhysicalKey::Unidentified, // 535
    PhysicalKey::Unidentified, // 536
    PhysicalKey::Unidentified, // 537
    PhysicalKey::Unidentified, // 538
    PhysicalKey::Unidentified, // 539
    PhysicalKey::Unidentified, // 540
    PhysicalKey::Unidentified, // 541
    PhysicalKey::Unidentified, // 542
    PhysicalKey::Unidentified, // 543
    PhysicalKey::Unidentified, // 544
    PhysicalKey::Unidentified, // 545
    PhysicalKey::Unidentified, // 546
    PhysicalKey::Unidentified, // 547
    PhysicalKey::Unidentified, // 548
    PhysicalKey::Unidentified, // 549
    PhysicalKey::Unidentified, // 550
    PhysicalKey::Unidentified, // 551
    PhysicalKey::Unidentified, // 552
    PhysicalKey::Unidentified, // 553
    PhysicalKey::Unidentified, // 554
    PhysicalKey::Unidentified, // 555
    PhysicalKey::Unidentified, // 556
    PhysicalKey::Unidentified, // 557
    PhysicalKey::Unidentified, // 558
    PhysicalKey::Unidentified, // 559
    PhysicalKey::Unidentified, // 560
    PhysicalKey::Unidentified, // 561
    PhysicalKey::Unidentified, // 562
    PhysicalKey::Unidentified, // 563
    PhysicalKey::Unidentified, // 564
    PhysicalKey::Unidentified, // 565
    PhysicalKey::Unidentified, // 566
    PhysicalKey::Unidentified, // 567
    PhysicalKey::Unidentified, // 568
    PhysicalKey::Unidentified, // 569
    PhysicalKey::Unidentified, // 570
    PhysicalKey::Unidentified, // 571
    PhysicalKey::Unidentified, // 572
    PhysicalKey::Unidentified, // 573
    PhysicalKey::Unidentified, // 574
    PhysicalKey::Unidentified, // 575
    PhysicalKey::Unidentified, // 576
    PhysicalKey::Unidentified, // 577
    PhysicalKey::Unidentified, // 578
    PhysicalKey::LaunchControlPanel, // 579
    PhysicalKey::SelectTask, // 580
    PhysicalKey::LaunchScreenSaver, // 581
    PhysicalKey::Unidentified, // 582
    PhysicalKey::LaunchAssistant, // 583
    PhysicalKey::KeyboardLayoutSelect, // 584
    PhysicalKey::Unidentified, // 585
    PhysicalKey::Unidentified, // 586
    PhysicalKey::Unidentified, // 587
    PhysicalKey::Unidentified, // 588
    PhysicalKey::Unidentified, // 589
    PhysicalKey::Unidentified, // 590
    PhysicalKey::Unidentified, // 591
    PhysicalKey::Unidentified, // 592
    PhysicalKey::Unidentified, // 593
    PhysicalKey::Unidentified, // 594
    PhysicalKey::Unidentified, // 595
    PhysicalKey::Unidentified, // 596
    PhysicalKey::Unidentified, // 597
    PhysicalKey::Unidentified, // 598
    PhysicalKey::Unidentified, // 599
    PhysicalKey::Unidentified, // 600
    PhysicalKey::Unidentified, // 601
    PhysicalKey::Unidentified, // 602
    PhysicalKey::Unidentified, // 603
    PhysicalKey::Unidentified, // 604
    PhysicalKey::Unidentified, // 605
    PhysicalKey::Unidentified, // 606
    PhysicalKey::Unidentified, // 607
    PhysicalKey::Unidentified, // 608
    PhysicalKey::Unidentified, // 609
    PhysicalKey::Unidentified, // 610
    PhysicalKey::Unidentified, // 611
    PhysicalKey::Unidentified, // 612
    PhysicalKey::Unidentified, // 613
    PhysicalKey::Unidentified, // 614
    PhysicalKey::Unidentified, // 615
    PhysicalKey::Unidentified, // 616
    PhysicalKey::Unidentified, // 617
    PhysicalKey::Unidentified, // 618
    PhysicalKey::Unidentified, // 619
    PhysicalKey::Unidentified, // 620
    PhysicalKey::Unidentified, // 621
    PhysicalKey::Unidentified, // 622
    PhysicalKey::Unidentified, // 623
    PhysicalKey::Unidentified, // 624
    PhysicalKey::Unidentified, // 625
    PhysicalKey::Unidentified, // 626
    PhysicalKey::Unidentified, // 627
    PhysicalKey::Unidentified, // 628
    PhysicalKey::Unidentified, // 629
    PhysicalKey::Unidentified, // 630
    PhysicalKey::Unidentified, // 631
    PhysicalKey::Unidentified, // 632
    PhysicalKey::PrivacyScreenToggle, // 633
];

impl PhysicalKey {
    /// Map a raw Linux/evdev keycode to a physical key position.
    ///
    /// Unknown codes return [`PhysicalKey::Unidentified`]. This mapping
    /// depends only on the evdev code — never on layout, modifiers, or compose.
    #[inline]
    pub fn from_evdev(evdev_code: u32) -> Self {
        EVDEV_TO_PHYSICAL
            .get(evdev_code as usize)
            .copied()
            .unwrap_or(PhysicalKey::Unidentified)
    }
}
