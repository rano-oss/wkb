//! X11 keysym constants and utilities.
//!
//! Auto-generated from xkb-core keysym data. All ~2,500 standard X11 keysyms.

#![allow(non_upper_case_globals)]

/// No symbol (empty/invalid keysym).
pub const NoSymbol: u32 = 0;

// pub const _0: u32 = 0x30;
pub const _1: u32 = 0x31;
// pub const _2: u32 = 0x32;
pub const _3: u32 = 0x33;
// pub const _4: u32 = 0x34;
// pub const _5: u32 = 0x35;
// pub const _6: u32 = 0x36;
// pub const _7: u32 = 0x37;
// pub const _8: u32 = 0x38;
pub const _9: u32 = 0x39;
// pub const _3270_AltCursor: u32 = 0xfd10;
pub const _3270_Attn: u32 = 0xfd0e;
// pub const _3270_BackTab: u32 = 0xfd05;
// pub const _3270_ChangeScreen: u32 = 0xfd19;
// pub const _3270_Copy: u32 = 0xfd15;
// pub const _3270_CursorBlink: u32 = 0xfd0f;
pub const _3270_CursorSelect: u32 = 0xfd1c;
// pub const _3270_DeleteWord: u32 = 0xfd1a;
// pub const _3270_Duplicate: u32 = 0xfd01;
pub const _3270_Enter: u32 = 0xfd1e;
pub const _3270_EraseEOF: u32 = 0xfd06;
// pub const _3270_EraseInput: u32 = 0xfd07;
pub const _3270_ExSelect: u32 = 0xfd1b;
// pub const _3270_FieldMark: u32 = 0xfd02;
// pub const _3270_Ident: u32 = 0xfd13;
// pub const _3270_Jump: u32 = 0xfd12;
// pub const _3270_KeyClick: u32 = 0xfd11;
// pub const _3270_Left2: u32 = 0xfd04;
// pub const _3270_PA1: u32 = 0xfd0a;
// pub const _3270_PA2: u32 = 0xfd0b;
// pub const _3270_PA3: u32 = 0xfd0c;
pub const _3270_Play: u32 = 0xfd16;
pub const _3270_PrintScreen: u32 = 0xfd1d;
// pub const _3270_Quit: u32 = 0xfd09;
// pub const _3270_Record: u32 = 0xfd18;
// pub const _3270_Reset: u32 = 0xfd08;
// pub const _3270_Right2: u32 = 0xfd03;
// pub const _3270_Rule: u32 = 0xfd14;
// pub const _3270_Setup: u32 = 0xfd17;
// pub const _3270_Test: u32 = 0xfd0d;
pub const a: u32 = 0x61;
pub const A: u32 = 0x41;
// pub const aacute: u32 = 0xe1;
// pub const Aacute: u32 = 0xc1;
// pub const abelowdot: u32 = 0x1001ea1;
// pub const Abelowdot: u32 = 0x1001ea0;
// pub const abovedot: u32 = 0x1ff;
// pub const abreve: u32 = 0x1e3;
// pub const Abreve: u32 = 0x1c3;
// pub const abreveacute: u32 = 0x1001eaf;
// pub const Abreveacute: u32 = 0x1001eae;
// pub const abrevebelowdot: u32 = 0x1001eb7;
// pub const Abrevebelowdot: u32 = 0x1001eb6;
// pub const abrevegrave: u32 = 0x1001eb1;
// pub const Abrevegrave: u32 = 0x1001eb0;
// pub const abrevehook: u32 = 0x1001eb3;
// pub const Abrevehook: u32 = 0x1001eb2;
// pub const abrevetilde: u32 = 0x1001eb5;
// pub const Abrevetilde: u32 = 0x1001eb4;
// pub const AccessX_Enable: u32 = 0xfe70;
// pub const AccessX_Feedback_Enable: u32 = 0xfe71;
// pub const acircumflex: u32 = 0xe2;
// pub const Acircumflex: u32 = 0xc2;
// pub const acircumflexacute: u32 = 0x1001ea5;
// pub const Acircumflexacute: u32 = 0x1001ea4;
// pub const acircumflexbelowdot: u32 = 0x1001ead;
// pub const Acircumflexbelowdot: u32 = 0x1001eac;
// pub const acircumflexgrave: u32 = 0x1001ea7;
// pub const Acircumflexgrave: u32 = 0x1001ea6;
// pub const acircumflexhook: u32 = 0x1001ea9;
// pub const Acircumflexhook: u32 = 0x1001ea8;
// pub const acircumflextilde: u32 = 0x1001eab;
// pub const Acircumflextilde: u32 = 0x1001eaa;
// pub const acute: u32 = 0xb4;
// pub const adiaeresis: u32 = 0xe4;
// pub const Adiaeresis: u32 = 0xc4;
// pub const ae: u32 = 0xe6;
// pub const AE: u32 = 0xc6;
// pub const agrave: u32 = 0xe0;
// pub const Agrave: u32 = 0xc0;
// pub const ahook: u32 = 0x1001ea3;
// pub const Ahook: u32 = 0x1001ea2;
pub const Alt_L: u32 = 0xffe9;
pub const Alt_R: u32 = 0xffea;
// pub const amacron: u32 = 0x3e0;
// pub const Amacron: u32 = 0x3c0;
// pub const ampersand: u32 = 0x26;
// pub const aogonek: u32 = 0x1b1;
// pub const Aogonek: u32 = 0x1a1;
// pub const apostrophe: u32 = 0x27;
// pub const approxeq: u32 = 0x1002248;
// pub const approximate: u32 = 0x8c8;
// pub const Arabic_0: u32 = 0x1000660;
// pub const Arabic_1: u32 = 0x1000661;
// pub const Arabic_2: u32 = 0x1000662;
// pub const Arabic_3: u32 = 0x1000663;
// pub const Arabic_4: u32 = 0x1000664;
// pub const Arabic_5: u32 = 0x1000665;
// pub const Arabic_6: u32 = 0x1000666;
// pub const Arabic_7: u32 = 0x1000667;
// pub const Arabic_8: u32 = 0x1000668;
// pub const Arabic_9: u32 = 0x1000669;
// pub const Arabic_ain: u32 = 0x5d9;
// pub const Arabic_alef: u32 = 0x5c7;
// pub const Arabic_alefmaksura: u32 = 0x5e9;
// pub const Arabic_beh: u32 = 0x5c8;
// pub const Arabic_comma: u32 = 0x5ac;
// pub const Arabic_dad: u32 = 0x5d6;
// pub const Arabic_dal: u32 = 0x5cf;
// pub const Arabic_damma: u32 = 0x5ef;
// pub const Arabic_dammatan: u32 = 0x5ec;
// pub const Arabic_ddal: u32 = 0x1000688;
// pub const Arabic_farsi_yeh: u32 = 0x10006cc;
// pub const Arabic_fatha: u32 = 0x5ee;
// pub const Arabic_fathatan: u32 = 0x5eb;
// pub const Arabic_feh: u32 = 0x5e1;
// pub const Arabic_fullstop: u32 = 0x10006d4;
// pub const Arabic_gaf: u32 = 0x10006af;
// pub const Arabic_ghain: u32 = 0x5da;
// pub const Arabic_ha: u32 = 0x5e7;
// pub const Arabic_hah: u32 = 0x5cd;
// pub const Arabic_hamza: u32 = 0x5c1;
// pub const Arabic_hamza_above: u32 = 0x1000654;
// pub const Arabic_hamza_below: u32 = 0x1000655;
// pub const Arabic_hamzaonalef: u32 = 0x5c3;
// pub const Arabic_hamzaonwaw: u32 = 0x5c4;
// pub const Arabic_hamzaonyeh: u32 = 0x5c6;
// pub const Arabic_hamzaunderalef: u32 = 0x5c5;
// pub const Arabic_heh: u32 = 0x5e7;
// pub const Arabic_heh_doachashmee: u32 = 0x10006be;
// pub const Arabic_heh_goal: u32 = 0x10006c1;
// pub const Arabic_jeem: u32 = 0x5cc;
// pub const Arabic_jeh: u32 = 0x1000698;
// pub const Arabic_kaf: u32 = 0x5e3;
// pub const Arabic_kasra: u32 = 0x5f0;
// pub const Arabic_kasratan: u32 = 0x5ed;
// pub const Arabic_keheh: u32 = 0x10006a9;
// pub const Arabic_khah: u32 = 0x5ce;
// pub const Arabic_lam: u32 = 0x5e4;
// pub const Arabic_madda_above: u32 = 0x1000653;
// pub const Arabic_maddaonalef: u32 = 0x5c2;
// pub const Arabic_meem: u32 = 0x5e5;
// pub const Arabic_noon: u32 = 0x5e6;
// pub const Arabic_noon_ghunna: u32 = 0x10006ba;
// pub const Arabic_peh: u32 = 0x100067e;
// pub const Arabic_percent: u32 = 0x100066a;
// pub const Arabic_qaf: u32 = 0x5e2;
// pub const Arabic_question_mark: u32 = 0x5bf;
// pub const Arabic_ra: u32 = 0x5d1;
// pub const Arabic_rreh: u32 = 0x1000691;
// pub const Arabic_sad: u32 = 0x5d5;
// pub const Arabic_seen: u32 = 0x5d3;
// pub const Arabic_semicolon: u32 = 0x5bb;
// pub const Arabic_shadda: u32 = 0x5f1;
// pub const Arabic_sheen: u32 = 0x5d4;
// pub const Arabic_sukun: u32 = 0x5f2;
// pub const Arabic_superscript_alef: u32 = 0x1000670;
// pub const Arabic_switch: u32 = 0xff7e;
// pub const Arabic_tah: u32 = 0x5d7;
// pub const Arabic_tatweel: u32 = 0x5e0;
// pub const Arabic_tcheh: u32 = 0x1000686;
// pub const Arabic_teh: u32 = 0x5ca;
// pub const Arabic_tehmarbuta: u32 = 0x5c9;
// pub const Arabic_thal: u32 = 0x5d0;
// pub const Arabic_theh: u32 = 0x5cb;
// pub const Arabic_tteh: u32 = 0x1000679;
// pub const Arabic_veh: u32 = 0x10006a4;
// pub const Arabic_waw: u32 = 0x5e8;
// pub const Arabic_yeh: u32 = 0x5ea;
// pub const Arabic_yeh_baree: u32 = 0x10006d2;
// pub const Arabic_zah: u32 = 0x5d8;
// pub const Arabic_zain: u32 = 0x5d2;
// pub const aring: u32 = 0xe5;
// pub const Aring: u32 = 0xc5;
// pub const Armenian_accent: u32 = 0x100055b;
// pub const Armenian_amanak: u32 = 0x100055c;
// pub const Armenian_apostrophe: u32 = 0x100055a;
// pub const Armenian_at: u32 = 0x1000568;
// pub const Armenian_AT: u32 = 0x1000538;
// pub const Armenian_ayb: u32 = 0x1000561;
// pub const Armenian_AYB: u32 = 0x1000531;
// pub const Armenian_ben: u32 = 0x1000562;
// pub const Armenian_BEN: u32 = 0x1000532;
// pub const Armenian_but: u32 = 0x100055d;
// pub const Armenian_cha: u32 = 0x1000579;
// pub const Armenian_CHA: u32 = 0x1000549;
// pub const Armenian_da: u32 = 0x1000564;
// pub const Armenian_DA: u32 = 0x1000534;
// pub const Armenian_dza: u32 = 0x1000571;
// pub const Armenian_DZA: u32 = 0x1000541;
// pub const Armenian_e: u32 = 0x1000567;
// pub const Armenian_E: u32 = 0x1000537;
// pub const Armenian_exclam: u32 = 0x100055c;
// pub const Armenian_fe: u32 = 0x1000586;
// pub const Armenian_FE: u32 = 0x1000556;
// pub const Armenian_full_stop: u32 = 0x1000589;
// pub const Armenian_ghat: u32 = 0x1000572;
// pub const Armenian_GHAT: u32 = 0x1000542;
// pub const Armenian_gim: u32 = 0x1000563;
// pub const Armenian_GIM: u32 = 0x1000533;
// pub const Armenian_hi: u32 = 0x1000575;
// pub const Armenian_HI: u32 = 0x1000545;
// pub const Armenian_ho: u32 = 0x1000570;
// pub const Armenian_HO: u32 = 0x1000540;
// pub const Armenian_hyphen: u32 = 0x100058a;
// pub const Armenian_ini: u32 = 0x100056b;
// pub const Armenian_INI: u32 = 0x100053b;
// pub const Armenian_je: u32 = 0x100057b;
// pub const Armenian_JE: u32 = 0x100054b;
// pub const Armenian_ke: u32 = 0x1000584;
// pub const Armenian_KE: u32 = 0x1000554;
// pub const Armenian_ken: u32 = 0x100056f;
// pub const Armenian_KEN: u32 = 0x100053f;
// pub const Armenian_khe: u32 = 0x100056d;
// pub const Armenian_KHE: u32 = 0x100053d;
// pub const Armenian_ligature_ew: u32 = 0x1000587;
// pub const Armenian_lyun: u32 = 0x100056c;
// pub const Armenian_LYUN: u32 = 0x100053c;
// pub const Armenian_men: u32 = 0x1000574;
// pub const Armenian_MEN: u32 = 0x1000544;
// pub const Armenian_nu: u32 = 0x1000576;
// pub const Armenian_NU: u32 = 0x1000546;
// pub const Armenian_o: u32 = 0x1000585;
// pub const Armenian_O: u32 = 0x1000555;
// pub const Armenian_paruyk: u32 = 0x100055e;
// pub const Armenian_pe: u32 = 0x100057a;
// pub const Armenian_PE: u32 = 0x100054a;
// pub const Armenian_pyur: u32 = 0x1000583;
// pub const Armenian_PYUR: u32 = 0x1000553;
// pub const Armenian_question: u32 = 0x100055e;
// pub const Armenian_ra: u32 = 0x100057c;
// pub const Armenian_RA: u32 = 0x100054c;
// pub const Armenian_re: u32 = 0x1000580;
// pub const Armenian_RE: u32 = 0x1000550;
// pub const Armenian_se: u32 = 0x100057d;
// pub const Armenian_SE: u32 = 0x100054d;
// pub const Armenian_separation_mark: u32 = 0x100055d;
// pub const Armenian_sha: u32 = 0x1000577;
// pub const Armenian_SHA: u32 = 0x1000547;
// pub const Armenian_shesht: u32 = 0x100055b;
// pub const Armenian_tche: u32 = 0x1000573;
// pub const Armenian_TCHE: u32 = 0x1000543;
// pub const Armenian_to: u32 = 0x1000569;
// pub const Armenian_TO: u32 = 0x1000539;
// pub const Armenian_tsa: u32 = 0x100056e;
// pub const Armenian_TSA: u32 = 0x100053e;
// pub const Armenian_tso: u32 = 0x1000581;
// pub const Armenian_TSO: u32 = 0x1000551;
// pub const Armenian_tyun: u32 = 0x100057f;
// pub const Armenian_TYUN: u32 = 0x100054f;
// pub const Armenian_verjaket: u32 = 0x1000589;
// pub const Armenian_vev: u32 = 0x100057e;
// pub const Armenian_VEV: u32 = 0x100054e;
// pub const Armenian_vo: u32 = 0x1000578;
// pub const Armenian_VO: u32 = 0x1000548;
// pub const Armenian_vyun: u32 = 0x1000582;
// pub const Armenian_VYUN: u32 = 0x1000552;
// pub const Armenian_yech: u32 = 0x1000565;
// pub const Armenian_YECH: u32 = 0x1000535;
// pub const Armenian_yentamna: u32 = 0x100058a;
// pub const Armenian_za: u32 = 0x1000566;
// pub const Armenian_ZA: u32 = 0x1000536;
// pub const Armenian_zhe: u32 = 0x100056a;
// pub const Armenian_ZHE: u32 = 0x100053a;
// pub const asciicircum: u32 = 0x5e;
// pub const asciitilde: u32 = 0x7e;
// pub const asterisk: u32 = 0x2a;
// pub const at: u32 = 0x40;
// pub const atilde: u32 = 0xe3;
// pub const Atilde: u32 = 0xc3;
// pub const AudibleBell_Enable: u32 = 0xfe7a;
// pub const b: u32 = 0x62;
pub const B: u32 = 0x42;
// pub const babovedot: u32 = 0x1001e03;
// pub const Babovedot: u32 = 0x1001e02;
// pub const backslash: u32 = 0x5c;
pub const BackSpace: u32 = 0xff08;
// pub const BackTab: u32 = 0x1000ff74;
// pub const ballotcross: u32 = 0xaf4;
// pub const bar: u32 = 0x7c;
// pub const because: u32 = 0x1002235;
pub const Begin: u32 = 0xff58;
// pub const blank: u32 = 0x9df;
// pub const block: u32 = 0x100000fc;
// pub const botintegral: u32 = 0x8a5;
// pub const botleftparens: u32 = 0x8ac;
// pub const botleftsqbracket: u32 = 0x8a8;
// pub const botleftsummation: u32 = 0x8b2;
// pub const botrightparens: u32 = 0x8ae;
// pub const botrightsqbracket: u32 = 0x8aa;
// pub const botrightsummation: u32 = 0x8b6;
// pub const bott: u32 = 0x9f6;
// pub const botvertsummationconnector: u32 = 0x8b4;
// pub const BounceKeys_Enable: u32 = 0xfe74;
// pub const braceleft: u32 = 0x7b;
// pub const braceright: u32 = 0x7d;
// pub const bracketleft: u32 = 0x5b;
// pub const bracketright: u32 = 0x5d;
// pub const braille_blank: u32 = 0x1002800;
// pub const braille_dot_1: u32 = 0xfff1;
// pub const braille_dot_10: u32 = 0xfffa;
// pub const braille_dot_2: u32 = 0xfff2;
// pub const braille_dot_3: u32 = 0xfff3;
// pub const braille_dot_4: u32 = 0xfff4;
// pub const braille_dot_5: u32 = 0xfff5;
// pub const braille_dot_6: u32 = 0xfff6;
// pub const braille_dot_7: u32 = 0xfff7;
// pub const braille_dot_8: u32 = 0xfff8;
// pub const braille_dot_9: u32 = 0xfff9;
// pub const braille_dots_1: u32 = 0x1002801;
// pub const braille_dots_12: u32 = 0x1002803;
// pub const braille_dots_123: u32 = 0x1002807;
// pub const braille_dots_1234: u32 = 0x100280f;
// pub const braille_dots_12345: u32 = 0x100281f;
// pub const braille_dots_123456: u32 = 0x100283f;
// pub const braille_dots_1234567: u32 = 0x100287f;
// pub const braille_dots_12345678: u32 = 0x10028ff;
// pub const braille_dots_1234568: u32 = 0x10028bf;
// pub const braille_dots_123457: u32 = 0x100285f;
// pub const braille_dots_1234578: u32 = 0x10028df;
// pub const braille_dots_123458: u32 = 0x100289f;
// pub const braille_dots_12346: u32 = 0x100282f;
// pub const braille_dots_123467: u32 = 0x100286f;
// pub const braille_dots_1234678: u32 = 0x10028ef;
// pub const braille_dots_123468: u32 = 0x10028af;
// pub const braille_dots_12347: u32 = 0x100284f;
// pub const braille_dots_123478: u32 = 0x10028cf;
// pub const braille_dots_12348: u32 = 0x100288f;
// pub const braille_dots_1235: u32 = 0x1002817;
// pub const braille_dots_12356: u32 = 0x1002837;
// pub const braille_dots_123567: u32 = 0x1002877;
// pub const braille_dots_1235678: u32 = 0x10028f7;
// pub const braille_dots_123568: u32 = 0x10028b7;
// pub const braille_dots_12357: u32 = 0x1002857;
// pub const braille_dots_123578: u32 = 0x10028d7;
// pub const braille_dots_12358: u32 = 0x1002897;
// pub const braille_dots_1236: u32 = 0x1002827;
// pub const braille_dots_12367: u32 = 0x1002867;
// pub const braille_dots_123678: u32 = 0x10028e7;
// pub const braille_dots_12368: u32 = 0x10028a7;
// pub const braille_dots_1237: u32 = 0x1002847;
// pub const braille_dots_12378: u32 = 0x10028c7;
// pub const braille_dots_1238: u32 = 0x1002887;
// pub const braille_dots_124: u32 = 0x100280b;
// pub const braille_dots_1245: u32 = 0x100281b;
// pub const braille_dots_12456: u32 = 0x100283b;
// pub const braille_dots_124567: u32 = 0x100287b;
// pub const braille_dots_1245678: u32 = 0x10028fb;
// pub const braille_dots_124568: u32 = 0x10028bb;
// pub const braille_dots_12457: u32 = 0x100285b;
// pub const braille_dots_124578: u32 = 0x10028db;
// pub const braille_dots_12458: u32 = 0x100289b;
// pub const braille_dots_1246: u32 = 0x100282b;
// pub const braille_dots_12467: u32 = 0x100286b;
// pub const braille_dots_124678: u32 = 0x10028eb;
// pub const braille_dots_12468: u32 = 0x10028ab;
// pub const braille_dots_1247: u32 = 0x100284b;
// pub const braille_dots_12478: u32 = 0x10028cb;
// pub const braille_dots_1248: u32 = 0x100288b;
// pub const braille_dots_125: u32 = 0x1002813;
// pub const braille_dots_1256: u32 = 0x1002833;
// pub const braille_dots_12567: u32 = 0x1002873;
// pub const braille_dots_125678: u32 = 0x10028f3;
// pub const braille_dots_12568: u32 = 0x10028b3;
// pub const braille_dots_1257: u32 = 0x1002853;
// pub const braille_dots_12578: u32 = 0x10028d3;
// pub const braille_dots_1258: u32 = 0x1002893;
// pub const braille_dots_126: u32 = 0x1002823;
// pub const braille_dots_1267: u32 = 0x1002863;
// pub const braille_dots_12678: u32 = 0x10028e3;
// pub const braille_dots_1268: u32 = 0x10028a3;
// pub const braille_dots_127: u32 = 0x1002843;
// pub const braille_dots_1278: u32 = 0x10028c3;
// pub const braille_dots_128: u32 = 0x1002883;
// pub const braille_dots_13: u32 = 0x1002805;
// pub const braille_dots_134: u32 = 0x100280d;
// pub const braille_dots_1345: u32 = 0x100281d;
// pub const braille_dots_13456: u32 = 0x100283d;
// pub const braille_dots_134567: u32 = 0x100287d;
// pub const braille_dots_1345678: u32 = 0x10028fd;
// pub const braille_dots_134568: u32 = 0x10028bd;
// pub const braille_dots_13457: u32 = 0x100285d;
// pub const braille_dots_134578: u32 = 0x10028dd;
// pub const braille_dots_13458: u32 = 0x100289d;
// pub const braille_dots_1346: u32 = 0x100282d;
// pub const braille_dots_13467: u32 = 0x100286d;
// pub const braille_dots_134678: u32 = 0x10028ed;
// pub const braille_dots_13468: u32 = 0x10028ad;
// pub const braille_dots_1347: u32 = 0x100284d;
// pub const braille_dots_13478: u32 = 0x10028cd;
// pub const braille_dots_1348: u32 = 0x100288d;
// pub const braille_dots_135: u32 = 0x1002815;
// pub const braille_dots_1356: u32 = 0x1002835;
// pub const braille_dots_13567: u32 = 0x1002875;
// pub const braille_dots_135678: u32 = 0x10028f5;
// pub const braille_dots_13568: u32 = 0x10028b5;
// pub const braille_dots_1357: u32 = 0x1002855;
// pub const braille_dots_13578: u32 = 0x10028d5;
// pub const braille_dots_1358: u32 = 0x1002895;
// pub const braille_dots_136: u32 = 0x1002825;
// pub const braille_dots_1367: u32 = 0x1002865;
// pub const braille_dots_13678: u32 = 0x10028e5;
// pub const braille_dots_1368: u32 = 0x10028a5;
// pub const braille_dots_137: u32 = 0x1002845;
// pub const braille_dots_1378: u32 = 0x10028c5;
// pub const braille_dots_138: u32 = 0x1002885;
// pub const braille_dots_14: u32 = 0x1002809;
// pub const braille_dots_145: u32 = 0x1002819;
// pub const braille_dots_1456: u32 = 0x1002839;
// pub const braille_dots_14567: u32 = 0x1002879;
// pub const braille_dots_145678: u32 = 0x10028f9;
// pub const braille_dots_14568: u32 = 0x10028b9;
// pub const braille_dots_1457: u32 = 0x1002859;
// pub const braille_dots_14578: u32 = 0x10028d9;
// pub const braille_dots_1458: u32 = 0x1002899;
// pub const braille_dots_146: u32 = 0x1002829;
// pub const braille_dots_1467: u32 = 0x1002869;
// pub const braille_dots_14678: u32 = 0x10028e9;
// pub const braille_dots_1468: u32 = 0x10028a9;
// pub const braille_dots_147: u32 = 0x1002849;
// pub const braille_dots_1478: u32 = 0x10028c9;
// pub const braille_dots_148: u32 = 0x1002889;
// pub const braille_dots_15: u32 = 0x1002811;
// pub const braille_dots_156: u32 = 0x1002831;
// pub const braille_dots_1567: u32 = 0x1002871;
// pub const braille_dots_15678: u32 = 0x10028f1;
// pub const braille_dots_1568: u32 = 0x10028b1;
// pub const braille_dots_157: u32 = 0x1002851;
// pub const braille_dots_1578: u32 = 0x10028d1;
// pub const braille_dots_158: u32 = 0x1002891;
// pub const braille_dots_16: u32 = 0x1002821;
// pub const braille_dots_167: u32 = 0x1002861;
// pub const braille_dots_1678: u32 = 0x10028e1;
// pub const braille_dots_168: u32 = 0x10028a1;
// pub const braille_dots_17: u32 = 0x1002841;
// pub const braille_dots_178: u32 = 0x10028c1;
// pub const braille_dots_18: u32 = 0x1002881;
// pub const braille_dots_2: u32 = 0x1002802;
// pub const braille_dots_23: u32 = 0x1002806;
// pub const braille_dots_234: u32 = 0x100280e;
// pub const braille_dots_2345: u32 = 0x100281e;
// pub const braille_dots_23456: u32 = 0x100283e;
// pub const braille_dots_234567: u32 = 0x100287e;
// pub const braille_dots_2345678: u32 = 0x10028fe;
// pub const braille_dots_234568: u32 = 0x10028be;
// pub const braille_dots_23457: u32 = 0x100285e;
// pub const braille_dots_234578: u32 = 0x10028de;
// pub const braille_dots_23458: u32 = 0x100289e;
// pub const braille_dots_2346: u32 = 0x100282e;
// pub const braille_dots_23467: u32 = 0x100286e;
// pub const braille_dots_234678: u32 = 0x10028ee;
// pub const braille_dots_23468: u32 = 0x10028ae;
// pub const braille_dots_2347: u32 = 0x100284e;
// pub const braille_dots_23478: u32 = 0x10028ce;
// pub const braille_dots_2348: u32 = 0x100288e;
// pub const braille_dots_235: u32 = 0x1002816;
// pub const braille_dots_2356: u32 = 0x1002836;
// pub const braille_dots_23567: u32 = 0x1002876;
// pub const braille_dots_235678: u32 = 0x10028f6;
// pub const braille_dots_23568: u32 = 0x10028b6;
// pub const braille_dots_2357: u32 = 0x1002856;
// pub const braille_dots_23578: u32 = 0x10028d6;
// pub const braille_dots_2358: u32 = 0x1002896;
// pub const braille_dots_236: u32 = 0x1002826;
// pub const braille_dots_2367: u32 = 0x1002866;
// pub const braille_dots_23678: u32 = 0x10028e6;
// pub const braille_dots_2368: u32 = 0x10028a6;
// pub const braille_dots_237: u32 = 0x1002846;
// pub const braille_dots_2378: u32 = 0x10028c6;
// pub const braille_dots_238: u32 = 0x1002886;
// pub const braille_dots_24: u32 = 0x100280a;
// pub const braille_dots_245: u32 = 0x100281a;
// pub const braille_dots_2456: u32 = 0x100283a;
// pub const braille_dots_24567: u32 = 0x100287a;
// pub const braille_dots_245678: u32 = 0x10028fa;
// pub const braille_dots_24568: u32 = 0x10028ba;
// pub const braille_dots_2457: u32 = 0x100285a;
// pub const braille_dots_24578: u32 = 0x10028da;
// pub const braille_dots_2458: u32 = 0x100289a;
// pub const braille_dots_246: u32 = 0x100282a;
// pub const braille_dots_2467: u32 = 0x100286a;
// pub const braille_dots_24678: u32 = 0x10028ea;
// pub const braille_dots_2468: u32 = 0x10028aa;
// pub const braille_dots_247: u32 = 0x100284a;
// pub const braille_dots_2478: u32 = 0x10028ca;
// pub const braille_dots_248: u32 = 0x100288a;
// pub const braille_dots_25: u32 = 0x1002812;
// pub const braille_dots_256: u32 = 0x1002832;
// pub const braille_dots_2567: u32 = 0x1002872;
// pub const braille_dots_25678: u32 = 0x10028f2;
// pub const braille_dots_2568: u32 = 0x10028b2;
// pub const braille_dots_257: u32 = 0x1002852;
// pub const braille_dots_2578: u32 = 0x10028d2;
// pub const braille_dots_258: u32 = 0x1002892;
// pub const braille_dots_26: u32 = 0x1002822;
// pub const braille_dots_267: u32 = 0x1002862;
// pub const braille_dots_2678: u32 = 0x10028e2;
// pub const braille_dots_268: u32 = 0x10028a2;
// pub const braille_dots_27: u32 = 0x1002842;
// pub const braille_dots_278: u32 = 0x10028c2;
// pub const braille_dots_28: u32 = 0x1002882;
// pub const braille_dots_3: u32 = 0x1002804;
// pub const braille_dots_34: u32 = 0x100280c;
// pub const braille_dots_345: u32 = 0x100281c;
// pub const braille_dots_3456: u32 = 0x100283c;
// pub const braille_dots_34567: u32 = 0x100287c;
// pub const braille_dots_345678: u32 = 0x10028fc;
// pub const braille_dots_34568: u32 = 0x10028bc;
// pub const braille_dots_3457: u32 = 0x100285c;
// pub const braille_dots_34578: u32 = 0x10028dc;
// pub const braille_dots_3458: u32 = 0x100289c;
// pub const braille_dots_346: u32 = 0x100282c;
// pub const braille_dots_3467: u32 = 0x100286c;
// pub const braille_dots_34678: u32 = 0x10028ec;
// pub const braille_dots_3468: u32 = 0x10028ac;
// pub const braille_dots_347: u32 = 0x100284c;
// pub const braille_dots_3478: u32 = 0x10028cc;
// pub const braille_dots_348: u32 = 0x100288c;
// pub const braille_dots_35: u32 = 0x1002814;
// pub const braille_dots_356: u32 = 0x1002834;
// pub const braille_dots_3567: u32 = 0x1002874;
// pub const braille_dots_35678: u32 = 0x10028f4;
// pub const braille_dots_3568: u32 = 0x10028b4;
// pub const braille_dots_357: u32 = 0x1002854;
// pub const braille_dots_3578: u32 = 0x10028d4;
// pub const braille_dots_358: u32 = 0x1002894;
// pub const braille_dots_36: u32 = 0x1002824;
// pub const braille_dots_367: u32 = 0x1002864;
// pub const braille_dots_3678: u32 = 0x10028e4;
// pub const braille_dots_368: u32 = 0x10028a4;
// pub const braille_dots_37: u32 = 0x1002844;
// pub const braille_dots_378: u32 = 0x10028c4;
// pub const braille_dots_38: u32 = 0x1002884;
// pub const braille_dots_4: u32 = 0x1002808;
// pub const braille_dots_45: u32 = 0x1002818;
// pub const braille_dots_456: u32 = 0x1002838;
// pub const braille_dots_4567: u32 = 0x1002878;
// pub const braille_dots_45678: u32 = 0x10028f8;
// pub const braille_dots_4568: u32 = 0x10028b8;
// pub const braille_dots_457: u32 = 0x1002858;
// pub const braille_dots_4578: u32 = 0x10028d8;
// pub const braille_dots_458: u32 = 0x1002898;
// pub const braille_dots_46: u32 = 0x1002828;
// pub const braille_dots_467: u32 = 0x1002868;
// pub const braille_dots_4678: u32 = 0x10028e8;
// pub const braille_dots_468: u32 = 0x10028a8;
// pub const braille_dots_47: u32 = 0x1002848;
// pub const braille_dots_478: u32 = 0x10028c8;
// pub const braille_dots_48: u32 = 0x1002888;
// pub const braille_dots_5: u32 = 0x1002810;
// pub const braille_dots_56: u32 = 0x1002830;
// pub const braille_dots_567: u32 = 0x1002870;
// pub const braille_dots_5678: u32 = 0x10028f0;
// pub const braille_dots_568: u32 = 0x10028b0;
// pub const braille_dots_57: u32 = 0x1002850;
// pub const braille_dots_578: u32 = 0x10028d0;
// pub const braille_dots_58: u32 = 0x1002890;
// pub const braille_dots_6: u32 = 0x1002820;
// pub const braille_dots_67: u32 = 0x1002860;
// pub const braille_dots_678: u32 = 0x10028e0;
// pub const braille_dots_68: u32 = 0x10028a0;
// pub const braille_dots_7: u32 = 0x1002840;
// pub const braille_dots_78: u32 = 0x10028c0;
// pub const braille_dots_8: u32 = 0x1002880;
pub const Break: u32 = 0xff6b;
// pub const breve: u32 = 0x1a2;
// pub const brokenbar: u32 = 0xa6;
// pub const Byelorussian_shortu: u32 = 0x6ae;
// pub const Byelorussian_SHORTU: u32 = 0x6be;
// pub const c: u32 = 0x63;
// pub const C: u32 = 0x43;
// pub const c_h: u32 = 0xfea3;
// pub const C_h: u32 = 0xfea4;
// pub const C_H: u32 = 0xfea5;
// pub const cabovedot: u32 = 0x2e5;
// pub const Cabovedot: u32 = 0x2c5;
// pub const cacute: u32 = 0x1e6;
// pub const Cacute: u32 = 0x1c6;
pub const Cancel: u32 = 0xff69;
pub const Caps_Lock: u32 = 0xffe5;
// pub const careof: u32 = 0xab8;
// pub const caret: u32 = 0xafc;
// pub const caron: u32 = 0x1b7;
// pub const ccaron: u32 = 0x1e8;
// pub const Ccaron: u32 = 0x1c8;
// pub const ccedilla: u32 = 0xe7;
// pub const Ccedilla: u32 = 0xc7;
// pub const ccircumflex: u32 = 0x2e6;
// pub const Ccircumflex: u32 = 0x2c6;
// pub const cedilla: u32 = 0xb8;
// pub const cent: u32 = 0xa2;
// pub const ch: u32 = 0xfea0;
// pub const Ch: u32 = 0xfea1;
// pub const CH: u32 = 0xfea2;
// pub const checkerboard: u32 = 0x9e1;
// pub const checkmark: u32 = 0xaf3;
// pub const circle: u32 = 0xbcf;
pub const Clear: u32 = 0xff0b;
// pub const ClearLine: u32 = 0x1000ff6f;
// pub const club: u32 = 0xaec;
pub const Codeinput: u32 = 0xff37;
// pub const colon: u32 = 0x3a;
// pub const ColonSign: u32 = 0x10020a1;
// pub const combining_acute: u32 = 0x1000301;
// pub const combining_belowdot: u32 = 0x1000323;
// pub const combining_grave: u32 = 0x1000300;
// pub const combining_hook: u32 = 0x1000309;
// pub const combining_tilde: u32 = 0x1000303;
// pub const comma: u32 = 0x2c;
// pub const containsas: u32 = 0x100220b;
pub const Control_L: u32 = 0xffe3;
pub const Control_R: u32 = 0xffe4;
// pub const copyright: u32 = 0xa9;
// pub const cr: u32 = 0x9e4;
// pub const crossinglines: u32 = 0x9ee;
// pub const CruzeiroSign: u32 = 0x10020a2;
// pub const cuberoot: u32 = 0x100221b;
// pub const currency: u32 = 0xa4;
// pub const cursor: u32 = 0xaff;
// pub const Cyrillic_a: u32 = 0x6c1;
// pub const Cyrillic_A: u32 = 0x6e1;
// pub const Cyrillic_be: u32 = 0x6c2;
// pub const Cyrillic_BE: u32 = 0x6e2;
// pub const Cyrillic_che: u32 = 0x6de;
// pub const Cyrillic_CHE: u32 = 0x6fe;
// pub const Cyrillic_che_descender: u32 = 0x10004b7;
// pub const Cyrillic_CHE_descender: u32 = 0x10004b6;
// pub const Cyrillic_che_vertstroke: u32 = 0x10004b9;
// pub const Cyrillic_CHE_vertstroke: u32 = 0x10004b8;
// pub const Cyrillic_de: u32 = 0x6c4;
// pub const Cyrillic_DE: u32 = 0x6e4;
// pub const Cyrillic_dzhe: u32 = 0x6af;
// pub const Cyrillic_DZHE: u32 = 0x6bf;
// pub const Cyrillic_e: u32 = 0x6dc;
// pub const Cyrillic_E: u32 = 0x6fc;
// pub const Cyrillic_ef: u32 = 0x6c6;
// pub const Cyrillic_EF: u32 = 0x6e6;
// pub const Cyrillic_el: u32 = 0x6cc;
// pub const Cyrillic_EL: u32 = 0x6ec;
// pub const Cyrillic_em: u32 = 0x6cd;
// pub const Cyrillic_EM: u32 = 0x6ed;
// pub const Cyrillic_en: u32 = 0x6ce;
// pub const Cyrillic_EN: u32 = 0x6ee;
// pub const Cyrillic_en_descender: u32 = 0x10004a3;
// pub const Cyrillic_EN_descender: u32 = 0x10004a2;
// pub const Cyrillic_er: u32 = 0x6d2;
// pub const Cyrillic_ER: u32 = 0x6f2;
// pub const Cyrillic_es: u32 = 0x6d3;
// pub const Cyrillic_ES: u32 = 0x6f3;
// pub const Cyrillic_ghe: u32 = 0x6c7;
// pub const Cyrillic_GHE: u32 = 0x6e7;
// pub const Cyrillic_ghe_bar: u32 = 0x1000493;
// pub const Cyrillic_GHE_bar: u32 = 0x1000492;
// pub const Cyrillic_ha: u32 = 0x6c8;
// pub const Cyrillic_HA: u32 = 0x6e8;
// pub const Cyrillic_ha_descender: u32 = 0x10004b3;
// pub const Cyrillic_HA_descender: u32 = 0x10004b2;
// pub const Cyrillic_hardsign: u32 = 0x6df;
// pub const Cyrillic_HARDSIGN: u32 = 0x6ff;
// pub const Cyrillic_i: u32 = 0x6c9;
// pub const Cyrillic_I: u32 = 0x6e9;
// pub const Cyrillic_i_macron: u32 = 0x10004e3;
// pub const Cyrillic_I_macron: u32 = 0x10004e2;
// pub const Cyrillic_ie: u32 = 0x6c5;
// pub const Cyrillic_IE: u32 = 0x6e5;
// pub const Cyrillic_io: u32 = 0x6a3;
// pub const Cyrillic_IO: u32 = 0x6b3;
// pub const Cyrillic_je: u32 = 0x6a8;
// pub const Cyrillic_JE: u32 = 0x6b8;
// pub const Cyrillic_ka: u32 = 0x6cb;
// pub const Cyrillic_KA: u32 = 0x6eb;
// pub const Cyrillic_ka_descender: u32 = 0x100049b;
// pub const Cyrillic_KA_descender: u32 = 0x100049a;
// pub const Cyrillic_ka_vertstroke: u32 = 0x100049d;
// pub const Cyrillic_KA_vertstroke: u32 = 0x100049c;
// pub const Cyrillic_lje: u32 = 0x6a9;
// pub const Cyrillic_LJE: u32 = 0x6b9;
// pub const Cyrillic_nje: u32 = 0x6aa;
// pub const Cyrillic_NJE: u32 = 0x6ba;
// pub const Cyrillic_o: u32 = 0x6cf;
// pub const Cyrillic_O: u32 = 0x6ef;
// pub const Cyrillic_o_bar: u32 = 0x10004e9;
// pub const Cyrillic_O_bar: u32 = 0x10004e8;
// pub const Cyrillic_pe: u32 = 0x6d0;
// pub const Cyrillic_PE: u32 = 0x6f0;
// pub const Cyrillic_schwa: u32 = 0x10004d9;
// pub const Cyrillic_SCHWA: u32 = 0x10004d8;
// pub const Cyrillic_sha: u32 = 0x6db;
// pub const Cyrillic_SHA: u32 = 0x6fb;
// pub const Cyrillic_shcha: u32 = 0x6dd;
// pub const Cyrillic_SHCHA: u32 = 0x6fd;
// pub const Cyrillic_shha: u32 = 0x10004bb;
// pub const Cyrillic_SHHA: u32 = 0x10004ba;
// pub const Cyrillic_shorti: u32 = 0x6ca;
// pub const Cyrillic_SHORTI: u32 = 0x6ea;
// pub const Cyrillic_softsign: u32 = 0x6d8;
// pub const Cyrillic_SOFTSIGN: u32 = 0x6f8;
// pub const Cyrillic_te: u32 = 0x6d4;
// pub const Cyrillic_TE: u32 = 0x6f4;
// pub const Cyrillic_tse: u32 = 0x6c3;
// pub const Cyrillic_TSE: u32 = 0x6e3;
// pub const Cyrillic_u: u32 = 0x6d5;
// pub const Cyrillic_U: u32 = 0x6f5;
// pub const Cyrillic_u_macron: u32 = 0x10004ef;
// pub const Cyrillic_U_macron: u32 = 0x10004ee;
// pub const Cyrillic_u_straight: u32 = 0x10004af;
// pub const Cyrillic_U_straight: u32 = 0x10004ae;
// pub const Cyrillic_u_straight_bar: u32 = 0x10004b1;
// pub const Cyrillic_U_straight_bar: u32 = 0x10004b0;
// pub const Cyrillic_ve: u32 = 0x6d7;
// pub const Cyrillic_VE: u32 = 0x6f7;
// pub const Cyrillic_ya: u32 = 0x6d1;
// pub const Cyrillic_YA: u32 = 0x6f1;
// pub const Cyrillic_yeru: u32 = 0x6d9;
// pub const Cyrillic_YERU: u32 = 0x6f9;
// pub const Cyrillic_yu: u32 = 0x6c0;
// pub const Cyrillic_YU: u32 = 0x6e0;
// pub const Cyrillic_ze: u32 = 0x6da;
// pub const Cyrillic_ZE: u32 = 0x6fa;
// pub const Cyrillic_zhe: u32 = 0x6d6;
// pub const Cyrillic_ZHE: u32 = 0x6f6;
// pub const Cyrillic_zhe_descender: u32 = 0x1000497;
// pub const Cyrillic_ZHE_descender: u32 = 0x1000496;
// pub const d: u32 = 0x64;
pub const D: u32 = 0x44;
// pub const dabovedot: u32 = 0x1001e0b;
// pub const Dabovedot: u32 = 0x1001e0a;
// pub const Dacute_accent: u32 = 0x1000fe27;
// pub const dagger: u32 = 0xaf1;
// pub const dcaron: u32 = 0x1ef;
// pub const Dcaron: u32 = 0x1cf;
// pub const Dcedilla_accent: u32 = 0x1000fe2c;
// pub const Dcircumflex_accent: u32 = 0x1000fe5e;
// pub const Ddiaeresis: u32 = 0x1000fe22;
// pub const dead_a: u32 = 0xfe80;
// pub const dead_A: u32 = 0xfe81;
// pub const dead_abovecomma: u32 = 0xfe64;
// pub const dead_abovedot: u32 = 0xfe56;
// pub const dead_abovereversedcomma: u32 = 0xfe65;
// pub const dead_abovering: u32 = 0xfe58;
// pub const dead_aboveverticalline: u32 = 0xfe91;
// pub const dead_acute: u32 = 0xfe51;
// pub const dead_apostrophe: u32 = 0xfe8e;
// pub const dead_belowbreve: u32 = 0xfe6b;
// pub const dead_belowcircumflex: u32 = 0xfe69;
// pub const dead_belowcomma: u32 = 0xfe6e;
// pub const dead_belowdiaeresis: u32 = 0xfe6c;
// pub const dead_belowdot: u32 = 0xfe60;
// pub const dead_belowmacron: u32 = 0xfe68;
// pub const dead_belowring: u32 = 0xfe67;
// pub const dead_belowtilde: u32 = 0xfe6a;
// pub const dead_belowverticalline: u32 = 0xfe92;
// pub const dead_breve: u32 = 0xfe55;
// pub const dead_capital_schwa: u32 = 0xfe8b;
// pub const dead_caron: u32 = 0xfe5a;
// pub const dead_cedilla: u32 = 0xfe5b;
// pub const dead_circumflex: u32 = 0xfe52;
// pub const dead_currency: u32 = 0xfe6f;
// pub const dead_dasia: u32 = 0xfe65;
// pub const dead_diaeresis: u32 = 0xfe57;
// pub const dead_doubleacute: u32 = 0xfe59;
// pub const dead_doublegrave: u32 = 0xfe66;
// pub const dead_e: u32 = 0xfe82;
// pub const dead_E: u32 = 0xfe83;
// pub const dead_grave: u32 = 0xfe50;
// pub const dead_greek: u32 = 0xfe8c;
// pub const dead_hamza: u32 = 0xfe8d;
// pub const dead_hook: u32 = 0xfe61;
// pub const dead_horn: u32 = 0xfe62;
// pub const dead_i: u32 = 0xfe84;
// pub const dead_I: u32 = 0xfe85;
// pub const dead_invertedbreve: u32 = 0xfe6d;
// pub const dead_iota: u32 = 0xfe5d;
// pub const dead_longsolidusoverlay: u32 = 0xfe93;
// pub const dead_lowline: u32 = 0xfe90;
// pub const dead_macron: u32 = 0xfe54;
// pub const dead_o: u32 = 0xfe86;
// pub const dead_O: u32 = 0xfe87;
// pub const dead_ogonek: u32 = 0xfe5c;
// pub const dead_perispomeni: u32 = 0xfe53;
// pub const dead_psili: u32 = 0xfe64;
// pub const dead_schwa: u32 = 0xfe8a;
// pub const dead_SCHWA: u32 = 0xfe8b;
// pub const dead_semivoiced_sound: u32 = 0xfe5f;
// pub const dead_small_schwa: u32 = 0xfe8a;
// pub const dead_stroke: u32 = 0xfe63;
// pub const dead_tilde: u32 = 0xfe53;
// pub const dead_u: u32 = 0xfe88;
// pub const dead_U: u32 = 0xfe89;
// pub const dead_voiced_sound: u32 = 0xfe5e;
// pub const decimalpoint: u32 = 0xabd;
// pub const degree: u32 = 0xb0;
pub const Delete: u32 = 0xffff;
// pub const DeleteChar: u32 = 0x1000ff73;
// pub const DeleteLine: u32 = 0x1000ff71;
// pub const Dgrave_accent: u32 = 0x1000fe60;
// pub const diaeresis: u32 = 0xa8;
// pub const diamond: u32 = 0xaed;
// pub const digitspace: u32 = 0xaa5;
// pub const dintegral: u32 = 0x100222c;
// pub const division: u32 = 0xf7;
// pub const dollar: u32 = 0x24;
// pub const DongSign: u32 = 0x10020ab;
// pub const doubbaselinedot: u32 = 0xaaf;
// pub const doubleacute: u32 = 0x1bd;
// pub const doubledagger: u32 = 0xaf2;
// pub const doublelowquotemark: u32 = 0xafe;
pub const Down: u32 = 0xff54;
// pub const downarrow: u32 = 0x8fe;
// pub const downcaret: u32 = 0xba8;
// pub const downshoe: u32 = 0xbd6;
// pub const downstile: u32 = 0xbc4;
// pub const downtack: u32 = 0xbc2;
// pub const DRemove: u32 = 0x1000ff00;
// pub const Dring_accent: u32 = 0x1000feb0;
// pub const dstroke: u32 = 0x1f0;
// pub const Dstroke: u32 = 0x1d0;
// pub const Dtilde: u32 = 0x1000fe7e;
// pub const e: u32 = 0x65;
pub const E: u32 = 0x45;
// pub const eabovedot: u32 = 0x3ec;
// pub const Eabovedot: u32 = 0x3cc;
// pub const eacute: u32 = 0xe9;
// pub const Eacute: u32 = 0xc9;
// pub const ebelowdot: u32 = 0x1001eb9;
// pub const Ebelowdot: u32 = 0x1001eb8;
// pub const ecaron: u32 = 0x1ec;
// pub const Ecaron: u32 = 0x1cc;
// pub const ecircumflex: u32 = 0xea;
// pub const Ecircumflex: u32 = 0xca;
// pub const ecircumflexacute: u32 = 0x1001ebf;
// pub const Ecircumflexacute: u32 = 0x1001ebe;
// pub const ecircumflexbelowdot: u32 = 0x1001ec7;
// pub const Ecircumflexbelowdot: u32 = 0x1001ec6;
// pub const ecircumflexgrave: u32 = 0x1001ec1;
// pub const Ecircumflexgrave: u32 = 0x1001ec0;
// pub const ecircumflexhook: u32 = 0x1001ec3;
// pub const Ecircumflexhook: u32 = 0x1001ec2;
// pub const ecircumflextilde: u32 = 0x1001ec5;
// pub const Ecircumflextilde: u32 = 0x1001ec4;
// pub const EcuSign: u32 = 0x10020a0;
// pub const ediaeresis: u32 = 0xeb;
// pub const Ediaeresis: u32 = 0xcb;
// pub const egrave: u32 = 0xe8;
// pub const Egrave: u32 = 0xc8;
// pub const ehook: u32 = 0x1001ebb;
// pub const Ehook: u32 = 0x1001eba;
// pub const eightsubscript: u32 = 0x1002088;
// pub const eightsuperior: u32 = 0x1002078;
pub const Eisu_Shift: u32 = 0xff2f;
pub const Eisu_toggle: u32 = 0xff30;
// pub const elementof: u32 = 0x1002208;
// pub const ellipsis: u32 = 0xaae;
// pub const em3space: u32 = 0xaa3;
// pub const em4space: u32 = 0xaa4;
// pub const emacron: u32 = 0x3ba;
// pub const Emacron: u32 = 0x3aa;
// pub const emdash: u32 = 0xaa9;
// pub const emfilledcircle: u32 = 0xade;
// pub const emfilledrect: u32 = 0xadf;
// pub const emopencircle: u32 = 0xace;
// pub const emopenrectangle: u32 = 0xacf;
// pub const emptyset: u32 = 0x1002205;
// pub const emspace: u32 = 0xaa1;
pub const End: u32 = 0xff57;
// pub const endash: u32 = 0xaaa;
// pub const enfilledcircbullet: u32 = 0xae6;
// pub const enfilledsqbullet: u32 = 0xae7;
// pub const eng: u32 = 0x3bf;
// pub const ENG: u32 = 0x3bd;
// pub const enopencircbullet: u32 = 0xae0;
// pub const enopensquarebullet: u32 = 0xae1;
// pub const enspace: u32 = 0xaa2;
// pub const eogonek: u32 = 0x1ea;
// pub const Eogonek: u32 = 0x1ca;
// pub const equal: u32 = 0x3d;
pub const Escape: u32 = 0xff1b;
// pub const eth: u32 = 0xf0;
// pub const Eth: u32 = 0xd0;
// pub const ETH: u32 = 0xd0;
// pub const etilde: u32 = 0x1001ebd;
// pub const Etilde: u32 = 0x1001ebc;
// pub const EuroSign: u32 = 0x20ac;
// pub const exclam: u32 = 0x21;
// pub const exclamdown: u32 = 0xa1;
pub const Execute: u32 = 0xff62;
// pub const Ext16bit_L: u32 = 0x1000ff76;
// pub const Ext16bit_R: u32 = 0x1000ff77;
// pub const ezh: u32 = 0x1000292;
// pub const EZH: u32 = 0x10001b7;
// pub const f: u32 = 0x66;
// pub const F: u32 = 0x46;
pub const F1: u32 = 0xffbe;
pub const F10: u32 = 0xffc7;
pub const F11: u32 = 0xffc8;
pub const F12: u32 = 0xffc9;
pub const F13: u32 = 0xffca;
pub const F14: u32 = 0xffcb;
pub const F15: u32 = 0xffcc;
pub const F16: u32 = 0xffcd;
pub const F17: u32 = 0xffce;
pub const F18: u32 = 0xffcf;
pub const F19: u32 = 0xffd0;
pub const F2: u32 = 0xffbf;
pub const F20: u32 = 0xffd1;
pub const F21: u32 = 0xffd2;
pub const F22: u32 = 0xffd3;
pub const F23: u32 = 0xffd4;
pub const F24: u32 = 0xffd5;
pub const F25: u32 = 0xffd6;
pub const F26: u32 = 0xffd7;
pub const F27: u32 = 0xffd8;
pub const F28: u32 = 0xffd9;
pub const F29: u32 = 0xffda;
pub const F3: u32 = 0xffc0;
pub const F30: u32 = 0xffdb;
pub const F31: u32 = 0xffdc;
pub const F32: u32 = 0xffdd;
pub const F33: u32 = 0xffde;
pub const F34: u32 = 0xffdf;
pub const F35: u32 = 0xffe0;
pub const F4: u32 = 0xffc1;
pub const F5: u32 = 0xffc2;
pub const F6: u32 = 0xffc3;
pub const F7: u32 = 0xffc4;
pub const F8: u32 = 0xffc5;
pub const F9: u32 = 0xffc6;
// pub const fabovedot: u32 = 0x1001e1f;
// pub const Fabovedot: u32 = 0x1001e1e;
// pub const Farsi_0: u32 = 0x10006f0;
// pub const Farsi_1: u32 = 0x10006f1;
// pub const Farsi_2: u32 = 0x10006f2;
// pub const Farsi_3: u32 = 0x10006f3;
// pub const Farsi_4: u32 = 0x10006f4;
// pub const Farsi_5: u32 = 0x10006f5;
// pub const Farsi_6: u32 = 0x10006f6;
// pub const Farsi_7: u32 = 0x10006f7;
// pub const Farsi_8: u32 = 0x10006f8;
// pub const Farsi_9: u32 = 0x10006f9;
// pub const Farsi_yeh: u32 = 0x10006cc;
// pub const femalesymbol: u32 = 0xaf8;
// pub const ff: u32 = 0x9e3;
// pub const FFrancSign: u32 = 0x10020a3;
// pub const figdash: u32 = 0xabb;
// pub const filledlefttribullet: u32 = 0xadc;
// pub const filledrectbullet: u32 = 0xadb;
// pub const filledrighttribullet: u32 = 0xadd;
// pub const filledtribulletdown: u32 = 0xae9;
// pub const filledtribulletup: u32 = 0xae8;
pub const Find: u32 = 0xff68;
// pub const First_Virtual_Screen: u32 = 0xfed0;
// pub const fiveeighths: u32 = 0xac5;
// pub const fivesixths: u32 = 0xab7;
// pub const fivesubscript: u32 = 0x1002085;
// pub const fivesuperior: u32 = 0x1002075;
// pub const fourfifths: u32 = 0xab5;
// pub const foursubscript: u32 = 0x1002084;
// pub const foursuperior: u32 = 0x1002074;
// pub const fourthroot: u32 = 0x100221c;
// pub const function: u32 = 0x8f6;
// pub const g: u32 = 0x67;
// pub const G: u32 = 0x47;
// pub const gabovedot: u32 = 0x2f5;
// pub const Gabovedot: u32 = 0x2d5;
// pub const gbreve: u32 = 0x2bb;
// pub const Gbreve: u32 = 0x2ab;
// pub const gcaron: u32 = 0x10001e7;
// pub const Gcaron: u32 = 0x10001e6;
// pub const gcedilla: u32 = 0x3bb;
// pub const Gcedilla: u32 = 0x3ab;
// pub const gcircumflex: u32 = 0x2f8;
// pub const Gcircumflex: u32 = 0x2d8;
// pub const Georgian_an: u32 = 0x10010d0;
// pub const Georgian_ban: u32 = 0x10010d1;
// pub const Georgian_can: u32 = 0x10010ea;
// pub const Georgian_char: u32 = 0x10010ed;
// pub const Georgian_chin: u32 = 0x10010e9;
// pub const Georgian_cil: u32 = 0x10010ec;
// pub const Georgian_don: u32 = 0x10010d3;
// pub const Georgian_en: u32 = 0x10010d4;
// pub const Georgian_fi: u32 = 0x10010f6;
// pub const Georgian_gan: u32 = 0x10010d2;
// pub const Georgian_ghan: u32 = 0x10010e6;
// pub const Georgian_hae: u32 = 0x10010f0;
// pub const Georgian_har: u32 = 0x10010f4;
// pub const Georgian_he: u32 = 0x10010f1;
// pub const Georgian_hie: u32 = 0x10010f2;
// pub const Georgian_hoe: u32 = 0x10010f5;
// pub const Georgian_in: u32 = 0x10010d8;
// pub const Georgian_jhan: u32 = 0x10010ef;
// pub const Georgian_jil: u32 = 0x10010eb;
// pub const Georgian_kan: u32 = 0x10010d9;
// pub const Georgian_khar: u32 = 0x10010e5;
// pub const Georgian_las: u32 = 0x10010da;
// pub const Georgian_man: u32 = 0x10010db;
// pub const Georgian_nar: u32 = 0x10010dc;
// pub const Georgian_on: u32 = 0x10010dd;
// pub const Georgian_par: u32 = 0x10010de;
// pub const Georgian_phar: u32 = 0x10010e4;
// pub const Georgian_qar: u32 = 0x10010e7;
// pub const Georgian_rae: u32 = 0x10010e0;
// pub const Georgian_san: u32 = 0x10010e1;
// pub const Georgian_shin: u32 = 0x10010e8;
// pub const Georgian_tan: u32 = 0x10010d7;
// pub const Georgian_tar: u32 = 0x10010e2;
// pub const Georgian_un: u32 = 0x10010e3;
// pub const Georgian_vin: u32 = 0x10010d5;
// pub const Georgian_we: u32 = 0x10010f3;
// pub const Georgian_xan: u32 = 0x10010ee;
// pub const Georgian_zen: u32 = 0x10010d6;
// pub const Georgian_zhar: u32 = 0x10010df;
// pub const grave: u32 = 0x60;
// pub const greater: u32 = 0x3e;
// pub const greaterthanequal: u32 = 0x8be;
// pub const Greek_accentdieresis: u32 = 0x7ae;
// pub const Greek_alpha: u32 = 0x7e1;
// pub const Greek_ALPHA: u32 = 0x7c1;
// pub const Greek_alphaaccent: u32 = 0x7b1;
// pub const Greek_ALPHAaccent: u32 = 0x7a1;
// pub const Greek_beta: u32 = 0x7e2;
// pub const Greek_BETA: u32 = 0x7c2;
// pub const Greek_chi: u32 = 0x7f7;
// pub const Greek_CHI: u32 = 0x7d7;
// pub const Greek_delta: u32 = 0x7e4;
// pub const Greek_DELTA: u32 = 0x7c4;
// pub const Greek_epsilon: u32 = 0x7e5;
// pub const Greek_EPSILON: u32 = 0x7c5;
// pub const Greek_epsilonaccent: u32 = 0x7b2;
// pub const Greek_EPSILONaccent: u32 = 0x7a2;
// pub const Greek_eta: u32 = 0x7e7;
// pub const Greek_ETA: u32 = 0x7c7;
// pub const Greek_etaaccent: u32 = 0x7b3;
// pub const Greek_ETAaccent: u32 = 0x7a3;
// pub const Greek_finalsmallsigma: u32 = 0x7f3;
// pub const Greek_gamma: u32 = 0x7e3;
// pub const Greek_GAMMA: u32 = 0x7c3;
// pub const Greek_horizbar: u32 = 0x7af;
// pub const Greek_iota: u32 = 0x7e9;
// pub const Greek_IOTA: u32 = 0x7c9;
// pub const Greek_iotaaccent: u32 = 0x7b4;
// pub const Greek_IOTAaccent: u32 = 0x7a4;
// pub const Greek_iotaaccentdieresis: u32 = 0x7b6;
// pub const Greek_IOTAdiaeresis: u32 = 0x7a5;
// pub const Greek_iotadieresis: u32 = 0x7b5;
// pub const Greek_IOTAdieresis: u32 = 0x7a5;
// pub const Greek_kappa: u32 = 0x7ea;
// pub const Greek_KAPPA: u32 = 0x7ca;
// pub const Greek_lambda: u32 = 0x7eb;
// pub const Greek_LAMBDA: u32 = 0x7cb;
// pub const Greek_lamda: u32 = 0x7eb;
// pub const Greek_LAMDA: u32 = 0x7cb;
// pub const Greek_mu: u32 = 0x7ec;
// pub const Greek_MU: u32 = 0x7cc;
// pub const Greek_nu: u32 = 0x7ed;
// pub const Greek_NU: u32 = 0x7cd;
// pub const Greek_omega: u32 = 0x7f9;
// pub const Greek_OMEGA: u32 = 0x7d9;
// pub const Greek_omegaaccent: u32 = 0x7bb;
// pub const Greek_OMEGAaccent: u32 = 0x7ab;
// pub const Greek_omicron: u32 = 0x7ef;
// pub const Greek_OMICRON: u32 = 0x7cf;
// pub const Greek_omicronaccent: u32 = 0x7b7;
// pub const Greek_OMICRONaccent: u32 = 0x7a7;
// pub const Greek_phi: u32 = 0x7f6;
// pub const Greek_PHI: u32 = 0x7d6;
// pub const Greek_pi: u32 = 0x7f0;
// pub const Greek_PI: u32 = 0x7d0;
// pub const Greek_psi: u32 = 0x7f8;
// pub const Greek_PSI: u32 = 0x7d8;
// pub const Greek_rho: u32 = 0x7f1;
// pub const Greek_RHO: u32 = 0x7d1;
// pub const Greek_sigma: u32 = 0x7f2;
// pub const Greek_SIGMA: u32 = 0x7d2;
// pub const Greek_switch: u32 = 0xff7e;
// pub const Greek_tau: u32 = 0x7f4;
// pub const Greek_TAU: u32 = 0x7d4;
// pub const Greek_theta: u32 = 0x7e8;
// pub const Greek_THETA: u32 = 0x7c8;
// pub const Greek_upsilon: u32 = 0x7f5;
// pub const Greek_UPSILON: u32 = 0x7d5;
// pub const Greek_upsilonaccent: u32 = 0x7b8;
// pub const Greek_UPSILONaccent: u32 = 0x7a8;
// pub const Greek_upsilonaccentdieresis: u32 = 0x7ba;
// pub const Greek_upsilondieresis: u32 = 0x7b9;
// pub const Greek_UPSILONdieresis: u32 = 0x7a9;
// pub const Greek_xi: u32 = 0x7ee;
// pub const Greek_XI: u32 = 0x7ce;
// pub const Greek_zeta: u32 = 0x7e6;
// pub const Greek_ZETA: u32 = 0x7c6;
// pub const guilder: u32 = 0x100000be;
// pub const guillemetleft: u32 = 0xab;
// pub const guillemetright: u32 = 0xbb;
// pub const guillemotleft: u32 = 0xab;
// pub const guillemotright: u32 = 0xbb;
// pub const h: u32 = 0x68;
// pub const H: u32 = 0x48;
// pub const hairspace: u32 = 0xaa8;
// pub const Hangul: u32 = 0xff31;
// pub const Hangul_A: u32 = 0xebf;
// pub const Hangul_AE: u32 = 0xec0;
// pub const Hangul_AraeA: u32 = 0xef6;
// pub const Hangul_AraeAE: u32 = 0xef7;
// pub const Hangul_Banja: u32 = 0xff39;
// pub const Hangul_Cieuc: u32 = 0xeba;
// pub const Hangul_Codeinput: u32 = 0xff37;
// pub const Hangul_Dikeud: u32 = 0xea7;
// pub const Hangul_E: u32 = 0xec4;
// pub const Hangul_End: u32 = 0xff33;
// pub const Hangul_EO: u32 = 0xec3;
// pub const Hangul_EU: u32 = 0xed1;
// pub const Hangul_Hanja: u32 = 0xff34;
// pub const Hangul_Hieuh: u32 = 0xebe;
// pub const Hangul_I: u32 = 0xed3;
// pub const Hangul_Ieung: u32 = 0xeb7;
// pub const Hangul_J_Cieuc: u32 = 0xeea;
// pub const Hangul_J_Dikeud: u32 = 0xeda;
// pub const Hangul_J_Hieuh: u32 = 0xeee;
// pub const Hangul_J_Ieung: u32 = 0xee8;
// pub const Hangul_J_Jieuj: u32 = 0xee9;
// pub const Hangul_J_Khieuq: u32 = 0xeeb;
// pub const Hangul_J_Kiyeog: u32 = 0xed4;
// pub const Hangul_J_KiyeogSios: u32 = 0xed6;
// pub const Hangul_J_KkogjiDalrinIeung: u32 = 0xef9;
// pub const Hangul_J_Mieum: u32 = 0xee3;
// pub const Hangul_J_Nieun: u32 = 0xed7;
// pub const Hangul_J_NieunHieuh: u32 = 0xed9;
// pub const Hangul_J_NieunJieuj: u32 = 0xed8;
// pub const Hangul_J_PanSios: u32 = 0xef8;
// pub const Hangul_J_Phieuf: u32 = 0xeed;
// pub const Hangul_J_Pieub: u32 = 0xee4;
// pub const Hangul_J_PieubSios: u32 = 0xee5;
// pub const Hangul_J_Rieul: u32 = 0xedb;
// pub const Hangul_J_RieulHieuh: u32 = 0xee2;
// pub const Hangul_J_RieulKiyeog: u32 = 0xedc;
// pub const Hangul_J_RieulMieum: u32 = 0xedd;
// pub const Hangul_J_RieulPhieuf: u32 = 0xee1;
// pub const Hangul_J_RieulPieub: u32 = 0xede;
// pub const Hangul_J_RieulSios: u32 = 0xedf;
// pub const Hangul_J_RieulTieut: u32 = 0xee0;
// pub const Hangul_J_Sios: u32 = 0xee6;
// pub const Hangul_J_SsangKiyeog: u32 = 0xed5;
// pub const Hangul_J_SsangSios: u32 = 0xee7;
// pub const Hangul_J_Tieut: u32 = 0xeec;
// pub const Hangul_J_YeorinHieuh: u32 = 0xefa;
// pub const Hangul_Jamo: u32 = 0xff35;
// pub const Hangul_Jeonja: u32 = 0xff38;
// pub const Hangul_Jieuj: u32 = 0xeb8;
// pub const Hangul_Khieuq: u32 = 0xebb;
// pub const Hangul_Kiyeog: u32 = 0xea1;
// pub const Hangul_KiyeogSios: u32 = 0xea3;
// pub const Hangul_KkogjiDalrinIeung: u32 = 0xef3;
// pub const Hangul_Mieum: u32 = 0xeb1;
// pub const Hangul_MultipleCandidate: u32 = 0xff3d;
// pub const Hangul_Nieun: u32 = 0xea4;
// pub const Hangul_NieunHieuh: u32 = 0xea6;
// pub const Hangul_NieunJieuj: u32 = 0xea5;
// pub const Hangul_O: u32 = 0xec7;
// pub const Hangul_OE: u32 = 0xeca;
// pub const Hangul_PanSios: u32 = 0xef2;
// pub const Hangul_Phieuf: u32 = 0xebd;
// pub const Hangul_Pieub: u32 = 0xeb2;
// pub const Hangul_PieubSios: u32 = 0xeb4;
// pub const Hangul_PostHanja: u32 = 0xff3b;
// pub const Hangul_PreHanja: u32 = 0xff3a;
// pub const Hangul_PreviousCandidate: u32 = 0xff3e;
// pub const Hangul_Rieul: u32 = 0xea9;
// pub const Hangul_RieulHieuh: u32 = 0xeb0;
// pub const Hangul_RieulKiyeog: u32 = 0xeaa;
// pub const Hangul_RieulMieum: u32 = 0xeab;
// pub const Hangul_RieulPhieuf: u32 = 0xeaf;
// pub const Hangul_RieulPieub: u32 = 0xeac;
// pub const Hangul_RieulSios: u32 = 0xead;
// pub const Hangul_RieulTieut: u32 = 0xeae;
// pub const Hangul_RieulYeorinHieuh: u32 = 0xeef;
// pub const Hangul_Romaja: u32 = 0xff36;
// pub const Hangul_SingleCandidate: u32 = 0xff3c;
// pub const Hangul_Sios: u32 = 0xeb5;
// pub const Hangul_Special: u32 = 0xff3f;
// pub const Hangul_SsangDikeud: u32 = 0xea8;
// pub const Hangul_SsangJieuj: u32 = 0xeb9;
// pub const Hangul_SsangKiyeog: u32 = 0xea2;
// pub const Hangul_SsangPieub: u32 = 0xeb3;
// pub const Hangul_SsangSios: u32 = 0xeb6;
// pub const Hangul_Start: u32 = 0xff32;
// pub const Hangul_SunkyeongeumMieum: u32 = 0xef0;
// pub const Hangul_SunkyeongeumPhieuf: u32 = 0xef4;
// pub const Hangul_SunkyeongeumPieub: u32 = 0xef1;
// pub const Hangul_switch: u32 = 0xff7e;
// pub const Hangul_Tieut: u32 = 0xebc;
// pub const Hangul_U: u32 = 0xecc;
// pub const Hangul_WA: u32 = 0xec8;
// pub const Hangul_WAE: u32 = 0xec9;
// pub const Hangul_WE: u32 = 0xece;
// pub const Hangul_WEO: u32 = 0xecd;
// pub const Hangul_WI: u32 = 0xecf;
// pub const Hangul_YA: u32 = 0xec1;
// pub const Hangul_YAE: u32 = 0xec2;
// pub const Hangul_YE: u32 = 0xec6;
// pub const Hangul_YEO: u32 = 0xec5;
// pub const Hangul_YeorinHieuh: u32 = 0xef5;
// pub const Hangul_YI: u32 = 0xed2;
// pub const Hangul_YO: u32 = 0xecb;
// pub const Hangul_YU: u32 = 0xed0;
pub const Hankaku: u32 = 0xff29;
// pub const hcircumflex: u32 = 0x2b6;
// pub const Hcircumflex: u32 = 0x2a6;
// pub const heart: u32 = 0xaee;
// pub const hebrew_aleph: u32 = 0xce0;
// pub const hebrew_ayin: u32 = 0xcf2;
// pub const hebrew_bet: u32 = 0xce1;
// pub const hebrew_beth: u32 = 0xce1;
// pub const hebrew_chet: u32 = 0xce7;
// pub const hebrew_dalet: u32 = 0xce3;
// pub const hebrew_daleth: u32 = 0xce3;
// pub const hebrew_doublelowline: u32 = 0xcdf;
// pub const hebrew_finalkaph: u32 = 0xcea;
// pub const hebrew_finalmem: u32 = 0xced;
// pub const hebrew_finalnun: u32 = 0xcef;
// pub const hebrew_finalpe: u32 = 0xcf3;
// pub const hebrew_finalzade: u32 = 0xcf5;
// pub const hebrew_finalzadi: u32 = 0xcf5;
// pub const hebrew_gimel: u32 = 0xce2;
// pub const hebrew_gimmel: u32 = 0xce2;
// pub const hebrew_he: u32 = 0xce4;
// pub const hebrew_het: u32 = 0xce7;
// pub const hebrew_kaph: u32 = 0xceb;
// pub const hebrew_kuf: u32 = 0xcf7;
// pub const hebrew_lamed: u32 = 0xcec;
// pub const hebrew_mem: u32 = 0xcee;
// pub const hebrew_nun: u32 = 0xcf0;
// pub const hebrew_pe: u32 = 0xcf4;
// pub const hebrew_qoph: u32 = 0xcf7;
// pub const hebrew_resh: u32 = 0xcf8;
// pub const hebrew_samech: u32 = 0xcf1;
// pub const hebrew_samekh: u32 = 0xcf1;
// pub const hebrew_shin: u32 = 0xcf9;
// pub const Hebrew_switch: u32 = 0xff7e;
// pub const hebrew_taf: u32 = 0xcfa;
// pub const hebrew_taw: u32 = 0xcfa;
// pub const hebrew_tet: u32 = 0xce8;
// pub const hebrew_teth: u32 = 0xce8;
// pub const hebrew_waw: u32 = 0xce5;
// pub const hebrew_yod: u32 = 0xce9;
// pub const hebrew_zade: u32 = 0xcf6;
// pub const hebrew_zadi: u32 = 0xcf6;
// pub const hebrew_zain: u32 = 0xce6;
// pub const hebrew_zayin: u32 = 0xce6;
pub const Help: u32 = 0xff6a;
// pub const Henkan: u32 = 0xff23;
pub const Henkan_Mode: u32 = 0xff23;
// pub const hexagram: u32 = 0xada;
pub const Hiragana: u32 = 0xff25;
pub const Hiragana_Katakana: u32 = 0xff27;
pub const Home: u32 = 0xff50;
// pub const horizconnector: u32 = 0x8a3;
// pub const horizlinescan1: u32 = 0x9ef;
// pub const horizlinescan3: u32 = 0x9f0;
// pub const horizlinescan5: u32 = 0x9f1;
// pub const horizlinescan7: u32 = 0x9f2;
// pub const horizlinescan9: u32 = 0x9f3;
// pub const hpBackTab: u32 = 0x1000ff74;
// pub const hpblock: u32 = 0x100000fc;
// pub const hpClearLine: u32 = 0x1000ff6f;
// pub const hpDeleteChar: u32 = 0x1000ff73;
// pub const hpDeleteLine: u32 = 0x1000ff71;
// pub const hpguilder: u32 = 0x100000be;
// pub const hpInsertChar: u32 = 0x1000ff72;
// pub const hpInsertLine: u32 = 0x1000ff70;
// pub const hpIO: u32 = 0x100000ee;
// pub const hpKP_BackTab: u32 = 0x1000ff75;
// pub const hplira: u32 = 0x100000af;
// pub const hplongminus: u32 = 0x100000f6;
// pub const hpModelock1: u32 = 0x1000ff48;
// pub const hpModelock2: u32 = 0x1000ff49;
// pub const hpmute_acute: u32 = 0x100000a8;
// pub const hpmute_asciicircum: u32 = 0x100000aa;
// pub const hpmute_asciitilde: u32 = 0x100000ac;
// pub const hpmute_diaeresis: u32 = 0x100000ab;
// pub const hpmute_grave: u32 = 0x100000a9;
// pub const hpReset: u32 = 0x1000ff6c;
// pub const hpSystem: u32 = 0x1000ff6d;
// pub const hpUser: u32 = 0x1000ff6e;
// pub const hpYdiaeresis: u32 = 0x100000ee;
// pub const hstroke: u32 = 0x2b1;
// pub const Hstroke: u32 = 0x2a1;
// pub const ht: u32 = 0x9e2;
pub const Hyper_L: u32 = 0xffed;
pub const Hyper_R: u32 = 0xffee;
// pub const hyphen: u32 = 0xad;
// pub const i: u32 = 0x69;
// pub const I: u32 = 0x49;
// pub const Iabovedot: u32 = 0x2a9;
// pub const iacute: u32 = 0xed;
// pub const Iacute: u32 = 0xcd;
// pub const ibelowdot: u32 = 0x1001ecb;
// pub const Ibelowdot: u32 = 0x1001eca;
// pub const ibreve: u32 = 0x100012d;
// pub const Ibreve: u32 = 0x100012c;
// pub const icircumflex: u32 = 0xee;
// pub const Icircumflex: u32 = 0xce;
// pub const identical: u32 = 0x8cf;
// pub const idiaeresis: u32 = 0xef;
// pub const Idiaeresis: u32 = 0xcf;
// pub const idotless: u32 = 0x2b9;
// pub const ifonlyif: u32 = 0x8cd;
// pub const igrave: u32 = 0xec;
// pub const Igrave: u32 = 0xcc;
// pub const ihook: u32 = 0x1001ec9;
// pub const Ihook: u32 = 0x1001ec8;
// pub const imacron: u32 = 0x3ef;
// pub const Imacron: u32 = 0x3cf;
// pub const implies: u32 = 0x8ce;
// pub const includedin: u32 = 0x8da;
// pub const includes: u32 = 0x8db;
// pub const infinity: u32 = 0x8c2;
pub const Insert: u32 = 0xff63;
// pub const InsertChar: u32 = 0x1000ff72;
// pub const InsertLine: u32 = 0x1000ff70;
// pub const integral: u32 = 0x8bf;
// pub const intersection: u32 = 0x8dc;
// pub const IO: u32 = 0x100000ee;
// pub const iogonek: u32 = 0x3e7;
// pub const Iogonek: u32 = 0x3c7;
pub const ISO_Center_Object: u32 = 0xfe33;
pub const ISO_Continuous_Underline: u32 = 0xfe30;
pub const ISO_Discontinuous_Underline: u32 = 0xfe31;
pub const ISO_Emphasize: u32 = 0xfe32;
pub const ISO_Enter: u32 = 0xfe34;
pub const ISO_Fast_Cursor_Down: u32 = 0xfe2f;
pub const ISO_Fast_Cursor_Left: u32 = 0xfe2c;
pub const ISO_Fast_Cursor_Right: u32 = 0xfe2d;
pub const ISO_Fast_Cursor_Up: u32 = 0xfe2e;
pub const ISO_First_Group: u32 = 0xfe0c;
pub const ISO_First_Group_Lock: u32 = 0xfe0d;
pub const ISO_Group_Latch: u32 = 0xfe06;
pub const ISO_Group_Lock: u32 = 0xfe07;
pub const ISO_Group_Shift: u32 = 0xff7e;
pub const ISO_Last_Group: u32 = 0xfe0e;
pub const ISO_Last_Group_Lock: u32 = 0xfe0f;
pub const ISO_Left_Tab: u32 = 0xfe20;
pub const ISO_Level2_Latch: u32 = 0xfe02;
pub const ISO_Level3_Latch: u32 = 0xfe04;
pub const ISO_Level3_Lock: u32 = 0xfe05;
pub const ISO_Level3_Shift: u32 = 0xfe03;
pub const ISO_Level5_Latch: u32 = 0xfe12;
pub const ISO_Level5_Lock: u32 = 0xfe13;
pub const ISO_Level5_Shift: u32 = 0xfe11;
pub const ISO_Lock: u32 = 0xfe01;
pub const ISO_Move_Line_Down: u32 = 0xfe22;
pub const ISO_Move_Line_Up: u32 = 0xfe21;
pub const ISO_Next_Group: u32 = 0xfe08;
pub const ISO_Next_Group_Lock: u32 = 0xfe09;
pub const ISO_Partial_Line_Down: u32 = 0xfe24;
pub const ISO_Partial_Line_Up: u32 = 0xfe23;
pub const ISO_Partial_Space_Left: u32 = 0xfe25;
pub const ISO_Partial_Space_Right: u32 = 0xfe26;
pub const ISO_Prev_Group: u32 = 0xfe0a;
pub const ISO_Prev_Group_Lock: u32 = 0xfe0b;
pub const ISO_Release_Both_Margins: u32 = 0xfe2b;
pub const ISO_Release_Margin_Left: u32 = 0xfe29;
pub const ISO_Release_Margin_Right: u32 = 0xfe2a;
pub const ISO_Set_Margin_Left: u32 = 0xfe27;
pub const ISO_Set_Margin_Right: u32 = 0xfe28;
// pub const itilde: u32 = 0x3b5;
// pub const Itilde: u32 = 0x3a5;
// pub const j: u32 = 0x6a;
// pub const J: u32 = 0x4a;
// pub const jcircumflex: u32 = 0x2bc;
// pub const Jcircumflex: u32 = 0x2ac;
// pub const jot: u32 = 0xbca;
// pub const k: u32 = 0x6b;
// pub const K: u32 = 0x4b;
// pub const kana_a: u32 = 0x4a7;
// pub const kana_A: u32 = 0x4b1;
// pub const kana_CHI: u32 = 0x4c1;
// pub const kana_closingbracket: u32 = 0x4a3;
// pub const kana_comma: u32 = 0x4a4;
// pub const kana_conjunctive: u32 = 0x4a5;
// pub const kana_e: u32 = 0x4aa;
// pub const kana_E: u32 = 0x4b4;
// pub const kana_FU: u32 = 0x4cc;
// pub const kana_fullstop: u32 = 0x4a1;
// pub const kana_HA: u32 = 0x4ca;
// pub const kana_HE: u32 = 0x4cd;
// pub const kana_HI: u32 = 0x4cb;
// pub const kana_HO: u32 = 0x4ce;
// pub const kana_HU: u32 = 0x4cc;
// pub const kana_i: u32 = 0x4a8;
// pub const kana_I: u32 = 0x4b2;
// pub const kana_KA: u32 = 0x4b6;
// pub const kana_KE: u32 = 0x4b9;
// pub const kana_KI: u32 = 0x4b7;
// pub const kana_KO: u32 = 0x4ba;
// pub const kana_KU: u32 = 0x4b8;
pub const Kana_Lock: u32 = 0xff2d;
// pub const kana_MA: u32 = 0x4cf;
// pub const kana_ME: u32 = 0x4d2;
// pub const kana_MI: u32 = 0x4d0;
// pub const kana_middledot: u32 = 0x4a5;
// pub const kana_MO: u32 = 0x4d3;
// pub const kana_MU: u32 = 0x4d1;
// pub const kana_N: u32 = 0x4dd;
// pub const kana_NA: u32 = 0x4c5;
// pub const kana_NE: u32 = 0x4c8;
// pub const kana_NI: u32 = 0x4c6;
// pub const kana_NO: u32 = 0x4c9;
// pub const kana_NU: u32 = 0x4c7;
// pub const kana_o: u32 = 0x4ab;
// pub const kana_O: u32 = 0x4b5;
// pub const kana_openingbracket: u32 = 0x4a2;
// pub const kana_RA: u32 = 0x4d7;
// pub const kana_RE: u32 = 0x4da;
// pub const kana_RI: u32 = 0x4d8;
// pub const kana_RO: u32 = 0x4db;
// pub const kana_RU: u32 = 0x4d9;
// pub const kana_SA: u32 = 0x4bb;
// pub const kana_SE: u32 = 0x4be;
// pub const kana_SHI: u32 = 0x4bc;
pub const Kana_Shift: u32 = 0xff2e;
// pub const kana_SO: u32 = 0x4bf;
// pub const kana_SU: u32 = 0x4bd;
// pub const kana_switch: u32 = 0xff7e;
// pub const kana_TA: u32 = 0x4c0;
// pub const kana_TE: u32 = 0x4c3;
// pub const kana_TI: u32 = 0x4c1;
// pub const kana_TO: u32 = 0x4c4;
// pub const kana_tsu: u32 = 0x4af;
// pub const kana_TSU: u32 = 0x4c2;
// pub const kana_tu: u32 = 0x4af;
// pub const kana_TU: u32 = 0x4c2;
// pub const kana_u: u32 = 0x4a9;
// pub const kana_U: u32 = 0x4b3;
// pub const kana_WA: u32 = 0x4dc;
// pub const kana_WO: u32 = 0x4a6;
// pub const kana_ya: u32 = 0x4ac;
// pub const kana_YA: u32 = 0x4d4;
// pub const kana_yo: u32 = 0x4ae;
// pub const kana_YO: u32 = 0x4d6;
// pub const kana_yu: u32 = 0x4ad;
// pub const kana_YU: u32 = 0x4d5;
pub const Kanji: u32 = 0xff21;
pub const Kanji_Bangou: u32 = 0xff37;
// pub const kappa: u32 = 0x3a2;
// pub const Katakana: u32 = 0xff26;
// pub const kcedilla: u32 = 0x3f3;
// pub const Kcedilla: u32 = 0x3d3;
// pub const Korean_Won: u32 = 0xeff;
pub const KP_0: u32 = 0xffb0;
pub const KP_1: u32 = 0xffb1;
pub const KP_2: u32 = 0xffb2;
pub const KP_3: u32 = 0xffb3;
pub const KP_4: u32 = 0xffb4;
pub const KP_5: u32 = 0xffb5;
pub const KP_6: u32 = 0xffb6;
pub const KP_7: u32 = 0xffb7;
pub const KP_8: u32 = 0xffb8;
pub const KP_9: u32 = 0xffb9;
pub const KP_Add: u32 = 0xffab;
// pub const KP_BackTab: u32 = 0x1000ff75;
pub const KP_Begin: u32 = 0xff9d;
pub const KP_Decimal: u32 = 0xffae;
pub const KP_Delete: u32 = 0xff9f;
pub const KP_Divide: u32 = 0xffaf;
pub const KP_Down: u32 = 0xff99;
pub const KP_End: u32 = 0xff9c;
pub const KP_Enter: u32 = 0xff8d;
pub const KP_Equal: u32 = 0xffbd;
pub const KP_F1: u32 = 0xff91;
pub const KP_F2: u32 = 0xff92;
pub const KP_F3: u32 = 0xff93;
pub const KP_F4: u32 = 0xff94;
pub const KP_Home: u32 = 0xff95;
pub const KP_Insert: u32 = 0xff9e;
pub const KP_Left: u32 = 0xff96;
pub const KP_Multiply: u32 = 0xffaa;
pub const KP_Next: u32 = 0xff9b;
pub const KP_Page_Down: u32 = 0xff9b;
pub const KP_Page_Up: u32 = 0xff9a;
pub const KP_Prior: u32 = 0xff9a;
pub const KP_Right: u32 = 0xff98;
pub const KP_Separator: u32 = 0xffac;
pub const KP_Space: u32 = 0xff80;
pub const KP_Subtract: u32 = 0xffad;
pub const KP_Tab: u32 = 0xff89;
pub const KP_Up: u32 = 0xff97;
// pub const kra: u32 = 0x3a2;
// pub const l: u32 = 0x6c;
// pub const L: u32 = 0x4c;
// pub const L1: u32 = 0xffc8;
// pub const L10: u32 = 0xffd1;
// pub const L2: u32 = 0xffc9;
// pub const L3: u32 = 0xffca;
// pub const L4: u32 = 0xffcb;
// pub const L5: u32 = 0xffcc;
// pub const L6: u32 = 0xffcd;
// pub const L7: u32 = 0xffce;
// pub const L8: u32 = 0xffcf;
// pub const L9: u32 = 0xffd0;
// pub const lacute: u32 = 0x1e5;
// pub const Lacute: u32 = 0x1c5;
// pub const Last_Virtual_Screen: u32 = 0xfed4;
// pub const latincross: u32 = 0xad9;
// pub const lbelowdot: u32 = 0x1001e37;
// pub const Lbelowdot: u32 = 0x1001e36;
// pub const lcaron: u32 = 0x1b5;
// pub const Lcaron: u32 = 0x1a5;
// pub const lcedilla: u32 = 0x3b6;
// pub const Lcedilla: u32 = 0x3a6;
pub const Left: u32 = 0xff51;
// pub const leftanglebracket: u32 = 0xabc;
// pub const leftarrow: u32 = 0x8fb;
// pub const leftcaret: u32 = 0xba3;
// pub const leftdoublequotemark: u32 = 0xad2;
// pub const leftmiddlecurlybrace: u32 = 0x8af;
// pub const leftopentriangle: u32 = 0xacc;
// pub const leftpointer: u32 = 0xaea;
// pub const leftradical: u32 = 0x8a1;
// pub const leftshoe: u32 = 0xbda;
// pub const leftsingleanglequotemark: u32 = 0x1002039;
// pub const leftsinglequotemark: u32 = 0xad0;
// pub const leftt: u32 = 0x9f4;
// pub const lefttack: u32 = 0xbdc;
// pub const less: u32 = 0x3c;
// pub const lessthanequal: u32 = 0x8bc;
// pub const lf: u32 = 0x9e5;
pub const Linefeed: u32 = 0xff0a;
// pub const lira: u32 = 0x100000af;
// pub const LiraSign: u32 = 0x10020a4;
// pub const logicaland: u32 = 0x8de;
// pub const logicalor: u32 = 0x8df;
// pub const longminus: u32 = 0x100000f6;
// pub const lowleftcorner: u32 = 0x9ed;
// pub const lowrightcorner: u32 = 0x9ea;
// pub const lstroke: u32 = 0x1b3;
// pub const Lstroke: u32 = 0x1a3;
// pub const m: u32 = 0x6d;
pub const M: u32 = 0x4d;
// pub const mabovedot: u32 = 0x1001e41;
// pub const Mabovedot: u32 = 0x1001e40;
// pub const Macedonia_dse: u32 = 0x6a5;
// pub const Macedonia_DSE: u32 = 0x6b5;
// pub const Macedonia_gje: u32 = 0x6a2;
// pub const Macedonia_GJE: u32 = 0x6b2;
// pub const Macedonia_kje: u32 = 0x6ac;
// pub const Macedonia_KJE: u32 = 0x6bc;
// pub const macron: u32 = 0xaf;
pub const Mae_Koho: u32 = 0xff3e;
// pub const malesymbol: u32 = 0xaf7;
// pub const maltesecross: u32 = 0xaf0;
// pub const marker: u32 = 0xabf;
// pub const masculine: u32 = 0xba;
pub const Massyo: u32 = 0xff2c;
pub const Menu: u32 = 0xff67;
pub const Meta_L: u32 = 0xffe7;
pub const Meta_R: u32 = 0xffe8;
// pub const MillSign: u32 = 0x10020a5;
// pub const minus: u32 = 0x2d;
// pub const minutes: u32 = 0xad6;
pub const Mode_switch: u32 = 0xff7e;
// pub const MouseKeys_Accel_Enable: u32 = 0xfe77;
// pub const MouseKeys_Enable: u32 = 0xfe76;
// pub const mu: u32 = 0xb5;
pub const Muhenkan: u32 = 0xff22;
pub const Multi_key: u32 = 0xff20;
pub const MultipleCandidate: u32 = 0xff3d;
// pub const multiply: u32 = 0xd7;
// pub const musicalflat: u32 = 0xaf6;
// pub const musicalsharp: u32 = 0xaf5;
// pub const mute_acute: u32 = 0x100000a8;
// pub const mute_asciicircum: u32 = 0x100000aa;
// pub const mute_asciitilde: u32 = 0x100000ac;
// pub const mute_diaeresis: u32 = 0x100000ab;
// pub const mute_grave: u32 = 0x100000a9;
// pub const n: u32 = 0x6e;
// pub const N: u32 = 0x4e;
// pub const nabla: u32 = 0x8c5;
// pub const nacute: u32 = 0x1f1;
// pub const Nacute: u32 = 0x1d1;
// pub const NairaSign: u32 = 0x10020a6;
// pub const ncaron: u32 = 0x1f2;
// pub const Ncaron: u32 = 0x1d2;
// pub const ncedilla: u32 = 0x3f1;
// pub const Ncedilla: u32 = 0x3d1;
// pub const NewSheqelSign: u32 = 0x10020aa;
pub const Next: u32 = 0xff56;
// pub const Next_Virtual_Screen: u32 = 0xfed2;
// pub const ninesubscript: u32 = 0x1002089;
// pub const ninesuperior: u32 = 0x1002079;
// pub const nl: u32 = 0x9e8;
// pub const nobreakspace: u32 = 0xa0;
// pub const notapproxeq: u32 = 0x1002247;
// pub const notelementof: u32 = 0x1002209;
// pub const notequal: u32 = 0x8bd;
// pub const notidentical: u32 = 0x1002262;
// pub const notsign: u32 = 0xac;
// pub const ntilde: u32 = 0xf1;
// pub const Ntilde: u32 = 0xd1;
pub const Num_Lock: u32 = 0xff7f;
// pub const numbersign: u32 = 0x23;
// pub const numerosign: u32 = 0x6b0;
// pub const o: u32 = 0x6f;
// pub const O: u32 = 0x4f;
// pub const oacute: u32 = 0xf3;
// pub const Oacute: u32 = 0xd3;
// pub const obarred: u32 = 0x1000275;
// pub const Obarred: u32 = 0x100019f;
// pub const obelowdot: u32 = 0x1001ecd;
// pub const Obelowdot: u32 = 0x1001ecc;
// pub const ocaron: u32 = 0x10001d2;
// pub const Ocaron: u32 = 0x10001d1;
// pub const ocircumflex: u32 = 0xf4;
// pub const Ocircumflex: u32 = 0xd4;
// pub const ocircumflexacute: u32 = 0x1001ed1;
// pub const Ocircumflexacute: u32 = 0x1001ed0;
// pub const ocircumflexbelowdot: u32 = 0x1001ed9;
// pub const Ocircumflexbelowdot: u32 = 0x1001ed8;
// pub const ocircumflexgrave: u32 = 0x1001ed3;
// pub const Ocircumflexgrave: u32 = 0x1001ed2;
// pub const ocircumflexhook: u32 = 0x1001ed5;
// pub const Ocircumflexhook: u32 = 0x1001ed4;
// pub const ocircumflextilde: u32 = 0x1001ed7;
// pub const Ocircumflextilde: u32 = 0x1001ed6;
// pub const odiaeresis: u32 = 0xf6;
// pub const Odiaeresis: u32 = 0xd6;
// pub const odoubleacute: u32 = 0x1f5;
// pub const Odoubleacute: u32 = 0x1d5;
// pub const oe: u32 = 0x13bd;
// pub const OE: u32 = 0x13bc;
// pub const ogonek: u32 = 0x1b2;
// pub const ograve: u32 = 0xf2;
// pub const Ograve: u32 = 0xd2;
// pub const ohook: u32 = 0x1001ecf;
// pub const Ohook: u32 = 0x1001ece;
// pub const ohorn: u32 = 0x10001a1;
// pub const Ohorn: u32 = 0x10001a0;
// pub const ohornacute: u32 = 0x1001edb;
// pub const Ohornacute: u32 = 0x1001eda;
// pub const ohornbelowdot: u32 = 0x1001ee3;
// pub const Ohornbelowdot: u32 = 0x1001ee2;
// pub const ohorngrave: u32 = 0x1001edd;
// pub const Ohorngrave: u32 = 0x1001edc;
// pub const ohornhook: u32 = 0x1001edf;
// pub const Ohornhook: u32 = 0x1001ede;
// pub const ohorntilde: u32 = 0x1001ee1;
// pub const Ohorntilde: u32 = 0x1001ee0;
// pub const omacron: u32 = 0x3f2;
// pub const Omacron: u32 = 0x3d2;
// pub const oneeighth: u32 = 0xac3;
// pub const onefifth: u32 = 0xab2;
// pub const onehalf: u32 = 0xbd;
// pub const onequarter: u32 = 0xbc;
// pub const onesixth: u32 = 0xab6;
// pub const onesubscript: u32 = 0x1002081;
// pub const onesuperior: u32 = 0xb9;
// pub const onethird: u32 = 0xab0;
// pub const ooblique: u32 = 0xf8;
// pub const Ooblique: u32 = 0xd8;
// pub const openrectbullet: u32 = 0xae2;
// pub const openstar: u32 = 0xae5;
// pub const opentribulletdown: u32 = 0xae4;
// pub const opentribulletup: u32 = 0xae3;
// pub const ordfeminine: u32 = 0xaa;
// pub const ordmasculine: u32 = 0xba;
// pub const osfActivate: u32 = 0x1004ff44;
// pub const osfAddMode: u32 = 0x1004ff31;
// pub const osfBackSpace: u32 = 0x1004ff08;
// pub const osfBackTab: u32 = 0x1004ff07;
// pub const osfBeginData: u32 = 0x1004ff5a;
// pub const osfBeginLine: u32 = 0x1004ff58;
// pub const osfCancel: u32 = 0x1004ff69;
// pub const osfClear: u32 = 0x1004ff0b;
// pub const osfCopy: u32 = 0x1004ff02;
// pub const osfCut: u32 = 0x1004ff03;
// pub const osfDelete: u32 = 0x1004ffff;
// pub const osfDeselectAll: u32 = 0x1004ff72;
// pub const osfDown: u32 = 0x1004ff54;
// pub const osfEndData: u32 = 0x1004ff59;
// pub const osfEndLine: u32 = 0x1004ff57;
// pub const osfEscape: u32 = 0x1004ff1b;
// pub const osfExtend: u32 = 0x1004ff74;
// pub const osfHelp: u32 = 0x1004ff6a;
// pub const osfInsert: u32 = 0x1004ff63;
// pub const osfLeft: u32 = 0x1004ff51;
// pub const osfMenu: u32 = 0x1004ff67;
// pub const osfMenuBar: u32 = 0x1004ff45;
// pub const osfNextField: u32 = 0x1004ff5e;
// pub const osfNextMenu: u32 = 0x1004ff5c;
// pub const osfPageDown: u32 = 0x1004ff42;
// pub const osfPageLeft: u32 = 0x1004ff40;
// pub const osfPageRight: u32 = 0x1004ff43;
// pub const osfPageUp: u32 = 0x1004ff41;
// pub const osfPaste: u32 = 0x1004ff04;
// pub const osfPrevField: u32 = 0x1004ff5d;
// pub const osfPrevMenu: u32 = 0x1004ff5b;
// pub const osfPrimaryPaste: u32 = 0x1004ff32;
// pub const osfQuickPaste: u32 = 0x1004ff33;
// pub const osfReselect: u32 = 0x1004ff73;
// pub const osfRestore: u32 = 0x1004ff78;
// pub const osfRight: u32 = 0x1004ff53;
// pub const osfSelect: u32 = 0x1004ff60;
// pub const osfSelectAll: u32 = 0x1004ff71;
// pub const osfUndo: u32 = 0x1004ff65;
// pub const osfUp: u32 = 0x1004ff52;
// pub const oslash: u32 = 0xf8;
// pub const Oslash: u32 = 0xd8;
// pub const otilde: u32 = 0xf5;
// pub const Otilde: u32 = 0xd5;
// pub const overbar: u32 = 0xbc0;
// pub const Overlay1_Enable: u32 = 0xfe78;
// pub const Overlay2_Enable: u32 = 0xfe79;
// pub const overline: u32 = 0x47e;
// pub const p: u32 = 0x70;
pub const P: u32 = 0x50;
// pub const pabovedot: u32 = 0x1001e57;
// pub const Pabovedot: u32 = 0x1001e56;
pub const Page_Down: u32 = 0xff56;
pub const Page_Up: u32 = 0xff55;
// pub const paragraph: u32 = 0xb6;
// pub const parenleft: u32 = 0x28;
// pub const parenright: u32 = 0x29;
// pub const partdifferential: u32 = 0x1002202;
// pub const partialderivative: u32 = 0x8ef;
pub const Pause: u32 = 0xff13;
// pub const percent: u32 = 0x25;
// pub const period: u32 = 0x2e;
// pub const periodcentered: u32 = 0xb7;
// pub const permille: u32 = 0xad5;
// pub const PesetaSign: u32 = 0x10020a7;
// pub const phonographcopyright: u32 = 0xafb;
// pub const plus: u32 = 0x2b;
// pub const plusminus: u32 = 0xb1;
// pub const Pointer_Accelerate: u32 = 0xfefa;
// pub const Pointer_Button_Dflt: u32 = 0xfee8;
// pub const Pointer_Button1: u32 = 0xfee9;
// pub const Pointer_Button2: u32 = 0xfeea;
// pub const Pointer_Button3: u32 = 0xfeeb;
// pub const Pointer_Button4: u32 = 0xfeec;
// pub const Pointer_Button5: u32 = 0xfeed;
// pub const Pointer_DblClick_Dflt: u32 = 0xfeee;
// pub const Pointer_DblClick1: u32 = 0xfeef;
// pub const Pointer_DblClick2: u32 = 0xfef0;
// pub const Pointer_DblClick3: u32 = 0xfef1;
// pub const Pointer_DblClick4: u32 = 0xfef2;
// pub const Pointer_DblClick5: u32 = 0xfef3;
// pub const Pointer_DfltBtnNext: u32 = 0xfefb;
// pub const Pointer_DfltBtnPrev: u32 = 0xfefc;
// pub const Pointer_Down: u32 = 0xfee3;
// pub const Pointer_DownLeft: u32 = 0xfee6;
// pub const Pointer_DownRight: u32 = 0xfee7;
// pub const Pointer_Drag_Dflt: u32 = 0xfef4;
// pub const Pointer_Drag1: u32 = 0xfef5;
// pub const Pointer_Drag2: u32 = 0xfef6;
// pub const Pointer_Drag3: u32 = 0xfef7;
// pub const Pointer_Drag4: u32 = 0xfef8;
// pub const Pointer_Drag5: u32 = 0xfefd;
// pub const Pointer_EnableKeys: u32 = 0xfef9;
// pub const Pointer_Left: u32 = 0xfee0;
// pub const Pointer_Right: u32 = 0xfee1;
// pub const Pointer_Up: u32 = 0xfee2;
// pub const Pointer_UpLeft: u32 = 0xfee4;
// pub const Pointer_UpRight: u32 = 0xfee5;
// pub const prescription: u32 = 0xad4;
// pub const Prev_Virtual_Screen: u32 = 0xfed1;
pub const PreviousCandidate: u32 = 0xff3e;
pub const Print: u32 = 0xff61;
pub const Prior: u32 = 0xff55;
// pub const prolongedsound: u32 = 0x4b0;
// pub const punctspace: u32 = 0xaa6;
pub const q: u32 = 0x71;
// pub const Q: u32 = 0x51;
// pub const quad: u32 = 0xbcc;
// pub const question: u32 = 0x3f;
// pub const questiondown: u32 = 0xbf;
// pub const quotedbl: u32 = 0x22;
// pub const quoteleft: u32 = 0x60;
// pub const quoteright: u32 = 0x27;
// pub const r: u32 = 0x72;
pub const R: u32 = 0x52;
// pub const R1: u32 = 0xffd2;
// pub const R10: u32 = 0xffdb;
// pub const R11: u32 = 0xffdc;
// pub const R12: u32 = 0xffdd;
// pub const R13: u32 = 0xffde;
// pub const R14: u32 = 0xffdf;
// pub const R15: u32 = 0xffe0;
// pub const R2: u32 = 0xffd3;
// pub const R3: u32 = 0xffd4;
// pub const R4: u32 = 0xffd5;
// pub const R5: u32 = 0xffd6;
// pub const R6: u32 = 0xffd7;
// pub const R7: u32 = 0xffd8;
// pub const R8: u32 = 0xffd9;
// pub const R9: u32 = 0xffda;
// pub const racute: u32 = 0x1e0;
// pub const Racute: u32 = 0x1c0;
// pub const radical: u32 = 0x8d6;
// pub const rcaron: u32 = 0x1f8;
// pub const Rcaron: u32 = 0x1d8;
// pub const rcedilla: u32 = 0x3b3;
// pub const Rcedilla: u32 = 0x3a3;
pub const Redo: u32 = 0xff66;
// pub const registered: u32 = 0xae;
// pub const RepeatKeys_Enable: u32 = 0xfe72;
// pub const Reset: u32 = 0x1000ff6c;
pub const Return: u32 = 0xff0d;
pub const Right: u32 = 0xff53;
// pub const rightanglebracket: u32 = 0xabe;
// pub const rightarrow: u32 = 0x8fd;
// pub const rightcaret: u32 = 0xba6;
// pub const rightdoublequotemark: u32 = 0xad3;
// pub const rightmiddlecurlybrace: u32 = 0x8b0;
// pub const rightmiddlesummation: u32 = 0x8b7;
// pub const rightopentriangle: u32 = 0xacd;
// pub const rightpointer: u32 = 0xaeb;
// pub const rightshoe: u32 = 0xbd8;
// pub const rightsingleanglequotemark: u32 = 0x100203a;
// pub const rightsinglequotemark: u32 = 0xad1;
// pub const rightt: u32 = 0x9f5;
// pub const righttack: u32 = 0xbfc;
pub const Romaji: u32 = 0xff24;
// pub const RupeeSign: u32 = 0x10020a8;
// pub const s: u32 = 0x73;
// pub const S: u32 = 0x53;
// pub const sabovedot: u32 = 0x1001e61;
// pub const Sabovedot: u32 = 0x1001e60;
// pub const sacute: u32 = 0x1b6;
// pub const Sacute: u32 = 0x1a6;
// pub const scaron: u32 = 0x1b9;
// pub const Scaron: u32 = 0x1a9;
// pub const scedilla: u32 = 0x1ba;
// pub const Scedilla: u32 = 0x1aa;
// pub const schwa: u32 = 0x1000259;
// pub const SCHWA: u32 = 0x100018f;
// pub const scircumflex: u32 = 0x2fe;
// pub const Scircumflex: u32 = 0x2de;
pub const script_switch: u32 = 0xff7e;
pub const Scroll_Lock: u32 = 0xff14;
// pub const seconds: u32 = 0xad7;
// pub const section: u32 = 0xa7;
pub const Select: u32 = 0xff60;
// pub const semicolon: u32 = 0x3b;
// pub const semivoicedsound: u32 = 0x4df;
// pub const Serbian_dje: u32 = 0x6a1;
// pub const Serbian_DJE: u32 = 0x6b1;
// pub const Serbian_dze: u32 = 0x6af;
// pub const Serbian_DZE: u32 = 0x6bf;
// pub const Serbian_je: u32 = 0x6a8;
// pub const Serbian_JE: u32 = 0x6b8;
// pub const Serbian_lje: u32 = 0x6a9;
// pub const Serbian_LJE: u32 = 0x6b9;
// pub const Serbian_nje: u32 = 0x6aa;
// pub const Serbian_NJE: u32 = 0x6ba;
// pub const Serbian_tshe: u32 = 0x6ab;
// pub const Serbian_TSHE: u32 = 0x6bb;
// pub const seveneighths: u32 = 0xac6;
// pub const sevensubscript: u32 = 0x1002087;
// pub const sevensuperior: u32 = 0x1002077;
pub const Shift_L: u32 = 0xffe1;
pub const Shift_Lock: u32 = 0xffe6;
pub const Shift_R: u32 = 0xffe2;
// pub const signaturemark: u32 = 0xaca;
// pub const signifblank: u32 = 0xaac;
// pub const similarequal: u32 = 0x8c9;
pub const SingleCandidate: u32 = 0xff3c;
// pub const singlelowquotemark: u32 = 0xafd;
// pub const Sinh_a: u32 = 0x1000d85;
// pub const Sinh_aa: u32 = 0x1000d86;
// pub const Sinh_aa2: u32 = 0x1000dcf;
// pub const Sinh_ae: u32 = 0x1000d87;
// pub const Sinh_ae2: u32 = 0x1000dd0;
// pub const Sinh_aee: u32 = 0x1000d88;
// pub const Sinh_aee2: u32 = 0x1000dd1;
// pub const Sinh_ai: u32 = 0x1000d93;
// pub const Sinh_ai2: u32 = 0x1000ddb;
// pub const Sinh_al: u32 = 0x1000dca;
// pub const Sinh_au: u32 = 0x1000d96;
// pub const Sinh_au2: u32 = 0x1000dde;
// pub const Sinh_ba: u32 = 0x1000db6;
// pub const Sinh_bha: u32 = 0x1000db7;
// pub const Sinh_ca: u32 = 0x1000da0;
// pub const Sinh_cha: u32 = 0x1000da1;
// pub const Sinh_dda: u32 = 0x1000da9;
// pub const Sinh_ddha: u32 = 0x1000daa;
// pub const Sinh_dha: u32 = 0x1000daf;
// pub const Sinh_dhha: u32 = 0x1000db0;
// pub const Sinh_e: u32 = 0x1000d91;
// pub const Sinh_e2: u32 = 0x1000dd9;
// pub const Sinh_ee: u32 = 0x1000d92;
// pub const Sinh_ee2: u32 = 0x1000dda;
// pub const Sinh_fa: u32 = 0x1000dc6;
// pub const Sinh_ga: u32 = 0x1000d9c;
// pub const Sinh_gha: u32 = 0x1000d9d;
// pub const Sinh_h2: u32 = 0x1000d83;
// pub const Sinh_ha: u32 = 0x1000dc4;
// pub const Sinh_i: u32 = 0x1000d89;
// pub const Sinh_i2: u32 = 0x1000dd2;
// pub const Sinh_ii: u32 = 0x1000d8a;
// pub const Sinh_ii2: u32 = 0x1000dd3;
// pub const Sinh_ja: u32 = 0x1000da2;
// pub const Sinh_jha: u32 = 0x1000da3;
// pub const Sinh_jnya: u32 = 0x1000da5;
// pub const Sinh_ka: u32 = 0x1000d9a;
// pub const Sinh_kha: u32 = 0x1000d9b;
// pub const Sinh_kunddaliya: u32 = 0x1000df4;
// pub const Sinh_la: u32 = 0x1000dbd;
// pub const Sinh_lla: u32 = 0x1000dc5;
// pub const Sinh_lu: u32 = 0x1000d8f;
// pub const Sinh_lu2: u32 = 0x1000ddf;
// pub const Sinh_luu: u32 = 0x1000d90;
// pub const Sinh_luu2: u32 = 0x1000df3;
// pub const Sinh_ma: u32 = 0x1000db8;
// pub const Sinh_mba: u32 = 0x1000db9;
// pub const Sinh_na: u32 = 0x1000db1;
// pub const Sinh_ndda: u32 = 0x1000dac;
// pub const Sinh_ndha: u32 = 0x1000db3;
// pub const Sinh_ng: u32 = 0x1000d82;
// pub const Sinh_ng2: u32 = 0x1000d9e;
// pub const Sinh_nga: u32 = 0x1000d9f;
// pub const Sinh_nja: u32 = 0x1000da6;
// pub const Sinh_nna: u32 = 0x1000dab;
// pub const Sinh_nya: u32 = 0x1000da4;
// pub const Sinh_o: u32 = 0x1000d94;
// pub const Sinh_o2: u32 = 0x1000ddc;
// pub const Sinh_oo: u32 = 0x1000d95;
// pub const Sinh_oo2: u32 = 0x1000ddd;
// pub const Sinh_pa: u32 = 0x1000db4;
// pub const Sinh_pha: u32 = 0x1000db5;
// pub const Sinh_ra: u32 = 0x1000dbb;
// pub const Sinh_ri: u32 = 0x1000d8d;
// pub const Sinh_rii: u32 = 0x1000d8e;
// pub const Sinh_ru2: u32 = 0x1000dd8;
// pub const Sinh_ruu2: u32 = 0x1000df2;
// pub const Sinh_sa: u32 = 0x1000dc3;
// pub const Sinh_sha: u32 = 0x1000dc1;
// pub const Sinh_ssha: u32 = 0x1000dc2;
// pub const Sinh_tha: u32 = 0x1000dad;
// pub const Sinh_thha: u32 = 0x1000dae;
// pub const Sinh_tta: u32 = 0x1000da7;
// pub const Sinh_ttha: u32 = 0x1000da8;
// pub const Sinh_u: u32 = 0x1000d8b;
// pub const Sinh_u2: u32 = 0x1000dd4;
// pub const Sinh_uu: u32 = 0x1000d8c;
// pub const Sinh_uu2: u32 = 0x1000dd6;
// pub const Sinh_va: u32 = 0x1000dc0;
// pub const Sinh_ya: u32 = 0x1000dba;
// pub const sixsubscript: u32 = 0x1002086;
// pub const sixsuperior: u32 = 0x1002076;
// pub const slash: u32 = 0x2f;
// pub const SlowKeys_Enable: u32 = 0xfe73;
// pub const soliddiamond: u32 = 0x9e0;
pub const space: u32 = 0x20;
// pub const squareroot: u32 = 0x100221a;
// pub const ssharp: u32 = 0xdf;
// pub const SSHARP: u32 = 0x1001e9e;
// pub const sterling: u32 = 0xa3;
// pub const StickyKeys_Enable: u32 = 0xfe75;
// pub const stricteq: u32 = 0x1002263;
// pub const SunAgain: u32 = 0xff66;
// pub const SunAltGraph: u32 = 0xff7e;
// pub const SunAudioLowerVolume: u32 = 0x1005ff77;
// pub const SunAudioMute: u32 = 0x1005ff78;
// pub const SunAudioRaiseVolume: u32 = 0x1005ff79;
// pub const SunCompose: u32 = 0xff20;
// pub const SunCopy: u32 = 0x1005ff72;
// pub const SunCut: u32 = 0x1005ff75;
pub const SunF36: u32 = 0x1005ff10;
pub const SunF37: u32 = 0x1005ff11;
// pub const SunFA_Acute: u32 = 0x1005ff03;
// pub const SunFA_Cedilla: u32 = 0x1005ff05;
// pub const SunFA_Circum: u32 = 0x1005ff01;
// pub const SunFA_Diaeresis: u32 = 0x1005ff04;
// pub const SunFA_Grave: u32 = 0x1005ff00;
// pub const SunFA_Tilde: u32 = 0x1005ff02;
// pub const SunFind: u32 = 0xff68;
// pub const SunFront: u32 = 0x1005ff71;
// pub const SunOpen: u32 = 0x1005ff73;
// pub const SunPageDown: u32 = 0xff56;
// pub const SunPageUp: u32 = 0xff55;
// pub const SunPaste: u32 = 0x1005ff74;
// pub const SunPowerSwitch: u32 = 0x1005ff76;
// pub const SunPowerSwitchShift: u32 = 0x1005ff7d;
// pub const SunPrint_Screen: u32 = 0xff61;
// pub const SunProps: u32 = 0x1005ff70;
// pub const SunStop: u32 = 0xff69;
pub const SunSys_Req: u32 = 0x1005ff60;
// pub const SunUndo: u32 = 0xff65;
// pub const SunVideoDegauss: u32 = 0x1005ff7a;
// pub const SunVideoLowerBrightness: u32 = 0x1005ff7b;
// pub const SunVideoRaiseBrightness: u32 = 0x1005ff7c;
pub const Super_L: u32 = 0xffeb;
pub const Super_R: u32 = 0xffec;
pub const Sys_Req: u32 = 0xff15;
// pub const System: u32 = 0x1000ff6d;
// pub const t: u32 = 0x74;
pub const T: u32 = 0x54;
pub const Tab: u32 = 0xff09;
// pub const tabovedot: u32 = 0x1001e6b;
// pub const Tabovedot: u32 = 0x1001e6a;
// pub const tcaron: u32 = 0x1bb;
// pub const Tcaron: u32 = 0x1ab;
// pub const tcedilla: u32 = 0x1fe;
// pub const Tcedilla: u32 = 0x1de;
// pub const telephone: u32 = 0xaf9;
// pub const telephonerecorder: u32 = 0xafa;
// pub const Terminate_Server: u32 = 0xfed5;
// pub const Thai_baht: u32 = 0xddf;
// pub const Thai_bobaimai: u32 = 0xdba;
// pub const Thai_chochan: u32 = 0xda8;
// pub const Thai_chochang: u32 = 0xdaa;
// pub const Thai_choching: u32 = 0xda9;
// pub const Thai_chochoe: u32 = 0xdac;
// pub const Thai_dochada: u32 = 0xdae;
// pub const Thai_dodek: u32 = 0xdb4;
// pub const Thai_fofa: u32 = 0xdbd;
// pub const Thai_fofan: u32 = 0xdbf;
// pub const Thai_hohip: u32 = 0xdcb;
// pub const Thai_honokhuk: u32 = 0xdce;
// pub const Thai_khokhai: u32 = 0xda2;
// pub const Thai_khokhon: u32 = 0xda5;
// pub const Thai_khokhuat: u32 = 0xda3;
// pub const Thai_khokhwai: u32 = 0xda4;
// pub const Thai_khorakhang: u32 = 0xda6;
// pub const Thai_kokai: u32 = 0xda1;
// pub const Thai_lakkhangyao: u32 = 0xde5;
// pub const Thai_lekchet: u32 = 0xdf7;
// pub const Thai_lekha: u32 = 0xdf5;
// pub const Thai_lekhok: u32 = 0xdf6;
// pub const Thai_lekkao: u32 = 0xdf9;
// pub const Thai_leknung: u32 = 0xdf1;
// pub const Thai_lekpaet: u32 = 0xdf8;
// pub const Thai_leksam: u32 = 0xdf3;
// pub const Thai_leksi: u32 = 0xdf4;
// pub const Thai_leksong: u32 = 0xdf2;
// pub const Thai_leksun: u32 = 0xdf0;
// pub const Thai_lochula: u32 = 0xdcc;
// pub const Thai_loling: u32 = 0xdc5;
// pub const Thai_lu: u32 = 0xdc6;
// pub const Thai_maichattawa: u32 = 0xdeb;
// pub const Thai_maiek: u32 = 0xde8;
// pub const Thai_maihanakat: u32 = 0xdd1;
// pub const Thai_maihanakat_maitho: u32 = 0xdde;
// pub const Thai_maitaikhu: u32 = 0xde7;
// pub const Thai_maitho: u32 = 0xde9;
// pub const Thai_maitri: u32 = 0xdea;
// pub const Thai_maiyamok: u32 = 0xde6;
// pub const Thai_moma: u32 = 0xdc1;
// pub const Thai_ngongu: u32 = 0xda7;
// pub const Thai_nikhahit: u32 = 0xded;
// pub const Thai_nonen: u32 = 0xdb3;
// pub const Thai_nonu: u32 = 0xdb9;
// pub const Thai_oang: u32 = 0xdcd;
// pub const Thai_paiyannoi: u32 = 0xdcf;
// pub const Thai_phinthu: u32 = 0xdda;
// pub const Thai_phophan: u32 = 0xdbe;
// pub const Thai_phophung: u32 = 0xdbc;
// pub const Thai_phosamphao: u32 = 0xdc0;
// pub const Thai_popla: u32 = 0xdbb;
// pub const Thai_rorua: u32 = 0xdc3;
// pub const Thai_ru: u32 = 0xdc4;
// pub const Thai_saraa: u32 = 0xdd0;
// pub const Thai_saraaa: u32 = 0xdd2;
// pub const Thai_saraae: u32 = 0xde1;
// pub const Thai_saraaimaimalai: u32 = 0xde4;
// pub const Thai_saraaimaimuan: u32 = 0xde3;
// pub const Thai_saraam: u32 = 0xdd3;
// pub const Thai_sarae: u32 = 0xde0;
// pub const Thai_sarai: u32 = 0xdd4;
// pub const Thai_saraii: u32 = 0xdd5;
// pub const Thai_sarao: u32 = 0xde2;
// pub const Thai_sarau: u32 = 0xdd8;
// pub const Thai_saraue: u32 = 0xdd6;
// pub const Thai_sarauee: u32 = 0xdd7;
// pub const Thai_sarauu: u32 = 0xdd9;
// pub const Thai_sorusi: u32 = 0xdc9;
// pub const Thai_sosala: u32 = 0xdc8;
// pub const Thai_soso: u32 = 0xdab;
// pub const Thai_sosua: u32 = 0xdca;
// pub const Thai_thanthakhat: u32 = 0xdec;
// pub const Thai_thonangmontho: u32 = 0xdb1;
// pub const Thai_thophuthao: u32 = 0xdb2;
// pub const Thai_thothahan: u32 = 0xdb7;
// pub const Thai_thothan: u32 = 0xdb0;
// pub const Thai_thothong: u32 = 0xdb8;
// pub const Thai_thothung: u32 = 0xdb6;
// pub const Thai_topatak: u32 = 0xdaf;
// pub const Thai_totao: u32 = 0xdb5;
// pub const Thai_wowaen: u32 = 0xdc7;
// pub const Thai_yoyak: u32 = 0xdc2;
// pub const Thai_yoying: u32 = 0xdad;
// pub const therefore: u32 = 0x8c0;
// pub const thinspace: u32 = 0xaa7;
// pub const thorn: u32 = 0xfe;
// pub const Thorn: u32 = 0xde;
// pub const THORN: u32 = 0xde;
// pub const threeeighths: u32 = 0xac4;
// pub const threefifths: u32 = 0xab4;
// pub const threequarters: u32 = 0xbe;
// pub const threesubscript: u32 = 0x1002083;
// pub const threesuperior: u32 = 0xb3;
// pub const tintegral: u32 = 0x100222d;
// pub const topintegral: u32 = 0x8a4;
// pub const topleftparens: u32 = 0x8ab;
// pub const topleftradical: u32 = 0x8a2;
// pub const topleftsqbracket: u32 = 0x8a7;
// pub const topleftsummation: u32 = 0x8b1;
// pub const toprightparens: u32 = 0x8ad;
// pub const toprightsqbracket: u32 = 0x8a9;
// pub const toprightsummation: u32 = 0x8b5;
// pub const topt: u32 = 0x9f7;
// pub const topvertsummationconnector: u32 = 0x8b3;
pub const Touroku: u32 = 0xff2b;
// pub const trademark: u32 = 0xac9;
// pub const trademarkincircle: u32 = 0xacb;
// pub const tslash: u32 = 0x3bc;
// pub const Tslash: u32 = 0x3ac;
// pub const twofifths: u32 = 0xab3;
// pub const twosubscript: u32 = 0x1002082;
// pub const twosuperior: u32 = 0xb2;
// pub const twothirds: u32 = 0xab1;
// pub const u: u32 = 0x75;
// pub const U: u32 = 0x55;
// pub const uacute: u32 = 0xfa;
// pub const Uacute: u32 = 0xda;
// pub const ubelowdot: u32 = 0x1001ee5;
// pub const Ubelowdot: u32 = 0x1001ee4;
// pub const ubreve: u32 = 0x2fd;
// pub const Ubreve: u32 = 0x2dd;
// pub const ucircumflex: u32 = 0xfb;
// pub const Ucircumflex: u32 = 0xdb;
// pub const udiaeresis: u32 = 0xfc;
// pub const Udiaeresis: u32 = 0xdc;
// pub const udoubleacute: u32 = 0x1fb;
// pub const Udoubleacute: u32 = 0x1db;
// pub const ugrave: u32 = 0xf9;
// pub const Ugrave: u32 = 0xd9;
// pub const uhook: u32 = 0x1001ee7;
// pub const Uhook: u32 = 0x1001ee6;
// pub const uhorn: u32 = 0x10001b0;
// pub const Uhorn: u32 = 0x10001af;
// pub const uhornacute: u32 = 0x1001ee9;
// pub const Uhornacute: u32 = 0x1001ee8;
// pub const uhornbelowdot: u32 = 0x1001ef1;
// pub const Uhornbelowdot: u32 = 0x1001ef0;
// pub const uhorngrave: u32 = 0x1001eeb;
// pub const Uhorngrave: u32 = 0x1001eea;
// pub const uhornhook: u32 = 0x1001eed;
// pub const Uhornhook: u32 = 0x1001eec;
// pub const uhorntilde: u32 = 0x1001eef;
// pub const Uhorntilde: u32 = 0x1001eee;
// pub const Ukrainian_ghe_with_upturn: u32 = 0x6ad;
// pub const Ukrainian_GHE_WITH_UPTURN: u32 = 0x6bd;
// pub const Ukrainian_i: u32 = 0x6a6;
// pub const Ukrainian_I: u32 = 0x6b6;
// pub const Ukrainian_ie: u32 = 0x6a4;
// pub const Ukrainian_IE: u32 = 0x6b4;
// pub const Ukrainian_yi: u32 = 0x6a7;
// pub const Ukrainian_YI: u32 = 0x6b7;
// pub const Ukranian_i: u32 = 0x6a6;
// pub const Ukranian_I: u32 = 0x6b6;
// pub const Ukranian_je: u32 = 0x6a4;
// pub const Ukranian_JE: u32 = 0x6b4;
// pub const Ukranian_yi: u32 = 0x6a7;
// pub const Ukranian_YI: u32 = 0x6b7;
// pub const umacron: u32 = 0x3fe;
// pub const Umacron: u32 = 0x3de;
// pub const underbar: u32 = 0xbc6;
// pub const underscore: u32 = 0x5f;
pub const Undo: u32 = 0xff65;
// pub const union: u32 = 0x8dd;
// pub const uogonek: u32 = 0x3f9;
// pub const Uogonek: u32 = 0x3d9;
pub const Up: u32 = 0xff52;
// pub const uparrow: u32 = 0x8fc;
// pub const upcaret: u32 = 0xba9;
// pub const upleftcorner: u32 = 0x9ec;
// pub const uprightcorner: u32 = 0x9eb;
// pub const upshoe: u32 = 0xbc3;
// pub const upstile: u32 = 0xbd3;
// pub const uptack: u32 = 0xbce;
// pub const uring: u32 = 0x1f9;
// pub const Uring: u32 = 0x1d9;
// pub const User: u32 = 0x1000ff6e;
// pub const utilde: u32 = 0x3fd;
// pub const Utilde: u32 = 0x3dd;
// pub const v: u32 = 0x76;
// pub const V: u32 = 0x56;
// pub const variation: u32 = 0x8c1;
// pub const vertbar: u32 = 0x9f8;
// pub const vertconnector: u32 = 0x8a6;
// pub const voicedsound: u32 = 0x4de;
// pub const VoidSymbol: u32 = 0xffffff;
// pub const vt: u32 = 0x9e9;
// pub const w: u32 = 0x77;
pub const W: u32 = 0x57;
// pub const wacute: u32 = 0x1001e83;
// pub const Wacute: u32 = 0x1001e82;
// pub const wcircumflex: u32 = 0x1000175;
// pub const Wcircumflex: u32 = 0x1000174;
// pub const wdiaeresis: u32 = 0x1001e85;
// pub const Wdiaeresis: u32 = 0x1001e84;
// pub const wgrave: u32 = 0x1001e81;
// pub const Wgrave: u32 = 0x1001e80;
// pub const WonSign: u32 = 0x10020a9;
// pub const x: u32 = 0x78;
// pub const X: u32 = 0x58;
// pub const xabovedot: u32 = 0x1001e8b;
// pub const Xabovedot: u32 = 0x1001e8a;
// pub const XF8610ChannelsDown: u32 = 0x100811b9;
// pub const XF8610ChannelsUp: u32 = 0x100811b8;
// pub const XF863DMode: u32 = 0x1008126f;
// pub const XF86Accessibility: u32 = 0x1008124e;
// pub const XF86AddFavorite: u32 = 0x1008ff39;
// pub const XF86Addressbook: u32 = 0x100811ad;
// pub const XF86ALSToggle: u32 = 0x10081230;
// pub const XF86ApplicationLeft: u32 = 0x1008ff50;
// pub const XF86ApplicationRight: u32 = 0x1008ff51;
// pub const XF86AppSelect: u32 = 0x10081244;
// pub const XF86AspectRatio: u32 = 0x10081177;
// pub const XF86Assistant: u32 = 0x10081247;
// pub const XF86AttendantOff: u32 = 0x1008121c;
// pub const XF86AttendantOn: u32 = 0x1008121b;
// pub const XF86AttendantToggle: u32 = 0x1008121d;
// pub const XF86Audio: u32 = 0x10081188;
// pub const XF86AudioBassBoost: u32 = 0x100810d1;
// pub const XF86AudioChannelMode: u32 = 0x10081175;
// pub const XF86AudioCycleTrack: u32 = 0x1008ff9b;
// pub const XF86AudioDesc: u32 = 0x1008126e;
// pub const XF86AudioForward: u32 = 0x1008ff97;
// pub const XF86AudioLowerVolume: u32 = 0x1008ff11;
// pub const XF86AudioMedia: u32 = 0x1008ff32;
// pub const XF86AudioMicMute: u32 = 0x1008ffb2;
// pub const XF86AudioMute: u32 = 0x1008ff12;
// pub const XF86AudioNext: u32 = 0x1008ff17;
// pub const XF86AudioPause: u32 = 0x1008ff31;
// pub const XF86AudioPlay: u32 = 0x1008ff14;
// pub const XF86AudioPreset: u32 = 0x1008ffb6;
// pub const XF86AudioPrev: u32 = 0x1008ff16;
// pub const XF86AudioRaiseVolume: u32 = 0x1008ff13;
// pub const XF86AudioRandomPlay: u32 = 0x1008ff99;
// pub const XF86AudioRecord: u32 = 0x1008ff1c;
// pub const XF86AudioRepeat: u32 = 0x1008ff98;
// pub const XF86AudioRewind: u32 = 0x1008ff3e;
// pub const XF86AudioStop: u32 = 0x1008ff15;
// pub const XF86AutopilotEngageToggle: u32 = 0x1008127d;
// pub const XF86Away: u32 = 0x1008ff8d;
// pub const XF86Back: u32 = 0x1008ff26;
// pub const XF86BackForward: u32 = 0x1008ff3f;
// pub const XF86Battery: u32 = 0x1008ff93;
// pub const XF86Blue: u32 = 0x1008ffa6;
// pub const XF86Bluetooth: u32 = 0x1008ff94;
// pub const XF86Book: u32 = 0x1008ff52;
// pub const XF86Break: u32 = 0x1008119b;
// pub const XF86BrightnessAdjust: u32 = 0x1008ff3b;
// pub const XF86BrightnessAuto: u32 = 0x100810f4;
// pub const XF86BrightnessMax: u32 = 0x10081251;
// pub const XF86BrightnessMin: u32 = 0x10081250;
// pub const XF86Buttonconfig: u32 = 0x10081240;
// pub const XF86Calculater: u32 = 0x1008ff54;
// pub const XF86Calculator: u32 = 0x1008ff1d;
// pub const XF86Calendar: u32 = 0x1008ff20;
// pub const XF86CameraAccessDisable: u32 = 0x1008124c;
// pub const XF86CameraAccessEnable: u32 = 0x1008124b;
// pub const XF86CameraAccessToggle: u32 = 0x1008124d;
// pub const XF86CameraDown: u32 = 0x10081218;
// pub const XF86CameraFocus: u32 = 0x10081210;
// pub const XF86CameraLeft: u32 = 0x10081219;
// pub const XF86CameraRight: u32 = 0x1008121a;
// pub const XF86CameraUp: u32 = 0x10081217;
// pub const XF86CameraZoomIn: u32 = 0x10081215;
// pub const XF86CameraZoomOut: u32 = 0x10081216;
// pub const XF86CD: u32 = 0x1008ff53;
// pub const XF86ChannelDown: u32 = 0x10081193;
// pub const XF86ChannelUp: u32 = 0x10081192;
// pub const XF86Clear: u32 = 0x1008ff55;
// pub const XF86ClearGrab: u32 = 0x1008fe21;
// pub const XF86ClearvuSonar: u32 = 0x10081286;
// pub const XF86Close: u32 = 0x1008ff56;
// pub const XF86Community: u32 = 0x1008ff3d;
// pub const XF86ContextMenu: u32 = 0x100811b6;
// pub const XF86ContrastAdjust: u32 = 0x1008ff22;
// pub const XF86ControlPanel: u32 = 0x10081243;
// pub const XF86Copy: u32 = 0x1008ff57;
// pub const XF86Cut: u32 = 0x1008ff58;
// pub const XF86CycleAngle: u32 = 0x1008ff9c;
// pub const XF86Data: u32 = 0x10081277;
// pub const XF86Database: u32 = 0x100811aa;
// pub const XF86Dictate: u32 = 0x1008124a;
// pub const XF86Display: u32 = 0x1008ff59;
// pub const XF86DisplayOff: u32 = 0x100810f5;
// pub const XF86DisplayToggle: u32 = 0x100811af;
// pub const XF86Documents: u32 = 0x1008ff5b;
// pub const XF86DoNotDisturb: u32 = 0x1008124f;
// pub const XF86DOS: u32 = 0x1008ff5a;
// pub const XF86DualRangeRadar: u32 = 0x10081283;
// pub const XF86DVD: u32 = 0x10081185;
// pub const XF86Editor: u32 = 0x100811a6;
// pub const XF86Eject: u32 = 0x1008ff2c;
// pub const XF86ElectronicPrivacyScreenOff: u32 = 0x10081253;
// pub const XF86ElectronicPrivacyScreenOn: u32 = 0x10081252;
// pub const XF86EmojiPicker: u32 = 0x10081249;
// pub const XF86Excel: u32 = 0x1008ff5c;
// pub const XF86Exit: u32 = 0x100810ae;
// pub const XF86Explorer: u32 = 0x1008ff5d;
// pub const XF86FastReverse: u32 = 0x10081275;
// pub const XF86Favorites: u32 = 0x1008ff30;
// pub const XF86Finance: u32 = 0x1008ff3c;
// pub const XF86FishingChart: u32 = 0x10081281;
// pub const XF86Fn: u32 = 0x100811d0;
// pub const XF86Fn_1: u32 = 0x100811de;
// pub const XF86Fn_2: u32 = 0x100811df;
// pub const XF86Fn_B: u32 = 0x100811e4;
// pub const XF86Fn_D: u32 = 0x100811e0;
// pub const XF86Fn_E: u32 = 0x100811e1;
// pub const XF86Fn_Esc: u32 = 0x100811d1;
// pub const XF86Fn_F: u32 = 0x100811e2;
// pub const XF86Fn_F1: u32 = 0x100811d2;
// pub const XF86Fn_F10: u32 = 0x100811db;
// pub const XF86Fn_F11: u32 = 0x100811dc;
// pub const XF86Fn_F12: u32 = 0x100811dd;
// pub const XF86Fn_F2: u32 = 0x100811d3;
// pub const XF86Fn_F3: u32 = 0x100811d4;
// pub const XF86Fn_F4: u32 = 0x100811d5;
// pub const XF86Fn_F5: u32 = 0x100811d6;
// pub const XF86Fn_F6: u32 = 0x100811d7;
// pub const XF86Fn_F7: u32 = 0x100811d8;
// pub const XF86Fn_F8: u32 = 0x100811d9;
// pub const XF86Fn_F9: u32 = 0x100811da;
// pub const XF86Fn_S: u32 = 0x100811e3;
// pub const XF86FnRightShift: u32 = 0x100811e5;
// pub const XF86Forward: u32 = 0x1008ff27;
// pub const XF86FrameBack: u32 = 0x1008ff9d;
// pub const XF86FrameForward: u32 = 0x1008ff9e;
// pub const XF86FullScreen: u32 = 0x1008ffb8;
// pub const XF86Game: u32 = 0x1008ff5e;
// pub const XF86Go: u32 = 0x1008ff5f;
// pub const XF86GoTo: u32 = 0x10081162;
// pub const XF86GraphicsEditor: u32 = 0x100811a8;
// pub const XF86Green: u32 = 0x1008ffa4;
// pub const XF86HangupPhone: u32 = 0x100811be;
// pub const XF86Hibernate: u32 = 0x1008ffa8;
// pub const XF86History: u32 = 0x1008ff37;
// pub const XF86HomePage: u32 = 0x1008ff18;
// pub const XF86HotLinks: u32 = 0x1008ff3a;
// pub const XF86Images: u32 = 0x100811ba;
// pub const XF86Info: u32 = 0x10081166;
// pub const XF86iTouch: u32 = 0x1008ff60;
// pub const XF86Journal: u32 = 0x10081242;
// pub const XF86KbdBrightnessDown: u32 = 0x1008ff06;
// pub const XF86KbdBrightnessUp: u32 = 0x1008ff05;
// pub const XF86KbdInputAssistAccept: u32 = 0x10081264;
// pub const XF86KbdInputAssistCancel: u32 = 0x10081265;
// pub const XF86KbdInputAssistNext: u32 = 0x10081261;
// pub const XF86KbdInputAssistNextgroup: u32 = 0x10081263;
// pub const XF86KbdInputAssistPrev: u32 = 0x10081260;
// pub const XF86KbdInputAssistPrevgroup: u32 = 0x10081262;
// pub const XF86KbdLcdMenu1: u32 = 0x100812b8;
// pub const XF86KbdLcdMenu2: u32 = 0x100812b9;
// pub const XF86KbdLcdMenu3: u32 = 0x100812ba;
// pub const XF86KbdLcdMenu4: u32 = 0x100812bb;
// pub const XF86KbdLcdMenu5: u32 = 0x100812bc;
// pub const XF86KbdLightOnOff: u32 = 0x1008ff04;
// pub const XF86Keyboard: u32 = 0x1008ffb3;
// pub const XF86Launch0: u32 = 0x1008ff40;
// pub const XF86Launch1: u32 = 0x1008ff41;
// pub const XF86Launch2: u32 = 0x1008ff42;
// pub const XF86Launch3: u32 = 0x1008ff43;
// pub const XF86Launch4: u32 = 0x1008ff44;
// pub const XF86Launch5: u32 = 0x1008ff45;
// pub const XF86Launch6: u32 = 0x1008ff46;
// pub const XF86Launch7: u32 = 0x1008ff47;
// pub const XF86Launch8: u32 = 0x1008ff48;
// pub const XF86Launch9: u32 = 0x1008ff49;
// pub const XF86LaunchA: u32 = 0x1008ff4a;
// pub const XF86LaunchB: u32 = 0x1008ff4b;
// pub const XF86LaunchC: u32 = 0x1008ff4c;
// pub const XF86LaunchD: u32 = 0x1008ff4d;
// pub const XF86LaunchE: u32 = 0x1008ff4e;
// pub const XF86LaunchF: u32 = 0x1008ff4f;
// pub const XF86LeftDown: u32 = 0x10081269;
// pub const XF86LeftUp: u32 = 0x10081268;
// pub const XF86LightBulb: u32 = 0x1008ff35;
// pub const XF86LightsToggle: u32 = 0x1008121e;
// pub const XF86LinkPhone: u32 = 0x100811bf;
// pub const XF86LogGrabInfo: u32 = 0x1008fe25;
// pub const XF86LogOff: u32 = 0x1008ff61;
// pub const XF86LogWindowTree: u32 = 0x1008fe24;
// pub const XF86Macro1: u32 = 0x10081290;
// pub const XF86Macro10: u32 = 0x10081299;
// pub const XF86Macro11: u32 = 0x1008129a;
// pub const XF86Macro12: u32 = 0x1008129b;
// pub const XF86Macro13: u32 = 0x1008129c;
// pub const XF86Macro14: u32 = 0x1008129d;
// pub const XF86Macro15: u32 = 0x1008129e;
// pub const XF86Macro16: u32 = 0x1008129f;
// pub const XF86Macro17: u32 = 0x100812a0;
// pub const XF86Macro18: u32 = 0x100812a1;
// pub const XF86Macro19: u32 = 0x100812a2;
// pub const XF86Macro2: u32 = 0x10081291;
// pub const XF86Macro20: u32 = 0x100812a3;
// pub const XF86Macro21: u32 = 0x100812a4;
// pub const XF86Macro22: u32 = 0x100812a5;
// pub const XF86Macro23: u32 = 0x100812a6;
// pub const XF86Macro24: u32 = 0x100812a7;
// pub const XF86Macro25: u32 = 0x100812a8;
// pub const XF86Macro26: u32 = 0x100812a9;
// pub const XF86Macro27: u32 = 0x100812aa;
// pub const XF86Macro28: u32 = 0x100812ab;
// pub const XF86Macro29: u32 = 0x100812ac;
// pub const XF86Macro3: u32 = 0x10081292;
// pub const XF86Macro30: u32 = 0x100812ad;
// pub const XF86Macro4: u32 = 0x10081293;
// pub const XF86Macro5: u32 = 0x10081294;
// pub const XF86Macro6: u32 = 0x10081295;
// pub const XF86Macro7: u32 = 0x10081296;
// pub const XF86Macro8: u32 = 0x10081297;
// pub const XF86Macro9: u32 = 0x10081298;
// pub const XF86MacroPreset1: u32 = 0x100812b3;
// pub const XF86MacroPreset2: u32 = 0x100812b4;
// pub const XF86MacroPreset3: u32 = 0x100812b5;
// pub const XF86MacroPresetCycle: u32 = 0x100812b2;
// pub const XF86MacroRecordStart: u32 = 0x100812b0;
// pub const XF86MacroRecordStop: u32 = 0x100812b1;
// pub const XF86Mail: u32 = 0x1008ff19;
// pub const XF86MailForward: u32 = 0x1008ff90;
// pub const XF86Market: u32 = 0x1008ff62;
// pub const XF86MarkWaypoint: u32 = 0x1008127e;
// pub const XF86MediaLanguageMenu: u32 = 0x10081170;
// pub const XF86MediaPlayer: u32 = 0x10081183;
// pub const XF86MediaPlayPause: u32 = 0x100810a4;
// pub const XF86MediaPlaySlow: u32 = 0x10081199;
// pub const XF86MediaRepeat: u32 = 0x100811b7;
// pub const XF86MediaSelectAuxiliary: u32 = 0x10081186;
// pub const XF86MediaSelectCable: u32 = 0x1008117a;
// pub const XF86MediaSelectCD: u32 = 0x1008ff53;
// pub const XF86MediaSelectDVD: u32 = 0x10081185;
// pub const XF86MediaSelectHome: u32 = 0x1008116e;
// pub const XF86MediaSelectPC: u32 = 0x10081178;
// pub const XF86MediaSelectProgramGuide: u32 = 0x1008116a;
// pub const XF86MediaSelectRadio: u32 = 0x10081181;
// pub const XF86MediaSelectSatellite: u32 = 0x1008117d;
// pub const XF86MediaSelectTape: u32 = 0x10081180;
// pub const XF86MediaSelectTeletext: u32 = 0x10081184;
// pub const XF86MediaSelectTuner: u32 = 0x10081182;
// pub const XF86MediaSelectTV: u32 = 0x10081179;
// pub const XF86MediaSelectVCR: u32 = 0x1008117b;
// pub const XF86MediaSelectVCRPlus: u32 = 0x1008117c;
// pub const XF86MediaTitleMenu: u32 = 0x10081171;
// pub const XF86MediaTopMenu: u32 = 0x1008126b;
// pub const XF86Meeting: u32 = 0x1008ff63;
// pub const XF86Memo: u32 = 0x1008ff1e;
// pub const XF86MenuKB: u32 = 0x1008ff65;
// pub const XF86MenuPB: u32 = 0x1008ff66;
// pub const XF86Messenger: u32 = 0x1008ff8e;
// pub const XF86ModeLock: u32 = 0x1008ff01;
// pub const XF86MonBrightnessAuto: u32 = 0x100810f4;
// pub const XF86MonBrightnessCycle: u32 = 0x1008ff07;
// pub const XF86MonBrightnessDown: u32 = 0x1008ff03;
// pub const XF86MonBrightnessUp: u32 = 0x1008ff02;
// pub const XF86Music: u32 = 0x1008ff92;
// pub const XF86MyComputer: u32 = 0x1008ff33;
// pub const XF86MySites: u32 = 0x1008ff67;
// pub const XF86NavChart: u32 = 0x10081280;
// pub const XF86NavInfo: u32 = 0x10081288;
// pub const XF86New: u32 = 0x1008ff68;
// pub const XF86News: u32 = 0x1008ff69;
// pub const XF86Next_VMode: u32 = 0x1008fe22;
// pub const XF86NextElement: u32 = 0x1008127b;
// pub const XF86NextFavorite: u32 = 0x10081270;
// pub const XF86NotificationCenter: u32 = 0x100811bc;
// pub const XF86NumberEntryMode: u32 = 0x1008119d;
// pub const XF86Numeric0: u32 = 0x10081200;
// pub const XF86Numeric1: u32 = 0x10081201;
// pub const XF86Numeric11: u32 = 0x1008126c;
// pub const XF86Numeric12: u32 = 0x1008126d;
// pub const XF86Numeric2: u32 = 0x10081202;
// pub const XF86Numeric3: u32 = 0x10081203;
// pub const XF86Numeric4: u32 = 0x10081204;
// pub const XF86Numeric5: u32 = 0x10081205;
// pub const XF86Numeric6: u32 = 0x10081206;
// pub const XF86Numeric7: u32 = 0x10081207;
// pub const XF86Numeric8: u32 = 0x10081208;
// pub const XF86Numeric9: u32 = 0x10081209;
// pub const XF86NumericA: u32 = 0x1008120c;
// pub const XF86NumericB: u32 = 0x1008120d;
// pub const XF86NumericC: u32 = 0x1008120e;
// pub const XF86NumericD: u32 = 0x1008120f;
// pub const XF86NumericPound: u32 = 0x1008120b;
// pub const XF86NumericStar: u32 = 0x1008120a;
// pub const XF86OfficeHome: u32 = 0x1008ff6a;
// pub const XF86OK: u32 = 0x10081160;
// pub const XF86OnScreenKeyboard: u32 = 0x10081278;
// pub const XF86Open: u32 = 0x1008ff6b;
// pub const XF86OpenURL: u32 = 0x1008ff38;
// pub const XF86Option: u32 = 0x1008ff6c;
// pub const XF86Paste: u32 = 0x1008ff6d;
// pub const XF86PauseRecord: u32 = 0x10081272;
// pub const XF86PerformanceMode: u32 = 0x100812bd;
// pub const XF86Phone: u32 = 0x1008ff6e;
// pub const XF86PickupPhone: u32 = 0x100811bd;
// pub const XF86Pictures: u32 = 0x1008ff91;
// pub const XF86PowerDown: u32 = 0x1008ff21;
// pub const XF86PowerOff: u32 = 0x1008ff2a;
// pub const XF86Presentation: u32 = 0x100811a9;
// pub const XF86Prev_VMode: u32 = 0x1008fe23;
// pub const XF86PreviousElement: u32 = 0x1008127c;
// pub const XF86PrivacyScreenToggle: u32 = 0x10081279;
// pub const XF86Q: u32 = 0x1008ff70;
// pub const XF86RadarOverlay: u32 = 0x10081284;
// pub const XF86Red: u32 = 0x1008ffa3;
// pub const XF86Refresh: u32 = 0x1008ff29;
// pub const XF86RefreshRateToggle: u32 = 0x10081232;
// pub const XF86Reload: u32 = 0x1008ff73;
// pub const XF86Reply: u32 = 0x1008ff72;
// pub const XF86RFKill: u32 = 0x1008ffb5;
// pub const XF86RightDown: u32 = 0x10081267;
// pub const XF86RightUp: u32 = 0x10081266;
// pub const XF86RockerDown: u32 = 0x1008ff24;
// pub const XF86RockerEnter: u32 = 0x1008ff25;
// pub const XF86RockerUp: u32 = 0x1008ff23;
// pub const XF86RootMenu: u32 = 0x1008126a;
// pub const XF86RotateWindows: u32 = 0x1008ff74;
// pub const XF86RotationKB: u32 = 0x1008ff76;
// pub const XF86RotationLockToggle: u32 = 0x1008ffb7;
// pub const XF86RotationPB: u32 = 0x1008ff75;
// pub const XF86Save: u32 = 0x1008ff77;
// pub const XF86Screensaver: u32 = 0x10081245;
// pub const XF86ScreenSaver: u32 = 0x1008ff2d;
// pub const XF86ScrollClick: u32 = 0x1008ff7a;
// pub const XF86ScrollDown: u32 = 0x1008ff79;
// pub const XF86ScrollUp: u32 = 0x1008ff78;
// pub const XF86Search: u32 = 0x1008ff1b;
// pub const XF86Select: u32 = 0x1008ffa0;
// pub const XF86SelectiveScreenshot: u32 = 0x1008127a;
// pub const XF86Send: u32 = 0x1008ff7b;
// pub const XF86Shop: u32 = 0x1008ff36;
// pub const XF86SidevuSonar: u32 = 0x10081287;
// pub const XF86SingleRangeRadar: u32 = 0x10081282;
// pub const XF86Sleep: u32 = 0x1008ff2f;
// pub const XF86SlowReverse: u32 = 0x10081276;
// pub const XF86Sos: u32 = 0x1008127f;
// pub const XF86Spell: u32 = 0x1008ff7c;
// pub const XF86SpellCheck: u32 = 0x100811b0;
// pub const XF86SplitScreen: u32 = 0x1008ff7d;
// pub const XF86Sport: u32 = 0x100810dc;
// pub const XF86Standby: u32 = 0x1008ff10;
// pub const XF86Start: u32 = 0x1008ff1a;
// pub const XF86Stop: u32 = 0x1008ff28;
// pub const XF86StopRecord: u32 = 0x10081271;
// pub const XF86Subtitle: u32 = 0x1008ff9a;
// pub const XF86Support: u32 = 0x1008ff7e;
// pub const XF86Suspend: u32 = 0x1008ffa7;
pub const XF86Switch_VT_1: u32 = 0x1008fe01;
// pub const XF86Switch_VT_10: u32 = 0x1008fe0a;
// pub const XF86Switch_VT_11: u32 = 0x1008fe0b;
pub const XF86Switch_VT_12: u32 = 0x1008fe0c;
// pub const XF86Switch_VT_2: u32 = 0x1008fe02;
// pub const XF86Switch_VT_3: u32 = 0x1008fe03;
// pub const XF86Switch_VT_4: u32 = 0x1008fe04;
// pub const XF86Switch_VT_5: u32 = 0x1008fe05;
// pub const XF86Switch_VT_6: u32 = 0x1008fe06;
// pub const XF86Switch_VT_7: u32 = 0x1008fe07;
// pub const XF86Switch_VT_8: u32 = 0x1008fe08;
// pub const XF86Switch_VT_9: u32 = 0x1008fe09;
// pub const XF86Taskmanager: u32 = 0x10081241;
// pub const XF86TaskPane: u32 = 0x1008ff7f;
// pub const XF86Terminal: u32 = 0x1008ff80;
// pub const XF86Time: u32 = 0x1008ff9f;
// pub const XF86ToDoList: u32 = 0x1008ff1f;
// pub const XF86Tools: u32 = 0x1008ff81;
// pub const XF86TopMenu: u32 = 0x1008ffa2;
// pub const XF86TouchpadOff: u32 = 0x1008ffb1;
// pub const XF86TouchpadOn: u32 = 0x1008ffb0;
// pub const XF86TouchpadToggle: u32 = 0x1008ffa9;
// pub const XF86TraditionalSonar: u32 = 0x10081285;
// pub const XF86Travel: u32 = 0x1008ff82;
// pub const XF86Ungrab: u32 = 0x1008fe20;
// pub const XF86Unmute: u32 = 0x10081274;
// pub const XF86User1KB: u32 = 0x1008ff85;
// pub const XF86User2KB: u32 = 0x1008ff86;
// pub const XF86UserPB: u32 = 0x1008ff84;
// pub const XF86UWB: u32 = 0x1008ff96;
// pub const XF86VendorHome: u32 = 0x1008ff34;
// pub const XF86VendorLogo: u32 = 0x10081168;
// pub const XF86Video: u32 = 0x1008ff87;
// pub const XF86VideoPhone: u32 = 0x100811a0;
// pub const XF86View: u32 = 0x1008ffa1;
// pub const XF86VOD: u32 = 0x10081273;
// pub const XF86VoiceCommand: u32 = 0x10081246;
// pub const XF86Voicemail: u32 = 0x100811ac;
// pub const XF86WakeUp: u32 = 0x1008ff2b;
// pub const XF86WebCam: u32 = 0x1008ff8f;
// pub const XF86WheelButton: u32 = 0x1008ff88;
// pub const XF86WLAN: u32 = 0x1008ff95;
// pub const XF86Word: u32 = 0x1008ff89;
// pub const XF86WPSButton: u32 = 0x10081211;
// pub const XF86WWAN: u32 = 0x1008ffb4;
// pub const XF86WWW: u32 = 0x1008ff2e;
// pub const XF86Xfer: u32 = 0x1008ff8a;
// pub const XF86Yellow: u32 = 0x1008ffa5;
// pub const XF86ZoomIn: u32 = 0x1008ff8b;
// pub const XF86ZoomOut: u32 = 0x1008ff8c;
// pub const XF86ZoomReset: u32 = 0x100811a4;
// pub const y: u32 = 0x79;
// pub const Y: u32 = 0x59;
// pub const yacute: u32 = 0xfd;
// pub const Yacute: u32 = 0xdd;
// pub const ybelowdot: u32 = 0x1001ef5;
// pub const Ybelowdot: u32 = 0x1001ef4;
// pub const ycircumflex: u32 = 0x1000177;
// pub const Ycircumflex: u32 = 0x1000176;
// pub const ydiaeresis: u32 = 0xff;
// pub const Ydiaeresis: u32 = 0x13be;
// pub const yen: u32 = 0xa5;
// pub const ygrave: u32 = 0x1001ef3;
// pub const Ygrave: u32 = 0x1001ef2;
// pub const yhook: u32 = 0x1001ef7;
// pub const Yhook: u32 = 0x1001ef6;
// pub const ytilde: u32 = 0x1001ef9;
// pub const Ytilde: u32 = 0x1001ef8;
// pub const z: u32 = 0x7a;
pub const Z: u32 = 0x5a;
// pub const zabovedot: u32 = 0x1bf;
// pub const Zabovedot: u32 = 0x1af;
// pub const zacute: u32 = 0x1bc;
// pub const Zacute: u32 = 0x1ac;
// pub const zcaron: u32 = 0x1be;
// pub const Zcaron: u32 = 0x1ae;
pub const Zen_Koho: u32 = 0xff3d;
pub const Zenkaku: u32 = 0xff28;
pub const Zenkaku_Hankaku: u32 = 0xff2a;
// pub const zerosubscript: u32 = 0x1002080;
// pub const zerosuperior: u32 = 0x1002070;
// pub const zstroke: u32 = 0x10001b6;
// pub const Zstroke: u32 = 0x10001b5;

// XFree86 vendor keysyms
pub const XF86_MonBrightnessUp: u32 = 0x1008FF02;
pub const XF86_MonBrightnessDown: u32 = 0x1008FF03;
pub const XF86_Standby: u32 = 0x1008FF10;
pub const XF86_AudioLowerVolume: u32 = 0x1008FF11;
pub const XF86_AudioMute: u32 = 0x1008FF12;
pub const XF86_AudioRaiseVolume: u32 = 0x1008FF13;
pub const XF86_AudioPlay: u32 = 0x1008FF14;
pub const XF86_AudioStop: u32 = 0x1008FF15;
pub const XF86_AudioPrev: u32 = 0x1008FF16;
pub const XF86_AudioNext: u32 = 0x1008FF17;
pub const XF86_HomePage: u32 = 0x1008FF18;
pub const XF86_Mail: u32 = 0x1008FF19;
pub const XF86_Search: u32 = 0x1008FF1B;
pub const XF86_AudioRecord: u32 = 0x1008FF1C;
pub const XF86_Calculator: u32 = 0x1008FF1D;
pub const XF86_Calendar: u32 = 0x1008FF20;
pub const XF86_PowerDown: u32 = 0x1008FF21;
pub const XF86_Back: u32 = 0x1008FF26;
pub const XF86_Forward: u32 = 0x1008FF27;
pub const XF86_Refresh: u32 = 0x1008FF29;
pub const XF86_PowerOff: u32 = 0x1008FF2A;
pub const XF86_WakeUp: u32 = 0x1008FF2B;
pub const XF86_Eject: u32 = 0x1008FF2C;
pub const XF86_ScreenSaver: u32 = 0x1008FF2D;
pub const XF86_WWW: u32 = 0x1008FF2E;
pub const XF86_Sleep: u32 = 0x1008FF2F;
pub const XF86_Favorites: u32 = 0x1008FF30;
pub const XF86_AudioPause: u32 = 0x1008FF31;
pub const XF86_MyComputer: u32 = 0x1008FF33;
pub const XF86_AudioRewind: u32 = 0x1008FF3E;
pub const XF86_Calculater: u32 = 0x1008FF54; // intentional typo from xkbcommon
pub const XF86_Close: u32 = 0x1008FF56;
pub const XF86_Copy: u32 = 0x1008FF57;
pub const XF86_Cut: u32 = 0x1008FF58;
pub const XF86_Excel: u32 = 0x1008FF5C;
pub const XF86_LogOff: u32 = 0x1008FF61;
pub const XF86_MySites: u32 = 0x1008FF67;
pub const XF86_New: u32 = 0x1008FF68;
pub const XF86_Open: u32 = 0x1008FF6B;
pub const XF86_Paste: u32 = 0x1008FF6D;
pub const XF86_Phone: u32 = 0x1008FF6E;
pub const XF86_Reply: u32 = 0x1008FF72;
pub const XF86_Reload: u32 = 0x1008FF73;
pub const XF86_Save: u32 = 0x1008FF77;
pub const XF86_Send: u32 = 0x1008FF7B;
pub const XF86_Spell: u32 = 0x1008FF7C;
pub const XF86_SplitScreen: u32 = 0x1008FF7D;
pub const XF86_Video: u32 = 0x1008FF87;
pub const XF86_Word: u32 = 0x1008FF89;
pub const XF86_ZoomIn: u32 = 0x1008FF8B;
pub const XF86_ZoomOut: u32 = 0x1008FF8C;
pub const XF86_WebCam: u32 = 0x1008FF8F;
pub const XF86_MailForward: u32 = 0x1008FF90;
pub const XF86_Music: u32 = 0x1008FF92;
pub const XF86_AudioForward: u32 = 0x1008FF97;
pub const XF86_AudioRandomPlay: u32 = 0x1008FF99;
pub const XF86_Subtitle: u32 = 0x1008FF9A;
pub const XF86_AudioCycleTrack: u32 = 0x1008FF9B;
pub const XF86_Suspend: u32 = 0x1008FFA7;
pub const XF86_Hibernate: u32 = 0x1008FFA8;
pub const XF86_Next_VMode: u32 = 0x1008FE22;

// Sun vendor keysyms
pub const SUN_Copy: u32 = 0x1005FF72;
pub const SUN_Open: u32 = 0x1005FF73;
pub const SUN_Paste: u32 = 0x1005FF74;
pub const SUN_Cut: u32 = 0x1005FF75;
pub const SUN_AudioLowerVolume: u32 = 0x1005FF04;
pub const SUN_AudioMute: u32 = 0x1005FF05;
pub const SUN_AudioRaiseVolume: u32 = 0x1005FF06;
pub const SUN_VideoLowerBrightness: u32 = 0x1005FF07;
pub const SUN_VideoRaiseBrightness: u32 = 0x1005FF08;

/// Look up the human-readable name of a keysym.
///
/// Returns `None` for unknown keysyms.
pub fn keysym_get_name(sym: u32) -> Option<&'static str> {
    // Sorted by keysym value for binary search
    KEYSYM_TO_NAME
        .binary_search_by_key(&sym, |&(ks, _)| ks)
        .ok()
        .map(|i| KEYSYM_TO_NAME[i].1)
}

/// Look up the keysym value from a human-readable name.
///
/// Delegates to xkb-core's keysym lookup (case-sensitive).
/// Returns `None` for unknown names.
pub fn keysym_from_name(name: &str) -> Option<u32> {
    let sym = xkb_core::xkb_keysym_from_name(name.as_bytes(), 0);
    if sym == 0 {
        None
    } else {
        Some(sym)
    }
}

/// Return the VT number (1–12) if this is a VT-switch keysym, or `None`.
pub fn vt_switch(sym: u32) -> Option<u32> {
    if sym >= XF86Switch_VT_1 && sym <= XF86Switch_VT_12 {
        Some(sym - XF86Switch_VT_1 + 1)
    } else {
        None
    }
}

static KEYSYM_TO_NAME: &[(u32, &str)] = &[
    (0x20, "space"),
    (0x21, "exclam"),
    (0x22, "quotedbl"),
    (0x23, "numbersign"),
    (0x24, "dollar"),
    (0x25, "percent"),
    (0x26, "ampersand"),
    (0x27, "apostrophe"),
    (0x27, "quoteright"),
    (0x28, "parenleft"),
    (0x29, "parenright"),
    (0x2a, "asterisk"),
    (0x2b, "plus"),
    (0x2c, "comma"),
    (0x2d, "minus"),
    (0x2e, "period"),
    (0x2f, "slash"),
    (0x30, "0"),
    (0x31, "1"),
    (0x32, "2"),
    (0x33, "3"),
    (0x34, "4"),
    (0x35, "5"),
    (0x36, "6"),
    (0x37, "7"),
    (0x38, "8"),
    (0x39, "9"),
    (0x3a, "colon"),
    (0x3b, "semicolon"),
    (0x3c, "less"),
    (0x3d, "equal"),
    (0x3e, "greater"),
    (0x3f, "question"),
    (0x40, "at"),
    (0x41, "A"),
    (0x42, "B"),
    (0x43, "C"),
    (0x44, "D"),
    (0x45, "E"),
    (0x46, "F"),
    (0x47, "G"),
    (0x48, "H"),
    (0x49, "I"),
    (0x4a, "J"),
    (0x4b, "K"),
    (0x4c, "L"),
    (0x4d, "M"),
    (0x4e, "N"),
    (0x4f, "O"),
    (0x50, "P"),
    (0x51, "Q"),
    (0x52, "R"),
    (0x53, "S"),
    (0x54, "T"),
    (0x55, "U"),
    (0x56, "V"),
    (0x57, "W"),
    (0x58, "X"),
    (0x59, "Y"),
    (0x5a, "Z"),
    (0x5b, "bracketleft"),
    (0x5c, "backslash"),
    (0x5d, "bracketright"),
    (0x5e, "asciicircum"),
    (0x5f, "underscore"),
    (0x60, "grave"),
    (0x60, "quoteleft"),
    (0x61, "a"),
    (0x62, "b"),
    (0x63, "c"),
    (0x64, "d"),
    (0x65, "e"),
    (0x66, "f"),
    (0x67, "g"),
    (0x68, "h"),
    (0x69, "i"),
    (0x6a, "j"),
    (0x6b, "k"),
    (0x6c, "l"),
    (0x6d, "m"),
    (0x6e, "n"),
    (0x6f, "o"),
    (0x70, "p"),
    (0x71, "q"),
    (0x72, "r"),
    (0x73, "s"),
    (0x74, "t"),
    (0x75, "u"),
    (0x76, "v"),
    (0x77, "w"),
    (0x78, "x"),
    (0x79, "y"),
    (0x7a, "z"),
    (0x7b, "braceleft"),
    (0x7c, "bar"),
    (0x7d, "braceright"),
    (0x7e, "asciitilde"),
    (0xa0, "nobreakspace"),
    (0xa1, "exclamdown"),
    (0xa2, "cent"),
    (0xa3, "sterling"),
    (0xa4, "currency"),
    (0xa5, "yen"),
    (0xa6, "brokenbar"),
    (0xa7, "section"),
    (0xa8, "diaeresis"),
    (0xa9, "copyright"),
    (0xaa, "ordfeminine"),
    (0xab, "guillemetleft"),
    (0xab, "guillemotleft"),
    (0xac, "notsign"),
    (0xad, "hyphen"),
    (0xae, "registered"),
    (0xaf, "macron"),
    (0xb0, "degree"),
    (0xb1, "plusminus"),
    (0xb2, "twosuperior"),
    (0xb3, "threesuperior"),
    (0xb4, "acute"),
    (0xb5, "mu"),
    (0xb6, "paragraph"),
    (0xb7, "periodcentered"),
    (0xb8, "cedilla"),
    (0xb9, "onesuperior"),
    (0xba, "masculine"),
    (0xba, "ordmasculine"),
    (0xbb, "guillemetright"),
    (0xbb, "guillemotright"),
    (0xbc, "onequarter"),
    (0xbd, "onehalf"),
    (0xbe, "threequarters"),
    (0xbf, "questiondown"),
    (0xc0, "Agrave"),
    (0xc1, "Aacute"),
    (0xc2, "Acircumflex"),
    (0xc3, "Atilde"),
    (0xc4, "Adiaeresis"),
    (0xc5, "Aring"),
    (0xc6, "AE"),
    (0xc7, "Ccedilla"),
    (0xc8, "Egrave"),
    (0xc9, "Eacute"),
    (0xca, "Ecircumflex"),
    (0xcb, "Ediaeresis"),
    (0xcc, "Igrave"),
    (0xcd, "Iacute"),
    (0xce, "Icircumflex"),
    (0xcf, "Idiaeresis"),
    (0xd0, "Eth"),
    (0xd0, "ETH"),
    (0xd1, "Ntilde"),
    (0xd2, "Ograve"),
    (0xd3, "Oacute"),
    (0xd4, "Ocircumflex"),
    (0xd5, "Otilde"),
    (0xd6, "Odiaeresis"),
    (0xd7, "multiply"),
    (0xd8, "Ooblique"),
    (0xd8, "Oslash"),
    (0xd9, "Ugrave"),
    (0xda, "Uacute"),
    (0xdb, "Ucircumflex"),
    (0xdc, "Udiaeresis"),
    (0xdd, "Yacute"),
    (0xde, "Thorn"),
    (0xde, "THORN"),
    (0xdf, "ssharp"),
    (0xe0, "agrave"),
    (0xe1, "aacute"),
    (0xe2, "acircumflex"),
    (0xe3, "atilde"),
    (0xe4, "adiaeresis"),
    (0xe5, "aring"),
    (0xe6, "ae"),
    (0xe7, "ccedilla"),
    (0xe8, "egrave"),
    (0xe9, "eacute"),
    (0xea, "ecircumflex"),
    (0xeb, "ediaeresis"),
    (0xec, "igrave"),
    (0xed, "iacute"),
    (0xee, "icircumflex"),
    (0xef, "idiaeresis"),
    (0xf0, "eth"),
    (0xf1, "ntilde"),
    (0xf2, "ograve"),
    (0xf3, "oacute"),
    (0xf4, "ocircumflex"),
    (0xf5, "otilde"),
    (0xf6, "odiaeresis"),
    (0xf7, "division"),
    (0xf8, "ooblique"),
    (0xf8, "oslash"),
    (0xf9, "ugrave"),
    (0xfa, "uacute"),
    (0xfb, "ucircumflex"),
    (0xfc, "udiaeresis"),
    (0xfd, "yacute"),
    (0xfe, "thorn"),
    (0xff, "ydiaeresis"),
    (0x1a1, "Aogonek"),
    (0x1a2, "breve"),
    (0x1a3, "Lstroke"),
    (0x1a5, "Lcaron"),
    (0x1a6, "Sacute"),
    (0x1a9, "Scaron"),
    (0x1aa, "Scedilla"),
    (0x1ab, "Tcaron"),
    (0x1ac, "Zacute"),
    (0x1ae, "Zcaron"),
    (0x1af, "Zabovedot"),
    (0x1b1, "aogonek"),
    (0x1b2, "ogonek"),
    (0x1b3, "lstroke"),
    (0x1b5, "lcaron"),
    (0x1b6, "sacute"),
    (0x1b7, "caron"),
    (0x1b9, "scaron"),
    (0x1ba, "scedilla"),
    (0x1bb, "tcaron"),
    (0x1bc, "zacute"),
    (0x1bd, "doubleacute"),
    (0x1be, "zcaron"),
    (0x1bf, "zabovedot"),
    (0x1c0, "Racute"),
    (0x1c3, "Abreve"),
    (0x1c5, "Lacute"),
    (0x1c6, "Cacute"),
    (0x1c8, "Ccaron"),
    (0x1ca, "Eogonek"),
    (0x1cc, "Ecaron"),
    (0x1cf, "Dcaron"),
    (0x1d0, "Dstroke"),
    (0x1d1, "Nacute"),
    (0x1d2, "Ncaron"),
    (0x1d5, "Odoubleacute"),
    (0x1d8, "Rcaron"),
    (0x1d9, "Uring"),
    (0x1db, "Udoubleacute"),
    (0x1de, "Tcedilla"),
    (0x1e0, "racute"),
    (0x1e3, "abreve"),
    (0x1e5, "lacute"),
    (0x1e6, "cacute"),
    (0x1e8, "ccaron"),
    (0x1ea, "eogonek"),
    (0x1ec, "ecaron"),
    (0x1ef, "dcaron"),
    (0x1f0, "dstroke"),
    (0x1f1, "nacute"),
    (0x1f2, "ncaron"),
    (0x1f5, "odoubleacute"),
    (0x1f8, "rcaron"),
    (0x1f9, "uring"),
    (0x1fb, "udoubleacute"),
    (0x1fe, "tcedilla"),
    (0x1ff, "abovedot"),
    (0x2a1, "Hstroke"),
    (0x2a6, "Hcircumflex"),
    (0x2a9, "Iabovedot"),
    (0x2ab, "Gbreve"),
    (0x2ac, "Jcircumflex"),
    (0x2b1, "hstroke"),
    (0x2b6, "hcircumflex"),
    (0x2b9, "idotless"),
    (0x2bb, "gbreve"),
    (0x2bc, "jcircumflex"),
    (0x2c5, "Cabovedot"),
    (0x2c6, "Ccircumflex"),
    (0x2d5, "Gabovedot"),
    (0x2d8, "Gcircumflex"),
    (0x2dd, "Ubreve"),
    (0x2de, "Scircumflex"),
    (0x2e5, "cabovedot"),
    (0x2e6, "ccircumflex"),
    (0x2f5, "gabovedot"),
    (0x2f8, "gcircumflex"),
    (0x2fd, "ubreve"),
    (0x2fe, "scircumflex"),
    (0x3a2, "kappa"),
    (0x3a2, "kra"),
    (0x3a3, "Rcedilla"),
    (0x3a5, "Itilde"),
    (0x3a6, "Lcedilla"),
    (0x3aa, "Emacron"),
    (0x3ab, "Gcedilla"),
    (0x3ac, "Tslash"),
    (0x3b3, "rcedilla"),
    (0x3b5, "itilde"),
    (0x3b6, "lcedilla"),
    (0x3ba, "emacron"),
    (0x3bb, "gcedilla"),
    (0x3bc, "tslash"),
    (0x3bd, "ENG"),
    (0x3bf, "eng"),
    (0x3c0, "Amacron"),
    (0x3c7, "Iogonek"),
    (0x3cc, "Eabovedot"),
    (0x3cf, "Imacron"),
    (0x3d1, "Ncedilla"),
    (0x3d2, "Omacron"),
    (0x3d3, "Kcedilla"),
    (0x3d9, "Uogonek"),
    (0x3dd, "Utilde"),
    (0x3de, "Umacron"),
    (0x3e0, "amacron"),
    (0x3e7, "iogonek"),
    (0x3ec, "eabovedot"),
    (0x3ef, "imacron"),
    (0x3f1, "ncedilla"),
    (0x3f2, "omacron"),
    (0x3f3, "kcedilla"),
    (0x3f9, "uogonek"),
    (0x3fd, "utilde"),
    (0x3fe, "umacron"),
    (0x47e, "overline"),
    (0x4a1, "kana_fullstop"),
    (0x4a2, "kana_openingbracket"),
    (0x4a3, "kana_closingbracket"),
    (0x4a4, "kana_comma"),
    (0x4a5, "kana_conjunctive"),
    (0x4a5, "kana_middledot"),
    (0x4a6, "kana_WO"),
    (0x4a7, "kana_a"),
    (0x4a8, "kana_i"),
    (0x4a9, "kana_u"),
    (0x4aa, "kana_e"),
    (0x4ab, "kana_o"),
    (0x4ac, "kana_ya"),
    (0x4ad, "kana_yu"),
    (0x4ae, "kana_yo"),
    (0x4af, "kana_tsu"),
    (0x4af, "kana_tu"),
    (0x4b0, "prolongedsound"),
    (0x4b1, "kana_A"),
    (0x4b2, "kana_I"),
    (0x4b3, "kana_U"),
    (0x4b4, "kana_E"),
    (0x4b5, "kana_O"),
    (0x4b6, "kana_KA"),
    (0x4b7, "kana_KI"),
    (0x4b8, "kana_KU"),
    (0x4b9, "kana_KE"),
    (0x4ba, "kana_KO"),
    (0x4bb, "kana_SA"),
    (0x4bc, "kana_SHI"),
    (0x4bd, "kana_SU"),
    (0x4be, "kana_SE"),
    (0x4bf, "kana_SO"),
    (0x4c0, "kana_TA"),
    (0x4c1, "kana_CHI"),
    (0x4c1, "kana_TI"),
    (0x4c2, "kana_TSU"),
    (0x4c2, "kana_TU"),
    (0x4c3, "kana_TE"),
    (0x4c4, "kana_TO"),
    (0x4c5, "kana_NA"),
    (0x4c6, "kana_NI"),
    (0x4c7, "kana_NU"),
    (0x4c8, "kana_NE"),
    (0x4c9, "kana_NO"),
    (0x4ca, "kana_HA"),
    (0x4cb, "kana_HI"),
    (0x4cc, "kana_FU"),
    (0x4cc, "kana_HU"),
    (0x4cd, "kana_HE"),
    (0x4ce, "kana_HO"),
    (0x4cf, "kana_MA"),
    (0x4d0, "kana_MI"),
    (0x4d1, "kana_MU"),
    (0x4d2, "kana_ME"),
    (0x4d3, "kana_MO"),
    (0x4d4, "kana_YA"),
    (0x4d5, "kana_YU"),
    (0x4d6, "kana_YO"),
    (0x4d7, "kana_RA"),
    (0x4d8, "kana_RI"),
    (0x4d9, "kana_RU"),
    (0x4da, "kana_RE"),
    (0x4db, "kana_RO"),
    (0x4dc, "kana_WA"),
    (0x4dd, "kana_N"),
    (0x4de, "voicedsound"),
    (0x4df, "semivoicedsound"),
    (0x5ac, "Arabic_comma"),
    (0x5bb, "Arabic_semicolon"),
    (0x5bf, "Arabic_question_mark"),
    (0x5c1, "Arabic_hamza"),
    (0x5c2, "Arabic_maddaonalef"),
    (0x5c3, "Arabic_hamzaonalef"),
    (0x5c4, "Arabic_hamzaonwaw"),
    (0x5c5, "Arabic_hamzaunderalef"),
    (0x5c6, "Arabic_hamzaonyeh"),
    (0x5c7, "Arabic_alef"),
    (0x5c8, "Arabic_beh"),
    (0x5c9, "Arabic_tehmarbuta"),
    (0x5ca, "Arabic_teh"),
    (0x5cb, "Arabic_theh"),
    (0x5cc, "Arabic_jeem"),
    (0x5cd, "Arabic_hah"),
    (0x5ce, "Arabic_khah"),
    (0x5cf, "Arabic_dal"),
    (0x5d0, "Arabic_thal"),
    (0x5d1, "Arabic_ra"),
    (0x5d2, "Arabic_zain"),
    (0x5d3, "Arabic_seen"),
    (0x5d4, "Arabic_sheen"),
    (0x5d5, "Arabic_sad"),
    (0x5d6, "Arabic_dad"),
    (0x5d7, "Arabic_tah"),
    (0x5d8, "Arabic_zah"),
    (0x5d9, "Arabic_ain"),
    (0x5da, "Arabic_ghain"),
    (0x5e0, "Arabic_tatweel"),
    (0x5e1, "Arabic_feh"),
    (0x5e2, "Arabic_qaf"),
    (0x5e3, "Arabic_kaf"),
    (0x5e4, "Arabic_lam"),
    (0x5e5, "Arabic_meem"),
    (0x5e6, "Arabic_noon"),
    (0x5e7, "Arabic_ha"),
    (0x5e7, "Arabic_heh"),
    (0x5e8, "Arabic_waw"),
    (0x5e9, "Arabic_alefmaksura"),
    (0x5ea, "Arabic_yeh"),
    (0x5eb, "Arabic_fathatan"),
    (0x5ec, "Arabic_dammatan"),
    (0x5ed, "Arabic_kasratan"),
    (0x5ee, "Arabic_fatha"),
    (0x5ef, "Arabic_damma"),
    (0x5f0, "Arabic_kasra"),
    (0x5f1, "Arabic_shadda"),
    (0x5f2, "Arabic_sukun"),
    (0x6a1, "Serbian_dje"),
    (0x6a2, "Macedonia_gje"),
    (0x6a3, "Cyrillic_io"),
    (0x6a4, "Ukrainian_ie"),
    (0x6a4, "Ukranian_je"),
    (0x6a5, "Macedonia_dse"),
    (0x6a6, "Ukrainian_i"),
    (0x6a6, "Ukranian_i"),
    (0x6a7, "Ukrainian_yi"),
    (0x6a7, "Ukranian_yi"),
    (0x6a8, "Cyrillic_je"),
    (0x6a8, "Serbian_je"),
    (0x6a9, "Cyrillic_lje"),
    (0x6a9, "Serbian_lje"),
    (0x6aa, "Cyrillic_nje"),
    (0x6aa, "Serbian_nje"),
    (0x6ab, "Serbian_tshe"),
    (0x6ac, "Macedonia_kje"),
    (0x6ad, "Ukrainian_ghe_with_upturn"),
    (0x6ae, "Byelorussian_shortu"),
    (0x6af, "Cyrillic_dzhe"),
    (0x6af, "Serbian_dze"),
    (0x6b0, "numerosign"),
    (0x6b1, "Serbian_DJE"),
    (0x6b2, "Macedonia_GJE"),
    (0x6b3, "Cyrillic_IO"),
    (0x6b4, "Ukrainian_IE"),
    (0x6b4, "Ukranian_JE"),
    (0x6b5, "Macedonia_DSE"),
    (0x6b6, "Ukrainian_I"),
    (0x6b6, "Ukranian_I"),
    (0x6b7, "Ukrainian_YI"),
    (0x6b7, "Ukranian_YI"),
    (0x6b8, "Cyrillic_JE"),
    (0x6b8, "Serbian_JE"),
    (0x6b9, "Cyrillic_LJE"),
    (0x6b9, "Serbian_LJE"),
    (0x6ba, "Cyrillic_NJE"),
    (0x6ba, "Serbian_NJE"),
    (0x6bb, "Serbian_TSHE"),
    (0x6bc, "Macedonia_KJE"),
    (0x6bd, "Ukrainian_GHE_WITH_UPTURN"),
    (0x6be, "Byelorussian_SHORTU"),
    (0x6bf, "Cyrillic_DZHE"),
    (0x6bf, "Serbian_DZE"),
    (0x6c0, "Cyrillic_yu"),
    (0x6c1, "Cyrillic_a"),
    (0x6c2, "Cyrillic_be"),
    (0x6c3, "Cyrillic_tse"),
    (0x6c4, "Cyrillic_de"),
    (0x6c5, "Cyrillic_ie"),
    (0x6c6, "Cyrillic_ef"),
    (0x6c7, "Cyrillic_ghe"),
    (0x6c8, "Cyrillic_ha"),
    (0x6c9, "Cyrillic_i"),
    (0x6ca, "Cyrillic_shorti"),
    (0x6cb, "Cyrillic_ka"),
    (0x6cc, "Cyrillic_el"),
    (0x6cd, "Cyrillic_em"),
    (0x6ce, "Cyrillic_en"),
    (0x6cf, "Cyrillic_o"),
    (0x6d0, "Cyrillic_pe"),
    (0x6d1, "Cyrillic_ya"),
    (0x6d2, "Cyrillic_er"),
    (0x6d3, "Cyrillic_es"),
    (0x6d4, "Cyrillic_te"),
    (0x6d5, "Cyrillic_u"),
    (0x6d6, "Cyrillic_zhe"),
    (0x6d7, "Cyrillic_ve"),
    (0x6d8, "Cyrillic_softsign"),
    (0x6d9, "Cyrillic_yeru"),
    (0x6da, "Cyrillic_ze"),
    (0x6db, "Cyrillic_sha"),
    (0x6dc, "Cyrillic_e"),
    (0x6dd, "Cyrillic_shcha"),
    (0x6de, "Cyrillic_che"),
    (0x6df, "Cyrillic_hardsign"),
    (0x6e0, "Cyrillic_YU"),
    (0x6e1, "Cyrillic_A"),
    (0x6e2, "Cyrillic_BE"),
    (0x6e3, "Cyrillic_TSE"),
    (0x6e4, "Cyrillic_DE"),
    (0x6e5, "Cyrillic_IE"),
    (0x6e6, "Cyrillic_EF"),
    (0x6e7, "Cyrillic_GHE"),
    (0x6e8, "Cyrillic_HA"),
    (0x6e9, "Cyrillic_I"),
    (0x6ea, "Cyrillic_SHORTI"),
    (0x6eb, "Cyrillic_KA"),
    (0x6ec, "Cyrillic_EL"),
    (0x6ed, "Cyrillic_EM"),
    (0x6ee, "Cyrillic_EN"),
    (0x6ef, "Cyrillic_O"),
    (0x6f0, "Cyrillic_PE"),
    (0x6f1, "Cyrillic_YA"),
    (0x6f2, "Cyrillic_ER"),
    (0x6f3, "Cyrillic_ES"),
    (0x6f4, "Cyrillic_TE"),
    (0x6f5, "Cyrillic_U"),
    (0x6f6, "Cyrillic_ZHE"),
    (0x6f7, "Cyrillic_VE"),
    (0x6f8, "Cyrillic_SOFTSIGN"),
    (0x6f9, "Cyrillic_YERU"),
    (0x6fa, "Cyrillic_ZE"),
    (0x6fb, "Cyrillic_SHA"),
    (0x6fc, "Cyrillic_E"),
    (0x6fd, "Cyrillic_SHCHA"),
    (0x6fe, "Cyrillic_CHE"),
    (0x6ff, "Cyrillic_HARDSIGN"),
    (0x7a1, "Greek_ALPHAaccent"),
    (0x7a2, "Greek_EPSILONaccent"),
    (0x7a3, "Greek_ETAaccent"),
    (0x7a4, "Greek_IOTAaccent"),
    (0x7a5, "Greek_IOTAdiaeresis"),
    (0x7a5, "Greek_IOTAdieresis"),
    (0x7a7, "Greek_OMICRONaccent"),
    (0x7a8, "Greek_UPSILONaccent"),
    (0x7a9, "Greek_UPSILONdieresis"),
    (0x7ab, "Greek_OMEGAaccent"),
    (0x7ae, "Greek_accentdieresis"),
    (0x7af, "Greek_horizbar"),
    (0x7b1, "Greek_alphaaccent"),
    (0x7b2, "Greek_epsilonaccent"),
    (0x7b3, "Greek_etaaccent"),
    (0x7b4, "Greek_iotaaccent"),
    (0x7b5, "Greek_iotadieresis"),
    (0x7b6, "Greek_iotaaccentdieresis"),
    (0x7b7, "Greek_omicronaccent"),
    (0x7b8, "Greek_upsilonaccent"),
    (0x7b9, "Greek_upsilondieresis"),
    (0x7ba, "Greek_upsilonaccentdieresis"),
    (0x7bb, "Greek_omegaaccent"),
    (0x7c1, "Greek_ALPHA"),
    (0x7c2, "Greek_BETA"),
    (0x7c3, "Greek_GAMMA"),
    (0x7c4, "Greek_DELTA"),
    (0x7c5, "Greek_EPSILON"),
    (0x7c6, "Greek_ZETA"),
    (0x7c7, "Greek_ETA"),
    (0x7c8, "Greek_THETA"),
    (0x7c9, "Greek_IOTA"),
    (0x7ca, "Greek_KAPPA"),
    (0x7cb, "Greek_LAMBDA"),
    (0x7cb, "Greek_LAMDA"),
    (0x7cc, "Greek_MU"),
    (0x7cd, "Greek_NU"),
    (0x7ce, "Greek_XI"),
    (0x7cf, "Greek_OMICRON"),
    (0x7d0, "Greek_PI"),
    (0x7d1, "Greek_RHO"),
    (0x7d2, "Greek_SIGMA"),
    (0x7d4, "Greek_TAU"),
    (0x7d5, "Greek_UPSILON"),
    (0x7d6, "Greek_PHI"),
    (0x7d7, "Greek_CHI"),
    (0x7d8, "Greek_PSI"),
    (0x7d9, "Greek_OMEGA"),
    (0x7e1, "Greek_alpha"),
    (0x7e2, "Greek_beta"),
    (0x7e3, "Greek_gamma"),
    (0x7e4, "Greek_delta"),
    (0x7e5, "Greek_epsilon"),
    (0x7e6, "Greek_zeta"),
    (0x7e7, "Greek_eta"),
    (0x7e8, "Greek_theta"),
    (0x7e9, "Greek_iota"),
    (0x7ea, "Greek_kappa"),
    (0x7eb, "Greek_lambda"),
    (0x7eb, "Greek_lamda"),
    (0x7ec, "Greek_mu"),
    (0x7ed, "Greek_nu"),
    (0x7ee, "Greek_xi"),
    (0x7ef, "Greek_omicron"),
    (0x7f0, "Greek_pi"),
    (0x7f1, "Greek_rho"),
    (0x7f2, "Greek_sigma"),
    (0x7f3, "Greek_finalsmallsigma"),
    (0x7f4, "Greek_tau"),
    (0x7f5, "Greek_upsilon"),
    (0x7f6, "Greek_phi"),
    (0x7f7, "Greek_chi"),
    (0x7f8, "Greek_psi"),
    (0x7f9, "Greek_omega"),
    (0x8a1, "leftradical"),
    (0x8a2, "topleftradical"),
    (0x8a3, "horizconnector"),
    (0x8a4, "topintegral"),
    (0x8a5, "botintegral"),
    (0x8a6, "vertconnector"),
    (0x8a7, "topleftsqbracket"),
    (0x8a8, "botleftsqbracket"),
    (0x8a9, "toprightsqbracket"),
    (0x8aa, "botrightsqbracket"),
    (0x8ab, "topleftparens"),
    (0x8ac, "botleftparens"),
    (0x8ad, "toprightparens"),
    (0x8ae, "botrightparens"),
    (0x8af, "leftmiddlecurlybrace"),
    (0x8b0, "rightmiddlecurlybrace"),
    (0x8b1, "topleftsummation"),
    (0x8b2, "botleftsummation"),
    (0x8b3, "topvertsummationconnector"),
    (0x8b4, "botvertsummationconnector"),
    (0x8b5, "toprightsummation"),
    (0x8b6, "botrightsummation"),
    (0x8b7, "rightmiddlesummation"),
    (0x8bc, "lessthanequal"),
    (0x8bd, "notequal"),
    (0x8be, "greaterthanequal"),
    (0x8bf, "integral"),
    (0x8c0, "therefore"),
    (0x8c1, "variation"),
    (0x8c2, "infinity"),
    (0x8c5, "nabla"),
    (0x8c8, "approximate"),
    (0x8c9, "similarequal"),
    (0x8cd, "ifonlyif"),
    (0x8ce, "implies"),
    (0x8cf, "identical"),
    (0x8d6, "radical"),
    (0x8da, "includedin"),
    (0x8db, "includes"),
    (0x8dc, "intersection"),
    (0x8dd, "union"),
    (0x8de, "logicaland"),
    (0x8df, "logicalor"),
    (0x8ef, "partialderivative"),
    (0x8f6, "function"),
    (0x8fb, "leftarrow"),
    (0x8fc, "uparrow"),
    (0x8fd, "rightarrow"),
    (0x8fe, "downarrow"),
    (0x9df, "blank"),
    (0x9e0, "soliddiamond"),
    (0x9e1, "checkerboard"),
    (0x9e2, "ht"),
    (0x9e3, "ff"),
    (0x9e4, "cr"),
    (0x9e5, "lf"),
    (0x9e8, "nl"),
    (0x9e9, "vt"),
    (0x9ea, "lowrightcorner"),
    (0x9eb, "uprightcorner"),
    (0x9ec, "upleftcorner"),
    (0x9ed, "lowleftcorner"),
    (0x9ee, "crossinglines"),
    (0x9ef, "horizlinescan1"),
    (0x9f0, "horizlinescan3"),
    (0x9f1, "horizlinescan5"),
    (0x9f2, "horizlinescan7"),
    (0x9f3, "horizlinescan9"),
    (0x9f4, "leftt"),
    (0x9f5, "rightt"),
    (0x9f6, "bott"),
    (0x9f7, "topt"),
    (0x9f8, "vertbar"),
    (0xaa1, "emspace"),
    (0xaa2, "enspace"),
    (0xaa3, "em3space"),
    (0xaa4, "em4space"),
    (0xaa5, "digitspace"),
    (0xaa6, "punctspace"),
    (0xaa7, "thinspace"),
    (0xaa8, "hairspace"),
    (0xaa9, "emdash"),
    (0xaaa, "endash"),
    (0xaac, "signifblank"),
    (0xaae, "ellipsis"),
    (0xaaf, "doubbaselinedot"),
    (0xab0, "onethird"),
    (0xab1, "twothirds"),
    (0xab2, "onefifth"),
    (0xab3, "twofifths"),
    (0xab4, "threefifths"),
    (0xab5, "fourfifths"),
    (0xab6, "onesixth"),
    (0xab7, "fivesixths"),
    (0xab8, "careof"),
    (0xabb, "figdash"),
    (0xabc, "leftanglebracket"),
    (0xabd, "decimalpoint"),
    (0xabe, "rightanglebracket"),
    (0xabf, "marker"),
    (0xac3, "oneeighth"),
    (0xac4, "threeeighths"),
    (0xac5, "fiveeighths"),
    (0xac6, "seveneighths"),
    (0xac9, "trademark"),
    (0xaca, "signaturemark"),
    (0xacb, "trademarkincircle"),
    (0xacc, "leftopentriangle"),
    (0xacd, "rightopentriangle"),
    (0xace, "emopencircle"),
    (0xacf, "emopenrectangle"),
    (0xad0, "leftsinglequotemark"),
    (0xad1, "rightsinglequotemark"),
    (0xad2, "leftdoublequotemark"),
    (0xad3, "rightdoublequotemark"),
    (0xad4, "prescription"),
    (0xad5, "permille"),
    (0xad6, "minutes"),
    (0xad7, "seconds"),
    (0xad9, "latincross"),
    (0xada, "hexagram"),
    (0xadb, "filledrectbullet"),
    (0xadc, "filledlefttribullet"),
    (0xadd, "filledrighttribullet"),
    (0xade, "emfilledcircle"),
    (0xadf, "emfilledrect"),
    (0xae0, "enopencircbullet"),
    (0xae1, "enopensquarebullet"),
    (0xae2, "openrectbullet"),
    (0xae3, "opentribulletup"),
    (0xae4, "opentribulletdown"),
    (0xae5, "openstar"),
    (0xae6, "enfilledcircbullet"),
    (0xae7, "enfilledsqbullet"),
    (0xae8, "filledtribulletup"),
    (0xae9, "filledtribulletdown"),
    (0xaea, "leftpointer"),
    (0xaeb, "rightpointer"),
    (0xaec, "club"),
    (0xaed, "diamond"),
    (0xaee, "heart"),
    (0xaf0, "maltesecross"),
    (0xaf1, "dagger"),
    (0xaf2, "doubledagger"),
    (0xaf3, "checkmark"),
    (0xaf4, "ballotcross"),
    (0xaf5, "musicalsharp"),
    (0xaf6, "musicalflat"),
    (0xaf7, "malesymbol"),
    (0xaf8, "femalesymbol"),
    (0xaf9, "telephone"),
    (0xafa, "telephonerecorder"),
    (0xafb, "phonographcopyright"),
    (0xafc, "caret"),
    (0xafd, "singlelowquotemark"),
    (0xafe, "doublelowquotemark"),
    (0xaff, "cursor"),
    (0xba3, "leftcaret"),
    (0xba6, "rightcaret"),
    (0xba8, "downcaret"),
    (0xba9, "upcaret"),
    (0xbc0, "overbar"),
    (0xbc2, "downtack"),
    (0xbc3, "upshoe"),
    (0xbc4, "downstile"),
    (0xbc6, "underbar"),
    (0xbca, "jot"),
    (0xbcc, "quad"),
    (0xbce, "uptack"),
    (0xbcf, "circle"),
    (0xbd3, "upstile"),
    (0xbd6, "downshoe"),
    (0xbd8, "rightshoe"),
    (0xbda, "leftshoe"),
    (0xbdc, "lefttack"),
    (0xbfc, "righttack"),
    (0xcdf, "hebrew_doublelowline"),
    (0xce0, "hebrew_aleph"),
    (0xce1, "hebrew_bet"),
    (0xce1, "hebrew_beth"),
    (0xce2, "hebrew_gimel"),
    (0xce2, "hebrew_gimmel"),
    (0xce3, "hebrew_dalet"),
    (0xce3, "hebrew_daleth"),
    (0xce4, "hebrew_he"),
    (0xce5, "hebrew_waw"),
    (0xce6, "hebrew_zain"),
    (0xce6, "hebrew_zayin"),
    (0xce7, "hebrew_chet"),
    (0xce7, "hebrew_het"),
    (0xce8, "hebrew_tet"),
    (0xce8, "hebrew_teth"),
    (0xce9, "hebrew_yod"),
    (0xcea, "hebrew_finalkaph"),
    (0xceb, "hebrew_kaph"),
    (0xcec, "hebrew_lamed"),
    (0xced, "hebrew_finalmem"),
    (0xcee, "hebrew_mem"),
    (0xcef, "hebrew_finalnun"),
    (0xcf0, "hebrew_nun"),
    (0xcf1, "hebrew_samech"),
    (0xcf1, "hebrew_samekh"),
    (0xcf2, "hebrew_ayin"),
    (0xcf3, "hebrew_finalpe"),
    (0xcf4, "hebrew_pe"),
    (0xcf5, "hebrew_finalzade"),
    (0xcf5, "hebrew_finalzadi"),
    (0xcf6, "hebrew_zade"),
    (0xcf6, "hebrew_zadi"),
    (0xcf7, "hebrew_kuf"),
    (0xcf7, "hebrew_qoph"),
    (0xcf8, "hebrew_resh"),
    (0xcf9, "hebrew_shin"),
    (0xcfa, "hebrew_taf"),
    (0xcfa, "hebrew_taw"),
    (0xda1, "Thai_kokai"),
    (0xda2, "Thai_khokhai"),
    (0xda3, "Thai_khokhuat"),
    (0xda4, "Thai_khokhwai"),
    (0xda5, "Thai_khokhon"),
    (0xda6, "Thai_khorakhang"),
    (0xda7, "Thai_ngongu"),
    (0xda8, "Thai_chochan"),
    (0xda9, "Thai_choching"),
    (0xdaa, "Thai_chochang"),
    (0xdab, "Thai_soso"),
    (0xdac, "Thai_chochoe"),
    (0xdad, "Thai_yoying"),
    (0xdae, "Thai_dochada"),
    (0xdaf, "Thai_topatak"),
    (0xdb0, "Thai_thothan"),
    (0xdb1, "Thai_thonangmontho"),
    (0xdb2, "Thai_thophuthao"),
    (0xdb3, "Thai_nonen"),
    (0xdb4, "Thai_dodek"),
    (0xdb5, "Thai_totao"),
    (0xdb6, "Thai_thothung"),
    (0xdb7, "Thai_thothahan"),
    (0xdb8, "Thai_thothong"),
    (0xdb9, "Thai_nonu"),
    (0xdba, "Thai_bobaimai"),
    (0xdbb, "Thai_popla"),
    (0xdbc, "Thai_phophung"),
    (0xdbd, "Thai_fofa"),
    (0xdbe, "Thai_phophan"),
    (0xdbf, "Thai_fofan"),
    (0xdc0, "Thai_phosamphao"),
    (0xdc1, "Thai_moma"),
    (0xdc2, "Thai_yoyak"),
    (0xdc3, "Thai_rorua"),
    (0xdc4, "Thai_ru"),
    (0xdc5, "Thai_loling"),
    (0xdc6, "Thai_lu"),
    (0xdc7, "Thai_wowaen"),
    (0xdc8, "Thai_sosala"),
    (0xdc9, "Thai_sorusi"),
    (0xdca, "Thai_sosua"),
    (0xdcb, "Thai_hohip"),
    (0xdcc, "Thai_lochula"),
    (0xdcd, "Thai_oang"),
    (0xdce, "Thai_honokhuk"),
    (0xdcf, "Thai_paiyannoi"),
    (0xdd0, "Thai_saraa"),
    (0xdd1, "Thai_maihanakat"),
    (0xdd2, "Thai_saraaa"),
    (0xdd3, "Thai_saraam"),
    (0xdd4, "Thai_sarai"),
    (0xdd5, "Thai_saraii"),
    (0xdd6, "Thai_saraue"),
    (0xdd7, "Thai_sarauee"),
    (0xdd8, "Thai_sarau"),
    (0xdd9, "Thai_sarauu"),
    (0xdda, "Thai_phinthu"),
    (0xdde, "Thai_maihanakat_maitho"),
    (0xddf, "Thai_baht"),
    (0xde0, "Thai_sarae"),
    (0xde1, "Thai_saraae"),
    (0xde2, "Thai_sarao"),
    (0xde3, "Thai_saraaimaimuan"),
    (0xde4, "Thai_saraaimaimalai"),
    (0xde5, "Thai_lakkhangyao"),
    (0xde6, "Thai_maiyamok"),
    (0xde7, "Thai_maitaikhu"),
    (0xde8, "Thai_maiek"),
    (0xde9, "Thai_maitho"),
    (0xdea, "Thai_maitri"),
    (0xdeb, "Thai_maichattawa"),
    (0xdec, "Thai_thanthakhat"),
    (0xded, "Thai_nikhahit"),
    (0xdf0, "Thai_leksun"),
    (0xdf1, "Thai_leknung"),
    (0xdf2, "Thai_leksong"),
    (0xdf3, "Thai_leksam"),
    (0xdf4, "Thai_leksi"),
    (0xdf5, "Thai_lekha"),
    (0xdf6, "Thai_lekhok"),
    (0xdf7, "Thai_lekchet"),
    (0xdf8, "Thai_lekpaet"),
    (0xdf9, "Thai_lekkao"),
    (0xea1, "Hangul_Kiyeog"),
    (0xea2, "Hangul_SsangKiyeog"),
    (0xea3, "Hangul_KiyeogSios"),
    (0xea4, "Hangul_Nieun"),
    (0xea5, "Hangul_NieunJieuj"),
    (0xea6, "Hangul_NieunHieuh"),
    (0xea7, "Hangul_Dikeud"),
    (0xea8, "Hangul_SsangDikeud"),
    (0xea9, "Hangul_Rieul"),
    (0xeaa, "Hangul_RieulKiyeog"),
    (0xeab, "Hangul_RieulMieum"),
    (0xeac, "Hangul_RieulPieub"),
    (0xead, "Hangul_RieulSios"),
    (0xeae, "Hangul_RieulTieut"),
    (0xeaf, "Hangul_RieulPhieuf"),
    (0xeb0, "Hangul_RieulHieuh"),
    (0xeb1, "Hangul_Mieum"),
    (0xeb2, "Hangul_Pieub"),
    (0xeb3, "Hangul_SsangPieub"),
    (0xeb4, "Hangul_PieubSios"),
    (0xeb5, "Hangul_Sios"),
    (0xeb6, "Hangul_SsangSios"),
    (0xeb7, "Hangul_Ieung"),
    (0xeb8, "Hangul_Jieuj"),
    (0xeb9, "Hangul_SsangJieuj"),
    (0xeba, "Hangul_Cieuc"),
    (0xebb, "Hangul_Khieuq"),
    (0xebc, "Hangul_Tieut"),
    (0xebd, "Hangul_Phieuf"),
    (0xebe, "Hangul_Hieuh"),
    (0xebf, "Hangul_A"),
    (0xec0, "Hangul_AE"),
    (0xec1, "Hangul_YA"),
    (0xec2, "Hangul_YAE"),
    (0xec3, "Hangul_EO"),
    (0xec4, "Hangul_E"),
    (0xec5, "Hangul_YEO"),
    (0xec6, "Hangul_YE"),
    (0xec7, "Hangul_O"),
    (0xec8, "Hangul_WA"),
    (0xec9, "Hangul_WAE"),
    (0xeca, "Hangul_OE"),
    (0xecb, "Hangul_YO"),
    (0xecc, "Hangul_U"),
    (0xecd, "Hangul_WEO"),
    (0xece, "Hangul_WE"),
    (0xecf, "Hangul_WI"),
    (0xed0, "Hangul_YU"),
    (0xed1, "Hangul_EU"),
    (0xed2, "Hangul_YI"),
    (0xed3, "Hangul_I"),
    (0xed4, "Hangul_J_Kiyeog"),
    (0xed5, "Hangul_J_SsangKiyeog"),
    (0xed6, "Hangul_J_KiyeogSios"),
    (0xed7, "Hangul_J_Nieun"),
    (0xed8, "Hangul_J_NieunJieuj"),
    (0xed9, "Hangul_J_NieunHieuh"),
    (0xeda, "Hangul_J_Dikeud"),
    (0xedb, "Hangul_J_Rieul"),
    (0xedc, "Hangul_J_RieulKiyeog"),
    (0xedd, "Hangul_J_RieulMieum"),
    (0xede, "Hangul_J_RieulPieub"),
    (0xedf, "Hangul_J_RieulSios"),
    (0xee0, "Hangul_J_RieulTieut"),
    (0xee1, "Hangul_J_RieulPhieuf"),
    (0xee2, "Hangul_J_RieulHieuh"),
    (0xee3, "Hangul_J_Mieum"),
    (0xee4, "Hangul_J_Pieub"),
    (0xee5, "Hangul_J_PieubSios"),
    (0xee6, "Hangul_J_Sios"),
    (0xee7, "Hangul_J_SsangSios"),
    (0xee8, "Hangul_J_Ieung"),
    (0xee9, "Hangul_J_Jieuj"),
    (0xeea, "Hangul_J_Cieuc"),
    (0xeeb, "Hangul_J_Khieuq"),
    (0xeec, "Hangul_J_Tieut"),
    (0xeed, "Hangul_J_Phieuf"),
    (0xeee, "Hangul_J_Hieuh"),
    (0xeef, "Hangul_RieulYeorinHieuh"),
    (0xef0, "Hangul_SunkyeongeumMieum"),
    (0xef1, "Hangul_SunkyeongeumPieub"),
    (0xef2, "Hangul_PanSios"),
    (0xef3, "Hangul_KkogjiDalrinIeung"),
    (0xef4, "Hangul_SunkyeongeumPhieuf"),
    (0xef5, "Hangul_YeorinHieuh"),
    (0xef6, "Hangul_AraeA"),
    (0xef7, "Hangul_AraeAE"),
    (0xef8, "Hangul_J_PanSios"),
    (0xef9, "Hangul_J_KkogjiDalrinIeung"),
    (0xefa, "Hangul_J_YeorinHieuh"),
    (0xeff, "Korean_Won"),
    (0x13bc, "OE"),
    (0x13bd, "oe"),
    (0x13be, "Ydiaeresis"),
    (0x20ac, "EuroSign"),
    (0xfd01, "3270_Duplicate"),
    (0xfd02, "3270_FieldMark"),
    (0xfd03, "3270_Right2"),
    (0xfd04, "3270_Left2"),
    (0xfd05, "3270_BackTab"),
    (0xfd06, "3270_EraseEOF"),
    (0xfd07, "3270_EraseInput"),
    (0xfd08, "3270_Reset"),
    (0xfd09, "3270_Quit"),
    (0xfd0a, "3270_PA1"),
    (0xfd0b, "3270_PA2"),
    (0xfd0c, "3270_PA3"),
    (0xfd0d, "3270_Test"),
    (0xfd0e, "3270_Attn"),
    (0xfd0f, "3270_CursorBlink"),
    (0xfd10, "3270_AltCursor"),
    (0xfd11, "3270_KeyClick"),
    (0xfd12, "3270_Jump"),
    (0xfd13, "3270_Ident"),
    (0xfd14, "3270_Rule"),
    (0xfd15, "3270_Copy"),
    (0xfd16, "3270_Play"),
    (0xfd17, "3270_Setup"),
    (0xfd18, "3270_Record"),
    (0xfd19, "3270_ChangeScreen"),
    (0xfd1a, "3270_DeleteWord"),
    (0xfd1b, "3270_ExSelect"),
    (0xfd1c, "3270_CursorSelect"),
    (0xfd1d, "3270_PrintScreen"),
    (0xfd1e, "3270_Enter"),
    (0xfe01, "ISO_Lock"),
    (0xfe02, "ISO_Level2_Latch"),
    (0xfe03, "ISO_Level3_Shift"),
    (0xfe04, "ISO_Level3_Latch"),
    (0xfe05, "ISO_Level3_Lock"),
    (0xfe06, "ISO_Group_Latch"),
    (0xfe07, "ISO_Group_Lock"),
    (0xfe08, "ISO_Next_Group"),
    (0xfe09, "ISO_Next_Group_Lock"),
    (0xfe0a, "ISO_Prev_Group"),
    (0xfe0b, "ISO_Prev_Group_Lock"),
    (0xfe0c, "ISO_First_Group"),
    (0xfe0d, "ISO_First_Group_Lock"),
    (0xfe0e, "ISO_Last_Group"),
    (0xfe0f, "ISO_Last_Group_Lock"),
    (0xfe11, "ISO_Level5_Shift"),
    (0xfe12, "ISO_Level5_Latch"),
    (0xfe13, "ISO_Level5_Lock"),
    (0xfe20, "ISO_Left_Tab"),
    (0xfe21, "ISO_Move_Line_Up"),
    (0xfe22, "ISO_Move_Line_Down"),
    (0xfe23, "ISO_Partial_Line_Up"),
    (0xfe24, "ISO_Partial_Line_Down"),
    (0xfe25, "ISO_Partial_Space_Left"),
    (0xfe26, "ISO_Partial_Space_Right"),
    (0xfe27, "ISO_Set_Margin_Left"),
    (0xfe28, "ISO_Set_Margin_Right"),
    (0xfe29, "ISO_Release_Margin_Left"),
    (0xfe2a, "ISO_Release_Margin_Right"),
    (0xfe2b, "ISO_Release_Both_Margins"),
    (0xfe2c, "ISO_Fast_Cursor_Left"),
    (0xfe2d, "ISO_Fast_Cursor_Right"),
    (0xfe2e, "ISO_Fast_Cursor_Up"),
    (0xfe2f, "ISO_Fast_Cursor_Down"),
    (0xfe30, "ISO_Continuous_Underline"),
    (0xfe31, "ISO_Discontinuous_Underline"),
    (0xfe32, "ISO_Emphasize"),
    (0xfe33, "ISO_Center_Object"),
    (0xfe34, "ISO_Enter"),
    (0xfe50, "dead_grave"),
    (0xfe51, "dead_acute"),
    (0xfe52, "dead_circumflex"),
    (0xfe53, "dead_perispomeni"),
    (0xfe53, "dead_tilde"),
    (0xfe54, "dead_macron"),
    (0xfe55, "dead_breve"),
    (0xfe56, "dead_abovedot"),
    (0xfe57, "dead_diaeresis"),
    (0xfe58, "dead_abovering"),
    (0xfe59, "dead_doubleacute"),
    (0xfe5a, "dead_caron"),
    (0xfe5b, "dead_cedilla"),
    (0xfe5c, "dead_ogonek"),
    (0xfe5d, "dead_iota"),
    (0xfe5e, "dead_voiced_sound"),
    (0xfe5f, "dead_semivoiced_sound"),
    (0xfe60, "dead_belowdot"),
    (0xfe61, "dead_hook"),
    (0xfe62, "dead_horn"),
    (0xfe63, "dead_stroke"),
    (0xfe64, "dead_abovecomma"),
    (0xfe64, "dead_psili"),
    (0xfe65, "dead_abovereversedcomma"),
    (0xfe65, "dead_dasia"),
    (0xfe66, "dead_doublegrave"),
    (0xfe67, "dead_belowring"),
    (0xfe68, "dead_belowmacron"),
    (0xfe69, "dead_belowcircumflex"),
    (0xfe6a, "dead_belowtilde"),
    (0xfe6b, "dead_belowbreve"),
    (0xfe6c, "dead_belowdiaeresis"),
    (0xfe6d, "dead_invertedbreve"),
    (0xfe6e, "dead_belowcomma"),
    (0xfe6f, "dead_currency"),
    (0xfe70, "AccessX_Enable"),
    (0xfe71, "AccessX_Feedback_Enable"),
    (0xfe72, "RepeatKeys_Enable"),
    (0xfe73, "SlowKeys_Enable"),
    (0xfe74, "BounceKeys_Enable"),
    (0xfe75, "StickyKeys_Enable"),
    (0xfe76, "MouseKeys_Enable"),
    (0xfe77, "MouseKeys_Accel_Enable"),
    (0xfe78, "Overlay1_Enable"),
    (0xfe79, "Overlay2_Enable"),
    (0xfe7a, "AudibleBell_Enable"),
    (0xfe80, "dead_a"),
    (0xfe81, "dead_A"),
    (0xfe82, "dead_e"),
    (0xfe83, "dead_E"),
    (0xfe84, "dead_i"),
    (0xfe85, "dead_I"),
    (0xfe86, "dead_o"),
    (0xfe87, "dead_O"),
    (0xfe88, "dead_u"),
    (0xfe89, "dead_U"),
    (0xfe8a, "dead_schwa"),
    (0xfe8a, "dead_small_schwa"),
    (0xfe8b, "dead_capital_schwa"),
    (0xfe8b, "dead_SCHWA"),
    (0xfe8c, "dead_greek"),
    (0xfe8d, "dead_hamza"),
    (0xfe8e, "dead_apostrophe"),
    (0xfe90, "dead_lowline"),
    (0xfe91, "dead_aboveverticalline"),
    (0xfe92, "dead_belowverticalline"),
    (0xfe93, "dead_longsolidusoverlay"),
    (0xfea0, "ch"),
    (0xfea1, "Ch"),
    (0xfea2, "CH"),
    (0xfea3, "c_h"),
    (0xfea4, "C_h"),
    (0xfea5, "C_H"),
    (0xfed0, "First_Virtual_Screen"),
    (0xfed1, "Prev_Virtual_Screen"),
    (0xfed2, "Next_Virtual_Screen"),
    (0xfed4, "Last_Virtual_Screen"),
    (0xfed5, "Terminate_Server"),
    (0xfee0, "Pointer_Left"),
    (0xfee1, "Pointer_Right"),
    (0xfee2, "Pointer_Up"),
    (0xfee3, "Pointer_Down"),
    (0xfee4, "Pointer_UpLeft"),
    (0xfee5, "Pointer_UpRight"),
    (0xfee6, "Pointer_DownLeft"),
    (0xfee7, "Pointer_DownRight"),
    (0xfee8, "Pointer_Button_Dflt"),
    (0xfee9, "Pointer_Button1"),
    (0xfeea, "Pointer_Button2"),
    (0xfeeb, "Pointer_Button3"),
    (0xfeec, "Pointer_Button4"),
    (0xfeed, "Pointer_Button5"),
    (0xfeee, "Pointer_DblClick_Dflt"),
    (0xfeef, "Pointer_DblClick1"),
    (0xfef0, "Pointer_DblClick2"),
    (0xfef1, "Pointer_DblClick3"),
    (0xfef2, "Pointer_DblClick4"),
    (0xfef3, "Pointer_DblClick5"),
    (0xfef4, "Pointer_Drag_Dflt"),
    (0xfef5, "Pointer_Drag1"),
    (0xfef6, "Pointer_Drag2"),
    (0xfef7, "Pointer_Drag3"),
    (0xfef8, "Pointer_Drag4"),
    (0xfef9, "Pointer_EnableKeys"),
    (0xfefa, "Pointer_Accelerate"),
    (0xfefb, "Pointer_DfltBtnNext"),
    (0xfefc, "Pointer_DfltBtnPrev"),
    (0xfefd, "Pointer_Drag5"),
    (0xff08, "BackSpace"),
    (0xff09, "Tab"),
    (0xff0a, "Linefeed"),
    (0xff0b, "Clear"),
    (0xff0d, "Return"),
    (0xff13, "Pause"),
    (0xff14, "Scroll_Lock"),
    (0xff15, "Sys_Req"),
    (0xff1b, "Escape"),
    (0xff20, "Multi_key"),
    (0xff20, "SunCompose"),
    (0xff21, "Kanji"),
    (0xff22, "Muhenkan"),
    (0xff23, "Henkan"),
    (0xff23, "Henkan_Mode"),
    (0xff24, "Romaji"),
    (0xff25, "Hiragana"),
    (0xff26, "Katakana"),
    (0xff27, "Hiragana_Katakana"),
    (0xff28, "Zenkaku"),
    (0xff29, "Hankaku"),
    (0xff2a, "Zenkaku_Hankaku"),
    (0xff2b, "Touroku"),
    (0xff2c, "Massyo"),
    (0xff2d, "Kana_Lock"),
    (0xff2e, "Kana_Shift"),
    (0xff2f, "Eisu_Shift"),
    (0xff30, "Eisu_toggle"),
    (0xff31, "Hangul"),
    (0xff32, "Hangul_Start"),
    (0xff33, "Hangul_End"),
    (0xff34, "Hangul_Hanja"),
    (0xff35, "Hangul_Jamo"),
    (0xff36, "Hangul_Romaja"),
    (0xff37, "Codeinput"),
    (0xff37, "Hangul_Codeinput"),
    (0xff37, "Kanji_Bangou"),
    (0xff38, "Hangul_Jeonja"),
    (0xff39, "Hangul_Banja"),
    (0xff3a, "Hangul_PreHanja"),
    (0xff3b, "Hangul_PostHanja"),
    (0xff3c, "Hangul_SingleCandidate"),
    (0xff3c, "SingleCandidate"),
    (0xff3d, "Hangul_MultipleCandidate"),
    (0xff3d, "MultipleCandidate"),
    (0xff3d, "Zen_Koho"),
    (0xff3e, "Hangul_PreviousCandidate"),
    (0xff3e, "Mae_Koho"),
    (0xff3e, "PreviousCandidate"),
    (0xff3f, "Hangul_Special"),
    (0xff50, "Home"),
    (0xff51, "Left"),
    (0xff52, "Up"),
    (0xff53, "Right"),
    (0xff54, "Down"),
    (0xff55, "Page_Up"),
    (0xff55, "Prior"),
    (0xff55, "SunPageUp"),
    (0xff56, "Next"),
    (0xff56, "Page_Down"),
    (0xff56, "SunPageDown"),
    (0xff57, "End"),
    (0xff58, "Begin"),
    (0xff60, "Select"),
    (0xff61, "Print"),
    (0xff61, "SunPrint_Screen"),
    (0xff62, "Execute"),
    (0xff63, "Insert"),
    (0xff65, "SunUndo"),
    (0xff65, "Undo"),
    (0xff66, "Redo"),
    (0xff66, "SunAgain"),
    (0xff67, "Menu"),
    (0xff68, "Find"),
    (0xff68, "SunFind"),
    (0xff69, "Cancel"),
    (0xff69, "SunStop"),
    (0xff6a, "Help"),
    (0xff6b, "Break"),
    (0xff7e, "Arabic_switch"),
    (0xff7e, "Greek_switch"),
    (0xff7e, "Hangul_switch"),
    (0xff7e, "Hebrew_switch"),
    (0xff7e, "ISO_Group_Shift"),
    (0xff7e, "kana_switch"),
    (0xff7e, "Mode_switch"),
    (0xff7e, "script_switch"),
    (0xff7e, "SunAltGraph"),
    (0xff7f, "Num_Lock"),
    (0xff80, "KP_Space"),
    (0xff89, "KP_Tab"),
    (0xff8d, "KP_Enter"),
    (0xff91, "KP_F1"),
    (0xff92, "KP_F2"),
    (0xff93, "KP_F3"),
    (0xff94, "KP_F4"),
    (0xff95, "KP_Home"),
    (0xff96, "KP_Left"),
    (0xff97, "KP_Up"),
    (0xff98, "KP_Right"),
    (0xff99, "KP_Down"),
    (0xff9a, "KP_Page_Up"),
    (0xff9a, "KP_Prior"),
    (0xff9b, "KP_Next"),
    (0xff9b, "KP_Page_Down"),
    (0xff9c, "KP_End"),
    (0xff9d, "KP_Begin"),
    (0xff9e, "KP_Insert"),
    (0xff9f, "KP_Delete"),
    (0xffaa, "KP_Multiply"),
    (0xffab, "KP_Add"),
    (0xffac, "KP_Separator"),
    (0xffad, "KP_Subtract"),
    (0xffae, "KP_Decimal"),
    (0xffaf, "KP_Divide"),
    (0xffb0, "KP_0"),
    (0xffb1, "KP_1"),
    (0xffb2, "KP_2"),
    (0xffb3, "KP_3"),
    (0xffb4, "KP_4"),
    (0xffb5, "KP_5"),
    (0xffb6, "KP_6"),
    (0xffb7, "KP_7"),
    (0xffb8, "KP_8"),
    (0xffb9, "KP_9"),
    (0xffbd, "KP_Equal"),
    (0xffbe, "F1"),
    (0xffbf, "F2"),
    (0xffc0, "F3"),
    (0xffc1, "F4"),
    (0xffc2, "F5"),
    (0xffc3, "F6"),
    (0xffc4, "F7"),
    (0xffc5, "F8"),
    (0xffc6, "F9"),
    (0xffc7, "F10"),
    (0xffc8, "F11"),
    (0xffc8, "L1"),
    (0xffc9, "F12"),
    (0xffc9, "L2"),
    (0xffca, "F13"),
    (0xffca, "L3"),
    (0xffcb, "F14"),
    (0xffcb, "L4"),
    (0xffcc, "F15"),
    (0xffcc, "L5"),
    (0xffcd, "F16"),
    (0xffcd, "L6"),
    (0xffce, "F17"),
    (0xffce, "L7"),
    (0xffcf, "F18"),
    (0xffcf, "L8"),
    (0xffd0, "F19"),
    (0xffd0, "L9"),
    (0xffd1, "F20"),
    (0xffd1, "L10"),
    (0xffd2, "F21"),
    (0xffd2, "R1"),
    (0xffd3, "F22"),
    (0xffd3, "R2"),
    (0xffd4, "F23"),
    (0xffd4, "R3"),
    (0xffd5, "F24"),
    (0xffd5, "R4"),
    (0xffd6, "F25"),
    (0xffd6, "R5"),
    (0xffd7, "F26"),
    (0xffd7, "R6"),
    (0xffd8, "F27"),
    (0xffd8, "R7"),
    (0xffd9, "F28"),
    (0xffd9, "R8"),
    (0xffda, "F29"),
    (0xffda, "R9"),
    (0xffdb, "F30"),
    (0xffdb, "R10"),
    (0xffdc, "F31"),
    (0xffdc, "R11"),
    (0xffdd, "F32"),
    (0xffdd, "R12"),
    (0xffde, "F33"),
    (0xffde, "R13"),
    (0xffdf, "F34"),
    (0xffdf, "R14"),
    (0xffe0, "F35"),
    (0xffe0, "R15"),
    (0xffe1, "Shift_L"),
    (0xffe2, "Shift_R"),
    (0xffe3, "Control_L"),
    (0xffe4, "Control_R"),
    (0xffe5, "Caps_Lock"),
    (0xffe6, "Shift_Lock"),
    (0xffe7, "Meta_L"),
    (0xffe8, "Meta_R"),
    (0xffe9, "Alt_L"),
    (0xffea, "Alt_R"),
    (0xffeb, "Super_L"),
    (0xffec, "Super_R"),
    (0xffed, "Hyper_L"),
    (0xffee, "Hyper_R"),
    (0xfff1, "braille_dot_1"),
    (0xfff2, "braille_dot_2"),
    (0xfff3, "braille_dot_3"),
    (0xfff4, "braille_dot_4"),
    (0xfff5, "braille_dot_5"),
    (0xfff6, "braille_dot_6"),
    (0xfff7, "braille_dot_7"),
    (0xfff8, "braille_dot_8"),
    (0xfff9, "braille_dot_9"),
    (0xfffa, "braille_dot_10"),
    (0xffff, "Delete"),
    (0xffffff, "VoidSymbol"),
    (0x100012c, "Ibreve"),
    (0x100012d, "ibreve"),
    (0x1000174, "Wcircumflex"),
    (0x1000175, "wcircumflex"),
    (0x1000176, "Ycircumflex"),
    (0x1000177, "ycircumflex"),
    (0x100018f, "SCHWA"),
    (0x100019f, "Obarred"),
    (0x10001a0, "Ohorn"),
    (0x10001a1, "ohorn"),
    (0x10001af, "Uhorn"),
    (0x10001b0, "uhorn"),
    (0x10001b5, "Zstroke"),
    (0x10001b6, "zstroke"),
    (0x10001b7, "EZH"),
    (0x10001d1, "Ocaron"),
    (0x10001d2, "ocaron"),
    (0x10001e6, "Gcaron"),
    (0x10001e7, "gcaron"),
    (0x1000259, "schwa"),
    (0x1000275, "obarred"),
    (0x1000292, "ezh"),
    (0x1000300, "combining_grave"),
    (0x1000301, "combining_acute"),
    (0x1000303, "combining_tilde"),
    (0x1000309, "combining_hook"),
    (0x1000323, "combining_belowdot"),
    (0x1000492, "Cyrillic_GHE_bar"),
    (0x1000493, "Cyrillic_ghe_bar"),
    (0x1000496, "Cyrillic_ZHE_descender"),
    (0x1000497, "Cyrillic_zhe_descender"),
    (0x100049a, "Cyrillic_KA_descender"),
    (0x100049b, "Cyrillic_ka_descender"),
    (0x100049c, "Cyrillic_KA_vertstroke"),
    (0x100049d, "Cyrillic_ka_vertstroke"),
    (0x10004a2, "Cyrillic_EN_descender"),
    (0x10004a3, "Cyrillic_en_descender"),
    (0x10004ae, "Cyrillic_U_straight"),
    (0x10004af, "Cyrillic_u_straight"),
    (0x10004b0, "Cyrillic_U_straight_bar"),
    (0x10004b1, "Cyrillic_u_straight_bar"),
    (0x10004b2, "Cyrillic_HA_descender"),
    (0x10004b3, "Cyrillic_ha_descender"),
    (0x10004b6, "Cyrillic_CHE_descender"),
    (0x10004b7, "Cyrillic_che_descender"),
    (0x10004b8, "Cyrillic_CHE_vertstroke"),
    (0x10004b9, "Cyrillic_che_vertstroke"),
    (0x10004ba, "Cyrillic_SHHA"),
    (0x10004bb, "Cyrillic_shha"),
    (0x10004d8, "Cyrillic_SCHWA"),
    (0x10004d9, "Cyrillic_schwa"),
    (0x10004e2, "Cyrillic_I_macron"),
    (0x10004e3, "Cyrillic_i_macron"),
    (0x10004e8, "Cyrillic_O_bar"),
    (0x10004e9, "Cyrillic_o_bar"),
    (0x10004ee, "Cyrillic_U_macron"),
    (0x10004ef, "Cyrillic_u_macron"),
    (0x1000531, "Armenian_AYB"),
    (0x1000532, "Armenian_BEN"),
    (0x1000533, "Armenian_GIM"),
    (0x1000534, "Armenian_DA"),
    (0x1000535, "Armenian_YECH"),
    (0x1000536, "Armenian_ZA"),
    (0x1000537, "Armenian_E"),
    (0x1000538, "Armenian_AT"),
    (0x1000539, "Armenian_TO"),
    (0x100053a, "Armenian_ZHE"),
    (0x100053b, "Armenian_INI"),
    (0x100053c, "Armenian_LYUN"),
    (0x100053d, "Armenian_KHE"),
    (0x100053e, "Armenian_TSA"),
    (0x100053f, "Armenian_KEN"),
    (0x1000540, "Armenian_HO"),
    (0x1000541, "Armenian_DZA"),
    (0x1000542, "Armenian_GHAT"),
    (0x1000543, "Armenian_TCHE"),
    (0x1000544, "Armenian_MEN"),
    (0x1000545, "Armenian_HI"),
    (0x1000546, "Armenian_NU"),
    (0x1000547, "Armenian_SHA"),
    (0x1000548, "Armenian_VO"),
    (0x1000549, "Armenian_CHA"),
    (0x100054a, "Armenian_PE"),
    (0x100054b, "Armenian_JE"),
    (0x100054c, "Armenian_RA"),
    (0x100054d, "Armenian_SE"),
    (0x100054e, "Armenian_VEV"),
    (0x100054f, "Armenian_TYUN"),
    (0x1000550, "Armenian_RE"),
    (0x1000551, "Armenian_TSO"),
    (0x1000552, "Armenian_VYUN"),
    (0x1000553, "Armenian_PYUR"),
    (0x1000554, "Armenian_KE"),
    (0x1000555, "Armenian_O"),
    (0x1000556, "Armenian_FE"),
    (0x100055a, "Armenian_apostrophe"),
    (0x100055b, "Armenian_accent"),
    (0x100055b, "Armenian_shesht"),
    (0x100055c, "Armenian_amanak"),
    (0x100055c, "Armenian_exclam"),
    (0x100055d, "Armenian_but"),
    (0x100055d, "Armenian_separation_mark"),
    (0x100055e, "Armenian_paruyk"),
    (0x100055e, "Armenian_question"),
    (0x1000561, "Armenian_ayb"),
    (0x1000562, "Armenian_ben"),
    (0x1000563, "Armenian_gim"),
    (0x1000564, "Armenian_da"),
    (0x1000565, "Armenian_yech"),
    (0x1000566, "Armenian_za"),
    (0x1000567, "Armenian_e"),
    (0x1000568, "Armenian_at"),
    (0x1000569, "Armenian_to"),
    (0x100056a, "Armenian_zhe"),
    (0x100056b, "Armenian_ini"),
    (0x100056c, "Armenian_lyun"),
    (0x100056d, "Armenian_khe"),
    (0x100056e, "Armenian_tsa"),
    (0x100056f, "Armenian_ken"),
    (0x1000570, "Armenian_ho"),
    (0x1000571, "Armenian_dza"),
    (0x1000572, "Armenian_ghat"),
    (0x1000573, "Armenian_tche"),
    (0x1000574, "Armenian_men"),
    (0x1000575, "Armenian_hi"),
    (0x1000576, "Armenian_nu"),
    (0x1000577, "Armenian_sha"),
    (0x1000578, "Armenian_vo"),
    (0x1000579, "Armenian_cha"),
    (0x100057a, "Armenian_pe"),
    (0x100057b, "Armenian_je"),
    (0x100057c, "Armenian_ra"),
    (0x100057d, "Armenian_se"),
    (0x100057e, "Armenian_vev"),
    (0x100057f, "Armenian_tyun"),
    (0x1000580, "Armenian_re"),
    (0x1000581, "Armenian_tso"),
    (0x1000582, "Armenian_vyun"),
    (0x1000583, "Armenian_pyur"),
    (0x1000584, "Armenian_ke"),
    (0x1000585, "Armenian_o"),
    (0x1000586, "Armenian_fe"),
    (0x1000587, "Armenian_ligature_ew"),
    (0x1000589, "Armenian_full_stop"),
    (0x1000589, "Armenian_verjaket"),
    (0x100058a, "Armenian_hyphen"),
    (0x100058a, "Armenian_yentamna"),
    (0x1000653, "Arabic_madda_above"),
    (0x1000654, "Arabic_hamza_above"),
    (0x1000655, "Arabic_hamza_below"),
    (0x1000660, "Arabic_0"),
    (0x1000661, "Arabic_1"),
    (0x1000662, "Arabic_2"),
    (0x1000663, "Arabic_3"),
    (0x1000664, "Arabic_4"),
    (0x1000665, "Arabic_5"),
    (0x1000666, "Arabic_6"),
    (0x1000667, "Arabic_7"),
    (0x1000668, "Arabic_8"),
    (0x1000669, "Arabic_9"),
    (0x100066a, "Arabic_percent"),
    (0x1000670, "Arabic_superscript_alef"),
    (0x1000679, "Arabic_tteh"),
    (0x100067e, "Arabic_peh"),
    (0x1000686, "Arabic_tcheh"),
    (0x1000688, "Arabic_ddal"),
    (0x1000691, "Arabic_rreh"),
    (0x1000698, "Arabic_jeh"),
    (0x10006a4, "Arabic_veh"),
    (0x10006a9, "Arabic_keheh"),
    (0x10006af, "Arabic_gaf"),
    (0x10006ba, "Arabic_noon_ghunna"),
    (0x10006be, "Arabic_heh_doachashmee"),
    (0x10006c1, "Arabic_heh_goal"),
    (0x10006cc, "Arabic_farsi_yeh"),
    (0x10006cc, "Farsi_yeh"),
    (0x10006d2, "Arabic_yeh_baree"),
    (0x10006d4, "Arabic_fullstop"),
    (0x10006f0, "Farsi_0"),
    (0x10006f1, "Farsi_1"),
    (0x10006f2, "Farsi_2"),
    (0x10006f3, "Farsi_3"),
    (0x10006f4, "Farsi_4"),
    (0x10006f5, "Farsi_5"),
    (0x10006f6, "Farsi_6"),
    (0x10006f7, "Farsi_7"),
    (0x10006f8, "Farsi_8"),
    (0x10006f9, "Farsi_9"),
    (0x1000d82, "Sinh_ng"),
    (0x1000d83, "Sinh_h2"),
    (0x1000d85, "Sinh_a"),
    (0x1000d86, "Sinh_aa"),
    (0x1000d87, "Sinh_ae"),
    (0x1000d88, "Sinh_aee"),
    (0x1000d89, "Sinh_i"),
    (0x1000d8a, "Sinh_ii"),
    (0x1000d8b, "Sinh_u"),
    (0x1000d8c, "Sinh_uu"),
    (0x1000d8d, "Sinh_ri"),
    (0x1000d8e, "Sinh_rii"),
    (0x1000d8f, "Sinh_lu"),
    (0x1000d90, "Sinh_luu"),
    (0x1000d91, "Sinh_e"),
    (0x1000d92, "Sinh_ee"),
    (0x1000d93, "Sinh_ai"),
    (0x1000d94, "Sinh_o"),
    (0x1000d95, "Sinh_oo"),
    (0x1000d96, "Sinh_au"),
    (0x1000d9a, "Sinh_ka"),
    (0x1000d9b, "Sinh_kha"),
    (0x1000d9c, "Sinh_ga"),
    (0x1000d9d, "Sinh_gha"),
    (0x1000d9e, "Sinh_ng2"),
    (0x1000d9f, "Sinh_nga"),
    (0x1000da0, "Sinh_ca"),
    (0x1000da1, "Sinh_cha"),
    (0x1000da2, "Sinh_ja"),
    (0x1000da3, "Sinh_jha"),
    (0x1000da4, "Sinh_nya"),
    (0x1000da5, "Sinh_jnya"),
    (0x1000da6, "Sinh_nja"),
    (0x1000da7, "Sinh_tta"),
    (0x1000da8, "Sinh_ttha"),
    (0x1000da9, "Sinh_dda"),
    (0x1000daa, "Sinh_ddha"),
    (0x1000dab, "Sinh_nna"),
    (0x1000dac, "Sinh_ndda"),
    (0x1000dad, "Sinh_tha"),
    (0x1000dae, "Sinh_thha"),
    (0x1000daf, "Sinh_dha"),
    (0x1000db0, "Sinh_dhha"),
    (0x1000db1, "Sinh_na"),
    (0x1000db3, "Sinh_ndha"),
    (0x1000db4, "Sinh_pa"),
    (0x1000db5, "Sinh_pha"),
    (0x1000db6, "Sinh_ba"),
    (0x1000db7, "Sinh_bha"),
    (0x1000db8, "Sinh_ma"),
    (0x1000db9, "Sinh_mba"),
    (0x1000dba, "Sinh_ya"),
    (0x1000dbb, "Sinh_ra"),
    (0x1000dbd, "Sinh_la"),
    (0x1000dc0, "Sinh_va"),
    (0x1000dc1, "Sinh_sha"),
    (0x1000dc2, "Sinh_ssha"),
    (0x1000dc3, "Sinh_sa"),
    (0x1000dc4, "Sinh_ha"),
    (0x1000dc5, "Sinh_lla"),
    (0x1000dc6, "Sinh_fa"),
    (0x1000dca, "Sinh_al"),
    (0x1000dcf, "Sinh_aa2"),
    (0x1000dd0, "Sinh_ae2"),
    (0x1000dd1, "Sinh_aee2"),
    (0x1000dd2, "Sinh_i2"),
    (0x1000dd3, "Sinh_ii2"),
    (0x1000dd4, "Sinh_u2"),
    (0x1000dd6, "Sinh_uu2"),
    (0x1000dd8, "Sinh_ru2"),
    (0x1000dd9, "Sinh_e2"),
    (0x1000dda, "Sinh_ee2"),
    (0x1000ddb, "Sinh_ai2"),
    (0x1000ddc, "Sinh_o2"),
    (0x1000ddd, "Sinh_oo2"),
    (0x1000dde, "Sinh_au2"),
    (0x1000ddf, "Sinh_lu2"),
    (0x1000df2, "Sinh_ruu2"),
    (0x1000df3, "Sinh_luu2"),
    (0x1000df4, "Sinh_kunddaliya"),
    (0x10010d0, "Georgian_an"),
    (0x10010d1, "Georgian_ban"),
    (0x10010d2, "Georgian_gan"),
    (0x10010d3, "Georgian_don"),
    (0x10010d4, "Georgian_en"),
    (0x10010d5, "Georgian_vin"),
    (0x10010d6, "Georgian_zen"),
    (0x10010d7, "Georgian_tan"),
    (0x10010d8, "Georgian_in"),
    (0x10010d9, "Georgian_kan"),
    (0x10010da, "Georgian_las"),
    (0x10010db, "Georgian_man"),
    (0x10010dc, "Georgian_nar"),
    (0x10010dd, "Georgian_on"),
    (0x10010de, "Georgian_par"),
    (0x10010df, "Georgian_zhar"),
    (0x10010e0, "Georgian_rae"),
    (0x10010e1, "Georgian_san"),
    (0x10010e2, "Georgian_tar"),
    (0x10010e3, "Georgian_un"),
    (0x10010e4, "Georgian_phar"),
    (0x10010e5, "Georgian_khar"),
    (0x10010e6, "Georgian_ghan"),
    (0x10010e7, "Georgian_qar"),
    (0x10010e8, "Georgian_shin"),
    (0x10010e9, "Georgian_chin"),
    (0x10010ea, "Georgian_can"),
    (0x10010eb, "Georgian_jil"),
    (0x10010ec, "Georgian_cil"),
    (0x10010ed, "Georgian_char"),
    (0x10010ee, "Georgian_xan"),
    (0x10010ef, "Georgian_jhan"),
    (0x10010f0, "Georgian_hae"),
    (0x10010f1, "Georgian_he"),
    (0x10010f2, "Georgian_hie"),
    (0x10010f3, "Georgian_we"),
    (0x10010f4, "Georgian_har"),
    (0x10010f5, "Georgian_hoe"),
    (0x10010f6, "Georgian_fi"),
    (0x1001e02, "Babovedot"),
    (0x1001e03, "babovedot"),
    (0x1001e0a, "Dabovedot"),
    (0x1001e0b, "dabovedot"),
    (0x1001e1e, "Fabovedot"),
    (0x1001e1f, "fabovedot"),
    (0x1001e36, "Lbelowdot"),
    (0x1001e37, "lbelowdot"),
    (0x1001e40, "Mabovedot"),
    (0x1001e41, "mabovedot"),
    (0x1001e56, "Pabovedot"),
    (0x1001e57, "pabovedot"),
    (0x1001e60, "Sabovedot"),
    (0x1001e61, "sabovedot"),
    (0x1001e6a, "Tabovedot"),
    (0x1001e6b, "tabovedot"),
    (0x1001e80, "Wgrave"),
    (0x1001e81, "wgrave"),
    (0x1001e82, "Wacute"),
    (0x1001e83, "wacute"),
    (0x1001e84, "Wdiaeresis"),
    (0x1001e85, "wdiaeresis"),
    (0x1001e8a, "Xabovedot"),
    (0x1001e8b, "xabovedot"),
    (0x1001e9e, "SSHARP"),
    (0x1001ea0, "Abelowdot"),
    (0x1001ea1, "abelowdot"),
    (0x1001ea2, "Ahook"),
    (0x1001ea3, "ahook"),
    (0x1001ea4, "Acircumflexacute"),
    (0x1001ea5, "acircumflexacute"),
    (0x1001ea6, "Acircumflexgrave"),
    (0x1001ea7, "acircumflexgrave"),
    (0x1001ea8, "Acircumflexhook"),
    (0x1001ea9, "acircumflexhook"),
    (0x1001eaa, "Acircumflextilde"),
    (0x1001eab, "acircumflextilde"),
    (0x1001eac, "Acircumflexbelowdot"),
    (0x1001ead, "acircumflexbelowdot"),
    (0x1001eae, "Abreveacute"),
    (0x1001eaf, "abreveacute"),
    (0x1001eb0, "Abrevegrave"),
    (0x1001eb1, "abrevegrave"),
    (0x1001eb2, "Abrevehook"),
    (0x1001eb3, "abrevehook"),
    (0x1001eb4, "Abrevetilde"),
    (0x1001eb5, "abrevetilde"),
    (0x1001eb6, "Abrevebelowdot"),
    (0x1001eb7, "abrevebelowdot"),
    (0x1001eb8, "Ebelowdot"),
    (0x1001eb9, "ebelowdot"),
    (0x1001eba, "Ehook"),
    (0x1001ebb, "ehook"),
    (0x1001ebc, "Etilde"),
    (0x1001ebd, "etilde"),
    (0x1001ebe, "Ecircumflexacute"),
    (0x1001ebf, "ecircumflexacute"),
    (0x1001ec0, "Ecircumflexgrave"),
    (0x1001ec1, "ecircumflexgrave"),
    (0x1001ec2, "Ecircumflexhook"),
    (0x1001ec3, "ecircumflexhook"),
    (0x1001ec4, "Ecircumflextilde"),
    (0x1001ec5, "ecircumflextilde"),
    (0x1001ec6, "Ecircumflexbelowdot"),
    (0x1001ec7, "ecircumflexbelowdot"),
    (0x1001ec8, "Ihook"),
    (0x1001ec9, "ihook"),
    (0x1001eca, "Ibelowdot"),
    (0x1001ecb, "ibelowdot"),
    (0x1001ecc, "Obelowdot"),
    (0x1001ecd, "obelowdot"),
    (0x1001ece, "Ohook"),
    (0x1001ecf, "ohook"),
    (0x1001ed0, "Ocircumflexacute"),
    (0x1001ed1, "ocircumflexacute"),
    (0x1001ed2, "Ocircumflexgrave"),
    (0x1001ed3, "ocircumflexgrave"),
    (0x1001ed4, "Ocircumflexhook"),
    (0x1001ed5, "ocircumflexhook"),
    (0x1001ed6, "Ocircumflextilde"),
    (0x1001ed7, "ocircumflextilde"),
    (0x1001ed8, "Ocircumflexbelowdot"),
    (0x1001ed9, "ocircumflexbelowdot"),
    (0x1001eda, "Ohornacute"),
    (0x1001edb, "ohornacute"),
    (0x1001edc, "Ohorngrave"),
    (0x1001edd, "ohorngrave"),
    (0x1001ede, "Ohornhook"),
    (0x1001edf, "ohornhook"),
    (0x1001ee0, "Ohorntilde"),
    (0x1001ee1, "ohorntilde"),
    (0x1001ee2, "Ohornbelowdot"),
    (0x1001ee3, "ohornbelowdot"),
    (0x1001ee4, "Ubelowdot"),
    (0x1001ee5, "ubelowdot"),
    (0x1001ee6, "Uhook"),
    (0x1001ee7, "uhook"),
    (0x1001ee8, "Uhornacute"),
    (0x1001ee9, "uhornacute"),
    (0x1001eea, "Uhorngrave"),
    (0x1001eeb, "uhorngrave"),
    (0x1001eec, "Uhornhook"),
    (0x1001eed, "uhornhook"),
    (0x1001eee, "Uhorntilde"),
    (0x1001eef, "uhorntilde"),
    (0x1001ef0, "Uhornbelowdot"),
    (0x1001ef1, "uhornbelowdot"),
    (0x1001ef2, "Ygrave"),
    (0x1001ef3, "ygrave"),
    (0x1001ef4, "Ybelowdot"),
    (0x1001ef5, "ybelowdot"),
    (0x1001ef6, "Yhook"),
    (0x1001ef7, "yhook"),
    (0x1001ef8, "Ytilde"),
    (0x1001ef9, "ytilde"),
    (0x1002039, "leftsingleanglequotemark"),
    (0x100203a, "rightsingleanglequotemark"),
    (0x1002070, "zerosuperior"),
    (0x1002074, "foursuperior"),
    (0x1002075, "fivesuperior"),
    (0x1002076, "sixsuperior"),
    (0x1002077, "sevensuperior"),
    (0x1002078, "eightsuperior"),
    (0x1002079, "ninesuperior"),
    (0x1002080, "zerosubscript"),
    (0x1002081, "onesubscript"),
    (0x1002082, "twosubscript"),
    (0x1002083, "threesubscript"),
    (0x1002084, "foursubscript"),
    (0x1002085, "fivesubscript"),
    (0x1002086, "sixsubscript"),
    (0x1002087, "sevensubscript"),
    (0x1002088, "eightsubscript"),
    (0x1002089, "ninesubscript"),
    (0x10020a0, "EcuSign"),
    (0x10020a1, "ColonSign"),
    (0x10020a2, "CruzeiroSign"),
    (0x10020a3, "FFrancSign"),
    (0x10020a4, "LiraSign"),
    (0x10020a5, "MillSign"),
    (0x10020a6, "NairaSign"),
    (0x10020a7, "PesetaSign"),
    (0x10020a8, "RupeeSign"),
    (0x10020a9, "WonSign"),
    (0x10020aa, "NewSheqelSign"),
    (0x10020ab, "DongSign"),
    (0x1002202, "partdifferential"),
    (0x1002205, "emptyset"),
    (0x1002208, "elementof"),
    (0x1002209, "notelementof"),
    (0x100220b, "containsas"),
    (0x100221a, "squareroot"),
    (0x100221b, "cuberoot"),
    (0x100221c, "fourthroot"),
    (0x100222c, "dintegral"),
    (0x100222d, "tintegral"),
    (0x1002235, "because"),
    (0x1002247, "notapproxeq"),
    (0x1002248, "approxeq"),
    (0x1002262, "notidentical"),
    (0x1002263, "stricteq"),
    (0x1002800, "braille_blank"),
    (0x1002801, "braille_dots_1"),
    (0x1002802, "braille_dots_2"),
    (0x1002803, "braille_dots_12"),
    (0x1002804, "braille_dots_3"),
    (0x1002805, "braille_dots_13"),
    (0x1002806, "braille_dots_23"),
    (0x1002807, "braille_dots_123"),
    (0x1002808, "braille_dots_4"),
    (0x1002809, "braille_dots_14"),
    (0x100280a, "braille_dots_24"),
    (0x100280b, "braille_dots_124"),
    (0x100280c, "braille_dots_34"),
    (0x100280d, "braille_dots_134"),
    (0x100280e, "braille_dots_234"),
    (0x100280f, "braille_dots_1234"),
    (0x1002810, "braille_dots_5"),
    (0x1002811, "braille_dots_15"),
    (0x1002812, "braille_dots_25"),
    (0x1002813, "braille_dots_125"),
    (0x1002814, "braille_dots_35"),
    (0x1002815, "braille_dots_135"),
    (0x1002816, "braille_dots_235"),
    (0x1002817, "braille_dots_1235"),
    (0x1002818, "braille_dots_45"),
    (0x1002819, "braille_dots_145"),
    (0x100281a, "braille_dots_245"),
    (0x100281b, "braille_dots_1245"),
    (0x100281c, "braille_dots_345"),
    (0x100281d, "braille_dots_1345"),
    (0x100281e, "braille_dots_2345"),
    (0x100281f, "braille_dots_12345"),
    (0x1002820, "braille_dots_6"),
    (0x1002821, "braille_dots_16"),
    (0x1002822, "braille_dots_26"),
    (0x1002823, "braille_dots_126"),
    (0x1002824, "braille_dots_36"),
    (0x1002825, "braille_dots_136"),
    (0x1002826, "braille_dots_236"),
    (0x1002827, "braille_dots_1236"),
    (0x1002828, "braille_dots_46"),
    (0x1002829, "braille_dots_146"),
    (0x100282a, "braille_dots_246"),
    (0x100282b, "braille_dots_1246"),
    (0x100282c, "braille_dots_346"),
    (0x100282d, "braille_dots_1346"),
    (0x100282e, "braille_dots_2346"),
    (0x100282f, "braille_dots_12346"),
    (0x1002830, "braille_dots_56"),
    (0x1002831, "braille_dots_156"),
    (0x1002832, "braille_dots_256"),
    (0x1002833, "braille_dots_1256"),
    (0x1002834, "braille_dots_356"),
    (0x1002835, "braille_dots_1356"),
    (0x1002836, "braille_dots_2356"),
    (0x1002837, "braille_dots_12356"),
    (0x1002838, "braille_dots_456"),
    (0x1002839, "braille_dots_1456"),
    (0x100283a, "braille_dots_2456"),
    (0x100283b, "braille_dots_12456"),
    (0x100283c, "braille_dots_3456"),
    (0x100283d, "braille_dots_13456"),
    (0x100283e, "braille_dots_23456"),
    (0x100283f, "braille_dots_123456"),
    (0x1002840, "braille_dots_7"),
    (0x1002841, "braille_dots_17"),
    (0x1002842, "braille_dots_27"),
    (0x1002843, "braille_dots_127"),
    (0x1002844, "braille_dots_37"),
    (0x1002845, "braille_dots_137"),
    (0x1002846, "braille_dots_237"),
    (0x1002847, "braille_dots_1237"),
    (0x1002848, "braille_dots_47"),
    (0x1002849, "braille_dots_147"),
    (0x100284a, "braille_dots_247"),
    (0x100284b, "braille_dots_1247"),
    (0x100284c, "braille_dots_347"),
    (0x100284d, "braille_dots_1347"),
    (0x100284e, "braille_dots_2347"),
    (0x100284f, "braille_dots_12347"),
    (0x1002850, "braille_dots_57"),
    (0x1002851, "braille_dots_157"),
    (0x1002852, "braille_dots_257"),
    (0x1002853, "braille_dots_1257"),
    (0x1002854, "braille_dots_357"),
    (0x1002855, "braille_dots_1357"),
    (0x1002856, "braille_dots_2357"),
    (0x1002857, "braille_dots_12357"),
    (0x1002858, "braille_dots_457"),
    (0x1002859, "braille_dots_1457"),
    (0x100285a, "braille_dots_2457"),
    (0x100285b, "braille_dots_12457"),
    (0x100285c, "braille_dots_3457"),
    (0x100285d, "braille_dots_13457"),
    (0x100285e, "braille_dots_23457"),
    (0x100285f, "braille_dots_123457"),
    (0x1002860, "braille_dots_67"),
    (0x1002861, "braille_dots_167"),
    (0x1002862, "braille_dots_267"),
    (0x1002863, "braille_dots_1267"),
    (0x1002864, "braille_dots_367"),
    (0x1002865, "braille_dots_1367"),
    (0x1002866, "braille_dots_2367"),
    (0x1002867, "braille_dots_12367"),
    (0x1002868, "braille_dots_467"),
    (0x1002869, "braille_dots_1467"),
    (0x100286a, "braille_dots_2467"),
    (0x100286b, "braille_dots_12467"),
    (0x100286c, "braille_dots_3467"),
    (0x100286d, "braille_dots_13467"),
    (0x100286e, "braille_dots_23467"),
    (0x100286f, "braille_dots_123467"),
    (0x1002870, "braille_dots_567"),
    (0x1002871, "braille_dots_1567"),
    (0x1002872, "braille_dots_2567"),
    (0x1002873, "braille_dots_12567"),
    (0x1002874, "braille_dots_3567"),
    (0x1002875, "braille_dots_13567"),
    (0x1002876, "braille_dots_23567"),
    (0x1002877, "braille_dots_123567"),
    (0x1002878, "braille_dots_4567"),
    (0x1002879, "braille_dots_14567"),
    (0x100287a, "braille_dots_24567"),
    (0x100287b, "braille_dots_124567"),
    (0x100287c, "braille_dots_34567"),
    (0x100287d, "braille_dots_134567"),
    (0x100287e, "braille_dots_234567"),
    (0x100287f, "braille_dots_1234567"),
    (0x1002880, "braille_dots_8"),
    (0x1002881, "braille_dots_18"),
    (0x1002882, "braille_dots_28"),
    (0x1002883, "braille_dots_128"),
    (0x1002884, "braille_dots_38"),
    (0x1002885, "braille_dots_138"),
    (0x1002886, "braille_dots_238"),
    (0x1002887, "braille_dots_1238"),
    (0x1002888, "braille_dots_48"),
    (0x1002889, "braille_dots_148"),
    (0x100288a, "braille_dots_248"),
    (0x100288b, "braille_dots_1248"),
    (0x100288c, "braille_dots_348"),
    (0x100288d, "braille_dots_1348"),
    (0x100288e, "braille_dots_2348"),
    (0x100288f, "braille_dots_12348"),
    (0x1002890, "braille_dots_58"),
    (0x1002891, "braille_dots_158"),
    (0x1002892, "braille_dots_258"),
    (0x1002893, "braille_dots_1258"),
    (0x1002894, "braille_dots_358"),
    (0x1002895, "braille_dots_1358"),
    (0x1002896, "braille_dots_2358"),
    (0x1002897, "braille_dots_12358"),
    (0x1002898, "braille_dots_458"),
    (0x1002899, "braille_dots_1458"),
    (0x100289a, "braille_dots_2458"),
    (0x100289b, "braille_dots_12458"),
    (0x100289c, "braille_dots_3458"),
    (0x100289d, "braille_dots_13458"),
    (0x100289e, "braille_dots_23458"),
    (0x100289f, "braille_dots_123458"),
    (0x10028a0, "braille_dots_68"),
    (0x10028a1, "braille_dots_168"),
    (0x10028a2, "braille_dots_268"),
    (0x10028a3, "braille_dots_1268"),
    (0x10028a4, "braille_dots_368"),
    (0x10028a5, "braille_dots_1368"),
    (0x10028a6, "braille_dots_2368"),
    (0x10028a7, "braille_dots_12368"),
    (0x10028a8, "braille_dots_468"),
    (0x10028a9, "braille_dots_1468"),
    (0x10028aa, "braille_dots_2468"),
    (0x10028ab, "braille_dots_12468"),
    (0x10028ac, "braille_dots_3468"),
    (0x10028ad, "braille_dots_13468"),
    (0x10028ae, "braille_dots_23468"),
    (0x10028af, "braille_dots_123468"),
    (0x10028b0, "braille_dots_568"),
    (0x10028b1, "braille_dots_1568"),
    (0x10028b2, "braille_dots_2568"),
    (0x10028b3, "braille_dots_12568"),
    (0x10028b4, "braille_dots_3568"),
    (0x10028b5, "braille_dots_13568"),
    (0x10028b6, "braille_dots_23568"),
    (0x10028b7, "braille_dots_123568"),
    (0x10028b8, "braille_dots_4568"),
    (0x10028b9, "braille_dots_14568"),
    (0x10028ba, "braille_dots_24568"),
    (0x10028bb, "braille_dots_124568"),
    (0x10028bc, "braille_dots_34568"),
    (0x10028bd, "braille_dots_134568"),
    (0x10028be, "braille_dots_234568"),
    (0x10028bf, "braille_dots_1234568"),
    (0x10028c0, "braille_dots_78"),
    (0x10028c1, "braille_dots_178"),
    (0x10028c2, "braille_dots_278"),
    (0x10028c3, "braille_dots_1278"),
    (0x10028c4, "braille_dots_378"),
    (0x10028c5, "braille_dots_1378"),
    (0x10028c6, "braille_dots_2378"),
    (0x10028c7, "braille_dots_12378"),
    (0x10028c8, "braille_dots_478"),
    (0x10028c9, "braille_dots_1478"),
    (0x10028ca, "braille_dots_2478"),
    (0x10028cb, "braille_dots_12478"),
    (0x10028cc, "braille_dots_3478"),
    (0x10028cd, "braille_dots_13478"),
    (0x10028ce, "braille_dots_23478"),
    (0x10028cf, "braille_dots_123478"),
    (0x10028d0, "braille_dots_578"),
    (0x10028d1, "braille_dots_1578"),
    (0x10028d2, "braille_dots_2578"),
    (0x10028d3, "braille_dots_12578"),
    (0x10028d4, "braille_dots_3578"),
    (0x10028d5, "braille_dots_13578"),
    (0x10028d6, "braille_dots_23578"),
    (0x10028d7, "braille_dots_123578"),
    (0x10028d8, "braille_dots_4578"),
    (0x10028d9, "braille_dots_14578"),
    (0x10028da, "braille_dots_24578"),
    (0x10028db, "braille_dots_124578"),
    (0x10028dc, "braille_dots_34578"),
    (0x10028dd, "braille_dots_134578"),
    (0x10028de, "braille_dots_234578"),
    (0x10028df, "braille_dots_1234578"),
    (0x10028e0, "braille_dots_678"),
    (0x10028e1, "braille_dots_1678"),
    (0x10028e2, "braille_dots_2678"),
    (0x10028e3, "braille_dots_12678"),
    (0x10028e4, "braille_dots_3678"),
    (0x10028e5, "braille_dots_13678"),
    (0x10028e6, "braille_dots_23678"),
    (0x10028e7, "braille_dots_123678"),
    (0x10028e8, "braille_dots_4678"),
    (0x10028e9, "braille_dots_14678"),
    (0x10028ea, "braille_dots_24678"),
    (0x10028eb, "braille_dots_124678"),
    (0x10028ec, "braille_dots_34678"),
    (0x10028ed, "braille_dots_134678"),
    (0x10028ee, "braille_dots_234678"),
    (0x10028ef, "braille_dots_1234678"),
    (0x10028f0, "braille_dots_5678"),
    (0x10028f1, "braille_dots_15678"),
    (0x10028f2, "braille_dots_25678"),
    (0x10028f3, "braille_dots_125678"),
    (0x10028f4, "braille_dots_35678"),
    (0x10028f5, "braille_dots_135678"),
    (0x10028f6, "braille_dots_235678"),
    (0x10028f7, "braille_dots_1235678"),
    (0x10028f8, "braille_dots_45678"),
    (0x10028f9, "braille_dots_145678"),
    (0x10028fa, "braille_dots_245678"),
    (0x10028fb, "braille_dots_1245678"),
    (0x10028fc, "braille_dots_345678"),
    (0x10028fd, "braille_dots_1345678"),
    (0x10028fe, "braille_dots_2345678"),
    (0x10028ff, "braille_dots_12345678"),
    (0x100000a8, "hpmute_acute"),
    (0x100000a8, "mute_acute"),
    (0x100000a9, "hpmute_grave"),
    (0x100000a9, "mute_grave"),
    (0x100000aa, "hpmute_asciicircum"),
    (0x100000aa, "mute_asciicircum"),
    (0x100000ab, "hpmute_diaeresis"),
    (0x100000ab, "mute_diaeresis"),
    (0x100000ac, "hpmute_asciitilde"),
    (0x100000ac, "mute_asciitilde"),
    (0x100000af, "hplira"),
    (0x100000af, "lira"),
    (0x100000be, "guilder"),
    (0x100000be, "hpguilder"),
    (0x100000ee, "hpIO"),
    (0x100000ee, "hpYdiaeresis"),
    (0x100000ee, "IO"),
    (0x100000f6, "hplongminus"),
    (0x100000f6, "longminus"),
    (0x100000fc, "block"),
    (0x100000fc, "hpblock"),
    (0x1000fe22, "Ddiaeresis"),
    (0x1000fe27, "Dacute_accent"),
    (0x1000fe2c, "Dcedilla_accent"),
    (0x1000fe5e, "Dcircumflex_accent"),
    (0x1000fe60, "Dgrave_accent"),
    (0x1000fe7e, "Dtilde"),
    (0x1000feb0, "Dring_accent"),
    (0x1000ff00, "DRemove"),
    (0x1000ff48, "hpModelock1"),
    (0x1000ff49, "hpModelock2"),
    (0x1000ff6c, "hpReset"),
    (0x1000ff6c, "Reset"),
    (0x1000ff6d, "hpSystem"),
    (0x1000ff6d, "System"),
    (0x1000ff6e, "hpUser"),
    (0x1000ff6e, "User"),
    (0x1000ff6f, "ClearLine"),
    (0x1000ff6f, "hpClearLine"),
    (0x1000ff70, "hpInsertLine"),
    (0x1000ff70, "InsertLine"),
    (0x1000ff71, "DeleteLine"),
    (0x1000ff71, "hpDeleteLine"),
    (0x1000ff72, "hpInsertChar"),
    (0x1000ff72, "InsertChar"),
    (0x1000ff73, "DeleteChar"),
    (0x1000ff73, "hpDeleteChar"),
    (0x1000ff74, "BackTab"),
    (0x1000ff74, "hpBackTab"),
    (0x1000ff75, "hpKP_BackTab"),
    (0x1000ff75, "KP_BackTab"),
    (0x1000ff76, "Ext16bit_L"),
    (0x1000ff77, "Ext16bit_R"),
    (0x1004ff02, "osfCopy"),
    (0x1004ff03, "osfCut"),
    (0x1004ff04, "osfPaste"),
    (0x1004ff07, "osfBackTab"),
    (0x1004ff08, "osfBackSpace"),
    (0x1004ff0b, "osfClear"),
    (0x1004ff1b, "osfEscape"),
    (0x1004ff31, "osfAddMode"),
    (0x1004ff32, "osfPrimaryPaste"),
    (0x1004ff33, "osfQuickPaste"),
    (0x1004ff40, "osfPageLeft"),
    (0x1004ff41, "osfPageUp"),
    (0x1004ff42, "osfPageDown"),
    (0x1004ff43, "osfPageRight"),
    (0x1004ff44, "osfActivate"),
    (0x1004ff45, "osfMenuBar"),
    (0x1004ff51, "osfLeft"),
    (0x1004ff52, "osfUp"),
    (0x1004ff53, "osfRight"),
    (0x1004ff54, "osfDown"),
    (0x1004ff57, "osfEndLine"),
    (0x1004ff58, "osfBeginLine"),
    (0x1004ff59, "osfEndData"),
    (0x1004ff5a, "osfBeginData"),
    (0x1004ff5b, "osfPrevMenu"),
    (0x1004ff5c, "osfNextMenu"),
    (0x1004ff5d, "osfPrevField"),
    (0x1004ff5e, "osfNextField"),
    (0x1004ff60, "osfSelect"),
    (0x1004ff63, "osfInsert"),
    (0x1004ff65, "osfUndo"),
    (0x1004ff67, "osfMenu"),
    (0x1004ff69, "osfCancel"),
    (0x1004ff6a, "osfHelp"),
    (0x1004ff71, "osfSelectAll"),
    (0x1004ff72, "osfDeselectAll"),
    (0x1004ff73, "osfReselect"),
    (0x1004ff74, "osfExtend"),
    (0x1004ff78, "osfRestore"),
    (0x1004ffff, "osfDelete"),
    (0x1005ff00, "SunFA_Grave"),
    (0x1005ff01, "SunFA_Circum"),
    (0x1005ff02, "SunFA_Tilde"),
    (0x1005ff03, "SunFA_Acute"),
    (0x1005ff04, "SunFA_Diaeresis"),
    (0x1005ff05, "SunFA_Cedilla"),
    (0x1005ff10, "SunF36"),
    (0x1005ff11, "SunF37"),
    (0x1005ff60, "SunSys_Req"),
    (0x1005ff70, "SunProps"),
    (0x1005ff71, "SunFront"),
    (0x1005ff72, "SunCopy"),
    (0x1005ff73, "SunOpen"),
    (0x1005ff74, "SunPaste"),
    (0x1005ff75, "SunCut"),
    (0x1005ff76, "SunPowerSwitch"),
    (0x1005ff77, "SunAudioLowerVolume"),
    (0x1005ff78, "SunAudioMute"),
    (0x1005ff79, "SunAudioRaiseVolume"),
    (0x1005ff7a, "SunVideoDegauss"),
    (0x1005ff7b, "SunVideoLowerBrightness"),
    (0x1005ff7c, "SunVideoRaiseBrightness"),
    (0x1005ff7d, "SunPowerSwitchShift"),
    (0x100810a4, "XF86MediaPlayPause"),
    (0x100810ae, "XF86Exit"),
    (0x100810d1, "XF86AudioBassBoost"),
    (0x100810dc, "XF86Sport"),
    (0x100810f4, "XF86BrightnessAuto"),
    (0x100810f4, "XF86MonBrightnessAuto"),
    (0x100810f5, "XF86DisplayOff"),
    (0x10081160, "XF86OK"),
    (0x10081162, "XF86GoTo"),
    (0x10081166, "XF86Info"),
    (0x10081168, "XF86VendorLogo"),
    (0x1008116a, "XF86MediaSelectProgramGuide"),
    (0x1008116e, "XF86MediaSelectHome"),
    (0x10081170, "XF86MediaLanguageMenu"),
    (0x10081171, "XF86MediaTitleMenu"),
    (0x10081175, "XF86AudioChannelMode"),
    (0x10081177, "XF86AspectRatio"),
    (0x10081178, "XF86MediaSelectPC"),
    (0x10081179, "XF86MediaSelectTV"),
    (0x1008117a, "XF86MediaSelectCable"),
    (0x1008117b, "XF86MediaSelectVCR"),
    (0x1008117c, "XF86MediaSelectVCRPlus"),
    (0x1008117d, "XF86MediaSelectSatellite"),
    (0x10081180, "XF86MediaSelectTape"),
    (0x10081181, "XF86MediaSelectRadio"),
    (0x10081182, "XF86MediaSelectTuner"),
    (0x10081183, "XF86MediaPlayer"),
    (0x10081184, "XF86MediaSelectTeletext"),
    (0x10081185, "XF86DVD"),
    (0x10081185, "XF86MediaSelectDVD"),
    (0x10081186, "XF86MediaSelectAuxiliary"),
    (0x10081188, "XF86Audio"),
    (0x10081192, "XF86ChannelUp"),
    (0x10081193, "XF86ChannelDown"),
    (0x10081199, "XF86MediaPlaySlow"),
    (0x1008119b, "XF86Break"),
    (0x1008119d, "XF86NumberEntryMode"),
    (0x100811a0, "XF86VideoPhone"),
    (0x100811a4, "XF86ZoomReset"),
    (0x100811a6, "XF86Editor"),
    (0x100811a8, "XF86GraphicsEditor"),
    (0x100811a9, "XF86Presentation"),
    (0x100811aa, "XF86Database"),
    (0x100811ac, "XF86Voicemail"),
    (0x100811ad, "XF86Addressbook"),
    (0x100811af, "XF86DisplayToggle"),
    (0x100811b0, "XF86SpellCheck"),
    (0x100811b6, "XF86ContextMenu"),
    (0x100811b7, "XF86MediaRepeat"),
    (0x100811b8, "XF8610ChannelsUp"),
    (0x100811b9, "XF8610ChannelsDown"),
    (0x100811ba, "XF86Images"),
    (0x100811bc, "XF86NotificationCenter"),
    (0x100811bd, "XF86PickupPhone"),
    (0x100811be, "XF86HangupPhone"),
    (0x100811bf, "XF86LinkPhone"),
    (0x100811d0, "XF86Fn"),
    (0x100811d1, "XF86Fn_Esc"),
    (0x100811d2, "XF86Fn_F1"),
    (0x100811d3, "XF86Fn_F2"),
    (0x100811d4, "XF86Fn_F3"),
    (0x100811d5, "XF86Fn_F4"),
    (0x100811d6, "XF86Fn_F5"),
    (0x100811d7, "XF86Fn_F6"),
    (0x100811d8, "XF86Fn_F7"),
    (0x100811d9, "XF86Fn_F8"),
    (0x100811da, "XF86Fn_F9"),
    (0x100811db, "XF86Fn_F10"),
    (0x100811dc, "XF86Fn_F11"),
    (0x100811dd, "XF86Fn_F12"),
    (0x100811de, "XF86Fn_1"),
    (0x100811df, "XF86Fn_2"),
    (0x100811e0, "XF86Fn_D"),
    (0x100811e1, "XF86Fn_E"),
    (0x100811e2, "XF86Fn_F"),
    (0x100811e3, "XF86Fn_S"),
    (0x100811e4, "XF86Fn_B"),
    (0x100811e5, "XF86FnRightShift"),
    (0x10081200, "XF86Numeric0"),
    (0x10081201, "XF86Numeric1"),
    (0x10081202, "XF86Numeric2"),
    (0x10081203, "XF86Numeric3"),
    (0x10081204, "XF86Numeric4"),
    (0x10081205, "XF86Numeric5"),
    (0x10081206, "XF86Numeric6"),
    (0x10081207, "XF86Numeric7"),
    (0x10081208, "XF86Numeric8"),
    (0x10081209, "XF86Numeric9"),
    (0x1008120a, "XF86NumericStar"),
    (0x1008120b, "XF86NumericPound"),
    (0x1008120c, "XF86NumericA"),
    (0x1008120d, "XF86NumericB"),
    (0x1008120e, "XF86NumericC"),
    (0x1008120f, "XF86NumericD"),
    (0x10081210, "XF86CameraFocus"),
    (0x10081211, "XF86WPSButton"),
    (0x10081215, "XF86CameraZoomIn"),
    (0x10081216, "XF86CameraZoomOut"),
    (0x10081217, "XF86CameraUp"),
    (0x10081218, "XF86CameraDown"),
    (0x10081219, "XF86CameraLeft"),
    (0x1008121a, "XF86CameraRight"),
    (0x1008121b, "XF86AttendantOn"),
    (0x1008121c, "XF86AttendantOff"),
    (0x1008121d, "XF86AttendantToggle"),
    (0x1008121e, "XF86LightsToggle"),
    (0x10081230, "XF86ALSToggle"),
    (0x10081232, "XF86RefreshRateToggle"),
    (0x10081240, "XF86Buttonconfig"),
    (0x10081241, "XF86Taskmanager"),
    (0x10081242, "XF86Journal"),
    (0x10081243, "XF86ControlPanel"),
    (0x10081244, "XF86AppSelect"),
    (0x10081245, "XF86Screensaver"),
    (0x10081246, "XF86VoiceCommand"),
    (0x10081247, "XF86Assistant"),
    (0x10081249, "XF86EmojiPicker"),
    (0x1008124a, "XF86Dictate"),
    (0x1008124b, "XF86CameraAccessEnable"),
    (0x1008124c, "XF86CameraAccessDisable"),
    (0x1008124d, "XF86CameraAccessToggle"),
    (0x1008124e, "XF86Accessibility"),
    (0x1008124f, "XF86DoNotDisturb"),
    (0x10081250, "XF86BrightnessMin"),
    (0x10081251, "XF86BrightnessMax"),
    (0x10081252, "XF86ElectronicPrivacyScreenOn"),
    (0x10081253, "XF86ElectronicPrivacyScreenOff"),
    (0x10081260, "XF86KbdInputAssistPrev"),
    (0x10081261, "XF86KbdInputAssistNext"),
    (0x10081262, "XF86KbdInputAssistPrevgroup"),
    (0x10081263, "XF86KbdInputAssistNextgroup"),
    (0x10081264, "XF86KbdInputAssistAccept"),
    (0x10081265, "XF86KbdInputAssistCancel"),
    (0x10081266, "XF86RightUp"),
    (0x10081267, "XF86RightDown"),
    (0x10081268, "XF86LeftUp"),
    (0x10081269, "XF86LeftDown"),
    (0x1008126a, "XF86RootMenu"),
    (0x1008126b, "XF86MediaTopMenu"),
    (0x1008126c, "XF86Numeric11"),
    (0x1008126d, "XF86Numeric12"),
    (0x1008126e, "XF86AudioDesc"),
    (0x1008126f, "XF863DMode"),
    (0x10081270, "XF86NextFavorite"),
    (0x10081271, "XF86StopRecord"),
    (0x10081272, "XF86PauseRecord"),
    (0x10081273, "XF86VOD"),
    (0x10081274, "XF86Unmute"),
    (0x10081275, "XF86FastReverse"),
    (0x10081276, "XF86SlowReverse"),
    (0x10081277, "XF86Data"),
    (0x10081278, "XF86OnScreenKeyboard"),
    (0x10081279, "XF86PrivacyScreenToggle"),
    (0x1008127a, "XF86SelectiveScreenshot"),
    (0x1008127b, "XF86NextElement"),
    (0x1008127c, "XF86PreviousElement"),
    (0x1008127d, "XF86AutopilotEngageToggle"),
    (0x1008127e, "XF86MarkWaypoint"),
    (0x1008127f, "XF86Sos"),
    (0x10081280, "XF86NavChart"),
    (0x10081281, "XF86FishingChart"),
    (0x10081282, "XF86SingleRangeRadar"),
    (0x10081283, "XF86DualRangeRadar"),
    (0x10081284, "XF86RadarOverlay"),
    (0x10081285, "XF86TraditionalSonar"),
    (0x10081286, "XF86ClearvuSonar"),
    (0x10081287, "XF86SidevuSonar"),
    (0x10081288, "XF86NavInfo"),
    (0x10081290, "XF86Macro1"),
    (0x10081291, "XF86Macro2"),
    (0x10081292, "XF86Macro3"),
    (0x10081293, "XF86Macro4"),
    (0x10081294, "XF86Macro5"),
    (0x10081295, "XF86Macro6"),
    (0x10081296, "XF86Macro7"),
    (0x10081297, "XF86Macro8"),
    (0x10081298, "XF86Macro9"),
    (0x10081299, "XF86Macro10"),
    (0x1008129a, "XF86Macro11"),
    (0x1008129b, "XF86Macro12"),
    (0x1008129c, "XF86Macro13"),
    (0x1008129d, "XF86Macro14"),
    (0x1008129e, "XF86Macro15"),
    (0x1008129f, "XF86Macro16"),
    (0x100812a0, "XF86Macro17"),
    (0x100812a1, "XF86Macro18"),
    (0x100812a2, "XF86Macro19"),
    (0x100812a3, "XF86Macro20"),
    (0x100812a4, "XF86Macro21"),
    (0x100812a5, "XF86Macro22"),
    (0x100812a6, "XF86Macro23"),
    (0x100812a7, "XF86Macro24"),
    (0x100812a8, "XF86Macro25"),
    (0x100812a9, "XF86Macro26"),
    (0x100812aa, "XF86Macro27"),
    (0x100812ab, "XF86Macro28"),
    (0x100812ac, "XF86Macro29"),
    (0x100812ad, "XF86Macro30"),
    (0x100812b0, "XF86MacroRecordStart"),
    (0x100812b1, "XF86MacroRecordStop"),
    (0x100812b2, "XF86MacroPresetCycle"),
    (0x100812b3, "XF86MacroPreset1"),
    (0x100812b4, "XF86MacroPreset2"),
    (0x100812b5, "XF86MacroPreset3"),
    (0x100812b8, "XF86KbdLcdMenu1"),
    (0x100812b9, "XF86KbdLcdMenu2"),
    (0x100812ba, "XF86KbdLcdMenu3"),
    (0x100812bb, "XF86KbdLcdMenu4"),
    (0x100812bc, "XF86KbdLcdMenu5"),
    (0x100812bd, "XF86PerformanceMode"),
    (0x1008fe01, "XF86Switch_VT_1"),
    (0x1008fe02, "XF86Switch_VT_2"),
    (0x1008fe03, "XF86Switch_VT_3"),
    (0x1008fe04, "XF86Switch_VT_4"),
    (0x1008fe05, "XF86Switch_VT_5"),
    (0x1008fe06, "XF86Switch_VT_6"),
    (0x1008fe07, "XF86Switch_VT_7"),
    (0x1008fe08, "XF86Switch_VT_8"),
    (0x1008fe09, "XF86Switch_VT_9"),
    (0x1008fe0a, "XF86Switch_VT_10"),
    (0x1008fe0b, "XF86Switch_VT_11"),
    (0x1008fe0c, "XF86Switch_VT_12"),
    (0x1008fe20, "XF86Ungrab"),
    (0x1008fe21, "XF86ClearGrab"),
    (0x1008fe22, "XF86Next_VMode"),
    (0x1008fe23, "XF86Prev_VMode"),
    (0x1008fe24, "XF86LogWindowTree"),
    (0x1008fe25, "XF86LogGrabInfo"),
    (0x1008ff01, "XF86ModeLock"),
    (0x1008ff02, "XF86MonBrightnessUp"),
    (0x1008ff03, "XF86MonBrightnessDown"),
    (0x1008ff04, "XF86KbdLightOnOff"),
    (0x1008ff05, "XF86KbdBrightnessUp"),
    (0x1008ff06, "XF86KbdBrightnessDown"),
    (0x1008ff07, "XF86MonBrightnessCycle"),
    (0x1008ff10, "XF86Standby"),
    (0x1008ff11, "XF86AudioLowerVolume"),
    (0x1008ff12, "XF86AudioMute"),
    (0x1008ff13, "XF86AudioRaiseVolume"),
    (0x1008ff14, "XF86AudioPlay"),
    (0x1008ff15, "XF86AudioStop"),
    (0x1008ff16, "XF86AudioPrev"),
    (0x1008ff17, "XF86AudioNext"),
    (0x1008ff18, "XF86HomePage"),
    (0x1008ff19, "XF86Mail"),
    (0x1008ff1a, "XF86Start"),
    (0x1008ff1b, "XF86Search"),
    (0x1008ff1c, "XF86AudioRecord"),
    (0x1008ff1d, "XF86Calculator"),
    (0x1008ff1e, "XF86Memo"),
    (0x1008ff1f, "XF86ToDoList"),
    (0x1008ff20, "XF86Calendar"),
    (0x1008ff21, "XF86PowerDown"),
    (0x1008ff22, "XF86ContrastAdjust"),
    (0x1008ff23, "XF86RockerUp"),
    (0x1008ff24, "XF86RockerDown"),
    (0x1008ff25, "XF86RockerEnter"),
    (0x1008ff26, "XF86Back"),
    (0x1008ff27, "XF86Forward"),
    (0x1008ff28, "XF86Stop"),
    (0x1008ff29, "XF86Refresh"),
    (0x1008ff2a, "XF86PowerOff"),
    (0x1008ff2b, "XF86WakeUp"),
    (0x1008ff2c, "XF86Eject"),
    (0x1008ff2d, "XF86ScreenSaver"),
    (0x1008ff2e, "XF86WWW"),
    (0x1008ff2f, "XF86Sleep"),
    (0x1008ff30, "XF86Favorites"),
    (0x1008ff31, "XF86AudioPause"),
    (0x1008ff32, "XF86AudioMedia"),
    (0x1008ff33, "XF86MyComputer"),
    (0x1008ff34, "XF86VendorHome"),
    (0x1008ff35, "XF86LightBulb"),
    (0x1008ff36, "XF86Shop"),
    (0x1008ff37, "XF86History"),
    (0x1008ff38, "XF86OpenURL"),
    (0x1008ff39, "XF86AddFavorite"),
    (0x1008ff3a, "XF86HotLinks"),
    (0x1008ff3b, "XF86BrightnessAdjust"),
    (0x1008ff3c, "XF86Finance"),
    (0x1008ff3d, "XF86Community"),
    (0x1008ff3e, "XF86AudioRewind"),
    (0x1008ff3f, "XF86BackForward"),
    (0x1008ff40, "XF86Launch0"),
    (0x1008ff41, "XF86Launch1"),
    (0x1008ff42, "XF86Launch2"),
    (0x1008ff43, "XF86Launch3"),
    (0x1008ff44, "XF86Launch4"),
    (0x1008ff45, "XF86Launch5"),
    (0x1008ff46, "XF86Launch6"),
    (0x1008ff47, "XF86Launch7"),
    (0x1008ff48, "XF86Launch8"),
    (0x1008ff49, "XF86Launch9"),
    (0x1008ff4a, "XF86LaunchA"),
    (0x1008ff4b, "XF86LaunchB"),
    (0x1008ff4c, "XF86LaunchC"),
    (0x1008ff4d, "XF86LaunchD"),
    (0x1008ff4e, "XF86LaunchE"),
    (0x1008ff4f, "XF86LaunchF"),
    (0x1008ff50, "XF86ApplicationLeft"),
    (0x1008ff51, "XF86ApplicationRight"),
    (0x1008ff52, "XF86Book"),
    (0x1008ff53, "XF86CD"),
    (0x1008ff53, "XF86MediaSelectCD"),
    (0x1008ff54, "XF86Calculater"),
    (0x1008ff55, "XF86Clear"),
    (0x1008ff56, "XF86Close"),
    (0x1008ff57, "XF86Copy"),
    (0x1008ff58, "XF86Cut"),
    (0x1008ff59, "XF86Display"),
    (0x1008ff5a, "XF86DOS"),
    (0x1008ff5b, "XF86Documents"),
    (0x1008ff5c, "XF86Excel"),
    (0x1008ff5d, "XF86Explorer"),
    (0x1008ff5e, "XF86Game"),
    (0x1008ff5f, "XF86Go"),
    (0x1008ff60, "XF86iTouch"),
    (0x1008ff61, "XF86LogOff"),
    (0x1008ff62, "XF86Market"),
    (0x1008ff63, "XF86Meeting"),
    (0x1008ff65, "XF86MenuKB"),
    (0x1008ff66, "XF86MenuPB"),
    (0x1008ff67, "XF86MySites"),
    (0x1008ff68, "XF86New"),
    (0x1008ff69, "XF86News"),
    (0x1008ff6a, "XF86OfficeHome"),
    (0x1008ff6b, "XF86Open"),
    (0x1008ff6c, "XF86Option"),
    (0x1008ff6d, "XF86Paste"),
    (0x1008ff6e, "XF86Phone"),
    (0x1008ff70, "XF86Q"),
    (0x1008ff72, "XF86Reply"),
    (0x1008ff73, "XF86Reload"),
    (0x1008ff74, "XF86RotateWindows"),
    (0x1008ff75, "XF86RotationPB"),
    (0x1008ff76, "XF86RotationKB"),
    (0x1008ff77, "XF86Save"),
    (0x1008ff78, "XF86ScrollUp"),
    (0x1008ff79, "XF86ScrollDown"),
    (0x1008ff7a, "XF86ScrollClick"),
    (0x1008ff7b, "XF86Send"),
    (0x1008ff7c, "XF86Spell"),
    (0x1008ff7d, "XF86SplitScreen"),
    (0x1008ff7e, "XF86Support"),
    (0x1008ff7f, "XF86TaskPane"),
    (0x1008ff80, "XF86Terminal"),
    (0x1008ff81, "XF86Tools"),
    (0x1008ff82, "XF86Travel"),
    (0x1008ff84, "XF86UserPB"),
    (0x1008ff85, "XF86User1KB"),
    (0x1008ff86, "XF86User2KB"),
    (0x1008ff87, "XF86Video"),
    (0x1008ff88, "XF86WheelButton"),
    (0x1008ff89, "XF86Word"),
    (0x1008ff8a, "XF86Xfer"),
    (0x1008ff8b, "XF86ZoomIn"),
    (0x1008ff8c, "XF86ZoomOut"),
    (0x1008ff8d, "XF86Away"),
    (0x1008ff8e, "XF86Messenger"),
    (0x1008ff8f, "XF86WebCam"),
    (0x1008ff90, "XF86MailForward"),
    (0x1008ff91, "XF86Pictures"),
    (0x1008ff92, "XF86Music"),
    (0x1008ff93, "XF86Battery"),
    (0x1008ff94, "XF86Bluetooth"),
    (0x1008ff95, "XF86WLAN"),
    (0x1008ff96, "XF86UWB"),
    (0x1008ff97, "XF86AudioForward"),
    (0x1008ff98, "XF86AudioRepeat"),
    (0x1008ff99, "XF86AudioRandomPlay"),
    (0x1008ff9a, "XF86Subtitle"),
    (0x1008ff9b, "XF86AudioCycleTrack"),
    (0x1008ff9c, "XF86CycleAngle"),
    (0x1008ff9d, "XF86FrameBack"),
    (0x1008ff9e, "XF86FrameForward"),
    (0x1008ff9f, "XF86Time"),
    (0x1008ffa0, "XF86Select"),
    (0x1008ffa1, "XF86View"),
    (0x1008ffa2, "XF86TopMenu"),
    (0x1008ffa3, "XF86Red"),
    (0x1008ffa4, "XF86Green"),
    (0x1008ffa5, "XF86Yellow"),
    (0x1008ffa6, "XF86Blue"),
    (0x1008ffa7, "XF86Suspend"),
    (0x1008ffa8, "XF86Hibernate"),
    (0x1008ffa9, "XF86TouchpadToggle"),
    (0x1008ffb0, "XF86TouchpadOn"),
    (0x1008ffb1, "XF86TouchpadOff"),
    (0x1008ffb2, "XF86AudioMicMute"),
    (0x1008ffb3, "XF86Keyboard"),
    (0x1008ffb4, "XF86WWAN"),
    (0x1008ffb5, "XF86RFKill"),
    (0x1008ffb6, "XF86AudioPreset"),
    (0x1008ffb7, "XF86RotationLockToggle"),
    (0x1008ffb8, "XF86FullScreen"),
];
